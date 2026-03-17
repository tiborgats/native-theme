# Roadmap: native-theme

## Overview

native-theme delivers a toolkit-agnostic Rust crate for unified OS theme data. The roadmap builds outward from a stable data model core: first the types and serde layer, then bundled presets for immediate usability, then platform readers (Linux KDE/GNOME, Windows) each as independent feature-gated modules, then cross-platform dispatch tying them together, then extended preset coverage (platform + community themes), and finally documentation with toolkit adapter examples. Each phase produces a verifiable, independently useful increment.

## Milestones

- ✅ **v0.1 MVP** — Phases 1-8 (shipped 2026-03-07)
- ✅ **v0.2 Platform Coverage & Publishing** — Phases 9-15 (shipped 2026-03-09)
- ✅ **v0.3 Icons** — Phases 16-21 (shipped 2026-03-09)
- ✅ **v0.3.3 Custom Icon Roles** — Phases 22-26 (shipped 2026-03-17)

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

### v0.3.3 Custom Icon Roles (Phases 22-26)

- [x] **Phase 22: Core Trait and Loading Functions** - IconProvider trait with correct bounds + load_custom_icon() and load_system_icon_by_name() APIs + platform loader refactoring (completed 2026-03-15)
  **Plans:** 2 plans
  Plans:
  - [x] 22-01-PLAN.md -- IconProvider trait definition + IconRole impl + object safety tests
  - [x] 22-02-PLAN.md -- Platform _by_name loaders + load_system_icon_by_name + load_custom_icon
- [x] **Phase 23: Build Crate and Code Generation** - native-theme-build workspace crate with TOML parsing, enum codegen, IconProvider impl generation, include_bytes! SVG embedding, and build-time validation (completed 2026-03-16)
  **Plans:** 5 plans
  Plans:
  - [x] 23-01-PLAN.md -- Crate skeleton with TOML schema structs and error types
  - [x] 23-02-PLAN.md -- Validation logic (missing roles, SVGs, unknown roles, defaults, orphans, duplicates)
  - [x] 23-03-PLAN.md -- Code generation (enum, IconProvider impl, include_bytes! embedding)
  - [x] 23-04-PLAN.md -- Public API wiring (generate_icons, IconGenerator builder, rerun-if-changed, size report)
  - [x] 23-05-PLAN.md -- Gap closure: fix include_bytes! path to use relative base_dir
- [x] **Phase 24: Linux DE Audit and Freedesktop DE-Aware Mapping** - Verify DE detection coverage + DE-aware inline tables in mapping TOML + generated per-DE icon name dispatch (completed 2026-03-16)
  **Plans:** 2 plans
  Plans:
  - [x] 24-01-PLAN.md -- DE-aware code generation (TDD): de_key_to_variant + cfg-gated DE dispatch in generate_icon_name
  - [x] 24-02-PLAN.md -- LNXDE audit tests + DE key validation + pipeline integration wiring
- [x] **Phase 25: Connector Integration** - Generic IconProvider-aware helpers for gpui and iced connectors (completed 2026-03-16)
  **Plans:** 1 plan
  Plans:
  - [x] 25-01-PLAN.md -- Custom icon helpers for gpui (custom_icon_to_image_source) and iced (custom_icon_to_image_handle, custom_icon_to_svg_handle) connectors
- [x] **Phase 25.1: Icon Gaps and Fallback Removal** (INSERTED) - Fill remaining icon mapping gaps + remove cross-set Material fallback + add coverage tests (completed 2026-03-17)
  **Plans:** 2 plans
  Plans:
  - [x] 25.1-01-PLAN.md -- Fill 3 icon mapping gaps + add coverage tests (known-gaps + bundled SVG completeness)
  - [x] 25.1-02-PLAN.md -- Remove cross-set Material SVG fallback from platform loaders and load_icon
- [x] **Phase 26: Documentation and Release** - Complete docs for all new APIs + README updates + version bumps + changelog + release checks (completed 2026-03-17)
  **Plans:** 2 plans
  Plans:
  - [x] 26-01-PLAN.md -- Rustdoc for native-theme-build + IconProvider trait docs + build crate metadata
  - [x] 26-02-PLAN.md -- READMEs + CHANGELOG v0.3.3 + design doc fixes + pre-release script

## Phase Details

Phase details for completed milestones are archived in `.planning/milestones/`.

### Phase 22: Core Trait and Loading Functions
**Goal**: Apps can define custom icon types that integrate with native-theme's loading infrastructure on all platforms
**Depends on**: Phase 21 (v0.3 shipped)
**Requirements**: PROV-01, PROV-02, PROV-03, PROV-04, PROV-05, LOAD-01, LOAD-02, LOAD-03, LOAD-04, LOAD-05
**Success Criteria** (what must be TRUE):
  1. A hand-written struct implementing IconProvider can be passed to load_custom_icon() and returns correct IconData for a bundled SVG mapping
  2. load_system_icon_by_name("arrow.right") loads an SF Symbol on macOS, load_system_icon_by_name with a hex codepoint loads a Segoe Fluent glyph on Windows, and load_system_icon_by_name loads a freedesktop icon on Linux
  3. Existing code using IconRole and load_icon() compiles and behaves identically (backward compatible)
  4. IconProvider is object-safe, verified by a test that creates Box<dyn IconProvider>
  5. load_custom_icon follows the same fallback chain as load_icon: platform loader -> bundled SVG -> fallback set -> None
**Plans**: 2 plans

### Phase 23: Build Crate and Code Generation
**Goal**: App developers can declare domain-specific icons in TOML and get a generated enum with compile-time validated mappings -- no hand-written match arms
**Depends on**: Phase 22
**Requirements**: BUILD-01, BUILD-02, BUILD-03, BUILD-04, BUILD-05, BUILD-06, BUILD-07, BUILD-08, BUILD-09, BUILD-10, VAL-01, VAL-02, VAL-03, VAL-04, VAL-05, VAL-06
**Success Criteria** (what must be TRUE):
  1. A build.rs calling generate_icons("icons.toml") produces a compilable Rust file with an enum that implements IconProvider
  2. Adding a new role to the master TOML without updating all mapping files causes a build error naming the missing role and file
  3. Referencing a nonexistent SVG file in a bundled theme mapping causes a build error with the file path
  4. The builder API (IconGenerator::new().add(path).generate()) composes multiple TOML files, with duplicate role names across files producing a build error
  5. Generated include_bytes! paths resolve correctly when the consuming crate is in any directory (uses CARGO_MANIFEST_DIR, not relative paths)
**Plans**: 5 plans

### Phase 24: Linux DE Audit and Freedesktop DE-Aware Mapping
**Goal**: Custom icon mappings can specify different freedesktop icon names per Linux desktop environment, and all major DEs are detected correctly
**Depends on**: Phase 23
**Requirements**: FDES-01, FDES-02, FDES-03, LNXDE-01, LNXDE-02, LNXDE-03
**Success Criteria** (what must be TRUE):
  1. A mapping TOML entry like `{ kde = "view-visible", default = "view-reveal" }` generates code that returns "view-visible" on KDE and "view-reveal" on GNOME, XFCE, and all other DEs
  2. Omitting the `default` key in a DE-aware inline table causes a build error
  3. detect_linux_de() / LinuxDesktop enum handles KDE, GNOME, XFCE, Cinnamon, MATE, LXQt, Budgie, and falls back to a default for Hyprland, Sway, COSMIC, and any unknown DE
**Plans**: 2 plans

### Phase 25: Connector Integration
**Goal**: Both gpui and iced connector crates can load and display custom icons with the same ergonomics as built-in IconRole icons
**Depends on**: Phase 22
**Requirements**: CONN-01, CONN-02, CONN-03
**Success Criteria** (what must be TRUE):
  1. gpui connector exposes custom_icon_to_image_source<P: IconProvider>() that returns an ImageSource from any custom icon provider
  2. iced connector exposes custom_icon_to_image_handle<P: IconProvider>() and custom_icon_to_svg_handle<P: IconProvider>() with the same return types as the existing IconRole helpers
  3. Connector helpers follow the same code pattern and error handling as existing to_image_source() / to_image_handle() for built-in IconRole
**Plans**: 1 plan

### Phase 25.1: Icon Gaps and Fallback Removal (INSERTED)
**Goal**: All icon mapping gaps are filled (or explicitly documented as known gaps), cross-set Material fallback is removed from all platform loaders, and coverage tests prevent future regressions
**Depends on**: Phase 25
**Requirements**: GAP-01, GAP-02, GAP-03, COV-01, COV-02, FB-01, FB-02, FB-03, FB-04
**Success Criteria** (what must be TRUE):
  1. Freedesktop Notification maps to "notification-active", Material TrashFull maps to "delete", Lucide TrashFull maps to "trash-2"
  2. SF Symbols FolderOpen/StatusLoading and Segoe StatusLoading remain as explicitly documented known gaps (None)
  3. No platform loader (freedesktop.rs, sficons.rs, winicons.rs) imports or calls bundled_icon_svg -- they return None when the icon is not found
  4. load_icon() wildcard branch returns None instead of falling back to Material
  5. no_unexpected_icon_gaps test catches any new IconRole variant that lacks mappings
  6. all_roles_have_bundled_svg test verifies Material and Lucide cover all 42 roles
**Plans**: 2 plans

### Phase 26: Documentation and Release
**Goal**: The custom icon roles feature is fully documented, all tests pass, and the crate is ready for release
**Depends on**: Phase 23, Phase 24, Phase 25
**Requirements**: DOC-01, DOC-02, DOC-03, DOC-04, DOC-05, DOC-06, DOC-07, REL-01, REL-02, REL-03, REL-04, REL-05, REL-06
**Success Criteria** (what must be TRUE):
  1. native-theme-build has crate-level rustdoc with a complete example showing TOML schema, build.rs setup, SVG directory layout, and load_custom_icon() usage
  2. Running cargo doc --workspace --no-deps produces zero warnings and all new public items have doc comments
  3. pre-release-check.sh passes cleanly (tests, clippy, fmt, semver-checks)
  4. Core crate README documents the custom icon roles workflow end-to-end
  5. CHANGELOG.md covers all v0.3.3 additions with links to relevant types and functions
**Plans**: 2 plans

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 22. Core Trait and Loading Functions | v0.3.3 | 2/2 | ✓ Complete | 2026-03-15 |
| 23. Build Crate and Code Generation | v0.3.3 | 5/5 | ✓ Complete | 2026-03-16 |
| 24. Linux DE Audit and Freedesktop DE-Aware Mapping | v0.3.3 | 2/2 | ✓ Complete | 2026-03-16 |
| 25. Connector Integration | v0.3.3 | 1/1 | ✓ Complete | 2026-03-16 |
| 25.1. Icon Gaps and Fallback Removal | v0.3.3 | 2/2 | ✓ Complete | 2026-03-17 |
| 26. Documentation and Release | v0.3.3 | 2/2 | ✓ Complete | 2026-03-17 |
