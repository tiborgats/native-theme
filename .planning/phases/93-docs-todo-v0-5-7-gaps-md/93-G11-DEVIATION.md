# Phase 93 — G11 Principled Deviation Record

**Created:** 2026-04-20 (Phase 95 Plan 03)
**Accepted:** 2026-04-19T18:28:22Z by `tiborgats` (recorded in `93-VERIFICATION.md` frontmatter `overrides` block)
**Deviation ID:** G11 (see `docs/todo_v0.5.7_gaps.md:546-614` for the definitive detailed record)

## What this file is

A single-source-of-truth annotation for the five Phase 93 PLAN files
that still contain `cargo test --workspace --all-features` as a
must_have truth or a verification command. Those raw claims predate
the discovery of the naga 27.0.3 / codespan-reporting 0.12.0 upstream
incompatibility (surfaced during Phase 93-01 execution, documented
during Phase 93-07).

Rather than rewriting the five PLAN files (which would violate the
user memory rule "Agents must append, never rewrite"), this file
serves as the canonical pointer. Each of the five PLAN files has a
short trailing annotation block directing readers here.

## The affected plans

The table below lists the CANONICAL `must_haves.truths` line per plan
(the headline claim). **ALL `<automated>` and verification-block
`--workspace` occurrences across these five plans are implicitly
superseded by this deviation record** — the table is a headline
index, not a full line-number census. A future reader grepping any
of these files for `--workspace` and finding a hit not listed below
should interpret it as falling under the same implicit supersession.

| Plan | File | Canonical claim location | Resolution |
|------|------|--------------------------|------------|
| 93-01 | `93-01-PLAN.md` | `:23` (must_haves truth) | Superseded by G11; see § below |
| 93-02 | `93-02-PLAN.md` | verification block (no must_haves truth; see `<automated>` blocks in the file) | Superseded by G11 |
| 93-03 | `93-03-PLAN.md` | verification block (no must_haves truth; see `<automated>` blocks in the file) | Superseded by G11 |
| 93-04 | `93-04-PLAN.md` | `:41` (must_haves truth) | Superseded by G11 |
| 93-05 | `93-05-PLAN.md` | `:30` (must_haves truth) | Superseded by G11 |

(Line numbers are snapshot as of 2026-04-20. Line drift is possible
if any of these files is ever edited append-only; the semantic
locations — frontmatter `must_haves.truths` + verification command
blocks — are stable.)

**Implicit-supersession clarifier:** every `cargo test --workspace
--all-features` occurrence in these five plans (in `<automated>`
blocks, in prose verification criteria, in success-criteria
checklists) is implicitly superseded by this deviation. No
per-line annotation is needed inside the plan files beyond the
single trailing annotation block documented in "How this deviation
is annotated in each plan file" below.

## Why the deviation was accepted

**Error signature:**

```
error[E0277]: the trait bound `std::string::String: WriteColor` is not satisfied
   --> ~/.cargo/registry/src/index.crates.io-.../naga-27.0.3/src/front/wgsl/error.rs:113:20
```

Three errors total, all in `naga 27.0.3`, all the same trait-bound mismatch.

**Root cause:** `naga 27.0.3` (pulled transitively by `gpui-component v0.5.1` via `wgpu 27.0.0`) uses `codespan_reporting::term::emit` with `&mut String` as the writer. `codespan-reporting 0.12.0` removed the `impl WriteColor for String` that `0.11.x` provided. `naga 27.0.3` was not yet migrated. The workspace `Cargo.lock:1064-1067` pins `codespan-reporting 0.12.0`, which is correct for every other consumer.

**Options considered (one-line each; full analysis at `docs/todo_v0.5.7_gaps.md:546-614`):**

- **A — `cargo update -p naga --precise 27.0.4`:** Not viable. `naga 27.0.4` does not exist on crates.io (`cargo info naga@27.0.4` → "could not find"). Next release is `28.x`, rejected by `gpui-component v0.5.1`'s `27.x` pin.
- **B — narrow must_have to `-p native-theme`:** Weak framing. This is technically the result, but phrased as "scope narrowing" it reads as hiding the problem rather than honestly documenting the upstream state.
- **C — exclude `connectors/native-theme-gpui` from `[workspace] members`:** Worse. Breaks developer ergonomics (workspace-wide `cargo check` would miss the connector) and propagates the upstream defect into project layout.
- **D — document as principled deviation; align must_have with `./pre-release-check.sh` per-crate posture:** **SELECTED.** The pre-release script is THE release gate for this project since Phase 14-03 (2026-03-09); it runs per-crate tests and treats `native-theme-gpui` as soft via `run_check_soft` (the per-crate test loop lives at `pre-release-check.sh` lines 288-294, immediately below the `# Run all tests` comment at line 287). The `--workspace` claim in the Phase 93 plans was an aspirational generalisation predating the naga discovery.

## The honest acceptance criterion

The current per-crate gate (equivalent to the pre-existing `./pre-release-check.sh` behaviour):

```
cargo test -p native-theme --all-features
cargo test -p native-theme-derive
cargo test -p native-theme-build
cargo test -p native-theme-iced
cargo test -p native-theme-gpui    # soft — may fail on upstream naga issue
```

If all non-gpui crates pass AND the gpui crate either passes OR fails with the known naga signature, the release gate is green.

## Re-evaluation trigger

When `gpui-component` ships a release that either (a) bumps `naga` past 27.0.3 to a `codespan-reporting 0.12.0`-compatible version, or (b) pins `codespan-reporting 0.11.x` in its own `Cargo.toml`, run:

```bash
cargo update -p gpui-component
cargo test --workspace --all-features
```

If both succeed, this deviation is obsolete. Delete this file, delete the trailing annotation blocks from the five Phase 93 PLAN files (or flip them to "resolved"), and restore `--workspace` as the plan must_have in all five files. Track upstream state at:

- https://github.com/gfx-rs/wgpu/issues (naga is part of wgpu)
- https://github.com/zed-industries/zed (gpui-component's upstream)

## Cross-references

- **Definitive detailed record:** `docs/todo_v0.5.7_gaps.md:546-614` (G11 section).
- **Formal acceptance:** `.planning/phases/93-docs-todo-v0-5-7-gaps-md/93-VERIFICATION.md` frontmatter `overrides` block (accepted_by: `tiborgats`, 2026-04-19T18:28:22Z).
- **Original deferred-item note:** `.planning/phases/93-docs-todo-v0-5-7-gaps-md/deferred-items.md` "Resolved during Plan 93-07 (gap closure)" section.
- **Release-maintainer surface:** `RELEASING.md` "Known upstream tool-chain deviations (v0.5.7+)" section (added by Phase 95-03 Task 02).
- **Phase 93-07 plan:** `.planning/phases/93-docs-todo-v0-5-7-gaps-md/93-07-PLAN.md` (the plan that originally landed the deviation in `gaps.md`).
- **Phase 93-07 summary:** `.planning/phases/93-docs-todo-v0-5-7-gaps-md/93-07-SUMMARY.md`.

## Audit invariants

- Zero source code changes to close G11. This is a principled deviation, not a code bug.
- The upstream root cause lives in `gpui-component` / `naga` / `codespan-reporting`; `native-theme` has no path to resolve without forking `gpui-component`.
- The `./pre-release-check.sh` green banner (all 20+ test suites ok, zero ⚠ markers in hard checks, `native-theme v0.5.7 is ready for release.`) IS the release gate.

## How this deviation is annotated in each plan file

Each of `93-01-PLAN.md` through `93-05-PLAN.md` has a single APPEND-ONLY
annotation block below its closing `</output>` tag. The block is labelled
"G11 annotation added 2026-04-20 (Phase 95 Plan 03)" and points readers
here. Nothing above `</output>` in any of the five files is modified.
