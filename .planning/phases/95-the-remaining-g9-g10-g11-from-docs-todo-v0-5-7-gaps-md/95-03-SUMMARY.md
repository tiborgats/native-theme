---
phase: 95-the-remaining-g9-g10-g11-from-docs-todo-v0-5-7-gaps-md
plan: 03
subsystem: docs-planning
tags: [docs, phase-93, principled-deviation, naga, gpui-component, release-gate, gap-closure, cross-plan-annotation, append-only]
requirements: [G11]

provides:
  - name: "93-G11-DEVIATION.md"
    description: "Cross-plan single-source-of-truth record for the Phase-93-wide --workspace deviation (Options A/B/C/D analysis, Option D selection, re-evaluation trigger, implicit-supersession clarifier)."
  - name: "G11 trailing annotation blocks on five Phase 93 PLAN files"
    description: "APPEND-ONLY labelled blocks (2026-04-20 marker) below each existing </output> tag, directing readers to the DEVIATION.md and docs/todo_v0.5.7_gaps.md §G11."
  - name: "RELEASING.md 'Known upstream tool-chain deviations (v0.5.7+)' section"
    description: "Structured top-level landing for G11 with precise pre-release-check.sh line-range citation (288-294) and re-evaluation trigger command."

requires:
  - docs/todo_v0.5.7_gaps.md:546-614 (§G11 — definitive detailed record)
  - .planning/phases/93-docs-todo-v0-5-7-gaps-md/93-VERIFICATION.md (frontmatter overrides block, accepted_by: tiborgats, 2026-04-19T18:28:22Z)
  - .planning/phases/93-docs-todo-v0-5-7-gaps-md/deferred-items.md (Phase 93-07 resolution section)
  - pre-release-check.sh:287-294 (the # Run all tests comment at :287; loop body 288-294)

affects:
  - Phase 93 planning legibility (readers of 93-01..93-05 PLAN files now see the accepted deviation)
  - Release maintainer workflow (RELEASING.md gains structured greppable deviation section)

key-files:
  created:
    - .planning/phases/93-docs-todo-v0-5-7-gaps-md/93-G11-DEVIATION.md
    - .planning/phases/95-the-remaining-g9-g10-g11-from-docs-todo-v0-5-7-gaps-md/95-03-SUMMARY.md
  modified:
    - .planning/phases/93-docs-todo-v0-5-7-gaps-md/93-01-PLAN.md (append G11 annotation block only; existing content verbatim)
    - .planning/phases/93-docs-todo-v0-5-7-gaps-md/93-02-PLAN.md (append G11 annotation block only; existing content verbatim)
    - .planning/phases/93-docs-todo-v0-5-7-gaps-md/93-03-PLAN.md (append G11 annotation block only; existing content verbatim)
    - .planning/phases/93-docs-todo-v0-5-7-gaps-md/93-04-PLAN.md (append G11 annotation block only; existing content verbatim)
    - .planning/phases/93-docs-todo-v0-5-7-gaps-md/93-05-PLAN.md (append G11 annotation block only; existing content verbatim)
    - RELEASING.md (insert new 'Known upstream tool-chain deviations (v0.5.7+)' section between Post-bootstrap cleanup and Version bumps; existing :30 inline reference preserved verbatim)

decisions:
  - "Option D (principled deviation; align must_have with ./pre-release-check.sh per-crate posture) was already selected and accepted during Phase 93-07; Phase 95-03 formalises the accepted decision into discoverable places rather than creating a new decision."
  - "Affected-plans table lists the CANONICAL must_haves.truths line per plan; ALL other --workspace occurrences in <automated>/verification blocks are implicitly superseded. Chose Option B per HIGH 2 checker finding — simpler than runtime grep census, keeps table compact, avoids hardcoded counts that could drift."
  - "New file (93-G11-DEVIATION.md) created instead of reusing deferred-items.md because deferred-items.md is a log of deferred-during-execution items; G11 is NOT deferred — it is an accepted principled deviation. The two categories differ semantically; conflating them would muddy the audit trail."
  - "RELEASING.md section promoted the existing inline G11 reference at :30 to a structured top-level landing (between Post-bootstrap cleanup and Version bumps) so re-evaluation trigger + cross-references are in one canonical greppable place. Inline reference at :30 retained verbatim for first-contact top-to-bottom readers."
  - "Precise citation discipline: pre-release-check.sh loop body is lines 288-294; line 287 is the `# Run all tests` comment. Both 93-G11-DEVIATION.md and RELEASING.md cite this range correctly. Stale `lines 287-294` citation in docs/todo_v0.5.7_gaps.md:576 flagged as follow-up annotation candidate (OUT of this plan's scope; touching the gaps doc was not in files_modified)."
  - "Files are gitignored under .planning/ by project policy (.gitignore:32). Force-added the 5 Phase 93 PLAN files + 93-G11-DEVIATION.md along with RELEASING.md in a single atomic commit — matches the tracking pattern established by Phase 93-07's commit a6e8d4e (which force-added deferred-items.md alongside docs/todo_v0.5.7_gaps.md). APPEND-ONLY discipline verified pre-staging by `git diff | grep -c '^-[^-]'` returning 0 for each of the 5 files."

metrics:
  duration_minutes: ~45
  files_created: 2   # 93-G11-DEVIATION.md + 95-03-SUMMARY.md
  files_modified: 6  # 5 Phase 93 PLAN files + RELEASING.md
  commit_count: 1    # atomic commit 10028ec
  completed_date: 2026-04-20
---

# Phase 95 Plan 03: Formalise G11 Principled Deviation Across Phase 93 Plans + RELEASING.md Summary

**One-liner:** Closed G11 from docs/todo_v0.5.7_gaps.md (lines 546-614) by creating a cross-plan single-source-of-truth DEVIATION.md record, appending APPEND-ONLY annotation blocks to five Phase 93 PLAN files that still carry `cargo test --workspace --all-features` as a must_have or verification criterion, and promoting the existing inline G11 reference at RELEASING.md:30 to a structured top-level "Known upstream tool-chain deviations (v0.5.7+)" section with the exact re-evaluation trigger command — all in one atomic docs-only commit (10028ec), zero source code changes, green banner unchanged.

## Objective Restated

Phase 93-07 (commit a6e8d4e, 2026-04-19) landed the G11 deviation record at `docs/todo_v0.5.7_gaps.md:546-614` and Phase 93-VERIFICATION.md formally accepted it via `overrides_applied` on 2026-04-19T18:28:22Z by `tiborgats`. What remained missing was:

1. A cross-plan single-source-of-truth file listing all five Phase 93 PLAN files (93-01..93-05) that still carry raw `--workspace` claims, contextualised for plan-file readers.
2. In-file annotations on each of the five PLAN files so a reader discovering any plan for the first time can see the accepted deviation without cross-referencing multiple other files.
3. A structured top-level "Known upstream tool-chain deviations" section in RELEASING.md that makes G11 first-class at the release-gate boundary.

## The 7 Files in the Atomic Commit

All seven files landed in **commit `10028ec` — `docs(95-03): formalise G11 principled deviation across Phase 93 plans + RELEASING.md`** (2176 insertions, 0 deletions).

| File | Change | Rationale |
|------|--------|-----------|
| `.planning/phases/93-docs-todo-v0-5-7-gaps-md/93-G11-DEVIATION.md` | NEW | Single-source-of-truth cross-plan record with Options A/B/C/D summary, Option D selection, re-evaluation trigger, affected-plans table, implicit-supersession clarifier, and cross-references to all five related records (gaps.md §G11, VERIFICATION.md overrides, deferred-items.md, RELEASING.md new section, 93-07-PLAN.md, 93-07-SUMMARY.md). |
| `.planning/phases/93-docs-todo-v0-5-7-gaps-md/93-01-PLAN.md` | APPEND (G11 annotation block below existing `</invoke>` trailing artifact) | Plan 93-01 claims `--workspace` at must_haves line 23 + four verification-block occurrences (206, 263, 269, 291). Annotation block labelled "G11 annotation added 2026-04-20 (Phase 95 Plan 03)" directs readers to the DEVIATION.md and gaps.md §G11. |
| `.planning/phases/93-docs-todo-v0-5-7-gaps-md/93-02-PLAN.md` | APPEND (G11 annotation block below existing `</output>` tag) | Plan 93-02 claims `--workspace` at verification-block lines 184, 211, 212. Same annotation block pattern. |
| `.planning/phases/93-docs-todo-v0-5-7-gaps-md/93-03-PLAN.md` | APPEND (G11 annotation block below existing `</invoke>` trailing artifact) | Plan 93-03 claims `--workspace` at seven verification-block occurrences (291, 292, 300, 307, 308, 329, 330). Same annotation block pattern. |
| `.planning/phases/93-docs-todo-v0-5-7-gaps-md/93-04-PLAN.md` | APPEND (G11 annotation block below existing `</invoke>` trailing artifact) | Plan 93-04 claims `--workspace` at must_haves line 41 + five verification-block occurrences (285, 353, 354, 374, 403). Same annotation block pattern. |
| `.planning/phases/93-docs-todo-v0-5-7-gaps-md/93-05-PLAN.md` | APPEND (G11 annotation block below existing `</invoke>` trailing artifact) | Plan 93-05 claims `--workspace` at must_haves line 30 + three verification-block occurrences (278, 447, 475). Same annotation block pattern. |
| `RELEASING.md` | INSERT new section between `## Post-bootstrap cleanup` and `## Version bumps` | New section "Known upstream tool-chain deviations (v0.5.7+)" with G11 sub-section citing the precise `pre-release-check.sh` line range (288-294, immediately below the `# Run all tests` comment at line 287), the re-evaluation-trigger command, and cross-references to gaps.md §G11 + DEVIATION.md + VERIFICATION.md overrides block. The existing inline G11 reference at `:30` (Pre-publication-checks prose) is preserved verbatim. |

## APPEND-ONLY Discipline Verified

For all five Phase 93 PLAN files, existing content (frontmatter `must_haves`, objective, tasks, verification, success_criteria, output) was preserved VERBATIM. The G11 annotation was appended BELOW the existing closing `</output>` tag (and its trailing `</content></invoke>` artifacts for 93-01, 93-03, 93-04, 93-05 — those XML artifacts from past tool calls were themselves preserved verbatim).

Verification before staging (pre-commit working-tree diff):

```
for f in 93-01-PLAN.md 93-02-PLAN.md 93-03-PLAN.md 93-04-PLAN.md 93-05-PLAN.md; do
  git diff -- .planning/phases/93-docs-todo-v0-5-7-gaps-md/$f | grep -c '^-[^-]'
done
# Expected: 0 for all 5 files
# Actual:   0 for all 5 files ✓
```

Post-commit semantic spot-check — each of the five PLAN files retains its full existing structure:

```
grep -c "must_haves:" 93-0N-PLAN.md      # 1 per file (frontmatter preserved)
grep -c "<verification>" 93-0N-PLAN.md   # 1 per file (verification block preserved)
grep -c "<success_criteria>" 93-0N-PLAN.md  # 1 per file (success criteria preserved)
grep -c "G11 annotation added 2026-04-20" 93-0N-PLAN.md  # 1 per file (annotation added)
```

All assertions passed.

## RELEASING.md `:30` Inline Reference Preserved

The existing inline G11 reference at `RELEASING.md:30` ("native-theme-gpui is treated as a soft check (warns but doesn't block) because of the upstream `naga` 27.0.3 / `codespan-reporting` 0.12.0 incompatibility documented in `docs/todo_v0.5.7_gaps.md` §G11.") was preserved VERBATIM. Post-commit spot-check:

```
sed -n '28,30p' RELEASING.md
# → `native-theme-gpui` is treated as a soft check (warns but doesn't block)
# → because of the upstream `naga` 27.0.3 / `codespan-reporting` 0.12.0
# → incompatibility documented in `docs/todo_v0.5.7_gaps.md` §G11.
```

Matches the pre-edit content word-for-word. The new top-level section was inserted between `## Post-bootstrap cleanup` (existing) and `## Version bumps` (existing), leaving the Pre-publication-checks prose (lines 9-30) untouched.

## Pre-release-check.sh Green Banner Unchanged

```
./pre-release-check.sh 2>&1 | grep -E "(🎉|ready for release|FAILED|❌)"
# → 🎉 All pre-release checks passed successfully!
# → native-theme v0.5.7 is ready for release.
```

Docs-only plan = zero impact on code gates. Same green banner as pre-edit baseline. Publish order per the script's "Next steps": `cargo publish -p native-theme-derive` first, then the remaining four crates. NO automated publishing without EXPLICIT user approval per user memory rule `feedback_never_bypass_checkpoints.md`.

## Commit Discipline

- Atomic commit (single `git commit`) covering exactly 7 files — matches plan success_criteria.
- Files staged individually via explicit `git add --force` (6 files under `.planning/`, which is gitignored per `.gitignore:32`) + `git add` for `RELEASING.md` (tracked). Never used `git add -A` or `git add .` — matches Phase 93-07's (commit a6e8d4e) pattern of force-adding gitignored planning docs alongside tracked docs.
- Commit message: `docs(95-03): formalise G11 principled deviation across Phase 93 plans + RELEASING.md` + body listing all 7 files.
- Zero `Co-Authored-By` trailer — grep confirms 0 matches on `git log -1 --format=%B`.
- Separate from the modifications-in-progress in the working tree from parallel plans 95-01 (native-theme/src/lib.rs, docs/todo_v0.5.7_native-theme-api-2.md) and 95-02 — my commit only includes files in my declared scope.

## Deviations from Plan

None. Plan executed exactly as written.

The earlier PLAN draft reviewed by HIGH 2 checker already incorporated Option B (canonical-line table + implicit-supersession clarifier) over the alternative of a full runtime grep census; no further deviations during execution.

## Known Stubs

None. This is a docs-only plan with no code paths.

## Follow-up Annotation Candidate

`docs/todo_v0.5.7_gaps.md:576` cites the `pre-release-check.sh` per-crate test loop as `lines 287-294`. The loop body is actually `lines 288-294` — line 287 is the `# Run all tests` comment immediately above the loop. This is the same citation-drift pattern that Phase 95-01 corrected in `native-theme/src/lib.rs:347` → `:371`+`:387` via append-only doc comment, and that Phase 95-02 is flagging for `docs/todo_v0.5.7_gaps.md:537`'s stale `windows.rs:557` citation.

**Correction shape for a future plan:** append a short "Citation correction (added YYYY-MM-DD)" note below line 614 of `docs/todo_v0.5.7_gaps.md` noting that the Option D rationale's loop-range citation (`lines 287-294`) elides the `# Run all tests` comment at line 287; the loop body is lines 288-294. No edit to existing content. Same APPEND-ONLY discipline.

**Why it was NOT fixed in this plan:** `docs/todo_v0.5.7_gaps.md` is NOT in `files_modified` per the 95-03 PLAN frontmatter. Touching it here would have violated scope. This flag is the honest hand-off to a follow-up plan.

## Verification Checklist — All Plan Success Criteria Met

- [x] `.planning/phases/93-docs-todo-v0-5-7-gaps-md/93-G11-DEVIATION.md` exists with affected-plans table (5 canonical anchors), implicit-supersession clarifier, Options A/B/C/D summary, Option D selection, acceptance criterion with precise `pre-release-check.sh` lines 288-294 citation, re-evaluation trigger command, and all cross-references.
- [x] Each of 93-01/02/03/04/05-PLAN.md has an APPEND-ONLY G11 annotation block labelled "G11 annotation added 2026-04-20 (Phase 95 Plan 03)" below the existing `</output>` tag.
- [x] Existing content in the five Phase 93 PLAN files preserved VERBATIM (0 non-header deletion lines per file).
- [x] RELEASING.md has a new section "Known upstream tool-chain deviations (v0.5.7+)" listing G11 with re-evaluation trigger and precise line-range citation.
- [x] Existing inline G11 reference at RELEASING.md:30 preserved VERBATIM.
- [x] RELEASING.md existing sections (Pre-publication checks, Publish order, Post-bootstrap cleanup, Version bumps, Tagging) all preserved (5 `^## ` matches).
- [x] One atomic git commit covers exactly 7 files (1 new DEVIATION + 5 Phase 93 PLAN files + RELEASING.md).
- [x] Commit message matches template; no `Co-Authored-By` trailer.
- [x] `./pre-release-check.sh` green banner unchanged.
- [x] Zero Rust code, preset, Cargo.toml, or Cargo.lock changes.
- [x] No stale `lines 287-294` citation in any file this plan touches — loop body correctly cited as `lines 288-294` in both DEVIATION.md and RELEASING.md.

## Self-Check: PASSED

**Created files exist:**
- FOUND: `/home/tibi/Rust/native-theme/.planning/phases/93-docs-todo-v0-5-7-gaps-md/93-G11-DEVIATION.md` (7510 bytes, 135 lines)
- FOUND: `/home/tibi/Rust/native-theme/.planning/phases/95-the-remaining-g9-g10-g11-from-docs-todo-v0-5-7-gaps-md/95-03-SUMMARY.md` (this file)

**Modified files exist with expected content:**
- FOUND: `93-01-PLAN.md` (grep "G11 annotation added 2026-04-20" → 1 match; grep "must_haves:" → 1 match)
- FOUND: `93-02-PLAN.md` (grep "G11 annotation added 2026-04-20" → 1 match; grep "must_haves:" → 1 match)
- FOUND: `93-03-PLAN.md` (grep "G11 annotation added 2026-04-20" → 1 match; grep "must_haves:" → 1 match)
- FOUND: `93-04-PLAN.md` (grep "G11 annotation added 2026-04-20" → 1 match; grep "must_haves:" → 1 match)
- FOUND: `93-05-PLAN.md` (grep "G11 annotation added 2026-04-20" → 1 match; grep "must_haves:" → 1 match)
- FOUND: `RELEASING.md` (grep "Known upstream tool-chain deviations" → 1 match; grep "line 288" → 1 match; grep "incompatibility documented in" → 1 match at :30)

**Commit exists:**
- FOUND: commit `10028ec` on `main` — `docs(95-03): formalise G11 principled deviation across Phase 93 plans + RELEASING.md` (7 files, 2176 insertions)

**Pre-release-check green banner:**
- FOUND: `🎉 All pre-release checks passed successfully!`
- FOUND: `native-theme v0.5.7 is ready for release.`

All expectations met. G11 formalisation complete.
