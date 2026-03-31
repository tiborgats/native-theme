# v0.5.2 API Review -- native-theme-iced (iced connector)

Verified against source code on 2026-03-31. Each chapter covers one
API problem, lists all resolution options with full pro/contra, and
recommends the best solution. Pre-1.0 -- backward compatibility is
not a constraint.

**Companion documents:**
- [todo_v0.5.2_native-theme_api.md](todo_v0.5.2_native-theme_api.md)
- [todo_v0.5.2_native-theme-build_api.md](todo_v0.5.2_native-theme-build_api.md)
- [todo_v0.5.2_native-theme-gpui_api.md](todo_v0.5.2_native-theme-gpui_api.md)

---

## Summary

| # | Issue | Severity | Fix complexity |
|---|-------|----------|----------------|
| 1 | `from_preset`, `from_system`, and `SystemThemeExt::to_iced_theme()` missing `#[must_use]` | Medium | Trivial |
| 2 | `AnimatedSvgHandles` missing derives | Medium | Trivial |
| 3 | `from_preset` / `from_system` drop `ResolvedThemeVariant` | Medium | Low |

---

## 1. from_preset, from_system, and to_iced_theme missing #[must_use]

**File:** `connectors/native-theme-iced/src/lib.rs:107,121,129`

**What:** Three public functions lack `#[must_use]`:

```rust
// line 107 -- no #[must_use]
pub fn from_preset(name: &str, is_dark: bool) -> native_theme::Result<iced_core::theme::Theme> {

// line 121 -- no #[must_use]
pub fn from_system() -> native_theme::Result<iced_core::theme::Theme> {

// line 129 -- no #[must_use] on trait method
pub trait SystemThemeExt {
    fn to_iced_theme(&self) -> iced_core::theme::Theme;
}
```

Meanwhile, every other public function in the crate has `#[must_use]`
(23 functions across `lib.rs`, `icons.rs`, and `palette.rs`):

| Function | `#[must_use]` |
|----------|:-------------:|
| `to_theme` | yes |
| `button_padding` | yes |
| `input_padding` | yes |
| `border_radius` | yes |
| `border_radius_lg` | yes |
| `scrollbar_width` | yes |
| `font_family` | yes |
| `font_size` | yes |
| `mono_font_family` | yes |
| `mono_font_size` | yes |
| `font_weight` | yes |
| `mono_font_weight` | yes |
| `line_height` | yes |
| All icon functions (10) | yes |
| All palette functions (2) | yes |
| `from_preset` | **no** |
| `from_system` | **no** |
| `SystemThemeExt::to_iced_theme` | **no** |

All three functions perform real work and return values that should
never be discarded. Calling `from_system()?;` (note the semicolon)
silently throws away the theme -- a bug that `#[must_use]` would
catch at compile time.

Note: the core crate's `SystemTheme::from_system()` correctly has
`#[must_use = "this returns the detected theme; it does not apply it"]`,
and the gpui connector's `to_gpui_theme()` also lacks it (see
**[GPUI-1]**).

### Options

**A. Add `#[must_use]` with custom messages to all three (recommended)**

```rust
#[must_use = "this returns the theme; it does not apply it"]
pub fn from_preset(name: &str, is_dark: bool) -> native_theme::Result<iced_core::theme::Theme> {

#[must_use = "this returns the theme; it does not apply it"]
pub fn from_system() -> native_theme::Result<iced_core::theme::Theme> {

pub trait SystemThemeExt {
    #[must_use = "this returns the theme; it does not apply it"]
    fn to_iced_theme(&self) -> iced_core::theme::Theme;
}
```

- Pro: Catches the discard-result bug at compile time.
- Pro: Consistent with every other function in the crate.
- Pro: Consistent with the core crate's `from_system()`.
- Pro: The custom message matches the core crate's convention and
  clarifies that the return value is the actual theme, not a
  side-effect.
- Pro: Zero runtime cost.
- Con: None. `#[must_use]` is purely a lint, never a breaking change.

**B. Add bare `#[must_use]` (no custom message)**

- Pro: Simpler, still catches discard bugs.
- Con: The default `must_use` warning on `Result` is already clear
  for `from_preset` and `from_system`. But `to_iced_theme()` returns
  `Theme` directly (not `Result`), so the bare `#[must_use]` message
  ("unused ... that must be used") is less helpful than a custom one.

**C. Keep as-is**

- Pro: No change.
- Con: Inconsistent with 23 other functions in the same crate.
- Con: Misses a free compile-time safety check.

### Recommendation

**Option A.** The custom message `"this returns the theme; it does not
apply it"` matches the core crate's convention and helps users who
don't immediately understand why the result matters. Including the
trait method `to_iced_theme()` is especially important since its
return type is `Theme` (not `Result`), so there is no built-in
`Result` warning to fall back on.

---

## 2. AnimatedSvgHandles missing derives

**File:** `connectors/native-theme-iced/src/icons.rs:14`

**What:** `AnimatedSvgHandles` has no derive macros at all:

```rust
pub struct AnimatedSvgHandles {
    /// SVG handles ready for iced rendering.
    pub handles: Vec<iced_core::svg::Handle>,
    /// Duration of each frame in milliseconds.
    pub frame_duration_ms: u32,
}
```

This is the only public struct in the crate without any derives.
Compare:
- Core crate `IconData`: `Debug, Clone, PartialEq, Eq`
- Core crate `AnimatedIcon`: `Debug, Clone, PartialEq, Eq`
- gpui crate `AnimatedImageSources`: also missing (same issue,
  **[GPUI-2]**)

Without `Debug`, the struct can't be inspected via `dbg!()` or
`{:?}` formatting. Without `Clone`, users who need the handles in
multiple places (e.g., caching + rendering) must re-generate them.

The derives that can be added depend on what `iced_core::svg::Handle`
implements. This needs verification.

### Options

**A. Add `Debug` and `Clone` if Handle supports them (recommended)**

```rust
#[derive(Debug, Clone)]
pub struct AnimatedSvgHandles { ... }
```

- Pro: `Debug` is essential for any public type -- standard Rust API
  guideline (C-DEBUG in the API checklist).
- Pro: `Clone` enables caching patterns (clone handles, store in
  multiple widgets).
- Pro: `iced_core::svg::Handle` implements both `Debug` and `Clone`
  (it wraps `Arc<Data>` -- cheap to clone, printable).
- Con: If a future iced version removes `Clone` or `Debug` from
  `Handle`, the derive would break. This is unlikely for such a
  fundamental type.

**B. Add `Debug` only (minimal fix)**

- Pro: Satisfies the C-DEBUG API guideline.
- Pro: No dependency on `Handle` being `Clone`.
- Con: Users still can't clone the struct. Caching requires calling
  `animated_frames_to_svg_handles()` again (re-parsing SVG handles).

**C. Add `Debug` via manual impl, skip `Clone`**

```rust
impl fmt::Debug for AnimatedSvgHandles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnimatedSvgHandles")
            .field("handles_count", &self.handles.len())
            .field("frame_duration_ms", &self.frame_duration_ms)
            .finish()
    }
}
```

- Pro: Works even if `Handle` doesn't implement `Debug`.
- Pro: Shows handle count instead of potentially large handle data.
- Con: More code than a derive.
- Con: Diverges from peer types that show full data in Debug output.

**D. Keep as-is**

- Pro: No change.
- Con: Violates Rust API guideline C-DEBUG ("almost every public type
  should implement Debug").
- Con: Users cannot inspect animation state during debugging.

### Recommendation

**Option A.** `iced_core::svg::Handle` implements both `Debug` and
`Clone` (it wraps `Arc<Data>` -- cheap to clone, printable). The
derives are simple and expected. This should be applied
simultaneously with the identical fix for `AnimatedImageSources` in
the gpui connector (**[GPUI-2]**).

---

## 3. from_preset / from_system drop ResolvedThemeVariant

**File:** `connectors/native-theme-iced/src/lib.rs:107-114,121-124`

**What:** Both convenience functions perform the full
load-resolve-validate pipeline but return only the iced `Theme`,
discarding the `ResolvedThemeVariant`:

```rust
pub fn from_preset(name: &str, is_dark: bool) -> native_theme::Result<iced_core::theme::Theme> {
    let spec = native_theme::ThemeSpec::preset(name)?;
    let variant = spec.pick_variant(is_dark).ok_or_else(|| ...)?;
    let resolved = variant.clone().into_resolved()?;
    Ok(to_theme(&resolved, name))
    //          ^^^^^^^^^ resolved dropped here
}

pub fn from_system() -> native_theme::Result<iced_core::theme::Theme> {
    let sys = native_theme::SystemTheme::from_system()?;
    Ok(to_theme(sys.active(), &sys.name))
    //          ^^^^^^^^^^^ sys dropped here
}
```

The iced `Theme` contains only 6 palette colors + 4 extended palette
overrides. All geometry, font, and widget metrics live in the
`ResolvedThemeVariant`. The crate provides 12 helper functions that
read metrics from the resolved variant:

- `button_padding(&resolved)`, `input_padding(&resolved)`
- `border_radius(&resolved)`, `border_radius_lg(&resolved)`
- `scrollbar_width(&resolved)`, `font_family(&resolved)`, etc.

Users who call `from_preset()` or `from_system()` get a Theme they
can render with, but must repeat the entire pipeline to access widget
metrics -- the very boilerplate the convenience functions were designed
to eliminate.

### Options

**A. Return a tuple `(Theme, ResolvedThemeVariant)` (recommended)**

```rust
pub fn from_preset(
    name: &str,
    is_dark: bool,
) -> native_theme::Result<(iced_core::theme::Theme, native_theme::ResolvedThemeVariant)> {
    let spec = native_theme::ThemeSpec::preset(name)?;
    let variant = spec.pick_variant(is_dark).ok_or_else(|| ...)?;
    let resolved = variant.clone().into_resolved()?;
    let theme = to_theme(&resolved, name);
    Ok((theme, resolved))
}
```

- Pro: Users get everything they need in one call.
- Pro: Eliminates the need to re-run the pipeline for widget metrics.
- Pro: Matches the v0.5.1 API review recommendation (archived in
  `docs/archive/v0.5.1_iced-connector-API.md`).
- Pro: The pattern `let (theme, resolved) = from_preset("x", true)?;`
  is idiomatic Rust.
- Con: Breaking change -- callers must destructure the tuple.
  Pre-1.0 so acceptable.
- Con: Tuple return is less self-documenting than named fields.
  Mitigated: only two elements, each with a distinct type.

**B. Return a wrapper struct**

```rust
pub struct IcedTheme {
    pub theme: iced_core::theme::Theme,
    pub resolved: native_theme::ResolvedThemeVariant,
}
```

- Pro: Named fields are self-documenting.
- Pro: Methods can be added later (e.g., `fn border_radius(&self)`).
- Con: Introduces a new public type for a simple pair.
- Con: More API surface to maintain.
- Con: Users must access `.theme` everywhere they currently use the
  return value directly.

**C. Keep as-is, document the workaround**

```rust
// Document that users should use the lower-level API:
let spec = ThemeSpec::preset("catppuccin-latte")?;
let resolved = spec.pick_variant(false).unwrap().clone().into_resolved()?;
let theme = to_theme(&resolved, "catppuccin-latte");
// Now you have both `theme` and `resolved`
```

- Pro: No code change.
- Con: The "convenience" functions provide no convenience for the
  common case of needing both Theme and metrics.
- Con: Users discover this limitation only after trying to use
  `button_padding()` and realizing they don't have a
  `ResolvedThemeVariant`.

**D. Add parallel `_with_variant` functions**

```rust
pub fn from_preset_with_variant(name: &str, is_dark: bool)
    -> native_theme::Result<(iced_core::theme::Theme, native_theme::ResolvedThemeVariant)>
```

- Pro: Backward compatible -- old functions still work.
- Pro: Users who only need the Theme aren't burdened.
- Con: API surface doubles for convenience functions.
- Con: Two nearly identical functions with different names is confusing.
- Con: The "without variant" versions are rarely useful -- if you
  only need colors, you could build the palette directly.

### Recommendation

**Option A.** The tuple return is idiomatic, compact, and gives users
everything they need. The breaking change is justified because the
current API actively misleads users: it presents `from_preset` as the
easy path, but users who follow it must immediately duplicate its work
to access widget metrics. Returning the tuple aligns the convenience
function with its actual use case.

Apply the same change to `from_system()`:

```rust
pub fn from_system()
    -> native_theme::Result<(iced_core::theme::Theme, native_theme::ResolvedThemeVariant)>
```

Note: `SystemThemeExt::to_iced_theme()` does NOT need this change
because the caller already holds the `SystemTheme` and can call
`.active()` to access the resolved variant.
