---
gsd_state_version: 1.0
milestone: v0.1
milestone_name: milestone
current_plan: 2 of 2
status: phase-complete
stopped_at: Completed 43-02-PLAN.md
last_updated: "2026-03-21T22:10:00.000Z"
last_activity: 2026-03-21 — Completed plan 43-02 (CI and scripts update for self-capture)
progress:
  total_phases: 11
  completed_phases: 11
  total_plans: 21
  completed_plans: 21
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-03-18)

**Core value:** Any Rust GUI app can look native on any platform by loading a single theme file or reading live OS settings, without coupling to any specific toolkit.
**Current focus:** Phase 43 — Self-Capture Screenshots with Window Decorations

## Current Position

Phase: 43-self-capture-screenshots-with-window-decorations
Current Plan: 2 of 2 (phase complete)
Last activity: 2026-03-21 — Completed plan 43-02 (CI and scripts update for self-capture)

## Performance Metrics

**Velocity:**
- Total plans completed: 95 (14 v0.1 + 20 v0.2 + 10 v0.3 + 4 v0.3.2 + 14 v0.3.3 + 8 v0.4.0 + 22 v0.4.1 + 3 v0.4.2)
- Average duration: ~4.1min (v0.2), 3.7min (v0.3)
- Total execution time: 70min (v0.2), 37min (v0.3), 15min (v0.3.2), 35min (v0.3.3), 35min (v0.4.0)

## Accumulated Context

### Decisions

All decisions logged in PROJECT.md Key Decisions table.
- [Phase 34]: Kept root README animated section shorter than core crate README since it is a workspace overview
- [Phase 35-02]: Used 50ms tick interval with per-animation frame duration tracking; subscription gated to Icons tab
- [Phase 35-01]: Used opacity pulse for spin animations since gpui Div lacks rotation; AnyElement for heterogeneous cards
- [Phase 36-02]: RGB GIF frames on white background (no GIF transparency); single GIF set since SVGs are toolkit-agnostic
- [Phase 36-03]: Pre-build release binaries before capture loop; spectacle -a requires showcase window focus on KDE Wayland
- [Phase 36-01]: Used std::env::args() only (no clap); OnceLock for iced State::default() CLI arg passing
- [Phase 37-02]: Used YAML issue forms (not Markdown templates) for validated dropdowns and required fields
- [Phase 38-01]: Used real doc comments (not blanket allow) for all 67 undocumented public items
- [Phase 38-02]: Connector crate dry-runs use run_check_soft since they depend on core crate being published first
- [Phase 38-03]: Enhanced release notes to include screenshot automation and visual asset CI additions beyond original template
- [Phase 39-02]: Archive completed milestone docs to docs/archive/ using git mv for history preservation
- [Phase 39-01]: Used deny(unsafe_code) instead of forbid(unsafe_code) because forbid cannot be overridden by allow in inner scopes
- [Phase 40]: Used 4 specific theme+variant pairings (dracula/dark, nord/light, catppuccin-mocha/dark, macos-sonoma/light) for CI and local screenshots
- [Phase 40]: Old icon-set screenshots already removed by plan 40-01; README tables updated to reference 4 theme presets
- [Phase 41]: Used spectacle external capture since gpui has no window::screenshot() API
- [Phase 41]: Added --icon-theme CLI arg to gpui showcase for explicit freedesktop icon theme selection
- [Phase 41]: Used 3 Linux-native presets (KDE Breeze, Material, Catppuccin Mocha) × dark+light with matching icon themes
- [Phase 41]: Adwaita needs GNOME, macOS Sonoma/Windows 11 need CI on native runners
- [Phase 42]: Split theme-switching GIF into per-toolkit (iced + gpui) instead of single combined
- [Phase 42]: Used same 4 Linux-native presets as Phase 41 with matching icon sets
- [Phase 42]: Standard markdown for GIF embedding (not HTML)
- [Phase 43-01]: Used screencapture CLI (not CGWindowListCreateImage) to avoid macOS 15 deprecation
- [Phase 43-01]: gpui uses raw-window-handle HasWindowHandle; iced uses NSApplication mainWindow for CGWindowID
- [Phase 43-01]: Linux gpui prints message and continues; Linux iced preserves existing framebuffer capture
- [Phase 43-02]: TCC.db grants both /bin/bash and /bin/zsh for macOS CI screen recording permission
- [Phase 43-02]: gpui CI screenshots only on macOS; local Linux scripts unchanged (spectacle)

### Roadmap Evolution

Phase history archived in .planning/milestones/.
- Phase 33-38 added: v0.4.1 Release Prep milestone (quick fixes, docs, examples, screenshots, community files, release)
- Phases 39-42 added: Finish incomplete v0.4.1 work (code quality, theme preset screenshots, GIF assets)
- Phase 43 added: Self-Capture Screenshots with Window Decorations

### Pending Todos

None.

### Blockers/Concerns

- v0.4.1 was published prematurely — screenshots show wrong content (icon sets, not theme presets), gpui screenshots missing, theme-switching GIF never created, core README has no images, forbid(unsafe_code) missing from core, prefers_reduced_motion() untested, design docs not archived
- Manual visual verification required before milestone completion (item 8 from docs/todo_v0.4.1.md)

## Session Continuity

Last session: 2026-03-21T22:10:00Z
Stopped at: Completed 43-02-PLAN.md (Phase 43 complete)
Resume file: None
