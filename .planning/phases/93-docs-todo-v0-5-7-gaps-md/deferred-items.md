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
