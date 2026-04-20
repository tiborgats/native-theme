---
phase: 95-the-remaining-g9-g10-g11-from-docs-todo-v0-5-7-gaps-md
plan: 01
subsystem: docs
tags: [docs, ownership-types, cow, arc-str, principled-deviation, v057-polish, gap-closure, G9, rustdoc, append-only]

# Dependency graph
requires:
  - phase: 87
    provides: "Phase 87-02 migrated FontSpec::family and ResolvedFontSpec::family from String to Arc<str> with serde rc feature; established the one-axis-that-keeps-Arc precedent this plan documents as the retained case."
  - phase: 88
    provides: "Phase 88-02 introduced Theme::name: Cow<'static, str> with manual Default impl using Cow::Borrowed(\"\") and PRESET_DISPLAY_NAMES const table; fixed the shape of the `name` field that this plan documents as a principled deviation from the uniform-Arc<str> recommendation."
  - phase: 94
    provides: "Phase 94-03 landed the G8 ThemeReader trait + per-backend reader structs; closed the final structural gap before the docs-only G9/G10/G11 polish that phase 95 now addresses. Post-94, lib.rs SystemTheme field line numbers are stable (name at 371, icon_theme at 387)."
provides:
  - "Expanded rustdoc on `SystemTheme::name` (native-theme/src/lib.rs:371) that documents the G9 principled deviation from doc 2 §J.2 / §K.3, cross-references `docs/todo_v0.5.7_gaps.md` §G9, and confirms `ResolvedFontSpec::family` is the sole `Arc<str>` retainer."
  - "Expanded rustdoc on `SystemTheme::icon_theme` (native-theme/src/lib.rs:387) that delegates to `SystemTheme::name` via an intra-doc link (DRY single-source-of-truth) and supplies only the icon-theme-specific dedup observation (KDE's breeze/breeze-dark pair is the maximum across platforms)."
  - "Two APPEND-ONLY post-scripts in `docs/todo_v0.5.7_native-theme-api-2.md` — one under §J.2 B3 refinement (after the `Confidence: high` line at line 2642) and one under §K.3 (after `Adopt uniformly across the crate.` at line 3105) — that record the deviation rationale and cross-reference `docs/todo_v0.5.7_gaps.md:449-506` as the definitive audit."
  - "G9 closed as a principled deviation (not a code change): `SystemTheme::name`, `SystemTheme::icon_theme`, `Theme::name`, and `ThemeDefaults::icon_theme` stay `Cow<'static, str>`; only `ResolvedFontSpec::family` honours the uniform-`Arc<str>` recommendation."
affects: [95-02, 95-03]  # Sibling plans in phase 95 closing G10 (platform-facts cross-reference) and G11 (Phase 93-07 PLAN annotations). Zero file overlap with this plan.

# Tech tracking
tech-stack:
  added: []  # Docs-only plan; no dependency, no feature flag, no new module added
  patterns:
    - "Principled-deviation documentation pattern: when the implementation chooses a better path than a recommendation in a design doc, document the deviation at BOTH the point-of-use (rustdoc on the type) AND the source of the recommendation (APPEND-ONLY post-script on the design-doc section). Cross-reference a single canonical audit record (§G9 in gaps.md) from both anchors. The point-of-use rustdoc is the one-truth-statement; the design-doc post-script preserves the audit trail."
    - "Append-within-rustdoc-block discipline: when user memory rule `Agents must append, never rewrite` meets rustdoc's line-above-field convention, PRESERVE the existing one-line rustdoc verbatim as the first `///` line and append new `///` lines below — still before the field declaration, per rustdoc grammar. Net effect: zero existing text edited, new paragraphs added within the same rustdoc block. Verified append-only via `git diff | grep -c '^-[^-]'` = 0."
    - "Intra-doc-link cross-references over prose repetition: the `icon_theme` rustdoc delegates to `name` via `[SystemTheme::name](Self::name)` for the full rationale and only adds icon-theme-specific content. Keeps the single-source-of-truth on `name` unambiguous and the `cargo doc` output DRY. All five intra-doc links in the new rustdoc (Self::name, Self::icon_theme, crate::model::font::ResolvedFontSpec, crate::theme::Theme, crate::model::defaults::ThemeDefaults) resolve cleanly under `cargo doc --no-deps -p native-theme --all-features` with zero new warnings."

key-files:
  created: []  # Plan 95-01 is append-only to existing files; no new files created
  modified:
    - "native-theme/src/lib.rs (rustdoc-only expansion on SystemTheme::name lines 370-397 and SystemTheme::icon_theme lines 410-421; zero field reshape, zero type-signature change, zero import change; +33 lines, -0 lines)"
    - "docs/todo_v0.5.7_native-theme-api-2.md (APPEND-ONLY post-scripts under §J.2 B3 refinement and §K.3; both cross-reference `docs/todo_v0.5.7_gaps.md` §G9; +37 lines, -0 lines)"

key-decisions:
  - "Full rationale lives on `SystemTheme::name`; `SystemTheme::icon_theme` delegates via intra-doc link. Rationale: the `Arc<str>` vs `Cow<'static, str>` trade-off is one argument, applied to two fields by the same audit. Duplicating the rationale on both fields would make the rustdoc block on `icon_theme` 26 lines long for 2 new lines of icon-theme-specific content (KDE's breeze/breeze-dark observation). Delegation keeps `name` as the canonical record and `icon_theme` as a reference to it; `cargo doc` renders the intra-doc link as a clickable reference, so readers land on `name` anyway when they follow the trail."
  - "Lowercase \"principled deviation\" in the `#` heading (not \"Principled\"). Rustdoc convention under `#` headings uses sentence case (matches the rest of the codebase's rustdoc conventions — grep `^/// # ` in native-theme/src/ shows sentence case for all section headings). The plan's verify command expected title case (`grep -c \"Principled deviation from doc 2 §J.2\"`) but the plan's own target-shape text (line 112 of the PLAN.md) uses lowercase `principled deviation`. The target-shape text is the contract, the verify command is a guide; following the target shape is correct. Post-task verification confirms lowercase `principled deviation` appears twice (once on each field) via `grep -c \"principled deviation\"` = 2."
  - "Single atomic commit covering both files (lib.rs + api-2.md). Rationale: the two changes are one logical unit — document the G9 deviation in-source AND in the design doc that recommended the uniform-Arc<str> path. Splitting into two commits would create a window where one half of the record exists without the other. The plan's `<tasks>` block allows either one-commit-covers-both or two-atomic-commits, with the one-commit form being the recommended shape when no independent verification gate sits between the two files."
  - "Zero Rust source-code change despite touching lib.rs. All edits are rustdoc comments above existing field declarations. Field types (`Cow<'static, str>`), visibility (`pub`), and names (`name`, `icon_theme`) all remain byte-identical. Verified via `grep -n \"pub \\(name\\|icon_theme\\): Cow\" native-theme/src/lib.rs` → exact `pub name: Cow<'static, str>,` and `pub icon_theme: Cow<'static, str>,` preserved. `./pre-release-check.sh` green banner unchanged (docs-only change cannot affect code gates)."
  - "The plan's expected insertion anchors (`Confidence: high` for J.2, `Adopt uniformly across the crate.` for K.3) matched the file exactly at the stated line numbers (2642 and 3105). Both post-scripts were inserted immediately after the respective anchor paragraph with one blank line separator; existing content is byte-identical. `git diff docs/todo_v0.5.7_native-theme-api-2.md | grep -c '^-[^-]'` = 0 (APPEND-ONLY confirmed)."

patterns-established:
  - "Docs-only gap-closure pattern: when a design doc recommends a path that the implementation chose NOT to follow for principled reasons, close the gap with (a) rustdoc on the type documenting the deviation at the point-of-use, (b) APPEND-ONLY post-script on the design-doc section recording the audit rationale, and (c) a single canonical audit record (here §G9 in gaps.md) that both anchors cross-reference. No code change, no version bump for crate behaviour, but the gap is formally closed."
  - "APPEND-ONLY verification protocol: for any plan where user memory rule `Agents must append, never rewrite` applies, add `git diff <file> | grep -c '^-[^-]'` = 0 to the plan's verify block AND re-check after the commit with `git diff 2f3e0e8^ 2f3e0e8 -- <file> | grep -c '^-[^-]'`. Both must return 0. The `^-[^-]` pattern excludes diff header lines (`---`) and matches only actual deletion lines."
  - "Cross-doc audit-trail linking: when an audit record spans multiple documents (rustdoc, design-doc post-scripts, gaps.md), choose ONE canonical location (`docs/todo_v0.5.7_gaps.md` §G9 in this case) and have every other location cross-reference it with file:line anchors (`docs/todo_v0.5.7_gaps.md:449-506`). Avoids drift between records."

requirements-completed: [G9]

# Metrics
duration: 12m
completed: 2026-04-20
---

# Phase 95 Plan 01: G9 Arc<str>/Cow<'static, str> Ownership-Type Principled Deviation — Summary

**G9 closed as principled deviation — expanded rustdoc on `SystemTheme::name` and `SystemTheme::icon_theme` (native-theme/src/lib.rs:371/387) documents why `Cow<'static, str>` was chosen over doc 2 §J.2/§K.3's uniform-`Arc<str>` recommendation (each resolved theme carries exactly one such string; bundled preset names are `&'static str` literals; `ResolvedFontSpec::family` is the sole Arc<str> retainer where real dedup across 26 widgets × connectors applies); APPEND-ONLY post-scripts in `docs/todo_v0.5.7_native-theme-api-2.md` §J.2 / §K.3 record the audit rationale and cross-reference `docs/todo_v0.5.7_gaps.md:449-506` as the definitive record.**

## Performance

- **Duration:** 12 min (wall-clock: 2026-04-20T08:58:00Z → 2026-04-20T09:10:13Z)
- **Started:** 2026-04-20T08:58:00Z
- **Completed:** 2026-04-20T09:10:13Z
- **Tasks:** 2 (Task 01 rustdoc in lib.rs + Task 02 post-scripts in api-2.md; committed as ONE atomic commit per plan allowance)
- **Files modified:** 2 (native-theme/src/lib.rs + docs/todo_v0.5.7_native-theme-api-2.md)
- **Lines changed:** +70 / -0 (APPEND-ONLY across both files)

## Accomplishments

- `native-theme/src/lib.rs` `SystemTheme::name` now carries a 22-line rustdoc section (`# Ownership type — principled deviation from doc 2 §J.2 / §K.3`) that names the recommendation, cites the audit (`docs/todo_v0.5.7_gaps.md` §G9), enumerates the reasoning (no dedup benefit; bundled `&'static str` literals; `Cow::Borrowed(static_lit)` is zero allocation), and cross-references three sibling fields by intra-doc link (`ResolvedFontSpec::family`, `Theme::name`, `ThemeDefaults::icon_theme`).
- `native-theme/src/lib.rs` `SystemTheme::icon_theme` carries an 8-line rustdoc section (`# Ownership type`) that delegates to `SystemTheme::name` via `[SystemTheme::name](Self::name)` and supplies only the icon-theme-specific dedup observation (KDE has two names across light/dark — `"breeze"` / `"breeze-dark"`; other platforms have one).
- `docs/todo_v0.5.7_native-theme-api-2.md` §J.2 B3 refinement gains a 21-line post-script titled `Post-script 2026-04-20 — principled deviation per docs/todo_v0.5.7_gaps.md §G9` that explains why the uniform-`Arc<str>` recommendation was NOT adopted for the `name`/`icon_theme` axis, names the retained case (`ResolvedFontSpec::family`), and cross-references `docs/todo_v0.5.7_gaps.md:449-506` + the new `SystemTheme::name` rustdoc.
- `docs/todo_v0.5.7_native-theme-api-2.md` §K.3 gains a 14-line post-script with the same title that softens the "Adopt uniformly across the crate" recommendation to the single axis where it applies, cross-references §J.2's post-script, and points at the definitive audit.
- Zero Rust source-code change: no struct-field reshape, no type-signature change, no function-body change, no import change. `./pre-release-check.sh` green banner `🎉 All pre-release checks passed successfully! native-theme v0.5.7 is ready for release.` unchanged.
- `cargo doc --no-deps -p native-theme --all-features` produces zero NEW warnings/errors from the added rustdoc. The 4 pre-existing warnings (`on_theme_change`, `ThemeSubscription` ×2, `ThemeChangeEvent`) pre-date this plan and were not touched (out of scope per the deviation-scope rule).

## Task Commits

Each task was executed and committed atomically. Per the plan's explicit allowance ("either one atomic commit covering both files OR two separate atomic commits"), both tasks shipped in ONE commit because the edits form a single logical unit (document the G9 deviation in-source AND in the design doc simultaneously):

1. **Task 95-01.01: Expand rustdoc on SystemTheme::name and SystemTheme::icon_theme in native-theme/src/lib.rs** — committed as part of `2f3e0e8` (docs — rustdoc-only expansion, append-within-rustdoc-block)
2. **Task 95-01.02: Append G9 principled-deviation post-scripts to docs/todo_v0.5.7_native-theme-api-2.md §J.2 and §K.3; create atomic commit for both files** — committed as `2f3e0e8` (docs — APPEND-ONLY; the atomic commit also includes Task 01's lib.rs changes per the one-commit-covers-both shape)

**Atomic commit:** `2f3e0e8 docs(95-01): document G9 Arc<str>/Cow<'static, str> principled deviation` (2 files changed, 70 insertions(+), 0 deletions(-))

**Plan metadata commit (final metadata update):** to follow — this SUMMARY.md + STATE.md + ROADMAP.md + REQUIREMENTS.md update will be committed separately as `docs(95-01): complete plan 95-01 metadata — G9 principled deviation closed`.

_Note: No TDD cycle applies (plan is `type: execute`, not `type: tdd`; no test changes; no behaviour change)._

## Files Created/Modified

- **`native-theme/src/lib.rs`** — rustdoc-only expansion on two fields of `pub struct SystemTheme`:
  - `SystemTheme::name` (line 371, field declaration `pub name: Cow<'static, str>,`): rustdoc block expanded from 1 line to 23 lines (existing one-liner preserved verbatim; 22 new lines appended above the field declaration per rustdoc grammar). New content introduces the `# Ownership type — principled deviation from doc 2 §J.2 / §K.3` section, cites doc 2 §J.2 + §K.3 + `docs/todo_v0.5.7_gaps.md` §G9, enumerates the two-point reasoning (no dedup benefit; `&'static str` literals), cross-references three sibling fields by intra-doc link, and ends with "See `docs/todo_v0.5.7_gaps.md` §G9 for the full audit."
  - `SystemTheme::icon_theme` (line 387, field declaration `pub icon_theme: Cow<'static, str>,`): rustdoc block expanded from 1 line to 9 lines (existing one-liner preserved verbatim; 8 new lines appended). New content introduces the `# Ownership type` section, delegates to `[SystemTheme::name](Self::name)` for the full rationale, and adds only the icon-theme-specific observation (KDE's breeze/breeze-dark pair is the maximum across platforms).
  - **Zero structural change:** struct definition, field types, field visibility, field names all byte-identical. `grep -n "pub name: Cow" native-theme/src/lib.rs` → line 396 (was line 371 pre-edit; the shift of +25 lines reflects the 22-line rustdoc insertion above `name` + a 3-line gap); `grep -n "pub icon_theme: Cow" native-theme/src/lib.rs` → line 420 (was line 387 pre-edit; shift reflects insertions above both fields). Field declarations themselves exact.

- **`docs/todo_v0.5.7_native-theme-api-2.md`** — two APPEND-ONLY post-scripts, each inserted at the specified anchor without modifying any existing prose:
  - Post-script under §J.2 B3 refinement (after line 2642 `Confidence: high. This is the standard Rust answer for "shared immutable string across many owners."`): 21 lines titled `**Post-script 2026-04-20 — principled deviation per docs/todo_v0.5.7_gaps.md §G9:**`. Explains the audit conclusion (no dedup benefit), names the retained axis (`ResolvedFontSpec::family`), cross-references `docs/todo_v0.5.7_gaps.md` §G9 + `native-theme/src/lib.rs` `SystemTheme::name` rustdoc, and ends with "**Net result:** `name` / `icon_theme` stay `Cow<'static, str>`; `family` stays `Arc<str>`. Mixed ownership is intentional, not accidental inconsistency."
  - Post-script under §K.3 (after line 3105 `Adopt uniformly across the crate.`): 14 lines with the same title. Softens "Adopt uniformly across the crate" to the single axis where it genuinely applies, enumerates the four fields where the audit demonstrated no dedup benefit, cross-references §J.2's post-script, and points at the definitive audit (`docs/todo_v0.5.7_gaps.md:449-506`).
  - **APPEND-ONLY confirmed:** `git diff 2f3e0e8^ 2f3e0e8 -- docs/todo_v0.5.7_native-theme-api-2.md | grep -c '^-[^-]'` = 0.

## Decisions Made

See key-decisions in frontmatter. Summary:

1. **Rationale split between the two rustdoc blocks** — full rationale on `SystemTheme::name`; `icon_theme` delegates via `[SystemTheme::name](Self::name)`. Keeps the single-source-of-truth on `name` and avoids duplicating 22 lines of ownership-type analysis on a field whose only unique observation is the KDE breeze/breeze-dark pair.
2. **Lowercase `principled deviation` in rustdoc section headings** — matches the plan's target-shape text (PLAN.md line 112) and the rest of the codebase's sentence-case rustdoc convention.
3. **Single atomic commit covering both files** — the two changes are one logical unit (document the deviation in-source AND in the design doc); splitting would create a window where one half of the record exists without the other.
4. **Zero Rust source-code change** — all edits are rustdoc comments above existing field declarations; struct shape, field types, field visibility, field names all byte-identical.
5. **Anchor matches confirmed** — the plan's stated insertion anchors (`Confidence: high` at line 2642 for §J.2, `Adopt uniformly across the crate.` at line 3105 for §K.3) matched the file exactly; no anchor-drift correction needed.

## Deviations from Plan

**None — plan executed exactly as written.**

Every target-shape text block in the plan was inserted verbatim, existing prose on both files was preserved byte-identically, and the plan's explicit allowance of "one atomic commit covers both files OR two separate atomic commits" was taken as the one-commit form per the plan's own guidance ("When no independent verification gate sits between the two files"). The plan's verify command expected title-case `Principled deviation` but the plan's target-shape text (PLAN.md line 112) used lowercase; the target-shape text is the contract and was followed verbatim. This is NOT a deviation — it's a resolution of an internal inconsistency in the plan's verify guide where the target shape takes precedence.

## Issues Encountered

**None.** The plan's three potential risk points (line-number drift post-Phase 94, intra-doc link resolution, APPEND-ONLY discipline across two files) were all handled without adjustment:

1. **Line-number stability:** the plan stated `SystemTheme::name` at line 371 and `SystemTheme::icon_theme` at line 387. Pre-edit grep confirmed the exact lines (`grep -n "pub name: Cow"` = 371, `grep -n "pub icon_theme: Cow"` = 387). No drift.
2. **Intra-doc link resolution:** all five new intra-doc links (`Self::name`, `Self::icon_theme`, `crate::model::font::ResolvedFontSpec`, `crate::theme::Theme`, `crate::model::defaults::ThemeDefaults`) resolved cleanly under `cargo doc --no-deps -p native-theme --all-features`. Zero new warnings. Four pre-existing warnings (`ThemeSubscription` ×2, `ThemeChangeEvent`, `on_theme_change`) relate to the watcher module — unrelated, out of scope.
3. **APPEND-ONLY discipline:** `git diff | grep -c '^-[^-]'` = 0 on both files (no prose deleted, no prose rewritten). Existing one-line rustdocs on `SystemTheme::name` and `SystemTheme::icon_theme` preserved byte-identically as the first `///` line of the expanded blocks.

## User Setup Required

**None.** This is a docs-only plan with zero runtime-behaviour change, no new environment variables, no CLI or UI impact, no new dependencies.

## Next Plan Readiness

- **Plan 95-02 (G10 — platform-facts cross-reference)** ready to start. Zero file overlap with this plan (95-01 touches `native-theme/src/lib.rs` + `docs/todo_v0.5.7_native-theme-api-2.md`; 95-02 touches `docs/platform-facts.md`). Both are wave 1 per plan frontmatter; parallel execution OK.
- **Plan 95-03 (G11 — Phase 93-07 principled-deviation PLAN annotations)** ready to start. Zero file overlap with this plan. Both are wave 1 per plan frontmatter; parallel execution OK.
- **Post-plan verification gate:** the phase 95 overall verification (sibling plans + phase-level sign-off) runs after all three plans land. Per plan-level success criteria ("one atomic git commit covers exactly two files"), the commit `2f3e0e8` satisfies this.
- **`./pre-release-check.sh` green banner unchanged** — the v0.5.7 release gate remains green. No new blocker introduced by this plan.

## Self-Check

Verifying claims in this SUMMARY:

**1. Commit `2f3e0e8` exists with exactly two files:**
- `git log -1 --format=%s` → `docs(95-01): document G9 Arc<str>/Cow<'static, str> principled deviation` ✓
- `git log -1 --name-only` lists `native-theme/src/lib.rs` and `docs/todo_v0.5.7_native-theme-api-2.md` ✓
- `git log -1 --format=%B | grep -c "Co-Authored-By"` → 0 ✓

**2. Rustdoc content landed on both `SystemTheme` fields:**
- `grep -c "§G9" native-theme/src/lib.rs` → 3 (once in the `name` rustdoc, once in the `icon_theme` rustdoc, once in the trailing `See docs/todo_v0.5.7_gaps.md §G9 for the full audit.` line) ✓
- `grep -c "dedup benefit" native-theme/src/lib.rs` → 3 ✓
- `grep -c "todo_v0.5.7_gaps.md" native-theme/src/lib.rs` → 3 ✓
- `grep -c "principled deviation" native-theme/src/lib.rs` → 2 ✓

**3. Post-scripts landed on both §J.2 and §K.3:**
- `grep -c "Post-script 2026-04-20" docs/todo_v0.5.7_native-theme-api-2.md` → 2 (exactly as targeted) ✓
- `grep -c "§G9" docs/todo_v0.5.7_native-theme-api-2.md` → 5 (2 in post-script headings + 3 in bodies) ✓
- `grep -c "no dedup benefit" docs/todo_v0.5.7_native-theme-api-2.md` → 2 ✓

**4. APPEND-ONLY discipline preserved:**
- `git diff 2f3e0e8^ 2f3e0e8 -- native-theme/src/lib.rs | grep -c '^-[^-]'` → 0 ✓
- `git diff 2f3e0e8^ 2f3e0e8 -- docs/todo_v0.5.7_native-theme-api-2.md | grep -c '^-[^-]'` → 0 ✓

**5. Release gate green:**
- `./pre-release-check.sh` exit code 0, banner `🎉 All pre-release checks passed successfully! native-theme v0.5.7 is ready for release.` ✓
- `cargo doc --no-deps -p native-theme --all-features` zero NEW warnings on added rustdoc ✓

## Self-Check: PASSED

All claims verified against `git log`, file contents, and the pre-release-check banner.

---
*Phase: 95-the-remaining-g9-g10-g11-from-docs-todo-v0-5-7-gaps-md*
*Plan: 01*
*Completed: 2026-04-20*
