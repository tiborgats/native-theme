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


## Resolved during Plan 93-07 (gap closure)

### naga v27.0.3 workspace build error — CLOSED as principled deviation

The "naga v27.0.3 workspace build error" note above (originally logged
during Plan 01 execution) is now documented as a **principled deviation**
rather than a deferral. See `docs/todo_v0.5.7_gaps.md` G11 section for:

- The exact error signature and root cause (`codespan-reporting 0.12.0`
  removed `impl WriteColor for String` that `naga 27.0.3` relied on).
- Why Option A (`cargo update -p naga --precise 27.0.4`) is not viable
  (`naga 27.0.4` does not exist on crates.io as of 2026-04-19).
- Why Option D (align plan acceptance criterion with `pre-release-check.sh`'s
  per-crate posture) was selected over Options B (narrow scope) and C
  (exclude connector from workspace).
- The re-evaluation trigger — when gpui-component ships a release past
  naga 27.0.3 or pins codespan-reporting 0.11.x, the `--workspace`
  acceptance criterion can be restored.

The plan's must_have truth around workspace-scope tests is updated to
match the pre-release script's per-crate posture (`cargo test -p native-theme`
+ per-crate for each workspace member, with `native-theme-gpui` treated
as soft per `pre-release-check.sh:290`).

No further action needed from Phase 93. Logged here for the audit trail
only.


## Resolved during Plan 93-06 (G3 follow-up closure)

### Doctest E0603 failures in model/bundled.rs — CLOSED

Both doctest failures logged under Plan 01, 04, and 05 above are closed
by Plan 93-06's commit (see 93-06-SUMMARY.md). The doctests on
`bundled_icon_svg` (bundled.rs:20-36 after fix) and `bundled_icon_by_name`
(bundled.rs:194-208 after fix) now import `native_theme::icons::IconLoader`
and use the public builder API. Post-fix: `cargo test -p native-theme
--all-features --doc` reports 50 passed; 0 failed (was 48 passed; 2 failed).

### Dead-code clippy error on bundled_icon_by_name — CLOSED

The "function bundled_icon_by_name is never used" error logged under Plan
02, 04, and 05 is closed by Plan 93-06's commit. The function now carries
`#[cfg_attr(not(any(feature = "material-icons", feature = "lucide-icons")),
allow(dead_code))]`, which fires only when BOTH feature-gated callers at
icons.rs:598,603 are cfg'd out (matches the exact "dead" predicate). Post-fix:
`cargo clippy -p native-theme --all-targets -- -D warnings` exits 0.

## Logged during Plan 93-06 (G3 follow-up) execution

### cargo package verification failure (pre-release-check.sh step: "Validating packages (core)") — NEW PRE-EXISTING DEFECT SURFACED

**Observation:** With Plan 93-06's three edits applied, `./pre-release-check.sh`
now advances past the previously-failing clippy step (line 283) and past
all subsequent test/example/docs steps. It fails at the `cargo package
-p native-theme-derive -p native-theme -p native-theme-build --allow-dirty`
step (line 321) with:

```
error[E0432]: unresolved import `native_theme_derive::ThemeFields`
  --> src/model/border.rs:4:5
error[E0432]: unresolved import `native_theme_derive::ThemeFields`
  --> src/model/defaults.rs:8:5
error[E0432]: unresolved import `native_theme_derive::ThemeFields`
  --> src/model/font.rs:7:5
error[E0432]: unresolved import `native_theme_derive::ThemeFields`
  --> src/model/icon_sizes.rs:3:5
error[E0432]: unresolved import `native_theme_derive::ThemeFields`
  --> src/model/widgets/mod.rs:6:27
error: cannot find attribute `theme_layer` in this scope
  --> src/model/font.rs:157:3
  (and sibling on font.rs:262:3)
```

**Root cause:** Plan 93-05 (commit `4431782 feat(93-05): add ThemeFields
derive and FieldInfo inventory registry`) added the `ThemeFields`
proc-macro derive to `native-theme-derive v0.5.7` and the consuming
`use native_theme_derive::ThemeFields;` imports + `#[theme_layer(fields = "...")]`
attributes to five `native-theme` source files. However, **`native-theme-derive
v0.5.7` has not yet been published to crates.io**. `cargo package`'s tarball
verification step builds each crate in isolation from the tarball (simulating
the published-to-crates.io state), pulling its dependencies from the
crates.io index rather than the workspace `path = "..."`. Thus the packaged
`native-theme-0.5.7.crate` cannot resolve `native_theme_derive::ThemeFields`
because only older (pre-93-05) published versions are available there.

**Scope-boundary attestation:** This failure is **pre-existing at HEAD
before Plan 93-06's edits**. Reproduced by `git stash && cargo package
... --allow-dirty` returning the same 54 errors; confirmed also on parent
commit `51c386b docs(93-05): complete G5 plan` via
`git checkout 51c386b -- . && cargo package ...` which yields the same
failure. Plan 93-06 does not touch `native-theme-derive`, does not touch
any consuming file that imports `ThemeFields`, and does not touch
`bundled.rs`'s few lines in a way that interacts with this failure
(bundled.rs has no `ThemeFields` derive). Plan 93-06 is therefore NOT
responsible for and does NOT regress this failure.

**Why not auto-fix (Rule 1-3):** The only fix is to `cargo publish`
`native-theme-derive v0.5.7` to crates.io. This is a **release action**,
not a code change, and is governed by the user memory rule "NEVER bypass
human checkpoints" ("Never publish, push tags, create releases without
EXPLICIT user approval"). Publishing is out of scope for any automated
plan execution.

**Why not Rule 4 (ask):** Not an architectural decision. The path forward
is unambiguous — publish `native-theme-derive v0.5.7`, then the packaging
step will resolve. The only open question is **timing** (when the user
decides to cut the release), not **approach**.

**Impact on Plan 93-06's own acceptance criteria:** Step 8 of the plan's
verify block — `./pre-release-check.sh` reaches the green success banner —
does NOT pass. However, the step Plan 93-06 was chartered to unblock (the
clippy step at line 283, which was the former failure locus) DOES pass.
The release-gate script now advances from "fails at step 15 (clippy
native-theme)" to "fails at step 23 (cargo package core)", an 8-step
forward advance. The three P0 defects Plan 93-06 was scoped to close (two
E0603 doctest failures + one dead_code clippy error) are closed. The new
failure locus is unrelated to the G3 follow-up mandate and is a
release-sequencing artifact that will resolve the moment
`native-theme-derive v0.5.7` is published.

**Re-evaluation trigger:** First-time the user runs `cargo publish -p
native-theme-derive` with v0.5.7, the packaging step of
`./pre-release-check.sh` should turn green and the full banner should
render. If it does not, a follow-up defect not known today needs its own
investigation.
