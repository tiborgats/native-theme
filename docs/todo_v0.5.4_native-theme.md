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

### 1g. Windows 11 `expander.header_height = 40` -- platform-facts says 48

**File:** `src/presets/windows-11.toml:199,398`

platform-facts.md SS1.2.4: "WinUI3: ExpanderMinHeight=48". Preset uses 40.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Fix to 48 | Matches WinUI3 resource | Taller header |
| B | Keep 40 | Compact | Contradicts source |

**Recommended:** A.

### 1h. Windows 11 `expander.content_padding = 16` -- platform-facts says 16

Correct. Matches `ExpanderContentPadding=16`.

### 1i. Adwaita `dialog.button_spacing = 8` -- platform-facts says 12

**File:** `src/presets/adwaita.toml:175,370`

platform-facts.md SS1.4.4: "AdwAlertDialog button spacing: 12px" from
`_message-dialog.scss .response-area { border-spacing: 12px }`. Preset uses 8.

**Impact:** Button gap 4px narrower than native Adwaita.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Fix to 12 | Matches libadwaita CSS source | Wider gap |
| B | Keep 8, document | Consistent with current look | Contradicts authoritative CSS |

**Recommended:** A.

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

### 1k. Adwaita `checkbox.indicator_size = 20` -- platform-facts says 14 (20 with padding)

**File:** `src/presets/adwaita.toml:99,289`

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
- `lint_toml()` (only doctest)
- `icon_name()` exhaustive mapping
- `resolve()` individual inheritance rules (tested indirectly, not directly)
- `SystemTheme::with_overlay()` accent re-derivation chain
- `from_toml_with_base()` public API
- Cross-platform `detect_*` functions (inherently hard to test)

### Unnecessary/bloated tests
- `resolved.rs` duplicate construction tests (~400 lines for same thing)
- Duplicated preset tests between integration and unit
- `dialog_order.rs` has 6 tests for a 2-variant enum

---

## Priority Summary

| # | Issue | Severity | Effort | Best Fix |
|---|-------|----------|--------|----------|
| 1j | macOS button_order leading vs trailing | high | trivial | Fix preset to `trailing_affirmative` |
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
