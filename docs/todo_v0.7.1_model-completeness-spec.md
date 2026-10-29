# v0.7.1 — Model completeness: Specification

**Status:** done, 2026-10-29: checked by thirteen independent reviews (§0) and finished by the maintainer's decision after the final core check; next is the value phase. This document states the model the rationale
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
| Core (this document) | drafted; the final check's findings and its confirming check's fixed (below) |
| Research the rationale requires first: kirigami-addons survey (R1.2), community ports (R19), Plasma's accent derivation (R2.6) | done; R19 and R25 decided from it |
| Widget catalogue | 86 typed widgets, 28 keyed components; its name and entry checks pass (working scripts of the specification's sessions, not kept in the repository, so they cannot be re-run from it; the kept check is S18's `scripts/check_spec_trace.py`) |
| Traceability | every row of the six platforms traced; `scripts/check_spec_trace.py` passes |
| Check passes | serious findings per pass, core / catalogue / traces (random sample): 1: 11 / 5 / 9 rows of 256; 2: 10 / 4 / 6.7 % (then every row re-verified); 3: 6 / 5 / 0.8 %; 4: 6 / 12 / 1.25 %; 5: 3 / 4 / 0.4 %; 6: 1 / 5 / 0.4 %; 7: 0 / 5 / 0 %; 8: 0 / 2 / 0 %; 9: 1 / 2 / 0.4 %; 10: 1 / 8 / 0.4 %; 11: 0 / 9 / 2.1 %; each fixed (pass 11's fix round done: the core's 9 should-fix and 19 minor findings, the catalogue's and the traces' findings, and the ten core items the catalogue round found, ADD-A110). The maintainer chose to finish the specification with one final check of the core alone: 12 (final, core only): 2 serious, 12 should fix, 21 minor, each fixed (ADD-A111): KDE's tint switch an `Option` and every `token.kde` value stated, so no function branches on absence; the sets with an `[Inactive]` group a stated input (`kde_scheme.inactive_groups`); a port's inactive roles by `applyScheme()`'s rules; the accent-dependent roles listed; corners, stroke edges and shadow offsets physical in right-to-left; one-of selector parts and the checks' precision in validation; element data declared by the preset and readable by any layer; the rest as the check recommended. The maintainer chose to finish after that final core check, with one confirming check of its fixes: 13 (confirming, core only): 1 serious, 11 should fix, 7 minor, each fixed in this last round (ADD-A112): a KDE scheme file lacking a colour group or key read as KColorScheme and `applyScheme()` read it (`kde_scheme.groups` and `unset_keys` beside `inactive_groups`; a scheme without a `Header` group gives `Header` `Window`'s results, its title-bar accent `[WM]`'s alone; Dracula's port converted, `PortError::MissingKey` gone); an accent run of the facts gate for `kde_accent` and `kde_accent_wm`; another platform's accent derived on `kde-breeze`; the KDE reader's scheme inputs from kdeglobals where its scheme file is missing (`ReaderStatus::Fallback`); the reader's `token.kde` under the scheme it read; the query returning right-to-left values physical, the transform origin unmirrored; the exact check's non-applying rules; `widget.<part>` refused on a part that can be absent or drawn as another; the accent layer per scheme under a mixed palette; `LayerError::UnknownLayer`; `kde_effects`' starting background; `ScrollDecay` variants with their own parameters; the model's implications among selector atoms; the rest as the check recommended |

## S1. Conventions

- **Names.** Rust types are `UpperCamelCase`, fields `snake_case`, with no unit suffix: the type carries the unit.
  TOML keys are the field names with the unit's suffix added from the type (`_px`, `_ms`, `_deg`; a `FontSize`
  field `_pt` or `_px` by its variant, S11), as today's files spell them, and a stated value under such a key is a plain number (`min_width_px = <n>`); `"<n>px"` is spelled only
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
  `Axes`, `Rect`, `Size`, `Point`, `ViewBox`, `TiledEdges`, `SetMatch`, `OutlineCorners`, and the newtypes `Px`, `Ms`,
  `Deg`, `Fraction`, `Key`) and the inputs their caller fills in full (a connector: `WindowConditions`, `RootInputs`
  and the `SizeClasses` in it, `Sibling`, `PlacementInput`, `SystemAppearance`; a reader or a stand-in application:
  `ReaderState`), so a field added
  later breaks every such caller's build, as a new enum variant does (R21);
  every public enum is exhaustive, error enums included, so a connector maps each in full (S21.2), a choice made for
  simplicity before 1.0. A public struct whose every field has a
  neutral default (the patch types but `ComponentValuesPatch`, `Selector`, `ElementData`, `Layer`, `Bases`,
  `SchemeMap`, empty by default) implements `Default` and is built from `default()` and field assignment, as R21
  prescribes (`SchemeMap` also has `SchemeMap::new(light, dark)` and `SchemeMap::one(scheme, value)`, a palette's one
  scheme, S14); any other struct, one with a field that has no neutral default (a `Value`, a `Key`, a `Scheme`, an
  enum), has a constructor taking exactly those fields, the others then assigned: `Element::new(part, scope,
  metrics)`, `Rule::new(selector, body)`, `Accent::new(accent: InputValue)` (its `derived` then assigned),
  `InputValue::new(per_scheme: SchemeMap<Value<Color>>)` (its `per_contrast` then assigned),
  `ComponentValuesPatch::new(key)`, `ContextMatch::new(scope)`, `KeyRepeat::new(delay, interval)`, unless it is a full
  block (S2), which is built through its patch type,
  `T::try_from(TPatch) -> Result<T, LayerError>`, which refuses a patch leaving a leaf missing and names every such
  leaf (S12). Validation refuses an `Absent` left where it is not allowed (S12), so a default
  never stands in for a value. A layer is also built from TOML in S11's format, `Layer::from_toml(&str) ->
  Result<Layer, LayerError>`.
- **Absence.** In the model, one route: `Value::Absent`, allowed where S6.3's two sources allow it: a block's
  sub-field this document marks "may be absent" (wherever that block appears), and the catalogue's `may_be_absent`
  tables (per widget, part, field, colour role and token). In the model, `Option` appears only where `None` means
  something other than "not drawn", stated at the field (`max_lines: None` is no limit); a resolved `Value` field is
  `None` when not drawn (S6.1).

- **Small types.** `PartRef` is `Typed(TypedPart)` or `Component { key: Key, part: Key }`, where `TypedPart` is a
  derive-generated enum over the typed widgets, each holding that widget's part enum (`TypedPart::Button(ButtonPart::
  Label)`), so a misspelled typed part, or a part paired with the wrong widget, does not compile;
  `WidgetOrPart` is `Widget(WidgetRef)` or `Part(PartRef)`, the one field where a selector or a substitution names
  a widget or one of its parts (a `PartRef` names its widget, so the two cannot disagree; TOML spells it as the keys
  `widget` and `part`, the part a bare name of that widget's parts, S11);
  `StateFlags` is a set of `StateFlag`; `InputName` is an input's path (`accent`, `accent_ramp.light1`);
  `SizeClassThreshold` is `{ horizontal: Option<Value<Px>>, vertical: Option<Value<Px>> }`,
  the class's inclusive lower bound on each axis (`None`: not a class of that axis); a window's class on an axis is
  the one with the largest bound not above its size there, and the axis has none when no class qualifies; `Accent` is `{ accent: InputValue, derived: Vec<(InputName, InputValue)> }`
  (S9.2); `WidgetValue` is the union of the value types widget values use: a number, integer, length, duration,
  bool, key, enum variant, colour, paint, timing, transition, loop, glyph, icon style, scroll physics, breakpoints,
  `Vec<PresentationTransition>`, a `Placement` (S13), `Sides<Value<Px>>` (a length per side, S10's `"sides"`), a
  font size (`Value<FontSize>`, S2.8: `input.min_font_size`; converted from points as S9.4 converts a font size, and
  not text-scaled, a platform that scales it stating the expression, `uifm_scaled`), `Vec<char>` (characters in order
  of preference, `input.mask_character`), `Vec<Key>` (a list of declared keys, S10's `keys`) and `Vec<MenuEntry>`
  (`MenuEntry` is the catalogue's, defined at `window`'s `window_menu_items`: an action key, a separator or a
  submenu of entries); a keyed component's values are typed by its declaration
  (S10). `DisabledRoles` and `StatusRoles` are the catalogue's (*Colour
  roles*). `Expr` is a parsed expression (S2.2); `StepJump` is CSS's `jump-start | jump-end | jump-none |
  jump-both`; the `Widget` trait, `ResolvedTokens`, `ResolvedFont`, `ResolvedPartStyle`, `ResolvedIconStyle` and the per-widget types are
  derive-generated (S6.1); a keyed component (named by a `Key`, not a Rust type; its parts and their fields the
  catalogue's, generated, and its availability, sizes, variants, flags and values' types the preset's declaration,
  S5.1, S10) has `ComponentBase { parts: Vec<(Key,
  PartPatch)>, values: Vec<(Key, Value<WidgetValue>)> }`, `ComponentValuesPatch { key: Key, values: Vec<(Key,
  Value<WidgetValue>)> }` and `ComponentValues = Vec<(Key, ResolvedWidgetValue)>`; `ResolvedWidgetValue` is a
  `WidgetValue` with every `Value` in it evaluated; `<T>Patch` is `T`'s derive-generated patch type (S6.1:
  `PartStylePatch`, `ColorRolesPatch`, `TokensPatch`, `<W>ValuesPatch`), and `WidgetsPatch` holds, per typed widget,
  its parts' `PartPatch`es and its values' patch; `TextRole` is the enum of the nineteen `TextRoles` slots (S4.4), one
  variant per field (`Body`, `BodyEmphasized`, … `Numeric`); `DataNeeds` is a set of element-data
  operands and metrics, with a flag for a part whose glyph or icon is drawn to its box (S8); `LayerId`, `PaletteId` are opaque ids; `KeyError` is `Key::new`'s error (the text and the
  first character outside the key grammar). `SchemeMap<T>` holds at most one `T` per `Scheme` (`Default`: none; a
palette states only its schemes, S14, and a value no layer states for a scheme is missing, S2.2); `SurfaceMap<T>` one per declared `SurfaceSet`; `SurfaceRole` and
  `GlobalRole` name a field of `SurfaceRoles` and `GlobalRoles` (a path, `disabled.foreground`); `WidgetKind` is the
  typed widgets' enum and `Variant` the typed variants' (both generated from the catalogue); `PresetId` names a
  bundled preset; `AxisTag` and `FeatureTag` are OpenType four-letter tags; `IconId` (an icon role or a name) and `IconData` (an
  icon's SVG or pixels) are today's `icons` types, kept (S16); `LanguageTag` is a BCP 47 tag;
  `FontStyle` is `Normal | Italic | Oblique`; `Sides<T>` is `{ top: T, right: T, bottom: T, left: T }`, in `Stroke`'s
  edge order, stated for a left-to-right layout (a stated `left` is the start side) and mirrored with it (R16), every
  `Sides` alike (padding, margin, a focus indicator's offsets, placement margins): the query returns it physical,
  `left` and `right` swapped for an element whose direction is right-to-left (S2.4), while a `Stroke`'s edges, a
  `Corners` and a shadow's offsets are physical as stated (S2.4), and `Axes<T>` is `{ horizontal: T,
  vertical: T }`, both generic (`Sides<Value<Px>>`, `Axes<bool>`), their members spelled in TOML as any field, with
  the unit's suffix where the member has a unit (`padding.top_px = <n>`, `flip.horizontal = true`; `padding.left_px`
  is the start side, the left in left-to-right, which the query returns on the right in right-to-left); `Rect`, `Size`,
  `Point` are plain geometry in `Px`; `ContentSizeCategory` is S8's; `ViewBox` is `{ x: f32, y: f32, width: f32, height: f32 }`, unitless, in a path's own coordinates (S2.9).
  Error and status types:

```rust
pub enum LayerError { Invalid(Vec<Problem>),             // every validation problem S12 finds, not the first only
                      ReaderPresent, AccentInputsMissing(Vec<InputName>), SchemeNotInPalette(Scheme),
                      PaletteNotForPreset(PresetId),
                      UnknownLayer(LayerId) }             // `replace_layer`, `remove_layer`: an id this theme does
                                                          // not hold (already removed, or another theme's; S16)
pub struct Problem { pub path: String,                   // the model path, or the TOML key path where read from TOML
                     pub kind: ProblemKind }             // one variant per S12 item, with that item's names
pub enum ThemeError { PlatformUnsupported, FeatureDisabled,
                      PresetNotForPlatform(PresetId),     // `from_system(Some(id))`: not a generation of this platform
                      Reader { preset: PresetId, failure: ReaderFailure },   // a failed first read, with the preset
                                                          // `from_system` chose, for `from_preset` (S16)
                      Preset(LayerError) }                // a bundled preset failing validation, which a build
                                                          // test rules out (S19.1)
pub enum ReaderStatus { Read, NoReader, StandIn,
                        Fallback(ReaderFailure),            // read, but one source failed and another stood in for
                                                            // it, which the failure names (the KDE scheme file, S9.1)
                        Failed(ReaderFailure) }             // Failed: a re-read failed; the previous
pub enum ReaderFailure { Platform(String), Invalid(LayerError) }           // reader layer stays (S8, S16)
pub enum PortError { Parse { line: u32 } }   // a `.colors` file it cannot read; a group or key the file lacks is
                                             // no error, as KColorScheme and `applyScheme()` read it unset (S9.1, S14)
pub enum PaletteStatus { Applied, Suspended, SkippedForContrast }   // S14
```

## S2. Blocks (R2.1)

### S2.1 Colour

```rust
/// Extended sRGB, straight (not premultiplied) alpha. Channels may lie outside 0..=1 (Display P3 colours
/// expressed in extended sRGB); alpha is 0..=1.
pub struct Color { pub r: f32, pub g: f32, pub b: f32, pub a: f32 }
```

It replaces `Rgba` everywhere. Conversion helpers: `to_srgb8_clamped()` for connectors that render sRGB, and
`to_display_p3()`. TOML: `"#rrggbb"`, `"#rrggbbaa"` (in gamut), `"srgb(r, g, b[, a])"` with floats, or
`"display_p3(r, g, b[, a])"` with floats, a colour as Apple publishes it in Display P3 (MAC-X10), converted to
extended sRGB when the file is read, so a preset cites the published value exactly.

### S2.2 Values and expressions (R5.4, R20)

Every field of every block that the platforms can derive or omit is a `Value<T>`:

```rust
pub enum Value<T> {
    /// A stated value.
    Stated(T),
    /// A derivation, evaluated where used (R2.5).
    Expr(Expr),
    /// The platform sizes by content (lengths and line heights; R20). On a minimum size (`min_width`,
    /// `min_height`): no minimum beyond the content; on a maximum size (`max_width`, `max_height`): no
    /// maximum, the content decides (S2.11). Never an operand of a call (S12).
    Auto,
    /// The platform does not draw this; resolves to none. Allowed only where the field is marked
    /// `may_be_absent` (S6.3).
    Absent,
}
```

A field with no value in any layer is **missing** and fails validation: the layer that leaves it so is refused (S12),
so resolution never meets it.

**Expression grammar** (TOML: a string starting with `=`; whitespace insignificant):

```text
expr     := literal | ref | call
literal  := colour literal | number | length ("<n>px") | duration ("<n>ms") | angle ("<n>deg") | font size ("<n>pt";
            "<n>px" is a font size where the field or parameter is one, else a length); <n> := ["-"] digits
            ["." digits], so a literal may be negative ("-1px", a negative offset or letter spacing), the range
            check then the field's (S12)
          | name (an identifier: an enum variant in snake_case, or "@" key for a keyed name)
          | "true" | "false"
path     := segment { "." segment }
segment  := [a-z0-9][a-z0-9_]*           a field's name, a map's key (a `Key`, S1: WinUI's elevation step `1`), or
                                          an enum-keyed map's variant in snake_case
ref      := "role." set "." path        typed colour role of a surface set, a path   (S3)
                                          (`role.selection.disabled.foreground`)
          | "role.global." path           typed global role (accent, status.error.fill, …)  (S3)
          | "role.@" key                  keyed platform role                          (S3)
          | "surface." role               role of the surface bound in scope           (S3.3)
          | "active(" ref ")"             a role's active value (below)                (R5.1)
          | "token." path                 a token                                      (S4)
          | "input." path                 an input                                     (S9)
          | "base"                        the value earlier rules gave in this query   (R2.5)
          | "scheme"                      the scheme of the colours being resolved     (R5.1)
          | "font_size" | "line_height"   the part's, in logical px after the point conversion and
                                          text scaling (S9.4, R10)
          | "element." data               the element's data (S8): width, height, progress,
                                          scroll_offset, scroll_progress, item_index,
                                          item_count, text, color, screen_width,
                                          screen_height, window_width, window_height, move_distance,
                                          and char_width (the average), max_char_width (the
                                          widest glyph), font_x_height, font_height,
                                          font_underline_offset,
                                          font_underline_thickness,
                                          font_strikeout_offset,
                                          font_strikeout_thickness (the part's own font,
                                          through `Element.metrics`, S8; a part without
                                          text measures another part's font as
                                          `widget.<part>.char_width`, `.max_char_width`,
                                          `.font_x_height`, `.font_height`,
                                          `.font_underline_offset`, … , `.font_size`,
                                          `.line_height`)                              (R25.3)
          | "body_font_height"            the `body` slot's font's height (ascent plus descent,
                                          `Element.metrics.height` of `body`'s scaled font):
                                          Kirigami's label-sized icons follow it, rounded down
                                          to an icon step (Q1.1, `units.cpp:297-356`); part
                                          expressions only
          | "part." path                  another resolved field of the same part      (R25.3)
          | "widget." part "." path       a resolved field of another part of the same widget (a band
                                          following the frame's shadow, GN-1.282†; a label following its
                                          half's corners, MAT-X37†), evaluated with the same element, in
                                          the scope that part is itself queried in (S8: the root part's
                                          input scope for the root; for another part, the root's returned
                                          scope with the bindings of the plain parts enclosing it), so the
                                          value is what that part draws;
                                          not across elements (an *(element)* part
                                          reaches only its own element's parts), and
                                          not to a part that can be absent as a whole
                                          or drawn as another widget (S12)             (R25.3)
          | "values." name                the element's widget's resolved value (the spinner's stroke weight,
                                          GN-1.227; a delay button's delay, Q2.3)      (R25.3)
          | "text_scale"                  the text-scaling factor (S9.4); refused where
                                          `token.text.scaling` can be `DynamicType`,
                                          whose lengths scale by `uifm_scaled` (S12)   (ADD-A55)
call     := name "(" expr { "," expr } ")"
```

A **colour literal** is one of S2.1's forms written bare: `#rrggbb`, `#rrggbbaa`, `srgb(r, g, b[, a])` and
`display_p3(r, g, b[, a])`, whose arguments are number literals; `srgb` and `display_p3` are literal forms, not
functions, and no function takes their names. The reserved words (`base`, `scheme`, `font_size`, `line_height`,
`body_font_height`, `text_scale`, `active`, `srgb`, `display_p3`, `true`, `false`) are not names. `TextStyle` tokens
may use `font_size` and `line_height`, the style's own: a referencing part's query evaluates them where used (R2.5),
and `ResolvedTheme::tokens()` evaluates them against the style's own size, leaving an `auto` line height as
`Length::Auto`; as it has no font metrics, a `TextStyle` token's expression that reads `line_height` while that
line height can be `Auto` is refused (S12). `part.` is not allowed in tokens (S12). An expression nests at most 32
deep, and a chain of references is at most 32 long (S12). `font_size` and `line_height` are lengths; a length is
accepted where a font size is expected, as px. Under `line_height = auto` the `line_height` operand of a part's
query is the font's own line spacing, `Element.metrics.line_spacing` (the part declares it as `line_spacing`, S10), and
`widget.<part>.line_height` is that part's font's line spacing likewise (the reading part declares
`widget.<part>.line_height`, S10).

**`Auto` is not a number.** An operand that can resolve `Auto` (a size limit, a gap or a line height a rule or base
leaves `auto`, followed through references and rules conservatively, as for `Absent`) is refused in a call (S12):
`add(part.max_width, 4px)` over an `auto` maximum would have to invent a value. Two uses stay: a part's
`line_height` operand, defined above as the line spacing; and a plain reference (an expression that is one reference
and no call) on a field that itself allows `Auto`, which then resolves `Auto` with it; a plain reference that can
bring `Auto` to a field that does not allow it is refused, as a stated `Auto` there is (S12). The first use covers
`widget.<part>.line_height` too (a mark's size from the text's line, a caret's width from it): under `auto` it is
that part's line spacing, never `Auto`.

**An *(element)* part reads none of its host's parts**, a font metric included (`widget.<host part>.char_width`):
one query evaluates one element, under that element's states, data and position (S8), while the host's part resolves
its font under the host element's, which only the host's own query has; reading it would resolve a second element
inside the first. Where an *(element)* part's value follows the font of a host part, it lists `text` itself, a
reference to the style that part's `text` references (the token or slot stays the one home of the font), and reads
its own font's metric (`element.char_width`), which it declares (S8).

**`active(ref)`** names a role's active value (R5.1): the role is found as any reference finds it (S3.3: a
`surface.x` through the `Surface(S)` bindings in scope to `role.S.x`), and its value is the colour roles' value from
the bases of every layer, before any rule; a `Binding::Role` is not followed, as it is an element's value, not the
role's base.

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
table is the whole block. A shadow names an elevation by `Shadow::Elevation(key)`, not by reference (R8). A colour
literal or colour expression stated on a `Value<Paint>` field, or in `append`, is one `Color` layer with
`BlendMode::Normal`.

**Patches over a reference compose lazily** (R2.5). A `Fields` patch, or an `Append` (S6.1), over a value that is a
`Value::Expr` is kept on that reference (an internal composed value, not a public `Value` variant) and applied where
the value is evaluated, in R2.4's order: the referenced value's own bases first; then each `Fields` patch stated in a
layer's bases (an `Append` there is an error, S6.1), in layer order, so it composes with the bases, before every rule; then every rule, in the stack's
stated order (S8), the referenced value's own rules (a token's `Tokens` rules, the rules of the part a
`widget.<part>.<field>` names, which the query evaluates last, R2.5) and the patches stated in rules each at its
rule's position. So a base never comes after a rule: a reader's font patch on a slot (S9.5), a base, composes before
a preset's `Tokens` rule on the keyed text style the slot references (iOS's Bold Text on `font.weight`, a contrast
rule), which therefore still reaches every slot that references the style and every part that references the slot.
The checks follow the reference: a `Fields` patch over a reference that can resolve `Absent` or `Auto` is refused
(S12).

**Functions** (a closed set, R5.4; each implemented to the source named, pinned in the facts' sources, S17):

| Function | Result | Definition |
|---|---|---|
| `alpha(c, a)` | colour | `c` with its alpha multiplied by `a` |
| `mix(a, b, t)` | colour | interpolation from `a` (t = 0) to `b` (t = 1) in sRGB with premultiplied alpha, as CSS `color-mix(in srgb, a (1−t)·100%, b t·100%)` |
| `kde_effects(c, background, group, role_kind)` | colour | KColorScheme's effects of a group (`disabled`, `inactive`) on a role of kind `foreground`, `decoration` or `background` (the contrast effect only on the first two, against `background`), in KColorScheme's order; `background` is the group's starting background, the same input role the background's own effects start from, never the effected one, as KColorScheme applies the foreground and decoration effects before it replaces the background with its effected value (`kcolorscheme.cpp:383-389`): for the disabled group the set's active background (`active(role.<set>.background)`, Window's for `Selection`), for the inactive group the inactive start's background (the set's inactive input role `input.inactive.<set>.background_normal`, or in S9.1's `[WM]` default the scheme input `input.kde_scheme.header.inactive.background_normal`), for `Selection` the background `kde_selection_inactive` gives; from `token.kde.<group>`; the inactive group only while `token.kde.inactive_enabled`; it reads those tokens itself, listed as its operands; group `active` returns `c` unchanged. KColorScheme applies a group's effects over every group but the active one, a stated `[Inactive]` group included (`kcolorscheme.cpp:380-391` at the facts' pinned commit) |
| `kde_selection_inactive(selection, window, selection_background, role_kind)` | colour | where KColorScheme starts the Selection set's Inactive group (`kcolorscheme.cpp:304-316`, `:374-378`): while `token.kde.inactive_change_selection_color` holds (it reads that token itself, listed as its operand), `window`, and for role kind `background` `window` tinted toward `selection_background` by `KColorUtils::tint` with the amount KColorScheme passes there (the implementation's, from that source; the core states no platform value); otherwise `selection`. The preset passes the Selection and Window sets' inactive input roles and the Selection set's active background, and applies `kde_effects(…, inactive, …)` over the result, as KColorScheme does (K3.40, K1.173; R5.1). The Selection set's Disabled group starts from Window's colours whatever the switch (`:317-319`), a plain reference in the preset |
| `kcu_mix(a, b, bias)`, `kcu_tint(a, b, amount)`, `kcu_shade(c, luma, chroma)` | colour | `KColorUtils`, to `kcolorutils.cpp` |
| `adw_standalone(c, scheme)` | colour | libadwaita's standalone accent derivation in OKLab, to its stylesheet |
| `m3_role(role)` | colour | material-color-utilities' dynamic scheme, the role of that scheme; it reads `input.accent` (the seed), `token.material.{variant, spec_version, contrast_level}` and the colours' scheme itself, listed as its operands |
| `mul(x, y)`, `div(x, y)`, `add(x, y)`, `sub(x, y)`, `min(x, y)`, `max(x, y)`, `floor(x)` | number, length, duration or font size | arithmetic (a token times the speed factor; a font size times a factor) (ADD-A56) |
| `qt_lighter(c, factor)`, `qt_darker(c, factor)` | colour | `QColor::lighter`, `QColor::darker` in HSV, to qtbase `qcolor.cpp` (ADD-A50) |
| `kcu_overlay(under, over)` | colour | `KColorUtils::overlayColors`, to `kcolorutils.cpp` (ADD-A51) |
| `kirigami_tint(c, tint, alpha)` | colour | Kirigami's `ColorUtils.tintWithAlpha` (ADD-A52) |
| `kirigami_label_icon_size(font_height)`, `kirigami_badge_text(text, background)` | length, colour | Kirigami's `units.cpp` and `Badge.qml` (ADD-A53) |
| `ka_card_border(background, text)`, `ka_banner_line(type_text, frame)`, `ka_contrast_text(c)` | colour | kirigami-addons' FormCard background, banner and avatar code (ADD-A66) |
| `kqc_series_color(seed, count, index)`, `kte_minimap_mark(mark, background)`, `kte_mark_underlay(hue_from, background, foreground)` | colour | KQuickCharts' `ColorGradientSource.cpp`, KTextEditor's `kateviewhelpers.cpp`: the marks' HSV recolouring, and their underlay in HSL (one colour's hue and saturation at the background's lightness moved toward the foreground, K-P3-12†) (ADD-A57, ADD-A90) |
| `sf_tracking(font_size, design: TrackingDesign)` | length | Apple's size-to-tracking tables for SF Pro, SF Pro Rounded, New York (ADD-A54) |
| `uikit_prominence(c, prominence)` | colour | `UIColor.withProminence(_:)` (X-CO-5) |
| `gtk_contrast_pick(c, on_light, on_dark)` | colour | `on_light` where GTK finds `c` light, else `on_dark`, to `gtkcolorswatch.c`'s lightness test (GN-2.27) |
| `shadow_extent(shadow, side)` | length | how far a shadow reaches beyond the box on that side, over its layers, to GTK's `gtk_css_shadow_value_get_extents` (GN-1.282†) |
| `gtk_wheel_step(page_size)` | length | GTK's scroll step from the page size, to `gtkrange.c` / `gtkscrolledwindow.c` (GN-P3-15†) |
| `uifm_scaled(x, text_role)` | length | `UIFontMetrics.scaledValue(for:)` (ADD-A55) |
| `kde_accent(set: SurfaceSet, key: KdeSchemeKey, group: KdeGroup)` | colour | Plasma's accent derivation for one key of one colour set and group, from `input.accent` and the scheme inputs `input.kde_scheme.<set>.<key>`, to `plasma-workspace` `kcms/colors/colorsapplicator.cpp` `applyScheme()` (R25.1); `set` is one of KDE's seven colour sets (`Window`, `View`, `Button`, `Selection`, `Tooltip`, `Complementary`, `Header`), declared by the preset (S12). It branches on the scheme inputs' three lists for the scheme being resolved (S9.1): `input.kde_scheme.groups`, the sets the file has a group for; `.inactive_groups`, the sets whose group has an `[Inactive]` group; `.unset_keys`, the keys a group the file has leaves unset. Group `active` gives the active role, as `applyScheme()` writes it with the accent (`:183-245`, `:270-278`), every set but a missing `Header` written whether or not the file has a group for it (a set without one as a group whose every key is unset, below); group `inactive` the inactive input role, as `applyScheme()` and KColorScheme give it. For a set `inactive_groups` names, what `applyScheme()` writes into its `[Inactive]` group (`colorsapplicator.cpp:247-284` at v6.8.2): while the scheme tints (`token.kde.tint_factor` is `Some`), every such set's own file value `input.kde_scheme.<set>.inactive.<key>`, `Header`'s included, tinted toward the accent; while it does not (`None`), `Header`'s the file's active `Window` value `input.kde_scheme.window.<key>` (Plasma 6.8's substitute, R25.1) and another set's its own file value untinted; the accent never replaces an `[Inactive]` group's accent keys. For a set `inactive_groups` does not name, the set's active result, as KColorScheme reads a set without an `Inactive` group (`kcolorscheme.cpp:345-351`). Then, where the file has a `Header` group and `token.kde.titlebar_is_accent_colored` holds, `Header`'s background and foreground are what that option writes into `Header`'s `[Inactive]` group (the file's `Window` background tinted toward the accent by the scheme's factor, `applyScheme()`'s default factor where `tint_factor` is `None`, `:155`, `:273`, and the text on it, `:280-282`), over either branch and whether or not `inactive_groups` names `Header`, as `applyScheme()` writes that group either way; where `inactive_groups` does not name `Header`, KColorScheme reads that group's other keys from `Header`'s defaults, the `Window` set's values (`kcolorscheme.cpp:330-333`), so they are `Window`'s active results. **`Header` where the file has no `Header` group:** `applyScheme()` writes no `Header` group at all, neither the accent keys, the tint nor the title bar (`colorsapplicator.cpp:186-192`), and KColorScheme reads `Header` from kdeglobals' `[Colors:Window]` (`kcolorscheme.cpp:330-333`), in the `Inactive` group too, there being no `[Inactive]` group (`:345-351`): groups `active` and `inactive` both give `Window`'s active result for the key where `applyScheme()` writes one into `[Colors:Window]`, else `input.kde_scheme.header.<key>` (KColorScheme's `Header` default, S9.1); the accent-coloured title bar is then `[WM]`'s alone (`kde_accent_wm`). **A key the file leaves unset** (named by `unset_keys`, or any key of a set `groups` does not name): wherever `applyScheme()` reads it from the file (a tinted key, `:220`, `:254`; the background of the link's contrast check, `:203`, of the `Selection` and `Button` blends, `:231`, `:241`, and of the title bar, `:271`, `:321`), it reads an invalid `QColor()`, and the function reads the same, implemented from that code; where `applyScheme()` writes nothing for it, the result is what KColorScheme then reads from kdeglobals for it (the set's defaults, `Header`'s from kdeglobals' `[Colors:Window]`, `kcolorscheme.cpp:330-357`). An accent equal to Qt's `transparent` is no accent to `applyScheme()` (`:129-131`), and the function then gives what `applyScheme()` writes without one. Group `disabled` is refused (S12): disabled roles are KColorScheme's effects over the active values (R5.1), not an accent derivation. The function branches only on `Option`s, bools, enums and those three lists, never on an absent value (S4.8) |
| `kde_accent_wm(key: KdeWmKey)` | colour | the rest of the same derivation, `applyScheme()`'s `[WM]` group (`colorsapplicator.cpp:287-338` at v6.8.2), one route with `kde_accent` (one implementation of `applyScheme()`, R25.1, its rules for an unset key and a transparent accent included): it starts from the scheme input `input.kde_scheme.wm.<key>`, what `applyScheme()` writes with no accent (S9.1: the file's `[WM]` key, else its `Header` default); then the inactive background and foreground, while the scheme tints (`token.kde.tint_factor` is `Some`), are the file's `Window` background `input.kde_scheme.window.background_normal` tinted toward the accent and the text on it (`tintColor`, `accentForeground`); the active background and foreground, while `token.kde.titlebar_is_accent_colored` holds, are that `Window` background accented as the title-bar option writes it and the text on it (`accentBackground`, `accentForeground`), else, while the scheme tints, the tinted `Window` background and the text on it; any key neither option changes keeps the start value. `KdeWmKey` (S4.8) names the four keys Breeze reads (`breezehelper.cpp:98-102`) |
| `name_index(text, count, method: NameIndexMethod)` | number | the index a platform's avatar hashes a name to; `NameIndexMethod = Kirigami \| Adwaita` (Kirigami's `nameutils.cpp`, libadwaita's `adw-avatar.c`), each to its source (R25.3; kirigami-addons' own `nameutils.cpp` is compared with Kirigami's in the value phase and becomes a member if it differs); `TrackingDesign = SfPro \| SfProRounded \| NewYork` |
| `pick(index, c0, c1, …)` | colour | the colour at `index` (R25.3) |

A function is added to this table only with the platform rule that needs it and its source (R5.4). How each KDE
function treats a translucent input (mixes, keeps or ignores alpha) is read from its source in the value phase and
recorded in its facts (R5.4). A platform
that computes a stored colour by composition (KDE's `overlayColors`) gets a function; composition drawn over what
lies beneath is a paint layer (R5.4, R6). Type checking: every operand and result is typed (colour, length,
duration, angle, number, font size, text, bool, name); a name operand is checked against the enum or declared key
set the function's parameter names (`design: TrackingDesign`, `group: KdeGroup`, `role_kind: KdeRoleKind`, `kde_accent`'s `key: KdeSchemeKey`,
`kde_accent_wm`'s `key: KdeWmKey`;
`m3_role`'s `role` is a declared keyed colour role, `uifm_scaled`'s `text_role` a `TextRoles` slot or a declared text
style); a bare number takes the type of
the other operand; `mul(T, number)`, `div(T, number)`, `div(T, T)` (a number), `add(T, T)`, `sub(T, T)`, `min(T, T)`, `max(T, T)` and `floor(T)` give a `T`;
a mismatch is an error when the layer is added (S12). A function that reads inputs or reader values itself
(`kde_accent` reads `input.accent`, `input.kde_scheme.<set>.<key>`, `.<set>.inactive.<key>`, `.groups`, `.inactive_groups` and
`.unset_keys`, `token.kde.tint_factor` and `titlebar_is_accent_colored`, every one stated, S4.8, S9.1; which roles depend on the accent is
S9.1's list; `kde_accent_wm` reads `input.accent`,
`input.kde_scheme.wm.<key>`, `input.kde_scheme.window.background_normal`, `.unset_keys` and the same two tokens; `uifm_scaled` the reader's content-size
category, S8, S9.4) lists them, and the checks treat them as its operands. A font size reaches a function in logical
pixels, converted and scaled (S9.4), except in an expression stated on a `Font.size` (a part's `text.font.size`, a
text style token's), where every font-size operand (another style's or slot's `font.size`, `widget.<part>.font_size`)
is converted and not yet scaled, and the result is scaled once (S9.4); a `FontSize` widget value, not text-scaled
(S1), reads its font-size operands converted only.

**Totality** (the query cannot fail, R2.8): `alpha` clamps the resulting alpha to `0..=1`, as CSS Color 5's relative
colours do (a factor above one is valid: libadwaita multiplies a shade's alpha by more, GN-1.210); `mix` clamps its
fraction to `0..=1`; `pick` floors the index and takes its Euclidean remainder by the number of its colours, at least
one (S12); every count operand of zero is one (`name_index`, `kqc_series_color`); a length or duration that comes out
negative where the field cannot be negative is zero; a computed `Material.saturation`, `MotionTokens.speed_factor` or
spring `time_scale` below zero is zero; a number that becomes an integer (a `Value<u16>` weight, a `Value<u32>`
count) is floored and clamped to the integer type's range, a negative one to zero; a loop whose period evaluates to
zero does not run (the part shows its style without the loop's keyframes: R9's animations off sets the speed factor,
and so a scaled period, to zero); a non-finite result, or non-finite element data, is zero. A platform function
follows its source's own clamping first. An expression naming `element.<data>` is re-evaluated by each
query, so a connector queries again when the data changes, as for a state (R25.3).

### S2.3 Paint (R6)

```rust
pub struct Paint { pub layers: Vec<PaintLayer> }             // bottom first; at least one layer (S12);
                                                             // "paints nothing" is `Value::Absent`
pub struct PaintLayer { pub kind: PaintKind, pub blend: BlendMode }
pub enum PaintKind {
    Color(Value<Color>),
    Linear { angle: Deg, stops: Vec<Stop>, repeat: bool,
             extent: Option<Value<Px>>,       // stops over this many px from the start edge (ADD-A31); None: the box
             offset: Option<Value<Px>> },     // the stops shifted along the axis by this many px (K1.110); None:
                                              // not shifted
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
pub enum BlendMode { Normal, Mask,           // Mask: the layer's alpha masks the layers beneath it (below)
                     Keyed(Key) }            // platform blending modes as the facts name them (macOS vibrancy)
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

A rule's patch either sets a paint (`fill = …`) or appends layers over `base` (`fill.append = [ … ]`) (R6). Angles
follow CSS: `Linear.angle` is `linear-gradient()`'s angle, `0deg` pointing toward the top of the box and angles
increasing clockwise; `Conic.start_angle` is `conic-gradient()`'s `from` angle on the same convention, its stops
running clockwise from it. A gradient has at least two stops (S12): one colour is a `Color` layer, the one route. A
repeating gradient repeats its stops beyond the last (stripes, K1.110). A linear gradient's `offset` moves its stops
along its axis, toward the axis's end for a positive offset (a repeating one wraps; a non-repeating one extends its
end colours, as without an offset), so a loop's keyframes, or a transition, that change it move the pattern: Breeze's
busy stripes, offset by one pattern period over the loop (K1.110). It interpolates as a length. A paint is drawn in
the box's own coordinates and is not mirrored in right-to-left, as a CSS gradient is not; a platform that reverses
one there (Breeze's stripes run the other way, `breezehelper.cpp:1245-1247`) states it by a rule under the direction
(S7, R16). Corners, stroke edges and shadow offsets are physical likewise (S2.4, S2.5).

A layer with `BlendMode::Mask` is not drawn: its alpha, at each point, multiplies the alpha of the layers beneath it
in the same paint (Porter-Duff destination-in, CSS's `mask` with an alpha mask), so a gradient from opaque to
transparent stated over a `Material` layer fades the material out (Apple 26's soft scroll edge, MAC-X05, C-18;
`scroll_view.edge_effect`); the layers above it are drawn normally. A paint whose bottom
layer is a `Mask` masks nothing and is refused (S12).

### S2.4 Corners, stroke (R7)

```rust
pub struct Corners { pub top_left: Radius, pub top_right: Radius, pub bottom_right: Radius,
                     pub bottom_left: Radius, pub curve: CornerCurve }
pub enum Radius {
    Px(Value<Px>),
    Full { maximum: Value<Px> },          // a capsule: half the smaller side of the part's box, at most `maximum`
                                          // (UIKit's `capsule(maximumRadius:)`, X-CR-2); `maximum` may be absent
    Concentric { minimum: Value<Px> },    // the container's radius less the inset, at least `minimum` (R7);
                                          // `minimum` may be absent
    FollowPart,                           // a focus indicator's: the part's radius plus the offset, concentric
}                                         // (R12); only in `FocusIndicator.corners`, and refused where the
                                          // corner's two sides' offsets differ (S12)
pub enum CornerCurve { Circular, Continuous }
pub struct Stroke { pub top: Edge, pub right: Edge, pub bottom: Edge, pub left: Edge,
                    pub position: StrokePosition,
                    pub outset: Value<Px> }      // the gap between the box and an `Outside` stroke (iOS's
                                                 // `strokeOutset`, CD-1); absent where no outset is drawn
pub struct Edge { pub width: Value<Px>, pub paint: Value<Paint>, pub dash: Value<DashPattern>,   // ADD-A32
                  pub snap: PixelSnap }                                                    // ADD-A33
pub enum DashPattern { Solid, Dotted, Dashed { on: Px, off: Px } }   // a drawn edge states its pattern, `solid`
                                                                    // included: the one route for a solid line;
                                                                    // `dash` is absent only on an edge not drawn
                                                                    // (S6.3)
pub enum PixelSnap { None, ToDevicePixel, AtLeastOneDevicePixel }   // the width stays logical
pub enum LineCap { Butt, Round, Square }
pub enum LineJoin { Miter, Round, Bevel }
pub enum StrokePosition { Inside, Center, Outside }
```

A part's outline is its `corners`, unless `PartStyle.shape` names a shape: `ShapeKind { Material(MaterialShape) }`, a
closed set implemented once in native-theme to `androidx.graphics.shapes` and `MaterialShapes.kt` at a pinned commit
(R25.5). `MaterialShape`'s variants are exactly the shapes `MaterialShapes.kt` defines at that commit, in that file's
order, each named as the file names it, in `UpperCamelCase` (TOML: snake_case, S11); the value phase writes the list
from the file when it pins the commit (it is not stated here, as no source for it was read). The shapes are code, as
placement strategies are (R16), in `native_theme::shapes`:

```rust
pub struct Path { pub commands: Vec<PathCommand> }   // in `Px`, in the coordinates of the `rect` it was made for;
                                                     // each `MoveTo` starts a closed outline; filled by the
                                                     // non-zero rule
pub enum PathCommand { MoveTo(Point), LineTo(Point), QuadTo(Point, Point), CubicTo(Point, Point, Point), Close }
                                                     // QuadTo: control, end; CubicTo: two controls, end
pub enum Outline { Corners(OutlineCorners), Shape(ShapeKind) }
pub struct OutlineCorners { pub radii: [Px; 4],          // every radius as the connector computed it at layout (a
                            pub curve: CornerCurve }     // stated `Radius::Px`, or S2.4's functions), in the order
                                                         // top-left, top-right, bottom-right, bottom-left
pub fn outline(shape: &ShapeKind, rect: Rect) -> Path;
pub fn morph(a: &Outline, b: &Outline, t: Fraction, rect: Rect) -> Option<Path>;   // t = 0: `a`, t = 1: `b`;
                                                     // None where either is `Corners` with a `Continuous` curve
```

As an `Outline`, a part is `Shape` where its `shape` is stated, else `Corners`, whose path is the rounded
rectangle `androidx.graphics.shapes` builds from the four radii (`RoundedPolygon.rectangle`), which draws circular
corners only. So a `Corners` outline is defined for `CornerCurve::Circular`: a part whose `shape` can be stated is
refused where its corners' curve can be `Continuous` (S12; only `material-3` states shapes, and its corners are
circular), so the model never asks `morph` for a continuous outline, and a direct call with one gets `None`. An `Interpolate`
transition of the `Shape` group (R25.5's transition of kind morph) morphs from the old outline to the new, so a
change between plain corners and a shape morphs too; a change of corners alone, with no shape on either side, is
the `Corners` group's.

`Full`, `Concentric` and `FollowPart` resolve as they are; the connector computes each such corner at layout, from
its own layout's numbers, with the pure functions of `native_theme::corners`:

```rust
pub fn full(width: Px, height: Px, maximum: Option<Px>) -> Px;                  // half the smaller side, at most
                                                                                // `maximum`
pub fn concentric(container_radius: Px, inset: Axes<Px>, minimum: Option<Px>) -> Px;
pub fn follow_part(part_radius: Px, offset: Px) -> Px;                          // part_radius + offset, at least 0
```

A `Concentric` corner's container is the nearest enclosing part or element whose catalogue entry lists `corners`,
decided by the catalogue, not by resolved values (a root that lists no corners, one that draws nothing, is skipped;
a full-screen part's container is the display, S4.6). The container's own corners may be `Full`,
`Concentric` or `FollowPart`, numbers only after layout (iOS 26 nests concentric containers): the `Scope` identifies
the container and holds its stated corners (S8), and the connector passes as `container_radius` the radius it
computed for the container's matching corner, which it has, as it lays out and queries top-down. A square container,
or the display with no radius (S4.6), passes 0, so the corner is its minimum, or square without one. `inset` is the
part's distance from the container's edges at that corner, per axis; `concentric` returns the larger of
`container_radius` less the inset and `minimum` (0 when absent) where the two distances are equal. Concentric corners
are Apple's (iOS 26, macOS 26), so where the distances differ `concentric` follows UIKit's rule, stated as UIKit's:
read in the value phase from `UICornerConfiguration` and `UIView.effectiveRadius(corner:)` (X-CR-2) and cited in the
function's doc comment (another platform's concentric corners, a full-screen window following the display, MAT-W5†,
sit at no inset).

TOML shorthands: `corners_px = <n>` sets the four corners to the plain radius `Radius::Px` (no other variant has a
shorthand: a capsule states each corner, `top_left = { full = { maximum_px = … } }`, S11); the curve beside it is
`corners.curve`, the one second key S12 accepts for that field. The `stroke` table's shorthand keys `width_px`,
`paint`, `dash` and `snap` each set that field on all four edges; the table's other keys are `position`,
`outset_px` and the edges' own tables (`top`, `right`, `bottom`, `left`). A table is written either with dotted keys
(`stroke.width_px = <n>`, `stroke.paint = "…"`, `stroke.dash = "solid"`, `stroke.snap = "to_device_pixel"`,
`stroke.position = "inside"`, `stroke.outset_px = "absent"`) or as one inline table holding all its keys (`stroke = {
width_px = <n>, paint = "…", dash = "solid", snap = "to_device_pixel", position = "inside", outset_px = "absent"
}`), never both, as TOML cannot extend an inline table. An edge's table states no field a shorthand key sets (a
duplicate key, S12): an edge that differs is stated with all four edges' tables and no shorthand.

Where two drawn edges meet, the corner square (or, under rounded corners, the corner's arc) is painted once, never
by both edges, so a translucent stroke's corner is no darker than its sides: where the two edges' paints and widths
are equal it is that edge's; where they differ it is divided between them along the line from the outer corner to
the inner one, as CSS divides a border's corner between two border colours, each half painted by its own edge (and
dashed by its own pattern).

**Right-to-left.** A `Corners`' corners (`top_left` …) and a `Stroke`'s edges (`top` … `left`) are physical: they
are not mirrored under right-to-left, as CSS's physical `border-*-radius` and `border-*` properties are not, and
neither is a shadow's `x` offset (S2.5). A platform that mirrors one of them (a segmented control's first segment
rounding its start corners in either direction, a sidebar pane's stroke on its content side) states it by a rule
under the direction (`dir rtl`, S7). What mirrors with the layout is the layout's own geometry, `Sides` (S1),
alignments (`Alignment`, and the start and end of every other alignment value), placement (S13: `PlacementSide`,
`PlacementAlign`, and its `Sides`), a presentation's start or end edge (`EdgeSide`), and of a part's drawing only a
translation (a transform's `translate_x`, a presentation's `Translate.dx`), for S2.6's reason: it is a displacement
in the mirrored layout, while a corner, an edge, a transform's origin or a light source belongs to the drawing.

**Who mirrors: the query.** Values are stated for a left-to-right layout; the query returns them physical, already
mirrored for the element's direction (its own, else its scope's, S8): for `rtl` it swaps `left` and `right` in
`Sides`, swaps `Start` and `End` in the alignments, `PlacementSide`, `PlacementAlign` and `EdgeSide` it lists above,
and negates `translate_x` and `Translate.dx`, so in a resolved value `left` and `Start` are the physical left. A
rule under `dir rtl` states these values the same way, for the left-to-right layout, and the query mirrors them with
the rest, so a platform that keeps one unmirrored states its mirror image there (a translation kept: the rule's
`translate_x` negated); the physical values (corners, stroke edges, shadow offsets, paints, the origin) a rule under
the direction states as drawn. A connector draws what it receives and never mirrors these again: where its toolkit mirrors start and end values itself
(Qt's and GTK's layouts), it passes them in the toolkit's physical form (Qt's `AlignAbsolute`) or turns that
mirroring off, and its contract records which (S21.3). `place()` takes the parameters so resolved and does not mirror
them again (S13). A corner that reads a `Sides` value (a `FollowPart` corner's offsets, S2.10) reads the resolved,
physical members on its own edges.

### S2.5 Shadow (R8)

```rust
pub enum Shadow { Layers(Vec<ShadowLayer>), Elevation(Key) }   // Key: an entry of the elevation ramp (S4), which
                                                                 // may be empty (Material's level 0: no shadow);
                                                                 // `Layers` has at least one; no shadow is `Absent`
pub struct ShadowLayer { pub x: Value<Px>, pub y: Value<Px>, pub blur: Value<Px>,   // x: physical, not mirrored
                         pub spread: Value<Px>, pub color: Value<Color>, pub inset: bool }   // in right-to-left (S2.4)
```

The query resolves an elevation reference to that entry's layers, so a connector receives layers only, to draw and
to interpolate (a change of entry, WinUI's pressed control dropping from 2 to 1, W3-02, is a change of layers). A
reference to an empty entry resolves to `None`, as `Absent` does, so "no shadow" reaches the connector in one form; a
change to or from it interpolates as layers that appear or disappear do (S2.7).

### S2.6 Transform (R9)

```rust
pub struct Transform { pub scale_x: Value<f32>, pub scale_y: Value<f32>, pub rotate: Value<Deg>,
                       pub translate_x: Value<Px>, pub translate_y: Value<Px>,
                       pub origin: (f32, f32) }                // fractions of the part's box from its top-left
                                                               // corner, physical (below)
```

A transform's translation is stated for a left-to-right layout and mirrored with it, as `Sides` and alignments are,
the one part of a part's drawing that mirrors (S1, S2.4, R16):
under right-to-left the query returns `translate_x` negated (S2.4), so a part moved toward its end moves toward the
end in either direction (an indeterminate bar's moving indicator, a thumb's offset). The reason is R16's: the toolkit
mirrors the layout, and a translation is a displacement in that layout, so stating it per direction would state one
motion twice. `origin`, `rotate`, `scale_x`, `scale_y` and `translate_y` are not mirrored: the origin is physical, as
the corners are (a fraction measured from the box's left edge in either direction), so a rotation or scale about an
origin off the centre is not mirrored either; a platform that does otherwise (a rotation reversed or an origin
mirrored under right-to-left, a translation kept) states it by a rule under the direction (S7).

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
    pub part: Option<PartRef>,                // None: the whole element; a part of the widget or of one of its
                                              // element parts (a tab strip's indicator sliding, `@indicator_move`)
    pub kind: PresentationKind,
    pub duration: Value<Ms>, pub delay: Value<Ms>, pub timing: Value<Timing>,   // duration may be absent: a spring
    pub stagger: Value<Ms>,                  // ms between items, where the element presents items
}
pub enum PresentationEvent { Appear, Disappear, Move, Keyed(Key) }   // widget-specific events, declared (S10)
pub enum RevealOrigin { Anchor, Center, Edge(EdgeSide) }
pub enum PresentationKind { Fade, Slide { from: EdgeSide, distance: Value<Px> }, Translate { dx: Value<Px>,
                            dy: Value<Px> }, Scale { from: f32 }, Morph, ClipReveal { from: RevealOrigin },
                            Keyed { key: Key, params: Vec<(Key, Value<Scalar>)> }, Instant }   // ADD-A25, A26
pub enum EdgeSide { Top, Bottom, Start, End, TowardAnchor }   // TowardAnchor: WinUI's menus (ADD-A25)
pub struct Loop { pub period: Value<Ms>, pub timing: Value<Timing>, pub keyframes: Vec<Keyframe> }
pub struct Keyframe { pub at: Fraction, pub part: Option<PartRef>,    // None: the part the loop is on; a loop
                                                                       // that is a widget's or component's value
                                                                       // is on no part, so each of its keyframes
                                                                       // names one (S12)
                      pub timing: Value<Timing>,                       // to the next keyframe; may be absent: the
                      pub patch: PartStylePatch }   // loop's timing. `at` a fraction of the period; the part's
                                                        // style, a shape included (R25.5)
```

Semantics (R9): the query is stateless: the connector compares successive query results per element and part, and
a transition starts when a property's resolved value changes; the transition stated by the style
after the change governs, both directions; a later rule's transition for the same property group wins; a property
whose group has no transition in that style takes its new value at once, in the frame of the change (no
transition is no animation, never a toolkit's default one); appearing
layers interpolate from zero alpha, layers match by stack position, colours interpolate in premultiplied sRGB;
a material giving way to a colour crossfades; a part that becomes drawn enters from transparent with its own
geometry, and one that becomes absent leaves to transparent, by the transitions of the style it goes to or comes
from. A `Geometry` transition also starts when an element's or a part's laid-out box
changes (a switch's thumb moving to its new end), when the connector's own layout moves it. During a theme change (S4.3) the theme-change token governs, the new resolution's (as the style after a change governs). A
state-triggered icon animation (WinUI's AnimatedIcon, GtkSvg's states, an SF Symbol effect on a change) is a
`TransitionKind::Keyed` transition of the `Icon` group; a repeating one is `IconStyle.animation` (R25.12).

Today's `AnimatedIcon` goes (R9's loops extend it), with its types (`FramesData`, `FrameList`, `TransformData`,
`TransformAnimation`, `EmptyFrameListError`) and the loaders that return it (`load_icon_indicator`, the icon loaders'
`load_indicator`, `icon_indicator`), and the bundled pre-rotated frames of `spinners.rs`: a spinner is the `spinner`
widget (the catalogue), its look a glyph or icon (S2.9) and its motion the widget's `loop`, whose keyframes set the
part's `transform` (a spin) or its `glyph` (a sequence of frames, GtkSpinner's animated icon), each at the platform's
period (S19.3).

### S2.8 Text style and font (R10)

```rust
pub struct Font {
    pub family: Value<String>,                // fallbacks are the text tokens' (S4.4, ADD-A41)
    pub size: Value<FontSize>,                // today's FontSize (pt or px); a point size becomes logical px by
                                              // the font DPI (S9.4)
    pub weight: Value<u16>,                   // the only route for weight: `axes` may not hold `wght` (S12)
    pub style: Value<FontStyle>,
    pub width: Value<f32>,                    // percent of normal, as the OpenType `wdth` axis, and the only
                                              // route for it: `axes` may not hold `wdth` (S12)
    pub axes: Vec<(AxisTag, AxisValue)>,
    pub features: Vec<(FeatureTag, u32)>,     // never `smcp`: small caps is `TextCase::SmallCaps` (S12)
}
pub enum AxisValue { Fixed(f32), FollowsSize }  // SF's and Segoe UI Variable's optical size
pub struct TextStyle {
    pub font: Font,
    pub scaling_role: Option<TextRole>,       // whose Dynamic Type curve scales it (S9.4); None: body's
    pub letter_spacing: Value<FontSize>, pub word_spacing: Value<FontSize>,   // word spacing: K3.46; pt or px,
                                              // may be negative, converted and scaled as a line height, or an
                                              // expression on font_size, which follows the scaled font (S9.4)
    pub line_height: Value<FontSize>,         // pt or px (converted and scaled as a font size, S9.4), an expression on
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
                   Fade { start: Option<FadeLength>, end: Option<FadeLength>,   // a fade at either or both edges
                                                              // (GN-1.129); None: no fade at that edge
                          under: Option<FadeUnder> } }        // None: the fade ends at the text's own edge
pub enum FadeLength { Px(Value<Px>), Fraction(Value<Fraction>) }   // in px, or a fraction of the box (ADD-A38)
pub enum Wrap { None, Word, Char }
pub enum FadeUnder { Part(PartRef), Element { widget: WidgetRef, variant: Option<VariantRef> } }
                                              // the fade ends under a part, or under a nested element (a
                                              // tab's close button, GN-1.129)
```

A decoration's `offset` is the distance from the baseline to the line's top edge, positive downward (an overline's
and a strike line's are negative), and its `thickness` the line's height. Where the platform draws the line from the
font's own metrics (Pango's underline and strikethrough metrics, Qt's `QFontMetrics::underlinePos`,
`strikeOutPos` and `lineWidth`, AppKit's `NSFont.underlinePosition` and `underlineThickness`, DirectWrite's font
metrics, Android's `Paint.getUnderlineThickness`), the preset states them as references to the part's font metrics
(`thickness = "= element.font_underline_thickness"`, `offset = "= element.font_strikeout_offset"`, S2.2, S8), which
the connector measures in this convention from its toolkit's; a number is stated only where the platform draws a
fixed line (R5.4: a value that follows the font cannot be stated as a result).

The text ramp (S4.4) holds the semantic `TextRole` slots and the keyed platform styles. Weight, width and small caps
each have one route (principle 2): `Font.weight`, `Font.width` and `TextCase::SmallCaps`; a connector drawing with a
variable font maps the fields onto its `wght` and `wdth` axes itself.

### S2.9 Icon style and glyph (R11)

```rust
pub struct IconStyle { pub size: Value<Px>, pub fill: Value<Paint>, pub rendering_mode: Value<RenderingMode>,
                       pub weight: Value<u16>,             // the only route for weight: `axes` may not hold `wght`
                       pub scale: Value<Key>,              // SF Symbols' named scales, declared in `symbol_scales`
                       pub axes: Vec<(AxisTag, AxisValue)>,  // Material Symbols' fill, grade, optical size (MAT-X27)
                       pub state_effect: Value<Key>,       // KDE's icon effect for the state the rule selects (K3.60)
                       pub symbol_variants: Vec<Key>,      // SF Symbols' `.fill`, `.circle`, … (R25.8)
                       pub status_colors: Vec<(Key, Value<Paint>)>,   // a symbolic icon's success, warning, error
                                                                       // colours (ADD-A37)
                       pub animation: Value<Loop> }        // a repeating icon animation; may be absent (R25.12)
pub enum RenderingMode { Template, Monochrome, Hierarchical, Palette, Multicolor, Original }
pub struct Glyph { pub source: GlyphSource, pub size: Value<Px>,
                   pub fill: Value<Paint>,              // absent only on a stroked-only path (S6.3)
                   pub stroke: Value<GlyphStroke>,      // absent where the glyph is not stroked
                   pub axes: Vec<(AxisTag, AxisValue)>,  // a symbol font's axes (MAT-X27)
                   pub draw_extent: Value<Fraction>,      // the fraction of the path drawn (Breeze's check mark, ADD-A35)
                   pub draw_start: Value<Fraction>,       // where along the path drawing starts (a spinner's arc)
                   pub mirroring: GlyphMirroring }   // Segoe Fluent's per-glyph mirroring (R11)
pub enum GlyphSource {
    FontCodePoint { family: String, code_point: u32 },
    ThemeIcon { name: String, fallbacks: Vec<String> },   // ADD-A36
    Path { data: String, view_box: ViewBox },           // SVG path syntax in the source's own coordinates, mapped
                                                        // onto the glyph's box at `size` as SVG's viewBox (cited
                                                        // in the preset's comment, S11)
    NamedSystemImage { name: String },                   // a name the platform states for that purpose
    SystemControl { control: SystemControl, replaces: Vec<PartRef> },   // the system's rendering (S15)
}
pub struct GlyphStroke { pub paint: Value<Paint>, pub width: Value<Px>, pub cap: LineCap, pub join: LineJoin }
                                            // a stroked path's outline (K1.184†, ADD-A34)
pub enum Mirroring { Never, Always, ExceptLanguages(Vec<LanguageTag>),
                     SwapWith(IconRole) }   // under right-to-left the role takes another role's icon
                                            // (KStandardAction's Back/Forward, Prior/Next, K-P4-12†)
pub enum GlyphMirroring { Never, Always, ExceptLanguages(Vec<LanguageTag>) }   // a glyph names no icon role,
                                            // so `SwapWith` is not representable there
```

Mirroring per icon role is `token.icons.mirroring`, stated per preset (R11; Material
mirrors some icons in Arabic and Farsi but not Hebrew). Icon lookup takes the element's direction and
language (GTK's `-rtl` and `-ltr` names, SF Symbols' localized forms; R25.8), from the scope its part's query returned
(`ResolvedTheme::icon`, S16).

`IconStyle.fill` is the paint the platform's recolouring gives an icon (a symbolic icon, a template image, a symbol in
its rendering mode's colours). It may be absent where `rendering_mode` is `Original`, which recolours nothing, and only
there (an icon-size context keeps its own mark, S4.5): S12 refuses it absent under any other mode, following references
and rules as for any absent operand.

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
pub enum FocusVisibleWhen { Keyboard { timeout: Option<Value<Ms>> },   // None: no timeout (Breeze, K1.21)
                           AnyFocus, FocusWithin, Never }                 // ADD-A16
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
    pub shape: Value<ShapeKind>,             // a shape replacing the corners' outline; may be absent (R25.5)
    pub shadow: Value<Shadow>,
    pub opacity: Value<Fraction>,                 // applied to the element and its descendants as a group
    pub transform: Value<Transform>,         // likewise
    pub text: Value<TextStyle>,
    pub icon: Value<IconStyle>,
    pub glyph: Value<Glyph>,
    pub focus: Value<FocusIndicator>,
    pub padding: Sides<Value<Px>>,
    pub margin: Sides<Value<Px>>,            // outside the box; may be negative (GN-1.18, GN-1.73, GN-1.119)
    pub min_width: Value<Px>, pub min_height: Value<Px>,   // `auto` on every part: no minimum beyond the content
    pub max_width: Value<Px>, pub max_height: Value<Px>,   // `auto` on every part: no maximum, the content
                                                           // decides (resolves `Length::Auto`, S6.1)
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
field, its plain enum and bool leaves included (a listed `text` states its `case`, `wrap`, `elision`, …), its
`Option` leaves (a listed `text` states `max_lines`, `"none"` for no limit, S11), and its list leaves (a `Vec`: a
listed part's `bindings` and `transitions`, a listed `text`'s `font.axes` and `font.features`, an icon style's
`symbol_variants`). A list is never `Absent`: where the platform has none it is stated empty, `[]`, with its citation,
as the catalogue's *Entry format* says; left out it is missing, as any leaf. An `Option` inside a list item or an
enum variant's payload (a gradient's `extent`, a fade's `under`, a keyframe's `part`) is not a leaf of the field, and
left out it is `None`. A widget's first part
in the catalogue is its root part; the root part's bindings apply to all the widget's parts and its descendants
(R5.2), and the other parts' bindings to themselves and their descendants. Every part of a widget is queried with the
widget's element (its states and siblings, S8): the root part in the scope the element is queried in, and every
other part in the scope returned by the query of the innermost part enclosing it (S8), which already holds the root's
bindings, once, so no query applies a binding twice (a `Binding::Role` whose value reads the role it binds would
otherwise change). Parts nest as the catalogue nests them (a window's title inside its title bar); a part's bindings, opacity and
transform apply to the parts nested in it and their elements. An element part's query sees its widget's variants
as well as its own (a list row matches the list's `sidebar` variant). An *(element)* part nested in a plain part is queried in the root's
scope with the plain part's bindings, opacity and transform applied. A part marked *(element)* in the catalogue (a
caption button, a tab) is an element of
its own, queried in the scope of the element that contains it (the root part's, or an enclosing *(element)* part's:
a caption button inside the caption-button group) with its own states, and the parts the catalogue names as belonging to it are queried with it; a part merely nested in it is queried with the widget's element. A
block-typed field set to a reference takes the referenced token's or field's value where it is used, with any
`Fields` patches over it composed in R2.4's order, bases before rules (S2.2). `CursorShape` is CSS's
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
`Surface(S)` binding decides; a `Surface(S)` answer becomes the lookup `role.S.x`, walked again from the querying
element outward (so a nearer `Set(S, x)` binding counts, R5.2), and with no binding at all it is `role.window.x`. (2) `role.S.x` (and `role.global.x`, `role.@x`): walk
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
pub struct Caret { pub color: Value<Color>, pub width: Value<Px>,   // its shape: the caret parts' rules
                   pub blink_on: Value<Ms>, pub blink_off: Value<Ms>, pub blink_timeout: Option<Value<Ms>>,   // None: the blink never stops
                   pub blink_style: BlinkStyle,                                         // ADD-A24
                   pub secondary: Value<Color>,          // the bidirectional secondary caret; may be absent
                   pub shown_in_read_only: Value<bool>,
                   pub solid_while_typing: Value<bool> }                                // X-CA-2
pub enum BlinkStyle { Hard, Fade }
pub struct Behavior { pub double_click_time: Value<Ms>, pub double_click_distance: Value<Px>,
                       pub drag_threshold: Value<Px>, pub hover_open_delay: Value<Ms>,
                       pub key_repeat: Option<KeyRepeat>,   // None: keys never repeat (the user turned it off)
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
pub struct KeyRepeat { pub delay: Value<Ms>, pub interval: Value<Ms> }   // neither may be absent
pub struct TouchBehavior { pub slop: Value<Px>, pub paging_slop: Value<Px>, pub double_tap_time: Value<Ms>,
                            pub double_tap_slop: Value<Px>, pub delay: Value<Ms> }
pub enum Mnemonics { Never, OnAlt { delay: Value<Ms> }, Always }
pub struct ScrollPhysics { pub deceleration: Value<ScrollDecay>, pub overscroll: Value<Timing>,
                           pub overscroll_effect: OverscrollEffect, pub overscroll_extent: Value<Px>,  // ADD-A29
                           pub min_fling_velocity: Value<f32>, pub max_fling_velocity: Value<f32>,   // px/s: a release
                                                 // slower than the minimum does not fling, a faster one flings at
                                                 // the maximum (MAT-P3-16†)
                           pub wheel_step_duration: Value<Ms>,       // smooth wheel scrolling (MAC-P4-4†)
                           pub wheel_step_duration_max: Value<Ms>,
                           pub touchpad_factor: Value<f32>,          // touchpad deltas' multiplier (GN-P3-15†)
                           pub touchpad_deceleration: Value<ScrollDecay>, pub touchpad_timing: Value<Timing>,
                           pub touchpad_velocity_samples: Value<u32>,   // touchpad inertia: deceleration, curve,
                                                                     // and how many of the latest deltas its start
                                                                     // velocity is estimated from (K-P3-18†)
                           pub wheel_step_timing: Value<Timing>,     // the duration grows with the distance
                                                                     // up to the maximum (K-P3-18†, MAC-P4-4†)
                           pub snap: Value<bool> }
pub enum OverscrollEffect { None, RubberBand, Stretch, Glow }   // None: the platform draws none
pub enum ScrollDecay { /* closed: one variant per platform deceleration, each holding the parameters its source
                          defines as stated `Value` fields, written in the value phase (below) */ }
```

A deceleration is a platform's own code, not one number with one meaning: the platforms' decelerations are
different physical models (a per-millisecond rate, a friction, a constant deceleration, a spline), so one `f32` read
the same way by every connector would convert some of them approximately, and a model may have more than one
parameter, each user- or platform-stated. `ScrollDecay` is therefore a closed set, as `PlatformCurve` is (S2.7): one
variant per platform deceleration the traceability's scroll rows cite, named for its source and implemented once in
`native_theme::scroll` to it, the variant holding exactly the parameters its source defines (each a `Value`, with its
unit) and defining how the velocity falls over time and how a touchpad's start velocity is estimated from its samples;
the value phase writes the variants and their parameters when it pins those sources, as it writes `MaterialShape`'s
(S2.4). A connector applies them through the pure functions, so every toolkit scrolls by the same numbers and the
facts gate compares each variant's parameters:

```rust
pub fn velocity(decay: &ResolvedScrollDecay, initial: f32, elapsed: Ms) -> f32;   // px/s at `elapsed` after release
pub fn distance(decay: &ResolvedScrollDecay, initial: f32, elapsed: Ms) -> Px;    // travelled since release
pub fn start_velocity(decay: &ResolvedScrollDecay, samples: &[(Ms, Px)]) -> f32;  // from the latest deltas, at most
                                                                                  // `touchpad_velocity_samples`
// ResolvedScrollDecay: ScrollDecay with every parameter evaluated; each function total (S2.2's rules)
```

The caret is also a part (`caret`) of the text widgets, whose base refers to `token.caret`, so rules apply to it by
widget, drop target and inactive window; the text widgets' further parts (preedit, misspelling, selection handles,
…) and the shared `overwrite` and `composing` flags are the catalogue's; hiding while text is selected and the
overwrite block caret are those parts' rules (ADD-A23, ADD-A24).

Widget-specific timings (tooltip delays, submenu delay and sloppy close, scroll-bar auto-hide and trough click,
toast duration, focus-visible rule) are widget values in the catalogue (R17).

A platform switch that a token or widget value carries and rules also select on (kdeglobals' `[KDE] SingleClick`,
Win32's menu drop alignment) has one route: the reader reports the condition only (`system_setting:single_click`,
`system_setting:menu_drop_alignment`, S7.1), and the preset states the value by a rule under it
(`token.behavior.single_click_activates`, `PlacementParams.handedness`, S13).

### S4.3 Theme change (R9)

```rust
pub enum ThemeChange {
    Crossfade { duration: Value<Ms>, timing: Value<Timing> },
    Suspend { duration: Value<Ms> },         // suspends the transitions marked `suspended_by_theme_change`
    Instant,
}
```

A theme change is a re-resolution caused by a change of the preset, a layer, an input or a reader value (a stated
value of the reader's layer, or `ReaderState.text_scaling_factor`, `font_dpi` or `content_size_category`; the `ReaderState`
conditions count as conditions), or of one of these conditions: `Scheme`, `ContrastIncreased`, `ContrastHigh` (R9);
any other condition change, the reader's included (`ReducedTransparency`, `BoldText`, `ReducedMotion`, …), runs the
property transitions of the styles it changes. `resolve` classifies a change by comparing the layers' values and the
conditions separately (S8), so a reader re-read that changes only a condition is a condition change. The new
resolution's `token.theme_change` governs a theme change, as the style after a change governs a transition (S2.7):
a `set_preset` from a preset that suspends to one that crossfades crossfades, and a rule that restates the token
under a condition (reduced motion) governs every theme change made while it holds. A transition
already running when a suspension starts finishes. Under `Crossfade` the window crossfades from the old frame to the
new, and the property transitions the change triggers do not run (each property is at its new value under the
fade); where a crossfading platform does otherwise, its facts say so. Under `Instant` there is no window effect: the
property transitions the change triggers run as stated, as for a condition change (R9).

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
                        pub fallbacks: FontFallbacks, pub rendering: TextRendering, pub scaling: TextScaling,
                        pub font_dpi: Value<f32> }   // logical px per inch (72 points), the platform's, from
                                                      // the facts; the reader's where it reports one (S9.4)
pub enum TextScaling { Linear, AndroidNonLinear, DynamicType }   // S9.4
pub struct FontFallbacks { pub families: Vec<String>, pub emoji_family: Value<String>,
                           pub per_language: Vec<(LanguageTag, Vec<String>)>,
                           pub per_family: Vec<(String, Vec<String>)> }   // Core Text's per-font cascade lists
```

The nineteen slots are closed (ADD-A42); a slot is a reference to the platform's keyed style
(`= token.text.keyed.body`), so a style is stated once. Where slots share one keyed style, exactly one of them, its
**owner**, references the style, and every other references the owner slot (`subheadline = "= token.text.roles.body"`).
The owner is no preset's choice: it is the first of the sharing slots in one fixed order, `body`, `caption`, `title1`,
`title2`, `title3`, `headline`, `subheadline`, `large_title`, `display_large`, `display_medium`, `display_small`,
`body_emphasized`, `callout`, `footnote`, `caption_emphasized`, `caption2`, `label`, `monospace`, `numeric`, so a
patch on a slot (a reader's font, S9.5) reaches the same slots under every preset that shares the same styles, never
depending on which slot a preset's facts happened to name. Which keyed style fills each slot is the preset's facts';
S12 refuses two slots that can reference one keyed style directly, and a slot that references a slot later in that
order, under one combination of conditions. Font fallbacks are the platform's one
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

Each context is an icon style (size, weight, scale, rendering mode), the platform's further contexts as keys (ADD-A43).
In a context only `size` is required: every other `IconStyle` `Value` field may be absent there (a context states
weight, scale and rendering mode only where the platform's context sets them), a mark of this document's (S6.3); its
lists (`axes`, `symbol_variants`, `status_colors`) are stated, empty where the context sets none (S2.11). A part takes a
context's fields by reference to each (`size_px = "= token.icons.sizes.toolbar.size"`); a reference to the whole context
brings its absent fields with it, which validation checks like any (S12). `theme_fallback` is the theme the platform's
own lookup falls back to before the system fallback (`KirigamiApp` makes Breeze the fallback of a Plasma application,
K-KA-47†): the platform's lookup chain, as a KDE application resolves it, not a mixing of sets.

### S4.6 Keyed scalars

`Tokens.params` holds keyed groups of scalars, referred to as `token.params.<group>.<key>` and declared per preset
(`params = { <group> = { unit = "px", keys = [...] } }`, S10, each group one unit): `layout` (Kirigami's grid unit and
spacings, Qt style metrics), `opacity` (disabled, dim, …; restated by `Tokens` rules under `ContrastHigh`), `shadow`,
`motion`, `color` (KDE's contrast and frame contrast), `icon_effect` (KDE's icon-effect amounts, K3.60, Q3.18), `caret`,
`shape`, `pointer_repeat` (`delay` and `interval`, `unit = "ms"`: the platform's one system auto-repeat of a held
pointer button, GTK's `gtkrange.c` timing, GN-3.69†, macOS's `NSEvent` periodic delay and interval, MAC-X12, declared
by the presets whose platform states one; a widget's own auto-repeat values have it as their base there, the
catalogue's one home per fact), `display` (`corner_radius_top_left`, `corner_radius_top_right`, `corner_radius_bottom_right`,
`corner_radius_bottom_left`, the measured display radius per corner and device model, set by rules under `DeviceModel`,
R25.11; for a model the preset does not list they are `Absent`, and concentric corners then fall back to their stated
minimum) (ADD-A44). The display is the root's container: a full-screen container's `Radius::Concentric` corners follow
the display's radius at the matching corner, which is the connector's `display_corner_radius` where the platform reports
it, else these tokens, else none, and then each corner its stated `minimum`. `RootInputs.display_corner_radius` holds
the four corners in the order top-left, top-right, bottom-right, bottom-left, the tokens' order, so both routes have one
shape.

### S4.7 Pointer

`PointerTokens { theme: Value<String>, size: Value<Px>, color: Value<Color>, drag_cursors: Vec<(Key, String)> }`,
read where the platform reports them (ADD-A46).

### S4.8 Platform scheme tokens

```rust
pub struct KdeTokens {                        // KDE's colour effects (K3.40, K3.50) and scheme options (R25.1);
    pub disabled: KdeEffects, pub inactive: KdeEffects,   // every value stated, none absent (below)
    pub inactive_enabled: Value<bool>, pub inactive_change_selection_color: Value<bool>,
    pub tint_factor: Option<Value<f32>>,          // None: the scheme does not tint (no `TintFactor`)
    pub titlebar_is_accent_colored: Value<bool>,
}
pub struct KdeEffects {                       // the amounts as kdeglobals states them, untyped to a range: each
                                              // function clamps them as KColorScheme does (`kde_effects`)
    pub intensity: KdeIntensityEffect, pub intensity_amount: Value<f32>,
    pub color: KdeColorEffect, pub color_amount: Value<f32>, pub color_color: Value<Color>,
    pub contrast: KdeContrastEffect, pub contrast_amount: Value<f32>,
}
pub enum KdeIntensityEffect { None, Shade, Darken, Lighten }   // None: KDE's "no effect" setting
pub enum KdeColorEffect { None, Desaturate, Fade, Tint }
pub enum KdeContrastEffect { None, Fade, Tint }
pub struct MaterialScheme { pub variant: MaterialVariant, pub spec_version: Key,   // declared in `spec_versions`
                           pub contrast_level: Value<f32> }   // stated: the standard level in the base, and
                                                              // `Tokens` rules under the contrast conditions
                                                              // (R5.1); `m3_role` reads it, so never absent
pub enum MaterialVariant { TonalSpot, Neutral, Vibrant, Expressive, Fidelity, Content, Monochrome, Rainbow,
                           FruitSalad }
```

Every value of `KdeTokens` and `KdeEffects` is stated, and "the scheme does not tint" is `tint_factor`'s `None`, so
the KDE functions branch only on `Option`s, bools, enums and the scheme inputs' lists `input.kde_scheme.groups`,
`inactive_groups` and `unset_keys` (S9.1), never on an absent value, and S12's absent-operand check passes them. The
preset states Breeze's; a reader and a KDE port state each from the files they read (the reader kdeglobals and the
scheme file it names, a port its `.colors`): the key's value where the file states it, else
the value the reading code takes for an unset key, cited, never left absent: the effects, amounts and colour as
KColorScheme reads them unset (`kcolorscheme.cpp:36-49`; where `[ColorEffects:Disabled] Enable` is false, the
disabled group's three effects are stated `none`, as KColorScheme then applies none, `:39-40`, `KdeTokens` having no
switch of its own for that group); `inactive_enabled` as `[ColorEffects:Inactive] Enable`, which KColorScheme reads as
false when unset (`:39-40`); `inactive_change_selection_color` as `ChangeSelectionColor`,
else the file's `[ColorEffects:Inactive] Enable` where it states one, else true (`:307`, R5.1), which is not
`inactive_enabled` when `Enable` is unset; `tint_factor` as the scheme file's `[General] TintFactor`, `None` where it
has none (`colorsapplicator.cpp:154-155`); `titlebar_is_accent_colored` as `applyScheme()` reads it
(`TitlebarIsAccentColored`, else `accentActiveTitlebar`, else false, `:152-153`).

These values are the scheme's: Plasma writes kdeglobals' effects and contrast from the scheme file
(`colorsapplicator.cpp:311-318`, `:340-359`), and the tint and title-bar options are in the scheme file alone. So the reader states
its `token.kde`, and its other scheme-file tokens (`token.params.color.contrast`, `frame_contrast`), as a `Tokens` rule
under the scheme it read (S9.3, carrying `preset:kde-breeze` as its preset-specific rules do), as S14 states a
one-scheme port's: after `pick(Some(<the other scheme>))` the preset's Breeze values apply with Breeze's scheme inputs,
never the user's scheme's effects or options over another scheme's inputs.

`KdeGroup` is `Active | Disabled | Inactive` (`kde_accent` takes the first and the last, S2.2); `KdeRoleKind` is
`Foreground | Decoration | Background`; `KdeSchemeKey` is the twelve keys of a scheme file's colour group that
KColorScheme reads and `applyScheme()` writes (`colorsapplicator.cpp:165-176`), `BackgroundNormal | BackgroundAlternate |
ForegroundNormal | ForegroundInactive | ForegroundActive | ForegroundLink | ForegroundVisited | ForegroundNegative |
ForegroundNeutral | ForegroundPositive | DecorationFocus | DecorationHover` (TOML and paths `background_normal` …, the
`<key>` of `input.kde_scheme.<set>.<key>`; the catalogue's key table maps each to its `SurfaceRoles` field); `KdeWmKey` is `ActiveBackground | ActiveForeground | InactiveBackground |
InactiveForeground`, the `[WM]` keys `activeBackground` … `inactiveForeground` (TOML `active_background` …), which the
keyed roles `role.@wm_active_background` … `role.@wm_inactive_foreground` carry (the catalogue's KDE keyed roles);
`applyScheme()` also writes `activeBlend` and `inactiveBlend`, which no gap-map row needs. A palette leaves the reader's `token.kde` out: a port states its own, a
mapping keeps the preset's (S8, S14).

## S5. Widgets and parts

A widget is a struct generated from its catalogue entry (the catalogue):

```rust
struct ButtonTheme {                   // the derive's input, not public: it names the widget's parts (the
    body: PartStyle,                   // catalogue's) and its values' type, from which S6.1 generates the
    label: PartStyle,                  // public types: the widget's patch in `WidgetsPatch` (a `PartPatch` per
    icon: PartStyle,                   // part), its part enum (`ButtonPart`), its `ButtonValues` and their
    values: ButtonValues,              // patch, and the query's `ResolvedPartStyle` per part
}
```

**S5.1 Typed and keyed.** A kind two or more platforms draw is a typed widget struct; a one-platform kind is a
keyed component (R14), named by a `Key`: bases state it as a `ComponentBase` (S1) in `Bases.components`, rules name
it by `PartRef::Component` and `WidgetRef::Component`, and the query answers with `ResolvedPartStyle` and
`ComponentValues` as for a typed widget. One route states a component's shape: the derive generates each component's table of
parts and their fields from the catalogue, as for a typed widget (S6.1), and the preset's
`[declare.component.<key>]` states only its availability, sizes, densities, variants, flags and values' types (S10);
adding a layer checks its rules and bases against both (S12).

**S5.2 Window.** `WindowTheme` has the parts the catalogue lists, among them `frame`, `title_bar`, `title`,
`caption_button` (one part type; each caption button an element whose identity is a typed variant: `close`,
`minimize`, `maximize`, `restore`, `window_menu`, and keyed ones where one platform has them, KDE's `@help`), and widget values for the
caption buttons' layout (R13).

## S6. The derive macro and the model's metadata

**S6.1** `#[derive(ThemeWidget)]` is replaced by `#[derive(ThemeParts)]`, which generates, per widget struct: its
patch type (every field `Option<…>`, where `None` is "this patch does not set it"; paints with `Set`/`Append`), its
resolved type (every `Value<T>` resolved to `Option<T>`, where `None` is "not drawn", possible only where
`may_be_absent` allows), the field-path registry used by the TOML key check of `Layer::from_toml` (S12) and the facts gate, and the
`may_be_absent` table. A length that may be `Auto` resolves to `Option<Length>`, `Length = Px(f32) | Auto`, so "sized
by content" reaches the toolkit; every other length resolves to `Option<Px>`. The resolved type is per type, not per
part: a `PartStyle` length field that the catalogue marks `auto` on any part, `max_width`, `max_height` and
`TextStyle.line_height` resolve to
`Option<Length>` on every part (`ResolvedPartStyle` is one type), validation refusing `Auto` where neither the part's
entry nor the core (S6.3) marks it (S12); a widget's value struct, generated per widget, resolves a length to `Option<Length>` only
where that widget's entry marks the value `auto`. Each component's part and field table is generated from the
catalogue the same way (S5.1), as data, its parts' styles being `PartStyle`.

```rust
pub enum PartPatch { Style(PartStylePatch), Absent,   // `Absent`: the whole part is not drawn (S6.3)
                     DrawnAs { target: WidgetOrPart,       // drawn by another widget, or a part
                               variants: Vec<VariantRef> } }   // of one (kirigami-addons' menu actions as form rows), in the
                                 // host's context (KDE's date picker trigger as a combo box); the query answers
                                 // `Styled::DrawnAs` and the connector builds that widget
pub enum ValuesPatch { Typed(WidgetValuesPatch), Component(ComponentValuesPatch) }
// WidgetValuesPatch: a derive-generated enum with one variant per typed widget, holding <W>ValuesPatch.
```

**Compound fields** (`Sides`, `Axes`, and block fields that are not `Value`s, `FocusIndicator.corners`) patch per
sub-field (`Sides<Option<Value<Px>>>`), with the same table spelling as a `Fields` patch.

**Merging patches:** a field a later patch sets replaces the earlier value; a map (`Vec<(Key, …)>`, the bindings by
role or surface, the presentations by event, the transitions by property group, a transition listing several groups
being split per group first; every `Vec` of pairs) merges by key, the later entry replacing the earlier; any other list (`stops`, `layers`
set with `Set`, `symbol_variants`, `fallbacks`) is replaced whole, an empty one included; `Append` adds paint layers over `base` (R6), and
over an `Absent` base gives a paint of the appended layers; over a reference it composes lazily, as a `Fields` patch
does (S2.2); `Append` in a layer's bases is an error, as `base` is.

**S6.2** Inheritance code goes: `inherit_from`, `theme_inherit`, the `[uniform]`, `[per_platform]` and
`[wrong_safety_nets]` registries and `resolve/inheritance.rs` are removed (R20). Their rules are converted once
into the presets' bases as expressions (S19.2).

**S6.3** `Absent` is allowed from two sources: a block's sub-field this document marks "may be absent" (S2),
wherever that block appears (one mark holds in one place only: `IconStyle`'s fields in an icon-size context, S4.5);
and the catalogue's tables, for colour roles, tokens and, per (widget, part, field
path), at three levels: a
field of a part (`button.body.shadow`), a whole part (`slider.ticks`: some platform draws no ticks), and a whole
widget (a preset declares it `available = false`, S10). A whole part is made absent by a base or a rule
(`PartPatch::Absent`, TOML `absent = true`), so it may be absent under a condition only (a title bar in full
screen); the query then answers `NotDrawn`. An unavailable widget and an absent part need no fields. A compound widget value listed `may_be_absent` (a `Placement`, a record of `Value`s) allows each of its `Value`
members to be absent, and, where it is a `Value` itself (`Value<Placement>`), to be absent as a whole (a desktop that
avoids no keyboard). A widget's root
part is never absent as a whole and is never a repeated element (the catalogue gives such widgets a container root);
its fields may be. `may_be_absent` is about what a platform draws, never about an element's content: a part the
element's content lacks (a dialog given no icon, an icon-only tab's label) is simply not queried by the connector,
and the catalogue does not list it. A `Style` patch after a `PartPatch::Absent` brings the part back with the fields stated below the
`Absent`. Likewise `Auto` is allowed only on the length fields the catalogue marks `auto` (sizes, widths, heights,
gaps), per part, and on the core's own marks: `TextStyle.line_height`, `PartStyle.min_width` and `min_height` ("no minimum
beyond the content") and `max_width` and `max_height` ("no maximum") on every part (S2.11); their resolved types are S6.1's.

**May be absent, in the core's blocks** (the first source; wherever the block appears unless a mark names its place,
because some platform has no
counterpart, R6, R20): every `Material` parameter but `identity` and `fallback`; `Ripple`'s values; a `FocusIndicator`'s
`shadow` and `transition` (its `dash` is stated, `solid` included, S2.4); `IconStyle.state_effect`, `weight`, `scale`,
`animation`, and `fill` where `rendering_mode` is `Original` only (S2.9);
`Glyph.stroke`, `draw_extent`, and `fill` on a stroked-only path (a spinner's arc, whose paint is its `stroke`'s;
S12 refuses `fill` absent where the stroke can be absent too); `TextStyle.decoration`, `mnemonic_decoration`, `letter_spacing`, `word_spacing`;
`Decoration.by_word`; an `Edge`'s `width` and `paint` (an edge not drawn);
`Font.width`; every `TextRendering` field; `Stroke.outset`; `Edge.dash` on an edge not drawn only (its `width` or
`paint` absent; S12 refuses it absent on a drawn edge, S2.4); `PartStyle.shape`, `touch_target`, `margin`,
and a part's drawing fields `fill`, `stroke`, `shadow`, `text`, `icon`, `glyph`, `focus`, `cursor` (a part that paints
nothing at rest, a flat button's fill, states `"absent"`; leaving a listed field out is still missing, S2.2);
`Caret.secondary`, `shown_in_read_only`, `solid_while_typing`; every `Value` field of `Behavior`, `TouchBehavior`,
`ScrollPhysics` and `PointerTokens` (a plain enum or record field, `Behavior.scroll`, is stated, and so is every
parameter of a `ScrollDecay`; the catalogue's rule, *Entry format*; `KdeTokens` and `KdeEffects` are stated whole,
`tint_factor`'s `None` saying the scheme does not tint, S4.8); in an icon-size context, every `IconStyle`
`Value` field but `size` (S4.5); `IconTokens.theme_fallback`; a spring's
`Transition.duration` and `PresentationStep.duration` (checked: `Absent` only where the timing, followed through
references, is a `Spring`; a `Loop.period` is never absent, as its keyframes' `at` are fractions of it);
`Concentric.minimum`; `Full.maximum`; `Keyframe.timing`; `Glyph.draw_start`. A list (`Vec`) is never `Absent`: it is
stated, empty where the platform has none (S2.11). The catalogue's tables are the second source; they cite these
marks rather than repeat them. The derive generates the tables from the catalogue; `Absent` anywhere
else is an error when the layer is added (S12).

## S7. Rules and selectors (R2.3)

```rust
pub struct Rule { pub selector: Selector, pub body: RuleBody }
pub enum RuleBody { Colors(ColorRolesPatch), Tokens(TokensPatch), Part(PartRef, PartPatch),
                    Values(ValuesPatch) }        // the target and its patch in one: a mismatch cannot be written
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
pub struct ContextMatch { pub scope: ContextScope, pub target: Option<WidgetOrPart>,   // None: any widget (R25.9)
                          pub sizes: SetMatch<ControlSize>,    // the ancestor's control size (MAC-MN15)
                          pub surface: Option<SurfaceSet>,     // the ancestor binds this surface set (Q1.37,
                                                               // ADD-A68); None in any of these: not matched on
                          pub variants: SetMatch<VariantRef>, pub states: SetMatch<StateFlag> }
pub enum ContextScope { AnyAncestor, NearestAmong(Vec<WidgetRef>) }
pub struct PositionMatch { pub among: SiblingScope,                 // which siblings count (R25.9)
                           pub first: Option<bool>, pub last: Option<bool>, pub only: Option<bool>,
                           pub parity: Option<Parity>,
                           pub previous: Option<SiblingMatch>, pub next: Option<SiblingMatch> }
                                                   // previous, next: the adjacent sibling among those `among` names
pub enum SiblingScope { SameKind, Any, Kinds(Vec<WidgetRef>) }     // default SameKind
pub enum Parity { Odd, Even }                                       // counted from 1, among `among`
pub struct SiblingMatch { pub target: Option<WidgetOrPart>,
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
    OverlayScrollbars, OnOffLabels, FullKeyboardAccess, PreferCrossFade,
    AccessibilityTextSize,                                 // the content-size category is one of the five
                                                           // accessibility categories (S8's `ContentSizeCategory`)
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
tiled to those two edges, R13, and a window maximised on both axes to all four, with `Maximized` holding too, so a
`tiled:left` rule matches it); every other parametrised condition holds on equality. `ContainerSize` holds for
exactly one size of each container the preset declares (`container_sizes`, S10): the size the reader or stand-in
reports for it (`ReaderState.container_sizes`), else its declared default, the first, also in the no-reader state
(S16), so a rule under the default size applies wherever nothing is read.

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
                      TextSelected,    // a text widget holding a selection (the caret hides, GN-3.73†)
                      DropCopy, DropMove }   // a drop target's operation (W3-49†, P3-10†)
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
```

A widget's `breakpoints: Vec<Breakpoint>` is a widget value read through `values()`; the connector evaluates each
against the allocation or the window it measures and adds the variant to `Element.variants` while it holds; rules of
two variants that hold together apply in stated rule order, as any rules do.

```rust
pub enum Measured { Allocation, Window, Part(PartRef) }   // the size the parent offers the element, the window's,
                                                       // or the allocation of a part of the widget that holds the
                                                       // content (AdwPreferencesDialog's breakpoint bin, GN-2.10):
                                                       // never a box the breakpoint's own variant sizes, so a
                                                       // variant that changes padding does not feed back into it
pub struct SizeClasses { pub horizontal: Option<SizeClass>, pub vertical: Option<SizeClass> }   // R25.10; None:
                                                                     // the platform states no class on that axis
```

Each preset's facts state which of its sizes fills which `ControlSize` slot (Material Expressive's button sizes,
AppKit's control sizes, per macOS version); a preset declares only the slots it fills (S10).

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
    pub idiom: Option<Idiom>, pub device_model: Option<Key>,
    pub container_sizes: Vec<(WidgetRef, Key)>, pub system_settings: Vec<Key>,
    pub text_scaling_factor: f32,                          // S9.4
    pub font_dpi: Option<f32>,                             // the system's font DPI where the platform has a setting
                                                           // (KDE's `forceFontDPI`, `Xft.dpi`); None: the preset's
                                                           // `token.text.font_dpi` (S9.4)
    pub content_size_category: Option<ContentSizeCategory>,   // iOS's, for `DynamicType` and `uifm_scaled`
                                                           // (S9.4); None: the default category, the one the
                                                           // `DynamicType` kind's table names (the facts, S17)
}
pub enum ContentSizeCategory {                             // UIKit's `UIContentSizeCategory`, smallest first
    ExtraSmall, Small, Medium, Large, ExtraLarge, ExtraExtraLarge, ExtraExtraExtraLarge,
    AccessibilityMedium, AccessibilityLarge, AccessibilityExtraLarge, AccessibilityExtraExtraLarge,
    AccessibilityExtraExtraExtraLarge,                     // `AccessibilityTextSize` holds for these five
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
the order it adds them (R2.4), the palette `apply_palette` adds among them where it was first added; and last the
accent layer, which `Theme` derives from the `Accent` the application set on every change of the stack (preset,
palette, reader re-read), so which accent shows never depends on call order (on `kde-breeze` also from another
platform's reader's accent, S9.2). The accent is a setting, not a layer
the application stacks. Every layer's bases apply in that order, then every
layer's rules in that order. While a palette applies, the reader's colour bases, colour rules, colour inputs (the
`kde_scheme.*` scheme inputs and their `groups`, `inactive_groups` and `unset_keys` lists among them) and `token.kde` are left out; the palette does not apply while `ContrastIncreased` or `ContrastHigh` holds unless it
states colours for that condition (colour rules under it, or per-contrast input bases for it, S9), and the reader's
colours then apply as without it (R19). While the current preset is not the reader's own (S9.5) and declares
`accent_inputs` beyond `accent` (S9.1), `Theme` leaves the reader's common `input.accent`, with its per-contrast
bases, out, as it leaves the reader's `for_preset` part out: that preset keeps its own accent and derived inputs,
and no accent mixes with derived inputs of another (S9.5). On the reader's own preset its accent applies with its
derived inputs.

**Validation** (S12) runs on every change of the stack (S16): it validates the whole stack, conservatively (every
rule and binding an expression can meet, under every combination of conditions that can hold together, S12,
R2.8), and a change that fails is refused
whole, leaving the theme as it was (R2.5). So resolution and the query cannot fail. A reader's re-read that fails keeps
the previous reader layer; `Theme::reader_status()` reports the error.

**Windows and resolution.** Each window's connector owns a `WindowState` (`Default`: never resolved), and
`Theme::resolve(&self, window: &mut WindowState, conditions: &WindowConditions) -> Resolution` resolves for it
(infallible), with `Resolution { theme: ResolvedTheme, change: Change }`, `Change = First | Unchanged | ThemeChange |
ConditionChange` relative to that window's last resolution, which `WindowState` records (a theme change wins when
both happened; a flavour switch is a condition change; S4.3): it compares the layers' values, the inputs and the
reader values with those it recorded, and the conditions (the window's, the reader's `ReaderState` conditions and the
application's) separately, and only a change of the first kind, or of `Scheme` or a contrast condition, is a theme
change. After any change but `Unchanged` a connector rebuilds the
scopes it kept, whose bound colours are the old resolution's (an inactive window changes roles too):

1. merge bases in order; for colours, the base of the conditions' scheme; for each input, its per-contrast base
   where a contrast condition holds (S9);
2. apply the `Colors` and `Tokens` rules whose conditions hold, in order (choosing expressions, not evaluating);
3. for each part and widget, keep its base and its rules whose conditions hold, in order.

A window resolves again on any change of its conditions, an input, a reader value, a layer, the flavour or the
preset.

**Query.** `ResolvedTheme::root_scope(&RootInputs) -> Scope` gives the window's root scope. Each query's returned
`Scope` is its input scope plus this part's ancestor entry and bindings; a connector passes, for any part or element,
the scope returned by the query of the innermost part that encloses it in the catalogue's nesting (a widget's root part:
its host's part that holds it). An ancestor entry records the element it belongs to: context selectors (`ContextMatch`,
`context` in S11) skip the entries of the queried element's own parts (its root's, an enclosing plain part's), so
`context button` means an enclosing button, never the button the part belongs to, while bindings found there still
apply (S2.11, S3.3). Every query takes its parent's scope and returns its own, so a connector queries
top-down, as it draws. A `Scope` holds owned, evaluated values only (the ancestor chain, each ancestor's widget, part,
variants, states, control size and the surface set it binds, which context selectors match; the bound colours; density,
size classes, direction, language; the window's and screen's size; which element and part is the nearest container and
its stated corners, the display at the root, whose computed radii the connector passes to `native_theme::corners`,
S2.4), so a scope kept across a re-resolution is safe to use but stale; the connector rebuilds it (above). Tokens and
roles are read in a scope too, for what no part carries (behaviour timings, the caret, text rendering, the pointer, the
window's background).

```rust
pub enum Styled<T> { Style(T), NotDrawn, DrawnAs { target: WidgetOrPart, variants: Vec<VariantRef> },
                     NotAvailable(Unavailable) }   // R2.7's answer; not an error
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
    pub fn palette_status(&self) -> Option<PaletteStatus>;                          // S14; None: no palette
    pub fn data_needs(&self, part: PartRef) -> DataNeeds;   // the element data and metrics its values read, and
                                                            // whether its glyph or icon is drawn to its box (below)
}
pub struct Element<'a> {                                   // Element::new(part, scope, metrics) and setters
    pub part: PartRef,                                     // names the widget too (S1)
    pub scope: &'a Scope,                                  // the scope returned by the query of the innermost
                                                           // part that encloses it in the catalogue's nesting
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
pub trait FontMetrics { fn char_width(&self, font: &ResolvedFont) -> Px;      // the average character width
                        fn max_char_width(&self, font: &ResolvedFont) -> Px;  // the widest glyph (KCharSelect)
                        fn x_height(&self, font: &ResolvedFont) -> Px;
                        fn height(&self, font: &ResolvedFont) -> Px;          // ascent plus descent
                        fn line_spacing(&self, font: &ResolvedFont) -> Px;    // plus leading
                        fn underline_offset(&self, font: &ResolvedFont) -> Px;      // the font's own decoration
                        fn underline_thickness(&self, font: &ResolvedFont) -> Px;   // lines, in `Decoration`'s
                        fn strikeout_offset(&self, font: &ResolvedFont) -> Px;      // convention (S2.8);
                        fn strikeout_thickness(&self, font: &ResolvedFont) -> Px; } // all infallible
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

The query tells the variants apart by name and element: the content variants are the catalogue's listed pairs, each a
widget or one of its *(element)* parts and a typed variant name (`button`: `icon_only`; `list.row`: `with_subtitle`;
*Entry format*, ADD-A61), a name being a content variant only on the widget or part its pair names; the breakpoint
variants are those the widget's `breakpoints` value names (S7.3); and every other variant in `Element.variants` (the
same name on another widget or part included: `vertical` on `slider`) is one the application asked for. The
declarations in force are the preset's, a flavour's section replacing the keys it states (S10). The query: ignores a
content or breakpoint variant the widget does not declare (as an undeclared density), and a keyed flag the widget does not declare selects nothing; answers `NotAvailable` for an undeclared size or a variant the
application asked for, an unavailable widget, a missing required
variant, or a part the widget does not have, and `NotDrawn` for a part whose style resolved `Absent` (the children of
a `NotDrawn` or `NotAvailable` element take the parent's scope); drops excluded flags by the exclusive groups (S10),
the element's by its widget's groups, and each ancestor's and sibling's states (which `ContextMatch.states` and
`SiblingMatch.states` match) by its own widget's groups, and ignores a density the widget does not declare (R15); takes density, size classes, direction, language and
bindings from the scope, the element's own overriding them; applies the part's kept rules in order, skipping those
that do not match (position counted among the siblings the rule's `among` names; `SameKind` siblings are those of
the queried element's widget and part; an `index` outside `siblings` is treated as having none); evaluates
expressions (S2.2) with `base`, the element data, `part.`, `widget.`, `values.`, the font metrics and the bindings
in scope; and returns the part's style with the scope its children receive. `element.width` and `element.height`
are the laid-out box of the element the part is queried with: the widget's root part's box for the widget's element,
an *(element)* part's box for that element and the parts that belong to it (S2.11), never a nested plain part's. A
corner operand (`part.corners.<corner>`, `part.focus.corners.<corner>`, or the same through `widget.<part>.`)
resolves `Full` to half the smaller of `element.width` and `element.height`, at most its `maximum` (the part declares
that data), so it is allowed only on the corners of the element's own part (the root part, or the *(element)* part:
a split button's label reads its half's corners, `widget.primary.corners`); on a nested part's corners (a thumb, a
handle), and on `Concentric` and `FollowPart` anywhere, it is refused (S12), as only the connector computes those at
layout (S2.4). `Element.variants` includes the variants the connector sets from the element's content
(`icon_only`, `with_icon`; GTK sets `.image-button` itself, ADD-A61).

The query cannot fail (R2.8): element data are plain values. Each part declares the data its values read (S10,
`data = [...]`, the font metrics included; a widget value that reads element data, a presentation step's duration
over `move_distance`, is declared under `values`), in the preset's `[declare]`, which every layer's expressions are
checked against (an application layer reads what the part declares, S10, S12); `data_needs` tells the connector what
to supply for a part, as it supplies the element's states. Expressions over `element.width` and `element.height` are not allowed in the fields that
lay the element out (a part's padding, margin, size limits and `touch_target`, widget gaps) or that size its content
(a text style's font size, line height, letter and word spacing, an icon's or glyph's size), unless the element's
size on that axis does not depend on its content (a fixed box: an avatar's initials sized to the avatar, GN-2.14;
the element's own part, root or *(element)*, is the box tested, S12), so one query per frame suffices (S12).

A glyph or icon **drawn to its box** is the one other exemption: a part's `glyph.size` or `icon.size` may read
`element.width` and `element.height` in a box that is not fixed (AdwSpinner draws its arc at the size it is allocated,
the smaller side capped, `adw-spinner-paintable.c:343`, while it measures at its minimum, `adw-spinner.c:70-81`).
That glyph or icon then takes no part in measuring the element: the connector measures the element without it, at
the element's own part's `min_width` and `min_height` on the axis read (which S12 requires stated as lengths, not
`auto`) and its other content, lays it out, and queries once with the laid-out box, which it draws the glyph to.
`data_needs` marks such a part as drawn to its box, so the connector knows to leave it out of the measurement.

A part a preset states `drawn_as` another widget (S6.1) is built as an element of that widget in the host's context,
queried in the scope the drawn part was queried in, with the variants `DrawnAs` names and the states of the host's
element (the element the drawn part belongs to: its hover, focus, pressed, …), as the platform's style draws the
substitute from its host's state (Breeze's scroll-area frame as a styled panel, K2.21); a flag the substitute's widget
does not declare selects nothing there, as on any element.

## S9. Inputs and readers (R2.6, R18)

```rust
pub struct InputBases { pub entries: Vec<(InputName, InputValue)>,
                        pub kde_groups: SchemeMap<Vec<SurfaceSet>>,            // `input.kde_scheme.groups`
                        pub kde_inactive_groups: SchemeMap<Vec<SurfaceSet>>,   // `input.kde_scheme.inactive_groups`
                        pub kde_unset_keys: SchemeMap<Vec<InputName>> }        // `input.kde_scheme.unset_keys`
                                              // (S9.1): `kde-breeze`'s only, empty elsewhere; stated together by
                                              // whoever states the scheme inputs; a later layer's list for a
                                              // scheme replaces the earlier one whole
pub struct InputValue { pub per_scheme: SchemeMap<Value<Color>>,
                        pub per_contrast: Vec<(Contrast, SchemeMap<Value<Color>>)> }
pub enum Contrast { Increased, High }         // one at a time: `ReaderState.contrast` and `set_contrast` hold one,
                                              // so the two conditions exclude each other (S12)
// An input a later layer states replaces the earlier one whole (per_scheme and per_contrast together); where it states
// no contrast base, its per_scheme value applies under contrast too.
```

**S9.1 Declared inputs per preset:** `accent` (all); `accent_ramp` entries (`windows-11`); the AppKit-derived
accent roles and `highlight` (`macos-*`); the tint-dependent roles (`ios-*`); on `material-3` the accent is the seed, which the scheme function reads; on
`kde-breeze` the inactive input roles `inactive.<set>.<key>` (the colour a set's key takes in an inactive window
before KColorScheme's inactive effects, for every declared set and key) and the scheme inputs, the values of the
scheme file before Plasma applies the accent (the preset's from Breeze's scheme files; the reader's from the file
kdeglobals names, `[General] ColorScheme`; a port's from its own file, S14) (R25.1): `kde_scheme.<set>.<key>` and
`kde_scheme.<set>.inactive.<key>` for every declared set and `KdeSchemeKey`; three lists per scheme, which `kde_accent`
branches on as `applyScheme()` and KColorScheme do (S2.2) and which are no expression operands:
`kde_scheme.groups`, the declared sets the file has a `[Colors:<Set>]` group for; `kde_scheme.inactive_groups`, the
declared sets whose group has an `[Inactive]` group (empty where none has); and `kde_scheme.unset_keys`, the scheme
inputs (`kde_scheme.<set>.<key>`, `.<set>.inactive.<key>`) of the groups the file has that it leaves unset (a set
`groups` does not name has every key unset and is not listed); for a set `inactive_groups` does not name, each
`kde_scheme.<set>.inactive.<key>` is stated `= input.kde_scheme.<set>.<key>`, what KColorScheme reads for a set without
an `[Inactive]` group (`kcolorscheme.cpp:345-351`). A value the file lacks, a whole group's or one key's, is stated as
KColorScheme reads it unset over the file, cited, never left missing: `Header`'s from the file's `[Colors:Window]`,
with KColorScheme's `Header` defaults beneath (`kcolorscheme.cpp:330-333`); another set's from KColorScheme's defaults
for that set (the `default*Colors` tables, `:122-216`); a key an `[Inactive]` group lacks from those same defaults, not
from the set's active value (`:345-357`). So every declared input has a value, the lists, not a missing value, carry
what the file lacks, and `kde_accent` follows `applyScheme()` where it reads an unset key itself (S2.2: in a tinting
scheme it tints such a key as `applyScheme()` does, from the invalid colour it reads). And `kde_scheme.wm.<key>` for the four `KdeWmKey`s (S4.8), what
`applyScheme()` writes into `[WM]` with no accent (`colorsapplicator.cpp:287-309`): the file's `[WM]` key where it
states one, else the default `applyScheme()` takes from KColorScheme over the same file, stated by whoever states
the scheme inputs as an expression over them: `active_background` and `active_foreground` the file's `Header` Normal
background and foreground (`= input.kde_scheme.header.background_normal`, `.foreground_normal`), `inactive_background`
and `inactive_foreground` the Inactive `Header` colours KColorScheme gives over the file, `kde_effects(…, inactive, …)`
as the inactive colour rules apply it (S2.2) over `input.kde_scheme.header.inactive.background_normal` and
`.foreground_normal`, which hold the file's `[Inactive]` values where `inactive_groups` names `Header` and its active
ones where it does not. The `kde-breeze` preset's own bases state each role as `applyScheme()` writes it on
Plasma's default path, "Accent color from color scheme" (no `AccentColor`): Breeze's scheme values with no accent
derivation applied, the inactive roles as that path writes them. `kde_accent` applies only when an accent is set:
the reader states the results kdeglobals holds when the user chose one, and the accent layer (`set_accent`, or
another platform's reader's accent, S9.2) states each accent-dependent role and inactive input role as `=
kde_accent(<set>, <key>, <group>)`, and each `[WM]` keyed role as `= kde_accent_wm(<key>)`, over the scheme inputs in
force (the reader's, a KDE port's, else Breeze's).

**The KDE reader without its scheme file.** The reader reads kdeglobals as KConfig does, with Plasma's
`kdedefaults/kdeglobals` beneath the user's file (`startplasma.cpp:421-426`, as today's reader does), and its scheme
inputs and scheme-file tokens from the file `[General] ColorScheme` names. Where neither file names a scheme, the scheme
is Plasma's own default, Breeze Light (`startplasma.cpp:452`), whose inputs are the preset's. Where the file it names
cannot be found or read, the reader takes them from kdeglobals itself, read as the scheme file would be
(its `[Colors:*]` groups and their `[Inactive]` groups, the lists from what kdeglobals has, its `[WM]` group; a key it
lacks as S9.1 and S4.8 state for an unset key, so `tint_factor` is `None` and the title-bar option false), keeps
everything else it read, and reports it: its status is `ReaderStatus::Fallback`, the failure naming the scheme and
the file. Only a kdeglobals the reader cannot read fails the read (`ReaderFailure::Platform`; a first read's failure is
`from_system`'s error, a re-read's keeps the previous reader layer, S8, S16). Weighed: failing the whole read, so that
`set_accent` never derives over anything but the user's scheme file, discards the user's fonts, colours and
conditions, all of which the reader has, to protect one derivation; while kdeglobals holds exactly the colours Plasma
shows, and is Plasma's own output of the scheme it was applied from. What the fallback costs is that a later `set_accent` derives over
those results rather than over the file (a tinting scheme's tint or an accent-coloured title bar already applied
there), which the status tells the application.

**Accent-dependent roles** (`kde-breeze`): the roles `applyScheme()` writes from the accent
(`colorsapplicator.cpp` at v6.8.2), named by scheme key per set (the catalogue's key table gives each key's
`SurfaceRoles` field), the one list the accent layer states (S9.2) and S12 checks a mapping palette against:

- in every declared set, `foreground_active`, `foreground_link`, `decoration_focus` and `decoration_hover`
  (`:178-218`);
- `Selection`'s `background_normal`, `background_alternate`, `foreground_normal` and `foreground_inactive`
  (`:230-238`);
- `Button`'s `background_alternate` (`:240-245`);
- `Header`'s `background_normal` and `foreground_normal` where the file has a `Header` group (the accent-coloured
  title bar, `:270-278`; without one `Header` reads `Window`'s results, S2.2);
- the four `[WM]` keyed roles `role.@wm_active_background` … `role.@wm_inactive_foreground` (`:320-338`);
- wherever `token.kde.tint_factor` can be `Some` (conservatively, S12), every other key of every declared set
  (`:219-221`).

The inactive input roles, every declared set's and key's, are stated beside them (S9.2), as `applyScheme()` writes
each `[Inactive]` group from the accent while the scheme tints and `Header`'s under the title-bar option where the
file has a `Header` group (`:247-284`). `kde_accent` gives a role the scheme input unchanged where `applyScheme()` changes nothing (a key the
accent does not touch while the scheme does not tint, `Header`'s colours without the title-bar option), so the list's
conditional entries are harmless under a combination where their condition does not hold.

**Accent inputs.** A preset declares which of its inputs follow the accent, `[declare] accent_inputs` (S10, a subset
of `inputs`): the inputs `set_accent` sets and requires, `set_preset` checks an `Accent` against
(`AccentInputsMissing`), a mapping palette must state (S14), and an application layer may not state (S12). On
`kde-breeze` that is `accent` alone: its other inputs are the scheme's, which Plasma's derivation reads (R25.1). On
every other preset it is every declared input, `macos-*`'s `highlight` included: whether or not the user's highlight
follows the accent, an application or palette replacing the accent states the text-selection colour with it, or the
user's selection colour shows beside the application's accent (R2.6).

**S9.2 Setting the accent:** `Theme::set_accent(Some(accent))` sets the preset's accent inputs (S9.1) in the accent
layer's bases (S8); it requires every one of them and otherwise returns `Err(AccentInputsMissing)` (on `adwaita`,
`material-3` and `kde-breeze` the `accent` alone). On `kde-breeze`, while no mapping palette applies, the accent
layer also states each accent-dependent role (S9.1's list) and each inactive input role as `= kde_accent(<set>, <key>, <group>)`,
and each `[WM]` keyed role (`role.@wm_active_background` …) as `= kde_accent_wm(<key>)` (whether tinting is on or
off: each function gives the untinted value when it is off), so they follow Plasma's
derivation over the scheme inputs in force (R25.1): the reader's, or a KDE port palette's (S14), whose stated roles
the layer so replaces. While a mapping palette applies, the layer states the `accent` input only: a mapping states
its roles itself, the accent-dependent ones (the `[WM]` keyed roles among them) by reference to `input.accent` where
they follow it and the inactive input roles as S14 says, and has no scheme inputs for `kde_accent` or `kde_accent_wm`
to derive over (S14). The choice is per scheme, as a palette's source is (S14): where a palette serves one scheme by
a KDE port and another by its mapping, the layer's colour bases (a `SchemeMap`) state the `kde_accent` and
`kde_accent_wm` roles under the port's scheme and the `accent` input alone under the mapping's.

**Another platform's accent on `kde-breeze`.** `kde-breeze`'s accent input is the accent alone (S9.1), so a reader's
or stand-in's common `input.accent` from another platform reaches it (S9.5); its accent-dependent roles are the
preset's literals of Plasma's no-accent path, which follow no `input.accent`. So where the application sets no accent
and the `input.accent` in force comes from a reader or stand-in whose own preset (S9.5) is not `kde-breeze`, `Theme`
derives the accent layer from that accent as from `set_accent`'s: it states the same `kde_accent` and `kde_accent_wm`
roles over the scheme inputs in force, Breeze's (no other platform's reader states KDE scheme inputs, and a palette
leaves the reader's accent out, S8), so the user's accent changes every role Plasma's derivation changes, never
`role.global.accent` alone. The KDE reader's own accent needs none: its results already hold the derivation.

**S9.3 Readers** state each value under the conditions they read it in: base conditions into the reader layer's bases,
other conditions into reader rules (or inputs' per-contrast bases). A reader rule carries the condition
`preset:<its own preset>` (S9.5) unless its patch touches the portable part only, so it is neither validated nor
applied under another preset (S12), as the reader's preset-specific bases are not (the macOS reader's
increased-contrast values of its keyed roles, `label_tertiary`, would otherwise name keys `adwaita` does not declare). A reader that reads both schemes (today's macOS
reader reads both appearances) states each value under the scheme it read it in. A reader that reads only the
system's current scheme, while inputs and colour bases are per scheme (S9, S3.1), states a value the platform
defines independently of the scheme (the accent setting on GNOME, Windows and KDE, which the facts mark so) under
both schemes, so the user's accent
shows when the application `pick`s the other scheme; a scheme-dependent value it states only under the scheme it read,
and the other scheme keeps the preset's value until the system switches and the reader reads again. The new reads per
platform are the rows of the traceability whose target is a reader read (S20); each is a `reader::<platform>` change.

**S9.4 Points, then text scaling.** A font size, line height, letter spacing or word spacing stated in points becomes logical pixels first, as
`pt × dpi / 72`, where `dpi` is `ReaderState.font_dpi` where the reader reports one (the platform's setting: on KDE
`forceFontDPI` or `Xft.dpi`, as `platform-facts.md` records them under *Font DPI override*) and otherwise the current
preset's `token.text.font_dpi`, stated per preset from its platform's facts (no number is stated here); one stated in
pixels is already logical. Validation refuses a DPI, the token's or the reader's, that is not finite and above zero (S12).
A reader states the font sizes it reads in logical pixels, converted by its own platform's DPI (its `font_dpi`, else
its own preset's `token.text.font_dpi`), so they keep their size under any preset it applies them to (S9.5). The
conversion comes before the text scaling, and both before every font-derived length (convert, then scale, then
derive).

**Text scaling.** The reader supplies `text_scaling_factor` (1 where the platform scales by the font size
itself, and 1 where a reader reads each text style's size, R10); the query scales font sizes by it before
font-derived lengths, by the preset's `token.text.scaling`: `Linear` (a product); `AndroidNonLinear`, Android 14's
`FontScaleConverter`, which scales large sizes less (MAT-X19); or `DynamicType`, each text style's size for the
reader's or stand-in's `content_size_category` (`ContentSizeCategory`, S8), to Apple's published table (the curve `uifm_scaled` uses for
lengths). A part's font is scaled by the curve of its `TextStyle.scaling_role`, `body`'s when `None`; a reader that
reads each style's size reports the default category. A font size is scaled once, where its own style is resolved: a
`Font.size` stated as an expression over other font sizes (`= mul(token.text.roles.body.font.size, <factor>)`) reads
them converted and not yet scaled, and its result is scaled by its own style's curve (S2.2), so a size derived from
another style's is never scaled twice, and under a non-linear kind it follows its own size's curve, as a platform
scales a size stated in its own scalable unit (Android's sp, UIKit's `UIFontMetrics` over a custom size). The last two stand in while `material-3` and `ios-*` have no
reader (R25.15). Under `DynamicType` the factor is not read: the category alone scales (`None` is the default
category), so a factor read on another platform does not reach an `ios-*` preset. Validation refuses a reader's or
stand-in's factor that is not finite and above zero (S12).

A line height follows its kind's rule, part of the kind's implementation and cited there: one stated as an
expression on `font_size` follows the scaled font, and `Auto` is the scaled font's own line spacing, under every kind;
one stated in points or pixels is, under `Linear`, multiplied by the factor (R10); under `AndroidNonLinear`, scaled as
Android scales a line height stated in sp (through the converter as a value of its own, or in proportion to its
font's change), the platform's own rule, read in the value phase from `FontScaleConverter` (MAT-X19) and Compose's
`FontScaling` (`androidx.compose.ui.unit`, `material-3`'s reference, R1.6); under `DynamicType`, the leading Apple's
table states for the style's `scaling_role` at that category (TS-2), the table's own line height, and for a style
whose stated line height differs from the table's at the default category (the tight and loose forms, MAC-TS03†), the
platform's own rule, read in the value phase from the fonts UIKit returns per category
(`preferredFont(forTextStyle:compatibleWith:)`, D-5). The value phase confirms each rule from its cited source.

Letter and word spacing follow the line height's rule, as a platform scales tracking stated in its own scalable unit
(Compose's typescale tracking is in sp, MAT-TS2): one stated as an expression on `font_size` (`sf_tracking`, a
multiple of the size) follows the scaled font; one stated in points or pixels is, under `Linear`, multiplied by the
factor; under `AndroidNonLinear`, scaled as Android scales a letter spacing stated in sp, read in the value phase from
Compose's `FontScaling` with the line height's rule; under `DynamicType`, as UIKit's fonts per category give it, read
in the value phase with the leading (D-5). A spacing keeps its sign.

**S9.5 What a reader states for any preset.** A reader builds its layer in two parts: its common bases, which any
preset takes (`input.accent` and its per-contrast bases, except where S8 leaves them out, below; the conditions,
text-scaling factor and font DPI in its `ReaderState`; and the typed text-role slots it reads, as `Fields` patches
over the style each slot references, their sizes in logical pixels (S9.4), composed lazily in R2.4's order (S2.2), so
the user's fonts reach the parts that reference the slots and the preset's token rules on the referenced styles still
apply over them), and everything else, its own platform's keyed roles, tokens and inputs, under
`for_preset.<its own preset>`. A reader patches exactly the slots its platform has a setting for (the KDE reader
`body` from `[General] font`, `monospace` from `fixed`, `caption` from `smallestReadableFont`, …; a reader that reads each
style's size, every slot); a slot it does not patch keeps the current preset's style, and a slot that shares
another's references that slot (S4.4), so under any preset the patch on `body` reaches every slot that preset shares
with `body`. Its rules split the same way (S9.3): a rule on the portable part (a text-role slot
read under a condition) applies to any preset, every other rule carries `preset:<its own preset>`, so the portable
part is the same for bases and rules. Its own preset is the one the theme was built with (`from_system`, S16: by the
OS version, or the generation the application names). `set_preset` applies the common part to the new preset and the rest only while its own
preset is current; it is refused (`Err(AccentInputsMissing)`) when the `Accent` the application set, or the applied
mapping palette's per-preset section (S14), lacks accent inputs the new preset declares, and
(`Err(PaletteNotForPreset)`) when the applied palette has neither a port nor a mapping for the new preset (S14). The
reader's `input.accent` is portable only to a preset whose `accent_inputs` is `accent` alone (on `kde-breeze` the
accent layer then derives Plasma's accent-dependent roles from it, S9.2), or to its own preset:
under another preset that declares derived accent inputs (`windows-11`'s ramp, the AppKit- and UIKit-derived roles),
which no reader from another platform states, `Theme` leaves the reader's accent out (S8), so that preset keeps its own
accent and derived inputs, and nothing mixes.

## S10. Declarations (R2.7)

```toml
[declare]
flavors = ["widgets", "kirigami"]            # first is the default
materials = ["mica", "mica_alt", "acrylic"]
color_roles = ["fill_tertiary", …]            # keyed roles
text_styles = [ … ]
elevation = ["<step>", …]                       # the platform's ramp's names
surface_sets = ["window", "view", …]
conditions = ["@graphic_effects_level", …,     # SystemSetting keys; one only applications read, which no rule
              { key = "@show_sounds", for_applications = true }]   # selects on, is marked (S12, `Theme::system`)
params = { layout = { unit = "px", keys = ["grid_unit", …] }, opacity = { unit = "fraction", keys = [ … ] } }
inputs = ["accent", "accent_ramp.light1", …]
accent_inputs = ["accent", …]                  # the inputs that follow the accent, a subset of `inputs` (S9.1)
# every keyed namespace is declared:
blend_modes = [ … ]
timings = [ … ]                                # motion timing tokens' keys
transition_kinds = { <kind> = { <param> = "<unit>" } }    # with their parameters' units
presentation_kinds = { <kind> = { <param> = "<unit>" } }
presentation_events = [ … ]
container_sizes = { sidebar = [ … ] }           # first is the default
symbol_variants = [ … ]
symbol_scales = [ … ]                          # IconStyle.scale (SF Symbols' named scales)
spec_versions = [ … ]                          # MaterialScheme.spec_version (the versions `m3_role` implements)
icon_state_effects = [ … ]
focus_kinds = [ … ]                            # FocusKind::Keyed (WinUI's reveal)
device_models = [ … ]
material_params = { <key> = "<unit>" }
durations = [ … ]                              # motion durations' keys
icon_contexts = [ … ]
status_colors = [ … ]
drag_cursors = [ … ]
accent_ramp = [ … ]
rendering_engines = [ … ]                      # TextRendering.engine
lcd_filters = [ … ]
schemes = ["light", "dark"]                    # a palette's schemes (S14); a palette states them here only

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


[declare.flavor.kirigami.widget.button]                   # a flavour's declarations where they differ (R2.7):
sizes = ["regular"]                                        # each key replaces the widget's own under that flavour
variants = ["primary", "@<variant>"]                       # the flavour's own variant set (below)

[declare.flavor.kirigami.component.<key>]                  # likewise for a component (which flavour draws it)
available = true

[declare.widget.spinner]
data = { indicator = ["width", "height"] }                 # element data a part's values read (S8; names below)

[declare.component.rule_editor]                # parts and their fields: the catalogue's, generated (S5.1, S6.1);
available = true                               # here only what varies per preset: availability,
sizes = ["regular"]                            # a component's sizes and densities, as a widget's,
values = { indent_per_level = "length", presentations = "presentations" }   # values' types,
variants = ["@<variant>"]                                           # variants, flags
flags = ["@<flag>"]

[declare.widget.segmented_control]
keys = { compression = ["<key>", …] }          # the keys of a typed widget's `Value<Key>` value, keyed map,
                                               # `Vec<Key>` list or list entries' keys (below)
breakpoints = [{ measured = { part = "content" }, width = { at_most = <n> }, variant = "@<v>" }]
                                               # `Measured::Part` in TOML, by S11's enum rule
```

A component value's type names one of these `WidgetValue` members (S1): `"number"`, `"integer"`, `"length"`, `"duration"`,
`"bool"`, `"color"`, `"paint"`, `"timing"`, `"transition"`, `"loop"`, `"glyph"`, `"icon_style"`,
`"scroll_physics"`, `"breakpoints"`, `"presentations"`, `"placement"`, `"sides"` (a length per side); `"enum:<Name>"` for one of the model's public
enums (`"enum:Alignment"`, a shared value type of the catalogue's); and `{ key = ["<k>", …] }` for an enumerated
value of the component's own, a `Value<Key>` whose keys the catalogue entry lists. The other members (a font size,
`Vec<char>`, `Vec<Key>`, `Vec<MenuEntry>`) type typed widgets' values only: no component's entry has such a value.

A `data` list (per part, and `values` for the widget's values) names what the query reads from `Element.data` and
`Element.metrics` (S8), one entry per operand: an `element.` operand by its name (`width`, `progress`, `char_width`,
`font_underline_offset`, …); `line_spacing`, the own font's line spacing that the `line_height` operand reads under
`auto` (S2.2); `body_font_height`; and another part's font metric as `widget.<part>.<metric>` (`char_width`,
`max_char_width`, `font_x_height`, `font_height`, `font_underline_offset`, `font_underline_thickness`,
`font_strikeout_offset`, `font_strikeout_thickness`, `font_size`, `line_height`). The root's inputs
(`element.window_width`, `window_height`, `screen_width`, `screen_height`) need none (S8). The declarations are the
preset's, as every `[declare]` is: an application layer's or a palette's expressions may read the element data a
part's declaration names, and no other (S12), so `data_needs` is the same for every stack over that preset.

A `keys` entry names a typed widget's value and lists every key the value may hold: a `Value<Key>` value's, a keyed
map's (`Vec<(Key, …)>`), a plain key list's (`Vec<Key>`, `input.context_menu_items`) and the keys inside a list's
entries at any depth (`Vec<MenuEntry>`: `window.window_menu_items`' actions and submenus). A flavour's section
(`[declare.flavor.<f>.widget.<w>]`, `[declare.flavor.<f>.component.<key>]`) states only the keys that differ under
that flavour, and each key it states replaces the widget's or component's own while the flavour is in force: its own
sizes, availability, flags, and its own variant set (`variants`, the whole set under that flavour, the variants
every flavour draws included), so a variant only one flavour draws is declared in that flavour's set alone and the
other flavour answers `NotAvailable` for it (S8).

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
corners.curve = "<curve>"

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
| the key of a field with a unit | one key per field: the suffixed key for every value of a one-unit field (`min_width_px = <n>`, `min_width_px = "= …"`, `"auto"`, `"absent"`); a `FontSize` field takes `<field>_pt` or `<field>_px` for a stated number and the bare name for an expression, `"auto"` or `"absent"` (`line_height = "= mul(font_size, <n>)"`), which carry their own units; a `Radius` corner takes `<corner>_px` for `Radius::Px` (`top_left_px = <n>` or `"= …"`) and the bare name for the other variants (`top_left = { full = { maximum_px = "absent" } }`, `top_left = { concentric = { minimum_px = <n> } }`, `top_left = "follow_part"`); `corners_px` sets the four corners' `Radius::Px` at once, beside `corners.curve` (S2.4); two keys for one field in one table are otherwise a duplicate key (S12) |
| `Value::Expr(e)` | `"= <expression>"` |
| `Value::Auto` | `"auto"` |
| `Value::Absent` | `"absent"`; a whole part: `absent = true` in its table or a rule's `set` |
| an `Option` that is `None` | `"none"`; an `Option` field or leaf of a field (`Caret.blink_timeout`, `Behavior.key_repeat`, `TextStyle.max_lines`, `PlacementParams.max_visible_items`) is stated like any other, so left out it is missing (S2.2, S2.11), never a silent `None`; an `Option` inside a list item or an enum variant's payload left out is `None` (`focus_visible_when = { keyboard = {} }`: no timeout) |
| an enum variant | a unit variant is its snake_case name (`"follow_part"`, `"continuous"`); a variant with data is a one-key table, `{ <variant> = <payload> }`: `top_left = { concentric = { minimum_px = <n> } }` (a `Corners` field is a table of its four corners and its curve, S2.4), `shadow = { elevation = "<step>" }`, `timing = { spring = { mass = <n>, … } }` |
| a list | a TOML array; where the platform has none, `[]` with its citation (a list is never `"absent"`, S2.11) |
| a tuple, a tuple variant | a positional TOML array: `center = [<x>, <y>]` (`Radial.center`, `Conic.center`), `origin = [<x>, <y>]` (`Transform.origin`); a tuple variant is a one-key table holding that array, `timing = { cubic_bezier = [<x1>, <y1>, <x2>, <y2>] }` |
| `input.kde_scheme.groups`, `.inactive_groups`, `.unset_keys` | `[inputs.kde_scheme.groups]`, `[inputs.kde_scheme.inactive_groups]` and `[inputs.kde_scheme.unset_keys]`, each with `light` and `dark`: arrays of surface sets (`["header"]`, or `[]`), and for `unset_keys` of scheme input names (`["kde_scheme.view.foreground_visited"]`, or `[]`) |
| a part drawn by another widget | `drawn_as = { widget = "<w>", part = "<p>", variants = [ … ] }` (`part` left out: the whole widget; `part` a bare name of `<w>`'s parts, so the two keys spell one `WidgetOrPart`, S1) |
| a condition | `"<name>"` in snake_case (`"window_inactive"`); with a parameter `"<name>:<value>"`: `"scheme:dark"`, `"tiled:left,top"`, `"input:touch"`, `"idiom:pad"`, `"flavor:kirigami"`, `"preset:adwaita"`, `"system_setting:<key>"`, `"container_size:sidebar:<key>"` |
| a rule's target | `"colors"`, `"tokens"`, `"<widget>.<part>"`, `"<widget>.values"`, `"component.<key>.<part>"`, `"component.<key>.values"` |
| a rule's patch | the table `set`; the keys `when`, `target` and `set` are the rule's own |
| per-contrast input bases | `[inputs.<name>.contrast.increased]`, `[inputs.<name>.contrast.high]`, each with `light` and `dark` |
| one preset's part of a layer | `[for_preset.<id>.…]` (bases naming that preset's keyed names, in an application layer or a palette) |
| widget values, component parts | `[widgets.<widget>.values]`, `[components.<key>.<part>]`, `[components.<key>.values]` |

`when` keys: `conditions`, `not_conditions`, `context` and `not_context` (arrays of `{ scope = "any" | { nearest =
[...] }, widget, part, surface, sizes, not_sizes, variants, not_variants, states, not_states }`; `widget` may be left out to match by
variant alone, and `part`, a bare name of `widget`'s parts, is stated only with it, the two keys spelling one
`WidgetOrPart`, S1), `position` (`{ among, first, last, only, parity, previous, next }`, `among` = `"same"`, `"any"` or a
list of widgets; `previous` and `next` as `{ widget, part, variants, not_variants, states, not_states }`),
`variants`, `not_variants`, `sizes`, `not_sizes`, `densities`, `not_densities`, `size_class` (`{ horizontal,
not_horizontal, vertical, not_vertical }`, each a list), `direction`, `states`, `not_states`. Every value carries its fact's citation in a comment, as today
(`scripts/check_preset_citations.py`). Palettes use the same format with `[preset] kind = "palette"` and only
`colors`, `inputs`, `tokens.kde` and colour rules.

## S12. Validation (R2.8)

Adding a layer (S8) fails with `LayerError` (listing every problem) when:

- a field is missing (S2.2), or `Absent` where not `may_be_absent` (S6.3): among the conditional marks, an `Edge.dash`
  absent on an edge whose `width` and `paint` can be drawn, an `IconStyle.fill` absent, outside an icon-size context
  (S4.5), where `rendering_mode` can be other than `Original`, or a `Glyph.fill` absent where its `stroke` can be
  absent too (S2.4, S2.9; conservatively, following references and rules);
- a keyframe of a loop stated as a widget's or a component's value leaves its `part` out (`None`): such a loop is on
  no part, so each keyframe names the part it sets (S2.7);
- a name is undeclared (S10), except a palette's typed role or surface set the current preset does not declare,
  which is skipped (S14); or a `Colors` or `Tokens` rule has non-condition selector parts;
- a `DrawnAs` names a widget the preset does not make available, a part that widget does not have, or a variant it
  does not declare (S6.1); `DrawnAs` patches form a cycle under some combination (a part drawn as another widget's
  part that is drawn, followed through `DrawnAs`, as the first);
- in TOML, a `context`, `not_context`, `previous`, `next` or `drawn_as` table states `part` without `widget` (S11;
  a `part` its `widget` lacks is an unknown name);
- an axis or feature duplicates a field's route (S2.8): `wght` or `wdth` in `Font.axes`, `wght` in `IconStyle.axes`,
  `smcp` in `Font.features`;
- `Radius::FollowPart` is stated outside a `FocusIndicator.corners`; `kde_accent` is called with group `disabled`, or
  with a set that is not one of KDE's seven colour sets (S2.2); `text_scale` is read where `token.text.scaling` can be
  `DynamicType` (S2.2, S9.4);
- `token.text.font_dpi`, or a reader's or stand-in's `ReaderState.font_dpi` or `text_scaling_factor`, is not finite
  and above zero (S9.4); a `ReaderState.container_sizes` names one container twice (one value at a time, below);
- on `kde-breeze`, a mapping palette leaves an accent-dependent role (S9.1's list, its tinting entries wherever the
  preset's `token.kde.tint_factor` can be `Some`) or an inactive input role to the preset's Breeze value, so an
  inactive window or a title bar would show Breeze's colours under the palette's (S9.2, S14); an
  `input.kde_scheme.groups` or `inactive_groups` list names a set twice or a set the preset does not declare, or an
  `unset_keys` list names an input twice or one that is not a declared `kde_scheme.<set>.<key>` or
  `.<set>.inactive.<key>` input;
- an application layer that is neither a palette nor a reader stand-in states an accent input (S9.1's
  `accent_inputs`, in its bases or a `for_preset` part): the accent is `Theme::set_accent`'s (S9.2), the one route
  that checks the derived inputs and restates `kde-breeze`'s roles;
- a map or table has a duplicate key (two TOML keys for one field included, S11); `Auto` is stated on a field
  marked `auto` neither by the catalogue for that part nor by the core (S6.3); a call is stated on a `Value<bool>`, `Value<Key>` or enum field (a
  reference is allowed); a paint has no layer, or its bottom layer is a `Mask`, or a `Shadow::Layers` no layer (S2.3,
  S2.5; an elevation entry may be empty); `pick` has no colours;
- a stated number lies outside its type's range: a negative length where the field cannot be negative (S1), a
  `Fraction` outside `0..=1`, a negative `Material.saturation` (a typo's sign; only a computed result is clamped,
  S2.2);
- an expression mistypes, refers to an unknown name, forms a cycle (through bindings too), or can meet an absent
  operand under any binding and rule it can apply under; a call can meet an operand that resolves `Auto` (S2.2:
  exempt the part's `line_height` operand and `widget.<part>.line_height`, each the line spacing under `auto`, and a
  plain reference on a field that allows `Auto`), or a plain
  reference can bring `Auto` to a field that does not allow it; a `TextStyle`
  token's expression reads `line_height` while that style's line height can be `Auto` (S2.2);
- a `widget.<part>` reference names a part of another element than the one its part is queried with (S2.2: an
  *(element)* part reads none of its host's parts, a font metric included, and a host's part none of an *(element)*
  part's), or a part that can resolve `Absent` as a whole or `DrawnAs` under some combination of the rules before it
  (following rules as for any absent operand): such a part has no fields of its own, a drawn-as part's style being
  another widget's (S6.1, S6.3);
- an expression names element data its part does not declare in the preset's `[declare]` (S10; an application
  layer's or palette's included, which may read what the part declares; root inputs need no declaration: the
  window's and screen's size), or names a corner that can resolve `Concentric` or `FollowPart`, or a corner that can resolve
  `Full` of a part other than the element's own part (the root part, or the *(element)* part it is queried with, S8),
  or a focus indicator states
  `FollowPart` on a corner whose two sides can have different offsets in either direction (S2.4);
- two `TextRoles` slots can reference one keyed text style directly, or a slot can reference a slot later in S4.4's
  owner order, under one combination (S4.4: the owner is the first of the sharing slots in that order);
- a part's `shape` can be stated while its corners' curve can be `Continuous` (S2.4);
- an expression nests deeper than 32, or a chain of references is longer than 32;
- a `Fields` patch can meet an `Absent` or `Auto` block under some combination of the rules before it, following
  references (S2.2; checked conservatively), or `base` or `Append` appears in a layer's bases;
- a field that lays the element out (a part's padding, margin, size limits and `touch_target`, widget gaps) or sizes
  its content (`text.font.size`, `text.line_height`, `text.letter_spacing`, `text.word_spacing`, `icon.size`,
  `glyph.size`) can read `element.width` or `element.height`, directly or through `part.`, `widget.`, `values.` or a
  `Full` corner (transitively), unless the element's own part (the widget's root part, or the *(element)* part the
  field's part is queried with, S8) states `min_width` and `max_width` equal (for `element.width`; `min_height` and
  `max_height` for `element.height`) under every combination it can resolve under, a fixed box (S8), or the field is
  a `glyph.size` or `icon.size`, drawn to its box, and the element's own part states that axis's minimum as a length,
  never `auto`, under every combination (S8);
- a patch sets a field its part does not have (S2.11);
- a spring has a mass or stiffness not above zero, a negative damping or a `rest_epsilon` not above zero; a cubic
  Bézier has an x outside `0..=1`; a `Steps` timing has no step, or has `jump-none` and fewer than two steps (CSS's
  own constraint: its evaluation divides by the steps less one); a `ThemeChange::Crossfade` or `Suspend` states a
  zero duration (that is `Instant`, S4.3; a computed zero, under a speed factor of zero, is not refused); a gradient
  has fewer than two stops (S2.3); a loop's keyframes' `at` do not increase, or repeat a value (S2.7);
- a token or colour-role expression names an operand only an element has (`element.`, `widget.`, `values.`,
  `part.`, `body_font_height`, and `font_size`, `line_height` outside `TextStyle` tokens) or a colour-role definition names
  `surface.` (S3.3);
- a keyed selector name (a variant, flag, condition or container size) is declared but no rule selects on it, or a
  declared keyed name no base states, or a declared event that no `presentations` list uses (a typo's sign); exempt:
  the default flavour, each container-size set's default, variants named in `requires_variants` and
  `sizes_by_variant`, what a connector sets (a content variant on the widget or *(element)* part its catalogue pair
  names, S8, and breakpoint variants), and a `SystemSetting` declared for
  applications to read (`for_applications = true` in `[declare] conditions`, S10; `Theme::system`);
- a stated number is not finite;
- a component's rule or base names a part or field that the catalogue's generated table does not give it, or a
  variant, flag, size or value its declaration does not (S5.1).

**Combinations, conservatively.** Where a check considers "every combination" of conditions and states (absent and
`Auto` operands, `Fields` over `Absent` or `Auto`, the conditional `Absent` marks, R2.8), it is decided per
combination of rule applicability, not field by field. The check evaluates the query under each assignment of the
selectors' atoms (each condition, state flag, variant, control size, density, direction, size class, position and
context match the rules name) that honours the exclusions below, every rule's and binding's applicability fixed by
that assignment: a rule that applies under it applies and one that does not is skipped, references and rules being
followed as the query follows them. A violation is reported exactly when some such assignment makes the path so
evaluated violate, rules that apply and rules that do not alike: an `Absent` from one rule and a `Fields` patch from
another fail together only under an assignment where no rule between them, applying under it too, restores the value.
The exclusions are all the check knows of the atoms: the values of each one-of parametrised condition (`Scheme`,
`Input`, `Idiom`, `DeviceModel`, `Posture`, `Flavor`, `Preset`, and `ContainerSize` per container; one value holds at
a time), the two contrast conditions (`ReaderState.contrast` is one value, S9), the flags of each declared exclusive
group, after the S10 reduction, the one-of selector parts (an element has one control size, one density, one
direction, one size class on each axis and one parity at a time, and each ancestor one control size, which a
`ContextMatch.sizes` tests per ancestor), and the model's own definitions among the atoms: among one `among` set of
siblings, `only` holds exactly when `first` and `last` do, and `first` implies odd parity; `Tiled(e)` implies
`Tiled(e')` for every `e'` whose edges `e` includes; and `Maximized` implies `Tiled` on all four edges (S7.1). Every
other combination is assumed possible, so a preset stating `Absent` under `scheme:dark` and a
`Fields` patch under `scheme:light` passes, as does a part `Absent` under `sizes = ["mini"]` with a `Fields` patch
under `sizes = ["regular"]`, and one stating them under two independent conditions fails. Deciding per combination
lets correlated settings pass that a field-by-field approximation would refuse (one rule stating `rendering_mode =
original` with `fill = "absent"`: `fill` can be absent and the mode can be other than `Original`, never together),
so the approximation is not allowed: each check is a small satisfiability problem over one part's rules and the
tokens and roles they read, solved exactly.

`Theme::add_layer` validates the layer against the theme and refuses it with the error (R2.5). A rule whose
`Preset` condition excludes the current preset is neither validated nor applied, so one application layer can carry
rules for several presets' keyed names (R2.3).

## S13. Placement (R16)

```rust
pub struct Placement { pub strategy: PlacementStrategy, pub params: PlacementParams }   // a widget value (S1)
pub enum PlacementStrategy { GtkPopover, GtkTooltip, QtMenu, QtTooltip, QtQuickTooltip, KToolTipWidget,
                             QtQuickMenu, QtComboBox, QtDialog, QtCompletionBox, QtWhatsThis,
                             KTextEditorCompletion, KirigamiTextActions,
                             WinUiFlyout, WinUiFlyoutFull, WinUiToolTip, WinUiComboBox, WinUiAutoSuggest,
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
                             pub align: PlacementAlign,            // side, align and the Sides are returned
                                                                  // physical, mirrored for rtl (S2.4)
                             pub flip: Axes<bool>, pub slide: Axes<bool>, pub width: AnchorWidth,
                             pub over_selected_item: bool, pub max_visible_items: Option<u32>,   // None: no limit
                             pub use_full_screen_height: bool, pub anchor_grow: Sides<Value<Px>>,
                             pub host: HostArea, pub window_edge_margin: Value<Px>,
                             pub handedness: bool }               // the menu drop alignment, set by the preset's
                                                                  // rule under `system_setting:menu_drop_alignment`
                                                                  // (S4.2; Win32 menus; WinUI flyouts to verify, W3-55†)
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
list is the traceability's placement rows, each implemented (R16). `params` are physical as the query returns them
(S2.4), so `place()` does not mirror them again; `direction` is the element's, for what a strategy's source reads of
the direction beyond those parameters. An optional input a strategy needs and lacks is
taken as: no pointer, the anchor's start corner; no selected item, the first item; no keyboard, none shown.

## S14. Palettes (R19)

A palette is a file in S11's format with `[preset] kind = "palette"`: colour-role bases, input bases and colour
tokens for the schemes its project defines (R19). Two sources, each stated in the file's header:

- **native-theme's mapping** (the default): each role it covers maps to a colour the project publishes, named by the
  project's name for it (`# catppuccin: Surface0`), or to a derivation of those (S2.2); a value picked from a range
  the project's guide gives is marked `# pick: <the guide's words>`. It is the same file on every platform.
- **the project's port** for one platform, where one exists (Catppuccin's and Dracula's KDE colour schemes, R19):
  `Palette::from_port(file, scheme) -> Result<Palette, PortError>` converts a KDE `.colors` file mechanically, and
  that palette is used on `kde-breeze` in place of the mapping. A `.colors` file does not say whether it is light or
  dark, so the caller names its one scheme: each bundled palette states it (Catppuccin Latte light, its other flavours
  and Dracula dark, R19), and an application loading a port names it (no luminance guess).

A palette's bases name typed roles, the `accent` input and the declared inputs of the presets it has sections for; a
typed role or surface set the current preset does not declare is skipped, not refused (S12). On `windows-11`,
`macos-*` and `ios-*`, whose accent derivations are not published (R2.6), the mapping states the preset's accent
inputs (S9.1) in a per-preset section (`[for_preset.windows-11.inputs]`), each labelled as native-theme's pick (R19), and
`apply_palette` refuses a mapping that lacks one of the current preset's (`Err(AccentInputsMissing)`), as `set_accent`
does (S9.2). On `kde-breeze` (S9.1), a mapping states the accent-dependent roles and the inactive input roles itself, in `[for_preset.kde-breeze]`
(no `kde_accent`: a mapping has no scheme inputs, and the preset's would be Breeze's): each accent-dependent role as
the project's colour or by reference to `input.accent` where it follows the accent; each inactive input role as the
project's colour where it publishes an inactive one, else as its own set's active role, `= active(role.<set>.<role>)`,
which is how KColorScheme reads a scheme with no `[Inactive]` group (`kcolorscheme.cpp`), and a mapping is one; and
each `[WM]` keyed role as the project's colour, or as `applyScheme()`'s default takes it from the `Header` set (S9.1):
the active ones by reference to its `Header` background and foreground (`= active(role.header.background)`, `=
active(role.header.foreground)`), the inactive ones as the inactive colour rule computes the `Header` set's
background and foreground, `kde_effects(…, inactive, …)` over its own `input.inactive.header.*` (the catalogue's KDE
inactive rule). One
that leaves such a role to the preset is refused (S12): an inactive window or a title bar would otherwise show
Breeze's colours under the palette's. A KDE port is a scheme file, converted by `applyScheme()`'s rules as Plasma's
default path writes it ("Accent color from color scheme", no accent), the rule the preset follows (S9.1), not as the
raw file reads, so the port shows what Plasma shows for that scheme and an accent set later changes only what
Plasma's derivation changes. It converts to the scheme inputs (`kde_scheme.<set>.<key>` and `.inactive.<key>`, the
file's values, a group or key it lacks as S9.1 states, as KColorScheme reads it unset; `kde_scheme.groups`,
`inactive_groups` and `unset_keys` from the file, a set `inactive_groups` does not name stated as S9.1 says; and
`kde_scheme.wm.*` from its `[WM]` group, or for a key the file lacks the `Header` default S9.1 states); to `token.kde`
(S4.8); to the colour roles as that path writes them, the file's own values with no accent
(`colorsapplicator.cpp:225-226`), and for a group or key it lacks what KColorScheme then reads, as the scheme inputs
hold it (S9.1: Dracula's port, which has no `Header` or `Complementary` group, gets `Window`'s values for `Header`,
`kcolorscheme.cpp:330-333`, and KColorScheme's defaults for `Complementary`); each inactive input role as that path writes it (`:247-284`): for a set the
list names, in a scheme that does not tint, `Header`'s the file's active `Window` value (Plasma 6.8's substitute) and
another set's its `[Inactive]` group's value, and in one that tints, the group as that path writes it, read from the
same code `kde_accent` implements; for a set the list does not name, `= active(role.<set>.<role>)`; and each `[WM]`
keyed role as `= input.kde_scheme.wm.<key>`, `applyScheme()`'s no-accent result. No group or key is required, so
`from_port` converts a file that lacks some, as Plasma applies it, and fails only on one it cannot parse (`PortError`,
S1); S19.1's build test validates every bundled port, Dracula's among them. So Plasma's derivation applies over the
port's own scheme when an accent is set (S9.2), and a non-tinting port's inactive title bar is, without the
accent-coloured title bar option, the same with an accent as without, as in Plasma (`kde_accent`'s non-tinting branch
gives `Header` the same `Window` values; under the option, where the file has a `Header` group, the accent tints it,
S2.2).

A `Palette` holds the mapping and the ports it has, per preset (a port carries its preset: a `.colors` file's is
`kde-breeze`); `Palette::with_port(self, port)` adds the ports another palette holds, one replacing another for the
same preset, and is refused where the port's scheme is not one of the palette's (`Err(SchemeNotInPalette)`), so an
application builds a mapping (`from_toml`) with its project's port (`from_port`) as the bundled palettes are built;
`Theme::apply_palette(Some(palette))` adds, as an
application layer that replaces the reader's colours (S8), the port for the current preset where there is one (a KDE port
converts to the `kde_scheme.*` inputs, the colour roles and `token.kde`), else the mapping. The source is chosen per
preset and scheme: where a preset has a port with fewer schemes than the mapping (a one-scheme `.colors` file beside
a two-scheme mapping), the port serves its scheme and the mapping the palette's others, so each scheme has one
source and the palette applies under every scheme it declares; the port's `token.kde` then applies under its scheme
only (a `Tokens` rule under that scheme), the mapping keeping the preset's under the others. It refuses a palette
under a scheme it lacks (`Err(SchemeNotInPalette)`); when the system scheme changes to one the palette lacks, the
palette is suspended, the theme is the preset with the reader's values, and the palette applies again when the
scheme returns (R19); `ResolvedTheme::palette_status()` reports `Applied`, `Suspended` or `SkippedForContrast` (`None`:
no palette). A palette carries no icon set. A palette states its schemes in its `[declare] schemes` (S10);
`Theme::pick(Some(s))` of a scheme the applied palette lacks is refused like `apply_palette`, while `pick(None)`, which
follows the system's scheme, suspends the palette while that scheme is one it lacks, as a system change does; a
`set_preset` to a preset whose accent inputs the applied mapping's per-preset section lacks is refused
(`Err(AccentInputsMissing)`), as `apply_palette` refuses it; a palette that has neither a
mapping nor a port for the current preset (a KDE port alone, on `adwaita`) is refused (`Err(PaletteNotForPreset)`).
The palette's source, port or mapping, is chosen again on every change of the stack, as the accent layer is derived
again (S8), so `set_preset` switches it to the new preset's; a `set_preset` to a preset the applied palette can
serve with neither is refused whole (`Err(PaletteNotForPreset)`, S8), as `apply_palette` refuses it, and the
application removes the palette first (`apply_palette(None)`) to switch. A mapping palette need not state
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

It returns `None` for a `ControlSize` AppKit has no size for on the running macOS (which sizes it has per version is
the facts', S17), as the preset declares no such size there (S7.3).

What each image covers: `AppKitCheckbox`, `AppKitRadio`, the box and mark; `AppKitDisclosure`, the triangle;
`AppKitPushDisclosure`, the bezel and chevron; `AppKitPopUp`, the whole button at `width`, its arrow area being the
cell's `indicatorRect` (`ControlImage.arrow`). The rendering uses the system's accent; an accent the
application or a palette sets cannot reach it, which the connector's contract records as its limit (R11, R23).

On macOS's main thread it renders; anywhere else, and off macOS, it returns `None`, so connectors need no `cfg` of
their own. Cached per (control, states, size, appearance, key, scale, width). **Approved by the maintainer (2026-10-29):** rendering through AppKit uses `unsafe` objc2 calls under the same
exception as the SF Symbols loader (`native-theme/src/sficons.rs`): module-scoped, each block with its safety
argument. A glyph naming a system control
(`GlyphSource::SystemControl { control, replaces }`) states the parts the rendering replaces (the checkbox's box and
mark together, R11). The list of `SystemControl`s is the traceability's rows that need it.

## S16. The API surface

`SystemTheme` is renamed `Theme`, the one owner of the stack, the inputs and the application's conditions.
`AccessibilityPreferences` goes: the reader reports conditions, the text-scaling factor and the font DPI in its
`ReaderState`. Today's `ColorMode` is `Scheme`, and today's `Rgba` is `Color` (S2.1).
Every mutation validates the whole stack and is refused whole on failure, the theme staying as it was (S8, S12):

```rust
impl Theme {
    pub fn from_system(generation: Option<PresetId>) -> Result<Theme, ThemeError>;
                                                          // reads the system; the preset is `generation`, the
                                                          // application's opt-out to another generation of this
                                                          // platform (R1.5; `Err(PresetNotForPlatform)` otherwise),
                                                          // else the one the OS version and the app's declared
                                                          // opt-out choose (as `pipeline::preset_for_version` does);
                                                          // a failed first read is `Err(Reader { preset, failure })`
    pub async fn from_system_async(generation: Option<PresetId>) -> Result<Theme, ThemeError>;
                                                          // the same; the portal reads are D-Bus calls, which an
                                                          // application inside an async executor awaits rather
                                                          // than blocking it (`from_system` blocks on them)
    pub fn from_preset(id: PresetId, scheme: Scheme) -> Result<Theme, ThemeError>;   // no reader: the no-reader
                                                          // state below, with this scheme
    pub fn set_preset(&mut self, id: PresetId) -> Result<(), LayerError>;          // a theme change: the reader
                                                          // applies its portable part (accent, scheme, fonts, text
                                                          // scaling, conditions) to any preset and its
                                                          // `[for_preset.<id>]` part and preset-conditioned rules
                                                          // to its own; the accent layer is derived again and the
                                                          // palette's source chosen again for the new preset (S9.5,
                                                          // S14); the flavour is kept where the new preset declares
                                                          // it, else it becomes the new preset's default (its first
                                                          // declared, S10; none where it declares none), which
                                                          // `flavor()` reports
    pub fn set_reader_stand_in(&mut self, state: ReaderState, layer: Layer) -> Result<(), LayerError>;
                                                          // sets it, or replaces the one set (the facts gate replaces
                                                          // it per combination, S17); refused while a reader exists
    pub fn remove_reader_stand_in(&mut self) -> Result<(), LayerError>;   // back to the no-reader state (below);
                                                          // a no-op without a stand-in
    pub fn pick(&mut self, scheme: Option<Scheme>) -> Result<(), LayerError>;     // None: follow the reader's or
                                                          // stand-in's scheme, else the one `from_preset` was given;
                                                          // under a palette as S14 says (Some: refused for a scheme
                                                          // it lacks; None: suspends it while the system's is one)
    pub fn set_flavor(&mut self, flavor: Key) -> Result<(), LayerError>;          // undeclared: refused
    pub fn preset(&self) -> PresetId;                     // the current preset
    pub fn flavor(&self) -> Option<Key>;                  // the flavour in force: set, or the preset's default;
                                                          // None on a preset that declares no flavours (R1.6)
    pub fn scheme(&self) -> Scheme;                       // the scheme in force: `pick`ed, else the reader's or
                                                          // stand-in's, else the one `from_preset` was given
    pub fn version_match(&self) -> Option<VersionMatch>; // how the running OS version relates to the current
                                                          // preset's generation (R1.5: macOS 27 under
                                                          // `macos-tahoe` is `PresetOlder`); None where the platform
                                                          // has no generation table (Linux), the version cannot be
                                                          // read (the reader's feature off), or the current preset
                                                          // is not a generation of the running platform
    pub fn add_layer(&mut self, layer: Layer) -> Result<LayerId, LayerError>;
    pub fn replace_layer(&mut self, id: LayerId, layer: Layer) -> Result<(), LayerError>;
    pub fn remove_layer(&mut self, id: LayerId) -> Result<(), LayerError>;   // both: `Err(UnknownLayer(id))` for an id
                                                          // this theme does not hold, as a stale id is the caller's
                                                          // error (S1)
    pub fn set_accent(&mut self, accent: Option<Accent>) -> Result<(), LayerError>;   // None: the system's again
    pub fn apply_palette(&mut self, palette: Option<Palette>) -> Result<(), LayerError>;  // None: removes it;
                                                          // refused as S14 says (a scheme it lacks, no source for
                                                          // the preset, missing accent inputs)
    pub fn reader_status(&self) -> ReaderStatus;         // `Fallback` where a read succeeded with a source standing
                                                          // in (S9.1): `from_system` returns the theme then
    pub fn reload_reader(&mut self) -> ReaderStatus;     // re-read the system into the reader layer in place
                                                          // (a failure keeps the old layer); the application calls it
                                                          // from its event loop on the watcher's `on_theme_change`
    pub async fn reload_reader_async(&mut self) -> ReaderStatus;   // the same, awaited (as `from_system_async`)
    pub fn system(&self) -> Option<&ReaderState>;         // the conditions the reader or stand-in reports, for what
                                                          // an application acts on itself (Windows' ShowSounds,
                                                          // W-P3-11†); None without either
    pub fn set_contrast(&mut self, contrast: Option<Contrast>) -> Result<(), LayerError>;  // None: the reader's
    pub fn resolve(&self, window: &mut WindowState, conditions: &WindowConditions) -> Resolution;   // S8
}
impl ResolvedTheme {
    pub fn icon(&self, scope: &Scope, id: IconId<'_>, style: &ResolvedIconStyle) -> Option<IconData>;
                                                          // from `token.icons`' set, theme and theme fallback (S4.5),
                                                          // in `style` (size, weight, scale, rendering mode, symbol
                                                          // variants, axes; its fill where the set recolours), for the
                                                          // direction and language in `scope` (the scope the part's
                                                          // query returned, which carries the element's own) and the
                                                          // role's `token.icons.mirroring` (S2.9, R25.8); None where
                                                          // the set has no such icon or lacks its feature
}
impl PresetId {
    pub fn all() -> &'static [PresetId];                  // every bundled preset, in a stable order
    pub fn for_platform() -> Vec<PresetId>;               // those whose platform is the running one, and `material-3`
    pub fn for_system() -> PresetId;                      // the preset the platform, the detected desktop and the OS
                                                          // version choose, as `from_system(None)` does, without
                                                          // reading the theme or asking the portal (on Linux the
                                                          // desktop the last read refined, as `platform_preset_name`
                                                          // does today); where the version cannot be read (the
                                                          // reader's feature off), the oldest generation's
    pub fn key(self) -> &'static str;                     // `"kde-breeze"`: S11's `id`, `preset:<key>`, `for_preset`
    pub fn from_key(key: &str) -> Option<PresetId>;
    pub fn display_name(self) -> &'static str;            // `"KDE Breeze"`
}
impl PaletteId {
    pub fn all() -> &'static [PaletteId];                 // every bundled palette
    pub fn key(self) -> &'static str;
    pub fn from_key(key: &str) -> Option<PaletteId>;
    pub fn display_name(self) -> &'static str;
}
impl Palette {
    pub fn bundled(id: PaletteId) -> Result<Palette, LayerError>;   // native-theme's mapping, with the ports it
                                                          // has; a build test parses and validates every bundled
                                                          // palette, so the error is never met in a release
    pub fn from_toml(text: &str) -> Result<Palette, LayerError>;
    pub fn from_port(file: &str, scheme: Scheme) -> Result<Palette, PortError>;   // S14
    pub fn with_port(self, port: Palette) -> Result<Palette, LayerError>;          // S14
}
```

**Without a reader.** A theme with neither a reader nor a stand-in (`from_preset`) resolves under the **no-reader
state**: the scheme `from_preset` was given (or `pick`ed), no contrast, every flag false, no idiom, device model,
container size, system setting or content-size category, a text-scaling factor of 1 and `font_dpi: None` (the
preset's token converts its points, S9.4). Nothing in it is a reading, so `system()` returns `None` and
`reader_status()` `NoReader`; the facts gate's base conditions are this state under each scheme (S17). A failed first
read in `from_system` is an error carrying the preset it chose, not a silent fallback: the application that wants the
preset alone calls `from_preset(preset, scheme)` with the scheme it chooses, so no scheme is assumed for it. A later
failed re-read keeps the previous reader layer and `ReaderState` (S8).

**Today's API, feature by feature** (S19.3 migrates the branch's callers):

- *Preset listing* (`Theme::list_presets`, `list_presets_for_platform`, `PresetInfo`): kept, as `PresetId::all`,
  `PresetId::for_platform` and `PresetId`'s `key` and `display_name` (`PresetInfo`'s fields; the platform tags stay
  internal, read by `for_platform`); the community presets, now palettes, are listed by `PaletteId::all`. The showcases'
  pickers and an application's own still need them.
- *`from_system_with_preset` and the two `_async` constructors:* `from_system_with_preset` is replaced by
  `from_system(Some(generation))`, one constructor with the generation an optional input (R1.5's opt-out; it differs
  from `from_system(None)` and `set_preset`, as the reader's preset-specific part follows the generation it is built
  with, S9.5); `from_system_async` and `from_system_with_preset_async` become `from_system_async(generation)`, and
  `reload_reader_async` is added, because the portal reader is asynchronous.
- *Icon loading* (`SystemTheme::icon`, `icon_theme_for`): `icon` is kept as `ResolvedTheme::icon` above, which takes the
  part's resolved icon style and the scope's direction and language (R25.8); `icon_theme_for(mode)` is replaced by the
  resolved `token.icons` (`ResolvedTheme::tokens(scope).icons`: set, theme, theme fallback), which rules state per
  scheme (`breeze`, `breeze-dark`) and which a re-resolution after a scheme change gives anew (S8). The `icons`
  module's free loaders (a set and theme the caller names, custom icons through `IconProvider`) stay for icons outside
  the theme, with `Color` for `Rgba`; `icon_indicator` and `load_icon_indicator` go (S2.7).
- *`Theme::to_toml`, `Theme::lint_toml`:* removed. There is no whole theme to write: a look is an application layer an
  author writes in TOML (S11); and `Layer::from_toml` and `Palette::from_toml` refuse every unknown key, each a
  `Problem` with its TOML key path (S12), which is what linting reported.
- *The `detect` functions:* `system_is_dark`, `detect_is_dark`, `prefers_reduced_motion`, `detect_reduced_motion` and
  `accessibility_preferences` are replaced by `Theme::system()` (the reader's `ReaderState`); `system_icon_set`,
  `system_icon_theme` and its other forms (`detect_icon_theme`, `DetectionContext::icon_theme`) by the resolved
  `token.icons`, which the reader states where the platform has a setting. The
  desktop detection (`LinuxDesktop`, `detect_linux_desktop`, `parse_linux_desktop`, `detect::system()`) and
  `invalidate_caches` stay: they are not theme values, `native-theme-build`'s generated icon code reads the desktop,
  and `reload_reader` clears the caches it reads through.
- *`ResolutionContext`* (`font_dpi`, `button_order`, `icon_theme`): removed, each field having its place in the
  model: the font DPI is `ReaderState.font_dpi` over `token.text.font_dpi` (S9.4), the dialog's button order its
  widget value (`dialog.values.action_layout.row_order`), the icon theme `token.icons.theme`.
- *`SystemTheme`'s fields:* `preset` is `Theme::preset()` (the gpui showcase reads it for its picker's default);
  `mode` is `Theme::scheme()`; `version_match` is `Theme::version_match()`, kept with `VersionMatch` (R1.5's case,
  an OS newer or older than the chosen generation; the type loses `#[non_exhaustive]`, S21.2); `light` and `dark`
  are `Theme::resolve` under each scheme (S8): `resolve` takes the scheme in force, so a caller wanting both
  `pick(Some(..))`s each in turn and resolves (a palette refusing a scheme it lacks, S14), then restores its choice; `icon_set` and `icon_theme` the resolved `token.icons`; `layout` the
  `layout` widget's values (the catalogue); `accessibility` `Theme::system()`. `name` (the KDE colour scheme's name,
  or the preset's) is removed: a scheme file's name is not a theme value, and its one caller, the gpui connector,
  needs only a name for its toolkit's theme, which it takes from `Theme::preset().display_name()`.
- *`pipeline::platform_preset_name()` and `PlatformPreset`:* replaced by `PresetId::for_system()`, the same choice
  without reading the theme; the three showcases call it for their pickers' default where the system read fails,
  and the egui connector's font tests for the platform's preset.
- *`pipeline::diagnose_platform_support()` and `DiagnosticEntry`:* kept: they report what the readers' detection
  depends on (the desktop, environment variables, tools, configuration files, features, the OS generation), which the
  model does not change, for an application to show when a read fails; `DiagnosticEntry::OsGeneration.preset`
  becomes a `PresetId`, and the enum loses `#[non_exhaustive]` (S21.2).
- *`SystemTheme::with_overlay`:* replaced by `Theme::add_layer` (R2.4): an application's look is a layer in the
  stack, validated when added (S8, S12).
- *Helpers outside the model:* `color::composite_over` is kept, over `Color` (a translucent colour or appended
  layer composited over what lies beneath, which connectors do, R5.3); the `fonts` module (`select_face`,
  `system_face`, `substitute_family`, `substitute_family_cached`) is kept: it finds a resolved font's face on the
  system, which is not a theme value; today's `Theme::preset(name)`, which loads a whole theme by name as
  today's `Theme::from_toml` does, goes with it (below, R21), replaced by `Theme::from_preset(id, scheme)` and `Palette::bundled(id)`; the hidden `kde::from_kde_content_pure`
  stays the KDE reader's test entry, returning the `ReaderState` and reader layer it builds from the kdeglobals and
  scheme-file contents it is given.
- *`crate::Error` and `crate::Result`:* kept for the APIs that are not theme values, with the variants they return:
  `PlatformUnsupported`, `WatchUnavailable` and `ReaderFailed` (`watch::on_theme_change` and its backends,
  `rasterize::rasterize_svg`; the `icons` loaders return `Option`). The variants the theme's APIs returned go with
  them: `FeatureDisabled` (now `ThemeError::FeatureDisabled`), `NoVariant`, `Toml`, `Io` (`Theme::from_toml`,
  `from_file`, `pick_variant`), `UnknownPreset` (a `PresetId` is never unknown; `PresetId::from_key` returns `None`),
  `ResolutionIncomplete` and `ResolutionInvalid` with `RangeViolation` (validation is `LayerError`'s, S12).
  `ErrorKind` and `Error::kind()` go: the
  three variants left are matched directly, the enum being exhaustive (S21.2). The theme's own errors are S1's.

A look of the application's own is an application layer over a bundled preset (`Layer::from_toml`, S1), never a whole
preset of its own: today's `Theme::from_toml` and `Theme::from_file`, which load a whole theme, go (R21), so every
look starts from a preset the facts gate checks.

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

**The gate** (`native-theme/tests/facts_gate.rs`): it resolves each platform preset with a stand-in `ReaderState` and no
reader, so the machine's own settings never enter: by default the no-reader state's conditions (S16) under each
scheme in turn, factor 1 and no font DPI of its own (`font_dpi: None`, so the preset's token converts its points). For every widget and
every combination of its state model, it replaces the stand-in with one holding the combination's reader conditions (and
passes its window conditions in `WindowConditions`), resolves, and queries the element, and compares each key with the
fact that applies (within tolerance); a key and combination with no fact fails (R4). It compares every token,
colour-role and input fact the same way, under its `when`.

**The perturbed run** (R4's derivation check) repeats this with the stand-in's layer stating, under each scheme, a
fixed colour for every declared input and for each colour role a derivation fact names as an operand, the colours
pinned in the gate's source and documented there, so the run is reproducible. It compares only the keys whose fact is a
derivation, evaluated over the same perturbed values in the same scope: a literal fact is the preset's own base
colour, which the perturbation replaces, so comparing it would always fail, while a preset stating a literal where
the platform derives fails here. iOS's facts are `docs/facts/ios/…` like any platform's (R22).

**The accent run** (`kde-breeze`): neither run sets an accent, and the preset states Plasma's no-accent path (S9.1), so
`kde_accent` and `kde_accent_wm`, the model's implementation of `applyScheme()`, are checked by a run of their own.
For each KDE scheme the facts hold (the scheme files the preset's inputs come from, the bundled ports, and the further
`.colors` files the facts pin as sources) and each accent the facts were measured with, the gate resolves `kde-breeze`
with that scheme's inputs in force (the preset's own for its files; for every other file a palette converted by
`Palette::from_port` and applied, under the scheme the facts name for it) and the accent set through `set_accent`, and compares every accent-dependent role, every inactive input
role and the four `wm_*` keyed roles with the facts recorded from Plasma with that scheme and accent applied (kind
`measured`: `applyScheme()`'s output in kdeglobals, read back as the KDE reader reads kdeglobals). Each such fact names, beside
its `when`, the scheme file and the accent it was measured with (`scheme_file = "<file>"`, `accent = "<colour>"`). The
schemes include at least one that tints, one with the accent-coloured title bar option and a `Header` group, one with
that option and no `Header` group, and one that lacks a colour group the others have; the value phase chooses the
files and the accents and records them as sources.

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
variant the project publishes, their invented light variants and their `icon_set` removed. A build test parses and
validates every bundled preset and palette (each palette over every preset it has a source for, Dracula's KDE port,
which lacks two colour groups, among them, S14), so `Theme::from_preset` and `Palette::bundled` never meet their error in a
release (S16).
**S19.2** Today's inheritance rules (`[uniform]`, `[per_platform]`) become expressions in each preset's bases,
converted by a script from `docs/inheritance-rules.toml` before it is removed, then checked by the gate; the
`[wrong_safety_nets]` are not converted, as the facts replace them.
**S19.3** Today's `AnimatedIcon` and its loaders go (S2.7): the connectors and showcases draw a busy indicator from the
`spinner` widget's parts and `loop`. Today's `Theme::from_toml` and `Theme::from_file` go (S16): the branch's own
callers (examples, tests) state their looks as application layers, loaded with `Layer::from_toml`. The branch's
callers of today's other API move as S16's feature list says: preset pickers to `PresetId` and `PaletteId`,
`from_system_with_preset` to `from_system(Some(generation))`, `icon` and `icon_theme_for` to `ResolvedTheme::icon` and
the resolved `token.icons`, the `detect` scheme and accessibility functions to `Theme::system()`, `ResolutionContext`
to its fields' places in the model, `SystemTheme`'s fields to `Theme`'s getters (the gpui connector's theme name to
`Theme::preset().display_name()`), `pipeline::platform_preset_name` to `PresetId::for_system`, `with_overlay` to
`add_layer` (the iced and gpui showcases and the connectors' READMEs), and `crate::Error`'s theme variants to S1's
error types; the tests of `to_toml` and `lint_toml` go with them.

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
**S21.2** Every public enum is exhaustive (S1), so a connector maps each in full: `IconRole`, `IconSet`,
`watch::ThemeChangeEvent`, `VersionMatch`, `DiagnosticEntry`, `crate::Error` and every other enum of today's API
that stays lose their `#[non_exhaustive]`. Every public
struct has it, except the two groups S1 names: the closed value structs, and the inputs their caller fills in full
(`WindowConditions`, `RootInputs`, `SizeClasses`, `Sibling`, `PlacementInput`, `SystemAppearance`, `ReaderState`),
which are exhaustive for the same reason as the enums (R21).

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
| ADD-A89 | `FadeUnder` | S2.8 |
| ADD-A90 | `uikit_prominence`, `gtk_wheel_step`, `shadow_extent`, `kte_mark_underlay` | S2.2 |
| ADD-A91 | `ScrollPhysics.touchpad_factor` | S4.2 |
| ADD-A92 | `SharedFlag::TextSelected` | S7.2 |
| ADD-A93 | `Mirroring::SwapWith` | S2.9 |
| ADD-A94 | `gtk_contrast_pick` | S2.2 |
| ADD-A95 | `Radius::FollowPart`, `Breakpoint`, `sizes_by_variant`, touchpad inertia, `Theme::set_contrast`, `WindowState` | S2.4, S7.3, S10, S4.2, S16, S8 |
| ADD-A96 | `Comparison`, `Measured`, `OverlayBottomCenter`, two-edge fade, touchpad velocity samples, `Theme::system`, decoration patterns | S7.3, S13, S2.8, S4.2, S16 |
| ADD-A97 | `PartPatch::DrawnAs`, `Styled::DrawnAs`, `RuleBody`, `Keyframe.part`/`timing`, `Glyph.draw_start`, `FontMetrics::line_spacing` | S6.1, S8, S2.7, S2.9 |
| ADD-A98 | `SharedFlag::DropCopy`/`DropMove`, `FontMetrics::max_char_width`, `DrawnAs.part`, the concentric container | S7.2, S8, S6.1, S2.4 |
| ADD-A99 | `ShapeKind` in code, `GlyphSource::Path.view_box`, `TextStyle.scaling_role`, closed connector inputs, the concentric helpers, `WinUiFlyoutFull` | S2.4, S2.9, S2.8, S1, S13 |
| ADD-A100 | `Caret.blink_timeout` as `Option`, `body_font_height`, another part's font metrics | S4.2, S2.2, S10 |
| ADD-A101 | `kde_selection_inactive` (KDE's Selection set in the Inactive group, from `kcolorscheme.cpp`); `TextRole` defined; `MaterialShape` as `MaterialShapes.kt`'s list; the `corners::{full, concentric, follow_part}` signatures and the container's computed radius; the display radius as four keys; `IconStyle.rendering_mode` as a `Value`, absent fields in icon-size contexts; `display_p3(…)` colour literals; lazy patches over references; exclusive conditions in validation; `ResolvedTheme::palette_status`; the palette's source chosen per preset (`set_preset` refused with `PaletteNotForPreset`); line heights per `TextScaling` kind; `Option` leaves stated; TOML keys of unit fields | S2.2, S1, S2.4, S4.6, S2.9, S4.5, S2.1, S12, S8, S14, S16, S9.4, S11 |
| ADD-A102 | `div` | S2.2 |
| ADD-A103 | the font DPI (`token.text.font_dpi`, `ReaderState.font_dpi`, points converted before text scaling, R10); `Radius::Full.maximum` and `corners::full`'s `maximum`; `Path`, `PathCommand`, `Outline` and `morph` over corners and shapes; `[declare] symbol_scales`, `spec_versions`; `KdeEffects` amounts as `f32`; `kde-breeze`'s accent-dependent roles as `kde_accent` in the preset, palettes' accent rules; `Palette::bundled` fallible; integer conversion and the clamps of the totality rules; one route for weight, width and small caps; lists never absent; `auto` maxima; `Placement`; the error and status types; component value types | S2.2, S2.4, S2.8, S2.11, S4.4, S4.8, S9.1, S9.2, S9.4, S10, S12, S13, S14, S16, S1 |
| ADD-A104 | key repeat as `Option`, `Measured::Part`, undeclared keyed flags | S4.2, S7.3, S8 |
| ADD-A105 | `OutlineCorners` (resolved radii for `outline`/`morph`); construction routes (`SchemeMap::new`/`one`, `InputValue::new`, `ComponentValuesPatch::new`, `ContextMatch::new`, `KeyRepeat::new`; `SystemAppearance` exhaustive); the no-reader state, `from_preset(id, scheme)`, `Theme::system() -> Option`, `ThemeError::Reader`/`PresetNotForPlatform`; today's API decided (`PresetId`/`PaletteId` listing, `from_system(generation)`, `from_system_async`, `reload_reader_async`, `ResolvedTheme::icon`, `ResolvedIconStyle`, `IconId`/`IconData` kept; `to_toml`, `lint_toml`, `ResolutionContext`, the `detect` scheme and accessibility functions removed); preset-scoped reader rules; `[declare] accent_inputs`; accent inputs refused in application layers; inactive input roles under a mapping palette and in a port; `kde_scheme.<set>.inactive.<key>`; `FocusVisibleWhen::Keyboard.timeout` as `Option`; one route for a solid line (`Edge.dash` absent only on an edge not drawn); `KeyRepeat`; `Palette::from_port(file, scheme)`, `with_port`; the perturbed gate run; content-sizing fields in the layout ban; `ViewBox`; `FadeLength`; `IconStyle.fill` absent under `Original` only; zero theme-change durations refused; elevation references resolved to layers; stand-in factor and container-size checks; `container_sizes`' default | S2.2, S2.4, S2.5, S2.8, S2.9, S2.10, S4.2, S4.3, S6.3, S7, S8, S9, S10, S12, S14, S16, S17, S1 |
| ADD-A106 | the reader's accent portable only where `accent_inputs` is the accent alone | S9.5 |
| ADD-A107 | `auto` minimums, the `sides` value type, `keys` declarations, root inputs undeclared | S2.11, S10, S12 |
| ADD-A108 | today's API decided further (`Theme::preset`, `flavor`, `scheme`, `version_match` with `VersionMatch` kept; `PresetId::for_system` for `platform_preset_name`; `diagnose_platform_support` kept; `SystemTheme.name` removed; `with_overlay` as `add_layer`; `crate::Error` narrowed to the non-theme APIs); the flavour across `set_preset`; `ContentSizeCategory`; `Auto` never a call's operand; `active(ref)` defined; colour literals in expressions; a shared text-role slot references the slot it shares, and the slots a reader patches; `element.width`/`height` as the queried element's box, `Full` corner operands on that element's part only; the font's underline and strikeout metrics (`FontMetrics`, `element.font_underline_offset`, …) and `Decoration`'s offset convention; the reader's accent left out under another preset with derived accent inputs; `Sides` and `Axes` members; the stroke table's shorthands; one form for "no shadow"; `morph` over circular corners only; `DrawnAs` cycles and `widget`/`part` disagreement refused; exclusive groups on ancestors' and siblings' states; `pick(None)` suspends a palette; `set_preset` refused for a mapping's missing accent inputs; `MaterialScheme.contrast_level` always stated; `SizeClasses` exhaustive; the inactive input roles' path | S1, S2.2, S2.4, S2.5, S2.8, S4.4, S4.8, S6.3, S7.1, S8, S9.1, S9.5, S12, S14, S16, S19.3, S21.2 |
| ADD-A109 | `kde-breeze` states Plasma's default no-accent path; `kde_accent` only when an accent is set | S9.1, S14 (R25.1) |
| ADD-A110 | `keys` for key lists and keys inside list entries, a flavour's own variant set; content variants as (widget or part, variant) pairs; `WidgetValue`'s font size, `Vec<char>`, `Vec<Key>`, `Vec<MenuEntry>`; `kde_accent_wm`, `KdeWmKey` and the `kde_scheme.wm.<key>` inputs, a port's `[WM]`, a mapping refused where it leaves the `wm_*` roles; `Linear.offset`; `BlendMode::Mask`; widget-value loops' keyframes name their part; `Glyph.fill` absent on a stroked-only path; a glyph or icon drawn to its box, `DataNeeds`' flag; a `drawn_as` substitute with its host's states; a font size scaled once; `widget.<part>.line_height` as the line spacing; no *(element)* part reads its host's parts; no transition is an immediate change; a stroke corner painted once; transforms mirrored in right-to-left; `pointer_repeat`; `Stroke.outset`'s comment | S1, S2.2, S2.3, S2.4, S2.6, S2.7, S2.9, S4.6, S4.8, S6.3, S8, S9.1, S9.2, S9.4, S10, S12, S14 |
| ADD-A111 | the final check's fixes: `KdeTokens.tint_factor` as `Option` and every `token.kde` value stated (the reader's and a port's unset keys as KColorScheme and `applyScheme()` read them), no KDE function branching on absence; `input.kde_scheme.inactive_groups` (`InputBases.kde_inactive_groups`), every set's inactive scheme inputs declared; `KdeSchemeKey`; a KDE port converted by `applyScheme()`'s rules; the accent-dependent roles listed; a missing KDE scheme file a reader failure; corners, stroke edges and shadow offsets physical in right-to-left; `WidgetOrPart`; `GlyphMirroring`; `Elision::Fade`'s ends as `Option`; letter and word spacing as `FontSize`, scaled as a line height; `ScrollDecay`, `DecayModel` and the `native_theme::scroll` functions; the new resolution's theme-change token; a plain part's scope and context selectors skipping the element's own parts; `widget.<part>` in its own scope; the one-of selector parts and the per-combination precision of the checks; element data declared by the preset and readable by application layers, the declaration names; the text-role owner order; `ContainerSize`'s default holding; `for_applications` settings; one route for single-click and menu drop alignment; gradient angles, at least two stops, `Steps` with `jump-none`, keyframe order; tuple spellings, negative literals; `remove_reader_stand_in`; resolving both schemes; a port's scheme beside a two-scheme mapping; today's helper APIs decided | S0, S1, S2.2, S2.3, S2.4, S2.5, S2.6, S2.7, S2.8, S2.9, S2.11, S4.2, S4.3, S4.4, S4.8, S6.1, S6.3, S7, S7.1, S7.3, S8, S9, S9.1, S9.2, S9.4, S10, S11, S12, S13, S14, S15, S16 |
| ADD-A112 | the confirming check's fixes: `input.kde_scheme.groups` and `unset_keys` (`InputBases.kde_groups`, `kde_unset_keys`), a scheme file's missing groups and keys as KColorScheme and `applyScheme()` read them, `Header` without a `Header` group as `Window`'s results, `PortError::MissingKey` removed; a transparent accent as no accent; `kde_accent`'s `set` one of KDE's seven; `kde_effects`' starting background; the facts gate's accent run; another platform's accent derived on `kde-breeze`; the accent layer per scheme under a palette with a port and a mapping; the KDE reader's fallback to kdeglobals (`ReaderStatus::Fallback`); the reader's scheme-file tokens as a `Tokens` rule under its scheme; the query returning `Sides`, alignments, placement, `EdgeSide` and translations physical for right-to-left, the transform origin physical; the exact checks evaluating non-applying rules, the model's implications among atoms; `widget.<part>` refused on a part that can be absent or drawn as another; `text_scale` refused under `DynamicType`; `ScrollDecay` as a closed enum of models with their own parameters; `LayerError::UnknownLayer` | S0, S1, S2.2, S2.4, S2.6, S4.2, S4.8, S8, S9, S9.1, S9.2, S9.5, S11, S12, S13, S14, S16, S17, S19.1 |
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
