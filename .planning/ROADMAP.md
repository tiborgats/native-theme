# Roadmap: native-theme

## Overview

native-theme delivers a toolkit-agnostic Rust crate for unified OS theme data. The roadmap builds outward from a stable data model core: first the types and serde layer, then bundled presets for immediate usability, then platform readers (Linux KDE/GNOME, Windows) each as independent feature-gated modules, then cross-platform dispatch tying them together, then extended preset coverage (platform + community themes), and finally documentation with toolkit adapter examples. Each phase produces a verifiable, independently useful increment.

## Milestones

- ✅ **v0.1 MVP** — Phases 1-8 (shipped 2026-03-07)
- ✅ **v0.2 Platform Coverage & Publishing** — Phases 9-15 (shipped 2026-03-09)
- ✅ **v0.3 Icons** — Phases 16-21 (shipped 2026-03-09)
- ✅ **v0.3.3 Custom Icon Roles** — Phases 22-26 (shipped 2026-03-17)
- 🚧 **v0.4.0 Animated Icons** — Phases 27-32 (in progress)

## Phases

<details>
<summary>v0.1 MVP (Phases 1-8) — SHIPPED 2026-03-07</summary>

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
<summary>v0.2 Platform Coverage & Publishing (Phases 9-15) — SHIPPED 2026-03-09</summary>

- [x] Phase 9: Cargo Workspace (1/1 plan) — completed 2026-03-08
- [x] Phase 10: API Breaking Changes (3/3 plans) — completed 2026-03-08
- [x] Phase 11: Platform Readers (4/4 plans) — completed 2026-03-08
- [x] Phase 12: Widget Metrics (3/3 plans) — completed 2026-03-08
- [x] Phase 13: CI Pipeline (1/1 plan) — completed 2026-03-08
- [x] Phase 14: Toolkit Connectors (5/5 plans) — completed 2026-03-09
- [x] Phase 15: Publishing Prep (3/3 plans) — completed 2026-03-09

</details>

<details>
<summary>v0.3 Icons (Phases 16-21) — SHIPPED 2026-03-09</summary>

- [x] Phase 16: Icon Data Model (2/2 plans) — completed 2026-03-09
- [x] Phase 17: Bundled SVG Icons (2/2 plans) — completed 2026-03-09
- [x] Phase 18: Linux Icon Loading (1/1 plan) — completed 2026-03-09
- [x] Phase 19: macOS Icon Loading (1/1 plan) — completed 2026-03-09
- [x] Phase 20: Windows Icon Loading (1/1 plan) — completed 2026-03-09
- [x] Phase 21: Integration and Connectors (3/3 plans) — completed 2026-03-09

</details>

<details>
<summary>v0.3.3 Custom Icon Roles (Phases 22-26) — SHIPPED 2026-03-17</summary>

- [x] Phase 22: Core Trait and Loading Functions (2/2 plans) — completed 2026-03-15
- [x] Phase 23: Build Crate and Code Generation (5/5 plans) — completed 2026-03-16
- [x] Phase 24: Linux DE Audit and Freedesktop DE-Aware Mapping (2/2 plans) — completed 2026-03-16
- [x] Phase 25: Connector Integration (1/1 plan) — completed 2026-03-16
- [x] Phase 25.1: Icon Gaps and Fallback Removal (2/2 plans) — completed 2026-03-17
- [x] Phase 26: Documentation and Release (2/2 plans) — completed 2026-03-17

</details>

### v0.4.0 Animated Icons (In Progress)

- [ ] **Phase 27: Animation Data Model and Breaking Changes** - AnimatedIcon/TransformAnimation types, loading_indicator() API, StatusLoading removal
- [ ] **Phase 28: Bundled SVG Spinner Frames** - Programmatic SVG frame generation for Material, Lucide, macOS, Windows, and GNOME spinners
- [ ] **Phase 29: Freedesktop Sprite Sheet Parser** - Runtime parsing of freedesktop process-working sprite sheets into animation frames
- [ ] **Phase 30: Reduced Motion Accessibility** - prefers_reduced_motion() OS query across Linux, macOS, and Windows
- [ ] **Phase 31: Connector Integration** - AnimatedIcon playback support in gpui and iced connectors
- [ ] **Phase 32: Documentation and Release** - API docs, CHANGELOG, migration guide for StatusLoading removal

## Phase Details

Phase details for completed milestones are archived in `.planning/milestones/`.

### Phase 27: Animation Data Model and Breaking Changes
**Goal**: Consumers can construct, inspect, and pattern-match on AnimatedIcon values and call loading_indicator() (which returns None until frames are implemented in Phase 28)
**Depends on**: Phase 26 (v0.3.3 complete)
**Requirements**: ANIM-01, ANIM-02, ANIM-03, ANIM-04, ANIM-05, ANIM-06, BREAK-01, BREAK-02
**Success Criteria** (what must be TRUE):
  1. Code can construct AnimatedIcon::Frames with a Vec<IconData>, frame_duration_ms, and Repeat value
  2. Code can construct AnimatedIcon::Transform with an IconData and TransformAnimation::Spin { duration_ms }
  3. calling first_frame() on either variant returns the expected IconData reference
  4. loading_indicator("material") compiles and returns Option<AnimatedIcon> (None until Phase 28 wires it)
  5. StatusLoading no longer exists in the IconRole enum and all existing code compiles without it
**Plans**: TBD

### Phase 28: Bundled SVG Spinner Frames
**Goal**: loading_indicator() returns platform-appropriate bundled SVG animation data for all five icon sets (Material, Lucide, macOS, Windows, GNOME)
**Depends on**: Phase 27
**Requirements**: SPIN-01, SPIN-02, SPIN-03, SPIN-04, SPIN-05, SPIN-06, SPIN-07
**Success Criteria** (what must be TRUE):
  1. loading_indicator("material") returns AnimatedIcon::Frames with circular arc SVG frames at 24x24 viewBox
  2. loading_indicator("lucide") returns AnimatedIcon::Transform::Spin wrapping the Lucide loader icon SVG
  3. loading_indicator("macos") returns Frames with 12 radial spoke SVG frames, loading_indicator("windows") returns Frames with arc expansion frames, loading_indicator("adwaita") returns Frames with overlapping arc frames
  4. Every bundled SVG frame rasterizes successfully through resvg in tests
  5. All bundled frame sets are embedded via include_bytes!() and gated on their respective feature flags
**Plans**: TBD

### Phase 29: Freedesktop Sprite Sheet Parser
**Goal**: Linux users with freedesktop-compliant icon themes get theme-native loading animations parsed from their installed theme's process-working icon
**Depends on**: Phase 28
**Requirements**: FD-01, FD-02, FD-03, FD-04
**Success Criteria** (what must be TRUE):
  1. A vertical SVG sprite sheet (like Breeze's process-working.svg) is split into individual SVG frame strings via viewBox rewriting
  2. A single-frame process-working-symbolic icon produces AnimatedIcon::Transform::Spin instead of Frames
  3. loading_indicator("freedesktop") returns theme-native AnimatedIcon when a sprite sheet exists in the active icon theme
  4. When no freedesktop sprite sheet is found, loading_indicator("freedesktop") falls back to bundled GNOME/Adwaita frames
**Plans**: TBD

### Phase 30: Reduced Motion Accessibility
**Goal**: Applications can query the OS-level reduced motion preference to decide whether to animate or show a static frame
**Depends on**: Phase 27
**Requirements**: A11Y-01, A11Y-02, A11Y-03, A11Y-04, A11Y-05
**Success Criteria** (what must be TRUE):
  1. prefers_reduced_motion() returns bool and caches the result via OnceLock (subsequent calls are free)
  2. On Linux, the function queries org.gnome.desktop.interface enable-animations via gsettings subprocess
  3. On macOS, the function queries NSWorkspace.accessibilityDisplayShouldReduceMotion
  4. On Windows, the function queries UISettings.AnimationsEnabled()
  5. On unsupported platforms or when the query fails, the function returns false (allow animations)
**Plans**: TBD

### Phase 31: Connector Integration
**Goal**: gpui and iced applications can play AnimatedIcon data as visible animations using each toolkit's native animation primitives
**Depends on**: Phase 28, Phase 30
**Requirements**: CONN-01, CONN-02, CONN-03, CONN-04
**Success Criteria** (what must be TRUE):
  1. gpui connector converts AnimatedIcon::Frames into timer-driven frame cycling that advances the displayed image
  2. gpui connector converts AnimatedIcon::Transform::Spin into continuous rotation via AnimationExt with Transformation::rotate()
  3. iced connector converts AnimatedIcon::Frames into frame cycling driven by time::every() subscription
  4. iced connector converts AnimatedIcon::Transform::Spin into Svg::rotation() with a timer-based angle
**Plans**: TBD

### Phase 32: Documentation and Release
**Goal**: Consumers migrating from v0.3.3 can find clear documentation for all new animated icon types and a step-by-step migration path away from StatusLoading
**Depends on**: Phase 31
**Requirements**: DOC-01, DOC-02, DOC-03
**Success Criteria** (what must be TRUE):
  1. All public types (AnimatedIcon, TransformAnimation, Repeat) and functions (loading_indicator, prefers_reduced_motion, first_frame) have rustdoc with examples
  2. CHANGELOG entry documents new features and lists StatusLoading removal as a breaking change with migration steps
  3. A migration guide section shows before/after code for replacing StatusLoading with loading_indicator()
**Plans**: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 27 -> 28 -> 29 -> 30 -> 31 -> 32
Note: Phase 29 and Phase 30 can execute in parallel (both depend on 27/28, neither depends on the other).

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 27. Animation Data Model and Breaking Changes | v0.4.0 | 0/? | Not started | - |
| 28. Bundled SVG Spinner Frames | v0.4.0 | 0/? | Not started | - |
| 29. Freedesktop Sprite Sheet Parser | v0.4.0 | 0/? | Not started | - |
| 30. Reduced Motion Accessibility | v0.4.0 | 0/? | Not started | - |
| 31. Connector Integration | v0.4.0 | 0/? | Not started | - |
| 32. Documentation and Release | v0.4.0 | 0/? | Not started | - |
