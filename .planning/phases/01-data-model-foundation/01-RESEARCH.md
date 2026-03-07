# Phase 1: Data Model Foundation - Research

**Researched:** 2026-03-07
**Domain:** Rust type system design, serde/TOML serialization, declarative macros
**Confidence:** HIGH

<user_constraints>

## User Constraints (from CONTEXT.md)

### Locked Decisions
- ThemeColors uses **nested sub-structs grouped by semantic role** (e.g., ButtonColors, WindowColors, InputColors) -- not a flat 36-field struct
- **Human-editability is a primary concern** -- theme TOML files should be easy to hand-edit, with readable structure and logical field ordering
- Nested color grouping was specifically chosen over flat -- the struct hierarchy should mirror how TOML sections naturally nest (e.g., `[light.colors.button]`)

### Claude's Discretion
- **Color representation:** internal representation (f32 vs u8), alpha default behavior when omitted in hex strings, convenience methods (to_array, to_tuple, etc.), color space handling
- **Struct organization:** whether Option wraps each field, each group, or both; exact color role groupings (design based on what OS theme APIs actually provide); whether ThemeFonts/ThemeGeometry/ThemeSpacing also nest or stay flat (decide based on field count)
- **Merge behavior:** ownership semantics (consume vs borrow), pairwise vs variadic, deep merge vs shallow replace for nested sub-structs, whether the merge macro is public or internal
- **Serialization style:** TOML variant structure (top-level vs wrapper), whether to include metadata section, field ordering strategy

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope

</user_constraints>

<phase_requirements>

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| CORE-01 | Rgba color type with 8-bit-per-channel sRGB + alpha, custom hex serde (#RRGGBB / #RRGGBBAA) | Custom Serialize/Deserialize impl for hex strings; u8 internal representation; see Color Type section |
| CORE-02 | ThemeColors struct with 36 semantic color roles, all fields Option<Rgba> | Nested sub-struct design with semantic groupings; see Architecture Patterns section |
| CORE-03 | ThemeFonts struct with family, size, monospace family/size, all Option<T> | 4 fields -- keep flat; see Discretion Recommendations |
| CORE-04 | ThemeGeometry struct with border radius, border width, disabled/border opacity, all Option<f32> | 5 fields -- keep flat; see Discretion Recommendations |
| CORE-05 | ThemeSpacing struct with named spacing scale (xxs through xxl), all Option<f32> | 7 fields -- keep flat; see Discretion Recommendations |
| CORE-06 | ThemeVariant composing ThemeColors, ThemeFonts, ThemeGeometry, ThemeSpacing | Standard composition with #[serde(default)] |
| CORE-07 | NativeTheme with name, light variant, dark variant | Top-level struct with Option<ThemeVariant> for each variant |
| CORE-08 | merge() method on all structs via declarative macro for theme layering | Declarative macro pattern documented; deep merge for nested sub-structs; see Code Examples |
| CORE-09 | All public structs #[non_exhaustive] for forward compatibility | Standard Rust pattern; works well with serde(default) |
| CORE-10 | All types Send + Sync, Default, Clone, Debug | All types are plain data (no Rc, no RefCell, no raw pointers) -- automatic |
| SERDE-01 | TOML serialization/deserialization mapping 1:1 to struct field names | toml 1.0.6 with serde 1.0.228; nested structs map naturally to TOML sections |
| SERDE-02 | #[serde(default)] + skip_serializing_if = "Option::is_none" on all fields | Use serde_with 3.17.0 #[skip_serializing_none] macro to reduce annotation noise |
| ERR-01 | Error enum with Unsupported, Unavailable, Format, Platform variants + Display + std::error::Error | Standard error pattern; see Code Examples |
| TEST-01 | Round-trip serde tests for all types | Standard pattern: construct -> serialize to TOML -> deserialize -> assert_eq |
| TEST-03 | Rgba hex parsing edge cases (3/4/6/8 char, with/without #, invalid) | Unit tests for FromStr implementation with comprehensive edge cases |

</phase_requirements>

## Summary

Phase 1 builds the complete type system for native-theme: a greenfield Rust crate with no existing source code. The domain is well-understood -- it is pure data modeling with serde serialization, requiring no platform-specific code, no async, and no complex dependencies. The entire phase uses only three crates: `serde` (derive), `toml`, and optionally `serde_with` (for `#[skip_serializing_none]` annotation convenience).

The primary technical challenge is designing the nested ThemeColors sub-structs so they produce human-readable TOML sections while remaining ergonomic for the merge macro. The IMPLEMENTATION.md (project research document) provides a complete specification including all 36 color roles, their platform mappings, and example TOML files. The user has locked the decision that ThemeColors must use nested sub-structs grouped by semantic role rather than a flat struct. This means the TOML output will have sections like `[light.colors.core]`, `[light.colors.button]`, etc., which aligns well with the human-editability requirement.

The declarative merge macro is the most architecturally significant piece. It must generate `merge()` methods for all theme structs at compile time, preventing the desynchronization risk identified in the roadmap. The macro should perform deep merge: for leaf `Option<T>` fields it replaces `None` with the overlay's `Some`, and for nested sub-structs it recursively calls `merge()`.

**Primary recommendation:** Use u8 internal representation for Rgba (matches all platform sources and hex serialization), nest ThemeColors into 6 semantic sub-structs with Option on every leaf field (not on groups), keep ThemeFonts/ThemeGeometry/ThemeSpacing flat (too few fields to justify nesting), and implement merge via a declarative macro that handles both leaf-Option and nested-struct fields.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| serde | 1.0.228 | Derive Serialize/Deserialize for all structs | De facto Rust serialization; required by toml crate |
| toml | 1.0.6 | TOML serialization and deserialization | Standard Rust TOML crate; supports TOML spec 1.1.0 |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| serde_with | 3.17.0 | `#[skip_serializing_none]` macro | Eliminates per-field `#[serde(skip_serializing_if = "Option::is_none")]` boilerplate on all ~50 Option fields |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| serde_with for skip_serializing_none | Manual per-field annotations | serde_with adds a compile dep but eliminates ~50 lines of repetitive annotations and prevents forgetting one |
| Custom hex serde for Rgba | hex_color or serde-hex crate | Custom is ~60 lines, avoids an extra dependency in the always-on path; hex parsing is trivially correct |
| thiserror for Error enum | Manual Display + Error impls | thiserror is convenient but adds a proc-macro dep for ~30 lines of hand-written code; either works |

**Installation (Cargo.toml):**
```toml
[package]
name = "native-theme"
version = "0.1.0"
edition = "2024"
license = "MIT OR Apache-2.0"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_with = "3"
toml = "1"
```

## Architecture Patterns

### Recommended Project Structure
```
native-theme/
  Cargo.toml
  src/
    lib.rs                    # Public API, re-exports, merge macro definition
    color.rs                  # Rgba type, custom serde, FromStr, Display
    error.rs                  # Error enum
    model/
      mod.rs                  # NativeTheme, ThemeVariant, re-exports sub-modules
      colors.rs               # ThemeColors + nested sub-structs (CoreColors, ButtonColors, etc.)
      fonts.rs                # ThemeFonts
      geometry.rs             # ThemeGeometry
      spacing.rs              # ThemeSpacing
```

### Pattern 1: Nested Color Sub-Structs (Locked Decision)

**What:** ThemeColors contains nested sub-structs grouped by semantic role rather than 36 flat Option fields. Each sub-struct groups related color roles.

**When to use:** Always -- this is a locked user decision.

**Recommended groupings** (based on analysis of what OS theme APIs actually provide and what semantic roles group together logically):

```rust
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ThemeColors {
    #[serde(default)]
    pub core: CoreColors,
    #[serde(default)]
    pub primary: ActionColors,
    #[serde(default)]
    pub secondary: ActionColors,
    #[serde(default)]
    pub status: StatusColors,
    #[serde(default)]
    pub interactive: InteractiveColors,
    #[serde(default)]
    pub surface: SurfaceColors,
    #[serde(default)]
    pub component: ComponentColors,
}

// Sub-struct: 7 fields -- core window/app colors
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct CoreColors {
    pub accent: Option<Rgba>,
    pub background: Option<Rgba>,
    pub foreground: Option<Rgba>,
    pub surface: Option<Rgba>,
    pub border: Option<Rgba>,
    pub muted: Option<Rgba>,
    pub shadow: Option<Rgba>,
}

// Sub-struct: 2 fields -- reused for primary and secondary actions
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ActionColors {
    pub background: Option<Rgba>,
    pub foreground: Option<Rgba>,
}

// Sub-struct: 8 fields -- semantic status colors
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct StatusColors {
    pub danger: Option<Rgba>,
    pub danger_foreground: Option<Rgba>,
    pub warning: Option<Rgba>,
    pub warning_foreground: Option<Rgba>,
    pub success: Option<Rgba>,
    pub success_foreground: Option<Rgba>,
    pub info: Option<Rgba>,
    pub info_foreground: Option<Rgba>,
}

// Sub-struct: 4 fields -- interactive/selection state
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct InteractiveColors {
    pub selection: Option<Rgba>,
    pub selection_foreground: Option<Rgba>,
    pub link: Option<Rgba>,
    pub focus_ring: Option<Rgba>,
}

// Sub-struct: 6 fields -- surface variants (sidebar, tooltip, popover)
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct SurfaceColors {
    pub sidebar: Option<Rgba>,
    pub sidebar_foreground: Option<Rgba>,
    pub tooltip: Option<Rgba>,
    pub tooltip_foreground: Option<Rgba>,
    pub popover: Option<Rgba>,
    pub popover_foreground: Option<Rgba>,
}

// Sub-struct: 7 fields -- UI component colors
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ComponentColors {
    pub button: Option<Rgba>,
    pub button_foreground: Option<Rgba>,
    pub input: Option<Rgba>,
    pub input_foreground: Option<Rgba>,
    pub disabled: Option<Rgba>,
    pub separator: Option<Rgba>,
    pub alternate_row: Option<Rgba>,
}
```

**Total: 36 color roles** (7 core + 2 primary + 2 secondary + 8 status + 4 interactive + 6 surface + 7 component = 36). This matches the requirement exactly.

**Resulting TOML structure:**
```toml
[light.colors.core]
accent = "#3daee9"
background = "#eff0f1"
foreground = "#232629"

[light.colors.primary]
background = "#3daee9"
foreground = "#ffffff"

[light.colors.status]
danger = "#da4453"
warning = "#f67400"
success = "#27ae60"

[light.colors.component]
button = "#fcfcfc"
button_foreground = "#232629"
```

This is human-readable and logically organized. TOML sections naturally mirror the struct nesting.

### Pattern 2: Option on Leaf Fields Only (Discretion Recommendation)

**What:** Option<T> wraps each individual color field (leaf), NOT the sub-struct groups. Sub-structs are always present but may have all-None fields.

**Why this is best:**
- The merge macro can always recurse into sub-structs without checking `Option<SubStruct>` first
- `#[serde(default)]` on the sub-struct fields means missing TOML sections deserialize as `Default::default()` (all None)
- Serialization with skip_serializing_none on each leaf field means empty sub-structs produce no TOML output
- Avoids double-Option complexity (`Option<Option<Rgba>>` if both group and leaf are Option)

**Caveat:** An all-None sub-struct will still serialize as an empty TOML table header if not handled. Solution: add a custom `is_empty()` method to each sub-struct and use `#[serde(skip_serializing_if = "SubStruct::is_empty")]` on the ThemeColors fields. The merge macro can generate `is_empty()` as well.

### Pattern 3: Flat ThemeFonts/ThemeGeometry/ThemeSpacing (Discretion Recommendation)

**What:** Keep ThemeFonts (4 fields), ThemeGeometry (5 fields), and ThemeSpacing (7 fields) as flat structs -- do not introduce nesting.

**Why:** These structs have too few fields (4-7 each) to justify sub-grouping. The TOML output is already clean:
```toml
[light.fonts]
family = "Noto Sans"
size = 10.0
mono_family = "Hack"
mono_size = 10.0

[light.geometry]
radius = 5.0
frame_width = 2.0

[light.spacing]
xxs = 2.0
xs = 4.0
```

### Anti-Patterns to Avoid

- **Using #[serde(flatten)] with TOML:** Known issues with flatten in the toml crate. Flatten causes problems with `default`, `alias`, and round-trip serialization. Use natural struct nesting instead -- it maps cleanly to TOML table sections.
- **Wrapping sub-struct groups in Option<T>:** Creates double-Option ergonomic pain and complicates the merge macro. Use `#[serde(default)]` on non-Option sub-struct fields instead.
- **Manual merge() implementations:** Writing merge() by hand for each struct is the desynchronization risk identified in the roadmap. Always generate via the declarative macro.
- **Deriving PartialEq on structs containing f32:** ThemeFonts, ThemeGeometry, ThemeSpacing contain `Option<f32>`. Deriving PartialEq is fine for tests (exact equality after round-trip), but document that float comparison is exact, not approximate.

## Discretion Recommendations

These are Claude's recommendations for areas marked as "Claude's Discretion":

### Color Representation: Use u8 Internally

**Recommendation:** `Rgba { r: u8, g: u8, b: u8, a: u8 }` with u8 components.

**Rationale:**
- All platform sources provide u8 or trivially convert to u8 (KDE: integer R,G,B; Windows: u8 Color; portal: `(v * 255.0).round() as u8`)
- Hex serialization (#RRGGBB) is a natural u8 format
- IMPLEMENTATION.md Section 17.2 documents this decision with full justification
- Downstream toolkits (egui Color32 uses u8, iced Color uses f32) -- the consumer does the conversion, which is a trivial `c as f32 / 255.0`
- u8 is Copy, Eq, Hash -- enables use in HashMaps, const fn constructors, etc.

Provide convenience methods for toolkit interop:
```rust
impl Rgba {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self { ... }
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self { ... }
    pub fn to_f32_array(&self) -> [f32; 4] { ... }
    pub fn to_f32_tuple(&self) -> (f32, f32, f32, f32) { ... }
    pub fn from_f32(r: f32, g: f32, b: f32, a: f32) -> Self { ... }
}
```

### Alpha Default: 0xFF (fully opaque) When Omitted

**Recommendation:** When parsing a 6-digit hex string (#RRGGBB), default alpha to 255 (fully opaque). This follows CSS convention and matches user expectations.

### Color Space: sRGB Only, Document the Assumption

**Recommendation:** All Rgba values are sRGB. Platform readers (later phases) convert P3/other to sRGB on read. Document this in the Rgba doc comment. No color space field in the struct.

### Merge Ownership: Borrow the Overlay (`&mut self, overlay: &Self`)

**Recommendation:** `fn merge(&mut self, overlay: &Self)` -- mutates self in-place, borrows the overlay immutably.

**Rationale:**
- Most idiomatic for "apply overlay on top of base" pattern
- Allows the caller to keep using the overlay after merge (e.g., merge same overlay into multiple bases)
- Matches the IMPLEMENTATION.md example
- Alternative (consuming `self` and `overlay`, returning new) would require cloning if caller needs to reuse either

### Merge Scope: Pairwise, Deep Merge

**Recommendation:** Pairwise merge (two operands), with deep merge for nested sub-structs.

- Pairwise is simpler and covers real use cases: `base.merge(&user_overlay)` or chained `base.merge(&preset); base.merge(&user);`
- Variadic can be added later as a convenience wrapper
- Deep merge means: for nested sub-structs, recursively call merge(); for leaf Option<T> fields, `if overlay.field.is_some() { self.field = overlay.field; }`

### Merge Macro: Public

**Recommendation:** Make the merge macro `pub` (exported from crate root).

**Rationale:**
- Downstream consumers may define extension structs that benefit from the same merge pattern
- Zero cost to make it public; can always restrict later (but `#[non_exhaustive]` makes breaking changes rare)
- The macro name should be descriptive: `impl_merge!` or `mergeable!`

### TOML Structure: Top-Level Variant Sections

**Recommendation:** Use top-level `[light]` and `[dark]` sections (not wrapped in a `[variants]` table):

```toml
name = "KDE Breeze"

[light]
[light.fonts]
family = "Noto Sans"

[light.colors.core]
accent = "#3daee9"

[dark]
[dark.fonts]
family = "Noto Sans"

[dark.colors.core]
accent = "#3daee9"
```

This is the cleanest for human editing and matches the IMPLEMENTATION.md examples.

### Metadata Section: Include Name Only

**Recommendation:** Only `name: String` at the top level. No version field, no metadata section. Rationale from IMPLEMENTATION.md Section 8.1: serde handles backward compatibility; a version field adds validation complexity for little value. Future preset system (Phase 2) can identify presets by name.

### Field Ordering: Logical Grouping in Source

**Recommendation:** Order struct fields logically (core before specific, background before foreground). Serde preserves struct field order during TOML serialization. The TOML output order matches the struct definition order, so logical struct ordering produces logical TOML ordering.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Per-field skip_serializing_if | 50+ `#[serde(skip_serializing_if = "Option::is_none")]` annotations | `serde_with::skip_serializing_none` macro | One annotation per struct instead of per field; prevents forgetting one |
| merge() for each struct | Manual method implementations | Declarative `impl_merge!` macro | Prevents field desynchronization; adding a field to the struct automatically includes it in merge |
| Hex color parsing | Custom character-by-character parsing | `u8::from_str_radix` on 2-char slices | Standard library handles hex parsing correctly |
| Error Display impl | Manual format string for each variant | Straightforward match arms (or thiserror) | Simple enough that hand-rolling is fine; don't need a crate |

**Key insight:** The merge macro is the single most important architectural decision. Without it, every new field requires updating merge() in multiple structs. With it, struct definitions are the single source of truth.

## Common Pitfalls

### Pitfall 1: Forgetting #[serde(default)] on Non-Option Sub-Struct Fields
**What goes wrong:** If ThemeVariant has `pub colors: ThemeColors` without `#[serde(default)]`, a TOML file that omits the entire `[light.colors]` section will fail to deserialize (missing required field).
**Why it happens:** ThemeColors is not Option<T> -- it's a required struct field. Without `#[serde(default)]`, serde expects the key to exist.
**How to avoid:** Always annotate non-Option struct fields with `#[serde(default)]`. The struct must derive Default.
**Warning signs:** Sparse TOML files fail to parse.

### Pitfall 2: Empty TOML Table Headers for All-None Sub-Structs
**What goes wrong:** Serializing a ThemeColors where `core` has all-None fields produces `[light.colors.core]` as an empty section in TOML -- ugly for human editing.
**Why it happens:** serde_with's `skip_serializing_none` skips None leaf fields but the parent struct field (`core: CoreColors`) is not None -- it's a zero-value struct.
**How to avoid:** Add `#[serde(skip_serializing_if = "CoreColors::is_empty")]` on the `core` field in ThemeColors. Implement `is_empty()` (returns true if all fields are None) via the merge macro.
**Warning signs:** Round-trip serialization produces empty table headers.

### Pitfall 3: TOML Table Ordering Constraints
**What goes wrong:** TOML requires all simple key-value pairs before table sections. If a struct has both simple fields and nested struct fields, the simple fields must come first in the serialization output.
**Why it happens:** TOML spec requirement. The toml crate handles this automatically in modern versions (uses `tables_last` internally).
**How to avoid:** Verify with round-trip tests. The toml 1.0.6 crate handles this correctly.
**Warning signs:** Serialization produces invalid TOML.

### Pitfall 4: #[non_exhaustive] Prevents Struct Literal Construction
**What goes wrong:** External crate users cannot construct structs with struct literal syntax (`ThemeColors { accent: Some(...), .. }`) because #[non_exhaustive] prevents it outside the defining crate.
**Why it happens:** This is the intended behavior of #[non_exhaustive] -- it forces use of Default + field mutation or builder pattern.
**How to avoid:** Provide `Default` derive and document the construction pattern: `let mut colors = CoreColors::default(); colors.accent = Some(Rgba::rgb(61, 174, 233));`. Consider builder methods for common use cases. Within the crate, struct literals work fine.
**Warning signs:** Users complain about construction ergonomics.

### Pitfall 5: Merge Macro Must Handle Both Leaf and Nested Fields
**What goes wrong:** A naive merge macro that only handles `Option<T>` fields will fail on ThemeColors, which contains non-Option nested struct fields (e.g., `core: CoreColors`).
**Why it happens:** The macro needs two behaviors: for `Option<T>` fields, do the `if overlay.is_some()` replacement; for nested struct fields, call `.merge()` recursively.
**How to avoid:** Design the macro with two field categories: `option_fields` and `nested_fields`. See Code Examples section.
**Warning signs:** Compile errors when using the macro on ThemeColors or ThemeVariant.

### Pitfall 6: Rgba PartialEq for Testing
**What goes wrong:** Rgba with u8 fields naturally derives PartialEq and Eq, which is fine. But ThemeFonts/ThemeGeometry/ThemeSpacing contain `Option<f32>`, and `f32` does not implement `Eq`. If PartialEq is derived, comparisons use exact float equality.
**Why it happens:** f32 precision and NaN semantics.
**How to avoid:** Derive PartialEq on all structs (it works for round-trip tests since TOML preserves exact float values). Do NOT derive Eq on structs containing f32 fields. For Rgba (u8 only), derive both PartialEq and Eq.
**Warning signs:** Test assertions fail on float fields after transformation (not an issue for simple round-trips).

## Code Examples

### Declarative Merge Macro

```rust
/// Generates `merge()` and `is_empty()` methods for theme structs.
///
/// Two field categories:
/// - `option { field1, field2, ... }` -- Option<T> leaf fields
/// - `nested { field1, field2, ... }` -- nested struct fields with their own merge()
macro_rules! impl_merge {
    (
        $struct_name:ident {
            $(option { $($opt_field:ident),* $(,)? })?
            $(nested { $($nest_field:ident),* $(,)? })?
        }
    ) => {
        impl $struct_name {
            /// Merge an overlay into this value. `Some` fields in the overlay
            /// replace the corresponding fields in self; `None` fields are
            /// left unchanged. Nested structs are merged recursively.
            pub fn merge(&mut self, overlay: &Self) {
                $($(
                    if overlay.$opt_field.is_some() {
                        self.$opt_field = overlay.$opt_field.clone();
                    }
                )*)?
                $($(
                    self.$nest_field.merge(&overlay.$nest_field);
                )*)?
            }

            /// Returns true if all fields are at their default (None/empty) state.
            pub fn is_empty(&self) -> bool {
                true
                $($(&& self.$opt_field.is_none())*)?
                $($(&& self.$nest_field.is_empty())*)?
            }
        }
    };
}

// Usage:
impl_merge!(CoreColors {
    option { accent, background, foreground, surface, border, muted, shadow }
});

impl_merge!(ThemeColors {
    nested { core, primary, secondary, status, interactive, surface, component }
});

impl_merge!(ThemeVariant {
    nested { colors, fonts, geometry, spacing }
});

impl_merge!(ThemeFonts {
    option { family, size, mono_family, mono_size }
});
```

**Note on `.clone()`:** For `Option<Rgba>` fields, Rgba is `Copy` so clone is free. For `Option<String>` fields (in ThemeFonts), clone allocates but this is acceptable for a merge operation that happens rarely.

### Rgba Custom Serde Implementation

```rust
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Convert to [r, g, b, a] in 0.0..=1.0 range (for toolkit interop).
    pub fn to_f32_array(&self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ]
    }
}

impl fmt::Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.a == 255 {
            write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            write!(f, "#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
        }
    }
}

impl FromStr for Rgba {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.strip_prefix('#').unwrap_or(s);
        match hex.len() {
            // #RGB -> #RRGGBB
            3 => {
                let r = u8::from_str_radix(&hex[0..1], 16)
                    .map_err(|e| e.to_string())?;
                let g = u8::from_str_radix(&hex[1..2], 16)
                    .map_err(|e| e.to_string())?;
                let b = u8::from_str_radix(&hex[2..3], 16)
                    .map_err(|e| e.to_string())?;
                Ok(Rgba::rgb(r * 17, g * 17, b * 17))
            }
            // #RGBA -> #RRGGBBAA
            4 => {
                let r = u8::from_str_radix(&hex[0..1], 16)
                    .map_err(|e| e.to_string())?;
                let g = u8::from_str_radix(&hex[1..2], 16)
                    .map_err(|e| e.to_string())?;
                let b = u8::from_str_radix(&hex[2..3], 16)
                    .map_err(|e| e.to_string())?;
                let a = u8::from_str_radix(&hex[3..4], 16)
                    .map_err(|e| e.to_string())?;
                Ok(Rgba::rgba(r * 17, g * 17, b * 17, a * 17))
            }
            // #RRGGBB
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16)
                    .map_err(|e| e.to_string())?;
                let g = u8::from_str_radix(&hex[2..4], 16)
                    .map_err(|e| e.to_string())?;
                let b = u8::from_str_radix(&hex[4..6], 16)
                    .map_err(|e| e.to_string())?;
                Ok(Rgba::rgb(r, g, b))
            }
            // #RRGGBBAA
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16)
                    .map_err(|e| e.to_string())?;
                let g = u8::from_str_radix(&hex[2..4], 16)
                    .map_err(|e| e.to_string())?;
                let b = u8::from_str_radix(&hex[4..6], 16)
                    .map_err(|e| e.to_string())?;
                let a = u8::from_str_radix(&hex[6..8], 16)
                    .map_err(|e| e.to_string())?;
                Ok(Rgba::rgba(r, g, b, a))
            }
            _ => Err(format!(
                "invalid hex color length {}: expected 3, 4, 6, or 8 hex digits",
                hex.len()
            )),
        }
    }
}

impl Serialize for Rgba {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Rgba {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Rgba::from_str(&s).map_err(de::Error::custom)
    }
}
```

### Error Enum

```rust
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Operation not supported on the current platform.
    Unsupported,
    /// Data source exists but cannot be read right now.
    Unavailable(String),
    /// TOML parsing or serialization error.
    Format(String),
    /// Wrapped platform-specific error.
    Platform(Box<dyn std::error::Error + Send + Sync>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unsupported => write!(f, "operation not supported on this platform"),
            Error::Unavailable(msg) => write!(f, "theme data unavailable: {msg}"),
            Error::Format(msg) => write!(f, "theme format error: {msg}"),
            Error::Platform(err) => write!(f, "platform error: {err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Platform(err) => Some(&**err),
            _ => None,
        }
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::Format(err.to_string())
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Error::Format(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Unavailable(err.to_string())
    }
}
```

### NativeTheme and ThemeVariant

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NativeTheme {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub light: Option<ThemeVariant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dark: Option<ThemeVariant>,
}

impl Default for NativeTheme {
    fn default() -> Self {
        Self {
            name: String::new(),
            light: None,
            dark: None,
        }
    }
}

impl NativeTheme {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            light: None,
            dark: None,
        }
    }

    /// Merge an overlay theme into this theme.
    pub fn merge(&mut self, overlay: &NativeTheme) {
        // Merge light variant
        match (&mut self.light, &overlay.light) {
            (Some(base), Some(over)) => base.merge(over),
            (None, Some(over)) => self.light = Some(over.clone()),
            _ => {}
        }
        // Merge dark variant
        match (&mut self.dark, &overlay.dark) {
            (Some(base), Some(over)) => base.merge(over),
            (None, Some(over)) => self.dark = Some(over.clone()),
            _ => {}
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ThemeVariant {
    pub colors: ThemeColors,
    pub fonts: ThemeFonts,
    pub geometry: ThemeGeometry,
    pub spacing: ThemeSpacing,
}
```

### Round-Trip TOML Test Pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_full_theme() {
        let mut theme = NativeTheme::new("Test Theme");
        let mut variant = ThemeVariant::default();
        variant.colors.core.accent = Some(Rgba::rgb(61, 174, 233));
        variant.colors.core.background = Some(Rgba::rgb(239, 240, 241));
        variant.fonts.family = Some("Noto Sans".to_string());
        variant.fonts.size = Some(10.0);
        variant.geometry.radius = Some(5.0);
        variant.spacing.m = Some(8.0);
        theme.light = Some(variant);

        let toml_str = toml::to_string_pretty(&theme).unwrap();
        let deserialized: NativeTheme = toml::from_str(&toml_str).unwrap();

        assert_eq!(theme.name, deserialized.name);
        assert_eq!(
            theme.light.as_ref().unwrap().colors.core.accent,
            deserialized.light.as_ref().unwrap().colors.core.accent
        );
    }

    #[test]
    fn sparse_toml_deserializes() {
        let sparse = r#"
            name = "Minimal"
            [light.colors.core]
            accent = "#e91e63"
        "#;
        let theme: NativeTheme = toml::from_str(sparse).unwrap();
        assert_eq!(theme.light.unwrap().colors.core.accent, Some(Rgba::rgb(233, 30, 99)));
    }

    #[test]
    fn serialization_skips_none_fields() {
        let mut theme = NativeTheme::new("Sparse");
        let mut variant = ThemeVariant::default();
        variant.colors.core.accent = Some(Rgba::rgb(61, 174, 233));
        theme.light = Some(variant);

        let toml_str = toml::to_string_pretty(&theme).unwrap();
        assert!(!toml_str.contains("background"));  // None fields omitted
        assert!(toml_str.contains("accent"));        // Some fields present
    }
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Rust edition 2021 | Rust edition 2024 | Feb 2025 (Rust 1.85) | Use `edition = "2024"` in Cargo.toml; new features available |
| toml 0.5.x | toml 1.0.6 (TOML spec 1.1.0) | Stable since 2023 | Full TOML 1.0/1.1 support; tables_last handled internally |
| serde 1.0.1xx | serde 1.0.228 | Ongoing | No breaking changes; latest minor version |
| Manual skip_serializing_if | serde_with 3.17 #[skip_serializing_none] | Available since serde_with 1.x | Dramatically reduces annotation boilerplate |

**Deprecated/outdated:**
- toml 0.5.x: Still works but 1.0.x is the standard; 0.5 had different API and TOML spec support
- serde(flatten) with TOML: Has known edge cases; prefer natural nesting for TOML serialization

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in test framework (cargo test) |
| Config file | none -- Rust tests are convention-based |
| Quick run command | `cargo test` |
| Full suite command | `cargo test --all-features` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| CORE-01 | Rgba hex parse/serialize, FromStr, Display | unit | `cargo test --lib color` | Wave 0 |
| CORE-02 | ThemeColors 36 roles across nested sub-structs | unit | `cargo test --lib model::colors` | Wave 0 |
| CORE-03 | ThemeFonts serde round-trip | unit | `cargo test --lib model::fonts` | Wave 0 |
| CORE-04 | ThemeGeometry serde round-trip | unit | `cargo test --lib model::geometry` | Wave 0 |
| CORE-05 | ThemeSpacing serde round-trip | unit | `cargo test --lib model::spacing` | Wave 0 |
| CORE-06 | ThemeVariant composition | unit | `cargo test --lib model` | Wave 0 |
| CORE-07 | NativeTheme with light/dark variants | unit | `cargo test --lib model` | Wave 0 |
| CORE-08 | merge() on all structs | unit | `cargo test --lib merge` | Wave 0 |
| CORE-09 | non_exhaustive on public structs | compile-time | Verified by derive attribute presence | N/A |
| CORE-10 | Send + Sync + Default + Clone + Debug | unit | `cargo test --lib traits` | Wave 0 |
| SERDE-01 | TOML round-trip all types | integration | `cargo test --test serde_roundtrip` | Wave 0 |
| SERDE-02 | skip_serializing_if + serde(default) | unit | `cargo test --lib sparse` | Wave 0 |
| ERR-01 | Error enum Display + Error impls | unit | `cargo test --lib error` | Wave 0 |
| TEST-01 | Round-trip serde tests for all types | integration | `cargo test --test serde_roundtrip` | Wave 0 |
| TEST-03 | Rgba hex parsing edge cases | unit | `cargo test --lib color::tests` | Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test`
- **Per wave merge:** `cargo test --all-features`
- **Phase gate:** Full suite green before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `Cargo.toml` -- project initialization with dependencies
- [ ] `src/lib.rs` -- crate root with re-exports
- [ ] `src/color.rs` -- Rgba type and tests
- [ ] `src/error.rs` -- Error enum
- [ ] `src/model/mod.rs` -- NativeTheme, ThemeVariant
- [ ] `src/model/colors.rs` -- ThemeColors and sub-structs
- [ ] `src/model/fonts.rs` -- ThemeFonts
- [ ] `src/model/geometry.rs` -- ThemeGeometry
- [ ] `src/model/spacing.rs` -- ThemeSpacing

All files are Wave 0 gaps because this is a greenfield project with no existing source code.

## Open Questions

1. **Naming collision in nested colors: `surface` appears as both a CoreColors field and a ThemeColors sub-struct group name (SurfaceColors)**
   - What we know: The IMPLEMENTATION.md uses `surface` as a CoreColors field (card/view/content background) and the research proposes SurfaceColors as a group for sidebar/tooltip/popover
   - What's unclear: Whether the TOML path `colors.surface.sidebar` reads well or is confusing (does `surface` refer to the surface color or the surface group?)
   - Recommendation: Rename the CoreColors field to `content_background` or keep `surface` and name the group `panels` instead of `surface`. Alternatively, rename the sub-struct to `SurfaceVariants` or `PanelColors`. The planner should make a decision here. A simple rename of the group to something like `panel` avoids the collision: `[light.colors.panel]` for sidebar/tooltip/popover.

2. **serde_with dependency weight**
   - What we know: serde_with 3.17.0 is a proc-macro crate that adds compile time
   - What's unclear: Whether the compile-time cost is acceptable for a library crate that aims to be lightweight
   - Recommendation: Use serde_with. The alternative is ~50 identical `#[serde(skip_serializing_if = "Option::is_none")]` annotations. The compile-time cost is paid once and is modest. If compile time becomes a concern, these annotations can be added manually later as a mechanical replacement.

3. **NativeTheme merge behavior for `name` field**
   - What we know: NativeTheme has a `name: String` field. The merge macro handles `Option<T>` and nested structs, but `name` is a plain `String`.
   - What's unclear: Should merge replace the name? Keep the base name? Concatenate?
   - Recommendation: Keep the base name (do not replace). The name identifies the base theme. If the user wants a different name, they set it explicitly after merge. NativeTheme.merge() should handle name separately from the macro (it already needs custom logic for Option<ThemeVariant> fields).

## Sources

### Primary (HIGH confidence)
- Project IMPLEMENTATION.md (docs/IMPLEMENTATION.md) -- complete data model specification, TOML format, platform mappings, design decisions
- Project REQUIREMENTS.md (.planning/REQUIREMENTS.md) -- all 15 phase requirements with descriptions
- Project ROADMAP.md (.planning/ROADMAP.md) -- phase dependencies, success criteria
- [serde.rs field attributes](https://serde.rs/field-attrs.html) -- skip_serializing_if, default behavior
- [serde.rs container attributes](https://serde.rs/container-attrs.html) -- non_exhaustive + serde(default) interaction
- [Rust Reference: non_exhaustive](https://doc.rust-lang.org/reference/attributes/type_system.html) -- non_exhaustive semantics
- cargo search: serde 1.0.228, toml 1.0.6, serde_with 3.17.0 -- verified via `cargo search`

### Secondary (MEDIUM confidence)
- [toml crate docs](https://docs.rs/toml) -- TOML serialization behavior with nested structs
- [serde_with skip_serializing_none](https://docs.rs/serde_with/latest/serde_with/attr.skip_serializing_none.html) -- macro behavior
- [egui Color32 docs](https://docs.rs/egui/latest/egui/struct.Color32.html) -- u8 representation confirmed
- [iced Color docs](https://docs.rs/iced/latest/iced/struct.Color.html) -- f32 representation confirmed

### Tertiary (LOW confidence)
- WebSearch results on serde(flatten) + TOML issues -- multiple sources confirm known edge cases but exact current status of fixes is uncertain

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- serde + toml are the only Rust choices; versions verified via cargo search
- Architecture: HIGH -- IMPLEMENTATION.md provides complete specification; nested sub-struct design is a straightforward Rust pattern
- Pitfalls: HIGH -- most pitfalls are well-known serde patterns documented in official serde.rs docs
- Merge macro: MEDIUM -- the macro pattern is standard Rust, but exact syntax may need iteration during implementation

**Research date:** 2026-03-07
**Valid until:** 2026-04-07 (30 days -- stable domain, no fast-moving dependencies)
