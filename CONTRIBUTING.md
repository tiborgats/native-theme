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

The script has three modes. At a task's end, run the per-task gate, which
runs the per-commit subset's steps but leaves to it each test, documentation
and workspace-MSRV step whose crates, what they build against and include,
the other files outside the crates that a crate may read, the tools, the
environment and the step's command are unchanged since that step last passed:

```bash
./scripts/check_release.sh --quick
```

Every third code task (unit), counted from the last `--commit`, on a
release's last code task, after a lock or toolchain change and where a plan
names a task structural, run the per-commit subset:

```bash
./scripts/check_release.sh --commit
```

On the last commit of every fix phase and before the release commit, run the
full pre-release check:

```bash
./scripts/check_release.sh
```

This is the canonical quality gate. In full, it runs, in order: the
clean-tree check (uncommitted changes are noted, not failed) and the
release-tag check, a TODO/FIXME scan
of every `.rs` file outside `target/`, the panic-pattern scan
(`scripts/check_panic_patterns.py` and its `--self-test`) of each workspace
member's `src/` outside its test modules and test-only files (not `build.rs`,
`examples/`, `tests/` or `benches/`), the step cache's self-test
(`scripts/step_cache.sh --self-test`; in `--commit` and `--quick` only: the
full run uses no cache), per-crate `cargo check --all-targets`,
CI's MSRV checks (`ci.yml`'s `msrv` job: the workspace's `rust-version` and the
connectors' toolchain, with `RUSTFLAGS=-Dwarnings`; a toolchain not installed
fails) and the check that they are the job's (`scripts/msrv_checks.sh`; these
two in `--commit` and `--quick` only: the full run leaves them to `--commit`
and CI),
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
panic lints (`scripts/check_strict_panic.sh`: `clippy::unwrap_used`,
`clippy::indexing_slicing` and the rest of the type-aware set) on library code,
on the three showcase examples, the OS probe and the test fixtures
(`native-theme-gpui` and its showcase with `native,watch` and with
`gpui-fast,native,watch`),
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
on Linux the three showcases' tests in CI's runner environment
(`scripts/run_in_runner_env.sh`, in the feature sets CI tests them with; a
server outage skips them with a warning),
`cargo build --examples` for the crates that have examples, the widget-coverage
check (`scripts/check_widget_coverage.py`, Python 3.11+), the connector-parity
check (`scripts/check_connector_parity.py` and its `--self-test`), the
README-version check (`scripts/check_readme_versions.py` and its `--self-test`), the showcase-parity
comparator's static modes (`scripts/check_showcase_parity.py --check-list` and
`--self-test`, which needs Pillow), the data-document checks
(`scripts/check_platform_facts_rows.py`, `scripts/check_wrong_safety_nets.py`,
`scripts/check_section_refs.py`, `scripts/check_todo_citations.py` and
`scripts/check_preset_citations.py`, each with its `--self-test`), per-crate
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
a single-line status per check, and a failing check's captured output. These
report a warning (`⚠`) and let the run go on: the semver checks, since
approved breaks are expected before a release (a semver warning is resolved
by the version number or by undoing the change); the outdated dependencies,
and a `cargo outdated` run that failed; the TODO/FIXME scan; a tag the version
already has; a cross-target check whose target or linker is missing, which is
skipped; the showcases in CI's runner environment where its packages can be
neither fetched nor found; the offline audit of `--commit` and `--quick`; the
PROVENANCE and compatibility stamps while the CHANGELOG entry for the version
is undated, and asset sources with uncommitted changes; a tree outside git,
where the clean-tree and release-tag checks are skipped; and a step-cache
record that could not be written or removed. Every other check is hard, and a
hard failure (`❌`) exits non-zero and must be fixed. It needs jq and installs nothing: a missing `cargo-audit`,
`cargo-outdated` or `cargo-semver-checks` fails the full run with its install
command.

`--commit` runs every one of those checks except the release-tag check, the gpui
connector's tests on GPUI Fast (the runner-environment leg on GPUI Fast among
them), `native-theme`'s ignored stem-index test and the release-only sections:
the docs.rs emulation (`scripts/check_docsrs.py`), packaging,
`cargo semver-checks`, `cargo outdated`, and the PROVENANCE and
compatibility stamp checks. Each of
them prints `skipped (--commit mode)`, and the summary names them as skipped,
not passed. `cargo audit` runs in every mode, as CI audits every pull request and every push to `main`; where
the advisory database cannot be fetched, the full run fails, and `--commit`
audits against the cached database with a warning naming its date, or warns
that the audit did not run where no cached database loads. `--commit` needs
`cargo-audit` installed too.

`--quick` runs every step `--commit` runs, skipping the same checks (each
printing `skipped (--quick mode)`), but it leaves to `--commit` each test step
(the showcases' tests in CI's runner environment among them), each
`cargo doc` step and the two MSRV checks at the workspace's `rust-version`
whose key is unchanged since the step last passed in `--commit` or `--quick`,
and its summary lists them. The key (`scripts/step_cache.sh`) follows the
crates the step names, everything they build against in the workspace and
the files their code includes, the files outside the crates but those
`QUICK_UNREAD` lists (the Markdown documents, `.github/`, the licences, the
git ignore and attribute files and the archived, issue, asset and evidence
folders under `docs/`), the tool versions, the environment but its thread
counts, output, location and speed settings and the per-session variables,
the gate's scripts and the step's command, and for an MSRV check its
toolchain. It does not follow what a test reads outside the repository (the
desktop, the installed fonts and icon themes, the system libraries, the
network, the clock), a run-time read of a file in `QUICK_UNREAD`, or what a
registry build script or procedural macro reads; the next `--commit` checks
those.

`--commit` and `--quick` are faster than the steps they list: where
`cargo-nextest` is installed their test steps run through it (the doctests
through `cargo test --doc`; the showcases' tests in CI's runner environment
stay on `cargo test`), and a compile-only or documentation step, or
native-theme-derive's tests, whose declared inputs, tools, environment and
command are unchanged since it last passed is skipped, the summary naming it
`unchanged since <commit>` (`scripts/step_cache.sh`; `--no-cache` runs every
step: it reads no record and records no pass; a failure still removes the
step's `--quick` record). The full run uses neither: every step runs, as
listed above, through `cargo test`; it reads no record and records no pass,
and a failure still removes the step's `--quick` record.

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
| `native-theme-derive` | `native-theme-derive/` | Proc-macro crate (internal; used by `native-theme`, not re-exported) |
| `native-theme-gpui` | `connectors/native-theme-gpui/` | gpui toolkit connector |
| `native-theme-iced` | `connectors/native-theme-iced/` | iced toolkit connector |
| `native-theme-egui` | `connectors/native-theme-egui/` | egui toolkit connector |
| `native-theme-egui-widgets` | `connectors/native-theme-egui-widgets/` | egui widgets drawn from native-theme-egui's theme where egui's own cannot follow it: a switch, a segmented control, and a slider, spinner, links, drop-down, radio button, progress bar and expander as the theme states them |

## Feature flags

The core `native-theme` crate has several feature flags:

- **Platform readers:** `kde`, `portal`, `windows`, `macos`, `linux` (meta: kde+portal), `native` (meta: all); `linux-kde` and `linux-portal` are aliases of `kde` and `portal`
- **Icons:** `system-icons`, `material-icons`, `lucide-icons`
- **Watching:** `watch` (runtime theme change notifications)
- **Rendering:** `svg-rasterize` (resvg-backed SVG→RGBA rasterisation)
- **Fonts:** `system-fonts` (face matching against the installed fonts, `fonts::system_face`, and the platform's substitute for a family the system lacks, `fonts::substitute_family`)

Platform-specific features only compile on their target OS. See
[`native-theme/Cargo.toml`](native-theme/Cargo.toml) for the full list and
dependency gates.

## Commit message conventions

Use conventional-commit-style prefixes. Common forms seen in this project:

- `feat(scope): <summary>` — new user-facing feature or API addition
- `fix(scope): <summary>` — bug fix
- `refactor(scope): <summary>` — restructuring with no behaviour change
- `docs(scope): <summary>` — docs-only change
- `test(scope): <summary>` — tests only
- `chore(scope): <summary>` — build, tooling, deps
- `ci(workflow): <summary>` — GitHub Actions changes

A breaking change marks its prefix with `!`, whatever its kind
(`refactor(connectors)!:`, `feat(iced)!:`). The `scope` is usually the crate
name (`native-theme`, `gpui`, `iced`, `readme`, `changelog`) or a subsystem
(`icons`, `api`, `watch/kde`). A commit that resolves a code-review item ends
its summary with the item's id in parentheses: a ledger entry (`(M7-08)`) or
a fix-phase task (`(M8 T19)`). When in doubt, match the style of recent
commits:

```bash
git log --oneline -30
```

Pre-1.0, breaking changes in minor versions are allowed and expected — just
ensure they appear in `CHANGELOG.md` under the `[Unreleased]` section (create it above the latest release if there
is none).

## Pull request workflow

1. Fork the repository, branch off `main`
2. Keep commits atomic (one concern per commit)
3. Ensure `./scripts/check_release.sh` passes on the last commit of your branch
4. Open a PR with a clear title matching the conventional-commit style
5. Reference related issues or design docs in the PR description
6. CI runs automatically; results appear on the PR page

## Where decisions live

Design-level choices are captured in files under `docs/`:

- [`docs/platform-facts.md`](docs/platform-facts.md) — canonical platform-native values (macOS 12–15 and 26, Windows 11, KDE Plasma, GNOME, Material 3), cited to official sources
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

A non-trivial GitHub issue gets a rationale before it is answered:
`docs/issues/<NNNN>_<topic>-rationale.md`, the issue number zero-padded to four
digits. It holds the facts checked, the options weighed, the decision and the
reply as posted, and it stays in `docs/issues/`: it is the issue's record, not a
plan, so it is not archived. A plan the issue leads to follows the rule above.

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
publish workflow's release gate runs the same check on the pushed tag, so a tag
whose assets were captured from other sources is refused before anything is
uploaded. Docs-only changes leave the hash untouched and need no recapture.

## Release order

1. Bump the workspace version in `Cargo.toml`, commit, push.
2. `./scripts/generate_assets_release.sh`. Besides the captures, it compares
   the three showcases at rest from the layout dumps its Linux screenshot runs
   write, and stops where they differ with no reason in
   `docs/showcase-exceptions.toml`: record the reason there
   (`scripts/check_showcase_parity.py --merge` proposes the groups) and run it
   again. Its last step rewrites `docs/COMPATIBILITY.toml` and the connector
   READMEs' Verified lines (`scripts/update_compatibility.sh run`). Review the
   images, then commit the assets together with `docs/assets/PROVENANCE.toml`,
   `docs/COMPATIBILITY.toml`, the connector READMEs and, where you changed it,
   `docs/showcase-exceptions.toml`. The script's last line prints the `git add`
   for all but the exceptions file.
3. Date the CHANGELOG entry and set its compare link; commit as
   `chore(release): vX.Y.Z`.
4. `./scripts/check_release.sh` on that commit (the asset check is hard now).
5. Fast-forward `main` to it and push; CI runs on the exact commit.
6. `git tag vX.Y.Z` on that commit and push the one tag by name. The publish
   workflow runs its release gate (the tag names this version; the asset and
   compatibility stamps are current), then all of CI on the tag's commit,
   `cargo package` of every crate and the docs.rs emulation, and only when
   all of them pass uploads the crates in dependency order. CI's security
   audit is one of those gates: an advisory published on release day blocks
   the upload until it is fixed or ignored, with its reason, in
   `.cargo/audit.toml`.
7. Create the GitHub release page from the tag with the CHANGELOG section as
   notes, then check the docs.rs build pages.

## License

By contributing, you agree your contributions will be triple-licensed under
**MIT OR Apache-2.0 OR 0BSD**.
