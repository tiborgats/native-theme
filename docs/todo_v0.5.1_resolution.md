# Theme Resolution Pipeline

## Problem

`ThemeVariant` uses `Option<T>` for every field to support the merge/overlay
system, where `None` means "don't override." This is correct for the merge
step, but after a theme is fully resolved — OS detection merged with
platform preset — consumers still receive `Option` fields. They must
choose between panicking (forbidden), fabricating fallback constants
(defeats native theming), or silently accepting `None` (broken layouts).
All three are wrong.

**After resolution, every field must have a value.** If it doesn't, that's
a theme error that must be reported, not hidden.

---

## 1. Pipeline: OS Reader → resolve() → TOML overlay

### Current Behavior

`from_system()` returns the OS-detected theme **without** applying
inheritance or merging the platform's matching preset. On macOS, KDE,
and GNOME this means all spacing, geometry, and derived fields are
`None`.

### Required Behavior

The OS is the primary source. The TOML is an overlay that fills
design-constant gaps and lets app developers customize native values.

```
  OS Reader            resolve()           Platform TOML    App TOML
  (live ⚙ values)      (inheritance)        (design consts)  (overrides)
        │                     │                   │               │
        └─────────────────────┘                   │               │
                  │                               │               │
                  ▼                               │               │
           ThemeVariant                           │               │
           (OS + derived)                         │               │
                  │                               │               │
                  └──── merge(platform TOML) ─────┘               │
                                │                                 │
                                └──── merge(app TOML) ────────────┘
                                                │
                                                ▼
                                         validate()
                                                │
                                                ▼
                                         ResolvedTheme
```

Merge order: **OS + inheritance form the base; TOMLs overlay on top.**
- OS reader provides live colors, fonts, DPI-scaled metrics (⚙ values)
- `resolve()` fills derived fields from OS-provided sources
  (accent → primary_bg, defaults.font → menu.font, etc.)
- Platform default TOML fills design-constant gaps (geometry, spacing,
  widget metrics, non-⚙ colors like Adwaita CSS values on GNOME)
- App TOML (optional) overrides any value the app developer wants
- `Some` values in any TOML always win over the base — this is how
  app developers customize the native look

### Platform-to-Preset Mapping

| Platform detection       | Base preset    |
|--------------------------|----------------|
| macOS (`from_macos`)     | `macos-sonoma` |
| Windows (`from_windows`) | `windows-11`   |
| KDE (`from_kde`)         | `kde-breeze`   |
| GNOME (`from_gnome`)     | `adwaita`      |
| Linux fallback           | `adwaita`      |

### Single-Variant Platforms

Windows, KDE, and GNOME only detect the active variant (light or dark).
macOS detects both. After the pipeline:

- The detected variant = OS reader + resolve() + platform TOML (complete, live)
- The other variant = platform TOML only (design constants, no live OS data)

For platforms where the TOML is minimal (KDE — no colors/fonts in TOML),
the inactive variant will be incomplete. Options:
- Platform TOMLs include static colors/fonts for the inactive variant only
- The inactive variant is simply unavailable until the user switches
- (TBD: decide based on real-world usage patterns)

---

## 2. ResolvedTheme — Non-Optional Output

After the full pipeline (OS reader → resolve → TOML overlays),
convert `ThemeVariant` (with `Option` fields) into a `ResolvedTheme`
(with direct values). The conversion validates that every required
field is `Some`. If any field is still `None`, return an error
listing all missing fields — not a panic.

```
  OS Reader ──────────▶ ThemeVariant (sparse)
                              │
                       resolve() — fills derived Nones
                              │
  Platform TOML ──────▶ merge() — fills design constants
                              │
  App TOML (opt) ─────▶ merge() — app overrides
                              │
                       validate()
                              │
                              ▼
                       ResolvedTheme (plain values)
                              │
                              ▼
                       Application uses directly
```

### ResolvedTheme

```rust
pub struct ResolvedTheme {
    pub colors:         ResolvedColors,
    pub fonts:          ResolvedFonts,
    pub geometry:       ResolvedGeometry,
    pub spacing:        ResolvedSpacing,
    pub widget_metrics: ResolvedWidgetMetrics,
    pub icon_set:       String,
}
```

The ResolvedTheme mirrors the per-widget architecture from the
theme-variant spec. Each widget's Resolved struct contains plain values
(no `Option`) — all inheritance from `ThemeDefaults` has been applied.

See `todo_v0.4.2_theme-variant.md` for the full per-widget struct
definitions. Each `Option` field becomes a concrete value in the
resolved counterpart.

### Two-Phase Resolution

`resolve()` and `validate()` are separate steps:

- **`resolve()`** runs after the OS reader. It fills `None` fields
  from inheritance sources (accent → primary_bg, font → menu.font,
  radius → button.radius, etc.). See `todo_v0.5.1_inheritance-rules.md`
  for the full inheritance table. This is a best-effort step — some
  fields may remain `None` if their source is also `None`.

- **`validate()`** runs after all TOML overlays. It converts
  `ThemeVariant` → `ResolvedTheme`, checking that every required
  field is `Some`. If any field is still `None`, it returns an error.

### Error Reporting

```rust
pub struct ThemeResolutionError {
    /// Every field path that was still None after the full pipeline.
    pub missing_fields: Vec<String>,
}
```

Example output:
```
Theme resolution failed: 3 missing field(s):
  - spacing.xs
  - colors.danger
  - fonts.tooltip.size
```

---

## 3. OS Reader Updates

### 3.1 macOS (`from_macos`)

Currently reads: `systemFontOfSize:`, `monospacedSystemFont...`,
~20 NSColors.

Add:
- `NSFont.smallSystemFontSize` → `caption_size`
- `NSFont.labelFontSize` → `small_size`
- `NSFont.titleBarFontOfSize:` → `title_bar` FontOverride
- `NSFont.menuFontOfSize:` → `menu` FontOverride
- `NSFont.toolTipsFontOfSize:` → `tooltip` FontOverride
- Font weight from `NSFontDescriptor` traits
- `NSColor.placeholderTextColor` → `placeholder`
- `NSColor.windowFrameTextColor` → `title_bar_foreground`
- `NSColor.insertionPointColor` → `caret`
- Title bar background from `windowBackgroundColor` or visual effect material

### 3.2 Windows (`from_windows`)

Currently reads: `lfMessageFont`, UISettings colors, geometry via
`GetSystemMetricsForDpi`, `winui3_spacing()`.

Add:
- `lfCaptionFont` → `title_bar` FontOverride (family, size, weight)
- `lfMenuFont` → `menu` FontOverride
- `lfStatusFont` → `status_bar` FontOverride
- `lfMessageFont` weight → base `weight`
- `DwmGetColorizationColor` → `title_bar` color
- SM_CXFOCUSBORDER / SM_CYFOCUSBORDER → `focus_ring_width`

### 3.3 KDE (`from_kde`)

Currently reads: `font`, `fixed` from [General], colors from Colors:*
sections.

Add:
- `smallestReadableFont` → `caption_size` (parse field 1)
- `toolBarFont` → `toolbar` FontOverride
- `menuFont` → `menu` FontOverride
- `activeFont` → `title_bar` FontOverride
- Qt font field 4 → `weight` for all font keys
- `[WM] activeBackground` → `title_bar`
- `[WM] activeForeground` → `title_bar_foreground`
- `[WM] frame` or `[WM] inactiveBackground` → `window_border`
- `ForegroundInactive` from Colors:View → `placeholder`

### 3.4 GNOME

Currently uses: bundled Adwaita preset + portal accent overlay.

**Architecture change:** The GNOME reader should NOT embed the Adwaita
preset internally. It should only read OS values. Adwaita CSS colors
(non-⚙ design constants) belong in the `adwaita.toml` platform TOML,
which overlays after the reader. This makes `adwaita.toml` larger
than other platform TOMLs (it has all CSS-derived colors), which is
correct — GNOME exposes fewer values via APIs than KDE or macOS.

Add:
- `titlebar-font` gsetting → `title_bar` FontOverride
- `text-scaling-factor` gsetting → `text_scaling_factor`
- `document-font-name` gsetting → (informational, not mapped currently)
- Portal accent color already handled

---

## 4. Connector Updates

Connectors (`native-theme-iced`, `native-theme-gpui`) should accept
`&ResolvedTheme` (or its sub-structs) instead of `&ThemeVariant`. This
eliminates all `Option` handling, `unwrap_or()` fallbacks, and fabricated
constants from connector code. Every value is guaranteed present.

---

## 5. Implementation Steps

### Step 1: Restructure ThemeVariant

Replace the flat ThemeColors/ThemeFonts/ThemeGeometry/WidgetMetrics
layout with the per-widget architecture from `todo_v0.4.2_theme-variant.md`.
Each widget gets its own struct with colors, font, sizing, and geometry.
Add `ThemeDefaults` for shared base properties. Update `impl_merge!` for
nested per-widget structs.

### Step 2: Create ResolvedTheme Module

Add `resolved.rs` with all Resolved* structs. Add
`ThemeResolutionError` to `error.rs`. Implement
`ThemeVariant::resolve()`.

Location: `native-theme/src/model/resolved.rs`,
`native-theme/src/error.rs`

### Step 3: Slim Down Platform Presets

Remove ⚙ fields (colors, fonts) from platform default TOMLs. Keep
only design constants (geometry, spacing, widget metrics, non-⚙
colors). See `todo_v0.5.1_inheritance-rules.md` §"What Platform
Default TOMLs Should Contain" for the field-by-field guide.

Note: `adwaita.toml` stays larger than others because GNOME exposes
few values via APIs — most Adwaita CSS colors are design constants.

Cross-platform presets (catppuccin, nord, etc.) keep all fields —
they have no OS reader.

Location: `native-theme/src/presets/*.toml`

### Step 4: Update OS Readers

Extend macOS, Windows, KDE, GNOME readers to populate all ⚙ fields
(see §3 above). Readers should return sparse ThemeVariants — only
fields they read from the OS. No embedded presets, no hardcoded
fallbacks.

### Step 5: Implement `from_system()` Pipeline

Change `from_system()` to run the full pipeline:
1. OS reader → sparse ThemeVariant
2. `resolve()` → fill derived Nones from inheritance rules
3. Load matching platform TOML → merge on top (design constants)
4. Return ThemeVariant ready for app TOML overlay + validate

Location: `native-theme/src/lib.rs`

### Step 6: Update Connectors

Change connectors to accept `&ResolvedTheme`. Remove all Option handling.

---

## 6. Testing

- **Unit**: `resolve()` fills accent-derived fields from provided accent
- **Unit**: `resolve()` fills font-inherited fields from provided base font
- **Unit**: `resolve()` skips fields that are already `Some`
- **Unit**: `resolve()` leaves field `None` if its source is also `None`
- **Unit**: `validate()` with all fields `Some` → `Ok(ResolvedTheme)`
- **Unit**: `validate()` with `None` fields → error listing each one
- **Unit**: TOML overlay after resolve() overrides resolved values
- **Unit**: Font inheritance — FontOverride with partial fields inherits
  from base correctly
- **Integration**: OS reader + resolve() + platform TOML → `validate()`
  succeeds on each platform
- **Integration**: Cross-platform preset + resolve() → `validate()`
  succeeds (preset provides all non-derived fields)
- **Integration**: App TOML overrides work — accent override in app
  TOML wins over OS-provided accent
- **Serde**: New ThemeFonts fields round-trip through TOML correctly
- **Serde**: ResolvedTheme does NOT implement Deserialize (output only)
