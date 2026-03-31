# v0.5.2 API Review -- native-theme-build (codegen crate)

Verified against source code on 2026-03-31. Each chapter covers one
API problem, lists all resolution options with full pro/contra, and
recommends the best solution. Pre-1.0 -- backward compatibility is
not a constraint.

**Companion documents:**
- [todo_v0.5.2_native-theme_api.md](todo_v0.5.2_native-theme_api.md)
- [todo_v0.5.2_native-theme-iced_api.md](todo_v0.5.2_native-theme-iced_api.md)
- [todo_v0.5.2_native-theme-gpui_api.md](todo_v0.5.2_native-theme-gpui_api.md)

---

## Summary

| # | Issue | Severity | Fix complexity |
|---|-------|----------|----------------|
| 1 | `IconGenerator` missing `Debug` derive | Low | Trivial |
| 2 | `BuildErrors` exposes inner `Vec` as public tuple field | Medium | Low |
| 3 | `GenerateOutput` missing `Clone` derive | Low | Trivial |
| 4 | `GenerateOutput.rerun_paths` inconsistent visibility | Low | Trivial |
| 5 | `IconGenerator` and `GenerateOutput` structs missing `#[must_use]` | Medium | Trivial |
| 6 | `BuildError` enum missing `#[non_exhaustive]` | Medium | Trivial |

---

## 1. IconGenerator missing Debug derive

**File:** `native-theme-build/src/lib.rs:269`

**What:** The `IconGenerator` builder struct has no `Debug` impl:

```rust
pub struct IconGenerator {
    sources: Vec<PathBuf>,
    enum_name_override: Option<String>,
    base_dir: Option<PathBuf>,
    crate_path: Option<String>,
    extra_derives: Vec<String>,
    output_dir: Option<PathBuf>,
}
```

Only `Default` is implemented (manually, at line 278). Compare with
`GenerateOutput` which derives `Debug`, and `BuildError` / `BuildErrors`
which both derive `Debug, Clone`.

Without `Debug`, users cannot inspect the builder state via
`dbg!(generator)` or `println!("{generator:?}")` when diagnosing
build script problems -- a common debugging need since build scripts
run in a constrained environment.

### Options

**A. Add `#[derive(Debug)]` (recommended)**

- Pro: All fields are standard types (`Vec<PathBuf>`, `Option<String>`,
  etc.) that implement `Debug`. The derive just works.
- Pro: Consistent with every other public type in the crate.
- Pro: Essential for build script debugging where `dbg!()` is the
  primary tool.
- Pro: Zero runtime cost.
- Con: None.

**B. Implement `Debug` manually with redacted output**

- Pro: Could hide sensitive paths or large vectors.
- Con: Build scripts don't handle sensitive data -- there's nothing
  to redact.
- Con: More code to maintain than a derive.

**C. Keep as-is**

- Pro: No change.
- Con: Users can't debug builder state. The workaround is to print
  each field individually before calling `generate()`.

### Recommendation

**Option A.** The derive is trivial, the need is real (build script
debugging), and there's no reason to omit it.

---

## 2. BuildErrors exposes inner Vec as public tuple field

**File:** `native-theme-build/src/error.rs:185`

**What:** `BuildErrors` is a tuple struct with a public inner field:

```rust
#[derive(Debug, Clone)]
pub struct BuildErrors(pub Vec<BuildError>);
```

Direct field access via `.0` allows users to:
- Mutate the error list (`errors.0.push(...)`, `errors.0.clear()`)
- Construct `BuildErrors` directly without going through the library's
  error-collecting pipeline
- Depend on the `Vec` representation (prevents changing to a different
  collection type later)

This is unusual for an error type. Standard practice is either a named
struct with a private field and accessor methods, or a newtype with
controlled access.

### Options

**A. Make the field private, add accessor methods (recommended)**

```rust
pub struct BuildErrors(Vec<BuildError>);

impl BuildErrors {
    pub fn errors(&self) -> &[BuildError] { &self.0 }
    pub fn into_errors(self) -> Vec<BuildError> { self.0 }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
    pub fn len(&self) -> usize { self.0.len() }
}

impl IntoIterator for BuildErrors { ... }
impl<'a> IntoIterator for &'a BuildErrors { ... }
```

- Pro: Encapsulates the representation -- free to change to `BTreeSet`
  or `SmallVec` later without breaking the public API.
- Pro: Prevents external mutation of the error list.
- Pro: `&[BuildError]` is the idiomatic Rust return type for a
  borrowed slice.
- Pro: `into_errors()` gives ownership when needed (e.g., filtering).
- Pro: `IntoIterator` enables `for e in &errors { ... }` and
  `for e in errors { ... }` -- currently impossible since the inner
  `Vec` is not iterable through the newtype.
- Con: Breaking change for code that accesses `.0` directly. Fixable
  with a mechanical search-and-replace. Pre-1.0 so acceptable.

**B. Keep public field, add named-struct wrapper**

```rust
pub struct BuildErrors {
    pub errors: Vec<BuildError>,
}
```

- Pro: Named field is more readable than `.0`.
- Pro: Field remains publicly accessible for flexibility.
- Con: Still exposes the `Vec` representation.
- Con: Still allows external mutation.

**C. Keep as-is**

- Pro: No change. Direct `.0` access is maximally flexible.
- Con: Unusual API surface for an error type.
- Con: Prevents future representation changes.

### Recommendation

**Option A.** Error types should be opaque about their internals.
The accessor methods cover all legitimate use cases (iteration,
inspection, ownership transfer) while preserving encapsulation.

---

## 3. GenerateOutput missing Clone derive

**File:** `native-theme-build/src/lib.rs:144`

**What:** `GenerateOutput` derives only `Debug`:

```rust
#[derive(Debug)]
pub struct GenerateOutput {
    pub output_path: PathBuf,
    pub warnings: Vec<String>,
    pub role_count: usize,
    pub bundled_theme_count: usize,
    pub svg_count: usize,
    pub total_svg_bytes: u64,
    rerun_paths: Vec<PathBuf>,
    pub code: String,
}
```

All fields are `Clone`-able types. Without `Clone`, users who need
the output in multiple places (e.g., logging metadata AND emitting
directives) must destructure and reconstruct, or use references
throughout.

### Options

**A. Add `#[derive(Clone)]` (recommended)**

- Pro: All fields implement `Clone`. The derive just works.
- Pro: Enables natural ownership patterns: clone for logging, consume
  original for emission.
- Pro: Consistent with `BuildError` and `BuildErrors` which both
  derive `Clone`.
- Con: Cloning `code: String` copies the entire generated source.
  This could be large (thousands of lines). However, Clone is opt-in
  -- users pay only when they call `.clone()`.

**B. Add `Clone` and also store `code` as `Arc<str>` for cheap clones**

- Pro: Makes cloning cheap regardless of code size.
- Con: Adds `Arc` dependency and complexity for an optimization that
  build scripts don't need (they run once per build).
- Con: `Arc<str>` changes the public field type -- bigger breaking
  change.

**C. Keep as-is**

- Pro: No change. Build scripts typically use the output exactly once.
- Con: The missing `Clone` is inconsistent with peer types.
- Con: Users who need the metadata (role_count, svg_count) AND want
  to emit directives must read the metadata before consuming the
  struct.

### Recommendation

**Option A.** The `Clone` derive is simple and the cost is opt-in.
Build scripts run once per build, so the cost of cloning a `String`
is negligible. Consistency with peer types matters more than
micro-optimization in this context.

---

## 4. GenerateOutput.rerun_paths inconsistent visibility

**File:** `native-theme-build/src/lib.rs:159`

**What:** All metadata fields in `GenerateOutput` are `pub` except
`rerun_paths`:

```rust
pub struct GenerateOutput {
    pub output_path: PathBuf,       // public
    pub warnings: Vec<String>,      // public
    pub role_count: usize,          // public
    pub bundled_theme_count: usize, // public
    pub svg_count: usize,           // public
    pub total_svg_bytes: u64,       // public
    rerun_paths: Vec<PathBuf>,      // PRIVATE
    pub code: String,               // public
}
```

Users who want custom build-script behavior (e.g., emitting additional
cargo directives, or writing the file to a non-default location) cannot
access the list of watched paths. They must call
`emit_cargo_directives()` which writes the file AND emits directives
as a single indivisible operation.

### Options

**A. Add a `rerun_paths()` getter method (recommended)**

```rust
impl GenerateOutput {
    pub fn rerun_paths(&self) -> &[PathBuf] { &self.rerun_paths }
}
```

- Pro: Exposes the data without making the field itself public.
- Pro: Returns `&[PathBuf]` (borrowed slice) -- idiomatic, no
  allocation.
- Pro: Users can emit `cargo::rerun-if-changed` selectively or add
  their own paths.
- Pro: Does not break existing code (additive change).
- Con: Minimal -- one more method to maintain.

**B. Make the field `pub`**

- Pro: Consistent visibility with all other fields.
- Pro: Simpler -- no getter method needed.
- Con: Allows external mutation (`output.rerun_paths.push(...)`).
- Con: Cannot add validation or transformation later without breaking.

**C. Keep as-is**

- Pro: No change. The `emit_cargo_directives()` method handles the
  standard case.
- Con: Users with non-standard build scripts are blocked.
- Con: The inconsistent visibility is confusing -- it suggests the
  field is internal when the data is clearly useful externally.

### Recommendation

**Option A.** A getter method is the Rust convention for exposing data
from structs with mixed visibility. It preserves encapsulation while
giving users the access they need. The field was likely made private
to prevent mutation, which the getter respects by returning `&[PathBuf]`.

---

## 5. IconGenerator and GenerateOutput structs missing #[must_use]

**File:** `native-theme-build/src/lib.rs`

**What:** No public function, method, or struct in the crate has
`#[must_use]`.

`Result` has a built-in `#[must_use]` in std, so `generate_icons();`
and `output.emit_cargo_directives();` already produce compiler
warnings about unused `Result` values. The built-in message is generic
("this `Result` may be an `Err` variant, which should be handled") and
adding a custom `#[must_use]` would only improve the diagnostic text.

The truly uncaught gaps are the **struct types** -- neither
`IconGenerator` nor `GenerateOutput` has `#[must_use]`:

| Item | Returns / Is | Uncaught pattern |
|------|-------------|------------------|
| `IconGenerator` struct | builder type | Configured builder silently dropped without `.generate()` |
| `GenerateOutput` struct | pipeline output | Output obtained but `.emit_cargo_directives()` never called |
| `generate_icons()` | `Result<_, _>` | Baseline warning from `Result` -- only custom message missing |
| `emit_cargo_directives()` | `io::Result<()>` | Same -- baseline warning exists |

The builder methods take `self` by value, so the borrow checker
catches some mistakes (using the builder after it was moved). But it
does NOT catch this pattern:

```rust
IconGenerator::new()
    .source("icons.toml")
    .enum_name("MyIcon"); // configured builder silently dropped
// no generate() call -- no warning
```

And `GenerateOutput` can be silently dropped after `?` unwrapping:

```rust
let output = generate_icons("icons.toml")?;
// output is never used -- emit_cargo_directives() never called
// no warning without #[must_use] on GenerateOutput
```

### Options

**A. Add `#[must_use]` to both structs and custom messages on functions
(recommended)**

```rust
#[must_use = "a configured builder does nothing until .generate() is called"]
pub struct IconGenerator { ... }

#[must_use = "call .emit_cargo_directives() to write the file and emit cargo directives"]
pub struct GenerateOutput { ... }
```

Plus custom `#[must_use]` on `generate_icons()` and
`emit_cargo_directives()` for improved diagnostic messages (the
baseline `Result` warning is generic).

- Pro: `IconGenerator` catches dropped builder chains.
- Pro: `GenerateOutput` catches the "obtained output but forgot to
  emit" pattern.
- Pro: Custom messages on functions replace the generic "unused
  `Result`" text with actionable guidance.
- Pro: Zero runtime cost.
- Con: `#[must_use]` on `IconGenerator` also warns for `let _ =
  IconGenerator::new()` -- harmless but noisy. Acceptable since this
  pattern is uncommon in build scripts.

**B. Add `#[must_use]` only to the two struct types**

- Pro: Catches the two truly uncaught patterns.
- Pro: No redundant warnings on `Result`-returning functions.
- Con: Misses the opportunity for better diagnostic messages on
  `generate_icons()` and `emit_cargo_directives()`.

**C. Add `#[must_use]` only to `Result`-returning functions**

- Pro: Improves diagnostic messages.
- Con: Misses the builder chain and GenerateOutput drop cases -- the
  only patterns where `#[must_use]` provides genuinely new coverage.

**D. Keep as-is**

- Pro: No change.
- Con: Configured builders and pipeline outputs silently dropped.

### Recommendation

**Option A.** The two struct annotations provide genuinely new
coverage (uncaught by `Result`'s built-in warning). The function
annotations add custom messages that improve diagnostics. Together
they close all gaps:

1. `IconGenerator` struct -- catches builder drops
2. `GenerateOutput` struct -- catches missing `emit_cargo_directives()`
3. `generate_icons()` -- custom message replaces generic `Result` text
4. `emit_cargo_directives()` -- same

---

## 6. BuildError enum missing #[non_exhaustive]

**File:** `native-theme-build/src/error.rs:10`

**What:** The `BuildError` enum has no `#[non_exhaustive]` attribute:

```rust
#[derive(Debug, Clone)]
pub enum BuildError {
    MissingRole { role: String, mapping_file: String },
    MissingSvg { path: String },
    UnknownRole { role: String, mapping_file: String },
    UnknownTheme { theme: String, config_file: String },
    MissingDefault { role: String, theme: String, config_file: String },
    DuplicateRole { role: String, config_file: String },
    Io { message: String },
    InvalidIdentifier { identifier: String, reason: String },
    IdentifierCollision { identifier: String, normalized: String },
    ThemeOverlap { theme: String, file_a: String, file_b: String },
    DuplicateRoleInFile { role: String, config_file: String },
    DuplicateTheme { theme: String, config_file: String },
    InvalidIconName { name: String, reason: String },
}
```

The core crate's `Error` enum at `error.rs:32` correctly uses
`#[non_exhaustive]`. Without it on `BuildError`, adding new error
variants in future versions is a semver-breaking change -- any
downstream `match` on `BuildError` without a wildcard arm would fail
to compile.

The `BuildErrors` wrapper struct does not need `#[non_exhaustive]`
(it's a concrete collection type, not a sum type).

### Options

**A. Add `#[non_exhaustive]` (recommended)**

```rust
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum BuildError {
    // ...
}
```

- Pro: Allows adding new error variants in future minor versions
  without breaking downstream code.
- Pro: Consistent with the core crate's `Error` enum.
- Pro: Forces downstream match statements to include a `_` arm,
  which is good practice for error handling anyway.
- Pro: Zero runtime cost.
- Con: Breaking change -- existing exhaustive matches must add a
  wildcard arm. Pre-1.0 so acceptable.
- Con: Build scripts rarely match on specific `BuildError` variants
  (most use `.unwrap_or_exit()` or `emit_cargo_errors()`), so the
  practical breakage is minimal.

**B. Keep as-is, add `#[non_exhaustive]` at 1.0**

- Pro: No breaking change now.
- Pro: Pre-1.0 is supposed to allow breaking changes anyway; new
  variants can be added freely.
- Con: The 1.0 release itself becomes a larger breaking change.
- Con: Inconsistent with the core crate's convention.
- Con: If third-party code starts depending on exhaustive matching
  before 1.0, the later addition of `#[non_exhaustive]` forces them
  to change.

**C. Keep as-is permanently**

- Pro: No change.
- Con: Every new error variant is a breaking change post-1.0.
- Con: Inconsistent with the rest of the project.

### Recommendation

**Option A.** The attribute is trivial to add and the breakage is
minimal (build scripts almost never match individual error variants).
Adding it now establishes the convention before external code depends
on exhaustive matching. This is consistent with the core crate's
approach.
