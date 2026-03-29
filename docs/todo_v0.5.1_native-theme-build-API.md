# v0.5.1 API Improvements -- native-theme-build (codegen crate)

Analysis of API critique, verified against actual code (2026-03-29).
Each chapter covers one problem, lists all resolution options with pro/contra,
and proposes the best solution. Pre-1.0 -- backward compatibility is not a
constraint.

**Companion documents**:
- [todo_v0.5.1_native-theme-API.md](todo_v0.5.1_native-theme-API.md)
  covers the native-theme core crate. Cross-references marked **[CORE-N]**.
- [todo_v0.5.1_gpui-connector-API.md](todo_v0.5.1_gpui-connector-API.md)
  covers the native-theme-gpui connector. Cross-references marked **[GPUI-N]**.

---

## Table of Contents

**Design (hard to fix later)**

1. [process::exit(1) instead of Result](#1-processexit1-instead-of-result)
2. [Shadow test API more capable than public API](#2-shadow-test-api-more-capable-than-public-api)
3. [Multi-file builder silently uses only first base directory](#3-multi-file-builder-silently-uses-only-first-base-directory)
4. [No validation of names that become Rust identifiers](#4-no-validation-of-names-that-become-rust-identifiers)
12. [generate() with zero sources panics at index out of bounds](#12-generate-with-zero-sources-panics-at-index-out-of-bounds)
14. [Generated code hardcodes native_theme:: path](#14-generated-code-hardcodes-native_theme-path)

**Correctness risk**

5. [Theme and DE key lists manually synchronized across multiple locations](#5-theme-and-de-key-lists-manually-synchronized-across-multiple-locations)
6. [Generated code re-reads XDG_CURRENT_DESKTOP on every call](#6-generated-code-re-reads-xdg_current_desktop-on-every-call)
7. [Bundled themes with DE-aware mappings only embed default SVG](#7-bundled-themes-with-de-aware-mappings-only-embed-default-svg)
11. [Mapping value strings not escaped in generated code](#11-mapping-value-strings-not-escaped-in-generated-code)
13. [Theme in both bundled-themes and system-themes silently accepted](#13-theme-in-both-bundled-themes-and-system-themes-silently-accepted)
15. [Duplicate roles within a single TOML file not caught](#15-duplicate-roles-within-a-single-toml-file-not-caught)

**Ergonomics**

8. [IconGenerator::add() name collides with std::ops::Add](#8-icongeneratoradd-name-collides-with-stdopsadd)
9. [Size report always emits as cargo::warning with no opt-out](#9-size-report-always-emits-as-cargowarning-with-no-opt-out)
16. [Generated enum derive list is fixed, not configurable](#16-generated-enum-derive-list-is-fixed-not-configurable)

**API hygiene**

10. [BuildError is pub(crate) -- structured error info discarded](#10-builderror-is-pubcrate----structured-error-info-discarded)

---

## 1. process::exit(1) instead of Result

**Verdict: VALID -- high priority**

Both public entry points terminate the process on validation errors instead of
returning structured results:

```rust
pub fn generate_icons(toml_path: impl AsRef<Path>)         // -> ()
pub fn generate(self)                                       // -> ()
```

Inside `emit_result()`, errors are emitted via `cargo::error=` and then:

```rust
std::process::exit(1);
```

This means:
- Callers have zero opportunity to handle errors -- no fallback, no retry, no
  custom reporting
- It is strictly worse than panicking: `catch_unwind` cannot intercept
  `process::exit`
- The return type `()` conceals the fact that the function may never return
- Build system plugins, test harnesses, and IDE integrations cannot wrap these
  functions and react to failures
- The `generate()` method also returns `()` on success -- callers cannot
  inspect what was generated, what file was written, or how many roles were
  processed

The internal `run_pipeline()` function already returns a clean `PipelineResult`
with structured `errors`, `warnings`, `code`, and `rerun_paths`. The
`emit_result()` wrapper destroys all of this by collapsing it into
print-and-die.

### Option A: Return `Result<GenerateOutput, Vec<BuildError>>`

```rust
pub fn generate_icons(toml_path: impl AsRef<Path>) -> Result<GenerateOutput, BuildErrors>

pub struct GenerateOutput {
    pub output_path: PathBuf,
    pub warnings: Vec<String>,
    pub role_count: usize,
    pub bundled_theme_count: usize,
    pub svg_count: usize,
    pub total_svg_bytes: u64,
}
```

**Pro:**
- Callers control error handling -- can log, aggregate, or retry
- Success path exposes useful metadata (output path, counts)
- Testable without `__run_pipeline_on_files` hack
- Idiomatic Rust -- `Result` is the standard error channel
- Warnings are returned rather than forced to cargo output
- Composable with other build steps

**Contra:**
- Breaking change for all callers (must add `?` or `.unwrap()`)
- Callers in build.rs still need to emit `cargo::error=` themselves if they
  want the current behavior
- Requires a public error type (see [section 10](#10-builderror-is-pubcrate----structured-error-info-discarded))

### Option B: Return `Result` but keep `generate_icons()` as a fire-and-forget wrapper

```rust
// Simple API (unchanged behavior, still exits):
pub fn generate_icons(toml_path: impl AsRef<Path>)

// Builder API returns Result:
pub fn generate(self) -> Result<GenerateOutput, BuildErrors>
```

**Pro:**
- Zero breakage for existing `generate_icons()` callers
- Builder API users get full control
- Migration path: start with `generate_icons()`, upgrade to builder when needed

**Contra:**
- Two entry points with fundamentally different error behavior -- confusing
- `generate_icons()` silently exits while `generate()` returns -- inconsistent
- Users of the simple API still can't test or handle errors
- Maintains the process::exit footgun for the most common entry point

### Option C: Add a `try_generate()` variant, keep `generate()` as-is

```rust
pub fn generate(self)                                   // exits on error (current)
pub fn try_generate(self) -> Result<GenerateOutput, BuildErrors>  // returns Result
```

**Pro:**
- Zero breakage
- Power users get `try_generate()`
- Naming convention `try_*` is established in the Rust ecosystem

**Contra:**
- Two methods that do the same thing with different error handling -- bloat
- Users must discover that `try_generate()` exists
- The "default" path (`generate()`) remains the dangerous one
- Cargo-specific concerns (emitting directives, writing to OUT_DIR) are mixed
  into one method but not the other

### Option D: Return `Result` from both, provide `emit_cargo_directives()` helper

```rust
pub fn generate_icons(toml_path: impl AsRef<Path>) -> Result<GenerateOutput, BuildErrors>

impl IconGenerator {
    pub fn generate(self) -> Result<GenerateOutput, BuildErrors>
}

impl GenerateOutput {
    /// Emit cargo::rerun-if-changed, cargo::warning, and write the output file.
    pub fn emit_cargo_directives(&self) { ... }
}

impl BuildErrors {
    /// Emit cargo::error= for each error and return ExitCode(1).
    pub fn emit_cargo_errors(&self) -> std::process::ExitCode { ... }
}
```

Typical build.rs usage:
```rust
fn main() {
    match native_theme_build::generate_icons("icons/icons.toml") {
        Ok(output) => output.emit_cargo_directives(),
        Err(errors) => std::process::exit(errors.emit_cargo_errors().into()),
    }
}
```

Or with the convenience `unwrap_or_exit()`:
```rust
fn main() {
    native_theme_build::generate_icons("icons/icons.toml")
        .unwrap_or_exit()  // emits cargo directives on Ok, cargo errors + exit on Err
        .emit_cargo_directives();
}
```

**Pro:**
- Separates concerns: pipeline logic vs. cargo output vs. process lifecycle
- Callers choose their error handling and output strategy
- `emit_cargo_directives()` / `emit_cargo_errors()` are opt-in, not forced
- `unwrap_or_exit()` gives a one-liner migration path for existing users
- Testable: `generate()` returns data, tests never touch cargo or process
- `GenerateOutput` and `BuildErrors` are inspectable
- Build system plugins can emit directives in their own way

**Contra:**
- More API surface (two helper methods + `unwrap_or_exit()`)
- Existing callers must update (add `?` or `.unwrap_or_exit()`)
- Two-step usage (`generate()` then `emit_cargo_directives()`) is more
  ceremony than the current one-liner

### PROPOSED: Option D -- Result + emit helpers

This is the cleanest separation of concerns. The pipeline returns data.
Cargo directive emission is a separate opt-in step. Process exit is never
forced. The `unwrap_or_exit()` convenience method makes migration trivial:

```rust
// Before:
native_theme_build::generate_icons("icons/icons.toml");

// After (one extra method call):
native_theme_build::generate_icons("icons/icons.toml")
    .unwrap_or_exit()
    .emit_cargo_directives();
```

Option A is close but bundles cargo emission into the pipeline. Option B is
inconsistent. Option C is bloat. Option D gives full control while keeping the
common case simple.

This change is a prerequisite for [section 2](#2-shadow-test-api-more-capable-than-public-api)
(eliminating the shadow test API) and [section 10](#10-builderror-is-pubcrate----structured-error-info-discarded)
(public error types).

---

## 2. Shadow test API more capable than public API

**Verdict: VALID -- high priority (becomes trivial after section 1)**

The crate has two API layers:

| Function | Visibility | Returns | Exits process? |
|---|---|---|---|
| `generate_icons()` | `pub` | `()` | Yes |
| `IconGenerator::generate()` | `pub` | `()` | Yes |
| `__run_pipeline_on_files()` | `pub #[doc(hidden)]` | `PipelineResult` | No |
| `run_pipeline()` | `pub #[doc(hidden)]` | `PipelineResult` | No |

The `#[doc(hidden)]` functions are more capable than the public API. Integration
tests use `__run_pipeline_on_files` exclusively because the real API is
untestable (it calls `process::exit`). The `__` prefix follows Python naming
conventions that do not exist in Rust.

Additionally, `PipelineResult`, `SizeReport`, and `MasterConfig` are all `pub`
but `#[doc(hidden)]` -- forming a shadow API that external users can depend on
but cannot discover through docs.

### Option A: Remove shadow API entirely after section 1 lands

Once `generate()` returns `Result<GenerateOutput, BuildErrors>` (section 1),
the shadow API becomes redundant:

- Integration tests call `generate()` or `IconGenerator::generate()` directly
- `PipelineResult` is replaced by the public `GenerateOutput`
- `__run_pipeline_on_files` is deleted
- `run_pipeline` becomes `pub(crate)` or stays internal
- `MasterConfig` becomes `pub(crate)` (only needed for TOML deserialization)

**Pro:**
- Zero shadow API -- one capability layer
- All tests use the same API as users
- No `#[doc(hidden)]` items in the public surface
- `pub` means documented and stable; `pub(crate)` means internal

**Contra:**
- Integration tests must adapt (use builder API instead of `__run_pipeline_on_files`)
- `run_pipeline` is a useful pure-function core; making it `pub(crate)` means
  external users cannot call it for advanced use cases
- Loses the ability to test the pipeline without `CARGO_MANIFEST_DIR` set

### Option B: Promote shadow API to documented public API

Make `run_pipeline`, `PipelineResult`, etc. fully documented `pub` items:

```rust
/// Run the icon codegen pipeline on pre-parsed configs.
pub fn run_pipeline(...) -> PipelineResult { ... }
```

**Pro:**
- No API removal -- only documentation improvement
- Power users get access to the pure pipeline
- Integration tests already work as-is

**Contra:**
- Large public API surface: `MasterConfig`, `PipelineResult`, `SizeReport`,
  `run_pipeline`, plus all their fields
- `MasterConfig` is a deserialization struct -- exposing it as public API
  locks in the TOML schema as a Rust-level contract
- `run_pipeline` takes `&[(String, MasterConfig)]` -- an awkward signature
  for public API

### Option C: Feature-gate the test API behind `#[cfg(feature = "test-internals")]`

```toml
[features]
test-internals = []
```

**Pro:**
- Integration tests opt in with `native-theme-build = { features = ["test-internals"] }`
- Normal users never see the test API
- No `#[doc(hidden)]` items in default docs

**Contra:**
- Feature-gated public items are still part of the API surface
- Adds feature complexity for a testing concern
- Still two API layers, just behind a gate

### PROPOSED: Option A -- remove shadow API after section 1

Once section 1 lands, the public API returns `Result<GenerateOutput, BuildErrors>`
which is fully testable. Integration tests switch from
`__run_pipeline_on_files(&[path], None)` to:

```rust
let result = IconGenerator::new()
    .add(path)
    .generate();
assert!(result.is_ok());
```

`run_pipeline` becomes `pub(crate)`. `MasterConfig` becomes `pub(crate)`.
`__run_pipeline_on_files` is deleted. `PipelineResult` and `SizeReport` are
replaced by the public `GenerateOutput` and `BuildErrors`.

The one complication is that integration tests currently run without
`CARGO_MANIFEST_DIR` set (they pass absolute paths to `__run_pipeline_on_files`).
The public API relies on `CARGO_MANIFEST_DIR`. Solution: the `generate()` method
should resolve paths relative to `CARGO_MANIFEST_DIR` only when it is set,
falling back to the current working directory for testing contexts. Or: tests
set `CARGO_MANIFEST_DIR` explicitly before calling `generate()`.

---

## 3. Multi-file builder silently uses only first base directory

**Verdict: VALID -- medium priority**

When the builder API merges multiple TOML files, all theme directories are
resolved relative to the **first** file's parent:

```rust
// lib.rs:242 in run_pipeline()
let base_dir = &base_dirs[0];
```

A comment says "For multi-file, all configs sharing a theme must use the same
base_dir" but this constraint is **not validated**. If the user adds TOML files
from different directories:

```rust
IconGenerator::new()
    .add("features/media/icons.toml")       // base: features/media/
    .add("features/navigation/icons.toml")  // base: features/navigation/
    .generate();
```

The second file's themes are resolved from `features/media/`, not
`features/navigation/`. No error, no warning. Theme mapping files for the
second config may silently not be found or -- worse -- may resolve to
unrelated files that happen to have the same name in the first directory.

### Option A: Validate that all base directories are equal

```rust
for base in &base_dirs[1..] {
    if base != &base_dirs[0] {
        errors.push(format!(
            "all TOML files must share a base directory, \
             but {} is in {} while {} is in {}",
            configs[0].0, base_dirs[0].display(),
            configs[i].0, base.display()
        ));
    }
}
```

**Pro:**
- Catches the misconfiguration at build time with a clear error
- Minimal implementation (3-5 lines)
- No behavior change for correct usage
- Documents the constraint in the error message itself

**Contra:**
- Turns a silent bug into a hard error -- may break existing setups that
  accidentally worked (though this is a feature, not a bug)
- Does not add the capability to use different base directories

### Option B: Support per-file base directories

Resolve each theme relative to its own file's parent. Themes shared across
multiple configs are loaded once from whichever file first declares them.

**Pro:**
- Most flexible: TOML files can live anywhere in the project
- No artificial constraint on directory layout
- Matches user expectation: "my TOML file references themes next to it"

**Contra:**
- Complex implementation: must track which base_dir corresponds to which
  theme, handle conflicts when two files declare the same theme from
  different directories
- If two files declare `bundled-themes = ["material"]` but their respective
  `material/` directories contain different SVGs, which one wins? Silent
  conflict
- The mapping TOML files are shared across all roles (merged config), not
  per-file -- this creates ambiguity about which `material/mapping.toml` to
  use

### Option C: Require a single base directory parameter on the builder

```rust
IconGenerator::new()
    .base_dir("icons/")                   // explicit base for all themes
    .add("icons/media.toml")
    .add("icons/navigation.toml")
    .generate();
```

**Pro:**
- Explicit -- no guessing about which directory is used
- Eliminates the ambiguity entirely
- Clear separation: TOML files define roles, base_dir defines where themes live

**Contra:**
- Breaking API change (new required method)
- Redundant when there's only one TOML file (its parent is the obvious base)
- Users must specify a path they could have inferred

### Option D: Use first base directory but warn if others differ

**Pro:**
- No hard errors -- doesn't break existing setups
- Warning signals the issue

**Contra:**
- Warnings are easily ignored
- The behavior is still wrong, just announced

### PROPOSED: Option A + Option C (validate, then allow override)

Default behavior: validate that all TOML files share the same parent directory.
Emit a clear error if they differ. This catches the common misconfiguration.

Additionally, add an optional `.base_dir()` method that overrides the
auto-detected base directory. When set, all themes are resolved from the
explicit base regardless of where the TOML files live. This enables the
legitimate use case of TOML files scattered across the project:

```rust
// All themes live under icons/, but TOMLs are split across feature dirs:
IconGenerator::new()
    .base_dir("icons/")
    .add("features/media/icons.toml")
    .add("features/nav/icons.toml")
    .generate();
```

Option B is rejected because shared theme directories across configs create
unsolvable conflicts. Option D is rejected because warnings don't prevent
incorrect output.

---

## 4. No validation of names that become Rust identifiers

**Verdict: VALID -- medium priority**

Neither role names in the master TOML nor the `enum_name()` override are
validated before being fed into code generation. The `heck` crate converts
them to PascalCase and the result is emitted directly into Rust source:

```rust
let enum_name = config.name.to_upper_camel_case();
writeln!(out, "pub enum {enum_name} {{").unwrap();
```

If the input is invalid, the generated code has syntax errors:

| Input | Generated | Result |
|---|---|---|
| `enum_name("fn")` | `pub enum Fn {}` | Reserved keyword -- compile error |
| `enum_name("")` | `pub enum  {}` | Empty identifier -- compile error |
| `enum_name("123-bad")` | `pub enum 123Bad {}` | Leading digit -- compile error |
| role `"play-pause"` + role `"play_pause"` | `PlayPause` + `PlayPause` | Duplicate variant -- compile error |

These errors surface as rustc failures deep inside `include!`'d generated
code, with no attribution to the TOML source. Users must trace through the
generated file in `OUT_DIR` to understand the problem.

### Option A: Validate all names before codegen, emit clear build errors

Add a validation pass that checks:
1. Role names and enum name are non-empty
2. PascalCase conversion produces a valid Rust identifier (`[A-Z][a-zA-Z0-9]*`)
3. The converted name is not a Rust keyword (strict or reserved)
4. No two roles produce the same PascalCase variant (collision detection)

```rust
fn validate_identifier(name: &str, source: &str) -> Option<BuildError> {
    let pascal = name.to_upper_camel_case();
    if pascal.is_empty() { ... }
    if RUST_KEYWORDS.contains(&pascal.as_str()) { ... }
    if !pascal.starts_with(|c: char| c.is_ascii_uppercase()) { ... }
}
```

**Pro:**
- Errors point to the TOML source, not the generated code
- Catches collisions (`"play-pause"` vs `"play_pause"`) before codegen
- Clear error messages: `role "play_pause" collides with "play-pause"
  (both produce variant PlayPause)`
- Validates early in the pipeline -- no wasted work on SVG checks etc.

**Contra:**
- Must maintain a list of Rust keywords (or use `syn::parse_str` for parsing)
- Adds a validation step that currently isn't needed for valid inputs
- Edge cases: raw identifiers (`r#type`) could be supported but add complexity

### Option B: Use `syn` to parse the generated identifier

```rust
if syn::parse_str::<syn::Ident>(&pascal).is_err() {
    errors.push(BuildError::InvalidIdentifier { ... });
}
```

**Pro:**
- Delegates identifier validation to the Rust parser itself -- always correct
- Handles keywords, leading digits, empty strings, special characters
- Future-proof against new reserved keywords

**Contra:**
- Adds `syn` as a dependency (heavyweight proc-macro parsing crate)
- `syn` is overkill for checking a single identifier
- Increases compile time of the build dependency

### Option C: Validate only collisions, let rustc catch the rest

Only check that no two roles produce the same PascalCase variant. Let rustc
catch keyword conflicts and invalid identifiers naturally.

**Pro:**
- Minimal implementation -- just collision checking
- No keyword list to maintain
- rustc error messages for keywords are already clear
- Catches the subtlest bug (collision) which rustc reports with a confusing
  "duplicate definition" error pointing at generated code

**Contra:**
- Non-collision errors (keywords, empty, digits) still surface as opaque
  rustc failures in `include!`'d code
- User must find the generated file in `OUT_DIR` to debug keyword conflicts
- Partial solution

### Option D: Emit a `compile_error!` comment in generated code with TOML source

When an identifier looks suspicious, emit a `compile_error!` in the generated
code that points back to the TOML source:

```rust
compile_error!("native-theme-build: role \"fn\" in icons.toml produces
               the reserved keyword `Fn` -- rename the role");
```

**Pro:**
- Error appears in the compiler output with context
- No need for a separate validation pass
- The generated file is self-documenting about the problem

**Contra:**
- Still generates code with errors rather than failing at the build-script level
- `cargo::error=` messages are cleaner than `compile_error!` for build scripts
- Detection logic is the same as Option A -- might as well fail early

### PROPOSED: Option A -- validate before codegen

Add a validation pass after TOML loading that checks all names. Use a hardcoded
keyword list (Rust keywords change rarely) rather than pulling in `syn`. Check
for:
1. Non-empty after PascalCase conversion
2. Starts with ASCII uppercase letter
3. Not a Rust keyword (strict keywords + `Self`)
4. No collision between any two roles' PascalCase forms

Collision detection is the highest-value check because it catches the subtlest
bug. The keyword/format checks are cheap additions once the pass exists.

Option B is rejected because `syn` is too heavy for a single-identifier check.
Option C is rejected because it leaves the most common errors (keywords, empty)
to surface as confusing rustc output. Option D is rejected because it delays
failure past the build-script phase.

This is related to **[CORE-7]** (standardize icon functions on `IconSet` enum):
if the core crate's `IconSet::from_name()` gets stricter validation, the build
crate's validation of theme names should align.

---

## 5. Theme and DE key lists manually synchronized across multiple locations

**Verdict: VALID -- medium priority**

The mapping between theme name strings, `IconSet` variants, and `LinuxDesktop`
variants is hardcoded in parallel across three files:

**Theme names (3 locations):**

| Location | Form |
|---|---|
| `schema.rs:KNOWN_THEMES` | `["sf-symbols", "segoe-fluent", ...]` |
| `codegen.rs:theme_name_to_icon_set()` | match arms mapping name -> `IconSet::*` |
| `error.rs:BuildError::UnknownTheme::fmt()` | hardcoded list in error message |

**DE keys (2 locations):**

| Location | Form |
|---|---|
| `validate.rs:KNOWN_DE_KEYS` | `["kde", "gnome", ...]` |
| `codegen.rs:de_key_to_variant()` | match arms mapping key -> `LinuxDesktop::*` |

If `native-theme` adds a new `IconSet` variant (e.g., `Phosphor`) or
`LinuxDesktop` variant (e.g., `Cosmic`), these lists must be updated in
lockstep. There is no compile-time check that they stay in sync.

### Option A: Single const table mapping (name, codegen_path) tuples

```rust
const THEME_TABLE: &[(&str, &str)] = &[
    ("sf-symbols",   "native_theme::IconSet::SfSymbols"),
    ("segoe-fluent", "native_theme::IconSet::SegoeIcons"),
    ("freedesktop",  "native_theme::IconSet::Freedesktop"),
    ("material",     "native_theme::IconSet::Material"),
    ("lucide",       "native_theme::IconSet::Lucide"),
];

const DE_TABLE: &[(&str, &str)] = &[
    ("kde",      "native_theme::LinuxDesktop::Kde"),
    ("gnome",    "native_theme::LinuxDesktop::Gnome"),
    // ...
];
```

All validation and codegen functions derive their data from these tables.

**Pro:**
- Single source of truth -- one place to update when adding a theme or DE
- `theme_name_to_icon_set()` becomes a table lookup, not a match
- `KNOWN_THEMES` is derived: `THEME_TABLE.iter().map(|(name, _)| *name)`
- Error message list is derived from the table
- DE table similarly unifies `KNOWN_DE_KEYS` and `de_key_to_variant()`

**Contra:**
- Codegen paths are strings, not checked at compile time -- a typo in
  `"native_theme::IconSet::SfSymbols"` produces broken generated code
  (but this is already true in the current match arms)
- Lookup by iteration is O(n) instead of match's O(1), but n=5-8 so
  irrelevant

### Option B: Use the core crate's `IconSet::ALL` and `IconSet::to_kebab()` (if added)

If the core crate exposes `IconSet::ALL` and a `to_kebab_case()` method
(**[CORE-7]** proposes `IconSet` serde support), the build crate can iterate
the enum directly.

```rust
for set in native_theme::IconSet::ALL {
    let name = set.to_kebab();
    let codegen_path = format!("native_theme::IconSet::{set:?}");
    // ...
}
```

**Pro:**
- Truly derived from the source of truth (the core crate's enum)
- Adding a variant to `IconSet` automatically makes it available here
- Compile-time guarantee of sync

**Contra:**
- `native-theme-build` is a build dependency -- it cannot depend on
  `native-theme` at build time without a circular dependency (the generated
  code is included by crates that depend on `native-theme`)
- The build crate generates code that references `native_theme::*` by string,
  not by import -- it cannot actually link against the core crate
- This option is architecturally impossible given the current crate structure

### Option C: Shared `native-theme-common` crate with enum definitions

Extract `IconSet` and `LinuxDesktop` enum definitions into a
`native-theme-common` crate. Both `native-theme` and `native-theme-build`
depend on it.

**Pro:**
- True compile-time sync -- both crates use the same enum
- Build crate can iterate `IconSet::ALL` directly

**Contra:**
- Adds a new crate to the workspace
- The build crate generates code as strings -- it still needs name->string
  mappings for `include_bytes!` paths and match arms
- `native-theme-common` would be an extremely thin crate (just two enums)
- Adds dependency for downstream users

### Option D: Add a compile-time test that validates sync with core crate strings

```rust
#[test]
fn theme_table_matches_known_themes() {
    assert_eq!(THEME_TABLE.len(), KNOWN_THEMES.len());
    for (name, _) in THEME_TABLE {
        assert!(KNOWN_THEMES.contains(name));
    }
}

#[test]
fn de_table_matches_known_de_keys() {
    assert_eq!(DE_TABLE.len(), KNOWN_DE_KEYS.len());
    for (key, _) in DE_TABLE {
        assert!(KNOWN_DE_KEYS.contains(key));
    }
}
```

**Pro:**
- Catches internal drift (table vs. array) at test time
- Simple to implement
- No structural changes

**Contra:**
- Doesn't catch drift between this crate and `native-theme` -- they're
  separate crates with no shared source of truth
- Tests catch bugs after the fact, not at compile time

### PROPOSED: Option A + Option D

Consolidate to a single `THEME_TABLE` and `DE_TABLE` as the single source of
truth within the build crate. Derive `KNOWN_THEMES`, `KNOWN_DE_KEYS`,
`theme_name_to_icon_set()`, and `de_key_to_variant()` from these tables.
Add tests that validate internal consistency.

Option B is architecturally impossible (circular dependency). Option C adds
a crate for two tiny enums -- disproportionate. Option A + D eliminates
internal duplication and catches drift within the crate. Cross-crate drift
(core adds a variant, build crate doesn't update) remains a manual concern,
but the build crate's tables are now a single place to update.

For cross-crate sync, add a comment in the core crate's `IconSet` and
`LinuxDesktop` definitions: `// Also update: native-theme-build THEME_TABLE / DE_TABLE`.

---

## 6. Generated code re-reads XDG_CURRENT_DESKTOP on every call

**Verdict: VALID -- low-medium priority**

For DE-aware mappings, the generated `icon_name()` method emits:

```rust
#[cfg(target_os = "linux")]
{
    let de = native_theme::detect_linux_de(
        &std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default()
    );
    match de {
        native_theme::LinuxDesktop::Kde => Some("media-playback-start"),
        _ => Some("play"),
    }
}
```

Every call to `icon_name()` on a DE-aware mapping re-reads the environment
variable and re-parses it. `std::env::var()` acquires a global lock
internally. For a function that could be called in a rendering loop (e.g.,
building an icon toolbar), this is unnecessary repeated work.

The parent crate already uses `OnceLock` for other detection functions
(e.g., `system_is_dark()`).

### Option A: Generate code that caches via `std::sync::OnceLock`

```rust
#[cfg(target_os = "linux")]
{
    static CACHED_DE: std::sync::OnceLock<native_theme::LinuxDesktop> =
        std::sync::OnceLock::new();
    let de = *CACHED_DE.get_or_init(|| {
        native_theme::detect_linux_de(
            &std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default()
        )
    });
    match de { ... }
}
```

**Pro:**
- Environment variable read once, result cached for process lifetime
- Zero overhead after first call
- Consistent with `system_is_dark()` caching in the core crate
- `OnceLock` is thread-safe with no runtime cost after init
- The DE does not change during process lifetime -- caching is correct

**Contra:**
- One `OnceLock` per generated enum that has DE-aware mappings (not per
  match arm -- the DE is the same for all arms)
- If the process sets `XDG_CURRENT_DESKTOP` after the first call, the
  cache is stale (extremely unlikely in practice)

### Option B: Read env var once per `icon_name()` call, but outside the match

Move the `detect_linux_de()` call to before the outer match so it runs at
most once per `icon_name()` call, not once per DE-aware arm:

```rust
fn icon_name(&self, set: native_theme::IconSet) -> Option<&str> {
    #[cfg(target_os = "linux")]
    let de = native_theme::detect_linux_de(
        &std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default()
    );
    match (self, set) { ... }
}
```

**Pro:**
- At most one env var read per `icon_name()` call (currently could be
  multiple if multiple arms are DE-aware, though only one arm matches)
- Simpler generated code than OnceLock
- No caching concerns

**Contra:**
- Still reads the env var on every call -- O(n) calls = O(n) reads
- Only helps if multiple DE-aware arms exist (currently at most one matches)
- Doesn't solve the hot-path performance concern

### Option C: Use `native_theme::cached_linux_de()` if core crate provides it

If the core crate adds a cached version of `detect_linux_de()` (analogous
to `system_is_dark()`), the generated code calls it directly.

**Pro:**
- Caching logic lives in one place (core crate)
- Generated code is simple: `let de = native_theme::cached_linux_de();`
- Core crate controls the caching policy
- Aligns with **[CORE-5]** (system_is_dark caching discussion)

**Contra:**
- Requires a core crate API addition
- If the core crate doesn't add this, the build crate is stuck
- Couples generated code to a new core crate function

### Option D: Keep current behavior, document as "not hot-path safe"

**Pro:**
- Zero implementation effort
- The env var read is cheap (~100ns including lock)
- Icon name lookups are not typically in hot loops
- DE-aware mappings are rare (only freedesktop theme uses them)

**Contra:**
- Surprising performance cliff if someone does call it in a loop
- Inconsistent with the core crate's caching philosophy

### PROPOSED: Option C if core crate adds it, otherwise Option A

The cleanest solution is a `cached_linux_de()` in the core crate (or making
`detect_linux_de()` itself cache the result), then the generated code simply
calls it. This aligns with the **[CORE-5]** discussion about caching
`system_is_dark()`.

If the core crate does not add caching, fall back to Option A: generate a
`OnceLock` in the icon_name method. The `OnceLock` should be shared across
all DE-aware arms (one per generated enum, not per arm).

Option B is insufficient -- it still reads on every call. Option D is
acceptable for now but should be addressed before v1.0.

---

## 7. Bundled themes with DE-aware mappings only embed default SVG

**Verdict: VALID -- low-medium priority**

For a bundled theme with a DE-aware mapping:

```toml
# In a BUNDLED theme's mapping.toml:
bluetooth = { kde = "preferences-system-bluetooth", default = "bluetooth" }
```

The generated `icon_name()` returns `"preferences-system-bluetooth"` on KDE
and `"bluetooth"` elsewhere. But the generated `icon_svg()` only embeds
`bluetooth.svg` (the default):

```rust
// icon_svg: only embeds the default name's SVG
(Self::Bluetooth, IconSet::Material) =>
    Some(include_bytes!("icons/material/bluetooth.svg")),
```

This means on KDE, `icon_name()` returns `"preferences-system-bluetooth"` but
`icon_svg()` returns the bytes for `bluetooth.svg` -- a mismatch. The name
says one thing, the embedded data is another.

In practice this is unlikely to be triggered because:
- DE-aware mappings are designed for freedesktop (a system theme, not bundled)
- Bundled themes (material, lucide) have their own consistent naming
- The schema allows the combination but nobody would use it

However, the crate does not prevent or warn about this combination.

### Option A: Warn at build time if a bundled theme uses DE-aware mappings

```rust
if is_bundled && matches!(value, MappingValue::DeAware(_)) {
    warnings.push(format!(
        "DE-aware mapping for \"{}\" in bundled theme \"{}\" -- \
         only the default SVG will be embedded",
        role, theme_name
    ));
}
```

**Pro:**
- Alerts the user to the implicit behavior
- No behavior change -- just transparency
- Simple implementation (2-3 lines in validation)
- Does not restrict the schema for future use

**Contra:**
- Warning may be noise if the user intentionally uses DE-aware names with
  a bundled fallback SVG (e.g., different icon names per DE but one shared SVG)

### Option B: Error at build time -- reject DE-aware mappings in bundled themes

**Pro:**
- Prevents the name/SVG mismatch entirely
- Forces users to use Simple mappings for bundled themes
- Clear invariant: bundled themes have 1:1 name-to-SVG mapping

**Contra:**
- Overly restrictive -- there may be valid use cases (e.g., a bundled theme
  with DE-specific names but a single shared SVG is acceptable)
- The mismatch between `icon_name()` and `icon_svg()` is a feature: the name
  is for system lookup, the SVG is a bundled fallback
- Breaking change if anyone uses this pattern

### Option C: Embed all DE-specific SVGs for bundled themes

For `{ kde = "preferences-system-bluetooth", default = "bluetooth" }` in a
bundled theme, embed both `preferences-system-bluetooth.svg` and
`bluetooth.svg`. Generate `icon_svg()` with DE-aware dispatch matching
`icon_name()`.

**Pro:**
- `icon_name()` and `icon_svg()` are always consistent
- Full DE-aware support for bundled themes

**Contra:**
- Significant codegen complexity: `icon_svg()` now needs DE-aware dispatch
  with `cfg(linux)` blocks, not just simple match arms
- Binary size increase: N SVGs per DE-aware role instead of 1
- SVG validation must check all DE-specific files, not just the default
- Over-engineered for a combination nobody uses

### Option D: Do nothing -- document the behavior

**Pro:**
- No implementation effort
- The current behavior is internally consistent: `icon_svg()` is documented
  as returning the bundled SVG for the default mapping, `icon_name()` returns
  the DE-specific name for system lookups
- The `IconProvider` trait makes no guarantee that `icon_name()` and
  `icon_svg()` correspond to the same visual

**Contra:**
- Undocumented implicit behavior
- Potential future footgun

### PROPOSED: Option A -- warn at build time

Emit a `cargo::warning` if a bundled theme's mapping contains a DE-aware value.
This transparently communicates the behavior without restricting it. The
warning message should explain: "only the default SVG will be embedded via
icon_svg(); DE-specific names are available via icon_name() for system lookup."

Option B is too restrictive. Option C is over-engineered for a nearly
theoretical scenario. Option D is acceptable but the warning costs nothing
and prevents confusion.

---

## 8. IconGenerator::add() name collides with std::ops::Add

**Verdict: VALID -- low priority**

The builder's `add()` method is annotated to suppress a clippy lint:

```rust
#[allow(clippy::should_implement_trait)]
pub fn add(mut self, path: impl AsRef<Path>) -> Self {
    self.sources.push(path.as_ref().to_path_buf());
    self
}
```

The `should_implement_trait` lint fires because `fn add(self, ...) -> Self`
matches the signature of `std::ops::Add::add()`. The `#[allow]` suppression
works but signals a naming issue -- the method name was chosen despite
conflicting with a standard trait.

### Option A: Rename to `source()`

```rust
IconGenerator::new()
    .source("icons/media.toml")
    .source("icons/navigation.toml")
    .generate();
```

**Pro:**
- Descriptive: the TOML files are sources for code generation
- No clippy suppression needed
- Builder pattern reads naturally: "new generator with these sources"

**Contra:**
- `source` could be confused with "source code" (ambiguous)
- Minor breaking change for existing callers

### Option B: Rename to `file()`

```rust
IconGenerator::new()
    .file("icons/media.toml")
    .file("icons/navigation.toml")
    .generate();
```

**Pro:**
- Short and clear -- you're adding a file to process
- No ambiguity

**Contra:**
- `file` is very generic -- doesn't communicate what kind of file
- Breaking change

### Option C: Rename to `with_toml()`

```rust
IconGenerator::new()
    .with_toml("icons/media.toml")
    .with_toml("icons/navigation.toml")
    .generate();
```

**Pro:**
- `with_` prefix is a common builder convention in Rust
- `toml` communicates the file format expected
- Self-documenting: "generator with this TOML definition"

**Contra:**
- Longer than `add`
- `with_toml` might suggest the method takes TOML content (a string) rather
  than a path

### Option D: Rename to `add_source()`

```rust
IconGenerator::new()
    .add_source("icons/media.toml")
    .add_source("icons/navigation.toml")
    .generate();
```

**Pro:**
- Explicit about what is being added
- `add_source` does not conflict with any std traits
- Reads naturally in builder chain

**Contra:**
- Verbose
- Breaking change

### Option E: Keep `add()` with the suppression

**Pro:**
- Zero breakage
- The suppression is a single attribute
- `add()` is intuitive for appending to a collection

**Contra:**
- Suppressing clippy lints to work around a naming issue is a code smell
- Signals to readers that the API conflicts with a standard trait

### PROPOSED: Option A -- rename to `source()`

`source()` is descriptive ("add a source definition"), concise, and avoids the
clippy lint naturally. Pre-1.0, this rename is trivial. The builder reads
naturally:

```rust
native_theme_build::IconGenerator::new()
    .source("icons/media.toml")
    .source("icons/navigation.toml")
    .enum_name("AppIcon")
    .generate()?;
```

Option C is a close second but risks confusion with TOML content strings.
Option E is acceptable but the rename is cheap and improves clarity.

---

## 9. Size report always emits as cargo::warning with no opt-out

**Verdict: VALID -- low priority**

Every successful build emits a `cargo::warning=` with size statistics:

```rust
// emit_result() in lib.rs:
println!(
    "cargo::warning={} roles x {} bundled themes = {} SVGs, {:.1} KB total",
    report.role_count, report.bundled_theme_count, report.svg_count, kb
);
```

This produces output like:
```
warning: 2 roles x 1 bundled themes = 2 SVGs, 1.2 KB total
```

In CI or large workspaces, these warnings are noise. There is no way to
suppress them. The `cargo::warning=` directive is designed for actionable
information, not informational statistics.

### Option A: Remove the size report warning entirely

**Pro:**
- Clean build output
- Size information is available in `GenerateOutput` (after section 1)
- Build output should only contain actionable warnings

**Contra:**
- Users lose visibility into embedded asset sizes
- No way to notice size regressions without external tooling

### Option B: Gate behind a `verbose` builder option

```rust
IconGenerator::new()
    .source("icons/icons.toml")
    .verbose(true)
    .generate()?;
```

**Pro:**
- Opt-in: only users who want size reports see them
- Default is clean output
- Builder pattern already exists -- one more option is natural

**Contra:**
- More API surface for a minor concern
- `generate_icons()` simple API has no builder to set `verbose` on

### Option C: Include in `GenerateOutput`, let caller decide

After section 1, `GenerateOutput` contains size data. The caller can print it
or not:

```rust
let output = generator.generate()?;
if verbose {
    eprintln!("{} SVGs, {} KB", output.svg_count, output.total_svg_bytes / 1024);
}
output.emit_cargo_directives(); // does NOT print size report
```

**Pro:**
- Caller controls all output
- No API addition beyond what section 1 already provides
- Separation of concerns: pipeline produces data, caller presents it

**Contra:**
- Size report is no longer automatic -- users who want it must print it
  themselves (extra ceremony)

### Option D: Move to `emit_cargo_directives()` behind a flag

```rust
output.emit_cargo_directives_with(EmitOptions { size_report: true });
// or
output.emit_cargo_directives(); // includes size report (current behavior)
output.emit_cargo_directives_quiet(); // no size report
```

**Pro:**
- Backward-compatible default behavior
- Opt-out available

**Contra:**
- API proliferation for a trivial concern
- Options structs for a build helper feel over-engineered

### PROPOSED: Option C -- include in GenerateOutput, no automatic emission

After section 1 lands, `GenerateOutput` contains `svg_count`,
`total_svg_bytes`, `role_count`, and `bundled_theme_count`. The
`emit_cargo_directives()` method emits `rerun-if-changed` and warnings from
validation but not the size report. Users who want size output can print it
themselves. The `unwrap_or_exit()` convenience can optionally include it for
migration smoothness.

Option A discards useful information. Option B adds API for a minor concern.
Option D over-engineers the solution. Option C naturally falls out of
section 1's design.

---

## 10. BuildError is pub(crate) -- structured error info discarded

**Verdict: VALID -- low-medium priority (becomes trivial after section 1)**

`BuildError` has well-structured variants:

```rust
pub(crate) enum BuildError {
    MissingRole { role: String, mapping_file: String },
    MissingSvg { path: String },
    UnknownRole { role: String, mapping_file: String },
    UnknownTheme { theme: String },
    MissingDefault { role: String, mapping_file: String },
    DuplicateRole { role: String, file_a: String, file_b: String },
}
```

But it is `pub(crate)`, so all structured data is flattened to `String` in
`run_pipeline()`:

```rust
errors.push(e.to_string());
```

Downstream users only see opaque error strings. Build system plugins,
IDE integrations, and custom error reporters cannot:
- Filter errors by type (show only missing SVGs, not role mismatches)
- Extract structured fields (which file, which role)
- Programmatically react to specific error categories

### Option A: Make BuildError public and return it from generate()

```rust
#[derive(Debug, Clone)]
pub enum BuildError {
    MissingRole { role: String, mapping_file: String },
    MissingSvg { path: String },
    // ...
}

pub struct BuildErrors {
    pub errors: Vec<BuildError>,
    pub warnings: Vec<BuildWarning>,
}
```

**Pro:**
- Callers can match on error variants and extract fields
- IDE integrations can map errors to file locations
- Enables downstream tooling (e.g., a custom error formatter)
- `BuildError` is already well-structured -- just change visibility
- Implements `Display` for human-readable messages
- Implements `std::error::Error` for standard error handling

**Contra:**
- Public enum variants become API surface -- adding a variant is breaking
  (unless `#[non_exhaustive]`)
- Field names become part of the contract

### Option B: Make BuildError public + non_exhaustive

```rust
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum BuildError {
    #[non_exhaustive]
    MissingRole { role: String, mapping_file: String },
    // ...
}
```

**Pro:**
- Same benefits as Option A
- `#[non_exhaustive]` on the enum allows adding new variants
- `#[non_exhaustive]` on struct variants allows adding new fields
- Future-proof API

**Contra:**
- Callers must have a wildcard arm when matching -- less convenient
- `#[non_exhaustive]` on individual variants prevents destructuring in
  external code, which limits the usefulness of exposing the fields
- Over-cautious: pre-1.0, variants can be added freely anyway

### Option C: Return opaque `BuildErrors` with accessors, not individual variants

```rust
pub struct BuildErrors {
    inner: Vec<BuildErrorInner>,
}

impl BuildErrors {
    pub fn messages(&self) -> Vec<&str> { ... }
    pub fn has_missing_svgs(&self) -> bool { ... }
    pub fn has_role_errors(&self) -> bool { ... }
}
```

**Pro:**
- Internal representation is not public -- full freedom to change
- Accessor methods define a stable query API
- Easier to evolve than a public enum

**Contra:**
- Accessor methods may not anticipate all downstream needs
- Less powerful than full pattern matching
- Must add accessors for every new query callers might want
- More boilerplate than just making the enum public

### Option D: Keep errors as strings, return them in a newtype

```rust
pub struct BuildErrors(pub Vec<String>);
pub struct BuildWarnings(pub Vec<String>);
```

**Pro:**
- Minimal change -- just make the types public
- No enum stability concerns
- Simple API

**Contra:**
- Structured data is still lost
- Callers must parse strings to extract information
- No improvement over the current internal representation

### PROPOSED: Option A -- make BuildError public, no non_exhaustive

Make `BuildError` a public enum. Pre-1.0, there is no backward-compatibility
constraint, so `#[non_exhaustive]` is unnecessary overhead. Add
`impl std::error::Error for BuildError`. Section 1's `BuildErrors` return
type wraps `Vec<BuildError>`.

If the crate approaches 1.0, add `#[non_exhaustive]` to the enum (but not
to individual variants -- field-level non_exhaustive defeats the purpose of
exposing structured data).

Also make `BuildWarning` a structured enum rather than raw strings, for
symmetry:

```rust
pub enum BuildWarning {
    OrphanSvg { file: String, theme: String },
    UnrecognizedDeKey { key: String, role: String, mapping_file: String },
}
```

Option B is premature caution pre-1.0. Option C is over-abstracted.
Option D preserves the current weakness.

---

## 11. Mapping value strings not escaped in generated code

**Verdict: VALID -- medium priority**

In `codegen.rs`, mapping value strings from the TOML are interpolated directly
into Rust string literals without escaping:

```rust
// codegen.rs:128
writeln!(out,
    "            (Self::{variant}, {icon_set}) => Some(\"{name}\"),")
```

The `{name}` comes from user-provided TOML content. If the value contains
characters that are special in Rust string literals (`"`, `\`, newlines), the
generated code is broken or -- in the worst case -- contains injected
expressions.

Example TOML:
```toml
play-pause = "play\"), panic!(\"oops"
```

After TOML escape processing, the deserialized Rust string is
`play"), panic!("oops`. The generated code emits:
```rust
(Self::PlayPause, native_theme::IconSet::Material) => Some("play"), panic!("oops"),
```

The same unescaped interpolation occurs in:
- Simple mapping arms (`codegen.rs:128`)
- DE-aware default arms (`codegen.rs:148`, `codegen.rs:187`)
- DE-specific arms (`codegen.rs:173`)
- `include_bytes!` paths (`codegen.rs:229`) -- where `"` or `/..` in the name
  produces broken paths or directory traversal

In practice, legitimate icon identifiers (SF Symbols, Material names,
freedesktop names) never contain special characters. But the absence of
sanitization means any malformed mapping TOML produces incomprehensible
rustc errors in `include!`'d code rather than a clear build-time diagnostic.

### Option A: Escape strings with Rust string literal escaping before interpolation

```rust
fn escape_rust_str(s: &str) -> String {
    s.replace('\\', "\\\\")
     .replace('"', "\\\"")
     .replace('\n', "\\n")
     .replace('\r', "\\r")
     .replace('\t', "\\t")
}

// Usage in codegen:
let escaped = escape_rust_str(name);
writeln!(out, "            (Self::{variant}, {icon_set}) => Some(\"{escaped}\"),")
```

**Pro:**
- Generated code is always syntactically valid regardless of input
- Transparent: the escaped string produces the correct value at runtime
- Simple implementation (~10 lines)
- No behavior change for normal inputs (no special chars to escape)

**Contra:**
- Silently accepts icon names that are almost certainly wrong (e.g.,
  names containing quotes or newlines)
- The icon name may not match what the system expects (e.g., freedesktop
  lookup with a backslash in the name will fail at runtime)
- Does not catch the actual problem: the mapping value is malformed

### Option B: Validate mapping values contain only safe characters, reject others

```rust
fn validate_icon_name(name: &str, role: &str, file: &str) -> Option<BuildError> {
    if name.is_empty() || name.contains(|c: char| c == '"' || c == '\\' || c.is_control()) {
        Some(BuildError::InvalidIconName { name, role, mapping_file: file })
    } else { None }
}
```

**Pro:**
- Catches the problem at the source: the TOML value is wrong
- Error message points to the exact role and file
- Eliminates the injection vector entirely -- only safe strings reach codegen
- Additionally catches empty strings (which produce `.svg` paths)
- Can be extended with a stricter allowlist (e.g., alphanumeric, `-`, `_`, `.`)

**Contra:**
- Must define what "safe" means -- could be too restrictive
- A strict allowlist might reject valid icon names in some future theme
- Additional validation pass to maintain

### Option C: Escape for safety AND validate for correctness

Apply Option A escaping in codegen as a defense-in-depth measure. Additionally
apply Option B validation in the validation pass. Validation catches the
intentional problems early; escaping prevents accidental breakage if a string
somehow bypasses validation.

**Pro:**
- Belt and suspenders: validation catches known-bad inputs early, escaping
  handles anything that slips through
- Validation errors are clear ("icon name contains invalid character `\"`")
- Generated code is never broken regardless of input
- Defense-in-depth is the standard approach for code generation

**Contra:**
- Two mechanisms for one concern -- slightly more code
- If validation is comprehensive, escaping never triggers (redundant)

### Option D: Use raw string literals in generated code

```rust
writeln!(out, "            (Self::{variant}, {icon_set}) => Some(r#\"{name}\"#),")
```

**Pro:**
- Raw strings don't interpret `\` or `"` as escapes (as long as the
  closing delimiter `"#` doesn't appear in the string)
- Simpler than explicit escaping

**Contra:**
- Breaks if the icon name contains `"#` (unlikely but possible)
- Doesn't solve the `include_bytes!` path injection
- Doesn't catch the real problem (malformed TOML value)
- Unusual codegen pattern that makes generated code harder to read

### PROPOSED: Option C -- validate then escape

Add a validation pass that rejects mapping values containing `"`, `\`,
control characters, or empty strings. Emit `BuildError::InvalidIconName`
with the role and file for attribution. Additionally apply escaping in codegen
as a safety net.

For `include_bytes!` paths, also reject values containing `/`, `..`, or null
bytes, since these would produce path traversal or broken include paths.

Option A alone is insufficient -- it hides the problem. Option B alone
leaves codegen vulnerable if validation is incomplete. Option D is fragile.
Option C combines the strengths of both.

This section overlaps with [section 4](#4-no-validation-of-names-that-become-rust-identifiers)
(identifier validation): both add validation passes. They should be implemented
together in the same wave.

---

## 12. generate() with zero sources panics at index out of bounds

**Verdict: VALID -- low-medium priority**

The builder allows constructing an `IconGenerator` with no source files:

```rust
IconGenerator::new()
    .enum_name("MyIcons")
    // forgot .add()
    .generate();
```

When `generate()` calls `run_pipeline()`, the pipeline accesses `configs[0]`
to determine the output filename:

```rust
// lib.rs:215
let first_name = enum_name_override
    .map(|s| s.to_string())
    .unwrap_or_else(|| configs[0].1.name.clone());
//                      ^^^^^^^^^^ panics: index out of bounds
```

If `enum_name_override` is `Some`, this line is bypassed but the pipeline
still generates an empty enum with no `IconProvider` body -- silently
producing useless code with no warning.

The `generate_icons()` simple API cannot trigger this (it always has one
file), so this is builder-only.

### Option A: Error at generate() time if sources is empty

```rust
pub fn generate(self) -> Result<GenerateOutput, BuildErrors> {
    if self.sources.is_empty() {
        return Err(BuildErrors::from(BuildError::NoSources));
    }
    // ...
}
```

**Pro:**
- Clear error message: "no source TOML files added to IconGenerator"
- Consistent with section 1's `Result` return type
- Catches the mistake at the natural boundary (generate call)
- Simple implementation

**Contra:**
- Requires section 1 to land first (Result return type)
- Does not prevent constructing the empty builder (only fails at generate)

### Option B: Require at least one source in the builder constructor

```rust
pub fn new(first_source: impl AsRef<Path>) -> Self {
    Self {
        sources: vec![first_source.as_ref().to_path_buf()],
        enum_name_override: None,
    }
}
```

**Pro:**
- Impossible to forget: the constructor requires a source
- Compile-time enforcement -- the empty state is unrepresentable
- No runtime check needed

**Contra:**
- Breaking API change (constructor now requires a parameter)
- Loses the "start empty, add incrementally" builder pattern
- Less flexible: can't build the source list in a loop starting from empty
- `new()` with a parameter is unconventional for builders

### Option C: Require at least one source but via a builder entry point

```rust
pub fn from_source(path: impl AsRef<Path>) -> Self { ... }
// No new() -- must start with at least one source

IconGenerator::from_source("icons/media.toml")
    .source("icons/navigation.toml")
    .generate();
```

**Pro:**
- Unrepresentable invalid state: can't have zero sources
- Builder reads naturally: "from this source, add more, generate"
- Removes the empty `new()` constructor

**Contra:**
- Breaking API change
- Can't build source list in a loop (must have first element before loop)
- Unusual pattern for Rust builders

### Option D: Validate at generate() time with a panic and clear message

```rust
pub fn generate(self) {
    assert!(!self.sources.is_empty(),
        "IconGenerator::generate() called with no source files. \
         Call .source() at least once before .generate()");
    // ...
}
```

**Pro:**
- Clear panic message instead of "index out of bounds"
- No API change needed
- Works regardless of section 1's Result return type

**Contra:**
- Still a panic, not an error
- Inconsistent with section 1's proposed Result return type

### PROPOSED: Option A if section 1 lands, otherwise Option D

If section 1 (Result return type) lands, return
`Err(BuildError::NoSources)` -- clean, idiomatic, no panic. If section 1
does not land, use Option D as a stopgap: replace the index-out-of-bounds
panic with a clear assert message.

Option B and C are rejected because they eliminate the zero-sources builder
state at the cost of flexibility. A loop that conditionally adds sources is a
legitimate pattern:

```rust
let mut gen = IconGenerator::new();
for toml in config.icon_sources {
    gen = gen.source(toml);
}
gen.generate()?;
```

This code is valid and should produce a clear error (not a panic) if
`config.icon_sources` happens to be empty.

---

## 13. Theme in both bundled-themes and system-themes silently accepted

**Verdict: VALID -- low-medium priority**

If a TOML file lists the same theme in both `bundled-themes` and
`system-themes`:

```toml
bundled-themes = ["material"]
system-themes = ["material"]
```

The pipeline processes `material` twice:
1. In the bundled loop: loads mapping, validates SVGs, generates
   `icon_name()` arms and `icon_svg()` arms with `include_bytes!`
2. In the system loop: loads mapping again, generates a second set of
   `icon_name()` arms (no `icon_svg()` arms)

The second `all_mappings.insert("material", ...)` overwrites the first with
identical data. The generated `icon_name()` contains **duplicate match arms**
for the same `(Self::Variant, IconSet::Material)` pattern. Rustc may emit
unreachable-pattern warnings.

The overlap is always a user mistake -- a theme is either bundled (SVGs
embedded) or system (looked up at runtime), never both. But the build
script does not detect this.

### Option A: Error if a theme appears in both lists

```rust
fn validate_theme_overlap(config: &MasterConfig) -> Vec<BuildError> {
    let bundled: BTreeSet<&str> = config.bundled_themes.iter().map(|s| s.as_str()).collect();
    config.system_themes.iter()
        .filter(|t| bundled.contains(t.as_str()))
        .map(|t| BuildError::ThemeOverlap { theme: t.clone() })
        .collect()
}
```

**Pro:**
- Catches the mistake immediately with a clear message
- Forces the user to choose bundled or system
- Eliminates duplicate match arms in generated code
- Simple implementation (5-6 lines)

**Contra:**
- Adds a new `BuildError` variant
- A user might intentionally want both behaviors (though it's hard to
  imagine why)

### Option B: Warn but allow the overlap

**Pro:**
- Non-breaking: existing configs that happen to have overlap still work
- Warning signals the issue

**Contra:**
- The generated code still has duplicate match arms
- Warnings are easily ignored
- The behavior is still wrong, just announced

### Option C: Silently deduplicate -- prefer bundled

If a theme is in both lists, treat it as bundled only (since bundled is
a superset of system behavior: it has both `icon_name` and `icon_svg`).

**Pro:**
- No error, no warning -- just correct behavior
- The bundled version is strictly more capable

**Contra:**
- Silent behavior change -- user may not realize the system-theme entry
  is being ignored
- Hides the user's mistake instead of surfacing it

### Option D: Also check for duplicates within each list

A theme listed twice in `bundled-themes` alone is also a mistake:
```toml
bundled-themes = ["material", "material"]
```

**Pro:**
- Complete duplicate detection
- Covers an edge case that Option A misses

**Contra:**
- Rare edge case
- Can be combined with any other option

### PROPOSED: Option A + Option D -- error on all theme duplicates

Error if any theme appears more than once across `bundled-themes` and
`system-themes` combined. This covers both cross-list overlap and
within-list duplicates. Add `BuildError::ThemeOverlap { theme }` and
`BuildError::DuplicateTheme { theme, list }`.

Option B is insufficient -- duplicate match arms in generated code can
cause compiler warnings that the user cannot fix. Option C hides the
problem. Combining A and D gives comprehensive duplicate detection.

---

## 14. Generated code hardcodes native_theme:: path

**Verdict: VALID -- medium priority**

All generated code references the core crate via the fully-qualified path
`native_theme::`:

```rust
impl native_theme::IconProvider for AppIcon {
    fn icon_name(&self, set: native_theme::IconSet) -> Option<&str> {
        // ...
        native_theme::detect_linux_de(
            &std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default()
        )
        // ...
        native_theme::LinuxDesktop::Kde => ...
    }
    fn icon_svg(&self, set: native_theme::IconSet) -> Option<&'static [u8]> { ... }
}
```

If the consumer crate uses a Cargo package alias:

```toml
[dependencies]
nt = { package = "native-theme", version = "0.5" }
```

Cargo exposes the crate as `nt`, not `native_theme`. The generated code
fails to compile because `native_theme::` does not resolve. The user must
add `use nt as native_theme;` as a workaround, which is undocumented and
surprising.

This is a known challenge in the Rust codegen ecosystem. Crates like `prost`,
`tonic`, and `sqlx` solve it with a configurable crate path.

### Option A: Add a `crate_path()` builder method

```rust
IconGenerator::new()
    .source("icons/icons.toml")
    .crate_path("nt")  // generated code uses `nt::IconSet`, `nt::IconProvider`, etc.
    .generate()?;
```

Default: `"native_theme"` (current behavior, no breakage for standard usage).

**Pro:**
- Explicit control over the generated path
- Standard pattern in the Rust ecosystem (prost, tonic, sqlx all do this)
- Default value preserves current behavior
- Simple implementation: replace all `native_theme::` in codegen with the
  configured path
- Also useful when the crate is re-exported through another crate

**Contra:**
- One more builder option to discover and understand
- Not available on the `generate_icons()` simple API (only builder)
- If the user forgets to set it when using an alias, the error is still
  confusing (but this is the current status quo)

### Option B: Auto-detect the crate name from Cargo metadata

At build time, parse `Cargo.toml` to discover what name the consumer uses
for `native-theme`:

```rust
// In build.rs context:
let metadata = cargo_metadata::MetadataCommand::new().exec()?;
let native_theme_name = find_dep_name(&metadata, "native-theme");
```

**Pro:**
- Fully automatic -- no configuration needed
- Works with any alias

**Contra:**
- Requires `cargo_metadata` as a build dependency (heavyweight)
- `cargo_metadata` invokes `cargo metadata` as a subprocess -- slow
- Fragile: parsing dependency graphs is complex (features, optional deps)
- Over-engineered for a rare use case

### Option C: Use `extern crate` in generated code to establish the binding

```rust
// Generated code:
extern crate native_theme;
```

This forces the compiler to resolve `native_theme` even if the dependency
has an alias, because `extern crate` uses the canonical crate name (the
`lib.name` from the dependency's `Cargo.toml`), not the alias.

**Pro:**
- Automatic -- works with any alias
- One line of generated code
- No configuration needed
- No new dependencies

**Contra:**
- `extern crate` is 2015-edition style -- discouraged in 2018+
- May not work correctly in all edition configurations
- `extern crate` with a renamed dependency uses the package name,
  which for `native-theme` would be `native_theme` -- so this
  actually works: `extern crate native_theme;` resolves the package
  regardless of the alias

Wait -- actually, in Rust 2018+, `extern crate` still works and correctly
resolves by the package's crate name, not the alias. Cargo makes the crate
available under its canonical name for `extern crate` statements. The alias
only affects `use` paths. So `extern crate native_theme;` would work even
with `nt = { package = "native-theme" }`.

Let me reconsider this option.

**Pro (revised):**
- Works correctly with Cargo aliases in all editions
- Single generated line: `extern crate native_theme;`
- No configuration needed from the user
- No new dependencies
- The rest of the generated code (`native_theme::IconSet`, etc.) works
  unchanged

**Contra (revised):**
- `extern crate` is considered old-style in 2018+ (lints may warn)
- Adds a top-level item to the generated code that may surprise users
  reading it
- If the consumer does NOT have `native-theme` as a direct dependency
  (e.g., it's only a transitive dependency), `extern crate` fails

### Option D: Generate code using a `use` alias at the top

```rust
// Generated code:
#[allow(unused_imports)]
use native_theme;
```

This doesn't actually help -- `use native_theme;` requires the name to
be in scope, which it isn't with an alias.

**Pro:**
- None meaningful

**Contra:**
- Doesn't solve the problem

### PROPOSED: Option A -- `crate_path()` builder method, with Option C as fallback

Add `crate_path()` to the builder for explicit control. Default value is
`"native_theme"`. For the `generate_icons()` simple API, emit
`extern crate native_theme;` at the top of the generated file to ensure
the path resolves regardless of aliases. Document both approaches.

```rust
// Builder API -- explicit:
IconGenerator::new()
    .source("icons/icons.toml")
    .crate_path("my_nt")
    .generate()?;

// Simple API -- extern crate fallback:
native_theme_build::generate_icons("icons/icons.toml");
// Generated code starts with: extern crate native_theme;
```

Option B is too heavyweight. Option D doesn't work. Option C alone is
sufficient for most cases, but `crate_path()` gives full control for edge
cases (re-exports, facade crates, unusual dependency graphs).

---

## 15. Duplicate roles within a single TOML file not caught

**Verdict: VALID -- low priority**

The cross-file duplicate role check only runs when there are multiple configs:

```rust
// lib.rs:219
if configs.len() > 1 {
    let dup_errors = validate::validate_no_duplicate_roles(configs);
```

But a single TOML file can have duplicate elements in the roles array:

```toml
roles = ["play-pause", "skip-forward", "play-pause"]
```

TOML parses this as a valid 3-element array with a duplicate entry. The
generated enum would have the `PlayPause` variant listed twice:

```rust
pub enum AppIcon {
    PlayPause,
    SkipForward,
    PlayPause,  // duplicate -- rustc error
}
```

Rustc catches this as a "the name `PlayPause` is defined multiple times"
error, but the message points at the generated file in `OUT_DIR`, not at the
TOML source. The user must trace through the generated code to understand
what happened.

This is related to [section 4](#4-no-validation-of-names-that-become-rust-identifiers)
(PascalCase collision detection), which catches the case where two different
role names produce the same variant. This section covers the simpler case of
exact duplicate strings in the roles array.

### Option A: Deduplicate roles in the validation pass, emit warning

```rust
let mut seen = BTreeSet::new();
let mut unique_roles = Vec::new();
for role in &config.roles {
    if !seen.insert(role.as_str()) {
        warnings.push(format!("duplicate role \"{role}\" in {file} -- ignored"));
    } else {
        unique_roles.push(role.clone());
    }
}
```

**Pro:**
- Non-breaking: duplicate is silently removed with a warning
- Doesn't block the build -- the TOML is merely redundant
- Simple implementation

**Contra:**
- Warnings can be missed
- Silently removing duplicates is magic behavior
- A duplicate is almost certainly a copy-paste mistake -- erroring is
  more helpful

### Option B: Error on exact duplicate role strings

```rust
fn validate_no_duplicate_roles_within_file(config: &MasterConfig, file: &str) -> Vec<BuildError> {
    let mut seen = BTreeSet::new();
    config.roles.iter()
        .filter(|r| !seen.insert(r.as_str()))
        .map(|r| BuildError::DuplicateRole { role: r.clone(), file_a: file.into(), file_b: file.into() })
        .collect()
}
```

**Pro:**
- Catches the mistake at the TOML source
- Clear error message pointing to the file
- Reuses the existing `DuplicateRole` error variant
- Consistent with the cross-file duplicate check

**Contra:**
- Breaking if any existing TOML has accidental duplicates (unlikely)
- Reusing `DuplicateRole` with `file_a == file_b` is slightly misleading
  (the variant was designed for cross-file duplicates)

### Option C: Extend the existing duplicate check to always run

Remove the `if configs.len() > 1` guard and run `validate_no_duplicate_roles`
unconditionally. A single config with duplicate roles would be detected as
`file_a == file_b` in the error.

**Pro:**
- Minimal code change -- remove one `if` guard
- Single code path for all duplicate detection

**Contra:**
- The `DuplicateRole` error says "defined in both {file_a} and {file_b}"
  which is confusing when both are the same file
- Doesn't distinguish "same role listed twice" from "different files
  define the same role"

### Option D: Add a dedicated `DuplicateRoleInFile` error variant

```rust
DuplicateRoleInFile { role: String, file: String },
```

With a display message: `role "play-pause" listed more than once in {file}`.

**Pro:**
- Clear, specific error message
- Distinguishes within-file from cross-file duplicates
- Easy to understand

**Contra:**
- One more error variant for a rare edge case
- Must add the validation call in the pipeline

### PROPOSED: Option D -- dedicated error variant

Add `BuildError::DuplicateRoleInFile { role, file }` and validate each
config's roles array for duplicates unconditionally (not just when
`configs.len() > 1`). The error message clearly says "role X listed more
than once in file Y."

Option A is too permissive. Option B reuses an error variant awkwardly.
Option C produces a confusing message. Option D is clean and specific.

This should be implemented alongside [section 4](#4-no-validation-of-names-that-become-rust-identifiers)
since both add validation passes on role names.

---

## 16. Generated enum derive list is fixed, not configurable

**Verdict: VALID -- low priority**

The generated code always emits exactly these derives:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AppIcon { ... }
```

There is no way to add `Ord`, `PartialOrd`, `serde::Serialize`,
`serde::Deserialize`, or custom derives. If the consumer needs to:
- Sort icons: requires `Ord` / `PartialOrd`
- Persist icon choices to settings: requires `Serialize` / `Deserialize`
- Use with a framework that requires specific traits: requires custom derives

They must write manual trait implementations, which is boilerplate the
codegen tool could automate.

The current derive set is the minimum useful set. The built-in `IconRole`
in the core crate also derives only `Debug, Clone, Copy, PartialEq, Eq, Hash`
(no `Ord`, no serde), so the generated code is consistent with the core
crate. But the core crate is constrained by being a library; user-generated
enums should be more flexible.

### Option A: Add a `derives()` builder method

```rust
IconGenerator::new()
    .source("icons/icons.toml")
    .derive("Ord")
    .derive("PartialOrd")
    .derive("serde::Serialize")
    .derive("serde::Deserialize")
    .generate()?;
```

The method accepts any string and emits it as a derive attribute. The base
set (`Debug, Clone, Copy, PartialEq, Eq, Hash`) is always included.

**Pro:**
- Full flexibility: any derive macro can be added
- Standard pattern in codegen crates (prost, sqlx)
- Default behavior is unchanged
- Simple implementation: append to the derive list in codegen

**Contra:**
- No validation that the derive exists or is importable
- The generated file must have the trait in scope -- e.g., `serde::Serialize`
  requires `serde` as a dependency of the consumer crate
- String-based API is not type-safe (typos produce compile errors in
  generated code)
- Not available on the `generate_icons()` simple API

### Option B: Add specific boolean flags for common derives

```rust
IconGenerator::new()
    .source("icons/icons.toml")
    .with_ord(true)
    .with_serde(true)
    .generate()?;
```

**Pro:**
- Type-safe: no string typos
- Discoverable via IDE autocomplete
- The crate can emit the correct `use` imports for known derives

**Contra:**
- Must add a new method for each derive -- doesn't scale
- Can't support custom/third-party derives
- API bloat for a tangential concern

### Option C: Add an `attributes()` method for arbitrary attributes

```rust
IconGenerator::new()
    .source("icons/icons.toml")
    .attribute("#[derive(Ord, PartialOrd)]")
    .attribute("#[serde(rename_all = \"kebab-case\")]")
    .generate()?;
```

**Pro:**
- Maximum flexibility: any attribute, not just derives
- Supports `serde` container attributes, `cfg` attributes, etc.
- Single method handles all attribute needs

**Contra:**
- Raw string attributes are error-prone (must include `#[...]` syntax)
- No validation whatsoever
- Confusing: where do these attributes go? (on the enum? on variants?)
- Over-general for the common case (adding a derive)

### Option D: Support a TOML-level derive list in the master config

```toml
name = "app-icon"
roles = ["play-pause"]
bundled-themes = ["material"]
extra-derives = ["Ord", "PartialOrd", "serde::Serialize"]
```

**Pro:**
- Configuration lives alongside other icon definitions
- No API change to the builder
- Discoverable: users see the option in the TOML schema
- Works with both `generate_icons()` and builder API

**Contra:**
- Mixing Rust-level concerns (derive macros) into a TOML config file
- The TOML file is about icon definitions, not code generation details
- String-based, same validation concerns as Option A
- Changes the TOML schema (needs `MasterConfig` update)

### Option E: Keep current behavior, document manual impls

**Pro:**
- Zero implementation effort
- Users can add trait impls manually after `include!`
- The base derive set covers 95% of use cases
- `Ord` and serde can be impl'd in 5 lines each

**Contra:**
- Manual impls are boilerplate that codegen should eliminate
- If the enum has many variants, manual `Ord` or serde impl is tedious
- Other codegen crates offer this, making this one feel incomplete

### PROPOSED: Option A -- `derive()` builder method

Add a `derive()` method that accepts any string token and appends it to the
generated `#[derive(...)]` attribute. The base set is always included. For
the `generate_icons()` simple API, the base set is sufficient (simple API
users are unlikely to need custom derives; they can switch to the builder).

```rust
native_theme_build::IconGenerator::new()
    .source("icons/icons.toml")
    .derive("Ord")
    .derive("PartialOrd")
    .derive("serde::Serialize")
    .derive("serde::Deserialize")
    .generate()?;
```

Generates:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum AppIcon { ... }
```

Option B doesn't scale. Option C is over-general. Option D mixes concerns.
Option E is acceptable but leaves ergonomic value on the table. Option A is
the standard approach in Rust codegen crates.

---

## Execution order

The following order minimizes rework by resolving dependencies first:

### Wave 1: Core API shape (must land first)

1. **Section 1**: Return `Result` from `generate_icons()` and `generate()`.
   Introduce `GenerateOutput`, `BuildErrors`, `emit_cargo_directives()`,
   `unwrap_or_exit()`.
2. **Section 10**: Make `BuildError` public. Introduce `BuildWarning`.
   Subsection of section 1 -- they share the `BuildErrors` type.
3. **Section 2**: Remove shadow API (`__run_pipeline_on_files`, `run_pipeline`
   becomes `pub(crate)`, `PipelineResult` removed). Rewrite integration tests
   to use the public API.
4. **Section 12**: Error on zero sources (uses section 1's Result type). Can
   land as part of section 1 -- add `BuildError::NoSources` to the new
   public error enum.

Section 1 is a prerequisite for sections 2, 9, 10, and 12. Section 10
should land alongside section 1 since they define the error types together.

### Wave 2: Validation and correctness

5. **Section 4**: Add identifier validation pass (keywords, collisions, format).
6. **Section 11**: Validate mapping value strings, add escaping in codegen.
   Implement alongside section 4 since both add validation passes.
7. **Section 15**: Detect duplicate roles within a single file. Add
   `BuildError::DuplicateRoleInFile`. Extends section 4's validation scope.
8. **Section 3**: Add `base_dir()` to builder, validate base directories match.
9. **Section 13**: Error on theme overlap between bundled and system lists.
   Simple validation addition.
10. **Section 5**: Consolidate theme/DE tables to single source of truth.
11. **Section 7**: Warn on DE-aware mappings in bundled themes.

Sections 4, 11, and 15 form a natural cluster (validation passes on TOML
content). Section 3 and section 13 are independent of each other. Section 5
is a refactoring that benefits from the validation infrastructure added in
section 4.

### Wave 3: Generated code quality

12. **Section 6**: Cache `XDG_CURRENT_DESKTOP` in generated code. Coordinate
    with **[CORE-5]** if core crate adds `cached_linux_de()`.
13. **Section 14**: Add `crate_path()` builder method and `extern crate`
    fallback. Touches codegen string generation.

Sections 6 and 14 both modify codegen output and can be done together.

### Wave 4: Polish

14. **Section 8**: Rename `add()` to `source()`.
15. **Section 9**: Remove automatic size report from cargo output (subsumed
    by section 1's `GenerateOutput`).
16. **Section 16**: Add `derive()` builder method for custom derives.

Section 8 is a trivial rename. Section 9 is automatically resolved by
section 1's design (size data in `GenerateOutput`, not forced to cargo
output). Section 16 is additive and independent.

Wave 1 should land in the same release as **[CORE-7]** (icon type
standardization) since both affect how icon sets are referenced. Section 6
should coordinate with **[CORE-5]** (caching policy for detection functions).
Section 14 should coordinate with any core crate re-export changes.
