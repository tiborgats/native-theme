# Phase 1: v0.3.2 quality improvements - Context

**Gathered:** 2026-03-14
**Status:** Ready for planning

<domain>
## Phase Boundary

Code quality, performance, and API hygiene fixes to existing v0.3.1 codebase. No new features — 7 specific improvements. All backward-compatible, no breaking changes.

Spec: `docs/v0.3.2-quality-improvements.md`

</domain>

<decisions>
## Implementation Decisions

### Caching (Issue 1)
- Use `std::sync::OnceLock` for `system_icon_theme()` and `system_is_dark()`
- Do NOT cache `from_system()` or `from_linux()` — users may want to re-read after portal signals
- Trade-off accepted: theme changes after process start won't be picked up

### pick_variant (Issue 2)
- Add `NativeTheme::pick_variant(&self, is_dark: bool) -> Option<&ThemeVariant>` to core
- Keep existing free functions in connectors as deprecated thin wrappers for one release
- Remove deprecated wrappers in v0.4

### colorize_svg docs (Issue 3)
- Add doc comments explaining when to use `to_svg_handle_colored` vs `to_svg_handle`
- Rename internal `colorize_svg` to `colorize_monochrome_svg`
- Do NOT add XML parser complexity

### Dead wrappers (Issue 4)
- Remove `lighten`, `darken`, and `with_alpha` from `derive.rs`
- Inline trait calls directly in `active_color`

### to_theme round-trip (Issue 5)
- Check if gpui-component exposes non-color field access
- If not, keep current pattern but improve the explanatory comment
- If yes, extract specific fields manually instead of overwrite-restore cycle

### #[must_use] (Issue 6)
- Add to all listed public functions with descriptive messages
- Also consider adding to `NativeTheme` and `IconData` types

### pre-release-check.sh (Issue 7)
- Replace python3 with jq for cargo metadata parsing
- Add jq availability check with pure-bash grep/sed fallback

### Claude's Discretion
- Exact `#[must_use]` message wording
- Whether `with_alpha` has external callers (remove if not)
- Comment text for the to_theme round-trip explanation

</decisions>

<specifics>
## Specific Ideas

Full implementation details with code snippets in `docs/v0.3.2-quality-improvements.md`. Follow the spec directly.

Implementation order from spec:
1. Cache system_icon_theme() / system_is_dark() (High priority)
2. Add #[must_use] annotations (Trivial)
3. Add NativeTheme::pick_variant() (Small)
4. Document colorize_svg limitations (Trivial)
5. Remove dead derive.rs wrappers (Trivial)
6. Fix to_theme color round-trip (Small)
7. Replace python3 with jq in script (Trivial)

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 01-v0-3-2-quality-improvements*
*Context gathered: 2026-03-14*
