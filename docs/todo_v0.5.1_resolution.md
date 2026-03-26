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

## 1. Pipeline: OS Reader → TOML overlay → resolve()

### Current Behavior

`from_system()` returns the OS-detected theme **without** applying
inheritance or merging the platform's matching preset. On macOS, KDE,
and GNOME this means all spacing, geometry, and derived fields are
`None`.

### Required Behavior

The OS is the primary source. The TOML is an overlay that fills
design-constant gaps and lets app developers customize native values.

```
  OS Reader          Platform TOML     resolve()      App TOML     resolve()
  (live ⚙ values)    (design consts)   (inheritance)  (overrides)  (re-derive)
        │                  │                │              │             │
        └──── merge ───────┘                │              │             │
                  │                         │              │             │
                  ▼                         │              │             │
           ThemeVariant                     │              │             │
           (OS + design consts)             │              │             │
                  │                         │              │             │
                  └─────────────────────────┘              │             │
                                │                         │             │
                                ▼                         │             │
                         ThemeVariant                     │             │
                         (all derivable fields filled)    │             │
                                │                         │             │
                                └──── merge(app TOML) ────┘             │
                                                │                       │
                                                └───────────────────────┘
                                                          │
                                                       validate()
                                                          │
                                                          ▼
                                                    ResolvedTheme
```

Merge order: **OS + TOML form the base; `resolve()` fills inheritance on top.**
- OS reader provides live colors, fonts, DPI-scaled metrics (⚙ values)
- Platform default TOML fills design-constant gaps (geometry, spacing,
  widget metrics, non-⚙ colors like Adwaita CSS values on GNOME)
- `resolve()` fills derived fields from both OS and TOML sources
  (accent → primary_bg, defaults.font → menu.font,
  defaults.radius → button.radius, etc.)
- App TOML (optional) overrides any value the app developer wants
- A second `resolve()` pass after app TOML propagates any changed
  source fields (e.g. custom accent → primary_bg)
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

- The detected variant = OS reader + platform TOML + resolve() (complete, live)
- The other variant = platform TOML only (design constants, no live OS data)

For platforms where the TOML is minimal (KDE — few non-⚙ colors, no fonts in TOML),
the inactive variant will be incomplete. Options:
- Platform TOMLs include static colors/fonts for the inactive variant only
- The inactive variant is simply unavailable until the user switches
- (TBD: decide based on real-world usage patterns)

---

## 2. ResolvedTheme — Non-Optional Output

After the full pipeline (OS reader → TOML overlay → resolve → app TOML → resolve),
convert `ThemeVariant` (with `Option` fields) into a `ResolvedTheme`
(with direct values). The conversion validates that every required
field is `Some`. If any field is still `None`, return an error
listing all missing fields — not a panic.

```
  OS Reader ──────────▶ ThemeVariant (sparse)
                              │
  Platform TOML ──────▶ merge() — fills design constants
                              │
                       resolve() — fills derived Nones
                              │
  App TOML (opt) ─────▶ merge() — app overrides
                              │
                       resolve() — re-derive from overrides
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
    pub defaults: ResolvedDefaults,
    pub text_scale: ResolvedTextScale,

    pub window: ResolvedWindow,
    pub button: ResolvedButton,
    pub input: ResolvedInput,
    pub checkbox: ResolvedCheckbox,
    pub menu: ResolvedMenu,
    pub tooltip: ResolvedTooltip,
    pub scrollbar: ResolvedScrollbar,
    pub slider: ResolvedSlider,
    pub progress_bar: ResolvedProgressBar,
    pub tab: ResolvedTab,
    pub sidebar: ResolvedSidebar,
    pub toolbar: ResolvedToolbar,
    pub status_bar: ResolvedStatusBar,
    pub list: ResolvedList,
    pub popover: ResolvedPopover,
    pub splitter: ResolvedSplitter,
    pub separator: ResolvedSeparator,

    pub switch: ResolvedSwitch,
    pub dialog: ResolvedDialog,
    pub spinner: ResolvedSpinner,
    pub combo_box: ResolvedComboBox,
    pub segmented_control: ResolvedSegmentedControl,
    pub card: ResolvedCard,
    pub expander: ResolvedExpander,
    pub link: ResolvedLink,

    pub icon_set: String,
}
```

The ResolvedTheme mirrors the per-widget architecture from the
theme-variant spec. Each widget's Resolved struct contains plain values
(no `Option`) — all inheritance from `ThemeDefaults` has been applied.

See `todo_v0.5.1_theme-variant.md` for the full per-widget struct
definitions. Each `Option` field becomes a concrete value in the
resolved counterpart.

### Two-Phase Resolution

`resolve()` and `validate()` are separate steps:

- **`resolve()`** runs after the OS reader and platform TOML overlay.
  It fills `None` fields from inheritance sources (accent → primary_bg,
  font → menu.font, radius → button.radius, etc.). See
  `todo_v0.5.1_inheritance-rules.md` for the full inheritance table.
  Since both OS and TOML sources are available at this point, all
  derivable fields should be populated.

- **`validate()`** runs after all TOML overlays and resolve passes. It converts
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
  - defaults.spacing.xs
  - defaults.danger
  - tooltip.font.size
```

---

## 3. OS Reader Updates

### 3.1 macOS (`from_macos`)

Currently reads: `systemFontOfSize:`, `monospacedSystemFont...`,
~22 NSColors (base defaults + button + input + sidebar + status + focus ring).

Add:
- `NSFont.TextStyle.caption1` → `text_scale.caption` (10pt, 400, 13pt line)
- `NSFont.TextStyle.headline` → `text_scale.section_heading` (13pt, 700, 16pt line)
- `NSFont.TextStyle.title1` → `text_scale.dialog_title` (22pt, 400, 26pt line)
- `NSFont.TextStyle.largeTitle` → `text_scale.display` (26pt, 400, 32pt line)
- `+titleBarFontOfSize:` → `window.title_bar_font` FontSpec
- `+menuFontOfSize:` → `menu.font` FontSpec
- `+toolTipsFontOfSize:` → `tooltip.font` FontSpec
- Font weight from `NSFontDescriptor` traits
- `NSColor.placeholderTextColor` → `input.placeholder`
- `NSColor.windowFrameTextColor` → `window.title_bar_foreground`
- `NSColor.textInsertionPointColor` (macOS 14+) → `input.caret`
- Title bar background ≈ `controlBackgroundColor` (§2.2: ≈ `defaults.surface`)
- `NSColor.unemphasizedSelectedContentBackgroundColor` → `defaults.selection_inactive`
- `alternatingContentBackgroundColors[1]` → `list.alternate_row`
- `NSColor.headerTextColor` → `list.header_foreground`
- `NSColor.gridColor` → `list.grid_color`
- `NSScroller.preferredScrollerStyle` → `scrollbar.overlay_mode`
- Accessibility text size pref (macOS 14+) → `text_scaling_factor`
- `accessibilityDisplayShouldReduceMotion` → `reduce_motion`
- `accessibilityDisplayShouldIncreaseContrast` → `high_contrast`
- `accessibilityDisplayShouldReduceTransparency` → `reduce_transparency`

### 3.2 Windows (`from_windows`)

Currently reads: `lfMessageFont`, UISettings colors, geometry via
`GetSystemMetricsForDpi`, `winui3_spacing()`.

Add:
- `lfCaptionFont` → `window.title_bar_font` FontSpec (family, size, weight)
- `lfMenuFont` → `menu.font` FontSpec
- `lfStatusFont` → `status_bar.font` FontSpec
- `lfMessageFont` weight → `defaults.font.weight`
- `DwmGetColorizationColor` → `window.title_bar_background`
- `COLOR_CAPTIONTEXT` → `window.title_bar_foreground`
- `COLOR_INACTIVECAPTION` → `window.inactive_title_bar_background`
- `COLOR_INACTIVECAPTIONTEXT` → `window.inactive_title_bar_foreground`
- SM_CXFOCUSBORDER / SM_CYFOCUSBORDER → `focus_ring_width`
- `GetSysColor` widget colors (⚙ values the OS reader must provide —
  the TOML should NOT include these):
  - `COLOR_BTNFACE` → `button.background`
  - `COLOR_BTNTEXT` → `button.foreground`
  - `COLOR_MENU` → `menu.background`
  - `COLOR_MENUTEXT` → `menu.foreground`
  - `COLOR_INFOBK` → `tooltip.background`
  - `COLOR_INFOTEXT` → `tooltip.foreground`
  - `COLOR_WINDOW` → `input.background`
  - `COLOR_WINDOWTEXT` → `input.foreground`
  - `COLOR_HIGHLIGHT` → `defaults.selection`
  - `COLOR_HIGHLIGHTTEXT` → `defaults.selection_foreground`
- `UISettings.TextScaleFactor` → `text_scaling_factor`
- `SPI_GETCLIENTAREAANIMATION` → `reduce_motion`
- `SPI_GETHIGHCONTRAST` → `high_contrast`
- ↕ `SM_CXSMICON` → `defaults.icon_sizes.small`
- ↕ `SM_CXICON` → `defaults.icon_sizes.large`

### 3.3 KDE (`from_kde`)

Currently reads: `font`, `fixed` from [General], colors from Colors:*
sections.

Add:
- `smallestReadableFont` → `text_scale.caption` (parse field 1 for size)
- `text_scale.section_heading`: compute `size = font.size × 1.20`
  (Kirigami Heading Level 2)
- `text_scale.dialog_title`: compute `size = font.size × 1.35`
  (Kirigami Heading Level 1)
- `toolBarFont` → `toolbar.font` FontSpec
- `menuFont` → `menu.font` FontSpec
- `activeFont` → `window.title_bar_font` FontSpec
- Qt font field 4 → `weight` for all font keys
- `[WM] activeBackground` → `window.title_bar_background`
- `[WM] activeForeground` → `window.title_bar_foreground`
- `[WM] inactiveBackground` → `window.inactive_title_bar_background`
- `[WM] inactiveForeground` → `window.inactive_title_bar_foreground`
- `[WM]` decoration theme colors → `window.border`
- `[Colors:View] ForegroundInactive` → `input.placeholder`
- `[Colors:View] BackgroundAlternate` → `list.alternate_row`
- `[Colors:View] ForegroundVisited` → `link.visited`
- `[Colors:Header] BackgroundNormal` → `list.header_background` (KF 5.71+)
- `[Colors:Header] ForegroundNormal` → `list.header_foreground` (KF 5.71+)
- `[Colors:Complementary] BackgroundNormal` → `sidebar.background`
- `[Colors:Complementary] ForegroundNormal` → `sidebar.foreground`
- `[Icons] Theme` from kdeglobals → `icon_set` (§1.3.6; default: `breeze`)
- Icon sizes from icon theme's `index.theme`: `MainToolbar`, `Small`,
  `Desktop`, `Dialog`, `Panel` → `defaults.icon_sizes` (§2.1.8)
- `forceFontDPI` / 96 → `text_scaling_factor`
- `AnimationDurationFactor` = 0 → `reduce_motion`

### 3.4 GNOME

Currently uses: bundled Adwaita preset + portal accent overlay.

**Architecture change:** The GNOME reader should NOT embed the Adwaita
preset internally. It should only read OS values. Adwaita CSS colors
(non-⚙ design constants) belong in the `adwaita.toml` platform TOML,
which overlays after the reader. This makes `adwaita.toml` larger
than other platform TOMLs (it has all CSS-derived colors), which is
correct — GNOME exposes fewer values via APIs than KDE or macOS.

Add:
- `font-name` gsetting → `defaults.font` FontSpec (family, size, weight)
- `monospace-font-name` gsetting → `defaults.mono_font` FontSpec
- `titlebar-font` gsetting → `window.title_bar_font` FontSpec
- `text_scale.caption`: compute `size = font.size × 0.82`
  (libadwaita `.caption` = 82%; weight 400 = default, inherits)
- `text_scale.dialog_title`: compute `size = font.size × 1.36`
  (libadwaita `.title-2` = 136%; weight 800 → `adwaita.toml`)
- `text_scale.display`: compute `size = font.size × 1.81`
  (libadwaita `.title-1` = 181%; weight 800 → `adwaita.toml`)
- Note: `text_scale.section_heading` needs no OS reader computation —
  `.heading` uses inherited (= base) font size, so `size ← defaults.font.size`
  via resolve(). Only `weight = 700` is needed (→ `adwaita.toml`).
- `text-scaling-factor` gsetting → `text_scaling_factor`
- `document-font-name` gsetting → (informational, not mapped currently)
- `overlay-scrolling` / `gtk-overlay-scrolling` gsetting → `scrollbar.overlay_mode`
- `enable-animations` gsetting / Portal `reduced-motion` → `reduce_motion`
- `a11y.interface high-contrast` / Portal `contrast` → `high_contrast`
- `icon-theme` gsetting → `icon_set` (§1.4.6; default: `Adwaita`)
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
layout with the per-widget architecture from `todo_v0.5.1_theme-variant.md`.
Each widget gets its own struct with colors, font, sizing, and geometry.
Add `ThemeDefaults` for shared base properties. Update `impl_merge!` for
nested per-widget structs.

### Step 2: Create ResolvedTheme Module

Add `resolved.rs` with all Resolved* structs. Add
`ThemeResolutionError` to `error.rs`. Implement
`ThemeVariant::resolve()`.

Location: `native-theme/src/model/resolved.rs`,
`native-theme/src/error.rs`

### Step 3: Update OS Readers

Extend macOS, Windows, KDE, GNOME readers to populate all ⚙ fields
(see §3 above). Readers should return sparse ThemeVariants — only
fields they read from the OS. No embedded presets, no hardcoded
fallbacks.

### Step 4: Slim Down Platform Presets

Remove ⚙ fields (colors, fonts) from platform default TOMLs. Keep
only design constants (geometry, spacing, widget metrics, non-⚙
colors). See `todo_v0.5.1_inheritance-rules.md` §"What Platform
Default TOMLs Should Contain" for the field-by-field guide.

**Depends on Step 3:** ⚙ values can only be removed from TOMLs after
the OS readers provide them. Removing TOML values before OS readers
supply replacements would leave gaps that neither `resolve()` nor the
TOML can fill.

Note: `adwaita.toml` stays larger than others because GNOME exposes
few values via APIs — most Adwaita CSS colors are design constants.

Cross-platform presets (catppuccin, nord, etc.) keep all fields —
they have no OS reader.

Location: `native-theme/src/presets/*.toml`

### Step 5: Implement `from_system()` Pipeline

Change `from_system()` to run the full pipeline:
1. OS reader → sparse ThemeVariant
2. Load matching platform TOML → merge on top (design constants)
3. `resolve()` → fill derived Nones from inheritance rules
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
- **Unit**: App TOML overlay after resolve() overrides resolved values
- **Unit**: Font inheritance — FontSpec with partial fields inherits
  from base correctly
- **Unit**: TextScaleEntry inheritance — `size` ← `defaults.font.size`,
  `weight` ← `defaults.font.weight`, `line_height` ← computed from
  `defaults.line_height` multiplier × resolved size
- **Integration**: OS reader + platform TOML + resolve() → `validate()`
  succeeds on each platform
- **Integration**: Cross-platform preset + resolve() → `validate()`
  succeeds (preset provides all non-derived fields)
- **Integration**: App TOML overrides work — accent override in app
  TOML wins over OS-provided accent
- **Serde**: Per-widget FontSpec fields round-trip through TOML correctly
- **Serde**: ResolvedTheme does NOT implement Deserialize (output only)
