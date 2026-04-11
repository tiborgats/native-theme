---
gsd_state_version: 1.0
milestone: v0.5.7
milestone_name: API Overhaul
status: defining_requirements
stopped_at: Defining v0.5.7 requirements from design docs
last_updated: "2026-04-12T00:00:00.000Z"
last_activity: 2026-04-12
progress:
  total_phases: 0
  completed_phases: 0
  total_plans: 0
  completed_plans: 0
  percent: 0
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-04-12)

**Core value:** Any Rust GUI app can look native on any platform by loading a single theme file or reading live OS settings, without coupling to any specific toolkit.
**Current focus:** v0.5.7 API Overhaul — defining requirements from `docs/todo_v0.5.7_native-theme-api*.md`

## Current Position

Phase: Not started (defining requirements)
Plan: —
Status: Defining requirements
Last activity: 2026-04-12 — Milestone v0.5.7 started

Progress: [░░░░░░░░░░] 0%

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

### v0.5.7 Design Context

Source documents (pre-milestone design work, 9,700+ lines, six verification passes):
- `docs/todo_v0.5.7_native-theme-api.md` — API critique §1-§33 (doc 1)
- `docs/todo_v0.5.7_native-theme-api-2.md` — Bugs/structural/API-shape §A-§M (doc 2)

Design verdict: "Ship v0.5.7" — scope closed, P0 cohort frozen, ship-unit sequencing established.
Cross-document consolidation: 15 P0 items + P1/P2/P3 follow-ups, 11 ship units with atomicity constraints.

User directive (2026-04-12): Implement EVERY proposed solution from both design docs — not just the P0 cohort.

### Roadmap Evolution

- v0.5.7 phase numbering continues from Phase 68 (last v0.5.6 phase) → starts at Phase 69
- Design docs already provide implementation-ready direction; research step skipped
- Scope: 45+ requirements across bugs, type vocabulary, error restructure, data model, accessibility, borders, validation, readers, watchers, icons, crate root, color, detection, features, polish, cleanup

### Pending Todos

None.

### Blockers/Concerns

- `AccessibilityPreferences` relocation from `ThemeDefaults` to `SystemTheme` is a cross-cutting refactor; touches resolve engine, connectors, and all presets
- Proc-macro codegen (K) is a P1 investment with ~1 week estimate; inheritance-expressiveness unknown flagged as medium-confidence
- §1 type rename + §12 crate-root partition touches connectors (gpui, iced) in lockstep
- C4 `Arc<str>` font family migration needs `serde rc` feature flag and connector-side `.family` access migration

## Session Continuity

Last session: 2026-04-12T00:00:00.000Z
Stopped at: Defining v0.5.7 requirements from design docs
Resume file: None
