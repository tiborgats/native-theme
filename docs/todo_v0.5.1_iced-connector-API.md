# v0.5.1 API Improvements -- native-theme-iced (connector crate)

Analysis of API problems, verified against actual code (2026-03-29).
Each chapter covers one problem, lists all resolution options with pro/contra,
and proposes the best solution. Pre-1.0 -- backward compatibility is not a
constraint.

**Companion documents**:
- [todo_v0.5.1_native-theme-API.md](todo_v0.5.1_native-theme-API.md)
  covers the native-theme core crate. Cross-references marked **[CORE-N]**.
- [todo_v0.5.1_gpui-connector-API.md](todo_v0.5.1_gpui-connector-API.md)
  covers the native-theme-gpui connector. Cross-references marked **[GPUI-N]**.

Many issues here parallel the gpui connector. Where the analysis is
substantively identical, the reasoning is condensed with a cross-reference.
Where the iced connector differs (e.g., no `is_dark` parameter, no SVG
rasterization, native SVG rendering), those differences are called out.

**Assumed core renames**: Proposed code in this document uses post-rename names
from the core crate per **[CORE-1]** (`ResolvedTheme` → `ResolvedThemeVariant`)
and **[CORE-2]** (`NativeTheme` → `ThemeSpec`). Current-code examples retain
the pre-rename names.

---

## Table of Contents

1. [Excessive ceremony for the primary use case](#1-excessive-ceremony-for-the-primary-use-case)
2. [name: &str mandatory in to_theme()](#2-name-str-mandatory-in-to_theme)
3. [No convenience constructors on SystemTheme](#3-no-convenience-constructors-on-systemtheme)
4. [Suffix explosion instead of optional parameters](#4-suffix-explosion-instead-of-optional-parameters)
5. [animated_frames_to_svg_handles discards timing data](#5-animated_frames_to_svg_handles-discards-timing-data)
6. [colorize_monochrome_svg misses explicit black fills](#6-colorize_monochrome_svg-misses-explicit-black-fills)
7. [Code duplication between to_theme() and apply_overrides()](#7-code-duplication-between-to_theme-and-apply_overrides)
8. [Massive silent data loss](#8-massive-silent-data-loss)
9. [No re-exports of API-surface types](#9-no-re-exports-of-api-surface-types)
10. [Deprecated pick_variant() function still present](#10-deprecated-pick_variant-function-still-present)
11. [Padding helpers return wrong order for iced](#11-padding-helpers-return-wrong-order-for-iced)
12. [Missing #[must_use] on all pure functions](#12-missing-must_use-on-all-pure-functions)
13. [Alpha channel silently discarded in SVG colorization](#13-alpha-channel-silently-discarded-in-svg-colorization)
14. [Incomplete font helper set](#14-incomplete-font-helper-set)
15. [README contradicts source on font size conversion](#15-readme-contradicts-source-on-font-size-conversion)
16. [to_color() inaccessible to external callers](#16-to_color-inaccessible-to-external-callers)
17. [spin_rotation_radians mixes Duration and u32 parameter types](#17-spin_rotation_radians-mixes-duration-and-u32-parameter-types)
18. [spin_rotation_radians produces NaN on zero duration](#18-spin_rotation_radians-produces-nan-on-zero-duration)
19. [colorize_monochrome_svg corrupts self-closing SVG tags](#19-colorize_monochrome_svg-corrupts-self-closing-svg-tags)
20. [Icon conversion functions force unnecessary clones](#20-icon-conversion-functions-force-unnecessary-clones)

---

## 1. Excessive ceremony for the primary use case

**Verdict: VALID -- high priority**

**Parallel: [GPUI-1]**. Same problem, same root cause, same dependency on
**[CORE-3]**: `ThemeVariant::into_resolved()` must land first.

The crate-level example (`lib.rs:18-27`) shows the canonical path:

```rust
let nt = NativeTheme::preset("catppuccin-mocha").unwrap();
let mut variant = nt.pick_variant(false).unwrap().clone();
variant.resolve();
let resolved = variant.validate().unwrap();
let theme = to_theme(&resolved, "My App");
```

Five steps for the most common operation. The internal `resolve_variant()` in
native-theme already does resolve+validate but is private.

### Option A: Add `from_preset(name, is_dark) -> Result<iced_core::theme::Theme>`

A free function that loads a preset, picks the variant, resolves, validates,
and converts -- all in one call.

**Pro:**
- One-liner for 90% of use cases
- Hides the entire resolve/validate/convert pipeline
- The preset name doubles as the theme display name (no extra `&str`)
- Easy to document as THE entry point

**Contra:**
- Adds a free function to the crate root
- Couples preset loading with iced conversion (less composable)
- Callers who need the intermediate `ResolvedTheme` for widget metrics
  (e.g., `button_padding()`) still need the manual path
- Cannot expose the `ResolvedTheme` without returning a richer type

### Option B: Extension trait on `NativeTheme`

```rust
pub trait NativeThemeExt {
    fn to_iced_theme(&self, is_dark: bool) -> Result<iced_core::theme::Theme>;
}
```

**Pro:**
- Method syntax on the type users already have
- Composable: use `NativeTheme` for other things first
- Extension traits are idiomatic Rust

**Contra:**
- Requires importing the trait (extra `use`)
- Method on a foreign type -- harder to discover
- Still two steps: `preset()` then `.to_iced_theme()`

### Option C: Make `resolve_variant()` public in native-theme + thin wrapper

**Pro:**
- Exposes a useful building block without coupling to iced
- Both crates benefit (see **[CORE-3]**)
- Keeps native-theme-iced as a pure mapper

**Contra:**
- Still 3 steps: `preset()` -> `into_resolved()` -> `to_theme()`
- Doesn't fully solve the ceremony problem

### Option D: Combine A + C

`from_preset()` as the one-liner, `into_resolved()` + `to_theme()` as the
composable path for power users.

**Pro:**
- One-liner for the common case
- Power users can still compose intermediate steps (and call `button_padding()`
  etc. on the `ResolvedTheme` they get from `into_resolved()`)
- Both crates get better APIs

**Contra:**
- More API surface (two ways to do the same thing)

### Option E: `from_preset()` returns both the Theme and the ResolvedTheme

```rust
pub fn from_preset(name: &str, is_dark: bool) -> Result<(iced_core::theme::Theme, ResolvedTheme)>
```

**Pro:**
- One-liner AND callers get the `ResolvedTheme` for widget metrics
- No data loss: callers can read `resolved.button.padding_horizontal` etc.
- Strictly more useful than returning only the Theme

**Contra:**
- Tuple return is slightly awkward; could use a named struct instead
- The `ResolvedTheme` may not be needed by most callers (wasted allocation)
- A named struct adds a new type to the API

### PROPOSED: Option D

Add `from_preset(name, is_dark) -> Result<Theme>` as the primary entry point.
This depends on **[CORE-3]** (`into_resolved()`) landing first. The
convenience function covers the common case; the composable path via
`into_resolved()` + `to_theme()` covers callers who also need widget metrics
from the `ResolvedTheme`.

Option E's tuple return is tempting (since section 8 documents the data loss),
but most callers only need the iced Theme. Those who need `ResolvedTheme` for
metrics should use the manual path -- it's only 2-3 steps once [CORE-3] lands.

---

## 2. name: &str mandatory in to_theme()

**Verdict: VALID -- low priority**

**Parallel: [GPUI-3]**. Same problem, simpler in iced because there is no
`is_dark` parameter alongside `name`.

```rust
pub fn to_theme(resolved: &ResolvedTheme, name: &str) -> iced_core::theme::Theme
```

`name` sets `Theme`'s internal display string. It has no functional effect on
widget rendering. Every caller must invent a string. The `from_preset()`
convenience (section 1) solves this for the common path by using the preset
name.

### Option A: Remove `name`, default to `"Native Theme"`

```rust
pub fn to_theme(resolved: &ResolvedTheme) -> iced_core::theme::Theme
```

Internally passes `"Native Theme".to_string()` to `Theme::custom_with_fn()`.

**Pro:**
- Simplest possible signature
- Pre-1.0: breaking the signature is free
- Callers who need a custom name can call `Theme::custom_with_fn()` directly

**Contra:**
- Loses the ability to set a meaningful display name via this function
- Users building theme selectors may want distinct names for debug/display
- Downstream code that pattern-matches on theme name would break (unlikely)

### Option B: Make `name` optional via `impl Into<Option<&str>>`

```rust
pub fn to_theme(resolved: &ResolvedTheme, name: impl Into<Option<&str>>) -> Theme
```

**Pro:**
- Backward compatible: bare `&str` still works via `Into<Option<&str>>`
- Callers who don't care pass `None`
- Default name when `None`

**Contra:**
- `impl Into<Option<&str>>` is an unusual pattern that confuses users
- Requires documenting the default behavior
- Pre-1.0, backward compatibility is not a constraint, so this complexity
  is unjustified

### Option C: `from_preset()` uses preset name, `to_theme()` keeps `name`

**Pro:**
- The convenience function (section 1) has a natural name (the preset name)
- `to_theme()` stays explicit for power users who build themes from
  non-preset sources and know what name to use
- Minimal change

**Contra:**
- Power users must still pass a name even if they don't care
- Inconsistent: convenience path auto-names, manual path forces a name

### Option D: Add `name` field to `ResolvedTheme` (core change)

**Pro:**
- Single source of truth carried through the pipeline
- `to_theme()` reads name from resolved, no parameter needed

**Contra:**
- Changes a core type for a purely cosmetic field
- `ResolvedTheme` is visual data; a display name doesn't belong there
- Who populates it? Presets have names, system themes have names, but what
  about custom themes?

### PROPOSED: Option C

**Aligned with [GPUI-3]** for cross-connector consistency. The `from_preset()`
convenience (section 1) derives the name from the preset name. The
`SystemThemeExt` trait (section 3) derives it from `SystemTheme.name`. The
manual `to_theme()` keeps its `name` parameter -- power users who call it
directly have a specific context where they know what name to use. This is
a non-issue once the convenience functions exist.

---

## 3. No convenience constructors on SystemTheme

**Verdict: VALID -- medium priority**

**Parallel: [GPUI-11]**. Same problem.

```rust
let system = native_theme::from_system()?;
let resolved = system.active();  // returns &ResolvedTheme
let theme = to_theme(resolved, &system.name);
```

Users must call a free function, then extract fields from `SystemTheme`,
then pass them to `to_theme()`. The connector doesn't extend the type it
maps from.

### Option A: Extension trait on `SystemTheme`

```rust
pub trait SystemThemeExt {
    fn to_iced_theme(&self) -> iced_core::theme::Theme;
}

impl SystemThemeExt for native_theme::SystemTheme {
    fn to_iced_theme(&self) -> iced_core::theme::Theme {
        let resolved = if self.is_dark { &self.dark } else { &self.light };
        to_theme(resolved)  // name derived internally
    }
}
```

**Pro:**
- Natural method syntax: `system.to_iced_theme()`
- `is_dark` and `name` derived from `SystemTheme` fields
- Idiomatic Rust for cross-crate extension
- Discoverable via IDE autocompletion on `SystemTheme`

**Contra:**
- Requires importing the trait: `use native_theme_iced::SystemThemeExt;`
- Extension traits are less discoverable for new Rust users
- Adds a trait to the API surface

### Option B: Free function `from_system_theme() -> Result<Theme>`

```rust
pub fn from_system_theme() -> native_theme::Result<iced_core::theme::Theme> {
    let sys = native_theme::from_system()?;
    let resolved = if sys.is_dark { &sys.dark } else { &sys.light };
    Ok(to_theme(resolved))
}
```

**Pro:**
- One-liner for "give me the system theme as iced"
- No trait imports needed
- Pairs with `from_preset()` for a consistent API

**Contra:**
- Hides the `SystemTheme` -- callers who need both light and dark variants,
  the overlay mechanism, or widget metrics lose access
- Name collision risk: `native_theme::from_system` vs
  `native_theme_iced::from_system_theme` (different return types, mitigated
  by the `_theme` suffix)

### Option C: Both -- extension trait AND free function

**Pro:**
- `from_system_theme()` for the quick path
- Extension trait for users who already have a `SystemTheme`
- Maximum convenience

**Contra:**
- Three ways to do the same thing (free fn, extension trait, manual)
- API surface growth

### PROPOSED: Option C

Add both. The extension trait serves users who have a `SystemTheme` and want
method syntax. The free function serves the "just give me the system theme"
one-liner. Pre-1.0, API surface growth is acceptable when each entry point
serves a distinct use pattern. **[CORE-11]** moves `native_theme::from_system()`
to `SystemTheme::from_system()`, eliminating the name collision -- so
`native_theme_iced::from_system()` is unambiguous.

---

## 4. Suffix explosion instead of optional parameters

**Verdict: VALID -- low priority**

**Parallel: [GPUI-6]**. Same pattern, slightly different because iced
separates SVG and raster handles (no raster colorization exists).

Current icon functions in `icons.rs`:

```rust
pub fn to_svg_handle(data: &IconData) -> Option<svg::Handle>
pub fn to_svg_handle_colored(data: &IconData, color: Color) -> Option<svg::Handle>
pub fn custom_icon_to_svg_handle(provider: &dyn IconProvider, icon_set: &str) -> Option<svg::Handle>
pub fn custom_icon_to_svg_handle_colored(provider: &dyn IconProvider, icon_set: &str, color: Color) -> Option<svg::Handle>
```

Two pairs where `Option<Color>` would halve the SVG function count.

### Option A: Merge pairs using `Option<Color>`

```rust
pub fn to_svg_handle(data: &IconData, color: Option<Color>) -> Option<svg::Handle>
pub fn custom_icon_to_svg_handle(p: &dyn IconProvider, set: &str, color: Option<Color>) -> Option<svg::Handle>
```

**Pro:**
- Halves the SVG function count (4 -> 2, total 6 -> 4)
- `None` clearly means "no colorization"
- Cleaner API surface

**Contra:**
- Callers who never colorize must write `None` every time
- Colorization caveat (monochrome SVGs only) is less discoverable when hidden
  behind an Option parameter vs. a distinctly named function

### Option B: Builder pattern / options struct

```rust
pub struct SvgOptions { pub color: Option<Color> }
pub fn to_svg_handle(data: &IconData, opts: &SvgOptions) -> Option<svg::Handle>
```

**Pro:**
- Extensible: future parameters slot in without signature changes

**Contra:**
- Over-engineered for a single optional parameter
- More ceremony than current suffixes or Option<Color>
- No other parameters are anticipated

### Option C: Keep separate functions

**Pro:**
- No breaking changes (irrelevant pre-1.0)
- Separate functions communicate intent clearly in docs
- The "colored" functions have important caveats (monochrome SVGs only)
  that benefit from dedicated doc comments

**Contra:**
- 4 SVG functions instead of 2
- Suffix pattern doesn't scale

### PROPOSED: Option A

Merge to `Option<Color>`. The colorization caveat belongs in the function docs
regardless of how many functions there are. Callers who don't colorize write
`to_svg_handle(data, None)` -- minimal overhead. Pre-1.0, the breaking change
is free.

The raster functions (`to_image_handle`, `custom_icon_to_image_handle`) stay
as-is -- raster colorization is not supported.

---

## 5. animated_frames_to_svg_handles discards timing data

**Verdict: VALID -- medium priority**

**Parallel: [GPUI-22]** (which absorbed [GPUI-8]).

```rust
pub fn animated_frames_to_svg_handles(anim: &AnimatedIcon) -> Option<Vec<svg::Handle>>
```

Returns `Vec<svg::Handle>` but discards `frame_duration_ms` and `repeat`.
Every caller who needs to animate the frames must destructure
`AnimatedIcon::Frames` a second time:

```rust
// With the function -- forced double pattern match:
if let Some(handles) = animated_frames_to_svg_handles(&anim) {
    let AnimatedIcon::Frames { frame_duration_ms, repeat, .. } = &anim else { unreachable!() };
    // Now have handles + timing. Set up iced::time::every(Duration::from_millis(...))
}

// Without the function -- one pattern match, less code:
if let AnimatedIcon::Frames { frames, frame_duration_ms, repeat } = &anim {
    let handles: Vec<_> = frames.iter().filter_map(|f| to_svg_handle(f, None)).collect();
    // Have everything
}
```

The function is strictly worse than not using it.

### Option A: Return a struct that includes timing

```rust
pub struct AnimatedSvgHandles {
    pub handles: Vec<iced_core::svg::Handle>,
    pub frame_duration_ms: u32,
}

pub fn animated_frames_to_svg_handles(anim: &AnimatedIcon) -> Option<AnimatedSvgHandles>
```

**Pro:**
- Callers get everything in one call
- No redundant pattern matching
- The struct name makes the return type self-documenting
- `Option` still signals "wrong variant or empty frames"
- Callers can use `handles.frame_duration_ms` directly for
  `iced::time::every(Duration::from_millis(...))`

**Contra:**
- Adds a new type to the API
- Minor: struct may feel over-engineered for three fields

### Option B: Return a tuple `Option<(Vec<svg::Handle>, u32)>`

**Pro:**
- No new type needed
- All data returned

**Contra:**
- Unnamed tuple fields are unclear at the call site
- Not idiomatic Rust for a public API
- Hard to remember which `u32` is which

### Option C: Remove the function entirely

Callers pattern-match `AnimatedIcon::Frames` and call `to_svg_handle()` per
frame themselves.

**Pro:**
- Eliminates a function that makes things worse
- One fewer API surface to maintain
- Callers write the same amount of code (one pattern match + one map)

**Contra:**
- The "cache this, don't call per-frame" guidance is valuable and would
  need to move to module-level docs
- Less discoverable: users must know to iterate frames manually
- Loses the filtering of non-SVG frames (the current function filters out
  RGBA frames silently)

### Option D: Take `&[IconData]` (the frames slice) directly

```rust
pub fn icon_data_to_svg_handles(frames: &[IconData]) -> Vec<svg::Handle>
```

**Pro:**
- Operates on exactly what it needs
- No `Option` return -- always produces a `Vec` (even if empty)
- Caller extracts frames + timing in one pattern match, passes frames here

**Contra:**
- Loses the conceptual link to `AnimatedIcon`
- The "don't call per-frame, cache this" guidance is less prominent
- Naming doesn't mention "animated" which reduces discoverability
- Caller must still destructure to get frames, so total code isn't less

### PROPOSED: Option A

**Depends on [CORE-18]**: `Repeat` is removed from native-theme; the struct
drops the `repeat` field accordingly.

Return `AnimatedSvgHandles` struct. The struct is small (2 fields), justified,
and eliminates the forced double pattern match. The `Option` return is retained
for the wrong-variant / empty-frames case.

---

## 6. colorize_monochrome_svg misses explicit black fills

**Verdict: VALID -- low priority**

**Parallel: [GPUI-27]**. Identical issue -- the iced connector's
`colorize_monochrome_svg()` (`icons.rs:175-205`) is essentially the same code
as the gpui connector's `colorize_svg()`.

The function handles two SVG patterns:

1. `currentColor` replacement (Lucide-style SVGs, `icons.rs:184`)
2. Root `<svg>` fill injection when no `fill=` attribute exists
   (Material-style implicit black, `icons.rs:190-201`)

But SVGs with explicit `fill="black"`, `fill="#000"`, or `fill="#000000"` on
the root tag pass through unchanged. `tag.contains("fill=")` returns true,
so injection is skipped. `currentColor` is not present, so replacement is
skipped. These icons remain black regardless of the requested color.

In practice, the bundled icon sets (Material, Lucide) use the two handled
patterns. The issue only affects third-party SVGs that use explicit black.

### Option A: Replace common black color values

```rust
let svg_str = svg_str
    .replace("currentColor", &hex)
    .replace("fill=\"black\"", &format!("fill=\"{}\"", hex))
    .replace("fill=\"#000\"", &format!("fill=\"{}\"", hex))
    .replace("fill=\"#000000\"", &format!("fill=\"{}\"", hex));
```

**Pro:**
- Covers the most common explicit-black patterns
- Simple string replacement
- Handles the fill variants most SVG tools produce

**Contra:**
- Doesn't cover `fill="#00000000"` (with alpha), `fill="rgb(0,0,0)"`,
  CSS `style="fill:black"`, or `stroke="black"`
- Replaces ALL occurrences, not just the root element -- could recolor
  intentionally black sub-elements in multi-color SVGs
- The function is documented as "for monochrome bundled icon sets" --
  multi-color SVGs should use `to_svg_handle()` without colorization
- Fragile: whitespace variations like `fill = "black"` won't match

### Option B: Parse the SVG properly and rewrite fills/strokes

Use a lightweight XML parser or regex to replace fill/stroke attributes on
all elements.

**Pro:**
- Handles all color representations and whitespace variations
- Can distinguish root element from child elements

**Contra:**
- Significant complexity increase for an edge case
- Adding an XML parser dependency for SVG colorization is heavy
- Over-engineered for the documented use case (monochrome bundled icons)

### Option C: Document the limitation, no code change

**Pro:**
- Sets expectations for callers with custom SVGs
- The two handled patterns cover all bundled icon sets

**Contra:**
- Third-party SVGs with explicit black remain broken
- Documentation doesn't fix the problem

### Option D: Add `fill="black"` and `fill="#000000"` to replacements, document remaining gaps

Pragmatic middle ground: handle the two most common explicit-black forms
without trying to cover every CSS color representation.

**Pro:**
- Covers 95%+ of real-world SVGs that use explicit black
- Two extra string replacements -- minimal complexity
- Doesn't try to be a full SVG rewriter

**Contra:**
- Still doesn't handle stroke, CSS style attributes, or rgb() notation
- Replaces all occurrences (not just root) -- same risk as Option A but
  acceptable for the documented "monochrome only" use case
- Partial fix that may create false confidence

### PROPOSED: Option D

Add replacements for `fill="black"` and `fill="#000000"`, and document that
exotic color representations are not supported. The function is explicitly for
monochrome bundled icon sets -- comprehensive SVG rewriting is out of scope.

---

## 7. Code duplication between to_theme() and apply_overrides()

**Verdict: VALID -- medium priority**

**Unique to the iced connector** -- no parallel in [GPUI].

`to_theme()` (`lib.rs:66-85`) applies 4 extended palette overrides inline:

```rust
iced_core::theme::Theme::custom_with_fn(name.to_string(), pal, move |p| {
    let mut ext = iced_core::theme::palette::Extended::generate(p);

    ext.secondary.base.color = palette::to_color(button_bg);
    ext.secondary.base.text = palette::to_color(button_fg);
    ext.background.weak.color = palette::to_color(surface);
    ext.background.weak.text = palette::to_color(foreground);

    ext
})
```

`extended::apply_overrides()` (`extended.rs:17-25`) does the same 4 overrides:

```rust
pub fn apply_overrides(extended: &mut Extended, resolved: &ResolvedTheme) {
    extended.secondary.base.color = to_color(resolved.button.background);
    extended.secondary.base.text = to_color(resolved.button.foreground);
    extended.background.weak.color = to_color(resolved.defaults.surface);
    extended.background.weak.text = to_color(resolved.defaults.foreground);
}
```

These are identical operations. If a new override is added to one, the other
won't pick it up -- a drift bug waiting to happen.

Additionally, the `to_theme()` doc comment (`lib.rs:58-59`) claims it uses
`extended::apply_overrides()`:

> overrides secondary and background.weak entries via
> [`extended::apply_overrides()`]

This is a documentation lie -- `to_theme()` inlines the overrides instead of
calling the function.

The duplication exists because `to_theme()` uses a `move` closure to capture
the needed `Rgba` values (since `Rgba` is `Copy`). However,
`Theme::custom_with_fn()` takes `FnOnce` and calls it immediately, so the
closure can borrow `&resolved` instead of cloning individual fields.

### Option A: Call `apply_overrides()` from within the closure

```rust
pub fn to_theme(resolved: &ResolvedTheme) -> Theme {
    let pal = palette::to_palette(resolved);

    iced_core::theme::Theme::custom_with_fn("Native Theme".to_string(), pal, |p| {
        let mut ext = iced_core::theme::palette::Extended::generate(p);
        extended::apply_overrides(&mut ext, resolved);
        ext
    })
}
```

Since `custom_with_fn` takes `FnOnce` and calls it immediately, the closure
can borrow `resolved` without `move`.

**Pro:**
- Single source of truth -- overrides defined only in `apply_overrides()`
- Fixes the doc comment lie (it now genuinely calls `apply_overrides()`)
- Removes 4 lines of duplicated code
- No new types or APIs

**Contra:**
- Requires verifying that `Theme::custom_with_fn` indeed calls the closure
  immediately (it does -- `FnOnce` closure is consumed, and the function
  needs the `Extended` value to construct the `Theme`)
- If iced changes `custom_with_fn` to store the closure (unlikely -- would
  require `FnOnce + 'static`), this would break

### Option B: Make `extended` module `pub(crate)`, remove `apply_overrides()` entirely

Keep the overrides only in `to_theme()`. The `extended` module becomes an
internal detail.

**Pro:**
- One copy of the overrides -- no drift risk
- Smaller public API surface
- `apply_overrides()` has no external callers (it's not used by `to_theme()`
  and no downstream code depends on it)

**Contra:**
- Power users lose the ability to build custom iced themes with native-theme
  overrides without going through `to_theme()`
- Removes a useful building block for advanced use cases
- If `extended` is already public, removing it is a breaking change (but
  pre-1.0, this is free)

### Option C: Keep both, accept the duplication, fix the doc comment

**Pro:**
- No code change beyond a doc comment fix
- The 4 overrides are unlikely to diverge (they're trivial mappings)

**Contra:**
- Duplication is a maintenance hazard regardless of how trivial the code is
- Code review will repeatedly flag the inconsistency
- The principle "don't repeat yourself" exists for exactly this pattern

### Option D: Refactor `apply_overrides()` to take individual fields instead of `&ResolvedTheme`

```rust
pub fn apply_overrides(
    extended: &mut Extended,
    button_bg: Rgba, button_fg: Rgba,
    surface: Rgba, foreground: Rgba,
)
```

Both `to_theme()` and external callers call this with the appropriate fields.

**Pro:**
- Single implementation
- Works with the `move` closure pattern (captures `Copy` fields)
- No borrowing concerns

**Contra:**
- Ugly signature (4 positional `Rgba` parameters)
- The current signature `(&mut Extended, &ResolvedTheme)` is cleaner
- Splitting a struct into individual fields defeats the purpose of the struct

### PROPOSED: Option A

Call `apply_overrides()` from the closure, borrowing `resolved`. This is the
cleanest fix: single source of truth, no new types, fixes the doc comment
lie, and removes 4 lines of duplicated code. The `FnOnce` closure can borrow
`resolved` because `custom_with_fn` consumes the closure immediately.

**Remaining question**: should `extended` stay `pub` or become `pub(crate)`?
See the secondary decision below.

### Secondary: `extended` module visibility

The `extended` module has exactly one public function (`apply_overrides`).
Its purpose is to override auto-generated iced `Extended` palette entries
with native-theme values. Once `to_theme()` calls it internally, the
question is whether external callers need it.

**Keep `pub`**: Power users who call `Theme::custom_with_fn()` directly
(e.g., to compose native-theme overrides with their own) can use
`apply_overrides()` as a building block. This is the `palette` module's
role too -- `to_palette()` is useful independently of `to_theme()`.

**Make `pub(crate)`**: `apply_overrides()` has no known external callers.
Users who build custom themes can read the 4 lines and replicate them.
Keeping it public commits to supporting it across versions.

**Decision**: Keep `pub` for now. Both `palette` and `extended` serve as
building blocks for the power-user path. If section 8 (data loss) leads to
richer extended overrides in the future, `apply_overrides()` becomes even
more valuable as a composable helper.

---

## 8. Massive silent data loss

**Verdict: VALID -- medium priority (documentation / design)**

**Parallel: [GPUI-13]**. Same structural problem, different severity.

`ResolvedTheme` contains 28 top-level fields (25 per-widget structs +
`defaults` + `text_scale` + `icon_set`) with ~300+ individual fields across
the entire tree. The connector uses a small fraction.

### Fields consumed by the iced connector

**From `defaults`** (13 of ~46 fields):
- `background`, `foreground`, `accent`, `success`, `warning`, `danger`
  (palette mapping, `palette.rs:24-34`)
- `surface` (extended override, `lib.rs:72` / `extended.rs:23`)
- `radius`, `radius_lg` (border radius helpers, `lib.rs:104-111`)
- `font.family`, `font.size` (font helpers, `lib.rs:119-129`)
- `mono_font.family`, `mono_font.size` (mono font helpers, `lib.rs:132-142`)

**From `button`** (4 of 14 fields):
- `background`, `foreground` (extended override, `lib.rs:70-71`)
- `padding_horizontal`, `padding_vertical` (padding helper, `lib.rs:88-93`)

**From `input`** (2 of 13 fields):
- `padding_horizontal`, `padding_vertical` (padding helper, `lib.rs:96-101`)

**From `scrollbar`** (1 of 7 fields):
- `width` (scrollbar helper, `lib.rs:114-116`)

**Total: ~20 fields consumed out of ~300+** (~7% coverage).

### Unused from `defaults` (33+ fields)

Colors: `accent_foreground`, `border`, `muted`, `shadow`, `link`,
`selection`, `selection_foreground`, `selection_inactive`,
`disabled_foreground`, `danger_foreground`, `warning_foreground`,
`success_foreground`, `info`, `info_foreground`

Geometry: `frame_width`, `disabled_opacity`, `border_opacity`,
`shadow_enabled`, `focus_ring_color`, `focus_ring_width`,
`focus_ring_offset`, `font.weight`, `mono_font.weight`, `line_height`

Spacing: all 7 entries (`xxs` through `xxl`)

Icon sizes: all 5 entries (`toolbar`, `small`, `large`, `dialog`, `panel`)

Text scale: all 4 entries (`caption`, `section_heading`,
`dialog_title`, `display`)

Accessibility: `text_scaling_factor`, `reduce_motion`, `high_contrast`,
`reduce_transparency`

### Completely unused widget structs (24 of 28)

`window`, `checkbox`, `menu`, `tooltip`, `slider`, `progress_bar`, `tab`,
`sidebar`, `toolbar`, `status_bar`, `list`, `popover`, `splitter`,
`separator`, `switch`, `dialog`, `spinner`, `combo_box`,
`segmented_control`, `card`, `expander`, `link`, `text_scale`, `icon_set`

### Root cause

iced's built-in theme system is fundamentally a 6-color `Palette` with an
auto-generated `Extended` palette. There are no per-widget geometry slots.
Widget sizing is applied inline per widget instance, not through the theme.
The connector *cannot* map most data because iced's `Theme` type has no
corresponding fields.

A user who sets `resolved.dialog.radius = 12.0` or
`resolved.tooltip.padding_horizontal = 8.0` gets zero feedback that those
values go nowhere.

### Option A: Document the coverage gap explicitly

Add a "Coverage" section to the crate-level and `to_theme()` docs listing
exactly which fields are consumed and which are discarded.

**Pro:**
- Zero code change -- pure documentation
- Honest with users about what the connector does and doesn't do
- Users can read discarded fields directly from the `ResolvedTheme`
  they already have (e.g., `resolved.tooltip.padding_horizontal`)
- The limitation is in iced's theme architecture, not in this crate

**Contra:**
- Documentation doesn't fix the gap, just makes it visible
- Large doc section that may become stale
- Doesn't help users who expect "apply theme" to be complete

### Option B: Return a richer struct that carries Theme + ResolvedTheme

```rust
pub struct IcedTheme {
    pub theme: iced_core::theme::Theme,
    pub resolved: ResolvedTheme,
}
```

**Pro:**
- Users get both the iced Theme AND full ResolvedTheme
- Application code can read `iced_theme.resolved.tooltip.padding_horizontal`
- No data is lost

**Contra:**
- Changes the return type of `to_theme()` (breaking, but pre-1.0)
- Awkward: users must access `.theme` everywhere
- The caller already has the `ResolvedTheme` they passed in
- Carrying a second copy increases memory usage for no reason

### Option C: Provide per-widget accessor helpers for common metrics

```rust
pub fn tooltip_padding(resolved: &ResolvedTheme) -> [f32; 2]
pub fn menu_item_height(resolved: &ResolvedTheme) -> f32
pub fn checkbox_indicator_size(resolved: &ResolvedTheme) -> f32
// ... one per widget
```

**Pro:**
- Bridges the geometry gap with typed helpers
- Consistent with existing `button_padding()` and `input_padding()` pattern
- Composable: use only the helpers you need

**Contra:**
- Massive API surface: one function per metric per widget (~50+ functions)
- Each is a trivial field access the user can do themselves on `ResolvedTheme`
- Maintenance burden: must track both `ResolvedTheme` and iced layout APIs
- Speculative: unclear which widgets users actually customize in iced apps

### Option D: Document coverage + provide a few high-value helpers

Document the coverage gap (Option A). Add helpers only for widgets where
iced doesn't provide theme-level styling but the metric is commonly needed:

```rust
pub fn tooltip_padding(resolved: &ResolvedTheme) -> [f32; 2]
pub fn tab_padding(resolved: &ResolvedTheme) -> [f32; 2]
pub fn list_item_height(resolved: &ResolvedTheme) -> f32
```

**Pro:**
- Documents the gap honestly
- Adds helpers where they provide the most value
- Doesn't try to wrap every field
- Helpers serve as examples for users who need other fields

**Contra:**
- Subjective: which widgets are "high value"?
- Still a fraction of the total fields
- Each helper is a one-line field access -- is the indirection worth it?

### PROPOSED: Option A

Document the coverage gap in crate-level and `to_theme()` docs. The
limitation is architectural (iced's `Theme` has 6 color slots + auto-generated
extended palette, no geometry). Users who need per-widget metrics read them
from the `ResolvedTheme` they already have. A coverage table in the docs is
the honest, low-effort solution.

The existing helpers (`button_padding`, `input_padding`, `border_radius`,
`scrollbar_width`, fonts) cover the most common inline-style needs. Adding
more one-liner field accessors provides marginal value over reading
`resolved.widget.field` directly.

---

## 9. No re-exports of API-surface types

**Verdict: VALID -- low priority**

**Parallel: [GPUI-17]**. Same problem, slightly different type set because
iced uses `iced_core::Color` instead of `gpui::Hsla`.

Users must import types from three crates:

```rust
use native_theme::{ResolvedTheme, NativeTheme, IconData, IconProvider, AnimatedIcon, Repeat};
use iced_core::{Color, theme::Theme};
use native_theme_iced::{to_theme, icons::to_svg_handle};
```

Types appearing in native-theme-iced's public function signatures:

| Type | Source crate | Used in |
|------|-------------|---------|
| `ResolvedThemeVariant` | `native_theme` | `to_theme()`, all metric helpers |
| `ThemeSpec` | `native_theme` | `pick_variant()` (deprecated) |
| `ThemeVariant` | `native_theme` | `pick_variant()` (deprecated) |
| `IconData` | `native_theme` | `to_image_handle()`, `to_svg_handle()` |
| `IconProvider` | `native_theme` | `custom_icon_to_*()` |
| `AnimatedIcon` | `native_theme` | `animated_frames_to_svg_handles()` |
| `Color` | `iced_core` | `to_svg_handle_colored()` |
| `image::Handle` | `iced_core` | `to_image_handle()` |
| `svg::Handle` | `iced_core` | `to_svg_handle()` |
| `Radians` | `iced_core` | `spin_rotation_radians()` |

### Option A: Re-export all types appearing in public signatures

```rust
pub use native_theme::{
    ResolvedThemeVariant, ThemeSpec, ThemeVariant, SystemTheme,
    IconData, IconProvider, AnimatedIcon, TransformAnimation,
};
pub use iced_core::{Color, Radians};
pub use iced_core::image::Handle as ImageHandle;
pub use iced_core::svg::Handle as SvgHandle;
```

**Pro:**
- One `use native_theme_iced::*` covers everything
- Users don't need to know that `IconData` comes from `native_theme`
- Standard practice for connector/bridge crates

**Contra:**
- Large re-export surface (~15 types)
- Version coupling: if native-theme renames a type, the re-export breaks
- Re-exporting `iced_core::Color` may conflict with users' existing
  `use iced::*` or `use iced_core::*`
- Users who already depend on `iced` get duplicate type paths

### Option B: Re-export only `native_theme` types

```rust
pub use native_theme::{
    ResolvedThemeVariant, ThemeSpec, SystemTheme,
    IconData, IconProvider, AnimatedIcon,
};
```

**Pro:**
- Re-exports only what users can't easily get elsewhere
- Users building iced apps already depend on `iced` / `iced_core`, so
  those types are available
- Smaller re-export surface (~7 types)
- Covers the main friction: needing `native-theme` as a direct `Cargo.toml`
  dependency just to name `ResolvedTheme` or `IconData`

**Contra:**
- Users still need `iced_core` for `Color` / `Handle` types
- Partial solution

### Option C: Keep as-is, document required dependencies

**Pro:**
- No code change
- Avoids version-coupling issues

**Contra:**
- Doesn't fix the ergonomic gap
- Every new user hits the same friction
- Documentation is not enforced by the compiler

### PROPOSED: Option B

Re-export `native_theme` types that appear in public signatures. Users
building iced apps already have `iced` / `iced_core` as dependencies, so
re-exporting those types adds collision risk for no benefit. The main
friction is needing `native-theme` as a direct dependency just to name
`ResolvedThemeVariant` -- the re-export eliminates that.

---

## 10. Deprecated pick_variant() function still present

**Verdict: VALID -- low priority (cleanup)**

**No parallel in [GPUI]** -- the gpui connector doesn't have deprecated
functions.

`lib.rs:39-52`:

```rust
#[deprecated(since = "0.3.2", note = "Use NativeTheme::pick_variant() instead")]
#[allow(deprecated)]
pub fn pick_variant(
    theme: &native_theme::NativeTheme,
    is_dark: bool,
) -> Option<&native_theme::ThemeVariant> {
    theme.pick_variant(is_dark)
}
```

This function was deprecated in v0.3.2 when `NativeTheme::pick_variant()` was
added to the core crate. It's a trivial delegation that adds nothing. The
`#[allow(deprecated)]` attribute suppresses the warning on the function's own
body, which calls the same method it's deprecated in favor of.

### Option A: Remove it entirely

**Pro:**
- Dead code removed
- Pre-1.0: no backward compatibility obligation
- One fewer function in the public API
- Removes the `#[allow(deprecated)]` workaround
- Users who still call it get a clear compile error pointing to the replacement

**Contra:**
- Any code that hasn't migrated since v0.3.2 will break (but pre-1.0)

### Option B: Keep for one more minor version, then remove

**Pro:**
- Gives any remaining callers one more release to migrate
- Standard deprecation lifecycle

**Contra:**
- Pre-1.0: no SemVer stability promise, so the courtesy is unnecessary
- Adds no value over Option A

### Option C: Keep indefinitely

**Pro:**
- Zero risk to existing callers

**Contra:**
- Dead code forever
- Clutters the public API
- Encourages a worse pattern than the core method
- Pre-1.0: there is no obligation to keep deprecated functions

### PROPOSED: Option A

Remove it. Pre-1.0, there is no backward compatibility obligation. The
function has been deprecated for multiple releases. Any code still using it
gets a clear compile error with a one-word fix (`theme.pick_variant(is_dark)`
instead of `pick_variant(theme, is_dark)`).

---

## 11. Padding helpers return wrong order for iced

**Verdict: VALID -- high priority (correctness trap)**

**Unique to the iced connector** -- no parallel in [GPUI].

```rust
pub fn button_padding(resolved: &ResolvedTheme) -> [f32; 2] {
    [resolved.button.padding_horizontal, resolved.button.padding_vertical]
}
pub fn input_padding(resolved: &ResolvedTheme) -> [f32; 2] {
    [resolved.input.padding_horizontal, resolved.input.padding_vertical]
}
```

These return `[horizontal, vertical]`. But iced's `Padding::from([f32; 2])`
(`iced_core-0.14.0/src/padding.rs:227-236`) maps:

```rust
impl From<[f32; 2]> for Padding {
    fn from(p: [f32; 2]) -> Self {
        Padding { top: p[0], right: p[1], bottom: p[0], left: p[1] }
    }
}
```

So `Padding::from([f32; 2])` expects `[vertical, horizontal]` -- the
**opposite order**. The showcase demonstrates the forced workaround at three
separate locations (`showcase.rs:1400-1401`, `1536-1537`, `1579-1580`):

```rust
let [h, v] = btn_pad;
b.padding(Padding::from([v, h]))  // must swap every time!
```

A user who writes the natural `b.padding(Padding::from(button_padding(&r)))`
gets horizontal padding on top/bottom and vertical padding on left/right.
This is a usability trap that every caller must know to avoid.

### Option A: Return `iced_core::Padding` directly

```rust
pub fn button_padding(resolved: &ResolvedTheme) -> iced_core::Padding {
    iced_core::Padding::from([
        resolved.button.padding_vertical,
        resolved.button.padding_horizontal,
    ])
}
```

**Pro:**
- Zero conversion needed at the call site: `button.padding(button_padding(&r))`
- Impossible to get the order wrong -- `Padding` is a named-field struct
- Matches iced widget API expectations (`.padding()` takes `impl Into<Padding>`)
- The crate already depends on `iced_core`, so no new dependency

**Contra:**
- Couples the return type to iced's `Padding` type (but the crate is an iced
  connector, so coupling to iced types is expected and appropriate)
- Users who need raw `[f32; 2]` for non-iced purposes must extract from
  `Padding` (but those users should call `resolved.button.padding_horizontal`
  directly)

### Option B: Return `[vertical, horizontal]` to match iced convention

```rust
pub fn button_padding(resolved: &ResolvedTheme) -> [f32; 2] {
    [resolved.button.padding_vertical, resolved.button.padding_horizontal]
}
```

**Pro:**
- `Padding::from(button_padding(&r))` works directly
- No type change -- still `[f32; 2]`
- Matches CSS shorthand convention (vertical first)

**Contra:**
- A raw `[f32; 2]` doesn't self-document which element is which
- The doc comment must explicitly state the order (and callers must read it)
- Swapping to `[v, h]` is a silent behavior change -- existing code that
  destructures as `let [h, v] = button_padding(...)` would get the values
  backwards without any compiler warning
- Still possible to get wrong if docs aren't read

### Option C: Keep `[horizontal, vertical]`, document the iced mismatch

**Pro:**
- No code change
- The current order (`[h, v]`) is arguably more intuitive as a standalone
  concept (width before height, x before y)

**Contra:**
- Every caller must swap when passing to iced
- "Document the trap" is not a fix, it's a workaround
- The crate exists to bridge native-theme TO iced -- returning values in
  iced's expected order is the whole point

### Option D: Return a named struct

```rust
pub struct WidgetPadding {
    pub horizontal: f32,
    pub vertical: f32,
}
impl From<WidgetPadding> for iced_core::Padding { ... }
```

**Pro:**
- Named fields prevent order confusion
- `impl From` allows direct use with iced
- Self-documenting at the call site

**Contra:**
- Adds a new type for two floats
- Over-engineered -- `iced_core::Padding` already exists and is the right type
- Users must learn a new type that maps 1:1 to an existing iced type

### PROPOSED: Option A

Return `iced_core::Padding` directly. This is an iced connector crate --
returning iced-native types is the correct design. Callers write
`button.padding(button_padding(&r))` with zero conversion and zero risk
of order confusion. Pre-1.0, the return type change is free.

---

## 12. Missing #[must_use] on all pure functions

**Verdict: VALID -- medium priority**

**No parallel in [GPUI]** (gpui connector was not audited for this).

Every public function in the crate is a pure computation that returns a
value. None have `#[must_use]`. If a caller writes:

```rust
to_theme(&resolved, "name");      // Theme silently discarded
to_svg_handle(&data);             // Handle silently discarded
button_padding(&resolved);        // Padding silently discarded
```

Rust compiles without warning. This is especially dangerous for
`to_theme()` where the computation is non-trivial.

Affected functions (17 total):
- `to_theme()`, `pick_variant()` (lib.rs)
- `to_palette()` (palette.rs)
- `apply_overrides()` (extended.rs -- mutates, but the `&mut` makes this
  less likely to be called as a statement)
- `to_image_handle()`, `to_svg_handle()`, `to_svg_handle_colored()`,
  `custom_icon_to_image_handle()`, `custom_icon_to_svg_handle()`,
  `custom_icon_to_svg_handle_colored()`,
  `animated_frames_to_svg_handles()`, `spin_rotation_radians()`
  (icons.rs)
- `button_padding()`, `input_padding()`, `border_radius()`,
  `border_radius_lg()`, `scrollbar_width()`, `font_family()`,
  `font_size()`, `mono_font_family()`, `mono_font_size()` (lib.rs)

### Option A: Add `#[must_use]` to all pure functions

**Pro:**
- Compiler catches accidental discard of expensive results
- Standard Rust practice for pure functions
- Zero runtime cost -- compile-time only
- Applies to `Option<T>` returns too (already `#[must_use]` via `Option`,
  but explicit annotation on the function is clearer)

**Contra:**
- Minor noise in the source (17 annotations)
- For trivial accessors like `font_size()`, the risk of accidental discard
  is low (but the annotation cost is also near zero)

### Option B: Add `#[must_use]` only to non-trivial functions

Apply to `to_theme()`, `to_palette()`, `animated_frames_to_svg_handles()`,
and icon conversion functions. Skip trivial field accessors.

**Pro:**
- Focuses on functions where accidental discard is most costly
- Less annotation noise

**Contra:**
- Arbitrary boundary between "trivial" and "non-trivial"
- Even trivial accessors can be accidentally discarded in refactoring

### Option C: Add `#![warn(clippy::return_self_not_must_use)]` crate-wide

**Pro:**
- Automated enforcement
- Catches future functions too

**Contra:**
- This lint targets `-> Self` methods, not all pure functions
- Doesn't cover the actual problem

### PROPOSED: Option A

Add `#[must_use]` to all public pure functions. The annotation cost is
negligible and the safety benefit applies uniformly. Trivial accessors
benefit too: a misplaced `font_size(&r);` in a refactoring is silently
wrong without the annotation.

---

## 13. Alpha channel silently discarded in SVG colorization

**Verdict: VALID -- medium priority**

`colorize_monochrome_svg()` (`icons.rs:175-179`) converts `iced_core::Color`
to a hex RGB string:

```rust
let r = (color.r.clamp(0.0, 1.0) * 255.0).round() as u8;
let g = (color.g.clamp(0.0, 1.0) * 255.0).round() as u8;
let b = (color.b.clamp(0.0, 1.0) * 255.0).round() as u8;
let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
```

`color.a` is never read. A user who passes
`Color { r: 1.0, g: 0.0, b: 0.0, a: 0.5 }` expecting semi-transparent red
icons gets fully opaque `fill="#ff0000"`. No documentation mentions this.

### Option A: Use 8-digit hex (#RRGGBBAA) to preserve alpha

```rust
let a = (color.a.clamp(0.0, 1.0) * 255.0).round() as u8;
let hex = format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a);
```

**Pro:**
- Preserves the full color including transparency
- SVG supports `#RRGGBBAA` in modern renderers
- No data loss

**Contra:**
- `#RRGGBBAA` is CSS Color Level 4 syntax -- not universally supported
  in all SVG renderers
- resvg (used by iced for SVG rendering) may or may not support it
- Some parsers interpret 8-digit hex as `#AARRGGBB` (ambiguity)

### Option B: Use `fill` + `fill-opacity` attributes

```rust
let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
let opacity = color.a.clamp(0.0, 1.0);
// inject: fill="hex" fill-opacity="0.5"
```

**Pro:**
- Universally supported SVG syntax
- Separates color from opacity cleanly
- Works in all SVG renderers

**Contra:**
- Requires modifying the injection logic to add two attributes instead of one
- Only applies to the `fill` attribute -- `currentColor` replacement can't
  set opacity this way (need `style="opacity: 0.5"` on the element)
- More complex code for the fill injection path

### Option C: Document the limitation, ignore alpha

Add a doc comment to `to_svg_handle_colored()` and
`custom_icon_to_svg_handle_colored()`:

> **Note:** Only the RGB channels of `color` are used. The alpha channel is
> ignored -- colorized SVGs are always fully opaque. Use iced's
> `Svg::opacity()` to control icon transparency.

**Pro:**
- Honest with callers
- iced provides `Svg::opacity()` as the correct way to control SVG
  transparency, so the limitation is not a dead end
- Zero code complexity increase
- Bundled icon sets (Material, Lucide) are designed as opaque icons

**Contra:**
- Users must know to use a separate iced API for transparency
- The `color` parameter's alpha channel is silently dead weight

### Option D: Accept `iced_core::Color` but only use RGB; add a separate opacity parameter

```rust
pub fn to_svg_handle_colored(
    data: &IconData,
    color: iced_core::Color,
    opacity: Option<f32>,
) -> Option<iced_core::svg::Handle>
```

**Pro:**
- Explicit opacity parameter
- Caller can't accidentally think `color.a` controls it

**Contra:**
- Adds a parameter to the signature
- iced's `Svg::opacity()` already controls this at the widget level
- Conflating SVG-level fill opacity with widget-level opacity is confusing

### PROPOSED: Option C

Document the limitation. The correct way to make an iced SVG icon
semi-transparent is `Svg::new(handle).opacity(0.5)`, which works at the
widget rendering level regardless of the SVG content. Baking alpha into the
SVG fill is fragile and less composable. The doc comment makes the behavior
explicit.

---

## 14. Incomplete font helper set

**Verdict: VALID -- low priority**

`ResolvedFontSpec` has three fields:

```rust
pub struct ResolvedFontSpec {
    pub family: String,   // exposed via font_family(), mono_font_family()
    pub size: f32,        // exposed via font_size(), mono_font_size()
    pub weight: u16,      // NOT exposed
}
```

The connector exposes `family` and `size` for both fonts but not `weight`.
Font weight is commonly needed alongside family and size -- iced's `Font`
struct has a `weight` field, and CSS-style text rendering requires all three.

### Option A: Add `font_weight()` and `mono_font_weight()` helpers

```rust
pub fn font_weight(resolved: &ResolvedTheme) -> u16 {
    resolved.defaults.font.weight
}
pub fn mono_font_weight(resolved: &ResolvedTheme) -> u16 {
    resolved.defaults.mono_font.weight
}
```

**Pro:**
- Completes the set: every `ResolvedFontSpec` field has a corresponding helper
- Consistent with the existing `font_family()` / `font_size()` pattern
- `u16` maps directly to iced's `font::Weight` values

**Contra:**
- Two more trivial one-liner accessors
- Users can read `resolved.defaults.font.weight` directly
- iced's `Font` struct accepts `Weight` enum variants (Normal, Bold, etc.),
  not raw `u16` -- a raw `u16` helper doesn't integrate cleanly

### Option B: Return `iced_core::Font` struct directly

```rust
pub fn font(resolved: &ResolvedTheme) -> iced_core::Font {
    iced_core::Font {
        family: iced_core::font::Family::Name(
            resolved.defaults.font.family.clone().leak()
        ),
        weight: iced_core::font::Weight::from(resolved.defaults.font.weight),
        ..Default::default()
    }
}
```

**Pro:**
- Returns a ready-to-use iced `Font` -- callers write `text.font(font(&r))`
- Bundles family + size + weight into one value
- Same philosophy as section 11: return iced-native types from an iced connector

**Contra:**
- `family.clone().leak()` leaks memory (iced's `Family::Name` takes `&'static str`)
- `Font` doesn't include `size` (that's set separately on text widgets)
- `Weight::from(u16)` may not exist -- would need a manual mapping

### Option C: Add `line_height()` helper too

If adding font helpers, also add:

```rust
pub fn line_height(resolved: &ResolvedTheme) -> f32 {
    resolved.defaults.line_height
}
```

**Pro:**
- Line height is commonly needed alongside font family/size/weight
- Completes the typography helper set

**Contra:**
- Another one-liner accessor

### PROPOSED: Option A + C

Add `font_weight()`, `mono_font_weight()`, and `line_height()` helpers. This
completes the typography accessor set. Return raw values (`u16`, `f32`) rather
than iced types because iced's `Font` struct has a `&'static str` family name
requirement that doesn't compose well with `ResolvedFontSpec`'s owned `String`.

---

## 15. README contradicts source on font size conversion

**Verdict: VALID -- low priority (documentation bug)**

README line 59:
> `font_size(resolved)` -- primary UI font size in pixels (converted from points)

Source code `lib.rs:125-126`:
> ResolvedFontSpec.size is already in logical pixels -- no pt-to-px conversion
> is applied.

These directly contradict each other. Either font sizes undergo pt-to-px
conversion somewhere in the pipeline, or the README is wrong. The source
code comment is authoritative.

### Option A: Fix the README

Change the README line to:

> `font_size(resolved)` -- primary UI font size in logical pixels

**Pro:**
- Matches the source code
- Accurate documentation
- One-line fix

**Contra:**
- None

### PROPOSED: Option A

Fix the README. The source code is correct -- `ResolvedFontSpec.size` is
already in logical pixels after the resolution pipeline. No conversion
happens in the connector.

---

## 16. to_color() inaccessible to external callers

**Verdict: VALID -- low priority**

`palette.rs:10`:

```rust
pub(crate) fn to_color(rgba: Rgba) -> iced_core::Color {
    let [r, g, b, a] = rgba.to_f32_array();
    iced_core::Color { r, g, b, a }
}
```

Power users who build custom iced themes from `ResolvedTheme` fields need
this conversion. For example, a user who wants to use
`resolved.input.caret` (not mapped by the connector) as an iced `Color`
must reimplement this trivial function themselves.

### Option A: Make `to_color()` public and re-export it

```rust
// palette.rs
pub fn to_color(rgba: Rgba) -> iced_core::Color { ... }

// lib.rs (optional convenience re-export)
pub use palette::to_color;
```

**Pro:**
- Power users can convert any `Rgba` field to `iced_core::Color`
- Useful building block for custom theme construction
- The function is already public to the crate -- making it externally
  public is a one-word change

**Contra:**
- Adds a function to the public API surface
- `Rgba` to `Color` conversion is trivial (4-line function) -- users
  could write it themselves
- Commits to supporting the function across versions

### Option B: Implement `From<Rgba> for iced_core::Color` (core change)

Add the conversion in native-theme's core crate (behind a feature flag
gated on `iced_core`).

**Pro:**
- Idiomatic Rust: `let color: Color = rgba.into()`
- No connector function needed

**Contra:**
- Couples native-theme core to iced_core (even behind a feature flag)
- Feature-flagged trait impls are a maintenance burden
- native-theme core should not know about iced

### Option C: Keep `pub(crate)`, document the conversion recipe

Add to the crate docs:

```rust
// To convert any Rgba to iced Color:
let [r, g, b, a] = rgba.to_f32_array();
let color = iced_core::Color { r, g, b, a };
```

**Pro:**
- No API change
- The conversion is trivial enough to inline

**Contra:**
- Every power user copies the same 2 lines
- If `Rgba::to_f32_array()` changes, the recipe breaks silently

### PROPOSED: Option A

Make `to_color()` public. It's a one-word change (`pub(crate)` -> `pub`)
that gives power users a useful building block. The function is trivial but
worth exposing because it's the canonical way to bridge native-theme's color
type to iced's color type within this connector crate.

---

## 17. spin_rotation_radians mixes Duration and u32 parameter types

**Verdict: VALID -- low priority**

```rust
pub fn spin_rotation_radians(
    elapsed: std::time::Duration,
    duration_ms: u32,
) -> iced_core::Radians
```

Both parameters represent durations but use different types. `elapsed` is
the idiomatic `std::time::Duration`; `duration_ms` is a raw `u32` in
milliseconds. The inconsistency forces callers to think about two different
time representations in one call.

The `u32` comes from `TransformAnimation::Spin { duration_ms: u32 }` in
native-theme's core types.

### Option A: Accept `Duration` for both parameters

```rust
pub fn spin_rotation_radians(
    elapsed: Duration,
    full_rotation: Duration,
) -> Radians
```

Callers convert: `spin_rotation_radians(elapsed, Duration::from_millis(duration_ms as u64))`

**Pro:**
- Consistent parameter types
- Self-documenting: both are `Duration`
- No precision loss from `u32` -> `f32` conversion (Duration handles this
  internally)

**Contra:**
- Callers must wrap `duration_ms` in `Duration::from_millis()` every time
- The value comes from `TransformAnimation::Spin { duration_ms: u32 }`,
  so every caller does `Duration::from_millis(spin.duration_ms as u64)` --
  more ceremony, not less
- `as u64` cast is required since `from_millis` takes `u64`

### Option B: Accept `u32` for both parameters

```rust
pub fn spin_rotation_radians(
    elapsed_ms: u32,
    duration_ms: u32,
) -> Radians
```

Callers convert: `spin_rotation_radians(elapsed.as_millis() as u32, duration_ms)`

**Pro:**
- Consistent parameter types
- Matches the source data format (`TransformAnimation::Spin`)
- No `Duration` construction overhead

**Contra:**
- `elapsed.as_millis()` returns `u128` -- `as u32` truncates after ~49 days
  (acceptable for animation purposes but technically lossy)
- Less idiomatic than `Duration`
- Loses sub-millisecond precision from `Duration`

### Option C: Keep as-is, document the rationale

**Pro:**
- Each parameter matches its source type: `elapsed` comes from iced's
  `Instant::elapsed()` (returns `Duration`); `duration_ms` comes from
  `TransformAnimation::Spin` (stores `u32`)
- No conversion needed at either call site
- Pragmatic: the API matches how the data flows in practice

**Contra:**
- Inconsistent types in the same function signature
- Readers must look at two different time systems

### PROPOSED: Option C

Keep as-is. The parameter types match their respective source types, which
minimizes conversion at call sites. `elapsed` is always from
`Instant::elapsed()` (returns `Duration`), and `duration_ms` is always from
`TransformAnimation::Spin { duration_ms }` (stores `u32`). Forcing either
type on the other parameter adds a conversion step for every caller. The
inconsistency is pragmatic, not accidental.

---

## 18. spin_rotation_radians produces NaN on zero duration

**Verdict: VALID -- medium priority (correctness bug)**

**Unique to the iced connector** -- the gpui connector's `with_spin_animation`
does not expose raw duration arithmetic.

`icons.rs:164-166`:

```rust
pub fn spin_rotation_radians(elapsed: std::time::Duration, duration_ms: u32) -> iced_core::Radians {
    let progress = (elapsed.as_millis() as f32 % duration_ms as f32) / duration_ms as f32;
    iced_core::Radians(progress * std::f32::consts::TAU)
}
```

When `duration_ms = 0`:

```
500.0f32 % 0.0f32 = NaN
NaN / 0.0f32 = NaN
NaN * TAU = NaN
→ Radians(NaN)
```

Verified with Rust: `f32` modulo zero produces `NaN` (IEEE 754), which
propagates through the division and multiplication. An iced `Svg` widget
receiving `Rotation::Floating(Radians(NaN))` has undefined rendering behavior
-- the element may disappear, render at an arbitrary angle, or cause a panic
in the rendering backend.

`TransformAnimation::Spin { duration_ms: 0 }` is nonsensical but
constructable. The function has no guard and no documentation of this
precondition.

### Option A: Return `Radians(0.0)` for `duration_ms == 0`

```rust
pub fn spin_rotation_radians(elapsed: Duration, duration_ms: u32) -> Radians {
    if duration_ms == 0 {
        return iced_core::Radians(0.0);
    }
    let progress = (elapsed.as_millis() as f32 % duration_ms as f32) / duration_ms as f32;
    iced_core::Radians(progress * std::f32::consts::TAU)
}
```

**Pro:**
- No NaN in the output -- always returns a valid angle
- `Radians(0.0)` means "no rotation", which is a sensible default for a
  nonsensical input (zero-duration spin = no spin)
- One-line guard, minimal change
- Callers never see NaN regardless of input

**Contra:**
- Silently masks a bogus `duration_ms = 0` instead of surfacing the error
- Callers who pass 0 by accident get `Radians(0.0)` with no indication
  that their animation isn't working (silent success instead of silent failure)

### Option B: Document the precondition, do not guard

Add to the doc comment:

> **Panics** (debug) / **Produces `Radians(NaN)`** (release) if
> `duration_ms` is 0.

Optionally add a `debug_assert!(duration_ms > 0)` to catch it in testing.

**Pro:**
- Makes the precondition explicit
- `debug_assert!` catches it during development
- No silent masking of bogus inputs
- Follows the Rust convention of documenting invariants rather than
  silently absorbing invalid arguments

**Contra:**
- Release builds still produce NaN
- Callers who don't read docs hit undefined rendering behavior
- `debug_assert!` panics in debug builds, which may not be desired

### Option C: Return `Option<Radians>`, with `None` for `duration_ms == 0`

```rust
pub fn spin_rotation_radians(elapsed: Duration, duration_ms: u32) -> Option<Radians>
```

**Pro:**
- Forces callers to handle the zero case
- Type-level safety -- impossible to accidentally produce NaN

**Contra:**
- Breaking change to return type (but pre-1.0, free)
- Every caller must unwrap or provide a default
- Over-engineered for a single edge case that should never happen in
  practice -- `TransformAnimation::Spin` is always created with a
  non-zero duration
- Makes the happy path more verbose for no practical benefit

### Option D: Clamp `duration_ms` to a minimum of 1

```rust
let duration_ms = duration_ms.max(1);
```

**Pro:**
- One-character fix (`max(1)`)
- Never produces NaN
- A 1ms spin duration produces valid (if absurd) rotation values

**Contra:**
- `duration_ms = 0` becomes `duration_ms = 1`, which produces a
  1000-RPM spin -- visible and absurd, but at least not NaN
- Silently changes the input

### PROPOSED: Option A

Return `Radians(0.0)` for `duration_ms == 0`. The semantic of "zero-duration
spin" is "no spin", and `Radians(0.0)` is the correct representation of no
rotation. This is a defensive guard, not silent masking -- the input is
definitionally nonsensical and the output is the most sensible interpretation.
A `debug_assert!(duration_ms > 0)` can be added alongside to catch it during
development.

---

## 19. colorize_monochrome_svg corrupts self-closing SVG tags

**Verdict: VALID -- low priority (correctness bug, rare in practice)**

**Related to section 6** (colorize limitations). This is a separate bug
in the fill injection code path.

`icons.rs:190-201` injects `fill="..."` immediately before the `>` of the
root `<svg>` tag:

```rust
if let Some(pos) = svg_str.find("<svg")
    && let Some(close) = svg_str[pos..].find('>')
{
    let tag_end = pos + close;
    let tag = &svg_str[pos..tag_end];
    if !tag.contains("fill=") {
        let mut result = String::with_capacity(svg_str.len() + 20);
        result.push_str(&svg_str[..tag_end]);
        result.push_str(&format!(" fill=\"{}\"", hex));
        result.push_str(&svg_str[tag_end..]);
        return result.into_bytes();
    }
}
```

For self-closing tags (`<svg ... />`), `tag_end` points to just before `>`,
which is after the `/`. The fill is injected between `/` and `>`, producing
malformed XML:

```
Input:  <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24"/>
Output: <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24"/ fill="#ff0000">
```

The `/ fill="#ff0000">` is invalid XML. The `/` must come after all
attributes, immediately before `>`.

Verified by running the exact function logic with self-closing SVG inputs.

In practice this is rare: a self-closing `<svg/>` is an empty SVG with no
visible content, so colorizing it is pointless. But the function doesn't
reject self-closing tags -- it silently produces broken output.

### Option A: Detect and handle self-closing tags

```rust
let tag_end = pos + close;
let tag = &svg_str[pos..tag_end];
if !tag.contains("fill=") {
    // Check for self-closing tag: if the character before '>' is '/',
    // inject before the '/' instead of before '>'.
    let inject_pos = if svg_str.as_bytes().get(tag_end.wrapping_sub(1)) == Some(&b'/') {
        tag_end - 1
    } else {
        tag_end
    };
    let mut result = String::with_capacity(svg_str.len() + 20);
    result.push_str(&svg_str[..inject_pos]);
    result.push_str(&format!(" fill=\"{}\"", hex));
    result.push_str(&svg_str[inject_pos..]);
    return result.into_bytes();
}
```

**Pro:**
- Handles both `<svg>` and `<svg/>` correctly
- Produces valid XML in all cases: `<svg ... fill="#ff0000"/>`
- Small change to existing logic

**Contra:**
- Adds complexity for an edge case that is meaningless in practice
  (a self-closing SVG has no visual content to colorize)
- Must handle whitespace variations: `<svg ... / >` (space before `>`)

### Option B: Return input unchanged for self-closing SVGs

If the tag ends with `/>`, skip fill injection entirely (the SVG has no
content to render, so colorization is moot).

```rust
if tag.ends_with('/') {
    return svg_bytes.to_vec(); // self-closing, nothing to colorize
}
```

**Pro:**
- Simplest fix -- two-line guard
- No malformed XML produced
- Correct semantics: an empty SVG doesn't benefit from a fill attribute

**Contra:**
- A self-closing SVG with a referenced external resource could
  theoretically benefit from a fill (extremely unlikely)
- Silent no-op rather than a fix

### Option C: Return input unchanged and document the limitation

Add to the doc comment:

> Self-closing `<svg/>` tags are returned unchanged (no fill injection).

**Pro:**
- No code change beyond documentation
- Explicitly states the behavior

**Contra:**
- The function still silently does the wrong thing if Option B's guard
  is not added
- Documentation alone doesn't prevent malformed output

### Option D: Fix the injection point (Option A) and also handle `currentcolor` case sensitivity

Combine the self-closing fix with a case-insensitive `currentColor` match
(currently the function only matches exact camelCase `currentColor`,
missing `currentcolor` or `CURRENTCOLOR` which are valid per CSS spec):

```rust
let svg_lower = svg_str.to_ascii_lowercase();
if svg_lower.contains("currentcolor") {
    // case-insensitive replacement using char indices
    ...
}
```

**Pro:**
- Fixes two related bugs in one pass
- Case-insensitive `currentColor` handles more third-party SVGs

**Contra:**
- More complex change
- Case-insensitive replacement on strings is non-trivial without regex
- The `currentColor` casing issue affects only third-party SVGs, not
  the bundled icon sets this function targets
- Scope creep: two fixes in one section makes review harder

### PROPOSED: Option A

Fix the injection point to handle self-closing tags. The fix is small (check
if the character before `>` is `/` and inject before it) and produces correct
XML in all cases. Option B (skip self-closing SVGs entirely) is simpler but
less correct -- Option A makes the function work correctly for all valid SVG
inputs, not just the common ones.

Leave `currentColor` case sensitivity as a sub-note in section 6 -- it's a
separate limitation of the string replacement approach, not a corruption bug.

---

## 20. Icon conversion functions force unnecessary clones

**Verdict: VALID -- low priority (performance)**

**Unique to the iced connector** -- the gpui connector's icon functions have
the same pattern but gpui's `ImageSource` ownership model differs.

`to_image_handle()` (`icons.rs:15-28`) and `to_svg_handle()` (`icons.rs:35-40`)
take `&IconData` by reference and must clone the inner data to create owned
iced handles:

```rust
pub fn to_image_handle(data: &IconData) -> Option<iced_core::image::Handle> {
    match data {
        IconData::Rgba { width, height, data } => Some(
            iced_core::image::Handle::from_rgba(*width, *height, data.clone()) // <-- clone
        ),
        _ => None,
    }
}

pub fn to_svg_handle(data: &IconData) -> Option<iced_core::svg::Handle> {
    match data {
        IconData::Svg(bytes) => Some(
            iced_core::svg::Handle::from_memory(bytes.clone()) // <-- clone
        ),
        _ => None,
    }
}
```

`Handle::from_rgba()` takes `Vec<u8>` (ownership required). `Handle::from_memory()`
takes `impl Into<Cow<'static, [u8]>>` (also effectively needs ownership).
Since the functions borrow `&IconData`, they must clone.

For a 256x256 RGBA icon, `data.clone()` allocates and copies 256KB. For
`animated_frames_to_svg_handles()`, every frame's SVG bytes are cloned.
Callers who have an owned `IconData` they no longer need cannot avoid the
allocation -- there is no consuming variant.

### Option A: Add consuming variants alongside borrowing ones

```rust
pub fn into_image_handle(data: IconData) -> Option<iced_core::image::Handle> {
    match data {
        IconData::Rgba { width, height, data } => Some(
            iced_core::image::Handle::from_rgba(width, height, data) // moved, no clone
        ),
        _ => None,
    }
}

pub fn into_svg_handle(data: IconData) -> Option<iced_core::svg::Handle> {
    match data {
        IconData::Svg(bytes) => Some(
            iced_core::svg::Handle::from_memory(bytes) // moved, no clone
        ),
        _ => None,
    }
}
```

**Pro:**
- Zero-copy path for callers with owned `IconData`
- Borrowing variants remain for callers who need the `IconData` afterward
- Follows Rust convention: `to_X` borrows, `into_X` consumes
- Significant performance win for large RGBA icons and bulk conversion

**Contra:**
- Doubles the icon conversion function count (6 -> 12 with colored variants)
- More API surface to maintain and document
- Callers must choose between `to_` and `into_` (minor cognitive load)

### Option B: Replace borrowing functions with consuming ones

Change `to_image_handle(&IconData)` to `to_image_handle(IconData)`.
Callers who need to keep the `IconData` clone it themselves.

**Pro:**
- No API surface growth -- same function count
- Zero-copy by default (the common case)
- Callers who need both the `IconData` and the `Handle` explicitly clone,
  making the allocation visible

**Contra:**
- Breaking change (but pre-1.0, free)
- Forces callers who DO need the `IconData` afterward to clone before
  calling (moves the clone to the caller, not the function)
- Less ergonomic for callers who iterate over `&[IconData]` (must clone
  each element to call the consuming function)
- `animated_frames_to_svg_handles` would need to consume the `AnimatedIcon`,
  which is awkward if the caller also needs the timing data

### Option C: Accept `impl Into<IconData>` or `Cow<IconData>`

```rust
pub fn to_image_handle(data: impl Into<IconData>) -> Option<Handle>
```

**Pro:**
- Single function that accepts both owned and borrowed data (if
  `&IconData` implements `Into<IconData>` via clone)

**Contra:**
- `&IconData` doesn't implement `Into<IconData>` without manual impls
- `Cow<'_, IconData>` is unusual and complex for the caller
- `impl Into<IconData>` forces a conversion even for the borrowing case
- Over-engineered for the actual use pattern

### Option D: Keep borrowing only, document the clone cost

**Pro:**
- No API change
- The clone cost is real but bounded: icons are typically small
  (24x24 RGBA = 2.3KB, SVG = 1-5KB)
- iced caches rendered SVGs and images internally, so the clone happens
  once at load time, not per-frame
- Users who need zero-copy can call `Handle::from_rgba` / `Handle::from_memory`
  directly

**Contra:**
- 256x256 RGBA icons (HiDPI) are 256KB -- not negligible
- `animated_frames_to_svg_handles` clones every frame, multiplying the cost
- "Users can call iced APIs directly" undermines the connector's value

### PROPOSED: Option A

Add consuming `into_` variants. This follows Rust convention (`to_` borrows,
`into_` consumes), provides a zero-copy path where it matters, and doesn't
break existing code. The API surface growth is manageable because the `into_`
variants are exact mirrors of the `to_` variants with the same signatures
minus the `&`.

For `animated_frames_to_svg_handles`, add a consuming variant that takes
`AnimatedIcon` and moves the frame data:

```rust
pub fn into_animated_svg_handles(anim: AnimatedIcon) -> Option<AnimatedSvgHandles>
```

This pairs naturally with the timing-data struct from section 5.

---

## Priority Summary

| Priority | # | Problem | Proposed Fix | Dependency |
|----------|---|---------|--------------|------------|
| **HIGH** | 1 | Excessive ceremony | Add `from_preset()` convenience | **[CORE-3]** |
| **HIGH** | 11 | Padding order trap | Return `iced_core::Padding` directly | -- |
| **MEDIUM** | 3 | No SystemTheme convenience | Extension trait + `from_system()` | -- |
| **MEDIUM** | 5 | Animated timing lost | Return `AnimatedSvgHandles` struct | -- |
| **MEDIUM** | 7 | Code duplication / doc lie | Call `apply_overrides()` from closure | -- |
| **MEDIUM** | 8 | Silent data loss | Document coverage gap in crate docs | -- |
| **MEDIUM** | 12 | Missing `#[must_use]` | Add to all 17 pure functions | -- |
| **MEDIUM** | 13 | Alpha silently dropped | Document: use `Svg::opacity()` instead | -- |
| **MEDIUM** | 18 | `spin_rotation_radians` NaN | Return `Radians(0.0)` for `duration_ms == 0` | -- |
| **LOW** | 2 | Mandatory name | Keep `name` param, aligned with [GPUI-3] | -- |
| **LOW** | 4 | Suffix explosion | Merge to `Option<Color>` | -- |
| **LOW** | 6 | colorize misses black fills | Add `fill="black"` / `fill="#000000"` | -- |
| **LOW** | 9 | No re-exports | Re-export `native_theme` types | -- |
| **LOW** | 10 | Deprecated `pick_variant()` | Remove it (pre-1.0) | -- |
| **LOW** | 14 | Incomplete font helpers | Add `font_weight()`, `line_height()` | -- |
| **LOW** | 15 | README contradiction | Fix README font size description | -- |
| **LOW** | 16 | `to_color()` private | Make `pub` | -- |
| **LOW** | 17 | Mixed Duration/u32 | Keep as-is (pragmatic match) | -- |
| **LOW** | 19 | Self-closing SVG corruption | Fix injection point for `/>` tags | -- |
| **LOW** | 20 | Unnecessary icon clones | Add consuming `into_` variants | -- |

## Implementation Order

1. **Section 11**: Fix padding return type to `iced_core::Padding` (correctness trap)
2. **Section 18**: Guard `spin_rotation_radians` against `duration_ms == 0`
3. **Section 7**: Fix code duplication (call `apply_overrides()` from closure, fix doc comment)
4. **Section 12**: Add `#[must_use]` to all pure functions
5. **Section 10**: Remove deprecated `pick_variant()`
6. **Section 15**: Fix README font size description
7. **Section 2**: Keep `name` parameter on `to_theme()` (aligned with [GPUI-3])
8. **Section 13**: Document alpha channel behavior in colorization docs
9. **Sections 6 + 19**: Fix `colorize_monochrome_svg` (black fills + self-closing tags)
10. **Section 16**: Make `to_color()` public
11. **Section 14**: Add `font_weight()`, `mono_font_weight()`, `line_height()` helpers
12. **Section 5**: Return `AnimatedSvgHandles` struct
13. **Section 4**: Merge `_colored` suffix variants into `Option<Color>`
14. **Section 20**: Add consuming `into_` icon conversion variants
15. **Section 9**: Add `native_theme` re-exports
16. **Wait for [CORE-3]**: `ThemeVariant::into_resolved()` in native-theme
17. **Section 1**: Add `from_preset()`
18. **Section 3**: Add `SystemThemeExt` trait + `from_system()`
19. **Section 8**: Write coverage documentation

## Issues from [GPUI] that do NOT apply to the iced connector

For completeness, here are the [GPUI] issues that were evaluated and found
inapplicable to native-theme-iced, with the reason:

| GPUI # | Issue | Why N/A |
|--------|-------|---------|
| 2 | is_dark split-brain bug | `to_theme()` has no `is_dark` parameter |
| 4 | Stringly-typed icon params | Only `icon_set: &str` exists; keeping `&str` is correct because `IconProvider` accepts arbitrary set names |
| 5 | Naming inconsistency | Iced icon naming is consistent (`to_X` / `custom_icon_to_X`) |
| 7 | Rendering in mapping layer | `spin_rotation_radians()` returns pure `Radians` data, not DOM elements |
| 10 | too_many_arguments | No complex private helpers |
| 12 | Monochromatic chart colors | iced has no chart color slots in its theme system |
| 14 | Hardcoded magenta/overlay | gpui-component-specific `ThemeColor` fields |
| 15 | No link hover/active | No link state colors in iced's palette |
| 16 | Selection alpha clamped | No selection field in iced's palette |
| 18 | Dead parameters | No dead parameters in any function |
| 19 | accent_foreground wrong source | No equivalent field in iced's palette |
| 20 | Radius truncated | Returns `f32` directly, no integer casting |
| 21 | Silently broken images | Returns `Option<Handle>` correctly |
| 23 | SVG rasterize hardcoded 48px | iced renders SVGs natively, no rasterization step |
| 24 | Icon mapping coverage tests | No icon name mapping tables |
| 25 | ThemeColor field coverage | No `ThemeColor` mutation pattern |
| 26 | hover_color doc typo | No equivalent helper function |
