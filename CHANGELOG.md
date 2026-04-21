# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.8] - Unreleased

### Changed

- Declared explicit `[package.metadata.docs.rs]` targets (`x86_64-unknown-linux-gnu`, `x86_64-apple-darwin`, `x86_64-pc-windows-msvc`) with `all-features = true` on `native-theme`, `native-theme-gpui`, and `native-theme-iced`. Preserves multi-target API rendering on docs.rs after the 2026-05-01 policy change that reduces the default build set to a single target ([announcement](https://blog.rust-lang.org/2026/04/04/docsrs-only-default-targets/)). Platform-gated items such as `LinuxDesktop`, `winicons`, `sficons`, and the macOS/Windows readers remain visible across the three published target docs.

## [0.5.7] - 2026-04-21

> **Major API overhaul.** This release renames the four core vocabulary types,
> restructures the crate into public submodules, replaces free icon-loading
> functions with a builder API, migrates strings to zero-copy types, and
> replaces the `define_widget_pair!` macro with a proc-macro derive.

### Breaking Changes

#### Type renames

| Old | New |
|-----|-----|
| `ThemeSpec` | `Theme` |
| `ThemeVariant` | `ThemeMode` |
| `ResolvedThemeVariant` | `ResolvedTheme` |
| `ResolvedThemeDefaults` | `ResolvedDefaults` |

#### Module restructure

The crate root is now partitioned into public submodules. Most types that were
previously re-exported at the crate root are now accessed through their module:

- `native_theme::theme::*` -- `Theme`, `ThemeMode`, `ResolvedTheme`, `ResolvedDefaults`, `IconSet`, `IconRole`, `AnimatedIcon`, etc.
- `native_theme::icons::*` -- `IconLoader`
- `native_theme::detect::*` -- `system_is_dark()`, `prefers_reduced_motion()`, `LinuxDesktop`, etc.
- `native_theme::color::*` -- `Rgba`
- `native_theme::error::*` -- `Error`, `ErrorKind`
- `native_theme::resolve::*` -- `ResolutionContext` (post-G7; inheritance and validate internals stay `pub(crate)`)
- `native_theme::prelude` -- convenience re-exports (`Theme`, `ResolvedTheme`, `SystemTheme`, `AccessibilityPreferences`, `ResolutionContext`, `Rgba`, `Error`, `Result`)

#### Icon loading API

The 13 standalone icon-loading functions (`load_icon`, `load_custom_icon`,
`load_icon_from_theme`, `load_system_icon_by_name`, `loading_indicator`, etc.)
are replaced by `IconLoader`, a single fluent builder:

```rust,ignore
// Before
let icon = load_icon(IconRole::ActionCopy, IconSet::Material, None);
let anim = loading_indicator(IconSet::Material);

// After
let icon = IconLoader::new(IconRole::ActionCopy).set(IconSet::Material).load();
let anim = IconLoader::new(IconRole::StatusBusy).set(IconSet::Material).load_indicator();
```

#### String type migrations

- `Theme.name`, `SystemTheme.name`, `ThemeDefaults.icon_theme` -- `String`/`Option<String>` → `Cow<'static, str>`/`Option<Cow<'static, str>>`
- `FontSpec.family`, `ResolvedFontSpec.family` -- `Option<String>`/`String` → `Option<Arc<str>>`/`Arc<str>`
- `IconData::Svg` -- `Svg(Vec<u8>)` → `Svg(Cow<'static, [u8]>)`
- `IconProvider::icon_svg()` return type -- `Option<&'static [u8]>` → `Option<Cow<'static, [u8]>>`

#### Error restructure

`Error` is now a flat, `#[non_exhaustive]` enum with 9 variants. `Error::kind()`
returns `ErrorKind` for coarse dispatch.

#### Other breaking changes

- `ColorMode` enum replaces `is_dark: bool` on `SystemTheme`; `SystemTheme.mode` is now `ColorMode`, `pick()` takes `ColorMode`
- `ThemeMode::into_resolved()` signature changed from `(font_dpi: Option<f32>)` to `(ctx: &ResolutionContext)` (see G7 ResolutionContext section below)
- `BorderSpec` split into `DefaultsBorderSpec` (for `ThemeDefaults`) and `WidgetBorderSpec` (for per-widget use)
- `AnimatedIcon` variant fields made private via `FramesData`/`TransformData` wrappers; duration fields now `NonZeroU32`
- `ThemeChangeEvent::ColorSchemeChanged` renamed to `ThemeChangeEvent::Changed`; `Other` variant removed
- `FontSize::to_px()` renamed to `FontSize::to_logical_px()`
- `detect_linux_de()` split into `parse_linux_desktop()` (pure) and `detect_linux_desktop()` (reads env)
- `icon_set` and `icon_theme` relocated from `ThemeMode` to `Theme`/`ThemeDefaults`
- `from_toml_with_base()` removed
- Platform reader functions (`from_kde`, `from_gnome`, `from_macos`, `from_windows`) demoted from `pub` to `pub(crate)`
- Feature flags `portal-tokio` and `portal-async-io` replaced by single `portal` feature (async-io only)
- `from_system()` and `from_system_async()` unified; `from_system()` uses `pollster` for sync-over-async on Linux

#### Icon loading API — typed per-set loaders (Phase 93-09)

The `IconLoader` builder introduced earlier in this release is itself replaced
by five typed per-set loader structs. Phase 93-03 exposed a silent-ignore bug
where `IconLoader::new(name).set(Freedesktop).theme("Adwaita").load()` silently
dropped `.theme()` for string-name lookups; this migration makes that class of
bug impossible by construction — calling a set-specific method on the wrong
loader is now a compile error, not a silent no-op.

```rust,ignore
// Before (IconLoader)
let icon = IconLoader::new(IconRole::ActionCopy).set(IconSet::Material).load();
let fd   = IconLoader::new("edit-copy").set(IconSet::Freedesktop).theme("Adwaita").size(24).color([0,0,0]).load();
let anim = IconLoader::new(IconRole::StatusBusy).set(IconSet::Material).load_indicator();

// After (typed per-set loaders)
let icon = MaterialLoader::new(IconRole::ActionCopy).load();
let fd   = FreedesktopLoader::new("edit-copy").theme("Adwaita").size(24).color([0,0,0]).load();
let anim = MaterialLoader::load_indicator();
```

| Old | New |
|-----|-----|
| `IconLoader::new(id).set(IconSet::Freedesktop).theme("X").size(24).color(c).load()` | `FreedesktopLoader::new(id).theme("X").size(24).color(c).load()` |
| `IconLoader::new(id).set(IconSet::Material).load()` | `MaterialLoader::new(id).load()` |
| `IconLoader::new(id).set(IconSet::Lucide).load()` | `LucideLoader::new(id).load()` |
| `IconLoader::new(id).set(IconSet::SfSymbols).load()` | `SfSymbolsLoader::new(id).load()` |
| `IconLoader::new(id).set(IconSet::SegoeIcons).load()` | `SegoeIconsLoader::new(id).load()` |
| `IconLoader::new(id).set(set).load()` (runtime set) | `load_icon(id, set)` (free fn) |
| `IconLoader::new(id).set(set).load_indicator()` (runtime set) | `load_icon_indicator(set)` (free fn) |

`FreedesktopLoader::load_indicator(theme: Option<&str>)`, `MaterialLoader::load_indicator()`, and `LucideLoader::load_indicator()` are associated functions (no `self`). `SfSymbolsLoader` and `SegoeIconsLoader` do not have `load_indicator` — those sets have no animated spinner, and calling it is a compile error rather than a silent `None`.

As a secondary fix, `freedesktop::load_freedesktop_spinner` now accepts
`theme: Option<&str>` to honor theme overrides for animated spinners,
closing a latent silent-drop that existed since the original freedesktop
spinner support.

#### Resolution-time inputs — `ResolutionContext` (Phase 94-02, G7)

The `font_dpi: Option<f32>` parameter on `ThemeMode::into_resolved`, the
matching field on the internal `OverlaySource`, and the implicit
`platform_button_order()` / `system_icon_theme()` calls inside
`resolve_platform_defaults` / `run_pipeline` are consolidated into a
first-class `ResolutionContext` struct that bundles the three
resolution-time inputs captured from the OS.

```rust,ignore
// Before
let resolved = variant.into_resolved(None)?;           // auto-detect DPI
let resolved = variant.into_resolved(Some(96.0))?;     // explicit DPI

// After
use native_theme::ResolutionContext;

let resolved = variant.resolve_system()?;              // OS-detected
let resolved = variant.into_resolved(&ResolutionContext::from_system())?;
let resolved = variant.into_resolved(&ResolutionContext::for_tests())?; // tests
```

`ResolutionContext` carries:
- `font_dpi: f32` — not `Option<f32>`; the `None`→`system_font_dpi()` fallback
  is resolved once at construction.
- `button_order: DialogButtonOrder` — what `platform_button_order()` returned
  at construction time.
- `icon_theme: Option<Cow<'static, str>>` — runtime system fallback used by
  the pipeline's three-tier icon_theme precedence (per-variant → Theme-level
  → this fallback).

Key design choices (per docs/todo_v0.5.7_gaps.md §G7 and doc 2 §J.2):

- **No `impl Default`.** Runtime-detected types must signal intent at the
  call site. Use `from_system()` for production or `for_tests()` for
  deterministic test values (96 DPI, `PrimaryRight`, no `icon_theme`).
- **`&ResolutionContext` parameter, not `Option<&ResolutionContext>`.** The
  None-overload would reintroduce the silent-default anti-pattern. Explicit
  shortcut `resolve_system()` covers the OS-detected path.
- **`resolve_system()` placed on `ThemeMode`, not `Theme`** (deviation from
  gap doc §G7 step 4). Rationale: `Theme` has both light and dark variants;
  explicit variant selection via
  `theme.into_variant(mode)?.resolve_system()` is unambiguous.
- **`AccessibilityPreferences` stays on `SystemTheme`**, not on the
  context (per ACCESS-01 / J.2 B4 refinement). Accessibility is a
  render-time concern, not a resolve-time concern.

Internal change: `OverlaySource.font_dpi: Option<f32>` replaced by
`OverlaySource.context: ResolutionContext`. Consumers are not affected
(the type has always been `pub(crate)`).

Migration is mechanical across 43 call sites in 18 files. No deprecation
shim — v0.5.7 is the no-backcompat window.

### Added

#### native-theme-derive (new crate)

- `#[derive(ThemeWidget)]` proc macro replaces the old `define_widget_pair!` macro,
  generating paired Option/Resolved struct hierarchies, merge logic, validation,
  range checks, and field-level inheritance

#### native-theme (core)

- `IconLoader` builder struct for all icon loading operations
- `ColorMode` enum (`Light`, `Dark`) with `is_dark()` method
- `AccessibilityPreferences` struct on `SystemTheme` (text_scaling_factor, reduce_motion, high_contrast, reduce_transparency)
- `DiagnosticEntry` enum and `PlatformPreset` struct for diagnostic reporting
- `FrameList` newtype wrapping `Vec<IconData>` with non-empty guarantee
- `DetectionContext` struct with `ArcSwapOption` caches for is_dark, reduced_motion, icon_theme
- `prelude` module with 8 convenience re-exports (post-G7: `Theme`, `ResolvedTheme`, `SystemTheme`, `AccessibilityPreferences`, `ResolutionContext`, `Rgba`, `Error`, `Result`)
- `IconRole::name()` method
- `Rgba` named constants: `TRANSPARENT`, `BLACK`, `WHITE`
- `#[non_exhaustive]` on `LinuxDesktop` enum; new variants: `CosmicDe`, `Hyprland`, `Sway`, `River`, `Niri`
- `ThemeMode::resolve_platform_defaults()` for DE-aware `button_order` resolution
- `#[doc(hidden)]` on pipeline intermediates (`ThemeMode::resolve()`, `resolve_all()`, `validate()`, etc.)
- Uniform bare `#[must_use]` convention across the crate
- `inventory`-driven widget registration replacing hand-maintained `VARIANT_KEYS`/`widget_fields()`
- `IconSetChoice` enum, `default_icon_choice()` function, and `list_freedesktop_themes()` function for icon-set selection UIs that need a library-level "use the platform default" option and an enumeration of available freedesktop themes; all three are re-exported from the crate root
- `Theme::resolve(mode)` convenience method — one-shot `Theme`→`Resolved` resolution using a `ResolutionContext::from_system()` internally, preserving the three-tier `icon_theme` precedence (per-variant → `Theme`-level → system fallback) that the `into_variant(mode)?.resolve_system()?` two-step silently dropped at tier 2

#### native-theme-build

- Generated code paths updated for type renames and `Cow<'static, [u8]>` icon data

#### Connectors

- Both `native-theme-gpui` and `native-theme-iced` updated for all type renames, `ColorMode` API, `Arc<str>` font families, and `Cow` icon data

### Fixed

#### native-theme (core)

- Adwaita icon coverage: `IconRole::StatusBusy` now maps to `content-loading-symbolic`, and `IconRole::DialogSuccess` now maps to `object-select-symbolic`, matching freedesktop spec coverage under the Adwaita theme
- PNG-only icon themes (e.g. `AdwaitaLegacy`) now decode to RGBA at load time so they render instead of producing blank output
- `watch/kde` backend ignores non-mutation filesystem events, preventing a feedback loop that could fire repeated `ThemeChangeEvent::Changed` signals on every save

#### Platform readers

- Windows dialog `button_order` is now `PrimaryLeft` per Microsoft's Windows UX Guidelines (previously `PrimaryRight` by default)

### Removed

#### native-theme (core)

- `Rgba::to_f32_tuple` -- use `to_f32_array()` and destructure
- `IconSet::default()` -- use `system_icon_set()` for the platform-appropriate icon set
- `define_widget_pair!` macro -- replaced by `#[derive(ThemeWidget)]` proc macro
- `from_toml_with_base()` -- use `Theme::preset()` + `merge()` instead
- `ThemeChangeEvent::Other` variant
- `ENV_MUTEX` test infrastructure (`test_util.rs`)

## [0.5.6] - 2026-04-10

### Added

#### native-theme (core)
- **Runtime theme watcher** (`watch` feature): `ThemeWatcher` struct, `ThemeChangeEvent` enum, and `on_theme_change()` async entry point for receiving live OS theme changes
  - KDE backend: inotify watching on `~/.config/kdeglobals` with parent-directory watching and debounce
  - GNOME backend: D-Bus portal `SettingChanged` signal via `zbus::blocking`
  - macOS backend: `NSDistributedNotificationCenter` via CFRunLoop
  - Windows backend: COM STA `UISettings.ColorValuesChanged` event
- **GTK symbolic icon recoloring**: `fg_color: Option<[u8; 3]>` parameter added to `load_icon()`, `load_custom_icon()`, `load_icon_from_theme()`, and `load_system_icon_by_name()` for tinting monochrome SVGs
- GTK symbolic icon normalization: viewBox/dimension inference, `currentColor` fill injection, class attribute stripping
- `GnomePortalData` struct and `build_gnome_spec_pure()` pure function for testable GNOME reader
- `from_kde_content_pure()` pure function for testable KDE reader
- 7 KDE fixture `.ini` files for deterministic testing
- 10 inline tests for `build_gnome_spec_pure`
- KDE edge-case fixture tests
- `ValidateNested` trait and `validate_widget!()` codegen in `define_widget_pair!` macro

### Changed

#### native-theme (core)
- `lib.rs` module split: extracted `detect.rs`, `pipeline.rs`, `icons.rs` into standalone modules
- `validate.rs` refactored: per-widget range checks moved to generated `check_ranges()` methods, helpers extracted to `validate_helpers.rs`
- Widget validation now uses generated `validate_widget()` calls instead of hand-written extraction
- Showcase examples now include runtime theme watcher integration
- Documentation: color role count updated from 22 to 24, `watch` feature added to feature tables, stale API examples corrected

## [0.5.5] - 2026-04-09

### Breaking Changes

> **Migration required.** This release renames ~70 fields, replaces flat border fields with `BorderSpec` sub-structs, removes `ThemeSpacing`, and changes per-widget foreground fields to `font.color`. Custom theme files and code accessing theme fields must be updated.

#### Field renames (~70 fields)

Color fields gain `_color` suffix, border fields move to sub-structs, and per-widget foreground fields are replaced by `font.color`:

**Before (v0.5.4 TOML):**

```toml
[light.defaults]
accent = "#3584e4"
background = "#ffffff"
foreground = "#000000"
muted = "#929292"

[light.button]
foreground = "#000000"
border_color = "#c0c0c0"
corner_radius = 6.0
border_width = 1.0
```

**After (v0.5.5 TOML):**

```toml
[light.defaults]
accent_color = "#3584e4"
background_color = "#ffffff"
text_color = "#000000"
muted_color = "#929292"

[light.button]
font = { color = "#000000" }
border = { color = "#c0c0c0", corner_radius = 6.0, line_width = 1.0 }
```

Key renames by category:
- **Colors**: `accent` -> `accent_color`, `background` -> `background_color`, `foreground` -> `text_color`, `muted` -> `muted_color`, `selection` -> `selection_color`, `focus_ring_color` -> `focus_ring_color` (unchanged), `accent_foreground` -> `accent_text_color`, `selection_foreground` -> `selection_text_color`, `disabled_foreground` -> `disabled_text_color`, `danger` -> `danger_color`, `danger_foreground` -> `danger_text_color`
- **Borders**: flat `border_color`, `corner_radius`, `border_width` -> `border.color`, `border.corner_radius`, `border.line_width`
- **Per-widget foreground**: `button.foreground`, `menu.foreground`, etc. -> `button.font.color`, `menu.font.color`, etc.

#### ThemeSpacing removed

`ThemeSpacing` (xs/sm/md/lg/xl) is replaced by `LayoutTheme` with semantic field names.

**Before (v0.5.4 TOML):**

```toml
[light.defaults.spacing]
xs = 2.0
sm = 4.0
md = 8.0
lg = 16.0
xl = 24.0
```

**After (v0.5.5 TOML):**

```toml
[light.layout]
widget_gap = 6.0
container_margin = 6.0
window_margin = 10.0
section_gap = 18.0
```

#### BorderSpec sub-struct

Flat border fields on widgets are now nested under `[widget.border]` TOML tables.

**Before (v0.5.4 TOML):**

```toml
[light.input]
border_color = "#c0c0c0"
corner_radius = 6.0
border_width = 1.0
```

**After (v0.5.5 TOML):**

```toml
[light.input]

[light.input.border]
color = "#c0c0c0"
corner_radius = 6.0
line_width = 1.0
```

#### FontSpec expanded

`FontSpec` gains `style` (`FontStyle` enum) and `color` fields.

**New TOML structure:**

```toml
[light.defaults.font]
family = "Inter"
size = 13.0
weight = 400
style = "normal"
color = "#000000"
```

#### Per-widget foreground removed

Text color is now `widget.font.color` instead of `widget.foreground`.

**Before (v0.5.4):**

```toml
[light.button]
foreground = "#000000"
```

**After (v0.5.5):**

```toml
[light.button]
font = { color = "#000000" }
```

### Added

#### native-theme (core)
- `BorderSpec` / `ResolvedBorderSpec` sub-structs for typed border configuration (color, corner_radius, corner_radius_lg, line_width, opacity, shadow_enabled, padding_horizontal, padding_vertical)
- `FontStyle` enum (`Normal`, `Italic`, `Oblique`) with serde lowercase rename
- `LayoutTheme` / `ResolvedLayoutTheme` replacing `ThemeSpacing` (widget_gap, container_margin, window_margin, section_gap)
- ~70 interactive state color fields across 18 widgets (hover_background, hover_text_color, active_background, disabled_background, etc.)
- Property-based tests (proptest) for TOML round-trip serialization and merge semantics
- Platform-facts cross-reference tests for drift detection between documentation and preset data
- `detect_is_dark()` GTK_THEME env var and `gtk-3.0/settings.ini` fallback for non-GNOME/non-KDE Linux desktops
- iOS platform detection (`target_os = "ios"`) in `detect_platform()`
- Spinner safety guards: dimension validation (width/height > 0), empty frames guard, zero duration guard, single-quote viewBox attribute handling
- gsettings command timeout (2-second) to prevent indefinite blocking
- Iced connector WCAG contrast enforcement for status foreground colors

#### native-theme-gpui
- `from_system()` now returns `(Theme, ResolvedThemeVariant, bool)` 3-tuple (adds `is_dark` flag, matching iced connector)

### Changed

#### native-theme (core)
- Field naming convention aligned with `property-registry.toml` (~70 renames across all structs)
- Resolution engine overhauled: all safety-net invented values removed, proper inheritance per `inheritance-rules.toml`
- `resolve_border()` and `resolve_font()` functions implement sub-field inheritance for all widgets
- All 17 presets rewritten for new schema with explicit `text_scale` and interactive state color values
- `into_resolved()` `#[must_use]` message corrected to reflect consuming semantics

#### native-theme-gpui
- Color derivations (hover, active, disabled) replaced with direct theme field reads where presets now provide explicit values
- Display name in `from_preset()` uses `spec.name` for human-readable output (was using raw preset key)

#### native-theme-iced
- Display name in `from_preset()` already used `spec.name` (consistent with gpui fix)

#### CI
- `pre-release-check.sh` timeout added (30 min max)
- Publish workflow: gpui connector gate added, better error handling
- Async-io variants tested in CI
- Example names disambiguated (`showcase-gpui`, `showcase-iced`)

### Fixed

#### native-theme (core)
- `detect_is_dark()` now works on non-GNOME/non-KDE Linux desktops via GTK_THEME and gtk-3.0/settings.ini fallback (C-1)
- `detect_platform()` returns `"ios"` on iOS targets (C-2)
- `into_resolved()` `#[must_use]` message corrected (C-3)
- Inheritance bugs: `input.selection` used wrong source (INH-1), `dialog.background_color` missing per-platform fallback (INH-2), `card` border inheritance removed where inappropriate (INH-3)

#### CI
- Async-io feature variants tested to prevent compilation regressions
- Example binary names disambiguated to avoid Cargo build conflicts

### Migration Notes

**Migration checklist for v0.5.4 -> v0.5.5:**

1. **Rename color fields** in custom TOML theme files:

   | Before | After |
   |--------|-------|
   | `accent` | `accent_color` |
   | `background` | `background_color` |
   | `foreground` | `text_color` |
   | `muted` | `muted_color` |
   | `selection` | `selection_color` |
   | `accent_foreground` | `accent_text_color` |
   | `disabled_foreground` | `disabled_text_color` |
   | `danger` | `danger_color` |
   | `danger_foreground` | `danger_text_color` |

2. **Update `[spacing]` sections** to `[layout]` with new field names:
   - `xs`/`sm`/`md`/`lg`/`xl` -> `widget_gap`/`container_margin`/`window_margin`/`section_gap`

3. **Move flat border fields** to `[widget.border]` sub-tables:
   - `border_color` -> `border.color`
   - `corner_radius` -> `border.corner_radius`
   - `border_width` -> `border.line_width`

4. **Replace `widget.foreground`** with `widget.font.color` in TOML and Rust code:
   - `button.foreground` -> `button.font.color`
   - `menu.foreground` -> `menu.font.color`
   - (applies to all widgets with text)

5. **Update Rust code** accessing resolved theme structs:
   - `resolved.defaults.accent` -> `resolved.defaults.accent_color`
   - `resolved.defaults.background` -> `resolved.defaults.background_color`
   - `resolved.button.foreground` -> `resolved.button.font.color`

## [0.5.4] - 2026-04-04

### Added

#### native-theme (core)
- `xdg_current_desktop()` helper for consistent `XDG_CURRENT_DESKTOP` parsing
- `Display` implementations for `IconRole` and `IconSet`
- Resolve safety nets: `line_height`, `button_order`, `accent_foreground`, `shadow`, `disabled_foreground`, `spinner.fill` falls back to `accent`, `text_scale` size ratios and weight defaults
- Validate hardening: NaN/Infinity rejection for geometry fields, `dialog` min/max cross-field validation
- `unpremultiply_alpha()` deduplication across platform readers
- 45+ new tests

#### native-theme-build
- `emit_cargo_directives()` returns `Result` instead of calling `process::exit()`
- Builder validation deferred to `generate()` (no `assert!` panics in `crate_path()`/`derives()`)
- Path traversal rejection for TOML source paths
- `crate_path` and `derives` Rust path validation
- Invisible Unicode rejection in role names and paths
- Bundled DE-aware mapping entries now produce `BuildError`
- Post-merge theme overlap validation
- Theme directory existence check
- Name normalization warnings for non-kebab-case identifiers
- 39 new tests, 3 compiled doctests

#### native-theme-gpui
- All 96 `ThemeColor` fields populated in `ThemeConfig` (prevents `apply_config` reset)
- 20+ new helper functions: accessibility queries, typography helpers, layout utilities, spacing calculators
- Animation frames: all-or-nothing semantics (returns `None` if any frame fails)
- SVG colorization: stroke pattern support
- 37 new tests (132 total)

#### native-theme-iced
- Extended palette: all 4 status families overridden (was 2 of 5)
- `apply_overrides` restructured (was dead code)
- `from_preset()`: proper display name, accurate error messages
- `from_system()`: returns OS `is_dark` flag
- SVG colorization: stroke pattern support
- 11 new helper functions
- 31 new tests (88 total)

### Changed

#### native-theme (core)
- Preset corrections:
  - Windows 11: 16 geometry fixes + 3 color fixes
  - Adwaita: 10 geometry fixes + dialog radius + text_scale corrections
  - KDE Breeze: 6 geometry fixes
  - macOS Sonoma: 4 geometry fixes + `button_order` corrected
  - iOS: `button_order` corrected
  - Community presets: `button_order` removed where inappropriate, Solarized border colors fixed, `radius_lg` fixed

#### native-theme-gpui
- Color mapping: `muted_fg` semantic fix, `_light` mode-aware derivation, `list_active` double-opacity fix, scrollbar track derived from resolved theme, WCAG contrast enforcement for status foregrounds, chart color saturation floor, `active_color` near-black fix, overlay respects `reduce_transparency`

#### native-theme-iced
- `from_preset()` uses correct display name and error messages
- `from_system()` propagates OS `is_dark` flag

### Fixed

#### native-theme-gpui
- `list_active` double-opacity rendering
- `active_color` near-black derivation
- Chart colors losing saturation at low lightness

#### native-theme-iced
- Extended palette: missing status family overrides caused iced defaults to leak through
- `apply_overrides` dead code path that prevented custom palette entries from taking effect

## [0.5.3] - 2026-04-01

### Added

- `ThemeVariant::resolve_platform_defaults()` — separated platform-dependent resolution (icon theme detection) from the pure data-transform `resolve()`
- `ThemeVariant::resolve_all()` — convenience for `resolve()` + `resolve_platform_defaults()`
- `ThemeSpec::from_toml_with_base()` — merge custom TOML overrides onto a named base preset in one call
- `ThemeSpec::lint_toml()` — detect unrecognized field names in TOML theme files (opt-in linting for theme authors)
- `detect_is_dark()`, `detect_reduced_motion()`, `detect_icon_theme()` — uncached polling variants of `system_is_dark()`, `prefers_reduced_motion()`, `system_icon_theme()`
- `diagnose_platform_support()` — human-readable diagnostic messages for OS theme detection availability
- `FIELD_NAMES` const on all 25 per-widget `Option` structs and `ThemeDefaults` (used by `lint_toml()`)
- Range validation in `validate()` for ~50 geometry, opacity, font-size, and font-weight fields
- `switch.unchecked_background` resolve rule (falls back to `muted`)
- gpui connector: `bundled_icon_to_image_source()` — single-call `IconName` + `IconSet` to `ImageSource` conversion
- gpui connector: re-exports `Error`, `TransformAnimation`, `LinuxDesktop` (Linux-only)
- iced connector: `to_iced_weight()` — CSS font weight (100-900) to iced `Weight` enum
- iced connector: `into_image_handle()`, `into_svg_handle()` — consuming variants of the borrow-based helpers
- iced connector: re-exports `Error`, `Result`, `Rgba`, `TransformAnimation`
- Build crate: drift detection tests for `THEME_TABLE` vs `IconSet` and `DE_TABLE` vs `LinuxDesktop`
- Build crate: digit-starting identifier validation (rejects names that produce invalid Rust identifiers)
- Build crate: empty-roles and no-themes warnings for likely misconfiguration

### Changed

- `ButtonTheme`: `primary_bg` → `primary_background`, `primary_fg` → `primary_foreground`
- `SwitchTheme`: `checked_bg` → `checked_background`, `unchecked_bg` → `unchecked_background`, `thumb_bg` → `thumb_background`
- `into_resolved()` now calls `resolve_all()` (includes platform defaults) for backward compatibility
- Preset registry refactored from 20 constants + 20 `LazyLock` + 20-arm match to data-driven `HashMap`
- Community presets no longer hardcode `icon_set = "freedesktop"`
- gpui `from_preset()` and `from_system()` return `(Theme, ResolvedThemeVariant)` tuple
- gpui `to_theme()` sets all `Theme` fields directly — eliminates `apply_config` workaround and radius truncation
- gpui `animated_frames_to_image_sources()` accepts `color` and `size` parameters
- gpui `into_image_source()` delegates to `to_image_source()` instead of duplicating logic
- iced `to_theme()` captures 4 `Copy` Rgba values instead of cloning entire `ResolvedThemeVariant`
- iced `from_preset()` uses `into_variant()` (avoids clone); `from_system()` moves variant
- iced `line_height()` renamed to `line_height_multiplier()` (returns raw multiplier, not pixels)
- iced `animated_frames_to_svg_handles()` accepts `color` parameter
- Build crate: `GenerateOutput::emit_cargo_directives()` returns `()` instead of `io::Result<()>` — handles errors internally
- Build crate: `IconGenerator::crate_path()` and `derive()` validate input (assert on empty/whitespace)
- Build crate: role deduplication in `merge_configs()` via `BTreeSet`

### Removed

- gpui connector: `pick_variant()` free function (use `ThemeSpec::into_variant()`)

### Fixed

- gpui BMP encoder validates dimensions and detects overflow (returns `Option` instead of panicking)
- gpui `colorize_svg()` returns original bytes on non-UTF-8 input (prevents data corruption)
- iced `colorize_monochrome_svg()` returns original bytes on non-UTF-8 input
- Build crate: Windows path separators in generated `include_bytes!` paths (backslash → forward slash)
- Build crate: empty config early return in pipeline (prevents index panic)
- Build crate: orphan SVG check emits warning instead of silently ignoring unreadable directories

## [0.5.2] - 2026-03-31

### Added

- `Deserialize` derive on all `Resolved*` types (enables caching, IPC, test fixtures)
- `Serialize` and `Deserialize` on `IconData`, `AnimatedIcon`, and `TransformAnimation`
- `Copy`, `Eq`, `Hash` derives on `DialogButtonOrder`; `Eq`, `Hash` on `LinuxDesktop`
- `#[must_use]` on 20+ public functions across all four crates
- `#[non_exhaustive]` on `BuildError` enum
- `Debug` and `Clone` derives on `IconGenerator`, `GenerateOutput`, `AnimatedImageSources`, `AnimatedSvgHandles`
- `into_image_source()` consuming variant in gpui connector
- KDE reader: `accent_foreground`, `list.background`/`foreground` from live color scheme
- GNOME reader: portal `reduce-motion` and gsettings `high-contrast` detection

### Changed

- `Error::Unsupported` now carries a `&'static str` context payload
- `icon_set` field on `ThemeVariant` changed from `Option<String>` to `Option<IconSet>` (validated at parse time)
- `rasterize_svg()` uses `Error::Format` instead of `Error::Unavailable` for invalid dimensions
- `BuildErrors` inner field is now private; access via `errors()`, `into_errors()`, `len()`, `is_empty()`, and `IntoIterator`
- gpui icon mapping functions (`lucide_name_for_gpui_icon`, `material_name_for_gpui_icon`, `freedesktop_name_for_gpui_icon`) return `&'static str` instead of `Option<&'static str>`
- iced `from_preset()` and `from_system()` return `(Theme, ResolvedThemeVariant)` tuple

### Fixed

- KDE Breeze preset: `radius` 4 -> 5, `focus_ring_width`/`focus_ring_offset` swapped, `line_height` 1.4 -> 1.36, four incorrect `icon_sizes`, `progress_bar.min_width` mismap, `spinner.diameter`, `expander.arrow_size`, `switch` dimensions
- KDE reader: `defaults.border` no longer overwritten with accent color; `forceFontDPI` read from correct file
- Adwaita preset: `radius` 12 -> 9, `radius_lg` 14 -> 15, `line_height` 1.4 -> 1.21, `focus_ring_offset` 1 -> -2, `section_heading` weight 400 -> 700
- macOS Sonoma preset: corrected geometry and metric values across both full and live presets
- Windows 11 preset: corrected geometry and metric values across both full and live presets
- gpui connector: `colorize_svg()` now handles self-closing SVG tags correctly
- Build crate: simplified error handling pipeline, improved codegen

## [0.5.1] - 2026-03-30

### Changed

- Renamed types and tightened visibility across core and build crates
- Build crate Result-based API for validation diagnostics
- Simplified GNOME/KDE readers and polished connector APIs
- Expanded widget resolved types and cleaned up build crate tests

### Fixed

- Windows compilation — swapped `icon_name` args, Rust 2024 unsafe blocks
- macOS compilation errors
- Test compilation — stale call sites after API changes
- iced screenshot delay to avoid blank capture on Windows
- CI: removed tag trigger from docs workflow

## [0.5.0] - 2026-03-28

### Added

- Per-widget data model: 25 `XxxTheme` / `ResolvedXxx` struct pairs (Window, Button, Input, Checkbox, Menu, Tooltip, Scrollbar, Slider, ProgressBar, Tab, Sidebar, Toolbar, StatusBar, List, Popover, Splitter, Separator, Switch, Dialog, Spinner, ComboBox, SegmentedControl, Card, Expander, Link)
- `ThemeDefaults` struct with ~40 global properties (colors, fonts, spacing, icon sizes, accessibility)
- `FontSpec` for per-widget font specification (family, size, weight)
- `TextScale` with 4 typographic roles (caption, section_heading, dialog_title, display)
- `IconSizes` struct (toolbar, small, large, dialog, panel)
- `DialogButtonOrder` enum (TrailingAffirmative / LeadingAffirmative)
- `ThemeSpacing` struct (xxs through xxl)
- `define_widget_pair!` macro generating paired Option/Resolved structs from a single definition
- `ResolvedThemeVariant` type where all fields are guaranteed populated (non-optional)
- `ResolvedThemeDefaults`, `ResolvedFontSpec`, `ResolvedThemeSpacing`, `ResolvedIconSizes`, `ResolvedTextScale`, `ResolvedTextScaleEntry` types
- `ThemeResolutionError` listing missing field paths; `Error::Resolution` variant
- `ThemeVariant::resolve()` with ~90 inheritance rules in 4 phases (defaults-internal, safety-nets, widget-from-defaults, widget-to-widget)
- `ThemeVariant::validate()` producing `ResolvedThemeVariant` or listing all missing fields
- `SystemTheme` type returned by `from_system()` with `active()`, `pick()`, `with_overlay()`, `with_overlay_toml()`
- Live platform presets (geometry-only, internal): `kde-breeze-live`, `adwaita-live`, `macos-sonoma-live`, `windows-11-live`
- `platform_preset_name()` mapping the current OS to its live preset
- `list_presets_for_platform()` filtering presets by current OS
- `system_is_dark()` cross-platform cached dark-mode detection (Linux gsettings/kdeglobals, macOS AppleInterfaceStyle, Windows UISettings)
- KDE reader: per-widget fonts (menuFont, toolBarFont), WM title bar colors, text scale via Kirigami multipliers, icon sizes from index.theme, accessibility (AnimationDurationFactor, forceFontDPI)
- GNOME reader: gsettings fonts (font-name, monospace-font-name, titlebar-font), text scale via CSS percentages, accessibility (text-scaling-factor, enable-animations, overlay-scrolling), icon-theme
- macOS reader: per-widget fonts (+menuFontOfSize:, +toolTipsFontOfSize:, +titleBarFontOfSize:), NSFont.TextStyle text scale, additional NSColor values, scrollbar overlay mode, accessibility (reduce_motion, high_contrast, reduce_transparency, text_scaling_factor)
- Windows reader: NONCLIENTMETRICSW per-widget fonts, DwmGetColorizationColor title bar, GetSysColor widget colors, text scale factor, high contrast, icon sizes via GetSystemMetrics

### Changed

- `ThemeVariant` composes `ThemeDefaults` + 25 per-widget structs instead of flat `ThemeColors`/`ThemeFonts`/`ThemeGeometry`
- `from_system()` and `from_system_async()` return `SystemTheme` instead of `ThemeSpec`
- gpui and iced connector `to_theme()` accept `&ResolvedThemeVariant` instead of `&ThemeVariant`
- All 16 preset TOMLs rewritten for per-widget structure; platform presets slimmed to design constants only
- `impl_merge!` macro extended with `optional_nested` category for per-widget font fields
- Both gpui and iced showcase examples updated for `SystemTheme` / `ResolvedThemeVariant` API

### Removed

- `ThemeColors` flat struct (replaced by `ThemeDefaults` base colors + per-widget color fields)
- `ThemeFonts` struct (replaced by `FontSpec` on `ThemeDefaults` + per-widget font fields)
- `ThemeGeometry` struct (replaced by per-widget geometry fields)
- `WidgetMetrics` and its 12 sub-structs (replaced by per-widget sizing fields on each `XxxTheme`)
- `default` preset (replaced by platform detection via `platform_preset_name()` and live presets)

### Migration from v0.4.x

**Data model:** `variant.colors.accent` -> `variant.defaults.accent`, `variant.fonts.family` -> `variant.defaults.font.family`, `variant.geometry.radius` -> `variant.defaults.radius`. Per-widget fields like `variant.button.min_height` replace `variant.widget_metrics.button.min_height`.

**from_system():**

```rust,ignore
// Before (v0.4.x)
let nt: ThemeSpec = from_system().unwrap_or_else(|_| ThemeSpec::preset("adwaita").unwrap());
let variant = nt.pick_variant(true).unwrap();

// After (v0.5.0)
let system: SystemTheme = from_system().unwrap();
let resolved: &ResolvedThemeVariant = system.active(); // all fields guaranteed
```

**Connectors:**

```rust,ignore
// Before (v0.4.x)
let theme = to_theme(variant, "My App", is_dark);

// After (v0.5.0)
let mut v = variant.clone();
v.resolve();
let resolved = v.validate().unwrap();
let theme = to_theme(&resolved, "My App", is_dark);

// Or from SystemTheme (already resolved):
let theme = to_theme(system.active(), "My App", system.is_dark);
```

## [0.4.1] - 2026-03-20

### Added

- `CONTRIBUTING.md` with development workflow and testing guide
- `CODE_OF_CONDUCT.md` (Contributor Covenant 2.1)
- `SECURITY.md` with responsible disclosure policy
- GitHub issue templates (bug report, feature request) using YAML forms
- Pull request template with CI checklist
- Animated icon sections in gpui and iced connector READMEs
- Animated icon showcase demonstrations in both gpui and iced examples
- CLI argument support (`--tab`, `--preset`) for showcase examples
- GIF generation script for bundled spinner animations
- Screenshot automation (`--screenshot` flag) for iced showcase example
- CI workflow for automated screenshot generation on Linux, macOS, and Windows
- Showcase screenshots embedded in root, iced, and gpui READMEs
- Spinner GIFs embedded in root README
- `#![warn(missing_docs)]` crate-level lint attribute in all workspace crates
- Doc comments for all public API items in native-theme core crate

### Changed

- Root README updated with animated icons section
- Version references updated from 0.3.x to 0.4.x across all documentation

### Fixed

- Broken intra-doc link for `iced::time::every()` in native-theme-iced
- Missing documentation warnings that caused CI failures under `-Dwarnings`
- Formatting violations in gpui showcase example

## [0.4.0] - 2026-03-18

### Added

- `AnimatedIcon` enum with `Frames` and `Transform` variants for animated icon data
- `TransformAnimation` enum with `Spin` variant for continuous rotation
- `Repeat` enum controlling animation looping behavior
- `AnimatedIcon::first_frame()` method returning a static fallback frame
- `loading_indicator(icon_set)` function dispatching to platform-appropriate spinner animations
- `prefers_reduced_motion()` function querying OS accessibility settings (Linux gsettings, macOS NSWorkspace, Windows UISettings)
- Bundled Lucide loader spinner (spin transform) and freedesktop `process-working` sprite sheet loading
- Freedesktop sprite sheet parser for runtime `process-working.svg` animation loading
- gpui connector: `animated_frames_to_image_sources()` and `with_spin_animation()` for animation playback
- iced connector: `animated_frames_to_svg_handles()` and `spin_rotation_radians()` for animation playback

### Changed

- `IconRole::StatusLoading` renamed to `IconRole::StatusBusy` (static icon for busy state)

### Removed

- `IconRole::StatusLoading` variant (use `loading_indicator()` for animated loading indicators, or `IconRole::StatusBusy` for a static busy icon)

### Migration from v0.3.x

**Before (v0.3.x):**

```rust,ignore
use native_theme::{load_icon, IconRole};

// Static loading icon
let icon = load_icon(IconRole::StatusLoading, "material");
```

**After (v0.4.0):**

```rust,ignore
use native_theme::{loading_indicator, prefers_reduced_motion, AnimatedIcon};

// Animated loading indicator with platform-native style
if let Some(anim) = loading_indicator("material") {
    // Check accessibility preference first
    if prefers_reduced_motion() {
        let static_icon = anim.first_frame();
        // Render a single static frame
    } else {
        match &anim {
            AnimatedIcon::Frames(data) => {
                // Cycle through data.frames() on a timer (data.frame_duration_ms().get() ms)
            }
            AnimatedIcon::Transform(data) => {
                // Apply continuous rotation to data.icon() via data.animation()
            }
            _ => {}
        }
    }
}

// If you just need a static busy icon (not animated):
use native_theme::{load_icon, IconRole};
let busy = load_icon(IconRole::StatusBusy, "material");
```

## [0.3.3] - 2026-03-17

### Added

- `IconProvider` trait for defining custom icon types that integrate with native-theme's loading system
- `load_custom_icon()` function dispatching custom icons through the same platform loader chain as built-in icons
- `load_system_icon_by_name()` function for loading platform icons by arbitrary name string
- `native-theme-build` crate: TOML-driven code generation for custom icon roles with `generate_icons()` and `IconGenerator` builder API
- DE-aware code generation: freedesktop mapping TOML entries can specify per-desktop-environment icon names (e.g., `{ kde = "view-visible", default = "view-reveal" }`)
- gpui connector: `custom_icon_to_image_source()` and `custom_icon_to_image_source_colored()` for loading custom icons
- iced connector: `custom_icon_to_image_handle()`, `custom_icon_to_svg_handle()`, and `custom_icon_to_svg_handle_colored()` for loading custom icons
- Icon mapping gap fills: Freedesktop `Notification` -> "notification-active", Material/Lucide `TrashFull` mappings
- Coverage tests: `no_unexpected_icon_gaps` and `all_roles_have_bundled_svg` prevent future mapping regressions

### Changed

- `IconRole` now implements `IconProvider`, delegating to built-in mapping functions
- Platform icon loaders (freedesktop, SF Symbols, Segoe Fluent) return `None` for unmapped roles instead of falling back to Material SVGs

### Removed

- Wildcard Material SVG fallback from `load_icon()` and all platform loaders (icons not found in the requested set now return `None`)

## [0.3.2] - 2026-03-14

### Added

- `ThemeSpec::pick_variant()` method for selecting the appropriate theme variant with cross-fallback
- `#[must_use]` annotations on all public API functions and key types (`ThemeSpec`, `IconData`)

### Changed

- `system_icon_theme()` and `system_is_dark()` now cache results with `OnceLock` (eliminates redundant subprocess spawns)
- `colorize_svg` renamed to `colorize_monochrome_svg` in iced connector with documentation clarifying monochrome-only contract
- Improved `to_theme` comment in gpui connector explaining the `apply_config`/restore pattern
- `pre-release-check.sh` uses `jq` instead of `python3` for JSON parsing (with bash fallback)

### Deprecated

- `pick_variant()` free functions in gpui and iced connectors (use `ThemeSpec::pick_variant()` instead)

### Removed

- Dead `lighten`, `darken`, and `with_alpha` wrapper functions from gpui `derive` module

## [0.3.1] - 2026-03-13

### Added

- Meta-features (`linux-full`, `macos-full`, `windows-full`) for simplified feature gate configuration
- `system_icon_theme()` with DE-aware detection (KDE, GNOME, Xfce, Cinnamon, Mate, LxQt, Budgie)
- `bundled_icon_by_name()` for string-based icon lookup
- `load_freedesktop_icon_by_name()` for arbitrary freedesktop icon lookups
- `LinuxDesktop` enum expanded with Xfce, Cinnamon, Mate, LxQt, Budgie variants
- `LinuxDesktop` and `detect_linux_de()` made public
- Freedesktop icon name mapping for all 86 gpui-component icons
- SVG colorization support in iced connector (`to_svg_handle_colored`)

### Changed

- Target-gated OS dependencies so meta-features compile on all platforms
- Renamed `icon_theme` field to `icon_set` (with serde alias for backward compatibility)
- Updated bundled Material and Lucide SVGs to latest releases (86+ icons each)

### Fixed

- BMP rasterization in gpui connector (red/blue channel swap for colored SVG themes)
- Plasma 6 icon theme detection via `kdedefaults/kdeglobals` fallback
- Symbolic icon preference to avoid animation sprite sheets from freedesktop themes

## [0.3.0] - 2026-03-09

### Added

- Icon system: `IconRole` enum (42 semantic icon roles), `IconSet` enum, `IconData` type
- Bundled SVG icon sets: Material Design and Lucide (86+ icons each, ~300KB total)
- Linux freedesktop icon loading via `freedesktop-icons` crate
- macOS SF Symbols icon loading (compile-time stub with bundled fallback)
- Windows Segoe Fluent Icons loading (compile-time stub with bundled fallback)
- `load_icon()` cross-platform dispatch function
- `rasterize_svg()` for SVG-to-bitmap conversion via `resvg`
- gpui connector: `icon_name()` mapping, `to_image_source()` conversion
- iced connector: `to_svg_handle()` for SVG icon display

## [0.2.0] - 2026-03-09

### Added

- macOS reader (`from_macos()`) with light and dark variant detection
- `WidgetMetrics` with 12 per-widget sub-structs (Button, Checkbox, Input, ListItem, MenuItem, ProgressBar, Scrollbar, Slider, Splitter, Tab, Toolbar, Tooltip)
- `ThemeGeometry::radius_lg` and `shadow` fields for extended geometry support
- Linux D-Bus portal backend detection for improved desktop environment heuristics
- Portal overlay for KDE themes (`from_kde_with_portal()`)
- `native-theme-iced` connector crate for iced toolkit integration
- `native-theme-gpui` connector crate for gpui toolkit integration
- GitHub Actions CI pipeline with cross-platform matrix (Linux, macOS, Windows)
- Windows accent shade colors (AccentDark1-3, AccentLight1-3)
- Windows system font and DPI-aware geometry reading
- GNOME font data population via portal reader
- Async `from_system_async()` with D-Bus portal backend detection

### Changed

- Restructured as Cargo workspace with `native-theme`, `native-theme-iced`, and `native-theme-gpui` crates
- Flattened `ThemeColors` from nested sub-structs to 36 direct `Option<Rgba>` fields
- Moved preset API from free functions to `ThemeSpec` associated methods (`preset()`, `from_toml()`, `from_file()`, `list_presets()`, `to_toml()`)
- Renamed primary/secondary color fields with prefix (`primary_background`, `primary_foreground`, `secondary_background`, `secondary_foreground`)

### Removed

- `CoreColors`, `ActionColors`, `StatusColors`, `InteractiveColors`, `PanelColors`, `ComponentColors` nested sub-structs (replaced by flat `ThemeColors`)
- Free-standing `preset()`, `from_toml()`, `from_file()`, `list_presets()`, `to_toml()` functions (now methods on `ThemeSpec`)

## [0.1.0] - 2026-03-07

### Added

- `ThemeSpec` data model with 22 semantic color roles, fonts, geometry, and spacing
- `Rgba` color type with hex string parsing and serialization
- `ThemeVariant` composing colors, fonts, geometry, and spacing
- TOML serialization and deserialization for all theme types
- 17 bundled presets (platform and community themes)
- KDE reader (`from_kde()`) parsing kdeglobals color scheme
- GNOME portal reader (`from_gnome()`) via D-Bus Settings portal
- Windows reader (`from_windows()`) using Windows registry
- Cross-platform `from_system()` dispatch with automatic desktop detection
- `impl_merge!` macro for recursive Option-based theme merging
- Deep merge support across all theme types

[0.5.8]: https://github.com/tiborgats/native-theme/compare/v0.5.7...HEAD
[0.5.7]: https://github.com/tiborgats/native-theme/compare/v0.5.6...v0.5.7
[0.5.6]: https://github.com/tiborgats/native-theme/compare/v0.5.5...v0.5.6
[0.5.5]: https://github.com/tiborgats/native-theme/compare/v0.5.4...v0.5.5
[0.5.4]: https://github.com/tiborgats/native-theme/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/tiborgats/native-theme/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/tiborgats/native-theme/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/tiborgats/native-theme/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/tiborgats/native-theme/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/tiborgats/native-theme/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/tiborgats/native-theme/compare/v0.3.3...v0.4.0
[0.3.3]: https://github.com/tiborgats/native-theme/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/tiborgats/native-theme/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/tiborgats/native-theme/compare/v0.3...v0.3.1
[0.3.0]: https://github.com/tiborgats/native-theme/compare/v0.2.0...v0.3
[0.2.0]: https://github.com/tiborgats/native-theme/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/tiborgats/native-theme/releases/tag/v0.1.0
