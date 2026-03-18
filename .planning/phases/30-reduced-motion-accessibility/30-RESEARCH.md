# Phase 30: Reduced Motion Accessibility - Research

**Researched:** 2026-03-18
**Domain:** OS accessibility settings, cross-platform reduced-motion detection
**Confidence:** HIGH

## Summary

This phase adds a single public function `prefers_reduced_motion() -> bool` that queries the OS-level "reduce motion" or "disable animations" setting, cached via `OnceLock` for zero-cost subsequent calls. The implementation follows the exact same pattern as the existing `system_is_dark()` function in `lib.rs`: a public function wrapping `OnceLock::get_or_init()` that delegates to a platform-specific inner function.

Each platform has a well-documented, stable API for this query. Linux uses the `gsettings` subprocess to read `org.gnome.desktop.interface enable-animations` (a boolean key that outputs `true`/`false`). macOS uses `NSWorkspace.sharedWorkspace().accessibilityDisplayShouldReduceMotion()` via the objc2-app-kit crate (requires adding `NSWorkspace` and `NSAccessibility` features). Windows uses `UISettings::new()?.AnimationsEnabled()` via the already-enabled `UI_ViewManagement` feature in the `windows` crate.

The function returns `false` (allow animations) on unsupported platforms or when any query fails, matching the graceful degradation pattern used throughout the crate.

**Primary recommendation:** Model `prefers_reduced_motion()` exactly on the existing `system_is_dark()` pattern -- `OnceLock` + inner function, platform `cfg` gating, `false` default on failure/unsupported.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| A11Y-01 | `prefers_reduced_motion() -> bool` queries OS accessibility setting, sync with OnceLock caching | Exact pattern exists in `system_is_dark()` at lib.rs:187-189: `static CACHED: OnceLock<bool>; *CACHED.get_or_init(inner)` |
| A11Y-02 | Linux: query via `gsettings org.gnome.desktop.interface enable-animations` (sync subprocess fallback) | gsettings subprocess pattern used 6+ times in codebase; key outputs bare `true`/`false`; note the *inverted* semantics |
| A11Y-03 | macOS: query `NSWorkspace.accessibilityDisplayShouldReduceMotion` | Available since macOS 10.12; objc2-app-kit needs `NSWorkspace` + `NSAccessibility` features added to Cargo.toml |
| A11Y-04 | Windows: query `UISettings.AnimationsEnabled()` | `UISettings` already imported in windows.rs; `AnimationsEnabled()` returns `Result<bool>`; `UI_ViewManagement` feature already enabled |
| A11Y-05 | Returns `false` (allow animations) on unsupported platforms or query failure | Matches `system_is_dark()` pattern which returns `false` as default |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| std::sync::OnceLock | stable (1.80+) | Thread-safe one-shot cache | Already used for `system_is_dark()` and `system_icon_theme()` in this crate |
| std::process::Command | stable | gsettings subprocess on Linux | Already used for 6+ gsettings queries in this crate |
| objc2-app-kit | 0.3 | NSWorkspace.accessibilityDisplayShouldReduceMotion on macOS | Already a dependency; needs `NSWorkspace` + `NSAccessibility` features added |
| windows (crate) | >=0.59, <=0.62 | UISettings.AnimationsEnabled on Windows | Already a dependency with `UI_ViewManagement` enabled |

### Supporting
No additional dependencies needed. All required crates are already in the dependency tree.

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| gsettings subprocess | D-Bus portal (ashpd) | Async-only; Out of Scope per REQUIREMENTS.md ("Async prefers_reduced_motion") |
| gsettings subprocess | dconf direct read | Less portable, gsettings is the standard CLI |
| OnceLock | LazyLock | OnceLock is the established pattern in this crate |

**Installation:**
No new crate installations. Only Cargo.toml feature additions for objc2-app-kit:
```toml
# Add to existing objc2-app-kit features list:
"NSWorkspace", "NSAccessibility"
```

## Architecture Patterns

### Recommended File Structure
```
native-theme/src/
  lib.rs          # prefers_reduced_motion() public function + OnceLock + cfg dispatch
                  # (alongside existing system_is_dark(), loading_indicator(), etc.)
```

The function lives directly in `lib.rs`, not in a separate module. This matches how `system_is_dark()` is implemented -- a small top-level utility function that delegates to platform-specific inner functions within the same file.

### Pattern 1: OnceLock Cached Platform Query (established in this crate)
**What:** A public function that caches a boolean OS query in a static OnceLock, delegating to a cfg-gated inner function for the actual platform call.
**When to use:** Any sync, one-shot OS setting query that does not change during process lifetime.
**Example:**
```rust
// Source: existing pattern from lib.rs:187-225
#[cfg(target_os = "linux")]
#[must_use = "this returns whether reduced motion is preferred"]
pub fn prefers_reduced_motion() -> bool {
    static CACHED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *CACHED.get_or_init(detect_reduced_motion_inner)
}
```

### Pattern 2: Inverted Boolean Semantics
**What:** The Linux `enable-animations` key is the logical inverse of "prefers reduced motion". When `enable-animations` is `false`, the user prefers reduced motion (`true`). The function must invert this.
**When to use:** Any platform where the OS setting has opposite polarity from the function's return value.
**Example:**
```rust
// gsettings returns "true" when animations are enabled
// prefers_reduced_motion returns true when animations should be REDUCED
// So: enable-animations=false => prefers_reduced_motion()=true
let val = String::from_utf8_lossy(&output.stdout);
let trimmed = val.trim();
if trimmed == "false" {
    return true; // animations disabled => reduced motion preferred
}
if trimmed == "true" {
    return false; // animations enabled => no reduced motion
}
```

### Pattern 3: Multi-Platform cfg Dispatch
**What:** Each platform gets its own `#[cfg(target_os = "...")]` block within a single function or via separate inner functions.
**When to use:** Cross-platform functions with platform-specific implementations.
**Example:**
```rust
// Source: existing pattern from lib.rs:276-304 (from_system)
pub fn prefers_reduced_motion() -> bool {
    #[cfg(target_os = "linux")]
    { /* gsettings query */ }

    #[cfg(target_os = "macos")]
    { /* NSWorkspace query */ }

    #[cfg(target_os = "windows")]
    { /* UISettings query */ }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    { false }
}
```

### Anti-Patterns to Avoid
- **Separate module file for a single function:** The existing pattern puts small utility functions in `lib.rs`. Do NOT create an `accessibility.rs` module for one function.
- **Async implementation:** The requirements explicitly state sync. The Out of Scope section in REQUIREMENTS.md says "Async prefers_reduced_motion -- Sync is sufficient; D-Bus portal not needed when gsettings works."
- **Feature-gating the function itself:** The function should always be available. Only the platform backends need `cfg` gating. On unsupported platforms, it should return `false`, not be absent.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| OnceLock caching logic | Custom Mutex + Option pattern | `std::sync::OnceLock` | Already proven in crate, thread-safe, zero overhead after init |
| gsettings output parsing | Complex parser for GVariant | Simple string comparison (`"true"`/`"false"`) | Boolean keys output bare `true`/`false` -- verified on live system |
| macOS Obj-C bridging | Raw msg_send calls | objc2-app-kit typed bindings | Safe wrappers already in use throughout macos.rs |

**Key insight:** This is a very straightforward feature. Each platform query is 3-10 lines of code. The risk is over-engineering it, not under-engineering it.

## Common Pitfalls

### Pitfall 1: Inverted Semantics on Linux
**What goes wrong:** `enable-animations=true` means "animations allowed" but `prefers_reduced_motion()=true` means "reduce motion." These are logical inverses.
**Why it happens:** The gsettings key name says "enable" (positive), but the API function asks about "reduced motion" (negative).
**How to avoid:** Explicitly invert: `enable-animations == "false"` => `prefers_reduced_motion() == true`.
**Warning signs:** If tests show `prefers_reduced_motion()` returns false when animations are disabled, the inversion is wrong.

### Pitfall 2: Windows AnimationsEnabled Also Has Inverted Semantics
**What goes wrong:** `UISettings.AnimationsEnabled()` returns `true` when animations are ON. Like Linux, this is the inverse of `prefers_reduced_motion()`.
**Why it happens:** Same positive/negative polarity mismatch.
**How to avoid:** `AnimationsEnabled() == false` => `prefers_reduced_motion() == true`. Invert the result: `!settings.AnimationsEnabled()?`.
**Warning signs:** Same as Linux -- test with animations disabled, expect `true`.

### Pitfall 3: macOS Has DIRECT Semantics (No Inversion Needed)
**What goes wrong:** Developer inverts the macOS result out of habit from the other platforms.
**Why it happens:** Copy-paste from Linux/Windows patterns where inversion is needed.
**How to avoid:** `accessibilityDisplayShouldReduceMotion` already returns `true` when reduced motion is preferred. No inversion needed. The function name literally says "should reduce motion."
**Warning signs:** If macOS returns the opposite of expected, check for accidental `!` negation.

### Pitfall 4: gsettings Output Has No Quotes for Booleans
**What goes wrong:** Developer expects `'true'` (with quotes) because the `color-scheme` key outputs `'prefer-dark'` (with quotes). Boolean keys output bare `true`/`false`.
**Why it happens:** The existing `detect_is_dark_inner()` uses `val.contains("prefer-dark")` which works with quotes. For booleans, the output is just `true\n` or `false\n`.
**How to avoid:** Use `val.trim() == "false"` (exact match after trimming newline), not `val.contains()`.
**Warning signs:** The value check never matches on a live system.

### Pitfall 5: OnceLock Means No Runtime Refresh
**What goes wrong:** User changes accessibility setting while app is running; `prefers_reduced_motion()` still returns the old value.
**Why it happens:** `OnceLock` caches forever by design.
**How to avoid:** This is intentional and matches `system_is_dark()` behavior. Document it. The requirements specify OnceLock caching. Runtime refresh is out of scope.
**Warning signs:** Not a bug -- it's the spec. Document that the value is cached at first call.

### Pitfall 6: macOS Feature Flag Additions Missing
**What goes wrong:** Code compiles on Linux but fails on macOS CI because `NSWorkspace` or `NSAccessibility` features are not enabled in Cargo.toml.
**Why it happens:** The features need to be explicitly added to the `objc2-app-kit` dependency.
**How to avoid:** Add `"NSWorkspace"` and `"NSAccessibility"` to the features list in `native-theme/Cargo.toml` before writing any macOS code.
**Warning signs:** Compile error about `NSWorkspace` not found on macOS.

## Code Examples

Verified patterns from the existing codebase and official sources:

### Linux: gsettings Boolean Query
```rust
// Source: modeled on lib.rs:196-225 (detect_is_dark_inner), verified gsettings output on live system
#[cfg(target_os = "linux")]
fn detect_reduced_motion_inner() -> bool {
    if let Ok(output) = std::process::Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "enable-animations"])
        .output()
        && output.status.success()
    {
        let val = String::from_utf8_lossy(&output.stdout);
        // enable-animations is INVERTED: false => reduced motion preferred
        return val.trim() == "false";
    }
    false // default: allow animations
}
```

### macOS: NSWorkspace Accessibility Query
```rust
// Source: objc2-app-kit docs (NSWorkspace::sharedWorkspace, accessibilityDisplayShouldReduceMotion)
// Requires features: "NSWorkspace", "NSAccessibility"
#[cfg(all(target_os = "macos", feature = "macos"))]
fn detect_reduced_motion_inner() -> bool {
    let workspace = objc2_app_kit::NSWorkspace::sharedWorkspace();
    workspace.accessibilityDisplayShouldReduceMotion()
    // Direct semantics: true = reduce motion preferred
}
```

### Windows: UISettings AnimationsEnabled Query
```rust
// Source: windows crate docs (UISettings::AnimationsEnabled), windows.rs existing patterns
#[cfg(all(target_os = "windows", feature = "windows"))]
fn detect_reduced_motion_inner() -> bool {
    let Ok(settings) = ::windows::UI::ViewManagement::UISettings::new() else {
        return false;
    };
    // AnimationsEnabled is INVERTED: false => reduced motion preferred
    match settings.AnimationsEnabled() {
        Ok(enabled) => !enabled,
        Err(_) => false,
    }
}
```

### Public API with OnceLock Caching (all platforms)
```rust
// Source: modeled exactly on system_is_dark() at lib.rs:187-189
/// Query whether the user prefers reduced motion.
///
/// Returns `true` when the OS accessibility setting indicates animations
/// should be reduced or disabled. Returns `false` (allow animations) on
/// unsupported platforms or when the query fails.
///
/// The result is cached after the first call using `OnceLock`.
#[must_use = "this returns whether reduced motion is preferred"]
pub fn prefers_reduced_motion() -> bool {
    static CACHED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *CACHED.get_or_init(detect_reduced_motion_inner)
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| No accessibility query | `prefers_reduced_motion()` with OnceLock | This phase | Applications can respect OS motion preference |
| N/A | gsettings `enable-animations` on GNOME | GNOME 3.x+ (stable for years) | Standard across GNOME-based DEs |
| N/A | `NSWorkspace.accessibilityDisplayShouldReduceMotion` | macOS 10.12 (2016) | Stable, unchanged API |
| N/A | `UISettings.AnimationsEnabled()` | Windows 10 (2015) | Stable WinRT API |

**Deprecated/outdated:**
- No deprecated APIs to worry about. All three platform APIs are current and stable.

## Open Questions

1. **Should `prefers_reduced_motion()` be feature-gated?**
   - What we know: `system_is_dark()` is gated on `#[cfg(target_os = "linux")]` only, with no feature gate. The macOS and Windows backends need their respective features.
   - What's unclear: Whether the public function should exist on all platforms (returning `false` on unsupported) or only on supported platforms.
   - Recommendation: Make it available unconditionally (like `from_system()` which exists on all platforms). Use internal `cfg` blocks to select the backend. This matches the requirements: "Returns false on unsupported platforms."

2. **KDE/XFCE/other Linux DEs: do they honor `enable-animations`?**
   - What we know: The gsettings `org.gnome.desktop.interface enable-animations` key is a global key that changes behavior of the window manager and panel. KDE Plasma, XFCE, and other DEs that use GTK applications generally respect it.
   - What's unclear: Whether KDE has its own separate animation setting.
   - Recommendation: Use the single gsettings key as specified in the requirements. This matches the `system_is_dark()` pattern which also uses gsettings as the primary source for all DEs.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | cargo test (built-in) |
| Config file | Cargo.toml (workspace) |
| Quick run command | `cargo test -p native-theme --lib` |
| Full suite command | `cargo test -p native-theme --features system-icons` |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| A11Y-01 | `prefers_reduced_motion()` returns bool, OnceLock caching | unit | `cargo test -p native-theme --lib prefers_reduced_motion` | Wave 0 |
| A11Y-02 | Linux gsettings query with inverted semantics | unit (platform-gated) | `cargo test -p native-theme --lib detect_reduced_motion` | Wave 0 |
| A11Y-03 | macOS NSWorkspace query | unit (platform-gated) | `cargo test -p native-theme --lib detect_reduced_motion --features macos` | Wave 0 |
| A11Y-04 | Windows UISettings query | unit (platform-gated) | `cargo test -p native-theme --lib detect_reduced_motion --features windows` | Wave 0 |
| A11Y-05 | Returns false on unsupported/failure | unit | `cargo test -p native-theme --lib prefers_reduced_motion` | Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test -p native-theme --lib`
- **Per wave merge:** `cargo test -p native-theme --features system-icons`
- **Phase gate:** Full suite green before verify

### Wave 0 Gaps
- [ ] Tests for `prefers_reduced_motion()` in lib.rs -- covers A11Y-01, A11Y-05
- [ ] Platform-gated tests for inner detection functions -- covers A11Y-02, A11Y-03, A11Y-04
- Note: Platform-specific tests (A11Y-02, A11Y-03, A11Y-04) can only run on their respective platforms. Cross-platform CI is the true validation.

## Sources

### Primary (HIGH confidence)
- Existing codebase: `lib.rs:187-225` (`system_is_dark()` + `detect_is_dark_inner()`) -- exact pattern to follow
- Existing codebase: `windows.rs:1-5` -- UISettings already imported with `UI_ViewManagement` feature
- Existing codebase: `Cargo.toml:39-44` -- objc2-app-kit features currently enabled
- Live system verification: `gsettings get org.gnome.desktop.interface enable-animations` outputs bare `true`/`false`
- Live system verification: `gsettings range org.gnome.desktop.interface enable-animations` confirms type `b` (boolean)

### Secondary (MEDIUM confidence)
- [Apple Developer: accessibilityDisplayShouldReduceMotion](https://developer.apple.com/documentation/appkit/nsworkspace/accessibilitydisplayshouldreducemotion) -- macOS 10.12+, Boolean property
- [Microsoft Learn: UISettings.AnimationsEnabled](https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uisettings.animationsenabled?view=winrt-26100) -- Windows 10+, Boolean property
- [docs.rs: objc2-app-kit NSWorkspace](https://docs.rs/objc2-app-kit/latest/objc2_app_kit/struct.NSWorkspace.html) -- Requires `NSWorkspace` + `NSAccessibility` features
- [windows-docs-rs: UISettings](https://microsoft.github.io/windows-docs-rs/doc/windows/UI/ViewManagement/struct.UISettings.html) -- `AnimationsEnabled(&self) -> Result<bool>`

### Tertiary (LOW confidence)
- None. All findings verified with primary or secondary sources.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - all dependencies already in Cargo.toml, only feature additions needed
- Architecture: HIGH - exact pattern (OnceLock + inner function) exists in codebase (`system_is_dark()`)
- Pitfalls: HIGH - verified gsettings output format on live system; semantic inversions documented with evidence

**Research date:** 2026-03-18
**Valid until:** 2026-04-18 (stable APIs, 30 days)
