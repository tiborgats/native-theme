---
phase: 93-docs-todo-v0-5-7-gaps-md
plan: 07
subsystem: docs
tags: [docs, upstream, naga, gpui-component, codespan-reporting, principled-deviation, v057-polish, gap-closure, audit-trail]

# Dependency graph
requires:
  - phase: 93-01
    provides: First deferred-items.md entry ("naga v27.0.3 workspace build error" logged during G1 execution) that this plan now formally closes as a principled deviation
  - phase: 93-02
    provides: Second deferred-items.md entry reinforcing the same naga upstream defect as a cross-cutting blocker
  - phase: 93-03
    provides: Third deferred-items.md entry (dead-code + doctest fallout) — illustrates that the upstream naga issue is separable from the in-scope fixes
  - phase: 93-04
    provides: Fourth deferred-items.md entry (pre-release-check.sh blocked chain) establishing the script as the operative release gate
  - phase: 93-05
    provides: Fifth deferred-items.md entry confirming the upstream defect survives every prior gap-closure plan
provides:
  - "§G11 section in docs/todo_v0.5.7_gaps.md: principled-deviation record for the naga 27.0.3 / codespan-reporting 0.12.0 / gpui-component v0.5.1 workspace-test block, with Options A/B/C/D enumerated and Option D (align plan acceptance criterion with ./pre-release-check.sh per-crate posture) selected"
  - "Cross-reference in .planning/phases/93-docs-todo-v0-5-7-gaps-md/deferred-items.md linking the original Plan 01 'naga v27.0.3 workspace build error' entry to the new G11 decision"
  - "Summary bullet in gaps.md Ship plan section noting G11 closes without an additional phase"
  - "Re-evaluation trigger captured: when gpui-component ships past naga 27.0.3 or pins codespan-reporting 0.11.x, the --workspace acceptance criterion returns"
affects: [93-verifier, 94-phase-planning, 95-docs-sync, v057-release-gate, future-gpui-component-upgrades]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Principled-deviation documentation pattern: when a must_have cannot be met due to upstream defect, enumerate alternative options with evidence-backed rejection rather than silently narrowing scope. Aligns with user memory rule 'NEVER LIE, NEVER INVENT'."
    - "Audit-trail preservation: deferred-items.md entries are appended-to (not rewritten) when resolved, so the audit trail from original logging through final resolution remains intact."

key-files:
  created:
    - docs/todo_v0.5.7_gaps.md (first commit to git history — file was previously untracked despite five phase-93 plans referencing it)
  modified:
    - .planning/phases/93-docs-todo-v0-5-7-gaps-md/deferred-items.md (APPEND-ONLY cross-reference section)

key-decisions:
  - "[Phase 93-07]: Option D (principled deviation) selected over Options A/B/C per evidence: Option A impossible (naga 27.0.4 does not exist on crates.io as of 2026-04-19, verified via `cargo info naga@27.0.4` returning 'could not find'); Option B weak (scope narrowing reads as hiding the problem); Option C worse (excluding connectors/native-theme-gpui from [workspace] members breaks developer ergonomics and propagates upstream defect into project layout)."
  - "[Phase 93-07]: Acceptance criterion realignment — Phase 93 must_have truth #5 ('cargo test --workspace --all-features still passes') is replaced by the per-crate equivalent tied to ./pre-release-check.sh lines 267-294 (cargo test -p native-theme + per-crate for each workspace member, with native-theme-gpui treated as soft per pre-release-check.sh:290). This matches the release gate since Phase 14-03 (2026-03-09)."
  - "[Phase 93-07]: Root cause anchored on codespan-reporting 0.12.0 (Cargo.lock:1064-1067) removal of impl WriteColor for String, with naga 27.0.3 still referencing that trait impl via codespan_reporting::term::emit. Root cause lives in gpui-component (not native-theme); native-theme cannot fix it without forking gpui-component."
  - "[Phase 93-07]: Re-evaluation trigger documented inline: when gpui-component ships a release that bumps naga past 27.0.3 to a codespan-reporting 0.12.0-compatible version, or pins codespan-reporting 0.11.x, run `cargo update -p gpui-component && cargo test --workspace --all-features`. If green, deviation is obsolete and --workspace criterion returns."
  - "[Phase 93-07]: Two-file atomic commit (docs/todo_v0.5.7_gaps.md + .planning/phases/93-docs-todo-v0-5-7-gaps-md/deferred-items.md) with no Co-Authored-By trailer per user memory rule 'No Co-Authored-By trailers'."

patterns-established:
  - "Principled-deviation pattern: when an upstream defect makes a plan must_have impossible to satisfy, append a G-numbered section to the gap doc that (a) states the exact error signature, (b) cites the root cause with version specificity, (c) enumerates considered alternatives with evidence-backed rejection, (d) names the selected option, (e) captures a re-evaluation trigger. Never silently narrow scope."
  - "APPEND-ONLY audit trail: deferred-items.md entries are never rewritten or deleted, even when resolved. Resolution is documented by appending a 'Resolved during Plan XX' section cross-referencing the decision record. The audit trail shows who discovered the issue, when it was logged, and how it was ultimately closed."

# Metrics
duration: 3m 17s
completed: 2026-04-19
---

# Phase 93 Plan 07: Document naga / --workspace Principled Deviation (G11) Summary

**Two-file atomic doc commit closing Phase 93's workspace-test gap (G-3b) as Option D: the `--workspace` must_have is reconciled to the per-crate posture `./pre-release-check.sh` already enforces, with the upstream naga 27.0.3 / codespan-reporting 0.12.0 / gpui-component v0.5.1 incompatibility documented as the root cause and a re-evaluation trigger captured for when gpui-component upgrades.**

## Performance

- **Duration:** 3m 17s
- **Started:** 2026-04-19T18:25:05Z
- **Completed:** 2026-04-19T18:28:22Z
- **Tasks:** 2 (Task 93-07.01 and Task 93-07.02 delivered as a single atomic commit per plan instructions)
- **Files modified:** 2 (both `.md`; zero `.rs`, zero `Cargo.toml`, zero `Cargo.lock` changes)

## Accomplishments

- Added §G11 "Principled deviation — `cargo test --workspace` vs `./pre-release-check.sh`" section to `docs/todo_v0.5.7_gaps.md` between G10 and the "Not in this list — intentional non-gaps" boundary. The section documents: (a) the exact trait-bound error signature at `naga-27.0.3/src/front/wgsl/error.rs:113` and `naga-27.0.3/src/span.rs:326`, (b) the root cause (codespan-reporting 0.12.0 dropped `impl WriteColor for String`), (c) Options A/B/C enumerated and rejected with evidence (Option A: `naga 27.0.4` does not exist on crates.io; Option B: scope narrowing reads as hiding the problem; Option C: excluding native-theme-gpui from workspace breaks developer ergonomics), (d) Option D (align plan acceptance criterion with `./pre-release-check.sh` per-crate posture) selected and justified by citing the script's actual lines 287-294 for the per-crate test loop and lines 50-62 for the `run_check_soft` wrapper that treats `native-theme-gpui` as a warning-only crate.
- Appended cross-reference section "Resolved during Plan 93-07 (gap closure) — naga v27.0.3 workspace build error — CLOSED as principled deviation" to `.planning/phases/93-docs-todo-v0-5-7-gaps-md/deferred-items.md` linking the original Plan 01 entry to the new G11 decision. The cross-reference summarises the root-cause citation, Option A non-viability, Option D justification, and re-evaluation trigger so future auditors reading deferred-items.md can follow the trail without context-switching.
- Added "G11 — naga / --workspace" summary bullet to the `gaps.md` "Ship plan" section noting G11 closes without requiring an additional phase.
- Created a single atomic commit (`a6e8d4e`) carrying both markdown files with the exact message specified in the plan body, no `Co-Authored-By` trailer.

## Task Commits

Plan 93-07 delivered its two tasks as a single atomic commit per plan instructions (Task 2's action block explicitly says "After both files are edited, create ONE atomic commit containing both"):

1. **Task 93-07.01: Append G11 section to docs/todo_v0.5.7_gaps.md** — included in `a6e8d4e` (docs)
2. **Task 93-07.02: Append cross-reference to deferred-items.md + atomic commit** — included in `a6e8d4e` (docs)

**Atomic commit:** `a6e8d4e docs(93-07): document naga/--workspace principled deviation (G11)`

No separate task commits were created, by design — the plan specifies a two-file atomic commit because the cross-reference in deferred-items.md only makes sense alongside the G11 section it points at.

## Files Created/Modified

- `docs/todo_v0.5.7_gaps.md` — **Created** in git history (file existed locally but was untracked before this commit). Final length 684 lines. New §G11 section at lines 546-607 with Options A/B/C/D table, Option D rationale citing pre-release-check.sh lines 50-62 and 287-294, Re-evaluation trigger heading with tracker URLs (github.com/gfx-rs/wgpu/issues and github.com/zed-industries/zed), and cross-reference pointing at deferred-items.md. Additional "G11 — naga / --workspace" bullet added to the "Ship plan" section after the Phase 95 bullet.
- `.planning/phases/93-docs-todo-v0-5-7-gaps-md/deferred-items.md` — **Modified** (APPEND-ONLY). 28 new lines at the tail, beginning with the heading `## Resolved during Plan 93-07 (gap closure)` and including the sub-section `### naga v27.0.3 workspace build error — CLOSED as principled deviation`. No existing line was modified or deleted. Existing entries from Plans 01, 02, 04, 05 remain untouched, preserving the raw audit trail.

## Decisions Made

- **Option D selected unambiguously** over Options A/B/C. Evidence recorded in the G11 section: Option A is factually impossible (naga 27.0.4 doesn't exist; the next release is 28.x which gpui-component's 27.x pin rejects via semver); Option B is dishonest framing (calling per-crate testing "scope narrowing" hides that it IS the release gate); Option C is Option B with more collateral damage (workspace-wide `cargo check` would silently miss the connector). Option D is the only honest close.
- **Acceptance criterion realignment is the core deliverable.** The Phase 93 plan-level must_have #5 was written aspirationally before the naga defect surfaced. Per user memory rule "NEVER LIE, NEVER INVENT", the gap doc must record that the criterion was revised, why, and what replaces it. The replacement (per-crate tests tied to pre-release-check.sh) is not weaker than `--workspace` for the sub-crates that matter (native-theme, native-theme-derive, native-theme-build, native-theme-iced all run `run_check` which exits on failure); only native-theme-gpui shifts to `run_check_soft`, which is the correct treatment given the upstream state.
- **Root cause lives outside native-theme.** The documentation explicitly states that the root cause is in gpui-component (its transitive pin of naga 27.0.3) or in naga (its reference to `codespan_reporting::term::emit` with a `&mut String` writer that no longer implements `WriteColor`). Native-theme has no fix without forking gpui-component. This honours user memory rule "Fix root causes, not symptoms" by locating the root cause precisely, even when it isn't actionable from this project.
- **Re-evaluation trigger is concrete, not vague.** The trigger is a specific command chain (`cargo update -p gpui-component && cargo test --workspace --all-features`) that any future maintainer (or a later agent) can run to detect when the deviation becomes obsolete. Tracker URLs to gfx-rs/wgpu and zed-industries/zed are included so the upstream status can be observed without re-discovering where the defect lives.

## Deviations from Plan

None — plan executed exactly as written, with two minor phrasing-level adjustments tracked as Rule 2 fixes:

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Adjusted Option A evidence phrasing to ensure the verification grep target `"naga 27.0.4 does not exist"` matches as a single substring**

- **Found during:** Task 93-07.01 mid-verification (the plan's verify step `grep -n "naga 27.0.4 does not exist" docs/todo_v0.5.7_gaps.md` initially returned empty because my first phrasing was ``` `naga 27.0.4` does not exist on crates.io ``` — the backticks around the version number interrupted the contiguous substring the plan's verifier checks for).
- **Issue:** The plan's success criteria include a grep-by-substring check for evidence that Option A was rejected. My initial phrasing semantically carried the same message but failed the automated check due to a backtick inside the target substring.
- **Fix:** Rewrote the table cell from `` `naga 27.0.4` does not exist on crates.io. `cargo info naga@27.0.4` returns "could not find" `` to `naga 27.0.4 does not exist on crates.io (`cargo info naga@27.0.4` returns "could not find")`. The bare substring `naga 27.0.4 does not exist` now matches; the cargo-info citation stays in parentheses with its own backticks.
- **Files modified:** `docs/todo_v0.5.7_gaps.md` (line 569 table cell only; no structural change)
- **Verification:** `grep -n "naga 27.0.4 does not exist" docs/todo_v0.5.7_gaps.md` returns line 569.
- **Committed in:** `a6e8d4e` (part of the atomic commit).

**2. [Rule 3 - Blocking] Staged the two markdown files explicitly despite `.planning/` directory being gitignored**

- **Found during:** Task 93-07.02 commit staging.
- **Issue:** The `.gitignore` file at line 32 contains `.planning/`, so `git add` of the deferred-items.md path printed an "ignored path" error despite the file already being tracked in git history (added by prior phase-93 plan commits). The error is a git UX quirk: it warns about the parent-directory gitignore match even when the specific path is already tracked.
- **Fix:** The error is cosmetic; git successfully staged the modification because `deferred-items.md` is tracked. Verified by `git diff --cached --stat` showing both files staged with correct insertion counts (28 + 684 = 712 insertions, 0 deletions). No gitignore change needed; no `git add -f` needed.
- **Files modified:** None (diagnosis only).
- **Verification:** Final commit `a6e8d4e` contains both files (`git log -1 --name-only` shows exactly two `.md` paths).
- **Committed in:** `a6e8d4e`.

---

**Total deviations:** 2 auto-fixed (1 phrasing adjustment for verification-grep compatibility, 1 tooling-quirk diagnosis). Both are cosmetic — neither altered the semantic content of the G11 deviation record or the cross-reference.
**Impact on plan:** Zero scope change. The G11 section reads identically to the plan's verbatim markdown block with the one Option-A-cell phrasing tweak; the deferred-items.md append is verbatim.

## Issues Encountered

- **docs/todo_v0.5.7_gaps.md was previously untracked in git.** The file existed on disk (26 KB, 611 lines) and was referenced by all five Phase 93 plan bodies and the phase verification, but had never been committed. Plan 07's commit is the first time this file enters git history. This is not a problem — it simply means the APPEND-ONLY verification via `git diff | grep '^-'` is vacuously satisfied (there is no pre-existing tracked version to delete from). The semantic APPEND-ONLY contract (preserve every line of the local file, only add new sections) was honoured and verified by reading the full file before editing and inserting new content at explicitly non-overlapping regions.
- **One `native-theme/src/model/bundled.rs` modification was present in the working tree.** Confirmed to be work-in-progress from parallel plan 93-06 (the code-change plan running concurrently with this doc-only plan on the same wave). Left untouched and not staged. The final commit contains exactly the two markdown files specified in the plan's `files_modified` frontmatter; `bundled.rs` remains unstaged on disk for plan 93-06 to commit.

## User Setup Required

None — this is a documentation-only plan. No external services, no environment variables, no manual configuration.

## Next Phase Readiness

- Phase 93 verifier (next run) should now treat must_have truth #5 as satisfied by the G11 deviation record: the `--workspace` gate is replaced with per-crate tests tied to `./pre-release-check.sh`. The verifier's prior gap_found status on this item should resolve to verified-with-deviation.
- Phase 93 verifier should continue to flag gap #1 (doctests) and gap #2 (dead-code clippy) as open — those are addressed by parallel plan 93-06 (not this plan). Plan 93-06's commit (bundled.rs edits, currently unstaged in the working tree) will close those two gaps.
- No blockers for Phase 94 P2 architectural-completion work (G6 + G7 + G8). The naga upstream issue does not affect any P2 gap scope.
- Long-term watchpoint: when gpui-component bumps past naga 27.0.3 (or codespan-reporting is pinned back to 0.11.x), run the re-evaluation command documented in G11. If green, delete the G11 deviation record and restore `--workspace` as the plan must_have.

## Self-Check: PASSED

Verification of claims before proceeding to state updates:

**Created files:**
- `FOUND: .planning/phases/93-docs-todo-v0-5-7-gaps-md/93-07-SUMMARY.md` (this file, just created)

**Modified files (content check):**
- `FOUND: docs/todo_v0.5.7_gaps.md` (contains `## G11` at line 546, 12 Option-letter cells, 2 codespan-reporting 0.12.0 mentions, 1 Re-evaluation trigger heading, `naga 27.0.4 does not exist` at line 569 for Option A rejection)
- `FOUND: .planning/phases/93-docs-todo-v0-5-7-gaps-md/deferred-items.md` (contains `CLOSED as principled deviation` at line 88, G11 cross-reference at line 92)

**Commit:**
- `FOUND: a6e8d4e docs(93-07): document naga/--workspace principled deviation (G11)` in `git log --oneline`
- Commit files: exactly 2 — `.planning/phases/93-docs-todo-v0-5-7-gaps-md/deferred-items.md` and `docs/todo_v0.5.7_gaps.md`
- Commit insertions: 712; deletions: 0
- Co-Authored-By trailer count: 0

**No unintended side effects:**
- `native-theme/src/model/bundled.rs` still shows as ` M ` in `git status` (parallel plan 93-06's work; not touched)
- No `.rs`, `.toml`, or `.lock` files in the commit

---
*Phase: 93-docs-todo-v0-5-7-gaps-md*
*Plan: 07*
*Completed: 2026-04-19*
