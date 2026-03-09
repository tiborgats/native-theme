# Phase 16: Icon Data Model - Context

**Gathered:** 2026-03-09
**Status:** Ready for planning

<domain>
## Phase Boundary

Define semantic icon role types, icon data containers, platform-specific name mapping tables, and TOML integration for icon theme selection. Pure data model — no I/O, no rendering, no platform API calls.

</domain>

<decisions>
## Implementation Decisions

### IconRole design
- Flat enum with prefixed naming: `IconRole::DialogError`, `IconRole::ActionCopy`, `IconRole::WindowClose`, etc.
- No nested sub-enums or category wrapper — the prefix IS the category
- Closed enum with `#[non_exhaustive]` — exactly 42 predefined variants, no `Custom(String)`
- Custom/app-specific icons are out of scope — those are app assets, not theme icons
- `#[non_exhaustive]` handles future additions in minor versions without breaking downstream

### IconData shape
- Claude's Discretion (see below)

### Name mapping architecture
- Claude's Discretion (see below)

### TOML integration
- Claude's Discretion (see below)

### Claude's Discretion
- **IconRole derives:** Choose appropriate derive set (serde, strum, etc.) based on what's actually needed
- **IconRole variant list:** Curate the 42 roles based on cross-platform coverage — icons that exist across all major sets get priority
- **IconData ownership:** Owned Vec<u8> vs Cow<[u8]> — decide based on how bundled static SVGs vs loaded icons interact
- **IconData color/tint:** Whether to include tint metadata or keep it the connector's responsibility
- **IconData size hint:** Whether SVG variant carries a nominal size or size is purely a load parameter
- **Mapping strategy:** Static match tables vs phf hash maps — pick based on performance/simplicity
- **Unmapped roles:** Whether icon_name() returns Option<&str> or always returns something with best-effort approximations
- **IconSet extensibility:** Fixed 5 sets with #[non_exhaustive] vs allowing custom sets
- **system_icon_set() placement:** This phase (simple cfg) vs Phase 21 (with runtime checks)
- **TOML field type:** String vs typed IconSet enum for icon_theme field
- **TOML field placement:** Per-variant (light/dark each have icon_theme) vs top-level theme-wide
- **Preset defaults:** Whether existing presets ship with icon_theme values or leave them None

</decisions>

<specifics>
## Specific Ideas

No specific requirements — open to standard approaches. The roadmap success criteria provide concrete test cases:
- `icon_name(IconSet::SfSymbols, IconRole::ActionCopy)` returns `"doc.on.doc"`
- `system_icon_set()` returns `IconSet::SfSymbols` on macOS, `IconSet::SegoeIcons` on Windows, `IconSet::Freedesktop` on Linux
- Loading preset TOML with `icon_theme = "material"` populates `theme.light.icon_theme`

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 16-icon-data-model*
*Context gathered: 2026-03-09*
