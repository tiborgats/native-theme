# Architecture Patterns: Per-Widget Theme Architecture

**Domain:** Per-widget ThemeVariant restructure, resolve() pipeline, ResolvedTheme generation
**Researched:** 2026-03-27
**Confidence:** HIGH -- analysis based on direct source code inspection of all relevant files

## Executive Summary

The migration from flat `ThemeColors` + `ThemeFonts` + `ThemeGeometry` + `WidgetMetrics` to per-widget structs with `ThemeDefaults` and `resolve()` is a significant structural change that touches every layer of the crate. After thorough analysis of the existing code, the key insight is: **the existing `impl_merge!` macro and Option-based overlay system already support nested structs** (as proven by `WidgetMetrics`), so the per-widget migration is an incremental expansion of a proven pattern, not a paradigm shift.

The change is best executed as a **phased migration** rather than a big-bang refactor: data model first, then resolve(), then OS readers, then presets, then connectors. Each phase produces a compilable, testable crate.

---

## 1. Migration Path: Flat to Per-Widget (Incremental vs Big-Bang)

### Verdict: Incremental migration is feasible, but the data model change itself is necessarily atomic within ThemeVariant.

**Why the data model must be one commit:** `ThemeVariant` is the central struct. Currently it has 6 fields (`colors`, `fonts`, `geometry`, `spacing`, `widget_metrics`, `icon_set`). The target has ~28 fields (`defaults`, `text_scale`, 24 widget structs, `icon_set`). You cannot have both layouts simultaneously because serde keys would conflict (e.g., `button` currently lives under `widget_metrics.button` but needs to become `button` at the variant level with a completely different shape). The data model change to `ThemeVariant` is therefore a single commit.

**What CAN be incremental:**

| Step | Can be separate commit? | Rationale |
|------|------------------------|-----------|
| Add `FontSpec` struct | Yes | New type, no conflicts with existing `ThemeFonts` |
| Add `TextScaleEntry`, `TextScale` | Yes | New types, no conflicts |
| Add `IconSizes`, `DialogButtonOrder` | Yes | New types |
| Add `ThemeDefaults` struct | Yes | New type definition only |
| Add all 24 per-widget `*Theme` structs | Yes | New type definitions only |
| **Restructure `ThemeVariant` fields** | **Atomic** | Replaces `colors`/`fonts`/`geometry`/`spacing`/`widget_metrics` with `defaults` + per-widget structs |
| Update `impl_merge!` invocations | Must accompany ThemeVariant restructure | Merge must match the struct layout |
| Update OS readers | Separate commits per platform | Each reader can be updated independently |
| Update presets (TOML files) | Must accompany ThemeVariant restructure | TOML layout mirrors struct layout |
| Add `resolve()` | Separate commit after data model | Operates on the new ThemeVariant |
| Add `ResolvedTheme` + `validate()` | Separate commit after resolve() | Output-only types |
| Update connectors | Separate commits per connector | Each connector updated independently |

### Recommended incremental plan

```
Commit 1: Add new types (FontSpec, TextScaleEntry, IconSizes, DialogButtonOrder,
          ThemeDefaults, 24 widget *Theme structs) -- compiles, no existing code
          changes, all tests pass
Commit 2: Restructure ThemeVariant + update impl_merge! + update presets
          -- breaking change to ThemeVariant shape
          -- OS readers temporarily broken (return old shape)
          -- connectors temporarily broken (read old shape)
Commit 3: Update OS readers (GNOME, KDE) to return new ThemeVariant shape
Commit 4: Add resolve() function
Commit 5: Add ResolvedTheme + validate()
Commit 6: Update connectors to accept &ResolvedTheme
```

### Why not keep backward compatibility

Keeping `ThemeColors` as a re-export or adapter is not worth it. The entire point is that colors are per-widget now, and the flat `ThemeColors` cannot represent "button has its own background distinct from tooltip background." A compatibility shim would either lose information (flatten) or lie about the type (Option fields that are always Some). Clean break is better -- semver major bump handles this.

---

## 2. impl_merge! Macro: Handling Nested Per-Widget Structs

### Current macro capabilities

The existing `impl_merge!` macro already supports two field categories:

```rust
impl_merge!(StructName {
    option { field1, field2, ... }   // Option<T> leaf fields
    nested { field1, field2, ... }   // Nested structs with their own merge()
});
```

This is already used for `WidgetMetrics`:

```rust
impl_merge!(WidgetMetrics {
    nested { button, checkbox, input, scrollbar, slider, progress_bar,
             tab, menu_item, tooltip, list_item, toolbar, splitter }
});
```

### What needs to change

The macro itself needs **no changes**. The new per-widget structs simply use `impl_merge!` with both `option` and `nested` categories. Example for `ButtonTheme`:

```rust
impl_merge!(ButtonTheme {
    option { background, foreground, border, min_width, min_height,
             padding_horizontal, padding_vertical, radius, icon_spacing,
             disabled_opacity, shadow, primary_bg, primary_fg }
    nested { font }  // FontSpec is a nested struct with its own merge
});
```

### ThemeVariant merge -- the critical change

The current `ThemeVariant::merge()` is hand-written (not via macro) because `widget_metrics` is `Option<WidgetMetrics>` requiring special handling. The new `ThemeVariant` has:
- `defaults: ThemeDefaults` -- nested struct (always present, uses merge recursively)
- `text_scale: TextScale` -- nested struct
- 24 widget structs -- all nested structs (always present, use merge recursively)
- `icon_set: Option<String>` -- option field

**Recommendation:** Use `impl_merge!` for the new `ThemeVariant` since all widget fields are now directly nested (not wrapped in `Option`):

```rust
impl_merge!(ThemeVariant {
    option { icon_set }
    nested { defaults, text_scale, window, button, input, checkbox,
             menu, tooltip, scrollbar, slider, progress_bar, tab,
             sidebar, toolbar, status_bar, list, popover, splitter,
             separator, switch, dialog, spinner, combo_box,
             segmented_control, card, expander, link }
});
```

This eliminates the hand-written `ThemeVariant::merge()` entirely. The macro handles everything.

### ThemeDefaults merge

`ThemeDefaults` has both option fields (colors, geometry) and nested structs (`font`, `mono_font`, `spacing`, `icon_sizes`):

```rust
impl_merge!(ThemeDefaults {
    option { line_height, background, foreground, accent, accent_foreground,
             surface, border, muted, shadow, link, selection,
             selection_foreground, selection_inactive, disabled_foreground,
             danger, danger_foreground, warning, warning_foreground,
             success, success_foreground, info, info_foreground,
             radius, radius_lg, frame_width, disabled_opacity,
             border_opacity, shadow_enabled,
             focus_ring_color, focus_ring_width, focus_ring_offset,
             text_scaling_factor, reduce_motion, high_contrast,
             reduce_transparency }
    nested { font, mono_font, spacing, icon_sizes }
});
```

### Per-widget font fields: FontSpec directly, not Option<FontSpec>

Some widget structs in the todo spec show `font: Option<FontSpec>`. The macro's `option` category handles `Option<T>` fields via `if overlay.field.is_some()`. But for `Option<FontSpec>`, this means the entire FontSpec is replaced, not merged field-by-field.

**Recommendation:** Use `FontSpec` directly (not `Option<FontSpec>`) in per-widget structs. Reasons:

1. serde `#[serde(default)]` makes an absent `[widget.font]` section produce `FontSpec::default()` (all None), which is identical in meaning to "inherit from defaults."
2. The `nested` merge path gives field-level FontSpec merging for free -- if a platform TOML sets `button.font.weight = 600` and an app TOML sets `button.font.size = 16`, both survive.
3. No loss of expressiveness -- "inherit everything" = all fields None = empty FontSpec.
4. Using `Option<FontSpec>` with the `option` macro category would cause a TOML with only `button.font.size = 12` to create `Some(FontSpec { size: Some(12), ..})` and merging would replace the entire FontSpec, losing any base family/weight. This is a bug waiting to happen.

---

## 3. resolve() Implementation

### Architecture: Single function with helper closures

**Why not per-widget methods:** Resolution requires cross-widget access. `button.primary_bg` inherits from `defaults.accent`. A method on `ButtonTheme` would need `&ThemeDefaults` passed in, creating a fragile parameter-passing chain. A single function that has `&mut ThemeVariant` can access any field.

**Why not trait-based:** A `Resolvable` trait adds abstraction without value. Each widget's inheritance rules are unique and documented in the inheritance table. A trait would need per-widget implementations anyway, and the indirection makes the inheritance rules harder to audit. Explicit code is better here.

**Why a single function:** All inheritance rules are in one place, auditable against the inheritance table, and the function is the single source of truth. The function is long (~200-300 lines) but structurally simple: a list of `if field.is_none() { field = source.clone() }` statements.

### Location

`native-theme/src/model/resolve.rs` -- new module within the model.

### Implementation pattern

```rust
pub fn resolve(variant: &mut ThemeVariant) {
    // Step 1: defaults self-derivation
    if variant.defaults.selection.is_none() {
        variant.defaults.selection = variant.defaults.accent;
    }
    if variant.defaults.focus_ring_color.is_none() {
        variant.defaults.focus_ring_color = variant.defaults.accent;
    }
    if variant.defaults.selection_inactive.is_none() {
        variant.defaults.selection_inactive = variant.defaults.selection;
    }

    // Step 2: snapshot defaults (avoids borrow conflict)
    let d = variant.defaults.clone();

    // Step 3: widget field inheritance (one block per widget)
    resolve_button(&mut variant.button, &d);
    resolve_input(&mut variant.input, &d);
    // ... for each widget

    // Step 4: FontSpec inheritance
    resolve_font(&mut variant.button.font, &d.font);
    resolve_font(&mut variant.menu.font, &d.font);
    // ... for each widget with a font field

    // Step 5: TextScale inheritance
    resolve_text_scale(&mut variant.text_scale, &d);
}

fn resolve_font(widget_font: &mut FontSpec, default_font: &FontSpec) {
    if widget_font.family.is_none() {
        widget_font.family.clone_from(&default_font.family);
    }
    if widget_font.size.is_none() {
        widget_font.size = default_font.size;
    }
    if widget_font.weight.is_none() {
        widget_font.weight = default_font.weight;
    }
}
```

### Borrow checker strategy

Clone defaults at the start: `let d = variant.defaults.clone()`. Simple, correct, small allocation (ThemeDefaults is ~500 bytes of Options). Split borrows via intermediate variables are possible but fragile and hard to maintain.

### Interaction with merge()

`resolve()` runs AFTER merge, not during. The pipeline is:
1. OS reader creates sparse ThemeVariant
2. Platform TOML parsed, merged (`preset.merge(&os_theme)` -- OS values overlay on preset base)
3. `resolve(&mut variant)` fills None fields from inheritance
4. (Optional) App TOML merged on top
5. `resolve(&mut variant)` again to propagate any changed sources
6. `validate()` converts to ResolvedTheme

`resolve()` is idempotent -- running it twice has no effect if no fields changed between runs.

---

## 4. ResolvedTheme Generation

### Verdict: Manual parallel structs, manual conversion

| Approach | Pros | Cons |
|----------|------|------|
| **Parallel struct hierarchy (manual)** | Full control, clear code, easy debugging | Boilerplate: 24+ structs that mirror the Option versions |
| **Derive macro** | Less boilerplate | Complex proc macro, harder to debug, overkill for this crate |
| **Code generation (build.rs)** | Could generate from schema | Couples schema format to struct layout |

**Recommendation: Manual parallel structs.** The boilerplate is manageable because:
1. Each Resolved struct is just the Option struct with `Option<T>` replaced by `T`. No logic, no methods -- just data.
2. The conversion (`validate()`) is one function that unwraps each field, collecting errors.
3. The structs rarely change after initial creation.

### Struct pattern

```rust
/// Fully resolved button theme -- all fields guaranteed present.
#[derive(Clone, Debug)]
pub struct ResolvedButton {
    pub background: Rgba,
    pub foreground: Rgba,
    pub border: Rgba,
    pub font: ResolvedFont,
    pub min_width: f32,
    pub min_height: f32,
    pub padding_horizontal: f32,
    pub padding_vertical: f32,
    pub radius: f32,
    pub icon_spacing: f32,
    pub disabled_opacity: f32,
    pub shadow: bool,
    pub primary_bg: Rgba,
    pub primary_fg: Rgba,
}
```

**Key traits:** `Clone + Debug` only. NOT `Default` (no sensible defaults). NOT `Serialize/Deserialize` (output only). NOT `PartialEq` (no use case).

### validate() implementation

The validate function must continue checking ALL fields even after finding the first missing one, so it can report all missing fields at once. Use a collection pass with helper functions:

```rust
fn require_rgba(missing: &mut Vec<String>, path: &str, value: &Option<Rgba>) -> Option<Rgba> {
    match value {
        Some(v) => Some(*v),
        None => { missing.push(path.to_string()); None }
    }
}
// Build ResolvedTheme only if missing is empty
```

---

## 5. TOML Serialization

### How serde handles nested tables

The new per-widget format produces deeper nesting that works natively with the `toml` crate:

```toml
[light.defaults]
accent = "#3daee9"
background = "#fcfcfc"

[light.defaults.font]
family = "Noto Sans"
size = 10.0
weight = 400

[light.defaults.spacing]
xxs = 2.0

[light.button]
min_width = 80.0

[light.button.font]
weight = 600
```

### Key serde patterns for new structs

Every per-widget struct needs:
- `#[serde(default)]` -- missing TOML sections produce default (all None) structs
- `#[serde_with::skip_serializing_none]` -- omit None fields
- `#[serde(default, skip_serializing_if = "T::is_empty")]` on the ThemeVariant field -- omit empty widget sections entirely

This matches the existing pattern used for `ThemeColors`, `ThemeFonts`, etc.

### Format changes summary

| Current TOML path | New TOML path |
|---|---|
| `[light.colors]` accent, background | `[light.defaults]` accent, background |
| `[light.fonts]` family, size | `[light.defaults.font]` family, size, weight |
| `[light.geometry]` radius | `[light.defaults]` radius |
| `[light.spacing]` | `[light.defaults.spacing]` |
| `[light.widget_metrics.button]` min_width | `[light.button]` min_width + background + ... |
| `[light.colors]` button, button_foreground | `[light.button]` background, foreground |
| `[light.colors]` tooltip, tooltip_foreground | `[light.tooltip]` background, foreground |

### Backward compatibility

There is no backward compatibility with the old TOML format. The structural change is too different for serde aliases to bridge. This is a semver major bump.

---

## 6. OS Reader Return Type

### Readers continue to return NativeTheme. ThemeVariant shape changes, but the wrapper stays.

The current readers return `NativeTheme` which wraps `Option<ThemeVariant>` for light and dark. This is correct because:
1. Readers detect which variant (light or dark) is active and populate only that one.
2. `NativeTheme` provides `pick_variant()` for consumers.
3. The merge pipeline operates on `NativeTheme`.

### Merge direction for OS-first pipeline

The merge semantics are "overlay.Some replaces base.Some." For OS-first, OS-read values must win over TOML values:

```rust
let mut base = NativeTheme::preset("kde-breeze")?; // TOML is the base (fallback)
let os_theme = from_kde()?;                        // OS-read values
base.merge(&os_theme);                             // OS values overlay (win)
resolve(&mut base);                                // Fill inheritance gaps
```

Then for app TOML:
```rust
let app_overlay = NativeTheme::from_file("app-theme.toml")?;
base.merge(&app_overlay);  // App overrides win over everything
resolve(&mut base);         // Re-derive from changed sources
```

---

## 7. Connector Migration Surface

### Iced connector -- fields accessed and new paths

| Function | Current access | New access |
|----------|---------------|-----------|
| `to_palette()` | `variant.colors.background` | `variant.defaults.background` |
| `to_palette()` | `variant.colors.accent` | `variant.defaults.accent` |
| `apply_overrides()` | `variant.colors.secondary_background` | `variant.defaults.secondary_background` (needs adding) |
| `apply_overrides()` | `variant.colors.surface` | `variant.defaults.surface` |
| `button_padding()` | `variant.widget_metrics.as_ref()?.button.padding_horizontal` | `variant.button.padding_horizontal` |
| `border_radius()` | `variant.geometry.radius.unwrap_or(4.0)` | `variant.defaults.radius.unwrap_or(4.0)` |
| `scrollbar_width()` | `variant.geometry.scroll_width` or `widget_metrics.scrollbar.width` | `variant.scrollbar.width` |
| `font_family()` | `variant.fonts.family` | `variant.defaults.font.family` |

### Missing fields in new model

The flat `ThemeColors` has `secondary_background` and `secondary_foreground` that have no per-widget counterpart. These are needed by the iced connector's Extended palette.

**Recommendation:** Keep `secondary_background` and `secondary_foreground` in `ThemeDefaults`. They represent a cross-platform concept (secondary action color) used by multiple toolkit connectors.

### After ResolvedTheme -- radical simplification

All `unwrap_or()` fallbacks and fabricated defaults disappear:

```rust
// Before
pub fn border_radius(variant: &ThemeVariant) -> f32 {
    variant.geometry.radius.unwrap_or(4.0)
}

// After
pub fn border_radius(theme: &ResolvedTheme) -> f32 {
    theme.defaults.radius
}
```

---

## 8. Build Order

### Dependency graph

```
FontSpec, TextScaleEntry, IconSizes, DialogButtonOrder (new types, no deps)
    |
    v
ThemeDefaults (depends on FontSpec, ThemeSpacing, IconSizes)
    |
    v
24 per-widget *Theme structs (depend on FontSpec, Rgba)
    |
    v
ThemeVariant restructure + preset rewrites (atomic)
    |
    v
OS readers update
    |
    v
resolve()
    |
    v
ResolvedTheme + validate()
    |
    v
from_system() pipeline
    |
    v
Connector updates
```

### Recommended build order

| Order | Component | Tests |
|-------|-----------|-------|
| **1** | New type definitions (FontSpec, TextScaleEntry, IconSizes, DialogButtonOrder) | merge, is_empty, serde round-trip |
| **2** | `ThemeDefaults` struct | merge, is_empty, serde |
| **3** | 24 per-widget structs | merge, is_empty, serde each |
| **4** | **ThemeVariant restructure + preset TOML rewrites** (atomic) | Existing preset tests rewritten |
| **5** | OS reader updates (GNOME, KDE) | Platform integration tests |
| **6** | `resolve()` function | Each inheritance rule individually |
| **7** | `ResolvedTheme` + `validate()` | Full variant -> Ok; missing -> Err |
| **8** | `from_system()` pipeline | Pipeline produces complete theme |
| **9** | Connector updates | Connector unit tests |

### Critical ordering constraints

1. **Steps 4 (ThemeVariant + presets) are atomic.** Cannot restructure ThemeVariant without rewriting TOML presets because `from_toml()` deserializes into ThemeVariant.
2. **Step 5 must follow 4.** OS readers construct ThemeVariant directly.
3. **Step 6 must follow 5.** resolve() needs realistic sparse ThemeVariants for testing.
4. **Step 7 must follow 6.** validate() checks that resolve() left no None fields.
5. **Step 9 can partially overlap with 6-8** if connectors initially adapt to new ThemeVariant shape (not ResolvedTheme).

---

## Component Boundaries

| Component | Responsibility | Communicates With |
|-----------|---------------|-------------------|
| `model/fontspec.rs` | FontSpec, TextScaleEntry, TextScale | ThemeDefaults, all widget structs |
| `model/defaults.rs` | ThemeDefaults: global base properties | ThemeVariant, resolve() |
| `model/widgets/*.rs` | 24 per-widget theme structs | ThemeVariant, resolve(), ResolvedTheme |
| `model/mod.rs` | ThemeVariant: top-level composed struct | OS readers, presets, connectors |
| `model/resolve.rs` | resolve(): inheritance rule application | Reads/writes ThemeVariant |
| `model/resolved.rs` | ResolvedTheme + validate() | Reads ThemeVariant, produces output |
| `gnome/mod.rs`, `kde/mod.rs` | OS readers | Produce sparse ThemeVariant |
| `presets.rs` | Preset loading + TOML parsing | Produce full ThemeVariant |
| `lib.rs` | from_system() pipeline | Composes all above |
| `connectors/iced/`, `connectors/gpui/` | Toolkit mapping | Consume ResolvedTheme |

---

## Anti-Patterns to Avoid

### Anti-Pattern 1: Inheritance in merge()
**What:** Making merge() aware of inheritance rules.
**Why bad:** Conflates overlay semantics with derivation logic. Makes both harder to reason about.
**Instead:** Keep merge() as dumb field-level overlay. Run resolve() as a separate step.

### Anti-Pattern 2: Optional per-widget structs
**What:** `pub button: Option<ButtonTheme>` on ThemeVariant.
**Why bad:** Three-level optionality: Option<struct> containing Option<fields>. merge() needs `match (Option, Option)` at struct level AND field level. is_empty() needs double checks.
**Instead:** `pub button: ButtonTheme` (always present, fields are Option). Empty ButtonTheme = "not specified." Same pattern as current ThemeColors/ThemeFonts.

### Anti-Pattern 3: Generic resolve via reflection
**What:** Using proc macros or runtime reflection to auto-resolve based on field names.
**Why bad:** Inheritance rules are not purely name-based. `spinner.fill` inherits from `defaults.accent` on one conceptual path but `defaults.foreground` on another. The universal resolve() picks ONE fallback intentionally.
**Instead:** Explicit hand-written resolve() with comments referencing the inheritance table.

### Anti-Pattern 4: Option<FontSpec> in per-widget structs
**What:** Using `Option<FontSpec>` instead of `FontSpec` for widget font fields.
**Why bad:** The `option` macro category replaces the entire Option on merge, destroying partial FontSpec data from the base. A TOML setting only `button.font.size = 12` would lose the base family/weight.
**Instead:** Use `FontSpec` directly with `nested` merge category. Empty FontSpec = inherit all.

---

## Sources

- Direct source code analysis of all files in `native-theme/src/`, `connectors/`, and `docs/todo_v0.5.1_*.md`
- `impl_merge!` macro definition: `native-theme/src/lib.rs` lines 47-77
- `ThemeVariant` current definition: `native-theme/src/model/mod.rs` lines 52-116
- `WidgetMetrics` nested merge precedent: `native-theme/src/model/widget_metrics.rs` lines 314-316
- Inheritance rules: `docs/todo_v0.5.1_inheritance-rules.md`
- Resolution pipeline: `docs/todo_v0.5.1_resolution.md`
- Per-widget struct definitions: `docs/todo_v0.5.1_theme-variant.md`
