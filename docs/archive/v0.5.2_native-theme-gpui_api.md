# v0.5.2 API Review -- native-theme-gpui (gpui connector)

Verified against source code on 2026-03-31. Each chapter covers one
API problem, lists all resolution options with full pro/contra, and
recommends the best solution. Pre-1.0 -- backward compatibility is
not a constraint.

**Companion documents:**
- [todo_v0.5.2_native-theme_api.md](todo_v0.5.2_native-theme_api.md)
- [todo_v0.5.2_native-theme-build_api.md](todo_v0.5.2_native-theme-build_api.md)
- [todo_v0.5.2_native-theme-iced_api.md](todo_v0.5.2_native-theme-iced_api.md)

---

## Summary

| # | Issue | Severity | Fix complexity |
|---|-------|----------|----------------|
| 1 | All public functions missing `#[must_use]` | Medium | Trivial |
| 2 | `AnimatedImageSources` missing derives | Medium | Trivial |
| 3 | Infallible icon mapping functions return `Option` | Medium | Low |
| 4 | No consuming (`into_`) icon conversion variants | Low | Low |
| 5 | `colorize_svg()` breaks self-closing SVG tags | High | Low |

---

## 1. All public functions missing #[must_use]

**File:** `connectors/native-theme-gpui/src/lib.rs` and
`connectors/native-theme-gpui/src/icons.rs`

**What:** No public function in the entire crate has `#[must_use]`.
Grep for `#[must_use]` in `connectors/native-theme-gpui/src/` returns
zero matches.

Compare with the iced connector where 23 of 26 public functions have
`#[must_use]`, and the core crate where all Result/Option-returning
functions have it.

Affected functions (all return values that should not be discarded):

| Function | File | Return type | Discard risk |
|----------|------|-------------|-------------|
| `to_theme` | `lib.rs:97` | `Theme` | High -- expensive conversion discarded |
| `from_preset` | `lib.rs:135` | `Result<Theme>` | High -- theme + error discarded |
| `from_system` | `lib.rs:159` | `Result<Theme>` | High -- theme + error discarded |
| `SystemThemeExt::to_gpui_theme` | `lib.rs:179` | `Theme` | High -- trait method, easy to discard |
| `icon_name` | `icons.rs:60` | `Option<IconName>` | Medium |
| `lucide_name_for_gpui_icon` | `icons.rs:117` | `Option<&str>` | Medium |
| `material_name_for_gpui_icon` | `icons.rs:214` | `Option<&str>` | Medium |
| `freedesktop_name_for_gpui_icon` | `icons.rs:329` | `Option<&str>` | Medium |
| `to_image_source` | `icons.rs` | `Option<ImageSource>` | Medium |
| `custom_icon_to_image_source` | `icons.rs` | `Option<ImageSource>` | Medium |
| `animated_frames_to_image_sources` | `icons.rs` | `Option<AnimatedImageSources>` | Medium |
| `with_spin_animation` | `icons.rs` | `impl IntoElement` | High -- element discarded |

### Options

**A. Add `#[must_use]` to all public functions (recommended)**

- Pro: Catches discard-result bugs at compile time.
- Pro: Brings the crate to parity with the iced connector.
- Pro: Zero runtime cost.
- Pro: Functions like `to_theme` perform expensive work (108-field
  color mapping, font config) -- discarding the result is always a bug.
- Con: None. `#[must_use]` is purely a lint.

**B. Add `#[must_use]` only to functions returning `Result` or
non-`Option` values**

- Pro: Targets the highest-risk discard cases.
- Con: `Option` return values are also worth guarding -- discarding
  `to_image_source()`'s result means the icon was loaded and converted
  for nothing.
- Con: Arbitrary cutoff that doesn't match the iced connector's
  approach (which marks everything).

**C. Add `#[must_use]` with custom messages on key functions**

```rust
#[must_use = "this returns the theme; it does not apply it"]
pub fn to_theme(...) -> Theme

#[must_use = "this returns the theme; it does not apply it"]
pub fn from_preset(...) -> Result<Theme>
```

- Pro: Custom messages guide users on what to do with the return value.
- Pro: Matches the core crate's style.
- Con: Writing custom messages for 12 functions is tedious. Most
  functions' purpose is obvious from the name.

**D. Keep as-is**

- Pro: No change.
- Con: Inconsistent with the iced connector and core crate.
- Con: Misses free compile-time safety checks.

### Recommendation

**Option A** for most functions, with **Option C** custom messages on
`to_theme`, `from_preset`, `from_system`, and `to_gpui_theme` (the
four main entry points where the message adds value). For icon mapping
and conversion functions, bare `#[must_use]` is sufficient.

---

## 2. AnimatedImageSources missing derives

**File:** `connectors/native-theme-gpui/src/icons.rs:28`

**What:** `AnimatedImageSources` has no derive macros:

```rust
pub struct AnimatedImageSources {
    /// Rasterized frames ready for gpui rendering.
    pub sources: Vec<ImageSource>,
    /// Duration of each frame in milliseconds.
    pub frame_duration_ms: u32,
}
```

This is the gpui counterpart of the iced crate's `AnimatedSvgHandles`,
which has the same problem (**[ICED-2]**). Without `Debug`, the struct
can't be inspected. Without `Clone`, caching patterns require
re-generating the sources.

The derives that can be added depend on what `gpui::ImageSource`
implements.

### Options

**A. Add `Debug` and `Clone` if ImageSource supports them
(recommended)**

```rust
#[derive(Debug, Clone)]
pub struct AnimatedImageSources { ... }
```

- Pro: `Debug` is required by Rust API guideline C-DEBUG.
- Pro: `Clone` enables caching (clone sources, store per-widget).
- Pro: `ImageSource` likely implements both (it wraps image data with
  reference counting internally).
- Con: If `ImageSource` doesn't implement `Debug` or `Clone`, the
  derive fails at compile time. Must verify against gpui 0.2.2.

**B. Add `Debug` only (safe minimum)**

- Pro: Satisfies C-DEBUG regardless of `ImageSource`'s Clone status.
- Pro: Minimal coupling to gpui's trait impls.
- Con: Users still can't clone the struct.

**C. Manual `Debug` impl with summary output**

```rust
impl fmt::Debug for AnimatedImageSources {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnimatedImageSources")
            .field("frame_count", &self.sources.len())
            .field("frame_duration_ms", &self.frame_duration_ms)
            .finish()
    }
}
```

- Pro: Works even if `ImageSource` has no `Debug`.
- Pro: Shows useful summary (frame count) instead of potentially
  large binary data.
- Con: More code than a derive.
- Con: Loses per-frame detail that might be useful for debugging.

**D. Keep as-is**

- Pro: No change.
- Con: Violates C-DEBUG.
- Con: Users cannot inspect animation state during debugging.

### Recommendation

**Option A**, after verifying that `gpui::ImageSource` implements both
traits. If `ImageSource` only implements `Debug`, fall back to
**Option B**. If neither, use **Option C**.

This fix should be coordinated with the identical fix for
`AnimatedSvgHandles` in the iced connector (**[ICED-2]**) to keep
the two connectors consistent.

---

## 3. Infallible icon mapping functions return Option

**Files:** `connectors/native-theme-gpui/src/icons.rs:117,214,329`

**What:** Three icon mapping functions return `Option<&'static str>`
but their match arms cover every `IconName` variant with a `Some(...)`:

```rust
// line 117
pub fn lucide_name_for_gpui_icon(icon: IconName) -> Option<&'static str> {
    Some(match icon {
        IconName::ALargeSmall => "a-large-small",
        IconName::ArrowDown => "arrow-down",
        // ... all 86 variants ...
        IconName::WindowRestore => "window-restore",
    })
}

// line 214 -- same pattern
pub fn material_name_for_gpui_icon(icon: IconName) -> Option<&'static str> { ... }

// line 329 (Linux-only) -- same pattern
pub fn freedesktop_name_for_gpui_icon(icon: IconName, de: LinuxDesktop) -> Option<&'static str> { ... }
```

Every branch returns `Some`. The `None` path is unreachable. The return
type promises fallibility that doesn't exist, forcing callers to handle
`None` for a case that can never occur.

Compare with `icon_name(role: IconRole) -> Option<IconName>` which
genuinely returns `None` for 12 unmapped roles -- a correct use of
`Option`.

### Options

**A. Change return type to `&'static str` (recommended)**

```rust
pub fn lucide_name_for_gpui_icon(icon: IconName) -> &'static str {
    match icon {
        IconName::ALargeSmall => "a-large-small",
        // ...
    }
}
```

- Pro: The type accurately reflects the function's behavior -- it
  always returns a value.
- Pro: Callers don't need `.unwrap()` or `?` for a guaranteed value.
- Pro: Reduces cognitive load -- "Option means it might be None" is
  a false signal here.
- Pro: If gpui-component adds a new `IconName` variant in the future,
  the match becomes non-exhaustive and fails at compile time
  (regardless of Option or bare return). The function must be updated
  either way.
- Con: Breaking change for callers that pattern-match on the Option.
  Fix: remove the `.unwrap()` / `if let Some(name)` wrapping.
  Pre-1.0 so acceptable.
- Con: If a future gpui version adds an `IconName` variant that has
  no mapping, the function would need to return `Option` again. But
  adding a variant to a `#[non_exhaustive]` enum already forces a code
  change in the match statement, so returning `Option` to "future-proof"
  provides no real protection.

**B. Keep `Option` but document that `None` is currently unreachable**

```rust
/// Returns the Lucide name for a gpui icon.
///
/// Currently returns `Some` for all `IconName` variants. Returns `None`
/// only if future gpui versions add variants this crate doesn't yet map.
pub fn lucide_name_for_gpui_icon(icon: IconName) -> Option<&'static str> { ... }
```

- Pro: No breaking change.
- Pro: Acknowledges the future-proofing intent.
- Con: The type still lies about the current behavior.
- Con: Callers still handle `None` for a case that never occurs.
- Con: If gpui adds a variant, the match is non-exhaustive anyway --
  the function won't compile until updated. The `Option` doesn't help.

**C. Add a `_ => None` catch-all instead of exhaustive match**

```rust
pub fn lucide_name_for_gpui_icon(icon: IconName) -> Option<&'static str> {
    match icon {
        IconName::ALargeSmall => Some("a-large-small"),
        // ...
        _ => None,
    }
}
```

- Pro: Compiles even when gpui adds new variants.
- Pro: `None` is genuinely reachable for unknown variants.
- Con: Silently returns `None` for new variants instead of failing at
  compile time. This masks missing mappings -- the opposite of what
  you want.
- Con: Loses the compile-time exhaustiveness check, which is the most
  valuable safety net.

**D. Keep as-is (no change)**

- Pro: No change.
- Con: Return type is misleading.
- Con: Callers handle phantom `None` cases.

### Recommendation

**Option A.** The function is exhaustive today and MUST be updated
when gpui adds new variants regardless of the return type (because the
match is exhaustive). Returning `&'static str` instead of
`Option<&'static str>` makes the API honest and eliminates pointless
`None` handling in every caller.

Apply the same change to all three functions:
- `lucide_name_for_gpui_icon` -- line 117
- `material_name_for_gpui_icon` -- line 214
- `freedesktop_name_for_gpui_icon` -- line 329 (Linux-only)

---

## 4. No consuming (into_) icon conversion variants

**File:** `connectors/native-theme-gpui/src/icons.rs`

**What:** The gpui connector provides only borrowing icon conversion:

```rust
pub fn to_image_source(data: &IconData, ...) -> Option<ImageSource>
```

The iced connector provides both borrowing AND consuming pairs:

```rust
// iced: borrowing (clones data internally)
pub fn to_image_handle(data: &IconData) -> Option<Handle>
pub fn to_svg_handle(data: &IconData, ...) -> Option<Handle>

// iced: consuming (moves data, avoids clone)
pub fn into_image_handle(data: IconData) -> Option<Handle>
pub fn into_svg_handle(data: IconData, ...) -> Option<Handle>
```

The gpui connector has no consuming equivalents. Users who already
own the `IconData` and won't use it again must still pay for an
internal clone of the `Vec<u8>` data.

The practical impact is lower than in iced because `to_image_source`
rasterizes SVGs to BMP (the rasterization cost dominates the clone
cost). For RGBA data, the clone is a pure overhead.

### Options

**A. Add `into_image_source` consuming variant (recommended)**

```rust
pub fn into_image_source(
    data: IconData,
    color: Option<Hsla>,
    size: Option<u32>,
) -> Option<ImageSource>
```

- Pro: API symmetry with the iced connector.
- Pro: Avoids cloning RGBA data when ownership is available.
- Pro: Follows the standard Rust `to_`/`into_` convention.
- Con: The performance benefit is marginal since SVG rasterization
  dominates. However, RGBA icons (loaded from system icon themes)
  skip rasterization and benefit directly.

**B. Also add `into_custom_icon_to_image_source` consuming variant**

- Pro: Complete symmetry.
- Con: The function loads the icon internally via `load_custom_icon`,
  which already produces an owned `IconData`. There's nothing to
  "consume" from the caller's side. A consuming variant would have
  the same signature as the borrowing one.
- Con: Unnecessary API expansion.

**C. Keep as-is**

- Pro: No change. The rasterization cost dominates anyway.
- Pro: Simpler API surface.
- Con: Inconsistent with the iced connector's conventions.
- Con: RGBA icons pay an avoidable clone cost.

### Recommendation

**Option A.** Add only `into_image_source` -- the one function where
the caller owns `IconData` and the clone is avoidable. Skip
`into_custom_icon_to_image_source` since that function's internal icon
loading already produces owned data.

---

## 5. colorize_svg() breaks self-closing SVG tags

**File:** `connectors/native-theme-gpui/src/icons.rs:925-936`

**What:** The internal `colorize_svg()` function injects a `fill`
attribute into the root `<svg>` tag for Material-style SVGs that have
no explicit fill. It does NOT handle self-closing tags (`<svg .../>`),
producing malformed XML.

Current code (lines 925-936):

```rust
if let Some(pos) = svg_str.find("<svg")
    && let Some(close) = svg_str[pos..].find('>')
{
    let tag_end = pos + close;
    let tag = &svg_str[pos..tag_end];
    if !tag.contains("fill=") {
        let mut result = String::with_capacity(svg_str.len() + 20);
        result.push_str(&svg_str[..tag_end]);       // up to '>'
        result.push_str(&format!(" fill=\"{}\"", hex));
        result.push_str(&svg_str[tag_end..]);        // from '>' onward
        return result.into_bytes();
    }
}
```

For input `<svg xmlns="..." />`:
- `tag_end` points to `>` (after the `/`)
- `svg_str[..tag_end]` = `<svg xmlns="..." /`
- Result: `<svg xmlns="..." / fill="#hex">` -- **malformed XML**

The iced connector's `colorize_monochrome_svg()` (lines 252-257)
correctly handles this by checking for `/` before `>`:

```rust
let inject_pos = if tag_end > 0 && svg_str.as_bytes()[tag_end - 1] == b'/' {
    tag_end - 1  // inject before '/' in '/>'
} else {
    tag_end       // inject before '>' in regular '>'
};
```

Producing: `<svg xmlns="..." fill="#hex"/>` -- **valid XML**.

**Impact:** Any Material-style SVG with a self-closing root tag and
no explicit fill will be corrupted when a color override is applied.
`resvg` rejects the malformed XML, so `svg_to_bmp_source()` returns
`None`, and `to_image_source()` silently fails -- the icon disappears.

**Evidence:** The iced connector has a test
`colorize_self_closing_svg_produces_valid_xml` for this case. The gpui
connector has no corresponding test.

### Options

**A. Port the self-closing tag fix from the iced connector
(recommended)**

```rust
if !tag.contains("fill=") {
    // Handle self-closing tags: inject before '/' in '<svg .../>'
    let inject_pos = if tag_end > 0 && svg_str.as_bytes()[tag_end - 1] == b'/' {
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

- Pro: Exact fix already proven in the iced connector.
- Pro: Small diff -- 5 lines changed.
- Pro: Add the matching test from iced to prevent regression.
- Con: None.

**B. Extract a shared `colorize_svg_impl` into the core crate**

- Pro: Single implementation shared by both connectors.
- Pro: Eliminates divergence permanently.
- Con: The core crate doesn't depend on any GUI toolkit, so the color
  type would need to be generic or use a hex string. Both connectors
  convert to hex internally, so the shared function would take a hex
  color string.
- Con: More refactoring than a point fix. Better as a follow-up.

**C. Keep as-is**

- Pro: No change.
- Con: Self-closing SVGs silently fail to render when colorized.
- Con: No test coverage for this code path.

### Recommendation

**Option A** now, **Option B** as follow-up. The point fix is urgent
because it causes silent icon rendering failures. Extracting a shared
implementation is the right long-term solution but can be done in a
later version.
