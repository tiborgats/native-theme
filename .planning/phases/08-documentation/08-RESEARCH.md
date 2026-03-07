# Phase 8: Documentation - Research

**Researched:** 2026-03-07
**Domain:** Rust crate documentation, GUI toolkit adapter patterns (egui, iced, slint)
**Confidence:** HIGH

## Summary

Phase 8 is a documentation-only phase: no new code logic, no new modules. The deliverable is a README.md that serves as both the GitHub landing page and (via `#[doc = include_str!("../README.md")]`) the crate-level rustdoc. The README must contain three categories of content: (1) adapter code examples showing how to map NativeTheme fields to egui Visuals/Color32, iced Theme::custom/Palette/Color, and slint global singletons with Color::from_rgb_u8; (2) workflow documentation for the preset workflow (load, merge overrides) and the runtime workflow (from_system with preset fallback); and (3) a feature flag reference table.

All three toolkit APIs have stable, well-documented color types that accept u8 RGB values, which maps directly to native-theme's `Rgba` type (u8 internal representation with `to_f32_array()` for f32-based APIs). The adapter examples are thin mapping functions (~20-50 lines each), not library code -- this is explicitly out of scope per REQUIREMENTS.md ("Built-in toolkit adapters ... adapters live in consumer code").

**Primary recommendation:** Write a single README.md with compile-tested code examples (using `#[cfg(doctest)]` + `include_str!`), structured as: overview, quick start, preset workflow, runtime workflow, adapter examples (egui/iced/slint), feature flags table, and API reference summary.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| DOC-01 | README with adapter examples for egui, iced, and slint | egui: Visuals + Color32::from_rgb mapping; iced: Theme::custom + Palette + Color::from_rgb8 mapping; slint: global singleton + Color::from_rgb_u8 mapping. All three toolkits accept u8 RGB which maps directly from Rgba fields. |
</phase_requirements>

## Standard Stack

### Core

This phase produces documentation only -- no new library dependencies.

| Tool | Purpose | Why Standard |
|------|---------|--------------|
| README.md | Crate documentation, GitHub landing page | Standard Rust crate practice |
| `#[doc = include_str!("../README.md")]` | Include README as crate-level rustdoc | Built into rustc since 1.54, ensures docs.rs shows README content |
| `#[cfg(doctest)]` | Compile-test README code examples | Built into rustdoc, catches broken examples during `cargo test` |

### Adapter Target APIs (documented, not depended on)

These are the toolkit APIs that adapter examples will reference. They are NOT dependencies of native-theme.

| Toolkit | Color Type | Constructor from u8 | Key Styling Entry Point |
|---------|-----------|---------------------|------------------------|
| egui 0.33+ | `Color32` | `Color32::from_rgb(r, g, b)`, `Color32::from_rgba_unmultiplied(r, g, b, a)` | `ctx.set_visuals(Visuals { ... })` |
| iced 0.13+ | `Color` | `Color::from_rgb8(r, g, b)` (returns f32-normalized Color) | `Theme::custom("name", Palette { ... })` |
| slint 1.x | `slint::Color` | `Color::from_rgb_u8(r, g, b)`, `Color::from_argb_u8(a, r, g, b)` | `app.global::<Palette>().set_xxx(color)` |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Inline README doctests | Separate `examples/` directory | Examples dir is discoverable but not compile-tested by default; README doctests verify on every `cargo test` |
| `#[doc = include_str!]` | cargo-rdme (generate README from doc comments) | include_str is zero-dependency, works out of the box; rdme adds build-time tool dependency |
| Markdown README | mdBook | Overkill for a single-crate library; README is sufficient |

## Architecture Patterns

### README Structure

```
README.md
├── Badges (crates.io, docs.rs, license)
├── One-line description
├── Overview (what native-theme is, what it is NOT)
├── Quick Start (add dependency, load a preset)
├── Preset Workflow (load preset, merge user overrides)
├── Runtime Workflow (from_system with preset fallback)
├── Toolkit Adapter Examples
│   ├── egui
│   ├── iced
│   └── slint
├── Feature Flags (table: flag, what it enables, platform)
├── Available Presets (table: name, description)
├── TOML Format Reference (annotated example)
├── License
└── (footer)
```

### Pattern 1: Compile-Tested README via include_str

**What:** The README.md is included as a doc attribute in lib.rs, causing rustdoc to compile-test all Rust code blocks during `cargo test`.
**When to use:** Always, for any crate with code examples in README.
**Example:**
```rust
// In lib.rs, add at the very top (before other doc comments):
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
```

### Pattern 2: Adapter Example as Standalone Function

**What:** Each adapter example is a standalone `fn apply_theme(theme: &NativeTheme, ...)` that maps native-theme fields to the toolkit's styling API. NOT a trait impl, NOT a library export.
**When to use:** For all three toolkit examples.
**Why:** The REQUIREMENTS.md explicitly states "Built-in toolkit adapters ... adapters live in consumer code (~50 lines each)". The examples show what users copy into their own code.

### Pattern 3: Workflow Example with Error Handling

**What:** Each workflow example (preset, runtime) shows the complete happy path including `unwrap_or_else` / `match` on Result, not just `.unwrap()`.
**When to use:** For the preset and runtime workflow sections.
**Why:** Users copy README examples verbatim. Showing proper error handling prevents panic-in-production bugs.

### Anti-Patterns to Avoid

- **Stale code examples:** Code blocks that reference nonexistent API or wrong field names. Prevention: `#[cfg(doctest)]` catches this on every `cargo test`.
- **Overly abstract examples:** Showing just `let theme = preset("default")?;` without showing what to DO with the theme. Each example must show end-to-end: load -> extract fields -> apply.
- **Toolkit version coupling:** Examples must NOT use toolkit-specific version numbers in comments or code. Write `egui::Color32` not "egui 0.33's Color32". The mapping pattern is stable across versions.
- **Documenting private API:** Only document the public re-exports from lib.rs. Don't mention internal module paths like `model::colors::CoreColors` -- use the re-exported `native_theme::CoreColors`.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| README doctest infrastructure | Custom test harness | `#[doc = include_str!("../README.md")]` + `#[cfg(doctest)]` | Built into rustdoc, zero config |
| Color conversion in examples | Manual bit math | `Rgba::to_f32_array()` for f32 APIs, direct `.r .g .b .a` field access for u8 APIs | Already provided by the crate |
| Feature flag documentation | Prose paragraphs | Markdown table with columns: Flag, Enables, Platform, Runtime | Tables are scannable; prose is not |

**Key insight:** This phase produces zero library code. Everything is documentation. The only "code" is in fenced code blocks inside README.md, compile-tested via doctests.

## Common Pitfalls

### Pitfall 1: README Code Examples Break Silently

**What goes wrong:** Someone changes a field name in a future phase, README examples refer to the old name, nobody notices until a user copies broken code.
**Why it happens:** README is not normally compile-tested.
**How to avoid:** Add `#[doc = include_str!("../README.md")]` with `#[cfg(doctest)]` to lib.rs. Every `cargo test` will catch broken examples.
**Warning signs:** Code examples that use `no_run` or `ignore` annotations unnecessarily.

### Pitfall 2: Adapter Examples That Require Toolkit Dependencies

**What goes wrong:** README shows `use egui::Color32;` in a code block, but native-theme does not depend on egui. The doctest fails because `egui` is not available.
**Why it happens:** Doctest code blocks are compiled against the crate's own dependencies.
**How to avoid:** Mark adapter examples with `ignore` or `no_run` since they depend on external crates. Only the native-theme-only examples (preset loading, merge, from_system) should be compile-tested. Alternatively, show the adapter as pseudocode / annotated mapping tables rather than compilable Rust.
**Warning signs:** CI failures on `cargo test` due to missing toolkit crates.

### Pitfall 3: Incomplete Feature Flag Documentation

**What goes wrong:** User enables `portal` but does not enable `portal-tokio` or `portal-async-io`, gets confusing compile errors.
**Why it happens:** Feature flag interactions are non-obvious. `portal` alone is not useful -- user must pick an async runtime sub-feature.
**How to avoid:** Feature flag table must explicitly document: (1) `portal` is the base feature, (2) user must also enable exactly one of `portal-tokio` or `portal-async-io`, (3) the `kde` feature is Linux-only and sync.
**Warning signs:** Feature flags listed without platform or dependency context.

### Pitfall 4: Missing unwrap() in Workflow Examples

**What goes wrong:** Users copy `let theme = from_system().unwrap();` which panics on unsupported platforms.
**Why it happens:** `.unwrap()` is tempting for brevity.
**How to avoid:** Show the fallback pattern: `from_system().unwrap_or_else(|_| preset("default").unwrap())`. This is the recommended runtime workflow.
**Warning signs:** Any example that calls `.unwrap()` on a function that can return Error::Unsupported.

### Pitfall 5: Forgetting `Option::unwrap_or` for Theme Fields

**What goes wrong:** Users access `theme.light.unwrap().colors.core.accent.unwrap()` which panics on partial themes.
**Why it happens:** All color fields are `Option<Rgba>`, and variants are `Option<ThemeVariant>`.
**How to avoid:** Examples must show the pattern: `let accent = variant.colors.core.accent.unwrap_or(Rgba::rgb(74, 144, 217));` with sensible fallback colors.
**Warning signs:** Chains of `.unwrap()` on Option fields in examples.

## Code Examples

### Example 1: Quick Start (compile-tested)
```rust
// Load a bundled preset
let theme = native_theme::preset("dracula").unwrap();

// Access the dark variant
let dark = theme.dark.as_ref().unwrap();
let accent = dark.colors.core.accent.unwrap();
let bg = dark.colors.core.background.unwrap();

// Convert to f32 for toolkits that use normalized colors
let [r, g, b, a] = accent.to_f32_array();
```

### Example 2: Preset + Merge Workflow (compile-tested)
```rust
use native_theme::{NativeTheme, Rgba, preset, from_toml};

// Start with a community preset
let mut theme = preset("nord").unwrap();

// User overrides from a TOML file (sparse -- only changed fields)
let user_overrides = from_toml(r#"
name = "My Custom Nord"
[light.colors.core]
accent = "#ff6600"
"#).unwrap();

// Merge: user values override preset, preset fills gaps
theme.merge(&user_overrides);
```

### Example 3: Runtime Workflow (compile-tested)
```rust
use native_theme::{from_system, preset};

// Try live OS theme, fall back to bundled preset
let theme = from_system().unwrap_or_else(|_| preset("default").unwrap());
let variant = if true { // replace with your app's dark mode detection
    theme.dark.as_ref().unwrap_or(theme.light.as_ref().unwrap())
} else {
    theme.light.as_ref().unwrap_or(theme.dark.as_ref().unwrap())
};
```

### Example 4: egui Adapter (NOT compile-tested -- requires egui dependency)
```rust,ignore
// Map native-theme to egui Visuals (in YOUR application code)
use egui::{Color32, style::Visuals};
use native_theme::Rgba;

fn rgba_to_color32(c: &Rgba) -> Color32 {
    Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)
}

fn apply_theme(ctx: &egui::Context, theme: &native_theme::NativeTheme) {
    let variant = theme.dark.as_ref().unwrap();
    let c = &variant.colors;

    let mut visuals = Visuals::dark();
    visuals.window_fill = rgba_to_color32(&c.core.background.unwrap());
    visuals.panel_fill = rgba_to_color32(&c.panel.sidebar.unwrap());
    visuals.hyperlink_color = rgba_to_color32(&c.interactive.link.unwrap());
    visuals.error_fg_color = rgba_to_color32(&c.status.danger.unwrap());
    visuals.warn_fg_color = rgba_to_color32(&c.status.warning.unwrap());
    visuals.selection.bg_fill = rgba_to_color32(&c.interactive.selection.unwrap());
    visuals.extreme_bg_color = rgba_to_color32(&c.core.surface.unwrap());
    visuals.faint_bg_color = rgba_to_color32(&c.component.alternate_row.unwrap());

    ctx.set_visuals(visuals);
}
```

### Example 5: iced Adapter (NOT compile-tested -- requires iced dependency)
```rust,ignore
// Map native-theme to iced Theme::custom (in YOUR application code)
use iced::{Color, Theme};
use iced::theme::Palette;

fn rgba_to_iced(c: &native_theme::Rgba) -> Color {
    Color::from_rgb8(c.r, c.g, c.b)
}

fn to_iced_theme(theme: &native_theme::NativeTheme) -> Theme {
    let v = theme.dark.as_ref().unwrap();
    let c = &v.colors;

    let palette = Palette {
        background: rgba_to_iced(&c.core.background.unwrap()),
        text: rgba_to_iced(&c.core.foreground.unwrap()),
        primary: rgba_to_iced(&c.core.accent.unwrap()),
        success: rgba_to_iced(&c.status.success.unwrap()),
        warning: rgba_to_iced(&c.status.warning.unwrap()),
        danger: rgba_to_iced(&c.status.danger.unwrap()),
    };
    Theme::custom("Native".into(), palette)
}
```

### Example 6: slint Adapter (NOT compile-tested -- requires slint dependency)

The slint pattern uses a global singleton defined in .slint markup:

```slint,ignore
// In your .slint file:
export global ThemeBridge {
    in-out property <color> background;
    in-out property <color> foreground;
    in-out property <color> accent;
    in-out property <color> surface;
    in-out property <color> danger;
    in-out property <color> success;
}
```

```rust,ignore
// In your Rust code:
fn apply_theme(app: &App, theme: &native_theme::NativeTheme) {
    let v = theme.light.as_ref().unwrap();
    let c = &v.colors;

    let bridge = app.global::<ThemeBridge>();
    bridge.set_background(to_slint(&c.core.background.unwrap()));
    bridge.set_foreground(to_slint(&c.core.foreground.unwrap()));
    bridge.set_accent(to_slint(&c.core.accent.unwrap()));
    bridge.set_surface(to_slint(&c.core.surface.unwrap()));
    bridge.set_danger(to_slint(&c.status.danger.unwrap()));
    bridge.set_success(to_slint(&c.status.success.unwrap()));
}

fn to_slint(c: &native_theme::Rgba) -> slint::Color {
    slint::Color::from_argb_u8(c.a, c.r, c.g, c.b)
}
```

### Example 7: Feature Flag Table (for README)

```markdown
| Feature | Enables | Platform | Notes |
|---------|---------|----------|-------|
| `kde` | `from_kde()` sync KDE reader | Linux | Parses `~/.config/kdeglobals` |
| `portal` | Base for GNOME portal reader | Linux | Must also enable one of the async runtime features below |
| `portal-tokio` | `from_gnome()` with tokio runtime | Linux | Implies `portal` |
| `portal-async-io` | `from_gnome()` with async-io runtime | Linux | Implies `portal` |
| `windows` | `from_windows()` Windows reader | Windows | Reads UISettings + GetSystemMetrics |
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `doc-comment` crate for README doctests | `#[doc = include_str!("../README.md")]` (built-in) | Rust 1.54 (2021) | No external crate needed for compile-testing README |
| Separate `examples/` dir only | README doctests + examples dir | Ongoing | README examples guaranteed to compile |
| egui `ctx.set_style()` | `ctx.set_visuals()` / `ctx.set_visuals_of()` | egui 0.28+ | More granular theme control |
| iced `palette::Theme::Custom(Box<Custom>)` | `Theme::custom(name, palette)` | iced 0.13 | Simpler custom theme constructor |

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | rustdoc doctests (built-in) + cargo test |
| Config file | none (built into rustdoc) |
| Quick run command | `cargo test --doc` |
| Full suite command | `cargo test` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| DOC-01 | README code examples compile | doctest | `cargo test --doc` | Wave 0: needs `#[cfg(doctest)]` struct in lib.rs |
| DOC-01 | Adapter examples are syntactically correct | manual-only | Visual review (examples marked `ignore` due to external deps) | N/A |
| DOC-01 | Feature flag table matches Cargo.toml | manual-only | Compare README table against Cargo.toml `[features]` | N/A |

### Sampling Rate
- **Per task commit:** `cargo test --doc`
- **Per wave merge:** `cargo test`
- **Phase gate:** Full suite green before verification

### Wave 0 Gaps
- [ ] Add `#[doc = include_str!("../README.md")]` + `#[cfg(doctest)]` struct to `src/lib.rs`
- [ ] Create `README.md` (does not exist yet)

## Open Questions

1. **Should adapter examples use `unwrap()` or show full error handling?**
   - What we know: The "Out of Scope" section says adapters are ~50 lines each. Full match/unwrap_or on every field would bloat examples to 100+ lines.
   - What's unclear: Where to draw the brevity-vs-safety line.
   - Recommendation: Use `unwrap()` in adapter examples (clearly marked as "your application code"), but show `unwrap_or_else` fallback in the workflow examples. Add a note that production code should handle None fields.

2. **Should the README include a TOML format reference section?**
   - What we know: The default.toml is 150 lines and shows every field. Users might want to create their own TOML themes.
   - What's unclear: How much TOML structure to document vs. just pointing to a preset file.
   - Recommendation: Include a condensed annotated TOML snippet showing the key structure (`name`, `[light.colors.core]`, etc.), then link to the full default.toml in the repo for reference.

3. **Should from_gnome() appear in the runtime workflow?**
   - What we know: `from_system()` on Linux/GNOME does NOT call `from_gnome()` -- it returns the Adwaita preset. The doc comment says "For live GNOME portal data, call `from_gnome()` directly."
   - What's unclear: Users might expect `from_system()` to cover GNOME portal.
   - Recommendation: Document this explicitly in the runtime workflow section. Show that `from_gnome().await` is available separately for async GNOME apps.

## Sources

### Primary (HIGH confidence)
- Crate source code (src/lib.rs, src/presets.rs, src/model/, src/color.rs) -- complete API surface
- Cargo.toml -- feature flags, dependencies, edition
- [egui Visuals docs](https://docs.rs/egui/latest/egui/style/struct.Visuals.html) -- 35 fields, Color32 type
- [egui Color32 docs](https://docs.rs/egui/latest/egui/struct.Color32.html) -- from_rgb, from_rgba_unmultiplied constructors
- [iced Palette docs](https://docs.rs/iced/latest/iced/theme/palette/struct.Palette.html) -- 6 fields: background, text, primary, success, warning, danger
- [iced Color docs](https://docs.rs/iced/latest/iced/struct.Color.html) -- from_rgb8(u8, u8, u8) constructor
- [iced Theme docs](https://docs.rs/iced/latest/iced/theme/enum.Theme.html) -- Theme::custom(name, palette)
- [slint Color docs](https://docs.rs/slint/latest/slint/struct.Color.html) -- from_rgb_u8, from_argb_u8 constructors
- [slint Globals docs](https://docs.slint.dev/latest/docs/slint/guide/language/coding/globals/) -- global singleton pattern for Rust interop

### Secondary (MEDIUM confidence)
- [Rust doc include_str pattern](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html) -- #[doc = include_str!] for README doctests
- [egui Widgets struct](https://docs.rs/egui/latest/egui/style/struct.Widgets.html) -- per-state widget visuals

### Tertiary (LOW confidence)
- None -- all findings verified with official docs.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - This phase has zero new dependencies; README + doctests are well-understood Rust patterns
- Architecture: HIGH - README structure follows standard Rust crate conventions; all three toolkit APIs verified via official docs.rs
- Pitfalls: HIGH - Doctest compilation failures and feature flag confusion are well-documented issues in the Rust ecosystem
- Adapter examples: MEDIUM - Toolkit APIs verified via docs.rs but not compile-tested against actual toolkit versions (they will be marked `ignore` in doctests)

**Research date:** 2026-03-07
**Valid until:** 2026-04-07 (30 days -- stable domain, no fast-moving parts)
