# v0.5.5 TODO

Issues found during comprehensive audit (2026-04-04).

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
