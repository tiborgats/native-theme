# Break up lib.rs into focused modules

Status: Not started
Date: 2026-04-09

---

## Problem

`native-theme/src/lib.rs` is 2,767 lines. It contains module declarations,
re-exports, a macro definition, an enum, 6 caching/detection subsystems,
the `SystemTheme` struct and its entire pipeline, icon loading dispatch,
platform diagnostics, and tests. This makes it hard to navigate and
violates single-responsibility: adding a new detection heuristic, a new
icon loader, or a new SystemTheme method all edit the same file.

### Current layout of lib.rs (by line range)

| Lines | Content | Logical group |
|-------|---------|---------------|
| 1-16 | Module doc, lints, ReadmeDoctests | Root |
| 17-85 | `impl_merge!` macro definition | Macro |
| 86-175 | Module declarations + `pub use` re-exports | Root |
| 176-229 | `LinuxDesktop` enum, `detect_linux_de()`, `xdg_current_desktop()` | Detection |
| 230-305 | `CACHED_IS_DARK`, `system_is_dark()`, `detect_is_dark()` | Detection |
| 306-350 | `run_gsettings_with_timeout()` | Detection (Linux helper) |
| 354-496 | `read_xft_dpi()`, `detect_physical_dpi()`, `parse_xrandr_dpi()` | Detection (DPI) |
| 498-553 | `xrandr_dpi_tests` module | Tests |
| 555-626 | `detect_system_font_dpi()`, `read_kde_force_font_dpi()` | Detection (DPI) |
| 628-740 | `detect_is_dark_inner()` (112 lines, multi-platform) | Detection |
| 742-850 | `CACHED_REDUCED_MOTION`, `prefers_reduced_motion()`, `detect_reduced_motion_inner()` | Detection |
| 852-1031 | `SystemTheme` struct + impl (active, pick, with_overlay, from_system, from_system_async) | Pipeline |
| 1033-1123 | `run_pipeline()` | Pipeline |
| 1125-1176 | `linux_preset_for_de()`, `platform_preset_name()` | Platform metadata |
| 1178-1298 | `diagnose_platform_support()` | Platform metadata |
| 1300-1468 | `reader_is_dark()`, `from_linux()`, `from_system_inner()`, `from_system_async_inner()` | Pipeline |
| 1470-1741 | `load_icon()`, `load_icon_from_theme()`, `is_freedesktop_theme_available()`, `load_system_icon_by_name()`, `loading_indicator()`, `load_custom_icon()` | Icon loading |
| 1743-end | `ENV_MUTEX`, `dispatch_tests`, `icon_tests`, `detect_tests` | Tests |

---

## Options

### Option A: Keep monolith, add section comments

Leave everything in lib.rs. Add `// ── Detection ───` region markers
and fold-friendly comments for editor navigation.

**Pros:**
- Zero risk. No code moves, no import changes.
- No macro scoping concerns.

**Cons:**
- Doesn't fix single-responsibility violation — every change still edits lib.rs.
- 2,767 lines is hard to navigate even with markers.
- Tests for unrelated subsystems (DPI detection, icon loading) are interleaved.

### Option B: Minimal 3-module split

Extract only the two largest logical blocks:

1. `src/detect.rs` (~500 lines) — all OS detection + caching + DPI
2. `src/system_theme.rs` (~450 lines) — SystemTheme + pipeline + platform routing

Keep icon loading, platform_preset_name, and diagnose_platform_support
in lib.rs (~470 lines remaining).

**Pros:**
- Addresses the two biggest contributors to file size.
- Fewer cross-module dependencies to manage.
- lib.rs drops from 2,767 → ~470 lines (icon loading + re-exports + metadata).

**Cons:**
- Still leaves icon dispatch (270 lines) and platform diagnostics in lib.rs.
- Doesn't fully separate concerns — "half refactored" state.

### Option C: Full 4-module split (recommended)

Extract four focused modules:

1. **`src/detect.rs`** (~500 lines) — OS state queries without constructing a theme:
   - `LinuxDesktop` enum + `detect_linux_de()` + `xdg_current_desktop()`
   - `CACHED_IS_DARK` + `system_is_dark()` + `detect_is_dark()` + `detect_is_dark_inner()`
   - `CACHED_REDUCED_MOTION` + `prefers_reduced_motion()` + `detect_reduced_motion_inner()`
   - `run_gsettings_with_timeout()`, `read_xft_dpi()`, `detect_physical_dpi()`,
     `parse_xrandr_dpi()`, `detect_system_font_dpi()`, `read_kde_force_font_dpi()`
   - `invalidate_caches()`
   - All detection tests (xrandr_dpi_tests, detect_tests)

2. **`src/system_theme.rs`** (~550 lines) — SystemTheme struct and OS-first pipeline:
   - `SystemTheme` struct + impl (active, pick, with_overlay, from_system, from_system_async)
   - `run_pipeline()`, `from_system_inner()`, `from_linux()`, `from_system_async_inner()`
   - `reader_is_dark()`, `linux_preset_for_de()`
   - `platform_preset_name()`, `diagnose_platform_support()`

   `platform_preset_name` and `diagnose_platform_support` go here (not
   their own module) because they share `linux_preset_for_de()` and
   `detect_linux_de()` — a 2-function module isn't worth the overhead.

3. **`src/icon_loader.rs`** (~270 lines) — icon loading dispatch:
   - `load_icon()`, `load_icon_from_theme()`, `load_system_icon_by_name()`
   - `load_custom_icon()`, `loading_indicator()`
   - `is_freedesktop_theme_available()`
   - Icon dispatch tests

4. **`src/macros.rs`** (~70 lines) — `impl_merge!` macro definition.
   Declared as `#[macro_use] mod macros;` before `mod model` in lib.rs
   (Rust requires `macro_rules!` to be defined before use-site modules;
   `#[macro_use]` makes it crate-visible).

**lib.rs after refactor** (~200 lines): module doc, lints, ReadmeDoctests,
module declarations, `pub use` re-exports, `pub type Result<T>`, `ENV_MUTEX`.

**Pros:**
- Each module has a single responsibility and is independently navigable.
- lib.rs drops from 2,767 → ~200 lines (pure root).
- Tests colocate with the code they exercise.
- New detection heuristics, SystemTheme methods, or icon loaders each
  touch one file.

**Cons:**
- More cross-module `pub(crate)` imports to manage.
- Must handle `impl_merge!` macro scoping carefully.
- Small risk of import cycles (mitigated by crate-internal visibility).

### Why Option C

Option A doesn't solve the problem. Option B solves 70% of it with the
same effort as Option C (the cross-module wiring is the hard part either
way, and it's the same for 2 or 4 modules). Option C finishes the job.

---

## Cross-module crate-internal access

Several functions are `pub(crate)` and called across the new modules:

- `run_gsettings_with_timeout` — called by `detect.rs` and `gnome/mod.rs`
- `xdg_current_desktop` — called by `detect.rs` and `system_theme.rs`
- `detect_system_font_dpi` — called by `resolve/mod.rs`
- `linux_preset_for_de` — called by `system_theme.rs`

These stay `pub(crate)` and are imported with `use crate::detect::...`.

## Public API stability

The public API (function signatures, types, re-exports) must not change.
All items currently exported from `lib.rs` remain exported from `lib.rs`
via `pub use` — the consumer never sees the internal module path.

---

## Risk

Low. This is a pure refactor with no behavioral changes. Every function
signature, visibility, and return type stays identical. The test suite
validates the refactor.

## Verification

- `cargo test --features native` passes
- `cargo clippy --features native -- -Dwarnings` passes
- `cargo doc --no-deps` passes (re-exports preserve doc locations)
- Public API diff is empty (compare `cargo public-api` before/after, or
  manual review of re-exports)
