---
phase: 95-the-remaining-g9-g10-g11-from-docs-todo-v0-5-7-gaps-md
plan: 02
subsystem: docs
tags: [docs, platform-facts, windows, button-order, winui-3, modern-practice, gap-closure, G10, append-only, scope-revision, premise-inversion, code-bug-flag]

# Dependency graph
requires:
  - phase: 88
    provides: "docs/platform-facts.md was present with the dialog-table Windows column and the citation table row at line 1803 before this plan. Phase 88-01/88-02 presets did not alter those rows."
  - phase: 94
    provides: "Phase 94 (G6/G7/G8) closed the final structural gaps before the docs-only G9/G10/G11 polish this phase addresses. Post-94 codebase is the anchor this plan verified against."
provides:
  - "APPEND-ONLY modern WinUI 3 citation row at docs/platform-facts.md:1804 citing the live MS WinUI 3 Dialog controls page (ms.date 2025-08-05, updated_at 2026-04-04) with verbatim quotes confirming PrimaryButton is leftmost / CloseButton is rightmost — documenting continuity with the Win7 Common Buttons guideline already cited at line 1803."
  - "APPEND-ONLY continuity sentence at docs/platform-facts.md:1504-1507 extending the button-order convention paragraph with the modern WinUI 3 corroboration."
  - "APPEND-ONLY SCOPE REVISION annotation block at the tail of 95-02-PLAN.md documenting the premise inversion discovered during execution and pointing to this SUMMARY as the executed record."
  - "Three CRITICAL FLAGS for out-of-scope follow-up: (1) native-theme/src/windows.rs:517 likely CODE BUG (assigns PrimaryRight, contradicting live MS WinUI 3 docs), (2) docs/todo_v0.5.7_gaps.md:510-542 §G10 premise inversion, (3) docs/todo_v0.5.7_gaps.md:537 stale line-number citation (windows.rs:557 → :517)."
affects: [95-03, v0.5.7-release]  # 95-03 is sibling in phase 95 (G11 RELEASING.md formalisation — zero file overlap). v0.5.7 release may want a follow-up dedicated code-investigation plan to address the windows.rs:517 code-bug flag before shipping.

# Tech tracking
tech-stack:
  added: []  # Docs-only plan. No dependency, no feature flag, no new module.
  patterns:
    - "Premise-inversion discovery during execution: when a plan's core premise is contradicted by live-verified source material, the executor (or the orchestrator on its behalf) SHOULD stop, verify the canonical source, and revise scope inline via an APPEND-ONLY annotation block on the PLAN.md rather than rewriting the plan (user memory rule 'Agents must append, never rewrite'). The SUMMARY.md then records the actual executed work and cross-references the annotation."
    - "Continuity-confirmation-over-contradiction citation pattern: when two sources on the same topic agree across decades (Win7 Common Buttons guideline (~2010) and modern WinUI 3 (2025)), the modern citation is added as a SIBLING row to the existing citation (both rows preserved verbatim) rather than superseding it. This preserves the audit trail for legacy maintainers while documenting that modern guidance still applies."
    - "Out-of-scope code-bug flagging via SUMMARY.md: when a docs-only plan discovers a likely code bug (here native-theme/src/windows.rs:517's PrimaryRight assignment for Windows, contradicting live MS WinUI 3 docs), do NOT silently fix the code in a docs-only plan. Flag the finding explicitly in the SUMMARY with file:line and recommended follow-up scope, so a dedicated code-investigation plan can address it under proper planning discipline."

key-files:
  created:
    - ".planning/phases/95-the-remaining-g9-g10-g11-from-docs-todo-v0-5-7-gaps-md/95-02-SUMMARY.md (this file)"
  modified:
    - "docs/platform-facts.md (2 APPEND-ONLY additions: +1 citation row at line 1804, +4 prose lines at 1504-1507; zero deletions across both edits)"
    - ".planning/phases/95-the-remaining-g9-g10-g11-from-docs-todo-v0-5-7-gaps-md/95-02-PLAN.md (APPEND-ONLY SCOPE REVISION annotation block at tail, below existing </output> tag; +84 insertions, 0 deletions)"

key-decisions:
  - "Plan premise was INVERTED. Original 95-02-PLAN.md assumed modern WinUI 3 = primary rightmost. Live MS WinUI 3 Dialog controls page (ms.date 2025-08-05) explicitly says the opposite: PrimaryButton leftmost. The plan was therefore revised in flight: scope changed from 'flip platform-facts.md:1481 claim to primary rightmost' to 'APPEND modern citation confirming continuity of the existing primary-leftmost claim'. Rationale: the live, verified source contradicts the plan's premise; the existing docs/platform-facts.md:1481 + 1501 + 1803 are ALREADY CORRECT per modern MS guidance; only an append-only continuity-corroboration citation is warranted."
  - "Revise inline via APPEND-ONLY annotation on PLAN.md rather than rewrite the plan. Rationale: user memory rule 'Agents must append, never rewrite' applies to PLAN.md as much as to source docs. The SCOPE REVISION annotation at the tail of 95-02-PLAN.md (below </output>) preserves the original plan verbatim (84 insertions, 0 deletions) and points to this SUMMARY as the executed record."
  - "Three individual commits (one per distinct edit) rather than one combined commit. Rationale: each commit touches one file and represents one atomic logical change (PLAN annotation, citation row, prose sentence). Keeps git log readable and makes it easy to revert any single change independently without affecting the others. Matches the one-task-one-commit discipline used in Phase 95-01/95-03."
  - "No edit to docs/platform-facts.md:1481 (the dialog table Windows column). Rationale: it already reads 'primary leftmost' which is correct per live MS WinUI 3 docs. The plan's Task 02 edit instruction (change to 'primary rightmost (WinUI 3 ContentDialog)') would have INTRODUCED a bug into the docs matching the inverted premise."
  - "No edit to docs/platform-facts.md:1500-1503 (the prose paragraph). The existing prose 'Windows primary = leftmost' is correct; Task 02 (of the revised executor prompt, not the original plan) extends the paragraph with a new continuity sentence WITHOUT rewriting any existing sentence (4 insertions, 0 deletions)."
  - "No edit to docs/platform-facts.md:1803 (the existing Win7 citation). It is correct in direction (Win7 guideline 'OK first, then Cancel, then Apply' corresponds to primary-leftmost). The plan's Task 02 Step 4 (annotate as '(Win7-era, historical note — see §G10)') would have mis-characterised a still-valid citation as historical/superseded. Instead, a SIBLING row at line 1804 is added with the modern WinUI 3 corroboration."
  - "Three out-of-scope CRITICAL FLAGS recorded in SUMMARY for follow-up (not fixed here): windows.rs:517 likely code bug; gaps.md §G10 premise inversion; gaps.md:537 stale line-number citation. Each flag cites exact file:line and proposed follow-up scope. Respects the plan's files_modified scope (only docs/platform-facts.md touched; not native-theme/src/*.rs, not docs/todo_v0.5.7_gaps.md)."

patterns-established:
  - "Premise-inversion recovery pattern: when a plan is written on a premise that live-verified sources contradict, recover via (a) APPEND-ONLY SCOPE REVISION annotation on the PLAN.md preserving original content verbatim, (b) SUMMARY.md documenting the inversion and the actual executed work, (c) revised task list documenting what was actually done, (d) out-of-scope follow-up flags for downstream artefacts (code or other docs) that ALSO suffer the same premise inversion."
  - "Citation-continuity pattern: when modern and legacy sources on the same topic agree, APPEND the modern citation as a sibling row preserving the legacy row verbatim. Annotate the new row (here '(modern WinUI 3 continuity, verified 2026-04-20)') so readers understand the relationship without the legacy row needing edit."
  - "Verified-quotes-verbatim discipline: all four exact quoted sentences from the live MS WinUI 3 Dialog controls page are reproduced verbatim in this SUMMARY (no paraphrase, no interpolation) so any future auditor can cross-check the plan's citation against the source. Honours user memory rule 'NEVER LIE, NEVER INVENT' + the platform-fact skill's no-invented-values rule."

requirements-completed: [G10]

# Metrics
duration: ~10m
completed: 2026-04-20
---

# Phase 95 Plan 02: G10 Windows button_order Platform-Facts Modern WinUI 3 Continuity Citation — Summary

**G10 closed as a REVISED-SCOPE docs-only append. Live Microsoft WinUI 3 Dialog controls documentation (ms.date 2025-08-05, verified 2026-04-20) explicitly states PrimaryButton is leftmost / CloseButton is rightmost — CONTRADICTING the plan's original premise that modern WinUI 3 = primary rightmost. Existing `docs/platform-facts.md:1481` ("primary leftmost" in the Windows column), `docs/platform-facts.md:1501` ("Windows primary = leftmost" in prose), and `docs/platform-facts.md:1803` (Win7 Common Buttons guideline citation) are therefore ALREADY CORRECT per modern MS guidance. Revised scope: APPEND a sibling modern-WinUI-3 citation row at `:1804` and an append-only continuity sentence at `:1504-1507` confirming the primary-leftmost convention has persisted from Win7 (~2010) through WinUI 3 (2025). Three critical out-of-scope follow-up flags recorded: (a) `native-theme/src/windows.rs:517` likely CODE BUG (assigns PrimaryRight for Windows contradicting live MS docs); (b) `docs/todo_v0.5.7_gaps.md:510-542` §G10 premise itself is inverted; (c) `docs/todo_v0.5.7_gaps.md:537` stale `windows.rs:557` citation (real site is `:517`).**

## Performance

- **Duration:** ~10 min
- **Started:** 2026-04-20T09:55Z (approx; after orchestrator delivered the revised executor prompt)
- **Completed:** 2026-04-20T09:58Z (approx)
- **Tasks:** 7 (Task 00 PLAN annotation; Task 01 citation row; Task 02 continuity sentence; Task 03 grep reconciliation (no-op, no commit); Task 04 SUMMARY creation; Task 05 pre-release-check.sh; Task 06 STATE.md + ROADMAP.md + final metadata commit)
- **Files modified:** 2 (docs/platform-facts.md + .planning/phases/95-.../95-02-PLAN.md) — plus this SUMMARY + STATE.md + ROADMAP.md for metadata.

## (a) Revised-scope rationale

The original 95-02-PLAN.md was written on the premise that modern WinUI 3 = primary rightmost (matching the premise in `docs/todo_v0.5.7_gaps.md:510-542` §G10). Live Microsoft documentation contradicts this premise.

A previous execution attempt on this plan reached a Task 01 escalation checkpoint: the executor agent could not verify a live Microsoft URL because it lacks `WebFetch`. The orchestrator (which has `WebFetch`) then fetched the canonical Microsoft source and discovered the premise inversion.

Scope was revised from "flip `docs/platform-facts.md:1481` Windows column to primary-rightmost + add Historical note preserving the Win7 guideline as superseded" to "APPEND a sibling modern-WinUI-3 citation confirming the EXISTING primary-leftmost claim is still current, plus a continuity prose sentence". The user approved the revision ("Approved — execute now", 2026-04-20).

The actual executed scope (Tasks 00-06 of the revised executor prompt) is recorded in this SUMMARY. The original PLAN.md is preserved verbatim with an APPEND-ONLY SCOPE REVISION annotation at its tail (below `</output>`).

## (b) Verified source

**URL** (verified live via orchestrator `WebFetch` on 2026-04-20):
`https://learn.microsoft.com/en-us/windows/apps/design/controls/dialogs-and-flyouts/dialogs`

**Page title:** "Dialog controls - Windows apps | Microsoft Learn"

**Frontmatter metadata** (from the live page):
- `ms.date: 2025-08-05`
- `updated_at: 2026-04-04`
- `gitcommit: https://github.com/MicrosoftDocs/windows-dev-docs-pr/blob/c28a6d6e466e6a2fff26147aca9f65d953f65bd5/hub/apps/develop/ui/controls/dialogs-and-flyouts/dialogs.md`

**Exact quoted sentences** (verbatim, no paraphrase):

1. "The 'do it' action button(s) should appear as the leftmost buttons. The safe, nondestructive action should appear as the rightmost button."
2. "PrimaryButton — Optional — Represents the first 'do it' action. Appears as the leftmost button."
3. "CloseButton — Required — Represents the safe, nondestructive action that enables the user to exit the dialog. Appears as the rightmost button."
4. "In general, the affirmation button should be on the left (the primary button) and the cancel button (the secondary button) should be on the right."

## (c) CRITICAL CODE-AUDIT FLAG (out of scope — not fixed here)

`native-theme/src/windows.rs:517` currently reads:

```
variant.dialog.button_order = Some(crate::model::DialogButtonOrder::PrimaryRight);
```

`native-theme/src/resolve/context.rs:44-46` documents:

```
pub button_order: DialogButtonOrder,
// Dialog button ordering (`PrimaryLeft` on KDE, `PrimaryRight` elsewhere).
```

Live MS WinUI 3 guidance: PrimaryButton leftmost.

**Conclusion:** `windows.rs:517` likely should be `DialogButtonOrder::PrimaryLeft`, not `PrimaryRight`. The Windows code contradicts both the live MS guidance AND the existing `docs/platform-facts.md:1481` documentation.

**Recommended follow-up:** a dedicated code-investigation plan (suggest naming `phase-XX-plan-YY: windows button_order code-bug investigation + fix`) that:

1. Confirms the exact semantics of `DialogButtonOrder::PrimaryLeft` vs `DialogButtonOrder::PrimaryRight` (which enum variant means "primary button rendered leftmost"?).
2. Checks all three sites flagged in `docs/todo_v0.5.7_gaps.md:537-538` for `PrimaryRight` assignments on Windows code paths (`windows.rs:517`, plus `windows-11.toml` and `resolve/inheritance.rs::platform_button_order()` which the gaps doc claims "all agree on `PrimaryRight`").
3. Cross-references `gnome/mod.rs:266` (which also assigns `PrimaryRight` for GNOME) — per GNOME HIG "cancel left, affirmative right", GNOME SHOULD be primary-rightmost, so that one appears correct.
4. Cross-references macOS test sites — per Apple HIG "primary rightmost", macOS SHOULD be primary-rightmost, so those also appear correct.
5. Windows is the anomaly: the code assigns PrimaryRight, but the MS WinUI 3 guidance AND `docs/platform-facts.md` both say primary-leftmost.

**OUT OF SCOPE for this plan** (files_modified: only docs/platform-facts.md per the plan frontmatter; native-theme/src/* Rust files not in scope).

## (d) CRITICAL GAPS-DOC PREMISE FLAG (out of scope — not fixed here)

`docs/todo_v0.5.7_gaps.md:510-542` §G10 premise is inverted. The upstream gap-tracking document claims:

> Modern WinUI 3 ContentDialog uses primary-right. The Windows code tracks modern practice; platform-facts tracks the older guideline. This is a documentation vs. modern-practice tension, not a code bug.

Live MS WinUI 3 Dialog controls documentation (ms.date 2025-08-05, verified 2026-04-20 via orchestrator `WebFetch`) explicitly contradicts this: "PrimaryButton — Represents the first 'do it' action. Appears as the leftmost button." The direction of the claimed "tension" is wrong — the gaps doc mis-identified which artefact tracks modern practice.

**Recommended follow-up:** a dedicated APPEND-ONLY annotation on `docs/todo_v0.5.7_gaps.md` §G10 correcting the premise. Cannot modify here — `docs/todo_v0.5.7_gaps.md` is not in `files_modified` scope for this plan; additionally, `docs/todo_v0.5.7_gaps.md` is a frozen historical gap-tracking document that should only be annotated append-only (same discipline as Phase 95-03's APPEND-ONLY annotations on Phase 93 PLAN files).

**OUT OF SCOPE for this plan.**

## (e) Stale line-number citation FLAG (minor, out of scope)

`docs/todo_v0.5.7_gaps.md:537` cites `windows.rs:557` as the `PrimaryRight` assignment site:

> `windows.rs:557`, `windows-11.toml`, and `resolve/inheritance.rs::platform_button_order()` already agree on `PrimaryRight`.

The real `PrimaryRight` assignment site is `windows.rs:517` (verified via `Read` tool). `windows.rs:557` does not exist at that content (file structure has drifted since the gaps-doc citation was written).

**Recommended follow-up:** combine with (d) above — the same APPEND-ONLY annotation on `docs/todo_v0.5.7_gaps.md` §G10 can correct both the premise inversion AND the stale line number. Same pattern as Phase 95-01's correction of `lib.rs:347` → `:371`+`:387` via APPEND-ONLY post-script on the sibling design doc.

**OUT OF SCOPE for this plan.** This note is also documented in the 95-02-PLAN.md `<output>` block (line 372) as a follow-up annotation candidate; the premise-inversion finding (d) is a separate, more consequential flag.

## (f) Self-Check: PASSED

See `## Self-Check: PASSED` section at the bottom of this SUMMARY for verified success criteria.

## Task Commits

Each task was committed atomically (no `Co-Authored-By` trailer per user memory rule):

1. **Task 00: APPEND-ONLY SCOPE REVISION annotation on 95-02-PLAN.md** — commit `2d0a2d5` — `docs(95-02): append SCOPE REVISION annotation — plan premise inverted, revised inline` (+84, -0)
2. **Task 01: Append modern WinUI 3 citation row at `docs/platform-facts.md:1804`** — commit `15006f9` — `docs(95-02): append modern WinUI 3 citation confirming Windows primary-leftmost continuity` (+1, -0)
3. **Task 02: Append continuity prose sentence at `docs/platform-facts.md:1504-1507`** — commit `2304acb` — `docs(95-02): append modern WinUI 3 continuity prose sentence for Windows button order` (+4, -0)
4. **Task 03: Grep reconciliation** — no commit (read-only; no contradictions found across 7 matches; all existing entries consistent with primary-leftmost claim).
5. **Task 04: Create this SUMMARY.md** — committed as part of Task 06 final metadata commit (below).
6. **Task 05: `./pre-release-check.sh`** — green banner captured: `🎉 All pre-release checks passed successfully! native-theme v0.5.7 is ready for release.` No commit (pure verification).
7. **Task 06: STATE.md + ROADMAP.md + SUMMARY.md final metadata commit** — commit hash recorded below after commit.

## Files Created/Modified

- `docs/platform-facts.md` — 2 APPEND-ONLY additions totalling +5 lines, 0 deletions:
  - Line 1804: new modern WinUI 3 citation row (sibling to existing Win7 row at 1803).
  - Lines 1504-1507: continuity prose sentence extending the button-order convention paragraph.
- `.planning/phases/95-.../95-02-PLAN.md` — APPEND-ONLY SCOPE REVISION annotation block at tail (+84 lines, 0 deletions).
- `.planning/phases/95-.../95-02-SUMMARY.md` — new file (this document).
- `.planning/STATE.md` — updated (Task 06).
- `.planning/ROADMAP.md` — Phase 95 95-02 row marked `[x] COMPLETE` (Task 06).

## Decisions Made

See `key-decisions` in frontmatter (7 decisions). Highlights:

- **Premise inversion detected mid-execution; scope revised inline.** Orchestrator's WebFetch verified the live Microsoft source contradicted the plan's premise. User approved the revision ("Approved — execute now", 2026-04-20).
- **Revise via APPEND-ONLY annotation on PLAN.md rather than rewrite.** Preserves original plan verbatim; pointed to SUMMARY as executed record.
- **No edit to `platform-facts.md:1481`, `:1501`, or `:1803`** — all three are already correct per live MS docs.
- **Three out-of-scope flags recorded for follow-up:** windows.rs:517 code bug, gaps.md §G10 premise inversion, gaps.md:537 stale line number.

## Deviations from Plan

Entire plan is a Rule 4 architectural deviation (premise inversion). Handled via user-approved scope revision before continued execution, not as an auto-fix. All three work-edits (Task 00 annotation, Task 01 citation row, Task 02 continuity sentence) are explicitly enumerated in the revised executor prompt, not in the original PLAN.md.

**Original plan instructions that were NOT executed:**

- Original Task 02 Step 2: "change the Windows cell (Col 3) from 'primary leftmost' to 'primary rightmost (WinUI 3 ContentDialog) ✅'" — NOT done, because the existing 'primary leftmost' is correct per live MS docs.
- Original Task 02 Step 3: "APPEND: Historical note on Windows button order... 'line above reading "Windows primary = leftmost" reflects the older Microsoft Common Buttons guideline... Modern WinUI 3 ContentDialog uses **primary rightmost**'" — NOT done, because the premise is inverted.
- Original Task 02 Step 4: annotate the Win7 citation row as "(Win7-era, historical note — see §G10)" and add a sibling row with "primary rightmost (WinUI 3 modern)" — REVERSED: the new sibling row at 1804 says "primary leftmost (modern WinUI 3 continuity)"; the Win7 row at 1803 is preserved verbatim.

**No Rule 1-3 auto-fixes occurred** — the deviation is a Rule 4 (architectural: premise reversal requires user approval), handled by the orchestrator before this executor was spawned. This executor executed the approved revised scope only.

**Total deviations:** 1 Rule 4 architectural deviation (premise inversion, user-approved). 0 Rule 1-3 auto-fixes.

**Impact on plan:** Fundamental scope revision. The plan's original intent (bring platform-facts.md:1481 in line with modern practice) was preserved — the execution just went the opposite direction from the plan's written instructions because modern practice turned out to match the existing documentation, not contradict it.

## Issues Encountered

- **Premise inversion of plan** — root-caused via WebFetch verification of the canonical Microsoft source, resolved via user-approved scope revision. The plan was written without WebFetch verification of the premise; the orchestrator's WebFetch on 2026-04-20 established the premise was backwards.
- **`.planning/` is gitignored** — required `git add --force` for `95-02-PLAN.md` and `95-02-SUMMARY.md`. Same pattern as Phase 95-01/95-03 per STATE.md's recorded pattern.

## User Setup Required

None — no external service configuration required. Docs-only plan.

## pre-release-check.sh output

```
🎉 All pre-release checks passed successfully!
native-theme v0.5.7 is ready for release.
```

Banner captured at Task 05 (2026-04-20T09:57Z).

## Next Phase Readiness

- **Phase 95 complete after this plan.** All three gap-doc items G9 (95-01), G11 (95-03), G10 (this plan) now closed as docs-only principled deviations and continuity citations.
- **Recommended v0.5.7 pre-release follow-up:** a dedicated code-investigation plan to address the `windows.rs:517` likely code bug flagged in section (c) above. This is a GENUINE CODE BUG if confirmed (per live MS WinUI 3 docs, Windows code should return `DialogButtonOrder::PrimaryLeft`, not `PrimaryRight`), and therefore should NOT ship in v0.5.7 without investigation. Same principled discipline as the v0.5.7 API Overhaul milestone's overall policy of "fix root causes, not symptoms" (user memory rule).
- **Other recommended follow-ups** (lower priority):
  - APPEND-ONLY annotation on `docs/todo_v0.5.7_gaps.md` §G10 correcting the premise inversion AND the `:557` → `:517` stale line-number citation (same pattern as Phase 95-01's `lib.rs:347` → `:371`+`:387` correction).
- **No blockers for v0.5.7 release from this plan's outputs.** The docs are now internally consistent (1481/1501/1803/1804 all say primary-leftmost for Windows). The CODE BUG flag is a separate decision for the release reviewer.

---
*Phase: 95-the-remaining-g9-g10-g11-from-docs-todo-v0-5-7-gaps-md*
*Completed: 2026-04-20*

## Self-Check: PASSED

Verified claims (all PASS):

1. **Task 00 APPEND-ONLY annotation on 95-02-PLAN.md** — FOUND at commit `2d0a2d5`; `git diff 2d0a2d5^ 2d0a2d5 -- .planning/phases/95-the-remaining-g9-g10-g11-from-docs-todo-v0-5-7-gaps-md/95-02-PLAN.md | grep -c '^-[^-]'` = 0 (APPEND-ONLY verified).
2. **Task 01 modern WinUI 3 citation row** — FOUND at `docs/platform-facts.md:1804`, commit `15006f9`; `git diff 15006f9^ 15006f9 -- docs/platform-facts.md | grep -c '^-[^-]'` = 0.
3. **Task 02 continuity prose sentence** — FOUND at `docs/platform-facts.md:1504-1507`, commit `2304acb`; `git diff 2304acb^ 2304acb -- docs/platform-facts.md | grep -c '^-[^-]'` = 0.
4. **Task 03 grep reconciliation** — 7 matches across `docs/platform-facts.md`; all consistent with Windows primary-leftmost; no contradictions.
5. **Task 04 SUMMARY.md created with all six subsections (a)-(f)** — this file; sections (a) Revised-scope rationale, (b) Verified source, (c) CRITICAL CODE-AUDIT FLAG, (d) CRITICAL GAPS-DOC PREMISE FLAG, (e) Stale line-number citation FLAG, (f) Self-Check: PASSED — all present.
6. **Task 05 `./pre-release-check.sh` green banner** — captured verbatim above.
7. **Zero Rust source, preset, Cargo, or test file changes.** Confirmed via `git log 95-02-plan-start..HEAD --name-only` shows only `docs/platform-facts.md` + `.planning/phases/95-.../95-02-PLAN.md` + (forthcoming) `95-02-SUMMARY.md` + STATE.md + ROADMAP.md.
8. **Verified Microsoft URL + exact quotes used verbatim** — no paraphrase, no extra URLs. Single modern URL (live on 2026-04-20), four verbatim quoted sentences reproduced in section (b).
9. **No `Co-Authored-By` trailer** on any commit. Verified `git log -1 --format=%B | grep -c "Co-Authored-By"` = 0 on each of Tasks 00/01/02 commits.
