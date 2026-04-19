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
