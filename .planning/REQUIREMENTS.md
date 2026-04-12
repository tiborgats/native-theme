# Requirements: native-theme v0.5.7

**Defined:** 2026-04-12
**Core Value:** Any Rust GUI app can look native on any platform by loading a single theme file or reading live OS settings, without coupling to any specific toolkit.

## v0.5.7 Requirements

No-backward-compat API overhaul implementing **every proposed solution** from the six-pass verified design documents:
- `docs/todo_v0.5.7_native-theme-api.md` (doc 1, §1–§33: API surface critique)
- `docs/todo_v0.5.7_native-theme-api-2.md` (doc 2, §A–§M: bugs, structural, API-shape, polish)

Scope per user directive (2026-04-12): **all P0, P1, P2, P3 items from both documents are in scope** (A1 is already fixed in commit `f9e5956` and is therefore excluded). All breaking changes ship in this milestone — there is no backward-compat window in v0.5.7.

### Verified Bugs

- [x] **BUG-01**: `check_ranges` stops running on `T::default()` placeholder data from `require()` fallback — short-circuit on `missing.is_empty() == false` OR two-vec split so range errors don't pollute `missing_fields` (doc 2 A2)
- [x] **BUG-02**: `ThemeResolutionError::missing_fields` stops carrying two error categories (missing vs out-of-range) — folded into the `Error` restructure (doc 2 A3)
- [x] **BUG-03**: `resolve()` documentation no longer lies about "no OS detection" — either move `button_order` OS-dispatch to `resolve_platform_defaults` only OR demote `resolve` intermediates so the doc claim is true (doc 2 A4)
- [x] **BUG-04**: `from_kde_content_pure` no longer hardcodes `button_order = "kde"` — resolver handles it (doc 2 D5)
- [x] **BUG-05**: `from_macos::build_theme` no longer hardcodes `button_order = "apple"` — resolver handles it (doc 2 M1)

### Type Vocabulary Renames

- [ ] **NAME-01**: Type rename package ships atomically: `ThemeSpec→Theme`, `ThemeVariant→ThemeMode`, `ResolvedThemeVariant→ResolvedTheme`, `ResolvedThemeDefaults→ResolvedDefaults`; `SystemTheme` unchanged (doc 1 §1 Option B)
- [ ] **NAME-02**: `ThemeWatcher` struct internals + constructor split documented; rename if needed (doc 1 §21)
- [ ] **NAME-03**: `FontSize::Px(v).to_px(dpi)` renamed to `to_logical_px` — DPI parameter is no longer silently ignored in the `Px` branch (doc 1 §25)

### Error Type Restructure

- [x] **ERR-01**: `Error::Clone` bound dropped — four-item atomic commit: drop `#[derive(Clone)]`, delete stale `error.rs:73-79` doc comment, delete stale `presets.rs:85-92` comment, delete `error_is_clone` test (doc 1 §6a + doc 2 L1)
- [x] **ERR-02**: `Error` variants restructured per §31.2 Option F (flat + `kind()` method); `WatchUnavailable` / `FeatureDisabled` / `ReaderFailed` boundaries explicit; folds in A3's category separation (doc 1 §6b/c/d)

### Data Model Restructure

- [ ] **MODEL-01**: Doubled `Option<T>` / `Resolved<T>` struct hierarchy generated from one source — new `native-theme-derive` proc-macro crate (Option K) emits paired structs, `FIELD_NAMES` constants, `impl_merge!` bodies, `check_ranges` impls, and `inventory::submit!` widget registry entries (doc 1 §2)
- [ ] **MODEL-02**: `SystemTheme` pre-resolve variant fields eliminated via `OverlaySource` replay model; `with_overlay()` rebuilds via `ResolutionContext` instead of storing pre-resolve copies (doc 1 §3)
- [ ] **MODEL-03**: `SystemTheme::active()` dropped; `pick(ColorMode)` keeps; `SystemTheme.mode: ColorMode` field exposed (doc 1 §4 Option C + §32.3 G)
- [ ] **MODEL-04**: `ThemeVariant::resolve*` method intermediates demoted to `#[doc(hidden)] pub` (Option B' preserves integration tests) (doc 1 §7)
- [ ] **MODEL-05**: `ThemeSpec` (→ `Theme`) method grab-bag cleanup; `from_toml_with_base` removal coordinates with `error.rs:63` hint message (doc 1 §15)
- [ ] **MODEL-06**: `icon_set` and `icon_theme` fields moved from the wrong type onto `Theme` (doc 1 §20)

### Accessibility & Runtime State

- [ ] **ACCESS-01**: Accessibility fields (`text_scaling_factor`, `reduce_motion`, `high_contrast`, `reduce_transparency`) extracted from `ThemeDefaults` into `AccessibilityPreferences` struct; lives on `SystemTheme`, NOT in `ResolutionContext` (doc 2 B4)
- [ ] **ACCESS-02**: `font_dpi` moved from `ThemeDefaults` into `ResolutionContext` runtime-data struct (doc 2 B5)

### Border Model

- [ ] **BORDER-01**: `BorderSpec` split along defaults-vs-widget — widget-level has `color`, defaults-level adds `width`, `corner_radius`, `padding`; ships hand-written in v0.5.7 with codegen migration deferred to v0.5.8 (doc 2 B6)
- [ ] **BORDER-02**: Three parallel border-inheritance validation paths (`require_border` / `border_all_optional` / `require_border_partial`) unified via `#[theme_layer(border_kind = "full" | "partial" | "none")]` class-level attribute (doc 2 B7, subsumed by MODEL-01 K)

### Validation & Codegen

- [ ] **VALID-01**: ~720 lines of hand-written validate/range-check boilerplate generated from `#[theme(range = "...")]` and `#[theme(check = "non_negative")]` attributes via `native-theme-derive` (doc 2 B1)
- [ ] **VALID-02**: Inheritance rules deduplicated between `inheritance-rules.toml` and `inheritance.rs` — per-field `#[theme(inherit_from = "...")]` attributes cover the ~55 of 82 expressible uniform rules; pattern-based rules stay hand-written (doc 2 B2)
- [ ] **VALID-03**: `lint_toml` driven by `inventory::submit!` widget registry entries instead of ~215 hand-maintained string literals (doc 1 §14)
- [ ] **VALID-04**: `check_ranges` stops eager `format!` path-string construction; allocate path strings lazily (doc 2 D3)

### Platform Readers

- [ ] **READER-01**: `ReaderOutput` contract homogenised across KDE/GNOME/Windows/macOS — unified single-vs-dual variant semantics; type flows through `run_pipeline` alongside `OverlaySource` from MODEL-02 (doc 2 B3)
- [ ] **READER-02**: `from_kde`/`from_gnome`/`from_windows`/`from_macos` demoted to `pub(crate)` after L3 visibility audit (doc 2 C6)

### Watchers & Events

- [x] **WATCH-01**: `ThemeChangeEvent::Other` removed — it has zero production emitters (doc 2 C1)
- [x] **WATCH-02**: `ThemeChangeEvent::ColorSchemeChanged` renamed to `Changed` — the `ColorScheme` framing is platform-inaccurate on KDE/GNOME which signal broader changes (doc 2 C2)
- [x] **WATCH-03**: `on_theme_change` fails at compile-time (not runtime) on missing `watch` feature — API is feature-gated at type/function level (doc 1 §22)

### Icons

- [ ] **ICON-01**: 13 icon-loading functions collapsed into one builder API — `IconLoader::new().role(...).size(...).theme(...).load()` with `impl Into<IconId>` constructor (doc 1 §8)
- [ ] **ICON-02**: `load_icon` freedesktop size-24 hardcode removed (covered by ICON-01 builder's `size()` method) (doc 1 §9)
- [ ] **ICON-03**: `IconProvider::icon_svg` returns `Cow<'static, [u8]>` instead of `&'static [u8]` — removes lifetime lock (doc 1 §10)
- [ ] **ICON-04**: `IconData::Svg` stores `Cow<'static, [u8]>` to avoid the `Vec<u8>` copy on bundled icon loads (doc 1 §11)
- [x] **ICON-05**: `IconSet::default()` removed — it was Freedesktop on all platforms, misleading on macOS/Windows (doc 1 §17)
- [ ] **ICON-06**: Drift-guard test added for `IconSet::from_name`/`name` round-trip (doc 1 §18, revised from strum)
- [ ] **ICON-07**: `IconRole::name()` method returning kebab-case added; `impl Display for IconRole` delegates to `name()` (not `Debug::fmt`) (doc 1 §33 F1)

### Crate Root Layout

- [ ] **LAYOUT-01**: 92-item flat crate root partitioned into submodules (`theme::`, `watch::`, `icons::`, `detect::`, etc.) with a `prelude` module exposing the 6 most-used items (doc 1 §12 Option C+F)
- [ ] **LAYOUT-02**: `LinuxDesktop` marked `#[non_exhaustive]`; new Wayland compositor variants added (Hyprland, Sway, River, Niri, etc.) (doc 1 §19)
- [ ] **LAYOUT-03**: `AnimatedIcon` public fields replaced with newtype wrappers that enforce construction invariants (no invalid public-field states possible) (doc 2 C3)
- [ ] **LAYOUT-04**: Font `family: String` migrated to `Arc<str>` across widget × connector leak; needs `serde rc` feature flag; gpui and iced connector `.family` access migrated in lockstep (doc 2 C4)

### Color Polish

- [x] **COLOR-01**: `Rgba` polish — default constants, naming cleanup, `to_f32_tuple` deleted (F2 concurrent-discovery confirmation) (doc 1 §16 + §33 F2)

### Detection Caching

- [ ] **DETECT-01**: Global `OnceLock` caches in `detect` and `model/icons` replaced by `DetectionContext` with invalidation; `arc_swap::ArcSwapOption<T>` supports both "cache on first read" and "force re-read on demand" (doc 1 §13 Option C+F)
- [ ] **DETECT-02**: `detect_linux_desktop()` no-arg convenience overload added — current `&str` signature forces a two-call idiom (doc 2 C5)

### Feature Flags & Async

- [ ] **FEATURE-01**: `from_system_async` and `from_system` unified via `pollster::block_on(async_inner)` — one code path, no async runtime exposed to sync consumers (doc 1 §5 Option G)
- [ ] **FEATURE-02**: Feature aggregators split into clearer `linux-kde`/`linux-portal`-style groupings (doc 2 L4)
- [ ] **FEATURE-03**: Feature-matrix cleanup bundled atomically with FEATURE-01 and FEATURE-02 — `Cargo.toml` feature graph simplified (doc 1 §31.3)

### Polish & Documentation

- [ ] **POLISH-01**: `diagnose_platform_support` returns `Vec<DiagnosticEntry>` instead of `Vec<String>` (doc 1 §23 Option B)
- [ ] **POLISH-02**: `platform_preset_name` returns structured data instead of leaking internal `-live` naming convention (doc 1 §24)
- [x] **POLISH-03**: `#[must_use]` convention trimmed to uniform bare `#[must_use]` across the crate — six call sites updated: `pipeline.rs:132`, `pipeline.rs:175`, `model/icons.rs:438`, `model/icons.rs:477`, `lib.rs:353` (`from_system`), `model/mod.rs:225` (`ThemeSpec` struct) (doc 1 §26 + §33 F3)
- [ ] **POLISH-04**: `FontSpec::style` default-consistency documented — current behaviour silently defaults while sibling fields error (doc 2 D1)
- [ ] **POLISH-05**: `defaults.border.padding` derives-from-presence rule documented or corrected (symptom of BORDER-01) (doc 2 D2)
- [ ] **POLISH-06**: Bundled preset `name` and `icon_theme` stored as `Cow<'static, str>` to avoid owned-String allocation (doc 2 D4)

### Test & Doc Cleanup

- [x] **CLEAN-01**: Stale `error.rs:73-79` doc comment deleted — bundled atomically with ERR-01's four-item commit (doc 2 L1)
- [x] **CLEAN-02**: Redundant `ENV_MUTEX` tests simplified after BUG-03 lands — resolve becomes pure, so env-var-mocking tests no longer need mutex serialization (doc 2 L2)
- [ ] **CLEAN-03**: `from_kde`/`from_gnome`/`from_windows`/`from_macos` visibility audit — grep pass for connector-level consumers before READER-02 demotion (doc 2 L3)

## Future Requirements

Deferred to v0.6.0+ / v1.0:

### Two-Crate Split (v1.0)

- **SPLIT-01**: `native-theme-model` (pure data library: types, TOML, inheritance, resolve, merge)
- **SPLIT-02**: `native-theme-system` (detection/watchers/icon loaders, depends on `-model`)

Per `docs/todo_v0.5.7_native-theme-api-2.md` §G post-script — the v0.5.7 work addresses symptoms; SPLIT-01/02 addresses the root cause ("two codebases pretending to be one").

### Connector Expansions (v0.6.0 / v0.6.1)

- **CONN-01**: iced connector full theme + geometry coverage (per `docs/todo_v0.6.0_iced-full-theme-geometry.md`)
- **CONN-02**: gpui connector full theme coverage (per `docs/todo_v0.6.1_gpui-full-theme.md`)
- **CONN-03**: egui connector crate (`native-theme-egui`)
- **CONN-04**: Extended watching (accent color, fonts, icon theme, high contrast toggle)
- **CONN-05**: ThemeWatcher integration helpers for gpui and iced consumers

## Out of Scope

| Feature | Reason |
|---------|--------|
| A1 `Instant` checked_sub fix | Already shipped in commit `f9e5956` on `main` — `watch/kde.rs` uses `Option<Instant>` |
| Backward compatibility with v0.5.6 public API | v0.5.7 is an explicit no-backcompat window; all breaking changes bundled into this release |
| Two-crate split (`native-theme-model` + `native-theme-system`) | Deferred to v1.0 per design-doc §G post-script — v0.5.7 addresses symptoms; split addresses root cause |
| Connector API expansions | Deferred to v0.6.0/v0.6.1 per separate design docs (`todo_v0.6.0_iced-*`, `todo_v0.6.1_gpui-*`) |
| iOS / Android readers | Deferred to post-1.0 |
| CSS/SCSS export format | Trivially implementable by consumers |
| Named palette colors | Too platform-specific to standardise |
| Change notification "replay theme" semantics | Out of scope per v0.5.6 design — watcher is signal-only; consumers re-run `from_system()` |
| `strum` dependency for `IconSet` | Revised recommendation: drift-guard test (ICON-06); dependency conservatism argues against proc-macro dep for 4-variant enum |

## Traceability

Which phases cover which requirements. Filled in during roadmap creation (2026-04-12).

| Requirement | Priority | Source | Phase | Status |
|-------------|----------|--------|-------|--------|
| BUG-01 | P0 | doc 2 A2 | Phase 71 | Pending |
| BUG-02 | P0 | doc 2 A3 | Phase 71 | Pending |
| BUG-03 | P0 | doc 2 A4 | Phase 69 | Pending |
| BUG-04 | P0 | doc 2 D5 | Phase 69 | Pending |
| BUG-05 | P0 | doc 2 M1 | Phase 69 | Pending |
| NAME-01 | P0 | doc 1 §1 | Phase 76 | Pending |
| NAME-02 | P3 | doc 1 §21 | Phase 85 | Pending |
| NAME-03 | P3 | doc 1 §25 | Phase 85 | Pending |
| ERR-01 | P0 | doc 1 §6a + doc 2 L1 | Phase 70 | Pending |
| ERR-02 | P1 | doc 1 §6b/c/d | Phase 71 | Pending |
| MODEL-01 | P1 | doc 1 §2 | Phase 80 | Pending |
| MODEL-02 | P0 | doc 1 §3 | Phase 78 | Pending |
| MODEL-03 | P0 | doc 1 §4 | Phase 77 | Pending |
| MODEL-04 | P1 | doc 1 §7 | Phase 85 | Pending |
| MODEL-05 | P1 | doc 1 §15 | Phase 85 | Pending |
| MODEL-06 | P0 | doc 1 §20 | Phase 77 | Pending |
| ACCESS-01 | P0 | doc 2 B4 | Phase 78 | Pending |
| ACCESS-02 | P1 | doc 2 B5 | Phase 78 | Pending |
| BORDER-01 | P0 | doc 2 B6 | Phase 79 | Pending |
| BORDER-02 | P2 | doc 2 B7 | Phase 80 | Pending |
| VALID-01 | P1 | doc 2 B1 | Phase 80 | Pending |
| VALID-02 | P1 | doc 2 B2 | Phase 80 | Pending |
| VALID-03 | P3 | doc 1 §14 | Phase 86 | Pending |
| VALID-04 | P2 | doc 2 D3 | Phase 86 | Pending |
| READER-01 | P1 | doc 2 B3 | Phase 84 | Pending |
| READER-02 | P1 | doc 2 C6 | Phase 79 | Pending |
| WATCH-01 | P0 | doc 2 C1 | Phase 73 | Pending |
| WATCH-02 | P0 | doc 2 C2 | Phase 73 | Pending |
| WATCH-03 | P0 | doc 1 §22 | Phase 75 | Pending |
| ICON-01 | P1 | doc 1 §8 | Phase 82 | Pending |
| ICON-02 | P3 | doc 1 §9 | Phase 82 | Pending |
| ICON-03 | P2 | doc 1 §10 | Phase 82 | Pending |
| ICON-04 | P2 | doc 1 §11 | Phase 82 | Pending |
| ICON-05 | P0 | doc 1 §17 | Phase 75 | Pending |
| ICON-06 | P3 | doc 1 §18 | Phase 82 | Pending |
| ICON-07 | P3 | doc 1 §33 F1 | Phase 82 | Pending |
| LAYOUT-01 | P0 | doc 1 §12 | Phase 76 | Pending |
| LAYOUT-02 | P0 | doc 1 §19 | Phase 75 | Pending |
| LAYOUT-03 | P1 | doc 2 C3 | Phase 87 | Pending |
| LAYOUT-04 | P1 | doc 2 C4 | Phase 87 | Pending |
| COLOR-01 | P0 | doc 1 §16 | Phase 74 | Pending |
| DETECT-01 | P2 | doc 1 §13 | Phase 83 | Pending |
| DETECT-02 | P2 | doc 2 C5 | Phase 83 | Pending |
| FEATURE-01 | P2 | doc 1 §5 Option G | Phase 81 | Pending |
| FEATURE-02 | P2 | doc 2 L4 | Phase 81 | Pending |
| FEATURE-03 | P2 | doc 1 §31.3 | Phase 81 | Pending |
| POLISH-01 | P2 | doc 1 §23 | Phase 88 | Pending |
| POLISH-02 | P2 | doc 1 §24 | Phase 88 | Pending |
| POLISH-03 | P3 | doc 1 §26 + §33 F3 | Phase 74 | Pending |
| POLISH-04 | P3 | doc 2 D1 | Phase 88 | Pending |
| POLISH-05 | P3 | doc 2 D2 | Phase 88 | Pending |
| POLISH-06 | P2 | doc 2 D4 | Phase 88 | Pending |
| CLEAN-01 | P0 | doc 2 L1 | Phase 70 | Pending |
| CLEAN-02 | P1 | doc 2 L2 | Phase 72 | Pending |
| CLEAN-03 | P1 | doc 2 L3 | Phase 79 | Pending |

**Coverage:**
- v0.5.7 requirements: 55 total
- Mapped to phases: 55
- Unmapped: 0 ✓

**Phase → Requirement counts:**

| Phase | Ship Unit | Requirement Count | Requirements |
|-------|-----------|-------------------|--------------|
| 69 | 1 (atomic) | 3 | BUG-03, BUG-04, BUG-05 |
| 70 | 3 (atomic) | 2 | ERR-01, CLEAN-01 |
| 71 | 2 (atomic) | 3 | BUG-01, BUG-02, ERR-02 |
| 72 | 4 | 1 | CLEAN-02 |
| 73 | 5 | 2 | WATCH-01, WATCH-02 |
| 74 | 6 (part A) | 2 | COLOR-01, POLISH-03 |
| 75 | 6 (part B) | 3 | LAYOUT-02, WATCH-03, ICON-05 |
| 76 | 7 (part A) | 2 | NAME-01, LAYOUT-01 |
| 77 | 7 (part B) | 2 | MODEL-03, MODEL-06 |
| 78 | 8 (atomic) | 3 | MODEL-02, ACCESS-01, ACCESS-02 |
| 79 | 9 | 3 | BORDER-01, CLEAN-03, READER-02 |
| 80 | 10 | 4 | MODEL-01, VALID-01, VALID-02, BORDER-02 |
| 81 | 11 (atomic) | 3 | FEATURE-01, FEATURE-02, FEATURE-03 |
| 82 | — | 6 | ICON-01, ICON-02, ICON-03, ICON-04, ICON-06, ICON-07 |
| 83 | — | 2 | DETECT-01, DETECT-02 |
| 84 | — | 1 | READER-01 |
| 85 | — | 4 | MODEL-04, MODEL-05, NAME-02, NAME-03 |
| 86 | — | 2 | VALID-03, VALID-04 |
| 87 | — | 2 | LAYOUT-03, LAYOUT-04 |
| 88 | — | 5 | POLISH-01, POLISH-02, POLISH-04, POLISH-05, POLISH-06 |
| **Total** | — | **55** | — |

**Ship-unit bundling constraints** (from doc 1 §33.13 + doc 2 §L.5):

| Ship Unit | Atomic? | Requirements | Phase |
|-----------|---------|--------------|-------|
| Unit 1 | **Yes** | BUG-03 + BUG-04 + BUG-05 (M1 + D5 + A4) | Phase 69 |
| Unit 2 | **Yes** | BUG-01 + BUG-02 + ERR-02 (A2 + A3 + §6 restructure) | Phase 71 |
| Unit 3 | **Yes** | ERR-01 + CLEAN-01 (§6a four-item) | Phase 70 |
| Unit 4 | No (after Unit 1) | CLEAN-02 (L2 test simplification) | Phase 72 |
| Unit 5 | No | WATCH-01 + WATCH-02 (C1 + C2) | Phase 73 |
| Unit 6 | No | COLOR-01 + ICON-05 + LAYOUT-02 + WATCH-03 + POLISH-03 (§16/17/19/22/26 polish) | Phases 74, 75 |
| Unit 7 | No (large refactor) | NAME-01 + LAYOUT-01 + MODEL-03 + MODEL-06 (§1/12/4/20) | Phases 76, 77 |
| Unit 8 | **Yes** | MODEL-02 + ACCESS-02 + ACCESS-01 (§3 + B5 + B4) | Phase 78 |
| Unit 9 | No | BORDER-01 + CLEAN-03 + READER-02 (B6 + L3 + C6) | Phase 79 |
| Unit 10 | No | MODEL-01 + VALID-01 + VALID-02 + BORDER-02 (minimum-viable K codegen) | Phase 80 |
| Unit 11 | **Yes** | FEATURE-01 + FEATURE-02 + FEATURE-03 (§5 G + L4 + §31.3) | Phase 81 |

Requirements not assigned to a ship unit are P1/P2/P3 follow-ups bundled thematically in Phases 82–88.

---
*Requirements defined: 2026-04-12*
*Last updated: 2026-04-12 — roadmap created, all 55 requirements mapped to Phases 69–88*
