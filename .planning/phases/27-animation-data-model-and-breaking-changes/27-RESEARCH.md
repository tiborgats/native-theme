# Phase 27: Animation Data Model and Breaking Changes - Research

**Researched:** 2026-03-18
**Domain:** Rust enum type design, breaking change management, API consistency in native-theme crate
**Confidence:** HIGH

## Summary

Phase 27 introduces the type foundation for animated icons in native-theme v0.4.0. The work has three pillars: (1) define the AnimatedIcon enum with Frames and Transform variants, plus supporting types TransformAnimation and Repeat, (2) add a `loading_indicator()` dispatch function that mirrors the existing `load_icon()` pattern, and (3) remove `StatusLoading` from `IconRole`, replacing it with `StatusBusy`.

The codebase is well-structured for this change. `IconRole` is `#[non_exhaustive]` so adding `StatusBusy` is non-breaking, but removing `StatusLoading` IS breaking (match arms referencing it will fail). All StatusLoading references have been mapped -- they exist in `icons.rs` (enum definition, ALL array, 5 icon_name mapping functions, ~12 test assertions), `bundled.rs` (2 match arms), `winicons.rs` (1 test), the gpui connector (`icons.rs` and `showcase.rs` example), and several docs files. The `loading_indicator()` function should return `None` for all icon sets in this phase -- Phase 28 wires up actual frame data.

**Primary recommendation:** Start with the new types (AnimatedIcon, TransformAnimation, Repeat), then add loading_indicator() and first_frame(), then do the StatusLoading-to-StatusBusy rename last (since the rename touches the most files and tests).

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- Remove StatusLoading in the SAME release (v0.4.0) that adds AnimatedIcon and loading_indicator()
- Replace with StatusBusy -- a static icon role for apps that want a non-animated busy/progress indicator
- StatusBusy maps to the same underlying icons (process-working, progress_activity, loader) but is semantically "static busy indicator" not "animated loading"
- The rename is a breaking change but pre-1.0, so acceptable on minor version bump
- loading_indicator() should be a single dispatch function taking &str for icon set name, consistent with load_icon() pattern
- Returns None when the requested icon set's feature flag is disabled (matches load_icon() behavior)

### Claude's Discretion
- AnimatedIcon enum shape: exact field names, derive traits (Debug, Clone, PartialEq?), pub vs pub(crate) boundaries
- Icon set naming for loading_indicator() (reuse load_icon() names vs platform names)
- Whether to include a from_system()-style auto-detect for loading_indicator()
- CHANGELOG/migration framing for StatusLoading -> StatusBusy + loading_indicator()

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| ANIM-01 | AnimatedIcon enum with Frames and Transform variants, both #[non_exhaustive] | Existing IconData pattern provides the model: `#[derive(Debug, Clone, PartialEq, Eq)]`, `#[non_exhaustive]` on enum. New types go in `model/icons.rs` or a new `model/animated.rs` |
| ANIM-02 | Frames variant holds Vec<IconData> frames, frame_duration_ms (u32), and Repeat enum | Repeat enum should be `#[non_exhaustive]` with `Infinite` variant per design doc |
| ANIM-03 | Transform variant holds IconData and TransformAnimation enum | TransformAnimation holds animation metadata for simple transforms applied to a single icon |
| ANIM-04 | TransformAnimation::Spin variant with duration_ms field | Single variant for now; `#[non_exhaustive]` allows future SpinEased etc. |
| ANIM-05 | loading_indicator(icon_set: &str) dispatches to platform/bundled loaders and returns Option<AnimatedIcon> | Mirror load_icon() pattern: parse icon_set via IconSet::from_name(), dispatch per set, return None for all sets this phase |
| ANIM-06 | AnimatedIcon provides first_frame() -> Option<&IconData> helper for static fallback | Method on AnimatedIcon; returns frames[0] for Frames, &icon for Transform; Option because frames Vec could be empty |
| BREAK-01 | Remove StatusLoading variant from IconRole enum | Direct removal from enum, ALL array (count drops 42->42 with StatusBusy replacing StatusLoading), icon_name mapping functions |
| BREAK-02 | Update all internal references, match arms, icon_name() mappings, and bundled icon lookups | 30+ reference sites catalogued in this research (see Architecture Patterns section) |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| native-theme | 0.4.0 (workspace) | Core crate where all changes land | This IS the project |
| Rust | 1.94.0+ (MSRV) | Rust edition 2024 | Workspace rust-version |

### Supporting
No new dependencies are needed for Phase 27. All work is pure type definitions and API additions.

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Separate `model/animated.rs` module | Inline in `model/icons.rs` | Separate module is cleaner for a distinct concept; icons.rs is already 1500+ lines |
| `PartialEq` on AnimatedIcon | Skip PartialEq | Vec<IconData> comparison can be expensive for large frame sets, but is consistent with IconData which derives PartialEq. Include it for API completeness. |

## Architecture Patterns

### Recommended Project Structure
```
native-theme/src/
├── model/
│   ├── icons.rs       # IconRole (StatusBusy replaces StatusLoading), IconData, IconSet
│   ├── animated.rs    # NEW: AnimatedIcon, TransformAnimation, Repeat
│   ├── bundled.rs     # StatusBusy replaces StatusLoading in match arms
│   └── mod.rs         # Re-exports new types
├── lib.rs             # loading_indicator() function, re-exports AnimatedIcon types
├── freedesktop.rs     # No changes this phase (icon_name mapping is in icons.rs)
├── sficons.rs         # No changes this phase
└── winicons.rs        # Test reference update (StatusLoading -> StatusBusy)
```

### Pattern 1: New Type Definitions (AnimatedIcon, TransformAnimation, Repeat)
**What:** Three new public enums following existing IconData conventions
**When to use:** This phase
**Recommendation:**

```rust
// Source: Design doc (docs/animated-icons.md) Combo 3, adapted to codebase conventions

/// Animation repeat behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Repeat {
    /// Loop the animation indefinitely.
    Infinite,
}

/// Transform-based animation applied to a single static icon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TransformAnimation {
    /// Continuous rotation (e.g., GNOME GtkSpinner, Lucide loader).
    Spin {
        /// Duration of one full rotation in milliseconds.
        duration_ms: u32,
    },
}

/// Animated icon data with platform-adaptive animation strategy.
///
/// Two variants support the range of native platform spinner animations:
/// - [`Frames`](AnimatedIcon::Frames): Pre-rendered frame sequences for complex
///   animations (macOS spokes, Windows arc, KDE sprite sheet).
/// - [`Transform`](AnimatedIcon::Transform): Simple geometric transforms on a
///   static icon (GNOME CSS rotation, Lucide loader spin).
///
/// Use [`first_frame()`](AnimatedIcon::first_frame) to extract a static fallback
/// for reduced-motion contexts or simple display.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AnimatedIcon {
    /// Frame sequence animation.
    Frames {
        /// Individual frames, each a complete icon.
        frames: Vec<IconData>,
        /// Duration of each frame in milliseconds.
        frame_duration_ms: u32,
        /// Repeat behavior for the animation.
        repeat: Repeat,
    },
    /// Transform animation applied to a single static icon.
    Transform {
        /// The static icon to animate.
        icon: IconData,
        /// The transform animation to apply.
        animation: TransformAnimation,
    },
}
```

**Derive trait rationale:**
- `Debug, Clone` -- mandatory for public API types, consistent with IconData
- `PartialEq, Eq` -- consistent with IconData which derives both; allows testing assertions
- `Hash` -- on Repeat and TransformAnimation (small Copy types) but NOT on AnimatedIcon (Vec<IconData> is not Hash)
- `#[non_exhaustive]` -- on all three enums for future extensibility
- No `Serialize/Deserialize` -- AnimatedIcon is runtime data, not persisted to TOML
- No `must_use` on AnimatedIcon -- it is returned from loading_indicator() which itself should be `#[must_use]`

### Pattern 2: first_frame() Implementation
**What:** Method on AnimatedIcon returning the first frame as static fallback
**Recommendation:**

```rust
impl AnimatedIcon {
    /// Returns a reference to the first frame of the animation for static display.
    ///
    /// Returns `None` if the animation has no frames (empty `Frames` variant).
    /// For `Transform`, returns the underlying static icon.
    ///
    /// Use this as a reduced-motion fallback or loading placeholder.
    #[must_use]
    pub fn first_frame(&self) -> Option<&IconData> {
        match self {
            AnimatedIcon::Frames { frames, .. } => frames.first(),
            AnimatedIcon::Transform { icon, .. } => Some(icon),
        }
    }
}
```

### Pattern 3: loading_indicator() Dispatch Function
**What:** Public function mirroring load_icon() pattern
**When to use:** This phase (returns None for all sets; Phase 28 wires up actual data)
**Recommendation:**

```rust
/// Load the platform-native loading indicator animation for the given icon set.
///
/// Dispatches to platform or bundled animation loaders based on `icon_set`.
/// Returns `None` when:
/// - The icon set name is not recognized (falls back to system icon set, then
///   returns None if that set has no loading indicator)
/// - The required feature flag for the icon set is not enabled
/// - The icon set does not yet have animation data implemented
///
/// # Examples
///
/// ```
/// use native_theme::loading_indicator;
///
/// // Returns None until animation data is implemented in a future release
/// let anim = loading_indicator("material");
/// assert!(anim.is_none());
/// ```
#[must_use = "this returns animation data; it does not display anything"]
#[allow(unused_variables)]
pub fn loading_indicator(icon_set: &str) -> Option<AnimatedIcon> {
    let set = IconSet::from_name(icon_set).unwrap_or_else(system_icon_set);

    match set {
        #[cfg(all(target_os = "linux", feature = "system-icons"))]
        IconSet::Freedesktop => None, // Phase 29: freedesktop sprite sheet parsing

        #[cfg(all(target_os = "macos", feature = "system-icons"))]
        IconSet::SfSymbols => None, // Phase 28: bundled macOS-style frames

        #[cfg(all(target_os = "windows", feature = "system-icons"))]
        IconSet::SegoeIcons => None, // Phase 28: bundled Windows-style frames

        #[cfg(feature = "material-icons")]
        IconSet::Material => None, // Phase 28: bundled Material spinner frames

        #[cfg(feature = "lucide-icons")]
        IconSet::Lucide => None, // Phase 28: Transform::Spin on loader icon

        _ => None,
    }
}
```

**Key design decision: reuse load_icon() icon set naming.** The `icon_set` parameter accepts the same strings as `load_icon()`: `"material"`, `"lucide"`, `"freedesktop"`, `"sf-symbols"`, `"segoe-fluent"`. Unknown names fall back to `system_icon_set()`. This is consistent and unsurprising.

**Auto-detect question:** Do NOT add a separate `loading_indicator_from_system()` function. The fallback behavior via `system_icon_set()` when an unknown name is passed already handles the auto-detect case. Callers who want auto-detection can pass any non-matching string or use `system_icon_set().name()`.

### Pattern 4: StatusLoading -> StatusBusy Rename
**What:** Remove StatusLoading, add StatusBusy, update all references
**Complete reference map:**

**`native-theme/src/model/icons.rs`** (highest impact):
- Line 125: `StatusLoading` variant definition -> rename to `StatusBusy`
- Line 124: Doc comment -> update from "Loading / in-progress" to "Busy / working state"
- Line 188: `Self::StatusLoading` in ALL array -> `Self::StatusBusy`
- Line 682: SF Symbols `StatusLoading => return None` -> `StatusBusy` with a proper mapping (could still return None, or map to a static indicator)
- Line 746: Segoe `StatusLoading => return None` -> `StatusBusy` with a proper mapping (could still return None, or map to a static indicator)
- Line 809: Freedesktop `StatusLoading => "process-working"` -> `StatusBusy => "process-working"`
- Line 874: Material `StatusLoading => "progress_activity"` -> `StatusBusy => "progress_activity"`
- Line 938: Lucide `StatusLoading => "loader"` -> `StatusBusy => "loader"`
- Tests (lines 1014, 1301, 1323, 1401, 1411, 1538-1539): Update all assertions

**StatusBusy icon_name decisions:**
- SF Symbols: Could now return `Some("hourglass")` or `Some("ellipsis")` for a static busy indicator, OR remain `None`. Recommendation: `None` for now, consistent with the fact that SF Symbols has no static process-working equivalent. The gap stays documented.
- Segoe: Could return `Some("Progress")` or remain `None`. Recommendation: `None` -- same reasoning.
- Freedesktop: `"process-working"` (same as before -- the freedesktop loader's symbolic-first strategy avoids the sprite sheet)
- Material: `"progress_activity"` (same as before)
- Lucide: `"loader"` (same as before)

**`native-theme/src/model/bundled.rs`**:
- Line 92: Material `StatusLoading =>` -> `StatusBusy =>`
- Line 156: Lucide `StatusLoading =>` -> `StatusBusy =>`

**`native-theme/src/winicons.rs`**:
- Lines 510-514: Test referencing StatusLoading -> StatusBusy

**`connectors/native-theme-gpui/src/icons.rs`**:
- Line 76: `IconRole::StatusLoading => IconName::Loader` -> `IconRole::StatusBusy => IconName::Loader`

**`connectors/native-theme-gpui/examples/showcase.rs`**:
- Line 347: `"Loader" => Some(IconRole::StatusLoading)` -> `Some(IconRole::StatusBusy)`

**Docs (NOT in scope for this phase but should be noted):**
- `docs/animated-icons.md` -- multiple references (docs update is Phase 32)
- `docs/extra-icons.md` -- one reference
- `docs/icon-gaps-and-fallback-removal.md` -- multiple references

**ALL array count:** Stays at 42. One variant removed (StatusLoading), one added (StatusBusy).

### Anti-Patterns to Avoid
- **Adding AnimatedIcon variants to IconData:** The design doc explicitly chose a separate type. Do NOT add `AnimatedIcon` as a variant inside `IconData` -- they are different type categories (static vs animated).
- **Deprecating StatusLoading instead of removing:** The user locked the decision to remove, not deprecate. Pre-1.0 crate, breaking changes are expected.
- **Making loading_indicator() return a compile error for disabled features:** User explicitly reconsidered and chose the None-returning pattern consistent with load_icon().
- **Mixing icon sets in loading_indicator():** Must follow the no-cross-set-fallback rule from the MEMORY.md critical rules.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Icon set resolution from string | Custom string parsing | `IconSet::from_name()` + `system_icon_set()` | Already exists, battle-tested, used by load_icon() |
| Feature-gated dispatch | Inline cfg!() blocks | Match-arm pattern with #[cfg()] per arm | Consistent with load_icon(), load_system_icon_by_name() |

**Key insight:** This phase is purely additive type definitions + a rename. No complex logic, no new dependencies, no platform-specific code. The main risk is missing a StatusLoading reference site.

## Common Pitfalls

### Pitfall 1: Missing a StatusLoading Reference
**What goes wrong:** Compilation fails because a match arm still references `StatusLoading` after removal.
**Why it happens:** StatusLoading appears in 30+ locations across the codebase, including tests, connectors, and examples.
**How to avoid:** Use the complete reference map in this research. After rename, run `cargo check -p native-theme --features material-icons,lucide-icons` to catch core crate issues, and `cargo check --workspace` (if connectors compile) for full coverage.
**Warning signs:** Compiler errors mentioning `StatusLoading` pattern not covered or variant not found.

### Pitfall 2: Breaking the ALL Array Count
**What goes wrong:** `IconRole::ALL.len()` assertion in tests fails, or all_roles_material / all_roles_lucide tests fail.
**Why it happens:** ALL array count is hardcoded as `[IconRole; 42]`. Adding StatusBusy and removing StatusLoading keeps it at 42, but if one is done without the other, the count changes.
**How to avoid:** Do both the add and remove in the same commit/task. Update the ALL array atomically.
**Warning signs:** `expected array of 42 elements, found 41` or `43`.

### Pitfall 3: Forgetting to Update known_gaps() Test
**What goes wrong:** The `known_gaps()` test function (icons.rs ~line 1535) lists `(IconSet::SfSymbols, IconRole::StatusLoading)` and `(IconSet::SegoeIcons, IconRole::StatusLoading)`. These must update to StatusBusy.
**Why it happens:** Test helper function is easy to miss during rename.
**How to avoid:** Grep for `StatusLoading` after rename -- zero results means success.
**Warning signs:** Test compilation errors in known_gaps function.

### Pitfall 4: Connector Compilation After Rename
**What goes wrong:** The gpui connector references `IconRole::StatusLoading` in two files. If only the core crate is updated, connector won't compile.
**Why it happens:** Workspace has 4 crates; easy to forget downstream consumers.
**How to avoid:** Include connector updates in the same phase. The iced connector does NOT reference StatusLoading directly (it goes through load_icon/load_custom_icon), so only the gpui connector needs updating.
**Warning signs:** `cargo check -p native-theme-gpui` fails.

### Pitfall 5: Module Re-export Chain
**What goes wrong:** New types (AnimatedIcon, TransformAnimation, Repeat) are defined but not exported from `model/mod.rs` and `lib.rs`.
**Why it happens:** Rust module visibility requires explicit re-exports at each level.
**How to avoid:** Follow the existing pattern: define in `model/animated.rs`, re-export from `model/mod.rs`, re-export from `lib.rs`. Check that `use native_theme::AnimatedIcon` works.
**Warning signs:** Type is defined but `use native_theme::AnimatedIcon` fails with "not found in crate root".

### Pitfall 6: Doc Comments and Doctests
**What goes wrong:** Category comments in IconRole say "Status (3)" but after rename the variant names change. Doctests referencing StatusLoading fail.
**Why it happens:** The rename preserves the count (still 3 Status variants) but changes variant names.
**How to avoid:** Check all `///` comments on IconRole and its variants. The `IconRole::ALL.len()` doctest says 42 which stays correct. Category counts stay correct (3 Status variants).
**Warning signs:** `cargo test --doc -p native-theme` fails.

## Code Examples

### Module Structure for animated.rs

```rust
// native-theme/src/model/animated.rs
//
// Animated icon types for frame-by-frame and transform-based animations.

use super::icons::IconData;

// ... (types as shown in Pattern 1 above)

impl AnimatedIcon {
    // ... (first_frame as shown in Pattern 2 above)
}
```

### Re-export Chain

```rust
// model/mod.rs additions:
pub mod animated;
pub use animated::{AnimatedIcon, Repeat, TransformAnimation};

// lib.rs additions:
pub use model::{AnimatedIcon, Repeat, TransformAnimation};
// ... and add loading_indicator to re-exports or define it directly in lib.rs
```

### Test Pattern for AnimatedIcon Construction

```rust
#[test]
fn animated_icon_frames_construction() {
    let frames = vec![
        IconData::Svg(b"<svg>frame1</svg>".to_vec()),
        IconData::Svg(b"<svg>frame2</svg>".to_vec()),
    ];
    let anim = AnimatedIcon::Frames {
        frames,
        frame_duration_ms: 83, // ~12fps
        repeat: Repeat::Infinite,
    };
    assert!(matches!(anim, AnimatedIcon::Frames { .. }));
}

#[test]
fn animated_icon_transform_construction() {
    let icon = IconData::Svg(b"<svg>spinner</svg>".to_vec());
    let anim = AnimatedIcon::Transform {
        icon,
        animation: TransformAnimation::Spin { duration_ms: 1000 },
    };
    assert!(matches!(anim, AnimatedIcon::Transform { .. }));
}

#[test]
fn first_frame_returns_first_for_frames() {
    let frames = vec![
        IconData::Svg(b"<svg>first</svg>".to_vec()),
        IconData::Svg(b"<svg>second</svg>".to_vec()),
    ];
    let anim = AnimatedIcon::Frames {
        frames: frames.clone(),
        frame_duration_ms: 100,
        repeat: Repeat::Infinite,
    };
    assert_eq!(anim.first_frame(), Some(&frames[0]));
}

#[test]
fn first_frame_returns_icon_for_transform() {
    let icon = IconData::Svg(b"<svg>spin</svg>".to_vec());
    let anim = AnimatedIcon::Transform {
        icon: icon.clone(),
        animation: TransformAnimation::Spin { duration_ms: 1000 },
    };
    assert_eq!(anim.first_frame(), Some(&icon));
}

#[test]
fn first_frame_returns_none_for_empty_frames() {
    let anim = AnimatedIcon::Frames {
        frames: vec![],
        frame_duration_ms: 100,
        repeat: Repeat::Infinite,
    };
    assert_eq!(anim.first_frame(), None);
}

#[test]
fn loading_indicator_returns_none_all_sets() {
    // Phase 27: all sets return None until Phase 28 wires up frame data
    assert!(loading_indicator("material").is_none());
    assert!(loading_indicator("lucide").is_none());
    assert!(loading_indicator("freedesktop").is_none());
    assert!(loading_indicator("sf-symbols").is_none());
    assert!(loading_indicator("segoe-fluent").is_none());
    assert!(loading_indicator("unknown").is_none());
}
```

### StatusBusy Test Updates

```rust
// In icons.rs tests, update:
assert!(all.contains(&IconRole::StatusBusy));

// Update known_gaps:
fn known_gaps() -> &'static [(IconSet, IconRole)] {
    &[
        (IconSet::SfSymbols, IconRole::FolderOpen),
        (IconSet::SfSymbols, IconRole::StatusBusy),
        (IconSet::SegoeIcons, IconRole::StatusBusy),
    ]
}

// Coverage counts stay the same:
// SF Symbols: 42 - 2 None (FolderOpen, StatusBusy) = 40 Some
// Segoe: 42 - 1 None (StatusBusy) = 41 Some
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| StatusLoading in IconRole | StatusBusy + loading_indicator() | v0.4.0 (this phase) | Breaking: match arms on StatusLoading must update |
| No animated icon types | AnimatedIcon enum | v0.4.0 (this phase) | New API surface, no breaking changes |

**Deprecated/outdated:**
- `StatusLoading`: Removed entirely in v0.4.0. Migration: use `StatusBusy` for static busy indicators, use `loading_indicator()` for animated spinners.

## Open Questions

1. **Should AnimatedIcon live in icons.rs or a new animated.rs?**
   - What we know: icons.rs is already ~1580 lines. AnimatedIcon is a distinct concept from static icons.
   - What's unclear: Whether the planner/implementer prefers a new module or inline additions.
   - Recommendation: New `model/animated.rs` module. Keeps separation of concerns clean. The re-export chain is 2 lines per module level.

2. **Should StatusBusy map to SF Symbols / Segoe names or remain None?**
   - What we know: StatusLoading returned None for both because "loading is animated." StatusBusy is semantically "static busy indicator" which is subtly different.
   - What's unclear: Whether there are reasonable static busy indicators on SF Symbols (e.g., `hourglass`, `ellipsis`) or Segoe.
   - Recommendation: Keep as None for now. The gap was documented and accepted for StatusLoading; the semantic shift to StatusBusy doesn't change the fact that these platforms lack a good static equivalent. Can be filled in a future minor release.

3. **Should loading_indicator() live in lib.rs or a separate module?**
   - What we know: load_icon() is defined directly in lib.rs. It's the established pattern.
   - What's unclear: Whether animated icon loading will grow complex enough to warrant its own module.
   - Recommendation: Define in lib.rs for consistency with load_icon(). If Phase 28+ needs more complex dispatch logic, it can be refactored then.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | cargo test (built-in Rust test framework) |
| Config file | Cargo.toml `[dev-dependencies]` |
| Quick run command | `cargo test -p native-theme --features material-icons,lucide-icons` |
| Full suite command | `cargo test -p native-theme --features material-icons,lucide-icons,svg-rasterize` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| ANIM-01 | AnimatedIcon enum with Frames and Transform variants | unit | `cargo test -p native-theme --features material-icons animated_icon -- --exact` | No -- Wave 0 |
| ANIM-02 | Frames variant fields: frames, frame_duration_ms, repeat | unit | `cargo test -p native-theme --features material-icons animated_icon_frames -- --exact` | No -- Wave 0 |
| ANIM-03 | Transform variant fields: icon, animation | unit | `cargo test -p native-theme --features material-icons animated_icon_transform -- --exact` | No -- Wave 0 |
| ANIM-04 | TransformAnimation::Spin with duration_ms | unit | `cargo test -p native-theme --features material-icons transform_spin -- --exact` | No -- Wave 0 |
| ANIM-05 | loading_indicator() dispatches and returns Option<AnimatedIcon> | unit | `cargo test -p native-theme --features material-icons,lucide-icons loading_indicator -- --exact` | No -- Wave 0 |
| ANIM-06 | first_frame() returns correct IconData reference | unit | `cargo test -p native-theme --features material-icons first_frame -- --exact` | No -- Wave 0 |
| BREAK-01 | StatusLoading removed, StatusBusy added | unit | `cargo test -p native-theme --features material-icons,lucide-icons StatusBusy` | No -- Wave 0 |
| BREAK-02 | All internal references compile and pass | integration | `cargo test -p native-theme --features material-icons,lucide-icons` | Partially -- existing tests need update |

### Sampling Rate
- **Per task commit:** `cargo test -p native-theme --features material-icons,lucide-icons`
- **Per wave merge:** `cargo test -p native-theme --features material-icons,lucide-icons,svg-rasterize`
- **Phase gate:** Full suite green + `cargo check -p native-theme-gpui` (connector compilation)

### Wave 0 Gaps
- [ ] `model/animated.rs` -- new file with AnimatedIcon, TransformAnimation, Repeat types and tests
- [ ] Tests for first_frame() on both variants + empty frames edge case
- [ ] Tests for loading_indicator() returning None for all known icon set names
- [ ] Tests for StatusBusy in ALL array, icon_name mappings, known_gaps
- [ ] Update existing tests referencing StatusLoading (12+ assertions in icons.rs, 1 in winicons.rs)

## Sources

### Primary (HIGH confidence)
- Project codebase: `native-theme/src/model/icons.rs` -- IconRole, IconData, IconSet definitions, icon_name mappings, ALL array, known_gaps, all tests
- Project codebase: `native-theme/src/model/bundled.rs` -- StatusLoading match arms in material_svg() and lucide_svg()
- Project codebase: `native-theme/src/lib.rs` -- load_icon() dispatch pattern, re-export chain
- Project codebase: `connectors/native-theme-gpui/src/icons.rs` -- StatusLoading reference in icon_name()
- Project codebase: `connectors/native-theme-gpui/examples/showcase.rs` -- StatusLoading in example
- Project codebase: `docs/animated-icons.md` -- Full design document with type definitions (Combo 3)
- Project codebase: `CONTEXT.md` -- User decisions for this phase

### Secondary (MEDIUM confidence)
- Project codebase: `native-theme/src/winicons.rs` -- StatusLoading in test (line 510-514)
- Project codebase: `docs/icon-gaps-and-fallback-removal.md` -- StatusLoading gap documentation

### Tertiary (LOW confidence)
None -- all findings sourced directly from codebase.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- no new dependencies, all work in existing codebase
- Architecture: HIGH -- clear patterns from existing load_icon/IconData/IconRole, design doc provides exact type shapes
- Pitfalls: HIGH -- comprehensive grep of all StatusLoading references, compiler will catch missed references

**Research date:** 2026-03-18
**Valid until:** 2026-04-18 (stable -- pure Rust type definitions, no external dependencies to drift)
