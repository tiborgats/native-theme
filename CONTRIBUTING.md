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
(`scripts/check_platform_facts_rows.py`,
`scripts/check_section_refs.py`, `scripts/check_todo_citations.py` and
`scripts/check_preset_citations.py`, each with its `--self-test`), the 0.8.0
plan's data checks, each with its `--self-test` (the plan's ledger,
`scripts/check_ledger.py`; the pinned platform sources,
`scripts/check_sources.py`, and their cache, `scripts/update_sources.sh
--verify`; the facts' form and citations, `scripts/check_facts.py` and
`scripts/check_fact_citations.py`; the generated widget declarations,
bundled presets, `sf_tracking` and `dynamic_type` tables, each
generator's `--check`; `docs/platform-facts.md`, which is not committed, its
generation into a scratch directory, `scripts/generate_platform_facts.py`; the platform functions' vector tables,
`scripts/check_function_vectors.py`; the enabled units' trace lines,
`scripts/check_facts_trace.py --enabled`; the line citations of today's
`platform-facts.md` at their commit, `scripts/check_pinned_citations.py`), per-crate
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
connector's tests on GPUI Fast (the runner-environment leg on GPUI Fast among
them), `native-theme`'s ignored stem-index test and the release-only sections:
the docs.rs emulation (`scripts/check_docsrs.py`), packaging,
`cargo semver-checks`, `cargo outdated`, the 0.8.0 ledger's triage and
source-cache checks, and the PROVENANCE and compatibility stamp checks. Each of
them prints `skipped (--commit mode)`, and the summary names them as skipped,
not passed. `cargo audit` runs in both modes, as CI audits every pull request and every push to `main`; where
the advisory database cannot be fetched, the full run fails, and `--commit`
audits against the cached database with a warning naming its date, or warns
that the audit did not run where no cached database loads. `--commit` needs
`cargo-audit` installed too.

`--commit` and `--quick` are faster than the steps they list: where
`cargo-nextest` is installed their test steps run through it (the doctests
through `cargo test --doc`), and a step whose declared inputs, tools,
environment and command are unchanged since it last passed is skipped, the
summary naming it `unchanged since <commit>` (`scripts/step_cache.sh`;
`--no-cache` runs every step). The full run uses neither: every step runs, as
listed above, through `cargo test`.

The script runs the checks of the CI pipeline on the local platform, plus the
packaging, semver, outdated-dependency and archive-index checks CI does not run.
On Linux it tests `native-theme` with no features and with all its Linux
features at once. CI also runs the Linux features one leg at a time, which
builds code no local leg builds (the reader choice's `kde`-without-`portal`
arm, for one), and the macOS and Windows test legs: the script checks and
lints those targets where they are installed, but cannot run their tests.
Both are left to CI.

While the 0.8.0 model replaces today's (`docs/todo_v0.7.1_model-completeness-plan.md`),
`PLAN_EXCLUDED`, a list near the top of `scripts/check_release.sh`, names
each step the transition breaks with the milestone that brings it back: the
connectors and the showcases, which build against the 0.7.0 API until they
are ported, and three data-document checks. `--commit` skips each listed
step; the full run runs it and reports it as a warning; `ci.yml` marks the
same steps with `# PLAN_EXCLUDED: <step>`, and the script checks that both
name the same set. So on the work branch `cargo test --workspace` fails in
the connectors and the showcases' shared crate: test the crates the list
does not name, `cargo test -p native-theme` among them. The list is empty when 0.8.0 is released.

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

The facts gate (`native-theme/tests/facts_gate.rs`) gates every bundled preset
in one run, minutes per preset. While you work on some presets, limit it to
them:

```bash
NT_FACTS_GATE_PRESETS=adwaita,macos-tahoe cargo test -p native-theme --test facts_gate
```

An id no bundled preset has, an empty name or an id named twice fails the
test; `none` runs no preset, the facts' own checks alone. The gate's report
(`-- --nocapture`) says it ran a subset. A subset is no gate:
`scripts/check_release.sh` runs every preset, in its full run in one step, in
`--commit` and `--quick` each preset in a step of its own beside the facts'
own checks (`none`), which its step cache skips one by one, and the release's
run (`FACTS_GATE_RELEASE=1`) refuses a subset.

`NT_FACTS_GATE_SHARD=k/N` runs the `k`th of `N` shards of the gate's work:
each condition vector is checked by one shard alone, and the rest of the run
by every shard but recorded by the first. The presets' `[pending]` tables are
checked against the listed paths of all the shards at once: each shard writes
those it found to the directory `NT_FACTS_GATE_LISTED` names, and the gate
with `NT_FACTS_GATE_MERGE` naming it checks the tables instead of gating.
Together they find what the whole run finds (`gate_shards_add_up`). CI runs
the gate so, one job per shard and one for the tables; the release's run
refuses a shard.

CI runs the gate on Linux with the default features alone, and
`scripts/check_release.sh` runs it once, because no code on its path depends
on the target or the features. `scripts/check_gate_cfg.py` keeps it so: it
finds the path from the sources (the gate's test files, then every module
their `use` and `mod` graph reaches) and fails on any `cfg`, `cfg!` or
`cfg_attr` along it but a test-only one or a lint-level `cfg_attr`. Where it
fails, move the platform code off the gate's path, or the gate must run on
every configuration again. Every mode of `check_release.sh` and CI's parity
job run it.

The gate runs its parallel steps on as many threads as the machine has;
`NT_FACTS_GATE_THREADS=8` runs them on eight, and a value that is no number
above zero fails the test. The count sets the speed and the memory: as many
condition vectors are checked at once as there are threads, each one's
combinations enumerated as they are checked, a chunk at a time, never all
held, so each thread adds about 120 MB and the six presets peak near 4 GB on
16 threads, 6 GB on 32.

## Project structure

| Crate | Path | Description |
|-------|------|-------------|
| `native-theme` | `native-theme/` | Core theme model (layers, resolution, the queries), its presets, icons and fonts; the platform readers, unlinked until the 0.8.0 plan's M1b |
| `native-theme-build` | `native-theme-build/` | Build-time code generation for custom icon roles |
| `native-theme-derive` | `native-theme-derive/` | Proc-macro crate (internal; used by `native-theme`, not re-exported) |
| `native-theme-gpui` | `connectors/native-theme-gpui/` | gpui toolkit connector |
| `native-theme-iced` | `connectors/native-theme-iced/` | iced toolkit connector |
| `native-theme-egui` | `connectors/native-theme-egui/` | egui toolkit connector |
| `native-theme-egui-widgets` | `connectors/native-theme-egui-widgets/` | egui widgets drawn from native-theme-egui's theme where egui's own cannot follow it: a switch, a segmented control, and a slider, spinner, links, drop-down, radio button, progress bar and expander as the theme states them |

## Feature flags

The core `native-theme` crate has several feature flags:

- **Platform readers:** `kde`, `portal`, `windows`, `macos`, `linux` (meta: kde+portal), `native` (meta: all); `linux-kde` and `linux-portal` are aliases of `kde` and `portal`. The readers are unlinked until the 0.8.0 plan's M1b: until then `kde` reads the KDE icon theme, `windows` and `macos` the OS version, and `portal` nothing
- **Icons:** `system-icons`, `material-icons`, `lucide-icons`
- **Watching:** `watch` (runtime theme change notifications; nothing until M1b)
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

- [`docs/facts/`](docs/facts/) — the verified platform facts, one directory per platform: each value with its source, pinned in [`docs/facts/sources.toml`](docs/facts/sources.toml), or its measurement with the evidence in the evidence repository, [`native-theme-evidence`](https://github.com/tiborgats/native-theme-evidence), at the paths its citations name (`docs/evidence/…`) and at the commit `sources.toml` pins; the bundled presets are generated from them (`scripts/generate_preset.py`)
- `docs/platform-facts.md` — every value the facts state, per preset, generated from `docs/facts/` by `scripts/generate_platform_facts.py` (`python3 scripts/generate_platform_facts.py` writes it); not committed, never edited (the hand-written document of 0.7.0, which the planning documents cite by line, is the one commit `fc6109d1` holds)
- [`docs/property-registry.toml`](docs/property-registry.toml) — the registry of 0.7.0's theme fields, generated from the 0.8.0 model at its plan's M2.6
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

Do not invent platform-native values. A platform value enters the model only
as a fact (specification S17,
`docs/todo_v0.7.1_model-completeness-spec.md`): published by a source pinned
to the platform's release, read from an API the reader calls, or measured,
with the method, the evidence and the exact OS and toolkit builds recorded.
When no source gives a value, it stays a pending fact naming its entry in the
ledger's unmeasured register, never a guess.
[`docs/new-os-version-guide.md`](docs/new-os-version-guide.md) walks through
the process for a new OS version.

## Regenerating visual assets

Screenshots, theme-switching GIFs, and the workspace
crate-relations diagram are generated by the scripts in `scripts/`. See
[`scripts/README.md`](scripts/README.md) for the full pipeline.

```bash
./scripts/generate_assets_release.sh        # full screenshots + GIFs pipeline (needs gh, spectacle and KDE Plasma 6's kscreen-doctor and qdbus6, dbus-monitor/dbus-send, Python 3.11+ with Pillow and network access; see scripts/README.md)
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
