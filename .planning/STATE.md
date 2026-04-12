---
gsd_state_version: 1.0
milestone: v0.5.7
milestone_name: API Overhaul
status: phase_complete
stopped_at: Phase 69 complete — verified, ready to plan Phase 70
last_updated: "2026-04-12T09:52:54.475Z"
last_activity: 2026-04-12
progress:
  total_phases: 28
  completed_phases: 9
  total_plans: 16
  completed_plans: 16
  percent: 100
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-04-12)

**Core value:** Any Rust GUI app can look native on any platform by loading a single theme file or reading live OS settings, without coupling to any specific toolkit.
**Current focus:** v0.5.7 API Overhaul — roadmap complete, 20 phases (69–88) ready to plan

## Current Position

Phase: 69 — resolver-button-order-unlock (COMPLETE, verified)
Plan: 2/2 complete, verified 5/5 must-haves
Status: Phase 69 complete — ready to plan Phase 70
Last activity: 2026-04-12 — Phase 69 executed and verified

Progress: [░░░░░░░░░░] 5% (1/20 phases complete)

## Accumulated Context

### Decisions (carried from v0.5.6)

All decisions logged in PROJECT.md Key Decisions table.

v0.5.6 decisions worth carrying into v0.5.7 planning:

- [Phase 61-01]: detect.rs narrow pub(crate) accessors pattern established
- [Phase 62-01]: `pub(crate)` visibility on resolve::validate for macro-generated code
- [Phase 62-02]: impl_merge! supports repeated optional_nested blocks for mixed border categories
- [Phase 62-03]: validate.rs split into validate_helpers.rs + per-widget check_ranges() methods
- [Phase 63-01]: `_pure` suffix convention for I/O-free parse functions (from_kde_content_pure)
- [Phase 65-01]: ThemeWatcher Debug derive; notify crate Linux-only target section
- [Phase 66-01]: zbus direct dep with blocking-api feature; watch feature gates notify + zbus
- [Phase 66-02]: cfg-gated match arms per DE feature
- [Phase 67-01]: Box<dyn FnOnce() + Send> platform_shutdown for immediate wakeup on Drop
- [Phase 67-02]: GetCurrentThreadId via oneshot channel for PostThreadMessageW(WM_QUIT)
- [Phase 68]: Raw string literals br##"..."## for test SVGs containing # hex colors

### v0.5.7 Roadmap Summary

**Source documents** (pre-milestone design work, 9,700+ lines, six verification passes):

- `docs/todo_v0.5.7_native-theme-api.md` — API critique §1-§33 (doc 1)
- `docs/todo_v0.5.7_native-theme-api-2.md` — Bugs/structural/API-shape §A-§M (doc 2)

**Roadmap:** 20 phases (69–88), 55 requirements, 100% coverage, granularity=fine.

**Ship unit mapping:**

- Phase 69 — Unit 1 atomic: BUG-03 + BUG-04 + BUG-05 (resolver button_order unlock)
- Phase 70 — Unit 3 atomic: ERR-01 + CLEAN-01 (drop Error::Clone)
- Phase 71 — Unit 2 atomic: BUG-01 + BUG-02 + ERR-02 (validation split + Error restructure)
- Phase 72 — Unit 4 (after Unit 1): CLEAN-02 (ENV_MUTEX test simplification)
- Phase 73 — Unit 5: WATCH-01 + WATCH-02 (ThemeChangeEvent cleanup)
- Phase 74 — Unit 6 part A: COLOR-01 + POLISH-03 (Rgba polish + must_use uniformity)
- Phase 75 — Unit 6 part B: LAYOUT-02 + WATCH-03 + ICON-05 (non_exhaustive + compile-gate + IconSet::default removal)
- Phase 76 — Unit 7 part A: NAME-01 + LAYOUT-01 (type rename + crate root partition)
- Phase 77 — Unit 7 part B: MODEL-03 + MODEL-06 (pick(ColorMode) + icon_set relocation)
- Phase 78 — Unit 8 atomic: MODEL-02 + ACCESS-01 + ACCESS-02 (OverlaySource + AccessibilityPreferences + font_dpi)
- Phase 79 — Unit 9: BORDER-01 + CLEAN-03 + READER-02 (border split + reader visibility audit)
- Phase 80 — Unit 10: MODEL-01 + VALID-01 + VALID-02 + BORDER-02 (native-theme-derive proc-macro K codegen)
- Phase 81 — Unit 11 atomic: FEATURE-01 + FEATURE-02 + FEATURE-03 (feature-matrix cleanup)

**Non-ship-unit bundles:**

- Phase 82 — Icon API rework: ICON-01, ICON-02, ICON-03, ICON-04, ICON-06, ICON-07
- Phase 83 — Detection cache layer: DETECT-01, DETECT-02
- Phase 84 — Reader output contract homogenisation: READER-01
- Phase 85 — Data model method and doc cleanup: MODEL-04, MODEL-05, NAME-02, NAME-03
- Phase 86 — Validation and lint codegen polish: VALID-03, VALID-04
- Phase 87 — Font family Arc<str> and AnimatedIcon invariants: LAYOUT-03, LAYOUT-04
- Phase 88 — Diagnostic and preset-polish sweep: POLISH-01, POLISH-02, POLISH-04, POLISH-05, POLISH-06

### Pending Todos

None. Ready to plan Phase 70.

### Blockers/Concerns

- `AccessibilityPreferences` relocation from `ThemeDefaults` to `SystemTheme` (Phase 78) is a cross-cutting refactor; touches resolve engine, connectors, and all presets
- Proc-macro codegen (Phase 80, Unit 10) is a P1 investment with ~1 week estimate; inheritance-expressiveness unknown flagged as medium-confidence
- §1 type rename + §12 crate-root partition (Phase 76) touches connectors (gpui, iced) in lockstep
- C4 `Arc<str>` font family migration (Phase 87) needs `serde rc` feature flag and connector-side `.family` access migration
- Phase 80 depends on Phase 71 (needs new Error shape) AND Phase 79 (needs clean border target) — longest dependency chain in the milestone
- Phase 81 must ship last — absorbs every other change before the feature graph is re-cut

## Session Continuity

Last session: 2026-04-12
Stopped at: Phase 69 complete — verified, ready to plan Phase 70
Resume file: .planning/ROADMAP.md (Phase 70 section)
