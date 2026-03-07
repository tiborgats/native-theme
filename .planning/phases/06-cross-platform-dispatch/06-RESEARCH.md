# Phase 6: Cross-Platform Dispatch - Research

**Researched:** 2026-03-07
**Domain:** Cross-platform dispatch, desktop environment detection, conditional compilation, mock-based testing
**Confidence:** HIGH

## Summary

Phase 6 ties together the three platform readers (KDE, GNOME portal, Windows) behind a single `from_system()` function that auto-detects the current platform and desktop environment, then calls the appropriate reader. The core challenge is threefold: (1) compile-time platform dispatch via `#[cfg(target_os)]`, (2) runtime desktop environment detection on Linux via `XDG_CURRENT_DESKTOP`, and (3) handling the async/sync divide since `from_gnome()` is async while `from_system()` must be sync.

The project's reference architecture (IMPLEMENTATION.md section 13.3) already specifies the design: `from_system()` is always sync. On Linux, it checks `XDG_CURRENT_DESKTOP` for "KDE" and calls `from_kde()` if the `kde` feature is enabled. For GNOME/GTK desktops, it falls back to the bundled Adwaita preset rather than calling the async `from_gnome()`. Users who want live portal data call `from_gnome().await` directly. On Windows, it calls `from_windows()` if the `windows` feature is enabled. On unsupported platforms, it returns `Error::Unsupported`. The function must compile on ALL platforms regardless of which features are enabled -- missing features produce `Error::Unsupported` at runtime, not compilation errors.

The TEST-04 requirement covers platform reader unit tests with mock/fixture data. The existing readers already have extensive unit tests using internal `build_theme()` / `from_kde_content()` helpers. Phase 6 adds dispatch-level tests (verifying `from_system()` routes correctly) and ensures each platform's mock coverage is complete.

**Primary recommendation:** Implement `from_system()` as a thin dispatch function in `lib.rs` (or a small `dispatch.rs` module) using nested `#[cfg]` attributes for platform and feature gates. Do NOT add the `detect-desktop-environment` crate -- the detection logic is 5 lines checking `XDG_CURRENT_DESKTOP` for "KDE". Keep `from_system()` sync; document that GNOME users should call `from_gnome().await` directly for live accent colors.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| PLAT-03 | Cross-platform dispatch: from_system() -- auto-detects platform/DE, calls appropriate reader | Architecture pattern (cfg dispatch + XDG_CURRENT_DESKTOP), code examples from IMPLEMENTATION.md reference design, feature flag compilability strategy |
| TEST-04 | Platform reader unit tests with mock data | Existing test patterns (from_kde_content fixtures, gnome build_theme tests, windows build_theme tests), dispatch routing tests via env var manipulation |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| std::env | (stdlib) | Read XDG_CURRENT_DESKTOP for DE detection on Linux | Zero-dependency, the standard approach per freedesktop.org spec |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| (none -- all dependencies already in Cargo.toml) | - | - | Phase 6 adds no new dependencies |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Manual XDG_CURRENT_DESKTOP parsing | `detect-desktop-environment` v1.2.0 (zero-dep crate) | Overkill -- we only need "is it KDE?" which is 5 lines. The crate detects 24 DEs we don't need. Adding a dependency for a string contains-check is not justified. |
| Sync `from_system()` only | Async `from_system()` that calls `from_gnome().await` | Would force an async runtime on all consumers. The reference design explicitly rejects this -- `from_system()` is sync, GNOME users call `from_gnome().await` directly. |

**Installation:**
```bash
# No new dependencies needed -- Phase 6 is pure dispatch logic
```

## Architecture Patterns

### Recommended Project Structure
```
src/
  lib.rs              # Add from_system() here (or thin dispatch.rs module)
  kde/mod.rs           # Existing -- from_kde() (sync)
  gnome/mod.rs         # Existing -- from_gnome() (async)
  windows.rs           # Existing -- from_windows() (sync)
  error.rs             # Existing -- Error::Unsupported already defined
  model/               # Existing -- NativeTheme, ThemeVariant
  presets.rs           # Existing -- preset("adwaita") used as GNOME fallback
```

### Pattern 1: Compile-Time Platform Dispatch with Feature Gates
**What:** Use `#[cfg(target_os)]` for platform routing and `#[cfg(feature)]` for reader availability, with `Error::Unsupported` as the fallback for missing features.
**When to use:** Always -- this is the only pattern for `from_system()`.
**Example:**
```rust
// Source: IMPLEMENTATION.md section 13.3 (project reference design)
/// Read the current system theme.
///
/// On Linux, auto-detects KDE (via `XDG_CURRENT_DESKTOP`) and calls
/// `from_kde()` if the `kde` feature is enabled. For GNOME/GTK desktops,
/// returns the bundled Adwaita preset. For live GNOME portal data
/// (accent color, contrast), call `from_gnome().await` directly.
///
/// On Windows, calls `from_windows()` if the `windows` feature is enabled.
///
/// Returns `Error::Unsupported` on platforms without a reader or when
/// the required feature is not enabled.
pub fn from_system() -> crate::Result<NativeTheme> {
    #[cfg(target_os = "windows")]
    {
        #[cfg(feature = "windows")]
        return crate::windows::from_windows();

        #[cfg(not(feature = "windows"))]
        return Err(Error::Unsupported);
    }

    #[cfg(target_os = "linux")]
    {
        return from_linux();
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        Err(Error::Unsupported)
    }
}
```

### Pattern 2: Linux Desktop Environment Detection
**What:** Parse `XDG_CURRENT_DESKTOP` (colon-separated list per freedesktop spec) and route to the appropriate reader.
**When to use:** Inside `from_linux()` helper.
**Example:**
```rust
// Source: IMPLEMENTATION.md section 13.3 + freedesktop Desktop Entry Spec
#[cfg(target_os = "linux")]
fn from_linux() -> crate::Result<NativeTheme> {
    // XDG_CURRENT_DESKTOP is a colon-separated list (e.g., "KDE", "ubuntu:GNOME")
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    let is_kde = desktop.split(':').any(|s| s == "KDE");

    // 1. KDE detected and feature enabled
    #[cfg(feature = "kde")]
    if is_kde {
        return crate::kde::from_kde();
    }

    // 2. KDE detected but feature not enabled
    #[cfg(not(feature = "kde"))]
    if is_kde {
        // Cannot read KDE -- fall through to preset fallback
    }

    // 3. Fallback: load Adwaita preset (covers GNOME, XFCE, Cinnamon, etc.)
    crate::preset("adwaita")
        .ok_or_else(|| Error::Unavailable(
            "no platform reader available and preset fallback failed".into()
        ))
}
```

### Pattern 3: Feature-Guarded Code Blocks for Cross-Compilation
**What:** Every reader call is wrapped in `#[cfg(feature = "...")]` so `from_system()` compiles on all platforms even without reader features.
**When to use:** All platform reader calls.
**Example:**
```rust
// This compiles even if "kde" feature is OFF:
#[cfg(feature = "kde")]
if is_kde {
    return crate::kde::from_kde();
}
// When feature is OFF, this cfg block is entirely removed by the compiler.
// Code falls through to the fallback.
```

### Anti-Patterns to Avoid
- **Calling `from_gnome()` from `from_system()`:** `from_gnome()` is async. Blocking on it (via `block_on`) would require pulling in a runtime dependency (tokio/async-std). The reference design explicitly avoids this -- GNOME gets the Adwaita preset from `from_system()`.
- **Using `cfg!(...)` macro (runtime) instead of `#[cfg(...)]` (compile-time):** `cfg!()` evaluates to a bool at runtime but does NOT remove dead code. Using it for platform dispatch would cause compilation errors on platforms where the reader module is not available.
- **Exact string equality on XDG_CURRENT_DESKTOP:** The variable is a colon-separated list. Using `de == "KDE"` would miss `"ubuntu:KDE"` or `"KDE:plasma"`. Always split on `:` and check each component.
- **Adding detect-desktop-environment as a dependency:** The crate is well-made but we need exactly one 5-line check. Adding a dependency for this is over-engineering.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Full desktop environment detection (24 DEs) | Custom DE detection framework | Only detect what we dispatch to (KDE for now) | We don't have readers for XFCE/Cinnamon/etc. -- detecting them adds no value. Adwaita preset is the correct fallback for all GTK-based DEs. |
| Async-to-sync bridge | Custom `block_on` for calling `from_gnome()` | Bundled Adwaita preset | The project already has a high-quality Adwaita preset. Building a sync bridge adds runtime dependencies and complexity for marginal benefit. |

**Key insight:** The dispatch layer should be as thin as possible. All the actual work is in the readers (already built) and presets (already built). `from_system()` is just routing logic.

## Common Pitfalls

### Pitfall 1: Compilation on Non-Linux Without Reader Features
**What goes wrong:** `from_system()` fails to compile on macOS/Windows because it references reader modules that don't exist without features enabled.
**Why it happens:** Missing `#[cfg(feature = "...")]` guards around reader calls, or using `cfg!()` macro (which doesn't remove code) instead of `#[cfg()]` attribute.
**How to avoid:** Every reader call must be inside a `#[cfg(feature = "xxx")]` block. Use `#[cfg(target_os = "...")]` for platform blocks, `#[cfg(feature = "...")]` for reader availability. Test compilation with `cargo check` on bare features.
**Warning signs:** CI fails on `cargo check --no-default-features` or `cargo check` (no features).

### Pitfall 2: XDG_CURRENT_DESKTOP Is a Colon-Separated List
**What goes wrong:** Code uses `de.contains("KDE")` which would incorrectly match `"FAKEKDE"` or `de == "KDE"` which misses `"ubuntu:KDE"`.
**Why it happens:** Treating the variable as a simple string instead of a list.
**How to avoid:** Split on `:` and check each component: `desktop.split(':').any(|s| s == "KDE")`.
**Warning signs:** Ubuntu KDE users get the wrong reader (Ubuntu sets `XDG_CURRENT_DESKTOP=KDE` but could theoretically set `ubuntu:KDE`).

### Pitfall 3: Dead Code Warnings from Feature-Gated Blocks
**What goes wrong:** When compiling without `kde` feature, variables like `is_kde` may trigger "unused variable" warnings because the `#[cfg(feature = "kde")]` block that uses them is removed.
**Why it happens:** Rust's dead-code analysis runs after cfg processing.
**How to avoid:** Either move the variable inside the cfg block, prefix with `_`, or restructure so the variable is only created when needed. The cleanest approach is to check both the env var AND the feature in the same block.
**Warning signs:** `cargo check` warnings with certain feature combinations.

### Pitfall 4: Forgetting Error::Unsupported for Unsupported Platforms
**What goes wrong:** `from_system()` compiles to an empty function body on unsupported platforms, causing a "mismatched types" error (no return value).
**Why it happens:** All cfg branches are platform-specific, with no fallback.
**How to avoid:** Always include a `#[cfg(not(any(...)))]` catch-all branch returning `Err(Error::Unsupported)`.
**Warning signs:** Cross-compilation to `wasm32-unknown-unknown` or other non-desktop targets fails.

### Pitfall 5: Test Isolation for Environment Variable Manipulation
**What goes wrong:** Tests that set `XDG_CURRENT_DESKTOP` interfere with each other when running in parallel.
**Why it happens:** Environment variables are process-global state.
**How to avoid:** Either run env-mutating tests with `--test-threads=1`, or (better) extract the DE detection into a pure function that takes the env var value as a parameter, making tests pure and parallelizable.
**Warning signs:** Flaky test results that depend on execution order.

## Code Examples

Verified patterns from project sources:

### Complete from_system() Implementation
```rust
// Source: IMPLEMENTATION.md section 13.3, adapted to actual project structure

/// Read the current system theme, auto-detecting the platform and
/// desktop environment.
///
/// # Platform Behavior
///
/// - **Linux (KDE):** Calls `from_kde()` when `XDG_CURRENT_DESKTOP` contains
///   "KDE" and the `kde` feature is enabled.
/// - **Linux (other):** Returns the bundled Adwaita preset. For live GNOME
///   portal data, call [`from_gnome()`] directly.
/// - **Windows:** Calls `from_windows()` when the `windows` feature is enabled.
/// - **Other platforms:** Returns `Error::Unsupported`.
///
/// # Errors
///
/// - `Error::Unsupported` if the platform has no reader or required feature
///   is not enabled.
/// - `Error::Unavailable` if the platform reader cannot access theme data.
pub fn from_system() -> crate::Result<NativeTheme> {
    #[cfg(target_os = "windows")]
    {
        #[cfg(feature = "windows")]
        return crate::windows::from_windows();

        #[cfg(not(feature = "windows"))]
        return Err(crate::Error::Unsupported);
    }

    #[cfg(target_os = "linux")]
    {
        return from_linux();
    }

    // Catch-all for unsupported platforms (macOS reader is v2)
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        Err(crate::Error::Unsupported)
    }
}

#[cfg(target_os = "linux")]
fn from_linux() -> crate::Result<NativeTheme> {
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    let is_kde = desktop.split(':').any(|s| s == "KDE");

    #[cfg(feature = "kde")]
    if is_kde {
        return crate::kde::from_kde();
    }

    // Fallback: Adwaita preset (covers GNOME, XFCE, Cinnamon, etc.)
    crate::preset("adwaita")
        .ok_or_else(|| crate::Error::Unavailable(
            "no platform reader available and preset fallback failed".into()
        ))
}
```

### Testable DE Detection Helper (Pure Function)
```rust
// Enables testing without modifying environment variables
#[cfg(target_os = "linux")]
fn detect_linux_de(xdg_current_desktop: &str) -> LinuxDesktop {
    let components: Vec<&str> = xdg_current_desktop.split(':').collect();
    if components.iter().any(|&s| s == "KDE") {
        LinuxDesktop::Kde
    } else if components.iter().any(|&s| s == "GNOME") {
        LinuxDesktop::Gnome
    } else {
        LinuxDesktop::Unknown
    }
}

#[cfg(target_os = "linux")]
#[derive(Debug, PartialEq)]
enum LinuxDesktop {
    Kde,
    Gnome,
    Unknown,
}
```

### Testing from_system() Dispatch
```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Pure function tests -- no env var mutation needed
    #[test]
    fn detect_kde_simple() {
        assert_eq!(detect_linux_de("KDE"), LinuxDesktop::Kde);
    }

    #[test]
    fn detect_kde_colon_separated() {
        assert_eq!(detect_linux_de("ubuntu:KDE"), LinuxDesktop::Kde);
    }

    #[test]
    fn detect_gnome_simple() {
        assert_eq!(detect_linux_de("GNOME"), LinuxDesktop::Gnome);
    }

    #[test]
    fn detect_gnome_ubuntu() {
        assert_eq!(detect_linux_de("ubuntu:GNOME"), LinuxDesktop::Gnome);
    }

    #[test]
    fn detect_unknown_xfce() {
        assert_eq!(detect_linux_de("XFCE"), LinuxDesktop::Unknown);
    }

    #[test]
    fn detect_empty_string() {
        assert_eq!(detect_linux_de(""), LinuxDesktop::Unknown);
    }
}
```

### Existing Test Patterns for Platform Readers (Reference)
```rust
// KDE: from_kde_content() internal helper accepts string fixture
// Source: src/kde/mod.rs
fn from_kde_content(content: &str) -> crate::Result<crate::NativeTheme> { /* ... */ }

// GNOME: build_theme() internal helper accepts portal values directly
// Source: src/gnome/mod.rs
pub(crate) fn build_theme(
    base: crate::NativeTheme,
    scheme: ColorScheme,
    accent: Option<Color>,
    contrast: Contrast,
) -> crate::Result<crate::NativeTheme> { /* ... */ }

// Windows: build_theme() internal helper accepts raw color/geometry
// Source: src/windows.rs
fn build_theme(
    accent: crate::Rgba,
    fg: crate::Rgba,
    bg: crate::Rgba,
    geometry: crate::ThemeGeometry,
) -> crate::NativeTheme { /* ... */ }
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `dark-light` crate (detects dark/light boolean only) | `native-theme` reads 36 semantic colors + fonts + geometry per platform | n/a (greenfield) | Much richer theme data; `from_system()` provides the same simple entry point |
| External `detect-desktop-environment` crate | Inline `XDG_CURRENT_DESKTOP` check (5 lines) | Design decision | Zero added dependencies; we only need KDE detection, not 24 DEs |
| Async `from_system()` (blocks on GNOME portal) | Sync `from_system()` with Adwaita preset fallback for GNOME | Design decision per IMPLEMENTATION.md | Avoids forcing async runtime on all consumers |

**Deprecated/outdated:**
- The original ARCHITECTURE.md mentions a `platform/mod.rs` directory structure, but the actual implementation uses flat modules (`kde/`, `gnome/`, `windows.rs`) directly in `src/`. Phase 6 should follow the actual structure, not the original architecture plan.

## Open Questions

1. **Should `from_system()` live in `lib.rs` or a separate `dispatch.rs` module?**
   - What we know: The function is ~30 lines total. The project currently has no `dispatch.rs` or `platform/mod.rs` module.
   - What's unclear: Whether adding a new module is cleaner than putting the function directly in `lib.rs`.
   - Recommendation: Put it directly in `lib.rs` -- it is a top-level public API function and only ~30 lines. Adding a module for such a small amount of code adds indirection without benefit. Re-export as `pub use` from `lib.rs`.

2. **Should `from_linux()` attempt kdeglobals fallback when XDG_CURRENT_DESKTOP is not KDE?**
   - What we know: The IMPLEMENTATION.md reference design includes a step 2 that checks `dirs::config_dir()` for kdeglobals even on non-KDE desktops. However, the project does not depend on the `dirs` crate.
   - What's unclear: Whether reading kdeglobals on non-KDE desktops (e.g., someone with KDE apps on GNOME) provides value.
   - Recommendation: Skip the kdeglobals-exists fallback. On non-KDE desktops, the Adwaita preset is a better fallback than possibly-stale KDE settings. The reference design's step 2 used the `dirs` crate which is not in our dependencies; using `kde::kdeglobals_path()` + `std::path::Path::exists()` would work but adds complexity for a niche case.

3. **Where should TEST-04 tests live?**
   - What we know: Existing platform tests are in `#[cfg(test)] mod tests` inside each reader module (src/kde/mod.rs, src/gnome/mod.rs, src/windows.rs). The dispatch tests need to verify routing logic.
   - What's unclear: Whether TEST-04 means new integration tests in `tests/` or additional unit tests in existing modules.
   - Recommendation: (a) Dispatch routing tests go in the same file as `from_system()` (likely `lib.rs`). (b) Any missing reader mock tests should be added to the existing reader modules. The existing reader tests already have good mock coverage via `from_kde_content()`, `build_theme()` helpers.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in test framework (cargo test) |
| Config file | Cargo.toml (edition 2024) |
| Quick run command | `cargo test --lib` |
| Full suite command | `cargo test --all-features` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PLAT-03 | from_system() returns NativeTheme on Linux/KDE | unit | `cargo test --lib --features kde from_system` | No -- Wave 0 |
| PLAT-03 | from_system() returns Adwaita preset on Linux/GNOME | unit | `cargo test --lib from_system` | No -- Wave 0 |
| PLAT-03 | from_system() returns Error::Unsupported on unsupported platform | unit | `cargo test --lib from_system` | No -- Wave 0 |
| PLAT-03 | from_system() compiles with no features enabled | unit | `cargo check --no-default-features` | No -- Wave 0 |
| PLAT-03 | XDG_CURRENT_DESKTOP colon-separated parsing | unit | `cargo test --lib detect_linux` | No -- Wave 0 |
| TEST-04 | KDE reader unit tests with fixture data | unit | `cargo test --lib --features kde kde::tests` | Yes (existing) |
| TEST-04 | GNOME reader unit tests with mock build_theme | unit | `cargo test --lib --features portal gnome::tests` | Yes (existing) |
| TEST-04 | Windows reader unit tests with mock build_theme | unit | `cargo test --lib --features windows windows::tests` | Yes (existing) |

### Sampling Rate
- **Per task commit:** `cargo test --lib` (no features, ~0.5s)
- **Per wave merge:** `cargo test --all-features` (full suite)
- **Phase gate:** `cargo check --no-default-features && cargo test --all-features`

### Wave 0 Gaps
- [ ] Dispatch routing tests (from_system / from_linux) -- covers PLAT-03
- [ ] DE detection pure function tests -- covers PLAT-03
- [ ] Compilation check with no features -- covers PLAT-03

*(Existing reader tests already cover TEST-04 substantially. May need additions for edge cases.)*

## Sources

### Primary (HIGH confidence)
- `docs/IMPLEMENTATION.md` section 13.3 -- project reference design for `from_system()`, `from_linux()` (authoritative, project-internal)
- `.planning/research/ARCHITECTURE.md` -- component responsibilities and flow diagrams (project-internal)
- `.planning/research/FEATURES.md` -- feature landscape documenting cross-platform dispatch rationale (project-internal)
- `src/lib.rs`, `src/kde/mod.rs`, `src/gnome/mod.rs`, `src/windows.rs` -- actual codebase (ground truth)
- [Rust Reference: Conditional compilation](https://doc.rust-lang.org/reference/conditional-compilation.html) -- `#[cfg]` attribute semantics
- [freedesktop Desktop Entry Specification](https://specifications.freedesktop.org/desktop-entry/latest-single/) -- XDG_CURRENT_DESKTOP is colon-separated list

### Secondary (MEDIUM confidence)
- [detect-desktop-environment crate v1.2.0](https://docs.rs/detect-desktop-environment/latest/detect_desktop_environment/) -- evaluated and rejected (zero-dep but overkill for our needs)
- [dark-light crate](https://github.com/frewsxcv/rust-dark-light) -- reference prior art for cross-platform theme dispatch
- [NullDeref: Supporting both async and sync code in Rust](https://nullderef.com/blog/rust-async-sync/) -- patterns for async/sync coexistence

### Tertiary (LOW confidence)
- None -- all findings verified against project sources and official docs.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - no new dependencies needed; all patterns verified against existing codebase
- Architecture: HIGH - reference design exists in IMPLEMENTATION.md, codebase structure is well-understood
- Pitfalls: HIGH - all pitfalls derived from actual Rust compilation behavior and project-specific patterns

**Research date:** 2026-03-07
**Valid until:** 2026-04-07 (stable -- no moving targets; pure Rust dispatch logic)
