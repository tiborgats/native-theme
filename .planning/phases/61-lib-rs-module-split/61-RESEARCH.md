# Phase 61: lib.rs Module Split - Research

**Researched:** 2026-04-09
**Domain:** Rust module extraction / crate reorganization
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- Tests move with their code: xrandr_dpi_tests and reduced_motion_tests to detect.rs, load_icon_tests/load_system_icon_by_name_tests/load_custom_icon_tests/loading_indicator_tests/spinner_rasterize_tests to icons.rs, dispatch_tests to pipeline.rs
- ENV_MUTEX goes in a dedicated test_util.rs (cfg(test)) -- shared test infrastructure separate from production code
- Flat files: detect.rs, pipeline.rs, icons.rs alongside lib.rs (not directory modules)
- test_util.rs for shared test infrastructure (cfg(test))
- pub(crate) items (run_gsettings_with_timeout, read_xft_dpi, detect_physical_dpi, etc.) become module-private with narrower accessor functions -- cleaner module boundaries

### Claude's Discretion
- system_theme_tests placement -- analyze test contents and place where they couple best (lib.rs vs pipeline.rs, or split)
- overlay_tests placement -- analyze dependencies and place optimally
- Whether platform glue (from_linux, linux_preset_for_de, reader_is_dark) stays in pipeline.rs or gets a separate platform.rs -- judge based on actual code coupling
- diagnose_platform_support() placement -- where it couples best
- LinuxDesktop enum and detect_linux_de() placement -- based on actual usage patterns across modules
- Re-export strategy -- whether modules are pub or private, and whether to use glob re-exports or selective -- pick what preserves the current public API with clean internal organization

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| STRUCT-01 | lib.rs broken into detect.rs, system_theme.rs, icon_loader.rs, macros.rs with lib.rs as pure root (~200 lines) | Full codebase analysis below maps every function, struct, and test to its target module. Note: CONTEXT.md uses different module names (detect.rs, pipeline.rs, icons.rs) which override the requirement's naming. |
</phase_requirements>

## Summary

The lib.rs file is exactly 2,767 lines containing 6 distinct functional domains: (1) detection logic (dark mode, reduced motion, DPI, gsettings), (2) pipeline orchestration (run_pipeline, from_linux, from_system_inner, from_system_async_inner), (3) icon dispatch (load_icon, load_system_icon_by_name, load_custom_icon, loading_indicator, load_icon_from_theme, is_freedesktop_theme_available), (4) the SystemTheme struct and its methods, (5) the impl_merge! macro, and (6) shared types (LinuxDesktop, Result alias, re-exports). The split is mechanically straightforward because the code has clear domain boundaries with well-defined cross-cutting dependencies.

The primary complexity lies in cross-module references. Five other modules in the crate call `pub(crate)` functions currently in lib.rs: `gnome` calls `run_gsettings_with_timeout`, `read_xft_dpi`, `detect_physical_dpi`; `kde` calls `read_xft_dpi`, `detect_physical_dpi`; `resolve` calls `detect_system_font_dpi`; `presets` and `resolve/inheritance` and `model/icons` call `detect_linux_de`, `xdg_current_desktop`, and `LinuxDesktop`. After the split, these callers must update their `crate::` paths to `crate::detect::` (or whatever accessor functions are provided).

There is one pre-existing test failure (`gnome::tests::build_gnome_variant_normal_contrast_no_flag`) that is unrelated to this phase. All 668 other tests pass. The split must preserve this exact pass/fail state.

**Primary recommendation:** Split lib.rs into detect.rs (~600 lines prod + ~90 lines test), pipeline.rs (~530 lines prod + ~200 lines test), icons.rs (~240 lines prod + ~350 lines test), and test_util.rs (~5 lines), leaving lib.rs as ~250 lines (macro + struct + module declarations + re-exports). Update all `crate::` references in gnome, kde, resolve, presets, and model/icons to use the new module paths.

## Standard Stack

This phase uses no external libraries. It is a pure Rust module reorganization using only language features.

### Core
| Tool | Version | Purpose | Why Standard |
|------|---------|---------|--------------|
| Rust | 1.94+ / Edition 2024 | Module system, cfg attributes | Project's MSRV [VERIFIED: Cargo.toml] |

### Relevant Language Features
| Feature | Usage | Notes |
|---------|-------|-------|
| `mod` declarations | Flat file modules alongside lib.rs | Edition 2024 uses the 2021 module resolution (file.rs preferred over file/mod.rs) [VERIFIED: Rust reference] |
| `pub(crate)` | Cross-module access within the crate | Currently used for detection helpers; will become module-private with accessor functions per CONTEXT.md decision |
| `cfg(test)` / `cfg(target_os)` | Conditional compilation | Heavy use throughout; must be preserved exactly during extraction |
| `pub use` re-exports | Public API preservation | lib.rs must re-export everything that is currently `pub` at crate level |

## Architecture Patterns

### Current Structure (lib.rs monolith)
```
native-theme/src/
  lib.rs          # 2,767 lines -- everything
  color.rs
  error.rs
  freedesktop.rs
  gnome/mod.rs
  kde/mod.rs
  macos.rs
  model/          # model types, icons submodule
  presets.rs
  rasterize.rs
  resolve/        # inheritance, validation
  sficons.rs
  spinners.rs
  windows.rs
  winicons.rs
```

### Target Structure (after split)
```
native-theme/src/
  lib.rs          # ~250 lines: impl_merge! macro, SystemTheme struct + impl,
                  #   module declarations, re-exports, shared types
  detect.rs       # ~600 lines: system_is_dark, detect_is_dark, prefers_reduced_motion,
                  #   detect_reduced_motion, gsettings helpers, xrandr/DPI detection,
                  #   detect_system_font_dpi, OnceLock caches, invalidate_caches
  pipeline.rs     # ~530 lines: run_pipeline, from_linux, from_system_inner,
                  #   from_system_async_inner, platform_preset_name,
                  #   linux_preset_for_de, reader_is_dark, diagnose_platform_support
  icons.rs        # ~240 lines: load_icon, load_icon_from_theme, load_system_icon_by_name,
                  #   load_custom_icon, loading_indicator, is_freedesktop_theme_available
  test_util.rs    # ~5 lines: ENV_MUTEX (cfg(test) only)
  [rest unchanged]
```

### Pattern 1: Module-Private + Accessor Functions
**What:** Functions that are currently `pub(crate)` in lib.rs become private within their new module, with explicit accessor functions for cross-module callers.
**When to use:** For run_gsettings_with_timeout, read_xft_dpi, detect_physical_dpi, detect_system_font_dpi, xdg_current_desktop. [LOCKED DECISION]
**Example:**
```rust
// In detect.rs:
// Private implementation
fn run_gsettings_with_timeout(args: &[&str]) -> Option<String> { ... }

// Accessor -- narrower interface for gnome module
pub(crate) fn gsettings_get(schema: &str, key: &str) -> Option<String> {
    run_gsettings_with_timeout(&["get", schema, key])
}

// Accessor -- expose DPI reading for gnome/kde
pub(crate) fn xft_dpi() -> Option<f32> { read_xft_dpi() }
pub(crate) fn physical_dpi() -> Option<f32> { detect_physical_dpi() }
pub(crate) fn system_font_dpi() -> f32 { detect_system_font_dpi() }
```

### Pattern 2: Re-export Preservation
**What:** lib.rs re-exports all public items from new modules so the crate's public API is unchanged.
**When to use:** Every `pub fn` and `pub type` that moves out of lib.rs.
**Example:**
```rust
// In lib.rs:
mod detect;
mod pipeline;
mod icons;
#[cfg(test)]
mod test_util;

// Re-export public API from detect
pub use detect::{
    system_is_dark, detect_is_dark, prefers_reduced_motion, detect_reduced_motion,
    invalidate_caches,
};
// Re-export public API from pipeline
pub use pipeline::{platform_preset_name, diagnose_platform_support};
// Re-export public API from icons
pub use icons::{
    load_icon, load_icon_from_theme, load_system_icon_by_name,
    load_custom_icon, loading_indicator, is_freedesktop_theme_available,
};
```

### Pattern 3: cfg Attribute Preservation
**What:** All `#[cfg(target_os = ...)]` and `#[cfg(feature = ...)]` attributes must be copied verbatim with the code they protect.
**When to use:** Every moved item.
**Critical items:**
- `LinuxDesktop` enum: `#[cfg(target_os = "linux")]`
- `detect_linux_de()`: `#[cfg(target_os = "linux")]`
- `run_gsettings_with_timeout()`: `#[cfg(target_os = "linux")]`
- `read_xft_dpi()`: `#[cfg(all(target_os = "linux", any(feature = "kde", feature = "portal")))]`
- `detect_physical_dpi()`: `#[cfg(all(target_os = "linux", any(feature = "kde", feature = "portal")))]`
- `from_system_async_inner()`: `#[cfg(target_os = "linux")]`
- Icon functions: various `#[cfg(feature = "...")]` gates

## Complete Item-to-Module Mapping

### detect.rs (production code, lines ~180-850 of current lib.rs)

| Item | Lines | Visibility | cfg | External Callers |
|------|-------|------------|-----|------------------|
| `LinuxDesktop` enum | 180-198 | pub | linux | presets.rs:137, resolve/inheritance.rs:101, model/icons.rs:573-584 |
| `xdg_current_desktop()` | 204-206 | pub(crate) | linux | presets.rs:137, resolve/inheritance.rs:101, model/icons.rs:573 |
| `detect_linux_de()` | 215-229 | pub | linux | presets.rs:137, resolve/inheritance.rs:101, model/icons.rs:573 |
| `CACHED_IS_DARK` static | 231 | private | none | none |
| `system_is_dark()` | 263-274 | pub | none | pipeline.rs (from_linux, from_system_async_inner) |
| `invalidate_caches()` | 284-292 | pub | none | none (called by consumers) |
| `detect_is_dark()` | 302-304 | pub | none | none (called by consumers) |
| `run_gsettings_with_timeout()` | 315-352 | pub(crate) | linux | gnome/mod.rs:126 |
| `read_xft_dpi()` | 359-400 | pub(crate) | linux+kde/portal | gnome/mod.rs:142, kde/mod.rs:92 |
| `detect_physical_dpi()` | 412-443 | pub(crate) | linux+kde/portal | gnome/mod.rs:146, kde/mod.rs:97 |
| `parse_xrandr_dpi()` | 454-496 | private | linux+kde/portal | none (internal to detect) |
| `detect_system_font_dpi()` | 567-597 | pub(crate) | none | resolve/mod.rs:102 |
| `read_kde_force_font_dpi()` | 605-626 | private | linux+kde | none (internal to detect) |
| `detect_is_dark_inner()` | 632-740 | private | none | none (internal to detect) |
| `CACHED_REDUCED_MOTION` static | 742 | private | none | none |
| `prefers_reduced_motion()` | 775-786 | pub | none | none (called by consumers) |
| `detect_reduced_motion()` | 796-798 | pub | none | none (called by consumers) |
| `detect_reduced_motion_inner()` | 804-850 | private | none | none (internal to detect) |

**detect.rs tests (move with code):**
- `xrandr_dpi_tests` (lines 498-553, 7 tests)
- `reduced_motion_tests` (lines 2600-2637, 4 tests)

### pipeline.rs (production code, lines ~1033-1468 of current lib.rs)

| Item | Lines | Visibility | cfg | External Callers |
|------|-------|------------|-----|------------------|
| `run_pipeline()` | 1039-1123 | private (crate) | none | from_linux, from_system_inner, from_system_async_inner |
| `linux_preset_for_de()` | 1135-1139 | private | linux | from_linux, from_system_async_inner, platform_preset_name |
| `platform_preset_name()` | 1159-1176 | pub | none | none (called by consumers) |
| `diagnose_platform_support()` | 1202-1298 | pub | none | none (called by consumers) |
| `reader_is_dark()` | 1308-1310 | private | none | from_system_inner |
| `from_linux()` | 1318-1349 | private | linux | from_system_inner |
| `from_system_inner()` | 1351-1393 | private | none | SystemTheme::from_system(), SystemTheme::from_system_async() (non-linux) |
| `from_system_async_inner()` | 1396-1468 | private | linux | SystemTheme::from_system_async() (linux) |

**Pipeline dependencies on detect.rs:**
- `system_is_dark()` -- called in from_linux() and from_system_async_inner()
- `detect_linux_de()` -- called in from_linux(), from_system_async_inner(), platform_preset_name(), diagnose_platform_support()
- `xdg_current_desktop()` -- called in from_linux(), from_system_async_inner(), platform_preset_name(), diagnose_platform_support()
- `LinuxDesktop` -- type used throughout

**Pipeline dependencies on lib.rs (SystemTheme):**
- `SystemTheme` struct -- constructed by run_pipeline()
- `ThemeSpec`, `ThemeVariant` -- used by run_pipeline(), from_linux(), etc.

**pipeline.rs tests (move with code):**
- `dispatch_tests` (lines 1749-1950, 16 tests) -- tests detect_linux_de pure function + from_linux/from_system integration

### icons.rs (production code, lines ~1470-1741 of current lib.rs)

| Item | Lines | Visibility | cfg | External Callers |
|------|-------|------------|-----|------------------|
| `load_icon()` | 1504-1528 | pub | none | none (called by consumers) |
| `load_icon_from_theme()` | 1556-1571 | pub | none | none (called by consumers) |
| `is_freedesktop_theme_available()` | 1581-1614 | pub | none | none (called by consumers) |
| `load_system_icon_by_name()` | 1640-1666 | pub | none | load_custom_icon (internal) |
| `loading_indicator()` | 1688-1701 | pub | none | none (called by consumers) |
| `load_custom_icon()` | 1726-1741 | pub | none | none (called by consumers) |

**icons.rs has no external callers within the crate** -- all icon functions are leaf public API. They depend on:
- `freedesktop::*` (cfg-gated platform modules)
- `sficons::*` (cfg-gated platform modules)
- `winicons::*` (cfg-gated platform modules)
- `spinners::*` (private module)
- `model::icons::{icon_name, system_icon_theme, bundled_icon_by_name, bundled_icon_svg}` (re-exports from model)
- `IconRole`, `IconSet`, `IconData`, `IconProvider`, `AnimatedIcon` (types from model)

**icons.rs tests (move with code):**
- `load_icon_tests` (lines 1952-2037, 7 tests)
- `load_system_icon_by_name_tests` (lines 2039-2082, 4 tests)
- `load_custom_icon_tests` (lines 2084-2186, 6 tests)
- `loading_indicator_tests` (lines 2188-2254, 5 tests)
- `spinner_rasterize_tests` (lines 2256-2283, 1 test)

### lib.rs (remaining, ~250 lines)

| Item | Lines | Notes |
|------|-------|-------|
| Crate-level doc comment + lints | 1-12 | Stays |
| ReadmeDoctests | 13-15 | Stays |
| `impl_merge!` macro | 17-85 | Stays (must be defined before modules that use it) |
| Module declarations | 87-152 | Stays + add detect, pipeline, icons, test_util |
| Re-exports | 110-174 | Stays + add re-exports from detect, pipeline, icons |
| `Result` type alias | 177 | Stays |
| `SystemTheme` struct | 859-876 | Stays in lib.rs |
| `impl SystemTheme` | 878-1031 | Stays in lib.rs (methods call pipeline functions) |

### test_util.rs (~5 lines, cfg(test) only)

```rust
/// Mutex to serialize tests that manipulate environment variables.
/// Env vars are process-global state, so tests that call set_var/remove_var
/// must hold this lock to avoid races with parallel test execution.
pub(crate) static ENV_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());
```

Used by: dispatch_tests (pipeline.rs), system_theme_tests (lib.rs), kde tests (kde/mod.rs).

## Discretion Recommendations

### system_theme_tests placement: STAY IN lib.rs

**Rationale:** system_theme_tests test `SystemTheme::active()`, `SystemTheme::pick()`, `run_pipeline()`, `reader_is_dark()`, `platform_preset_name()`, and `SystemTheme::with_overlay()`. The tests directly construct `SystemTheme` structs and call `run_pipeline()`. Since `SystemTheme` stays in lib.rs and `run_pipeline` moves to pipeline.rs, these tests bridge both modules. Keeping them in lib.rs is the simplest approach -- they test the public API surface that lib.rs exports.

However, the tests that specifically test `run_pipeline()` (test_run_pipeline_produces_both_variants, test_run_pipeline_reader_values_win, test_run_pipeline_single_variant, test_run_pipeline_with_preset_as_reader, test_run_pipeline_propagates_font_dpi_to_inactive_variant, test_run_pipeline_inactive_variant_from_full_preset) could move to pipeline.rs if run_pipeline is made `pub(crate)` in pipeline.rs (which it needs to be anyway for SystemTheme::from_system to call it).

**Recommendation:** Split system_theme_tests:
- Tests for `active()`, `pick()`, `platform_preset_name()` (4 tests) -- stay in lib.rs
- Tests for `run_pipeline()`, `reader_is_dark()` (8 tests) -- move to pipeline.rs
- Tests for `with_overlay()`/`with_overlay_toml()` in overlay_tests -- see below

### overlay_tests placement: STAY IN lib.rs

**Rationale:** overlay_tests test `SystemTheme::with_overlay()` and `SystemTheme::with_overlay_toml()`. These methods are on `SystemTheme` which stays in lib.rs. The helper `default_system_theme()` calls `run_pipeline()` but that is just test setup. The assertions are against `SystemTheme` fields. Keep in lib.rs alongside the `SystemTheme` impl.

### Platform glue placement: pipeline.rs (no separate platform.rs)

**Rationale:** `from_linux()`, `linux_preset_for_de()`, `reader_is_dark()`, `from_system_inner()`, `from_system_async_inner()`, `platform_preset_name()`, `diagnose_platform_support()` form a tightly coupled call graph:
- `from_system_inner()` calls `from_linux()` (Linux), or directly calls `run_pipeline()` (macOS/Windows)
- `from_linux()` calls `linux_preset_for_de()`, `system_is_dark()`, `run_pipeline()`
- `from_system_async_inner()` calls `linux_preset_for_de()`, `system_is_dark()`, `run_pipeline()`
- `platform_preset_name()` calls `linux_preset_for_de()`, `detect_linux_de()`
- `diagnose_platform_support()` calls `detect_linux_de()`

All 7 functions are in the same call chain. Splitting to a separate platform.rs would create a 2-file pipeline with no clear benefit. Keep all in pipeline.rs.

### diagnose_platform_support() placement: pipeline.rs

**Rationale:** It uses `detect_linux_de()` (from detect.rs, accessed as `crate::detect::detect_linux_de`) and checks platform feature flags. It is diagnostics for the pipeline. Fits pipeline.rs conceptually.

### LinuxDesktop enum and detect_linux_de() placement: detect.rs

**Rationale:** These are called from 5 locations across the crate:
- `presets.rs:137` -- `crate::detect_linux_de(&crate::xdg_current_desktop()) == crate::LinuxDesktop::Kde`
- `resolve/inheritance.rs:101` -- same pattern
- `model/icons.rs:573-584` -- matches on `crate::LinuxDesktop::*`
- `pipeline.rs` (after move) -- `from_linux`, `from_system_async_inner`, `platform_preset_name`, `diagnose_platform_support`
- `detect.rs` (after move) -- used internally

The enum and function are fundamentally "detection" -- they detect what desktop environment is running. They belong in detect.rs. After the move, all callers update from `crate::LinuxDesktop` to `crate::detect::LinuxDesktop` (if detect is private) or just `crate::LinuxDesktop` (if re-exported from lib.rs). Since `LinuxDesktop` is currently `pub` in the crate API, it must be re-exported from lib.rs.

### Re-export strategy: Private modules with selective re-exports

**Rationale:** The modules (detect, pipeline, icons) are implementation details. External consumers use the crate-level API (`native_theme::system_is_dark()`, not `native_theme::detect::system_is_dark()`). Making modules private (`mod detect;` not `pub mod detect;`) preserves the flat public API.

Use selective re-exports (not glob) for clarity:
```rust
mod detect;
mod pipeline;
mod icons;

pub use detect::{system_is_dark, detect_is_dark, prefers_reduced_motion, ...};
```

Internal crate modules (`gnome`, `kde`, `resolve`, etc.) access via `crate::detect::function_name()`.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Module re-export verification | Manual checking of public API | `cargo doc --no-deps` + grep for exported items | Compiler catches missing re-exports via "unresolved import" errors |
| Cross-module path updates | Manual search-and-replace | `cargo check` on all feature combinations | Compiler will report every broken `crate::` path |
| Test discovery | Manual list of test functions | `cargo test -- --list` | Lists all test functions, can diff before/after |

**Key insight:** The Rust compiler is the best verification tool for this refactor. A successful `cargo check` across all feature combinations proves correctness. Use `cargo test -- --list 2>/dev/null | wc -l` before and after to verify no tests were lost.

## Common Pitfalls

### Pitfall 1: Missing cfg Attributes After Move
**What goes wrong:** A function is moved without its `#[cfg(target_os = "linux")]` attribute. On macOS/Windows CI, the function either doesn't compile or includes Linux-only code.
**Why it happens:** cfg attributes are often on the line above the function, easy to miss when cutting code.
**How to avoid:** Copy the complete attribute stack (doc comments + cfg + allow + fn signature) as a unit. After moving, verify each function's cfg matches the original.
**Warning signs:** `cargo check` succeeds on Linux but fails on other platforms.

### Pitfall 2: Broken crate:: Paths in Other Modules
**What goes wrong:** `gnome/mod.rs` still says `crate::run_gsettings_with_timeout()` but the function moved to `crate::detect`.
**Why it happens:** After moving code out of lib.rs, items are no longer at the crate root.
**How to avoid:** After each module extraction, run `cargo check` and fix all path errors. The 5 external callers are documented in the mapping table above.
**Warning signs:** Compiler error "cannot find function `run_gsettings_with_timeout` in crate root".

### Pitfall 3: Test Module Visibility
**What goes wrong:** A test module in `detect.rs` tries to call `parse_xrandr_dpi()` which is private to detect.rs -- but `#[cfg(test)]` modules inside the same file can access private items.
**Why it happens:** Misunderstanding Rust's test visibility rules. `mod xrandr_dpi_tests` inside `detect.rs` CAN access private `parse_xrandr_dpi()` because it's a child module of the same file.
**How to avoid:** Tests that access private functions MUST be in the same file as the function. This is already the plan (xrandr_dpi_tests moves with parse_xrandr_dpi to detect.rs).
**Warning signs:** No warning signs -- this works correctly if tests move with their code.

### Pitfall 4: ENV_MUTEX Accessibility Across Modules
**What goes wrong:** After moving ENV_MUTEX to test_util.rs, test modules in pipeline.rs or kde/mod.rs can't find it.
**Why it happens:** test_util.rs is declared as `#[cfg(test)] mod test_util;` in lib.rs, so it's only visible under test compilation. The mutex must be `pub(crate)` for other modules' test code to access it.
**How to avoid:** Declare `pub(crate) static ENV_MUTEX` in test_util.rs, declare `#[cfg(test)] mod test_util;` in lib.rs. Access as `crate::test_util::ENV_MUTEX` from all test modules.
**Warning signs:** "cannot find value `ENV_MUTEX` in crate `crate`" during `cargo test`.

### Pitfall 5: impl_merge! Macro Ordering
**What goes wrong:** The `impl_merge!` macro is defined in lib.rs and used by types in `model/` which are declared after the macro. Moving the macro to a separate file could break this ordering.
**Why it happens:** Rust macros must be defined before they're used in the same crate (unless using proc macros or macro re-exports).
**How to avoid:** Keep `impl_merge!` in lib.rs, declared before any `mod` statements that use types invoking it. The CONTEXT.md decision to keep it in lib.rs is correct.
**Warning signs:** "cannot find macro `impl_merge!`" errors.

### Pitfall 6: SystemTheme::from_system() Calling Pipeline Functions
**What goes wrong:** `SystemTheme::from_system()` calls `from_system_inner()` which moves to pipeline.rs. The impl block stays in lib.rs.
**How to avoid:** `from_system_inner()` must be `pub(crate)` in pipeline.rs. The `impl SystemTheme` block in lib.rs calls `crate::pipeline::from_system_inner()` (or `pipeline::from_system_inner()` since pipeline is a child module).
**Same for:** `from_system_async_inner()` -- must be `pub(crate)` in pipeline.rs.

### Pitfall 7: Losing the #[deny(unsafe_code)] Crate Attribute
**What goes wrong:** New files don't inherit `#![deny(unsafe_code)]` from lib.rs.
**Why it happens:** `#![...]` attributes (crate-level) only apply to lib.rs. But `#[deny(unsafe_code)]` in lib.rs applies to the entire crate including submodules.
**How to avoid:** No action needed -- `#![deny(unsafe_code)]` in lib.rs applies crate-wide. The test code that uses `#[allow(unsafe_code)]` for `std::env::set_var` will continue to work because the `#[allow]` is on individual test functions.

### Pitfall 8: Feature Combination Verification
**What goes wrong:** The split works with all features enabled but breaks with minimal feature sets.
**Why it happens:** cfg-gated code may have different dependencies depending on feature flags.
**How to avoid:** Test with multiple feature combinations:
```bash
cargo check --package native-theme                                    # no features
cargo check --package native-theme --features kde                     # KDE only
cargo check --package native-theme --features portal-tokio            # portal only
cargo check --package native-theme --features "kde,portal-tokio"      # both
cargo check --package native-theme --all-features                     # everything
cargo test --package native-theme --features "kde,portal-tokio,material-icons,lucide-icons,svg-rasterize,system-icons"
```

## Code Examples

### detect.rs Module Structure
```rust
// Source: Derived from codebase analysis [VERIFIED: native-theme/src/lib.rs]

//! OS detection: dark mode, reduced motion, DPI, desktop environment.

// === Types ===

/// Desktop environments recognized on Linux.
#[cfg(target_os = "linux")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinuxDesktop { /* ... */ }

// === Public API ===

#[cfg(target_os = "linux")]
pub fn detect_linux_de(xdg_current_desktop: &str) -> LinuxDesktop { /* ... */ }
pub fn system_is_dark() -> bool { /* ... */ }
pub fn detect_is_dark() -> bool { /* ... */ }
pub fn prefers_reduced_motion() -> bool { /* ... */ }
pub fn detect_reduced_motion() -> bool { /* ... */ }
pub fn invalidate_caches() { /* ... */ }

// === Crate-internal accessors ===

#[cfg(target_os = "linux")]
pub(crate) fn xdg_current_desktop() -> String { /* ... */ }

#[cfg(target_os = "linux")]
pub(crate) fn gsettings_get(schema: &str, key: &str) -> Option<String> {
    run_gsettings_with_timeout(&["get", schema, key])
}

#[cfg(all(target_os = "linux", any(feature = "kde", feature = "portal")))]
pub(crate) fn xft_dpi() -> Option<f32> { read_xft_dpi() }

#[cfg(all(target_os = "linux", any(feature = "kde", feature = "portal")))]
pub(crate) fn physical_dpi() -> Option<f32> { detect_physical_dpi() }

pub(crate) fn system_font_dpi() -> f32 { detect_system_font_dpi() }

// === Private implementation ===

static CACHED_IS_DARK: std::sync::RwLock<Option<bool>> = std::sync::RwLock::new(None);
static CACHED_REDUCED_MOTION: std::sync::RwLock<Option<bool>> = std::sync::RwLock::new(None);

#[cfg(target_os = "linux")]
fn run_gsettings_with_timeout(args: &[&str]) -> Option<String> { /* ... */ }
// ... etc

// === Tests ===

#[cfg(all(test, target_os = "linux", any(feature = "kde", feature = "portal")))]
mod xrandr_dpi_tests { /* ... */ }

#[cfg(test)]
mod reduced_motion_tests { /* ... */ }
```

### pipeline.rs Module Structure
```rust
// Source: Derived from codebase analysis [VERIFIED: native-theme/src/lib.rs]

//! Theme pipeline: reader -> preset merge -> resolve -> validate.

use crate::detect::{self, LinuxDesktop};
use crate::model::{ThemeSpec, ThemeVariant};
use crate::SystemTheme;

// === Public API ===

pub fn platform_preset_name() -> &'static str { /* ... */ }
pub fn diagnose_platform_support() -> Vec<String> { /* ... */ }

// === Crate-internal (called by SystemTheme impl in lib.rs) ===

pub(crate) fn run_pipeline(
    reader_output: ThemeSpec,
    preset_name: &str,
    is_dark: bool,
) -> crate::Result<SystemTheme> { /* ... */ }

pub(crate) fn from_system_inner() -> crate::Result<SystemTheme> { /* ... */ }

#[cfg(target_os = "linux")]
pub(crate) async fn from_system_async_inner() -> crate::Result<SystemTheme> { /* ... */ }

// === Private ===

fn reader_is_dark(reader: &ThemeSpec) -> bool { /* ... */ }

#[cfg(target_os = "linux")]
fn from_linux() -> crate::Result<SystemTheme> { /* ... */ }

#[cfg(target_os = "linux")]
fn linux_preset_for_de(de: LinuxDesktop) -> &'static str { /* ... */ }

// === Tests ===

#[cfg(all(test, target_os = "linux"))]
mod dispatch_tests { /* ... */ }

// run_pipeline tests (moved from system_theme_tests):
#[cfg(test)]
mod pipeline_tests { /* ... */ }
```

### lib.rs After Split (skeleton)
```rust
// Source: Derived from codebase analysis [VERIFIED: native-theme/src/lib.rs]

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

macro_rules! impl_merge { /* ... unchanged ... */ }

// --- Existing module declarations (unchanged) ---
pub mod color;
pub mod error;
// ... etc

// --- New internal modules ---
mod detect;
mod pipeline;
mod icons;
#[cfg(test)]
mod test_util;

// --- Re-exports from new modules ---
#[cfg(target_os = "linux")]
pub use detect::LinuxDesktop;
#[cfg(target_os = "linux")]
pub use detect::detect_linux_de;
pub use detect::{
    system_is_dark, detect_is_dark, invalidate_caches,
    prefers_reduced_motion, detect_reduced_motion,
};
pub use pipeline::{platform_preset_name, diagnose_platform_support};
pub use icons::{
    load_icon, load_icon_from_theme, load_system_icon_by_name,
    load_custom_icon, loading_indicator, is_freedesktop_theme_available,
};

// --- Existing re-exports (unchanged) ---
pub use color::{ParseColorError, Rgba};
// ... etc

/// Convenience Result type alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Result of the OS-first pipeline.
pub struct SystemTheme { /* ... unchanged ... */ }

impl SystemTheme {
    pub fn active(&self) -> &ResolvedThemeVariant { /* ... */ }
    pub fn pick(&self, is_dark: bool) -> &ResolvedThemeVariant { /* ... */ }
    pub fn with_overlay(&self, overlay: &ThemeSpec) -> crate::Result<Self> { /* ... */ }
    pub fn with_overlay_toml(&self, toml: &str) -> crate::Result<Self> { /* ... */ }
    pub fn from_system() -> crate::Result<Self> {
        pipeline::from_system_inner()
    }
    #[cfg(target_os = "linux")]
    pub async fn from_system_async() -> crate::Result<Self> {
        pipeline::from_system_async_inner().await
    }
    #[cfg(not(target_os = "linux"))]
    pub async fn from_system_async() -> crate::Result<Self> {
        pipeline::from_system_inner()
    }
}

// --- Tests that stay in lib.rs ---
#[cfg(test)]
mod system_theme_tests { /* active/pick/platform_preset_name tests */ }

#[cfg(test)]
mod overlay_tests { /* with_overlay tests */ }
```

## Cross-Module Reference Update Map

After the split, these files need `crate::` path updates:

| File | Current Path | New Path |
|------|-------------|----------|
| gnome/mod.rs:126 | `crate::run_gsettings_with_timeout(&["get", schema, key])` | `crate::detect::gsettings_get(schema, key)` |
| gnome/mod.rs:142 | `crate::read_xft_dpi()` | `crate::detect::xft_dpi()` |
| gnome/mod.rs:146 | `crate::detect_physical_dpi()` | `crate::detect::physical_dpi()` |
| kde/mod.rs:92 | `crate::read_xft_dpi()` | `crate::detect::xft_dpi()` |
| kde/mod.rs:97 | `crate::detect_physical_dpi()` | `crate::detect::physical_dpi()` |
| kde/mod.rs:429,440,705 | `crate::ENV_MUTEX` | `crate::test_util::ENV_MUTEX` |
| resolve/mod.rs:102 | `crate::detect_system_font_dpi()` | `crate::detect::system_font_dpi()` |
| resolve/inheritance.rs:101 | `crate::detect_linux_de(&crate::xdg_current_desktop())` | `crate::detect::detect_linux_de(&crate::detect::xdg_current_desktop())` |
| presets.rs:137 | `crate::detect_linux_de(&crate::xdg_current_desktop()) == crate::LinuxDesktop::Kde` | `crate::detect::detect_linux_de(&crate::detect::xdg_current_desktop()) == crate::detect::LinuxDesktop::Kde` |
| model/icons.rs:573-584 | `crate::detect_linux_de(...)`, `crate::LinuxDesktop::*` | `crate::detect::detect_linux_de(...)`, `crate::detect::LinuxDesktop::*` |

**Alternative (simpler):** If `LinuxDesktop` and `detect_linux_de` are re-exported from lib.rs, some paths could remain as `crate::LinuxDesktop`. But since the CONTEXT.md decision says "module-private with narrower accessor functions", the callers should use the module path. However, `LinuxDesktop` is `pub` (crate public API), so it must be re-exported regardless. Internal callers can use either `crate::LinuxDesktop` or `crate::detect::LinuxDesktop` -- both work because re-exports create aliases.

**Recommendation:** Keep re-exports in lib.rs for public API stability. Internal callers use `crate::detect::` paths for clarity about where things live.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| file/mod.rs directory modules | file.rs flat files | Rust 2018 edition | Flat files preferred for small modules [VERIFIED: Rust Reference] |
| pub(crate) free access | Module-private + accessors | Rust best practice | Cleaner API boundaries, easier refactoring |

**Rust Edition 2024 note:** Edition 2024 (used by this project) uses the same module resolution as 2021. `mod detect;` looks for `detect.rs` or `detect/mod.rs`. Flat files are the correct choice per CONTEXT.md. [VERIFIED: Cargo.toml shows edition = "2024"]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | run_gsettings_with_timeout callers only use "get" subcommand pattern | Accessor Functions | If callers use other gsettings subcommands, the narrower `gsettings_get()` accessor won't work. Verified: gnome/mod.rs:126 only calls with `["get", schema, key]` -- confirmed. | 
| A2 | The pre-existing test failure (gnome::tests::build_gnome_variant_normal_contrast_no_flag) is unrelated to this phase | Summary | LOW RISK: This test existed before the split and will continue to fail/pass independently. |

**All other claims in this research were verified by direct codebase analysis** -- no user confirmation needed.

## Open Questions

1. **Accessor function naming for gsettings**
   - What we know: `run_gsettings_with_timeout` is only called with `["get", ...]` args from gnome module.
   - What's unclear: Whether future phases might need other gsettings subcommands.
   - Recommendation: Create `gsettings_get(schema, key)` for now. If broader access is needed later, add more accessor functions. The private `run_gsettings_with_timeout` remains available for extension.

2. **dispatch_tests module name after move**
   - What we know: dispatch_tests tests both `detect_linux_de()` (detection) and `from_linux()` (pipeline).
   - What's unclear: Whether to split the test module or keep it whole.
   - Recommendation: Keep the test module whole in pipeline.rs since the from_linux tests depend on detect_linux_de results and they test the integration between detection and pipeline dispatch. The detect_linux_de pure function tests (14 of them) could alternatively move to detect.rs for better locality. The planner should decide the cut point.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in test framework (libtest) |
| Config file | None (standard cargo test) |
| Quick run command | `cargo test --package native-theme --features "kde,portal-tokio,material-icons,lucide-icons,svg-rasterize,system-icons" --lib` |
| Full suite command | `cargo test --package native-theme --features "kde,portal-tokio,material-icons,lucide-icons,svg-rasterize,system-icons"` |

### Phase Requirements Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| STRUCT-01 | lib.rs under 300 lines | smoke | `wc -l native-theme/src/lib.rs` | N/A (manual check) |
| STRUCT-01 | detect.rs exists with correct functions | compilation | `cargo check --package native-theme --features "kde,portal-tokio"` | Will be created |
| STRUCT-01 | pipeline.rs exists with correct functions | compilation | `cargo check --package native-theme` | Will be created |
| STRUCT-01 | icons.rs exists with correct functions | compilation | `cargo check --package native-theme --features "material-icons,system-icons"` | Will be created |
| STRUCT-01 | All existing tests pass unchanged | unit/integration | `cargo test --package native-theme --features "kde,portal-tokio,material-icons,lucide-icons,svg-rasterize,system-icons" --lib` | Existing (668 pass, 1 pre-existing failure) |
| STRUCT-01 | No test count regression | smoke | `cargo test --package native-theme ... -- --list 2>/dev/null \| wc -l` (compare before/after) | N/A |

### Sampling Rate
- **Per task commit:** `cargo check --package native-theme --features "kde,portal-tokio,material-icons,lucide-icons,svg-rasterize,system-icons"`
- **Per wave merge:** Full test suite + multi-feature check
- **Phase gate:** Full suite green + `wc -l` on lib.rs < 300 + all 668 tests pass

### Wave 0 Gaps
None -- existing test infrastructure covers all phase requirements. No new test framework or fixtures needed.

## Sources

### Primary (HIGH confidence)
- Direct codebase analysis of `native-theme/src/lib.rs` (2,767 lines, fully read)
- Direct codebase analysis of cross-module references via grep across `native-theme/src/`
- `Cargo.toml` workspace and crate configuration (edition 2024, Rust 1.94)
- `cargo test` baseline run (668 pass, 1 fail, 3 ignored)

### Secondary (MEDIUM confidence)
- Rust Edition 2024 module resolution rules [ASSUMED -- based on Rust 2021 precedent, Edition 2024 follows same rules]

## Metadata

**Confidence breakdown:**
- Module mapping: HIGH - every function, struct, and test module mapped with line numbers from actual source
- Architecture patterns: HIGH - verified by codebase structure and Rust language rules
- Cross-module references: HIGH - grep-verified, all 5 external caller files identified
- Pitfalls: HIGH - based on concrete analysis of this specific codebase, not generic advice

**Research date:** 2026-04-09
**Valid until:** 2026-05-09 (stable -- pure refactor, no external dependency changes)
