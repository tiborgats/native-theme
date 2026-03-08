# Feature Landscape: v0.2 New Features

**Domain:** Toolkit-agnostic cross-platform OS theme data crate (Rust) -- subsequent milestone
**Researched:** 2026-03-08
**Overall confidence:** HIGH

This document covers ONLY the new features planned for v0.2. Existing v0.1 features (36-field color model, TOML serde, presets, KDE/GNOME/Windows readers, merge, Rgba) are already shipped.

---

## Table Stakes

Features users expect when these capabilities are advertised. Missing any = the feature feels unfinished.

### 1. macOS Reader: NSColor Semantic Color Reading

| Aspect | Detail |
|--------|--------|
| Why expected | macOS is a primary desktop target. v0.1 already supports KDE, GNOME portal, and Windows. Shipping without macOS means cross-platform coverage has a visible gap. Every competing approach (dark-light, system-theme, Electron nativeTheme) supports macOS. |
| Complexity | HIGH |
| Dependencies | New `macos` feature flag, `objc2-app-kit` dependency |

**What users expect from a macOS reader:**

The macOS reader maps NSColor semantic properties to NativeTheme fields. Based on Apple's AppKit documentation and objc2-app-kit bindings (HIGH confidence), the available semantic colors are:

**Core colors (map directly to ThemeColors fields):**
- `controlAccentColor` -> `accent`
- `windowBackgroundColor` -> `background`
- `labelColor` -> `foreground`
- `controlBackgroundColor` -> `surface`
- `separatorColor` -> `border`
- `secondaryLabelColor` -> `muted`
- `shadowColor` -> `shadow`

**Status colors:**
- `systemRedColor` -> `danger`
- `systemOrangeColor` -> `warning`
- `systemGreenColor` -> `success`
- `systemBlueColor` -> `info`

**Interactive colors:**
- `selectedContentBackgroundColor` -> `selection`
- `selectedTextColor` -> `selection_foreground`
- `linkColor` -> `link`
- `keyboardFocusIndicatorColor` -> `focus_ring`

**Panel colors:**
- `underPageBackgroundColor` -> `sidebar`
- (no sidebar foreground exposed by AppKit)

**Component colors:**
- `controlColor` -> `button`
- `controlTextColor` -> `button_foreground`
- `textColor` -> `input_foreground`
- `textBackgroundColor` -> `input`
- `disabledControlTextColor` -> `disabled`
- `gridColor` -> `separator`
- `alternatingContentBackgroundColors` (first alternate) -> `alternate_row`

**Total: ~20-22 color mappings** -- the richest semantic color source after KDE.

**Technical requirements (from Apple docs + objc2-app-kit docs, HIGH confidence):**

1. **P3 -> sRGB conversion**: macOS uses Display P3 color space by default since macOS 10.15. Semantic NSColor properties may return P3 colors. Must call `colorUsingColorSpace:` with `NSColorSpace.sRGBColorSpace` before extracting RGBA components. Without this conversion, RGB values will be wrong (clipped or mis-mapped). This is a crash risk -- calling `redComponent` on a catalog color without converting first throws an NSException.

2. **NSAppearance resolution**: Dynamic/semantic colors resolve differently in light vs dark mode. To read both variants, set `NSAppearance.current` to the desired appearance before querying colors:
   ```
   let old = NSAppearance.current
   NSAppearance.current = <light appearance>
   // read light colors
   NSAppearance.current = <dark appearance>
   // read dark colors
   NSAppearance.current = old
   ```
   On macOS 11+, `performAsCurrentDrawingAppearance` wraps this pattern.

3. **NSFont reading**: `NSFont.systemFont(ofSize: 0)` returns the default system font (San Francisco). `NSFont.monospacedSystemFont(ofSize: 0)` returns the monospace variant. The `pointSize` property gives the size. The `familyName` property gives the family (will be ".AppleSystemUIFont" or "SF Pro" depending on context -- use `.displayName` for the user-visible name).

4. **Hardcoded geometry**: AppKit does not expose border radius, spacing, or widget metrics via API. Use hardcoded HIG defaults: ~5px corner radius, ~8pt default spacing. Apple enforces visual consistency through HIG, so these values are stable across macOS versions.

5. **Feature flag**: `macos = ["dep:objc2-app-kit", "dep:objc2-foundation"]` with `target_os = "macos"` cfg gate. Must not compile on non-macOS targets.

### 2. API Methods on NativeTheme (from free functions)

| Aspect | Detail |
|--------|--------|
| Why expected | Idiomatic Rust convention (File::open, String::from, HashMap::new). Discoverability via autocomplete on `NativeTheme::`. Currently `preset("name")` gives no hint it returns a `NativeTheme`. |
| Complexity | LOW |
| Dependencies | None -- pure refactor of existing code |

Moving these free functions to associated methods:
- `preset("name")` -> `NativeTheme::preset("name")`
- `list_presets()` -> `NativeTheme::list_presets()`
- `from_toml(s)` -> `NativeTheme::from_toml(s)`
- `from_file(path)` -> `NativeTheme::from_file(path)`
- `to_toml(&theme)` -> `theme.to_toml()`

Pre-1.0, so no deprecation period needed. Remove old free functions outright.

### 3. Flat ThemeColors (36 direct fields)

| Aspect | Detail |
|--------|--------|
| Why expected | The nested sub-struct pattern (`colors.core.accent`, `colors.status.danger`) was an implementation detail that leaked into the TOML format. Users writing TOML presets must learn a taxonomy that has no precedent in CSS, shadcn/ui, Tailwind, or Material Design. Flat `colors.accent` matches every ecosystem convention. |
| Complexity | MEDIUM |
| Dependencies | Requires updating all 17 presets, all 3 platform readers, all tests |

This is a breaking API change. The data model flattens from 7 nested sub-structs to 36 direct `Option<Rgba>` fields on `ThemeColors`. The TOML format changes from `[light.colors.core]` with 7 sections to `[light.colors]` with 36 keys.

### 4. ThemeGeometry: radius_lg and shadow fields

| Aspect | Detail |
|--------|--------|
| Why expected | The implementation spec defines these. `radius_lg` is needed for window/dialog corners (GNOME `--window-radius` = 15px, Material 3 large = 16dp). `shadow` (bool) indicates whether the platform uses drop shadows (Breeze: yes, Adwaita: yes, some HiDPI setups: no). |
| Complexity | LOW |
| Dependencies | `#[non_exhaustive]` makes this non-breaking |

Two new fields on the existing struct:
- `radius_lg: Option<f32>` -- large element corner radius
- `shadow: Option<bool>` -- drop shadows enabled

---

## Differentiators

Features that set v0.2 apart. Not strictly expected but create significant value.

### 5. Widget Metrics System

| Aspect | Detail |
|--------|--------|
| Value proposition | Enables pixel-perfect native UIs. No other toolkit-agnostic crate provides per-widget sizing data. KDE breezemetrics.h has ~80 constants. Windows GetSystemMetrics has ~20 widget-related values. This is data that toolkit connector authors need to make buttons, checkboxes, and scrollbars match the host platform. |
| Complexity | HIGH |
| Dependencies | New `WidgetMetrics` struct, new field on `ThemeVariant`, platform reader updates |

**What widgets need metrics and what measurements matter:**

Based on analysis of KDE breezemetrics.h, Windows GetSystemMetrics, macOS HIG, and libadwaita CSS (HIGH confidence for KDE/Windows, MEDIUM for macOS/GNOME):

| Widget | Measurements | KDE Source | Windows Source | macOS | GNOME |
|--------|-------------|------------|---------------|-------|-------|
| **Button** | min_height, padding_h, padding_v, icon_size, spacing | breezemetrics.h: Button_MinWidth, Button_MarginWidth, Button_ItemSpacing | SM_CXSIZE (caption button), SPI_GETNONCLIENTMETRICS | HIG: 20pt min height (hardcoded) | libadwaita SCSS (hardcoded) |
| **Checkbox** | indicator_size, spacing | CheckBox_Size (~20px), CheckBox_ItemSpacing | SM_CXMENUCHECK, SM_CYMENUCHECK | HIG: 14pt (hardcoded) | ~20px (hardcoded) |
| **Input** | min_height, padding_h, padding_v | LineEdit_MarginWidth, LineEdit_FrameWidth | SM_CXEDGE | HIG: 22pt (hardcoded) | ~34px (hardcoded) |
| **Scrollbar** | width, min_slider_height | ScrollBar_Width (~14px), ScrollBar_MinSliderHeight | SM_CXVSCROLL, SM_CYHSCROLL | HIG: overlay ~7px / legacy ~15px | --scrollbar-width (hardcoded) |
| **Slider** | groove_height, handle_size | Slider_TickLength, Slider_GrooveThickness | SM_CXHTHUMB | HIG: ~22pt (hardcoded) | ~34px (hardcoded) |
| **ProgressBar** | height | ProgressBar_BusyIndicatorSize, ProgressBar_Thickness | n/a | HIG: ~4pt (hardcoded) | ~10px (hardcoded) |
| **Tab** | min_height, padding_h, overlap | TabBar_TabMarginWidth, TabBar_TabMinHeight, TabBar_TabOverlap | n/a | HIG: ~28pt (hardcoded) | ~46px (hardcoded) |
| **MenuItem** | min_height, padding_h, icon_size | MenuItem_MarginWidth, MenuItem_MarginHeight, MenuItem_ItemSpacing | SM_CYMENUSIZE | n/a | ~32px (hardcoded) |
| **Tooltip** | padding | ToolTip_FrameWidth | n/a | n/a | ~6px (hardcoded) |
| **ListItem** | min_height, padding_h | n/a | n/a | HIG: ~24pt (hardcoded) | ~34px (hardcoded) |
| **Toolbar** | height, separator_width | ToolBar_FrameWidth, ToolBar_HandleWidth, ToolBar_SeparatorItemWidth | SM_CXMENUSIZE | HIG: ~38pt (hardcoded) | ~46px (hardcoded) |
| **Splitter** | handle_width | Splitter_SplitterWidth (~6px) | SM_CXSIZEFRAME | n/a | n/a |

**Key design decisions:**
- Each widget gets its own sub-struct (e.g., `ButtonMetrics`, `CheckboxMetrics`) with all `Option<f32>` fields, `#[non_exhaustive]`, serde defaults
- `WidgetMetrics` composes all sub-structs with `skip_serializing_if` for empty ones
- Add `widget_metrics: Option<WidgetMetrics>` to `ThemeVariant`
- Platform sourcing: KDE = hardcoded constants from breezemetrics.h (versioned per Plasma release), GNOME = hardcoded from libadwaita SCSS (versioned per libadwaita release), Windows = runtime via GetSystemMetrics (no versioning needed), macOS = hardcoded HIG defaults (stable across versions)

**Recommendation: Start with the 12 widget types listed above.** These cover the widgets that gpui-component and iced both provide. Each sub-struct should have 2-5 fields maximum -- just the measurements that vary across platforms. Do not model every possible dimension; model only what connectors need.

### 6. gpui-component Connector (native-theme-gpui)

| Aspect | Detail |
|--------|--------|
| Value proposition | gpui-component has 108 ThemeColor fields (HIGH confidence -- verified from docs.rs). A connector maps native-theme's 36 semantic colors + fonts + geometry + widget metrics to gpui-component's styling system, so gpui users get native-harmonious look with one function call. |
| Complexity | HIGH |
| Dependencies | Workspace restructuring, gpui-component git dependency, upstream PRs for missing hooks |

**gpui-component ThemeColor mapping analysis:**

gpui-component's ThemeColor has 108 fields (verified from docs.rs/gpui-component, HIGH confidence). The mapping from native-theme's 36 colors covers the core subset:

| native-theme field | gpui-component field(s) |
|-------------------|------------------------|
| accent | accent |
| background | background |
| foreground | foreground |
| surface | (derive from background) |
| border | border |
| muted | muted |
| shadow | (used for shadow opacity) |
| primary_bg | primary |
| primary_fg | primary_foreground |
| secondary_bg | secondary |
| secondary_fg | secondary_foreground |
| danger | danger |
| danger_foreground | danger_foreground |
| warning | warning |
| warning_foreground | warning_foreground |
| success | success |
| success_foreground | success_foreground |
| info | info |
| info_foreground | info_foreground |
| selection | list_active, table_active |
| link | link |
| focus_ring | ring |
| sidebar | sidebar |
| sidebar_foreground | sidebar_foreground |
| tooltip | popover (gpui uses popover for tooltips) |
| tooltip_foreground | popover_foreground |
| button | (derive from secondary) |
| button_foreground | (derive from secondary_foreground) |
| input | input |
| disabled | (derive with disabled_opacity) |
| separator | (derive from border) |

**Fields gpui-component has that native-theme does NOT provide** (must be derived or use defaults):
- Hover/active variants: `primary_hover`, `primary_active`, `danger_hover`, etc. (18 fields) -- derive by lightening/darkening base color
- Chart colors: `chart_1` through `chart_5`, `bullish`, `bearish` -- use defaults
- List/table variants: `list_even`, `list_hover`, `table_even`, etc. -- derive from base colors
- Tab variants: `tab`, `tab_active`, `tab_foreground`, etc. -- derive from accent/background
- Scrollbar: `scrollbar`, `scrollbar_thumb`, `scrollbar_thumb_hover` -- derive from muted
- Switch/slider: `switch`, `switch_thumb`, `slider_bar`, `slider_thumb` -- derive from accent/muted
- Title bar: `title_bar`, `title_bar_border` -- derive from background

**Per-widget styling gaps in gpui-component:**

gpui-component currently hardcodes many widget dimensions (padding, icon sizes, corner radii). The connector will likely need upstream PRs to expose these as theme tokens. The todo.md correctly identifies the PR strategy: frame as "more theming flexibility," one concern per PR, no breaking changes.

**Cannot publish to crates.io** until gpui-component is published there. Usable via git dependency in the meantime.

### 7. iced Connector (native-theme-iced)

| Aspect | Detail |
|--------|--------|
| Value proposition | iced (25k GitHub stars) has the strongest per-widget styling of the pure-Rust toolkits. The Catalog/closure-based system (since iced 0.13) allows complete visual customization. COSMIC desktop proves this approach at scale. |
| Complexity | MEDIUM |
| Dependencies | Workspace restructuring, iced crates.io dependency |

**iced Palette and styling analysis:**

iced's theming works at two levels (HIGH confidence -- verified from docs.rs and discourse.iced.rs):

**Level 1: Palette (6 base colors)**
The `Palette` struct has exactly 6 fields:
- `background: Color`
- `text: Color`
- `primary: Color`
- `success: Color`
- `warning: Color`
- `danger: Color`

From these 6, iced auto-generates an `Extended` palette with derived shades for primary, secondary, success, warning, danger, and background -- each with base/weak/strong variants.

**Mapping from native-theme to iced Palette:**
- `background` -> `Palette::background`
- `foreground` -> `Palette::text`
- `accent` (or `primary_bg`) -> `Palette::primary`
- `success` -> `Palette::success`
- `warning` -> `Palette::warning`
- `danger` -> `Palette::danger`

This covers the Palette level completely.

**Level 2: Per-widget Style structs (closure-based)**

Each styleable widget has a `Style` struct. The connector can provide custom Catalog implementations for deeper customization. Widgets with Style support:

| Widget | Style fields | native-theme source |
|--------|-------------|-------------------|
| Button | background, text_color, border, shadow, snap | primary_bg/fg, border, shadow |
| Container | text_color, background, border, shadow, snap | background, foreground, border |
| TextInput | background, border, icon, placeholder, value, selection | input, input_fg, muted, selection |
| Checkbox | background, icon_color, border, text_color | accent, background, border |
| Radio | background, dot_color, border, text_color | accent, background, border |
| Toggler | background, foreground, background_border | accent, background |
| Slider | rail, handle (colors, border, width) | accent, muted, border |
| ProgressBar | background, bar, border_radius | accent, muted |
| Scrollable | container, scrollbar (background, border, scroller) | muted, scrollbar metrics |
| PickList | background, text_color, placeholder_color, handle_color, border | input, foreground, muted, border |
| Rule | color, width, radius, fill_mode | separator |
| Tooltip | background, text_color, border | tooltip, tooltip_fg, border |

**Key advantage:** iced is on crates.io, so `native-theme-iced` can be published immediately. No git dependency needed.

**iced also supports:** Border (width, color, radius), Shadow (color, offset, blur_radius), Background (Color or Gradient). The connector can use ThemeGeometry.radius for border radii and ThemeGeometry.shadow for shadow enable/disable.

### 8. Cargo Workspace Restructuring

| Aspect | Detail |
|--------|--------|
| Value proposition | Enables separate connector crates while keeping the core crate publishable. Standard Rust ecosystem pattern for core+extension crate families. |
| Complexity | MEDIUM |
| Dependencies | Must happen before connector crates |

**Workspace conventions (HIGH confidence -- from Cargo Book):**

Structure:
```
native-theme/
  Cargo.toml              # workspace root (not a package)
  native-theme/
    Cargo.toml             # core crate (publishable)
    src/
  native-theme-gpui/
    Cargo.toml             # connector (git dep on gpui-component)
    src/
    examples/
  native-theme-iced/
    Cargo.toml             # connector (crates.io dep on iced)
    src/
    examples/
```

**Key conventions:**
- Workspace root Cargo.toml has `[workspace]` section with `members` list
- Core crate uses `native-theme` as its package name (the crates.io name)
- Connectors use path dependency for development: `native-theme = { path = "../native-theme" }`
- For publishing: connectors also need `version` in their native-theme dependency
- Shared settings (edition, rust-version, license) go in `[workspace.package]` and are inherited via `package.edition.workspace = true`
- Shared dependencies go in `[workspace.dependencies]` for version consistency
- Publishing order: core crate first, then connectors (connectors depend on core)
- Use `cargo publish -p native-theme` to publish specific crate

**Publishing constraint for native-theme-gpui:** gpui-component is not on crates.io (only available as git dependency). Therefore native-theme-gpui CANNOT be published to crates.io until gpui-component is published. It can still exist in the workspace and be used via git dependency.

### 9. CI Pipeline

| Aspect | Detail |
|--------|--------|
| Value proposition | Cross-platform testing catches platform-specific compilation errors before release. Feature flag matrix ensures each reader compiles independently. semver-checks prevents accidental API breakage. |
| Complexity | LOW |
| Dependencies | GitHub Actions |

Standard GitHub Actions workflow:
- Test on Linux + Windows + macOS runners
- Feature flag matrix: `--no-default-features`, individual features (`kde`, `portal-tokio`, `windows`, `macos`)
- `cargo clippy` + `cargo fmt --check`
- `cargo semver-checks` to catch accidental breaking changes
- `#[non_exhaustive]` means new field additions are non-breaking (semver-checks won't flag them)

### 10. Publishing Prep

| Aspect | Detail |
|--------|--------|
| Value proposition | Professional crate presence on crates.io and docs.rs. Required metadata for discoverability. Doc examples that compile via `cargo test --doc`. |
| Complexity | LOW |
| Dependencies | All other v0.2 work should be complete first |

**Required crates.io metadata (HIGH confidence -- from Cargo Book):**

| Field | Value | Status |
|-------|-------|--------|
| `name` | `native-theme` | Already set |
| `version` | `0.2.0` | Already set |
| `edition` | `2024` | Already set |
| `license` | `MIT OR Apache-2.0 OR 0BSD` | Already set |
| `description` | (current is good) | Already set |
| `rust-version` | `1.85` | Needs adding |
| `repository` | `https://github.com/tiborgats/native-theme` | Needs adding |
| `homepage` | (same as repository or docs.rs link) | Needs adding |
| `documentation` | (auto-detected by docs.rs if omitted) | Optional |
| `readme` | `README.md` | Needs adding |
| `keywords` | `["theme", "native", "gui", "colors", "desktop"]` | Needs adding (max 5) |
| `categories` | `["gui", "config"]` | Needs adding (must be valid slugs) |

**Documentation expectations:**
- `/// # Examples` doc comments on `NativeTheme`, `Rgba`, `ThemeVariant` -- these compile via `cargo test --doc` and appear on docs.rs
- README.md with usage examples, feature flag table, platform support matrix
- CHANGELOG.md following Keep a Changelog format
- LICENSE files (MIT, Apache-2.0, 0BSD) at repo root

### 11. Windows Reader Enhancements

| Aspect | Detail |
|--------|--------|
| Value proposition | The v0.1 Windows reader reads accent + background + foreground. v0.2 adds accent shades (AccentLight1-3, AccentDark1-3), system font via SystemParametersInfo, and primary_foreground derivation. |
| Complexity | MEDIUM |
| Dependencies | Existing `windows` feature flag |

**What's being added:**
- `ApiInformation::IsMethodPresent` capability check before UISettings calls (graceful degradation on older Windows)
- 6 accent shade colors: AccentDark1-3 and AccentLight1-3 (useful for hover/active states in connectors)
- System font: `SystemParametersInfo(SPI_GETNONCLIENTMETRICS)` -> `NONCLIENTMETRICS.lfMessageFont` -> font name and size
- `primary_foreground` derivation: white or black based on accent luminance contrast
- WinUI3 default spacing values

### 12. Linux Reader Enhancements

| Aspect | Detail |
|--------|--------|
| Value proposition | Fills gaps in the v0.1 Linux readers: KDE async portal overlay for accent on kdeglobals palette, D-Bus portal backend detection (more reliable than XDG_CURRENT_DESKTOP), GNOME font reading from gsettings/dconf, and kdeglobals fallback for non-KDE desktops. |
| Complexity | MEDIUM |
| Dependencies | Existing `kde` and `portal` feature flags |

**What's being added:**
- `from_kde_with_portal()`: async function that overlays portal accent color on top of kdeglobals palette
- Portal backend detection via D-Bus: check for `org.freedesktop.impl.portal.desktop.gtk`/`kde`/`cosmic` to identify DE without relying on env var
- GNOME font reading: `org.gnome.desktop.interface font-name` and `monospace-font-name` via gsettings/dconf (currently hardcoded to Adwaita Sans 11pt)
- kdeglobals fallback: try reading `~/.config/kdeglobals` even on non-KDE desktops (file may exist from KDE apps)

---

## Anti-Features

Features to explicitly NOT build in v0.2.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| Built-in change notification system | Complex, opinionated (which async runtime?), duplicates what every GUI toolkit already provides. Each toolkit has its own event loop. Building a mini-runtime is scope creep. | Document the platform-specific notification sources in a guide. Users can poll `from_system()` or use their toolkit's appearance observer. Defer to post-1.0 when demand is clearer. |
| Color manipulation utilities (darken, lighten, contrast ratio) | Out of scope for a data crate. The `palette` crate does this comprehensively. Duplicating it adds maintenance burden and bloats the API surface. | Connectors that need derived colors (hover/active variants) can use the `palette` crate or simple arithmetic on Rgba values. Document the conversion pattern. |
| egui connector in v0.2 | egui has the least structured theming API (single `Visuals` struct, no per-widget styling traits). An egui connector is simpler to build but provides less value than gpui/iced connectors. egui's API also changes frequently between minor versions. | Defer to v0.3 or community contribution. Document the ~50-line adapter pattern in examples. |
| iOS/Android runtime readers | Small Rust GUI audience on mobile in 2026. Requires device testing infrastructure. | Ship iOS and Material preset TOML files for static theming. Defer runtime readers. |
| Widget-level animation/transitions | Animation is rendering concern, not data concern. Each toolkit has its own animation system. | Consumers can lerp between ThemeVariant values using their toolkit's animation primitives. |
| Exhaustive widget metrics (every possible dimension) | KDE has ~80 constants. Modeling all of them creates massive structs that are 95% None on non-KDE platforms. Diminishing returns past the core measurements. | Model only the measurements that connectors actually consume: min_height, padding, indicator_size, width. Start with 12 widget types, 2-5 fields each. Expand based on connector demand. |

---

## Feature Dependencies

```
[Flat ThemeColors refactor]
    blocks -> [All preset updates]
    blocks -> [All reader updates]
    blocks -> [macOS reader] (writes to flat fields)

[API methods refactor (preset -> NativeTheme::preset)]
    blocks -> [Publishing prep] (API must be stable)

[ThemeGeometry: radius_lg, shadow]
    independent (non-breaking, #[non_exhaustive])

[macOS reader]
    requires -> [Flat ThemeColors]
    requires -> [macos feature flag + objc2-app-kit dep]
    blocks -> [from_system() macOS dispatch]

[Widget Metrics data model]
    requires -> [Flat ThemeColors] (do breaking changes together)
    blocks -> [gpui-component connector]
    blocks -> [iced connector]

[Workspace restructuring]
    blocks -> [gpui-component connector]
    blocks -> [iced connector]
    blocks -> [Publishing individual crates]

[gpui-component connector]
    requires -> [Workspace restructuring]
    requires -> [Widget Metrics] (for per-widget styling)
    cannot publish until gpui-component is on crates.io

[iced connector]
    requires -> [Workspace restructuring]
    requires -> [Widget Metrics] (for per-widget styling)
    can publish to crates.io immediately

[Windows reader enhancements]
    independent (extends existing feature)

[Linux reader enhancements]
    independent (extends existing features)

[CI pipeline]
    should run after macOS reader (to test on macOS runner)
    should run after workspace restructuring (to test all crates)

[Publishing prep]
    requires -> [All API changes complete]
    requires -> [CI pipeline green]
    should be last step
```

**Critical path:** Flat ThemeColors -> Widget Metrics data model -> Workspace restructuring -> Connectors -> Publishing prep.

**Parallelizable work:** Windows enhancements, Linux enhancements, and macOS reader can proceed in parallel once ThemeColors is flat. CI pipeline can be set up early and iterated.

---

## MVP Recommendation for v0.2

### Must ship (core value of v0.2):

1. **Flat ThemeColors** -- breaking change, do first. Simplifies everything downstream.
2. **API methods on NativeTheme** -- breaking change, do with #1.
3. **ThemeGeometry additions** -- non-breaking, quick win.
4. **macOS reader** -- completes the 4-platform coverage story.
5. **Workspace restructuring** -- enables connectors.
6. **Publishing prep** -- the entire point is getting on crates.io.

### Should ship (high value, moderate effort):

7. **Widget Metrics data model** -- enables meaningful connector work.
8. **iced connector** -- can publish immediately, high-star-count audience.
9. **CI pipeline** -- quality gate for publishing.
10. **Windows reader enhancements** -- fills visible gaps.
11. **Linux reader enhancements** -- fills visible gaps.

### Stretch (ship if time permits):

12. **gpui-component connector** -- high value but blocked on upstream availability. Build it, but accept it may be git-dep-only for now.

---

## Complexity Assessment Summary

| Feature | Complexity | LOC Estimate | Risk |
|---------|-----------|-------------|------|
| Flat ThemeColors | MEDIUM | ~200 (model) + ~500 (preset updates) + ~200 (reader updates) | Low -- mechanical refactor |
| API methods | LOW | ~50 | Low -- straightforward move |
| ThemeGeometry additions | LOW | ~10 | None |
| macOS reader | HIGH | ~300-400 | Medium -- P3 color space conversion, NSAppearance resolution |
| Widget Metrics | HIGH | ~400 (model) + ~200 (per platform) | Medium -- getting the right abstraction level |
| Workspace restructuring | MEDIUM | ~100 (config) | Low -- well-documented Cargo pattern |
| gpui connector | HIGH | ~500-700 | High -- 108 color fields to map, upstream PR dependency |
| iced connector | MEDIUM | ~300-400 | Low -- well-documented iced API, can publish immediately |
| CI pipeline | LOW | ~100 (YAML) | None |
| Publishing prep | LOW | ~100 (metadata + docs) | None |
| Windows enhancements | MEDIUM | ~150 | Low -- extending existing reader |
| Linux enhancements | MEDIUM | ~200 | Low -- extending existing readers |

---

## Sources

- [Apple NSColor documentation](https://developer.apple.com/documentation/appkit/nscolor) -- semantic color properties, color space conversion (HIGH confidence)
- [Apple Standard Colors](https://developer.apple.com/documentation/appkit/standard-colors) -- complete list of system colors (HIGH confidence)
- [NSColor catalog colors gist (macOS Sonoma 14.4)](https://gist.github.com/martinhoeller/38509f37d42814526a9aecbb24928f46) -- verified color names (HIGH confidence)
- [objc2-app-kit docs.rs](https://docs.rs/objc2-app-kit/latest/objc2_app_kit/struct.NSColor.html) -- Rust bindings for NSColor, NSFont, NSAppearance (HIGH confidence)
- [NSAppearance resolution pattern](https://christiantietze.de/posts/2021/10/nscolor-performAsCurrentDrawingAppearance-resolve-current-appearance/) -- how to resolve dynamic colors outside draw context (HIGH confidence)
- [Apple colorUsingColorSpace docs](https://developer.apple.com/documentation/appkit/nscolor/usingcolorspace(_:)) -- P3 to sRGB conversion method (HIGH confidence)
- [gpui-component ThemeColor docs.rs](https://docs.rs/gpui-component/latest/gpui_component/theme/struct.ThemeColor.html) -- 108 color fields, complete field list (HIGH confidence)
- [gpui-component theme documentation](https://longbridge.github.io/gpui-component/docs/theme) -- ActiveTheme trait, ThemeRegistry, theme loading (MEDIUM confidence)
- [iced Palette struct](https://docs.iced.rs/iced/theme/struct.Palette.html) -- 6 base color fields (HIGH confidence)
- [iced Extended palette](https://docs.rs/iced/latest/iced/theme/palette/struct.Extended.html) -- auto-generated shade variants (HIGH confidence)
- [iced button Style](https://docs.iced.rs/iced/widget/button/struct.Style.html) -- background, text_color, border, shadow, snap (HIGH confidence)
- [iced text_input Style](https://docs.rs/iced/latest/iced/widget/text_input/struct.Style.html) -- background, border, icon, placeholder, value, selection (HIGH confidence)
- [iced container Style](https://docs.iced.rs/iced/widget/container/struct.Style.html) -- text_color, background, border, shadow, snap (HIGH confidence)
- [iced widget index](https://docs.rs/iced/latest/iced/widget/index.html) -- complete widget list with Style support (HIGH confidence)
- [iced discourse on styling](https://discourse.iced.rs/t/changing-the-default-styling-of-widget/775) -- Catalog/closure pattern confirmation (MEDIUM confidence)
- [KDE breeze repository](https://github.com/KDE/breeze) -- breezemetrics.h widget constants (HIGH confidence)
- [libadwaita CSS variables](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/css-variables.html) -- complete variable list, no widget sizing variables (HIGH confidence)
- [Windows GetSystemMetrics](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics) -- widget dimension constants (HIGH confidence)
- [Cargo workspace documentation](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) -- workspace conventions, publishing order (HIGH confidence)
- [crates.io publishing guide](https://doc.rust-lang.org/cargo/reference/publishing.html) -- required metadata, keyword/category rules (HIGH confidence)
- [Tweag: Publish all crates at once](https://www.tweag.io/blog/2025-07-10-cargo-package-workspace/) -- workspace publishing improvements (MEDIUM confidence)

---
*Feature research for: native-theme v0.2 (subsequent milestone features)*
*Researched: 2026-03-08*
