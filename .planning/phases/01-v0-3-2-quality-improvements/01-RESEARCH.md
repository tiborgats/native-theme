# Phase 1: v0.3.2 Quality Improvements - Research

**Researched:** 2026-03-14
**Domain:** Rust API hygiene, caching, deprecation patterns, shell scripting
**Confidence:** HIGH

## Summary

This phase covers 7 targeted code quality improvements to the existing native-theme v0.3.1 codebase. No new features, no breaking changes, and no new dependencies. All changes are internal refactoring, documentation improvements, and API hygiene additions.

The codebase is well-structured and the spec (`docs/v0.3.2-quality-improvements.md`) provides exact code snippets for most changes. Research confirms all decisions are sound: `OnceLock` is available at the project's MSRV (1.94.0, far above the 1.70.0 stabilization), `with_alpha` has zero external callers (confirmed via grep), all Theme struct fields in gpui-component 0.5.1 are `pub` (enabling direct field assignment as an alternative to apply_config), and `jq` is the standard replacement for python3 JSON parsing in shell scripts.

**Primary recommendation:** Follow the spec implementation order directly. Each issue is independent and can be implemented as a separate commit. The gpui `to_theme` round-trip (Issue 5) should keep the current pattern with an improved comment since bypassing `apply_config` would lose its side effects (setting `light_theme`/`dark_theme` Rc, highlight_theme, and default fallbacks for None config fields).

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **Caching (Issue 1):** Use `std::sync::OnceLock` for `system_icon_theme()` and `system_is_dark()`. Do NOT cache `from_system()` or `from_linux()`. Trade-off accepted: theme changes after process start won't be picked up.
- **pick_variant (Issue 2):** Add `NativeTheme::pick_variant(&self, is_dark: bool) -> Option<&ThemeVariant>` to core. Keep existing free functions in connectors as deprecated thin wrappers for one release. Remove deprecated wrappers in v0.4.
- **colorize_svg docs (Issue 3):** Add doc comments explaining when to use `to_svg_handle_colored` vs `to_svg_handle`. Rename internal `colorize_svg` to `colorize_monochrome_svg`. Do NOT add XML parser complexity.
- **Dead wrappers (Issue 4):** Remove `lighten`, `darken`, and `with_alpha` from `derive.rs`. Inline trait calls directly in `active_color`.
- **to_theme round-trip (Issue 5):** Check if gpui-component exposes non-color field access. If not, keep current pattern but improve the explanatory comment. If yes, extract specific fields manually instead of overwrite-restore cycle.
- **#[must_use] (Issue 6):** Add to all listed public functions with descriptive messages. Also consider adding to `NativeTheme` and `IconData` types.
- **pre-release-check.sh (Issue 7):** Replace python3 with jq for cargo metadata parsing. Add jq availability check with pure-bash grep/sed fallback.

### Claude's Discretion
- Exact `#[must_use]` message wording
- Whether `with_alpha` has external callers (remove if not)
- Comment text for the to_theme round-trip explanation

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope.
</user_constraints>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| std::sync::OnceLock | std (stable 1.70+) | One-time initialization cache | Zero-dep, thread-safe, explicit init point |

No new dependencies required. All changes use existing std library features and the crate's existing dependency tree.

### Existing Dependencies (unchanged)
| Library | Version | Relevant To |
|---------|---------|-------------|
| gpui-component | 0.5.1 | Issue 5 (to_theme round-trip analysis) |
| gpui | 0.2.2 | Issue 4 (Hsla, Colorize trait) |
| serde | 1.0.228 | No changes needed |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| OnceLock | LazyLock | LazyLock requires closure in static; OnceLock keeps detection as normal fn (user decision: locked) |
| OnceLock | once_cell::sync::Lazy | External dep; OnceLock is std since 1.70 (user decision: locked) |

## Architecture Patterns

### Issue 1: OnceLock Caching Pattern

**What:** Wrap subprocess-spawning detection functions in `OnceLock` statics for process-lifetime caching.

**Where:**
- `native-theme/src/model/icons.rs` -- `system_icon_theme()` wraps `detect_linux_icon_theme()`
- `native-theme/src/lib.rs` -- `system_is_dark()` wraps its inline detection logic

**Pattern:**
```rust
// Source: std docs + spec
use std::sync::OnceLock;

static CACHED_ICON_THEME: OnceLock<String> = OnceLock::new();

pub fn system_icon_theme() -> String {
    // On non-Linux, returns compile-time constants -- no caching needed
    #[cfg(target_os = "linux")]
    {
        CACHED_ICON_THEME.get_or_init(|| detect_linux_icon_theme()).clone()
    }
    // macOS/Windows/other: compile-time constants, no cache needed
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    { "sf-symbols".to_string() }
    // ...etc
}
```

**Key details:**
- `system_icon_theme()` currently calls `detect_linux_icon_theme()` on Linux, which spawns subprocesses. Only the Linux path needs caching.
- `system_is_dark()` is Linux-only (`#[cfg(target_os = "linux")]`). Wrapping the entire function body in OnceLock works.
- The `detect_theme()` function in `freedesktop.rs` calls `crate::system_icon_theme()` (line 16), so caching `system_icon_theme` automatically caches all 42 icon lookups at startup.
- `.clone()` is needed because `OnceLock<String>` returns `&String`, and the function returns `String`.

### Issue 2: Method + Deprecated Free Function Pattern

**What:** Move `pick_variant` to `NativeTheme` as a method, deprecate connector free functions.

**Where:**
- `native-theme/src/model/mod.rs` -- add method to `impl NativeTheme`
- `connectors/native-theme-gpui/src/lib.rs` -- deprecate `pick_variant()`
- `connectors/native-theme-iced/src/lib.rs` -- deprecate `pick_variant()`

**Pattern:**
```rust
// In model/mod.rs
impl NativeTheme {
    /// Pick the appropriate variant for the given mode, with cross-fallback.
    pub fn pick_variant(&self, is_dark: bool) -> Option<&ThemeVariant> {
        if is_dark {
            self.dark.as_ref().or(self.light.as_ref())
        } else {
            self.light.as_ref().or(self.dark.as_ref())
        }
    }
}

// In connectors: thin wrappers with deprecation
#[deprecated(since = "0.3.2", note = "Use NativeTheme::pick_variant() instead")]
pub fn pick_variant(theme: &NativeTheme, is_dark: bool) -> Option<&ThemeVariant> {
    theme.pick_variant(is_dark)
}
```

**Key details:**
- Both connectors have identical implementations (confirmed by code review).
- The connectors' own `to_theme` functions call `pick_variant` -- those internal calls should use the method too.
- Existing tests in connectors use the free function; update to use method or leave as deprecation coverage.

### Issue 3: Documentation + Rename Pattern

**What:** Improve `colorize_svg` docs, rename to `colorize_monochrome_svg`.

**Where:** `connectors/native-theme-iced/src/icons.rs`

**Key details:**
- `colorize_svg` is a private (`fn`, not `pub fn`) function -- renaming is non-breaking.
- Only called by `to_svg_handle_colored` (line 56) and in tests (lines 148, 162, 183).
- Update all 3 call sites + add doc comments to the public functions.

### Issue 4: Dead Code Removal

**What:** Remove `lighten`, `darken`, `with_alpha` from `derive.rs`, inline `darken` call in `active_color`.

**Where:** `connectors/native-theme-gpui/src/derive.rs`

**Research finding (Claude's Discretion area):** `with_alpha` has ZERO external callers. The only import from `derive` is `use crate::derive::{active_color, hover_color};` in `colors.rs` (line 12). No other file imports `lighten`, `darken`, or `with_alpha`. All three should be removed.

**Pattern:**
```rust
// Before:
pub fn active_color(base: Hsla, is_dark: bool) -> Hsla {
    let factor = if is_dark { 0.2 } else { 0.1 };
    darken(base, factor)
}

// After:
pub fn active_color(base: Hsla, is_dark: bool) -> Hsla {
    let factor = if is_dark { 0.2 } else { 0.1 };
    base.darken(factor)  // direct Colorize trait call
}
```

**Test impact:** Remove tests for `lighten`, `darken`, `with_alpha` (5 tests: `lighten_increases_lightness`, `darken_decreases_lightness`, `lighten_preserves_hue_and_saturation`, `darken_preserves_hue_and_saturation`, `with_alpha_sets_alpha`). These test gpui_component::Colorize, not our code.

### Issue 5: to_theme Round-Trip

**What:** Improve the explanatory comment on the apply_config/restore pattern.

**Research finding:** After examining gpui-component 0.5.1 source:
- `Theme` struct has ALL fields `pub` (confirmed in `theme/mod.rs` lines 41-76)
- `Theme::apply_config` does 3 things beyond setting non-color fields:
  1. Sets `self.light_theme` or `self.dark_theme` (Rc<ThemeConfig>)
  2. Handles `highlight_theme` (Arc<HighlightTheme>)
  3. Applies `ThemeColor::apply_config` which overwrites ALL color fields (line 702)
- We COULD bypass `apply_config` and set fields directly, but we'd lose the `light_theme`/`dark_theme` Rc assignment and highlight theme handling.

**Recommendation:** Keep the current pattern. Bypassing `apply_config` would require reimplementing its non-color logic AND maintaining compatibility as gpui-component evolves. The current 3-line pattern (create, apply_config, restore colors) is safe and well-documented. Just improve the comment.

**Pattern (improved comment):**
```rust
// gpui-component's apply_config sets font_family, font_size, radius, shadow,
// mode, light_theme/dark_theme Rc, and highlight_theme. However, it also calls
// ThemeColor::apply_config which overwrites ALL color fields with defaults
// (since our ThemeConfig.colors is empty). We restore our mapped colors after.
// This is a known gpui-component API limitation, not a bug in our code.
let mut theme = Theme::from(&theme_color);
theme.apply_config(&theme_config.into());
theme.colors = theme_color;
```

### Issue 6: #[must_use] Annotations

**What:** Add `#[must_use]` with descriptive messages to public API functions.

**Where:**
- `native-theme/src/lib.rs`: `from_system()`, `from_system_async()`, `load_icon()`, `system_is_dark()`
- `native-theme/src/model/icons.rs`: `system_icon_set()`, `system_icon_theme()`
- `native-theme/src/model/bundled.rs`: `bundled_icon_svg()`, `bundled_icon_by_name()`
- `native-theme/src/model/mod.rs`: `NativeTheme::preset()`, `NativeTheme::from_toml()`, `NativeTheme::from_file()`, `NativeTheme::to_toml()`, `NativeTheme::list_presets()`, `NativeTheme::pick_variant()` (new from Issue 2)
- Types: `NativeTheme`, `IconData`

**Recommended message wording (Claude's Discretion):**
```rust
// Result-returning functions:
#[must_use = "this returns the detected theme; it does not apply it"]
pub fn from_system() -> crate::Result<NativeTheme> { ... }

#[must_use = "this returns the detected theme; it does not apply it"]
pub async fn from_system_async() -> crate::Result<NativeTheme> { ... }

#[must_use = "this returns the loaded icon data; it does not display it"]
pub fn load_icon(role: IconRole, icon_set: &str) -> Option<IconData> { ... }

#[must_use = "this returns whether the system uses dark mode"]
pub fn system_is_dark() -> bool { ... }

#[must_use = "this returns the current icon set for the platform"]
pub fn system_icon_set() -> IconSet { ... }

#[must_use = "this returns the current icon theme name"]
pub fn system_icon_theme() -> String { ... }

#[must_use = "this returns SVG bytes; it does not render the icon"]
pub fn bundled_icon_svg(set: IconSet, role: IconRole) -> Option<&'static [u8]> { ... }

#[must_use = "this returns SVG bytes; it does not render the icon"]
pub fn bundled_icon_by_name(set: IconSet, name: &str) -> Option<&'static [u8]> { ... }

// NativeTheme methods:
#[must_use = "this returns a theme preset; it does not apply it"]
pub fn preset(name: &str) -> crate::Result<Self> { ... }

#[must_use = "this parses a TOML string into a theme; it does not apply it"]
pub fn from_toml(toml_str: &str) -> crate::Result<Self> { ... }

#[must_use = "this loads a theme from a file; it does not apply it"]
pub fn from_file(path: impl AsRef<std::path::Path>) -> crate::Result<Self> { ... }

#[must_use = "this serializes the theme to TOML; it does not write to a file"]
pub fn to_toml(&self) -> crate::Result<String> { ... }

#[must_use = "this returns the list of preset names"]
pub fn list_presets() -> &'static [&'static str] { ... }

#[must_use = "this returns the selected variant; it does not apply it"]
pub fn pick_variant(&self, is_dark: bool) -> Option<&ThemeVariant> { ... }

// Types:
#[must_use = "constructing a theme without using it is likely a bug"]
pub struct NativeTheme { ... }

#[must_use = "loading icon data without using it is likely a bug"]
pub enum IconData { ... }
```

### Issue 7: Shell Script Fix

**What:** Replace python3 JSON parsing with jq (with bash fallback).

**Where:** `pre-release-check.sh` line 127-128.

**Pattern:**
```bash
# Get all workspace crate names
if command -v jq &>/dev/null; then
    WORKSPACE_CRATES=$(cargo metadata --no-deps --format-version 1 2>/dev/null \
        | jq -r '.packages[].name')
else
    WORKSPACE_CRATES=$(cargo metadata --no-deps --format-version 1 2>/dev/null \
        | grep -o '"name":"[^"]*"' | sed 's/"name":"//;s/"//')
fi
```

### Anti-Patterns to Avoid
- **Adding OnceLock to from_system() or from_linux():** These are explicitly NOT to be cached (user decision).
- **Breaking the connector API:** `pick_variant` free functions must remain as deprecated wrappers, not be removed yet.
- **Adding XML parsing to colorize_svg:** Explicitly rejected by user decision.
- **Bypassing apply_config entirely:** Would lose side effects (light_theme/dark_theme Rc, highlight theme).

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Thread-safe one-time init | Custom AtomicBool + Mutex | std::sync::OnceLock | Handles race conditions, panic safety |
| JSON parsing in bash | python3 one-liner | jq (with grep/sed fallback) | jq is standard, lighter than python3 runtime |

**Key insight:** All improvements in this phase use existing standard library features or ecosystem tools. No custom solutions needed.

## Common Pitfalls

### Pitfall 1: OnceLock on Non-Linux Platforms
**What goes wrong:** Adding OnceLock caching around code that only runs on Linux, but the static is defined unconditionally.
**Why it happens:** `system_icon_theme()` has `#[cfg]` branches for each platform. Caching only matters on Linux (subprocess spawning).
**How to avoid:** Either: (a) wrap only the Linux branch in OnceLock, or (b) define the OnceLock static unconditionally but only use `get_or_init` in the Linux branch. Option (a) is cleaner since macOS/Windows return compile-time strings.
**Warning signs:** `unused import` or `dead_code` warnings on non-Linux platforms.

### Pitfall 2: Deprecation Attribute Syntax
**What goes wrong:** Putting `#[deprecated]` on a function that's also `pub fn` in `impl` block vs. free function -- different syntax rules.
**Why it happens:** `#[deprecated]` on free functions is straightforward; on methods it's the same but can be confused with trait methods.
**How to avoid:** Both connectors use free functions (not methods), so `#[deprecated(since = "0.3.2", note = "...")]` directly on the `pub fn` works.

### Pitfall 3: Test Breakage from derive.rs Removal
**What goes wrong:** Removing `lighten`, `darken`, `with_alpha` but forgetting to remove their tests.
**Why it happens:** Tests are in the same file but in a `#[cfg(test)] mod tests` block.
**How to avoid:** Remove the 5 tests that test the removed functions. Keep `hover_color_*` and `active_color_*` tests.

### Pitfall 4: colorize_svg Rename Missed Call Sites
**What goes wrong:** Renaming `colorize_svg` to `colorize_monochrome_svg` but missing a call site.
**Why it happens:** Function is private so the compiler will catch it, but tests also call it directly.
**How to avoid:** The compiler will error on any missed references. There are exactly 3 call sites: line 56 (production), lines 148 and 162 (tests), and line 183 (test).

### Pitfall 5: #[must_use] on Types vs Functions
**What goes wrong:** Adding `#[must_use]` to `NativeTheme` struct triggers warnings in test code where themes are constructed and dropped.
**Why it happens:** `#[must_use]` on types warns whenever the type is created but not bound/used.
**How to avoid:** Check that existing tests don't create NativeTheme values without using them. Review test output for new warnings. Tests that call `NativeTheme::default()` without using the result would need `let _ = ...`.

### Pitfall 6: grep/sed Fallback in Shell Script
**What goes wrong:** The bash fallback `grep -o '"name":"[^"]*"'` doesn't handle JSON with spaces in key formatting.
**Why it happens:** `cargo metadata --format-version 1` outputs compact JSON (no spaces around colons), so this specific pattern is safe.
**How to avoid:** Test with actual `cargo metadata` output from the workspace. The 3 crate names (native-theme, native-theme-gpui, native-theme-iced) all have simple ASCII names.

## Code Examples

### OnceLock for system_is_dark()
```rust
// Source: std docs, verified against MSRV 1.94.0
use std::sync::OnceLock;

static CACHED_IS_DARK: OnceLock<bool> = OnceLock::new();

#[cfg(target_os = "linux")]
pub fn system_is_dark() -> bool {
    *CACHED_IS_DARK.get_or_init(|| detect_is_dark_inner())
}

// Extract current body of system_is_dark() into this function
#[cfg(target_os = "linux")]
fn detect_is_dark_inner() -> bool {
    // ... existing gsettings + kdeglobals detection logic ...
}
```

### Deprecated free function wrapper
```rust
// Source: Rust reference, #[deprecated] attribute
#[deprecated(since = "0.3.2", note = "Use NativeTheme::pick_variant() instead")]
pub fn pick_variant(theme: &NativeTheme, is_dark: bool) -> Option<&ThemeVariant> {
    theme.pick_variant(is_dark)
}
```

### #[must_use] on struct
```rust
// Source: Rust reference
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[non_exhaustive]
#[must_use = "constructing a theme without using it is likely a bug"]
pub struct NativeTheme {
    pub name: String,
    pub light: Option<ThemeVariant>,
    pub dark: Option<ThemeVariant>,
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `once_cell::sync::Lazy` | `std::sync::OnceLock` (1.70) / `LazyLock` (1.80) | Rust 1.70 (2023) | No external dependency needed |
| Python3 for JSON in shell | `jq` | Widespread by 2020 | Lighter runtime dependency |

**Deprecated/outdated:**
- `once_cell` crate: Still maintained but `OnceLock`/`LazyLock` are in std now. Use std versions for new code.
- `lazy_static!` macro: Superseded by `LazyLock`. Not relevant here since we use `OnceLock`.

## Open Questions

1. **gpui-component apply_config future changes**
   - What we know: v0.5.1 `apply_config` overwrites all colors and sets non-color fields. All Theme fields are pub.
   - What's unclear: Whether future gpui-component versions might add more side effects to `apply_config`.
   - Recommendation: Keep the apply_config pattern with improved comments. It's more future-proof than bypassing it.

2. **#[must_use] on NativeTheme type -- test impact**
   - What we know: Adding `#[must_use]` to the type will warn on any unused NativeTheme value, including in tests.
   - What's unclear: Whether any existing tests create NativeTheme without using the value.
   - Recommendation: Add `#[must_use]` to the type; fix any resulting test warnings with `let _theme = ...` or actual assertions.

## Validation Architecture

> `workflow.nyquist_validation` not set in config.json -- treating as enabled.

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in (`#[test]`, cargo test) |
| Config file | Cargo workspace (Cargo.toml) |
| Quick run command | `cargo test -p native-theme` |
| Full suite command | `cargo test --workspace` |

### Phase Requirements to Test Map
| Issue | Behavior | Test Type | Automated Command | File Exists? |
|-------|----------|-----------|-------------------|-------------|
| 1 - Caching | `system_icon_theme()` returns cached value | unit | `cargo test -p native-theme system_icon_theme` | Existing tests cover return value; caching is transparent |
| 1 - Caching | `system_is_dark()` returns cached value | unit | `cargo test -p native-theme system_is_dark` | No explicit cache test; behavior is transparent |
| 2 - pick_variant | `NativeTheme::pick_variant` works correctly | unit | `cargo test -p native-theme pick_variant` | New test needed in `model/mod.rs` |
| 2 - pick_variant | Deprecated free functions still work | unit | `cargo test -p native-theme-gpui pick_variant && cargo test -p native-theme-iced pick_variant` | Existing tests cover this |
| 3 - colorize_svg docs | Renamed function compiles | unit | `cargo test -p native-theme-iced colorize` | Existing tests use old name; update references |
| 4 - Dead wrappers | `active_color` uses direct trait calls | unit | `cargo test -p native-theme-gpui active_color` | Existing tests cover behavior |
| 5 - to_theme round-trip | `to_theme` produces valid theme | unit | `cargo test -p native-theme-gpui to_theme` | Existing test covers this |
| 6 - #[must_use] | Compilation succeeds with no new warnings | build | `cargo check --workspace` | N/A (compile-time) |
| 7 - pre-release-check.sh | Script runs without python3 | manual | `bash pre-release-check.sh` | Manual-only; script is interactive |

### Sampling Rate
- **Per task commit:** `cargo test -p <affected-crate> --lib`
- **Per wave merge:** `cargo test --workspace`
- **Phase gate:** Full workspace test suite green + `cargo clippy --workspace -- -D warnings`

### Wave 0 Gaps
- [ ] `native-theme/src/model/mod.rs` -- add tests for `NativeTheme::pick_variant()` (light fallback, dark fallback, both present, empty)
- [ ] Verify `#[must_use]` doesn't cause warnings in existing test code

## Sources

### Primary (HIGH confidence)
- **Direct codebase analysis** -- all source files read and analyzed:
  - `native-theme/src/lib.rs` -- `system_is_dark()`, `from_system()`, `from_system_async()`, `load_icon()`
  - `native-theme/src/model/icons.rs` -- `system_icon_theme()`, `system_icon_set()`, `detect_linux_icon_theme()`
  - `native-theme/src/model/mod.rs` -- `NativeTheme`, `ThemeVariant`
  - `native-theme/src/model/bundled.rs` -- `bundled_icon_svg()`, `bundled_icon_by_name()`
  - `native-theme/src/freedesktop.rs` -- `detect_theme()` calls `system_icon_theme()`
  - `connectors/native-theme-gpui/src/lib.rs` -- `pick_variant()`, `to_theme()`
  - `connectors/native-theme-gpui/src/derive.rs` -- `lighten`, `darken`, `with_alpha`, `active_color`, `hover_color`
  - `connectors/native-theme-gpui/src/colors.rs` -- imports from derive
  - `connectors/native-theme-gpui/src/config.rs` -- `to_theme_config()`
  - `connectors/native-theme-iced/src/lib.rs` -- `pick_variant()`, `to_theme()`
  - `connectors/native-theme-iced/src/icons.rs` -- `colorize_svg()`, `to_svg_handle_colored()`
  - `pre-release-check.sh` -- python3 usage on line 128
- **gpui-component 0.5.1 source** (cargo registry cache):
  - `theme/mod.rs` -- Theme struct definition, all fields pub
  - `theme/schema.rs` -- `apply_config()` implementation, `ThemeColor::apply_config()` overwrites all colors
- **Rust std docs** -- `OnceLock` stable since 1.70.0, `#[must_use]` attribute, `#[deprecated]` attribute
- **Project Cargo.toml** -- MSRV 1.94.0, edition 2024, workspace structure

### Secondary (MEDIUM confidence)
- None needed -- all findings verified from primary sources.

### Tertiary (LOW confidence)
- None.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- using only std library features, no new deps
- Architecture: HIGH -- all patterns verified against actual source code
- Pitfalls: HIGH -- identified from actual code analysis, not theoretical

**Research date:** 2026-03-14
**Valid until:** 2026-04-14 (stable codebase, no external dependency changes expected)
