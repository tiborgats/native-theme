# v0.5.5 TODO

Issues found during comprehensive audit (2026-04-04).
Updated with property-registry.toml vs code audit (2026-04-06).

---

## CRITICAL: Missing Interactive State Colors Across All Widgets

The widget model is missing hover, active/pressed, disabled, and focus state
colors that platforms explicitly provide. This forces both connectors to
**derive** these states via hardcoded math (`derive.rs`, `resolve.rs`), which
breaks on edge cases (pure black, low-contrast accents) and removes theme
author control.

`platform-facts.md` documents these platform APIs. The KDE reader could read
`DecorationHover` per color group. Windows provides `COLOR_HOTLIGHT`. macOS
and GNOME provide explicit CSS/API states. None of this data is captured.

### What the connectors currently invent (should be theme fields instead)

**gpui connector** (`connectors/native-theme-gpui/src/`):

| Derivation | File:Line | Formula | Should Be |
|---|---|---|---|
| Hover color (all widgets) | `derive.rs:18` | `bg.blend(base.opacity(0.9))` | Per-widget `hover_background` field |
| Active/pressed color | `derive.rs:34` | `darken(0.1)` light / `darken(0.2)` dark | Per-widget `active_background` field |
| Near-black active safety | `derive.rs:37` | threshold `l < 0.15` → lighten | Unnecessary if color comes from theme |
| Light variant (tinted bg) | `derive.rs:74-82` | `l + 0.15` dark / `blend(0.8)` light | `danger_background`, `success_background` etc. |
| Muted background | `colors.rs:147` | `bg.blend(fg.opacity(0.1))` | `defaults.muted_background` field |
| List active background | `colors.rs:277-278` | `primary.opacity(0.08/0.1)` | `list.active_background` field |
| List active border | `colors.rs:279` | `primary.opacity(0.6)` | `list.active_border` field |
| Overlay alpha | `colors.rs:374-380` | dark=0.5, light=0.4 | `defaults.overlay_opacity` field |
| Accordion hover | `colors.rs:360` | `accent.opacity(0.08)` | Field on accordion/expander widget |
| Group box bg | `colors.rs:365` | `secondary.opacity(0.3/0.4)` | Dedicated field |
| Chart colors 2-5 | `colors.rs:327-345` | Hue offsets from accent | `defaults.chart_2..chart_5` fields |
| Magenta/cyan | `colors.rs:434-447` | Fixed hues 0.833/0.5 | `defaults.magenta`, `defaults.cyan` |
| Drag/drop | `colors.rs:416-417` | opacity 0.65/0.2 | Dedicated fields |
| Status contrast fix | `colors.rs:47-65` | WCAG fallback to white/black | Unnecessary if theme provides correct fg |

**resolve.rs** (`native-theme/src/resolve.rs`):

| Derivation | Line | Formula | Should Be |
|---|---|---|---|
| `scrollbar.thumb_hover` | 480-494 | RGB ±38 shift based on bg luminance | Explicit field from platform/preset |
| `accent_foreground` | 321 | Hardcoded white `#ffffff` | Contrast-aware or explicit in preset |
| `defaults.shadow` | 325 | Hardcoded `rgba(0,0,0,64)` | Explicit in preset |

### Missing fields per widget (from platform-facts.md cross-reference)

Every property below is documented in `platform-facts.md` as available from
at least one platform API, but has no corresponding field in the widget struct.

**§2.3 Button** (`widgets/mod.rs:109-145`)
- [ ] `hover_background` — macOS: computed; Windows: Fluent ControlFillColorSecondary;
  KDE: `[Colors:Button] DecorationHover`; GNOME: `:hover` CSS
- [ ] `hover_foreground`
- [ ] `active_background` — pressed/active state
- [ ] `active_foreground`
- [ ] `disabled_background`
- [ ] `disabled_foreground`

**§2.4 Text Input** (`widgets/mod.rs:149-183`)
- [ ] `hover_border` — focus/hover border color change
- [ ] `focus_border`
- [ ] `disabled_background`
- [ ] `disabled_foreground`

**§2.5 Checkbox / Radio** (`widgets/mod.rs:187-203`)
- [ ] `unchecked_background` — SwitchTheme has both, Checkbox only has checked
- [ ] `unchecked_border`
- [ ] `checked_foreground` — checkmark/indicator color
- [ ] `foreground` — label text color
- [ ] `hover_background`
- [ ] `disabled_background`
- [ ] `disabled_foreground`

**§2.6 Menu** (`widgets/mod.rs:207-231`)
- [ ] `hover_background` — highlighted menu item
- [ ] `hover_foreground`
- [ ] `disabled_foreground`

**§2.7 Tooltip** (`widgets/mod.rs:235-257`)
- [ ] `border` — some platforms have tooltip borders

**§2.8 Scrollbar** (`widgets/mod.rs:261-281`)
- [ ] `thumb_active` — pressed/dragging state (currently `thumb_hover` is computed
  via RGB ±38 shift in `resolve.rs:480-494` instead of being a real field)

**§2.9 Slider** (`widgets/mod.rs:285-303`)
- [ ] `thumb_hover`
- [ ] `disabled_fill`
- [ ] `disabled_track`
- [ ] `disabled_thumb`

**§2.10 Tab Bar** (`widgets/mod.rs:327-351`)
- [ ] `hover_background`
- [ ] `hover_foreground`
- [ ] `border`

**§2.11 Sidebar** (`widgets/mod.rs:355-365`)
- [ ] `hover_background`
- [ ] `selected_background`
- [ ] `selected_foreground`

**§2.12 Toolbar** (`widgets/mod.rs:369-385`)
- [ ] `background`
- [ ] `foreground`
- [ ] `border`

**§2.13 Status Bar** (`widgets/mod.rs:389-397`)
- [ ] `background`
- [ ] `foreground`
- [ ] `border`

**§2.14 List / Table** (`widgets/mod.rs:401-429`)
- [ ] `hover_background`
- [ ] `hover_foreground`
- [ ] `disabled_foreground`

**§2.15 Popover** (`widgets/mod.rs:433-447`)
- [ ] `shadow` — drop shadow presence/color

**§2.16 Splitter** (`widgets/mod.rs:451-459`)
- [ ] `color` — handle color
- [ ] `hover_color`

**§2.17 Switch / Toggle** (`widgets/mod.rs:475-495`)
- [ ] `hover_checked_background`
- [ ] `hover_unchecked_background`
- [ ] `disabled_checked_background`
- [ ] `disabled_unchecked_background`
- [ ] `disabled_thumb`

**§2.22 Dialog** (`widgets/mod.rs:499-527`)
- [ ] `background`
- [ ] `foreground`
- [ ] `border`
- [ ] `shadow`

**§2.23 ComboBox** (`widgets/mod.rs:549-567`)
- [ ] `background`
- [ ] `foreground`
- [ ] `border`
- [ ] `hover_background`
- [ ] `disabled_background`
- [ ] `disabled_foreground`

**§2.24 Segmented Control** (`widgets/mod.rs:571-585`)
- [ ] `background`
- [ ] `foreground`
- [ ] `active_background`
- [ ] `active_foreground`
- [ ] `hover_background`

**§2.25 Card** (`widgets/mod.rs:589-605`)
- [ ] `foreground` — text color on card

**§2.26 Expander** (`widgets/mod.rs:609-623`)
- [ ] `background`
- [ ] `border`
- [ ] `hover_background`
- [ ] `arrow_color`

**§2.27 Link** (`widgets/mod.rs:627-643`)
- [ ] `hover_color` — has `hover_bg` but no hover text color
- [ ] `active_color` — pressed link color
- [ ] `disabled_color`

### Implementation plan

1. Add missing fields to widget structs (all `Option<Rgba>` following existing pattern)
2. Add fields to `platform-facts.md` tables where not yet documented
3. Populate in TOML presets (values from platform specs)
4. Add inheritance rules in `resolve.rs` (fall back to defaults where appropriate)
5. Update KDE reader to extract `DecorationHover` per color group
6. Update connectors to use theme fields directly, remove `derive.rs` computations
7. Update `define_widget_pair!` FIELD_NAMES constants (automatic via macro)

### Geometry and font struct audit result: CLEAN

A property-by-property comparison of `platform-facts.md` against the model
confirms that all **53 defaults properties** (fonts, geometry, spacing, icon
sizes, accessibility) and all **widget geometry/sizing fields** (padding, min/max
dimensions, radius, border_width, etc.) are correctly represented in the structs.
No geometry or font property is missing from the struct definitions.

### Scope note

Not every platform provides every state color. Where a platform has no explicit
value, the preset should still specify a reasonable value (measured or derived
once at authoring time, not at runtime). The goal is: **zero runtime color
computation in connectors**. All derivation happens at preset authoring time or
in `resolve()` fallbacks, where it can be overridden by theme authors.

---

## CRITICAL: resolve.rs Invents Values That Should Come From Data

`resolve.rs` has a "Phase 2: Safety nets" section (lines 315-356) and a
`resolve_text_scale()` function (lines 649-686) that fabricate values out of
thin air. Every property starts as `None`. If the OS reader or TOML preset
doesn't fill it, `validate()` should return an error — NOT silently insert
an invented number. The preset author must be forced to specify every value.

### Invented values that must be removed

**Safety nets** (`resolve.rs:315-356`) — these silently fill None fields with
hardcoded values, hiding incomplete presets instead of erroring:

| Line | Field | Invented Value | Problem |
|------|-------|----------------|---------|
| 318 | `defaults.line_height` | `1.2` | Matches no platform. macOS=1.19, Win=1.43, KDE=1.36, GNOME=1.21 |
| 322 | `defaults.accent_foreground` | `#ffffff` | Fails WCAG on light accents. Must come from preset. |
| 326 | `defaults.shadow` | `rgba(0,0,0,64)` | Alpha 64/255=25%. No platform uses this exact value. |
| 330 | `disabled_foreground` | `← muted` | Copies another field instead of requiring explicit value. |
| 334 | `dialog.button_order` | platform detection | This one is arguably OK (runtime OS detection). |
| 338 | `input.caret` | `← foreground` | Copies another field. Some platforms have explicit caret color. |
| 342 | `scrollbar.track` | `← background` | Copies another field. |
| 346 | `spinner.fill` | `← accent` | Copies another field. |
| 350 | `popover.background` | `← background` | Copies another field. |
| 354 | `list.background` | `← background` | Copies another field. |

**Text scale computation** (`resolve.rs:649-686`) — fabricates font sizes from
ratios that match no platform typography spec:

| Line | Field | Invented Formula | Problem |
|------|-------|-----------------|---------|
| 659 | `text_scale.caption.size` | `font.size * 0.82` | Where does 0.82 come from? macOS=10, Win=12, KDE=10. |
| 667 | `text_scale.section_heading.size` | `font.size * 1.0` | Just copies body size. Not a real heading size. |
| 675 | `text_scale.dialog_title.size` | `font.size * 1.2` | Where does 1.2 come from? macOS=22, Win=28. |
| 683 | `text_scale.display.size` | `font.size * 2.0` | Where does 2.0 come from? Win=68, not 14*2=28. |
| 660 | `text_scale.caption.weight` | `body_weight` (400) | Correct for some, wrong for others. |
| 668 | `text_scale.section_heading.weight` | `700` | Hardcoded bold. |
| 676 | `text_scale.dialog_title.weight` | `700` | Hardcoded bold. |
| 684 | `text_scale.display.weight` | `700` | Hardcoded bold. |
| 57 | `text_scale.*.line_height` | `line_height * size` | Computed. Should be explicit per entry. |

Only 3 of 16 presets specify text_scale (adwaita, windows-11, macos-sonoma).
The other 13 (ALL community presets + kde-breeze, material, ios) silently get
the invented ratios. They need explicit text_scale entries.

For safety nets: all 16 presets already specify `line_height`,
`accent_foreground`, `disabled_foreground`, and `shadow` — the safety nets
are dead code and should be removed without breaking any preset.

**Also in resolve_text_scale** (`resolve.rs:652`):
```rust
let body_weight = defaults_font.weight.unwrap_or(400);
```
This `unwrap_or(400)` invents a font weight if defaults.font.weight is None.

### Same problem in OS readers — text_scale computed from invented ratios

**Windows reader** (`windows.rs:165-188`):
- `caption.size = base * 0.85`, weight 400
- `section_heading.size = base * 1.15`, weight 600
- `dialog_title.size = base * 1.35`, weight 600
- `display.size = base * 1.80`, weight 300
- These values don't come from any Windows API. The `windows-11.toml` preset
  already has the REAL values (12, 20, 28, 68) which override these during merge,
  so the reader computation is both wrong AND dead code.

**KDE reader** (`kde/mod.rs:93-110`):
- `caption`: correctly reads `smallestReadableFont` from kdeglobals (line 83-91)
- `section_heading.size = base * 1.20` — invented (line 96)
- `dialog_title.size = base * 1.35` — invented (line 101)
- `display.size = base * 1.80`, weight 300 — invented (line 106-107)
- KDE has no API for these sizes. Kirigami uses its own hardcoded scale. The
  `kde-breeze.toml` preset should specify these instead.

**macOS reader** (`macos.rs:257-284`):
- Uses Apple HIG reference sizes (11, 15, 22, 34) scaled by system font ratio.
- Comment says "Tries the system API first" but `read_text_scale()` (line 291-293)
  just calls `compute_text_scale(system_size)` — never actually calls
  `NSFont.preferredFontForTextStyle:`.
- The `macos-sonoma.toml` preset already overrides with real values.

### `scrollbar.thumb_hover` in resolve.rs — invented color from math

**File**: `resolve.rs:481-494`
- Computes thumb hover color via RGB ±38 shift based on background luminance.
- Also invents luminance fallback: `map_or(128.0, ...)` at line 485 if background
  is None — fabricates a mid-gray luminance out of nothing.
- Should be an explicit field populated from presets/OS readers.

### `list.alternate_row` inheritance — dead code

**File**: `resolve.rs:548-549`
- Falls back to `list.background` (same color = no striping).
- All 16 presets already specify `alternate_row` explicitly. This fallback never
  triggers. Remove it — a missing `alternate_row` should error.

### What to do

1. **Remove all safety nets** (lines 317-355). Let `validate()` catch the `None`
   fields and report them as errors. Theme authors fix their presets.
2. **Remove text_scale computation** from `resolve.rs` (lines 649-686), `windows.rs`
   (lines 165-188), `kde/mod.rs` (lines 95-110), and `macos.rs` (lines 257-284).
   Require all 4 text_scale entries in every preset TOML. Add them to the 13 presets
   that currently lack them. For the macOS reader, actually call
   `NSFont.preferredFontForTextStyle:` instead of computing.
3. **Remove `unwrap_or(400)`** at line 652. If font weight is None, it should fail.
4. **Remove `scrollbar.thumb_hover` computation** (lines 481-494). Make it an explicit
   field. Populate from platform APIs and presets.
5. **Remove `list.alternate_row` fallback** (line 549). All presets specify it.
6. **Remove `map_or(128.0, ...)` luminance invention** (line 485). Unreachable after
   removing the thumb_hover computation.
7. **Keep widget-from-defaults inheritance** (Phase 3, lines 360-635). Inheriting
   `button.background ← defaults.background` is correct — it's not inventing a
   value, it's applying the documented inheritance rule from `platform-facts.md`.
8. **Keep defaults-internal chains** (Phase 1, lines 296-311). `selection ← accent`
   and `focus_ring_color ← accent` are documented inheritance rules, not inventions.
   But verify each chain is actually documented in `platform-facts.md`.

### Distinction: inheritance vs invention

- **Inheritance** (OK): `button.background ← defaults.background`. The value exists
  in the theme, it's just being propagated to a widget field. Documented in
  `platform-facts.md` as "← defaults.background".
- **Invention** (BUG): `defaults.line_height = 1.2`. The value doesn't exist
  anywhere in the theme data. It was made up by the code.

---

## Correctness

### C-1. `detect_is_dark()` fails silently on non-GNOME/non-KDE Linux
- **File**: `native-theme/src/lib.rs:276-302`
- **Issue**: Shells out to `gsettings` which doesn't exist on bare Wayland (sway,
  Hyprland without GNOME deps). KDE fallback only runs with `feature = "kde"`.
  A minimal Wayland setup always returns `false` (light mode) regardless of actual
  desktop state.
- **Fix**: Check `GTK_THEME` env var (many compositors set it), or read
  `~/.config/gtk-3.0/settings.ini` as additional fallback.

### C-2. iOS platform filter doesn't match on actual iOS builds
- **File**: `native-theme/src/presets.rs:119, 126-146`
- **Issue**: `("ios", &["macos", "ios"])` in `PLATFORM_SPECIFIC` never matches on
  `target_os = "ios"` because `detect_platform()` has no `cfg(target_os = "ios")`
  arm. Falls through to `#[cfg(not(...))]` returning `"linux"`.
- **Fix**: Add `#[cfg(target_os = "ios")] { return "ios"; }` to `detect_platform()`.

### C-3. `#[must_use]` message misleading on `into_resolved()`
- **File**: `native-theme/src/resolve.rs:288`
- **Issue**: Says `"this returns the resolved theme; it does not modify self"` but
  `into_resolved(mut self)` takes self by value and consumes it. The message was
  probably copied from a `&self` method.
- **Fix**: Change to `"this consumes the variant and returns a resolved theme"`.

---

## Widget Model Gaps (Non-State)

Specific per-widget issues not covered by the top-level interactive state audit.

### W-1. Missing font fields on text-bearing widgets
- **Files**: `native-theme/src/model/widgets/mod.rs`
- **Widgets missing font**: CheckboxTheme (line 187, for label text), TabTheme
  (line 327, for tab labels), ListTheme (line 401, for item text).
- **Impact**: These widgets inherit the global default font with no way to override
  per-widget.

### W-2. Doc comment inaccuracies on widget structs
- `CheckboxTheme` (line 188): Says "indicator geometry" but has a color field.
- `SwitchTheme` (line 476): Says "track, thumb, and geometry" but has 3 color fields.
- `DialogTheme` (line 500): Says "title font" but doesn't document the absence of
  color fields (intentional, but surprising and undocumented).

### W-3. `scrollbar.thumb_hover` is the only color computed via math in resolve.rs
- **File**: `native-theme/src/resolve.rs:480-494`
- **Formula**: Shifts each RGB channel by ±38 based on background luminance.
- **Issue**: This is the only color field in the core crate that uses arithmetic
  derivation rather than simple inheritance. Should be an explicit field in
  `ScrollbarTheme`, populated from platform APIs (KDE `DecorationHover`,
  macOS measured, etc.) and TOML presets.

---

## Connector Issues

### K-1. `from_preset()` display name inconsistency between connectors
- **GPUI**: `native-theme-gpui/src/lib.rs:196` uses `name` (the input param, e.g. `"catppuccin-mocha"`).
- **Iced**: `native-theme-iced/src/lib.rs:153,161` uses `spec.name` (e.g. `"Catppuccin Mocha"`).
- **Fix**: Both should use `spec.name` for the human-readable display name.

### K-2. `from_system()` return type inconsistency
- **GPUI**: Returns `(Theme, ResolvedThemeVariant)` (2-tuple).
- **Iced**: Returns `(Theme, ResolvedThemeVariant, bool)` (3-tuple, includes `is_dark`).
- The iced version is more useful. Consider adding `is_dark` to gpui as well.

### K-3. Iced connector lacks contrast enforcement for status colors
- **GPUI**: Has `ensure_status_contrast()` (`native-theme-gpui/src/colors.rs:44-65`)
  that enforces 4.5:1 WCAG ratio for status foreground colors.
- **Iced**: No equivalent. Low-contrast status colors pass through uncorrected.

### K-4. GPUI `ResolvedColors.surface` field is dead code
- **File**: `native-theme-gpui/src/colors.rs:78`
- **Issue**: `#[allow(dead_code)]` on the `surface` field. Computed but never read.
- **Fix**: Use it or remove it.

### K-5. GPUI `from_system()` clones name unnecessarily
- **File**: `native-theme-gpui/src/lib.rs:232`
- **Issue**: `let name = sys.name.clone();` when `sys` is owned and could move.
  Iced does `let name = sys.name;` (direct move).
- **Fix**: `let name = sys.name;` to avoid allocation.

### K-6. Hardcoded opacity values undocumented in gpui connector
- **File**: `native-theme-gpui/src/colors.rs`
- **Values**:
  - Group box: dark=0.3, light=0.4 (line 365)
  - Overlay: dark=0.5, light=0.4 (line 376-379)
  - List active: dark=0.08, light=0.1 (line 277-278)
  - Magenta/cyan saturation cap: 0.85 (lines 435, 444)
- **Issue**: No comments explaining why the values differ between modes.

---

## CI / Publishing

### P-1. Publish workflow doesn't test gpui connector
- **File**: `.github/workflows/publish.yml:26-46`
- **Issue**: CI gate runs clippy and tests for `native-theme`, `native-theme-build`,
  and `native-theme-iced` but NOT `native-theme-gpui`. Yet the publish step
  (line 78) publishes gpui. Buggy gpui connector can reach crates.io.
- **Fix**: Add gpui clippy and test to the `ci` job gate.

### P-2. Publish steps use `continue-on-error: true`
- **File**: `.github/workflows/publish.yml:57,63,73,79`
- **Issue**: If native-theme publishes but a dependent crate fails, the workflow
  "succeeds" silently. Failed publishes go unnoticed.
- **Fix**: Remove `continue-on-error` or add a final verification step.

### P-3. Missing CI feature combination tests
- **File**: `.github/workflows/ci.yml:52-60`
- **Missing combos**: `portal-async-io`, `linux-async-io`, `native-async-io`,
  `kde` alone (always bundled with portal-tokio), `portal` alone (without runtime).
- **Impact**: async-io runtime variants are never tested.

### P-4. Example name collision warning
- **Files**: `connectors/native-theme-gpui/Cargo.toml:50-51`, `connectors/native-theme-iced/Cargo.toml:18-19`
- **Issue**: Both connectors have `examples/showcase.rs` producing
  `target/debug/examples/showcase`. Cargo emits a warning; may become a hard error
  in future Rust versions.
- **Fix**: Rename to `showcase-gpui` / `showcase-iced`.

### P-5. `pre-release.sh` infinite wait loop
- **File**: `scripts/pre-release.sh:120-134`
- **Issue**: Polls CI artifact downloads with no timeout. If the workflow never
  completes, the script hangs forever.
- **Fix**: Add max iteration count.

---

## Design

### D-1. `OnceLock` caching prevents live theme updates
- **Files**: `lib.rs:390` (`prefers_reduced_motion`), `model/icons.rs:479`
  (`system_icon_theme`), `system_is_dark()`
- **Issue**: Cached forever per process. The dual API (cached + uncached) is
  documented, but apps that call the cached API expecting fresh results get stale
  data. No cache invalidation mechanism exists.
- **Note**: Already documented in `docs/todo.md` post-1.0 section. Keep as-is for
  0.x; revisit for 1.0.

### D-2. `is_empty()` inconsistency for `optional_nested` fields
- **File**: `native-theme/src/lib.rs:73`
- **Issue**: `is_empty()` checks `$on_field.is_none()` for optional_nested. This
  means `Some(FontSpec { family: None, size: None, weight: None })` is "not empty"
  even though the inner struct carries no data. The merge logic handles this correctly
  (recursive merge), but `is_empty()` and serialization behavior diverge: a user who
  explicitly sets an empty font gets a different result than one who leaves it `None`.
- **Severity**: Low. Unlikely to surface in practice.

### D-3. `Error` is not `Clone`
- **File**: `native-theme/src/error.rs:74`
- **Issue**: `Platform(Box<dyn Error + Send + Sync>)` and `Io(std::io::Error)` prevent
  `Clone`. Forces the preset cache to store `Result<ThemeSpec, String>` instead of
  `Result<ThemeSpec, Error>` (see `presets.rs:88`).
- **Impact**: Low. Users can't cache error results easily, but this is rare.

### D-4. Large `resolve.rs` file
- **File**: `native-theme/src/resolve.rs` (~3880 lines)
- **Issue**: Contains resolve(), validate(), all 85+ inheritance rules, range checks,
  and lint_toml(). Largest single file in the codebase.
- **Fix**: Consider splitting `resolve()` and `validate()` into separate modules.

### D-5. Magic number text scale ratios
- **File**: `native-theme/src/resolve.rs:655-685`
- **Issue**: Text scale ratios (0.82 for caption, 1.0 for section_heading, 1.2 for
  dialog_title, 2.0 for display) and default weights (400 for caption body weight,
  700 for headings) are passed as inline literals with no named constants.
- **Fix**: Extract to `const CAPTION_SIZE_RATIO: f32 = 0.82;` etc.

---

## Testing Gaps

### T-1. No property-based testing
- For a serialization-heavy library, proptest for TOML round-trips, color parsing,
  merge semantics, and validation bounds would catch edge cases that hand-written
  tests miss.

### T-2. No fuzzing for TOML parsing
- Theme files come from untrusted sources (user-created). While the `toml` crate
  handles parsing safely, post-parse validation (hex colors, numeric ranges) hasn't
  been fuzzed.

### T-3. No code coverage reporting
- Neither tarpaulin nor llvm-cov is configured. Coverage reports would reveal which
  `#[cfg(...)]` branches are actually exercised.

### T-4. Platform readers untested in CI
- The CI matrix tests feature compilation but never exercises actual OS readers
  (`from_kde()`, `from_gnome()`, `from_macos()`, `from_windows()`) because CI runners
  lack those desktop environments.

### T-5. `platform-facts.md` not cross-referenced against presets
- The 1475-line authoritative reference (`docs/platform-facts.md`) is never
  programmatically compared to actual preset TOML values. A test that parses
  platform-facts and spot-checks key values against resolved presets would catch drift.

---

## Documentation

### DOC-1. MSRV 1.94.0 is very aggressive
- Released March 25, 2026 (10 days before this audit). Most Rust ecosystem libraries
  target N-2 or older for broader adoption. Consider whether this limits users.

### DOC-2. No API stability policy
- The crate is pre-1.0 (0.5.4). CHANGELOG documents per-version changes, but there's
  no stability policy or migration guide. `#[non_exhaustive]` is on `Error` but not on
  `IconRole`, `IconSet`, or widget theme structs.

---

## Preset Data

### PRE-1. Identical `muted` color in light/dark variants
- **ios.toml**: `muted = "#8e8e93"` in both light (line 17) and dark (line 215).
- **nord.toml**: `muted = "#4c566a"` in both light (line 17) and dark (line 214).
- Likely intentional (system colors), but worth verifying these remain legible in
  both modes.

### PRE-2. Negative `focus_ring_offset` values
- **adwaita.toml**: `focus_ring_offset = -2.0` (line 44)
- **macos-sonoma.toml**: `focus_ring_offset = -1.0` (line 44)
- Intentional inset styling. Already documented in `resolve.rs:1470-1471`:
  `"focus_ring_offset is intentionally NOT range-checked — negative values mean
  an inset focus ring"`. No action needed.

---

## Dependencies

### DEP-1. Four unmaintained transitive dependencies
- `async-std`, `instant`, `paste`, `rustls-pemfile` (all via gpui ecosystem).
- No security vulnerabilities, but monitor for gpui updates that remove them.

### DEP-2. 944 total packages in lock file
- Heavy for a theme library. Most duplication is from gpui/iced ecosystems (3 versions
  of `ashpd`, 4 versions of `windows`, 2 of `resvg`, 3 of `toml`).
- Not actionable directly, but worth noting for compile-time impact.

---

## Spinner / Animation Safety

### S-1. Freedesktop sprite sheet parsing relies on f64 Infinity behavior
- **File**: `native-theme/src/freedesktop.rs:128`
- **Code**: `let frame_count = (height / width).round() as usize;`
- **Issue**: If a malformed SVG has `viewBox="0 0 0 N"` (width=0), this produces
  `Infinity` (f64 div-by-zero is Infinity, not a panic). The `as usize` saturates
  to `usize::MAX`, and the tolerance check at line 134 catches it. No crash occurs,
  but the correctness relies on coincidental Infinity propagation rather than an
  explicit guard.
- **Fix**: Add `if width <= 0.0 || height <= 0.0 { return None; }` before the division
  for clarity and robustness.

### S-2. SVG `<svg` prefix slice assumes minimum tag length
- **File**: `native-theme/src/spinners.rs:75`
- **Code**: `svg_tag_rest = &svg_tag[4..]` (skips `<svg` prefix)
- **Issue**: If `svg_tag` is shorter than 4 bytes, this panics. Currently unreachable
  because the function is only called with compile-time `include_bytes!()` on genuine
  bundled SVG files (lines 85, 93). Would panic if ever called with arbitrary input.
- **Severity**: Very low (private function, controlled inputs). Add a guard only if
  the function is ever made public.

### S-3. `AnimatedIcon::Frames` allows empty frames vector
- **File**: `native-theme/src/model/animated.rs:59-63`
- **Issue**: No validation prevents `AnimatedIcon::Frames { frames: vec![], ... }`.
  The `first_frame()` method returns `Option` so callers must handle it, but the
  type-level invariant is violated.

### S-4. Zero `frame_duration_ms` not validated
- **File**: `native-theme/src/model/animated.rs:63`
- **Issue**: `frame_duration_ms: 0` could cause division by zero in consumer playback
  code that computes frame indices from elapsed time.

### S-5. `spinners.rs` only handles double-quoted viewBox attributes
- **File**: `native-theme/src/spinners.rs:46`
- **Issue**: Only searches for `viewBox="`. The `freedesktop.rs` parser (line 101-104)
  handles both `viewBox="` and `viewBox='` (single quotes). SVGs with single-quoted
  viewBox would fail silently in spinner generation.

---

## Platform Reader Issues

### R-1. `gsettings` commands have no timeout
- **File**: `native-theme/src/gnome/mod.rs:120-134`
- **Issue**: `Command::new("gsettings").output()` blocks indefinitely. If D-Bus is
  stuck or gsettings hangs, the entire thread blocks. Also applies to dark-mode
  detection in `lib.rs:276` and reduced-motion detection in `lib.rs:415`.
- **Fix**: Consider using `Command::timeout()` (available since Rust 1.82 via
  `std::process::Command` with timeout, or spawn + waitpid with timeout).

### R-2. Freedesktop icon loading reads from disk on every call
- **File**: `native-theme/src/freedesktop.rs:65`
- **Issue**: `std::fs::read(&path)` on every `load_freedesktop_icon()` call with no
  caching. For GUI apps calling this per-icon, this causes repeated disk I/O.
- **Note**: The `system_icon_theme()` function caches the theme name, but individual
  icon files are not cached. Callers should cache, but the doc doesn't mention this.

---

## Low Priority / Cosmetic

- [ ] `from_toml()` wrapper in `presets.rs:168-171` is trivial (two-line pass-through
  to `toml::from_str`). Exists for symmetry with `from_file` / `to_toml`. Fine as-is.
- [ ] `Rgba` stores u8 but all toolkit work is f32. Quantization is documented
  (`color.rs:77-78`) and visual impact is negligible. Not worth changing.
- [ ] `active_color()` fails on pure black `l=0.0` (documented in test at
  `derive.rs:260-270`). Acceptable since pure black rarely appears as a control bg.
- [ ] `ThemeSpec::merge()` keeps the base name (documented + tested). Could be
  surprising but changing it would be a semantic break.

---

## Spec-vs-Implementation Audit (2026-04-05)

Cross-reference of `docs/property-registry.toml` and `docs/inheritance-rules.toml`
against the Rust structs and `resolve.rs`. Four mismatches found.

### SPEC-1. `defaults.spacing` — REMOVE, replace with `layout` widget — MAJOR

**Problem**: The registry originally listed 4 semantic layout names
(`widget_gap`, `container_margin`, `window_margin`, `section_gap`)
under `[defaults]` spacing. The code has a 7-tier T-shirt scale
(`xxs`..`xxl`). Neither is correct in `[defaults]` — they are two
different mistakes:

- The **4 semantic names** are layout container defaults (§2.20) —
  they belong on a `layout` widget, not in `defaults`.
- The **T-shirt scale** is not a platform theme property at all. Every
  platform spacing value is already captured as a direct per-widget
  field (`button.padding_horizontal`, `dialog.content_padding`,
  `toolbar.item_spacing`, `menu.icon_spacing`, etc.) across §2.3–2.28.
  The T-shirt scale duplicates these as an abstract ramp that no
  platform provides or uses.

**Fix** (docs — applied):
- `property-registry.toml`: removed T-shirt scale entries from
  `[defaults]`. Added `[layout]` widget with 4 semantic fields. Added
  note that `defaults.spacing` should be removed from code.
- `platform-facts.md` §2.20: renamed to "Layout Container Defaults"
  with explanatory text.

**Fix** (code — pending):
- Remove `ThemeSpacing` struct (`spacing.rs`), `spacing` field from
  `ThemeDefaults`, `ResolvedThemeSpacing` from `resolved.rs`.
- Remove `[light.defaults.spacing]` / `[dark.defaults.spacing]` from
  all 21 presets.
- Remove `spacing()` helpers from both connectors.
- Update showcase to use per-widget spacing fields and layout container
  values instead of the T-shirt scale.
- Add `Layout` struct to `native-theme/src/model/widgets/`.
- Add `layout` field to `ThemeVariant`.
- Populate from platform presets (values from §2.20 table).
- KDE reader: populate `widget_gap` and `window_margin` from the
  `breezemetrics.h` constants already in `kde/metrics.rs`.
- No inheritance rules needed — values come from preset/reader only.

### SPEC-2. `scrollbar.thumb_hover` — code does more than spec says

**Spec** (`inheritance-rules.toml:89`):
`"scrollbar.thumb_hover" = "defaults.muted"` — plain copy.

**Code** (`resolve.rs:484-495`): Derives from `scrollbar.thumb` (not
`defaults.muted` directly) with a ~15% brightness shift computed from
background luminance. This fabricates a computed color rather than
applying a simple inheritance rule.

Already tracked in the "Invented Values" section above
(`resolve.rs:480-494`). The `inheritance-rules.toml` entry should be
updated to reflect what the code actually does:

**Fix**: In `inheritance-rules.toml`, change line 89 to:
```toml
"scrollbar.thumb_hover" = "COMPUTED: scrollbar.thumb ± 15% brightness"
```
And move it to `[wrong_safety_nets]` or `[hardcoded_safety_nets]` since
it invents a value rather than inheriting.

### SPEC-3. `switch.unchecked_background` — code contradicts no_inheritance (KNOWN)

**Spec** (`inheritance-rules.toml:214`): Listed in `[no_inheritance]`.
**Code** (`resolve.rs:587-588`): `self.switch.unchecked_background = d.muted`.

Already self-documented as a BUG in `inheritance-rules.toml:215-216`.
The code should remove this inheritance line. All 16 presets already
specify `unchecked_background` explicitly.

**Fix**: Remove `resolve.rs:587-588`. The field stays in `[no_inheritance]`.

### SPEC-4. `text_scale` weight inheritance — spec inaccurate for headings

**Spec** (`inheritance-rules.toml:329`):
`"weight.inherits_from" = "defaults.font.weight"` — applies to ALL entries.

**Code** (`resolve.rs:652-685`):
- `caption`: weight fallback = `defaults.font.weight.unwrap_or(400)` — matches spec.
- `section_heading`: weight fallback = hardcoded `700` — contradicts spec.
- `dialog_title`: weight fallback = hardcoded `700` — contradicts spec.
- `display`: weight fallback = hardcoded `700` — contradicts spec.

The code behavior (bold headings) is correct design intent. The spec
is inaccurate — it oversimplifies by claiming all entries inherit from
`defaults.font.weight`.

**Fix**: Update `inheritance-rules.toml` [text_scale_inheritance] to:
```toml
"weight.caption_inherits_from"   = "defaults.font.weight"
"weight.heading_default"         = "700 (hardcoded bold for section_heading, dialog_title, display)"
```

---

## Spacing Design Analysis

### Why `defaults.spacing` (T-shirt scale) must be removed

The original registry conflated two things under `[defaults]` spacing:
4 semantic layout names and a 7-tier abstract ramp. Neither belongs
there. The 4 semantic names are layout container properties (§2.20).
The 7-tier ramp is not a platform property at all.

Every platform spacing value is already a direct per-widget field:

- **Per-widget spacing** (§2.3–2.28): `button.padding_horizontal`,
  `button.icon_spacing`, `checkbox.spacing`, `input.padding_horizontal`,
  `menu.padding_horizontal`, `menu.icon_spacing`, `tooltip.padding_horizontal`,
  `tab.padding_horizontal`, `toolbar.item_spacing`, `toolbar.padding`,
  `list.padding_horizontal`, `dialog.content_padding`,
  `dialog.button_spacing`, `combo_box.padding_horizontal`,
  `expander.content_padding`, `card.padding`,
  `segmented_control.padding_horizontal`, etc.

- **Layout container defaults** (§2.20): `layout.widget_gap`,
  `layout.container_margin`, `layout.window_margin`, `layout.section_gap`.

The T-shirt scale (`xxs`..`xxl`) duplicates these as an abstract ramp
that no platform provides. The WinUI3 Fluent token ramp (§1.2.5) might
look similar, but it is just a value palette — individual control
templates pick specific values (often off-ramp: 11px, 9px, 3px), and
those per-control values are already captured in the per-widget fields.

### What the Fluent spacing tokens are for

The Fluent 2 spacing tokens (`sizeNone`..`size320`, exported as
`spacingHorizontalXXS` etc.) are a **design guideline ramp** — a menu
of available values for WinUI3 control template authors. They are not
a system API, not user-configurable, and not exposed at runtime.

Their semantic usage is visible in the individual control templates:
`ButtonPadding=11,5,11,6`, `ContentDialogButtonSpacing=8`,
`CheckBoxPadding` first value=8, `MenuFlyoutItemThemePadding=11,8,11,9`,
etc. All of these are already captured as per-widget fields in §2.3–2.28.

No other platform has an abstract spacing scale:
- **macOS**: HIG gives per-context values (8pt, 20pt). No ramp.
- **KDE**: `breezemetrics.h` has per-context constants. No ramp.
- **GNOME**: Per-widget CSS values only. No ramp.

### Layout container defaults (§2.20) — not user-configurable

| Property | macOS HIG | Windows | KDE Breeze | GNOME |
|---|---|---|---|---|
| `widget_gap` | 8 | **(none)** | `Layout_DefaultSpacing`=6 | 6 |
| `container_margin` | **(none)** | **(none)** | `Layout_ChildMarginWidth`=6 | 12 |
| `window_margin` | 20 | **(none)** | `Layout_TopLevelMarginWidth`=10 | 12 |
| `section_gap` | 20 | **(none)** | **(none)** | 18 |

These are compile-time constants (KDE), design guidelines (macOS), or
hardcoded CSS (GNOME). Windows has none — `StackPanel.Spacing` defaults
to 0. Not user-configurable on any platform. They belong on a `layout`
widget, same pattern as `dialog.content_padding` or `toolbar.item_spacing`.


---

## CRITICAL: property-registry.toml vs Code Naming / Structure Audit

Comprehensive field-by-field comparison of every entry in
`docs/property-registry.toml` against the actual Rust struct definitions in
`native-theme/src/model/`. The registry is the source of truth for the v0.5.5
naming convention overhaul; the code must be updated to match.

### Summary

- **3 structural issues** (Font missing fields, no Border sub-struct, enum mismatch)
- **~70 field naming mismatches** (systematic rename patterns, listed per widget)
- **~30 missing fields** (in registry but not in code, beyond interactive states)
- **~15 extra fields** (in code but not in registry)

### REG-1. FontSpec missing `style` and `color` fields

**Registry** (`_structures.Font`):
```
family = "string"
size   = "f32"
weight = "u16"
style  = "enum"   # Normal | Italic | Oblique
color  = "color"
```

**Code** (`model/font.rs:12-19`):
```rust
pub struct FontSpec {
    pub family: Option<String>,
    pub size: Option<f32>,
    pub weight: Option<u16>,
    // MISSING: style, color
}
```

**Impact**: `style` is never captured from any platform API (macOS NSFont italic
trait, Windows LOGFONT.lfItalic, KDE font style, GNOME Pango style). `color`
being on FontSpec means each widget's default text color lives inside its font
sub-struct (e.g., `button.font.color`), rather than as a separate `foreground`
field. This affects the TOML format for every widget.

**Action**:
- [ ] Add `FontStyle` enum: `Normal | Italic | Oblique`
- [ ] Add `style: Option<FontStyle>` to `FontSpec`
- [ ] Add `color: Option<Rgba>` to `FontSpec`
- [ ] Migrate per-widget `foreground` fields into `font.color`
- [ ] Update `ResolvedFontSpec` with corresponding non-optional fields
- [ ] Update all presets and OS readers

### REG-2. Border sub-struct does not exist — fields are flattened

**Registry** (`_structures.Border`):
```
color              = "color"
corner_radius      = "f32"
corner_radius_lg   = "f32"    # defaults only
line_width         = "f32"
opacity            = "f32"    # defaults only
shadow_enabled     = "bool"
padding_horizontal = "f32"
padding_vertical   = "f32"
```

**Code**: No `Border` struct exists. Each widget flattens these fields at the
parent level with different names:
- `border.color` → widget has `border: Option<Rgba>` (flat)
- `border.corner_radius` → widget has `radius: Option<f32>` (flat)
- `border.line_width` → `frame_width` (defaults) or `border_width` (input/checkbox)
- `border.opacity` → `border_opacity` (defaults only)
- `border.shadow_enabled` → `shadow_enabled` (defaults) or `shadow` (widgets)
- `border.padding_horizontal` → `padding_horizontal` (flat on widget, not under border)
- `border.padding_vertical` → `padding_vertical` (flat on widget, not under border)

**TOML format change**: Creating a `Border` sub-struct changes serialization from:
```toml
[button]
border = "#cccccc"
radius = 6.0
```
to:
```toml
[button.border]
color = "#cccccc"
corner_radius = 6.0
```

**Action**:
- [ ] Create `BorderSpec` struct with all 8 fields as `Option<T>`
- [ ] Create `ResolvedBorderSpec` with non-optional fields
- [ ] Replace flat border fields on every widget with `border: BorderSpec`
- [ ] Update the `define_widget_pair!` macro to support nested border
- [ ] Update all presets (every TOML file) to use `[widget.border]` sections
- [ ] Update OS readers and resolve.rs

### REG-3. DialogButtonOrder enum variant naming and cardinality mismatch

**Registry**: `button_order = "enum"  # PrimaryRight | PrimaryLeft | KdeLayout | GnomeLayout`

**Code** (`model/dialog_order.rs:14-20`):
```rust
pub enum DialogButtonOrder {
    TrailingAffirmative,   // registry: PrimaryRight
    LeadingAffirmative,    // registry: PrimaryLeft
    // MISSING: KdeLayout, GnomeLayout
}
```

The registry specifies 4 variants but code has only 2. The missing variants
(`KdeLayout`, `GnomeLayout`) may represent more nuanced button placement
patterns beyond simple left/right affirmative positioning.

**Action**:
- [ ] Decide: are 4 variants needed or are 2 sufficient? Check platform-facts.md
- [ ] Rename variants to match registry names (or update registry)
- [ ] Add `#[serde(rename = "...")]` for backwards compatibility if needed

---

### REG-4. Per-widget field audit

Each table below compares registry fields against code. Status key:

- OK = correct name and type
- **RENAME** = field exists under a different name
- **MISSING** = field not in code at all
- **EXTRA** = field in code but not in registry

#### §2.1 Global Defaults — `ThemeDefaults` (`model/defaults.rs:34-134`)

| Registry Field | Code Field | Status |
|---|---|---|
| `font` | `font: FontSpec` | OK (but FontSpec missing style/color, see REG-1) |
| `line_height` | `line_height` | OK |
| `mono_font` | `mono_font: FontSpec` | OK (but FontSpec missing style/color) |
| `background_color` | `background` | **RENAME** → `background_color` |
| `text_color` | `foreground` | **RENAME** → `text_color` |
| `accent_color` | `accent` | **RENAME** → `accent_color` |
| `accent_text_color` | `accent_foreground` | **RENAME** → `accent_text_color` |
| `surface_color` | `surface` | **RENAME** → `surface_color` |
| `muted_color` | `muted` | **RENAME** → `muted_color` |
| `shadow_color` | `shadow` | **RENAME** → `shadow_color` |
| `link_color` | `link` | **RENAME** → `link_color` |
| `selection_background` | `selection` | **RENAME** → `selection_background` |
| `selection_text_color` | `selection_foreground` | **RENAME** → `selection_text_color` |
| `selection_inactive_background` | `selection_inactive` | **RENAME** → `selection_inactive_background` |
| `text_selection_background` | — | **MISSING** (macOS has distinct text selection) |
| `text_selection_color` | — | **MISSING** (macOS `selectedTextColor`) |
| `disabled_text_color` | `disabled_foreground` | **RENAME** → `disabled_text_color` |
| `danger_color` | `danger` | **RENAME** → `danger_color` |
| `danger_text_color` | `danger_foreground` | **RENAME** → `danger_text_color` |
| `warning_color` | `warning` | **RENAME** → `warning_color` |
| `warning_text_color` | `warning_foreground` | **RENAME** → `warning_text_color` |
| `success_color` | `success` | **RENAME** → `success_color` |
| `success_text_color` | `success_foreground` | **RENAME** → `success_text_color` |
| `info_color` | `info` | **RENAME** → `info_color` |
| `info_text_color` | `info_foreground` | **RENAME** → `info_text_color` |
| `focus_ring_color` | `focus_ring_color` | OK |
| `focus_ring_width` | `focus_ring_width` | OK |
| `focus_ring_offset` | `focus_ring_offset` | OK |
| `border` (sub-struct) | flattened fields | **STRUCTURAL** (see REG-2) |
| `border.color` | `border: Option<Rgba>` | **RENAME** → `border.color` |
| `border.corner_radius` | `radius` | **RENAME** → `border.corner_radius` |
| `border.corner_radius_lg` | `radius_lg` | **RENAME** → `border.corner_radius_lg` |
| `border.line_width` | `frame_width` | **RENAME** → `border.line_width` |
| `border.opacity` | `border_opacity` | **RENAME** → `border.opacity` |
| `border.shadow_enabled` | `shadow_enabled` | **RENAME** → `border.shadow_enabled` |
| `border.padding_horizontal` | — | **MISSING** (no padding on defaults) |
| `border.padding_vertical` | — | **MISSING** (no padding on defaults) |
| `disabled_opacity` | `disabled_opacity` | OK |
| `text_scaling_factor` | `text_scaling_factor` | OK |
| `reduce_motion` | `reduce_motion` | OK |
| `high_contrast` | `high_contrast` | OK |
| `reduce_transparency` | `reduce_transparency` | OK |
| `icon_sizes` | `icon_sizes: IconSizes` | OK |
| — | `spacing: ThemeSpacing` | **EXTRA** (registry NOTE says should be removed) |

#### §2.19 Text Scale — `TextScale` (`model/font.rs:77-86`)

| Registry Field | Code Field | Status |
|---|---|---|
| `caption` | `caption` | OK |
| `section_heading` | `section_heading` | OK |
| `dialog_title` | `dialog_title` | OK |
| `display` | `display` | OK |

TextScaleEntry: registry has `size`, `weight`. Code adds `line_height` (noted
in registry as extra, acceptable).

#### Top-level `ThemeVariant` fields

| Registry Field | Code Field | Status |
|---|---|---|
| `icon_set` | `icon_set` | OK |
| `icon_theme` | `icon_theme` | OK |

#### §2.2 Window — `WindowTheme` (`widgets/mod.rs:77-105`)

| Registry Field | Code Field | Status |
|---|---|---|
| `background_color` | `background` | **RENAME** → `background_color` |
| `title_bar_background` | `title_bar_background` | OK |
| `title_bar_font` | `title_bar_font` | OK |
| `inactive_title_bar_background` | `inactive_title_bar_background` | OK |
| `inactive_title_bar_text_color` | `inactive_title_bar_foreground` | **RENAME** → `inactive_title_bar_text_color` |
| `border` (sub-struct) | flattened | **STRUCTURAL** |
| `border.color` | `border: Rgba` | **RENAME** → nest under border |
| `border.corner_radius` | `radius` | **RENAME** → nest under border |
| `border.shadow_enabled` | `shadow` | **RENAME** → nest under border |
| — | `foreground` | **EXTRA** (not in registry) |
| — | `title_bar_foreground` | **EXTRA** (not in registry — active title bar text) |

**Note**: The registry has no active title bar text color field. But `title_bar_font`
in the registry includes `color` (via Font sub-struct), which would serve this
purpose. Check platform-facts.md §2.2 to confirm whether `title_bar_foreground`
should be in the registry or is handled by `title_bar_font.color`.

#### §2.3 Button — `ButtonTheme` (`widgets/mod.rs:109-145`)

| Registry Field | Code Field | Status |
|---|---|---|
| `background_color` | `background` | **RENAME** → `background_color` |
| `font` | `font: FontSpec` | OK (but FontSpec missing style/color) |
| `min_width` | `min_width` | OK |
| `min_height` | `min_height` | OK |
| `icon_text_gap` | `icon_spacing` | **RENAME** → `icon_text_gap` |
| `primary_background` | `primary_background` | OK |
| `primary_text_color` | `primary_foreground` | **RENAME** → `primary_text_color` |
| `disabled_opacity` | `disabled_opacity` | OK |
| `hover_background` | — | **MISSING** (tracked above in interactive states) |
| `hover_text_color` | — | **MISSING** (tracked above) |
| `border` (sub-struct) | flattened | **STRUCTURAL** |
| `border.color` | `border: Rgba` | **RENAME** → nest |
| `border.corner_radius` | `radius` | **RENAME** → nest |
| `border.shadow_enabled` | `shadow` | **RENAME** → nest |
| `border.padding_horizontal` | `padding_horizontal` | Name matches, needs nesting |
| `border.padding_vertical` | `padding_vertical` | Name matches, needs nesting |
| — | `foreground` | **EXTRA** (replaced by `font.color`) |
| — | `border.line_width` | **MISSING** (no line_width on button border) |

#### §2.4 Text Input — `InputTheme` (`widgets/mod.rs:149-183`)

| Registry Field | Code Field | Status |
|---|---|---|
| `background_color` | `background` | **RENAME** |
| `placeholder_color` | `placeholder` | **RENAME** → `placeholder_color` |
| `caret_color` | `caret` | **RENAME** → `caret_color` |
| `selection_background` | `selection` | **RENAME** → `selection_background` |
| `selection_text_color` | `selection_foreground` | **RENAME** → `selection_text_color` |
| `font` | `font: FontSpec` | OK |
| `min_height` | `min_height` | OK |
| `disabled_opacity` | — | **MISSING** |
| `border` (sub-struct) | flattened | **STRUCTURAL** |
| `border.color` | `border: Rgba` | **RENAME** → nest |
| `border.corner_radius` | `radius` | **RENAME** → nest |
| `border.line_width` | `border_width` | **RENAME** → `border.line_width` |
| `border.padding_horizontal` | `padding_horizontal` | Name matches, needs nesting |
| `border.padding_vertical` | `padding_vertical` | Name matches, needs nesting |
| — | `foreground` | **EXTRA** (replaced by `font.color`) |

#### §2.5 Checkbox / Radio — `CheckboxTheme` (`widgets/mod.rs:187-203`)

| Registry Field | Code Field | Status |
|---|---|---|
| `background_color` | — | **MISSING** (unchecked bg) |
| `font` | — | **MISSING** (label font, tracked in W-1) |
| `indicator_color` | — | **MISSING** (checkmark/dot color) |
| `indicator_width` | `indicator_size` | **RENAME** → `indicator_width` |
| `label_gap` | `spacing` | **RENAME** → `label_gap` |
| `checked_background` | `checked_background` | OK |
| `disabled_opacity` | — | **MISSING** |
| `border` (sub-struct) | partial flat fields | **STRUCTURAL** |
| `border.corner_radius` | `radius` | **RENAME** → nest |
| `border.line_width` | `border_width` | **RENAME** → nest |
| `border.color` | — | **MISSING** |

#### §2.6 Menu — `MenuTheme` (`widgets/mod.rs:207-231`)

| Registry Field | Code Field | Status |
|---|---|---|
| `background_color` | `background` | **RENAME** |
| `separator_color` | `separator` | **RENAME** → `separator_color` |
| `font` | `font: FontSpec` | OK |
| `row_height` | `item_height` | **RENAME** → `row_height` |
| `icon_text_gap` | `icon_spacing` | **RENAME** → `icon_text_gap` |
| `icon_size` | — | **MISSING** |
| `hover_background` | — | **MISSING** (tracked above) |
| `hover_text_color` | — | **MISSING** (tracked above) |
| `disabled_text_color` | — | **MISSING** (tracked above) |
| `border` (sub-struct) | partial flat fields | **STRUCTURAL** |
| `border.padding_horizontal` | `padding_horizontal` | Name matches, needs nesting |
| `border.padding_vertical` | `padding_vertical` | Name matches, needs nesting |
| `border.color` | — | **MISSING** |
| `border.corner_radius` | — | **MISSING** |
| — | `foreground` | **EXTRA** (replaced by `font.color`) |

#### §2.7 Tooltip — `TooltipTheme` (`widgets/mod.rs:235-257`)

| Registry Field | Code Field | Status |
|---|---|---|
| `background_color` | `background` | **RENAME** |
| `font` | `font: FontSpec` | OK |
| `max_width` | `max_width` | OK |
| `border` (sub-struct) | partial flat fields | **STRUCTURAL** |
| `border.corner_radius` | `radius` | **RENAME** → nest |
| `border.padding_horizontal` | `padding_horizontal` | Name matches, needs nesting |
| `border.padding_vertical` | `padding_vertical` | Name matches, needs nesting |
| `border.color` | — | **MISSING** (tracked above) |
| — | `foreground` | **EXTRA** (replaced by `font.color`) |

#### §2.8 Scrollbar — `ScrollbarTheme` (`widgets/mod.rs:261-281`)

| Registry Field | Code Field | Status |
|---|---|---|
| `track_color` | `track` | **RENAME** → `track_color` |
| `thumb_color` | `thumb` | **RENAME** → `thumb_color` |
| `thumb_hover_color` | `thumb_hover` | **RENAME** → `thumb_hover_color` |
| `groove_width` | `width` | **RENAME** → `groove_width` |
| `min_thumb_length` | `min_thumb_height` | **RENAME** → `min_thumb_length` |
| `thumb_width` | `slider_width` | **RENAME** → `thumb_width` |
| `overlay_mode` | `overlay_mode` | OK |

#### §2.9 Slider — `SliderTheme` (`widgets/mod.rs:285-303`)

| Registry Field | Code Field | Status |
|---|---|---|
| `fill_color` | `fill` | **RENAME** → `fill_color` |
| `track_color` | `track` | **RENAME** → `track_color` |
| `thumb_color` | `thumb` | **RENAME** → `thumb_color` |
| `track_height` | `track_height` | OK |
| `thumb_diameter` | `thumb_size` | **RENAME** → `thumb_diameter` |
| `tick_mark_length` | `tick_length` | **RENAME** → `tick_mark_length` |
| `disabled_opacity` | — | **MISSING** |

#### §2.10 Progress Bar — `ProgressBarTheme` (`widgets/mod.rs:307-323`)

| Registry Field | Code Field | Status |
|---|---|---|
| `fill_color` | `fill` | **RENAME** → `fill_color` |
| `track_color` | `track` | **RENAME** → `track_color` |
| `track_height` | `height` | **RENAME** → `track_height` |
| `min_width` | `min_width` | OK |
| `border` (sub-struct) | partial flat fields | **STRUCTURAL** |
| `border.corner_radius` | `radius` | **RENAME** → nest |
| `border.color` | — | **MISSING** |

#### §2.11 Tab Bar — `TabTheme` (`widgets/mod.rs:327-351`)

| Registry Field | Code Field | Status |
|---|---|---|
| `background_color` | `background` | **RENAME** |
| `active_background` | `active_background` | OK |
| `active_text_color` | `active_foreground` | **RENAME** → `active_text_color` |
| `bar_background` | `bar_background` | OK |
| `min_width` | `min_width` | OK |
| `min_height` | `min_height` | OK |
| `font` | — | **MISSING** (tracked in W-1) |
| `border` (sub-struct) | partial flat fields | **STRUCTURAL** |
| `border.padding_horizontal` | `padding_horizontal` | Name matches, needs nesting |
| `border.padding_vertical` | `padding_vertical` | Name matches, needs nesting |
| `border.color` | — | **MISSING** |
| — | `foreground` | **EXTRA** (replaced by `font.color`) |

#### §2.12 Sidebar — `SidebarTheme` (`widgets/mod.rs:355-365`)

| Registry Field | Code Field | Status |
|---|---|---|
| `background_color` | `background` | **RENAME** |
| `font` | — | **MISSING** |
| `selection_background` | — | **MISSING** (tracked above) |
| `selection_text_color` | — | **MISSING** (tracked above) |
| `hover_background` | — | **MISSING** (tracked above) |
| `border` (sub-struct) | — | **MISSING** entirely |
| — | `foreground` | **EXTRA** (replaced by `font.color`) |

#### §2.13 Toolbar — `ToolbarTheme` (`widgets/mod.rs:369-385`)

| Registry Field | Code Field | Status |
|---|---|---|
| `font` | `font: FontSpec` | OK |
| `bar_height` | `height` | **RENAME** → `bar_height` |
| `item_gap` | `item_spacing` | **RENAME** → `item_gap` |
| `background_color` | — | **MISSING** (tracked above) |
| `icon_size` | — | **MISSING** |
| `border` (sub-struct) | — | **MISSING** entirely (tracked above) |
| — | `padding` | **EXTRA** (not in registry) |

#### §2.14 Status Bar — `StatusBarTheme` (`widgets/mod.rs:389-397`)

| Registry Field | Code Field | Status |
|---|---|---|
| `font` | `font: FontSpec` | OK |
| `background_color` | — | **MISSING** (tracked above) |
| `border` (sub-struct) | — | **MISSING** entirely (tracked above) |

#### §2.15 List / Table — `ListTheme` (`widgets/mod.rs:401-429`)

| Registry Field | Code Field | Status |
|---|---|---|
| `background_color` | `background` | **RENAME** |
| `item_font` | — | **MISSING** (tracked in W-1) |
| `alternate_row_background` | `alternate_row` | **RENAME** → `alternate_row_background` |
| `selection_background` | `selection` | **RENAME** → `selection_background` |
| `selection_text_color` | `selection_foreground` | **RENAME** → `selection_text_color` |
| `header_font` | — | **MISSING** |
| `header_background` | `header_background` | OK |
| `grid_color` | `grid_color` | OK |
| `row_height` | `item_height` | **RENAME** → `row_height` |
| `hover_background` | — | **MISSING** (tracked above) |
| `border` (sub-struct) | partial flat fields | **STRUCTURAL** |
| `border.padding_horizontal` | `padding_horizontal` | Name matches, needs nesting |
| `border.padding_vertical` | `padding_vertical` | Name matches, needs nesting |
| `border.color` | — | **MISSING** |
| — | `foreground` | **EXTRA** (replaced by `item_font.color`) |
| — | `header_foreground` | **EXTRA** (replaced by `header_font.color`) |

#### §2.16 Popover / Dropdown — `PopoverTheme` (`widgets/mod.rs:433-447`)

| Registry Field | Code Field | Status |
|---|---|---|
| `background_color` | `background` | **RENAME** |
| `font` | — | **MISSING** |
| `border` (sub-struct) | partial flat fields | **STRUCTURAL** |
| `border.color` | `border: Rgba` | **RENAME** → nest |
| `border.corner_radius` | `radius` | **RENAME** → nest |
| — | `foreground` | **EXTRA** (replaced by `font.color`) |

#### §2.17 Splitter — `SplitterTheme` (`widgets/mod.rs:451-459`)

| Registry Field | Code Field | Status |
|---|---|---|
| `divider_width` | `width` | **RENAME** → `divider_width` |
| `divider_color` | — | **MISSING** (tracked above as `color`) |

#### §2.18 Separator — `SeparatorTheme` (`widgets/mod.rs:463-471`)

| Registry Field | Code Field | Status |
|---|---|---|
| `line_color` | `color` | **RENAME** → `line_color` |
| `line_width` | — | **MISSING** |

#### §2.20 Layout Container Defaults — NOT YET IMPLEMENTED

Registry notes: "NOT YET IMPLEMENTED — struct and presets do not exist yet."
No code exists. Needs new `LayoutDefaults` struct with `widget_gap`,
`container_margin`, `window_margin`, `section_gap`.

#### §2.21 Switch / Toggle — `SwitchTheme` (`widgets/mod.rs:475-495`)

| Registry Field | Code Field | Status |
|---|---|---|
| `track_width` | `track_width` | OK |
| `track_height` | `track_height` | OK |
| `thumb_diameter` | `thumb_size` | **RENAME** → `thumb_diameter` |
| `track_radius` | `track_radius` | OK |
| `checked_background` | `checked_background` | OK |
| `unchecked_background` | `unchecked_background` | OK |
| `thumb_background` | `thumb_background` | OK |
| `disabled_opacity` | — | **MISSING** |

#### §2.22 Dialog — `DialogTheme` (`widgets/mod.rs:499-527`)

| Registry Field | Code Field | Status |
|---|---|---|
| `background_color` | — | **MISSING** (tracked above) |
| `body_font` | — | **MISSING** |
| `min_width` | `min_width` | OK |
| `max_width` | `max_width` | OK |
| `min_height` | `min_height` | OK |
| `max_height` | `max_height` | OK |
| `button_gap` | `button_spacing` | **RENAME** → `button_gap` |
| `button_order` | `button_order` | OK (but enum variants differ, see REG-3) |
| `title_font` | `title_font` | OK |
| `icon_size` | `icon_size` | OK |
| `border` (sub-struct) | partial flat fields | **STRUCTURAL** |
| `border.corner_radius` | `radius` | **RENAME** → nest |
| `border.padding_horizontal` | `content_padding` | **RENAME** → `border.padding_horizontal` (or keep separate?) |

#### §2.23 Spinner / Progress Ring — `SpinnerTheme` (`widgets/mod.rs:531-545`)

| Registry Field | Code Field | Status |
|---|---|---|
| `diameter` | `diameter` | OK |
| `min_diameter` | `min_size` | **RENAME** → `min_diameter` |
| `stroke_width` | `stroke_width` | OK |
| `fill_color` | `fill` | **RENAME** → `fill_color` |

#### §2.24 ComboBox — `ComboBoxTheme` (`widgets/mod.rs:549-567`)

| Registry Field | Code Field | Status |
|---|---|---|
| `background_color` | — | **MISSING** (tracked above) |
| `font` | — | **MISSING** |
| `min_height` | `min_height` | OK |
| `min_width` | `min_width` | OK |
| `arrow_icon_size` | `arrow_size` | **RENAME** → `arrow_icon_size` |
| `arrow_area_width` | `arrow_area_width` | OK |
| `disabled_opacity` | — | **MISSING** |
| `border` (sub-struct) | partial flat fields | **STRUCTURAL** |
| `border.corner_radius` | `radius` | **RENAME** → nest |
| `border.padding_horizontal` | `padding_horizontal` | Name matches, needs nesting |
| `border.color` | — | **MISSING** |

#### §2.25 Segmented Control — `SegmentedControlTheme` (`widgets/mod.rs:571-585`)

| Registry Field | Code Field | Status |
|---|---|---|
| `background_color` | — | **MISSING** (tracked above) |
| `font` | — | **MISSING** |
| `active_background` | — | **MISSING** (tracked above) |
| `active_text_color` | — | **MISSING** (tracked above) |
| `segment_height` | `segment_height` | OK |
| `separator_width` | `separator_width` | OK |
| `disabled_opacity` | — | **MISSING** |
| `border` (sub-struct) | partial flat fields | **STRUCTURAL** |
| `border.corner_radius` | `radius` | **RENAME** → nest |
| `border.padding_horizontal` | `padding_horizontal` | Name matches, needs nesting |
| `border.color` | — | **MISSING** |

#### §2.26 Card / Container — `CardTheme` (`widgets/mod.rs:589-605`)

| Registry Field | Code Field | Status |
|---|---|---|
| `background_color` | `background` | **RENAME** |
| `border` (sub-struct) | partial flat fields | **STRUCTURAL** |
| `border.color` | `border: Rgba` | **RENAME** → nest |
| `border.corner_radius` | `radius` | **RENAME** → nest |
| `border.shadow_enabled` | `shadow` | **RENAME** → nest |
| `border.padding_horizontal/vertical` | `padding` | **RENAME** (single padding → split h/v) |

#### §2.27 Expander / Disclosure — `ExpanderTheme` (`widgets/mod.rs:609-623`)

| Registry Field | Code Field | Status |
|---|---|---|
| `font` | — | **MISSING** |
| `header_height` | `header_height` | OK |
| `arrow_icon_size` | `arrow_size` | **RENAME** → `arrow_icon_size` |
| `border` (sub-struct) | partial flat fields | **STRUCTURAL** |
| `border.corner_radius` | `radius` | **RENAME** → nest |
| `border.padding_horizontal/vertical` | `content_padding` | **RENAME** (single → split h/v) |
| `border.color` | — | **MISSING** (tracked above) |

#### §2.28 Link — `LinkTheme` (`widgets/mod.rs:627-643`)

| Registry Field | Code Field | Status |
|---|---|---|
| `font` | — | **MISSING** |
| `visited_text_color` | `visited` | **RENAME** → `visited_text_color` |
| `underline_enabled` | `underline` | **RENAME** → `underline_enabled` |
| `background_color` | `background` | **RENAME** |
| `hover_background` | `hover_bg` | **RENAME** → `hover_background` |
| — | `color` | **EXTRA** (replaced by `font.color`) |

---

### REG-5. Systematic rename patterns

These patterns cover the bulk of the ~70 naming mismatches:

| Pattern | Registry Convention | Current Code Convention | Count |
|---|---|---|---|
| Color suffix | `xxx_color` (e.g. `background_color`) | `xxx` (e.g. `background`) | ~25 |
| Text color | `xxx_text_color` | `xxx_foreground` | ~15 |
| Border nesting | `border.corner_radius` | `radius` (flat) | ~15 |
| Border line_width | `border.line_width` | `frame_width` or `border_width` | ~3 |
| Border shadow | `border.shadow_enabled` | `shadow` (bool) | ~5 |
| Widget-specific | `icon_text_gap`, `row_height`, etc. | `icon_spacing`, `item_height`, etc. | ~10 |
| Diameter/size | `thumb_diameter`, `min_diameter` | `thumb_size`, `min_size` | ~3 |

### REG-6. Missing non-state fields not already tracked

These are fields that appear in the registry but NOT in the existing
"Missing Interactive State Colors" section above:

| Widget | Missing Field | Registry Type | Notes |
|---|---|---|---|
| defaults | `text_selection_background` | color | macOS `selectedTextBackgroundColor` |
| defaults | `text_selection_color` | color | macOS `selectedTextColor` |
| checkbox | `background_color` | color | Unchecked background |
| checkbox | `indicator_color` | color | Checkmark/dot color |
| checkbox | `disabled_opacity` | f32 | |
| input | `disabled_opacity` | f32 | |
| menu | `icon_size` | f32 | Menu item icon size |
| slider | `disabled_opacity` | f32 | |
| list | `header_font` | font | Column header font |
| popover | `font` | font | Popover text font |
| separator | `line_width` | f32 | Separator thickness |
| switch | `disabled_opacity` | f32 | |
| dialog | `body_font` | font | Dialog body text font |
| combo_box | `font` | font | Trigger text font |
| combo_box | `disabled_opacity` | f32 | |
| segmented_control | `font` | font | Segment label font |
| segmented_control | `disabled_opacity` | f32 | |
| expander | `font` | font | Header label font |
| link | `font` | font | Link text font |
| toolbar | `icon_size` | f32 | Toolbar button icon size |
| sidebar | `font` | font | Sidebar item font |
| layout | `widget_gap` | f32 | Entire struct not yet implemented |
| layout | `container_margin` | f32 | |
| layout | `window_margin` | f32 | |
| layout | `section_gap` | f32 | |

### REG-7. Extra fields in code NOT in registry

These exist in the code but have no corresponding registry entry. They should
either be added to the registry (and platform-facts.md), or removed from code
if replaced by Font.color or Border sub-struct.

| Widget | Code Field | Disposition |
|---|---|---|
| defaults | `spacing: ThemeSpacing` | **REMOVE** (registry NOTE confirms) |
| window | `foreground` | Remove if `title_bar_font.color` covers it, or add to registry |
| window | `title_bar_foreground` | Remove if `title_bar_font.color` covers it, or add to registry |
| button | `foreground` | Remove — replaced by `font.color` |
| input | `foreground` | Remove — replaced by `font.color` |
| menu | `foreground` | Remove — replaced by `font.color` |
| tooltip | `foreground` | Remove — replaced by `font.color` |
| tab | `foreground` | Remove — replaced by `font.color` |
| sidebar | `foreground` | Remove — replaced by `font.color` |
| list | `foreground` | Remove — replaced by `item_font.color` |
| list | `header_foreground` | Remove — replaced by `header_font.color` |
| popover | `foreground` | Remove — replaced by `font.color` |
| link | `color` | Remove — replaced by `font.color` |
| toolbar | `padding` | Not in registry — add to registry or remove |
| dialog | `content_padding` | Maps to `border.padding_horizontal/vertical` or keep? |
