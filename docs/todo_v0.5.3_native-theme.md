# v0.5.3 Review -- native-theme (core crate)

Verified against source code on 2026-04-01. Each chapter covers one
problem, lists all resolution options with full pro/contra, and
recommends the best solution. Pre-1.0 -- backward compatibility is
not a constraint.

**Companion documents:**
- [todo_v0.5.3_native-theme-build.md](todo_v0.5.3_native-theme-build.md)
- [todo_v0.5.3_native-theme-gpui.md](todo_v0.5.3_native-theme-gpui.md)
- [todo_v0.5.3_native-theme-iced.md](todo_v0.5.3_native-theme-iced.md)

---

## Summary

| # | Issue | Severity | Fix complexity |
|---|-------|----------|----------------|
| 1 | `resolve()` has no completeness check for inheritance rules | High | Low |
| 2 | `validate()` checks presence but not value ranges | High | Low |
| 3 | Community presets hardcode `icon_set = "freedesktop"` | High | Trivial |
| 4 | Custom themes from scratch need ~116 explicit fields | High | Low |
| 5 | `resolve()` Phase 6 reads OS icon theme at runtime | Medium | Low |
| 6 | Three detection functions cache forever via OnceLock | Medium | Low |
| 7 | Preset registry requires manual bookkeeping at 5 sites | Medium | Medium |
| 8 | Linux subprocess fallbacks fail silently | Medium | Low |
| 9 | TOML deserialization silently ignores unknown fields | Medium | Medium |
| 10 | `from_linux()` and `from_system_async_inner()` duplicate DE dispatch | Low | Medium |
| 11 | `resolve_font_inheritance()` clones `defaults.font` unnecessarily | Low | Trivial |
| 12 | `bundled_icon_svg()` needs `#[allow(unreachable_patterns)]` for cosmetic reasons | Low | Trivial |
| 13 | `resolve()` doc comment says "~90 rules in 4-phase order" | Low | Trivial |
| 14 | `switch.unchecked_bg` lacks a resolve rule despite obvious derivation | Low | Trivial |
| 15 | Widget field naming is inconsistent across structs | Low | Medium |

---

## 1. resolve() has no completeness check for inheritance rules

**File:** `native-theme/src/resolve.rs:133-163`

**What:** The resolve engine applies 94 hand-written `if x.is_none() {
x = d.something; }` rules across 6 phases. Adding a new field to a
widget requires adding its resolve rule here. If forgotten, the field
stays `None` and `validate()` reports it as missing -- but only at
runtime, and only when the field is not explicitly set by the input
theme.

The `validate()` struct literal construction IS structurally complete
(adding a field to `define_widget_pair!` forces a `require()` call
because the `ResolvedXxxTheme` struct literal won't compile without
it). But `resolve()` has no such check. A missing resolve rule for a
field that all bundled presets happen to set explicitly is invisible to
the test suite.

**Risk scenario:** A new field `button.hover_bg` is added. All 16
presets set it. Tests pass. A user creates a custom theme without
`hover_bg`. resolve() doesn't fill it (missing rule). validate()
returns `Err("missing field: button.hover_bg")`. The user gets a
confusing error about a field they never heard of.

### Options

**A. Add a "minimal variant" completeness test (recommended)**

Create a test that constructs a `ThemeVariant` with only the root
fields that have no derivation source. These root fields are the
~46 leaf defaults (colors, fonts, spacing, geometry, icon sizes,
accessibility flags) plus the ~65 geometry/behavior fields that have
no derivation path in `resolve()`. Call `resolve()` then `validate()`.
If any *derived* field lacks a resolve rule, it remains `None` and
validate reports it.

The test should construct TWO variants:

1. A "roots-only" variant with just the 46 defaults root fields and
   the 65 widget geometry fields. This verifies that resolve() can
   fill ALL remaining ~94 fields via inheritance.

2. A "cross-check" test that programmatically compares the set of
   fields resolve() touches against the set validate() requires, to
   flag any field validated but never assigned by a rule.

- Pro: Catches any missing resolve rule, even for fields that presets
  set explicitly.
- Pro: Low effort -- one test function, ~60 lines.
- Pro: No architectural change; works within the existing design.
- Pro: Documents the set of "root fields" that must be explicitly
  provided.
- Pro: Self-policing -- adding a root field without adding it to
  this test causes the test to fail.
- Con: The "minimal set" must be maintained as new root fields are
  added.

**B. Declarative inheritance table**

Replace the procedural if-chains with a const array:
```rust
const RULES: &[(&str, &str)] = &[
    ("button.background", "defaults.background"),
    ("button.primary_bg", "defaults.accent"),
    ...
];
```
Walk the table in resolve() via reflection or field-accessor closures.

- Pro: Single source of truth; completeness can be checked at compile
  time against the struct fields.
- Pro: Rules are auditable at a glance.
- Con: High effort -- Rust lacks runtime field access, so either a proc
  macro or a code generator is needed.
- Con: Complex rules (font sub-field inheritance, cross-widget chains)
  are hard to express in a flat table.
- Con: Significant refactor risk for a working system.

**C. Extend `define_widget_pair!` to emit resolve code**

Each widget definition declares its inheritance rules inline:
```rust
define_widget_pair! {
    ButtonTheme / ResolvedButtonTheme {
        option {
            background: Rgba => defaults.background,
            ...
        }
    }
}
```

- Pro: Single source of truth per widget.
- Con: Cross-widget rules (Phase 4: inactive title bar <- active
  title bar) cannot be expressed in a per-widget macro.
- Con: The macro becomes significantly more complex.
- Con: Font inheritance has 3-sub-field logic that doesn't fit a simple
  `=> source` syntax.

**D. Keep status quo**

- Pro: No change needed.
- Con: Silent bug risk for new fields remains.

### Recommendation

**Option A.** The minimal-variant test is cheap, effective, and
self-policing. It provides >90% of the safety of a declarative table
at <5% of the effort. Option B is a worthwhile long-term goal if the
widget count continues growing, but is not justified today.

---

## 2. validate() checks presence but not value ranges

**File:** `native-theme/src/resolve.rs:556-1515`

**What:** `validate()` calls `require()` on each field, which checks
`is_some()` and returns the inner value. No range validation is
performed. A theme with `radius = -5.0`, `disabled_opacity = 3.0`, or
`font.size = 0.0` passes validation and produces a
`ResolvedThemeVariant` that connectors consume uncritically.

Downstream consequences:
- gpui config.rs does `d.radius.round() as usize` -- a negative
  radius produces 0 via saturating cast (Rust 2021+), which is wrong
  but not UB. On older Rust editions this was UB.
- iced renders with zero-size fonts, invisible borders, or negative
  padding.
- Debug time is spent chasing rendering bugs back to config values
  that should have been rejected at the validation boundary.

### Options

**A. Add range checks in validate() (recommended)**

After the existing `require()` calls, add range assertions before the
struct literal:

```rust
if defaults_radius < 0.0 { missing.push("defaults.radius (must be >= 0)"); }
if defaults_disabled_opacity < 0.0 || defaults_disabled_opacity > 1.0 { ... }
if defaults_font.size <= 0.0 { ... }
```

Fields to validate (exhaustive):
- `>= 0.0`: radius, radius_lg, frame_width, focus_ring_width,
  focus_ring_offset, all spacing values, all padding values, all
  min_width/min_height, all icon sizes, track_height, thumb_size,
  tick_length, height, width, min_thumb_height, slider_width,
  diameter, min_size, stroke_width, arrow_size, arrow_area_width,
  segment_height, separator_width, header_height, content_padding,
  button_spacing, item_height, item_spacing, icon_spacing, max_width
- `> 0.0`: font.size, mono_font.size, line_height,
  text_scaling_factor, all text_scale entry sizes
- `0.0..=1.0`: disabled_opacity, border_opacity
- `100..=900`: font.weight, mono_font.weight (CSS font-weight spec)

- Pro: Catches config errors at the validation boundary, before any
  connector sees them.
- Pro: Error messages are clear ("defaults.radius must be >= 0, got
  -5.0").
- Pro: No architectural change -- range checks interleave naturally
  with the existing require() calls.
- Pro: Prevents the saturating-cast issue in gpui.
- Con: ~50 additional lines of validation code.
- Con: May reject exotic configurations (e.g., radius=-1 meaning
  "pill shape" on some platforms). Mitigated: no platform uses
  negative radius semantically.

**B. Newtype wrappers (NonNegativeF32, Opacity, etc.)**

- Pro: Enforces invariants at construction -- impossible to create an
  invalid value.
- Pro: Self-documenting types.
- Con: Pervasive API change: every field in ThemeDefaults, every
  widget struct, every preset TOML, every connector mapping.
- Con: Serde integration adds complexity (custom deserialize impls).
- Con: Massive refactor for a marginal improvement over Option A.

**C. Separate `validate_ranges()` pass**

- Pro: Keeps validate() focused on presence.
- Con: Two validation passes that could be one.
- Con: Users must remember to call both.
- Con: `into_resolved()` would need to call both, but then it's just
  Option A split across two methods for no gain.

**D. Keep status quo**

- Pro: No change.
- Con: Invalid values propagate to connectors and cause hard-to-debug
  rendering bugs.

### Recommendation

**Option A.** Range checks in validate() are simple, comprehensive,
and catch the most common class of theme authoring errors. They
belong at the same boundary where presence is checked.

---

## 3. Community presets hardcode icon_set = "freedesktop"

**Files:** `native-theme/src/presets/catppuccin-latte.toml`,
`catppuccin-frappe.toml`, `catppuccin-macchiato.toml`,
`catppuccin-mocha.toml`, `nord.toml`, `dracula.toml`, `gruvbox.toml`,
`solarized.toml`, `tokyo-night.toml`, `one-dark.toml`

**What:** All 10 community presets set `icon_set = "freedesktop"` in
both their `[light]` and `[dark]` sections (line 8 and ~205 in each
file). The resolve engine (`resolve.rs:155-157`) only fills `icon_set`
from `system_icon_set()` when the field is `None`:

```rust
if self.icon_set.is_none() {
    self.icon_set = Some(crate::model::icons::system_icon_set());
}
```

Since the presets explicitly set `Some(Freedesktop)`, the system
default is never consulted. A macOS user calling
`ThemeSpec::preset("dracula")?.into_variant(true)?.into_resolved()?`
gets `IconSet::Freedesktop`, which resolves to nothing on macOS. The
same applies to Windows users getting freedesktop instead of
`SegoeIcons`.

This does not affect the `SystemTheme::from_system()` pipeline (which
uses platform presets), but it blocks the standalone preset path.

Platform presets (`kde-breeze`, `adwaita`) correctly set freedesktop
because they are explicitly Linux-targeted. `windows-11` correctly
sets `segoe-fluent` and `macos-sonoma` correctly sets `sf-symbols`.
Those should remain as-is.

### Options

**A. Remove icon_set from community preset TOMLs (recommended)**

Delete the `icon_set = "freedesktop"` line from all 10 community
presets. After removal:
- On Linux: `resolve()` fills `Freedesktop` (same behavior as before)
- On macOS: `resolve()` fills `SfSymbols` (correct)
- On Windows: `resolve()` fills `SegoeIcons` (correct)

Also update the test `icon_set_community_presets_are_freedesktop` in
`presets.rs:379-408`: change it to assert `icon_set.is_none()` on the
parsed `ThemeSpec`, or assert the correct platform value after
`resolve()`.

- Pro: Fixes cross-platform preset usage.
- Pro: Trivial change -- delete 20 lines (2 per preset, light+dark).
- Pro: No behavioral change on Linux.
- Pro: Community presets are color schemes; they have no platform-
  specific icon semantics.
- Con: None identified.

**B. Add per-platform icon_set overrides in preset TOML format**

Extend the TOML schema to support platform-conditional fields:
```toml
[light]
icon_set.linux = "freedesktop"
icon_set.macos = "sf-symbols"
icon_set.windows = "segoe-fluent"
```

- Pro: Explicit per-platform control.
- Con: Schema change affecting all consumers.
- Con: Unnecessary complexity -- `resolve()` already provides the
  correct platform default.

**C. Keep status quo**

- Pro: No change.
- Con: macOS and Windows users cannot use community presets with
  working icons via the standalone preset path.

### Recommendation

**Option A.** The fix is trivial and correct. Community presets have
no platform-specific icon semantics, so there is no reason to hardcode
freedesktop.

---

## 4. Custom themes from scratch need ~116 explicit fields

**Files:** `native-theme/src/resolve.rs` (resolve engine),
`native-theme/src/model/mod.rs:328-333` (from_toml docs)

**What:** Of the ~210 fields in `ResolvedThemeVariant`, the resolve
engine automatically fills 94 via inheritance rules. The remaining
~116 fields have no derivation path and MUST be provided explicitly in
the TOML/preset. These fall into two categories:

- **~46 "root" defaults fields** (colors, fonts, spacing, geometry
  constants, icon sizes, accessibility flags) -- the inputs that drive
  all inheritance.
- **~69 widget geometry/behavior fields** that are widget-specific and
  cannot be sensibly derived from defaults: min_width, min_height,
  padding_horizontal, padding_vertical, track_height, thumb_size,
  button_order, overlay_mode, etc. (One additional field,
  `switch.unchecked_bg`, is covered in issue #14.)

The `from_toml()` docstring says: "All fields are `Option<T>` -- omit
any field you don't need. Unknown fields are silently ignored."
(mod.rs:333). This implies minimal themes work out of the box, but in
reality any TOML that lacks the ~65 geometry fields will fail
`validate()` with a wall of "missing field" errors.

**Risk scenario:** A user writes a color-only custom theme:

```toml
name = "My Dark Theme"
[dark.defaults]
accent = "#ff6600"
background = "#1e1e1e"
foreground = "#e0e0e0"
```

Calling `into_resolved()` on this produces ~100 missing fields
spanning every widget's geometry. The error gives no hint that the user
should start from a preset.

**Impact:** Every custom-theme author hits this on first contact.
The workaround (merge onto a preset base) is simple but undocumented
in the API surface:

```rust
let mut base = ThemeSpec::preset("material")?;
let custom = ThemeSpec::from_toml(user_toml)?;
base.merge(&custom);
let resolved = base.into_variant(is_dark).unwrap().into_resolved()?;
```

### Options

**A. Add `ThemeSpec::from_toml_with_base()` convenience (recommended)**

```rust
impl ThemeSpec {
    /// Parse custom TOML and merge onto a platform-appropriate preset.
    /// Uses "material" as the default base (cross-platform geometry).
    pub fn from_toml_with_base(toml_str: &str, base: &str) -> Result<Self> {
        let mut theme = Self::preset(base)?;
        let overlay = Self::from_toml(toml_str)?;
        theme.merge(&overlay);
        Ok(theme)
    }
}
```

- Pro: One-step API for the most common custom-theme workflow.
- Pro: Non-breaking addition; `from_toml()` unchanged.
- Pro: Makes it obvious that a base preset is expected.
- Pro: "material" preset provides platform-neutral geometry that
  works everywhere.
- Con: Users must choose a base preset name (but "material" is a
  sensible default).
- Con: Minor API surface addition.

**B. Add geometry-only defaults to resolve() as a final fallback**

For the 69 geometry fields, add hardcoded platform-neutral fallbacks:
```rust
if self.button.min_height.is_none() { self.button.min_height = Some(32.0); }
```

- Pro: Custom themes "just work" without a base preset.
- Pro: No API change.
- Con: 69 hardcoded magic numbers in the core crate.
- Con: Couples the crate to specific design conventions.
- Con: Users get Material-like geometry without choosing it, which may
  clash with their platform expectations.
- Con: Undermines the design: presets should own geometry, not the
  resolve engine.

**C. Improve error messages and documentation only**

Add a note to `into_resolved()` docs: "Custom themes should start
from a preset. See `ThemeSpec::preset()` and `merge()`." Also improve
the ThemeResolutionError to categorize missing fields into "root
defaults" vs "widget geometry" with a hint that the geometry fields
typically come from a base preset.

- Pro: No code change beyond docs and error formatting.
- Pro: Guides users to the correct workflow.
- Con: Does not simplify the actual workflow -- still requires 3-step
  parse-merge-resolve.

**D. Keep status quo**

- Pro: No change.
- Con: First-contact failure for every custom theme author.

### Recommendation

**Option A + C.** The convenience method makes the preset-first
workflow a single call, and improved error messages guide users who
call `from_toml()` directly. Together they eliminate the
first-contact confusion without hardcoding geometry.

---

## 5. resolve() Phase 6 reads OS icon theme at runtime

**File:** `native-theme/src/resolve.rs:155-162`

**What:** `resolve()` is documented as a pure data transform
("apply all 94 inheritance rules"). Phases 1-5 are pure: they only
read and assign fields within the `ThemeVariant` struct. Phase 5
(`system_icon_set()`) is also pure -- it uses compile-time `cfg!()`
macros to return a platform constant, with no runtime OS access.

However, Phase 6 calls `system_icon_theme()`, which on Linux reads
`XDG_CURRENT_DESKTOP`, spawns `gsettings`, reads KDE/XFCE/LXQt
config files, etc. (`model/icons.rs:458-517`). This makes `resolve()`
impure:

- Unit tests that call `resolve()` get different `icon_theme` results
  depending on the developer's desktop environment.
- Two calls to `resolve()` on the same input produce different results
  if env vars change between calls (though OnceLock caching masks this
  for the first-vs-second call in the same process).

Note: On macOS and Windows, `system_icon_theme()` returns a
compile-time constant (`"sf-symbols"` and `"segoe-fluent"`
respectively), so Phase 6 is only impure on Linux.

### Options

**A. Move Phase 6 to a separate method (recommended)**

Split the icon_theme fallback out of resolve():
```rust
impl ThemeVariant {
    /// Apply all inheritance rules (phases 1-5, pure).
    pub fn resolve(&mut self) {
        self.resolve_defaults_internal();
        self.resolve_safety_nets();
        self.resolve_widgets_from_defaults();
        self.resolve_widget_to_widget();
        // Phase 5: icon_set via cfg!() -- pure
        if self.icon_set.is_none() {
            self.icon_set = Some(crate::model::icons::system_icon_set());
        }
    }

    /// Fill icon_theme from OS detection (phase 6, impure on Linux).
    pub fn resolve_platform_defaults(&mut self) {
        if self.icon_theme.is_none() {
            self.icon_theme = Some(crate::model::icons::system_icon_theme().to_string());
        }
    }

    /// Apply all rules including platform detection.
    pub fn resolve_all(&mut self) {
        self.resolve();
        self.resolve_platform_defaults();
    }
}
```
`into_resolved()` and `SystemTheme::from_system()` call
`resolve_all()`. Users who want pure resolution call `resolve()`.

- Pro: resolve() becomes a pure data transform, deterministic per
  build target.
- Pro: Testable in isolation without mocking OS state.
- Pro: Non-breaking -- existing callers of `into_resolved()` get the
  same behavior via `resolve_all()`.
- Con: Two methods instead of one; users must choose.
- Con: Only 1 rule moves (Phase 6), which limits the motivation.
  Phase 5 is already pure.

**B. Accept icon_theme as a parameter**

```rust
pub fn resolve_with_icon_theme(&mut self, default_theme: &str)
```

- Pro: Fully explicit; no hidden OS access.
- Con: Changes the most common API path; callers must supply a value
  they may not care about.
- Con: The `SystemTheme` pipeline already resolves icons correctly, so
  this adds friction to the common case.

**C. Keep status quo**

- Pro: No change; `resolve()` "just works" for the common case.
- Con: Impure function with one runtime side effect disguised as a
  pure data transform.
- Con: Test reproducibility on Linux depends on desktop environment.

### Recommendation

**Option A.** Separating pure resolution from platform defaults makes
the API honest and testable. The combined `resolve_all()` preserves
convenience. Since only Phase 6 is impure, the separation is
minimal but principled.

---

## 6. Three detection functions cache forever via OnceLock

**Files:** `native-theme/src/lib.rs:243-245, 369-371`,
`native-theme/src/model/icons.rs:458-477`

**What:** Three public detection functions use `OnceLock` to cache
their result forever:

| Function | File | What it caches |
|----------|------|----------------|
| `system_is_dark()` | lib.rs:243 | Dark mode preference |
| `prefers_reduced_motion()` | lib.rs:369 | Reduced motion setting |
| `system_icon_theme()` | icons.rs:458 | Icon theme name string |

```rust
pub fn system_is_dark() -> bool {
    static CACHED_IS_DARK: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *CACHED_IS_DARK.get_or_init(detect_is_dark_inner)
}
```

If a user toggles dark mode or changes icon theme while the app is
running, these functions return stale data. The only way to get fresh
data is `SystemTheme::from_system()`, which re-runs the full pipeline.
There is no lightweight uncached query.

Note: `system_icon_set()` (`icons.rs:419-429`) is NOT cached via
OnceLock -- it uses compile-time `cfg!()` macros and returns a
platform constant. Its result is deterministic per build target and
does not change at runtime, so caching is irrelevant.

The caching concern is most relevant for `system_is_dark()` (user
toggles light/dark mode) and `system_icon_theme()` (user changes icon
theme in DE settings). `prefers_reduced_motion()` changes less often
but the same issue applies.

### Options

**A. Add uncached public variants (recommended)**

```rust
/// Detect dark mode without caching. Suitable for polling.
pub fn detect_is_dark() -> bool { detect_is_dark_inner() }

/// Detect reduced motion without caching.
pub fn detect_reduced_motion() -> bool { detect_reduced_motion_inner() }

/// Detect icon theme without caching.
pub fn detect_icon_theme() -> String { detect_linux_icon_theme() }
```

- Pro: Non-breaking addition.
- Pro: Users who need fresh data call the uncached variant.
- Pro: Cached variant stays available for startup-path performance.
- Con: Two functions for the same query; users must choose.

**B. Replace OnceLock with RwLock + refresh()**

```rust
pub fn refresh_system_settings() { *CACHED.write() = detect_is_dark_inner(); }
```

- Pro: Single function; cached value can be updated.
- Con: RwLock contention on every read (even though the write is rare).
- Con: Global mutable state; harder to reason about.

**C. Remove caching entirely**

- Pro: Always fresh.
- Con: `system_is_dark()` spawns a subprocess on Linux (gsettings).
  Calling it per-frame (60fps) would spawn 60 processes/second.

**D. Keep status quo**

- Pro: No change.
- Con: No lightweight path for apps that need to track theme changes.

### Recommendation

**Option A.** Expose the existing inner functions as public uncached
variants. The cached functions remain for the startup path.

---

## 7. Preset registry requires manual bookkeeping at 5 sites

**File:** `native-theme/src/presets.rs:15-117`

**What:** Adding a new bundled preset requires updating 5 disconnected
locations in `presets.rs`:

1. `const XXX_TOML: &str = include_str!(...)` (line ~15)
2. `static XXX: LazyLock<Parsed>` in the `cached` module (line ~69)
3. Match arm in `cached::get()` (line ~92)
4. `PRESET_NAMES` array (line ~39)
5. Optionally `PLATFORM_SPECIFIC` (line ~132)

Missing any one site causes a subtle bug: the preset either won't
load, won't appear in `list_presets()`, or won't filter correctly.
Live presets (4 additional entries) have the same 3-site pattern
(const + LazyLock + match arm) but are excluded from PRESET_NAMES.

### Options

**A. Data-driven array + HashMap (recommended)**

Replace individual `const` + `LazyLock` + match-arm triples with a
single data array that drives all behavior:

```rust
const PRESET_ENTRIES: &[(&str, &str)] = &[
    ("kde-breeze", include_str!("presets/kde-breeze.toml")),
    ("adwaita", include_str!("presets/adwaita.toml")),
    // ...all 16 user-facing + 4 live presets
];

const PRESET_NAMES: &[&str] = &[
    "kde-breeze", "adwaita", /* ...first 16 entries only */
];

static CACHE: LazyLock<HashMap<&str, Parsed>> = LazyLock::new(|| {
    PRESET_ENTRIES
        .iter()
        .map(|(name, toml)| (*name, parse(toml)))
        .collect()
});

pub(crate) fn get(name: &str) -> Option<&Parsed> {
    CACHE.get(name)
}
```

- Pro: Adding a preset requires updating exactly 2 sites: the
  `PRESET_ENTRIES` array and `PRESET_NAMES` (or 1 site if live
  preset).
- Pro: No match arm, no individual LazyLock static to declare.
- Pro: Simpler than a macro; easy to understand and debug.
- Pro: Compile error if the TOML file doesn't exist (include_str!).
- Con: Still requires `PRESET_NAMES` to be maintained separately
  (it's a subset of PRESET_ENTRIES; the user-facing list excludes
  live presets). Could be derived with a const filter, but
  `PLATFORM_SPECIFIC` still needs a separate declaration.
- Con: HashMap has allocation and lookup overhead (negligible
  compared to match, and happens once at init).

**B. Declarative macro**

```rust
macro_rules! define_presets {
    ($($slug:literal => $file:literal),* $(,)?) => {
        $(const $slug_CONST: &str = include_str!($file);)*
        mod cached { ... } // generate LazyLock + get()
        const PRESET_NAMES: &[&str] = &[$($slug),*];
    }
}
```

- Pro: Single site to add a new preset.
- Pro: Eliminates 3 of the 5 manual steps.
- Pro: Compile error if the TOML file doesn't exist.
- Con: Macro complexity; ident generation from string literals
  requires a helper (e.g., `paste!` crate or manual snake_case).
- Con: `PLATFORM_SPECIFIC` still needs a separate declaration.
- Con: Live presets need a second invocation or a flag in the macro.

**C. Build script that scans presets/ directory**

- Pro: Fully automatic; drop a TOML file in presets/ and it's
  discovered.
- Pro: No macro complexity.
- Con: Adds a build.rs to the core crate (currently has none).
- Con: Preset ordering becomes filesystem-dependent.
- Con: Harder to understand and debug than a macro.

**D. Keep status quo**

- Pro: No change; existing code is straightforward if verbose.
- Con: Error-prone when adding presets.

### Recommendation

**Option A** (HashMap). It eliminates the most common error (forgetting
a match arm or LazyLock static) with minimal complexity. The remaining
2-site bookkeeping (entry + PRESET_NAMES) is manageable and
self-policing (the test `list_presets_returns_all_sixteen` catches
missing PRESET_NAMES entries).

---

## 8. Linux subprocess fallbacks fail silently

**File:** `native-theme/src/lib.rs:253-283`

**What:** `system_is_dark()` on Linux spawns `gsettings` as a
subprocess. If gsettings is not installed (minimal installs,
containers, Wayland compositors without GNOME), the function silently
returns `false` (light mode). The user gets the wrong theme with no
indication that detection failed.

Similarly, `prefers_reduced_motion()` spawns gsettings and silently
returns `false` (allow animations) on failure.

### Options

**A. Add a diagnostic function (recommended)**

```rust
/// Check whether OS theme detection is available on this platform.
/// Returns a human-readable description of what works and what doesn't.
pub fn diagnose_platform_support() -> Vec<String> { ... }
```

- Pro: Users can call this at startup to understand why detection
  failed.
- Pro: Non-breaking.
- Pro: Useful for debugging in CI and containers.
- Con: Additional API surface.

**B. Return Result instead of bool**

Change `system_is_dark() -> Result<bool>` to distinguish "detected
light" from "detection failed."

- Pro: Explicit error handling.
- Con: Breaking change.
- Con: Most callers want a bool; wrapping in Result adds unwrap noise.

**C. Use the `log` crate for warnings**

- Pro: Standard Rust logging pattern.
- Pro: Visible in any app that configures a log subscriber.
- Con: Adds a dependency (`log`).
- Con: Only visible if the caller has a log subscriber configured.
- Con: Silent in apps that don't use logging.

**D. Keep status quo**

- Pro: No change; `false` is a safe default.
- Con: Silent failure can confuse users who expect dark mode detection.

### Recommendation

**Option A.** A diagnostic function is the most useful for debugging
without changing existing return types or adding dependencies. It can
report: gsettings availability, D-Bus status, kdeglobals readability,
platform features enabled.

---

## 9. TOML deserialization silently ignores unknown fields

**Files:** `native-theme/src/model/mod.rs:57-59`,
`native-theme/src/model/defaults.rs:32-34`,
`native-theme/src/model/widgets/mod.rs:45-51`

**What:** All theme structs (`ThemeVariant`, `ThemeSpec`,
`ThemeDefaults`, all 25 widget structs via `define_widget_pair!`) use
`#[serde(default)]` without `#[serde(deny_unknown_fields)]`. This
means a TOML theme file with a misspelled field (e.g., `backround`
instead of `background`) will silently ignore the typo. The
`from_toml()` docstring explicitly documents this behavior:
"Unknown fields are silently ignored" (mod.rs:333).

After `resolve()`, the missing field is filled from defaults, so
`validate()` also passes. The theme author's custom color is silently
lost with no error or warning at any stage.

**Risk scenario:** A theme author writes:

```toml
[light.defaults]
accent = "#ff6600"
backround = "#1e1e1e"  # typo: "backround" instead of "background"
```

The theme loads successfully. `background` remains `None`, resolve()
fills it from the preset default, and the author's dark background is
silently replaced with the preset's light background.

### Options

**A. Add a `ThemeSpec::lint_toml()` validation function (recommended)**

Provide a function that re-parses the TOML as a generic `toml::Value`
table and walks all keys, comparing them against the known field names
for each section:

```rust
impl ThemeSpec {
    /// Check a TOML string for unrecognized field names.
    /// Returns a list of warnings for keys that don't match any known field.
    pub fn lint_toml(toml_str: &str) -> Vec<String> { ... }
}
```

The known-field lists can be auto-generated by extending the
`define_widget_pair!` macro to emit a `const FIELD_NAMES: &[&str]`
per struct, ensuring they stay in sync automatically:

```rust
impl ButtonTheme {
    pub const FIELD_NAMES: &[&str] = &["background", "foreground", ...];
}
```

- Pro: Non-breaking -- existing behavior unchanged.
- Pro: Theme authors can call `lint_toml()` during development.
- Pro: Can report exact line numbers from TOML spans.
- Pro: No new dependencies.
- Pro: Field name lists stay in sync if generated by the macro.
- Con: Additional API surface and ~100-150 lines of validation code.
- Con: If FIELD_NAMES is not macro-generated, the lists can drift.

**B. Add `#[serde(deny_unknown_fields)]` to all theme structs**

- Pro: Catches typos at parse time with clear error messages.
- Pro: Zero additional code; just add the attribute.
- Con: Breaks forward compatibility -- a TOML file written for a
  newer native-theme version (with new fields) would fail to parse
  on an older version.
- Con: Breaks backward compatibility -- custom TOML files with
  deprecated field names would fail instead of being silently
  ignored.
- Con: Theme authors who intentionally include extra keys (comments
  stored as fields, custom metadata) would need to stop.

**C. Use the `serde_ignored` crate**

Wrap deserialization in `serde_ignored::deserialize()` to intercept
keys that serde drops:

```rust
let mut unused = Vec::new();
let theme: ThemeSpec = serde_ignored::deserialize(
    toml::Deserializer::new(toml_str),
    |path| unused.push(path.to_string()),
)?;
```

- Pro: Reuses serde's own knowledge of known fields; no manual lists.
- Pro: Reports exact paths (e.g., `light.defaults.backround`).
- Pro: Zero maintenance -- automatically stays in sync as fields
  are added or removed.
- Con: Adds a dependency (`serde_ignored`).
- Con: Must be opt-in (can't change `from_toml()` without breaking
  the documented "silently ignored" contract).
- Con: serde_ignored reports ignored keys but doesn't prevent parsing,
  so the return type needs to change or a wrapper function is needed.

**D. Keep status quo**

- Pro: No change; forward compatibility preserved.
- Con: Theme authors waste time debugging silently dropped fields.

### Recommendation

**Option A.** A standalone `lint_toml()` function gives theme authors a
clear debugging tool without changing existing parse behavior or adding
dependencies. It can be called in development builds or theme editor
tooling while leaving production parsing fast and forward-compatible.

Option C (`serde_ignored`) is a strong alternative if zero-maintenance
sync is prioritized over zero-dependency. It avoids the risk of
manual field-name lists drifting, at the cost of one small
dependency.

---

## 10. `from_linux()` and `from_system_async_inner()` duplicate DE dispatch

**Files:** `native-theme/src/lib.rs:731-761` (`from_linux`),
`native-theme/src/lib.rs:808-878` (`from_system_async_inner`)

**What:** Both functions contain a `match detect_linux_de(&desktop)`
block that dispatches to the correct reader and preset for each
desktop environment. The sync version (`from_linux`) has 5 match arms;
the async version (`from_system_async_inner`) has the same 5 plus
portal-enhanced variants guarded by `#[cfg(feature = "portal")]`.

The shared logic is: detect DE → pick preset name → pick reader →
call `run_pipeline()`. The sync version always uses Adwaita preset
for non-KDE desktops; the async version can use portal-enhanced
readers for GNOME and KDE.

Adding a new desktop environment (e.g., COSMIC) requires updating both
functions with matching logic. If only one is updated, the sync and
async paths produce different results for the same DE.

### Options

**A. Extract shared dispatch into a helper (recommended)**

Factor the common DE -> preset-name mapping into a pure function:

```rust
fn linux_preset_for_de(de: LinuxDesktop) -> &'static str {
    match de {
        LinuxDesktop::Kde => "kde-breeze-live",
        _ => "adwaita-live",
    }
}
```

Each caller handles reader acquisition (sync vs async) and calls
`run_pipeline()` with the preset name from the helper.

- Pro: Single site for DE -> preset mapping.
- Pro: Adding a new DE updates one match block.
- Pro: Sync and async paths stay consistent automatically.
- Con: Portal-enhanced variants (async KDE+portal, async GNOME+portal)
  still need separate handling in the async function.

**B. Keep status quo**

- Pro: Each function is self-contained and easy to read.
- Con: 5 match arms duplicated; adding a DE touches 2 functions.

### Recommendation

**Option A.** Extract the DE -> preset mapping. The portal-specific
reader logic stays in `from_system_async_inner`, but the preset
selection is unified.

---

## 11. `resolve_font_inheritance()` clones `defaults.font` unnecessarily

**File:** `native-theme/src/resolve.rs:502-512, 514-529`

**What:** Two methods clone `self.defaults.font` to work around a
perceived borrow checker constraint:

```rust
fn resolve_font_inheritance(&mut self) {
    let defaults_font = &self.defaults.font.clone(); // unnecessary clone
    resolve_font(&mut self.window.title_bar_font, defaults_font);
    ...
}
fn resolve_text_scale(&mut self) {
    let defaults_font = &self.defaults.font.clone(); // same
    ...
}
```

Since `self.defaults.font` and (e.g.) `self.window.title_bar_font`
are disjoint fields of `ThemeVariant`, Rust's NLL borrow checker
permits split borrows without the clone. The `FontSpec` clone
allocates (it contains `Option<String>` for font family).

The fix is:
```rust
let defaults_font = &self.defaults.font;
```

`resolve_font()` takes `(widget_font: &mut Option<FontSpec>,
defaults_font: &FontSpec)` -- it mutates the first arg and only reads
the second. Each call borrows a different widget's font field mutably
while sharing `defaults_font` immutably. NLL handles this correctly
because each mutable borrow path (`self.window.title_bar_font`,
`self.button.font`, etc.) is disjoint from the shared borrow path
(`self.defaults.font`).

### Options

**A. Remove the clone, use direct reference (recommended)**

```rust
let defaults_font = &self.defaults.font;
```

- Pro: Removes 2 unnecessary heap allocations per resolve() call.
- Pro: One-character change (delete `.clone()`) per site.
- Pro: NLL split borrows are well-established; this will compile.
- Con: None identified.

**B. Keep status quo**

- Pro: No risk.
- Con: Unnecessary allocation on every resolve() call.

### Recommendation

**Option A.** Remove both clones. NLL split borrows handle disjoint
struct field access correctly.

---

## 12. bundled_icon_svg() needs #[allow(unreachable_patterns)] for cosmetic reasons

**File:** `native-theme/src/model/bundled.rs:42-103, 106-168`

**What:** The `material_svg()` and `lucide_svg()` functions match on
all 42 `IconRole` variants inside `Some(match ...)`, with a wildcard
arm at the end:

```rust
#[allow(unreachable_patterns)]
fn material_svg(role: IconRole) -> Option<&'static [u8]> {
    Some(match role {
        IconRole::DialogWarning => include_bytes!(...),
        ...  // all 42 variants
        _ => return None, // #[non_exhaustive] forward compat
    })
}
```

The wildcard arm exists for forward compatibility (`IconRole` is
`#[non_exhaustive]`), but since `bundled.rs` is in the same crate,
the attribute has no effect -- all 42 variants are exhaustively
matched, making `_` unreachable. The `#[allow(unreachable_patterns)]`
suppresses the resulting compiler warning.

The code is functionally correct. The remaining issue is cosmetic:
the `return None` inside `Some(match ...)` is unusual. Most match
arms return values wrapped by `Some(...)`, but the wildcard breaks
out of the wrapper via `return`. This works but is non-obvious to
readers.

### Options

**A. Restructure to match-with-Some-per-arm (recommended)**

```rust
fn material_svg(role: IconRole) -> Option<&'static [u8]> {
    match role {
        IconRole::DialogWarning => Some(include_bytes!(...)),
        ...
        _ => None,
    }
}
```

- Pro: No `#[allow(unreachable_patterns)]` needed (wildcard handles
  the unreachable case naturally without the attribute, since `_` is
  a legitimate fallback arm returning `None`).
- Pro: No `return` inside an expression wrapper.
- Pro: Standard Rust match-returning-Option pattern.
- Con: Adds `Some(...)` to every arm (~42 lines touched per function).
- Con: The `_` arm is still technically unreachable within the same
  crate, but the compiler doesn't warn because `_` catches any
  future variant (the intent matches the code).

**B. Keep status quo**

- Pro: No churn; code is correct and well-commented.
- Con: `#[allow(unreachable_patterns)]` and `return None` inside
  `Some()` are mildly confusing.

### Recommendation

**Option A** if touching these functions for other reasons (e.g.,
adding new icon roles). Otherwise, **Option B** is acceptable -- this
is a purely cosmetic issue with no functional impact.

---

## 13. resolve() doc comment says "~90 rules in 4-phase order"

**File:** `native-theme/src/resolve.rs:134`

**What:** The doc comment on `resolve()` reads:

```rust
/// Apply all ~90 inheritance rules in 4-phase order.
```

This is inaccurate on both counts:

1. The actual rule count is **94**, not "~90".
2. The actual phase count is **6**, not 4. The doc comment lists
   phases 1-4 as named methods, but Phase 5 (icon_set via `cfg!()`)
   and Phase 6 (icon_theme via `system_icon_theme()`) are applied
   inline after the four method calls at lines 154-162.

The doc's `# Phases` section describes only the 4 named method phases.
Phases 5 and 6 are present in the code but omitted from the doc.

### Options

**A. Update the doc comment (recommended)**

```rust
/// Apply all 94 inheritance rules in 6-phase order.
///
/// # Phases
///
/// 1. **Defaults internal chains** -- accent derives selection,
///    focus_ring_color; selection derives selection_inactive.
/// 2. **Safety nets** -- platform-divergent fields get a reasonable
///    fallback.
/// 3. **Widget-from-defaults** -- colors, geometry, fonts, text scale
///    entries all inherit from defaults.
/// 4. **Widget-to-widget** -- inactive title bar fields fall back to
///    active.
/// 5. **Icon set** -- fills `icon_set` from platform default
///    (`cfg!()`; pure).
/// 6. **Icon theme** -- fills `icon_theme` from OS detection (impure
///    on Linux; see `system_icon_theme()`).
```

- Pro: Accurate documentation.
- Pro: One-line change + 4 lines added to the doc section.
- Con: None.

**B. Keep status quo**

- Pro: No change.
- Con: Misleading count and phase documentation.

### Recommendation

**Option A.** Fix the doc comment to reflect reality.

---

## 14. `switch.unchecked_bg` lacks a resolve rule despite obvious derivation

**File:** `native-theme/src/resolve.rs` (resolve_color_inheritance,
lines 451-457)

**What:** The switch widget resolves `checked_bg` from
`defaults.accent` (line 452) and `thumb_bg` from `defaults.surface`
(line 455), following the same pattern as slider (fill <- accent,
track <- muted, thumb <- surface). But `unchecked_bg` has no resolve
rule, breaking the analogy:

| Switch field | Resolve rule | Slider analogue |
|-------------|-------------|-----------------|
| `checked_bg` | `defaults.accent` (line 452) | `slider.fill` <- `defaults.accent` (line 367) |
| `thumb_bg` | `defaults.surface` (line 455) | `slider.thumb` <- `defaults.surface` (line 373) |
| `unchecked_bg` | **none** | `slider.track` <- `defaults.muted` (line 370) |

All 16 presets set `unchecked_bg` explicitly (to platform-specific
muted/border tones), so the missing rule is invisible to tests.

**Discarded candidates:** Two other fields were considered but ruled
out after verification:

- `switch.track_radius` -- initially appeared analogous to
  `checkbox.radius` <- `defaults.radius`, but presets show the switch
  track radius is always much larger than `defaults.radius` (e.g.,
  KDE: 9.0 vs 5.0, Material: 16.0 vs 16.0, Adwaita: 14.0 vs 12.0).
  The track is a pill shape; its radius is typically ≈ track_height/2,
  not the general corner radius. A resolve rule would produce wrong
  values.
- `dialog.button_order` -- a platform convention, not a visual
  property. Both readers and all presets (including live presets) set
  it. A resolve fallback of `TrailingAffirmative` would be wrong on
  KDE (`LeadingAffirmative`). This field correctly belongs in the
  preset/reader category.

### Options

**A. Add the resolve rule (recommended)**

```rust
// In resolve_color_inheritance(), switch section:
if self.switch.unchecked_bg.is_none() {
    self.switch.unchecked_bg = d.muted;
}
```

- Pro: Consistent with the slider.track analogy.
- Pro: Custom themes without a base preset get a reasonable "off"
  track color.
- Pro: 3 lines of code.
- Con: None identified.

**B. Keep status quo**

- Pro: No change; all presets set it explicitly.
- Con: Inconsistency with slider.track pattern.
- Con: Custom themes without a base preset fail for this field.

### Recommendation

**Option A.** A single missing rule in an otherwise consistent
pattern. The fix is trivial.

---

## 15. Widget field naming is inconsistent across structs

**Files:** `native-theme/src/model/widgets/mod.rs` (all 25 widget
struct definitions)

**What:** Widget color fields use three different naming conventions
depending on the widget:

| Convention | Examples | Widgets |
|-----------|---------|--------|
| Full names | `background`, `foreground`, `title_bar_background`, `title_bar_foreground` | Window, Button, Input, Menu, Tooltip, Tab, Sidebar, List, Popover, Card |
| Abbreviated | `primary_bg`, `primary_fg`, `checked_bg`, `unchecked_bg`, `thumb_bg` | Button (primary), Switch, Checkbox |
| Semantic (no suffix) | `fill`, `track`, `thumb`, `caret`, `placeholder` | Slider, ProgressBar, Scrollbar, Spinner, Input |

This creates three related problems:

1. **Discoverability:** A user looking for "button primary background"
   might search for `primary_background` and not find `primary_bg`.
2. **Predictability:** After seeing `window.title_bar_background`, a
   user expects `switch.checked_background`, not `checked_bg`.
3. **grep-ability:** Searching for `_background` misses `_bg` fields
   and vice versa.

The semantic names (fill, track, thumb) are justified -- they describe
distinct visual parts, not just "background." The inconsistency is
between `_bg`/`_fg` abbreviations and full `_background`/`_foreground`
names.

### Options

**A. Standardize on full names (recommended)**

Rename abbreviated fields to match the full-name pattern:
- `primary_bg` -> `primary_background`
- `primary_fg` -> `primary_foreground`
- `checked_bg` -> `checked_background`
- `unchecked_bg` -> `unchecked_background`
- `thumb_bg` -> `thumb_background`

Keep semantic names (fill, track, thumb, caret, placeholder) as-is.

- Pro: Consistent across all widgets.
- Pro: More discoverable; matches Rust naming conventions.
- Pro: Pre-1.0; breaking changes are acceptable.
- Pro: grep for `_background` finds all background fields.
- Con: Longer field names in TOML presets and code.
- Con: Touches 5 fields across 3 structs, plus all 16 preset TOMLs
  and both connector crates.
- Con: Churn in a working codebase.

**B. Standardize on abbreviations**

Rename full names to `_bg`/`_fg`:
- `background` -> `bg`, `foreground` -> `fg`
- `title_bar_background` -> `title_bar_bg`

- Pro: Shorter field names everywhere.
- Con: Less readable; `bg` is ambiguous (could be "big", "batch
  group" in different domains).
- Con: Touches far more fields (~50+) than Option A (~5).
- Con: The abbreviated convention is less common in Rust crates.

**C. Keep status quo**

- Pro: No churn.
- Pro: Each widget's naming makes local sense within its own context.
- Con: Inconsistency hurts API learnability.
- Con: Theme authors must remember which widgets use `_bg` vs
  `_background`.

### Recommendation

**Option A** if doing a broader API cleanup pass. Otherwise
**Option C** -- the inconsistency is a minor annoyance, not a
functional problem. The semantic names (fill, track, thumb) should
not change regardless.
