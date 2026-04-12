# Phase 69: Resolver-Level button_order Unlock - Context

**Gathered:** 2026-04-12
**Status:** Ready for planning

<domain>
## Phase Boundary

Remove reader-side `button_order` hardcodes from `from_kde` / `from_macos` so pre-resolve `ThemeMode` contains `button_order = None`. Move platform dispatch from `resolve_safety_nets` into `resolve_platform_defaults` (Option C from doc 2 A4). The `resolve()` "no OS detection" rustdoc becomes literally true.

Requirements: BUG-03, BUG-04, BUG-05. Scope limited to KDE and macOS readers — GNOME and Windows have the same pattern but are not in this phase.

</domain>

<decisions>
## Implementation Decisions

### Dispatch strategy
- Option C (recommended, accepted): move `button_order` fill from `resolve_safety_nets` (`inheritance.rs:164-167`) to `resolve_platform_defaults` (`mod.rs:52`)
- Semantic shift: bare `resolve()` no longer fills `button_order`; callers use `resolve_all()` or `into_resolved()` for platform defaults
- `platform_button_order()` helper (`inheritance.rs:98`) stays; only its call site moves

### Preset TOML cleanup — two-tier system
- **Regular presets** (`kde-breeze.toml`, `windows-11.toml`, `material.toml`, `macos-sonoma.toml`): complete theme specifications — KEEP `button_order`
- **Live presets** (`*-live.toml`): pair with platform readers — STRIP `button_order` and other reader-provided fields (they should already be absent; verify and fix)
- When stripping `button_order` from live TOMLs, add a brief TOML comment (e.g., `# button_order: provided by platform reader`)
- Create `native-theme/src/presets/README.md`: full preset guide explaining both tiers, each preset file with a one-line purpose, and the complete list of reader-provided fields

### Rustdoc updates
- Update `resolve_platform_defaults()` doc (`mod.rs:46-51`) to mention `button_order` alongside `icon_theme`
- Update `resolve_safety_nets` doc after removing the `button_order` branch — accurately describe remaining safety nets
- Expand module-level comment (`mod.rs:1`) to describe the resolve / resolve_platform_defaults separation (pipeline overview)

### Claude's Discretion
- Rustdoc depth beyond the three locked changes above
- Whether to add a semantic shift note on `resolve()` doc
- Whether to add migration notes in rustdoc
- Test coverage depth beyond the two required assertions (SC1, SC2)

</decisions>

<specifics>
## Specific Ideas

- `macos.rs:805` already has a test `build_theme_dialog_button_order_not_set_by_build` asserting the goal state — may already be partially implemented or currently failing
- Source doc (A4) notes "D5 is independent of A4" — KDE reader cleanup and resolver move can be shipped independently
- Source doc estimates Option C as "~5 lines of code movement"

</specifics>

<deferred>
## Deferred Ideas

- GNOME (`gnome/mod.rs:270`) and Windows (`windows.rs:517`) button_order hardcodes — same pattern as KDE/macOS, not in Phase 69 requirements
- `button_order` type migration (string vs enum) — future consideration

</deferred>

---

*Phase: 69-resolver-button-order-unlock*
*Context gathered: 2026-04-12*
