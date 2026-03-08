# native-theme: TODO

Execution order reflects dependencies — API-breaking changes first,
then platform work, then publishing prep.

---

## Step 1: API Refactors (breaking changes — do first)

### Flatten ThemeColors to 36 direct fields

The spec (Section 8.4) defines a **flat** `ThemeColors` with 36 direct
`Option<Rgba>` fields. The implementation uses **nested sub-structs**
(CoreColors, ActionColors, etc.). This changes both the Rust API and the
TOML format:

| | Spec (flat) | Implementation (nested) |
|---|---|---|
| Rust API | `colors.accent` | `colors.core.accent` |
| Rust API | `colors.primary` | `colors.primary.background` |
| TOML | `[light.colors]` (one section, 36 keys) | 7 sections: `[light.colors.core]`, `[light.colors.primary]`, ... |
| Adapter code | `v.colors.danger` | `v.colors.status.danger` |

**Recommendation: revert to flat.** Reasons:

1. **Simpler TOML for humans.** The primary value proposition is
   human-editable theme files. One `[colors]` section with 36 keys is more
   approachable than 7 sub-sections. Community preset contributors (who may
   not be Rust developers) don't need to learn the sub-group taxonomy.
2. **Simpler Rust API.** `colors.accent` is shorter and clearer than
   `colors.core.accent`. Every color access saves one level of indirection.
   In a typical adapter (~30 field mappings), this adds up.
3. **Matches ecosystem conventions.** CSS custom properties, shadcn/ui,
   Tailwind, and Material Design tokens all use flat naming
   (`--primary`, `--on-primary`, `--danger`). The nested grouping has no
   precedent outside COSMIC's deeply-coupled theme system.
4. **Naming clarity.** In the flat model, `colors.primary` IS the primary
   action color. In the nested model, `colors.primary` is a struct with
   `.background` and `.foreground` — ambiguous when someone says "the
   primary color."
5. **Easier extensibility.** Adding a new color field means appending to one
   struct. With nested, you must decide which sub-group it belongs to; if
   none fit, adding a new sub-group changes the TOML schema.
6. **The grouping is cosmetic.** No code ever operates on "all status colors"
   or "all panel colors" as a unit. The sub-structs don't enable any
   functionality that the flat struct can't provide.

- [ ] Flatten `ThemeColors` back to 36 direct `Option<Rgba>` fields
- [ ] Update all presets to use flat `[light.colors]` / `[dark.colors]` format
- [ ] Update platform readers (KDE, GNOME, Windows) for flat field access
- [ ] Update tests

### Move preset API to methods on NativeTheme

Spec (Section 10) defines preset functions as **methods on `NativeTheme`**.
The implementation uses **free functions** re-exported from `presets` module.

| | Spec (methods) | Implementation (free functions) |
|---|---|---|
| Load preset | `NativeTheme::preset("kde-breeze")` | `preset("kde-breeze")` |
| List presets | `NativeTheme::list_presets()` | `list_presets()` |
| Parse TOML | `NativeTheme::from_toml(s)` | `from_toml(s)` |
| Load file | `NativeTheme::from_file(path)` | `from_file(path)` |
| Serialize | `theme.to_toml()` | `to_toml(&theme)` |

**Recommendation: switch to methods.** Reasons:

1. **Discoverability.** `NativeTheme::` autocomplete shows all constructors.
2. **Self-documenting.** `NativeTheme::preset("name")` is unambiguous.
3. **Cleaner imports.** Only need `use native_theme::NativeTheme;`.
4. **Idiomatic Rust.** (`File::open()`, `String::from()`, `HashMap::new()`).
5. **Natural `to_toml()`.** `theme.to_toml()` reads better than
   `to_toml(&theme)`.

Note: serde crates use free functions for *generic* deserialization
(`serde_json::from_str::<T>()`, `toml::from_str::<T>()`). Our `from_toml()`
is not generic — it always returns `NativeTheme` — making it a domain-specific
constructor where associated functions are the right pattern.

- [ ] Move preset functions to `impl NativeTheme` associated functions
- [ ] Remove old free functions (no deprecation period — pre-1.0)

### ThemeGeometry: add missing fields

Keep existing fields (`border_opacity`, `scroll_width` — both are used by
platform readers). Add the two from the spec that are missing:

- [ ] Add `radius_lg: Option<f32>` — large element corner radius
- [ ] Add `shadow: Option<bool>` — drop shadows enabled flag
- [ ] Update presets that have large-radius or shadow data

---

## Step 2: Platform Readers

### macOS reader

- [ ] `from_macos()` function (Section 13.5)
- [ ] `objc2-app-kit` dependency with `macos` feature flag (Section 12)
- [ ] Read ~20 NSColor semantic colors → ThemeColors mapping (Section 13.5)
- [ ] P3 → sRGB color space conversion via `colorUsingColorSpace`
- [ ] NSAppearance resolution for dynamic/semantic colors
- [ ] Read `NSFont.systemFont` and `NSFont.monospacedSystemFont` for fonts
- [ ] Hardcoded AppKit geometry (~5px radius)
- [ ] Wire `from_macos()` into `from_system()` dispatch

### Windows reader gaps

- [ ] `ApiInformation::IsMethodPresent` capability check before calling
      `UISettings.GetColorValue` (Section 13.4, Section 6.1)
- [ ] Read `AccentDark1`–`AccentDark3` and `AccentLight1`–`AccentLight3`
      color shades (Section 7.3, 5.2)
- [ ] Read `SystemParametersInfo(SPI_GETNONCLIENTMETRICS)` for system font
      name and size — currently fonts are `Default::default()`
- [ ] Populate spacing from WinUI3 defaults
- [ ] Derive `primary_foreground` as white/black from accent luminance

### Linux reader enhancements

- [ ] `from_kde_with_portal()` — async overlay of portal accent on kdeglobals
      palette (Section 13.1)
- [ ] D-Bus portal backend detection for DE heuristic
      (`org.freedesktop.impl.portal.desktop.gtk` /
      `org.freedesktop.impl.portal.desktop.kde` /
      `org.freedesktop.impl.portal.desktop.cosmic`) — currently only
      `XDG_CURRENT_DESKTOP` env var is checked
- [ ] GNOME gsettings/dconf font reading (`org.gnome.desktop.interface
      font-name`) — currently uses hardcoded Adwaita Sans 11pt defaults
- [ ] `from_linux()` fallback: try kdeglobals if file exists even on non-KDE
      desktops (spec Section 13.3 step 2)

---

## Step 3: Widget Metrics

Per-widget sizing and spacing constants for pixel-perfect native UIs.

### Data model

- [ ] `WidgetMetrics` struct with 12 per-widget sub-structs: `ButtonMetrics`,
      `CheckboxMetrics`, `InputMetrics`, `ScrollbarMetrics`, `SliderMetrics`,
      `ProgressBarMetrics`, `TabMetrics`, `MenuItemMetrics`, `TooltipMetrics`,
      `ListItemMetrics`, `ToolbarMetrics`, `SplitterMetrics`
- [ ] Each sub-struct: all `Option<f32>` fields, `#[non_exhaustive]`,
      serde defaults — same pattern as `ThemeGeometry`
- [ ] Add `widget_metrics: Option<WidgetMetrics>` to `ThemeVariant`

### Platform sources

**KDE (~80 constants from `breezemetrics.h`):**

- [ ] Hardcode constants from
      [KDE/breeze breezemetrics.h](https://github.com/KDE/breeze/blob/master/kstyle/breezemetrics.h)
- [ ] Detect installed Breeze/Plasma version at runtime to select the
      correct constant set
- [ ] Each version's constants are write-once — when a new KDE Plasma ships
      with changed values, add a new version entry; old entries stay untouched

**GNOME/Adwaita (~30 values from CSS variables):**

- [ ] Hardcode values from
      [libadwaita CSS variables](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/css-variables.html)
- [ ] Detect libadwaita version at runtime to select the correct constant set
- [ ] Same write-once versioning: new libadwaita release → new entry,
      old entries immutable

**Windows (~20 dynamic values):**

- [ ] Read via `GetSystemMetrics` and `SystemParametersInfo` — these are
      runtime APIs that return the current OS values, no versioning needed

**macOS:**

- [ ] Hardcoded AppKit/HIG defaults (~10pt radius, standard margins)
- [ ] No per-version constants needed — Apple enforces consistency via HIG,
      and metrics are very stable across macOS versions

### Versioning approach

Hardcoded constants (KDE, GNOME) are immutable historical facts about a
specific toolkit version — once researched and written, they never change.

When a new toolkit version ships with different metrics:

1. Research the new values from source (`breezemetrics.h`, libadwaita CSS docs)
2. Add a new constant set alongside existing ones (old sets untouched)
3. Version detection at runtime picks the closest matching set

Windows and macOS don't need versioning — Windows reads metrics from the live
OS via `GetSystemMetrics`, macOS uses stable HIG defaults.

### Preset files

- [ ] Add widget metrics to existing preset TOML files (each preset pins
      metrics for the toolkit version it represents)

---

## Step 4: CI Pipeline

- [ ] GitHub Actions workflow: test on Linux + Windows + macOS
- [ ] Feature flag matrix: `--no-default-features`, `--features kde`,
      `--features portal-tokio`, `--features windows`, `--features macos`
- [ ] `cargo semver-checks` in CI — catches accidental breaking changes
      (removing public APIs, changing type signatures, etc.). Note:
      `#[non_exhaustive]` on structs means new field additions are already
      non-breaking, so semver-checks won't flag those.
- [ ] `cargo clippy` + `cargo fmt --check`

---

## Step 5: Toolkit Connectors & Examples

### Workspace restructuring

Convert the repo to a Cargo workspace to support connector sub-crates.
The core crate stays publishable to crates.io.

```
native-theme/
├── Cargo.toml              (workspace root)
├── native-theme/
│   ├── Cargo.toml          (core crate — published to crates.io)
│   └── src/
├── native-theme-gpui/
│   ├── Cargo.toml          (connector — path dep on native-theme,
│   │                         git dep on gpui-component)
│   ├── src/lib.rs
│   └── examples/
│       └── showcase.rs     (widget gallery with theme selector)
└── native-theme-iced/
    ├── Cargo.toml          (connector — path dep on native-theme,
    │                         crates.io dep on iced)
    ├── src/lib.rs
    └── examples/
        └── demo.rs         (widget gallery with theme selector)
```

- [ ] Convert repo to Cargo workspace
- [ ] Move core crate into `native-theme/` subdirectory

### native-theme-gpui connector (first priority)

Maps all native-theme data to gpui-component's styling system, so gpui users
get native-harmonious look with one function call:

```rust
let theme = NativeTheme::from_system()?;
native_theme_gpui::apply(&theme, &mut cx);
```

- [ ] Map `ThemeColors` → gpui-component color tokens
- [ ] Map `ThemeFonts` → gpui-component font configuration
- [ ] Map `ThemeGeometry` + `ThemeSpacing` → gpui-component layout values
- [ ] Map `WidgetMetrics` → gpui-component per-widget styling
- [ ] Theme selector widget (dropdown of all presets + OS theme)
- [ ] `examples/showcase.rs` — widget gallery demonstrating all mappings

Note: Cannot publish to crates.io until gpui-component is published there.
Usable via git dependency in the meantime.

#### Upstream PRs to gpui-component

Where the connector needs customization hooks that gpui-component doesn't
expose, submit PRs to gpui-component upstream. Guidelines for acceptance:

- **Frame as "more theming flexibility"** — not "native platform look."
  The maintainers follow shadcn/ui + Apple HIG + Fluent design philosophy;
  they'll accept exposing knobs, not changing defaults.
- **No API breaking changes.** Add new builder methods, new optional theme
  tokens, or new style parameters — never change existing signatures or
  defaults.
- **One concern per PR.** Each PR should expose one category of
  customization (e.g., "allow custom checkbox indicator size via theme
  token" or "expose button padding as configurable").
- **Provide concrete benefit.** Show how the change enables theming use
  cases (screenshots of before/after with different themes help).
- **Follow their CONTRIBUTING.md.** AI-generated code must be disclosed
  and human-reviewed. Default cursor for buttons (not pointer). Medium
  sizes as default.

Checklist of likely needed PRs (discover exact gaps during connector work):

- [ ] Audit gpui-component widgets for hardcoded values that should be
      theme tokens (padding, icon sizes, corner radii, spacing)
- [ ] PR: expose per-widget padding/margin as theme-configurable
- [ ] PR: expose checkbox/radio indicator size as theme token
- [ ] PR: expose scrollbar dimensions as theme-configurable
- [ ] PR: expose button min-height and icon spacing as theme tokens
- [ ] Additional PRs as gaps are discovered during connector implementation

### native-theme-iced connector

Maps native-theme data to iced's `Style` / `Appearance` system. iced has
the strongest per-widget styling of the pure-Rust toolkits (trait-based
`StyleSheet` per widget type), gradients, shadows, and `Canvas` + lyon for
custom 2D drawing. COSMIC desktop proves this approach works at scale.

```rust
let theme = NativeTheme::from_system()?;
native_theme_iced::apply(&theme, &mut settings);
```

- [ ] Map `ThemeColors` → iced `Theme` / custom palette
- [ ] Map `ThemeFonts` → iced `Font` configuration
- [ ] Map `ThemeGeometry` + `ThemeSpacing` → iced `Style` (rounding, spacing)
- [ ] Map `WidgetMetrics` → per-widget `StyleSheet` implementations
- [ ] Theme selector widget (dropdown of all presets + OS theme)
- [ ] `examples/demo.rs` — widget gallery demonstrating all mappings

Note: iced is on crates.io, so `native-theme-iced` can be published
immediately.

---

## Step 6: Publishing Prep

- [ ] `rust-version = "1.85"` in Cargo.toml (edition 2024 minimum)
- [ ] `repository = "https://github.com/tiborgats/native-theme"`
- [ ] `homepage` and `documentation` fields
- [ ] `keywords = ["theme", "native", "gui", "colors", "desktop"]`
- [ ] `categories = ["gui", "config"]`
- [ ] LICENSE-MIT and LICENSE-APACHE and LICENSE-0BSD files at repo root
- [ ] CHANGELOG.md
- [ ] Doc examples (`/// # Examples`) on `NativeTheme`, `Rgba`, `ThemeVariant`
      — these compile via `cargo test --doc` and show on docs.rs
- [ ] Update IMPLEMENTATION.md spec to match actual implementation where the
      implementation is better (serde_with, manual XDG path resolution,
      feature-gated deps in `[dependencies]`, nested→flat ThemeColors)
- [ ] Create `docs/new-os-version-guide.md` — step-by-step instructions for
      an LLM agent on what to do when a new OS or toolkit version is released
      (where to find new metric values, how to add a version entry, how to
      update presets, how to test, how to detect the version at runtime)
- [ ] Publish to crates.io

---

## Post-1.0 / Deferred

### Change notification
Ship without it. Users can poll `from_system()` or use their toolkit's
appearance observer. Add when there's demand.

- [ ] Linux portal: `SettingChanged` D-Bus signal via ashpd stream
- [ ] Linux KDE: `notify` crate file watching (`watch` feature)
- [ ] macOS: ObjC notification observers
- [ ] Windows: `UISettings.ColorValuesChanged` event

### Mobile readers
- [ ] iOS: `from_ios()` via `objc2-ui-kit`
- [ ] Android: `from_android()` via `jni` + `ndk`, Material You (API 31+)
