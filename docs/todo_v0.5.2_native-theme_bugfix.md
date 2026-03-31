# native-theme v0.5.2 bugfix todo

Cross-referenced against `docs/platform-facts.md`. Each chapter is one
issue. Within each chapter every solution option is listed with full
pro/contra, followed by the recommended choice and the reasoning behind
it.

## Architecture: full preset vs live preset vs OS reader

Understanding this architecture is essential for interpreting the
issues below. The OS-first pipeline in `run_pipeline()` (lib.rs:611)
merges three layers:

```
full preset  <-  live preset  <-  OS reader output
```

| Layer | Example | What it contains | When it's used |
|-------|---------|-----------------|----------------|
| **Full preset** | `kde-breeze.toml` | Everything: colors, fonts, icons, geometry, metrics | Standalone use; also provides the inactive variant's fallback colors in the OS-first pipeline |
| **Live preset** | `kde-breeze-live.toml` | Geometry and metrics ONLY (no colors, fonts, icons) | OS-first pipeline: overrides full preset's geometry |
| **OS reader** | `from_kde()` | Live OS data: colors, fonts, icons, accessibility | OS-first pipeline: overlays on top of both presets |

**What goes where:**
- **Live preset**: design constants the OS does NOT expose via API --
  widget sizing, spacing, border radii, focus ring geometry, line_height.
  Annotated `(measured)`, `(Breeze src)`, `(Adwaita CSS)`, `(Fluent)`,
  `(preset)`, or `(none)` in platform-facts.
- **OS reader**: live values the OS DOES expose -- colors (`⚙`),
  fonts (`⚙`), icon theme, DPI/scaling, accessibility prefs.
- **Full preset**: everything from both above, plus static fallback
  colors/fonts for standalone use and the inactive variant.

**Consequence for this todo:** Geometry/metric bugs must be fixed in
BOTH the full preset AND its corresponding live preset (they have the
same wrong values). Reader bugs only affect the OS reader code.

## Verification status (2026-03-31)

Every issue was re-verified against source code, cross-checked against
the v0.5.0 design docs in `docs/archive/`, and re-evaluated against
the full/live/reader architecture.

### KDE Breeze preset + reader issues

| Ch | Issue | Fix target | Status |
|----|-------|-----------|--------|
| 1  | radius 4 -> 5 | `kde-breeze.toml` + `kde-breeze-live.toml` | **CONFIRMED** |
| 2  | focus_ring swap | `kde-breeze.toml` + `kde-breeze-live.toml` | **CONFIRMED** |
| 3  | line_height 1.4 -> 1.36 | `kde-breeze.toml` + `kde-breeze-live.toml` | **CONFIRMED** |
| 4  | icon_sizes wrong (4 of 5) | `kde-breeze.toml` + `kde-breeze-live.toml` | **CONFIRMED** |
| 5  | progress_bar.min_width mismap | `kde-breeze.toml` + `kde-breeze-live.toml` + `metrics.rs` | **CONFIRMED** |
| 6  | icon_set/icon_theme confusion | readers | **DOWNGRADED** (doc/naming inconsistency) |
| 7  | forceFontDPI wrong file | `kde/mod.rs` reader | **CONFIRMED** |
| 8  | accent_foreground missing | `kde/colors.rs` reader | **CONFIRMED** |
| 9  | KDE reader missing geometry | N/A | **RESOLVED BY ARCHITECTURE** -- `run_pipeline` merges live preset; only affects standalone `from_kde()` |
| 10 | KDE reader missing spacing | N/A | **RESOLVED BY ARCHITECTURE** |
| 11 | KDE reader missing ComboBox | N/A | **RESOLVED BY ARCHITECTURE** |
| 12 | KDE reader missing Switch | N/A | **RESOLVED BY ARCHITECTURE** |
| 13 | KDE reader missing Spinner | N/A | **RESOLVED BY ARCHITECTURE** |
| 14 | KDE reader missing Expander | N/A | **RESOLVED BY ARCHITECTURE** |
| 15 | KDE reader missing line_height | N/A | **RESOLVED BY ARCHITECTURE** |
| 39 | `defaults.border` = DecorationFocus | `kde/colors.rs` reader | **CONFIRMED** -- reader overwrites full preset's correct border with accent color |
| 40 | `list.background`/`foreground` not set | `kde/colors.rs` reader | **CONFIRMED** -- full preset provides static Breeze colors, but custom KDE color schemes lose their live Colors:View |

Note: the wrong VALUES flagged in ch 12-14 (switch/spinner/expander)
are still real bugs -- they affect the live preset values, covered in
the "additional kde-breeze preset data bugs" table below.

### Additional kde-breeze preset data bugs

All of these affect both `kde-breeze.toml` AND `kde-breeze-live.toml`.

| Preset field | Current | Platform-facts | Source |
|---|---|---|---|
| spinner.diameter | 24 | **36** | QQC2 BusyIndicator: `gridUnit*2` = 36px at default |
| expander.arrow_size | 12 | **10** | breezemetrics.h `ItemView_ArrowSize = 10` |
| switch.track_width | 40 | **~36** | QQC2 font-derived: `height*2` ~36px at default |
| switch.track_height | 20 | **~18** | QQC2 font-derived: `fontMetrics.height` ~18px |
| switch.thumb_size | 16 | **~18** | QQC2: `= fontMetrics.height` ~18px |

### GNOME reader + preset issues

| Ch | Issue | Fix target | Status |
|----|-------|-----------|--------|
| 16 | missing portal reduced-motion | `gnome/mod.rs` reader | **CONFIRMED** |
| 17 | missing gsettings high-contrast | `gnome/mod.rs` reader | **CONFIRMED** |
| 18 | section_heading weight=400 | `adwaita.toml` + `adwaita-live.toml` (add entry) | **CONFIRMED** |

### Adwaita preset data bugs

All of these affect both `adwaita.toml` AND `adwaita-live.toml`.

| Ch | Field | Preset | Platform-facts | Source |
|----|-------|--------|----------------|--------|
| 19 | `radius` | 12.0 | **9** | §1.4.3 `$button_radius` (12 is `$card_radius`) |
| 20 | `radius_lg` | 14.0 | **15** | §1.4.3 `$button_radius + 6` |
| 21 | `line_height` | 1.4 | **1.21** | §2.1.1 Adwaita Sans sTypo `(1984+494)/2048` |
| 22 | `focus_ring_offset` | 1.0 | **-2** | §1.4.3 `outline-offset: -$width` (inset) |
| 23 | `slider.track_height` | 6 | **10** | §1.4.4 Scale trough min-height |
| 24 | `progress_bar.height` | 6 | **8** | §1.4.4 ProgressBar bar height |
| 25 | `progress_bar.min_width` | 100 | **80** | §2.10 Adwaita CSS |
| 26 | `menu.item_height` | 34 | **32** | §1.4.4 modelbutton min-height |
| 27 | `menu.padding_horizontal` | 8 | **12** | §1.4.4 `$menu_padding = 12` |
| 28 | `tooltip.padding_horizontal` | 6 | **10** | §1.4.4 tooltip 6px vert / 10px horiz |
| 29 | `button.padding_horizontal` | 12 | **10** | §1.4.4 button CSS `padding: 5px 10px` |
| 30 | `button.padding_vertical` | 8 | **5** | §1.4.4 button CSS `padding: 5px 10px` |
| 31 | `input.padding_horizontal` | 12 | **9** | §1.4.4 entry `padding-left/right: 9px` |
| 32 | `input.padding_vertical` | 8 | **0** | §2.4 GNOME: "0 (vertical space from min-height)" |
| 33 | `splitter.width` | 6 | **1** | §2.17 GNOME GtkPaned default = 1 (wide = 5) |
| 34 | `toolbar.height` | 46 | **47** | §1.4.4 headerbar min-height |
| 35 | `spinner.diameter` | 24 | **16** | §1.4.4 GtkSpinner `DEFAULT_SIZE = 16` |
| 36 | `switch` (3 values) | 40/20/16 | **~46/~26/20** | §1.4.4 / §2.21 derived metrics |
| 37 | `icon_sizes.toolbar` | 24 | **16** | §1.4.6 / §2.1.8 `GTK_ICON_SIZE_NORMAL` |
| 38 | `tab.min_height` | 34 | **30** | §1.4.4 Notebook tab min-height |

### resolve.rs inheritance

| Ch | Issue | Fix target | Status |
|----|-------|-----------|--------|
| 41 | `list.alternate_row` fallback uses `d.background` | `resolve.rs` | **CONFIRMED** |

### Cross-cutting preset errors

All of these affect both `*.toml` AND `*-live.toml` for all 4 platforms.

| Ch | Issue | Affected presets |
|----|-------|-----------------|
| 42 | `line_height = 1.4` in all presets | All 4 platform presets + live + community |
| 43 | `focus_ring_offset` wrong in all presets | All 4 platform presets + live |

### macOS Sonoma preset data bugs

All of these affect both `macos-sonoma.toml` AND `macos-sonoma-live.toml`.

| Ch | Field | Preset | Platform-facts | Source |
|----|-------|--------|----------------|--------|
| 44 | `radius` | 6.0 | **5** | §1.1.3 control corner radius |
| 45 | `disabled_opacity` | 0.5 | **~0.3** | §1.1.3 disabledControlTextColor alpha |
| 46 | `focus_ring_width` | 2.0 | **3** | §1.1.3 / §2.1.5 |
| 47 | button (3 values) | 12/4/6 | **8/3/4** | §1.1.4 / §2.3 |
| 48 | `input.padding_vertical` | 4 | **3** | §2.4 measured (22-16)/2 |
| 49 | menu (2 values) | 4/8 | **3/4** | §2.6 |
| 50 | slider (2 values) | 4/4 | **5/8** | §1.1.4 / §2.9 |
| 51 | scrollbar (2 values) | 15/30 | **16/40** | §1.1.3 / §2.8 |
| 52 | `toolbar.padding` | 4 | **8** | §2.13 |
| 53 | `list.padding_vertical` | 2 | **4** | §2.15 measured (24-16)/2 |
| 54 | `splitter.width` | 9 | **6** | §1.1.4 NSSplitView thick divider |
| 55 | switch (3 values) | 40/20/16 | **38/22/18** | §1.1.4 / §2.21 |
| 56 | spinner (2 values) | 24/16 | **32/10** | §2.23 |
| 57 | combo_box (3 values) | 22/12/12 | **21/~9/~17** | §1.1.4 / §2.24 |

### Windows 11 preset data bugs

All of these affect both `windows-11.toml` AND `windows-11-live.toml`.

| Ch | Field | Preset | Platform-facts | Source |
|----|-------|--------|----------------|--------|
| 58 | `disabled_opacity` | 0.4 | **~0.3** | §1.2.3 Fluent per-control |
| 59 | button (2 values) | 32/12 | **~27/11** | §1.2.4 / §2.3 |
| 60 | input padding (asymmetric) | 12/6 | **10L,6R / 5** | §1.2.4 model limitation |
| 61 | `slider.thumb_size` | 22 | **18** | §1.2.4 |
| 62 | `progress_bar.height` | 4 | **1** track / **3** min | §1.2.4 |
| 63 | menu (2 values) | 32/4 | **~23-31/8** | §1.2.4 |
| 64 | scrollbar (2 values) | 16/24 | **17/17** | §1.2.3 |
| 65 | toolbar (2 values) | 48/4 | **64/0** | §1.2.4 |
| 66 | `list.item_height` | 36 | **40** | §1.2.4 |
| 67 | `switch.thumb_size` | 16 | **12** rest | §1.2.4 |
| 68 | `combo_box.arrow_area_width` | 28 | **38** | §1.2.4 |
| 69 | `icon_sizes.toolbar` | 24 | **20** | §1.2.6 / §2.1.8 |
| 70 | `splitter.width` | 4 | **1** | §2.17 Fluent SplitView |
| 71 | Model limitation: asymmetric padding | N/A | N/A | §1.2.4 multiple widgets |

---

## 1. KDE Breeze preset `radius` is 4.0 -- should be 5.0

**Files:** `native-theme/src/presets/kde-breeze.toml` lines 36, 234
AND `native-theme/src/presets/kde-breeze-live.toml` lines 8, 142

**What:** The preset sets `radius = 4.0` for both light and dark. Per
platform-facts section 1.3.3 the Breeze constant `Frame_FrameRadius` is
5px (verified against `breezemetrics.h`).

### Options

**A. Change the preset value to 5.0**

- Pro: Matches the authoritative Breeze source (`breezemetrics.h`).
- Pro: Trivial one-line fix per variant.
- Con: May alter the visual appearance for existing users who loaded the
  preset and relied on 4.0. However, that appearance was already wrong.

**B. Leave as-is and document the deviation**

- Pro: No risk of visual change for existing users.
- Con: The preset continues to be factually incorrect versus the source
  it claims to represent.
- Con: Any future audits will keep flagging this.

### Recommendation

**Option A.** The preset's purpose is to represent Breeze faithfully.
A wrong radius is simply a data bug. The value 4.0 has no cited
justification; it likely originated from confusing Windows Fluent
`ControlCornerRadius = 4` with KDE.

---

## 2. KDE Breeze preset `focus_ring_width` and `focus_ring_offset` are swapped

**Files:** `native-theme/src/presets/kde-breeze.toml` lines 43-44, 241-242
AND `native-theme/src/presets/kde-breeze-live.toml` lines 15-16, 149-150

**What:** The preset has `focus_ring_width = 2.0` and
`focus_ring_offset = 1.0`. Platform-facts section 2.1.5 shows KDE Breeze
uses a 1.001px stroke (the ring width) with a 2px outset margin (the
ring offset). The two values are transposed.

### Options

**A. Swap the values: width = 1.0, offset = 2.0**

- Pro: Matches `PenWidth::Frame` (1.001) for the stroke and
  `PM_FocusFrameHMargin` (2) for the outset in `breezemetrics.h`.
- Pro: One-line swap per variant.
- Con: Visually changes focus ring rendering for preset users.

**B. Use width = 1.001, offset = 2.0 (exact Breeze value)**

- Pro: Perfectly matches the source constant.
- Con: 1.001 is a Breeze anti-aliasing trick (avoids sub-pixel seams);
  GUI frameworks that render the focus ring are unlikely to benefit from
  the 0.001 extra. Clean 1.0 is equivalent in practice.

**C. Leave as-is**

- Pro: No visual change.
- Con: Both values are wrong -- a thicker-than-native ring drawn too
  close to the element.

### Recommendation

**Option A** (with 1.0, not 1.001). The 0.001 hack is internal to
Breeze's QPainter rendering and irrelevant when the consumer paints its
own focus ring. Clean 1.0 is the correct portable representation.

---

## 3. KDE Breeze preset `line_height` is 1.4 -- should be 1.36

**Files:** `native-theme/src/presets/kde-breeze.toml` lines 42, 240
AND `native-theme/src/presets/kde-breeze-live.toml` lines 14, 148

**What:** The preset uses `line_height = 1.4`. Platform-facts section
2.1.1 documents Noto Sans sTypo metrics yielding
`(1069 + 293 + 0) / 1000 = 1.362`, rounded to 1.36.

### Options

**A. Change to 1.36**

- Pro: Matches the font-metric-derived value from the platform-facts
  research.
- Pro: 1.36 produces tighter, more authentic KDE line spacing.
- Con: Slightly tighter text for existing users.

**B. Change to 1.362 (full precision)**

- Pro: Exact font metrics.
- Con: False precision: line_height is multiplied by font size to
  produce a pixel value, and sub-hundredth precision vanishes after
  rounding. The format elsewhere uses two decimal places.

**C. Leave as 1.4**

- Pro: Familiar, round number.
- Con: Factually wrong. 1.4 is 3% larger than the real metrics, which
  at 10pt produces 14px instead of 13.6px -- a full extra pixel of
  leading at common sizes.

### Recommendation

**Option A (1.36).** Two decimal places is the convention used for all
other platforms in the platform-facts document. 1.362 gains nothing
practical. The 1.4 value has no cited source.

---

## 4. KDE Breeze preset `icon_sizes` are mostly wrong

**Files:** `native-theme/src/presets/kde-breeze.toml` lines 69-74, 267-272
AND `native-theme/src/presets/kde-breeze-live.toml` lines 27-32, 161-166

**What:** The preset values vs. platform-facts section 1.3.6 / 2.1.8
(Breeze `index.theme` defaults and `kicontheme.cpp` fallbacks):

| Field     | Preset | Platform-facts | Source                        |
|-----------|--------|----------------|-------------------------------|
| `toolbar` | 24.0   | **22.0**       | `ToolbarDefault` in index.theme |
| `small`   | 16.0   | 16.0           | `SmallDefault` -- correct     |
| `large`   | 32.0   | **48.0**       | `DesktopDefault` (Breeze overrides C++ fallback of 32) |
| `dialog`  | 22.0   | **32.0**       | `DialogDefault`               |
| `panel`   | 20.0   | **48.0**       | `PanelDefault` (Breeze matches C++ fallback of 48) |

Four out of five values are wrong.

### Options

**A. Fix all four values to match Breeze index.theme**

- Pro: Accurately represents the Breeze icon theme as installed.
- Pro: The KDE reader already parses `index.theme` at runtime and
  returns the live sizes. The preset should match what the reader would
  produce on a stock Breeze installation.
- Con: `large = 48` and `panel = 48` are large. But that is what Breeze
  actually configures.

**B. Keep the current values as "practical" overrides**

- Pro: 32px desktop icons might look better in non-KDE-native apps.
- Con: The preset claims to be "sourced from KDE/breeze repository" but
  doesn't reflect actual Breeze defaults. Consumers who trust the preset
  get wrong icon lookup sizes.

**C. Match the C++ fallbacks instead of index.theme**

- Pro: The C++ fallbacks (32/22/16/48/32) are more conservative and
  historically stable.
- Con: Breeze overrides several of these intentionally; using C++
  fallbacks misrepresents the actual Breeze experience.

### Recommendation

**Option A.** The preset's name and header comment say "sourced from
KDE/breeze repository". The Breeze `index.theme` is the authoritative
source for icon sizes when Breeze is installed. All four wrong values
should be corrected: `toolbar = 22.0`, `large = 48.0`, `dialog = 32.0`,
`panel = 48.0`.

---

## 5. `progress_bar.min_width` incorrectly mapped from `ProgressBar_BusyIndicatorSize`

**Files:** `native-theme/src/kde/metrics.rs` line 33,
`native-theme/src/presets/kde-breeze.toml` lines 117, 315,
AND `native-theme/src/presets/kde-breeze-live.toml` lines 69, 203

**What:** Both the KDE reader and the preset set
`progress_bar.min_width = 14.0`, citing `ProgressBar_BusyIndicatorSize`.
But this Breeze constant is the width of the moving segment in an
*indeterminate* progress bar animation, not the minimum width of the
progress bar widget. Platform-facts section 2.10 explicitly states KDE
progress bar `min_width` is "(none) -- no minimum".

### Options

**A. Remove the mapping entirely (set to None)**

- Pro: Matches platform-facts: KDE has no minimum progress bar width.
- Pro: Consumers who size their progress bar to content or container get
  native behavior.
- Con: Loses the 14px value from the data model. But this value has no
  correct target field to live in.

**B. Add a `busy_indicator_size` field to `ProgressBarTheme`**

- Pro: Preserves the Breeze constant with correct semantics.
- Con: Only Breeze uses this concept. macOS uses fin-based spinners,
  Windows track height is 1px (different concept), GNOME has no
  equivalent. A field used by one platform out of four is dubious.
- Con: Increases model surface area for a rarely needed value.

**C. Map to `spinner.min_size` instead**

- Pro: Conceptually closer -- busy indicator is a form of spinner.
- Con: Semantically wrong. A progress bar busy segment is not a spinner
  widget; they are drawn differently and in different contexts.

**D. Leave as-is and document the imprecision**

- Pro: No code change.
- Con: Actively incorrect. A consumer reading `min_width = 14` would
  constrain their progress bar to 14px minimum, which KDE does not do.

### Recommendation

**Option A.** Remove the mapping from `metrics.rs` and remove
`min_width = 14.0` from both variants in `kde-breeze.toml`. The value
does not belong in `min_width`. If a `busy_indicator_size` field is ever
needed it can be added later, but no consumer currently needs it and no
other platform provides an equivalent.

---

## 6. Readers store icon theme name in `icon_set` instead of `icon_theme`

**Status: DOWNGRADED to doc/naming inconsistency (not a functional bug)**

**Files:** `native-theme/src/gnome/mod.rs` line 216-217,
`native-theme/src/kde/mod.rs` line 37-40

**What:** `ThemeVariant` has two fields:
- `icon_set`: doc comment says "naming convention" (`"freedesktop"`,
  etc.) -- `IconSet::from_name()` parses these.
- `icon_theme`: doc comment says "visual icon theme name" (`"breeze"`,
  `"Adwaita"`).

Both readers store the OS icon theme name in `icon_set`:
- GNOME: `variant.icon_set = Some(icon_theme)` where `icon_theme` is
  from `gsettings icon-theme` (returns e.g. `"Adwaita"`).
- KDE: `variant.icon_set = Some(theme_name)` where `theme_name` is
  from `kdeglobals [Icons] Theme` (returns e.g. `"breeze"`).

**Why downgraded:** The v0.5.0 design doc (`v0.5.0_resolution.md:295,330`)
explicitly maps `[Icons] Theme -> icon_set` and `icon-theme gsetting ->
icon_set`. The `icon_theme` field did not exist when the design doc was
written. Additionally, `resolve.rs:2481-2514` has a test that validates
`icon_set = "Adwaita"` as the expected value. The behavior is not a
regression -- it is the documented, tested design.

**Why still an inconsistency:**
- Presets set `icon_set = "freedesktop"` (naming convention) and
  `icon_theme = "Adwaita"` (visual theme). The reader overwrites
  `icon_set` with the visual name, destroying the naming convention.
- `IconSet::from_name("Adwaita")` returns `None`. Consumers must fall
  back to `system_icon_set()` (which `resolve()` Phase 5 no longer
  provides, since `icon_set` is already `Some`).
- After GNOME merge: `icon_set = "Adwaita"` (wrong semantics),
  `icon_theme = "Adwaita"` (correct, from preset). Two fields with
  the same value but different intended meanings.

### Options

**A. Store in `icon_theme` instead; leave `icon_set` to preset/resolve fallback**

- Pro: Matches the current doc comments and preset semantics.
- Pro: `IconSet::from_name()` on `icon_set` works correctly.
- Con: Breaks the existing test at resolve.rs:2513 (must be updated).
- Con: Contradicts the v0.5.0 design doc mapping.

**B. Store in both: `icon_theme = <name>`, `icon_set = "freedesktop"`**

- Pro: Explicit; no fallback needed.
- Con: Hardcodes naming convention.

**C. Leave as-is; update doc comments to match reality**

- Pro: No behavior change; no test changes.
- Con: Presets still set `icon_set = "freedesktop"` which the reader
  immediately overwrites. Redundant preset field.
- Con: `IconSet::from_name()` on the resolved value returns None.

**D. Rename the field to match its actual usage**

- Pro: Eliminates the semantic confusion.
- Con: Breaking API change.

### Recommendation

**Option A** is the cleanest long-term fix but is lower priority than
the confirmed bugs (chapters 1-5, 7-8). The test must be updated, and
the design doc mapping acknowledged as outdated. If chapter 9 (preset
merge for KDE) is implemented, the icon_set/icon_theme split becomes
more important because the preset's `icon_set = "freedesktop"` would
serve as base and the reader should only overlay `icon_theme`.

For now: **defer** until the design doc / field semantics are clarified
as part of a broader icon system review.

---

## 7. KDE reader reads `forceFontDPI` from wrong config file

**File:** `native-theme/src/kde/mod.rs` lines 128-133

**What:** `populate_accessibility()` reads `forceFontDPI` from the
kdeglobals INI object (`ini.get("General", "forceFontDPI")`). Per
platform-facts section 1.3.7, this key lives in
`~/.config/kcmfontsrc` (under `[General]`), not in kdeglobals.

The kdeglobals INI is the only file the KDE reader loads, so this key
is never found unless the user manually copies it to kdeglobals.

### Options

**A. Read `kcmfontsrc` as a second INI file**

- Pro: Reads from the documented, authoritative source.
- Pro: kcmfontsrc is a plain INI file in the same XDG config directory;
  parsing it is identical to parsing kdeglobals.
- Con: Adds a second file read and a second parser instance.

**B. Merge kcmfontsrc into the existing INI before processing**

- Pro: Single parser, single pass through populate_accessibility.
- Con: Key collisions are possible if both files have a `[General]`
  section (they do). The kcmfontsrc `[General]` section has different
  keys but merging two INI files is fragile.

**C. Read only the specific key from kcmfontsrc with a targeted helper**

- Pro: Minimal scope -- opens the file, reads one key, closes it.
- Pro: No risk of INI section collision.
- Con: Introduces a pattern different from the rest of the reader (which
  parses the full INI up front).

**D. Leave as-is and document the limitation**

- Pro: No code change.
- Con: `text_scaling_factor` is never set from the correct source for
  most KDE users. The feature is silently broken.

### Recommendation

**Option C.** A small helper like `read_kcmfontsrc_key("General",
"forceFontDPI")` that opens `$XDG_CONFIG_HOME/kcmfontsrc` (or
`~/.config/kcmfontsrc`) and reads one value keeps the change contained.
This avoids INI merging issues and matches how the GNOME reader uses
targeted `gsettings get` calls for individual keys.

Option A is viable but overkill for a single key. Option B risks
collision in the `[General]` section.

---

## 8. KDE reader does not set `accent_foreground`

**File:** `native-theme/src/kde/colors.rs`

**What:** Platform-facts section 2.1.3 maps KDE `accent_foreground` to
`[Colors:Selection] ForegroundNormal`. The KDE reader sets
`selection_foreground` from this exact key but does not also set
`accent_foreground`:

```rust
variant.defaults.selection_foreground = get_color(ini, "Colors:Selection", "ForegroundNormal");
// accent_foreground is never set
```

### Options

**A. Add a one-line assignment for `accent_foreground`**

- Pro: Trivial fix. The value is already parsed (same source as
  `selection_foreground`).
- Pro: Matches platform-facts mapping exactly.
- Con: None.

**B. Set it from a different source (e.g. Button/ForegroundNormal)**

- Pro: Could be argued that "foreground on accent" is closer to button
  foreground.
- Con: Not what the platform-facts research says. On KDE the Selection
  group is the authoritative source for text-on-accent-colored
  backgrounds.

**C. Leave unset and rely on resolution fallback**

- Pro: If the theme resolver already derives `accent_foreground` from
  `selection_foreground`, this is a non-issue.
- Con: Looking at `resolve.rs`, there's no guarantee the resolver does
  this derivation. Missing `accent_foreground` means consumers get None
  for text on primary buttons, accent badges, etc.

### Recommendation

**Option A.** Add one line:
```rust
variant.defaults.accent_foreground = get_color(ini, "Colors:Selection", "ForegroundNormal");
```

There is no downside. The data is already available.

---

## 9-15. KDE reader missing geometry/spacing/metrics

**Status: RESOLVED BY ARCHITECTURE**

**What was originally flagged:** The KDE reader (`from_kde()`) does not
set global geometry defaults (radius, frame_width, etc.), spacing scale,
or several widget metrics (ComboBox, Switch, Spinner, Expander,
line_height).

**Why this is not a bug in the deployed pipeline:** The OS-first
pipeline `run_pipeline()` (lib.rs:611) merges:

```
full preset <- live preset <- reader output
```

The `kde-breeze-live.toml` provides all geometry, spacing, and widget
metrics. The reader only needs to provide live OS data (colors, fonts,
icons, accessibility). This is the correct architecture per the comment
at the top of each live preset: *"geometry/metrics only -- used by the
OS-first pipeline as merge base; OS reader provides colors/fonts."*

**When this still matters:** Only when `from_kde()` is called directly
(standalone use without `run_pipeline()`). The returned ThemeSpec has
no geometry/spacing/metrics and will fail `validate()`.

**The real bugs:** The live preset VALUES are wrong (e.g., radius=4
instead of 5, spinner.diameter=24 instead of 36). These are tracked in
chapters 1-5 and the "Additional kde-breeze preset data bugs" table.
Each fix must be applied to BOTH `kde-breeze.toml` AND
`kde-breeze-live.toml`.

The original chapters 9-15 detailed analysis is preserved below for
reference but the recommendation (have from_kde merge with preset) is
superseded by the existing `run_pipeline()` architecture.

<details>
<summary>Original chapter 9 analysis (superseded)</summary>

**File:** `native-theme/src/kde/metrics.rs` (and `kde/mod.rs`)

**What:** The KDE reader populates per-widget sizing (button, checkbox,
scrollbar, etc.) but never sets global `ThemeDefaults` geometry:
`radius`, `radius_lg`, `frame_width`, `disabled_opacity`,
`border_opacity`, `shadow_enabled`, `focus_ring_width`,
`focus_ring_offset`.

These are known Breeze constants (section 1.3.3 / 2.1.6):
- `radius = 5.0` (`Frame_FrameRadius`)
- `frame_width = 1.0` (`PenWidth::Frame` rounded from 1.001)
- `focus_ring_width = 1.0` (stroke width)
- `focus_ring_offset = 2.0` (`PM_FocusFrameHMargin`)
- `shadow_enabled = true` (KWin compositor)

`radius_lg`, `disabled_opacity`, `border_opacity` are not Breeze
constants (platform-facts marks them "(none) -- preset") so they
legitimately stay None.

Unlike the GNOME reader (which merges onto an adwaita preset
containing these values), the KDE reader builds from scratch and returns
a ThemeSpec with all geometry defaults as None.

### Options

**A. Add the 5 known constants to `populate_widget_sizing()` in metrics.rs**

- Pro: `from_kde()` returns a self-sufficient theme with correct
  geometry. Consumers need not merge with a preset.
- Pro: Matches the existing pattern -- `metrics.rs` already hardcodes
  Breeze constants for widgets.
- Con: Hardcodes Breeze values. If a user uses a different Qt style
  (Kvantum, Oxygen) the geometry would be wrong. But the existing widget
  metrics already hardcode Breeze, so this is not a new problem.

**B. Have `from_kde()` merge with the kde-breeze preset internally (like GNOME does)**

- Pro: All preset values (geometry, spacing, switch, spinner, etc.) are
  included automatically.
- Pro: Eliminates all "missing from KDE reader" issues at once
  (chapters 9, 10, 11, 12, 13, 14, 15).
- Con: The GNOME reader does this because GNOME's CSS-sourced values
  cannot be read at runtime -- they must come from a preset. KDE's
  values *can* be read from kdeglobals, so the architecture is
  intentionally different.
- Con: Merging a preset means `from_kde()` returns Breeze-preset
  colors as defaults, overwriting them with live colors. But for
  non-color fields that the reader doesn't set, preset values leak
  through. This is desirable for geometry but means the theme is a
  hybrid of live data and static preset data -- the same design the
  GNOME reader uses, but it was an explicit choice to keep KDE
  different.
- Con: If a user has a non-Breeze color scheme (e.g. Nordic), merging
  the kde-breeze preset would inject Breeze-specific non-color values
  (like spinner diameter, switch sizes) that may not match the active Qt
  style. This is also true of the GNOME reader (Adwaita values leak
  into non-Adwaita GNOME themes), so it is a known limitation rather
  than a new flaw.

**C. Leave as-is and document that users should merge with a preset**

- Pro: No code change.
- Con: API inconsistency with `from_gnome()` which produces a complete
  theme. Users of `from_kde()` must know to call
  `ThemeSpec::preset("kde-breeze")?.merge(&live_theme)` or get a theme
  full of None geometry.

### Recommendation

This is the hardest trade-off in this list. The choice is between A
(add just the 5 constants) and B (merge with preset like GNOME does).

**Arguments for A over B:**

- A is a minimal, safe change that fixes only the geometry gap.
- A does not change the `from_kde()` return type semantics (still a
  "live" theme, not a preset hybrid).
- The remaining missing fields (spacing, switch, spinner, etc.) are
  widget-specific sizing that has lower impact than geometry defaults.
  Geometry defaults are inherited by all widgets during resolution.

**Arguments for B over A:**

- B fixes every remaining "missing from KDE reader" issue at once.
- B gives `from_kde()` API parity with `from_gnome()`.
- Consumers expect `from_<platform>()` to return a ready-to-use theme.
  Returning one with None geometry, None spacing, and None switch/
  spinner sizing forces every consumer to manually merge with a preset.

**Deciding factor:** A consumer calling `from_kde()` expects a theme
that resolves to a complete set of values. If `from_gnome()` returns a
complete theme (it does), `from_kde()` should too. Anything less is a
usability trap. The hybrid-preset-plus-live approach is already the
accepted pattern in the GNOME reader.

**Final recommendation: Option B.** Have `from_kde()` load the
kde-breeze preset as base, then overlay live colors, fonts, icon set,
text scale, and accessibility from kdeglobals -- exactly mirroring the
GNOME reader's architecture. This resolves chapters 9, 10, 11, 12, 13,
14, and 15 in one structural change.

If option B is chosen, `metrics.rs` becomes redundant (the preset
already contains every Breeze constant). It could be kept as a source-
of-truth reference or removed to avoid duplication.

However, there is a counter-argument: `metrics.rs` values match the
*current* version of Breeze (they can be updated in sync with the
installed Breeze), while the preset is a shipped snapshot. Keeping
`metrics.rs` and overlaying it after the preset gives the reader
version-correct widget sizing. This makes the merge order:
**preset -> metrics -> live kdeglobals**.

---

## 10. KDE reader does not set spacing scale

**File:** `native-theme/src/kde/mod.rs`, `native-theme/src/kde/metrics.rs`

**What:** Breeze defines three spacing constants (section 1.3.5):
- `Layout_DefaultSpacing = 6`
- `Layout_ChildMarginWidth = 6`
- `Layout_TopLevelMarginWidth = 10`

The KDE reader does not map these to `ThemeSpacing` (`xxs` through
`xxl`).

### Options

**A. Map the three constants to appropriate ThemeSpacing slots**

A reasonable mapping:
- `s = 6.0` (DefaultSpacing / ChildMarginWidth)
- `l = 10.0` (TopLevelMarginWidth)
- Remaining slots (xxs, xs, m, xl, xxl) derived from a geometric
  progression or left to the preset.

- Pro: Consumers get a functional spacing scale from `from_kde()`.
- Con: The mapping from 3 specific constants to a 7-level abstract
  scale requires inventing values for the 4 unmapped slots.

**B. Resolved by chapter 9 option B (preset merge)**

- Pro: The kde-breeze preset already has a complete spacing scale
  (`xxs=2, xs=4, s=6, m=8, l=12, xl=16, xxl=24`).
- Pro: No new mapping logic needed.
- Con: The preset's spacing scale is itself an invention (KDE has no
  abstract spacing scale per platform-facts section 1.3.5). But this is
  acceptable -- the preset documents its design choices.

**C. Leave unset**

- Pro: Honest: KDE has no spacing scale concept.
- Con: Consumers get None for all spacing values, making layout
  impossible without manual overrides.

### Recommendation

**If chapter 9 chooses option B** (preset merge), this is automatically
resolved.

**If chapter 9 chooses option A**, then option A here is the fallback:
add the 3 known constants to the spacing slots and leave the rest to
resolution defaults. However, this produces a partial scale with gaps,
which is worse than a complete preset-derived scale.

**Recommendation: Resolved by chapter 9 option B.**

---

## 11. KDE reader does not set ComboBox metrics

**File:** `native-theme/src/kde/metrics.rs`

**What:** Platform-facts section 1.3.4 lists two ComboBox constants:
- `ComboBox_FrameWidth = 6` -> `combo_box.padding_horizontal`
- `MenuButton_IndicatorWidth = 20` -> `combo_box.arrow_area_width`

Neither is populated by `populate_widget_sizing()`.

### Options

**A. Add two lines to `metrics.rs`**

- Pro: Trivial.
- Con: Only 2 of 6 ComboBoxTheme fields are populated; the rest
  (min_height, min_width, arrow_size, radius) remain None.

**B. Resolved by chapter 9 option B (preset merge)**

- Pro: The preset has all 6 ComboBox fields populated.

**C. Leave unset**

- Con: ComboBox sizing is wrong without manual intervention.

### Recommendation

**Resolved by chapter 9 option B.** If that is not chosen, option A as
a partial fix.

---

## 12. KDE reader does not set Switch metrics

**File:** `native-theme/src/kde/metrics.rs`

**What:** Platform-facts section 1.3.4 documents QQC2 Switch:
- Track: ~36 x 18px (font-derived: `implicitWidth = height * 2`,
  `height = fontMetrics.height` ~18px at default font)
- Handle: ~18px (= fontMetrics.height)

No SwitchTheme fields are set.

### Options

**A. Add constants to `metrics.rs`**

```rust
variant.switch.track_width = Some(36.0);
variant.switch.track_height = Some(18.0);
variant.switch.thumb_size = Some(18.0);
variant.switch.track_radius = Some(9.0); // half height (pill)
```

- Pro: Direct from platform-facts.
- Con: These are approximate (font-derived). If the user's font metrics
  differ from Noto Sans 10pt, the native switch will be a different size.
- Con: 4 lines of code for values that are approximations.

**B. Resolved by chapter 9 option B (preset merge)**

- Pro: Preset has switch values
  (`track_width=40, track_height=20, thumb_size=16, track_radius=10`).
  Note: the preset values differ slightly from the platform-facts
  QQC2 font-derived values. The preset values are rounder numbers that
  may have been chosen for visual consistency.
- Con: The preset's switch values (40x20) don't match the platform-facts
  QQC2 values (36x18). This warrants a separate check.

**C. Leave unset**

- Con: Toggle switches render without native sizing.

### Recommendation

**Resolved by chapter 9 option B.** However, the preset's switch values
should be verified against the platform-facts QQC2 measurements and
corrected if needed. A follow-up audit should compare preset switch
values (40x20 track, 16 thumb) against platform-facts (36x18 track, 18
thumb). Given that the QQC2 values are font-metric-derived and vary with
the user's font, the preset's rounded values are a defensible
approximation but should be documented as such.

---

## 13. KDE reader does not set Spinner diameter

**File:** `native-theme/src/kde/metrics.rs`

**What:** Platform-facts section 1.3.4 / 2.23 document KDE QQC2
BusyIndicator default diameter = 36px (`Kirigami.Units.gridUnit * 2`).
SpinnerTheme.diameter is not set.

### Options

**A. Add to `metrics.rs`:** `variant.spinner.diameter = Some(36.0);`

- Pro: One line. Matches platform-facts.
- Con: gridUnit is dynamic (depends on font metrics). 36px is only
  correct at the default 10pt Noto Sans.

**B. Resolved by chapter 9 option B (preset merge)**

- Pro: Preset has `spinner.diameter = 24.0`.
- Con: Preset value (24) differs from platform-facts (36). This is
  another preset accuracy issue (see below).

**C. Leave unset**

- Con: Spinner renders without native sizing.

### Recommendation

**Resolved by chapter 9 option B.** However, the preset's spinner
diameter (24) should be checked against the platform-facts value (36).
If 36 is correct for Breeze default, the preset should be updated.
The discrepancy likely comes from confusing with some other platform's
spinner size.

---

## 14. KDE reader does not set Expander arrow_size

**File:** `native-theme/src/kde/metrics.rs`

**What:** Platform-facts section 1.3.4 documents
`ItemView_ArrowSize = 10` (tree/disclosure arrow size). This maps to
`expander.arrow_size` per section 2.27.

### Options

**A. Add to `metrics.rs`:** `variant.expander.arrow_size = Some(10.0);`

- Pro: One line. Direct from `breezemetrics.h`.
- Con: Minimal.

**B. Resolved by chapter 9 option B (preset merge)**

- Pro: Preset has `expander.arrow_size = 12.0`.
- Con: Preset value (12) differs from platform-facts (10). Another
  preset accuracy issue.

**C. Leave unset**

- Con: Expander arrows render without native sizing.

### Recommendation

**Resolved by chapter 9 option B.** The preset's expander arrow_size
(12) should be checked: platform-facts says `ItemView_ArrowSize = 10`.
If 10 is the correct Breeze value, the preset should be corrected.

---

## 15. KDE reader does not set `line_height`

**File:** `native-theme/src/kde/mod.rs`

**What:** Platform-facts section 2.1.1 documents KDE Noto Sans
`line_height = 1.36` (from sTypo metrics). The reader never sets
`defaults.line_height`.

### Options

**A. Hardcode `1.36` in the reader**

- Pro: Simple. Correct for the default Noto Sans font.
- Con: Wrong for any other font. If the user sets their KDE font to
  Inter (line_height ~1.21) or Roboto (1.36 same as Noto), the
  hardcoded value would be wrong for non-Noto-metric-compatible fonts.

**B. Compute from font metrics at runtime**

- Pro: Correct for any font.
- Con: Requires reading the font file, parsing the OS/2 or hhea table
  to extract ascender/descender/lineGap, and computing the ratio. This
  is a significant addition (font file lookup + binary parsing).
- Con: May require new dependencies (e.g., a font metrics crate).

**C. Resolved by chapter 9 option B (preset merge)**

- Pro: Preset has `line_height = 1.4` (or 1.36 after chapter 3 fix).
- Con: Like option A, this is correct only for the default font and
  wrong for custom fonts. But it is the same limitation the GNOME reader
  has (Adwaita preset provides line_height for Cantarell/Adwaita Sans).

**D. Leave unset and let the resolver apply a default**

- Pro: No code change.
- Con: Consumers get None, which the resolver must handle. If the
  resolver has a sensible global default (e.g. 1.2), this may be
  acceptable.

### Recommendation

**Option C (resolved by chapter 9 option B)**, with the preset value
corrected to 1.36 per chapter 3. The GNOME reader has the same
limitation (preset-derived line_height is wrong for custom fonts), so
this maintains consistency. Computing line_height from font metrics
(option B) would be a valuable future improvement for both readers, but
it is a larger feature, not a bugfix.

</details>

---

## 16. GNOME reader does not read portal `reduced-motion`

**File:** `native-theme/src/gnome/mod.rs` lines 197-203

**What:** The reader only checks `gsettings get
org.gnome.desktop.interface enable-animations` for reduce_motion.
Platform-facts section 1.4.7 lists the portal
`org.freedesktop.appearance reduced-motion` key as an additional source.
The portal is the *preferred* source per the freedesktop spec (it works
in sandboxed Flatpak environments where gsettings may not be available).

### Options

**A. Read `reduced-motion` from the portal via ashpd, fall back to gsettings**

- Pro: Preferred source per freedesktop spec.
- Pro: Works in sandboxed (Flatpak) environments.
- Pro: The portal `Settings` object is already constructed in
  `from_gnome()` and passed indirectly via `build_theme()` -- but
  currently only `color_scheme`, `accent_color`, and `contrast` are
  read. Adding `reduced-motion` is consistent.
- Con: The ashpd crate must support reading the `reduced-motion` key.
  If it doesn't have a typed accessor, a raw
  `read_value("org.freedesktop.appearance", "reduced-motion")` call is
  needed.
- Con: The portal read is async, adding a bit of complexity if the
  current gsettings path is sync.

**B. Keep gsettings only, add portal as a future enhancement**

- Pro: Simpler. gsettings works for non-sandboxed desktop apps.
- Con: Flatpak apps cannot read gsettings; they rely on the portal.
- Con: Inconsistent: `color_scheme`, `accent_color`, and `contrast`
  already come from the portal; `reduced-motion` is the only one going
  through gsettings.

**C. Leave as-is**

- Pro: No change.
- Con: Flatpak apps get no reduce_motion information; inconsistent with
  how the reader handles the other appearance keys.

### Recommendation

**Option A.** The `from_gnome()` function already reads three portal
appearance keys. `reduced-motion` is the fourth key in the same portal
namespace (`org.freedesktop.appearance`). Adding it makes the reader
complete and consistent. The gsettings path should remain as fallback
for environments where the portal is unavailable.

Concretely: read from portal first; if the portal returns a value, use
it. If portal read fails, fall back to gsettings `enable-animations`.

---

## 17. GNOME reader does not fall back to gsettings `high-contrast`

**File:** `native-theme/src/gnome/mod.rs` lines 155-158

**What:** The reader sets `high_contrast` only from the portal
`contrast` key (`Contrast::High`). Platform-facts section 1.4.7 also
lists `org.gnome.desktop.a11y.interface high-contrast` as a gsettings
source. If the portal is unavailable (no D-Bus session, old GNOME
version), there is no gsettings fallback.

### Options

**A. Add a gsettings fallback for high-contrast**

```rust
// After portal contrast check, if high_contrast is still None:
if variant.defaults.high_contrast.is_none() {
    if let Some(hc_str) = read_gsetting("org.gnome.desktop.a11y.interface", "high-contrast") {
        match hc_str.as_str() {
            "true" => variant.defaults.high_contrast = Some(true),
            "false" => variant.defaults.high_contrast = Some(false),
            _ => {}
        }
    }
}
```

- Pro: Matches the precedent set by `enable-animations` (also read from
  gsettings as fallback).
- Pro: Covers old GNOME versions that have the gsetting but not the
  portal key.
- Con: Slightly more code.

**B. Leave portal-only**

- Pro: The portal is the modern, preferred path.
- Con: On GNOME < 44 (which predates the portal `contrast` key), users
  with high contrast enabled get no signal.

**C. Read gsettings only (drop portal check)**

- Pro: Simpler.
- Con: Portal is preferred for sandboxed apps.

### Recommendation

**Option A.** Mirror the pattern already used for `enable-animations`:
portal first, gsettings fallback. The two-line check is trivial and
covers older GNOME versions.

---

## 18. GNOME `section_heading` gets wrong weight (400 instead of 700)

**Root cause: missing Adwaita preset entry + missing reader computation**

**Files:** `adwaita.toml` + `adwaita-live.toml` (no text_scale section
in either), `native-theme/src/gnome/mod.rs` lines 234-253

**What:** Platform-facts section 2.19 maps GNOME `section_heading` to
the `.heading` CSS class: inherited base font size, weight 700 (Bold).

The current pipeline produces weight 400 (Regular):
1. GNOME reader `compute_text_scale()` returns `section_heading: None`.
2. Adwaita preset has NO `text_scale.section_heading` entry at all.
3. After merge: `section_heading = None`.
4. `resolve_text_scale_entry()` creates the entry, fills `size` from
   `defaults.font.size` (correct) and `weight` from
   `defaults.font.weight` which is 400 (WRONG -- should be 700).

The v0.5.0 design doc (`v0.5.0_resolution.md:322-324`) says:
> `text_scale.section_heading` needs no OS reader computation --
> `.heading` uses inherited (= base) font size, so `size <-
> defaults.font.size` via resolve(). Only `weight = 700` is needed
> (-> `adwaita.toml`).

So the design doc intended the Adwaita preset to provide weight 700,
but the preset was never updated with this entry.

### Options

**A. Add `section_heading.weight = 700` to the Adwaita preset**

```toml
[light.text_scale.section_heading]
weight = 700
```

- Pro: Follows the v0.5.0 design doc exactly (weight from preset).
- Pro: `resolve()` fills `size` from `defaults.font.size`, so it adapts
  to custom base font sizes automatically.
- Pro: Minimal change (2 lines per variant in adwaita.toml).
- Con: None.

**B. Add section_heading to GNOME `compute_text_scale`**

```rust
section_heading: Some(TextScaleEntry {
    size: Some(base_size), // 100% of base, same as .heading CSS
    weight: Some(700),     // Bold
    line_height: None,
}),
```

- Pro: All four text scale entries are computed in one place.
- Pro: Adapts to custom base font size (though resolve() also does
  this for option A).
- Con: Contradicts the design doc which says "needs no OS reader
  computation".
- Con: Explicitly setting `size = base_size` is redundant with
  resolve(), which would fill the same value from defaults.font.size.

**C. Do both: preset provides weight, reader provides size**

- Pro: Belt-and-suspenders.
- Con: Unnecessary duplication. resolve() already fills size from
  defaults if None.

### Recommendation

**Option A.** This follows the design doc's intent precisely: the preset
provides the Bold weight, and resolve() fills the base font size. The
design doc explicitly says the reader should NOT compute this entry.
The only fix needed is the 2-line preset addition that was missed during
implementation.

Also add the same entry to `kde-breeze.toml` for KDE. For KDE, the
section_heading weight is less clear-cut (Kirigami Level 2 headings
use inherited weight, not explicitly Bold), but the KDE reader already
computes section_heading explicitly (kde/mod.rs:97-100) so the preset
entry would only serve as a fallback and would be overwritten by the
reader.

---

## 19. Adwaita preset `radius` is 12.0 -- should be 9.0

**Files:** `adwaita.toml` lines 36, 238 AND `adwaita-live.toml` lines 8, 142

**What:** The preset uses `radius = 12.0`. Platform-facts section 1.4.3
shows Adwaita has two distinct radii:
- `$button_radius = 9px` -- the **control** corner radius (buttons,
  inputs, checkboxes, etc.)
- `$card_radius = 12px` -- the **card** corner radius

Per section 2.1.6, `defaults.radius` maps to the control radius (9px on
GNOME). The preset has used the card radius instead. Since
`card.radius` inherits from `defaults.radius_lg` (not `defaults.radius`)
in resolve.rs, the card radius is not relevant here.

### Options

**A. Change to 9.0**

- Pro: Matches `$button_radius`, the authoritative GNOME control radius.
- Pro: Affects all widgets that inherit `← defaults.radius` (button,
  input, checkbox, tooltip, progress_bar, combo_box, segmented_control,
  expander). All of these use the control radius per platform-facts.
- Con: Visible change for existing users.

**B. Leave as 12.0**

- Pro: No visual change.
- Con: Buttons, inputs, checkboxes all get 12px radius, which is
  Adwaita's card radius -- not the control radius. They look too rounded
  compared to native GNOME.

### Recommendation

**Option A.** The `defaults.radius` field is documented as the control
radius in both platform-facts §2.1.6 and the v0.5.0 design docs.
12.0 is `$card_radius`, which belongs in card.radius (inherited from
radius_lg), not in defaults.radius.

---

**Note for chapters 20-38:** Every Adwaita fix applies to BOTH
`adwaita.toml` AND `adwaita-live.toml` (same values, both variants).

## 20. Adwaita preset `radius_lg` is 14.0 -- should be 15.0

**Files:** `adwaita.toml` + `adwaita-live.toml`

**What:** The preset uses `radius_lg = 14.0`. Platform-facts section
1.4.3 says `$button_radius + 6 = 9 + 6 = 15px` for window/dialog
radius. 14.0 has no cited source.

### Options

**A. Change to 15.0**

- Pro: Matches `$button_radius + 6` from `_common.scss`.
- Pro: Affects window.radius, popover.radius, dialog.radius, card.radius
  via inheritance. All documented as using 15px on GNOME.
- Con: None.

**B. Leave as 14.0**

- Con: Off by 1px from the documented GNOME value. No source for 14.

### Recommendation

**Option A.**

---

## 21. Adwaita preset `line_height` is 1.4 -- should be ~1.21

**File:** `native-theme/src/presets/adwaita.toml` lines 42, 244

**What:** The preset uses `line_height = 1.4`. Platform-facts section
2.1.1 documents Adwaita Sans (GNOME 48+) font metrics yielding
`(1984 + 494 + 0) / 2048 = 1.21`. Pre-48 Cantarell yields 1.2.

1.4 is 16% higher than the real value. At 11pt, this produces
`11 * 1.4 = 15.4px` line height instead of `11 * 1.21 = 13.3px`.
The text_scale line_height derivations (which compute
`line_height * size`) are all wrong as a consequence.

### Options

**A. Change to 1.21 (Adwaita Sans)**

- Pro: Matches GNOME 48+ default font.
- Con: Wrong for pre-48 GNOME (Cantarell = 1.2). But Adwaita Sans is
  the current default since GNOME 48.

**B. Change to 1.2 (Cantarell)**

- Pro: More conservative; close to both Cantarell (1.2) and Adwaita
  Sans (1.21).
- Con: Not exact for either font.

**C. Leave as 1.4**

- Con: Wrong for every GNOME font. 1.4 matches no known default.

### Recommendation

**Option A (1.21).** The preset name is "Adwaita" and represents
GNOME 48+. Cantarell pre-48 compatibility is not the preset's job.
Same analysis as chapter 3 (KDE line_height).

---

## 22. Adwaita preset `focus_ring_offset` is 1.0 -- should be -2.0

**File:** `native-theme/src/presets/adwaita.toml` lines 44, 246

**What:** The preset uses `focus_ring_offset = 1.0` (outset). Platform-
facts section 1.4.3 says libadwaita uses `outline-offset: -$width`
where `$width = 2`, giving `-2px` (inset). GNOME focus rings are drawn
*inside* the element, not outside.

This is the same class of bug as chapter 2 (KDE focus ring swap), but
with an additional sign error: 1.0 is positive (outset) when the real
value is -2.0 (inset). Both magnitude and polarity are wrong.

### Options

**A. Change to -2.0**

- Pro: Matches libadwaita CSS `outline-offset: -2px`.
- Pro: Focus ring renders inside the element boundary, matching native
  GNOME appearance.
- Con: Requires the framework to support negative offsets (inset). Most
  do (CSS offset is signed by design).

**B. Change to 2.0 (correct magnitude, wrong polarity)**

- Pro: Gets the magnitude right.
- Con: Outset instead of inset. GNOME focus rings would look wrong.

**C. Leave as 1.0**

- Con: Both magnitude and polarity are wrong.

### Recommendation

**Option A.** The `focus_ring_offset` field is documented as signed
(negative = inset). libadwaita's CSS is explicit: `-$width` = -2. The
semantic difference matters: inset focus rings on GNOME avoid the
Adwaita element-spacing issues that outset rings cause.

---

## 23. Adwaita preset `slider.track_height` is 6 -- should be 10

**File:** `native-theme/src/presets/adwaita.toml` (light.slider / dark.slider)

**What:** Preset has `track_height = 6.0`. Platform-facts section 1.4.4
says Scale trough min-height = 10px (`_scale.scss`). Section 2.9
confirms GNOME slider track_height = 10.

### Options

**A. Change to 10.0**

- Pro: Matches libadwaita _scale.scss. Straightforward data fix.
- Con: None.

**B. Leave as 6.0**

- Con: Off by 67%. Sliders look visibly thinner than native GNOME.

### Recommendation

**Option A.**

---

## 24. Adwaita preset `progress_bar.height` is 6 -- should be 8

**File:** `native-theme/src/presets/adwaita.toml` (light.progress_bar / dark.progress_bar)

**What:** Preset has `height = 6.0`. Platform-facts section 1.4.4 says
ProgressBar bar height = 8px (`_progress-bar.scss`). Section 2.10
confirms GNOME progress bar height = 8.

### Options

**A. Change to 8.0** -- Pro: Matches source. Con: None.

**B. Leave as 6.0** -- Con: 25% thinner than native GNOME.

### Recommendation

**Option A.**

---

## 25. Adwaita preset `progress_bar.min_width` is 100 -- should be 80

**File:** `native-theme/src/presets/adwaita.toml` (light.progress_bar / dark.progress_bar)

**What:** Preset has `min_width = 100.0`. Platform-facts section 2.10
says GNOME progress bar min_width is 80px from Adwaita CSS.

### Options

**A. Change to 80.0** -- Pro: Matches Adwaita CSS. Con: None.

**B. Leave as 100.0** -- Con: 25% wider minimum than native.

### Recommendation

**Option A.**

---

## 26. Adwaita preset `menu.item_height` is 34 -- should be 32

**File:** `native-theme/src/presets/adwaita.toml` (light.menu / dark.menu)

**What:** Preset has `item_height = 34.0`. Platform-facts section 1.4.4
says `modelbutton { min-height: 32px }` from `_menus.scss`. Section 2.6
confirms GNOME menu item_height = 32.

### Options

**A. Change to 32.0** -- Pro: Matches _menus.scss. Con: None.

**B. Leave as 34.0** -- Con: 2px taller than native. Menus feel slightly
off, especially with many items.

### Recommendation

**Option A.**

---

## 27. Adwaita preset `menu.padding_horizontal` is 8 -- should be 12

**File:** `native-theme/src/presets/adwaita.toml` (light.menu / dark.menu)

**What:** Preset has `padding_horizontal = 8.0`. Platform-facts section
1.4.4 says `$menu_padding = 12` from `_common.scss`, and section 2.6
confirms GNOME menu padding_horizontal = 12.

### Options

**A. Change to 12.0** -- Pro: Matches `$menu_padding`. Con: None.

**B. Leave as 8.0** -- Con: Menu text sits 4px closer to edges than
native GNOME. Noticeable.

### Recommendation

**Option A.**

---

## 28. Adwaita preset `tooltip.padding_horizontal` is 6 -- should be 10

**File:** `native-theme/src/presets/adwaita.toml` (light.tooltip / dark.tooltip)

**What:** Preset has `padding_horizontal = 6.0`, `padding_vertical = 6.0`.
Platform-facts section 1.4.4 says tooltip padding = **6px vert / 10px
horiz** (`_tooltip.scss`). The vertical value is correct but the
horizontal value is wrong.

### Options

**A. Change horizontal to 10.0, keep vertical at 6.0**

- Pro: Matches _tooltip.scss exactly.
- Con: None.

**B. Leave both at 6.0**

- Con: Tooltips are 4px narrower on each side than native GNOME.

### Recommendation

**Option A.**

---

## 29. Adwaita preset `button.padding_horizontal` is 12 -- should be 10

**File:** `native-theme/src/presets/adwaita.toml` (light.button / dark.button)

**What:** Preset has `padding_horizontal = 12.0`. Platform-facts section
1.4.4 says button CSS `padding: 5px 10px` -- the horizontal padding is
10px.

### Options

**A. Change to 10.0** -- Pro: Matches _buttons.scss. Con: None.

**B. Leave as 12.0** -- Con: Buttons 2px wider on each side than native.

### Recommendation

**Option A.**

---

## 30. Adwaita preset `button.padding_vertical` is 8 -- should be 5

**File:** `native-theme/src/presets/adwaita.toml` (light.button / dark.button)

**What:** Preset has `padding_vertical = 8.0`. Platform-facts section
1.4.4 says button CSS `padding: 5px 10px` -- the vertical padding is
5px.

Note: `button.min_height = 34` is correct as total height
(24 CSS min-height + 2 * 5px padding = 34px). But with
`padding_vertical = 8`, the implied content area is 34 - 2*8 = 18px,
which contradicts the CSS min-height of 24px. Fixing padding_vertical
to 5 restores internal consistency.

### Options

**A. Change to 5.0** -- Pro: Matches _buttons.scss. Restores consistency
with min_height. Con: None.

**B. Leave as 8.0** -- Con: Internal inconsistency: 34 - 2*8 = 18 ≠ 24
(the CSS min-height).

### Recommendation

**Option A.**

---

## 31. Adwaita preset `input.padding_horizontal` is 12 -- should be 9

**File:** `native-theme/src/presets/adwaita.toml` (light.input / dark.input)

**What:** Preset has `padding_horizontal = 12.0`. Platform-facts section
1.4.4 says `padding-left: 9px; padding-right: 9px` from `_entries.scss`.

### Options

**A. Change to 9.0** -- Pro: Matches _entries.scss. Con: None.

**B. Leave as 12.0** -- Con: Input text sits 3px further from edges than
native GNOME.

### Recommendation

**Option A.**

---

## 32. Adwaita preset `input.padding_vertical` is 8 -- should be 0

**File:** `native-theme/src/presets/adwaita.toml` (light.input / dark.input)

**What:** Preset has `padding_vertical = 8.0`. Platform-facts section 2.4
says GNOME input padding_vertical is "0 (vertical space from
min-height)". In Adwaita CSS, the entry min-height (34px) provides the
vertical space; explicit vertical padding is 0 or near-0.

### Options

**A. Change to 0.0**

- Pro: Matches platform-facts and Adwaita CSS semantics.
- Pro: The `min_height = 34.0` already ensures the correct total height.
- Con: Frameworks that rely solely on padding (no min-height concept)
  would render a zero-height input. But min_height exists precisely for
  this case.

**B. Change to a derived value like 5.0 ((34 - ~15 font) / 2 ≈ 9.5, rounded)**

- Pro: Gives frameworks without min-height support a working padding.
- Con: Fabricated value with no CSS source. Contradicts platform-facts.

**C. Leave as 8.0**

- Con: No CSS source. Inconsistent with min_height (34 - 2*8 = 18, but
  the CSS content area is larger).

### Recommendation

**Option A.** The model has min_height specifically so that vertical
space can come from the height constraint rather than padding. Setting
padding_vertical = 0 with min_height = 34 is exactly how Adwaita CSS
works. Frameworks that need padding-based layout can compute it from
`(min_height - font_size) / 2`.

---

## 33. Adwaita preset `splitter.width` is 6 -- should be 1

**File:** `native-theme/src/presets/adwaita.toml` (light.splitter / dark.splitter)

**What:** Preset has `width = 6.0`. Platform-facts section 2.17 says
GNOME GtkPaned default separator = 1px, wide handle = 5px. Neither
matches 6.

### Options

**A. Change to 1.0 (the default)**

- Pro: Matches the default GtkPaned separator width.
- Con: 1px splitters can be hard to grab with a mouse.

**B. Change to 5.0 (the wide handle)**

- Pro: Matches the wide-handle GtkPaned variant.
- Con: Wide handles are opt-in, not the default.

**C. Leave as 6.0**

- Con: Matches neither documented value.

### Recommendation

**Option A (1.0).** The preset represents the default GNOME appearance.
Wide handles are an application-level opt-in, not the default.

---

## 34. Adwaita preset `toolbar.height` is 46 -- should be 47

**File:** `native-theme/src/presets/adwaita.toml` (light.toolbar / dark.toolbar)

**What:** Preset has `height = 46.0`. Platform-facts section 1.4.4 says
headerbar min-height = 47px from `_header-bar.scss`.

### Options

**A. Change to 47.0** -- Pro: Matches _header-bar.scss. Con: None.

**B. Leave as 46.0** -- Con: Off by 1px. Minor but incorrect.

### Recommendation

**Option A.**

---

## 35. Adwaita preset `spinner.diameter` is 24 -- should be 16

**File:** `native-theme/src/presets/adwaita.toml` (light.spinner / dark.spinner)

**What:** Preset has `diameter = 24.0`. Platform-facts section 1.4.4
says `GtkSpinner DEFAULT_SIZE = 16` from `gtkspinner.c`. Section 2.23
confirms GNOME spinner diameter = 16.

### Options

**A. Change to 16.0**

- Pro: Matches GtkSpinner default size.
- Con: 16px is small for some use cases. But the preset should reflect
  the platform default; apps can override.

**B. Leave as 24.0**

- Con: 50% larger than native. No cited source for 24 on GNOME.

### Recommendation

**Option A.**

---

## 36. Adwaita preset switch metrics all wrong

**File:** `native-theme/src/presets/adwaita.toml` (light.switch / dark.switch)

**What:** Three switch values don't match platform-facts section 1.4.4 /
section 2.21:

| Field | Preset | Platform-facts | Source |
|---|---|---|---|
| `track_width` | 40 | ~46 | §1.4.4: total track ~46px (derived from 2*thumb + 2*3px padding) |
| `track_height` | 20 | ~26 | §1.4.4: 20px thumb + 2*3px padding = 26px |
| `thumb_size` | 16 | 20 | §1.4.4: GtkSwitch thumb = 20x20px |

The preset's `thumb_size = 16` appears to be a KDE value
(`kde-breeze.toml` also uses 16); the GNOME thumb is 20x20px per
`_checks.scss`.

### Options

**A. Change all three: track_width=46, track_height=26, thumb_size=20**

- Pro: Matches the CSS-derived dimensions from platform-facts.
- Con: The ~46 and ~26 values are derived (not direct CSS values). The
  direct CSS values are: thumb=20x20, padding=3px, track_radius=14px.
  The total track size is derived from these.

**B. Use the direct CSS values: thumb=20, keep track as derived**

- Pro: thumb_size=20 is a direct CSS value. track_width/height can be
  derived by the framework from thumb + padding.
- Con: The model expects explicit track dimensions, not derivation.

**C. Leave as-is**

- Con: All three values are wrong for GNOME.

### Recommendation

**Option A.** The model fields are explicit dimensions; the framework
should not need to derive them. Use the derived totals from platform-
facts: `track_width = 46.0`, `track_height = 26.0`, `thumb_size = 20.0`.
Also set `track_radius = 14.0` (pill shape, from §1.4.4).

---

## 37. Adwaita preset `icon_sizes.toolbar` is 24 -- should be 16

**File:** `native-theme/src/presets/adwaita.toml` lines 70, 272

**What:** Preset has `toolbar = 24.0`. Platform-facts section 1.4.6 says
GNOME uses `GTK_ICON_SIZE_NORMAL` (16px) for toolbar icons. Section
2.1.8 confirms GNOME toolbar = 16. GNOME has no 24px icon context; GTK4
has only two sizes: NORMAL (16) and LARGE (32).

### Options

**A. Change to 16.0**

- Pro: Matches GTK4's GTK_ICON_SIZE_NORMAL, the standard toolbar size.
- Con: 16px toolbar icons may feel small to users coming from KDE (22)
  or macOS (24-32). But the preset represents GNOME native.

**B. Leave as 24.0**

- Con: No GNOME source for 24. Falls between GTK4's two sizes. Icon
  lookups for 24px would miss the 16px native SVGs or wastefully upscale
  them.

### Recommendation

**Option A.**

---

## 38. Adwaita preset `tab.min_height` is 34 -- should be 30

**File:** `native-theme/src/presets/adwaita.toml` (light.tab / dark.tab)

**What:** Preset has `min_height = 34.0`. Platform-facts section 1.4.4
says Notebook tab min-height = 30px from `_notebook.scss`. Section 2.11
confirms GNOME tab min_height = 30.

### Options

**A. Change to 30.0** -- Pro: Matches _notebook.scss. Con: None.

**B. Leave as 34.0** -- Con: Tabs 4px taller than native.

### Recommendation

**Option A.**

---

## 39. KDE reader maps `defaults.border` to accent color (DecorationFocus)

**File:** `native-theme/src/kde/colors.rs` line 25

**What:** The reader sets:
```rust
variant.defaults.border = get_color(ini, "Colors:Window", "DecorationFocus");
```

`DecorationFocus` is the focus/accent decoration color. This makes
`defaults.border` identical to the accent color (typically blue). On
Breeze Light, `Colors:Window DecorationFocus` is `#3daee9` (blue),
the same as the accent.

Platform-facts section 2.1.3 says KDE border is `"(preset) -- derived
from background"`, meaning KDE has **no native border color API**. The
border should be a neutral separator color, not the accent.

The `kde-breeze.toml` preset correctly sets border to neutral grays
(`#bcc0bf` light / `#4d545b` dark). But if chapter 9 is implemented
(from_kde merges with preset), the reader's DecorationFocus value
would **overwrite** the preset's correct border, making borders blue.

### Options

**A. Remove the border assignment from the reader entirely**

- Pro: Platform-facts says border is "(preset)". The reader should not
  set it. The preset provides the correct neutral gray.
- Pro: If chapter 9 (preset merge) is implemented, the preset's border
  is preserved.
- Pro: If from_kde() is used standalone, border stays None and resolve()
  falls back to d.border (which would need to be set by something --
  see con).
- Con: Without preset merge, border is None. resolve.rs has no
  safety-net fallback for defaults.border. validate() would flag it
  as missing.

**B. Derive border from background at read time**

- Pro: Matches the "(preset) -- derived from background" description.
  Could brighten/darken the background by ~20% for a neutral separator.
- Con: Hardcodes a derivation formula. Different KDE themes may want
  different border colors. The preset approach is more flexible.

**C. Read from a different KDE color key**

Candidates:
- `Colors:Window ForegroundInactive` -- used for muted text, not
  borders. Too transparent/light for a border.
- `Colors:View DecorationHover` -- hover decoration, not border.
- No KDE color key directly maps to "border/separator" per platform-
  facts.

- Pro: Stays dynamic / per-theme.
- Con: No KDE color key is semantically correct for borders. Any choice
  would be a guess.

**D. Leave as-is**

- Con: Borders are accent-colored (blue). Visually wrong: every
  separator, card border, and input border would be blue.

### Recommendation

**Option A.** Remove the line. Platform-facts is explicit: KDE border is
a preset value, not a system-provided one. The preset should be the
sole source.

If chapter 9 (preset merge) is implemented, the preset's neutral gray
border is the base and the reader doesn't touch it. If from_kde() is
used standalone without a preset, border being None is a signal that
the theme is incomplete -- which is accurate.

---

## 40. KDE reader does not set `list.background` and `list.foreground`

**File:** `native-theme/src/kde/colors.rs`

**What:** Platform-facts section 2.15 maps KDE list colors to
`[Colors:View]`:
- `list.background` <- `Colors:View BackgroundNormal`
- `list.foreground` <- `Colors:View ForegroundNormal`

The reader does not set these. They inherit via resolve() from
`defaults.background` (`Colors:Window BackgroundNormal`) and
`defaults.foreground` (`Colors:Window ForegroundNormal`).

In Breeze, these differ:
- `Colors:Window BackgroundNormal` = `#eff0f1` (gray)
- `Colors:View BackgroundNormal` = `#ffffff` (white)

Lists get a gray background instead of the native white. This is
visible in any list or table view.

### Options

**A. Add two lines to `populate_colors()`**

```rust
variant.list.background = get_color(ini, "Colors:View", "BackgroundNormal");
variant.list.foreground = get_color(ini, "Colors:View", "ForegroundNormal");
```

- Pro: Matches platform-facts exactly. Same pattern already used for
  `input.background` / `input.foreground` (which also use Colors:View).
- Pro: Trivial two-line addition.
- Con: None.

**B. Resolved by chapter 9 (preset merge)**

- Pro: The kde-breeze preset could set list colors.
- Con: The preset currently does NOT set list.background/foreground
  (only alternate_row, item_height, and padding). So the preset merge
  alone does not fix this. The reader must set them.

**C. Leave as-is**

- Con: Lists have the wrong background on any KDE theme where Window
  and View backgrounds differ (which is most themes).

### Recommendation

**Option A.** This is a straightforward two-line fix with no downsides.
The Colors:View group is the documented source for list/table content
areas on KDE.

---

## 41. `resolve.rs` -- `list.alternate_row` fallback is `defaults.background`

**File:** `native-theme/src/resolve.rs` line 416

**What:**
```rust
if self.list.alternate_row.is_none() {
    self.list.alternate_row = d.background;
}
```

On all four platforms, `alternate_row` is a slightly different shade
from the list background (section 2.15):
- macOS: `alternatingContentBackgroundColors[1]` (a distinct color)
- Windows: Fluent preset `#f9f9f9` light / `#262626` dark
- KDE: `Colors:View BackgroundAlternate` (a distinct color)
- GNOME: Adwaita CSS even row (a distinct color)

Falling back to `d.background` makes alternating rows identical to the
list background, defeating the purpose. In the worst case, after chapter
40 fix (list.background = Colors:View white) and this fallback
(alternate_row = Colors:Window gray), the alternate row would actually
be *darker* than the list background -- inverted zebra striping.

### Options

**A. Fall back to `self.list.background` instead of `d.background`**

```rust
if self.list.alternate_row.is_none() {
    self.list.alternate_row = self.list.background;
}
```

- Pro: alternate_row matches the list background, giving uniform rows
  (no stripes). This is a safe no-stripe default rather than an
  incorrect stripe.
- Con: Still doesn't provide actual alternating colors. But that data
  must come from the reader or preset, not from a resolve heuristic.

**B. Derive from `self.list.background` with a slight shade shift**

- Pro: Provides visible alternating rows as a fallback.
- Con: Hardcodes a derivation formula (e.g., 3% darker/lighter). The
  correct shade depends on the theme's lightness/darkness and color
  palette. A heuristic is unreliable.

**C. Fall back to `d.surface`**

- Pro: Surface is typically a slightly different shade from background,
  giving some visual distinction.
- Con: The distinction is not guaranteed and may be wrong in direction
  (lighter when it should be darker, or vice versa).

**D. Leave as `d.background`**

- Con: Incorrect on all platforms. The fallback produces a value that
  is not the list background (it's the window background), creating
  accidental zebra striping with the wrong color.

### Recommendation

**Option A.** Change the fallback to `self.list.background`. This runs
after `list.background` has already been resolved (line 232-233 in
phase 2, line 411 not needed since list.background is set by
resolve_color_inheritance). The result is uniform rows (no stripes)
rather than incorrect stripes. Actual alternating colors should always
come from the reader or preset.

Note: this must run AFTER `list.background` is resolved. Currently
`resolve_color_inheritance` sets `list.background` before reaching
`list.alternate_row`, so the ordering is already correct.

---

# Cross-cutting preset errors

## 42. All four platform presets use `line_height = 1.4`

**Files:** All 8 platform preset files (4 full + 4 live), both variants
in each. Community presets (dracula, catppuccin, nord, etc.) also
affected.

**What:** Every platform preset hardcodes `line_height = 1.4`.
Platform-facts section 2.1.1 documents different values per platform:

| Preset | Current | Correct | Source |
|--------|---------|---------|--------|
| KDE Breeze | 1.4 | **1.36** | Noto Sans sTypo `(1069+293)/1000` |
| Adwaita | 1.4 | **1.21** | Adwaita Sans sTypo `(1984+494)/2048` |
| macOS Sonoma | 1.4 | **1.19** | SF Pro sTypo `(1950+494)/2048` |
| Windows 11 | 1.4 | **1.43** | Fluent Body ramp `20px / 14px` |

1.4 matches no platform. It appears to have been a placeholder copied
to all presets during initial development.

See also: chapter 3 (KDE), chapter 21 (Adwaita) for per-preset detail.

### Options

**A. Set each preset to its correct platform value**

- Pro: Each preset accurately represents its platform's line spacing.
- Pro: text_scale line_height derivations (`line_height × size`) produce
  correct values.
- Con: Requires 4 different values across 8 files (2 variants each).

**B. Leave all at 1.4**

- Con: Wrong for every platform. macOS gets 18% too loose, Windows
  gets 2% too tight, GNOME gets 16% too loose, KDE gets 3% too loose.

### Recommendation

**Option A.** Each preset is named after a specific platform and must
reflect that platform's font metrics.

Community presets (Dracula, Catppuccin, etc.) that don't target a
specific platform can keep a reasonable default. 1.2 (the geometric
mean of the four values) would be better than 1.4.

---

## 43. All four platform presets have wrong `focus_ring_offset`

**Files:** All 8 platform preset files (4 full + 4 live), both variants.

**What:** Every preset uses `focus_ring_offset = 1.0`. The correct
values differ per platform:

| Preset | Current | Correct | Source |
|--------|---------|---------|--------|
| KDE Breeze | 1.0 | **2.0** | §2.1.5 Breeze PM_FocusFrameHMargin (outset) |
| Adwaita | 1.0 | **-2.0** | §1.4.3 `outline-offset: -$width` (inset) |
| macOS Sonoma | 1.0 | **-1.0** | §1.1.3 measured (inset) |
| Windows 11 | 1.0 | **0.0** | §2.1.5 Fluent 0px default margin |

1.0 matches no platform. Three of four platforms use a different
polarity (macOS and GNOME use negative/inset, Windows uses zero).

See also: chapter 2 (KDE swap), chapter 22 (Adwaita).

### Options

**A. Set each preset to its correct value**

- Pro: Focus rings render at the correct offset per platform.
- Pro: Inset vs outset is a visible, platform-defining characteristic.
  macOS and GNOME focus rings look distinctly different from KDE/Windows.
- Con: Frameworks must support negative offset values (most do since
  CSS outline-offset is signed).

**B. Leave all at 1.0**

- Con: Wrong for every platform.

### Recommendation

**Option A.** Focus ring offset defines a core visual behavior: whether
the ring sits inside the element (GNOME/macOS) or outside (KDE) or
flush (Windows).

---

# macOS Sonoma preset errors

**Note for chapters 44-57:** Every macOS fix applies to BOTH
`macos-sonoma.toml` AND `macos-sonoma-live.toml` (same values, both
variants).

## 44. macOS preset `radius` is 6.0 -- should be 5.0

**Files:** `macos-sonoma.toml` + `macos-sonoma-live.toml`

**What:** Preset uses `radius = 6.0`. Platform-facts section 1.1.3 says
control corner radius = 5px (measured, confirmed by WebKit
`baseBorderRadius = 5`). Section 2.1.6 confirms macOS radius = 5.

### Options

**A. Change to 5.0** -- Pro: Matches WebKit/measured value. Con: None.

**B. Leave as 6.0** -- Con: 20% too round.

### Recommendation

**Option A.**

---

## 45. macOS preset `disabled_opacity` is 0.5 -- should be ~0.3

**File:** `native-theme/src/presets/macos-sonoma.toml`

**What:** Preset uses `disabled_opacity = 0.5`. Platform-facts section
1.1.3 says `disabledControlTextColor` alpha ≈ 0.25 and overall visual
effect ≈ 0.3. Section 2.1.6 confirms macOS ≈ 0.25-0.3.

### Options

**A. Change to 0.3**

- Pro: Matches the measured overall visual effect.
- Pro: 0.3 is the upper end of the documented range, giving slightly
  more visibility than 0.25.
- Con: macOS has no single global disabled opacity -- the 0.3 is an
  approximation of the visual effect produced by `disabledControlTextColor`.

**B. Change to 0.25**

- Pro: Matches the measured alpha of `disabledControlTextColor`.
- Con: This is only the text alpha; the overall element effect (which
  includes control dimming beyond text) is slightly higher.

**C. Leave as 0.5**

- Con: Nearly 2x the actual opacity. Disabled controls appear much
  more opaque than native macOS.

### Recommendation

**Option A (0.3).** It is the documented upper-bound approximation.
Overshoot is safer than undershoot: too-transparent disabled controls
become invisible, while slightly-too-opaque controls merely look less
dimmed.

---

## 46. macOS preset `focus_ring_width` is 2.0 -- should be 3.0

**File:** `native-theme/src/presets/macos-sonoma.toml`

**What:** Preset uses `focus_ring_width = 2.0`. Platform-facts section
1.1.3 says AppKit focus ring width = 3px, confirmed by WebKit SPI
`UIFocusRingStyle.borderThickness = 3`. Section 2.1.5 confirms macOS = 3.

### Options

**A. Change to 3.0** -- Pro: Matches WebKit/measured. Con: None.

**B. Leave as 2.0** -- Con: Focus ring 33% thinner than native.

### Recommendation

**Option A.**

---

## 47. macOS preset button metrics: 3 values wrong

**File:** `native-theme/src/presets/macos-sonoma.toml` (light.button / dark.button)

**What:**

| Field | Preset | Platform-facts | Source |
|---|---|---|---|
| `padding_horizontal` | 12 | **8** | §1.1.4 / §2.3 WebKit controlPadding(PushButton) = 8 |
| `padding_vertical` | 4 | **3** | §2.3 measured (22-16)/2 = 3 |
| `icon_spacing` | 6 | **4** | §2.3 measured AppKit |

### Options

**A. Fix all three values**

- Pro: Matches platform-facts measurements and WebKit source.
- Con: None.

**B. Leave as-is**

- Con: Buttons 4px wider, 1px taller padding, and 2px wider icon gaps
  than native macOS.

### Recommendation

**Option A.**

---

## 48. macOS preset `input.padding_vertical` is 4 -- should be 3

**File:** `native-theme/src/presets/macos-sonoma.toml`

**What:** Preset has `padding_vertical = 4.0`. Platform-facts section 2.4
says macOS input padding_vertical = 3 (measured: (22-16)/2).

### Options

**A. Change to 3.0** -- Pro: Matches measurement. Con: None.

**B. Leave as 4.0** -- Con: 1px too much.

### Recommendation

**Option A.**

---

## 49. macOS preset menu metrics: 2 values wrong

**File:** `native-theme/src/presets/macos-sonoma.toml` (light.menu / dark.menu)

**What:**

| Field | Preset | Platform-facts | Source |
|---|---|---|---|
| `padding_vertical` | 4 | **3** | §2.6 measured (22-16)/2 |
| `icon_spacing` | 8 | **4** | §2.6 measured AppKit layout |

### Options

**A. Fix both** -- Pro: Matches measurements. Con: None.

**B. Leave as-is** -- Con: Menu items taller and icons further from
text than native.

### Recommendation

**Option A.**

---

## 50. macOS preset slider metrics: 2 values wrong

**File:** `native-theme/src/presets/macos-sonoma.toml` (light.slider / dark.slider)

**What:**

| Field | Preset | Platform-facts | Source |
|---|---|---|---|
| `track_height` | 4 | **5** | §1.1.4 WebKit `sliderTrackWidth = 5` |
| `tick_length` | 4 | **8** | §1.1.4 / §2.9 measured |

### Options

**A. Fix both** -- Pro: Matches WebKit source and measurements.
Con: None.

**B. Leave as-is** -- Con: Track 20% thinner, ticks 50% shorter.

### Recommendation

**Option A.**

---

## 51. macOS preset scrollbar metrics: 2 values wrong

**File:** `native-theme/src/presets/macos-sonoma.toml`

**What:**

| Field | Preset | Platform-facts | Source |
|---|---|---|---|
| `width` | 15 | **16** | §1.1.3 legacy style confirmed by `scrollerWidth` API |
| `min_thumb_height` | 30 | **40** | §2.8 measured legacy mode |

### Options

**A. Fix both** -- Pro: Matches API and measurements. Con: None.

**B. Leave as-is** -- Con: Scrollbar 1px too narrow and allows thumb
10px shorter than native minimum.

### Recommendation

**Option A.**

---

## 52. macOS preset `toolbar.padding` is 4 -- should be 8

**File:** `native-theme/src/presets/macos-sonoma.toml`

**What:** Preset has `padding = 4.0`. Platform-facts section 2.13 says
macOS NSToolbar padding = 8 (measured).

### Options

**A. Change to 8.0** -- Pro: Matches measurement. Con: None.

**B. Leave as 4.0** -- Con: Toolbar content sits 4px closer to edges.

### Recommendation

**Option A.**

---

## 53. macOS preset `list.padding_vertical` is 2 -- should be 4

**File:** `native-theme/src/presets/macos-sonoma.toml`

**What:** Preset has `padding_vertical = 2.0`. Platform-facts section
2.15 says macOS list padding_vertical = 4 (measured: (24-16)/2).

### Options

**A. Change to 4.0** -- Pro: Matches measurement. Con: None.

**B. Leave as 2.0** -- Con: List rows have 2px less vertical space.

### Recommendation

**Option A.**

---

## 54. macOS preset `splitter.width` is 9 -- should be 6

**File:** `native-theme/src/presets/macos-sonoma.toml`

**What:** Preset has `width = 9.0`. Platform-facts section 1.1.4 says
NSSplitView thick divider = 6px (GNUstep source confirms, CocoaDev
confirms). Section 2.17 confirms macOS = 6.

### Options

**A. Change to 6.0** -- Pro: Matches NSSplitView. Con: None.

**B. Leave as 9.0** -- Con: Splitters 50% wider than native. 9px has
no cited source.

### Recommendation

**Option A.**

---

## 55. macOS preset switch metrics: 3 values wrong

**File:** `native-theme/src/presets/macos-sonoma.toml`

**What:**

| Field | Preset | Platform-facts | Source |
|---|---|---|---|
| `track_width` | 40 | **38** | §2.21 / §1.1.4 NSSwitch intrinsic 38×22 |
| `track_height` | 20 | **22** | §1.1.4 NSSwitch intrinsic 38×22 |
| `thumb_size` | 16 | **~18** | §2.21 measured |

NSSwitch intrinsic size is 38×22px per WebKit `RenderThemeMac.mm`.

### Options

**A. Fix all three: 38, 22, 18**

- Pro: Matches WebKit source and measurements.
- Con: None.

**B. Leave as-is**

- Con: Switch track is wrong aspect ratio (40×20 vs 38×22).

### Recommendation

**Option A.**

---

## 56. macOS preset spinner metrics: 2 values wrong

**File:** `native-theme/src/presets/macos-sonoma.toml`

**What:**

| Field | Preset | Platform-facts | Source |
|---|---|---|---|
| `diameter` | 24 | **32** | §2.23 NSProgressIndicator spinning regular = 32 |
| `min_size` | 16 | **10** | §2.23 NSProgressIndicator spinning mini = 10 |

### Options

**A. Fix both: diameter=32, min_size=10**

- Pro: Matches `sizeToFit` with regular/mini control sizes.
- Con: None.

**B. Leave as-is** -- Con: Spinner 25% too small, minimum 60% too large.

### Recommendation

**Option A.**

---

## 57. macOS preset combo_box metrics: 3 values wrong

**File:** `native-theme/src/presets/macos-sonoma.toml`

**What:**

| Field | Preset | Platform-facts | Source |
|---|---|---|---|
| `min_height` | 22 | **21** | §1.1.4 / §2.24 NSPopUpButton regular = 21 (WebKit) |
| `padding_horizontal` | 12 | **~9** | §2.24 ~8-10px measured; midpoint ~9 |
| `arrow_size` | 12 | **~17** | §2.24 ~16-18px measured visible indicator |

### Options

**A. Fix all three: 21, 9, 17**

- Pro: Matches WebKit source and measurements.
- Con: arrow_size is measured with uncertainty (❓ in platform-facts).
  17 is the midpoint of the 16-18 range.

**B. Leave as-is**

- Con: Height off by 1, padding 3px too wide, arrow 5px too small.

### Recommendation

**Option A** with `arrow_size = 17` (midpoint of measured range).

---

# Windows 11 preset errors

**Note for chapters 58-70:** Every Windows fix applies to BOTH
`windows-11.toml` AND `windows-11-live.toml` (same values, both
variants).

## 58. Windows preset `disabled_opacity` is 0.4 -- should be ~0.3

**Files:** `windows-11.toml` + `windows-11-live.toml`

**What:** Preset uses `disabled_opacity = 0.4`. Platform-facts section
1.2.3 says Fluent uses per-control disabled opacities, with
`ListViewItemDisabledThemeOpacity = 0.3`. Section 2.1.6 confirms the
range as approximately 0.3.

### Options

**A. Change to 0.3**

- Pro: Matches the closest documented WinUI3 resource value.
- Con: Fluent has no single global opacity. Different controls use
  different `*Disabled` color brushes. 0.3 is a representative value.

**B. Leave as 0.4**

- Con: 33% more opaque than the documented representative value.
  0.4 has no WinUI3 source (0.55 was the legacy Win8.x value).

### Recommendation

**Option A (0.3).**

---

## 59. Windows preset button metrics: 2 values wrong

**File:** `native-theme/src/presets/windows-11.toml`

**What:**

| Field | Preset | Platform-facts | Source |
|---|---|---|---|
| `min_height` | 32 | **~27** | §1.2.4 effective: 14px text + 5+6 pad + 2 border |
| `padding_horizontal` | 12 | **11** | §1.2.4 ButtonPadding=11,5,11,6 |

`button.padding_vertical = 6.0` is addressed in chapter 71 (model
limitation: the real value is asymmetric 5 top / 6 bottom).

### Options

**A. Fix both: min_height=27, padding_horizontal=11**

- Pro: min_height matches the effective computed height. padding matches
  ButtonPadding first value.
- Con: min_height = 27 is approximate (no explicit WinUI3 MinHeight
  resource). 32 matches `ContentDialogButtonHeight` which is dialog-
  specific, not a general button height.

**B. Keep min_height=32, fix only padding**

- Pro: 32 is a clean WinUI3 resource value (dialog button height).
- Con: Over-sizes non-dialog buttons by 5px.

### Recommendation

**Option A.** General buttons are ~27px effective height. The dialog
button height (32) is for `ContentDialog` only.

---

## 60. Windows preset input padding: asymmetric values cannot be expressed

**File:** `native-theme/src/presets/windows-11.toml`

**What:** Preset has `padding_horizontal = 12.0`, `padding_vertical = 6.0`.
Platform-facts section 1.2.4 documents WinUI3 TextBox
`TextControlThemePadding = 10,5,6,6` (left=10, top=5, right=6, bottom=6).

The horizontal padding is asymmetric: 10px left, 6px right (the right
is smaller to leave room for the clear button). The model's single
`padding_horizontal` field cannot represent this.

### Options

**A. Use the left (larger) value: padding_horizontal=10, padding_vertical=5**

- Pro: Left padding is the user-facing text indent. Right padding is
  visually less important (clear button occupies that space).
- Pro: 10 is the primary padding consumers care about for text alignment.
- Con: Doesn't represent the 6px right padding.

**B. Use the average: padding_horizontal=8, padding_vertical=5.5→6**

- Pro: Splits the difference.
- Con: Neither side is accurate.

**C. Leave as padding_horizontal=12, padding_vertical=6**

- Con: 12 doesn't match either the left (10) or right (6) padding.
  6 matches only the bottom padding (5 top, 6 bottom).

### Recommendation

**Option A (padding_horizontal=10, padding_vertical=5).** Use the left
value for horizontal (the typographically important indent) and the top
value for vertical (the smaller of the asymmetric pair -- frameworks
typically center text, so the smaller value is safer).

See chapter 71 for the broader model limitation discussion.

---

## 61. Windows preset `slider.thumb_size` is 22 -- should be 18

**File:** `native-theme/src/presets/windows-11.toml`

**What:** Preset has `thumb_size = 22.0`. Platform-facts section 1.2.4
says `SliderHorizontalThumbWidth/Height = 18`. Section 2.9 confirms
Windows slider thumb_size = 18.

### Options

**A. Change to 18.0** -- Pro: Matches WinUI3 resource. Con: None.

**B. Leave as 22.0** -- Con: Thumb 22% larger than native. 22 has no
WinUI3 source.

### Recommendation

**Option A.**

---

## 62. Windows preset `progress_bar.height` is 4 -- ambiguous

**File:** `native-theme/src/presets/windows-11.toml`

**What:** Preset has `height = 4.0`. Platform-facts section 1.2.4 says:
- `ProgressBarTrackHeight = 1` (the visual track height)
- `ProgressBarMinHeight = 3` (the control minimum height)

The preset's 4 matches neither value.

### Options

**A. Change to 3 (control minimum)**

- Pro: Matches `ProgressBarMinHeight`, the minimum renderable size.
- Pro: 3px is more practical than 1px for cross-platform rendering.
- Con: Not the visual track height.

**B. Change to 1 (track height)**

- Pro: Matches `ProgressBarTrackHeight`, the actual visual dimension.
- Con: 1px is extremely thin. Frameworks without sub-pixel rendering
  may struggle.

**C. Leave as 4**

- Con: Matches neither documented value.

### Recommendation

**Option A (3.0).** The model field is "bar height" which maps most
naturally to the minimum renderable control height. The 1px track is
a Fluent visual design detail (the track is thin with a thicker
filled section) that is hard to replicate in cross-platform frameworks.

---

## 63. Windows preset menu metrics: 2 values wrong

**File:** `native-theme/src/presets/windows-11.toml`

**What:**

| Field | Preset | Platform-facts | Source |
|---|---|---|---|
| `item_height` | 32 | **~23** | §1.2.4 narrow/mouse mode: 14px text + 4+5 pad ≈ 23 |
| `padding_vertical` | 4 | **8** | §1.2.4 MenuFlyoutItemThemePadding=11,8,11,9 |

The item_height has two WinUI3 modes: touch (~31px) and narrow/mouse
(~23px). The preset's 32 is close to touch mode but matches neither
exactly.

### Options

**A. Use narrow/mouse values: item_height=23, padding_vertical=8**

- Pro: Mouse-mode is the default for desktop applications. Touch mode
  is only active on touch devices.
- Con: Touch-first apps would want the larger value.

**B. Use touch values: item_height=31, padding_vertical=8**

- Pro: More accessible (larger tap targets).
- Con: Wastes space on desktop mouse-driven apps.

**C. Leave as-is**

- Con: item_height=32 matches neither mode. padding_vertical=4 is half
  the documented value.

### Recommendation

**Option A.** Desktop presets should use mouse-mode values. Fix
`padding_vertical` to 8 regardless of item_height choice.

---

## 64. Windows preset scrollbar metrics: 2 values wrong

**File:** `native-theme/src/presets/windows-11.toml`

**What:**

| Field | Preset | Platform-facts | Source |
|---|---|---|---|
| `width` | 16 | **17** | §1.2.3 SM_CXVSCROLL at 96 DPI |
| `min_thumb_height` | 24 | **17** | §1.2.3 SM_CYVTHUMB at 96 DPI |

### Options

**A. Fix both: width=17, min_thumb_height=17**

- Pro: Matches Win32 system metrics at 96 DPI.
- Con: None.

**B. Leave as-is**

- Con: Width off by 1, min thumb 7px too tall.

### Recommendation

**Option A.**

---

## 65. Windows preset toolbar metrics: 2 values wrong

**File:** `native-theme/src/presets/windows-11.toml`

**What:**

| Field | Preset | Platform-facts | Source |
|---|---|---|---|
| `height` | 48 | **64** | §1.2.4 AppBarThemeMinHeight=64 (48 is compact) |
| `item_spacing` | 4 | **0** | §1.2.4 StackPanel has no Spacing |

### Options

**A. Use default height: height=64, item_spacing=0**

- Pro: 64 is the default CommandBar height. 48 is only the compact
  mode (`AppBarThemeCompactHeight`).
- Con: 64px toolbars may feel tall. But the preset should represent
  the default, not the compact mode.

**B. Keep compact height=48, fix only spacing**

- Pro: Compact mode is common in modern apps.
- Con: Not the default per WinUI3 resources.

### Recommendation

**Option A.** The preset should represent the default configuration.
Compact mode is an application-level opt-in, like "wide handle" for
GNOME splitters.

---

## 66. Windows preset `list.item_height` is 36 -- should be 40

**File:** `native-theme/src/presets/windows-11.toml`

**What:** Preset has `item_height = 36.0`. Platform-facts section 1.2.4
says `ListViewItemMinHeight = 40`. Section 2.15 confirms Windows = 40.

### Options

**A. Change to 40.0** -- Pro: Matches WinUI3 resource. Con: None.

**B. Leave as 36.0** -- Con: List rows 4px shorter than native.

### Recommendation

**Option A.**

---

## 67. Windows preset `switch.thumb_size` is 16 -- should be 12

**File:** `native-theme/src/presets/windows-11.toml`

**What:** Preset has `thumb_size = 16.0`. Platform-facts section 1.2.4
says WinUI3 ToggleSwitch thumb = 12px at rest, 14px on hover. Section
2.21 confirms Windows = 12 (rest) / 14 (hover).

The model has a single `thumb_size` field with no hover variant.

### Options

**A. Use the rest-state value: thumb_size=12**

- Pro: Rest state is the default visual appearance.
- Pro: 12px matches the WinUI3 at-rest size.
- Con: On hover, the thumb should grow to 14px. The model cannot
  express this; frameworks would need to add 2px on hover themselves.

**B. Use the hover value: thumb_size=14**

- Pro: Hover size is the more "prominent" visual state.
- Con: Rest state would be 2px too large.

**C. Leave as 16.0**

- Con: Neither rest nor hover state. 16 appears to be copied from a
  different platform's value.

### Recommendation

**Option A (12.0).** The model field represents the default/rest size.
Hover enlargement is an animation detail that the model doesn't
currently capture.

---

## 68. Windows preset `combo_box.arrow_area_width` is 28 -- should be 38

**File:** `native-theme/src/presets/windows-11.toml`

**What:** Preset has `arrow_area_width = 28.0`. Platform-facts section
1.2.4 says WinUI3 ComboBox arrow column = 38px
(`ColumnDefinition Width=38`). Section 2.24 confirms Windows = 38.

### Options

**A. Change to 38.0** -- Pro: Matches WinUI3 XAML. Con: None.

**B. Leave as 28.0** -- Con: Arrow area 10px too narrow. 28 has no
WinUI3 source.

### Recommendation

**Option A.**

---

## 69. Windows preset `icon_sizes.toolbar` is 24 -- should be 20

**File:** `native-theme/src/presets/windows-11.toml`

**What:** Preset has `toolbar = 24.0`. Platform-facts section 1.2.6 says
Fluent `AppBarButton` icon size = 20px. Section 2.1.8 confirms
Windows toolbar icon size = 20.

### Options

**A. Change to 20.0** -- Pro: Matches WinUI3 AppBarButton docs.
Con: None.

**B. Leave as 24.0** -- Con: 20% larger than native. Icon lookups at
24px would miss 20px assets.

### Recommendation

**Option A.**

---

## 70. Windows preset `splitter.width` is 4 -- should be 1

**File:** `native-theme/src/presets/windows-11.toml`

**What:** Preset has `width = 4.0`. Platform-facts section 2.17 says
Fluent SplitView pane border = 1px (WinUI3 has no draggable divider
control).

### Options

**A. Change to 1.0**

- Pro: Matches Fluent SplitView pane border.
- Con: 1px is hard to grab as a splitter handle. But WinUI3 genuinely
  has no traditional draggable divider.

**B. Leave as 4.0**

- Con: No WinUI3 source for 4.

### Recommendation

**Option A.** The preset reflects the platform. If apps need wider
grabable handles, they override.

---

## 71. Model limitation: asymmetric padding cannot be expressed

**Status: design observation, not a fix action**

**What:** Several WinUI3 controls use asymmetric padding that the
model's single `padding_horizontal` / `padding_vertical` fields cannot
represent:

| Widget | WinUI3 Padding (L,T,R,B) | Model field | Best approximation |
|--------|--------------------------|-------------|-------------------|
| Button | 11,5,11,6 | h=11, v=6 | h correct, v=bottom only |
| TextBox | 10,5,6,6 | h=12, v=6 | both wrong (see ch 60) |
| TabView | 8,3,4,3 | h=8, v=4 | h=left only, v off by 1 |
| Tooltip | 9,6,9,8 | h=9, v=8 | h correct, v=bottom only |
| ComboBox | 12,5,0,7 | h=12, v=6 | h=left only, v=average |
| MenuFlyout | 11,8,11,9 | h=11, v=4 | h correct, v=half (wrong, ch 63) |

macOS also has mild asymmetry (measured values that round differently
top/bottom), but it is less pronounced.

### Options

**A. Extend the model with `padding_top` / `padding_bottom` / `padding_left` / `padding_right`**

- Pro: Perfectly expresses all platform values.
- Con: Quadruples the padding field count per widget (2 → 4 or 6).
  Bloats the model. Only Windows regularly needs this.
- Con: Breaking API change.

**B. Add `padding_top` / `padding_bottom` as optional overrides**

- Pro: Only adds fields where needed. Symmetric platforms ignore them.
- Con: Three padding representations (horizontal+vertical, top+bottom,
  all four) is confusing.

**C. Accept the limitation and use best-fit values**

- Pro: No model change. Symmetric padding is good enough for 3 of 4
  platforms.
- Pro: For Windows, the asymmetry is typically 1px (5 vs 6). The visual
  difference is negligible.
- Con: Purists will notice the 1px discrepancy on Windows.

### Recommendation

**Option C** for now. The asymmetry is 1px in most cases. Fixing the
individual preset values to use the best approximation (typically the
larger side or the left/top value) is sufficient. If future connectors
or toolkits need per-side padding, option B can be revisited.

When choosing the best approximation:
- **Horizontal**: use the left value (the typographic indent users see)
- **Vertical**: use the top value (text is visually anchored from top)
