# v0.7.1 → 0.8.0 — Model completeness: Ledger

The working record of [the plan](todo_v0.7.1_model-completeness-plan.md) ("the plan"). One writer: the orchestrating
session; lanes, verifiers, tie-breakers, implementers and reviewers report to it. `scripts/check_ledger.py` checks
the triage (every item has a destination) and, from M4, `--final` (the plan's M4 gate).

## Milestones

| Milestone | Status | Closed at | Notes |
|---|---|---|---|
| D5 `v0.7.1-rc1` merged to `main` | in progress | | PR #6; the maintainer asked for the merge on 2026-10-29 |
| M0 setup | in progress | | `v0.8.0-rc1` and `v0.8.0-tools` cut from fc6109d1 |
| Spikes M · G · K · I · W · A | not started | | |
| M1a slice core | not started | | |
| M1b slice complete | not started | | |
| D6 checkpoint | not started | | |
| M2 model core | not started | | |
| M3 units | not started | | per unit below |
| M4 facts gate complete | not started | | |
| M5 showcases | not started | | |
| M6 release 0.8.0 | not started | | |

## Units

Per unit and platform: facts (`—` none, `L` lane writing, `F` written), verification (`V` agreed, `T` at the
tie-breaker, `A` audited), presets (`P`), gate (`G`), connectors (`C`).

| Unit | adwaita | kde-breeze | macos-monterey | macos-tahoe | ios-13 | ios-26 | windows-11 | material-3 | Connectors |
|---|---|---|---|---|---|---|---|---|---|
| slice (`button`, colour roles, slice tokens) | — | — | — | — | — | — | — | — | — |

## Module inventory

Every module, integration test and `lib.rs` item of `native-theme`, one fate each (the plan, *The module
inventory*): ported beside, kept in place, split, deleted, unlinked; with its milestone. Written before the switch.

| Item | Fate | Milestone | Notes |
|---|---|---|---|

## `PLAN_EXCLUDED`

The expected list before the switch is the plan's (*What cannot build meanwhile*). Current list: empty.

| Step | Until | Since | Reason |
|---|---|---|---|

## Full runs

Each milestone's full `scripts/check_release.sh` run: commit, result, every warning read.

| Commit | Milestone | Result | Warnings |
|---|---|---|---|

## Spec-change queue

Class 1 data corrections, class 2 model changes (an S22 line and one independent check of the batch), class 3
rationale changes (the maintainer). Each with its reporter, status and the commit that applied it.

| # | Class | Document and place | Change | Reported by | Status |
|---|---|---|---|---|---|

## Unmeasured register

Values that could not be measured: the key and combination, the attempts and their evidence, the options; each is
the maintainer's decision.

| Key and combination | Attempts and evidence | Options | Decision |
|---|---|---|---|

## Verification rates

Per lane and unit: the verifier's agreement rate and the audit sample's rate of agreed but wrong facts.

| Unit | Platform | Facts | Agreed | Tie-broken | Audit sample | Agreed but wrong |
|---|---|---|---|---|---|---|

## Spikes and drift runs

| Spike or run | Result | Run URL | Notes |
|---|---|---|---|
| M — macOS probe's button pass | | | |
| G — GNOME harness and libadwaita vector generator | | | |
| K — KDE container and accent script | | | |
| I — iOS simulator host | | | |
| W — microsoft-ui-xaml coverage and WinUI host | | | |
| A — Android host | | | |

## Evidence size

| Date | `docs/evidence/` size | Change | Notes |
|---|---|---|---|

## Tokens

The sum of the agents' reported usage per milestone, against D7's cap (+15,000k for M0–M1b).

| Milestone | Tokens | Cap | Notes |
|---|---|---|---|

## Triage

Every open item of `docs/todo.md`, the catalogue's *Open items*, ROADMAP's *Later — the connectors* and *Also in the
0.7.x releases*, and the plans `docs/todo_gpui-full-theme.md` and `docs/todo_iced-full-theme-geometry.md`, with its
destination: a named milestone, unit or lane; `settled by the model`; `after 0.8.0` with its reason; or `not this
plan's work`.
