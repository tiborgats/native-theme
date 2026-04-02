# v0.5.4 -- native-theme-iced: Deep Critical Analysis

Comprehensive audit of the iced connector crate covering unit tests,
correctness vs platform-facts.md, code quality, API design, and
cross-connector symmetry with the gpui connector.

Files analyzed:
- `connectors/native-theme-iced/Cargo.toml`
- `connectors/native-theme-iced/src/lib.rs` (519 lines)
- `connectors/native-theme-iced/src/extended.rs` (118 lines)
- `connectors/native-theme-iced/src/icons.rs` (615 lines)
- `connectors/native-theme-iced/src/palette.rs` (112 lines)
- `connectors/native-theme-iced/examples/showcase.rs` (2903 lines)
- `native-theme/src/model/mod.rs` (ThemeVariant structure)
- `native-theme/src/model/resolved.rs` (ResolvedThemeVariant, 252 lines)
- `native-theme/src/lib.rs` (public API, re-exports)
- `connectors/native-theme-gpui/src/lib.rs` (reference for symmetry)
- `connectors/native-theme-gpui/src/colors.rs` (reference for color mapping)
- `docs/platform-facts.md` (1475 lines, cross-reference target)

All 57 tests pass (lib.rs:21, extended.rs:4, icons.rs:29, palette.rs:3).
No `unwrap()`/`expect()`/`panic!()` in production code -- all instances
are inside `#[cfg(test)] #[allow(clippy::unwrap_used, clippy::expect_used)]`
blocks or doc comments.

---

## 1. `apply_overrides()` Is Public Dead Code

`extended.rs:17-25` defines a public function `apply_overrides()` with
4 unit tests (lines 56-116). However, `to_theme()` at `lib.rs:116-123`
inlines the identical logic in a closure instead of calling this function:

```rust
// lib.rs:116-123 (actual production code path -- inlined)
iced_core::theme::Theme::custom_with_fn(name.to_string(), pal, move |p| {
    let mut ext = iced_core::theme::palette::Extended::generate(p);
    ext.secondary.base.color = palette::to_color(btn_bg);
    ext.secondary.base.text = palette::to_color(btn_fg);
    ext.background.weak.color = palette::to_color(surface);
    ext.background.weak.text = palette::to_color(foreground);
    ext
})
```

```rust
// extended.rs:17-25 (dead code -- never called from production path)
pub fn apply_overrides(
    extended: &mut iced_core::theme::palette::Extended,
    resolved: &native_theme::ResolvedThemeVariant,
) {
    extended.secondary.base.color = to_color(resolved.button.background);
    extended.secondary.base.text = to_color(resolved.button.foreground);
    extended.background.weak.color = to_color(resolved.defaults.surface);
    extended.background.weak.text = to_color(resolved.defaults.foreground);
}
```

The doc comment at `lib.rs:92` falsely claims overrides are done "via
`extended::apply_overrides()`". This creates three problems:
1. Dual maintenance: changes to one copy are not reflected in the other.
2. 4 tests validate dead code, giving false confidence.
3. Doc comment is factually wrong.

The reason for the inline approach is performance: `to_theme()` captures
4 `Copy` `Rgba` values (16 bytes total) rather than cloning the entire
`ResolvedThemeVariant` (~2KB with heap data) into the closure. The dead
`apply_overrides()` takes `&ResolvedThemeVariant` which would require the
closure to own a clone.

**Impact:** High. Tests pass on dead code. Doc comment is wrong.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Restructure `apply_overrides()` to accept 4 individual `Rgba` values; call it from `to_theme()` closure | Single source of truth; tests validate production code; doc comment becomes accurate | Breaking change to `apply_overrides()` signature (minor, pre-1.0) |
| B | Restructure `apply_overrides()` to accept a small 4-field struct; call from `to_theme()` | Same as A; cleaner parameter list | New struct type for 4 values; more indirection |
| C | Delete `apply_overrides()` and its 4 tests; keep inline code; fix doc comment | No dead code; simpler module | Loses named abstraction; override logic only testable via integration tests on `to_theme()` |
| D | Make `apply_overrides()` a thin wrapper that takes 4 `Rgba` values + 4 computed `iced_core::Color` values, eliminating conversion inside | Most flexible; can be tested with arbitrary Color values | Over-engineering for 4 assignments |

**Recommended: A.** The function name is clear, the tests are valuable,
and the signature change is trivial:

```rust
pub fn apply_overrides(
    extended: &mut Extended,
    btn_bg: Rgba, btn_fg: Rgba,
    surface: Rgba, foreground: Rgba,
) { ... }
```

Then `to_theme()` calls it inside the closure with the 4 captured values.
Fix the doc comment at `lib.rs:92`.

---

## 2. Extended Palette Only Overrides 2 of 5 Color Families

`to_theme()` overrides `secondary.base` and `background.weak` but relies
on iced's `Extended::generate()` auto-derivation for the other three
families: **primary**, **success**, and **danger**.

The auto-generation algorithm (iced's `Extended::generate()`) derives
tints/shades from the 6-field base palette using HSL manipulation. This
diverges from the actual resolved theme colors for two reasons:

1. Community presets (Catppuccin, Dracula, Nord, Gruvbox) choose status
   colors with deliberate artistic intent, not by algorithmic derivation
   from a single base hue.
2. The `*_foreground` pairs (text-on-status-background) are platform-
   derived values documented in platform-facts.md section 2.1.4. These
   are discarded entirely.

The resolved theme has all the data needed:
- `d.accent` / `d.accent_foreground` (primary)
- `d.success` / `d.success_foreground`
- `d.danger` / `d.danger_foreground`

The gpui connector (colors.rs:84-100, 133, 160-178) maps ALL of these
explicitly. The iced connector does not.

Note: iced's Extended palette has no `warning` family. The existing
`palette.warning -> d.warning` mapping in `to_palette()` is the only
path for warning colors. Similarly, there is no `info` family.

**Impact:** Medium. Status-colored iced widgets (danger buttons, success
badges) use iced-generated HSL approximations instead of the theme's
actual status colors. Visually noticeable on community presets where
status colors have distinct character.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Override `primary.base`, `success.base`, `danger.base` color+text from resolved theme (6 additional lines) | Theme colors propagate accurately to all iced widgets; consistent with gpui connector; status badges/alerts use theme-chosen colors | 6 more Rgba captures; 6 more override lines |
| B | Override all `.base`, `.weak`, `.strong` tiers for all 5 families using resolved + derived tints | Closest possible match to theme intent across all sub-palette tiers | Complex; deriving weak/strong variants needs a tinting algorithm; over-engineering |
| C | Document the limitation; leave auto-generation | No code change; users know what to expect | Widgets visually diverge from theme intent on community presets |

**Recommended: A.** Six additional lines of overrides (same pattern
as the existing secondary/background overrides) eliminate the biggest
divergence. The `.weak` and `.strong` sub-tiers can remain auto-derived
since they are visual variants (hover, disabled) rather than semantic
colors.

```rust
ext.primary.base.color = to_color(accent);
ext.primary.base.text = to_color(accent_foreground);
ext.success.base.color = to_color(success);
ext.success.base.text = to_color(success_foreground);
ext.danger.base.color = to_color(danger);
ext.danger.base.text = to_color(danger_foreground);
```

---

## 3. `from_preset()` Uses Slug as Display Name

`lib.rs:139-144`:

```rust
let spec = native_theme::ThemeSpec::preset(name)?;
let variant = spec.into_variant(is_dark).ok_or_else(|| { ... })?;
let resolved = variant.into_resolved()?;
let theme = to_theme(&resolved, name);  // <-- slug, not spec.name
```

`from_preset("catppuccin-mocha", true)` passes the lookup slug
`"catppuccin-mocha"` as the theme display name. `ThemeSpec` has a
proper display name in `spec.name` (e.g., `"Catppuccin Mocha"`), but
`spec` is consumed by `into_variant()` before the name is captured.

The gpui connector has the same issue at its `lib.rs:152-157`.

**Impact:** Cosmetic. Theme pickers show `"catppuccin-mocha"` instead
of `"Catppuccin Mocha"`.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Capture `spec.name` before consuming `spec` with `let display_name = spec.name.clone()` | Human-readable names in theme pickers; matches spec's intended display name; consistent with `from_system()` which uses `sys.name` | One extra String clone (~20 bytes) |
| B | Use `ThemeSpec::preset_ref()` or similar non-consuming lookup if available | Avoids clone | No such API exists currently |
| C | Keep slug as display name | No change | Theme pickers show kebab-case slugs; inconsistent with `from_system()` |

**Recommended: A.** One-line addition. Apply same fix to gpui connector.

---

## 4. `from_preset()` Error Message Misleading

`lib.rs:140-142`:

```rust
let variant = spec.into_variant(is_dark).ok_or_else(|| {
    native_theme::Error::Format(format!("preset '{name}' has no light or dark variant"))
})?;
```

`into_variant(is_dark)` returns `None` only when BOTH variants are
empty (it falls back from preferred to alternate before giving up).
The error message "has no light or dark variant" implies one specific
variant is absent, which is misleading.

The gpui connector has the same incorrect message at its `lib.rs:152-154`.

**Impact:** Low. Confusing error for edge case of completely empty
ThemeSpecs.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Fix the error message to `"preset '{name}' has no variants (both light and dark are empty)"` | Accurate | Trivial change |
| B | Include which variant was preferred: `"preset '{name}' has no usable variant (requested {mode}, fallback also empty)"` | More diagnostic detail | Slightly more complex format string |
| C | Keep current message | No change | Misleading wording |

**Recommended: A.** Apply same fix to gpui connector.

---

## 5. `from_system()` Discards `is_dark` Flag

`lib.rs:154-161`:

```rust
pub fn from_system()
-> native_theme::Result<(iced_core::theme::Theme, native_theme::ResolvedThemeVariant)> {
    let sys = native_theme::SystemTheme::from_system()?;
    let name = sys.name;
    let resolved = if sys.is_dark { sys.dark } else { sys.light };
    let theme = to_theme(&resolved, &name);
    Ok((theme, resolved))
}
```

The `sys.is_dark` flag is used to pick the variant but not returned.
Callers receive `(Theme, ResolvedThemeVariant)` with no way to determine
whether the system is in dark mode without calling
`SystemTheme::from_system()` a second time.

The gpui connector's `from_system()` has the same pattern but it is
less problematic because `gpui_component::Theme` stores `.mode`
(dark/light) internally. Iced's `Theme` has no mode accessor.

**Impact:** Medium (API ergonomic gap). Any iced app that needs to
conditionally render based on dark/light mode must duplicate the system
detection call.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Return a triple: `Ok((theme, resolved, is_dark))` | No data lost; trivial for callers to destructure; complete information | Breaking change to tuple shape |
| B | Wrap return in a named struct `SystemThemeResult { theme, resolved, is_dark }` | Named fields; extensible for future additions | New public type; more API surface |
| C | Add `is_dark` field to `SystemTheme` re-export and document that users should use it | No connector API change | Users must import SystemTheme directly; convenience API incomplete |
| D | Keep current signature, document the gap | No change | Users surprised when they need the flag |

**Recommended: A.** Schedule for next minor version alongside issue 6
and 7. The breaking change is small and the ergonomic benefit is clear.

---

## 6. `SystemThemeExt::to_iced_theme()` Discards Resolved Variant

`lib.rs:164-174`:

```rust
pub trait SystemThemeExt {
    fn to_iced_theme(&self) -> iced_core::theme::Theme;
}

impl SystemThemeExt for native_theme::SystemTheme {
    fn to_iced_theme(&self) -> iced_core::theme::Theme {
        to_theme(self.active(), &self.name)
    }
}
```

Returns only `Theme`, discarding `ResolvedThemeVariant`. Both
`from_system()` and `from_preset()` return tuples including the resolved
variant because it is essential for metric helpers (`button_padding()`,
`scrollbar_width()`, `font_family()`, etc.).

Users of `to_iced_theme()` lose access to all 15 widget metric helpers.

The gpui connector's `to_gpui_theme()` at `lib.rs:203-209` also returns
only `Theme`, so this is a consistent pattern between connectors -- but
both are missing the resolved variant.

**Impact:** Low-Medium. API asymmetry within the iced connector itself.
Users discover the gap when they try `button_padding()` and realize they
need the resolved variant.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Return `(Theme, &ResolvedThemeVariant)` from `to_iced_theme()` | Consistent with from_system/from_preset; users get everything they need | Breaking change; callers must destructure |
| B | Add separate `to_iced_theme_with_resolved()` returning the tuple | Backward-compatible | Two near-identical methods; API clutter |
| C | Deprecate `to_iced_theme()` and direct users to `from_system()` | Cleaner API; no duplicate methods | Users with an existing `SystemTheme` cannot reuse it conveniently |
| D | Keep current signature | No change | Inconsistent with other entry points |

**Recommended: A.** Consistency across entry points matters. Pre-1.0
crate, so breaking changes are acceptable.

---

## 7. `to_theme()` Missing `is_dark` Parameter

`lib.rs:103`: the iced connector's `to_theme()` is
`fn to_theme(resolved: &ResolvedThemeVariant, name: &str) -> Theme`,
while the gpui connector's is
`fn to_theme(resolved: &ResolvedThemeVariant, name: &str, is_dark: bool) -> Theme`.

Currently no visible effect because iced's `Theme::custom_with_fn()`
does not use a mode parameter. However:

1. If iced adds mode-aware Extended generation, the iced connector
   cannot pass mode info.
2. Inconsistency between connectors makes switching frameworks harder.
3. The resolved variant does not carry an `is_dark` flag (it is a
   pure data snapshot).

**Impact:** Low today. Forward-compatibility and symmetry gap.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add `is_dark: bool` parameter to `to_theme()` | Consistent with gpui connector; forward-compatible | Breaking change; callers must pass extra param; unused today |
| B | Store `is_dark` inside `to_theme()` closure for future use but don't expose in API | Internal future-proofing | Hidden parameter; inconsistent with gpui |
| C | Keep current signature; add `is_dark` when iced needs it | Simpler API today | Deferred breaking change |

**Recommended: C.** Unlike the gpui connector which uses `is_dark` to
set `ThemeMode`, iced has no equivalent concept today. Adding an unused
parameter violates YAGNI. Revisit when iced adds mode awareness.

---

## 8. No Integration Tests

All 57 tests are unit tests within `#[cfg(test)]` modules in the 4 source
files. There is no `tests/` directory. The full pipeline
(`ThemeSpec::preset()` -> `into_variant()` -> `into_resolved()` ->
`to_theme()` -> verify palette/extended values) is untested end-to-end.

Missing integration test scenarios:
- **All-presets smoke test**: does every preset produce a valid theme
  for both light and dark variants?
- **Palette color precision**: do palette fields exactly match the
  resolved theme colors (not just "non-zero")?
- **Extended override precision**: do overridden extended fields match
  specific resolved theme hex values?
- **Variant fallback through convenience API**: does `from_preset()`
  correctly fall back when only one variant exists?
- **Multi-preset comparison**: do different presets produce different
  themes? (regression guard)

**Impact:** Medium. Cross-crate regressions between native-theme and
native-theme-iced are undetected until manual testing.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Create `tests/integration.rs` with the scenarios above | Validates the primary user workflow; catches cross-crate regressions; documents expected behavior | Small compile time increase; needs dev-dependency on native-theme |
| B | Add a broader smoke test inside `lib.rs` `#[cfg(test)]` module | No new file; tests are near the code | Still technically unit tests; cannot test with different feature flags |
| C | No change | No work | Full pipeline untested |

**Recommended: A.** Integration tests are the standard Rust pattern
for validating a crate's public API from the consumer's perspective.

---

## 9. Missing `LinuxDesktop` Re-export

`lib.rs:82-85` re-exports many core types:

```rust
pub use native_theme::{
    AnimatedIcon, Error, IconData, IconProvider, IconRole, IconSet,
    ResolvedThemeVariant, Result, Rgba, SystemTheme, ThemeSpec, ThemeVariant,
    TransformAnimation,
};
```

But not `LinuxDesktop`. The gpui connector does at `lib.rs:78-79`:

```rust
#[cfg(target_os = "linux")]
pub use native_theme::LinuxDesktop;
```

**Impact:** Low. Linux users must add `native-theme` as a direct
dependency to access the desktop environment enum.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add the conditional re-export (one line) | Parity with gpui; iced Linux users don't need native-theme as direct dep | One more conditional re-export |
| B | No change | No change | Asymmetric with gpui; Linux users need direct dep |

**Recommended: A.** One-line addition.

---

## 10. `colorize_monochrome_svg()` Missing `stroke="black"` Handling

`icons.rs:250-258`: the explicit-black replacement handles:
- `fill="black"`
- `fill="#000000"`
- `fill="#000"`

But not the `stroke` equivalents. SVGs using explicit black strokes
without `currentColor` (e.g., outline-style icons from third-party
sources) will not be colorized. The stroke attribute is semantically
distinct from fill -- outline icons that use `stroke` for their paths
would appear permanently black regardless of the color parameter.

The gpui connector has the same gap.

**Impact:** Low. All bundled sets (Material uses implicit fill, Lucide
uses `currentColor` for both fill and stroke) are handled correctly.
Only affects third-party SVGs with explicit `stroke="black"`.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add stroke replacements (3 lines) after the fill replacements | Covers stroke-based monochrome SVGs; handles Lucide-like third-party icons | 3 more string replacements per colorize call |
| B | Also inject `stroke` attribute in the root `<svg>` tag when no stroke exists | Most thorough | Could unintentionally add strokes to fill-only SVGs |
| C | No change | All bundled sets work | Third-party stroke-based SVGs permanently black |

**Recommended: A.** Three additional replacements, same pattern as the
existing fill replacements. Apply to both iced and gpui connectors.

```rust
let stroke_hex = format!("stroke=\"{}\"", hex);
replaced = replaced
    .replace("stroke=\"black\"", &stroke_hex)
    .replace("stroke=\"#000000\"", &stroke_hex)
    .replace("stroke=\"#000\"", &stroke_hex);
```

---

## 11. `extended` Module Unnecessarily Public

`lib.rs:77`: `pub mod extended` exposes `apply_overrides()` as public
API. Per issue 1, this function is dead code. Even after fixing issue 1,
the module's role is internal (apply overrides during theme construction).

By contrast, the gpui connector makes its equivalent modules `pub(crate)`
at `lib.rs:66-68`:

```rust
pub(crate) mod colors;
pub(crate) mod config;
pub(crate) mod derive;
```

**Impact:** Low. Unnecessarily large public API surface.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Change to `pub(crate) mod extended` | Reduces public API surface; consistent with gpui | Breaking if anyone imports it (unlikely for pre-1.0 crate) |
| B | Keep `pub` but document it as internal | Transparent | Internal function still exposed |
| C | Keep `pub` | No change | Inconsistent with gpui; exposes internal detail |

**Recommended: A.** Pre-1.0 crate. Clean up the API surface.

---

## 12. Many Resolved Colors Have No Helper Functions

Cross-referencing `ResolvedThemeDefaults` (resolved.rs:83-177) against
the iced connector's public API reveals these unmapped fields without
helper functions:

| Field | Purpose | Mapped to palette? | Helper fn? |
|-------|---------|-------------------|------------|
| `accent_foreground` | Text on accent bg | No | No |
| `border` | Border/divider color | No | No |
| `muted` | Secondary text | No | No |
| `link` | Hyperlink color | No | No |
| `selection` | Selection highlight bg | No | No |
| `selection_foreground` | Text on selection | No | No |
| `selection_inactive` | Unfocused selection bg | No | No |
| `disabled_foreground` | Disabled text | No | No |
| `disabled_opacity` | Disabled control opacity | No | No |
| `shadow` | Shadow color | No | No |
| `shadow_enabled` | Whether shadows are used | No | No |
| `focus_ring_color` | Focus indicator color | No | No |
| `focus_ring_width` | Focus indicator width | No | No |
| `focus_ring_offset` | Focus indicator gap | No | No |
| `info` | Info/attention color | No | No |
| `info_foreground` | Text on info bg | No | No |
| `danger_foreground` | Text on danger bg | No (see issue 2) | No |
| `success_foreground` | Text on success bg | No (see issue 2) | No |
| `warning_foreground` | Text on warning bg | No | No |
| `border_opacity` | Border alpha multiplier | No | No |
| `frame_width` | Border width | No | No |
| `spacing` (7 tiers) | Logical spacing scale | No | No |
| `icon_sizes` (5 contexts) | Per-context icon sizes | No | No |
| `text_scale` (4 roles) | Per-role text scale | No | No |

Out of ~36 color/geometry fields in `ResolvedThemeDefaults`, only 10
are mapped (6 palette + 4 extended overrides). The lib.rs coverage
table (lines 56-70) documents this honestly, but no helper functions
exist for the remaining fields.

The gpui connector does not expose helpers either (its `ThemeColor` maps
108 fields internally), but the iced connector's pattern of explicit
helper functions (e.g. `border_radius()`, `scrollbar_width()`) sets an
expectation that other commonly-needed fields would also have helpers.

Users who need border, link, selection, disabled, focus-ring, spacing,
or icon-size values must access `resolved.defaults.*` directly.

**Impact:** Medium. Not a bug, but a discoverability gap. The helper
pattern is established; its incompleteness is surprising.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add helpers for the most commonly needed fields: `border_color`, `disabled_opacity`, `focus_ring_color`, `link_color`, `selection_color`, `spacing` accessor, `icon_size` accessor | Most useful properties easily accessible; consistent with existing helpers; discoverable via IDE autocomplete | More API surface; ~15-20 more one-liner functions |
| B | Add a single `defaults()` accessor returning `&ResolvedThemeDefaults` | One function covers everything; minimal API surface | Not as discoverable; users must know the field names |
| C | No change, document that users access resolved fields directly | No API surface increase | Existing helper pattern creates false expectation of completeness |

**Recommended: B as primary, A selectively.** Add `pub fn defaults(&ResolvedThemeVariant) -> &ResolvedThemeDefaults` as a general accessor, plus targeted helpers for the ~5 most commonly needed fields (border_color, disabled_opacity, focus_ring_color, spacing, icon_sizes). This balances discoverability with API surface.

---

## 13. `info` and `warning_foreground` Colors Not Mapped

The core model provides:
- `info` / `info_foreground` (blue/teal semantic color pair)
- `warning_foreground` (text on warning background)

Iced's Extended palette has no `info` or `warning` family. The
`warning` base color reaches iced via `palette.warning` -> `d.warning`,
but its foreground pair is lost. `info` is lost entirely.

The gpui connector maps ALL of these: `tc.info`, `tc.info_foreground`,
`tc.info_hover`, `tc.info_active` at colors.rs:175-178.

**Impact:** Low. Users needing `info` or `warning_foreground` must
access `resolved.defaults.*` directly. This is an iced framework
limitation (no info family in Extended), not a connector bug.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Document the gap in extended.rs module docs and lib.rs coverage table | Users understand which colors are not mapped and why | No automatic mapping |
| B | Map `info` to an unused Extended palette slot (e.g., override `primary.weak` or `background.strong`) | Some mapping exists | Semantically wrong; confusing |
| C | Expose `info_color()` and `warning_foreground_color()` helper functions (returns iced Color) | Discoverable; users don't need raw Rgba access | Not integrated into iced's catalog system |

**Recommended: A + C.** Document the gap. Add simple helper functions
so users can easily access these colors even though iced's theme system
cannot consume them automatically.

---

## 14. Test Quality Assessment

### 14.1 Test inventory (57 tests total)

| File | Count | Scope |
|------|-------|-------|
| `lib.rs` | 21 | to_theme, from_preset, from_system, SystemThemeExt, all metric helpers, weight conversion |
| `icons.rs` | 29 | to_image_handle, to_svg_handle, colorize_monochrome_svg (7 variants), custom_icon_*, animated_frames_*, spin_rotation_*, into_* |
| `extended.rs` | 4 | apply_overrides (all 4 override fields) |
| `palette.rs` | 3 | to_color, to_palette (light), to_palette (dark) |

### 14.2 What is well tested

- `to_color()` conversion from Rgba to iced Color (exact value check)
- `to_palette()` maps all 6 fields for both light and dark variants
- All 15 font/metric helper functions return valid values
- `to_iced_weight()` maps all 9 standard weights + non-standard rounding + boundary values (0, 1000)
- `from_preset()` valid (both variants) and invalid paths
- `from_system()` does not panic (graceful skip when system unavailable)
- Icon handle conversions (Rgba->image, SVG->svg, both directions, both borrowing and consuming)
- SVG colorization: currentColor, explicit black fill, existing fill preserved, self-closing SVG valid XML, non-UTF-8 passthrough, Material-style fill injection
- Animated icon: frame conversion, spin rotation math, zero elapsed, half rotation, full wrap, zero duration safety, empty frames, RGBA-only frames, Transform variant
- Custom icon provider with `dyn IconProvider` support

### 14.3 What is missing

| Missing test | Severity | Why it matters |
|-------------|----------|---------------|
| All-presets smoke test | Medium | Any broken preset goes undetected |
| Palette values match resolved colors precisely (exact hex) | Medium | Off-by-one in color conversion hidden by "non-zero" assertions |
| Extended palette override values match specific resolved theme hex | Medium | Current tests only check "changed from original DARK palette" |
| `colorize_monochrome_svg()` with mixed fill attributes (e.g., `fill="black"` on one path and `fill="red"` on another) | Low | Partial colorization could corrupt multi-path SVGs |
| `colorize_monochrome_svg()` with `fill="#FFF"` or other non-black explicit colors | Low | Only black variants tested |
| `to_svg_handle()` color arg with non-UTF-8 SVG | Low | Combination of two edge case paths untested |
| `animated_frames_to_svg_handles()` with mixed SVG+RGBA frames | Low | Documented behavior (RGBA silently excluded) but not verified |
| `spin_rotation_radians()` with very large elapsed (days) | Low | f32 modulo precision at large values |
| `from_preset()` with a preset that has only one variant | Low | Variant fallback path tested in core but not through iced convenience API |
| `to_theme()` produces different themes for different presets | Low | Regression guard against all-same output |
| `to_theme()` extended palette overrides with light variant (tests only use catppuccin-mocha light, never dark) | Medium | Dark variant overrides untested in extended.rs |
| `apply_overrides()` with a dark variant input | Medium | All 4 tests use `make_resolved()` which always passes `is_dark=false` |

### 14.4 Tests that only test one preset

All test helper functions use only `catppuccin-mocha`:

- `lib.rs:306-313 make_resolved(is_dark)` -- always catppuccin-mocha
- `extended.rs:47-53 make_resolved()` -- always catppuccin-mocha, always `is_dark=false`
- `palette.rs:76-81` and `palette.rs:99-104` -- always catppuccin-mocha

No test ever exercises a different preset (e.g., dracula, nord, adwaita,
breeze). This means preset-specific resolution bugs are invisible.

### 14.5 Tests with weak assertions

- `to_theme_produces_non_default_theme` (lib.rs:318-331) -- only checks
  `primary.r > 0.0 || primary.g > 0.0 || primary.b > 0.0` ("non-zero").
  This passes for any non-black color. Should check that the palette
  matches catppuccin-mocha's known accent color.

- `to_theme_from_preset` (lib.rs:334-341) -- checks `background.r > 0.9`
  for "light variant has light background". Fragile if catppuccin-mocha
  light ever adjusts its background shade.

- `apply_overrides_sets_secondary_base_color` (extended.rs:56-73) --
  asserts color differs from DARK palette original. Should assert the
  specific value matches `resolved.button.background`.

### 14.6 No redundant or bloated tests

All 57 tests serve a clear purpose. None are redundant.

---

## 15. Correctness: Cross-Reference with platform-facts.md

### 15.1 Palette mapping -- CORRECT

The 6-field mapping in `palette.rs:29-39`:

| iced Palette | Resolved field | platform-facts section | Verified |
|-------------|---------------|----------------------|----------|
| background | d.background | 2.1.3 -- all platforms provide this | Correct |
| text | d.foreground | 2.1.3 -- all platforms provide this | Correct |
| primary | d.accent | 2.1.3 -- all platforms provide this | Correct |
| success | d.success | 2.1.4 -- all platforms provide this | Correct |
| warning | d.warning | 2.1.4 -- all platforms provide this | Correct |
| danger | d.danger | 2.1.4 -- all platforms provide this | Correct |

### 15.2 Extended overrides -- CORRECT

| Extended field | Resolved field | platform-facts | Verified |
|---------------|---------------|---------------|----------|
| secondary.base.color | button.background | 2.3 -- all platforms provide button bg | Correct |
| secondary.base.text | button.foreground | 2.3 -- all platforms provide button fg | Correct |
| background.weak.color | defaults.surface | 2.1.3 -- surface is elevated/card bg | Correct |
| background.weak.text | defaults.foreground | 2.1.3 -- foreground text on surface | Correct |

### 15.3 Widget metric helpers -- CORRECT

| Helper | Resolved field | platform-facts | Verified |
|--------|---------------|---------------|----------|
| `button_padding()` | button.padding_vertical/horizontal | 2.3 -- all platforms provide | Correct |
| `input_padding()` | input.padding_vertical/horizontal | 2.4 -- all platforms provide | Correct |
| `border_radius()` | defaults.radius | 2.1.6 -- macOS:5, Win:4, KDE:5, GNOME:9 | Correct |
| `border_radius_lg()` | defaults.radius_lg | 2.1.6 -- macOS:10, Win:8, GNOME:15 | Correct |
| `scrollbar_width()` | scrollbar.width | 2.8 -- macOS:16/7, Win:17, KDE:21, GNOME:8 | Correct |

### 15.4 Font mapping -- CORRECT

| Helper | Resolved field | platform-facts | Verified |
|--------|---------------|---------------|----------|
| `font_family()` | defaults.font.family | 2.1.1 -- all platforms | Correct |
| `font_size()` | defaults.font.size | 2.1.1 -- logical px | Correct |
| `font_weight()` | defaults.font.weight | 2.1.1 -- CSS 100-900 | Correct |
| `mono_font_family()` | defaults.mono_font.family | 2.1.2 | Correct |
| `mono_font_size()` | defaults.mono_font.size | 2.1.2 | Correct |
| `mono_font_weight()` | defaults.mono_font.weight | 2.1.2 | Correct |
| `line_height_multiplier()` | defaults.line_height | 2.1.1 -- macOS:1.19, Win:1.43, KDE:1.36, GNOME:1.2 | Correct |

Doc comments correctly note that `ResolvedFontSpec.size` is already
in logical pixels.

### 15.5 `to_iced_weight()` -- CORRECT

| CSS weight | Mapped to | CSS Fonts Module Level 4 | Verified |
|-----------|-----------|-------------------------|----------|
| 100 | Thin | Correct | Yes |
| 200 | ExtraLight | Correct | Yes |
| 300 | Light | Correct | Yes |
| 400 | Normal | Correct | Yes |
| 500 | Medium | Correct | Yes |
| 600 | Semibold | Correct (iced spells it "Semibold") | Yes |
| 700 | Bold | Correct | Yes |
| 800 | ExtraBold | Correct | Yes |
| 900 | Black | Correct | Yes |

Rounding boundaries verified:
- 349 -> Light, 350 -> Normal (midpoint rounds up to next band)
- 449 -> Normal, 450 -> Medium (consistent)
- 0 -> Thin, 1000+ -> Black (safe saturation at boundaries)

### 15.6 No mismatches found

No color values, field mappings, palette construction errors, weight
conversion errors, or geometry helper errors detected when
cross-referencing against platform-facts.md.

---

## 16. Showcase: `std::process::exit(1)` at Line 605

`showcase.rs:601-606`:

```rust
None => {
    eprintln!(
        "Fatal: OS theme failed ({e}) and adwaita fallback \
         also failed. Cannot start."
    );
    std::process::exit(1);
}
```

When both OS theme detection AND the adwaita fallback fail inside the
`Default` impl for `State`, the showcase calls `process::exit(1)`.
This terminates without unwinding (no Drop calls, no cleanup).

While `process::exit` is not technically a `panic!()`, it violates the
project's "no runtime panics" rule in spirit (abrupt termination).

**Impact:** Low in practice. Requires a system with no desktop
environment AND a corrupted adwaita preset -- essentially impossible
with bundled data. However, it sets a bad precedent.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Change `State::default()` to a fallible `State::try_new() -> Result<Self, String>` and handle in `main()` | Clean error handling; no abrupt termination | iced's `application()` API expects infallible init; requires restructuring |
| B | Fall back to iced's built-in `Theme::Dark`/`Theme::Light` with a dummy `ResolvedThemeVariant` and display an error banner | App stays functional; error visible to user | Constructing a valid dummy `ResolvedThemeVariant` is complex (30+ required fields) |
| C | Keep `process::exit(1)` but add a clear comment explaining why it is acceptable in this example | Documents the decision; readers understand the tradeoff | Still violates convention |
| D | Use `std::process::ExitCode` from `main()` to propagate the error | Clean exit code; proper Rust pattern | Major restructuring of showcase init flow |

**Recommended: C.** The showcase is an example binary, not a library.
The double-failure case is near-impossible with bundled data. A comment
explaining "this is the only safe fallback when both OS theme and bundled
preset fail" is sufficient. B is the ideal long-term fix but the
complexity of constructing a dummy `ResolvedThemeVariant` makes it
disproportionate effort.

---

## 17. Showcase: Hardcoded Spacing Values in Sidebar

`showcase.rs:1221-1332`: the sidebar uses hardcoded pixel values:

| Line(s) | Value | Correct theme equivalent |
|---------|-------|------------------------|
| 1236, 1242, 1259, 1303 | `.spacing(4)` | `sp.xs` (4px in most themes) |
| 1282 | `.spacing(2)` | `sp.xxs` (2px in most themes) |
| 1323 | `.spacing(8)` | `sp.s` or `sp.m` depending on theme |
| 1298 | `Padding::from(6)` | `sp.s` (6px in KDE) or `sp.xs` (4px in Adwaita) |
| 1324 | `Padding::from(10)` | `sp.m` (12px) or `sp.s` (8px) |
| 1325 | `Length::Fixed(210.0)` | No direct equivalent (layout fixed width) |

The right panel at lines 1367-1369 correctly uses resolved spacing:

```rust
let sp = &state.current_resolved.defaults.spacing;
let tab_padding = Padding::ZERO.left(sp.l).right(sp.l).top(sp.s);
let content_padding = Padding::from(sp.l);
```

The inconsistency means the sidebar does not adapt when switching
between themes with different spacing scales.

**Impact:** Low. The sidebar layout is static and functional. But it
violates the project rule "Never hardcode spacing/padding/sizing."

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Replace hardcoded values with spacing scale references throughout the sidebar | Follows project rules; layout adapts to theme | Must audit each value and choose the right tier |
| B | Keep hardcoded values in sidebar, add comment explaining it is intentional for fixed-width sidebar stability | Explicit decision | Still violates convention; inconsistent with right panel |
| C | Keep as-is | No work | Violates project conventions |

**Recommended: A.** The right panel demonstrates the correct pattern.
Apply it consistently. The fixed sidebar width (210px) can remain
hardcoded since it is a layout constraint, not spacing.

---

## 18. Showcase: Repetitive Extended Palette Section

`showcase.rs:2535-2797` (`view_theme_map()`): the extended palette
visualization repeats the same pattern 5 times for background, primary,
secondary, success, danger, and warning color families. Each is
~25 lines of identical structure:

```rust
let ext_$name = column![
    text("$Name (Extended)").size(16),
    row![
        color_swatch("base.color", extended.$name.base.color),
        color_swatch("base.text", extended.$name.base.text),
        // ... 4 more swatches
    ].spacing(12),
].spacing(8);
```

Three of the six (secondary, success, warning, danger at lines 2650-2707)
do not even have hoverable wrapping, unlike background and primary.
This inconsistency suggests copy-paste drift.

**Impact:** Low. ~150 lines of mechanical repetition. Maintenance
burden if the swatch layout needs to change.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Extract a helper function taking the family name + `iced_core::theme::palette::Secondary`/etc. reference | Eliminates repetition; single place to update | iced's Extended palette families are different types (not a common trait); need a macro or pass individual `Pair` references |
| B | Use a macro to generate the 6 sections | Compact; easy to add new families | Macros harder to read than functions |
| C | Pass `(name, base_color, base_text, weak_color, weak_text, strong_color, strong_text)` tuple to a helper | Works with any Extended family type; no trait needed | Long parameter list (7 values) |
| D | Keep repetition | Each section independently readable | ~150 lines of copy-paste; inconsistent hoverable wrapping |

**Recommended: C.** A helper function taking 7 values is more readable
than 6 copies of 25 lines. Also fix the missing hoverable wrappers on
4 of the 6 sections for consistency.

---

## 19. Showcase: Hardcoded Font Sizes

Throughout the showcase, widget text sizes are hardcoded:

| Pattern | Count | Examples |
|---------|-------|---------|
| `.size(9)` | ~20 | Icon cell labels, swatch hex text |
| `.size(10)` | ~15 | Theme config inspector values |
| `.size(11)` | ~5 | Animation labels, source labels |
| `.size(12)` | ~15 | Section labels, status text |
| `.size(13)` | ~5 | Descriptive text |
| `.size(14)` | ~3 | Button content text |
| `.size(16)` | ~15 | Sub-section headings |
| `.size(18)` | 1 | Title "native-theme" |
| `.size(24)` | ~7 | Section headers |

None of these derive from the resolved theme's text scale
(`resolved.text_scale.caption.size`, `resolved.text_scale.section_heading.size`,
etc.) or font size (`resolved.defaults.font.size`).

**Impact:** Low. The showcase is a demonstration app, and hardcoded
sizes ensure stable screenshots. But it misses the opportunity to
demonstrate text scale usage.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Replace at least the main categories with text scale references: `.size(24)` -> `text_scale.section_heading.size`, `.size(12)` -> `text_scale.caption.size` | Demonstrates text scale usage; adapts to theme | Must map each hardcoded size to the right scale tier; some sizes have no direct equivalent |
| B | Add one dedicated section showing text scale in the Display tab | Demonstrates the feature without changing existing layout | Existing sections still hardcoded |
| C | Keep hardcoded sizes | Stable layout; predictable screenshots | Misses demo opportunity |

**Recommended: B.** Add a text scale demonstration in the Display tab
without disrupting the existing stable layout. This showcases the
feature without risking layout shifts in screenshots.

---

## 20. `to_palette()` Discards Alpha Channel Information

`palette.rs:14-17`:

```rust
pub fn to_color(rgba: Rgba) -> iced_core::Color {
    let [r, g, b, a] = rgba.to_f32_array();
    iced_core::Color { r, g, b, a }
}
```

The conversion itself preserves alpha. However, the `to_palette()`
function at lines 29-39 maps colors that are always fully opaque
(`a = 1.0`) in resolved themes. Some resolved theme colors (notably
`shadow`, `selection_inactive`, `border` with `border_opacity` applied)
may carry meaningful alpha values.

Since these are not mapped through `to_palette()` (they are not in the
6-field Palette), this is not a bug. But if issue 2 is implemented
(overriding more Extended fields), the alpha channel handling is correct
and ready.

**Impact:** None currently. Noted for completeness.

---

## Summary: Priority Order

| # | Issue | Severity | Effort | Recommended Fix |
|---|-------|----------|--------|----------------|
| 1 | `apply_overrides()` dead code / misleading doc comment | High | Low | Restructure to accept 4 Rgba values; call from `to_theme()` |
| 2 | Extended palette missing 3 color family overrides | Medium | Low | Override primary/success/danger `.base` from resolved theme |
| 8 | No integration tests | Medium | Medium | Create `tests/integration.rs` with all-presets smoke test |
| 14 | Test quality: single preset, weak assertions, no dark variant tests in extended.rs | Medium | Medium | Add multi-preset tests, exact-value assertions, dark variant coverage |
| 12 | Many resolved colors have no helper functions | Medium | Low | Add `defaults()` accessor + targeted helpers for common fields |
| 5 | `from_system()` discards `is_dark` flag | Medium | Low | Return triple `(Theme, Resolved, bool)` in next minor version |
| 3 | `from_preset()` uses slug as display name | Low | Trivial | Capture `spec.name` before consuming |
| 4 | `from_preset()` misleading error message | Low | Trivial | Fix message text |
| 6 | `SystemThemeExt` discards resolved variant | Low | Low | Return tuple; schedule with issue 5 |
| 9 | Missing `LinuxDesktop` re-export | Low | Trivial | One-line conditional re-export |
| 10 | SVG colorization missing `stroke="black"` | Low | Trivial | 3 additional replacements |
| 11 | `extended` module unnecessarily public | Low | Trivial | Change to `pub(crate)` |
| 13 | `info`/`warning_foreground` not mapped | Low | Trivial | Document gap; add helper functions |
| 17 | Showcase: hardcoded sidebar spacing | Low | Medium | Replace with spacing scale references |
| 18 | Showcase: repetitive palette section | Low | Low | Extract helper function |
| 19 | Showcase: hardcoded font sizes | Low | Low | Add text scale demo section |
| 16 | Showcase: `process::exit(1)` | Low | Low | Add explanatory comment |
| 7 | `to_theme()` missing `is_dark` param | Low | N/A | Defer until iced needs it (YAGNI) |
| 20 | `to_palette()` alpha handling | None | N/A | No action needed |
