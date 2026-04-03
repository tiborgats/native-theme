# v0.5.4 -- native-theme-build: Deep Critical Analysis

Thorough review of the `native-theme-build` crate covering correctness,
safety, code quality, API design, and test coverage.

Files reviewed:
- `Cargo.toml`
- `src/lib.rs` (2160 lines), `src/codegen.rs` (1003 lines), `src/error.rs` (257 lines), `src/schema.rs` (344 lines), `src/validate.rs` (1020 lines)
- `tests/integration.rs` (502 lines)
- `tests/fixtures/sample-icons.toml`, `tests/fixtures/material/mapping.toml`, `tests/fixtures/sf-symbols/mapping.toml`
- `docs/platform-facts.md` (cross-referenced for icon/theme accuracy)

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

- **`lib.rs:345-347`** -- `crate_path()` builder method:
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

`validate_mapping_values()` at `validate.rs:346-368` does reject control
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

**`validate.rs:346-368`** -- `validate_mapping_values()` rejects
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

**`validate.rs:127-130`** -- `check_orphan_svgs()` builds its
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

**`lib.rs:820-828`** -- `check_orphan_svgs_and_collect_paths()` uses
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

**`validate.rs:150-153`** -- In `check_orphan_svgs()`:

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

**`validate.rs:144`** -- `for entry in entries.flatten()` silently
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

**`validate.rs:1-2`** -- `BTreeMap` is imported with `#[cfg(test)]` at
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

**`validate.rs:13-18`** -- The list has 38 entries but is missing
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

`codegen.rs:100`, `codegen.rs:110`, `codegen.rs:208`, `codegen.rs:312`
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

**`validate.rs:223-246`** -- The identifier validation accepts names
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

Validation at `validate.rs:24-34` should catch all unknown themes, so
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

**`lib.rs:65`**, **`lib.rs:92`**, **`lib.rs:205`**, **`lib.rs:358`**
-- Every doctest in the crate is marked `rust,ignore`, meaning they are
never compiled or run by `cargo test`. They can silently drift from the
actual API.

Examining the doctests:
- Lines 65-79: Simple and Builder API example -- references `generate_icons()`,
  `unwrap_or_exit()`, `IconGenerator::new()`.
- Lines 92-99: Generated code usage example.
- Lines 205-211: `UnwrapOrExit` trait example.
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

`lib.rs:615-621` emits a warning when both `bundled_themes` and
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

### 34. No test for `enum_name()` normalization

The `enum_name()` builder method at `lib.rs:319` stores the raw
string and `generate_code()` at `codegen.rs:77` applies
`to_upper_camel_case()`. No test verifies that an `enum_name` like
`"my-icons"` produces `pub enum MyIcons`. The existing test at
`lib.rs:1481-1493` (`merge_configs_uses_enum_name_override`) only
checks that the raw string reaches `merged.name` -- not that the
final generated code has the correct PascalCase form.

**Risk:** A regression in enum name normalization when using the
`enum_name()` override would go undetected.

**Recommended:** Add a test that calls `run_pipeline` with
`enum_name_override` set to a kebab-case name and asserts the
generated code contains the PascalCase version.

### 35. No test for `output_dir()` builder method

The `output_dir()` method at `lib.rs:380-382` is used in all
integration tests (to bypass `OUT_DIR`), but there is no test
verifying the behavior when `output_dir()` is NOT set and `OUT_DIR`
IS set. The `output_dir` vs `OUT_DIR` fallback path at
`lib.rs:417-423` is tested only indirectly.

**Recommended:** Add a test that sets `OUT_DIR` and does not call
`output_dir()`, verifying the output file lands in `OUT_DIR`.

---

## Inline Test Module Coverage Analysis

### `lib.rs` inline tests (63 tests)

Coverage is strong for the core pipeline:

- **MasterConfig deserialization**: 3 tests -- happy path, optional
  fields, unknown field rejection. Covers the serde boundary well.
- **MappingValue deserialization**: 3 tests -- Simple, DeAware, mixed.
  Good structural coverage.
- **MappingValue::default_name()**: 3 tests -- Simple, DeAware with
  default, DeAware without default. Complete branch coverage.
- **BuildError Display**: 7 tests -- one per variant (MissingRole,
  MissingSvg, UnknownRole, UnknownTheme, MissingDefault, DuplicateRole,
  InvalidIdentifier, IdentifierCollision, ThemeOverlap,
  DuplicateRoleInFile). Missing: `DuplicateTheme`, `InvalidIconName`.
  The newer error variants have no Display format test.
- **THEME_TABLE**: 1 test -- verifies 5 entries and all names present.
- **run_pipeline**: 7 tests -- happy path, output filename, rerun
  paths, size report, missing role error, missing SVG error, orphan SVG
  warnings. These are the most meaningful tests in the crate.
- **merge_configs**: 2 tests -- combines roles/themes, enum name
  override. Lacks test for deduplication edge cases.
- **Builder pipeline**: 2 tests -- merges two files, detects duplicate
  roles across files. No test for divergent base dirs.
- **include_bytes! relative paths**: 1 test -- verifies manifest_dir
  stripping. Important test.
- **System theme**: 1 test -- verifies system themes skip SVG checks.
- **BuildErrors**: 1 test -- Display format.
- **Identifier validation**: 3 tests -- collision, invalid keyword,
  duplicate in file.
- **DE-aware warnings**: 2 tests -- bundled DE-aware produces warning,
  system DE-aware does not.
- **crate_path**: 2 tests -- custom path used in impl, default emits
  extern crate.
- **Builder input validation**: 6 `#[should_panic]` tests for
  `derive()` and `crate_path()` assertions.

### `codegen.rs` inline tests (37 tests)

- **theme_name_to_icon_set**: 6 tests -- all 5 themes + unknown.
  Complete coverage.
- **theme_name_to_qualified_icon_set**: 1 test -- verifies crate path
  prefix.
- **generate_code structural**: 6 tests -- header, derives,
  non_exhaustive, enum name, variants, const ALL. Good.
- **IconProvider impl**: 7 tests -- icon_name for bundled/system,
  wildcards, include_bytes for bundled, no include_bytes for system,
  SVG path uses mapping value. Good.
- **de_key_to_variant**: 9 tests -- all 7 DEs + default + unknown.
  Complete.
- **DE-aware codegen**: 9 tests -- cfg gates, detect_linux_de call,
  KDE arm, default arm, OnceLock cache, non-Linux default, default-only
  collapse, simple value regression.
- **escape_rust_str**: 7 tests -- plain, backslash, quote, newline,
  carriage return, tab, combined. Thorough.
- **crate_path codegen**: 3 tests -- extern crate, custom path used,
  custom path with DE-aware.
- **extra_derives codegen**: 3 tests -- none, single, multiple.

### `validate.rs` inline tests (37 tests)

- **validate_themes**: 4 tests -- all known, unknown bundled, unknown
  system, multiple unknown. Good.
- **validate_mapping (VAL-01, VAL-03, VAL-04)**: 5 tests -- missing
  role, unknown role, missing default, all valid, DeAware with default.
- **validate_svgs (VAL-02)**: 3 tests -- missing SVG, all present,
  DeAware uses default name. Good.
- **check_orphan_svgs (VAL-05)**: 2 tests -- orphan found, no orphans.
- **validate_no_duplicate_roles (VAL-06)**: 3 tests -- duplicate found,
  no duplicates, three-file duplicate.
- **validate_de_keys**: 5 tests -- all recognized, cosmic unknown,
  mixed, default only, simple ignored.
- **validate_theme_overlap**: 3 tests -- detected, none, multiple.
- **validate_identifiers**: 7 tests -- valid, empty pascal, keyword
  role, keyword enum, collision, no collision, digit start role, digit
  start enum, all digits.
- **validate_no_duplicate_roles_in_file**: 4 tests -- detected, none,
  multiple, case-sensitive.
- **validate_no_duplicate_themes**: 3 tests -- bundled, system, none.
- **validate_mapping_values**: 4 tests -- valid, empty name, control
  char, DeAware empty value.

### `schema.rs` inline tests (11 tests)

- **THEME_TABLE consistency**: 4 tests -- no duplicate keys, no
  duplicate variants, variants start with IconSet::, keys non-empty.
- **DE_TABLE consistency**: 5 tests -- same pattern as THEME_TABLE,
  plus no "default" key.
- **is_known_theme**: 2 tests -- known returns true, unknown returns
  false.
- **Drift detection**: 4 tests -- THEME_TABLE entries match IconSet
  variants, covers all variants; DE_TABLE entries match LinuxDesktop
  variants, covers all variants. The DE tests are correctly
  `#[cfg(target_os = "linux")]`.

### `tests/integration.rs` (13 tests)

- **Happy path**: 7 tests -- correct enum, IconProvider impl, Material
  icon_name, SfSymbols icon_name, bundled icon_svg, const ALL, output
  filename, size report. These test the full public API through
  `IconGenerator`.
- **Error paths**: 3 tests -- missing role, unknown role, missing SVG.
  All use temp dirs with intentional errors.
- **Builder API**: 1 test -- merges disjoint roles from two files.
- **DE-aware**: 2 tests -- DE dispatch code generated, unknown DE key
  produces warning.

**Overall assessment of test quality:**
- Unit tests are focused and well-named; each tests one thing.
- The pipeline tests in `lib.rs` provide the most value because they
  exercise the full flow without I/O.
- Integration tests verify the public API surface well.
- The drift detection tests in `schema.rs` are an excellent pattern
  for catching enum mismatches between crates.
- The main gap is the absence of compile-verification: all codegen
  tests check substring presence, not syntactic validity (issue 33).
- `#[should_panic]` tests for builder assertions will need updating
  when issue 2 is resolved (move to `generate()` errors).

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

The drift detection tests in `schema.rs:210-258` verify sync with
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

The drift detection tests in `schema.rs:267-343` verify sync with
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

## 36. Missing `BuildError::Display` Tests for `DuplicateTheme` and `InvalidIconName`

**File:** `src/lib.rs:1027-1107` (existing Display test block)

The `test_display_*` tests at `lib.rs:1027-1107` cover `MissingRole`,
`MissingSvg`, `UnknownRole`, `UnknownTheme`, `MissingDefault`,
`DuplicateRole`, `InvalidIdentifier`, `IdentifierCollision`,
`ThemeOverlap`, `DuplicateRoleInFile` -- but NOT `DuplicateTheme` or
`InvalidIconName`. The `Display` implementations at `error.rs:158-175`
for these two variants are tested only indirectly through `validate.rs`
tests that use `.contains()` substring checks, not full format verification.

### Solutions

| # | Solution | Pros | Cons |
|---|----------|------|------|
| A | Add `Display` tests for both missing variants | Complete coverage; consistent with existing pattern; catches formatting regressions | Two trivial tests |
| B | Rely on indirect testing from `validate.rs` | No new code | Indirect tests check `contains()`, not full format; if validation tests change, Display gaps silently open |

**Recommended:** A. Two trivial tests matching the existing pattern at
`lib.rs:1027+`.

---

## 37. `enum_name_override` Not Validated for Output Filename Edge Cases

**File:** `lib.rs:319-321,577-581`

`enum_name()` builder stores raw string with no validation. If set to `"---"`,
`to_snake_case()` produces `""`, and `output_filename` becomes `".rs"`. The
filename is computed BEFORE validation runs at `lib.rs:624`.

**Recommended:** Validate that `output_filename` is not just `".rs"` after
snake_case conversion.

---

## 38. `merge_configs` Silently Deduplicates Themes Across Files

**File:** `lib.rs:849-864`

`merge_configs` silently deduplicates `bundled_themes` and `system_themes`
across files. No warning emitted. Users cannot tell why a theme is loaded
once instead of twice.

**Recommended:** Emit warning when cross-file dedup occurs.

---

## 39. `output_filename` Computed Twice From Parallel Sources

**Files:** `lib.rs:578-581` and `lib.rs:838`

`first_name` at line 578 and `merged.name` at line 838 compute the same
thing via different paths. If one changes without the other, the output
file name and enum name diverge.

**Recommended:** Derive `output_filename` from `merged.name` only.

---

## 40. `escape_rust_str` Scope Undocumented

**File:** `codegen.rs:11-24`

The function handles string literals but NOT format strings. No doc comment
states this limitation. If ever used inside `format!()`, unescaped `{`/`}`
would cause issues.

**Recommended:** Add doc comment clarifying scope.

---

## 41. Generated `icon_svg` Path Uses Escaped Name vs Validation Uses Raw Name

**File:** `codegen.rs:308-316` vs `validate.rs:100-116`

`escape_rust_str(icon_name)` is used in `include_bytes!` path, but
`validate_svgs` checks with unescaped `format!("{name}.svg")`. For icon
names with backslashes, these resolve to different filesystem paths.

**Recommended:** Resolved by issue 4's fix (reject `\` in icon names).

---

## 42. `#[should_panic]` Tests Are Tautological and Fragile

**File:** `lib.rs:2117-2158`

Six tests verify `assert!()` panics when its condition is false -- they
test the assert macro, not the builder. When issue 2 is implemented
(defer validation to `generate()`), all six become obsolete.

**Recommended:** Rewrite as `Result`-checking tests when issue 2 is fixed.

---

## 43. `validate_mapping_values` Does Not Check DeAware Keys

**File:** `validate.rs:346-368`

Checks values but not the DE keys themselves. An empty string DE key `""`
passes both `validate_mapping_values` and `validate_de_keys` (the latter
only warns about unrecognized keys).

**Recommended:** Validate DE keys for empty/control-char alongside values.

---

## 44. `check_orphan_svgs` Does Not Handle Symlinks

**File:** `validate.rs:133-161`

A symlink named `escape.svg` pointing outside the theme directory is treated
as a valid SVG. `svg_path.exists()` follows symlinks. Distinct from issue 4
(path traversal in icon names).

**Recommended:** Warn when a referenced SVG is a symlink outside theme dir.

---

## 45. Integration Test Cleanup Ignores `remove_dir_all` Errors (Stale Data Risk)

**File:** `tests/integration.rs:12-17`

`let _ = fs::remove_dir_all(&dir)` ignores ALL errors including permission
denied. A previous test run's stale data persists silently. Combined with
fixed directory names (issue 32), tests can pass against stale fixtures.

**Recommended:** Use `tempfile::tempdir()` (reinforces issue 32 with
additional justification). Issue 32 severity should be **Medium** not Low.

---

## 46. No Enum Variant vs Enum Name Collision Check

**File:** `validate.rs:223-286`

`validate_identifiers` does not check if a role's PascalCase matches the
enum name. `name = "play-pause"` with `roles = ["play-pause"]` produces
`pub enum PlayPause { PlayPause }` which is confusing.

**Recommended:** Warn when collision detected.

---

## 47. Generated `extern crate` May Produce Lint Warnings

**File:** `codegen.rs:86-89`

`extern crate native_theme;` generates `unused_extern_crates` warnings in
2021+ editions.

**Recommended:** Add `#[allow(unused_extern_crates)]` above the line.

---

## 48. `icon_svg_has_wildcard` Test Is Tautological

**File:** `codegen.rs:604-616`

Checks `output.matches("_ => None").count() >= 2` but `_ => None` is always
emitted regardless of match arms. Would pass even with all arms removed.

**Recommended:** Replace with tests checking specific match arms, or delete
(other tests already cover them).

---

## 49. `generate_icons()` Simple API Limitations Undocumented

**File:** `lib.rs:232-279`

`generate_icons()` always passes `None` for `crate_path` and `&[]` for
`extra_derives`. The doc comment does not mention these limitations.

**Recommended:** Add doc sentence listing what it does NOT support.

---

## 50. `IdentifierCollision` Error Lacks File Context

**File:** `validate.rs:248-283`

`validate_identifiers` operates on the merged config and has no file context.
With multiple source files, the collision error does not say which file
contains the offending roles.

**Recommended:** Pass file path context to `validate_identifiers`.

---

## 51. `include_bytes!` Path Malformed With Absolute `base_dir`

**File:** `lib.rs:767-775`, `codegen.rs:316`

When `manifest_dir: None` and `base_dir` is absolute, generated path
concatenates `CARGO_MANIFEST_DIR` with an already-absolute path.

**Recommended:** Skip `CARGO_MANIFEST_DIR` concat when `base_dir_str`
starts with `/`.

---

## 52. `validate_identifiers` on Merged Config Loses Per-File Context

**File:** `lib.rs:624`, `validate.rs:223-286`

Keyword/digit-starting role errors have no source file path. Distinct from
issue 50 (collisions only). All `InvalidIdentifier` errors are affected.

**Recommended:** Run `validate_identifiers` per-file before merge too.

---

## 53. DE-Aware Mappings Where All Overrides Equal Default Generate Redundant Code

**File:** `codegen.rs:226-274`

`{ kde = "play", gnome = "play", default = "play" }` generates full
cfg/match code when a simple arm suffices.

**Recommended:** Collapse to simple arm when all values equal default.

---

## 57. Per-File PascalCase Collisions Lose File Context

**File:** `validate.rs:292-309`

`validate_no_duplicate_roles_in_file` checks exact strings only.
PascalCase collisions caught by merged validator lack file path.

**Recommended:** Add PascalCase collision check to per-file validator.

---

## 58. Integration Test Warning Assertions Coupled to Exact Prose

**File:** `tests/integration.rs:435-441`

Substring matching on `"unrecognized DE key"` breaks on wording changes.

**Recommended:** Extract stable prefix or use structured warnings.

---

## 59. `validate_themes` `UnknownTheme` Error Lacks Source File

**File:** `lib.rs:633`, `validate.rs:24-34`

When multiple source files provide themes, error doesn't say which file
contains the unknown theme name.

**Recommended:** Run `validate_themes` per-file in the validation loop.

---

## 61. `builder_merges_disjoint_roles` Test Does Not Verify Variant Ordering

**File:** `tests/integration.rs:287-341`

Checks presence of `PlayPause` and `SkipForward` but not their ORDER.
Merge ordering contract is undocumented.

**Recommended:** Add position-order assertion.

---

## 62. Invisible Unicode Passes Validation and Escaping Unchanged

**File:** `codegen.rs:11-24`, `validate.rs:346-368`

BOM (`\u{FEFF}`), zero-width space (`\u{200B}`) pass both validation and
`escape_rust_str` unchanged. Generated code contains invisible characters.

**Recommended:** Reject invisible Unicode in icon name validation.

---

## 64. Default-Only DeAware Test Does Not Assert `!OnceLock`

**File:** `codegen.rs:787-803`

Test checks simple arm exists and no `detect_linux_de`, but does NOT check
that `OnceLock` cache is omitted. Would pass if OnceLock incorrectly emitted.

**Recommended:** Add `!output.contains("OnceLock")` assertion.

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
| 32 | Non-unique temp dirs in tests | **Medium** | Low | Use `tempfile::tempdir()` |
| 33 | No test: generated code compiles | **Medium** | Medium | Add golden compile test |
| 34 | No test: `enum_name()` normalization | **Low** | Trivial | Add codegen test |
| 35 | No test: `output_dir()` vs `OUT_DIR` fallback | **Low** | Trivial | Add test |
| 36 | Missing `BuildError::Display` tests for 2 variants | **Low** | Trivial | Add 2 format tests |
| 37 | `enum_name_override` filename edge case | **Low** | Trivial | Validate non-empty after snake_case |
| 38 | `merge_configs` silent cross-file theme dedup | **Low** | Trivial | Emit warning on dedup |
| 39 | `output_filename` computed twice from parallel sources | **Low** | Trivial | Derive from `merged.name` only |
| 40 | `escape_rust_str` scope undocumented | **Low** | Trivial | Add doc comment |
| 41 | `icon_svg` path escaping vs validation | **Low** | None | Resolved by issue 4 fix |
| 42 | `#[should_panic]` tests tautological | **Low** | Low | Rewrite when issue 2 is fixed |
| 43 | `validate_mapping_values` skips DeAware keys | **Low** | Trivial | Validate keys alongside values |
| 44 | `check_orphan_svgs` ignores symlinks | **Low** | Low | Warn on external symlinks |
| 45 | Integration test cleanup ignores errors | **Medium** | Low | Use `tempfile::tempdir()` |
| 46 | No enum variant vs enum name collision check | **Low** | Trivial | Warn on collision |
| 47 | Generated `extern crate` lint warning | **Low** | Trivial | Add `#[allow]` attribute |
| 48 | `icon_svg_has_wildcard` test tautological | **Low** | Trivial | Replace or delete |
| 49 | `generate_icons()` limitations undocumented | **Low** | Trivial | Add doc sentence |
| 50 | `IdentifierCollision` lacks file context | **Medium** | Low | Pass file path to validator |
| 51 | `include_bytes!` path malformed with absolute base_dir | **Medium** | Low | Skip CARGO_MANIFEST_DIR for absolute paths |
| 52 | `validate_identifiers` merged loses per-file context | **Medium** | Low | Also run per-file before merge |
| 53 | DE-aware redundant code when all overrides equal default | **Low** | Trivial | Collapse to simple arm |
| 57 | Per-file PascalCase collisions lose context | **Medium** | Low | Add PascalCase to per-file validator |
| 58 | Integration test warning assertions fragile | **Low** | Low | Extract stable prefix |
| 59 | `UnknownTheme` error lacks source file | **Medium** | Trivial | Run per-file |
| 61 | Merge ordering not tested | **Low** | Trivial | Add position assertion |
| 62 | Invisible Unicode passes validation | **Low** | Low | Reject invisible chars |
| 64 | Default-only DeAware test missing OnceLock assertion | **Low** | Trivial | Add assertion |

---

## New Findings: Second-Pass Deep Audit

### 65. Generated Code Missing `#[allow(unused_imports)]` for `std::sync::OnceLock`

**Category:** codegen-bug
**Severity:** low
**File(s):** `src/codegen.rs:181`

**Problem:** When DE-aware mappings are present, the generated code emits
`static CACHED_DE: std::sync::OnceLock<...>` inside a `#[cfg(target_os = "linux")]`
block. On non-Linux targets, this entire block is compiled out. However, the
generated code does not contain a `#[cfg(target_os = "linux")]` guard around
`use std::sync::OnceLock` (it uses the fully-qualified path inline instead).
This is technically fine because the `OnceLock` type is referenced only inside
the `cfg`-gated block.

However, if the generated code is ever refactored to use a `use` import for
cleanliness, the import would trigger an `unused_imports` warning on non-Linux.
Additionally, the `std::env::var` call on line 185 is also inside the cfg block,
but `std::env` is used without import. No actual warning is produced today, but
this is fragile.

This is distinct from issue 47 (which covers `extern crate` lint warnings).

**Solution Options:**

1. **Add `#[allow(dead_code, unused_imports)]` at the top of the generated file**
   - *Pros:* Prevents any future warnings from cfg-gated code paths
   - *Cons:* Overly broad suppression; hides real unused items

2. **Keep status quo (fully-qualified paths avoid the issue)**
   - *Pros:* No generated code change; works correctly today
   - *Cons:* Fragile if codegen style changes; no protection for future refactors

**Best Solution:** 2 -- The current approach of using fully-qualified paths
(`std::sync::OnceLock`, `std::env::var`) avoids the warning entirely.
This is a non-issue today but worth noting for future codegen changes.

---

### 66. `MappingValue::DeAware` Accepts Empty Table `{}` Without Error

**Category:** validation-bug
**Severity:** medium
**File(s):** `src/schema.rs:76-81`, `src/validate.rs:71-79`

**Problem:** The `#[serde(untagged)]` deserialization of `MappingValue` tries
`Simple(String)` first, then `DeAware(BTreeMap<String, String>)`. A TOML
value of `play-pause = {}` (empty inline table) deserializes as
`DeAware(BTreeMap::new())` -- an empty map with no keys at all.

`validate_mapping` at `validate.rs:71-79` checks for missing `"default"` key
and correctly reports `MissingDefault` for this case. So validation catches it.

However, the error message says `DE-aware mapping for "play-pause" is missing
the required "default" key`, which is confusing when the user wrote `{}` -- they
may not realize they created a "DE-aware mapping" at all. The user likely
intended a simple string and made a TOML syntax error.

**Solution Options:**

1. **Add a specific error or warning for empty DeAware maps**
   - *Pros:* Better DX; points user to the real problem (TOML syntax error)
   - *Cons:* One more check in validation

2. **Keep current behavior (MissingDefault catches it)**
   - *Pros:* No code change; technically caught
   - *Cons:* Confusing error message for a common TOML typo

**Best Solution:** 1 -- Check for empty DeAware maps explicitly and emit
a more helpful error: `mapping for "play-pause" in {file} is an empty table;
expected a string or a table with at least a "default" key`.

---

### 67. `validate_svgs` Only Checks Default Name for DE-Aware Bundled Mappings

**Category:** validation-bug
**Severity:** low
**File(s):** `src/validate.rs:93-115`

**Problem:** `validate_svgs()` uses `value.default_name()` at line 103 to
determine which SVG file must exist. For a DE-aware bundled mapping like
`{ kde = "media-playback-start", default = "play_pause" }`, only
`play_pause.svg` is checked.

This is related to issues 11 and 12 (orphan SVG detection and path tracking)
but is a distinct code path: `validate_svgs` is about *required* SVGs, not
orphan detection. The existing document covers this only indirectly through
issue 7 (bundled DE-aware mismatch), which proposes making bundled DE-aware
an error. If issue 7 is NOT implemented (i.e., bundled DE-aware remains a
warning), then `validate_svgs` is correct -- only the default SVG is embedded,
so only it needs to exist.

This finding is documented for completeness: it is NOT a bug given the
current design, but becomes one if bundled DE-aware is ever allowed without
issue 7's fix. The dependency chain is: if issue 7 is resolved (bundled
DE-aware becomes an error), this is moot. If issue 7 is deferred, this
is correct behavior.

**Solution Options:**

1. **No change needed if issue 7 is implemented**
   - *Pros:* Correct by construction
   - *Cons:* Depends on issue 7

2. **Add a comment documenting the dependency on issue 7's design decision**
   - *Pros:* Future-proofs the code against accidental changes
   - *Cons:* One comment

**Best Solution:** 2 -- Add a comment in `validate_svgs` noting that it
intentionally checks only the default name because bundled DE-aware
mappings only embed the default SVG.

---

### 68. `merge_configs` Does Not Validate Cross-File Theme Overlap

**Category:** validation-bug
**Severity:** medium
**File(s):** `src/lib.rs:834-874`, `src/lib.rs:612-633`

**Problem:** This is already documented as issue 8. However, the existing
document's analysis misses a subtlety: `merge_configs` at lines 856-864
deduplicates themes into separate `seen_bundled` and `seen_system` sets.
If file A has `bundled-themes = ["material"]` and file B has
`system-themes = ["material"]`, the merged config has "material" in BOTH
`bundled_themes` and `system_themes` because they use separate `BTreeSet`s.

The per-file `validate_theme_overlap` at line 597 only checks within a
single file. The post-merge `validate_themes` at line 633 only checks
for unknown theme names, NOT overlap. So the overlap goes entirely
undetected.

**This is already covered by issue 8.** No new issue needed.

---

### 69. No Test for `BuildErrors::is_empty()` or `BuildErrors::len()`

**Category:** test-gap
**Severity:** low
**File(s):** `src/error.rs:223-231`

**Problem:** `BuildErrors` exposes `is_empty()` and `len()` methods at
`error.rs:223-231`. Neither has a dedicated unit test. They are tested only
indirectly through integration tests that call `errors.is_empty()` in
assertions, but the methods' return values for non-empty collections are
never explicitly verified.

The `IntoIterator` implementations at `error.rs:241-257` are also untested
directly, though they are used implicitly in `for e in &errors` loops
in the Display implementation tests.

**Solution Options:**

1. **Add targeted tests for `is_empty`, `len`, `into_iter`, and `iter`**
   - *Pros:* Documents the API contract; catches regressions in trivial methods
   - *Cons:* Tests for trivial delegation to Vec

2. **Skip (methods are trivial delegations)**
   - *Pros:* No test bloat
   - *Cons:* Small API surface untested

**Best Solution:** 1 -- These are public API methods. A single test
function covering all four is sufficient:
`assert!(!errors.is_empty()); assert_eq!(errors.len(), 2);
assert_eq!(errors.into_iter().count(), 2);`.

---

### 70. `check_orphan_svgs` Does Not Consider DE-Specific SVGs From Other Roles

**Category:** validation-bug
**Severity:** low
**File(s):** `src/validate.rs:121-161`

**Problem:** Issue 11 documents that `check_orphan_svgs` misses DE-specific
names from the same role. But there is an additional dimension: the
`referenced` set at line 127 is built from `mapping.values()` using
`default_name()`. This means only the *default* name of each role is
considered referenced.

In practice, this means:
- Role A maps to `{ kde = "icon-kde-a", default = "icon-a" }`
- Role B maps to `"icon-b"` (simple)
- SVG `icon-kde-a.svg` exists in the directory

The SVG `icon-kde-a.svg` is reported as orphan even though it belongs to
role A's KDE-specific mapping.

**This is fully covered by issue 11's fix** (include all icon names in
the referenced set). No new issue needed -- this is the same root cause.

---

### 71. Generated `match de` Block Uses `_ =>` Wildcard That Matches `LinuxDesktop::Unknown`

**Category:** codegen-bug
**Severity:** low
**File(s):** `src/codegen.rs:256-259`

**Problem:** The generated DE dispatch code at `codegen.rs:256-259` emits:

```rust
match de {
    native_theme::LinuxDesktop::Kde => Some("media-playback-start"),
    _ => Some("play"),
}
```

The `_ =>` wildcard matches all `LinuxDesktop` variants not explicitly
listed, including `LinuxDesktop::Unknown`. This means that when the desktop
environment is unknown/unrecognized, the default icon name is returned.

This is actually correct behavior -- the `default` key in the TOML mapping
IS intended as the fallback for all unmatched DEs including Unknown. The
runtime crate's `detect_linux_de` returns `LinuxDesktop::Unknown` for
unrecognized desktops, and the default icon name is the appropriate
fallback.

However, if a new `LinuxDesktop` variant is added (e.g., `Cosmic`), the
wildcard silently captures it and returns the default instead of
potentially returning `None`. This is a non-exhaustive match concern.

The drift detection tests in `schema.rs:313-343` will catch the missing
DE_TABLE entry when a new variant is added. And even if DE_TABLE is
updated, the wildcard in generated code is still correct (unknown DE
variants should get the default). So this is actually a correct design
choice.

**This is not a bug.** The `_ =>` wildcard correctly handles both
`LinuxDesktop::Unknown` and any future variants. The drift detection
tests in `schema.rs` provide the safety net for table updates.

---

### 72. `has_any_de_aware_mappings` Only Checks Known DE Keys

**Category:** codegen-bug
**Severity:** low
**File(s):** `src/codegen.rs:135-159`

**Problem:** `has_any_de_aware_mappings()` at line 150 checks whether any
DE-aware mapping has a non-default key that maps to a *known* DE variant
(`de_key_to_variant(k).is_some()`). If a mapping has ONLY unrecognized DE
keys (e.g., `{ cosmic = "x", default = "y" }`), the function returns
`false`, and no `OnceLock` cache is emitted.

Meanwhile, `generate_icon_name()` at line 226-233 filters DE entries the
same way (via `de_key_to_qualified_variant`), so unrecognized keys produce
no match arms. The DE-aware value correctly collapses to a simple arm at
line 235-240.

So the OnceLock is correctly omitted when no known DE keys produce match
arms. The behavior is consistent: if no known DE variants are referenced,
no DE dispatch code is generated, and no cache is needed.

**This is not a bug.** The logic is correct and consistent between
`has_any_de_aware_mappings` and `generate_icon_name`. Both filter on
known DE keys.

---

### 73. `native-theme` Is a Dev-Dependency But Used in `#[cfg(test)]` Schema Drift Tests

**Category:** api-inconsistency
**Severity:** low
**File(s):** `Cargo.toml:19`, `src/schema.rs:210-343`

**Problem:** The `native-theme` crate is listed in `[dev-dependencies]` at
`Cargo.toml:19`. The drift detection tests in `schema.rs:210-343` import
`native_theme::IconSet` and `native_theme::LinuxDesktop` to verify that
`THEME_TABLE` and `DE_TABLE` stay in sync with the runtime crate.

This is correct Rust practice -- `#[cfg(test)]` code can use dev-dependencies.
However, it means that **the build crate's production code has no compile-time
link to the runtime crate's types**. The string constants in `THEME_TABLE`
and `DE_TABLE` (e.g., `"IconSet::SfSymbols"`) are opaque strings that are
only verified by tests, not by the type system.

If someone modifies the runtime crate's `IconSet` enum variant names (e.g.,
renames `SfSymbols` to `AppleSfSymbols`) without running the build crate's
tests, the generated code would reference a non-existent variant.

The existing drift detection tests mitigate this well. This is not a bug
but a design trade-off worth noting.

**This is not a bug.** The drift detection tests at `schema.rs:210-258`
and `schema.rs:267-343` provide strong protection against this scenario.
No action needed.

---

### 74. `validate_no_duplicate_roles_in_file` Does Not Catch TOML-Level Duplicates

**Category:** validation-bug
**Severity:** medium
**File(s):** `src/validate.rs:292-309`, `src/lib.rs:592-594`

**Problem:** TOML specification says that duplicate keys in a table are
forbidden. The `toml` crate (v1.1.0) rejects duplicate keys at parse time
with a parse error. So a TOML file with:

```toml
roles = ["play-pause", "skip-forward"]
roles = ["volume-up"]
```

would fail at `toml::from_str()` before `validate_no_duplicate_roles_in_file`
ever runs.

However, `validate_no_duplicate_roles_in_file` checks for duplicates
*within* the `roles` array:

```toml
roles = ["play-pause", "play-pause"]
```

TOML arrays DO allow duplicate values (only table keys must be unique).
So this validation is correct and necessary.

The test at `validate.rs:912-923` constructs a `MasterConfig` directly
(not via TOML parsing) to test this, which is correct since TOML parsing
alone cannot produce a `roles` array with duplicates from a valid TOML file.

Wait -- TOML arrays *can* contain duplicate string values. A TOML file with
`roles = ["play-pause", "play-pause"]` is valid TOML and produces a Vec
with two identical entries. The `toml` crate deserializes this without
error. So `validate_no_duplicate_roles_in_file` is a necessary check.

**This is not a bug.** The validation correctly catches duplicate entries
in TOML arrays, which are valid TOML but semantically incorrect.

---

### 75. `MasterConfig` `name` Field Allows Empty String

**Category:** validation-bug
**Severity:** medium
**File(s):** `src/schema.rs:47-58`, `src/validate.rs:223-246`

**Problem:** `MasterConfig.name` is a `String` field with no serde
validation. A TOML file with `name = ""` deserializes successfully.
`validate_identifiers` at `validate.rs:227-228` checks the PascalCase
conversion of the name and catches the empty-string case (produces
`InvalidIdentifier` with reason "PascalCase conversion produces an
empty string").

However, issue 37 already documents a related edge case:
`output_filename` is computed at `lib.rs:581` BEFORE validation runs.
For `name = ""`, `to_snake_case("")` produces `""`, and
`output_filename` becomes `".rs"`. The validation error at line 624
catches the identifier issue, but by then `output_filename` has already
been set to `".rs"`.

Since the pipeline returns errors without writing any file (line 751-759),
the malformed filename is never used on the error path. On the success
path (which cannot happen with `name = ""`), the filename would be wrong.
But since validation prevents the success path, this is a latent bug
that cannot be triggered.

**This is already covered by issue 37.** No new issue needed.

---

### 76. `generate_icon_svg` Uses `default_name()` for `include_bytes!` Path Even for DE-Aware Values

**Category:** codegen-bug
**Severity:** low
**File(s):** `src/codegen.rs:308-316`

**Problem:** At `codegen.rs:309-311`, the `icon_svg` match arm construction
uses `mv.default_name()` to get the SVG filename for `include_bytes!`.
For a DE-aware bundled value like `{ kde = "media-playback-start",
default = "play_pause" }`, only `play_pause.svg` is embedded.

The `icon_svg` method returns this same SVG data regardless of the active
desktop environment. But `icon_name()` returns different strings depending
on the DE (issue 7's mismatch).

**This is already fully covered by issue 7.** No new issue needed.

---

### 77. No Validation That `name` Field Contains Only Safe Characters for Filenames

**Category:** validation-bug
**Severity:** low
**File(s):** `src/lib.rs:578-581`, `src/validate.rs:223-246`

**Problem:** The `name` field from TOML is used to derive the output
filename via `to_snake_case()` at `lib.rs:581`:

```rust
let output_filename = format!("{}.rs", first_name.to_snake_case());
```

`to_snake_case()` from the `heck` crate strips non-alphanumeric characters
and converts to lowercase with underscores. For most inputs this produces
safe filenames. But edge cases exist:

- `name = "..."` produces `to_snake_case("...") = ""` -> filename `".rs"`
  (issue 37)
- `name = "a/b"` produces `to_snake_case("a/b") = "a_b"` -> filename
  `"a_b.rs"` (safe, path separator stripped)
- `name = "NUL"` on Windows produces `to_snake_case("NUL") = "nul"` ->
  filename `"nul.rs"` which is a Windows reserved device name

The Windows reserved name issue is theoretical (the build crate is a
build-time tool running on the developer's machine, not in production).
The identifier validation catches most problematic names because
PascalCase conversion filters aggressively.

**Solution Options:**

1. **Add Windows reserved name check to filename generation**
   - *Pros:* Prevents rare Windows build failure
   - *Cons:* Extremely unlikely scenario; `heck` already sanitizes most inputs

2. **No change (issue 37's fix subsumes the empty-string case)**
   - *Pros:* No code change
   - *Cons:* Windows reserved names remain theoretically possible

**Best Solution:** 2 -- The practical risk is negligible. A TOML file with
`name = "nul"` would produce `pub enum Nul` which is a valid Rust
identifier but a problematic filename only on Windows. Issue 37's fix
(validate non-empty after snake_case) covers the worst case.

---

### 78. `validate_mapping` Quadratic Complexity for Large Role Lists

**Category:** api-inconsistency
**Severity:** low
**File(s):** `src/validate.rs:42-82`

**Problem:** `validate_mapping()` at lines 49-57 iterates over all
`master_roles` and calls `mapping.contains_key(role)` for each. Since
`mapping` is a `BTreeMap`, each `contains_key` is O(log n). The overall
complexity is O(m * log n) where m = number of roles and n = number of
mapping entries.

At line 59, a `BTreeSet` is built from `master_roles` for the reverse
check (unknown roles in mapping). This set construction is O(m * log m).
The reverse check iterates mapping keys (O(n)) with set lookups (O(log m))
for O(n * log m) total.

For any realistic number of roles (tens to low hundreds), this is
completely fine. The observation is recorded for completeness, not as
an actionable issue.

**This is not a bug.** Performance is irrelevant at build-time for
realistic input sizes.

---

### 79. `check_orphan_svgs_and_collect_paths` Only Tracks SVGs That Exist on Disk

**Category:** validation-bug
**Severity:** low
**File(s):** `src/lib.rs:812-831`

**Problem:** At `lib.rs:823`, `check_orphan_svgs_and_collect_paths` only
adds an SVG to `rerun_paths` and `svg_paths` if `svg_path.exists()` returns
true. If the SVG file is created AFTER the build script runs but BEFORE
the next build, cargo will not detect the change because the path was never
registered for watching.

However, this scenario is already handled: if the SVG does not exist,
`validate_svgs` (run earlier at line 664) will report a `MissingSvg`
error, and the pipeline will abort before reaching
`check_orphan_svgs_and_collect_paths`. So the `.exists()` check at line
823 is effectively dead code on the success path -- all SVGs must exist
for the pipeline to reach this point.

On the error path, the pipeline returns at line 751-759 and
`pipeline_result_to_output` emits rerun-if-changed for all collected
paths (including the master TOML and mapping files). When the user
creates the missing SVG, the mapping.toml file's containing directory
is already tracked at line 648, which will trigger a rebuild.

**This is not a bug.** The `.exists()` guard is redundant but harmless
on the success path. On the error path, directory-level tracking provides
sufficient rebuild coverage.

---

### 80. `Cinnamon` DE Detection Mismatch Between Build and Runtime Crates

**Category:** schema-gap
**Severity:** medium
**File(s):** `src/schema.rs:25` (DE_TABLE), runtime crate `src/lib.rs:207`

**Problem:** The runtime crate's `detect_linux_de()` at `lib.rs:207`
matches both `"X-Cinnamon"` and `"Cinnamon"` for `LinuxDesktop::Cinnamon`:

```rust
"X-Cinnamon" | "Cinnamon" => return LinuxDesktop::Cinnamon,
```

The drift detection test at `schema.rs:272-279` maps `("cinnamon", "Cinnamon")`
for the XDG value, which correctly detects `Cinnamon`. Both string variants
in the runtime crate resolve to the same `LinuxDesktop::Cinnamon` variant.

The `DE_TABLE` entry `("cinnamon", "LinuxDesktop::Cinnamon")` is used for
the *TOML key* in mapping files, not for XDG detection. The TOML key
`cinnamon` in a `mapping.toml` file generates a match arm
`LinuxDesktop::Cinnamon => Some("...")` in the output code. This match arm
correctly catches both `X-Cinnamon` and `Cinnamon` XDG values because
the runtime `detect_linux_de` returns the same enum variant for both.

**This is not a bug.** The layering is correct: TOML keys map to enum
variants, and the runtime crate handles the XDG string-to-variant mapping.

---

### 81. No Validation That Enum Name Does Not Collide With Generated `impl` Block Items

**Category:** validation-bug
**Severity:** low
**File(s):** `src/validate.rs:223-246`, `src/codegen.rs:106-114`

**Problem:** The generated code creates `impl {EnumName}` with
`pub const ALL: &[Self]`. If the enum name were somehow identical to a
type already in scope (e.g., the user names their enum `IconSet`), the
generated `impl IconSet` would collide with or shadow the imported
`native_theme::IconSet`.

In practice, the generated code uses `Self` inside the impl block, and
the enum is defined in the user's module (via `include!`), so there is no
actual name collision with the imported `native_theme::IconSet` -- they
are different types in different modules. The `impl` block applies to
the locally defined enum, not the imported one.

**This is not a bug.** Rust's module system prevents actual collisions.
A user naming their enum `IconSet` would get confusing code but no
compile error.

---

### 82. `generate_icons()` Does Not Support `crate_path` or `extra_derives`

**Category:** api-inconsistency
**Severity:** low
**File(s):** `src/lib.rs:247-279`

**Problem:** The simple `generate_icons()` API at line 269-276 passes
`None` for `crate_path` and `&[]` for `extra_derives` to `run_pipeline`.
Users of the simple API cannot customize these.

**This is already covered by issue 49** which notes that the simple API's
limitations are undocumented. No new issue needed.

---

### 83. `BuildErrors::new()` Accepts Empty Vec

**Category:** api-inconsistency
**Severity:** low
**File(s):** `src/error.rs:202-204`

**Problem:** `BuildErrors::new(vec![])` creates a `BuildErrors` instance
with zero errors. `is_empty()` returns `true` and `Display` shows
"0 build error(s):". This is semantically odd -- a "build errors"
collection with no errors.

The constructor is `pub(crate)`, so external users cannot create empty
`BuildErrors`. Internally, the only caller that could produce this is
`pipeline_result_to_output` at `lib.rs:881`, but it checks
`!result.errors.is_empty()` before constructing `BuildErrors`.

**Solution Options:**

1. **Add `debug_assert!(!errors.is_empty())` in `BuildErrors::new()`**
   - *Pros:* Documents invariant; catches internal misuse in debug builds
   - *Cons:* Minimal value since constructor is pub(crate) only

2. **No change (internal-only, guarded by caller)**
   - *Pros:* No code change
   - *Cons:* Invariant not enforced

**Best Solution:** 1 -- A `debug_assert` documents the intent that
`BuildErrors` should always contain at least one error.

---

### 84. `merge_configs` Uses First Config's Name Without Warning When Multiple Configs Have Different Names

**Category:** api-inconsistency
**Severity:** low
**File(s):** `src/lib.rs:838-840`

**Problem:** When merging multiple configs without an `enum_name_override`,
`merge_configs` at line 838-840 silently uses the first config's `name`:

```rust
let name = enum_name_override
    .map(|s| s.to_string())
    .unwrap_or_else(|| configs[0].1.name.clone());
```

If file A has `name = "media-icons"` and file B has `name = "nav-icons"`,
the merged enum is silently named `MediaIcons` with no warning that file
B's name was discarded.

This is similar to issue 38 (silent theme deduplication) but applies to
the enum name, which is arguably more surprising -- the user explicitly
chose a name in each file.

**Solution Options:**

1. **Emit a warning when multiple configs have different names and no override is set**
   - *Pros:* Users know their name choice was discarded
   - *Cons:* One more warning

2. **Require `enum_name()` override when multiple sources have different names**
   - *Pros:* No ambiguity
   - *Cons:* Breaking change for existing multi-file setups (if any)

3. **No change (document the behavior)**
   - *Pros:* No code change
   - *Cons:* Surprising behavior

**Best Solution:** 1 -- Emit a warning. The user should know that only
the first file's name is used when merging without an explicit override.
