# Project Research Summary

**Project:** native-theme v0.2
**Domain:** Cross-platform Rust library crate for OS theme data -- subsequent milestone extending v0.1
**Researched:** 2026-03-08
**Confidence:** HIGH

## Executive Summary

native-theme v0.2 extends a shipping ~7,000 LOC crate (140+ tests, 17 TOML presets, 3 platform readers) with six categories of work: breaking API refactors (flatten ThemeColors from 7 nested sub-structs to 36 direct fields, move free functions to `impl NativeTheme`), a macOS reader via `objc2-app-kit 0.3.2`, a widget metrics data model (12 per-widget sub-structs), Cargo workspace restructuring for connector crates, two toolkit connectors (gpui-component and iced), and CI/publishing infrastructure. The only new runtime dependency is `objc2-app-kit 0.3.2` behind a `macos` feature flag. Connector crates add `gpui-component 0.5.1` / `gpui 0.2.2` and `iced 0.14.0` as their own dependencies, not polluting the core crate.

The recommended approach is strictly sequenced: do all breaking API changes first while the crate is still single-crate (flattening colors is the largest refactor by diff size, touching all 17 presets, 3 readers, and 140+ tests); then add new data models (widget metrics) and platform support (macOS reader, Windows/Linux enhancements) on the stabilized API; then restructure to a Cargo workspace and build connector crates last, when the core API is frozen for v0.2. This ordering is driven by a key architectural insight: workspace restructuring before API stabilization forces cascading connector fixes on every core change. The build order was independently confirmed by ARCHITECTURE.md and FEATURES.md dependency analysis.

The primary risks are: (1) macOS NSColor semantic colors returning wrong-mode values without explicit `NSAppearance` context setting -- a silent correctness bug, not a crash; (2) objc2 autorelease pool leaks in non-AppKit contexts causing unbounded memory growth on repeated calls; (3) field name collisions when flattening ThemeColors (`primary.background` and `core.background` both want the name `background`) -- solved by renaming to `primary` / `primary_foreground`; (4) Windows `GetSystemMetrics` returning DPI-virtualized values without `GetSystemMetricsForDpi`; and (5) iced's `Base` trait requiring per-widget `Catalog` implementations, not just a `Palette`. All five have concrete prevention strategies documented in PITFALLS.md.

## Key Findings

### Recommended Stack

v0.2 adds exactly one new runtime dependency to the core crate: `objc2-app-kit 0.3.2` behind the `macos` feature flag. All other core dependencies (serde, toml, configparser, ashpd, windows) are unchanged from v0.1. Connector crates bring their own toolkit dependencies. CI adds `cargo-semver-checks 0.46.0` and `cargo-hack` as dev tools.

**Core technologies (new in v0.2):**
- **objc2-app-kit 0.3.2:** macOS AppKit bindings for NSColor, NSFont, NSAppearance -- only actively maintained safe ObjC bindings; needs `NSColor`, `NSFont`, `NSAppearance`, `NSColorSpace` features
- **gpui-component 0.5.1 + gpui 0.2.2:** Target toolkit for the gpui connector crate -- 108-field ThemeColor with remarkably clean 1:1 mapping to native-theme's 36 colors
- **iced 0.14.0:** Target toolkit for the iced connector crate -- 6-color Palette + per-widget Catalog system; on crates.io, publishable immediately
- **cargo-semver-checks 0.46.0:** CI tool for SemVer violation detection -- catches removed public items, changed signatures, type changes

**Critical version requirements:**
- objc2-app-kit 0.3.2 requires objc2 >=0.6.2, <0.8.0 (current latest 0.6.4 is within range)
- iced 0.14 specifically -- the Base trait and Catalog system changed significantly across 0.12/0.13/0.14
- gpui-component pinned to >=0.5.1, <0.6 due to pre-1.0 instability risk
- Windows crate should tighten to 0.62 (no reason to support 0.59-0.61)

### Expected Features

**Must have (table stakes -- the v0.2 release is incomplete without these):**
- Flat ThemeColors (36 direct `Option<Rgba>` fields) -- eliminates the 7-sub-struct taxonomy that has no precedent in CSS/shadcn/Tailwind
- `NativeTheme::preset()`, `NativeTheme::from_toml()`, etc. -- idiomatic Rust associated function convention
- macOS reader -- completes 4-platform coverage (KDE, GNOME portal, Windows, macOS)
- ThemeGeometry additions: `radius_lg` and `shadow` -- non-breaking, required by connectors
- Workspace restructuring -- prerequisite for connector crates
- Publishing prep -- crates.io metadata, license files, CHANGELOG.md

**Should have (differentiators -- high value, moderate effort):**
- Widget metrics data model (12 per-widget sub-structs) -- enables pixel-perfect native UIs; no competing crate provides this
- iced connector (`native-theme-iced`) -- can publish to crates.io immediately; iced has 25k GitHub stars
- CI pipeline -- cross-platform matrix with feature flag testing and semver checks
- Windows reader enhancements -- accent shades, system font, DPI-aware metrics
- Linux reader enhancements -- portal accent overlay on kdeglobals, GNOME font reading, D-Bus backend detection

**Defer (v0.3+):**
- gpui-component connector publishing (build it, but accept git-dep-only until gpui-component stabilizes)
- egui connector -- least structured theming API, simpler but less value
- iOS/Android runtime readers -- small Rust audience on mobile
- Built-in change notification -- opinionated async runtime choice, scope creep
- Color manipulation utilities -- out of scope for a data crate; use the `palette` crate

### Architecture Approach

v0.2 follows a strict layered integration into the existing codebase. The `impl_merge!` macro is the single most important integration point -- it generates `merge()` and `is_empty()` for every model struct and must be updated atomically with any struct change. ThemeColors flattening eliminates 6 sub-struct types and their macro invocations (net LOC reduction), while widget metrics adds 12 new sub-structs following the exact same pattern. The macOS reader follows the proven reader pattern: feature-gated module producing a `NativeTheme` from platform APIs, with a testable `build_theme()` core separated from ObjC API calls. Connector crates are thin mapping layers with zero intermediate types -- they map directly from `NativeTheme` fields to toolkit types.

**Major components (changed or new):**
1. **`model/colors.rs`** -- REWRITE: flatten from 7 nested sub-structs to 36 direct `Option<Rgba>` fields
2. **`model/metrics.rs`** -- NEW: `WidgetMetrics` + 12 per-widget sub-structs (ButtonMetrics, ScrollbarMetrics, etc.)
3. **`src/macos.rs`** -- NEW: macOS reader via objc2-app-kit (NSColor semantic colors, NSFont, NSAppearance)
4. **`native-theme-gpui/`** -- NEW: connector crate mapping 36 colors to 108 gpui-component ThemeColor fields
5. **`native-theme-iced/`** -- NEW: connector crate mapping to iced Palette + per-widget Catalog implementations

### Critical Pitfalls

1. **NSColor appearance context (macOS reader)** -- Semantic colors resolve to wrong light/dark variant without explicit `NSAppearance::setCurrentAppearance`. Set appearance once at top of `from_macos()`, read all colors, restore. Silent correctness bug, not a crash.
2. **Autorelease pool leak (macOS reader)** -- Non-AppKit Rust binaries have no run loop to drain autoreleased ObjC objects. Wrap entire `from_macos()` in `objc2::rc::autoreleasepool`. Without this, memory grows unboundedly on repeated calls.
3. **Field name collision during ThemeColors flattening** -- `primary.background` and `core.background` both want `background`. Rename: `primary.background` becomes `primary`, `primary.foreground` becomes `primary_foreground`. Apply the same pattern to `secondary`, `danger`, `warning`, `success`, `info`, `selection`.
4. **Windows DPI virtualization** -- `GetSystemMetrics()` returns 96-DPI values for DPI-unaware processes. Use `GetSystemMetricsForDpi()` with `GetDpiForSystem()` (requires `Win32_UI_HiDpi` feature on windows crate).
5. **iced Base trait requirement** -- Simply providing a `Palette` does not style widgets. Must implement `Base` trait (5 methods) and per-widget `Catalog` entries. Study COSMIC Desktop's `cosmic-theme` as proven reference.

## Implications for Roadmap

Based on combined research across all four files, the critical path is: **Flat ThemeColors -> API methods -> Widget Metrics -> macOS reader -> Workspace restructuring -> Connectors -> CI -> Publishing**. The ordering is driven by three principles: (1) breaking changes before non-breaking additions, (2) data model before consumers, (3) single-crate development before workspace restructuring.

### Phase 1: API Breaking Changes
**Rationale:** All breaking changes must land first while the crate is single-crate and has no downstream workspace consumers. The ThemeColors flattening is the largest refactor by diff size (touches all 17 presets, 3 readers, all tests). Moving free functions to `impl NativeTheme` is smaller and cleaner on the already-flat API. ThemeGeometry additions are non-breaking but logically grouped here.
**Delivers:** Flat `ThemeColors` with 36 direct fields, `NativeTheme::preset()` / `NativeTheme::from_toml()` / `theme.to_toml()` API, `radius_lg` and `shadow` on `ThemeGeometry`
**Addresses:** Features 1-4 from FEATURES.md (flat colors, API methods, geometry additions)
**Avoids:** Pitfall 3 (field name collision -- use `primary`/`primary_foreground` naming), Pitfall about `serde(flatten)` (do a clean break, no backward compat)

### Phase 2: Platform Readers
**Rationale:** macOS reader completes 4-platform coverage, which is the primary v0.2 value proposition. Windows and Linux enhancements fill visible gaps. All three are independent of each other and can be developed in parallel once the data model is stable.
**Delivers:** `from_macos()` reader (~20 NSColor semantic mappings + fonts + appearance), Windows accent shades and system font, Linux portal overlay and GNOME font reading
**Uses:** objc2-app-kit 0.3.2 (new), existing windows and ashpd crates (enhanced)
**Avoids:** Pitfall 1 (NSAppearance context), Pitfall 2 (autorelease pool), Pitfall 6 (Windows DPI)

### Phase 3: Widget Metrics
**Rationale:** Widget metrics must exist before connectors can consume them for per-widget styling. The data model follows the exact same pattern as existing model structs (Option fields, non_exhaustive, impl_merge!). Platform population is independent work per reader.
**Delivers:** `WidgetMetrics` with 12 sub-structs, KDE breezemetrics.h constants, Windows GetSystemMetricsForDpi values, macOS HIG hardcoded values, GNOME hardcoded values
**Addresses:** Feature 5 (widget metrics system) from FEATURES.md
**Avoids:** Pitfall 5 (false precision -- return `None` for values not exposed at runtime, document coverage matrix)

### Phase 4: CI Pipeline
**Rationale:** CI should run after all platform features exist (to test macOS runner) but before workspace restructuring (to catch regressions). Feature flag matrix ensures each reader compiles independently.
**Delivers:** GitHub Actions workflow with Linux/Windows/macOS matrix, feature flag testing via `cargo hack`, `cargo-semver-checks` integration, `cargo clippy` + `cargo fmt`
**Addresses:** Feature 9 (CI pipeline) from FEATURES.md
**Avoids:** Pitfall about cargo-semver-checks first-run baseline (handle no-previous-version case)

### Phase 5: Workspace Restructuring and Connectors
**Rationale:** Workspace restructuring must happen after all core API changes are complete. Connectors are thin mapping layers that depend on the stable core API. The iced connector is lower risk (on crates.io, well-documented API, 6-color Palette) than the gpui connector (108 color fields, upstream instability).
**Delivers:** Cargo workspace with `native-theme/`, `native-theme-gpui/`, `native-theme-iced/` members. Working connector crates with examples.
**Uses:** iced 0.14.0 (crates.io), gpui-component 0.5.1 (crates.io)
**Avoids:** Pitfall 4 (include_str paths -- run `cargo publish --dry-run` immediately), Pitfall 7 (gpui instability -- pin exact version, isolate mapping), Pitfall 8 (iced Base trait -- implement full Catalog, study COSMIC Desktop)

### Phase 6: Publishing Prep
**Rationale:** Final step. All API changes complete, CI green, connectors functional. Focus on crates.io metadata, documentation, license files, CHANGELOG.md.
**Delivers:** Published `native-theme` on crates.io, published `native-theme-iced` on crates.io, `native-theme-gpui` available via git dependency
**Addresses:** Feature 10 (publishing prep) from FEATURES.md
**Avoids:** Pitfall about missing presets in package (verify with `cargo package --list`)

### Phase Ordering Rationale

- **Phase 1 before Phase 2:** Flattening colors is a prerequisite for the macOS reader (it writes to flat fields). All readers must be updated to flat field access during flattening, so adding a new reader before flattening means doing the work twice.
- **Phase 2 before Phase 3:** Widget metrics in platform readers require the reader infrastructure to exist first. The macOS reader is independent data model work, while widget metrics extend all readers.
- **Phase 3 before Phase 5:** Connectors need widget metrics to provide per-widget styling. Building connectors without metrics means retrofitting them later.
- **Phase 4 (CI) before Phase 5:** CI catches regressions during workspace restructuring, which is a high-risk operation for build breakage (include_str paths, publish dry-run).
- **Phase 5 last before publishing:** Workspace restructuring is a repo organization change, not functionality. Doing it last means all code changes happen in the simpler single-crate layout.

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 2 (macOS reader):** NSAppearance resolution API differences between macOS 11+ and earlier, exact NSColor-to-ThemeColors mapping editorial decisions, NSFont display name vs private name (.AppleSystemUIFont), P3-to-sRGB conversion edge cases (nil return from colorUsingColorSpace on pattern colors)
- **Phase 3 (Widget Metrics):** Coverage matrix design (which fields to expose per platform), KDE Plasma 5 vs 6 constant differences, libadwaita SCSS source extraction for hardcoded values
- **Phase 5 (Connectors):** gpui-component shade generation (11-shade scale from single accent color), iced 0.14 Base trait + Catalog implementation patterns, COSMIC Desktop cosmic-theme code study for iced reference

Phases with standard patterns (skip deeper research):
- **Phase 1 (API refactors):** Mechanical refactor -- search-and-replace field paths, update macro invocations, rewrite preset TOML files
- **Phase 4 (CI):** Well-documented GitHub Actions patterns, cargo-semver-checks has an official action
- **Phase 6 (Publishing):** Standard crates.io metadata, `cargo publish --dry-run` catches everything

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | All crate versions verified on crates.io/lib.rs with publication dates. objc2-app-kit 0.3.2 (Oct 2025), gpui-component 0.5.1 (Feb 2026), iced 0.14.0 (Dec 2025), cargo-semver-checks 0.46.0. Dependency chains confirmed. |
| Features | HIGH | Feature landscape mapped against platform API docs (Apple NSColor, KDE breezemetrics.h, libadwaita CSS variables, Windows GetSystemMetrics), toolkit source code (gpui-component ThemeColor 108 fields, iced Palette 6 fields), and existing codebase analysis. Clear dependency graph with critical path identified. |
| Architecture | HIGH | Based on full source code analysis of existing ~7,000 LOC codebase. Build order independently confirmed by architecture patterns and feature dependencies. Merge macro integration point and workspace restructuring sequence validated. |
| Pitfalls | HIGH | 8 critical pitfalls identified with verified sources (Apple docs, serde issue tracker, Microsoft DPI docs, objc2 autorelease semantics). All have concrete prevention strategies, phase assignments, and recovery costs assessed. |

**Overall confidence:** HIGH

### Gaps to Address

- **gpui-component shade generation:** native-theme provides 36 single colors; gpui-component uses 11-shade scales (50-950) per color family. The algorithm for generating a visually correct gradient from one seed color needs prototyping during Phase 5 connector work. Consider using Oklch color space for perceptual uniformity.
- **iced Catalog coverage depth:** The number of per-widget Catalog trait implementations needed for a "complete" iced connector is unclear. Start with Button, Container, TextInput, Scrollable (the 4 most common widgets) and expand based on demand. COSMIC Desktop's code is the best reference.
- **Widget metrics cross-platform coverage honesty:** KDE provides ~80 metrics, Windows ~20, macOS ~10 (hardcoded), GNOME ~4 (CSS variables). The `WidgetMetrics` struct will be mostly `None` on non-KDE platforms. Document this honestly and consider a `coverage()` method.
- **gpui-component crates.io status:** STACK.md and PITFALLS.md disagree on whether gpui-component is on crates.io. STACK.md says it IS on crates.io (v0.5.1, confirmed). PITFALLS.md initially assumed git-only but corrected itself. Resolution: it IS on crates.io and the connector CAN be published.
- **macOS testing infrastructure:** The macOS reader can only be integration-tested on macOS CI runners. The testable-core pattern (build_theme with raw values) handles unit tests, but verifying actual NSColor reads requires a macOS environment.

## Sources

### Primary (HIGH confidence)
- [crates.io/objc2-app-kit 0.3.2](https://crates.io/crates/objc2-app-kit) -- published Oct 4 2025, dependency chain verified
- [docs.rs/gpui-component ThemeColor](https://docs.rs/gpui-component/latest/gpui_component/theme/struct.ThemeColor.html) -- 108 color fields enumerated
- [docs.rs/iced 0.14 Palette](https://docs.rs/iced/latest/iced/theme/palette/struct.Palette.html) -- 6 fields verified
- [docs.rs/iced Base trait](https://docs.iced.rs/iced/widget/theme/trait.Base.html) -- 5 required methods for custom themes
- [Apple NSColor documentation](https://developer.apple.com/documentation/appkit/nscolor) -- semantic color properties, dynamic resolution
- [KDE breezemetrics.h](https://github.com/KDE/breeze/blob/master/kstyle/breezemetrics.h) -- ~80 widget metric constants
- [libadwaita CSS variables](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1-latest/css-variables.html) -- only window-radius and opacity variables exposed
- [Windows GetSystemMetricsForDpi](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetricsfordpi) -- DPI-aware metrics API
- [cargo-semver-checks-action v2](https://github.com/obi1kenobi/cargo-semver-checks-action) -- GitHub Actions integration
- [serde flatten + TOML issues](https://github.com/serde-rs/serde/issues/1379) -- confirms clean break over serde(flatten) for TOML

### Secondary (MEDIUM confidence)
- [gpui-component theme documentation](https://longbridge.github.io/gpui-component/docs/theme) -- ActiveTheme trait, ThemeRegistry, theme loading patterns
- [iced discourse on styling](https://discourse.iced.rs/t/changing-the-default-styling-of-widget/775) -- Catalog/closure pattern confirmation
- [NSAppearance resolution pattern](https://christiantietze.de/posts/2021/10/nscolor-performAsCurrentDrawingAppearance-resolve-current-appearance/) -- how to resolve dynamic colors outside drawing context
- [Existing codebase analysis](file:///home/tibi/Rust/native-theme/src/) -- full source read of 14 .rs files, 17 TOML presets, 140+ tests

### Tertiary (LOW confidence)
- gpui-component shade generation algorithm -- no documented standard for generating 11-shade scales from single seed colors; needs prototyping
- iced Catalog implementation depth -- unclear how many widget Catalog impls are needed for "complete" coverage; depends on usage patterns

---
*Research completed: 2026-03-08*
*Ready for roadmap: yes*
