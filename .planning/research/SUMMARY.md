# Project Research Summary

**Project:** native-theme v0.5.x — per-widget architecture and resolution pipeline
**Domain:** Rust theme data crate with per-widget struct hierarchy, inheritance-based resolve(), and OS-first platform integration
**Researched:** 2026-03-27
**Confidence:** HIGH

## Executive Summary

native-theme's next evolution is a structural migration from flat theme collections (ThemeColors, ThemeFonts, ThemeGeometry) to a per-widget struct hierarchy with a formal inheritance resolution pipeline. Every major platform theme system — GTK CSS, Qt QPalette/QStyle, WinUI3 XAML resources, and macOS NSColor/NSFont — exposes per-widget properties with inheritance. The current flat model forces consumers to manually map generic color roles to their specific widgets, a known pain point the redesign eliminates. The research confirms the design documented in `docs/todo_v0.5.1_*.md` is architecturally correct and directly parallels production theme systems.

The recommended approach is a phased migration across four phases: data model restructure (atomic ThemeVariant change + preset rewrites), resolve() and ResolvedTheme implementation, OS reader extensions, and finally connector migration. The key insight from architecture research is that the existing `impl_merge!` macro already supports nested structs (proven by WidgetMetrics), so the migration extends a proven pattern rather than introducing a paradigm shift. The critical constraint is that `ThemeVariant` + preset TOML rewrites must be a single atomic commit — serde keys cannot coexist between the old and new shapes.

The primary risk is silent data loss from the TOML field path breaking change: `[light.colors]` becomes `[light.defaults]`, and serde's `#[serde(default)]` swallows unrecognized sections without error. Secondary risks include maintaining 28 parallel struct pairs (Option and Resolved versions) and ensuring resolve() processes inheritance chains in the correct dependency order. All three risks have documented mitigations: serde aliases for backward compat, a `define_widget_pair!` declarative macro to eliminate struct duplication, and explicit phased structure in the resolve() function with one test per inheritance rule.

## Key Findings

### Recommended Stack

The stack requires zero new crate dependencies. All needed capabilities exist in the current dependency set. The only changes are two new feature flags on the `windows` crate (`Win32_Graphics_Dwm` for DwmGetColorizationColor, `Win32_UI_Accessibility` for HIGHCONTRASTW), and a new internal `define_widget_pair!` `macro_rules!` macro that generates Option and Resolved struct pairs from a single definition. Every platform API needed for extended OS readers — NSFont.TextStyle on macOS, LOGFONT fields on Windows, KDE kdeglobals font keys and index.theme parsing, GNOME gsettings extensions and ashpd portal reduced_motion — is available in the existing pinned dependency versions.

**Core technologies:**
- `serde_with::skip_serializing_none` 3.17.0 — sparse TOML serialization; already proven on 14 structs, scales identically to 24+ widget structs with no changes
- `macro_rules! define_widget_pair!` (new internal macro) — generates Option struct + Resolved struct pairs with `impl_merge!`; zero new dependencies, replaces 28x manual struct duplication
- `objc2-app-kit` 0.3.2 — macOS reader extensions; `NSFontTextStyle*`, `preferredFontForTextStyle_options`, `NSFontTraitsAttribute`/`NSFontWeightTrait` all available under already-enabled feature flags
- `windows` 0.62.2 — Windows reader extensions; `GetSysColor` + `DwmGetColorizationColor` + `HIGHCONTRASTW` + `UISettings::TextScaleFactor()` available; only 2 feature flags to add
- `configparser` 3.1.0 — KDE reader extensions; `[WM]`, `[Colors:Header]`, `[Icons]`, and icon `index.theme` all parse correctly with the existing INI parser
- `ashpd` 0.13.9 — GNOME reader extensions; `Settings::reduced_motion()` verified at line 313 of settings.rs; portal-unavailable gsettings keys (fonts, animations, overlay-scrolling) read via existing `gsettings` subprocess pattern

**Critical version requirements:**
- ashpd 0.13.7+ required for `reduced_motion()` — workspace resolves to 0.13.9, no action needed
- Qt5 vs Qt6 font weight scale difference (0-99 vs 1-1000) requires field-count-based detection in KDE reader

### Expected Features

**Must have (table stakes) — v0.5.1 Core Architecture:**
- Per-widget theme structs (25 widgets: window, button, input, checkbox, menu, tooltip, scrollbar, slider, progress_bar, tab, sidebar, toolbar, status_bar, list, popover, splitter, separator, switch, dialog, spinner, combo_box, segmented_control, card, expander, link) — consumers write `theme.button.background` directly instead of mapping generic color roles
- ThemeDefaults struct — shared base properties serving the same role as GTK initial values, Qt QPalette, and WinUI3 system resource keys
- FontSpec with family/size/weight — per-widget fonts to match macOS `+menuFontOfSize:`, KDE `menuFont`, Windows `NONCLIENTMETRICSW.lfMenuFont`
- resolve() — universal inheritance function filling None fields from ThemeDefaults per ~90 documented rules; the OS-first pipeline cannot function without it
- ResolvedTheme + validate() — concrete non-optional output; eliminates all `unwrap_or()` fabrication in connectors
- TextScale type ramp (caption, section_heading, dialog_title, display) — the only cross-platform Rust implementation of portable type ramp roles
- DialogButtonOrder enum — captures macOS/GNOME trailing-affirmative vs Windows/KDE leading-affirmative convention
- IconSizes struct — toolbar/small/large/dialog/panel per-context sizing; KDE reads from index.theme, Windows from SM_CXSMICON/SM_CXICON
- Accessibility signals (reduce_motion, high_contrast, text_scaling_factor, reduce_transparency) — runtime OS state in ThemeDefaults, not preset-authored

**Should have (competitive advantage) — v0.5.2 OS Readers + Pipeline:**
- Extended OS readers (macOS, Windows, KDE, GNOME) populating all new per-widget fields
- OS-first pipeline in from_system(): OS reader + platform TOML merge + resolve()
- Platform TOML slimming: remove OS-readable fields, keep only design constants
- App TOML overlay + second resolve() pass: consumer overrides accent, all 8 derived fields (primary_bg, slider.fill, etc.) update automatically

**Defer to v0.6+:**
- Connector migration to ResolvedTheme — breaking API change for native-theme-gpui and native-theme-iced; defer until ResolvedTheme is stable
- WindowTheme — title bar colors, inactive state (depends on connector consumer feedback)
- Additional TextScale entries beyond the 4-entry set

**Anti-features (explicitly out of scope):**
- Per-widget style class system — multiplicative struct explosion; use `button.primary_bg` instead
- Full CSS cascade with selectors — runtime evaluation engine, not a data structure library
- Runtime theme change watchers — couples to async runtimes; document re-calling from_system() instead
- Computed color algebra (darken/lighten/mix) — rendering concern, not theme data concern
- Per-widget interaction state colors (hover/pressed/focus) — multiplies fields 4-5x; use `disabled_opacity` + toolkit-level color operations

### Architecture Approach

The migration from flat to per-widget is a phased incremental approach, not a big-bang refactor. New types (FontSpec, TextScaleEntry, TextScale, IconSizes, DialogButtonOrder, ThemeDefaults, 24 widget structs) can be added in separate commits without breaking anything. The ThemeVariant restructure + preset TOML rewrites must be a single atomic commit because serde keys conflict between old and new shapes. After that commit, OS readers and connectors update independently. The `impl_merge!` macro requires no changes — per-widget structs use the existing `option {}` / `nested {}` categories. ThemeVariant itself switches from a hand-written merge() to `impl_merge!` because all widget fields become direct nested structs (not Option-wrapped). The resolve() function is a single hand-written function in `model/resolve.rs` that clones defaults once and processes inheritance in four explicit phases (defaults internal chains, defaults safety nets, widget-from-defaults, widget-to-widget chains). ResolvedTheme uses manual parallel structs generated by `define_widget_pair!` macro — proc macros (overkill, adds syn/quote) and the `optional_struct` crate (wrong direction: concrete->optional) are both rejected.

**Major components:**
1. `model/fontspec.rs` — FontSpec, TextScaleEntry, TextScale; dependency of ThemeDefaults and all widget structs
2. `model/defaults.rs` — ThemeDefaults; global base properties, root of all inheritance; must exist before resolve() can run
3. `model/widgets/*.rs` — 24 per-widget structs; each with color + font + sizing fields; all use `define_widget_pair!` for Option and Resolved versions
4. `model/resolve.rs` — single function, four explicit phases, ~200-300 lines; runs after every merge, idempotent
5. `model/resolved.rs` — ResolvedTheme + validate(); converts Option structs to concrete; collects all missing fields before failing
6. `gnome/mod.rs`, `kde/mod.rs`, macOS reader, Windows reader — OS readers returning sparse ThemeVariant with new fields
7. `lib.rs` (from_system()) — composes OS reader + platform TOML merge + resolve() + optional app overlay + second resolve()
8. `connectors/iced/`, `connectors/gpui/` — consume ResolvedTheme; all unwrap_or() fallbacks removed

### Critical Pitfalls

1. **Silent TOML field path breakage** — `[light.colors]` becomes `[light.defaults]`; `#[serde(default)]` silently swallows unrecognized sections producing an all-None theme with no error. Mitigation: add serde aliases for old field names during transition; use `deny_unknown_fields` in testing; add validation function that detects old-format sections and emits clear deprecation warnings. Phase: 1.

2. **28 parallel struct pairs become a maintenance nightmare** — every field added to a widget struct must be added to its Resolved counterpart, resolve(), and validate(). One missed location causes silent bugs. Mitigation: use `define_widget_pair!` macro_rules! to generate both Option and Resolved structs from a single definition; use struct literal syntax (no `..Default::default()`) in validate() so the compiler catches missing fields. Phase: 1-2.

3. **resolve() ordering bugs from dependency chains** — `defaults.selection <- defaults.accent` must run before `button.primary_bg <- defaults.accent`; if the order is wrong, button.primary_bg inherits a None. Mitigation: explicit four-phase structure with code comments; one unit test per inheritance rule documented in the inheritance table. Phase: 2.

4. **FontSpec partial inheritance requires two-level resolution** — a TOML setting only `menu.font.size` must still inherit `family` and `weight` from `defaults.font`; if resolve() treats font as atomic, partial overrides lose the un-overridden sub-fields. Mitigation: implement `FontSpec::resolve_from(&mut self, base: &FontSpec)` called from resolve() for every widget with a font field. Phase: 2.

5. **Qt5/Qt6 font weight scale mismatch** — Qt5 uses 0-99 (Normal=50, Bold=75), Qt6 uses 1-1000 (Normal=400, Bold=700); a Qt5 weight of 50 reads as near-Thin in CSS terms if interpreted as Qt6. Mitigation: detect Qt version from field count (<=16 fields = Qt5, 17+ = Qt6); convert Qt5 scale to CSS 100-900 using documented mapping. Phase: 3.

## Implications for Roadmap

Based on the dependency graph from ARCHITECTURE.md and pitfall phase mapping from PITFALLS.md, four phases are strongly indicated:

### Phase 1: Data Model Restructure
**Rationale:** ThemeVariant is the foundation everything else builds on. The atomic restructure must come first because OS readers, resolve(), and connectors all depend on it. New type definitions (FontSpec, TextScaleEntry, etc.) can precede the atomic change in sub-commits. The TOML backward compatibility strategy (serde aliases) must be implemented in this same phase — retrofitting it later risks silent data loss reaching users.
**Delivers:** New ThemeVariant with ThemeDefaults + 24 per-widget structs; all presets rewritten to new TOML format; `define_widget_pair!` macro generating Option + Resolved struct pairs; empty table suppression with `skip_serializing_if = "is_empty"` on all widget fields; serde round-trip tests pass.
**Addresses:** Per-widget structs, ThemeDefaults, FontSpec, TextScale, DialogButtonOrder, IconSizes (all P1 table-stakes features)
**Avoids:** Pitfall 1 (TOML breakage) and Pitfall 4 (empty table serialization) — both must be resolved here.

### Phase 2: resolve() and ResolvedTheme
**Rationale:** resolve() and ResolvedTheme are tightly coupled — validate() requires that resolve() has run. Both must exist before OS readers or connectors can produce useful output. Inheritance ordering bugs (Pitfall 3), FontSpec partial inheritance (Pitfall 7), and TextScaleEntry line_height dependency (Pitfall 8) are all Phase 2 concerns that must be addressed with full test coverage before moving to OS readers.
**Delivers:** `resolve()` function with explicit 4-phase structure and one test per inheritance rule; `ResolvedTheme` + `validate()` producing concrete non-optional output; `ThemeResolutionError` with field path reporting; completeness integration test (every preset + full pipeline asserts success on both light and dark variants).
**Addresses:** resolve() inheritance, ResolvedTheme + validate(), accessibility signals (all remaining P1 features)
**Avoids:** Pitfall 2 (struct explosion — define_widget_pair! from Phase 1 handles this), Pitfall 3 (ordering), Pitfall 7 (FontSpec partial), Pitfall 8 (TextScaleEntry line_height), Pitfall 9 (TOML-struct sync — completeness test enforced here).

### Phase 3: Extended OS Readers
**Rationale:** OS readers require the completed data model (Phase 1) and benefit from having resolve() and the completeness test (Phase 2) available to catch TOML-struct sync issues immediately. Each platform reader is independent. Windows requires 2 new feature flags. KDE requires Qt5/Qt6 weight detection. GNOME requires combined portal + gsettings strategy with explicit fallback.
**Delivers:** macOS reader: NSFont.TextStyle, specialized fonts (menu, tooltip, title bar), NSFontDescriptor weight extraction, textInsertionPointColor (macOS 14+ with version guard), additional NSColor values. Windows reader: GetSysColor widget colors, DwmGetColorizationColor, NONCLIENTMETRICSW font fields (caption, menu, status bar), UISettings extras, SPI_GETHIGHCONTRAST. KDE reader: [WM] section, [Colors:Header], additional font keys, Qt5/Qt6 weight detection, AnimationDurationFactor, index.theme icon sizes. GNOME reader: text-scaling-factor, enable-animations, overlay-scrolling, titlebar-font, portal reduced_motion with gsettings fallback. Also: platform TOML slimming (remove OS-readable fields, keep design constants), app TOML overlay + second resolve() pass.
**Addresses:** Extended OS readers, OS-first from_system() pipeline, app TOML overlay (all P2 features)
**Avoids:** Pitfall 5 (Qt font weight), Pitfall 6 (version-specific API guards — every new API call must include OS version/availability check).

### Phase 4: Connector Migration
**Rationale:** Connectors consume the final output — they must come last, after the full pipeline is stable. Both connectors (iced, gpui) switch from `&ThemeVariant` to `&ResolvedTheme`, eliminating all `unwrap_or()` fabrication. This is a breaking semver change for downstream consumers. The iced connector gap (`secondary_background`/`secondary_foreground` needed for Extended palette) must be addressed in ThemeDefaults during Phase 1 to avoid a rework here.
**Delivers:** native-theme-iced and native-theme-gpui updated to `&ResolvedTheme`; all fabricated fallbacks removed; showcase examples updated and verified; semver bump.
**Addresses:** Connector migration to ResolvedTheme (P3 feature)
**Avoids:** No new pitfalls — this is the integration cleanup phase.

### Phase Ordering Rationale

- Phase 1 must come first: ThemeVariant is the type boundary crossing all crate layers; nothing compiles correctly with the new design until this is done.
- Phase 2 before Phase 3: the completeness integration test must exist before OS readers add new fields; this catches TOML-struct sync issues (Pitfall 9) in Phase 3 immediately rather than in user bug reports.
- Phase 3 before Phase 4: connectors should migrate to ResolvedTheme only after from_system() can produce a verified, complete ResolvedTheme; migrating earlier leads to validate() failures in integration testing.
- Phase 4 last: the connector API is the public-facing change; defer until the internal architecture is stable.

### Research Flags

Phases with well-documented patterns — skip additional research during planning:
- **Phase 1 (Data Model):** Design fully specified in `docs/todo_v0.5.1_theme-variant.md`. Architecture research identifies exact macro structure and serde patterns. Standard Rust patterns throughout.
- **Phase 2 (resolve + ResolvedTheme):** ~90 inheritance rules documented in `docs/todo_v0.5.1_inheritance-rules.md`. Four-phase structure and borrow-checker strategy specified in ARCHITECTURE.md. Implementation is mechanical (long but not complex).

Phases that benefit from focused implementation checklists during planning:
- **Phase 3 (OS Readers):** Each reader has documented API calls and no new dependencies, but the version guard strategy for macOS 14+-specific APIs and the GNOME reduced_motion portal/gsettings fallback path need explicit implementation checklists per platform.
- **Phase 4 (Connectors):** The gpui connector's field access patterns are not fully detailed in the research files. Before planning Phase 4, read `connectors/native-theme-gpui/` source to produce the same field mapping table that ARCHITECTURE.md has for iced.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | All API calls verified against local cargo registry source files with exact line numbers. Zero new dependencies confirmed. |
| Features | HIGH | Cross-referenced against GTK4, Qt6, WinUI3, macOS HIG official documentation. Feature set mirrors what production platform theme systems expose. Design documents in `docs/todo_v0.5.1_*.md` cover every feature with implementation notes. |
| Architecture | HIGH | Based on direct source code inspection of the existing codebase, not inference. `impl_merge!` macro behavior, ThemeVariant layout, and connector field paths all verified by reading actual source files. |
| Pitfalls | HIGH | Most pitfalls identified from direct codebase analysis (serde behavior, macro semantics, OS API version gates) rather than speculation. Qt5/Qt6 weight pitfall verified against Qt documentation. |

**Overall confidence:** HIGH

### Gaps to Address

- **gpui connector field mapping:** ARCHITECTURE.md covers iced connector field mapping in detail but the gpui connector's field access patterns are not explicitly mapped. Read `connectors/native-theme-gpui/` source during Phase 4 planning to produce the same mapping table.
- **secondary_background / secondary_foreground placement:** The iced connector needs these fields; ARCHITECTURE.md recommends keeping them in ThemeDefaults. Confirm they are included in the Phase 1 ThemeDefaults struct definition to avoid Phase 4 rework.
- **Inactive variant TOML completeness:** The variant NOT detected by the OS (e.g., light when user is in dark mode) is TOML-only. Phase 2 completeness tests must cover both variants. Flagged as a "looks done but isn't" risk in PITFALLS.md.
- **Platform TOML slim-down matrix:** Which fields are "OS-reader-provided" vs "TOML design constants" per platform is documented in the inheritance rules doc but not verified in a cross-platform matrix. Phase 3 should produce and verify this matrix before removing fields from platform TOMLs.

## Sources

### Primary (HIGH confidence)
- Official GTK4 docs (docs.gtk.org) — CSS properties, cascade/inheritance, widget CSS nodes
- Official Qt6 docs (doc.qt.io) — QStyle, QPalette, QFont, stylesheet inheritance, pixelMetric
- Official WinUI3 docs (learn.microsoft.com) — XAML theme resources, BasedOn chaining, Fluent type ramp
- Apple HIG and developer docs (developer.apple.com) — macOS text styles, Dynamic Type, NSColor system colors
- Material Design 3 (m3.material.io) — type scale tokens, accessibility signals
- Fluent 2 typography (fluent2.microsoft.design)
- objc2-app-kit 0.3.2 local source (`~/.cargo/registry`) — NSFont.rs, NSFontDescriptor.rs, exact line numbers verified
- windows 0.62.2 local source (`~/.cargo/registry`) — Gdi/mod.rs, Dwm/mod.rs, Accessibility/mod.rs, exact line numbers verified
- ashpd 0.13.9 local source (`~/.cargo/registry`) — settings.rs line 313, reduced_motion() verified
- Direct source analysis of `native-theme/src/`, `connectors/`, `docs/todo_v0.5.1_*.md`

### Secondary (MEDIUM confidence)
- Apple Typography gist (GitHub) — macOS text style sizes/weights/tracking
- Qt Forum: QFont::fromString() change between Qt4 and Qt5 (weight scale difference)
- serde_with docs.rs — skip_serializing_none nested struct behavior
- serde GitHub issue #2451 — Option::None exclusion discussion

### Tertiary (LOW confidence)
- SwiftUI theming community posts — resolve() pattern in ShapeStyle
- XAML lightweight styling community posts — per-control resource key patterns

---
*Research completed: 2026-03-27*
*Ready for roadmap: yes*
