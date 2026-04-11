# v0.5.7 -- native-theme: API Overhaul

Deep critical analysis of the public API of the `native-theme` crate. Every
issue has file:line references, multiple solution options with pros/cons, a
recommended fix with justification, and an honest "confidence" marker when
the recommendation involves judgement calls.

Since v0.5.7 is explicitly a no-backward-compat window, the recommendations
below do not preserve existing type names, method names, or module layout
unless doing so is the best outcome on the merits.

---

## Guiding principles

Before the issue catalog, the assumptions this document operates under:

1. **Users are app authors who want theme data for their toolkit**, not
   compiler engineers or OS specialists. The API should be as shallow and
   predictable as the domain allows.
2. **Connectors are the primary consumers.** What connectors reach for is
   what should be canonical; what they avoid is what should be hidden.
3. **OS detection is unreliable by nature** (subprocesses, D-Bus races,
   stale env vars). The API should make that unreliability visible, not
   hide it behind apparent infallibility.
4. **Memory cost per `SystemTheme` instance is measured**, not asserted.
   A `ResolvedThemeVariant` carries ~25 widget structs plus defaults --
   roughly 2 KB of concrete data per variant plus heap for font family
   strings. Two variants plus two pre-resolve copies is ~6-8 KB per
   `SystemTheme`. That is not enormous, but it is wasted on the common
   path where `with_overlay` is never called.
5. **No runtime panics, no unsafe** (from CLAUDE memory). Every
   recommendation preserves this constraint; several points call it out
   explicitly when an "easy" fix would break it.
6. **Never invent values** (from CLAUDE memory). Every claimed value,
   file path, line number, and behavior in this document is cross-checked
   against the current tree. Any place where I could not verify is flagged
   as `[low confidence]`.

---

## Table of contents

1. [Type vocabulary: six nouns for one thing](#1-type-vocabulary-six-nouns-for-one-thing)
2. [Doubled `Option<T>` / `Resolved<T>` struct hierarchy](#2-doubled-optiont--resolvedt-struct-hierarchy)
3. [`SystemTheme` stores pre-resolve variants to support one feature](#3-systemtheme-stores-pre-resolve-variants-to-support-one-feature)
4. [`SystemTheme::active()` vs `pick()` redundancy and staleness](#4-systemthemeactive-vs-pick-redundancy-and-staleness)
5. [`from_system_async()` is synchronous on non-Linux](#5-from_system_async-is-synchronous-on-non-linux)
6. [`Error` enum: structure, `Clone` bound, message fidelity](#6-error-enum-structure-clone-bound-message-fidelity)
7. [`ThemeVariant::resolve*` method proliferation](#7-themevariantresolve-method-proliferation)
8. [Icon loading: 12 functions, one user intent](#8-icon-loading-12-functions-one-user-intent)
9. [`load_icon` hardcodes freedesktop size to 24](#9-load_icon-hardcodes-freedesktop-size-to-24)
10. [`IconProvider::icon_svg` locks to `&'static [u8]`](#10-iconprovidericon_svg-locks-to-static-u8)
11. [`IconData::Svg(Vec<u8>)` forces a copy on bundled loads](#11-icondatasvgvecu8-forces-a-copy-on-bundled-loads)
12. [Flat crate root exports 80+ items](#12-flat-crate-root-exports-80-items)
13. [Global static caches in `detect` and `model/icons`](#13-global-static-caches-in-detect-and-modelicons)
14. [`ThemeSpec::lint_toml` hand-maintained duplicate registry](#14-themespeclint_toml-hand-maintained-duplicate-registry)
15. [`ThemeSpec` method grab-bag](#15-themespec-method-grab-bag)
16. [`Rgba` defaults, naming, and conversions](#16-rgba-defaults-naming-and-conversions)
17. [`IconSet::default()` is Freedesktop on all platforms](#17-iconsetdefault-is-freedesktop-on-all-platforms)
18. [`IconSet::from_name` / `name` duplicates serde](#18-iconsetfrom_name--name-duplicates-serde)
19. [`LinuxDesktop` is not `#[non_exhaustive]`](#19-linuxdesktop-is-not-non_exhaustive)
20. [`icon_set` and `icon_theme` live on the wrong type](#20-icon_set-and-icon_theme-live-on-the-wrong-type)
21. [`ThemeWatcher` struct internals and constructor split](#21-themewatcher-struct-internals-and-constructor-split)
22. [`on_theme_change` runtime-errors instead of compile-errors on missing features](#22-on_theme_change-runtime-errors-instead-of-compile-errors-on-missing-features)
23. [`diagnose_platform_support` returns `Vec<String>`](#23-diagnose_platform_support-returns-vecstring)
24. [`platform_preset_name` leaks the internal `-live` convention](#24-platform_preset_name-leaks-the-internal--live-convention)
25. [`FontSize::Px(v).to_px(dpi)` silently ignores the DPI parameter](#25-fontsizepxvto_pxdpi-silently-ignores-the-dpi-parameter)
26. [`#[must_use]` messages on value types are preachy](#26-must_use-messages-on-value-types-are-preachy)
27. [Priority summary](#27-priority-summary)
28. [Open questions / items requiring discussion](#28-open-questions--items-requiring-discussion)

---

## 1. Type vocabulary: six nouns for one thing

**Files:**
- `native-theme/src/model/mod.rs:226` (`ThemeSpec`)
- `native-theme/src/model/mod.rs:59` (`ThemeVariant`)
- `native-theme/src/model/resolved.rs:169` (`ResolvedThemeVariant`)
- `native-theme/src/model/defaults.rs:34` (`ThemeDefaults`)
- `native-theme/src/model/resolved.rs:66` (`ResolvedThemeDefaults`)
- `native-theme/src/lib.rs:215` (`SystemTheme`)

### Problem

A user who wants "theme colors for my app" must learn and sequence six
distinct nouns. The canonical bundle path is:

```rust
let spec:     ThemeSpec           = ThemeSpec::preset("dracula")?;
let variant:  ThemeVariant        = spec.into_variant(true).ok_or(...)?;
let resolved: ResolvedThemeVariant = variant.into_resolved()?;
```

The canonical OS path is:

```rust
let system: SystemTheme                = SystemTheme::from_system()?;
let ref_v:  &ResolvedThemeVariant      = system.active();
```

Inside each of those, the user also meets `ThemeDefaults` and
`ResolvedThemeDefaults` when reading `active.defaults.accent_color`.

The names are internally consistent (everything is a "theme" of some sort)
but they all live at the crate root (`lib.rs:124-133`), so the rustdoc and
the IDE completion surface six names that mean "theme" in slightly
different ways. A new user cannot predict the right name without reading
docs for each.

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Keep names as-is; improve docs only** | Zero code churn. Existing users unaffected. | Does nothing for the complaint. New users keep hitting the same confusion. |
| B | **Rename only**: `ThemeSpec → Theme`, `ThemeVariant → ThemeLayer`, `ResolvedThemeVariant → ResolvedTheme`, `SystemTheme → ThemeBundle` | Smallest code change. Each rename is a sed-able mechanical pass. `ThemeDefaults` stays because it is accessed as a nested field, not a top-level type. | Still four top-level nouns. `ThemeLayer` is a new name users must learn. |
| C | **Rename + collapse `ThemeVariant`**: users never see it. Methods like `Theme::merge` and `Theme::resolve` operate at the `Theme` level; internally a light/dark pair still exists, but users only ever see `Theme` and `ResolvedTheme` | Three top-level nouns: `Theme`, `ResolvedTheme`, `SystemTheme`. Matches how users actually think ("a theme", "a resolved theme", "a detected theme"). | Requires rethinking `Theme::merge` semantics (what happens when you merge a pair onto a pair?). Loses the ability to work with a single variant in isolation for advanced users. |
| D | **Typestate**: one `Theme<State>` generic with `Raw`/`Resolved` marker types, one `SystemTheme` | Smallest possible public surface. Invalid states unrepresentable. | Ergonomics suffer: field access differs between states (Raw has `Option<T>`, Resolved has `T`) so the same `theme.defaults.accent_color` does different things. Generics everywhere. Most Rust users find typestate heavy. |
| E | **Rename + type alias for compatibility window** (`pub type ThemeSpec = Theme;`) | Migration path for external users | Defeats the "no backward compat" freedom. Aliases stay around forever in practice. |
| F | **Module disambiguation, no top-level rename** (added in merge review): leave every struct's identifier alone but move them inside `native_theme::theme::` so users write `theme::Spec`, `theme::Variant`, `theme::Resolved`, `theme::Bundle`. The short names are context-disambiguated by the `theme::` prefix; no rename migration needed. Couples tightly with §12 module partition. | Zero rename churn -- `grep -n ThemeSpec` lines keep matching. Short names are readable once the module context is known. The `Spec` / `Variant` / `Resolved` triplet is locally obvious inside `theme::`. | Still six nouns, just hidden behind a prefix. Connectors doing `use native_theme::{ThemeSpec, ThemeVariant, ...}` have to migrate imports. Paths become longer at the use site unless the user re-imports. |

### Recommended: **B**

Rename mechanically, keep the four-noun structure:

- `ThemeSpec`       → `Theme`
- `ThemeVariant`    → `ThemeLayer` (or `ThemeData`)
- `ResolvedThemeVariant` → `ResolvedTheme`
- `ResolvedThemeDefaults` → `ResolvedDefaults`
- `SystemTheme`     → `DetectedTheme` (better still: `ThemeBundle`, but see rationale)
- `ThemeDefaults`   → `Defaults` (inside the theme module namespace, not at crate root)

### Rationale

Option **C** is attractive on paper -- three top-level nouns feels cleaner
-- but the cost is hidden: `ThemeVariant` exists because **inheritance
must operate on a single mode's data**. Resolution rules like "button
background inherits from accent color" walk a single variant's fields.
If we collapse `ThemeVariant` into `Theme`, every inheritance method
has to re-establish "am I resolving light or dark?" and pass that around.
Two resolution calls per theme. Double the code, same output. Not worth it.

Option **D** is worth considering for a crate where types-as-proofs is
culturally valued, but `native-theme` is a data-modelling crate, not a
type-theory playground. Generic types in public signatures force users to
write `Theme<Raw>` / `Theme<Resolved>` constantly. Field access diverges
between the two states. Rustdoc becomes harder to read. Not worth it.

Option **E** sounds friendly but creates permanent legacy. If v0.5.7 is
the compat-free window, use it.

Option **B** does the minimum work for the maximum readability benefit:
the names track user intuition ("it is a theme" beats "it is a theme
spec"), the structure is preserved, and the rename is one merge commit
without risk. The one judgement call is the fifth rename: should
`SystemTheme` become `DetectedTheme` (emphasises "came from the OS") or
`ThemeBundle` (emphasises "holds both variants")? I marginally prefer
`DetectedTheme` because the user's question at the call site is usually
"did I detect this or load it from disk" -- the "bundle" shape is an
implementation detail.

Option **F** is a real alternative that trades a different set of costs
for a different set of benefits. It avoids the rename churn entirely
(every existing `ThemeSpec` token keeps resolving), and the short names
become unambiguous *inside* `theme::`. The cost is that users at the
top level see `native_theme::theme::Spec` or have to add a
`use native_theme::theme::Spec` line. Whether F is better than B
depends on whether the crate expects users to reach for the inner
types frequently (in which case B wins because identifiers stay short
at the use site) or rarely (in which case F wins because there is no
rename migration at all). For a data-modelling library where
`ResolvedThemeVariant` is the primary consumer type, B is still my
recommendation: the short top-level name is the more common path. F
is a reasonable fallback if the rename migration is judged too risky.

**Confidence:** high. Pure renaming with no semantic change. The one open
question is the fifth rename, flagged in §28.

---

## 2. Doubled `Option<T>` / `Resolved<T>` struct hierarchy

**Files:**
- `native-theme/src/model/widgets/mod.rs:48-156` (`define_widget_pair!` macro)
- `native-theme/src/model/defaults.rs:34` + `native-theme/src/model/resolved.rs:66` (`ThemeDefaults` + `ResolvedThemeDefaults`)
- `native-theme/src/model/font.rs:78` + `:159` (`FontSpec` + `ResolvedFontSpec`)
- `native-theme/src/model/border.rs:13` + `:60` (`BorderSpec` + `ResolvedBorderSpec`)
- `native-theme/src/model/icon_sizes.rs:12` + `native-theme/src/model/resolved.rs:15` (`IconSizes` + `ResolvedIconSizes`)
- `native-theme/src/model/font.rs:179` + `native-theme/src/model/resolved.rs:32` (`TextScaleEntry` + `ResolvedTextScaleEntry`)
- `native-theme/src/model/font.rs:285` + `native-theme/src/model/resolved.rs:48` (`TextScale` + `ResolvedTextScale`)

### Problem

For each leaf data shape the crate has two public structs: an
`Option<T>`-field version used during parsing/merging, and a
non-`Option<T>` version used after resolution. The `define_widget_pair!`
macro at `widgets/mod.rs:48-156` emits both at once. There are 25 widget
pairs plus 6 non-widget pairs = **~62 public struct types** for what is
conceptually one theme.

The consequences:

1. **Doc burden.** Every field is documented on both the `Option` side and
   the `Resolved` side. The doc at `widgets/mod.rs:85` copies doc attrs
   into the Resolved struct via `$(#[doc = $opt_doc])*`, so in practice
   both variants get the same doc text -- 60+ duplicated doc blocks.
2. **Rename risk.** Changing `ButtonTheme::background_color` means
   updating two structs, one serde rename, one `FIELD_NAMES` constant,
   one resolver match arm, and tests.
3. **Lost "was this explicit?" information.** After resolution, a caller
   cannot ask "was this value set by the preset or derived from accent?"
   -- the resolved struct only has the concrete value. Connectors
   occasionally want this (e.g., "inherit background from default unless
   user set it explicitly"). The pipeline design forbids that query.
4. **Macro load.** `define_widget_pair!` is 108 lines with five field
   categories (`option`, `soft_option`, `optional_nested`, `border_partial`,
   `border_optional`). Contributors must learn this DSL to add a widget.

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo.** Keep both hierarchies. | No migration pain. Clear separation between "input shape" and "validated shape". Typed non-null guarantees for connectors. | All the problems above. Doubled doc burden. Macro complexity. |
| B | **Single `Theme` struct with `Option<T>` everywhere; resolved state is just "all fields have been filled, verified by runtime invariant"** | One struct per concept. Half the types. | Connectors lose the compile-time "this field is always present" guarantee. `.unwrap_or_default()` or `.unwrap()` on every field read is ugly and invites inconsistency. Panics become possible. |
| C | **Single `Theme` struct; resolved state exposed via a `&View` wrapper with typed getter methods** | Users interact through methods like `view.button().background_color()` which return `Rgba` directly. No panics. Single struct under the hood. | Method-based access is less rustacean than struct access. Every connector must be rewritten. View lifetime management is fiddly. |
| D | **Keep the split, but generate both from a single `#[derive]` macro with one field definition per field** | Preserves compile-time non-null guarantees. Single source of truth for field definitions. | Needs a real proc-macro crate (current design is a declarative macro). Migration cost. Attribute DSL must be designed. |
| E | **Two structs, but make Resolved a thin view over Option**: `ResolvedTheme` owns an `UnresolvedTheme` internally and offers typed accessors that `.unwrap()` fields whose non-null invariant is established by `resolve()`. Resolve returns `ResolvedTheme` only when validation passes. | Single backing store. Resolved view provides the `Rgba`-returning accessors. Non-null invariant is local to the `Resolved` wrapper and proven once in `resolve`. | Every access through `resolved.button().background_color()` walks an Option under the hood. Method indirection. The `&ResolvedTheme` value is still 2 KB (same size). |
| F | **Keep the split; drop the macro; write both structs out by hand** | Simpler to read. No DSL. | More lines of code (roughly doubled). Harder to maintain rename consistency. |

### Recommended: **D** (medium confidence)

Move to a proc-macro based code generator that reads a single field
definition list (one per field, not one per struct pair) and emits both
the `Option` and `Resolved` structs plus their merge/resolve glue. This
keeps the compile-time non-null guarantee (the Achilles heel of B and C)
while eliminating the doc duplication and the manual macro maintenance
burden.

### Rationale

Option **A** is the default but the complaints above are real and
compounding. The v0.5.4 archived doc already had mapping fixes for
individual widget presets -- each future change multiplies the work.

Option **B** is rejected because it violates the "no runtime panics"
constraint from CLAUDE memory. The moment every field is `Option<T>`,
someone writes `.unwrap()` and a preset ships with a missing field.
The current design makes "missing field" impossible at the
`ResolvedThemeVariant` type level, and that guarantee is load-bearing
for connectors.

Option **C** has the same compile-time benefit as the current design but
trades struct field access for method calls. That is a large ergonomics
hit across two connector crates, plus the rest of the ecosystem. I cannot
justify this cost.

Option **E** is a half-measure. It saves memory but loses nothing else.
In practice `ResolvedTheme::button()` returning a derived view has the
same shape as `&self.button`, and the optimiser may even inline it to the
same thing. But now every method must be maintained, and the view type
must be designed. That's a lot of code for a marginal win.

Option **F** loses the consistency benefit of the macro without gaining
much. Also a half-measure.

Option **D** is the clean answer: replace the `define_widget_pair!`
declarative macro with a proc-macro (e.g. a new crate
`native-theme-derive`) that takes a single field list and emits both
`XxxTheme` and `ResolvedXxxTheme`. Field renames happen in one place.
Docs are in one place. The macro itself becomes a real piece of
infrastructure with its own tests, rather than a growing pile of
`$($($(#[doc = ...])*)*)*` inside `widgets/mod.rs`. It's more work up
front but pays back on every future widget addition.

**Confidence:** medium. The proc-macro path is the right direction but
the design work for the attribute DSL is non-trivial, and the migration
cost is real. If v0.5.7 does not have the runway for this, option **A**
(status quo) is an acceptable deferral -- none of the other options are
strictly better than A without the macro-generator infrastructure.

**Flag for §28:** the proc-macro approach interacts with the
`property-registry.toml` file (referenced in CLAUDE memory as the source
of truth for fields). The derive should probably read the registry
rather than re-specifying fields in Rust attributes. That requires
design discussion before committing.

---

## 3. `SystemTheme` stores pre-resolve variants to support one feature

**Files:**
- `native-theme/src/lib.rs:215-232` (struct definition)
- `native-theme/src/lib.rs:279-306` (`with_overlay` implementation)

### Problem

```rust
pub struct SystemTheme {
    pub name: String,
    pub is_dark: bool,
    pub light: ResolvedThemeVariant,         // ~2 KB
    pub dark:  ResolvedThemeVariant,         // ~2 KB
    pub(crate) light_variant: ThemeVariant,  // pre-resolve copy, ~1 KB
    pub(crate) dark_variant:  ThemeVariant,  // pre-resolve copy, ~1 KB
    pub preset: String,
    pub(crate) live_preset: String,
}
```

The doc comment at `lib.rs:224` says explicitly:
*"Pre-resolve light variant (retained for overlay support)"*.

So **every `SystemTheme` pays ~2 KB of memory per variant for a feature
(`with_overlay`) that most apps never call**. Apps that call `from_system`
once at startup and never call `with_overlay` are paying for plumbing they
do not use.

The reason `with_overlay` needs pre-resolve state is real: the docstring
at `lib.rs:257-263` explains that merging an overlay onto a resolved
variant would fail to propagate accent-color changes to derived fields
(like `button.primary_background`), because those fields are already
concrete. Overlaying on pre-resolve state and re-running resolution
produces the correct cascade.

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo.** Keep both. | No code change. `with_overlay` works as documented. | Memory cost on every `SystemTheme`. |
| B | **Drop pre-resolve fields; rebuild from `(reader_output, preset_name)` on overlay** | Smaller `SystemTheme`. Pre-resolve storage becomes raw reader output + preset name, which is smaller (~200-500 bytes for a sparse reader spec) than a fully-merged pre-resolve variant (~1 KB). | `with_overlay` must re-run `run_pipeline`. Extra CPU on the rare path. Must also preserve `font_dpi` separately (currently inferred in `into_resolved`, not the pipeline). |
| C | **Drop pre-resolve fields; `with_overlay` re-reads the OS** | Zero extra memory. `with_overlay` gets fresh OS state. | `with_overlay` becomes an expensive operation with potential for subprocess spawns, D-Bus round trips, and timing-dependent different results. Surprising for users. |
| D | **Split into `SystemTheme` (small, resolved-only) and `SystemThemeBuilder` (large, supports overlay)**: users opt into the large type when they know they need overlays | Common path pays zero. Overlay path is explicit. | Two public types where there was one. "Builder" is a misnomer (it's not building; it's supporting re-resolution). Names get awkward. |
| E | **Make pre-resolve fields lazy / on-demand** via `Option<Box<...>>` that `with_overlay` populates on first call by re-running the pipeline | Best of both: cheap on common path, supports overlay on demand, no new public types. | Still needs the reader output or preset re-run to populate. Adds an interior-mutable hot spot inside `SystemTheme`, which breaks the "everything is a cheap `Clone`" assumption for readers. |
| F | **Make `with_overlay` take the pre-resolve variants as a separate argument**: `SystemTheme` stays slim; callers who want overlays save the variants themselves | Simplest refactor. No hidden state. | Overlay ergonomics degrade: the user must plumb a tuple `(SystemTheme, ThemeVariant, ThemeVariant)` through their code. |

### Recommended: **B** (medium confidence)

Replace `light_variant` / `dark_variant` with one of:

```rust
pub(crate) overlay_source: Option<OverlaySource>,

struct OverlaySource {
    reader_output: ThemeSpec,   // just the OS-detected data
    preset_name:   String,       // the base preset to merge against
    font_dpi:      f32,          // captured at detection time
}
```

`with_overlay` then rebuilds: `run_pipeline(reader_output, preset_name, ...)`
→ merge overlay → `resolve()`. Same behaviour, smaller storage, and
**callers without overlay use pay `sizeof(Option)` = ~8 bytes** instead of
several KB.

### Rationale

Option **A** is defensible -- the absolute memory cost is bounded, and most
apps create at most a few `SystemTheme` instances per lifetime. But "we
pay for a feature nobody uses" is a smell, and v0.5.7 is the right window
to eliminate it.

Option **C** is wrong: users expect `with_overlay` to be a pure function
of the `SystemTheme` they already have. If calling it re-reads the OS,
users get non-deterministic behaviour. Rejected.

Option **D** is tempting for the API-clarity argument ("overlay support
is opt-in") but the naming is awkward. `SystemThemeBuilder` is not a
builder. `OverlayableSystemTheme` is ugly. I cannot find a clean name.

Option **E** is elegant but introduces interior mutability
(`OnceCell<PreResolve>` or similar) inside `SystemTheme`, which means
`SystemTheme` can no longer be `#[derive(Clone)]` trivially (or clones
become expensive / share state). It also only defers the storage cost
rather than eliminating it: the first `with_overlay` call fills the
field, and thereafter the memory is held.

Option **F** pushes the cost onto every user of overlays, which is the
wrong direction.

Option **B** is the best tradeoff. It captures the minimum information
needed to replay the pipeline (reader output + preset name + DPI) and
stores it compactly. The extra CPU on `with_overlay` is negligible -- the
pipeline is a data transform, not an I/O operation, once the reader has
already run. Callers who never touch `with_overlay` pay ~8 bytes instead
of ~2 KB.

**Confidence:** medium. This is the right direction, but the exact fields
of `OverlaySource` need to be worked out against the actual pipeline
code. In particular, `font_dpi` is currently set lazily in
`into_resolved` (`resolve/mod.rs:102-104`), not during the pipeline; that
lazy detection path must be preserved or eagerised. The refactor
interacts with §7 (`resolve*` proliferation) and should be done
together.

### Merge-review refinement: pair with doc 2 §B5 to avoid double-capture of DPI

The doc as drafted stores `font_dpi` inside `OverlaySource`. Doc 2 B5
separately recommends extracting `font_dpi` out of `ThemeDefaults` into
a `ResolutionContext`. These two recommendations are in tension: if B5
lands, then `OverlaySource` no longer needs to hold `font_dpi` directly
-- it can hold a `ResolutionContext` (or just a `&ResolutionContext`
from outside) and the DPI is captured once, at detection time, through
that context instead of as a separate field here.

Recommend: design §3 and B5 together. The `OverlaySource` shape should
become:

```rust
pub(crate) struct OverlaySource {
    reader_output: Theme,          // OS-detected data only
    preset_name:   String,         // base preset to merge against
    ctx:           ResolutionContext,  // from doc 2 B5 -- captures DPI + button_order + icon_theme
}
```

This collapses the two "where does font_dpi live" questions into one
answer: **in `ResolutionContext`, captured eagerly at `from_system()`
time, consumed by both the initial resolve and any later `with_overlay`
replay**. The "lazy detection in `into_resolved`" quirk disappears.

The "~8 bytes vs ~2 KB" memory argument is slightly weakened -- a
`ResolutionContext` holding strings and enums is ~200-400 bytes rather
than ~8 -- but still a solid win over ~2 KB per `SystemTheme` instance.
The architectural win (single source of truth for OS-captured data) is
larger than the memory micro-win.

**Confidence:** medium-high for the combined design. If §3 and B5 are
split across releases, §3's `OverlaySource` should take a standalone
`font_dpi: f32` field as drafted and be refactored when B5 lands.

---

## 4. `SystemTheme::active()` vs `pick()` redundancy and staleness

**File:** `native-theme/src/lib.rs:239-253`

### Problem

```rust
pub fn active(&self) -> &ResolvedThemeVariant {
    if self.is_dark { &self.dark } else { &self.light }
}

pub fn pick(&self, is_dark: bool) -> &ResolvedThemeVariant {
    if is_dark { &self.dark } else { &self.light }
}
```

Two methods that differ only in whose `is_dark` they trust. `active()`
uses the captured `self.is_dark` (set at construction time by the
reader). `pick(is_dark)` uses an explicitly passed value.

The problem with `active()` is the *freshness illusion*. A new user
reads the method name and assumes "active" means "currently active on
the OS". But `self.is_dark` is a **snapshot from the moment
`from_system()` was called**. If the OS switches mode two hours later
and the app hasn't re-run detection, `active()` returns stale data with
no warning.

This is a classic recipe for "works in testing, fails on the users'
machines at the worst moment" bugs, because the app author tested by
starting the app in both modes and never saw the stale case.

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Keep both as-is.** | No change. | The trap remains. |
| B | **Keep both, rename `active()` to `as_detected()` or `captured()`** to make the snapshot explicit | Forces the user to confront staleness semantics when reading the name. One-line change. | `captured` / `as_detected` are awkward. Users may still assume "captured" means "captured each time". |
| C | **Drop `active()` entirely; keep only `pick(is_dark)`** | Zero ambiguity: the user must supply `is_dark`, which forces them to decide whether to use cached or live detection. | Every call site now has one extra token. Slightly less ergonomic for the common "just give me whichever the OS was in" case. |
| D | **Drop `pick()`; keep `active()` but have it re-query `system_is_dark()` on every call** | Name matches behaviour. | Hidden OS calls on a method that looks like a cheap accessor. On Linux, `system_is_dark` is cached but may still spawn a subprocess if the cache was invalidated. Surprising. |
| E | **Introduce a `ColorMode { Light, Dark, System }` enum** and one method `variant(ColorMode) -> &ResolvedThemeVariant` where `ColorMode::System` triggers live detection | Maximally explicit. Three-way signal (Light / Dark / live) is what app UIs actually need. | More verbose: `sys.variant(ColorMode::System)` is wordier than `sys.active()`. |
| F | **Drop `active()`; keep `pick(is_dark)`; add `SystemTheme::is_dark_now() -> bool` as a convenience that calls `detect_is_dark` with no caching** | Two methods, each with a single clear purpose. User chains: `sys.pick(sys.is_dark_now())` or `sys.pick(sys.is_dark)` (the struct field) depending on staleness intent. | Two API decisions on every access. |

### Recommended: **C**, with field access preserved

Drop `active()`. Keep `pick(is_dark) -> &ResolvedTheme`. Document the
idiom `sys.pick(sys.is_dark)` for "whichever was detected" and
`sys.pick(native_theme::detect::is_dark())` for "live". The `is_dark`
field remains `pub`, so `sys.pick(sys.is_dark)` is trivially short.

### Rationale

Option **A** leaves a landmine. Option **B** moves the landmine to a
name that users still misread. Option **D** hides OS work behind a
simple accessor, which I find strictly worse than making the work
explicit. Option **E** is the most principled but has the highest
verbosity cost; in an API that already has too many names, adding
another noun is a tax.

Option **C** is the minimum change that eliminates the ambiguity: the
user *must* pass `is_dark`, which forces them to think about where it
comes from. The convenience of "I already detected it, use that" is
preserved by making `sys.is_dark` a public field. The convenience of
"always use live detection" is handled by calling the detect function
directly. Both idioms are one expression. No method does both jobs
poorly.

Option **F** is an acceptable alternative, but adding a new method
`is_dark_now()` duplicates `detect_is_dark()` for the marginal benefit
of saving the user from importing it. The call site becomes
`sys.pick(sys.is_dark_now())` which is not shorter than
`sys.pick(native_theme::detect::is_dark())` in terms of characters and
has the disadvantage of being `SystemTheme`-scoped when the question is
really "what does the OS say right now", not "what does this bundle
think".

**Confidence:** high. This is a straight removal; the rename /
replacement idioms are well-understood.

---

## 5. `from_system_async()` is synchronous on non-Linux

**File:** `native-theme/src/lib.rs:372-386`

### Problem

```rust
#[cfg(target_os = "linux")]
pub async fn from_system_async() -> crate::Result<Self> {
    pipeline::from_system_async_inner().await
}

#[cfg(not(target_os = "linux"))]
pub async fn from_system_async() -> crate::Result<Self> {
    pipeline::from_system_inner()   // sync body inside async fn
}
```

On macOS and Windows the async version is a synchronous function with
the word `async` sprinkled on top. This is not "async-compatible" (it
blocks), it is not "non-blocking" (the macOS NSUserDefaults call blocks
the current thread), and it is not "uniform" (the Linux branch awaits a
portal call while the macOS branch does not).

On Linux the runtime is selected by Cargo feature: `portal-tokio`
enables `ashpd/tokio`, `portal-async-io` enables `ashpd/async-io`
(`native-theme/Cargo.toml:17-18`). A user whose dependency tree enables
`portal-tokio` cannot use `async-std` at the same time, and vice versa.
Feature unification across dependencies may silently pick the wrong
runtime.

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo.** | No work. | The two problems remain. |
| B | **Eliminate `from_system_async`.** `from_system()` always-sync. On Linux, the portal call is run through `futures::executor::block_on` internally; no runtime dependency. | Consistent API across platforms. No runtime coupling. Feature `portal-tokio` and `portal-async-io` merge into one `portal` feature. | The portal call blocks the calling thread on Linux. An async app must wrap in `spawn_blocking`. Short but real penalty. |
| C | **Keep `from_system_async`, make it runtime-agnostic.** Use a runtime-agnostic executor (e.g. `pollster`, or `futures::executor::block_on` internally; or expose the underlying future for the caller to drive) | Async users get a non-blocking API on Linux; sync users wrap in block_on. | Implementation complexity: `ashpd` is the blocker, and its API is not runtime-agnostic. We'd need to either swap to a direct D-Bus crate or wrap `ashpd` behind a thin adapter per runtime. |
| D | **Split the async API by runtime**: `from_system_async_tokio()` (gated on `portal-tokio`) and `from_system_async_io()` (gated on `portal-async-io`). Only one exists per build. | Explicit. No silent runtime mismatches. | Two functions, two feature flags, two docs. Users doing conditional compilation hit `#[cfg(feature = ...)]` noise. |
| E | **Take an executor at call time**: `from_system_async(&dyn Executor)` or have the user pass in a D-Bus connection. The library becomes runtime-agnostic because the runtime is an argument. | Fully decoupled. | API ergonomics degrade; every call site must have an executor. Doesn't exist as a standard trait in the ecosystem. |
| F | **Expose the inner Future** (`from_system_future()`) and let users drive it however they want | Maximum flexibility. | Users must know what `Future` means and how to drive it. Beginners get lost. |
| G | **Keep both `from_system` and `from_system_async`, make sync wrap `block_on(async)`** (added in merge review). Exactly one implementation (`from_system_async_inner`) exists. The sync `from_system()` is `pollster::block_on(from_system_async_inner())`. Async users get a genuine non-blocking future; sync users get the exact same behaviour as today. Feature `portal-tokio` / `portal-async-io` collapse into a single `portal` feature that pins `ashpd` to its `async-io` backend (which `pollster` can drive without a runtime). | Strict superset of B: sync users see no change, async users keep their async API, one implementation path for the portal call. No tokio coupling. An async app that wraps sync in `spawn_blocking` today still works (it just spawns a thread that calls `block_on` internally -- a tiny waste but zero behaviour change). | Adds a dependency on `pollster` (~50 LoC, zero transitive deps). `from_system_async` on macOS/Windows is still "async fn with sync body" unless those readers are also wrapped in a trivial `async {}` block -- which is trivial to do. |

### Recommended: **G** (strict superset of **B**; preserves async entry point)

Make `from_system` always-sync. Drop `from_system_async`. On Linux, use
an internal `block_on` (via `pollster` or `futures::executor::block_on`)
to drive the portal call. Async users wrap in `tokio::task::spawn_blocking`
or equivalent. Merge `portal-tokio` and `portal-async-io` into a single
`portal` feature that does not depend on any specific async runtime.

### Rationale

Option **A** leaves two real problems. Option **C** sounds right but
runs into the hard fact that `ashpd` (the crate that does the D-Bus work)
is tied to an async runtime by its dependencies. Making the crate
runtime-agnostic on Linux requires either replacing `ashpd` with
direct D-Bus (e.g. `zbus::blocking`) or maintaining two parallel
implementations. Both are more work than they are worth.

Option **D** preserves the runtime choice but is a tax on every user:
the function they call depends on which feature flag is set, and cross-
platform code becomes cfg-heavy.

Option **E** is architecturally ideal but has no ecosystem precedent.
There is no trait `Executor` that `tokio`, `async-std`, and `smol` agree
on. This would need to be ad hoc and adds a parameter to a function that
90% of users want to call with no arguments.

Option **F** is fine for advanced users but makes the 101 case harder.

Option **B** wins on simplicity. The cost is that `from_system` may
block the current thread for up to a few hundred milliseconds on Linux
when the portal is slow. Async apps already know the pattern for
blocking calls (`spawn_blocking`). Sync apps don't change anything.

One subtle point: `zbus` (already a dependency for the watcher at
`Cargo.toml:89-91`) offers a `blocking` API that does not require an
async runtime. The portal call can be migrated from `ashpd` to raw
`zbus::blocking` to eliminate the async dependency entirely. This is
more work but removes the "which runtime" question permanently.

**Confidence:** high on the direction; medium on the migration cost.
Flag for §28: verify that `zbus::blocking` can replace the `ashpd`
portal surface without losing functionality. If it cannot, fall back to
`pollster::block_on(ashpd_call())` -- ashpd without a runtime feature
uses `async-io` by default, which we can drive from `pollster` without
pulling tokio.

### Merge-review: why G beats B

Option G is a strict superset of B. The difference is whether we keep
`from_system_async` as a public entry point. B deletes it; G keeps it
and implements sync on top of the same future. Both options:

- Merge `portal-tokio` and `portal-async-io` into one `portal` feature.
- Drop the runtime coupling.
- Make sync users pay the blocking cost (because there is no way to
  make a sync call non-blocking).

The advantage of keeping `from_system_async` is that async callers
don't have to wrap in `spawn_blocking`. `spawn_blocking` isn't free:
it moves the call to a blocking-thread pool, adds thread-spawn
latency (~50-200 μs), and on small runtimes (smol, async-io) may
starve the blocking pool if called frequently. A native async path
avoids all of that. For GUI apps on Linux, the portal call is
already the slow path (hundreds of ms worst case); having a true
async version lets the UI thread stay responsive during startup
without a dedicated blocking-pool thread.

The cost of G over B is **one dependency** (`pollster`, which is
50 LoC and has no transitive deps) and **one more public function**
(`from_system_async`). Both costs are small. The benefit -- keeping
the async entry point genuinely non-blocking on Linux -- is
meaningful for the target audience.

Recommend **G**. Fall back to B if the dependency cost is considered
unacceptable or if keeping two entry points is judged to pollute the
surface. Fall back further to ashpd + zbus::blocking only if even
pollster is too much.

---

## 6. `Error` enum: structure, `Clone` bound, message fidelity

**File:** `native-theme/src/error.rs:73-142`

### Problem

Three separate complaints:

#### 6a. `Clone` bound is vestigial

`error.rs:80`: `#[derive(Debug, Clone)]`. The comment at `error.rs:77-80`
justifies the `Clone` bound with *"This type is Clone so it can be
stored in caches alongside crate::ThemeSpec. The Platform and Io
variants use Arc internally to enable cloning without losing the
original error chain."*

But **the preset cache does not store `Error`**. At
`native-theme/src/presets.rs:85-92`:

```rust
// Errors are stored as String because Error is not Clone, which is
// required for LazyLock storage without lifetime constraints.
type Parsed = std::result::Result<ThemeSpec, String>;
```

The comment is stale: `Error` *was* not `Clone`, then `Error` was made
`Clone` to support the cache, then the cache was never migrated.
**Clone is load-bearing for nothing.** Dropping it frees us to simplify
`Platform` and `Io` variants.

#### 6b. `Unsupported(&'static str)` conflates message and discriminant

`error.rs:84`: `Unsupported(&'static str)`. The string is both the
display message and the only discriminant. Callers who want to
programmatically react to "macOS feature disabled" must grep the
message.

Actual uses:
- `pipeline.rs:336-338`: `Error::Unsupported("macOS theme detection requires the `macos` feature")`
- `pipeline.rs:351-353`: `Error::Unsupported("Windows theme detection requires the `windows` feature")`
- `pipeline.rs:363-365`: `Error::Unsupported("no theme reader available for this platform")`
- `watch/mod.rs:198-200`: `Error::Unsupported("theme watching not supported for this desktop environment")`
- `watch/mod.rs:213-215`: `Error::Unsupported("theme watching requires both 'watch' and 'macos' features")`

All of these carry structured information (which feature, which
operation) packed into a string.

#### 6c. `Unavailable(String)` is a dumping ground

`error.rs:86`: `Unavailable(String)`. Used for:
- Preset name not found (`presets.rs:103`: `Error::Unavailable(format!("unknown preset: {name}"))`)
- Reader runtime failure (many sites)
- File-not-found-but-expected conditions (portal unavailable, etc.)

These are categorically different events. A misspelled preset name is a
programming bug (caught at the point of use); a portal timeout is a
runtime event (caught with retry logic). Mixing them forces callers to
string-match.

#### 6d. `Format(String)` strips TOML error spans

`error.rs:126-130`:
```rust
impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::Format(err.to_string())
    }
}
```

`toml::de::Error` carries line/column information. Stringifying it is
fine for display but loses the structured information that editors,
linters, and tests might use.

### Options

Each sub-issue has its own options; I'll bundle the recommendation.

**For 6a (Clone bound):**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | Keep `Clone` | No change to users who depend on it | Dead weight; implies features the code doesn't use |
| B | Drop `Clone`. Store `Arc<Error>` at any caller that needs sharing. | Error types stop looking like plain data. Matches ecosystem norm (thiserror-derived errors are rarely Clone). | One-line break for any external user that clones errors. Unlikely to matter in practice. |
| C | Keep `Clone` but simplify `Platform`/`Io` to use `Arc<io::Error>` only, drop the dyn trait object | Less complex error variant | Still keeps an API feature nobody uses |

Recommendation: **B**. Also update the stale comment in `presets.rs:85-92`.

**For 6b (Unsupported):**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | Keep `Unsupported(&'static str)` | No change | Landmines for programmatic error handling |
| B | `Unsupported { reason: UnsupportedReason }` where `UnsupportedReason` is an enum of `FeatureDisabled { name, needed_for }`, `PlatformReader { reader, platform }`, `ReaderUnavailable { reader }` | Structured, matchable, documented | More enum variants; more `#[non_exhaustive]` layers |
| C | Split into multiple top-level variants: `Error::FeatureDisabled { ... }`, `Error::PlatformReader { ... }` | Flat; no nested enum | Top-level `Error` enum grows |

Recommendation: **B or C; prefer C** for flat matching.

**For 6c (Unavailable):**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | Status quo | No change | Programming bugs and runtime errors look the same |
| B | Split out `UnknownPreset { name, known: &'static [&'static str] }` as its own variant | Programming bug is typed | One more variant |
| C | Split into `UnknownPreset`, `ReaderUnavailable { reader, reason }`, `FileMissing { path }` | Full classification | Three more variants |

Recommendation: **C**. The current lumping is what creates the problem.

**For 6d (TOML errors):**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | `Error::Format(String)` (status quo) | No change | Spans lost |
| B | `Error::Toml { source: toml::de::Error }` | Span preserved | `toml::de::Error` is not `Clone`, which (a) forces dropping Clone (fine -- see 6a) and (b) couples the public error type to the `toml` crate version |
| C | `Error::Format { message: String, span: Option<(usize, usize)> }` | No `toml` coupling | Manual span extraction from `toml::de::Error` |
| D | `Error::Format { message: String, line: Option<usize>, column: Option<usize> }` | Similar to C, simpler shape | Same manual extraction work |

Recommendation: **B** unless the `toml` coupling concern is strong. It
isn't -- the error chain is pretty robust and users already handle
`toml::de::Error` in other places.

### Recommended (bundled): all four fixes together

```rust
#[non_exhaustive]
pub enum Error {
    /// Required Cargo feature not enabled at compile time.
    FeatureDisabled { name: &'static str, needed_for: &'static str },

    /// Current platform has no supported detection path.
    PlatformUnsupported { platform: Platform },

    /// Preset name is not recognized.
    UnknownPreset { name: String, known: &'static [&'static str] },

    /// Theme watcher not available for the current desktop environment.
    WatchUnavailable { reason: &'static str },

    /// TOML parse or serialization error, preserving source spans.
    Toml(toml::de::Error),

    /// File I/O.
    Io(std::io::Error),

    /// Theme resolution left fields unfilled.
    ResolutionIncomplete { missing: Vec<FieldPath>, hint: Option<ResolutionHint> },

    /// Platform-specific reader runtime failure with source.
    ReaderFailed {
        reader: &'static str,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}
```

Drop `Clone`. Update `presets.rs` to use `Arc<Error>` if caching is
still desired; otherwise keep the string form. Fold
`ThemeResolutionError` (`error.rs:9-71`) into
`Error::ResolutionIncomplete` -- the separate wrapper exists only to
host a `Display` impl, and that logic can move into the `Error` impl.

### Rationale

The current error type is a mix of "structured enough to match on" and
"stringly-typed escape hatches". The right move is to commit to
structure. Dropping `Clone` is a free win because it is not used
anywhere. Splitting `Unsupported` and `Unavailable` gives callers a
matchable API without giving up display quality (the `Display` impl can
still produce the same strings). Preserving TOML spans is a 10-line
change that makes theme-authoring errors 10× more useful for editor
integrations down the line.

**Confidence:** high on 6a/6c/6d (they are clean wins). Medium on 6b
because the exact enum shape needs design; I have sketched one option
but there are reasonable alternatives.

### Merge-review addendum on 6a: the `presets.rs` stale comment

Verification confirmed the 6a claim exactly: `error.rs:80` derives
`Clone`, and `presets.rs:85-92` still says "`Error is not Clone`" with
a `type Parsed = Result<ThemeSpec, String>;`. When 6a lands and drops
the `Clone` bound, **also**:

1. Delete the stale comment at `presets.rs:85-87`.
2. Change the cache type. Two sensible options:
   - `Result<ThemeSpec, Arc<Error>>` -- preserves error variant info,
     shareable across the LazyLock, one heap alloc per cached error.
   - `Result<ThemeSpec, Error>` -- if `LazyLock<HashMap<_, _>>` can
     hold non-Clone values at the leaf (it can; the `Clone` bound was
     imagined, not required).

The second option is cleaner: `LazyLock<HashMap<&'static str,
Result<ThemeSpec, Error>>>` compiles fine and the map is never cloned
wholesale. The comment was wrong in both directions: `Clone` was never
required for the cache, and stringifying the error was gratuitous.

### Merge-review on 6b: a less-structured alternative worth weighing

The recommendation prefers C (split `Unsupported` into flat top-level
variants). A minor alternative **not** listed above:

| # | Option | Pros | Cons |
|---|---|---|---|
| D' | **Keep `Unsupported(&'static str)` but attach a machine-readable code constant**: `pub const FEATURE_MACOS_DISABLED: &str = "macos-feature-disabled";` etc., and emit `Error::Unsupported(FEATURE_MACOS_DISABLED)`. Callers compare `if err.as_str() == FEATURE_MACOS_DISABLED { ... }`. | Smallest API change; no new variants. Callers get programmatic matching. | Still stringly typed under the hood; easy to typo; no exhaustiveness check. |

D' is genuinely less good than C, but it is the "minimum viable
matchable" option and worth having on the table as a fallback if the
enum-golf for C goes on too long. Keep **C** as the recommendation;
D' is a fallback.

### Merge-review on 6d: `toml::de::Error` is fine for coupling

`toml::de::Error` already appears in the `From` impl, so the public
type already depends on the `toml` crate version implicitly. Making
that explicit via `Error::Toml(toml::de::Error)` adds zero new
coupling. Option B stands as recommended.

---

## 7. `ThemeVariant::resolve*` method proliferation

**File:** `native-theme/src/resolve/mod.rs:14-108`

### Problem

Four public methods:

- `resolve(&mut self)` -- pure inheritance transform (line 34)
- `resolve_platform_defaults(&mut self)` -- fills `icon_theme` from OS (line 52)
- `resolve_all(&mut self)` -- calls both (line 68)
- `into_resolved(self) -> Result<ResolvedThemeVariant>` -- full pipeline including `font_dpi` detection and validation (line 97)

The comment at `resolve/mod.rs:102-104` is instructive:
*"Done here (not in resolve_all) to preserve resolve_all idempotency."*

The author has painted themselves into a corner where `resolve_all` must
be idempotent to satisfy some unwritten contract, so `font_dpi` detection
(which is stateful because it touches the OS) has to live elsewhere.

Grep results from across the codebase:

- Internal tests and `tests/resolve_and_validate.rs:21,65,92,100` call
  `resolve_all()` + `validate()` -- but those are tests, not the public
  API's intended usage pattern.
- `connectors/native-theme-iced/src/lib.rs:163`,
  `connectors/native-theme-gpui/src/lib.rs:196`, and *every other
  connector call site* use **`into_resolved()` exclusively**.

So: 3 of the 4 public methods are used only by tests.

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo.** All four remain public. | No change. | Three methods exist for test-only convenience, confusing new users. |
| B | **Make `resolve`, `resolve_platform_defaults`, `resolve_all` `pub(crate)`.** Keep `into_resolved` public. | Smallest change. Public surface narrows by 75%. Tests can still use the internal methods. | Test file at `tests/resolve_and_validate.rs` is an *integration* test (not `src/`), so it cannot see `pub(crate)` methods. The test must either move to `src/resolve/tests.rs` or switch to `into_resolved`. |
| C | **Delete the three intermediates entirely.** Rewrite `tests/resolve_and_validate.rs` to use `into_resolved`. | Cleanest surface. | The resolution-idempotency tests at `tests/resolve_and_validate.rs:92-101` genuinely need to call `resolve_all` twice and compare; that specific scenario cannot trivially use `into_resolved` (which consumes self). Requires more thoughtful test rewrite. |
| D | **Keep all four but collapse their naming:** `resolve_mut()`, `resolve_mut_with_os_defaults()`, `resolve_mut_all()`, `resolve()` (returning `Result<ResolvedTheme>`). Naming makes the distinction explicit. | Clearer names | Same count of methods, still four |

### Recommended: **B** with test rewrite

Demote `resolve`, `resolve_platform_defaults`, `resolve_all` to
`pub(crate)`. Rewrite the idempotency test to use a pattern that doesn't
need public access to the intermediates:

```rust
// Before (tests/resolve_and_validate.rs:92-101):
variant.resolve_all();
let original = variant.validate().unwrap();
let mut v2 = variant.clone();
v2.resolve_all();
let new_resolved = v2.validate().unwrap();
assert_eq!(original, new_resolved);

// After: move to src/resolve/tests.rs (which has crate-internal access)
// OR use into_resolved twice on clones:
let v = v_before.clone();
let resolved_a = v.into_resolved().unwrap();
let v = v_before.clone();
let resolved_b = v.into_resolved().unwrap();
assert_eq!(resolved_a, resolved_b);
```

### Rationale

Option **A** leaves 3 public methods solely for tests, which is bad API
hygiene. Option **C** is the purest answer but the idempotency test
rewrite is non-trivial -- `into_resolved` consumes `self`, so comparing
"same variant resolved twice" must clone before each call. Workable but
non-obvious.

Option **D** just renames the problem without solving it.

Option **B** threads the needle: the methods survive for the rare power
user who wants to inspect intermediate state, but they don't appear in
rustdoc or code completion. The test file either migrates to `src/` (has
access to `pub(crate)`) or uses the "clone twice, resolve each" pattern.
Both are acceptable.

**Confidence:** high. Simple demotion. The only judgement call is
whether to demote or delete; I chose demote because keeping the internals
accessible to `pub(crate)` callers costs nothing.

### Merge-review refinement: `#[doc(hidden)] pub` instead of `pub(crate)`

The doc's Option B proposes demoting `resolve`, `resolve_platform_defaults`,
and `resolve_all` to `pub(crate)`. This hides them from rustdoc and
downstream code-completion, which is the goal. But it also breaks
**integration tests** that live under `tests/` (not inside `src/`),
including the idempotency test the doc calls out -- and the doc's
suggested "rewrite as clone-twice + into_resolved" loses a subtle
property: `into_resolved` runs the full pipeline (validation, font_dpi
detection), so the rewritten test no longer isolates pure-resolution
idempotency from validation idempotency.

A better alternative that preserves the test:

| # | Option | Pros | Cons |
|---|---|---|---|
| B' | **`#[doc(hidden)] pub`** instead of `pub(crate)` | Hidden from rustdoc (the discoverability win) but still accessible from integration tests and downstream power users who know the name. Idempotency test at `tests/resolve_and_validate.rs:92-101` keeps working unchanged. | Not *hidden* from code completion if the user types the method name; sophisticated tooling may surface it. The method is still part of the SemVer surface (visible to downstream crates that reach for it). |

Recommendation: prefer **B'** to **B** for the three intermediate
methods. The SemVer cost is real but small (nobody uses these today
outside tests -- the doc confirmed this). The test-rewriting cost is
avoided. Rustdoc discoverability -- the actual user complaint -- is
addressed by `#[doc(hidden)]` exactly as well as by `pub(crate)`.

**Updated confidence:** high. B' is a one-attribute change (add
`#[doc(hidden)]` to three functions, no other edits). Drop the test
rewrite entirely.

---

## 8. Icon loading: 13 functions, one user intent

> **Merge-review count correction:** The original heading said "12
> functions". The list below has 13 entries (`load_icon`,
> `load_icon_from_theme`, `load_system_icon_by_name`, `load_custom_icon`,
> `loading_indicator`, `bundled_icon_svg`, `bundled_icon_by_name`,
> `load_sf_icon`, `load_sf_icon_by_name`, `load_freedesktop_icon`,
> `load_freedesktop_icon_by_name`, `load_windows_icon`,
> `load_windows_icon_by_name`). The argument of the section is
> unchanged -- there are too many loaders -- but the number is 13 not 12.
> `is_freedesktop_theme_available` at `icons.rs:121` is a capability
> probe, not a loader, and is excluded from the loader count.

**Files:**
- `native-theme/src/icons.rs:43` (`load_icon`)
- `native-theme/src/icons.rs:95` (`load_icon_from_theme`)
- `native-theme/src/icons.rs:180` (`load_system_icon_by_name`)
- `native-theme/src/icons.rs:232` (`loading_indicator`)
- `native-theme/src/icons.rs:270` (`load_custom_icon`)
- `native-theme/src/icons.rs:121` (`is_freedesktop_theme_available`)
- `native-theme/src/freedesktop.rs:74,111` (Linux)
- `native-theme/src/sficons.rs:102,124` (macOS)
- `native-theme/src/winicons.rs:368,419` (Windows)
- `native-theme/src/model/bundled.rs:30,196` (bundled SVG lookups)

### Problem

Thirteen public functions for "load an icon", with subtly different
parameter shapes:

```
load_icon(role, set, fg_color)
load_icon_from_theme(role, set, preferred_theme, fg_color)
load_system_icon_by_name(name, set, fg_color)
load_custom_icon(&provider, set, fg_color)
loading_indicator(set)
bundled_icon_svg(role, set)
bundled_icon_by_name(name, set)
load_sf_icon(role)                                    // macOS
load_sf_icon_by_name(name)                            // macOS
load_freedesktop_icon(role, size, fg_color)           // Linux
load_freedesktop_icon_by_name(name, theme, size, fg_color)  // Linux
load_windows_icon(role)                               // Windows
load_windows_icon_by_name(name)                       // Windows
```

Cross-platform code must either cfg-gate imports or fall back to the
lowest-common-denominator `load_icon(role, set, fg_color)`, which has
its own limitations (see §9, the hardcoded `24`).

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo.** 12 functions. | No change. | Discoverability is awful. |
| B | **Config struct + single function**: `load(IconRequest) -> Option<IconData>` where `IconRequest` carries role/name/provider, set, size, theme, fg_color | One entry point. Flexible. | Users must construct the struct literal even for the simple case. Optional fields mean either `Default` or `..Default::default()`. |
| C | **Builder pattern**: `IconRequest::new(role).set(set).size(24).color([0,0,0]).load()` | Fluent API. Common case stays short. | Builders are a matter of taste; some users prefer free functions. |
| D | **Two free functions + struct for extras**: `load_icon(role, set, size)` (the common case) and `load_icon_full(&IconRequestExt)` (escape hatch with all options) | Common case has no boilerplate | Two APIs to maintain |
| E | **Preserve the free functions, eliminate the platform-specific ones as public**: keep `load_icon`, `load_icon_from_theme`, etc., but make `load_sf_icon`, `load_freedesktop_icon`, and friends `pub(crate)` | Smallest change. | Doesn't fix the parameter inconsistency across the remaining functions. |

### Recommended: **C** (builder) with a free-function shortcut

```rust
// Common case, no builder needed:
pub fn load_icon(role: IconRole, set: IconSet) -> Option<IconData>;

// Full control via builder:
pub struct IconRequest<'a> { ... private ... }
impl<'a> IconRequest<'a> {
    pub fn new(id: impl Into<IconId<'a>>) -> Self;  // role, name, or &dyn IconProvider
    pub fn set(self, set: IconSet) -> Self;
    pub fn size(self, size: u32) -> Self;          // default 24
    pub fn color(self, rgb: [u8; 3]) -> Self;
    pub fn freedesktop_theme(self, theme: &str) -> Self;
    pub fn load(self) -> Option<IconData>;
    pub fn load_indicator(self) -> Option<AnimatedIcon>;
}

pub enum IconId<'a> {
    Role(IconRole),
    Name(&'a str),
    Custom(&'a dyn IconProvider),
}
```

Hide the 6 platform-specific loaders as `pub(crate)`.

### Rationale

Option **A** has been ruled out above.

Option **B** (config struct literal) is a small step up from free
functions but users end up typing `IconRequest { role: X, set: S, size:
24, theme: None, fg_color: None }` repeatedly. Defaults help but the
struct literal style does not chain.

Option **C** (builder) is the idiomatic Rust answer for "function with
many optional parameters." The pattern is well known and works cleanly
with rustdoc. The common case is served by a free `load_icon(role, set)`
that internally constructs a default builder. Advanced users chain
methods.

Option **D** (two APIs) is fine but requires maintaining two entry
points forever. Builders already give you that for free (short path =
`load_icon(role, set)`, long path = chained builder).

Option **E** is a half-measure. It cleans up the public API but leaves
`load_icon`, `load_icon_from_theme`, and `load_system_icon_by_name` as
three near-duplicate functions.

The builder consolidates all 12 functions into one chained API and one
shortcut. `loading_indicator` merges in as `IconRequest::load_indicator`
which eliminates the special case of "animated icon load path".

**Confidence:** medium-high. Builder is the right answer for this shape
of API, but the exact field set and method names need a round of
bikeshedding. The `IconId` enum is my best guess at how to handle
"role, name, or custom provider" uniformly; the alternative is three
constructors on `IconRequest` (`from_role`, `from_name`, `from_provider`).

### Merge-review refinement: `impl Into<IconId>` instead of exposing `IconId`

The sketch shows `pub enum IconId<'a>` with three variants and has
users write `IconRequest::new(IconId::Role(role))` or
`IconRequest::new(IconId::Name("copy"))`. That's more ceremony than
needed. Better:

```rust
impl<'a> IconRequest<'a> {
    pub fn new(id: impl Into<IconId<'a>>) -> Self { ... }
}

impl From<IconRole> for IconId<'_> { ... }
impl<'a> From<&'a str> for IconId<'a> { ... }
impl<'a> From<&'a dyn IconProvider> for IconId<'a> { ... }
```

Now users just write:

```rust
IconRequest::new(IconRole::ActionCopy).load();
IconRequest::new("doc.on.doc").load();
IconRequest::new(&custom_provider).load();
```

The `IconId` enum becomes an implementation detail -- it's still
public (needed for the `From` impls to work in downstream code), but
users rarely mention it by name. This is the standard Rust pattern
for "accept several natural input types via one method".

One trade-off: with `From<&'a str>`, a user passing a literal
`"sf-symbols"` by accident gets an icon named "sf-symbols" rather
than a compiler error. Consider documenting that the `Name` path
expects platform-specific names (e.g. SF Symbols ids on macOS, FDO
names on Linux), not icon set names. No behavioural change, just a
doc note.

Update the recommendation: same builder shape, use `impl Into<IconId>`
at the constructor. `loading_indicator` becomes
`IconRequest::new(role).load_indicator()` as before.

---

## 9. `load_icon` hardcodes freedesktop size to 24

**File:** `native-theme/src/icons.rs:45-46`

### Problem

```rust
IconSet::Freedesktop => crate::freedesktop::load_freedesktop_icon(role, 24, fg_color),
```

The cross-platform dispatcher takes no `size` parameter and hardcodes
`24` for Freedesktop. But `load_freedesktop_icon` at `freedesktop.rs:74`
*accepts* a size parameter. Users who want 16 or 48 must abandon the
cross-platform layer and call `load_freedesktop_icon` directly, which
doesn't exist on non-Linux, so they end up with `#[cfg]` blocks around
their icon loads.

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Keep the hardcode; document `24` as the only supported size.** | No change | Users who need other sizes are blocked at the API boundary |
| B | **Add a `size` parameter** to `load_icon` | Honors the parameter on Freedesktop; bundled sets ignore it (SVG is vector); SF Symbols and Segoe are configured at render time | One more parameter per call. API bloat. |
| C | **Handle size via §8's builder** -- subsumed by the unified builder. `load_icon(role, set)` keeps the short form (defaults to 24); `IconRequest::new(role).size(48).load()` provides the flexible form. | Part of the larger cleanup; no new surface area | Depends on §8 being accepted. |
| D | **Pass size through a method receiver**: `IconSet::Freedesktop.size(48)` returns a new `IconSet::FreedesktopSized(48)` variant | No API change | Enum variants multiplying; doesn't scale to per-role sizes |

### Recommended: **C**, folded into §8

### Rationale

This is not really a separate issue -- it is a symptom of §8. The icon
API needs one redesign, not two. If §8 is accepted, this disappears.
If §8 is deferred, option **B** is the minimum fix: add `size: u32` as
the third parameter of `load_icon` (and variants).

**Confidence:** high. The cleanup path is clear.

---

## 10. `IconProvider::icon_svg` locks to `&'static [u8]`

**File:** `native-theme/src/model/icons.rs:378-381`

### Problem

```rust
pub trait IconProvider: std::fmt::Debug {
    fn icon_name(&self, set: IconSet) -> Option<&str>;
    fn icon_svg(&self, set: IconSet) -> Option<&'static [u8]>;
}
```

The `'static` lifetime on `icon_svg` excludes any provider whose SVG
bytes are not compile-time static. That rules out:

- Runtime icon loaders (from files, databases, network)
- Providers that decode / decompress at call time
- Providers that cache icons in per-instance storage

The only providers that satisfy the trait today are build-script-
generated ones (via `native-theme-build`).

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change; matches current providers | Blocks runtime providers permanently |
| B | **Return `Option<&[u8]>` (self-lifetime)** | Accommodates providers that store bytes inside themselves | Cannot return a `Box<[u8]>` that outlives self |
| C | **Return `Option<Cow<'static, [u8]>>`** | Static case stays zero-copy; owned case is supported | More complex type in the signature |
| D | **Return `Option<Vec<u8>>`** (owned) | Simplest | Forces a copy for static bytes (the common case), even when it is avoidable |
| E | **Add a second method `icon_svg_owned(&self, set: IconSet) -> Option<Vec<u8>>`** with a default impl that clones from `icon_svg` | Preserves the fast path; runtime providers override the second method | Two methods to keep consistent |

### Recommended: **C** (`Cow<'static, [u8]>`)

```rust
pub trait IconProvider: std::fmt::Debug {
    fn icon_name(&self, set: IconSet) -> Option<&str>;
    fn icon_svg(&self, set: IconSet) -> Option<Cow<'static, [u8]>>;
}
```

### Rationale

Option **A** blocks a legitimate use case. Option **B** with self-
lifetime works for providers that embed bytes but breaks the common
case (bundled, build-generated static arrays) by forcing them through
an owned `Vec<u8>` when they could stay `&'static`.

Option **D** always copies -- slow for the bundled case.

Option **E** works but requires two method definitions and has a
subtle cost: implementers must choose which is "canonical" and override
the right one.

Option **C** (`Cow<'static, [u8]>`) is the one shape that handles both:

- Bundled providers return `Cow::Borrowed(&'static [u8])` -- zero copy.
- Runtime providers return `Cow::Owned(Vec<u8>)` -- one allocation per call.
- Consumers match `.as_ref()` and treat it uniformly.

This is the standard Rust answer for "either borrowed or owned".

**Confidence:** high.

---

## 11. `IconData::Svg(Vec<u8>)` forces a copy on bundled loads

**File:** `native-theme/src/icons.rs:54-62`

### Problem

```rust
#[cfg(feature = "material-icons")]
IconSet::Material => {
    bundled_icon_svg(role, IconSet::Material).map(|b| IconData::Svg(b.to_vec()))
}
```

Every call to `load_icon` for a bundled set does a `.to_vec()` on the
static SVG bytes. For an icon-heavy UI rendering dozens of icons per
frame, that is dozens of allocations for data that never changes.

Also relevant: `IconData::Rgba { data: Vec<u8>, ... }` at
`model/icons.rs:242-249`. Rasterized platform icons genuinely own their
bytes, so that field is correct. But for `Svg`, the source is almost
always `&'static` at the bundled-icon call sites.

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo**: `Svg(Vec<u8>)` | No change | Allocations on every icon load |
| B | **Change to `Svg(Cow<'static, [u8]>)`** | Bundled case is `Cow::Borrowed`, zero copy. Runtime case is `Cow::Owned`. | `Cow` in a public enum may surprise users. |
| C | **Split into two variants**: `IconData::BundledSvg(&'static [u8])` and `IconData::OwnedSvg(Vec<u8>)` | Maximum clarity | Users who just want "the SVG bytes" need to match two variants |
| D | **Add `IconData::svg_bytes(&self) -> &[u8]` accessor**; keep internal representation flexible | Users have a single call; internal shape can change | Requires deciding between `Svg(Cow)` or `Svg(Vec)` or keeping both in a private enum |
| E | **Use `Arc<[u8]>`**: shared-ownership byte slice | Zero-copy share across calls; avoids `'static` lifetime | Slight runtime cost (refcount); less natural for the common static case |

### Recommended: **B** (`Svg(Cow<'static, [u8]>)`), folding in **D**

```rust
#[non_exhaustive]
pub enum IconData {
    Svg(Cow<'static, [u8]>),
    Rgba { width: u32, height: u32, data: Cow<'static, [u8]> },
}

impl IconData {
    /// Borrow the underlying bytes regardless of variant / ownership.
    pub fn bytes(&self) -> &[u8] {
        match self {
            IconData::Svg(c) => c,
            IconData::Rgba { data, .. } => data,
        }
    }
}
```

### Rationale

Option **A** leaves measurable waste on the hot path. Option **C** is
more explicit but makes consumers handle the distinction that they
almost never care about. Option **E** (Arc) works but is unnecessarily
fancy -- `Cow::Borrowed` is strictly zero-cost for the static case,
whereas Arc adds atomic refcounting.

Option **B** is ordinary Rust idiom and removes the allocation on the
common path. Adding the `bytes()` accessor hides the `Cow` for consumers
who just want bytes. The `#[non_exhaustive]` attribute remains so adding
variants stays non-breaking.

**Confidence:** high. This is a straight-ahead optimisation with no
downside beyond the `Cow` in the public shape (which `bytes()` hides).

---

## 12. Flat crate root exports ~70-75 items

> **Merge-review count correction:** The original heading said "80+
> items". Counting the re-exports in `lib.rs:122-203` precisely gives
> about **70-75** items (36 model types + 2 color + 2 error + ~6 icons
> free-fn + 7 detect + 2 pipeline + ~11 feature-gated platform + ~4
> watcher). The "too many for a flat root" argument is unchanged --
> 70+ is still dramatically beyond any reasonable alphabetical scan --
> but the exact number is 70-75, not 80+.

**File:** `native-theme/src/lib.rs:122-203`

### Problem

The crate root re-exports:

- ~36 model types (every widget, every Resolved version, defaults, etc.)
- `Error`, `Result`, `ParseColorError`, `ThemeResolutionError`
- `Rgba`
- 6 icon-loading free functions from `icons` (the originally-reported
  "11" included `detect_icon_theme`/`icon_name`/`system_icon_set`/
  `system_icon_theme` which are `icons.rs` module utilities, plus the
  feature-gated platform loaders)
- 7 detection functions (cached + uncached)
- 5 platform reader functions (`from_kde`, `from_macos`, etc.)
- 6 platform-specific icon functions
- 3 pipeline helpers (`platform_preset_name`, `diagnose_platform_support`, `rasterize_svg`)
- 2 bundled lookup functions
- `LinuxDesktop`, `detect_linux_de`
- Watcher types (`ThemeChangeEvent`, `ThemeWatcher`, `on_theme_change`)

Users arriving at `docs.rs/native-theme` scroll through ~70-75
alphabetically sorted items to find what they need. Discoverability
is awful regardless of whether the exact count is 70, 75, or 80+.

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo.** Flat root. | No change. | Scales poorly with every new widget / function. |
| B | **Keep re-exports but group them in rustdoc** via `#[doc(alias)]` or sections | Cosmetic only; zero code impact. | Rustdoc grouping is limited; no code browsing improvement. |
| C | **Partition into modules**: `native_theme::model::*`, `::icons::*`, `::detect::*`, `::watch::*`, `::readers::*` (internal). Keep a handful of root re-exports for the most common items (`Theme`, `ResolvedTheme`, `DetectedTheme`, `Error`, `Result`, `Rgba`). | Dramatic discoverability win. Users scan a dozen modules, each with 5-15 items, instead of a flat 80-item list. | Every import in user code changes. |
| D | **Full hierarchy (no root re-exports)**. Users always use `native_theme::model::Theme`. | Most principled | Verbose at use site |
| E | **Just rename the crate to `nt` or similar** to shorten root imports | Easy | Doesn't fix the underlying problem |
| F | **C plus a `prelude` module** (added in merge review): `native_theme::prelude::*` contains just the ~6 most-used items (`Theme`, `ResolvedTheme`, `DetectedTheme`, `Rgba`, `Error`, `Result`). Users doing a quick demo write `use native_theme::prelude::*;` and get a working short-form namespace without having to remember which of the ~70 root items made the cut. | Matches ecosystem convention (`diesel::prelude`, `iced::prelude`, `serde::prelude`). Zero new re-exports beyond C -- the prelude is a thin `pub mod prelude { pub use super::{Theme, ...}; }`. The root keeps its six re-exports, and users who prefer explicit imports are unaffected. | One more concept users have to learn ("which is the prelude?"). Slight duplication between the root re-exports and `prelude::*`. |

### Recommended: **C**, optionally extended with **F**'s prelude module

Module layout:

```
native_theme::
    Theme, ResolvedTheme, DetectedTheme     // primary types, re-exported
    Rgba, Error, Result                     // fundamentals, re-exported

native_theme::theme::                       // everything nested in a theme
    Defaults, ResolvedDefaults,
    FontSpec, ResolvedFontSpec, FontSize, FontStyle,
    BorderSpec, ResolvedBorderSpec,
    TextScale, ResolvedTextScale,
    IconSizes, ResolvedIconSizes,
    DialogButtonOrder,
    widgets::*                              // all 25 XxxTheme + ResolvedXxxTheme pairs

native_theme::detect::
    is_dark(), reduced_motion(),
    icon_theme(), icon_set(),
    linux_desktop(),                        // Linux only
    invalidate_cache(),
    detect_is_dark(), detect_reduced_motion(), detect_icon_theme()  // uncached variants
    LinuxDesktop                            // Linux only

native_theme::icons::
    load(request), load_icon(role, set),
    loading_indicator(),
    is_freedesktop_theme_available(),
    IconRole, IconSet, IconData, IconProvider, IconId, IconRequest,
    AnimatedIcon, TransformAnimation

native_theme::watch::                       // unchanged from today
    Subscribe, SubscriptionHandle, ThemeChangeEvent, subscribe()

native_theme::readers::                     // pub(crate) by default
    kde::*, gnome::*, macos::*, windows::*

native_theme::presets::
    list(), list_for_platform(), preset(name),
    PresetInfo                              // new, see §15
```

Root re-exports limited to: `Theme`, `ResolvedTheme`, `DetectedTheme`,
`Rgba`, `Error`, `Result`.

### Rationale

Option **A** doesn't scale. Option **B** is cosmetic. Option **D**
(no root re-exports) is too austere -- even connectors that use
`native_theme::Theme` repeatedly benefit from a short import. Option
**E** trades one problem for another.

Option **C** groups related items into predictable modules. A user
looking for "load an icon" types `native_theme::icons::` and sees 5-10
items; same for detection, watcher, presets. The root stays small
enough to scan at a glance. The `theme` module holds all the data-model
types because they're all components of a single `Theme`, and nesting
them inside that module is honest.

**Confidence:** high. The only judgement call is exactly where the
fence lines go (e.g. should `LinuxDesktop` live in `detect` or at the
root? I chose `detect` because it's detection-adjacent).

### Merge-review note: adding a prelude (Option F) is low-cost

If the crate lands C, adding F is one additional file:

```rust
// native-theme/src/prelude.rs
pub use crate::{Theme, ResolvedTheme, DetectedTheme, Rgba, Error, Result};
```

Plus `pub mod prelude;` at the crate root. ~8 lines total. The cost
is a single extra symbol in rustdoc (the `prelude` module). The
benefit is that new users can copy `use native_theme::prelude::*;`
from the README and have a working short form without scrolling.

Recommend adding F as a small supplement to C. Not blocking -- skip
if the maintainer prefers zero preludes.

---

## 13. Global static caches in `detect` and `model/icons`

**Files:**
- `native-theme/src/detect.rs:55` (`CACHED_IS_DARK`)
- `native-theme/src/detect.rs:587` (`CACHED_REDUCED_MOTION`; line 584 in the original draft, corrected by merge review)
- `native-theme/src/detect.rs:108-116` (`invalidate_caches`)
- `native-theme/src/model/icons.rs:9` (`CACHED_ICON_THEME`)
- `native-theme/src/model/icons.rs:502` (`Box::leak` to produce `&'static str`)

### Problem

Three independent process-wide caches, hidden behind free functions,
invalidated by one coarse `invalidate_caches()` call. Two variants per
detection: `system_*` (cached) and `detect_*` (uncached).

1. **Testability.** Unit tests that manipulate env vars must serialize
   through `test_util::ENV_MUTEX` to avoid races, and must also work
   around stale cache state from earlier tests in the same process.
2. **Granularity.** A "color scheme changed" signal from the OS
   invalidates only dark-mode state, but `invalidate_caches()` also
   drops `CACHED_REDUCED_MOTION` and `CACHED_ICON_THEME`.
3. **Leak.** `Box::leak` for the icon theme string at
   `model/icons.rs:502` is intentional but means the leaked memory
   grows on every invalidation (one `&'static str` per unique theme name).
4. **API split.** Having `system_is_dark` (cached) and `detect_is_dark`
   (uncached) exposed as separate functions forces callers to choose at
   every call site. Most callers want "cached with a way to invalidate",
   which is what `system_is_dark` + `invalidate_caches` already provide --
   but `detect_is_dark` exists and is easy to grab by mistake.

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo.** | No change | All the problems above |
| B | **Add fine-grained invalidation**: `invalidate_is_dark()`, `invalidate_reduced_motion()`, `invalidate_icon_theme()`, `invalidate_caches()` | Addresses granularity | Still global state; testability unchanged |
| C | **DetectionContext struct** owning its own caches, plus a process-wide default via `fn system() -> &'static DetectionContext` | Testable, explicit, still ergonomic for the common case | New type; users must learn about it |
| D | **Drop the cached/uncached split**: keep only `is_dark()`, `reduced_motion()`, `icon_theme()` as the cached form; delete `detect_*` variants. Invalidate via methods on a (possibly global) cache handle. | Fewer names in rustdoc | Users who need live detection call a function that looks like a cached accessor |
| E | **Stop caching altogether**. Every call reads the OS. | Simplest mental model. No staleness possible. | Performance: `system_is_dark()` is called per-frame in connectors (see `showcase-gpui.rs:702` and `showcase-iced.rs:254`). Uncached version on Linux can spawn `gsettings` subprocess every call. |

### Recommended: **C** with **B** as a fallback

```rust
pub struct DetectionContext {
    is_dark:        OnceLock<bool>,
    reduced_motion: OnceLock<bool>,
    icon_theme:     OnceLock<String>,
    icon_set:       OnceLock<IconSet>,
}

impl DetectionContext {
    pub fn new() -> Self;
    pub fn is_dark(&self) -> bool;
    pub fn reduced_motion(&self) -> bool;
    pub fn icon_theme(&self) -> &str;
    pub fn icon_set(&self) -> IconSet;

    pub fn invalidate_is_dark(&self);
    pub fn invalidate_reduced_motion(&self);
    pub fn invalidate_icon_theme(&self);
    pub fn invalidate_all(&self);
}

/// Return the process-wide default DetectionContext.
pub fn system() -> &'static DetectionContext;

// Convenience free functions for the common path:
pub fn is_dark() -> bool { system().is_dark() }
// ... etc
```

Tests construct their own `DetectionContext` and pass it through;
production code uses `native_theme::detect::is_dark()` which forwards
to the global. The `Box::leak` disappears because the `String` lives in
the context. `OnceLock` replaces `RwLock<Option<T>>` because writes only
happen on invalidate-then-read, which can use `OnceLock::take` + re-init
via an outer `Mutex`.

### Rationale

Option **A** leaves the testability pain in place. Option **B** is an
incremental improvement but doesn't fix tests. Option **D** is
attractive but loses the uncached variants entirely; some callers
genuinely want live detection (a watcher callback that re-reads to
confirm the signal was real, for example). Option **E** kills
performance on hot paths.

Option **C** is the clean answer: the `DetectionContext` is the unit of
caching, and tests / alternate runtimes can create their own. The free
functions stay for beginners who don't want to think about it, but they
forward to `system()`. Fine-grained invalidation comes naturally from
having per-field methods.

The one caveat: `OnceLock` is normally write-once. True invalidation
requires either `RwLock<Option<T>>` (same as today) or a more complex
structure. The skeleton above elides this for brevity but the real
implementation needs to thread that through.

**Confidence:** medium-high. The direction is right; the exact
invalidation mechanism needs care.

### Merge-review refinement: use `arc_swap::ArcSwapOption<T>` for invalidation

The doc flags the `OnceLock` write-once problem. Three concrete
primitives solve it; only the first two are worth considering:

| # | Primitive | Reads | Writes | Dependency |
|---|---|---|---|---|
| F1 | `RwLock<Option<T>>` (status quo) | Requires a read lock; blocks on any concurrent writer | Requires write lock; blocks all readers | stdlib |
| F2 | `arc_swap::ArcSwapOption<T>` | Lock-free atomic pointer-load | Lock-free atomic pointer-swap | `arc-swap` crate (1 dep, widely used) |
| F3 | `Mutex<Option<Arc<T>>>` + manual caching | One mutex acquisition per read | One mutex acquisition per write | stdlib |

`ArcSwapOption<T>` is the right primitive for "hot reads, rare
invalidation". Reads are a single atomic load with no lock contention
-- critical for the per-frame callers the doc flags (`showcase-gpui.rs:702`).
Writes (invalidation) are atomic pointer swaps. There is no reader
starvation, and no writer starvation.

`arc-swap` is ~1500 LoC, zero transitive runtime deps, used in
production by rustls, prometheus, rscsh, and the `pfp` serverless
platform. Adding it is a small dependency cost with a meaningful
correctness win. The doc's recommendation (DetectionContext) stands;
use `ArcSwapOption` as the underlying cell type rather than manually
threading `RwLock<Option<T>>` through every field.

If the dependency cost is unacceptable, fall back to F1 (`RwLock`).
Performance is acceptable for the current call volume; the correctness
concern is the read-write starvation interaction, not raw throughput.

---

## 14. `ThemeSpec::lint_toml` hand-maintained duplicate registry

**File:** `native-theme/src/model/mod.rs:540-745`

### Problem

The `lint_toml` function walks a TOML value and reports unknown field
names. Its implementation hand-maintains:

- `const TOP_KEYS: &[&str] = &["name", "light", "dark", "layout"];` (line 554)
- `const VARIANT_KEYS: &[&str] = &[29 widget names];` (line 563)
- `fn widget_fields(section: &str)` -- 26 match arms mapping section
  names to each widget's `FIELD_NAMES` (line 599)
- Hand-coded `font`/`mono_font`/`border`/`icon_sizes` sub-section
  handling for `ThemeDefaults` (lines 649-675)
- Hand-coded `font`/`border` sub-section handling for widget fields
  (lines 701-720)

Adding a new widget requires updates in FOUR places:

1. The widget struct definition in `widgets/mod.rs`
2. The field in `ThemeVariant` at `mod.rs:68-167`
3. The `VARIANT_KEYS` const at `mod.rs:563`
4. The `widget_fields` match at `mod.rs:599-628`
5. Test coverage

The `FIELD_NAMES` const is generated by `define_widget_pair!`, so that's
not duplicated -- but the widget *name registry* is maintained by hand.

Additionally, per CLAUDE memory: `docs/property-registry.toml` and
`docs/platform-facts.md` are the external source of truth for the data
model. The Rust lint function duplicates none of that registry, which
is a hint that the registry exists but is not wired in.

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo.** | No change | Five-place update for every new widget |
| B | **Extract a single `const REGISTRY: &[(&str, &[&str])]` inside `widgets/mod.rs`** that `define_widget_pair!` appends to as it defines each widget | One place to look up widget → fields. | Declarative macros cannot append to a const list; requires either build-time code generation or a runtime-assembled registry. |
| C | **Use serde's `deny_unknown_fields`**: add the attribute to every struct; silently-ignored-field behaviour becomes an error | Zero hand-maintained lint code | Breaks tolerant parsing (users with theme files from older versions get errors for removed fields). Has to be opt-in via a separate parse mode. |
| D | **Generate the linter from `property-registry.toml`** via `native-theme-build` | Single source of truth (the registry), no Rust-side duplication, registry-to-Rust codegen is already the pattern | Build-script complexity; registry must be designed to capture all the information the linter needs |
| E | **Use a proc-macro** (same as §2 option D) to collect field names into a single registry at compile time | One macro-generated registry; no manual list | Needs a proc-macro crate |
| F | **Use the `inventory` crate to build a runtime registry from per-widget submissions** (added in merge review). Each `define_widget_pair!` invocation calls `inventory::submit! { WidgetEntry { name, fields } }`. `lint_toml` iterates `inventory::iter::<WidgetEntry>()` at runtime. The registry is assembled at link time from distributed contributions. | Zero code generation. Declarative macro can submit entries across invocations (which the doc correctly notes it cannot do via a `const`). One dep (`inventory` ~600 LoC, used by sqlx, datatest, tracing-opentelemetry). | Runtime iteration cost on each `lint_toml` call (trivial for a one-shot API). `inventory` uses linker magic that doesn't work on WebAssembly without special handling. Build-target audit needed if WASM is ever a goal. |

### Recommended: **D** long-term (registry-driven), with **F** as a v0.5.7 fallback and **C** as a strict-parse mode

Generate the lint tables from `docs/property-registry.toml` via
`native-theme-build`. Make `deny_unknown_fields` the strict parse mode;
call it via `Theme::parse_strict(&str) -> Result<Theme>`, while
`Theme::parse(&str)` stays tolerant.

### Rationale

Option **A** is the status quo and the complaint is valid. Option **B**
is blocked by the declarative macro (decl macros cannot build a central
registry, only emit per-invocation items).

Option **C** alone is too aggressive -- breaking tolerance on unknown
fields means old theme files stop parsing. As a *mode* (opt-in strict),
it's useful.

Option **E** (proc-macro) is the same path as §2 and should be
coordinated with that decision.

Option **D** (registry-driven) is the best long-term answer because
the registry already exists as the source of truth per CLAUDE memory.
Wiring the Rust code to read the registry at build time means: add a
field to the registry → the linter automatically accepts it, the
struct definition gets the new field, the docs pick it up. One place,
one change.

**Confidence:** medium. The direction is clear, but the design of the
registry schema and the build pipeline that generates Rust from it are
non-trivial and should be discussed before implementation. Flag for
§28.

### Merge-review: Option F (`inventory`) breaks the "declarative macros cannot" wall

The original rationale rejected Option B on the grounds that
"declarative macros cannot append to a const list". That's true of
`const` accumulation, but **not** of runtime registries assembled via
link-time collection. The `inventory` crate exists specifically to do
this: each `inventory::submit!` inside a `macro_rules!` expansion
adds one entry to a process-wide collection that can be iterated at
runtime.

Concretely:

```rust
// in widgets/mod.rs:
pub struct WidgetEntry { pub name: &'static str, pub fields: &'static [&'static str] }
inventory::collect!(WidgetEntry);

// in define_widget_pair! macro (one line added per invocation):
inventory::submit! {
    $crate::model::widgets::WidgetEntry {
        name: stringify!($opt_name),
        fields: $opt_name::FIELD_NAMES,
    }
}

// in lint_toml:
let registry: HashMap<_, _> = inventory::iter::<WidgetEntry>
    .into_iter()
    .map(|e| (e.name, e.fields))
    .collect();
```

Adding a new widget becomes: call `define_widget_pair!`, done. The
`VARIANT_KEYS` const and the `widget_fields` match arms disappear
entirely. No proc-macro, no build script, no registry TOML file.

The cost is one runtime dependency (`inventory`) and one-time
registry collection at the start of `lint_toml`. On WASM, `inventory`
requires a specific initializer mechanism that does not always work;
if WASM support is a goal for `native-theme` now or soon, stick with
D or keep A.

**Recommend** pairing F as the short-term v0.5.7 fix (1 dep, ~20 LoC
change) with D as the longer-term registry-driven evolution. F gives
the drift-hazard win immediately; D subsumes it once the schema
is designed.

---

## 15. `ThemeSpec` method grab-bag

**File:** `native-theme/src/model/mod.rs:243-746`

### Problem

A laundry list of small issues in one type:

#### 15a. `from_toml_with_base` is a one-liner wrapper (mod.rs:448-453)

```rust
pub fn from_toml_with_base(toml_str: &str, base: &str) -> crate::Result<Self> {
    let mut theme = Self::preset(base)?;
    let overlay = Self::from_toml(toml_str)?;
    theme.merge(&overlay);
    Ok(theme)
}
```

That's literally three public-API calls the user could chain themselves.

#### 15b. `with_overlay_toml` is similar (lib.rs:312-315)

```rust
pub fn with_overlay_toml(&self, toml: &str) -> crate::Result<Self> {
    let overlay = ThemeSpec::from_toml(toml)?;
    self.with_overlay(&overlay)
}
```

Adding `_toml` suffixed wrappers doesn't scale -- what about JSON? YAML?

#### 15c. Inconsistent `list_presets` return types

- `list_presets() -> &'static [&'static str]` (mod.rs:478)
- `list_presets_for_platform() -> Vec<&'static str>` (mod.rs:495)

Same conceptual operation, different return types.

#### 15d. `ThemeSpec::new(name)` is mostly useless (mod.rs:245-252)

Creates an empty shell with a name and no variants. Every real use
immediately sets `theme.light = Some(...)` afterward. This is not
better than a struct literal.

#### 15e. `pick_variant` / `into_variant` return `Option` for the "no variants" case (mod.rs:284-315)

If a `ThemeSpec` is constructed with `ThemeSpec::new(name)` and never
has variants attached, `pick_variant(true)` returns `None`. Callers
then write `theme.pick_variant(true).unwrap_or_else(|| ...)` or
`theme.into_variant(true).ok_or(Error::...)`. The `None` case is
almost always a programmer error: no real preset would lack variants.

#### 15f. `list_presets` returns strings, not structured info (mod.rs:478)

Users who want to know "is this a light-only theme?", "is it platform-
specific?", "is it a live variant?" can only parse the name.

### Options per sub-issue

**15a (from_toml_with_base):**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | Keep it | Saves 2 lines at call sites | Bloats surface |
| B | Delete it | Smaller API | Users write 3 lines |

Recommend **B**.

**15b (with_overlay_toml):**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | Keep it | Convenience | API-bloat tax and naming convention that doesn't scale |
| B | Delete it | Smaller API | Users write `ThemeSpec::from_toml(s)?.merge_into(&theme)` |

Recommend **B**.

**15c (list_presets return type):**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | Both return `Vec<...>` | Consistent | Allocates on every call for `list_presets()` |
| B | Both return `&'static [...]` | Zero-alloc | Requires platform filtering at build time, or a const array per target_os |
| C | Both return `impl Iterator<Item = &'static str>` | Zero-alloc, iterable | Iterator API is slightly less ergonomic |
| D | Return a richer `PresetInfo` struct | Carries metadata | Breaking change |

Recommend **C or D**; prefer **D** (see 15f).

**15d (`new(name)`):**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | Keep it | Allows empty-theme construction | Empty themes have no use |
| B | Delete it; rely on struct literal | Smallest API | Struct literal is verbose |
| C | Replace with a builder | Idiomatic Rust for multi-field construction | New type |

Recommend **B** for v0.5.7, **C** if a builder emerges elsewhere.

**15e (pick_variant returning Option):**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | Status quo: return `Option<&ThemeVariant>` | Handles "no variants" case | Makes the common case wordy |
| B | Panic on "no variants" | Common case is clean | Violates no-panic rule |
| C | Return `&ThemeVariant` and require the theme to have at least one variant by construction | Type-level invariant | Needs a new `NonEmptyTheme` type or a construction gate |
| D | Return `Result<&ThemeVariant>` with a dedicated error variant | Common case is `?`-able | Still wordy |

Recommend **D** (revised in merge review, from the original **C**).

**Merge-review re-weigh of C vs D:**

Option C needs either:
1. A zero-size generic marker: `Theme<Populated>` vs `Theme<Empty>`.
   This introduces a type parameter into every public signature that
   mentions `Theme`. Rustdoc gets busier. Users see `Theme<Populated>`
   in error messages and have to understand the distinction.
2. A non-generic "new type" split: `Theme` (always populated) vs
   `ThemeBuilder` (possibly empty). Users construct via the builder
   and convert to `Theme` only when the invariant holds. Two types,
   two sets of methods.

Option D needs one new `Error` variant (`Error::NoVariants` or via
doc 1 §6's restructure) and one `?` per caller that wants to avoid
the `unwrap`. That's strictly cheaper than C's generic / builder
burden for a library that treats `Theme` as its primary public type.

D also composes better with `?` chains: `Theme::preset("dracula")?.pick_variant(true)?`
flows naturally. C would force either `Theme::preset("dracula")?.pick_variant(true)`
(returning `&ThemeVariant` directly, good) **or** an extra generic
parameter threading through every method.

The tipping factor: the "no variants" state is not actually
unreachable. Any external TOML file is allowed to omit both `light`
and `dark` sections -- `ThemeSpec::from_toml(r#"name = "x""#)` parses
successfully today. Making this impossible at the type level either
rejects those TOML files at parse time (a behaviour change) or
requires C's generic split (an API cost).

D is the minimum correct answer: keep `Theme` as one type, return
`Result` for `pick_variant`, accept the one `?` per caller. Move to
C only if D proves insufficient.

**Updated recommendation: D.** Pair with 15d's deletion of
`ThemeSpec::new` to keep the common path clean; pair with 15f's
`PresetInfo` to distinguish "deliberately light-only" from "missing
both" at list time.

**15f (structured preset info):**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | Keep `&'static str` | Zero-alloc | No metadata |
| B | `pub struct PresetInfo { name: &'static str, kind: PresetKind, platform: Platform, has_light: bool, has_dark: bool }` | Rich metadata | More types |

Recommend **B**.

### Recommended (bundled, revised in merge review)

1. Delete `from_toml_with_base` and `with_overlay_toml`. **Also update
   the "hint" message in `ThemeResolutionError::fmt` at `error.rs:63`**
   which currently references `from_toml_with_base()` -- rewrite the
   hint to the two-call idiom so the message does not name a removed
   method. (Cross-referenced from doc 2 §I2.)
2. Make `list_presets()` and `list_presets_for_platform()` both return
   `&[PresetInfo]` (the former is a compile-time constant; the latter
   is a filtered view, which can be a `Vec<PresetInfo>` or an iterator).
3. Delete `ThemeSpec::new(name)`.
4. Change `pick_variant` and `into_variant` to return
   `Result<..., Error::NoVariants>` (Option D in 15e's revised table).
   Do **not** introduce a `Theme<Populated>` generic or a `ThemeBuilder`
   split.

### Rationale

Each sub-issue is small on its own, but together they represent a
grab-bag type that has accreted convenience methods without a consistent
design. v0.5.7 is the right window to prune.

The non-obvious call is 15e. Option **B** (panic) is rejected by the
no-panic rule. Option **C** (type-level guarantee) was the original
recommendation, but merge review re-weighed it against **D** (`Result`)
and found that C's generic or builder cost outweighs the one-`?`
benefit. Option **D** is the revised recommendation; see the extended
discussion inside the 15e table above.

**Confidence:** high on each sub-issue individually. The aggregate
change is a chunk of work but mechanical.

---

## 16. `Rgba` defaults, naming, and conversions

**File:** `native-theme/src/color.rs`

### Problem

Three small issues:

#### 16a. `Rgba::default()` is transparent black

`color.rs:46` derives `Default`, giving `Rgba { r:0, g:0, b:0, a:0 }`.
Documented at `color.rs:42-45` -- but this is a footgun in a theme
library where "alpha 0" means "invisible widget". Most sensible
defaults for theme colors are opaque (white or black), not transparent.

#### 16b. `Rgba::rgba()` is self-named

`color.rs:66-70`:
```rust
#[allow(clippy::self_named_constructors)]
pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
    Self { r, g, b, a }
}
```

The clippy lint exists specifically because self-named constructors
are confusing. `Rgba::rgba(...)` looks like a recursive call.

#### 16c. `to_f32_array` and `to_f32_tuple` are parallel (color.rs:94-112)

Two methods that return identical data in two shapes. Users pick one.
For toolkits that take arrays, one form suffices; users who want a
tuple can destructure the array.

### Options

**16a (default):**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | Keep transparent-black default | Matches CSS; unchanged | Footgun |
| B | Change to opaque black `Rgba::rgb(0,0,0)` | Less surprising default | Changes existing behaviour |
| C | Remove `Default` entirely | Forces explicit construction | `ThemeDefaults` field-by-field construction sites in tests must add explicit values |

Recommend **C**. The `Option<Rgba>` fields in `ThemeDefaults` (which is
most of them) already represent "unset" via `None`, so `Rgba::default()`
is rarely useful outside test helpers.

**16b (rgba method):**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | Keep `Rgba::rgba(...)` | No change | Self-named |
| B | Rename to `Rgba::new(r,g,b,a)` | Clean | One breaking rename |
| C | Rename to `Rgba::with_alpha(r,g,b,a)` | Reads naturally | Longer |

Recommend **B**.

**16c (to_f32_tuple):**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | Keep both | No change | Surface bloat |
| B | Delete `to_f32_tuple` | Smaller API | Users who use it must destructure an array (one extra token) |
| C | Delete `to_f32_array` | Inverse of B | Most toolkits expect arrays |

Recommend **B**.

### Recommended (bundled)

Remove `#[derive(Default)]` from `Rgba`. Rename `Rgba::rgba(...)` →
`Rgba::new(...)`. Delete `Rgba::to_f32_tuple`.

### Rationale

All three are small polish items. 16a and 16c are straight removals.
16b is a rename. None have subtle trade-offs.

**Confidence:** high.

---

## 17. `IconSet::default()` is Freedesktop on all platforms

**File:** `native-theme/src/model/icons.rs:274-295`

### Problem

```rust
#[derive(Default, ...)]
pub enum IconSet {
    SfSymbols,
    ...
    #[default]
    Freedesktop,
    Material,
    Lucide,
}
```

And the doc explicitly admits:
*"This is the `#[default]` variant, so `IconSet::default()` returns
`Freedesktop`. This serves as a serialization-friendly fallback, not a
platform-correct value."*

So the default is **known-wrong** on macOS and Windows. It exists only
for serde round-tripping.

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo.** | No change | Wrong default on 2/3 platforms |
| B | **Remove `#[derive(Default)]`**. Serde handles missing fields via `#[serde(skip_serializing_if = "Option::is_none")]` or by wrapping in `Option<IconSet>`. | No wrong default | Need to audit serde paths to ensure nothing depends on `IconSet::default()` |
| C | **Platform-conditional default** via `#[cfg_attr(target_os = "macos", default = "SfSymbols"), ...]` | Platform-correct | Complex; differs by target |
| D | **Make `#[default]` point to a new `IconSet::Unset` variant** that resolve() replaces with the platform default | Explicit "unset" state | New variant; user match arms must handle it |

### Recommended: **B**

Remove `#[derive(Default)]` from `IconSet`. Callers that need a
platform default call `system_icon_set()` (already exists at
`model/icons.rs:439`), which is platform-aware. Fields like
`ThemeVariant::icon_set: Option<IconSet>` already use the `Option`
shape, so the serde path doesn't actually need `Default` on the enum.

### Rationale

Option **A** is the footgun. Option **C** adds complexity. Option **D**
works but adds a useless variant.

Option **B** is the minimum change. The only question is whether any
call site relies on `IconSet::default()` today; if so, those sites must
migrate to `system_icon_set()`. Quick grep suggests this is a small
number of sites.

**Confidence:** high.

---

## 18. `IconSet::from_name` / `name` duplicates serde

**File:** `native-theme/src/model/icons.rs:297-333`

### Problem

```rust
#[derive(..., Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IconSet {
    SfSymbols,
    #[serde(rename = "segoe-fluent")]
    SegoeIcons,
    Freedesktop,
    Material,
    Lucide,
}

impl IconSet {
    pub fn from_name(name: &str) -> Option<Self> { ... }  // hand-coded match
    pub fn name(&self) -> &'static str { ... }             // hand-coded match
}
```

Three places to update when a variant is added: the enum, the `serde`
rename (if needed), and the two match functions. Misalignment between
serde and the hand-coded pair silently breaks round-tripping.

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Keep the duplication.** | No change | Three-place update; drift risk |
| B | **Derive from-name / to-name via `strum::EnumString` + `strum::IntoStaticStr`** | Removes duplication; serde and strum can share the same rename | Adds `strum` dependency |
| C | **Implement `from_name` / `name` using serde internals** (round-trip through a JSON or TOML string) | No new dependency | Allocation per call; clunky |
| D | **Delete `from_name` / `name`**; require users to use serde for (de)serialization | Smallest API | Users doing ad-hoc name parsing lose convenience |

### Recommended: **A with sync comment** (revised in merge review, from the original **B**)

### Rationale

`strum` is a well-maintained crate, but adding a proc-macro dependency
for the benefit of removing drift risk on a **4-variant enum** is not
a favourable trade. The entire duplication is:

- one `#[serde]` attribute list (already present)
- one ~10-line `from_name` match
- one ~10-line `name` match

That is ~25 lines of hand-written code. A proc-macro crate pulls in
`syn`, `quote`, and `proc-macro2` at compile time (~2 seconds on a
cold build) to save those ~25 lines. The ROI is poor for this
particular issue.

**Revised recommendation:** keep the hand-written `from_name` / `name`
methods but add a compile-time cross-check test:

```rust
#[test]
fn icon_set_names_roundtrip_serde_and_from_name() {
    for set in [IconSet::SfSymbols, IconSet::SegoeIcons, IconSet::Freedesktop,
                IconSet::Material, IconSet::Lucide] {
        // Names from both paths must agree:
        let serde_name = serde_json::to_string(&set).unwrap();
        let serde_name = serde_name.trim_matches('"');
        assert_eq!(serde_name, set.name(),
                   "serde and name() disagree for {set:?}");
        // Round-trip via from_name:
        let parsed = IconSet::from_name(set.name());
        assert_eq!(parsed, Some(set), "from_name roundtrip failed for {set:?}");
    }
}
```

This single test is ~15 lines, catches any drift at CI time, and
costs zero runtime dependencies. Adding a new variant is still two
places (enum + matches), but the drift hazard is eliminated by the
test.

**Weighing against the user's dependency-conservatism preferences**
(per CLAUDE.md memory "careful Rust", "strict, never lie, no unsafe"),
the 4-variant match is a better fit than a proc-macro dep. If the
enum grows to 10+ variants in the future, revisit and add `strum`.

Option **B** (strum) remains acceptable but is no longer preferred.
Option **D** (delete) is still rejected: command-line argument
parsing is a real use case that should not require round-tripping
through serde.

Option **C** is hacky -- no.

**Confidence:** medium-high on the revised recommendation. The test
is a direct, durable fix for the drift hazard without any dependency
cost.

---

## 19. `LinuxDesktop` is not `#[non_exhaustive]`

**File:** `native-theme/src/detect.rs:6-23`

### Problem

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinuxDesktop {
    Kde, Gnome, Xfce, Cinnamon, Mate, LxQt, Budgie, Unknown,
}
```

No `#[non_exhaustive]`. Adding a new variant (say `Hyprland`, `Cosmic`,
`Sway`) is a breaking change because downstream `match` statements that
were exhaustive become non-exhaustive.

Today, Hyprland/Sway/COSMIC all map to `LinuxDesktop::Unknown`
(`pipeline.rs:619-631` tests). This is wrong -- they are known desktops
with their own conventions -- but changing that later is a breakpoint
for any user who matches `LinuxDesktop`.

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo.** | No change | Breaks if new variants are added |
| B | **Add `#[non_exhaustive]`.** | Future-proof | Downstream users must add a `_` arm to existing matches |
| C | **Add `#[non_exhaustive]` and immediately add `Hyprland`, `Cosmic`, `Sway` as variants** | Both future-proof and recognises the current state | Reader logic must be updated to map the new variants to presets (currently they all go to adwaita via `Unknown`) |

### Recommended: **C**

Add `#[non_exhaustive]`. Add `Hyprland`, `Cosmic`, `Sway`, `Niri` as
variants (all current Wayland compositors without a GNOME / KDE
heritage). Map all of them to the `adwaita` preset in
`pipeline.rs:linux_preset_for_de` (they typically use GNOME settings
for theming). Add a test mapping each XDG value to its variant.

### Rationale

Option **A** is bad: v0.5.7 is the right window to add
`non_exhaustive`, and the cost of forcing downstream users to add a `_`
arm is trivial.

Option **B** is acceptable but misses the opportunity to classify
current desktops that today incorrectly map to `Unknown`.

Option **C** does both at once. The cost is minor -- the reader
dispatch just maps the new variants to the same preset they already
get via `Unknown`.

**Confidence:** high.

---

## 20. `icon_set` and `icon_theme` live on the wrong type

**File:** `native-theme/src/model/mod.rs:174-183` (fields on `ThemeVariant`)

### Problem

```rust
pub struct ThemeVariant {
    ...
    pub icon_set:   Option<IconSet>,
    pub icon_theme: Option<String>,
}
```

These live on the light/dark variant. Consequence: a theme can specify
*different* icon sets for light and dark, or *different* icon themes.

In practice this is nonsensical -- you don't switch between SF Symbols
and Freedesktop when the OS goes dark. And `LayoutTheme` already lives
at the `ThemeSpec` level (`mod.rs:240`) precisely because it is shared.
So there is already a precedent for shared-across-variants fields.

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo.** | No change | Cognitive tax; per-variant icon set is almost always a bug |
| B | **Move to `ThemeSpec`**, same level as `layout` | Consistent with `layout`; matches intuition | Breaking change for any user that set `variant.icon_set` |
| C | **Move to `ThemeSpec` but keep `ThemeVariant::icon_set_override: Option<IconSet>`** as an optional per-variant override for the rare case | Both forms supported | Two fields for one concept; users must know the precedence |

### Recommended: **B**

Move `icon_set` and `icon_theme` to `Theme` (the new name for
`ThemeSpec`). Remove them from `ThemeLayer` (the new name for
`ThemeVariant`).

### Rationale

Option **A** leaves a real footgun. Option **C** preserves flexibility
but nobody has asked for it; YAGNI applies.

Option **B** is the minimum change. If a real use case emerges for
per-variant overrides, we can add 15c's style of override field
later -- that's an additive, non-breaking change.

**Confidence:** high.

---

## 21. `ThemeWatcher` struct internals and constructor split

**File:** `native-theme/src/watch/mod.rs:91-160`

### Problem

```rust
pub struct ThemeWatcher {
    shutdown_tx:       Option<mpsc::Sender<()>>,
    thread:            Option<JoinHandle<()>>,
    platform_shutdown: Option<Box<dyn FnOnce() + Send>>,
}
```

Three internal fields of three different shapes. The `Debug` impl at
`watch/mod.rs:97-108` goes out of its way to print `"..."` for the
boxed closure field. There are two `pub(crate)` constructors -- `new`
and `with_platform_shutdown` -- that differ only in whether they accept
the platform shutdown closure.

The public API is fine -- users call `on_theme_change(cb)` and drop the
returned watcher. But the internals are more complex than the problem
they solve.

Also: the type is `Send` but not `Sync` (confirmed by the test at
`watch/mod.rs:313-316` which asserts `Send` only). This is undocumented
in the struct doc.

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo.** | No change | Fine for users; ugly internals |
| B | **Rename** to `ThemeSubscription` or `ThemeWatchHandle` to match "drop-guard" semantics | More accurate name | One rename |
| C | **Collapse the two constructors** into one: `pub(crate) fn new(shutdown_tx, thread, platform_shutdown: Option<Box<...>>)` | Single constructor | Same internal state |
| D | **Replace the `Box<dyn FnOnce()>` with a typed enum** `PlatformShutdown { CfRunLoop(...), PostThread(...), None }` | Inspectable, testable | Platform-specific types in a cross-platform struct; cfg-heavy |
| E | **Document `Send + !Sync` explicitly** in the struct doc comment | Removes ambiguity | Doc-only change |

### Recommended: **B + C + E**

Rename `ThemeWatcher` → `ThemeSubscription`. Collapse the two internal
constructors into one that accepts `Option<Box<dyn FnOnce() + Send>>`
for the platform shutdown (caller passes `None` or `Some(boxed)`). Add
a line to the doc comment: *"`ThemeSubscription` is `Send` but not
`Sync`. To share theme-change events across threads, send the
`ThemeChangeEvent` through an `mpsc::Sender` from inside the callback
(as shown in the example)."*

Leave the boxed-closure as-is; option **D** adds cfg complexity that
isn't worth it for an internal field.

### Rationale

The user-facing API (`on_theme_change` returning a drop-guard) is
correct. The complaints are all about naming and internals. Renaming
is free; collapsing constructors is free; documenting Send/!Sync is
free. The boxed-closure internal is fine and changing it would make
the code harder to read, not easier.

**Confidence:** high.

---

## 22. `on_theme_change` runtime-errors instead of compile-errors on missing features

**File:** `native-theme/src/watch/mod.rs:183-241`

### Problem

```rust
pub fn on_theme_change(callback: ...) -> crate::Result<ThemeWatcher> {
    ...
    #[cfg(target_os = "macos")]
    {
        #[cfg(all(feature = "watch", feature = "macos"))]
        {
            return macos::watch_macos(callback);
        }
        #[cfg(not(all(feature = "watch", feature = "macos")))]
        {
            let _ = callback;
            return Err(crate::Error::Unsupported(
                "theme watching requires both 'watch' and 'macos' features",
            ));
        }
    }
    ...
}
```

The function exists at compile time regardless of feature flags. A user
on macOS who forgets `watch + macos` gets a runtime `Err(Unsupported)`
at the first call, instead of a compile error saying "function
unavailable".

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo.** | Consistent function availability across cfgs | Runtime errors where compile errors are possible |
| B | **Feature-gate the function entirely**: `#[cfg(feature = "watch")] pub fn on_theme_change(...)`. Users without the feature get "function not found" at compile time. | Fast fail | Cross-platform code has to cfg-gate the call |
| C | **Add a compile-time capability constant** `pub const WATCH_AVAILABLE: bool = cfg!(feature = "watch")`. Users can branch on it. Function always exists but only does useful work when the feature is enabled. | Users choose at runtime | Verbose |
| D | **Stub the function when the feature is missing**, but return `Err(Unsupported)` at runtime (status quo behaviour) | No change | Same as A |

### Recommended: **B**

Gate the function and all watch-related types behind `#[cfg(feature =
"watch")]`. Users without the feature get clean compile errors pointing
at the missing dependency.

### Rationale

Option **A**/**D**: runtime-error-for-missing-feature is bad API
hygiene. Users should find out at `cargo build`, not at first call.

Option **C** is clever but verbose -- users still write feature-gate
conditionals, just on a `const` instead of a `#[cfg]`.

Option **B** is the idiomatic Rust answer: features gate visibility.
Cross-platform code that wants the watcher must require the feature
via `features = ["watch"]` in its `Cargo.toml`. Code that doesn't want
the watcher doesn't compile it in at all.

**Confidence:** high.

---

## 23. `diagnose_platform_support` returns `Vec<String>`

**File:** `native-theme/src/pipeline.rs:176-272`

### Problem

Returns a vector of human-readable strings. Users can print the
vector, but cannot programmatically check specific conditions
("does gsettings exist?", "is the `macos` feature enabled?").

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo.** | No change | Stringly typed |
| B | **Return a struct** `DiagnosticReport { platform, desktop: Option<LinuxDesktop>, gsettings: Option<String>, kde_config_path: Option<PathBuf>, features: FeatureFlags }` with a `Display` impl that produces the current string format | Structured and readable | Breaking change |
| C | **Return both**: a struct with `Display` impl, plus a `to_strings() -> Vec<String>` convenience | Migration-friendly | Two APIs |

### Recommended: **B**

Return a typed `DiagnosticReport`. The `Display` impl reproduces the
current line-by-line format for users who just want to print it. Users
who want to test specific conditions match fields.

### Rationale

The current shape works, but v0.5.7 is a breaking window and the
structured form is strictly better.

**Confidence:** high.

### Merge-review refinement: per-entry typing

The doc sketches the report as a struct with several typed fields
(`platform`, `desktop`, `gsettings`, `kde_config_path`, `features`).
A cleaner shape that avoids the "one field per diagnostic" explosion:

```rust
pub struct DiagnosticReport {
    pub platform: Platform,
    pub entries: Vec<DiagnosticEntry>,
}

pub enum DiagnosticEntry {
    Detected(String),              // e.g. "XDG_CURRENT_DESKTOP: KDE"
    DesktopEnv(LinuxDesktop),
    Missing { tool: &'static str, impact: &'static str },
    FeatureEnabled(&'static str),
    FeatureDisabled(&'static str),
    KdeConfig { path: PathBuf, exists: bool },
}

impl fmt::Display for DiagnosticReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in &self.entries { writeln!(f, "{entry}")?; }
        Ok(())
    }
}
```

Why this is better than the doc's struct-with-many-fields:

1. New diagnostic categories add variants to `DiagnosticEntry`, not
   fields to `DiagnosticReport`. `#[non_exhaustive]` on the enum
   preserves forward-compat.
2. Users who just want strings get a working `Display` from the same
   shape.
3. Users who want programmatic inspection iterate `report.entries`
   and pattern-match the variant they care about.
4. The `Vec<DiagnosticEntry>` shape is closer to the existing
   `Vec<String>` than a flat struct, so the migration diff is
   smaller.

Recommend the per-entry typing over the struct-with-many-fields
sketch. Same Option B, cleaner shape.

---

## 24. `platform_preset_name` leaks the internal `-live` convention

**File:** `native-theme/src/pipeline.rs:133-150`

### Problem

```rust
pub fn platform_preset_name() -> &'static str {
    #[cfg(target_os = "macos")] { return "macos-sonoma-live"; }
    #[cfg(target_os = "windows")] { return "windows-11-live"; }
    ...
}
```

The function returns `"macos-sonoma-live"`, exposing the internal "live
preset" convention to users. Callers who pass this name to
`ThemeSpec::preset` get a geometry-only preset without colors (live
presets are merge bases, not complete themes). If the `-live` suffix
convention is ever renamed, external callers break.

The only current user is the showcase at
`connectors/native-theme-gpui/examples/showcase-gpui.rs:180` (which
uses it to build a "default (...)" label).

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Leaked convention |
| B | **Strip `-live` before returning**: `platform_preset_name() -> &'static str` returns `"macos-sonoma"` on macOS | No convention leak | Changes the public value of the function |
| C | **Return structured info**: `PlatformPreset { display: &str, base_name: &str, live_name: &str }` | Fully expressive | Breaking |
| D | **Rename to `platform_live_preset_name` and add `platform_preset_name` returning the base** | Both forms available | Two functions |

### Recommended: **C**

Return a struct. Give it a `Display` impl that returns the base name
(what a user label wants). Give it accessors for the internal live
variant name, which is what the pipeline uses internally.

### Rationale

The breaking change is justified -- callers who need both the user-
facing name and the internal variant get both, and the `-live`
convention becomes an internal detail again.

**Confidence:** medium. If v0.5.7 does not want to break this
particular function, **B** (strip `-live`) is an acceptable minimum.
The showcase needs to be updated either way.

---

## 25. `FontSize::Px(v).to_px(dpi)` silently ignores the DPI parameter

**File:** `native-theme/src/model/font.rs:44-49`

### Problem

```rust
pub fn to_px(self, dpi: f32) -> f32 {
    match self {
        Self::Pt(v) => v * dpi / 72.0,
        Self::Px(v) => v,    // dpi is unused
    }
}
```

A user passing `FontSize::Px(16.0).to_px(200.0)` might reasonably
expect the pixel value to scale with DPI. It does not. The DPI
parameter is silently ignored for the `Px` case.

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Surprising |
| B | **Split the methods**: `Pt(v).to_px(dpi)` and `Px(v).get()` as separate methods on separate types, requiring users to match before calling | Explicit asymmetry | Two functions instead of one |
| C | **Rename `to_px` to `resolve(dpi)` and document the asymmetry clearly** | No behavioural change; just clarity | Still silent about the ignored case |
| D | **Make `to_px` take `&self` only and look up the unit first**; require users to call `.resolve(dpi)` on a separate unit-resolver | Same as B | More verbose |

### Recommended: **C** (minimal) or **B** (principled)

### Rationale

**C** is the minimum change that addresses the complaint: rename to
signal "this is a unit conversion step" rather than "this is always
pixel math". **B** is more principled but invites API churn for
little benefit. I lean **C** for pragmatism.

**Confidence:** medium. The complaint is mild; either solution works.

---

## 26. `#[must_use]` messages on value types are preachy

**File:** `native-theme/src/model/mod.rs:225`, `native-theme/src/lib.rs:353`, others

### Problem

```rust
#[must_use = "constructing a theme without using it is likely a bug"]
pub struct ThemeSpec { ... }

#[must_use = "this returns the detected theme; it does not apply it"]
pub fn from_system() -> crate::Result<Self> { ... }
```

Two sub-complaints:

- **26a.** `#[must_use]` on a value *type* fires even during legitimate
  partial construction (like "build a theme step by step in a test
  helper"). The message "is likely a bug" is moralising.
- **26b.** `#[must_use]` on a function returning `Result` is redundant
  -- `Result` is already `#[must_use]`. The custom message also asserts
  "it does not apply it", which is misleading in a library that has no
  concept of "applying" themes (that's the connectors' job).

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Small irritation |
| B | **Remove `#[must_use]` from the struct**, keep on functions (with shorter messages) | Eliminates 26a | Users who discard a freshly built `Theme` get no warning |
| C | **Remove all custom `must_use` messages**, keep the attribute for tidiness | Shorter code | Loses the lint guidance |
| D | **Remove both `#[must_use]` from the struct and the custom messages on functions** | Cleanest | Loses warnings |

### Recommended: **B**

Remove `#[must_use]` from `ThemeSpec` (and its renamed successor
`Theme`). Keep `#[must_use]` on fallible functions but simplify the
messages to neutral descriptions like `"this returns the detected theme"`.

### Rationale

The attribute on a value type is the real irritant; users building a
theme have perfectly legitimate reasons to discard it (testing,
comparison, default values). The attribute on functions is harmless;
the messages just need to drop the editorial "it does not apply it"
that does not match the library's role.

**Confidence:** high.

---

## 27. Priority summary

Ordered by (impact × ease), highest first:

| Priority | Issue | Impact | Effort | Confidence |
|---|---|---|---|---|
| P0 | §1 Type vocabulary rename | Very high | Low (mechanical) | High |
| P0 | §4 Drop `active()`, keep `pick(is_dark)` | High | Trivial | High |
| P0 | §12 Partition crate root into modules | High | Medium | High |
| P0 | §6a Drop `Error::Clone` bound | Medium | Trivial | High |
| P0 | §19 Add `#[non_exhaustive]` to `LinuxDesktop` + new variants | Medium | Trivial | High |
| P0 | §16 `Rgba` polish (defaults, naming) | Low each, medium total | Trivial | High |
| P1 | §6b/c/d Restructure `Error` variants | High | Medium | High (structure); medium (exact shape) |
| P1 | §7 Demote `resolve*` intermediates to `pub(crate)` | Medium | Trivial | High |
| P1 | §8 Icon API builder | High | Medium | Medium-high |
| P1 | §15 `ThemeSpec` method grab-bag cleanup | Medium | Low | High |
| P1 | §20 Move `icon_set`/`icon_theme` to `Theme` | Medium | Trivial | High |
| P1 | §22 Feature-gate `on_theme_change` | Medium | Trivial | High |
| P1 | §17 Remove `IconSet::default()` | Low | Trivial | High |
| P1 | §3 Eliminate `SystemTheme` pre-resolve fields | Medium | Medium | Medium |
| P2 | §5 Unify `from_system_async` / `from_system` | High | High (may need `zbus::blocking` migration) | Medium |
| P2 | §13 `DetectionContext` | Medium | Medium | Medium |
| P2 | §10 `IconProvider::icon_svg` → `Cow<'static, [u8]>` | Low | Trivial | High |
| P2 | §11 `IconData::Svg` → `Cow<'static, [u8]>` | Medium (hot path) | Trivial | High |
| P2 | §24 `platform_preset_name` structured return | Low | Low | Medium |
| P2 | §23 `diagnose_platform_support` structured return | Low | Low | High |
| P3 | §2 Macro-generated doubled struct hierarchy | High | Very high | Medium |
| P3 | §14 Registry-driven `lint_toml` | Medium | High (design + codegen) | Medium |
| P3 | §18 Drift-guard test for `IconSet::from_name` (revised from strum) | Low | Trivial | High |
| P3 | §21 `ThemeWatcher` rename + doc | Low | Trivial | High |
| P3 | §25 `FontSize::Px(v).to_px(dpi)` rename to `resolve` | Low | Trivial | Medium |
| P3 | §26 Trim `#[must_use]` messages | Very low | Trivial | High |
| P3 | §9 `load_icon` size hardcode | Low (covered by §8) | Trivial | High |

**P0** items should ship in v0.5.7 with no further design discussion.
**P1** items need design discussion on specific details (exact Error
shape, icon builder method names, etc.) but the direction is clear.
**P2** items depend on larger work (async runtime decision,
DetectionContext design) and should not hold up P0/P1.
**P3** items are either very large (§2, §14 needing proc-macro work)
or very small polish that can slip to v0.5.8.

---

## 28. Open questions / items requiring discussion

These are points where I am not absolutely sure of the recommended
solution and they warrant a conversation before implementation:

1. **§1: "DetectedTheme" vs "ThemeBundle" as the rename target for
   `SystemTheme`.** Both work; "DetectedTheme" emphasises provenance,
   "ThemeBundle" emphasises shape. Needs a quick "which reads better
   in context" check.

2. **§2: Proc-macro infrastructure.** The widget-pair macro is the
   biggest design choice in the crate. Moving to a proc-macro that
   reads `property-registry.toml` is the right long-term direction,
   but the build pipeline and registry schema need design. If this
   can't happen in v0.5.7, defer and keep the existing
   `define_widget_pair!`.

3. **§3: Exact shape of `OverlaySource`.** Must preserve enough
   information to replay the pipeline. In particular, `font_dpi`
   currently lives in `into_resolved` (lazy); moving it into
   `OverlaySource` means capturing it eagerly during `from_system`,
   which touches the detect module.

4. **§5: `zbus::blocking` for the portal?** The ideal "one code path,
   no async runtime" answer depends on whether `zbus::blocking` can
   fully replace `ashpd` for the settings portal. If not, we fall
   back to `pollster::block_on(ashpd_call())` which adds `pollster`
   as a dependency but removes `tokio`/`async-io` as choices.

5. **§6: Exact Error enum shape.** I sketched one option but the
   variant split is not obvious. Should `WatchUnavailable` be a
   separate variant or a subcase of `FeatureDisabled`? Should
   `ReaderFailed` subsume `Io`? Worth an hour of enum golf.

6. **§13: `DetectionContext` invalidation.** `OnceLock` is
   write-once; true invalidation needs `RwLock<Option<T>>`. Need to
   settle on a primitive that supports both "cache on first read"
   and "force re-read on demand" without being clunky.

7. **§14: `property-registry.toml` schema for codegen.** If the lint
   logic (and potentially the widget structs themselves) are
   generated from the registry, the registry must capture:
   serde rename rules, field types, field categories, per-field docs.
   That's a bigger design task than the lint function itself.

8. **§18: `strum` dependency acceptable?** (Merge-review update: the
   recommendation is now **A with a drift-guard test**, not B with
   strum. The crate's dependency-conservatism per CLAUDE memory argues
   against a proc-macro dep for a 4-variant enum. Keep this open
   question only if the enum grows to 10+ variants or a future
   maintainer prefers the strum path.)

9. **§24: Is `platform_preset_name` a stable contract?** It has one
   known external user (the gpui showcase). Breaking it requires
   updating the showcase, which is in the same tree -- fine. But if
   downstream users outside this tree depend on the current return
   value, they break without notice. Low-probability but flag it.

10. **Stale `todo.md`**: `docs/todo.md` contains a "Post-1.0 /
    Deferred" section saying "ship without change notification" --
    this is contradicted by v0.5.6 which implemented the watcher.
    The top-level `todo.md` needs a pass to remove stale items. Not
    blocking this document but worth a cleanup pass.

---

## Post-script: issues I deliberately excluded

The following are either out of scope for an API critique or covered
by other v0.5.x archived work:

- Preset value mismatches vs `platform-facts.md` (covered in
  `docs/archive/v0.5.4_native-theme.md`).
- Internal reader correctness (KDE, GNOME, macOS, Windows readers).
- Test coverage adequacy (a separate concern).
- Documentation completeness beyond the specific doc-vs-code issues
  noted above.
- MSRV compatibility impact of recommended changes.
- `native-theme-build` API (covered separately).
- Connector APIs (covered in `docs/todo_v0.6.0_iced-full-theme-geometry.md`
  and `docs/todo_v0.6.1_gpui-full-theme.md`).

---

## 29. Merge-review verification notes

This section was added after a second verification pass through the
codebase (reading every cited file:line against the current tree) and
summarises the corrections and additions that were folded back into
the sections above. It is a navigation aid, not a change log; each
item points at the section where the correction was made in-line.

### 29.1 Verified claims

Every file:line reference in §1–§26 was re-checked against the
current tree. All issues are real. All code snippets match the
source (off by ≤3 lines in a couple of places, documented in-line
where the offset was corrected). No fabricated values found. The
original document is honest and thorough.

### 29.2 Corrections applied in-line

| Section | Correction |
|---|---|
| §8 | Count was "12" functions; re-verified list has 13. Heading updated. |
| §12 | Count was "80+" items; re-verified count is ~70-75. Heading updated. |
| §13 | `CACHED_REDUCED_MOTION` was at `detect.rs:584`; actual line is 587. |
| §15 | `from_toml_with_base` removal needs to coordinate with the hint message at `error.rs:63` which names it. Cross-reference added in §15 recommended-bundled block. |
| §15e | Original recommendation **C** (type-level "at least one variant" invariant) re-weighed against **D** (return `Result`). New recommendation: **D**. Full rationale in 15e's updated table. |
| §18 | Original recommendation **B** (add `strum`) reversed to **A with drift-guard test**, reflecting the crate's dependency-conservatism. |

### 29.3 New options added in-line

| Section | New option |
|---|---|
| §1 | **F** — module disambiguation (no rename; move types under `theme::`). |
| §5 | **G** — keep `from_system` and `from_system_async`, sync wraps `pollster::block_on(async_inner)`. Strict superset of B. **Now recommended over B.** |
| §6b | **D'** — machine-readable code constants on `Unsupported(&'static str)`. Fallback if C's flat variants are rejected. |
| §7 | **B'** — `#[doc(hidden)] pub` instead of `pub(crate)` to preserve the integration test. **Now recommended over B.** |
| §8 | Use `impl Into<IconId>` at the constructor instead of exposing `IconId` directly. Refinement inside C. |
| §12 | **F** — C plus a `prelude` module with the 6 most-used items. Small supplement to C. |
| §13 | **F** — use `arc_swap::ArcSwapOption<T>` for the invalidation cell. Refinement to C. |
| §14 | **F** — `inventory` crate for link-time registry collection. Short-term path that unblocks the "declarative macros cannot" wall without a build script. |
| §23 | Per-entry typing refinement (`Vec<DiagnosticEntry>`) inside the doc's Option B. |

### 29.4 Cross-references to doc 2

| Doc 2 item | Doc 1 interaction |
|---|---|
| A1 | **Already fixed** in commit `f9e5956`. No longer blocks v0.5.7. |
| A3 | Depends on doc 1 §6 error restructure. Fold A3 into §6 if both land together. |
| A4 | Resolves naturally if doc 1 §7 demotes `resolve` to `pub(crate)` / `#[doc(hidden)]`. Otherwise A4's own fix (move button_order to `resolve_platform_defaults`) is needed. |
| B1/B2/B7 | All three depend on the same codegen/registry infrastructure that doc 1 §2/§14 discuss. Design once, use three times. |
| B5 | `ResolutionContext` in doc 2 B5 is the natural home for doc 1 §3's `OverlaySource` DPI capture. Design §3 and B5 together (see §3's merge-review addendum). |
| C4 | Migrating `family: String → Arc<str>` needs the `serde` `rc` feature flag. Noted in doc 2 C4's merge-review addendum. |

### 29.5 Priority re-tiering from merge review

Updates to §27's priority table:

- **§18**: Demoted from "P3 strum dependency" to "P3 drift-guard test"
  with **High** confidence (previously Medium). The work is smaller
  and the confidence is higher because it no longer depends on a
  dependency decision.
- **§5**: Still P2, but the recommendation is now Option G (pollster
  wrapping) which is a smaller migration than B (rewrite + dep audit)
  or zbus::blocking (which may not cover the full surface). The
  confidence on the recommendation is higher, even if the priority
  tier is unchanged.
- **§7**: Confidence still high; implementation simpler (B' is one
  attribute per method, no test rewrite). Priority unchanged.

No new P0/P1 items. No items moved up from P2/P3 to P0/P1. The
merge-review corrections are all in-section refinements or
recommendation re-weighs, not priority shifts.

### 29.6 Items the merge review found but did not make new sections for

These were noted during verification but either belong to doc 2 or
are too small to warrant their own section in doc 1:

1. **`presets.rs:85-92` stale comment about `Error: !Clone`** -- see
   doc 2 §I1. Folded into doc 1 §6a's merge-review addendum as a
   concrete cleanup target.
2. **`run_gsettings_with_timeout` 2-second worst case** at
   `detect.rs:138-177` -- see doc 2 §I3. Not a new option in §13, but
   a latency consideration the `DetectionContext` redesign should
   account for.
3. **`FontSize::default() = Px(0.0)` at font.rs:66-70** as a second
   root cause of doc 2 A2's spurious errors -- see doc 2 A2's
   merge-review addendum. Not a doc 1 issue; noted for completeness.
4. **`inheritance-rules.toml` vs `inheritance.rs` drift** -- this is
   doc 2 B2 and is already covered there. Doc 1 §14's codegen
   recommendation should be designed to subsume B2.
5. **`lint_toml` has ~215 hand-maintained string literals total** --
   see doc 2 §I5 for the count. Doc 1 §14's ROI number.

### 29.7 Confidence statement (merge review)

**High confidence** on:

- Every file:line reference resolves against the current tree (with
  the 3 documented offsets).
- A1 is fixed in commit `f9e5956`.
- Every issue in doc 1 is real and reproducible.
- The P0 cohort is correct and shippable as a coherent v0.5.7 release.

**Medium confidence** on:

- The exact shapes of the added options (§1 F, §5 G, §12 F, §13 F,
  §14 F). Each has been reasoned about from first principles but
  not prototyped end-to-end.
- The revised recommendations (§7 B', §15e D, §18 A-with-test). Each
  is supported by an explicit weighing against the original
  recommendation, but reasonable maintainers could disagree.

**Low confidence** on:

- Precise sizing of the codegen spike (doc 1 §2 + §14 plus doc 2
  B1/B2/B7). The design work is non-trivial and the prototype will
  surface issues I cannot anticipate from static reading alone.
- The §1 fifth-rename judgement (`DetectedTheme` vs `ThemeBundle`).
  I lean `DetectedTheme`, doc 1 agrees, but both are defensible.
- B4 accessibility split (doc 2). The philosophical question "is
  high_contrast a theme property or a user preference?" admits
  multiple defensible answers. Flagged in doc 2 open questions.

### 29.8 What was NOT changed

Deliberately preserved from the original document:

- The P0 cohort from §27 and doc 2 §H (except A1 removal).
- The overall recommendation direction for §2 (proc-macro codegen
  is still the long-term answer, even with §14 F adding an
  inventory-based short-term path).
- The §28 open questions list (added one clarification, did not
  remove any).
- Every pros/cons entry in the original option tables.

If a section does not have a "merge-review" sub-heading, it is
unchanged from the original analysis.

---

## 30. Third-pass review: deep-ultrathink refinement under "no backward compat"

This section records a third pass performed under the explicit
"backward compatibility does not matter; I want the perfect API"
directive. Where earlier passes hedged under migration risk, this
pass commits harder to the optimal answer. Where earlier passes
already reached the optimal answer, no change is made and no new
section is added.

New options are appended to existing problems as Option **G**,
**H**, etc. -- the existing option letters are not disturbed.
Cross-cutting findings (a bug the earlier passes did not flag,
priority rebalancing) follow in §30.3–§30.5.

### 30.1 Methodology

The pass re-read each existing recommendation without first
consulting §29's merge-review notes, then reconciled. Where this
pass reaches a different conclusion, the rationale is stated
explicitly. Where this pass agrees with earlier passes, no change
is made.

The pass was **conservative about adding new issues** (the existing
documents are already thorough) and **aggressive about strengthening
recommendations** that were hedged for migration concerns. Every
new claim was independently verified against the current tree;
file:line references are exact.

### 30.2 New options added to existing problems

#### §2 — Option G: elide docs on Resolved variants via macro change

The `define_widget_pair!` macro at `widgets/mod.rs:48-156` currently
copies the Option struct's doc attributes to the Resolved struct via
`$(#[doc = $opt_doc])*`. That is the "60+ duplicated doc blocks"
complaint from §2's problem statement.

An alternative that addresses **only** the doc-burden pain point
without moving to full codegen infrastructure:

| # | Option | Pros | Cons |
|---|---|---|---|
| G | **Edit the macro to emit a single `/// See [`XxxTheme::field`] for documentation.` line on each resolved field instead of copying the full doc block** | Macro change is ~15 lines. Zero public-API change. Rustdoc becomes a pair of linked pages instead of a wall of duplicated text. Resolved variant's rustdoc shrinks by ~70%. Preserves every type name, test, and connector reference. | Does not fix rename-drift (still two structs). Does not fix `check_ranges` duplication. Does not eliminate the macro DSL burden. Strictly a partial fix to §2's doc-burden complaint. |

**Position:** G is an 80/20 stepping stone that addresses the
doc-burden pain immediately (~1-day change) while D (registry-driven
codegen) remains the systemic fix. G and D are not mutually
exclusive — ship G for immediate rustdoc quality, ship D for
drift elimination.

**Recommendation under "perfect API":** ship **both G and D**.
G is a small-effort P0 for v0.5.7 (visible rustdoc quality
improvement); D is the larger P1 investment (drift elimination,
single source of truth). Under "no backward compat" both land
in the same release; G unblocks the rustdoc win even if D slips.

**Confidence:** high on G alone. Complements, does not replace,
§2's merge-review content.

#### §2 — Option H: proc-macro reading registry at expansion

Doc 1 §2 Option D recommends `native-theme-build` as a build script.
A structurally different variant worth listing for completeness:

| # | Option | Pros | Cons |
|---|---|---|---|
| H | **Proc-macro crate that reads `property-registry.toml` at macro-expansion time** (via `include_str!` or `std::fs::read_to_string` inside the proc-macro's `lib.rs`, tracked via `proc_macro::tracked_path::path` on nightly or a file-stamp workaround on stable) | Keeps generation inline with source (no materialised `.rs` files under `target/`). Macro output inspectable via `cargo expand`. Single file change per widget (edit the registry). Composes with the existing declarative macro by replacing its body, not its call sites. | `include_str!` inside a proc-macro requires careful path handling relative to `CARGO_MANIFEST_DIR`. Incremental compilation on registry edit needs `track_path` (nightly) or a stamp-file workaround. Error messages inside proc-macros are harder to pinpoint than build-script errors. Less common Rust idiom, harder for new contributors. |

**Position vs D:** D (build script) has better traceability,
inspectable generated code under `target/`, and mature tooling.
H (proc-macro) has better locality (nothing materialised). For a
schema like `property-registry.toml`, **D remains the recommended
path**; H is listed so a maintainer can choose on taste without
losing any correctness property.

**Recommendation:** **keep D**, do not switch to H. H is a
fallback if D's build-pipeline complexity blocks adoption.

**Confidence:** high on the recommendation.

#### §6 — alternative E: 4-variant category hierarchy as the bundled recommendation

The existing §6 bundled recommendation fuses 6a/6b/6c/6d into a
9-variant flat `Error` enum. A structurally different fusion:

**Option E (meta-alternative to the bundled form):**

```rust
#[non_exhaustive]
pub enum Error {
    /// OS detection, reader, watcher, or compile-time feature failures.
    Platform(PlatformError),
    /// TOML / serde parse or preset-lookup errors.
    Parse(ParseError),
    /// Theme resolution left fields unfilled or out of range.
    Resolution(ResolutionError),
    /// File I/O (preserves `std::io::Error` source).
    Io(std::io::Error),
}

#[non_exhaustive]
pub enum PlatformError {
    FeatureDisabled { name: &'static str, needed_for: &'static str },
    PlatformUnsupported { platform: Platform },
    WatchUnavailable { reason: &'static str },
    ReaderFailed {
        reader: &'static str,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

#[non_exhaustive]
pub enum ParseError {
    Toml(toml::de::Error),
    UnknownPreset { name: String, known: &'static [&'static str] },
}

#[non_exhaustive]
pub enum ResolutionError {
    Incomplete { missing: Vec<FieldPath> },
    Invalid { errors: Vec<RangeViolation> },
}
```

Side-by-side comparison:

| | Flat (original bundled) | Hierarchy (E) |
|---|---|---|
| Top-level variant count | 9 | 4 |
| Match on category | 9 arms (or `_` catch-all) | 4 arms |
| Match on specific variant | 1 depth | 2 depth |
| Adding a new platform sub-variant | Touches top-level `Error` | Touches `PlatformError` only |
| Rustdoc top-level page | 9 variants | 4 variants + 4 linked sub-enums |
| `From` impls | `From<toml::de::Error> for Error` direct | `From<toml::de::Error> for ParseError` + `From<ParseError> for Error` |
| Std precedent | — | `io::Error` / `io::ErrorKind` pattern |

**Rationale for preferring E under "perfect API":**

Error handling in library consumers splits into two distinct modes:

1. **Category-level matching** — "is this transient?", "is this a
   user error?", "should I retry?". This is the **common path** and
   it wants category-level distinctions, not variant enumeration.
2. **Variant-level matching** — "was it specifically a missing
   `accent_color`?". This is the **rare path** and tolerates more
   verbosity.

A 9-variant flat enum makes mode (1) require 9 match arms or a
`_` catch-all; mode (2) is one match depth. A 4-variant hierarchy
makes mode (1) a 4-arm match (scales) and mode (2) a 2-depth
match (one extra `(...)` wrapper, common Rust).

Under "perfect API" the hierarchy is strictly better for evolution:
adding a new `PlatformError::XdgFailure` variant is local to
`PlatformError`'s `#[non_exhaustive]` and does not touch the outer
`Error`. The flat form grows the outer enum on every addition.

**Recommendation:** **switch from the flat shape to E (4-variant
hierarchy)**.

**Downstream impact:**
- Doc 2 A3's `missing_fields` dual-category fix folds naturally
  into `Error::Resolution(ResolutionError::Incomplete { missing })`
  and `Error::Resolution(ResolutionError::Invalid { errors })`.
- Doc 2 I1's stale `presets.rs:85-88` comment is resolved by the
  same Clone drop plus the new cache shape
  `Result<ThemeSpec, Error>`.
- Doc 2 I2's hint rewrite is independent and bundles with §15a.

**Confidence:** medium-high. The flat form is acceptable; the
hierarchy is strictly better for evolution. Under "perfect API"
E is the recommendation.

#### §8 — refinement: `IconSize` enum instead of raw `u32`

The icon builder in §8 Option C sketches `IconRequest::size(u32)`
taking a raw pixel value. A refinement:

```rust
#[non_exhaustive]
pub enum IconSize {
    Small,       // 16 on Freedesktop; .small on SF Symbols; Small on Fluent
    Medium,      // 24 on Freedesktop; .medium on SF Symbols; Standard on Fluent
    Large,       // 32 on Freedesktop; .large on SF Symbols; Large on Fluent
    ExtraLarge,  // 48 on Freedesktop; .xlarge on SF Symbols
    Px(u32),     // explicit escape hatch for exact sizing
}
```

**Pros:**
- Matches how native icon systems think — SF Symbols and Fluent
  Icons both use size tokens, not raw pixels.
- Freedesktop has standard sizes (16/22/24/32/48/64/128) that map
  to tokens cleanly; raw `u32` forces users to memorise these.
- `IconRequest::new(role).size(IconSize::Medium).load()` is more
  meaningful than `.size(24)` at the call site.
- `Px(u32)` preserves the exact-sizing escape hatch.

**Cons:**
- Two layers: users always ask "token or `Px`?" once.
- Hardcoded token-to-pixel mapping may not match every app's
  preferred sizing (but the escape hatch covers that).
- More types in rustdoc.

**Position:** This is a refinement *inside* §8 Option C (builder),
not a new top-level option. The default in the short-form
`load_icon(role, set)` becomes `IconSize::Medium`. The long-form
builder exposes `IconRequest::size(IconSize)`.

**Recommendation:** **adopt the enum**. The token-vs-raw question
is one of the small differences between "works" and "feels native."
Commit to tokens with a `Px` escape hatch.

**Confidence:** medium. The exact token count (3/4/5) and variant
names are bikeshed; any reasonable set that maps to concrete
platform sizes works.

#### §12 — Option G: one module per widget

| # | Option | Pros | Cons |
|---|---|---|---|
| G | **Partition widget types into one module per widget**: `theme::widgets::button::{ButtonTheme, ResolvedButtonTheme, FIELD_NAMES}` — 25 submodules | Each rustdoc page has 2 types instead of 50. Matches `serde_json`, `syn`, `tokio` organisation. Finding a widget is always `theme::widgets::{name}::`. | Every import gains one path segment. Users with multiple widget types write `use theme::widgets::{button, input, ...};` enumerations. Larger file count (one per widget module). |

**Position:** G is technically more granular than §12 Option C's
flat `theme::widgets::*`, but the marginal benefit is small. Users
typically reference widget types by name (`ButtonTheme`), and
`theme::widgets::ButtonTheme` already pinpoints them. Adding a
`button::` segment improves discoverability only marginally while
adding path verbosity to every import.

**Recommendation:** **do not adopt G.** Keep §12 Option C's flat
`theme::widgets::*` layout. G is overkill even under "perfect API".

**Confidence:** high on the rejection.

#### §13 — Option G: remove caching entirely

| # | Option | Pros | Cons |
|---|---|---|---|
| G | **Remove crate-level caching entirely.** Export only uncached `detect_is_dark()`, `detect_reduced_motion()`, `detect_icon_theme()`. Caller decides caching policy. | No global state in `native-theme`. Testability is trivial (every call pure). `invalidate_caches()` disappears. The 2 s `gsettings` timeout (doc 2 I3) becomes the caller's problem, which is honest: the caller knows the UI-thread situation. Matches the "data library" half of doc 2 §G's split. | Per-frame `system_is_dark()` in `showcase-gpui.rs:702` becomes 2 s worst-case on cold Linux. The showcase would need its own cache or a background thread. Every current caller must migrate to caller-owned caching. |

**Position:** G is the "perfect API" answer under the assumption
that `native-theme` is a data library, not a system-detection
library. Doc 2 §G flags this direction as v1.0 scope (crate
split into `-model` + `-system`).

**Recommendation:** **defer G to v1.0 crate split.** Keep
§13 Option C (`DetectionContext` + `ArcSwapOption`) for v0.5.7.
Removing caching in v0.5.7 without the accompanying crate split
would leave every connector to rebuild per-frame caching on its
own, which is a larger migration than v0.5.7 should absorb.

Once `-system` exists, G becomes the natural answer for its
reduced scope.

**Confidence:** high on the deferral. Direction is correct; timing
is wrong.

### 30.3 New issue M1: macOS reader hardcodes wrong `DialogButtonOrder`

Both documents flagged reader correctness as explicitly out of scope
(post-scripts in doc 1 and doc 2). One such issue is tightly coupled
to doc 2 D5's architectural recommendation and therefore worth
surfacing even under that scope exclusion — because the fix that
D5 proposes on architectural grounds *also happens to fix a bug
this pass verified against three independent sources*.

**Files:**
- `native-theme/src/macos.rs:504-505` — reader sets `PrimaryLeft`
- `native-theme/src/presets/macos-sonoma.toml:254,586` — preset says `"primary_right"`
- `native-theme/src/presets/macos-sonoma-live.toml:126,285` — live preset says `"primary_right"`
- `docs/platform-facts.md:1481` — column header lists macOS under "primary rightmost"
- `docs/platform-facts.md:1802` — "Dialog button order: macOS primary rightmost ✅ Apple HIG: 'A button that initiates an action is furthest to the right, Cancel to its left.'"
- `native-theme/src/resolve/inheritance.rs:98-109` — `platform_button_order()` returns `PrimaryRight` on non-Linux

#### Problem

```rust
// native-theme/src/macos.rs:504-505
// macOS uses leading affirmative (OK/Cancel) dialog button order.
v.dialog.button_order = Some(crate::DialogButtonOrder::PrimaryLeft);
```

Both the value and the comment are factually wrong:

1. **Platform-facts contradicts it.** `platform-facts.md:1481`
   lists macOS under "primary rightmost" with a ✅ confirmation
   at line 1802 citing Apple HIG directly: *"A button that
   initiates an action is furthest to the right, Cancel to its
   left."*
2. **The macOS presets contradict it.** Both `macos-sonoma.toml`
   and `macos-sonoma-live.toml` carry `button_order = "primary_right"` —
   the correct value per platform-facts.
3. **The in-tree resolver contradicts it.** `platform_button_order()`
   at `inheritance.rs:98-109` returns `PrimaryRight` on all non-Linux
   platforms — the correct value for macOS.
4. **The code comment misattributes the convention.** "Leading
   affirmative (OK first)" is the KDE convention per platform-facts,
   not macOS. macOS uses trailing affirmative.

The reader hardcode **overrides the correct preset value** during
merging. A user loading a `SystemTheme` on macOS via `from_system()`
receives `PrimaryLeft` (wrong) regardless of what the preset says.

The existing test at `macos.rs:805-815` only verifies
`button_order.is_none()` on a freshly-built `ThemeVariant` *before*
the reader runs — it does not check the concrete value after, so
the bug is not caught.

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Verified bug; reader disagrees with preset, platform-facts, and resolver |
| B | **Correct the hardcoded value to `PrimaryRight` and fix the code comment** | Minimum diff; preserves the reader-level hardcode | Leaves the architectural duplication doc 2 D5 wants to eliminate |
| C | **Delete the hardcode entirely**, extending doc 2 D5's principle to macOS. Preset value propagates via merge; if a custom TOML omits the field, resolver's `platform_button_order()` fills `PrimaryRight`. | Architecturally clean (single source of truth: preset + resolver). Symmetrically applies D5. Fixes the bug as a side effect. | A user loading a custom macOS TOML that omits `button_order` now depends on the resolver fallback (previously guaranteed by reader hardcode). |
| D | **Keep the hardcode, fix only the comment** | No behaviour change | Value is still wrong; user-visible bug persists |

#### Recommended: **C**

Delete `native-theme/src/macos.rs:504-505`. The
`macos-sonoma.toml` and `macos-sonoma-live.toml` presets already
carry `button_order = "primary_right"`, which propagates through
the pipeline merge. The reader hardcode is both wrong in value
**and** architecturally redundant.

**Ship M1 bundled with doc 2 D5.** Both become a single commit:
"delete reader-side `button_order` hardcodes, let presets +
resolver be authoritative on all platforms."

#### Rationale

Option **A** leaves a verified bug. Option **B** fixes the value
but leaves the architectural duplication doc 2 D5 aims to eliminate;
if D5 deletes KDE's hardcode while macOS keeps its own, readers
become inconsistent in the wrong direction. Option **D** fixes
the comment but not the behaviour; users still get wrong button
order on macOS.

Option **C** is the minimum change that restores correctness AND
aligns with D5's principle. The only risk (users whose custom
TOML omits `button_order`) is bounded by the resolver's fallback,
which already returns `PrimaryRight` on macOS.

**Confidence:** high. The bug is verified against three
independent sources: platform-facts with Apple HIG citation, the
bundled macOS preset TOMLs, and the resolver's platform default.
The fix is mechanical (delete two lines + one comment).

#### Out-of-scope note on Windows

`native-theme/src/windows.rs:517` hardcodes `PrimaryRight`, which
agrees with both `windows-11.toml` and `windows-11-live.toml`
presets. However, `platform-facts.md:1481` lists Windows under
"primary leftmost" (with a citation to the older MS Command
Buttons guideline at line 1803). Modern WinUI 3 content dialogs
use primary-right; the presets track modern practice while
platform-facts tracks the older guideline.

This is a **platform-facts vs. modern-practice tension**, not a
reader bug: reader, preset, and resolver default all agree on
`PrimaryRight` for Windows; only `platform-facts.md` disagrees,
and its citation is the older guideline. Resolving the tension
requires a platform-facts refresh — out of scope for an API
critique.

No Windows-reader action is recommended here. Flagged only so
a future platform-facts update has a bookmark.

### 30.4 Strengthened recommendations under "no backward compat"

Three recommendations earlier passes marked as "medium confidence,
defer if time is tight" should be committed to rather than
deferred under the "perfect API" directive:

#### §2 + §14 + doc 2 B1 + B2 + B7: registry-driven codegen — promote to P0/P1 for v0.5.7

Earlier passes rated this as **P3** ("very large effort, medium
confidence"). Under "perfect API":

- **Blast radius is measured.** Doc 2 §I5 counts ~215
  hand-maintained string literals in `lint_toml` alone. Add
  ~450 lines of per-widget `check_ranges`, ~280 lines of
  `require()` defaults extraction, ~100 lines of inheritance
  rules duplicated between `inheritance-rules.toml` and
  `inheritance.rs`, and the 108-line `define_widget_pair!`
  declarative macro. **~1100 lines of hand-maintained code**
  at drift risk on every widget addition.
- **Drift compounds.** Each new widget costs 5+ file edits today.
  Each future maintainer must learn the macro DSL cold.
- **The registry is ~80 % designed.** `property-registry.toml`
  already captures structure definitions, field types,
  inheritance markers, and serde rename mapping. Only range-check
  metadata per field and `border_kind` per widget are missing.
- **v0.5.7 is the window.** The no-backward-compat gate does
  not reopen for free.

**Strengthened recommendation:** promote from **P3 to P1 for
v0.5.7**, accept schedule risk.

**Minimum viable D under schedule pressure:** ship the registry
extension + codegen for `check_ranges`, `FIELD_NAMES`, and
`lint_toml` tables only. Leave widget struct generation for
v0.5.8. This eliminates the ~215 `lint_toml` literals and
~450 lines of `check_ranges` — the biggest drift hazards —
without the larger struct-generation lift.

**Fallback if even minimum-viable D is too large:** ship doc 1
§14 Option F (`inventory` crate, link-time registry collection)
for v0.5.7 as a bridge. F eliminates the `VARIANT_KEYS` +
`widget_fields` match-arm drift immediately via ~20 lines of
changes and one dependency.

**Combined v0.5.7 plan under "perfect API":**
- §2 Option G (macro doc elision) — ship as small-effort P0 for
  immediate rustdoc quality.
- Minimum-viable D — ship as P1 structural codegen win.
- §14 F (`inventory`) as fallback if minimum-viable D slips.

#### §3 + doc 2 B5: pair them as a single ship-unit, mandatory

Both earlier passes noted this coordination in merge-review
addenda but neither made it mandatory in the priority table.
Under "perfect API" this is **mandatory sequencing**: split
landing forces a double-edit of `OverlaySource` (first with a
standalone `font_dpi: f32`, then with `ctx: ResolutionContext`).
The combined single-PR diff is smaller than two separate passes.

**Strengthened recommendation:** merge §3 + B5 into a single
"`OverlaySource` + `ResolutionContext` refactor" ship-unit. Tag
as **P0** for v0.5.7. Do not allow split landing across releases.

#### §6: commit to the 4-variant hierarchy (Option E above)

Argued in §30.2's §6 subsection. Under "perfect API" the hierarchy
is strictly better for evolution and category-level matching at
the cost of one extra match depth for variant-specific code (the
less common path).

**Strengthened recommendation:** switch from the flat 9-variant
form to the 4-variant hierarchy. Doc 2 A3 folds into
`Error::Resolution(ResolutionError::Incomplete | Invalid)`.

### 30.5 Updated priority rebalance

Given §30.4's strengthenings, the v0.5.7 cohort updates as:

**P0 additions / promotions:**
- §3 + doc 2 B5 combined `OverlaySource + ResolutionContext`
  refactor (was P1 in earlier passes)
- §6 restructured using the 4-variant hierarchy (shape change,
  effort unchanged)
- M1 delete macOS reader `button_order` hardcode (bundle with
  doc 2 D5)
- §2 Option G macro doc elision (small-effort new item)

**P1 additions / promotions:**
- §2 + §14 + doc 2 B1/B2/B7 minimum-viable registry-driven
  codegen (was P3):
  - `check_ranges`, `FIELD_NAMES`, `lint_toml` tables from registry
  - Widget struct generation deferred to v0.5.8

The P0 cohort from earlier passes remains otherwise intact.
The P1/P2/P3 items not strengthened in §30.4 remain at their
earlier-pass priority.

### 30.6 Confidence statement

**High confidence** on:
- Every file:line claim in §30.2 and §30.3 verified against the
  current tree. M1 verified against three independent sources
  (platform-facts with Apple HIG, bundled macOS presets,
  resolver default).
- §2 Option G (macro doc elision) is a safe ~1-day change.
- §6 Option E (4-variant hierarchy) is strictly better for
  evolution, even if the flat form is acceptable.
- §3 + B5 pairing is mandatory under "perfect API".
- §30.4 codegen promotion is correct given measured blast radius.

**Medium confidence** on:
- §2 + B1 minimum-viable-D scope fits v0.5.7 schedule (depends
  on build-pipeline work that may surface unknown complexity).
- §8 `IconSize` enum variant count and names (3 / 4 / 5 are
  all reasonable).

**Deferred / explicitly out of scope:**
- §13 Option G (remove caching entirely) — correct for v1.0
  crate split, premature for v0.5.7.
- §12 Option G (one module per widget) — overkill even under
  "perfect API".
- Windows reader / platform-facts `button_order` tension —
  flagged in §30.3 as a future platform-facts refresh, not an
  API change.

### 30.7 What this pass did NOT change

Deliberately preserved from earlier passes:

- Every existing pros/cons entry in the original option tables
  and §29 merge-review tables.
- Every existing recommendation that was not explicitly
  strengthened or given a new alternative in §30.2 / §30.4.
- The §28 open questions list (no items removed; §30.4 adds one
  implicit open question about minimum-viable-D scope).
- Doc 2's A1 STATUS block (A1 remains fully shipped and out of
  v0.5.7 scope).

If §30 does not reference a prior recommendation, it is unchanged
from the earlier-pass analysis.

---

## 31. Fourth-pass review: merged critical refinements

This section records a fourth ultrathink pass performed under the
explicit directive "backward compatibility does not matter; I want
the perfect API." The pass re-verified every prior claim against
the current tree (all still accurate), added new options where a
cleaner shape exists, surfaced one missed issue (feature-flag
matrix), and strengthened four prior recommendations that were
hedged against migration cost.

New options are appended as letters following the existing ones
(so §1 gains **G**, §2 gains **K**, §6 gains **F**, §13 gains
**H**) without disturbing earlier letter assignments. Where this
pass supersedes a prior recommendation, the rationale is stated
explicitly; earlier option tables are preserved for auditability.

### 31.1 Verification pass (full re-check)

Every file:line reference in §1–§30 was personally re-verified
against the current tree. All claims pass within ≤5-line tolerance:

- **§1** six type locations — VERIFIED
- **§2** `define_widget_pair!` at `widgets/mod.rs:48-156` (109
  lines), 26 invocations found (25 widgets + `LayoutTheme`) —
  VERIFIED
- **§3** `SystemTheme` pre-resolve fields at `lib.rs:215-232` —
  VERIFIED
- **§4** `active()` / `pick()` at `lib.rs:239-253` — VERIFIED
- **§5** async dispatch at `lib.rs:372-386` — VERIFIED, non-Linux
  path at line 385 is `pipeline::from_system_inner()` with no
  `.await`, exactly as claimed
- **§6** `error.rs` has 6 variants (Unsupported, Unavailable,
  Format, Platform, Io, Resolution); tests at `error.rs:239-250`
  explicitly assert `Error: Clone` — dropping Clone per §6a must
  also delete these tests
- **§7** four `resolve*` methods at documented lines — VERIFIED
- **§8** 12 loaders + 1 probe at documented lines — VERIFIED (the
  §8 merge-review corrected "12" to "13" by including the probe;
  excluding it gives 12)
- **§12** crate root count — **RE-CORRECTED: ~91 items**, not
  "~70-75". The §29.2 merge-review correction was too conservative.
  Hand-count at `lib.rs:122-206`: ~51 model re-exports + ~12
  cfg-gated platform helpers + ~6 icon free-functions + ~5 detect
  functions + 2 color + 2 error + 3 watch + 2 detect types + 4
  pipeline/icons helpers + 1 `Result` alias + `SystemTheme` struct
  defined at :215 ≈ **89-91 items**. The argument gets *stronger*,
  not weaker — 91 is well past any reasonable flat-root threshold.
- **§13** all three cache locations — VERIFIED
- **§14** `VARIANT_KEYS` has exactly 29 entries at `mod.rs:563-593`
  — VERIFIED
- **§15** all sub-issues a–f — VERIFIED
- **§19** `LinuxDesktop` has no `#[non_exhaustive]` — VERIFIED
- **§20** `icon_set` / `icon_theme` on `ThemeVariant` at
  `mod.rs:174,182` — VERIFIED
- **§25** `to_px(dpi)` asymmetry — VERIFIED, and `font.rs:42-43`
  **explicitly documents the asymmetry in the method rustdoc**
  (`"- Px(v) -> v (dpi ignored)"`). See §31.6 for the severity
  nuance this introduces.
- **§30.3 M1** macOS `button_order` hardcode — VERIFIED against
  three independent sources (`platform-facts.md:1481` dialog table
  row `"primary rightmost"` for macOS; `macos-sonoma.toml:254,586`
  + `macos-sonoma-live.toml:126,285` all reading `button_order =
  "primary_right"`; `resolve/inheritance.rs:108` returning
  `PrimaryRight` on non-Linux). The `macos.rs:504-505` hardcode is
  **the only source disagreeing**. Bug is verified, reproducible,
  and ships on every `SystemTheme::from_system()` call on macOS
  today.

**Additional verifications performed this pass:**

- `Cargo.lock` shows `inventory` v0.3.24 and `strum`/`strum_macros`
  already in the dep graph (pulled in by connectors). For
  `native-theme` as a standalone crate, both are still new direct
  deps; for the connector ecosystem they are zero marginal cost.
- `cargo tree -p native-theme` shows only `serde`, `serde_with`,
  `toml`, and platform-specific crates in the direct dep tree.
  `syn` / `quote` / `proc-macro2` are already transitive via
  `serde_derive` — **a new proc-macro crate for §2 K reuses them
  at zero new-dep cost**.

### 31.2 New options added to existing problems

#### §1 — Option G: rename `ThemeVariant → ThemeMode` rather than `ThemeLayer`

§1's recommended rename (Option B) names `ThemeVariant → ThemeLayer`.
"Layer" communicates *stacking / composition* semantics, which is
misleading here: a `ThemeVariant` is **one OS mode's data** (light
OR dark). Stacking is what `with_overlay` does across variants, not
what the variant itself represents. The name should communicate
"this is one mode's concrete data", not "this is a stackable piece".

| # | Option | Pros | Cons |
|---|---|---|---|
| G | **`ThemeVariant → ThemeMode`** (drop-in replacement for B's `ThemeLayer`). All other renames in §1 Option B remain as written. | "Mode" matches established OS vocabulary: macOS "Appearance Mode" / "System Appearance Mode", Windows "Color Mode", GNOME `org.freedesktop.appearance.color-scheme`. Every reader of the code already knows the term from the OS they develop for. Reads naturally at the call site: `theme.mode_data(Mode::Dark)`, `theme.dark_mode`, `pick_mode(is_dark)`. Parallels `IconSet` / `IconData` (concrete-shape-per-category naming). Zero migration cost relative to B — both are pure renames of the same type. Communicates the invariant ("one mode's data") directly in the type name. | Creates a potential vocabulary conflict with a hypothetical future `ColorMode { Light, Dark, System }` enum (§4 Option E). Any such enum would need a different name (`ModeSelection`, `ModePreference`). "Mode" is slightly less unique than "Layer" in a codebase search — greping for "mode" has more false positives than greping for "layer". |

**Position:** G is a strict refinement of B, not a replacement. B's
other four renames (`ThemeSpec → Theme`, `ResolvedThemeVariant →
ResolvedTheme`, `ResolvedThemeDefaults → ResolvedDefaults`,
`SystemTheme → DetectedTheme`) remain as drafted.

**Rationale under "perfect API":** a type's name should communicate
its meaning. `ThemeLayer` implies composition / stacking, which is
wrong. `ThemeMode` implies "state the OS is in", which is exactly
right. The vocabulary-conflict cost (a future `ModeSelection` enum
instead of `ColorMode`) is smaller than the semantic cost of a
misleading name. Under "perfect API," accuracy wins ties.

**Recommendation:** **adopt G**, replacing B's `ThemeLayer` with
`ThemeMode`. All other §1 renames remain unchanged.

**Confidence:** medium. Both names are defensible; `ThemeMode` is
marginally better because it communicates the "one OS state's data"
meaning directly and matches OS-level terminology.

#### §2 — Option K: narrow `#[derive(ThemeLayer)]` proc-macro keeping Rust as source of truth

§2 Option D recommends registry-driven codegen via
`native-theme-build` reading `property-registry.toml` at build
time. §30.2 adds Option G (macro doc elision) as a P0 partial fix
and Option H (proc-macro reading registry at expansion). A
structurally different path under "perfect API":

| # | Option | Pros | Cons |
|---|---|---|---|
| K | **Narrow `#[derive(ThemeLayer)]` proc-macro** in a new `native-theme-derive` crate. Reads the Rust struct definition directly. Emits the paired resolved struct, `FIELD_NAMES`, `impl_merge!` body, `check_ranges` impl, and (via integrated `inventory::submit!`) the §14 widget registry. Field metadata (ranges, required, inheritance-from, border_kind) lives in struct-level attributes — no external TOML schema. | **Single source of truth is the Rust struct itself.** No external schema to design. Zero new file formats. IDE support works out-of-the-box (rust-analyzer understands struct attributes). Review diffs stay in Rust (`git blame` tracks field changes at struct-definition time). Subsumes `define_widget_pair!`, duplicated docs, manual `check_ranges`, manual `FIELD_NAMES`, and (via `inventory::submit!`) solves **§14's drift hazard as a side-effect**. The proc-macro reuses `syn` / `quote` / `proc-macro2` already transitive via `serde_derive` — **~0 new direct deps for `native-theme`**. Proc-macro crate is ~800-1500 LoC, substantially less work than the full registry pipeline D requires. Preserves compile-time non-null guarantees that §2 B/C/E sacrifice. Inventory is already in the workspace's `Cargo.lock` (pulled by connector deps), so integration cost is minimal. | Field metadata lives in attributes, which are less expressive than a full TOML schema for cross-widget rules (cross-field inheritance, conditional validation). Non-Rust stakeholders (designers, platform auditors) cannot review a registry without reading Rust source. If multiple widgets need the same range-check logic, it must be expressed via attribute macros rather than schema-level rules (minor duplication). `inventory` uses linker-section magic that does not work on WebAssembly without special handling — but `native-theme` targets desktop platforms, so WASM is not a stated requirement. Does not cover `inheritance-rules.toml` as cleanly as D — attribute-based inheritance is per-field, not pattern-based. |

**Example attribute surface:**

```rust
#[derive(ThemeLayer)]
#[theme_layer(resolved = "ResolvedButtonTheme", border_kind = "full")]
pub struct ButtonTheme {
    #[theme(required)]
    pub background_color: Option<Rgba>,

    #[theme(required, range = "0.0..=1.0")]
    pub disabled_opacity: Option<f32>,

    #[theme(inherit_from = "defaults.background_color")]
    pub primary_background: Option<Rgba>,

    /// Minimum button width in logical pixels.
    #[theme(required, check = "non_negative")]
    pub min_width: Option<f32>,
}
```

**Generated code (sketched):**

```rust
// Paired resolved struct with non-Option fields:
pub struct ResolvedButtonTheme {
    pub background_color: Rgba,
    pub disabled_opacity: f32,
    pub primary_background: Rgba,
    pub min_width: f32,
}

impl ButtonTheme {
    pub const FIELD_NAMES: &[&str] = &[
        "background_color", "disabled_opacity",
        "primary_background", "min_width",
    ];
}

// The impl_merge! body is emitted inline (not a separate call):
impl ButtonTheme {
    pub fn merge(&mut self, overlay: &Self) { /* ... */ }
    pub fn is_empty(&self) -> bool { /* ... */ }
}

impl ResolvedButtonTheme {
    pub(crate) fn check_ranges(&self, prefix: &str, errors: &mut Vec<String>) {
        check_range_f32(self.disabled_opacity, 0.0, 1.0,
                        prefix, "disabled_opacity", errors);
        check_non_negative(self.min_width, prefix, "min_width", errors);
    }
}

// Register in the inventory-backed widget registry for §14:
inventory::submit! {
    crate::model::widgets::WidgetEntry {
        name: "button",
        fields: ButtonTheme::FIELD_NAMES,
    }
}
```

**Position vs §2 Option D (registry-driven build script):**

| | D (external TOML registry) | K (Rust struct attributes) |
|---|---|---|
| Single source of truth | TOML file | Rust struct |
| New artifact types | `.toml` schema + generated `.rs` under `target/` | Only a new proc-macro crate |
| Build pipeline | `build.rs` reads registry, writes Rust source, `include!` from `src/` | None beyond `cargo build` |
| Field metadata expressiveness | High (full DSL) | Medium (attributes only) |
| Non-Rust audit | Yes (TOML is reviewable) | No (requires reading `.rs`) |
| IDE completion on fields | Indirect (needs schema→Rust roundtrip) | Native (rust-analyzer sees struct fields directly) |
| Upfront design cost | High (schema design is the bottleneck) | Medium (attribute syntax is well-trodden) |
| Upfront LoC | ~1000 (schema + codegen + build script + migration) | ~1500 (single proc-macro crate) |
| Delivers §2 | Yes | Yes |
| Delivers §14 | Via separate step or F (inventory) | Yes, in the same macro via integrated `inventory::submit!` |
| Delivers doc 2 B1 (check_ranges boilerplate) | Yes | Yes |
| Delivers doc 2 B2 (inheritance rules) | Yes (via TOML schema) | Partial (attribute-based inheritance handles simple cases; pattern-based rules fall back to `inheritance.rs`) |
| Delivers doc 2 B6/B7 (BorderSpec split, border validation) | Yes | Yes via `border_kind` attribute |
| v0.5.7 schedule fit | Tight | Achievable |
| Risk that scope blows up | High (registry schema evolves over design) | Low (proc-macro is mechanical once attribute syntax freezes) |

**Position vs §30.2 Option H (proc-macro reading TOML at expansion):**

H is D's TOML storage with a proc-macro frontend — it keeps the
dual-source structure (TOML as data, Rust struct derived from
TOML) but moves the generation into proc-macro rather than a
build script. K eliminates the TOML file entirely and uses Rust
struct attributes as both data and schema.

K wins on simplicity; H wins on inheritance-rule expressiveness
if that matters.

**Position vs §14 Option F (inventory as short-term bridge):**

F adds `inventory::submit!` inside the existing declarative
`define_widget_pair!` macro as a bridge that solves §14 without
touching §2. K integrates the same `inventory::submit!` into the
unified proc-macro derive, solving §2 and §14 together with no
separate bridge step.

K subsumes F entirely.

**Position vs §30.2 Option G (macro doc elision):**

§30.2 G is an 80/20 small-effort fix (edit `define_widget_pair!`
to emit `/// See [\`XxxTheme::field\`]` on resolved fields instead
of duplicating docs). K replaces `define_widget_pair!` outright,
so the doc elision becomes moot — K naturally deduplicates docs
because it generates resolved-field docs from the same source
attribute as the Option-field docs.

If K lands, §30.2 G is unnecessary. If K slips, §30.2 G is still
worth shipping standalone.

**Recommendation:** **ship K for v0.5.7** as the primary codegen
path. K delivers §2 + §14 + doc 2 B1 in one coherent proc-macro
crate. §14 F (inventory alone) becomes a fallback only if K slips
past v0.5.7. §2 D / §30.2 H remain v1.0 fallbacks if attribute
expressiveness proves insufficient for inheritance-rule edge
cases.

**Confidence:** high on the direction (Rust-struct-as-source is
strictly better for a Rust-only toolchain). Medium on scope (the
inheritance-rule expressiveness question is the main unknown; a
prototype implementation against 2-3 widgets should surface this
cheaply in a day or two before full commitment).

**Flag for §28:** whether `inheritance-rules.toml`'s pattern-based
rules (e.g. "every widget's `border.color` inherits from
`defaults.border.color`") can be expressed via attribute-level
`inherit_from` declarations without losing expressiveness. If not,
K must either keep reading the TOML file (degenerating toward H)
or accept partial coverage with hand-written inheritance logic.

#### §6 — Option F: flat variants + `kind()` method matching std `io::Error` precedent

§30.2 introduces Option E: 4-variant hierarchy (`Error::Platform |
Parse | Resolution | Io`) with nested sub-enums. Under "perfect
API" a third shape is genuinely superior because it matches an
existing std precedent and avoids the match-depth tax:

| # | Option | Pros | Cons |
|---|---|---|---|
| F | **Flat 9-variant `Error` enum + `impl Error { pub fn kind(&self) -> ErrorKind }` method** exposing a lightweight `ErrorKind` categorizing enum (`Platform`, `Parse`, `Resolution`, `Io`). Callers that want category-level matching do `err.kind() == ErrorKind::Platform`. Callers that want variant-level matching do `match err { Error::FeatureDisabled { .. } => ..., _ => ... }` directly — no extra match depth. | **Matches `std::io::Error::kind()` precedent exactly.** Millions of Rust users already know this pattern. Zero match-depth tax on variant-specific handling (the common case). Category-level matching is one method call, no longer than hierarchy's `matches!(err, Error::Platform(_))`. Rustdoc top-level page lists 9 entries (manageable — `std::io::ErrorKind` has 40+). Compile-time enforcement of the `ErrorKind` mapping is free via an exhaustive `match` inside the crate-private `kind()` impl. Zero-depth variant matching is strictly the most common pattern in real library error handling per the §6 usage analysis in §30.2. | Two APIs (variant match + `kind()` method). Users may not discover `kind()` from rustdoc on first read without the doc example pointing at it. `ErrorKind` is a separate public type. Must document the precedent explicitly so users understand why both exist. |

**Compile-time enforcement of the `ErrorKind` mapping:**

```rust
impl Error {
    pub fn kind(&self) -> ErrorKind {
        match self {
            Error::FeatureDisabled { .. }      => ErrorKind::Platform,
            Error::PlatformUnsupported { .. }  => ErrorKind::Platform,
            Error::WatchUnavailable { .. }     => ErrorKind::Platform,
            Error::ReaderFailed { .. }         => ErrorKind::Platform,
            Error::UnknownPreset { .. }        => ErrorKind::Parse,
            Error::Toml(_)                     => ErrorKind::Parse,
            Error::ResolutionIncomplete { .. } => ErrorKind::Resolution,
            Error::ResolutionInvalid { .. }    => ErrorKind::Resolution,
            Error::Io(_)                       => ErrorKind::Io,
        }
    }
}
```

This is an exhaustive `match` inside the defining crate, which is
**not subject to `#[non_exhaustive]`'s wildcard requirement**. A new
variant in `Error` that lacks a `kind()` branch is a compile error.
Zero-cost drift prevention.

**Access-pattern comparison against §30.2 Option E (hierarchy):**

| Pattern | §30.2 E (hierarchy) | §31 F (flat + kind) |
|---|---|---|
| Variant-specific handling (common) | `match err { Error::Parse(ParseError::UnknownPreset { name, known }) => ..., _ => ... }` — **2 depth** | `match err { Error::UnknownPreset { name, known } => ..., _ => ... }` — **1 depth** |
| Category-level policy (rare) | `if matches!(err, Error::Platform(_)) { retry(); }` | `if err.kind() == ErrorKind::Platform { retry(); }` |
| `?` propagation | identical | identical |
| Top-level error display | `match err { Error::Platform(p) => write!(f, "{p}"), ... }` — 4 arms + nested | `match err { ... }` — 9 arms OR `match err.kind() { ... }` — 4 arms |
| Adding a new variant | Edit sub-enum only | Edit top-level + `kind()` match |
| std precedent | None | `std::io::Error` + `ErrorKind` |

**F is tied-or-better with E on every access pattern**, with the
one cost being one extra public type (`ErrorKind`) and one extra
method body (`kind()`). Both costs are trivial; the sync obligation
is compile-time-checked.

**Position vs §30.2 E:**

E's argument is "category matching is cleaner". F delivers category
matching via method call with **equivalent** cleanliness and
**strictly better** variant matching. The match-depth tax E imposes
on variant-specific handling (the common case per the §30.2 usage
analysis) is not paid by F.

**Recommendation under "perfect API":** **switch to F.** The std
precedent (`io::Error::kind()`) is the strongest available argument
— matching an established Rust idiom is itself a perfect-API virtue
because it reduces learning cost for every user.

Doc 2 A3 (missing_fields dual-category) folds into F as two flat
top-level variants: `Error::ResolutionIncomplete { missing }` and
`Error::ResolutionInvalid { errors }`, both mapping to
`ErrorKind::Resolution` via `kind()`.

**Confidence:** medium-high. Both E and F are defensible; F has
the stronger std precedent and cleaner common-case matching.
Under "perfect API" the std precedent is the tiebreaker.

#### §13 — Option H: plain `RwLock<Option<T>>` is sufficient; skip `arc-swap`

§30.2 refines §13 with Option F: use `arc_swap::ArcSwapOption<T>`
for lock-free reads, citing per-frame caller latency concerns.
Under rigorous latency analysis the dependency is not justified:

| # | Option | Pros | Cons |
|---|---|---|---|
| H | **`RwLock<Option<T>>`** wrapped in `DetectionContext` as §13 Option C drafts. **Do not** add `arc-swap`. | **Zero new deps.** Standard-library primitive. Uncontended `RwLock::read()` is ~15-25 ns on modern x86 (~0.0001% of a 16 ms frame at 60 Hz). Writer path (`invalidate_*`) blocks readers only for the duration of the write — typically <1 μs for an `Option<T>` swap. The inherent latency of **detection itself** (doc 2 §I3: `run_gsettings_with_timeout` worst case 2 seconds) is ~200,000× the lock overhead; optimising the lock primitive while keeping the 2 s subprocess call is micro-optimising the wrong thing. | Not *strictly* lock-free. Under pathological write contention, readers could serialize on the write lock (irrelevant in practice — invalidation fires ≤once per second at worst). |

**Latency math against the stated hot path** (`showcase-gpui.rs:702`
per-frame `system_is_dark()`):

| Primitive | Read latency | Write latency | Measurable per frame? |
|---|---|---|---|
| `RwLock::read()` uncontended | ~15-25 ns | n/a | No (0.0001% of 16 ms) |
| `ArcSwap::load_full()` uncontended | ~5-10 ns | n/a | No (0.00006% of 16 ms) |
| Difference | ~10 ns | n/a | **Not measurable** |

**Reality check against doc 2 §I3:**

`run_gsettings_with_timeout` can block for up to 2 seconds on cold
cache on Linux. This is the real latency dominant for theme
detection. A ~10 ns difference in the lock primitive is a rounding
error compared to 2 s of inherent subprocess I/O. The `arc-swap`
optimisation applies to a layer that is 2e8 times faster than the
underlying operation — there is no hot path that can possibly
notice the difference.

**Position vs §30.2 F (`ArcSwapOption`):**

F's argument was "hot reads, rare invalidation → lock-free is
strictly better." That reasoning applies to genuinely hot code
(e.g. shared metric counters updated per-request at high rate).
Theme detection is not hot by that standard: reads happen at
frame rate (60-120 Hz), writes happen at user-action rate (≤1 Hz),
and the backing OS call dominates the wall-clock cost.

**Recommendation:** **adopt H (plain `RwLock`).** Keep `arc-swap`
off the dep tree. Zero new dep is a perfect-API virtue: every
dep added is friction for every user of the crate.

**Confidence:** high. The latency numbers do not support the dep.

### 31.3 New issue: feature-flag matrix has four redundant variants

**File:** `native-theme/Cargo.toml:14-34`

**Problem**

The current feature definitions are:

```toml
[features]
kde              = ["dep:configparser"]
portal           = ["dep:ashpd"]
portal-tokio     = ["portal", "ashpd/tokio"]
portal-async-io  = ["portal", "ashpd/async-io"]
windows          = ["dep:windows"]
macos            = [...]
linux            = ["kde", "portal-tokio"]
linux-async-io   = ["kde", "portal-async-io"]
native           = ["linux", "macos", "windows"]
native-async-io  = ["linux-async-io", "macos", "windows"]
watch            = ["dep:notify", "dep:zbus"]
material-icons   = []
lucide-icons     = []
svg-rasterize    = ["dep:resvg"]
system-icons     = [...]
```

Fifteen features, of which **four exist only to plumb an
async-runtime choice** through a platform-group combination:
`portal-tokio`, `portal-async-io`, `linux-async-io`,
`native-async-io`. The runtime choice propagates up through two
additional feature layers.

Consequences:

1. **User confusion at `cargo add` time.** A user running `cargo
   add native-theme --features native` must understand (a) what
   `native` means, (b) why `native-async-io` exists, (c) which
   one to pick, (d) what happens if their dep graph unifies both.
2. **Fragile feature unification.** If crate A enables
   `native-theme/native` and crate B enables
   `native-theme/native-async-io`, Cargo unifies to the union,
   which forces `ashpd` to enable **both** `tokio` and `async-io`
   features. `ashpd` may or may not handle this; the behaviour is
   not documented.
3. **Interaction with §5.** If §5 Option G lands (pollster wraps
   async internally, no runtime coupling), the runtime choice
   disappears entirely — and the `-async-io` variants become dead
   features that still pollute the feature namespace until
   explicitly removed.
4. **Downstream API docs.** Every `#[cfg(feature = "...")]` gate
   in the public surface must account for the runtime-variant
   features as well as their base, multiplying the rustdoc feature
   annotations.

This issue is not called out in the existing critique. It surfaces
naturally under the §5 discussion but is not itself listed as an
item.

**Options**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo.** Keep all 15 features. | No change. Every current user path continues working. | Feature explosion tax persists. User confusion remains. Feature unification hazard is unresolved. |
| B | **Delete the four `-async-io` variants once §5 G lands.** `portal-tokio`, `portal-async-io`, `linux-async-io`, `native-async-io` are removed. `linux` and `native` survive without a runtime pin. Users who explicitly selected `-async-io` migrate to the plain variant. Feature count drops to 11. | Minimum change consistent with §5 G's runtime decoupling. Natural cleanup — when §5 eliminates the runtime choice, the features plumbing that choice become redundant. Preserves `linux` / `native` convenience aggregators. | **Hard depends on §5 G** — cannot land before the runtime coupling is eliminated. Users who currently type `--features "native-async-io"` see a build error and must migrate. |
| C | **Collapse aggregators entirely**: keep only `kde`, `portal`, `macos`, `windows`, `watch`, `material-icons`, `lucide-icons`, `svg-rasterize`, `system-icons`. Drop `linux`, `native`, `linux-async-io`, `native-async-io`, `portal-tokio`, `portal-async-io`. Users write `features = ["kde", "portal", "macos", "windows"]` explicitly. | Cleanest possible surface. Zero aggregators to confuse. Each feature is one concept with one reason. | Users who currently type `--features native` must now spell out four features. More characters at install time. Loses the "everything on this platform" shortcut — a small convenience loss. |
| D | **Keep aggregators (`linux`, `native`), drop only the `-async-io` variants.** Mid-ground between B and C. Feature count: 11. | Preserves the `--features native` convenience. Drops the runtime-coupling redundancy. | Still has two levels of feature indirection (`native → linux → kde + portal`). |
| E | **Fully dynamic**: add a single `all-platforms` feature that pulls in everything platform-relevant. Keep the base features. Drop aggregators. | Single discoverable knob (`--features all-platforms`). | Loses the ability to do "everything on Linux only" via a single knob. |
| F | **Status quo + documentation fix.** Keep the current features but add a clear feature-selection guide to the crate-level rustdoc that explains when to pick `-async-io` vs `tokio`. | Minimum code change. | Doesn't fix the underlying redundancy; users still face the choice on every `cargo add`. |

**Recommended: B, bundled with §5 G**

Drop the four `-async-io` variants when §5 Option G lands. Final
feature list: `kde`, `portal`, `linux`, `macos`, `windows`,
`native`, `watch`, `material-icons`, `lucide-icons`,
`svg-rasterize`, `system-icons` — eleven features, no
runtime-coupling redundancy.

**Rationale**

Option **A** / **F** leave the confusion in place. Option **C**
is cleaner in absolute terms but sacrifices the `--features
native` convenience for marginal gains; the aggregator is useful
in practice. Option **D** is essentially the same as B after
removal (the runtime-coupled variants are precisely the four that
B drops). Option **E** trades the runtime-variant duplication for
a different kind of coarseness (`all-platforms` flattens too
much).

B is the minimum change that eliminates the runtime-coupling
redundancy without sacrificing the aggregator convenience. It
pairs naturally with §5 G (the removal of runtime choice at the
feature level **is** the user-visible consequence of §5 G at the
code level).

**Flag for §28:** B hard-depends on §5 G. If §5 is rejected or
§5 G is replaced by a different runtime strategy, B cannot land
in its current form. In that case, fall back to **F**
(documentation-only) until a clean runtime decoupling is available.

**Confidence:** high. Once §5 G is in place, the cleanup is
mechanical.

### 31.4 Strengthened recommendations

#### §1: use `ThemeMode` (§31.2 G) as the variant rename

Supersedes §1 Option B's `ThemeLayer`. Pure rename, zero migration
cost relative to B.

#### §2: ship Option K (narrow derive proc-macro) for v0.5.7

Supersedes §2 Option D as the v0.5.7 recommendation. D is retained
as a v1.0 fallback if inheritance-rule expressiveness proves
insufficient for struct attributes. §30.2 G (macro doc elision)
and §14 F (inventory bridge) are subsumed by K.

**v0.5.7 codegen plan under "perfect API":**
- Ship K (`native-theme-derive` proc-macro crate) delivering §2 +
  §14 + doc 2 B1 + B6/B7 in one crate.
- §30.2 G becomes unnecessary (K deduplicates docs naturally).
- §14 F becomes unnecessary (K integrates `inventory::submit!`).
- Defer §2 D (external TOML registry) to v1.0 if needed.

#### §6: switch to Option F (flat + kind() method); supersedes §30.2 E

Supersedes §30.2 Option E (4-variant hierarchy). F matches
`std::io::Error::kind()` exactly, eliminates the match-depth tax,
and keeps variant-specific handling flat.

Doc 2 A3 folds into F as two top-level variants
(`Error::ResolutionIncomplete` and `Error::ResolutionInvalid`),
both mapping to `ErrorKind::Resolution` via `kind()`.

#### §13: use plain `RwLock` (§31.2 H); supersedes §30.2 F

Supersedes §30.2 Option F (ArcSwap refinement). Latency math does
not justify the dep. Zero new deps is a perfect-API virtue.

#### §31.3 feature matrix cleanup: P2, bundled with §5 G

Drop `portal-tokio`, `portal-async-io`, `linux-async-io`,
`native-async-io` when §5 G lands.

#### §25: retain the rename recommendation; downgrade severity framing

See §31.6 below. The rename stands; the "silent" framing is
weakened because the rustdoc documents the asymmetry.

### 31.5 Priority rebalance

Updates to §27:

**P0 (new or strengthened):**

| Priority | Issue | Change |
|---|---|---|
| P0 | **§31.2 G** `ThemeMode` rename | Replaces §1 Option B's `ThemeLayer` at no additional cost |
| P0 | **§6 Option F** (flat + kind() method) | Supersedes §30.2 E; same effort, strictly better shape |

**P1 (promotions):**

| Priority | Issue | Change |
|---|---|---|
| P1 | **§31.2 K** (narrow derive proc-macro) | Was §2 P3 "very high effort"; now P1 "achievable in v0.5.7 via narrow proc-macro". Delivers §2 + §14 + doc 2 B1/B6/B7 in one shipment. |

**P2 (additions):**

| Priority | Issue | Change |
|---|---|---|
| P2 | **§31.3** feature-flag matrix cleanup | Bundled with §5 G |

**Superseded (kept for reference):**

| Superseded | Replacement | Reason |
|---|---|---|
| §30.2 E (4-variant hierarchy) | §31.2 F (flat + kind()) | std precedent, zero match-depth tax |
| §30.2 F (ArcSwap) | §31.2 H (RwLock) | latency math does not justify dep |
| §2 D (build script registry) for v0.5.7 | §31.2 K (narrow derive) | Rust-as-source is simpler; attributes cover the common case |
| §30.2 G (macro doc elision) | §31.2 K (subsumes it) | K deduplicates docs by generating from one source |
| §14 F (inventory bridge alone) | §31.2 K (integrates inventory) | One macro, not two |

**Unchanged from §27 / §29 / §30:**

- M1 macOS button_order hardcode (§30.3): P0, verified, ship
- All P0 items from doc 2 §H (A2/A3/A4/B4/B6/C1/C2)
- §3 + doc 2 B5 combined ship-unit (P0, mandatory pairing)
- §4 / §12 / §16 / §17 / §19 / §20 / §22 / §26 P0 items
- §7 B' `#[doc(hidden)] pub` for resolve intermediates
- §18 A-with-drift-test (see §31.7 for the non-regression rationale)

### 31.6 §25 severity correction

**File:** `native-theme/src/model/font.rs:40-49`

§25 describes `FontSize::Px(v).to_px(dpi)` as "silently ignoring"
the DPI parameter. Verified: `font.rs:47` reads `Self::Px(v) => v,`.

**Severity nuance:** the method's own rustdoc at lines 42-43
**explicitly documents the asymmetry** (as part of the method doc,
not hidden in a comment):

```rust
/// Convert to logical pixels.
///
/// - `Pt(v)` -> `v * dpi / 72.0`
/// - `Px(v)` -> `v` (dpi ignored)
pub fn to_px(self, dpi: f32) -> f32 { ... }
```

The complaint is not "silent behaviour" — the behaviour is
**explicitly documented** on the method itself. The real complaint
is **signature surprise**: a user reading only the method signature
(via rust-analyzer tooltip, or autocomplete hover, or cargo doc
header) sees `to_px(self, dpi: f32) -> f32` and assumes `dpi` is
used on both branches. Only reading the full rustdoc reveals the
asymmetry.

This is still a legitimate complaint — signatures should not
require reading the full rustdoc to predict behaviour — but it
is **weaker than the initial §25 framing suggested**. The word
"silently" implies the behaviour is hidden, and it is not.

**No option change.** §25 Option C (rename `to_px → resolve(dpi)`)
remains the recommendation — the rename addresses the
signature-surprise concern by signalling "this is a unit conversion
step, not always pixel math". The severity tier stays P3.

**Confidence:** high. The correction is factual (the rustdoc
really does document the asymmetry), and the recommendation is
unchanged.

### 31.7 Issue non-regressions

Three proposals I considered during the fourth pass but did NOT
promote to new options:

#### §8 struct-literal + `IconOptions` pattern — rejected after token-count analysis

I initially considered proposing a struct-literal alternative to
§8 Option C's builder:

```rust
// Struct-literal sketch:
load_icon(role, set);
load_icon_with(role, set, IconOptions { size: Small, ..Default::default() });
```

**Token-count analysis** against the builder:

| Scenario | Builder | Struct literal | Winner |
|---|---|---|---|
| Zero options | `load_icon(role, set)` — 22 chars | `load_icon(role, set)` — 22 chars | Tie |
| One option (`.size(Small)`) | `load_icon(role, set).size(Small)` — 35 chars | `load_icon_with(role, set, IconOptions { size: Small, ..Default::default() })` — 78 chars | **Builder by 43 chars** |
| Three options | `...size(S).color(c).theme("A")` — 50 chars | `IconOptions { size: S, fg_color: Some(c), theme: Some("A"), ..Default::default() }` — 82 chars | **Builder by 32 chars** |
| All options | comparable (~60-70 chars) | comparable (~80-90 chars) | Builder |

**The builder is strictly shorter at every non-trivial usage**,
and the struct-literal pattern only wins in zero-option cases
where both use the shortcut. The `..Default::default()` suffix
costs ~24 chars per call, which dominates the savings from
omitting method-chain punctuation.

**Recommendation:** **§8 Option C (builder) stands unchanged.**
The struct-literal pattern is a worse fit for this API.

#### §14 hand-maintained master slice — subsumed by §31.2 K

I initially considered a lightweight Option for §14: a
hand-maintained `const ALL_WIDGETS: &[(&str, &'static [&'static
str])]` slice that `lint_toml` iterates instead of the current
`VARIANT_KEYS` + `widget_fields` match. This collapses two drift
sites into one.

**Rejection rationale:** §31.2 K subsumes §14 entirely via
integrated `inventory::submit!` from the proc-macro derive. A
hand-maintained slice is still a drift hazard (one-site instead
of two-site, but not zero). K eliminates hand-maintenance
completely.

If K slips past v0.5.7, fall back to §14 F (inventory bridge
alone). The hand-maintained master slice is strictly worse than
both K and F — it lives in the backlog as a "last-resort fallback
if even inventory is unacceptable."

#### §18 strum dependency revisit under "perfect API"

I reconsidered whether the `#[derive(EnumString, IntoStaticStr)]`
strum path should be preferred over the drift-guard test under
"perfect API":

- **Argument for strum:** `strum` / `strum_macros` are already in
  `Cargo.lock` via connector dependencies, so connector users pay
  no marginal cost. Strum is cleaner code (attribute-driven
  generation vs hand-coded match). Under "perfect API" cleaner
  code wins.
- **Argument against strum:** For **standalone** `native-theme`
  users (not using connectors), strum is still a new direct dep.
  The drift-guard test has zero-dep cost and catches the same
  drift class. The enum has only 5 variants; the hand-coded
  match is 10 lines.

**Tiebreaker:** zero new deps is a genuine perfect-API virtue that
benefits the "standalone library" use case. The drift-guard test
provides equivalent correctness at zero dep cost.

**Recommendation:** **§18 A-with-drift-test stands.** If the enum
grows to 10+ variants in a future release, revisit and adopt strum
at that time.

### 31.8 Confidence statement (fourth pass)

**High confidence:**
- All §31.1 verification results
- §31.2 F (flat + kind() method) is the std-matching shape; precedent argument is airtight
- §31.2 H (plain RwLock) — latency math is unambiguous
- §31.3 feature-matrix cleanup bundled with §5 G
- §31.6 §25 severity correction (rustdoc verified)
- §31.7 §8 rejection (token counts confirmed)
- §31.7 §14 rejection (subsumed by K)

**Medium confidence:**
- §31.2 G (`ThemeMode` over `ThemeLayer`) — taste call, both are
  defensible; `ThemeMode` is marginally better per semantic
  accuracy + OS-vocabulary precedent
- §31.2 K direction (narrow derive proc-macro) — shape and scope
  are right; medium confidence on the exact attribute-syntax
  design surface, which needs a 1-2 day prototype against 2-3
  real widgets to finalise
- §31.2 K's ability to cover `inheritance-rules.toml` expressiveness
  without degenerating — the main unknown

**Low confidence / explicit unknowns:**
- Exact `native-theme-derive` LoC estimate (800-1500 is a bounded
  range but the lower bound depends on how much of `impl_merge!`'s
  existing logic can be preserved vs rewritten)
- Whether `inventory::submit!` inside a derive macro requires
  workarounds for incremental compilation (the pattern is known
  to work but has rough edges in older rustc releases)

### 31.9 What this pass did NOT change

Deliberately preserved from §1-§30:

- Every option letter A-F in the existing §1-§30 tables
- Every recommendation not explicitly strengthened or superseded
  in §31.4
- The §30.3 M1 macOS `button_order` bug — endorsed and unchanged
- The §30 third-pass additions — retained; §31's refinements are
  cumulative, not replacements
- §28's open questions list — §31 adds three new entries implicitly
  via the flagged unknowns in §31.8; existing entries remain
- §27's priority table entries that are not explicitly
  superseded in §31.5

If §31 does not reference a prior recommendation, that
recommendation is unchanged.

### 31.10 Endorsement of the existing P0 cohort

The fourth-pass review personally verified every P0 item against
the current tree and endorses the following items for v0.5.7
without reservation:

1. **M1** (§30.3) — macOS reader `button_order = PrimaryLeft` is a
   verified correctness bug with three-source contradiction. Ship
   the 2-line fix bundled with D5.
2. **§4** drop `active()`, keep `pick(is_dark)`. Verified landmine.
3. **§6a** drop `Error::Clone` bound. Dead weight.
4. **§12** partition crate root. Real count is ~91 items, well
   past any reasonable threshold.
5. **§17** remove `IconSet::default()`. The `#[default]` doc comment
   itself admits the value is known-wrong on non-Linux.
6. **§19** `LinuxDesktop` + `#[non_exhaustive]` + new Wayland
   compositor variants.
7. **§20** move `icon_set` / `icon_theme` to `Theme`.
8. **§22** feature-gate `on_theme_change` at compile time.
9. **Doc 2 A2/A3** `check_ranges` on placeholders — personally
   verified orchestration at `validate.rs:428-458`, the shared
   `missing` vec pattern is exactly as described.
10. **Doc 2 A4** move `button_order` fallback to
    `resolve_platform_defaults` — personally verified the
    "pure transform" doc lie at `resolve/mod.rs:20-22`.
11. **Doc 2 B4** split accessibility off `ThemeDefaults`.
12. **Doc 2 B6** split `BorderSpec` into defaults/widget types.
13. **Doc 2 C1/C2** remove `ThemeChangeEvent::Other`; rename
    `ColorSchemeChanged → Changed`.

These are verified facts on the ground. Ship the P0 cohort as a
coherent v0.5.7 release. The refinements in §31 (new options,
strengthened recommendations, feature-flag cleanup) slot into
this cohort without disrupting its shape.

---

## 32. Fifth-pass review: merged critical refinements

This section records a fifth ultrathink pass under the explicit
"backward compatibility does not matter; I want the perfect API"
directive. It re-verifies prior claims against the current tree,
corrects one self-mistake on the §12 crate-root count, surfaces
additional options on §1, §4, §6, §8, §13, and §25, narrows the
§2/§14 codegen scope to a concrete minimum-viable slice, and
strengthens the M1+D5+A4 bundling and §6a cleanup checklists.

This section runs parallel to **doc 2 §L** which hosts doc-2-specific
fifth-pass findings (L1–L4 new issues, A2/A3 re-verification with a
specific source-code self-admission, and cross-doc sequencing).

### 32.1 Methodology

Same approach as §30 and §31: independently re-read each existing
recommendation without consulting prior merge-review notes, then
reconciled. Every claim was independently verified against the
current tree at exact file:line positions. The pass was conservative
about adding new issues and aggressive about strengthening or
narrowing recommendations that still hedge.

### 32.2 Verification re-confirmations (and one self-correction)

- **A2/A3 shared-vec pattern** — personally re-read
  `native-theme/src/resolve/validate_helpers.rs:217-282`. The
  comment at lines 218-221 explicitly acknowledges the conflation:
  `"These push a descriptive message to the `errors` vec (reusing
  the same error-collection pattern as require()) so that all
  problems -- missing fields AND out-of-range values -- are reported
  in a single pass."` `check_positive` at lines 233-238 pushes
  strings like `"{path} must be a finite positive number, got
  {value}"`; `require` at line 31 pushes just `path`. The
  `ThemeResolutionError.missing_fields: Vec<String>` field holds
  two distinct string shapes. **A2/A3 verified with higher
  confidence than earlier passes.** See doc 2 §L.3 for the full
  re-verification.

- **§12 crate-root count self-correction.** During this pass I
  initially estimated "~78 items" on a quick recount and flagged
  §31.1's "~91" as inflated. On careful re-count of
  `native-theme/src/lib.rs:122-203` (with the 52-item `pub use
  model::{...}` block counted individually): 2 color + 2 error +
  52 model + 4 model::icons helpers + 2 freedesktop + 2 gnome +
  1 kde + 1 macos + 1 rasterize + 2 sficons + 1 windows + 2
  winicons + 3 watch + 2 detect types + 5 detect fns + 6 icons
  fns + 2 pipeline + 1 `Result` + 1 `SystemTheme` struct = **92
  items** (within ±1 of §31.1's "~89-91"). **My "~78" estimate
  was wrong; §31.1's count is accurate.** The argument for §12
  partitioning is if anything stronger than earlier passes
  suggested — 92 items is well beyond any reasonable flat-root
  scanning threshold.

- **M1 four-source cross-verification.** Re-ran
  `grep button_order=` across `native-theme/src/presets/`. All
  four macOS TOMLs (`macos-sonoma.toml:254,586` +
  `macos-sonoma-live.toml:126,285`) carry `"primary_right"`.
  Combined with `platform-facts.md:1481,1802` (Apple HIG citation)
  and `resolve/inheritance.rs:108` (`PrimaryRight` for non-Linux),
  **M1 is verified against four independent sources.**
  `macos.rs:504-505` is the sole disagreement across the entire
  tree. This is a higher bar than §30.3's three-source
  verification.

### 32.3 New options added to existing problems

#### §1 — Option H: preserve `SystemTheme` unchanged

§1 Option B is a five-rename package. The fifth rename (`SystemTheme
→ DetectedTheme` or `ThemeBundle`) is the weakest link: both
replacement names add a new word users must learn while `SystemTheme`
is already exactly what the type represents (*the theme as the
system currently has it*).

| # | Option | Pros | Cons |
|---|---|---|---|
| H | **Keep `SystemTheme` unchanged** while adopting B's other four renames (via §31.2 G: `ThemeSpec → Theme`, `ThemeVariant → ThemeMode`, `ResolvedThemeVariant → ResolvedTheme`, `ResolvedThemeDefaults → ResolvedDefaults`). | Zero rename churn on the type most users reach for at the top of `main.rs`. `SystemTheme` is already accurate — literally the detected system theme. `DetectedTheme` adds a preamble adjective for no semantic gain; `ThemeBundle` is actively misleading (the type's salient property is *"from the OS"*, not *"holds two variants"*). Saves one rename, one import update per user, and resolves §28 open question 1. | Readers who skim the crate root see a bare `SystemTheme` that lacks the parallel structure of the four `…Theme` renames. The parallelism win is cosmetic. |

**Recommendation:** adopt **H**. Drop the fifth rename from §1
Option B. Replace §28 open question 1 with a note that the rename
was reconsidered and `SystemTheme` stays. This is the only rename
in the original B that fails the "no rename without a semantic
win" test.

**Confidence:** medium-high. Taste call at the margin; the
cost/benefit inverts once `SystemTheme` is evaluated on its own
merits rather than by analogy to the other four.

#### §4 — Option G: `pick(ColorMode)` replaces `pick(bool)`

§4 Option C (drop `active()`, keep `pick(is_dark: bool)`) is
correct on the staleness-trap axis but leaves a `bool` argument
in the public API. `bool` arguments are a well-known anti-pattern
at call sites:

```rust
sys.pick(true);   // true = dark? light? ambiguous at call site
sys.pick(false);  // same question on every call
```

Under "perfect API" the fix is obvious: replace `bool` with a
self-documenting enum. This composes naturally with §31.2 G's
`ThemeVariant → ThemeMode` rename — once theme data is per-mode,
the mode discriminator wants its own type too.

| # | Option | Pros | Cons |
|---|---|---|---|
| G | **`pick(mode: ColorMode)`** where `#[non_exhaustive] pub enum ColorMode { Light, Dark }`. `SystemTheme.is_dark: bool` becomes `SystemTheme.mode: ColorMode`. Callers write `sys.pick(ColorMode::Dark)` for explicit, `sys.pick(sys.mode)` for captured, `sys.pick(ColorMode::from_os())` for live. | Eliminates the `bool`-argument anti-pattern. Call sites are self-documenting. Parallels `DialogButtonOrder`, `FontStyle`, `IconSet` — already-enum types in the crate. Adding a third variant (e.g. `HighContrast`) via `#[non_exhaustive]` in a future release is non-breaking. `ColorMode::from_os()` is a cleaner replacement for the current `detect::system_is_dark() -> bool` at the call site. Matches GNOME `color-scheme` / macOS NSAppearance / Windows ColorMode terminology. | One more enum for users to learn. Eight extra characters per call site vs a `bool`. |

**Final shape:**

```rust
#[non_exhaustive]
pub enum ColorMode { Light, Dark }

impl ColorMode {
    /// Read the current OS color mode (live detection, no caching).
    pub fn from_os() -> Self { ... }
}

pub struct SystemTheme {
    pub mode: ColorMode,                    // was: is_dark: bool
    pub light: ResolvedTheme,
    pub dark: ResolvedTheme,
    // ...
}

impl SystemTheme {
    pub fn pick(&self, mode: ColorMode) -> &ResolvedTheme {
        match mode {
            ColorMode::Light => &self.light,
            ColorMode::Dark  => &self.dark,
        }
    }
    // No active() method. No is_dark: bool field.
}
```

Call-site idioms (all three read cleanly; zero bool):
- **Captured:** `sys.pick(sys.mode)`
- **Live:** `sys.pick(ColorMode::from_os())`
- **Explicit:** `sys.pick(ColorMode::Dark)`

**Recommendation:** adopt **G** paired with §4 Option C's removal
of `active()`. Name: `ColorMode` (not `Mode` — too ambiguous; not
`ThemeMode` — collides with §31.2 G's `ThemeVariant → ThemeMode`
rename).

**Naming clarity:** `ThemeMode` is the data-per-mode type (what
§31.2 G renames). `ColorMode` is the enum discriminator. The two
never conflict because one is a struct-type and the other is an
enum; users read `ThemeMode` as "one mode of a theme" and
`ColorMode` as "a color mode selector."

**Rationale against `Mode` alone:** "Mode" is the shortest but
most ambiguous name possible in a Rust crate (file mode, debug
mode, edit mode, etc.). Under "perfect API" the name should be
unambiguous at the use site. `ColorMode` is one extra word but
eliminates all collision risk.

**Confidence:** high on the direction (`bool` arguments are a
well-known anti-pattern; this is a textbook refactor under "no
backward compat"). High on `ColorMode` as the name (GNOME /
Windows terminology match, zero collision with other renames).

**Cross-reference:** resolves §28 open question on naming. Adds
one implicit open question: whether to also add `ColorMode::Auto`
as a third variant for "follow system", for connectors that want
to express "don't care; just do what the OS says" without the
`from_os()` lookup. Lean: **no** — `Auto` is a rendering concept
belonging to the connector, not a data concept belonging to
`native-theme`. Defer to v0.5.8 if it ever becomes a real request.

#### §6 — §6a is a four-item commit, not a one-line Clone drop

§6a's merge-review addendum catalogues the stale `presets.rs:85-92`
comment. This pass found two additional stale items that must be
deleted in the same commit:

1. **`native-theme/src/error.rs:73-79`** — doc comment on the
   `Error` enum justifying `Clone` via a caching use case that
   doesn't use `Clone`:
   ```rust
   /// This type is [`Clone`] so it can be stored in caches alongside
   /// [`crate::ThemeSpec`]. The `Platform` and `Io` variants use [`Arc`]
   /// internally to enable cloning without losing the original error chain.
   ```
   Same stale rationale as `presets.rs:85-87`. Deleted with §6a.

2. **`native-theme/src/error.rs:239-250`** — `error_is_clone` test:
   ```rust
   #[test]
   fn error_is_clone() {
       fn assert_clone<T: Clone>() {}
       assert_clone::<Error>();
       // ... more Clone assertions
   }
   ```
   Dropping `Clone` from `Error` makes this test fail to compile.
   **Delete entirely** when §6a lands.

3. **`native-theme/src/presets.rs:85-88`** — already in §6a's
   merge-review addendum and doc 2 §I1.

**§6a four-item commit contents:**

| Item | File:line | Action |
|---|---|---|
| 1 | `error.rs:80` | Remove `Clone` from `#[derive(...)]` |
| 2 | `error.rs:73-79` | Delete the stale doc comment |
| 3 | `error.rs:239-250` | Delete the `error_is_clone` test |
| 4 | `presets.rs:85-92` | Update cache to `Result<ThemeSpec, Error>`; delete stale comment |

Ship as a single commit titled *"drop `Error: Clone` bound; remove
stale caching justifications and tests."*

**Confidence:** high. Four-item checklist, all verified, all
mechanical. Cross-referenced from doc 2 §L1 (which adds L1 for the
`error.rs:73-79` comment) and doc 2 §I1 (unchanged).

#### §8 — refinement: `IconSize::Px(NonZeroU32)` + `#[non_exhaustive]`

§30.2's §8 refinement introduces `IconSize` as a token enum with
`Px(u32)` as the escape hatch. Two small strengthenings under
"perfect API":

| Refinement | Why |
|---|---|
| **`Px(NonZeroU32)`** instead of `Px(u32)` | Zero is never a valid icon size. The type should reject it at construction. Matches doc 2 §C3's `TransformAnimation::Spin { duration_ms: NonZeroU32 }` — another "zero is a category error" case already caught in doc 2. |
| **`#[non_exhaustive]`** on the enum | Adding future token variants (e.g. `Jumbo`, `Display`) is non-breaking. Every public data enum in the crate should be `#[non_exhaustive]` under "perfect API" unless there is a specific reason not to be (`LinuxDesktop` in §19 is a canonical exception — tied to a finite real-world set of desktop environments, and even there §19 recommends adding the attribute). |

**Final sketch (supersedes §30.2's):**

```rust
use std::num::NonZeroU32;

#[non_exhaustive]
pub enum IconSize {
    Small,       // ~16 px on Freedesktop / .small on SF Symbols
    Medium,      // ~24 px on Freedesktop / .medium on SF Symbols
    Large,       // ~32 px on Freedesktop / .large on SF Symbols
    ExtraLarge,  // ~48 px on Freedesktop / .xlarge on SF Symbols
    Px(NonZeroU32),
}

impl IconSize {
    /// Construct an explicit pixel size. Returns `None` if `value` is 0.
    pub const fn px(value: u32) -> Option<Self> {
        match NonZeroU32::new(value) {
            Some(nz) => Some(Self::Px(nz)),
            None     => None,
        }
    }
}
```

The `px()` constructor returns `Option<Self>` — users cannot construct
a zero-px icon at all without `unsafe`. No runtime panics possible.

**Recommendation:** adopt both refinements. Zero downside; one
extra line of code; type-level zero-rejection.

**Confidence:** high.

#### §25 — additional options under "perfect API" (none better than C)

§25's recommendation hedges between C (rename to `resolve(dpi)`)
and B (split by type). §31.6 notes the rustdoc already documents
the asymmetry, softening the severity. Under "perfect API" three
more options deserve listing:

| # | Option | Pros | Cons |
|---|---|---|---|
| F | **Split into two methods**: `to_px_from_pt(dpi)` (errors if Px) and `as_px()` (errors if Pt). Users match the variant before calling. | Type-level asymmetry is explicit | Verbose at the call site; the match-first pattern is friction for a one-liner. |
| G | **Typestate**: `FontSize<PtUnit>` and `FontSize<PxUnit>` as generic markers. Pt-to-px is a typed conversion. | Fully type-safe | Generics in public signatures; serde round-trip breaks; overkill for a conversion helper. |
| H | **Delete the enum**: `FontSize = f32 (pt only)`. Users who want pixels compute `pt * dpi / 72` at author time. | Eliminates the whole §25 problem. | Breaks any TOML preset that stores pixel-exact sizes. Semantic loss. |

**Recommendation:** **keep §25 Option C** as drafted. F/G/H each
trade a real cost (verbosity, generics, semantic loss) for a small
clarity win. §31.6's severity-note is correct: the rustdoc already
documents the asymmetry, and the signature surprise is bounded.

**One low-cost improvement:** rename `to_px(dpi)` → `to_logical_px(dpi)`
instead of §25 C's `resolve(dpi)`. "Logical pixels" is the term
already used elsewhere in the crate (`ResolvedFontSpec::size` doc
comments). Same effort as §25 C, slightly more descriptive.

**Confidence:** high on keeping C; medium on the `to_logical_px`
rename over `resolve`.

#### §2 + §14 — K scope narrowing for v0.5.7

§31.2 K recommends a narrow `#[derive(ThemeLayer)]` proc-macro.
The scope under that recommendation is implicitly "everything
B1/B6/B7 asks for." Under "perfect API" the scope should be
**explicitly narrowed** to a minimum-viable slice for v0.5.7,
with the broader scope deferred to v0.5.8.

**Minimum-viable K for v0.5.7:**

| Delivered | Mechanism | Replaces |
|---|---|---|
| `check_ranges` impls per widget | Derive generates from `#[theme(range = "0.0..=1.0")]` attributes | ~450 lines at `model/widgets/mod.rs:908-1347` |
| `FIELD_NAMES` constants | Derive generates from the struct field list | `FIELD_NAMES` emission in `define_widget_pair!` |
| `impl_merge!` body | Derive generates per-field `if other.foo.is_some() { self.foo = other.foo.clone() }` | `impl_merge!` macro body |
| Border-kind dispatch (B6/B7) | `#[theme_layer(border_kind = "full" \| "partial" \| "none")]` attribute | Hand-coded decision in `define_widget_pair!` clauses |
| `inventory::submit!` widget registry entry | One line per widget in the derive output | `VARIANT_KEYS` + `widget_fields` at `model/mod.rs:563-628` |

**Explicitly deferred to v0.5.8 (do NOT ship in v0.5.7):**

| Deferred | Why | v0.5.7 workaround |
|---|---|---|
| Paired struct generation (`XxxTheme` + `ResolvedXxxTheme` from one source) | Biggest scope risk; changes every widget at once | Keep `define_widget_pair!` for struct generation; derive only operates on the structs it produces |
| Pattern-based cross-widget inheritance codegen | Attribute expressiveness unknown; cross-widget rules need design discussion | Keep `inheritance.rs` hand-maintained; per-field `#[theme(inherit_from = "...")]` only for simple per-field cases |
| `BorderSpec` split via registry-driven type generation | Schema design required | Ship doc 2 B6 **by hand** (two hand-written types); migrate to generated in v0.5.8 |

**Rationale for the narrowing:**

1. **Risk containment.** Minimum-viable scope is a ~1-week job;
   full scope is 2–4 weeks with multiple structural risks.
2. **Incremental value.** The minimum-viable slice already
   eliminates the worst drift hazards: ~215 `lint_toml` literals
   + ~450 `check_ranges` lines + `VARIANT_KEYS`/`widget_fields`
   double-maintenance. That is ~80% of the drift-risk reduction
   for ~20% of the work.
3. **Proof by construction.** If minimum-viable K ships
   successfully in v0.5.7, the attribute-syntax design gets
   validated against real usage before committing to harder
   struct-generation work. If it surfaces unexpected blockers,
   v0.5.7's doc 1 P0 + doc 2 P0 cohorts still ship without the
   codegen dependency.
4. **Fallback path.** If even minimum-viable K slips past the
   v0.5.7 schedule, §14 Option F (`inventory` bridge alone) is
   a ~20-line drop-in that eliminates the `VARIANT_KEYS` /
   `widget_fields` drift for ~1/10 the effort.

**Recommendation:** **adopt minimum-viable K for v0.5.7** with
§14 F as the fallback bridge. Promote the deferred items to
v0.5.8 as explicit backlog entries (not "TBD"). This is the
sixth revision of the §2/§14 direction across passes; each
previous pass sharpened the approach and this pass sharpens
the scope.

**Confidence:** high on the scope cut. Medium on the 1-week
estimate — `impl_merge!` translation and attribute-syntax design
are both "straightforward but might surface edge cases" work.

#### §13 — cache-fill blocking design consideration

§31.2 H (plain `RwLock<Option<T>>`) is correct on latency math.
Doc 2 §I3 flags the `run_gsettings_with_timeout` 2-second worst
case. These interact in a way that neither §31.2 nor §I3
addresses:

**The interaction:** when `DetectionContext::is_dark()` runs for
the first time after process start (or after an `invalidate_*()`
call), the thread calling it holds the write lock for the
duration of the detection call. On cold Linux caches, that is
**up to 2 seconds with the write lock held**. Any concurrent
reader blocks for the full 2 seconds.

For a GUI app where thread A renders frames (calling `is_dark()`
every frame) and thread B just invalidated the cache after a
theme-change event, thread A can stall for 2 s on the next frame
read. That is a visible UI freeze.

**Options:**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Accept it.** Document "first call after invalidation may block up to 2 s on cold Linux." | Zero complexity | Visible stalls in GUI apps |
| B | **Spawn detection on a dedicated background thread**; foreground returns a tentative value until detection completes. Use `Arc<Mutex<DetectionState>>` with `NotStarted`/`InProgress(JoinHandle)`/`Done(T)` states. | Foreground never blocks; stale value briefly visible, then updates | Complexity; thread spawn per invalidation; short window of stale reads |
| C | **Hold only a read lock during detection**, re-acquire as write once the value is ready. Multiple readers can see `None` and race; winner publishes. | Simpler than B; foreground never blocks after the first reader | First `invalidate_*()` caller still sees "stale" until its own call returns; race between invalidation and detection is subtle |
| D | **Cap the detection timeout at 250 ms** (down from 2 s). A cold `gsettings` over 250 ms is rare; treat as "unavailable" and fall through. | Bounds the worst case at 250 ms — below one 60 Hz frame | Risk of missing slow-but-successful responses on pathologically slow systems |
| E | **Don't cache at all in `DetectionContext`** — every call does fresh detection. Punt caching to the caller. | Simplest; no cached state to invalidate | Per-frame `system_is_dark()` in connectors becomes 2 s per call on cold caches. Worse. |

**Recommendation:** pair **C** (read-only lock during detection)
with **D** (cap timeout at 250 ms). C eliminates the
write-lock-hold-during-detection interaction; D bounds the
absolute worst case to under a frame budget even in the
pathological case. Together they keep the hot path responsive.

**Position:** refinement inside §13 Option C (`DetectionContext`)
as an implementation detail. Does not replace any earlier
recommendation; adds a design consideration to resolve during
implementation.

**Confidence:** medium. The write-lock interaction is real (a
consequence of `RwLock` semantics); the best mitigation depends
on measured latency that should be gathered during
implementation.

### 32.4 Strengthened recommendations

#### Ship M1 + D5 + A4 as a single atomic commit

§30.3 and doc 2 K.4 recommend bundling M1 + D5. Doc 2 A4
recommends moving `button_order` fallback from
`resolve_safety_nets` to `resolve_platform_defaults`. Under
"perfect API" all three touch one conceptual decision and should
**ship as one atomic commit**:

**Commit title:** *"delete reader-side `button_order` hardcodes;
move platform fallback to `resolve_platform_defaults`; presets +
resolver are authoritative"*

**Commit contents:**

1. Delete `native-theme/src/macos.rs:504-505` (M1 fix; including
   the factually-wrong comment at line 504)
2. Delete `native-theme/src/kde/mod.rs:52-53` (D5 fix)
3. Move `button_order` fallback from
   `native-theme/src/resolve/inheritance.rs:164-167`
   (`resolve_safety_nets`) to
   `native-theme/src/resolve/mod.rs:52-56`
   (`resolve_platform_defaults`) (A4 fix)
4. Update any reader-side tests that depended on the hardcodes
   (spot-check `macos.rs:805+` test block and `kde/mod.rs` test
   block; adjust assertions that verify the reader-side value)

**Why single-commit:**

- All three touch one conceptual decision (`button_order`
  provenance: *who decides what it is*).
- Splitting into three commits leaves intermediate states where
  one source is "right" while another is in flux.
- Single commit gives a clean revert if any edge case surfaces
  during bisection.
- Review burden is easier as one atomic architectural change.

**Strengthened recommendation:** **mandatory single-commit
shipping**. Tagged **P0** in §27 (already P0, now with explicit
atomicity constraint). See doc 2 §L.4 for the broader ship-unit
sequencing.

**Confidence:** high. Single-commit atomicity is the correct
granularity for an architectural-principle fix.

#### §6a four-item commit per §32.3 above

Already documented in §32.3 §6. Ship as a single commit titled
*"drop `Error: Clone` bound; remove stale caching justifications
and tests."*

### 32.5 Priority rebalance

No new P0 items introduced. §32.3 additions slot in as refinements
to existing P0s:

**P0 refinements (no tier change):**

- §1 gets Option H (keep `SystemTheme`); resolves §28 open
  question 1
- §4 gets Option G (`pick(ColorMode)`); strengthens existing
  drop-`active()` recommendation
- §6 gains the three-deletion checklist (§32.3 §6 + §32.4)
- §8 gains `NonZeroU32` + `#[non_exhaustive]` refinements

**P1 refinements:**

- §2 + §14 get explicit minimum-viable scope cut
- §13 gets write-lock + timeout design considerations

**Bundling strengthenings:**

- M1 + D5 + A4 mandatory single commit (strengthens §30.3 + §31
  which recommended M1+D5 only)
- §6a four-item mandatory checklist (strengthens §6a which listed
  only the Clone drop + `presets.rs` cleanup)

**Nothing promoted or demoted.** The §31.5 cohort stands.

### 32.6 Cross-document consistency

Items in §32 that require coordinated doc 2 changes:

- §32.3 §6 three-deletion checklist → doc 2 §L1 adds L1 for the
  `error.rs:73-79` comment (new issue not in I1). Coordinate so
  both doc references converge on the same four-item §6a commit.
- §32.4 M1 + D5 + A4 bundling → doc 2 §L.4 endorses this
  strengthening and extends the ship-unit sequencing.
- §32.3 §2 K scope narrowing → doc 2 §K.2 delivers B1 + B6 + B7
  via K. Narrowing means **B6 ships hand-written in v0.5.7**,
  codegen migration deferred to v0.5.8. Doc 2 §L.5 re-tiers B6
  accordingly.

### 32.7 What this pass did NOT change

Deliberately preserved from passes 1–4:

- Every option letter A–F in the §1–§30 tables
- Every pros/cons entry in existing option tables
- §31 entries not explicitly extended in §32.3 / §32.4
- §27 priority table entries not explicitly rebalanced in §32.5
- §28 open questions list (§32.3 H resolves one item;
  §32.3 G potentially adds one on `ColorMode::Auto`)
- §30.3 M1 problem description and verification chain —
  §32.4 only strengthens the atomicity constraint
- §31.2 K as the codegen direction — §32.3 only narrows the
  v0.5.7 scope
- §31.10 P0 cohort endorsement — §32.2 re-confirms every item

If §32 does not reference a prior recommendation, that
recommendation is unchanged.

### 32.8 Confidence statement (fifth pass)

**High confidence:**

- §32.2 verification re-confirms (including the self-correction
  on §12 count — the §31.1 figure is accurate)
- §32.3 §1 Option H (`SystemTheme` preservation) — pure
  cost/benefit analysis under "perfect API"
- §32.3 §4 Option G (`pick(ColorMode)`) — `bool`-argument
  elimination is a textbook refactor
- §32.3 §6 four-item checklist — all items personally verified
- §32.3 §8 `NonZeroU32` + `#[non_exhaustive]` — zero-downside
  additions
- §32.3 §2+§14 minimum-viable scope — risk-containment reasoning
  is sound
- §32.4 M1 + D5 + A4 single-commit bundle — atomicity is the
  correct granularity for an architectural-principle fix

**Medium confidence:**

- §32.3 §25 `to_logical_px` rename over `resolve` — marginal
  taste call
- §32.3 §13 write-lock + timeout design — the interaction is
  real; best mitigation depends on measured latency

**Explicit unknowns:**

- Whether minimum-viable K's 1-week estimate holds up against
  `impl_merge!` translation edge cases
- Whether `ColorMode::Auto` will become a real request in
  v0.5.8+ (deferred; lean: no)

### 32.9 Endorsement of the v0.5.7 P0 cohort (fifth-pass confirmation)

All items in §31.10's P0 list personally re-verified this pass.
The §32 additions slot as refinements to existing P0s without
disturbing the cohort's shape.

**Final v0.5.7 P0 cohort, consolidated across all five passes:**

1. M1 + D5 + A4 atomic commit (macOS/KDE button_order correctness
   + resolve purity leak)
2. A2 + A3 + §6 Option F restructure (validate orchestration +
   Error shape)
3. §6a four-item commit (drop Clone + three stale cleanups)
4. §19 (`LinuxDesktop` `#[non_exhaustive]` + Wayland compositor
   variants)
5. C1 + C2 atomic commit (`ThemeChangeEvent` cleanup)
6. §17 (remove `IconSet::default`)
7. §1 Option B + §31.2 G + §32.3 H rename package:
   `ThemeSpec → Theme`, `ThemeVariant → ThemeMode`,
   `ResolvedThemeVariant → ResolvedTheme`,
   `ResolvedThemeDefaults → ResolvedDefaults`;
   `SystemTheme` unchanged.
8. §4 Option C + §32.3 G (`pick(ColorMode)`, `SystemTheme.mode:
   ColorMode`, drop `active()`)
9. §12 + prelude (partition crate root; 92-item count verified)
10. §20 (move `icon_set`/`icon_theme` to `Theme`)
11. §16 (`Rgba` polish)
12. §22 (feature-gate `on_theme_change`)
13. §3 + doc 2 B5 atomic commit (`OverlaySource` +
    `ResolutionContext`)
14. Doc 2 B4 (`AccessibilityPreferences` on `SystemTheme`)
15. Doc 2 B6 hand-written split (codegen deferred to v0.5.8)

**P1 (concurrent with or after P0):**

- Minimum-viable K codegen (doc 1 §32.3 §2+§14)
- Doc 2 L2 `ENV_MUTEX` test simplification follow-up after A4
- Doc 2 L3 `from_kde` visibility audit before C6
- §7 `#[doc(hidden)] pub` demotion (doc 1 §31 B')
- Doc 2 C3 `AnimatedIcon` newtype wrappers
- Doc 2 C4 `Arc<str>` font family refactor

**P2 (scheduled after P0/P1 lands):**

- §5 G + §31.3 + doc 2 L4 feature-matrix cleanup (bundled; §5 G
  gates §31.3 and L4)
- §13 `DetectionContext` with write-lock + timeout mitigations
- Doc 2 C5 `detect_linux_desktop()` no-arg convenience
- Doc 2 C6 demotion (after L3 audit)
- Doc 2 D3 lazy error-string allocation (subsumed by minimum-viable
  K if it lands)
- Doc 2 D4 `Cow<'static, str>` for bundled names

**P3:**

- §25 `to_logical_px` rename
- Doc 2 D1 `FontSpec::style` asymmetry documentation
- §26 prune preachy `must_use` messages
- §18 drift-guard test for `IconSet::from_name`
- §21 `ThemeWatcher` rename + doc
- §24 `platform_preset_name` structured return
- §23 `diagnose_platform_support` structured return

These lists consolidate the P-tiering across all five passes.
Ship P0 as v0.5.7; stage P1/P2/P3 across v0.5.8 / v0.5.9 per
developer capacity.
