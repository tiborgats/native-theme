# v0.5.4 -- native-theme-build: Deep Critical Analysis

Thorough review of the `native-theme-build` crate covering correctness,
safety, code quality, API design, and test coverage.

Files reviewed:
- `Cargo.toml`
- `src/lib.rs` (2160 lines), `src/codegen.rs` (~1000 lines), `src/error.rs` (258 lines), `src/schema.rs` (344 lines), `src/validate.rs` (1020 lines)
- `tests/integration.rs` (502 lines)
- `tests/fixtures/material/mapping.toml`, `tests/fixtures/sample-icons.toml`, `tests/fixtures/sf-symbols/mapping.toml`
- `docs/platform-facts.md` (1475 lines, cross-referenced for icon/theme accuracy)

---

## 1. `process::exit(1)` in Production Code (2 sites)

Two call sites invoke `std::process::exit(1)`, which terminates the
process immediately, skipping destructors, Drop implementations, and
any cleanup logic.

- **`lib.rs:189`** -- `emit_cargo_directives()` calls `process::exit(1)`
  when `std::fs::write()` fails.
- **`lib.rs:226`** -- `UnwrapOrExit::unwrap_or_exit()` calls
  `process::exit(1)` on `Err`.

The `UnwrapOrExit` call is somewhat defensible (it exists as an explicit
opt-in migration helper). But `emit_cargo_directives()` is a method on
`GenerateOutput` -- users who have an `Ok` result may not expect that
calling a method on it will kill the process. There is no
Result-returning alternative.

### Solutions

#### A. Make `emit_cargo_directives()` return `Result<(), std::io::Error>` (recommended)

| Pros | Cons |
|------|------|
| Standard Rust error handling | Breaking API change (return type changes from `()`) |
| Callers decide how to handle write failure | Callers must add `?` or handle the error |
| No destructor-skipping termination | |
| Composable with other error handling | |

#### B. Add `try_emit_cargo_directives() -> Result<(), std::io::Error>` alongside

| Pros | Cons |
|------|------|
| Backward-compatible | API surface duplication |
| Gradual migration path | Two methods doing the same thing |
| Users can choose behavior | Old method still has the `process::exit` |

#### C. Keep `process::exit(1)` (status quo)

| Pros | Cons |
|------|------|
| No code change | Skips destructors on write failure |
| Simple for build.rs scripts that don't need cleanup | Violates crate-level no-exit rule |
| Matches `UnwrapOrExit` pattern | Users can't intercept the failure |

**Best solution: A.** `emit_cargo_directives()` is the only non-`UnwrapOrExit`
site that exits the process. Since `UnwrapOrExit` is an explicit opt-in
with "Exit" in the name, users understand what they are getting. But
`emit_cargo_directives` looks like a normal method -- the hidden exit
violates least-surprise. Return `Result` and let callers use
`UnwrapOrExit` or `?` as they prefer. Schedule for the next semver break
(the trait's exit is documented and intentional, so it can remain).

---

## 2. `assert!` / `assert_eq!` Panics in Production Code (3 sites)

Three locations use panic-based assertions outside `#[cfg(test)]`:

- **`lib.rs:345-348`** -- `crate_path()` builder method:
  `assert!(!path.is_empty() && !path.contains(' '), ...)`.
- **`lib.rs:367-369`** -- `derive()` builder method:
  `assert!(!name.is_empty() && !name.contains(char::is_whitespace), ...)`.
- **`lib.rs:569`** -- `run_pipeline()` internal:
  `assert_eq!(configs.len(), base_dirs.len())`.

A user typo like `.crate_path("")` crashes the build script with a
raw panic backtrace instead of a structured cargo error. The internal
`assert_eq!` guards a caller invariant that should always hold, but
produces an unhelpful message if triggered by a bug.

### Solutions

#### A. Defer builder validation to `generate()` (recommended)

Store raw input in the builder fields. Validate in `generate()` and
return errors via the normal `BuildErrors` collection. Convert the
internal `assert_eq!` to `debug_assert_eq!`.

| Pros | Cons |
|------|------|
| All errors flow through `BuildErrors` | Errors surface at generate-time, not builder-time |
| No panics in any non-test code path | Slightly more validation code in `generate()` |
| Consistent error experience with all other validations | |
| Builder methods remain infallible (ergonomic for fluent chaining) | |

#### B. Return `Result` from builder methods

Change `crate_path()` and `derive()` to return `Result<Self, BuildErrors>`.

| Pros | Cons |
|------|------|
| Immediate validation at call site | Breaks fluent `.source().crate_path().derive().generate()` |
| Clear error on bad input | Every builder call needs `?` or `.unwrap()` |
| Rare pattern in Rust builder APIs | |

#### C. Keep panics but replace with `BuildErrors`-based panic messages

Replace `assert!` with a custom macro that formats the message as a
cargo error before panicking.

| Pros | Cons |
|------|------|
| Better DX than raw panic | Still panics, still skips destructors |
| No API change | Violates no-panic rule |
| At least the user sees a cargo-formatted error | |

**Best solution: A.** Defer to `generate()`. Builder methods become
infallible, matching the standard Rust builder convention. The internal
`assert_eq!` becomes `debug_assert_eq!` (fires only in debug builds,
documents the invariant).

---

## 3. `escape_rust_str()` Does Not Escape Null Bytes

**`codegen.rs:11-24`** -- `escape_rust_str()` handles `\`, `"`, `\n`,
`\r`, `\t` but passes all other characters through unchanged. A null
byte (`\0`) in an icon name (after bypassing validation -- see issue 4)
would produce `"icon\0name"` in generated code. Rust string literals
cannot contain raw null bytes; this would be a compile error in the
generated code.

`validate_mapping_values()` at `validate.rs:357` does reject control
characters (which includes `\0`), so this is currently defended by
validation. But `escape_rust_str` is a standalone function that
should be correct in isolation.

### Solutions

#### A. Add `\0` to the escape table (recommended)

Add `'\0' => out.push_str("\\0")` to the match in `escape_rust_str`.

| Pros | Cons |
|------|------|
| Defense in depth -- escape function is correct standalone | One more match arm |
| Prevents breakage if validation is bypassed or relaxed | Null bytes already blocked by validation |
| Cheap | |

#### B. Escape all control characters generically

Replace the match with a range check: any `c.is_control()` gets
`\u{XXXX}` escaping.

| Pros | Cons |
|------|------|
| Catches every control character | More complex code |
| Future-proof | Performance cost for uncommon case |
| Redundant with validation | |

#### C. Rely on validation (status quo)

| Pros | Cons |
|------|------|
| No code change | `escape_rust_str` is silently incorrect for inputs it claims to handle |
| Validation catches it today | Two modules must agree on what's safe |

**Best solution: A.** One line, defense in depth. Add `'\0'` escaping
and optionally a test.

---

## 4. Path Traversal via Icon Names in `include_bytes!`

**`validate.rs:345-368`** -- `validate_mapping_values()` rejects
empty strings and control characters, but does NOT reject `/`, `\`,
or `..`. An icon name like `"../../etc/passwd"` passes validation
and is interpolated into the generated `include_bytes!` path at
`codegen.rs:316`:

```
include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/{base_dir}/{theme_name}/../../etc/passwd.svg"))
```

This allows embedding arbitrary filesystem files into the compiled
binary. While the `.svg` suffix limits practical damage, a file at
`/etc/secret.svg` or a carefully crafted path could still leak data.

Note: `escape_rust_str` at `codegen.rs:313` escapes the icon name for
the string literal, but it is also used in the `include_bytes!` path
on line 316 where the escaped value becomes part of a filesystem path.
The escaping prevents breaking the Rust string but does NOT prevent
path traversal.

### Solutions

#### A. Reject `/`, `\`, and `..` in icon name validation (recommended)

Extend `validate_mapping_values()` to reject names containing path
separator characters or parent-directory sequences.

| Pros | Cons |
|------|------|
| Prevents path traversal in generated code | Slightly more restrictive |
| Catches the problem at validation time | Very unlikely to affect real users |
| Simple character check | |
| Combines naturally with existing validation | |

#### B. Canonicalize the resolved path and verify it stays within theme dir

| Pros | Cons |
|------|------|
| Most robust -- handles symlinks, multi-level traversal | Requires the file to exist at validation time |
| OS-aware normalization | System themes have no SVGs to resolve |
| Complex implementation | |

#### C. Only reject in bundled themes (system themes have no file access)

| Pros | Cons |
|------|------|
| Smaller scope | `icon_name()` values are still used at runtime |
| Less restrictive for system theme names | Inconsistent validation |

**Best solution: A.** Character-level rejection is simple, catches all
practical attack vectors, and applies uniformly. Icon names have no
legitimate reason to contain path separators.

---

## 5. `crate_path` Interpolated Without Syntax Validation -- Code Injection

**`lib.rs:344-349`** -- `crate_path()` only checks for empty strings
and spaces. The value is interpolated directly into generated code at
multiple locations:

- `codegen.rs:86`: `if crate_path == "native_theme"` -- safe, just comparison.
- `codegen.rs:118`: `impl {crate_path}::IconProvider for {enum_name}`.
- `codegen.rs:170`: `fn icon_name(&self, set: {crate_path}::IconSet)`.
- `codegen.rs:181-185`: `OnceLock<{crate_path}::LinuxDesktop>`, `{crate_path}::detect_linux_de(...)`.
- `codegen.rs:297`: `fn icon_svg(&self, set: {crate_path}::IconSet)`.

A malicious `crate_path` like `"native_theme{}\nstruct Exploit;\nimpl Foo"` (no
spaces, so it passes the assert) produces syntactically valid but
semantically wrong generated code. In a build script context the
attacker controls their own `build.rs`, so this is low-risk in
practice, but it violates defense-in-depth.

### Solutions

#### A. Validate `crate_path` as a Rust path (identifiers separated by `::`) (recommended)

Check that the path matches `^[a-zA-Z_][a-zA-Z0-9_]*(::[a-zA-Z_][a-zA-Z0-9_]*)*$`.

| Pros | Cons |
|------|------|
| Prevents code injection | Slightly more validation |
| Allows valid paths like `my_crate::native_theme` | May reject Unicode identifiers (rare in crate names) |
| Simple regex-free check | |

#### B. Use a typed enum for known crate paths

| Pros | Cons |
|------|------|
| Type-safe | Must maintain the enum for every possible re-export |
| Zero injection risk | Too restrictive |

#### C. Keep current check (spaces-only) + document the risk

| Pros | Cons |
|------|------|
| No code change | `crate_path` is injectable |
| Build.rs authors control their own input | Violates defense-in-depth |

**Best solution: A.** Validate as a Rust path. Simple loop: split on
`::`, verify each segment matches `[a-zA-Z_][a-zA-Z0-9_]*`. Defer
the validation to `generate()` per issue 2.

---

## 6. `extra_derives` Interpolated Without Syntax Validation -- Code Injection

**`lib.rs:366-371`** -- `derive()` checks for empty/whitespace only.
The value is interpolated directly into `#[derive({derives})]` at
`codegen.rs:96`. A value like `"Ord)]\nstruct Exploit;\n#[derive(Debug"`
contains no whitespace and passes the assert, but produces:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord)]
struct Exploit;
#[derive(Debug)]
```

Same risk profile as issue 5 (low in practice, but defense-in-depth
violation).

### Solutions

#### A. Validate as Rust path segments (recommended)

Same pattern as issue 5: split on `::`, verify each segment is
`[a-zA-Z_][a-zA-Z0-9_]*`. This allows `serde::Serialize`, `Ord`, etc.

| Pros | Cons |
|------|------|
| Prevents code injection | May reject generic derives like `MyDerive<T>` |
| Simple validation | Generics in derives are extremely rare |
| Consistent with crate_path validation | |

#### B. Use allowlist of common derives

| Pros | Cons |
|------|------|
| Guaranteed safe | Too restrictive for custom derives |
| Must maintain the list | |

#### C. Keep current check + document

| Pros | Cons |
|------|------|
| No change | Injectable |

**Best solution: A.** Same Rust-path validation as issue 5. Can share
the validation function.

---

## 7. `icon_svg()` / `icon_name()` Semantic Mismatch for DE-Aware Bundled Mappings

**`codegen.rs:308-316`** -- `generate_icon_svg()` uses
`mv.default_name()` to generate `include_bytes!` paths. For a
DE-aware bundled mapping like
`{ kde = "media-playback-start", default = "play_pause" }`,
only `play_pause.svg` is embedded.

But `generate_icon_name()` at `codegen.rs:207-274` returns
`"media-playback-start"` when running on KDE. The SVG data from
`icon_svg()` does not correspond to the icon name from `icon_name()`.

A warning is already emitted at `lib.rs:673-681` (inline comment says
"Issue 7"), but the mismatch is fundamental: bundled themes embed one
SVG per role, while DE-aware mappings declare multiple icon names per
role.

### Solutions

#### A. Make bundled DE-aware mappings a build error (recommended)

Change the warning at `lib.rs:673-681` to push `BuildError` instead
of a warning string. DE-aware mappings are inherently a system-theme
concern (the OS resolves different icon names at runtime).

| Pros | Cons |
|------|------|
| Eliminates the mismatch entirely | Users must use system themes for DE-aware icons |
| Clear constraint documented by the error message | Reduces flexibility |
| DE-aware icons are inherently a runtime/system-theme concept | |
| No more silent semantic mismatch | |

#### B. Embed all DE-specific SVGs for bundled themes

Generate multiple `include_bytes!` per role, selecting at runtime.

| Pros | Cons |
|------|------|
| `icon_svg()` and `icon_name()` always match | Larger binary (N SVGs per role) |
| Correct semantics | Significantly more complex codegen |
| | DE-specific SVGs must exist (new validation burden) |

#### C. Keep warning, return `None` from `icon_svg()` for DE-overridden names

When a DE override is active, `icon_name()` returns the DE-specific
name but `icon_svg()` returns `None` (forcing system lookup).

| Pros | Cons |
|------|------|
| No binary bloat | Complex codegen (cfg-gated `icon_svg` arms) |
| Falls back gracefully | Partial bundling is confusing |

**Best solution: A.** Bundled themes are for embedding SVGs. DE-aware
mappings exist for system themes that resolve names at runtime. These
are fundamentally different use cases. Enforcing this at build time
is the cleanest solution.

---

## 8. Cross-File Theme Overlap Not Validated After Merge

**`lib.rs:597-599`** -- `validate_theme_overlap()` runs per config
file before merging. But after `merge_configs()` at `lib.rs:612`,
the merged config is not re-checked for overlap.

If file A declares `bundled-themes = ["material"]` and file B declares
`system-themes = ["material"]`, the merged config has "material" in
both lists with no error. The pipeline proceeds and generates code with
`material` treated as both bundled and system.

### Solutions

#### A. Re-run overlap validation on the merged config (recommended)

Add `validate_theme_overlap(&merged)` after `merge_configs()`.

| Pros | Cons |
|------|------|
| Catches cross-file overlap | One extra loop after merge |
| Clear error message | |
| Consistent with per-file validation | |
| Simple -- two lines of code | |

#### B. Validate during merge (fail fast)

Make `merge_configs()` return `Result`, check as themes are added.

| Pros | Cons |
|------|------|
| Fails at point of conflict | Mixes merge logic with validation |
| Earlier error | `merge_configs` currently returns no errors |
| | More complex function signature |

**Best solution: A.** Add one line after the existing merge call. The
rest of the validation already runs on the merged config.

---

## 9. `pipeline_result_to_output()` Has Hidden Side Effects

**`lib.rs:877-888`** -- On the error path,
`pipeline_result_to_output()` calls
`println!("cargo::rerun-if-changed=...")` for every tracked path
before returning `Err(BuildErrors)`. This is hidden I/O in a function
whose name suggests pure data conversion.

This means:
- The function cannot be tested without capturing stdout.
- Callers cannot control when or whether rerun directives are emitted.
- The function does different I/O depending on whether the result is
  Ok or Err (on Ok, no I/O is done -- the caller does it later in
  `emit_cargo_directives()`).

### Solutions

#### A. Include rerun paths in `BuildErrors` and move printing to the caller (recommended)

Add a `rerun_paths: Vec<PathBuf>` field to `BuildErrors` (or a wrapper).
The caller (`generate_icons()` / `IconGenerator::generate()`) or
`UnwrapOrExit` can emit them.

| Pros | Cons |
|------|------|
| No hidden side effects | Changes `BuildErrors` struct (but it's `#[non_exhaustive]`) |
| Callers control when directives are emitted | Slightly more code in callers |
| Function becomes pure -- testable without stdout | |

#### B. Move the println into `emit_cargo_errors()`

When `BuildErrors` is emitted, also print rerun-if-changed. Store
the paths in `BuildErrors`.

| Pros | Cons |
|------|------|
| Collocates related output | `BuildErrors` must carry extra data |
| Single call site | |

#### C. Keep side effect, rename to `finalize_pipeline_result()`

| Pros | Cons |
|------|------|
| Name matches behavior | Still can't suppress output |
| No structural change | |

**Best solution: A.** Move rerun-if-changed data into the error type
and let callers decide when to print. The `#[non_exhaustive]` attribute
on `BuildErrors` means adding fields is not a semver break.

---

## 10. Double Slash in `include_bytes!` Path When TOML at Manifest Root

**`lib.rs:767-775`** -- When the master TOML is placed directly in
`CARGO_MANIFEST_DIR` (not in a subdirectory), `strip_prefix(mdir)`
produces an empty string for `base_dir_str`. The generated path at
`codegen.rs:316` becomes:

```
include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "//material/play.svg"))
```

The double leading slash `//` is technically valid on Unix (POSIX
collapses it) but is malformed on Windows and produces confusing paths
in error messages.

### Solutions

#### A. Skip the leading `/` when `base_dir_str` is empty (recommended)

In `codegen.rs:316`, use `"/{base_dir}"` only when `base_dir` is
non-empty, otherwise use `""`.

| Pros | Cons |
|------|------|
| Clean generated paths | Minor conditional in codegen |
| Correct on all platforms | |
| Better error messages | |

#### B. Normalize in `lib.rs` by trimming trailing `/` from `base_dir_str`

| Pros | Cons |
|------|------|
| Fix at the source | Still generates `"/material/play.svg"` with leading slash |
| Single place to fix | Unclear whether the slash is wanted or not |

#### C. Keep double-slash (status quo)

| Pros | Cons |
|------|------|
| Works on Unix today | Malformed paths on Windows |
| No code change | Confusing generated code |

**Best solution: A.** The codegen line should be:

```rust
let sep = if base_dir.is_empty() { "" } else { "/" };
writeln!(out, "... concat!(env!(\"CARGO_MANIFEST_DIR\"), \"{sep}{base_dir}/{theme_name}/...\")");
```

---

## 11. Orphan SVG Detection Ignores DE-Specific Icon Names

**`validate.rs:126-129`** -- `check_orphan_svgs()` builds its
`referenced` set from `v.default_name()` only. For a DE-aware mapping
like `{ kde = "media-playback-start", default = "play" }`, only
`"play"` is in the referenced set.

If a developer intentionally places `media-playback-start.svg` in the
theme directory (for preview/reference), it is falsely reported as an
orphan.

### Solutions

#### A. Include all icon names (Simple + all DeAware values) in the referenced set (recommended)

Change the `referenced` collection to iterate over all values in
DeAware maps, not just the default.

| Pros | Cons |
|------|------|
| No false positive orphan warnings | Slightly larger set |
| Simple change to the iterator | |
| Correct semantics -- all referenced names are "used" | |

#### B. Only suppress warnings for known DE names

Check whether the orphan SVG stem matches any DE-specific name in the
same role.

| Pros | Cons |
|------|------|
| More targeted | More complex logic |
| Still warns about truly orphaned files | |

**Best solution: A.** The referenced set should include every name
that appears in any mapping value.

---

## 12. SVG Path/Rerun Tracking Ignores DE-Specific Names

**`lib.rs:820-826`** -- `check_orphan_svgs_and_collect_paths()` uses
`value.default_name()` for both `rerun_paths` and `svg_paths`.
DE-specific SVGs (e.g., `media-playback-start.svg`) placed in a
bundled theme directory are not tracked for cargo rebuild and not
counted in the size report.

Same root cause as issue 11.

### Solutions

#### A. Track all icon names for rerun/size (recommended)

Iterate all values in MappingValue, not just default.

| Pros | Cons |
|------|------|
| All relevant SVGs tracked for rebuild | More paths in rerun list |
| Accurate size reporting | |
| Consistent with orphan detection fix | |

**Best solution: A.** Fix alongside issue 11 -- same iterator change.

---

## 13. `unwrap_or("unknown")` Hides Non-UTF-8 Filenames

**`validate.rs:149-153`** -- In `check_orphan_svgs()`:

```rust
let file_name = path
    .file_name()
    .and_then(|n| n.to_str())
    .unwrap_or("unknown");
```

On Linux, filenames can contain arbitrary bytes. A non-UTF-8 filename
causes `to_str()` to return `None`, and the warning says
"unknown is not referenced" -- impossible to identify which file.

### Solutions

#### A. Use `to_string_lossy()` (recommended)

```rust
let file_name = path
    .file_name()
    .map(|n| n.to_string_lossy())
    .unwrap_or_else(|| std::borrow::Cow::Borrowed("unknown"));
```

| Pros | Cons |
|------|------|
| Non-UTF-8 filenames shown with replacement character | Lossy representation |
| Consistent with the rest of the crate | |
| `to_string_lossy` is already used in multiple places | |

#### B. Use the `stem` variable already checked above

The `stem` variable is already confirmed as UTF-8 (the `if let` guard
checks `to_str()`), so use `format!("{stem}.svg")` in the warning.

| Pros | Cons |
|------|------|
| No `unwrap_or` needed | Slightly couples warning text to filtering logic |
| Same data, different path | |

**Best solution: A.** Consistent with the crate's existing pattern.

---

## 14. `check_orphan_svgs()` Silently Flattens `read_dir` Errors

**`validate.rs:143`** -- `for entry in entries.flatten()` silently
discards individual directory entry errors. On Linux, permission-denied
or broken-symlink entries are silently skipped. The orphan detection
may report incorrect results without any indication of why.

### Solutions

#### A. Emit a warning per failed directory entry (recommended)

```rust
for entry_result in entries {
    let entry = match entry_result {
        Ok(e) => e,
        Err(e) => {
            warnings.push(format!("cannot read entry in {}: {e}", theme_dir.display()));
            continue;
        }
    };
    // ... existing logic
}
```

| Pros | Cons |
|------|------|
| User informed of filesystem issues | More verbose output on failure |
| Debugging is possible | |

#### B. Keep `.flatten()` (status quo)

| Pros | Cons |
|------|------|
| Clean code | Silently drops errors |
| | Impossible to debug orphan false positives |

**Best solution: A.** Orphan detection is advisory, but silently
swallowing filesystem errors makes debugging harder than necessary.

---

## 15. `#[cfg(test)]` Import Misplaced at Module Scope

**`validate.rs:1`** -- `BTreeMap` is imported with `#[cfg(test)]` at
module scope:

```rust
#[cfg(test)]
use std::collections::BTreeMap;
```

But `BTreeMap` is only used inside `#[cfg(test)] mod tests`. The
`#[cfg(test)]` attribute on line 1 applies only to the `use` statement
on line 2, not to any production code. While functionally harmless, it
looks like a mistake (why is a production import cfg-gated?) and could
confuse maintainers.

### Solutions

#### A. Move the import inside the test module (recommended)

| Pros | Cons |
|------|------|
| Clear intent | Minor diff |
| No confusion about scope | |

**Best solution: A.** Trivial cleanup.

---

## 16. `let _ = writeln!(...)` Intent Unclear

**`codegen.rs`** uses `let _ = writeln!(out, ...)` approximately 50
times throughout `generate_code()`, `generate_icon_name()`, and
`generate_icon_svg()`.

`String`'s `fmt::Write` implementation is infallible (it can always
allocate), so the `Result` is always `Ok`. The `let _ =` discard is
correct but the intent is not obvious. A reader may wonder: "Why might
this fail? Are we silently dropping errors?"

### Solutions

#### A. Add an explanatory comment at the top of `generate_code()` (recommended)

```rust
// Note: writeln! to a String is infallible. `let _ = ...` suppresses
// the #[must_use] warning on the Ok(()) result.
```

| Pros | Cons |
|------|------|
| Intent clear to future maintainers | One comment |
| No code change | |

#### B. Create a `write_line!` helper macro that calls `.unwrap()`

| Pros | Cons |
|------|------|
| No `let _ =` anywhere | Introduces a panic (violates no-panic rule) |
| Cleaner syntax | |

#### C. Use `.ok();` instead of `let _ =`

| Pros | Cons |
|------|------|
| Slightly more explicit | Same ambiguity |

**Best solution: A.** A single comment at the function entry.

---

## 17. Incomplete `RUST_KEYWORDS` List

**`validate.rs:12-17`** -- The list has 38 entries but is missing
reserved-for-future keywords: `abstract`, `become`, `box`, `do`,
`final`, `macro`, `override`, `priv`, `try`, `typeof`, `unsized`,
`virtual`, `yield`.

**Practical impact is zero**: `to_upper_camel_case()` always produces
PascalCase, and no Rust keyword starts with an uppercase letter except
`Self` (already listed). A role named `"try"` becomes `"Try"`, which
is a valid identifier. A role named `"self"` becomes `"Self"`, which
IS caught.

### Solutions

#### A. Add a comment explaining why reserved words are excluded (recommended)

```rust
/// Note: `to_upper_camel_case()` always produces PascalCase.
/// Only `Self` is practically reachable from this list because it is
/// the only keyword that starts with an uppercase letter. Reserved-
/// for-future keywords (abstract, try, etc.) are omitted because they
/// produce valid PascalCase identifiers (Abstract, Try).
```

| Pros | Cons |
|------|------|
| Documents the reasoning | Technically incomplete per spec |
| No misleading additions | |

#### B. Add all reserved keywords anyway

| Pros | Cons |
|------|------|
| Complete per Rust spec | All additions are unreachable via PascalCase |
| Future-proof if Rust adds an uppercase keyword | Misleadingly suggests they matter |

**Best solution: A.** Document why they are omitted. Adding them is
harmless but misleading.

---

## 18. Repeated `to_upper_camel_case()` Calls

`codegen.rs:99`, `codegen.rs:110`, `codegen.rs:210`, `codegen.rs:312`
all call `role.to_upper_camel_case()` on the same role string. For an
enum with N roles and M themes, each role is converted at least
2 + M times.

Performance impact is negligible (build-time code, roles are short
strings). The concern is code clarity: the repeated conversion
obscures that the variant name is a function of the role name.

### Solutions

#### A. Pre-compute a `Vec<(role, variant)>` and pass it through (recommended)

Build `let role_variants: Vec<(&str, String)>` once in
`generate_code()` and pass it to the sub-functions.

| Pros | Cons |
|------|------|
| Each role converted exactly once | One extra Vec allocation |
| Clearer intent -- "variant" is a named concept | Slightly more function parameters |
| Eliminates scattered `to_upper_camel_case` calls | |

#### B. Use a `HashMap<&str, String>` cache

| Pros | Cons |
|------|------|
| Lazy computation | Overkill for linear iteration |
| | HashMap overhead for small N |

**Best solution: A.** Pre-compute the variants once. Pass as
`&[(&str, String)]`.

---

## 19. Silent Name Normalization

**`validate.rs:222-246`** -- The identifier validation accepts names
with spaces, underscores, or mixed casing (e.g., `"app icon"`,
`"APP_ICON"`) and silently normalizes them to PascalCase via
`to_upper_camel_case()`. Users may not realize their name was
transformed.

This also applies to `enum_name()` at `lib.rs:319` -- a value like
`"my icon"` becomes `"MyIcon"` with no feedback.

### Solutions

#### A. Emit a warning when the input differs from its PascalCase output (recommended)

```rust
if name != pascal {
    warnings.push(format!("name \"{name}\" will be used as \"{pascal}\""));
}
```

| Pros | Cons |
|------|------|
| User is informed of the normalization | More warning output |
| Does not reject valid names | Warns on common input like kebab-case |
| Catches unexpected transformations | |

#### B. Reject names with characters other than `[a-z0-9-]` (kebab-case only)

| Pros | Cons |
|------|------|
| No ambiguity | Too strict -- underscores and CamelCase are common |
| | Would reject `enum_name("MyIcon")` |

#### C. Only warn when the input contains spaces

| Pros | Cons |
|------|------|
| Narrower scope | Misses underscore/mixed-case surprises |

**Best solution: A.** Warn on any input-to-output difference, but
exclude the common case of kebab-case (which is the documented format).
Warn when the input contains spaces, uppercase letters, or underscores
that are not kebab-case.

---

## 20. Missing Theme Directory Check

When a theme directory (e.g., `material/`) does not exist, the error
comes from `mapping.toml` read failing at `lib.rs:650-705`. The error
message is:

```
failed to read /path/to/material/mapping.toml: No such file or directory
```

This is technically correct but unhelpful for new users who may not
realize they need to create the directory structure at all.

### Solutions

#### A. Check directory existence before reading mapping.toml (recommended)

```rust
if !theme_dir.exists() {
    errors.push(BuildError::Io {
        message: format!("theme directory not found: {}", theme_dir.display()),
    });
    continue;
}
```

| Pros | Cons |
|------|------|
| Clear error naming the missing directory | One extra filesystem check per theme |
| Better DX for new users | |
| Points to the directory, not a file inside it | |

#### B. Improve the error message for read failure

| Pros | Cons |
|------|------|
| No extra filesystem call | Still says "mapping.toml" not "directory" |

**Best solution: A.** Better error messages cost almost nothing.

---

## 21. `OnceLock` Caching Semantics Undocumented in Generated Code

**`codegen.rs:177-187`** -- The generated code uses a `static CACHED_DE:
OnceLock<LinuxDesktop>` that reads `XDG_CURRENT_DESKTOP` once and
caches forever. If the environment variable changes at runtime (rare
but possible in test frameworks, containers, or session managers), the
cached value is permanently stale.

The generated code has no comment explaining this behavior. Users
reading the generated file may not understand why their DE detection
is "stuck".

### Solutions

#### A. Add a doc comment in the generated code (recommended)

Generate a comment above the `OnceLock` block:

```rust
// Note: XDG_CURRENT_DESKTOP is read once and cached for the
// lifetime of the process. Changes after first access are not seen.
```

| Pros | Cons |
|------|------|
| Most discoverable place | Comment in generated code |
| Users reading the file understand the behavior | |

#### B. Document in the crate-level doc only

| Pros | Cons |
|------|------|
| No comment in generated code | Users may not read crate docs |

**Best solution: A.** The generated file is the first place users look
when debugging DE detection issues.

---

## 22. Codegen Silently Skips Unknown Theme Names

**`codegen.rs:201-204`** and **`codegen.rs:303-306`** -- Both
`generate_icon_name()` and `generate_icon_svg()` use `continue` when
`theme_name_to_qualified_icon_set()` returns `None`. If an unknown
theme somehow bypasses validation and reaches codegen, the generated
enum silently lacks match arms for that theme.

Validation at `validate.rs:23-33` should catch all unknown themes, so
this should be unreachable. But the `continue` silently hides bugs.

### Solutions

#### A. Replace `continue` with `debug_assert!` + `continue` (recommended)

```rust
let icon_set = match theme_name_to_qualified_icon_set(theme_name, crate_path) {
    Some(s) => s,
    None => {
        debug_assert!(false, "unknown theme {theme_name} reached codegen");
        continue;
    }
};
```

| Pros | Cons |
|------|------|
| Documents the invariant | No effect in release builds |
| Catches validation bypass during development | |
| `continue` is still the fallback | |

**Best solution: A.** Documents intent and catches bugs during
development.

---

## 23. All 4 Doctests Are `rust,ignore`

**`lib.rs:65`, `lib.rs:92`, `lib.rs:205`, `lib.rs:358`** -- Every
doctest in the crate is marked `rust,ignore`, meaning they are never
compiled or run by `cargo test`. They can silently drift from the
actual API.

Examining the doctests:
- Lines 65-70: Simple API example -- references `generate_icons()` and
  `unwrap_or_exit()`.
- Lines 92-99: Builder API example -- references `IconGenerator::new()`.
- Lines 205-210: `UnwrapOrExit` trait example.
- Lines 358-365: `derive()` method example.

All four require `CARGO_MANIFEST_DIR` and `OUT_DIR` to be set and a
real file to exist, which is why they are `ignore`.

### Solutions

#### A. Change `ignore` to `no_run` (recommended)

`no_run` compiles but does not execute. This catches API drift (renamed
methods, changed signatures) without needing fixture files at doctest
time.

| Pros | Cons |
|------|------|
| Catches compile-breaking API drift | Cannot verify runtime behavior |
| No filesystem setup needed | Slightly longer compile |
| Zero maintenance burden | |

#### B. Set up fixture files and make doctests fully runnable

| Pros | Cons |
|------|------|
| Full verification | Complex fixture management |
| | Fragile test infrastructure |

**Best solution: A.** `no_run` gives compile-time verification at zero
setup cost.

---

## Missing Test Coverage

### 24. No test for empty roles list behavior

When `roles = []`, the pipeline emits a warning (`lib.rs:586-590`) but
still generates code. No test verifies the generated enum is valid
(empty enum with `#[non_exhaustive]`, empty `ALL`, wildcard-only matches
in `icon_name` and `icon_svg`).

**Risk:** The generated code for zero roles might not compile (empty
enum, empty const array).

**Recommended:** Add a unit test that runs `run_pipeline` with
`roles = []` and verifies the output compiles (or at least contains
the expected structural elements).

### 25. No test for multiple DE overrides in a single mapping

All existing DE-aware tests use a single override (`kde`). No test
verifies behavior with multiple overrides (e.g., `kde`, `gnome`,
`xfce`) in the same mapping entry.

**Risk:** If the codegen generates overlapping match arms or incorrect
ordering, it would go undetected.

**Recommended:** Add a test with 3+ DE overrides and verify each
produces a separate match arm with the correct value.

### 26. No test for empty themes warning

`lib.rs:614-621` emits a warning when both `bundled_themes` and
`system_themes` are empty. This code path has no test.

**Recommended:** Add a test.

### 27. No test for DE-specific SVG non-requirement

`validate_svgs()` intentionally does NOT require DE-specific SVGs to
exist (it uses `default_name()` only). No test locks this behavior as
an intentional design choice.

**Risk:** If someone "fixes" this to require all SVGs, it breaks
system themes.

**Recommended:** Add a regression test with a DE-aware mapping and
verify no `MissingSvg` error for the DE-specific name.

### 28. `generate_icons()` simple API untested

The simple entry point `generate_icons()` at `lib.rs:247-279` is
never directly tested. All tests use the `IconGenerator` builder API
or `run_pipeline()` directly.

**Risk:** Regressions in the `generate_icons()` path-resolution or
env-var logic would go undetected.

**Recommended:** Add a test with explicit `CARGO_MANIFEST_DIR` and
`OUT_DIR` env var setup.

### 29. `base_dir()` builder method untested

The `base_dir()` method at `lib.rs:332-335` and its
divergent-base-dirs error path at `lib.rs:480-489` are both untested.

**Risk:** The base_dir logic could be broken without detection.

**Recommended:** Add integration tests for: (a) happy path with
explicit base_dir, (b) error path with divergent parent directories.

### 30. No test for Windows backslash path normalization

The `.replace('\\', "/")` normalization at `lib.rs:772` has no test.
On Linux, backslashes in paths are valid filename characters, so this
code path is never exercised by CI.

**Recommended:** Add a unit test that manually constructs a `PathBuf`
with backslashes and verifies the output contains forward slashes.

### 31. No test fixtures for freedesktop, lucide, or segoe-fluent themes

The committed test fixtures only cover `material` (bundled) and
`sf-symbols` (system). Three of the five supported themes have no
fixture files and no integration test coverage.

The `freedesktop` theme is used in integration tests but with
inline `write_file` calls, not committed fixtures. `lucide` and
`segoe-fluent` have no test coverage at all.

**Recommended:** Add fixture files for at least `freedesktop` (the
most complex theme due to DE-aware mappings).

### 32. Non-unique temp directories in tests

Both `lib.rs` tests (`create_fixture_dir`) and `integration.rs`
(`create_temp_dir`) use fixed-name temp directories:

```rust
std::env::temp_dir().join(format!("native_theme_test_pipeline_{suffix}"))
```

Concurrent test runs (e.g., `cargo test -j N` with test threads) can
collide when two tests use the same suffix or when the `remove_dir_all`
at the end races with another test's creation.

**Recommended:** Add `tempfile` as a dev-dependency and use
`tempfile::tempdir()` for automatic unique naming and cleanup.

### 33. No test verifying generated code actually compiles

The integration tests check that the generated string contains expected
substrings, but no test writes the generated code to a file and
compiles it with `rustc`. This means structural issues (mismatched
braces, invalid syntax) in the codegen could go undetected.

**Risk:** A codegen change could produce syntactically invalid Rust
that passes all substring-matching tests.

**Recommended:** Add one "golden" integration test that writes the
generated code to `OUT_DIR`, then `include!`s it from a test binary
that calls the generated methods. This requires the test binary to
depend on `native-theme`.

---

## Cross-Reference with platform-facts.md

### Icon set coverage: CORRECT

`THEME_TABLE` in `schema.rs:8-14` lists 5 sets:
- `sf-symbols` -> `IconSet::SfSymbols`
- `segoe-fluent` -> `IconSet::SegoeIcons`
- `freedesktop` -> `IconSet::Freedesktop`
- `material` -> `IconSet::Material`
- `lucide` -> `IconSet::Lucide`

platform-facts.md confirms:
- macOS: SF Symbols (section 1.1.6, lines 240-256)
- Windows: Segoe Fluent Icons (section 1.2.6, lines 486-511)
- Linux KDE/GNOME: freedesktop icon themes (section 1.3.6, line 670+)
- Material and Lucide are cross-platform bundled sets

The drift detection tests in `schema.rs:208-258` verify sync with
the runtime crate's `IconSet` enum. No mismatch found.

### DE table coverage: CORRECT

`DE_TABLE` in `schema.rs:21-29` lists 7 desktop environments:
- `kde` -> `LinuxDesktop::Kde`
- `gnome` -> `LinuxDesktop::Gnome`
- `xfce` -> `LinuxDesktop::Xfce`
- `cinnamon` -> `LinuxDesktop::Cinnamon`
- `mate` -> `LinuxDesktop::Mate`
- `lxqt` -> `LinuxDesktop::LxQt`
- `budgie` -> `LinuxDesktop::Budgie`

The drift detection tests in `schema.rs:266-343` verify sync with
the runtime crate's `LinuxDesktop` enum. These tests are correctly
`#[cfg(target_os = "linux")]` because `LinuxDesktop` and
`detect_linux_de()` only exist on Linux in the main crate.

### COSMIC desktop: Not yet supported

COSMIC is correctly treated as unrecognized. The integration test at
`integration.rs:408-453` verifies that `cosmic` produces a warning,
not an error. When `LinuxDesktop` adds a `Cosmic` variant, the drift
detection tests will catch the missing `DE_TABLE` entry automatically.

### No platform-specific value mismatches found

The build crate does not hardcode any platform-specific values (colors,
spacing, geometry). It only maps role names to icon identifier strings
and generates Rust code. All platform-specific behavior is delegated
to the runtime crate. This is correct by design.

---

## Summary

| # | Issue | Severity | Effort | Best Fix |
|---|-------|----------|--------|----------|
| 1 | `process::exit(1)` in `emit_cargo_directives()` | **High** | Low | Return `Result<(), io::Error>` |
| 2 | `assert!`/`assert_eq!` panics in production | **High** | Low | Defer validation to `generate()` |
| 3 | `escape_rust_str` does not escape null bytes | **Low** | Trivial | Add `'\0'` to escape table |
| 4 | Path traversal via icon names | **High** | Trivial | Reject `/`, `\`, `..` in validation |
| 5 | `crate_path` code injection | **Medium** | Trivial | Validate as Rust path |
| 6 | `extra_derives` code injection | **Medium** | Trivial | Validate as Rust path (share with #5) |
| 7 | `icon_svg`/`icon_name` DE-aware mismatch | **High** | Low | Make bundled DE-aware an error |
| 8 | Cross-file theme overlap not validated | **Medium** | Trivial | Post-merge validation |
| 9 | `pipeline_result_to_output` hidden I/O | **Medium** | Low | Move rerun paths to error type |
| 10 | Double slash in `include_bytes!` path | **Medium** | Trivial | Skip separator when empty |
| 11 | Orphan SVG ignores DE-specific names | **Low** | Trivial | Include all names in referenced set |
| 12 | SVG path/rerun tracking ignores DE names | **Low** | Trivial | Track all names (fix with #11) |
| 13 | `unwrap_or("unknown")` hides filenames | **Low** | Trivial | Use `to_string_lossy()` |
| 14 | `check_orphan_svgs` flattens dir errors | **Low** | Trivial | Emit warning per failure |
| 15 | `#[cfg(test)]` import misplaced | **Low** | Trivial | Move inside test module |
| 16 | `let _ = writeln!` intent unclear | **Low** | Trivial | Add explanatory comment |
| 17 | Incomplete `RUST_KEYWORDS` | **Low** | Trivial | Add comment explaining PascalCase |
| 18 | Repeated `to_upper_camel_case()` calls | **Low** | Trivial | Pre-compute in Vec |
| 19 | Silent name normalization | **Low** | Trivial | Emit warning on transform |
| 20 | Missing theme directory check | **Low** | Trivial | Early existence check |
| 21 | `OnceLock` caching undocumented | **Low** | Trivial | Add comment in generated code |
| 22 | Codegen silently skips unknown themes | **Low** | Trivial | Add `debug_assert!` |
| 23 | All doctests `ignore` | **Medium** | Trivial | Change to `no_run` |
| 24 | No test: empty roles list | **Medium** | Low | Add test |
| 25 | No test: multiple DE overrides | **Low** | Low | Add test |
| 26 | No test: empty themes warning | **Low** | Trivial | Add test |
| 27 | No test: DE-specific SVG non-requirement | **Low** | Trivial | Add regression test |
| 28 | `generate_icons()` simple API untested | **Medium** | Low | Add test with env vars |
| 29 | `base_dir()` builder method untested | **Medium** | Low | Add integration tests |
| 30 | No test: backslash path normalization | **Low** | Trivial | Add unit test |
| 31 | No fixtures for freedesktop/lucide/segoe | **Low** | Low | Add fixture files |
| 32 | Non-unique temp dirs in tests | **Low** | Low | Use `tempfile::tempdir()` |
| 33 | No test: generated code compiles | **Medium** | Medium | Add golden compile test |
