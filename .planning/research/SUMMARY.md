# Project Research Summary

**Project:** native-theme
**Domain:** Cross-platform Rust library crate for OS theme data (colors, fonts, geometry, spacing)
**Researched:** 2026-03-07
**Confidence:** HIGH

## Executive Summary

native-theme is a toolkit-agnostic Rust crate that provides a unified data model for OS theme information -- 36 semantic color roles, fonts, geometry, and spacing -- serializable to/from TOML and populatable from live OS APIs via feature-gated platform readers. No existing crate fills this niche: `dark-light` only detects dark/light mode, `system-theme` is coupled to iced with only 6 colors, `cosmic-theme` is locked to the COSMIC desktop and RON format, and `catppuccin` is static data with no OS reading capability. The approach is well-validated: the "data-model-core + optional-backends" architecture is the established Rust pattern for cross-platform crates (used by `notify`, `arboard`, `keyring`), and the entire technology stack (serde, toml 1.0, objc2-app-kit, ashpd, windows crate) has been verified against crates.io with HIGH confidence.

The recommended build strategy is core-first: establish the data model with all-`Option<T>` fields, custom hex serde for `Rgba`, declarative-macro-generated structs (guaranteeing merge synchronization), and TOML round-trip correctness before building any platform reader. Three bundled presets (default, kde-breeze, adwaita) provide immediate usability without platform dependencies. Platform readers are then added incrementally -- Linux first (KDE sync reader, then portal async reader), followed by Windows and macOS, with iOS and Android deferred to post-1.0. Each reader is an independent, feature-gated module that populates the same `NativeTheme` type.

The primary risks are: (1) macOS NSColor color space crashes -- extracting RGB components from P3/catalog colors without sRGB conversion causes unrecoverable Objective-C exceptions; (2) ashpd's default tokio dependency leaking into sync consumers via Cargo feature unification -- must be designed correctly before first publish since changing feature structure is a breaking change; (3) merge desynchronization -- manually maintaining a 36-field merge function invites silent bugs when fields are added, solved by using a declarative macro from day one; and (4) missing `#[serde(default)]` on nested structs breaking the core promise of partial TOML overrides. All four are preventable with the correct patterns applied in the right phases.

## Key Findings

### Recommended Stack

The core stack is minimal and stable: serde 1.0 (universal serialization) + toml 1.0 (just stabilized March 2026, TOML spec 1.1.0). Rust Edition 2024 with MSRV 1.85.0 is appropriate for a greenfield crate. Platform dependencies are all feature-gated and verified against crates.io. A critical correction from the project's IMPLEMENTATION.md: ashpd 0.13.x supports `async-io` (not `async-std`) as its alternative to tokio.

**Core technologies:**
- **serde 1.0 + toml 1.0:** Serialization and TOML parsing -- uncontested standards, verified stable
- **configparser 3.1 + dirs 6.0:** KDE kdeglobals INI parsing -- zero-dep, case-sensitive mode required
- **ashpd 0.13:** Freedesktop portal D-Bus access -- async-only, typed accent/scheme/contrast enums
- **windows 0.62:** Official Microsoft Windows API bindings -- UISettings for accent colors + system metrics
- **objc2-app-kit 0.3:** macOS AppKit bindings -- NSColor, NSFont, NSAppearance access via objc2 0.6 ecosystem
- **objc2-ui-kit 0.3:** iOS UIKit bindings -- same objc2 ecosystem, deferred to late phase
- **jni 0.22 + ndk 0.9:** Android JNI bridge -- weakest part of the stack, deferred to post-1.0

**Critical version notes:**
- ashpd must disable default features to avoid forcing tokio on all consumers
- windows crate needs exactly `UI_ViewManagement` + `Win32_UI_WindowsAndMessaging` features
- objc2-app-kit needs `NSColor`, `NSFont`, `NSAppearance`, `NSColorSpace` features
- Android tooling is the least mature -- MEDIUM confidence, expect 3-5x more code than other platforms

### Expected Features

**Must have (table stakes):**
- Semantic color data model (36 roles, all `Option<Rgba>`) covering accent, background, foreground, error, warning, success, and derivatives
- Light/dark variant support (`NativeTheme { light: Option<ThemeVariant>, dark: Option<ThemeVariant> }`)
- TOML serialization with custom hex serde for `Rgba` (`#rrggbb` / `#rrggbbaa`)
- Bundled presets (default, kde-breeze, adwaita) with both light and dark variants
- Theme merging/layering via field-by-field `Option` merge (macro-generated)
- Font, geometry, and spacing data models
- `#[non_exhaustive]` on all public structs for forward compatibility
- Typed error enum (Unsupported, Unavailable, Format, Platform)
- `preset()`, `list_presets()`, `from_file()`, `to_toml()` API

**Should have (differentiators):**
- Runtime OS theme reading via feature-gated platform readers (KDE, portal, Windows, macOS)
- Cross-platform `from_system()` dispatch with auto-detection
- Toolkit-agnostic design (zero GUI dependencies) -- the primary differentiator vs all competitors
- Community-contributable TOML presets (lower barrier than RON or Rust code)
- Partial TOML overrides (3-line accent override merged onto full base theme)

**Defer (v1.0+):**
- iOS runtime reader -- small Rust iOS audience, requires device testing
- Android runtime reader -- immature JNI tooling, 3-5x more code than other platforms
- Widget-level metrics -- no toolkit consumes these today
- W3C design token format -- different schema, low demand from Rust GUI developers
- Built-in toolkit adapters -- keep them as separate community crates to avoid coupling

### Architecture Approach

The crate follows a three-layer architecture: (1) Core Layer (always compiled) containing the data model, TOML serde, error types, and merge logic with only serde+toml as dependencies; (2) Preset Layer (always compiled) with embedded TOML files via `include_str!()` and the preset loading API; (3) Platform Layer (feature-gated) where each reader is an independent module populating `NativeTheme` from OS APIs. No trait abstraction is needed for readers -- `from_system()` dispatches via `#[cfg(target_os)]` at compile time. The sync/async boundary is handled by keeping `kde` (sync) and `portal` (async) as separate features with `from_system()` always being sync.

**Major components:**
1. **`model/`** -- Canonical data types (NativeTheme, ThemeVariant, ThemeColors with 36 fields, ThemeFonts, ThemeGeometry, ThemeSpacing, Rgba) with serde derives, `Default` impls, and macro-generated `merge()` methods
2. **`presets/`** -- Embedded TOML files, `preset(name)` loader, `from_file()`, `to_toml()`, `list_presets()` API
3. **`error.rs`** -- Unified error enum with `From` impls for toml and io errors
4. **`platform/`** -- Feature-gated readers: `kde.rs` (sync), `portal.rs` (async), `windows.rs` (sync), `macos.rs` (sync), plus `mod.rs` with `from_system()` dispatch

### Critical Pitfalls

1. **configparser case sensitivity (Phase 3)** -- `Ini::new()` lowercases all keys, silently dropping all KDE color data. Always use `Ini::new_cs()`. Add integration test asserting `accent.is_some()` with real kdeglobals snippet.
2. **macOS NSColor color space crash (Phase 5)** -- Extracting RGB from P3/catalog colors causes unrecoverable ObjC exception. Always convert via `colorUsingColorSpace(&NSColorSpace::sRGBColorSpace())` before component access.
3. **ashpd tokio dependency leak (Phase 3)** -- Must disable ashpd default features in Cargo.toml to prevent tokio from infecting sync consumers. Design feature flags correctly before first publish -- this is unfixable without a breaking change.
4. **merge() desynchronization (Phase 1)** -- Manual 36-field merge function will silently miss new fields. Use declarative macro to generate both struct fields and merge from a single source of truth.
5. **Missing `#[serde(default)]` on nested structs (Phase 1)** -- Breaks partial TOML deserialization. Every nested struct field needs `#[serde(default)]`. Test with minimal TOML files (single field only).
6. **KDE font string variable field count (Phase 3)** -- Qt 4 uses 10 fields, Qt 5/6 uses 16 fields. Parse defensively, return `None` for unparseable fields, test with all Qt version formats.

## Implications for Roadmap

Based on research, suggested phase structure:

### Phase 1: Data Model Foundation
**Rationale:** Everything depends on the data model. The model must be stable before presets are authored or readers are built. The macro-generated struct pattern and `#[serde(default)]` discipline must be correct from the first commit -- retrofitting these is painful.
**Delivers:** `Rgba` type with custom hex serde, `ThemeColors` (36 fields), `ThemeFonts`, `ThemeGeometry`, `ThemeSpacing`, `ThemeVariant`, `NativeTheme`, `merge()` methods, `Error` enum. All types `Send + Sync`, `#[non_exhaustive]`, `Default`, `Clone`, `Debug`.
**Addresses:** Semantic color model, light/dark variants, hex alpha support, merge/layering, typed errors, forward compatibility
**Avoids:** Pitfall 4 (merge desync via macro), Pitfall 5 (serde defaults)

### Phase 2: Preset System and TOML I/O
**Rationale:** Presets make the crate immediately useful without any platform features. They validate the data model schema against real theme data. Presets must exist before `from_system()` can use them as fallbacks.
**Delivers:** `preset()`, `list_presets()`, `from_file()`, `to_toml()` API. Three bundled presets: default, kde-breeze, adwaita (light + dark each). TOML round-trip tests, minimal TOML tests.
**Addresses:** Bundled presets, TOML serialization, file loading, partial override workflow
**Avoids:** Eager preset parsing (use const array for listing, lazy parse on access)

### Phase 3: Linux Runtime Readers
**Rationale:** Linux has the richest freely-accessible theme data (KDE kdeglobals: 60+ colors) and the most complex feature flag design (sync kde vs async portal). Getting the feature flag structure right here -- especially the ashpd tokio isolation -- is critical before publishing any version.
**Delivers:** `from_kde()` sync reader, `from_gnome()` async portal reader, `from_system()` Linux dispatch with DE auto-detection, `watch` feature for kdeglobals file monitoring.
**Uses:** configparser 3.1, dirs 6.0, ashpd 0.13 (default-features = false), notify 8.2
**Avoids:** Pitfall 1 (case sensitivity), Pitfall 3 (tokio leak), Pitfall 6 (font parsing), portal accent out-of-range

### Phase 4: Additional Presets
**Rationale:** With the Linux reader validating the data model against live KDE data, this is the right time to author additional presets: windows-11, macos-sonoma, material, ios. These presets are authored from design specs and serve as fallbacks on platforms without readers yet.
**Delivers:** 4+ additional TOML presets covering all target platforms' design languages.
**Addresses:** Immediate usability on Windows/macOS before those readers are built

### Phase 5: Windows and macOS Readers
**Rationale:** Desktop platform readers are the next highest-value features. Windows (sync, UISettings) is simpler than macOS (color space conversion, thread safety). Building them after Linux readers means the reader pattern is already proven.
**Delivers:** `from_windows()` sync reader (accent + 8 colors + system metrics), `from_macos()` sync reader (~20 semantic NSColors + fonts + appearance detection), cross-platform `from_system()` dispatch.
**Uses:** windows 0.62, objc2-app-kit 0.3
**Avoids:** Pitfall 2 (NSColor crash -- sRGB conversion helper), Windows API presence checks for graceful degradation

### Phase 6: Polish, Documentation, and v1.0
**Rationale:** Stabilize the API, document the adapter pattern for each major toolkit (egui, iced, gpui, slint), document change notification sources, add comprehensive examples. Prepare for crates.io publication.
**Delivers:** Complete documentation, toolkit adapter examples, cookbook, API stability guarantees, crates.io publication.

### Phase 7: Mobile Readers (Post-1.0)
**Rationale:** iOS and Android have the smallest Rust GUI audiences and the most testing friction (simulators/devices). Android JNI is the weakest part of the stack (MEDIUM confidence, 3-5x more code). Defer until desktop platforms are proven.
**Delivers:** `from_ios()` reader (UIColor, UIFont, UITraitCollection), `from_android()` reader (Material You via JNI).
**Uses:** objc2-ui-kit 0.3, jni 0.22, ndk 0.9

### Phase Ordering Rationale

- **Model before presets, presets before readers:** Strict dependency chain. Each layer depends on the one below. Building out of order creates rework when the model changes.
- **Linux before Windows/macOS:** Linux has two readers (kde sync + portal async) exercising both sync and async patterns, validating the feature flag design that affects the entire crate. It also has the richest data source (kdeglobals).
- **Additional presets between Linux and Windows/macOS readers:** Presets for Windows/macOS design languages provide immediate value on those platforms while readers are still being built.
- **Mobile deferred to post-1.0:** Lowest user value, highest implementation cost (especially Android), hardest to test. The preset system provides static theme data for mobile in the meantime.
- **Pitfall prevention is front-loaded:** The three Phase 1 pitfalls (macro merge, serde defaults, serde round-trip) and the Phase 3 feature flag pitfall (tokio leak) are all design decisions that become exponentially more expensive to fix after publication.

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 3 (Linux readers):** ashpd async API surface, portal accent-color edge cases, KDE kdeglobals format documentation, DE detection heuristics on Wayland
- **Phase 5 (macOS reader):** NSColor semantic color catalog (which ~20 colors to read), P3-to-sRGB conversion correctness, NSAppearance resolution API (macOS 11+ vs earlier), thread safety patterns with objc2
- **Phase 5 (Windows reader):** UISettings API availability on different Windows versions, UIColorType enum coverage, GetSystemMetrics usage patterns

Phases with standard patterns (skip deeper research):
- **Phase 1 (data model):** Well-documented serde patterns, Option-all-fields is standard for config-like structs, declarative macro pattern is straightforward
- **Phase 2 (presets):** `include_str!()` embedding is trivial, TOML authoring is manual data entry from design specs
- **Phase 4 (additional presets):** Pure data work, no code complexity

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | All crate versions verified against crates.io on 2026-03-07. Dependency compatibility confirmed. ashpd async-io correction identified and documented. |
| Features | HIGH | Feature landscape mapped against 5 competing crates, 6 target toolkits, platform API docs, and W3C design token spec. Clear differentiation identified. |
| Architecture | HIGH | Pattern validated by 3 production cross-platform Rust crates (notify, arboard, keyring). Anti-patterns identified from system-theme's design mistakes. |
| Pitfalls | HIGH | 6 critical pitfalls identified with verified sources. All have concrete prevention strategies and phase assignments. Recovery costs assessed. |

**Overall confidence:** HIGH

### Gaps to Address

- **Android runtime reading maturity:** JNI-based Material You reading from Rust has no established pattern. Expect significant prototyping effort in Phase 7. The jni/ndk crate versions are verified but the integration pattern is not.
- **macOS NSColor catalog completeness:** The exact set of ~20 semantic NSColor properties to read needs validation during Phase 5 planning. The objc2-app-kit bindings are verified, but the best subset of NSColor properties for a theme crate is an editorial decision.
- **GNOME Adwaita preset accuracy:** Adwaita color values will be hardcoded from CSS variables in the GTK source. These values change with GNOME releases. Document staleness risk and plan periodic updates.
- **Cross-platform CI coverage:** Testing feature flag combinations requires CI runners on Linux, Windows, and macOS. `cargo hack --feature-powerset` handles compilation checks, but integration tests for platform readers require actual OS environments.
- **ashpd runtime feature pass-through:** The exact Cargo.toml pattern for letting consumers choose between tokio and async-io for the portal feature needs validation. Consider `portal-tokio` and `portal-async-io` convenience features.

## Sources

### Primary (HIGH confidence)
- [crates.io](https://crates.io) -- All dependency versions verified (serde 1.0.228, toml 1.0.6, ashpd 0.13.4, configparser 3.1.0, dirs 6.0.0, windows 0.62.2, objc2-app-kit 0.3.2, objc2-ui-kit 0.3.2, jni 0.22.3, ndk 0.9.0, notify 8.2.0)
- [docs.rs/ashpd](https://docs.rs/ashpd/latest/ashpd/) -- Feature flags verified: tokio (default), async-io alternative, no async-std
- [microsoft.github.io/windows-docs-rs](https://microsoft.github.io/windows-docs-rs) -- UISettings, UIColorType, GetSystemMetrics API surface verified
- [Apple NSColor docs](https://developer.apple.com/documentation/appkit/nscolor) -- Color space crash behavior documented
- [Cargo Book - Features](https://doc.rust-lang.org/cargo/reference/features.html) -- Feature unification semantics
- [Rust 1.85.0 announcement](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/) -- Edition 2024 stabilization confirmed

### Secondary (MEDIUM confidence)
- [notify-rs/notify](https://github.com/notify-rs/notify), [arboard](https://github.com/1Password/arboard), [keyring-rs](https://github.com/open-source-cooperative/keyring-rs) -- Cross-platform Rust crate architecture patterns
- [cosmic-theme](https://github.com/pop-os/libcosmic/tree/master/cosmic-theme) -- Option-field data model validation for theme crates
- [W3C Design Tokens spec 2025.10](https://www.w3.org/community/design-tokens/) -- Design token format landscape
- [configparser case sensitivity issue](https://github.com/QEDK/configparser-rs/issues/6) -- Confirms default lowercase behavior
- [XDG Desktop Portal Settings spec](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Settings.html) -- Accent color semantics

### Tertiary (LOW confidence)
- Android Material You from Rust via JNI -- No established pattern exists; extrapolated from jni crate examples and Android API docs. Needs prototyping validation.

---
*Research completed: 2026-03-07*
*Ready for roadmap: yes*
