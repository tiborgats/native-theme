# v0.5.7 -- native-theme: API Overhaul (Part 2)

Second-pass critical analysis of the public API of the `native-theme` crate.
This document is a follow-up to `docs/todo_v0.5.7_native-theme-api.md`
(henceforth "doc 1") and covers issues not included there.

All items have file:line references, multiple solution options with
pros/cons tables, a recommended fix with explicit rationale, and an
honest "confidence" marker when the recommendation involves judgement.

Every claim has been cross-checked against the current tree. Where a
first-pass assertion turned out to be overstated on closer reading, the
recommendation is adjusted in-line rather than dropped silently.

---

## Relationship to doc 1

Doc 1 covered: type vocabulary, doubled struct hierarchy, `SystemTheme`
memory cost, `active()` vs `pick()`, async path unification, `Error`
restructure, `resolve*` method count, icon builder, flat crate root,
global detect caches, `lint_toml` duplication, `ThemeSpec` method
grab-bag, `Rgba` polish, `IconSet::default`, `LinuxDesktop`
`non_exhaustive`, `icon_set`/`icon_theme` location, `ThemeWatcher`
internals, feature-gated `on_theme_change`, `diagnose_platform_support`,
`platform_preset_name`, `FontSize::Px` DPI ignore, `#[must_use]`
messages.

This document covers: runtime panics, validator error conflation, "pure
transform" layers that aren't, validate/range-check boilerplate as a
codegen target, inheritance-rules drift, reader-output heterogeneity,
accessibility state placement, `font_dpi` placement, `BorderSpec`
type-vs-usage, `AnimatedIcon` construction validity, `ThemeChangeEvent`
naming, font family ownership, `detect_linux_de` ergonomics, platform
reader publicness, and a handful of smaller items.

Nothing in this document should contradict doc 1; the two are a
partitioned set of recommendations.

---

## Guiding principles (recap)

- **No runtime panics** (CLAUDE.md rule).
- **No invented values** (CLAUDE.md rule).
- **Breaking changes are allowed** for v0.5.7.
- **Connectors are canonical consumers**; private power-user paths can
  be narrowed.
- **Pure-vs-impure boundaries are load-bearing**; if a module is
  documented as pure, it must be pure.

---

## Table of contents

- **A. Bugs**
  - [A1. Defensive `checked_sub` for `Instant` arithmetic](#a1-defensive-checked_sub-for-instant-arithmetic)
  - [A2. `check_ranges` runs on `T::default()` placeholders, producing spurious errors](#a2-check_ranges-runs-on-tdefault-placeholders-producing-spurious-errors)
  - [A3. `ThemeResolutionError::missing_fields` carries two error categories](#a3-themeresolutionerrormissing_fields-carries-two-error-categories)
  - [A4. `resolve()` doc says "no OS detection" but reads `XDG_CURRENT_DESKTOP`](#a4-resolve-doc-says-no-os-detection-but-reads-xdg_current_desktop)
- **B. Structural issues**
  - [B1. ~720 lines of hand-maintained validate/range-check boilerplate](#b1-720-lines-of-hand-maintained-validaterange-check-boilerplate)
  - [B2. Inheritance rules duplicated between TOML source-of-truth and Rust implementation](#b2-inheritance-rules-duplicated-between-toml-source-of-truth-and-rust-implementation)
  - [B3. Platform readers have a heterogeneous variant contract (single vs dual)](#b3-platform-readers-have-a-heterogeneous-variant-contract-single-vs-dual)
  - [B4. `ThemeDefaults` mixes static theme data with accessibility preferences](#b4-themedefaults-mixes-static-theme-data-with-accessibility-preferences)
  - [B5. `font_dpi` on `ThemeDefaults` mixes runtime and static data](#b5-font_dpi-on-themedefaults-mixes-runtime-and-static-data)
  - [B6. `BorderSpec` allows defaults-only fields at widget level](#b6-borderspec-allows-defaults-only-fields-at-widget-level)
  - [B7. Three parallel border-inheritance validation paths](#b7-three-parallel-border-inheritance-validation-paths)
- **C. API-shape issues**
  - [C1. `ThemeChangeEvent::Other` is defined but never emitted](#c1-themechangeeventother-is-defined-but-never-emitted)
  - [C2. `ColorSchemeChanged` is platform-inaccurate on Linux](#c2-colorschemechanged-is-platform-inaccurate-on-linux)
  - [C3. `AnimatedIcon` public fields allow invalid construction](#c3-animatedicon-public-fields-allow-invalid-construction)
  - [C4. Font family ownership: owned `String` per widget × connector leak](#c4-font-family-ownership-owned-string-per-widget--connector-leak)
  - [C5. `detect_linux_de` takes `&str`, forcing a two-call idiom](#c5-detect_linux_de-takes-str-forcing-a-two-call-idiom)
  - [C6. Platform reader functions are `pub` but only useful inside the pipeline](#c6-platform-reader-functions-are-pub-but-only-useful-inside-the-pipeline)
- **D. Polish**
  - [D1. `FontSpec::style` silently defaults while sibling fields error](#d1-fontspecstyle-silently-defaults-while-sibling-fields-error)
  - [D2. `defaults.border.padding` derives from the presence of unrelated fields](#d2-defaultsborderpadding-derives-from-the-presence-of-unrelated-fields)
  - [D3. `check_ranges` builds path strings eagerly via `format!`](#d3-check_ranges-builds-path-strings-eagerly-via-format)
  - [D4. `name` and `icon_theme` are owned `String` for bundled presets](#d4-name-and-icon_theme-are-owned-string-for-bundled-presets)
  - [D5. `from_kde_content_pure` hardcodes `button_order` that resolution already handles](#d5-from_kde_content_pure-hardcodes-button_order-that-resolution-already-handles)
- [E. Priority summary](#e-priority-summary)
- [F. Open questions](#f-open-questions)
- [G. Post-script: the crate is two codebases pretending to be one](#g-post-script-the-crate-is-two-codebases-pretending-to-be-one)

---

## A. Bugs

### A1. Defensive `checked_sub` for `Instant` arithmetic

> **STATUS: ALREADY FIXED** -- commit `f9e5956`, current tip of `main`.
>
> The exact Option C fix recommended below (`Option<Instant>` with a
> `None`-means-never-fired sentinel) was applied before this document
> was finalised. Current `native-theme/src/watch/kde.rs:56` reads
> `let mut last_fire: Option<Instant> = None;` and the debounce check at
> line 66 is `if is_relevant && last_fire.is_none_or(|t| t.elapsed() >= debounce)`.
> The problem description, options table, severity correction, and
> rationale below are retained as a historical record of the reasoning,
> but **no v0.5.7 work is required for A1** -- it is already shipped.
> This item has been removed from the §E priority summary and the §H
> cross-document P0/P2 consolidation.

**File (pre-fix snapshot, for historical reference only):** `native-theme/src/watch/kde.rs:54-56`

```rust
// Allow the first event to fire immediately by setting last_fire far
// in the past.
let mut last_fire = Instant::now() - Duration::from_secs(10);
```

#### Problem

`std::time::Instant` implements `Sub<Duration>`, and Rust's public
documentation says:

> This function may panic if the resulting point in time cannot be
> represented by the underlying data structure. See
> `Instant::checked_sub` for a version without panic.

The crate's "no runtime panics" invariant should anchor on std's
**documented contract**, not on current implementation quirks. By that
standard, the `- Duration::from_secs(10)` is a violation: it uses an
operator documented as "may panic" where a `checked_*` alternative
exists and is explicitly recommended by std.

##### Severity correction

I initially catalogued this as a verified startup panic. On closer
reading of the Rust standard library source, that's overstated for the
platform this file actually compiles on. Correcting the record:

- `watch/kde.rs` is `#[cfg(all(target_os = "linux", feature = "kde"))]`,
  so only the Linux code path matters.
- On Linux, `Instant` is backed by `Timespec { tv_sec: i64, tv_nsec:
  u32 }`. `checked_sub_duration` computes
  `tv_sec.checked_sub(other.as_secs())`; for `tv_sec = 5` (5 seconds
  of uptime) and `other = 10s`, that returns `Some(-5)` -- a valid
  `i64`, just negative. `Timespec::new` validates only that `tv_nsec`
  is in range; it does **not** reject negative `tv_sec`.
  `checked_sub_duration` therefore returns
  `Some(Timespec { tv_sec: -5, ... })`, the outer `Sub::sub` does not
  hit its `.expect(...)` path, and no panic occurs in current Rust.
- `last_fire.elapsed()` on the resulting weird `Instant` produces a
  large `Duration` (approximately uptime-plus-ten-seconds), which is
  exactly what the author of `- Duration::from_secs(10)` wanted:
  "already past the debounce window".

So **on the platform where this code actually runs, the current Rust
std implementation silently produces a valid-if-strange `Instant` and
does not panic.** The accurate label is "latent may-panic per
documented contract", not "verified startup crash".

The fix is still worth making because:

1. Rust **explicitly recommends** `checked_sub` over the `-` operator
   on `Instant`, and the crate's "no panics" invariant should track
   documented contracts, not observed implementation behaviour.
2. A future Rust release could tighten `Timespec::new` or
   `checked_sub_duration` to reject negative `tv_sec`; the latent
   may-panic would become a real one with no source-code change in
   this crate.
3. The pre-release script's panic-detection (regex for `.unwrap()`
   and `.expect(`) cannot see operator-trait panics inside std, so
   any such tightening in Rust std would ship into this crate with
   no alarm.
4. The fix is four lines.

**Severity:** demoted from P0 (verified bug) to P2 (defensive coding
against a documented may-panic contract). Priority summary and
H. Cross-document P0 consolidation updated to reflect the correction.
The "every issue is real and reasonable" promise of this document
holds under the weaker claim; the first-pass claim overreached and is
corrected here.

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Latent panic |
| B | **Use `checked_sub`** and fall back to `Instant::now()` | Minimal change; preserves "fire on first event" intent | The fallback is "just `now()`", which means the first event will NOT fire immediately if it arrives within the 300 ms debounce window after watcher start -- slight behaviour change |
| C | **Represent `last_fire` as `Option<Instant>`**; `None` means "never fired"; the debounce check becomes `last_fire.map_or(true, |t| t.elapsed() >= debounce)` | Removes the magic sentinel entirely; matches the intent directly ("never fired → fire now"); also fixes B's edge case | Slightly longer code at the check site; requires editing the check expression as well |
| D | **Initialise `last_fire` to `Instant::now()` and subtract from first-event time** -- invert the debounce logic so the first event bypasses the check | No panic | Requires invasive restructuring; easy to get wrong |
| E | **Compute `Instant::now() - Duration::from_secs(0)`** (no-op) and use a boolean `first_event: bool` flag | No panic, no restructuring | Two fields for one concept; awkward |

#### Recommended: **C** (`Option<Instant>`)

```rust
let mut last_fire: Option<Instant> = None;
let debounce = Duration::from_millis(300);

loop {
    match rx.recv_timeout(Duration::from_millis(200)) {
        Ok(Ok(event)) => {
            let is_relevant = event.paths.iter().any(|p| ...);
            if is_relevant && last_fire.map_or(true, |t| t.elapsed() >= debounce) {
                last_fire = Some(Instant::now());
                callback(ThemeChangeEvent::ColorSchemeChanged);
            }
        }
        ...
    }
}
```

#### Rationale

Option **A** leaves the may-panic contract in place. Rejected.

Option **B** is the minimum diff but introduces a subtle behaviour
change: immediately after watcher start, if an event arrives within
the debounce window of the fallback `Instant::now()`, the event is
dropped. The existing `- 10s` is specifically a "fire first event
immediately" sentinel; **B** loses this property.

Option **C** is the cleanest: `None` explicitly means "never fired",
the check reads directly as "fire if never, or if debounce elapsed",
and there's no arithmetic on `Instant` at all. One extra `Option`
layer is a rounding error versus the clarity gain.

Option **D** requires restructuring the loop. Too invasive for a
trivial defensive fix.

Option **E** adds a second field that means the same thing as the
`Option` discriminant -- strictly worse than **C**.

**Confidence:** high on the fix direction; severity corrected from
first pass (see the "Severity correction" subsection above). The
fix is ~4 lines; I have read the full watcher loop and the relevant
Rust std source and can commit to it in one pass.

---

### A2. `check_ranges` runs on `T::default()` placeholders, producing spurious errors

**Files:**
- `native-theme/src/resolve/validate_helpers.rs:23-35` (the `require` helper)
- `native-theme/src/resolve/validate.rs:428-458` (orchestration order)
- `native-theme/src/model/widgets/mod.rs:908-1347` (`check_ranges` impls)

#### Problem

The `require` helper substitutes `T::default()` for missing fields and
records the path:

```rust
pub(crate) fn require<T: Clone + Default>(
    field: &Option<T>,
    path: &str,
    missing: &mut Vec<String>,
) -> T {
    match field {
        Some(val) => val.clone(),
        None => {
            missing.push(path.to_string());
            T::default()                          // <-- silent placeholder
        }
    }
}
```

The orchestration in `validate.rs:428-458` runs **range checks on the
placeholder-defaulted values before checking whether `missing` is
non-empty**:

```rust
// validate.rs:429-452 (abbreviated)
window.check_ranges("window", &mut missing);
button.check_ranges("button", &mut missing);
// ... 22 more check_ranges calls ...

if !missing.is_empty() {
    return Err(crate::Error::Resolution(ThemeResolutionError {
        missing_fields: missing,
    }));
}
```

For fields whose `T::default()` falls **inside** the valid range
(e.g. `f32::default() == 0.0` with `check_non_negative`), the check
silently passes -- no spurious error.

For fields whose `T::default()` falls **outside** the valid range, the
check records a spurious error in addition to the missing-field error.
Specifically affected:

- `font.weight: u16` -- `u16::default() == 0`, range `100..=900` →
  `check_range_u16(0, 100, 900)` fires
- `font.size: f32` -- `f32::default() == 0.0`, `check_positive` requires
  `> 0.0` → fires
- `disabled_opacity: f32` -- `0.0..=1.0` range, `0.0` is valid, no
  spurious error
- `text_scale.*.size` and `text_scale.*.weight` -- same pattern

For a theme missing a single font (say, `button.font` is `None`), the
user sees:

```
theme resolution failed: 5 missing field(s):
  [widget fields]
    - button.font
    - button.font.size
    - button.font.weight
    - button.font.size            (again, from check_positive)
    - button.font.weight          (again, from check_range_u16)
```

The user gets five error lines for one root cause. Two of the five are
spurious.

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | User-facing error noise |
| B | **Short-circuit on missing before running `check_ranges`**: check `missing.is_empty()` immediately after construction, return `Err` if not. Range checks only run on fully-validated data. | Zero spurious errors; simpler orchestration | One extra `is_empty()` check; minimal |
| C | **Separate the two error classes** into distinct `Vec`s; only push missing-field paths into `missing` and range errors into a separate `range_errors` vec | Same effect as B plus enables separate error categories (see A3) | Slightly more code; `Error` enum gains another payload |
| D | **Change `require` to return `Result<T, MissingField>` (no placeholder)**. Construction becomes fallible and uses `?` or collects errors via `try_fold`. | No placeholders ever; type-level guarantee | Invasive refactor of every `require` call site (~30 in `validate.rs`, ~24 per-widget functions via the macro) |
| E | **Change `require` to take a closure**: `require(|| ...)` returning `Option<impl FnOnce() -> T>`. Construction only runs when all fields are known present. | Lazy construction; no placeholder materialisation | Complex closure chains; rustdoc awkward |

#### Recommended: **B + C combined**

Restructure `validate.rs` so that:

1. All `require()` calls run, populating `missing`.
2. **Immediately** check `if !missing.is_empty() { return Err(...) }`.
3. Construct the resolved structs (now provably non-null).
4. Run `check_ranges`, writing into a **separate** `Vec<String>`.
5. If `range_errors.is_empty()`, return `Ok`; else return a new
   `Error::ResolutionInvalid` variant (distinct from
   `Error::Resolution`).

Diff sketch:

```rust
// After all require() calls:
if !missing.is_empty() {
    return Err(Error::Resolution(ThemeResolutionError {
        missing_fields: missing,
    }));
}

// Construct, then run range checks on clean data:
let mut range_errors = Vec::new();
window.check_ranges("window", &mut range_errors);
// ... etc ...

if !range_errors.is_empty() {
    return Err(Error::Invalid { range_errors });
}
```

#### Rationale

Option **A** leaves noisy errors. Option **B** alone fixes A2 but
leaves A3 (error conflation) unaddressed. Option **C** alone also works
but requires the `Vec` split.

The combined **B + C** fix is the minimum change that resolves both
A2 and A3 together. It's a local refactor of `validate()` -- ~20 lines
of reordering plus a new `Error` variant (or, cleaner, a new field on
the existing Resolution variant per doc 1's error restructure).

Option **D** is ideal in a type-theoretic sense but requires rewriting
every `require` call site. Too invasive for the scope of v0.5.7 unless
it's bundled with the codegen work in B1.

Option **E** (closures) is elegant but makes the code harder to read
than just reordering the operations.

**Confidence:** high. The diagnostic is clear; the fix is mechanical.

#### Merge-review addendum: the `T::default()` contribution comes from two places

The spurious-error set is amplified by a **second** default choice in
`native-theme/src/model/font.rs:66-70`:

```rust
impl Default for FontSize {
    fn default() -> Self {
        Self::Px(0.0)
    }
}
```

So when `require(&font.size, ...)` falls through to `T::default()`, it
materialises `FontSize::Px(0.0)` -> `0.0` after `to_px(dpi)` -> fails
`check_positive`. This is consistent with the analysis above but makes
the root cause structurally explicit: **two independent defaults (`u16 = 0`
and `FontSize::Px(0.0)`) land outside their validation ranges**. A
separate minor option E' would be to change `FontSize`'s `Default` impl
to sit inside `check_positive`'s legal range (e.g. `Px(1.0)`), but that
is a band-aid that only hides the symptom for one field. B+C is still
the right fix; this addendum simply records the additional root-cause
component so future contributors don't rediscover it.

---

### A3. `ThemeResolutionError::missing_fields` carries two error categories

**File:** `native-theme/src/error.rs:9-12`, `native-theme/src/resolve/validate.rs:429-452`

#### Problem

```rust
pub struct ThemeResolutionError {
    /// Dot-separated paths of fields that remained `None` after resolution.
    pub missing_fields: Vec<String>,
}
```

Doc says *"fields that remained `None`"*. But `validate.rs:429-452`
passes the same `&mut missing` vec into 24 `check_ranges` calls:

```rust
window.check_ranges("window", &mut missing);
button.check_ranges("button", &mut missing);
// ...
```

`check_ranges` can push entries like `"button.font.weight out of range:
got 0"`, which is not a "missing field" -- it's a range violation. The
user receives both kinds under a field called `missing_fields`.

The `Display` impl for `ThemeResolutionError` groups by field name
prefix (`[root defaults]`, `[widget fields]`, etc.) but does not
distinguish missing-vs-invalid. A field that is both missing and
out-of-range appears twice.

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Misnamed field; callers cannot distinguish the two categories programmatically |
| B | **Rename `missing_fields` → `problems` and carry a typed enum**: `Vec<ResolutionProblem>` with `Missing(String)` and `OutOfRange { path: String, value: f32, expected: RangeInclusive<f32> }` | Structured, matchable, typed | Breaking shape; range errors need more data than currently captured |
| C | **Two separate `Vec<String>`s**: `missing_fields` and `range_errors` | Minimal change; preserves stringly-typed errors | Still stringly-typed |
| D | **Fold into doc 1's `Error` restructure**: `Error::ResolutionIncomplete { missing: Vec<FieldPath> }` and `Error::ResolutionInvalid { errors: Vec<RangeViolation> }` as distinct top-level variants | Most expressive; aligns with doc 1's §6 restructure | Needs doc 1 §6 to land first or concurrently |

#### Recommended: **D**, deferring to doc 1 §6

This issue is a special case of doc 1 §6 (Error restructure). In that
document the recommendation is to replace the current `Error::Resolution`
with structured variants. A3 should be handled by:

```rust
#[non_exhaustive]
pub enum Error {
    ...
    /// Resolution left fields unfilled.
    ResolutionIncomplete { missing: Vec<FieldPath> },
    /// Resolution produced values outside their valid ranges.
    ResolutionInvalid { errors: Vec<RangeViolation> },
    ...
}

pub struct RangeViolation {
    pub path: String,
    pub value: f64,
    pub min: Option<f64>,
    pub max: Option<f64>,
}
```

`ThemeResolutionError` is deleted; its display logic moves to the
`Error` impl. Callers can match on the variant and get typed access to
the relevant payload.

#### Rationale

Option **A** leaves the naming inaccurate. Option **B** preserves the
`ThemeResolutionError` wrapper but adds structure; workable but
misses the opportunity to do doc 1 §6 in a coordinated way.

Option **C** is the minimum fix (two `Vec<String>`s) but leaves the
overall error type stringly-typed.

Option **D** is clean and aligns with doc 1's error restructure. The
two issues are linked and should be resolved together. If doc 1 §6 is
deferred, fall back to **C**.

**Confidence:** medium-high. The direction is clear but depends on the
wider error design decided in doc 1.

---

### A4. `resolve()` doc says "no OS detection" but reads `XDG_CURRENT_DESKTOP`

**Files:**
- `native-theme/src/resolve/mod.rs:20-22` (the doc claim)
- `native-theme/src/resolve/inheritance.rs:164-193` (`resolve_safety_nets`)
- `native-theme/src/resolve/inheritance.rs:98-109` (`platform_button_order`)
- `native-theme/src/detect.rs:28-30` (`xdg_current_desktop`)

#### Problem

`resolve/mod.rs:20-22` says:

> This method is a pure data transform: it does not perform any OS
> detection or I/O. For full resolution including platform defaults
> (icon theme from the system), use `resolve_all()`.

But `resolve()` (at `resolve/mod.rs:34`) calls `resolve_safety_nets()`,
which at `inheritance.rs:164-167` contains:

```rust
// dialog.button_order <- platform convention
if self.dialog.button_order.is_none() {
    self.dialog.button_order = Some(platform_button_order());
}
```

And `platform_button_order()` at `inheritance.rs:98-109`:

```rust
fn platform_button_order() -> DialogButtonOrder {
    #[cfg(target_os = "linux")]
    {
        if crate::detect::detect_linux_de(&crate::detect::xdg_current_desktop())
            == crate::detect::LinuxDesktop::Kde
        {
            return DialogButtonOrder::PrimaryLeft;
        }
    }
    DialogButtonOrder::PrimaryRight
}
```

`xdg_current_desktop()` is `std::env::var("XDG_CURRENT_DESKTOP")`. Env
var access is OS interaction.

**Consequences:**

1. Tests asserting resolve-is-pure have to serialize env var state via
   `test_util::ENV_MUTEX`.
2. `resolve()` is not reproducible given only the input theme; it also
   depends on env state.
3. Callers who want deterministic caching of resolve output are
   surprised when the same input produces different results in
   different shells.

This is not merely a doc bug. It's an architectural leak: the
"resolution" layer is documented and designed as pure, but reaches into
the OS-detection layer for a single field.

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Doc lies; architecture leaks |
| B | **Fix the doc**: remove the "no OS detection" claim. Document the env read. | Zero code churn | Accepts the architectural leak; still complicates testing and caching |
| C | **Move `button_order` fallback from `resolve_safety_nets` to `resolve_platform_defaults`** -- the module already exists for OS-adjacent defaults (it already fills `icon_theme`) | Restores the invariant; reuses an existing seam | Slight semantic shift: `button_order` now only gets filled by `resolve_all()`, not by bare `resolve()` -- callers using only `resolve()` see `None` and must handle it |
| D | **Inject platform context**: `resolve(platform: PlatformContext)` takes a struct with `button_order_convention`, `font_dpi`, etc. Pure resolve becomes `resolve(PlatformContext::static_defaults())`; OS-aware resolve passes `PlatformContext::from_os()`. | Most principled; fully decouples from OS | Every call site must pass context; more verbose |
| E | **Drop `platform_button_order` entirely** and require presets to specify `button_order`. Safety nets do not fill it. | Purest | Presets must be updated; live presets must include it; user TOML themes missing `button_order` now fail validation |

#### Recommended: **C** (move to `resolve_platform_defaults`)

`resolve_platform_defaults` already exists (`resolve/mod.rs:52-56`) and
is documented as OS-adjacent. Moving `button_order` there is the
minimum change that restores the documented invariant.

```rust
// resolve/mod.rs:
pub fn resolve_platform_defaults(&mut self) {
    if self.icon_theme.is_none() {
        self.icon_theme = Some(crate::model::icons::system_icon_theme().to_string());
    }
    // new:
    if self.dialog.button_order.is_none() {
        self.dialog.button_order = Some(platform_button_order());
    }
}
```

And remove the `button_order` branch from `resolve_safety_nets`.

#### Rationale

Option **A** leaves the inconsistency. Option **B** (fix the doc) is
the minimum diff but accepts the architectural leak; it means future
maintainers have to remember "resolve is *almost* pure, except for
this one thing".

Option **C** restores purity with ~5 lines of code movement. The
semantic shift (bare `resolve()` no longer fills `button_order`) is a
minor user-visible change that is actually helpful: it forces users
who want OS defaults to opt in explicitly via `resolve_all()` or
`into_resolved()`, which matches the naming convention.

Option **D** is the most principled but requires rewriting every
caller. The v0.5.7 scope is limited and this is too much churn for a
single-field fix.

Option **E** breaks every existing preset. Rejected.

**One consideration for Option C:** if doc 1's §7 recommendation is
adopted (demoting `resolve`, `resolve_all`, `resolve_platform_defaults`
to `pub(crate)` in favor of only `into_resolved`), then this issue
disappears naturally -- `into_resolved` is the only public path and
it's allowed to touch the OS.

**Confidence:** high. Low-risk move.

#### Merge-review addendum: D5 is independent of A4

D5 (below) deletes the KDE reader's hardcoded `button_order = PrimaryLeft`.
A4 moves the resolution-side fallback into `resolve_platform_defaults`.
These are **independently correct**: D5 removes the reader-side
duplication regardless of whether A4's move happens, and A4's move is
correct regardless of whether D5 removes the reader-side hardcode. The
"depends on A4" remark in D5 is too cautious. Ship D5 unconditionally.

---

## B. Structural issues

### B1. ~720 lines of hand-maintained validate/range-check boilerplate

**Files:**
- `native-theme/src/resolve/validate.rs` (280 lines of `require()` calls for defaults extraction)
- `native-theme/src/model/widgets/mod.rs:908-1347` (~450 lines of per-widget `check_ranges` impls)

#### Problem

Every field in `ResolvedThemeDefaults` has a hand-written `require()`
line with a duplicated string literal for the field path:

```rust
let defaults_background = require(
    &self.defaults.background_color,
    "defaults.background_color",
    &mut missing,
);
// repeated ~30 times for defaults
```

Every `Resolved*Theme` has a hand-written `check_ranges` impl:

```rust
impl ResolvedButtonTheme {
    pub(crate) fn check_ranges(&self, prefix: &str, errors: &mut Vec<String>) {
        check_non_negative(self.min_width, &format!("{prefix}.min_width"), errors);
        check_non_negative(self.min_height, &format!("{prefix}.min_height"), errors);
        check_non_negative(self.icon_text_gap, &format!("{prefix}.icon_text_gap"), errors);
        check_range_f32(self.disabled_opacity, 0.0, 1.0, ...);
        check_positive(self.font.size, ...);
        check_range_u16(self.font.weight, 100, 900, ...);
    }
}
// repeated for all 25 widgets
```

Maintenance hazards per field added/renamed/removed:

1. **Path string drift.** Rename `min_width` but forget the string
   literal → error points at a nonexistent field.
2. **Forgotten range check.** New numeric field added; no `check_*`
   call → invalid values pass silently.
3. **Inconsistent rules.** `font.weight` is range-checked on some
   widgets but not others, because each impl was written
   independently. Similar drift for `disabled_opacity`.
4. **Spurious checks survive.** When a field is removed from the
   struct, the `check_*` call is often left behind, dangling.

This is the single biggest argument for a codegen approach.

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | All hazards above |
| B | **Extend the existing `define_widget_pair!` declarative macro** to emit `check_ranges` from a `range_check { field: non_negative, ... }` DSL | No new crate; macro path stays familiar | Declarative macros cannot cross-reference the struct fields they generate; the DSL would be added to the already-complex macro syntax |
| C | **Move to a proc-macro** that reads a single field-definition list and emits both structs + merge + validate + check_ranges + serde + FIELD_NAMES + TOML linter | Single source of truth for all field-related logic | Needs a new `native-theme-derive` crate; migration is significant |
| D | **Drive everything from `property-registry.toml`** via `native-theme-build`. Registry specifies type, required, range, inheritance, serde rename. Build script generates Rust code at compile time. | Most expressive; single source; review-friendly TOML | Highest upfront cost; requires schema design; build pipeline complexity |
| E | **Keep the generation minimal**: only generate `check_ranges` via a small proc-macro; leave `validate()` hand-written for now | Smallest codegen footprint | Doesn't fix the 280-line defaults-extraction boilerplate |
| F | **Delete `check_ranges` entirely** and rely on preset testing to catch invalid values | Less code | No runtime safety net; invalid user TOML slips through validation |

#### Recommended: **D** (registry-driven via `native-theme-build`)

Promote `property-registry.toml` (referenced in CLAUDE memory as the
source of truth) to load-bearing status. Schema sketch:

```toml
[defaults.fields.accent_color]
type = "Rgba"
required = true

[defaults.fields.disabled_opacity]
type = "f32"
required = true
range = { min = 0.0, max = 1.0 }

[defaults.fields.font]
type = "FontSpec"
required = true
nested = true

[widgets.button.fields.min_width]
type = "f32"
required = true
range = { min = 0.0 }
serde_rename = "min_width_px"

[widgets.button.fields.font]
type = "FontSpec"
required = true
inherit_from = "defaults.font"

[widgets.button.fields.disabled_opacity]
type = "f32"
required = true
range = { min = 0.0, max = 1.0 }
```

`native-theme-build` generates:

- Option-field structs (`ButtonTheme`)
- Resolved structs (`ResolvedButtonTheme`) with `#[non_exhaustive]`
- Merge impls (via `impl_merge!` or inline)
- `validate_widget` / `check_ranges` impls
- `FIELD_NAMES` constants
- Lint table for `lint_toml`
- Optionally: inheritance logic (see B2)

The ~720 lines of hand-maintained boilerplate become `include!` of
generated code. Adding a field is one edit in the registry.

#### Rationale

Option **A** is the status quo and the hazards are real. The crate is
currently adding ~2-3 widgets per release, and each addition requires
touches across 5+ places.

Option **B** (extend the declarative macro) hits a wall: declarative
macros cannot dynamically enumerate or cross-reference fields, and the
existing macro is already at the limit of readability.

Option **C** (bespoke proc-macro reading attribute DSL) is a valid
middle ground, but duplicates information that `property-registry.toml`
already captures. Two sources of truth are worse than one.

Option **D** (registry-driven) aligns with the existing intention in
CLAUDE memory and eliminates the drift risk permanently. The cost is
upfront: designing the schema and writing the build script. Once done,
every field change is a single TOML edit.

Option **E** is a half-measure that leaves the defaults boilerplate.

Option **F** (delete check_ranges) is tempting for the simplicity, but
it removes a runtime safety net. Theme authors rely on validation to
catch bad TOML values; removing it would push the failure to visual
inspection, which is worse. Rejected.

**Confidence:** medium. The direction is clearly right. The upfront
design cost (schema, build pipeline, codegen edge cases) is non-trivial
and should be staged. If v0.5.7 cannot absorb the full migration,
**E** is an acceptable partial step.

**Flag for §F:** this is the same recommendation as doc 1 §2 (doubled
struct hierarchy). Both should be solved by the same codegen pipeline,
not two different mechanisms.

---

### B2. Inheritance rules duplicated between TOML source-of-truth and Rust implementation

**Files:**
- `docs/inheritance-rules.toml` (declared rules, ~100+ lines)
- `native-theme/src/resolve/inheritance.rs` (Rust implementation)

#### Problem

The TOML file opens with:

```
# Source: docs/platform-facts.md  (2026-04-06 snapshot)
# Implementation: native-theme/src/resolve.rs
```

It then declares rules like:

```toml
[defaults_internal]
"defaults.selection_background"          = "defaults.accent_color"
"defaults.focus_ring_color"              = "defaults.accent_color"
...
```

Which are reimplemented by hand in `inheritance.rs:114-160`:

```rust
if d.selection_background.is_none() {
    d.selection_background = d.accent_color;
}
if d.focus_ring_color.is_none() {
    d.focus_ring_color = d.accent_color;
}
// ...
```

Nothing enforces that the TOML and the Rust agree. A contributor who
updates one without the other produces a silent drift.

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Drift risk |
| B | **Delete the TOML as "documentation only"** | Single source (Rust) | Loses review-friendly rule spec; harder to audit from platform-facts |
| C | **Generate the Rust from the TOML** via `native-theme-build` (same mechanism as B1) | Single source (TOML); aligns with B1 | Needs schema design covering all rule categories |
| D | **Keep both; add a verification test** that parses the TOML and greps the Rust for each rule, failing the build on drift | Cheapest; safety net | Only catches structural presence, not semantic equivalence |
| E | **Keep both; run the Rust through TOML-generated snapshot tests** that assert "given theme X, resolve() produces expected field Y" per rule | Tests semantic equivalence | Many tests to write; slow test suite |

#### Recommended: **C**, bundled with B1

The TOML already has a pattern-match structure
(`[defaults_internal]`, `[border_inheritance]`, `[font_inheritance]`,
`[uniform]`) that is close to a code-gen DSL. Extending the
`property-registry.toml` schema from B1 to include inheritance rules
closes the drift gap in one shot.

Concretely, fold the TOML categories into the registry:

```toml
[inheritance.defaults_internal]
"defaults.selection_background" = "defaults.accent_color"
"defaults.focus_ring_color" = "defaults.accent_color"
# ...

[inheritance.border]
widgets = ["window", "button", "input", ...]
rule = "full"   # full / partial / none

[inheritance.font]
widgets = ["button.font", "input.font", ...]
source = "defaults.font"
```

Generate `inheritance.rs` from the registry.

#### Rationale

Option **A** leaves the drift. Option **B** deletes a review-friendly
document that non-Rust stakeholders (designers, platform auditors)
can read. Option **D** only catches omission; a rule that is present
in both but semantically different is not caught. Option **E** is
expensive and slow.

Option **C** aligns with B1 and solves both problems at once. If B1
is accepted, B2 is nearly free additional code on the same machinery.

**Confidence:** medium. Same conditionality as B1: the full schema
design is nontrivial.

---

### B3. Platform readers have a heterogeneous variant contract (single vs dual)

**Files:**
- `native-theme/src/kde/mod.rs:61-75` (single-variant output)
- `native-theme/src/macos.rs:397-499` (dual-variant output)
- `native-theme/src/windows.rs:578+` (single-variant, not re-read but pattern matches)
- `native-theme/src/pipeline.rs:49-59` (pipeline branches on which variant is populated)
- `native-theme/src/pipeline.rs:281-284` (`reader_is_dark` heuristic)

#### Problem

`from_kde_content_pure` sets either `light` or `dark` but not both:

```rust
let theme = if dark {
    ThemeSpec { name, light: None, dark: Some(variant), layout: ... }
} else {
    ThemeSpec { name, light: Some(variant), dark: None, layout: ... }
};
```

`from_macos` (verified at `macos.rs:397-480`) reads both NSAppearance
variants and populates both `theme.light` and `theme.dark`.

`run_pipeline` at `pipeline.rs:49-59` then has to branch:

```rust
let mut light_variant = if reader_output.light.is_some() {
    merged.light.unwrap_or_default()
} else {
    full_preset.light.unwrap_or_default()
};
```

And there's a `reader_is_dark` helper at `pipeline.rs:282-284` that
infers dark-mode by checking *"only dark populated, light is None"* --
which is fragile for macOS (which populates both).

**Consequences:**

1. The reader contract is implicit. A new reader author has to read
   existing readers to figure out what shape to return.
2. The pipeline reasons about "which variant did the reader supply" --
   a question that should not exist.
3. The `reader_is_dark` heuristic is specific to single-variant readers
   and is not used for macOS.

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Implicit contract; fragile pipeline branching |
| B | **Standardise on "readers always populate both variants"**: single-variant readers (KDE, Windows) fill the inactive variant from the platform preset internally | Uniform contract; pipeline simplifies | Duplicates preset-merging logic in readers; readers gain a dependency on presets |
| C | **Standardise on `ReaderOutput { active: ThemeLayer, inactive: Option<ThemeLayer>, is_dark: bool }`** as a typed return type distinct from `ThemeSpec` | Explicit; typed "which variant is live" via `is_dark` | New type; pipeline transforms `ReaderOutput → ThemeSpec` in one place |
| D | **Standardise on "readers return only the active variant + `is_dark` flag"**: `fn from_kde() -> (ThemeLayer, bool)`. Pipeline fills the inactive side from preset. | Minimal reader contract; simplest | Loses macOS's ability to provide both; macOS reader must drop its dark-variant read |
| E | **Keep both shapes; wrap in an enum**: `pub enum ReaderOutput { Active { variant: ThemeLayer, is_dark: bool }, Both { light, dark } }`. Pipeline matches on the variant. | Both shapes preserved; explicit | More complexity than **C** |
| F | **`ReaderOutput` with trilean `known_is_dark: Option<bool>`** (added in merge review). `Some(true)` when the reader knows the OS is dark (KDE via kdeglobals, macOS via NSAppearance), `Some(false)` when it knows the OS is light, `None` when it cannot tell (portal-only readers, some fallbacks). The pipeline falls back to `detect::system_is_dark()` when the reader returns `None`. | Makes "reader couldn't determine" a typed state; avoids the fragile `reader.dark.is_some() && reader.light.is_none()` heuristic that doc 1 §4's staleness trap feeds on; single path for all readers | One more field on `ReaderOutput` than C; callers must handle the `None` branch via an explicit fallback call |

#### Recommended: **F** (strict superset of **C**)

```rust
pub struct ReaderOutput {
    /// Theme name from the reader (e.g. "Breeze Dark").
    pub name: String,
    /// The OS-active variant.
    pub active: ThemeLayer,
    /// The inactive variant, if the reader could obtain it (macOS always; KDE/Windows never).
    pub inactive: Option<ThemeLayer>,
    /// Whether `active` is the dark variant.
    pub is_dark: bool,
}

pub(crate) fn run_pipeline(
    reader: ReaderOutput,
    preset_name: &str,
) -> Result<SystemTheme> { ... }
```

Each reader returns a `ReaderOutput`. The pipeline uses `is_dark` as
the single source of truth and always knows which variant is active.
No more heuristics, no more branching on "which variant is
populated".

#### Rationale

Option **A** leaves the implicit contract. Option **B** (always
dual-variant) pushes preset knowledge into readers; readers should be
"OS-to-spec translators", not "OS-to-spec + preset-merger".

Option **D** (always single-variant) loses macOS's genuine dual-
variant capability. macOS can read both appearances simultaneously via
`performAsCurrentDrawingAppearance`; throwing that away is a
regression.

Option **E** (enum wrapper) supports both shapes but keeps the
branching in the pipeline -- just pushes it into a `match` statement
instead of `is_some()` checks. Slightly clearer but more complex than
**C**.

Option **C** explicitly distinguishes "active variant" from "optional
inactive variant" and carries `is_dark` as typed data. Readers that
can only get one variant return `inactive: None`. Readers that get
both return `inactive: Some(...)`. The pipeline always knows which
variant is live without inference.

Option **F** is a strict superset of **C**: it replaces the boolean
`is_dark: bool` with `known_is_dark: Option<bool>`. This matters
because today's `reader_is_dark` helper at `pipeline.rs:282-284`
(`reader.dark.is_some() && reader.light.is_none()`) is a lossy
inference. macOS populates both variants, so the helper returns
`false` regardless of what the OS actually reports -- `from_system_inner`
at `pipeline.rs:331` works around this by calling the helper AND the
real `detect::system_is_dark()`. Option C's `is_dark: bool` hard-codes
a single answer and forces every reader to provide it even when it
cannot. Option F makes "cannot determine" a first-class state, which
matches the truth and lets the pipeline apply its own fallback once,
in one place.

The cost of F over C is tiny (one extra match arm in the pipeline).
The correctness win is meaningful: the gpui watcher and portal-only
readers today cannot reliably report is_dark, and C would force them
to lie. F lets them return `None` honestly.

**Confidence:** medium-high. The shape is right; the migration is
mechanical but touches every reader.

---

### B4. `ThemeDefaults` mixes static theme data with accessibility preferences

**Files:**
- `native-theme/src/model/defaults.rs:131-140` (`ThemeDefaults`)
- `native-theme/src/model/resolved.rs:146-159` (`ResolvedThemeDefaults`)

#### Problem

`ThemeDefaults` has four accessibility fields:

```rust
// ---- Accessibility ----
pub text_scaling_factor: Option<f32>,
pub reduce_motion: Option<bool>,
pub high_contrast: Option<bool>,
pub reduce_transparency: Option<bool>,
```

These are **user preferences from the OS accessibility layer**, not
theme data. Storing them inside the theme creates four problems:

1. **TOML presets can override them.** A theme file can set
   `reduce_motion = true` regardless of what the user actually wants.
   That's wrong: reduce_motion is a user setting, not a theme choice.
2. **They live on every variant.** Light and dark variants each carry
   their own copy. A change in user preference must flow to both.
3. **`ResolvedThemeVariant` becomes equality-non-deterministic.** Two
   resolved themes compare equal only if the OS accessibility state
   matches. If the user toggles "reduce motion" between loads,
   equality breaks.
4. **Readers must detect accessibility state** as part of theme
   reading -- an orthogonal concern that bloats reader code
   (`macos.rs:492-499`).

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | All four problems above |
| B | **Move accessibility to a separate `AccessibilityPreferences` struct** held by `SystemTheme` (or `DetectedTheme` per doc 1) | Clean separation; one copy instead of two; `ResolvedTheme` becomes pure theme data | Callers who read from `sys.defaults` must migrate to `sys.accessibility` |
| C | **Keep accessibility in `ThemeDefaults` but add a second, OS-only detection path** that overwrites theme values at read time | Preserves TOML round-trip | Two sources of truth for accessibility; user overrides unclear |
| D | **Remove accessibility fields entirely**; callers call `prefers_reduced_motion()` / etc. directly | Simplest; no theme pollution | Convenient access point through `sys.defaults.reduce_motion` lost; connectors must plumb accessibility separately |
| E | **Move accessibility to its own parallel struct inside `SystemTheme`**, provide a `sys.accessibility: AccessibilityPreferences` field, and deprecate the `defaults.reduce_motion` path | Best of both | Transition cost; but doc 1 accepts breaking changes so this is free |

#### Recommended: **B** (or **E**, same outcome)

```rust
pub struct AccessibilityPreferences {
    pub text_scaling_factor: f32,
    pub reduce_motion: bool,
    pub high_contrast: bool,
    pub reduce_transparency: bool,
}

impl AccessibilityPreferences {
    /// Read current accessibility preferences from the OS.
    pub fn from_system() -> Self { ... }
}

pub struct SystemTheme {
    pub name: String,
    pub is_dark: bool,
    pub light: ResolvedTheme,
    pub dark: ResolvedTheme,
    pub accessibility: AccessibilityPreferences,  // single copy
    pub preset: String,
}
```

`ResolvedThemeDefaults` loses the four accessibility fields. Readers
stop filling them. TOML presets can no longer override them.

#### Rationale

Option **A** leaves the conflation. Option **C** tries to have it both
ways -- theme presets can set accessibility, and OS detection
overwrites -- but introduces precedence ambiguity ("which wins when
both are set?").

Option **D** removes the convenient aggregate access, which is
genuinely useful in connectors (one struct has everything needed for
rendering). Rejected.

Options **B** and **E** are the same proposal phrased differently.
They provide a single `sys.accessibility` field, separate from theme
data, with a dedicated `from_system()` detection entry point.

**Confidence:** high. The conflation is architecturally wrong; the
fix is clean.

---

### B5. `font_dpi` on `ThemeDefaults` mixes runtime and static data

**File:** `native-theme/src/model/defaults.rs:118-129`

#### Problem

```rust
/// Font DPI for pt-to-px conversion. ...
/// This is a **runtime** value -- not stored in TOML presets. OS readers
/// auto-detect it from system settings. ...
#[serde(skip)]
pub font_dpi: Option<f32>,
```

The doc acknowledges the issue: *"This is a runtime value -- not
stored in TOML presets."* But the field lives in `ThemeDefaults`
alongside `accent_color` and `background_color` -- the static theme
data.

Consequences:

1. Two `ThemeDefaults` can be bitwise identical in TOML but compare
   unequal after DPI detection, breaking `PartialEq` determinism.
2. `#[serde(skip)]` means the field silently disappears from TOML
   round-trips. Users cloning a theme, serializing, deserializing lose
   DPI.
3. `into_resolved` auto-detects DPI if unset (`resolve/mod.rs:102-104`),
   which is an OS call hidden inside a method that looks like a data
   transform.

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Hidden OS call; non-determinism |
| B | **Move `font_dpi` to a resolution-time parameter**: `Theme::resolve(dpi: f32) -> ResolvedTheme`. Helper `Theme::resolve_with_os_dpi() -> ResolvedTheme` wraps it with OS detection. | Explicit; reproducible; no hidden data on theme data | Every resolve call site must be updated to pass DPI |
| C | **Move `font_dpi` to `AccessibilityPreferences`** (from B4) since both are OS-detected runtime state | Bundles OS state into one struct | DPI is not really "accessibility"; name is misleading |
| D | **Introduce a `ResolutionContext`** struct with `font_dpi`, `icon_theme`, `button_order`, etc.; pass to `resolve()` | Most expressive; bundles all OS-adjacent resolution parameters | New type; more explicit plumbing |
| E | **Keep `font_dpi` in `ThemeDefaults` but make it mandatory (not `Option`)** and require callers to set it before calling resolve | Simplifies validation | Every call site must set it or use a helper; breaks existing TOML round-trip |

#### Recommended: **D** (`ResolutionContext`)

```rust
pub struct ResolutionContext {
    pub font_dpi: f32,
    pub icon_theme: String,
    pub dialog_button_order: DialogButtonOrder,
    pub accessibility: AccessibilityPreferences,  // from B4
}

impl ResolutionContext {
    pub fn from_system() -> Self { ... }
    pub fn default_96dpi() -> Self { ... }
}

impl Theme {
    pub fn resolve(self, ctx: &ResolutionContext) -> Result<ResolvedTheme>;

    /// Convenience: resolve with system-detected context.
    pub fn resolve_system(self) -> Result<ResolvedTheme> {
        self.resolve(&ResolutionContext::from_system())
    }
}
```

`ThemeDefaults` and `ResolvedThemeDefaults` lose `font_dpi` entirely.
The resolved DPI can still appear in the resolved theme (if callers
care), but as a direct field of `ResolvedTheme` rather than nested in
defaults.

#### Rationale

Option **A** leaves the hybrid. Option **B** (DPI as resolve param)
fixes font_dpi but leaves icon_theme and button_order (A4) in the
same state.

Option **C** (bundle into accessibility) is wrong categorically: DPI
is not accessibility.

Option **D** bundles all runtime-resolved OS context into a single
struct. This is architecturally clean: "here are all the things we
needed from the OS to resolve this theme". A4 (button_order leak) and
B5 (font_dpi hybrid) both dissolve. Callers that want full OS-aware
resolution use `resolve_system()`; callers that want pure /
deterministic resolution (tests, caching, snapshot fixtures) use
`resolve(&ctx)` with an explicitly constructed context.

Option **E** forces every caller to set DPI but doesn't solve the
other hybrid fields.

**Confidence:** medium-high. Direction is clear. The exact fields of
`ResolutionContext` need a round of discussion; I've listed the
obvious candidates.

#### Merge-review refinement: use `Option<&ResolutionContext>` at the API boundary

As drafted, `Theme::resolve(self, ctx: &ResolutionContext)` forces
every caller -- including tests, doctests, examples, and snapshot
fixtures -- to construct a context before they can resolve a theme.
That's a large amount of touch churn across the tree. A better shape:

```rust
impl Theme {
    /// Resolve with explicit context, or auto-detect when `None`.
    pub fn resolve(self, ctx: Option<&ResolutionContext>) -> Result<ResolvedTheme>;
    /// Convenience alias for `resolve(None)`.
    pub fn resolve_system(self) -> Result<ResolvedTheme> { self.resolve(None) }
}
```

`Option<&ResolutionContext>` has two payoffs the mandatory form lacks:

1. Existing call sites that today rely on auto-detection inside
   `into_resolved` stay mostly unchanged -- they pass `None`.
2. Tests that want to pin DPI + button_order for determinism pass
   `Some(&fixture_ctx)` and get a fully-reproducible resolve.

This is functionally equivalent to the doc's `resolve` + `resolve_system`
pair, but collapses two methods into one that can be called either way.
The choice between "two methods with distinct names" (doc's form) and
"one method with `Option<&ctx>`" (merge refinement) is taste-level --
the latter is more Rustacean, the former is arguably more discoverable
in rustdoc. I marginally prefer the single-method form, but will not
fight hard if the two-method form is chosen.

---

### B6. `BorderSpec` allows defaults-only fields at widget level

**File:** `native-theme/src/model/border.rs:13-35`

#### Problem

```rust
pub struct BorderSpec {
    pub color: Option<Rgba>,
    pub corner_radius: Option<f32>,
    /// Large corner radius in logical pixels (defaults only).
    pub corner_radius_lg: Option<f32>,
    pub line_width: Option<f32>,
    /// Border alpha multiplier 0.0–1.0 (defaults only).
    pub opacity: Option<f32>,
    pub shadow_enabled: Option<bool>,
    pub padding_horizontal: Option<f32>,
    pub padding_vertical: Option<f32>,
}
```

Two fields are documented as "defaults only" in inline comments, but
the type allows them at the widget level. A TOML author can write:

```toml
[light.button.border]
opacity = 0.5                # silently ignored
corner_radius_lg_px = 20.0   # silently ignored
```

Neither `lint_toml` nor the serde layer flags it. Resolution silently
discards the values.

Separately, `padding_horizontal` / `padding_vertical` exist at the
defaults level but have no semantic meaning there (the defaults-level
border is never rendered). The result is a bizarre inheritance rule
(see D2) that fills padding with `0.0` sometimes.

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Type-vs-usage mismatch; silent field discard |
| B | **Split into two types**: `DefaultsBorderSpec` (no padding) and `WidgetBorderSpec` (no corner_radius_lg, no opacity) | Type system enforces the intent; lint catches mistakes | New types; migration work across presets, readers, validate, resolve |
| C | **Keep one `BorderSpec`** but add a `lint_toml` check for defaults-only fields at widget level | Minimum code change | Runtime-only guard; users still construct bad values programmatically |
| D | **Deny unused fields at validation time**: validate() errors if a widget-level border has `opacity` or `corner_radius_lg` set | Runtime guard; no type change | Validate becomes chatty; legitimate debugging may trip it |
| E | **Make `corner_radius_lg` and `opacity` effective at widget level too** | Type is honest | Changes semantics; presets must be updated; unclear what "widget opacity" means if it's not already a thing |

#### Recommended: **B** (split into two types)

```rust
pub struct DefaultsBorderSpec {
    pub color: Option<Rgba>,
    pub corner_radius: Option<f32>,
    pub corner_radius_lg: Option<f32>,
    pub line_width: Option<f32>,
    pub opacity: Option<f32>,
    pub shadow_enabled: Option<bool>,
    // No padding -- not meaningful at defaults level
}

pub struct WidgetBorderSpec {
    pub color: Option<Rgba>,
    pub corner_radius: Option<f32>,
    pub line_width: Option<f32>,
    pub shadow_enabled: Option<bool>,
    pub padding_horizontal: Option<f32>,
    pub padding_vertical: Option<f32>,
    // No corner_radius_lg -- defaults only
    // No opacity -- defaults only
}
```

`ThemeDefaults::border: DefaultsBorderSpec`, widget
`border: Option<WidgetBorderSpec>`.

#### Rationale

Option **A** leaves the silent-discard bug. Option **C** (lint
runtime) is a weaker band-aid; users reading the `BorderSpec` doc
still think they can set `opacity` on a button.

Option **D** (validate chatty) is functional but produces a new class
of validation error that users must handle.

Option **E** would change semantics. "Widget opacity" would need to
be defined -- currently there's no concept of per-widget alpha --
and the resolution rules would need updating. Too much scope for a
type cleanup.

Option **B** is the type-safe fix. The two types can share a common
trait if useful, but the distinct fields prevent invalid states at
the type level.

**Confidence:** high.

#### Merge-review note: coordinate with B1/B2 codegen

If B1/B2's registry-driven codegen lands, the two border-spec types
should be generated from the same registry entry using a `kind`
discriminator (`defaults` vs `widget`) rather than hand-written. This
avoids the "now we have two types to keep in sync for every new
border field" maintenance burden. Practically: define `border_fields`
once in `property-registry.toml` with a `level = "defaults"|"widget"`
tag per field, and have the generator emit `DefaultsBorderSpec` and
`WidgetBorderSpec` as two views over that one field list. Defer the
hand-written B split until the codegen approach is decided so we
don't do the work twice.

---

### B7. Three parallel border-inheritance validation paths

**Files:**
- `native-theme/src/resolve/validate_helpers.rs:131` (`require_border`)
- `native-theme/src/resolve/validate_helpers.rs:172` (`border_all_optional`)
- `native-theme/src/resolve/validate_helpers.rs:190` (`require_border_partial`)
- `native-theme/src/model/widgets/mod.rs:48-156` (the macro dispatches between them via `optional_nested` / `border_partial` / `border_optional` clauses)

#### Problem

Three validation functions for borders, each used by a different
subset of widgets:

| Function | Widgets | Behavior |
|---|---|---|
| `require_border` | 13 widgets | All 4 inherited sub-fields (color, corner_radius, line_width, shadow_enabled) must be present |
| `border_all_optional` | `menu`, `tab`, `card` | All sub-fields optional; no errors recorded |
| `require_border_partial` | `sidebar`, `status_bar` | Only `color` + `line_width` required |

Each widget's macro invocation picks one by clause name
(`optional_nested` / `border_optional` / `border_partial`). A
contributor adding a new widget must know which clause to use, and
the choice has no visible justification in the invocation.

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Three hand-coded paths with hidden decision criteria |
| B | **Unify to one `validate_border(kind: BorderInheritance)`** function that dispatches on a typed enum parameter | Single code path; explicit classification | Still hand-classified per widget |
| C | **Drive from `property-registry.toml`**: each widget declares its `border_kind = "full" / "partial" / "none"` in the registry; codegen emits the right validation path | Single source of truth; aligns with B1/B2 | Needs the codegen infrastructure |
| D | **Delete `border_all_optional` and require all widgets to have borders fully optional at the preset level** (i.e. drop the inheritance for menu/tab/card) | Fewer categories | Changes resolution semantics; existing presets may need updates |
| E | **Eliminate `require_border_partial` by making it a degenerate case of `require_border`** with an empty "required fields" list | Fewer functions | Still needs the classification |

#### Recommended: **C** (registry-driven), bundled with B1/B2

Add `border_kind` to the per-widget registry entry:

```toml
[widgets.button]
border_kind = "full"

[widgets.menu]
border_kind = "none"

[widgets.sidebar]
border_kind = "partial"
```

Codegen emits the appropriate validation path. The three hand-coded
functions collapse into one generated dispatch.

#### Rationale

Option **A** leaves the three-way decision hand-maintained.

Option **B** collapses to one function with a typed parameter -- an
improvement -- but still requires the contributor to pick the right
kind manually at each call site.

Option **C** puts the classification in the registry alongside the
field definitions. Adding a widget is one registry edit; codegen
handles the rest.

Option **D** is tempting (fewer cases) but would change resolution
semantics for menu/tab/card. Whether that's acceptable is a design
question that needs a separate audit.

Option **E** is a code cleanup without solving the underlying "how
does a contributor know which to pick" problem.

**Confidence:** medium. Same conditionality as B1.

---

## C. API-shape issues

### C1. `ThemeChangeEvent::Other` is defined but never emitted

**Files:**
- `native-theme/src/watch/mod.rs:65-70` (definition)
- `native-theme/src/watch/kde.rs:68` (emits `ColorSchemeChanged`)
- `native-theme/src/watch/gnome.rs:61` (emits `ColorSchemeChanged`)
- `native-theme/src/watch/macos.rs:95` (emits `ColorSchemeChanged`)
- `native-theme/src/watch/windows.rs` (emits `ColorSchemeChanged`; not re-read but same pattern)

#### Problem

```rust
#[non_exhaustive]
pub enum ThemeChangeEvent {
    ColorSchemeChanged,
    Other,
}
```

A grep for `ThemeChangeEvent::Other` in production code returns zero
hits. It appears only in the test at `watch/mod.rs:252`:

```rust
assert_ne!(
    ThemeChangeEvent::ColorSchemeChanged,
    ThemeChangeEvent::Other
);
```

Each watcher backend fires only `ColorSchemeChanged`. `Other` is a
placeholder for hypothetical future use, but `#[non_exhaustive]`
already preserves that freedom -- new variants can be added
non-breakingly. `Other` is dead code occupying API surface.

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Dead variant in public API |
| B | **Remove `Other`** | Smaller API surface; `#[non_exhaustive]` preserves forward compat | Tests using it must be updated (trivial) |
| C | **Rename `Other` to `Unknown`** and document it as a placeholder | Preserves the API shape | Same problem under a different name |
| D | **Actually emit `Other`** from backends for non-color changes (file-watcher sees a kdeglobals change but can't tell what) | Makes the variant meaningful | Would require distinguishing color changes from other kdeglobals changes, which the current file watcher cannot do |

#### Recommended: **B** (remove)

Delete the variant. The `#[non_exhaustive]` attribute on the enum
allows adding new variants in the future without breaking callers.
There is no reason to carry a placeholder.

#### Rationale

Option **A** leaves dead code. Option **C** is cosmetic. Option **D**
conflicts with C2 (see below): backends currently fire
`ColorSchemeChanged` for all kdeglobals changes, which is already a
misnomer. Adding `Other` as "non-color change" would require splitting
the existing behavior, which is a larger design decision.

Option **B** (delete) is the correct minimum change. If future
backends need a new event class, they add it at that time, guarded by
`#[non_exhaustive]`.

**Confidence:** high.

---

### C2. `ColorSchemeChanged` is platform-inaccurate on Linux

**Files:**
- `native-theme/src/watch/mod.rs:63-70` (definition and doc)
- `native-theme/src/watch/kde.rs:66-69` (KDE emission)
- `native-theme/src/watch/gnome.rs:42-62` (GNOME emission via portal)
- `native-theme/src/watch/macos.rs:88-96` (macOS emission via NSDistributedNotificationCenter)

#### Problem

The event is named `ColorSchemeChanged`. The truth per platform:

- **macOS:** Fires only on `AppleInterfaceThemeChangedNotification`,
  which Apple documents as the light/dark appearance toggle. **Name
  is accurate.**
- **KDE:** Fires whenever `kdeglobals` or `kcmfontsrc` is modified.
  That includes color scheme switches, but also font changes, icon
  theme changes, and many other settings. **Name is inaccurate.**
- **GNOME:** Fires on the `org.freedesktop.appearance` portal
  namespace, which per the spec includes `color-scheme`,
  `accent-color`, and `contrast`. A user who changes accent color
  without toggling light/dark gets `ColorSchemeChanged`. **Name is
  inaccurate.**
- **Windows:** (code not re-read; presumed to fire on `UISettings`
  color value change, which is similar to KDE -- any color value
  change fires).

So the name is accurate only on macOS. On every other platform it
misrepresents what triggered it.

This was not in the first critique and deserves its own entry.

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Misleading on 3 of 4 platforms |
| B | **Rename to `ThemeMayHaveChanged`** or just `Changed`. The payload is "re-read the theme", no more specific than that. | Accurate on all platforms | Loses the "hint what changed" intent |
| C | **Keep `ColorSchemeChanged` but add `AccentChanged`, `ContrastChanged`, `FontsChanged`, etc.** as new variants (taking advantage of `non_exhaustive`) and have each backend fire the narrowest applicable variant | Maximally informative | Requires distinguishing the trigger per backend, which KDE's file watcher cannot do without re-reading and diffing the config |
| D | **Fire both**: the current notification is the coarse `ThemeMayHaveChanged`, and fine-grained variants are added over time as backends grow the capability | Migration path | Two events per real change if not careful |
| E | **Drop the payload entirely**: change the callback signature from `Fn(ThemeChangeEvent)` to `Fn()`. The callback is a "something changed" pulse, nothing more. | Simplest; honest | Loses future extensibility; may want specific payloads later |
| F | **Keep `ColorSchemeChanged` but add a `source: EventSource` field** (added in merge review) that records the backend's precision: `EventSource::KdeColorsOrFonts`, `EventSource::GnomeAppearancePortal`, `EventSource::MacOsInterfaceTheme`, `EventSource::WindowsUIColor`. Backends emit `ColorSchemeChanged { source: ... }` and callers who care can match on the source. | macOS name stays accurate for the platform that can tell; callers who want "did colors *actually* change?" can filter out `KdeColorsOrFonts` and re-read only on `MacOsInterfaceTheme` | Adds a payload field; still a single variant so the name covers the union |

#### Recommended: **B** (rename to `Changed`)

```rust
#[non_exhaustive]
pub enum ThemeChangeEvent {
    /// The theme may have changed. Re-run theme detection.
    Changed,
}
```

Combined with C1's removal of `Other`, the enum becomes a single-
variant placeholder. Keep `#[non_exhaustive]` for future specific
variants.

#### Rationale

Option **A** leaves the misleading name. Option **C** requires
capability that the current backends do not have (KDE's `notify`
watcher sees file-level events, not per-key changes).

Option **D** (fire both) creates double-delivery if implementations
miss the mark.

Option **E** (drop the payload) is clean but forecloses on future
use cases where a specific event type would be helpful (e.g.,
"user just changed accent color, re-derive accent-dependent fields
but skip geometry re-detection").

Option **B** is the minimum honest change. The name stops lying. The
payload is preserved as an enum so future variants can be added
via `non_exhaustive`.

**A refinement worth considering:** on macOS, where the name was
genuinely accurate, future versions could add a narrower
`ColorSchemeChanged` variant and have the macOS backend emit it
specifically. The current `Changed` variant would remain as the
fallback for less-precise backends. But that's a refinement, not a
blocker for v0.5.7.

**Option F weighed against B:** F preserves the informative intent
of the original name (the macOS backend can still say
"ColorSchemeChanged") while tagging backends that cannot actually
tell. The objection to F is that it leaves the variant name
misleading for the KDE and GNOME cases even if `source` is wired
up -- users who `match ThemeChangeEvent::ColorSchemeChanged` without
checking `source` still get false-positives. Option B renames the
variant to something accurate for all cases and is the safer default.
Keep **B** as the P0 fix. F can be added later as an additive
refinement without breaking B.

**Confidence:** high.

---

### C3. `AnimatedIcon` public fields allow invalid construction

**File:** `native-theme/src/model/animated.rs:57-104`

#### Problem

```rust
#[non_exhaustive]
pub enum AnimatedIcon {
    Frames {
        pub frames: Vec<IconData>,
        pub frame_duration_ms: u32,
    },
    Transform {
        pub icon: IconData,
        pub animation: TransformAnimation,
    },
}

impl AnimatedIcon {
    pub fn new_frames(frames: Vec<IconData>, frame_duration_ms: u32) -> Option<Self> {
        if frames.is_empty() || frame_duration_ms == 0 {
            return None;
        }
        Some(AnimatedIcon::Frames { frames, frame_duration_ms })
    }
}
```

Two problems:

1. **`new_frames` is validated**; it rejects empty frames and zero
   duration. But the struct literal `AnimatedIcon::Frames { frames:
   vec![], frame_duration_ms: 0 }` is a legal expression because the
   fields are `pub`. Bypasses validation.
2. **`AnimatedIcon::Transform` has no constructor.** Users can
   directly construct `Transform { icon, animation: Spin { duration_ms:
   0 } }`, which is a zero-period spin (undefined in the playback
   code).

`first_frame()` gracefully handles the empty case by returning `None`
for empty Frames, but downstream rendering code is not expected to
handle "animation with zero-period spin" or "frames with zero
duration".

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Invalid states are constructible |
| B | **Make fields private**; force construction through `new_frames` / `new_transform` constructors | Type-level validity | Breaks struct-literal construction; users must match on private fields via accessors |
| C | **Use typed newtype wrappers**: `NonEmptyFrames` for the frame vec, `NonZeroU32` for duration | Type-level validity via standard Rust types | New types; migration cost |
| D | **Keep fields public but use `#[non_exhaustive]` on the struct variants**: `Frames { frames, frame_duration_ms, _private: () }`. Requires a constructor call. | Minimal type surface change | `_private: ()` is an anti-pattern |
| E | **Document the invariant and add a `validate()` method**; runtime safety net | Zero breaking change | Invariant not enforced; users still construct invalid values |

#### Recommended: **C** (typed newtype wrappers)

```rust
use std::num::NonZeroU32;

pub struct FrameList(Vec<IconData>);  // non-empty invariant

impl FrameList {
    pub fn new(v: Vec<IconData>) -> Option<Self> {
        (!v.is_empty()).then_some(Self(v))
    }
    pub fn as_slice(&self) -> &[IconData] { &self.0 }
    pub fn first(&self) -> &IconData {
        // SAFETY/invariant: FrameList is never empty
        &self.0[0]  // but this is a bounds check, not unsafe
    }
}

#[non_exhaustive]
pub enum AnimatedIcon {
    Frames {
        pub frames: FrameList,
        pub frame_duration_ms: NonZeroU32,
    },
    Transform {
        pub icon: IconData,
        pub animation: TransformAnimation,
    },
}
```

`TransformAnimation::Spin` similarly uses `NonZeroU32`:

```rust
pub enum TransformAnimation {
    Spin { duration_ms: NonZeroU32 },
}
```

`first_frame()` becomes infallible for `Frames` (always returns
`Some`); for `Transform`, returns the icon.

#### Rationale

Option **A** leaves the invalid-state door open. Option **B** (private
fields) works but makes matching ugly.

Option **D** (`_private: ()`) is a technique but ugly and
non-idiomatic.

Option **E** (documentation) doesn't enforce anything.

Option **C** uses standard Rust types (`NonZeroU32`) plus one tiny
newtype. The invariants are at the type level -- you cannot construct
an invalid `AnimatedIcon` without unsafe code. The match syntax still
works (`AnimatedIcon::Frames { frames, frame_duration_ms } => ...`)
with the typed fields.

**Confidence:** high.

#### Merge-review refinement: `impl Deref<Target = [IconData]> for FrameList`

As sketched, `FrameList` exposes `as_slice()` and `first()` as named
methods. Every consumer then writes `frame_list.as_slice().iter()` or
`frame_list.as_slice().len()`. That's extra tokens for no type-safety
gain. Adding `impl Deref<Target = [IconData]> for FrameList` (and
`Deref` only, not `DerefMut`, to preserve the non-empty invariant)
lets callers write `frame_list.iter()`, `frame_list.len()`,
`&frame_list[0]`, etc. directly. The newtype still prevents
construction of an empty `FrameList`, so the invariant is intact, but
the ergonomic cost of wrapping the `Vec` disappears. This is the
standard pattern for validated collection newtypes in Rust (see
`NonEmpty<T>` in the `nonempty` crate, which does exactly this).

Recommend adding this to the sketch. One line of impl, zero semantic
change.

---

### C4. Font family ownership: owned `String` per widget × connector leak

**Files:**
- `native-theme/src/model/font.rs:159-171` (`ResolvedFontSpec::family: String`)
- `native-theme/src/model/resolved.rs:169-234` (`ResolvedThemeVariant` has ~22 `font` fields across widgets + defaults)
- `connectors/native-theme-iced/src/lib.rs:40-51` (the `Box::leak` workaround)

#### Problem

```rust
pub struct ResolvedFontSpec {
    pub family: String,
    pub size: f32,
    pub weight: u16,
    pub style: FontStyle,
    pub color: Rgba,
}
```

A `ResolvedTheme` contains a `ResolvedFontSpec` inside `defaults.font`,
`defaults.mono_font`, and every widget's `font` field. For a typical
theme where every widget inherits `"Inter"`, that's ~22 separate owned
`String` clones of the same 5-byte string per resolved theme.

Separately, the public API forces downstream iced users to leak the
font family to obtain `&'static str` for iced's `Font::Family::Name`.
**Framing correction (merge review):** The `Box::leak` snippet below
is a **doc-comment example** at `connectors/native-theme-iced/src/lib.rs:40-43`,
*not* runtime code inside the connector. The connector's own
`font_family()` at `lib.rs:251` returns a plain `&str` that borrows
from `resolved.defaults.font.family`. So the leak is the user's
responsibility, not the library's, but the underlying problem is the
same: iced's `Font::Family::Name` wants `&'static str`, and the only
way a user can produce one from the current `ResolvedFontSpec::family:
String` is to `Box::leak` the value themselves on every call. The
connector documents the pattern as "standard iced" because the
public API offers no alternative.

```rust
// From connectors/native-theme-iced/src/lib.rs:40-43 (doc comment!):
//! let name: &'static str = Box::leak(
//!     native_theme_iced::font_family(&resolved).to_string().into_boxed_str()
//! );
```

Each call a user makes per the documented pattern leaks another copy.
The workaround is inelegant regardless of whether the leak is in
connector code or user code.

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Memory waste; connector leak burden |
| B | **Use `Arc<str>` for `family`** | One allocation per unique name; inheritance reuses the Arc | `Arc<str>` in public API; `&String` access patterns break |
| C | **Intern font family names** at resolution time: the crate maintains a static `HashMap<String, &'static str>` interner, and `ResolvedFontSpec::family: &'static str` | Zero-alloc after first use; iced gets `&'static str` for free | Uses `Box::leak` (bounded per unique family) |
| D | **Reference-count via `Rc<str>`** | Single-threaded sharing | Cannot cross thread boundaries; breaks `Send` |
| E | **Keep owned String but add a `family_static` method** that leaks on demand for the connector's benefit | Backwards compatible | Still one leak per call; doesn't solve the per-widget duplication |

#### Recommended: **B** + **C** combined

1. **Change `ResolvedFontSpec::family` to `Arc<str>`** for normal
   shared ownership inside a `ResolvedTheme`. All 22 copies point at
   the same Arc. Cloning a theme increments the refcount; no heap
   traffic beyond that.
2. **Add a crate-level intern helper**:
   ```rust
   pub fn intern_family(name: &str) -> &'static str;
   ```
   Internally maintains a `OnceLock<Mutex<HashMap<Arc<str>, &'static str>>>`.
   Leaks each unique name exactly once. Iced connector calls this
   instead of its own `Box::leak`.

```rust
pub struct ResolvedFontSpec {
    pub family: Arc<str>,
    pub size: f32,
    pub weight: u16,
    pub style: FontStyle,
    pub color: Rgba,
}

// In native-theme root (or a new `fonts` module):
pub fn intern_family(name: &str) -> &'static str {
    // Intern once per unique name across the process lifetime.
    // Bounded growth: only grows with distinct family names.
    ...
}
```

#### Rationale

Option **A** is wasteful. Option **B** alone saves memory but the
iced connector still has to leak. Option **C** alone saves iced's
leak but doesn't help defaults/widgets sharing the same name within
one theme.

Option **D** (`Rc<str>`) breaks `Send` and therefore rules out
sending themes across threads -- common in GUI apps with worker
threads. Rejected.

Option **E** is the minimum change but doesn't address the
per-theme duplication.

Combining **B** (Arc sharing within a theme) and **C** (intern helper
for connector consumption) gives the best of both: minimal memory
per theme, and a sanctioned path to `&'static str` that replaces
the iced user's documented ad-hoc leak.

**Confidence:** medium-high. `Arc<str>` changes public signatures in
non-trivial ways; connector rewrites are needed. But the connectors
are in this tree, so they can migrate in lockstep.

#### Merge-review note: `Arc<str>` serde considerations

Changing `ResolvedFontSpec::family: String → Arc<str>` needs two
small deserialize accommodations:

1. `serde` has a `rc` feature flag that enables `Arc<str>` directly.
   Enable it in `native-theme/Cargo.toml`: `serde = { version = "1",
   features = ["derive", "rc"] }`. Without this flag, `Arc<str>` does
   not implement `Deserialize` out of the box.
2. `PartialEq` on `Arc<str>` compares the *contents* (via `Deref`),
   not the pointer, so existing test equality expectations still
   hold. No changes to test fixtures needed.

These are ~5 lines total (one Cargo.toml flag plus a comment in the
type definition). The doc's recommendation stands; this addendum
just records the migration detail.

---

### C5. `detect_linux_de` takes `&str`, forcing a two-call idiom

**File:** `native-theme/src/detect.rs:39-53`

#### Problem

```rust
pub fn detect_linux_de(xdg_current_desktop: &str) -> LinuxDesktop { ... }
```

Every caller writes:

```rust
detect_linux_de(&xdg_current_desktop())
```

Where `xdg_current_desktop()` is `pub(crate)` and reads the env var.
Public callers can only get the env var by reading it themselves:

```rust
let de = detect_linux_de(&std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default());
```

That exposes the implementation convention (the exact env var name)
and makes the common case verbose. The public API is "parse this
string" when users want "detect the current desktop".

The current shape exists for testability (tests inject fake XDG
values), but the trade-off is wrong: tests should do the injection,
not production callers.

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Verbose public idiom |
| B | **Split into two**: a public `detect_linux_desktop()` that reads the env var, and a `pub(crate) parse_linux_desktop(&str)` for tests | Public API ergonomic; tests retain injectability | Two functions |
| C | **Make `xdg_current_desktop()` public** | Users can call both | Exposes the env var name as a crate contract |
| D | **Keep the current signature but add a convenience wrapper** `detect_linux_desktop()` that reads the env var | Fine-grained control | Two functions; more names |

#### Recommended: **B**

```rust
// Public:
pub fn detect_linux_desktop() -> LinuxDesktop {
    parse_linux_desktop(&xdg_current_desktop())
}

// pub(crate) for tests:
pub(crate) fn parse_linux_desktop(xdg_current_desktop: &str) -> LinuxDesktop { ... }
pub(crate) fn xdg_current_desktop() -> String { ... }  // stays as-is
```

Rename `detect_linux_de` to `detect_linux_desktop` while we're at it
(doc 1 §1 recommends renaming abbreviations).

#### Rationale

Option **A** leaves the verbose idiom. Option **C** exposes the env
var name, which is fine in practice but creates a subtle contract
("we will always read this exact env var") that the crate shouldn't
commit to.

Option **D** ends up with the same two-function result as **B** but
the naming is less clear ("detect_linux_desktop" vs "detect_linux_de"
exists for what reason?).

Option **B** is the right shape: one public no-argument function for
the common case, one `pub(crate)` pure parser for tests.

**Confidence:** high.

---

### C6. Platform reader functions are `pub` but only useful inside the pipeline

**Files:**
- `native-theme/src/kde/mod.rs:341` (`pub fn from_kde`)
- `native-theme/src/macos.rs:397` (`pub fn from_macos`)
- `native-theme/src/windows.rs:578` (`pub fn from_windows`)
- `native-theme/src/gnome/mod.rs:280` (`pub fn build_gnome_spec_pure`)
- `native-theme/src/gnome/mod.rs` (`pub fn from_gnome`, `pub fn from_kde_with_portal`)

#### Problem

These functions return raw `ThemeSpec` -- not merged with a preset,
not resolved, not validated. A user who calls `from_kde()` directly
and tries to `into_resolved()` gets a flood of missing-field errors
because the KDE reader doesn't populate widget geometry (that comes
from merging with `kde-breeze-live`).

Legitimate use cases:

1. Debug output (`println!("{:?}", from_kde())`)
2. Custom merge with a non-default preset (niche)
3. Snapshot-testing reader output (test-only)

All three are covered by:

- `SystemTheme::from_system()` for the main use case
- Crate-internal access (`pub(crate)`) for snapshot tests inside `src/`
- External snapshot tests running via `#[cfg(test)]` with access to the tree

There is no legitimate downstream public consumer.

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Leaky implementation detail at public API |
| B | **Demote to `pub(crate)`** | Hides implementation detail | Users who today depend on these lose access |
| C | **Rename to signal raw-output nature**: `pub fn kde_reader_output()`, etc. | Preserves access for power users | Still clutters public namespace |
| D | **Move to a `native_theme::readers::*` module** that is explicitly marked as "raw; not for normal use" and feature-gated | Preserves access in a signposted place | More ceremony |
| E | **Delete entirely and replace with a `SystemTheme::from_reader(reader_kind)` entry point** that bundles the reader + pipeline | Most ergonomic; single entry | Requires enumerating reader kinds and dispatching |

#### Recommended: **B** (demote to `pub(crate)`)

A grep shows no consumer outside the pipeline or tests. Demoting to
`pub(crate)` is the minimum change and the safest path.

If external feedback indicates a real use case after the change,
those specific functions can be re-exposed with clear "raw output"
naming.

#### Rationale

Option **A** leaks implementation details. Option **C** (rename) is
busy-work for something that should just be hidden.

Option **D** (explicit `readers` module) was recommended in doc 1 §12
as part of the namespace partition. If that lands, the demotion should
be done in concert: the reader functions move *into* that module as
`pub(crate)`, and the module itself is not exported at the root.

Option **E** (single `from_reader` entry) is elegant but requires
defining a `ReaderKind` enum and a dispatch table; overkill for a
functionality that users should rarely reach into.

Option **B** is the right default. If doc 1 §12 lands, merge the two
changes.

**Confidence:** high.

---

## D. Polish

### D1. `FontSpec::style` silently defaults while sibling fields error

**File:** `native-theme/src/resolve/validate_helpers.rs:39-59`

```rust
pub(crate) fn require_font(
    font: &FontSpec,
    prefix: &str,
    dpi: f32,
    missing: &mut Vec<String>,
) -> ResolvedFontSpec {
    let family = require(&font.family, &format!("{prefix}.family"), missing);
    let size = font.size.map(|fs| fs.to_px(dpi)).unwrap_or_else(|| {
        missing.push(format!("{prefix}.size"));
        0.0
    });
    let weight = require(&font.weight, &format!("{prefix}.weight"), missing);
    let color = require(&font.color, &format!("{prefix}.color"), missing);
    ResolvedFontSpec {
        family,
        size,
        weight,
        style: font.style.unwrap_or_default(),    // silently defaults
        color,
    }
}
```

Four fields use `require` (recording "missing"). One field (`style`)
uses `unwrap_or_default`. The result is that a theme missing `style`
silently gets `FontStyle::Normal`, while missing any other field
produces an error.

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Inconsistent; missing `style` is silent |
| B | **Require `style`** via `require(&font.style, ...)` | Consistent | Every preset must specify style (many do not) |
| C | **Document the asymmetry** with a comment explaining that Normal is the natural default | No behavioural change | Still inconsistent; just signposted |
| D | **Make `style` default explicit at the type level**: `ResolvedFontSpec::style: FontStyle` with a default via `const DEFAULT_STYLE: FontStyle = FontStyle::Normal` referenced in the construction | Same behaviour, more visible | No real improvement |

#### Recommended: **C** (document the asymmetry)

```rust
// style is inherently optional; Normal is the natural default when the
// theme doesn't specify italic/oblique. Other font fields (family, size,
// weight, color) are required because they have no universally-safe
// default.
style: font.style.unwrap_or_default(),
```

#### Rationale

Option **A** is silent. Option **B** would require every preset to
specify `style = "normal"` explicitly, which is friction for a default
value that is almost universally what users want.

Option **C** is minimum-change and honest. The asymmetry exists for a
reason; document it so future maintainers don't remove it.

**Confidence:** low-medium. The issue is minor. Reasonable
maintainers could pick any of A, B, or C.

---

### D2. `defaults.border.padding` derives from the presence of unrelated fields

**File:** `native-theme/src/resolve/inheritance.rs:148-159`

#### Problem

```rust
if d.border.padding_horizontal.is_none() {
    d.border.padding_horizontal = d.border.line_width.map(|_| 0.0_f32);
    if d.border.padding_horizontal.is_none() {
        d.border.padding_horizontal = d.border.corner_radius.map(|_| 0.0_f32);
    }
}
if d.border.padding_vertical.is_none() {
    d.border.padding_vertical = d.border.line_width.map(|_| 0.0_f32);
    if d.border.padding_vertical.is_none() {
        d.border.padding_vertical = d.border.corner_radius.map(|_| 0.0_f32);
    }
}
```

Translation: *"if `line_width` is set, fill padding with 0.0; else if
`corner_radius` is set, fill padding with 0.0; else leave None."*

The resolved value is `0.0` in both branches -- the branching is
about **whether** to fill, not **what** to fill with. This is a
workaround for B6 (`BorderSpec` containing fields not meaningful at
the defaults level). The resolver has to decide "is this border
real enough to warrant filling padding with its placeholder value?"

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Weird hand-coded rule; no design justification |
| B | **Fix B6 first** (split `BorderSpec` into `DefaultsBorderSpec` / `WidgetBorderSpec`). Defaults borders no longer have padding; this whole rule disappears. | Root cause fix | Depends on B6 |
| C | **Simplify to a single `is_any_border_field_set` check** | Marginally cleaner | Same underlying hack |
| D | **Always fill padding with 0.0 at defaults level** | Simplest | But then the validator doesn't catch a truly-absent border at all |

#### Recommended: **B** (fix B6)

This is a symptom of B6. Once `DefaultsBorderSpec` loses padding
fields, the rule is deleted along with them.

#### Rationale

Options **C** and **D** treat the symptom. Option **B** removes the
symptom by removing the cause.

**Confidence:** high (contingent on B6).

---

### D3. `check_ranges` builds path strings eagerly via `format!`

**File:** `native-theme/src/model/widgets/mod.rs:908-1347`

#### Problem

Every `check_ranges` impl contains ~6 `format!` calls to build path
strings:

```rust
check_non_negative(self.min_width, &format!("{prefix}.min_width"), errors);
check_non_negative(self.min_height, &format!("{prefix}.min_height"), errors);
// ... etc
```

For 25 widgets × ~5 fields each, that's ~125 `format!` allocations
per resolution, **regardless of whether any error is found**. On the
happy path (theme validates successfully), all 125 allocations are
discarded immediately.

Not a huge cost, but measurable. And easily fixed.

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | 125 wasted allocations per validate |
| B | **Defer string construction to the error path** by passing prefix + field name separately: `check_non_negative(value, prefix, "min_width", errors)`. The check function only `format!`s when it actually records an error. | Zero allocation on happy path | Changes all check function signatures |
| C | **Use `Cow<'static, str>` for error strings** and pass `prefix` as a struct containing field name constants | More type-safe | Overkill |
| D | **Subsumed by B1 codegen** -- generated check functions can use whatever optimal shape | Aligns with codegen direction | Depends on B1 |

#### Recommended: **B** (if B1 is deferred) or **D** (if B1 lands)

Short-term minimum fix: change the check function signatures:

```rust
pub(crate) fn check_non_negative(
    value: f32,
    prefix: &str,
    field: &'static str,
    errors: &mut Vec<String>,
) {
    if value < 0.0 {
        errors.push(format!("{prefix}.{field}: got {value}"));
    }
}
```

Call sites become:

```rust
check_non_negative(self.min_width, prefix, "min_width", errors);
```

Zero allocation on happy path.

#### Rationale

Option **A** wastes allocations. Option **C** is architecturally
heavier than the fix warrants.

Option **D** is the right long-term path if B1 lands -- generated
code can use whatever shape is optimal. Option **B** is the
minimum fix for v0.5.7 if B1 is deferred.

**Confidence:** high.

---

### D4. `name` and `icon_theme` are owned `String` for bundled presets

**Files:**
- `native-theme/src/model/mod.rs:228` (`ThemeSpec::name: String`)
- `native-theme/src/model/resolved.rs:233` (`ResolvedThemeVariant::icon_theme: String`)

#### Problem

For bundled presets, both values are compile-time constants
(`"Dracula"`, `"breeze-dark"`, etc.), but each load allocates a
fresh `String` via `to_string()` / `into()` calls.

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Wasteful for bundled case |
| B | **Use `Cow<'static, str>`**: bundled presets use `Cow::Borrowed`, user presets use `Cow::Owned` | Zero alloc for bundled case | `Cow` in public API |
| C | **Use `Arc<str>`** (same direction as C4) | Shared; cheap clone | Extra indirection |
| D | **Status quo, but optimize the common case internally** via string interning | Transparent to users | Needs global interner |

#### Recommended: **B** (`Cow<'static, str>`)

Aligns with C4's direction but uses `Cow` for values that don't
need refcounting. Names are small and rarely cloned inside a theme.

#### Rationale

Option **A** wastes. Option **C** is heavier than needed for values
that are read once and not widely shared. Option **D** is hidden
optimization that doesn't help users who want to avoid allocation
explicitly.

Option **B** is the cleanest honest representation: *"this name is
either borrowed from a compile-time constant or owned from a user
load"*.

**Confidence:** medium. `Cow` in public types interacts with
serde and `PartialEq`; worth a round of testing before committing.

---

### D5. `from_kde_content_pure` hardcodes `button_order` that resolution already handles

**Files:**
- `native-theme/src/kde/mod.rs:52-53` (reader hardcodes `PrimaryLeft`)
- `native-theme/src/resolve/inheritance.rs:98-109` (`platform_button_order`)

#### Problem

The KDE reader sets `variant.dialog.button_order = Some(PrimaryLeft)`
unconditionally. The resolution layer also has a `platform_button_order`
that sets it to `PrimaryLeft` on KDE. Two code paths, one decision.

In practice the reader runs first, so the resolver's check sees
`Some(...)` and doesn't fire. But there are two places where a change
in KDE's button order convention would need to be updated.

#### Options

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | Two places to keep in sync |
| B | **Delete the hardcode in `from_kde_content_pure`**; let the resolution layer handle it | Single source | Reader output is less "complete" -- a caller using `from_kde()` directly and observing the raw `ThemeSpec` no longer sees `button_order` set |
| C | **Delete the resolution layer fallback**; require readers to set it | Single source | macOS and Windows readers must also set it; TOML presets must specify it |

#### Recommended: **B**

Delete the hardcode. Let resolution handle it uniformly. Tests verify
that a KDE-loaded theme ends up with `PrimaryLeft` via the resolver
path, not the reader path.

If A4's recommendation is adopted (move `button_order` to
`resolve_platform_defaults`), this is doubly clean: the resolver no
longer tries to set it in `resolve()`, and the reader doesn't set it
either. Both paths converge on `resolve_platform_defaults` as the
single source.

#### Rationale

Option **A** leaves the drift risk. Option **C** would require every
reader to think about button order, which duplicates the decision N
ways instead of 2.

Option **B** is the minimum change and matches the architectural
direction of A4.

**Confidence:** medium. Depends on A4's resolution.

---

## E. Priority summary

| Priority | Issue | Type | Effort |
|---|---|---|---|
| P0 | A2 `check_ranges` on placeholder data | Bug | Low |
| P0 | A3 `missing_fields` dual-category (bundles with A2) | Bug | — |
| P0 | A4 `resolve()` OS-detection doc leak | Bug / design | Low |
| P0 | C1 Remove `ThemeChangeEvent::Other` | Cleanup | Trivial |
| P0 | C2 Rename `ColorSchemeChanged` → `Changed` | Correctness | Trivial |
| P0 | B4 Accessibility fields off `ThemeDefaults` | Design | Medium |
| P0 | B6 `BorderSpec` split for defaults vs widget | Design | Medium |
| P1 | B1 Codegen for validate/range-check boilerplate | Structural | Very high |
| P1 | B2 Inheritance rules TOML/Rust drift | Structural | High |
| P1 | B3 Reader contract heterogeneity | Structural | Medium |
| P1 | B5 `font_dpi` → `ResolutionContext` | Design | Medium |
| P1 | C3 `AnimatedIcon` typed construction | Safety | Low |
| P1 | C4 `Arc<str>` font family + intern helper | Memory/perf | Medium |
| P1 | C6 Demote platform readers | Cleanup | Low |
| ~~P2~~ FIXED | ~~A1 `Instant` arithmetic without `checked_sub`~~ Fixed by commit `f9e5956`, see STATUS block on A1 | — | — |
| P2 | B7 Unify border-inheritance paths (bundles with B1) | Structural | — |
| P2 | C5 `detect_linux_desktop()` no-arg | Ergonomics | Trivial |
| P2 | D3 `check_ranges` path string allocation | Perf | Low |
| P2 | D4 `Cow<'static, str>` for names | Memory | Low |
| P2 | D5 KDE reader `button_order` hardcode | Cleanup | Trivial |
| P3 | D1 `FontSpec::style` default consistency doc | Polish | Trivial |
| P3 | D2 Padding-derives-from-presence rule (symptom of B6) | — | — |

- **P0 items** should land in v0.5.7. They are verified bugs (A2-A4)
  or small corrections with immediate correctness impact (C1, C2, B4,
  B6). A1 was initially catalogued here, demoted to P2 after verification
  showed it is a documented may-panic rather than a verified startup
  crash, and is now **removed entirely from the v0.5.7 backlog** because
  commit `f9e5956` has already applied the Option C fix to current
  `watch/kde.rs`.
- **P1 items** are the substantial structural changes. B1 and B2 are
  the long-term investments; B3, B5, C3, C4, C6 are one-shot fixes
  with medium-sized diffs.
- **P2/P3 items** can safely slip to v0.5.8.

---

## F. Open questions

These are points where I am not absolutely sure of the recommendation
or where the answer depends on a broader decision:

1. **A4's solution depends on doc 1 §7.** If doc 1 §7 (demote `resolve`
   intermediates to `pub(crate)`) is adopted, A4 dissolves because
   `into_resolved()` becomes the only public path. If §7 is rejected,
   the move-button_order-to-resolve_platform_defaults fix is still
   needed.

2. **A3 depends on doc 1 §6.** The cleanest fix for the dual-category
   `missing_fields` is to restructure `Error` per doc 1 §6. If that
   restructure is deferred, A3 falls back to the two-vec alternative.

3. **B1 + B2 + B7 depend on the same codegen infrastructure.** All
   three are the same bet: promote `property-registry.toml` (and the
   inheritance rules) to a load-bearing source of truth with codegen.
   The schema design is nontrivial and should be done in one go.

4. **B3's reader contract change interacts with the pipeline.** The
   `ReaderOutput` type needs to flow through `run_pipeline`, which
   also has to serve the overlay-source path (doc 1 §3). These two
   design choices should be worked out together.

5. **B4 + B5 pair up.** Both are "extract runtime-detected state out
   of `ThemeDefaults`". The `ResolutionContext` I sketched in B5 is a
   natural home for the accessibility struct from B4, but that's a
   design decision that should be made consciously rather than as a
   side effect.

6. **C2's rename may want per-platform refinement.** Future versions
   could add a narrower `ColorSchemeChanged` variant that only the
   macOS backend emits (since it's the only one that can tell).
   Whether to plan for that now or defer is a judgement call.

7. **C4's `Arc<str>` migration affects connectors.** The iced and
   gpui connectors both read `.family` directly today; they need to
   migrate from `&String` to `&str` (via `Arc::as_ref()`) or `&Arc<str>`.
   The migration should happen in lockstep with the core crate change
   since the connectors are in-tree.

8. **`docs/todo.md` is stale.** As noted in doc 1 §28 item 10, the
   top-level `todo.md` has contradictions with v0.5.6 work. A cleanup
   pass is independent of this document but should be scheduled.

---

## G. Post-script: the crate is two codebases pretending to be one

Both doc 1 and doc 2 circle around the same root cause: `native-theme`
is trying to be both:

1. **A data library.** Types, TOML round-tripping, pure transforms,
   testability. The `from_kde_content_pure` / `from_kde_content` split,
   the `inheritance-rules.toml`, the `property-registry.toml`, the
   generated `FIELD_NAMES`, and the (claimed) purity of `resolve()` all
   point in this direction.

2. **A system detection library.** OS subprocess spawns, env var reads,
   file I/O, async portal calls, thread-based watchers, global caches,
   per-platform feature flags. The `invalidate_caches` coarseness, the
   cached/uncached function pairs, the async/sync split, the
   platform-specific readers exposed at the crate root, the
   accessibility fields mixed into `ThemeDefaults`, and `font_dpi`
   living on `ThemeDefaults` all point here.

These two codebases have **different requirements for purity,
testability, error handling, and memory discipline**, but they share
types, modules, and the crate-level namespace. A4 (doc leak), B4
(accessibility conflation), B5 (font_dpi conflation), and A2 (range
checks on placeholder data via defaults) are all symptoms of the same
root pressure: code that should be pure gets nudged into touching OS
state because the surrounding types already mix the two.

A v1.0 refactor worth considering (beyond v0.5.7's scope): **split
into two crates.**

- **`native-theme-model`** — the data library. `Theme`, `ResolvedTheme`,
  resolution, merging, presets, TOML, inheritance. Pure. Testable. No
  OS access. Depends only on serde + toml + small helpers.
- **`native-theme-system`** — the detection/watcher library. Depends on
  `-model`. Reads the OS, runs the pipeline, exposes
  `SystemTheme::from_system`, watchers, icon loaders, accessibility
  detection, feature flags for platform backends.

Connectors depend on `-system` (which re-exports `-model` types for
convenience). Apps that just want to load a bundled preset depend only
on `-model` -- smaller deps, faster compile, no platform baggage, no
`watch` / `portal-tokio` / `kde` feature surface to reason about.

Not proposing this for v0.5.7 -- the P0/P1 items from both documents
are the right focus. But it is the direction both analyses are
pointing, and it's worth stating out loud so future roadmap decisions
can align.

---

## H. Cross-document P0 consolidation

Bundling the P0 items from both documents into a single shippable
v0.5.7 scope:

**From doc 1:**
- §1 Type vocabulary rename (P0)
- §4 Drop `SystemTheme::active()`, keep `pick` (P0)
- §6a Drop `Error::Clone` bound (P0)
- §12 Partition crate root (P0)
- §16 `Rgba` polish (P0)
- §19 `LinuxDesktop` `non_exhaustive` + new variants (P0)

**From doc 2 (this document):**
- A2 `check_ranges` on placeholders (P0)
- A3 `missing_fields` dual category (P0, bundled with A2)
- A4 `resolve()` OS-detection leak (P0)
- B4 Accessibility off `ThemeDefaults` (P0)
- B6 `BorderSpec` split (P0)
- C1 Remove `ThemeChangeEvent::Other` (P0)
- C2 Rename `ColorSchemeChanged` → `Changed` (P0)

**Correction note on A1 (SUPERSEDED BY FIXED STATUS).** This item was
initially catalogued as P0 (verified bug), then demoted to P2 after a
closer reading of Rust std source showed the `Instant -
Duration::from_secs(10)` expression produces a valid-if-strange
`Instant` with negative internal seconds on the Linux code path this
file compiles on, without panicking. A subsequent merge-review pass
(commit `f9e5956` already on `main`) applied the Option C `Option<Instant>`
fix and the item is now **fully removed from the v0.5.7 cohort**. See
the STATUS block at the top of A1 for the current code shape. No work
required.

This is 13 P0 items totaling **~30-50 diffs of varying size**. Bugs
(A2, A3, A4) are small. Renames (§1, §4, §16, §19, C1, C2) are
mostly mechanical. Structural (§12, §6a, B4, B6) are medium. All
together they constitute a coherent v0.5.7 release.

The P1 items from both documents can be staged across v0.5.7, v0.5.8,
and v0.5.9 depending on capacity. The biggest investments are the
codegen work (doc 1 §2 + §14 + this doc's B1 + B2 + B7, all the same
bet) which is appropriate for a dedicated phase.

---

## I. Supplemental items discovered in merge review

A second verification pass (reading every cited file:line against the
current tree) surfaced five additional items neither A/B/C/D sections
catch. All are small. All are real.

### I1. `presets.rs` stale comment claims `Error` is not `Clone`

**Files:**
- `native-theme/src/presets.rs:85-88`
- `native-theme/src/error.rs:80` (`#[derive(Debug, Clone)]`)

The cache-type comment reads:

```rust
// Errors are stored as String because Error is not Clone, which is
// required for LazyLock storage without lifetime constraints.
type Parsed = std::result::Result<ThemeSpec, String>;
```

But `error.rs:80` already derives `Clone`. The comment is stale: at
some earlier point `Error` was not `Clone`, then `Clone` was added
(see doc 1 §6a's rationale for why the bound was added -- to support
caching), and the presets cache was never migrated. Two consequences:

1. The cache loses variant information: every error becomes a
   `Format(String)` on the way back out (via the `|e| e.to_string()`
   at `presets.rs:91`), even if the original was `Error::Io` or
   `Error::Resolution`.
2. Doc 1 §6a's recommendation to drop the `Clone` bound is now
   clean: nothing depends on it, including the comment that claimed
   to.

**Recommended fix (bundled with §6a):** update the cache to store
`std::result::Result<ThemeSpec, Arc<Error>>` (or `Error` directly if
§6a's Clone drop is paired with a storage shape that does not need
Clone). Remove the stale comment. This is a 5-line change.

**Confidence:** high.

### I2. `ThemeResolutionError` `Display` hint references `from_toml_with_base`

**File:** `native-theme/src/error.rs:55-65`

The hint branch in `ThemeResolutionError::fmt` says:

```rust
write!(
    f,
    "\n  hint: root defaults drive widget inheritance; \
     consider using ThemeSpec::from_toml_with_base() to inherit from a complete preset"
)?;
```

Doc 1 §15a recommends deleting `from_toml_with_base` as a one-liner
wrapper with no unique value. If §15a lands, this hint message points
at a nonexistent method. Cross-cutting cleanup needed.

**Options:**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Delete `from_toml_with_base` per §15a and leave the hint alone** | — | Hint references a gone method; users see a dangling name in diagnostics |
| B | **Delete `from_toml_with_base` and update the hint** to point at the two-call idiom (`preset(name).merge(&from_toml(s))`) | Hint still useful | Longer hint message |
| C | **Delete `from_toml_with_base` and delete the hint** | Minimum cleanup | Loses guidance for the "missing root defaults" case which is the most common user mistake |
| D | **Keep `from_toml_with_base` (reject §15a)** | Preserves the hint as-is | Keeps a one-liner wrapper that doesn't scale to JSON/YAML |

**Recommended: B.** Rewrite the hint to:

> hint: root defaults drive widget inheritance; load a complete base
> preset with `ThemeSpec::preset(name)` and merge your overlay onto it.

This preserves the guidance without naming the removed wrapper.

**Confidence:** high.

### I3. `run_gsettings_with_timeout` can block for 2 seconds on cold caches

**File:** `native-theme/src/detect.rs:138-177`

`system_is_dark()` caches its result, but on the first call (or after
`invalidate_caches()`), Linux falls through to
`run_gsettings_with_timeout(&["get", "org.gnome.desktop.interface", "color-scheme"])`.
That function uses a 2-second timeout loop with `try_wait` + 50 ms
`sleep`. If `gsettings` is installed but D-Bus is slow to respond,
the call can block for up to **2 seconds** of wall clock.

Doc 1 §13 discusses the cache-granularity and staleness problems but
never mentions this latency. It matters for:

- Connectors that call `system_is_dark()` on the UI thread (e.g.
  `showcase-gpui.rs:702` and `showcase-iced.rs:254` per doc 1 §13's
  "per-frame" claim -- if a cache invalidation lands mid-frame, the
  next frame stalls up to 2 s).
- Windowed apps that call `SystemTheme::from_system()` at startup
  -- the 2 s worst case compounds with the portal call's own delay.

**Options:**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** | No change | 2 s worst-case stall on cold cache |
| B | **Reduce the timeout to 500 ms** | Bounds the worst case | Risk of missing slow-but-successful responses |
| C | **Make the timeout a build-time const** (e.g. `DETECT_TIMEOUT_MS = 2000`) and document it in the module header | Trivial; at least surfaces the number | Still 2 s by default |
| D | **Decouple cache warming from the hot path**: spawn a background thread at first-use that runs detection and publishes to the cache; the foreground path returns a tentative `false` until the detection completes | Hot path is non-blocking | Short window of wrong answer after startup; concurrency complexity |
| E | **Defer the gsettings call into `detect_is_dark_async()`** and mark the sync form as "may block up to 2 seconds on Linux" in the rustdoc; connectors that cannot tolerate that use the async form | Explicit contract | Requires connectors to have an async runtime; couples with doc 1 §5 |

**Recommended: C (short-term) + E (longer-term).** Exposing the 2 s
timeout as a documented constant is immediate and free. The async
split is a downstream change that should happen as part of doc 1 §5's
async decision.

**Confidence:** medium. The doc 1 §13 DetectionContext work should
probably absorb this item; I'm recording it here so it doesn't get
lost.

### I4. `FontSize::default() = Px(0.0)` cross-reference to A2

**File:** `native-theme/src/model/font.rs:66-70`

Cross-referenced in A2's merge-review addendum above. Noted here for
the §I inventory: the root cause of A2's spurious range errors has
two contributors, not one, and the second (`FontSize::default() =
Px(0.0)`) is a design choice worth questioning on its own merits.

No separate options table -- the A2 fix (restructure validate ordering)
solves this as a side effect. If A2 is not adopted, consider changing
the default to `Px(1.0)` or making `FontSize` have no `Default` at all
(require callers to construct a specific unit). But that's a much
smaller improvement than A2 itself.

### I5. `property-registry.toml` ≈ 174 field-name repetitions in the current `lint_toml`

**File:** `native-theme/src/model/mod.rs:540-745` (doc 1 §14)

Doc 1 §14 catalogues the hand-maintained duplication but doesn't put
a number on it. Merge review counted: there are roughly **29 widgets
× ~6 fields each ≈ 174 field names** that `lint_toml` matches
against, plus `TOP_KEYS` (4) + `VARIANT_KEYS` (29) + nested sections
(font, mono_font, border, icon_sizes) = **~215 string literals total
hand-maintained** across `model/mod.rs:554-720`. This is the ROI
number for §14's codegen recommendation and doc 1 §2's proc-macro
decision: any codegen path that eliminates these 215 literals wins
big on rename safety, even before touching validate/check_ranges.

**No new options.** Doc 1 §14's Option D is still the right direction.
The merge review only adds the number to help weigh the ROI.

---

## J. Third-pass review: deep-ultrathink refinement under "no backward compat"

This section records a third review pass performed under the
explicit "backward compatibility does not matter; I want the perfect
API" directive. Where earlier passes hedged under migration risk,
this pass commits harder. New findings are recorded here so existing
content is preserved unchanged.

This section should be read alongside **doc 1 §30**, which covers
doc-1-specific refinements and hosts M1 (a cross-document finding)
that this document cross-references here.

### J.1 Methodology

Same approach as doc 1 §30: independently re-read each existing
recommendation without first consulting earlier merge-review notes,
then reconciled. Every new claim was independently verified against
the current tree. The pass was **conservative about adding new
issues** and **aggressive about strengthening recommendations**
that were hedged for migration concerns.

### J.2 New options and refinements per section

#### B3 — refinement: use `Arc<str>` (or `Cow<'static, str>`) for `ReaderOutput::name`

The merge-reviewed `ReaderOutput` struct in B3 Option F uses
`name: String` while **C4 simultaneously recommends `Arc<str>`
for font family names on consistency grounds.** Applying C4's
reasoning symmetrically to B3: every reader produces a theme
name, and for bundled / preset-based paths the name is a
compile-time constant. Allocating a fresh `String` on every read
is waste.

**Refinement:**

```rust
pub struct ReaderOutput {
    pub name: Arc<str>,                  // was: String
    pub active: ThemeLayer,
    pub inactive: Option<ThemeLayer>,
    pub known_is_dark: Option<bool>,
}
```

Pick one ownership type across the crate and use it consistently.
Under "perfect API", **`Arc<str>` with `serde`'s `rc` feature** is
the cleanest answer: it composes with C4's font-family interning,
matches D4's `Cow<'static, str>` direction for `name`/`icon_theme`
in `ThemeSpec`/`ResolvedThemeVariant`, and avoids the "one type
uses `Arc<str>`, another uses `String`" inconsistency.

The ownership-type question propagates to:
- `SystemTheme::name` (doc 1 §3 + D4)
- `ResolvedThemeVariant::icon_theme` (doc 1 §20 + D4)
- `ReaderOutput::name` (this refinement)
- `ResolvedFontSpec::family` (C4)

**Recommendation:** use `Arc<str>` uniformly across all four.
Enable `serde = { version = "1", features = ["derive", "rc"] }`.
Touch all four fields in a single ownership-type refactor PR.

**Confidence:** high. This is the standard Rust answer for
"shared immutable string across many owners."

**Post-script 2026-04-20 — principled deviation per `docs/todo_v0.5.7_gaps.md` §G9:**

The uniform-`Arc<str>` recommendation above was NOT adopted for the
`SystemTheme::name` and `SystemTheme::icon_theme` axis. An audit
(recorded in `docs/todo_v0.5.7_gaps.md:449-506` as §G9) counted the
unique values these fields take in practice and concluded there is
**no dedup benefit**: each resolved theme carries exactly one `name`
(and KDE's two icon-theme names across light/dark are the maximum for
any platform — every other platform has one). Bundled preset names
are `&'static str` literals, so `Cow::Borrowed(static_lit)` costs
zero allocations and zero refcounts; `Arc::from(static_lit)` would
pay one allocation per unique string at construction time for a
dedup that structurally cannot fire.

The recommendation is retained for `ResolvedFontSpec::family` — the
one axis where dedup genuinely applies (26 widgets × connectors
share font families). See `docs/todo_v0.5.7_gaps.md` §G9 for the
audit detail and `native-theme/src/lib.rs` `SystemTheme::name`
rustdoc for the in-source record.

**Net result:** `name` / `icon_theme` stay `Cow<'static, str>`;
`family` stays `Arc<str>`. Mixed ownership is intentional, not
accidental inconsistency.

#### B3 — Option G: define a `ThemeReader` trait alongside the data shape

B3's existing options A-F all describe the **data shape** readers
return. None describe the **reader interface**. Under "no backward
compat" a cleaner internal architecture pairs F (data shape) with
a reader trait:

| # | Option | Pros | Cons |
|---|---|---|---|
| G | **Define `trait ThemeReader { fn read(&self) -> Result<ReaderOutput, ReaderError>; }` internally**, with one impl per backend (`KdeReader`, `MacosReader`, `WindowsReader`, `GnomeReader`, ...), dispatched via an internal `select_reader() -> Option<Box<dyn ThemeReader>>` or a static sum type | Mock readers in tests become trivial (one trait impl). Static dispatch via enum gives zero overhead. Encapsulates the reader + `is_dark` fallback logic in one place. Aligns with C6 (demoted platform readers) by making the public surface a single entry point rather than N `from_*` functions. Eliminates the implicit reader contract B3 complains about. | Adds one trait to the internal surface (not exposed publicly). Existing readers must be converted from free functions to methods on `Self`. Slightly larger migration. |

**Position:** G pairs naturally with F (data shape) — F defines
*what* readers return, G defines *how* they are invoked. Adopting
both gives the cleanest internal architecture.

**Recommendation:** adopt **F for v0.5.7 (P0, already
recommended)**; **defer G to v0.5.8** unless the C6 demotion work
uncovers a compelling reason to land it sooner. F alone already
eliminates the heterogeneous-contract complaint; G is polish
on the internal seam.

**Confidence:** medium. G is desirable but not urgent; F alone
is the critical fix for the B3 problem.

#### B5 — refinement: `ResolutionContext` constructor naming

The merge-review addendum on B5 proposes `Option<&ResolutionContext>`
at the API boundary. A small but important ergonomic refinement
on constructor naming:

```rust
impl ResolutionContext {
    /// Construct with all fields auto-detected from the current OS.
    pub fn from_system() -> Self { /* queries detect::* */ }

    /// Construct with explicit, deterministic values (96 DPI,
    /// PrimaryRight, etc.) for tests, snapshot fixtures, and
    /// caches where reproducibility matters.
    pub fn for_tests() -> Self { /* hardcoded values */ }
}

impl Theme {
    pub fn resolve(self, ctx: Option<&ResolutionContext>) -> Result<ResolvedTheme>;
    pub fn resolve_system(self) -> Result<ResolvedTheme> { self.resolve(None) }
}
```

The **`for_tests()`** name (or `test_defaults()` — bikeshed) is
explicit about its purpose and is not a fake "default" pretending
to be OS-detected. This avoids a trap where
`ResolutionContext::default()` silently produces a 96-DPI value on
a Retina display and the test passes with wrong pt-to-px math.

Under "perfect API", **runtime-detected types should not have
silent `Default` implementations**. Either the type implements
`Default` truthfully (by calling OS detection) or it forces the
user to choose a constructor explicitly.

**Recommendation:**
1. Do **not** implement `Default` for `ResolutionContext`.
2. Expose `from_system()` for the auto-detect path.
3. Expose `for_tests()` (or `deterministic()` or similar) for
   the deterministic path. Name must signal intent.

**Confidence:** high. The naming matters for test correctness.

#### B4 — refinement: `AccessibilityPreferences` belongs on `SystemTheme`, not in `ResolutionContext`

Doc 2 B5's merge-review addendum sketched a `ResolutionContext`
containing `accessibility: AccessibilityPreferences` from B4.
That is architecturally wrong under "perfect API":

- `ResolutionContext` is a **resolution-time input**: values
  needed to *compute* a `ResolvedTheme` from a `Theme` (DPI,
  icon theme, button order convention).
- `AccessibilityPreferences` is a **runtime state**: the user's
  current accessibility settings, which affect how an app
  *renders* a resolved theme (e.g., apply `text_scaling_factor`
  to the final font size).

Mixing these conflates "inputs needed to build the theme" with
"runtime rendering hints" — the exact conflation B4 argues
against when pulling accessibility fields off `ThemeDefaults`.

**Recommendation:** keep `AccessibilityPreferences` on
`SystemTheme` (doc 2 B4 Option B), **not** on
`ResolutionContext`. The two types live at different pipeline
stages:

- `Theme::resolve(Some(&ResolutionContext::from_system()))` →
  `ResolvedTheme` (no accessibility in the resolution step)
- `SystemTheme { light, dark, accessibility, ... }` (accessibility
  carried alongside the resolved variants, consumed by the
  app at render time)

The B5 addendum suggesting `ResolutionContext { accessibility: ... }`
should be reverted to just `{ font_dpi, icon_theme, dialog_button_order }`.

**Confidence:** high. This is a category error; B4's argument
applies recursively.

### J.3 Cross-reference to doc 1 §30.3 M1

Doc 1 §30.3 documents **M1: macOS reader hardcodes wrong
`DialogButtonOrder`** — a verified bug that is tightly coupled
to this document's **D5** recommendation.

**Summary for doc-2 readers:** `native-theme/src/macos.rs:504-505`
hardcodes `Some(DialogButtonOrder::PrimaryLeft)` with the comment
"macOS uses leading affirmative (OK/Cancel)." Both value and
comment contradict:
- `platform-facts.md:1481,1802` ("macOS primary rightmost" ✅ Apple HIG)
- `macos-sonoma.toml:254,586` (`button_order = "primary_right"`)
- `macos-sonoma-live.toml:126,285` (same)
- `resolve/inheritance.rs:98-109` (`platform_button_order()` returns
  `PrimaryRight` on non-Linux)

**Implication for D5:** D5 as drafted proposes deleting KDE's
reader-side `button_order` hardcode. The same architectural
principle applies symmetrically to macOS, and deleting
`macos.rs:504-505` *also fixes the verified bug* because the
preset value (`primary_right`) then propagates correctly through
the pipeline merge.

**Action item for D5:** when D5 lands, extend it to cover
`native-theme/src/macos.rs:504-505` as well. Ship D5 + M1
bundled as a single commit: **"delete reader-side `button_order`
hardcodes on all platforms; presets + resolver are authoritative."**

See **doc 1 §30.3** for the full M1 options table, rationale,
and three-source cross-verification.

### J.4 Strengthened recommendations under "no backward compat"

Consistent with doc 1 §30.4:

#### B1 + B2 + B7: promote codegen to P0/P1 for v0.5.7

Doc 1 §30.4 argues the registry-driven codegen path should be
committed to in v0.5.7 rather than deferred. The blast radius
(§I5's ~215 literals + ~450 lines `check_ranges` + ~280 lines
`require()` + ~100 lines inheritance duplication + 108-line
macro = ~1100 lines at drift risk) justifies the investment.

**Strengthened recommendation:** promote doc 2 **B1 + B2 + B7**
from P1 to **P0/P1 firm commitment** for v0.5.7, paired with
doc 1 §2/§14.

**Minimum viable version:** ship registry-driven codegen for
`check_ranges`, `FIELD_NAMES`, and `lint_toml` tables only;
defer widget struct generation to v0.5.8. This eliminates the
~215 `lint_toml` literals and ~450 lines of `check_ranges`
boilerplate — the worst drift hazards — without the larger
struct-generation work.

**Fallback:** doc 1 §14 Option F (`inventory` crate) as a
~20-line bridge that unblocks the `lint_toml` drift hazard
alone if even minimum-viable codegen is too large.

#### B5: hard-pair with doc 1 §3 as a single ship-unit

Doc 1 §30.4 makes this mandatory. Under "perfect API," split
landing of §3 and B5 forces a double-edit of `OverlaySource`
(first with standalone `font_dpi: f32`, then with
`ctx: ResolutionContext`). The combined single-PR diff is
smaller than two separate passes.

**Strengthened recommendation:** merge doc 1 §3 + doc 2 B5
into a single "`OverlaySource + ResolutionContext` refactor"
ship-unit, tagged **P0** for v0.5.7.

#### A3: fold into doc 1 §6's 4-variant hierarchy (Option E)

Doc 1 §30.2 introduces Option E for §6: a 4-variant category
hierarchy (`Error::Platform | Parse | Resolution | Io`) with
nested sub-enums. Doc 2 A3 (missing_fields dual-category)
folds naturally into this shape as:

```rust
Error::Resolution(ResolutionError::Incomplete { missing: Vec<FieldPath> })
Error::Resolution(ResolutionError::Invalid    { errors:  Vec<RangeViolation> })
```

**Strengthened recommendation:** A3's fix *requires* doc 1 §6
to land in the hierarchy form. If doc 1 §6 stays flat,
A3 falls back to `Error::ResolutionIncomplete` and
`Error::ResolutionInvalid` as separate top-level variants.
Under "perfect API" the hierarchy is recommended.

### J.5 Confidence statement

**High confidence** on:
- J.2 B3 `Arc<str>` for `name` is the right consistency move —
  matches C4 / D4 under a single ownership-type refactor.
- J.2 B5 `for_tests()` naming rule — runtime-detected types
  must not have silent `Default` implementations.
- J.2 B4 refinement: `AccessibilityPreferences` belongs on
  `SystemTheme`, not in `ResolutionContext` (category error
  otherwise).
- J.3 M1 cross-reference: the macOS reader bug is verified and
  bundles cleanly with D5.
- J.4 strengthenings: all three are direct implications of
  "perfect API" + the earlier merge-reviewed analysis.

**Medium confidence** on:
- J.2 B3 Option G (`ThemeReader` trait) is desirable but not
  urgent; F alone is the critical B3 fix.
- J.4 minimum-viable codegen scope fits v0.5.7 schedule.

**Deferred / out of scope:**
- A potential `windows.rs:517` `button_order` question (modern
  WinUI vs classic Win32 tension in platform-facts) — needs a
  platform-facts audit, not an API change. See doc 1 §30.3's
  "out-of-scope note on Windows."
- Doc 1 §30.2 §13 Option G (remove caching entirely) — v1.0
  crate-split scope.

### J.6 What this pass did NOT change

Deliberately preserved from earlier passes:

- Every existing pros/cons entry in doc 2's A-I option tables
  and their merge-review additions.
- Every existing recommendation not explicitly strengthened or
  refined in J.2 / J.4.
- Doc 2's §F open questions list (no items removed).
- Doc 2's A1 STATUS block (A1 remains fully shipped and out of
  v0.5.7 scope).
- Doc 2's §G post-script ("two codebases pretending to be one")
  — the v1.0 crate-split direction is reaffirmed, not modified.

If section J does not reference a prior recommendation, it is
unchanged from the earlier-pass analysis.

---

## K. Fourth-pass review: merged critical refinements

This section records a fourth ultrathink pass performed under the
explicit "backward compatibility does not matter; I want the
perfect API" directive. It runs parallel to **doc 1 §31**, which
hosts doc-1-specific refinements that this document
cross-references here.

Every prior doc-2 claim was re-verified against the current tree
and remains accurate. This section records only the refinements
that change recommendations or surface new options; unchanged
items are not repeated.

### K.1 Verification pass (re-confirmed)

Personal verification this pass against the current tree:

- **A1** `watch/kde.rs:54-68` — still uses `Option<Instant>` +
  `is_none_or`, exactly as the STATUS block describes. No
  v0.5.7 work needed. ✅
- **A2** `resolve/validate.rs:428-458` — **personally verified**
  the orchestration pattern. Lines 428-452 run 24 `check_ranges`
  calls passing `&mut missing` (the same vec populated by
  earlier `require()` calls). Line 454 checks `if
  !missing.is_empty()` *after* `check_ranges`. The bug is
  reproducible: `require()` substitutes `T::default()` and
  pushes a "missing" entry; the subsequent `check_ranges` then
  runs on the placeholder and can push *additional* entries
  for range violations of the placeholder. For
  `font.weight: u16` with `u16::default() == 0` and range
  `100..=900`, a spurious range error is produced alongside the
  legitimate missing-field error. **Bug verified.**
- **A3** `error.rs:9-12` + `validate.rs:429-452` — personally
  verified that `missing_fields: Vec<String>` holds both
  missing-field paths and range-violation strings. The
  doc-level claim ("fields that remained None") contradicts
  the actual contents. **Bug verified.**
- **A4** `resolve/mod.rs:20-22` (doc says "pure data transform")
  + `inheritance.rs:164-167` (`resolve_safety_nets` sets
  `button_order` via `platform_button_order()`) +
  `inheritance.rs:98-109` (`platform_button_order` reads
  `xdg_current_desktop()`) + `detect.rs:28-30`
  (`xdg_current_desktop` is `std::env::var("XDG_CURRENT_DESKTOP")`)
  — **personally verified the full chain**. The "pure transform"
  claim is false: `resolve()` does reach into env vars through
  two levels of indirection. Architectural leak confirmed.
- **B1** `resolve/validate.rs` (280 lines of `require()` calls)
  + `model/widgets/mod.rs:908-1347` (~450 lines of per-widget
  `check_ranges` impls) — verified. Drift hazard is real.
- **B4** `model/defaults.rs:131-140` four accessibility fields
  — verified. Mixing of user-preference state with theme data
  confirmed.
- **B6** `model/border.rs:13-35` — verified `corner_radius_lg`
  and `opacity` are marked as "defaults only" in inline
  comments but structurally allowed at widget level.
  Type-vs-usage mismatch confirmed.
- **C1** `watch/mod.rs:65-70` — `ThemeChangeEvent::Other` is
  defined but grep shows zero production emissions. Only
  appears in `watch/mod.rs:252` test. Dead variant confirmed.
- **C2** `watch/kde.rs:66-69` — KDE watcher fires on *any*
  `kdeglobals` or `kcmfontsrc` change, not just color. Name
  `ColorSchemeChanged` is a lie on KDE. **Personally verified
  the file-level event pattern.** Same conclusion for GNOME
  portal watcher at `watch/gnome.rs:42-62` which fires on any
  `org.freedesktop.appearance` key change (color-scheme,
  accent-color, contrast).
- **I1** `presets.rs:85-88` still contains the stale comment
  ("Errors are stored as String because Error is not Clone")
  while `error.rs:80` still derives `Clone`. The contradiction
  is preserved in the current tree. **Verified drift.**
- **I2** `error.rs:55-65` hint message still references
  `ThemeSpec::from_toml_with_base()` — verified. If §15a
  removes the method (doc 1), this hint must be updated
  simultaneously (captured in doc 1 §15 merge-review).

**Additional verification performed this pass:**

- `Cargo.toml:14-34` feature matrix — personally counted 15
  features, of which 4 are runtime-variant duplicates. See
  **doc 1 §31.3** for the new issue this surfaces.
- `Cargo.lock` shows `inventory` and `strum` already in the
  workspace dep graph via connector crates. Direct
  `native-theme` dep tree (`cargo tree -p native-theme`) shows
  only `serde`, `serde_with`, `toml`, and platform deps — i.e.
  `syn` / `quote` / `proc-macro2` are already transitive via
  `serde_derive`, so a new `native-theme-derive` proc-macro
  crate for doc 1 §31.2 K reuses them at zero new-dep cost.

### K.2 Cross-reference to doc 1 §31 refinements

Doc 1 §31 introduces four option letters that directly affect
doc 2 recommendations. The cross-references below explain how
doc 2 items update when the doc 1 refinements land.

#### A3 now folds into doc 1 §31.2 Option F (flat + `kind()`), not §30.2 E

A3's recommendation (D: fold into doc 1 §6's restructure) was
predicated on §6 adopting **§30.2 Option E** (4-variant
hierarchy). Doc 1 §31.2 **supersedes §30.2 E with Option F**
(flat variants + `kind()` method matching `std::io::Error`).

Under F, A3's fix becomes:

```rust
#[non_exhaustive]
pub enum Error {
    // ... other variants ...
    /// Resolution left required fields unfilled.
    ResolutionIncomplete { missing: Vec<FieldPath> },
    /// Resolution produced values outside their valid ranges.
    ResolutionInvalid { errors: Vec<RangeViolation> },
    // ... other variants ...
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        match self {
            // ...
            Error::ResolutionIncomplete { .. } => ErrorKind::Resolution,
            Error::ResolutionInvalid { .. }    => ErrorKind::Resolution,
            // ...
        }
    }
}
```

Two flat top-level variants (not nested under
`Error::Resolution(ResolutionError::Incomplete | Invalid)`), both
mapping to `ErrorKind::Resolution` via `kind()`. Callers that
want "handle all resolution errors" do `if err.kind() ==
ErrorKind::Resolution { ... }`. Callers that want a specific
variant do flat matching with no extra depth.

This is strictly better than A3's original nested form because:
- Variant matching stays 1-depth (was 2-depth under §30.2 E)
- Category matching is one method call (was one match-depth
  under §30.2 E, which is equivalent)
- Matches `std::io::Error::kind()` precedent

**Updated A3 recommendation:** ship two flat variants
(`ResolutionIncomplete`, `ResolutionInvalid`) as part of the
doc 1 §31.2 F `Error` restructure. The sub-enum form from
A3's original Option D sketch is superseded.

**Confidence:** high — this is a mechanical consequence of the
doc 1 §6 shape change.

#### B1 + B2 + B7: ship via doc 1 §31.2 K (narrow derive proc-macro), not §2 D

Doc 1 §31.2 **supersedes the registry-driven §2 Option D with
Option K**: a narrow `#[derive(ThemeLayer)]` proc-macro that
reads Rust struct definitions directly. This changes the doc-2
codegen path materially:

**Previous plan (§J.4 "promote B1+B2+B7 to P0/P1"):**
- Build `native-theme-build` with external TOML registry
- Design schema for fields, ranges, inheritance, border_kind
- Codegen Rust from TOML
- Migrate widgets gradually

**Updated plan (via doc 1 §31.2 K):**
- Build `native-theme-derive` proc-macro crate
- Define attribute syntax on existing Rust struct definitions
  (`#[theme(required, range = "...")]`,
  `#[theme_layer(border_kind = "full")]`, etc.)
- Macro emits paired Resolved struct, `FIELD_NAMES`,
  `impl_merge!` body, `check_ranges` impl, and
  `inventory::submit!` for §14 widget registry
- No external TOML, no build script, Rust-only pipeline

**What K delivers for doc 2:**

| Doc 2 item | Delivered by K? |
|---|---|
| **B1** ~720 lines of validate/check_ranges boilerplate | **Yes** — generated from `#[theme(required, range = "0.0..=1.0")]` attributes |
| **B2** inheritance rules duplicated TOML ↔ Rust | **Partial** — simple per-field inheritance via `#[theme(inherit_from = "...")]` attribute; pattern-based rules (e.g. "all widgets inherit `border.color` from `defaults.border.color`") still need `inheritance.rs` unless the attribute DSL is extended |
| **B6** `BorderSpec` split into defaults vs widget | **Yes** — via `border_kind = "full" / "partial" / "none"` attribute per widget |
| **B7** three parallel border-validation paths | **Yes** — generated from `border_kind` per widget, collapses the three hand-coded functions |

**The B2 partial coverage is the main risk.** If inheritance rules
cannot be expressed via per-field attributes, that portion of the
codegen must fall back to one of:
- Reading `inheritance-rules.toml` inside the proc-macro
  (degenerating toward doc 1 §30.2 H)
- Keeping `inheritance.rs` hand-maintained for the complex rules,
  auto-generating only the simple per-field inheritance

Either fallback is acceptable. The important part is that B1,
B6, B7 are fully deliverable via K.

**Updated recommendation for doc 2 B1+B2+B7:** ship as part of
the doc 1 §31.2 K codegen crate. Fold B1 and B6+B7 into the
proc-macro derive. Fold B2 into the derive to the extent
attributes allow; keep `inheritance.rs` for the residue.

**Priority:** B1 + B6 + B7 promote to **P1** for v0.5.7 (via
K). B2 stays P1 but with partial delivery acknowledged.

**Confidence:** high on B1 + B6 + B7 delivery via K. Medium on
B2 (depends on attribute expressiveness, same unknown as doc 1
§31.2 K's main flag).

#### B5 `ResolutionContext` pairs with doc 1 §31.2 via §30 refinement chain

Doc 1 §30.4 already mandates the §3 + B5 pairing as a single
ship-unit. Doc 1 §31 does not change this pairing; it only
refines the surrounding context (error shape via §31.2 F, codegen
via §31.2 K). B5's `ResolutionContext` recommendation stands as
written in J.2, with the constructor naming rule preserved
(`from_system()` + `for_tests()`, no `Default` impl).

**No change to B5.** Ship bundled with doc 1 §3 as a single
commit.

### K.3 B3 `ReaderOutput::name` consistency with §31.2 ownership types

Doc 2 J.2 "B3 refinement" proposes `Arc<str>` for
`ReaderOutput::name`, matching C4's `Arc<str>` direction for
`ResolvedFontSpec::family`. Doc 1 §31 does not introduce
competing ownership-type recommendations, so J.2's refinement
stands without change.

**Confirmation under "perfect API":** the single ownership-type
refactor (C4 + B3 + D4 + doc 1 §20 `icon_theme`) via `Arc<str>`
with `serde = { features = ["rc"] }` remains the correct path.
Adopt uniformly across the crate.

**Post-script 2026-04-20 — principled deviation per `docs/todo_v0.5.7_gaps.md` §G9:**

"Adopt uniformly across the crate" was softened in the G9 audit
(recorded in `docs/todo_v0.5.7_gaps.md:449-506`). The uniform
`Arc<str>` path was adopted ONLY for `ResolvedFontSpec::family`
(real dedup across 26 widgets × connectors). For `SystemTheme::name`,
`SystemTheme::icon_theme`, `Theme::name`, and `ThemeDefaults::icon_theme`
the audit demonstrated no dedup benefit, and `Cow<'static, str>` was
retained to preserve the zero-allocation fast path on bundled
`&'static str` preset literals. See §J.2 B3 refinement post-script
above for the symmetrical record and `docs/todo_v0.5.7_gaps.md` §G9
for the definitive audit.

### K.4 Confirmation of the macOS M1 bug (doc 1 §30.3)

Doc 1 §30.3 surfaces M1: `native-theme/src/macos.rs:504-505`
hardcodes `Some(DialogButtonOrder::PrimaryLeft)` with an
incorrect comment ("macOS uses leading affirmative"). The bug
contradicts three independent sources.

**Personal verification this pass (doc 2 side):**

| Source | Says | Verified location |
|---|---|---|
| `macos.rs:505` (the bug) | `PrimaryLeft` | ✅ personally read |
| `presets/macos-sonoma.toml` | `button_order = "primary_right"` (lines 254, 586) | ✅ `grep` confirmed |
| `presets/macos-sonoma-live.toml` | `button_order = "primary_right"` (lines 126, 285) | ✅ `grep` confirmed |
| `docs/platform-facts.md:1468-1481` | macOS column in dialog table reads "primary rightmost" | ✅ personally read, including table header |
| `resolve/inheritance.rs:98-109` | `platform_button_order()` returns `PrimaryRight` on non-Linux | ✅ personally verified |

**All three non-bug sources agree on `PrimaryRight`.** The reader
hardcode is the sole disagreement. User-visible impact: every
`SystemTheme::from_system()` on macOS returns a `SystemTheme`
with wrong dialog button order.

This bundles cleanly with **D5** (delete KDE reader
`button_order` hardcode): both issues are resolved by deleting
reader-side `button_order` hardcodes on *all* platforms and
letting presets + resolver be authoritative.

**Endorsement:** ship M1 (2-line deletion + comment fix) + D5
(2-line deletion) as a single commit titled "delete reader-side
`button_order` hardcodes; presets + resolver are authoritative."

**Update to D5 recommendation:** the "depends on A4" caveat in
D5's original text is too cautious. D5 (delete KDE reader
hardcode) and A4 (move `button_order` fallback from
`resolve_safety_nets` to `resolve_platform_defaults`) are
**independently correct**. Ship D5 + M1 unconditionally. Ship
A4 independently (it fixes the `resolve()` purity leak, which
is a separate architectural concern).

### K.5 Windows platform-facts tension (out of scope for v0.5.7)

Doc 1 §30.3 "Out-of-scope note on Windows" flags that
`windows.rs:517` hardcodes `PrimaryRight`, which agrees with
the Windows presets and the resolver default — but
`platform-facts.md:1481` lists Windows under "primary
leftmost" per an older Microsoft Common Buttons guideline.

**Personal verification this pass:** `platform-facts.md:1481`
confirmed reading "primary leftmost" for Windows in the dialog
table. The Windows reader, presets, and resolver default all
agree on `PrimaryRight`. Only `platform-facts.md` disagrees,
and its citation is the older Win7-era guideline.

Modern WinUI 3 `ContentDialog` uses primary-right. The Windows
code tracks modern practice; `platform-facts.md` tracks the
older guideline. This is a **documentation vs. modern-practice
tension**, not a code bug.

**No v0.5.7 action recommended.** Platform-facts requires an
independent refresh citing current MS Style Guide /
WinUI 3 `ContentDialog` documentation. Flagged for a future
platform-facts audit.

### K.6 Priority rebalance (doc 2 updates)

Updates to doc 2 §E / §H in light of doc 1 §31:

**Promotions (new or strengthened):**

| Priority | Issue | Change |
|---|---|---|
| P0 | **A3** as two flat variants in doc 1 §31.2 F | Superseded prior "nested under Resolution(...)" form |
| P1 | **B1 + B6 + B7** ship via doc 1 §31.2 K | Was §J.4 "promote to P0/P1 via codegen"; now concretely "via narrow derive proc-macro K". Scope fits v0.5.7. |
| P1 | **B2** ship partial delivery via K | Partial: simple inheritance via attributes, pattern rules stay in `inheritance.rs` until a follow-up |

**No changes to:**

- A2 (personally verified bug; fix is mechanical)
- A4 (move `button_order` to `resolve_platform_defaults`)
- B3 (`ReaderOutput` with `known_is_dark: Option<bool>`)
- B4 (`AccessibilityPreferences` on `SystemTheme`, not in
  `ResolutionContext`, per J.2)
- B5 (bundled with doc 1 §3)
- C1 / C2 / C3 / C4 / C5 / C6 (unchanged recommendations)
- D1-D5 (D5 strengthened by K.4 bundling with M1; others unchanged)
- I1-I5 (all still relevant)

### K.7 Confidence statement (fourth pass)

**High confidence:**
- All K.1 verification results (personally re-checked against
  the current tree)
- K.2 A3 shape update (follows mechanically from doc 1 §31.2 F)
- K.2 B1 + B6 + B7 delivery via doc 1 §31.2 K (direction is
  right; scope is medium confidence)
- K.4 M1 bundling with D5 unconditionally

**Medium confidence:**
- K.2 B2 partial delivery via K (depends on attribute-syntax
  expressiveness for inheritance rules; same unknown as doc 1
  §31.2 K's main flag)
- K.6 priority promotions (depends on K landing cleanly within
  v0.5.7 schedule)

**Low confidence / explicit unknowns:**
- Whether `#[theme(inherit_from = "defaults.border.color")]`
  attributes can express *all* of `inheritance-rules.toml`'s
  current rules, or whether pattern-based rules (e.g. "every
  widget inherits border.color") need a different mechanism

### K.8 What this pass did NOT change

Deliberately preserved from §A-§J:

- Every pros/cons entry in doc 2's existing option tables
- Every recommendation not explicitly superseded in K.2 / K.6
- §G post-script (v1.0 crate split direction reaffirmed)
- §H cross-document P0 consolidation (updated only to replace
  the A3 shape; everything else in §H stands)
- §F open questions list (no items removed; K.7 adds one new
  implicit unknown about attribute-syntax expressiveness)
- Doc 2's A1 STATUS block (A1 remains fully shipped)

If section K does not reference a prior recommendation, that
recommendation is unchanged.

### K.9 Endorsement of the doc 2 P0 cohort for v0.5.7

The fourth-pass review personally verified every doc 2 P0 item
and endorses them for v0.5.7 without reservation:

1. **A2** — verified orchestration bug at `validate.rs:428-458`
2. **A3** (in doc 1 §31.2 F shape) — verified dual-category
   pollution in `ThemeResolutionError.missing_fields`
3. **A4** — verified `resolve()` doc-vs-code contradiction
4. **B4** — verified accessibility pollution in `ThemeDefaults`
5. **B6** — verified type-vs-usage mismatch in `BorderSpec`
6. **C1** — verified `Other` has zero emitters
7. **C2** — verified `ColorSchemeChanged` lies on KDE / GNOME
8. **D5 + M1** bundled (per K.4) — verified both hardcodes
   against three independent sources each

These are the ship-critical items from doc 2. Combined with
doc 1's P0 cohort (see doc 1 §31.10), they form a coherent
v0.5.7 release.

---

## L. Fifth-pass review: new issues and refinements

This section records a fifth ultrathink pass under the explicit
"backward compatibility does not matter; I want the perfect API"
directive. It runs parallel to **doc 1 §32** which hosts
doc-1-specific fifth-pass findings.

This pass surfaced **four new issues** (L1–L4) not in any prior
A–K section, re-verified A2/A3's shared-vec claim through a
specific source-level self-admission that earlier passes did not
quote, added coordination notes for the M1 + D5 + A4 atomic-commit
bundling (previously only M1 + D5 were bundled), and proposed a
detailed eleven-ship-unit sequencing plan for the v0.5.7 release.

### L.1 Methodology

Same approach as §J and §K: independently re-read each existing
claim against the current tree, record only refinements that
change recommendations or surface new items, and preserve all
earlier content unchanged.

New findings come from:

1. Personal re-read of `native-theme/src/resolve/validate_helpers.rs:217-282`
   (the `check_*` helpers) — turned up a source-level self-admission
   of the A2/A3 conflation that earlier passes did not quote.
2. Grep-based search for stale `Clone`-justification doc comments
   across `error.rs` — turned up a second stale comment beyond I1.
3. Re-read of `Cargo.toml:14-34` under §31.3's lens — turned up an
   aggregator-coupling issue separate from the runtime-variant
   duplication.
4. Re-read of C6 demotion recommendation under the lens of `pub`
   module vs feature-gated re-export — turned up a visibility
   audit step that C6 does not mention.
5. Re-read of A4 fix implications on tests — turned up a
   `test_util::ENV_MUTEX` simplification follow-up.

### L.2 New issues

#### L1. `error.rs:73-79` is a second stale `Clone` justification (pair with I1)

**File:** `native-theme/src/error.rs:73-79`

**The comment:**

```rust
/// Errors that can occur when reading or processing theme data.
///
/// This type is [`Clone`] so it can be stored in caches alongside
/// [`crate::ThemeSpec`]. The `Platform` and `Io` variants use [`Arc`]
/// internally to enable cloning without losing the original error chain.
///
/// [`Arc`]: std::sync::Arc
```

This doc comment justifies the `Clone` derive on `Error` by
pointing at a cache that never actually used `Clone` (the cache
stores `Result<ThemeSpec, String>` per I1's find at
`presets.rs:85-88`). **I1 catches the `presets.rs` comment but
not this one.**

Doc 1 §6a's recommendation to drop `Clone` therefore requires
deleting **three** stale items, not two:

1. `native-theme/src/error.rs:73-79` — this doc comment (**L1**)
2. `native-theme/src/error.rs:239-250` — `error_is_clone` test
   (cross-referenced from doc 1 §32.3 §6)
3. `native-theme/src/presets.rs:85-92` — stale comment (**I1**,
   plus cache type migration)

All three live in the same commit as §6a's Clone drop. Doc 1
§32.3 §6 documents this as a four-item commit (three deletions
plus the actual Clone removal on `error.rs:80`).

**Options:**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** (keep the comment alongside §6a) | No change | Doc lies about why Clone exists; future maintainer will re-justify Clone on a false basis |
| B | **Delete the comment entirely** when §6a lands | Minimum cleanup; aligns with I1's `presets.rs` deletion; consistent with doc 1 §32.3 §6 four-item checklist | None |
| C | **Rewrite the comment** to describe the new shape (e.g. "Error is `#[non_exhaustive]`; add new variants via minor release") | Documentation gains context | More work; fills space for a self-documenting pattern |

**Recommendation:** **B** (delete). The `#[non_exhaustive]` is
already documented by the attribute itself. Any replacement comment
would be filler. Ship as part of the §6a four-item commit.

**Confidence:** high.

#### L2. `test_util::ENV_MUTEX` test simplification follow-up post-A4

**Files:**
- `native-theme/src/test_util.rs` (defines `ENV_MUTEX`)
- Multiple test sites that import and use it

**The situation:**

A4's recommendation moves `button_order` fallback out of
`resolve_safety_nets` into `resolve_platform_defaults`. One
consequence: `resolve()` becomes genuinely pure — no env var
reads, no OS interaction. Tests that exercised `resolve()` and
needed to serialize `XDG_CURRENT_DESKTOP` state via `ENV_MUTEX`
no longer need that serialization.

Concretely, tests that match the pattern:

```rust
let _guard = test_util::ENV_MUTEX.lock().unwrap();
std::env::set_var("XDG_CURRENT_DESKTOP", "KDE");
let mut variant = ...;
variant.resolve();  // now pure; env var read is gone
// assert on variant.dialog.button_order
```

are broken by A4 in a specific way: the test's intent was *"set
env var, call resolve, observe env-driven behavior."* Post-A4,
`resolve()` does not read the env var, so the set_var is dead
code. The test must either:
1. Switch to calling `into_resolved()` (which still goes through
   `resolve_platform_defaults` and reads the env var), OR
2. Drop the env var setup entirely and assert on a different
   invariant.

Either way, the `ENV_MUTEX` lock may no longer be needed for
tests in category (2). This is not a bug — A4 changes the
semantic contract of `resolve()`, and the tests should reflect
that — but it is a cleanup opportunity the A4 plan does not
mention.

**Options:**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** (ship A4, leave tests unchanged) | No follow-up work | Tests retain unnecessary `ENV_MUTEX` locks; future maintainers cannot tell which tests actually need the mutex and which do not; the `ENV_MUTEX` semantic ("serializes env var access") becomes "sometimes serializes, sometimes not, no way to tell" |
| B | **Audit and simplify tests post-A4** as a follow-up commit: remove `ENV_MUTEX` locks from tests that no longer manipulate env vars; keep the mutex only for tests that deliberately exercise env-dependent code paths (`into_resolved()`, `platform_button_order()` inside `resolve_platform_defaults`) | Tests become clearer; `ENV_MUTEX` semantics become "actually needed when present"; future maintainers know what they are looking at | One follow-up commit; audit work |
| C | **Delete `ENV_MUTEX` entirely if no tests need it post-A4** | Smallest test-util surface | Requires confirming zero remaining use sites; if even one test still needs env-var serialization the mutex must stay |

**Recommendation:** **B**. Ship A4 as part of the M1 + D5 + A4
atomic commit (doc 1 §32.4). Then ship a **follow-up cleanup
commit** in the same release: *"post-A4: simplify tests that no
longer need `ENV_MUTEX` serialization."* Do **not** bundle the
test audit with A4 itself — the A4 bundle already touches four
files (`macos.rs`, `kde/mod.rs`, `inheritance.rs`, `resolve/mod.rs`),
and adding a cross-tree test audit makes the commit harder to
review.

**Priority:** tag as **P1 follow-up** in §E and doc 1 §27. Ships
in v0.5.7 alongside A4 but as a separate commit.

**Confidence:** high on the direction (A4 clearly makes some
tests redundant). Medium on the exact count of affected tests —
requires running the grep after A4 lands to get the real number.

#### L3. `kde::from_kde` visibility audit prerequisite for C6

**Files:**
- `native-theme/src/kde/mod.rs:341` (`pub fn from_kde`)
- `native-theme/src/lib.rs:171-172` (conditional re-export)
- Possible consumers in the workspace

**The situation:**

C6 recommends demoting platform readers (including `from_kde`)
from `pub` to `pub(crate)`. The function is declared `pub fn
from_kde` inside the `kde` module, which is itself feature-gated.
The crate-root re-export at `lib.rs:172` is behind
`#[cfg(all(target_os = "linux", feature = "kde"))]`.

The audit C6 does not mention: **are there any `use
crate::kde::from_kde` paths inside the crate or workspace that
would break when the function becomes `pub(crate)`?** Usually
`pub(crate)` is a no-op for intra-crate callers. But integration
tests under `native-theme/tests/` and the connectors under
`connectors/` are *external* crates that can only see items
declared `pub`. If any of them access `native_theme::kde::from_kde`
(or the other readers), the C6 demotion breaks them.

This is the same kind of audit doc 1 §31 B' performs implicitly
via `#[doc(hidden)] pub`.

**Options:**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Demote without audit** | Minimum work | Breaks integration tests or connectors if any use the functions; surfaces as a compilation failure downstream |
| B | **Audit first, then demote** per audit result | Confirmed safety | One-time grep pass (~1 minute) |
| C | **Use `#[doc(hidden)] pub`** instead of `pub(crate)` (same pattern as doc 1 §31 B' for §7) | Downstream callers keep working; rustdoc discoverability still hidden | Not as strict as `pub(crate)`; item is still technically in the SemVer contract |

**Recommendation:** **B audit, then A or C based on audit result.**
Concretely:

1. Run `grep -rn 'native_theme::kde::from_kde\|use native_theme::kde::\|crate::kde::from_kde' native-theme/ connectors/` across the workspace root.
2. If zero hits outside `native-theme/src/pipeline.rs`: ship **A**
   (plain `pub(crate)`).
3. If any hit in `native-theme/tests/`: ship **C** (`#[doc(hidden)]
   pub`) to preserve integration tests.
4. If any hit in `connectors/`: that is a real downstream use case.
   Do **not** demote. Instead, re-expose the function via a
   signposted `native_theme::readers::*` module per doc 1 §12's
   module partition, with `#[doc = "Raw platform reader output;
   not for normal use — prefer SystemTheme::from_system()"]`.

Apply the same audit pattern to `from_macos` (`macos.rs:397`),
`from_windows` (`windows.rs:578`), `from_gnome` (`gnome/mod.rs:280`),
`build_gnome_spec_pure` (`gnome/mod.rs:280`), and
`from_kde_with_portal` (`gnome/mod.rs`).

**Rationale:** C6 is a straightforward demotion *only if nothing
outside the pipeline uses the functions*. The audit step is a
~1-minute grep that either confirms safety (ship A) or surfaces
a real consumer (ship C or re-expose). Either way, the outcome
is better than "demote and hope."

**Priority:** audit is a prerequisite to C6 (itself P1). Tag
audit as **pre-C6 checklist item**, not its own priority tier.

**Confidence:** high on the need for the audit; the exact outcome
(A, C, or re-expose) depends on what the grep finds.

#### L4. `Cargo.toml` `linux` aggregator couples KDE and portal

**File:** `native-theme/Cargo.toml:14-24`

**The situation:**

```toml
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
```

The `linux` aggregator forces both `kde` AND `portal-tokio`
together. Consequences:

1. A user wanting portal-only (GNOME via xdg-desktop-portal)
   without KDE's `configparser` dep cannot use the `linux`
   convenience aggregator.
2. A user wanting KDE-only (reading `kdeglobals` directly without
   the portal) cannot use `linux` either — they get `portal-tokio`
   forced on them.
3. When §31.3 lands (dropping the four `-async-io` variants via
   §5 G), the `linux` definition collapses to `linux = ["kde",
   "portal"]`, but the KDE↔portal coupling remains unchanged.

The user's actual *"I want Linux theme detection"* intent is
ambiguous: does it mean "all Linux desktops including KDE" or
"the most common Linux desktops (GNOME via portal)"? Both are
reasonable. The current `linux = ["kde", "portal-tokio"]` picks
the first reading and forces users who wanted the second to
explicitly enable `portal` only.

This issue is independent of §31.3 (runtime-variant redundancy)
but is in the same category: *feature-aggregator coupling should
minimise unnecessary dependency bundling under "perfect API".*

**Options:**

| # | Option | Pros | Cons |
|---|---|---|---|
| A | **Status quo** (after §31.3 drops `-async-io`): `linux = ["kde", "portal"]` | No additional change | KDE↔portal coupling persists |
| B | **Split into sub-aggregators**: `linux-kde = ["kde"]`, `linux-portal = ["portal"]`, `linux = ["linux-kde", "linux-portal"]` | Explicit; users can pick `linux-kde` or `linux-portal` alone AND still have `linux` as a meta-aggregator | Three features in the matrix for what was one; users see more feature names |
| C | **Drop the `linux` aggregator entirely**; users write `features = ["kde", "portal"]` explicitly | Cleanest possible surface; zero aggregator coupling | Loses the `--features linux` convenience |
| D | **Redefine `linux` as "portal only"** (the minimum most Linux desktops need); add `linux-full = ["kde", "portal"]` for everything | Matches the common user intent | Silent semantic shift on an existing feature name — users who currently rely on `linux` for KDE detection get a silent break under "no backward compat" this is allowed, but the shift is still surprising |
| E | **Rename `linux` to `linux-full`**; add a new `linux-portal = ["portal"]` convenience. Forces explicit choice. | No silent semantic shift; explicit naming | `linux-full` is slightly awkward; users still need to learn a new name |
| F | **Status quo + rustdoc feature guide** that documents when to pick `kde`, `portal`, or `linux` | Minimum code change | Doesn't fix the underlying coupling; users still face the choice on every `cargo add` |

**Recommendation:** **B** bundled with §31.3's `-async-io`
cleanup. Final feature list after §5 G + §31.3 + L4:

```toml
kde            = ["dep:configparser"]
portal         = ["dep:ashpd"]          # single runtime after §5 G
windows        = ["dep:windows"]
macos          = [...]
linux-kde      = ["kde"]
linux-portal   = ["portal"]
linux          = ["linux-kde", "linux-portal"]
native         = ["linux", "macos", "windows"]
watch          = ["dep:notify", "dep:zbus"]
material-icons = []
lucide-icons   = []
svg-rasterize  = ["dep:resvg"]
system-icons   = [...]
```

**13 features** total (was 15). No runtime-coupling redundancy.
Sub-aggregators allow mix-and-match.

Usage patterns:
- Everything on Linux: `features = ["linux"]` (convenience unchanged)
- KDE only: `features = ["linux-kde"]` or `features = ["kde"]` (equivalent)
- Portal only: `features = ["linux-portal"]` or `features = ["portal"]` (equivalent)
- All platforms: `features = ["native"]` (convenience unchanged)

**Rationale:**

- **A** ships the coupling as a known limitation — wrong under
  "perfect API."
- **C** removes a working convenience with no replacement — too
  austere.
- **D** creates a silent semantic shift that breaks user
  expectations in a subtle way (the cargo feature name is the
  same but means something different).
- **E** adds verbosity without the `linux-*` sub-aggregator
  discoverability.
- **F** documents the limitation instead of fixing it.
- **B** delivers the full aggregator convenience AND the
  independent knobs, at the cost of three features instead of
  one. Under "perfect API" the three features buy a real
  decoupling.

**Bundling:** ship L4 with the §31.3 feature-matrix cleanup
(which depends on §5 G). All feature-matrix work lives in one
ship unit.

**Flag for §F:** L4 hard-depends on §5 G the same way §31.3
does. If §5 is rejected, L4 falls back to **F** (documentation
only) until a clean runtime decoupling is available.

**Confidence:** high on the direction (decoupling aggregators is
a perfect-API win). Medium on the exact naming
(`linux-kde`/`linux-portal` vs alternatives like `linux-files`/
`linux-desktop-portal`).

### L.3 Verification re-confirmations

#### A2/A3 shared-vec with source-level self-admission

The A2/A3 issue was described in earlier passes and verified in
§K.1. This pass found a more pointed piece of evidence at
`native-theme/src/resolve/validate_helpers.rs:217-221` that earlier
passes did not quote:

```rust
// --- Range-check helpers for validate() ---
//
// These push a descriptive message to the `errors` vec (reusing the same
// error-collection pattern as require()) so that all problems -- missing
// fields AND out-of-range values -- are reported in a single pass.
```

**The code itself acknowledges** that:

1. The same vec is reused for both categories (*"same error-collection
   pattern as require()"*).
2. The author intended the two categories to be reported together
   (*"all problems... in a single pass"*).

The doc-level claim at `error.rs:10-11` (*"Dot-separated paths of
fields that remained `None` after resolution"*) **contradicts the
actual implementation contract**. The doc says "fields that
remained None"; the implementation code admits that range
violations also flow through the same vec.

Concretely, `check_positive` at `validate_helpers.rs:233-238`:

```rust
pub(crate) fn check_positive(value: f32, path: &str, errors: &mut Vec<String>) {
    if !value.is_finite() || value <= 0.0 {
        errors.push(format!(
            "{path} must be a finite positive number, got {value}"
        ));
    }
}
```

pushes strings like `"defaults.font.size must be a finite positive
number, got 0"` — a range-violation message format — into the same
vec that `require` populates with bare field paths like
`"defaults.accent_color"`. The two string shapes are different;
the user's error output mixes both.

Also: the parameter name inside the helpers is `errors: &mut
Vec<String>`, while at the call site in `validate.rs:429-452`
the same vec is named `missing`. **The naming mismatch between
call sites is itself evidence of the conflation** — the helper
author thought of it as "errors" while the orchestration author
thought of it as "missing fields."

This is not merely conflation at the type level — it is a
documented design intent at the helper module level that the
public-facing doc does not reflect. A2/A3's fix (split the vec,
make the categories distinct at the `Error` type level via doc 1
§31.2 F) is now doubly justified:

1. The `missing_fields` doc comment at `error.rs:10-11` is false.
2. The `validate_helpers.rs:218-221` comment's "single pass" design
   goal is still achievable **with** the split — multiple vecs
   populated in one pass is still "one pass," just with
   correctly-categorised output.

**No option change.** Reconfirms A2's **B + C** recommendation
(short-circuit on missing + separate vecs) and doc 1 §31.2 F's
`Error::ResolutionIncomplete` + `Error::ResolutionInvalid` fold.

**Confidence:** very high. The source code literally admits the
behaviour described in the doc's claim. The Explore agent that
initially disputed this finding during the fifth-pass verification
was wrong; direct reading of `validate_helpers.rs:217-282` is
unambiguous.

#### B6 `require_border` silently preserves defaults-only fields

**File:** `native-theme/src/resolve/validate_helpers.rs:131-166`

Reading `require_border` directly shows the `Some(b) =>` branch:

```rust
ResolvedBorderSpec {
    color,
    corner_radius,
    corner_radius_lg: b.corner_radius_lg.unwrap_or_default(),  // line 157
    line_width,
    opacity: b.opacity.unwrap_or_default(),                    // line 159
    shadow_enabled,
    padding_horizontal: b.padding_horizontal.unwrap_or_default(),  // line 161
    padding_vertical: b.padding_vertical.unwrap_or_default(),      // line 162
}
```

Four fields are extracted via `.unwrap_or_default()` — the
widget-level border silently *accepts and preserves*
`corner_radius_lg` and `opacity` values even though `border.rs:19,25`
comments say they are "defaults only."

The extraction path does not reject them; it quietly includes
them in the resolved output. **B6's original framing is subtly
wrong in a way that makes the problem slightly worse.**

B6 says: *"Resolution silently discards the values."*

More accurate framing: **resolution silently STORES the values
in `ResolvedBorderSpec`, but no connector reads them at widget
level.**

Looking at `connectors/native-theme-gpui/src/lib.rs` and
`connectors/native-theme-iced/src/lib.rs`, neither currently reads
widget-level border `opacity` — they use `defaults.border.opacity`
only. So a user who writes `[light.button.border] opacity = 0.5`
in their TOML gets:

1. The field parsed successfully by serde.
2. The field preserved through merge.
3. The field extracted into `ResolvedBorderSpec.opacity` via
   `.unwrap_or_default()`.
4. The field stored in the resolved struct.
5. **No visual effect**, because no connector reads it at widget
   level.

This is worse than "silently discarded." A user verifying their
TOML via `println!("{:?}", resolved.button.border)` will see the
value present in the debug output and conclude the feature works,
only to observe no visual change when rendering.

**No option change to B6.** The type-split recommendation
(`DefaultsBorderSpec` + `WidgetBorderSpec`) is still correct
because it eliminates this drift by making the field
unrepresentable at widget level — a missing field is a compile
error, not a silently-stored-but-unused value.

**Strengthening:** the B6 problem statement should be updated to
reflect the "stored but unused" framing. This is a doc
clarification, not a new option.

**Confidence:** high. Verified directly in
`validate_helpers.rs:141-165`.

### L.4 Strengthened recommendations

#### M1 + D5 + A4 mandatory single commit (cross-ref to doc 1 §32.4)

Doc 1 §32.4 strengthens the M1 + D5 + A4 bundle to a mandatory
atomic commit. Doc 2 concurs: these three fixes touch one
conceptual decision (`button_order` provenance) and should ship
together. See doc 1 §32.4 for the full rationale and commit
contents.

**Doc 2 contribution:** the atomic bundle also fixes doc 2 D2's
"weird padding derivation rule" contingent on the B6 type split
— not because D2 touches `button_order`, but because D2 and the
M1/D5/A4 triad are both symptoms of the same architectural
pressure (*readers and resolvers redundantly deciding the same
data*). Shipping M1/D5/A4 atomically establishes the principle;
B6 and D2 then fall out naturally when B6 lands in its own ship
unit.

**No new options.** Strengthens existing D5 and A4 recommendations
to "mandatory atomic bundle with M1."

#### §6a four-item commit (cross-ref to doc 1 §32.3 §6)

Doc 1 §32.3 §6 documents §6a as a four-item commit. L1 (this
section) adds the `error.rs:73-79` doc comment deletion to that
commit. Ship as:

| Item | File:line | Action |
|---|---|---|
| 1 | `error.rs:80` | Remove `Clone` from `#[derive(...)]` |
| 2 | `error.rs:73-79` | Delete the stale doc comment (**L1**) |
| 3 | `error.rs:239-250` | Delete the `error_is_clone` test |
| 4 | `presets.rs:85-92` | Update cache to `Result<ThemeSpec, Error>`; delete stale comment (**I1**) |

Single commit titled *"drop `Error: Clone` bound; remove stale
caching justifications and tests."*

### L.5 Ship-unit sequencing for v0.5.7

Under "perfect API" the following eleven ship units sequence
v0.5.7's P0 + P1 cohort. Each unit is a self-contained commit or
small commit cluster; intra-unit atomicity is enforced where noted.

| # | Priority | Atomic? | Commits | Touches |
|---|---|---|---|---|
| 1 | P0 | **Yes** | M1 + D5 + A4 | `macos.rs`, `kde/mod.rs`, `inheritance.rs`, `resolve/mod.rs` |
| 2 | P0 | **Yes** | A2 + A3 + §6 Option F restructure | `validate.rs`, `validate_helpers.rs`, `error.rs` |
| 3 | P0 | **Yes** | §6a four-item commit (L1 + I1 + test deletion + Clone drop) | `error.rs`, `presets.rs` |
| 4 | P1 follow-up | No (after Unit 1) | **L2** test simplification post-A4 | test files using `ENV_MUTEX` |
| 5 | P0 | No | C1 + C2 | `watch/mod.rs`, backend files |
| 6 | P0 | No (several small commits) | §16, §17, §19, §22, §26 polish | `color.rs`, `model/icons.rs`, `detect.rs`, `watch/mod.rs`, various `#[must_use]` sites |
| 7 | P0 | No (one large refactor commit) | §1 renames + §12 partition + §4 Option G + §20 | lib.rs, model module tree, detect module, connectors |
| 8 | P0 | **Yes** | §3 + doc 2 B5 + B4 (`OverlaySource` + `ResolutionContext` + `AccessibilityPreferences`) | `lib.rs`, `resolve/mod.rs`, `model/defaults.rs`, `model/resolved.rs` |
| 9 | P0 | No | Doc 2 B6 hand-written `BorderSpec` split + **L3** audit for C6 | `model/border.rs`, all `check_*` helpers |
| 10 | P1 | No (one focused commit) | Minimum-viable K codegen (doc 1 §32.3 §2+§14) | new `native-theme-derive` crate, `widgets/mod.rs` |
| 11 | P2 | **Yes** | §5 G + §31.3 + **L4** feature-matrix cleanup | `Cargo.toml`, `lib.rs`, `pipeline.rs` |

**Atomicity notes:**

- **Unit 1** (M1+D5+A4) must be atomic per doc 1 §32.4's
  architectural-principle argument.
- **Unit 2** must be atomic because A2's fix requires the new
  `Error` shape from §6 F; shipping one without the other leaves
  an intermediate state where `validate()` returns a structured
  error into a string-based vec.
- **Unit 3** must be atomic because dropping `Clone` without
  also deleting the test breaks the build.
- **Unit 8** must be atomic because §3's `OverlaySource` shape
  directly embeds (or holds a reference to) the `ResolutionContext`
  from B5. Split landing forces a double-edit.
- **Unit 11** must be atomic because §5 G, §31.3, and L4 are
  three sides of the same feature-matrix cleanup.

**Non-atomic units** can be split into smaller commits if needed.
Unit 6 (polish) can ship as 5 separate commits; Unit 7 can ship as
2–3 commits (rename sweep, module partition, pick/ColorMode
refactor).

### L.6 Priority rebalance

No new P0 items. §L additions slot in as follow-ups or
prerequisites:

**New P1 follow-up (within v0.5.7):**

- **L2** test simplification after A4 — small, one commit, Unit 4
  in the sequencing.

**New P2 items:**

- **L4** feature aggregator split — bundled with §31.3 (also P2)
  and §5 G, Unit 11.

**New prerequisite (not its own tier):**

- **L3** audit — grep pass before C6's demotion, Unit 9 pre-step.

**B6 priority correction:**

- Doc 1 §32.3 §2+§14 scope cut means B6 ships **hand-written in
  v0.5.7** (Unit 9), codegen migration deferred to v0.5.8. Doc 2
  §K.6 had B6 shipping via K; this is now explicit B6 by-hand +
  later K migration.

**Everything else** from §E / §H / §K.6 is unchanged.

### L.7 Confidence statement (fifth pass)

**High confidence:**

- L1 problem description (stale doc comment verified at
  `error.rs:73-79`)
- L2 direction (A4 clearly makes some tests redundant)
- A2/A3 re-verification via `validate_helpers.rs:218-221`
  source-level self-admission — the most definitive confirmation
  yet
- B6 "stored but unused" clarification (verified in
  `validate_helpers.rs:141-165`)
- L.4 strengthened bundling (M1+D5+A4 atomic + §6a four-item)
- L.5 ship-unit sequencing atomicity constraints

**Medium confidence:**

- L3 audit outcome (A vs C vs re-expose) depends on grep findings
- L4 naming (`linux-kde` / `linux-portal` vs alternatives) is a
  taste call
- L.5 Unit 10 (minimum-viable K) timing depends on the 1-week
  estimate from doc 1 §32.3 §2+§14

**Low confidence / explicit unknowns:**

- Whether L2's test simplification produces one follow-up commit
  or needs per-file splitting (depends on how many tests are
  affected — unknown until A4 lands)
- Whether L3's audit surfaces any connector-level consumers
  (grep will answer this in ~1 minute)

### L.8 What this pass did NOT change

Deliberately preserved from §A–§K:

- Every pros/cons entry in doc 2's existing A–K option tables
- Every recommendation not explicitly strengthened or extended
  in §L.2 / §L.4
- §G post-script (v1.0 crate split direction reaffirmed)
- §H cross-document P0 consolidation (supplemented via §L.5,
  not replaced)
- §F open questions list (L3's audit outcome and L4's `linux-*`
  naming add two implicit unknowns; existing entries remain)
- A1 STATUS block (A1 remains fully shipped and out of v0.5.7
  scope)
- §J.2 B4 refinement: `AccessibilityPreferences` on `SystemTheme`,
  not in `ResolutionContext` — §L.5 Unit 8 preserves this
  distinction in the atomic bundle

If section L does not reference a prior recommendation, that
recommendation is unchanged.

### L.9 Endorsement of the doc 2 P0 cohort (fifth-pass confirmation)

All items in §K.9's P0 list personally re-verified this pass
against the current tree. The §L additions (L1–L4 new issues,
A2/A3 re-verification via `validate_helpers.rs:218-221`, atomic
bundling via §L.4, ship-unit sequencing via §L.5) slot as
refinements and follow-ups to the existing cohort without
disturbing its shape.

Doc 2's P0 cohort for v0.5.7 (unchanged from §K.9, with §L.4
bundling constraints applied):

1. **A2** — verified orchestration bug; ships in Unit 2 with A3
2. **A3** — verified dual-category pollution; ships in Unit 2
   with A2 in doc 1 §31.2 F shape
3. **A4** — verified `resolve()` doc-vs-code contradiction; ships
   in Unit 1 atomically with M1 + D5 (per doc 1 §32.4)
4. **B4** — verified accessibility pollution; ships in Unit 8
   atomically with §3 + B5
5. **B6** — verified type-vs-usage mismatch; ships hand-written
   in Unit 9
6. **C1** — verified `Other` has zero emitters; ships in Unit 5
7. **C2** — verified `ColorSchemeChanged` lies on KDE/GNOME;
   ships in Unit 5 with C1
8. **D5 + M1** — verified against four independent sources (doc
   1 §32.2); ships in Unit 1 atomically with A4

**Plus §L refinements and additions:**

9. **L1** — stale `error.rs:73-79` comment; ships in Unit 3 with
   §6a
10. **L2** — test simplification follow-up; ships in Unit 4 after
    Unit 1
11. **L3** — pre-C6 visibility audit; ships before Unit 9's C6
    demotion
12. **L4** — feature aggregator split; ships in Unit 11 with §5 G
    + §31.3

These are the ship-critical items from doc 2 for v0.5.7. Combined
with doc 1's P0 cohort (see doc 1 §32.9), they form a coherent
release ready for execution.

---

## M. Sixth-pass review: verification-only refinements

This section records a sixth ultrathink pass under the explicit
"backward compatibility does not matter; I want the perfect API"
directive. It runs parallel to **doc 1 §33** which hosts the
doc-1-specific sixth-pass findings (F1-F3 polish items, §12/§14
count verification, A2 false-alarm resolution, K feasibility
re-verification).

This pass is **verification-dominated**: every prior doc-2 P0
claim was re-checked against the current tree via parallel
subagents plus direct re-reads, the ~215 literal count in §I5
was nuance-corrected (see §M.3), and three small polish findings
(F1-F3, documented in doc 1 §33) have cross-document implications
minor enough that they do not warrant new doc-2 sections. The
existing doc-2 P0 cohort is re-endorsed without structural
changes.

### M.1 Methodology

Same approach as §J / §K / §L: independently re-read each
existing claim against the current tree, record only refinements
that change recommendations or surface new items, preserve all
earlier content unchanged.

New findings come from four parallel verification subagents (see
doc 1 §33.1 for their specific scopes) plus direct re-reads of
disputed claims. Where a subagent's conclusion conflicted with a
prior doc-2 consensus, the direct re-read settled the question.

### M.2 Verification re-confirmations

Every doc-2 P0/P1 claim was re-verified this pass. Results:

| Claim | File:line | Verdict |
|---|---|---|
| A1 already fixed | `watch/kde.rs:54-68` | ✅ matches commit `f9e5956`; `Option<Instant>` + `is_none_or` pattern present |
| A2 check_ranges on placeholders | `resolve/validate.rs:428-458` | ✅ **re-verified** (see M.4 for the one false-alarm) |
| A3 `missing_fields` dual-category | `error.rs:9-12` + `validate.rs:429-452` + `validate_helpers.rs:217-282` | ✅ all three sites verified |
| A4 purity leak chain | `resolve/mod.rs:20-22`, `inheritance.rs:164-167`, `inheritance.rs:98-109`, `detect.rs:28-30` | ✅ entire chain verified |
| B1 boilerplate span | `validate.rs` (~280 lines) + `widgets/mod.rs:908-1347` (440 lines) | ✅ exact |
| B4 accessibility on ThemeDefaults | `model/defaults.rs:131-140` | ✅ four accessibility fields verified |
| B6 defaults-only at widget level | `model/border.rs:13-35` + `validate_helpers.rs:131-166` | ✅ both structural claim and "stored but unused" L.3 reframing verified |
| C1 dead `Other` variant | `watch/mod.rs:65-70` + workspace grep | ✅ zero production emitters (only the definition itself and the test assertion) |
| C2 `ColorSchemeChanged` platform-inaccurate | `watch/kde.rs:66-69`, `watch/gnome.rs:42-62`, `watch/macos.rs:88-96` | ✅ KDE fires on any kdeglobals change; macOS is the only accurate backend |
| D5 KDE reader hardcode | `kde/mod.rs:52-53` | ✅ exact |
| M1 macOS reader hardcode | `macos.rs:504-505` | ✅ exact; four-source verification (platform-facts + sonoma.toml + sonoma-live.toml + inheritance.rs) unchanged |
| I1 `presets.rs` stale comment | `presets.rs:85-88` | ✅ exact |
| I2 hint references removed method | `error.rs:55-65` | ✅ exact |
| I3 2s `gsettings` timeout | `detect.rs:138-177` | ✅ exact |
| L1 second stale Clone comment | `error.rs:73-79` | ✅ exact |
| L3 visibility audit targets | `kde/mod.rs:341`, `lib.rs:171-172` | ✅ exact |

**No drift beyond pre-existing merge-review corrections.** The
doc-2 claims are reliable at exact line positions.

### M.3 §I5 literal count nuance correction

§I5 claims "~215 string literals total hand-maintained" across
`model/mod.rs:554-720`. The sixth-pass direct count reveals the
number is **correct for the full codegen ROI** but **misleading
if interpreted as literals inside `lint_toml` itself**. Split
counts (personally verified):

**Literals directly inside `lint_toml` (`model/mod.rs:540-745`):**

| Source | Count |
|---|---|
| `TOP_KEYS` (line 554) | 4 |
| `VARIANT_KEYS` (lines 563-593) | 29 |
| `widget_fields` match arms (lines 600-626) | 25 |
| `lint_defaults` nested match (lines 662-666) | 4 |
| `lint_variant` match (lines 691-694) | 2 |
| `lint_variant` nested match (lines 703-708) | 3 |
| `variant_key` iter (line 729) | 2 |
| Layout section (line 736) | 1 |
| Format! strings | ~8-10 |
| **Subtotal in `lint_toml`** | **~78-80** |

**Literals in widget `FIELD_NAMES` arrays (`widgets/mod.rs:164-906`):**

Each of the 25 widgets has a `FIELD_NAMES: &[&str]` constant
generated by `define_widget_pair!` containing the struct's field
name literals. Total: ~135-140 literals across the 25 invocations.

**Combined ROI for codegen**: ~213-220 literals. The §I5 "~215"
figure is a defensible ROI total for Option K (which subsumes
both the `lint_toml` drift hazard and the widget FIELD_NAMES
duplication via direct struct introspection). **The phrasing
"~215 string literals total hand-maintained across
`model/mod.rs:554-720`" is wrong in one specific way**: the
~135-140 widget FIELD_NAMES literals do not live in
`model/mod.rs:554-720` at all — they live inside
`model/widgets/mod.rs` macro invocations. The ROI number is
right; the file:line attribution is wrong.

**Correction**: the 5th-pass framing should read "~78-80 hand-
maintained literals in `lint_toml` (`model/mod.rs:540-745`) plus
~135-140 literals across widget `FIELD_NAMES` arrays
(`widgets/mod.rs:164-906`) ≈ ~215 total drift-risk string
literals eliminable by codegen." Both numbers are useful for
different arguments:

- For "how many literals in `lint_toml` alone are at drift risk"
  → **~78-80** (the §14 argument)
- For "how many literals does the full K codegen pipeline
  eliminate" → **~215** (the §J.4 / §K.2 ROI argument)

**No action required on the doc-2 recommendations.** Both
arguments (§14's "demote the drift hazard" and §K.2's "ship K
for maximum ROI") remain correct. Only the file:line attribution
in §I5's phrasing needs prose tightening in a future doc revision.

**Confidence:** very high. Numbers counted directly by reading
each relevant range.

### M.4 A2 false-alarm resolution (doc-2 mirror of doc 1 §33.5)

One verification subagent in this pass reported A2 as FALSE,
claiming that `validate.rs:428-458` runs the emptiness check
**before** the `check_ranges` calls. Direct re-read contradicts
this. The 24 `check_ranges` calls (lines 429-452) run first, each
passed `&mut missing`; the `if !missing.is_empty()` guard at
line 454 comes after. `require()` substitutes `T::default()`
placeholders earlier in the function and pushes field paths into
`missing`. `check_ranges` then runs on the placeholder-defaulted
`Resolved*Theme` structs and can push **additional spurious
entries** for ranges like `u16::default() == 0 ∉ 100..=900`
(font weight) or `FontSize::Px(0.0).to_px(dpi) ∉ (0.0, ∞)` (font
size).

**A2 is real.** The 5-pass consensus holds. **The doc-2
recommendation (A2 Option B + C: short-circuit + vec split) is
correct.** Doc 1 §33.5 and doc 2 §L.3's `validate_helpers.rs:217-282`
self-admission both independently confirm the bug.

**The false-alarm was a single subagent's misread of the
orchestration order at line 454 vs lines 429-452.** Direct re-read
settled it in seconds; no doc-2 recommendation change.

**Confidence:** very high. Re-verified three times across passes
5 (§L.3), 6 (§33.5), and this direct re-read.

### M.5 F1 / F2 / F3 cross-references to doc 1 §33.6

Doc 1 §33.6 documents three P3 polish findings from the
sixth-pass fresh-eyes audit:

- **F1**: `IconRole::Display` delegates to `Debug::fmt`, producing
  unstable CamelCase output. Recommendation: add `IconRole::name()`
  returning kebab-case, implement `Display` via `name()`. Bundle
  with §18 icon enum stability.
- **F2**: `Rgba::to_f32_tuple` doc comment omits the round-trip
  loss note that `to_f32_array`'s doc explicitly has. Subsumed by
  doc 1 §16's recommendation to delete `to_f32_tuple` entirely.
- **F3**: Mixed `#[must_use]` convention across the crate (bare
  in pipeline.rs, custom-message in model/icons.rs, preachy in
  lib.rs:353). Bundle with §26 as "uniform `#[must_use]`
  convention after §1 renames land."

**Doc-2 relevance:** all three findings are cross-cutting polish
items that affect `native-theme`'s public API surface, not
doc-2-specific A/B/C/D/L categories. They are documented in
doc 1 §33.6 for cohesion with other API-surface polish items.
**No new doc-2 sections required.**

**Priority:** P3 polish. None of the three blocks v0.5.7's
doc-2 P0 cohort.

**Confidence:** high. Each finding was directly verified by
re-reading the cited file:line in doc 1 §33.6.

### M.6 Uncovered-module audit — no new doc-2 findings

The fourth verification subagent audited:

- `pipeline.rs`, `test_util.rs`, `sficons.rs`, `winicons.rs`,
  `freedesktop.rs`, `rasterize.rs`, `spinners.rs`, `bundled.rs`,
  `presets.rs`
- `model/animated.rs`, `model/dialog_order.rs`, `model/icon_sizes.rs`
- `kde/metrics.rs`, `kde/fonts.rs`, `kde/colors.rs`, `gnome/mod.rs`
- All four `watch/` backends

for bugs, runtime panics, public/private leakage, resource leaks,
platform-facts contradictions, error handling gaps, dead code,
and doc/code mismatches.

**Result: zero new issues.** Unwrap/expect calls are properly
gated behind `#[cfg(test)]` or test-module guards. No resource
leaks beyond the already-catalogued `Box::leak` for icon theme
names (doc 1 §13 + I3). No platform-facts contradictions beyond
M1 + the Windows `button_order` tension explicitly scoped out in
§K.5. No dead public code beyond C1. No doc/code mismatches beyond
A4.

**Interpretation:** the modules that doc-2 passes 1-5 did not
cover heavily are genuinely clean. Doc-2's coverage gap is
therefore narrow — the issues in doc-2 cluster in `model/`,
`resolve/`, `error.rs`, `watch/` definitions, and platform reader
contracts, and those areas have been thoroughly audited.

### M.7 K feasibility re-verification impact on doc-2

Doc 1 §33.7 re-verifies the K proc-macro feasibility via an
independent subagent pass. Findings relevant to doc-2:

- **B1**: K unambiguously delivers check_ranges impls from
  `#[theme(range = "...")]` attributes. ~440 lines of hand-written
  boilerplate replaced by attribute metadata + derive-generated
  code. **B1 delivery via K confirmed.**
- **B2**: partial delivery. ~55 of 82 uniform inheritance rules
  expressible via per-field `#[theme(inherit_from = "...")]`
  attributes. ~27 pattern-based rules (border-class, font-class,
  per-platform) need either class-level attributes or hand-
  maintained `inheritance.rs` residue. **B2 partial delivery
  confirmed; the attribute-expressiveness unknown flagged in
  §K.2 remains the main risk.**
- **B6**: K supports `#[theme_layer(border_kind = "full" |
  "partial" | "none")]` at the class level, generating the right
  validation dispatch. **B6 delivery via K confirmed** — though
  §L.5 Unit 9 still ships B6 hand-written for v0.5.7 with
  codegen migration deferred to v0.5.8.
- **B7**: K auto-generates the three validation dispatch paths
  from the same `border_kind` class attribute. **B7 delivery via
  K confirmed** — collapses the three hand-coded
  `require_border` / `border_all_optional` /
  `require_border_partial` helpers into one generated path.

**Border-kind classification re-verified against the tree:**

- **full (13 widgets):** window, button, input, checkbox, tooltip,
  progress_bar, toolbar, list, popover, dialog, combo_box,
  segmented_control, expander — matches `inheritance-rules.toml`
  `widgets_with_border` list exactly
- **partial (2 widgets):** sidebar, status_bar — matches
  `require_border_partial` call sites
- **none (3 widgets):** menu, tab, card — matches
  `border_all_optional` call sites

**Classification is stable and consistent across source,
inheritance rules, and validation code.** K can safely encode
`border_kind` as a class-level attribute.

**Timeline**: Agent 3 estimates 6-7 days for minimum-viable K
(aligning with the 5th-pass 1-week figure).

**Dependencies**: `syn`, `quote`, `proc-macro2`, `inventory` all
already transitive in the workspace. `native-theme-derive` adds
them as direct deps of itself but zero new indirect deps for
`native-theme`.

**Confidence:** high on B1/B6/B7 delivery via K. Medium on B2
delivery (attribute expressiveness unknown; recommended
prototyping on 2-3 widgets before full commitment).

### M.8 Strengthened recommendations (none)

No doc-2 recommendation changed this pass. All strengthenings
from §J.4 / §K.2 / §K.6 / §L.4 / §L.5 stand unchanged:

- M1 + D5 + A4 atomic single-commit (doc 1 §32.4)
- §6a four-item commit (doc 1 §32.3 §6 + L1)
- B1 + B6 + B7 via K codegen (or hand-written B6 with deferred
  K migration per §L.5 Unit 9)
- A3 fold into doc 1 §31.2 F (flat + `kind()` method)
- Arc<str> uniform ownership across `name` / `icon_theme` /
  `family` / `ReaderOutput::name` (§J.2 + §K.3)
- L2 test simplification post-A4 (§L.2)
- L3 visibility audit before C6 demotion (§L.2)
- L4 feature aggregator split (§L.2)
- B5 `ResolutionContext::for_tests()` naming rule; no `Default`
  impl (§J.2)
- B4 `AccessibilityPreferences` on `SystemTheme`, not in
  `ResolutionContext` (§J.2)

### M.9 Priority rebalance (none)

No tier changes. The §L.6 rebalance stands:

- **P0 cohort** (unchanged): A2, A3, A4, B4, B6, C1, C2, D5 + M1
  bundled
- **P1** (unchanged): B1 via K, B2 partial via K, B3, B5, C3, C4,
  C6 (with L3 audit), L2
- **P2** (unchanged): B7 subsumed, C5, D3, D4, L4 bundled with
  §5 G + §31.3
- **P3** (unchanged): D1, D2 subsumed by B6

### M.10 Confidence statement (sixth pass)

**Very high confidence:**

- Every doc-2 P0/P1 file:line claim verified against the current
  tree (§M.2 table)
- A2 bug re-verified after one-agent false-alarm (§M.4 direct
  re-read)
- §I5 literal count nuance correction (§M.3 direct count)
- M1 four-source verification chain unchanged and re-endorsed
- L1/L2/L3/L4 findings all re-verified at their stated file:line

**High confidence:**

- K proc-macro feasibility for B1/B6/B7 delivery (§M.7 Agent 3
  + prior pass consensus)
- F1/F2/F3 doc-1 polish items (cross-referenced from doc 1 §33.6)

**Medium confidence:**

- None new this pass. The B2 attribute-expressiveness unknown
  (§K.2) persists; recommend prototyping before full commitment.

**Deferred / out of scope:**

- §M.3 phrasing correction to §I5 is a prose tweak; not a code
  change.
- Windows `button_order` platform-facts tension (§K.5) remains
  out of scope.

### M.11 What this pass did NOT change

Deliberately preserved from §A-§L:

- Every option table entry in the existing A-L sections
- Every recommendation not explicitly revisited in §M
- §G post-script (v1.0 crate split direction reaffirmed)
- §H cross-document P0 consolidation (re-endorsed)
- §F open questions list (no items removed; §M.5 resolves one
  implicit unknown about cross-doc polish coordination)
- A1 STATUS block (A1 remains fully shipped)
- §J.2 B4 refinement: AccessibilityPreferences on `SystemTheme`,
  not in `ResolutionContext`
- §K.4 M1 bundling with D5 unconditionally
- §L.5 ship-unit sequencing and atomicity constraints

If section M does not reference a prior recommendation, that
recommendation is unchanged.

### M.12 Endorsement of the doc-2 P0 cohort (sixth-pass confirmation)

All items in §K.9's / §L.9's P0 list personally re-verified this
pass against the current tree. The §M additions (literal count
nuance, A2 false-alarm resolution, F1/F2/F3 cross-references,
empty uncovered-module audit result) are refinements at the
documentation-tightening tier and do not disturb the cohort shape.

**Final doc-2 P0 cohort, unchanged from §L.9:**

1. **A2** — verified orchestration bug; ships in Unit 2 with A3
2. **A3** — verified dual-category pollution; ships in Unit 2
   with A2 in doc 1 §31.2 F shape
3. **A4** — verified `resolve()` doc-vs-code contradiction; ships
   in Unit 1 atomically with M1 + D5
4. **B4** — verified accessibility pollution; ships in Unit 8
   atomically with §3 + B5
5. **B6** — verified type-vs-usage mismatch + "stored but unused"
   reframing; ships hand-written in Unit 9
6. **C1** — verified `Other` has zero emitters; ships in Unit 5
7. **C2** — verified `ColorSchemeChanged` lies on KDE/GNOME;
   ships in Unit 5 with C1
8. **D5 + M1** — verified against four independent sources each;
   ships in Unit 1 atomically with A4

**Plus §L refinements and additions (unchanged):**

9. **L1** — stale `error.rs:73-79` comment; ships in Unit 3 with §6a
10. **L2** — test simplification follow-up; ships in Unit 4 after
    Unit 1
11. **L3** — pre-C6 visibility audit; ships before Unit 9's C6
    demotion
12. **L4** — feature aggregator split; ships in Unit 11 with §5 G
    + §31.3

**Sixth-pass closing statement**: six independent review passes
have converged on the same doc-2 P0 shape. Every bug is real,
every cited file:line resolves against the current tree, every
recommendation has been stress-tested against alternatives, and
the cross-document P0 consolidation (doc 1 §32.9 + doc 2 §L.9)
forms a coherent v0.5.7 release.

**The scope is closed. Ship v0.5.7.**

**Confidence:** very high. The convergence of six passes on an
unchanged P0 cohort — with pass 6 contributing only verification
re-confirmations, a literal-count nuance, and three P3 polish
additions (in doc 1 §33) — is strong evidence that the design is
settled. Further passes would at this point be searching for
problems that do not exist.
