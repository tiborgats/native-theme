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

**File:** `native-theme/src/watch/kde.rs:54-56`

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

#### Recommended: **C** (typed `ReaderOutput`)

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

Separately, the iced connector has to leak the font family to obtain
`&'static str` for iced's `Font::Family::Name` (per
`connectors/native-theme-iced/src/lib.rs:40-51`):

```rust
let name: &'static str = Box::leak(
    native_theme_iced::font_family(&resolved).to_string().into_boxed_str()
);
```

Each call leaks another copy. The workaround is documented ("standard
iced pattern") but is inelegant.

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
the iced connector's ad-hoc leak.

**Confidence:** medium-high. `Arc<str>` changes public signatures in
non-trivial ways; connector rewrites are needed. But the connectors
are in this tree, so they can migrate in lockstep.

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
| P2 | A1 `Instant` arithmetic without `checked_sub` (demoted from P0 after verification) | Defensive | Trivial |
| P2 | B7 Unify border-inheritance paths (bundles with B1) | Structural | — |
| P2 | C5 `detect_linux_desktop()` no-arg | Ergonomics | Trivial |
| P2 | D3 `check_ranges` path string allocation | Perf | Low |
| P2 | D4 `Cow<'static, str>` for names | Memory | Low |
| P2 | D5 KDE reader `button_order` hardcode | Cleanup | Trivial |
| P3 | D1 `FontSpec::style` default consistency doc | Polish | Trivial |
| P3 | D2 Padding-derives-from-presence rule (symptom of B6) | — | — |

- **P0 items** should land in v0.5.7. They are verified bugs (A2-A4)
  or small corrections with immediate correctness impact (C1, C2, B4,
  B6). A1 was initially catalogued here but is demoted to P2 after
  verification showed it is a documented may-panic per Rust's std
  contract, not a verified startup crash on the Linux code path this
  file actually compiles on.
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

**Correction note on A1.** This item was initially catalogued as P0
(verified bug). Closer reading of Rust std source showed the `Instant
- Duration::from_secs(10)` expression produces a valid-if-strange
`Instant` with negative internal seconds on the Linux code path this
file compiles on, without panicking. The accurate label is "latent
may-panic per documented std contract" and the item is demoted to
**P2** (defensive coding). The fix is still worth making -- std
explicitly recommends `checked_sub` -- but it does not belong in the
P0 cohort.

This is 13 P0 items totaling **~30-50 diffs of varying size**. Bugs
(A2, A3, A4) are small. Renames (§1, §4, §16, §19, C1, C2) are
mostly mechanical. Structural (§12, §6a, B4, B6) are medium. All
together they constitute a coherent v0.5.7 release.

The P1 items from both documents can be staged across v0.5.7, v0.5.8,
and v0.5.9 depending on capacity. The biggest investments are the
codegen work (doc 1 §2 + §14 + this doc's B1 + B2 + B7, all the same
bet) which is appropriate for a dedicated phase.
