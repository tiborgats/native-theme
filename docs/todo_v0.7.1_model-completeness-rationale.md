# v0.7.1 — Model completeness: Rationale

**Status:** design, drafted 2026-10-28 from the gap map; checked by the author, twice
independently, and revised after each check (§5); awaiting the maintainer's review. Input:
[`todo_v0.7.1_model-completeness-gap-map.md`](todo_v0.7.1_model-completeness-gap-map.md) and
its six platform files ("the map"; rows are cited by ID, unique within their file). Next: the
specification, `todo_v0.7.1_model-completeness-spec.md`, which states every type, field and
rule decided here and maps every row of the map to them (R24).

The maintainer's rule (2026-10-27): native-theme covers **everything a platform draws
visibly**; a value no document states is measured on the platform; no gap is closed by a
toolkit's default. The map lists 2,292 rows of what the six platforms draw that the model
cannot express. This document decides how the model expresses them: the model, its presets
and its readers. The connectors move to the new model in the same release (R21); their own
milestones (ROADMAP.md, *Later — the connectors*) follow. Values are collected after the
specification, because what to measure depends on the shape decided here (the map's §3).

Each decision gives the options, their arguments for and against, and the choice with its
reason. §1 states the model as it is; §2 the principles; §3 the decisions: R1 scope; R2–R4
the model's shape (blocks, rules, the order of everything, and the facts gate that keeps the
unstated honest), which every later decision builds on; R5–R18 one group of the map's
capabilities each; R19–R24 palettes, value kinds, release, documents, connectors and
traceability. §4 lists what the specification decides; §5 is the check record.

## 1. The model today

Read at caeddd6b:

- **A widget is a flat struct of `Option` fields** (`native-theme/src/model/widgets/mod.rs`):
  `ButtonTheme` has `background_color`, `hover_background`, `active_background`,
  `disabled_background`, `checked_background`, `hover_text_color`, … each its own field, with
  a nested `font: Option<FontSpec>` and `border: Option<WidgetBorderSpec>`.
- **`#[derive(ThemeWidget)]`** (`native-theme-derive`) generates each struct's resolved twin
  (no `Option`), its merge, emptiness test, validation, range checks and inheritance rules. A
  field is `option` (resolution fails while it is unset) or `soft_option` (it may stay
  `None`). A `soft_option` `None` means one of several things today: "draw another value
  instead" (`button.checked_background`: the pressed pair), "sized by content", "the platform
  draws none", or "no published number".
- **Resolution** (`native-theme/src/resolve/mod.rs`) fills unset fields from defaults and
  related widgets ("~91 inheritance rules" by the module's own count, documented in
  `docs/inheritance-rules.toml`, whose `[wrong_safety_nets]` lists rules known to give wrong
  values), fills the run-time-detected fields, and validates into `ResolvedTheme`.
- **Presets** are TOML (`native-theme/src/presets/`), a full light and a full dark `ThemeMode`
  each: geometry and fonts are stated twice. A reader returns a partial `ThemeMode` merged over
  its platform preset (chosen by OS version, `pipeline::preset_for_version`); an application
  can merge its own (`SystemTheme::with_overlay`).
- **Community presets** (Catppuccin, Dracula, Nord, …; 497 lines each) state every field
  themselves, geometry and an icon set (`lucide`) included.
- **Colours** are `Rgba`: four 8-bit sRGB channels; the macOS reader converts system colours to
  sRGB (`macos.rs`), clipping what lies outside it. **Borders** carry a colour, a width, a corner
  radius (on `defaults` also a large radius and an opacity), padding per side and
  `shadow_enabled`; `defaults` carries one `shadow_color`. **Fonts** carry family, size, weight,
  style and colour; line heights are on `defaults` and the text-scale entries.
- **Motion** is `dialog.entrance` and the animated icons (`AnimatedIcon`: frame lists and
  transforms, which carry the spinners).
- **Accessibility** is four run-time flags on `AccessibilityPreferences`
  (`text_scaling_factor`, `reduce_motion`, `high_contrast`, `reduce_transparency`), with no
  theme values behind them.
- **The `Option` structs are not `#[non_exhaustive]`** (docs/todo.md, *Dialog: the platform's
  vertical placement*), so a new field breaks struct literals; the resolved structs and some
  enums (`IconRole`, `IconSet`) are.

## 2. Principles

1. **What an application draws, the model carries.** A visible property of something an
   application or its toolkit draws, or configures for the system to draw, has a place in the
   model.
2. **One mechanism per job.** The map groups its rows by mechanism (C01–C28); each mechanism is
   designed once and used by every widget that needs it, and two mechanisms never do one job.
3. **The model invents no value.** A value is a platform's: read, published or measured. Where a
   platform derives one value from another, the model carries the derivation, not its result
   over one background. Every value a preset resolves to is checked against the platform's
   facts (R4).
4. **Typed where a portable application needs it, keyed where only one platform has it.** Both
   forms are built from the same blocks, declared, and validated the same way.

## 3. Decisions

### R1 — Scope

Each scope question is settled by the model's purpose: to let an application's toolkit draw
what the application shows.

**R1.1 — What the system draws.**

- *Include everything the system draws.* Against: what only the system ever draws (the
  on-screen keyboard, a notification banner) has no consumer, and the system draws it natively.
- *Exclude everything another process or a system library draws.* Against: a toolkit that
  draws its own window frame draws its own window menu (GTK's fallback, GN-P3-3†); an Android
  application configures its system bars and its splash screen (MAT-W1, MAT-P3-6†).
- *In: what the application or its toolkit draws; what a toolkit draws in place of the
  system's when it draws its own chrome or its own equivalent (an in-window toast, its own file
  list); what the application configures for the system to draw with; and what the system
  leaves to the toolkit to act on (hiding the pointer while typing, the focus-visible rule).
  Out: what only the system ever draws or applies, whatever the toolkit does.*

**Decision:** the third. In: the window frame, title bar, window menu and shadow wherever a
toolkit can draw them; an Android application's system-bar and splash values; the IME pre-edit
text; Win32's global effect switches, which a toolkit honours (W3-53†). Out: the on-screen
keyboard, notification banners, home-screen widgets, Live Activities, Control Center, a
shell's tiling preview and Snap layouts, an input method's own candidate window, system
dialogs a toolkit only invokes (the print dialog, GNOME's portal file dialog drawn by Files),
and what the windowing system applies to every toolkit without its involvement (display scale
and safe-area insets as the windowing library reports them, whole-screen colour filters and
Smart Invert; K17, K23, K27, X-A11-8). Each of the map's rows and Appendix B's kinds gets its
verdict by this test in the specification's traceability (R24).

**R1.2 — Libraries beside the toolkit.** Test: maintained by the platform's own project
(GNOME, KDE, Microsoft, Apple, Google's Material team) and released as stable.

- *The toolkit only.* Against: GNOME Text Editor's and Kate's code views, KDE's form cards and
  Windows' settings cards are the platform's look to its users.
- *Every library any platform application uses.* Against: a third-party library carries its
  vendor's look (KDDockWidgets draws KDAB's own drop arrows; the seed follow-up, G19).

**Decision:** the test. In: GtkSourceView, libpanel, kirigami-addons, KTextEditor,
KSyntaxHighlighting, KQuickCharts, the Windows Community Toolkit, Swift Charts. Out: the
Toolkit Labs (experimental; its `MarkdownTextBlock` rows wait for a Toolkit release) and
KDDockWidgets. kirigami-addons has no rows yet: its survey is the specification's first step.

**R1.3 — Other toolkits on a platform** (GTK 3 applications on GNOME; WPF and Win32 controls on
Windows). A platform preset describes the platform's current native toolkit; the others are
candidate presets of their own (ROADMAP.md, *Beyond v0.7*). One preset holding two toolkits'
looks for one widget would make the application choose which to copy, which is what choosing
a preset already does. Rows: GN-P3-17†, W-P3-15†.

**R1.4 — SwiftUI and Mac Catalyst.** AppKit and UIKit are the reference where SwiftUI draws a
control through them; a control only SwiftUI has (`Gauge`, the grouped `Form`) is a control of
the platform (R14). Mac Catalyst applications draw the iOS look; its Catalyst-only rows are out.

**R1.5 — Versions inside a generation, and choosing the generation.** A preset describes the
newest release of its generation (macOS 15 for `macos-monterey`, libadwaita 1.10 for `adwaita`,
iOS 18 for `ios-13`); readers read what the running release states; a release whose look
differs as a whole starts a new generation and preset, as macOS 26 did. A per-release overlay
inside a preset was rejected: it multiplies values by releases for differences the readers
already read where they can. The generation is chosen by the running OS version, as
`pipeline::preset_for_version` does now, and by the application's own opt-out where a platform
has one (iOS's `UIDesignRequiresCompatibility`, readable from the application's Info.plist,
X-MT-13).

**R1.6 — Two current looks on one platform.** KDE draws Qt Widgets and Kirigami applications
differently in places (the focus indicator, K3.20 against Q3.30); Material 3 has its baseline
and its Expressive look.

- *Two presets each.* Against: they share most values, stated twice.
- *Pick one.* Against: the other current, widely used look is left out.
- *One preset with a flavour,* a condition the application chooses (R2), the default stated.

**Decision:** flavours. `kde-breeze`: `widgets` (default; most KDE applications) and `kirigami`;
`material-3`: `baseline` (default) and `expressive`. R1.2's stability test passes for Expressive
as a design: its tokens are published in Material's own token set (`latest` in material-web),
while Compose's implementation is marked experimental; the values come from the tokens, cited
at a pinned material-web commit, never at `latest`.

### R2 — The model's shape: typed blocks and one rule mechanism

The map needs values per part; per state and combination of states; per condition and
combination of conditions; per container (libadwaita's icon buttons are flat inside a header
bar unless raised, suggested, destructive or opaque, GN-1.151; Kirigami's colour set follows
modality, Q1.53); per variant, where variants stack (libadwaita's style classes); per control
size; plus shadows, motion and materials, which today have no type.

- **A. More flat fields on today's structs.** For: no new concepts. Against: every combination
  is a field; one button body (fill, text colour, stroke, shadow, icon colour, corners,
  opacity) times nine state combinations is some 60 fields per part before conditions,
  containers and variants, most repeating another value.
- **B. A string-keyed style-rule engine** (CSS-like). For: compact; GTK's themes are built this
  way. Against: untyped, so the compiler checks nothing and a typo is a silent miss.
- **C. Typed blocks and parts with separate mechanisms** for states, conditions, variants,
  sizes and palettes. Against: several mechanisms merging partial looks with no order between
  them; none handles containers or combined conditions.
- **D. Typed blocks and parts, and one typed rule mechanism.** A rule is a typed selector and a
  typed patch: *when* these conditions hold, inside this container, with these variants, at
  this size, in these states, *set* these values of this part (or of the palette). For: typed
  throughout; one mechanism carries states, combinations, conditions, containers, variants and
  sizes; it mirrors how platforms state their looks (GTK's CSS, WinUI's visual states and theme
  dictionaries; Breeze's C++ branches on the same inputs). Against: every widget is rewritten,
  and the derive macro must generate typed patches.

**Decision: D.** Pre-1.0 the project promises no compatibility; D is the only option that is
both typed and one mechanism.

**R2.1 — The blocks** (fields are the specification's):

| Block | Carries | Capabilities | Decision |
|---|---|---|---|
| colour | an extended-sRGB colour, or a colour derivation | C04, C05, C28 | R5 |
| length | logical pixels, automatic, or a derivation from the font or line height | C26 | R5.3 |
| paint | a stack of layers, bottom first: colour, gradient or material, each with a blend mode | C03, C08, C27 | R6 |
| corners | a radius per corner, fully rounded, concentric; the corner curve | C11 | R7 |
| stroke | a width and paint per side; inside, centred or outside | C12 | R7 |
| shadow | layers (offset, blur, spread, colour, inset), or a reference to the platform's elevation ramp | C01, C02 | R8 |
| transform | scale, rotation and translation about an origin | C13, C14 | R9 |
| transition | what changes, duration, delay, timing (curve, steps, spring) | C14 | R9 |
| text style | font (family, size, weight, style, width, axes, features, fallbacks), letter spacing, line height, decoration, elision and wrapping, colour | C18 | R10 |
| icon style, glyph | an icon's size, colour, rendering mode, weight, effect; a glyph's source and geometry | C19, C13 | R11 |

A **part style** combines them: fill, stroke, corners, shadow, opacity, transform, text style,
icon style or glyph, padding per side, size limits, cursor (R17), the surface its descendants
draw on (R5.1), and its property transitions (R9). A **widget** is a typed set of named parts
plus widget-level values (gaps, placement, behaviour). A **preset** is its declarations (R2.4),
the palette's bases per colour scheme (R5.1), the tokens (motion, elevation ramp, text ramp,
behaviour), each part's base, and an ordered list of rules. Today's `defaults` becomes the
palette and the tokens.

**R2.2 — Rules.** A rule's patch sets values of one target: a widget's part, or the palette.
Its selector has these parts, each optional, each one a set that may also be negated (*not*
these), and the rule matches when all its parts hold:

- *conditions,* window- or theme-wide: colour scheme (light, dark); window inactive; the macOS
  key and main window and active application (MAC-X16); window maximised, tiled or full screen;
  increased contrast; high contrast; reduced transparency; reduced motion; bold text; button
  shapes; differentiate without colour; text direction; window size class; flavour (R1.6).
  Material's contrast levels (MAT-X18) map to them: standard is neither, medium increased, high
  high contrast. The four `AccessibilityPreferences` flags become conditions;
  `text_scaling_factor` stays a factor (R10).
- *context,* the element's containers: by default any ancestor of the named kind; or the
  nearest ancestor among a stated set of kinds, so that libadwaita's "flat in a header bar, but
  raised again inside a popover opened from it" is "nearest of {header bar, popover} is a
  header bar". Each ancestor is matched by widget, part, variants and states.
- *variants,* a set the element must have, and a set it must not (libadwaita's
  `:not(.raised):not(.destructive-action)`, GN-1.151);
- *size,* the element's control size (R15);
- *states,* a set of state flags (R3), and a set that must be absent.

A palette rule may use conditions only: the palette is resolved per window (R2.3). Values that
vary per element are in part rules, which may refer to palette roles (R5.1).

**R2.3 — Order, layers, resolution and query.** Rules apply in stated order, each over what the
earlier ones gave; there is no specificity arithmetic, because the platforms' own orders
differ, and an explicit order is the one every author can read. The sources come in two passes:

1. **Everything but accessibility:** the preset's bases, the reader's bases (R18), a palette's
   bases (R19), the application's bases; then the preset's rules, the reader's rules, the
   palette's rules, the application's rules.
2. **Accessibility:** every rule whose conditions include an accessibility condition (contrast,
   high contrast, reduced transparency, reduced motion, bold text, button shapes, differentiate
   without colour), from the preset, the reader, the palette and the application, in that order.

So a user's accessibility setting wins over any decoration a palette or an application states,
as on the platforms themselves, and an application can still state its own high-contrast look
(as WinUI lets it), because its accessibility rules come last.

The palette is resolved per window: its bases for the colour scheme, then its rules whose
conditions hold, in order, evaluated (R5.3). A **query** asks for one part's style for one
element: its input is the element's widget, part, variants, size and states and its ancestor
chain with each ancestor's widget, part, variants and states (iced and egui hold no such tree,
so their connectors build one, R23). The query applies, over the part's base, the part's rules
whose conditions hold, in stated order, skipping those whose context, variants, size or states
do not match, and evaluates derivations last, because a derivation can refer to the value
beneath it in this very query (R5.3). The resolved theme holds the palette and, per part, its
base and its condition-filtered rule list; applying condition-only part rules early instead
would put them before element rules stated ahead of them.

**R2.4 — Declarations.** A preset declares, per widget, the variants and sizes it has, its
keyed components and their parts (R14), its keyed palette roles, text styles and elevation ramp
entries, and its exclusive state groups (R3). Resolution checks every rule and reference against
the declarations, so a typo is an error, not a silent miss; a query for a variant or size the
widget does not declare returns "not available" (R15).

**R2.5 — Validation and infallible queries.** The project forbids runtime panics, and a query
runs inside a connector's draw call. Resolution therefore checks every rule's derivations
against every base and rule they can apply over: types, references, cycles, and operands that
could be absent. A preset that fails is an error at resolution; a query cannot fail.

### R3 — State flags

The flags, the union of what the map's platforms draw: hover, pressed, focused (has focus),
focus visible (focus shown), disabled, checked (on), selected, indeterminate, open (expanded,
menu shown), default (a dialog's default button), dragged, drop target, read-only, invalid,
visited, autofilled, scrolled (content under a bar: MAT-TA3, T-10), and per widget the flags
only it has (a slider's overlapping handles, MAT-S10). Checked and selected are separate because
a Qt list item carries both independently (`checkState` and `State_Selected`); focused and focus
visible because macOS draws an unemphasised selection without focus (MAC-I05, MAC-SB05) while
most platforms show focus only after keyboard navigation (K3.20); when focus becomes visible is
a platform behaviour in the behaviour record (R17). Window activity and window states are
conditions (R2.2), as they hold for the whole window.

**Exclusive groups.** A toolkit may report combinations a platform never draws: egui reports
hover on a disabled widget, while WinUI's `CommonStates` and GTK's insensitive widgets never
combine them. A preset declares its platform's exclusive groups (disabled excludes hover and
pressed, where the platform says so), and the query drops the excluded flags before matching,
so no look is made up for a combination the platform does not have.

**State layers** (Material's hover, focus, pressed and dragged overlays, C08; libadwaita's
`currentColor` shares) are a paint layer a rule appends over the fill beneath (R6), with a colour
derivation (R5.3). Material's ripple is a transition (R9).

### R4 — The facts gate

With rules in stated order, a combination no rule states gets what the earlier rules give: the
platform's own answer where the platform composes the same way, and not necessarily elsewhere.
libadwaita's flat button is 7 % on hover, 10 % checked, 13 % checked and hovered (GN-1.29), which
appending the first two (16.3 %) does not give, so the preset must state {checked, hover}.

- *Trust the order.* Against: a forgotten combination passes silently.
- *State every combination of every flag.* Against: thousands of repeats.
- *A gate that resolves every value a platform draws and compares it with its fact.*

**Decision:** the gate, built so that it runs mechanically:

- **Facts as data** (R22): each fact is a value (or the platform's own derivation) for a key:
  the model path and the selector it holds under (scheme, conditions, context, variants, size,
  states), with its source kind and citation.
- **Which combinations:** per widget, from the platform's own state model: WinUI's visual
  states, libadwaita's selectors, Breeze's state branches, Material's tokens, and on macOS and
  iOS the API's control states (`NSControl` and `UIControl.State`, the appearance names) times
  the conditions, each measured by the probe. The map is a survey; the platforms' state models
  are the list.
- **Derivations:** where a platform states a derivation, the fact is the derivation. The gate
  resolves each preset twice, with its own palette and with a perturbed one (another accent, text
  and background), and compares both with the facts evaluated the same way, so a literal stated
  where the platform derives fails.

The gate also replaces trust in inheritance: there is no model inheritance left (R20).

### R5 — Colours (C04, C05, C08, C28)

**R5.1 — The palette and surfaces.** Today's `defaults` has 23 colours in a flat list. KDE's
scheme has seven sets of the same roles (View, Window, Button, Selection, Tooltip, Complementary,
Header), each in Active, Inactive and Disabled groups, the latter two computed by its colour
effects (K3.50–K3.55); libadwaita names its colours per surface (window, view, header bar,
sidebar, card, dialog, popover); Material has a ladder of surface containers and on-colours
(MAT-X1), WinUI a ladder of fill and stroke tiers.

- *A longer flat list.* Against: loses which roles belong together; cannot hold KDE's sets.
- *Surface sets with one role record for every platform.* Against: Material's and WinUI's
  ladders are not surfaces.
- *Typed surface sets with a common role record, global accent and status roles, and a keyed
  set of the platform's own roles* (Material's ladder, WinUI's tiers).

**Decision:** the third (principle 4). The window being inactive holds for a whole window,
so an inactive group is a palette rule under the inactive condition, restating the roles that
change; being disabled is per element, so each surface set also holds disabled roles, which
parts' {disabled} rules refer to. On KDE both are derivations by KDE's colour effects (R5.3)
with the effect parameters the reader reads, so a user's own scheme gets KDE's own effects.

The palette has a base per colour scheme (light, dark) because exactly one scheme always holds
and the two differ in most roles; every other condition is optional and combines with others,
so it is a palette rule, even where it restates every role: Windows' high contrast replaces the
light and dark colours alike (one rule, filled from the reader's eight system colours, W3-35),
and Material's medium and high contrast schemes (MAT-X18) are rules under the scheme and the
contrast condition together. **Surfaces are inherited:** a part style may set the surface its descendants
draw on (a header bar sets the header set; Kirigami's `Theme.colorSet`, Q1.53, Q1.96), and a role
reference written relative to the surface ("the surface's foreground") resolves against the
nearest ancestor that set one. GTK's inheritance of `color` is the same thing. Without it every
widget would restate its role references per container.

**R5.2 — Translucent colours.** KDE, libadwaita and WinUI state many fills as translucent colours
composited over what lies beneath (GN-3.08, W3-45†); today's presets flatten them over one
background, which is wrong on every other surface. **Decision:** a preset states the translucent
colour the platform states, and the toolkit composites it, as the platform does.

**R5.3 — Derivations.** A value that follows another value (the accent the user picks, a
high-contrast colour, the current foreground, the font size) cannot be stated as a result.

- *Derive in each reader.* Against: without a reader, under a palette, or after a rule changes
  the accent, every derived value would be stale.
- *Derivation expressions in the model.*

**Decision:** expressions. Their operands: palette roles (typed, keyed, or relative to the
current surface), `base` (the value the earlier rules gave in this query), `foreground` (the
element's current foreground: the text colour its text parts get, inherited from its surface
where the part states none; libadwaita's `currentColor`), and for lengths the part's font size
and line height. Their functions are a closed set, each with one definition and its source:
`alpha(c, a)` multiplies the alpha (GN-1.210 multiplies an alpha already below one); `mix(a, b,
t)` interpolates in sRGB, as CSS `color-mix(in srgb …)` for libadwaita; KDE's colour effects and
the `KColorUtils` functions they use, to KDE's code, on the RGB channels with alpha kept (their
behaviour on translucent input is checked against KDE's code in the specification); Material's
scheme generation from a seed colour (material-color-utilities, which defines Material's dynamic
colour, MAT-X17); and for lengths, multiples of the font size or line height (Kirigami's grid
unit, Q1.62; a width in characters, GN-1.283†). A platform whose arithmetic differs gets its own
function; a function is added only when a platform's rule needs it.

Composition over the value beneath is not a colour function: it is a paint layer appended over
`base` (R6), so one mechanism does it. Evaluation follows dependencies; a cycle is an error at
resolution (R2.5).

**WinUI's accent ramp** (AccentLight1–3, AccentDark1–3) comes from an algorithm Microsoft does
not publish; the reader reads it from `UISettings`. A palette or an application that changes the
accent on `windows-11` must therefore state the ramp too; the API refuses an accent alone there.

**R5.4 — Representation (C28).** macOS and iOS define system colours beyond sRGB (Display P3;
MAC-X10, D-17); `Rgba` and the macOS reader's conversion to sRGB clip them. HDR appears only in
the colour well's content (MAC-X10), the application's data.

- *Keep 8-bit sRGB.* Against: P3 colours clip.
- *8-bit channels and a colour-space tag.* Against: derivations would need a rule for mixed
  spaces.
- *Floating-point extended sRGB* (sRGB primaries and transfer, channels beyond 0–1), which
  AppKit and UIKit hand out when asked for it. For: one space and one arithmetic; P3 kept
  exactly; a connector converts or clamps for a toolkit that renders sRGB.

**Decision:** floating-point extended sRGB; the readers ask for it. HDR is not modelled: no theme
value of the six platforms uses it.

### R6 — Paint: layers, gradients, materials, blending (C03, C08, C27)

A paint is a stack of layers, bottom first: a colour (R5), a linear or radial gradient (colour
stops with positions; macOS 12–15's checkbox box, MAC-X09, bezels to verify, MAC-B05; WinUI's
elevation strokes, W3-04), or a material, each with a blend mode (macOS's vibrant foregrounds
blend with what is beneath, MAC-X03). A rule sets a part's paint or appends layers over `base`:
libadwaita's stacked fills (GN-1.30, GN-3.06) and every state layer are appended layers.

**Materials** (MAC-X02, MAC-X03; Liquid Glass, MAC-X04–MAC-X06, X-MT-5–X-MT-13; Mica, Mica Alt,
Acrylic, W3-06–W3-10; KWin's blur, K3.11):

- *The opaque fallback only* (today). Against: the look is the translucency.
- *A named material only.* For: exact where a toolkit can ask the system. Against: inside a window
  most toolkits cannot.
- *The platform's identity of the material, the parameters the platform states or a measurement
  gives (blur radius, tint colour and opacity, saturation, luminosity, noise), and the opaque
  colour the platform falls back to.*

**Decision:** the third. A parameter the platform has no counterpart for is absent (R20); what
parameters cannot describe (Liquid Glass's refraction) is carried by the identity, and a toolkit
that cannot use the system's material records that as its own limit (R23).

### R7 — Corners and stroke (C11, C12)

**Corners:** a radius per corner; fully rounded (a capsule at any height: Material's `full`,
iOS 26's capsules); concentric (the container's radius less the inset, with a minimum where the
platform states one, X-CR-1–X-CR-3), which the toolkit computes at layout from its container;
and the corner curve (circular, or Apple's continuous). A shape that changes between states is a
rule with its transition.

**Stroke:** a width and paint per side, inside, centred or outside the edge: WinUI's gradient
elevation strokes and the thicker accent bottom edge of a focused text box (W1-026); libadwaita's
outlines outside the box. A second outline (a rim, a bevel's highlight) is an inset shadow layer
(R8), not a second stroke.

### R8 — Shadows and elevation (C01, C02)

**Shadow:** a list of layers (x and y offset, blur, spread, colour, inset), or a reference to an
entry of the platform's elevation ramp. GNOME draws three layers for a window (GN-1.01); Material
a key and an ambient layer (MAT-X2–MAT-X4); Windows only the ambient layer of Fluent's two
(W3-01); Breeze two, with strength read from `breezerc` (K3.1).

- *Layers per part only.* Against: Windows, Material and KDE share shadows by elevation level.
- *Typed portable roles in the tokens* (card, menu, dialog). Against: a role cannot say that a
  WinUI control drops from elevation 2 to 1 when pressed (W3-02), or that a Material button rises
  a level on hover (MAT-X4), without restating shadows.
- *Both forms: a platform with a ramp (WinUI's 1–128, Material's levels 0–5, Breeze's sizes)
  states it once, keyed by its own names, and parts refer to entries, a rule changing the entry;
  a platform without one (GNOME, macOS, iOS) states layers per part.*

**Decision:** both forms. `shadow_color` and `shadow_enabled` go: a shadow's colour is in its
layers, no layers is no shadow, and the inactive window's shadow is a rule. Material's surface
tint by elevation is a paint layer in the same rules.

### R9 — Motion and transforms (C14, C08's ripple)

The platforms animate nearly every control (MAC-X11, W3-30–W3-34, K3.30–K3.37, GN-3.18–GN-3.25,
MAT-X13–MAT-X15, X-MO-1).

- **Transform:** scale, rotation and translation about an origin, a part-style property that rules
  set and transitions animate: iPadOS's pointer lift (X-PH-1, B-17), macOS's disclosure triangle
  turning (MAC-X11), WinUI's pressed knob stretching (W1-120) and its chevron (W1-132).
- **Timing:** a cubic Bézier, linear, steps, or a spring stated physically (mass, stiffness,
  damping; with the initial velocity, rest threshold and clamping libadwaita's
  `AdwSpringAnimation` states), as libadwaita and Material state springs; Apple's response and
  damping fraction convert to it exactly; velocity-dependent curves (Kirigami's, Q3.26) are a
  timing kind of their own.
- **Property transitions:** a part states how its properties move when a rule starts or stops
  applying, per property group, with duration, delay and timing; a rule may state its own. When
  several rules start or stop at once, the transition of the rule being entered governs; on
  leaving, the leaving rule's. This is how CSS, QML behaviours and XAML visual-state transitions
  work.
- **Presentation transitions:** an element appearing, disappearing or moving (a popover opening, a
  dialog's entrance, a page pushed, a sheet presented): fade, slide from an edge, scale, shape
  morph, ripple, keyed by event per widget. `dialog.entrance` becomes one of them.
- **Loops:** a period, a timing and the keyframes the platform states, extending `AnimatedIcon`
  (spinners, indeterminate progress).
- **Scroll physics:** deceleration, overscroll and its spring, snapping, distance per wheel step
  (GN-3.24, W3-52†).
- **Tokens and factor:** the platform's published durations and curves, referred to by parts; a
  global speed factor (KDE's `AnimationDurationFactor`, K3.30) read by the reader.
- **Reduced motion:** rules under the reduced-motion condition stating what the platform does
  instead (libadwaita swaps slides for crossfades, GN-3.25), not a flag that zeroes durations.

### R10 — Typography (C18)

**Text style:** the font gains width (stretch), variable-font axes (`opsz`, `wdth`, …; an axis may
follow the size automatically, as SF's optical size and Segoe UI Variable's do, W3-23), OpenType
features, a fallback family list, an emoji family and a per-language fallback map (MAC-FM01†,
W-FM01†, K-FM01†, GN-FM01†, MAT-FM01†, FM-1†); the style gains letter spacing (tracking, which SF
Pro states per size, D-1), its own line height per text role and style (today only `defaults` and
the four text-scale entries have one), **decoration** (a line's kind, thickness, offset and colour:
the spell-check and IME underlines, mnemonic underlines, a link's underline; K1.49†, W1-151†,
GN-1.281†), and **elision and wrapping** (where a platform truncates, at the end or the middle, or
wraps).

**Text roles:** today four `text_scale` entries besides the body font; macOS and iOS have eleven
styles, Material fifteen and fifteen emphasized (MAT-TS4), WinUI eight, libadwaita about nine.

- *Each platform's own names only.* Against: no portable "heading".
- *A fixed semantic ramp only.* Against: either huge or merging styles.
- *A typed semantic ramp (each preset filling every slot, its facts saying which of its styles
  fills it) plus a keyed set of the platform's own styles.*

**Decision:** the third (principle 4). The text-scaling factor stays for platforms that scale
linearly; Dynamic Type and Android's non-linear font scaling (MAT-X19) are read per style by those
platforms' readers (R18), and a preset states the default size category's values.

### R11 — Icons and glyphs (C19, C13)

**Icon style:** size (the existing `defaults.icon_sizes` contexts become its tokens), colour (the
recolouring of symbolic icons), rendering mode (SF Symbols' monochrome, hierarchical, palette,
multicolour), weight and scale, and a per-state effect where the platform changes an icon by state
(KDE's disabled and active effects, K3.60). Mirroring in right-to-left layouts is a property of each
icon role (MAT-FM04†). Symbol effects are transitions.

**Glyph:** a check mark, a radio dot, a disclosure arrow, a combo chevron: glyphs no icon set has
(`IconRole` has none of them). A glyph is the platform's own: a code point of its symbol font
(Segoe Fluent Icons), an icon of its icon theme by name, or a vector path taken from its source code
(Breeze draws its check mark with `QPainterPath`), with size, stroke width and colour. Following the
project's rule on assets, a path is only ever the platform's own geometry.

### R12 — Focus, caret, selection and highlights (C15, C16, C17)

**Focus indicator:** a kind (ring, filled ring, underline, none), stroke, offset, corners (concentric
with its part unless stated), shadow and transition, shown in the focus-visible state; a part
overrides the platform's (KDE's focus line under tab labels, K3.21). A focus that only changes a fill
is a {focus visible} rule, not a kind. Today's three `focus_ring_*` fields become this; when focus
becomes visible is in the behaviour record (R17).

**Caret:** colour, width, shape (bar, block in overwrite mode, K1.47†), the secondary caret in
mixed-direction text (GN-SF03†), blink on and off times and timeout, read where the platform has a
setting (GNOME, GN-3.32–GN-3.38; Windows; KDE; macOS's blink period, MAC-X14), and when it hides
(while text is selected, GN-3.73†).

**Selection and highlights:** palette roles per surface (R5.1); the find highlight and the
drop-target highlight are roles and parts of the widgets that draw them.

### R13 — Window chrome (C20)

The window becomes parts: the frame (paint, corners, outline, shadow), the title bar (heights per
variant, padding, title alignment and text style), the caption buttons (one part type, each button
with its glyph and its own rules, such as the close button's red hover), the window menu, and the
resize zone. Maximised, tiled, full-screen and inactive windows are conditions, so the corners and
shadow a tiled window drops are rules. The caption buttons' order and side are read where the
platform has a setting (GNOME's `button-layout`, KDE's `kwinrc`).

### R14 — Parts and controls (C13, C24)

A widget gains the parts the map names (a field's helper text and counter, a tab's close button, a
slider's ticks, a sheet's grabber, a selection pill).

Controls the model lacks: the map's §4.2 lists 118 kinds. A kind is portable when two or more
platforms draw it, counted with the cross-check decisions that found it drawn inside an element the
model has (every platform's list draws a tree, though only Windows lists a tree view as a control).

- *A typed struct per kind.* Against: structs each used by one preset.
- *Typed for kinds two or more platforms draw (a struct of its own, or parts of the element it is
  drawn in); keyed components for the rest,* named components of named parts from the same blocks,
  declared per preset (R2.4).
- *Only the controls the three toolkits have.* Against: the model would follow toolkits.

**Decision:** the second (principle 4). A keyed component that later appears on a second platform
becomes typed (breaking, allowed before 1.0). Kinds out of scope by R1.1 or R1.2 are not modelled.

### R15 — Variants and control sizes (C09, C10)

Both are selector parts (R2.2), declared per widget (R2.4).

**Variants** (flat, outlined, filled, tonal, destructive, suggested, pill, circular, AppKit's bezel
styles, iOS's button configurations, glass): an element carries a set of variants; rules match sets
and their negations, so libadwaita's stacked classes are rules for {flat, circular}, and its
"flat unless destructive" a rule with a negated set. Names are typed where two or more platforms
share the meaning (taken from the map's C09 rows) and keyed otherwise.

**Control sizes** (macOS's mini to extra large, MAC-X17; Material's density; WinUI's compact sizing,
W3-39): an element's size is inherited from its ancestors unless it states its own, as macOS's
`controlSize`, Material's density and WinUI's compact resource scope all propagate; the query
receives the effective size. An application-wide density is the size at the root. A size or variant
a widget does not declare returns "not available" from the query (R2.4): an application asking for a
small button on `adwaita` learns there is none, rather than getting an invented one.

### R16 — Layout and placement (C26)

Geometry is in part styles (padding per side, size limits, gaps) and widget values (alignment, the
order of dialog buttons, a content clamp, GN-1.195). **Placement** (where a menu, popover, tooltip or
dialog opens against its anchor or window, its margins, flipping and sliding) is parameters plus a
named strategy from a closed set, each implemented once, in native-theme, as a pure function of the
anchor, the element's size and the window (no toolkit types), to its platform's source and cited
(GTK's tooltip placement, GN-1.83; the dialog rows of the seed follow-up, G24). Code cannot be data,
and writing each strategy once beats writing it in three connectors. **Breakpoints:** a platform's
window size classes are a condition; a breakpoint one widget owns is its value (AdwDialog becomes a
bottom sheet at a width of 450 or a height of 360 or less). **Right-to-left:** the toolkit mirrors the
layout; the model states what the platform does not mirror or does differently (Material keeps
time-based content left to right and mirrors question-mark icons in Arabic and Farsi but not Hebrew,
MAT-FM03†; libadwaita's `:dir()` rules, GN-FM03†), as icon-role properties (R11) and rules under the
text-direction condition.

### R17 — Timings, behaviours and the pointer (C21, C22)

Timings and behaviours that apply everywhere form a behaviour record among the tokens: double-click
time and distance, drag threshold, hover-open delay, key repeat, scroll distance per step, when focus
becomes visible (Breeze after keyboard focus, K3.20; macOS text fields on any focus), and whether the
pointer hides while typing. Those that belong to one widget stay with it: the tooltip's show, hide
and reshow beside today's `tooltip.show_delay_ms`; the menu's submenu delay and sloppy-close timeout
(K-RV2†); the scroll bar's auto-hide and what a click in its trough does (K-RV1†, MAC-P3-2†); a toast's
duration. Each is read where the platform has a setting and stated otherwise. The pointer: the cursor
shape per part, the cursor theme and size (read), and iPadOS's pointer effects (X-PH-1), which are
hover rules with a transform.

### R18 — Readers (C25)

Every value the map marks readable is read by the platform's reader where native-theme has one; the
preset states the default the reader replaces; the specification lists the new reads, and today's
wrong reads (KDE's title bar from `[WM]` instead of `[Colors:Header]`, GNOME's dropped font list;
docs/todo.md) are fixed with them. **A reader states each value under the conditions it was read in:**
a value read under the base conditions goes into the reader's bases, one read under a condition (the
macOS appearance with increased contrast, Windows' system colours under high contrast) into a reader
rule under that condition, so no preset rule applies on top of it twice.

iOS and Android have no reader, and native-theme has no target there; an application drawn with iced
or egui can run on both through winit, and without a reader would ignore the user's text size,
contrast and colours (Android's dynamic colour, MAT-X17).

- *Readers for both in the same release.* Against: two new platforms to build and test, which is
  platform support, not model completeness, and would hold the model back.
- *Neither, ever.* Against: principle 1.
- *Both, as a platform-support milestone after the model release,* with the iOS icon loader (X-IC-7).
  For: the model already carries what they will read; the presets state the defaults meanwhile.

**Decision:** the third, the same for both; their R rows map to that milestone (R24).

### R19 — Palettes (community presets)

A community preset (Catppuccin, Dracula, Nord, …) states every field today. With this model it would
need shadows, motion, materials and states its project never defined, which principle 3 forbids.

- *State them anyway.* Against: invented values.
- *A fixed platform base per community preset.* Against: arbitrary.
- *A palette over a platform preset, mapped from the project's style guide.* Against: guides do not
  give one colour per role (Catppuccin's: "Overlay 2 20% - 30% Opacity" for the selection, "Crust,
  Mantle" for secondary panes), so mapping them is choosing values; and not every project has a guide.
- *A palette over a platform preset, taken from the project's own port for that platform* (its KDE
  colour scheme, its libadwaita colour overrides, …), converted mechanically. For: the project states
  the platform mapping itself; what the port leaves to the platform is what the project's look is on
  that platform. Against: a project without a port for a platform has no palette there.

**Decision:** the fourth. Palettes are a layer (R2.3), named "palettes" in the API and documents so
that "preset" keeps meaning a whole platform theme. A palette:

- is offered on a platform only where its project publishes a port for it; which ports exist is the
  specification's survey, and today's community presets keep only what a port states;
- replaces the reader's colour scheme: with a palette applied, the reader's colour values (its colour
  bases and colour rules) are not applied, so a user's own scheme cannot leak into it; the reader's
  other values and every accessibility rule still apply (R2.3's second pass);
- leaves to the platform what its port leaves (its derivations follow the palette's colours: a
  libadwaita hover is the foreground at an opacity, whatever the foreground is);
- states which colour schemes it has; applying it under a scheme it lacks is refused by the API (a
  dark-only palette does not invent a light one);
- carries no icon set unless its port does (today's `icon_set = "lucide"` goes).

### R20 — Value kinds, and no model inheritance

Today a `soft_option` `None` has four meanings and ~91 inheritance rules fill unset fields beside the
presets (§1).

- *Keep inheritance as base-layer default derivations.* Against: a second mechanism beside the rules,
  which also needs a place in the order; it is how `[wrong_safety_nets]` exist.
- *No model inheritance: a preset states every reference in its base as a derivation* (`= the surface's
  foreground`), and the gate checks it. For: one mechanism; the reference is visible where it applies.
  Against: longer presets; today's rules are converted once, mechanically, into the presets.

**Decision:** no model inheritance. A value is one of:

- *stated:* a value or a derivation (R5.3);
- *automatic,* where the platform sizes by content (stated, so a toolkit sizes by content because the
  platform does);
- *absent:* the platform does not draw the thing; it resolves to none and a connector draws nothing.
  A field may be absent only where the model marks it so;
- *missing:* none of these, an error at resolution.

"Draw another value instead" becomes stated values (where a platform draws a checked button with its
pressed fill, its {checked} rule states that fill); "no published number" is not a kind: it is
measured.

### R21 — Compatibility and release

The model's public API changes and the seven crates are released together, so the release is
**0.8.0**, carrying the connectors moved to the new model (each mapping what its toolkit can draw and
recording the rest, R23); the connectors' feature milestones stay later.

**`#[non_exhaustive]`:** every model struct gets it, so a later field is not breaking; code outside the
crate builds a struct from `default()` and field assignment (the connectors, showcases and examples
change accordingly). Enums the connectors must map in full (state flags, conditions, value kinds,
`IconRole`) do not get it, and `IconRole` loses it: a new variant then breaks the connectors' builds,
which before 1.0 is the check that they map it, rather than a wildcard arm that hides it.

- *Release in stages.* Against: the rules touch every widget at once, each stage would break the
  connectors again, and a release with some capabilities filled shows mixed looks.
- *One release.* Against: a long branch.

**Decision:** one release. v0.7.1 is this milestone's documents; the implementation runs on its own
branch (`v0.8.0-rc1`), staged by the specification's plan, and nothing is released until every
platform preset, `ios-26` included, is complete and passes the facts gate (R4).

### R22 — The documents that follow the model

- **Platform facts become data.** R4's gate must parse a value per key, and `docs/platform-facts.md`
  is prose. The facts move into per-platform, per-area data files (a key per model path and selector,
  the value or derivation, the source kind and the citation), and the readable tables are generated
  from them. `scripts/check_platform_facts_rows.py` and `scripts/check_preset_citations.py` move to
  the data. iOS gets its columns (13–18 and 26). Keeping the prose and splitting it was rejected: a
  test cannot read it.
- **`docs/property-registry.toml` and `docs/inheritance-rules.toml`:** the registry stays, checked
  against the code by its drift test extended to the new types; the inheritance rules go with
  inheritance (R20).
- **The showcases' gates** (element lists, parity) follow the connectors in R21.

### R23 — A value a toolkit cannot draw

A connector maps every value its toolkit can draw and lists every value its toolkit cannot in its
contract, with the toolkit's lines that show it; it also builds what the query needs (the element's
ancestor chain, R2.3) where its toolkit holds none. A toolkit's limit is recorded where it is; the
model still carries the value. This is the inverse of the failure that started this milestone, where
the model's limit was recorded as the toolkit's.

### R24 — Traceability

The specification maps every row of the map, identified by its file and ID, to the fields, rules,
roles or components that carry it, or to the decision that places it elsewhere: out of scope
(R1.1–R1.4), another preset (R1.3), the newest release (R1.5), the readers' milestone (R18). The rows
are the platform files' gap rows; withdrawn rows and the 42 pointer rows are excluded, and the
cross-check and pass-3 decision tables are decisions, not rows. A script checks that every row maps
to at least one target. This proves the model carries everything the map found; what the map, a
survey, did not find is caught by R4's gate, whose lists come from the platforms' own state models.

## 4. What the specification decides

The names and types of every field; the palette's surface sets, role record and keyed roles; the
text ramp's slots; the semantic variants and sizes; the condition, context and state lists in full,
and the exclusive groups; the selector and patch types and the TOML spelling of rules, derivations
and value kinds; the derive macro's changes; the resolution and query API; the placement strategies;
the derivation functions with their sources; the facts' data format and the gate's test; the
conversion of today's inheritance rules into the presets; and the mapping of every row (R24). Its
first steps are the kirigami-addons survey (R1.2) and the survey of the community projects' platform
ports (R19).

## 5. Check record

- **2026-10-28, author, passes 1–2** (first draft): numbering, the colour count, R14's scope cut,
  R18's iOS claim, R21's `ios-26`, right-to-left, Liquid Glass's refraction, unverified variant names,
  per-widget timings, cited row IDs.
- **2026-10-28, independent check 1:** 9 serious and 16 should-fix findings, all accepted; the
  document was rewritten around one rule mechanism (R2) and a facts gate (R4), with derivations over
  `currentColor`, KDE's effects and lengths, value kinds, palettes with a sourcing rule, flavours, iOS
  and Android readers decided alike, and the missing flags, text decoration, glyphs, scroll physics,
  the split caret, layered paint, the colour representation, placement strategies, the connectors in
  the release, and the documents.
- **2026-10-28, author, pass 3:** R2's resolution applied condition-only rules out of stated order;
  R4's combinations taken from the map instead of the platforms' sources; an unverified WinUI timing;
  R20 deriving from another state's value; the walk-through asking `adwaita` for a size it lacks.
- **2026-10-28, independent check 2:** of check 1's 35 findings, 25 resolved, 7 mostly, 3 partly,
  none open; 6 new serious and 18 should-fix findings, all accepted and resolved here: the layer
  order against accessibility and against palettes (R2.3's two passes; R19's palette replacing the
  reader's colours); palette rules naming states (palette rules take conditions only; disabled and
  inactive groups are roles, R5.1); the gate's prose facts, closed platforms and derivations (R4,
  R22's facts as data, macOS's and iOS's API state models, the perturbed palette); inheritance as a
  second mechanism (R20, removed); `over` beside paint layers and an undefined `text` (R5.3's
  `foreground`, R6's appended layers); palettes needing invented values (R19, from the projects'
  ports); and transforms, selector matching and negation, toolkit-reported combinations, density
  against size, undeclared sizes and variants, unvalidated keys, scoped surfaces, checked against
  selected, a part's own shadow, Expressive's pin, the windowing system's values, enum
  exhaustiveness, placement's implementer, the focus-visible rule, readers under conditions, light
  and dark as a condition, accent ramps, and query failure (R1.1, R1.6, R2.2–R2.5, R3, R5, R8, R9,
  R15–R18, R21). Its eleven minor findings (citations, the macOS reader's clipping, the
  `non_exhaustive` facts, `preset_for_version`, Breeze as C++, concentric's minimum, the `defaults`
  and tokens relation, spring parameters, line heights and automatic axes, R24's claim, KDE's effects
  on translucent input) are fixed in place. Its walk-through of a WinUI toggle switch under high
  contrast in a compact container now has every step decided: the switch's size is inherited from the
  compact container (R15), the reader's high-contrast colours are a reader rule under high contrast
  (R18), and the accessibility pass applies them last (R2.3).
- **2026-10-28, author, pass 4 (after the revision):** R5.1 had the inactive group as roles
  beside the disabled ones (it is a palette rule under a condition; disabled stays roles, being
  per element); why the colour scheme is a palette base while high contrast and Material's
  contrast schemes are rules was missing; a walk-through stated a port's existence as fact. A
  walk-through of a KDE Breeze list row,
  selected, in an inactive window, with the Catppuccin Mocha palette and the Kirigami flavour. The
  palette applies only if the specification's survey finds a Catppuccin port for KDE (a KDE colour
  scheme), converted mechanically into palette roles; it is dark only, so the window must be in dark mode or the API
  refuses it (R19). The reader's colour bases and rules are skipped, its other values kept. The
  palette is resolved for {dark, inactive, kirigami}: its bases, then its rules whose conditions hold,
  KDE's inactive group derived from the palette's active roles by KDE's effects with the user's effect
  parameters (R5.1). The row's query applies the list's rules in order for {selected} inside its list
  (the Kirigami flavour's rules where they differ); the selection fill refers to the selection surface's
  role, resolved against the palette. Every step is decided; R4's gate checks the result for Breeze's
  own colours, and the palette's result follows from KDE's own effects on the port's colours.
