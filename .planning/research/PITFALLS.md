# Pitfalls Research

**Domain:** v0.2 feature additions to cross-platform Rust OS theme data crate
**Researched:** 2026-03-08
**Confidence:** HIGH (verified via official docs, crate source analysis, and community post-mortems)

This document covers pitfalls specific to v0.2 features: macOS reader (objc2/objc2-app-kit), struct flattening migration, Cargo workspace restructuring, widget metrics, gpui-component connector, iced connector, cargo-semver-checks, and crates.io publishing. v0.1 pitfalls (ashpd tokio leak, configparser case sensitivity, merge desync) are already resolved and documented in v0.1 planning -- they are not repeated here.

---

## Critical Pitfalls

### Pitfall 1: NSColor Semantic Colors Return Garbage Without Appearance Context

**What goes wrong:**
macOS semantic colors (e.g., `controlAccentColor`, `labelColor`, `selectedContentBackgroundColor`) are *dynamic* -- their resolved RGBA values depend on `NSAppearance.current`. When reading these colors from a Rust binary without a running NSApplication or drawing context, `NSAppearance.current` is nil or stale. Calling `colorUsingColorSpace:` on the raw color returns the *initial appearance's* values regardless of whether the system is actually in dark mode. The reader silently returns light-mode colors even when the system is in dark mode, or vice versa.

**Why it happens:**
AppKit resolves dynamic colors lazily via `NSAppearance.current`, which is an ephemeral thread-local set during drawing operations. A Rust CLI or GUI toolkit that does not use NSApplication's run loop never sets this correctly. The v0.1 pitfalls document covered the `colorUsingColorSpace` crash (P3 to sRGB), but this is a different bug: the conversion *succeeds* but returns the wrong variant's colors.

**How to avoid:**
Before extracting any semantic color, explicitly set the appearance context:
```rust
// macOS 11+: performAsCurrentDrawingAppearance
// macOS 10.14-10.15: manually set NSAppearance.current
let app = NSApplication::sharedApplication();
let appearance = app.effectiveAppearance();
unsafe {
    NSAppearance::setCurrentAppearance(Some(&appearance));
}
// NOW read colors -- they will resolve for the current system appearance
let srgb = color.colorUsingColorSpace(&NSColorSpace::sRGBColorSpace());
// Restore previous appearance afterward
unsafe {
    NSAppearance::setCurrentAppearance(previous.as_deref());
}
```
The `from_macos()` function must set the appearance context once at the top, read all colors, then restore it. Do NOT set/restore per-color -- that is both slow and risks partial reads with inconsistent appearance.

**Warning signs:**
- `from_macos()` always returns light-mode colors regardless of system setting
- Colors match when tested in a full Cocoa app but differ when called from a standalone binary
- Works on one macOS version but not another (appearance resolution behavior changed in macOS 11)

**Phase to address:**
Step 2 (Platform Readers -- macOS reader). This must be the very first thing the macOS reader handles, before any color extraction.

---

### Pitfall 2: objc2 Autorelease Pool Leak in Non-AppKit Context

**What goes wrong:**
Every `msg_send!` call that returns an Objective-C object may place it in the current autorelease pool. In a normal AppKit application, the run loop drains this pool each iteration. A Rust binary calling `from_macos()` has no run loop. Without an explicit `autoreleasepool` block, every `NSColor` method call leaks its return value until the thread exits. Calling `from_macos()` in a loop (e.g., polling for theme changes) accumulates unbounded memory.

**Why it happens:**
objc2's `Retained<T>` handles reference counting correctly for *retained* returns, but many AppKit methods return *autoreleased* objects. The `Retained::retain_autoreleased` optimization helps in some cases, but not all AppKit methods follow the standard naming conventions that enable it.

**How to avoid:**
Wrap the entire `from_macos()` body in an autorelease pool:
```rust
pub fn from_macos() -> crate::Result<crate::NativeTheme> {
    objc2::rc::autoreleasepool(|_pool| {
        // All NSColor/NSFont/NSAppearance calls go here
        // Pool is drained when this closure returns
    })
}
```
This is a single wrapper around all ObjC calls. Do NOT create a pool per-color -- the overhead is unnecessary and the pool boundary is the function call, not individual color reads.

**Warning signs:**
- Memory usage grows steadily when `from_macos()` is called repeatedly
- Instruments shows growing autorelease pool allocations
- No crash or error -- pure silent memory leak

**Phase to address:**
Step 2 (Platform Readers -- macOS reader). Autorelease pool must wrap the entire reader function from day one.

---

### Pitfall 3: Flattening ThemeColors Breaks All 17 Preset TOML Files and Downstream Parsers

**What goes wrong:**
v0.1 ships with nested TOML format: `[light.colors.core]`, `[light.colors.status]`, etc. (7 sub-tables per variant). v0.2 flattens to `[light.colors]` with 36 direct keys. Every one of the 17 preset TOML files must be rewritten. Any downstream user who saved themes in v0.1 format, or wrote TOML files by hand following v0.1 examples, gets a deserialization error with zero guidance.

The specific collision risk: flattening brings `background` from `CoreColors` and `background` from `ActionColors` (for primary/secondary) into the same namespace. The current struct has `core.background`, `primary.background`, and `secondary.background` -- three fields all named `background`. Flattening without renaming causes a compile error from serde (duplicate field names).

**Why it happens:**
The todo.md correctly identifies the flattening as desirable for API simplicity, but does not address the field name collision or the TOML migration path. The nested sub-struct reuses `ActionColors` for both `primary` and `secondary`, each with `background` and `foreground` fields.

**How to avoid:**
1. Rename fields during flattening to disambiguate:
   - `core.background` becomes `background`
   - `primary.background` becomes `primary`  (the "primary action color" IS the primary background)
   - `primary.foreground` becomes `primary_foreground`
   - `secondary.background` becomes `secondary`
   - `secondary.foreground` becomes `secondary_foreground`
   - `core.accent` stays `accent`, `core.foreground` stays `foreground`, etc.
2. Update all 17 preset TOML files in the same commit as the struct change -- never leave them out of sync.
3. Since this is pre-1.0, there is no backward compatibility obligation. Document the migration in CHANGELOG.md with a before/after example.
4. Do NOT use `#[serde(flatten)]` on sub-structs for backward compat -- it has known issues with TOML (panics with `toml::Value`, conflicts with `deny_unknown_fields`). Do a clean break: new struct with 36 fields.

**Warning signs:**
- Serde compile error about duplicate field names
- One preset compiles but another fails (inconsistent field naming across presets)
- `to_toml()` output has ambiguous keys like multiple `background` entries

**Phase to address:**
Step 1 (API Refactors). This is the very first task and must be done atomically -- struct change + all preset rewrites + all reader updates + all test updates in one logical change.

---

### Pitfall 4: Cargo Workspace Restructuring Breaks include_str! Paths and README Doctests

**What goes wrong:**
Moving the core crate from repo root into `native-theme/` subdirectory changes the relative path context for `include_str!` and `#[doc = include_str!("../README.md")]`. The `include_str!` macro resolves paths relative to the source file invoking it. The preset embeds in `src/presets.rs` use `include_str!("presets/default.toml")` which resolves relative to `src/` -- these survive the move because the `src/` directory moves as a unit. But `#[doc = include_str!("../README.md")]` in `src/lib.rs` points one directory up, which after the workspace move points to the *crate* directory, not the repo root. If README.md lives at repo root, this breaks.

Additionally, `cargo publish` packages only files within the crate directory. A README.md at the repo root will not be included in the published crate unless explicitly referenced via `package.readme` in Cargo.toml, or copied/symlinked into the crate directory.

**Why it happens:**
Developers restructure the directory layout but don't verify all `include_str!` paths, doctests, and `cargo publish --dry-run` packaging. The build succeeds locally (because the repo root README exists on disk) but `cargo publish` fails (because it packages from the crate subdirectory).

**How to avoid:**
1. Place a README.md inside the `native-theme/` crate directory (can be a symlink or a separate copy with crate-specific content).
2. Update `src/lib.rs` to point to the crate-local README: `#[doc = include_str!("../README.md")]` (this now correctly points to `native-theme/README.md`).
3. Run `cargo publish --dry-run` immediately after restructuring to catch any missing files.
4. Set `package.readme = "README.md"` in the crate's Cargo.toml explicitly.
5. Verify that `package.include` (if used) does not accidentally exclude preset TOML files.

**Warning signs:**
- `cargo publish --dry-run` fails with "file not found" errors
- Doctests pass locally but fail in CI (different working directory)
- Published crate on crates.io has no documentation or broken doc links

**Phase to address:**
Step 5 (Workspace restructuring). Run `cargo publish --dry-run` as the acceptance test for this step.

---

### Pitfall 5: Widget Metrics Platform Granularity Mismatch Creates False Precision

**What goes wrong:**
KDE Breeze defines ~80 widget metric constants (exact pixel values for button padding, checkbox indicator size, scrollbar groove width, etc.). GNOME/Adwaita exposes ~30 CSS variables. macOS HIG gives rough guidelines (~10 values). Windows `GetSystemMetrics` returns ~20 values but most are for non-client area (title bar, borders), not widget internals. A unified `WidgetMetrics` struct with 12 sub-structs and many fields will be `Some(...)` for KDE, `Some(...)` for some GNOME fields, and `None` for most macOS/Windows fields. Downstream consumers check `if let Some(padding) = metrics.button.padding_horizontal` and silently fall back to their own defaults on macOS/Windows, negating the value of widget metrics on those platforms.

Worse: hardcoding macOS HIG values (like 10pt corner radius) as concrete `Some(10.0)` creates false precision. Apple changes HIG values across macOS versions without a public changelog. The hardcoded values become stale silently.

**Why it happens:**
Platform APIs expose different levels of detail. The desire for a "complete" struct leads to filling in hardcoded values for platforms that don't expose them, creating an illusion of runtime accuracy.

**How to avoid:**
1. Be honest about coverage: return `None` for values the platform does not expose at runtime. Document the expected coverage matrix per platform.
2. For KDE: version-detect and select the matching hardcoded constant set (as the todo.md correctly describes). Version detection via `plasmashell --version` or reading `/usr/share/knotifications5/plasma_version` at runtime.
3. For GNOME: version-detect libadwaita via `pkg-config --modversion libadwaita-1` or parsing the CSS variable source.
4. For Windows: use `GetSystemMetricsForDpi()` instead of `GetSystemMetrics()` to get DPI-correct values (see Pitfall 6).
5. For macOS: keep the set small and well-sourced. Only hardcode values that are documented in HIG and have been stable across 3+ macOS releases.
6. Add a `WidgetMetrics::coverage()` method that returns the fraction of non-None fields -- helps downstream consumers decide whether to use the data.

**Warning signs:**
- Widget metrics are `Some` on Linux but `None` on macOS/Windows for the same field
- Hardcoded macOS values don't match actual measurements on latest macOS
- Downstream code has `unwrap_or(default)` for every metric, meaning native-theme adds no value

**Phase to address:**
Step 3 (Widget Metrics). Design the coverage matrix BEFORE implementing any reader. Decide which fields to expose per platform and document it.

---

### Pitfall 6: Windows GetSystemMetrics Returns Wrong Values Without DPI Awareness

**What goes wrong:**
`GetSystemMetrics()` returns values scaled for the *primary monitor's DPI at process startup*. If the application runs on a non-primary monitor, or if DPI settings changed since login, the values are wrong. A Rust binary compiled without a DPI awareness manifest (the default for Rust) is treated as "DPI unaware" by Windows, causing the OS to lie about metrics -- returning 96 DPI values even on 144 or 192 DPI displays. The existing v0.1 `from_windows()` uses `GetSystemMetrics(SM_CXBORDER)` and `GetSystemMetrics(SM_CXVSCROLL)` and will return physically incorrect pixel counts on high-DPI displays.

**Why it happens:**
Rust binaries do not include a Windows application manifest by default. Without declaring DPI awareness, Windows applies "DPI virtualization" -- returning fake 96-DPI values. Developers testing on 100% scaling never notice.

**How to avoid:**
Use the DPI-aware variant: `GetSystemMetricsForDpi(metric, dpi)` where `dpi` is obtained from the target monitor or from `GetDpiForSystem()`. This requires Windows 10 1607+.
```rust
use windows::Win32::UI::HiDpi::GetSystemMetricsForDpi;
use windows::Win32::UI::HiDpi::GetDpiForSystem;

let dpi = unsafe { GetDpiForSystem() };
let scroll_width = unsafe { GetSystemMetricsForDpi(SM_CXVSCROLL, dpi) };
```
Add `Win32_UI_HiDpi` to the `windows` crate feature flags. Document that the crate returns logical pixels at system DPI, not physical pixels.

For v0.2, also update the existing `read_geometry()` function to use `GetSystemMetricsForDpi`.

**Warning signs:**
- Border width reports as 1px on a 200% scaling display (should be 2px in logical, or 4px physical)
- Widget metrics match on 100% scaling but are halved on 200% scaling
- Values differ between `from_windows()` and what the user sees in Windows Settings

**Phase to address:**
Step 2 (Platform Readers -- Windows enhancements). Fix existing `read_geometry()` AND use DPI-aware APIs for new widget metrics.

---

### Pitfall 7: gpui-component API Instability Breaks Connector on Every Upstream Update

**What goes wrong:**
gpui-component is at v0.5.1 and actively developed (1,697 commits, frequent releases). The theme API has changed multiple times. A connector crate pinned to `gpui-component = "0.5"` will break when 0.6 ships with renamed color tokens or restructured `ThemeColor` fields. Because gpui-component's internal `ThemeColor` struct and `ActiveTheme` trait are the integration surface, any change to token names, type signatures, or the theme application mechanism directly breaks `native-theme-gpui`.

A specific risk: gpui-component uses shadcn/ui-inspired color scales (16 color families x 11 shades = 176 color values), while native-theme provides 36 semantic roles. The mapping is inherently lossy and assumes specific token names that may be renamed upstream.

**Why it happens:**
Pre-1.0 crates have no stability guarantees. The connector must map between two independently evolving APIs. Each upstream release potentially invalidates the mapping.

**How to avoid:**
1. Pin to an exact version range: `gpui-component = ">=0.5.1, <0.6"`. Do NOT use `"0.5"` which allows any 0.5.x (where 0.5.x can have breaking changes per semver for pre-1.0).
2. Isolate the mapping in a single `mapping.rs` file with clear comments showing which gpui-component token each native-theme field maps to. When upstream changes, only this file needs updating.
3. Add a CI job that builds the connector against gpui-component `main` branch (separate from release CI) to get early warning of breakage.
4. Document the minimum tested gpui-component version in the connector's README.
5. Accept that the connector will require maintenance releases tracking upstream. Budget for this.

**Warning signs:**
- CI fails after a gpui-component release with "method not found" or "type mismatch"
- Color tokens in gpui-component renamed from e.g., `primary` to `accent`
- `ThemeRegistry` API changes how themes are applied

**Phase to address:**
Step 5 (Toolkit Connectors). Build the connector against a pinned version, document the dependency version, set up breakage detection CI.

---

### Pitfall 8: iced Theme System Requires Implementing the Base Trait, Not Just Providing Colors

**What goes wrong:**
iced 0.14 uses a `Base` trait with five required methods: `default(preference: Mode)`, `mode()`, `base()`, `palette()`, `name()`. A naive connector that just creates an `iced::Theme::Custom` with a palette will not work correctly because:
1. iced's built-in widgets use `Catalog` implementations (class-based theming from 0.13+), not direct color access.
2. The `Base` trait is not dyn-compatible, so the connector cannot use trait objects.
3. Per-widget styling in iced uses closures that receive the `Theme` and widget `Status` -- the connector must provide these closures for every widget type it wants to style.

Simply providing colors without implementing the per-widget `Catalog` entries means widgets fall back to iced's default styling, ignoring the native-theme colors entirely.

**Why it happens:**
iced's styling system evolved significantly across 0.12, 0.13, and 0.14. The old `StyleSheet` trait approach was replaced by closure-based and class-based theming. Documentation lags behind the API changes. Developers read outdated examples and assume they can just set a palette.

**How to avoid:**
1. Target iced 0.14 specifically (current stable). Do not attempt backward compatibility with 0.13.
2. Create a custom theme type that wraps `iced::Theme` and implements `Base`:
   ```rust
   pub struct NativeIcedTheme {
       inner: iced::Theme,
       native: NativeTheme,
   }
   impl iced::widget::theme::Base for NativeIcedTheme {
       fn default(preference: Mode) -> Self { /* ... */ }
       fn mode(&self) -> Mode { /* ... */ }
       fn base(&self) -> Style { /* ... */ }
       fn palette(&self) -> Option<Palette> { /* ... */ }
       fn name(&self) -> &str { /* ... */ }
   }
   ```
3. Implement `Catalog` for key widgets (button, container, text_input, scrollable, etc.) by delegating to closures that read from native-theme colors.
4. Study COSMIC Desktop's iced theming code (cosmic-theme crate) as proven reference for iced-based native theming at scale.
5. Pin to `iced = "0.14"` exactly. iced 0.15 will likely change the styling system again.

**Warning signs:**
- Widgets render with default iced colors despite the theme being "applied"
- Compile errors about `Base` not being implemented for the custom theme type
- Widget state-dependent colors (hover, pressed, disabled) not changing

**Phase to address:**
Step 5 (Toolkit Connectors -- iced). Prototype the `Base` implementation first before building any widget-specific styling.

---

## Technical Debt Patterns

Shortcuts that seem reasonable but create long-term problems.

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Using `GetSystemMetrics` instead of `GetSystemMetricsForDpi` | Simpler code, fewer feature flags | Wrong values on all high-DPI displays | Never -- most modern Windows displays are high-DPI |
| Hardcoding macOS HIG metric values as `Some(x)` | Fills out the struct completely | False precision; values silently stale across macOS releases | Only for values documented as stable in HIG for 3+ releases |
| Mapping native-theme colors to gpui-component by token name strings | Fast to implement | Breaks silently if upstream renames tokens | Never -- use typed references where possible, string names as last resort |
| Flattening ThemeColors with `#[serde(flatten)]` for backward compat | Reads both old and new format | Known serde issues with flatten + TOML; panics with `toml::Value`; cannot use `deny_unknown_fields` | Never -- do a clean break for pre-1.0 |
| Skipping `cargo publish --dry-run` after workspace restructuring | Saves CI time | Broken publish discovered at release time | Never |
| Pinning gpui-component to git branch instead of version | Gets latest features | Build breaks unpredictably; not reproducible | Only during active development; switch to version pin before release |
| Generating a single `WidgetMetrics` struct covering all platforms | One struct to learn | Mostly `None` on macOS/Windows; consumers wrap everything in `unwrap_or` | Acceptable if coverage matrix is documented honestly |

## Integration Gotchas

Common mistakes when connecting to external APIs and crates.

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| objc2-app-kit NSColor | Reading dynamic colors without setting `NSAppearance.current` | Set appearance context via `NSAppearance::setCurrentAppearance` before reading, restore after |
| objc2-app-kit NSColor | Calling `colorUsingColorSpace:` which can return nil | Check return value; nil means conversion impossible (rare but real for some pattern colors) |
| objc2 autorelease pool | No autorelease pool in non-AppKit context | Wrap entire `from_macos()` in `objc2::rc::autoreleasepool` |
| objc2-app-kit NSFont | Assuming `systemFont(ofSize:)` returns a font with a human-readable family name | System font returns `.AppleSystemUIFont` (a private name); use `NSFont::systemFontOfSize` and extract the display name via `displayName()`, or hardcode "San Francisco" / ".SF NS" |
| Windows `GetSystemMetrics` | Using non-DPI-aware variant | Use `GetSystemMetricsForDpi` with `GetDpiForSystem()` |
| Windows `SystemParametersInfo` | Assuming `NONCLIENTMETRICS` struct size is constant | Size differs between Windows versions; set `cbSize` correctly based on compile-time target |
| Windows `ApiInformation::IsMethodPresent` | Not checking before calling newer WinRT APIs | Check presence; gracefully degrade to `None` for missing methods |
| gpui-component `ActiveTheme` | Assuming `cx.theme()` returns a mutable reference | `ActiveTheme` provides read-only access; theme changes go through `ThemeRegistry` |
| gpui-component color mapping | Mapping `native-theme::accent` to `gpui::primary` 1:1 | gpui-component uses 11-shade scales (50-950) per color; need to generate a full scale from a single accent color |
| iced `Base` trait | Implementing only `palette()` and expecting widgets to pick it up | Must implement per-widget `Catalog` entries; `palette()` is only for debug overlays |
| iced `Theme::Custom` | Creating a custom theme at runtime and expecting hot-reload | `Theme::Custom` is immutable once created; rebuild and re-apply on theme change |
| cargo-semver-checks first run | Expecting it to work before first crates.io publish | No baseline exists; either skip the check for initial publish or provide `--baseline-rev` pointing to the git tag of the first release |
| crates.io metadata | Omitting `rust-version` field | `cargo publish` warns; docs.rs may build with wrong MSRV; set `rust-version = "1.85"` (edition 2024 minimum) |
| crates.io `package.include` | Including preset TOML files via `include` but forgetting to add new presets | Use explicit `include` list or verify with `cargo package --list` |

## Performance Traps

Patterns that work at small scale but fail as usage grows.

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Creating autorelease pool per-color in macOS reader | 20x overhead for ~20 color reads | Single pool wrapping entire `from_macos()` | Always measurable; ~1ms vs ~20ms |
| Calling `plasmashell --version` synchronously for KDE version detection | 200-500ms subprocess spawn per call | Cache the version; it does not change during a session | On first call; subsequent calls should use cached value |
| Shelling out to `pkg-config` for libadwaita version | Same subprocess overhead | Cache or read from `/usr/lib/*/pkgconfig/libadwaita-1.pc` directly | On first call |
| Generating full 11-shade color scale for gpui-component mapping | CPU cost of Oklch interpolation for 16 color families x 11 shades | Compute lazily or precompute once and cache | Not a real problem at theme-load time; only matters if called per-frame |
| Publishing workspace crates in wrong order | `cargo publish` fails with "dependency not found" | Publish `native-theme` core first, then connectors | Every publish cycle |

## "Looks Done But Isn't" Checklist

Things that appear complete but are missing critical pieces.

- [ ] **macOS reader:** Colors resolve correctly -- but only tested in one appearance mode. Verify dark mode AND light mode return different values for `labelColor`, `controlBackgroundColor`, etc.
- [ ] **macOS reader:** NSFont returns a family name -- but it's `.AppleSystemUIFont`, not a user-facing name like "SF Pro". Verify the font family name is what a GUI toolkit can actually use.
- [ ] **macOS reader:** autorelease pool wraps function -- but verify with Instruments that no ObjC objects leak after 1000 calls.
- [ ] **Struct flattening:** All 17 presets compile -- but verify there are no field name collisions. Grep all preset TOML files for duplicate keys within a single `[colors]` section.
- [ ] **Struct flattening:** `impl_merge!` macro updated -- but verify the macro handles 36 flat fields (no nested delegation). The existing `nested { ... }` arm of the macro is no longer used for colors.
- [ ] **Workspace restructuring:** `cargo build` works -- but `cargo publish --dry-run` may fail due to missing files. Run it.
- [ ] **Workspace restructuring:** README doctest path updated -- but verify `cargo test --doc` passes from the crate subdirectory, not just repo root.
- [ ] **Widget metrics:** KDE metrics populated -- but verify the version detection actually picks the right constant set. Test with KDE Plasma 5 AND Plasma 6 installations.
- [ ] **Widget metrics:** Windows `GetSystemMetricsForDpi` called -- but verify the `Win32_UI_HiDpi` feature is enabled in the `windows` crate dependency.
- [ ] **gpui connector:** Colors mapped -- but verify the 11-shade scale generation produces visually correct results. A single `accent = #3daee9` should produce a usable light-to-dark gradient, not 11 identical shades.
- [ ] **iced connector:** `Base` trait implemented -- but verify widgets actually USE the custom palette. Drop a Button, TextInput, and Container into a test app and screenshot both iced-default and native-theme styling.
- [ ] **cargo-semver-checks in CI:** Job runs -- but on the first publish there is no baseline. The CI job must handle the "no previous version" case without failing the build.
- [ ] **crates.io publish:** Metadata complete -- but verify `keywords` has at most 5 entries (crates.io limit) and `categories` uses [valid slugs](https://crates.io/category_slugs).
- [ ] **crates.io publish:** `cargo package --list` shows all files -- but verify preset TOML files are included and `.planning/` directory is excluded.

## Recovery Strategies

When pitfalls occur despite prevention, how to recover.

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| NSColor appearance context not set (Pitfall 1) | LOW | Add `setCurrentAppearance` call at top of `from_macos()`; single function change |
| Autorelease pool missing (Pitfall 2) | LOW | Wrap function body in `autoreleasepool { }` block; no API change |
| Field name collision during flattening (Pitfall 3) | MEDIUM | Must rename fields and update all 17 presets + all readers + all tests; but pre-1.0 so no compatibility concern |
| include_str path broken after workspace move (Pitfall 4) | LOW | Update path in source file; but must also fix `cargo publish --dry-run` |
| Widget metrics DPI wrong on Windows (Pitfall 6) | LOW | Replace `GetSystemMetrics` with `GetSystemMetricsForDpi`; add one feature flag to `windows` dep |
| gpui connector breaks on upstream update (Pitfall 7) | MEDIUM | Update mapping.rs to match new token names; release new connector version. Risk: may need to support multiple gpui-component versions |
| iced widgets ignore custom theme (Pitfall 8) | HIGH | Must implement per-widget `Catalog` entries -- potentially dozens of trait impls. This is not a fix but a missing feature that was misidentified as complete |
| cargo-semver-checks blocks CI on first publish | LOW | Add `--baseline-rev` flag or conditional skip for initial release |
| crates.io publish with missing presets | MEDIUM | Cannot unpublish; must publish a new patch version with correct `package.include`. Version number is burned |

## Pitfall-to-Phase Mapping

How roadmap phases should address these pitfalls.

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| NSColor appearance context (Pitfall 1) | Step 2: macOS reader | Test that returns different accent colors for dark vs light system appearance |
| Autorelease pool (Pitfall 2) | Step 2: macOS reader | Run with Instruments leak detector; call `from_macos()` 1000 times and verify constant memory |
| Field name collision (Pitfall 3) | Step 1: API Refactors | Compile all 17 presets; grep for duplicate TOML keys per section; serde round-trip test |
| include_str paths (Pitfall 4) | Step 5: Workspace restructuring | `cargo publish --dry-run` and `cargo test --doc` pass from crate subdirectory |
| Widget metrics granularity (Pitfall 5) | Step 3: Widget Metrics | Document coverage matrix; assert coverage ratio matches expected per-platform |
| Windows DPI (Pitfall 6) | Step 2: Windows enhancements | Test on 200% scaling display; compare values with Windows Settings |
| gpui-component instability (Pitfall 7) | Step 5: Connectors | CI job building against gpui-component main; pinned version in Cargo.toml |
| iced Base trait (Pitfall 8) | Step 5: Connectors | Visual test: screenshot of iced app with native-theme vs default theme |
| cargo-semver-checks baseline (Pitfall 9) | Step 4: CI Pipeline | CI handles first-publish case without failure |
| crates.io metadata (Pitfall 10) | Step 6: Publishing | `cargo publish --dry-run` passes; `cargo package --list` reviewed |

## Sources

- [NSColor performAsCurrentDrawingAppearance](https://christiantietze.de/posts/2021/10/nscolor-performAsCurrentDrawingAppearance-resolve-current-appearance/) -- appearance resolution outside drawing context
- [Apple NSColor documentation](https://developer.apple.com/documentation/appkit/nscolor) -- dynamic color behavior, colorUsingColorSpace may return nil
- [objc2 Retained documentation](https://docs.rs/objc2/latest/objc2/rc/struct.Retained.html) -- autorelease pool and retain semantics
- [objc2 GitHub](https://github.com/madsmtm/objc2) -- framework-crates for AppKit bindings
- [serde flatten duplicate fields issue #1820](https://github.com/serde-rs/serde/issues/1820) -- field name collision behavior with flatten
- [serde flatten + TOML panic issue #1379](https://github.com/serde-rs/serde/issues/1379) -- flatten panics with toml::Value
- [serde_with flattened_maybe](https://docs.rs/serde_with/latest/serde_with/macro.flattened_maybe.html) -- alternative for dual-format deserialization
- [Cargo workspace publishing](https://doc.rust-lang.org/cargo/reference/publishing.html) -- path dependency stripping during publish
- [cargo publish include_str issue #13309](https://github.com/rust-lang/cargo/issues/13309) -- include_str with relative paths fails on publish
- [Mixed-Mode DPI Scaling - Microsoft](https://learn.microsoft.com/en-us/windows/win32/hidpi/high-dpi-improvements-for-desktop-applications) -- GetSystemMetricsForDpi requirement
- [Avalonia GetSystemMetrics DPI issue](https://github.com/AvaloniaUI/Avalonia/issues/12112) -- real-world DPI awareness bug in .NET framework
- [gpui-component theme docs](https://longbridge.github.io/gpui-component/docs/theme) -- ActiveTheme trait, color token system
- [gpui-component crates.io](https://crates.io/crates/gpui-component) -- now published at v0.5.1 (contradicts todo.md assumption it was git-only)
- [iced Base trait docs](https://docs.iced.rs/iced/widget/theme/trait.Base.html) -- five required methods for custom themes
- [iced 0.13 release notes](https://github.com/iced-rs/iced/releases/tag/0.13.0) -- closure-based and class-based theming introduced
- [iced 0.14 release notes](https://github.com/iced-rs/iced/releases/tag/0.14.0) -- Palette generation overhaul using Oklch
- [cargo-semver-checks README](https://github.com/obi1kenobi/cargo-semver-checks/blob/main/README.md) -- baseline flags, no-false-positives design goal
- [SemVer in Rust: Tooling, Breakage, and Edge Cases (FOSDEM 2024)](https://predr.ag/blog/semver-in-rust-tooling-breakage-and-edge-cases/) -- non_exhaustive interaction with semver tooling
- [crates.io publishing guide](https://doc.rust-lang.org/cargo/reference/publishing.html) -- metadata requirements, 10MB limit, permanence
- [docs.rs build environment](https://docs.rs/about/builds) -- docsrs cfg, nightly builds, feature detection
- [Cargo feature unification docs](https://doc.rust-lang.org/cargo/reference/features.html) -- resolver v2 behavior for workspace members

---
*Pitfalls research for: v0.2 feature additions to native-theme crate*
*Researched: 2026-03-08*
