# Contributing to native-theme

Thank you for your interest in contributing.

## Quick start

1. Fork and clone the repository
2. Install Rust stable (workspace MSRV **1.88.0**, edition **2024**; the gpui
   connector alone needs **1.95.0**, so `cargo test --workspace` needs 1.95.0 or newer)
3. Verify the workspace builds and tests pass:

   ```bash
   cargo test --workspace
   ```

## Pre-commit gate — `./pre-release-check.sh`

Before every commit, run the full pre-release check:

```bash
./pre-release-check.sh
```

This is the canonical quality gate. It runs, in order: a TODO/FIXME and
panic-pattern scan of non-test code, per-crate `cargo check --all-targets`,
`cargo fmt --all` (formats in place; CI uses `--check`), per-crate
`cargo clippy --all-targets -- -D warnings`, the strict panic lints on library
code (`clippy::unwrap_used`, `clippy::indexing_slicing` and the rest of the
type-aware set), per-crate `cargo test`, `cargo build --examples` for the
crates that have examples, per-crate `cargo doc --no-deps`, `cargo package` on
every publishable crate, `cargo audit` and `cargo outdated`. The script prints
a single-line status per check; the gpui-connector checks and the connector
packaging are soft (`⚠`), tolerable on a WIP branch but to be resolved before
opening a release PR; a hard failure (`❌`) exits non-zero and must be fixed.

The script runs the checks of the CI pipeline on the local platform, plus the
strict panic lints, packaging and outdated-dependency checks CI does not run.
CI additionally tests `native-theme` across its feature matrix on Linux,
Windows and macOS, which the script cannot do locally.

## Individual checks

If you want to run pieces manually instead of the full script:

```bash
# Format
cargo fmt --all --check

# Lint each crate (CI runs clippy per-crate, not workspace-wide)
cargo clippy -p native-theme --all-targets
cargo clippy -p native-theme-build --all-targets
cargo clippy -p native-theme-gpui --all-targets
cargo clippy -p native-theme-iced --all-targets

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

## Feature flags

The core `native-theme` crate has several feature flags:

- **Platform readers:** `kde`, `portal`, `windows`, `macos`, `linux` (meta: kde+portal), `native` (meta: all)
- **Icons:** `system-icons`, `material-icons`, `lucide-icons`
- **Watching:** `watch` (runtime theme change notifications)
- **Rendering:** `svg-rasterize` (resvg-backed SVG→RGBA rasterization)

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
3. Ensure `./pre-release-check.sh` passes on the last commit of your branch
4. Open a PR with a clear title matching the conventional-commit style
5. Reference related issues or design docs in the PR description
6. CI runs automatically; results appear on the PR page

## Where decisions live

Design-level choices are captured in files under `docs/`:

- [`docs/platform-facts.md`](docs/platform-facts.md) — canonical platform-native values (KDE Plasma, GNOME, macOS Sonoma/Tahoe, Windows 11, iOS), cited to official sources
- [`docs/property-registry.toml`](docs/property-registry.toml) — semantic color and property registry
- [`docs/inheritance-rules.toml`](docs/inheritance-rules.toml) — widget-field inheritance rules driving the resolution pipeline

Do not invent platform-native values. When the authoritative reference is
silent, flag it explicitly rather than guessing.

## Regenerating visual assets

Screenshots, theme-switching GIFs, spinner GIFs, and the workspace
crate-relations diagram are generated by the scripts in `scripts/`. See
[`scripts/README.md`](scripts/README.md) for the full pipeline.

```bash
./scripts/pre-release.sh        # full screenshots + GIFs pipeline (needs gh, spectacle, Python+Pillow, ImageMagick)
./scripts/render-diagrams.sh    # workspace crate-relations SVG (needs Node ≥ 18)
```

Each asset is generated into the crate it documents, so relative paths in
per-crate READMEs resolve on GitHub, on crates.io, and offline.

Regeneration cannot be forgotten: `pre-release.sh` ends by writing
`docs/assets/PROVENANCE.toml`, which records the workspace version, the
commit, and a hash over the git object ids of every path that feeds the
showcases (crate sources, presets, icon bundles, `Cargo.lock`, the capture
scripts, the screenshots workflow). `pre-release-check.sh` recomputes that
hash at HEAD; a mismatch is a warning while the CHANGELOG entry for the
version still says Unreleased and a hard failure once the entry is dated. The
publish workflow's CI gate runs the same check on the pushed tag, so a tag
whose assets were captured from other sources is refused before anything is
uploaded. Docs-only changes leave the hash untouched and need no recapture.

## Release order

1. Bump the workspace version in `Cargo.toml`, commit, push.
2. `./scripts/pre-release.sh`, review the images, commit the assets together
   with `docs/assets/PROVENANCE.toml`.
3. Date the CHANGELOG entry and set its compare link; commit as
   `chore(release): vX.Y.Z`.
4. `./pre-release-check.sh` on that commit (the asset check is hard now).
5. Fast-forward `main` to it and push; CI runs on the exact commit.
6. `git tag vX.Y.Z` on that commit and push the one tag by name. The publish
   workflow runs its CI gate, then uploads the crates in dependency order.
7. Create the GitHub release page from the tag with the CHANGELOG section as
   notes, then check the docs.rs build pages.

## License

By contributing, you agree your contributions will be triple-licensed under
**MIT OR Apache-2.0 OR 0BSD**.
