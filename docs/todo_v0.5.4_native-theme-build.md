# v0.5.4 -- native-theme-build: Critical Analysis

Deep review of the `native-theme-build` crate covering unit tests,
correctness, code quality, and API design.

Files reviewed:
- `Cargo.toml`
- `src/lib.rs`, `src/codegen.rs`, `src/error.rs`, `src/schema.rs`, `src/validate.rs`
- `tests/integration.rs`
- `tests/fixtures/material/mapping.toml`, `tests/fixtures/sample-icons.toml`, `tests/fixtures/sf-symbols/mapping.toml`
- `docs/platform-facts.md` (1475 lines, cross-referenced)

---

## 1. `assert!` / `assert_eq!` Panics in Production Code

Three locations use panic-based assertions in the public API and
internal pipeline:

- `lib.rs:345-348`: `crate_path()` calls `assert!(!path.is_empty() && !path.contains(' '))`.
- `lib.rs:366-370`: `derive()` calls `assert!(!name.is_empty() && !name.contains(char::is_whitespace))`.
- `lib.rs:569`: `run_pipeline()` calls `assert_eq!(configs.len(), base_dirs.len())`.

This violates the project's no-panic rule. A user typo in `.crate_path("")`
or `.derive("Foo Bar")` crashes the build script with a panic backtrace
instead of returning a structured `BuildErrors` result. The internal
`assert_eq!` guards an invariant that should hold but produces an
unhelpful message if triggered.

### Solutions

#### A. Defer validation to `generate()` (recommended)

Store raw input in the builder. Validate in `generate()` / `run_pipeline()`
and return errors via the normal `BuildErrors` collection. Convert the
internal `assert_eq!` to `debug_assert_eq!`.

| Pros | Cons |
|------|------|
| All errors flow through `BuildErrors` | Errors surface at generate time, not builder time |
| No panics in any code path | Slightly more validation code in run_pipeline |
| Consistent error experience | |

#### B. Return `Result` from builder methods

| Pros | Cons |
|------|------|
| Immediate validation at call site | Breaks fluent `.source().crate_path().generate()` pattern |
| Clear error on bad input | Every builder call needs `?` |

#### C. Keep asserts, document the panic behavior

| Pros | Cons |
|------|------|
| No code change | Violates project no-panic rule |
| Panics are documented | Build scripts crash with backtraces |

**Best solution: A.** Defer validation to `generate()` where all other
errors are collected. Builder methods become infallible, matching
ergonomic expectations. Convert the internal `assert_eq!` to
`debug_assert_eq!`.

---

## 2. `emit_cargo_directives()` Calls `process::exit(1)` on Write Failure

`lib.rs:184-190`: when `std::fs::write` fails,
`emit_cargo_directives()` calls `std::process::exit(1)`. This
terminates immediately, skipping destructors and cleanup.

### Solutions

#### A. Return `Result<(), std::io::Error>` (recommended)

| Pros | Cons |
|------|------|
| Callers decide how to handle write failure | Breaking API change |
| No abrupt termination | Callers must add `?` or `.unwrap()` |
| Standard Rust error handling | |

#### B. Add `try_emit_cargo_directives()` alongside existing method

| Pros | Cons |
|------|------|
| Backward-compatible | API surface growth |
| Users can choose behavior | Two methods doing the same thing |

#### C. Keep `process::exit(1)` (status quo)

| Pros | Cons |
|------|------|
| No change | Skips destructors |
| Simple for users who don't need cleanup | Violates Rust idioms |

**Best solution: A.** Return `Result`. The existing `UnwrapOrExit`
pattern shows this crate supports both styles. Schedule for next
semver break.

---

## 3. Icon Names With Quotes/Backslashes Break Generated Code

`validate.rs:358` checks for control characters but not for `"` or
`\`. An icon name like `my"icon` generates broken Rust string literals.

### Solutions

#### A. Reject `"` and `\` in icon name validation (recommended)

| Pros | Cons |
|------|------|
| Prevents broken generated code | Slightly more restrictive |
| Catches the problem at validation time | Edge case -- unlikely in practice |

#### B. Escape special characters in code generation

| Pros | Cons |
|------|------|
| Accepts more icon names | Complex escaping logic |
| | Hard to test all combinations |

**Best solution: A.** These characters have no business in icon names.

---

## 4. Icon Names With Path Separators Enable Path Traversal

`validate_mapping_values()` only rejects empty strings and control
characters. An icon name like `"../../etc/passwd"` passes validation
and gets interpolated into `include_bytes!` paths, allowing embedding
arbitrary filesystem files into the compiled binary.

### Solutions

#### A. Reject `/`, `\`, and `..` in icon name validation (recommended)

| Pros | Cons |
|------|------|
| Prevents path traversal in generated code | Slightly more restrictive |
| Catches the problem at validation time | Unlikely to affect real users |
| Combines with issue 3 fix | |

#### B. Canonicalize path and verify it stays within theme directory

| Pros | Cons |
|------|------|
| Most robust -- handles symlinks | Requires file to exist at validation time |
| OS-aware | Doesn't work for system-theme names |

**Best solution: A.** Character-level rejection catches practical
attack vectors. Combine with issue 3 into a single validation function.

---

## 5. `icon_svg()` / `icon_name()` Mismatch for DE-Aware Bundled Mappings

`codegen.rs:310` generates `icon_svg()` match arms using only
`default_name()`. For a DE-aware mapping like
`{ kde = "media-playback-start", default = "play_pause" }` on a
bundled theme, only `play_pause.svg` is embedded. But `icon_name()`
returns `"media-playback-start"` on KDE. This creates a semantic
mismatch: the SVG data does not correspond to the declared icon name.

A warning already exists (Issue 7 in code) for this case. Related
issues: orphan SVG detection ignores DE-specific names (false orphan
warnings), and SVG path/rerun tracking also only uses default names.

### Solutions

#### A. Make bundled DE-aware mappings an error (recommended)

| Pros | Cons |
|------|------|
| Eliminates the mismatch entirely | Reduces flexibility |
| Clear constraint | Users must use system themes for DE-aware icons |
| DE-aware icons are inherently a system-theme concern | |

#### B. Embed all DE-specific SVGs for bundled themes

| Pros | Cons |
|------|------|
| icon_svg() and icon_name() always match | Larger binary |
| Correct semantics | Significantly more complex codegen |

#### C. Keep warning, enhance documentation

| Pros | Cons |
|------|------|
| No behavior change | Semantic mismatch persists |
| Build log explains constraint | |

**Best solution: A.** Bundled themes embed SVGs; DE-aware mappings
mean different DEs want different icons. These are fundamentally at
odds. System themes are the correct choice for DE-aware icons.

---

## 6. Cross-File Theme Overlap Not Validated After Merge

`validate_theme_overlap()` checks each config file in isolation. But
after `merge_configs()` combines configs from multiple files, the
merged result is not re-checked. If file A has
`bundled-themes = ["material"]` and file B has
`system-themes = ["material"]`, the merged config has `material` in
both lists with no error.

### Solutions

#### A. Re-run overlap validation on merged config (recommended)

| Pros | Cons |
|------|------|
| Catches cross-file overlap | One extra loop after merge |
| Clear error message | |
| Consistent with per-file validation | |

#### B. Validate during merge (fail fast)

| Pros | Cons |
|------|------|
| Fails at point of conflict | Mixes merge logic with validation |
| Earlier error | merge_configs currently doesn't return errors |

**Best solution: A.** Simple post-merge check.

---

## 7. `pipeline_result_to_output()` Has Hidden Side Effects

`lib.rs:877-888`: on the error path, `pipeline_result_to_output()`
calls `println!("cargo::rerun-if-changed=...")` for every tracked
path before returning `Err(BuildErrors)`. This is hidden I/O in a
function whose name suggests pure conversion.

### Solutions

#### A. Move rerun printing to the caller's error path (recommended)

Include rerun paths in the error type so callers can emit them.

| Pros | Cons |
|------|------|
| No hidden side effects | Changes BuildErrors struct or call sites |
| Callers control when directives are emitted | |
| Testable without stdout capture | |

#### B. Keep side effect, rename function

| Pros | Cons |
|------|------|
| Name matches behavior | Still can't suppress output |

**Best solution: A.** Move rerun-if-changed printing to where build-script
context is explicit.

---

## 8. `extra_derives` Interpolated Without Rust-Syntax Validation

`derive()` only checks for empty/whitespace. The value is interpolated
directly into `#[derive(...)]`. A value like
`"Ord)] struct Exploit; #[derive(Debug"` produces syntactically valid
but semantically wrong Rust.

### Solutions

#### A. Validate as Rust path (identifiers separated by `::`) (recommended)

| Pros | Cons |
|------|------|
| Prevents code injection | Slightly more validation code |
| Allows `serde::Serialize`, `Ord`, etc. | May reject exotic derive paths (unlikely) |

#### B. Use a typed `DeriveTrait` enum

| Pros | Cons |
|------|------|
| Type-safe for common derives | Must maintain the enum |

**Best solution: A.** Simple check for ident-joined-by-`::`.

---

## 9. All 4 Doctests Are `#[ignore]`

Every doctest in `src/lib.rs` is marked `ignore`, meaning they never
run and can silently drift from the actual API.

### Solutions

#### A. Change `ignore` to `no_run` (recommended)

| Pros | Cons |
|------|------|
| Catches compile-breaking API drift | Can't verify runtime behavior |
| No filesystem setup needed | |

#### B. Set up fixture directory for doctests

| Pros | Cons |
|------|------|
| Full verification | Complex fixture management |

**Best solution: A.** `no_run` gives compile-time verification at
zero setup cost.

---

## 10. Double Slash in `include_bytes!` Path When TOML at Manifest Root

When the TOML is at `CARGO_MANIFEST_DIR` root,
`strip_prefix(mdir)` produces an empty string. The generated path
becomes `"//material/play.svg"` -- technically malformed and
potentially wrong on Windows.

### Solutions

#### A. Skip leading `/` when `base_dir_str` is empty (recommended)

| Pros | Cons |
|------|------|
| Clean generated paths | Minor logic addition |
| Correct on all platforms | |

#### B. Keep double-slash (status quo)

| Pros | Cons |
|------|------|
| Works on Unix | Malformed paths on Windows |

**Best solution: A.** One conditional check.

---

## 11. Silent Name Normalization

`validate.rs:223-246` accepts names with spaces (e.g., `"app icon"`)
and silently normalizes to `AppIcon` via `to_upper_camel_case()`.
Also applies to `enum_name()` builder method (issue 23 subsumes).

### Solutions

#### A. Emit a warning for names that were normalized (recommended)

| Pros | Cons |
|------|------|
| User is informed | One more warning message |
| Doesn't reject valid names | |

#### B. Reject names with unusual characters

| Pros | Cons |
|------|------|
| No ambiguity | Too strict -- kebab-case is common in TOML |

**Best solution: A.** Warning is the right middle ground.

---

## 12. Missing Theme Directory Check

When a theme directory doesn't exist, the error comes from
`mapping.toml` read failing. No upfront check for the directory itself.

### Solutions

#### A. Add early directory existence check (recommended)

| Pros | Cons |
|------|------|
| Clear error naming the missing directory | One extra filesystem check |
| Better DX for new users | |

**Best solution: A.** Better error messages cost almost nothing.

---

## 13. Orphan SVG Detection Ignores DE-Specific Icon Names

`check_orphan_svgs()` builds the `referenced` set from
`default_name()` only. DE-specific names like
`"media-playback-start"` are not collected. Intentionally placed
DE-specific SVGs are falsely reported as orphans.

### Solutions

#### A. Include all icon names (Simple + all DeAware values) in referenced set (recommended)

| Pros | Cons |
|------|------|
| No false positive orphan warnings | Slightly larger set |
| Simple change | |

**Best solution: A.** Correct orphan detection.

---

## 14. SVG Path/Rerun Tracking Ignores DE-Specific Names

`check_orphan_svgs_and_collect_paths()` uses `default_name()` for
both `rerun_paths` and `svg_paths`. DE-specific SVGs are not tracked
for rebuild and not counted in the size report.

Same root cause as issue 13.

### Solutions

#### A. Track all icon names for rerun/size (recommended)

| Pros | Cons |
|------|------|
| All relevant SVGs tracked for rebuild | Slightly more paths |
| Accurate size reporting | |

**Best solution: A.** Fix alongside issue 13.

---

## 15. `unwrap_or("unknown")` Hides Non-UTF-8 Filenames

`validate.rs:153`: `check_orphan_svgs()` uses
`.unwrap_or("unknown")`. On Linux, non-UTF-8 filenames cause the
warning to say "unknown is not referenced" -- unidentifiable.

### Solutions

#### A. Use `to_string_lossy()` (recommended)

| Pros | Cons |
|------|------|
| Non-UTF-8 filenames shown with replacement char | Lossy |

#### B. Use the `stem` variable directly (already validated as UTF-8)

| Pros | Cons |
|------|------|
| Eliminates `unwrap_or` entirely | Warning loses `.svg` extension |

**Best solution: A.** `to_string_lossy()` is already used elsewhere
in the crate.

---

## 16. `check_orphan_svgs()` Silently Flattens `read_dir` Errors

`validate.rs:144`: `entries.flatten()` silently discards directory
entry errors (permission denied, broken symlinks).

### Solutions

#### A. Emit a warning per failed entry (recommended)

| Pros | Cons |
|------|------|
| User informed of filesystem issues | More verbose |

#### B. Keep `.flatten()` (status quo)

| Pros | Cons |
|------|------|
| Clean code | Silently drops errors |

**Best solution: A.** Orphan detection is advisory but silently
swallowing errors makes debugging harder.

---

## 17. Incomplete `RUST_KEYWORDS` List

`validate.rs:13-18` has 38 entries but is missing reserved-for-future
keywords: `abstract`, `become`, `box`, `do`, `final`, `macro`,
`override`, `priv`, `try`, `typeof`, `unsized`, `virtual`, `yield`.

**Practical impact is zero** -- `to_upper_camel_case()` always produces
PascalCase, and no Rust keyword starts with an uppercase letter except
`Self` (which is already listed). A role `"try"` becomes `"Try"`,
which is a valid identifier.

### Solutions

#### A. Add all reserved keywords (recommended)

| Pros | Cons |
|------|------|
| Comprehensive | Most additions are unreachable via PascalCase |
| Correct per Rust spec | |

#### B. Add a comment explaining why reserved words are excluded

| Pros | Cons |
|------|------|
| Documents reasoning | Still technically incomplete |

**Best solution: B.** Add a comment explaining that PascalCase
conversion means only `Self` is practically reachable from this list.
Adding the reserved words is harmless but misleadingly suggests they
matter.

---

## 18. Repeated `to_upper_camel_case()` Calls

`codegen.rs:99-112` calls `role.to_upper_camel_case()` twice per
role -- once for the enum definition, once for `const ALL`. Then
again in `generate_icon_name()` and `generate_icon_svg()`.

### Solutions

#### A. Cache the conversion in a Vec (recommended)

| Pros | Cons |
|------|------|
| Eliminates redundant allocations | One extra Vec |
| Clearer intent | |

**Best solution: A.** Not for performance but for code clarity.

---

## 19. `OnceLock` Caching Semantics Undocumented in Generated Code

The generated code uses `static CACHED_DE: OnceLock<...>` inside
`icon_name()`. This reads `XDG_CURRENT_DESKTOP` once and caches
forever. If the env var changes at runtime, the cached value is
permanently stale.

### Solutions

#### A. Add a doc comment in generated code (recommended)

| Pros | Cons |
|------|------|
| Users understand caching semantics | Comment in generated code |

**Best solution: A.** Most discoverable place.

---

## 20. `#[cfg(test)]` Import Misplaced at Module Scope

`validate.rs:1`: `BTreeMap` is imported with `#[cfg(test)]` at module
scope but is only used inside `#[cfg(test)] mod tests`.

### Solutions

#### A. Move the import inside the test module (recommended)

| Pros | Cons |
|------|------|
| Clearer intent | Minor diff |

**Best solution: A.** Trivial cleanup.

---

## 21. `let _ = writeln!(...)` Intent Unclear

`codegen.rs` uses `let _ = writeln!(out, ...)` approximately 50 times.
`String`'s `fmt::Write` is infallible, so the discard is correct but
the intent is not obvious.

### Solutions

#### A. Add an explanatory comment at the top of `generate_code()` (recommended)

| Pros | Cons |
|------|------|
| Intent clear to future maintainers | One comment |

**Best solution: A.** A single comment suffices.

---

## 22. Codegen Silently Skips Unknown Theme Names

`generate_icon_name()` and `generate_icon_svg()` both use `continue`
when `theme_name_to_qualified_icon_set()` returns `None`. If an unknown
theme somehow reaches codegen (should be caught by validation), the
generated enum silently lacks match arms.

### Solutions

#### A. Add `debug_assert!` documenting the invariant (recommended)

| Pros | Cons |
|------|------|
| Clarifies intent | No functional change in release |
| Catches validation bypass in dev | |

**Best solution: A.** Documents and enforces the invariant.

---

## Missing Test Coverage

### 23. No test for empty roles list behavior

When `roles = []`, the pipeline generates a warning but still produces
code. No test verifies the generated enum is valid (empty enum, empty
ALL, wildcard-only matches).

**Recommended:** Add a test.

### 24. No test for multiple DE overrides in a single mapping

Tests only use a single DE override (`kde`). No test checks behavior
with multiple overrides (e.g., `kde`, `gnome`, `xfce`).

**Recommended:** Add a test verifying all recognized DE variants
produce separate match arms.

### 25. No test for empty themes warning

`lib.rs:615-621`: when both theme lists are empty, a warning is
generated. This code path has no test.

**Recommended:** Add a test.

### 26. No test for DE-specific SVG non-requirement

`validate_svgs()` intentionally does NOT require DE-specific SVGs.
No test locks this behavior.

**Recommended:** Add a regression test.

### 27. `generate_icons()` simple API untested

The simple entry point `generate_icons()` is never directly tested.
All tests use the `IconGenerator` builder API.

**Recommended:** Add a test with env var setup.

### 28. `base_dir()` builder method untested

The `base_dir()` method and its divergent-base-dirs error path are
both untested.

**Recommended:** Add integration tests for both happy and error paths.

### 29. No test for Windows backslash path normalization

The `.replace('\\', "/")` normalization at `lib.rs:771-772` has no
test.

**Recommended:** Add a unit test with backslash paths.

### 30. No test fixtures for lucide or segoe-fluent themes

Integration tests only exercise `material` (bundled) and `sf-symbols`
(system). Three of five themes have no integration test coverage.

**Recommended:** Add fixture files for at least one more theme.

### 31. Non-unique temp directories in tests

Tests create temp directories with fixed names. Concurrent test runs
can collide.

#### A. Use `tempfile::tempdir()` (recommended)

| Pros | Cons |
|------|------|
| Random names, automatic cleanup | Adds `tempfile` dev-dependency |
| Standard practice | |

**Best solution: A.**

---

## Cross-Reference with platform-facts.md

### Icon set coverage: CORRECT

`THEME_TABLE` lists 5 sets: `sf-symbols`, `segoe-fluent`, `freedesktop`,
`material`, `lucide`. These match `IconSet` variants exactly.

platform-facts.md confirms: macOS uses SF Symbols (1.1.6), Windows
uses Segoe Fluent Icons (1.2.6), KDE/GNOME use freedesktop (1.3.6,
1.4.6). Material and Lucide are cross-platform. No mismatch.

### DE table coverage: CORRECT

`DE_TABLE` lists 7 DEs matching `LinuxDesktop` variants exactly. The
drift detection tests in `schema.rs` verify sync. These tests are
correctly `#[cfg(target_os = "linux")]` because `LinuxDesktop` and
`detect_linux_de()` are themselves `#[cfg(target_os = "linux")]` in
the main crate -- they do not exist on other platforms.

### COSMIC desktop: Not yet supported

The integration test correctly treats `cosmic` as unrecognized. When
`LinuxDesktop` adds `Cosmic`, the drift detection tests will catch
it automatically.

### No platform-specific value mismatches found

The build crate does not hardcode any platform-specific values (colors,
spacing, geometry). It only maps role names to icon names and generates
Rust code. All platform-specific behavior is delegated to the runtime
crate (`native-theme`). This is correct by design.

---

## Summary

| # | Issue | Severity | Best Fix | Effort |
|---|-------|----------|----------|--------|
| 1 | assert!/assert_eq! panics in production | High | Defer validation to generate() | Low |
| 2 | process::exit on write failure | High | Return Result (breaking) | Low |
| 3 | Quote/backslash in icon names | High | Reject in validation | Trivial |
| 4 | Path traversal in icon names | High | Reject `/`, `\`, `..` | Trivial |
| 5 | icon_svg/icon_name DE-aware mismatch | High | Make bundled DE-aware an error | Low |
| 6 | Cross-file theme overlap | Medium | Post-merge validation | Low |
| 7 | pipeline_result_to_output side effects | Medium | Move printing to caller | Low |
| 8 | extra_derives code injection | Medium | Validate as Rust path | Trivial |
| 9 | All doctests ignored | Medium | Change to `no_run` | Trivial |
| 10 | Double slash in include_bytes path | Medium | Skip separator when empty | Trivial |
| 11 | Silent name normalization | Low | Emit warning | Trivial |
| 12 | Missing directory check | Low | Early existence check | Trivial |
| 13 | Orphan SVG ignores DE names | Low | Include all names in set | Trivial |
| 14 | SVG tracking ignores DE names | Low | Track all DE names | Trivial |
| 15 | unwrap_or("unknown") hides filenames | Low | Use to_string_lossy() | Trivial |
| 16 | check_orphan_svgs flattens errors | Low | Emit warning per failure | Trivial |
| 17 | Incomplete RUST_KEYWORDS | Low | Add comment explaining PascalCase | Trivial |
| 18 | Repeated to_upper_camel_case calls | Low | Cache in Vec | Trivial |
| 19 | OnceLock caching undocumented | Low | Add comment in generated code | Trivial |
| 20 | cfg(test) import misplaced | Low | Move inside test module | Trivial |
| 21 | `let _ = writeln!` intent unclear | Low | Add explanatory comment | Trivial |
| 22 | Codegen silently skips unknown themes | Low | Add debug_assert | Trivial |
| 23-31 | Missing test coverage (9 items) | Low-Med | Add tests | Trivial-Low |
