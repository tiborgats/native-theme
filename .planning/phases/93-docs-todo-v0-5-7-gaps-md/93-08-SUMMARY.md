---
phase: 93-docs-todo-v0-5-7-gaps-md
plan: 08
completed: 2026-04-19T21:05:00Z
atomic_commits:
  - 2f4c8a5  # docs(93-08): fix pre-release-check bootstrap + document ordered-publish workflow
tasks_completed: 4/4
gap_closure: true
requirements_closed: [G11-followup]
---

# Plan 93-08 Summary

## Objective

Close the release-gate gap identified by the phase-93 re-verification: `./pre-release-check.sh` was red at line 321 (`Validating packages (core)`) because the `cargo package` invocation failed tarball verification for the unpublished workspace-internal crate `native-theme-derive`. The original VERIFICATION.md classified this as `human_needed` with incorrect guidance to publish the derive first. This plan applied the correct fix (release-script level) and documented the ordered-publish workflow properly.

## What Landed

### 1. `pre-release-check.sh` lines 320-335 (+16/-4)

Replaced the single-line comment and three bare `cargo package` invocations with a twelve-line comment block + `--no-verify` on all three `cargo package` calls:

```bash
# Validate packages before publishing.
#
# --no-verify rationale: cargo package verification compiles each tarball as if it
# were downloaded from crates.io. For first-ever publication of this workspace, the
# internal proc-macro crate native-theme-derive is not yet on the registry, so
# tarball verification cannot resolve the workspace-internal dep. The real tarball
# compilation check happens during `cargo publish` itself, which this script does
# not run. See RELEASING.md for the ordered publish workflow (derive -> build ->
# core -> connectors).
#
# Once native-theme-derive 0.5.7 is published to crates.io, remove --no-verify
# from these three lines to restore full tarball-compile verification for
# subsequent releases.
run_check "Validating packages (core)" cargo package --no-verify -p native-theme-derive -p native-theme -p native-theme-build --allow-dirty
run_check_soft "Validating package (iced connector)" cargo package --no-verify -p native-theme-derive -p native-theme -p native-theme-iced --allow-dirty
run_check_soft "Validating package (gpui connector)" cargo package --no-verify -p native-theme-derive -p native-theme -p native-theme-gpui --allow-dirty
```

### 2. `RELEASING.md` — new, 114 lines, 6 sections

Root-level doc covering: pre-publication checks, why `--no-verify` on bootstrap, cold-start publish order (derive → build → core → iced → gpui), post-bootstrap cleanup condition, version-bump workflow, tagging (single-tag push per user rule; no `git push --tags`).

### 3. `.planning/phases/93-docs-todo-v0-5-7-gaps-md/93-VERIFICATION.md`

Two targeted frontmatter edits + one append:
- Line 4: `status: human_needed` → `status: passed`.
- Inside `re_verification:` block: added `superseded_by: plan-93-08` field.
- Appended `## Update 2026-04-19 (Plan 93-08)` section at the file tail explaining the correction (with `human_verification:` block above preserved for audit trail). Zero narrative paragraphs modified.

### 4. `README.md`

Appended a new `## Release` section of 4 lines before `## License` pointing to `RELEASING.md`. No existing paragraph modified.

## Final Verification

```
$ ./pre-release-check.sh 2>&1 | tail -6
unicode-ident  1.0.24   Removed  Removed  Normal       ---

🎉 All pre-release checks passed successfully!
native-theme v0.5.7 is ready for release.

Next steps:
```

All 25+ per-crate steps green (check, fmt, clippy, tests, examples, docs, cargo package) plus cargo audit. Zero red markers. Exit code 0.

## Regression Check

- `cargo test -p native-theme --all-features --lib` — 791 passed, 0 failed (unchanged).
- `cargo test -p native-theme --all-features --doc` — 50 passed, 0 failed (unchanged).
- `cargo test -p native-theme --all-features --tests` — 79 passed across 8 binaries (unchanged).
- `cargo clippy -p native-theme --all-targets -- -D warnings` — exit 0 (unchanged).
- No Rust file modified in this plan.
- No Cargo.toml or Cargo.lock modified.

## Deviations

None. Plan body was followed verbatim (comment block wording, three `--no-verify` edits, RELEASING.md structure, VERIFICATION.md single-field edits + append).

## Decisions Recorded

See STATE.md v0.5.7 Decisions section for the new Phase 93-08 entries documenting: the corrected diagnosis, the rejection of Option C (local registry via `cargo vendor`), and the APPEND-ONLY discipline on VERIFICATION.md.

## Key Files

- `pre-release-check.sh` — bootstrap-safe package validation (line 320-335)
- `RELEASING.md` — ordered publish workflow + bootstrap cleanup
- `.planning/phases/93-docs-todo-v0-5-7-gaps-md/93-VERIFICATION.md` — corrected frontmatter + Update section
- `README.md` — Release section added

## Commit

- `2f4c8a5` — `docs(93-08): fix pre-release-check bootstrap + document ordered-publish workflow`
  - 4 files changed, 375 insertions(+), 4 deletions(-)
  - No `Co-Authored-By` trailer.
