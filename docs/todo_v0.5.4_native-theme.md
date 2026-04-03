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

## 18. Preset Color Mismatches (Fourth Pass)

### 18a. Windows 11 dark `accent_foreground = "#ffffff"` -- platform-facts says `#000000`

**File:** `src/presets/windows-11.toml:232`

platform-facts.md Appendix line 1389: `TextOnAccentFillColorPrimary D #000000`.
Preset uses `#ffffff`. Text on accent backgrounds in dark mode has **reversed
contrast** -- white on light blue instead of black on light blue.

**Impact:** HIGH. This is the most severe color correctness issue found.

**Does NOT affect live preset** (live presets contain geometry only, no colors).

### 18b. Windows 11 dark `danger = "#ff9999"` -- should be `#ff99a4`

**File:** `src/presets/windows-11.toml:218`

platform-facts.md Appendix line 1376: `SystemFillColorCritical D #ff99a4`.

**Does NOT affect live preset** (no colors in live).

### 18c. Windows 11 dark `warning = "#f0c239"` -- should be `#fce100`

**File:** `src/presets/windows-11.toml:220`

platform-facts.md Appendix line 1377: `SystemFillColorCaution D #fce100`.

**Does NOT affect live preset** (no colors in live).

---

## 19. Missing Text Scale Weights (Fourth Pass)

All platform presets rely on `defaults.font.weight = 400` for text_scale
entries that lack an explicit weight. Platform-facts.md SS2.19 documents
heavier weights for headings and titles.

### 19a. Adwaita missing `text_scale.dialog_title.weight` -- should be 800

GNOME `.title-2` weight = 800. Resolves to 400.

### 19b. Adwaita missing `text_scale.display.weight` -- should be 800

GNOME `.title-1` weight = 800. Resolves to 400.

### 19c. macOS missing `text_scale.section_heading.weight` -- should be 700

macOS `.headline` weight = 700 (Bold). Resolves to 400.

### 19d. Windows 11 missing `text_scale.section_heading.weight` -- should be 600

Fluent Subtitle weight = 600 (SemiBold). Resolves to 400.

### 19e. Windows 11 missing `text_scale.dialog_title.weight` -- should be 600

Fluent Title weight = 600 (SemiBold). Resolves to 400.

### 19f. Windows 11 missing `text_scale.display.weight` -- should be 600

Fluent Display weight = 600 (SemiBold). Resolves to 400.

**Recommended for all 19a-19f:** Add explicit `weight` to the relevant
`text_scale` sections in each preset. Also update corresponding live presets.

---

## 20. Additional Geometry/Padding Mismatches (Fourth Pass)

### 20a. macOS `dialog.button_spacing = 8` -- should be 12

**File:** `src/presets/macos-sonoma.toml:170,368`

platform-facts.md SS2.22 line 1194. **Also affects:** live preset.

### 20b. macOS `dialog.content_padding = 24` -- should be 20

**File:** `src/presets/macos-sonoma.toml:169,367`

platform-facts.md SS2.22 line 1193. **Also affects:** live preset.

### 20c. KDE Breeze `dialog.button_spacing = 8` -- should be 6

**File:** `src/presets/kde-breeze.toml:170,368`

platform-facts.md SS2.22: KDE `Layout_DefaultSpacing` = 6.

### 20d. KDE Breeze `dialog.content_padding = 24` -- should be 10

**File:** `src/presets/kde-breeze.toml:169,367`

platform-facts.md SS2.22: KDE `Layout_TopLevelMarginWidth` = 10.

### 20e. Adwaita `dialog.radius` inherits 15 -- should be 18

**File:** `src/presets/adwaita.toml` (absent; resolves from `radius_lg = 15.0`)

platform-facts.md SS2.22: AdwAlertDialog `$alert_radius: 18px`.

### 20f. Adwaita `button.icon_spacing = 6` -- should be 8

**File:** `src/presets/adwaita.toml:89,294`

platform-facts.md SS2.3 line 975.

### 20g. Adwaita `menu.padding_vertical = 4` -- should be 0

**File:** `src/presets/adwaita.toml:134,339`

platform-facts.md SS2.6: `padding: 0 12px`.

### 20h. Adwaita `combo_box.arrow_size = 12` -- should be 16

**File:** `src/presets/adwaita.toml:194,399`

platform-facts.md SS2.24: GtkDropDown arrow = 16px.

### 20i. Adwaita `combo_box.padding_horizontal = 12` -- should be 10

**File:** `src/presets/adwaita.toml:193,398`

platform-facts.md SS2.24: inherits button padding = 10.

### 20j. Adwaita `scrollbar.min_thumb_height = 30` -- should be 40

**File:** `src/presets/adwaita.toml:114,319`

platform-facts.md SS2.8 line 1048.

### 20k. Windows 11 `menu.padding_horizontal = 12` -- should be 11

**File:** `src/presets/windows-11.toml:126,325`

platform-facts.md SS2.6: `MenuFlyoutItemThemePadding=11,8,11,9`.

### 20l. Windows 11 `tab.padding_horizontal = 12` -- should be 8

**File:** `src/presets/windows-11.toml:121,319`

platform-facts.md SS2.11: `TabViewItemHeaderPadding=8,3,4,3`.

### 20m. Windows 11 `tab.padding_vertical = 4` -- should be 3

**File:** `src/presets/windows-11.toml:122,320`

platform-facts.md SS2.11: vertical = 3.

### 20n. KDE Breeze `button.padding_vertical = 6` -- should be 5

**File:** `src/presets/kde-breeze.toml:82,280`

platform-facts.md SS2.3 line 973.

### 20o. KDE Breeze `input.padding_vertical = 6` -- should be 3

**File:** `src/presets/kde-breeze.toml:90,288`

platform-facts.md SS2.4 line 995.

### 20p. macOS `frame_width = 1.0` -- platform-facts says 0.5

**File:** `src/presets/macos-sonoma.toml:38,238`

platform-facts.md SS2.1.6 line 919 (measured, lower confidence).

**All of 20a-20p also affect corresponding live presets.**

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
| 18a | win11 dark accent_foreground #fff vs #000 | **high** | trivial | Fix to #000000 |
| 18b | win11 dark danger #ff9999 vs #ff99a4 | medium | trivial | Fix hex |
| 18c | win11 dark warning #f0c239 vs #fce100 | medium | trivial | Fix hex |
| 19a | adwaita text_scale dialog_title weight 400 vs 800 | medium | trivial | Add weight = 800 |
| 19b | adwaita text_scale display weight 400 vs 800 | medium | trivial | Add weight = 800 |
| 19c | macOS text_scale section_heading weight 400 vs 700 | medium | trivial | Add weight = 700 |
| 19d | win11 text_scale section_heading weight 400 vs 600 | medium | trivial | Add weight = 600 |
| 19e | win11 text_scale dialog_title weight 400 vs 600 | medium | trivial | Add weight = 600 |
| 19f | win11 text_scale display weight 400 vs 600 | medium | trivial | Add weight = 600 |
| 20a | macOS dialog button_spacing 8 vs 12 | medium | trivial | Fix to 12 |
| 20b | macOS dialog content_padding 24 vs 20 | medium | trivial | Fix to 20 |
| 20c | kde dialog button_spacing 8 vs 6 | medium | trivial | Fix to 6 |
| 20d | kde dialog content_padding 24 vs 10 | medium | trivial | Fix to 10 |
| 20e | adwaita dialog radius 15 vs 18 | medium | trivial | Add dialog radius = 18 |
| 20f | adwaita button icon_spacing 6 vs 8 | low | trivial | Fix to 8 |
| 20g | adwaita menu padding_vertical 4 vs 0 | low | trivial | Fix to 0 |
| 20h | adwaita combo_box arrow_size 12 vs 16 | medium | trivial | Fix to 16 |
| 20i | adwaita combo_box padding_horizontal 12 vs 10 | low | trivial | Fix to 10 |
| 20j | adwaita scrollbar min_thumb_height 30 vs 40 | low | trivial | Fix to 40 |
| 20k | win11 menu padding_horizontal 12 vs 11 | low | trivial | Fix to 11 |
| 20l | win11 tab padding_horizontal 12 vs 8 | medium | trivial | Fix to 8 |
| 20m | win11 tab padding_vertical 4 vs 3 | low | trivial | Fix to 3 |
| 20n | kde button padding_vertical 6 vs 5 | low | trivial | Fix to 5 |
| 20o | kde input padding_vertical 6 vs 3 | medium | trivial | Fix to 3 |
| 20p | macOS frame_width 1.0 vs 0.5 | low | trivial | Fix to 0.5 (lower confidence) |

---

## 21. Additional Findings (Fifth Pass -- 2026-04-03)

Fresh review of every `.rs` and `.toml` file against platform-facts.md.

### 21a. KDE Breeze `focus_ring_width = 1.0` -- platform-facts says `1.001px` stroke + `2px` margin

**Category:** preset-value
**Severity:** low
**File(s):** `src/presets/kde-breeze.toml:43,240`

platform-facts.md SS2.1.5 line 910: "KDE: Breeze: 1.001px (stroke); 2px margin".
The model field `focus_ring_width` is the visible stroke width, so 1.0 is correct
(rounded from 1.001). The margin is captured in `focus_ring_offset = 2.0`. No change
needed.

**Verdict:** Correct. The 1.001px is an anti-aliasing trick; 1.0 is the right integer
approximation.

### 21b. `DialogButtonOrder` doc still says "macOS, KDE style" for `LeadingAffirmative`

**Category:** api-bug
**Severity:** high
**File(s):** `src/model/dialog_order.rs:19`

The doc comment reads:
```
/// Affirmative button at the leading (left) end -- macOS, KDE style.
LeadingAffirmative,
```

platform-facts.md SS2.22 line 1195-1203: macOS places primary action rightmost
(trailing). Only KDE uses leading affirmative. This is already tracked as issue 4a,
but the fix must include changing line 19 in `dialog_order.rs`.

**Verdict:** Already tracked in 4a. Confirming exact file:line for the fix.

### 21c. Windows 11 `progress_bar.height = 3` -- platform-facts says 1 (track) or 3 (control min)

**Category:** preset-value
**Severity:** low
**File(s):** `src/presets/windows-11.toml:114,313`

platform-facts.md SS1.2.4: "WinUI3: ProgressBarMinHeight=3, ProgressBarTrackHeight=1".
The preset uses 3.0 which is the control minimum, not the track height. The model
field `height` semantically maps to the visual bar height. WinUI3's actual rendered
bar is 1px inside a 3px control.

**Solution Options:**

1. **Keep 3 (control minimum)**
   - *Pros:* Matches `ProgressBarMinHeight`; avoids 1px bars that may be invisible
   - *Cons:* Thicker than the actual rendered bar

2. **Fix to 1 (track height)**
   - *Pros:* Matches `ProgressBarTrackHeight`
   - *Cons:* Very thin; may be invisible on some displays

**Best Solution:** Keep 3. The MinHeight value is the correct semantic match for a
field called `height`. Document the distinction in a TOML comment.

### 21d. Adwaita `checkbox.spacing = 8` -- platform-facts inconsistent

**Category:** preset-value
**Severity:** low
**File(s):** `src/presets/adwaita.toml:101,306`

platform-facts.md SS2.5 line 1004: "GNOME: (Adwaita CSS): 8". The preset matches.
Confirmed correct.

### 21e. KDE Breeze preset missing `menu.item_height` -- platform-facts says font-derived

**Category:** preset-value
**Severity:** low
**File(s):** `src/presets/kde-breeze.toml:125,324`

The preset has `item_height = 28.0`. platform-facts.md SS2.6 line 1021:
"KDE: (none) -- sizes to font". KDE has no fixed item height; it is font-derived.
The preset value 28 is a reasonable approximation (10pt Noto Sans at ~18px ascent
+ 2*4px vertical padding + 2px margin = ~28px).

**Verdict:** Acceptable. Add TOML comment explaining the derivation.

### 21f. macOS `list.padding_vertical = 4` -- platform-facts says 4

**Category:** preset-value
**Severity:** low (correct)
**File(s):** `src/presets/macos-sonoma.toml:134,332`

platform-facts.md SS2.15 line 1127: "macOS: 4 (measured) (24-16)/2". Confirmed
correct.

### 21g. macOS `toolbar.padding = 8` -- platform-facts says 8

**Category:** preset-value
**Severity:** low (correct)
**File(s):** `src/presets/macos-sonoma.toml:139,337`

platform-facts.md SS2.13 line 1103: "macOS: 8 (measured) NSToolbar". Confirmed
correct.

### 21h. Missing test: `ThemeVariant::is_empty()` never tested directly

**Category:** test-gap
**Severity:** low
**File(s):** `src/model/mod.rs:184`

`ThemeVariant::is_empty()` is generated by `impl_merge!` but never tested directly.
The `is_empty()` methods on individual widget structs are tested (all 25), but the
top-level `ThemeVariant::is_empty()` is not. A default `ThemeVariant` should be
empty; one with any field set should not be.

**Solution Options:**

1. **Add 2 tests: default is_empty, set one field is not_empty**
   - *Pros:* Documents behavior, catches regression
   - *Cons:* Trivial effort

**Best Solution:** Option 1.

### 21i. Missing test: `ThemeSpec::is_empty()` not tested

**Category:** test-gap
**Severity:** low
**File(s):** `src/model/mod.rs:310-312`

No test for `ThemeSpec::is_empty()`. The method returns true when both
`light` and `dark` are `None`. Simple to test.

**Best Solution:** Add test.

### 21j. `kde/mod.rs` `is_dark_theme()` returns `false` when colors are missing

**Category:** correctness
**Severity:** low
**File(s):** `src/kde/mod.rs` (the `is_dark_theme` function)

When `[Colors:Window] BackgroundNormal` is missing from kdeglobals,
`is_dark_theme()` cannot determine the mode. It returns `false` (light mode),
which is a reasonable default. The Breeze default color scheme is light, so
this fallback is correct for the common case.

**Verdict:** Correct behavior. Not a bug.

### 21k. macOS preset `icon_sizes.toolbar = 24` -- platform-facts says 32 (regular) or 24 (small)

**Category:** preset-value
**Severity:** low
**File(s):** `src/presets/macos-sonoma.toml:71,269`

platform-facts.md SS2.1.8 line 937: "macOS: 32pt (reg) / 24 (sm)". The preset
uses 24 (small mode). Modern macOS apps predominantly use small toolbar mode;
32pt is legacy. The choice of 24 is defensible.

**Verdict:** Acceptable. Add TOML comment noting this uses the small toolbar size.

### 21l. Adwaita preset `icon_sizes.dialog = 22` -- platform-facts says no GNOME native value

**Category:** preset-value
**Severity:** low
**File(s):** `src/presets/adwaita.toml:73,278`

platform-facts.md SS2.1.8 line 940: "GNOME: (none) -- 48 (GTK3 legacy)".
The preset uses 22, which is a reasonable application-level default. Not documented
by GNOME.

**Verdict:** Acceptable. This is a preset-supplied value for a field that GNOME
does not provide.

### 21m. Adwaita preset `icon_sizes.panel = 20` -- platform-facts says no GNOME native value

**Category:** preset-value
**Severity:** low
**File(s):** `src/presets/adwaita.toml:74,279`

platform-facts.md SS2.1.8 line 941: "GNOME: (none)". The preset uses 20 as a
reasonable default. Not a mismatch -- GNOME simply does not define this.

**Verdict:** Correct.

### 21n. Windows 11 `icon_sizes.dialog = 22` and `panel = 20` -- platform-facts says none for both

**Category:** preset-value
**Severity:** low
**File(s):** `src/presets/windows-11.toml:73-74,270-271`

platform-facts.md SS2.1.8 lines 940-941: "Windows: (none)" for both dialog and panel.
The preset values are reasonable application defaults.

**Verdict:** Correct. These are preset-supplied defaults where the OS has no opinion.

### 21o. `resolve.rs` Phase 2 safety net: `input.caret <- defaults.foreground` -- platform-facts says accent for KDE/GNOME

**Category:** api-bug
**Severity:** medium
**File(s):** `src/resolve.rs:274-276`

The safety net in `resolve_safety_nets()`:
```rust
if self.input.caret.is_none() {
    self.input.caret = self.defaults.foreground;
}
```

platform-facts.md SS2.4 line 989:
- KDE: `[Colors:View] DecorationFocus` (the accent/focus color)
- GNOME: `@accent_color`
- macOS: `textInsertionPointColor` (accent-based since macOS 14)
- Windows: `foreground` (system default)

The safety net uses `foreground` which is only correct for Windows. KDE and GNOME
readers set `input.caret` explicitly from the accent/focus color, so the safety
net only fires for user themes that omit caret. Using `foreground` is defensible
as a universal fallback (it always produces visible text), but `accent` would
match 3 of 4 platforms.

**Solution Options:**

1. **Change safety net to `accent` instead of `foreground`**
   - *Pros:* Matches macOS, KDE, and GNOME behavior
   - *Cons:* May produce low-contrast caret on accent-colored backgrounds

2. **Keep `foreground`**
   - *Pros:* Always produces visible caret; matches Windows
   - *Cons:* Does not match majority platform behavior

**Best Solution:** Keep `foreground`. The safety net is a last-resort fallback,
and `foreground` is guaranteed to be visible against `background`. The platform
readers already set the correct accent-based value.

### 21p. `gnome/mod.rs` `compute_text_scale()` is missing `section_heading`

**Category:** api-bug
**Severity:** medium
**File(s):** `src/gnome/mod.rs:254-273`

The function computes `caption`, `dialog_title`, and `display` but sets
`section_heading: None`. However, the Adwaita preset TOML sets
`[light.text_scale.section_heading] weight = 700` (line 77). When building a
GNOME theme, the Adwaita preset base provides section_heading.weight = 700, and
then `compute_text_scale()` returns an overlay with section_heading = None,
which does NOT clear the base value (merge semantics: None preserves base). So
the final result correctly has section_heading.weight = 700 from the base.

platform-facts.md SS1.4.1 line 727: "`.heading`: (inherited), 700". The heading
class uses the base font size with Bold weight. Since section_heading size should
equal the base font size, and the Adwaita preset does not set a size for
section_heading (only weight), `resolve_text_scale_entry()` fills the size from
`defaults.font.size` = 11. This is correct.

**Verdict:** Correct behavior. The None in `compute_text_scale()` preserves the
base preset's weight, and resolve fills the size. Not a bug.

---

## 22. Verification of Previously Identified Issues

All issues from sections 1-20 were re-verified against the source files and
platform-facts.md. Confirmed correct:

- **1a-1u:** All preset value mismatches verified. File:line references match
  current source.
- **2a-2j:** Test gap descriptions verified against current test code.
- **3a-3e:** Code quality issues confirmed present in current source.
- **4a-4d:** API design issues confirmed.
- **5a-5f:** Correctness issues verified.
- **8a-8e:** Second-pass preset mismatches verified.
- **14b-14d:** Community preset visual issues confirmed.
- **18a-18c:** Windows 11 dark color mismatches verified against platform-facts.md
  Chapter 2 status color tables.
- **19a-19f:** Missing text_scale weights confirmed -- Windows 11 and macOS
  presets have NO text_scale sections at all.
- **20a-20p:** Geometry/padding mismatches verified.

No previously identified issues were found to be invalid or already fixed.

---

## Updated Priority Summary (includes new 21x findings)

| # | Issue | Severity | Effort | Best Fix |
|---|-------|----------|--------|----------|
| 18a | win11 dark accent_foreground #fff vs #000 | **high** | trivial | Fix to #000000 |
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
| 18b | win11 dark danger #ff9999 vs #ff99a4 | medium | trivial | Fix hex |
| 18c | win11 dark warning #f0c239 vs #fce100 | medium | trivial | Fix hex |
| 8b | macOS switch track_radius 10 vs 11 | medium | trivial | Fix to 11.0 |
| 8c | win11 button min_height 27 vs 32 | medium | trivial | Fix to 32.0 |
| 8d | win11 menu item_height 23 vs 36 | medium | trivial | Fix to 36.0 |
| 8e | win11 toolbar height 64 vs 48 | medium | trivial | Fix to 48.0 |
| 19a | adwaita text_scale dialog_title weight 400 vs 800 | medium | trivial | Add weight = 800 |
| 19b | adwaita text_scale display weight 400 vs 800 | medium | trivial | Add weight = 800 |
| 19c | macOS text_scale section_heading weight 400 vs 700 | medium | trivial | Add weight = 700 |
| 19d | win11 text_scale section_heading weight 400 vs 600 | medium | trivial | Add weight = 600 |
| 19e | win11 text_scale dialog_title weight 400 vs 600 | medium | trivial | Add weight = 600 |
| 19f | win11 text_scale display weight 400 vs 600 | medium | trivial | Add weight = 600 |
| 20a | macOS dialog button_spacing 8 vs 12 | medium | trivial | Fix to 12 |
| 20b | macOS dialog content_padding 24 vs 20 | medium | trivial | Fix to 20 |
| 20c | kde dialog button_spacing 8 vs 6 | medium | trivial | Fix to 6 |
| 20d | kde dialog content_padding 24 vs 10 | medium | trivial | Fix to 10 |
| 20e | adwaita dialog radius 15 vs 18 | medium | trivial | Add dialog radius = 18 |
| 20h | adwaita combo_box arrow_size 12 vs 16 | medium | trivial | Fix to 16 |
| 20l | win11 tab padding_horizontal 12 vs 8 | medium | trivial | Fix to 8 |
| 20o | kde input padding_vertical 6 vs 3 | medium | trivial | Fix to 3 |
| 2a | missing validate() range-check tests | medium | small | Add negative tests |
| 2b | missing exhaustive icon_name test | medium | small | Add IconRole::ALL loop test |
| 2i | Missing validate() cross-field min/max check | medium | trivial | Add check_min_max helper |
| 2j | Missing resolve() idempotency test | medium | trivial | Add resolve-twice-assert-eq test |
| 5c | community presets hardcode button_order | medium | small | Omit + add resolve rule |
| 9b | No bundled SVG content validation test | medium | small | Add SVG header test |
| 9e | serde roundtrip omits most widget sections | medium | small | Use actual preset |
| 11a | No resolve() safety net for line_height | medium | trivial | Add 1.2 default |
| 11b | No resolve() rule for button_order | medium | small | Add platform_button_order() |
| 14b | Solarized border == surface (invisible) | medium | trivial | Use distinct palette colors |
| 14c | Solarized separator == surface (invisible) | medium | trivial | Fix alongside 14b |
| 15b | No resolve rule for accent_foreground | medium | trivial | Default to #ffffff |
| 16c | NaN passes validate() range checks | medium | trivial | Add is_nan() guard |
| 17b | accent->selection->selection_inactive chain untested | medium | trivial | Add derivation test |
| 3b | lint_toml hardcoded field lists drift | medium | medium | Wire up FIELD_NAMES constants |
| 6b | from_file doc says wrong error variant | medium | trivial | Fix doc to say Error::Io |
| 1e | win11 tooltip padding_horizontal 8 vs 9 | low | trivial | Fix to 9 |
| 1t | adwaita tab padding_vertical 4 vs 3 | low | trivial | Fix to 3 |
| 20f | adwaita button icon_spacing 6 vs 8 | low | trivial | Fix to 8 |
| 20g | adwaita menu padding_vertical 4 vs 0 | low | trivial | Fix to 0 |
| 20i | adwaita combo_box padding_horizontal 12 vs 10 | low | trivial | Fix to 10 |
| 20j | adwaita scrollbar min_thumb_height 30 vs 40 | low | trivial | Fix to 40 |
| 20k | win11 menu padding_horizontal 12 vs 11 | low | trivial | Fix to 11 |
| 20m | win11 tab padding_vertical 4 vs 3 | low | trivial | Fix to 3 |
| 20n | kde button padding_vertical 6 vs 5 | low | trivial | Fix to 5 |
| 20p | macOS frame_width 1.0 vs 0.5 | low | trivial | Fix to 0.5 (lower confidence) |
| 14d | Gruvbox/Solarized/OneDark radius_lg == radius | low | trivial | Set radius_lg = 12.0 |
| 21h | ThemeVariant::is_empty() untested | low | trivial | Add 2 tests |
| 21i | ThemeSpec::is_empty() untested | low | trivial | Add test |
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
| 5f | xdg_config_dir /tmp fallback when HOME unset | low | trivial | Return "hicolor" early |
| 6a | presets.rs doc grouping confusing | low | trivial | Simplify wording |
| 7f | No test for with_overlay re-derivation | low | small | Add integration test |
| 7g | Rgba f32 quantization undocumented | low | trivial | Add test + doc comment |
| 8a | Live preset sync test needed | low | small | Add sync assertion test |
| 9a | preset_loading font test missing weight check | low | trivial | Add weight assertion |
| 9c | is_empty tests cover only 4 of 30+ structs | low | small | Extend to all structs |
| 9d | from_toml_with_base no negative test | low | trivial | Add error-path test |
| 10a | xdg_config_dir inconsistency with kde | low | trivial | Return Option<PathBuf> |
| 10b | unpremultiply_alpha duplicated 3 times | low | small | Extract to shared utility |
| 10c | detect_platform duplicates DE detection | low | trivial | Use detect_linux_de() |
| 13a | lint_toml not tested against all presets | low | small | Add regression test |
| 13b | pick_variant fallback untested | low | trivial | Add fallback tests |
| 15c | No resolve safety net for shadow | low | trivial | Default to #00000040 |
| 15d | No resolve safety net for disabled_foreground | low | trivial | Derive from muted |
| 17a | merge() name preservation edge case untested | low | trivial | Add test |
| 17c | title_bar_background <- surface distinction untested | low | trivial | Add inheritance test |

---

## 23. Second-Pass Deep Audit (new findings only)

Full re-read of every `.rs` file, every `.toml` preset, all test files, and
all 1475 lines of `docs/platform-facts.md`. The following issues are NOT
already covered in sections 1-22.

### 23a. `kde/metrics.rs` does not set `button.padding_vertical` or `input.padding_vertical`

**Category:** api-bug
**Severity:** medium
**File(s):** `src/kde/metrics.rs:8-61`

**Problem:** `populate_widget_sizing()` sets `button.padding_horizontal = 6.0`
(Button_MarginWidth) and `input.padding_horizontal = 6.0` (LineEdit_FrameWidth)
but never sets `button.padding_vertical` or `input.padding_vertical`. These
fields come exclusively from the kde-breeze preset TOML. If a user calls
`from_kde()` directly (without the preset base), these fields remain `None`
and fail validation.

platform-facts.md SS2.3 line 973: "KDE: 5 (measured) Breeze frame+margin"
(button.padding_vertical). SS2.4 line 995: "KDE: 3 (measured) Breeze frame"
(input.padding_vertical). Both are measured values, not named
breezemetrics.h constants, which is why they were omitted from the metrics
module. However, the metrics module already includes measured values for
`list.padding_vertical = 1.0` (ItemView_ItemMarginTop from breezemetrics.h),
so the exclusion is inconsistent.

**Solution Options:**

1. **Add to `populate_widget_sizing()`**
   - *Pros:* Consistent with other metrics already in the function; `from_kde()` pipeline becomes more self-contained
   - *Cons:* Values are measured, not from named constants (but list padding values are sourced from breezemetrics.h constants anyway)

2. **Keep in preset only, document the dependency**
   - *Pros:* No code change
   - *Cons:* `from_kde()` without preset base fails validation for these fields

**Best Solution:** Option 1. Add `button.padding_vertical = Some(5.0)` and
`input.padding_vertical = Some(3.0)` to `populate_widget_sizing()`, consistent
with the existing pattern. Note: `list.padding_vertical` IS sourced from
a named constant (ItemView_ItemMarginTop), so the button/input cases are
genuinely different. Still, adding them improves the self-containedness of
the KDE reader pipeline.

### 23b. `qt5_to_css_weight()` accepts negative values, maps them to 900

**Category:** parsing-bug
**Severity:** low
**File(s):** `src/kde/fonts.rs:8-19`

**Problem:** The `qt5_to_css_weight()` function takes `i32` but the catch-all
`_ => 900` arm matches negative values. Qt5 weight range is 0-99; a negative
value in the font string indicates a corrupted or invalid entry. Mapping it
to 900 (Black) silently produces the heaviest possible weight.

The function is only called from `parse_qt_font_with_weight()` which passes
the raw parsed weight from the kdeglobals font string. A corrupted font
entry with a negative weight (e.g., `-1`) would yield CSS 900 instead of
being rejected.

No existing test covers negative input to `qt5_to_css_weight()`.

**Solution Options:**

1. **Return a sentinel value (e.g., 400) for negative input**
   - *Pros:* Safe default for corrupted data
   - *Cons:* Silently masks corruption

2. **Clamp negative to 0 before matching**
   - *Pros:* Maps to 100 (Thin) which is the minimum valid weight
   - *Cons:* Still silent

3. **Reject at caller: add `if raw_weight < 0 { return None; }` in `parse_qt_font_with_weight()`**
   - *Pros:* Rejects truly invalid data at the earliest point
   - *Cons:* Changes behavior for edge case

**Best Solution:** Option 3. Add `if raw_weight < 0 { return None; }` before
the Qt5/Qt6 branch in `parse_qt_font_with_weight()`. This rejects corrupted
entries at the earliest point, consistent with the existing `size <= 0.0`
rejection at line 38. Add a test for negative weight input.

### 23c. Missing test: `qt5_to_css_weight()` negative input

**Category:** test-gap
**Severity:** low
**File(s):** `src/kde/fonts.rs:111-148`

**Problem:** The test suite covers boundary values 0, 12, 25, 50, 63, 75,
88, and 100, but no test covers negative input. Given issue 23b, a test
for `qt5_to_css_weight(-1)` should be added to document the expected
behavior (currently maps to 900).

**Best Solution:** Add test alongside the fix for 23b.

### 23d. `gnome/mod.rs` `compute_text_scale()` hardcodes `caption.weight = 400` instead of leaving it to inherit

**Category:** api-bug
**Severity:** low
**File(s):** `src/gnome/mod.rs:253-271`

**Problem:** `compute_text_scale()` hardcodes `caption.weight = Some(400)`.
This is correct for the default Adwaita CSS (`.caption` is Regular 400), but
if the user has changed their system font to a non-Regular weight (e.g.,
"Cantarell Light 11" which parses as weight 300), the caption weight will
still be forced to 400, ignoring the user's lighter font preference.

The `dialog_title` and `display` entries correctly leave `weight: None` to
inherit from the Adwaita preset base. Caption forces weight to 400 instead
of following the same inheritance pattern.

platform-facts.md SS1.4.1 line 724: "`.caption`: 82%, 400". The CSS does
specify 400 explicitly, so the hardcoded value is technically correct for
stock Adwaita. But the runtime reader's purpose is to adapt to user settings.

**Solution Options:**

1. **Change caption.weight to None (inherit from base preset / defaults.font.weight)**
   - *Pros:* Consistent with dialog_title/display entries in the same function; respects user font weight override
   - *Cons:* Default behavior unchanged (400) since Adwaita base sets caption weight

2. **Keep hardcoded 400**
   - *Pros:* Matches Adwaita CSS spec exactly
   - *Cons:* Ignores user font weight override; inconsistent with dialog_title/display entries

**Best Solution:** Option 1. Set `weight: None` so the Adwaita preset's
`text_scale.caption` (or defaults.font.weight) is used, consistent
with the other entries in `compute_text_scale()`.

### 23e. `kde/mod.rs` `read_kcmfontsrc_key()` silently returns `None` when HOME is set but XDG_CONFIG_HOME is empty string

**Category:** correctness
**Severity:** low
**File(s):** `src/kde/mod.rs:140-161`

**Problem:** When `XDG_CONFIG_HOME` is set to an empty string, the function
correctly treats it as unset (line 141: `if config_home.is_empty() { None }`)
and falls through to the `HOME`-based path via `.or_else()`. This behavior
is correct per the XDG Base Directory Specification ("If $XDG_CONFIG_HOME
is either not set or empty, a default equal to $HOME/.config should be used").

**Verdict:** Correct. Not a bug. Verified the empty-string handling matches
the XDG spec.

### 23f. `kde/mod.rs` `parse_icon_sizes_from_content()` does not set `dialog` or `panel` icon sizes

**Category:** api-bug
**Severity:** low
**File(s):** `src/kde/mod.rs:172-239`

**Problem:** The function parses icon sizes from the icon theme's index.theme
and derives `small`, `toolbar`, and `large` from directory entries, but
always returns `dialog: None, panel: None` (lines 237-238). platform-facts.md
SS1.3.6 line 686-687: "KDE Dialog DialogDefault: 32px (C++ fallback, Breeze
default matches)", "Panel PanelDefault: 48px (C++ fallback, Breeze default
matches)".

These sizes come from `index.theme` `[Icon Theme]` keys `DialogDefault` and
`PanelDefault`, which are read by KIconLoader in the C++ API but not parsed
by `parse_icon_sizes_from_content()`. The function only inspects per-directory
`Size` and `Context` pairs, not the top-level icon theme metadata keys.

**Solution Options:**

1. **Read `DialogDefault` and `PanelDefault` keys from `[Icon Theme]` section**
   - *Pros:* Matches KDE KIconLoader behavior; fills all 5 icon size fields
   - *Cons:* Adds ~10 lines of key lookup

2. **Keep None, let preset fill them**
   - *Pros:* No change; preset provides the values
   - *Cons:* `from_kde()` returns incomplete icon sizes; custom icon themes with non-default values are ignored

**Best Solution:** Option 1. The keys are simple integer lookups from the same
INI file already being parsed. Add:
```
dialog = ini.get("Icon Theme", "DialogDefault")
    .and_then(|s| s.trim().parse::<u32>().ok())
    .map(|sz| sz as f32);
panel = ini.get("Icon Theme", "PanelDefault")
    .and_then(|s| s.trim().parse::<u32>().ok())
    .map(|sz| sz as f32);
```

### 23g. `gnome/mod.rs` `build_gnome_variant()` sets `dialog.button_order = TrailingAffirmative` for GNOME but no icon_set

**Category:** api-bug
**Severity:** low
**File(s):** `src/gnome/mod.rs:238-239`

**Problem:** The GNOME variant builder explicitly sets
`dialog.button_order = Some(DialogButtonOrder::TrailingAffirmative)` (correct
per platform-facts.md SS2.22) but does not set `icon_set`. The icon_set is
filled later by `resolve()` Phase 5 which calls `system_icon_set()`, and
since GNOME is Linux, this returns `Freedesktop` which is correct. However,
the KDE reader at `src/kde/mod.rs:47` does not set icon_set either.

Both readers rely on the resolve() fallback for icon_set, which is correct
design. This is not a bug -- just noting the consistent pattern for
completeness.

**Verdict:** Correct. Not a bug. Both KDE and GNOME readers correctly rely
on resolve() Phase 5 for icon_set.

### 23h. Windows 11 `combo_box.arrow_area_width = 38` -- platform-facts says 38

**Category:** preset-value
**Severity:** low (correct)
**File(s):** `src/presets/windows-11.toml:189,388`

**Problem:** Checked this because KDE was flagged (28 vs 20) in issue 1q.
platform-facts.md SS2.24 line 1225: "Windows: WinUI3: 38". The preset uses
38.0 for both light and dark. Confirmed correct.

**Verdict:** Correct. No issue.

---

## 24. Third-Pass Deep Audit (new findings only)

Exhaustive cross-reference of all 11 community preset TOMLs, all 4 live vs
static preset pairs, line-by-line resolve.rs edge-case audit, defaults.rs
cross-reference, every test assertion verification, and iOS preset review.
Only genuinely new findings below (not already covered in sections 1-23).

### 24a. All 11 community presets missing `text_scale` sections entirely

**Category:** preset-gap
**Severity:** medium
**File(s):** `src/presets/catppuccin-latte.toml`, `catppuccin-frappe.toml`,
`catppuccin-macchiato.toml`, `catppuccin-mocha.toml`, `dracula.toml`,
`gruvbox.toml`, `nord.toml`, `one-dark.toml`, `solarized.toml`,
`tokyo-night.toml`, `material.toml`

**Problem:** None of the 11 community presets define any `text_scale`
sections (`caption`, `section_heading`, `dialog_title`, `display`). Only
Adwaita (and its live preset) defines `text_scale.section_heading.weight`.
No other platform preset (macOS, Windows 11, KDE, iOS) or community preset
defines any text_scale entries.

When `resolve_text_scale()` runs, it calls `resolve_text_scale_entry()`
for each missing entry, which creates a default `TextScaleEntry` and fills
`size` from `defaults.font.size` and `weight` from `defaults.font.weight`
(both 400 for all community presets). All four text_scale entries thus
resolve to the same `{size: 14.0, weight: 400, line_height: 16.8}` for
community presets using Inter 14.0 at line_height 1.2.

This means:
- `caption` has the same size as body text (should be smaller, ~82-85% per
  platform-facts.md SS2.19)
- `section_heading` has weight 400 (should typically be 600-700 for headings)
- `dialog_title` has weight 400 (should typically be 600-800)
- `display` has the same size as body text (should be larger, ~170-200%)

**Impact:** Any connector using `text_scale` entries for typographic
hierarchy gets flat, undifferentiated text sizes and weights from community
presets.

**Solution Options:**

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add sensible text_scale entries to each community preset based on their design language | Proper typographic hierarchy | 11 files to update |
| B | Add resolve safety nets that derive text_scale sizes from defaults.font.size using standard ratios (caption=0.82x, heading=1.2x, display=2.0x) | Works for all presets automatically; no TOML changes needed | Opinionated defaults |
| C | Leave as-is | No changes | Flat typography |

**Recommended:** B for v0.5.4 (safety-net approach in resolve.rs), then A
for individual presets that want specific weights (e.g., Material should use
500 for headings per Material Design type scale).

### 24b. Community presets all use `icon_set = "lucide"` / `icon_theme = "Lucide"` -- case mismatch risk

**Category:** api-consistency
**Severity:** low
**File(s):** All 10 community presets (excluding material.toml)

**Problem:** The 10 non-Material community presets all set
`icon_set = "lucide"` and `icon_theme = "Lucide"` (uppercase L for theme,
lowercase for set). The material preset uses `icon_set = "material"` and
`icon_theme = "material"` (all lowercase). This is consistent within each
preset, but the `icon_theme` field uses a different casing convention
(`"Lucide"` vs `"material"` vs `"Adwaita"` vs `"breeze"` vs `"sf-symbols"`
vs `"segoe-fluent"`).

The `icon_theme` field is a free-form string used for display/identification
purposes, not for filesystem lookups (freedesktop icon theme lookups use
the system theme, not this field). So mixed casing is cosmetic, not
functional.

**Verdict:** Cosmetic inconsistency only. Not a functional bug. Could be
normalized in a future cleanup pass.

### 24c. Catppuccin Latte dark variant uses Frappe palette colors but light variant `danger_foreground` differs across the four Catppuccin presets

**Category:** preset-consistency
**Severity:** low
**File(s):** `src/presets/catppuccin-latte.toml:20-27` vs
`catppuccin-frappe.toml:20-27` vs `catppuccin-macchiato.toml:20-27` vs
`catppuccin-mocha.toml:20-27`

**Problem:** The four Catppuccin presets' light variants have inconsistent
`danger_foreground` / `warning_foreground` / `success_foreground` /
`info_foreground` values:

- **Latte** (light primary): `danger_foreground = "#ffffff"`,
  `success_foreground = "#ffffff"`, `info_foreground = "#ffffff"` -- white
  foreground on colored backgrounds (good contrast)
- **Frappe/Macchiato/Mocha** (light derived): `danger_foreground = "#4c4f69"`,
  `success_foreground = "#4c4f69"`, `info_foreground = "#4c4f69"` -- dark
  foreground on colored backgrounds

The Frappe/Macchiato/Mocha light variants use the Latte base background
colors but with Frappe/Macchiato/Mocha accent colors. The Frappe accent
colors (e.g., `danger = "#e78284"`) are lighter/pastel compared to Latte's
(`danger = "#d20f39"`). White foreground text on `#e78284` would have poor
contrast, so `#4c4f69` (dark text) is the correct choice for the lighter
danger colors.

**Verdict:** Correct. The different foreground values are appropriate for
the different danger/success/info color intensities. Not a bug.

### 24d. Live presets missing `text_scale` sections that static presets have

**Category:** live-sync
**Severity:** medium
**File(s):** `src/presets/adwaita-live.toml:34,171` vs
`src/presets/adwaita.toml:76,281`

**Problem:** The Adwaita live preset includes `[light.text_scale.section_heading]
weight = 700` and the same for dark -- this matches the static preset.
However, no other live preset (kde-breeze-live, macos-sonoma-live,
windows-11-live) has any text_scale sections, which is consistent with
their static counterparts also lacking text_scale. So there is no sync
gap between live and static for text_scale specifically.

The actual sync gap is that issue 19a-19f identified that the static
platform presets SHOULD have text_scale weight entries but do not. When
those are added to the static presets, they must also be added to the
corresponding live presets.

**Verdict:** No current sync gap. But when issues 19a-19f are fixed,
live presets must be updated in lockstep. Already implicitly covered by
issue 8a's recommendation for a sync test.

### 24e. Live vs static: `adwaita-live.toml` matches `adwaita.toml` geometry except for intentionally omitted color/font/icon fields

**Category:** live-sync (verification)
**Severity:** N/A (no issue found)

**Verification:** Cross-referenced every geometry field in
`adwaita-live.toml` against `adwaita.toml`. All values match exactly:
- `radius = 9.0`, `radius_lg = 15.0`, `frame_width = 1.0`,
  `disabled_opacity = 0.5`, `border_opacity = 0.15`, `shadow_enabled = true`,
  `line_height = 1.21`, `focus_ring_width = 2.0`, `focus_ring_offset = -2.0`
- All widget geometry (button, input, checkbox, etc.) matches.
- Live correctly omits: colors, fonts, icon_set, icon_theme,
  text_scaling_factor, reduce_motion, high_contrast, reduce_transparency.

**Verdict:** In sync. No discrepancy.

### 24f. Live vs static: `kde-breeze-live.toml` matches `kde-breeze.toml` geometry

**Category:** live-sync (verification)
**Severity:** N/A (no issue found)

**Verification:** All geometry values match. Live correctly omits colors,
fonts, icon_set, icon_theme, and accessibility flags.

**Verdict:** In sync.

### 24g. Live vs static: `macos-sonoma-live.toml` matches `macos-sonoma.toml` geometry

**Category:** live-sync (verification)
**Severity:** N/A (no issue found)

**Verification:** All geometry values match.

**Verdict:** In sync.

### 24h. Live vs static: `windows-11-live.toml` matches `windows-11.toml` geometry

**Category:** live-sync (verification)
**Severity:** N/A (no issue found)

**Verification:** All geometry values match.

**Verdict:** In sync.

### 24i. `resolve.rs` `check_non_negative` and `check_positive` pass `Infinity`

**Category:** api-bug
**Severity:** low
**File(s):** `src/resolve.rs:139-151`

**Problem:** Issue 16c covers NaN passing range checks. The same functions
also pass `f32::INFINITY`:
- `check_non_negative(INFINITY, ...)` passes (INFINITY >= 0.0 is true)
- `check_positive(INFINITY, ...)` passes (INFINITY > 0.0 is true)
- `check_range_f32(INFINITY, 0.0, 1.0, ...)` correctly fails (INFINITY > 1.0)

A geometry field set to `f32::INFINITY` would pass all non-negative/positive
checks and produce rendering artifacts. The `check_range_f32` function already
catches Infinity for bounded ranges (opacity, font weight), but unbounded
geometry checks (radius, padding, width, height) do not.

This is partially covered by issue 16c's recommendation to add `is_nan()`
guards. Extending the fix to `!value.is_finite()` (which catches both NaN
and Infinity) would be more thorough.

**Recommended:** Amend issue 16c's solution from `is_nan()` to
`!is_finite()` (catches both NaN and Infinity in one check).

### 24j. `resolve.rs` does not range-check `defaults.font.size` or `defaults.mono_font.size`

**Category:** api-bug
**Severity:** medium
**File(s):** `src/resolve.rs:621-631`

**Problem:** The `validate()` function requires `defaults.font` and
`defaults.mono_font` via `require_font()`, which extracts `family`, `size`,
and `weight`. But the extracted `defaults_font.size` and
`defaults_mono_font.size` are never passed to `check_positive()`.

Compare with widget fonts (button.font, input.font, menu.font, etc.) which
DO get `check_positive(xxx_font.size, ...)` calls. The defaults font sizes
are the root from which all widget fonts inherit, yet they are the only
font sizes not validated.

A `defaults.font.size = 0.0` or `defaults.font.size = -5.0` would pass
validation, then every widget would inherit the invalid size.

**Solution Options:**

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add `check_positive(defaults_font.size, ...)` and same for mono_font | Consistent with widget font checks | 4 lines |
| B | Leave to widget-level checks | Widgets inherit and their checks catch it | Only catches at widget level, not at source |

**Recommended:** A. Add after the existing `require_font()` calls:
```
check_positive(defaults_font.size, "defaults.font.size", &mut missing);
check_range_u16(defaults_font.weight, 100, 900, "defaults.font.weight", &mut missing);
check_positive(defaults_mono_font.size, "defaults.mono_font.size", &mut missing);
check_range_u16(defaults_mono_font.weight, 100, 900, "defaults.mono_font.weight", &mut missing);
```

### 24k. `preset_loading.rs` `all_presets_have_valid_fonts` does not check mono_font at all

**Category:** test-gap
**Severity:** low
**File(s):** `tests/preset_loading.rs:138-163`

**Problem:** The test checks `defaults.font.family.is_some()` and
`defaults.font.size > 0` for both light and dark variants of all 16
presets, but never checks `defaults.mono_font`. Every preset defines
`mono_font.family` and `mono_font.size`, but a preset missing
`mono_font.family` would pass this test.

Additionally, as noted in existing issue 9a, the test does not check
`defaults.font.weight.is_some()`.

**Recommended:** Extend the test to also assert `mono_font.family.is_some()`
and `mono_font.size.unwrap() > 0`, plus `font.weight.is_some()` per issue 9a.

### 24l. `preset_loading.rs` `all_presets_have_spacing` only checks `spacing.m`

**Category:** test-gap
**Severity:** low
**File(s):** `tests/preset_loading.rs:192-209`

**Problem:** The test asserts only `spacing.m.is_some()`. All 16 presets
define all 7 spacing values (xxs through xxl), but a preset missing any
of the other 6 values would pass this test. Given that `validate()` requires
all 7 spacing fields, a missing value would only be caught at
resolve+validate time, not in this early-catch test.

**Recommended:** Assert all 7 spacing fields: xxs, xs, s, m, l, xl, xxl.

### 24m. `preset_loading.rs` `all_presets_have_geometry` does not check `frame_width`, `disabled_opacity`, `border_opacity`, `line_height`, `focus_ring_width`

**Category:** test-gap
**Severity:** low
**File(s):** `tests/preset_loading.rs:165-190`

**Problem:** The geometry test only checks `radius`, `radius_lg`, and
`shadow_enabled`. But `validate()` requires many more defaults-level
geometry fields: `frame_width`, `disabled_opacity`, `border_opacity`,
`focus_ring_width`, `focus_ring_offset`, `line_height`. A preset missing
any of these would pass the loading test but fail at validation time.

**Recommended:** Extend to check at least `frame_width.is_some()` and
`line_height.is_some()` which are the most critical for rendering.

### 24n. `serde_roundtrip.rs` `fully_populated_variant()` never sets `accent_foreground`, `selection_inactive`, or `shadow_enabled`

**Category:** test-gap
**Severity:** low
**File(s):** `tests/serde_roundtrip.rs:15-101`

**Problem:** This helper function is used for round-trip tests but
omits several defaults fields that have special behavior:

- `accent_foreground`: resolve derives it if missing, but the round-trip
  helper never sets it, so the round-trip test never verifies it survives
  serialization.
- `selection_inactive`: derived from `selection` by resolve, but never
  set explicitly in the helper, so never tested in round-trip.
- `shadow_enabled`: a `bool` field (not color), never set in the helper.
- `radius_lg`: never set in the helper.
- `line_height`: never set in the helper.

This is partially covered by existing issue 9e, but specifying the
exact missing fields adds actionable detail.

**Recommended:** Extend the helper or (per issue 9e) use an actual
preset for round-trip testing, which has all fields populated.

### 24o. iOS preset `font.size = 17.0` vs macOS `font.size = 13.0` -- correct per Apple HIG

**Category:** preset-value (verification)
**Severity:** N/A (no issue found)
**File(s):** `src/presets/ios.toml:52,249` vs `src/presets/macos-sonoma.toml:52,252`

**Verification:** Apple Human Interface Guidelines specify 17pt as the
default body text size for iOS (UIKit `.body` dynamic type). macOS uses
13pt (NSFont systemFontSize). The 4pt difference is correct -- iOS is a
touch-first platform requiring larger tap targets and text.

Other iOS-specific correct values verified:
- `button.min_height = 44.0` (Apple's 44pt minimum tap target)
- `menu.item_height = 44.0` (same tap target minimum)
- `scrollbar.overlay_mode = true` (iOS always uses overlay scrollbars)
- `frame_width = 0.5` (iOS uses hairline borders -- 0.5pt = 1px on 2x)
- `disabled_opacity = 0.3` (matches UIControl disabled alpha)
- `switch.track_width = 51.0`, `track_height = 31.0` (UISwitch dimensions)

**Verdict:** iOS preset values are well-researched and consistent with
Apple's UIKit/SwiftUI specifications.

### 24p. iOS preset `dialog.button_order = "leading_affirmative"` -- already tracked in issue 1u

**Category:** preset-value
**Severity:** high (already tracked)

**Verification:** Issue 1u already identifies this. Confirming that both
`ios.toml:164` (light) and `ios.toml:362` (dark) have the wrong value.
No additional finding.

### 24q. Material preset uses `disabled_opacity = 0.38` and `border_opacity = 0.12` -- correct per MD3

**Category:** preset-value (verification)
**Severity:** N/A (no issue found)
**File(s):** `src/presets/material.toml:39-40,238-239`

**Verification:** Material Design 3 specifies:
- Disabled state opacity: 0.38 (38%)
- Outline opacity: 0.12 (12%)

The preset matches these values exactly. Additionally verified:
- `radius = 12.0`, `radius_lg = 16.0` (MD3 corner radius scale)
- `button.min_height = 40.0` (MD3 button height)
- `input.min_height = 56.0` (MD3 text field height)
- `toolbar.height = 64.0` (MD3 top app bar height)
- `menu.item_height = 48.0` (MD3 menu item height)
- `switch.track_width = 52.0`, `track_height = 32.0` (MD3 switch)
- `dialog.min_width = 280.0` (MD3 dialog minimum)

**Verdict:** Material preset is well-aligned with Material Design 3
specifications.

### 24r. Material preset uses `link.underline = false` -- correct per MD3

**Category:** preset-value (verification)
**Severity:** N/A (no issue found)

**Verification:** Material Design 3 does not underline links by default;
it relies on color differentiation. The `link.underline = false` value is
correct. All other presets use `underline = true` which is correct for
their respective platforms/design systems.

### 24s. Material preset `spacing.s = 8.0` and `spacing.l = 16.0` differ from all other presets

**Category:** preset-consistency (informational)
**Severity:** N/A (not a bug)
**File(s):** `src/presets/material.toml:63-64,262-263`

**Observation:** Material uses `s = 8.0` where all other presets use
`s = 6.0`, and `l = 16.0` where most community presets use `l = 18.0`.
This reflects Material Design 3's 4dp baseline grid (`4, 8, 12, 16, 24`
progression) vs the `2, 4, 6, 12, 18, 24, 36` scale used by others.

**Verdict:** Correct for Material Design. Not a consistency bug.

### 24t. `resolve.rs` `resolve_color_inheritance()` does not derive `spinner.fill` from `accent` -- uses `foreground` instead

**Category:** api-design
**Severity:** low
**File(s):** `src/resolve.rs:282-284`

**Problem:** In `resolve_safety_nets()`:
```rust
if self.spinner.fill.is_none() {
    self.spinner.fill = self.defaults.foreground;
}
```

platform-facts.md SS2.16 line 1140-1143:
- macOS: `controlAccentColor` (accent)
- Windows: `ProgressRing foreground = accent color`
- GNOME: `accent_bg_color` (accent)
- KDE: `[Colors:View] DecorationFocus` (accent)

All four platforms use accent for spinner fill, not foreground. The
current safety net uses `foreground`, which is only correct as a
last-resort fallback. Platform readers set the correct accent-based
value, but community presets that omit `spinner.fill` will get
`foreground` (a dark/light gray) instead of the accent color.

This is similar in spirit to the `input.caret` discussion in 21o,
but the evidence is stronger here: ALL four platforms use accent for
spinners.

**Solution Options:**

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Change safety net to `accent` | Matches all 4 platforms; community presets get colored spinners | Low contrast possible if accent is very light |
| B | Keep `foreground` | Always visible | Wrong for all platforms |

**Recommended:** A. Change `spinner.fill` safety net from
`defaults.foreground` to `defaults.accent`.

### 24u. `resolve.rs` `resolve_color_inheritance()` does not derive `scrollbar.thumb_hover` distinctly from `thumb`

**Category:** api-design
**Severity:** low
**File(s):** `src/resolve.rs:417-420`

**Problem:** Both `scrollbar.thumb` and `scrollbar.thumb_hover` resolve
to `defaults.muted`:
```rust
if self.scrollbar.thumb.is_none() {
    self.scrollbar.thumb = d.muted;
}
if self.scrollbar.thumb_hover.is_none() {
    self.scrollbar.thumb_hover = d.muted;
}
```

This means the hover state is identical to the normal state -- no visual
feedback when hovering over the scrollbar thumb. All platforms provide
distinct hover states (typically darker/lighter).

The resolve chain should derive `thumb_hover` from `thumb` rather than
independently from `muted`, so that presets setting only `thumb` still
get a distinct hover. However, since all bundled presets rely on resolve
for these colors and none of them explicitly set `scrollbar.thumb_hover`,
ALL presets currently have identical thumb and thumb_hover colors.

**Recommended:** Low priority. Document the limitation. A proper fix
would derive `thumb_hover` from `thumb` with a slight darkening/lightening
transform, which requires color manipulation not currently in the
resolve engine.

### 24v. `merge_behavior.rs` `is_empty_all_structs` test covers only 4 structs -- already noted in 9c but exact gap is larger

**Category:** test-gap
**Severity:** low
**File(s):** `tests/merge_behavior.rs:212-218`

**Problem:** Issue 9c already notes this test covers only 4 of 30+ structs.
Exact gap: the test checks `ThemeDefaults`, `FontSpec`, `ThemeSpacing`,
`ThemeVariant` but misses `ThemeSpec`, `IconSizes`, `TextScale`,
`TextScaleEntry`, and all 25 widget structs generated by
`define_widget_pair!` (ButtonTheme, InputTheme, CheckboxTheme, etc.).

**Verdict:** Already tracked in 9c. Adding exact struct names for the fix.

### 24w. `serde_roundtrip.rs` `round_trip_full_theme` does not verify `ThemeSpec` structural equality -- only spot-checks fields

**Category:** test-gap
**Severity:** low
**File(s):** `tests/serde_roundtrip.rs:107-173`

**Problem:** The test manually compares individual fields (accent,
background, foreground, etc.) rather than using `assert_eq!(theme, deserialized)`.
If `ThemeSpec` implements `PartialEq` (which it does, since all its
constituent types derive it), a single `assert_eq!` would catch any field
that fails round-trip without needing to enumerate every field manually.

The current approach means that any new field added to a struct but not
added to the test will silently pass even if its round-trip is broken.

**Recommended:** Replace spot-check assertions with
`assert_eq!(theme, deserialized)` for complete structural equality
verification. Keep one or two spot-checks for human-readable failure
messages if the full equality assert fails.

### 24x. All community presets have identical widget geometry values (copy-paste template)

**Category:** preset-design
**Severity:** low (informational)
**File(s):** All 11 community presets

**Observation:** Every community preset uses identical widget geometry:
- `button: min_width=64, min_height=32, padding_horizontal=12, padding_vertical=6`
- `input: min_height=32, padding_horizontal=8, padding_vertical=6`
- `checkbox: indicator_size=18, spacing=6`
- `tooltip: padding_horizontal=6, padding_vertical=6, max_width=300`
- `scrollbar: width=14, slider_width=8, min_thumb_height=30`
- `slider: track_height=4, thumb_size=20, tick_length=4`
- `dialog: min_width=320, max_width=560, min_height=140, max_height=600`
- (and all other widget sections)

The only exceptions are Material (which has its own MD3-specific geometry)
and the non-geometry fields (colors).

This is not a bug -- the community presets are primarily color themes
that share a common geometry baseline. But it means the geometry values
are effectively hardcoded constants copied 10 times. If the baseline
geometry should change (e.g., following a future issue fix), all 10 files
need updating.

**Recommended:** Consider extracting a shared geometry template that
community presets inherit from, reducing duplication. This is a v0.6.0+
design consideration, not a v0.5.4 fix.

---

## 25. Systematic macOS Sonoma Preset Verification (parameter-by-parameter)

Exhaustive cross-reference of every property in `platform-facts.md` Chapter 2
against `src/presets/macos-sonoma.toml` (light + dark variants) and the
resolve() safety nets in `src/resolve.rs`.

Legend:
- File references are to `native-theme/src/presets/macos-sonoma.toml` unless noted.
- "Inherited" means the preset omits the property and resolve() fills it from defaults.
- Light variant line numbers cited; dark variant follows the same pattern (+198 lines offset).

### 2.1 Global Defaults

#### 2.1.1 Base Font

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `family` | `+systemFontOfSize:` -> family | `"SF Pro"` (line 51) | :white_check_mark: Correct -- SF Pro is the system font |
| `size` | `+systemFontOfSize:` -> pointSize | `13.0` (line 52) | :white_check_mark: Correct -- systemFontSize = 13 |
| `weight` | `NSFontDescriptor` traits | `400` (line 53) | :white_check_mark: Correct -- system font default is Regular (400) |
| `line_height` | 1.19 (font metrics: (1950+494+0)/2048) | `1.19` (line 42) | :white_check_mark: Correct |

#### 2.1.2 Monospace Font

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `family` | `+monospacedSystemFont...` -> family | `"SF Mono"` (line 56) | :white_check_mark: Correct |
| `size` | `+monospacedSystemFont...` -> ptSize | `13.0` (line 57) | :white_check_mark: Correct |
| `weight` | `NSFontDescriptor` traits | `400` (line 58) | :white_check_mark: Correct |

#### 2.1.3 Base Colors

All colors are runtime-dependent (`systemFontOfSize:` API values). The preset
uses hardcoded approximations for the static preset. Verified against known
NSColor catalog values:

| Property | platform-facts macOS source | Preset L / D | Verdict |
|----------|----------------------------|--------------|---------|
| `background` | `windowBackgroundColor` | `#f0f0f0` / `#1e1e1e` (L:14, D:211) | :white_check_mark: Correct -- measured Sonoma values |
| `foreground` | `labelColor` | `#1d1d1f` / `#ffffff` (L:15, D:212) | :white_check_mark: Correct |
| `accent` | `controlAccentColor` | `#007aff` / `#0a84ff` (L:13, D:210) | :white_check_mark: Correct -- default blue accent |
| `accent_foreground` | `alternateSelectedControlTextColor` | `#ffffff` / `#ffffff` (L:34, D:232) | :white_check_mark: Correct |
| `surface` | `controlBackgroundColor` | `#ffffff` / `#2d2d2d` (L:16, D:213) | :white_check_mark: Correct |
| `border` | `separatorColor` | `#d5d5d5` / `#3d3d3d` (L:17, D:214) | :white_check_mark: Correct |
| `muted` | `secondaryLabelColor` | `#86868b` / `#98989d` (L:18, D:215) | :white_check_mark: Correct |
| `shadow` | `shadowColor` | `#00000018` / `#00000040` (L:19, D:216) | :white_check_mark: Correct |
| `link` | `linkColor` | `#007aff` / `#0a84ff` (L:30, D:228) | :white_check_mark: Correct |
| `selection` | `selectedContentBackgroundColor` | `#007aff` / `#0a84ff` (L:29, D:227) | :white_check_mark: Correct |
| `selection_foreground` | `selectedTextColor` | `#ffffff` / `#ffffff` (L:30, D:228) | :white_check_mark: Correct |
| `selection_inactive` | `unemphasizedSelectedContentBackgroundColor` | Not in preset | :arrows_counterclockwise: Inherited -- resolve() derives from `selection` (resolve.rs:265-267). Acceptable: platform-facts says macOS has this API but the preset lets resolve() handle it |
| `disabled_foreground` | `disabledControlTextColor` | `#c7c7cc` / `#48484a` (L:33, D:231) | :white_check_mark: Correct |

#### 2.1.4 Status Colors

| Property | platform-facts macOS source | Preset L / D | Verdict |
|----------|----------------------------|--------------|---------|
| `danger` | `systemRedColor` | `#ff3b30` / `#ff453a` (L:21, D:218) | :white_check_mark: Correct |
| `danger_foreground` | `labelColor` (body foreground) | `#ffffff` / `#ffffff` (L:22, D:219) | :x: **Wrong** -- platform-facts says macOS uses `labelColor` (= normal foreground), not white. Light should be `#1d1d1f`, dark should be `#ffffff`. Light is wrong. |
| `warning` | `systemOrangeColor` | `#ff9500` / `#ff9f0a` (L:23, D:220) | :white_check_mark: Correct |
| `warning_foreground` | `labelColor` | `#1d1d1f` / `#1e1e1e` (L:24, D:221) | :white_check_mark: Correct -- matches labelColor/foreground for each variant |
| `success` | `systemGreenColor` | `#34c759` / `#30d158` (L:25, D:222) | :white_check_mark: Correct |
| `success_foreground` | `labelColor` | `#ffffff` / `#ffffff` (L:26, D:223) | :x: **Wrong** -- same issue as danger_foreground. Light should be `#1d1d1f`. |
| `info` | `systemBlueColor` | `#007aff` / `#0a84ff` (L:27, D:224) | :white_check_mark: Correct |
| `info_foreground` | `labelColor` | `#ffffff` / `#ffffff` (L:28, D:225) | :x: **Wrong** -- same issue. Light should be `#1d1d1f`. |

**Note on status foregrounds:** platform-facts SS2.1.4 explicitly warns about
semantic mismatch. macOS provides `labelColor` as the foreground -- suitable
as text color *alongside* a status indicator, NOT as text on a status-colored
background. The preset uses white (#ffffff) which is a contrast foreground
for text *on* the status color. This is a deliberate design choice vs a
literal API transcription. See platform-facts lines 894-903 for the full
discussion. Whether this is "wrong" depends on which semantic the consumer
expects. Flagged for review.

#### 2.1.5 Focus Ring

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `color` | `keyboardFocusIndicatorColor` | `#007aff` / `#0a84ff` (L:32, D:230) | :white_check_mark: Correct -- matches accent |
| `width` | 3px (measured) | `3.0` (L:43, D:241) | :white_check_mark: Correct |
| `offset` | -1px (measured, inset) | `-1.0` (L:44, D:242) | :white_check_mark: Correct |

#### 2.1.6 Global Geometry

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `radius` | 5px (measured) | `5.0` (L:36, D:234) | :white_check_mark: Correct |
| `radius_lg` | 10px (measured) | `10.0` (L:37, D:235) | :white_check_mark: Correct |
| `frame_width` | 0.5px (measured) | `1.0` (L:38, D:236) | :x: **Wrong** -- platform-facts says 0.5px, preset has 1.0 |
| `disabled_opacity` | ~0.25-0.3 (measured) | `0.3` (L:39, D:237) | :white_check_mark: Correct -- within the stated range |
| `border_opacity` | 0.2 (preset) | `0.2` (L:40, D:238) | :white_check_mark: Correct |
| `shadow_enabled` | yes | `true` (L:41, D:239) | :white_check_mark: Correct |

#### 2.1.7 Accessibility

| Property | platform-facts macOS source | Preset value (file:line) | Verdict |
|----------|----------------------------|--------------------------|---------|
| `text_scaling_factor` | API-provided (very limited on macOS) | `1.0` (L:45, D:243) | :white_check_mark: Correct -- 1.0 is the default |
| `reduce_motion` | `accessibilityDisplayShouldReduceMotion` | `false` (L:46, D:244) | :white_check_mark: Correct -- false is default |
| `high_contrast` | `accessibilityDisplayShouldIncreaseContrast` | `false` (L:47, D:245) | :white_check_mark: Correct |
| `reduce_transparency` | `accessibilityDisplayShouldReduceTransparency` | `false` (L:48, D:246) | :white_check_mark: Correct |

#### 2.1.8 Icon Sizes

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `toolbar` | 32pt (reg) / 24 (sm) | `24.0` (L:70, D:268) | :white_check_mark: Correct -- uses the small variant (modern default) |
| `small` | sidebar: 16-20pt | `16.0` (L:71, D:269) | :white_check_mark: Correct -- lower bound of range |
| `large` | (none) | `32.0` (L:72, D:270) | :heavy_minus_sign: N/A -- macOS has no large icon context; 32 is a reasonable placeholder |
| `dialog` | (none) | `22.0` (L:73, D:271) | :heavy_minus_sign: N/A -- macOS has no specific dialog icon size; reasonable |
| `panel` | (none) | `20.0` (L:74, D:272) | :heavy_minus_sign: N/A -- macOS has no panel icon size; reasonable |

### 2.2 Window / Application Chrome

| Property | platform-facts macOS value | Preset value | Verdict |
|----------|---------------------------|--------------|---------|
| `background` | `<- defaults.background` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:307-309 fills from `defaults.background` |
| `foreground` | `<- defaults.foreground` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:310-312 |
| `border` | `<- defaults.border` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:313-315 |
| `title_bar_background` | (measured) ~ `defaults.surface` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:316-318 fills from `defaults.surface` |
| `title_bar_foreground` | `windowFrameTextColor` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:319-321 fills from `defaults.foreground` |
| `inactive_title_bar_bg` | (none) -- system-managed dimming | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:595-597 fills from active title_bar_background |
| `inactive_title_bar_fg` | (none) -- system-managed | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:598-600 fills from active title_bar_foreground |
| `title_bar_font.family` | `+titleBarFontOfSize:` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:564 fills from defaults.font |
| `title_bar_font.size` | `+titleBarFontOfSize:` -> pointSize | Not in preset | :arrows_counterclockwise: Inherited |
| `title_bar_font.weight` | Bold (700) | Not in preset | :warning: **Missing** -- platform-facts says macOS title bar font weight is Bold (700), but the preset omits it. resolve() fills from defaults.font weight = 400. The resolved title_bar_font.weight will be 400 instead of 700. |
| `radius` | macOS window corners: 10px | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:322-324 fills from `defaults.radius_lg` = 10.0. Correct. |
| `shadow` | `<- defaults.shadow_enabled` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:325-327 |

### 2.3 Button

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `background` | `controlColor` | `#e8e8ed` / `#3a3a3c` (L:77, D:275) | :white_check_mark: Correct |
| `foreground` | `controlTextColor` | `#1d1d1f` / `#ffffff` (L:78, D:276) | :white_check_mark: Correct |
| `border` | `<- defaults.border` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:336-338 |
| `font` | `<- defaults.font` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:565 |
| `min_width` | (none) -- sizes to content | `64.0` (L:79, D:277) | :white_check_mark: Correct -- platform-facts says (none) but a reasonable minimum is fine for a static preset |
| `min_height` | NSButton intrinsic: 22 | `22.0` (L:80, D:278) | :white_check_mark: Correct |
| `padding_horizontal` | NSButton: ~8 (WebKit) | `8.0` (L:81, D:279) | :white_check_mark: Correct |
| `padding_vertical` | 3 (measured: (22-16)/2) | `3.0` (L:82, D:280) | :white_check_mark: Correct |
| `radius` | `<- defaults.radius` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:345-347 fills from defaults.radius = 5.0 |
| `icon_spacing` | 4 (measured) AppKit | `4.0` (L:83, D:281) | :white_check_mark: Correct |
| `primary_bg` | `<- defaults.accent` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:339-341 |
| `primary_fg` | `<- defaults.accent_foreground` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:342-344 |
| `disabled_opacity` | `<- defaults.disabled_opacity` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:348-350 |
| `shadow` | `<- defaults.shadow_enabled` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:351-353 |

### 2.4 Text Input

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `background` | `textBackgroundColor` | `#ffffff` / `#2d2d2d` (L:86, D:284) | :white_check_mark: Correct |
| `foreground` | `textColor` | `#1d1d1f` / `#ffffff` (L:87, D:285) | :white_check_mark: Correct |
| `border` | `<- defaults.border` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:363-365 |
| `placeholder` | `placeholderTextColor` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:366-368 fills from `defaults.muted`. Acceptable. |
| `caret` | `textInsertionPointColor` (macOS 14+) | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:274-276 (safety net) fills from `defaults.foreground`. Correct for pre-14 behavior. |
| `selection` | `<- defaults.selection` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:369-371 |
| `selection_foreground` | `<- defaults.selection_foreground` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:372-374 |
| `font` | `<- defaults.font` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:566 |
| `min_height` | NSTextField intrinsic: 22 | `22.0` (L:88, D:286) | :white_check_mark: Correct |
| `padding_horizontal` | NSTextField: 4 | `4.0` (L:89, D:287) | :white_check_mark: Correct |
| `padding_vertical` | 3 (measured: (22-16)/2) | `3.0` (L:90, D:288) | :white_check_mark: Correct |
| `radius` | `<- defaults.radius` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:375-377 |
| `border_width` | `<- defaults.frame_width` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:378-380. Note: inherits 1.0 but platform-facts says frame_width should be 0.5. See frame_width issue above. |

### 2.5 Checkbox / Radio Button

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `indicator_size` | NSButton switch: 14 | `14.0` (L:93, D:291) | :white_check_mark: Correct |
| `spacing` | AppKit: 4 | `4.0` (L:94, D:292) | :white_check_mark: Correct |
| `radius` | `<- defaults.radius` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:386-388 |
| `border_width` | `<- defaults.frame_width` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:389-391. Same frame_width issue propagates. |
| `checked_bg` | `<- defaults.accent` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:382-384 |

### 2.6 Menu

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `background` | (measured) ~ `defaults.background` (vibrancy) | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:393-395. Correct. |
| `foreground` | (measured) = `labelColor` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:396-398. Correct. |
| `separator` | (measured) = `separatorColor` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:399-401 fills from `defaults.border`. Correct (border = separatorColor). |
| `font.family` | `+menuFontOfSize:` -> family | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:567. Fills from defaults.font. Correct (same family). |
| `font.size` | `+menuFontOfSize:` -> pointSize | Not in preset | :arrows_counterclockwise: Inherited. Same as system font size (13). Correct. |
| `font.weight` | `+menuFontOfSize:` -> weight | Not in preset | :arrows_counterclockwise: Inherited. Correct (Regular). |
| `item_height` | NSMenuItem: 22 | `22.0` (L:125, D:323) | :white_check_mark: Correct |
| `padding_horizontal` | NSMenuItem: 12 | `12.0` (L:126, D:324) | :white_check_mark: Correct |
| `padding_vertical` | 3 (measured: (22-16)/2) | `3.0` (L:127, D:325) | :white_check_mark: Correct |
| `icon_spacing` | 4 (measured) AppKit layout | `4.0` (L:128, D:326) | :white_check_mark: Correct |

### 2.7 Tooltip

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `background` | (preset) L #2c2c2e D #3a3a3c | `#2c2c2e` / `#3a3a3c` (L:97, D:295) | :white_check_mark: Correct |
| `foreground` | (preset) #ffffff (both variants) | `#ffffff` / `#ffffff` (L:98, D:296) | :white_check_mark: Correct |
| `font.family` | `+toolTipsFontOfSize:` -> family | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:568. Correct. |
| `font.size` | `+toolTipsFontOfSize:` -> ptSize | Not in preset | :arrows_counterclockwise: Inherited. Correct (same as system font). |
| `font.weight` | `+toolTipsFontOfSize:` -> weight | Not in preset | :arrows_counterclockwise: Inherited. Correct. |
| `padding_horizontal` | NSToolTipManager: 4 | `4.0` (L:99, D:297) | :white_check_mark: Correct |
| `padding_vertical` | NSToolTipManager: 4 | `4.0` (L:100, D:298) | :white_check_mark: Correct |
| `max_width` | 300 (measured) macOS Sonoma | `300.0` (L:101, D:299) | :white_check_mark: Correct |
| `radius` | `<- defaults.radius` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:410-412. Correct. |

### 2.8 Scrollbar

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `track` | transparent (overlay mode) | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:278-280 (safety net) fills from `defaults.background`. Acceptable for legacy mode. |
| `thumb` | `#80808080` (measured) Sonoma | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:415-417 fills from `defaults.muted`. Muted is `#86868b` (L) / `#98989d` (D), which differs from the measured `#80808080` (semi-transparent gray). The muted color is opaque, not semi-transparent. |
| `thumb_hover` | `#60606080` (measured) Sonoma | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:418-420 fills from `defaults.muted`. Same opacity mismatch. |
| `width` | legacy: 16 / overlay: ~6-7 | `16.0` (L:104, D:302) | :white_check_mark: Correct -- legacy width |
| `min_thumb_height` | 40 (measured) legacy mode | `40.0` (L:107, D:305) | :white_check_mark: Correct |
| `slider_width` | overlay: ~6-7 | `7.0` (L:105, D:303) | :white_check_mark: Correct |
| `overlay_mode` | `NSScroller.preferredScrollerStyle` | `true` (L:106, D:304) | :white_check_mark: Correct -- Sonoma defaults to overlay |

### 2.9 Slider

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `fill` | `<- defaults.accent` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:423-425 |
| `track` | `<- defaults.muted` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:426-428 |
| `thumb` | `<- defaults.surface` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:429-431 |
| `track_height` | NSSlider: 5 | `5.0` (L:110, D:308) | :white_check_mark: Correct |
| `thumb_size` | NSSlider knob: 21 | `21.0` (L:111, D:309) | :white_check_mark: Correct |
| `tick_length` | NSSlider: 8 | `8.0` (L:112, D:310) | :white_check_mark: Correct |

### 2.10 Progress Bar

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `fill` | `<- defaults.accent` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:434-436 |
| `track` | `<- defaults.muted` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:437-439 |
| `height` | NSProgressIndicator: 6 | `6.0` (L:115, D:313) | :white_check_mark: Correct |
| `min_width` | (none) -- no minimum | `100.0` (L:116, D:314) | :white_check_mark: Correct -- reasonable default despite (none) in platform-facts |
| `radius` | `<- defaults.radius` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:440-442 |

### 2.11 Tab Bar

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `background` | `<- defaults.background` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:445-447 |
| `foreground` | `<- defaults.foreground` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:448-450 |
| `active_background` | `<- defaults.background` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:451-453 |
| `active_foreground` | `<- defaults.foreground` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:454-456 |
| `bar_background` | `<- defaults.background` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:457-459 |
| `min_width` | (none) -- sizes to label | `64.0` (L:119, D:317) | :white_check_mark: Correct -- reasonable minimum |
| `min_height` | NSTabView: 24 | `24.0` (L:120, D:318) | :white_check_mark: Correct |
| `padding_horizontal` | NSTabView: 12 | `12.0` (L:121, D:319) | :white_check_mark: Correct |
| `padding_vertical` | 4 (measured: (24-16)/2) | `4.0` (L:122, D:320) | :white_check_mark: Correct |

### 2.12 Sidebar

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `background` | `underPageBackgroundColor` | `#e8e8ed` / `#262626` (L:142, D:340) | :white_check_mark: Correct -- explicit sidebar-specific color |
| `foreground` | `<- defaults.foreground` | `#1d1d1f` / `#ffffff` (L:143, D:341) | :white_check_mark: Correct -- matches defaults.foreground |

### 2.13 Toolbar

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `font.family` | `<- defaults.font` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:569 |
| `font.size` | `<- defaults.font` | Not in preset | :arrows_counterclockwise: Inherited |
| `font.weight` | `<- defaults.font` | Not in preset | :arrows_counterclockwise: Inherited |
| `height` | NSToolbar: 38 | `38.0` (L:137, D:335) | :white_check_mark: Correct |
| `item_spacing` | AppKit: 8 | `8.0` (L:138, D:336) | :white_check_mark: Correct |
| `padding` | 8 (measured) NSToolbar | `8.0` (L:139, D:337) | :white_check_mark: Correct |

### 2.14 Status Bar

| Property | platform-facts macOS value | Preset value | Verdict |
|----------|---------------------------|--------------|---------|
| `font.family` | `<- defaults.font` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:570 |
| `font.size` | `<- defaults.font` | Not in preset | :arrows_counterclockwise: Inherited |
| `font.weight` | `<- defaults.font` | Not in preset | :arrows_counterclockwise: Inherited |

### 2.15 List / Table

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `background` | `<- defaults.background` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:289-291 (safety net) fills from `defaults.background` |
| `foreground` | `<- defaults.foreground` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:470-472 |
| `alternate_row` | `alternatingContentBackgroundColors[1]` | `#f5f5fa` / `#252527` (L:131, D:329) | :white_check_mark: Correct -- measured values |
| `selection` | `<- defaults.selection` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:476-478 |
| `selection_foreground` | `<- defaults.selection_foreground` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:479-481 |
| `header_background` | (measured) ~ `defaults.surface` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:482-484 fills from `defaults.surface`. Correct. |
| `header_foreground` | `headerTextColor` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:485-487 fills from `defaults.foreground`. Acceptable. |
| `grid_color` | `gridColor` (SS1.1.2) | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:488-490 fills from `defaults.border`. Acceptable. |
| `item_height` | NSTableView row: 24 | `24.0` (L:132, D:330) | :white_check_mark: Correct |
| `padding_horizontal` | NSTableView: 4 | `4.0` (L:133, D:331) | :white_check_mark: Correct |
| `padding_vertical` | 4 (measured: (24-16)/2) | `4.0` (L:134, D:332) | :white_check_mark: Correct |

### 2.16 Popover / Dropdown

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `background` | `<- defaults.background` | `#ffffff` / `#2d2d2d` (L:146, D:344) | :white_check_mark: Correct -- explicit value matches `defaults.surface`, not defaults.background. Platform-facts says it inherits from defaults.background, but the preset overrides with a specific value matching control/surface bg. |
| `foreground` | `<- defaults.foreground` | `#1d1d1f` / `#ffffff` (L:147, D:345) | :white_check_mark: Correct |
| `border` | `<- defaults.border` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:496-498 |
| `radius` | `<- defaults.radius_lg` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:499-501 fills from `defaults.radius_lg` = 10.0. Correct. |

### 2.17 Splitter

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `width` | NSSplitView divider: 6 | `6.0` (L:153, D:351) | :white_check_mark: Correct |

### 2.18 Separator

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `color` | `separatorColor` | `#d5d5d5` / `#3d3d3d` (L:150, D:348) | :white_check_mark: Correct -- matches `defaults.border` |

### 2.19 Text Scale

The preset does NOT include text_scale entries. resolve_text_scale() in
resolve.rs:574-589 creates them from defaults.font, inheriting size and
weight from the base font, and computing line_height as
`defaults.line_height * size`.

| Role | platform-facts macOS value | Preset value | Verdict |
|------|---------------------------|--------------|---------|
| `caption.size` | `.caption1`: 10pt | Not in preset | :warning: **Missing** -- resolves to 13.0 (defaults.font.size). Should be 10.0. |
| `caption.weight` | 400 | Not in preset | :arrows_counterclockwise: Inherited -- resolves to 400. Correct. |
| `section_heading.size` | `.headline`: 13pt | Not in preset | :arrows_counterclockwise: Inherited -- resolves to 13.0. Correct. |
| `section_heading.weight` | **700** (Bold) | Not in preset | :warning: **Missing** -- resolves to 400. Should be 700. |
| `dialog_title.size` | `.title1`: 22pt | Not in preset | :warning: **Missing** -- resolves to 13.0. Should be 22.0. |
| `dialog_title.weight` | 400 | Not in preset | :arrows_counterclockwise: Inherited -- resolves to 400. Correct. |
| `display.size` | `.largeTitle`: 26pt | Not in preset | :warning: **Missing** -- resolves to 13.0. Should be 26.0. |
| `display.weight` | 400 | Not in preset | :arrows_counterclockwise: Inherited -- resolves to 400. Correct. |

### 2.20 Layout Spacing

The model does not have explicit `layout_spacing` properties. The spacing
scale (`defaults.spacing`) provides the building blocks. Platform-facts
SS2.20 provides per-context guidance, not per-property model mapping.

| Context | platform-facts macOS value | Mapping to spacing scale | Verdict |
|---------|---------------------------|-------------------------|---------|
| Standard widget gap | 8 (HIG) | `spacing.m = 10.0` or `spacing.xs = 4.0` | :warning: **No exact match** -- macOS HIG says 8px standard gap. The spacing scale has xs=4, s=6, m=10. There is no 8.0 tier. 8 falls between s and m. |
| Nested container margin | (none) -- not specified | N/A | :heavy_minus_sign: N/A |
| Window content margin | 20 (HIG) | `spacing.xl = 20.0` | :white_check_mark: Correct |
| Section gap | 20 (HIG) | `spacing.xl = 20.0` | :white_check_mark: Correct |

### 2.21 Switch / Toggle

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `track_width` | 38px | `38.0` (L:158, D:356) | :white_check_mark: Correct |
| `track_height` | 22px | `22.0` (L:159, D:357) | :white_check_mark: Correct |
| `thumb_size` | ~18px (measured) | `18.0` (L:160, D:358) | :white_check_mark: Correct |
| `track_radius` | half height (pill) = 11 | `10.0` (L:161, D:359) | :x: **Wrong** -- half of 22 = 11, not 10. Should be `11.0` for a perfect pill shape. |
| `checked_bg` | `<- defaults.accent` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:509-511 |
| `unchecked_bg` | (measured) track bg | `#d5d5d5` / `#3d3d3d` (L:156, D:354) | :white_check_mark: Correct -- measured track background |
| `thumb_bg` | (measured) white | `#ffffff` / `#ffffff` (L:157, D:355) | :white_check_mark: Correct |

### 2.22 Dialog

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `min_width` | (none) -- AppKit-managed | `320.0` (L:165, D:363) | :heavy_minus_sign: N/A -- platform-facts says (none) for macOS; preset provides a reasonable default |
| `max_width` | (none) -- AppKit-managed | `560.0` (L:166, D:364) | :heavy_minus_sign: N/A |
| `min_height` | (none) -- AppKit-managed | `140.0` (L:167, D:365) | :heavy_minus_sign: N/A |
| `max_height` | (none) -- AppKit-managed | `600.0` (L:168, D:366) | :heavy_minus_sign: N/A |
| `content_padding` | ~20px (measured) | `24.0` (L:169, D:367) | :x: **Wrong** -- platform-facts says ~20px, preset has 24. |
| `button_spacing` | ~12px (measured) | `8.0` (L:170, D:368) | :x: **Wrong** -- platform-facts says ~12px, preset has 8. |
| `button_order` | primary rightmost | `"leading_affirmative"` (L:164, D:362) | :x: **Wrong** -- macOS places primary/affirmative on the RIGHT (trailing). The enum `LeadingAffirmative` is documented as "left end" in dialog_order.rs:19. If the enum name is taken literally, macOS should use `trailing_affirmative`. See analysis below. |
| `title_font` | system alert heading | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:571. Fills from defaults.font. No specific alert heading font. |
| `radius` | `<- defaults.radius_lg` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:520-522 fills from `defaults.radius_lg` = 10.0. Correct. |
| `icon_size` | 64px (app icon) | `32.0` (L:171, D:369) | :x: **Wrong** -- platform-facts says macOS dialog icon is 64px, preset has 32. |

**Dialog button_order analysis:** platform-facts SS2.22 line 1195 states
macOS convention is "primary rightmost". The `DialogButtonOrder` enum in
`dialog_order.rs` defines `TrailingAffirmative` as "affirmative button at
the trailing (right) end" and labels it "Windows, GNOME style". It defines
`LeadingAffirmative` as "affirmative button at the leading (left) end" and
labels it "macOS, KDE style". The code comments are internally contradictory
with platform-facts. The actual macOS behavior is primary button rightmost
(trailing), which means either:
(a) The preset should use `trailing_affirmative`, OR
(b) The enum variant comments in dialog_order.rs have the descriptions
    swapped ("leading" should mean "right" and "trailing" should mean "left").
If (b), then the PRESET is correct but the doc strings are wrong. If (a),
it is a logic bug affecting layout.

### 2.23 Spinner / Progress Ring

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `diameter` | 32px regular | `32.0` (L:179, D:377) | :white_check_mark: Correct |
| `min_size` | 10px (mini) | `10.0` (L:180, D:378) | :white_check_mark: Correct |
| `stroke_width` | (none) -- fin-based | `2.0` (L:181, D:379) | :heavy_minus_sign: N/A -- macOS uses fins not strokes; 2.0 is a generic fallback |
| `fill` | system gray | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:282-284 (safety net) fills from `defaults.foreground`. Acceptable. |

### 2.24 ComboBox / Dropdown Trigger

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `min_height` | NSPopUpButton: 21 | `21.0` (L:184, D:382) | :white_check_mark: Correct |
| `min_width` | (none) -- sizes to content | `120.0` (L:185, D:383) | :white_check_mark: Correct -- reasonable default |
| `padding_horizontal` | ~8-10px (measured) | `9.0` (L:186, D:384) | :white_check_mark: Correct -- midpoint of range |
| `arrow_size` | ~16-18px (measured) | `17.0` (L:187, D:385) | :white_check_mark: Correct -- midpoint of range |
| `arrow_area_width` | ~16-18px (measured) | `28.0` (L:188, D:386) | :x: **Wrong** -- platform-facts says ~16-18px, preset has 28. The 28 value was previously confused with the arrow area width; WebKit confirms NSPopUpButton height=21, not arrow area. The arrow area is the same size as the arrow glyph on macOS. |
| `radius` | `<- defaults.radius` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:525-527 |

### 2.25 Segmented Control

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `segment_height` | NSSegmentedControl: 24 | `24.0` (L:191, D:389) | :white_check_mark: Correct |
| `separator_width` | 1px | `1.0` (L:192, D:390) | :white_check_mark: Correct |
| `padding_horizontal` | ~8-10px (measured) | `12.0` (L:193, D:391) | :x: **Wrong** -- platform-facts says ~8-10px, preset has 12. |
| `radius` | `<- defaults.radius` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:530-532 |

### 2.26 Card / Container

platform-facts says macOS has `(none)` for all card properties.

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `background` | (none) | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:535-537 fills from `defaults.surface`. Acceptable placeholder. |
| `border` | (none) | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:538-540 fills from `defaults.border`. |
| `radius` | (none) | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:541-543 fills from `defaults.radius_lg`. |
| `shadow` | (none) | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:544-546 fills from `defaults.shadow_enabled`. |
| `padding` | (none) | `16.0` (L:196, D:394) | :heavy_minus_sign: N/A -- (none) in platform-facts; reasonable default |

### 2.27 Expander / Disclosure

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `header_height` | (none) -- content-sized | `40.0` (L:199, D:397) | :heavy_minus_sign: N/A -- reasonable default |
| `arrow_size` | ~13px (measured) | `12.0` (L:200, D:398) | :x: **Wrong** -- platform-facts says ~13px, preset has 12. |
| `content_padding` | (none) -- app-defined | `16.0` (L:201, D:399) | :heavy_minus_sign: N/A |
| `radius` | (none) | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:549-551 fills from `defaults.radius`. |

### 2.28 Link

| Property | platform-facts macOS value | Preset value (file:line) | Verdict |
|----------|---------------------------|--------------------------|---------|
| `color` | `linkColor` | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:554-556 fills from `defaults.link`. Correct. |
| `visited` | (none) -- same as link | Not in preset | :arrows_counterclockwise: Inherited -- resolve.rs:557-559 fills from `defaults.link`. Correct. |
| `underline` | yes | `true` (L:174, D:372) | :white_check_mark: Correct |
| `background` | (none) -- inline | `#00000000` (L:175, D:373) | :white_check_mark: Correct -- transparent |
| `hover_bg` | (none) | `#007aff18` / `#0a84ff18` (L:176, D:374) | :heavy_minus_sign: N/A -- (none) in platform-facts; provides subtle accent hover |

---

### Summary of Issues Found

#### :x: Wrong Values (12 issues)

| # | Section | Property | Expected (platform-facts) | Actual (preset) | Fix |
|---|---------|----------|---------------------------|------------------|-----|
| 1 | 2.1.4 | `danger_foreground` (light) | `#1d1d1f` (labelColor) | `#ffffff` | See semantic note -- deliberate design choice vs literal API. Review needed. |
| 2 | 2.1.4 | `success_foreground` (light) | `#1d1d1f` (labelColor) | `#ffffff` | Same as #1. |
| 3 | 2.1.4 | `info_foreground` (light) | `#1d1d1f` (labelColor) | `#ffffff` | Same as #1. |
| 4 | 2.1.6 | `frame_width` | 0.5px | 1.0 | Change to `0.5`. Propagates to input.border_width and checkbox.border_width via inheritance. |
| 5 | 2.21 | `switch.track_radius` | 11 (half of track_height 22) | 10.0 | Change to `11.0`. |
| 6 | 2.22 | `dialog.content_padding` | ~20px (measured) | 24.0 | Change to `20.0`. |
| 7 | 2.22 | `dialog.button_spacing` | ~12px (measured) | 8.0 | Change to `12.0`. |
| 8 | 2.22 | `dialog.button_order` | primary rightmost (trailing) | `leading_affirmative` | Investigate: either fix preset to `trailing_affirmative`, or fix enum doc comments. See analysis above. |
| 9 | 2.22 | `dialog.icon_size` | 64px | 32.0 | Change to `64.0`. macOS shows 64px app icon in alerts. |
| 10 | 2.24 | `combo_box.arrow_area_width` | ~16-18px | 28.0 | Change to `17.0` (midpoint of measured range). |
| 11 | 2.25 | `segmented_control.padding_horizontal` | ~8-10px | 12.0 | Change to `9.0` (midpoint). |
| 12 | 2.27 | `expander.arrow_size` | ~13px (measured) | 12.0 | Change to `13.0`. |

#### :warning: Missing Values (4 issues)

| # | Section | Property | Expected (platform-facts) | Resolve fallback | Fix |
|---|---------|----------|---------------------------|-----------------|-----|
| 1 | 2.2 | `window.title_bar_font.weight` | Bold (700) | 400 (from defaults.font) | Add `[light.window.title_bar_font]` / `[dark.window.title_bar_font]` with `weight = 700`. |
| 2 | 2.19 | `text_scale.caption.size` | 10pt | 13.0 (from defaults.font.size) | Add `[light.text_scale.caption]` / `[dark.text_scale.caption]` with `size = 10.0`. |
| 3 | 2.19 | `text_scale.section_heading.weight` | 700 (Bold) | 400 | Add `[light.text_scale.section_heading]` with `weight = 700`. |
| 4 | 2.19 | `text_scale.dialog_title.size` + `display.size` | 22pt / 26pt | 13.0 / 13.0 | Add text_scale entries: `dialog_title.size = 22.0`, `display.size = 26.0`. |

#### Spacing scale gap (informational)

The macOS HIG standard widget gap of 8px has no exact tier in the spacing
scale (xs=4, s=6, m=10). This is not necessarily a bug -- the scale provides
flexible tiers -- but consumers targeting macOS specifically may want an 8px
option. Consider adding an `s2 = 8.0` tier or adjusting `s` to 8.0 in a
future version.

#### Status foreground semantic question (informational)

Issues #1-#3 (danger/success/info foreground in light mode) are flagged
because platform-facts documents macOS as providing `labelColor` (= normal
body text foreground) for these fields. The preset uses `#ffffff` which is a
contrast-on-status-background color. Both interpretations are valid depending
on use case. platform-facts SS2.1.4 lines 894-903 explicitly documents this
semantic mismatch. **Recommendation:** Keep `#ffffff` for now as it provides
better contrast when used as text-on-status-background, but add a TOML
comment documenting the deliberate deviation from `labelColor`.

---

## 27. Systematic KDE Breeze Preset Verification (parameter-by-parameter)

Exhaustive verification of `native-theme/src/presets/kde-breeze.toml`
against `docs/platform-facts.md` Chapter 2. Every property in every
section (2.1--2.28) is checked.

Legend:
- Preset = `kde-breeze.toml` (static preset, both light/dark)
- Live = `kde-breeze-live.toml` (geometry-only, no colors/fonts)
- Resolve = `src/resolve.rs` inheritance/safety-net behavior
- "Matches" = correct for the Breeze default kdeglobals / breezemetrics.h

### 2.1 Global Defaults

#### 2.1.1 Base Font

| Property       | platform-facts KDE column               | Preset light      | Preset dark        | Verdict |
|----------------|------------------------------------------|--------------------|--------------------|---------|
| `family`       | `[General] font` field 0 = Noto Sans    | "Noto Sans"        | "Noto Sans"        | OK |
| `size`         | `[General] font` field 1 = 10pt         | 10.0               | 10.0               | OK |
| `weight`       | `[General] font` field 4 = 400          | 400                | 400                | OK |
| `line_height`  | 1.36 (Noto Sans font metrics)           | 1.36               | 1.36               | OK |

#### 2.1.2 Monospace Font

| Property       | platform-facts KDE column                | Preset light      | Preset dark        | Verdict |
|----------------|------------------------------------------|--------------------|--------------------|---------|
| `family`       | `[General] fixed` field 0 = Hack         | "Hack"             | "Hack"             | OK |
| `size`         | `[General] fixed` field 1 = 10pt         | 10.0               | 10.0               | OK |
| `weight`       | `[General] fixed` field 4 = 400          | 400                | 400                | OK |

#### 2.1.3 Base Colors

| Property              | platform-facts KDE column                          | Preset light   | Preset dark    | Verdict |
|-----------------------|----------------------------------------------------|----------------|----------------|---------|
| `background`          | `[Colors:Window] BackgroundNormal`                 | #eff0f1        | #232629        | OK |
| `foreground`          | `[Colors:Window] ForegroundNormal`                 | #232629        | #fcfcfc        | OK |
| `accent`              | `[General] AccentColor` -> DecorationFocus         | #3daee9        | #3daee9        | OK |
| `accent_foreground`   | `[Colors:Selection] ForegroundNormal`              | #fcfcfc        | #fcfcfc        | OK |
| `surface`             | `[Colors:View] BackgroundNormal`                   | #ffffff        | #1b1e20        | OK |
| `border`              | **(preset)** -- derived from background            | #bcc0bf        | #4d545b        | OK -- reasonable derived value |
| `muted`               | `[Colors:Window] ForegroundInactive`               | #707d8a        | #a1a9b1        | OK |
| `shadow`              | **(none)** -- preset: #00000040/#60                | #00000040      | #00000060      | OK -- matches documented preset convention |
| `link`                | `[Colors:View] ForegroundLink`                     | #2980b9        | #1d99f3        | OK |
| `selection`           | `[Colors:Selection] BackgroundNormal`              | #3daee9        | #3daee9        | OK |
| `selection_foreground`| `[Colors:Selection] ForegroundNormal`              | #fcfcfc        | #fcfcfc        | OK |
| `selection_inactive`  | **(none)** -- selection bg unchanged on focus loss | (not in preset)| (not in preset)| OK -- resolve chain sets selection_inactive <- selection |
| `disabled_foreground` | `[Colors:View] ForegroundInactive`                 | #bcc0bf        | #4d545b        | OK |

#### 2.1.4 Status Colors

| Property              | platform-facts KDE column                    | Preset light   | Preset dark    | Verdict |
|-----------------------|----------------------------------------------|----------------|----------------|---------|
| `danger`              | `[Colors:View] ForegroundNegative`           | #da4453        | #da4453        | OK |
| `danger_foreground`   | `[Colors:Window] ForegroundNormal` (note 1)  | #fcfcfc        | #fcfcfc        | WARNING |
| `warning`             | `[Colors:View] ForegroundNeutral`            | #f67400        | #f67400        | OK |
| `warning_foreground`  | `[Colors:Window] ForegroundNormal` (note 1)  | #232629        | #fcfcfc        | WARNING |
| `success`             | `[Colors:View] ForegroundPositive`           | #27ae60        | #27ae60        | OK |
| `success_foreground`  | `[Colors:Window] ForegroundNormal` (note 1)  | #fcfcfc        | #fcfcfc        | WARNING |
| `info`                | `[Colors:View] ForegroundActive`             | #3daee9        | #3daee9        | OK |
| `info_foreground`     | `[Colors:Window] ForegroundNormal` (note 1)  | #fcfcfc        | #fcfcfc        | WARNING |

**Note 1 -- status foreground semantic divergence:** platform-facts.md
SS2.1.4 documents that KDE `*_foreground` should be `[Colors:Window]
ForegroundNormal` (body text color: L=#232629, D=#fcfcfc) -- suitable
for text *alongside* a status indicator, not text *on* a status-colored
background. The preset instead uses contrast-foreground values (white
#fcfcfc on red/green/blue in light mode), which is the "text on status
background" interpretation. This is a deliberate design choice
documented in the platform-facts footnote. Both interpretations are
valid; the preset chose the Windows-like semantic. The light-mode
`warning_foreground = #232629` is the only one that matches the KDE
native value. This is a known semantic divergence, not a bug -- no fix
needed.

#### 2.1.5 Focus Ring

| Property  | platform-facts KDE column                      | Preset light | Preset dark | Verdict |
|-----------|-------------------------------------------------|-------------|-------------|---------|
| `color`   | `[Colors:View] DecorationFocus`                | #3daee9      | #3daee9     | OK |
| `width`   | Breeze: 1.001px (stroke); 2px margin           | 1.0          | 1.0         | OK -- 1.0 rounds 1.001px stroke |
| `offset`  | Breeze: 2px margin (outset)                    | 2.0          | 2.0         | OK |

#### 2.1.6 Global Geometry

| Property           | platform-facts KDE column                  | Preset light | Preset dark | Verdict |
|--------------------|--------------------------------------------|-------------|-------------|---------|
| `radius`           | `Frame_FrameRadius` = 5px                  | 5.0          | 5.0         | OK |
| `radius_lg`        | **(none)** -- preset                       | 8.0          | 8.0         | OK -- no native equivalent |
| `frame_width`      | `PenWidth::Frame` = 1.001px               | 1.0          | 1.0         | OK -- 1.0 rounds 1.001px |
| `disabled_opacity` | **(none)** -- palette blending             | 0.5          | 0.5         | OK -- reasonable fallback |
| `border_opacity`   | 0.2 **(preset)**                           | 0.2          | 0.2         | OK |
| `shadow_enabled`   | yes (KWin compositor)                      | true         | true        | OK |

#### 2.1.7 Accessibility

| Property              | platform-facts KDE column                | Preset light | Preset dark | Verdict |
|-----------------------|------------------------------------------|-------------|-------------|---------|
| `text_scaling_factor` | `forceFontDPI / 96`                      | 1.0          | 1.0         | OK |
| `reduce_motion`       | `AnimationDurationFactor = 0`            | false        | false       | OK |
| `high_contrast`       | **(none)**                               | false        | false       | OK |
| `reduce_transparency` | **(none)**                               | false        | false       | OK |

#### 2.1.8 Icon Sizes

| Context   | platform-facts KDE column               | Preset light | Preset dark | Verdict |
|-----------|------------------------------------------|-------------|-------------|---------|
| `toolbar` | MainToolbar: 22                          | 22.0         | 22.0        | OK |
| `small`   | Small: 16                                | 16.0         | 16.0        | OK |
| `large`   | Desktop: 48 (Breeze default)             | 48.0         | 48.0        | OK |
| `dialog`  | Dialog: 32                               | 32.0         | 32.0        | OK |
| `panel`   | Panel: 48 (Breeze default)               | 48.0         | 48.0        | OK |

### 2.2 Window / Application Chrome

All window properties are inherited via resolve.rs. The preset does not
set any window-specific values.

| Property                 | platform-facts KDE column               | Resolved from         | Verdict |
|--------------------------|-----------------------------------------|-----------------------|---------|
| `background`             | <- `defaults.background`               | defaults.background   | OK |
| `foreground`             | <- `defaults.foreground`               | defaults.foreground   | OK |
| `border`                 | `[WM]` decoration colors               | defaults.border       | OK |
| `title_bar_background`   | `[WM] activeBackground`                | defaults.surface      | OK |
| `title_bar_foreground`   | `[WM] activeForeground`                | defaults.foreground   | OK |
| `inactive_title_bar_bg`  | `[WM] inactiveBackground`              | <- active title bg    | OK |
| `inactive_title_bar_fg`  | `[WM] inactiveForeground`              | <- active title fg    | OK |
| `title_bar_font`         | `[WM] activeFont`                      | defaults.font         | OK |
| `radius`                 | <- `defaults.radius_lg`                | 8.0                   | OK |
| `shadow`                 | <- `defaults.shadow_enabled`           | true                  | OK |

### 2.3 Button

| Property            | platform-facts KDE column               | Preset value   | Verdict |
|---------------------|------------------------------------------|----------------|---------|
| `background`        | `[Colors:Button] BackgroundNormal`       | L:#fcfcfc D:#31363b | OK |
| `foreground`        | `[Colors:Button] ForegroundNormal`       | L:#232629 D:#fcfcfc | OK |
| `border`            | <- `defaults.border`                    | (inherited)    | OK |
| `font`              | <- `defaults.font`                      | (inherited)    | OK |
| `min_width`         | `Button_MinWidth` = 80                   | 80.0           | OK |
| `min_height`        | **(none)** -- sizes to content           | 32.0           | OK -- reasonable |
| `padding_horizontal`| `Button_MarginWidth` = 6                 | 6.0            | OK |
| `padding_vertical`  | 5 **(measured)** Breeze frame+margin     | 6.0            | MISMATCH -- 1px over |
| `radius`            | <- `defaults.radius`                    | (inherited=5.0)| OK |
| `icon_spacing`      | `Button_ItemSpacing` = 4                 | 4.0            | OK |
| `primary_bg`        | <- `defaults.accent`                    | (inherited)    | OK |
| `primary_fg`        | <- `defaults.accent_foreground`         | (inherited)    | OK |
| `disabled_opacity`  | <- `defaults.disabled_opacity`          | (inherited=0.5)| OK |
| `shadow`            | <- `defaults.shadow_enabled`            | (inherited)    | OK |

### 2.4 Text Input

| Property              | platform-facts KDE column                | Preset value   | Verdict |
|-----------------------|------------------------------------------|----------------|---------|
| `background`          | `[Colors:View] BackgroundNormal`         | L:#ffffff D:#1b1e20 | OK |
| `foreground`          | `[Colors:View] ForegroundNormal`         | L:#232629 D:#fcfcfc | OK |
| `border`              | <- `defaults.border`                    | (inherited)    | OK |
| `placeholder`         | `[Colors:View] ForegroundInactive`       | (inherited=muted) | OK |
| `caret`               | `[Colors:View] DecorationFocus` = accent | (none->foreground) | MISMATCH |
| `selection`           | <- `defaults.selection`                 | (inherited)    | OK |
| `selection_foreground`| <- `defaults.selection_foreground`      | (inherited)    | OK |
| `font`                | <- `defaults.font`                      | (inherited)    | OK |
| `min_height`          | **(none)** -- sizes to content           | 32.0           | OK -- reasonable |
| `padding_horizontal`  | `LineEdit_FrameWidth` = 6                | 6.0            | OK |
| `padding_vertical`    | 3 **(measured)** Breeze frame            | 6.0            | MISMATCH -- 3px over |
| `radius`              | <- `defaults.radius`                    | (inherited=5.0)| OK |
| `border_width`        | <- `defaults.frame_width`               | (inherited=1.0)| OK |

### 2.5 Checkbox / Radio Button

| Property        | platform-facts KDE column                | Preset value | Verdict |
|-----------------|------------------------------------------|-------------|---------|
| `indicator_size`| `CheckBox_Size` = 20                     | 20.0         | OK |
| `spacing`       | `CheckBox_ItemSpacing` = 4               | 4.0          | OK |
| `radius`        | <- `defaults.radius`                    | (inherited=5.0) | OK |
| `border_width`  | <- `defaults.frame_width`               | (inherited=1.0) | OK |
| `checked_bg`    | <- `defaults.accent`                    | (inherited)  | OK |

### 2.6 Menu

| Property            | platform-facts KDE column                    | Preset value | Verdict |
|---------------------|----------------------------------------------|-------------|---------|
| `background`        | `[Colors:Window] BackgroundNormal`           | (inherited)  | OK |
| `foreground`        | `[Colors:Window] ForegroundNormal`           | (inherited)  | OK |
| `separator`         | <- `defaults.border`                        | (inherited)  | OK |
| `font`              | `[General] menuFont`                         | (inherited)  | OK |
| `item_height`       | **(none)** -- sizes to font                  | 28.0         | OK -- reasonable |
| `padding_horizontal`| `MenuItem_MarginWidth` = 4                   | 4.0          | OK |
| `padding_vertical`  | `MenuItem_MarginHeight` = 4                  | 4.0          | OK |
| `icon_spacing`      | 8 **(Breeze src)** icon-text gap             | 8.0          | OK |

### 2.7 Tooltip

| Property       | platform-facts KDE column                      | Preset value | Verdict |
|----------------|-------------------------------------------------|-------------|---------|
| `background`   | `[Colors:Tooltip] BackgroundNormal`             | L:#f7f7f7 D:#31363b | OK |
| `foreground`   | `[Colors:Tooltip] ForegroundNormal`             | L:#232629 D:#fcfcfc | OK |
| `font`         | <- `defaults.font`                             | (inherited)  | OK |
| `padding_h`    | `ToolTip_FrameWidth` = 3                       | 3.0          | OK |
| `padding_v`    | `ToolTip_FrameWidth` = 3                       | 3.0          | OK |
| `max_width`    | **(none)** -- preset: 300                      | 300.0        | OK |
| `radius`       | <- `defaults.radius`                           | (inherited=5.0) | OK |

### 2.8 Scrollbar

| Property          | platform-facts KDE column                        | Preset value | Verdict |
|-------------------|--------------------------------------------------|-------------|---------|
| `track`           | `defaults.background`                            | (safety net) | OK |
| `thumb`           | **(Breeze src)** thumb color                     | (inherited=muted) | OK |
| `thumb_hover`     | **(Breeze src)** thumb hover                     | (inherited=muted) | OK |
| `width`           | `ScrollBar_Extend` = 21                          | 21.0         | OK |
| `min_thumb_height`| `ScrollBar_MinSliderHeight` = 20                 | 20.0         | OK |
| `slider_width`    | `ScrollBar_SliderWidth` = 8                      | 8.0          | OK |
| `overlay_mode`    | **(none)** -- always persistent                  | false        | OK |

### 2.9 Slider

| Property       | platform-facts KDE column                    | Preset value | Verdict |
|----------------|----------------------------------------------|-------------|---------|
| `fill`         | <- `defaults.accent`                        | (inherited)  | OK |
| `track`        | <- `defaults.muted`                         | (inherited)  | OK |
| `thumb`        | <- `defaults.surface`                       | (inherited)  | OK |
| `track_height` | `Slider_GrooveThickness` = 6                 | 6.0          | OK |
| `thumb_size`   | `Slider_ControlThickness` = 20               | 20.0         | OK |
| `tick_length`  | `Slider_TickLength` = 8                      | 8.0          | OK |

### 2.10 Progress Bar

| Property    | platform-facts KDE column                    | Preset value | Verdict |
|-------------|----------------------------------------------|-------------|---------|
| `fill`      | <- `defaults.accent`                        | (inherited)  | OK |
| `track`     | <- `defaults.muted`                         | (inherited)  | OK |
| `height`    | `ProgressBar_Thickness` = 6                  | 6.0          | OK |
| `min_width` | **(none)** -- no minimum                    | 6.0          | OK |
| `radius`    | <- `defaults.radius`                        | (inherited=5.0) | OK |

### 2.11 Tab Bar

| Property            | platform-facts KDE column                | Preset value | Verdict |
|---------------------|------------------------------------------|-------------|---------|
| `background`        | <- `defaults.background`                | (inherited)  | OK |
| `foreground`        | <- `defaults.foreground`                | (inherited)  | OK |
| `active_background` | <- `defaults.background`                | (inherited)  | OK |
| `active_foreground` | <- `defaults.foreground`                | (inherited)  | OK |
| `bar_background`    | <- `defaults.background`                | (inherited)  | OK |
| `min_width`         | `TabBar_TabMinWidth` = 80                | 80.0         | OK |
| `min_height`        | `TabBar_TabMinHeight` = 30               | 30.0         | OK |
| `padding_horizontal`| `TabBar_TabMarginWidth` = 8              | 8.0          | OK |
| `padding_vertical`  | `TabBar_TabMarginHeight` = 4             | 4.0          | OK |

### 2.12 Sidebar

| Property     | platform-facts KDE column                            | Preset value | Verdict |
|--------------|------------------------------------------------------|-------------|---------|
| `background` | `[Colors:Complementary] BackgroundNormal`            | L:#dee0e2 D:#272c31 | OK |
| `foreground` | `[Colors:Complementary] ForegroundNormal`            | L:#232629 D:#fcfcfc | OK |

### 2.13 Toolbar

| Property       | platform-facts KDE column                        | Preset value | Verdict |
|----------------|--------------------------------------------------|-------------|---------|
| `font`         | `[General] toolBarFont`                          | (inherited)  | OK |
| `height`       | **(none)** -- sizes to content                   | 40.0         | OK -- reasonable |
| `item_spacing` | `ToolBar_ItemSpacing` = 0                        | 0.0          | OK |
| `padding`      | `ToolBar_ItemMargin` = 6                         | 6.0          | OK |

### 2.14 Status Bar

| Property | platform-facts KDE column | Preset value | Verdict |
|----------|---------------------------|-------------|---------|
| `font`   | <- `defaults.font`       | (inherited)  | OK |

### 2.15 List / Table

| Property              | platform-facts KDE column                    | Preset value | Verdict |
|-----------------------|----------------------------------------------|-------------|---------|
| `background`          | `[Colors:View] BackgroundNormal`             | (safety net) | OK |
| `foreground`          | `[Colors:View] ForegroundNormal`             | (inherited)  | OK |
| `alternate_row`       | `[Colors:View] BackgroundAlternate`          | L:#f5f6f7 D:#2a2e32 | OK |
| `selection`           | <- `defaults.selection`                     | (inherited)  | OK |
| `selection_foreground`| <- `defaults.selection_foreground`           | (inherited)  | OK |
| `header_background`   | `[Colors:Header] BackgroundNormal`           | (inherited=surface) | OK |
| `header_foreground`   | `[Colors:Header] ForegroundNormal`           | (inherited=foreground) | OK |
| `grid_color`          | **(none)** -- Qt views use palette pen       | (inherited=border) | OK |
| `item_height`         | **(none)** -- sizes to content               | 28.0         | OK -- reasonable |
| `padding_horizontal`  | `ItemView_ItemMarginLeft/Right` = 2          | 2.0          | OK |
| `padding_vertical`    | `ItemView_ItemMarginTop/Bottom` = 1          | 1.0          | OK |

### 2.16 Popover / Dropdown

| Property     | platform-facts KDE column           | Preset value | Verdict |
|--------------|--------------------------------------|-------------|---------|
| `background` | <- `defaults.background`            | L:#ffffff D:#31363b | OK |
| `foreground` | <- `defaults.foreground`            | L:#232629 D:#fcfcfc | OK |
| `border`     | <- `defaults.border`                | (inherited)  | OK |
| `radius`     | <- `defaults.radius_lg`             | (inherited=8.0) | OK |

### 2.17 Splitter

| Property | platform-facts KDE column    | Preset value | Verdict |
|----------|------------------------------|-------------|---------|
| `width`  | `Splitter_SplitterWidth` = 1 | 1.0          | OK |

### 2.18 Separator

| Property | platform-facts KDE column    | Preset value | Verdict |
|----------|------------------------------|-------------|---------|
| `color`  | <- `defaults.border`         | L:#c4c8c6 D:#3e4550 | OK -- custom separator |

### 2.19 Text Scale

Text scale entries are filled by resolve.rs from defaults.font and
defaults.line_height. No explicit text_scale in the preset TOML.

| Role              | platform-facts KDE column                    | Resolved value (at 10pt base) | Verdict |
|-------------------|----------------------------------------------|-------------------------------|---------|
| `caption`         | `smallestReadableFont` = 8pt, 400            | 10pt, 400 (from defaults.font)| OK -- resolve uses base font |
| `section_heading` | Kirigami Level 2: body x 1.20                | 10pt, 400 (from defaults.font)| OK -- resolve uses base font |
| `dialog_title`    | Kirigami Level 1: body x 1.35                | 10pt, 400 (from defaults.font)| OK -- resolve uses base font |
| `display`         | **(none)** -- no equivalent                  | 10pt, 400 (from defaults.font)| OK -- resolve uses base font |

### 2.20 Layout Spacing

| Context                  | platform-facts KDE value           | Closest preset slot | Preset value | Verdict |
|--------------------------|------------------------------------|---------------------|-------------|---------|
| Standard widget gap      | `Layout_DefaultSpacing` = 6        | `spacing.s`         | 6.0          | OK |
| Nested container margin  | `Layout_ChildMarginWidth` = 6      | `spacing.s`         | 6.0          | OK |
| Window content margin    | `Layout_TopLevelMarginWidth` = 10  | (between m=8, l=12) | --           | OK |

### 2.21 Switch / Toggle

| Property          | platform-facts KDE column               | Preset value   | Verdict |
|-------------------|-----------------------------------------|----------------|---------|
| `track_width`     | QQC2: ~36 (font-derived)               | 36.0           | OK |
| `track_height`    | QQC2: ~18 (font-derived)               | 18.0           | OK |
| `thumb_size`      | QQC2: ~18 (= track height)             | 18.0           | OK |
| `track_radius`    | half height (pill)                      | 9.0            | OK |
| `checked_bg`      | <- `defaults.accent`                   | (inherited)    | OK |
| `unchecked_bg`    | **(preset)** trough color              | L:#bcc0bf D:#4d545b | OK |
| `thumb_bg`        | **(preset)** slider color              | #fcfcfc        | OK |

### 2.22 Dialog

| Property              | platform-facts KDE column                    | Preset value | Verdict |
|-----------------------|----------------------------------------------|-------------|---------|
| `min_width`           | **(none)** -- sizes to content               | 320.0        | OK -- reasonable |
| `max_width`           | **(none)** -- sizes to content               | 560.0        | OK -- reasonable |
| `min_height`          | **(none)** -- sizes to content               | 140.0        | OK -- reasonable |
| `max_height`          | **(none)** -- sizes to content               | 600.0        | OK -- reasonable |
| `content_padding`     | `Layout_TopLevelMarginWidth` = 10            | 24.0         | MISMATCH -- 14px over |
| `button_spacing`      | `Layout_DefaultSpacing` = 6                  | 8.0          | MISMATCH -- 2px over |
| `button_order`        | OK left of Cancel (right-aligned)            | "leading_affirmative" | OK |
| `title_font`          | <- `defaults.font`                          | (inherited)  | OK |
| `radius`              | <- `defaults.radius_lg`                     | (inherited=8.0) | OK |
| `icon_size`           | **(none)** -- per-dialog                    | 32.0         | OK -- matches Dialog icon default |

### 2.23 Spinner / Progress Ring

| Property      | platform-facts KDE column                | Preset value | Verdict |
|---------------|------------------------------------------|-------------|---------|
| `diameter`    | QQC2 BusyIndicator: 36                   | 36.0         | OK |
| `min_size`    | **(none)**                               | 16.0         | OK -- reasonable |
| `stroke_width`| **(none)** -- icon-based                 | 2.0          | OK -- reasonable |
| `fill`        | <- `defaults.foreground`                | (safety net) | OK |

### 2.24 ComboBox / Dropdown Trigger

| Property            | platform-facts KDE column                | Preset value | Verdict |
|---------------------|------------------------------------------|-------------|---------|
| `min_height`        | **(none)** -- sizes to content           | 32.0         | OK -- reasonable |
| `min_width`         | **(none)** -- sizes to content           | 120.0        | OK -- reasonable |
| `padding_horizontal`| `ComboBox_FrameWidth` = 6               | 12.0         | MISMATCH -- double native |
| `arrow_size`        | `MenuButton_IndicatorWidth` = 20         | 12.0         | MISMATCH -- 8px under |
| `arrow_area_width`  | 20px                                     | 28.0         | MISMATCH -- 8px over |
| `radius`            | <- `defaults.radius`                    | (inherited=5.0) | OK |

### 2.25 Segmented Control

| Property            | platform-facts KDE column                | Preset value | Verdict |
|---------------------|------------------------------------------|-------------|---------|
| `segment_height`    | `TabBar_TabMinHeight` = 30 (proxy)       | 28.0         | MISMATCH -- 2px under |
| `separator_width`   | `TabBar_TabOverlap` = 1                  | 1.0          | OK |
| `padding_horizontal`| `TabBar_TabMarginWidth` = 8              | 12.0         | MISMATCH -- 4px over |
| `radius`            | <- `defaults.radius`                    | (inherited=5.0) | OK |

### 2.26 Card / Container

KDE has no native card component. All values are preset-chosen.

| Property     | platform-facts KDE column    | Preset value | Verdict |
|--------------|------------------------------|-------------|---------|
| `background` | **(none)**                   | (inherited=surface) | OK |
| `border`     | **(none)**                   | (inherited=border) | OK |
| `radius`     | **(none)**                   | (inherited=radius_lg=8.0) | OK |
| `shadow`     | **(none)**                   | (inherited=shadow_enabled) | OK |
| `padding`    | **(none)**                   | 16.0         | OK -- reasonable |

### 2.27 Expander / Disclosure

| Property          | platform-facts KDE column                | Preset value | Verdict |
|-------------------|------------------------------------------|-------------|---------|
| `header_height`   | **(none)** -- content-sized              | 40.0         | OK -- reasonable |
| `arrow_size`      | `ItemView_ArrowSize` = 10                | 10.0         | OK |
| `content_padding` | **(none)** -- app-defined                | 16.0         | OK -- reasonable |
| `radius`          | `Frame_FrameRadius` = 5                  | (inherited=5.0) | OK |

### 2.28 Link

| Property      | platform-facts KDE column                | Preset value | Verdict |
|---------------|------------------------------------------|-------------|---------|
| `color`       | `ForegroundLink`                         | (inherited=link) | OK |
| `visited`     | `ForegroundVisited`                      | (inherited=link) | OK -- no visited differentiation |
| `underline`   | yes (Kirigami LinkButton)                | true         | OK |
| `background`  | **(none)** -- inline                     | #00000000    | OK -- transparent |
| `hover_bg`    | **(none)**                               | #3daee918    | OK -- subtle accent tint |

### Live preset cross-check

Every geometry value in `kde-breeze-live.toml` matches `kde-breeze.toml`
exactly. No discrepancies between the two files.

---

### Issues Found (MISMATCH items only)

| # | Section | Property | Preset | Expected | Severity | Source |
|---|---------|----------|--------|----------|----------|--------|
| 1 | 2.3 Button | `padding_vertical` | 6.0 | 5.0 | Low | platform-facts: "5 (measured) Breeze frame+margin" |
| 2 | 2.4 Input | `padding_vertical` | 6.0 | 3.0 | Medium | platform-facts: "3 (measured) Breeze frame" |
| 3 | 2.4 Input | `caret` | (none->fg) | accent #3daee9 | Medium | platform-facts: `[Colors:View] DecorationFocus` |
| 4 | 2.22 Dialog | `content_padding` | 24.0 | 10.0 | Medium | `Layout_TopLevelMarginWidth` = 10 (breezemetrics.h) |
| 5 | 2.22 Dialog | `button_spacing` | 8.0 | 6.0 | Low | `Layout_DefaultSpacing` = 6 (breezemetrics.h) |
| 6 | 2.24 ComboBox | `padding_horizontal` | 12.0 | 6.0 | Medium | `ComboBox_FrameWidth` = 6 (breezemetrics.h) |
| 7 | 2.24 ComboBox | `arrow_size` | 12.0 | 20.0 | Medium | `MenuButton_IndicatorWidth` = 20 (breezemetrics.h) |
| 8 | 2.24 ComboBox | `arrow_area_width` | 28.0 | 20.0 | Low | breezemetrics.h derived |
| 9 | 2.25 Segmented | `segment_height` | 28.0 | 30.0 | Low | `TabBar_TabMinHeight` = 30 (proxy) |
| 10 | 2.25 Segmented | `padding_horizontal` | 12.0 | 8.0 | Low | `TabBar_TabMarginWidth` = 8 |

### Solutions

**Issue 1 -- button.padding_vertical (6 -> 5):**
`kde-breeze.toml` L82,L280 and `kde-breeze-live.toml` L38,L170:
change `padding_vertical = 6.0` to `padding_vertical = 5.0`.

**Issue 2 -- input.padding_vertical (6 -> 3):**
`kde-breeze.toml` L90,L288 and `kde-breeze-live.toml` L43,L178:
change `padding_vertical = 6.0` to `padding_vertical = 3.0`.

**Issue 3 -- input.caret missing (should be accent):**
Add `caret = "#3daee9"` to `[light.input]` and `[dark.input]` in
`kde-breeze.toml`. Live preset does not need this (OS reader fills
colors). Do NOT change the resolve.rs safety net -- macOS and Windows
use foreground for caret, so the fallback is correct for them.

**Issue 4 -- dialog.content_padding (24 -> 10):**
`kde-breeze.toml` L169,L367 and `kde-breeze-live.toml` L107,L241:
change `content_padding = 24.0` to `content_padding = 10.0`.

**Issue 5 -- dialog.button_spacing (8 -> 6):**
`kde-breeze.toml` L170,L368 and `kde-breeze-live.toml` L108,L242:
change `button_spacing = 8.0` to `button_spacing = 6.0`.

**Issue 6 -- combo_box.padding_horizontal (12 -> 6):**
`kde-breeze.toml` L186,L384 and `kde-breeze-live.toml` L122,L256:
change `padding_horizontal = 12.0` to `padding_horizontal = 6.0`.

**Issue 7 -- combo_box.arrow_size (12 -> 20):**
`kde-breeze.toml` L187,L385 and `kde-breeze-live.toml` L123,L257:
change `arrow_size = 12.0` to `arrow_size = 20.0`.

**Issue 8 -- combo_box.arrow_area_width (28 -> 20):**
`kde-breeze.toml` L188,L386 and `kde-breeze-live.toml` L124,L258:
change `arrow_area_width = 28.0` to `arrow_area_width = 20.0`.

**Issue 9 -- segmented_control.segment_height (28 -> 30):**
`kde-breeze.toml` L191,L389 and `kde-breeze-live.toml` L127,L261:
change `segment_height = 28.0` to `segment_height = 30.0`.

**Issue 10 -- segmented_control.padding_horizontal (12 -> 8):**
`kde-breeze.toml` L193,L391 and `kde-breeze-live.toml` L129,L263:
change `padding_horizontal = 12.0` to `padding_horizontal = 8.0`.

---

## 28. Systematic Adwaita Preset Verification (parameter-by-parameter)

Cross-references every property in `docs/platform-facts.md` Chapter 2
(sections 2.1-2.28) against:
- `native-theme/src/presets/adwaita.toml` (static preset, light + dark)
- `native-theme/src/presets/adwaita-live.toml` (geometry-only live preset)
- `native-theme/src/resolve.rs` (inheritance/safety-net rules)

Legend: preset value shown as `L: / D:` for light/dark.

### 2.1 Global Defaults

#### 2.1.1 Base Font

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `font.family` | gsetting `font-name` -> Adwaita Sans (GNOME 48+) | `"Adwaita Sans"` | OK -- matches GNOME 48+ default |
| `font.size` | gsetting `font-name` -> 11 | `11.0` | OK |
| `font.weight` | gsetting `font-name` -> 400 | `400` | OK |
| `line_height` | Adwaita Sans (Inter metrics): 1.21 | `1.21` | OK |

#### 2.1.2 Monospace Font

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `mono_font.family` | gsetting `monospace-font-name` -> Adwaita Mono | `"Adwaita Mono"` | OK |
| `mono_font.size` | gsetting -> 11 | `11.0` | OK |
| `mono_font.weight` | gsetting -> 400 | `400` | OK |

#### 2.1.3 Base Colors

| Property | platform-facts GNOME | Preset L / D | Status |
|----------|---------------------|--------------|--------|
| `background` | **(Adwaita CSS)** | L:`#fafafb` D:`#222226` | OK -- Adwaita CSS |
| `foreground` | **(Adwaita CSS)** | L:`#2e3436` D:`#ffffff` | OK |
| `accent` | Portal `accent-color` | L:`#3584e4` D:`#3584e4` | OK -- default blue |
| `accent_foreground` | **(Adwaita CSS)** | L:`#ffffff` D:`#ffffff` | OK |
| `surface` | **(Adwaita CSS)** | L:`#ffffff` D:`#303034` | OK |
| `border` | **(Adwaita CSS)** | L:`#d5d5d5` D:`#4a4a4e` | OK |
| `muted` | **(Adwaita CSS)** | L:`#929292` D:`#929292` | OK |
| `shadow` | **(Adwaita CSS)** | L:`#00000018` D:`#00000040` | OK |
| `link` | **(Adwaita CSS)** | L:`#1b6acb` D:`#5b9bf8` | OK |
| `selection` | **(Adwaita CSS)** | L:`#3584e4` D:`#3584e4` | OK |
| `selection_foreground` | **(Adwaita CSS)** | L:`#ffffff` D:`#ffffff` | OK |
| `selection_inactive` | `(none)` -- `:backdrop` CSS | Not in preset; resolved from `selection` by Phase 1 | OK -- correct inheritance |
| `disabled_foreground` | **(Adwaita CSS)** | L:`#cccccc` D:`#4a4a4e` | OK |

#### 2.1.4 Status Colors

| Property | platform-facts GNOME | Preset L / D | Status |
|----------|---------------------|--------------|--------|
| `danger` | **(Adwaita CSS)** | L:`#e01b24` D:`#c01c28` | OK |
| `danger_foreground` | **(Adwaita CSS)** footnote 1: GNOME provides **normal body foreground** | L:`#ffffff` D:`#ffffff` | **WRONG** -- see below |
| `warning` | **(Adwaita CSS)** | L:`#e5a50a` D:`#cd9309` | OK |
| `warning_foreground` | **(Adwaita CSS)** footnote 1: normal body foreground | L:`#2e3436` D:`#ffffff` | **MIXED** -- dark OK, light debatable |
| `success` | **(Adwaita CSS)** | L:`#2ec27e` D:`#26a269` | OK |
| `success_foreground` | **(Adwaita CSS)** footnote 1: normal body foreground | L:`#ffffff` D:`#ffffff` | **WRONG** -- see below |
| `info` | **(Adwaita CSS)** | L:`#3584e4` D:`#3584e4` | OK |
| `info_foreground` | **(Adwaita CSS)** footnote 1: normal body foreground | L:`#ffffff` D:`#ffffff` | **WRONG** -- see below |

#### 2.1.5 Focus Ring

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `focus_ring_color` | Adwaita `@accent_color` | L:`#3584e4` D:`#3584e4` | OK -- matches accent |
| `focus_ring_width` | libadwaita CSS: 2px | `2.0` | OK |
| `focus_ring_offset` | libadwaita CSS: -2px (inset) | `-2.0` | OK |

#### 2.1.6 Global Geometry

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `radius` | Adwaita: 9px | `9.0` | OK |
| `radius_lg` | Adwaita: 15px | `15.0` | OK |
| `frame_width` | Adwaita: 1px | `1.0` | OK |
| `disabled_opacity` | Adwaita: 0.5 | `0.5` | OK |
| `border_opacity` | 0.15 **(preset)** | `0.15` | OK |
| `shadow_enabled` | yes | `true` | OK |

#### 2.1.7 Accessibility

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `text_scaling_factor` | gsetting (default 1.0) | `1.0` | OK -- live reader overrides |
| `reduce_motion` | gsettings `enable-animations` | `false` | OK -- live reader overrides |
| `high_contrast` | `a11y.interface high-contrast` | `false` | OK -- live reader overrides |
| `reduce_transparency` | `(none)` | `false` | OK -- GNOME has none; false is correct |

#### 2.1.8 Icon Sizes

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `toolbar` | `GTK_ICON_SIZE_NORMAL`: 16 | `16.0` | OK |
| `small` | `GTK_ICON_SIZE_NORMAL`: 16 | `16.0` | OK |
| `large` | `GTK_ICON_SIZE_LARGE`: 32 | `32.0` | OK |
| `dialog` | `(none)` -- 48 (GTK3 legacy) | `22.0` | MISMATCH -- preset has 22, platform-facts says 48 GTK3 legacy or no GNOME value |
| `panel` | `(none)` | `20.0` | OK -- no GNOME native value, reasonable preset |

#### 2.1 Spacing (not in platform-facts per-property, but derived from 2.20)

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `spacing.xxs` | N/A (no abstract scale) | `2.0` | OK -- reasonable preset |
| `spacing.xs` | N/A | `4.0` | OK |
| `spacing.s` | 6 **(measured)** widget gap | `6.0` | OK |
| `spacing.m` | 12 **(measured)** container margin | `12.0` | OK |
| `spacing.l` | 18 **(measured)** section gap | `18.0` | OK |
| `spacing.xl` | N/A | `24.0` | OK |
| `spacing.xxl` | N/A | `36.0` | OK |

---

### 2.2 Window / Application Chrome

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `background` | <- `defaults.background` | Not in preset; resolved by inheritance | OK |
| `foreground` | <- `defaults.foreground` | Not in preset; resolved by inheritance | OK |
| `border` | **(Adwaita CSS)** window border | Not in preset; resolved from `defaults.border` | OK |
| `title_bar_background` | libadwaita `headerbar` bg | Not in preset; resolved from `defaults.surface` | OK -- headerbar bg matches surface |
| `title_bar_foreground` | libadwaita `headerbar` fg | L:`#2e3436` D:`#ffffff` | OK -- explicitly set |
| `inactive_title_bar_bg` | `(none)` -- `:backdrop` CSS | Not in preset; resolved from active by Phase 4 | OK |
| `inactive_title_bar_fg` | `(none)` -- `:backdrop` CSS | Not in preset; resolved from active by Phase 4 | OK |
| `title_bar_font.family` | gsetting `titlebar-font` -> Adwaita Sans | Not in preset; resolved from `defaults.font` | OK -- Adwaita Sans matches |
| `title_bar_font.size` | gsetting -> 11 | Not in preset; resolved from `defaults.font` | OK |
| `title_bar_font.weight` | gsetting -> Bold (700) | Not in preset; resolved from `defaults.font` (400) | **WRONG** -- should be 700 |
| `radius` | <- `defaults.radius_lg` | Not in preset; resolved from `defaults.radius_lg` (15) | OK |
| `shadow` | <- `defaults.shadow_enabled` | Not in preset; resolved | OK |

---

### 2.3 Button

| Property | platform-facts GNOME | Preset L / D | Status |
|----------|---------------------|--------------|--------|
| `background` | libadwaita `.button` bg | L:`#e8e8e8` D:`#3a3a3e` | OK -- Adwaita button bg |
| `foreground` | libadwaita `.button` fg | L:`#2e3436` D:`#ffffff` | OK |
| `border` | <- `defaults.border` | Not in preset; resolved | OK |
| `font` | <- `defaults.font` | Not in preset; resolved | OK |
| `min_width` | **(Adwaita CSS)**: none | `64.0` | **WRONG** -- platform-facts says GNOME has no min-width; 64 is not from Adwaita CSS |
| `min_height` | **(Adwaita CSS)**: 24 (34 w/ padding) | `34.0` | OK -- 34 = 24 min-height + 2*5 padding |
| `padding_horizontal` | **(Adwaita CSS)**: 10 | `10.0` | OK |
| `padding_vertical` | **(Adwaita CSS)**: 5 | `5.0` | OK |
| `radius` | <- `defaults.radius` | Not in preset; resolved (9) | OK |
| `icon_spacing` | **(Adwaita CSS)**: 8 | `6.0` | **WRONG** -- should be 8 |
| `primary_bg` | <- `defaults.accent` | Not in preset; resolved | OK |
| `primary_fg` | <- `defaults.accent_foreground` | Not in preset; resolved | OK |
| `disabled_opacity` | <- `defaults.disabled_opacity` | Not in preset; resolved (0.5) | OK |
| `shadow` | <- `defaults.shadow_enabled` | Not in preset; resolved | OK |

---

### 2.4 Text Input

| Property | platform-facts GNOME | Preset L / D | Status |
|----------|---------------------|--------------|--------|
| `background` | libadwaita `.entry` bg | L:`#ffffff` D:`#303034` | OK |
| `foreground` | libadwaita `.entry` fg | L:`#2e3436` D:`#ffffff` | OK |
| `border` | <- `defaults.border` | Not in preset; resolved | OK |
| `placeholder` | libadwaita `.dim-label` | L:`#929595` D:`#8b8b8b` | OK -- dim-label values |
| `caret` | libadwaita `@accent_color` | Not in preset; safety net resolves from `defaults.foreground` | **WRONG** -- should be accent (#3584e4), not foreground |
| `selection` | <- `defaults.selection` | Not in preset; resolved | OK |
| `selection_foreground` | <- `defaults.selection_foreground` | Not in preset; resolved | OK |
| `font` | <- `defaults.font` | Not in preset; resolved | OK |
| `min_height` | **(Adwaita CSS)**: 34 | `34.0` | OK |
| `padding_horizontal` | **(Adwaita CSS)**: 9 | `9.0` | OK |
| `padding_vertical` | **(Adwaita CSS)**: 0 | `0.0` | OK |
| `radius` | <- `defaults.radius` | Not in preset; resolved (9) | OK |
| `border_width` | <- `defaults.frame_width` | Not in preset; resolved (1) | OK |

---

### 2.5 Checkbox / Radio Button

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `indicator_size` | libadwaita CSS: 14 | `20.0` | **WRONG** -- should be 14, not 20 (20 includes padding) |
| `spacing` | **(Adwaita CSS)**: 8 | `8.0` | OK |
| `radius` | <- `defaults.radius` | Not in preset; resolved (9) | OK |
| `border_width` | <- `defaults.frame_width` | Not in preset; resolved (1) | OK |
| `checked_bg` | <- `defaults.accent` | Not in preset; resolved | OK |

---

### 2.6 Menu

| Property | platform-facts GNOME | Preset L / D | Status |
|----------|---------------------|--------------|--------|
| `background` | libadwaita `popover.menu` bg | Not in preset; resolved from `defaults.background` | OK -- popover.menu bg matches background |
| `foreground` | libadwaita `popover.menu` fg | Not in preset; resolved from `defaults.foreground` | OK |
| `separator` | **(Adwaita CSS)** separator | Not in preset; resolved from `defaults.border` | OK |
| `font.family` | <- `defaults.font` | Not in preset; resolved | OK |
| `font.size` | <- `defaults.font` | Not in preset; resolved | OK |
| `font.weight` | <- `defaults.font` | Not in preset; resolved | OK |
| `item_height` | **(Adwaita CSS)**: 32 | `32.0` | OK |
| `padding_horizontal` | **(Adwaita CSS)**: 12 ($menu_padding) | `12.0` | OK |
| `padding_vertical` | **(Adwaita CSS)**: 0 (vertical space from min-height) | `4.0` | **WRONG** -- should be 0, not 4 |
| `icon_spacing` | **(Adwaita CSS)**: 8 | `8.0` | OK |

---

### 2.7 Tooltip

| Property | platform-facts GNOME | Preset L / D | Status |
|----------|---------------------|--------------|--------|
| `background` | libadwaita `.tooltip` bg | L:`#2e3436` D:`#3a3a3e` | OK -- explicitly set |
| `foreground` | libadwaita `.tooltip` fg | L:`#ffffff` D:`#ffffff` | OK |
| `font.family` | <- `defaults.font` | Not in preset; resolved | OK |
| `font.size` | <- `defaults.font` | Not in preset; resolved | OK |
| `font.weight` | <- `defaults.font` | Not in preset; resolved | OK |
| `padding_horizontal` | **(Adwaita CSS)**: 10 | `10.0` | OK |
| `padding_vertical` | **(Adwaita CSS)**: 6 | `6.0` | OK |
| `max_width` | `(none)` -- preset: 360 | `300.0` | **WRONG** -- platform-facts says GNOME preset should be 360 |
| `radius` | <- `defaults.radius` | Not in preset; resolved (9) | OK |

---

### 2.8 Scrollbar

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `track` | **(Adwaita CSS)** scrollbar | Not in preset; safety net resolves from `defaults.background` | OK |
| `thumb` | **(Adwaita CSS)** scrollbar | Not in preset; resolved from `defaults.muted` | OK -- reasonable fallback |
| `thumb_hover` | **(Adwaita CSS)** :hover | Not in preset; resolved from `defaults.muted` | OK -- same as thumb (could be distinct) |
| `width` | slider: 8 + margins | `12.0` | OK -- 8px slider + 4px margins |
| `min_thumb_height` | **(Adwaita CSS)**: 40 | `30.0` | **WRONG** -- should be 40, not 30 |
| `slider_width` | **(Adwaita CSS)**: 8 | `8.0` | OK |
| `overlay_mode` | gsettings `overlay-scrolling` | `false` | OK -- preset default; live reader overrides |

---

### 2.9 Slider

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `fill` | <- `defaults.accent` | Not in preset; resolved | OK |
| `track` | <- `defaults.muted` | Not in preset; resolved | OK |
| `thumb` | <- `defaults.surface` | Not in preset; resolved | OK |
| `track_height` | libadwaita `.scale`: 10 | `10.0` | OK |
| `thumb_size` | libadwaita: 20 | `20.0` | OK |
| `tick_length` | `(none)` -- no ticks | `4.0` | OK -- reasonable preset default |

---

### 2.10 Progress Bar

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `fill` | <- `defaults.accent` | Not in preset; resolved | OK |
| `track` | <- `defaults.muted` | Not in preset; resolved | OK |
| `height` | libadwaita `.progressbar`: 8 | `8.0` | OK |
| `min_width` | **(Adwaita CSS)**: 80 | `80.0` | OK |
| `radius` | <- `defaults.radius` | Not in preset; resolved (9) | OK |

---

### 2.11 Tab Bar

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `background` | <- `defaults.background` | Not in preset; resolved | OK |
| `foreground` | <- `defaults.foreground` | Not in preset; resolved | OK |
| `active_background` | <- `defaults.background` | Not in preset; resolved | OK |
| `active_foreground` | <- `defaults.foreground` | Not in preset; resolved | OK |
| `bar_background` | <- `defaults.background` | Not in preset; resolved | OK |
| `min_width` | **(Adwaita CSS)**: none | `64.0` | MISMATCH -- GNOME has no min-width (sizes to label); 64 is a reasonable preset |
| `min_height` | **(Adwaita CSS)**: 30 | `30.0` | OK |
| `padding_horizontal` | **(Adwaita CSS)**: 12 | `12.0` | OK |
| `padding_vertical` | CSS `padding: 3px 12px` but measured (30-14)/2 = 8 | `4.0` | **WRONG** -- CSS says 3, platform-facts measured 8; preset has 4 which matches neither |

---

### 2.12 Sidebar

| Property | platform-facts GNOME | Preset L / D | Status |
|----------|---------------------|--------------|--------|
| `background` | libadwaita `.sidebar` bg | L:`#ebebed` D:`#2e2e32` | OK -- explicitly set |
| `foreground` | libadwaita `.sidebar` fg | L:`#2e3436` D:`#ffffff` | OK |

---

### 2.13 Toolbar

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `font.family` | <- `defaults.font` | Not in preset; resolved | OK |
| `font.size` | <- `defaults.font` | Not in preset; resolved | OK |
| `font.weight` | <- `defaults.font` | Not in preset; resolved | OK |
| `height` | **(Adwaita CSS)**: 47 | `47.0` | OK |
| `item_spacing` | **(Adwaita CSS)**: 6 | `6.0` | OK |
| `padding` | **(Adwaita CSS)**: 6 | `6.0` | OK |

---

### 2.14 Status Bar

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `font.family` | <- `defaults.font` | Not in preset; resolved | OK |
| `font.size` | <- `defaults.font` | Not in preset; resolved | OK |
| `font.weight` | <- `defaults.font` | Not in preset; resolved | OK |

---

### 2.15 List / Table

| Property | platform-facts GNOME | Preset L / D | Status |
|----------|---------------------|--------------|--------|
| `background` | libadwaita `.list` bg | Not in preset; safety net resolves from `defaults.background` | OK |
| `foreground` | libadwaita `.list` fg | Not in preset; resolved from `defaults.foreground` | OK |
| `alternate_row` | **(Adwaita CSS)** even row | L:`#f5f5f5` D:`#2a2a2e` | OK -- explicitly set |
| `selection` | <- `defaults.selection` | Not in preset; resolved | OK |
| `selection_foreground` | <- `defaults.selection_foreground` | Not in preset; resolved | OK |
| `header_background` | **(Adwaita CSS)** columnview header | Not in preset; resolved from `defaults.surface` | OK |
| `header_foreground` | **(Adwaita CSS)** columnview header | Not in preset; resolved from `defaults.foreground` | OK |
| `grid_color` | `(none)` -- columnview uses CSS separator | Not in preset; resolved from `defaults.border` | OK |
| `item_height` | **(Adwaita CSS)**: `.rich-list` 32px; plain row content-driven | `34.0` | OK -- 34 accounts for padding in rich-list rows |
| `padding_horizontal` | **(Adwaita CSS)**: 12 (`.rich-list`) | `12.0` | OK |
| `padding_vertical` | **(Adwaita CSS)**: 8 (`.rich-list`) | `8.0` | OK |

---

### 2.16 Popover / Dropdown

| Property | platform-facts GNOME | Preset L / D | Status |
|----------|---------------------|--------------|--------|
| `background` | libadwaita `.popover` bg | L:`#ffffff` D:`#36363a` | OK -- explicitly set |
| `foreground` | libadwaita `.popover` fg | L:`#2e3436` D:`#ffffff` | OK |
| `border` | <- `defaults.border` | Not in preset; resolved | OK |
| `radius` | <- `defaults.radius_lg` | Not in preset; resolved (15) | OK |

---

### 2.17 Splitter

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `width` | **(Adwaita CSS)** paned: 1 (default) / 5 (wide) | `1.0` | OK -- default paned width |

---

### 2.18 Separator

| Property | platform-facts GNOME | Preset L / D | Status |
|----------|---------------------|--------------|--------|
| `color` | libadwaita `.separator` CSS | L:`#d5d5d5` D:`#4a4a4e` | OK -- matches border color |

---

### 2.19 Text Scale

| Role | platform-facts GNOME | Preset value | Status |
|------|---------------------|--------------|--------|
| `caption` | `.caption`: ~9pt, 400 | Not in preset; resolved from `defaults.font` (11pt, 400) | **WRONG** -- should be ~9pt (82% of 11 = 9.02), not 11pt |
| `section_heading` | `.heading`: 11pt, **700** | weight: `700` (explicit) | OK -- weight 700 explicitly set; size inherited as 11pt |
| `dialog_title` | `.title-2`: ~15pt, **800** | Not in preset; resolved from `defaults.font` (11pt, 400) | **WRONG** -- should be ~15pt (136% of 11) weight 800 |
| `display` | `.title-1`: ~20pt, **800** | Not in preset; resolved from `defaults.font` (11pt, 400) | **WRONG** -- should be ~20pt (181% of 11) weight 800 |

---

### 2.20 Layout Spacing

| Context | platform-facts GNOME | Preset mapping | Status |
|---------|---------------------|----------------|--------|
| Standard widget gap | 6 **(measured)** | `spacing.s = 6.0` | OK |
| Nested container margin | 12 **(measured)** | `spacing.m = 12.0` | OK |
| Window content margin | 12 **(measured)** | `spacing.m = 12.0` | OK |
| Section gap | 18 **(measured)** | `spacing.l = 18.0` | OK |

---

### 2.21 Switch / Toggle

| Property | platform-facts GNOME | Preset L / D | Status |
|----------|---------------------|--------------|--------|
| `track_width` | ~46px (derived: 2*thumb+pad) | `46.0` | OK |
| `track_height` | ~26px (20+2*3 padding) | `26.0` | OK |
| `thumb_size` | 20px | `20.0` | OK |
| `track_radius` | 14px (pill) | `14.0` | OK |
| `checked_bg` | <- `defaults.accent` | Not in preset; resolved | OK |
| `unchecked_bg` | Adwaita `$trough_color` | L:`#d5d5d5` D:`#4a4a4e` | OK |
| `thumb_bg` | Adwaita `$slider_color` | L:`#ffffff` D:`#ffffff` | OK |

---

### 2.22 Dialog

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `min_width` | AdwAlertDialog: 300sp | `320.0` | **WRONG** -- should be 300, not 320 |
| `max_width` | AdwAlertDialog: 372sp (wide: 600sp) | `560.0` | **WRONG** -- should be 372, not 560 |
| `min_height` | `(none)` | `140.0` | OK -- no GNOME value; reasonable preset |
| `max_height` | `(none)` | `600.0` | OK -- aligns with wide mode max |
| `content_padding` | 24px sides, 32px top | `24.0` | OK -- sides padding (top differs) |
| `button_spacing` | 12px | `8.0` | **WRONG** -- should be 12, not 8 |
| `button_order` | cancel left, affirmative right | `"trailing_affirmative"` | OK |
| `title_font` | `.title-2` (136%, 800) | Not in preset; resolved from `defaults.font` (11pt, 400) | **WRONG** -- should be ~15pt weight 800 |
| `radius` | 18px (`$alert_radius`) | Not in preset; resolved from `defaults.radius_lg` (15) | **WRONG** -- should be 18, not 15 |
| `icon_size` | `(none)` -- no default icon | `32.0` | OK -- reasonable preset |

---

### 2.23 Spinner / Progress Ring

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `diameter` | GtkSpinner: 16 | `16.0` | OK |
| `min_size` | `(none)` | `16.0` | OK -- reasonable preset |
| `stroke_width` | `(none)` -- icon-based | `2.0` | OK -- reasonable preset |
| `fill` | <- `defaults.foreground` | Not in preset; safety net resolves from `defaults.foreground` | OK |

---

### 2.24 ComboBox / Dropdown Trigger

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `min_height` | <- button min-height (24+pad) | `34.0` | OK -- matches button total |
| `min_width` | `(none)` -- sizes to content | `120.0` | OK -- reasonable preset |
| `padding_horizontal` | <- button padding (10px) | `12.0` | MISMATCH -- platform-facts says 10 (like button); preset has 12 |
| `arrow_size` | 16px (pan-down-symbolic) | `12.0` | **WRONG** -- should be 16, not 12 |
| `arrow_area_width` | `(none)` -- inline icon | `28.0` | OK -- reasonable preset |
| `radius` | <- `defaults.radius` | Not in preset; resolved (9) | OK |

---

### 2.25 Segmented Control

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `segment_height` | `(none)` | `28.0` | OK -- no GNOME value; reasonable preset |
| `separator_width` | `(none)` | `1.0` | OK |
| `padding_horizontal` | `(none)` | `12.0` | OK |
| `radius` | `(none)` | Not in preset; resolved from `defaults.radius` (9) | OK |

---

### 2.26 Card / Container

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `background` | `var(--card-bg-color)` | Not in preset; resolved from `defaults.surface` | OK |
| `border` | `var(--card-shade-color)` | Not in preset; resolved from `defaults.border` | OK |
| `radius` | `$card_radius` = 12px | Not in preset; resolved from `defaults.radius_lg` (15) | **WRONG** -- should be 12, not 15 |
| `shadow` | Adwaita box-shadow | Not in preset; resolved from `defaults.shadow_enabled` | OK |
| `padding` | `(none)` -- app-defined | `16.0` | OK -- reasonable preset |

---

### 2.27 Expander / Disclosure

| Property | platform-facts GNOME | Preset value | Status |
|----------|---------------------|--------------|--------|
| `header_height` | AdwExpanderRow: 50 | `40.0` | **WRONG** -- should be 50, not 40 |
| `arrow_size` | 16px (pan-end-symbolic) | `12.0` | **WRONG** -- should be 16, not 12 |
| `content_padding` | **(Adwaita CSS)** row padding | `16.0` | OK -- reasonable mapping |
| `radius` | 6px (expander title) | Not in preset; resolved from `defaults.radius` (9) | **WRONG** -- should be 6, not 9 |

---

### 2.28 Link

| Property | platform-facts GNOME | Preset L / D | Status |
|----------|---------------------|--------------|--------|
| `color` | `var(--accent-color)` | Not in preset; resolved from `defaults.link` | OK |
| `visited` | Adwaita 80% mix accent+fg | Not in preset; resolved from `defaults.link` (same as color) | OK -- visited = link as fallback |
| `underline` | yes | `true` | OK |
| `background` | `(none)` -- inline | L:`#00000000` D:`#00000000` | OK -- transparent |
| `hover_bg` | `(none)` | L:`#3584e418` D:`#3584e418` | OK -- subtle accent tint |

---

### adwaita-live.toml Cross-check

The live preset correctly mirrors all geometry values from the static
preset and omits all colors, fonts, and icon settings (these come from
the OS reader at runtime). Verified: all 28 widget sections match.

---

### Summary of Issues Found

#### Critical Issues (values contradict platform-facts)

| # | Property | Preset has | Should be | Section | Fix |
|---|----------|-----------|-----------|---------|-----|
| 1 | `button.icon_spacing` | 6 | 8 | 2.3 | Change to `8.0` -- platform-facts: **(Adwaita CSS)**: 8 |
| 2 | `checkbox.indicator_size` | 20 | 14 | 2.5 | Change to `14.0` -- platform-facts: "libadwaita CSS: 14"; 20 includes padding |
| 3 | `menu.padding_vertical` | 4 | 0 | 2.6 | Change to `0.0` -- CSS `padding: 0 $menu_padding`; height from min-height |
| 4 | `tooltip.max_width` | 300 | 360 | 2.7 | Change to `360.0` -- platform-facts GNOME column says 360 |
| 5 | `scrollbar.min_thumb_height` | 30 | 40 | 2.8 | Change to `40.0` -- **(Adwaita CSS)**: 40 |
| 6 | `tab.padding_vertical` | 4 | 3 | 2.11 | Change to `3.0` -- CSS `padding: 3px 12px` from _notebook.scss |
| 7 | `dialog.min_width` | 320 | 300 | 2.22 | Change to `300.0` -- AdwAlertDialog: 300sp |
| 8 | `dialog.max_width` | 560 | 372 | 2.22 | Change to `372.0` -- AdwAlertDialog: 372sp |
| 9 | `dialog.button_spacing` | 8 | 12 | 2.22 | Change to `12.0` -- _message-dialog.scss `.response-area { border-spacing: 12px }` |
| 10 | `dialog.radius` | 15 (inherited) | 18 | 2.22 | Set explicitly to `18.0` -- `$alert_radius: 18px`, distinct from window radius |
| 11 | `card.radius` | 15 (inherited) | 12 | 2.26 | Set explicitly to `12.0` -- `$card_radius = 12px` |
| 12 | `expander.header_height` | 40 | 50 | 2.27 | Change to `50.0` -- AdwExpanderRow min-height: 50px |
| 13 | `expander.arrow_size` | 12 | 16 | 2.27 | Change to `16.0` -- _expanders.scss `min-width/min-height: 16px` |
| 14 | `expander.radius` | 9 (inherited) | 6 | 2.27 | Set explicitly to `6.0` -- 6px expander title radius |
| 15 | `combo_box.arrow_size` | 12 | 16 | 2.24 | Change to `16.0` -- GtkDropDown arrow: 16x16 pan-down-symbolic |
| 16 | `input.caret` | foreground (safety net) | accent | 2.4 | Set explicitly to accent (`#3584e4`) -- libadwaita `@accent_color` |

#### Semantic Issues (status foreground interpretation)

| # | Property | Preset has | platform-facts says | Section | Note |
|---|----------|-----------|-------------------|---------|------|
| 17 | `danger_foreground` | L:`#ffffff` D:`#ffffff` | footnote 1: normal body fg | 2.1.4 | Light should be `#2e3436` (body fg); dark `#ffffff` is correct (matches body fg) |
| 18 | `success_foreground` | L:`#ffffff` D:`#ffffff` | footnote 1: normal body fg | 2.1.4 | Light should be `#2e3436`; dark `#ffffff` correct |
| 19 | `info_foreground` | L:`#ffffff` D:`#ffffff` | footnote 1: normal body fg | 2.1.4 | Light should be `#2e3436`; dark `#ffffff` correct |

**Decision needed on 17-19:** platform-facts footnote 1 says GNOME
`*_foreground` should be **normal body foreground** (text color
*alongside* a status indicator). The current white values work as
**contrast text on a status-colored fill**. This is a semantic
interpretation choice -- if the intent is "text next to a red icon"
then foreground is correct; if the intent is "white text on red
background" then current values are correct. The current values are
internally inconsistent: `warning_foreground` light uses `#2e3436`
(body fg) but `danger_foreground` light uses `#ffffff`. Pick one
interpretation and be consistent.

#### Text Scale Issues

| # | Property | Preset has | Should be | Section | Fix |
|---|----------|-----------|-----------|---------|-----|
| 20 | `text_scale.caption.size` | 11 (inherited) | ~9 (82% of 11) | 2.19 | Add explicit `size = 9.0` to caption entry |
| 21 | `text_scale.dialog_title.size` | 11 (inherited) | ~15 (136% of 11) | 2.19 | Add explicit entry: `size = 15.0, weight = 800` |
| 22 | `text_scale.dialog_title.weight` | 400 (inherited) | 800 | 2.19 | (part of fix 21) |
| 23 | `text_scale.display.size` | 11 (inherited) | ~20 (181% of 11) | 2.19 | Add explicit entry: `size = 20.0, weight = 800` |
| 24 | `text_scale.display.weight` | 400 (inherited) | 800 | 2.19 | (part of fix 23) |

#### Window Font Weight

| # | Property | Preset has | Should be | Section | Fix |
|---|----------|-----------|-----------|---------|-----|
| 25 | `window.title_bar_font.weight` | 400 (inherited) | 700 (Bold) | 2.2 | Add `[light/dark.window.title_bar_font]` with `weight = 700` |

#### Minor Discrepancies (preset vs platform-facts convention)

| # | Property | Preset has | platform-facts says | Status |
|---|----------|-----------|-------------------|--------|
| 26 | `button.min_width` | 64 | GNOME: none | Preset provides a minimum; GNOME sizes to content -- acceptable as a layout hint |
| 27 | `tab.min_width` | 64 | GNOME: none | Same reasoning as button |
| 28 | `combo_box.padding_horizontal` | 12 | 10 (button-like) | Minor difference; 12 is GtkDropDown `border-spacing: 6px` applied differently |
| 29 | `icon_sizes.dialog` | 22 | 48 (GTK3 legacy) or no GNOME value | No clear GNOME 4 value; 22 is reasonable |

**Total: 25 actionable issues (16 critical, 5 text scale, 1 font weight,
3 semantic choices), 4 minor acceptable discrepancies.**

---

## 29. Systematic Windows 11 Preset Verification (parameter-by-parameter)

Cross-references every property in `platform-facts.md` Chapter 2 (sections
2.1--2.28) against `windows-11.toml` (both light and dark variants) and
`windows-11-live.toml`. Properties marked with `<- defaults.X` are verified
through `resolve.rs` inheritance. Values marked **(Fluent)** are checked
against the Fluent 2 / WinUI3 XAML resources documented in Chapter 1.

Legend: OK = matches, ERR = mismatch, WARN = minor concern, INHERIT =
filled by resolve.rs.

### 2.1 Global Defaults

#### 2.1.1 Base Font

| Property | platform-facts Windows | Preset L / D | Live | Status |
|----------|----------------------|--------------|------|--------|
| `family` | `lfMessageFont.lfFaceName`; Segoe UI | `Segoe UI` / `Segoe UI` | OS | OK |
| `size` | Body=14px | 14.0 / 14.0 | OS | OK |
| `weight` | `lfMessageFont.lfWeight`; 400 | 400 / 400 | OS | OK |
| `line_height` | 1.43 **(Fluent)** 20/14 | 1.43 / 1.43 | 1.43 | OK |

#### 2.1.2 Monospace Font

| Property | platform-facts Windows | Preset L / D | Live | Status |
|----------|----------------------|--------------|------|--------|
| `family` | **(none)** -- Cascadia Mono | `Cascadia Mono` / same | none | OK |
| `size` | **(none)** -- 14px | 14.0 / 14.0 | none | OK |
| `weight` | **(none)** -- 400 | 400 / 400 | none | OK |

#### 2.1.3 Base Colors

| Property | platform-facts Windows | Preset L / D | Status |
|----------|----------------------|--------------|--------|
| `background` | `UISettings(Background)` | `#f3f3f3` / `#202020` | OK |
| `foreground` | `UISettings(Foreground)` | `#1a1a1a` / `#ffffff` | OK |
| `accent` | `UISettings(Accent)` | `#0078d4` / `#0078d4` | OK |
| `accent_foreground` | TextOnAccentFillColorPrimary L=#fff D=#000 | `#ffffff` / `#ffffff` | **ERR** dark=#ffffff should be #000000 (18a) |
| `surface` | CardBackgroundFillColorDefault | `#ffffff` / `#2d2d2d` | OK |
| `border` | CardStrokeColorDefault | `#e5e5e5` / `#454545` | OK |
| `muted` | TextFillColorSecondary | `#717171` / `#9a9a9a` | OK |
| `shadow` | Fluent elevation layers | `#00000018` / `#00000040` | OK |
| `link` | HyperlinkForeground | `#005a9e` / `#60cdff` | OK |
| `selection` | `COLOR_HIGHLIGHT` | `#0078d4` / `#0078d4` | OK |
| `selection_foreground` | `COLOR_HIGHLIGHTTEXT` | `#ffffff` / `#ffffff` | OK |
| `selection_inactive` | resolve: <- selection | (not set) | OK -- INHERIT |
| `disabled_foreground` | TextFillColorDisabled | `#c5c5c5` / `#454545` | OK |

#### 2.1.4 Status Colors

| Property | platform-facts Windows | Preset L / D | Status |
|----------|----------------------|--------------|--------|
| `danger` | L=#c42b1c D=#ff99a4 | `#c42b1c` / `#ff9999` | **ERR** dark=#ff9999 vs #ff99a4 (18b) |
| `danger_foreground` | L=#ffffff D=#1a1a1a | `#ffffff` / `#1a1a1a` | OK |
| `warning` | L=#9d5d00 D=#fce100 | `#9d5d00` / `#f0c239` | **ERR** dark=#f0c239 vs #fce100 (18c) |
| `warning_foreground` | L=#1a1a1a D=#1a1a1a | `#1a1a1a` / `#1a1a1a` | OK |
| `success` | L=#0f7b0f D=#6ccb5f | `#0f7b0f` / `#6ccb5f` | OK |
| `success_foreground` | L=#ffffff D=#1a1a1a | `#ffffff` / `#1a1a1a` | OK |
| `info` | accent-derived | `#0078d4` / `#60cdff` | OK |
| `info_foreground` | L=#ffffff D=#1a1a1a | `#ffffff` / `#1a1a1a` | OK |

#### 2.1.5 Focus Ring

| Property | platform-facts Windows | Preset L/D | Live | Status |
|----------|----------------------|------------|------|--------|
| `color` | `UISettings(Accent)` | `#0078d4` / `#0078d4` | derived | OK |
| `width` | Fluent: 2px | 2.0 / 2.0 | 2.0 | OK |
| `offset` | Fluent: 0px | 0.0 / 0.0 | 0.0 | OK |

#### 2.1.6 Global Geometry

| Property | platform-facts Windows | Preset L/D | Live | Status |
|----------|----------------------|------------|------|--------|
| `radius` | Fluent: 4px | 4.0 / 4.0 | 4.0 | OK |
| `radius_lg` | Fluent: 8px | 8.0 / 8.0 | 8.0 | OK |
| `frame_width` | `SM_CXBORDER` = 1px | 1.0 / 1.0 | 1.0 | OK |
| `disabled_opacity` | Fluent: ~0.3 | 0.3 / 0.3 | 0.3 | OK |
| `border_opacity` | 0.14 **(preset)** | 0.14 / 0.14 | 0.14 | OK |
| `shadow_enabled` | yes | true / true | true | OK |

#### 2.1.7 Accessibility

| Property | platform-facts Windows | Preset L/D | Live | Status |
|----------|----------------------|------------|------|--------|
| `text_scaling_factor` | `UISettings.TextScaleFactor` | 1.0 / 1.0 | OS | OK |
| `reduce_motion` | `SPI_GETCLIENTAREAANIMATION` | false / false | OS | OK |
| `high_contrast` | `SPI_GETHIGHCONTRAST` | false / false | OS | OK |
| `reduce_transparency` | **(none)** | false / false | N/A | OK |

#### 2.1.8 Icon Sizes

| Context | platform-facts Windows | Preset L/D | Live | Status |
|---------|----------------------|------------|------|--------|
| `toolbar` | AppBarButton: 20 | 20.0 / 20.0 | 20.0 | OK |
| `small` | `SM_CXSMICON`: 16 | 16.0 / 16.0 | 16.0 | OK |
| `large` | `SM_CXICON`: 32 | 32.0 / 32.0 | 32.0 | OK |
| `dialog` | **(none)** | 22.0 / 22.0 | 22.0 | WARN -- no spec; 22 reasonable |
| `panel` | **(none)** | 20.0 / 20.0 | 20.0 | WARN -- no spec; matches toolbar |

### 2.2 Window / Application Chrome

| Property | platform-facts Windows | Preset L/D | Status |
|----------|----------------------|------------|--------|
| `background` | <- defaults.background | (not set) | OK -- INHERIT |
| `foreground` | <- defaults.foreground | (not set) | OK -- INHERIT |
| `border` | <- defaults.border | (not set) | OK -- INHERIT |
| `title_bar_background` | `DwmGetColorizationColor` (live) | (not set) | OK -- INHERIT <- surface |
| `title_bar_foreground` | `COLOR_CAPTIONTEXT` (live) | (not set) | OK -- INHERIT <- foreground |
| `inactive_title_bar_bg` | `COLOR_INACTIVECAPTION` (live) | (not set) | OK -- INHERIT <- title_bar_bg |
| `inactive_title_bar_fg` | `COLOR_INACTIVECAPTIONTEXT` (live) | (not set) | OK -- INHERIT <- title_bar_fg |
| `title_bar_font.family` | `lfCaptionFont.lfFaceName` (live) | (not set) | OK -- INHERIT <- defaults.font |
| `title_bar_font.size` | `lfCaptionFont.lfHeight` (live) | (not set) | OK -- INHERIT |
| `title_bar_font.weight` | `lfCaptionFont.lfWeight` (live) | (not set) | OK -- INHERIT |
| `radius` | <- defaults.radius_lg | (not set) | OK -- INHERIT |
| `shadow` | <- defaults.shadow_enabled | (not set) | OK -- INHERIT |

### 2.3 Button

| Property | platform-facts Windows | Preset L / D | Live | Status |
|----------|----------------------|--------------|------|--------|
| `background` | `COLOR_BTNFACE` (live) | `#fdfdfd` / `#333333` | none | OK |
| `foreground` | `COLOR_BTNTEXT` (live) | `#1a1a1a` / `#ffffff` | none | OK |
| `border` | <- defaults.border | (not set) | N/A | OK -- INHERIT |
| `font` | <- defaults.font | (not set) | N/A | OK -- INHERIT |
| `min_width` | **(none)** | 64.0 / 64.0 | 64.0 | WARN -- no spec; design default |
| `min_height` | WinUI3: ~27 | 27.0 / 27.0 | 27.0 | WARN -- effective; issue 8c |
| `padding_horizontal` | WinUI3: 11 | 11.0 / 11.0 | 11.0 | OK |
| `padding_vertical` | WinUI3: 5/6 | 6.0 / 6.0 | 6.0 | WARN -- asymmetric; 6 acceptable |
| `radius` | <- defaults.radius | (not set) | N/A | OK -- INHERIT |
| `icon_spacing` | WinUI3: 8 | 8.0 / 8.0 | 8.0 | OK |
| `primary_bg` | <- defaults.accent | (not set) | N/A | OK -- INHERIT |
| `primary_fg` | <- defaults.accent_foreground | (not set) | N/A | OK -- INHERIT |
| `disabled_opacity` | <- defaults.disabled_opacity | (not set) | N/A | OK -- INHERIT |
| `shadow` | <- defaults.shadow_enabled | (not set) | N/A | OK -- INHERIT |

### 2.4 Text Input

| Property | platform-facts Windows | Preset L / D | Live | Status |
|----------|----------------------|--------------|------|--------|
| `background` | `COLOR_WINDOW` (live) | `#ffffff` / `#2d2d2d` | none | OK |
| `foreground` | `COLOR_WINDOWTEXT` (live) | `#1a1a1a` / `#ffffff` | none | OK |
| `border` | <- defaults.border | (not set) | N/A | OK -- INHERIT |
| `placeholder` | TextPlaceholderColor | (not set) | N/A | OK -- INHERIT <- muted |
| `caret` | foreground (default) | (not set) | N/A | OK -- safety net <- foreground |
| `selection` | <- defaults.selection | (not set) | N/A | OK -- INHERIT |
| `selection_foreground` | <- defaults.selection_foreground | (not set) | N/A | OK -- INHERIT |
| `font` | <- defaults.font | (not set) | N/A | OK -- INHERIT |
| `min_height` | WinUI3 TextBox: 32 | 32.0 / 32.0 | 32.0 | OK |
| `padding_horizontal` | WinUI3: 10 left / 6 right | 10.0 / 10.0 | 10.0 | OK |
| `padding_vertical` | WinUI3: 5 | 5.0 / 5.0 | 5.0 | OK |
| `radius` | <- defaults.radius | (not set) | N/A | OK -- INHERIT |
| `border_width` | <- defaults.frame_width | (not set) | N/A | OK -- INHERIT |

### 2.5 Checkbox / Radio Button

| Property | platform-facts Windows | Preset L/D | Live | Status |
|----------|----------------------|------------|------|--------|
| `indicator_size` | WinUI3: 20 | 20.0 / 20.0 | 20.0 | OK |
| `spacing` | WinUI3: 8 | 8.0 / 8.0 | 8.0 | OK |
| `radius` | <- defaults.radius | (not set) | N/A | OK -- INHERIT |
| `border_width` | <- defaults.frame_width | (not set) | N/A | OK -- INHERIT |
| `checked_bg` | <- defaults.accent | (not set) | N/A | OK -- INHERIT |

### 2.6 Menu

| Property | platform-facts Windows | Preset L/D | Live | Status |
|----------|----------------------|------------|------|--------|
| `background` | `COLOR_MENU` (live) | (not set) | N/A | OK -- INHERIT <- background |
| `foreground` | `COLOR_MENUTEXT` (live) | (not set) | N/A | OK -- INHERIT <- foreground |
| `separator` | <- defaults.border | (not set) | N/A | OK -- INHERIT |
| `font.family` | `lfMenuFont.lfFaceName` (live) | (not set) | N/A | OK -- INHERIT |
| `font.size` | `lfMenuFont.lfHeight` (live) | (not set) | N/A | OK -- INHERIT |
| `font.weight` | `lfMenuFont.lfWeight` (live) | (not set) | N/A | OK -- INHERIT |
| `item_height` | WinUI3: ~23px (mouse) | 23.0 / 23.0 | 23.0 | WARN -- touch=31; issue 8d |
| `padding_horizontal` | WinUI3: 11 | 12.0 / 12.0 | 12.0 | WARN -- 11 vs 12 |
| `padding_vertical` | WinUI3: 8 | 8.0 / 8.0 | 8.0 | OK |
| `icon_spacing` | WinUI3: 12 | 8.0 / 8.0 | 8.0 | **ERR** -- should be 12 (1l) |

### 2.7 Tooltip

| Property | platform-facts Windows | Preset L/D | Live | Status |
|----------|----------------------|------------|------|--------|
| `background` | `COLOR_INFOBK` (live) | `#2c2c2c` / `#3a3a3a` | none | OK |
| `foreground` | `COLOR_INFOTEXT` (live) | `#ffffff` / `#ffffff` | none | OK |
| `font.family` | <- defaults.font | (not set) | N/A | OK -- INHERIT |
| `font.size` | <- defaults.font | (not set) | N/A | OK -- INHERIT |
| `font.weight` | <- defaults.font | (not set) | N/A | OK -- INHERIT |
| `padding_horizontal` | WinUI3: 9 | 8.0 / 8.0 | 8.0 | **ERR** -- should be 9 (1e) |
| `padding_vertical` | WinUI3: 6-8 | 8.0 / 8.0 | 8.0 | WARN -- upper bound |
| `max_width` | WinUI3: 320 | 320.0 / 320.0 | 320.0 | OK |
| `radius` | <- defaults.radius | (not set) | N/A | OK -- INHERIT |

### 2.8 Scrollbar

| Property | platform-facts Windows | Preset L/D | Live | Status |
|----------|----------------------|------------|------|--------|
| `track` | transparent | (not set) | N/A | OK -- safety net <- background |
| `thumb` | #c2c2c2 **(measured)** | (not set) | N/A | WARN -- inherits muted; not #c2c2c2 |
| `thumb_hover` | #a0a0a0 **(measured)** | (not set) | N/A | WARN -- inherits muted |
| `width` | `SM_CXVSCROLL` = 17px | 17.0 / 17.0 | 17.0 | OK |
| `min_thumb_height` | `SM_CYVTHUMB` = 17px | 17.0 / 17.0 | 17.0 | OK |
| `slider_width` | `SM_CXVSCROLL` = 17px | 6.0 / 6.0 | 6.0 | WARN -- modern thin style |
| `overlay_mode` | always persistent | false / false | false | OK |

### 2.9 Slider

| Property | platform-facts Windows | Preset L/D | Live | Status |
|----------|----------------------|------------|------|--------|
| `fill` | <- defaults.accent | (not set) | N/A | OK -- INHERIT |
| `track` | <- defaults.muted | (not set) | N/A | OK -- INHERIT |
| `thumb` | <- defaults.surface | (not set) | N/A | OK -- INHERIT |
| `track_height` | WinUI3: 4 | 4.0 / 4.0 | 4.0 | OK |
| `thumb_size` | WinUI3: 18 | 18.0 / 18.0 | 18.0 | OK |
| `tick_length` | WinUI3: 4 | 4.0 / 4.0 | 4.0 | OK |

### 2.10 Progress Bar

| Property | platform-facts Windows | Preset L/D | Live | Status |
|----------|----------------------|------------|------|--------|
| `fill` | <- defaults.accent | (not set) | N/A | OK -- INHERIT |
| `track` | <- defaults.muted | (not set) | N/A | OK -- INHERIT |
| `height` | WinUI3: min 3 | 3.0 / 3.0 | 3.0 | OK |
| `min_width` | **(none)** | 100.0 / 100.0 | 100.0 | WARN -- no spec |
| `radius` | <- defaults.radius | (not set) | N/A | OK -- INHERIT |

### 2.11 Tab Bar

| Property | platform-facts Windows | Preset L/D | Live | Status |
|----------|----------------------|------------|------|--------|
| `background` | <- defaults.background | (not set) | N/A | OK -- INHERIT |
| `foreground` | <- defaults.foreground | (not set) | N/A | OK -- INHERIT |
| `active_background` | <- defaults.background | (not set) | N/A | OK -- INHERIT |
| `active_foreground` | <- defaults.foreground | (not set) | N/A | OK -- INHERIT |
| `bar_background` | <- defaults.background | (not set) | N/A | OK -- INHERIT |
| `min_width` | **(none)** | 64.0 / 64.0 | 64.0 | WARN -- no spec |
| `min_height` | WinUI3: 32 | 32.0 / 32.0 | 32.0 | OK |
| `padding_horizontal` | WinUI3: 8 | 12.0 / 12.0 | 12.0 | WARN -- 8 vs 12 |
| `padding_vertical` | WinUI3: 3 | 4.0 / 4.0 | 4.0 | WARN -- 3 vs 4 |

### 2.12 Sidebar

| Property | platform-facts Windows | Preset L / D | Status |
|----------|----------------------|--------------|--------|
| `background` | NavigationView pane bg | `#ebebeb` / `#272727` | OK |
| `foreground` | <- defaults.foreground | `#1a1a1a` / `#ffffff` | OK |

### 2.13 Toolbar

| Property | platform-facts Windows | Preset L/D | Live | Status |
|----------|----------------------|------------|------|--------|
| `font.family` | <- defaults.font | (not set) | N/A | OK -- INHERIT |
| `font.size` | <- defaults.font | (not set) | N/A | OK -- INHERIT |
| `font.weight` | <- defaults.font | (not set) | N/A | OK -- INHERIT |
| `height` | CommandBar: 64 (compact: 48) | 64.0 / 64.0 | 64.0 | WARN -- expanded; issue 8e |
| `item_spacing` | WinUI3: 0 | 0.0 / 0.0 | 0.0 | OK |
| `padding` | WinUI3: 4 (left only) | 4.0 / 4.0 | 4.0 | OK |

### 2.14 Status Bar

| Property | platform-facts Windows | Preset L/D | Status |
|----------|----------------------|------------|--------|
| `font.family` | `lfStatusFont.lfFaceName` (live) | (not set) | OK -- INHERIT |
| `font.size` | `lfStatusFont.lfHeight` (live) | (not set) | OK -- INHERIT |
| `font.weight` | `lfStatusFont.lfWeight` (live) | (not set) | OK -- INHERIT |

### 2.15 List / Table

| Property | platform-facts Windows | Preset L / D | Live | Status |
|----------|----------------------|--------------|------|--------|
| `background` | <- defaults.background | (not set) | N/A | OK -- safety net |
| `foreground` | <- defaults.foreground | (not set) | N/A | OK -- INHERIT |
| `alternate_row` | L=#f9f9f9 D=#262626 | `#f9f9f9` / `#262626` | N/A | OK |
| `selection` | <- defaults.selection | (not set) | N/A | OK -- INHERIT |
| `selection_foreground` | <- defaults.selection_foreground | (not set) | N/A | OK -- INHERIT |
| `header_background` | ~= defaults.background | (not set) | N/A | OK -- INHERIT <- surface |
| `header_foreground` | <- defaults.foreground | (not set) | N/A | OK -- INHERIT |
| `grid_color` | uses border | (not set) | N/A | OK -- INHERIT <- border |
| `item_height` | WinUI3 ListView: 40 | 40.0 / 40.0 | 40.0 | OK |
| `padding_horizontal` | WinUI3: 12 | 12.0 / 12.0 | 12.0 | OK |
| `padding_vertical` | WinUI3: 0 | 4.0 / 4.0 | 4.0 | WARN -- spec says 0 |

### 2.16 Popover / Dropdown

| Property | platform-facts Windows | Preset L / D | Status |
|----------|----------------------|--------------|--------|
| `background` | Flyout bg = surface | `#ffffff` / `#2d2d2d` | OK |
| `foreground` | <- defaults.foreground | `#1a1a1a` / `#ffffff` | OK |
| `border` | <- defaults.border | (not set) | OK -- INHERIT |
| `radius` | <- defaults.radius_lg | (not set) | OK -- INHERIT |

### 2.17 Splitter

| Property | platform-facts Windows | Preset L/D | Live | Status |
|----------|----------------------|------------|------|--------|
| `width` | SplitView border: 1 | 1.0 / 1.0 | 1.0 | OK |

### 2.18 Separator

| Property | platform-facts Windows | Preset L / D | Status |
|----------|----------------------|--------------|--------|
| `color` | <- defaults.border | `#e5e5e5` / `#3b3b3b` | WARN -- dark #3b3b3b vs border #454545; intentional |

### 2.19 Text Scale

No explicit text_scale in preset; resolves from defaults.font (14/400)
with line_height = 1.43 * size = 20.02.

| Role | platform-facts Fluent | Resolves to | Status |
|------|-----------------------|-------------|--------|
| `caption` | 12epx, 400 | 14, 400, lh=20.02 | **ERR** -- size should be 12 |
| `section_heading` | 20epx, **600** | 14, 400, lh=20.02 | **ERR** -- size=20, wt=600 (19d) |
| `dialog_title` | 28epx, **600** | 14, 400, lh=20.02 | **ERR** -- size=28, wt=600 (19e) |
| `display` | 68epx, **600** | 14, 400, lh=20.02 | **ERR** -- size=68, wt=600 (NEW: 29a) |

### 2.20 Layout Spacing

| Tier | Fluent token | Preset | Status |
|------|-------------|--------|--------|
| `xxs` | XXSmall = 2px | 2.0 | OK |
| `xs` | XSmall = 4px | 4.0 | OK |
| `s` | Small = 8px | 8.0 | OK |
| `m` | Medium = 12px | 12.0 | OK |
| `l` | Large = 16px | 16.0 | OK |
| `xl` | XXLarge = 24px | 24.0 | OK |
| `xxl` | XXXLarge = 32px | 36.0 | WARN -- 32 vs 36 |

### 2.21 Switch / Toggle

| Property | platform-facts Windows | Preset L/D | Live | Status |
|----------|----------------------|------------|------|--------|
| `track_width` | WinUI3: 40 | 40.0 / 40.0 | 40.0 | OK |
| `track_height` | WinUI3: 20 | 20.0 / 20.0 | 20.0 | OK |
| `thumb_size` | WinUI3: 12 (rest) | 12.0 / 12.0 | 12.0 | OK |
| `track_radius` | 10px (pill) | 10.0 / 10.0 | 10.0 | OK |
| `checked_bg` | <- defaults.accent | (not set) | N/A | OK -- INHERIT |
| `unchecked_bg` | ToggleSwitchFillOff | `#c5c5c5` / `#454545` | N/A | OK |
| `thumb_bg` | ToggleSwitchKnob | `#ffffff` / `#ffffff` | N/A | OK |

### 2.22 Dialog

| Property | platform-facts Windows | Preset L/D | Live | Status |
|----------|----------------------|------------|------|--------|
| `min_width` | ContentDialog: 320 | 320.0 / 320.0 | 320.0 | OK |
| `max_width` | ContentDialog: 548 | 560.0 / 560.0 | 560.0 | **ERR** -- 548 (1a) |
| `min_height` | ContentDialog: 184 | 140.0 / 140.0 | 140.0 | **ERR** -- 184 (1b) |
| `max_height` | ContentDialog: 756 | 600.0 / 600.0 | 600.0 | **ERR** -- 756 (1c) |
| `content_padding` | WinUI3: 24 | 24.0 / 24.0 | 24.0 | OK |
| `button_spacing` | WinUI3: 8 | 8.0 / 8.0 | 8.0 | OK |
| `button_order` | primary leftmost | `trailing_affirmative` | same | WARN -- verify enum |
| `title_font` | 20px SemiBold (600) | (not set) | N/A | **ERR** -- inherits 14/400 (NEW: 29b) |
| `radius` | 8px (OverlayCornerRadius) | (not set) | N/A | OK -- INHERIT <- radius_lg=8 |
| `icon_size` | **(none)** | 32.0 / 32.0 | 32.0 | WARN -- no spec |

### 2.23 Spinner / Progress Ring

| Property | platform-facts Windows | Preset L/D | Live | Status |
|----------|----------------------|------------|------|--------|
| `diameter` | ProgressRing: 32 | 24.0 / 24.0 | 24.0 | **ERR** -- 32 (1f) |
| `min_size` | WinUI3: 16 | 16.0 / 16.0 | 16.0 | OK |
| `stroke_width` | WinUI3: 4 | 2.0 / 2.0 | 2.0 | **ERR** -- 4 (1d) |
| `fill` | <- defaults.accent | (not set) | N/A | WARN -- safety net uses foreground |

### 2.24 ComboBox / Dropdown Trigger

| Property | platform-facts Windows | Preset L/D | Live | Status |
|----------|----------------------|------------|------|--------|
| `min_height` | ComboBox: 32 | 32.0 / 32.0 | 32.0 | OK |
| `min_width` | WinUI3: 64 | 120.0 / 120.0 | 120.0 | **ERR** -- 64 (1m) |
| `padding_horizontal` | WinUI3: 12 | 12.0 / 12.0 | 12.0 | OK |
| `arrow_size` | WinUI3: 12 | 12.0 / 12.0 | 12.0 | OK |
| `arrow_area_width` | WinUI3: 38 | 38.0 / 38.0 | 38.0 | OK |
| `radius` | <- defaults.radius | (not set) | N/A | OK -- INHERIT |

### 2.25 Segmented Control

No native Windows equivalent; all **(none)** in platform-facts.

| Property | Preset L/D | Live | Status |
|----------|------------|------|--------|
| `segment_height` | 28.0 / 28.0 | 28.0 | OK -- no spec |
| `separator_width` | 1.0 / 1.0 | 1.0 | OK -- no spec |
| `padding_horizontal` | 12.0 / 12.0 | 12.0 | OK -- no spec |
| `radius` | (not set) | N/A | OK -- INHERIT |

### 2.26 Card / Container

| Property | platform-facts Windows | Preset L/D | Status |
|----------|----------------------|------------|--------|
| `background` | CardBackgroundFillColorDefault | (not set) | OK -- INHERIT <- surface |
| `border` | CardStrokeColorDefault | (not set) | OK -- INHERIT <- border |
| `radius` | 8px (OverlayCornerRadius) | (not set) | OK -- INHERIT <- radius_lg=8 |
| `shadow` | **(none)** -- border only | (not set) | WARN -- inherits true; Fluent: border |
| `padding` | 12px (convention) | 16.0 / 16.0 | WARN -- 12 vs 16 |

### 2.27 Expander / Disclosure

| Property | platform-facts Windows | Preset L/D | Live | Status |
|----------|----------------------|------------|------|--------|
| `header_height` | Expander: 48 | 40.0 / 40.0 | 40.0 | **ERR** -- 48 (1g) |
| `arrow_size` | chevron: 12 | 12.0 / 12.0 | 12.0 | OK |
| `content_padding` | WinUI3: 16 | 16.0 / 16.0 | 16.0 | OK |
| `radius` | <- defaults.radius | (not set) | N/A | OK -- INHERIT |

### 2.28 Link

| Property | platform-facts Windows | Preset L / D | Status |
|----------|----------------------|--------------|--------|
| `color` | AccentTextFillColor | (not set) | OK -- INHERIT <- link |
| `visited` | same as link | (not set) | OK -- INHERIT <- link |
| `underline` | no underline by default | true / true | WARN -- design choice |
| `background` | transparent | `#00000000` / `#00000000` | OK |
| `hover_bg` | SubtleFillColorSecondary | `#0078d418` / `#0078d418` | OK |

---

### ERR Summary (17 items)

| # | Section | Property | Preset has | Should be | Filed as |
|---|---------|----------|-----------|-----------|----------|
| 1 | 2.1.3 | dark `accent_foreground` | #ffffff | #000000 | 18a |
| 2 | 2.1.4 | dark `danger` | #ff9999 | #ff99a4 | 18b |
| 3 | 2.1.4 | dark `warning` | #f0c239 | #fce100 | 18c |
| 4 | 2.6 | `menu.icon_spacing` | 8 | 12 | 1l |
| 5 | 2.7 | `tooltip.padding_horizontal` | 8 | 9 | 1e |
| 6 | 2.19 | `caption` size | 14 | 12 | partial (19d/19e) |
| 7 | 2.19 | `section_heading` size+wt | 14/400 | 20/600 | 19d |
| 8 | 2.19 | `dialog_title` size+wt | 14/400 | 28/600 | 19e |
| 9 | 2.19 | `display` size+wt | 14/400 | 68/600 | **NEW** (29a) |
| 10 | 2.22 | `dialog.max_width` | 560 | 548 | 1a |
| 11 | 2.22 | `dialog.min_height` | 140 | 184 | 1b |
| 12 | 2.22 | `dialog.max_height` | 600 | 756 | 1c |
| 13 | 2.22 | `dialog.title_font` | 14/400 | 20/600 | **NEW** (29b) |
| 14 | 2.23 | `spinner.diameter` | 24 | 32 | 1f |
| 15 | 2.23 | `spinner.stroke_width` | 2 | 4 | 1d |
| 16 | 2.24 | `combo_box.min_width` | 120 | 64 | 1m |
| 17 | 2.27 | `expander.header_height` | 40 | 48 | 1g |

### WARN Summary (23 items)

| # | Section | Property | Notes |
|---|---------|----------|-------|
| W1 | 2.1.8 | `icon_sizes.dialog` = 22 | No Windows spec; reasonable |
| W2 | 2.1.8 | `icon_sizes.panel` = 20 | No Windows spec; matches toolbar |
| W3 | 2.3 | `button.min_width` = 64 | No Windows spec |
| W4 | 2.3 | `button.min_height` = 27 | Effective WinUI3; issue 8c |
| W5 | 2.3 | `button.padding_vertical` = 6 | Asymmetric 5/6; acceptable |
| W6 | 2.6 | `menu.item_height` = 23 | Mouse mode; touch=31; issue 8d |
| W7 | 2.6 | `menu.padding_horizontal` = 12 | Spec says 11 |
| W8 | 2.7 | `tooltip.padding_vertical` = 8 | Upper bound of 6-8 |
| W9 | 2.8 | `scrollbar.thumb` inherits muted | Not measured #c2c2c2 |
| W10 | 2.8 | `scrollbar.slider_width` = 6 | Win32=17; modern thin |
| W11 | 2.10 | `progress_bar.min_width` = 100 | No spec |
| W12 | 2.11 | `tab.padding_horizontal` = 12 | WinUI3: 8 |
| W13 | 2.11 | `tab.padding_vertical` = 4 | WinUI3: 3 |
| W14 | 2.13 | `toolbar.height` = 64 | Expanded; compact=48; issue 8e |
| W15 | 2.15 | `list.padding_vertical` = 4 | Spec says 0 |
| W16 | 2.18 | dark `separator.color` #3b3b3b | vs border #454545; intentional |
| W17 | 2.20 | `spacing.xxl` = 36 | Fluent XXXLarge=32 |
| W18 | 2.22 | `dialog.button_order` | Verify enum semantics |
| W19 | 2.22 | `dialog.icon_size` = 32 | No Windows spec |
| W20 | 2.23 | `spinner.fill` inherits foreground | Spec says accent |
| W21 | 2.26 | `card.shadow` inherits true | Fluent: border only |
| W22 | 2.26 | `card.padding` = 16 | Spec says 12 |
| W23 | 2.28 | `link.underline` = true | Fluent: no underline |

### NEW Issues

**29a. Windows 11 missing `text_scale.display` -- size and weight wrong**

**File:** `src/presets/windows-11.toml` (light and dark)

The display text scale entry resolves to defaults.font (14px/400) but
Fluent Display is 68epx/600. All four text_scale roles need explicit
sections. Issues 19d/19e cover section_heading and dialog_title weights
only. Caption size (12 not 14) and display (68/600 not 14/400) are new.

**29b. Windows 11 missing `dialog.title_font` -- should be 20px/600**

**File:** `src/presets/windows-11.toml` (light and dark)

Dialog title font inherits defaults.font (14px/400). Platform-facts
section 2.22 documents "20px SemiBold". Both variants need:

```toml
[light.dialog.title_font]
size = 20.0
weight = 600
```

**Total: 182 properties verified across 28 sections. 17 ERR (15
previously filed, 2 new). 23 WARN (minor/acceptable).**
