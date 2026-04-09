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

## Proposed module split

### 1. `src/detect.rs` (~500 lines)

Everything related to querying OS state without constructing a full theme:

- `LinuxDesktop` enum + `detect_linux_de()` + `xdg_current_desktop()`
- `CACHED_IS_DARK` + `system_is_dark()` + `detect_is_dark()` + `detect_is_dark_inner()`
- `CACHED_REDUCED_MOTION` + `prefers_reduced_motion()` + `detect_reduced_motion()` + `detect_reduced_motion_inner()`
- `run_gsettings_with_timeout()` (used by detect + gnome reader)
- `read_xft_dpi()`, `detect_physical_dpi()`, `parse_xrandr_dpi()`, `detect_system_font_dpi()`, `read_kde_force_font_dpi()`
- `invalidate_caches()`
- All associated tests (`xrandr_dpi_tests`, `detect_tests`)

Public items: `LinuxDesktop`, `detect_linux_de`, `system_is_dark`, `detect_is_dark`,
`prefers_reduced_motion`, `detect_reduced_motion`, `invalidate_caches`.

Crate-internal items: `run_gsettings_with_timeout`, `xdg_current_desktop`,
`detect_system_font_dpi`, `read_xft_dpi`, `detect_physical_dpi`.

### 2. `src/system_theme.rs` (~450 lines)

The SystemTheme struct and the OS-first pipeline:

- `SystemTheme` struct definition
- `SystemTheme::from_system()`, `from_system_async()`, `active()`, `pick()`, `with_overlay()`, `with_overlay_toml()`
- `run_pipeline()` (private)
- `from_system_inner()`, `from_linux()`, `from_system_async_inner()` (private)
- `reader_is_dark()` (private)
- `linux_preset_for_de()` (crate-internal, also used by platform_preset_name)

### 3. `src/icon_loader.rs` (~270 lines)

Icon loading dispatch (the functions that combine platform loaders + bundled icons):

- `load_icon()`
- `load_icon_from_theme()`
- `load_system_icon_by_name()`
- `load_custom_icon()`
- `loading_indicator()`
- `is_freedesktop_theme_available()`
- Associated tests (`icon_tests`)

### 4. `src/platform_info.rs` (~120 lines)

Platform metadata queries that don't construct themes:

- `platform_preset_name()`
- `diagnose_platform_support()`

### 5. `src/macros.rs` (~70 lines)

The `impl_merge!` macro definition. Must be declared before any module that
uses it (Rust macro scoping requires `#[macro_use] mod macros;` or
`macro_rules!` before the using modules).

### 6. `src/lib.rs` after refactor (~200 lines)

What remains in lib.rs:

- Module doc, lints, ReadmeDoctests
- Module declarations (detect, system_theme, icon_loader, platform_info, macros, model, presets, resolve, kde, gnome, windows, macos, freedesktop, spinners, etc.)
- `pub use` re-exports (unchanged)
- `pub type Result<T>` alias
- `ENV_MUTEX` (test-only)

---

## Constraints

### Macro scoping

`impl_merge!` is used in `model/mod.rs`, `model/widgets/mod.rs`,
`model/defaults.rs`, `model/font.rs`. Rust `macro_rules!` macros must
be defined before the modules that use them. Options:

- **Option A:** `#[macro_use] mod macros;` at the top of lib.rs before
  `mod model`. The macro is file-scoped to the crate.
- **Option B:** Move `impl_merge!` into its own `src/macros.rs` and
  include via `#[macro_use]`. Same effect, cleaner file.
- **Option C (proc-macro):** Replace `impl_merge!` with a derive macro
  in `native-theme-build`. Overkill for this task — save for a separate
  todo if desired.

Recommendation: **Option B** — simplest, no behavior change.

### Cross-module crate-internal access

Several functions are `pub(crate)` and called across modules:

- `run_gsettings_with_timeout` — called by `detect.rs` and `gnome/mod.rs`
- `xdg_current_desktop` — called by `detect.rs` and `system_theme.rs`
- `detect_system_font_dpi` — called by `resolve/mod.rs`
- `linux_preset_for_de` — called by `system_theme.rs` and `platform_info.rs`

These stay `pub(crate)` and are imported with `use crate::detect::...`.

### Public API stability

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
