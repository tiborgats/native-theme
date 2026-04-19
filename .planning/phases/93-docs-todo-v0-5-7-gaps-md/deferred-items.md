# Deferred Items — Phase 93

Logged during Plan 01 execution (G1). Not fixed because out of scope per
executor Rule "SCOPE BOUNDARY".

## Doctest failures in native-theme/src/model/bundled.rs (lines 22 and 188)

**Root cause:** Plan 93-03 (commit `7ba2b4c`) demoted `bundled_icon_svg` and
`bundled_icon_by_name` to `pub(crate)`. The doctest examples on those
functions still `use native_theme::theme::bundled_icon_svg;`, which is now
private. Fix is to rewrite the doctests to use `IconLoader` (the public
replacement) or delete them if redundant. Belongs to Plan 93-03 follow-up
or to Plan 93-fix (not to 93-01).

## naga v27.0.3 workspace build error

**Root cause:** `naga` build error (`WriteColor` trait bound not satisfied)
on the upstream crate version pinned by `gpui-component`. Affects only
`cargo test --workspace` (which pulls in GPUI). Does not affect
`cargo test --package native-theme` or the G1 deliverable. Likely needs a
lockfile bump or workaround in GPUI; out of scope for Phase 93.

## Unrelated working-tree modifications (detected at start)

`native-theme/src/detect.rs`, `native-theme/src/model/icons.rs`,
`native-theme/src/pipeline.rs` had uncommitted edits (not made by this
plan) when Plan 01 started. These are presumably in-flight work from a
parallel plan and were left untouched.

## Logged during Plan 02 (G2) execution

### Dead-code clippy error in native-theme/src/model/bundled.rs:197

**Root cause:** Commit `7ba2b4c refactor(93-03): demote icon helper fns to
pub(crate)` left `bundled_icon_by_name` as `pub(crate)` but the crate no
longer has any internal caller (all call sites migrated to `IconLoader`).
`cargo clippy -D warnings` now fails with `function bundled_icon_by_name
is never used`. Fix belongs to Plan 93-03 follow-up or Plan 93-fix: either
delete the function outright, or add `#[allow(dead_code)]` if internal
reuse is expected later. Out of scope for Plan 93-02.

## Logged during Plan 04 (G4) execution

### pre-release-check.sh blocked by 93-03 dead-code item

**Observation:** Running `./pre-release-check.sh` after Plan 04 fails at
the `cargo clippy -p native-theme --all-targets -- -D warnings` step for
the exact same reason logged under Plan 02 above: `bundled_icon_by_name`
is never used. Plan 04 did not introduce the issue and did not fix it
(SCOPE BOUNDARY). All other steps of the script pass through successfully
(fmt, builds, checks for native-theme + both connectors). Once Plan 93-fix
or a 93-03 follow-up removes or annotates the unused function, the full
`pre-release-check.sh` will be green for Plan 04 output.

### Doctest failures in model/bundled.rs (still present)

**Observation:** The two doctest failures logged under Plan 01 persist.
Plan 04's `cargo test -p native-theme --all-features` output shows the
doctest run fails with 48 passed / 2 failed / 10 ignored. All lib tests
(782/782) and integration tests (79/79 across 8 test binaries) pass.
The 2 failing doctests are the `model::bundled::bundled_icon_svg` and
`model::bundled::bundled_icon_by_name` examples that still reference the
now-private `use native_theme::theme::bundled_icon_svg` / `use
native_theme::theme::bundled_icon_by_name` paths.

## Logged during Plan 05 (G5) execution

### pre-release-check.sh still blocked by 93-03 dead-code item

**Observation:** Running `./pre-release-check.sh` after Plan 05 still
fails at the `cargo clippy -p native-theme --all-targets -- -D warnings`
step for the SAME reason logged under Plan 02: `bundled_icon_by_name`
is never used. Plan 05 did not introduce the issue and did not fix it
(SCOPE BOUNDARY). Plan 05's own lib-targeted clippy
(`cargo clippy -p native-theme --all-features --lib -- -D warnings`)
is green. 791 lib tests + 79 integration tests all pass.

### Doctest failures in model/bundled.rs persist

**Observation:** Same 2 doctest failures on
`model::bundled::bundled_icon_svg` and
`model::bundled::bundled_icon_by_name` continue from Plan 01/04. Out of
scope for Plan 05's G5 deliverable.
