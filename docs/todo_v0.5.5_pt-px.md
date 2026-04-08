# Explicit units for all dimensional fields in TOML presets

Status: Chapter 2 implemented (Phase 59), Chapter 3 pending
Date: 2026-04-08

---

## Chapter 1 — Problem analysis and options

### 1.1 The problem

Font sizes in TOML presets are ambiguous: some are in typographic **points**,
others in logical **pixels**, and there is no way to tell which is which by
reading the TOML alone.

Current state:

| Category | Fields affected | Unit | How the unit is determined |
|----------|----------------|------|---------------------------|
| Platform preset fonts | `font.size`, `mono_font.size`, per-widget `.font.size`, `text_scale.*.size`, `text_scale.*.line_height` | **pt** (typographic points) | `font_dpi` is set at runtime by the OS reader |
| Non-platform preset fonts | Same fields | **px** (logical pixels) | `font_dpi` is `None` — no conversion happens |
| Widget geometry | All other dimension fields (padding, min_width, corner_radius, ...) | **px** (always) | Convention; never converted |
| Ratios | `line_height`, `disabled_opacity`, `text_scaling_factor` | unitless | Convention |

The root cause: `font_dpi` is `#[serde(skip)]` — it never appears in the TOML.
A reader of the TOML has no signal to distinguish `font.size = 10` (meaning
10 pt on KDE Breeze) from `font.size = 14` (meaning 14 px on Dracula).

Only `windows-11.toml` and `windows-11-live.toml` have a comment explaining
the unit. All other presets are silent.

The ambiguity is worst for font sizes (pt vs px confusion causes ~25-33%
sizing errors), but the broader problem is that **no** dimensional field
in TOML carries its unit. A reader must know by convention that
`corner_radius = 5.0` means pixels and `font.size = 10.0` means points.
Chapter 2 solves font sizes; Chapter 3 extends explicit units to all
dimensional fields.

### 1.2 Affected fields (exhaustive list)

All fields flowing through the `convert_pt_to_px()` path in
`resolve/inheritance.rs:7-11`:

1. `defaults.font.size` — primary UI font
2. `defaults.mono_font.size` — monospace font
3. 19 per-widget `.font.size` fields (window title_bar_font, button, input,
   checkbox, menu, tooltip, tab, sidebar, toolbar, status_bar, list.item_font,
   list.header_font, popover, dialog.title_font, dialog.body_font, combo_box,
   segmented_control, expander, link)
4. `text_scale.caption.size`, `.section_heading.size`, `.dialog_title.size`,
   `.display.size`
5. `text_scale.*.line_height` — these are in the same unit as the size they
   were derived from (pt when `font_dpi` is set, px otherwise)

### 1.3 How the pipeline works today

```
TOML preset (font sizes: pt or px — ambiguous)
  |
  v
OS reader (KDE/GNOME/Windows) populates variant + sets font_dpi
  |
  v
ThemeVariant with font_dpi: Some(96.0) or None
  |
  v
resolve() phases:
  Phase 1   : defaults internal chains (accent -> selection, etc.)
  Phase 1.5 : resolve_font_dpi_conversion()
              if font_dpi > 0: px = pt * font_dpi / 72
              then clears font_dpi to None (idempotency guard)
  Phase 2   : safety nets
  Phase 3   : widget-from-defaults (font inheritance, colors, borders)
  Phase 4   : widget-to-widget chains
  Phase 5   : icon set fallback
  |
  v
validate() -> ResolvedThemeVariant (all font sizes guaranteed in px)
```

### 1.4 Options considered

#### Option 1: Rename TOML keys with `_pt` / `_px` suffix

In the TOML, font sizes become `size_pt = 10` or `size_px = 14`. The Rust
struct gets two mutually-exclusive fields (`size_pt: Option<f32>`,
`size_px: Option<f32>`) or a single enum field with flatten-based serde.

**Pros:**
- Unambiguous at point of use — impossible to misread
- Machine-enforceable (TOML linter can reject bare `size`)
- Each preset declares its own unit independently

**Cons:**
- Breaking change: all TOML files, Rust structs, connectors, tests
- Two field names for the same concept
- Every code path touching font size must handle both variants
- Merge logic becomes complex (what if base has `size_pt` and overlay `size_px`?)
- Validation needed to reject both being set simultaneously
- No compile-time guarantee — a plain `Option<f32>` pair can still be misused

#### Option 2: Standardize all TOML values to pixels

Eliminate points from the TOML layer. OS readers convert pt -> px before
populating the variant. `font_dpi` becomes reader-internal.

**Pros:**
- Eliminates ambiguity completely — one unit everywhere
- Simplest mental model
- No field renames, no struct changes beyond removing `font_dpi`
- Resolution pipeline simplifies (no Phase 1.5)

**Cons:**
- Live preset `text_scale` sizes become DPI-dependent — must pre-convert at an
  assumed DPI (96?), wrong on other systems
- Loses the platform-native "source of truth" (10 pt is what KDE actually uses)
- OS readers must know the display DPI before populating the struct
- Hand-authored platform presets must reverse-engineer px from platform docs
  that spec in pt

#### Option 3: Tagged value type in TOML

Font sizes become strings `"10pt"` or inline tables `{ value = 10, unit = "pt" }`.
Custom serde deserializer handles both forms plus bare `f32` for backward compat.

**Pros:**
- Self-documenting per-value
- Can mix pt and px freely within one file
- Backward-compatible if bare `f32` defaults to px

**Cons:**
- Breaks the natural `size = 10.0` numeric syntax — `"10pt"` is a string
- TOML validators/formatters won't treat these as numeric
- Complex serde implementation (custom deserializer for 3 input forms)
- Inconsistent: only font sizes are tagged, everything else is bare `f32`
- The inline table form is verbose for something appearing 25+ times per preset

#### Option 4: Documentation only (comments + per-file metadata section)

Add a `[_meta]` section to each TOML (`font_size_unit = "pt"`) and comments.

**Pros:**
- Zero breaking changes
- Can be done immediately
- `_meta` is machine-readable

**Cons:**
- Not enforced — comments and metadata drift from reality
- Users may not notice `_meta` while editing font sizes lower in the file
- Doesn't prevent misuse when copy-pasting values between presets
- If code checks `_meta`, it's just a second `font_dpi` mechanism

#### Option 5: Rust newtype wrappers with suffix-based serde (chosen)

Define a `FontSize` enum (`Pt(f32)` / `Px(f32)`) with custom serde that
maps to `size_pt` or `size_px` in TOML via a flattened helper struct.

TOML:
```toml
[font]
size_pt = 10       # platform preset — points

[font]
size_px = 14       # community preset — pixels
```

Rust:
```rust
pub enum FontSize {
    Pt(f32),
    Px(f32),
}
```

**Pros:**
- Unambiguous in TOML: `size_pt = 10` or `size_px = 14`
- Compile-time safety: impossible to pass pt where px is expected
- Resolution becomes a type transformation (`FontSize -> f32` in px),
  replacing a hidden mutation
- `font_dpi` can leave `ThemeDefaults` — becomes a resolution parameter
- TOML stays numeric (no strings)
- Enforced: the code *knows* what unit a value is in

**Cons:**
- Significant refactoring: FontSpec, TextScaleEntry, all widget font fields,
  merge logic, resolve pipeline, connectors, tests
- Custom serde for flatten-to-suffix pattern
- Widget geometry stays bare `f32`, creating two patterns in the model
- Contributors must understand the enum

#### Option 6: Require `font_dpi` in TOML (remove `#[serde(skip)]`)

Formalize the existing convention: `font_dpi` presence = font sizes are in pt.

**Pros:**
- Minimal change — remove `#[serde(skip)]` + add doc comments
- Backward-compatible

**Cons:**
- Indirect — must notice `font_dpi` at file top, then remember while reading
  `font.size = 10.0` much later
- `font_dpi` is a display property, not a theme property — muddies the model
- For live presets, OS reader already provides `font_dpi` — TOML copy creates
  two sources
- Doesn't label individual values

### 1.5 Decision

**Option 5** — newtype wrappers with suffix-based serde.

Rationale: if we're going to touch every font-size field anyway (which any
non-documentation-only solution requires), we should get compile-time safety
for the effort. Option 5 is the only option where a confused-unit bug becomes
a compile error.

---

## Chapter 2 — Implementation plan for Option 5

### 2.1 New type: `FontSize`

File: `native-theme/src/model/font.rs`

```rust
/// A font size with an explicit unit.
///
/// In TOML presets, this appears as either `size_pt` (typographic points)
/// or `size_px` (logical pixels). Serde mapping is handled by the parent
/// struct (`FontSpec`, `TextScaleEntry`) — `FontSize` itself has no
/// `Serialize`/`Deserialize` impl.
///
/// During validation, all `FontSize` values are converted to logical pixels
/// via `FontSize::to_px(dpi)`, producing a plain `f32` for the resolved model.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontSize {
    /// Typographic points (1/72 inch). Used by platform presets where the OS
    /// reports font sizes in points (KDE, GNOME, Windows).
    /// Converted to px during validation: `px = pt * dpi / 72`.
    Pt(f32),
    /// Logical pixels. Used by community/non-platform presets where font sizes
    /// are hand-authored in pixels.
    Px(f32),
}

impl FontSize {
    /// Convert to logical pixels.
    ///
    /// - `Pt(v)` -> `v * dpi / 72.0`
    /// - `Px(v)` -> `v` (dpi ignored)
    pub fn to_px(self, dpi: f32) -> f32 {
        match self {
            Self::Pt(v) => v * dpi / 72.0,
            Self::Px(v) => v,
        }
    }

    /// Return the raw numeric value regardless of unit.
    /// Used during inheritance to compute derived values (e.g. line_height)
    /// before unit conversion.
    pub fn raw(self) -> f32 {
        match self {
            Self::Pt(v) | Self::Px(v) => v,
        }
    }

    /// True when the value is in typographic points.
    pub fn is_pt(self) -> bool {
        matches!(self, Self::Pt(_))
    }
}

impl Default for FontSize {
    fn default() -> Self {
        Self::Px(0.0)
    }
}
```

`FontSize` does **not** implement `Serialize` or `Deserialize`. It never
appears as a standalone TOML value — the `size_pt`/`size_px` mapping lives
on the parent struct (see 2.2, 2.3).

### 2.2 Changes to `FontSpec`

File: `native-theme/src/model/font.rs`

#### 2.2.1 Struct change

Before:
```rust
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FontSpec {
    pub family: Option<String>,
    pub size: Option<f32>,        // ambiguous unit
    pub weight: Option<u16>,
    pub style: Option<FontStyle>,
    pub color: Option<Rgba>,
}
```

After:
```rust
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "FontSpecRaw", into = "FontSpecRaw")]
pub struct FontSpec {
    pub family: Option<String>,
    pub size: Option<FontSize>,   // explicit unit
    pub weight: Option<u16>,
    pub style: Option<FontStyle>,
    pub color: Option<Rgba>,
}
```

The field stays `Option<FontSize>` — `None` still means "not set / inherit
from defaults," preserving the existing partial-overlay semantics.

#### 2.2.2 Serde proxy struct

`FontSpec` uses `#[serde(try_from, into)]` to delegate serialization to a
flat proxy struct. The proxy has derived `Serialize`/`Deserialize`, so adding
future fields to `FontSpec` only requires adding them to the proxy too — no
hand-written serialization logic.

```rust
/// Serde proxy for FontSpec. Maps `FontSize` to two mutually-exclusive keys.
#[serde_with::skip_serializing_none]
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
struct FontSpecRaw {
    family: Option<String>,
    size_pt: Option<f32>,
    size_px: Option<f32>,
    weight: Option<u16>,
    style: Option<FontStyle>,
    color: Option<Rgba>,
}

impl TryFrom<FontSpecRaw> for FontSpec {
    type Error = String;
    fn try_from(raw: FontSpecRaw) -> Result<Self, Self::Error> {
        let size = match (raw.size_pt, raw.size_px) {
            (Some(v), None) => Some(FontSize::Pt(v)),
            (None, Some(v)) => Some(FontSize::Px(v)),
            (None, None)    => None,
            (Some(_), Some(_)) => return Err(
                "font: set `size_pt` or `size_px`, not both".into()
            ),
        };
        Ok(FontSpec {
            family: raw.family, size, weight: raw.weight,
            style: raw.style, color: raw.color,
        })
    }
}

impl From<FontSpec> for FontSpecRaw {
    fn from(fs: FontSpec) -> Self {
        let (size_pt, size_px) = match fs.size {
            Some(FontSize::Pt(v)) => (Some(v), None),
            Some(FontSize::Px(v)) => (None, Some(v)),
            None                  => (None, None),
        };
        FontSpecRaw {
            family: fs.family, size_pt, size_px,
            weight: fs.weight, style: fs.style, color: fs.color,
        }
    }
}
```

The old bare `size` key is gone. TOML with `size = 10.0` will fail to
deserialize — TOML linting will also flag it via the updated `FIELD_NAMES`.

#### 2.2.3 FIELD_NAMES update

```rust
pub const FIELD_NAMES: &[&str] = &[
    "family", "size_pt", "size_px", "weight", "style", "color"
];
```

#### 2.2.4 `impl_merge!` — no change needed

The current macro invocation stays as-is:

```rust
impl_merge!(FontSpec {
    option { family, size, weight, style, color }
});
```

The `option` arm does `if overlay.size.is_some() { self.size = overlay.size.clone() }`.
`Option<FontSize>` satisfies `is_some()`, `is_none()`, and `Clone` — the
macro works without modification.

**Mixed-unit merge:** If the base has `Some(Pt(10))` and the overlay has
`Some(Px(14))`, the overlay wins (standard `option` merge). This is correct:
the overlay is a more-specific declaration. No cross-unit arithmetic needed.

### 2.3 Changes to `TextScaleEntry`

File: `native-theme/src/model/font.rs`

Same pattern as `FontSpec`: change `size` from `Option<f32>` to
`Option<FontSize>`, add a serde proxy.

```rust
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "TextScaleEntryRaw", into = "TextScaleEntryRaw")]
pub struct TextScaleEntry {
    pub size: Option<FontSize>,       // was Option<f32>
    pub weight: Option<u16>,
    pub line_height: Option<f32>,     // stays f32 — see below
}
```

**`line_height` stays `f32`:** It shares the unit of its sibling `size`
(both pt in platform presets, both px in community presets), but it is a
layout metric, not a font size. Wrapping it in `FontSize` would stretch the
type's semantic meaning. Instead, validation converts it using the same DPI
when the sibling `size` is `Pt` (see 2.5).

The proxy struct (`TextScaleEntryRaw`) mirrors the `FontSpecRaw` pattern —
`size_pt`, `size_px` as mutually-exclusive keys, `line_height` passed through.

`FIELD_NAMES` update:
```rust
pub const FIELD_NAMES: &[&str] = &["size_pt", "size_px", "weight", "line_height"];
```

`impl_merge!` — no change needed (same reasoning as FontSpec).

### 2.4 Resolution pipeline

Files: `native-theme/src/resolve/inheritance.rs`, `native-theme/src/resolve/mod.rs`

#### 2.4.1 Delete Phase 1.5

Delete `convert_pt_to_px()` (inheritance.rs:7-11) and the entire
`resolve_font_dpi_conversion()` method (inheritance.rs:174-228). Their job
moves to validation (see 2.5), where `FontSize::to_px(dpi)` replaces the
hidden in-place mutation.

This is architecturally cleaner: resolve fills `None` fields via inheritance
(operating on `Option<FontSize>` without touching units), and validate
extracts the final `f32` in px.

Remove the Phase 1.5 call from `resolve()` (mod.rs:36):

```rust
pub fn resolve(&mut self) {
    self.resolve_defaults_internal();
    self.resolve_safety_nets();
    self.resolve_widgets_from_defaults();
    self.resolve_widget_to_widget();
    if self.icon_set.is_none() {
        self.icon_set = Some(crate::model::icons::system_icon_set());
    }
}
```

Update the doc comment to remove the Phase 1.5 entry.

#### 2.4.2 Inheritance — transparent propagation

`resolve_font()` (inheritance.rs:16-39) is unchanged in logic. The `size`
field is now `Option<FontSize>` instead of `Option<f32>`, but the code is
identical:

```rust
if font.size.is_none() {
    font.size = defaults_font.size;  // copies Option<FontSize> — unit preserved
}
```

A `Pt(10)` from defaults propagates as `Pt(10)` to every widget that inherits.
The unit is never stripped during inheritance.

#### 2.4.3 Text scale line_height computation

Current code (inheritance.rs:90-94) computes line_height from a ratio and size:

```rust
if entry.line_height.is_none()
    && let (Some(lh_mult), Some(size)) = (defaults_line_height, entry.size)
{
    entry.line_height = Some(lh_mult * size);
}
```

Change: extract the raw value via `.raw()` (the result stays in the same
unit as the font size — converted later in validate):

```rust
if entry.line_height.is_none()
    && let (Some(lh_mult), Some(font_size)) = (defaults_line_height, entry.size)
{
    entry.line_height = Some(lh_mult * font_size.raw());
}
```

### 2.5 Validation

File: `native-theme/src/resolve/validate.rs`

#### 2.5.1 `require_font` gains a `dpi` parameter

```rust
fn require_font(
    font: &FontSpec,
    prefix: &str,
    dpi: f32,
    missing: &mut Vec<String>,
) -> ResolvedFontSpec {
    let family = require(&font.family, &format!("{prefix}.family"), missing);
    let size = font.size
        .map(|fs| fs.to_px(dpi))
        .unwrap_or_else(|| { missing.push(format!("{prefix}.size")); 0.0 });
    let weight = require(&font.weight, &format!("{prefix}.weight"), missing);
    let color = require(&font.color, &format!("{prefix}.color"), missing);
    ResolvedFontSpec {
        family: family.cloned().unwrap_or_default(),
        size,
        weight: weight.copied().unwrap_or_default(),
        style: font.style.unwrap_or_default(),
        color: color.copied().unwrap_or_default(),
    }
}
```

#### 2.5.2 Text scale entry: convert `line_height` alongside `size`

`line_height` is a bare `f32` whose unit matches the sibling `size`.
Use `FontSize::is_pt()` to decide whether to convert:

```rust
fn require_text_scale_entry(
    entry: &TextScaleEntry,
    prefix: &str,
    dpi: f32,
    missing: &mut Vec<String>,
) -> ResolvedTextScaleEntry {
    let size = entry.size
        .map(|fs| fs.to_px(dpi))
        .unwrap_or_else(|| { missing.push(format!("{prefix}.size")); 0.0 });

    let needs_pt_conversion = entry.size.is_some_and(|fs| fs.is_pt());
    let line_height = entry.line_height
        .map(|lh| if needs_pt_conversion { lh * dpi / 72.0 } else { lh })
        .unwrap_or_else(|| { missing.push(format!("{prefix}.line_height")); 0.0 });

    let weight = require(&entry.weight, &format!("{prefix}.weight"), missing);
    ResolvedTextScaleEntry {
        size,
        weight: weight.copied().unwrap_or_default(),
        line_height,
    }
}
```

#### 2.5.3 DPI source

Keep `font_dpi` on `ThemeDefaults` as a runtime-only field. Update its doc
comment to clarify its new role:

```rust
/// Font DPI for pt-to-px conversion during validation.
///
/// When font sizes are `FontSize::Pt(v)`, validation converts them via
/// `v * font_dpi / 72`. When font sizes are `FontSize::Px(v)`, this field
/// is ignored for those values.
///
/// Set by OS readers or auto-detected in `into_resolved()`.
#[serde(skip)]
pub font_dpi: Option<f32>,
```

`validate()` reads `self.defaults.font_dpi.unwrap_or(96.0)` as the DPI
for all conversions and passes it to `require_font` / `require_text_scale_entry`.

### 2.6 OS readers

Each OS reader currently stores font sizes as bare `f32` in points.
Change: wrap in `FontSize::Pt(...)`.

| Reader | File | Function | Change |
|--------|------|----------|--------|
| KDE | `kde/fonts.rs` | `parse_qt_font_with_weight()` | `size: Some(v)` -> `size: Some(FontSize::Pt(v))` |
| GNOME | `gnome/mod.rs` | `parse_gnome_font_to_fontspec()` | same |
| Windows | `windows.rs` | `logfont_to_fontspec_raw()` | same |

`font_dpi` assignment stays as-is in all readers — same field, same value.

### 2.7 TOML presets

20 preset files. The change is a mechanical rename of `size` keys.

**Platform presets** — `size` becomes `size_pt`:

`kde-breeze.toml`, `kde-breeze-live.toml`, `adwaita.toml`,
`adwaita-live.toml`, `macos-sonoma.toml`, `macos-sonoma-live.toml`,
`windows-11.toml`, `windows-11-live.toml`, `ios.toml`

```toml
# Before                          # After
[light.text_scale.caption]        [light.text_scale.caption]
size = 8.2                        size_pt = 8.2
weight = 400                      weight = 400
line_height = 11.2                line_height = 11.2
```

**Non-platform presets** — `size` becomes `size_px`:

`catppuccin-mocha.toml`, `catppuccin-frappe.toml`, `catppuccin-latte.toml`,
`catppuccin-macchiato.toml`, `one-dark.toml`, `nord.toml`, `dracula.toml`,
`gruvbox.toml`, `solarized.toml`, `tokyo-night.toml`, `material.toml`

```toml
# Before                          # After
[light.defaults.font]             [light.defaults.font]
family = "Inter"                  family = "Inter"
size = 14.0                       size_px = 14.0
weight = 400                      weight = 400
```

Remove the now-unnecessary unit comment from `windows-11.toml` and
`windows-11-live.toml` (the suffix makes it self-documenting).

### 2.8 Unchanged components

The following require **no code changes**:

- **`ResolvedFontSpec`** and **`ResolvedTextScaleEntry`** (`model/resolved.rs`)
  — already `f32` in logical pixels (output of validation).
- **Connectors** (`native-theme-gpui`, `native-theme-iced`) — consume only
  `Resolved*` types, which are unchanged.
- **`ThemeDefaults`** (`model/defaults.rs`) — `font_dpi` stays as-is.
  `impl_merge!` and `FIELD_NAMES` are unaffected (font fields are `nested`,
  delegating to `FontSpec` which has its own `FIELD_NAMES`).
- **`into_resolved()`** (`resolve/mod.rs`) — still auto-detects `font_dpi`,
  calls `resolve_all()`, calls `validate()`. The only difference is that
  `resolve_all()` no longer runs Phase 1.5 (already handled in 2.4.1).
- **`impl_merge!` macro** (`lib.rs`) — no change to macro definition.

### 2.9 `property-registry.toml`

File: `docs/property-registry.toml`

Update the Font structure:
```toml
[_structures.Font]
# Exactly one of size_pt or size_px must be set.
#   size_pt — typographic points (1/72 inch), used by platform presets
#   size_px — logical pixels, used by community presets
family      = "string"
size_pt     = "f32"
size_px     = "f32"
weight      = "u16"
style       = "enum"
color       = "color"
```

### 2.10 Tests

#### FontSize unit tests

```rust
#[test]
fn pt_to_px_at_96_dpi() {
    assert_eq!(FontSize::Pt(10.0).to_px(96.0), 10.0 * 96.0 / 72.0);
}

#[test]
fn px_ignores_dpi() {
    assert_eq!(FontSize::Px(14.0).to_px(96.0), 14.0);
    assert_eq!(FontSize::Px(14.0).to_px(144.0), 14.0);
}

#[test]
fn pt_to_px_at_72_dpi_is_identity() {
    assert_eq!(FontSize::Pt(10.0).to_px(72.0), 10.0);
}

#[test]
fn raw_extracts_value() {
    assert_eq!(FontSize::Pt(10.0).raw(), 10.0);
    assert_eq!(FontSize::Px(14.0).raw(), 14.0);
}
```

#### Serde round-trip tests

```rust
#[test]
fn fontspec_toml_round_trip_size_pt() {
    let fs = FontSpec {
        family: Some("Inter".into()),
        size: Some(FontSize::Pt(10.0)),
        weight: Some(400),
        ..Default::default()
    };
    let toml_str = toml::to_string(&fs).unwrap();
    assert!(toml_str.contains("size_pt"));
    assert!(!toml_str.contains("size_px"));
    let deserialized: FontSpec = toml::from_str(&toml_str).unwrap();
    assert_eq!(deserialized, fs);
}

#[test]
fn fontspec_toml_round_trip_size_px() {
    let fs = FontSpec {
        size: Some(FontSize::Px(14.0)),
        ..Default::default()
    };
    let toml_str = toml::to_string(&fs).unwrap();
    assert!(toml_str.contains("size_px"));
    assert!(!toml_str.contains("size_pt"));
    let deserialized: FontSpec = toml::from_str(&toml_str).unwrap();
    assert_eq!(deserialized, fs);
}

#[test]
fn fontspec_toml_rejects_both_pt_and_px() {
    let toml_str = r#"size_pt = 10.0
size_px = 14.0"#;
    assert!(toml::from_str::<FontSpec>(toml_str).is_err());
}

#[test]
fn fontspec_toml_rejects_bare_size() {
    let toml_str = r#"size = 10.0"#;
    assert!(toml::from_str::<FontSpec>(toml_str).is_err());
}

#[test]
fn fontspec_toml_no_size_is_valid() {
    let fs: FontSpec = toml::from_str(r#"family = "Inter""#).unwrap();
    assert!(fs.size.is_none());
}
```

#### Existing test updates

| Test location | Change |
|---------------|--------|
| All `FontSpec { size: Some(X), .. }` constructors | `Some(FontSize::Pt(X))` or `Some(FontSize::Px(X))` depending on context |
| `kde/fonts.rs`, `gnome/mod.rs`, `windows.rs` test assertions | `Some(10.0)` -> `Some(FontSize::Pt(10.0))` |
| `resolve/mod.rs` Phase 1.5 tests | Rewrite: verify `validate()` converts `Pt` -> px correctly |
| `presets.rs` preset-loading tests | Unchanged — they call `into_resolved()` which produces `f32` |

### 2.11 Execution order

1. **Add `FontSize` enum** (font.rs) — compiles independently.

2. **Update `FontSpec` and `TextScaleEntry`** (font.rs) — change
   `size: Option<f32>` to `size: Option<FontSize>`, add proxy structs,
   update `FIELD_NAMES`. This breaks compilation everywhere.

3. **Fix OS readers** (kde/fonts.rs, gnome/mod.rs, windows.rs) — wrap
   parsed sizes in `FontSize::Pt(...)`.

4. **Fix resolution** (resolve/inheritance.rs, resolve/mod.rs) — delete
   `convert_pt_to_px()` and Phase 1.5, update text scale line_height to
   use `font_size.raw()`.

5. **Fix validation** (resolve/validate.rs) — add `dpi` parameter to
   `require_font` and `require_text_scale_entry`, convert via `to_px(dpi)`.

6. **Update TOML presets** (20 files) — `size` -> `size_pt` or `size_px`.

7. **Fix tests** — update constructors, assertions, add new tests.

8. **Update docs** (property-registry.toml).

### 2.12 Open questions

1. **Should `text_scale.*.line_height` also become `FontSize`?**
   Yes — see Chapter 3 §3.4. It shares the unit of its sibling `size`,
   so `line_height_pt` / `line_height_px` in TOML, `Option<FontSize>` in Rust.

2. **Should widget geometry fields get explicit `_px` suffixes?**
   Yes — see Chapter 3. Every dimensional field gets an explicit unit suffix
   (`_px` for always-pixel fields, `_pt`/`_px` for dual-unit fields).
   Ratios and multipliers are exempt.

---

## Chapter 3 — Explicit units for all dimensional fields

Chapter 2 added `_pt`/`_px` suffixes to font sizes. This chapter extends
the same principle to **every** dimensional field: widget geometry, icon
sizes, border properties, spacing, and `text_scale.*.line_height`.

### 3.1 Rationale

After Chapter 2, the codebase has two conventions:

- Font sizes: `size_pt = 10.0` or `size_px = 14.0` — unit is explicit
- Everything else: `min_width = 64.0`, `corner_radius = 5.0` — unit is implicit

A reader must *know* that widget geometry is always in logical pixels.
Adding `_px` suffixes makes every numeric dimension self-documenting.
There is no ambiguity today, but there is no *signal* either — the suffix
removes the need to remember the convention.

### 3.2 Field categories

#### 3.2.1 Always-pixel fields → `_px` suffix

These fields are always in logical pixels. They get a `_px` suffix in TOML.
The Rust struct field name stays unchanged; serde mapping uses
`#[serde(rename = "field_name_px")]`.

**Border sub-struct (`BorderSpec`):**

| Current key | New key |
|-------------|---------|
| `corner_radius` | `corner_radius_px` |
| `corner_radius_lg` | `corner_radius_lg_px` |
| `line_width` | `line_width_px` |
| `padding_horizontal` | `padding_horizontal_px` |
| `padding_vertical` | `padding_vertical_px` |

**Icon sizes (`IconSizes`):**

| Current key | New key |
|-------------|---------|
| `toolbar` | `toolbar_px` |
| `small` | `small_px` |
| `large` | `large_px` |
| `dialog` | `dialog_px` |
| `panel` | `panel_px` |

**Defaults (`ThemeDefaults`):**

| Current key | New key |
|-------------|---------|
| `focus_ring_width` | `focus_ring_width_px` |
| `focus_ring_offset` | `focus_ring_offset_px` |

**Button:**

| Current key | New key |
|-------------|---------|
| `min_width` | `min_width_px` |
| `min_height` | `min_height_px` |
| `icon_text_gap` | `icon_text_gap_px` |

**Input:**

| Current key | New key |
|-------------|---------|
| `min_height` | `min_height_px` |

**Checkbox:**

| Current key | New key |
|-------------|---------|
| `indicator_width` | `indicator_width_px` |
| `label_gap` | `label_gap_px` |

**Menu:**

| Current key | New key |
|-------------|---------|
| `row_height` | `row_height_px` |
| `icon_text_gap` | `icon_text_gap_px` |
| `icon_size` | `icon_size_px` |

**Tooltip:**

| Current key | New key |
|-------------|---------|
| `max_width` | `max_width_px` |

**Scrollbar:**

| Current key | New key |
|-------------|---------|
| `groove_width` | `groove_width_px` |
| `min_thumb_length` | `min_thumb_length_px` |
| `thumb_width` | `thumb_width_px` |

**Slider:**

| Current key | New key |
|-------------|---------|
| `track_height` | `track_height_px` |
| `thumb_diameter` | `thumb_diameter_px` |
| `tick_mark_length` | `tick_mark_length_px` |

**Progress bar:**

| Current key | New key |
|-------------|---------|
| `track_height` | `track_height_px` |
| `min_width` | `min_width_px` |

**Tab:**

| Current key | New key |
|-------------|---------|
| `min_width` | `min_width_px` |
| `min_height` | `min_height_px` |

**Toolbar:**

| Current key | New key |
|-------------|---------|
| `bar_height` | `bar_height_px` |
| `item_gap` | `item_gap_px` |
| `icon_size` | `icon_size_px` |

**List:**

| Current key | New key |
|-------------|---------|
| `row_height` | `row_height_px` |

**Splitter:**

| Current key | New key |
|-------------|---------|
| `divider_width` | `divider_width_px` |

**Separator:**

| Current key | New key |
|-------------|---------|
| `line_width` | `line_width_px` |

**Layout:**

| Current key | New key |
|-------------|---------|
| `widget_gap` | `widget_gap_px` |
| `container_margin` | `container_margin_px` |
| `window_margin` | `window_margin_px` |
| `section_gap` | `section_gap_px` |

**Switch:**

| Current key | New key |
|-------------|---------|
| `track_width` | `track_width_px` |
| `track_height` | `track_height_px` |
| `thumb_diameter` | `thumb_diameter_px` |
| `track_radius` | `track_radius_px` |

**Dialog:**

| Current key | New key |
|-------------|---------|
| `min_width` | `min_width_px` |
| `max_width` | `max_width_px` |
| `min_height` | `min_height_px` |
| `max_height` | `max_height_px` |
| `button_gap` | `button_gap_px` |
| `icon_size` | `icon_size_px` |

**Spinner:**

| Current key | New key |
|-------------|---------|
| `diameter` | `diameter_px` |
| `min_diameter` | `min_diameter_px` |
| `stroke_width` | `stroke_width_px` |

**Combo box:**

| Current key | New key |
|-------------|---------|
| `min_height` | `min_height_px` |
| `min_width` | `min_width_px` |
| `arrow_icon_size` | `arrow_icon_size_px` |
| `arrow_area_width` | `arrow_area_width_px` |

**Segmented control:**

| Current key | New key |
|-------------|---------|
| `segment_height` | `segment_height_px` |
| `separator_width` | `separator_width_px` |

**Expander:**

| Current key | New key |
|-------------|---------|
| `header_height` | `header_height_px` |
| `arrow_icon_size` | `arrow_icon_size_px` |

**Total: 63 always-pixel field renames.**

#### 3.2.2 Dual-unit fields → `_pt` / `_px` suffix

These fields share the unit of their sibling font size:

| Struct | Current key | New keys |
|--------|-------------|----------|
| `TextScaleEntry` | `line_height` | `line_height_pt` / `line_height_px` |

Platform presets use `line_height_pt` (alongside `size_pt`).
Community presets use `line_height_px` (alongside `size_px`).

Implementation: change `TextScaleEntry.line_height` from `Option<f32>` to
`Option<FontSize>`. The proxy struct (`TextScaleEntryRaw`) gains
`line_height_pt: Option<f32>` and `line_height_px: Option<f32>`,
same pattern as `size_pt`/`size_px`.

The `TryFrom<TextScaleEntryRaw>` enforces consistency: if `size_pt` is
set, `line_height_pt` must be used (not `line_height_px`), and vice versa.
Both None is valid (inherit from defaults).

The resolve-time computation also changes:

```rust
// Before (Chapter 2):
entry.line_height = Some(lh_mult * font_size.raw());

// After (Chapter 3):
entry.line_height = Some(match font_size {
    FontSize::Pt(_) => FontSize::Pt(lh_mult * font_size.raw()),
    FontSize::Px(_) => FontSize::Px(lh_mult * font_size.raw()),
});
```

And validation simplifies — `line_height` is now `FontSize`, so the
existing `FontSize::to_px(dpi)` handles conversion. The special-case
`needs_pt_conversion` logic in `require_text_scale_entry` is deleted:

```rust
let line_height = e.line_height
    .map(|fs| fs.to_px(dpi))
    .unwrap_or_else(|| { missing.push(format!("{prefix}.line_height")); 0.0 });
```

#### 3.2.3 Exempt fields — unitless ratios and multipliers (no suffix)

| Field | Type | Why exempt |
|-------|------|------------|
| `defaults.line_height` | multiplier | Ratio of font size (e.g. 1.36) |
| `defaults.disabled_opacity` | ratio 0–1 | Alpha multiplier |
| `defaults.text_scaling_factor` | multiplier | Accessibility scale (1.0 = normal) |
| `border.opacity` | ratio 0–1 | Alpha multiplier |
| `button.disabled_opacity` | ratio 0–1 | Alpha multiplier |
| `input.disabled_opacity` | ratio 0–1 | Alpha multiplier |
| `checkbox.disabled_opacity` | ratio 0–1 | Alpha multiplier |
| `slider.disabled_opacity` | ratio 0–1 | Alpha multiplier |
| `switch.disabled_opacity` | ratio 0–1 | Alpha multiplier |
| `combo_box.disabled_opacity` | ratio 0–1 | Alpha multiplier |
| `segmented_control.disabled_opacity` | ratio 0–1 | Alpha multiplier |

These are dimensionless — they have no unit to suffix.

### 3.3 Implementation approach

#### 3.3.1 Always-pixel fields: `#[serde(rename)]`

For fields that are always in logical pixels, the Rust struct field name
stays unchanged. The TOML key gets a `_px` suffix via serde rename:

```rust
pub struct ButtonTheme {
    // ...
    #[serde(rename = "min_width_px")]
    pub min_width: Option<f32>,
    #[serde(rename = "min_height_px")]
    pub min_height: Option<f32>,
    #[serde(rename = "icon_text_gap_px")]
    pub icon_text_gap: Option<f32>,
    // ...
}
```

This preserves the Rust API (`button.min_width`) while making TOML
self-documenting (`min_width_px = 64.0`).

`FIELD_NAMES` arrays are updated to use the TOML names (with `_px`
suffix) since they drive TOML linting.

The same pattern applies to `BorderSpec`, `IconSizes`, and all widget
structs with dimensional fields.

#### 3.3.2 `TextScaleEntry.line_height`: FontSize

See §3.2.2. Change `line_height: Option<f32>` to
`line_height: Option<FontSize>`. The proxy struct handles mapping to
`line_height_pt`/`line_height_px`.

#### 3.3.3 Resolved types: unchanged

`ResolvedButtonTheme`, `ResolvedBorderSpec`, etc. keep their `f32` field
names without suffix — resolved values are always in logical pixels by
definition.

### 3.4 TOML examples

**Before (current):**

```toml
[light.defaults.border]
corner_radius = 5.0
corner_radius_lg = 8.0
line_width = 1.0

[light.defaults.icon_sizes]
toolbar = 22.0
small = 16.0

[light.button]
min_width = 64.0
min_height = 32.0
icon_text_gap = 8.0

[light.text_scale.caption]
size_pt = 8.2
weight = 400
line_height = 11.2
```

**After:**

```toml
[light.defaults.border]
corner_radius_px = 5.0
corner_radius_lg_px = 8.0
line_width_px = 1.0

[light.defaults.icon_sizes]
toolbar_px = 22.0
small_px = 16.0

[light.button]
min_width_px = 64.0
min_height_px = 32.0
icon_text_gap_px = 8.0

[light.text_scale.caption]
size_pt = 8.2
weight = 400
line_height_pt = 11.2
```

### 3.5 Execution order

1. **Update `TextScaleEntry`** — change `line_height` from `Option<f32>`
   to `Option<FontSize>`, update proxy struct with `line_height_pt`/
   `line_height_px`, update resolve computation, simplify validate.

2. **Add `#[serde(rename)]`** to all always-pixel fields across all model
   structs (`BorderSpec`, `IconSizes`, `ButtonTheme`, `InputTheme`, ...).
   Update `FIELD_NAMES` arrays.

3. **Rename TOML preset keys** — mechanical rename across all 20 presets.
   Estimated ~3800 renames (63 fields × ~60 occurrences average across
   20 presets × 2 variants).

4. **Update `property-registry.toml`** — all field names get `_px` suffix.

5. **Fix tests** — update constructors, assertions, TOML string literals.

6. **Run `pre-release-check.sh`**.
