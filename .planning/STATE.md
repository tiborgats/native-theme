---
gsd_state_version: 1.0
milestone: v0.5.0
milestone_name: Per-Widget Architecture & Resolution Pipeline
status: executing
stopped_at: Completed 47-01-PLAN.md
last_updated: "2026-03-27T14:06:00.589Z"
last_activity: 2026-03-27
progress:
  total_phases: 5
  completed_phases: 3
  total_plans: 14
  completed_plans: 13
  percent: 46
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-03-27)

**Core value:** Any Rust GUI app can look native on any platform by loading a single theme file or reading live OS settings, without coupling to any specific toolkit.
**Current focus:** Phase 46 complete — next: Phase 47

## Current Position

Phase: 47 (4 of 5 in v0.5.0) — OS-First Pipeline
Plan: 1 of 2 complete
Status: Executing — Plan 01 complete, Plan 02 pending
Last activity: 2026-03-27

Progress: [████▌░░░░░] 46%

## Performance Metrics

**Velocity:**

- Total plans completed: 96 (across v0.1-v0.4.1)
- Average duration: ~4.1min (v0.2), 3.7min (v0.3)
- Total execution time: 70min (v0.2), 37min (v0.3), 15min (v0.3.2), 35min (v0.3.3), 35min (v0.4.0)

## Accumulated Context

### Decisions

All decisions logged in PROJECT.md Key Decisions table.

Recent decisions from research:

- ThemeVariant restructure + preset TOML rewrites must be atomic (serde keys conflict between old and new shapes)
- define_widget_pair! macro generates Option + Resolved struct pairs from single definition (not proc macro, not optional_struct crate)
- resolve() uses explicit 4-phase structure: defaults internal chains, defaults safety nets, widget-from-defaults, widget-to-widget chains
- Qt5/Qt6 font weight detection via field count (<=16 fields = Qt5, 17+ = Qt6)
- Zero new crate dependencies for v0.5.0; only 2 new windows crate feature flags

Decisions from 44-01 and 44-02:

- define_widget_pair! optional_nested uses [OptType, ResType] bracket syntax (Rust ty/path fragments cannot precede / token)
- DialogButtonOrder serde tests require wrapper struct (TOML cannot serialize bare enum as top-level value)
- [Phase 44]: ThemeDefaults non-Option nested structs use skip_serializing_if per-field to suppress empty TOML sections
- [Phase 44]: ResolvedXxx types named without Theme suffix (ResolvedWindow not ResolvedWindowTheme) to avoid double suffix
- [Phase 44-03]: impl_merge! nested clause auto-generates is_empty() — no manual impl needed on ThemeVariant
- [Phase 44-03]: NativeTheme needs PartialEq derive for round-trip equality tests; SplitterTheme must be in pub use exports
- [Phase 44-03]: TOML preset icon_set stored at [light]/[dark] level (not inside [defaults]); widget colors in their own [v.widget] tables
- [Phase 45]: [Phase 45-01]: ResolvedTheme uses explicit per-widget fields (not HashMap) matching ThemeVariant for type safety
- [Phase 45]: [Phase 45-01]: No serde derives on Resolved types -- output-only consumed by connectors
- [Phase 45-resolution-engine]: [Phase 45-02]: resolve() uses 4-phase mutation on ThemeVariant; validate() collects all missing fields before returning error
- [Phase 45]: icon_set = freedesktop added to community/default presets for validate() pipeline
- [Phase 45]: Platform dialog button_order: KDE/macOS leading_affirmative, GNOME/Windows trailing_affirmative
- [Phase 46-04]: Windows module always compiled (cfg_attr dead_code on non-Windows) enabling 31 unit tests on Linux
- [Phase 46-04]: AllFonts struct collects NONCLIENTMETRICSW fonts before build_theme distributes to per-widget structs
- [Phase 46]: GNOME apply_accent targets 3 fields (accent, selection, focus_ring_color); weight extraction uses longest-match-first suffix matching
- [Phase 46]: input.caret intentionally not read from macOS (textInsertionPointColor requires macOS 14+); resolve() safety net fills from defaults.foreground
- [Phase 46]: macOS text scale uses proportional computation from system font size (Apple's known ratios) rather than NSFontTextStyle API
- [Phase 46]: KDE reader merges with default preset before resolve/validate (sparse reader + preset = complete theme)
- [Phase 46-05]: Icon sizes derived from index.theme Context+Size: small=smallest Actions/Status, toolbar=closest-to-22 Actions, large=smallest Applications>=32
- [Phase 46]: GNOME integration test placed in resolve.rs (not gnome/mod.rs) due to portal feature gate compilation issue
- [Phase 47]: SystemTheme in lib.rs (not separate module), pre-resolve variants retained for overlay, reader_is_dark for cross-platform is_dark inference

### Roadmap Evolution

Phase history archived in .planning/milestones/.

- Phases 44-48 added: v0.5.0 Per-Widget Architecture & Resolution Pipeline

### Pending Todos

None.

### Blockers/Concerns

- macOS reader extensions cannot be fully tested on Linux dev machine (Windows reader now testable via module cfg change)
- gpui connector field mapping not yet documented in research (read source during Phase 48 planning)

## Session Continuity

Last session: 2026-03-27T14:06:00.587Z
Stopped at: Completed 47-01-PLAN.md
Resume file: None
