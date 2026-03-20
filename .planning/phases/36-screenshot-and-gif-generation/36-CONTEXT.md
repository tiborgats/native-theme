# Phase 36: Screenshot and GIF Generation - Context

**Gathered:** 2026-03-20
**Status:** Ready for planning

<domain>
## Phase Boundary

Create visual assets (showcase screenshots, spinner GIFs) for documentation and automation tooling to regenerate them. No new features or showcase changes — only capture what exists.

</domain>

<decisions>
## Implementation Decisions

### Screenshot subjects
- Capture the first tab (Colors) only
- One screenshot per theme, per variant (light and dark are separate images)
- Same icon theme across all screenshots for consistency
- Both gpui and iced showcases captured
- Individual images (not composited grids) — arranged in README markdown

### GIF content
- All bundled spinners get their own GIF
- Show spinners in showcase context (inside the Icons tab), not isolated
- One full animation cycle per GIF, looping infinitely
- Both gpui and iced toolkits get matching GIFs

### Output specs
- Screenshots in PNG format
- GIFs: format at Claude's discretion (GIF vs APNG — pick best quality/compatibility tradeoff)
- Assets stored in `docs/assets/` (not root `assets/` which is for application graphics)
- Native window size — no forced resize
- File naming at Claude's discretion

### Automation tooling
- Fully scripted — one command regenerates all screenshots and GIFs
- Automated theme switching — script cycles through all themes and captures each
- Capture approach at Claude's discretion (external tool vs built-in rendering)
- Scripts live in `scripts/` directory at repo root

### Claude's Discretion
- GIF format choice (GIF vs APNG)
- Screen capture tool/approach
- File naming convention
- Script language and structure

</decisions>

<specifics>
## Specific Ideas

- `docs/assets/` is for documentation graphics; root `assets/` is for application-level graphics (icons, etc.)
- User wants the visual assets to demonstrate the toolkit-agnostic nature by showing both gpui and iced

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 36-screenshot-and-gif-generation*
*Context gathered: 2026-03-20*
