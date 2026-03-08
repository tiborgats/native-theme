# Technology Stack: v0.2 Additions

**Project:** native-theme v0.2
**Researched:** 2026-03-08
**Scope:** New dependencies and tools for v0.2 features only. Does NOT repeat v0.1 validated stack (serde, toml, configparser, ashpd, windows crate, serde_with).

---

## 1. macOS Reader: objc2-app-kit

### Recommendation

| Technology | Version | Feature Flag | Purpose | Why |
|------------|---------|-------------|---------|-----|
| objc2-app-kit | 0.3.2 | `macos` | NSColor, NSFont, NSAppearance bindings | Only actively maintained, safe(r) ObjC bindings for Rust. Generated from Xcode 16.4 SDKs. Provides full access to ~40 NSColor semantic colors including `controlAccentColor` (which `cacao` 0.3.2 is missing). Part of the objc2 ecosystem (objc2 0.6.x). |

**Confidence: HIGH** -- Version 0.3.2 verified on crates.io (published October 4, 2025). Dependency chain confirmed: objc2-app-kit 0.3.2 requires objc2 >=0.6.2, <0.8.0; objc2-foundation 0.3.2 pulled transitively.

### Required Cargo.toml Features

```toml
[target.'cfg(target_os = "macos")'.dependencies]
objc2-app-kit = { version = "0.3", optional = true, default-features = false, features = [
    "NSColor",
    "NSFont",
    "NSAppearance",
    "NSColorSpace",
] }
```

**Feature flags explained:**
- `NSColor` -- semantic color access (`controlAccentColor`, `windowBackgroundColor`, `labelColor`, ~40 semantic colors)
- `NSFont` -- `systemFont(ofSize:)`, `monospacedSystemFont(ofSize:weight:)`, `systemFontSize`
- `NSAppearance` -- dark/light detection, `performAsCurrentDrawingAppearance` for resolving dynamic colors
- `NSColorSpace` -- `colorUsingColorSpace:` for P3-to-sRGB conversion

**Thread safety note:** NSColor is thread-safe for reading components. However, dynamic/semantic colors (e.g., `labelColor`) resolve differently in light vs dark mode based on `NSAppearance.current`. The reader must resolve the appearance context (via `performAsCurrentDrawingAppearance` on macOS 11+) before extracting RGB components. Once resolved to concrete sRGB u8 values, the data is Send+Sync.

### Alternatives Rejected

| Alternative | Why Not |
|-------------|---------|
| cacao 0.3.2 | Missing `controlAccentColor` -- the single most important color for theme reading. Wraps 39 NSColor variants but not the one that matters most. |
| cocoa crate (servo) | Deprecated. Uses legacy `objc` 0.2 runtime. No longer maintained. Superseded by objc2 ecosystem. |
| core-foundation + raw FFI | Verbose, unsafe, error-prone. objc2-app-kit provides the same access with safer APIs. |

---

## 2. Widget Metrics Data Model

### No New Dependencies Required

Widget metrics are a data model extension, not a new dependency. The `WidgetMetrics` struct and its 12 sub-structs (`ButtonMetrics`, `CheckboxMetrics`, `InputMetrics`, `ScrollbarMetrics`, `SliderMetrics`, `ProgressBarMetrics`, `TabMetrics`, `MenuItemMetrics`, `TooltipMetrics`, `ListItemMetrics`, `ToolbarMetrics`, `SplitterMetrics`) use the same patterns as existing model types: `Option<f32>` fields, `#[non_exhaustive]`, `#[serde(default)]`, `impl_merge!()`.

### Platform Source Verification

**KDE breezemetrics.h (~80 constants):**
Source: `https://github.com/KDE/breeze/blob/master/kstyle/breezemetrics.h`. Constants are C++ `constexpr` values like `Frame_FrameRadius = 2.5` (note: multiplied by `smallSpacing` which is 2 on Wayland). These are compile-time constants -- no runtime API needed. Strategy: hardcode per-version constant sets in Rust code.

**Confidence: HIGH** -- Source file verified on GitHub. Values are stable within a Plasma release.

**libadwaita CSS variables:**
Source: `https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1-latest/css-variables.html`. Geometry-relevant variables are limited to:
- `--window-radius` (15px)
- `--disabled-opacity` (50%, 40% in high contrast)
- `--dim-opacity` (55%, 90% in high contrast)
- `--border-opacity` (15%, 50% in high contrast)

libadwaita does NOT expose per-widget sizing/padding/spacing as CSS variables. Widget metrics (button height, checkbox size, input padding) are internal SCSS constants in the libadwaita source, not exposed as public CSS variables. Strategy: hardcode values extracted from libadwaita SCSS source, versioned per libadwaita release.

**Confidence: HIGH** -- Official libadwaita CSS variables documentation verified. The absence of widget-level sizing variables is confirmed (only opacity and window radius).

**Windows GetSystemMetrics (~20 dynamic values):**
Already available via existing `windows` crate dependency (`Win32_UI_WindowsAndMessaging` feature). `GetSystemMetrics` provides runtime values for `SM_CXBORDER`, `SM_CXEDGE`, `SM_CXFOCUSBORDER`, `SM_CXVSCROLL`, `SM_CXSMICON`, `SM_CYMENU`, `SM_CYSMCAPTION`, etc. No new dependencies needed.

**Confidence: HIGH** -- Already verified in v0.1 research.

**macOS:** No widget metrics API. Hardcode from Apple HIG documentation (~10pt radius, standard margins). Stable across macOS versions.

---

## 3. Toolkit Connectors

### 3a. gpui-component Connector (native-theme-gpui)

| Technology | Version | Relationship | Purpose |
|------------|---------|-------------|---------|
| gpui-component | 0.5.1 | `[dependencies]` of native-theme-gpui | Target toolkit for theme mapping |
| gpui | 0.2.2 | Transitive (via gpui-component) | GPUI runtime, `App` context |

**Confidence: HIGH** -- gpui-component 0.5.1 verified on lib.rs (published February 5, 2026). gpui 0.2.2 verified on crates.io.

**gpui-component Theme Architecture:**

The gpui-component theming system consists of:
- `Theme` struct: 17 fields covering colors (`ThemeColor`), fonts (`font_family`, `font_size`, `mono_font_family`, `mono_font_size`), geometry (`radius`, `radius_lg`, `shadow`), and display options (`scrollbar_show`, `tile_grid_size`)
- `ThemeColor` struct: **108 color fields** (HSLA format internally) organized into core UI, component-specific, status/semantic, sidebar, chart, and base color groups
- `ThemeConfig` / `ThemeSet`: JSON-serializable configuration structs. Theme files are JSON with `$schema` validation.
- `ActiveTheme` trait: implemented for `App`, provides `cx.theme()` access
- `ThemeRegistry`: watches a directory for JSON theme files, dynamically loads them
- `Theme::change(mode, theme_name, cx)`: applies a theme globally
- `Theme::global_mut(cx).apply_config(&config)`: applies a `ThemeConfig`

**native-theme to gpui-component mapping coverage:**

| native-theme field | gpui-component target | Notes |
|--------------------|----------------------|-------|
| `colors.accent` | `ThemeColor.accent` | Direct 1:1 |
| `colors.background` | `ThemeColor.background` | Direct 1:1 |
| `colors.foreground` | `ThemeColor.foreground` | Direct 1:1 |
| `colors.border` | `ThemeColor.border` | Direct 1:1 |
| `colors.primary` | `ThemeColor.primary` | Direct 1:1 |
| `colors.primary_foreground` | `ThemeColor.primary_foreground` | Direct 1:1 |
| `colors.secondary` | `ThemeColor.secondary` | Direct 1:1 |
| `colors.secondary_foreground` | `ThemeColor.secondary_foreground` | Direct 1:1 |
| `colors.danger` | `ThemeColor.danger` | Direct 1:1 |
| `colors.danger_foreground` | `ThemeColor.danger_foreground` | Direct 1:1 |
| `colors.warning` | `ThemeColor.warning` | Direct 1:1 |
| `colors.success` | `ThemeColor.success` | Direct 1:1 |
| `colors.info` | `ThemeColor.info` | Direct 1:1 |
| `colors.muted` | `ThemeColor.muted` | Direct 1:1 |
| `colors.link` | `ThemeColor.link` | Direct 1:1 |
| `colors.sidebar` | `ThemeColor.sidebar` | Direct 1:1 |
| `colors.sidebar_foreground` | `ThemeColor.sidebar_foreground` | Direct 1:1 |
| `colors.popover` | `ThemeColor.popover` | Direct 1:1 |
| `colors.popover_foreground` | `ThemeColor.popover_foreground` | Direct 1:1 |
| `colors.selection` | `ThemeColor.selection` | Direct 1:1 |
| `colors.tooltip` | (no direct match) | gpui-component uses popover for tooltips |
| `colors.separator` | (no direct match) | Could derive from border |
| `fonts.family` | `Theme.font_family` | Direct 1:1 |
| `fonts.size` | `Theme.font_size` | Direct 1:1 |
| `fonts.mono_family` | `Theme.mono_font_family` | Direct 1:1 |
| `fonts.mono_size` | `Theme.mono_font_size` | Direct 1:1 |
| `geometry.radius` | `Theme.radius` | Direct 1:1 |
| `geometry.radius_lg` | `Theme.radius_lg` | Direct 1:1 |
| `geometry.shadow` | `Theme.shadow` | Direct 1:1 |

**Key insight: The mapping is remarkably clean.** gpui-component's `Theme` and `ThemeColor` fields align closely with native-theme's data model -- both use shadcn/ui-inspired naming conventions. The connector's primary job is color space conversion (native-theme `Rgba` u8 sRGB to gpui's `Hsla`) and populating the ~60 gpui-specific fields that native-theme doesn't cover (chart colors, hover/active states, etc.) with sensible derivations.

**Color conversion:** native-theme uses `Rgba { r: u8, g: u8, b: u8, a: u8 }` in sRGB. gpui-component uses `Hsla { h: f32, s: f32, l: f32, a: f32 }`. The connector must implement RGB-to-HSL conversion. This is a ~20-line pure function with no dependencies.

**Cargo.toml for native-theme-gpui:**
```toml
[package]
name = "native-theme-gpui"
version = "0.2.0"

[dependencies]
native-theme = { path = "../native-theme" }
gpui-component = "0.5"
gpui = "0.2"
```

**Publishing note:** gpui-component 0.5.1 IS on crates.io. gpui 0.2.2 IS on crates.io. The connector CAN be published to crates.io.

### 3b. iced Connector (native-theme-iced)

| Technology | Version | Relationship | Purpose |
|------------|---------|-------------|---------|
| iced | 0.14.0 | `[dependencies]` of native-theme-iced | Target toolkit for theme mapping |

**Confidence: HIGH** -- iced 0.14.0 verified on crates.io (published December 7, 2025).

**iced Theme Architecture:**

iced's theming system is simpler than gpui-component's:
- `Theme` enum: 23 built-in variants (Light, Dark, Catppuccin variants, Nord, Gruvbox, etc.) + `Custom(Arc<Custom>)`
- `Palette` struct: **6 fields** -- `background`, `text`, `primary`, `success`, `warning`, `danger` (all `Color { r: f32, g: f32, b: f32, a: f32 }` in sRGB)
- `Extended` struct: generated from `Palette`, adds `secondary` + `is_dark` boolean. Each group (Background, Primary, Secondary, Success, Warning, Danger) has sub-variants (base, weak, strong).
- `Theme::custom(name, palette)` / `Theme::custom_with_fn(name, palette, generator)`: creates custom themes
- `theme.palette()` / `theme.extended_palette()`: reads back colors

**native-theme to iced mapping:**

| native-theme field | iced Palette target | Notes |
|--------------------|---------------------|-------|
| `colors.background` | `Palette.background` | Direct, convert u8 to f32 |
| `colors.foreground` | `Palette.text` | Direct, convert u8 to f32 |
| `colors.primary` (or `accent`) | `Palette.primary` | Direct |
| `colors.success` | `Palette.success` | Direct |
| `colors.warning` | `Palette.warning` | Direct |
| `colors.danger` | `Palette.danger` | Direct |
| `colors.secondary` | `Extended.secondary` | Via custom_with_fn generator |
| `fonts.*` | iced `Font` config | Applied at application level |
| `geometry.radius` | Per-widget `border_radius` | Applied via StyleSheet impls |
| `spacing.*` | Per-widget padding/spacing | Applied via StyleSheet impls |

**Key difference from gpui-component:** iced's Palette is much simpler (6 colors). The heavy lifting is in the `Extended` palette generator and per-widget `StyleSheet` trait implementations. The connector needs to:
1. Map 6 core colors to `Palette` (trivial)
2. Provide a custom `generate` function for `Extended` that uses native-theme's richer color set
3. Implement `StyleSheet` traits for widgets that consume geometry/spacing data

**Color conversion:** native-theme `Rgba { r: u8 }` to iced `Color { r: f32 }` is `value as f32 / 255.0`. Trivial.

**Cargo.toml for native-theme-iced:**
```toml
[package]
name = "native-theme-iced"
version = "0.2.0"

[dependencies]
native-theme = { path = "../native-theme" }
iced = "0.14"
```

**Publishing note:** iced IS on crates.io. The connector CAN be published immediately.

---

## 4. CI Pipeline

### GitHub Actions

No Cargo dependencies. CI tools installed via GitHub Actions marketplace.

| Tool | Installation | Purpose | Why |
|------|-------------|---------|-----|
| `dtolnay/rust-toolchain@stable` | GH Action | Install Rust toolchain | De facto standard for Rust CI. Supports target installation, component selection. |
| `obi1kenobi/cargo-semver-checks-action@v2` | GH Action | SemVer violation detection | Official GH Action for cargo-semver-checks. Installs the tool, caches baseline rustdoc, runs checks. Zero configuration for single-crate repos. |
| `cargo-semver-checks` | (installed by action above) | CLI tool | Currently at 0.46.0. Requires Rust 1.85+ (within our MSRV). 177+ lints detecting accidental breaking changes. |
| `cargo hack` | `cargo install cargo-hack` | Feature flag testing | Tests all feature flag combinations compile: `cargo hack check --feature-powerset`. Essential for a crate with 5+ feature flags. |

**Confidence: HIGH** -- cargo-semver-checks-action@v2 verified on GitHub Marketplace. cargo-semver-checks 0.46.0 verified on crates.io docs.rs.

### Workflow Matrix

```yaml
strategy:
  matrix:
    include:
      - os: ubuntu-latest
        features: "--no-default-features"
      - os: ubuntu-latest
        features: "--features kde"
      - os: ubuntu-latest
        features: "--features portal-tokio"
      - os: windows-latest
        features: "--features windows"
      - os: macos-latest
        features: "--features macos"
```

**Key consideration:** Feature flags are platform-specific. The `kde` and `portal-*` features only compile on Linux. The `windows` feature only compiles on Windows. The `macos` feature only compiles on macOS. The CI matrix must match features to their target OS.

### cargo-semver-checks Integration

```yaml
- name: Check semver
  uses: obi1kenobi/cargo-semver-checks-action@v2
```

The action compares the current code against the last published version on crates.io. For a workspace, it checks each publishable crate. `#[non_exhaustive]` on structs means new field additions are already non-breaking, so semver-checks will not flag those.

**What it catches:** removed public items, changed function signatures, added required fields to non-`#[non_exhaustive]` structs, trait method additions, type changes.

**What it does NOT catch:** behavioral changes, performance regressions, subtle API misuse patterns.

---

## 5. Cargo Workspace Restructuring

### No New Dependencies

Workspace restructuring is a Cargo.toml configuration change, not a dependency addition.

### Target Structure

```
native-theme/
  Cargo.toml              # workspace root: [workspace] members = [...]
  native-theme/
    Cargo.toml             # core crate (published to crates.io)
    src/
    presets/
    tests/
  native-theme-gpui/
    Cargo.toml             # connector (path dep on native-theme, crates.io dep on gpui-component)
    src/lib.rs
    examples/showcase.rs
  native-theme-iced/
    Cargo.toml             # connector (path dep on native-theme, crates.io dep on iced)
    src/lib.rs
    examples/demo.rs
```

### Workspace Root Cargo.toml

```toml
[workspace]
members = [
    "native-theme",
    "native-theme-gpui",
    "native-theme-iced",
]
resolver = "3"

[workspace.package]
edition = "2024"
license = "MIT OR Apache-2.0 OR 0BSD"
rust-version = "1.85.0"
repository = "https://github.com/tiborgats/native-theme"
```

**Key decisions:**
- `resolver = "3"` (edition 2024 default, but explicit in workspace root)
- Shared package metadata via `[workspace.package]` -- each sub-crate inherits with `edition.workspace = true`
- Core crate uses `path = "../native-theme"` for workspace-internal deps
- Connectors use path deps for native-theme, crates.io version deps for toolkit crates

### Publishing Order

1. `native-theme` (core, no workspace-internal deps)
2. `native-theme-iced` (depends on published native-theme + iced from crates.io)
3. `native-theme-gpui` (depends on published native-theme + gpui-component from crates.io)

Each crate is independently versioned. Connector crate versions can differ from core crate version.

---

## 6. Publishing to crates.io

### Required Cargo.toml Fields (core crate)

```toml
[package]
name = "native-theme"
version = "0.2.0"
edition = "2024"
rust-version = "1.85.0"
license = "MIT OR Apache-2.0 OR 0BSD"
description = "Cross-platform native theme detection and loading for Rust GUI applications"
repository = "https://github.com/tiborgats/native-theme"
homepage = "https://github.com/tiborgats/native-theme"
documentation = "https://docs.rs/native-theme"
keywords = ["theme", "native", "gui", "colors", "desktop"]
categories = ["gui", "config"]
readme = "README.md"
```

**Required by crates.io:** `name`, `version`, `description`, `license` (or `license-file`).
**Strongly recommended:** `repository`, `keywords`, `categories`, `readme`, `rust-version`.
**Must exist in repo:** LICENSE-MIT, LICENSE-APACHE, LICENSE-0BSD files.

### Pre-Publish Checklist (dev tooling, not runtime deps)

| Tool | Command | Purpose |
|------|---------|---------|
| cargo publish --dry-run | `cargo publish -p native-theme --dry-run` | Surfaces packaging warnings before upload |
| cargo doc | `cargo doc --no-deps --all-features` | Verify docs build cleanly |
| cargo deny | `cargo deny check` | License compatibility, advisory database |
| cargo hack | `cargo hack check --feature-powerset` | All feature combinations compile |

### What NOT to Publish

The connector crates (`native-theme-gpui`, `native-theme-iced`) can be published, but they are secondary. The core `native-theme` crate should be published first and is the primary deliverable.

---

## 7. Updated Feature Flags (v0.2)

```toml
[features]
default = []
kde = ["dep:configparser"]
portal = ["dep:ashpd"]
portal-tokio = ["portal", "ashpd/tokio"]
portal-async-io = ["portal", "ashpd/async-io"]
windows = ["dep:windows"]
macos = ["dep:objc2-app-kit"]    # NEW in v0.2
```

**New in v0.2:** The `macos` feature flag gating objc2-app-kit. All other feature flags remain unchanged from v0.1.

**Deferred (not in v0.2):** `ios` (objc2-ui-kit), `android` (jni + ndk), `watch` (notify).

---

## 8. Version Compatibility Matrix (v0.2)

| Package | Version | Compatible With | Notes |
|---------|---------|-----------------|-------|
| serde | 1.0.228+ | toml 1.0.6, gpui-component 0.5.1, iced 0.14.0 | Universal. No conflicts possible. |
| toml | 1.0.6 | serde ^1.0.145 | Stable. |
| objc2-app-kit | 0.3.2 | objc2 >=0.6.2, <0.8.0 | objc2 0.6.4 is current latest -- within range. |
| windows | 0.62.2 | Rust 1.61+ | Latest stable. No 0.63 release exists yet. |
| gpui-component | 0.5.1 | gpui 0.2.2, serde 1.x | On crates.io. |
| gpui | 0.2.2 | (many deps) | On crates.io. Heavy dep tree. |
| iced | 0.14.0 | Rust 1.80+ | On crates.io. Stable release. |
| ashpd | 0.13.4 | zbus ^5.13, tokio ^1.43 | Unchanged from v0.1. |
| configparser | 3.1.0 | (standalone) | Unchanged from v0.1. |
| cargo-semver-checks | 0.46.0 | Rust 1.85+ | Dev tool only. Within our MSRV. |

---

## 9. What NOT to Add in v0.2

| Avoid | Why | Notes |
|-------|-----|-------|
| objc2-ui-kit (iOS) | Deferred to post-v0.2. iOS reader is lower priority than macOS. | Same objc2 ecosystem -- trivial to add later. |
| jni + ndk (Android) | Deferred. Android theme reading from Rust is immature. | Most verbose platform, least ergonomic. |
| notify (file watching) | Deferred. Change notification is post-v0.2. | Users can poll `from_system()`. |
| thiserror | Manual Error enum is sufficient with 4-5 variants. | Reconsider if variant count exceeds 10. |
| anyhow | Library crate must expose typed errors. | Never use in library crates. |
| serde_json (runtime) | gpui-component themes use JSON, but the connector reads gpui's internal types, not JSON files. | Only needed as dev-dependency for tests. |
| cssparser / lightningcss | Do NOT try to parse libadwaita CSS at runtime. The values are hardcoded editorial choices from source inspection. | Runtime CSS parsing would be fragile and version-dependent. |
| dbus / zbus (direct) | Already wrapped by ashpd. Using raw zbus loses type safety. | ashpd is the right abstraction. |
| dirs crate | Already in v0.1 for KDE path resolution. No changes needed. | Check if macOS reader needs it (it does not -- NSColor APIs don't use file paths). |

---

## 10. Dependency Delta Summary (v0.1 to v0.2)

### New Runtime Dependencies

| Dependency | Feature Gate | Added For |
|------------|-------------|-----------|
| objc2-app-kit 0.3 | `macos` | macOS reader (NSColor, NSFont, NSAppearance) |

### New Workspace Member Dependencies (connector crates only)

| Dependency | Crate | Added For |
|------------|-------|-----------|
| gpui-component 0.5 | native-theme-gpui | gpui-component theme mapping |
| gpui 0.2 | native-theme-gpui | GPUI App context |
| iced 0.14 | native-theme-iced | iced Theme/Palette mapping |

### New Dev/CI Dependencies

| Dependency | Type | Added For |
|------------|------|-----------|
| cargo-semver-checks 0.46 | CI tool | SemVer violation detection |
| cargo-hack | CI tool | Feature powerset testing |

### Unchanged from v0.1

| Dependency | Version | Notes |
|------------|---------|-------|
| serde | 1.0.228 | No change |
| serde_with | 3.17.0 | No change |
| toml | 1.0.6 | No change |
| configparser | 3.1.0 | No change (kde feature) |
| ashpd | 0.13.4 | No change (portal feature) |
| windows | >=0.59, <=0.62 | Consider tightening to 0.62 (no reason to support <0.62) |
| serde_json | 1.0.149 | No change (dev-dependency) |

### Recommended Changes to Existing Dependencies

| Change | Rationale |
|--------|-----------|
| Tighten `windows` version to `"0.62"` | No reason to support 0.59-0.61. Simplifies testing. 0.62.2 is latest stable. |
| Add `dirs` to `kde` feature deps | Already in v0.1 STACK.md but not in actual Cargo.toml. Either add it or confirm manual path resolution is preferred. |

---

## Sources

- [crates.io/objc2-app-kit](https://crates.io/crates/objc2-app-kit) -- version 0.3.2, published Oct 4 2025 (HIGH confidence)
- [lib.rs/objc2-app-kit](https://lib.rs/crates/objc2-app-kit) -- dependency chain verified: objc2 >=0.6.2, <0.8.0 (HIGH confidence)
- [docs.rs/gpui-component/theme](https://docs.rs/gpui-component/latest/gpui_component/theme/index.html) -- Theme module API verified (HIGH confidence)
- [docs.rs/gpui-component/ThemeColor](https://docs.rs/gpui-component/latest/gpui_component/theme/struct.ThemeColor.html) -- 108 color fields enumerated (HIGH confidence)
- [docs.rs/gpui-component/Theme](https://docs.rs/gpui-component/latest/gpui_component/theme/struct.Theme.html) -- 17 fields verified (HIGH confidence)
- [docs.rs/gpui-component/ThemeConfig](https://docs.rs/gpui-component/latest/gpui_component/theme/struct.ThemeConfig.html) -- 12 fields verified (HIGH confidence)
- [lib.rs/gpui-component](https://lib.rs/crates/gpui-component) -- version 0.5.1, published Feb 5 2026 (HIGH confidence)
- [crates.io/gpui-component](https://crates.io/crates/gpui-component) -- on crates.io, publishable connectors confirmed (HIGH confidence)
- [docs.rs/iced Theme enum](https://docs.rs/iced/latest/iced/enum.Theme.html) -- version 0.14.0, 23 variants + Custom (HIGH confidence)
- [docs.rs/iced Palette](https://docs.rs/iced/latest/iced/theme/palette/struct.Palette.html) -- 6 fields (background, text, primary, success, warning, danger) (HIGH confidence)
- [docs.rs/iced Extended](https://docs.rs/iced/latest/iced/theme/palette/struct.Extended.html) -- 7 fields (6 color groups + is_dark) (HIGH confidence)
- [docs.rs/iced Color](https://docs.rs/iced/latest/iced/struct.Color.html) -- f32 RGBA (HIGH confidence)
- [lib.rs/iced](https://lib.rs/crates/iced) -- version 0.14.0, published Dec 7 2025 (HIGH confidence)
- [github.com/obi1kenobi/cargo-semver-checks-action](https://github.com/obi1kenobi/cargo-semver-checks-action) -- v2 action verified (HIGH confidence)
- [crates.io/cargo-semver-checks](https://crates.io/crates/cargo-semver-checks) -- version 0.46.0 (HIGH confidence)
- [gnome.pages.gitlab.gnome.org libadwaita CSS variables](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1-latest/css-variables.html) -- geometry variables: only --window-radius, opacity vars (HIGH confidence)
- [github.com/KDE/breeze breezemetrics.h](https://github.com/KDE/breeze/blob/master/kstyle/breezemetrics.h) -- ~80 widget metric constants (HIGH confidence)
- [lib.rs/windows](https://lib.rs/crates/windows) -- version 0.62.2, published Oct 6 2025 (HIGH confidence)
- [github.com/microsoft/windows-rs releases](https://github.com/microsoft/windows-rs/releases) -- no 0.63 release exists (HIGH confidence)
- [github.com/longbridge/gpui-component themes](https://github.com/longbridge/gpui-component/tree/main/themes) -- 21 JSON theme files, hex color format (HIGH confidence)

---
*Stack research for: native-theme v0.2 (new features)*
*Researched: 2026-03-08*
