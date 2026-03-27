# Phase 46: OS Reader Extensions - Context

**Gathered:** 2026-03-27
**Status:** Ready for planning

<domain>
## Phase Boundary

Extend all four platform readers (macOS, Windows, KDE, GNOME) to populate per-widget fields, text scale, per-widget fonts, and accessibility flags in the new ThemeVariant structure. The exact fields per reader are specified in `docs/todo_v0.5.1_resolution.md` §3.1-3.4. Reader output must pass through resolve() then validate() to produce a valid ResolvedTheme.

</domain>

<decisions>
## Implementation Decisions

### Inactive variant strategy
- Platform TOMLs have 4 sections: `live-light`, `live-dark`, `preset-light`, `preset-dark`
- **Live variants** (`live-light`, `live-dark`): contain only what the OS reader CANNOT provide (design constants, non-⚙ values). Overlaid on top of OS reader output when the OS is in that mode. These are minimal TOMLs.
- **Preset variants** (`preset-light`, `preset-dark`): contain full measured/predetermined color values for the variant that ISN'T currently active on the OS. Still uses OS reader output for variant-agnostic settings (fonts, DPI, icon sizes, accessibility flags). Only colors come from the TOML.
- `from_system()` returns only the active variant (live). Apps that want the other variant load the preset variant and merge it with variant-agnostic OS reader data.
- Example: Windows in dark mode → dark variant uses `live-dark` overlay on OS reader output. If app wants light, it uses `preset-light` + OS fonts/DPI/accessibility.

### Cross-platform preset restrictions
- Platform-specific presets (macos-sonoma, kde-breeze, windows-11, adwaita) are restricted to their native platform — they depend on OS-specific icons and reader output
- Community presets (catppuccin, nord, dracula, etc.) work on all platforms — they bundle everything needed including icon set reference

### Claude's Discretion
- GNOME reader refactoring approach (currently embeds Adwaita preset — docs say it should only read OS values; how much of this change belongs in Phase 46 vs Phase 47)
- Platform TOML slimming timing (removing ⚙ fields from platform TOMLs after readers provide them — Phase 46 or Phase 47)
- Exact TOML section naming convention for the 4-variant structure

</decisions>

<specifics>
## Specific Ideas

- The docs (`todo_v0.5.1_resolution.md` §3.1-3.4) specify exactly which OS APIs map to which ThemeVariant fields for all four platforms
- "preset" name chosen because the non-live variant's colors come from the preset definition, not the OS — even though it still uses live fonts/DPI/accessibility
- Platform presets can't be loaded cross-platform because "there are missing icons that we cannot bundle" (e.g., macOS theme on Windows)

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 46-os-reader-extensions*
*Context gathered: 2026-03-27*
