# Phase 9: Cargo Workspace - Context

**Gathered:** 2026-03-08
**Status:** Ready for planning

<domain>
## Phase Boundary

Restructure the repo into a Cargo workspace so connector crates can be developed alongside the core crate. Core crate moves into a subdirectory, workspace root ties members together. No new functionality — purely structural.

</domain>

<decisions>
## Implementation Decisions

### Git history
- Use `git mv` to move files into the core crate subdirectory (preserve blame/log tracking)
- Single atomic commit for the entire restructuring (move + workspace Cargo.toml + adjustments)
- Keep at repo root: .git, .gitignore, LICENSE files, README, .planning/, Cargo.lock
- Everything else moves into the core crate subdirectory

### Connector crate stubs
- Create placeholder crates for gpui and iced connectors in this phase (not deferred to Phase 14)
- Stubs include Cargo.toml with a workspace dependency on the core crate, plus an empty lib.rs
- Use workspace dependency inheritance — define shared deps at workspace level, connectors use `{ workspace = true }`

### Claude's Discretion
- Cargo.lock placement (workspace convention)
- Crate naming for connectors
- Workspace-level metadata (edition, resolver)
- Directory naming for the core crate subdirectory

</decisions>

<specifics>
## Specific Ideas

No specific requirements — open to standard approaches.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 09-cargo-workspace*
*Context gathered: 2026-03-08*
