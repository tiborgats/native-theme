---
phase: 94-close-remaining-v0-5-7-gaps-g6-g8-border-inheritance-codegen-resolutioncontext-themereader-trait
plan: 03
subsystem: reader
tags: [refactor, trait, async-trait, pipeline, dispatch, object-safety, v057-polish, G8]

# Dependency graph
requires:
  - phase: 94
    provides: "Plan 94-02 landed ResolutionContext + wired pipeline.rs to build ctx per invocation; provides the pipeline.rs base that this plan's select_reader call site sits on."
  - phase: 93
    provides: "Phase 93-07/93-08 established ./pre-release-check.sh as the release gate (not cargo test --workspace); Phase 93-09 established the typed-per-set refactor precedent (structural restructure to eliminate silent-bug classes) that informs this plan's typed-reader-per-backend approach."
  - phase: 79
    provides: "Phase 79-02 C6 exception: from_kde_content_pure stays pub because native-theme/tests/reader_kde.rs imports it from outside the crate. Preserved verbatim in this plan."
provides:
  - "`native-theme/src/reader.rs`: new pub(crate) module housing the `ThemeReader` trait (#[async_trait::async_trait], Send + Sync supertraits)."
  - "`pub(crate) trait ThemeReader`: object-safe async trait contract for per-platform readers. Box<dyn ThemeReader> coercion is the load-bearing object-safety probe."
  - "Five pub(crate) unit structs + `#[async_trait::async_trait] impl ThemeReader`: KdeReader (kde/mod.rs), GnomeReader + GnomePortalKdeReader (gnome/mod.rs, latter gated on feature=kde), MacosReader (macos.rs), WindowsReader (windows.rs)."
  - "`pipeline::select_reader() -> Option<(Box<dyn ThemeReader>, &'static str)>`: the platform+feature cascade as a single async function. Tuple return keeps reader + live-preset name tied because DE detection picks both together (KDE -> kde-breeze-live, GNOME -> adwaita-live, macOS -> macos-sonoma-live, Windows -> windows-11-live)."
  - "`pipeline::from_system_inner`: body collapsed from a ~150-line #[cfg(...)] dispatch ladder to a ~30-line `if let Some((r, p)) = select_reader().await { ... }` with a small preset-only fallback for unsupported platform+feature combos."
  - "`async-trait = \"0.1\"` promoted from transitive (via zbus) to direct dep in native-theme/Cargo.toml. Same package, same version (0.1.89) — zero supply-chain impact."
affects: [95, 96, 97]  # Future plans adding new platform backends implement the trait + extend select_reader; mock-reader substitution for testing unblocked (out of G8 scope but trait is internal-same-crate so test modules can add `struct MockReader;` impls).

# Tech tracking
tech-stack:
  added:
    - "async-trait 0.1 direct dep (promoted from transitive; zero supply-chain impact — same crate, same version 0.1.89)"
  patterns:
    - "Object-safe-async-trait via #[async_trait::async_trait] macro annotation (Option A from the three-option table). Macro rewrites async fn -> Pin<Box<dyn Future + Send + '_>> making Box<dyn Trait> compile. Native async-fn-in-trait NOT object-safe on stable Rust 1.95."
    - "Typed-reader-per-backend (unit struct + impl trait) replaces typed-free-function-per-backend. Each reader knows its identity via its type; dispatcher passes Box<dyn Trait> through one vtable. Mirrors Phase 93-09 typed-per-set IconLoader refactor (both eliminate silent-ignore bug classes at the type system level)."
    - "Tuple-return for select_reader -> Option<(Box<dyn Reader>, &'static str)> instead of pure reader option. Live-preset name is part of the reader selection because DE detection picks both together; separating them would re-create the ~100-line cfg ladder in from_system_inner."

key-files:
  created:
    - "native-theme/src/reader.rs (internal trait + docs; 46 LoC)"
  modified:
    - "native-theme/Cargo.toml (add async-trait = \"0.1\" direct dep)"
    - "native-theme/src/lib.rs (add `mod reader;` alongside pipeline/presets/resolve)"
    - "native-theme/src/kde/mod.rs (replace `pub(crate) fn from_kde` with KdeReader struct + trait impl)"
    - "native-theme/src/gnome/mod.rs (replace `from_gnome` + `from_kde_with_portal` with GnomeReader + GnomePortalKdeReader)"
    - "native-theme/src/macos.rs (replace `from_macos` with MacosReader struct + trait impl; body extracted to private `read_macos()` to keep sync body testable via direct call)"
    - "native-theme/src/windows.rs (replace `from_windows` with WindowsReader struct + trait impl; body extracted to private `read_windows()` to keep sync body testable via direct call)"
    - "native-theme/src/pipeline.rs (add `select_reader()`; reduce `from_system_inner` body; 6 Task-1 RED tests added in `theme_reader_trait_tests` module)"

key-decisions:
  - "Option A (async-trait crate with #[async_trait::async_trait] macro annotation) selected over Option B (manual Pin<Box<dyn Future>>) and Option C (trait_variant crate). async-trait is already transitive via zbus per Cargo.lock 2026-04-20 — promoting to direct dep is metadata-only (same package v0.1.89, zero supply-chain impact). Option B would be zero-dep but verbose across 5 impl sites with varying lifetime requirements; Option C (trait_variant, released late 2024) would add a NEW transitive dep for marginal gain. async-trait is battle-tested (maintained since 2019), idiomatic in ~60% of async-trait-containing crates on crates.io."
  - "Trait kept pub(crate), not pub. Per docs/todo_v0.5.7_gaps.md §G8 (\"Define trait ThemeReader { ... } internally\") and Phase 79-02 C6 audit (readers demoted to pub(crate)). Consumers call SystemTheme::from_system() / from_system_async() and never touch readers; the dispatch machinery is an implementation detail."
  - "`: Send + Sync` supertrait bound on the trait definition. Required so the macro-generated `Pin<Box<dyn Future + Send>>` return type satisfies Send — native-theme's async-runtime-agnostic `pollster::block_on` path and ashpd's D-Bus paths both require Send futures. All 5 unit-struct impls are zero-size with no interior mutability, so the bound adds no runtime cost."
  - "`select_reader() -> Option<(Box<dyn ThemeReader>, &'static str)>` tuple return instead of pure `Option<Box<dyn ThemeReader>>` (deviation from plan's `<interfaces>` block). Rationale: the live-preset name (`kde-breeze-live`, `adwaita-live`, etc.) is chosen by the same DE-detection step that picks the reader. Returning them together keeps the DE -> (reader, preset) association in ONE function; splitting them across select_reader + preset-lookup would re-create the cfg ladder in from_system_inner. The plan's interface block gave the simpler signature for clarity of intent; the tuple form is the correct refinement once the dispatcher body is collapsed."
  - "MacosReader / WindowsReader bodies extracted to private `read_macos()` / `read_windows()` helpers; the trait impl's `async fn read` is a thin wrapper. This keeps the sync body directly callable from unit tests and other internal code paths without going through the async trait surface. Mirrors the `from_kde_content` + `from_kde_content_pure` pattern preserved from Phase 79-02 — the trait dispatches, the helper implements."
  - "GnomePortalKdeReader::read() composes via `<crate::kde::KdeReader as crate::reader::ThemeReader>::read(&crate::kde::KdeReader).await?` instead of calling the deleted `from_kde()` free function. Future KDE-side migrations (extra I/O, extra parsing) automatically propagate to the portal-composite reader through the trait surface."
  - "Three helpers preserved verbatim per C6 and test-support invariants: `from_kde_content_pure` (pub, C6 exception for integration tests), `from_kde_content` (pub(crate), 25+ unit tests depend on its direct call shape), `from_kde_at` (pub(crate) #[cfg(test)], path-driven variant). KdeReader::read() delegates to `from_kde_content` after filesystem IO — avoids duplicating the 25-unit-test-covered parsing logic in the trait impl body."
  - "`return` statements in select_reader's cfg-gated arms kept (with `#[allow(clippy::needless_return)]`) because the cfg combinations produce arms that are syntactically present on different feature sets. A pure expression rewrite would require match arms that only compile under a union of cfg predicates, complicating the dispatch for no functional gain. The allow is narrowly scoped to select_reader + from_system_inner; the surrounding code is clippy -D warnings clean."

patterns-established:
  - "Object-safe-async-trait pattern: When a trait needs to be consumed as Box<dyn Trait> AND has async methods, use `#[async_trait::async_trait]` macro on both the trait definition and every impl. Native async-fn-in-trait is static-dispatch-only on stable Rust 1.95. Document the pattern at the trait definition site (reader.rs module docs do this)."
  - "Typed-reader-per-backend dispatch: Each backend is a pub(crate) unit struct implementing a single pub(crate) trait. Dispatcher (select_reader) returns `Option<(Box<dyn Trait>, metadata)>` when the backend + metadata are picked together. from_system_inner becomes an `if let Some((r, m)) = dispatcher() { r.read().await? }` with a feature-gated preset-only fallback for the None branch."
  - "Async-trait body = sync helper wrapper: For backends whose I/O is genuinely sync (macOS CoreGraphics, Windows registry), extract the body to a private `read_X()` free function; the trait impl's `async fn read` is a one-line call to it. Keeps unit tests that want to exercise the sync body directly callable without pollster::block_on, while the trait dispatch path is preserved."
  - "Trait-composite reader: GnomePortalKdeReader's read() calls `<KdeReader as ThemeReader>::read(&KdeReader).await?` instead of duplicating KDE parsing or calling a free function. Future per-backend changes propagate automatically through the trait surface to the composite reader."

requirements-completed: [G8]

# Metrics
duration: 15m
completed: 2026-04-20
---

# Phase 94 Plan 03: G8 ThemeReader Trait + Per-Backend Reader Structs Summary

**Reader surface normalised into a pub(crate) async trait dispatched via `select_reader() -> Option<(Box<dyn ThemeReader>, &'static str)>`; five free `from_*` functions replaced by unit-struct impls; pipeline dispatch collapses from a ~150-line #[cfg(...)] match ladder to a ~30-line `if let Some((r, p)) = select_reader().await { r.read().await?; ... }` body.**

## Performance

- **Duration:** 15 min (wall-clock: 2026-04-20T00:21:03Z → 2026-04-20T00:35:53Z)
- **Started:** 2026-04-20T00:21:03Z
- **Completed:** 2026-04-20T00:35:53Z
- **Tasks:** 2 (RED + GREEN, both atomic)
- **Files modified:** 9 (8 source + 1 Cargo.lock metadata update)

## Accomplishments

- **Object-safe async trait via `#[async_trait::async_trait]`**: `pub(crate) trait ThemeReader: Send + Sync { async fn read(&self) -> crate::Result<crate::ReaderResult>; }` in new `native-theme/src/reader.rs`. The Send+Sync supertrait bound plus the macro rewrite produce a `Pin<Box<dyn Future + Send + '_>>` return type that vtables can hold, making `Box<dyn ThemeReader>` compile on stable Rust 1.95 (where native async-fn-in-trait is NOT object-safe for dyn dispatch).
- **Five typed readers**: KdeReader, GnomeReader, GnomePortalKdeReader (feature=kde), MacosReader, WindowsReader — all pub(crate), all zero-size unit structs, all annotated with `#[async_trait::async_trait] impl ThemeReader`. Five free functions deleted: `from_kde`, `from_gnome`, `from_kde_with_portal`, `from_macos`, `from_windows`.
- **Pipeline dispatch collapsed**: `pipeline::select_reader()` encapsulates the platform+feature cascade (macOS, Windows, Linux KDE/GNOME/Budgie/other, Linux Unknown with portal-backend detection refinement + sync kdeglobals fallback). `from_system_inner` body reduces to the `if let Some((reader, preset_live)) = select_reader().await { ... }` pattern + a small preset-only fallback block for unsupported platform+feature combos.
- **async-trait promoted from transitive to direct dep**: same package, same version (0.1.89 per Cargo.lock 2026-04-20), **zero supply-chain impact**. The direct entry in native-theme/Cargo.toml just lets our own source use `#[async_trait::async_trait]`.
- **Three helpers preserved verbatim**: `from_kde_content_pure` (pub, C6 exception for native-theme/tests/reader_kde.rs), `from_kde_content` (pub(crate), 25+ unit tests depend on its direct call shape), `from_kde_at` (pub(crate) #[cfg(test)], path-driven). KdeReader::read() delegates to `from_kde_content` after filesystem IO, avoiding duplication of the unit-test-covered parser logic.
- **GnomePortalKdeReader composes through the trait**: `<crate::kde::KdeReader as crate::reader::ThemeReader>::read(&crate::kde::KdeReader).await?` instead of calling the deleted `from_kde()` free function. Future KDE migrations propagate automatically.
- **Full test matrix green**: 891 tests pass with all-features, 557 lib tests pass on no-default-features, 725 lib tests pass with `--features "kde,portal"`. `./pre-release-check.sh` banner green (1185+ tests, 0 failed, clippy -D warnings clean). Six Task-1 RED tests (object-safety probes + per-platform reader existence + select_reader contract) now all PASS.

## Task Commits

Each task committed atomically:

1. **Task 1 (RED):** Six failing regression tests asserting trait existence, object-safety, per-platform reader struct existence, and select_reader shape — `ce10734` (`test(94-03): RED - regression tests for G8 ThemeReader trait + per-platform reader structs + select_reader`). Before Task 2: all 6 fail to compile with expected errors (`cannot find reader in crate`, `no struct KdeReader/GnomeReader/etc`, `cannot find function select_reader`).
2. **Task 2 (GREEN):** Atomic 9-file refactor introducing ThemeReader trait + 5 impls + select_reader + pipeline collapse + async-trait direct dep — `66423ae` (`refactor(94-03): introduce ThemeReader trait (async-trait) + per-backend reader structs; collapse pipeline dispatch to select_reader`). All 6 RED tests now PASS; 891-test matrix green; pre-release-check green.

No REFACTOR phase needed — the GREEN commit already produced the final shape.

## Files Created/Modified

- **`native-theme/src/reader.rs` (created, 46 LoC)**: `pub(crate) trait ThemeReader: Send + Sync` + `#[async_trait::async_trait]` annotation + module docs explaining Option A rationale. Single responsibility: declare the reader contract.
- **`native-theme/Cargo.toml` (modified)**: add `async-trait = "0.1"` to `[dependencies]` (promoted from transitive via zbus; zero supply-chain impact).
- **`native-theme/src/lib.rs` (modified)**: add `mod reader;` declaration alongside the other internal modules (no pub use — trait is crate-internal).
- **`native-theme/src/kde/mod.rs` (modified, ~40 LoC net delta)**: replace `pub(crate) fn from_kde()` with `pub(crate) struct KdeReader;` + `#[async_trait::async_trait] impl crate::reader::ThemeReader for KdeReader`. Body preserved verbatim inside the async trait method. `from_kde_content_pure`, `from_kde_content`, `from_kde_at` all unchanged.
- **`native-theme/src/gnome/mod.rs` (modified, ~55 LoC net delta)**: replace `from_gnome()` + `from_kde_with_portal()` with `GnomeReader` + `GnomePortalKdeReader` (latter gated on feature=kde). GnomePortalKdeReader::read() composes through the trait surface to KdeReader::read(). `detect_portal_backend` stays as a detection helper (not a reader).
- **`native-theme/src/macos.rs` (modified, ~15 LoC net delta)**: replace `from_macos()` with `MacosReader` struct + `#[async_trait::async_trait] impl ThemeReader`. Body extracted to private `read_macos()` helper so the sync body remains directly callable.
- **`native-theme/src/windows.rs` (modified, ~15 LoC net delta)**: replace `from_windows()` with `WindowsReader` struct + `#[async_trait::async_trait] impl ThemeReader`. Body extracted to private `read_windows()` helper.
- **`native-theme/src/pipeline.rs` (modified, ~100 LoC net delta)**: add `select_reader() -> Option<(Box<dyn ThemeReader>, &'static str)>`; reduce `from_system_inner` body from ~150 lines (feature-gated match-on-LinuxDesktop) to ~30 lines (`if let Some((reader, preset_live)) = select_reader().await { r.read().await?; run_pipeline(result, preset_live, mode) } else { preset-only fallback }`). Task 1 RED tests added in `theme_reader_trait_tests` module at the end of the file.

## Decisions Made

See the `key-decisions:` frontmatter block above. Key points:

- **Option A (async-trait) over B (manual Pin<Box<dyn Future>>) and C (trait_variant)**: already transitive via zbus; zero supply-chain impact; idiomatic; macro produces object-safe Pin<Box<dyn Future + Send + '_>> return type.
- **Trait pub(crate), not pub**: per §G8 "internally"; Phase 79-02 C6 audit; dispatch machinery is impl detail.
- **: Send + Sync supertrait**: required for Send on the macro-generated future return type; zero runtime cost on zero-size unit struct impls.
- **Tuple return from select_reader**: live-preset name tied to reader selection. Separating them would re-create the ~100-line cfg ladder.
- **Body-extraction pattern for macOS/Windows readers**: private `read_macos()` / `read_windows()` helpers keep sync bodies directly callable. Mirrors Phase 79-02's `from_kde_content_pure` + `from_kde_content` pattern (trait dispatches, helper implements).
- **Trait-composite dispatch for GnomePortalKdeReader**: `<KdeReader as ThemeReader>::read(&KdeReader).await?` auto-propagates KDE-side migrations.
- **Three helpers preserved**: `from_kde_content_pure` (C6), `from_kde_content` (25 unit tests), `from_kde_at` (path-driven, #[cfg(test)]).

## Deviations from Plan

### 1. [Rule 3 - Blocking] select_reader return type refined to tuple

- **Found during:** Task 2 (GREEN), Step G (pipeline.rs collapse)
- **Issue:** The plan's `<interfaces>` block specified `select_reader() -> Option<Box<dyn crate::reader::ThemeReader>>` (pure reader option). Implementing that shape would leave the preset name (`kde-breeze-live`, `adwaita-live`, etc.) as a separate cascade in `from_system_inner` — re-creating the ~100-line cfg ladder the plan was collapsing.
- **Fix:** Extended select_reader's return type to `Option<(Box<dyn ThemeReader>, &'static str)>` carrying the companion `-live` preset name. This keeps reader-selection and preset-selection in ONE function. The DE-detection logic picks both together (KDE => kde-breeze-live, GNOME => adwaita-live, macOS => macos-sonoma-live, Windows => windows-11-live); exposing them as a tuple is the natural refinement of the plan's original simpler-for-documentation signature.
- **Files modified:** native-theme/src/pipeline.rs (select_reader definition + from_system_inner call site + Task 1 RED test assertion on the tuple shape).
- **Verification:** `cargo test -p native-theme --all-features` green; select_reader_returns_platform_specific_impl RED test updated to destructure the tuple and assert the `-live` preset invariant.
- **Committed in:** `66423ae` (Task 2 GREEN commit).

### 2. [Rule 1 - Lint regression] Clippy needless_return + dead_code fixes post-implementation

- **Found during:** Task 2 GREEN, Step H verification sweep (`./pre-release-check.sh` step 15: `cargo clippy -p native-theme --all-targets -- -D warnings`)
- **Issue:** After the select_reader implementation, clippy caught:
  - 5 `clippy::needless_return` errors in select_reader + from_system_inner arms. The cfg-gated match arms make the `return`s syntactically necessary on some feature-combinations and unnecessary on others; a pure expression rewrite would require match arms that only compile under a union of cfg predicates.
  - 1 `dead_code` error on `fn assert_object_safe<T: ?Sized>(_: &T)` in `theme_reader_trait_exists_and_is_object_safe` — on cfg permutations that exclude every concrete reader (e.g. linux without kde feature) the function is only name-resolved but never called.
- **Fix:**
  - Added `#[allow(clippy::needless_return)] // Explicit returns clarify cfg-gated dispatch.` narrowly on `select_reader` and `from_system_inner` functions (not crate-wide; the surrounding code stays clippy-D-warnings clean).
  - Added `#[allow(dead_code)]` on the `assert_object_safe` inner fn in the test module.
- **Files modified:** native-theme/src/pipeline.rs (3 `#[allow(...)]` additions — 2 on functions, 1 on inner test helper).
- **Verification:** `cargo clippy -p native-theme --all-targets -- -D warnings` clean; `./pre-release-check.sh` green banner across all 5 workspace crates.
- **Committed in:** `66423ae` (Task 2 GREEN commit — single atomic commit per plan instructions).

---

**Total deviations:** 2 auto-fixed (1 blocking type-shape refinement, 1 lint regression fix).
**Impact on plan:** Both deviations were mandatory for plan completion. The tuple-return refinement preserves the plan's structural goal (collapse the cfg ladder) while honouring the dispatcher-decides-both invariant that fell out of the actual implementation. The clippy fixes are narrowly scoped `#[allow(...)]` annotations with inline justification; no scope creep into unrelated code.

## Issues Encountered

None beyond the two auto-fixed deviations above. The async-trait coercion compiled on first attempt thanks to the plan's pre-locked Option A strategy — no deviations from the three-option table needed.

## User Setup Required

None — this is a source-code refactor with no external service configuration. The `async-trait = "0.1"` Cargo.toml change resolves to the same 0.1.89 version that was already being compiled transitively via zbus; no `cargo update` required and `Cargo.lock` changes reduce to a single metadata line adding async-trait to native-theme's dependency array.

## Next Phase Readiness

- **Phase 94 complete.** All three plans (94-01 G6 border/font inheritance codegen, 94-02 G7 ResolutionContext, 94-03 G8 ThemeReader trait) landed. 55-95 coverage: 89 generated inheritance rules + 29 hand-written rules (G6), first-class `ResolutionContext` with `resolve_system()` shortcut (G7), and typed-reader-per-backend dispatch via `pub(crate) trait ThemeReader` (G8).
- **v0.5.7 readiness.** All known v0.5.7 API gaps per docs/todo_v0.5.7_gaps.md are closed. `./pre-release-check.sh` green banner; 1185+ tests passing; clippy -D warnings clean. Publish order per the script's final "Next steps" block: `cargo publish -p native-theme-derive` first (unblocks the workspace tarball verify), then the remaining four crates in dependency order.
- **Mock-reader substitution unblocked** (out of G8 scope but enabled by the trait being pub(crate) at same-crate visibility). Future test-only modules can declare `struct MockReader;` + `#[async_trait::async_trait] impl ThemeReader for MockReader` without touching production surface.
- **Trait extension pattern established.** Adding a new platform backend now requires:
  1. A new `pub(crate) struct NewReader;` + `#[async_trait::async_trait] impl crate::reader::ThemeReader for NewReader`.
  2. One new arm in `pipeline::select_reader`.
  No other dispatcher changes needed.

### Post-Plan Verification

- **Grep-count facts (verified 2026-04-20)**:
  - 0 matches for `fn from_kde\(|fn from_gnome|fn from_macos|fn from_windows|fn from_kde_with_portal` under native-theme/src/ (all 5 free functions deleted).
  - 5 matches for `impl .* ThemeReader for` under native-theme/src/ (KdeReader, GnomeReader, GnomePortalKdeReader, MacosReader, WindowsReader).
  - 6 matches for `^#\[async_trait::async_trait\]` under native-theme/src/ (1 on the trait in reader.rs + 5 on the impl blocks).
  - 0 matches for `crate::kde::from_kde\b|crate::gnome::from_gnome|crate::macos::from_macos|crate::windows::from_windows|crate::gnome::from_kde_with_portal` in live source (historical refs remain in docs/ and .planning/, not code).
  - `async-trait = "0.1"` direct dep present in native-theme/Cargo.toml.
- **Test matrix**: `cargo test -p native-theme --all-features --all-targets` 891 passed; `cargo test -p native-theme --no-default-features` 557 lib + 49 doctests passed; `cargo test -p native-theme --features "kde,portal"` 725 passed. Zero regressions.
- **Release gate**: `./pre-release-check.sh` final banner: `🎉 All pre-release checks passed successfully!` + `native-theme v0.5.7 is ready for release.`

## Self-Check

Files created verified:
- [x] `/home/tibi/Rust/native-theme/native-theme/src/reader.rs` — FOUND (new file, 46 LoC with module docs + trait).

Commits exist verified:
- [x] `ce10734` (RED) — FOUND in git log.
- [x] `66423ae` (GREEN) — FOUND in git log (HEAD~0).

## Self-Check: PASSED

---
*Phase: 94-close-remaining-v0-5-7-gaps-g6-g8-border-inheritance-codegen-resolutioncontext-themereader-trait*
*Completed: 2026-04-20*
