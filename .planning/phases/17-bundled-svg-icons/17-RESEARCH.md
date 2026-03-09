# Phase 17: Bundled SVG Icons - Research

**Researched:** 2026-03-09
**Domain:** Compile-time SVG embedding, Cargo feature gates, Material Symbols + Lucide icon sets
**Confidence:** HIGH

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| BNDL-01 | Material Symbols SVGs (~42 icons covering all IconRole variants) as compile-time fallback (feature "material-icons") | SVG source verified (marella/material-symbols repo, Apache 2.0); individual SVGs average 350-550 bytes; 42 icons total ~18KB raw, well under 200KB budget; include_bytes! pattern documented |
| BNDL-02 | Lucide SVGs (~42 icons) as optional alternative icon set (feature "lucide-icons") | SVG source verified (lucide-icons/lucide repo, ISC license); individual SVGs average 300-500 bytes; 42 icons total ~15KB raw, well under 100KB budget; same include_bytes! pattern |
</phase_requirements>

## Summary

Phase 17 bundles two icon sets (Material Symbols and Lucide) as compile-time-embedded SVG data behind Cargo feature flags. Each set provides SVG bytes for all 42 `IconRole` variants. The implementation is straightforward: download 42 SVG files per set into the repository, use `include_bytes!` to embed them as `&'static [u8]` slices, and expose a public function `bundled_icon_svg(IconSet, IconRole) -> Option<&'static [u8]>` that returns `None` when the corresponding feature is disabled.

The size constraints are easily met. Material Symbols SVGs from the marella/material-symbols repository (outlined style, weight 400) average 350-550 bytes each, totaling approximately 18KB for 42 icons -- far below the 200KB budget. Lucide SVGs from the lucide-icons/lucide repository average 300-500 bytes each, totaling approximately 15KB -- far below the 100KB budget.

The key design decision is handling the `TrashFull` role, which has no dedicated icon in either Material or Lucide. The success criteria state "every IconRole variant resolves to valid SVG bytes," so both sets must provide a fallback for `TrashFull` (reusing the trash/delete icon). This means the bundled icon function must cover all 42 roles even where `icon_name()` returns `None` for that set.

**Primary recommendation:** Store SVG files in `native-theme/icons/material/` and `native-theme/icons/lucide/` directories. Use a dedicated `bundled.rs` module with `#[cfg(feature = "...")]` blocks containing `include_bytes!` for each icon. Expose `bundled_icon_svg(IconSet, IconRole) -> Option<&'static [u8]>` returning `Some` when the feature is enabled and `None` otherwise.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| (none new) | - | No new dependencies | `include_bytes!` is a std macro; feature gates are Cargo-native |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| (none needed) | - | - | Zero-dependency phase -- all functionality is Rust std + Cargo features |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Raw `include_bytes!` per file | `include_dir!` crate | Adds a proc-macro dependency for directory-level inclusion; unnecessary for 42 known files with stable names |
| Raw SVG bytes | `icondata`-style decomposed struct | Decomposing SVGs into fields (viewBox, paths, etc.) adds complexity; our `IconData::Svg(Vec<u8>)` expects raw SVG bytes, so decomposition would need re-serialization |
| Checking in SVG files | Build script downloading at build time | Network dependency at build time is fragile; 42 small files (~18KB total) are trivially small for git |
| Static `&[u8]` returns | Pre-constructing `IconData::Svg` at compile time | `IconData::Svg(Vec<u8>)` uses owned `Vec<u8>`, which cannot be const; returning `&'static [u8]` and letting callers `.to_vec()` is cleaner and avoids forced allocation |

**Installation:**
```bash
# No new crate dependencies needed
# SVG files must be downloaded and committed to the repository
```

## Architecture Patterns

### Recommended Project Structure
```
native-theme/
  icons/
    material/                  # 42 Material Symbols SVGs (Apache 2.0)
      warning.svg
      error.svg
      info.svg
      ...
    lucide/                    # 42 Lucide SVGs (ISC license)
      triangle-alert.svg
      circle-x.svg
      info.svg
      ...
    LICENSE-MATERIAL.txt       # Apache 2.0 license text
    LICENSE-LUCIDE.txt         # ISC license text
  src/
    model/
      icons.rs                 # Existing: IconRole, IconData, IconSet, icon_name()
      bundled.rs               # NEW: feature-gated include_bytes! + bundled_icon_svg()
      mod.rs                   # Updated: pub mod bundled; pub use bundled::*;
  Cargo.toml                   # Updated: add material-icons and lucide-icons features
```

### Pattern 1: Feature-Gated Module with include_bytes!
**What:** A module that conditionally compiles SVG data behind feature flags. Each icon is a `const` byte slice using `include_bytes!`. A single dispatch function maps `(IconSet, IconRole)` to the embedded data.
**When to use:** For all bundled icon embedding -- this is the core pattern for the entire phase.
**Example:**
```rust
// native-theme/src/model/bundled.rs

use super::icons::{IconRole, IconSet};

/// Returns the raw SVG bytes for a bundled icon, if the corresponding
/// feature flag is enabled.
///
/// Returns `None` when:
/// - The requested `IconSet` is not `Material` or `Lucide`
/// - The feature flag for the requested set is not enabled
///
/// Returns `Some(&[u8])` containing valid SVG bytes when the feature
/// is enabled. Callers typically convert to `IconData::Svg(bytes.to_vec())`.
pub fn bundled_icon_svg(set: IconSet, role: IconRole) -> Option<&'static [u8]> {
    match set {
        #[cfg(feature = "material-icons")]
        IconSet::Material => material_svg(role),

        #[cfg(feature = "lucide-icons")]
        IconSet::Lucide => lucide_svg(role),

        _ => None,
    }
}

#[cfg(feature = "material-icons")]
fn material_svg(role: IconRole) -> Option<&'static [u8]> {
    Some(match role {
        IconRole::DialogWarning => include_bytes!("../../icons/material/warning.svg"),
        IconRole::DialogError => include_bytes!("../../icons/material/error.svg"),
        // ... all 42 variants
        IconRole::TrashFull => include_bytes!("../../icons/material/delete.svg"), // reuse delete
        _ => return None, // #[non_exhaustive] forward compat
    })
}

#[cfg(feature = "lucide-icons")]
fn lucide_svg(role: IconRole) -> Option<&'static [u8]> {
    Some(match role {
        IconRole::DialogWarning => include_bytes!("../../icons/lucide/triangle-alert.svg"),
        IconRole::DialogError => include_bytes!("../../icons/lucide/circle-x.svg"),
        // ... all 42 variants
        IconRole::TrashFull => include_bytes!("../../icons/lucide/trash-2.svg"), // reuse trash-2
        _ => return None,
    })
}
```

### Pattern 2: Returning &'static [u8] (Not IconData)
**What:** The bundled function returns `Option<&'static [u8]>` (raw SVG bytes), not `Option<IconData>`. The caller wraps in `IconData::Svg(bytes.to_vec())` when needed.
**When to use:** Always -- returning a reference avoids allocation in the common case where the caller just needs to check if an icon exists or pass the bytes to a renderer that accepts `&[u8]`.
**Example:**
```rust
// Caller usage (Phase 21 load_icon dispatch):
if let Some(svg_bytes) = bundled_icon_svg(IconSet::Material, role) {
    return Some(IconData::Svg(svg_bytes.to_vec()));
}
```

### Pattern 3: Feature Flag Declaration
**What:** Cargo features declared in Cargo.toml with no dependencies -- they only control `#[cfg]` compilation.
**When to use:** For `material-icons` and `lucide-icons` features.
**Example:**
```toml
# native-theme/Cargo.toml
[features]
material-icons = []    # Bundle Material Symbols SVGs (~42 icons, ~18KB)
lucide-icons = []      # Bundle Lucide SVGs (~42 icons, ~15KB)
```

### Anti-Patterns to Avoid
- **Downloading SVGs at build time:** Do not use a build script that fetches SVGs from the network. This breaks offline builds and hermetic CI. Check the SVG files into the repository.
- **Using a proc macro for inclusion:** Do not write a custom proc macro to iterate a directory and generate `include_bytes!` calls. For 42 known, stable files, explicit `include_bytes!` per file is simpler, more transparent, and easier to audit.
- **Returning `IconData` directly from bundled function:** Do not return `IconData::Svg(Vec<u8>)` -- this forces an allocation every time the function is called, even if the caller just wants to check existence. Return `&'static [u8]` instead.
- **Single mega-file embedding:** Do not concatenate all SVGs into one blob with an index. Individual `include_bytes!` per file is cleaner, and the compiler handles them efficiently for this small number of files.
- **Forgetting the `#[allow(unreachable_patterns)]` on match:** The `#[non_exhaustive]` wildcard arm generates unreachable pattern warnings inside the defining crate. Use `#[allow(unreachable_patterns)]` as established in `icons.rs`.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| SVG optimization/minification | Custom SVG minifier | Pre-optimized SVGs from upstream repos | Both marella/material-symbols and lucide-icons/lucide already serve optimized SVGs |
| Icon file enumeration | Build script scanning directories | Explicit `include_bytes!` per file | 42 files is manageable; explicit is auditable and refactor-safe |
| Feature flag conditional logic | Runtime checks for feature availability | `#[cfg(feature = "...")]` compile-time | Zero-cost: disabled features contribute no bytes to the binary |

**Key insight:** This phase is almost entirely mechanical: download 84 SVG files (42 per set), write 84 `include_bytes!` lines (42 per feature), and one dispatch function. The complexity is in getting every role mapped correctly, not in any algorithmic challenge.

## Common Pitfalls

### Pitfall 1: TrashFull Has No Dedicated Icon in Either Set
**What goes wrong:** `material_name(IconRole::TrashFull)` and `lucide_name(IconRole::TrashFull)` both return `None`, but the success criteria require every IconRole variant to resolve to valid SVG bytes.
**Why it happens:** Neither Material Symbols nor Lucide has a separate "full trash" icon. Material uses "delete" for both trash concepts; Lucide uses "trash-2".
**How to avoid:** In the bundled match arms, map `TrashFull` to the same SVG file as `TrashEmpty` (material: `delete.svg`, lucide: `trash-2.svg`). Document this reuse in code comments.
**Warning signs:** A test iterating `IconRole::ALL` and calling `bundled_icon_svg()` returns `None` for `TrashFull`.

### Pitfall 2: Material SVG Naming Does Not Use Underscores in Filenames
**What goes wrong:** `material_name()` returns names like `content_copy`, `folder_open`, `check_circle` with underscores, but the actual SVG filenames in marella/material-symbols may use the same underscored names or different conventions.
**Why it happens:** The SVG filename convention in marella/material-symbols uses the exact Material symbol name (with underscores), e.g., `content_copy.svg`, `folder_open.svg`.
**How to avoid:** Verify each filename against the repository before writing `include_bytes!` paths. The `include_bytes!` macro will fail at compile time if a file doesn't exist, so this is caught early.
**Warning signs:** Compilation failure with "couldn't read ... file not found" from `include_bytes!`.

### Pitfall 3: Forgetting License Attribution
**What goes wrong:** Bundling third-party icons without proper license attribution violates the license terms.
**Why it happens:** Easy to focus on code and forget legal requirements.
**How to avoid:** Include `LICENSE-MATERIAL.txt` (Apache 2.0 from Google) and `LICENSE-LUCIDE.txt` (ISC from Lucide) in the `icons/` directory. Add attribution in the crate's LICENSE file or NOTICE file.
**Warning signs:** License checker tools flag missing attribution; crates.io publishing guidelines require license coverage for all bundled content.

### Pitfall 4: SVG viewBox Inconsistency Between Sets
**What goes wrong:** Material Symbols SVGs use `viewBox="0 -960 960 960"` (48x48 nominal) while Lucide SVGs use `viewBox="0 0 24 24"` (24x24 nominal). Renderers that assume a specific viewBox break.
**Why it happens:** Different icon sets have different design conventions.
**How to avoid:** This is not a problem for Phase 17 -- we return raw SVG bytes and let the renderer handle viewBox. Document the difference so Phase 21 (SVG rasterization) accounts for it.
**Warning signs:** Icons render at wrong sizes when mixing sets.

### Pitfall 5: Feature Flags Not Declared in Cargo.toml
**What goes wrong:** `#[cfg(feature = "material-icons")]` compiles away silently even without the feature being declared, leading to code that appears correct but is never compiled.
**Why it happens:** Rust does not warn about unknown `cfg` feature names by default (though `cargo check` with MSRV 1.80+ does via `check-cfg`).
**How to avoid:** Declare both features in `[features]` table of Cargo.toml. Run `cargo test --features material-icons` and `cargo test --features lucide-icons` to verify the feature-gated code compiles.
**Warning signs:** Tests pass without feature flags but bundled_icon_svg always returns None.

## Code Examples

Verified patterns from project source and upstream repositories:

### Material Symbols SVG Format (from marella/material-symbols, weight 400, outlined)
```xml
<!-- warning.svg - 512 bytes -->
<svg xmlns="http://www.w3.org/2000/svg" width="48" height="48"
     viewBox="0 -960 960 960">
  <path d="m40-120 440-760 440 760H40Zm104-60h672L480-760 144-180Zm361.5-65.68q..."/>
</svg>

<!-- search.svg - 361 bytes -->
<svg xmlns="http://www.w3.org/2000/svg" width="48" height="48"
     viewBox="0 -960 960 960">
  <path d="M796-121 533-384q-30 26-70 40.5T378-329q-108 0-183-75t-75-181..."/>
</svg>

<!-- content_copy.svg - 447 bytes -->
<svg xmlns="http://www.w3.org/2000/svg" width="48" height="48"
     viewBox="0 -960 960 960">
  <path d="M300-200q-24 0-42-18t-18-42v-560q0-24 18-42t42-18h440q24 0 42 18t18 42v560..."/>
</svg>
```

### Lucide SVG Format (from lucide-icons/lucide)
```xml
<!-- copy.svg - ~370 bytes -->
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24"
     viewBox="0 0 24 24" fill="none" stroke="currentColor"
     stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <rect width="14" height="14" x="8" y="8" rx="2" ry="2"/>
  <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>
</svg>

<!-- search.svg - ~320 bytes -->
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24"
     viewBox="0 0 24 24" fill="none" stroke="currentColor"
     stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <path d="m21 21-4.34-4.34"/>
  <circle cx="11" cy="11" r="8"/>
</svg>
```

### Complete Material Icon File Mapping (42 files)
```
icons/material/
  warning.svg              # DialogWarning
  error.svg                # DialogError, StatusError (same icon)
  info.svg                 # DialogInfo
  help.svg                 # DialogQuestion, Help (same icon)
  check_circle.svg         # DialogSuccess
  shield.svg               # Shield
  close.svg                # WindowClose
  minimize.svg             # WindowMinimize
  open_in_full.svg         # WindowMaximize
  close_fullscreen.svg     # WindowRestore
  save.svg                 # ActionSave
  delete.svg               # ActionDelete, TrashEmpty, TrashFull
  content_copy.svg         # ActionCopy
  content_paste.svg        # ActionPaste
  content_cut.svg          # ActionCut
  undo.svg                 # ActionUndo
  redo.svg                 # ActionRedo
  search.svg               # ActionSearch
  settings.svg             # ActionSettings
  edit.svg                 # ActionEdit
  add.svg                  # ActionAdd
  remove.svg               # ActionRemove
  refresh.svg              # ActionRefresh
  print.svg                # ActionPrint
  arrow_back.svg           # NavBack
  arrow_forward.svg        # NavForward
  arrow_upward.svg         # NavUp
  arrow_downward.svg       # NavDown
  home.svg                 # NavHome
  menu.svg                 # NavMenu
  description.svg          # FileGeneric
  folder.svg               # FolderClosed
  folder_open.svg          # FolderOpen
  progress_activity.svg    # StatusLoading
  check.svg                # StatusCheck
  person.svg               # UserAccount
  notifications.svg        # Notification
  lock.svg                 # Lock
```
Note: Some roles share the same SVG file (e.g., `error.svg` for both `DialogError` and `StatusError`). The `include_bytes!` macro de-duplicates at the linker level -- two includes of the same file produce one copy in the binary.

### Complete Lucide Icon File Mapping (42 files)
```
icons/lucide/
  triangle-alert.svg       # DialogWarning
  circle-x.svg             # DialogError, StatusError
  info.svg                 # DialogInfo
  circle-question-mark.svg # DialogQuestion, Help
  circle-check.svg         # DialogSuccess
  shield.svg               # Shield
  x.svg                    # WindowClose
  minimize.svg             # WindowMinimize
  maximize.svg             # WindowMaximize
  minimize-2.svg           # WindowRestore
  save.svg                 # ActionSave
  trash-2.svg              # ActionDelete, TrashEmpty, TrashFull
  copy.svg                 # ActionCopy
  clipboard-paste.svg      # ActionPaste
  scissors.svg             # ActionCut
  undo-2.svg               # ActionUndo
  redo-2.svg               # ActionRedo
  search.svg               # ActionSearch
  settings.svg             # ActionSettings
  pencil.svg               # ActionEdit
  plus.svg                 # ActionAdd
  minus.svg                # ActionRemove
  refresh-cw.svg           # ActionRefresh
  printer.svg              # ActionPrint
  chevron-left.svg         # NavBack
  chevron-right.svg        # NavForward
  chevron-up.svg           # NavUp
  chevron-down.svg         # NavDown
  house.svg                # NavHome
  menu.svg                 # NavMenu
  file.svg                 # FileGeneric
  folder-closed.svg        # FolderClosed
  folder-open.svg          # FolderOpen
  loader.svg               # StatusLoading
  check.svg                # StatusCheck
  user.svg                 # UserAccount
  bell.svg                 # Notification
  lock.svg                 # Lock
```

### Cargo.toml Feature Declaration
```toml
[features]
kde = ["dep:configparser"]
portal = ["dep:ashpd"]
portal-tokio = ["portal", "ashpd/tokio"]
portal-async-io = ["portal", "ashpd/async-io"]
windows = ["dep:windows"]
macos = ["dep:objc2", "dep:objc2-foundation", "dep:objc2-app-kit", "dep:block2"]
material-icons = []    # Bundle Material Symbols SVGs as cross-platform fallback
lucide-icons = []      # Bundle Lucide SVGs as optional icon set
```

### Test: All Roles Resolve with Feature Enabled
```rust
#[test]
#[cfg(feature = "material-icons")]
fn material_icons_cover_all_roles() {
    for role in IconRole::ALL {
        let svg = bundled_icon_svg(IconSet::Material, role);
        assert!(
            svg.is_some(),
            "Material icons missing SVG for {:?}",
            role
        );
        let bytes = svg.unwrap();
        // Verify it starts with an SVG tag
        let content = std::str::from_utf8(bytes).expect("SVG should be valid UTF-8");
        assert!(
            content.contains("<svg"),
            "Material {:?} does not contain <svg tag",
            role
        );
    }
}

#[test]
#[cfg(feature = "lucide-icons")]
fn lucide_icons_cover_all_roles() {
    for role in IconRole::ALL {
        let svg = bundled_icon_svg(IconSet::Lucide, role);
        assert!(
            svg.is_some(),
            "Lucide icons missing SVG for {:?}",
            role
        );
        let bytes = svg.unwrap();
        let content = std::str::from_utf8(bytes).expect("SVG should be valid UTF-8");
        assert!(
            content.contains("<svg"),
            "Lucide {:?} does not contain <svg tag",
            role
        );
    }
}
```

### Test: No Bloat Without Features
```rust
#[test]
fn no_bundled_icons_without_features() {
    // This test runs without any icon features enabled
    // Both should return None
    assert!(bundled_icon_svg(IconSet::Material, IconRole::ActionCopy).is_none());
    assert!(bundled_icon_svg(IconSet::Lucide, IconRole::ActionCopy).is_none());
}
```

### Test: Binary Size Budget
```rust
#[test]
#[cfg(feature = "material-icons")]
fn material_icons_total_size_under_200kb() {
    let total: usize = IconRole::ALL
        .iter()
        .filter_map(|role| bundled_icon_svg(IconSet::Material, *role))
        .map(|svg| svg.len())
        .sum();
    assert!(
        total < 200 * 1024,
        "Material icons total size {} bytes exceeds 200KB budget",
        total
    );
}

#[test]
#[cfg(feature = "lucide-icons")]
fn lucide_icons_total_size_under_100kb() {
    let total: usize = IconRole::ALL
        .iter()
        .filter_map(|role| bundled_icon_svg(IconSet::Lucide, *role))
        .map(|svg| svg.len())
        .sum();
    assert!(
        total < 100 * 1024,
        "Lucide icons total size {} bytes exceeds 100KB budget",
        total
    );
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Download icons at build time | Check SVG files into repository | Current Rust ecosystem norm | Hermetic builds, no network at compile time |
| `include_dir!` for directory embedding | Individual `include_bytes!` for known files | Both valid; individual is simpler for < 100 files | No proc-macro dependency |
| Decomposed SVG structs (icondata pattern) | Raw SVG bytes | Both valid; raw bytes match `IconData::Svg` | Zero conversion overhead |

**Deprecated/outdated:**
- google/material-design-icons repo for SVG access: The official Google repo is hard to navigate for individual SVGs. marella/material-symbols provides cleaner, pre-optimized SVGs with consistent naming.

## Open Questions

1. **SVG style choice for Material Symbols**
   - What we know: Material Symbols has 3 styles (outlined, rounded, sharp) and 7 weights (100-700). The SVG repo at marella/material-symbols organizes by weight then style.
   - What's unclear: Which style to default to.
   - Recommendation: Use **outlined, weight 400** (the default in Google Fonts UI). This is the most recognizable and widely used style. The SVGs are already optimized and small.

2. **Deduplication of shared icons**
   - What we know: Some roles map to the same SVG file (e.g., `DialogError` and `StatusError` both use `error.svg` in Material). Two `include_bytes!` of the same file path produce identical data.
   - What's unclear: Whether the Rust compiler/linker deduplicates identical static byte arrays.
   - Recommendation: Do not manually deduplicate. Use separate `include_bytes!` calls even for the same file. The linker's `--icf=safe` (identical code folding) handles this on most platforms, and even without it, the overhead is negligible (~500 bytes per duplicate for a total of ~2-3KB max).

3. **Whether `bundled_icon_svg` should live in `model::bundled` or at crate root**
   - What we know: The icon types live in `model::icons`. The bundled data is closely related.
   - What's unclear: Whether the module hierarchy matters for downstream phases.
   - Recommendation: Place in `model::bundled` and re-export from `model::mod.rs` and `lib.rs`. This follows the existing pattern where model types are defined in submodules and re-exported.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in test (cargo test) |
| Config file | Cargo.toml (workspace) |
| Quick run command | `cargo test -p native-theme --lib --features material-icons,lucide-icons` |
| Full suite command | `cargo test -p native-theme --features material-icons,lucide-icons` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| BNDL-01 | Material icons cover all 42 roles | unit | `cargo test -p native-theme --lib --features material-icons material_icons_cover_all_roles` | Wave 0 |
| BNDL-01 | Material total size under 200KB | unit | `cargo test -p native-theme --lib --features material-icons material_icons_total_size` | Wave 0 |
| BNDL-02 | Lucide icons cover all 42 roles | unit | `cargo test -p native-theme --lib --features lucide-icons lucide_icons_cover_all_roles` | Wave 0 |
| BNDL-02 | Lucide total size under 100KB | unit | `cargo test -p native-theme --lib --features lucide-icons lucide_icons_total_size` | Wave 0 |
| BNDL-01+02 | No bloat without features | unit | `cargo test -p native-theme --lib no_bundled_icons_without_features` | Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test -p native-theme --lib --features material-icons,lucide-icons`
- **Per wave merge:** `cargo test -p native-theme --features material-icons,lucide-icons`
- **Phase gate:** Full suite green with features on AND off

### Wave 0 Gaps
- [ ] `native-theme/icons/material/` -- 42 SVG files to download from marella/material-symbols
- [ ] `native-theme/icons/lucide/` -- 42 SVG files to download from lucide-icons/lucide
- [ ] `native-theme/icons/LICENSE-MATERIAL.txt` -- Apache 2.0 license for Material Symbols
- [ ] `native-theme/icons/LICENSE-LUCIDE.txt` -- ISC license for Lucide
- [ ] `native-theme/src/model/bundled.rs` -- new module with feature-gated include_bytes!
- [ ] Cargo.toml `[features]` -- add `material-icons = []` and `lucide-icons = []`

## SVG Source URLs

### Material Symbols (outlined, weight 400)
Base URL: `https://raw.githubusercontent.com/marella/material-symbols/main/svg/400/outlined/`

| IconRole | Filename | URL suffix |
|----------|----------|------------|
| DialogWarning | `warning.svg` | `warning.svg` |
| DialogError | `error.svg` | `error.svg` |
| DialogInfo | `info.svg` | `info.svg` |
| DialogQuestion | `help.svg` | `help.svg` |
| DialogSuccess | `check_circle.svg` | `check_circle.svg` |
| Shield | `shield.svg` | `shield.svg` |
| WindowClose | `close.svg` | `close.svg` |
| WindowMinimize | `minimize.svg` | `minimize.svg` |
| WindowMaximize | `open_in_full.svg` | `open_in_full.svg` |
| WindowRestore | `close_fullscreen.svg` | `close_fullscreen.svg` |
| ActionSave | `save.svg` | `save.svg` |
| ActionDelete | `delete.svg` | `delete.svg` |
| ActionCopy | `content_copy.svg` | `content_copy.svg` |
| ActionPaste | `content_paste.svg` | `content_paste.svg` |
| ActionCut | `content_cut.svg` | `content_cut.svg` |
| ActionUndo | `undo.svg` | `undo.svg` |
| ActionRedo | `redo.svg` | `redo.svg` |
| ActionSearch | `search.svg` | `search.svg` |
| ActionSettings | `settings.svg` | `settings.svg` |
| ActionEdit | `edit.svg` | `edit.svg` |
| ActionAdd | `add.svg` | `add.svg` |
| ActionRemove | `remove.svg` | `remove.svg` |
| ActionRefresh | `refresh.svg` | `refresh.svg` |
| ActionPrint | `print.svg` | `print.svg` |
| NavBack | `arrow_back.svg` | `arrow_back.svg` |
| NavForward | `arrow_forward.svg` | `arrow_forward.svg` |
| NavUp | `arrow_upward.svg` | `arrow_upward.svg` |
| NavDown | `arrow_downward.svg` | `arrow_downward.svg` |
| NavHome | `home.svg` | `home.svg` |
| NavMenu | `menu.svg` | `menu.svg` |
| FileGeneric | `description.svg` | `description.svg` |
| FolderClosed | `folder.svg` | `folder.svg` |
| FolderOpen | `folder_open.svg` | `folder_open.svg` |
| TrashEmpty | `delete.svg` | `delete.svg` (shared with ActionDelete) |
| TrashFull | `delete.svg` | `delete.svg` (no separate full-trash icon) |
| StatusLoading | `progress_activity.svg` | `progress_activity.svg` |
| StatusCheck | `check.svg` | `check.svg` |
| StatusError | `error.svg` | `error.svg` (shared with DialogError) |
| UserAccount | `person.svg` | `person.svg` |
| Notification | `notifications.svg` | `notifications.svg` |
| Help | `help.svg` | `help.svg` (shared with DialogQuestion) |
| Lock | `lock.svg` | `lock.svg` |

Unique files needed: ~32 (some roles share files)

### Lucide
Base URL: `https://raw.githubusercontent.com/lucide-icons/lucide/main/icons/`

| IconRole | Filename | URL suffix |
|----------|----------|------------|
| DialogWarning | `triangle-alert.svg` | `triangle-alert.svg` |
| DialogError | `circle-x.svg` | `circle-x.svg` |
| DialogInfo | `info.svg` | `info.svg` |
| DialogQuestion | `circle-question-mark.svg` | `circle-question-mark.svg` |
| DialogSuccess | `circle-check.svg` | `circle-check.svg` |
| Shield | `shield.svg` | `shield.svg` |
| WindowClose | `x.svg` | `x.svg` |
| WindowMinimize | `minimize.svg` | `minimize.svg` |
| WindowMaximize | `maximize.svg` | `maximize.svg` |
| WindowRestore | `minimize-2.svg` | `minimize-2.svg` |
| ActionSave | `save.svg` | `save.svg` |
| ActionDelete | `trash-2.svg` | `trash-2.svg` |
| ActionCopy | `copy.svg` | `copy.svg` |
| ActionPaste | `clipboard-paste.svg` | `clipboard-paste.svg` |
| ActionCut | `scissors.svg` | `scissors.svg` |
| ActionUndo | `undo-2.svg` | `undo-2.svg` |
| ActionRedo | `redo-2.svg` | `redo-2.svg` |
| ActionSearch | `search.svg` | `search.svg` |
| ActionSettings | `settings.svg` | `settings.svg` |
| ActionEdit | `pencil.svg` | `pencil.svg` |
| ActionAdd | `plus.svg` | `plus.svg` |
| ActionRemove | `minus.svg` | `minus.svg` |
| ActionRefresh | `refresh-cw.svg` | `refresh-cw.svg` |
| ActionPrint | `printer.svg` | `printer.svg` |
| NavBack | `chevron-left.svg` | `chevron-left.svg` |
| NavForward | `chevron-right.svg` | `chevron-right.svg` |
| NavUp | `chevron-up.svg` | `chevron-up.svg` |
| NavDown | `chevron-down.svg` | `chevron-down.svg` |
| NavHome | `house.svg` | `house.svg` |
| NavMenu | `menu.svg` | `menu.svg` |
| FileGeneric | `file.svg` | `file.svg` |
| FolderClosed | `folder-closed.svg` | `folder-closed.svg` |
| FolderOpen | `folder-open.svg` | `folder-open.svg` |
| TrashEmpty | `trash-2.svg` | `trash-2.svg` (shared with ActionDelete) |
| TrashFull | `trash-2.svg` | `trash-2.svg` (no separate full-trash icon) |
| StatusLoading | `loader.svg` | `loader.svg` |
| StatusCheck | `check.svg` | `check.svg` |
| StatusError | `circle-x.svg` | `circle-x.svg` (shared with DialogError) |
| UserAccount | `user.svg` | `user.svg` |
| Notification | `bell.svg` | `bell.svg` |
| Help | `circle-question-mark.svg` | `circle-question-mark.svg` (shared with DialogQuestion) |
| Lock | `lock.svg` | `lock.svg` |

Unique files needed: ~33 (some roles share files)

## Sources

### Primary (HIGH confidence)
- marella/material-symbols GitHub repo -- SVG files verified by fetching individual icons (warning.svg: 512 bytes, search.svg: 361 bytes, content_copy.svg: 447 bytes, folder_open.svg: 556 bytes, progress_activity.svg: 474 bytes, close_fullscreen.svg: 282 bytes, open_in_full.svg: 284 bytes)
- lucide-icons/lucide GitHub repo -- SVG files verified by fetching individual icons (copy.svg: ~370 bytes, search.svg: ~320 bytes, circle-question-mark.svg: ~400 bytes, folder-closed.svg: ~494 bytes)
- Rust std `include_bytes!` documentation -- confirms `&'static [u8; N]` return type
- Rust Reference conditional compilation -- confirms `#[cfg(feature = "...")]` pattern
- Existing `icons.rs` in project -- 42 IconRole variants and icon_name() mappings verified
- Material Symbols license: Apache 2.0 (verified from marella/material-symbols LICENSE)
- Lucide license: ISC + MIT (verified from lucide-icons/lucide LICENSE)

### Secondary (MEDIUM confidence)
- nickb.dev blog on embedding data tradeoffs -- confirms include_bytes! is efficient for small files
- docs/native-icons.md -- authoritative project spec for icon identifier mappings

### Tertiary (LOW confidence)
- None -- all findings verified from primary or secondary sources.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- no new dependencies, just std macros and Cargo features
- Architecture: HIGH -- follows existing crate patterns (cfg feature gates, model submodules, re-exports)
- Pitfalls: HIGH -- identified from direct analysis of icon_name() None cases, SVG format differences, license requirements
- SVG availability: HIGH -- verified by fetching actual SVG files from upstream repos
- Size estimates: HIGH -- based on measured individual SVG sizes (300-600 bytes each)

**Research date:** 2026-03-09
**Valid until:** 2026-04-08 (stable domain; icon repos update frequently but file format/naming is stable)
