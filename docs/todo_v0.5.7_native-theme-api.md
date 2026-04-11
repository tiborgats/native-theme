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

### Recommended: **B** (high confidence for sync; medium for block_on)

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

---

## 8. Icon loading: 12 functions, one user intent

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

Twelve public functions for "load an icon", with subtly different
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

## 12. Flat crate root exports 80+ items

**File:** `native-theme/src/lib.rs:122-203`

### Problem

The crate root re-exports:

- ~35 model types (every widget, every Resolved version, defaults, etc.)
- `Error`, `Result`, `ParseColorError`, `ThemeResolutionError`
- `Rgba`
- 11 icon-loading free functions
- 7 detection functions (cached + uncached)
- 5 platform reader functions (`from_kde`, `from_macos`, etc.)
- 6 platform-specific icon functions
- 3 pipeline helpers (`platform_preset_name`, `diagnose_platform_support`, `rasterize_svg`)
- 2 bundled lookup functions
- `LinuxDesktop`, `detect_linux_de`
- Watcher types
- The `Skill` macro hasn't eaten the name space but plenty else has

Users arriving at `docs.rs/native-theme` scroll through 80+ alphabetically
sorted items to find what they need. Discoverability is awful.

### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo.** Flat root. | No change. | Scales poorly with every new widget / function. |
| B | **Keep re-exports but group them in rustdoc** via `#[doc(alias)]` or sections | Cosmetic only; zero code impact. | Rustdoc grouping is limited; no code browsing improvement. |
| C | **Partition into modules**: `native_theme::model::*`, `::icons::*`, `::detect::*`, `::watch::*`, `::readers::*` (internal). Keep a handful of root re-exports for the most common items (`Theme`, `ResolvedTheme`, `DetectedTheme`, `Error`, `Result`, `Rgba`). | Dramatic discoverability win. Users scan a dozen modules, each with 5-15 items, instead of a flat 80-item list. | Every import in user code changes. |
| D | **Full hierarchy (no root re-exports)**. Users always use `native_theme::model::Theme`. | Most principled | Verbose at use site |
| E | **Just rename the crate to `nt` or similar** to shorten root imports | Easy | Doesn't fix the underlying problem |

### Recommended: **C**

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

---

## 13. Global static caches in `detect` and `model/icons`

**Files:**
- `native-theme/src/detect.rs:55` (`CACHED_IS_DARK`)
- `native-theme/src/detect.rs:584` (`CACHED_REDUCED_MOTION`)
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

### Recommended: **D** (registry-driven), with **C** as a mode

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

Recommend **C**: have `Theme::preset`, `Theme::from_toml`, and the
pipeline all return a `Theme` type that guarantees at least one
variant; `Theme::pick_variant(Mode) -> &ThemeLayer` is infallible.
The only path that produces a variant-less theme is the (now-deleted)
`ThemeSpec::new` from 15d.

**15f (structured preset info):**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | Keep `&'static str` | Zero-alloc | No metadata |
| B | `pub struct PresetInfo { name: &'static str, kind: PresetKind, platform: Platform, has_light: bool, has_dark: bool }` | Rich metadata | More types |

Recommend **B**.

### Recommended (bundled)

1. Delete `from_toml_with_base` and `with_overlay_toml`.
2. Make `list_presets()` and `list_presets_for_platform()` both return
   `&[PresetInfo]` (the former is a compile-time constant; the latter
   is a filtered view, which can be a `Vec<PresetInfo>` or an iterator).
3. Delete `ThemeSpec::new(name)`.
4. Require `Theme` to have at least one variant by construction
   (`Theme::preset` and `Theme::from_toml` already guarantee this in
   practice). `pick_variant` becomes infallible.

### Rationale

Each sub-issue is small on its own, but together they represent a
grab-bag type that has accreted convenience methods without a consistent
design. v0.5.7 is the right window to prune.

The only non-obvious call is 15e. Option **B** (panic) is rejected by
the no-panic rule. Option **C** (type-level guarantee) requires that
`Theme::new` be either deleted (15d) or replaced with something that
forces at least one variant. I think deletion is cleaner.

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

### Recommended: **B** (add strum)

### Rationale

`strum` is a well-maintained, minimal-dependency crate that does
exactly this job. Dropping into the dependency graph is a small cost
for eliminating the three-place-update problem.

Option **D** (delete) is tempting but breaks users who parse icon-set
names from config outside of serde (e.g. command-line arguments). Not a
huge constituency but non-zero.

Option **C** is hacky -- no.

Option **B** costs one dependency and removes a drift hazard.

**Confidence:** medium. The `strum` dependency is a real cost; if the
crate values zero extra deps highly, option **A** with a "keep in sync"
comment is an acceptable fallback.

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
| P3 | §18 Replace `IconSet::from_name`/`name` with strum | Low | Low | Medium |
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

8. **§18: `strum` dependency acceptable?** Adding a minor dependency
   removes a drift hazard. If the crate's philosophy is "zero optional
   deps for the core model," this is a no-go.

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
