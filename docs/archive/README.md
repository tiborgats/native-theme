# Archive

Design documents and development history from completed milestones. Nothing here
describes the current API — see the crate docs, `CHANGELOG.md`, and `ROADMAP.md`
for that. This directory is the record of how the crate got to where it is, and
why each fork in the road was taken the way it was.

## Consolidated planning history

Four cross-linked documents covering v0.1 through v0.5.7. They consolidate the
per-phase planning records that used to live outside the source tree, in
`.planning/` and `.paul/`, before those directories were removed.

| Document | What it holds |
|----------|---------------|
| [v0.5.7_planning-decision-log.md](v0.5.7_planning-decision-log.md) | Every recorded implementation decision and its reasoning, grouped by milestone, plus the curated project-level decision tables and standing constraints |
| [v0.5.7_planning-milestone-history.md](v0.5.7_planning-milestone-history.md) | What shipped in each milestone, milestone audit scores, and the full phase index |
| [v0.5.7_planning-deviations-and-deferrals.md](v0.5.7_planning-deviations-and-deferrals.md) | Where execution diverged from plan, what was deferred, accepted debt with current status, and principled deviations with re-evaluation triggers |
| [v0.5.7_planning-research-findings.md](v0.5.7_planning-research-findings.md) | Dependency choices and rejected alternatives, what not to hand-roll, platform pitfalls, and open questions with their resolutions |

Start with the decision log if you are asking "why is it built this way", and
the research findings if you are asking "why this crate and not that one".

## Milestone design documents

The specifications and critiques that drove each milestone, in version order.
These are the inputs; the consolidated documents above are the record of what
actually happened when they were implemented.

| Milestone | Documents |
|-----------|-----------|
| v0.2.0 | `v0.2.0-stale-publish-plan.md` |
| v0.3.x | `v0.3.0-extra-icons.md`, `v0.3.1-feature-simplification.md`, `v0.3.2-quality-improvements.md`, `v0.3.3-custom-icon-roles.md`, `v0.3.3-icon-gaps-and-fallback-removal.md` |
| v0.4.x | `v0.4.0-animated-icons.md`, `v0.4.0-finalize.md`, `v0.4.1.md` |
| v0.5.0 | `v0.5.0_resolution.md`, `v0.5.0_inheritance-rules.md`, `v0.5.0_theme-variant.md` |
| v0.5.1 | `v0.5.1_native-theme-API.md`, `v0.5.1_native-theme-build-API.md`, `v0.5.1_gpui-connector-API.md`, `v0.5.1_iced-connector-API.md` |
| v0.5.2 | `v0.5.2_native-theme_api.md`, `v0.5.2_native-theme_bugfix.md`, `v0.5.2_native-theme-build_api.md`, `v0.5.2_native-theme-gpui_api.md`, `v0.5.2_native-theme-iced_api.md` |
| v0.5.3 | `v0.5.3_native-theme.md`, `v0.5.3_native-theme-build.md`, `v0.5.3_native-theme-gpui.md`, `v0.5.3_native-theme-iced.md` |
| v0.5.4 | `v0.5.4_native-theme.md`, `v0.5.4_native-theme-build.md`, `v0.5.4_native-theme-gpui.md`, `v0.5.4_native-theme-iced.md` |
| v0.5.5 | `v0.5.5.md`, `v0.5.5_size-fix.md`, `v0.5.5_pt-px.md` |
| v0.5.6 | `v0.5.6_break-up-lib-rs.md`, `v0.5.6_gtk-icon-theme.md`, `v0.5.6_platform-reader-testing.md`, `v0.5.6_runtime-theme-change.md`, `v0.5.6_validate-derive-macro.md` |
| v0.5.7 | `v0.5.7_native-theme-api.md`, `v0.5.7_native-theme-api-2.md`, `v0.5.7_gaps.md`, `v0.5.7_icon-theme.md` |

## A note on accuracy

File paths and line numbers throughout the archive were correct when written and
many have since drifted. Treat them as historical pointers. Where an archived
claim was later found to be wrong, the correction was appended in place rather
than overwriting the original — so a section may state something and then
contradict it further down. The later text wins.

The canonical, current sources of truth are `docs/platform-facts.md`,
`docs/property-registry.toml`, and `docs/inheritance-rules.toml`.
