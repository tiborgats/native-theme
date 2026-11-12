# v0.7.1 — Model completeness: Rationale

**Status:** design, drafted 2026-10-28 from the gap map; checked eight times independently (§5), then amended
in place during the specification's thirteen checks; done with the specification, 2026-10-29, and amended in place
since by the 0.8.0 plan's decisions D12 and D13 and by the specification's batches, the last ADD-A147 (the outcomes
of the plan's checkpoint D6, decided 2026-11-10, applied 2026-11-12).
Every amendment is noted under R25.13, in the order made. Input: [`todo_v0.7.1_model-completeness-gap-map.md`](todo_v0.7.1_model-completeness-gap-map.md)
and its six platform files ("the map"; a row is identified by its file and ID, and cited here by ID,
with its file where the prefix does not say it). Next: the specification,
`todo_v0.7.1_model-completeness-spec.md`, which states every type, field and rule decided here and
maps every row of the map to them (R24).

The maintainer's rule (2026-10-27): native-theme covers **everything a platform draws visibly**; a
value no document states is measured on the platform; no gap is closed by a toolkit's default. The
map lists 2,339 rows of what the six platforms draw that the model cannot express. This document
decides how the model expresses them: the model, its presets and its readers. The connectors move to
the new model in the same release (R21); their own milestones (ROADMAP.md, *Later — the connectors*)
follow. Values are collected after the specification, because what to measure depends on the shape
decided here (the map's §3).

Each decision gives the options, their arguments for and against, and the choice with its reason.

## Contents and glossary

| § | Decision | Map capabilities |
|---|---|---|
| R1 | Scope: what the system draws, libraries, other toolkits, SwiftUI, versions, flavours | — |
| R2 | The model's shape: blocks, parts and widgets, rules and conditions (theme, window, accessibility, touch), order, resolution and query, inputs, declarations, validation | all; C06, C23 here |
| R3 | State flags | C07, C08 |
| R4 | The facts gate | — |
| R5 | Colours: roles, bindings, translucency, derivations, representation | C04, C05, C08, C28 |
| R6 | Paint: layers, gradients, materials | C03, C08, C27 |
| R7 | Corners and stroke | C11, C12 |
| R8 | Shadows and elevation | C01, C02 |
| R9 | Motion and transforms; theme changes | C08, C14 |
| R10 | Typography | C18 |
| R11 | Icons and glyphs | C13, C19 |
| R12 | Focus, caret, selection | C15, C16, C17 |
| R13 | Window chrome | C20 |
| R14 | Parts and controls | C13, C24 |
| R15 | Variants, sizes, density | C09, C10 |
| R16 | Layout, placement, breakpoints, direction | C26 |
| R17 | Timings, behaviours, the pointer | C21, C22 |
| R18 | Readers | C25 |
| R19 | Palettes | — |
| R20 | Value kinds; no model inheritance | — |
| R21 | Compatibility and release | — |
| R22 | The documents that follow | — |
| R23 | A value a toolkit cannot draw | — |
| R24 | Traceability | — |
| R25 | Decisions the specification's work required | C05, C10, C11, C14, C18, C19, C26, C27 |

- **Part:** a named piece of a widget (a button's body, label, icon; a switch's track and thumb).
- **Widget:** a typed set of parts plus widget-level values; one per control kind the model types.
- **Element:** one drawn instance of a widget or of a part that has its own interaction state (a tab;
  the tab's close button is an element of its own, whose ancestor is the tab). Queries are per element.
- **Base:** a value as stated before any rule applies.
- **Rule:** a typed selector and a typed patch; the one mechanism that changes values (R2.3).
- **Condition:** a selector part that holds for a whole window or theme (R2.3).
- **Colour role:** a named colour of the model (R5.1). **Token:** any other named value a preset states
  once for many parts (motion, elevation, text ramp, icon sizes, behaviour, KDE's colour-effect
  parameters).
- **Input:** a value from outside the preset that derived values follow (the user's accent, the
  scheme's stated Inactive values, Material's seed; R2.6).
- **Binding:** an element setting a colour role for itself and its descendants (R5.2).
- **Tree inheritance:** a value an element takes from its ancestors at run time (a binding, a density, a
  size class, the text direction). There is no model inheritance between fields (R20).
- **Layer:** a source of bases and rules: the preset, the reader, an application layer (R2.4).
- **Palette:** a community colour scheme, applied as an application layer (R19).
- **Flavour:** one of two current looks of one platform in one preset (R1.6).
- **Facts gate:** the test comparing every resolved value with the platform's facts (R4).

## 1. The model today

Read at caeddd6b:

- **A widget is a flat struct of `Option` fields** (`native-theme/src/model/widgets/mod.rs`):
  `ButtonTheme` has `background_color`, `hover_background`, `active_background`,
  `disabled_background`, `checked_background`, `hover_text_color`, … each its own field, with a nested
  `font: Option<FontSpec>` and `border: Option<WidgetBorderSpec>`.
- **`#[derive(ThemeWidget)]`** (`native-theme-derive`) generates each struct's resolved twin (no
  `Option`), its merge, emptiness test, validation, range checks and inheritance rules. A field is
  `option` (resolution fails while it is unset) or `soft_option` (it may stay `None`). A `soft_option`
  `None` means one of several things today: "draw another value instead" (`button.checked_background`:
  the pressed pair), "sized by content", "the platform draws none", or "no published number".
- **Resolution** (`native-theme/src/resolve/mod.rs`) fills unset fields from defaults and related
  widgets ("~91 inheritance rules" by the module's own count, documented in
  `docs/inheritance-rules.toml`, whose `[wrong_safety_nets]` lists rules known to give wrong values),
  fills the run-time-detected fields, and validates into `ResolvedTheme`.
- **Presets** are TOML (`native-theme/src/presets/`), a full light and a full dark `ThemeMode` each:
  geometry and fonts are stated twice. A reader returns a partial `ThemeMode` merged over its platform
  preset (chosen by OS version, `pipeline::preset_for_version`); an application can merge its own
  (`SystemTheme::with_overlay`) and choose light or dark (`SystemTheme::pick`).
- **Community presets** (Catppuccin, Dracula, Nord, …; 497 lines each) state every field themselves,
  geometry and an icon set (`lucide`) included.
- **Colours** are `Rgba`: four 8-bit sRGB channels; the macOS reader converts system colours to sRGB
  (`macos.rs`), clipping what lies outside it. **Borders** carry a colour, a width, a corner radius (on
  `defaults` also a large radius and an opacity), padding per side and `shadow_enabled`; `defaults`
  carries one `shadow_color`. **Fonts** carry family, size, weight, style and colour; line heights are on
  `defaults` and the text-scale entries.
- **Motion** is `dialog.entrance` and the animated icons (`AnimatedIcon`: frame lists and transforms,
  which carry the spinners).
- **Accessibility** is four run-time flags on `AccessibilityPreferences` (`text_scaling_factor`,
  `reduce_motion`, `high_contrast`, `reduce_transparency`), with no preset values behind them; under
  `high_contrast` the Windows reader writes the system colours and the GNOME reader the focus ring into the theme,
  the macOS reader's colours follow Increase Contrast, and the connectors honour `reduce_motion` for the busy
  indicator and their own animations (ADD-A147; D6, decided by logic, awaiting the maintainer's veto: veto list item
  40).
- **The `Option` structs are not `#[non_exhaustive]`** (docs/todo.md, *Dialog: the platform's vertical
  placement*), so a new field breaks struct literals; the resolved structs and some enums (`IconRole`,
  `IconSet`) are.

## 2. Principles

1. **What an application draws, the model carries.** A visible property of something an application
   or its toolkit draws, or configures for the system to draw, has a place in the model.
2. **One mechanism per job.** The map groups its rows by mechanism (C01–C28); each mechanism is designed
   once and used by every widget that needs it, and two mechanisms never do one job.
3. **The model invents no value.** A value is a platform's: read, published or measured. Where a
   platform derives one value from another, the model carries the derivation, not its result over one
   background. Every value a preset resolves to is checked against the platform's facts (R4).
4. **Typed where a portable application needs it, keyed where only one platform has it.** Both forms are
   built from the same blocks, declared, and validated the same way.

## 3. Decisions

### R1 — Scope

Each scope question is settled by the model's purpose: to let an application's toolkit draw what the
application shows.

**R1.1 — What the system draws.**

- *Include everything the system draws.* Against: what only the system ever draws (the on-screen
  keyboard, a notification banner) has no consumer, and the system draws it natively.
- *Exclude everything another process or a system library draws.* Against: a toolkit that draws its own
  window frame draws its own window menu (GTK's fallback, GN-P3-3†); an Android application configures
  its system bars and its splash screen (MAT-W1, MAT-P3-6†).
- *In: what the application or its toolkit draws; what a toolkit draws in place of the system's when it
  draws its own chrome or its own equivalent (an in-window toast, its own file list); what the
  application configures for the system to draw with; and what the system leaves to the toolkit to act
  on. Out: what only the system draws or applies, whatever the toolkit does.*

**Decision:** the third. In: the window frame, title bar, window menu and shadow wherever a toolkit can
draw them; an Android application's system-bar and splash values; the IME pre-edit text; moving content
out of the on-screen keyboard's way, which the toolkit does (Appendix B's kind K16); hiding the pointer
while typing, which the toolkit asks for (R17); Win32's global effect switches, which a toolkit honours
(W3-53†). Out: the on-screen keyboard itself, notification banners, home-screen widgets, Live
Activities, Control Center, a shell's tiling preview and Snap layouts, an input method's own candidate
window, system dialogs a toolkit only invokes, which another process draws (GNOME's print and app-chooser dialogs,
shown through the portal by xdg-desktop-portal-gnome, GN-P4-3†; the portal file dialog drawn by Files), and what the windowing system applies to every toolkit without its involvement (the display
scale and safe-area insets the windowing library reports, whole-screen colour filters and Smart Invert;
Appendix B's kinds K17, K23 and K27, and X-A11-8). A dialog the toolkit draws in the application's process (Qt's
`QPrintDialog` and `QPageSetupDialog`, KIO's Open With dialog, K-P3-9†) is in: it is built of the toolkit's widgets,
and what it draws of its own (the page setup's page preview) is carried as any part is. Each row of the map and each kind of its Appendix B
gets its verdict by this test in the specification's traceability (R24).

**R1.2 — Libraries beside the toolkit.** Test: maintained by the platform's own project (GNOME, KDE,
Microsoft, Apple, Google's Material team) and released as stable.

- *The toolkit only.* Against: GNOME Text Editor's and Kate's code views, KDE's form cards and Windows'
  settings cards are the platform's look to its users.
- *Every library any platform application uses.* Against: a third-party library carries its vendor's
  look (KDDockWidgets draws KDAB's own drop arrows; the seed follow-up, G19).

**Decision:** the test. In: GtkSourceView, libspelling, libpanel, kirigami-addons, KTextEditor, KSyntaxHighlighting,
KQuickCharts, the Windows Community Toolkit, Swift Charts. Out: the Toolkit Labs (experimental; its
`MarkdownTextBlock` rows wait for a Toolkit release) and KDDockWidgets. kirigami-addons was surveyed
during the specification (the KDE map's *kirigami-addons* section, K-KA-1† … K-KA-47†).

**R1.3 — Other toolkits on a platform** (GTK 3 applications on GNOME; WPF and Win32 controls on
Windows). A platform preset describes the platform's current native toolkit; the others are candidate
presets of their own (ROADMAP.md, *Beyond v0.7*). One preset holding two toolkits' looks for one widget
would make the application choose which to copy, which is what choosing a preset already does. Rows:
GN-P3-17†, W-P3-15†.

**R1.4 — SwiftUI and Mac Catalyst.** AppKit and UIKit are the reference where SwiftUI draws a control
through them; a control only SwiftUI has (`Gauge`, the grouped `Form`) is a control of the platform
(R14). A Mac Catalyst application draws either the iOS look or, with the Mac idiom, AppKit-style
controls, both of which the `ios-*` and `macos-*` presets cover; its Catalyst-only rows are out.

**R1.5 — Versions inside a generation, and choosing the generation.** A preset describes the newest
release of its generation that a supported runner measures (macOS 15 for `macos-monterey`, libadwaita 1.10 for
`adwaita`, iOS 18 for `ios-13`, macOS 26 for `macos-tahoe` and iOS 26 for `ios-26` while GitHub's macOS 27 image is a
preview, Material 3 as its current token set states it for `material-3`); readers read what the running
release states; a release whose look differs as a whole starts a new generation and preset, as macOS 26
did. A per-release overlay inside a preset was rejected: it multiplies values by releases for
differences the readers already read where they can. The generation is chosen by the running OS
version, as `pipeline::preset_for_version` does now, and by the application's own opt-out where a
platform has one (iOS's `UIDesignRequiresCompatibility`, X-MT-13). On iOS and Android, where
native-theme has no reader and no supported, tested target until R18's milestone, the application names
the preset and supplies the inputs and conditions (R2.6).

**R1.6 — Two current looks on one platform.** KDE draws Qt Widgets and Kirigami applications differently
in places (the focus indicator, K3.20 against Q3.30); Material 3 has its baseline and its Expressive look.

- *Two presets each.* Against: they share most values, stated twice.
- *Pick one.* Against: the other current, widely used look is left out.
- *One preset with a flavour,* a condition the application chooses (R2.3), the default stated.

**Decision:** flavours. `kde-breeze`: `widgets` (default; most KDE applications) and `kirigami`;
`material-3`: `baseline` (default) and `expressive`. `material-3` describes the Material 3 design system
as Google currently publishes it, the published token set, which Android's own Material 3 (Compose) draws save
where the facts record otherwise: its token set
`tokens/versions/latest/` in material-web, at a pinned commit, which holds the baseline and the
Expressive tokens and agrees with Compose (the pressed state layer's opacity is 0.1 in both, where the
older `v0_192/` set says 0.12, which the map's MAT-X8 records as web against Compose). Where the tokens are
silent, Compose Material 3 is the reference, cited at a pinned commit, experimental or not (for some
Expressive behaviour it is Google's only implementation); material-web's components, built on the older
token set, only where Compose has no such component. Where the tokens and Compose disagree, the tokens decide
(ADD-A126): a look the tokens state that Compose does not draw stays, as the published system is the preset's
reference and Compose's omission no statement (the filled text field's and select's hover layer,
`_md-comp-filled-text-field.scss:141`, `:143`, MAT-I11), and a motion no token states is Compose's (its engines,
its interruptions, the switch handle's spring, MAT-SW8), with no transition where no source times a look the tokens
add. Weighed: following Android's drawing wherever it departs from the tokens (fidelity to the one implementation
users meet, but the preset would no longer be the published design system, and the tokens' agreement with Compose,
the reason for choosing them, would decide nothing); the tokens alone, Compose never (no engines, no motion for most
transitions). A look the tokens add that Compose lacks takes its states from material-web's component, which
the tokens were written for, where it states them (the filled field's layer shown only on an enabled field,
`material-web@2.5.0 field/internal/_filled-field.scss:71-73`).

### R2 — The model's shape

The map needs values per part; per state and combination of states; per condition and combination of
conditions; per container (libadwaita's icon buttons are flat inside a header bar unless raised,
suggested, destructive or opaque, GN-1.151; Kirigami's colour set follows modality, Q1.53); per position
among siblings (a boxed list rounds its first and last rows, GN-1.165; linked buttons, GN-1.42; segment
ends, MAC-SG08); per variant, where variants stack (libadwaita's style classes); per control size; plus
shadows, motion and materials, which today have no type.

- **A. More flat fields on today's structs.** For: no new concepts. Against: every combination is a field;
  one button body (fill, text colour, stroke, shadow, icon colour, corners, opacity) times nine state
  combinations is some 60 fields per part before conditions, containers and variants, most repeating
  another value.
- **B. A string-keyed style-rule engine** (CSS-like). For: compact; GTK's themes are built this way.
  Against: untyped, so the compiler checks nothing and a typo is a silent miss.
- **C. Typed blocks and parts with separate mechanisms** for states, conditions, variants, sizes and
  palettes. Against: several mechanisms merging partial looks with no order between them; none handles
  containers, siblings or combined conditions.
- **D. Typed blocks and parts, and one typed rule mechanism.** A rule is a typed selector and a typed
  patch: *when* these conditions hold, inside this container, at this position, with these variants, at
  this size, in these states, *set* these values. For: typed throughout; one mechanism carries states,
  combinations, conditions, containers, positions, variants and sizes; it mirrors how platforms state
  their looks (GTK's CSS, WinUI's visual states and theme dictionaries; Breeze's C++ branches on the same
  inputs). Against: every widget is rewritten, and the derive macro must generate typed patches.

**Decision: D.** Pre-1.0 the project promises no compatibility; D is the only option that is both typed
and one mechanism. R2.1–R2.8 define it.

**R2.1 — Blocks** (fields are the specification's):

| Block | Carries | Capabilities | Decision |
|---|---|---|---|
| colour | an extended-sRGB colour, or a colour derivation | C04, C05, C28 | R5 |
| length | logical pixels (negative allowed where parts overlap: Breeze's tabs, `TabBar_TabOverlap`), automatic, or a derivation from the font or line height | C26 | R5.4 |
| duration | milliseconds, or a derivation (a token times the speed factor) | C14 | R9 |
| paint | a stack of layers, bottom first: colour, gradient or material, each with a blend mode | C03, C08, C27 | R6 |
| corners | a radius per corner, fully rounded, concentric; the corner curve | C11 | R7 |
| stroke | a width and paint per side; inside, centred or outside | C12 | R7 |
| shadow | layers (offset, blur, spread, colour, inset), or a reference to an elevation ramp entry | C01, C02 | R8 |
| transform | scale, rotation and translation about an origin | C13, C14 | R9 |
| transition | what changes, duration, delay, timing, and kind-specific values (a keyed kind's measured values) | C14 | R9 |
| text style | font (family, size, weight, style, width, axes, features, fallbacks), letter spacing, line height, decoration, elision and wrapping, and its fill, a paint | C18 | R10 |
| icon style, glyph | an icon's size, rendering mode, weight, effect; a glyph's source and geometry; the fill of either, a paint | C13, C19 | R11 |
| focus indicator | kind, stroke, offset, corners, shadow, transition | C15 | R12 |

A value of any block may be a **reference** to a token or a colour role; references are derivations
(R5.4).

**R2.2 — Parts, widgets, presets.** A **part style** combines the blocks: fill, stroke, corners, shadow,
opacity and transform (both applied to the element and its descendants as one group, as CSS's `opacity` and
`filter: opacity()` are: libadwaita dims an inactive window's tabs by half that way), text style, icon style or glyph, focus indicator, padding per side, size limits,
cursor (R17), the bindings it sets (R5.2), and its property transitions (R9). A **widget** is a typed set
of named parts plus widget-level values (gaps between its parts, alignment, placement, presentation
transitions, behaviour). A **preset** is its declarations (R2.7); its tokens (motion, the speed factor,
the theme-change transition, the elevation ramp, the text ramp, icon sizes, behaviour, size-class
thresholds, and the colour tokens such as KDE's effect parameters); its colour roles' bases per colour
scheme (R5.1); each part's and widget's base; and an ordered list of rules. Today's `defaults` becomes the
colour roles and the tokens.

**R2.3 — Rules and selectors.** A rule's patch sets values of one target: a widget's part, a widget's
own values, the colour roles, or the tokens. Its selector has these parts, each optional, each a set that
may also be negated (*not* these); the rule matches when every stated part holds.

- *Conditions,* window- or theme-wide, each with the source that supplies it:
  - from the system, by the reader: increased contrast, high contrast, reduced transparency, reduced
    motion, animations off (GNOME has both settings, `reduced-motion` and `enable-animations`; KDE's
    speed factor 0 sets both, the one setting standing for both, as KDE's portal and plasma-integration
    report it, the specification's S9.3, so the factor and the conditions are not two routes to one job;
    ADD-A146),
    bold text, button shapes, differentiate without colour, tablet mode (KDE, K3.83, Q3.31), the
    mobile form factor where a platform styles by it (Kirigami's `isMobile`, which turns menu hover off
    on Plasma Mobile), battery saver (Windows turns Acrylic off under it, W3-07), the user settings that choose a size for one kind
    of container (macOS's sidebar icon size, small, medium, large, MAC-SB01, for source lists at the
    default row size; a list stating its own row size style has that as a variant, MAC-L08), and the
    colour scheme (light, dark), which the application may set instead, as `SystemTheme::pick` lets it
    today. Each reader states which of its platform's settings is increased and which high contrast:
    macOS's and iOS's Increase Contrast are increased; Windows' contrast themes and GNOME's high
    contrast are high; Material's levels (MAT-X18) are standard (neither), medium (increased) and high
    (high); KDE has no contrast setting of its own (a high-contrast colour scheme is a scheme the reader
    reads). `AccessibilityPreferences`' four flags become these conditions; `text_scaling_factor` stays a
    factor (R10). The full list, with every condition the map names, is the specification's;
  - from the window, by the connector: window inactive (the window does not have the input focus: on
    macOS, not key), and, on macOS, not main and application inactive (MAC-X16); window maximised, full
    screen, or tiled with the edges it is tiled to (Breeze, GTK and xdg-shell track tiling per edge);
    iOS's elevated level where it holds for the whole window (multitasking; in a sheet or a popover it is
    a binding, R5.2); transient touch input (the last input was touch, live and application-wide:
    Kirigami's `hasTransientTouchInput` widens menu padding from 4 to 8 while it holds, Q1.28), the
    window-wide counterpart of the per-element "opened by touch" flag (R3); a game controller as the
    input (WinUI pads for it, W1-134) is a value of the same input-kind condition;
  - from the application: flavour (R1.6) and preset (an application's rule for one preset only, so that
    rules naming one preset's keyed names do not fail on another).
- *Context,* the element's ancestors: any ancestor of the named kind, or the nearest ancestor among a
  stated set of kinds ("nearest of {header bar, popover} is a header bar": flat in a header bar, raised
  again in a popover opened from it), each matched by widget, part, variants and states.
- *Position* among the element's siblings of the same widget kind by default, or of any kind or stated kinds
  where the rule says so (R25.9): first, last, only, which follow from absent previous or next siblings of that kind; and the
  previous and the next sibling's widget, variants and states (libadwaita's lone tab is a rule for
  "only", where its stylesheet uses a `.single-tab` class), for the separators that hide beside a hovered or selected neighbour
  (MAC-SG01, Q2.56†) and the shared glass capsules of grouped bar items (MAC-X07, TB-5).
- *Variants:* a set the element must have and a set it must not (libadwaita's
  `:not(.raised):not(.destructive-action)`).
- *Size:* the element's control size, density and size class (R15, R16).
- *Direction:* the element's text direction, tree-inherited, set at the root by the connector, and stated
  by an element that differs (a Material text field can keep left-to-right text inside a right-to-left
  layout).
- *States:* a set of state flags (R3), and a set that must be absent.

Rules targeting the colour roles or the tokens may use conditions only, because they are resolved per
window (R2.5); values that vary per element are set by rules on parts and widgets, which refer to the roles
and tokens.

**R2.4 — Order and layers.** Rules apply in stated order, each over what the earlier ones gave, with no
specificity arithmetic: the platforms' own orders differ, and an explicit order is the one every author can
read. The layers are the preset, the reader (R18), and the application's layers in the order the
application adds them; a palette is an application layer (R19). Every layer's bases come first, in that
order, then every layer's rules, in that order: a base after a rule would undo it (a reader's selection
colour stated after the preset's inactive-window rule would erase the inactive look).

- *Accessibility last, over every layer.* Against: false to the platforms (GTK's application stylesheet
  outranks the theme's, WinUI's local values outrank theme resources), and a preset's accessibility rules
  would override all its own state rules, so every state look would have to be restated under each
  accessibility condition.
- *One order, the application last, as on the platforms.* For: true to the platforms; a preset states its
  accessibility rules where they belong in its own order. Against: an application layer can override the
  user's accessibility look.

**Decision:** one order. Where an application overrides the platform's accessibility look, it does what the
platform lets it do; a palette is suspended under the contrast conditions (R19).

**R2.5 — Resolution and query.** **Resolution** runs per window and again when anything it reads changes:
a condition, an input (R2.6), any value a reader supplies (a scheme, a font, the text-scaling factor, the
speed factor), a palette or application layer, or the preset; its result is cached. Every such change but
a condition's is a theme change (R9); a change of conditions is one only where R9 says so. It applies the colour-role and token rules
whose conditions hold, in order, which chooses each role's and token's expression; and for each part and
widget it keeps the base and the list of rules whose conditions hold, still in stated order. Expressions are
evaluated where they are used, in the query, not where they are defined: a role or token derived from the
accent follows a subtree that rebinds the accent (libadwaita computes its focus-ring colour at each use from
the accent in scope, `_colors.scss:307`), and `foreground` means the using element's (R5.4). An application
layer that fails validation (R2.8) is refused when it is added, with the error returned to the application,
and the theme stays as it was.

A **query** asks for one part's style, or one widget's values, for one element. Its input is the element's
widget, part, variants, size and states, its own density and direction where it states them, its position
among its siblings (R2.3), its ancestor chain (each ancestor's widget, part, variants and states, and the bindings, density, size class and
direction it passes down), and its previous and next siblings' widget, variants and states. iced and egui
hold no such tree, so their connectors build one (R23). The query applies the kept rules in order, skipping
those whose context, position, variants, size, direction or states do not match, and evaluates derivations
last, because a derivation can refer to the value beneath it in this very query. Applying condition-only
rules early would put them before element rules stated ahead of them.

**R2.6 — Inputs.** An input is a value from outside the preset that derived values follow; a preset
declares its inputs (R2.7). An input is set only in bases, and has a base per colour scheme and, where the
platform states a different value, per contrast condition: macOS's and iOS's accent-dependent colours have
Increase Contrast variants (MAC-X22, D-15), which the reader reads under that condition and sets as the
input's base for it. The inputs:

- **The accent.** On most platforms the accent role. On `material-3` the seed colour (the roles follow by
  Material's scheme function, R5.4). On `windows-11` the accent and its ramp (AccentLight1–3,
  AccentDark1–3), which Microsoft derives by an algorithm it does not publish: the reader reads the ramp
  from `UISettings`, and an application or palette changing the accent states the ramp too, or the API
  refuses it. On the `macos-*` presets the accent, the colours AppKit derives from it by a derivation Apple
  does not publish (the selection, focus and accent-tinted colours, read through `NSColor`), and the
  separate highlight colour setting. On the `ios-*` presets the accent alone, UIKit's tint (ADD-A146; kept at
  D6, outcome 12, ADD-A147, open to the maintainer's veto: veto list item 42): what
  UIKit draws in the tint is stated as derivations over it, measured under the default tint and a second one, as
  the accent-following values of `macos-*` are measured under a second accent; until the host pass under the
  second tint is run, the facts hold the default tint's measured literals and their derivations are pending (the
  unmeasured register's U706). Rejected: the tint-dependent values as declared inputs, as this bullet had it (they
  are per style and state: 555 facts of the slice's button alone hold a measured default tint as a literal, values
  every application setting a tint would have to supply); the default tint's literals kept and the accent refused on `ios-*` (it
  removes the application's tint on the platform where every application chooses one). On
  `kde-breeze` the accent alone: Plasma derives the accent-dependent roles when the accent or scheme is
  applied and writes them into kdeglobals, so the reader reads them as stated values; the preset states Breeze's scheme
  values as Plasma's default ("Accent color from color scheme") writes them, with no accent derivation, and an
  application changing the accent, or a reader's or stand-in's accent (another platform's reader, the KDE reader at the
  scheme it did not read), gets each accent-dependent role its layer does not state itself by that derivation, a function (R25.1), over the scheme inputs in force, the reader's or those of a palette from a KDE port,
  which is a scheme file. A palette that is native-theme's mapping (R19) has no scheme inputs: it states its
  accent-dependent roles itself, following the accent by reference where they follow it, and is refused where it
  leaves one to the preset's Breeze value, which would mix the two schemes.
- **KDE's stated Inactive values** (`[Colors:Header][Inactive]`, K3.55): inactive input roles. The preset states
  each as Plasma's default path writes it, and an accent the application sets (or a reader's or stand-in's, for each inactive input role its layer does not state) restates
  each by Plasma's derivation
  (`kde_accent(…, inactive)`, R25.1: what `applyScheme()`
  writes for a set whose scheme states an Inactive group, that group's own values tinted toward the accent in a tinting
  scheme and, in one that does not tint, Header's as a copy of Window's active values (Plasma 6.8) and another set's as
  stated, Header's background and foreground then replaced under the accent-coloured title bar option where the
  scheme has a Header group (whether or not it states an Inactive group, as `applyScheme()` writes them either way;
  a scheme without one gets no Header written, and KColorScheme reads Header from Window); and the set's
  active value for one that states none, as KColorScheme reads it, the scheme inputs naming the sets the scheme has a
  group for, those whose group states an Inactive group and the keys it leaves unset, as the preset, the reader and a
  port each read it from their own scheme file); the reader
  replaces them with what kdeglobals states, and a palette from a KDE port with what Plasma's default path writes over
  the port's scheme (`applyScheme()`'s rules, as the preset's values follow them), not the raw file, so an accent set
  later cannot shift a non-tinting port's inactive title bar without the accent-coloured title bar option. A mapping palette has no scheme, so it states them
  itself (the project's colours, or each its set's active value, which is how KColorScheme reads a scheme without an
  Inactive group), and is refused where it leaves one to the preset's derivation over Breeze's inputs, which would
  show Breeze's colours in an inactive window under the palette's. KDE's inactive effects apply over them, as
  `kcolorscheme.cpp:381` applies them over a stated value (R5.1).
- **Material's seed, scheme variant and specification version:** the seed is the accent input; the scheme
  variant (tonal spot, … , and the variant Material also calls "expressive", unrelated to the flavour) and
  the specification version are tokens of the preset; the contrast level comes from the contrast conditions
  (R2.3), as the scheme function's contrast input (R5.1).
- **Where there is no reader** (iOS, Android until R18's milestone), an application may add a layer marked
  as standing for the reader; its values are placed as a reader's are (bases, and rules under the
  conditions they were read in, R18).

An application changing the accent, and a palette's colours, are bases of their layers; the platform's
condition rules (KDE's inactive effects, the contrast rules) then apply over them as over the user's own
colours.

**R2.7 — Declarations.** A preset declares, per widget, its variants, sizes and densities (per flavour where
they differ, and per container where the platform draws a size only in some, such as a large button outside a
navigation bar), its widget-specific state flags, exclusive state groups with their precedence and recency sets (R3), its
keyed components' declarations, the same keys as a widget's with their values' types (their parts and fields come from the
catalogue, as typed widgets' do, R14), its keyed colour roles, text styles and elevation entries, its
flavours, and its inputs (R2.6). Adding a layer checks every rule and reference against the declarations: a typo
is an error, not a silent miss. A query for a size or variant the element's widget does not declare in its
context returns "not available", a result the application handles, not a failure; a density the widget does
not declare is ignored (R15). A variant, a size, a density or a flavour the platform has and a later unit states is answered
'later': the theme does not state it yet (ADD-A147; D6, decided by logic, awaiting the maintainer's veto: veto list
item 1).

**R2.8 — Validation and infallible queries.** The project forbids runtime panics, and a query runs inside a
connector's draw call. The checks prove every rule's derivations against every base and rule they can apply
over, and every expression against every binding it can meet: types, references, cycles, and operands that
could be absent, or automatic where arithmetic would need a number (an automatic size has none). A preset or layer that fails is an error when it is added, and it is refused (R2.5); so resolution
and the query cannot fail. A bundled preset whose `[pending]` table is empty (every one at a release), a bundled
palette, and every layer `Theme` derives from them alone pass these checks in the crate's own tests at every commit
(S19.1's build test and the facts gate), and a theme runs neither the preset's own checks nor the checks over
combinations of those stacks again when it loads them. What the system and
the application add passes them when it is added: they run over every layer whose stack, as they read it, the
addition or a later change alters, and are not run where nothing they read differs from a stack that passed them, so
they run over a reader's structural reading and every layer above it, a bundled palette among them, where a change
alters what the reading leaves out (a bundled palette applied or removed over it, `set_accent` on `kde-breeze` with
the KDE reader); the checks of each
value against the declarations, and the check that no input reads itself over the stack up to each layer, run over
every layer above the preset at every change (ADD-A147; D6, decided by logic, awaiting the maintainer's veto: veto list item 4).
Rejected: the checks run again at every load, which repeats at run time what every commit already proves of the same
data (four of the slice's presets loaded in 259 ms to 17.6 s with them, against the 16 ms of the plan's *The load budget*).

### R3 — State flags

The flags, the union of what the map's platforms draw: hover, pressed, focused (has focus), focus within (a
descendant has focus), focus visible (focus shown), disabled, checked (on), selected, indeterminate, open
(expanded, menu shown), default (a dialog's default button), dragged, drop target, read-only, invalid,
visited, autofilled, scrolled (content under a bar: T-1, C-1), over dark content (iOS 26's glass flips its
look by what lies beneath, which the toolkit knows), opened by touch (WinUI sizes a menu by the input that
opened it, W1-047, W1-134, W3-40; the connector sets the flag on the element it opens), and per widget the
flags only it has (a slider's overlapping handles, MAT-S10). Checked and selected are separate because a Qt
list item carries both (`checkState`, `State_Selected`); focused and focus visible because macOS draws an
unemphasised selection without focus (MAC-I05, MAC-SB05) while most platforms show focus only after keyboard
navigation (K3.20); when focus becomes visible is a behaviour (R17). Window activity and window states are
conditions (R2.3).

**Exclusive groups.** Some platforms hold one state of a group at a time: WinUI's `CommonStates` is one of
Normal, PointerOver, Pressed and Disabled. A connector may still report flags together that such a platform
never combines (a pointer over a disabled control). A preset declares its platform's exclusive groups, each
with its flags in order of precedence (for `CommonStates`: disabled, pressed, hover), and the query keeps the
highest present and drops the others before matching, so no look is made up for a combination the platform
does not have.

**Recency** (CONTRA-6). Some platforms show the most recent of several states rather than a fixed one. Compose
keeps a control's interactions in the order they began and shows the last: its state layer over hover, focus and
drag, the press ripple drawn beside it (`androidx@e4d933c9
compose/material3/material3-ripple/src/commonMain/kotlin/androidx/compose/material3/ripple/Ripple.kt:250-280`), and a
button's elevation over hover, focus and press (`compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Button.kt:1703-1747`;
`FloatingActionButton.kt`, `ToggleButton.kt` and the split button's halves through the button's `ButtonElevation`,
`SplitButton.kt:687-1294`, likewise, `Card.kt` and `Chip.kt` with drag too). An elevated
button hovered after it was focused is at level 2, and focused after it was hovered at level 1
(`tokens/ElevatedButtonTokens.kt:43-74`), so any fixed precedence is wrong for one of the two sequences. An
exclusive group cannot state them either: it drops flags for every property, while the press, a member of the
elevation's set, is drawn beside the state layer.

- *An exclusive group with a fixed order.* Against: the two sequences above.
- *One most-recent flag per element.* Against: the press is the latest of the elevation's set yet drawn beside the
  state layer, so one element-wide answer is wrong for one of the two properties.
- *The connector drops the older flags itself.* Against: it needs the sets, which are preset data a generic
  connector does not have.
- *Named recency sets: the preset declares each, per widget, its members cited from the platform's code; a rule's
  selector may require a flag to be the latest of a set, that is kept (after the exclusive groups' drop) and set
  after every other kept flag of the set; the connector reports the order it set the element's flags in, which it
  records (element data, R25.3); flags it reports no order for count by the set's stated order, its
  first as the latest.*

**Decision:** named recency sets. The stated order is the tie-break, so a connector that reports nothing gets a
fixed precedence (Material's: focus over hover, as its View library ranks them), and the gate checks each order of
the flags present (R4). Material's state layers are rules on the latest of the set `state_layer`, not an exclusive
group: a layer rule matching only its latest flag already draws one layer. Only the fields the platform's recency
code drives follow a set (Compose's state layer and elevation); its other state-dependent fields are drawn by fixed
precedences of their own (the list item's shape, `ListItemDefaults.kt:1144-1152`), so they stay rules on plain
flags, in that order.

**State layers.** libadwaita's `currentColor` shares are CSS background layers, so paint layers a rule appends over
the fill beneath (R6), with a colour derivation (R5.4), animated as CSS animates them. Material's hover, focus and
dragged overlays and its press ripple (C08) are Compose's ripple node, which draws them apart from the container and
over the content, times their alpha by specs of its own while drawing them in the current colour every frame, and
grows and fades the ripple by its own lifecycle, while the container's colour may snap (a toggle's, a chip's, an icon
button's).

- *Paint layers with roles:* each layer names its role, transitions time a role's layers, a part holds one layer of a
  role, role layers are timed by their alpha alone and stated at rest so they never appear. Against: each rule
  compensates for the one before, and a rule that sets the fill still drops them.
- *The state layer as a part of its own.* Against: a part more on every widget that draws one, the ripple still
  sharing its fill.
- *A block of the part, `interaction`, holding a state layer (colour, alpha) and a ripple (colour, alpha, its
  geometry), drawn over the part's fill and content and beneath its stroke, set by field patches, with
  a property group of its own, `StateLayer`; the state layer's colour changes at once and its alpha alone is timed; the ripple is the
  platform's effect, keeping the colour of its first frame (S2.12).* Its cost: a field on a dozen Material parts that every
  other preset states absent.

**Decision:** the block. It holds one state layer and one ripple by its type, a rule setting the fill cannot touch it,
and its timing follows from its fields; its colours reference the content colour, `= surface.foreground`, so they
follow every rebinding with no rule restating them; Material's ripple is the block's effect, started by the press (R9), on
Android the framework's `RippleDrawable`, as Compose draws it there. Weighed for the ripple: a transition kind (a
transition with no timing of its own, started by an event rather than a value change, against R9's own rule, with
checks to fence it in) against a field of the block whose start is the flag: the field.

### R4 — The facts gate

With rules in stated order, a combination no rule states gets what the earlier rules give: the platform's own
answer where the platform composes the same way, and not necessarily elsewhere. libadwaita's flat button is
7 % on hover, 10 % checked, 13 % checked and hovered (GN-1.29), which appending the first two does not give,
so the preset must state {checked, hover}.

- *Trust the order.* Against: a forgotten combination passes silently.
- *State every combination of every flag.* Against: thousands of repeats.
- *A gate that resolves every value a platform draws and compares it with its fact.*

**Decision:** the gate, built to run mechanically:

- **Facts as data** (R22): each fact is a value (or the platform's own derivation) for a key: the model path
  and the selector it holds under, with its source kind and citation.
- **Which combinations:** per widget, from the platform's own state model: WinUI's visual states,
  libadwaita's selectors, Breeze's state branches, Material's tokens; on macOS and iOS the API's control
  states (`NSControl`, `UIControl.State`) and appearances, times the conditions, the containers the API
  distinguishes (a toolbar item, a bar item), and the styles and sizes the API enumerates
  (`NSButton.BezelStyle`, `NSControl.ControlSize`, `UIButton.Configuration`), each measured by the probe;
  where a recency set holds several present flags, each order of them (R3). The map is a survey; the platforms'
  state models are the list. At each release the gate checks the combinations whose names its enabled units hold; a
  combination holding a variant, a size, a host context, a position, a direction, a state or a condition of a unit
  not enabled is that unit's, checked from its commit on (ADD-A147; D6, decided by logic, awaiting the maintainer's
  veto: veto list item 1).
- **Derivations:** where a platform states a derivation, the fact is the derivation. The gate resolves each
  preset twice, with its own inputs and base colours and with perturbed ones (another accent, foreground and
  background), and
  compares both with the facts evaluated the same way, so a literal stated in a preset where the facts derive fails.
  On macOS the reader reads the system's colours under the user's accent, so the facts there are
  measurements under each appearance; iOS's are measured under each appearance, under the default tint and a
  second one, so that what UIKit draws in the tint is a measured derivation over the accent (R2.6), as macOS's
  accent-following values are from its second-accent pass; until the host pass under the second tint is run,
  the facts hold the default tint's literals and their derivations are pending (the unmeasured register's U706;
  ADD-A146; kept at D6, outcome 12, ADD-A147, open to the maintainer's veto: veto list item 42; the run is M2's).
  Rejected: the default tint alone until a reader exists, as this bullet had it: on iOS the
  application supplies the accent itself, reader or not (R1.5), and the measuring host reads these values under
  any tint now; and under one tint the facts hold the default tint as literals, which an application's accent
  leaves in place and the perturbed run cannot tell from a derivation.
- **What the gate proves** (ADD-A146): the presets are generated from the facts (the plan's D6 item), so the gate
  proves that every key has a fact under every listed combination of the enabled units' names (ADD-A147; D6, decided
  by logic, awaiting the maintainer's veto: veto list item 1), that the model resolves to it, and that a
  measured derivation holds over each of its passes. It cannot prove that the facts hold every combination and
  every derivation the platform has: a fact is a selector, as a rule is, so a look the lane never wrote is covered
  by a broader fact, and a literal written where the platform derives is the generated preset's literal too. That
  is the work of the facts' independent verification, whose verifier derives each unit's state model from the
  sources without the lane's (the plan's *How values are checked*), and, on a measured platform, of a pass per
  input that derived values follow.

- **Parts not drawn** (ADD-A130): where the preset does not draw a part, the gate compares none of its fields and
  fails no field's fact that holds there; whether the part is drawn is checked once, by the part's own key (R25.17).

There is no model inheritance for the gate to trust (R20).

### R5 — Colours (C04, C05, C08, C28)

**R5.1 — Colour roles.** Today's `defaults` has 23 colours in a flat list. KDE's scheme has seven sets of the
same roles (View, Window, Button, Selection, Tooltip, Complementary, Header), each in Active, Inactive and
Disabled groups (K3.50–K3.55); libadwaita names its colours per surface (window, view, header bar, sidebar,
card, dialog, popover); Material has a ladder of surface containers and on-colours (MAT-X1), WinUI a ladder of
fill and stroke tiers.

- *A longer flat list.* Against: loses which roles belong together; cannot hold KDE's sets.
- *Surface sets with one role record for every platform.* Against: Material's and WinUI's ladders are not
  surfaces.
- *Typed surface sets with a common role record, global accent and status roles, and a keyed set of the
  platform's own roles* (Material's ladder, WinUI's tiers).

**Decision:** the third (principle 4). A role the platform does not have is absent there, as the Selection set's
outline is on `macos-*` (R25.17), unless the platform draws its meaning in a colour of its own: then it is
native-theme's labelled mapping to that colour (the specification's S3), as the info role's fill and text colour on
`adwaita` and the status roles on `macos-*` and `ios-*` where the ledger's U757–U760 pair them (R25.17; ADD-A147; D6,
decided by logic, awaiting the maintainer's veto: veto list items 3, 56 and 57).

**Groups.** The window being inactive holds for a whole window, so an inactive group is a colour-role rule under the
inactive condition; being disabled is per element, so each surface set also holds disabled roles, which parts'
{disabled} rules refer to. On KDE both are derived by KDE's colour effects (R5.4) with the scheme's effect parameters,
as `KColorScheme` does: the inactive group by the inactive effects over the inactive input roles (R2.6), the disabled
roles by the disabled effects over the active values, in an inactive window as in an active one. The Selection set is
the exception, as KColorScheme starts two of its groups from Window's colours (`kcolorscheme.cpp:304-319`,
`:374-378`): its disabled roles are the disabled effects over Window's active values, and its inactive roles, while
the scheme's `ChangeSelectionColor` holds (left unset, it follows `[ColorEffects:Inactive] Enable`, read there with a
default of true, `:307`), the inactive effects over Window's inactive input roles, the backgrounds tinted toward
Selection's background; the effects apply over these starting colours as over any group's (`:380-391`). A role's
**active value** is its value from the bases of every layer, before any rule; a derivation can name it.

**Schemes and contrast.** The colour roles have a base per colour scheme (light, dark), because exactly one
scheme always holds and the two differ in most roles; every other condition is optional and combines with
others, so it is a rule: Windows' high contrast replaces light and dark alike (one rule, from the reader's
eight system colours, W3-35). On `material-3` the contrast levels are no restated roles: a token rule under
each contrast condition sets the scheme function's contrast input, and the roles follow (one route).
Geometry is no longer stated twice: a value that differs in dark is a rule under the dark condition.

**R5.2 — Bindings (tree inheritance).** A part style may bind colour roles for its element and its
descendants: which surface set they draw on (a header bar binds the header set; Kirigami's `Theme.colorSet`,
Q1.53, Q1.96), and any single role (libadwaita's `.error` rebinds the accent to the error colour for its
subtree, `.osd` and toasts rebind their surface, KDE's sets their decoration roles; iOS's elevated level is a
sheet or a popover binding its descendants' backgrounds to the elevated roles). A role reference resolves
against the nearest element, itself or an ancestor, that bound it, and otherwise against the window's root
surface, so a reference always resolves. A binding's whole value is evaluated in the scope outside it, as
CSS's `inherit` is, so a binding never refers to itself, directly or through other roles. GTK's inheritance
of `color` is the same mechanism. Without bindings every widget would restate its role references per
container, and an invalid entry row would restate the error colour in every part.

**R5.3 — Translucent colours.** KDE, libadwaita and WinUI state many fills as translucent colours composited
over what lies beneath (GN-3.08, W3-45†); today's presets flatten them over one background, wrong on every
other surface. **Decision:** a preset states the translucent colour the platform states, and the toolkit
composites it, as the platform does.

**R5.4 — Derivations.** A value that follows another value (an input, a high-contrast colour, the current
foreground, the font size, a token) cannot be stated as a result.

- *Derive in each reader.* Against: without a reader, under a palette, or after a rule changes the accent,
  every derived value would be stale.
- *Derivation expressions in the model.*

**Decision:** expressions. Their operands: colour roles (typed, keyed, or bound); a role's active value
(R5.1); tokens; inputs; `base` (the value the earlier rules gave in this query); `foreground` (the element's
current foreground: the foreground role of the surface bound for it, unless a rule rebinds it; independent of
whether the part draws text; libadwaita's `currentColor`); and for lengths the part's font size and line
height, after the text-scaling factor (R10). Their functions are a closed set, each with one definition and
its source: `alpha(c, a)` multiplies the alpha (GN-1.210 multiplies an alpha already below one); `mix(a, b,
t)` interpolates in sRGB with premultiplied alpha, as CSS `color-mix(in srgb …)` does (libadwaita's most
frequent form, `color-mix(in srgb, currentColor 7%, transparent)`, is the colour at 7 % alpha only so); KDE's
colour effects and the `KColorUtils` functions they use, to KDE's code (their behaviour on translucent input
is read from KDE's code in the value phase and recorded in the facts); libadwaita's derivation of its standalone accent colours
in OKLab, to its stylesheet's code; Material's scheme function (material-color-utilities, which defines
Material's dynamic colour, MAT-X17), over its inputs (R2.6); and, for lengths and durations, multiples of the
font size, the line height or a token, and products of tokens (Kirigami's grid unit, Q1.62; a width in
characters, a multiple of the font's character width, which the connector supplies, R25.3, GN-1.283†; a duration times the speed factor, and Kirigami's `longDuration` times it, Q3.24;
R9). A platform whose arithmetic differs gets its own function; a function is
added only when a platform's rule needs it. Composition over the value beneath is a paint layer appended over
`base` (R6), not a colour function. Evaluation follows dependencies; a cycle is an error when its layer is added
(R2.8).

*libadwaita's standalone colours* (the specification's ADD-A128, ledger queue #105). The stylesheet writes them as
GTK's CSS relative colour, the background colour in OKLab with its lightness limited (at most 0.5 light, at least
0.85 dark), and GTK computes an sRGB colour from it at once, unclamped, which every later CSS function and
`gtk_widget_get_color` read (and `GtkTextView`'s text); the renderer draws a label's and an entry's text,
backgrounds, borders and shadows from the OKLab coordinates by a single-precision conversion of its own, premultiplies,
and the 8-bit store clamps. Which form the function is:

- *libadwaita's C function `adw_rgba_to_standalone`* (the first reading, spike G's table). For: a public API, a table
  already generated. Against: the stylesheet does not call it, and it clamps each channel, where GTK keeps them;
  most of libadwaita's documented standalone colours leave the gamut, which its documentation notes are clipped
  only before display (`libadwaita@1.10.0 doc/css-variables.md:89-92`, `:96-99`), so an
  `alpha` over it differs from what GTK draws, visibly on the focus ring (a `mix` such as the visited link's by at
  most one step of 255).
- *GTK's computed sRGB colour, unclamped, one function.* For: the value the style holds and the icon palette, the
  links, every later CSS function and GTK's public colour API read; the model's extended sRGB holds it (R5.5).
  Against: a label's and an entry's text, backgrounds, borders and shadows are drawn by the renderer's own conversion, within 7.9·10⁻⁴ (0–255)
  in gamut on spike G's grid, which no check can see (a vector compares this form, a published fact the same
  function, a capture rounds to 8 bits).
- *Two functions, a form per drawing path.* For: each path's arithmetic. Against: the renderer's conversion is a
  drawing step, not a theme value; one role is drawn both ways (a destructive button's standalone colour is its label's
  text and its symbolic icon's colour), so each role would split in two, differing once drawn by less than a capture shows, which no connector's
  toolkit draws apart.
- *The renderer's form alone.* Against: no later CSS function reads it, and out of the gamut it mirrors negative
  channels where the computed form keeps the linear piece.

**Decision:** GTK's computed sRGB colour, one function, unclamped; a channel outside the gamut is clamped where the
colour is drawn, as GTK clamps it, not by the function (which changes an `alpha` or `mix` over it) nor at the end
of resolution (which clamps a translucent colour before GTK's renderer premultiplies it). The connector's hand-over
follows GTK: premultiplied, then clamped, where the toolkit takes premultiplied colours (egui's `Color32`); the
unclamped value where it takes float colours and premultiplies them itself; a straight clamp, a recorded limit,
in the third case (straight 8-bit colours, or floats the toolkit clamps first), as that clamp brings back the end-of-resolution error (on the dark accents' focus ring up to 34 steps of 255 (13 to 34 for blue, orange, red, pink and purple; about 2 for yellow; 0 for teal, green and slate, in gamut)); `color::composite_over` draws as the hand-over does. Its vectors come from GTK
itself, through `gtk_widget_get_color` with libadwaita's stylesheet loaded, so the reference is the stylesheet's own path; the
conversion follows CSS Color 4, and GTK's constants and per-step precision, which the vectors' 1e-3 needs, come from
the clean room (D10).

**R5.5 — Representation (C28).** macOS and iOS define system colours beyond sRGB (Display P3; MAC-X10, D-17);
`Rgba` and the macOS reader's conversion to sRGB clip them. HDR appears only in the colour well's content
(MAC-X10), the application's data.

- *Keep 8-bit sRGB.* Against: P3 colours clip.
- *8-bit channels and a colour-space tag.* Against: derivations would need a rule for mixed spaces.
- *Floating-point extended sRGB* (sRGB primaries and transfer, channels beyond 0–1), which AppKit and UIKit
  hand out when asked for it. For: one space and one arithmetic; P3 kept exactly; a connector converts or
  clamps for a toolkit that renders sRGB.

**Decision:** floating-point extended sRGB; the readers ask for it. HDR is not modelled: no theme value of the
six platforms uses it.

### R6 — Paint: layers, gradients, materials, blending (C03, C08, C27)

A paint is a stack of layers, bottom first: a colour, a linear or radial gradient (colour stops with positions;
macOS 12–15's checkbox box, MAC-X09, bezels to verify, MAC-B05; WinUI's elevation strokes, W3-04), or a
material, each with a blend mode (macOS's vibrant foregrounds blend with what is beneath, MAC-X03). Paint fills
parts, strokes, text, icons and glyphs. A rule sets a part's paint or appends layers over `base`: libadwaita's
stacked fills (GN-1.30, GN-3.06) and GTK's and libadwaita's state layers are appended layers; Material's state layer
and press ripple, a layer the tokens state and Compose does not draw included (R1.6), are the part's `interaction`
block (R3; ADD-A146).

**Materials** (MAC-X02, MAC-X03; Liquid Glass, MAC-X04–MAC-X06, X-MT-5–X-MT-13; Mica, Mica Alt, Acrylic,
W3-06–W3-10; KWin's blur, K3.11):

- *The opaque fallback only* (today). Against: the look is the translucency.
- *A named material only.* For: exact where a toolkit can ask the system. Against: inside a window most
  toolkits cannot.
- *The platform's identity of the material, the parameters the platform states or a measurement gives (blur
  radius, tint colour and opacity, saturation, luminosity, noise), and the opaque colour the platform falls
  back to.*

**Decision:** the third. A parameter the platform has no counterpart for is absent (R20); a parameter refers to
a colour role only where the platform derives it from one. What parameters cannot describe (Liquid Glass's
refraction) is carried by the identity, and a toolkit that cannot use the system's material records that as its
own limit (R23). Two things are kept apart: what the platform draws under reduced transparency, an inactive
window or battery saver is a rule of the preset under that condition (an opaque colour on macOS, a frostier
glass on iOS 26, Mica's fallback colour in an inactive window, W3-10); the material's opaque fallback is what a
toolkit that cannot draw the material draws instead.

### R7 — Corners and stroke (C11, C12)

**Corners:** a radius per corner; fully rounded (a capsule at any height: Material's `full`, iOS 26's
capsules); elliptical (each corner a quarter of the ellipse inscribed in the box, half its width across and half its
height down, equal to the capsule on a square box: Breeze's QWidget round button, drawn with a radius of half its
larger side, which Qt clamps per axis to half each side, `breeze@6.8.2 kstyle/breezehelper.cpp:727-739`,
`qtbase@6.12.0 src/gui/painting/qpainter.cpp:4015-4018`, `qpaintengineex.cpp:745-746`; an application reaches it by giving such a button a
label or a stretched box); concentric (the container's radius less the inset, with a minimum where the platform states one,
X-CR-1–X-CR-3), which the toolkit computes at layout from its container; and the corner curve (circular, or
Apple's continuous). A boxed list's rounded first and last rows are rules by position (R2.3). A shape that
changes between states is a rule with its transition. For the ellipse, recording it as not expressible would leave a
visible deviation on a reachable case; two radii per corner (CSS's `a / b`) would add a field to every corner of
every preset and connector for one shape; and a shape (R25.5) is Material's closed library, implemented to
`androidx.graphics.shapes`, so it would state Breeze's drawing by another platform's code, and it replaces the
corners, whose stroke, inset, ring and shadow rules a shape does not carry. So it is one corner kind, the smallest exact form: on all four corners or
none, with circular curves, its shadow and a focus ring that follows the part following the ellipse of their own
boxes, where a ring is stated `FollowPart` on such a part (Breeze's own ring keeps its `Frame_FrameRadius` corners
on a round button, `breeze@6.8.2 kstyle/breezestyle.cpp:6631-6647`, K3.20).

**Stroke:** a width and paint per side, inside, centred or outside the edge: WinUI's gradient elevation strokes
and the thicker accent bottom edge of a focused text box (W1-026); libadwaita's outlines outside the box. A
second outline (a rim, a bevel's highlight) is an inset shadow layer (R8), not a second stroke.

### R8 — Shadows and elevation (C01, C02)

**Shadow:** a list of layers (x and y offset, blur, spread, colour, inset), or a reference to an entry of the
platform's elevation ramp. GNOME draws three layers for a window (GN-1.01); Material a key and an ambient layer
(MAT-X2–MAT-X4); Windows only the ambient layer of Fluent's two (W3-01); Breeze two, with strength read from
`breezerc` (K3.1).

- *Layers per part only.* Against: Windows, Material and KDE share shadows by elevation level.
- *Typed portable roles in the tokens* (card, menu, dialog). Against: a role cannot say that a WinUI control
  drops from elevation 2 to 1 when pressed (W3-02), or that a Material button rises a level on hover (MAT-X4),
  without restating shadows.
- *Both forms: a platform with a ramp (WinUI's 1–128, Material's levels 0–5, Breeze's sizes) states it once,
  keyed by its own names, and parts refer to entries, a rule changing the entry; a platform without one (GNOME,
  macOS, iOS) states layers per part.*

**Decision:** both forms. `shadow_color` and `shadow_enabled` go: a shadow's colour is in its layers, no layers
is no shadow, and the inactive window's shadow is a rule. Material's surface tint by elevation is a paint layer
in the same rules.

### R9 — Motion and transforms (C08's ripple, C14)

The platforms animate nearly every control (MAC-X11, W3-30–W3-34, K3.30–K3.37, GN-3.18–GN-3.25,
MAT-X13–MAT-X15, X-MO-1).

- **Transform:** scale, rotation and translation about an origin, a part-style property rules set and
  transitions animate: iPadOS's pointer lift (X-PH-1, B-17), macOS's disclosure triangle turning (MAC-X11),
  WinUI's pressed knob stretching (W1-120) and its chevron (W1-132).
- **Timing:** a cubic Bézier, linear, steps, or a spring stated physically (mass, stiffness, damping, with the
  initial velocity, rest threshold and clamping libadwaita's `AdwSpringAnimation` states), as libadwaita and
  Material state springs; a spring Apple states by response and damping fraction, or by duration and bounce, has
  no conversion to these a pinned source states (the unmeasured register's U128, UIKit's default spring: a source
  to pin, or a spring form by duration and bounce; ADD-A146); the identity, however a source
  writes it, is `Linear` (ADD-A125); any other curve the toolkit computes
  by a function of its own is one of a closed set of platform curves, each implemented once in native-theme to its
  cited source (`Timing::Platform`, R25.6): a curve is code, as a placement strategy is (R16). The set holds
  Kirigami's velocity-dependent animations (Q3.26, CONTRA-2) and Qt's and libadwaita's easing functions (CONTRA-7),
  several of which are no cubic Bézier at all (Qt's in-out sine, a cosine, `qtbase@6.12.0
  src/3rdparty/easing/easing.cpp:275-278`; its out-expo, `:309-312`; its in-out quad and cubic, two pieces, `:70-79`,
  `:122-131`; libadwaita's sine and expo curves, `libadwaita@1.10.0 src/adw-easing.c:276-295`). Rejected: stating
  the curves that equal a cubic Bézier in exact arithmetic (Qt's in and out quad and cubic, libadwaita's in and out
  cubic) as Béziers (it does not avoid the set, which the others need; it adds a second form and a checker for the
  derivation, and the facts would carry control points no source writes; a connector whose toolkit takes only
  Béziers can be given those curves' Béziers by a helper beside the evaluation, with no model change); a sampled
  table (approximate); a general expression language for curves (a second evaluator for a closed set of named
  functions).
- **Property transitions:** each style (a part's base, or a rule's patch) may state how properties move into
  it, per property group, with duration, delay and timing. A transition starts when a property's resolved
  value changes, for an element whose matched rules changed or whose values changed beneath them (an
  input), except during a theme change, which the theme-change token governs, and except under Breeze's engines,
  which start on their own test, GtkSwitch's slide, which starts on an activation, ButtonGroup's press, which starts
  as `pressed` becomes kept, WinUI's AnimatedIcon, which starts on a change of its key (below), and a CSS animation,
  which starts as its block or part becomes drawn (ADD-A146); the transition of
  the style after the change governs, in both directions, as in CSS and GTK
  (`gtkcssanimatedstyle.c`); a part hidden at rest is a `shown` part exactly when its source animates its entrance or
  exit (an unmeasured motion `Absent` meanwhile; a `popup` part's opening and closing queue #118's, its presence ordinary state; an expander's collapsed content (its presence, layout and query scope) and its reveal wholly #118's, a layout and presentation question for ADD-A129's survey; one rule for every interim of #118: a deferred subject whose motion needs machinery the model lacks, or whose row states no presentation, appears and leaves at once until ADD-A129; so a `popup` part drawn as itself opens and closes at once, one drawn as another widget presents as that widget, and every other stated presentation runs, R25.6, but an expander's reveal, a layout change no presentation kind makes; rejected: deciding the expanders here, where `shown` keeps a layout box every source collapses and an `Absent` box leaves its rows and controls drawn or unqueried (checks 18 to 23); counting the overlays as popups, whose line read either way (their parts, a substitute); and no presentation until ADD-A129, which would undo R25.6 for a deferral), styled
  in every state and hidden by `shown` (below), and otherwise `Absent` where hidden, appearing and gone at once, as
  the style after states nothing for it (ADD-A126; rejected: a part becoming drawn entering from transparent black
  under the style it enters, the ninth check's form: a part's entry is stated by its hidden style (`shown`), so it
  never enters from a style the preset did not state, while the indicator, a block, takes its other end from the
  style it is in without the flag, S2.10; rejected too:
  timing it by the style it leaves, ADD-A125's reading, under which Breeze's tab focus line could not both fade out as
  the focus leaves a selected tab and vanish as another tab is selected, both ways leaving one style). A transition may name the states it applies from (`from`, a set of flags): one with
  none applies from any state, and one with a set only where the element held one of its flags before the change,
  after the exclusive groups' drop (R3); where several transitions that apply state one for the same property, the
  later in stated order wins (Breeze's runs rank instead, below). Two platforms need `from`. Compose chooses an elevation's spec by the interaction
  entered, or, moving to rest, by the one left (150 ms from press or focus, and drag on cards and chips; 120 ms from
  hover), and snaps from none and into disabled (`androidx@e4d933c9
  compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/internal/Elevation.kt:46-113`,
  `Button.kt:1750-1770`), and its state layer's fade the same way (15 ms from hover or focus, 150 ms from drag;
  `compose/material3/material3-ripple/src/commonMain/kotlin/androidx/compose/material3/ripple/Ripple.kt:313-322`,
  `:463-483`). Breeze fades a button's pen only on leaving hover, visible focus or press and its fill only
  on leaving press, into whatever state follows, checked included, while checked, default and window-activation
  changes are instant (`breeze@6.8.2 kstyle/breezehelper.cpp:655-725`, `kstyle/breezestyle.cpp:4546-4549`). Both are
  the style entered governing with each of its transitions limited to the states it names: Material's rest
  elevation states 150 ms `from {pressed, focus_visible}` (with `dragged` on cards and chips) and then 120 ms
  `from {hover}`, the later winning where both apply, and its rest state layer a 15 ms fade and then 150 ms
  `from {dragged}`; Breeze's base states its fill transition `from {pressed}` and its stroke transition
  `from {hover, focus_visible, pressed}`, which a checked rule stating no transitions keeps, and after them zero ones
  `from {disabled}`, as Breeze changes a button at once on being enabled again, and its disabled rule zero ones, as
  Breeze runs neither fade on a disabled button. Rejected: a transition per (from, to) pair of states, XAML's form (the style
  entered already names `to`, so the pair adds nothing `from` does not, and no WinUI row needs one); exit
  transitions on the style left (they need a second precedence rule against the entered style's). A paint layer
  that appears interpolates from transparent black, and one that disappears to it (ADD-A125: Compose and Qt Quick
  fade to `#00000000` in their straight spaces, and in premultiplied sRGB every zero-alpha colour is that one), a stroke that becomes
  absent likewise, to zero width; layers are matched by their position in the stack; the `interaction` block's
  colours change at once and its alphas move (R3); colours interpolate in the space the preset's toolkit interpolates
  in, one of a closed set implemented once in native-theme to each source, as platform curves are (ADD-A125): GTK's
  CSS transitions and Breeze's `KColorUtils::mix` in premultiplied sRGB (`gtk@4.24.1 gtk/gtkcsscolorvalue.c:586-604`,
  `kguiaddons@6.31.0 src/colors/kcolorutils.cpp:144-166`), Qt Quick's `ColorAnimation` in straight 8-bit sRGB
  (`qtbase@6.12.0 src/gui/animation/qguivariantanimation.cpp:16-22`), Compose in Oklab with straight alpha
  (`androidx@e4d933c9 compose/animation/animation/src/commonMain/kotlin/androidx/compose/animation/ColorVectorConverter.kt:30-48`),
  so a fade passes through other colours in Oklab than in sRGB, and, between colours of different alpha, other
  colours in straight than in premultiplied sRGB. *One space for all* (premultiplied sRGB, as `mix`
  does): simplest, but Compose's and Qt Quick's transitions would draw intermediate colours their platforms do not.
  *A space on every transition*: exact, but each transition states what one toolkit fixes for all of its own, and
  the facts cite it again each time. *A motion token per preset*, which rules can restate by condition (Kirigami's
  flavour): exact for every source read, one fact per preset. WinUI holds two interpolators, its compositor's
  (`CompositionColorSpace_Rgb`, which its brush transitions use, `microsoft-ui-xaml@2.5.1
  src/dxaml/xcp/components/comptree/SharedTransitionAnimations.cpp:111`) and a UI-thread straight 8-bit one for
  storyboard animations it does not convert (`src/dxaml/xcp/core/animation/ColorAnimation.cpp:51-131`); its controls'
  brush transitions take the first, so one token states them; whether a control's storyboard colour animation takes
  the second is open (the specification's catalogue, Open items), and needs an override only if a fact shows one. **Decision:** the token, measured where the toolkit's arithmetic is not in a source (WinUI's
  compositor, Core Animation); a toolkit found to mix spaces within one preset and condition is a class 2 change to a
  per-transition override. A material that gives
  way to a colour crossfades between them over the transition stated (Mica's fade in an inactive window,
  W3-10).
- **Engines** (ADD-A126; ledger queue #85, #95, #103): what starts a transition, where the next one starts while a
  value moves, a spring's velocity, how several on one value combine and what a theme change does to them are the
  rules of the code that runs it, and they differ within one preset: GTK's CSS transitions start from the value drawn,
  cut a running one where the style after has none, take a value's curve by its property id (`gtk@4.24.1
  gtk/gtkcssanimatedstyle.c:822-882`), and are what libadwaita's theme switch suspends (`libadwaita@1.10.0
  src/adw-style-manager.c:227-239`), while its `AdwAnimation`s, which widget code plays from the value shown
  (`src/adw-tab.c:124-133`), are not; Breeze's state engines follow a test of their own, turn where they are on an undo, rank
  by a fixed order and mix the live state's colours (`breeze@6.8.2 kstyle/animations/breezewidgetstatedata.cpp:12-30`,
  `breezewidgetstateengine.h:45-103`, `kstyle/breezehelper.cpp:142-166`), its tab bar's restart on every change in
  two places per bar (`breezetabbardata.cpp:54-119`), while Kirigami's `Behavior`s start from the value drawn and
  animate a theme switch (`qtdeclarative@6.12.0 src/quick/util/qquickbehavior.cpp:361-426`); Compose's `Animatable`
  carries a spring's velocity (`androidx@e4d933c9 …/animation/core/Animatable.kt:227-243`) and its
  `updateTransition` replaces an interrupted tween by a spring (`Transition.kt:1615-1659`). The options weighed:
  - *One rule for every transition* (from the value drawn, ADD-A124's implicit one): simplest, but Breeze's turns,
    restarts and ranks, Compose's velocity and interruption spring and GTK's cut are lost, each visible within a
    transition's run (a line edit hovered and clicked, a tab left mid-fade, a switch toggled twice).
  - *A trigger field* (`ValueChange | StateChange { conditions }`, ADD-A125's form) with an interruption field: exact
    for when Breeze's engines and its decoration's shadow start, but ADD-A125's eight checks on it patched each toolkit in turn
    without converging (Breeze's rank, its tab bar's restart, a spring's velocity), and libadwaita's `AdwAnimation`s
    and Compose's state layer start on a change of their own value too, so the trigger no longer told libadwaita's
    suspended transitions from the rest.
  - *Orthogonal kinds* (an interruption kind, a velocity, an interrupted timing, a rank, slots and a theme-change lane,
    the survey's grouping): each exact only with the sub-rules of one toolkit (GTK's cut and its curve index, the tab
    bar's two places, the decoration's live ends, AdwEntryRow's replay), six fields where each engine uses one
    combination, and a combination no toolkit has stays expressible.
  - *A per-preset engine*: `adwaita`, `kde-breeze` and `material-3` each run two engines whose rules differ.
  - *A preset default with a per-transition override*: two homes for one fact.
  - *An engine per part* (stated once on a part, rules restating it, Breeze's test per transition): it saves the
    `engine` repeated on each transition; but a part runs two engines on its groups (Compose's segment, its press
    `ComposeHeld` and its state layer `ComposeAnimatable`, MAT-N6, MAT-X9; its check mark, its drawn fraction
    `ComposeTransition` and its colour `ComposeAnimatable`, MAT-C8), a rule changes a group's engine (Q3.26's
    `QtQuick` under `flavor:kirigami`), the focus indicator's runs on its own, and Breeze's test and the AnimatedIcon's
    visual, the engines' data, are per transition anyway, so the part's engine would be a second home beside the
    transition it governs.
  - *A gate check in place of Breeze's reduction* (S12 proving, for every combination of flags, the style under the
    kept flags equal to the style under the reduced ones, so the connector draws the kept flags): a simpler runtime,
    but each preset would restate Breeze's precedence as rules for every combination a test excludes (a hovered
    disabled control, a focused selected tab), the run's ends need the reduction with its own test excepted, which
    the cascade has no rule for, and the precedence is Breeze's code (`if (animated) … else if (test)`), which the
    engine implements once, as it does Breeze's curves.
  - *Stated limitations*: Breeze's differ for at most its duration (100 ms by default, `kstyle/breeze.kcfg:53-55`),
    but native fidelity and cover-everything-visible admit none where the source states the rule.
  - **Decision**: *the engine*, `Transition.engine`, a closed set of the toolkits' engines (`GtkCss`, `AdwAnimation`,
    `QtQuick`, `Breeze`, `BreezeTabBar`, `ComposeAnimatable`, `ComposeTransition`, `ComposeShapeMorph`,
    `ComposeHeld`, `WinUi`, `WinUiAnimatedIcon`, `GtkSwitch`, `CssAnimation`; the specification's S2.7 holds the
    list, ADD-A146), each implemented once in
    native-theme to its source, as the platform curves, the ripple effect and the scroll decelerations are: a curve's,
    an effect's and a deceleration's rules are code, and so are an engine's. Its data is only what the code takes per
    transition: Breeze's test, quoted from its `updateState` call, whether AdwEntryRow's title replays, and the
    AnimatedIcon's visual, whose duration and markers `token.icons.animated_visuals` states once per visual. The value
    engines start from the value drawn; Breeze's turn where they are or restart, ranked in stated order with live
    ends; `from` stays ADD-A124's, the flags held before, as no fact reads otherwise under "a flag cleared" (ADD-A125's
    attempt) and the one case that needs two ways out of a style told apart, Breeze's tab focus line, is stated by
    where the line is drawn; a theme change's suspension reaches `GtkCss` alone; a stated zero names the engine that makes it instant, as
    GTK delays one and Compose's `updateTransition` replaces an interrupted one; where one group's transitions name
    two engines, the governing transition's engine draws it, one hand-off rule deciding the update where it changes
    (a value engine leaving ending at the value drawn, Breeze's runs going on hidden, a value engine taking over from
    there, Breeze taking over at its rest value; a Breeze zero is Breeze governing). Breeze's group draws its flags reduced by its tests, at rest, under a
    zero and at a run's two ends alike (each failing test's `any` flags dropped, the run's own test excepted, but those a passing test names), so a
    fade a flag's style makes invisible is not seen, and the fades Breeze hides under an excluded flag (its pen and
    fill, its header) are hidden by a stated zero in that flag's style; a rule hiding every run while an excluded flag holds was
    rejected, as Breeze draws a line edit's focus fading out as it is disabled and a disabled tab's hover fade
    (`kstyle/breezehelper.cpp:146-155`, `:1498-1499`). Apple's toolkits, whose
    code is not in the cache, are measured (interrupted at three points, a spring's velocity, stacked changes, a theme
    change, a deactivation, an occlusion and reduced motion landing in a run) and get their member with the
    measurement, their transitions pending facts the presets state none of meanwhile, each path in the preset's `[pending]` table
    (S17, ADD-A128; never a placeholder `[]` or `Absent`), with no placeholder member (a
    member with no source would be an invented rule); WinUI's theme switch, its compositor's velocity and Mica's engine
    likewise. A presentation interrupted (a popover dismissed while it
    appears) is a presentation event's matter, not settled here: ADD-A129 (queue #118), meanwhile cut, the state after
    drawn at once (S2.7's one rule).
- **A part not shown at rest, drawn on, grown or trimmed off** (ADD-A126; ledger queue #85): Breeze draws its check
  on by a dash and grows its radio dot both ways, WinUI's accept visual trims its check on and off and its pressed
  radio dot grows, Compose draws its check by a fraction and grows its dot by its radius, GTK swaps the icon at once,
  and macOS's dot is measured.
  - *`Absent` at rest*, for a part the platform animates in or out (ADD-A125's form): the rest look by the part decision, which S6.3 now keeps under a later
    layer's field patch (#110, below), but an absent part answers `NotDrawn` and vanishes at once, so the motion has
    no values to move from or to; letting every absent part enter from and leave to the fields beneath it would
    change every part the platforms remove at once.
  - *Hidden values alone* (a zero extent or size in every state, check 17's form): plain transitions, but an extent
    has no reading on a code point, a theme icon or a system rendering, and GTK's swap has no value to zero.
  - *`opacity` 0*: a later patch of the opacity shows the part, and a fade runs where the platform draws on.
  - *A hidden look on the transition*: one patch per transition, but it cannot follow the state the mark trims off in
    (WinUI's per-state foregrounds, Compose's unselected colour), and it is a second home for values the rules state.
  - *A `Keyed` transition per platform*: the motion out of the model's terms, which a connector cannot draw from the
    theme; right only where the platform's own asset is an animation (WinUI's AnimatedIcon).
  - *WinUI's check as one stroked path* (`draw_extent` on, `draw_start` off): the accept visual is a composition of
    visibility-switched layers trimmed per segment, which one path's extents do not reproduce.
  - **Decision**: *`PartStyle.shown`*, a field listed only on the parts a platform hides by state and animates in
    or out (the check mark, the radio dot, a value bubble, WinUI's list selection pill): the part is styled in every state,
    `shown = false` where nothing is drawn at rest, laid out as styled, so a later layer's patch of another field
    leaves it hidden and its look is kept in every state; it is
    drawn while `shown` holds or while it leaves (the runs running as `shown` became false), a run started while it
    is hidden drawing nothing, so its hidden style, its style where `shown` is false, is what it enters from and
    leaves to under the transition that governs, one rule for every engine; a part removed at once stays `Absent`. `GlyphDraw` times a glyph's path and drawn span apart from its colour, as `StrokeWidth` does a
    stroke's widths, for Compose's check, whose spatial and effects motions differ; a path moves to one of the same
    commands point by point (the check pulled to the dash), and any other source change is discrete. WinUI's mark
    stays the AnimatedIcon's, its visual's duration and markers in `token.icons.animated_visuals` and its frames the
    Windows lane's asset; AppKit's and UIKit's motion is measured.
- **A later patch over a conditionally absent part** (ADD-A126; ledger queue #110): under ADD-A123's S6.3 a `Style`
  patch after an `Absent` brought the part back, and every layer's rules apply after the preset's, so an
  application's `@ {hover}` colour drew a title bar in full screen (GN-P3-5†, W-P3-2†) or WinUI's combo-box pill on
  unselected items (W1-135), as it would an unchecked mark; a field made `Absent` by a rule (the menu's check glyph,
  Breeze's tab focus line) met the same patch as a refused merge conflict.
  - *`shown` on every part a condition can hide*: robust, but every such part would state it, and an absent part
    leaves the layout while a hidden one keeps it, so a title bar in full screen still needs `Absent`.
  - *An S12 refusal of a later rule that can match where a part is conditionally absent*: it refuses an
    application's ordinary rule instead of keeping the part absent.
  - *Documented only*: the defect stays.
  - *For fields*: (a) every field a rule hides made a nested part (the menu's check glyph a `check_mark` part), so
    the part rule covers it: a part per such field, its widget's structure changed for a presence; (c) `shown` on
    the field's part: it hides the whole part, while the menu's column and Breeze's box or ring stay drawn; (b) the
    part rule for block fields too.
  - **Decision**: *presence by its own patches* (S6.3): a part's presence changes only by `Absent`, `DrawnAs` or
    `Present` (`absent = false`), a patch that states none setting fields only, so no later field rule restores an
    absent part; and a block field's only by a `Set`, a `Fields` patch over an `Absent` block leaving it absent
    (S2.2), the smallest form and the part rule's own; a paint follows the same rule, an `Append` over an `Absent`
    paint appending to nothing, so outside a keyframe only a `Set` draws it again (S6.1; in a keyframe's patch any
    patch of an absent field, a `Set` included, is `None`, S2.7), and a preset whose state rules append over a rest
    paint states that rest drawn, as the platform does (adwaita's list rows keep GTK's initial transparent
    `background-color`, GN-1.158). `Present` is needed beyond the presets (GtkInfoBar's edge line,
    GN-2.31, could be restated by narrower selectors): an application layer that draws a part its preset hides
    cannot restate the preset's selectors, so it says so. One rule for every part and block, base or conditional, in
    every layer; a whole `Set` stated for every state draws the block in every state, as `absent = false` does a
    part, which is the layer's statement, not an accident. Rejected: refusing a *dead patch*, one meeting `Absent`
    wherever its rule applies (ADD-A126's tenth check): S6.3 keeps a patch's fields beneath an `Absent` part for a
    later `Present` to draw, so a patch latent under one combination is not dead; for a block, the one spelling
    (below) keeps every base patch live, and a narrower rule meeting an `Absent` block wherever it applies is the
    layer's own choice; refusing it would make a layer's validity hang on each preset's selectors (the eleventh check's
    withdrawal). Rejected for paints: an `Append` over an `Absent` paint giving a paint of the appended layers (S6.1's
    rule until commit 8a567e44), which CSS's `background-image` matches, GTK drawing an element's image layers whether or not
    a colour lies beneath them (`gtk@4.24.1 gtk/gtkrenderbackground.c:266-311`): a later layer's `Append @ {hover}`
    would draw a paint a condition made absent, the defect #110 removes for parts and blocks, and a paint would be the
    one field a patch stating no presence could bring back.
  - *Spelling a block drawn by state* (ADD-A126's twelfth check): (a) one spelling on every preset, the base stating
    it drawn, its state rules patching it, the rule hiding it last; (b) each preset its own spelling, the catalogue
    documenting that a layer's base patch reaches the block only where the base draws it; (c) an S12 check of the
    spelling. (b) makes one application layer act on one preset and not on another; (c) checks what (a) states once.
    **Decision**: (a), stated in the catalogue's Entry format, so an application layer's base patch reaches the drawn
    block on every preset, beneath each preset's state rules as every base patch is.
- **Presentation transitions:** an element appearing, disappearing or moving (a popover opening, a dialog's
  entrance, a page pushed, a sheet presented), keyed by event per widget, which rules can change (reduced
  motion, flavours); the kinds are an open list in the specification, from the map's rows: fade, slide from an
  edge, scale, shape morph, clip reveal (WinUI's combo box opens by revealing from the selected item, W1-136),
  and the rest. `dialog.entrance` becomes one of them.
- **Ripple:** no transition but a field of the part's `interaction` block, a platform's own effect from a closed set
  implemented once in native-theme to its source, as platform curves are, which the press starts and whose whole
  lifecycle, the release's exit included, is the effect's own (R3); its origin is the point pressed where it is bounded,
  which the toolkit has from the press itself, and the box's centre where it is not. Compose on Android draws it with the framework's `RippleDrawable` in its patterned style (MAT-X9),
  whose shader draws a wave with sparkles, so the ripple is that effect, implemented to its source, not a circle with
  stated durations. One transition governs a property group per change, as in CSS (Breeze's engines rank their runs instead, ADD-A126); the state layer has a group of its own,
  so it does not time the container (R3). A stroke's widths may be timed by a group of their own,
  `StrokeWidth`, which overrides `Stroke` for them where stated, as Compose moves a text field's indicator and an outlined toggle's border colour and width by
  different springs and CSS transitions `border-color` and `border-width` apart; a transition naming fields within a
  group was weighed and rejected, a field-path list on every transition for one pair.
- **Loops:** a period, a timing and the keyframes the platform states, extending `AnimatedIcon` (spinners,
  indeterminate progress).
- **Scroll physics:** deceleration, overscroll and its spring, snapping, distance per wheel step (GN-3.24,
  W3-52†). A deceleration is a platform's own model, from a closed set implemented once in native-theme to each
  source, as the platform curves are: the platforms decelerate by different physics (a decay per millisecond, a
  constant deceleration, Android's spline fling, MAT-P3-16†), so one number read one way by every connector would
  convert some of them approximately.
- **Tokens and the speed factor:** the platform's published durations and curves, referred to by parts; a global
  speed factor (KDE's `AnimationDurationFactor`, K3.30) read by the reader. Which durations the factor scales is
  the platform's, so each scaled duration is stated as a derivation, a token times the factor, but under an engine
  that scales its own play time, Compose's, whose durations are plain, the factor applying once (ADD-A126) (Breeze's
  animations are its factor times 100 ms while `AnimationDurationFactor` is set, and breezerc's
  `AnimationsDuration` otherwise, both of which the reader reads), and an unscaled one as a plain duration
  (Qt's own 250 ms tab move is not scaled). At a factor of zero a spring jumps to its end, as a curve does.
- **Theme changes:** what the platform does when the colour scheme, the accent or another input, a contrast
  setting, a palette or the preset changes (Appendix B's kind K57; GN-3.23, W-P3-16†) is the theme-change
  token: one of a crossfade of the window with its duration (no source read states one: it is the answer the
  map's rows on the platforms whose code is not read may get, whether the switch changes at once or cross-fades,
  MAC-P4-9†, X-MO-11†, W-P3-16†, MAT-P3-18†, so a preset states it only from the measurement that finds it;
  ADD-A146); property transitions suspended for a duration
  (libadwaita turns CSS transitions off for 250 ms on every style update, `adw-style-manager.c:236-239`,
  `:296-299`, with no crossfade; it reaches the `GtkCss` engine's transitions only, so the tab's 150 ms close-button
  fade, an `AdwAnimation`, still runs, `adw-tab.c:854-862`, and a CSS transition running when the suspension starts
  ends at its new value, as GTK carries none into a style whose transitions are all zero,
  `gtkcssanimatedstyle.c:822-826`; libadwaita also suspends on a reduced-motion change, which the token names,
  `adw-style-manager.c:412`, and on the application's own scheme choice, `adw_style_manager_set_color_scheme`,
  `:979-991`, the model's `pick`; ADD-A126); or instant (an instant switch with each engine's transitions as for any
  change). Each preset's kind is a fact, measured where no source states it (`kde-breeze`'s is `Instant`, from
  Breeze's and Qt Quick's code, which stop nothing and add no window effect, ADD-A126). Other condition changes (a window becoming inactive, maximised or tiled) are not theme changes: they start the value
  engines' transitions of the values they move and turn the Breeze tests that name them, so Breeze's decoration
  shadow fades on activation alone and changes at once on a maximise (K1.17), and Mica's fade runs as its engine
  does (W3-10).
- **Reduced motion and animations off:** rules under these conditions stating what the platform does instead:
  libadwaita swaps a bottom sheet's slide for a crossfade under reduced motion, and under animations off jumps to
  the end (GN-3.25), which the preset states as a rule setting the speed factor to zero. The model never zeroes
  durations by itself.

### R10 — Typography (C18)

**Text style:** the font gains width (stretch), variable-font axes (`opsz`, `wdth`, …; an axis may follow the size
automatically, as SF's optical size and Segoe UI Variable's do, W3-23), OpenType features, a fallback family list,
an emoji family and a per-language fallback map (MAC-FM01†, W-FM01†, K-FM01†, GN-FM01†, MAT-FM01†, FM-1†); the style
gains letter spacing (tracking, which SF Pro states per size, D-1), its own line height per text role and style
(today only `defaults` and the four text-scale entries have one), **decoration** (a line's kind, thickness, offset
and paint: the spell-check and IME underlines, mnemonic underlines, a link's underline; K1.49†, W1-151†,
GN-1.281†), and **elision and wrapping** (where a platform truncates, at the end or the middle, fades the text out under
another part, as AdwTab fades its title under its close button with a mask whose strength follows the
button's opacity, GN-1.129, or wraps). The colour, elision, wrapping and line limit are what a widget does with the
text, so a named style states them only where the platform's style sets them, and the parts that draw text state
them otherwise; a low underline is placed from the ink of the run it marks, as Pango draws a mnemonic's (R25.17).

**Text roles:** today four `text_scale` entries besides the body font; macOS and iOS have eleven styles, Material
fifteen and fifteen emphasized (MAT-TS4), WinUI eight, libadwaita about nine.

- *Each platform's own names only.* Against: no portable "heading".
- *A fixed semantic ramp only.* Against: either huge or merging styles.
- *A typed semantic ramp (each preset filling every slot, its facts saying which of its styles fills it) plus a
  keyed set of the platform's own styles.*

**Decision:** the third (principle 4). Which style fills a slot the platform names no style for is native-theme's
labelled mapping, never the platform's fact: every slot is filled, outside R5.1's conditions for a colour role, by a
style the platform names, or a family it names for the nearest uses, never a made-up font (R25.17; ADD-A147; D6, decided by logic, awaiting the maintainer's veto:
veto list item 3). **Text scaling:** the text-scaling factor the reader reads (as today's
`text_scaling_factor`; 1 where the platform scales text another way, as KDE does on Wayland by the font size
itself, read) multiplies font sizes when a query evaluates them, before any length derived from the font, as
platforms that derive metrics from the font do (Kirigami's grid unit grows with it); a line height stated in
pixels is scaled with it, one stated as a multiple of the font size follows the scaled font, and letter and word
spacing follow the line height's rule (Compose's typescale tracking is in sp, which scales with the font, MAT-TS2). Dynamic Type and Android's
non-linear font scaling (MAT-X19) are not a single factor: those platforms' readers read each style's size
(R18), and a preset states the default size category's values.

**Points.** A size stated in points becomes logical pixels before any of this, at the platform's font DPI, which is not
one constant across the platforms (today's `ResolutionContext::font_dpi`): the preset states its platform's DPI as a
token, from its facts, or 72 where the platform's point is the logical pixel, the model's own value (Apple's; the
specification's S9.4, ADD-A125), and a reader reports the system's where the platform has a setting its toolkit's
scale factor does not already carry (on an X11 session the font DPI reaches the application through that scale, so
no reader of 0.8.0's reports one, the specification's S9.4; ADD-A146); a change of it is a theme change (R9). Folding the DPI into the
text-scaling factor was rejected: that factor also scales sizes stated in pixels and is 1 where the platform scales by
the font size itself, so it would scale pixel-stated fonts wrongly; a fixed DPI in the code was rejected as false on
some platforms and as dropping the user's KDE setting.

### R11 — Icons and glyphs (C13, C19)

**Icon style:** size (the existing `defaults.icon_sizes` contexts become its tokens), fill (the recolouring of
symbolic icons, a paint, so vibrant icons blend, MAC-X03), rendering mode (SF Symbols' monochrome, hierarchical,
"palette" in Apple's sense, multicolour; and, where the platform decides per icon, as GTK and KDE do, the icon's own
kind, R25.17), weight and scale, and a per-state effect where the platform changes an
icon by state (KDE's disabled and active effects, K3.60). Mirroring in right-to-left layouts is a property of
each icon role (MAT-FM04†). Symbol effects are transitions.

**Glyph:** a check mark, a radio dot, a disclosure arrow, a combo chevron: glyphs no icon set has (`IconRole` has
none of them). A glyph is the platform's own, from one of five sources: a code point of its symbol font (Segoe
Fluent Icons); an icon of its icon theme by name; a vector path taken from its source code (Breeze draws its
check mark with `QPainterPath`); a system image the platform names for that purpose (AppKit's menu on-state check
mark); or, where the platform names none, the system's own rendering of the whole control.

AppKit draws its controls from CoreUI's compiled `.car` assets (as `platform-facts.md` §2.11 records for the tab
bezel) and names no image for the checkbox's mark, the pop-up button's arrows or the disclosure triangle (its
named triangles are generic left- and right-facing images, which Apple says to use for their purpose, not their
look). For MAC-C06, MAC-CB01, MAC-EX02 and MAC-L10 the part's look is therefore the control as AppKit draws it
(`NSButton` with its checkbox or disclosure bezel, `NSPopUpButton`), in the state, size and appearance asked for,
rendered by native-theme at run time through AppKit, on the main thread as AppKit requires, at the display's
backing scale and the control's pixel size, with the window's key state, and cached per control, state, size,
appearance, key state, layout direction and scale, as the SF Symbols loader renders symbols today; the cache is emptied at every
theme change the reader reports (R2.5), as AppKit's drawing follows settings the key does not hold (the accent;
ADD-A146). Called off the main thread, or
off macOS (a `macos-*` preset used on another platform), the rendering is absent and the modelled parts stand;
a toolkit that cannot use the rendering records that limit (R23). For a toolkit that composites it, the rendering replaces the parts it covers (the checkbox's box and mark
together, the rendering stated on the box, drawn in every state, ADD-A126); the modelled box (its measured fill,
stroke and corners, R6, R7) remains for a toolkit that cannot, and the radio's dot (its measured diameter and its
colour, a cornered fill) likewise, while the checkbox's mark has no model of its own, so without the rendering no
mark is drawn and a checked and a mixed checkbox differ only where their box fills do, the mixed one a measurement
(MAC-C01), the limit recorded where it is (R23). The rendering uses the system's accent: an application's accent cannot
reach it, which the connector records (R23). Measuring the mark's geometry from renders was weighed and rejected:
its outline can only be traced from a picture, which the project's asset rule forbids; its size and colour are
still measured. Elsewhere a glyph's size, stroke width and fill are its values.

### R12 — Focus, caret, selection and highlights (C15, C16, C17)

**Focus indicator:** a block (R2.1) every focusable part's style carries: a kind (a ring, an underline, or a
platform's own keyed kind, WinUI's Reveal; a part with no indicator states the block absent, and a filled ring is
the ring, no kind of its own: the specification's S2.10, ADD-A141, ADD-A146), stroke, offset, corners (concentric
with its part unless stated), shadow and transition, shown in the focus-visible state. The base states it, by
reference to the platform's token where the platform has one indicator, or its own where a part differs (KDE's focus line under tab labels, K3.21). A focus that only changes a
fill is a {focus visible} rule, not a kind. Today's three `focus_ring_*` fields become the token; when focus
becomes visible is a widget behaviour (R17). The indicator is drawn while focus is visible and while its transition out runs; its other end is the
indicator of the style without the flag under the element's other flags after the change, its paint and shadow
transparent, so libadwaita's ring grows in from and shrinks out to its rest outline, zero wide and transparent at a
larger offset (`libadwaita@1.10.0 src/stylesheet/_drawing.scss:12-27`), and an indicator `Absent` there appears and
leaves at once: Breeze's tab line, drawn in the tab's base and
`Absent` off `{selected}`, fades in on a newly selected tab from `{selected}`'s transparent line and is gone at once
from the tab left (ADD-A126, ledger queue #85). Weighed: the rest style's ring with its own paint (a meaning for a value
no platform draws at rest); every preset stating its rest ring transparent, with no rule (every focusable part's rest
style restating the ring, one left at the focused paint jumping in); an entering geometry on the block (a repeat of
the rest style). The focus flags are the toolkit's in an inactive window, which GTK and Qt clear (S7.2).

**Caret:** colour, width (a token where the platform has one width; where it follows each line, as GTK's does, the
caret parts', R25.17), shape (bar, block in overwrite mode; K1.47†, KDE file), the secondary caret in
mixed-direction text (GN-SF03†), blink on and off times and timeout, read where the platform has a setting (GNOME,
GN-3.32–GN-3.38; Windows; KDE; macOS's blink period, MAC-X14), and when it hides (while text is selected,
GN-3.73†, GNOME file).

**Selection and highlights:** colour roles per surface (R5.1); the find highlight and the drop-target highlight
are roles and parts of the widgets that draw them.

### R13 — Window chrome (C20)

The window becomes parts: the frame (paint, corners, outline, shadow), the title bar (heights per variant,
padding, title alignment and text style), the caption buttons (one part type, each button an element with its
glyph and its own rules, such as the close button's red hover), the window menu, and the resize zone. Maximised,
full-screen, tiled (per edge; a window maximised along one axis counts as tiled to those edges, as Breeze's
edge tests do) and inactive windows are conditions, so the corners and shadow a tiled window drops
are rules. The caption buttons' order and side are read where the platform has a setting (GNOME's
`button-layout`, KDE's `kwinrc`).

### R14 — Parts and controls (C13, C24)

A widget gains the parts the map names (a field's helper text and counter, a tab's close button, a slider's
ticks, a sheet's grabber, a selection pill).

Controls the model lacks: the map's §4.2 lists 118 kinds. A kind is portable when two or more platforms draw it,
counted with the cross-check decisions that found it drawn inside an element the model has (every platform's list
draws a tree, though only Windows lists a tree view as a control).

- *A typed struct per kind.* Against: structs each used by one preset.
- *Typed for kinds two or more platforms draw (a struct of its own, or parts of the element it is drawn in); keyed
  components for the rest,* named components of named parts from the same blocks, declared per preset (R2.7).
- *Only the controls the three toolkits have.* Against: the model would follow toolkits.

**Decision:** the second (principle 4). A keyed component that later appears on a second platform becomes typed
(breaking, allowed before 1.0). Kinds out of scope by R1.1 or R1.2 are not modelled.

### R15 — Variants, control sizes and density (C09, C10)

All three are selector parts (R2.3), declared per widget (R2.7).

**Variants** (flat, outlined, filled, tonal, destructive, suggested, pill, circular, AppKit's bezel styles, iOS's
button configurations, glass): an element carries a set of variants; rules match sets and their negations, so
libadwaita's stacked classes are rules for {flat, circular}, and its "flat unless destructive" a rule with a negated
set. Names are typed where two or more platforms share the meaning (taken from the map's C09 rows) and keyed
otherwise.

**Control size and density** are two axes, because the platforms have both and combine them: a control's size
(macOS's mini to extra large, MAC-X17, set per control; iOS's `UIButton.Configuration.Size`) and a density
(Material's density scale, WinUI's compact sizing, W3-39, macOS 26's compact control metrics). Density is
tree-inherited: an element takes its ancestors' unless it states its own, as Material's density and WinUI's compact
resource scope propagate; an application-wide density is the root's. A density reaches only the widgets that
declare it (Material's density applies to some components); a widget that declares none ignores it, whether it
inherits a density or its element states one. Size is the element's own, as AppKit's `controlSize` is. A size or
variant the element's widget does not declare in its context returns "not available" (R2.7): an application asking
for a small button on `adwaita` learns there is none, rather than getting an invented one. A variant, a size, a
density or a flavour the platform has and a later unit states is answered 'later': the theme does not state it yet (ADD-A147; D6,
decided by logic, awaiting the maintainer's veto: veto list item 1).

### R16 — Layout, placement, breakpoints and direction (C26)

Geometry is in part styles (padding per side, size limits) and widget values (gaps between parts, which may be
negative where parts overlap, alignment, the order of dialog buttons, a content clamp, GN-1.195).

**Placement** (where a menu, popover, tooltip or dialog opens against its anchor or window, its margins, flipping
and sliding) is parameters plus a named strategy from a closed set, each implemented once, in native-theme, as a
pure function of the anchor, the element's size, the window and the text direction (no toolkit types), to its
platform's source and cited (GTK's tooltip placement, GN-1.83; the dialog rows of the seed follow-up, G24): code
cannot be data, and writing each strategy once beats writing it in three connectors.

**Breakpoints:** a size class is the platform's. Where the platform defines it by width thresholds (Material's
window size classes), the preset states the thresholds among its tokens and the connector computes the class from
the window's size; where the system assigns it (iOS's `UITraitCollection`, set "based on the device type, window
configuration, and multitasking state", X-SV-1), the system's class is read and passed in. Either way it is passed in
at the root, and a container with its own (an iPad split view's column) passes its own to its subtree; rules select
on it (R2.3). A breakpoint one widget owns is its value: AdwDialog becomes a bottom sheet at a width of 450 or a
height of 360 or less, and the bottom sheet is then a variant of its presentation that rules select on.

**Right-to-left:** the query mirrors the layout: values are stated for a left-to-right layout, and the query
returns them physical for the element's direction, so no connector mirrors again (the thirteenth check's amendment,
the specification's S2.4; ADD-A146); the model states what the platform does not mirror or does differently
(Material keeps time-based content left to right and mirrors question-mark icons in Arabic and Farsi but
not Hebrew, MAT-FM03†; libadwaita's `:dir()` rules, GN-FM03†), as icon-role properties (R11) and rules selecting on
the direction (R2.3). What mirrors is the layout's own geometry (padding and margins per side, alignments, placement)
and a part's translation, a displacement in that layout; a part's drawing (its corners, stroke edges, shadow offsets
and paints) is physical, and a platform that mirrors one states a rule under the direction. Mirroring the drawing
with the layout was weighed: it would save the rules where a platform mirrors (a segmented control's rounded start
corners), but the platforms state these physically and mirror by their own direction branches (GTK's borders and
radii with `:dir(rtl)` rules, GN-1.42; Breeze's painter rectangles), which the traceability carries as direction
rules, while a light source does not turn with the text.

### R17 — Timings, behaviours and the pointer (C21, C22)

Timings and behaviours that apply everywhere form a behaviour record among the tokens: double-click time and
distance, drag threshold, the time a held press takes to start a drag and the earliest a drag may start, the system's
hover time, key repeat, whether a disabled element shows its mnemonic, and whether the pointer hides while typing;
the distance a wheel step scrolls, in lines or pixels, belongs with the rest of a wheel step's motion in the scroll
physics (R25.17). Those that belong to one widget are its values: when its focus becomes visible (Breeze after keyboard
focus, K3.20; macOS text fields on any focus, its buttons only with Full Keyboard Access); the tooltip's show, hide
and reshow beside today's `tooltip.show_delay_ms`; the menu's submenu delay and sloppy-close timeout (K-RV2†); the
scroll bar's auto-hide and what a click in its trough does (K-RV1†, MAC-P3-2†); a toast's duration. Each is read
where the platform has a setting and stated otherwise. The pointer: the cursor shape per part, the cursor theme and
size (read), and iPadOS's pointer effects (X-PH-1), which are hover rules with a transform.

### R18 — Readers (C25)

Every value the map marks readable is read by the platform's reader where native-theme has one; the preset states
the default the reader replaces; the specification lists the new reads, and today's wrong reads are fixed with them
(KDE's title bar from `[WM]` instead of `[Colors:Header]`, docs/todo.md, *Values the v0.7.1 gap map found against
their sources*; GNOME's dropped font list, GN-FM02†). **A reader states each value under the conditions it was read
in:** a value read under the base conditions goes into its bases, one read under a condition (the macOS appearance
with increased contrast, Windows' system colours under high contrast) into a rule under that condition, so no preset
rule applies on top of it twice. It also supplies the system's conditions (R2.3) and the inputs it reads (R2.6).

iOS and Android have no reader, and native-theme has no supported, tested target there (it compiles for iOS); an
application drawn with iced or egui can run on both through winit, and without a reader would ignore the user's text
size, contrast and colours (Android's dynamic colour, MAT-X17).

- *Readers for both in the same release.* Against: two new platforms to build and test, which is platform support,
  not model completeness, and would hold the model back.
- *Neither, ever.* Against: principle 1.
- *Both, as a platform-support milestone after the model release,* with the iOS icon loader (X-IC-7). For: the model
  already carries what they will read; meanwhile the application stands in for the reader (R2.6).

**Decision:** the third, the same for both; their R rows map to that milestone (R24).

### R19 — Palettes (community colour schemes)

A community preset (Catppuccin, Dracula, Nord, …) states every field today. With this model it would need shadows,
motion, materials and states its project never defined, which principle 3 forbids.

- *State them anyway.* Against: invented values.
- *A fixed platform base per community preset.* Against: arbitrary.
- *A palette mapped from the project's style guide.* Against: guides do not give one colour per role (Catppuccin's:
  "Overlay 2 20% - 30% Opacity" for the selection, "Crust, Mantle" for secondary panes), so mapping them is choosing
  values; and not every project has a guide.
- *A palette taken from the project's own port for that platform* (its KDE colour scheme, its libadwaita colour
  overrides, …), converted mechanically. For: the project states the platform mapping itself; what the port leaves to
  the platform is the project's look on that platform. Against: a project without a port for a platform has no
  palette there.

**What the survey found** (2026-10-28, the specification's first step): of the seven projects (ten presets), only Catppuccin
(a maintained KDE colour scheme for each of its 4 flavours × 14 accents) and Dracula (a KDE colour scheme, dark only, and
a whole GTK theme without libadwaita's colour names) publish platform ports, and only for KDE; Gruvbox, Nord, One Dark,
Solarized and Tokyo Night publish none, and no project publishes one for Windows 11, macOS, Material 3 or iOS. The fourth
option would leave palettes on KDE only and drop five projects.

**Decision (the maintainer's, 2026-10-28):** a fifth option, which the survey made necessary. Every community scheme
stays, as a palette on every platform. A palette is **native-theme's own mapping** of the project's published colours
onto the colour roles, documented and labelled as native-theme's (neither a platform fact nor the project's), the same
on every platform; where a project publishes a port for a platform, that port is the palette there instead, converted
mechanically. Principle 3 concerns platform values: a palette is a decoration an application chooses, not a platform's
look, and its mapping is stated as native-theme's choice, never presented as a fact. The mapping:

- maps each role it covers to a colour the project publishes (Catppuccin's `Surface0`, Dracula's `Background`), or to a
  derivation of those by the model's functions; where a project's guide gives a range ("Overlay 2 20% - 30% Opacity"),
  the mapping records the value it picks and labels it as its pick;
- leaves every role it does not cover to the platform's value and derivation (a libadwaita hover is the foreground at an
  opacity, whatever the foreground is);
- is one palette per variant the project publishes. A variant is a set of the project's base colours under one
  name with one of the accents the project publishes for it, in the schemes the project defines under that name:
  Catppuccin's 4 flavours × 14 accents are 56 variants (ADD-A146). 0.8.0 bundles the ten schemes 0.7.0 ships (the
  plan's D13), each as the variant whose accent 0.7.0's preset states (at `v0.7.0` Catppuccin Latte `#1e66f5`,
  Frappé `#8caaee`, Macchiato `#8aadf4`, Mocha `#89b4fa`), which the palette's facts check against the project's
  published colours; another accent's variant is another palette, not bundled. Rejected: bundling every variant, 56
  Catppuccin palettes and as many KDE ports, where D13 asks for what 0.7.0 has. It offers only the
  schemes the project defines (Catppuccin Latte is light, the other three flavours dark; Dracula dark), and today's
  invented light variants of the dark flavours go;
- is an application layer (R2.4) of colour-role bases, input bases and colour tokens, marked as replacing the reader's
  colour scheme: the reader's colour values (its scheme, its effect parameters, and under a mapping the user's accent)
  are not applied; under a KDE port, which states no accent, the reader's accent stays in force and Plasma's
  derivation applies over the port's scheme (R2.6; the specification's S14 recorded the choice; ADD-A146); the
  platform's condition rules (KDE's inactive effects) apply over it;
- is suspended while either contrast condition holds, unless its project states colours for it (R2.4);
- applies only under the schemes it has (ADD-A146; kept at D6, outcome 12, ADD-A147, open to the maintainer's veto:
  veto list item 42). Where the application follows the system's scheme and that
  scheme is one the palette lacks, the API accepts the palette and suspends it: it reports that the palette does
  not apply, the theme is the preset with the reader's values, and the palette applies when the scheme returns,
  whichever came first, the palette or the scheme (it stays chosen, so the application need not watch the scheme to
  apply it or to re-apply it). The API refuses the palette only where the application itself picked a scheme the
  palette lacks (R2.3), two choices of the application's that contradict each other, as it refuses that pick under
  the palette; an application that wants a dark-only palette in a light system sets the colour scheme itself.
  Rejected: refusing it under a followed scheme too, the earlier rule, under which the same state (a dark-only
  palette chosen, the system light) was an error or a suspension by which came first, and an application restoring
  its saved palette at start-up in a light session had to watch the scheme to apply it later;
- carries no icon set (today's `icon_set = "lucide"` goes, as no project's colours state one);
- on `windows-11` and `macos-*`, whose accent-dependent colours follow derivations their makers do not
  publish (R2.6), states those derived inputs too, mapped to the project's colours in a per-preset section and
  labelled as native-theme's picks like any other (the alternatives: leaving the platform's own accent in place under
  the palette, which mixes two colour schemes, or refusing the palette on those presets, which drops it there); on
  `ios-*` the accent is the one accent input, and what UIKit draws in the tint follows the palette's accent by the
  measured derivations (R2.6; ADD-A146);
- over a preset whose widgets read keyed colours (`windows-11`'s button reads 24 keyed colour roles and no typed one,
  21 of them literals under the plain schemes), reaches them by a stated mapping method: each keyed colour role
  those widgets read that follows nothing the palette states, mapped once to the project's own vocabulary, the
  mapping reviewed; a palette without it is not offered over that preset (ADD-A147; D6, decided by logic, awaiting
  the maintainer's veto: veto list item 49; the method is designed with the palettes, the specification's S14).
  Rejected: typed roles alone, the mixed look recorded (the Windows button in WinUI's greys under the project's
  colours, the mix of two schemes this decision rejects); the keyed colours picked one by one for a veto (21 picks
  per palette and scheme for that button alone, ten palettes, thirty widgets to come);
- is not offered over a preset whose widget colours the mapping method cannot reach, measured literals that follow
  no role (the `macos-*` and `ios-*` bezels; `macos-tahoe`'s button fills are 2,561 measured literals of its 2,795
  fill lines): a recorded limit, stated with the palettes (the specification's S14; ADD-A147; D6, decided by logic,
  awaiting the maintainer's veto: veto list item 49). Rejected: offered there with the limit recorded (the mixed
  look this decision rejects, the platform's literals under the palette's colours);
- skips a role or surface set the current preset does not have, so one file serves every preset.

The facts gate checks a palette's colours against the project's published colours; its labelled picks are recorded as
such and not compared with any platform.

### R20 — Value kinds, and no model inheritance

Today a `soft_option` `None` has four meanings and ~91 inheritance rules fill unset fields beside the presets (§1).

- *Keep inheritance as base-layer default derivations.* Against: a second mechanism beside the rules, needing a place in
  the order; it is how `[wrong_safety_nets]` exist.
- *No model inheritance: a preset states every reference in its bases as a derivation* (`= the bound surface's
  foreground`), and the gate checks it. For: one mechanism; the reference is visible where it applies. Against: longer
  presets; today's rules are converted once, mechanically, into the presets.

**Decision:** no model inheritance; tree inheritance stays (glossary). A value is one of:

- *stated:* a value or a derivation (R5.4), references included;
- *automatic,* where the platform sizes by content (stated, so a toolkit sizes by content because the platform does);
- *absent:* the platform does not draw the thing; it resolves to none and a connector draws nothing. A field may be
  absent only where the model marks it so;
- *missing:* none of these, an error when the layer that leaves it so is added (R2.8).

"Draw another value instead" becomes stated values (where a platform draws a checked button with its pressed fill, its
{checked} rule states that fill); "no published number" is not a kind: it is measured.

### R21 — Compatibility and release

The model's public API changes and the seven crates are released together, so the release is **0.8.0**, carrying the
connectors moved to the new model (each mapping what its toolkit can draw and recording the rest, R23); the connectors'
feature milestones stay later.

**`#[non_exhaustive]`:** every model struct gets it, so a later field is not breaking; code outside the crate builds a
struct from `default()` and field assignment (the connectors, showcases and examples change accordingly). Enums a
connector must map in full do not get it, and `IconRole` loses it: state flags, conditions, value kinds, icon roles,
blend modes, timing kinds, glyph sources, placement strategies, transition kinds, theme-change kinds. A new variant then
breaks the connectors' builds, which before 1.0 is the check that they map it, rather than a wildcard arm that hides it.
The enums no connector maps, the error, diagnostic and status enums (a refused layer's problems, a reader's failure
and status, the platform diagnostics), get it (ADD-A146): an exhaustive match checks nothing where nothing is mapped,
and a new member of an exhaustive public enum makes its release a breaking one (the plan's D14), so one new
validation problem would turn a follow-up release that adds only keyed components or placement into a breaking one.
Rejected: every public enum exhaustive, the error enums included, the specification's wider reading, made for
simplicity before D14 gave it that price.
Enums with keyed counterparts are typed sets whose keyed side carries what is platform-specific (material identities are
keyed per platform, R6), so they need no wildcard either. Structs their caller fills in full (a window's conditions, the
root's inputs, a sibling, a placement's input, which a connector fills; the reader's state, which a reader or a
stand-in application fills) are exhaustive for the same reason as the enums: a field added later must reach every
caller's build, and a default would hide it.

**Custom looks.** Today an application can load a whole theme of its own (`Theme::from_toml`, `Theme::from_file`).

- *Keep loading a whole non-bundled preset,* validated like a bundled one. For: a company's own look in one file.
  Against: a look outside the bundled presets has no facts, so the gate (R4) cannot check it, and it must state every
  value the model carries (shadows, motion, materials, every state), which its author never defined: how today's
  community presets came to invent values (R19).
- *Custom looks as application layers over a bundled preset* (an application layer from TOML). For: one mechanism
  (R2.4); the preset supplies what the custom look leaves alone, and the gate still checks that preset. Against: a
  look that differs everywhere restates much in its layer.

**Decision:** the second, before 1.0 the simpler and the one that keeps the gate meaningful; `Theme::from_toml` and
`Theme::from_file` go.

- *Release in stages by capability.* Against: the rules touch every widget at once, each stage would break the
  connectors again, and a release with some capabilities filled shows mixed looks.
- *One release.* Against: a long branch.
- *0.8.0 at today's coverage, then the rest by units* (added by the maintainer's decision D13 of the 0.8.0 plan,
  2026-11-06): 0.8.0 holds every widget, colour role, token and value today's model holds, the catalogue's other
  units following in later releases. For: users lose no widget, colour role, token or value of 0.7.0's model but the
  invented light variants (R19), the `button` values 0.7.0 stated with no
  source, or against its own cited source, which the facts correct (D6's comparison, the plan's *The depth 0.8.0
  states*: the `macos-*` and `ios-13` hover fills, `kde-breeze`'s inherited primary, `adwaita`'s text colour
  `#2e3436`, the unsourced sizes and icon gaps, the `macos-monterey` pressed fill, the `ios-13` pressed and disabled
  fills, the `windows-11` disabled fill); the values 0.7.0 took from a source the preset's reference rule does not
  choose for it (R1.6 for `material-3`: the older `v0_192` token set's disabled container, and material-web's
  component's vertical padding and minimum width where Compose has the button; R1.3 and R1.4 for `ios-13`: WebKit's
  HTML button's pressed label against UIKit's), which the facts restate by that rule; the look of a button with no
  variant on `material-3` and `ios-13`, which draws the platform's default button (Compose's `Button`, the filled
  one; UIKit's system button), 0.7.0's look staying under `tinted` and `@gray` (0.7.0 mapped its plain button to
  Material's tonal button and to WebKit's HTML button, neither the platform's default: `androidx@e4d933c9
  …/material3/Button.kt:77-88` against `FilledTonalButton`, `:439-447`; `xcode@26.6 UIKit/UIButton.h:22-23`);
  `material-3`'s checked text-button colours (0.7.0 stated the tonal toggle of Material's latest token set; baseline
  Material draws a toggle only as an icon button, the content unit's, and a text toggle under the `expressive`
  flavour, the flavours unit's, where Compose 1.5's Expressive `ToggleButton` brings it back); of the two motions
  0.7.0 has, one the Apple motion trial does not measure (the plan's M2, task 1): the dialog's entrance kind
  (`adwaita`, `kde-breeze`, `windows-11`, `material-3`), or the busy indicator's loop as theme data (every preset;
  the icon sets' frames on `adwaita`, `kde-breeze` and `material-3`) (ADD-A147; D6, decided by logic, awaiting the
  maintainer's veto: veto list items 5, 6 and 32 to 39); the icon size of a context the platform names none for,
  which 0.7.0 stated on every preset (the catalogue's absence table for `token.icons.sizes`: `material-3`'s `small`
  and `large`, 0.7.0's 16 and 32 with no source; `adwaita`'s `dialog` and `panel`, 0.7.0's 22 and 20; on `macos-*`,
  `windows-11` and `ios-*` as far as the register's U102, U516 and U127 find); the icon theme's name on the presets
  whose icon set is not freedesktop, where 0.7.0 repeated the set's name (`material-3`'s `"material"`), the name
  staying in `token.icons.set` (ADD-A147; D6, decided by logic, awaiting the maintainer's veto: veto list items 7
  and 8); on `material-3` the warning, success and info colours and their text colours, which 0.7.0 stated with no
  source (`#7d5700`, `#386a20`, `#6750a4` on `#ffffff`, and the dark scheme's;
  `v0.7.0:native-theme/src/presets/material-3.toml:29-34`, `:548-553`), Material drawing none of those meanings, and
  on `macos-*` and `ios-*` a status colour the ledger's U757–U760 close with no witness, each held by the connector's
  contract as the toolkit's own colour (the specification's S3, S21.3; ADD-A147; D6, decided by logic, awaiting the
  maintainer's veto: veto list items 56 and 57); and the widgets a preset declares unavailable where 0.7.0 resolved them (`tab`, `card`, `expander`,
  `status_bar`; ADD-A146), which a connector draws by its contract's list (the specification's S21.3), a widget
  being 0.8.0's on every preset whose platform draws it (ADD-A147; D6, decided by logic, awaiting the maintainer's
  veto: veto list item 48), what 0.7.0's API holds beyond them and 0.8.0 does not keep being the plan's *Not kept*, and get the
  new model weeks earlier; each
  release's units are whole in what they state: every part, field and value of their entries, in every combination
  of the names the release holds. What a unit does not hold yet is a named later unit's and cannot be asked for
  meanwhile: placement (their menus, popovers and dialogs open where the toolkit puts them, as in 0.7.0); the
  variants, sizes, host contexts, states and conditions beyond those 0.7.0's model and readers distinguish, each
  entering whole, on every preset, with a later unit of depth; and the state-change transitions (a state changes at
  once, as in 0.7.0). So no element is drawn from rules of two depths (ADD-A147; D6, decided by logic, awaiting the
  maintainer's veto: veto list items 1 and 2). Against: several releases, each with its gate run; a later widget that adds members to an
  exhaustive enum is a breaking release (the plan's D14); widgets 0.7.0 styles through other widgets' fields keep
  those stand-ins until their units.

**Decision:** the third (D13; until then, one release). v0.7.1 is this milestone's documents; the implementation runs
on its own branch (`v0.8.0-rc1`), staged by the specification's plan, and 0.8.0 is not released until every platform
preset, `ios-26` included, is complete for the units holding today's model, at the depth today's model distinguishes,
and passes the facts gate (R4); each later release enables only units, of widgets or of depth, complete on every
preset and passing the gate (ADD-A147; D6, decided by logic, awaiting the maintainer's veto: veto list item 1).

### R22 — The documents that follow the model

- **Platform facts become data.** R4's gate must parse a value per key, and `docs/platform-facts.md` is prose. The facts
  move into per-platform, per-area data files (a key per model path and selector, the value or derivation, the source
  kind and the citation), and the readable tables are generated from them. `scripts/check_platform_facts_rows.py` and
  `scripts/check_preset_citations.py` move to the data. iOS gets its columns (13–18 and 26). Keeping the prose and
  splitting it was rejected: a test cannot read it.
- **`docs/property-registry.toml` and `docs/inheritance-rules.toml`:** the registry stays, checked against the code by
  its drift test extended to the new types; the inheritance rules go with inheritance (R20).
- **The showcases' gates** (element lists, parity) follow the connectors in R21.

### R23 — A value a toolkit cannot draw

A connector maps every value its toolkit can draw and lists every value its toolkit cannot in its contract, with the
toolkit's lines that show it; it also builds what the query needs (the element tree, R2.5) where its toolkit holds none,
and supplies the window's conditions (R2.3). A toolkit's limit is recorded where it is; the model still carries the
value. This is the inverse of the failure that started this milestone, where the model's limit was recorded as the
toolkit's.

### R24 — Traceability

The specification maps every row of the map, identified by its file and ID, to the fields, rules, roles or components
that carry it, or to the decision that places it elsewhere: out of scope (R1.1–R1.4), another preset (R1.3), the newest
release (R1.5), a look the chosen reference does not draw (R1.6), the readers' milestone (R18). The rows are the platform files' gap rows; withdrawn rows and the 42
pointer rows are excluded, and the cross-check and pass decision tables are decisions, not rows. A script checks that
every row maps to at least one target. This proves the model carries everything the map found; what the map, a survey,
did not find is caught by R4's lists, which come from the platforms' own state models, and by the facts' independent
verification (R4's *What the gate proves*; ADD-A146).

### R25 — Decisions the specification's work required (2026-10-28; R25.16 and R25.17 by the specification's batches ADD-A127 and ADD-A130)

Writing the widget catalogue and mapping every row found mechanisms no decision above carried, and facts that
refine decisions above. Each is decided here, with the decision it extends.

**R25.1 — Plasma's accent (extends R2.6).** Plasma derives every accent-dependent colour once, when the accent or the
scheme is applied (`plasma-workspace/kcms/colors/colorsapplicator.cpp`, `applyScheme()`, at v6.8.2), and writes the
results into kdeglobals; no application derives at run time. So the KDE reader reads the results, and on `kde-breeze`
the accent is the only accent input: the preset's own bases state Breeze's scheme as Plasma's default writes it, "Accent color from color scheme",
which applies no accent derivation (a Breeze blue chosen as the accent does not reproduce it), and when an
application changes the accent, under the reader's scheme or a
KDE port palette's (whose scheme inputs the port carries), the model applies that derivation over those inputs, a
function `kde_accent(scheme, accent)` implemented to `applyScheme()` (the accent copied into
`ForegroundActive` and the decoration roles of every colour set, the link colour adjusted for contrast, the selection a 0.7 blend over the
view background, the text on it black or white by KDE's contrast measure, the `[WM]` title-bar colours, from the
scheme file's `[WM]` or its `Header` colours and then tinted or accented by the next two options, `:287-338`, and the
scheme file's `TintFactor` and accent-coloured title bar options, which become tokens of the preset). Its `scheme` is the scheme file kdeglobals
names (`[General] ColorScheme`, found as `color-schemes/<name>.colors`), which the reader reads, since kdeglobals
holds only the derived values; where kdeglobals names none, or the file is missing, kdeglobals' own colours stand in
for it, and the reader reports that rather than failing, as failing would discard every other value it read. A group
or key a scheme file lacks is read as KColorScheme and `applyScheme()` read it unset: a scheme without a Header group
(Breeze Classic, Dracula's port) gets no Header written, accent or title bar included, and KColorScheme reads Header
from Window, so its title bar takes the accent through `[WM]` alone. A mapping palette (R19) has no scheme inputs, so it states its accent-dependent roles
itself, by reference to the accent where they follow it (R2.6). KDE's stated Inactive values (R2.6) are, in kdeglobals, Plasma's: since
Plasma 6.8.0, `applyScheme()` writes `[Colors:Header][Inactive]`, in a scheme that does not tint, as a copy of the
scheme's active `[Colors:Window]`, not the scheme's own Header Inactive values; in a tinting scheme it writes every
set's `[Inactive]` values, Header's included, as the scheme's own tinted toward the accent; and under the
accent-coloured title bar option it then writes Header's inactive background and foreground from Window's background
tinted toward the accent (`colorsapplicator.cpp:247-284`), where the scheme has a Header group. The inactive input
roles stay, filled from what kdeglobals holds.

**R25.2 — KDE and contrast (confirms R2.3).** One KDE row (K2.23†, the date picker) has code that drops
hard-coded colours under a high-contrast preference. Plasma reports none (the KDE portal's appearance keys are
`color-scheme`, `accent-color` and `reduced-motion`, the KDE map's C23 summary), so R2.3 stands: on KDE the contrast
condition is high only when an application sets it, and that branch is a rule under it.

**R25.3 — Element data (extends R2.5, R5.4).** Some values depend on data only the element has: its laid-out size (a
spinner's stroke as a fraction of its radius, GN-1.227; initials sized to an avatar, GN-2.14), its progress (a
duration that follows progress, Q2.3), its scroll position (values that follow scrolling rather than time: a carousel,
a navigation bar collapsing, GN-2.55†, MAT-N18, C-1), its index and count among items (a chart series' colour,
K-P3-10†, C-45†), and a text whose hash picks a colour (an avatar's colour from its name, GN-2.14, Q2.28). Options:
leave them to the connector (against: the platform's rule would be lost), or a typed set of **element data** the
connector supplies in the query (size, progress, scroll offset and progress, index and count, a text), which
expressions may name, with the functions the platforms use over them (`pick(hash(text), list)` to the platform's own
hash and list, cited). Also element data: a colour the application chose
for the element that derived values follow (a badge's custom colour, Q2.26), the width of a character of the part's
font as the toolkit measures it (GTK's `max-width-chars`, KMessageBox's widths in characters, GN-1.283†, K-P3-7†),
the font's own underline and strikeout position and thickness, from which the target toolkits draw those lines
(Pango, AppKit's `NSFont`, DirectWrite, Android's `Paint`; Qt draws its own adjustment of them, a register row,
R25.17, the specification's S2.8), so a decoration follows the user's
font rather than one measured number (R10),
and the display's corner radius, a root input rather than per-element data (R25.11). A value may also follow another resolved field of the same part (Compose's
optical centring by the corner radius, MAT-X37†). **Decision:** element data in the query, as plain values each part
declares it reads, so the query still cannot fail (R2.8); the connector supplies them and queries again when they
change, as for a state; a part's own other fields are a reference, under the cycle check (R5.4). The order in which
the connector set the element's state flags (R3's recency) is element data too: the connector records when each flag
became set, beside the previous query results it already keeps per element to compare them (R9), as an
immediate-mode toolkit recomputes the flags every frame. It is carried beside the flags it orders, not among the expression operands: selectors read it, as they
read the flags, and no expression does; and it needs no declaration, as flags it leaves out count by the recency
set's stated order, so a connector that reports none gets a defined answer.

**R25.4 — Pattern paint (extends R6).** Some fills are patterns no gradient makes: a colour well's transparency
checkerboard (GN-2.53†, W2-42†), Qt's dense brush patterns (K-SF09†), a code editor's background grid (GN-SF11†),
libadwaita's developer-build stripes (GN-1.10). **Decision:** a paint layer kind for patterns, a closed set each
defined to its platform's source (checkerboard, Qt's brush patterns, grid), plus repeating gradients (Breeze's busy
stripes, K1.110) and a conic gradient (the activity rings, C-46†, as CSS's `conic-gradient`); never an image drawn by native-theme. libadwaita's developer stripes are its own SVG file
(`devel-stripes.svg`); the value phase reads what it draws and states it with these kinds, and a platform image a
pattern cannot state is a finding for the rationale.

**R25.5 — Shapes beyond corners (extends R7).** Material Expressive draws polygon shapes (its shape library: cookie,
clover, …) and morphs between them (MAT-N24, MAT-X12), which corners cannot express.
**Decision:** a part's shape is its corners or a typed shape from the platform's shape library, implemented once in
native-theme to the platform's source (`androidx.graphics.shapes` and Material's `MaterialShapes.kt`, cited at a pinned
commit), as placement strategies and platform curves are code (R16, R9): native-theme computes the outlines and the
morphs between them, which needs a closed set, and a keyed shape would name an outline no code draws. A shape change
is a transition of kind morph; a loop's keyframes may name shapes.

**R25.6 — Presentation transitions (extends R9).** Several transitions may run on one event together (a sheet's fade
with its slide, Q2.40), one may target a single part (a clear button's fade), a slide states its distance, and
items may be staggered (Q2.14, Q2.15, Q2.52). **Decision:** an event's presentation is a set of transitions, each with
an optional part, distance and stagger; R9's platform curves (the curves a toolkit computes by a function of its own,
the velocity-dependent ones among them) are implemented in native-theme (the specification's `Timing::Platform`).

**R25.7 — Text rendering (extends R10).** Antialiasing, hinting, subpixel order, LCD filter and gamma or contrast
(ClearType's, FreeType's through fontconfig, GNOME's `font-antialiasing`, `font-hinting`, `font-rgba-order`, macOS's font
smoothing) are visible and user-set. **Decision:** a text-rendering token the readers read and toolkits apply where
they can (R23).

**R25.8 — Icon variants (extends R11).** SF Symbols have variants (`.fill`, `.circle`) and localized and right-to-left
forms; GTK looks up `-rtl` and `-ltr` icon names. **Decision:** an icon style names a symbol variant, and icon lookup
takes the element's language and direction.

**R25.9 — Selectors (extends R2.3).** The rows need: position counted among siblings of any kind or of a stated kind
(GTK's `.linked` groups buttons and entries together, against R2.3's "of the same widget kind"), parity (Breeze's alternate
rows, Q1.77), siblings matched by part (separators beside a selected tab, KDE's per-cell rounding), and a context
matched by variant alone, whatever the ancestor's widget (libadwaita's `.osd &`). **Decision:** all four, in the
position and context parts of the selector; position counts siblings of the same widget kind unless the rule states
any kind or a set of kinds (R2.3's default stays: tabs and segments count each other, and a separator is a part).

**R25.10 — Sizes (extends R15, R16).** Material Expressive has five button and slider sizes; `ControlSize` gains
`ExtraSmall`, and each preset's facts say which of its sizes fills which slot. Size classes have two axes, horizontal and
vertical, as iOS's do.

**R25.11 — Scope details (extends R1.1, R1.2).** libspelling (GN-P3-27†) passes R1.2's test and is in. The
`adwaita` preset carries GtkSourceView's Adwaita and Adwaita-dark schemes, which GNOME Text Editor sets, and
`classic`, GtkSourceView's default for an application that sets none (GN-SF01†), as a keyed variant. Drawing a display's corners is the system's
(R1.1, out), but its radius is what a full-screen container's concentric corners follow (W-6, MAT-W5, X-CR-4): it is
a root input the connector supplies where the platform reports it, the radius of the root's container (Android's `RoundedCorner`), and the preset's
measured value per device model where it does not (iOS states none publicly; the radius differs between models
of one idiom, W-6), absent for a model the preset does not list.

**R25.12 — Symbol effects (confirms R11).** An SF Symbol effect triggered by a change is a transition of a keyed
kind; one that repeats while a state holds is a loop. A third route (an "effect" on the icon style) is not adopted.

**R25.13 — Details inside decided mechanisms.** The catalogue also needed conditions, flags, placement strategies,
functions, block fields and token fields inside the mechanisms above; the specification lists them with their rows in
its additions register (S22), and each function there is implemented to the platform source it names (R5.4).
Three of them refine decisions above: a platform that computes a stored colour by composition (KDE's
`overlayColors`) gets a function, while composition drawn over what lies beneath stays a paint layer (R5.4); the font
fallbacks are one token, the platform's chain, not a field of every font (R10); and selected items take the `Selection`
surface set's roles while selected text has its own roles, which a platform that does not distinguish them states as
references to that set (R12). One refines R2.4: the application's accent is a setting applied after its layers and a palette,
derived again on every change of the stack, so the accent shown never depends on the order of the calls.

The amendments made in place, in the order made (R25.14 to R25.17 follow this list).

Amended in place after the specification's eighth check:

- R5.1: the Selection set's disabled and inactive groups start from Window's colours, as `kcolorscheme.cpp:304-319`
  does, and the effects apply over them.
- R2.8, R5.4, R20: a missing value or a cycle is an error when its layer is added, which is refused, not an error at
  resolution.
- R21: structs their caller fills in full are exhaustive, as the enums are.
- R21: custom looks are application layers over a bundled preset; loading a whole non-bundled preset goes.
- R25.5: a shape is typed and implemented in code, not keyed.

Amended in place after the specification's ninth check:

- R10: point sizes become logical pixels at the font DPI, a preset token the reader overrides where the platform has a
  setting, before text scaling; a change of it is a theme change.
- R2.6: on `kde-breeze` the preset states the accent-dependent roles as Plasma's derivation over Breeze's scheme
  inputs; a KDE port palette carries scheme inputs, so the derivation applies over it; a mapping palette states those
  roles itself and is refused where it leaves one to the preset.
- R25.1: the same, in Plasma's accent decision.

Amended in place after the specification's tenth check:

- R1.1: "system dialogs a toolkit only invokes" are those another process draws (GNOME's print and app-chooser
  dialogs through the portal, the portal file dialog drawn by Files); a dialog the toolkit draws in the application's
  process (Qt's `QPrintDialog`) is in, built of the toolkit's widgets.
- R2.6: KDE's inactive input roles are stated in the preset as Plasma's derivation, replaced by the reader or a KDE
  port, and stated by a mapping palette itself (refused where it leaves one to the preset), as the accent-dependent
  roles are; their base is no longer "the active value".
- R19: "of the ten projects" corrected to the seven projects behind the ten community presets.

Amended in place after the specification's eleventh check:

- R2.6, R25.1: what `applyScheme()` writes into `[Inactive]` groups stated once, from `colorsapplicator.cpp:247-284`.
- R2.8: an operand that can be automatic is refused where arithmetic would need a number.
- R25.3: the font's underline and strikeout metrics are element data, so decorations follow the font.
- R25.1 (the eleventh check's catalogue round): Plasma's derivation covers the `[WM]` title-bar colours
  `applyScheme()` writes too, so the model applies it to KDE's `[WM]` keyed roles as to the colour sets.

Amended in place after the specification's final (twelfth) check:

- R2.6: which colour sets have an Inactive group is a stated scheme input; a KDE port's inactive input roles follow
  `applyScheme()`'s rules, as the preset's do, not the raw file.
- R9: a scroll deceleration is one of a closed set of platform models, each implemented to its source.
- R10: letter and word spacing are scaled by the line height's rule.
- R16: a part's corners, stroke edges and shadow offsets are physical in right-to-left; only the layout's geometry and
  a translation mirror.

Amended in place after the specification's confirming (thirteenth) check:

- R2.6, R25.1: a scheme file's missing colour groups and keys are read as KColorScheme and `applyScheme()` read them
  unset (a scheme without a Header group gets no Header written, its title-bar accent `[WM]`'s alone); another
  platform's reader's accent on `kde-breeze` is derived like the application's; a non-tinting port's inactive title bar
  is unchanged by an accent only without the title-bar option.
- R25.1: where kdeglobals names no scheme file or the file is missing, kdeglobals' own colours stand in and the reader
  reports it, instead of the whole read failing.
- R4: the gate also sets accents on `kde-breeze` and compares Plasma's derivation with facts measured from Plasma.
- R9: each scroll deceleration model carries the parameters its source defines, not one number.
- R16: the query returns the mirrored values physical, so no connector mirrors again; a transform's origin is physical.

Amended by the maintainer's decision D12 of the 0.8.0 plan (2026-10-29):

- R1.5: a preset describes the newest release a supported runner measures, so `macos-tahoe` and `ios-26` describe
  26.x while macOS 27 runs only on a preview image; what only 27 draws is carried to `docs/todo.md`'s macOS 27 entry.

Amended in place by the specification's batch ADD-A124 (2026-10-30), decided from the sources in the ledger's queue
#42, #49, #60 and #73:

- R3: recency sets and a selector term on a set's latest flag (CONTRA-6, settled); R25.3: the order the connector set
  the flags in is element data.
- R7: the elliptical corner, Breeze's round button on a box that is not square.
- R9: a transition's `from` states; the platform curves widened to the curves a toolkit computes by a function of its
  own (CONTRA-7, settled; CONTRA-2's velocity curves stay members), R25.6 following.
- R3, R9 (the independent checks' fixes): Material's state layer and press ripple a block of the part,
  `interaction`, with groups of its own (replacing paint layers with roles, which needed a compensating rule each
  round); Compose's state-layer fades by `from`; Breeze's zero transitions `from {disabled}` and
  none under Kirigami (qqc2-desktop-style's style item paints with no widget); the press ripple's
  effect (Android's patterned `RippleDrawable`, a closed set of effects implemented to their sources, the ripple keeping
  its pressed colour); the ripple a field of the block, `ripple.effect`, not a transition; the press delay a touch behaviour
  (`token.behavior.touch.delay`); the checkbox's, radio's and switch's
  `state_layer` parts holding the block, their per-state opacity dropped; Material's navigation items' ripple pill as a part of its own; the stroke's widths a
  property group that overrides `Stroke` where stated; a stroke becoming absent interpolating to zero width.

Amended in place by the specification's batch ADD-A128 (2026-10-31), decided from the sources in the ledger's queue
#105:

- R5.4: libadwaita's standalone colours are the colour GTK computes for its stylesheet, unclamped, one function for
  every role; a colour outside the gamut is clamped where it is drawn, premultiplied first, the connector's hand-over
  following GTK's.

Amended by the maintainer's decision D13 of the 0.8.0 plan (2026-11-06):

- R21: 0.8.0 is released when the new model holds everything today's model holds, on every platform preset, with the
  connectors on it; the catalogue's other units follow in later releases, each whole (less the values a unit defers
  to a later one, 0.8.0's placement) and passing the gate. "Everything
  visible" stays the goal; only when each part is released changes.

Amended in place by the specification's batch ADD-A146 (2026-11-09), the fixes of the independent document check
of that day:

- R2.3: KDE's speed factor of zero sets both motion conditions, reduced motion and animations off, as the
  specification's S9.3 and the reader have it.
- R2.6, R4, R19: `ios-*` declares the accent alone; what UIKit draws in the tint is a measured derivation over it,
  pending the host pass under a second tint (the unmeasured register's U706), so `ios-*` leaves the presets whose
  derived accent inputs a palette or an application states (a recorded decision reversed, kept at D6, outcome 12).
- R4, R24: what the gate proves now that the presets are generated from the facts; that the facts hold every
  combination and derivation is the independent verification's work.
- R6: GTK's and libadwaita's state layers are the appended paint layers; a layer Material's tokens state is the
  `interaction` block's, as R1.6 and the specification's S2.12 have it.
- R2.7: a keyed component's declaration has the keys a widget's has, with its values' types.
- R9: a spring Apple states by response and damping fraction or by duration and bounce has no pinned conversion
  (U128); no source read states a theme change's crossfade, which a preset states only from a measurement; a CSS
  animation among the starts that are no value change, and the engines' list brought to the specification's
  (`ComposeShapeMorph`, `CssAnimation`).
- R10: a reader reports a font DPI only where its toolkit's scale factor does not already carry the setting, so none
  does on an X11 session; points still convert at the platform's DPI, and the user's setting reaches the application
  through that scale.
- R11: the AppKit rendering's cache is emptied at every theme change the reader reports, and its key holds the
  layout direction.
- R12: the focus indicator's kinds are the specification's three (a ring, an underline, a keyed kind).
- R16: the section's opening follows the thirteenth check's amendment: the query mirrors, not the toolkit.
- R19: a variant defined, a set of the project's base colours under one name with one of the accents the project
  publishes for it, so Catppuccin's are 56 and its four bundled palettes the variants with 0.7.0's accents; a
  palette under a followed scheme it lacks is accepted and suspended, and refused only under a scheme the
  application picked (a recorded decision reversed, kept at D6, outcome 12); under
  a KDE port the reader's accent stays in force, as the specification's S14 recorded.
- R21: the error, diagnostic and status enums, which no connector maps, are `#[non_exhaustive]`; what 0.8.0 keeps
  of 0.7.0 is its model's widgets, roles, tokens and values but the invented light variants (R19), the `button` values 0.7.0 stated with no source, or against its own cited source,
  which the facts correct (D6's comparison, the plan's *The depth 0.8.0 states*: the `macos-*` and `ios-13` hover
  fills, `kde-breeze`'s inherited primary, `adwaita`'s text colour `#2e3436`, the unsourced sizes and icon gaps, the
  `macos-monterey` pressed fill, the `ios-13` pressed and disabled fills, the `windows-11` disabled fill); the values
  0.7.0 took from a source the preset's reference rule does not choose for it (R1.6 for `material-3`: the older
  `v0_192` token set's disabled container, and material-web's component's vertical padding and minimum width where
  Compose has the button; R1.3 and R1.4 for `ios-13`: WebKit's HTML button's pressed label against UIKit's), which
  the facts restate by that rule; the look of a button with no variant on `material-3` and `ios-13`, which draws the
  platform's default button (Compose's `Button`, the filled one; UIKit's system button), 0.7.0's look staying under
  `tinted` and `@gray` (0.7.0 mapped its plain button to Material's tonal button and to WebKit's HTML button, neither
  the platform's default: `androidx@e4d933c9 …/material3/Button.kt:77-88` against `FilledTonalButton`, `:439-447`;
  `xcode@26.6 UIKit/UIButton.h:22-23`); `material-3`'s checked text-button colours (0.7.0 stated the tonal toggle of
  Material's latest token set; baseline Material draws a toggle only as an icon button, the content unit's, and a
  text toggle under the `expressive` flavour, the flavours unit's, where Compose 1.5's Expressive `ToggleButton`
  brings it back); of the two motions 0.7.0 has, one the Apple motion trial does not measure (the plan's M2, task 1):
  the dialog's entrance kind (`adwaita`, `kde-breeze`, `windows-11`, `material-3`), or the busy indicator's loop as
  theme data (every preset; the icon sets' frames on `adwaita`, `kde-breeze` and `material-3`) (ADD-A147; D6, decided
  by logic, awaiting the maintainer's veto: veto list items 5, 6 and 32 to 39); the icon size of a context the
  platform names none for, which 0.7.0 stated on every preset (`material-3`'s `small` and `large`, `adwaita`'s
  `dialog` and `panel`, and on `macos-*`, `windows-11` and `ios-*` as far as the register's U102, U516 and U127 find);
  the icon theme's name on the presets whose icon set is not freedesktop, where 0.7.0 repeated the set's name
  (ADD-A147; D6, decided by logic, awaiting the maintainer's veto: veto list items 7 and 8) and the widgets a preset
  declares unavailable where 0.7.0 resolved them (`tab`, `card`, `expander`, `status_bar`), which a connector draws
  by its contract's list (ADD-A147; D6, decided by logic, awaiting the maintainer's veto: veto list item 48), the
  plan listing what its API does not keep.
- R25.16: on `macos-*` the pop-up's arrow area is placed from the resolved values, the rendering's image stating
  none (the specification's S15).

Amended by the 0.8.0 plan's checkpoint D6 (2026-11-10, outcomes 1, 2, 3, 5, 6, 7, 12 and 14; the specification's
batch ADD-A147; D6, decided by logic, awaiting the maintainer's veto: veto list items 1 to 8, 32 to 40, 42, 48, 49,
54 and 56):

- R21: a unit enters at the depth 0.7.0's model and readers distinguish, its further variants, sizes, host contexts,
  states and conditions in later units of depth; 0.8.0 holds the two motions 0.7.0's model has, the dialog's
  presentation and the busy indicator's loop, with the speed factor and the reduced-motion conditions, each if an
  Apple trial measures it on the Apple presets, and state-change transitions are the motion unit's; the invented
  values D6's comparison finds join R19's on the list of what users lose, and so do `material-3`'s checked text
  toggle, until the flavours unit (veto list item 5), a motion the Apple trial does not measure (item 6), and, by
  the register's rows the checkpoint settled, the icon sizes of contexts a platform names none for and the icon
  theme's name on the presets whose icon set is not freedesktop (veto list items 7 and 8).
- R4: the gate checks the combinations of the names its enabled units hold.
- R2.7, R15: a variant, size, density or flavour the platform has and a later unit states is answered 'later', never 'not
  available'.
- *Today's model*: 0.7.0's accessibility has no preset values behind it, but its readers write high-contrast values
  into the theme and its connectors honour reduced motion, as its code does (veto list item 40).
- R2.8: a bundled preset whose `[pending]` table is empty, a bundled palette and the layers `Theme` derives from them
  alone are checked by the crate's own tests at every commit, and a theme runs neither the preset's own checks nor
  the checks over combinations of those stacks again when it loads them, the checks of each layer above the preset
  running at every change; what the system
  and the application add is checked when added, over every layer whose stack, as the checks read it, the addition
  changes, and a system reading with every layer above it, a bundled palette among them, where a change alters what
  the reading leaves out (veto list item 4).
- R5.1, R10, R25.17: a typed role no source of the platform states is native-theme's labelled mapping where the
  platform draws its meaning in a colour of its own, a text slot the platform names no style for by a style or family
  it names (the specification's S3; veto list item 3); where the platform draws the meaning in no colour the role is
  absent and the connector's contract records the toolkit's own colour there, an exception to following the theme's
  data (veto list item 56), `material-3`'s warning, success and info colours a loss R21 lists; on `adwaita` the info
  role's fill and text colour refer to the accent's; on `macos-*` and `ios-*` the statuses wait on the ledger's
  U757–U760, so 0.8.0 keeps the Apple status colours where a row pairs them (R21; veto list item 57).
- R19: over a preset whose widgets read keyed colours, a palette reaches them by a stated, reviewed mapping method,
  or it is not offered over that preset; nor is it offered over a preset whose widget colours are measured literals
  that follow no role (`macos-*`, `ios-*`), a recorded limit (veto list item 49).
- R21: a widget is 0.8.0's on every preset whose platform draws it; where a preset declares unavailable a widget
  0.7.0 resolved, a connector draws its toolkit's widget by its contract's list (veto list item 48).
- R2.6, R4, R10, R19, R21: the five amendments of ADD-A146 stand (outcome 12; veto list item 42).

**R25.14 — Orientation.** Vertical widgets restate their geometry under a `vertical` variant; an axis-relative
convention would halve those rules but adds a second way to state geometry, so it is not adopted.

**R25.15 — Text scaling where no reader reads it (amends R10).** R10 has readers read each text style's size where the
platform's scaling is not one factor (Dynamic Type, Android's non-linear scaling). Until the iOS and Android readers
exist (R18), an application on `ios-*` or `material-3` would have to state every style's size itself. So the preset
states its scaling function, Android 14's `FontScaleConverter` (MAT-X19) or Apple's published Dynamic Type table,
applied to the stand-in's factor or content-size category; a reader that reads each style's size
reports a factor of one, so nothing is scaled twice. The alternative, the application stating every size, moves a
platform rule into every application.

**R25.16 — The combo box's content layout (extends R16; ADD-A127).** Every platform lays its combo trigger out as
one row, [icon] [the content's space] gap [the arrow], the content start-aligned and taking any spare width, the arrow
keeping the end; they differ in four ways the model must state, each surveyed at the pins form by form (Breeze's and
Kirigami's editable and non-editable combos with and without an icon, GtkDropDown and GtkComboBox with and without an
entry, WinUI's at both densities and editable, Compose's and material-web's select with and without a leading icon,
the iOS pop-up button, AppKit's pop-up and combo box). The catalogue's `combo_box` entry states the result once.

*The row.* Options: a rule of the combo box's own; `button`'s content form with the arrow as `button`'s
`menu_indicator` under `Inline`; that form with the arrow area as the end part where one is drawn (ADD-A125's).
The second is chosen: it places the icon, takes written values one to one (WinUI's `ComboBoxPadding` end 0 and its
chevron's `0,0,14,0`; Breeze's glyph centred in its 20 px square as a margin of 5; Compose's 24 dp icon in its 48 dp
box as a margin of 12, with the text's own end padding 4 the gap), and needs only `margin` on the arrow and icon and
`padding` on the label (GTK's cell renderer's 2 px, which in the body's padding would break S2.11's GTK minimum, an
expression over that padding; Breeze's text pixel). The third fails WinUI, whose chevron sits in its 38 px column by
its own margin, not centred in the 30 px overlay that is its editable arrow area, and Breeze, whose edit field runs
2 px into the arrow's square rather than ending before it. So the arrow area, where drawn, is a box at the body's end
by its own `margin`, `min_width` and `max_height`, taking no layout space (each platform's arrow column is already
the gap, the arrow's margin box and the padding), and the arrow is placed by the row whether or not an area is drawn;
on `macos-*`, where the rendering replaces the arrow, its arrow area, placed from the resolved values (the
specification's S15; ADD-A146), is the end part.

*The cross axis.* WinUI centres its chevron on the whole row under an uneven padding (`12,5,0,7`), Compose its
trailing box on the field's height; `Inline` centres on the content box. Options: `Inline`'s with WinUI's chevron
given margins 1 and −1; the body's box. The body's box keeps written values and agrees with `Inline` under every even
padding; Breeze's 20 px minimum content height, which under it no longer reaches the padding, is `body.min_height`,
stated from its own terms.

*The natural width.* A size hint is visible. Breeze sizes the content plus 38 but its drawn row holds 28: 10 px
(the end `ComboBox_FrameWidth` and `Button_ItemSpacing`) are sized and never placed, 11 with an icon, 8 editable, 14
frameless editable; Kirigami's style item 10, 7 with an icon, and editable a larger-of between its style item and its
text field's own width. Options: (a) the gap widened by the slack (natural width and positions right, a long text cut
10 px early); (b) the gap as drawn and the slack dropped (the width 10 px short at every size); (c) the slack in the
label's padding (cut early); (d) the slack as the arrow's margin moved under `Inline` (start d, end −d), which keeps
the row's size and moves the box: exact for one measure, but Kirigami's editable width is the larger of the style
item's (with the slack) and the text field's (without), which a margin on the arrow, counted in both, cannot give,
and Breeze's arrow would carry size terms where its source writes a centring; (e) a `min_width` over the content's
width, which S8 forbids; (f) a value, `content_slack_px`, added to the content's space at its natural width only. (f)
is the smallest form exact for every surveyed form: the positions and the cut stay the drawn row's, the size the
platform's. Under the widest-item basis the slack differs by form (Breeze 10 without an icon, 11 with one) and Qt adds
an icon's `iconSize + 4` per item, so the measure is the largest of whole rows, one per item (and for the
placeholder and the empty run), each in its own form with that form's slack: an item's icon is counted once, by its
own row, and a list mixing iconed and plain items comes out as Qt sizes it; measuring texts and adding the current
item's icon would count an icon twice or for the wrong item. The slack is nonzero only under that basis, so it could
be a field of `Widest`; it stays a value of its own because it varies by form (with an icon, editable, frameless,
under Kirigami) and a rule restating one length is smaller than a rule restating the whole basis with its floor, run
and `or_shown`, and because each whole row reads it under its own form's rules, as it reads the gaps; it is stated 0
under the other bases. What the content's space is measured at differs too, visibly: the shown text (GtkDropDown, WinUI), the
shown text with the placeholder and the floating label (Compose), the widest item, each in its own form, with the
placeholder (QComboBox, GtkComboBox; Kirigami at least 75 and, editable, beside its text field), and a fixed width
(GtkComboBox's entry, 150 px of text); one basis for all would misstate four platforms, and leaving it to the
connector would lose a platform rule (R25.3's argument), so `content_width` names it, with QComboBox's empty list
(seven advances of `x`) as data. When a toolkit measures again (Qt's default takes the width at the first show) is the
application's policy, not the theme's. Compose measures its resting floating label without the text's padding, and
bounds the box by a floated label plus the field's content padding on both sides, interpolating between the two as
the label floats (the outlined field), while its filled field pads its label as its text and measures it at its
current, interpolated size, a third shown text (`TextFieldImpl.kt:423-432`, `:1950-1951`): options, one form for
both (the outlined field's 20 dp too wide at rest, 8 with a leading icon, or the filled field's label short by the padding), a separate
value for the label's padding (a value read only by one basis), or the padding inside `ShownAndHints` as
`label_padding`, `Some` for the outlined form and `None`, the label a third shown text, for the filled; the last is
the smallest exact form. So is a measure that depends on
the element's history (Kirigami keeps its
floor of 75 for a combo emptied after holding items, while one created empty measures nothing,
`kquickstyleitem.cpp:126`): the model measures the element's present content.

*The natural height.* Compose centres its 48 dp icon boxes on the field's height and takes the larger of those boxes
and its text with its vertical padding (`TextFieldImpl.kt:1709-1745`, `:1786`, `:1904`), 56, where boxes inside the
padding would give 80; Breeze centres its icon in the edit field and reserves Qt's `iconSize + 2` inside the frame.
Under the widest-item basis the height is the tallest row's too, as Qt reserves an icon's height when any item has
one (`qcombobox.cpp:634-635`), and Kirigami's style item ignores the icon's height (`kquickstyleitem.cpp:1189`), its
icon taking no height by its margins. Options: the icon in the content box (Compose's height wrong); the icon's vertical margins on Material restated
smaller (its centring then off under the floating label's uneven padding); a value choosing the content box or the
body's box per preset; the icon, as the end part, always on the body's box, Breeze's top and bottom margins 7 (Qt's 1
and the frame's 6). The last is chosen: Breeze's even frame centres the edit field on the body, so its icon lands on
the same pixels, the 7 keeps Qt's reserve on the frameless editable combo too, whose field has no frame, and no
surveyed form needs the content box for the icon, so the value would decide nothing. Breeze cuts the icon at its edit field
(`breezestyle.cpp:5800`; editable QCommonStyle's, `qcommonstyle.cpp:2193`), the widget's box less its frame of 6, 0 on
the frameless editable field, reached only under Kirigami, whose content (`fontHeight + 2`, at least 20,
`breezestyle.cpp:3363`) can be shorter than its icon. Options: the cut at the content box (wrong under Kirigami's
editable combo, whose padding is qqc2's 5); at `body.padding` (the same value, and 6 where the frameless editable clip
is 0); no cut (a tall icon drawn past the field, as Breeze never draws it); a value, `icon_clip_inset_px`, the inset
from the body's box. No stated value gives the field's inset on every form, so the value is the smallest exact form.
The editable forms' inputs take
the vertical padding that makes the height Qt's (`fontHeight + 2` inside Breeze's frame, `qcombobox.cpp:632`,
`breezestyle.cpp:3366-3370`): 1 at the top and bottom on Breeze, 2 under Kirigami inside qqc2's 5, the frameless
editable combo keeping the 6 of the framed one; their text is centred, so nothing drawn moves.

*The editable input.* Breeze, Kirigami, Compose and GTK place the input in the content's space beside the arrow
(Kirigami's editable end padding of 24 is the 2 px gap, the 20 px square and the 2 px frame, so the row counts the column
once); WinUI's text box spans the whole body with `11,5,38,6`, its padding holding the arrow's column, and appears
only while the combo is focused, or under a game controller from the popup's opening until focus leaves, which the
label's absence rules state, the input drawn where the label is not (the game controller's history, the box staying
after the popup closes, approximated by the popup's `open`). Options: WinUI's input in the row with its padding restated (its written 38 becoming
an unwritten 1); `input_placement`, `Row` or `Body`. The second keeps every written value and counts each column
once.

*Left to measurement:* the macOS pop-up's and combo box's content row per control size and the iOS pop-up's width
basis, spare width and cut, which no pinned source states (register rows); a measured result outside these forms
(a packed iOS title and indicator) is a model change its row records.

**R25.17 — The slice's model gaps (2026-10-31; amends R4, R5, R10, R11, R12, R17; the specification's ADD-A130).**
Stating every field of `adwaita` and `macos-tahoe` (M1a) found fields the model could not state from the sources.
Each is decided from the pinned sources; the specification cites them.

- *The caret's width.* GTK's stem is the caret's height times the aspect-ratio setting, plus one, in whole pixels,
  and the height is the run's extents rounded outward to pixels, unhinted under GTK's default automatic font
  rendering, so it depends on where the baseline falls and changes from line to line. Options: a number in the token
  (invented: GTK states none); a token expression evaluated where used, as a text style's `font_size` is (the caret
  has no font, so it would read another part's, which a token cannot name); the token absent where the width follows
  the line, each caret part stating the expression over a font metric (the font height is off by one where the
  rounding crosses a step, the line height holds leading the caret leaves out); the token absent and each caret part
  stating the expression over the height of the caret its toolkit draws, element data the connector reports (one home
  per part, checked: a part keeping the absent token as its base is refused). **Decision:** the last.
- *The wheel step.* GTK's step is a power of the view's page size, already a `scroll_view` value; the lines and
  pixels per step read by KDE, Windows and Android are read only by widgets of later units (the scroll view, Qt's
  slider). Options: keep both in the
  behaviour record and state them now (their readers come later, and GTK's absence would be checked by nothing);
  stage the two fields one by one (the step's distance apart from its duration and from the touchpad's factor); move
  them into the scroll physics, which is staged with the scroll view already. **Decision:** the last.
- *Drag start.* Qt's start-drag time starts a drag from a press held still; GTK's 100 ms is the earliest a drag that
  movement starts may begin. Options: one field with either meaning (the other platform loses its value, and the two
  are opposite); two fields. **Decision:** two, `drag_time` Qt's and `drag_min_delay` GTK's, each absent or zero
  where the platform has none.
- *The hover time.* The record's "hover-open delay" matched no platform's value: Windows has one hover time, of which
  WinUI's tooltip delays are multiples, while GTK hard-codes each widget's delay (the tooltip's, a submenu's).
  Options: drop the field (Windows' reader would compute WinUI's multiples in its code, out of the gate's sight); make
  it the submenu delay (the menu's own value already); the system's hover time, the tooltip values expressions on
  it, absent where there is none. **Decision:** the last, renamed `hover_time`.
- *Mnemonics.* Pango draws a mnemonic's underline a line thickness below the ink of the character it marks, which no
  font metric gives; GTK draws none on a disabled label or beside a disabled target. Options for the position: an
  element datum for the ink's bottom (one value per query, where every decorated run has its own ink); a fixed
  offset (wrong for a descender); an origin field on every decoration, the baseline or the run's ink (a field every
  lane's decorations would state, and an ink origin an overline could take, which no platform draws); Pango's own
  line kind, a low underline whose offset is measured from the run's ink, the connector taking the ink from its
  glyphs (exact for GTK's mnemonic, the one lane that draws it; Qt's drawn underline is a register row). For the disabled case: a `{disabled}` rule on every part whose text takes a
  mnemonic (one fact restated per widget, and a label beside a disabled target is not disabled itself); a behaviour
  flag the connector applies with the mnemonics setting. **Decision:** the low underline, and the flag.
- *Rendering by icon.* GTK recolours an icon whose file is symbolic, and in any SVG the paths given symbolic paints,
  drawing the rest as they are, and KDE recolours an SVG's colour-scheme stylesheet classes; the field was required, with no value for "the icon decides".
  Options: `Absent` (which means not drawn); `Template` always (a full-colour icon would be recoloured); a content
  variant naming each icon's kind on every widget that draws icons (the connector already holds the icon and its
  kind); a rendering mode for the per-icon rule. **Decision:** the last, `FromIcon`, the icon's own colouring path by path, on `adwaita` and
  `kde-breeze`, each mark beyond the foreground an entry of the icon style's colour map under the platform's own name
  (libadwaita's accent beside its three statuses, KDE's seven stylesheet classes), the map renamed `symbolic_colors`
  as it holds more than statuses; a context that recolours whatever
  the file, libadwaita's check box, states `Template`. The tab button's icon, which libadwaita names itself, is a glyph
  by its icon name, with a content variant for the form it shows from a hundred tabs.
- *A paintable's motion.* libadwaita's status page shows a spinner paintable, which loops for ever. Options: an icon
  animation on an icon context (its keyframes could move only the icon, while the spinner's arc grows and shrinks);
  a reference from the context to the spinner's loop (a token reads no widget value); the spinner element in the
  status page's context, its loop the spinner's. **Decision:** the last, as the catalogue already places it.
- *Styles and drawn text.* Every keyed text style, and so every slot, held a colour, elision, wrapping and a line limit, which most
  platforms' styles do not set (libadwaita's classes and AppKit's styles are fonts; GtkLabel's defaults are a
  widget's), while WinUI's styles set trimming and wrapping. Options: each style states its platform's label defaults
  (another widget's values, and a part that forgot to restate them would pass with the style's); move the four leaves
  onto the part (WinUI's styles would lose their one home, and every text path would move); keep them in the style,
  each absent where the platform's style does not set it, every part that draws text stating all four in its base
  table, by value or by reference to the style's leaf, which validation enforces. **Decision:** the last. The text-scaling role stays the style's, a `Value` so a part references it: on iOS each of Apple's
  styles has its curve; where no curve is read (every scaling but Dynamic Type, `macos-tahoe`'s included, AppKit's styles taking no
  size category) it is stated as none.
- *A misspelled word holding the caret.* Kirigami's spell checker gives that word a faint negative background, by a
  comparison of its own (the word's place in its paragraph against the caret's, or the selection end's, in the whole
  document). Options: a register row until a state exists (the look stays unstated); a state flag of the text widget
  (it is per word, and the widget holds several); a connector-set keyed flag on the misspelling, made an *(element)*
  part per word on every text widget, as a code editor's current line carries its mark flags, the connector computing
  it as Sonnet does, its first-paragraph quirk included. **Decision:** the last, `@holds_caret`.
- *Font width on macOS.* AppKit's width trait runs from −1 to 1, the model's width is OpenType's percent, and no pinned
  source maps one to the other. Options: a conversion of our own (invented); a table from the four named widths (the
  SDK declares them without values); the font's own `wdth` axis, measured with its scale (OpenType's percent, which no source here pins), the register
  deciding how the reading becomes the width. **Decision:**
  the measurement.
- *The status colours of `macos-*` and `ios-*`.* AppKit declares no status colour, and its selection no outline
  colour; UIKit declares no status colour either (the register's U456). Options: map statuses to the system palette
  as the platform's fact (no Apple source maps them; it would invent the platform's meaning); leave them absent there
  whatever the platform draws, every route that reads them checked (the decision until D6: iced's palette,
  gpui-component's theme and egui's visuals hold status colours as plain colours that cannot be left empty, so each
  connector would put there the toolkit's default also where the platform draws the meaning in a colour the theme
  could give, which following the theme's data forbids, or a pairing of its own, unlabelled and different per
  connector); per-preset code in each connector reading the keyed roles (the same pairing, in four places and
  unchecked); one rule for every typed role or slot no source of the platform states: native-theme's own mapping,
  labelled as such, outside the gate's platform comparison and never recorded as the platform's fact.
  **Decision:** the last (ADD-A147; D6, decided by logic, awaiting the maintainer's veto: veto list item 3). It
  invents no colour: what is native-theme's is the pairing, which the label says, and unlike R19's picks it rests on
  a use the platform itself makes of the colour. A typed role is the mapping's where the platform draws its meaning
  in a colour of its own, documented or measured; a text slot by a style or family the platform names (R10); the
  specification's S3 states the rule. Where the platform draws the meaning in no colour (`material-3`'s warning,
  success and info), the role stays absent and the connector's contract records there the colour the toolkit's
  default theme of the resolved scheme holds (ADD-A147; D6, decided by logic, awaiting the maintainer's veto: veto
  list item 56). Where the platform draws the meaning in no colour the theme has no data to follow, so this is an
  exception to "connectors follow the theme's data": for a widget the platform lacks the toolkit gives only its
  geometry (the specification's S21.3), here it gives a colour. On `macos-*` and `ios-*` no pinned Apple source names
  the colours 0.7.0 drew (systemRed, systemOrange, systemGreen, systemBlue) for these meanings, so each status waits
  on its read (the ledger's U757–U760, which hold the candidates and what closes each), its facts pending meanwhile;
  a status a read closes with no witness is absent there, a loss of 0.7.0's status colour (ADD-A147; D6, decided by
  logic, awaiting the maintainer's veto: veto list item 57, whose veto keeps the pairing). Rejected: the fields
  absent while the read is open, which the catalogue could only mark by a condition no check reads and which would
  put a temporary exception in every connector's contract. A status `foreground`, the text on its fill, is the
  colour the platform is measured drawing as text over that fill (U756, U755): 0.7.0's `labelColor` on `macos-*` is
  by its own preset "the body text beside a status mark, not text on the status fill"
  (`v0.7.0:native-theme/src/presets/macos-tahoe.toml:37-39`), and its iOS status text a literal with no source
  (`v0.7.0:native-theme/src/presets/ios-13.toml:45-56`), so neither gives a pairing to keep (ADD-A147; D6, decided
  by logic, awaiting the maintainer's veto: veto list item 3). On `adwaita` the `info` record's `fill` and
  `foreground` refer to the accent's, as libadwaita's info bar draws info (`_deprecated.scss:174`) and as 0.7.0 drew
  it (`v0.7.0:native-theme/src/presets/adwaita.toml:42-45`) (ADD-A147; D6, decided by logic, awaiting the
  maintainer's veto: veto list item 56). `destructive`, a status's `standalone` the platform is not found drawing as
  text or a mark, `background`, `background_solid` and `neutral` stay absent where the catalogue allows it: no
  committed source or measurement shows the platform drawing them at 0.8.0's depth. The Selection set's outline stays
  absent on `macos-*`; macOS's destructive buttons keep their system red, its level indicator its own fills (U757's
  and U758's candidates).
- *Parts not drawn.* The gate compared no field of a part the preset does not draw but failed a field's fact that
  held there, so a field's facts had to restate where their part is drawn (eleven fields of GNOME's menu arrow, and as
  two facts each, since the arrow is drawn under either of two variants). Options: keep the failure (no check gained:
  the part's own key already fails a preset that drops or adds the part, being expected drawn wherever no part fact
  applies); compare and fail nothing there.
  **Decision:** the second; a measured derivation's pass whose part is not drawn still fails, as its evidence
  measured the part drawn.

## 4. What the specification decides

The names and types of every field; the colour roles' surface sets, role record and keyed roles; the text ramp's slots;
the semantic variants, sizes and densities; the condition, context, position, direction and state lists in full, and the
exclusive groups with their precedence, the recency sets; the selector and patch types and the TOML spelling of rules, derivations,
references, inputs and value kinds; the derive macro's changes; the resolution and query API and the element tree a
connector builds; the placement strategies; the derivation functions with their sources; the theme-change token's
fields; the transition kinds and their kind-specific values; the press ripple's effects; which controls AppKit renders whole and the
rendering's API; the facts' data format and the gate's test; the conversion of today's inheritance rules into the
presets; and the mapping of every row (R24). Its first steps, the kirigami-addons survey (R1.2), the survey of the
community projects' platform ports (R19) and reading Plasma's accent derivation (R2.6), are done; what they and
the catalogue required is R25.

## 5. Check record

| Check | Earlier findings | New findings | Main changes |
|---|---|---|---|
| author, passes 1–2 | — | numbering, counts, scope cuts, unchecked claims | first draft corrected |
| independent 1 | — | 9 serious, 16 should fix, 10 minor | rewritten around one rule mechanism (R2) and the facts gate (R4) |
| author, pass 3 | — | resolution out of order; combinations from the map | resolution order; the platforms' state models as the gate's lists |
| independent 2 | 25 resolved, 7 mostly, 3 partly | 6 serious, 18 should fix, 11 minor | facts as data; no model inheritance; palettes from ports; light and dark as a condition; appended paint layers; declarations and validation |
| author, pass 4 | — | groups and contrast inconsistent | KDE groups; why the scheme is a base |
| independent 3 | 29 resolved, 4 mostly, 2 partly | 4 serious, 15 should fix, 14 minor | rules on widget values and tokens; sibling position; transitions as in CSS; one layer order; "colour role" and "palette" named apart; bindings; size and density |
| author, pass 5 | — | bases and rules interleaved | bases before rules restored |
| independent 4 | 28 resolved, 4 partly | 4 serious, 9 should fix, 13 minor | paints for text, icons and glyphs; the system's glyph artwork; density only for declaring widgets; size classes in the selector |
| independent 5 | 22 resolved, 3 partly, 1 not | 3 serious, 9 should fix, 5 minor | expressions evaluated where used; the sidebar size as a condition; named images or the whole-control rendering |
| independent 6 | 11 resolved, 6 partly | 3 serious, 10 should fix, 9 minor | the theme-change transition; size classes the platform's; touch and direction; Material token sets |
| independent 7 | 15 resolved, 7 partly | 2 serious, 11 should fix, 9 minor | consolidated into this text: the theme-change token's three kinds and its trigger; `material-3` as Material's current token set for both flavours; tablet mode against opened-by-touch; the speed factor's scope; text scaling before font-derived lengths; the element defined; tiling per edge; the whole-control rendering's needs; the ripple's values on the transition and its origin from the toolkit; Material contrast by one route; material crossfades and layer interpolation; KDE's accent inputs; glossary, index, and R2 split by concern |
| independent 8 | 12 resolved, 4 mostly, 6 partly | 3 serious, 6 should fix, 16 minor | re-resolution on every reader value, and which changes are theme changes; inputs per scheme and contrast condition; transient touch input; transitions on value changes; group opacity; `material-3`'s platform and reference order; the AppKit rendering's cache and absence; springs at factor zero; Plasma's accent; the consolidation's losses restored |
| specification's work, 2026-10-28 | — | the maintainer's palette decision; 12 mechanisms or facts from the catalogue and the source readings | R19 rewritten (own labelled mapping); R25 added; R1.2 gains libspelling; R2.6's KDE accent and Inactive bullets follow Plasma's `applyScheme()` |

Each check's full report and the worked cases it walked through (a destructive toggle button in a header bar; a WinUI
toggle switch under high contrast in a compact container; a KDE list row with a palette, inactive, under the Kirigami
flavour; an iOS 26 glass button in a sheet; a libadwaita entry row in a bottom sheet; a macOS sidebar row with a
disclosure triangle and a dark-only palette; a Material Expressive button with a ripple; a WinUI combo box with Mica and an
accent change; a KDE tab in a window being tiled; a libadwaita tab under an accent switch) were working files of the
checks and are not kept; this table records what each check changed, and the steps a case left open became the
findings of the next check.
