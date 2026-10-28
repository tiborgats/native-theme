# v0.7.1 — Model completeness: Specification

**Status:** draft, 2026-10-28; in progress (§0). This document states the model the rationale
([`todo_v0.7.1_model-completeness-rationale.md`](todo_v0.7.1_model-completeness-rationale.md), "R…")
decides: its types, values, rules, order, API, file formats and checks. It is in three parts:

- this core: the mechanisms every widget uses (S1–S22);
- the widget catalogue, [`todo_v0.7.1_model-completeness-spec-widgets.md`](todo_v0.7.1_model-completeness-spec-widgets.md):
  every typed widget and keyed component, with its parts, widget values, variants, sizes and flags;
- the traceability, `todo_v0.7.1_model-completeness-spec-trace-<platform>.md`: every row of the gap map
  mapped to what carries it (R24).

Rust in this document is a sketch of the public shape: names, fields and variants are normative; derives,
visibility of internals and method signatures beyond those stated are the implementation's. No value of any
platform is stated here: values come from the facts (S17) in the value phase.

## 0. Progress

| Part | State |
|---|---|
| Core (this document) | drafted |
| Research the rationale requires first: kirigami-addons survey (R1.2), community ports (R19), Plasma's accent derivation (R2.6) | done; R19 and R25 decided from it |
| Widget catalogue | drafted in eight groups, being merged |
| Traceability | drafted in eight groups, being merged; `scripts/check_spec_trace.py` checks it |
| Check passes | not started |

## S1. Conventions

- **Names.** Rust types are `UpperCamelCase`, fields `snake_case`; TOML keys equal the Rust field names, with
  the unit suffixes the model uses today (`_px`, `_ms`, `_pt`) where a field is a plain number.
- **Units.** Lengths are logical pixels (`f32`); durations milliseconds (`f32`); angles degrees; opacities and
  fractions `0.0..=1.0`; colours extended sRGB (S2.1).
- **Keys.** A keyed name (a platform's own colour role, variant, component, text style, elevation entry,
  material, presentation kind) is a `Key`: a lower-case identifier, `[a-z][a-z0-9_]*`, declared by the preset
  (S10). A typed name is an enum variant.
- **`#[non_exhaustive]`** (R21): every struct below has it; the enums of S21.2 do not.

## S2. Blocks (R2.1)

### S2.1 Colour

```rust
/// Extended sRGB, straight (not premultiplied) alpha. Channels may lie outside 0..=1 (Display P3 colours
/// expressed in extended sRGB); alpha is 0..=1.
pub struct Color { pub r: f32, pub g: f32, pub b: f32, pub a: f32 }
```

It replaces `Rgba` everywhere. Conversion helpers: `to_srgb8_clamped()` for connectors that render sRGB, and
`to_display_p3()`. TOML: `"#rrggbb"`, `"#rrggbbaa"` (in gamut), or `"srgb(r, g, b[, a])"` with floats.

### S2.2 Values and expressions (R5.4, R20)

Every field of every block that the platforms can derive or omit is a `Value<T>`:

```rust
pub enum Value<T> {
    /// A stated value.
    Stated(T),
    /// A derivation, evaluated where used (R2.5).
    Expr(Expr),
    /// The platform sizes by content (lengths only; R20).
    Auto,
    /// The platform does not draw this; resolves to none. Allowed only where the field is marked
    /// `may_be_absent` (S6.3).
    Absent,
}
```

A field with no value in any layer is **missing** and fails resolution (S12).

**Expression grammar** (TOML: a string starting with `=`; whitespace insignificant):

```text
expr     := literal | ref | call
literal  := colour literal | number | duration ("150ms")
ref      := "role." set "." role        typed colour role of a surface set           (S3)
          | "role." global                typed global role (accent, status, …)        (S3)
          | "role.@" key                  keyed platform role                          (S3)
          | "surface." role               role of the surface bound in scope           (S3.3)
          | "active(" ref ")"             a role's active value                        (R5.1)
          | "token." path                 a token                                      (S4)
          | "input." name                 an input                                     (S9)
          | "base"                        the value earlier rules gave in this query   (R2.5)
          | "foreground"                  the element's current foreground             (R5.4)
          | "font_size" | "line_height"   the part's, after text scaling               (R10)
          | "element." data               the element's data (S8): width, height, progress,
                                          scroll_offset, scroll_progress, item_index,
                                          item_count, text, color, char_width,
                                          display_corner_radius                        (R25.3)
          | "part." field                 another resolved field of the same part      (R25.3)
          | "text_scale"                  the text-scaling factor (S9.4)               (ADD-A55)
call     := name "(" expr { "," expr } ")"
```

**Functions** (a closed set, R5.4; each implemented to the source named, pinned in the facts' sources, S17):

| Function | Result | Definition |
|---|---|---|
| `alpha(c, a)` | colour | `c` with its alpha multiplied by `a` |
| `mix(a, b, t)` | colour | interpolation from `a` (t = 0) to `b` (t = 1) in sRGB with premultiplied alpha, as CSS `color-mix(in srgb, a (1−t)·100%, b t·100%)` |
| `kde_intensity(c, effect, amount)` | colour | KColorScheme's intensity effect (shade, darken, lighten), to `kcolorscheme.cpp` |
| `kde_color(c, effect, amount, effect_colour)` | colour | KColorScheme's colour effect (desaturate, fade, tint) |
| `kde_contrast(c, background, effect, amount)` | colour | KColorScheme's contrast effect (fade, tint) |
| `kcu_mix(a, b, bias)`, `kcu_tint(a, b, amount)`, `kcu_shade(c, luma, chroma)` | colour | `KColorUtils`, to `kcolorutils.cpp` |
| `adw_standalone(c, scheme)` | colour | libadwaita's standalone accent derivation in OKLab, to its stylesheet |
| `m3_role(role, seed, variant, version, contrast, scheme)` | colour | material-color-utilities' dynamic scheme, the role of that scheme |
| `mul(x, y)` | length or duration | product (a token times the speed factor; a font size times a factor) |
| `add(x, y)`, `sub(x, y)`, `min(x, y)`, `max(x, y)`, `floor(x)` | number, length, duration or font size | arithmetic; `mul` and `add` also over font sizes (ADD-A56) |
| `qt_lighter(c, factor)`, `qt_darker(c, factor)` | colour | `QColor::lighter`, `QColor::darker` in HSV, to qtbase `qcolor.cpp` (ADD-A50) |
| `kcu_overlay(base, paint)` | colour | `KColorUtils::overlayColors`, to `kcolorutils.cpp` (ADD-A51) |
| `kirigami_tint(c, tint, alpha)` | colour | Kirigami's `ColorUtils.tintWithAlpha`; QML's `Qt.tint` is `qt_tint(base, tint)`, added only if the value phase finds it differs from `mix` over Kirigami's inputs (ADD-A52) |
| `kirigami_label_icon_size(line_height)`, `kirigami_badge_text(text, background)` | length, colour | Kirigami's `units.cpp` and `Badge.qml` (ADD-A53) |
| `ka_card_border(background, text)`, `ka_banner_line(type_text, frame)`, `ka_contrast_text(c)` | colour | kirigami-addons' FormCard background, banner and avatar code (ADD-A66) |
| `kqc_series_colour(seed, count, index)`, `kte_minimap_mark(mark, background)` | colour | KQuickCharts' `ColorGradientSource.cpp`, KTextEditor's `kateviewhelpers.cpp` (ADD-A57) |
| `sf_tracking(font_size, design)` | length | Apple's size-to-tracking tables for SF Pro, SF Pro Rounded, New York (ADD-A54) |
| `uifm_scaled(x, text_role)` | length | `UIFontMetrics.scaledValue(for:)` (ADD-A55) |
| `kde_accent(set, key)` | colour | Plasma's accent derivation for one key of one colour set, from `input.accent` and the scheme inputs `input.kde_scheme.<set>.<key>`, to `plasma-workspace` `kcms/colors/colorsapplicator.cpp` `applyScheme()` (R25.1) |
| `name_index(text, count, method)` | number | the index a platform's avatar hashes a name to; `method` keyed per platform (Kirigami's `nameutils.cpp`, libadwaita's `adw-avatar.c`), each to its source (R25.3) |
| `pick(index, c0, c1, …)` | colour | the colour at `index` (R25.3) |

A function is added to this table only with the platform rule that needs it and its source (R5.4). Type
checking: every operand and result is typed (colour, length, duration, number, text); a mismatch is a resolution
error (S12). An expression naming `element.<data>` is re-evaluated by each query, so a connector queries again when the
data changes, as for a state (R25.3).

### S2.3 Paint (R6)

```rust
pub struct Paint { pub layers: Vec<PaintLayer> }             // bottom first; empty = nothing
pub struct PaintLayer { pub kind: PaintKind, pub blend: BlendMode }
pub enum PaintKind {
    Color(Value<Color>),
    Linear { angle_deg: f32, stops: Vec<Stop>, repeat: bool,
             extent: Option<Value<f32>> },    // stops over this many px from the start edge (ADD-A31); else the box
    Radial { center: (f32, f32), radius: f32, stops: Vec<Stop>, repeat: bool },   // relative to the part's box
    Conic { center: (f32, f32), start_angle_deg: f32, stops: Vec<Stop> },          // R25.4
    Pattern { pattern: Pattern, colors: Vec<Value<Color>>, cell: Value<f32> },  // R25.4
    Material(Material),
}
pub enum Pattern { Checkerboard, Grid, QtBrush(QtBrushStyle) }   // each to its platform's source (R25.4)
pub enum QtBrushStyle { Dense1, Dense2, Dense3, Dense4, Dense5, Dense6, Dense7, Hor, Ver, Cross, BDiag, FDiag,
                        DiagCross }                               // Qt's `Qt::BrushStyle` patterns
pub struct Stop { pub position: f32, pub color: Value<Color> }
pub enum BlendMode { Normal, Keyed(Key) }    // platform blending modes as the facts name them (macOS vibrancy)
pub struct Material {
    pub identity: Key,                        // the platform's material (S10 declares the set)
    pub blur_radius: Value<f32>,
    pub tint: Value<Color>,
    pub saturation: Value<f32>,
    pub luminosity: Value<f32>,
    pub noise_opacity: Value<f32>,
    pub fallback: Value<Color>,               // what a toolkit that cannot draw it draws (R6)
    pub params: Vec<(Key, Value<f32>)>,       // what a platform's material states beyond these (ADD-A47)
}
```

A rule's patch either sets a paint (`fill = …`) or appends layers over `base` (`fill.append = [ … ]`) (R6). A
repeating gradient repeats its stops beyond the last (stripes, K1.110).

### S2.4 Corners, stroke (R7)

```rust
pub struct Corners { pub top_left: Radius, pub top_right: Radius, pub bottom_right: Radius,
                     pub bottom_left: Radius, pub curve: CornerCurve }
pub enum Radius { Px(Value<f32>), Full, Concentric { minimum: Value<f32> } }
pub enum CornerCurve { Circular, Continuous }
pub struct Stroke { pub top: Edge, pub right: Edge, pub bottom: Edge, pub left: Edge,
                    pub position: StrokePosition }
pub struct Edge { pub width: Value<f32>, pub paint: Paint, pub dash: Value<DashPattern>,   // ADD-A32
                  pub snap: PixelSnap }                                                    // ADD-A33
pub enum DashPattern { Solid, Dotted, Dashed { on: f32, off: f32 } }
pub enum PixelSnap { None, ToDevicePixel, AtLeastOneDevicePixel }   // the width stays logical
pub enum LineCap { Butt, Round, Square }
pub enum LineJoin { Miter, Round, Bevel }
pub enum StrokePosition { Inside, Center, Outside }
```

A part's outline is its `corners`, unless `PartStyle.shape` names a keyed shape of the platform's shape library
(declared in S10, each defined to its source, R25.5); a shape change is a transition of the `Shape` group, which
morphs.

TOML shorthands: `corners = 6.0` sets all four `Px` with `Circular`; `stroke = { width = 1, paint = "…" }` sets
all four edges.

### S2.5 Shadow (R8)

```rust
pub enum Shadow { Layers(Vec<ShadowLayer>), Elevation(Key) }   // Key: an entry of the elevation ramp (S4)
pub struct ShadowLayer { pub x: Value<f32>, pub y: Value<f32>, pub blur: Value<f32>,
                         pub spread: Value<f32>, pub color: Value<Color>, pub inset: bool }
```

`Layers(vec![])` is no shadow.

### S2.6 Transform (R9)

```rust
pub struct Transform { pub scale_x: Value<f32>, pub scale_y: Value<f32>, pub rotate_deg: Value<f32>,
                       pub translate_x: Value<f32>, pub translate_y: Value<f32>,
                       pub origin: (f32, f32) }                // fractions of the part's box
```

### S2.7 Transitions and timing (R9)

```rust
pub enum Timing {
    CubicBezier(f32, f32, f32, f32),
    Linear,
    Steps { count: u32, jump: StepJump },
    Spring { mass: f32, stiffness: f32, damping: f32,
             initial_velocity: f32, rest_epsilon: f32, clamp: bool },
    Keyed(Key),                               // a platform's own, defined in its facts (velocity-based curves, R25.6)
}
pub enum PropertyGroup { Fill, Stroke, Corners, Shape, Shadow, Opacity, Transform, Text, Icon, Glyph, Focus,
                         Geometry }
pub struct Transition {
    pub properties: Vec<PropertyGroup>,
    pub kind: TransitionKind,
    pub duration: Value<f32>,                 // ms
    pub delay: Value<f32>,
    pub timing: Timing,
}
pub enum TransitionKind {
    Interpolate,
    Ripple { max_radius: Value<f32>, opacity: Value<f32>, fade_ms: Value<f32> },   // MAT-X9
    FocusMove,                                // the indicator travelling between elements (ADD-A19)
    Keyed(Key),                               // SF Symbols' content transitions and effects, WinUI's AnimatedIcon
}                                             // states; their values measured (ADD-A27, R25.12)
pub struct PresentationTransition {          // one event's presentation: its steps run together (R25.6)
    pub event: PresentationEvent,             // Appear, Disappear, Move, and widget-specific keyed events
    pub steps: Vec<PresentationStep>,
}
pub struct PresentationStep {
    pub part: Option<PartRef>,                // None: the whole element
    pub kind: PresentationKind,
    pub duration: Value<f32>, pub delay: Value<f32>, pub timing: Timing,
    pub stagger: Value<f32>,                  // ms between items, where the element presents items
}
pub enum PresentationKind { Fade, Slide { from: EdgeSide, distance: Value<f32> }, Translate { dx: Value<f32>,
                            dy: Value<f32> }, Scale { from: f32 }, Morph, ClipReveal { from: RevealOrigin },
                            Keyed { key: Key, params: Vec<(Key, Value<f32>)> }, None }   // ADD-A25, ADD-A26
pub enum EdgeSide { Top, Bottom, Start, End, TowardAnchor }   // TowardAnchor: WinUI's menus (ADD-A25)
pub struct Loop { pub period_ms: Value<f32>, pub timing: Timing, pub keyframes: Vec<Keyframe> }
pub struct Keyframe { pub at: f32, pub patch: Patch }   // `at` a fraction of the period; a patch of the part's
                                                        // style, a keyed shape included (R25.5)
```

Semantics (R9): a transition starts when a property's resolved value changes; the transition stated by the style
after the change governs, both directions; a later rule's transition for the same property group wins; appearing
layers interpolate from zero alpha, layers match by stack position, colours interpolate in premultiplied sRGB;
a material giving way to a colour crossfades. During a theme change (S4.3) the theme-change token governs.

### S2.8 Text style and font (R10)

```rust
pub struct Font {
    pub family: Value<String>,                // fallbacks are the text tokens' (S4.4, ADD-A41)
    pub size: Value<FontSize>,                // today's FontSize (pt or px)
    pub weight: Value<u16>,
    pub style: Value<FontStyle>,
    pub width: Value<f32>,                    // percent of normal, as the `wdth` axis
    pub axes: Vec<(AxisTag, AxisValue)>,
    pub features: Vec<(FeatureTag, u32)>,
}
pub enum AxisValue { Fixed(f32), FollowsSize }  // SF's and Segoe UI Variable's optical size
pub struct TextStyle {
    pub font: Font,
    pub letter_spacing: Value<f32>,
    pub line_height: Value<FontSize>,         // pt or px (scaled with the text-scaling factor), or an Expr on
                                              // font_size (ADD-A40)
    pub decoration: Option<Decoration>,
    pub mnemonic_decoration: Option<Decoration>,   // the mnemonic's underline (ADD-A12)
    pub case: TextCase,                       // ADD-A39
    pub elision: Elision,
    pub wrap: Wrap,
    pub max_lines: Option<u32>,               // wrap to at most n lines, then elide (ADD-A67)
    pub fill: Paint,
}
pub enum TextCase { Mixed, Upper, Lower, SmallCaps, Capitalize }
pub struct Decoration { pub line: DecorationLine, pub style: DecorationStyle,
                        pub thickness: Value<f32>, pub offset: Value<f32>, pub paint: Paint }
pub enum DecorationLine { Underline, Overline, Strike }
pub enum DecorationStyle { Solid, Double, Dotted, Dashed, Wavy }
pub enum Elision { None, End, Middle, Start,
                   Fade { length: Value<f32>, relative: bool, under: Option<PartName> } }  // relative: a fraction
                                                                                         // of the box (ADD-A38)
pub enum Wrap { None, Word, Char }
```

The text ramp (S4.4) holds the semantic `TextRole` slots and the keyed platform styles.

### S2.9 Icon style and glyph (R11)

```rust
pub struct IconStyle { pub size: Value<f32>, pub fill: Paint, pub rendering_mode: RenderingMode,
                       pub weight: Value<u16>, pub scale: Value<f32>, pub effect: IconEffect,
                       pub symbol_variants: Vec<Key>,      // SF Symbols' `.fill`, `.circle`, … (R25.8)
                       pub palette: Vec<(Key, Paint)>,     // a symbolic icon's success, warning, error colours (ADD-A37)
                       pub animation: Option<Loop> }       // a repeating icon animation (ADD-A27, R25.12)
pub enum RenderingMode { Template, Monochrome, Hierarchical, Palette, Multicolor, Original }
pub enum IconEffect { None, Disabled { kind: Key }, Active { kind: Key } }   // KDE's effects, keyed (K3.60)
pub struct Glyph { pub source: GlyphSource, pub size: Value<f32>, pub stroke_width: Value<f32>,
                   pub fill: Paint, pub line_cap: LineCap, pub line_join: LineJoin,      // ADD-A34
                   pub draw_extent: Value<f32>,      // the fraction of the path drawn (Breeze's check mark, ADD-A35)
                   pub mirrors_in_rtl: bool }        // Segoe Fluent's per-glyph mirroring (R25.8)
pub enum GlyphSource {
    FontCodePoint { family: String, code_point: u32 },
    ThemeIcon { name: String, fallbacks: Vec<String> },   // ADD-A36
    Path { data: String, source: Citation },            // SVG path syntax, from the platform's source code
    NamedSystemImage { name: String },                   // a name the platform states for that purpose
    SystemControl(SystemControl),                        // the system's rendering of the whole control (S15)
}
```

`IconRole` gains `mirrors_in_rtl` per role, stated per preset (R11). Icon lookup takes the element's direction and
language (GTK's `-rtl` and `-ltr` names, SF Symbols' localized forms; R25.8).

### S2.10 Focus indicator (R12)

```rust
pub struct FocusIndicator { pub kind: FocusKind, pub width: Value<f32>, pub paint: Paint,
                            pub offset: Sides<Value<f32>>,                 // per side (ADD-A18)
                            pub corners: Option<Corners>,                  // None: concentric with the part
                            pub secondary: Option<FocusRing>,              // the inner ring (ADD-A17)
                            pub shadow: Shadow, pub transition: Option<Transition> }
pub struct FocusRing { pub width: Value<f32>, pub paint: Paint, pub offset: Sides<Value<f32>> }
pub enum FocusVisibleWhen { Keyboard { timeout_ms: Value<f32> }, AnyFocus, FocusWithin, Never }   // ADD-A16
pub enum FocusKind { Ring, FilledRing, Underline, None }
```

### S2.11 Part style

```rust
pub struct PartStyle {
    pub fill: Paint,
    pub stroke: Stroke,
    pub corners: Corners,
    pub shape: Option<Key>,                  // a keyed shape replacing the corners' outline (R25.5)
    pub shadow: Shadow,
    pub opacity: Value<f32>,                 // applied to the element and its descendants as a group
    pub transform: Transform,                // likewise
    pub text: Option<TextStyle>,
    pub icon: Option<IconStyle>,
    pub glyph: Option<Glyph>,
    pub focus: Option<FocusIndicator>,
    pub padding: Sides<Value<f32>>,
    pub min_width: Value<f32>, pub min_height: Value<f32>,
    pub max_width: Value<f32>, pub max_height: Value<f32>,
    pub cursor: Value<CursorShape>,
    pub bindings: Vec<Binding>,
    pub transitions: Vec<Transition>,
}
pub enum Binding { Surface(SurfaceSet), Role { role: RoleRef, value: Value<Color> } }
```

A part style that does not draw a property states it `Absent` where the model allows (S6.3). `CursorShape` gains
`Crosshair` and `WhatsThis` (ADD-A59). Every widget's "when focus becomes visible" value is a
`focus_visible_when: FocusVisibleWhen`, its default `token.behaviour.focus_visible_when` (ADD-A16).

## S3. Colour roles (R5.1–R5.3)

```rust
pub enum SurfaceSet { Window, View, Button, Header, Sidebar, SecondarySidebar, Card, Popover, Dialog, Tooltip,
                      Selection, Complementary, Osd }         // a preset declares the sets it has (ADD-A48)
pub struct SurfaceRoles {                                      // the common role record, per surface set
    pub background: Value<Color>, pub background_alternate: Value<Color>,
    pub foreground: Value<Color>, pub foreground_inactive: Value<Color>, pub foreground_active: Value<Color>,
    pub link: Value<Color>, pub link_visited: Value<Color>,
    pub negative: Value<Color>, pub neutral: Value<Color>, pub positive: Value<Color>,
    pub decoration_focus: Value<Color>, pub decoration_hover: Value<Color>,
    pub shade: Value<Color>, pub shade_darker: Value<Color>, pub border: Value<Color>,
    pub background_active: Value<Color>, pub background_link: Value<Color>,          // KDE's computed status
    pub background_visited: Value<Color>, pub background_negative: Value<Color>,      // backgrounds (ADD-A48)
    pub background_neutral: Value<Color>, pub background_positive: Value<Color>,
    pub disabled: DisabledRoles,                               // per element; parts' {disabled} rules refer to it
}
pub struct GlobalRoles {
    pub accent: Value<Color>, pub accent_foreground: Value<Color>, pub accent_standalone: Value<Color>,
    pub accent_ramp: Vec<(Key, Value<Color>)>,                // WinUI's Light1–3 / Dark1–3, keyed
    pub status: StatusRoles,                                   // success, warning, error, info, neutral,
                                                               // destructive: fill, foreground, standalone,
                                                               // background, background_solid each (ADD-A48)
    pub shadow_reference: Value<Color>,                        // what the platform names a shadow colour (MAC)
    pub selection_text: Value<Color>,                          // may be absent (ADD-A48)
    pub selection_text_foreground: Value<Color>,
    pub selection_background_inactive: Value<Color>,           // an unfocused field's selection (ADD-A49)
    pub find_highlight: Value<Color>, pub drop_highlight: Value<Color>,
    pub scrim: Value<Color>,
}
pub struct ColorRoles {
    pub surfaces: SurfaceMap<SurfaceRoles>,
    pub global: GlobalRoles,
    pub keyed: Vec<(Key, Value<Color>)>,                       // Material's ladder, WinUI's tiers
}
```

The inactive window's selection is a rule under `WindowInactive`, not a role. KDE's 3-D shades (Qt's Light, Midlight,
Dark, Mid, Shadow) are keyed roles computed from Window (`role.@shade_light` …), as KDE computes them (ADD-A48).
WinUI's attention, success, caution and critical map to `info`, `success`, `warning`, `error`.

**S3.1 Bases.** `colors.light` and `colors.dark` are the two bases (R5.1). Every other condition changes roles
by colour-role rules (S7).

**S3.2 Groups.** The inactive group is a colour-role rule under `window_inactive`; disabled roles are
`SurfaceRoles.disabled`, which parts' `{disabled}` rules refer to (R5.1).

**S3.3 Bindings and scope.** `PartStyle.bindings` bind a surface set or a single role for the element and its
descendants; `surface.<role>` resolves against the nearest binding (itself or an ancestor), else the window's
root surface (`Window`). A binding's value is evaluated in the scope outside it (R5.2).

## S4. Tokens

```rust
pub struct Tokens {
    pub motion: MotionTokens,              // durations, timings (keyed), the speed factor
    pub theme_change: ThemeChange,
    pub elevation: Vec<(Key, Vec<ShadowLayer>)>,   // the platform's ramp, or empty (R8)
    pub text: TextTokens,
    pub icon_sizes: IconSizeTokens,        // S4.5
    pub focus: FocusIndicator,             // the platform's indicator (R12)
    pub caret: Caret,
    pub behaviour: Behaviour,
    pub size_classes: Vec<(SizeClass, SizeClassThreshold)>,   // where the platform states widths (R16)
    pub colour_effects: Vec<(Key, Value<f32>)>,              // KDE's effect parameters
    pub params: Vec<(Key, Vec<(Key, Value<f32>)>)>,          // keyed scalar groups (S4.6)
    pub pointer: PointerTokens,                              // S4.7
    pub material_scheme: Option<MaterialScheme>,             // Material's variant and specification version
}
pub struct TextRendering { pub engine: Value<Key>,       // Qt Quick's render type (Q3.32)
                           pub antialiasing: Value<Antialiasing>, pub hinting: Value<Hinting>,
                           pub subpixel_order: Value<SubpixelOrder>, pub lcd_filter: Value<Key>,
                           pub gamma: Value<f32>, pub contrast: Value<f32> }
pub enum Antialiasing { None, Grayscale, Subpixel }
pub enum Hinting { None, Slight, Medium, Full }
pub enum SubpixelOrder { None, Rgb, Bgr, Vrgb, Vbgr }
```

`TextRendering` is `token.text.rendering`. The readers read the text-rendering settings (ClearType's, fontconfig's and GNOME's `font-antialiasing`,
`font-hinting`, `font-rgba-order`, macOS's font smoothing); a toolkit applies what it can (R23).

### S4.1 Motion

`MotionTokens { durations: Vec<(Key, Value<f32>)>, timings: Vec<(Key, Timing)>, speed_factor: Value<f32> }`. A
scaled duration is stated as `= mul(token.motion.durations.x, token.motion.speed_factor)` (R9).

### S4.2 Caret and behaviour (R12, R17)

```rust
pub struct Caret { pub color: Value<Color>, pub width: Value<f32>, pub shape: CaretShape,
                   pub blink_on_ms: Value<f32>, pub blink_off_ms: Value<f32>, pub blink_timeout_ms: Value<f32>,
                   pub blink_style: BlinkStyle,                                         // ADD-A24
                   pub secondary: Value<Color>,          // the bidirectional secondary caret; may be absent
                   pub hides_while_selecting: bool, pub shown_in_read_only: Value<bool>,
                   pub overwrite_shape: CaretShape, pub block_text: Value<Color> }     // ADD-A23
pub enum BlinkStyle { Hard, Fade }
pub struct Behaviour { pub double_click_ms: Value<f32>, pub double_click_distance: Value<f32>,
                       pub drag_threshold: Value<f32>, pub hover_open_delay_ms: Value<f32>,
                       pub key_repeat_delay_ms: Value<f32>, pub key_repeat_interval_ms: Value<f32>,
                       pub scroll_lines_per_step: Value<f32>, pub hide_pointer_while_typing: Value<bool>,
                       pub scroll: ScrollPhysics,
                       pub long_press_ms: Value<f32>, pub spring_loading_ms: Value<f32>,       // ADD-A45
                       pub drag_time_ms: Value<f32>, pub single_click_activates: Value<bool>,
                       pub double_click_height: Value<f32>,      // with double_click_distance as the width
                       pub touch: TouchBehaviour,
                       pub mnemonics: Mnemonics,                 // ADD-A12
                       pub focus_visible_when: FocusVisibleWhen }
pub struct TouchBehaviour { pub slop: Value<f32>, pub paging_slop: Value<f32>, pub double_tap_ms: Value<f32>,
                            pub double_tap_slop: Value<f32>, pub delay_ms: Value<f32> }
pub enum Mnemonics { Never, OnAlt { delay_ms: Value<f32> }, Always }
pub struct ScrollPhysics { pub deceleration: Value<f32>, pub overscroll: Option<Timing>,
                           pub overscroll_effect: OverscrollEffect, pub overscroll_extent: Value<f32>,  // ADD-A29
                           pub snap: Value<bool> }
pub enum OverscrollEffect { None, RubberBand, Stretch, Glow }
```

The caret is also a part (`caret`) of the text widgets, whose base refers to `token.caret`, so rules apply to it by
widget, drop target and inactive window; the text widgets' further parts (preedit, misspelling, selection handles,
…) and the `@overwrite` and `@composing` flags are the catalogue's (ADD-A23, ADD-A24).

Widget-specific timings (tooltip delays, submenu delay and sloppy close, scroll-bar auto-hide and trough click,
toast duration, focus-visible rule) are widget values in the catalogue (R17).

### S4.3 Theme change (R9)

```rust
pub enum ThemeChange {
    Crossfade { duration: Value<f32>, timing: Timing },
    Suspend { duration: Value<f32>, reaches: Vec<PropertyGroup> },   // libadwaita: CSS transitions only
    Instant,
}
```

A theme change is any re-resolution not caused by a condition change alone (S8). A transition already running
when a suspension starts finishes.

### S4.4 Text ramp

```rust
pub enum TextRole { Body, BodyEmphasized, Caption, CaptionEmphasized, Footnote, Subheadline, Callout,
                    Headline, Title1, Title2, Title3, LargeTitle, DisplayLarge, DisplayMedium, DisplaySmall,
                    Label, Monospace, Numeric }        // the slot list is closed by the catalogue (R10)
pub struct TextTokens { pub roles: Vec<(TextRole, TextSlot)>, pub keyed: Vec<(Key, TextStyle)>,
                        pub fallbacks: FontFallbacks, pub rendering: TextRendering }
pub enum TextSlot { Style(TextStyle), Keyed(Key) }            // a slot may name a keyed style (ADD-A42)
pub struct FontFallbacks { pub families: Vec<String>, pub emoji_family: Value<String>,
                           pub per_language: Vec<(LanguageTag, Vec<String>)>,
                           pub per_family: Vec<(String, Vec<String>)> }   // Core Text's per-font cascade lists
```

`TextRole` gains `Caption2`; the slot list is closed at nineteen (ADD-A42). Font fallbacks are the platform's one
chain (per family where the platform keeps one per font), so they are a token and not a field of each font (ADD-A41).

### S4.5 Icon sizes

`IconSizeTokens { toolbar, small, large, dialog, panel: Value<f32>, keyed: Vec<(Key, IconStyle)> }`: the platform's
further contexts as keys, each an icon style (ADD-A43). The theme also states a fallback icon theme, used before the
system fallback (`KirigamiApp` makes Breeze the fallback of a Plasma application, ADD-A65).

### S4.6 Keyed scalars

`Tokens.params` holds keyed groups of scalars, referred to as `token.params.<group>.<key>` and declared per preset
(`params = [...]`, S10): `layout` (Kirigami's grid unit and spacings, Qt style metrics), `opacity` (disabled, dim,
…; restated by `Tokens` rules under `ContrastHigh`), `shadow`, `motion`, `colour` (KDE's contrast and frame
contrast), `caret`, `shape` (ADD-A44). KDE's colour-effect parameters stay `Tokens.colour_effects`.

### S4.7 Pointer

`PointerTokens { theme: Value<String>, size: Value<f32>, color: Value<Color>, drag_cursors: Vec<(Key, String)> }`,
read where the platform reports them (ADD-A46).

## S5. Widgets and parts

A widget is a struct generated from its catalogue entry (the catalogue):

```rust
pub struct ButtonTheme {               // example of the shape; the parts are the catalogue's
    pub body: PartStyle,
    pub label: PartStyle,
    pub icon: PartStyle,
    pub values: ButtonValues,           // gaps, alignment, behaviour, presentation transitions
}
```

**S5.1 Typed and keyed.** A kind two or more platforms draw is a typed widget struct; a one-platform kind is a
`Component { key: Key, parts: Vec<(Key, PartStyle)>, values: Vec<(Key, Value<WidgetValue>)> }` in the theme's
keyed component list, its parts declared by the preset (S10) (R14).

**S5.2 Window.** `WindowTheme` has the parts `frame`, `title_bar`, `title`, `caption_button` (one part type;
each caption button an element with a keyed identity: close, minimise, maximise, restore, help, …),
`window_menu` and `resize_zone`, and widget values for the caption buttons' order and side (R13).

## S6. The derive macro and the model's metadata

**S6.1** `#[derive(ThemeWidget)]` is replaced by `#[derive(ThemeParts)]`, which generates, per widget struct: its
patch type (every field `Option<…>`, paints with `Set`/`Append`), its resolved type (every `Value<T>` resolved to
`T` or `None` for absent), the field-path registry used by the TOML linter and the facts gate, and the
`may_be_absent` table.

**S6.2** Inheritance code goes: `inherit_from`, `theme_inherit`, the `[uniform]`, `[per_platform]` and
`[wrong_safety_nets]` registries and `resolve/inheritance.rs` are removed (R20). Their rules are converted once
into the presets' bases as expressions (S19.2).

**S6.3** `may_be_absent` is a per-field attribute (`#[theme(may_be_absent)]`), stated in the catalogue for each
field a platform may not draw; `Absent` elsewhere is a resolution error.

## S7. Rules and selectors (R2.3)

```rust
pub struct Rule { pub selector: Selector, pub target: Target, pub patch: Patch }
pub enum Target { Colors, Tokens, Part { widget: WidgetRef, part: PartRef }, Widget { widget: WidgetRef } }
pub enum WidgetRef { Typed(WidgetKind), Component(Key) }
pub struct Selector {
    pub conditions: SetMatch<Condition>,
    pub context: Vec<ContextMatch>,
    pub position: Option<PositionMatch>,
    pub variants: SetMatch<VariantRef>,
    pub size: Option<SizeMatch>,
    pub direction: Option<Direction>,
    pub states: SetMatch<StateFlag>,
}
pub struct SetMatch<T> { pub all_of: Vec<T>, pub none_of: Vec<T> }
pub struct ContextMatch { pub scope: ContextScope, pub widget: Option<WidgetRef>,   // None: any widget (R25.9)
                          pub part: Option<PartRef>,
                          pub variants: SetMatch<VariantRef>, pub states: SetMatch<StateFlag> }
pub enum ContextScope { AnyAncestor, NearestAmong(Vec<WidgetRef>) }
pub struct PositionMatch { pub among: SiblingScope,                 // which siblings count (R25.9)
                           pub first: Option<bool>, pub last: Option<bool>, pub only: Option<bool>,
                           pub parity: Option<Parity>,
                           pub previous: Option<SiblingMatch>, pub next: Option<SiblingMatch> }
pub enum SiblingScope { SameKind, Any, Kinds(Vec<WidgetRef>) }     // default SameKind
pub enum Parity { Odd, Even }                                       // counted from 1, among `among`
pub struct SiblingMatch { pub widget: Option<WidgetRef>, pub part: Option<PartRef>,
                          pub variants: SetMatch<VariantRef>, pub states: SetMatch<StateFlag> }
pub struct SizeMatch { pub size: Option<ControlSize>, pub density: Option<Density>,
                       pub size_class: Option<SizeClassMatch> }
pub struct SizeClassMatch { pub horizontal: Option<SizeClass>, pub vertical: Option<SizeClass> }
pub enum Direction { Ltr, Rtl }
```

A rule targeting `Colors` or `Tokens` may only have `conditions` (S12 rejects others).

### S7.1 Conditions

```rust
pub enum Condition {
    // system, by the reader (R2.3)
    Scheme(Scheme), ContrastIncreased, ContrastHigh, ReducedTransparency, ReducedMotion, AnimationsOff,
    BoldText, ButtonShapes, DifferentiateWithoutColour, TabletMode, MobileFormFactor, BatterySaver,
    ContainerSize { container: WidgetRef, size: Key },     // macOS's sidebar icon size
    OverlayScrollbars, OnOffLabels, FullKeyboardAccess, PreferCrossFade, AccessibilityTextSize,
    Idiom(Idiom), SystemSetting(Key),                      // a platform switch the reader reads, declared (S10)
    // window, by the connector
    WindowInactive, NotMain, AppInactive, Maximized, FullScreen, Tiled(TiledEdges),
    Elevated, TransientTouch, Input(InputKind),
    NotComposited, Shaded, AttentionFlash, TintDimmed, MnemonicsVisible,
    // application
    Flavour(Key), Preset(PresetId),
}
pub enum Scheme { Light, Dark }
pub enum InputKind { Mouse, Touch, Pen, GameController }
pub enum Idiom { Phone, Pad }                          // what the rows need (ADD-A09)
pub struct TiledEdges { pub top: bool, pub right: bool, pub bottom: bool, pub left: bool }
```

The list is closed by the catalogue and the traceability: a condition a row needs that is not here is added here.
The additions and their rows: ADD-A01 … ADD-A12 (S22). `OverlayScrollbars` replaces `scrollbar.overlay_mode`.
`AccessibilityPreferences`' flags map to `ReducedMotion`, `ContrastHigh`/`ContrastIncreased` (as each reader
states, R2.3) and `ReducedTransparency`; `text_scaling_factor` stays a factor (S9.4).

### S7.2 State flags (R3)

```rust
pub enum StateFlag { Hover, Pressed, Focused, FocusWithin, FocusVisible, Disabled, Checked, Selected,
                     Indeterminate, Open, Default, Dragged, DropTarget, ReadOnly, Invalid, Visited,
                     Autofilled, Scrolled, OverDarkContent, OpenedByTouch, Overflowing, Redacted,
                     Shared(SharedFlag), Widget(Key) }
pub enum SharedFlag { Empty, InFill, Overlapping, Today, OtherMonth, Current, Multiline, Filled, Half, HasValue,
                      HasText, Pulling, Refreshing }   // flags two or more platforms draw on a typed widget (ADD-A15)
```

`Shared` flags have one name for a portable application; the catalogue says which widgets declare each.
`Widget(Key)` is a one-platform flag the preset declares (S10). Exclusive groups: S10.

### S7.3 Sizes, density, size classes (R15, R16)

```rust
pub enum ControlSize { ExtraSmall, Mini, Small, Regular, Large, ExtraLarge }
pub enum Density { Default, Compact, Keyed(Key) }      // Material's density steps keyed
pub enum SizeClass { Compact, Medium, Expanded, Large, ExtraLarge, Regular }   // closed by the catalogue
pub struct SizeClasses { pub horizontal: Option<SizeClass>, pub vertical: Option<SizeClass> }   // R25.10
```

Each preset's facts state which of its sizes fills which `ControlSize` slot (Material Expressive's five button sizes,
AppKit's four control sizes); a preset declares only the slots it fills (S10).

## S8. Layers, order and resolution (R2.4, R2.5)

```rust
pub struct Layer { pub kind: LayerKind, pub bases: Bases, pub rules: Vec<Rule> }
pub enum LayerKind { Preset, Reader, App { replaces_reader_colours: bool, stands_for_reader: bool } }
pub struct Bases { pub colors: SchemeMap<Patch>, pub inputs: InputBases, pub tokens: Patch, pub widgets: Patch }
```

**Order:** every layer's bases in layer order (preset, reader, application layers in the order added), then every
layer's rules in that order (R2.4). A layer with `replaces_reader_colours` (a palette) removes the reader's colour
bases, colour rules and colour inputs; it is skipped while `ContrastIncreased` or `ContrastHigh` holds unless it
states rules for that condition (R19). A layer with `stands_for_reader` is placed as the reader (R2.6).

**Resolution** (`Theme::resolve(&WindowConditions) -> Result<ResolvedTheme, ResolveError>`):

1. merge bases in order; for colours, the base of the condition set's scheme;
2. apply the `Colors` and `Tokens` rules whose conditions hold, in order (choosing expressions, not evaluating);
3. for each part and widget, keep its base and its rules whose conditions hold, in order;
4. validate (S12).

It runs again on any change of a condition, an input, a reader value, a layer, or the preset; every such change but
a condition-only change is a theme change (S4.3).

**Query** (`ResolvedTheme::style(&Element) -> ResolvedPartStyle`, `ResolvedTheme::values(&Element)
-> ResolvedWidgetValues`; infallible):

```rust
pub struct Element<'a> {
    pub widget: WidgetRef, pub part: PartRef,
    pub variants: &'a [VariantRef], pub size: Option<ControlSize>,
    pub density: Option<Density>, pub direction: Option<Direction>,     // the element's own, if it states one
    pub language: Option<LanguageTag>,                                  // for icon lookup (S2.9)
    pub states: StateFlags,
    pub siblings: &'a [Sibling<'a>], pub index: usize,                  // all siblings, itself at `index`
    pub ancestors: &'a [Ancestor<'a>],                                  // nearest first
    pub data: ElementData<'a>,
}
pub struct Sibling<'a> { pub widget: WidgetRef, pub part: Option<PartRef>, pub variants: &'a [VariantRef],
                         pub states: StateFlags }
pub struct Ancestor<'a> { pub widget: WidgetRef, pub part: Option<PartRef>, pub variants: &'a [VariantRef],
                          pub states: StateFlags, pub density: Option<Density>,
                          pub size_classes: Option<SizeClasses>, pub direction: Option<Direction> }
pub struct ElementData<'a> {                                            // R25.3; Default: zero size, progress 0,
    pub width: f32, pub height: f32, pub progress: f32,                 // scroll 0, item 0 of 1, empty text
    pub scroll_offset: f32, pub scroll_progress: f32,
    pub item_index: u32, pub item_count: u32, pub text: &'a str,
    pub color: Color,                          // an application-chosen colour derived values follow
    pub char_width: f32,                       // the part's font's character width, as the toolkit measures it
    pub display_corner_radius: f32,            // where the platform reports it (R25.11)
}
```

The query: drops excluded flags by the exclusive groups (S10); computes the inherited density, size class and
direction from the element and its ancestors; applies the part's kept rules in order, skipping those that do not
match (position counted among the siblings the rule's `among` names); evaluates expressions (S2.2) with `base`,
`foreground`, `element.<data>`, `part.<field>` and bindings in scope. `Element.variants` includes the variants
the connector sets from the element's content (`icon_only`, `with_icon`; GTK sets `.image-button` itself,
ADD-A61). The query cannot fail (R2.8): element data are plain values,
and each part declares the data its values read (S10, `data = [...]`), which is what a connector must supply for
it, as it supplies the element's states; S12 rejects an expression naming data its part does not declare. `ResolvedTheme::available(&Element)
-> Availability` answers "not available" for an undeclared size or variant (S10).

## S9. Inputs and readers (R2.6, R18)

```rust
pub struct InputBases { pub entries: Vec<(InputName, InputValue)> }
pub struct InputValue { pub per_scheme: SchemeMap<Value<Color>>,
                        pub per_contrast: Vec<(Condition, SchemeMap<Value<Color>>)> }
```

**S9.1 Declared inputs per preset:** `accent` (all); `accent_ramp` entries (`windows-11`); the AppKit-derived
accent roles and `highlight` (`macos-*`); the tint-dependent roles (`ios-*`); `seed` (`material-3`); on
`kde-breeze` the inactive input roles and the scheme inputs `kde_scheme.<set>.<key>`, the values of the scheme file
before Plasma applies the accent (the preset's from Breeze's scheme files; the reader's from the file kdeglobals
names, `[General] ColorScheme`) (R25.1).

**S9.2 Setting the accent:** `Theme::set_accent(Accent)` sets the declared accent inputs in the application's
layer bases; on `windows-11`, `macos-*` and `ios-*` it requires every declared accent input and otherwise returns
`Err(AccentInputsMissing)`. On `kde-breeze` it takes the accent alone and also states, in the same layer's colour
bases, each accent-dependent role as `= kde_accent(<set>, <key>)`, so they follow Plasma's derivation over the
scheme inputs (R25.1); the reader itself states the derived values kdeglobals holds.

**S9.3 Readers** state each value under the conditions they read it in: base conditions into the reader layer's
bases, other conditions into reader rules (or inputs' per-contrast bases). The new reads per platform are the rows
of the traceability whose target is a reader read (S20); each is a `reader::<platform>` change.

**S9.4 Text scaling:** the reader supplies `text_scaling_factor` (1 where the platform scales by the font size
itself); the query multiplies font sizes by it before font-derived lengths, and pixel line heights with it (R10).

## S10. Declarations (R2.7)

```toml
[declare]
flavours = ["widgets", "kirigami"]            # first is the default
materials = ["mica", "mica_alt", "acrylic"]
colour_roles = ["fill_tertiary", …]            # keyed roles
text_styles = [ … ]
shapes = [ … ]                                 # the platform's shape library (R25.5)
elevation = ["1", "2", "8", "16", "32", "64", "128"]
surface_sets = ["window", "view", …]
conditions = ["@graphic_effects_level", …]    # SystemSetting keys
params = { layout = ["grid_unit", …], opacity = [ … ] }
inputs = ["accent", "accent_ramp.light1", …]

[declare.widget.button]
variants = ["primary", "destructive", "flat", "@pill"]      # @: keyed
sizes = ["small", "regular", "large"]
densities = ["compact"]
flags = ["@default_button"]
exclusive = [["disabled", "pressed", "hover"]]              # highest precedence first
sizes_by_context = [{ context = "navigation_bar", sizes = ["regular"] }]
default_size = "regular"                                   # the size of an element that states none (ADD-A60)
available = true                                           # false: the platform has no such control
requires_variants = []                                     # e.g. iOS's checkbox only as `selection`

[declare.widget.spinner]
data = { indicator = ["width", "height"] }                 # element data a part's values read (S8)

[declare.component.rule_editor]
parts = ["row", "popup_button", "add_button"]
```

Resolution rejects any rule, reference or patch naming something undeclared (S12).

## S11. Preset files (TOML)

```toml
[preset]
id = "adwaita"
generation = "libadwaita-1.10"
[declare] …                                    # S10

[inputs.accent]                                # every value below is a placeholder; values come
light = "<colour>"                             # from the facts in the value phase
dark = "<colour>"

[colors.light.surfaces.window]
background = "<colour>"
foreground = "<colour>"
[colors.light.global]
accent = "= input.accent"

[tokens.motion.durations]
button = "<ms>"

[widgets.button.body]
fill = "= surface.background"
corners = "<px>"

[[rules]]
when = { conditions = ["scheme:dark"] }
target = "button.body"
shadow = "<layers>"

[[rules]]
when = { states = ["checked", "hover"], variants = ["flat"] }
target = "button.body"
fill.append = ["= alpha(foreground, <fraction>)"]
```

`when` keys: `conditions`, `not_conditions`, `context` (array of `{ nearest = [...], widget, part, variants,
not_variants, states, not_states }` or `{ any = widget, … }`; `widget` may be left out to match by variant alone),
`position` (`{ among, first, last, only, parity, previous, next }`, `among` = `"same"`, `"any"` or a list of
widgets; `previous` and `next` as `{ widget, part, variants, states }`), `variants`, `not_variants`, `size`,
`density`, `size_class` (`{ horizontal, vertical }`), `direction`, `states`, `not_states`. Every value
carries its fact's citation in a comment, as today (`scripts/check_preset_citations.py`). Palettes use the same
format with `[preset] kind = "palette"` and only `colors`, `inputs`, `tokens.colour_effects` and colour rules.

## S12. Validation (R2.8)

Resolution fails with `ResolveError` (listing every problem) when:

- a field is missing (S2.2), or `Absent` where not `may_be_absent` (S6.3);
- a name is undeclared (S10) or a rule targeting `Colors`/`Tokens` has non-condition selector parts;
- an expression mistypes, refers to an unknown name, forms a cycle (through bindings too), or can meet an absent
  operand under any binding and rule it can apply under;
- a declared size, variant or flag has no rule or base that states its look (it would be indistinguishable);
- an expression names element data its part does not declare (S8).

`Theme::add_layer` validates the layer against the theme and refuses it with the error (R2.5).

## S13. Placement (R16)

```rust
pub enum PlacementStrategy { GtkPopover, GtkTooltip, QtMenu, QtTooltip, QtQuickTooltip, KToolTipWidget,
                             QtQuickMenu, QtComboBox, QtDialog, QtCompletionBox, QtWhatsThis,
                             KTextEditorCompletion, KirigamiTextActions,
                             WinUiFlyout(WinUiPlacement), WinUiToolTip, WinUiComboBox, WinUiAutoSuggest,
                             WinUiTeachingTip,
                             AppKitMenu, AppKitPopover, AppKitToolTip, AppKitExpansionTip, AppKitComboList,
                             AppKitSheet, AppKitWindowCenter,
                             MaterialMenu, MaterialTooltip, MaterialScaffoldSlot(ScaffoldSlot),
                             MaterialLevitatedPane, AndroidTextToolbar,
                             UiKitPopover, UiKitMenu, UiKitAlert, UiKitEditMenu,
                             CenteredInHost, BottomSheet, KeyboardAvoid(KeyboardAvoidance), Keyed(Key) }
pub enum ScaffoldSlot { Fab, Snackbar }
pub enum KeyboardAvoidance { WinUiContentDialog, UiKitAlert, WinUiScrollIntoView, AndroidPan, AndroidResize,
                             AndroidInsetsAnimation }
pub struct PlacementParams { pub gap: Value<f32>, pub edge_margin: Sides<Value<f32>>, pub side: PlacementSide,
                             pub align: PlacementAlign,            // mirrored in RTL
                             pub flip: Axes<bool>, pub slide: Axes<bool>, pub width: AnchorWidth,
                             pub over_selected_item: bool, pub max_visible_items: Option<u32>,
                             pub use_full_screen_height: bool, pub anchor_grow: Sides<Value<f32>>,
                             pub host: HostArea, pub window_edge_margin: Value<f32>,
                             pub handedness: bool }               // WinUI's and AppKit's menus (ADD-A21, ADD-A22)
pub enum PlacementSide { Below, Above, Start, End, AtPointer }
pub enum PlacementAlign { Start, Centre, End }
pub enum AnchorWidth { Content, AtLeastAnchor, Anchor }
pub enum HostArea { Window, ClientArea, Overlay }
pub fn place(strategy: &PlacementStrategy, params: &PlacementParams, anchor: Rect, size: Size,
             window: Rect, direction: Direction) -> Rect;
```

`native_theme::placement`, pure functions, each to its platform's source (cited in its doc comment); the closed
list is the traceability's placement rows.

## S14. Palettes (R19)

A palette is a file in S11's format with `[preset] kind = "palette"`: colour-role bases, input bases and colour
tokens for the schemes its project defines (R19). Two sources, each stated in the file's header:

- **native-theme's mapping** (the default): each role it covers maps to a colour the project publishes, named by the
  project's name for it (`# catppuccin: Surface0`), or to a derivation of those (S2.2); a value picked from a range
  the project's guide gives is marked `# pick: <the guide's words>`. It is the same file on every platform.
- **the project's port** for one platform, where one exists (Catppuccin's and Dracula's KDE colour schemes, R19):
  `Palette::from_port(file) -> Result<Palette, PortError>` converts a KDE `.colors` file mechanically, and that
  palette is used on `kde-breeze` in place of the mapping.

`Theme::apply_palette(Palette)` adds it as an application layer with `replaces_reader_colours`, refusing it
(`Err(SchemeNotInPalette)`) under a scheme it lacks; when the system scheme changes under it, the theme reports
`PaletteSuspended`. A palette carries no icon set. The facts gate checks each mapped colour against the project's
published colours (pinned like any source, S17) and skips the picks.

## S15. System control rendering (R11)

```rust
pub enum SystemControl { AppKitCheckbox, AppKitRadio, AppKitDisclosure, AppKitPushDisclosure, AppKitPopUp,
                         Keyed(Key) }                                     // ADD-A58
#[cfg(target_os = "macos")]
pub fn render_system_control(control: SystemControl, state: ControlState, size: ControlSize,
                             appearance: Appearance, key_window: bool, scale: f32) -> Option<RgbaImage>;
```

Main thread only (`None` elsewhere and off macOS); cached per (control, state, size, appearance, key, scale). The
list of `SystemControl`s is the traceability's rows that need it.

## S16. AccessibilityPreferences and the API surface

`AccessibilityPreferences` remains as the reader's report and gains the conditions of S7.1 it reports; `SystemTheme`
keeps `pick(Scheme)` (it sets the application's scheme condition, R2.3), `with_overlay` becomes `add_layer`, and
gains `apply_palette`, `set_accent`, `resolve`.

## S17. Facts as data and the gate (R4, R22)

**Format** (`docs/facts/<platform>/<area>.toml`):

```toml
[[fact]]
key = "button.body.fill"                          # model path
when = { states = ["checked", "hover"], variants = ["flat"] }   # as S11's `when`
value = "= alpha(foreground, <fraction>)"        # value or the platform's derivation
kind = "published"                                # read | published | measured
source = "libadwaita 1.10.0 src/stylesheet/widgets/_buttons.scss:L"
tolerance = 0.0                                   # measured: the measurement's precision
```

`docs/platform-facts.md` is generated from these files (a script), and the two existing scripts read the data.

**The gate** (`native-theme/tests/facts_gate.rs`): for every platform preset and every fact, resolve under the
fact's conditions and query its element; compare with the fact (within tolerance); repeat with perturbed inputs and
base colours, evaluating the fact's derivation the same way. The combination lists come from each platform's state
model (R4): the catalogue states, per widget and platform, the source of its list.

## S18. Documents and scripts

- `docs/property-registry.toml`: generated from the derive's field registry (S6.1) and drift-tested.
- `docs/inheritance-rules.toml`, `scripts/check_wrong_safety_nets.py`: removed (R20).
- `scripts/check_platform_facts_rows.py`, `scripts/check_preset_citations.py`: read the facts data.
- `scripts/check_spec_trace.py`: new; checks that every gap-map row (the IDs of the map's Appendix A), excluding
  withdrawn rows, has at least one line in the traceability (S20), that every line names a live row, and that each
  line's kind and target agree (a target kind has a target, a verdict has `-`). It checks working documents, so
  it is run by hand while they change and is not part of `scripts/check_release.sh`; it is deleted when the
  documents are archived.
- The showcases' element lists and parity gates follow the connectors (R21).

## S19. Migration inside the branch

**S19.1** The presets are rewritten to S11 from the facts; the community presets become palettes (S14): one per
variant the project publishes, their invented light variants and their `icon_set` removed.
**S19.2** Today's inheritance rules become expressions in each preset's bases, converted by a script from
`docs/inheritance-rules.toml` before it is removed, then checked by the gate.

## S20. Traceability format (R24)

Per platform, `todo_v0.7.1_model-completeness-spec-trace-<platform>.md`: at least one line per live gap-map row,
several where a row needs several targets:

| Row | Target | Kind |
|---|---|---|
| GN-1.29 | `button.body.fill` rules for {hover}, {checked}, {checked, hover} under variant `flat` | rule |
| … | | |

Kinds: `field`, `rule`, `role`, `token`, `component`, `input`, `reader` (a new read), `placement`, `system-control`,
`condition`, `flag`, and the verdicts, whose target is `-` (or `- (note)` naming the part of the row a verdict
covers when the row's other parts are carried; `readers (R18)` may instead name the read the later reader makes): `out (R1.1)` (the system draws it), `out (R1.2)` (a
library failing R1.2's test), `out (R1.4)` (SwiftUI-only drawing), `preset (R1.3)` (another toolkit's preset),
`newest (R1.5)` (an older generation's look), `readers (R18)` (the iOS and Android readers' milestone).

## S21. Release and compatibility (R21)

**S21.1** 0.8.0, one release, on `v0.8.0-rc1`; every platform preset, `ios-26` included, complete and passing the gate.
**S21.2** Exhaustive enums (no `#[non_exhaustive]`): `StateFlag`, `Condition`, `Value`, `IconRole` (loses it),
`BlendMode`, `Timing`, `GlyphSource`, `PlacementStrategy`, `TransitionKind`, `PresentationKind`, `ThemeChange`,
`FocusKind`, `Radius`, `PaintKind`, `Pattern`, `QtBrushStyle`, `Shadow`, `ControlSize`, `SiblingScope`, `Parity`,
`Antialiasing`, `Hinting`, `SubpixelOrder`, `DashPattern`, `PixelSnap`, `LineCap`, `LineJoin`, `EdgeSide`,
`TextCase`, `TextSlot`, `FocusVisibleWhen`, `Mnemonics`, `OverscrollEffect`, `BlinkStyle`, `Idiom`, `SharedFlag`,
`ScaffoldSlot`, `KeyboardAvoidance`, `PlacementSide`, `PlacementAlign`, `AnchorWidth`, `HostArea`, `SystemControl`. Every struct: `#[non_exhaustive]`.

## S22. Additions register

The catalogue groups asked for additions to this core; the traceability cites them by ID. Each is decided: class A
(`ADD-Ann`) are details inside the rationale's mechanisms, class B (`ADD-Bnn`) needed decisions the rationale took in
R25, and `CONTRA-n` were contradictions with the rationale, settled there. The rows each needs are the traceability's
lines that cite it.

| ID | What | Carried by |
|---|---|---|
| ADD-A01 | `Condition::NotComposited` | S7.1 |
| ADD-A02 | `Condition::Shaded` | S7.1 |
| ADD-A03 | `Condition::AttentionFlash` | S7.1 |
| ADD-A04 | `Condition::SystemSetting(Key)` and `[declare] conditions = [...]` | S7.1 |
| ADD-A05 | `Condition::OnOffLabels` | S7.1 |
| ADD-A06 | `Condition::FullKeyboardAccess` | S7.1 |
| ADD-A07 | `Condition::PreferCrossFade` | S7.1 |
| ADD-A08 | `Condition::AccessibilityTextSize` | S7.1 |
| ADD-A09 | `Condition::Idiom` (phone or pad) beside the size class | S7.1 |
| ADD-A10 | `Condition::TintDimmed` | S7.1 |
| ADD-A11 | `Condition::OverlayScrollbars` | S7.1 |
| ADD-A12 | Mnemonics | S7.1, S2.8, S4.2 |
| ADD-A13 | `StateFlag::Overflowing` | S7.2 |
| ADD-A14 | `StateFlag::Redacted` | S7.2 |
| ADD-A15 | Typed widget flags | S7.2 |
| ADD-A16 | one `focus_visible_when` type for every widget | S2.10, S2.11, S4.2 |
| ADD-A17 | `FocusIndicator.secondary` | S2.10 |
| ADD-A18 | `FocusIndicator.offset` per side | S2.10 |
| ADD-A19 | `TransitionKind::FocusMove` | S2.7 |
| ADD-A20 | size classes per axis | S7, S8 (R25.10) |
| ADD-A21 | Placement strategies for S13's closed list | S13 |
| ADD-A22 | `PlacementParams` fields | S13 |
| ADD-A23 | Overwrite-mode caret | S4.2 |
| ADD-A24 | The caret as a part of the text widgets | S4.2 |
| ADD-A25 | Presentation-transition details | S2.7 |
| ADD-A26 | `PresentationKind::Keyed` with parameters; keyed kinds | S2.7 |
| ADD-A27 | `TransitionKind::Keyed(Key)` | S2.7, S2.9 (R25.12) |
| ADD-A28 | `Keyframe` defined | S2.7 |
| ADD-A29 | Overscroll effect in `ScrollPhysics` | S4.2 |
| ADD-A30 | Repeating linear gradient | S2.3 |
| ADD-A31 | Gradient extent in pixels | S2.3 |
| ADD-A32 | Dashed and dotted strokes | S2.4 |
| ADD-A33 | Device-pixel hairlines | S2.4 |
| ADD-A34 | Line caps and joins | S2.4, S2.9 |
| ADD-A35 | `Glyph.draw_extent` | S2.9 |
| ADD-A36 | theme-icon glyph fallbacks | S2.9 |
| ADD-A37 | Icon-style additions | S2.9 |
| ADD-A38 | `Elision::Fade` length as a fraction of the text box | S2.8 |
| ADD-A39 | `TextStyle.case` | S2.8 |
| ADD-A40 | `TextStyle.line_height` in points, pixels or an expression on `font_size` | S2.8 |
| ADD-A41 | Font fallbacks as one token, `token.text.fallbacks` | S4.4 |
| ADD-A42 | Text ramp | S4.4 |
| ADD-A43 | `IconSizeTokens` with keyed contexts | S4.5 |
| ADD-A44 | Keyed scalar tokens, `Tokens.params`, referred to as `token.params.<group>.<key>` and … | S4.6 |
| ADD-A45 | Behaviour fields | S4.2 |
| ADD-A46 | `Tokens.pointer` | S4.7 |
| ADD-A47 | `Material.params` | S2.3 |
| ADD-A48 | Colour-role closure | S3 |
| ADD-A49 | `GlobalRoles.selection_background_inactive` | S3 |
| ADD-A50 | `qt_lighter(c, factor)`, `qt_darker(c, factor)` | S2.2 |
| ADD-A51 | `kcu_overlay(base, paint)` | S2.2 |
| ADD-A52 | `qt_tint(base, tint)` | S2.2 |
| ADD-A53 | Kirigami functions | S2.2 |
| ADD-A54 | `sf_tracking(font_size, design)` | S2.2 |
| ADD-A55 | Lengths that follow text scaling | S2.2 |
| ADD-A56 | Arithmetic | S2.2 |
| ADD-A57 | `kqc_series_colour(seed, count, index)` | S2.2 |
| ADD-A58 | `SystemControl::AppKitPushDisclosure` | S15 |
| ADD-A59 | `CursorShape::Crosshair`, `WhatsThis` | S2.11 |
| ADD-A60 | size defaults and availability declarations | S10 |
| ADD-A61 | content variants set by the connector | S8 |
| ADD-A62 | Material Expressive's five sizes | S7.3 (R25.10) |
| ADD-A63 | libspelling in scope | R1.2 |
| ADD-A64 | GtkSourceView schemes in the `adwaita` preset | the catalogue's `code_editor` (R25.11) |
| ADD-A65 | a fallback icon theme | S4.5 |
| ADD-A66 | kirigami-addons functions | S2.2 |
| ADD-A67 | `TextStyle.max_lines` | S2.8 |
| ADD-B01 | Pattern paint | S2.3 (R25.4) |
| ADD-B02 | conic gradient | S2.3 (R25.4) |
| ADD-B03 | Polygon and path shapes, and morphing | S2.4, S2.7 (R25.5) |
| ADD-B04 | Text rendering settings | S4.4 (R25.7) |
| ADD-B05 | Icon variants by language and direction | S2.9 (R25.8) |
| ADD-B06 | Parity selector | S7 (R25.9) |
| ADD-B07 | Sibling and position matching by part | S7 (R25.9) |
| ADD-B08 | Context matching by variant alone | S7 (R25.9) |
| ADD-B09 | Presentation transitions targeting parts | S2.7 (R25.6) |
| ADD-B10 | Presentation transitions combined per event | S2.7 (R25.6) |
| ADD-B11 | element data: its own size, and the part's other fields | S2.2, S8 (R25.3) |
| ADD-B12 | element data: progress | S2.2, S8 (R25.3) |
| ADD-B13 | Values interpolated by a continuous scroll position rather than over time | S2.2, S8 (R25.3) |
| ADD-B14 | A colour chosen by hashing the element's text | S2.2, S8 (R25.3) |
| ADD-B15 | A series index and count as operands | S2.2, S8 (R25.3) |
| ADD-B16 | An application-chosen colour that derived values follow | S2.2, S8 (R25.3) |
| ADD-B17 | Character-width operand `char_width` | S2.2, S8 (R25.3) |
| ADD-B18 | the display's corner radius as element data | S8 (R25.11) |
| CONTRA-1 | R2.3 "KDE has no contrast setting" against K2.23† | R25.2 |
| CONTRA-2 | R9 velocity curves as a timing kind of their own | R9, R25.6 |
| CONTRA-3 | R2.3 position among the same widget kind against GTK's `.linked` | R2.3, R25.9 |
| CONTRA-4 | R11 symbol effects as transitions against an icon "effect" | R25.12 |
| CONTRA-5 | R5.4 a width in characters as a multiple of the font size | R5.4, R25.3 |
