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
| Widget catalogue | merged (84 typed widgets, keyed components per platform) |
| Traceability | every row of the six platforms traced; `scripts/check_spec_trace.py` passes |
| Check passes | 1: core 11 serious, catalogue 5, traces 9 in 256 rows; 2: core 10, catalogue 4, traces 6.7 % of 240 rows (then every row re-verified); 3: core 6, catalogue 5, traces 0.8 % of 240 rows; each fixed |

## S1. Conventions

- **Names.** Rust types are `UpperCamelCase`, fields `snake_case`, with no unit suffix: the type carries the unit.
  TOML keys are the field names with the unit's suffix added from the type (`_px`, `_ms`, `_deg`), as today's files
  spell them, and a stated value under such a key is a plain number (`min_width_px = <n>`); `"<n>px"` is spelled only
  inside expressions. Paths in expressions, the catalogue and the traceability use the Rust names (`part.min_width`,
  `token.caret.blink_on`); the catalogue's declarations of scalar widget values show the TOML key
  (`icon_text_gap_px: Value<Px>`); compound types (`Sides`, `Axes`) take no suffix. Identifiers use American spelling
  (`color`, `center`, `minimize`), as the code does; prose is British.
- **Units.** `Px(f32)`, logical pixels; `Ms(f32)`, milliseconds; `Deg(f32)`, degrees; `Fraction(f32)`, `0..=1`
  (opacities, amounts); plain `f32` for unitless numbers (scales, factors) and for velocities (px per second, stated
  at the field); colours extended sRGB (S2.1). `Scalar` is `Px | Ms | Deg | Fraction | Number(f32)`, for keyed
  values whose unit their declaration states (S10). A length may be negative only where its field says so (offsets,
  translations, letter and word spacing); elsewhere a negative result is zero (S2.2).
- **Keys.** A keyed name (a platform's own colour role, variant, component, text style, elevation entry,
  material, presentation kind) is a `Key`, `[a-z0-9][a-z0-9_]*` (WinUI's elevation steps are `1` … `128`),
  declared by the preset (S10); `Key::new(&str) -> Result<Key, KeyError>`, and `key!("name")` checks a literal at
  compile time. A typed name is an enum variant.
- **`#[non_exhaustive]`** (R21): every public struct has it, except the closed value structs (`Color`, `Sides`,
  `Axes`, `Rect`, `Size`, `Point`, `TiledEdges`, `SetMatch`, and the newtypes `Px`, `Ms`, `Deg`, `Fraction`, `Key`);
  every public enum is exhaustive, so a connector maps each in full (S21.2). A public struct whose every field has a
  neutral default (the patch types, `Selector`, `ElementData`, `ReaderState`, `WindowConditions`, `Layer`, `Bases`)
  implements `Default` and is built from `default()` and field assignment, as R21 prescribes; a struct with required
  fields has a constructor (`Element::new(part, scope, metrics)`, `Rule::new(selector, target, patch)`); a full block
  is built through its patch type. Validation refuses an `Absent` left where it is not allowed (S12), so a default
  never stands in for a value. Every public enum is exhaustive, error enums included, a choice made for
  simplicity before 1.0. A layer is also built from TOML in S11's format, `Layer::from_toml(&str) ->
  Result<Layer, LayerError>`.
- **Absence.** In the model, one route: `Value::Absent`, allowed where S6.3's two sources allow it: a block's
  sub-field this document marks "may be absent" (wherever that block appears), and the catalogue's `may_be_absent`
  tables (per widget, part, field, colour role and token). In the model, `Option` appears only where `None` means
  something other than "not drawn", stated at the field (`max_lines: None` is no limit); a resolved `Value` field is
  `None` when not drawn (S6.1).

- **Small types.** `PartRef` is `Typed(TypedPart)` or `Component { key: Key, part: Key }`, where `TypedPart` is a
  derive-generated enum over the typed widgets, each holding that widget's part enum (`TypedPart::Button(ButtonPart::
  Label)`), so a misspelled typed part, or a part paired with the wrong widget, does not compile;
  `StateFlags` is a set of `StateFlag`; `InputName` is an input's path (`accent`, `accent_ramp.light1`);
  `CaretShape` is `Bar | Block | Underline`; `SizeClassThreshold` is `{ horizontal: Value<Px>, vertical: Value<Px> }`, the
  class's inclusive lower bound on each axis (either may be absent: that class is not chosen by that axis); a
  window's class on an axis is the one with the largest bound not above its size there, or the class with no bound
  where none is; `WinUiPlacement` is WinUI's `FlyoutPlacementMode`; `Accent` is `{ accent: InputValue, derived: Vec<(InputName, InputValue)> }`
  (S9.2); `WidgetValue` is the union of the value types widget values use: a number, integer, length, duration,
  bool, key, enum variant, colour, paint, timing, transition, loop, glyph, icon style, scroll physics,
  `Vec<PresentationTransition>` and a placement (`PlacementStrategy` with its `PlacementParams`); a keyed component's
  values are typed by its declaration (S10). `DisabledRoles` and `StatusRoles` are the catalogue's (*Colour
  roles*). `Expr` is a parsed expression (S2.2); `StepJump` is CSS's `jump-start | jump-end | jump-none |
  jump-both`; the `Widget` trait, `ComponentValues`, `ComponentBase`, `ResolvedTokens`, `ResolvedFont`,
  `ResolvedPartStyle` and the per-widget types are derive-generated (S6.1); `DataNeeds` is a set of element-data
  operands and metrics; `LayerId`, `PaletteId` are opaque ids; `ThemeError`, `ReaderStatus`, `KeyError` are the
  errors and status their calls name. `SchemeMap<T>` holds a `T` per `Scheme`; `SurfaceMap<T>` one per declared `SurfaceSet`; `SurfaceRole` and
  `GlobalRole` name a field of `SurfaceRoles` and `GlobalRoles` (a path, `disabled.foreground`); `WidgetKind` is the
  typed widgets' enum and `Variant` the typed variants' (both generated from the catalogue); `PresetId` names a
  bundled preset; `AxisTag` and `FeatureTag` are OpenType four-letter tags; `LanguageTag` is a BCP 47 tag;
  `FontStyle` is `Normal | Italic | Oblique`; `Sides<T>`, `Axes<T>`, `Rect`, `Size`, `Point` are plain geometry in
  `Px`. Error and status enums: `LayerError` (every validation problem, S12, and `ReaderPresent`,
  `AccentInputsMissing`, `SchemeNotInPalette`), `PortError`, `PaletteStatus`.

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
literal  := colour literal | number | length ("<n>px") | duration ("<n>ms") | angle ("<n>deg") | font size ("<n>pt";
            "<n>px" is a font size where the field or parameter is one, else a length)
          | name (an identifier: an enum variant in snake_case, or "@" key for a keyed name)
          | "true" | "false"
path     := segment { "." segment }       a map's key, or an enum-keyed map's variant in snake_case, is a segment
ref      := "role." set "." role        typed colour role of a surface set           (S3)
          | "role.global." path           typed global role (accent, status.error.fill, …)  (S3)
          | "role.@" key                  keyed platform role                          (S3)
          | "surface." role               role of the surface bound in scope           (S3.3)
          | "active(" ref ")"             a role's active value                        (R5.1)
          | "token." path                 a token                                      (S4)
          | "input." path                 an input                                     (S9)
          | "base"                        the value earlier rules gave in this query   (R2.5)
          | "scheme"                      the scheme of the colours being resolved     (R5.1)
          | "font_size" | "line_height"   the part's, in logical px after text scaling (R10)
          | "element." data               the element's data (S8): width, height, progress,
                                          scroll_offset, scroll_progress, item_index,
                                          item_count, text, color, screen_width,
                                          screen_height, window_width, window_height, move_distance,
                                          and char_width, font_x_height, font_height (the
                                          part's font, through `Element.metrics`)      (R25.3)
          | "part." field                 another resolved field of the same part      (R25.3)
          | "widget." part "." field      a resolved field of another part of the same widget (a band
                                          following the frame's shadow, GN-1.282†; a label following its
                                          half's corners, MAT-X37†), evaluated with the same element and
                                          scope; not into or out of an *(element)* part (R25.3)
          | "values." name                the element's widget's resolved value (the spinner's stroke weight,
                                          GN-1.227; a delay button's delay, Q2.3)      (R25.3)
          | "text_scale"                  the text-scaling factor (S9.4)               (ADD-A55)
call     := name "(" expr { "," expr } ")"
```

The reserved words (`base`, `scheme`, `font_size`, `line_height`, `text_scale`, `active`, `true`, `false`) are not
names. The block tokens parts reference (`TextStyle`, `FocusIndicator`, `IconStyle`) may use `font_size` and
`line_height`, the style's own: `ResolvedTheme::tokens()` evaluates them against it, and a referencing part's query
evaluates them where used (R2.5). `part.` is not allowed in tokens (S12). An expression
nests at most 32 deep, and a chain of references is at most 32 long (S12). `font_size` and `line_height` are lengths; a length is accepted where a font size is expected, as px. Under
`line_height = auto` the `line_height` operand is the font's own line spacing, `Element.metrics.height` (the part
declares it, S8).

**Typing.** One rule for every field: a field of type `T` accepts a stated `T`; a reference to a token or a field
of type `T` (a block, enum, bool, key, paint, shadow or scalar alike: `= token.behavior.focus_visible_when`,
`= widget.frame.corners`); and, for scalar and colour types only, a call. A block field's expression is a reference
only. Operand types: colour, length, duration, angle, number, font size, text, bool, name, `enum<E>` (a literal
variant or a reference to a field of type `E`, `token.kde.disabled.intensity`), shadow, paint. In `add`, `sub`, `min`
and `max` a bare number takes the other operand's type; in `mul` one operand is a number. A `Fraction` result is
clamped to `0..=1`; non-finite TOML numbers are refused (S12).

Every role reference (`role.…`, `surface.…`) resolves against the nearest binding of that role in scope (S3.3),
else the root's value. The element's current foreground is `surface.foreground` (R5.4); a `Binding::Role` on it
rebinds it. `base` is an error in a layer's bases (nothing is beneath); in a `Colors` or `Tokens` rule it is the
expression the rules before it chose (S8).

A field whose type is a block (`TextStyle`, `FocusIndicator`, `IconStyle`, `Timing`) is a `Value` of that block,
and its expression may only be a reference to a token or field of that type (`= token.focus`, `= token.text.keyed.body`,
`= token.motion.timings.standard`). A struct block field's patch is `Set(Value<T>)` (a whole block or a reference)
or `Fields(TPatch)`, which applies its fields over the block below (refused over an `Absent` or `Auto` value, and
for enum blocks such as `Timing`, which take `Set` only): the token's focus indicator with only its kind
changed (R2.1). In TOML (S11), a table under a rule's `set`, or in a layer's bases above the preset, is a `Fields`
patch; a whole block is replaced there only by a reference or by `{ replace = { … } }`; in the preset's own bases a
table is the whole block. A shadow names an elevation by `Shadow::Elevation(key)`, not by reference (R8). A colour literal or
colour expression stated on a `Value<Paint>` field, or in `append`, is one `Color` layer with `BlendMode::Normal`.

**Functions** (a closed set, R5.4; each implemented to the source named, pinned in the facts' sources, S17):

| Function | Result | Definition |
|---|---|---|
| `alpha(c, a)` | colour | `c` with its alpha multiplied by `a` |
| `mix(a, b, t)` | colour | interpolation from `a` (t = 0) to `b` (t = 1) in sRGB with premultiplied alpha, as CSS `color-mix(in srgb, a (1−t)·100%, b t·100%)` |
| `kde_effects(c, background, group, role_kind)` | colour | KColorScheme's effects of a group (`disabled`, `inactive`) on a role of kind `foreground`, `decoration` or `background` (the contrast effect only on the first two, against `background`), in KColorScheme's order, from `token.kde.<group>`; the inactive group only while `token.kde.inactive_enabled`; on the Selection set's inactive roles, while `inactive_change_selection_color` is on, KColorScheme's substitution instead (Window's background tinted with Selection's, Window's text, K3.40, `kcolorscheme.cpp:303-321`); it reads those tokens itself |
| `kcu_mix(a, b, bias)`, `kcu_tint(a, b, amount)`, `kcu_shade(c, luma, chroma)` | colour | `KColorUtils`, to `kcolorutils.cpp` |
| `adw_standalone(c, scheme)` | colour | libadwaita's standalone accent derivation in OKLab, to its stylesheet |
| `m3_role(role, seed, variant, version, contrast, scheme)` | colour | material-color-utilities' dynamic scheme, the role of that scheme |
| `mul(x, y)`, `add(x, y)`, `sub(x, y)`, `min(x, y)`, `max(x, y)`, `floor(x)` | number, length, duration or font size | arithmetic (a token times the speed factor; a font size times a factor) (ADD-A56) |
| `qt_lighter(c, factor)`, `qt_darker(c, factor)` | colour | `QColor::lighter`, `QColor::darker` in HSV, to qtbase `qcolor.cpp` (ADD-A50) |
| `kcu_overlay(under, over)` | colour | `KColorUtils::overlayColors`, to `kcolorutils.cpp` (ADD-A51) |
| `kirigami_tint(c, tint, alpha)` | colour | Kirigami's `ColorUtils.tintWithAlpha` (ADD-A52) |
| `kirigami_label_icon_size(line_height)`, `kirigami_badge_text(text, background)` | length, colour | Kirigami's `units.cpp` and `Badge.qml` (ADD-A53) |
| `ka_card_border(background, text)`, `ka_banner_line(type_text, frame)`, `ka_contrast_text(c)` | colour | kirigami-addons' FormCard background, banner and avatar code (ADD-A66) |
| `kqc_series_color(seed, count, index)`, `kte_minimap_mark(mark, background)`, `kte_mark_underlay(hue_from, background, foreground)` | colour | KQuickCharts' `ColorGradientSource.cpp`, KTextEditor's `kateviewhelpers.cpp`: the marks' HSV recolouring, and their underlay in HSL (one colour's hue and saturation at the background's lightness moved toward the foreground, K-P3-12†) (ADD-A57, ADD-A90) |
| `sf_tracking(font_size, design: TrackingDesign)` | length | Apple's size-to-tracking tables for SF Pro, SF Pro Rounded, New York (ADD-A54) |
| `uikit_prominence(c, prominence)` | colour | `UIColor.withProminence(_:)` (X-CO-5) |
| `gtk_contrast_pick(c, on_light, on_dark)` | colour | `on_light` where GTK finds `c` light, else `on_dark`, to `gtkcolorswatch.c`'s lightness test (GN-2.27) |
| `shadow_extent(shadow, side)` | length | how far a shadow reaches beyond the box on that side, over its layers, to GTK's `gtk_css_shadow_value_get_extents` (GN-1.282†) |
| `gtk_wheel_step(page_size)` | length | GTK's scroll step from the page size, to `gtkrange.c` / `gtkscrolledwindow.c` (GN-P3-15†) |
| `uifm_scaled(x, text_role)` | length | `UIFontMetrics.scaledValue(for:)` (ADD-A55) |
| `kde_accent(set, key)` | colour | Plasma's accent derivation for one key of one colour set, from `input.accent` and the scheme inputs `input.kde_scheme.<set>.<key>`, to `plasma-workspace` `kcms/colors/colorsapplicator.cpp` `applyScheme()` (R25.1) |
| `name_index(text, count, method: NameIndexMethod)` | number | the index a platform's avatar hashes a name to; `NameIndexMethod = Kirigami \| Adwaita` (Kirigami's `nameutils.cpp`, libadwaita's `adw-avatar.c`), each to its source (R25.3; kirigami-addons' own `nameutils.cpp` is compared with Kirigami's in the value phase and becomes a member if it differs); `TrackingDesign = SfPro \| SfProRounded \| NewYork` |
| `pick(index, c0, c1, …)` | colour | the colour at `index` (R25.3) |

A function is added to this table only with the platform rule that needs it and its source (R5.4). How each KDE
function treats a translucent input (mixes, keeps or ignores alpha) is read from its source in the value phase and
recorded in its facts (R5.4). A platform
that computes a stored colour by composition (KDE's `overlayColors`) gets a function; composition drawn over what
lies beneath is a paint layer (R5.4, R6). Type checking: every operand and result is typed (colour, length,
duration, angle, number, font size, text, bool, name); a name operand is checked against the enum or declared key
set the function's parameter names (`effect: KdeIntensityEffect`, `design: @key`); a bare number takes the type of
the other operand; `mul(T, number)`, `add(T, T)`, `sub(T, T)`, `min(T, T)`, `max(T, T)` and `floor(T)` give a `T`;
a mismatch is an error when the layer is added (S12). A function that reads inputs or reader values itself
(`kde_accent` reads `input.accent`, `input.kde_scheme.<set>.<key>`, `token.kde.tint_factor` and
`titlebar_is_accent_colored`; with tinting on, every role depends on the accent; `uifm_scaled` the reader's content-size
category, S9.4) lists them, and the checks treat them as its operands.

**Totality** (the query cannot fail, R2.8): `alpha` clamps the resulting alpha to `0..=1`, as CSS Color 5's
relative colours do (a factor above one is valid: libadwaita multiplies a shade's alpha by more, GN-1.210); `mix`
clamps its fraction to `0..=1`; `pick` floors the index and takes its Euclidean remainder by the count; every count
operand of zero is one (`pick`, `name_index`, `kqc_series_color`); a length or duration that comes out negative where the field cannot
be negative is zero; a non-finite result, or non-finite element data, is zero. A platform function follows its
source's own clamping first. An expression naming `element.<data>` is re-evaluated by each query, so a connector queries again when the
data changes, as for a state (R25.3).

### S2.3 Paint (R6)

```rust
pub struct Paint { pub layers: Vec<PaintLayer> }             // bottom first; at least one layer (S12);
                                                             // "paints nothing" is `Value::Absent`
pub struct PaintLayer { pub kind: PaintKind, pub blend: BlendMode }
pub enum PaintKind {
    Color(Value<Color>),
    Linear { angle: Deg, stops: Vec<Stop>, repeat: bool,
             extent: Option<Value<Px>> },     // stops over this many px from the start edge (ADD-A31); None: the box
    Radial { center: (f32, f32), radius: Value<Px>, stops: Vec<Stop>, repeat: bool },  // center: fractions of
                                                                                        // the box; radius in px
    Conic { center: (f32, f32), start_angle: Deg, stops: Vec<Stop> },          // R25.4
    Pattern { pattern: Pattern, foreground: Value<Color>, background: Value<Color>,  // R25.4; the grid's lines
              cell: Value<Px> },                                                  // are its foreground
    Material(Material),
}
pub enum Pattern { Checkerboard, Grid, QtBrush(QtBrushStyle) }   // each to its platform's source (R25.4)
pub enum QtBrushStyle { Dense1, Dense2, Dense3, Dense4, Dense5, Dense6, Dense7, Hor, Ver, Cross, BDiag, FDiag,
                        DiagCross }                               // Qt's `Qt::BrushStyle` patterns
pub struct Stop { pub position: f32, pub color: Value<Color> }
pub enum BlendMode { Normal, Keyed(Key) }    // platform blending modes as the facts name them (macOS vibrancy)
pub struct Material {
    pub identity: Key,                        // the platform's material (S10 declares the set)
    pub blur_radius: Value<Px>,
    pub tint: Value<Color>,
    pub saturation: Value<f32>,               // a multiplier, at least zero (CSS `saturate()`, K3.11)
    pub luminosity: Value<Fraction>,
    pub noise_opacity: Value<Fraction>,
    pub fallback: Value<Color>,               // what a toolkit that cannot draw it draws (R6)
    pub params: Vec<(Key, Value<Scalar>)>,    // what a platform's material states beyond these (ADD-A47)
}
```

A rule's patch either sets a paint (`fill = …`) or appends layers over `base` (`fill.append = [ … ]`) (R6). A
repeating gradient repeats its stops beyond the last (stripes, K1.110).

### S2.4 Corners, stroke (R7)

```rust
pub struct Corners { pub top_left: Radius, pub top_right: Radius, pub bottom_right: Radius,
                     pub bottom_left: Radius, pub curve: CornerCurve }
pub enum Radius { Px(Value<Px>), Full, Concentric { minimum: Value<Px> },
                  FollowPart }   // a focus indicator's: the part's radius plus the offset, concentric (R12);
                                 // refused where the corner's two sides have different offsets (S12)
pub enum CornerCurve { Circular, Continuous }
pub struct Stroke { pub top: Edge, pub right: Edge, pub bottom: Edge, pub left: Edge,
                    pub position: StrokePosition,
                    pub outset: Value<Px> }      // the gap between the box and an `Outside` stroke (iOS's
                                                 // `strokeOutset`, CD-1); absent elsewhere
pub struct Edge { pub width: Value<Px>, pub paint: Value<Paint>, pub dash: Value<DashPattern>,   // ADD-A32
                  pub snap: PixelSnap }                                                    // ADD-A33
pub enum DashPattern { Solid, Dotted, Dashed { on: Px, off: Px } }
pub enum PixelSnap { None, ToDevicePixel, AtLeastOneDevicePixel }   // the width stays logical
pub enum LineCap { Butt, Round, Square }
pub enum LineJoin { Miter, Round, Bevel }
pub enum StrokePosition { Inside, Center, Outside }
```

A part's outline is its `corners`, unless `PartStyle.shape` names a keyed shape of the platform's shape library
(declared in S10, each defined to its source, R25.5); an `Interpolate` transition of the `Shape` group morphs from one
shape to the other.

TOML shorthands: `corners_px = <n>` sets all four radii (the curve is stated on its own); `stroke = { width_px = <n>, paint = "…" }`
sets all four edges.

### S2.5 Shadow (R8)

```rust
pub enum Shadow { Layers(Vec<ShadowLayer>), Elevation(Key) }   // Key: an entry of the elevation ramp (S4), which
                                                                 // may be empty (Material's level 0: no shadow);
                                                                 // `Layers` has at least one; no shadow is `Absent`
pub struct ShadowLayer { pub x: Value<Px>, pub y: Value<Px>, pub blur: Value<Px>,
                         pub spread: Value<Px>, pub color: Value<Color>, pub inset: bool }
```

### S2.6 Transform (R9)

```rust
pub struct Transform { pub scale_x: Value<f32>, pub scale_y: Value<f32>, pub rotate: Value<Deg>,
                       pub translate_x: Value<Px>, pub translate_y: Value<Px>,
                       pub origin: (f32, f32) }                // fractions of the part's box
```

### S2.7 Transitions and timing (R9)

```rust
pub enum Timing {
    CubicBezier(f32, f32, f32, f32),
    Linear,
    Steps { count: u32, jump: StepJump },
    Spring { mass: f32, stiffness: f32, damping: f32,
             initial_velocity: f32, rest_epsilon: f32, clamp: bool,
             time_scale: Value<f32> },        // `= token.motion.speed_factor` where the platform scales springs,
                                              // else 1; zero jumps to the end (R9)
    Platform(PlatformCurve),                  // a platform's own curve, implemented once in native-theme to its
}                                             // cited source (timing::evaluate)
pub enum PlatformCurve { KirigamiVelocity }   // Kirigami's velocity-based animations (Q3.26, R25.6); closed
pub enum PropertyGroup { Fill, Stroke, Corners, Shape, Shadow, Opacity, Transform, Text, Icon, Glyph, Geometry }
                                              // the focus indicator's motion is its own `transition` (S2.10)
pub struct Transition {
    pub properties: Vec<PropertyGroup>,
    pub kind: TransitionKind,
    pub duration: Value<Ms>,                 // may be absent: for a spring, whose time is its own (S2.7)
    pub delay: Value<Ms>,
    pub timing: Value<Timing>,                // a timing, or a reference to a timing token
    pub suspended_by_theme_change: bool,      // libadwaita's CSS transitions are, its AdwAnimations are not
}
pub enum TransitionKind {
    Interpolate,
    Ripple { max_radius: Value<Px>, initial_scale: Value<f32>, padding: Value<Px>, soft_edge: Value<Px>,
             opacity: Value<Fraction>, fade: Value<Ms>, min_press: Value<Ms>,
             touch_delay: Value<Ms> },    // MAT-X9, MAT-X25
    FocusMove,                                // the indicator travelling between elements (ADD-A19)
    Keyed { key: Key, params: Vec<(Key, Value<Scalar>)> },   // SF Symbols' content transitions and effects,
}                                             // WinUI's AnimatedIcon states; values measured (ADD-A27, R25.12)
pub struct PresentationTransition {          // one event's presentation: its steps run together (R25.6)
    pub event: PresentationEvent,             // Appear, Disappear, Move, and widget-specific keyed events
    pub steps: Vec<PresentationStep>,
}
pub struct PresentationStep {
    pub part: Option<PartRef>,                // None: the whole element
    pub kind: PresentationKind,
    pub duration: Value<Ms>, pub delay: Value<Ms>, pub timing: Value<Timing>,
    pub stagger: Value<Ms>,                  // ms between items, where the element presents items
}
pub enum PresentationEvent { Appear, Disappear, Move, Keyed(Key) }   // widget-specific events, declared (S10)
pub enum RevealOrigin { Anchor, Center, Edge(EdgeSide) }
pub enum PresentationKind { Fade, Slide { from: EdgeSide, distance: Value<Px> }, Translate { dx: Value<Px>,
                            dy: Value<Px> }, Scale { from: f32 }, Morph, ClipReveal { from: RevealOrigin },
                            Keyed { key: Key, params: Vec<(Key, Value<Scalar>)> }, Instant }   // ADD-A25, A26
pub enum EdgeSide { Top, Bottom, Start, End, TowardAnchor }   // TowardAnchor: WinUI's menus (ADD-A25)
pub struct Loop { pub period: Value<Ms>, pub timing: Value<Timing>, pub keyframes: Vec<Keyframe> }
pub struct Keyframe { pub at: Fraction, pub patch: PartStylePatch }   // `at` a fraction of the period; the part's
                                                        // style, a keyed shape included (R25.5)
```

Semantics (R9): a transition starts when a property's resolved value changes; the transition stated by the style
after the change governs, both directions; a later rule's transition for the same property group wins; appearing
layers interpolate from zero alpha, layers match by stack position, colours interpolate in premultiplied sRGB;
a material giving way to a colour crossfades. A `Geometry` transition also starts when an element's or a part's laid-out box
changes (a switch's thumb moving to its new end), which the connector reports by querying that element, or the part
with its element, with the new box in `ElementData`. During a theme change (S4.3) the theme-change token governs. A
state-triggered icon animation (WinUI's AnimatedIcon, GtkSvg's states, an SF Symbol effect on a change) is a
`TransitionKind::Keyed` transition of the `Icon` group; a repeating one is `IconStyle.animation` (R25.12).

### S2.8 Text style and font (R10)

```rust
pub struct Font {
    pub family: Value<String>,                // fallbacks are the text tokens' (S4.4, ADD-A41)
    pub size: Value<FontSize>,                // today's FontSize (pt or px)
    pub weight: Value<u16>,
    pub style: Value<FontStyle>,
    pub width: Value<f32>,                    // percent of normal, as the OpenType `wdth` axis
    pub axes: Vec<(AxisTag, AxisValue)>,
    pub features: Vec<(FeatureTag, u32)>,
}
pub enum AxisValue { Fixed(f32), FollowsSize }  // SF's and Segoe UI Variable's optical size
pub struct TextStyle {
    pub font: Font,
    pub letter_spacing: Value<Px>, pub word_spacing: Value<Px>,      // word spacing: K3.46
    pub line_height: Value<FontSize>,         // pt or px (scaled with the text-scaling factor), an expression on
                                              // font_size, or `Auto`: the font's own line spacing (ADD-A40)

    pub decoration: Value<Decoration>,                  // may be absent
    pub mnemonic_decoration: Value<Decoration>,         // the mnemonic's underline (ADD-A12)
    pub case: TextCase,                       // ADD-A39
    pub elision: Elision,
    pub wrap: Wrap,
    pub max_lines: Option<u32>,               // wrap to at most n lines, then elide; None: no limit (ADD-A67)
    pub fill: Value<Paint>,
}
pub enum TextCase { Mixed, Upper, Lower, SmallCaps, Capitalize }
pub struct Decoration { pub line: DecorationLine, pub style: DecorationStyle, pub by_word: Value<bool>,
                        pub thickness: Value<Px>, pub offset: Value<Px>, pub paint: Value<Paint> }
pub enum DecorationLine { Underline, Overline, Strike }
pub enum DecorationStyle { Solid, Double, Dotted, Dashed, DashDot, DashDotDot, Wavy }   // AppKit's patterns
                                                                                      // (MAC-LK01)
pub enum Elision { None, End, Middle, Start,
                   Fade { start: Value<Px>, end: Value<Px>, relative: bool, under: Option<FadeUnder> } }
                                                       // a fade at either or both edges (GN-1.129); each may be absent  // relative: a fraction
                                                                                        // of the box (ADD-A38);
                                                                                        // under: None, the text's own
pub enum Wrap { None, Word, Char }
pub enum FadeUnder { Part(PartRef), Element { widget: WidgetRef, variant: Option<VariantRef> } }
                                              // the fade ends under a part, or under a nested element (a
                                              // tab's close button, GN-1.129)
```

The text ramp (S4.4) holds the semantic `TextRole` slots and the keyed platform styles.

### S2.9 Icon style and glyph (R11)

```rust
pub struct IconStyle { pub size: Value<Px>, pub fill: Value<Paint>, pub rendering_mode: RenderingMode,
                       pub weight: Value<u16>, pub scale: Value<Key>,   // SF Symbols' named scales
                       pub axes: Vec<(AxisTag, AxisValue)>,  // Material Symbols' fill, grade, optical size (MAT-X27)
                       pub state_effect: Value<Key>,       // KDE's icon effect for the state the rule selects (K3.60)
                       pub symbol_variants: Vec<Key>,      // SF Symbols' `.fill`, `.circle`, … (R25.8)
                       pub status_colors: Vec<(Key, Value<Paint>)>,   // a symbolic icon's success, warning, error
                                                                       // colours (ADD-A37)
                       pub animation: Value<Loop> }        // a repeating icon animation; may be absent (R25.12)
pub enum RenderingMode { Template, Monochrome, Hierarchical, Palette, Multicolor, Original }
pub struct Glyph { pub source: GlyphSource, pub size: Value<Px>,
                   pub fill: Value<Paint>, pub stroke: Value<GlyphStroke>,   // absent where the glyph is not stroked
                   pub axes: Vec<(AxisTag, AxisValue)>,  // a symbol font's axes (MAT-X27)
                   pub draw_extent: Value<Fraction>,      // the fraction of the path drawn (Breeze's check mark, ADD-A35)
                   pub mirroring: Mirroring }        // Segoe Fluent's per-glyph mirroring (R11)
pub enum GlyphSource {
    FontCodePoint { family: String, code_point: u32 },
    ThemeIcon { name: String, fallbacks: Vec<String> },   // ADD-A36
    Path { data: String },                              // SVG path syntax, from the platform's source code (cited
                                                        // in the preset's comment, S11)
    NamedSystemImage { name: String },                   // a name the platform states for that purpose
    SystemControl { control: SystemControl, replaces: Vec<PartRef> },   // the system's rendering (S15)
}
pub struct GlyphStroke { pub paint: Value<Paint>, pub width: Value<Px>, pub cap: LineCap, pub join: LineJoin }
                                            // a stroked path's outline (K1.184†, ADD-A34)
pub enum Mirroring { Never, Always, ExceptLanguages(Vec<LanguageTag>),
                     SwapWith(IconRole) }   // under right-to-left the role takes another role's icon
                                            // (KStandardAction's Back/Forward, Prior/Next, K-P4-12†)
```

Mirroring per icon role is `token.icons.mirroring`, stated per preset (R11; Material
mirrors some icons in Arabic and Farsi but not Hebrew). Icon lookup takes the element's direction and
language (GTK's `-rtl` and `-ltr` names, SF Symbols' localized forms; R25.8).

### S2.10 Focus indicator (R12)

```rust
pub struct FocusIndicator { pub kind: FocusKind, pub width: Value<Px>, pub paint: Value<Paint>,
                            pub dash: Value<DashPattern>,                  // WinUI's DottedLine (W3-20)
                            pub offset: Sides<Value<Px>>,                 // per side (ADD-A18)
                            pub corners: Corners,
                                                                           // an inner ring is a shadow layer (R7)
                            pub shadow: Value<Shadow>, pub transition: Value<Transition> }
                                                        // `corners`: `FollowPart` (R12's concentric default), or
                                                        // stated where the platform draws otherwise
pub enum FocusVisibleWhen { Keyboard { timeout: Value<Ms> }, AnyFocus, FocusWithin, Never }   // ADD-A16
pub enum FocusKind { Ring, FilledRing, Underline, Keyed(Key) }   // drawn only while the element has
                                                                // `FocusVisible` (R12)  // Keyed: WinUI's Reveal (W3-20); no indicator:
                                                                // the part's `focus` is `Absent`
```

### S2.11 Part style

```rust
pub struct PartStyle {
    pub fill: Value<Paint>,
    pub stroke: Value<Stroke>,
    pub corners: Value<Corners>,
    pub shape: Value<Key>,                   // a keyed shape replacing the corners' outline; may be absent (R25.5)
    pub shadow: Value<Shadow>,
    pub opacity: Value<Fraction>,                 // applied to the element and its descendants as a group
    pub transform: Value<Transform>,         // likewise
    pub text: Value<TextStyle>,
    pub icon: Value<IconStyle>,
    pub glyph: Value<Glyph>,
    pub focus: Value<FocusIndicator>,
    pub padding: Sides<Value<Px>>,
    pub margin: Sides<Value<Px>>,            // outside the box; may be negative (GN-1.18, GN-1.73, GN-1.119)
    pub min_width: Value<Px>, pub min_height: Value<Px>,
    pub max_width: Value<Px>, pub max_height: Value<Px>,
    pub cursor: Value<CursorShape>,
    pub touch_target: Value<Px>,             // the minimum interactive size around the drawn box; layout only,
                                              // never drawn (MAT-X22)
    pub bindings: Vec<Binding>,
    pub transitions: Vec<Transition>,
}
pub enum Binding { Surface(SurfaceSet), Role { role: RoleRef, value: Value<Color> } }
pub enum RoleRef { Surface(SurfaceRole), Set(SurfaceSet, SurfaceRole), Global(GlobalRole), Keyed(Key) }
                                                      // `surface.x`, `role.<set>.x`, `role.global.x`, `role.@x`
```

**Fields per part.** A part has the fields its catalogue entry lists ("the fields that matter"); the derive
generates that set per part, a field outside it is not part of that part (a patch setting it is refused, S12; the
resolved style has it `None`), and a listed field must be stated or stated `Absent` where the model allows (S6.3).
So a window title states no shadow, and nothing is invented for it. "Missing" applies to every leaf of a listed
field, its plain enum and bool leaves included (a listed `text` states its `case`, `wrap`, `elision`, …). A widget's first part
in the catalogue is its root part; the root part's bindings apply to all the widget's parts and its descendants
(R5.2), and the other parts' bindings to themselves and their descendants. Every part of a widget is queried with the
widget's element (its states, siblings and scope, S8); the query applies the root part's resolved bindings before
it evaluates any other part. Parts nest as the catalogue nests them (a window's title inside its title bar); a part's bindings, opacity and
transform apply to the parts nested in it and their elements. An element part's query sees its widget's variants
as well as its own (a list row matches the list's `sidebar` variant). An *(element)* part nested in a plain part is queried in the root's
scope with the plain part's bindings, opacity and transform applied. A part marked *(element)* in the catalogue (a
caption button, a tab) is an element of
its own, queried in the scope of the element that contains it (the root part's, or an enclosing *(element)* part's:
a caption button inside the caption-button group) with its own states, and its sub-parts are queried with it. A block-typed field set to a reference
holds the referenced token (S2.2). `CursorShape` is CSS's
cursor keywords (CSS Basic UI 4: `Default`, `ContextMenu`, `Help`, `Pointer`, `Progress`, `Wait`, `Cell`,
`Crosshair`, `Text`, `VerticalText`, `Alias`, `Copy`, `Move`, `NoDrop`, `NotAllowed`, `Grab`, `Grabbing`,
`AllScroll`, `ColResize`, `RowResize`, `NResize`, `EResize`, `SResize`, `WResize`, `NeResize`, `NwResize`,
`SeResize`, `SwResize`, `EwResize`, `NsResize`, `NeswResize`, `NwseResize`, `ZoomIn`, `ZoomOut`), which GTK uses by
name and Qt, WinUI and AppKit map onto, plus `WhatsThis` (Qt's) and `None`; a cursor `Absent` sets none, so the
window's stays (ADD-A59). Every widget's "when focus becomes visible" value is a
`focus_visible_when: Value<FocusVisibleWhen>`, whose base each preset states, usually as
`= token.behavior.focus_visible_when` (a reference, not inheritance, R20; ADD-A16).

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
    pub selection_text_background: Value<Color>,               // selected text; may be absent (ADD-A48)
    pub selection_text_foreground: Value<Color>,
    pub selection_text_background_unfocused: Value<Color>,     // an unfocused field's selection (ADD-A49)
    pub selection_text_foreground_unfocused: Value<Color>,     // macOS's unemphasized text colour (MAC-I05)
    pub find_highlight: Value<Color>, pub drop_highlight: Value<Color>,
    pub scrim: Value<Color>,
}
pub struct ColorRoles {
    pub surfaces: SurfaceMap<SurfaceRoles>,
    pub global: GlobalRoles,
    pub keyed: Vec<(Key, Value<Color>)>,                       // Material's ladder, WinUI's tiers
}
```

Selected items (list rows, cells) take the `Selection` surface set's roles; selected text takes the text-selection
roles, which a preset whose platform does not distinguish them states as references to the `Selection` set (R12).
The inactive window's selection is a rule under `WindowInactive`, not a role. KDE's 3-D shades (Qt's Light, Midlight,
Dark, Mid, Shadow) are keyed roles computed from Window (`role.@shade_light` …), as KDE computes them (ADD-A48).
WinUI's attention, success, caution and critical map to `info`, `success`, `warning`, `error`.

A preset declares the surface sets it has (S10); `Window` and `Selection` are declared by every preset: `surface.` falls back
to `Window`, and selected items take `Selection`'s roles.

**S3.1 Bases.** `colors.light` and `colors.dark` are the two bases (R5.1). Every other condition changes roles
by colour-role rules (S7).

**S3.2 Groups.** The inactive group is a colour-role rule under `window_inactive`; disabled roles are
`SurfaceRoles.disabled`, which parts' `{disabled}` rules refer to (R5.1).

**S3.3 Bindings and scope.** `PartStyle.bindings` bind a surface set or a single role for the element and its
descendants (the root part's for the whole widget, S2.11). Every role reference resolves against the nearest binding
of that role in scope (the element's own, then its ancestors' as its `Scope` carries them, S8), else the root's:
In two steps. (1) `surface.x`: walk outward; the first element with a `Role { Surface(x) }` binding or a
`Surface(S)` binding decides; a `Surface(S)` answer becomes the lookup `role.S.x`, continued from that element
outward, and with no binding at all it is `role.window.x`. (2) `role.S.x` (and `role.global.x`, `role.@x`): walk
outward; the first `Role` binding of exactly that role decides, else the colour roles' value. A `Role { Surface(x) }`
binding never answers step 2. A binding's
value is evaluated in the scope outside it (R5.2). Colour-role definitions do not use `surface.`: a set's roles name
`role.<set>.<role>` explicitly, so a role means the same wherever it is used (S12). `ResolvedTheme::role` returns
`Option<Color>`, `None` for a role that is not drawn.

## S4. Tokens

```rust
pub struct Tokens {
    pub motion: MotionTokens,              // durations, timings (keyed), the speed factor
    pub theme_change: ThemeChange,
    pub elevation: Vec<(Key, Vec<ShadowLayer>)>,   // the platform's ramp, or empty (R8)
    pub text: TextTokens,
    pub icons: IconTokens,                 // S4.5
    pub focus: FocusIndicator,             // the platform's indicator (R12)
    pub caret: Caret,
    pub behavior: Behavior,
    pub size_classes: Vec<(SizeClass, SizeClassThreshold)>,   // where the platform states widths (R16)
    pub params: Vec<(Key, Vec<(Key, Value<Scalar>)>)>,       // keyed scalar groups (S4.6)
    pub pointer: PointerTokens,                              // S4.7
    pub kde: Value<KdeTokens>,                               // `kde-breeze` only (S4.8)
    pub material: Value<MaterialScheme>,                     // `material-3` only (S4.8)
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

`MotionTokens { durations: Vec<(Key, Value<Ms>)>, timings: Vec<(Key, Timing)>, speed_factor: Value<f32> }`. A
scaled duration is stated as `= mul(token.motion.durations.x, token.motion.speed_factor)` (R9); a spring scales by
its `time_scale` (S2.7). Map keys (keyed durations, params) carry no unit suffix: their values are plain numbers in
the unit their declaration names (S10).

### S4.2 Caret and behaviour (R12, R17)

```rust
pub struct Caret { pub color: Value<Color>, pub width: Value<Px>, pub shape: CaretShape,
                   pub blink_on: Value<Ms>, pub blink_off: Value<Ms>, pub blink_timeout: Value<Ms>,
                   pub blink_style: BlinkStyle,                                         // ADD-A24
                   pub secondary: Value<Color>,          // the bidirectional secondary caret; may be absent
                   pub shown_in_read_only: Value<bool>,
                   pub solid_while_typing: Value<bool> }                                // X-CA-2
pub enum BlinkStyle { Hard, Fade }
pub struct Behavior { pub double_click_time: Value<Ms>, pub double_click_distance: Value<Px>,
                       pub drag_threshold: Value<Px>, pub hover_open_delay: Value<Ms>,
                       pub key_repeat_delay: Value<Ms>, pub key_repeat_interval: Value<Ms>,
                       pub scroll_lines_per_step: Value<f32>, pub scroll_px_per_step: Value<Px>,  // MAT-X32†
                       pub hide_pointer_while_typing: Value<bool>, pub hide_pointer_delay: Value<Ms>,  // K3.65†
                       pub hide_pointer_fade: Value<Ms>,                                     // MAT-P3-15†
                       pub scroll: ScrollPhysics,
                       pub long_press: Value<Ms>, pub spring_loading: Value<Ms>,       // ADD-A45
                       pub drag_time: Value<Ms>, pub single_click_activates: Value<bool>,
                       pub double_click_height: Value<Px>,      // with double_click_distance as the width
                       pub touch: TouchBehavior,
                       pub mnemonics: Mnemonics,                 // ADD-A12
                       pub focus_visible_when: Value<FocusVisibleWhen> }
pub struct TouchBehavior { pub slop: Value<Px>, pub paging_slop: Value<Px>, pub double_tap_time: Value<Ms>,
                            pub double_tap_slop: Value<Px>, pub delay: Value<Ms> }
pub enum Mnemonics { Never, OnAlt { delay: Value<Ms> }, Always }
pub struct ScrollPhysics { pub deceleration: Value<f32>, pub overscroll: Value<Timing>,
                           pub overscroll_effect: OverscrollEffect, pub overscroll_extent: Value<Px>,  // ADD-A29
                           pub min_fling_velocity: Value<f32>, pub max_fling_velocity: Value<f32>,   // px/s (MAT-P3-16†)
                           pub wheel_step_duration: Value<Ms>, pub wheel_step_duration_max: Value<Ms>,
                           pub touchpad_factor: Value<f32>,          // touchpad deltas' multiplier (GN-P3-15†)
                           pub touchpad_deceleration: Value<f32>, pub touchpad_timing: Value<Timing>,
                           pub touchpad_velocity_samples: Value<f32>,   // touchpad inertia: deceleration, curve,
                                                                     // and the deltas its start velocity is
                                                                     // estimated from (K-P3-18†)
                           pub wheel_step_timing: Value<Timing>,     // the duration grows with the distance
                                                                     // up to the maximum (K-P3-18†)
                                                                 // smooth wheel scrolling (MAC-P4-4†)
                           pub snap: Value<bool> }
pub enum OverscrollEffect { None, RubberBand, Stretch, Glow }   // None: the platform draws none
```

The caret is also a part (`caret`) of the text widgets, whose base refers to `token.caret`, so rules apply to it by
widget, drop target and inactive window; the text widgets' further parts (preedit, misspelling, selection handles,
…) and the shared `overwrite` and `composing` flags are the catalogue's; hiding while text is selected and the
overwrite block caret are those parts' rules (ADD-A23, ADD-A24).

Widget-specific timings (tooltip delays, submenu delay and sloppy close, scroll-bar auto-hide and trough click,
toast duration, focus-visible rule) are widget values in the catalogue (R17).

### S4.3 Theme change (R9)

```rust
pub enum ThemeChange {
    Crossfade { duration: Value<Ms>, timing: Value<Timing> },
    Suspend { duration: Value<Ms> },         // suspends the transitions marked `suspended_by_theme_change`
    Instant,
}
```

A theme change is a re-resolution caused by a change of the preset, a layer, an input or a reader value, or of one
of these conditions: `Scheme`, `ContrastIncreased`, `ContrastHigh` (R9); any other condition change runs the
property transitions of the styles it changes. A transition already running when a suspension starts finishes.

### S4.4 Text ramp

```rust
pub struct TextRoles {                        // one field per semantic slot, each filled by every preset (R10)
    pub body: Value<TextStyle>, pub body_emphasized: Value<TextStyle>,
    pub caption: Value<TextStyle>, pub caption_emphasized: Value<TextStyle>, pub caption2: Value<TextStyle>,
    pub footnote: Value<TextStyle>, pub subheadline: Value<TextStyle>, pub callout: Value<TextStyle>,
    pub headline: Value<TextStyle>, pub title1: Value<TextStyle>, pub title2: Value<TextStyle>,
    pub title3: Value<TextStyle>, pub large_title: Value<TextStyle>, pub display_large: Value<TextStyle>,
    pub display_medium: Value<TextStyle>, pub display_small: Value<TextStyle>, pub label: Value<TextStyle>,
    pub monospace: Value<TextStyle>, pub numeric: Value<TextStyle>,
}
pub struct TextTokens { pub roles: TextRoles, pub keyed: Vec<(Key, TextStyle)>,
                        pub fallbacks: FontFallbacks, pub rendering: TextRendering, pub scaling: TextScaling }
pub enum TextScaling { Linear, AndroidNonLinear, DynamicType }   // S9.4
pub struct FontFallbacks { pub families: Vec<String>, pub emoji_family: Value<String>,
                           pub per_language: Vec<(LanguageTag, Vec<String>)>,
                           pub per_family: Vec<(String, Vec<String>)> }   // Core Text's per-font cascade lists
```

The nineteen slots are closed (ADD-A42); a slot is a reference to the platform's keyed style
(`= token.text.keyed.body`), so a style is stated once. Font fallbacks are the platform's one
chain (per family where the platform keeps one per font), so they are a token and not a field of each font (ADD-A41).

### S4.5 Icon sizes

```rust
pub struct IconTokens { pub set: Value<IconSet>, pub theme: Value<String>,    // today's `icon_set`, `icon_theme`
                        pub theme_fallback: Value<String>,     // may be absent
                        pub sizes: IconSizeTokens, pub mirroring: Vec<(IconRole, Mirroring)> }
pub struct IconSizeTokens { pub toolbar: Value<IconStyle>, pub small: Value<IconStyle>, pub large: Value<IconStyle>,
                            pub dialog: Value<IconStyle>, pub panel: Value<IconStyle>,
                            pub keyed: Vec<(Key, IconStyle)> }
```

Each context is an icon style (size, weight, scale, rendering mode), the platform's further contexts as keys
(ADD-A43). `theme_fallback` is the theme the platform's own lookup falls back to before the system fallback
(`KirigamiApp` makes Breeze the fallback of a Plasma application, K-KA-47†): the platform's lookup chain, as a KDE
application resolves it, not a mixing of sets.

### S4.6 Keyed scalars

`Tokens.params` holds keyed groups of scalars, referred to as `token.params.<group>.<key>` and declared per preset
(`params = { <group> = { unit = "px", keys = [...] } }`, S10, each group one unit): `layout` (Kirigami's grid unit and spacings, Qt style metrics), `opacity` (disabled, dim,
…; restated by `Tokens` rules under `ContrastHigh`), `shadow`, `motion`, `color` (KDE's contrast and frame
contrast), `icon_effect` (KDE's icon-effect amounts, K3.60, Q3.18), `caret`, `shape`, `display` (`corner_radius`, the measured display radius per device model, set by rules
under `DeviceModel`, R25.11; for a model the preset does not list it is `Absent`, and concentric corners then fall
back to their stated minimum) (ADD-A44). The display is the root's container: a full-screen container's
`Radius::Concentric` corners follow the display's radius, which is the connector's `display_corner_radius` where the
platform reports it, else this token, else none, and then each corner its stated `minimum`. The four corners are in
the order top-left, top-right, bottom-right, bottom-left.

### S4.7 Pointer

`PointerTokens { theme: Value<String>, size: Value<Px>, color: Value<Color>, drag_cursors: Vec<(Key, String)> }`,
read where the platform reports them (ADD-A46).

### S4.8 Platform scheme tokens

```rust
pub struct KdeTokens {                        // KDE's colour effects (K3.40, K3.50) and scheme options (R25.1)
    pub disabled: KdeEffects, pub inactive: KdeEffects,
    pub inactive_enabled: Value<bool>, pub inactive_change_selection_color: Value<bool>,
    pub tint_factor: Value<Fraction>,              // absent: the scheme does not tint
    pub titlebar_is_accent_colored: Value<bool>,
}
pub struct KdeEffects {
    pub intensity: KdeIntensityEffect, pub intensity_amount: Value<Fraction>,
    pub color: KdeColorEffect, pub color_amount: Value<Fraction>, pub color_color: Value<Color>,
    pub contrast: KdeContrastEffect, pub contrast_amount: Value<Fraction>,
}
pub enum KdeIntensityEffect { None, Shade, Darken, Lighten }   // None: KDE's "no effect" setting
pub enum KdeColorEffect { None, Desaturate, Fade, Tint }
pub enum KdeContrastEffect { None, Fade, Tint }
pub struct MaterialScheme { pub variant: MaterialVariant, pub spec_version: Key,
                           pub contrast_level: Value<f32> }   // a base, and `Tokens` rules under the contrast
                                                              // conditions (R5.1); `m3_role` reads it
pub enum MaterialVariant { TonalSpot, Neutral, Vibrant, Expressive, Fidelity, Content, Monochrome, Rainbow,
                           FruitSalad }
```

The `kde_*` functions' effect parameters are these enums. A palette replacing the reader's colours replaces
`token.kde` too (S14).

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

**S5.2 Window.** `WindowTheme` has the parts the catalogue lists, among them `frame`, `title_bar`, `title`,
`caption_button` (one part type; each caption button an element whose identity is a typed variant: `close`,
`minimize`, `maximize`, `restore`, `window_menu`, and keyed ones where one platform has them, KDE's `@help`), and widget values for the
caption buttons' layout (R13).

## S6. The derive macro and the model's metadata

**S6.1** `#[derive(ThemeWidget)]` is replaced by `#[derive(ThemeParts)]`, which generates, per widget struct: its
patch type (every field `Option<…>`, where `None` is "this patch does not set it"; paints with `Set`/`Append`), its
resolved type (every `Value<T>` resolved to `Option<T>`, where `None` is "not drawn", possible only where
`may_be_absent` allows; a length field resolves to `Option<Length>`, `Length = Px(f32) | Auto`, so "sized by
content" reaches the toolkit), the field-path registry used by the TOML linter and the facts gate, and the
`may_be_absent` table.

```rust
pub enum Patch { Colors(ColorRolesPatch), Tokens(TokensPatch), Part(PartPatch), Values(ValuesPatch) }
pub enum PartPatch { Style(PartStylePatch), Absent }   // `Absent`: the whole part is not drawn (S6.3)
pub enum ValuesPatch { Typed(WidgetValuesPatch), Component(ComponentValuesPatch) }
// WidgetValuesPatch: a derive-generated enum with one variant per typed widget, holding <W>ValuesPatch.
// Target::Colors takes Patch::Colors, Tokens takes Tokens, Part takes Part, Values takes Values (S12).
```

**Compound fields** (`Sides`, `Axes`, and block fields that are not `Value`s, `FocusIndicator.corners`) patch per
sub-field (`Sides<Option<Value<Px>>>`), with the same table spelling as a `Fields` patch.

**Merging patches:** a field a later patch sets replaces the earlier value; a map (`Vec<(Key, …)>`, the bindings by
role or surface, the presentations by event, the transitions by property group, a transition listing several groups
being split per group first; every `Vec` of pairs) merges by key, the later entry replacing the earlier; any other list (`stops`, `layers`
set with `Set`, `symbol_variants`, `fallbacks`) is replaced whole; `Append` adds paint layers over `base` (R6), and
over an `Absent` base gives a paint of the appended layers; `Append` in a layer's bases is an error, as `base` is.

**S6.2** Inheritance code goes: `inherit_from`, `theme_inherit`, the `[uniform]`, `[per_platform]` and
`[wrong_safety_nets]` registries and `resolve/inheritance.rs` are removed (R20). Their rules are converted once
into the presets' bases as expressions (S19.2).

**S6.3** `Absent` is allowed from two sources: a block's sub-field this document marks "may be absent" (S2),
wherever that block appears; and the catalogue's tables, for colour roles, tokens and, per (widget, part, field
path), at three levels: a
field of a part (`button.body.shadow`), a whole part (`slider.ticks`: some platform draws no ticks), and a whole
widget (a preset declares it `available = false`, S10). A whole part is made absent by a base or a rule
(`PartPatch::Absent`, TOML `absent = true`), so it may be absent under a condition only (a title bar in full
screen); the query then answers `NotDrawn`. An unavailable widget and an absent part need no fields. A compound widget value listed `may_be_absent` (a `Placement`, a record of `Value`s) allows each of its `Value`
members to be absent. A widget's root
part is never absent as a whole and is never a repeated element (the catalogue gives such widgets a container root);
its fields may be. `may_be_absent` is about what a platform draws, never about an element's content: a part the
element's content lacks (a dialog given no icon, an icon-only tab's label) is simply not queried by the connector,
and the catalogue does not list it. A `Style` patch after a `PartPatch::Absent` brings the part back with the fields stated below the
`Absent`. Likewise `Auto` is allowed only on the length fields the catalogue marks `auto` (sizes, widths, heights,
gaps, and `TextStyle.line_height`), which resolve to `Option<Length>`; every other length resolves to `Option<Px>`.

**May be absent, in the core's blocks** (the first source; wherever the block appears, because some platform has no
counterpart, R6, R20): every `Material` parameter but `identity` and `fallback`; `Ripple`'s values; a
`FocusIndicator`'s `dash`, `shadow` and `transition`; `IconStyle.state_effect`, `weight`, `scale`, `axes`,
`animation`, `status_colors`; `Glyph.stroke`, `draw_extent`, `axes`; `TextStyle.decoration`,
`mnemonic_decoration`, `letter_spacing`, `word_spacing`; `Decoration.by_word`; `Elision::Fade`'s `start` and `end`;
an `Edge`'s `width` and `paint` (an edge not drawn); `Font.width`; every `TextRendering` field; `Stroke.outset`;
`Edge.dash`; `PartStyle.shape`, `touch_target`, `margin`; `Caret.secondary`, `shown_in_read_only`,
`solid_while_typing`; every `Behavior`, `TouchBehavior`, `ScrollPhysics` and `PointerTokens` field; every
`KdeTokens` field; `MaterialScheme.contrast_level`; `IconTokens.theme_fallback`; a spring's `Transition.duration`.
The catalogue's tables are the second source; they cite these marks rather than repeat them. The derive generates the tables from the catalogue; `Absent` anywhere else is an error when the
layer is added (S12).

## S7. Rules and selectors (R2.3)

```rust
pub struct Rule { pub selector: Selector, pub target: Target, pub patch: Patch }
pub enum Target { Colors, Tokens, Part(PartRef), Values(WidgetRef) }
pub enum WidgetRef { Typed(WidgetKind), Component(Key) }
pub struct Selector {
    pub conditions: SetMatch<Condition>,
    pub context: Vec<ContextMatch>,           // every one must match
    pub not_context: Vec<ContextMatch>,       // none may match
    pub position: Option<PositionMatch>,
    pub variants: SetMatch<VariantRef>,
    pub size: SizeMatch,
    pub direction: Option<Direction>,
    pub states: SetMatch<StateFlag>,
}
pub enum VariantRef { Typed(Variant), Keyed(Key) }   // Variant: the catalogue's typed variants, one enum
pub struct SetMatch<T> { pub all_of: Vec<T>, pub none_of: Vec<T> }
pub struct ContextMatch { pub scope: ContextScope, pub widget: Option<WidgetRef>,   // None: any widget (R25.9)
                          pub part: Option<PartRef>,
                          pub sizes: SetMatch<ControlSize>,    // the ancestor's control size (MAC-MN15)
                          pub surface: Option<SurfaceSet>,     // the ancestor binds this surface set (Q1.37,
                                                               // ADD-A68); None in any of these: not matched on
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
pub struct SizeMatch { pub sizes: SetMatch<ControlSize>, pub densities: SetMatch<Density>,
                       pub size_classes: SizeClassMatch }
pub struct SizeClassMatch { pub horizontal: SetMatch<SizeClass>, pub vertical: SetMatch<SizeClass> }
pub enum Direction { Ltr, Rtl }
```

A rule targeting `Colors` or `Tokens` may only have `conditions` (S12 rejects others).

### S7.1 Conditions

```rust
pub enum Condition {
    // system, by the reader (R2.3)
    Scheme(Scheme), ContrastIncreased, ContrastHigh, ReducedTransparency, ReducedMotion, AnimationsOff,
    BoldText, ButtonShapes, DifferentiateWithoutColor, TabletMode, MobileFormFactor, BatterySaver,
    ContainerSize { container: WidgetRef, size: Key },     // macOS's sidebar icon size
    OverlayScrollbars, OnOffLabels, FullKeyboardAccess, PreferCrossFade, AccessibilityTextSize,
    Idiom(Idiom), SystemSetting(Key), DeviceModel(Key),    // a platform switch the reader reads, declared (S10);
                                                           // the device model where a measured value differs per
                                                           // model (the display radius, W-6, X-CR-4)
    // window, by the connector
    WindowInactive, NotMain, AppInactive, Maximized, FullScreen, Tiled(TiledEdges),
    Elevated, Input(InputKind),                            // the kind of the last input (Kirigami's transient
                                                           // touch, WinUI's input kind; R2.3)
    NotComposited, Shaded, AttentionFlash, TintDimmed, MnemonicsVisible, Freeform,   // Freeform: Android's desktop
                                                           // windowing (MAT-W5†)
    Posture(Posture),                                      // MAT-P3-13†
    UnitScale,                                             // device pixel ratio 1 (Qt Quick's text rendering, Q3.32)
    // application
    Flavor(Key), Preset(PresetId),
}
pub enum Scheme { Light, Dark }
pub enum InputKind { Mouse, Touch, Pen, GameController }
pub enum Idiom { Phone, Pad }                          // what the rows need (ADD-A09)
pub enum Posture { Flat, Tabletop, Book }              // a foldable's posture (Jetpack WindowManager)
pub struct TiledEdges { pub top: bool, pub right: bool, pub bottom: bool, pub left: bool }
```

`Tiled(e)` holds when the window's tiled edges include every edge `e` names (a window maximised along one axis is
tiled to those two edges, R13); every other parametrised condition holds on equality.

The list is closed by the catalogue and the traceability: a condition a row needs that is not here is added here.
The additions and their rows: ADD-A01 … ADD-A12, ADD-A76, ADD-A83 (S22). `OverlayScrollbars` replaces `scrollbar.overlay_mode`.
The readers report the system conditions directly (R2.3; today's `AccessibilityPreferences` goes, S16);
`text_scaling_factor` stays a reader value (S9.4). `MobileFormFactor` is Plasma Mobile's shell, which is not a device
idiom; `Idiom` is UIKit's and Material's phone and pad.

### S7.2 State flags (R3)

```rust
pub enum StateFlag { Hover, Pressed, Focused, FocusWithin, FocusVisible, Disabled, Checked, Selected,
                     Indeterminate, Open, Default, Dragged, DropTarget, ReadOnly, Invalid, Visited,
                     Autofilled, Scrolled, OverDarkContent, OpenedByTouch, Overflowing, Redacted,
                     Shared(SharedFlag), Widget(Key) }
pub enum SharedFlag { Empty, InFill, Today, OtherMonth, Current, Multiline, Filled, Half, Preview, Placeholder,
                      Pulling, Refreshing, PlacedAbove, PlacedBelow, RouteActive, Editing, Swiped, ContextMenuOpen,
                      Sorted, Descending, Overwrite, Composing, Minimized,
                      TextSelected }   // a text widget holding a selection (the caret hides, GN-3.73†)
                      // flags two or more platforms draw on a typed widget (ADD-A15); the catalogue's
                      // *Shared flags* table names the widgets that declare each
```

`Selected` is "the current choice among siblings" (a list row, a tab, a segment, a combo list's chosen value); a
menu item's keyboard or pointer highlight is `Focused` or `Hover`, never `Selected`; `Checked` is "toggled on" (a check
box, a toggle button, a menu item's check). A connector sets them by that meaning on every platform. `Shared` flags have
one name for a portable application: the list is exactly the catalogue's flags that two or
more platforms draw, and the catalogue says which widgets declare each.
`Widget(Key)` is a one-platform flag the preset declares (S10). Exclusive groups: S10.

### S7.3 Sizes, density, size classes (R15, R16)

```rust
pub enum ControlSize { ExtraSmall, Mini, Small, Regular, Large, ExtraLarge }
pub enum Density { Default, Compact, Keyed(Key) }      // Material's density steps keyed
pub enum SizeClass { Compact, Medium, Expanded, Large, ExtraLarge, Regular }   // closed by the catalogue
pub struct Breakpoint { pub measured: Measured, pub width: Option<(Comparison, Value<Px>)>,
                        pub height: Option<(Comparison, Value<Px>)>, pub combine: Combine,
                        pub variant: VariantRef }      // R16; the platform's own conditions, not inverted
pub enum Comparison { AtLeast, AtMost, Above, Below }  // ≥, ≤, >, < (AdwDialog: width ≤ or height ≤)
pub enum Combine { All, Any }                          // both stated sizes must hold, or either
pub enum Measured { Allocation, Window }               // the size the parent offers the element (libadwaita's
                                                       // breakpoint bin), or the window's: never the element's own
                                                       // laid-out size, so a variant that changes padding does not
                                                       // feed back into its own breakpoint
pub struct SizeClasses { pub horizontal: Option<SizeClass>, pub vertical: Option<SizeClass> }   // R25.10; None:
                                                                     // the platform states no class on that axis
```

Each preset's facts state which of its sizes fills which `ControlSize` slot (Material Expressive's five button sizes,
AppKit's four control sizes, five on macOS 26); a preset declares only the slots it fills (S10).

## S8. Layers, order and resolution (R2.4, R2.5)

```rust
pub struct Layer { pub bases: Bases, pub for_preset: Vec<(PresetId, Bases)>,  // an application layer;
                   pub rules: Vec<Rule> }             // Layer::from_toml (S1); a preset's own bases apply after the
                                                      // layer's common bases while that preset is current, and are
                                                      // validated only then (S11's `[for_preset.<id>]`)
pub struct Bases { pub colors: SchemeMap<ColorRolesPatch>, pub inputs: InputBases, pub tokens: TokensPatch,
                   pub widgets: WidgetsPatch,              // derive-generated: per widget, parts and values
                   pub components: Vec<(Key, ComponentBase)> }
pub struct ReaderState {                                   // the system conditions a reader reports (R2.3, S7.1)
    pub scheme: Scheme, pub contrast: Option<Contrast>, pub reduced_transparency: bool, pub reduced_motion: bool,
    pub animations_off: bool, pub bold_text: bool, pub button_shapes: bool, pub differentiate_without_color: bool,
    pub tablet_mode: bool, pub mobile_form_factor: bool, pub battery_saver: bool, pub overlay_scrollbars: bool,
    pub on_off_labels: bool, pub full_keyboard_access: bool, pub prefer_cross_fade: bool,
    pub idiom: Option<Idiom>, pub device_model: Option<Key>,   // AccessibilityTextSize: from the category
    pub container_sizes: Vec<(WidgetRef, Key)>, pub system_settings: Vec<Key>,
    pub text_scaling_factor: f32,                          // S9.4
    pub content_size_category: Option<Key>,                // iOS's, for `uifm_scaled`; None elsewhere
}
pub struct WindowConditions {                              // a window's conditions, by the connector
    pub inactive: bool, pub not_main: bool, pub app_inactive: bool, pub maximized: bool, pub full_screen: bool,
    pub tiled: TiledEdges, pub elevated: bool, pub input: InputKind, pub not_composited: bool, pub shaded: bool,
    pub attention_flash: bool, pub tint_dimmed: bool, pub mnemonics_visible: bool, pub freeform: bool,
    pub posture: Option<Posture>, pub unit_scale: bool,
}
pub struct RootInputs { pub density: Option<Density>, pub size_classes: SizeClasses, pub direction: Direction,
                        pub language: Option<LanguageTag>, pub window: Size, pub screen: Size,  // the work area
                        pub display_corner_radius: Option<[Px; 4]> }   // where the platform reports it (Android,
                                                                       // MAT-W5†); see S4.6
```

Typed structs make a condition from the wrong source unrepresentable: a connector cannot state the scheme, nor a
reader a window's state. The application's conditions (flavour, the scheme and the contrast it picks, R2.3, R25.2)
come from `Theme`'s setters, and win over the reader's.

**Layers and order.** The stack is: the preset; the reader, or the stand-in an application without a reader sets
(`set_reader_stand_in`, R2.6; refused while a reader exists, `Err(ReaderPresent)`); then the application's layers in
the order it adds them (R2.4): its own layers, the accent layer `set_accent` writes and the palette `apply_palette`
adds, each staying where it was first added when it is replaced. Every layer's bases apply in that order, then every
layer's rules in that order. While a palette applies, the reader's colour bases, colour rules, colour inputs and
`token.kde` are left out; the palette does not apply while `ContrastIncreased` or `ContrastHigh` holds unless it
states rules for that condition, and the reader's colours then apply as without it (R19).

**Validation** (S12) runs on every change of the stack (S16): it validates the whole stack, conservatively (every
rule and binding an expression can meet, whatever conditions hold together, R2.8), and a change that fails is refused
whole, leaving the theme as it was (R2.5). So resolution and the query cannot fail. A reader's re-read that fails keeps
the previous reader layer; `Theme::reader_status()` reports the error.

**Windows and resolution.** Each window's connector owns a `WindowState` (`Default`: never resolved), and
`Theme::resolve(&self, window: &mut WindowState, conditions: &WindowConditions) -> Resolution` resolves for it
(infallible), with `Resolution { theme: ResolvedTheme, change: Change }`, `Change = First | Unchanged | ThemeChange |
ConditionChange` relative to that window's last resolution, which `WindowState` records (a theme change wins when
both happened; a flavour switch is a condition change; S4.3). After any change but `Unchanged` a connector rebuilds the
scopes it kept, whose bound colours are the old resolution's (an inactive window changes roles too):

1. merge bases in order; for colours, the base of the conditions' scheme; for each input, its per-contrast base
   where a contrast condition holds (S9);
2. apply the `Colors` and `Tokens` rules whose conditions hold, in order (choosing expressions, not evaluating);
3. for each part and widget, keep its base and its rules whose conditions hold, in order.

A window resolves again on any change of its conditions, an input, a reader value, a layer, the flavour or the
preset.

**Query.** `ResolvedTheme::root_scope(&RootInputs) -> Scope` gives the window's root scope. Every query takes its
parent's scope and returns its own, so a connector queries top-down, as it draws. A `Scope` holds owned, evaluated
values only (the ancestor chain, each ancestor's widget, part, variants, states and control size, which context
selectors match; the bound colours; density, size classes, direction, language; the window's and screen's size), so a scope kept across a re-resolution is safe to use but stale; the connector rebuilds it (above).
Tokens and roles are read in a scope too, for what no part carries (behaviour timings, the caret, text rendering, the
pointer, the window's background).

```rust
pub enum Styled<T> { Style(T), NotDrawn, NotAvailable(Unavailable) }   // R2.7's answer; not an error
pub enum Unavailable { Widget, Size, Variant, Part }
impl ResolvedTheme {
    pub fn root_scope(&self, root: &RootInputs) -> Scope;
    pub fn style(&self, element: &Element) -> Styled<(ResolvedPartStyle, Scope)>;  // the scope for its children
    pub fn values<W: Widget>(&self, element: &Element) -> Styled<W::Values>;       // another widget's element:
                                                                                   // NotAvailable(Widget)
    pub fn component_values(&self, element: &Element) -> Styled<ComponentValues>;
    pub fn tokens(&self, scope: &Scope) -> ResolvedTokens;
    pub fn role(&self, scope: &Scope, role: RoleRef) -> Option<Color>;             // None: not drawn
    pub fn size_classes(&self, window: Size) -> SizeClasses;                        // from the thresholds (R16)
    pub fn data_needs(&self, part: PartRef) -> DataNeeds;   // the element data and metrics its values read
}
pub struct Element<'a> {                                   // Element::new(part, scope) and setters
    pub part: PartRef,                                     // names the widget too (S1)
    pub scope: &'a Scope,                                  // the parent's, or the root's
    pub variants: &'a [VariantRef], pub size: Option<ControlSize>,      // None: the declared default size
    pub density: Option<Density>, pub direction: Option<Direction>,     // the element's own, if it states one,
    pub size_classes: Option<SizeClasses>,                              // passed to its subtree (R16)
    pub language: Option<LanguageTag>,                                  // for icon lookup (S2.9)
    pub states: StateFlags,
    pub siblings: &'a [Sibling<'a>], pub index: usize,                  // all siblings, itself at `index`
    pub data: ElementData<'a>,
    pub metrics: &'a dyn FontMetrics,                      // the toolkit's measurement
}
pub struct Scope { /* owned: see above */ }
pub trait FontMetrics { fn char_width(&self, font: &ResolvedFont) -> Px; fn x_height(&self, font: &ResolvedFont)
                        -> Px; fn height(&self, font: &ResolvedFont) -> Px; }   // infallible
pub struct Sibling<'a> { pub part: PartRef, pub variants: &'a [VariantRef], pub states: StateFlags }
pub struct ElementData<'a> {                               // R25.3; Default: zero size, progress 0, scroll 0,
    pub width: Px, pub height: Px, pub progress: Fraction, // item 0 of 1, empty text, transparent
    pub scroll_offset: Px, pub scroll_progress: Fraction,
    pub item_index: u32, pub item_count: u32, pub text: &'a str,
    pub color: Color,                                      // a colour the connector supplies that derived values
                                                           // follow: the application's choice (a badge), or the
                                                           // text's colour under a caret (K-P3-11†)
    pub move_distance: Px,                                 // how far a Move presentation moves it (K1.133†)
}
```

The operands `element.window_width`, `window_height`, `screen_width` and `screen_height` (Q2.50, Q1.60, K-P3-7†)
come from the root's inputs through the scope; the font metrics from `Element.metrics`; the others from
`Element.data`.

The query: answers `NotAvailable` for an undeclared size or variant, an unavailable widget, a missing required
variant, or a part the widget does not have, and `NotDrawn` for a part whose style resolved `Absent` (the children of
a `NotDrawn` or `NotAvailable` element take the parent's scope); drops excluded flags by the exclusive groups (S10),
and ignores a density the widget does not declare (R15); takes density, size classes, direction, language and
bindings from the scope, the element's own overriding them; applies the part's kept rules in order, skipping those
that do not match (position counted among the siblings the rule's `among` names; `SameKind` siblings are those of
the queried element's widget and part; an `index` outside `siblings` is treated as having none); evaluates
expressions (S2.2) with `base`, the element data, `part.`, `widget.`, `values.`, the font metrics and the bindings
in scope; and returns the part's style with the scope its children receive. A `part.corners.<corner>` operand
resolves `Full` to half the smaller of `element.width` and `element.height` (the part declares that data) and is
refused on `Concentric` (S12). `Element.variants` includes the variants the connector sets from the element's content
(`icon_only`, `with_icon`; GTK sets `.image-button` itself, ADD-A61).

The query cannot fail (R2.8): element data are plain values. Each part declares the data its values read (S10,
`data = [...]`, the font metrics included; a widget value that reads element data, a presentation step's duration
over `move_distance`, is declared under `values`); `data_needs` tells the connector what to supply for a part, as it
supplies the element's states. Expressions over `element.width` and `element.height` are not allowed in a part's
padding, margin and size limits or in widget gaps, which lay the element out, so one query per frame suffices (S12).

## S9. Inputs and readers (R2.6, R18)

```rust
pub struct InputBases { pub entries: Vec<(InputName, InputValue)> }
pub struct InputValue { pub per_scheme: SchemeMap<Value<Color>>,
                        pub per_contrast: Vec<(Contrast, SchemeMap<Value<Color>>)> }
pub enum Contrast { Increased, High }         // when both conditions hold, High's base applies
// An input a later layer states replaces the earlier one whole (per_scheme and per_contrast together); where it states
// no contrast base, its per_scheme value applies under contrast too.
```

**S9.1 Declared inputs per preset:** `accent` (all); `accent_ramp` entries (`windows-11`); the AppKit-derived
accent roles and `highlight` (`macos-*`); the tint-dependent roles (`ios-*`); on `material-3` the accent is the seed, which the scheme function reads; on
`kde-breeze` the inactive input roles and the scheme inputs `kde_scheme.<set>.<key>`, the values of the scheme file
before Plasma applies the accent (the preset's from Breeze's scheme files; the reader's from the file kdeglobals
names, `[General] ColorScheme`) (R25.1).

**S9.2 Setting the accent:** `Theme::set_accent(Accent)` sets the declared accent inputs in the accent layer's bases
(S8); on `windows-11`, `macos-*` and `ios-*` it requires every declared accent input and otherwise returns
`Err(AccentInputsMissing)`. On `kde-breeze` it takes the accent alone and, unless a mapping palette applies, also states, in the same layer's
colour bases, each accent-dependent role as `= kde_accent(<set>, <key>)`; under a mapping palette it sets the
`accent` input only, and the palette's roles that follow the accent by reference follow it, whatever the order the
two were set in. Otherwise so they follow Plasma's derivation over the
scheme inputs (R25.1); the reader itself states the derived values kdeglobals holds.

**S9.3 Readers** state each value under the conditions they read it in: base conditions into the reader layer's
bases, other conditions into reader rules (or inputs' per-contrast bases). The new reads per platform are the rows
of the traceability whose target is a reader read (S20); each is a `reader::<platform>` change.

**S9.4 Text scaling:** the reader supplies `text_scaling_factor` (1 where the platform scales by the font size
itself, and 1 where a reader reads each text style's size, R10); the query scales font sizes by it before
font-derived lengths, and pixel line heights with it (R10), by the preset's `token.text.scaling`: `Linear` (a
product); `AndroidNonLinear`, Android 14's `FontScaleConverter`, which scales large sizes less (MAT-X19); or
`DynamicType`, each text style's size for the reader's or stand-in's `content_size_category`, to Apple's published
table (the curve `uifm_scaled` uses for lengths): a part's font is scaled by the curve of the text role its style
references (the resolved style records it), a font stated directly by `body`'s; a reader that reads each style's
size reports the default category. The last two stand in while `material-3` and `ios-*` have no
reader (R25.15).

## S10. Declarations (R2.7)

```toml
[declare]
flavors = ["widgets", "kirigami"]            # first is the default
materials = ["mica", "mica_alt", "acrylic"]
color_roles = ["fill_tertiary", …]            # keyed roles
text_styles = [ … ]
shapes = [ … ]                                 # the platform's shape library (R25.5)
elevation = ["<step>", …]                       # the platform's ramp's names
surface_sets = ["window", "view", …]
conditions = ["@graphic_effects_level", …]    # SystemSetting keys
params = { layout = { unit = "px", keys = ["grid_unit", …] }, opacity = { unit = "fraction", keys = [ … ] } }
inputs = ["accent", "accent_ramp.light1", …]
# every keyed namespace is declared:
blend_modes = [ … ]
timings = [ … ]                                # motion timing tokens' keys
transition_kinds = { <kind> = { <param> = "<unit>" } }    # with their parameters' units
presentation_kinds = { <kind> = { <param> = "<unit>" } }
presentation_events = [ … ]
container_sizes = { sidebar = [ … ] }
symbol_variants = [ … ]
icon_state_effects = [ … ]
focus_kinds = [ … ]                            # FocusKind::Keyed (WinUI's reveal)
device_models = [ … ]
material_params = { <key> = "<unit>" }
durations = [ … ]                              # motion durations' keys
icon_contexts = [ … ]
status_colors = [ … ]
drag_cursors = [ … ]
accent_ramp = [ … ]
rendering_engines = [ … ]                      # TextRendering.engine and lcd_filter
lcd_filters = [ … ]
schemes = ["light", "dark"]                    # a palette's schemes (S14)

[declare.widget.button]
variants = ["primary", "destructive", "flat", "@pill"]      # @: keyed
sizes = ["small", "regular", "large"]
densities = ["compact"]
flags = ["@<flag>"]
exclusive = [["disabled", "pressed", "hover"]]              # one group, highest precedence first; the gate checks
                                                           # the order against the state models (S17)
sizes_by_context = [{ context = "navigation_bar", sizes = ["regular"] }]   # the nearest such ancestor
sizes_by_variant = [{ variant = "@fab", sizes = ["small", "regular", "large", "extra_large"] }]   # MAT-N1
default_size = "regular"                                   # the size of an element that states none (ADD-A60)
available = true                                           # false: the platform has no such control
requires_variants = []                                     # e.g. iOS's checkbox only as `selection`


[declare.flavor.kirigami.widget.button]                   # a flavour's declarations where they differ (R2.7)
sizes = ["regular"]

[declare.flavor.kirigami.component.<key>]                  # likewise for a component (which flavour draws it)
available = true

[declare.widget.spinner]
data = { indicator = ["width", "height"] }                 # element data a part's values read (S8)

[declare.component.rule_editor]                # parts and their fields: the catalogue's table, generated (S6.3)
sizes = ["regular"]                            # a component's sizes and densities, as a widget's
values = { row_gap = "length", presentations = "presentations" }   # what varies per preset: values' types,
variants = ["@<variant>"]                                           # variants, flags
flags = ["@<flag>"]
```

Adding a layer rejects any rule, reference or patch naming something undeclared (S12). Which flags form an
exclusive group comes from the platform's state model (the catalogue names the flags, mapping each platform's own
state names to them); the order of precedence inside a group is a fact, cited from the platform's control code: the
preset states it as the order of its `exclusive` arrays, and the gate checks that order against the state-model data
(S17).

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
corners_px = <n>

[[rules]]
when = { conditions = ["scheme:dark"] }
target = "button.body"
set = { shadow = "<layers>" }

[[rules]]
when = { states = ["checked", "hover"], variants = ["flat"] }
target = "button.body"
set = { fill.append = ["= alpha(surface.foreground, <fraction>)"] }
```

**Spelling.**

| Model | TOML |
|---|---|
| `Value::Stated(x)` | the value: `"<colour>"`; a plain number under a key with its unit's suffix (`_px`, `_ms`, `_deg`; a `FontSize` takes `_pt` or `_px` by its variant); `true`; a table for a block |
| `Value::Expr(e)` | `"= <expression>"` |
| `Value::Auto` | `"auto"` |
| `Value::Absent` | `"absent"`; a whole part: `absent = true` in its table or a rule's `set` |
| a condition | `"<name>"` in snake_case (`"window_inactive"`); with a parameter `"<name>:<value>"`: `"scheme:dark"`, `"tiled:left,top"`, `"input:touch"`, `"idiom:pad"`, `"flavor:kirigami"`, `"preset:adwaita"`, `"system_setting:<key>"`, `"container_size:sidebar:<key>"` |
| a rule's target | `"colors"`, `"tokens"`, `"<widget>.<part>"`, `"<widget>.values"`, `"component.<key>.<part>"`, `"component.<key>.values"` |
| a rule's patch | the table `set`; the keys `when`, `target` and `set` are the rule's own |
| per-contrast input bases | `[inputs.<name>.contrast.increased]`, `[inputs.<name>.contrast.high]`, each with `light` and `dark` |
| one preset's part of a layer | `[for_preset.<id>.…]` (bases naming that preset's keyed names, in an application layer or a palette) |
| widget values, component parts | `[widgets.<widget>.values]`, `[components.<key>.<part>]`, `[components.<key>.values]` |

`when` keys: `conditions`, `not_conditions`, `context` and `not_context` (arrays of `{ scope = "any" | { nearest =
[...] }, widget, part, surface, sizes, not_sizes, variants, not_variants, states, not_states }`; `widget` may be left out to match by
variant alone), `position` (`{ among, first, last, only, parity, previous, next }`, `among` = `"same"`, `"any"` or a
list of widgets; `previous` and `next` as `{ widget, part, variants, not_variants, states, not_states }`),
`variants`, `not_variants`, `sizes`, `not_sizes`, `densities`, `not_densities`, `size_class` (`{ horizontal,
not_horizontal, vertical, not_vertical }`, each a list), `direction`, `states`, `not_states`. Every value carries its fact's citation in a comment, as today
(`scripts/check_preset_citations.py`). Palettes use the same format with `[preset] kind = "palette"` and only
`colors`, `inputs`, `tokens.kde` and colour rules.

## S12. Validation (R2.8)

Adding a layer (S8) fails with `LayerError` (listing every problem) when:

- a field is missing (S2.2), or `Absent` where not `may_be_absent` (S6.3);
- a name is undeclared (S10) or a rule targeting `Colors`/`Tokens` has non-condition selector parts; a rule's patch
  does not match its target (S6.1);
- a map has a duplicate key; `Auto` is stated on a field the catalogue does not mark `auto`; a call is stated on a
  `Value<bool>`, `Value<Key>` or enum field (a reference is allowed); a paint has no layer or a shadow no layer (S2.3, S2.5); `pick` has no colours;
- an expression mistypes, refers to an unknown name, forms a cycle (through bindings too), or can meet an absent
  operand under any binding and rule it can apply under;
- an expression names element data its part does not declare, or names `element.width` or `element.height` in a
  part's padding, margin or size limits or in a widget gap (S8), or names a `Concentric` corner, or a focus
  indicator states `FollowPart` on a corner whose two sides have different offsets;
- an expression nests deeper than 32, or a chain of references is longer than 32;
- a `Fields` patch can meet an `Absent` or `Auto` block under some combination of the rules before it (checked
  conservatively), or `base` or `Append` appears in a layer's bases;
- a layout field (padding, margin, size limits, widget gaps) can read `element.width` or `element.height`, directly
  or through `part.`, `widget.` or a `Full` corner (transitively);
- a patch sets a field its part does not have (S2.11);
- a spring has a mass or stiffness not above zero, a negative damping or a `rest_epsilon` not above zero; a cubic
  Bézier has an x outside `0..=1`; a `Steps` timing has no step;
- a token or colour-role expression names an operand only an element has (`element.`, `widget.`, `values.`,
  `part.`, and `font_size`, `line_height` outside the block tokens S2.2 names) or a colour-role definition names
  `surface.` (S3.3);
- a keyed selector name (a variant, flag, condition, container size or event) is declared but no rule selects on it,
  or a declared keyed name no base states (a typo's sign); a `SystemSetting` the preset declares for applications to
  read (`Theme::system`) is exempt;
- a stated number is not finite.

`Theme::add_layer` validates the layer against the theme and refuses it with the error (R2.5). A rule whose
`Preset` condition excludes the current preset is neither validated nor applied, so one application layer can carry
rules for several presets' keyed names (R2.3).

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
                             KCommandBar, AppKitContextHelp, WinUiKeyTip, UiKitContextMenu,
                             KirigamiAddonsSearchPopup, KirigamiAddonsOnboardingTip,
                             CenteredInHost, OverlayBottomCenter, BottomSheet,
                             KeyboardAvoid(KeyboardAvoidance) }   // OverlayBottomCenter: AdwToastOverlay's and
                                                                  // Kirigami's toasts (GN-2.12, Q2.50)
pub enum ScaffoldSlot { Fab, Snackbar }
pub enum KeyboardAvoidance { WinUiContentDialog, UiKitAlert, UiKitSheet, WinUiScrollIntoView, AndroidPan,
                             AndroidResize, AndroidInsetsAnimation }
pub struct PlacementParams { pub gap: Value<Px>, pub edge_margin: Sides<Value<Px>>, pub side: PlacementSide,
                             pub align: PlacementAlign,            // mirrored in RTL
                             pub flip: Axes<bool>, pub slide: Axes<bool>, pub width: AnchorWidth,
                             pub over_selected_item: bool, pub max_visible_items: Option<u32>,   // None: no limit
                             pub use_full_screen_height: bool, pub anchor_grow: Sides<Value<Px>>,
                             pub host: HostArea, pub window_edge_margin: Value<Px>,
                             pub handedness: bool }               // WinUI's and AppKit's menus (ADD-A21, ADD-A22)
pub enum PlacementSide { Below, Above, Start, End, AtPointer }
pub enum PlacementAlign { Start, Center, End }
pub enum AnchorWidth { Content, AtLeastAnchor, Anchor }
pub enum HostArea { Window, ClientArea, Overlay }
pub struct PlacementInput { pub anchor: Rect, pub size: Size, pub window: Rect, pub work_area: Rect,
                             pub pointer: Option<Point>, pub selected_item: Option<Rect>,
                             pub keyboard: Option<Rect>, pub direction: Direction }
pub fn place(strategy: &PlacementStrategy, params: &ResolvedPlacementParams, input: &PlacementInput) -> Rect;
// ResolvedPlacementParams: PlacementParams with every Value evaluated, as `values()` returns it (S8)
```

`native_theme::placement`, pure functions, each to its platform's source (cited in its doc comment); the closed
list is the traceability's placement rows, each implemented (R16). An optional input a strategy needs and lacks is
taken as: no pointer, the anchor's start corner; no selected item, the first item; no keyboard, none shown.

## S14. Palettes (R19)

A palette is a file in S11's format with `[preset] kind = "palette"`: colour-role bases, input bases and colour
tokens for the schemes its project defines (R19). Two sources, each stated in the file's header:

- **native-theme's mapping** (the default): each role it covers maps to a colour the project publishes, named by the
  project's name for it (`# catppuccin: Surface0`), or to a derivation of those (S2.2); a value picked from a range
  the project's guide gives is marked `# pick: <the guide's words>`. It is the same file on every platform.
- **the project's port** for one platform, where one exists (Catppuccin's and Dracula's KDE colour schemes, R19):
  `Palette::from_port(file) -> Result<Palette, PortError>` converts a KDE `.colors` file mechanically, and that
  palette is used on `kde-breeze` in place of the mapping.

A palette's bases name typed roles, the `accent` input and the declared inputs of the presets it has sections for; a
typed role or surface set the current preset does not declare is skipped, not refused. On `windows-11`, `macos-*`
and `ios-*`, whose accent derivations are not published (R2.6), the mapping states the derived accent inputs in a
per-preset section (`[for_preset.windows-11.inputs]`), each labelled as native-theme's pick (R19).

A `Palette` holds the mapping and the ports it has, per preset; `Theme::apply_palette(Palette)` adds, as an
application layer that replaces the reader's colours (S8), the port for the current preset where there is one (a KDE port
converts to the `kde_scheme.*` inputs, the colour roles and `token.kde`), else the mapping. It refuses a palette
under a scheme it lacks (`Err(SchemeNotInPalette)`); when the system scheme changes to one the palette lacks, the
palette is suspended, the theme is the preset with the reader's values, and the palette applies again when the
scheme returns (R19); `ResolvedTheme::palette_status()` reports `Applied`, `Suspended` or `SkippedForContrast`. A palette
carries no icon set. A palette states its schemes in its header (`schemes = [...]`); `Theme::pick` of a scheme the
applied palette lacks is refused like `apply_palette`; a palette that has neither a mapping nor a port for the current
preset (a KDE port alone, on `adwaita`) is refused (`Err(PaletteNotForPreset)`). A mapping palette need not state
`token.kde`: the preset's applies. The facts gate checks each mapped colour against the project's
published colours (pinned like any source, S17) and skips the picks.

## S15. System control rendering (R11)

```rust
pub enum SystemControl { AppKitCheckbox, AppKitRadio, AppKitDisclosure, AppKitPushDisclosure, AppKitPopUp }
pub struct ControlImage { pub width: u32, pub height: u32, pub pixels: Vec<[f32; 4]>,  // extended sRGB, straight
                          pub arrow: Option<Rect> }                                     // alpha (S2.1)
pub struct SystemAppearance { pub scheme: Scheme, pub increased_contrast: bool }   // AppKit's four appearances
pub fn render_system_control(control: SystemControl, states: StateFlags, size: ControlSize,
                             appearance: SystemAppearance, key_window: bool, scale: f32,
                             width: Option<Px>) -> Option<ControlImage>;   // width: a content-sized control's
```

What each image covers: `AppKitCheckbox`, `AppKitRadio`, the box and mark; `AppKitDisclosure`, the triangle;
`AppKitPushDisclosure`, the bezel and chevron; `AppKitPopUp`, the whole button at `width`, its arrow area being the
cell's `indicatorRect` (`ControlImage.arrow`). The rendering uses the system's accent,
which the connector records as the accent the theme has (R11).

On macOS's main thread it renders; anywhere else, and off macOS, it returns `None`, so connectors need no `cfg` of
their own. Cached per (control, states, size, appearance, key, scale, width). **Open, the maintainer's decision:**
rendering through AppKit needs `unsafe` objc2 calls, as the SF Symbols loader does under its approved exception
(`native-theme/src/sficons.rs`, module-scoped, each block with its safety argument); the project allows `unsafe` only
with the maintainer's approval, so this section stands on that approval. A glyph naming a system control
(`GlyphSource::SystemControl { control, replaces }`) states the parts the rendering replaces (the checkbox's box and
mark together, R11). The list of `SystemControl`s is the traceability's rows that need it.

## S16. The API surface

`SystemTheme` is renamed `Theme`, the one owner of the stack, the inputs and the application's conditions.
`AccessibilityPreferences` goes: the reader reports conditions and the text-scaling factor in its `ReaderState`.
Every mutation validates the whole stack and is refused whole on failure, the theme staying as it was (S8, S12):

```rust
impl Theme {
    pub fn from_system() -> Result<Theme, ThemeError>;    // reads the system; the preset and its generation by the
                                                          // OS version and the app's opt-out (R1.5, as
                                                          // `pipeline::preset_for_version` does); an invalid first
                                                          // read falls back to the preset alone, reported by
                                                          // reader_status()
    pub fn from_preset(id: PresetId) -> Result<Theme, ThemeError>;
    pub fn set_preset(&mut self, id: PresetId) -> Result<(), LayerError>;          // a theme change: the reader
                                                          // applies its portable part (accent, scheme, fonts, text
                                                          // scaling, conditions) to any preset and its
                                                          // `[for_preset.<id>]` part to its own; the accent layer is
                                                          // derived again for the new preset
    pub fn set_reader_stand_in(&mut self, state: ReaderState, layer: Layer) -> Result<(), LayerError>;
    pub fn pick(&mut self, scheme: Option<Scheme>) -> Result<(), LayerError>;     // None: follow the system
    pub fn set_flavor(&mut self, flavor: Key) -> Result<(), LayerError>;          // undeclared: refused
    pub fn add_layer(&mut self, layer: Layer) -> Result<LayerId, LayerError>;
    pub fn replace_layer(&mut self, id: LayerId, layer: Layer) -> Result<(), LayerError>;
    pub fn remove_layer(&mut self, id: LayerId) -> Result<(), LayerError>;
    pub fn set_accent(&mut self, accent: Option<Accent>) -> Result<(), LayerError>;   // None: the system's again
    pub fn apply_palette(&mut self, palette: Option<Palette>) -> Result<(), LayerError>;  // None: removes it
    pub fn reader_status(&self) -> ReaderStatus;
    pub fn reload_reader(&mut self) -> ReaderStatus;     // re-read the system into the reader layer in place
                                                          // (a failure keeps the old layer); the application calls it
                                                          // from its event loop on the watcher's `on_theme_change`
    pub fn system(&self) -> &ReaderState;                 // the conditions the reader reports, for what an
                                                          // application acts on itself (Windows' ShowSounds, W-P3-11†)
    pub fn set_contrast(&mut self, contrast: Option<Contrast>) -> Result<(), LayerError>;  // None: the reader's
    pub fn resolve(&self, window: &mut WindowState, conditions: &WindowConditions) -> Resolution;   // S8
}
impl Palette {
    pub fn bundled(id: PaletteId) -> Palette;             // native-theme's mapping, with the ports it has
    pub fn from_toml(text: &str) -> Result<Palette, LayerError>;
    pub fn from_port(file: &str) -> Result<Palette, PortError>;   // S14
}
```

The watcher (`native-theme/src/watch`) tells the application that the system changed; `on_theme_change` stays the
notification, and the application calls `reload_reader` and resolves its windows again (S8).

## S17. Facts as data and the gate (R4, R22)

**Format** (`docs/facts/<platform>/<area>.toml`):

```toml
[[fact]]
key = "button.body.fill"                          # model path
when = { states = ["checked", "hover"], variants = ["flat"] }   # as S11's `when`
value = "= alpha(surface.foreground, <fraction>)"   # value or the platform's derivation
kind = "published"                                # read | published | measured
source = "libadwaita 1.10.0 src/stylesheet/widgets/_buttons.scss:L"
tolerance = 0.0                                   # measured: the measurement's precision
```

`docs/platform-facts.md` is generated from these files (a script), and the two existing scripts read the data.

**State models** (`docs/facts/<platform>/state-models.toml`, cited): per widget, the combinations the platform
draws (states × conditions × containers × variants × sizes, R4), the exclusive groups' order of precedence, and the
source of each (the catalogue's "state-model source" line points here).

**Which fact applies.** Facts in a file are ordered as the platform's source orders them (CSS order for libadwaita,
visual-state order for WinUI, branch order for Breeze), so their order is the platform's, independent of the preset's
rule order. For a key and a combination, the fact that applies is the **last** in file order whose `when` holds, after
the exclusive groups' reduction (S10). Where the platform states a combination ({checked, hover} in GN-1.29), its
facts state it, and a preset that forgot it fails.

**The gate** (`native-theme/tests/facts_gate.rs`): it resolves each platform preset with a fixed stand-in
`ReaderState` (the base conditions, factor 1) and no reader, so the machine's own settings never enter. For every
widget and every combination of its state model, it resolves under the combination's conditions and queries the
element, and compares each key with the fact that applies (within tolerance); a key and combination with no fact
fails (R4). It compares every token, colour-role and input fact the same way, under its `when`. It repeats with
perturbed inputs and base colours, evaluating the facts' derivations the same way. iOS's facts are
`docs/facts/ios/…` like any platform's (R22).

## S18. Documents and scripts

- `docs/property-registry.toml`: stays the checked list of the model's fields (R22), now generated from the
  derive's field registry (S6.1); a test fails when the checked-in file differs from the generated one.
- `docs/inheritance-rules.toml`, `scripts/check_wrong_safety_nets.py`: removed (R20).
- `scripts/check_platform_facts_rows.py`, `scripts/check_preset_citations.py`: read the facts data.
- `scripts/check_spec_trace.py`: new; checks that every gap-map row (the IDs of the map's Appendix A, which
  holds no pointer row, R24), excluding withdrawn rows, has at least one line in the traceability (S20), that every line names a live row, and that each
  line's kind and target agree (a target kind has a target, a verdict has `-` or `- (note)`, and `readers (R18)` may
  name its read). It checks working documents, so
  it is run by hand while they change and is not part of `scripts/check_release.sh`; it is deleted when the
  documents are archived.
- The showcases' element lists and parity gates follow the connectors (R21).

## S19. Migration inside the branch

**S19.1** The presets are rewritten to S11 from the facts; the community presets become palettes (S14): one per
variant the project publishes, their invented light variants and their `icon_set` removed.
**S19.2** Today's inheritance rules (`[uniform]`, `[per_platform]`) become expressions in each preset's bases,
converted by a script from `docs/inheritance-rules.toml` before it is removed, then checked by the gate; the
`[wrong_safety_nets]` are not converted, as the facts replace them.

## S20. Traceability format (R24)

Per platform, `todo_v0.7.1_model-completeness-spec-trace-<platform>.md`: at least one line per live gap-map row,
several where a row needs several targets:

| Row | Target | Kind |
|---|---|---|
| GN-1.29 | `button.body.fill` rules for each state combination the row lists, under variant `flat` | rule |
| … | | |

Kinds: `field`, `rule`, `role`, `token`, `component`, `input`, `reader` (a new read), `placement`, `system-control`,
`condition`, `flag`, and the verdicts, whose target is `-` (or `- (note)` naming the part of the row a verdict
covers when the row's other parts are carried; `readers (R18)` may instead name the read the later reader makes): `out (R1.1)` (the system draws it), `out (R1.2)` (a
library failing R1.2's test), `out (R1.4)` (a Mac Catalyst-only row; a SwiftUI-only control is in, R1.4), `preset (R1.3)` (another toolkit's preset),
`newest (R1.5)` (an older generation's look), `reference (R1.6)` (a look the preset's chosen reference does not
draw: material-web where Compose is Material's reference), `readers (R18)` (the iOS and Android readers' milestone).

## S21. Release and compatibility (R21)

**S21.1** 0.8.0, one release, on `v0.8.0-rc1`; every platform preset, `ios-26` included, complete and passing the gate.
**S21.2** Every public enum is exhaustive (S1), so a connector maps each in full: `IconRole` loses its
`#[non_exhaustive]`. Every public struct has it, except the closed value structs S1 names.

**S21.3 The connector contract (R23).** Each connector keeps a contract table (today's contract tables, extended):
every resolved field it receives, with the toolkit lines that draw it, or the reason the toolkit cannot and what it
draws instead; the element tree it builds for the query (S8); the window conditions it supplies and how it reads
them. A connector test fails when a resolved field is neither mapped nor listed. The connectors' milestones apply it
(R21).

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
| ADD-A17 | an inner focus ring, stated as a shadow layer of the indicator (R7) | S2.10 |
| ADD-A18 | `FocusIndicator.offset` per side | S2.10 |
| ADD-A19 | `TransitionKind::FocusMove` | S2.7 |
| ADD-A20 | size classes per axis | S7, S8 (R25.10) |
| ADD-A21 | Placement strategies for S13's closed list | S13 |
| ADD-A22 | `PlacementParams` fields | S13 |
| ADD-A23 | Overwrite-mode caret | S4.2 |
| ADD-A24 | The caret as a part of the text widgets | S4.2 |
| ADD-A25 | Presentation-transition details | S2.7 |
| ADD-A26 | `PresentationKind::Keyed` with parameters; keyed kinds | S2.7 |
| ADD-A27 | `TransitionKind::Keyed { key, params }` | S2.7, S2.9 (R25.12) |
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
| ADD-A44 | Keyed scalar tokens, `Tokens.params` | S4.6 |
| ADD-A45 | Behaviour fields | S4.2 |
| ADD-A46 | `Tokens.pointer` | S4.7 |
| ADD-A47 | `Material.params` | S2.3 |
| ADD-A48 | Colour-role closure | S3 |
| ADD-A49 | `GlobalRoles.selection_text_background_unfocused` | S3 |
| ADD-A50 | `qt_lighter(c, factor)`, `qt_darker(c, factor)` | S2.2 |
| ADD-A51 | `kcu_overlay(under, over)` | S2.2 |
| ADD-A52 | `kirigami_tint(c, tint, alpha)` | S2.2 |
| ADD-A53 | Kirigami functions | S2.2 |
| ADD-A54 | `sf_tracking(font_size, design)` | S2.2 |
| ADD-A55 | Lengths that follow text scaling | S2.2 |
| ADD-A56 | Arithmetic | S2.2 |
| ADD-A57 | `kqc_series_color(seed, count, index)` | S2.2 |
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
| ADD-A68 | `ContextMatch.surface`: a context matched by the surface set an ancestor binds | S7 |
| ADD-A69 | `TextStyle.word_spacing` | S2.8 |
| ADD-A70 | `Caret.shown_in_read_only` | S4.2 |
| ADD-A71 | `RenderingMode::Template`, `Original` | S2.9 |
| ADD-A72 | `PartStyle.touch_target` | S2.11 |
| ADD-A73 | `ReaderState`, the reader's and stand-in's system conditions | S8, S16 |
| ADD-A74 | `MaterialScheme.contrast_level` | S4.8 |
| ADD-A75 | `Glyph.stroke` | S2.9 |
| ADD-A76 | `Condition::UnitScale` | S7.1 |
| ADD-A77 | `Stroke.outset` | S2.4 |
| ADD-A78 | the window's and the screen's size as operands, from `RootInputs` | S8 |
| ADD-A79 | `PartStyle.margin` | S2.11 |
| ADD-A80 | `widget.<part>.<field>` references | S2.2 |
| ADD-A81 | (withdrawn: its row, MAT-X10's focus-ring animation, is `reference (R1.6)`) | — |
| ADD-A82 | `FocusIndicator.dash`, `FocusKind::Keyed` | S2.10 |
| ADD-A83 | `Condition::Freeform`, `Posture`, `DeviceModel` | S7.1 |
| ADD-A84 | `ContextMatch.sizes` | S7 |
| ADD-A85 | `Caret.solid_while_typing` | S4.2 |
| ADD-A86 | pointer-hide delay and fade, pixels per wheel step, wheel-step duration range | S4.2 |
| ADD-A87 | `ElementData.move_distance` | S8 |
| ADD-A88 | `TextScaling` (R25.15) | S4.4, S9.4 |
| ADD-A96 | `Comparison`, `Measured`, `OverlayBottomCenter`, two-edge fade, touchpad velocity samples, `Theme::system`, decoration patterns | S7.3, S13, S2.8, S4.2, S16 |
| ADD-A95 | `Radius::FollowPart`, `Breakpoint`, `sizes_by_variant`, touchpad inertia, `Theme::set_contrast`, `WindowState` | S2.4, S7.3, S10, S4.2, S16, S8 |
| ADD-A89 | `FadeUnder` | S2.8 |
| ADD-A90 | `uikit_prominence`, `gtk_wheel_step`, `shadow_extent`, `kte_mark_underlay` | S2.2 |
| ADD-A91 | `ScrollPhysics.touchpad_factor` | S4.2 |
| ADD-A92 | `SharedFlag::TextSelected` | S7.2 |
| ADD-A93 | `Mirroring::SwapWith` | S2.9 |
| ADD-A94 | `gtk_contrast_pick` | S2.2 |
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
| ADD-B18 | the display's corner radius, the root's container (`RootInputs`) | S4.6, S8 (R25.11) |
| CONTRA-1 | R2.3 "KDE has no contrast setting" against K2.23† | R25.2 |
| CONTRA-2 | R9 velocity curves as a timing kind of their own | R9, R25.6 |
| CONTRA-3 | R2.3 position among the same widget kind against GTK's `.linked` | R2.3, R25.9 |
| CONTRA-4 | R11 symbol effects as transitions against an icon "effect" | R25.12 |
| CONTRA-5 | R5.4 a width in characters as a multiple of the font size | R5.4, R25.3 |
