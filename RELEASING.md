# Releasing `native-theme`

This workspace publishes five crates that depend on each other. Publication is
not order-independent — `native-theme` depends on `native-theme-derive`
(proc-macro) and `native-theme-build` (codegen helpers), and the connectors
(`native-theme-iced`, `native-theme-gpui`) depend on `native-theme`. Publishing
has to happen in dependency order.

## Pre-publication checks

Run the pre-release script from the repo root:

```bash
./pre-release-check.sh
```

The script runs, per crate:

- `cargo check`
- `cargo fmt` (autofixes if possible)
- `cargo clippy --all-targets -D warnings`
- `cargo test`
- Example builds (for crates that have an `examples/` directory)
- `cargo doc --no-deps`

It also runs `cargo package --no-verify` on the five crates to confirm each
tarball can be built, metadata is valid, and the file list is correct.
`native-theme-gpui` is treated as a soft check (warns but doesn't block)
because of the upstream `naga` 27.0.3 / `codespan-reporting` 0.12.0
incompatibility documented in `docs/todo_v0.5.7_gaps.md` §G11.

### Why `--no-verify`?

`cargo package` verification compiles each tarball as if it were downloaded
from crates.io. On the first-ever publication of a workspace with internal
path deps, the depended-on crate (`native-theme-derive`) is not yet indexed,
so that compilation can't succeed — cargo searches crates.io, doesn't find
the crate, errors out. `--no-verify` skips only the verification-compile
step; metadata and tarball creation are still validated.

**The real tarball-verification happens during `cargo publish` itself** (see
below). Once the first publish cycle seats `native-theme-derive 0.5.7` on
crates.io, `--no-verify` can be removed from the three lines near the bottom
of `pre-release-check.sh` so subsequent releases get full tarball-verify.

## Publish order

For a cold-start release (nothing on crates.io yet), publish in this order,
one at a time, waiting for the crates.io sparse index to update between each:

```bash
# 1. Proc-macro crate (no workspace-internal deps)
cargo publish -p native-theme-derive

# Wait ~30-60 seconds for the index to settle.

# 2. Codegen helpers (depends on native-theme-derive)
cargo publish -p native-theme-build

# Wait.

# 3. Core crate (depends on native-theme-derive AND native-theme-build)
cargo publish -p native-theme

# Wait.

# 4. Iced connector (depends on native-theme)
cargo publish -p native-theme-iced

# Wait.

# 5. GPUI connector (depends on native-theme; soft-gated)
cargo publish -p native-theme-gpui
```

Each `cargo publish` invocation performs the tarball-compile-against-registry
check that `pre-release-check.sh --no-verify` skips. Because prior crates are
indexed by the time the next `cargo publish` runs, those verifications
succeed. `cargo publish` is the real release-grade validation of the
workspace.

If any step fails: crates.io does not allow unpublishing, but versions can be
yanked. Publish a patch or bump to the next minor version rather than trying
to recover in place.

## Post-bootstrap cleanup

After the first successful publish cycle (i.e. `native-theme-derive 0.5.7` is
live on crates.io), remove `--no-verify` from the three lines near the bottom
of `pre-release-check.sh`:

```bash
# Before (bootstrap):
run_check "Validating packages (core)" cargo package --no-verify -p native-theme-derive -p native-theme -p native-theme-build --allow-dirty

# After (post-bootstrap):
run_check "Validating packages (core)" cargo package -p native-theme-derive -p native-theme -p native-theme-build --allow-dirty
```

From the next release onwards, the script will perform full tarball-verify
against the live registry because the prior version of `native-theme-derive`
is now resolvable from crates.io.

## Known upstream tool-chain deviations (v0.5.7+)

This section lists current principled deviations from the
"cargo test --workspace --all-features" ideal that are in effect
because of upstream issues outside this project's control. Each
deviation has a specific re-evaluation trigger; when the upstream
state changes, run the trigger command and delete the deviation
entry from this section.

### G11 · `cargo test --workspace --all-features` → per-crate posture (active since 2026-04-19)

**Status:** active. Accepted `2026-04-19T18:28:22Z` by `tiborgats`.

**Why:** `cargo test --workspace --all-features` fails to compile
due to `naga 27.0.3` (transitively via `gpui-component v0.5.1`) being
written against `codespan-reporting 0.11.x`'s `impl WriteColor for String`,
which `codespan-reporting 0.12.0` (workspace `Cargo.lock:1064-1067`)
removed. `naga 27.0.4` does not exist on crates.io; `naga 28.x`
would require a `gpui-component` version bump that hasn't shipped.

**Current release gate (what this script actually enforces):**

```bash
./pre-release-check.sh
```

The script runs `cargo test -p <crate>` per workspace crate
(the per-crate test loop starts at `pre-release-check.sh` line 288,
immediately below the `# Run all tests` comment at line 287, and ends
at line 294) and treats `native-theme-gpui` as soft via
`run_check_soft` (a failure becomes a warning, not a blocker). This
per-crate posture has been the release gate since Phase 14-03
(2026-03-09).

**Re-evaluation trigger command:**

```bash
cargo update -p gpui-component && cargo test --workspace --all-features
```

Run this when `gpui-component` ships a release that either (a) bumps
`naga` past 27.0.3 to a `codespan-reporting 0.12.0`-compatible version,
or (b) pins `codespan-reporting 0.11.x` in its own `Cargo.toml`.
If both succeed, this deviation is obsolete — delete this G11 entry.

Track upstream state at:

- `https://github.com/gfx-rs/wgpu/issues` (naga is part of wgpu)
- `https://github.com/zed-industries/zed` (gpui-component's upstream)

**Detailed records:**

- `docs/todo_v0.5.7_gaps.md:546-614` (§G11 — the definitive technical analysis of Options A/B/C/D).
- `.planning/phases/93-docs-todo-v0-5-7-gaps-md/93-G11-DEVIATION.md` (Phase-93-cross-plan annotation).
- `.planning/phases/93-docs-todo-v0-5-7-gaps-md/93-VERIFICATION.md` frontmatter `overrides` block (formal acceptance record).

**Note on the existing inline reference at `RELEASING.md:30`:** the
Pre-publication-checks prose already mentions §G11 inline (the
soft-check rationale for `native-theme-gpui`). This top-level section
does NOT replace that inline mention — both are retained. The inline
reference is first-contact context for someone reading the release
procedure top-to-bottom; this section is the structured, greppable
landing for anyone searching for deviations directly.

## Version bumps

Workspace version is declared once in the root `Cargo.toml` under
`[workspace.package]`. All five crates inherit it via
`version.workspace = true`. Update `version` in `[workspace.package]`,
commit, then run the publish sequence above.

The `CHANGELOG.md` and per-crate `README.md` badges should be updated in the
same commit as the version bump.

## Tagging

After all five `cargo publish` commands succeed:

```bash
git tag v0.5.7
git push origin v0.5.7
```

Push only the specific tag — never `git push --tags`.
