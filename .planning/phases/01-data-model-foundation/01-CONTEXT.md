# Phase 1: Data Model Foundation - Context

**Gathered:** 2026-03-07
**Status:** Ready for planning

<domain>
## Phase Boundary

Define the complete type system for native-theme: Rgba color type, all theme structs (NativeTheme, ThemeVariant, ThemeColors, ThemeFonts, ThemeGeometry, ThemeSpacing), a declarative merge macro, error types, TOML serde, and validation tests. This is the foundation every subsequent phase builds on — no platform readers, no presets, no runtime detection.

</domain>

<decisions>
## Implementation Decisions

### Color representation
- Claude's Discretion: internal representation (f32 vs u8) — pick what works best for the downstream toolkit ecosystem (egui, iced, slint all use f32)
- Claude's Discretion: alpha default behavior when omitted in hex strings
- Claude's Discretion: convenience methods (to_array, to_tuple, etc.) — pick the right level of utility
- Claude's Discretion: color space handling — pragmatic approach given OS API realities

### Struct organization
- ThemeColors uses **nested sub-structs grouped by semantic role** (e.g., ButtonColors, WindowColors, InputColors) — not a flat 36-field struct
- Claude's Discretion: whether Option wraps each field, each group, or both — pick what's most ergonomic for the merge macro
- Claude's Discretion: exact color role groupings — design based on what OS theme APIs actually provide
- Claude's Discretion: whether ThemeFonts/ThemeGeometry/ThemeSpacing also nest or stay flat — decide based on field count

### Merge behavior
- Claude's Discretion: ownership semantics (consume vs borrow) — pick the most idiomatic Rust approach
- Claude's Discretion: pairwise vs variadic — pick the simpler approach that covers real use cases
- Claude's Discretion: deep merge vs shallow replace for nested sub-structs — pick what's most useful for theme layering
- Claude's Discretion: whether the merge macro is public or internal — decide based on extensibility needs

### Serialization style
- Claude's Discretion: TOML variant structure (top-level vs wrapper) — pick what's cleanest for human editing
- Claude's Discretion: whether to include metadata section — decide what supports the preset system best
- **Human-editability is a primary concern** — theme TOML files should be easy to hand-edit, with readable structure and logical field ordering
- Claude's Discretion: field ordering strategy — pick what best serves human-editability

</decisions>

<specifics>
## Specific Ideas

- User explicitly prioritizes human-editability of TOML files — this should influence serialization choices (readable section names, logical ordering, skip_serializing_if for None fields)
- Nested color grouping was specifically chosen over flat — the struct hierarchy should mirror how TOML sections naturally nest (e.g., `[light.colors.button]`)

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 01-data-model-foundation*
*Context gathered: 2026-03-07*
