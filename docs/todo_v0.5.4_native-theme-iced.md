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
