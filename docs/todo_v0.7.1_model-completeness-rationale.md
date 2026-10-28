# v0.7.1 — Model completeness: Rationale

**Status:** design, drafted 2026-10-28 from the gap map; checked by the author and five times
independently, and revised after each check (§5); awaiting the maintainer's review. Input:
[`todo_v0.7.1_model-completeness-gap-map.md`](todo_v0.7.1_model-completeness-gap-map.md) and
its six platform files ("the map"; a row is identified by its file and ID, and cited here by
ID, with its file where the prefix does not say it). Next: the specification,
`todo_v0.7.1_model-completeness-spec.md`, which states every type, field and rule decided here
and maps every row of the map to them (R24).

The maintainer's rule (2026-10-27): native-theme covers **everything a platform draws
visibly**; a value no document states is measured on the platform; no gap is closed by a
toolkit's default. The map lists 2,292 rows of what the six platforms draw that the model
cannot express. This document decides how the model expresses them: the model, its presets and
its readers. The connectors move to the new model in the same release (R21); their own
milestones (ROADMAP.md, *Later — the connectors*) follow. Values are collected after the
specification, because what to measure depends on the shape decided here (the map's §3).

Each decision gives the options, their arguments for and against, and the choice with its
reason. §1 states the model as it is; §2 the principles; §3 the decisions: R1 scope; R2–R4 the
model's shape (blocks, rules, their order, and the facts gate that keeps the unstated honest),
which every later decision builds on; R5–R18 one group of the map's capabilities each; R19–R24
palettes, value kinds, release, documents, connectors and traceability. §4 lists what the
specification decides; §5 is the check record.

Two words are used in one sense each: **colour roles** are the model's named colours (R5.1);
a **palette** is a community colour scheme applied as a layer (R19). **Tree inheritance** is a
value an element takes from its containers at run time (a surface, a size); there is no model
inheritance between fields (R20).

## 1. The model today

Read at caeddd6b:

- **A widget is a flat struct of `Option` fields** (`native-theme/src/model/widgets/mod.rs`):
  `ButtonTheme` has `background_color`, `hover_background`, `active_background`,
  `disabled_background`, `checked_background`, `hover_text_color`, … each its own field, with a
  nested `font: Option<FontSpec>` and `border: Option<WidgetBorderSpec>`.
- **`#[derive(ThemeWidget)]`** (`native-theme-derive`) generates each struct's resolved twin
  (no `Option`), its merge, emptiness test, validation, range checks and inheritance rules. A
  field is `option` (resolution fails while it is unset) or `soft_option` (it may stay `None`).
  A `soft_option` `None` means one of several things today: "draw another value instead"
  (`button.checked_background`: the pressed pair), "sized by content", "the platform draws
  none", or "no published number".
- **Resolution** (`native-theme/src/resolve/mod.rs`) fills unset fields from defaults and
  related widgets ("~91 inheritance rules" by the module's own count, documented in
  `docs/inheritance-rules.toml`, whose `[wrong_safety_nets]` lists rules known to give wrong
  values), fills the run-time-detected fields, and validates into `ResolvedTheme`.
- **Presets** are TOML (`native-theme/src/presets/`), a full light and a full dark `ThemeMode`
  each: geometry and fonts are stated twice. A reader returns a partial `ThemeMode` merged over
  its platform preset (chosen by OS version, `pipeline::preset_for_version`); an application can
  merge its own (`SystemTheme::with_overlay`).
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
  (`text_scaling_factor`, `reduce_motion`, `high_contrast`, `reduce_transparency`), with no theme
  values behind them.
- **The `Option` structs are not `#[non_exhaustive]`** (docs/todo.md, *Dialog: the platform's
  vertical placement*), so a new field breaks struct literals; the resolved structs and some enums
  (`IconRole`, `IconSet`) are.

## 2. Principles

1. **What an application draws, the model carries.** A visible property of something an
   application or its toolkit draws, or configures for the system to draw, has a place in the
   model.
2. **One mechanism per job.** The map groups its rows by mechanism (C01–C28); each mechanism is
   designed once and used by every widget that needs it, and two mechanisms never do one job.
3. **The model invents no value.** A value is a platform's: read, published or measured. Where a
   platform derives one value from another, the model carries the derivation, not its result over
   one background. Every value a preset resolves to is checked against the platform's facts (R4).
4. **Typed where a portable application needs it, keyed where only one platform has it.** Both
   forms are built from the same blocks, declared, and validated the same way.

## 3. Decisions

### R1 — Scope

Each scope question is settled by the model's purpose: to let an application's toolkit draw what
the application shows.

**R1.1 — What the system draws.**

- *Include everything the system draws.* Against: what only the system ever draws (the on-screen
  keyboard, a notification banner) has no consumer, and the system draws it natively.
- *Exclude everything another process or a system library draws.* Against: a toolkit that draws
  its own window frame draws its own window menu (GTK's fallback, GN-P3-3†); an Android
  application configures its system bars and its splash screen (MAT-W1, MAT-P3-6†).
- *In: what the application or its toolkit draws; what a toolkit draws in place of the system's
  when it draws its own chrome or its own equivalent (an in-window toast, its own file list); what
  the application configures for the system to draw with; and what the system leaves to the
  toolkit to act on. Out: what only the system draws or applies, whatever the toolkit does.*

**Decision:** the third. In: the window frame, title bar, window menu and shadow wherever a
toolkit can draw them; an Android application's system-bar and splash values; the IME pre-edit
text; moving content out of the on-screen keyboard's way, which the toolkit does (Appendix B's kind K16); hiding the
pointer while typing, which the toolkit asks for (R17); Win32's global effect switches, which a
toolkit honours (W3-53†). Out: the on-screen keyboard itself, notification banners, home-screen
widgets, Live Activities, Control Center, a shell's tiling preview and Snap layouts, an input
method's own candidate window, system dialogs a toolkit only invokes (the print dialog, GNOME's
portal file dialog drawn by Files), and what the windowing system applies to every toolkit without
its involvement (the display scale and safe-area insets the windowing library reports, whole-screen
colour filters and Smart Invert; Appendix B's kinds K17, K23 and K27, and X-A11-8). Each row of the map and each kind of its
Appendix B gets its verdict by this test in the specification's traceability (R24).

**R1.2 — Libraries beside the toolkit.** Test: maintained by the platform's own project (GNOME,
KDE, Microsoft, Apple, Google's Material team) and released as stable.

- *The toolkit only.* Against: GNOME Text Editor's and Kate's code views, KDE's form cards and
  Windows' settings cards are the platform's look to its users.
- *Every library any platform application uses.* Against: a third-party library carries its
  vendor's look (KDDockWidgets draws KDAB's own drop arrows; the seed follow-up, G19).

**Decision:** the test. In: GtkSourceView, libpanel, kirigami-addons, KTextEditor,
KSyntaxHighlighting, KQuickCharts, the Windows Community Toolkit, Swift Charts. Out: the Toolkit
Labs (experimental; its `MarkdownTextBlock` rows wait for a Toolkit release) and KDDockWidgets.
kirigami-addons has no rows yet: its survey is the specification's first step.

**R1.3 — Other toolkits on a platform** (GTK 3 applications on GNOME; WPF and Win32 controls on
Windows). A platform preset describes the platform's current native toolkit; the others are
candidate presets of their own (ROADMAP.md, *Beyond v0.7*). One preset holding two toolkits'
looks for one widget would make the application choose which to copy, which is what choosing a
preset already does. Rows: GN-P3-17†, W-P3-15†.

**R1.4 — SwiftUI and Mac Catalyst.** AppKit and UIKit are the reference where SwiftUI draws a
control through them; a control only SwiftUI has (`Gauge`, the grouped `Form`) is a control of the
platform (R14). A Mac Catalyst application draws either the iOS look or, with the Mac idiom, AppKit-style
controls, both of which the `ios-*` and `macos-*` presets cover; its Catalyst-only rows are out.

**R1.5 — Versions inside a generation, and choosing the generation.** A preset describes the
newest release of its generation (macOS 15 for `macos-monterey`, libadwaita 1.10 for `adwaita`,
iOS 18 for `ios-13`); readers read what the running release states; a release whose look differs
as a whole starts a new generation and preset, as macOS 26 did. A per-release overlay inside a
preset was rejected: it multiplies values by releases for differences the readers already read
where they can. The generation is chosen by the running OS version, as
`pipeline::preset_for_version` does now, and by the application's own opt-out where a platform has
one (iOS's `UIDesignRequiresCompatibility`, X-MT-13). On iOS and Android, where native-theme has no
reader and no supported, tested target until R18's milestone, the application names the preset and
supplies the conditions meanwhile, and may add a layer marked as standing for the reader, whose values
are then placed as a reader's are (bases, and rules under the conditions they were read in, R18).

**R1.6 — Two current looks on one platform.** KDE draws Qt Widgets and Kirigami applications
differently in places (the focus indicator, K3.20 against Q3.30); Material 3 has its baseline and
its Expressive look.

- *Two presets each.* Against: they share most values, stated twice.
- *Pick one.* Against: the other current, widely used look is left out.
- *One preset with a flavour,* a condition the application chooses (R2.2), the default stated.

**Decision:** flavours. `kde-breeze`: `widgets` (default; most KDE applications) and `kirigami`;
`material-3`: `baseline` (default) and `expressive`. For a design system the source test is its
published tokens at a pinned material-web commit, never a moving branch: the baseline flavour's from
`tokens/versions/v0_192/`, the expressive flavour's from `tokens/versions/latest/` (which holds the
Expressive tokens). The two sets differ in places (the pressed state layer's opacity: 0.12 in v0_192,
0.1 in latest), and each flavour takes its own. Where a flavour's tokens are silent, material-web's
components are the reference; where material-web has no such component, Compose's, cited at a pinned
commit, experimental or not, because for some Expressive behaviour it is Google's only implementation.

### R2 — The model's shape: typed blocks and one rule mechanism

The map needs values per part; per state and combination of states; per condition and combination
of conditions; per container (libadwaita's icon buttons are flat inside a header bar unless raised,
suggested, destructive or opaque, GN-1.151; Kirigami's colour set follows modality, Q1.53); per
position among siblings (a boxed list rounds its first and last rows, GN-1.165; linked buttons,
GN-1.42; segment ends, MAC-SG08); per variant, where variants stack (libadwaita's style classes);
per control size; plus shadows, motion and materials, which today have no type.

- **A. More flat fields on today's structs.** For: no new concepts. Against: every combination is a
  field; one button body (fill, text colour, stroke, shadow, icon colour, corners, opacity) times
  nine state combinations is some 60 fields per part before conditions, containers and variants,
  most repeating another value.
- **B. A string-keyed style-rule engine** (CSS-like). For: compact; GTK's themes are built this way.
  Against: untyped, so the compiler checks nothing and a typo is a silent miss.
- **C. Typed blocks and parts with separate mechanisms** for states, conditions, variants, sizes and
  palettes. Against: several mechanisms merging partial looks with no order between them; none
  handles containers, siblings or combined conditions.
- **D. Typed blocks and parts, and one typed rule mechanism.** A rule is a typed selector and a typed
  patch: *when* these conditions hold, inside this container, at this position, with these variants,
  at this size, in these states, *set* these values. For: typed throughout; one mechanism carries
  states, combinations, conditions, containers, positions, variants and sizes; it mirrors how
  platforms state their looks (GTK's CSS, WinUI's visual states and theme dictionaries; Breeze's C++
  branches on the same inputs). Against: every widget is rewritten, and the derive macro must
  generate typed patches.

**Decision: D.** Pre-1.0 the project promises no compatibility; D is the only option that is both
typed and one mechanism.

**R2.1 — The blocks** (fields are the specification's):

| Block | Carries | Capabilities | Decision |
|---|---|---|---|
| colour | an extended-sRGB colour, or a colour derivation | C04, C05, C28 | R5 |
| length | logical pixels, automatic, or a derivation from the font or line height | C26 | R5.3 |
| paint | a stack of layers, bottom first: colour, gradient or material, each with a blend mode | C03, C08, C27 | R6 |
| corners | a radius per corner, fully rounded, concentric; the corner curve | C11 | R7 |
| stroke | a width and paint per side; inside, centred or outside | C12 | R7 |
| shadow | layers (offset, blur, spread, colour, inset), or a reference to an elevation ramp entry | C01, C02 | R8 |
| transform | scale, rotation and translation about an origin | C13, C14 | R9 |
| transition | what changes, duration, delay, timing (curve, steps, spring) | C14 | R9 |
| text style | font (family, size, weight, style, width, axes, features, fallbacks), letter spacing, line height, decoration, elision and wrapping, and its fill, a paint (R6) | C18 | R10 |
| icon style, glyph | an icon's size, rendering mode, weight, effect; a glyph's source and geometry; the fill of either, a paint (R6) | C19, C13 | R11 |
| focus indicator | kind, stroke, offset, corners, shadow, transition | C15 | R12 |

A value of any block may also be a **reference** to a token (a motion token, an elevation entry, a
text-ramp slot, an icon size) or a colour role; references are derivations (R5.3).

A **part style** combines the blocks: fill, stroke, corners, shadow, opacity, transform, text style,
icon style or glyph, focus indicator, padding per side, size limits, cursor (R17), role bindings for
its descendants (R5.1), and its property transitions (R9). A **widget** is a typed set of named parts
plus widget-level values (gaps between its parts, alignment, placement, presentation transitions,
behaviour). A **preset** is its
declarations (R2.4); its tokens (motion, elevation ramp, text ramp, icon sizes, behaviour); its colour
roles' bases per colour scheme (R5.1); each part's and widget's base; and an ordered list of rules.
Today's `defaults` becomes the colour roles and the tokens.

**R2.2 — Rules.** A rule's patch sets values of one target: a widget's part, a widget's own values,
the colour roles, or the tokens. Its selector has these parts, each optional, each a set that may also
be negated (*not* these), and the rule matches when all its parts hold:

- *conditions,* window- or theme-wide, each with the source that supplies it:
  - from the system, by the reader: increased contrast, high contrast, reduced transparency, reduced
    motion, animations off (GNOME has both settings, `reduced-motion` and `enable-animations`; KDE's
    speed factor 0 sets animations off, so the factor and the condition are not two routes to one
    job), bold text, button shapes, differentiate without colour, and the colour scheme (light, dark),
    which the application may set instead, as `SystemTheme::pick` lets it today. Each reader states
    which of its platform's settings is increased and which high contrast: macOS's and iOS's Increase
    Contrast are increased; Windows' contrast themes and GNOME's high contrast are high; Material's
    levels (MAT-X18) are standard (neither), medium (increased) and high (high); KDE has no contrast
    setting of its own (a high-contrast colour scheme is a scheme the reader reads). User settings
    that choose a size for one kind of container are conditions of this kind too: macOS's sidebar icon
    size (small, medium, large; MAC-SB01), which the sidebar's rules select on; it applies to source
    lists at the default row size, and a list that states its own row size style has that as a variant
    (MAC-L08).
    `AccessibilityPreferences`' four flags become these conditions; `text_scaling_factor` stays a
    factor (R10);
  - from the window, by the connector: window inactive (the window does not have the input focus: on
    macOS, not key), and, on macOS, not main and application inactive (MAC-X16); window maximised,
    tiled or full screen; iOS's elevated level where it holds for the whole window (multitasking; in a
    sheet or a popover it is a role binding, R5.1); touch input (WinUI sizes for touch and mouse
    differently, W1-047, W1-134, W3-40; KDE's tablet mode, K3.83, Q3.31), from the input the window
    last received, or from the reader where the system has a mode;
  - the full list, with every system condition the map names (Windows' battery saver, which turns
    Acrylic off, W3-07, among them), is the specification's;
  - from the application: flavour (R1.6) and preset (an application's rule for one preset only, so
    that rules naming one preset's keyed names do not fail on another).
- *context,* the element's containers: any ancestor of the named kind, or the nearest ancestor among a
  stated set of kinds ("nearest of {header bar, popover} is a header bar": flat in a header bar, raised
  again in a popover opened from it), each ancestor matched by widget, part, variants and states.
- *position,* among the element's siblings: first, last, only; and the previous and the next
  sibling's widget, variants and states, for the separators that hide beside a hovered or selected neighbour (MAC-SG01,
  Q2.56†) and the shared glass capsules of grouped bar items (MAC-X07, TB-5).
- *variants,* a set the element must have and a set it must not (libadwaita's
  `:not(.raised):not(.destructive-action)`).
- *size,* the element's own control size, and the density and window size class it inherits from its
  containers (R15, R16); and its text direction, which it inherits too (a Material text field can keep
  left-to-right text inside a right-to-left layout), set at the root by the connector.
- *states,* a set of state flags (R3), and a set that must be absent.

Rules targeting the colour roles or the tokens may use conditions only, because they are resolved per
window (R2.3); values that vary per element are set by rules on parts and widgets, which refer to the
roles and tokens.

**R2.3 — Order, layers, resolution and query.** Rules apply in stated order, each over what the earlier
ones gave, with no specificity arithmetic: the platforms' own orders differ, and an explicit order is
the one every author can read. The sources are the preset, the reader (R18), and the application's
layers in the order the application adds them; a palette is an application layer (R19). Every
source's bases come first, in that order, then every source's rules, in that order: a base after a
rule would undo it (a reader's selection colour stated after the preset's inactive-window rule would
erase the inactive look).

- *Accessibility last, over every layer.* Against: false to the platforms (GTK's application
  stylesheet outranks the theme's, WinUI's local values outrank theme resources), and a preset's
  accessibility rules would override all its own state rules, so every state look would have to be
  restated under each accessibility condition.
- *One order, the application last, as on the platforms.* For: true to the platforms; a preset states
  its accessibility rules where they belong in its own order. Against: an application layer can
  override the user's accessibility look.

**Decision:** one order. Where an application overrides the platform's accessibility look, it does what
the platform lets it do; a palette, which replaces the reader's colours, is not applied while either
contrast condition holds (R19): the preset's contrast colours are then the user's choice, and a palette
states no contrast colours of its own unless its port does.

**Resolution** runs per window and again when a condition, a reader value (the user's accent, a
setting) or an application layer changes; its result is cached. An application layer that fails
validation (R2.5) is refused when it is added, with the error returned to the application, and the theme
stays as it was. It applies the colour-role and token rules whose conditions hold, in order, which chooses each
role's and token's expression; and for each part and widget keeps its base and the list of its rules
whose conditions hold, still in stated order. Expressions are evaluated where they are used, in the
query, not where they are defined: a role or token derived from the accent follows a subtree that
rebinds the accent (libadwaita computes its focus-ring colour at each use from the accent in scope,
`_colors.scss:307`), and `foreground` means the using element's. Resolution still checks every
expression against every binding it can meet (R2.5). Inside a binding, a reference to the role being
bound means its value outside the binding (as CSS's `inherit`), so a binding never refers to itself. A **query** asks for one part's style (or one widget's values) for one element. Its input is the
element's widget, part, variants, size and states, the point it was pressed at (for the ripple, R9), its
ancestor chain (each ancestor's widget, part, variants and states, and the role bindings, density, size
class and text direction it passes down), and its position
among its siblings with its previous and next siblings' widget, variants and states. iced and egui hold
no such tree, so
their connectors build one (R23). The query applies the kept rules in order, skipping those whose
context, position, variants, size or states do not match, and evaluates derivations last, because a
derivation can refer to the value beneath it in this very query (R5.3). Applying condition-only rules
early would put them before element rules stated ahead of them.

**R2.4 — Declarations.** A preset declares, per widget, its variants, sizes and densities (per flavour
where they differ, and per container where the platform draws a size only in some, such as a large
button outside a navigation bar), its widget-specific state flags and exclusive state groups (R3), its
keyed components and their parts (R14), its keyed colour roles, text styles and elevation entries, its
flavours, and which colour roles are inputs that a change of accent sets (R5.3). Resolution checks every
rule and reference against the declarations: a typo is an error, not a silent miss; a query for a
variant or size the element's widget does not declare in its context returns "not available", which
is a result an application handles, not a failure.

**R2.5 — Validation and infallible queries.** The project forbids runtime panics, and a query runs inside
a connector's draw call. Resolution checks every rule's derivations against every base and rule they can
apply over, and every expression against every role binding it can meet: types, references, cycles,
and operands that could be absent. A preset that fails is an
error at resolution; a query cannot fail.

### R3 — State flags

The flags, the union of what the map's platforms draw: hover, pressed, focused (has focus), focus
within (a descendant has focus), focus visible (focus shown), disabled, checked (on), selected,
indeterminate, open (expanded, menu shown), default (a dialog's default button), dragged, drop target,
read-only, invalid, visited, autofilled, scrolled (content under a bar: T-1, C-1), over dark content
(iOS 26's glass flips its look by what lies beneath, which the toolkit knows), and per widget the flags
only it has (a slider's overlapping handles, MAT-S10). Checked and selected are separate because a Qt
list item carries both (`checkState`, `State_Selected`); focused and focus visible because macOS draws
an unemphasised selection without focus (MAC-I05, MAC-SB05) while most platforms show focus only after
keyboard navigation (K3.20); when focus becomes visible is a behaviour (R17). Window activity and window
states are conditions (R2.2).

**Exclusive groups.** Some platforms hold one state of a group at a time: WinUI's `CommonStates` is one
of Normal, PointerOver, Pressed and Disabled. A connector may still report flags together that such a
platform never combines (a pointer over a disabled control). A preset declares its platform's exclusive
groups and, for each, its flags in order of precedence (for `CommonStates`: disabled, pressed, hover),
and the query keeps the highest present and drops the others before matching, so no look is made up for a combination the platform does not have.

**State layers** (Material's hover, focus, pressed and dragged overlays, C08; libadwaita's
`currentColor` shares) are a paint layer a rule appends over the fill beneath (R6), with a colour
derivation (R5.3). Material's ripple is the pressed state layer's transition, which grows from the
press point (R9).

### R4 — The facts gate

With rules in stated order, a combination no rule states gets what the earlier rules give: the
platform's own answer where the platform composes the same way, and not necessarily elsewhere.
libadwaita's flat button is 7 % on hover, 10 % checked, 13 % checked and hovered (GN-1.29), which
appending the first two does not give, so the preset must state {checked, hover}.

- *Trust the order.* Against: a forgotten combination passes silently.
- *State every combination of every flag.* Against: thousands of repeats.
- *A gate that resolves every value a platform draws and compares it with its fact.*

**Decision:** the gate, built to run mechanically:

- **Facts as data** (R22): each fact is a value (or the platform's own derivation) for a key: the model
  path and the selector it holds under, with its source kind and citation.
- **Which combinations:** per widget, from the platform's own state model: WinUI's visual states,
  libadwaita's selectors, Breeze's state branches, Material's tokens; on macOS and iOS the API's control
  states (`NSControl`, `UIControl.State`) and appearances, times the conditions, and the containers the
  API distinguishes (a toolbar item, a bar item), and the styles and sizes the API enumerates
  (`NSButton.BezelStyle`, `NSControl.ControlSize`, `UIButton.Configuration`), each measured by the
  probe. The map is a survey; the
  platforms' state models are the list.
- **Derivations:** where a platform states a derivation, the fact is the derivation. The gate resolves
  each preset twice, with its own colour roles and with perturbed inputs (another accent, foreground
  and background), and compares both with the facts evaluated the same way, so a literal stated where
  the platform derives fails. On macOS the reader reads the system's own colours under the user's
  accent (`NSColor` resolves them), so its accent-dependent values are read, and the facts are
  measurements under each appearance. iOS has no reader until R18's milestone: its facts are measured
  under each appearance and the default tint, its tint-dependent roles are accent inputs (R5.3), and
  an application that sets another tint states them.

There is no model inheritance for the gate to trust (R20).

### R5 — Colours (C04, C05, C08, C28)

**R5.1 — Colour roles and surfaces.** Today's `defaults` has 23 colours in a flat list. KDE's scheme has
seven sets of the same roles (View, Window, Button, Selection, Tooltip, Complementary, Header), each in
Active, Inactive and Disabled groups (K3.50–K3.55); libadwaita names its colours per surface (window,
view, header bar, sidebar, card, dialog, popover); Material has a ladder of surface containers and
on-colours (MAT-X1), WinUI a ladder of fill and stroke tiers.

- *A longer flat list.* Against: loses which roles belong together; cannot hold KDE's sets.
- *Surface sets with one role record for every platform.* Against: Material's and WinUI's ladders are
  not surfaces.
- *Typed surface sets with a common role record, global accent and status roles, and a keyed set of the
  platform's own roles* (Material's ladder, WinUI's tiers).

**Decision:** the third (principle 4).

**Groups.** The window being inactive holds for a whole window, so an inactive group is a colour-role
rule under the inactive condition; being disabled is per element, so each surface set also holds
disabled roles, which parts' {disabled} rules refer to. On KDE both groups are derived from the active
roles by KDE's colour effects (R5.3) with the scheme's effect parameters, as `KColorScheme` does: the
inactive group by the inactive effects applied to an inactive input role, whose base is the active
value and which the reader (or a palette from a KDE port) replaces in its bases where the scheme states
an Inactive value (`[Colors:Header][Inactive]`, K3.55; `kcolorscheme.cpp:381` applies the effects over
it too); the disabled roles by the disabled effects over the active values, in an inactive window as in
an active one. The scheme's stated Inactive values are inputs, never rules, so KDE's inactive effects
apply over them. A
role's **active value** is its value from the bases of every layer, before any rule; a derivation can
name it (R5.3).

**Schemes and contrast.** The colour roles have a base per colour scheme (light, dark), because exactly
one scheme always holds and the two differ in most roles; every other condition is optional and combines
with others, so it is a rule, even where it restates every role: Windows' high contrast replaces light and
dark alike (one rule, from the reader's eight system colours, W3-35), and Material's medium and high
contrast schemes (MAT-X18) are rules under the scheme and the contrast condition together. Geometry is no
longer stated twice: a value that differs in dark is a rule under the dark condition.

**Role bindings (tree inheritance).** A part style may bind roles for its descendants: which surface set
they draw on (a header bar binds the header set; Kirigami's `Theme.colorSet`, Q1.53, Q1.96), and any
single role (libadwaita's `.error` rebinds the accent to the error colour for its subtree, `.osd` and
toasts rebind their surface, KDE's sets their decoration roles; iOS's elevated level is a sheet or a
popover binding its descendants' backgrounds to the elevated roles). A binding applies to the binding
element's own parts and to its descendants, as GTK's `color` applies to the node that sets it and its
children. A role reference resolves against the nearest element, itself or an ancestor, that bound it,
and otherwise against the window's root surface, so a reference always resolves. Without it every
widget would restate its role references per container, and an invalid entry row would restate the error
colour in every part.

**R5.2 — Translucent colours.** KDE, libadwaita and WinUI state many fills as translucent colours
composited over what lies beneath (GN-3.08, W3-45†); today's presets flatten them over one background,
wrong on every other surface. **Decision:** a preset states the translucent colour the platform states,
and the toolkit composites it, as the platform does.

**R5.3 — Derivations.** A value that follows another value (the accent the user picks, a high-contrast
colour, the current foreground, the font size, a token) cannot be stated as a result.

- *Derive in each reader.* Against: without a reader, under a palette, or after a rule changes the
  accent, every derived value would be stale.
- *Derivation expressions in the model.*

**Decision:** expressions. Their operands: colour roles (typed, keyed, or bound, R5.1); tokens; `base`
(the value the earlier rules gave in this query); a role's active value (before any inactive rule, for
KDE's disabled group, R5.1); `foreground` (the element's current foreground: the
foreground role of the surface bound for it, unless a rule rebinds it; independent of whether the part
draws text; libadwaita's `currentColor`); and for lengths the part's font size and line height. Their
functions are a closed set, each with one definition and its source: `alpha(c, a)` multiplies the alpha
(GN-1.210 multiplies an alpha already below one); `mix(a, b, t)` interpolates in sRGB with premultiplied
alpha, as CSS `color-mix(in srgb …)` does (libadwaita's most frequent form, `color-mix(in srgb,
currentColor 7%, transparent)`, is the colour at 7 % alpha only so); KDE's colour effects and the
`KColorUtils` functions they use, to KDE's code (their behaviour on translucent input is checked against
KDE's code in the specification); libadwaita's derivation of its standalone accent colours in OKLab, to
its stylesheet's code; Material's scheme generation (material-color-utilities, which defines Material's
dynamic colour, MAT-X17), from the seed colour, an accent input (R2.4), with the scheme variant (tonal
spot, … , and the variant Material also calls "expressive", unrelated to the flavour) and the
specification version as the preset's tokens, and the contrast level from the contrast conditions
(R2.2); and for lengths, multiples
of the font size or line height (Kirigami's grid unit, Q1.62; a width in characters, GN-1.283†). A
platform whose arithmetic differs gets its own function; a function is added only when a platform's rule
needs it. Composition over the value beneath is a paint layer appended over `base` (R6), not a colour
function. Evaluation follows dependencies; a cycle is an error at resolution (R2.5).

**Accent inputs.** A preset declares which roles a change of accent sets (R2.4): the accent on most
platforms, the seed colour on `material-3` (its roles follow by the scheme function). WinUI's accent ramp
(AccentLight1–3, AccentDark1–3) comes from an algorithm Microsoft does not publish; the reader reads it
from `UISettings`, so on `windows-11` the ramp is an input too, and a palette or an application that
changes the accent must state it; the API refuses an accent alone there. macOS is the same: its selection,
focus and accent-tinted colours follow the user's accent by a derivation Apple does not publish (the
reader reads them, R4), and its highlight colour is a separate user setting; on the `macos-*` presets the
accent-dependent roles and the highlight colour are inputs, stated together when an application changes
the accent. On iOS the tint-dependent roles are inputs in the same way (R4).

**R5.4 — Representation (C28).** macOS and iOS define system colours beyond sRGB (Display P3; MAC-X10,
D-17); `Rgba` and the macOS reader's conversion to sRGB clip them. HDR appears only in the colour well's
content (MAC-X10), the application's data.

- *Keep 8-bit sRGB.* Against: P3 colours clip.
- *8-bit channels and a colour-space tag.* Against: derivations would need a rule for mixed spaces.
- *Floating-point extended sRGB* (sRGB primaries and transfer, channels beyond 0–1), which AppKit and
  UIKit hand out when asked for it. For: one space and one arithmetic; P3 kept exactly; a connector
  converts or clamps for a toolkit that renders sRGB.

**Decision:** floating-point extended sRGB; the readers ask for it. HDR is not modelled: no theme value of
the six platforms uses it.

### R6 — Paint: layers, gradients, materials, blending (C03, C08, C27)

A paint is a stack of layers, bottom first: a colour (R5), a linear or radial gradient (colour stops with
positions; macOS 12–15's checkbox box, MAC-X09, bezels to verify, MAC-B05; WinUI's elevation strokes,
W3-04), or a material, each with a blend mode (macOS's vibrant foregrounds blend with what is beneath,
MAC-X03). A rule sets a part's paint or appends layers over `base`: libadwaita's stacked fills (GN-1.30,
GN-3.06) and every state layer are appended layers.

**Materials** (MAC-X02, MAC-X03; Liquid Glass, MAC-X04–MAC-X06, X-MT-5–X-MT-13; Mica, Mica Alt, Acrylic,
W3-06–W3-10; KWin's blur, K3.11):

- *The opaque fallback only* (today). Against: the look is the translucency.
- *A named material only.* For: exact where a toolkit can ask the system. Against: inside a window most
  toolkits cannot.
- *The platform's identity of the material, the parameters the platform states or a measurement gives
  (blur radius, tint colour and opacity, saturation, luminosity, noise), and the opaque colour the platform
  falls back to.*

**Decision:** the third. A parameter the platform has no counterpart for is absent (R20); what parameters
cannot describe (Liquid Glass's refraction) is carried by the identity, and a toolkit that cannot use the
system's material records that as its own limit (R23). Two things are kept apart: what the platform
draws under reduced transparency is a rule of the preset under that condition (an opaque colour on
macOS, a frostier glass on iOS 26); the material's opaque fallback is what a toolkit that cannot draw the
material draws instead.

### R7 — Corners and stroke (C11, C12)

**Corners:** a radius per corner; fully rounded (a capsule at any height: Material's `full`, iOS 26's
capsules); concentric (the container's radius less the inset, with a minimum where the platform states one,
X-CR-1–X-CR-3), which the toolkit computes at layout from its container; and the corner curve (circular, or
Apple's continuous). A boxed list's rounded first and last rows are rules by position (R2.2). A shape that
changes between states is a rule with its transition.

**Stroke:** a width and paint per side, inside, centred or outside the edge: WinUI's gradient elevation
strokes and the thicker accent bottom edge of a focused text box (W1-026); libadwaita's outlines outside
the box. A second outline (a rim, a bevel's highlight) is an inset shadow layer (R8), not a second stroke.

### R8 — Shadows and elevation (C01, C02)

**Shadow:** a list of layers (x and y offset, blur, spread, colour, inset), or a reference to an entry of
the platform's elevation ramp. GNOME draws three layers for a window (GN-1.01); Material a key and an
ambient layer (MAT-X2–MAT-X4); Windows only the ambient layer of Fluent's two (W3-01); Breeze two, with
strength read from `breezerc` (K3.1).

- *Layers per part only.* Against: Windows, Material and KDE share shadows by elevation level.
- *Typed portable roles in the tokens* (card, menu, dialog). Against: a role cannot say that a WinUI
  control drops from elevation 2 to 1 when pressed (W3-02), or that a Material button rises a level on
  hover (MAT-X4), without restating shadows.
- *Both forms: a platform with a ramp (WinUI's 1–128, Material's levels 0–5, Breeze's sizes) states it
  once, keyed by its own names, and parts refer to entries, a rule changing the entry; a platform without
  one (GNOME, macOS, iOS) states layers per part.*

**Decision:** both forms. `shadow_color` and `shadow_enabled` go: a shadow's colour is in its layers, no
layers is no shadow, and the inactive window's shadow is a rule. Material's surface tint by elevation is a
paint layer in the same rules.

### R9 — Motion and transforms (C14, C08's ripple)

The platforms animate nearly every control (MAC-X11, W3-30–W3-34, K3.30–K3.37, GN-3.18–GN-3.25,
MAT-X13–MAT-X15, X-MO-1).

- **Transform:** scale, rotation and translation about an origin, a part-style property rules set and
  transitions animate: iPadOS's pointer lift (X-PH-1, B-17), macOS's disclosure triangle turning (MAC-X11),
  WinUI's pressed knob stretching (W1-120) and its chevron (W1-132).
- **Timing:** a cubic Bézier, linear, steps, or a spring stated physically (mass, stiffness, damping, with
  the initial velocity, rest threshold and clamping libadwaita's `AdwSpringAnimation` states), as
  libadwaita and Material state springs; Apple's response and damping fraction convert to it exactly;
  velocity-dependent curves (Kirigami's, Q3.26) are a timing kind of their own.
- **Property transitions:** each style (a part's base, or a rule's patch) may state how properties move
  into it, per property group, with duration, delay and timing. When an element's matched rules change,
  the transition of the style after the change governs, in both directions, as in CSS and GTK
  (`gtkcssanimatedstyle.c`); where several rules state one for the same property, the later in stated
  order wins, as for any value. XAML's alternative, a transition per (from, to) pair of states, was
  weighed: CSS's form expresses a direction-dependent motion by putting it on the style entered, and no
  WinUI row needs a pair.
- **Layers and materials in transitions:** a paint layer that appears interpolates from its colour at
  zero alpha, and one that disappears to it; a material that gives way to its reduced-transparency look
  crossfades between them over the transition stated (Mica's fade, W3-10).
- **Presentation transitions:** an element appearing, disappearing or moving (a popover opening, a
  dialog's entrance, a page pushed, a sheet presented), keyed by event per widget, which rules can change
  (reduced motion, flavours); the kinds are an open list in the specification, from the map's rows: fade,
  slide from an edge, scale, shape morph, clip reveal (WinUI's combo box opens by revealing from the
  selected item, W1-136), and the rest. `dialog.entrance` becomes one of them.
- **Theme changes:** what the platform does when the colour scheme, the accent, a contrast setting or the
  theme itself changes (Appendix B's kind K57; GN-3.23, W-P3-16†) is a token, the theme-change
  transition: a crossfade of the window with its duration, or none; and while it runs, the elements'
  property transitions are suspended, because they would animate every value at once (libadwaita
  switches its CSS transitions off for 250 ms on every style update, `adw-style-manager.c`). A theme
  change is a change of conditions, reader values or layers (R2.3), never of an element's state.
- **Ripple:** a property transition of the pressed state layer whose origin is the point pressed, which
  the query receives (R2.3): the layer grows from it, with the radius, durations and opacities Material
  states (MAT-X9) as its values; its fade on release is the rest style's transition for that layer, as
  the style entered governs.
- **Loops:** a period, a timing and the keyframes the platform states, extending `AnimatedIcon` (spinners,
  indeterminate progress).
- **Scroll physics:** deceleration, overscroll and its spring, snapping, distance per wheel step (GN-3.24,
  W3-52†).
- **Tokens and factor:** the platform's published durations and curves, referred to by parts; a global
  speed factor (KDE's `AnimationDurationFactor`, K3.30) read by the reader.
- **Reduced motion and animations off:** rules under these conditions stating what the platform does
  instead: libadwaita swaps a bottom sheet's slide for a crossfade under reduced motion, and under
  animations off jumps to the end (GN-3.25), which the preset states as a rule setting the speed factor
  token to zero. The model never zeroes durations by itself.

### R10 — Typography (C18)

**Text style:** the font gains width (stretch), variable-font axes (`opsz`, `wdth`, …; an axis may follow
the size automatically, as SF's optical size and Segoe UI Variable's do, W3-23), OpenType features, a
fallback family list, an emoji family and a per-language fallback map (MAC-FM01†, W-FM01†, K-FM01†,
GN-FM01†, MAT-FM01†, FM-1†); the style gains letter spacing (tracking, which SF Pro states per size, D-1),
its own line height per text role and style (today only `defaults` and the four text-scale entries have
one), **decoration** (a line's kind, thickness, offset and colour: the spell-check and IME underlines,
mnemonic underlines, a link's underline; K1.49†, W1-151†, GN-1.281†), and **elision and wrapping** (where
a platform truncates, at the end or the middle, or wraps).

**Text roles:** today four `text_scale` entries besides the body font; macOS and iOS have eleven styles,
Material fifteen and fifteen emphasized (MAT-TS4), WinUI eight, libadwaita about nine.

- *Each platform's own names only.* Against: no portable "heading".
- *A fixed semantic ramp only.* Against: either huge or merging styles.
- *A typed semantic ramp (each preset filling every slot, its facts saying which of its styles fills it)
  plus a keyed set of the platform's own styles.*

**Decision:** the third (principle 4). The text-scaling factor stays for platforms that scale linearly;
Dynamic Type and Android's non-linear font scaling (MAT-X19) are read per style by those platforms'
readers (R18), and a preset states the default size category's values.

### R11 — Icons and glyphs (C19, C13)

**Icon style:** size (the existing `defaults.icon_sizes` contexts become its tokens), fill (the
recolouring of symbolic icons, a paint, so vibrant icons blend, MAC-X03), rendering mode (SF Symbols'
monochrome, hierarchical, "palette" in Apple's sense, multicolour), weight and scale, and a per-state effect where the platform changes an icon by state (KDE's
disabled and active effects, K3.60). Mirroring in right-to-left layouts is a property of each icon role
(MAT-FM04†). Symbol effects are transitions.

**Glyph:** a check mark, a radio dot, a disclosure arrow, a combo chevron: glyphs no icon set has
(`IconRole` has none of them). A glyph is the platform's own, from one of five sources: a code point of
its symbol font (Segoe Fluent Icons); an icon of its icon theme by name; a vector path taken from its
source code (Breeze draws its check mark with `QPainterPath`); a system image the platform names for
that purpose (AppKit's menu on-state check mark); or, where the platform names none, the system's own
rendering of the whole control. AppKit draws its controls from CoreUI's compiled `.car` assets (as
`platform-facts.md` §2.11 records for the tab bezel) and names no image for the checkbox's mark, the
pop-up button's arrows or the disclosure triangle (its named triangles are generic left- and
right-facing images, which Apple says to use for their purpose, not their look): for MAC-C06, MAC-CB01,
MAC-EX02 and MAC-L10 the part's look is the control as AppKit draws it (`NSButton` with its checkbox or
disclosure bezel, `NSPopUpButton`), in the state, size and appearance asked for, rendered by native-theme
at run time through AppKit, as the SF Symbols loader renders symbols today, and cached per resolution.
That rendering replaces the parts it covers (the checkbox's box and mark together), uses the system's
accent (AppKit draws with it; an application's accent cannot reach it, which the connector records,
R23), and carries no separate glyph geometry. Measuring the glyph's geometry from renders instead was
weighed and rejected: its outline can only be traced from a picture, which the project's asset rule
forbids; its size and colour are still measured as values. A toolkit that cannot use a system rendering
records that limit (R23). Elsewhere a glyph's size, stroke width and fill are its values.

### R12 — Focus, caret, selection and highlights (C15, C16, C17)

**Focus indicator:** a block (R2.1) every focusable part's style carries: a kind (ring, filled ring,
underline, none), stroke, offset, corners (concentric with its part unless stated), shadow and transition,
shown in the focus-visible state. The base states it, by reference to the platform's token where the
platform has one indicator, or its own where a part differs (KDE's focus line under tab labels, K3.21). A
focus that only changes a fill is a {focus visible} rule, not a kind. Today's three `focus_ring_*` fields
become the token.

**Caret:** colour, width, shape (bar, block in overwrite mode; K1.47†, KDE file), the secondary caret in
mixed-direction text (GN-SF03†), blink on and off times and timeout, read where the platform has a setting
(GNOME, GN-3.32–GN-3.38; Windows; KDE; macOS's blink period, MAC-X14), and when it hides (while text is
selected, GN-3.73†, GNOME file).

**Selection and highlights:** colour roles per surface (R5.1); the find highlight and the drop-target
highlight are roles and parts of the widgets that draw them.

### R13 — Window chrome (C20)

The window becomes parts: the frame (paint, corners, outline, shadow), the title bar (heights per variant,
padding, title alignment and text style), the caption buttons (one part type, each button with its glyph
and its own rules, such as the close button's red hover), the window menu, and the resize zone. Maximised,
tiled, full-screen and inactive windows are conditions, so the corners and shadow a tiled window drops are
rules. The caption buttons' order and side are read where the platform has a setting (GNOME's
`button-layout`, KDE's `kwinrc`).

### R14 — Parts and controls (C13, C24)

A widget gains the parts the map names (a field's helper text and counter, a tab's close button, a
slider's ticks, a sheet's grabber, a selection pill).

Controls the model lacks: the map's §4.2 lists 118 kinds. A kind is portable when two or more platforms
draw it, counted with the cross-check decisions that found it drawn inside an element the model has (every
platform's list draws a tree, though only Windows lists a tree view as a control).

- *A typed struct per kind.* Against: structs each used by one preset.
- *Typed for kinds two or more platforms draw (a struct of its own, or parts of the element it is drawn
  in); keyed components for the rest,* named components of named parts from the same blocks, declared per
  preset (R2.4).
- *Only the controls the three toolkits have.* Against: the model would follow toolkits.

**Decision:** the second (principle 4). A keyed component that later appears on a second platform becomes
typed (breaking, allowed before 1.0). Kinds out of scope by R1.1 or R1.2 are not modelled.

### R15 — Variants, control sizes and density (C09, C10)

All three are selector parts (R2.2), declared per widget (R2.4).

**Variants** (flat, outlined, filled, tonal, destructive, suggested, pill, circular, AppKit's bezel
styles, iOS's button configurations, glass): an element carries a set of variants; rules match sets and
their negations, so libadwaita's stacked classes are rules for {flat, circular}, and its "flat unless
destructive" a rule with a negated set. Names are typed where two or more platforms share the meaning
(taken from the map's C09 rows) and keyed otherwise.

**Control size and density** are two axes, because the platforms have both and combine them: a control's
size (macOS's mini to extra large, MAC-X17, set per control; iOS's `UIButton.Configuration.Size`) and a
density (Material's density scale, WinUI's compact sizing, W3-39, macOS 26's compact control metrics).
Density is tree-inherited: an element takes its ancestors' unless it states its own, as Material's density
and WinUI's compact resource scope propagate; an application-wide density is the root's. A density reaches
only the widgets that declare it (Material's density applies to some components); a widget that declares
none ignores it, whether it inherits a density or its element states one.
Size is the element's own, as AppKit's `controlSize` is. The query receives both. A size or variant the
element's widget does not declare in its context returns "not available" (R2.4): an application asking for
a small button on `adwaita` learns there is none, rather than getting an invented one.

### R16 — Layout and placement (C26)

Geometry is in part styles (padding per side, size limits) and widget values (gaps between parts,
alignment, the order of dialog buttons, a content clamp, GN-1.195). **Placement** (where a menu, popover, tooltip or dialog opens
against its anchor or window, its margins, flipping and sliding) is parameters plus a named strategy from a
closed set, each implemented once, in native-theme, as a pure function of the anchor, the element's size
and the window (no toolkit types), to its platform's source and cited (GTK's tooltip placement, GN-1.83;
the dialog rows of the seed follow-up, G24): code cannot be data, and writing each strategy once beats
writing it in three connectors. **Breakpoints:** a size class is the platform's. Where the platform
defines it by width thresholds (Material's window size classes), the preset states the thresholds among
its tokens and the connector computes the class from the window's size; where the system assigns it
(iOS's `UITraitCollection`, set "based on the device type, window configuration, and multitasking state",
X-SV-1), the system's class is read and passed in. Either way it is passed in at the root, and a
container with its own (an iPad split view's column) passes its own to its subtree; rules select on it through the size part of the selector (R2.2); a breakpoint one widget owns is its value (AdwDialog becomes a bottom
sheet at a width of 450 or a height of 360 or less, and the bottom sheet is then a variant of its
presentation that rules select on). **Right-to-left:** the toolkit mirrors the layout; the model states what
the platform does not mirror or does differently (Material keeps time-based content left to right and
mirrors question-mark icons in Arabic and Farsi but not Hebrew, MAT-FM03†; libadwaita's `:dir()` rules,
GN-FM03†), as icon-role properties (R11) and rules selecting on the inherited text direction (R2.2); the
placement strategies take the direction as an input.

### R17 — Timings, behaviours and the pointer (C21, C22)

Timings and behaviours that apply everywhere form a behaviour record among the tokens: double-click time and
distance, drag threshold, hover-open delay, key repeat, scroll distance per step, and whether the pointer
hides while typing. Those that belong to one widget are its values: when its focus becomes visible (Breeze
after keyboard focus, K3.20; macOS text fields on any focus, its buttons only with Full Keyboard Access);
the tooltip's show, hide and reshow beside today's `tooltip.show_delay_ms`; the menu's submenu delay and
sloppy-close timeout (K-RV2†); the scroll bar's auto-hide and what a click in its trough does (K-RV1†,
MAC-P3-2†); a toast's duration. Each is read where the platform has a setting and stated otherwise. The
pointer: the cursor shape per part, the cursor theme and size (read), and iPadOS's pointer effects (X-PH-1),
which are hover rules with a transform.

### R18 — Readers (C25)

Every value the map marks readable is read by the platform's reader where native-theme has one; the preset
states the default the reader replaces; the specification lists the new reads, and today's wrong reads are
fixed with them (KDE's title bar from `[WM]` instead of `[Colors:Header]`, docs/todo.md, *Values the v0.7.1
gap map found against their sources*; GNOME's dropped font list, GN-FM02†). **A reader states each value
under the conditions it was read in:** a value read under the base conditions goes into its bases, one read
under a condition (the macOS appearance with increased contrast, Windows' system colours under high
contrast) into a rule under that condition, so no preset rule applies on top of it twice. It also supplies
the system's conditions (R2.2).

iOS and Android have no reader, and native-theme no supported, tested target there (it compiles for
iOS); an application drawn with iced or egui
can run on both through winit, and without a reader would ignore the user's text size, contrast and colours
(Android's dynamic colour, MAT-X17).

- *Readers for both in the same release.* Against: two new platforms to build and test, which is platform
  support, not model completeness, and would hold the model back.
- *Neither, ever.* Against: principle 1.
- *Both, as a platform-support milestone after the model release,* with the iOS icon loader (X-IC-7). For:
  the model already carries what they will read; meanwhile the application names the preset and its
  conditions (R1.5).

**Decision:** the third, the same for both; their R rows map to that milestone (R24).

### R19 — Palettes (community colour schemes)

A community preset (Catppuccin, Dracula, Nord, …) states every field today. With this model it would need
shadows, motion, materials and states its project never defined, which principle 3 forbids.

- *State them anyway.* Against: invented values.
- *A fixed platform base per community preset.* Against: arbitrary.
- *A palette mapped from the project's style guide.* Against: guides do not give one colour per role
  (Catppuccin's: "Overlay 2 20% - 30% Opacity" for the selection, "Crust, Mantle" for secondary panes), so
  mapping them is choosing values; and not every project has a guide.
- *A palette taken from the project's own port for that platform* (its KDE colour scheme, its libadwaita
  colour overrides, …), converted mechanically. For: the project states the platform mapping itself; what
  the port leaves to the platform is the project's look on that platform. Against: a project without a port
  for a platform has no palette there.

**Decision:** the fourth. A palette is an application layer (R2.3) of colour-role values, the colour
tokens its port states (KDE's effect parameters), and colour-role rules, marked as replacing the reader's
colour scheme; material tints that refer to colour roles (Mica's tint, the window background) follow it; there is no separate palette layer in the order.
"Palette" names it in the API and documents, so that "preset" keeps meaning a whole platform theme. A
palette:

- is offered on a platform only where its project publishes a port for it; which ports exist is the
  specification's survey, and today's community presets keep only what a port states;
- is one port file: Catppuccin's Mocha with its Mauve accent is one palette, as its port is one file;
- replaces the reader's colour scheme, the effect parameters and the user's accent included (a KDE port's
  `.colors` file states its own colour effects and accent, which become the palette's): the reader's colour
  values are not applied, so a user's own scheme cannot leak into it, and the user's accent is not mixed
  into a scheme whose port chose its own (a combination neither the port nor the platform states); the
  reader's other values still apply;
- is made of bases (its colour roles and effect parameters) and of rules only where its port states a
  condition, so the platform's condition rules, such as KDE's inactive effects, apply over it as over the
  user's own scheme; an application changing the accent does so in its bases too (R5.3);
- is not applied while either contrast condition holds, unless its port states colours for that condition,
  because the preset's contrast colours are then the user's choice (R2.3); on KDE, which has no contrast
  condition, a high-contrast colour scheme is a scheme like any other, and applying a palette replaces it,
  as applying any palette replaces the user's scheme: that is the application's choice to offer;
- leaves to the platform what its port leaves (its derivations follow the palette's colours: a libadwaita
  hover is the foreground at an opacity, whatever the foreground is);
- states which colour schemes it has; applying it under a scheme it lacks is refused by the API (a dark-only
  palette does not invent a light one); an application that wants a dark-only palette in a light system
  sets the colour scheme to dark itself (R2.2), and when the system's scheme changes under an applied
  palette the API reports that the palette no longer applies, and the theme is then the preset with the
  reader's values, rather than a mix of the palette with the other scheme;
- carries no icon set unless its port does (today's `icon_set = "lucide"` goes).

### R20 — Value kinds, and no model inheritance

Today a `soft_option` `None` has four meanings and ~91 inheritance rules fill unset fields beside the
presets (§1).

- *Keep inheritance as base-layer default derivations.* Against: a second mechanism beside the rules, needing
  a place in the order; it is how `[wrong_safety_nets]` exist.
- *No model inheritance: a preset states every reference in its bases as a derivation* (`= the bound surface's
  foreground`), and the gate checks it. For: one mechanism; the reference is visible where it applies.
  Against: longer presets; today's rules are converted once, mechanically, into the presets.

**Decision:** no model inheritance. Tree inheritance (role bindings, density, size classes) is not between
fields but from an element's containers at run time, and stays. A value is one of:

- *stated:* a value or a derivation (R5.3), references included;
- *automatic,* where the platform sizes by content (stated, so a toolkit sizes by content because the platform
  does);
- *absent:* the platform does not draw the thing; it resolves to none and a connector draws nothing. A field
  may be absent only where the model marks it so;
- *missing:* none of these, an error at resolution.

"Draw another value instead" becomes stated values (where a platform draws a checked button with its pressed
fill, its {checked} rule states that fill); "no published number" is not a kind: it is measured.

### R21 — Compatibility and release

The model's public API changes and the seven crates are released together, so the release is **0.8.0**,
carrying the connectors moved to the new model (each mapping what its toolkit can draw and recording the rest,
R23); the connectors' feature milestones stay later.

**`#[non_exhaustive]`:** every model struct gets it, so a later field is not breaking; code outside the crate
builds a struct from `default()` and field assignment (the connectors, showcases and examples change
accordingly). Enums a connector must map in full do not get it, and `IconRole` loses it: state flags,
conditions, value kinds, icon roles, blend modes, timing kinds, glyph sources, placement strategies, transition
kinds. A new variant then breaks the connectors' builds, which before 1.0 is the check that they map it,
rather than a wildcard arm that hides it. Enums with keyed counterparts are typed sets whose keyed side carries
what is platform-specific (material identities are keyed per platform, R6), so they need no wildcard either.

- *Release in stages.* Against: the rules touch every widget at once, each stage would break the connectors
  again, and a release with some capabilities filled shows mixed looks.
- *One release.* Against: a long branch.

**Decision:** one release. v0.7.1 is this milestone's documents; the implementation runs on its own branch
(`v0.8.0-rc1`), staged by the specification's plan, and nothing is released until every platform preset,
`ios-26` included, is complete and passes the facts gate (R4).

### R22 — The documents that follow the model

- **Platform facts become data.** R4's gate must parse a value per key, and `docs/platform-facts.md` is prose.
  The facts move into per-platform, per-area data files (a key per model path and selector, the value or
  derivation, the source kind and the citation), and the readable tables are generated from them.
  `scripts/check_platform_facts_rows.py` and `scripts/check_preset_citations.py` move to the data. iOS gets its
  columns (13–18 and 26). Keeping the prose and splitting it was rejected: a test cannot read it.
- **`docs/property-registry.toml` and `docs/inheritance-rules.toml`:** the registry stays, checked against the
  code by its drift test extended to the new types; the inheritance rules go with inheritance (R20).
- **The showcases' gates** (element lists, parity) follow the connectors in R21.

### R23 — A value a toolkit cannot draw

A connector maps every value its toolkit can draw and lists every value its toolkit cannot in its contract,
with the toolkit's lines that show it; it also builds what the query needs (the element's ancestor chain and
siblings, R2.3) where its toolkit holds none, and supplies the window's conditions (R2.2). A toolkit's limit is
recorded where it is; the model still carries the value. This is the inverse of the failure that started this
milestone, where the model's limit was recorded as the toolkit's.

### R24 — Traceability

The specification maps every row of the map, identified by its file and ID, to the fields, rules, roles or
components that carry it, or to the decision that places it elsewhere: out of scope (R1.1–R1.4), another
preset (R1.3), the newest release (R1.5), the readers' milestone (R18). The rows are the platform files' gap
rows; withdrawn rows and the 42 pointer rows are excluded, and the cross-check and pass decision tables are
decisions, not rows. A script checks that every row maps to at least one target. This proves the model carries
everything the map found; what the map, a survey, did not find is caught by R4's gate, whose lists come from
the platforms' own state models.

## 4. What the specification decides

The names and types of every field; the colour roles' surface sets, role record and keyed roles; the text
ramp's slots; the semantic variants, sizes and densities; the condition, context, position and state lists in
full, and the exclusive groups; the selector and patch types and the TOML spelling of rules, derivations,
references and value kinds; the derive macro's changes; the resolution and query API and the tree a connector
builds; the placement strategies; the derivation functions with their sources; the facts' data format and the
gate's test; the conversion of today's inheritance rules into the presets; and the mapping of every row (R24).
Its first steps are the kirigami-addons survey (R1.2) and the survey of the community projects' platform ports
(R19).

## 5. Check record

- **2026-10-28, author, passes 1–2** (first draft): numbering, the colour count, R14's scope cut, R18's iOS
  claim, R21's `ios-26`, right-to-left, Liquid Glass's refraction, unverified variant names, per-widget
  timings, cited row IDs.
- **2026-10-28, independent check 1:** 9 serious and 16 should-fix findings, all accepted; rewritten around
  one rule mechanism (R2) and a facts gate (R4).
- **2026-10-28, author, pass 3:** resolution out of stated order; combinations taken from the map; an
  unverified WinUI timing; R20's derivation from another state; a walk-through asking `adwaita` for a size.
- **2026-10-28, independent check 2:** check 1's findings 25 resolved, 7 mostly, 3 partly; 6 serious and 18
  should-fix new ones, all accepted: facts as data, no model inheritance, palettes from ports, light and dark
  as a condition, appended paint layers, declarations and validation, transforms, negation, exclusive groups,
  checked against selected, and the rest.
- **2026-10-28, author, pass 4:** inactive and disabled groups made consistent; why the colour scheme is a base
  and contrast a rule; a port's existence no longer asserted.
- **2026-10-28, independent check 3:** check 2's findings 29 resolved, 4 mostly, 2 partly, none open; 4 serious,
  15 should-fix and 14 minor new ones, all accepted (check 4 found four of them only partly resolved;
  the revision after check 4 finished them). Serious: rules could not reach widget
  values or tokens (R2.2's targets); nothing matched siblings (R2.2's position); R9's transition direction was
  false (the style after the change governs both ways, as CSS and GTK); R2.3's accessibility-last pass was
  false to the platforms and overrode a preset's own state rules (one order, the palette suspended under high
  contrast), and the material fallback had no trigger (R6). Should fix: "palette" in two senses (colour roles
  and palettes), a KDE port's own effects (R19), resolution per window (R2.3), KDE's groups from the active
  roles (R5.1), subtree role bindings (R5.1), per-widget focus visibility and focus within (R3, R17), token
  references and the focus indicator as a block (R2.1, R12), `foreground` defined by the bound surface (R5.3),
  accent inputs (R2.4, R5.3), the elevated level and per-column size classes (R5.1, R16), closed platforms in
  the gate (R4), size and density as two axes (R15), application rules per preset (R2.2), GNOME's two motion
  settings (R2.2, R9), iOS glass over dark content (R3). Minor: premultiplied `mix`, citations, the iOS
  generation before its reader, who supplies each condition, declarations completed, size per context, R1.6's
  test, the enums' side, "inheritance" in two senses, the palette as an application layer, K16's verdict, rows
  identified by file.
- **2026-10-28, author, pass 5 (after the revision):** R2.3 had come to interleave each source's
  bases and rules, so a reader's base would undo a preset's condition rule; bases first, then rules,
  restored. Walk-throughs of the two cases check 3 left open.
  (a) An iOS 26 glass button in a navigation bar, pressed, with Increase Contrast and Reduce Transparency, in
  a sheet: the conditions come from the application until R18's milestone (R1.5); the sheet binds its
  descendants' backgrounds to the elevated roles (R5.1); the glass capsule it shares with grouped bar items is a
  position rule (R2.2); a large size, if the bar item does not declare it, returns "not available" (R2.4, R15);
  the glass material's reduced-transparency fallback and the increased-contrast look are the preset's rules in
  its own order (R2.3, R6), and the pressed look is a {pressed} rule with its transform; the label's colour over
  the glass follows the "over dark content" flag (R3). (b) A libadwaita entry row in a boxed list, inside a
  dialog presented as a bottom sheet on a narrow window, keyboard-focused and invalid, under reduced motion,
  with an application layer changing the accent (in its bases, R19): the row's corners are position rules (first, last, only); the
  dialog's presentation is a widget value chosen by its breakpoint, and the bottom sheet a variant rules select
  on (R16); reduced motion swaps the sheet's slide for libadwaita's crossfade by a rule on the dialog's
  presentation transitions (R9); {focus visible, invalid} select the entry row's rules, and invalid binds the
  accent to the error colour for the row's subtree (R5.1), so the focus indicator and the caret follow without
  restating; the application's accent base sets the declared accent input (R5.3), and every derived
  role follows. Every step is decided; R4's gate compares the results with libadwaita's facts.
- **2026-10-28, independent check 4:** check 3's findings 28 resolved (10 with a remainder), 4 partly,
  none open; every code claim and all 111 cited row IDs true but two; nothing overcomplicated. 4 serious,
  9 should-fix and 13 minor new ones, all accepted (check 5 found three only partly resolved and one,
  the sidebar size, wrongly resolved; later revisions finished them): density against "not available" (a
  density reaches only the widgets that declare it, R15); size classes without a selector part (R2.2's size
  part); vibrant labels and icons without a paint (text, icon and glyph fills are paints, R2.1, R11); AppKit's
  compiled glyphs without a source (the system's own artwork, drawn at run time, R11); palettes against the
  preset's increased-contrast colours, the user's accent and per-accent ports (R2.3, R19); the application's
  colour scheme (R2.2, R19); KDE's groups from the active values and its stated Inactive values (R5.1, R5.3);
  a binding's reach and the root surface (R5.1); macOS's unpublished accent derivation and its highlight
  colour (R5.3); previous and next siblings (R2.2); a user setting that picks a container's size (R2.3); a
  false egui example (R3); the reduced-transparency look against the material fallback (R6); and the minor
  ones (gaps in one place, Apple's "palette", the macOS window conditions, Catalyst's two looks, animations
  off and the speed factor, XAML's transition pairs, the elevated level in multitasking, the probe's style
  and size axes, the OKLab standalone accent, "not available" as a result, the iOS target).
- **2026-10-28, author, pass 6 (after the revision):** the macOS 15 sidebar case check 4 walked through is
  decided at every step now: the row's size follows the user's sidebar icon size (as revised after check
  5, a reader-supplied condition the sidebar's rules select on, R2.2); the disclosure triangle is AppKit's
  own rendering of a disclosure button (as revised after checks 5 and 6, R11), turning
  by a transform with the transition of the style entered (R9); a dark-only palette in a light system is
  refused unless the application sets the scheme to dark (R19), and under Increase Contrast it is not
  applied, so the preset's increased-contrast colours and the reader's colours read under that condition
  give the look (R2.3, R18).
- **2026-10-28, independent check 5:** check 4's 26 findings 22 resolved, 3 partly, 1 not; 3 serious, 9
  should-fix and 5 minor new ones, all accepted (check 6 found six only partly resolved; the revision
  after check 6 finished them): the sidebar size set by the reader as a
  tree-inherited value contradicted size being the element's own (now a reader-supplied condition,
  R2.2); AppKit names no image for the checkbox mark or the pop-up arrows, so "drawn like a symbol" could
  not be implemented (a named system image where AppKit has one, the system's rendering of the whole
  control otherwise, R11); token and role derivations were evaluated once per window, so a subtree that
  rebinds the accent would not change them (expressions are evaluated where used, R2.3). Should fix:
  palettes' effect parameters, bases and port-stated contrast colours, KDE's high-contrast scheme (R19);
  an application's accent and palette colours as bases (R19); KDE's stated Inactive values as inputs and
  "active value" defined (R5.1); precedence in exclusive groups (R3); iOS's tint-dependent values before
  its reader (R4); Material's scheme function inputs and the two senses of "expressive" (R5.3); the ripple
  (R3, R9); size-class thresholds and who computes the class (R16); material-web's `latest` directory
  and the reference implementations (R1.6). Minor: the status line, the check-4 overstatement, an element's
  own undeclared density, the theme after a palette stops applying, and why a palette replaces the user's
  accent (R15, R19).
- **2026-10-28, independent check 6:** check 5's 17 findings 11 resolved, 6 partly, none open; nothing
  overcomplicated. 3 serious, 10 should-fix and 9 minor new ones, all accepted and resolved: what happens
  when the theme itself changes had no home (Appendix B's kind K57; the theme-change transition, R9);
  iOS's size classes are assigned by the system, not computed from widths (R16); AppKit names no
  down-facing disclosure triangle (the system's rendering of the disclosure button, R11). Should fix:
  touch input as a condition (R2.2); what the whole-control rendering replaces, its accent, and why
  measuring glyph geometry was rejected (R11); a binding's reference to the role it binds (R2.3, R2.5);
  re-resolution on reader and layer changes and a failing layer (R2.3); text direction as inherited and
  as a placement input (R2.2, R16); appearing layers, material crossfades, the press point and the
  ripple's release (R9); a KDE port's Inactive values as inputs (R5.1, R19); the two Material token sets
  per flavour and Compose as the reference of last resort (R1.6); Material's scheme inputs (R5.3); the
  application standing in for a missing reader (R1.5). Minor: the check-5 overstatement; colour tokens
  in a palette; the inactive input's base; battery saver and the full condition list; an open list of
  presentation kinds (WinUI's clip reveal); iOS's tint inputs; material tints referring to roles; the
  sidebar size against an explicit row size style.
