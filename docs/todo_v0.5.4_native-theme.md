# v0.5.4 -- native-theme: Deep Critical Analysis

Issues found in the core `native-theme` crate. Every issue has file:line
references, multiple solution options with pros/cons tables, and a
recommended fix.

---

## 1. Preset Value Mismatches vs platform-facts.md

### 1a. Windows 11 `dialog.max_width = 560` -- platform-facts says 548

**File:** `src/presets/windows-11.toml:166,365`

platform-facts.md SS1.2.4: "WinUI3 ContentDialog: 320-548px" (XAML confirmed).
Preset uses `max_width = 560.0` in both light and dark.

**Impact:** Dialogs 12px wider than native maximum.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Fix to 548 | Matches XAML-confirmed value | Slightly narrower than current |
| B | Keep 560, add comment | No change needed | Contradicts documented source |

**Recommended:** A. The XAML resource is authoritative.

**Also affects:** `src/presets/windows-11-live.toml:104,238`

### 1b. Windows 11 `dialog.min_height = 140` -- platform-facts says 184

**File:** `src/presets/windows-11.toml:167,366`

platform-facts.md SS1.2.4: "WinUI3 ContentDialog: 184-756". Preset uses 140.

**Impact:** Allows dialogs 44px shorter than native minimum.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Fix to 184 | Matches XAML-confirmed value | Taller minimum |
| B | Keep 140 | More flexible | Contradicts authoritative source |

**Recommended:** A.

**Also affects:** `src/presets/windows-11-live.toml:105,239`

### 1c. Windows 11 `dialog.max_height = 600` -- platform-facts says 756

**File:** `src/presets/windows-11.toml:168,367`

platform-facts.md SS1.2.4: "WinUI3 ContentDialog: 184-756". Preset uses 600.

**Impact:** Dialogs 156px shorter than native allows.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Fix to 756 | Matches XAML-confirmed value | Taller max |
| B | Keep 600, add comment | Deliberate conservative choice | Contradicts source |

**Recommended:** A.

**Also affects:** `src/presets/windows-11-live.toml:106,240`

### 1d. Windows 11 `spinner.stroke_width = 2` -- platform-facts says 4

**File:** `src/presets/windows-11.toml:181,380`

platform-facts.md SS1.2.4: "WinUI3: ProgressRingStrokeThickness=4". Preset uses 2.0.

**Impact:** Spinner ring is half the native thickness.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Fix to 4.0 | Matches WinUI3 resource exactly | Thicker stroke |
| B | Keep 2.0, document as intentional | Thinner appearance may be preferred | Contradicts platform source |

**Recommended:** A.

**Also affects:** `src/presets/windows-11-live.toml:117,251`

### 1e. Windows 11 `tooltip.padding = 8.0` both horiz and vert -- platform-facts says 9/6-8

**File:** `src/presets/windows-11.toml:98-100,297-299`

platform-facts.md SS1.2.4: "WinUI3: ToolTipBorderPadding=9,6,9,8". The preset uses
symmetric 8.0 for both horizontal and vertical. The model only has `padding_horizontal`
and `padding_vertical` (no per-side values), so some imprecision is expected, but
horizontal should be 9, not 8.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Fix horizontal to 9, vertical to 7 (average of 6/8) | Closer to WinUI3 spec | Not perfectly symmetric |
| B | Keep 8/8 as reasonable midpoint | Simple | 1px off horizontally |

**Recommended:** A. The horizontal value is clearly documented as 9.

**Also affects:** `src/presets/windows-11-live.toml:51-52,185-186`

### 1f. Windows 11 `spinner.diameter = 24` -- platform-facts says 32

**File:** `src/presets/windows-11.toml:179,378`

platform-facts.md SS1.2.4: "WinUI3: ProgressRing Width/Height=32". Preset uses 24.0.

**Impact:** Spinner is 8px smaller than native default. This could be intentional
(24 is a common "compact" size), but should match the documented 32px default.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Fix to 32 | Matches WinUI3 default resource | Larger spinner |
| B | Keep 24, add TOML comment explaining compact choice | Consistent with "compact" aesthetic | Contradicts WinUI3 default |

**Recommended:** A. The WinUI3 XAML resource is authoritative.

**Also affects:** `src/presets/windows-11-live.toml:115,249`

### 1g. Windows 11 `expander.header_height = 40` -- platform-facts says 48

**File:** `src/presets/windows-11.toml:199,398`

platform-facts.md SS1.2.4: "WinUI3: ExpanderMinHeight=48". Preset uses 40.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Fix to 48 | Matches WinUI3 resource | Taller header |
| B | Keep 40 | Compact | Contradicts source |

**Recommended:** A.

**Also affects:** `src/presets/windows-11-live.toml:135,269`

### 1h. Windows 11 `expander.content_padding = 16` -- platform-facts says 16

Correct. Matches `ExpanderContentPadding=16`.

### 1i. Adwaita `dialog.button_spacing = 8` -- platform-facts says 12

**File:** `src/presets/adwaita.toml:177,382`

platform-facts.md SS1.4.4: "AdwAlertDialog button spacing: 12px" from
`_message-dialog.scss .response-area { border-spacing: 12px }`. Preset uses 8.

**Impact:** Button gap 4px narrower than native Adwaita.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Fix to 12 | Matches libadwaita CSS source | Wider gap |
| B | Keep 8, document | Consistent with current look | Contradicts authoritative CSS |

**Recommended:** A.

**Also affects:** `src/presets/adwaita-live.toml:111,248`

### 1j. macOS `dialog.button_order = "leading_affirmative"` -- should be trailing

**File:** `src/presets/macos-sonoma.toml` (dialog section)

platform-facts.md SS2.22: "macOS primary action = rightmost." The macOS HIG places
the default (affirmative) button at the trailing (right) end of the button row.
The enum doc for `LeadingAffirmative` says "macOS, KDE style" which is incorrect
for macOS. KDE does put OK left of Cancel, but macOS puts the default button
rightmost.

**Impact:** macOS dialog button order is wrong -- affirmative button renders on
the left instead of the right.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Change macOS preset to `trailing_affirmative` | Matches Apple HIG | Changes dialog layout for macOS users |
| B | Fix enum docs only | Non-breaking | Preset still wrong |
| C | Add a third variant `AffirmativeRightmost` for macOS | Precise semantics | Breaking API change, over-engineering |

**Recommended:** A. Fix the preset and fix the enum doc comment that incorrectly
lists macOS with `LeadingAffirmative`.

**Also affects:** `src/presets/macos-sonoma-live.toml:102,236`

### 1k. Adwaita `checkbox.indicator_size = 20` -- platform-facts says 14 (20 with padding)

**File:** `src/presets/adwaita.toml:100,305`

platform-facts.md SS1.4.4: "CheckButton indicator size: 14px (20px with padding)".
The field name `indicator_size` semantically refers to the visual indicator itself,
not the indicator + padding bounding box.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Fix to 14 | Matches native indicator size | Visually smaller checkbox |
| B | Keep 20, document that it includes padding | No visual change | Semantic mismatch with field name |
| C | Add `indicator_padding` field, set indicator=14, padding=3 | Precise model | Adds a new field to the API |

**Recommended:** B for v0.5.4 (add TOML comment), C for a future release.
The 20px value produces correct visual results when connectors use it as the
clickable hit-target size.

### 1l. Windows 11 `menu.icon_spacing = 8` -- platform-facts says 12

**File:** `src/presets/windows-11.toml:128,327`

platform-facts.md SS1.2.4: "WinUI3: icon placeholder=28px minus 16px icon = 12px gap".
Preset uses 8.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Fix to 12 | Matches WinUI3 derived value | Wider gap |
| B | Keep 8 | Compact | Contradicts source |

**Recommended:** A.

**Also affects:** `src/presets/windows-11-live.toml:80,214`

### 1m. Windows 11 `combo_box.min_width = 120` -- platform-facts says 64

**File:** `src/presets/windows-11.toml:185,384`

platform-facts.md SS1.2.4: "WinUI3: ComboBoxThemeMinWidth=64". Preset uses 120.

**Impact:** ComboBox minimum 56px wider than native.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Fix to 64 | Matches WinUI3 resource | Narrower minimum |
| B | Keep 120, document as deliberate wider minimum | Prevents clipped text | Contradicts source |

**Recommended:** A.

**Also affects:** `src/presets/windows-11-live.toml:121,255`

### 1n. Windows 11 `toolbar.padding = 4` -- platform-facts says "4px left only"

**File:** `src/presets/windows-11.toml:139,338`

platform-facts.md SS1.2.4: "WinUI3: CommandBar Padding=4,0,0,0". The model field
`padding` applies uniformly on all sides. The WinUI3 value is 4px left only.

**Verdict:** Acceptable approximation given model constraints. Add TOML comment.

### 1o. KDE Breeze `expander.header_height = 40` -- consistent with Kirigami but not Breeze metrics

**File:** `src/presets/kde-breeze.toml:198`

There is no `ExpanderMinHeight` constant in breezemetrics.h. The Kirigami
equivalent is the `SwipeListItem` or delegateHeight -- typically 36px
(`Kirigami.Units.gridUnit * 2`). Preset uses 40. The AdwExpanderRow
equivalent is 50px (GNOME). 40 is a reasonable compromise.

**Verdict:** Acceptable. Add TOML comment citing Kirigami.

### 1p. KDE Breeze `combo_box.padding_horizontal = 12` -- platform-facts says 6

**File:** `src/presets/kde-breeze.toml:186,384`

platform-facts.md SS1.3.4 line 649: "ComboBox_FrameWidth | ComboBox padding | 6px |
breezemetrics.h". Cross-reference SS2.24 line 1223: "KDE: `ComboBox_FrameWidth` = 6".
Preset uses `padding_horizontal = 12.0` in both light and dark.

**Impact:** ComboBox horizontal padding is double the native Breeze value.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Fix to 6 | Matches breezemetrics.h exactly | Narrower padding |
| B | Keep 12, document as deliberate | Roomier appearance | Contradicts authoritative Breeze source |

**Recommended:** A. The `ComboBox_FrameWidth` constant in breezemetrics.h is authoritative.

**Also affects:** `src/presets/kde-breeze-live.toml:122,256`

### 1q. KDE Breeze `combo_box.arrow_area_width = 28` -- platform-facts says 20

**File:** `src/presets/kde-breeze.toml:188,386`

platform-facts.md SS1.3.4 line 650: "MenuButton_IndicatorWidth | ComboBox arrow area
width | 20px | breezemetrics.h". Cross-reference SS2.24 line 1225: "KDE: 20px".
Preset uses `arrow_area_width = 28.0` in both light and dark.

**Impact:** Arrow area 8px wider than native Breeze, visually over-sized.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Fix to 20 | Matches breezemetrics.h exactly | Narrower arrow area |
| B | Keep 28 | Possibly accommodates larger icon | Contradicts authoritative source |

**Recommended:** A. The `MenuButton_IndicatorWidth` constant is authoritative.

**Also affects:** `src/presets/kde-breeze-live.toml:124,258`

### 1r. Adwaita `expander.arrow_size = 12` -- platform-facts says 16

**File:** `src/presets/adwaita.toml:207,412`

platform-facts.md SS1.4.4 line 794: "GtkExpander | arrow size | 16 x 16px |
_expanders.scss `min-width/min-height: 16px`". Cross-reference SS2.27 line 1260:
"GNOME: 16px (pan-end-symbolic)". Preset uses `arrow_size = 12.0` in both light
and dark.

**Impact:** Expander arrow 4px smaller than native Adwaita, visually undersized.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Fix to 16 | Matches libadwaita CSS source | Larger arrow |
| B | Keep 12 | Compact | Contradicts authoritative _expanders.scss |

**Recommended:** A. The libadwaita CSS `min-width/min-height: 16px` is authoritative.

**Also affects:** `src/presets/adwaita-live.toml:139,276`

### 1s. Adwaita `expander.header_height = 40` -- platform-facts says 50

**File:** `src/presets/adwaita.toml:206,411`

platform-facts.md SS1.4.4 line 795: "AdwExpanderRow | header min-height | 50px |
_lists.scss". Cross-reference SS2.27 line 1259: "GNOME: AdwExpanderRow: 50".
Preset uses `header_height = 40.0` in both light and dark.

**Impact:** Expander header 10px shorter than native AdwExpanderRow, may feel cramped.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Fix to 50 | Matches libadwaita CSS source | Taller header |
| B | Keep 40 | Compact | Contradicts authoritative _lists.scss |

**Recommended:** A. The AdwExpanderRow min-height of 50px from _lists.scss is
authoritative.

**Also affects:** `src/presets/adwaita-live.toml:138,275`

### 1t. Adwaita `tab.padding_vertical = 4` -- platform-facts says 3

**File:** `src/presets/adwaita.toml:129,334`

platform-facts.md SS1.4.4 line 801: "Notebook (tab) | tab padding | 3px 12px |
_notebook.scss `padding: 3px 12px`". The horizontal padding (12) matches but the
vertical padding is 4 in the preset vs 3 in the CSS. Both light and dark variants
have `padding_vertical = 4.0`.

**Impact:** Tab vertical padding 1px more than native -- subtle but measurable.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Fix to 3 | Matches _notebook.scss exactly | 1px shorter tabs |
| B | Keep 4 | Slightly roomier | Contradicts authoritative CSS |

**Recommended:** A. The _notebook.scss `padding: 3px 12px` is authoritative.

**Also affects:** `src/presets/adwaita-live.toml:77,214`

### 1u. iOS `dialog.button_order = "leading_affirmative"` -- same Apple HIG issue as macOS

**File:** `src/presets/ios.toml:164,362`

Apple's Human Interface Guidelines apply to both macOS and iOS. For iOS
`UIAlertController` with 2 side-by-side buttons, the preferred (affirmative)
action is on the right; cancel is on the left. This is the same
`trailing_affirmative` convention as macOS.

The iOS preset has `button_order = "leading_affirmative"` in both light and
dark variants, which is wrong for the same reason as the macOS preset (issue 1j).

**Impact:** iOS dialog button order is wrong -- affirmative button renders on
the left instead of the right.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Fix to `trailing_affirmative` | Matches Apple HIG for iOS | Changes dialog layout for iOS users |
| B | Keep as-is | No change | Contradicts Apple HIG |

**Recommended:** A. The Apple HIG is authoritative for both macOS and iOS.

---

## 2. Unit Test Issues

### 2a. Missing test: `validate()` range checks never tested

**File:** `src/resolve.rs:139-164`

The range-check helpers (`check_non_negative`, `check_positive`,
`check_range_f32`, `check_range_u16`) exist and are called in `validate()`,
but there are zero tests verifying they fire. A preset with `font.size = -1.0`,
`disabled_opacity = 2.0`, or `font.weight = 0` should fail validation with a
descriptive error. Without negative tests, regressions here would be silent.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add negative tests for each range-check category | Catches regressions, documents valid ranges | ~10-15 new tests |
| B | Add one parametric test covering all range categories | Compact | Less readable |
| C | Leave untested | No work | Silent regressions possible |

**Recommended:** A. These are critical correctness checks.

### 2b. Missing test: exhaustive `icon_name()` coverage

**File:** `src/model/icons.rs` (the `icon_name()` function)

`icon_name()` maps 42 `IconRole` variants across 5 `IconSet` values = 210
combinations. There is no test that verifies every (role, set) combination
returns a consistent non-None value for the bundled sets (Material, Lucide).
A typo or missing match arm would silently return `None`.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add test iterating `IconRole::ALL` x `{Material, Lucide}`, asserting `is_some()` | Catches missing mappings | ~84 assertions |
| B | Add compile-time exhaustiveness check via match without wildcard | Compile-time guarantee | Already has `_ => None` fallback by design |
| C | Leave as-is | No work | Missing icon names undetected |

**Recommended:** A. This is easy to write and catches real bugs.

**Specific gaps identified:**
1. No test that Material maps all 42 roles to `Some(...)` (Material covers everything)
2. No test that Lucide maps all 42 roles to `Some(...)` (Lucide covers everything)
3. No test that Freedesktop maps all 42 roles to `Some(...)` (Freedesktop covers everything)
4. No test that Segoe maps 40 of 42 roles (missing StatusBusy) to `Some(...)`

These gaps mean a missing match arm in `material_name()`, `lucide_name()`,
or `freedesktop_name()` would go undetected.

### 2c. Duplicated test coverage between `tests/preset_loading.rs` and `src/presets.rs`

**File:** `tests/preset_loading.rs` vs `src/presets.rs::tests`

Six duplicated test groups:
- `all_presets_parse_without_error` / `all_presets_loadable_via_preset_fn`
- `all_presets_have_both_variants` (duplicated logic)
- `all_presets_have_core_colors` / `all_presets_have_nonempty_core_colors`
- `all_presets_round_trip_toml` / `all_presets_round_trip_exact`
- `list_presets_returns_sixteen_entries` / `list_presets_returns_all_sixteen`
- `preset_names_are_correct` / `presets_have_correct_names`

**Impact:** Any change requires updating two files.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Keep integration tests, remove unit-level duplicates | Single source of truth | Slightly less defense-in-depth |
| B | Keep both, accept duplication | Belt-and-suspenders | Maintenance cost |

**Recommended:** A. The integration tests exercise the public API, which is the
right level. Unit tests in `presets.rs` should focus on internal functions
(cache behavior, `from_toml`, `from_file`).

### 2d. `resolved.rs` tests are construction-only, not behavioral

**File:** `src/model/resolved.rs::tests`

Two massive tests (`resolved_theme_construction_with_all_widgets` and
`resolved_theme_derives_clone_debug_partialeq`) both construct a full
`ResolvedThemeVariant` by hand. They test that the type compiles with all
fields, not that resolution behavior is correct. Combined ~400 lines of
boilerplate.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Replace with one compact construction test + behavioral tests | Tests actual behavior | Needs new test design |
| B | Keep one, delete the other | Halves boilerplate | Still not testing behavior |
| C | Keep both | No work | 200 lines of waste |

**Recommended:** A. Keep one minimal construction test. Add tests like
"resolve fills button.background from defaults.background when None" for
critical inheritance rules.

### 2e. Missing test: `lint_toml()` coverage

**File:** `src/model/mod.rs` (the `lint_toml()` method)

Only a doctest exercises `lint_toml()`. No dedicated unit test. If a widget
struct gains a field but `FIELD_NAMES` is not updated, `lint_toml` silently
treats valid fields as "unknown".

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add test verifying FIELD_NAMES matches actual serde field names | Catches drift | Requires introspection |
| B | Auto-derive FIELD_NAMES at compile time via proc-macro | Cannot drift | Adds macro complexity |
| C | Add test that lint_toml reports no warnings for a known-good preset | Lightweight regression test | Does not catch all drift |

**Recommended:** C first (easy), then A later.

### 2f. `error.rs` test `non_platform_source_is_none` misses Io variant

**File:** `src/error.rs:186-189`

The test name is "non_platform_source_is_none" but it only checks `Unsupported`,
`Unavailable`, and `Format`. It does not check that `Io` returns `Some` from
`source()`, which it does. The test name is misleading and coverage is incomplete.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Split into two tests: "source_is_none" for {Unsupported, Unavailable, Format}, "source_is_some" for {Platform, Io, Resolution} | Accurate coverage, clear names | Minor refactor |
| B | Add Io check to existing test | Minimal change | Name still misleading |

**Recommended:** A.

### 2g. `dialog_order.rs` has 6 tests for a 2-variant enum

**File:** `src/model/dialog_order.rs:23-84`

Six tests for a trivial 2-variant enum. Three pairs of
serialize/deserialize/round-trip for each variant. This is excessive -- a
single round-trip per variant covers the same ground.

**Verdict:** Low priority. Not harmful, just bloated. Could consolidate to 2-3
tests.

### 2h. Missing test: `from_toml_with_base()` base inheritance

**File:** `src/model/mod.rs` (the `from_toml_with_base()` method)

This is a key API for users -- load a base preset, then overlay a sparse
TOML. There are no dedicated tests for this path. It is tested indirectly
through `build_theme` in `gnome/mod.rs`, but no test verifies the public API
directly with realistic user scenarios.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add integration test: load adwaita, overlay sparse TOML, verify merged fields | Tests real user workflow | ~20 lines |
| B | Leave to doctest | Existing doctest covers it | Doctests are fragile |

**Recommended:** A.

### 2i. Missing test: `validate()` cross-field constraints (dialog min/max)

**File:** `src/resolve.rs:1674-1677`

`validate()` checks each geometry field is non-negative, but never verifies
cross-field constraints:
- `dialog.min_width <= dialog.max_width` is not checked
- `dialog.min_height <= dialog.max_height` is not checked

A theme with `min_width = 600, max_width = 400` passes validation but
produces nonsensical rendering constraints.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add cross-field checks after range checks | Catches nonsensical themes; 4 lines | Slightly more validation code |
| B | Leave to toolkit connectors | No core change | Bad themes pass validation silently |
| C | Add a `check_min_max(min, max, prefix, errors)` helper | Reusable, DRY; follows existing `check_non_negative()` pattern | One more helper function |

**Recommended:** C. The helper follows the existing `check_non_negative()` pattern
and can guard both dialog width and height dimensions.

### 2j. Missing test: `resolve()` idempotency guarantee

**File:** `src/resolve.rs:174`

The doc comment states: "Calling resolve() twice produces the same result
(idempotent)." No test verifies this. If a future resolve rule mutates a
field unconditionally (rather than only when `None`), the guarantee breaks
silently.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add test: `resolve()`, clone, `resolve()` again, assert_eq | Documents the property; catches drift | ~15 lines per representative preset |
| B | Leave untested | No work | Regression risk on a documented guarantee |

**Recommended:** A. A single test over one or two presets is sufficient.

---

## 3. Code Quality Issues

### 3a. Repeated `std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default()` (5 call sites)

**Files:** `src/lib.rs:743`, `src/lib.rs:894`, `src/lib.rs:973`,
`src/model/icons.rs` (detect_icon_theme), `src/presets.rs` (detect_platform)

The same env var read with the same fallback is duplicated 5 times.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Extract `fn xdg_current_desktop() -> String` | DRY, single place for caching | One more function |
| B | Keep duplicated | No refactor | 5 copies to maintain |

**Recommended:** A.

### 3b. `lint_toml()` hardcodes field name lists that can drift

**File:** `src/model/mod.rs` (the `lint_toml()` function)

`VARIANT_KEYS`, `TEXT_SCALE_ENTRY_FIELDS`, `TEXT_SCALE_KEYS`, `FONT_FIELDS`,
`SPACING_FIELDS` are hardcoded string lists. The `define_widget_pair!` macro
already generates `FIELD_NAMES` constants for each widget struct, but
`lint_toml()` does not use them for defaults, text_scale, font, or spacing.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Replace hardcoded lists with `FIELD_NAMES` constants from each struct | Single source of truth, cannot drift | Need to add FIELD_NAMES to ThemeDefaults, FontSpec, ThemeSpacing, TextScale, TextScaleEntry |
| B | Add test that hardcoded lists match serde fields | Catches drift at test time | Still duplicated |
| C | Keep as-is | No work | Silent drift possible |

**Recommended:** A. ThemeDefaults already has `FIELD_NAMES`; the remaining
structs need it added.

### 3c. `gnome/mod.rs` `build_theme` clones the Adwaita base even when only selecting one variant

**File:** `src/gnome/mod.rs:288-316`

`build_theme()` takes ownership of `base: ThemeSpec`, clones either
`base.dark.unwrap_or_default()` or `base.light.unwrap_or_default()`, merges
the OS variant, then constructs a new `ThemeSpec` with only one variant.
The unused variant from the base is discarded.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Use `Option::take()` instead of `unwrap_or_default()` | Avoids clone of the variant | Base is consumed, already owned -- need mut |
| B | Keep as-is | Simple, correct | One unnecessary clone per call |

**Recommended:** B for v0.5.4. The clone is a one-time cost during theme load.

### 3d. `presets.rs` cache stores `Result<ThemeSpec, String>` instead of `Error`

**File:** `src/presets.rs:86-97`

The comment explains: "Errors are stored as String (Error is not Clone)."
The error is re-wrapped as `Error::Format` in `preset()`. This works but
discards the original error type.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Make `Error::Format` carry the original `toml::de::Error` | Preserves full error chain | `toml::de::Error` is not Clone either |
| B | Store `Arc<Error>` in cache | Preserves error, shareable | Over-engineering for a compile-time cache |
| C | Keep String (current) | Simple | Loses error type info |

**Recommended:** C. The only errors in this cache are TOML parse errors,
and they are already re-wrapped as `Error::Format(string)`, which is the
correct variant.

### 3e. `resolve.rs` `validate()` is 900+ lines of mechanical field extraction

**File:** `src/resolve.rs:600-1500+`

Every field is manually extracted with `require()`, then manually constructed
into a `ResolvedThemeVariant`. Every new widget field needs ~5 lines in
`validate()` plus the corresponding resolved struct field.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Generate validate() with a proc-macro from Option-struct to concrete-struct | Auto-maintained, no drift | Adds a proc-macro dependency |
| B | Add "add new fields here" comments at each section | Helps maintainers | Still manual |
| C | Keep as-is | No new complexity | Risk of forgetting a field |

**Recommended:** B for v0.5.4. The proc-macro approach is future work.

---

## 4. API Design Issues

### 4a. `DialogButtonOrder` enum docs are incorrect

**File:** `src/model/dialog_order.rs:11-19`

The doc says `LeadingAffirmative` = "macOS, KDE style". This is wrong for
macOS. macOS places the default button rightmost (trailing). Only KDE uses
leading affirmative. The macOS preset should use `TrailingAffirmative`.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Fix docs: Leading = "KDE style", Trailing = "Windows, GNOME, macOS style" | Accurate docs | No behavior change alone |
| B | Rename variants to `AffirmativeLeft`/`AffirmativeRight` | Clear semantics | Breaking API change |
| C | Fix docs + fix macOS preset (see 1j) | Both correct | Two changes in one |

**Recommended:** C. Fix the docs AND the macOS preset together.

### 4b. `IconSet::Default` is `Freedesktop` -- platform-inappropriate on non-Linux

**File:** `src/model/icons.rs` (the `#[default]` on `IconSet`)

`IconSet` derives `Default` with `#[default] Freedesktop`. On macOS or
Windows, `IconSet::default()` returns `Freedesktop` which makes no sense.
The `resolve()` pipeline fills it correctly via `system_icon_set()`, but any
code path using `IconSet::default()` directly gets a wrong value.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Remove `Default` derive, force explicit construction | No accidental wrong value | Breaking change -- `ThemeVariant::default()` needs icon_set=None which it already is |
| B | Keep, document the gotcha | Non-breaking | Silent wrong value if misused |
| C | Use `cfg`-conditioned default (Freedesktop on Linux, SfSymbols on macOS, SegoeIcons on Windows) | Correct on all platforms | More complex impl |

**Recommended:** B for v0.5.4. The `resolve()` pipeline handles this.
Document that `IconSet::default()` is a serialization-friendly fallback,
not a platform-correct value.

### 4c. `Rgba::default()` is transparent black `(0,0,0,0)`

**File:** `src/color.rs:41`

`Rgba` derives `Default`, yielding `{r:0, g:0, b:0, a:0}` (transparent black).
This is used as a placeholder in `require()`. Since require() only uses the
placeholder when validation will fail anyway, this is not a bug. But users
calling `Rgba::default()` get transparent black, not opaque black, which could
surprise.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add doc comment to `Rgba` noting default is transparent | Clear expectation | No behavior change |
| B | Change default to opaque black | More intuitive | Breaks `require()` semantics, though benignly |

**Recommended:** A.

### 4d. No `Display` impl for `IconRole`, `IconSet`

**File:** `src/model/icons.rs`

`IconRole` and `IconSet` have no `Display` impl. Debugging or logging which
icon role failed to load requires `Debug` format (`{:?}`) which is noisy.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add `Display` impls that return the snake_case name | Clean logging | ~10 lines per enum |
| B | Keep using `Debug` | No work | Noisy output |

**Recommended:** A. Small effort, good ergonomics.

---

## 5. Correctness Issues

### 5a. `run_pipeline` uses `unwrap_or` safely but fragile pattern

**File:** `src/lib.rs:649`

```rust
let full_preset_name = preset_name.strip_suffix("-live").unwrap_or(preset_name);
```

This is safe (`unwrap_or` on `Option`, not `unwrap()`). However, the
convention relies on live presets always having a "-live" suffix. If a
live preset is added without this suffix, the fallback silently uses
the live preset as the full preset, which would lack colors.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add assertion or debug_assert that full_preset_name exists | Catches naming convention violations | Extra check |
| B | Keep as-is | Simple | Silent failure if naming convention broken |
| C | Accept preset_name + explicit full_preset_name as a tuple | Cannot be wrong | API change in internal function |

**Recommended:** A. A `debug_assert!(ThemeSpec::preset(full_preset_name).is_ok())` would
catch broken naming conventions during development.

### 5b. `gnome/mod.rs` `from_gnome()` unwraps portal settings with `unwrap_or_default`

**File:** `src/gnome/mod.rs:343`

```rust
let scheme = settings.color_scheme().await.unwrap_or_default();
```

This is correct -- `ColorScheme::default()` is `NoPreference`, which is the
right fallback. Same for `contrast`. Not a bug.

### 5c. Community presets hardcode `button_order = "trailing_affirmative"`

**File:** All 10 community preset TOMLs

All community presets (catppuccin-*, nord, dracula, gruvbox, solarized,
tokyo-night, one-dark) hardcode `trailing_affirmative`. When used on KDE
(which uses `leading_affirmative`), the dialog button order feels non-native.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Omit `button_order` from community presets entirely | Platform resolve() fills correct value | Community presets need resolve() to be fully usable |
| B | Add `resolve()` fallback for button_order from platform | Correct on every platform | New resolve rule |
| C | Keep as-is | No change | Wrong for KDE/macOS users |

**Recommended:** A+B. Omit from community presets AND add a resolve rule:
`if dialog.button_order.is_none() { dialog.button_order = Some(platform_button_order()) }`.

**Prerequisite:** The `resolve()` rule is a prerequisite for omitting
`button_order` from community presets. Without a `platform_button_order()`
resolve function, removing the field would cause those presets to fail
validation. The correct platform defaults are:
- KDE: `LeadingAffirmative`
- Everything else (Windows, GNOME, macOS, iOS): `TrailingAffirmative`

### 5d. `from_kde_content` calls `ini.get("General", "ColorScheme").unwrap_or_else`

**File:** `src/kde/mod.rs:52`

```rust
let name = ini
    .get("General", "ColorScheme")
    .unwrap_or_else(|| "KDE".to_string());
```

This is safe (`unwrap_or_else` on `Option`, not `unwrap()`). The fallback
"KDE" is reasonable when no scheme name is configured. Not a bug.

### 5e. MSRV compatibility with `let` chains

**File:** `src/resolve.rs:46-49`, `src/gnome/mod.rs:150-153`, `src/kde/mod.rs` (multiple)

The codebase uses `let` chains (e.g., `if let Some(x) = foo && let Some(y) = bar`),
which were stabilized in Rust 1.87.0. The workspace `rust-version` must be >= 1.87
or these will fail to compile on older toolchains.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Verify workspace Cargo.toml sets `rust-version = "1.87"` or higher | Correct MSRV | None |
| B | Refactor to nested `if let` for lower MSRV | Supports older Rust | Verbose code |

**Recommended:** A. Simply verify and document.

### 5f. `xdg_config_dir()` falls back to `/tmp/.config` when `$HOME` is unset

**File:** `src/model/icons.rs:637-639`

When neither `$XDG_CONFIG_HOME` nor `$HOME` is set, the function returns
`/tmp/.config`. Icon theme detection then looks at `/tmp/.config/kdeglobals`,
which is semantically wrong. While `$HOME` is virtually always set on Linux,
the fallback path is incorrect in principle.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Return early with `"hicolor"` when both env vars are unset | Correct fallback behavior | Slight behavior change in pathological edge case |
| B | Keep `/tmp` fallback (current) | No change | Wrong path if HOME unset |

**Recommended:** A. Low-risk fix with correct semantics.

---

## 6. Documentation Issues

### 6a. `presets.rs` module doc says "2 core + 4 platform + 10 community"

**File:** `src/presets.rs:1-8`

The grouping "2 core platform (kde-breeze, adwaita), 4 platform (windows-11,
macos-sonoma, material, ios)" is confusing. All 6 are platform presets.

**Fix:** Simplify to "6 platform presets and 10 community presets."

### 6b. `from_file` doc says `Error::Unavailable` but returns `Error::Io`

**File:** `src/presets.rs` (the `from_file` function)

The `# Errors` section says "Returns `Error::Unavailable`" but the code does
`std::fs::read_to_string(path)?` which converts via `From<std::io::Error>` to
`Error::Io`.

**Fix:** Change doc to say `Error::Io`.

### 6c. `ThemeDefaults` doc does not mention `selection_inactive`

**File:** `src/model/defaults.rs:31`

The doc comment for `ThemeDefaults` mentions `accent`, `radius`, `line_height`
as examples of Option fields but does not mention `selection_inactive`, which
has special resolve behavior (derived from `selection`, not a direct default).

**Verdict:** Low priority. The field has its own doc comment.

---

## 7. Additional Findings

### 7a. `color.rs` hex parsing slices strings by byte index, assumes ASCII

**File:** `src/color.rs:143-188`

The `FromStr` impl for `Rgba` slices hex strings by byte position (`&hex[0..1]`).
This is safe because hex characters are always ASCII, and the parser rejects
non-hex characters via `u8::from_str_radix`. However, there is no explicit
guard against multi-byte UTF-8 input before slicing. The `strip_prefix('#')`
handles the '#' correctly (it is ASCII). Non-ASCII input would be caught by
the length check or by `from_str_radix` returning an error.

**Verdict:** Correct but could benefit from a comment noting the ASCII-only
assumption.

### 7b. `freedesktop.rs` `detect_theme()` calls `system_icon_theme()` which may cache stale value

**File:** `src/freedesktop.rs:17`

`detect_theme()` calls `crate::system_icon_theme()` which uses `OnceLock` for
caching. If the user changes their icon theme while the app is running,
`load_freedesktop_icon()` will use the stale theme.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Use `detect_icon_theme()` (uncached) instead of `system_icon_theme()` | Fresh value every time | Slight perf cost (env var read + gsettings/ini parse) |
| B | Keep cached, document the limitation | Fast | Stale after theme change |
| C | Add a `refresh_icon_theme()` function | User can invalidate cache | More API surface |

**Recommended:** B. The `system_is_dark()` / `detect_is_dark()` pattern already
documents this tradeoff. Icon loading during a theme change is an edge case.

### 7c. `kde/mod.rs` `parse_icon_sizes_from_index_theme` allocates a new INI parser per call

**File:** `src/kde/mod.rs:252-269`

Every call creates a new `Ini` parser, reads the file, and parses it. This
happens once during theme detection so it is not a hot path.

**Verdict:** Acceptable. Not a performance concern.

### 7d. `windows.rs` uses `face_name.iter().position(|&c| c == 0).unwrap_or(32)`

**File:** `src/windows.rs:110`

This is safe: `unwrap_or(32)` provides a fallback that uses the full 32-char
LOGFONTW faceName buffer. Not a bug.

### 7e. `model/mod.rs` `ThemeSpec::new()` takes `impl Into<String>` -- good API

No issue. Just noting this is well-designed.

### 7f. No test for `SystemTheme::with_overlay()` accent re-derivation

**File:** `src/lib.rs:525-552`

`with_overlay()` starts from pre-resolve variants, merges overlay, then
re-resolves. This should cause accent-derived fields (button.primary_background,
checkbox.checked_background, slider.fill, etc.) to be re-derived from the new
accent. There are no tests verifying this re-derivation chain.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add integration test: from_system() (or preset), then with_overlay(accent change), verify derived fields changed | Documents critical behavior | Requires test infrastructure |
| B | Leave untested | No work | Regression risk |

**Recommended:** A.

### 7g. `Rgba::from_f32()` and `to_f32_array()` are not exact inverses due to u8 quantization

**File:** `src/color.rs:70-89`

`from_f32(0.5, ...)` produces `r=128` (0.5 * 255 = 127.5, rounded up).
`to_f32_array()` on `r=128` produces `128/255 = 0.5019...`, not `0.5`. This
is inherent to u8 quantization and is NOT a bug, but there is no test or doc
comment documenting this non-invertibility. A user might expect round-trip
fidelity.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add a test documenting the quantization behavior | Sets expectations; ~5 lines | Trivial effort |
| B | Add doc comment noting u8 quantization | Sets expectations in docs | Slightly more verbose |

**Recommended:** A+B. Both are cheap and prevent user confusion.

---

## 8. Test Completeness Summary

### Well-tested areas
- Color hex parsing (`color.rs`): 15 tests with good edge cases
- Merge semantics: thorough across defaults, fonts, spacing, variants, ThemeSpec
- Serde round-trip: comprehensive for all 16 presets
- Error types: Display, source(), From conversions all covered
- KDE reader: extensive color mapping tests with real kdeglobals fixtures
- KDE fonts: Qt5/Qt6 weight conversion, all populate paths
- GNOME reader: font parsing, text scale computation, portal color conversion
- Sprite sheet parser: excellent edge cases
- Preset loading: thorough integration tests

### Under-tested areas
- `validate()` range checks (zero negative tests)
- `validate()` cross-field constraints: dialog min <= max (2i)
- `lint_toml()` (only doctest)
- `icon_name()` exhaustive mapping per icon set (2b)
- `resolve()` individual inheritance rules (tested indirectly, not directly)
- `resolve()` idempotency guarantee (documented but untested) (2j)
- `SystemTheme::with_overlay()` accent re-derivation chain
- `from_toml_with_base()` public API
- `Rgba::from_f32()` / `to_f32_array()` quantization round-trip (7g)
- Cross-platform `detect_*` functions (inherently hard to test)

### Unnecessary/bloated tests
- `resolved.rs` duplicate construction tests (~400 lines for same thing)
- Duplicated preset tests between integration and unit
- `dialog_order.rs` has 6 tests for a 2-variant enum

---

## 8. Additional Preset Value Mismatches (Second Pass)

### 8a. Live presets systematically mirror all bugs from full presets

All live presets (`*-live.toml`) duplicate every geometry value from their
full counterparts with no sync mechanism. Every fix to issues 1a-1u must
be applied to BOTH the full preset AND the live preset manually.

**Recommended:** After fixing all preset values, add a test asserting each
live preset's geometry fields match the corresponding full preset.

### 8b. macOS `switch.track_radius = 10` -- should be 11

**File:** `src/presets/macos-sonoma.toml:162,359`

`track_height = 22.0` and `track_radius = 10.0`. For a pill-shaped NSSwitch
the radius should be half the height (22/2 = 11). Current value produces a
slightly squared-off track. **Also affects:** `macos-sonoma-live.toml:97,231`.

**Recommended:** Fix to 11.0.

### 8c. Windows 11 `button.min_height = 27` -- platform-facts says 32

**File:** `src/presets/windows-11.toml:80,276`

WinUI3 `Button.MinHeight = 32`. The preset uses 27 (compact mode value).
**Also affects:** `windows-11-live.toml:35,169`.

**Recommended:** Fix to 32.0.

### 8d. Windows 11 `menu.item_height = 23` -- platform-facts says 36

**File:** `src/presets/windows-11.toml:125,324`

WinUI3 `MenuFlyoutItemHeight = 36`. The preset uses 23 (compact/dense value).
**Also affects:** `windows-11-live.toml:77,211`.

**Recommended:** Fix to 36.0.

### 8e. Windows 11 `toolbar.height = 64` -- should be 48

**File:** `src/presets/windows-11.toml:137,336`

WinUI3 `CommandBar.Height = 48` (default collapsed state). The preset uses
64 (expanded state). **Also affects:** `windows-11-live.toml:88,222`.

**Recommended:** Fix to 48.0.

---

## 9. Additional Unit Test Issues (Second Pass)

### 9a. `preset_loading.rs` font test does not check weight

**File:** `tests/preset_loading.rs:139-163`

Asserts `font.family.is_some()` and `font.size > 0` but never checks
`font.weight.is_some()`. A preset missing weight would pass this test.

**Recommended:** Add `assert!(variant.defaults.font.weight.is_some())`.

### 9b. No test that `bundled_icon_svg()` returns valid SVG content

**File:** `src/model/bundled.rs`

Bundled icons are included via `include_bytes!()` with no validation that
the bytes contain valid SVG. A corrupted file produces silent broken icons.

**Recommended:** Add test iterating `IconRole::ALL` x `{Material, Lucide}`,
asserting result starts with `<svg` or `<?xml`.

### 9c. `is_empty_all_structs` only tests 4 of 30+ structs

**File:** `tests/merge_behavior.rs:212-218`

Checks `ThemeDefaults`, `FontSpec`, `ThemeSpacing`, `ThemeVariant` but skips
all 27 widget theme structs. Any could have broken `is_empty()`.

**Recommended:** Extend to all structs generated by `define_widget_pair!`.

### 9d. No negative test for `from_toml_with_base()` with unknown base

**File:** `src/model/mod.rs:443-448`

Never tested with an invalid base name. Should return `Error::Unavailable`.

**Recommended:** Add `assert!(ThemeSpec::from_toml_with_base(..., "nonexistent").is_err())`.

### 9e. Serde round-trip `fully_populated_variant()` omits most widget sections

**File:** `tests/serde_roundtrip.rs:15-101`

Builds a "fully populated" variant but omits `text_scale`, `icon_set`,
`slider`, `progress_bar`, `tab`, `menu`, `switch`, `dialog`, `spinner`,
`combo_box`, `segmented_control`, `card`, `expander`, `link`, `toolbar`,
`splitter`, `checkbox`. A serde bug in any would go undetected.

**Recommended:** Use an actual preset (which has all fields) or extend
the helper.

### 9f. `dark_backgrounds_are_darker` uses naive RGB sum

**File:** `tests/preset_loading.rs:262-290`

Computes `r + g + b` instead of BT.601 weighted luminance. Could produce
false positives for saturated colors.

**Recommended:** Low priority. Add comment noting the limitation.

---

## 10. Additional Code Quality Issues (Second Pass)

### 10a. `xdg_config_dir()` inconsistency with `kde/mod.rs`

**File:** `src/model/icons.rs:637-639` vs `src/kde/mod.rs:141-156`

`xdg_config_dir()` falls back to `/tmp/.config` when `$HOME` is unset, but
`read_kcmfontsrc_key()` in kde/mod.rs correctly returns `None`. Inconsistent
error handling for the same env var.

**Recommended:** Make `xdg_config_dir()` return `Option<PathBuf>`. Subsumes 5f.

### 10b. Duplicated `unpremultiply_alpha()` across 3 files

**Files:** `src/sficons.rs:96-105`, `src/winicons.rs:108-117`, `src/rasterize.rs:76-85`

Identical function copy-pasted. A bug fix in one must be applied to all three.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Extract into shared `color.rs` utility | DRY; single place for fixes | One more public utility |
| B | Keep duplicated | Each module self-contained | 3 copies to maintain |

**Recommended:** A.

### 10c. `detect_platform()` in `presets.rs` duplicates DE detection logic

**File:** `src/presets.rs:124-148`

Simplified version of `detect_linux_de()` that only cares about KDE vs
non-KDE. One of the 5 duplicated `XDG_CURRENT_DESKTOP` read sites (3a).

**Recommended:** Use `detect_linux_de()` internally, reinforcing issue 3a.

---

## 11. Architecture Issues (Second Pass)

### 11a. No `resolve()` safety net for `defaults.line_height`

**File:** `src/resolve.rs:167-197`

If a minimal user theme omits `line_height`, resolve() does not fill it,
and `resolve_text_scale_entry()` cannot derive text scale line heights.
Validation then fails on the missing field. Bundled presets all set it, so
this only affects truly minimal custom themes.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add safety net: `if self.defaults.line_height.is_none() { self.defaults.line_height = Some(1.2); }` | Minimal themes work out of box | Magic default value |
| B | Keep current (require explicit line_height) | Explicit | Confusing validation error for minimal themes |

**Recommended:** A. Document 1.2 as the default multiplier.

### 11b. Missing resolve rule for `dialog.button_order` (reinforces 5c)

**File:** `src/resolve.rs` (absent)

Issue 5c proposed adding a resolve rule as prerequisite for removing
`button_order` from community presets. Emphasizing: the resolve rule is
completely absent, not merely missing for one scenario. Any user theme
omitting `button_order` without inheriting from a preset fails validation.

**Recommended:** Add `platform_button_order()` to `resolve_safety_nets()`.

---

## 12. Corrections to First-Pass Issues

### 12a. Issue 2f: `Io` variant `source()` not tested

The gap is specifically: no test that `Error::Io(..)` returns `Some` from
`std::error::Error::source()`. The existing `from_io_error` test at
`error.rs:210` tests construction but not the `source()` method.

### 12b. Issue 3a: 5 call sites count may need re-verification

The exact count depends on how many `#[cfg(test)]` sites are included.
The recommendation to extract a helper remains valid regardless.

---

## 13. Additional Missing Test Scenarios (Second Pass)

### 13a. No test running `lint_toml()` against all 16 presets

Running lint on all actual preset TOMLs would catch field-name drift
between struct definitions and `FIELD_NAMES` constants.

**Recommended:** Add `for name in PRESET_ENTRIES { assert!(warnings.is_empty()); }`.

### 13b. No test for `pick_variant()` cross-fallback behavior

**File:** `src/model/mod.rs:277-283`

`pick_variant(true)` should prefer dark but fall back to light. No test
verifies the fallback case (preferred variant is `None`).

**Recommended:** Add tests with themes having only one variant.

### 13c. No test for `into_variant()` consuming fallback

**File:** `src/model/mod.rs:302-308`

Consuming counterpart to `pick_variant()`. No tests verify fallback.

**Recommended:** Add parallel tests to 13b.

---

## 14. Preset Color/Radius Issues (Third Pass)

### 14b. Solarized `border` == `surface` (invisible borders)

**File:** `src/presets/solarized.toml:16,214` (light and dark)

Light: `surface = "#eee8d5"`, `border = "#eee8d5"` -- identical.
Dark: `surface = "#073642"`, `border = "#073642"` -- identical.
Elements on surface backgrounds have invisible borders.

**Recommended:** Light border -> `#93a1a1` (Base1). Dark border -> `#586e75` (Base01).

### 14c. Solarized `separator.color` == `surface` (invisible separators)

**File:** `src/presets/solarized.toml:150,348`

Same colors as border/surface. Separators are completely invisible.

**Recommended:** Fix alongside 14b.

### 14d. Gruvbox/Solarized/One Dark `radius_lg` == `radius` (no tier distinction)

**Files:** `src/presets/gruvbox.toml:37-38`, `solarized.toml:37-38`, `one-dark.toml:37-38`

All three have `radius = 8.0` and `radius_lg = 8.0`. Dialogs/cards get
same corners as buttons/inputs, defeating the two-tier radius system.

**Recommended:** Set `radius_lg = 12.0` (consistent with Catppuccin/Nord/Dracula).

---

## 15. Missing Resolve Safety Nets (Third Pass)

### 15b. No resolve rule for `accent_foreground` when missing

**File:** `src/resolve.rs:251-268`

If user theme sets `accent` but omits `accent_foreground`, validation
fails. Every preset sets it explicitly but minimal themes hit this.

**Recommended:** Default to `#ffffff` in `resolve_safety_nets()`.

### 15c. No resolve safety net for `shadow` color

If omitted, validation fails. Reasonable default: `#00000040`.

### 15d. No resolve safety net for `disabled_foreground`

If omitted, validation fails. Reasonable default: derive from `muted`.

---

## 16. Subtle Code Issues (Third Pass)

### 16c. NaN silently passes all range checks in `validate()`

**File:** `src/resolve.rs:139-165`

`check_range_f32`, `check_positive`, `check_non_negative` all pass NaN
because `NAN < min` and `NAN > max` are both `false`. A NaN geometry
field would cause rendering artifacts.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add `value.is_nan()` guard to each check function | Catches NaN; 4 lines | Trivial effort |
| B | Add single `check_finite()` helper | DRY | One more helper |

**Recommended:** A. Defensive, low cost.

---

## 17. Missing Test Patterns (Third Pass)

### 17a. No test for `merge()` name preservation with empty base name

**File:** `src/model/mod.rs:255-269`

Merge keeps empty base name over non-empty overlay name. No test documents
this edge case.

### 17b. No test for `accent -> selection -> selection_inactive` derivation chain

**File:** `src/resolve.rs:253-268`

The three-step derivation chain is the most complex internal chain. No test
verifies it end-to-end.

### 17c. No test for `title_bar_background <- surface` (not `background`)

**File:** `src/resolve.rs:316-318`

Title bar inherits from `surface` while window background inherits from
`background`. No test documents this distinction.

---

## Priority Summary

**Note:** Every preset mismatch marked with "Also affects" requires updating BOTH the full preset and the corresponding live preset in lockstep.

| # | Issue | Severity | Effort | Best Fix |
|---|-------|----------|--------|----------|
| 1j | macOS button_order leading vs trailing | high | trivial | Fix preset to `trailing_affirmative` |
| 1u | iOS button_order leading vs trailing | high | trivial | Fix preset to `trailing_affirmative` |
| 4a | DialogButtonOrder doc incorrect for macOS | high | trivial | Fix doc: Leading = KDE only |
| 1a | win11 dialog max_width 560 vs 548 | medium | trivial | Fix to 548 |
| 1b | win11 dialog min_height 140 vs 184 | medium | trivial | Fix to 184 |
| 1c | win11 dialog max_height 600 vs 756 | medium | trivial | Fix to 756 |
| 1d | win11 spinner stroke_width 2 vs 4 | medium | trivial | Fix to 4.0 |
| 1f | win11 spinner diameter 24 vs 32 | medium | trivial | Fix to 32 |
| 1g | win11 expander header_height 40 vs 48 | medium | trivial | Fix to 48 |
| 1i | adwaita dialog button_spacing 8 vs 12 | medium | trivial | Fix to 12 |
| 1l | win11 menu icon_spacing 8 vs 12 | medium | trivial | Fix to 12 |
| 1m | win11 combo_box min_width 120 vs 64 | medium | trivial | Fix to 64 |
| 1p | kde combo_box padding_horizontal 12 vs 6 | medium | trivial | Fix to 6 |
| 1q | kde combo_box arrow_area_width 28 vs 20 | medium | trivial | Fix to 20 |
| 1r | adwaita expander arrow_size 12 vs 16 | medium | trivial | Fix to 16 |
| 1s | adwaita expander header_height 40 vs 50 | medium | trivial | Fix to 50 |
| 1t | adwaita tab padding_vertical 4 vs 3 | low | trivial | Fix to 3 |
| 2a | missing validate() range-check tests | medium | small | Add negative tests |
| 2b | missing exhaustive icon_name test | medium | small | Add IconRole::ALL loop test |
| 5c | community presets hardcode button_order | medium | small | Omit + add resolve rule |
| 3b | lint_toml hardcoded field lists drift | medium | medium | Wire up FIELD_NAMES constants |
| 6b | from_file doc says wrong error variant | medium | trivial | Fix doc to say Error::Io |
| 1e | win11 tooltip padding_horizontal 8 vs 9 | low | trivial | Fix to 9 |
| 2c | test duplication preset loading | low | medium | Remove unit-level duplicates |
| 2d | resolved.rs construction-only tests | low | medium | Replace with behavioral tests |
| 2e | missing lint_toml test | low | small | Add regression test |
| 2f | error.rs test name misleading | low | trivial | Split into two tests |
| 2h | missing from_toml_with_base test | low | small | Add integration test |
| 3a | repeated env var pattern (5 sites) | low | small | Extract helper fn |
| 4b | IconSet::Default is Linux-only | low | trivial | Add doc comment |
| 4c | Rgba::default() is transparent black | low | trivial | Add doc comment |
| 4d | No Display for IconRole/IconSet | low | small | Add Display impls |
| 5e | Verify MSRV >= 1.87 for let chains | low | trivial | Verify workspace Cargo.toml |
| 6a | presets.rs doc grouping confusing | low | trivial | Simplify wording |
| 7f | No test for with_overlay re-derivation | low | small | Add integration test |
| 2i | Missing validate() cross-field min/max check | medium | trivial | Add check_min_max helper |
| 2j | Missing resolve() idempotency test | medium | trivial | Add resolve-twice-assert-eq test |
| 5f | xdg_config_dir /tmp fallback when HOME unset | low | trivial | Return "hicolor" early |
| 7g | Rgba f32 quantization undocumented | low | trivial | Add test + doc comment |
| 8b | macOS switch track_radius 10 vs 11 | medium | trivial | Fix to 11.0 |
| 8c | win11 button min_height 27 vs 32 | medium | trivial | Fix to 32.0 |
| 8d | win11 menu item_height 23 vs 36 | medium | trivial | Fix to 36.0 |
| 8e | win11 toolbar height 64 vs 48 | medium | trivial | Fix to 48.0 |
| 8a | Live preset sync test needed | low | small | Add sync assertion test |
| 9a | preset_loading font test missing weight check | low | trivial | Add weight assertion |
| 9b | No bundled SVG content validation test | medium | small | Add SVG header test |
| 9c | is_empty tests cover only 4 of 30+ structs | low | small | Extend to all structs |
| 9d | from_toml_with_base no negative test | low | trivial | Add error-path test |
| 9e | serde roundtrip omits most widget sections | medium | small | Use actual preset |
| 10a | xdg_config_dir inconsistency with kde | low | trivial | Return Option<PathBuf> |
| 10b | unpremultiply_alpha duplicated 3 times | low | small | Extract to shared utility |
| 10c | detect_platform duplicates DE detection | low | trivial | Use detect_linux_de() |
| 11a | No resolve() safety net for line_height | medium | trivial | Add 1.2 default |
| 11b | No resolve() rule for button_order | medium | small | Add platform_button_order() |
| 13a | lint_toml not tested against all presets | low | small | Add regression test |
| 13b | pick_variant fallback untested | low | trivial | Add fallback tests |
| 14b | Solarized border == surface (invisible) | medium | trivial | Use distinct palette colors |
| 14c | Solarized separator == surface (invisible) | medium | trivial | Fix alongside 14b |
| 14d | Gruvbox/Solarized/OneDark radius_lg == radius | low | trivial | Set radius_lg = 12.0 |
| 15b | No resolve rule for accent_foreground | medium | trivial | Default to #ffffff |
| 15c | No resolve safety net for shadow | low | trivial | Default to #00000040 |
| 15d | No resolve safety net for disabled_foreground | low | trivial | Derive from muted |
| 16c | NaN passes validate() range checks | medium | trivial | Add is_nan() guard |
| 17a | merge() name preservation edge case untested | low | trivial | Add test |
| 17b | accent->selection->selection_inactive chain untested | medium | trivial | Add derivation test |
| 17c | title_bar_background <- surface distinction untested | low | trivial | Add inheritance test |
