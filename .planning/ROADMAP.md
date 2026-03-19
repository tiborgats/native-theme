# Roadmap: native-theme

## Overview

native-theme delivers a toolkit-agnostic Rust crate for unified OS theme data. The roadmap builds outward from a stable data model core: first the types and serde layer, then bundled presets for immediate usability, then platform readers (Linux KDE/GNOME, Windows) each as independent feature-gated modules, then cross-platform dispatch tying them together, then extended preset coverage (platform + community themes), and finally documentation with toolkit adapter examples. Each phase produces a verifiable, independently useful increment.

## Milestones

- ✅ **v0.1 MVP** — Phases 1-8 (shipped 2026-03-07)
- ✅ **v0.2 Platform Coverage & Publishing** — Phases 9-15 (shipped 2026-03-09)
- ✅ **v0.3 Icons** — Phases 16-21 (shipped 2026-03-09)
- ✅ **v0.3.3 Custom Icon Roles** — Phases 22-26 (shipped 2026-03-17)
- ✅ **v0.4.0 Animated Icons** — Phases 27-32 (shipped 2026-03-18)
- 🔄 **v0.4.1 Release Prep** — Phases 33-38

## Phases

<details>
<summary>✅ v0.1 MVP (Phases 1-8) — SHIPPED 2026-03-07</summary>

- [x] Phase 1: Data Model Foundation (3/3 plans) — completed 2026-03-07
- [x] Phase 2: Core Presets (2/2 plans) — completed 2026-03-07
- [x] Phase 3: KDE Reader (2/2 plans) — completed 2026-03-07
- [x] Phase 4: GNOME Portal Reader (2/2 plans) — completed 2026-03-07
- [x] Phase 5: Windows Reader (1/1 plan) — completed 2026-03-07
- [x] Phase 6: Cross-Platform Dispatch (1/1 plan) — completed 2026-03-07
- [x] Phase 7: Extended Presets (2/2 plans) — completed 2026-03-07
- [x] Phase 8: Documentation (1/1 plan) — completed 2026-03-07

</details>

<details>
<summary>✅ v0.2 Platform Coverage & Publishing (Phases 9-15) — SHIPPED 2026-03-09</summary>

- [x] Phase 9: Cargo Workspace (1/1 plan) — completed 2026-03-08
- [x] Phase 10: API Breaking Changes (3/3 plans) — completed 2026-03-08
- [x] Phase 11: Platform Readers (4/4 plans) — completed 2026-03-08
- [x] Phase 12: Widget Metrics (3/3 plans) — completed 2026-03-08
- [x] Phase 13: CI Pipeline (1/1 plan) — completed 2026-03-08
- [x] Phase 14: Toolkit Connectors (5/5 plans) — completed 2026-03-09
- [x] Phase 15: Publishing Prep (3/3 plans) — completed 2026-03-09

</details>

<details>
<summary>✅ v0.3 Icons (Phases 16-21) — SHIPPED 2026-03-09</summary>

- [x] Phase 16: Icon Data Model (2/2 plans) — completed 2026-03-09
- [x] Phase 17: Bundled SVG Icons (2/2 plans) — completed 2026-03-09
- [x] Phase 18: Linux Icon Loading (1/1 plan) — completed 2026-03-09
- [x] Phase 19: macOS Icon Loading (1/1 plan) — completed 2026-03-09
- [x] Phase 20: Windows Icon Loading (1/1 plan) — completed 2026-03-09
- [x] Phase 21: Integration and Connectors (3/3 plans) — completed 2026-03-09

</details>

<details>
<summary>✅ v0.3.3 Custom Icon Roles (Phases 22-26) — SHIPPED 2026-03-17</summary>

- [x] Phase 22: Core Trait and Loading Functions (2/2 plans) — completed 2026-03-15
- [x] Phase 23: Build Crate and Code Generation (5/5 plans) — completed 2026-03-16
- [x] Phase 24: Linux DE Audit and Freedesktop DE-Aware Mapping (2/2 plans) — completed 2026-03-16
- [x] Phase 25: Connector Integration (1/1 plan) — completed 2026-03-16
- [x] Phase 25.1: Icon Gaps and Fallback Removal (2/2 plans) — completed 2026-03-17
- [x] Phase 26: Documentation and Release (2/2 plans) — completed 2026-03-17

</details>

<details>
<summary>✅ v0.4.0 Animated Icons (Phases 27-32) — SHIPPED 2026-03-18</summary>

- [x] Phase 27: Animation Data Model and Breaking Changes (2/2 plans) — completed 2026-03-18
- [x] Phase 28: Bundled SVG Spinner Frames (2/2 plans) — completed 2026-03-18
- [x] Phase 29: Freedesktop Sprite Sheet Parser (1/1 plan) — completed 2026-03-18
- [x] Phase 30: Reduced Motion Accessibility (1/1 plan) — completed 2026-03-18
- [x] Phase 31: Connector Integration (1/1 plan) — completed 2026-03-18
- [x] Phase 32: Documentation and Release (1/1 plan) — completed 2026-03-18

</details>

### v0.4.1 Release Prep (Phases 33-38)

- [x] Phase 33: Quick Fixes and Version Consistency (completed 2026-03-19)
- [ ] Phase 34: Animated Icon Documentation
- [ ] Phase 35: Animated Icon Showcase Examples
- [ ] Phase 36: Screenshot and GIF Generation
- [ ] Phase 37: Community Files and GitHub Templates
- [ ] Phase 38: CI, Smoke Tests, and Release

### Phase 33: Quick Fixes and Version Consistency

**Goal:** Fix version references (0.3→0.4), license text, lint attributes, crate version consistency, and archive design docs
**Depends on:** Phase 32
**Plans:** 1/1 plans complete

Plans:
- [x] 33-01-PLAN.md — Fix version refs, license text, lint attributes, verify versions, archive design docs (completed 2026-03-19)

### Phase 34: Animated Icon Documentation

**Goal:** Add animated icon sections to connector READMEs and root README documenting the v0.4.0 headline feature
**Depends on:** Phase 33
**Plans:** 1 plan

Plans:
- [ ] 34-01-PLAN.md — Add Animated Icons sections to root, gpui, and iced READMEs

### Phase 35: Animated Icon Showcase Examples

**Goal:** Add animated icon demonstrations to both gpui and iced showcase examples
**Depends on:** Phase 33
**Plans:** 0 plans

Plans:
- [ ] TBD (run /gsd:plan-phase 35 to break down)

### Phase 36: Screenshot and GIF Generation

**Goal:** Create visual assets (showcase screenshots, spinner GIFs) and automation tooling for generating them
**Depends on:** Phase 35
**Plans:** 0 plans

Plans:
- [ ] TBD (run /gsd:plan-phase 36 to break down)

### Phase 37: Community Files and GitHub Templates

**Goal:** Add CONTRIBUTING.md, CODE_OF_CONDUCT.md, SECURITY.md, and issue/PR templates
**Depends on:** Phase 33
**Plans:** 0 plans

Plans:
- [ ] TBD (run /gsd:plan-phase 37 to break down)

### Phase 38: CI, Smoke Tests, and Release

**Goal:** Verify CI coverage for animated icons, run pre-release smoke tests, tag v0.4.1, and publish to crates.io
**Depends on:** Phase 34, 35, 36, 37
**Plans:** 0 plans

Plans:
- [ ] TBD (run /gsd:plan-phase 38 to break down)

## Phase Details

Phase details for completed milestones are archived in `.planning/milestones/`.

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1-8 | v0.1 | 14/14 | Complete | 2026-03-07 |
| 9-15 | v0.2 | 20/20 | Complete | 2026-03-09 |
| 16-21 | v0.3 | 10/10 | Complete | 2026-03-09 |
| 22-26 | v0.3.3 | 14/14 | Complete | 2026-03-17 |
| 27-32 | v0.4.0 | 8/8 | Complete | 2026-03-18 |
| 33-38 | v0.4.1 | 0/? | In Progress | — |
