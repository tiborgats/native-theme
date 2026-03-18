# Phase 27: Animation Data Model and Breaking Changes - Context

**Gathered:** 2026-03-18
**Status:** Ready for planning

<domain>
## Phase Boundary

Define AnimatedIcon/TransformAnimation/Repeat types, the loading_indicator() dispatch function, first_frame() helper, and remove StatusLoading from IconRole. This is the type foundation — loading_indicator() returns None until Phase 28 wires up actual frame data.

</domain>

<decisions>
## Implementation Decisions

### StatusLoading removal
- Remove StatusLoading in the SAME release (v0.4.0) that adds AnimatedIcon and loading_indicator()
- Replace with StatusBusy — a static icon role for apps that want a non-animated busy/progress indicator
- StatusBusy maps to the same underlying icons (process-working, progress_activity, loader) but is semantically "static busy indicator" not "animated loading"
- The rename is a breaking change but pre-1.0, so acceptable on minor version bump

### loading_indicator dispatch
- loading_indicator() should be a single dispatch function taking &str for icon set name, consistent with load_icon() pattern
- Returns None when the requested icon set's feature flag is disabled (matches load_icon() behavior — user reconsidered compile error approach in favor of consistency)
- Icon set naming and auto-detect behavior are Claude's discretion — pick what's cleanest given the existing load_icon()/from_system() precedents

### Claude's Discretion
- AnimatedIcon enum shape: exact field names, derive traits (Debug, Clone, PartialEq?), pub vs pub(crate) boundaries
- Icon set naming for loading_indicator() (reuse load_icon() names vs platform names)
- Whether to include a from_system()-style auto-detect for loading_indicator()
- CHANGELOG/migration framing for StatusLoading → StatusBusy + loading_indicator()

</decisions>

<specifics>
## Specific Ideas

- StatusBusy name chosen to match freedesktop convention (process-working) and convey "system is busy"
- first_frame() on AnimatedIcon provides the static fallback path for reduced-motion or simple use cases
- The design doc (docs/animated-icons.md) has the full type definitions — use as reference but Claude can adjust details

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 27-animation-data-model-and-breaking-changes*
*Context gathered: 2026-03-18*
