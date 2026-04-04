# v0.5.4 -- native-theme-iced: Deep Critical Analysis

Comprehensive audit of the iced connector crate covering unit tests,
correctness vs platform-facts.md, code quality, API design, and
cross-connector symmetry with the gpui connector.

Files analyzed:
- `connectors/native-theme-iced/Cargo.toml`
- `connectors/native-theme-iced/src/lib.rs` (519 lines)
- `connectors/native-theme-iced/src/extended.rs` (117 lines)
- `connectors/native-theme-iced/src/icons.rs` (615 lines)
- `connectors/native-theme-iced/src/palette.rs` (111 lines)
- `connectors/native-theme-iced/examples/showcase.rs` (2903 lines)
- `native-theme/src/model/mod.rs` (ThemeVariant structure)
- `native-theme/src/model/resolved.rs` (ResolvedThemeVariant)
- `native-theme/src/lib.rs` (public API, re-exports)
- `connectors/native-theme-gpui/src/lib.rs` (reference for symmetry)
- `connectors/native-theme-gpui/src/colors.rs` (reference for color mapping)
- `docs/platform-facts.md` (cross-reference target)

All 57 tests pass (lib.rs:21, extended.rs:4, icons.rs:29, palette.rs:3).
No `unwrap()`/`expect()`/`panic!()` in production code -- all instances
are inside `#[cfg(test)] #[allow(clippy::unwrap_used, clippy::expect_used)]`
blocks or doc comments. The crate enforces this with `#![deny(clippy::unwrap_used)]`
and `#![deny(clippy::expect_used)]` at `lib.rs:74-75`.

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

**Recommended: A.** Override the `.base.text` fields for all families
that have platform-specific foreground colors.

**Correction (issue 29):** The `.base.color` overrides below are
REDUNDANT -- palette auto-generation already sets them correctly via
`to_palette()`. Only the `.base.text` fields need overriding. Also add
`warning` (iced DOES have a Warning family, contrary to issue 13's
original claim). The correct fix is 4 text-only overrides:

```rust
ext.primary.base.text = to_color(accent_foreground);
ext.success.base.text = to_color(success_foreground);
ext.danger.base.text = to_color(danger_foreground);
ext.warning.base.text = to_color(warning_foreground);
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
`spec` is consumed by `into_variant()` at line 140 before the name
is captured.

Verified: `native-theme/src/presets/catppuccin-mocha.toml` has
`name = "Catppuccin Mocha"`, confirming the slug and display name differ.

The gpui connector has the same issue at its `lib.rs:151-156`.

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

The gpui connector has the same incorrect message at its `lib.rs:152-153`.

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

Note: the `currentColor` replacement at line 247 does handle both fill
and stroke correctly, since it replaces the string `"currentColor"`
regardless of which attribute contains it. This gap only affects the
explicit-black-fill branch at lines 250-258.

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

Cross-referencing `ResolvedThemeDefaults` (resolved.rs:83-191) against
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

**Correction (issue 29):** The claim below that iced has no `warning`
family is INCORRECT. Iced `Extended` HAS a `Warning` family with
`base`, `weak`, `strong`. The `warning.base.color` is correct via
palette, but `warning.base.text` defaults to `d.foreground` instead of
`d.warning_foreground`. See issue 29 for the fix.

Iced's Extended palette has no `info` family. The
`warning` base color reaches iced via `palette.warning` -> `d.warning`,
but its foreground pair (`warning_foreground`) is not overridden (see
issue 29). `info` is lost entirely.

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

#### lib.rs -- 21 tests

| # | Test name | What it tests |
|---|-----------|---------------|
| 1 | `to_theme_produces_non_default_theme` | `to_theme()` returns a custom theme, not built-in Light/Dark; palette primary is non-zero |
| 2 | `to_theme_from_preset` | `to_theme()` with catppuccin-mocha light produces background > 0.9 |
| 3 | `border_radius_returns_resolved_value` | `border_radius()` returns positive value |
| 4 | `border_radius_lg_returns_resolved_value` | `border_radius_lg()` returns positive value >= `border_radius()` |
| 5 | `scrollbar_width_returns_resolved_value` | `scrollbar_width()` returns positive value |
| 6 | `button_padding_returns_iced_padding` | `button_padding()` maps vertical to top/bottom and horizontal to left/right; symmetry |
| 7 | `input_padding_returns_iced_padding` | `input_padding()` maps vertical to top/bottom and horizontal to left/right |
| 8 | `font_family_returns_concrete_value` | `font_family()` returns non-empty string |
| 9 | `font_size_returns_concrete_value` | `font_size()` returns positive value |
| 10 | `mono_font_family_returns_concrete_value` | `mono_font_family()` returns non-empty string |
| 11 | `mono_font_size_returns_concrete_value` | `mono_font_size()` returns positive value |
| 12 | `font_weight_returns_concrete_value` | `font_weight()` returns value in 100-900 range |
| 13 | `mono_font_weight_returns_concrete_value` | `mono_font_weight()` returns value in 100-900 range |
| 14 | `line_height_multiplier_returns_concrete_value` | `line_height_multiplier()` returns positive value < 5.0 (a multiplier, not pixels) |
| 15 | `to_iced_weight_standard_weights` | All 9 standard CSS weights (100-900) map to correct iced Weight variants |
| 16 | `to_iced_weight_non_standard_rounds_correctly` | Non-standard weights 350, 450, 550 round to correct band; boundary values 0 and 1000 safe |
| 17 | `from_preset_valid_light` | `from_preset("catppuccin-mocha", false)` succeeds; produces custom theme with populated font family |
| 18 | `from_preset_valid_dark` | `from_preset("catppuccin-mocha", true)` succeeds; produces custom theme |
| 19 | `from_preset_invalid_name` | `from_preset("nonexistent-preset", false)` returns `Err` |
| 20 | `system_theme_ext_to_iced_theme` | `SystemTheme::to_iced_theme()` does not panic (gracefully skips if system unavailable) |
| 21 | `from_system_does_not_panic` | `from_system()` does not panic regardless of success/failure |

#### extended.rs -- 4 tests

| # | Test name | What it tests |
|---|-----------|---------------|
| 1 | `apply_overrides_sets_secondary_base_color` | `apply_overrides()` changes `secondary.base.color` from DARK default to `resolved.button.background` |
| 2 | `apply_overrides_sets_secondary_base_text` | `apply_overrides()` sets `secondary.base.text` to match `resolved.button.foreground` |
| 3 | `apply_overrides_sets_background_weak_color` | `apply_overrides()` sets `background.weak.color` to match `resolved.defaults.surface` |
| 4 | `apply_overrides_sets_background_weak_text` | `apply_overrides()` sets `background.weak.text` to match `resolved.defaults.foreground` |

#### icons.rs -- 29 tests

| # | Test name | What it tests |
|---|-----------|---------------|
| 1 | `to_image_handle_with_rgba_returns_some` | RGBA IconData produces an image handle |
| 2 | `to_image_handle_with_svg_returns_none` | SVG IconData returns None for image handle |
| 3 | `to_svg_handle_with_svg_returns_some` | SVG IconData produces an SVG handle |
| 4 | `to_svg_handle_with_rgba_returns_none` | RGBA IconData returns None for SVG handle |
| 5 | `to_svg_handle_colored_replaces_current_color` | Colorizing SVG with `currentColor` replaces with hex; no `currentColor` remains |
| 6 | `to_svg_handle_colored_injects_fill_for_material_style` | Material-style SVG (no fill attr) gets fill injected into root `<svg>` tag |
| 7 | `to_svg_handle_colored_with_rgba_returns_none` | RGBA data with color parameter still returns None for SVG handle |
| 8 | `custom_icon_to_image_handle_with_svg_provider_returns_none` | SVG-only provider returns None for image handle (type mismatch) |
| 9 | `custom_icon_to_svg_handle_with_svg_provider_returns_some` | SVG-only provider returns Some for SVG handle |
| 10 | `custom_icon_to_svg_handle_with_color_returns_some` | Custom SVG icon with color parameter produces handle |
| 11 | `custom_icon_to_image_handle_with_empty_provider_returns_none` | Empty provider returns None for image handle |
| 12 | `custom_icon_to_svg_handle_with_empty_provider_returns_none` | Empty provider returns None for SVG handle |
| 13 | `custom_icon_helpers_accept_dyn_provider` | Custom icon helpers work with `Box<dyn IconProvider>` (trait object) |
| 14 | `colorize_svg_preserves_existing_fill` | SVG with `fill="red"` is not overwritten by colorization |
| 15 | `animated_frames_returns_handles` | 2-frame SVG animation produces 2 handles with correct frame_duration_ms |
| 16 | `animated_frames_transform_returns_none` | Transform (spin) variant returns None from frame converter |
| 17 | `animated_frames_empty_returns_none` | Empty frame list returns None |
| 18 | `animated_frames_rgba_only_returns_none` | All-RGBA frame list returns None (only SVG handled) |
| 19 | `spin_rotation_zero_elapsed` | Zero elapsed time produces 0.0 radians |
| 20 | `spin_rotation_half` | 500ms of 1000ms cycle produces PI radians |
| 21 | `spin_rotation_full_wraps` | 1000ms of 1000ms cycle wraps to ~0.0 radians |
| 22 | `spin_rotation_zero_duration_returns_zero` | Zero duration_ms returns 0.0 (not NaN) |
| 23 | `colorize_replaces_explicit_black_fill` | `fill="black"` is replaced by hex color |
| 24 | `into_image_handle_with_rgba` | Consuming version converts RGBA data |
| 25 | `into_svg_handle_with_svg` | Consuming version converts SVG data |
| 26 | `to_svg_handle_with_rgba_returns_none_no_color` | RGBA data without color returns None (duplicate coverage with #4) |
| 27 | `colorize_self_closing_svg_produces_valid_xml` | Self-closing `<svg/>` gets fill injected before `/`, producing valid XML |
| 28 | `colorize_non_utf8_returns_original_bytes` | Non-UTF-8 SVG bytes pass through unmodified |
| 29 | `animated_frames_with_color_colorizes_frames` | Animated frames with color parameter produce handles (colorization path exercised) |

#### palette.rs -- 3 tests

| # | Test name | What it tests |
|---|-----------|---------------|
| 1 | `to_color_converts_rgba` | `to_color(Rgba::rgb(255, 0, 0))` produces `Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }` (exact value check) |
| 2 | `to_palette_maps_all_fields_from_resolved` | Light catppuccin-mocha: background light (>0.9), text dark (<0.3), primary non-zero |
| 3 | `to_palette_dark_variant_has_dark_background` | Dark catppuccin-mocha: background dark (<0.3) |

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
| `to_theme()` extended palette overrides with dark variant | Medium | All extended.rs tests use `make_resolved()` which always passes `is_dark=false` (light only) |
| `apply_overrides()` with a dark variant input | Medium | All 4 tests use `make_resolved()` which always passes `is_dark=false` |

### 14.4 Tests that only test one preset

All test helper functions use only `catppuccin-mocha`:

- `lib.rs:306-313 make_resolved(is_dark)` -- always catppuccin-mocha
- `extended.rs:46-53 make_resolved()` -- always catppuccin-mocha, always `is_dark=false`
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

### 14.6 Near-duplicate test

`to_svg_handle_with_rgba_returns_none_no_color` (icons.rs:565-572)
is functionally identical to `to_svg_handle_with_rgba_returns_none`
(icons.rs:314-321). Both pass RGBA data and assert `to_svg_handle`
returns `None`. The only difference is #4 passes `None` for color
while #26 also passes `None` for color. They test the same code path
with the same inputs. Not harmful but adds no coverage.

### 14.7 No redundant or bloated tests (aside from 14.6)

All other 56 tests serve a clear purpose.

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

`showcase.rs:600-606`:

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

The sidebar at `showcase.rs:1221-1332` uses hardcoded pixel values
while the right panel at lines 1366-1369 correctly uses resolved
spacing (`sp.l`, `sp.s`, `sp.xs`).

Hardcoded sidebar values:

| Line(s) | Value | Correct theme equivalent |
|---------|-------|------------------------|
| 1235, 1247, 1259, 1303 | `.spacing(4)` | `sp.xs` (4px in most themes) |
| 1281 | `.spacing(2)` | `sp.xxs` (2px in most themes) |
| 1322 | `.spacing(8)` | `sp.s` or `sp.m` depending on theme |
| 1298 | `Padding::from(6)` | `sp.s` (6px in KDE) or `sp.xs` (4px in Adwaita) |
| 1323 | `Padding::from(10)` | `sp.m` (12px) or `sp.s` (8px) |
| 1324 | `Length::Fixed(210.0)` | No direct equivalent (layout fixed width) |

Correctly themed right panel (lines 1366-1369):

```rust
let sp = &state.current_resolved.defaults.spacing;
let tab_padding = Padding::ZERO.left(sp.l).right(sp.l).top(sp.s);
let content_padding = Padding::from(sp.l);
```

Additional hardcoded spacing outside the sidebar:

| Line(s) | Value | Context |
|---------|-------|---------|
| 1347 | `Padding::from([4, 10])` | Tab bar button padding |
| 1351 | `.spacing(4)` | Tab bar button row spacing |
| 1380 | `Padding::from([4, 8])` | Error banner padding |

The inconsistency means the sidebar and tab bar do not adapt when
switching between themes with different spacing scales.

**Impact:** Low. The sidebar layout is static and functional. But it
violates the project rule "Never hardcode spacing/padding/sizing."

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Replace hardcoded values with spacing scale references throughout the sidebar and tab bar | Follows project rules; layout adapts to theme | Must audit each value and choose the right tier |
| B | Keep hardcoded values in sidebar, add comment explaining it is intentional for fixed-width sidebar stability | Explicit decision | Still violates convention; inconsistent with right panel |
| C | Keep as-is | No work | Violates project conventions |

**Recommended: A.** The right panel demonstrates the correct pattern.
Apply it consistently. The fixed sidebar width (210px) can remain
hardcoded since it is a layout constraint, not spacing.

---

## 18. Showcase: Repetitive Extended Palette Section

`showcase.rs:2535-2797` (`view_theme_map()`): the extended palette
visualization repeats the same pattern for 7 color displays. Three
(`base_palette`, `ext_background`, `ext_primary`) are wrapped in
`hoverable()` with `widget_tooltip()` for Widget Info integration.
Four (`ext_secondary`, `ext_success`, `ext_warning`, `ext_danger`
at lines 2650-2707) are plain `column![]` without hoverable wrapping.

Each section is ~15-25 lines of identical structure:

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

**Impact:** Low. ~150 lines of mechanical repetition. Maintenance
burden if the swatch layout needs to change. The missing hoverable
wrapping on 4 of 7 sections is an inconsistency -- hovering those
sections shows no widget info.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Extract a helper function taking the family name + `iced_core::theme::palette::Secondary`/etc. reference | Eliminates repetition; single place to update | iced's Extended palette families are different types (not a common trait); need a macro or pass individual `Pair` references |
| B | Use a macro to generate the 7 sections | Compact; easy to add new families | Macros harder to read than functions |
| C | Pass `(name, base_color, base_text, weak_color, weak_text, strong_color, strong_text)` tuple to a helper | Works with any Extended family type; no trait needed | Long parameter list (7 values) |
| D | Keep repetition | Each section independently readable | ~150 lines of copy-paste; inconsistent hoverable wrapping |

**Recommended: C.** A helper function taking 7 values is more readable
than 7 copies of 15-25 lines. Also add hoverable wrapping to the 4
sections that lack it for consistency.

---

## 19. Showcase: Hardcoded Font Sizes

Throughout the showcase, widget text sizes are hardcoded:

| Pattern | Count | Examples |
|---------|-------|---------|
| `.size(8)` | ~1 | Icon cell source label |
| `.size(9)` | ~3 | Icon cell labels, swatch hex text |
| `.size(10)` | ~7 | Theme config inspector values, widget info text |
| `.size(11)` | ~5 | Animation labels, source labels |
| `.size(12)` | ~15 | Section labels, status text, tab button labels |
| `.size(13)` | ~5 | Descriptive text, icon set info |
| `.size(14)` | ~7 | Button content text, value display text, container labels |
| `.size(16)` | ~15 | Sub-section headings |
| `.size(18)` | 1 | Title "native-theme" |
| `.size(20)` | 1 | "Animated Icons" heading |
| `.size(24)` | ~2 | Section headers via `section_header()` |

None of these derive from the resolved theme's text scale
(`resolved.text_scale.caption`, `resolved.text_scale.section_heading`,
`resolved.text_scale.dialog_title`, `resolved.text_scale.display`)
or font size (`resolved.defaults.font.size`).

The `ResolvedTextScale` has 4 roles, each with `size`, `weight`, and
`line_height`:
- `caption` -- small label text (e.g., 11px)
- `section_heading` -- section heading text (e.g., 14px)
- `dialog_title` -- dialog title text (e.g., 16px)
- `display` -- large hero text (e.g., 24px)

**Impact:** Low. The showcase is a demonstration app, and hardcoded
sizes ensure stable screenshots. But it misses the opportunity to
demonstrate text scale usage.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Replace at least the main categories with text scale references: `.size(24)` -> `text_scale.display.size`, `.size(16)` -> `text_scale.dialog_title.size`, `.size(12)` -> `text_scale.caption.size` | Demonstrates text scale usage; adapts to theme | Must map each hardcoded size to the right scale tier; some sizes have no direct equivalent |
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

## 21. Showcase Error Banner Uses Hardcoded Red

`showcase.rs:1377`: the error banner color is `Color::from_rgb(0.9, 0.1, 0.1)`
-- a hardcoded red. This could use `palette.danger` from the active theme,
consistent with issues 17 and 19 about deriving values from the theme.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Use `palette.danger` from the theme | Consistent with theme; follows same principle as issues 17/19 | Must propagate palette to error display |
| B | Keep hardcoded | Always visible red regardless of theme | Inconsistent with theme-awareness goal |

**Recommended:** A. Same principle as issues 17 and 19 -- derive from theme.

---

## 22. Six of Seven Doc-tests Are `ignore`d

Six doc-tests are `ignore`d (only the manual-path example at `lib.rs:25-32`
runs). The `from_preset`, `from_system`, font configuration, `animated_frames`,
and `spin_rotation` doc examples are all `ignore`d. This is expected for
examples requiring a display system or runtime state, but means doc examples
are not CI-validated.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Change applicable doc-tests from `ignore` to `no_run` | Compile-checked without execution; catches API drift | May need mock setup |
| B | Keep `ignore`d | No change | Doc examples can silently rot |

**Recommended:** A where feasible. `no_run` at least validates that doc
examples compile.

---

## 23. Showcase Uses `resolve()` Instead of `resolve_all()`

**File:** `examples/showcase.rs:186,787`

Two code paths call `variant.resolve()` instead of `variant.resolve_all()`.
The skipped `resolve_platform_defaults()` fills `icon_theme` from the system
when TOML does not specify one. Without it, custom presets missing
`icon_theme` would fail validation.

The gpui showcase has the same bug at its `showcase.rs:1636`.

#### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Replace `variant.resolve()` with `variant.resolve_all()` | Correct behavior; one-word change | None |
| B | Replace entire manual pipeline with `variant.clone().into_resolved()?` | Simplest | Requires clone |

**Recommended:** A. Apply same fix to gpui showcase.

---

## 24. `colorize_monochrome_svg()` -- `fill="none"` Blocks All Colorization

**File:** `icons.rs:262-280`

When root tag has `fill="none"` (common in stroke-based SVGs), the fill
injection branch skips. Combined with no `stroke="black"` replacement
(issue 10), a stroke-only SVG with explicit `stroke="#000"` receives
no colorization at all.

**Recommended:** Move explicit-black-stroke replacements (issue 10's fix)
before the `fill=` check on the root tag, so they run regardless.

---

## 25. Re-export Asymmetry: `Result`/`Rgba` in Iced but Not Gpui

**File:** `lib.rs:82-85` vs gpui `lib.rs:73-76`

Iced re-exports `Result` and `Rgba`. Gpui does not. These appear in public
API signatures. Reverse of existing issue 9 (gpui exports `LinuxDesktop`
but iced doesn't).

**Recommended:** Add `Result` and `Rgba` to gpui's re-exports.

---

## 26. Showcase `color_swatch()` Hardcoded Border Values

**File:** `examples/showcase.rs:2818-2824`

Three hardcoded values: `Color::from_rgba(0.5, 0.5, 0.5, 0.3)` (border
color), `width: 1.0`, `radius: 4.0`. Should derive from
`resolved.defaults.border`, `resolved.defaults.frame_width`, and
`border_radius()`. Called ~36 times in Theme Map tab.

**Recommended:** Pass radius and border color as parameters.

---

## 27. Tab Content Areas: ~65 Hardcoded `.spacing()` Calls (Extends Issue 17)

**File:** `examples/showcase.rs:1490-2797`

Issue 17 identified sidebar (~12) and tab bar (~3) hardcoded values. The
tab content functions contain ~65 additional hardcoded `.spacing()` calls
plus 4 `.gap(5)` instances at lines 2198,2206,2214,2222. Issue 17's effort
should be **High** not Medium.

---

## 28. Missing Tests for `fill="#000"` and `fill="#000000"` Patterns

**File:** `icons.rs:253-255`

Code handles three fill patterns (`fill="black"`, `fill="#000000"`,
`fill="#000"`) but only `fill="black"` has a dedicated test (line 537).

**Recommended:** Add two trivial tests for hex patterns.

---

## Correction: Issue 14.3/14.5 -- `extended.rs` Test Setup Mismatch

The `make_resolved()` helper uses `is_dark=false` (catppuccin-mocha light),
but Extended palette is generated from `Palette::DARK` (hardcoded at
`extended.rs:43`). The "changed from original" assertion is trivially
satisfied because light theme colors always differ from DARK palette
defaults. The test should assert exact expected values, not just "different."

---

## 29. Issue #2/#13 Correction: `warning.base.text` Also Needs Override

**File:** `lib.rs:116-123`

Issue #13 falsely claims iced has no `warning` family. Iced `Extended` HAS
a `Warning` family. `warning.base.text` gets `d.foreground` from
auto-generation, should be `d.warning_foreground`.

Issue #2's fix should be 4 `.base.text` overrides (not 8 lines):
```rust
ext.primary.base.text   = to_color(accent_foreground);
ext.success.base.text   = to_color(success_foreground);
ext.danger.base.text    = to_color(danger_foreground);
ext.warning.base.text   = to_color(warning_foreground);
```

Note: `.base.color` overrides are redundant -- already correct via palette.

---

## 30. Showcase Drops Alpha Channel in Resolved Color Swatches

**File:** `examples/showcase.rs:2760-2761`

`Color::from_rgb(cr, cg, cb)` discards alpha. Colors with meaningful alpha
(e.g., `shadow` ~0.3) display fully opaque.

**Recommended:** Use `Color::from_rgba(cr, cg, cb, a)`.

---

## 34. No Tripwire Test for Iced Palette/Extended Field Count

**File:** `lib.rs` tests

The gpui connector has a `theme_color_field_count_tripwire` test. The iced
connector has no equivalent. Upstream palette additions go undetected.

**Recommended:** Add `size_of::<Palette>() / size_of::<Color>() == 6`.

---

## 35. Showcase `_ => false` Catch-All in Icon Set Availability

**File:** `examples/showcase.rs:301`

`_ => false` silently treats any future `IconSet` variant as unavailable,
defeating compiler exhaustiveness checking.

**Recommended:** Use explicit match arms for all 5 `IconSet` variants.

---

## 36. Missing `DialogButtonOrder` Re-export (Cross-Crate CC-3)

`ResolvedThemeVariant` carries `dialog.button_order: DialogButtonOrder`
but `DialogButtonOrder` is not in the re-export block at `lib.rs:82-85`.
Users must depend on `native-theme` directly to name the type. Neither
connector re-exports it. The gpui connector tracks this as issue #48.

**Recommended:** Add `DialogButtonOrder` to re-export block.

---

## 37. `to_theme()` Bare `#[must_use]` (Cross-Crate CC-7)

**File:** `lib.rs:102`

Iced `to_theme()` uses bare `#[must_use]` without a message. The gpui
connector's identical function has `#[must_use = "this returns the theme; it does not apply it"]`. All other entry points in both connectors include
the message.

**Recommended:** Add message string for consistency.

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
| 21 | Showcase error banner hardcoded red | Low | Trivial | Use `palette.danger` |
| 22 | Six of seven doc-tests `ignore`d | Low | Trivial | Change to `no_run` where feasible |
| 23 | Showcase uses `resolve()` not `resolve_all()` | Medium | Trivial | Replace with `resolve_all()` |
| 24 | `fill="none"` blocks all SVG colorization | Low | Low | Restructure replacement order |
| 25 | Re-export asymmetry: `Result`/`Rgba` vs gpui | Low | Trivial | Add to gpui re-exports |
| 26 | `color_swatch()` hardcoded border values | Low | Low | Derive from resolved theme |
| 27 | ~65 hardcoded spacing in tab content (extends #17) | Low | High | Apply spacing scale |
| 28 | Missing tests for fill hex patterns | Low | Trivial | Add 2 pattern tests |
| 29 | Issue #2/#13 correction: warning.base.text needs override | Medium | Trivial | Add 4th .base.text override |
| 30 | Showcase drops alpha in resolved color swatches | Low | Trivial | Use `from_rgba` |
| 34 | No Palette/Extended field count tripwire test | Low | Trivial | Add size_of assertion |
| 35 | Showcase `_ => false` defeats exhaustiveness | Very Low | Trivial | Explicit match arms |
| 36 | Missing `DialogButtonOrder` re-export (CC-3) | Low | Trivial | Add to re-export block |
| 37 | `to_theme()` bare `#[must_use]` (CC-7) | Very Low | Trivial | Add message string |

---

## New Findings: Second-Pass Deep Audit

Second-pass audit of all 4 source files, the showcase example, all 57
tests, the core model types, and platform-facts.md. Cross-referenced
every mapping, analyzed every test assertion, and checked for edge
cases. Issue numbering continues from the existing document.

---

### 38. `colorize_monochrome_svg()` Replaces `currentColor` Globally Including Non-Color Attributes

**Category:** mapping-bug
**Severity:** low
**File(s):** `connectors/native-theme-iced/src/icons.rs:246-248`

**Problem:** The `currentColor` replacement at line 247 uses
`svg_str.replace("currentColor", &hex)` which performs a global string
replacement. This is correct for `fill="currentColor"` and
`stroke="currentColor"`, but SVG attributes can reference
`currentColor` in other properties too -- for example,
`stop-color="currentColor"` in gradients, `flood-color="currentColor"`
in filters, or `lighting-color="currentColor"`. For monochrome icons
this is actually the desired behavior (recolor everything), so this is
not a bug for the intended use case (Material, Lucide). However, if a
user passes a *multi-color* SVG that happens to use `currentColor` in
one gradient stop alongside explicit colors in another, the global
replacement would recolor the `currentColor` stop while leaving
explicit colors intact, potentially producing unexpected results.

The doc comment at line 223 correctly says "for **monochrome** SVG
icon" and the public `to_svg_handle()` doc at line 48 correctly says
"Pass `None` for multi-color system icons to preserve their native
palette." So the API contract is clear. But the `currentColor` branch
short-circuits before the fill-specific branch -- if a multi-color SVG
accidentally contains one `currentColor` reference alongside explicit
colors, all `currentColor` instances get replaced and the function
returns early without touching the explicit colors.

**Solution Options:**
| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | No code change; add a warning to the `colorize_monochrome_svg` doc comment noting that `currentColor` replacement is global | Documents the behavior explicitly | No code change |
| B | Only replace `currentColor` inside `fill=` and `stroke=` attribute values | More precise; avoids replacing gradient/filter references | More complex regex or parsing; over-engineering for monochrome icons |

**Best Solution:** A. The function is internal (`fn`, not `pub fn`) and
its doc comment already says "monochrome SVG icon." Adding a one-line
note clarifying that `currentColor` replacement is global (not
attribute-scoped) is sufficient. The intended usage pattern (users pass
`None` for multi-color icons) prevents the issue in practice.

---

### 39. `from_system()` Moves `sys.dark`/`sys.light` Without Selecting by `is_dark`

**Category:** api-misuse
**Severity:** low
**File(s):** `connectors/native-theme-iced/src/lib.rs:156-158`

**Problem:** The `from_system()` function at line 158 uses
`if sys.is_dark { sys.dark } else { sys.light }` to select the
variant. This moves the selected `ResolvedThemeVariant` out of the
`SystemTheme`, consuming it. The non-selected variant is dropped.

Meanwhile, `SystemThemeExt::to_iced_theme()` at line 171 calls
`self.active()` which returns a reference and does not consume
anything. The two entry points behave differently:
- `from_system()` moves and consumes the `SystemTheme` (returning
  owned `ResolvedThemeVariant`)
- `to_iced_theme()` borrows the `SystemTheme` (discarding the
  `ResolvedThemeVariant`)

This is not a bug -- `from_system()` returns the resolved variant so
callers have it for metrics, while `to_iced_theme()` discards it
(issue 6 already covers that). But there is a subtle asymmetry: if a
caller needs both variants (e.g., to support live dark/light switching
without re-reading the system theme), `from_system()` drops the
inactive variant. The caller must hold onto the `SystemTheme` directly
instead.

This is already partially covered by issue 5 (discards `is_dark` flag)
and issue 6 (discards resolved variant). No separate fix needed beyond
those.

**Solution Options:** Subsumed by issues 5 and 6.

**Best Solution:** No additional action. Issues 5 and 6 already cover
the necessary API improvements. Noting for completeness.

---

### 40. `into_image_handle()` and `into_svg_handle()` Have No Tests for Wrong-Variant Input

**Category:** test-gap
**Severity:** low
**File(s):** `connectors/native-theme-iced/src/icons.rs:549-562`

**Problem:** The consuming versions `into_image_handle()` and
`into_svg_handle()` have tests for the happy path (RGBA for image,
SVG for svg) but no tests for the rejection path:
- `into_image_handle(IconData::Svg(...))` -- should return `None`
- `into_svg_handle(IconData::Rgba { .. }, None)` -- should return `None`

The borrowing versions (`to_image_handle`, `to_svg_handle`) DO have
both happy and rejection tests. The consuming versions rely on the
same `match` logic, so they are almost certainly correct, but the
asymmetric test coverage means a future refactor that changes only the
consuming path could introduce a regression undetected.

**Solution Options:**
| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add 2 tests: `into_image_handle_with_svg_returns_none` and `into_svg_handle_with_rgba_returns_none` | Symmetric coverage with borrowing variants; trivial to write | 2 more tests |
| B | No change | The borrowing tests cover the logic | Asymmetric coverage |

**Best Solution:** A. Two trivial tests for completeness.

---

### 41. Showcase `build_animation_caches()` Only Caches One Icon Set's Animations

**Category:** styling-gap
**Severity:** low
**File(s):** `connectors/native-theme-iced/examples/showcase.rs:446-505`

**Problem:** `build_animation_caches()` takes a single `IconSet` and
caches animations only for that set. It calls `loading_indicator(icon_set)`
which returns the loading indicator for the given set. The `set_name`
variable at line 461 is used as a label in the UI.

However, when the user switches icon sets via the pick list (line
1174-1192), the animation caches are rebuilt from scratch for the new
set, discarding the old caches. This means the animated icons section
shows only the current set's loading indicator, not a comparison of
all sets' animations.

This is an intentional design choice (show the active set's animation),
not a bug. The gpui showcase likely does the same. Noting only because
the function name `build_animation_caches` (plural) suggests it builds
caches for multiple sets, but it only handles one.

**Solution Options:** No action needed. This is a naming observation,
not a functional issue.

**Best Solution:** No change. The function correctly builds caches for
the active icon set. The plural "caches" refers to the multiple cache
data structures (frames, spins, statics), not multiple icon sets.

---

### 42. No Test That `to_color()` Preserves Alpha for Non-Opaque Input

**Category:** test-gap
**Severity:** low
**File(s):** `connectors/native-theme-iced/src/palette.rs:57-71`

**Problem:** The single `to_color` test at `palette.rs:57-71` uses
`Rgba::rgb(255, 0, 0)` which produces `a = 1.0`. There is no test
that verifies alpha preservation for non-opaque colors (e.g.,
`Rgba::new(128, 128, 128, 128)` should produce `a ~= 0.502`).

Issue 20 in the existing document notes that the alpha channel is
preserved by `to_color()` but all palette colors are fully opaque.
However, `to_color()` is a public function documented as useful "for
power users who need to map arbitrary `ResolvedThemeVariant` fields to
iced colors" (line 11-12). Some resolved fields carry meaningful alpha
(e.g., `shadow` at ~0.3 opacity). A test verifying alpha round-trip
would guard against future regressions in the conversion.

**Solution Options:**
| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add one test: `to_color_preserves_alpha` with a non-1.0 alpha input | Guards the documented alpha-preserving behavior | 1 more test |
| B | No change | `to_color` is trivial; unlikely to regress | Alpha path untested |

**Best Solution:** A. One trivial test:
```rust
#[test]
fn to_color_preserves_alpha() {
    let result = to_color(Rgba::new(0, 0, 0, 128));
    assert!((result.a - 128.0 / 255.0).abs() < 0.01);
}
```

---

### 43. Showcase `color_to_hex()` Truncates Alpha (Inconsistent With `colorize_monochrome_svg`)

**Category:** styling-gap
**Severity:** very low
**File(s):** `connectors/native-theme-iced/examples/showcase.rs:2799-2804`

**Problem:** The showcase's `color_to_hex()` function converts an iced
`Color` to a 6-digit hex string (`#rrggbb`), discarding the alpha
channel. This is used in color swatch labels throughout the Theme Map
tab.

This is closely related to issue 30 (which notes that `from_rgb` at
line 2761 discards alpha when constructing the swatch color itself).
However, even after issue 30 is fixed (using `from_rgba`), the hex
label would still show only 6 digits because `color_to_hex` only
formats RGB.

After issue 30 is applied, swatches for colors like `shadow` (which
has ~0.3 alpha) would render with correct semi-transparent appearance
but display a misleading hex label like `#000000` instead of
`#00000040` or `rgba(0,0,0,0.24)`.

**Solution Options:**
| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Extend `color_to_hex()` to include alpha when `a < 1.0` as `#rrggbbaa` | Complete color information in labels | 8-char hex less familiar to some users |
| B | Show alpha as a separate annotation: `#000000 @24%` | Clear separation of RGB and alpha | Custom format |
| C | No change | Simpler labels | Incomplete color info after issue 30 is fixed |

**Best Solution:** A. Emit 8-digit hex (`#rrggbbaa`) only when alpha
is not 1.0, 6-digit hex otherwise. Consistent with CSS Color Level 4
8-digit hex notation. Should be done when issue 30 is implemented.

---

### 44. `from_preset()` Light/Dark Selection Inconsistency With `catppuccin-mocha`

**Category:** test-gap
**Severity:** medium
**File(s):** `connectors/native-theme-iced/src/lib.rs:486-498`

**Problem:** Test `from_preset_valid_light` at line 487 calls
`from_preset("catppuccin-mocha", false)`. This requests the light
variant of catppuccin-mocha. The test passes because catppuccin-mocha
has both light and dark variants (the light variant is the
auto-generated inverse).

However, the test name says "valid_light" and the comment at line 491
says "Should produce a valid custom theme (not Light or Dark built-in)"
-- it then asserts `assert_ne!(theme, Theme::Light)`. This assertion
is trivially true because any custom theme with `Theme::custom_with_fn`
is never `==` to the built-in `Theme::Light` variant, regardless of
whether the palette mapping is correct.

Similarly, `from_preset_valid_dark` at line 496 asserts
`assert_ne!(theme, Theme::Dark)`, which is equally trivially true.

These tests verify that `from_preset()` succeeds and returns a custom
theme, but they do not verify that the returned theme actually
corresponds to the requested light/dark variant. A test could verify
that `from_preset("catppuccin-mocha", false)` produces a light
background (>0.9) and `from_preset("catppuccin-mocha", true)` produces
a dark background (<0.3).

This is partially covered by existing issue 14.5 (weak assertions) and
14.3 (missing test scenarios). However, the specific concern that
`from_preset` tests do not verify variant correctness (light vs dark)
is not explicitly called out.

**Solution Options:**
| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add background luminance assertions to both tests: light should be >0.9, dark should be <0.3 | Verifies variant selection correctness | Fragile if theme adjusts background slightly |
| B | Verify specific resolved field (e.g., `resolved.defaults.background`) matches known catppuccin-mocha hex | Exact verification | Couples test to preset values |

**Best Solution:** A. Add one assertion per test. The light test
already has this pattern in `to_theme_from_preset` (line 340:
`palette.background.r > 0.9`), so applying it to `from_preset_valid_light`
and adding `palette.background.r < 0.3` to `from_preset_valid_dark`
is consistent.

---

### 45. `input_padding()` Test Does Not Assert Top/Bottom Symmetry

**Category:** test-gap
**Severity:** low
**File(s):** `connectors/native-theme-iced/src/lib.rs:384-393`

**Problem:** The `button_padding_returns_iced_padding` test at line
371-382 has four assertions including symmetry checks:
```rust
assert_eq!(pad.top, pad.bottom, "top and bottom should be equal");
assert_eq!(pad.left, pad.right, "left and right should be equal");
```

The `input_padding_returns_iced_padding` test at line 384-393 only
asserts `pad.top > 0.0` and `pad.right > 0.0`, without the symmetry
checks. Since `input_padding()` uses the same `Padding::from([v, h])`
pattern as `button_padding()`, the symmetry is guaranteed by iced's
`Padding::from` implementation. But the asymmetric test coverage means
a future change to `input_padding()` that breaks symmetry (e.g.,
switching to per-side padding for asymmetric input fields like WinUI3
TextBox with 5px top / 6px bottom) would not be caught.

**Solution Options:**
| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add symmetry assertions to `input_padding` test matching `button_padding` test | Consistent test quality between the two helpers | 2 more assertions |
| B | No change | Symmetry is guaranteed by `Padding::from` | Inconsistent test rigor |

**Best Solution:** A. Two additional assertions for consistency:
```rust
assert_eq!(pad.top, pad.bottom, "top and bottom should be equal");
assert_eq!(pad.left, pad.right, "left and right should be equal");
```

---

### Summary: Second-Pass New Findings

| # | Issue | Severity | Effort | Recommended Fix |
|---|-------|----------|--------|----------------|
| 38 | `currentColor` replacement is global (non-attribute-scoped) | Low | Trivial | Add doc comment note |
| 39 | `from_system()` variant asymmetry | Low | N/A | Subsumed by issues 5+6 |
| 40 | `into_image_handle`/`into_svg_handle` missing rejection-path tests | Low | Trivial | Add 2 tests |
| 41 | `build_animation_caches()` naming suggestion | Very Low | N/A | No action needed |
| 42 | `to_color()` has no alpha preservation test | Low | Trivial | Add 1 test |
| 43 | `color_to_hex()` truncates alpha (complements issue 30) | Very Low | Trivial | Extend to 8-digit hex when alpha < 1.0 |
| 44 | `from_preset` tests do not verify light/dark variant correctness | Medium | Trivial | Add background luminance assertions |
| 45 | `input_padding()` test missing symmetry assertions | Low | Trivial | Add 2 assertions |

---

## New Findings: Third-Pass Deep Audit

Third-pass audit of all 4 source files, the showcase example, all 57
tests, the gpui connector for symmetry, and `Rgba::to_f32_array()` for
color precision. Focused on test assertion correctness, SVG edge cases,
showcase state consistency, and cross-connector gaps missed by passes 1
and 2. Issue numbering continues from the existing document.

---

### 46. `color_approx_eq` Epsilon Is 250x Too Loose for Direct Conversions

**Category:** test-quality
**Severity:** medium
**File(s):** `connectors/native-theme-iced/src/palette.rs:49-53`,
`connectors/native-theme-iced/src/extended.rs:34-38`

**Problem:** Both test modules define `color_approx_eq` with an epsilon
of 0.01 per channel. The conversion path is `u8 -> f32 / 255.0 ->
iced_core::Color { f32 }` which is a single floating-point division.
The maximum error from f32 representation for any u8 value divided by
255.0 is approximately 3e-8 (verified empirically for all 256 values).

An epsilon of 0.01 in the 0.0..1.0 range corresponds to 2.55 u8 steps.
This means a color could be off by 2 sRGB values per channel (e.g., the
test expects `#ff0000` but would also pass with `#fd0202`) and the test
would still pass. Since the `to_color()` path is a direct lossless
conversion with no intermediate color space transforms, the appropriate
epsilon is approximately 1e-6 (or use exact equality).

The `to_color_converts_rgba` test at `palette.rs:57-71` uses
`color_approx_eq` to verify that `Rgba::rgb(255, 0, 0)` maps to
`Color { 1.0, 0.0, 0.0, 1.0 }`. This conversion is mathematically
exact (255/255.0 = 1.0 in f32), so the tolerance hides nothing here.
But the same helper is used in `extended.rs` tests where the expected
value is computed through the same `to_color()` call, meaning the
"expected" and "actual" go through the identical path -- the comparison
is always exact. The loose epsilon gives false confidence that the
tests would catch a real off-by-one bug when they would not.

**Solution Options:**
| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Tighten epsilon to 1e-6 (or use `assert_eq!` directly since both sides go through the same `to_color()` path) | Catches real off-by-one bugs; accurate for the conversion's actual precision | If a future code path introduces legitimate rounding (e.g., color space conversion), tests would need updating |
| B | No change | No test disruption | Epsilon tolerates 2-step errors that should be zero |

**Best Solution:** A. For the `palette.rs` test that checks a known
constant, use `assert_eq!` directly (the conversion is exact for 0
and 255). For `extended.rs` tests where both sides use `to_color()`,
either use `assert_eq!` or tighten to `1e-6`.

---

### 47. `colorize_monochrome_svg` Incorrectly Handles SVGs with Multiple `<svg>` Elements

**Category:** edge-case
**Severity:** very low
**File(s):** `connectors/native-theme-iced/src/icons.rs:262-280`

**Problem:** The root `<svg>` tag injection branch at line 262 uses
`svg_str.find("<svg")` which finds the FIRST `<svg` substring. In a
valid SVG, this is always the root element. However, SVGs can contain
nested `<svg>` elements (embedded sub-documents) or `<svg` appearing
inside comments (`<!-- <svg ... -->`), CDATA sections, or text content.

For the intended use case (Material/Lucide monochrome icons from bundled
sets), these edge cases do not arise -- bundled icons have a single root
`<svg>` with child `<path>` elements. However, if a user passes a
complex system icon with nested SVG elements, the fill injection would
correctly target the root `<svg>` tag (since `find` returns the first
occurrence, which is the root in valid XML).

The only true edge case is `<svg` appearing inside an XML comment before
the actual root `<svg>` tag, e.g.:
```xml
<!-- <svg removed for debugging -->
<svg xmlns="..."><path d="..."/></svg>
```
Here, `find("<svg")` would match the comment, and the `find('>')`
would locate the comment's `>`, injecting `fill=` into a comment. The
result would be a malformed SVG. This is extremely unlikely in practice.

No test covers this, and no fix is needed. Documenting for completeness.

**Best Solution:** No action. The edge case requires malformed/unusual
input that bundled icon sets never produce. The gpui connector has the
identical behavior.

---

### 48. Showcase `button_press_count` Can Overflow `u32`

**Category:** edge-case
**Severity:** very low
**File(s):** `connectors/native-theme-iced/examples/showcase.rs:528,1155`

**Problem:** `button_press_count` is `u32` and incremented with
`state.button_press_count += 1` at line 1155. In debug mode, Rust's
overflow checking would panic after 4,294,967,295 presses. In release
mode, it would wrap to 0.

This is a showcase example, not production code. A user would need to
click a button 4 billion times to trigger this. Not a practical concern.

**Best Solution:** No action. Documenting only because the project rule
is "no runtime panics" and debug-mode overflow panics are technically
panics. If a fix is desired, change to `state.button_press_count =
state.button_press_count.wrapping_add(1)` or use `saturating_add`.

---

### 49. `from_system()` Moves `sys.name` Before Using `sys.dark`/`sys.light`

**Category:** code-quality
**Severity:** none (informational)
**File(s):** `connectors/native-theme-iced/src/lib.rs:156-160`

**Problem:** Line 157 does `let name = sys.name;` which moves
`sys.name` out of the `SystemTheme`. Line 158 then accesses `sys.dark`
and `sys.light` which are still valid because `name` is a separate
field. This works because Rust allows partial moves -- after moving
`sys.name`, the other fields of `sys` are still accessible.

The gpui connector at `lib.rs:180-183` avoids this by borrowing first:
`let theme = to_theme(sys.active(), &sys.name, sys.is_dark)` borrows
`sys.name` with `&sys.name`, then moves the variant on line 183. The
iced connector moves the name first, then destructures the variant.

Both are correct Rust. The iced version is slightly more idiomatic for
moving all data out (no borrow needed since `name` is moved into
`to_theme` via `&name`). No change needed.

**Best Solution:** No action. This is a style observation, not a bug.

---

### 50. Gpui Connector Missing `Result` and `Rgba` Re-exports (Reverse of Issue 25)

**Category:** cross-connector-symmetry
**Severity:** low
**File(s):** `connectors/native-theme-gpui/src/lib.rs:73-76` vs
`connectors/native-theme-iced/src/lib.rs:82-85`

**Problem:** Issue 25 already notes this asymmetry. However, a closer
examination reveals the full scope. Iced re-exports:
```
AnimatedIcon, Error, IconData, IconProvider, IconRole, IconSet,
ResolvedThemeVariant, Result, Rgba, SystemTheme, ThemeSpec,
ThemeVariant, TransformAnimation
```

Gpui re-exports:
```
AnimatedIcon, Error, IconData, IconProvider, IconRole, IconSet,
ResolvedThemeVariant, SystemTheme, ThemeSpec, ThemeVariant,
TransformAnimation
```

Missing from gpui: `Result`, `Rgba`.
Missing from iced: `LinuxDesktop` (issue 9 already covers this).

Additionally, neither connector re-exports the free functions that
the showcase imports directly from `native_theme`:
- `prefers_reduced_motion()`
- `loading_indicator()`
- `system_icon_theme()`
- `system_icon_set()`
- `system_is_dark()`
- `load_icon()`
- `load_icon_from_theme()`
- `is_freedesktop_theme_available()`

These are used in the iced showcase at lines 21-22 via
`use native_theme::{..., loading_indicator, prefers_reduced_motion}`.
Users of the connector crate must depend on `native_theme` directly
to access these functions. This is different from type re-exports
(which enable using the connector as the sole dependency for types
in public signatures); function re-exports are less critical since
users can call `native_theme::load_icon()` directly.

Issue 25 already covers the type re-export gap. The free function
gap is an intentional design choice (connectors re-export types for
signature compatibility, not utility functions). No additional fix
needed beyond issue 25.

**Best Solution:** Subsumed by issue 25. The free function observation
is informational only.

---

### 51. `colorize_monochrome_svg` Does Not Handle `style` Attribute CSS Colors

**Category:** mapping-gap
**Severity:** low
**File(s):** `connectors/native-theme-iced/src/icons.rs:232-283`

**Problem:** The colorize function handles three patterns:
1. `currentColor` keyword (global string replace)
2. `fill="black"` / `fill="#000000"` / `fill="#000"` (attribute replace)
3. Root `<svg>` tag fill injection (when no fill attribute exists)

It does NOT handle colors specified via CSS `style` attributes, which
are valid SVG:
- `style="fill:black"` or `style="fill:#000000"`
- `style="fill:currentColor"` (the `currentColor` branch catches this
  because it does a global string replace, but the replacement produces
  `style="fill:#ff0000"` which is valid CSS)
- `<style>` blocks: `.icon { fill: black; }` (these are never handled)

The gpui connector's `colorize_svg` doc comment at line 975 explicitly
documents this: "Not handled: [...] CSS `style=\"fill:black\"` [...]".
The iced connector's doc comment does not document this limitation.

All bundled icon sets (Material, Lucide) use XML attributes, not CSS
style attributes, so this gap has no practical impact. Third-party
SVGs that use inline CSS styles would not be colorized.

**Solution Options:**
| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add a doc comment noting that CSS `style` attribute colors are not handled (matching gpui's doc) | Documents the limitation; consistent between connectors | No code change |
| B | Add `style="fill:black"` replacements | More thorough | Complex; risk of corrupting multi-property style strings |

**Best Solution:** A. Add one line to the doc comment matching gpui's
explicit documentation of what is NOT handled.

---

### 52. `from_system()` and `to_iced_theme()` Use Different `SystemTheme` Consumption Patterns

**Category:** api-design
**Severity:** low
**File(s):** `connectors/native-theme-iced/src/lib.rs:154-173`

**Problem:** `from_system()` consumes the `SystemTheme` by moving
fields out (line 157: `let name = sys.name`, line 158:
`if sys.is_dark { sys.dark } else { sys.light }`). After this, `sys`
is partially moved and cannot be reused.

`SystemThemeExt::to_iced_theme()` borrows via `&self` (line 171-173),
calling `self.active()` which returns `&ResolvedThemeVariant`. The
caller retains full ownership of the `SystemTheme`.

This means there is no way to get both the iced `Theme` AND retain
the `SystemTheme` for later variant switching using the convenience
API. A user who wants to switch between dark/light on the same
`SystemTheme` must either:
- Use `to_iced_theme()` (but loses the resolved variant per issue 6)
- Call `to_theme()` directly with `sys.active()` and `&sys.name`

This is already partially covered by issues 5, 6, and 39. However,
a specific pattern that is NOT covered: an app that wants to
pre-build both light and dark iced themes from a single `SystemTheme`
must call `to_theme()` twice manually:

```rust
let sys = SystemTheme::from_system()?;
let light_theme = to_theme(&sys.light, &sys.name);
let dark_theme = to_theme(&sys.dark, &sys.name);
```

This works today. No additional issue needed. The existing API
supports this use case via the manual `to_theme()` path.

**Best Solution:** No additional action beyond issues 5 and 6. The
manual `to_theme()` path serves users who need both variants.

---

### 53. Showcase `resolve_icon_choice()` Silently Falls Back to System for Unknown `IconSet` Variants

**Category:** robustness
**Severity:** very low
**File(s):** `connectors/native-theme-iced/examples/showcase.rs:294-301`

**Problem:** `resolve_icon_choice()` at line 294-302 matches
`resolved.icon_set` to determine availability:

```rust
let available = match resolved.icon_set {
    IconSet::Material | IconSet::Lucide => true,
    IconSet::Freedesktop => {
        native_theme::is_freedesktop_theme_available(&resolved.icon_theme)
    }
    _ => false,
};
```

The `_ => false` wildcard (already noted in issue 35) means any future
`IconSet` variant added to native-theme would silently be treated as
unavailable, falling back to the System icon set even though the new
set might be perfectly usable.

Issue 35 already covers this. However, issue 35 frames it as
"defeats exhaustiveness checking" -- the deeper consequence is that
adding a new icon set to native-theme would cause the showcase to
silently fall back without any indication to the user. The same
wildcard appears at lines 628-633 in the initial `Default` impl and
at lines 724-729 in the CLI override handling, and at lines 825-829
in `rebuild_theme()`, totaling 4 instances of this pattern in the
showcase.

**Best Solution:** Subsumed by issue 35. When fixing issue 35, apply
the fix to all 4 instances of the `IconSet` match in showcase.rs,
not just the one at line 301.

---

### 54. `colorize_monochrome_svg` Root Tag Detection Does Not Skip XML Declaration or Comments

**Category:** edge-case
**Severity:** very low
**File(s):** `connectors/native-theme-iced/src/icons.rs:262`

**Problem:** The fill injection branch uses `svg_str.find("<svg")` to
locate the root `<svg>` tag. However, SVG files can begin with an XML
declaration (`<?xml version="1.0"?>`) or contain comments before the
root element. If there is a comment containing the literal string
`<svg` before the actual root `<svg>` tag, the function would
incorrectly inject `fill=` into the comment.

More practically, if an SVG file starts with `<?xml ...?>` (which
many system-generated SVGs do), `find("<svg")` correctly skips the
XML declaration because `<?xml` does not match `<svg`. So the XML
declaration case is already handled correctly.

The only failing case is: a comment containing `<svg` before the
actual root element. This is addressed in issue 47. No additional
finding here.

**Best Solution:** Subsumed by issue 47. No additional action.

---

### 55. Gpui `from_preset()` Passes `name` (Slug) to `to_theme()` -- Same Issue as Iced #3

**Category:** cross-connector-symmetry
**Severity:** low
**File(s):** `connectors/native-theme-gpui/src/lib.rs:151-157`

**Problem:** Issue 3 notes that iced's `from_preset()` passes the
lookup slug (e.g., `"catppuccin-mocha"`) as the display name instead
of `spec.name` (e.g., `"Catppuccin Mocha"`). Examining the gpui
connector confirms the exact same issue at `lib.rs:151-156`:

```rust
let spec = ThemeSpec::preset(name)?;
let variant = spec.into_variant(is_dark).ok_or_else(|| { ... })?;
let resolved = variant.into_resolved()?;
let theme = to_theme(&resolved, name, is_dark);
```

Issue 3 already notes "The gpui connector has the same issue at its
`lib.rs:151-156`" at line 179 of the existing document. This is
already fully covered. No new finding.

**Best Solution:** Already covered by issue 3.

---

### 56. Showcase Missing `has_toml_icon_theme` Propagation on Error Fallback Paths

**Category:** logic-gap
**Severity:** low
**File(s):** `connectors/native-theme-iced/examples/showcase.rs:770-778`

**Problem:** In `rebuild_theme()`, when `ThemeChoice::OsTheme` and
the system theme lookup fails (line 770-778), the fallback path
loads adwaita but does NOT set `has_toml_icon_theme`. The variable
is initialized to `false` at line 757 and only set to `true` at
line 763 (system success path) or line 785 (preset success path).

When the system lookup fails and adwaita fallback is used,
`has_toml_icon_theme` remains `false`. This is passed to
`resolve_icon_choice()` at line 822, which then returns
`(IconSetChoice::System, ...)` -- skipping the adwaita preset's
`icon_theme` specification.

This is actually correct behavior: when the system theme fails,
falling back to the System icon set is reasonable since the user's
desktop environment is accessible even if the theme detection failed.
The adwaita TOML specifies `icon_theme = "default"` which means
"use whatever the TOML says", but since we are in fallback mode and
the system theme detection failed, respecting the system icon theme
is the safer choice.

However, there is a subtle inconsistency: in the `Default::default()`
init path at line 625, `has_toml_icon_theme` is hardcoded to `true`
regardless of whether the system succeeded or the adwaita fallback
was used. So the initial load and the rebuild-on-error paths handle
`has_toml_icon_theme` differently for the same adwaita fallback
scenario.

**Solution Options:**
| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Set `has_toml_icon_theme = true` in the system-failure-adwaita-fallback branch of `rebuild_theme()` to match `Default::default()` | Consistent between init and rebuild | Adwaita fallback uses adwaita's icon_theme spec even when system theme is broken |
| B | Change `Default::default()` init to pass `has_toml_icon_theme = false` for the fallback path | Consistent in the other direction | Initial load would use System icons instead of adwaita's icon_theme |
| C | No change | Both paths work reasonably for their contexts | Inconsistent behavior |

**Best Solution:** A. The init path's behavior (use adwaita's
icon_theme) is correct, so align `rebuild_theme()` to match.

---

### 57. No Test Verifies That `to_theme()` Extended Overrides Survive Round-Trip Through Palette Extraction

**Category:** test-gap
**Severity:** medium
**File(s):** `connectors/native-theme-iced/src/lib.rs` tests

**Problem:** The `to_theme()` function constructs a theme via
`Theme::custom_with_fn(name, pal, closure)`. The closure overrides 4
Extended palette fields. However, no test ever extracts the Extended
palette from the resulting theme and verifies the override values.

The existing tests verify:
- `to_theme_produces_non_default_theme`: checks palette.primary is
  non-zero (does NOT check Extended)
- `to_theme_from_preset`: checks palette.background > 0.9 (does NOT
  check Extended)

The `extended.rs` tests verify `apply_overrides()`, but as issue 1
documents, this function is dead code -- the production path uses an
inline closure in `to_theme()`. No test verifies that the inline
closure's overrides actually take effect on the final theme.

To verify, a test would need to:
```rust
let theme = to_theme(&resolved, "test");
let ext = theme.extended_palette();
assert_eq!(ext.secondary.base.color, palette::to_color(resolved.button.background));
assert_eq!(ext.secondary.base.text, palette::to_color(resolved.button.foreground));
assert_eq!(ext.background.weak.color, palette::to_color(resolved.defaults.surface));
assert_eq!(ext.background.weak.text, palette::to_color(resolved.defaults.foreground));
```

This is the most significant test gap in the crate: the primary
production code path (the inline closure) has zero test coverage for
its Extended palette overrides. The 4 tests in `extended.rs` test
the dead `apply_overrides()` function, not the live code.

**Solution Options:**
| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add a test in `lib.rs` that calls `to_theme()`, extracts the Extended palette, and verifies all 4 overridden fields match the resolved theme values exactly | Tests the actual production code path; highest-value single test in the crate | 1 new test |
| B | Fix issue 1 first (make `to_theme()` call `apply_overrides()`), then the existing extended.rs tests cover the production path | Eliminates the gap structurally | Depends on issue 1 |

**Best Solution:** A immediately, then B when issue 1 is addressed.
This single test would be the highest-value addition to the test
suite because it validates the only production code path that is
currently untested.

---

### Summary: Third-Pass New Findings

| # | Issue | Severity | Effort | Recommended Fix |
|---|-------|----------|--------|----------------|
| 46 | `color_approx_eq` epsilon 250x too loose for direct conversions | Medium | Trivial | Tighten to 1e-6 or use `assert_eq!` |
| 47 | `colorize_monochrome_svg` matches `<svg` in XML comments | Very Low | N/A | No action (edge case too unlikely) |
| 48 | Showcase `button_press_count` u32 overflow in debug mode | Very Low | Trivial | Use `wrapping_add` if desired |
| 49 | `from_system()` partial move style (informational) | None | N/A | No action |
| 50 | Gpui missing `Result`/`Rgba` + function re-export scope | Low | N/A | Subsumed by issue 25 |
| 51 | `colorize_monochrome_svg` missing CSS `style` attribute doc | Low | Trivial | Add doc comment matching gpui |
| 52 | `from_system()` vs `to_iced_theme()` consumption asymmetry | Low | N/A | Subsumed by issues 5+6 |
| 53 | Showcase 4 instances of `_ => false` on `IconSet` match | Very Low | N/A | Subsumed by issue 35 (apply to all 4) |
| 54 | Root tag detection skips XML decl correctly but not comments | Very Low | N/A | Subsumed by issue 47 |
| 55 | Gpui `from_preset()` slug-as-name (same as iced) | Low | N/A | Already covered by issue 3 |
| 56 | Showcase `has_toml_icon_theme` inconsistency between init and rebuild | Low | Trivial | Set `true` in adwaita-fallback rebuild path |
| 57 | No test verifies `to_theme()` Extended overrides on live code path | Medium | Low | Add 1 test extracting Extended palette from `to_theme()` output |
