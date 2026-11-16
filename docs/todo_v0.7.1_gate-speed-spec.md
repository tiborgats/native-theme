# v0.7.1: the gate's speed (`--quick`, nextest, the step cache) and CI's MSRV checks — specification

- Rationale: [`todo_v0.7.1_batch-speed-rationale.md`](todo_v0.7.1_batch-speed-rationale.md),
  decision S1, approved by the maintainer on 2026-11-15. This document says what
  changes, where, with which tests and gates. It decides what S1 left to it (what
  `--quick` runs on 0.7.x) and the points listed under *Decisions taken while
  specifying*; what needed the maintainer is under *Open points*, the last section (O1 to
  O3), and decision 4 (y), the reviewed registry list, all answered by the maintainer on
  2026-11-15 (*Open points*).
- Written: 2026-11-15, against `v0.7.1-rc2` at `0fa28027` (worktree
  `.claude/worktrees/v071`); revised the same day after its first-pass reviews (the
  scope of `--quick` is the per-step key of decision 2 (f), which replaced the recorded
  tree and `scripts/change_scope.sh` of the first version). Every `file:line` is
  `0fa28027`'s unless it names another commit or version; a registry source is cited by its
  folder under `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`. The batch's
  other documents are untracked and were being edited while this was written, so they
  are cited by heading or quoted text.
- Abbreviations: `CR` is `scripts/check_release.sh` at `0fa28027`; `CR8` is
  `v0.8.0-rc1:scripts/check_release.sh`; `SC8` is `v0.8.0-rc1:scripts/step_cache.sh`
  (846 lines); `SC8a` is `818f0eab:scripts/step_cache.sh` (216 lines, the cache as
  first written, before the shared parts); `ci.yml` is `.github/workflows/ci.yml`.
- Status: **settled 2026-11-15.** The maintainer answered the open points that day (O1a
  no, O1b no, O2 (e), O3's keyed option, decision 4 (y)) and approved the 0.7.1 plans as
  written: "When the plan that goes in the 0.7.1 version is ready, without errors, start
  implementing it."
- Place: v0.7.1's first unit, **GS**, on `v0.7.1-rc2` in the core worktree, after the
  GPUI plan's P1 and P0 and before any unit of the GPUI or the KDE step 1 plan runs a
  gate (`todo_v0.7.1_gpui-kit-0.9.1-plan.md`, header and its rows P0, GS, E1;
  `todo_v0.7.1_kde-open-items-step1-plan.md`, header). Its plan is
  [`todo_v0.7.1_gate-speed-plan.md`](todo_v0.7.1_gate-speed-plan.md).

## How to read it

| Part | Answers | Files it owns |
|---|---|---|
| M — CI's MSRV checks | the six checks of `ci.yml`'s `msrv` job, the check that they are its, where 1.88.0 builds (O2) | `scripts/msrv_checks.sh` (new), `scripts/check_release.sh` |
| A — `--quick` | what it runs, how it knows a step's inputs are unchanged | `scripts/check_release.sh` |
| B — nextest | which steps, doctests, one process per test | `scripts/check_release.sh` |
| C — the step cache | the two keys, which steps each keys, the records, the self-test | `scripts/step_cache.sh` (new), `scripts/step_cache_includes.py` (new), `scripts/check_release.sh` |
| D — what comes from `v0.8.0-rc1`, and the texts | ported, adapted, left out; header, usage, `scripts/README.md`, `CONTRIBUTING.md`, `CHANGELOG.md` | those files |
| E — tests of the change | the self-tests and the mechanical checks | none committed but the self-tests |
| F — commits, gates, review | how GS lands | — |
| *Under another outcome* | not taken (2026-11-15): what would have changed under the other answers to O1 to O3 and decision 4, kept as the record of the options weighed | — |

**The full run.** S1 says it is not changed. GS adds eight steps that the full run
would run too only on the maintainer's approval (*Open points*, O1): the six MSRV
checks and the check that they are `ci.yml`'s (Part M, seven steps), and the step
cache's self-test (C.6). No existing step of the full run is removed, skipped or changed
in its command either way. The maintainer answered O1a and O1b no on 2026-11-15, so GS
runs the eight in `--commit` and `--quick` only and the full run stays exactly as it is
(E.3 (c) proves it); the full run then checks less than `--commit` does, which O1 named.
The MSRV steps themselves, in `--commit` and `--quick`, went beyond S1 and waited for O3;
where 1.88.0 builds, for O2; both answered the same day.
Everything else in this document applies to `--commit` and `--quick` only.

**The outcome this document is written for**, the maintainer's answers of 2026-11-15:
O3's keyed option (the six MSRV checks in `--commit` and `--quick`, the two `+1.88.0`
ones keyed in `--quick`), O2 (e) (the `+1.88.0` checks' folder deleted after every run),
O1a and O1b no (the full run unchanged), and decision 4 (y). *Under another outcome*,
after Part F, lists for each other answer the sections, counts, texts and gates known to
change; those outcomes were not taken (2026-11-15), and the section stays as the record
of the options weighed.

## What the gate is today, and what 0.8.0 built

- `CR` has two modes, the full run and `--commit` (`CR:26-37`, `:46-71`). `--commit`
  skips the release-tag check, the gpui connector's tests on GPUI Fast (two legs,
  `:710-716`, `:849-855`), the ignored stem-index test (`:691-700`) and the release-only
  sections (docs.rs `:1074-1080`, packaging `:1095-1126`, semver `:1145-1164`, outdated
  `:1242-1285`, the two stamps `:1298-1350`), each through `print_skipped`
  (`:150-154`). Every test step runs `cargo test` (`:667-772`); there is no cache and no
  MSRV check (`grep -i 'msrv\|rust-version\|nextest' CR` finds no line).
- Measured (rationale §1): `--commit` 5 to 12 minutes, 29 cold (before the 16-job cap;
  under it not measured); the full run about 23 minutes.
- CI (`ci.yml:6-10`) runs on every pull request and every push to `main`; the batch's
  branches have pull requests (v0.7.1's is #10), so every push of one runs it.
- On `v0.8.0-rc1`: `--quick` (`350ee147`) is a fixed list for the core alone:
  formatting, the panic scan, clippy and the strict lints on `native-theme` and
  `native-theme-derive`, their tests, the docs of `native-theme`, CI's MSRV checks and
  0.8.0's data checks (`CR8:1156-1221`). nextest (`24711322`) runs the test steps of
  `--commit` and `--quick` (`CR8:569-661`). The step cache (`818f0eab`) skips a step
  whose key is unchanged since it last passed (`CR8:663-821`, `:927-1108`; `SC8`).
  The full run uses neither (`CR8:927-931`).

0.7.x's units mostly change a connector, not the core (rationale S1), so 0.8.0's
fixed core list does not fit: `--quick` here is defined by what each step reads.

## Decisions taken while specifying

Each states the options, what each gives and costs, and the choice.

1. **What `--quick` runs: `--commit`'s steps, but a test, rustdoc or `+1.88.0` MSRV step
   is left out where its key equals the key of its last pass** (the strict key where the strict
   cache keys the step, the `--quick` key of decision 2 otherwise).
   - (a) 0.8.0's form, a fixed list: a second step list beside `--commit`'s, which
     drifts from it, and a list per unit would be needed anyway.
   - (b) S1's floor alone, hand-listed: leaves out the cross-target legs, the feature
     combinations, the examples build, the MSRV checks and the audit. The batch's plans
     push a unit once its gate passes (`todo_v0.7.x_kde-open-items-steps4-5-spec.md`
     C.1, *Pushes*: "a commit is pushed only at a unit's end, once its gate passes"), and CI
     runs those checks on the push (`ci.yml`'s `clippy` job's cross-target steps, the
     `features`, `msrv` and `audit` jobs), so each one left out is a red CI run and an
     e-mail to the maintainer when it fails.
   - (c) chosen: every `--commit` step runs, but a test step (`run_tests`,
     `run_tests_min`: doctests and the runner-environment legs among them) or a
     `cargo doc` step is skipped where its inputs are unchanged since it last passed.
     The strict cache skips its 34 steps as in `--commit` (C.3), 24 of them compile-only
     (the check, clippy and strict-lint steps of the crates whose strict reach it
     accepts). Every other compile-only step keeps running: Cargo's fingerprints, which
     follow every file rustc read (its dep-info), make them near no-ops for an untouched
     crate (not measured), and they catch every compile-level effect of a change,
     including any a key misses. That backstop holds for the gpui, iced and egui
     connectors (their check, clippy and strict-lint steps, whose strict reaches the
     probes refuse, C.3, and the examples build), the feature combinations, the cross-target steps and the four
     `+1.95.0` MSRV checks. The four `+1.95.0` MSRV checks are such steps: they build in the
     shared build folder (M.3), where fingerprints make an unchanged crate cheap. The two
     `+1.88.0` checks are the exception: under O2 (e) they build in a fresh folder every
     run (14.0 s + 5.2 s measured, O2), so no fingerprint helps them, and O3's keyed
     option keys those two in `--quick` (A.2). What runs code, or re-documents (rustdoc's
     two configurations of one crate re-document each other every run, `CR:994-997`),
     costs time whether or not the crate changed, so only those are skipped.
   - What would change this: the clean-tree `--quick` time (E.4 item 4). If the
     compile-only steps dominate it, a later unit can move the feature combinations or
     the cross-target legs to `--commit`, at the cost named in (b).
2. **How `--quick` knows a step's inputs are unchanged: a per-step key, (f).**
   - (a) the staged diff against `HEAD`: a unit that is a stack of commits
     (`todo_v0.7.x_kde-open-items-steps4-5-spec.md` C.1: "one commit, or a stack whose
     intermediate commits pass the gates of the crates they touch and whose last passes
     the unit's gate") would be judged by its last commit alone, and a merge's gate
     runs on the merge commit with nothing staged (`todo_v0.7.1_gpui-kit-0.9.1-plan.md`:
     "**a merge's gate** (M1) runs on the merge commit", in its §4, *Gates*; the KDE step 1 plan's MK1 and MK3
     gates are `--quick` on a merge, its MK2 `--commit`), where the diff is empty and
     nothing would be tested.
   - (b) a `--since <rev>` argument: correct when given, silently narrow when
     forgotten. (c) the remote branch: nothing of a release is pushed before its merge
     with its predecessor's tag (MG; rationale S2, as amended on 2026-11-15: "the branch is
     *merged with* `<prev-tag>`, not rebased onto it"). (d) `HEAD` recorded at the last pass: the gate passes before the
     commit, so the next diff re-holds the change already gated.
   - (e) (the first version): the working tree recorded as a git tree object at each
     passing run; the next `--quick` diffs against it, maps paths to crates by package
     directory, an include scan and stated run-time reads, and closes over
     `cargo metadata`'s path dependencies. For: a crate-level scope line; one `git diff`
     per run. Against: tree objects and blobs written into the shared object database
     through a temporary index, a per-worktree ref needed so that `git gc` never prunes
     the recorded tree, checked exit statuses for every git call; whole-run granularity
     (a failed run records nothing); conditions on when a record may be written; a
     second dependency model beside the step cache's, which can drift from it; a
     closure over dev-dependency edges that selects more than Cargo builds; a second
     self-tested file.
   - (f) chosen: a key per test or rustdoc step, computed as the step cache computes
     one (`step_cache_key`), over the reach of its `-p` crates without the strict
     cache's refusals for host-reading build code, their included files, the tool
     versions, the environment but a named deny-list (decision 3), the gate's scripts,
     every file outside the packages but a short list no crate reads, and the command
     (A.2); records per step in a set of their own. For: one dependency model, the
     cache's, for both (a key over the reach of crate X changes when X or anything X
     builds against changes: the floor's "every crate that depends on one it
     changes"); content-based, so stacks, merges, rebases and fixes after a failure need
     nothing (a step that passed in a failed run keeps its record); no git writes; no
     record conditions beyond `step_passed`'s. Against: a key computed per step (one
     `cargo tree` per crate set, memoised; the reach's files hashed per step; time not
     measured); keys miss more often than (e)'s scope (any edit of the gate's scripts,
     any tool or environment change outside the deny-list, any file outside the
     packages not on the unread list), which only runs more; the scope is shown per
     step and in the summary's list, not as a crate list; it relies on `cargo tree`'s
     output format, as the cache already does.
   - Neither (e) nor (f) follows what a test reads outside the repository (the host's
     desktop files, fonts, icon themes and libraries, the network, the clock), so each
     can skip a test whose verdict that has changed; S1's floor accepts this for
     `--quick`, and `--commit` runs those tests (A.4). Within what both follow, (f) is
     the better design by the project's rule against overcomplication: it meets the same
     requirement with fewer mechanisms and no second model that could drift. What would
     change this: a measured `--quick` whose key computation takes a large share of its
     time.
3. **The `--quick` key's environment: every variable but a named deny-list**
   (`step_cache_quick_environment`, C.1), fail-safe like the strict cache's whole
   environment (`SC8a:48-52`): an unknown variable that changes makes the step run.
   - (a) an allow-list of the variables known to change a verdict (the first revision's
     `CARGO_*`, `RUST*`, `NEXTEST_*`, `NATIVE_THEME_*`, `XDG_*`, `GTK_THEME`, …): it
     misses what no list foresaw, and a miss is a skipped test. Found missed, each read
     by the workspace or a crate it builds: `HOME` and `PATH` (the KDE reader's files,
     the programs tests run), the locale, `ImageOS` and `ImageVersion`
     (`tools/os-probe/src/main.rs:160`, `:165`), `DBUS_SESSION_BUS_ADDRESS`
     (`zbus-5.19.0/src/address/mod.rs:68`), `FONTCONFIG_FILE`
     (`fontdb-0.23.0/src/lib.rs:509`), `GPUI_LEAK_DETECTION`
     (`gpui-pre-0.3.12/build.rs:14`, read there; `:12` is its `rerun-if-env-changed`
     line), `ICU4X_DATA_DIR`
     (`icu_normalizer_data-2.4.0/build.rs:6`), `ZTRACING`
     (`gpui-pre-ztracing-0.3.12/build.rs:4`), `VULKAN_SDK`
     (`ash-0.38.0+1.3.281/build.rs:10`), each at the version `Cargo.lock` pins, and the
     variables `cc` and `pkg-config` read.
   - (b) the whole environment, as the strict key: every run of another shell or
     session (any variable that differs) misses every key.
   - (c) chosen: the whole environment but a deny-list of what cannot change a verdict:
     `_`, `SHLVL`, `OLDPWD`; the thread and job counts `RUST_TEST_THREADS`,
     `CARGO_BUILD_JOBS` and `NEXTEST_TEST_THREADS`, left out only as a plain positive
     number of up to nine digits, keyed by their text otherwise (`SC8:239-243`'s form
     rule); the output settings; the build locations; the speed settings; and the
     per-session variables (C.1 lists each). Its cost: a variable outside the list that differs between two calls makes
     every `--quick` key miss, which only runs more; E.4 item 4 shows whether it
     happens. Not taken: unsetting the per-session variables in the wrappers instead of
     leaving them out of the key, since the tests would then run in another environment
     than the session's.
4. **The strict reach's registry build code, and the `--quick` reach without the host
   refusals.**
   - *Registry build scripts and proc macros, in the strict reach.* (x) Trust them as
     pinned, 0.8.0's choice: `CR8:761-770` refuses only a reach holding a named
     host-probing helper (`CACHE_HOST_PROBES`), a path package outside the workspace or
     an unlisted workspace build script or proc macro, and says "Registry packages are
     pinned by Cargo.lock and their sources never change". For: no upkeep. Against: a
     pinned build script can still read the host (C.2 found `libc` running `emcc` on every
     host and `inotify-sys` running `pkg-config` directly on two BSDs, neither seen by the
     name test), and a lock change can bring a new one unread, which the cache would then
     skip over silently. (y) A reviewed list (`CACHE_REVIEWED_REGISTRY_CODE`, C.2): a
     strict reach holding a registry package with a build script or proc macro the list
     does not name is refused, the step runs. For: no unread build code under a strict
     key, and a lock change that adds or moves one is caught mechanically. Against: its
     cost lands on every unit that changes `Cargo.lock` (in v0.7.1, the GPUI plan's L1 and
     L2): until that unit re-reviews the moved packages, the steps whose reaches hold them
     run (time, never a skipped check), and the review is a grep of each moved build
     script and proc macro for program runs, file reads and variables. Chosen by the
     maintainer on 2026-11-15, as recommended: (y),
     because a silent skip is the one failure the cache must not have and the cost is time
     and a short grep per lock change; it was put to the maintainer with the plan, since
     it puts a review on every unit that changes `Cargo.lock` (first raised by the
     orchestrating session in its second review round, 2026-11-15). What would change it:
     lock changes frequent enough that the refusals cost more than a review per change.
   - *The `--quick` reach.* `--quick` accepts that the host is not followed, as S1's floor
     does, and leaves it to `--commit`: its reach skips both the host-probe refusal and
     (y)'s list. The refusals that keep a key complete within the repository stay: a path
     package outside the workspace, an unreviewed workspace build script or proc macro, an
     include the scan cannot resolve (C.4).
5. **A step the strict cache keys gets no `--quick` key.** Where a test or docs step is
   declared in `CACHE_INPUTS` and its reach is accepted, the strict key decides for it in
   `--quick` too: it is the more complete key, and one record per step keeps the two sets
   from disagreeing. At `0fa28027` that is ten of the 39 test and docs steps (C.3), so 29
   carry a `--quick` key. A declared step whose strict key is off for the run (the full
   metadata unread, `step_cache_init` failed, C.2) or whose strict reach is refused falls
   back to the `--quick` key in `--quick`, rather than simply running: the strict
   cache's refusals are about the host and the registry, which the `--quick` key does not
   claim to follow anyway (decision 4), and without the fallback the gpui, iced and egui
   docs steps, whose strict reaches the probes refuse, would run in every `--quick` (the
   29 count them as quick-keyed). In `--commit` such a step runs (no lookup), and its
   pass is recorded in the `--quick` set (A.2's recording rule), so the next `--quick`
   can skip it.
6. **One rule for every crate, no per-crate tables.** A table of what a change of crate
   X runs would restate the reach and drift from it.
7. **The showcases' runner-environment legs are keyed like any test step.** Each names
   its connector with `-p` (`CR:835-848`), and `scripts/run_in_runner_env.sh`, which
   pins the environment's packages, is outside the packages and not unread, so it is in
   every `--quick` key. The GPUI Fast leg stays skipped in `--quick` as in `--commit`
   (`CR:849-855`); the plans run it beside the gate where gpui changed
   (`todo_v0.7.1_gpui-kit-0.9.1-plan.md`: "the gate adds `cargo test -p
   native-theme-gpui --features gpui-fast,native,watch --all-targets` and ...").
   `run_in_runner_env.sh --prepare` runs as today.
8. **CI's MSRV checks in `--commit` and `--quick`, not in the full run (O1a answered
   no)**, ported inside GS, decided on 2026-11-15 by the session orchestrating the
   batch. This goes beyond the approved S1, which names `--quick`, nextest and the step
   cache only, so it waited for the maintainer's approval (O3), the full run's part for
   O1, and where 1.88.0 builds for O2; the maintainer answered on 2026-11-15 (O3's keyed
   option, O1a no, O2 (e)). 0.8.0 runs them in every mode (`CR8:888-925`, from
   `350ee147`, `c36a874f` and `649209d0`); 0.7.x runs none locally. The reasons given
   with the decision: the local gate's missing them once left CI's MSRV job red for a
   day unnoticed (reported by the orchestrating session, not traced here to a CI run).
   The history names four MSRV-only cases, each subject read with `git show -s`:
   - `4470c0da` (0.8.0, 2026-10-31): "next: the msrv job green: the nine dead-code
     markers on items that only dead code under an `expect` uses are `allow`s, as Rust
     1.88 counts such items used and stable does not": the job was red over a lint at
     1.88.0 alone;
   - `62498b45` (0.8.0, 2026-10-31): "the registry's const checks the derive's
     assertions read keep their dead-code allow, for Rust 1.88 (the MSRV), which counts
     no const assertion as a use": a 1.88.0 case; whether CI was red is not stated;
   - `c36a874f` (0.8.0, 2026-10-31): "the msrv job green on its connector toolchain too:
     … as Rust 1.95 (the egui and gpui checks) counts what expect-marked dead code uses
     as used": a 1.95.0 case;
   - `a9c34cdd` (0.7.x's own history, an ancestor of `0fa28027`, 2026-10-21): "without
     the kde and portal readers linux_reader is the empty match alone, so the MSRV build
     the gpui connector's default features make no longer warns of an unused binding;
     CI's MSRV job checks the gpui connector's three configurations with warnings denied";
     `ci.yml:262-263` records why the job denies warnings: "A lint can fire at an MSRV and
     not on stable (native-theme without its Linux readers once warned of an unused
     binding at 1.95.0 alone)".

   And under S4 the branch is pushed before the full run, so an MSRV-only lint would
   otherwise be first seen by CI, as a red run and an e-mail. The cost: the workspace
   checked on two more toolchains (`rustup toolchain list` lists `1.88.0` and
   `1.95.0`); the first run builds for each, the dependencies' compiles partly served by
   the global sccache wrapper (`~/.cargo/config.toml:9`, M.3), the workspace crates,
   build scripts and proc macros not (1.88.0's two checks measured at about 19 s a run
   from a fresh folder with sccache warm, O2; 1.95.0's not measured); and, for 1.88.0, a build
   folder the global `build.build-dir` does not reach (M.3, O2). The MSRV checks' times
   go into GS's report to the maintainer, as the cache's saving does (E.4 items 1, 2 and
   4; the report is completed with the GPUI plan's E1, which runs after GS ends). Part M.
9. **The security audit runs in `--quick`**, as in `--commit` (`CR:1188-1239`): CI
   audits the push (`ci.yml` `audit` job), and decision 1 (c) keeps running every step
   but the ones it keys (the test and docs steps and the two `+1.88.0` MSRV checks).
10. **nextest runs the test-step commands of B.3 in `--commit` and `--quick` (eleven of
    its twelve; `:715` is the full run's, B.3), as
    on 0.8.0; the runner-environment legs stay on `cargo test`; an argument after `--`
    is refused; `RUST_TEST_THREADS` caps nextest's concurrency.** Part B.
11. **No serial or single-process configuration for nextest.** B.4 gives the evidence.
    What nextest cannot show, interference between tests of one process, the full run
    and CI keep (`cargo test`).
12. **The strict cache declares every host-target compile-only cargo step of the
    workspace's crates (0.8.0's loop, with 0.7.x's other step descriptions by name) and
    `native-theme-derive`'s tests; no Python check is cached.** C.3. 0.8.0 cached its
    own data checks after tracing what each reads. 0.7.x's Python checks read more
    than the repository and were not traced for this specification: the widget
    coverage reads the toolkits' sources in Cargo's registry through `cargo metadata`
    (`CR:878-885`), the connector parity the three connectors' sources (`CR:887-893`),
    the README check every README and every crate's `src/` (`CR:895-902`). A missed
    read would be a check skipped silently, for a time not measured (each prints its
    own in the run's output).
13. **A cargo step's inputs include the files its reached crates include from outside
    their packages, and target sources outside a package** (C.4). 0.8.0's `@rust:`
    inputs are the reached packages' directories and Cargo's configuration
    (`CR8:740-821`); 0.7.x's sources include `docs/*.toml`, `docs/platform-facts.md`,
    `scripts/capture_window.sh` and other packages' showcase files (C.4's table),
    which that rule does not hash (the root `Cargo.toml`, also included, is in
    `rust_config_inputs` already).
14. **The include scan reads code only, in Python** (`scripts/step_cache_includes.py`,
    C.4): comments and string literals are blanked by `scripts/check_panic_patterns.py`'s
    `code_only`, which the helper loads by path, so that an include written in a comment
    or a generated string (`native-theme-build/src/lib.rs:110`'s doc comment, the code
    `native-theme-build/src/codegen.rs` emits) is not taken for one. A bash scan of the
    text, the first revision's, read those as includes. The helper first runs before
    the gate's Python 3.11 check (`CR:905-913`): it serves the cache's first keys, after
    `CR:408` and at `CR:425`. It needs no 3.11 feature: it is written with the standard
    library's `os`, `re`, `sys` and `importlib` only (it reads no metadata, so no `json`:
    the target sources outside a package are the reach's, read in bash with `jq`, C.4),
    and the lexer it loads already runs
    before that check today (the panic scan, `CR:400`); a grep of
    `scripts/check_panic_patterns.py` for later forms (`:=`, `match`, `tomllib`,
    `except*`, `removeprefix`) finds none. So the check is not moved; an older
    interpreter failing the helper refuses the reach and the step runs.
15. **Records per worktree; the shared form is not ported.** 0.8.0 shares the facts
    gate's parts between worktrees (`80b722b8`, `46d66d93`, `1e970f6a`), because one part
    ran for hours and its verdict depended on a known, narrow set of variables. 0.7.x
    has no such step, and each worktree builds in a build folder of its own anyway.
16. **The step cache's self-test in `--commit` and `--quick`, not in the full run (O1b
    answered no)** (0.8.0
    runs it in every mode, `CR8:1322`, which S1 ports, rationale §1). The include
    helper's self-test is one of its cases, so it is one step.
17. **`--no-cache` without `--commit` or `--quick` is refused** (exit 2); 0.8.0 accepts
    it and does nothing (`CR8:136-156`). With `--quick` it turns both keys off, so
    `--quick --no-cache` runs every step `--commit` runs.
18. **The review is `review-hard`** (Part F): a wrong key skips a check silently, and
    0.8.0's first review of the same cache found must-fix defects (`46d66d93` fixes six
    numbered findings of "the review of cache-key", the first a must-fix, and five
    minors; `1e970f6a` its finding 2).

## Part M — CI's MSRV checks

### M.1 The steps

`msrv_checks`, ported from `CR8:888-925` without its `PLAN_EXCLUDED` part (the loop
`CR8:903-909`, which took plan-excluded crates out of the workspace check and changed
its description; `649209d0`), and with its comment (`CR8:888-895`) rewritten: 0.8.0's
"Every mode runs all of them, exactly as the job does" and its `PLAN_EXCLUDED` sentence
give way to "`--commit` and `--quick` run them, `--quick` the two at the workspace's
`rust-version` only where anything their key follows changed (their crates' reach and
included files, the files outside the crates but `QUICK_UNREAD`, the tools, the
environment, their toolchain); the full run leaves them to `--commit` and CI", in a new file `scripts/msrv_checks.sh` that
`check_release.sh` sources, so that the `ci.yml` check below can be called on a scratch
copy (E.3 (m)): it holds the toolchain reads, the six argument lists and
`msrv_ci_check <path to a ci.yml>`, which returns 0 or prints the difference and
returns 1; sourcing it defines functions and runs nothing. The script reads only the two
toolchains, through a function (`msrv_toolchains`) that `check_release.sh` calls only in
the modes that run Part M; the six argument lists are written in it, as on 0.8.0:

- `MSRV_TOOLCHAIN="+<rust-version>"`, from the root `Cargo.toml` (`CR8:896`): `+1.88.0`
  at `0fa28027` (`Cargo.toml`, `rust-version = "1.88.0"`);
- `CONNECTOR_TOOLCHAIN`, the second `rust-toolchain@<version>` of `ci.yml`'s `msrv` job
  (`CR8:897-898`): `+1.95.0` at `0fa28027` (`ci.yml:272` is `@1.88.0`, `:275` `@1.95.0`).
  A read that gives an empty version fails the run before the first MSRV step, naming
  the file.
- Six `run_check` steps, each `env RUSTFLAGS=-Dwarnings cargo <toolchain> check …`, as
  the job runs them under its `RUSTFLAGS: -Dwarnings` (`ci.yml:268-269`):

| Description | Arguments after `check` | `ci.yml` |
|---|---|---|
| `MSRV +1.88.0, -D warnings (the workspace but the gpui and egui crates, all features)` | `--workspace --exclude native-theme-gpui --exclude native-theme-egui --exclude native-theme-egui-widgets --all-features --locked` | `:273` |
| `MSRV +1.88.0, -D warnings (native-theme, default features)` | `-p native-theme --locked` | `:274` |
| `MSRV +1.95.0, -D warnings (the egui crates, all features)` | `-p native-theme-egui -p native-theme-egui-widgets --all-features --locked` | `:276` |
| `MSRV +1.95.0, -D warnings (native-theme-gpui)` | `-p native-theme-gpui --locked` | `:281` |
| `MSRV +1.95.0, -D warnings (native-theme-gpui, native, watch)` | `-p native-theme-gpui --features native,watch --locked` | `:282` |
| `MSRV +1.95.0, -D warnings (native-theme-gpui, gpui-fast, native, watch)` | `-p native-theme-gpui --features gpui-fast,native,watch --locked` | `:283` |

- A seventh step, `MSRV: the six checks are ci.yml's msrv job` (`run_check … msrv_ci_check
  .github/workflows/ci.yml`), first in the section. `msrv_ci_check` takes the first
  toolchain from `Cargo.toml`'s `rust-version` and reads the second, and the job, from
  the file it is given. It reads the `msrv` job from its header line `  msrv:` up to, and
  not including, the next job's header line, a line matching `^  [a-z-]*:$`:
  `awk '/^  msrv:$/{f=1;print;next} f&&/^  [a-z-]*:$/{exit} f'`, which also ends cleanly
  where `msrv` is the file's last job (a `sed -n '/^  msrv:$/,/^  [a-z-]*:$/p' | sed '$d'`
  would there drop the job's own last line). Run on `0fa28027`'s `ci.yml` for this
  document, the `awk` prints 19 lines, `ci.yml:266-284`, from `  msrv:` to the blank line
  before `  test-build-crate:`. It accepts only the line shapes the job has at `0fa28027`
  (`ci.yml:266-283`), failing on any other: `runs-on: ubuntu-latest`; a job-level `env:`
  holding `RUSTFLAGS: -Dwarnings` and nothing else; `steps:`; `- uses:
  actions/checkout@<v>`; `- uses: dtolnay/rust-toolchain@<version>`; `- run: cargo check
  <arguments>`; the system-dependency step, `- name: Install system dependencies (gpui)`
  followed by `run: sudo apt-get …` (`:277-278`); comment lines; blank lines. So a
  step-level `env:`, `working-directory:`, `if:`, `continue-on-error:`, `shell:`,
  `strategy:` or any other key fails it. Outside the job, the file's top-level keys must
  be `name`, `on`, `env` and `jobs` and no other (`ci.yml:1`, `:6`, `:12`, `:15`; a
  top-level `defaults:` or `permissions:`, which would change every job, fails it), and
  the workflow-level `env:` must hold `CARGO_TERM_COLOR: always` and nothing else
  (`ci.yml:12-13`; an output setting, on the deny-list, C.1): a new workflow-level
  variable fails it. Its `run: cargo check …`
  lines, each with the toolchain of the last `uses: dtolnay/rust-toolchain@<version>`
  above it, must be exactly the six (toolchain, arguments) pairs of the table, in order.
  A job not found, an empty version, or fewer or more `cargo check` lines fail it too. So
  a change of `ci.yml` that the script does not follow fails locally instead of leaving
  the two to differ.
- **Written, not read at run time.** Reading the six commands from `ci.yml` would keep
  them in one place and drop the table and the check. Against it: `ci.yml`'s `run:` is
  shell, so running it locally needs either `eval`, which would run whatever a CI edit
  adds, or a parser that accepts only plain `cargo check <arguments>` lines, which is
  the check's own parser, so reading saves only the six-line table; an edit of `ci.yml`
  would change what the local gate runs without a change to the gate's code or its
  review; and the descriptions, which the summary, the record and the plans name, would
  follow the YAML's text. Written, a drift costs one loud failure and a two-place edit.
  Chosen: written, with the check. What would change this: a `msrv` job that changes
  often.

### M.2 Where, and in which modes

- A section `MSRV (CI's msrv job)` right after the `cargo check` section (after
  `CR:425-428`), as `CR8:1334-1335`: in `--commit` and `--quick`, not in the full run (O1a
  answered no, 2026-11-15). In `--quick` the two `+1.88.0` checks follow A.2's key (O3's keyed
  option): they run where it differs from their last pass's; the four `+1.95.0` checks
  run whatever changed, as the compile-only steps decision 1 (c) keeps; M.1's check runs
  always.
- Before the six steps, `rustup toolchain list` is read once and each toolchain matched,
  by the first field of a line (a line can carry ` (active, default)` after it), against
  its exact name `<version>-<host>` (`1.88.0-x86_64-unknown-linux-gnu`, the host from
  `rustc -vV`; the list also holds `1.95-…` and `1.95.0-…`, which a prefix match would
  confuse). A step whose toolchain it does not list fails without running cargo, with
  `toolchain 1.88.0 is not installed (rustup toolchain install 1.88.0)` (the version as
  read); where `rustup` is missing or the listing fails, the first MSRV step fails with
  that reason and, as every hard failure does (`CR:215`), the run ends. None is skipped
  with a warning: the script's own rule for a check whose toolchain is missing is that "a gate
  nobody runs is not a gate" (`CR:1069-1071`, of the docs.rs emulation's nightly), and
  CI runs these on the push. Checking first keeps rustup from installing a toolchain
  inside a gate where its auto-install is on (`RUSTUP_AUTO_INSTALL`; its default on
  this machine's rustup not verified); for the same reason the two `+1.88.0` checks'
  `--quick` keys, which run `rustc +1.88.0 -vV` (A.2), are computed only after this
  check has found the toolchain listed.
- Not keyed by the strict cache: its tool versions are the default toolchain's
  (`cache_tool_versions`, C.2), not `+1.88.0`'s or `+1.95.0`'s. Under O3's keyed option
  the two `+1.88.0` checks carry the `--quick` key in `--quick`, with `rustc
  +1.88.0 -vV` added to their key (A.2), which removes that obstacle for them; in
  `--commit` they always run. The four `+1.95.0` checks and M.1's `ci.yml` check (it
  reads two small files) are never keyed.
- The header gains `CR8:54-55`'s sentence in this form: "CI's MSRV job (every `cargo
  check` it runs, each on its toolchain, -D warnings) runs in --commit and --quick (in
  --quick the rust-version checks where anything their key follows changed: their
  crates' reach and included files, the files outside the crates but QUICK_UNREAD, the
  tools, the environment, their toolchain); the full run leaves them to --commit and
  CI." 0.8.0's "runs in every mode, so no local gate misses
  what it checks" would be false, O1a being answered no.

### M.3 Where 1.88.0 builds (O2)

The user's global Cargo configuration sets `build.build-dir =
"/run/media/tibi/work/cargo-build/{workspace-path-hash}"` (`~/.cargo/config.toml:8`),
so that builds stay off `/home`. Cargo 1.88.0 does not read it: `cargo +1.88.0 metadata
--no-deps --format-version 1` has no `build_directory` field (`jq 'has("build_directory")'`
gives `false`) and reports the worktree's `target/` as `target_directory`, as the stable
toolchain and 1.95.0 do too; those two also report the build folder as `build_directory`
(`/run/media/tibi/work/cargo-build/fe/88b35d036b2878`). So the two `+1.88.0` steps
would build the workspace's checks into `target/` on `/home`. Where they build is O2's
(the options, *Open points*); the maintainer chose (e) on 2026-11-15, and the folder's
size is recorded at E.4 item 1 (`du -sb`, in bytes): the gate's trap prints it just
before it deletes the folder, since a `du` after the run would find none; the trap's
rules are O2 (e)'s.

The same configuration sets a global compile wrapper, `rustc-wrapper =
"/home/tibi/.cargo/rustc-low-priority.sh"` (`~/.cargo/config.toml:9`), which lowers its
priority and `exec`s sccache (`/home/tibi/.cargo/rustc-low-priority.sh`, its last line);
`config.toml:11-14` says sccache "does not cache incremental (workspace) crates, only the
dependencies". Cargo 1.88.0 uses it here: the orchestrator's measurement of 2026-11-15
(O2) found sccache counting 286 compile requests in each run of the two `+1.88.0`
checks. A 1.88.0 build in a fresh folder takes from sccache what it holds for 1.88.0
and compiles anew what it does not cache. In the measured second run
(`sccache-after-2.json` minus `sccache-before-2.json`), the 286 requests were 214 hits,
0 misses, 64 not cacheable (sccache's `not_cached` counts for that run: 25 binaries, the
build scripts among them, 24 proc-macro crates, 6 incremental crates, the workspace's,
and the rest of other kinds), 4 `requests_not_compile` (requests that were no compile)
and 4 `compile_fails`: 214 + 64 + 4 + 4 = 286. The cost of a fresh
1.88.0 folder, sccache warm, is about 19 s a run (O2's measured paragraph).

## Part A — `--quick`

### A.1 Arguments and modes

- `CR:64-71` becomes 0.8.0's parser (`CR8:132-156`) without `--cache-keys`: `--commit`
  sets `COMMIT_MODE`, `--quick` `QUICK_MODE`, `--no-cache` `NO_CACHE`. Both modes:
  `--commit and --quick exclude each other`, usage, exit 2 (`CR8:147-151`). `--no-cache`
  with neither: `--no-cache goes with --commit or --quick`, usage, exit 2 (before commit
  (iv), `--no-cache goes with --commit`). An unknown argument as today (`CR:69`).
- After those checks, `--quick` also sets `COMMIT_MODE=1`: every per-commit skip of `CR`
  applies in `--quick` at its existing condition. `MODE_FLAG` is `--quick` or `--commit`,
  for the texts below.
- Texts that name the mode: the title (`CR:327-331`), three lines as `CR8:867-873`
  (`per-task checks (--quick)`); `print_skipped` (`CR:150-154`) prints
  `skipped ($MODE_FLAG mode)` and the summary's heading `Skipped (--commit mode):`
  (`CR:1395-1400`) becomes `Skipped ($MODE_FLAG mode):`, both as today in `--commit`;
  the banner (`CR:1360-1367`) in `--quick`, `per-task gate (not a per-commit or release
  check)` (`CR8:1201-1203`); the closing note (`CR:1404-1409`) keeps its text in
  `--commit`, and in `--quick` is `./scripts/check_release.sh --commit runs every third
  code task (unit), counted from the last --commit, on a release's last code task, after
  a lock or toolchain change and where a plan names a task structural; --quick at a
  task's end.` (S1's cadence, rationale §2, "every third code unit of a release, counted
  from the last `--commit`").

### A.2 The `--quick` key

- **Which steps:** a test step (`_run_tests_impl`) or a step whose command's first
  `cargo` word is followed by `doc` (`CR:1000-1021`, run through `env RUSTDOCFLAGS=…`),
  where the strict cache does not key it (decision 5). `--commit` at `0fa28027` has 39
  such steps, each naming its crate with `-p`: 18 test steps (`CR:665-677`, `:683-684`,
  `:704-705`, `:736-739`, `:759-762`, `:769-772`), 7 runner-environment legs
  (`:835-848`), 14 docs steps (`:1000-1021`). The strict cache keys ten of them (C.3:
  `test (native-theme-derive)` and nine docs steps); the other 29 carry a `--quick` key:
  17 test steps, the 7 legs, and the docs steps of gpui, iced and egui (`docs, private
  items` of each, `docs (… all features)` of iced and egui). A test or docs step with no
  `-p` fails the run (`a test or docs step names no crate with -p: <description>`).
- **The two `+1.88.0` MSRV checks, under O3's keyed option** (chosen 2026-11-15): they carry the
  `--quick` key too, so 31 steps are quick-keyed (the 29 and the two). `run_check`
  recognises them as the steps whose command's first `cargo` word is followed by
  `$MSRV_TOOLCHAIN check` (M.1's `env RUSTFLAGS=-Dwarnings cargo +1.88.0 check …`); the
  four `+1.95.0` commands, `$CONNECTOR_TOOLCHAIN check`, do not match. Their crates are
  the `-p` crates of the command, or, for `--workspace`, every workspace member it does
  not `--exclude` (`step_cache_command_crates`, C.1); their key adds `rustc +1.88.0 -vV`
  (the toolchain the step runs, whose release names its cargo too) to the base, computed
  only after M.2's toolchain check has found it listed. At `0fa28027` their reaches
  (A.3's): the workspace check, the members but gpui, egui and egui-widgets
  (`native-theme`, derive, test-fixtures, build, showcase-support, iced, the OS probe),
  with C.4's included files of that reach (iced's four showcase files among them); the
  `native-theme` check, `native-theme`, derive, test-fixtures. The four `+1.95.0` checks
  are not keyed: they build in the shared build folder, where fingerprints make an
  unchanged crate cheap, and they keep the compile-only backstop (decision 1 (c)).
  The other keyings, not taken (2026-11-15), are in *Under another outcome*.
- **The base** (`step_cache_quick_base`, C.1), computed at the run's start and again in
  each re-key (`step_passed`): a SHA-256 over `cache_tool_versions` (C.2);
  `step_cache_quick_environment` (decision 3, C.1); and the content of every file `git
  ls-files -co --exclude-standard` lists outside the workspace packages' directories but
  those matching `QUICK_UNREAD`, by path and content (`step_cache_inputs`): the root
  manifests, the lock, `.cargo/`, `.config/nextest.toml` if there is one, `docs/*.toml`,
  every script under `scripts/` (the gate's own among them, whichever exist at the
  commit, so the `--quick` base needs no list of them), `tools/gap-map/` and the rest.
  `QUICK_UNREAD` is `*.md`
  (any depth), `docs/archive/*`, `docs/issues/*`, `docs/assets/*`, `docs/evidence/*`,
  `.github/*`, `LICENSE-*`, `.gitignore`, `.gitattributes`, each matched against a path
  `git ls-files` prints with bash's `[[ $path == $pattern ]]`, where `*` also matches
  `/`: so each `<dir>/*` pattern matches at any depth (`docs/evidence/` holds files up
  to eight `/` deep at `0fa28027`, `.github/` two). Any depth is safe: a `git grep` of the
  workspace's Rust code lines (comments left out) for those folders' names in an
  `include*!`, a string path, a `CARGO_MANIFEST_DIR` or `parent()` join, run for this
  document on 2026-11-15, finds no read of any file under them by a test or a build
  script (the hits are comments and message texts; the two `parent()` joins read
  registry sources from `cargo metadata`), and `native-theme/build.rs` reads `icons/`
  only (C.2). A search of every `.rs` file
  found no include or run-time read of them but `docs/platform-facts.md`, which C.4 adds
  to the keys reaching `native-theme` (the reads searched: `include*!`, `#[path]`, file
  reads, `join("..`, `"/..`, `CARGO_MANIFEST_DIR` paths; the hits outside a package are
  C.4's table; showcase-support's test of the capture scripts,
  `tools/showcase-support/src/lib.rs:66-75`, `:126-138`, whose scripts are in the
  base; and `native-theme-build/src/schema.rs:294`, whose test reads
  `../native-theme/src/detect.rs` at run time, covered because `native-theme` is in
  build's reach, A.3). Where `git ls-files` fails, or this is no git work tree, no `--quick` key is
  computed: every quick-keyed step runs, with the reason printed once.
- **The inputs:** `rust_config_inputs` (C.2) and, for each `-p` crate of the command,
  its reach in `--quick` form (decision 4) with C.4's included files and target sources.
  Where a reach is refused or `cargo tree` fails, the step runs, the reason printed once
  per crate set.
- **The key:** `step_cache_key --quick` (C.1): `step_cache_key`'s form with the
  `--quick` base in place of the strict base, over the step's description, those inputs
  and its command.
- **Lookup**, in `--quick` only, before the step runs: `step_cache_hit --quick`; equal
  key and record, the step is skipped and printed `↷  <description>  unchanged since
  <label> (--quick)`, added to `QUICK_UNCHANGED`, counted in the summary's `unchanged`
  count (not as passed) and listed under `Left to --commit, unchanged since their last
  pass (--quick):`.
- **Recording**, in `--commit` and `--quick`, after a quick-keyed step passes: its key
  computed again, the base with it; where equal to the key before the step,
  `step_cache_record --quick` (`step_passed`'s rule). A `--commit` pass therefore makes the
  next `--quick` skip the steps nothing has touched since. `--no-cache` neither reads nor
  writes these records; the full run never does.
- **Each status checked:** every function computing a base, a reach or a key returns its
  status (C.1's pipe rule, so a failure inside a pipe is seen), and the callers test it
  explicitly (`if ! …; then`), not through `set -e`; a
  failure means the step runs.

### A.3 What a change re-runs, at `0fa28027`

The reach of each crate (`cargo tree -p <crate> --all-features -e normal,build,dev
--prefix none --format '{p}' --locked --offline`, its workspace packages):
`native-theme` → itself, derive, test-fixtures; `native-theme-build` → itself,
`native-theme`, derive; gpui and iced → themselves, `native-theme`, derive,
showcase-support, test-fixtures; egui → itself, egui-widgets, `native-theme`, derive,
showcase-support, test-fixtures; egui-widgets → itself, egui, `native-theme`, derive,
test-fixtures; showcase-support → itself, `native-theme`, derive; derive, test-fixtures
and the OS probe → themselves. So, for the 29 quick-keyed steps:

| A change of | Re-runs the quick-keyed steps of |
|---|---|
| gpui | gpui (5) |
| iced | iced (7) |
| egui or egui-widgets | egui (8) and egui-widgets (3) |
| showcase-support | showcase-support, gpui, iced, egui |
| `native-theme` | every crate but derive, test-fixtures and the OS probe (27 steps) |
| derive | every crate but test-fixtures and the OS probe |
| test-fixtures | itself, `native-theme`, gpui, iced, egui, egui-widgets |
| `native-theme-build`, the OS probe | itself |
| the gpui showcase's `pages/basic.rs`, `demo.rs`, `pages/mod.rs` | gpui and iced (C.4) |
| the egui showcase's `pages/basic.rs` | egui, egui-widgets and iced (C.4) |
| `docs/platform-facts.md` | the steps reaching `native-theme` (C.4) |
| any other file outside the packages and not in `QUICK_UNREAD` (`docs/*.toml`, `scripts/`, the manifests, the lock), a tool, a variable outside the deny-list, the gate's scripts | all 29 |
| a file matching `QUICK_UNREAD` | none |

(Counts: gpui's five are `test (native-theme-gpui, native, watch)`, `test
(native-theme-gpui, no features)`, its two legs, `docs, private items
(native-theme-gpui)`; iced's seven its three test steps, two legs and two docs steps;
egui's eight its three test steps, three legs and two docs steps; egui-widgets' three
its three test steps (its docs steps are strict-keyed); `native-theme`'s two test steps
and one test step each of build, test-fixtures, showcase-support and the OS probe make
the 29. The strict-keyed ten follow C.3's rules.)

### A.4 What `--quick` runs, and what it leaves to `--commit`

- Every step of `--commit`: the panic scan and its self-test, the step cache's
  self-test, `cargo check` per crate, M.1's `ci.yml` check and the four `+1.95.0` MSRV
  checks (Part M), the feature combinations, the cross-target checks, clippy and the
  strict lints, formatting, the iced and egui configurations' clippy, the one-`skrifa`
  check, the examples build, the Python checks and the audit; the strict cache skips
  those of its steps it may (C);
- the test and docs steps, and the two `+1.88.0` MSRV checks (O3's keyed option, A.2),
  whose key differs from their last pass's, every configuration of each;
- nothing of the per-commit skips (A.1).

It leaves to `--commit` the quick-keyed test, docs and `+1.88.0` MSRV steps whose inputs,
as A.2 keys them, are unchanged since they last passed. Their verdict can change only through what
the `--quick` key does not follow: files outside the repository (the host's desktop
configuration, installed fonts and icon themes, system libraries and the programs on the
path, which the key names only by `PATH`), the network, the clock, a run-time read of a
file in `QUICK_UNREAD`, and the build code decision 4 does not review. The next
`--commit` checks those.

S1's floor holds: formatting (`CR:518`), the panic scan (`CR:398-408`), clippy
`-D warnings` and the strict lints over the workspace (`CR:523-654`, `:740-746`,
`:763-774`), the tests and doctests of every crate the unit changes and of every crate
depending on one (A.3), the documentation checks (the Python checks always; rustdoc
wherever a crate or what it builds against changed, its verdict depending on those and
on its links into its dependencies), and never fewer checks than a task's targeted
commands: those are the tests of the crate a task edits (its key changes with it), the
checkers, a `-p` clippy or build and `cargo fmt --all --check`, each of which `--quick`
runs; the GPUI Fast legs, which `--commit` skips too, the plans run beside the gate.

## Part B — nextest

### B.1 Where

`HAVE_NEXTEST` is set in `--commit` and `--quick` where `cargo nextest --version`
succeeds (`CR8:933-939`), with the info line `test steps: cargo nextest (doctests: cargo
test --doc)` (`CR8:1106-1108`). The full run leaves it empty. nextest 0.9.138 is
installed here (`cargo nextest --version`: `cargo-nextest 0.9.138 (fc97e97bb
2026-06-21)`).

### B.2 `cargo_tests`

Ported from `CR8:569-595` with two adaptations:

- no nextest: `cargo test "$@"`, as today;
- with nextest, an argument `--` fails the step (`cargo_tests: no nextest form for
  test-binary arguments: …`, return 1). 0.8.0 translated one form,
  `-- --skip <name> --exact` (`CR8:597-614`), because nextest 0.9.138 ignored
  libtest's `--skip` and ran the whole facts gate where it should have been skipped
  (`ecde0fc3`); no 0.7.x step passes test-binary arguments through `cargo_tests`, so
  only the refusal is ported;
- else `cargo nextest run --no-tests=pass --status-level fail --final-status-level fail
  --color never [--test-threads <N>] "$@"` (`--color never`, `cargo nextest run --help`:
  `--color <WHEN>`, so that the summary line the count reads carries no colour codes
  whatever the terminal; `CARGO_TERM_COLOR` stays on the deny-list, C.1), then `cargo test "$@" --doc` where `doctests_selected
  "$@"` (`CR8:642-661`, ported as is: no target selected and the `-p` package has a
  doctested library or proc macro);
- `--test-threads "$RUST_TEST_THREADS"` where `RUST_TEST_THREADS` is set and
  `NEXTEST_TEST_THREADS` is not. nextest's help names `NEXTEST_TEST_THREADS` as its
  variable (`cargo nextest run --help`, `-j, --test-threads`), this machine has 32 CPUs
  (`nproc`), and the plans cap the gate with `RUST_TEST_THREADS=16`
  (`todo_v0.7.1_gpui-kit-0.9.1-plan.md`, P1: "gains the line `export
  CARGO_BUILD_JOBS=16 RUST_TEST_THREADS=16` (the gate's limit while the maintainer
  works"). Under `cargo test` the cap is the threads of the one binary running; under
  nextest, `--test-threads` caps the test processes running: the same concurrency.

### B.3 The steps

`cargo test` becomes `cargo_tests` at the twelve test-step commands `CR:667`, `:673`,
`:675`, `:684`, `:705`, `:715`, `:737`, `:739`, `:760`, `:762`, `:770`, `:772` (the set
`24711322` changed on 0.8.0); `--commit` runs eleven of them (`:715`, GPUI Fast, is the
full run's), as 18 steps, since `:675` runs for the eight crates the loop's other
branches do not take. Not changed: the OS probe's listing (`:670`), the stem-index test (`:698`,
full run only, with libtest arguments) and the runner-environment legs (`:835-854`),
whose `cargo test` runs each binary through `run_in_runner_env.sh` as Cargo's target
runner (`scripts/run_in_runner_env.sh:22-25`). `_run_tests_impl` counts nextest's
summary beside `cargo test`'s per-binary lines and names nextest's skipped tests
(`CR8:513-552`, ported without its `step_cached` and `step_passed` calls, which come with
the cache, C.2). In the full run the counting finds no nextest line and gives today's
count.

### B.4 One process per test

nextest runs each test in a process of its own and the tests of several binaries at
once; `cargo test` runs one binary at a time, its tests as threads of one process. What
this changes, read in the sources:

- **Environment variables**: `git grep -nE 'set_var|remove_var'` over `native-theme`,
  `native-theme-build`, `native-theme-derive`, `connectors` and `tools` finds no call;
  `native-theme-build` passes the environment as a table for that reason
  (`native-theme-build/src/lib.rs:3791-3795`: "`std::env::set_var` would change it for
  every test running at the same time").
- **Serial order**: no `serial_test`, no test lock (`git grep` for `serial_test`,
  `#[serial`, `Mutex<()>` finds only `native-theme/src/winicons.rs:279`, Windows library
  code, which CI's Windows legs test with `cargo test`).
- **The KDE watcher's tests** (the module at `native-theme/src/watch/kde.rs:109`) each
  watch a directory of their own named by process id and tag (`:124-133`), wait with a
  5-second timeout (`:115`), and assert only what a loaded machine cannot break: a fire
  at or after the debounce window, "fires exactly once" left to made-up instants
  (`kde.rs:175-181`; `native-theme/src/watch/mod.rs:907`
  `a_burst_is_due_once_after_its_last_message`, and `:971`
  `fire_debounced_fires_after_the_last_relevant_message`). A process per test gives each
  its own inotify instance and threads; nothing they assert depends on sharing one.
- **Shared temporary paths**: the tests with a fixed name under the temporary directory
  (`native-theme/src/presets.rs:303-320`, `native-theme-build/src/validate.rs:756-839`)
  each use a name no other test uses; the others add the process id. nextest runs each
  test once per step.
- **Process-wide caches**: `native_theme::detect::invalidate_caches()` resets caches
  every test of a process shares; `git grep -n 'invalidate_caches()' -- '*.rs'` gives 45
  lines, of which 9
  are calls (the rest comments, its definition and two generated strings in
  `native-theme-build/src/codegen.rs:275`, `:1184`): in the egui showcase's `app.rs`
  (three), the egui connector's `icons.rs:1269` and `watch.rs:124`, the gpui showcase's
  `app.rs:2045`, the gpui connector's `widgets/spinner.rs:657`, the iced showcase
  (`:2316`) and `native-theme/tests/timings.rs:71` (an ignored timing probe). Under
  nextest each test starts with empty caches; a test that passed only because another
  warmed a cache would fail under nextest, loudly, and is a defect of that test.
- **Per-process start-up**: what a process builds once, each nextest test process
  builds again: the system font database (`native-theme/src/fonts.rs:175`,
  `load_system_fonts`), the icon themes' fallback chains and stems
  (`native-theme/src/freedesktop.rs:88`, `:216`), the detection context
  (`native-theme/src/detect.rs:1213`). How much this costs against nextest's
  parallelism is not known; E.4 item 3 measures it.

So no `.config/nextest.toml` and no serial group. nextest does not reproduce two tests
interfering inside one process (the Windows icon loads `winicons.rs:270-279` describes
were such a case, found by CI); the full run and CI keep `cargo test` (`ci.yml`'s
`test`, `test-build-crate`, `test-connectors`: every leg runs `cargo test`), which this
unit does not touch.

## Part C — the step cache

### C.1 `scripts/step_cache.sh`

`SC8a` (818f0eab's 216 lines) and the parts both keys share, so that the self-test
covers them:

- `LC_ALL=C` on the three sorts (`SC8:176`, `:320`, `:331`), so that a key does not
  depend on the locale;
- the pipe rule: `step_cache_inputs`, `step_cache_key` and every function of this file
  that pipes see a failure inside a pipe, so that a file that cannot be read, or a
  failing `find` or `sha256sum`, fails the key instead of hashing a short listing. They
  do so locally, never by a shell-global `set -o pipefail`: a function body run in a
  subshell (`( set -o pipefail; … )`), `local -` before `set -o pipefail` (bash 4.4 and
  later; bash 5.3.20 here), or `PIPESTATUS` read after the pipe. A global `pipefail`
  in `check_release.sh` would change other code's pipes: `CR:670`'s `cargo test … --
  --list 2>/dev/null | grep -q ': test$'` ends its pipe early, so `cargo` may die of
  SIGPIPE, the pipe would then fail and `probe_min` drop to 0. `pipefail` sees only a
  loop's last command, so inside a loop that feeds a pipe every command that can fail
  (`find`, `sha256sum`, `git ls-files`, each pipe inside it) ends the function with
  `|| return 1` (`for f in bad ok; do sha256sum "$f"; done | sha256sum` under
  `pipefail` exits 0 where `bad` is unreadable and `ok` is not, and 1 with `sha256sum
  "$f" || return 1`; run for this document on bash 5.3.20). `step_cache_key` assigns the
  inputs hash first (`h=$(step_cache_inputs …) || return 1`) and prints it afterwards: a
  command substitution inside a `printf` argument hides the failure. The self-test checks the rule
  in the gate's own context (cases 3 and 5): the standalone entry, `bash
  scripts/step_cache.sh --self-test`, sets `set -euo pipefail` (as `SC8a` does, D.1),
  which would hide a function that lacks the rule;
- **the reach** (`rust_reach`, `rust_reach_resolve`, `rust_config_inputs`,
  `cache_inputs`), moved from `CR8:740-821` into this file, with `--quick` as an option
  of `rust_reach` (decision 4: the host-probe and the registry build-code refusals
  skipped). `cargo tree`'s output comes through one function (`step_cache_cargo_tree`)
  and the workspace's and the full metadata through two variables, so that the
  self-test feeds synthetic ones; the first `cargo tree` per crate set is memoised and
  serves both forms. `step_cache_command_crates <cargo arguments>` gives a command's
  crate set (A.2, for the `+1.88.0` MSRV keys): the crates its `-p` options name, or,
  where it has `--workspace`, every workspace member (`WORKSPACE_CRATES`, the package
  names of `CR:315`'s metadata, `CR:319`) but those its `--exclude` options name; an `--exclude` that names
  no member, or a command with neither `-p` nor `--workspace`, returns non-zero (no key,
  the step runs; C.1 case 11);
- **the included files** (`step_cache_includes`), which runs
  `scripts/step_cache_includes.py` (C.4) on the reached packages' directories and adds
  its targets to the reach's inputs; the helper's refusal refuses the reach; its result
  is memoised with the reach, per crate set, for the run. That is safe: every scanned
  file is itself in the reach's inputs and hashed by content, so an edit of one during
  the run (a new include among it) changes the key, and `step_passed`'s re-key then
  records nothing;
- **the `--quick` base**: `QUICK_UNREAD`, `step_cache_quick_environment` (decision 3)
  and `step_cache_quick_base` (A.2);
- **the two record sets**: `step_cache_init <dir> <label> <base>` as today (the strict
  set in `<dir>`), `step_cache_init_quick <base>` (the `--quick` set in `<dir>/quick/`),
  and `--quick` as the first option of `step_cache_key`, `step_cache_hit` and
  `step_cache_record` (0.8.0's `--shared` form, `SC8:390-486`, for another set);
- the self-test prints each case (`  ok  <case>: <ran|skipped>`, `SC8:526-528`) under a
  heading (`SC8:530`);
- the header comment as `SC8:3-26` up to "one per step", without the shared paragraphs
  (`SC8:28-59`) and the `!<pattern>` sentences (`SC8:19-21`), with paragraphs on the
  reach, the included files and the `--quick` key; the usage as `SC8a:22-31` with the
  new functions.

`step_cache_quick_environment`'s deny-list (decision 3): `_`, `SHLVL`, `OLDPWD`; the
thread and job counts `RUST_TEST_THREADS`, `CARGO_BUILD_JOBS`, `NEXTEST_TEST_THREADS`
where their value matches `^[1-9][0-9]{0,8}$` (keyed by text otherwise, `SC8:239-243`);
the output settings `CARGO_TERM_*`, `CARGO_LOG`, `RUST_BACKTRACE`, `RUST_LIB_BACKTRACE`,
`RUST_LOG`, `RUSTC_LOG`, `RUST_TEST_NOCAPTURE`, `NEXTEST_STATUS_LEVEL`,
`NEXTEST_FINAL_STATUS_LEVEL`, `NEXTEST_FAILURE_OUTPUT`, `NEXTEST_SUCCESS_OUTPUT`,
`NEXTEST_HIDE_PROGRESS_BAR`, `NEXTEST_SHOW_PROGRESS`, `NEXTEST_NO_INPUT_HANDLER`,
`LIBC_BUILD_VERBOSE` (it makes `libc`'s build script print, `libc-0.2.189/build.rs:69`,
`:82-85`); the build locations `CARGO_HOME` (its configuration files are inputs),
`CARGO_TARGET_DIR`, `CARGO_BUILD_TARGET_DIR`, `CARGO_BUILD_BUILD_DIR`; the speed settings
`CARGO_INCREMENTAL`, `CARGO_CACHE_RUSTC_INFO`; and the per-session variables a new
session, terminal or service gives each call, `CLAUDE_CODE_SESSION_ID`, `CLAUDE_PID`,
`CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET`,
`CLAUDE_CODE_CHILD_SESSION`, `CLAUDE_EFFORT`, `WINDOWID`, `KONSOLE_DBUS_SESSION`,
`KONSOLE_DBUS_WINDOW`, `SHELL_SESSION_ID`, `INVOCATION_ID`, `JOURNAL_STREAM`,
`SYSTEMD_EXEC_PID`, `MANAGERPID`, `MANAGERPIDFDID`, `MEMORY_PRESSURE_WATCH`,
`MEMORY_PRESSURE_WRITE`, `KONSOLE_DBUS_SERVICE`, `KONSOLE_DBUS_ACTIVATION_COOKIE`,
`CLAUDE_CODE_EXECPATH`, `CLAUDE_CODE_SESSION_ATTENDED`, `AI_AGENT` (the last two in this
session's environment; whether their values vary between calls was not checked, and they
are denied because they name only who runs the gate). Each of these per-session variables was searched for this
specification in the workspace's `.rs`, `.sh` and `.py` files (`git grep`) and in every
`.rs` file of Cargo's registry (`grep -r`): none is read (unlike some of the output
settings above, `RUST_LOG` and `RUST_BACKTRACE` among them, which registry crates do
read, and which are denied because they change what a run prints, not its verdict). The
one hit, `WINDOWID`, is in `windows-sys`'s (0.45, 0.48) and `windows`'s (0.54 to 0.62)
`Win32/System/DesktopSharing/mod.rs`, the constant `DISPID_RDPSRAPI_PROP_WINDOWID`, not a
variable read. Every other variable is keyed, sorted,
NUL-separated. The strict key keeps `step_cache_environment`, the whole environment but
`_`, `SHLVL` and `OLDPWD` (`SC8a:48-52`).

Its self-test is `SC8a`'s cases (`SC8:531-577` but the five exclusion cases
`:555-564`), the include helper's self-test (C.4) as one case, and these, on a synthetic
git tree under `target/` (its git commands with the caller's `GIT_*` removed and `-c
user.name=… -c user.email=… -c commit.gpgsign=false -c core.hooksPath=/dev/null`, as
`SC8:581-582`), with a synthetic workspace and full metadata and a cargo stub (behind
`step_cache_cargo_tree` and `step_cache_full_metadata`). Every case that removes a
permission (cases 1, 3, 5, and the helper's unreadable file) restores it with `chmod -R
u+rwx` on its folder before the self-test's `rm -rf`, also where the case fails (a
`trap` on the self-test's exit):

1. an unwritable cache folder: `step_cache_record` returns non-zero and the caller goes
   on (skipped with a note where the user can write it anyway, `SC8:823-826`);
2. the locale: inputs `B.txt` and `a.txt` hashed by `step_cache_inputs` under `LC_ALL=C`
   and under `en_US.UTF-8` give one hash (skipped with a note where that locale is not
   installed, `SC8:747-755`);
3. a pipe failure: an input file made unreadable (`chmod 000`), the first of two inputs,
   once given as a file and once inside a given directory, fails `step_cache_inputs` and
   the key, each time (skipped with a note where the user can read it anyway, as case
   1); its assertions run in the gate's own context, `bash -c 'set -e; set -u; source
   scripts/step_cache.sh; …'`, without `pipefail` (the pipe rule, above), and each
   assertion calls the function as the gate does, in a condition (`if ! h=$(step_cache_inputs
   …); then`), never as a bare command: under `set -e` a bare call fails on code that lacks
   `|| return 1` too, so it would not tell the two apart;
4. the `--quick` environment: `RUSTFLAGS`, `HOME`, `PATH`, `LANG`,
   `DBUS_SESSION_BUS_ADDRESS` or an unrelated new variable changed changes it;
   `CARGO_BUILD_JOBS=8`, `RUST_TEST_THREADS=4`, `NEXTEST_TEST_THREADS=2`,
   `CARGO_TERM_COLOR`, `CARGO_TARGET_DIR` changed leave it equal; `RUST_TEST_THREADS=0`
   (not a positive number) changes it; and the names next to the deny-list, each
   changing it: `CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER`,
   `CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS`,
   `CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER`, `CARGO_BUILD_TARGET`,
   `CARGO_BUILD_RUSTFLAGS`, `RUST_LOG_STYLE` (the deny-list names whole variables,
   `CARGO_TARGET_DIR` and `RUST_LOG` among them, but `CARGO_TERM_*`, its one prefix;
   none of the names above starts with `CARGO_TERM_`); every per-session variable of the
   deny-list (`CLAUDE_CODE_SESSION_ID` to `AI_AGENT`, above), each changed and each
   unset from a baseline that sets every one of them (so that "unset" removes a
   variable that was there), leaves it equal; and a denied variable whose value holds a newline and an `=`
   (`CARGO_TERM_COLOR=$'always\nRUSTFLAGS=-Dwarnings'`, with `RUSTFLAGS` unset in the
   case's baseline) leaves it equal, which proves the variables are split at NUL, not at
   newlines (split at newlines, that value would read as a `RUSTFLAGS` and change the
   key; with `RUSTFLAGS` unset in the baseline, an implementation that also dropped
   duplicate entries cannot pass by matching a `RUSTFLAGS=-Dwarnings` already there); so every gate checks the list and its
   parsing, and E.4 item 5 (A) and (B) check them end to end;
5. the `--quick` base: a nested file matching `QUICK_UNREAD`
   (`docs/archive/sub/notes.txt`) edited leaves it equal; a file outside the packages
   and not unread edited, or a new untracked one, changes it; `git ls-files` failing
   (the tree's `.git` made unreadable, with `GIT_CEILING_DIRECTORIES` set to the
   synthetic tree's parent, so that git cannot find the worktree's own repository above
   it) gives no base (that part skipped with a note where the user can read the `.git`
   anyway, as case 1); its assertions run in the gate's own context, as case 3's, and
   each calls the function as the gate does, in a condition (`if ! b=$(step_cache_quick_base
   …); then`), never as a bare command, for case 3's reason;
6. the reach: an include outside its package (a literal path to a `.md` file of the
   synthetic tree) is hashed: editing the target changes the key, in either form; in the
   `--quick` form this asserts that an include target that matches `QUICK_UNREAD` stays
   in the key, which is the `docs/platform-facts.md` exception (A.2, C.4); a non-literal
   include refuses the reach; the `cargo tree` stub failing gives no key in either form;
7. the refusals: a reach holding a probe (`cc`) or a registry build script missing from
   `CACHE_REVIEWED_REGISTRY_CODE` is refused in the strict form and accepted in the
   `--quick` form; an unreviewed workspace build script, and a path package outside the
   workspace, are refused in both;
8. the two sets: a step recorded in the strict set is not found by a `--quick` lookup,
   and a step recorded in the `--quick` set not by a strict one;
9. a step whose input changes while it runs records nothing, in either set;
10. the full metadata: a read that fails or lists no package turns the strict cache off
    for the run, with its reason (C.2), and no strict key is computed;
11. the command's crates (`step_cache_command_crates`, used by the `+1.88.0` MSRV keys,
    A.2): on a synthetic workspace `a`, `b`, `c`, `check --workspace --exclude c` keys
    `a` and `b`, so editing `c` leaves the key and editing `b` changes it (the step runs);
    the same command with `--exclude b` in place of `--exclude c` gives another key (the
    step runs); an `--exclude` naming no member refuses the key (the step runs); `-p a -p
    c` keys `a` and `c`.

### C.2 `scripts/check_release.sh`

Ported from `818f0eab` (its hunks of `CR`), with `46d66d93`'s warning: where
`step_cache_record` fails, `step_passed` prints `step cache: the pass of "<description>"
not recorded (its record could not be written)` as a warning (`CR8:708-709`). Without
it, `818f0eab`'s `step_passed` returns the failed record's status as its own, and
`_run_check_impl` calls it as a plain command under `set -e` (`CR:43`), so an unwritable
record would end a passing run there, with nothing but the shell's redirection error.

- `print_cached` (`CR8:384-392`); in both runners, `step_cached` before the run and
  `step_passed` after a pass, in `818f0eab`'s form (no `step_key_of`, no shared scope),
  extended by A.2's second key for the quick-keyed steps. A.2's rule holds for the strict
  key too: `step_cached` and `step_passed` test the status of `cache_inputs`,
  `rust_reach` and `step_cache_key` explicitly (`if ! …; then`), so a strict key that
  fails to compute runs the step (and records nothing), never ends the run through
  `set -e` and never skips (`818f0eab`'s `step_key=$(step_cache_key …)` took the key
  whatever its status);
- the full metadata (`cargo metadata --format-version 1 --locked`, with dependencies),
  read once where a cache is on, by `step_cache_full_metadata` in `step_cache.sh`
  (through the same cargo hook as `step_cache_cargo_tree`, so that C.1's case 10 runs it
  against a failing stub): `CR` reads only `--no-deps` (`CR:315`), which lists no
  registry package. A read that fails, or lists no package, turns the strict cache off
  for the run with an info line naming the reason (every strict step runs; the
  `--quick` key, which does not use the list, stays on);
- `rust_config_inputs` (in `step_cache.sh`, C.1) also names `.config/nextest.toml`, and
  `clippy.toml` and `.clippy.toml` in every reached package's directory and each of its
  ancestors up to `/`, as clippy looks its configuration up from the manifest's directory
  upward (`connectors/native-theme-gpui/clippy.toml` is the one that exists; none in the
  worktree's root or above it, up to `/home/tibi`);
- `HAVE_NEXTEST`, `STEP_CACHE` and `CACHED` set in `--commit` and `--quick`, the cache
  off with `--no-cache` (`CR8:933-943`);
- `CACHE_HOST_PROBES="pkg-config cc cmake bindgen system-deps vcpkg metadeps"` and
  `CACHE_TRUSTED_BUILD_CODE="native-theme native-theme-derive"` (`CR8:985-986`),
  checked for 0.7.x: the workspace packages with a build script or a proc macro are
  these two (`cargo metadata`: `native-theme`'s `custom-build`, `native-theme-derive`'s
  `proc-macro`); `native-theme/build.rs:11-22` reads `CARGO_MANIFEST_DIR`, `OUT_DIR` and
  `icons/` only, and `native-theme-derive/src/` has no `std::fs`, `include_*!` or
  `env::var` (`grep`);
- `CACHE_REVIEWED_REGISTRY_CODE`, new: the registry packages with a build script or a
  proc macro that the strict reaches hold, each `name vX.Y.Z`, reviewed for this
  specification. A reach's package is matched by the first two fields of its `cargo tree
  --format '{p}'` line, which can carry more after them (`enumflags2_derive v0.7.12
  (proc-macro)`, and ` (*)` on a repeated package, `proc-macro2 v1.0.107 (*)`, both
  printed at `0fa28027`). A strict reach that holds a registry package with a
  `custom-build` or `proc-macro` target (one `jq` over the full metadata) not on the list
  is refused, the step runs, the package named; so a lock change that adds or moves one
  is caught mechanically, and the list is extended by the review of that change. At
  `0fa28027` the seven accepted crates reach these, each searched for `Command::new`,
  file reads and `env::var`:
  - build scripts (23): `ahash v0.8.12`, `async-io v2.6.0`, `crc32fast v1.5.1`,
    `crossbeam-utils v0.8.23`, `getrandom v0.4.3`, `inotify-sys v0.1.8`, `libc v0.2.189`,
    `libm v0.2.16`, `num-traits v0.2.19`, `parking_lot_core v0.9.12`,
    `proc-macro2 v1.0.107`, `quote v1.0.47`, `rustix v1.1.6`, `rustversion v1.0.23`,
    `serde_core v1.0.229`, `serde_json v1.0.151`, `serde v1.0.229`, `slotmap v1.1.1`,
    `thiserror v1.0.69`, `thiserror v2.0.20`, `unicode-general-category v1.1.0`,
    `zerocopy v0.8.56`, `zmij v1.0.23`. Found: `rustc` run for its version or features
    (keyed by `rustc -vV`); Cargo's variables; `zerocopy` reading its own pinned
    manifest; `libm`'s `ENSURE_NO_PANIC`, `libc`'s `WIND_RELEASE_ID`, `LIBC_CI` and
    `LIBC_BUILD_VERBOSE`, the `RUST_LIBC_UNSTABLE_*` variables and `RUSTC_WRAPPER`,
    which it runs in `rustc`'s place where set (all keyed with the whole environment,
    and none on the `--quick` deny-list but `LIBC_BUILD_VERBOSE`); and programs other than
    `rustc`: `libc` runs `emcc -dumpversion` on every host (`libc-0.2.189/build.rs:137`,
    `:337-344`; it sets only `emscripten_old_stat_abi`, a cfg of the Emscripten target's
    code) and `freebsd-version` only under `LIBC_CI` (`:121-122`), and `inotify-sys`
    runs `pkg-config --variable=libdir libinotify` on NetBSD and OpenBSD only
    (`inotify-sys-0.1.8/build.rs`), directly, so `CACHE_HOST_PROBES`' name test cannot
    see it. Residual, recorded: on Linux, where the gate runs, only `emcc`'s presence
    and version are read and not keyed, and they change nothing a Linux build compiles;
  - proc macros (15): `async-recursion v1.1.1`, `async-trait v0.1.92`,
    `bytemuck_derive v1.12.0`, `darling_macro v0.23.0`, `enumflags2_derive v0.7.12`,
    `futures-macro v0.3.34`, `rustversion v1.0.23`, `serde_derive v1.0.229`,
    `serde_repr v0.1.21`, `serde_with_macros v3.22.0`, `thiserror-impl v1.0.69`,
    `thiserror-impl v2.0.20`, `tracing-attributes v0.1.31`, `zbus_macros v5.19.0`,
    `zvariant_derive v5.15.0`. Found: no file read, no program, no variable, but
    `rustversion`'s reading of the clock (`rustversion-1.0.23/src/time.rs`, for its
    date comparisons, which a stable toolchain does not evaluate) — recorded as residual.
  The list and its refusal are decision 4 (y); proc macros are in it on the same ground
  as build scripts: they run code at compile time and could read files.
  **Upkeep:** the list names each package with its version, so a lock change that moves
  one refuses the reaches holding it until it is reviewed again. The review is part of
  every unit that changes `Cargo.lock` (S1's warm-up gate follows each): the unit's
  first strict run names the refused packages, and the unit re-reviews and lists them.
  Listing names only would survive a version bump without review, at the cost of
  trusting a build script nobody read; the version form costs a slower gate until the
  review, never a skipped check;
- `STRICT_PANIC_INPUTS="scripts/check_strict_panic.sh"` (`CR8:991`);
- `cache_tool_versions` (`CR8:1065-1077`) and the start (`818f0eab`'s form of
  `CR8:1079-1093`): `step_cache_init target/check-release-cache <label> <base>` (which
  creates the folder), the base the tool versions, `sha256sum scripts/check_release.sh
  scripts/step_cache.sh scripts/step_cache_includes.py scripts/check_panic_patterns.py
  scripts/msrv_checks.sh` (the gate's five scripts: the helper loads its lexer from
  `check_panic_patterns.py`; `check_release.sh` sources `msrv_checks.sh`, which is in
  the list from commit (iii)),
  and `step_cache_environment`. Where `step_cache_init` fails (the folder cannot be
  created or its base not computed), the strict cache is off for the run, with an info
  line naming the reason: every step it would key runs, and the declared test and docs
  steps fall back to the `--quick` key in `--quick` (decision 5). `step_cache_init_quick`
  with A.2's base; where it fails (its folder `quick/` cannot be created or its base not
  computed, A.2), the `--quick` set is off for the run, with its own info line: no
  `--quick` key is looked up or recorded, so every quick-keyed step runs in `--quick` and
  `--commit` records no `--quick` pass, while the strict cache works as before. The label `<short HEAD>[ with uncommitted
  changes]`; the info line `step cache: on (target/check-release-cache; --no-cache runs
  every step)`;
- the summary: `TOTAL` counts the skipped steps of both sets (`CR8:2407`), and
  `print_cached` runs before the skipped list (`CR8:2457`), with the `--quick` list after
  it; the count `· %d unchanged` is printed only where a cache is on, so the full run's
  summary line stays `CR:1378-1379`'s.

### C.3 What the strict cache keys

`CACHE_INPUTS` (`CR8:993-1012`'s form): every cargo step on the host target that
compiles and runs nothing declares `@rust:<its -p crate>`, the strict lints the script as
well, and one test step:

- by the loop over the workspace crates (`CR8:1007-1012`): `check (<crate>)`,
  `clippy (<crate>)`, `strict-panic (<crate>)` (+ `STRICT_PANIC_INPUTS`),
  `docs, private items (<crate>)`;
- by name, the other host compile-only steps of `CR`: `clippy (native-theme, Linux
  features)`, `clippy (native-theme-gpui, native, watch)`, `clippy (native-theme-gpui,
  gpui-fast, native, watch)`, `clippy (native-theme-gpui, no features)`, `clippy
  (native-theme-iced, iced_aw, watch, native)`, `clippy (native-theme-iced, the
  showcase's required features)`, `clippy (native-theme-egui, all features)`, `clippy
  (native-theme-egui-widgets, all features)` (each `@rust:<crate>`); `strict-panic
  (native-theme, Linux features)`, `(native-theme, no features)`, `(native-theme-iced,
  iced_aw, watch, native)`, `(showcase-iced, iced_aw, watch, native)`, `(showcase-iced,
  default features)`, `(native-theme-gpui, native, watch)`, `(native-theme-gpui,
  gpui-fast, native, watch)`, `(showcase-gpui, native, watch)`, `(showcase-gpui,
  gpui-fast, native, watch)`, `(showcase-egui, all features)`, `(showcase-egui, no
  features)` (each `@rust:<its -p crate>` + `STRICT_PANIC_INPUTS`); `docs
  (native-theme)`, `docs, private items (native-theme, Linux features)`, `docs
  (native-theme-iced, all features)`, `docs (native-theme-egui, all features)`, `docs
  (native-theme-egui-widgets, all features)`;
- `test (native-theme-derive)`: `@rust:native-theme-derive` (its tests read no file:
  `CR8:953-954`'s finding, and `native-theme-derive/src/` holds no file read).

Whether a declared step is cached is the reach's verdict (`CR8:761-804`, with C.2's list
and C.4's scan). At `0fa28027`, under C.4's code-only scan, `native-theme-gpui`,
`native-theme-iced` and `native-theme-egui` reach `cc` and `pkg-config` (`cargo tree` as
in A.3), so their declared steps are refused and run in `--commit` every time (their
docs steps carry a `--quick` key); the other seven reach no probe, only listed registry
build code (C.2) and only resolvable includes (C.4). So on an unchanged tree `--commit`
skips exactly these 34 steps:

- `check (<c>)` for `native-theme`, `native-theme-derive`, `native-theme-test-fixtures`,
  `native-theme-build`, `native-theme-showcase-support`, `native-theme-egui-widgets`,
  `native-theme-os-probe` (7);
- `clippy (<c>)` for the same seven, `clippy (native-theme, Linux features)`,
  `clippy (native-theme-egui-widgets, all features)` (9);
- `strict-panic (native-theme, Linux features)`, `strict-panic (native-theme, no
  features)`, and `strict-panic (<c>)` for derive, build, egui-widgets, the OS probe,
  test-fixtures, showcase-support (8);
- `docs (native-theme)`, `docs, private items (native-theme, Linux features)`, `docs,
  private items (<c>)` for derive, test-fixtures, build, showcase-support, egui-widgets,
  the OS probe, and `docs (native-theme-egui-widgets, all features)` (9);
- `test (native-theme-derive)` (1).

Not keyed by the strict cache, each running in `--commit` every time: every other test
step (`native-theme`'s read the desktop, `native-theme-build`'s run `cargo check` on a
fresh project against crates.io, showcase-support's read `scripts/` at run time, the
connectors' read the desktop and run `cargo metadata`; in `--quick` the `--quick` key
decides for them); the cross-target steps (the reach is the host's, and a target's own
dependencies, which may build C code, are not in it); the feature combinations (one
step over every crate, `CR:443`, so its reach holds the connectors' probes); the three
`examples (<crate>)` steps (`CR:861-873`: `cargo build --examples` for gpui, iced and
egui, the crates with an `examples/` folder, each reaching `cc` and `pkg-config`, so
declaring them would change nothing; they also link, which no other cached step does);
the MSRV
checks (M.2); the one-`skrifa` check; the Python checks (decision 12); the audit (it reads
an external database); the self-test; `cargo fmt --all`, which writes the tree.

### C.4 The included files (`scripts/step_cache_includes.py`)

Given package directories (and which of them `CACHE_TRUSTED_BUILD_CODE` names), the
helper reads every `.rs` file `find -L <dir> -type f -name '*.rs'` lists under them,
ignored files included and symbolic links followed (Cargo compiles a `mod` file whether
git ignores it or not, rustc follows a link, and `step_cache_inputs` hashes a directory
the same way, `find -L` at `SC8a:69`), blanks comments and
literals with `code_only` (`scripts/check_panic_patterns.py:73`, loaded by path), and
finds in the blanked text each `include_str!`, `include_bytes!`, `include!` and
`#[path = …]`, taking its argument from the original text at the same position. It
prints each target outside the including package, with the including file and line, or
a refusal with its reason; it exits non-zero on a refusal or a file it cannot read. The
helper reads no metadata. Beside it, the reach in `step_cache.sh` (C.1, bash with `jq`,
as `rust_reach_resolve` already reads the metadata, `CR8:779-804`) adds, for each reached
package, every target source (`targets[].src_path`, read from the workspace metadata
`CR` already holds, `CR:315`, `--no-deps`, which lists every workspace package's targets)
outside its directory as an input (none at `0fa28027`).
So the `--quick` key, which uses this helper, stays independent of the full-metadata
read, which only the strict cache's registry list needs (C.2).

- **Inside a `macro_rules!` body**, an `include_str!`, `include_bytes!`, `include!` or
  `#[path]`, a literal one included, is resolved where the macro is invoked, not where it
  is defined (rustc expands it at the call site: in the confirming check's probe of
  2026-11-15, a literal `include_str!("data.txt")` in a macro of `a/mod.rs` invoked from
  `b/c/mod.rs` read `b/c/data.txt`). rustc resolves the include where the outermost macro
  is invoked, so a macro invoked from another macro's body, or re-exported under another
  name (`use … as`), reads a file beside the caller (a rustc probe of 2026-11-15: a macro
  of `a/mod.rs` invoked only from another macro's body, and one re-exported by `use … as`,
  each invoked from `b/c/mod.rs`, read `b/c/data.txt`). So each is accepted, by the rules
  below, only where no attribute before the macro names `macro_export` (a `cfg_attr` one
  included); the identifier `<name>` occurs in the package's code only in the defining
  file (any occurrence, not only `<name>!`); and no occurrence there lies inside another
  `macro_rules!` body. There call site and definition share a directory; otherwise it is
  refused. A code-only scan of the tracked `.rs` files for
  this document (2026-11-15) finds one such include, `native-theme/src/presets.rs:49`,
  and no `#[path]` in a macro body; that one is accepted (below), so C.3's 34 and A.2's
  29 stand.
- **Accepted:**
  - a string literal;
  - a `concat!` whose parts are string literals without `..`, optionally led by
    `env!("CARGO_MANIFEST_DIR")`, or by `env!("OUT_DIR")` in a package
    `CACHE_TRUSTED_BUILD_CODE` names;
  - inside a `macro_rules!`, a `concat!` whose other parts
    are metavariables declared `$name:literal` in the matcher, where the first part is a
    string literal (a relative path, so the join is never absolute), no literal part
    holds `..`, no attribute before the macro names `macro_export` (a `cfg_attr` one
    included), the identifier `<name>` occurs in the package's code only in the defining
    file (any occurrence, not only `<name>!`), no occurrence there lies inside another
    `macro_rules!` body, and every string literal in those invocations
    is free of `..` and `\`. This is the form
    `native-theme/src/presets.rs:49` uses (`include_str!(concat!("presets/", $key,
    ".toml"))`; the macro at `:34` is not exported, its matcher declares `$key:literal`
    (`:35`), and its one invocation (`:56`) passes keys such as `"kde-breeze"`, none
    with `..`; the identifier `bundled_presets` occurs in the package's files only in
    `presets.rs`, in the definition and that invocation, and the invocation lies inside
    no `macro_rules!` body: `git grep` of the package at `0fa28027`, 2026-11-16). Without it, `native-theme`'s reach, and every reach holding
    `native-theme`, would be refused, and both keys would skip almost nothing.
    **Weighed:** the simpler check, every string literal in each invocation free of `..`
    and `\`, without reading the matcher, is not enough alone: a metavariable declared
    `$name:expr` (or `:ident`, `:tt`) can receive a constant's name or a macro call whose
    value no literal in the invocation shows. Mapping each invocation's tokens to the
    metavariable would be the exact form but needs a `macro_rules!` matcher parser. The
    fragment `:literal` guarantees the value is one of the invocation's literals, so the
    matcher's fragment plus the invocations' literals is exact for this form at the cost
    of reading one declaration. Chosen: both.
- **Resolved:**
  - a literal against the including file's directory;
  - every `#[path]` against the including file's directory, whether or not it sits in an
    inline module block. This is safe without modelling rustc's rule for inline blocks:
    every `.rs` file of a reached package is scanned anyway, so a target inside the
    package is covered however rustc resolves it; and rustc resolves a path inside an
    inline block against a deeper directory (the inline modules' names added below the
    file's directory), so where this resolution stays inside the package, rustc's stays
    inside too. The rule can only err towards refusing a target outside the package
    (below) that rustc would have found inside. Every `#[path]` at `0fa28027` sits
    outside an inline block (`native-theme/src/lib.rs:274`,
    `connectors/native-theme-gpui/src/widgets/tests.rs:24`, the gpui showcase's
    `tests.rs:65`), and each resolves inside its package;
  - a `concat!` led by `env!("CARGO_MANIFEST_DIR")` against the package's directory;
    one led by `env!("OUT_DIR")` names the trusted build script's output, outside the
    tree, and adds no input; the metavariable form lies under its first part's
    directory, inside the package.
- **Refused** (the reach is refused, the step runs):
  - every other `include_str!`, `include_bytes!` or `include!` argument: a `concat!` with
    any other part, a metavariable outside that form (another fragment kind, a macro
    that fails the test of the next bullet, a literal with `..` or `\`), any
    other macro call, an identifier;
  - any include or `#[path]` inside the body of a `macro_rules!`, a literal one
    included, unless no attribute before the macro names `macro_export` (a `cfg_attr`
    one included); the identifier `<name>` occurs in the package's code only in the
    defining file (any occurrence, not only `<name>!`); and no occurrence there lies
    inside another `macro_rules!` body;
  - a literal path holding `\` (an escape or a Windows separator, which the join would
    read otherwise than rustc);
  - a `#[path]` or `include!` target that lies outside the including package: its own
    `mod` declarations and includes are code the scan, which reads the reached packages'
    directories, would not follow (the one exception is `env!("OUT_DIR")`'s generated
    file of a trusted package, which its build script writes from the package's own
    files, C.2);
  - an `include!` of a file not ending in `.rs` (Rust code the scan would not read as
    such);
  - every `#[cfg_attr(…, path = …)]`.

At `0fa28027` none of these refusals occurs (the macro-body one checked by the scan
above): every `#[path]` and `include!` target is a
`.rs` inside its package or a trusted `OUT_DIR` file, and no include literal holds `\`;
so C.3's 34 and A.2's 29 hold.

Its self-test (`--self-test`, run by `step_cache.sh`'s): a literal include outside the
package found; an include in a line comment, a block comment, a doc comment and a string
literal ignored; a `concat!` with `..` refused; `env!("CARGO_MANIFEST_DIR")` accepted;
`env!("OUT_DIR")` accepted for a trusted package and refused for another; the
metavariable form accepted, and with an unrelated `"/tmp/…"` literal elsewhere in the
same file still accepted; refused where the macro is `#[macro_export]`, where the
metavariable is `$name:expr`, where the macro is invoked from a second file, and where an
invocation passes a literal with `..` or `\`; a literal `include_str!` in a macro body
refused where the macro is invoked from a second file, refused where the macro is
`#[macro_export]`, refused where the macro is invoked only from another macro's body,
refused where the macro is re-exported by `use … as`, and accepted where it is invoked
only in its own file; a literal path
with `\` refused; a
`#[path]` and an `include!` to a `.rs` outside the package refused; an `include!` of a
`.txt` refused; `#[cfg_attr(test, path = "x.rs")]` refused;
`#[path]` resolved against the including file's directory, and one whose target lies
outside the package refused; an ignored `.rs` file (listed by `find`, not by git)
scanned, and one reached through a symbolic link; an unreadable file fails (skipped
with a note where the user can read it anyway, as `SC8:823-826` skips an unwritable
folder).

The scan at `0fa28027` (code only; the lines whose target leaves the including package):

| Included path | Included by |
|---|---|
| `Cargo.toml` | gpui (`src/compat.rs:24`, the showcase's `tests.rs:17863`), iced (`src/compat.rs:22`), egui (`src/compat.rs:29`) |
| `docs/COMPATIBILITY.toml` | gpui (`src/compat.rs:30`), iced (`src/compat.rs:28`), egui (`src/compat.rs:35`) |
| `docs/showcase-elements.toml` | iced (`examples/showcase-iced.rs:21496`), showcase-support (`src/elements.rs:50`, `src/dump.rs:99`) |
| `docs/inheritance-rules.toml` | `native-theme` (`src/resolve/inheritance.rs:370`, `:412`) |
| `docs/platform-facts.md` | `native-theme` (`src/presets/documented_sizes.rs:55`, `tests/platform_facts_xref.rs:22`) |
| `docs/property-registry.toml` | `native-theme` (`src/model/mod.rs:2058`) |
| `scripts/capture_window.sh` | showcase-support (`src/lib.rs:118`) |
| `connectors/native-theme-gpui/examples/showcase-gpui/pages/basic.rs`, `…/demo.rs`, `…/pages/mod.rs` | iced (`examples/showcase-iced.rs:21493`, `:21675`, `:21676`) |
| `connectors/native-theme-egui/examples/showcase-egui/pages/basic.rs` | iced (`examples/showcase-iced.rs:21495`) |

So the keys reaching `native-theme` add `docs/inheritance-rules.toml`,
`docs/platform-facts.md` and `docs/property-registry.toml`; those reaching
showcase-support add `docs/showcase-elements.toml` and `scripts/capture_window.sh`; those
reaching egui (egui's and egui-widgets', whose reach holds egui), gpui or iced add
`docs/COMPATIBILITY.toml`, and iced's the four showcase files; the root `Cargo.toml` is in
`rust_config_inputs` already. Since egui-widgets' strict key is accepted, a
`docs/COMPATIBILITY.toml` edit re-runs its six strict steps.

### C.5 Records

`target/check-release-cache/` (the strict set) and `target/check-release-cache/quick/`
(the `--quick` set), one file per step (`SC8a:102-124`), in the worktree's own
`target/`. The full run never reads or writes them (`STEP_CACHE` empty, `CR8:927-931`);
`--no-cache` runs every step and neither reads nor writes either.

### C.6 The self-test, and its place

`run_check "step cache: self-test" bash scripts/step_cache.sh --self-test`, right after
the panic scan's self-test (`CR:408`; `CR8:1322`'s place) and before the first cached
step, in `--commit` and `--quick`; not in the full run (O1b answered no).

## Part D — what comes from `v0.8.0-rc1`, and the texts

### D.1 Ported, adapted, written anew, left out

| Block | Source | Here |
|---|---|---|
| `msrv_checks`, its section and the header's sentence | `CR8:888-925`, `:1334-1335`, `:54-55`; `350ee147`, `c36a874f` | ported (Part M) into the sourced `scripts/msrv_checks.sh`; its `PLAN_EXCLUDED` loop (`CR8:903-909`, `649209d0`) left out; the check against `ci.yml`, the toolchain check and the 1.88.0 build folder (M.1 to M.3) new |
| argument parser, mode exclusion | `CR8:132-156` | adapted: no `--cache-keys`; `--no-cache` alone refused; `--quick` sets `COMMIT_MODE` too |
| title, banner and closing note for `--quick` | `CR8:867-873`, `:1199-1220` | adapted: the summary is the common one (A.1), not `--quick`'s early exit |
| `--quick`'s step list | `CR8:1156-1221` | left out: replaced by A.2's key |
| the `--quick` key, its base, `QUICK_UNREAD`, its environment, its record set | — | new (A.2, C.1) |
| `cargo_tests`, `doctests_selected`, nextest's counting | `CR8:513-552`, `:569-595`, `:642-661` | ported; `cargo_tests` refuses `--` arguments instead of translating one form, and passes `RUST_TEST_THREADS` on |
| `nextest_filter_of`, `GATE_SKIP`, `gate_skipped_under_nextest` | `CR8:597-640`; `ecde0fc3`, `7430a0a2` | left out: the facts gate is 0.8.0's |
| `step_cache.sh`, worktree half | `SC8a`; `SC8:176`, `:320`, `:331`, `:526-530` | ported, with the local pipe rule (C.1) and the new parts and cases of C.1; the standalone entry (`bash scripts/step_cache.sh --self-test`) keeps `SC8a`'s `set -euo pipefail`, and cases 3 and 5 run their assertions without it, in the gate's context |
| `step_cache_verdict_environment`'s form rule | `SC8:239-243` | its thread-count rule reused in `step_cache_quick_environment` (a deny-list, not 0.8.0's allow-list) |
| `step_cache.sh`, shared half | `SC8:87-171`, `:262-281`, `:359-381`, the `--shared` branches of `:390-486`, `:608-835`; `80b722b8`, `46d66d93`, `1e970f6a` | left out (decision 15); the `--shared` option form reused for `--quick` |
| `!<pattern>` exclusion | `SC8:19-21`, `:288-308`'s `excluded`, `:341-357`; `a0ea8218` | left out: no 0.7.x step needs it |
| `step_cached`, `step_passed` | `818f0eab`; `CR8:708-709` | ported in `818f0eab`'s form, with `46d66d93`'s warning, extended by A.2 |
| `step_key_of`, `CACHE_SHARED`, `--cache-keys` | `CR8:713-738`, `:1051-1062`, `:115-118`; `80b722b8` | left out |
| `cache_inputs`, `rust_reach`, `rust_config_inputs`, probes, trusted code | `CR8:740-821`, `:985-986` | moved into `step_cache.sh` and adapted (C.1, C.2: the `--quick` form, the reviewed registry list, C.4, the clippy and nextest configuration files) |
| the include helper | — | new (C.4), on `check_panic_patterns.py`'s `code_only` |
| `CACHE_INPUTS` | `CR8:993-1029` | adapted: 0.7.x's steps (C.3); 0.8.0's data checks, facts gate, `EVIDENCE_PIN`, `PY_DATA_INPUTS` left out |
| `cache_tool_versions`, cache start, `print_cached`, summary counts | `CR8:1065-1093`, `:384-392`, `:2407`, `:2433-2434`, `:2457` | ported; the full run's summary line unchanged |
| `PLAN_EXCLUDED`, `plan_excluded_step`, the plan-exclusions section and checks | `CR8:197-331`, `:462-490`, `:1223-1244`, `:2392-2401`, `:2420-2422`, `:2451-2455`, `:2475-2477`; `649209d0` | left out: 0.8.0's transition |
| the facts gate's parts, `NT_FACTS_GATE_*` unset, `platform_facts_generated`, `check_gate_cfg.py`, the ledger and specification checks | `CR8:160-166`, `:1110-1151`, `:1323-1324`, `:1179-1195`; `a0ea8218`, `80b722b8`, `246dacaf`, `baee1666` | left out: 0.8.0's own; the three `check_spec_*.py` scripts present on 0.7.x check the parked rebuild's documents and no 0.7.x gate runs them |

### D.2 The script's header and usage

- Header (`CR:26-41`): the Modes list gains, after `--commit`'s entry: `--quick  the
  per-task gate: --commit's steps, but a test, docs or rust-version MSRV step is skipped where its key
  is unchanged since it last passed in --commit or --quick: its crates' reach and the
  files they include, the files outside the crates but those QUICK_UNREAD lists, the tools, the
  environment but its thread counts, its output, location and speed settings and the
  per-session variables of a terminal or session, the gate's scripts and its command. Its summary names the steps it left to --commit.`
  `--commit`'s entry gains "Run every third code task (unit), counted from the last
  --commit, on a release's last code task, after a lock or toolchain change and where a
  plan names a task structural.", and `--quick`'s "Run at a task's end." Then M.2's MSRV
  sentence and a paragraph "Speed in --commit and --quick, never in the full run", as
  `CR8:57-70` without its facts-gate and `--cache-keys` sentences and with "the
  showcases' tests in CI's runner environment stay on cargo test".
- Usage (`CR:46-62`) becomes:

```
Usage: scripts/check_release.sh [--commit | --quick] [--no-cache]

  (no argument)  the full pre-release check; required on the last commit of
                 every fix phase and before the release commit
  --commit       the per-commit subset: every lint, check, test, docs,
                 widget-coverage, showcase-parity and archive-index gate,
                 CI's MSRV checks and the security audit (offline: the cached
                 advisory database, a warning); skips the release-tag check,
                 the gpui connector's tests on GPUI Fast, native-theme's
                 ignored stem-index test, docs.rs emulation, packaging,
                 semver checks, outdated dependencies, and the visual-asset
                 and compatibility stamp checks; run every third code
                 task (unit), counted from the last --commit, on a release's
                 last code task, after a lock or toolchain change and where
                 a plan names a task structural
  --quick        the per-task gate: --commit's steps, but a test, docs or
                 rust-version MSRV step is skipped where its crates, what
                 they build against and include, the other files outside
                 the crates but those QUICK_UNREAD lists (step_cache.sh),
                 the tools, the environment and its command are unchanged
                 since it last passed in --commit or --quick; the summary
                 names what it left to --commit; run at a task's end
  --no-cache     with --commit or --quick: run every step, no cache read or
                 written (the full run never uses one)
  -h, --help     print this help and exit

With --commit or --quick, cargo nextest runs the test steps where it is
installed (cargo test --doc the doctests; the showcases' runner-environment
tests stay on cargo test), and a compile-only or documentation step, or
native-theme-derive's tests, whose declared inputs, tools, environment and
command are unchanged since it last passed is skipped, named in the summary
"unchanged since <commit>". The full run runs every step through cargo test,
with no cache.
```

  Before commit (iv) the `--quick` line and `--quick`'s mentions are absent. The rule
  holds for every text a commit writes, its subject included: a commit names `--quick`
  only where its tree has it, so (i), (ii) and (iii), landing before (iv), say
  "`--commit`" where this specification says "`--commit` and `--quick`" (M.1's comment
  in `scripts/msrv_checks.sh`, M.2's header sentence, B.1's `HAVE_NEXTEST`, the speed
  paragraph, the README's and CONTRIBUTING's texts), and (iv) adds `--quick` to each.
  The cadence texts land in (iv) too, since they name `--quick`'s place beside
  `--commit`: the header's `--commit` entry ("Run every third code task …"), the usage's
  `--commit` line's cadence clause, D.3's "`The subset runs before every commit` …
  becomes the cadence of D.2's usage" and D.4's "The script has three modes."

### D.3 `scripts/README.md`

- `## check_release.sh` (`:303-332`): the first paragraph's list of checks (`:305`,
  "The workspace's quality gate: source hygiene, `cargo check`, …") gains "CI's MSRV
  checks (`ci.yml`'s `msrv` job, on its two toolchains) and the check that they are the
  job's" after "`cargo check`", and "the step cache's self-test" after "source hygiene"
  (the panic scan's section, `CR:372-408`, where the self-test runs, C.6), each said to
  run in `--commit` and `--quick` (O1 answered no); after the `--commit` paragraph, a `--quick` paragraph with
  A.4's content (what it runs, what it leaves to `--commit`, what its key follows and
  what it does not) and `818f0eab`'s "Two speed-ups" paragraph (its `scripts/README.md`
  hunk), without the later version's facts-gate and `--cache-keys` sentences, its cache
  sentence reading "the step cache skips a compile-only or documentation step, or
  native-theme-derive's tests, whose declared inputs are unchanged since it last
  passed"; `The subset runs before every commit` (`:325`) becomes the cadence of D.2's
  usage; the code block gains `./scripts/check_release.sh --quick` and
  `./scripts/check_release.sh --commit --no-cache`.
- A new section after it, `## step_cache.sh`: `818f0eab`'s text (its hunk), with the
  0.7.x facts: the strict key's base (the tool versions, the hashes of the gate's five
  scripts, `check_release.sh`, `step_cache.sh`, `step_cache_includes.py`,
  `check_panic_patterns.py` and `msrv_checks.sh`, the last in the list from commit
  (iii), the environment),
  the `--quick` key (A.2: its base, its environment's deny-list, `QUICK_UNREAD`, its set
  in `target/check-release-cache/quick/`), the reach (the refusals, the reviewed
  registry list, the included files), the connectors' strict steps refused by their
  probes, the self-test's cases; a short section `## step_cache_includes.py`: C.4's
  rules and `python3 scripts/step_cache_includes.py --self-test`; and a short section
  `## msrv_checks.sh`: Part M (the toolchains read, the six checks, `msrv_ci_check`, the
  toolchain check, where 1.88.0 builds).
- "both modes" becomes "every mode" at `:320`, `:345-346`, `:612`, `:631`, `:651`,
  `:673`, `:707`; "(in the `--commit` subset too)" becomes "(in `--commit` and `--quick`
  too)" at `:494`, `:545`, `:584-585`; at `:771-772` "Both modes run them all but the
  GPUI Fast leg, which `--commit` skips" becomes "Every mode runs them all but the GPUI
  Fast leg, which `--commit` and `--quick` skip; `--quick` skips a leg whose key is
  unchanged since it last passed".

### D.4 `CONTRIBUTING.md`

- `:19-30`: "The script has three modes." At a task's end, `--quick` (one sentence of
  what it runs, A.4); every third code task (unit), counted from the last `--commit`, on
  a release's last code task, after a lock or toolchain change and where a plan names a
  task structural, `--commit`; on the last
  commit of every fix phase and before the release commit, the full run. Three code
  blocks, as today's two.
- `:32-36`: the full list gains "the step cache's self-test
  (`scripts/step_cache.sh --self-test`)" after the panic scan, and after "per-crate
  `cargo check --all-targets`," "CI's MSRV checks (`ci.yml`'s `msrv` job: the
  workspace's `rust-version` and the connectors' toolchain, with
  `RUSTFLAGS=-Dwarnings`; a toolchain not installed fails) and the check that they are
  the job's," each said to run in `--commit` and `--quick` (the full run lacks them, O1
  answered no).
- After `:98-109`: a paragraph on `--quick` (A.4) and `818f0eab`'s paragraph on nextest
  and the cache (its `CONTRIBUTING.md` hunk), its cache clause as D.3's, ending "The full
  run uses neither: every step runs, as listed above, through `cargo test`."
- `:105`: "runs in both modes" becomes "runs in every mode".

### D.5 `CHANGELOG.md`

The 0.7.1 section's `### Added` (`CHANGELOG.md:10`) gains, after its last entry, a
`**Development**` entry in the form the 0.7.0 section uses for the gate
(`CHANGELOG.md:185-186`):

```
- **Development**: `scripts/check_release.sh --quick`, the per-task gate: it runs `--commit`'s checks, but skips a test or documentation step where the crates it tests, what they build against and include, the repository's other files but those no crate reads (its Markdown documents, the CI workflows, the licences, the git ignore and attribute files, and the archived, issue, asset and evidence folders under `docs/`), the tools, the environment and the step's command are unchanged since that step last passed. `--commit` and `--quick` run their test steps through `cargo nextest` where it is installed (the doctests through `cargo test --doc`), skip a compile-only or documentation step, or native-theme-derive's tests, whose inputs, tools, environment and command are unchanged since it last passed (`scripts/step_cache.sh`; `--no-cache` runs every step), and run the MSRV checks of CI's `msrv` job, of which `--quick` skips the two at the workspace's `rust-version` where everything their key follows, their toolchain among it, is unchanged since they last passed.
```

## Part E — tests of the change

### E.1 The self-tests

`bash scripts/step_cache.sh --self-test` (C.1, the helper's C.4 cases among them) exits
0, every case `ok`; each tool is also seen to fail once broken on purpose, in a scratch
copy, the failure quoted in the record: `step_cache_key` without the command;
`step_cache_quick_environment` keeping `RUST_TEST_THREADS=4`; the helper accepting a
`concat!` with `..`; the pipe rule broken in `step_cache_inputs`, in two forms, each made
in its own scratch copy and case 3 seen to fail on each: (1) `|| return 1` removed, the
local `pipefail` kept; (2) the local `pipefail` removed, `|| return 1` kept. The two forms
are one break.

### E.2 The stub harness

A scratch script in the work folder (not committed) runs `CR` with a stub `cargo` and a
stub `python3` first on `PATH`, so that a run takes about a minute and every command is
logged. The harness, its stubs, their log and its outputs live in the plan's work
folder outside the worktree (or under the worktree's `target/`, which git ignores), never
in a path the `--quick` base hashes (A.2: every file `git ls-files -co
--exclude-standard` lists outside the packages); otherwise a log written during a run
would change the base it computes. The `_before` copies (below) are the one exception:
they exist under `scripts/` only while E.3 (c) and (d) run, and never during the (a) to
(a7) series, whose `--quick` bases they would change. The stubs (`cargo`, `python3`, and
for E.3 (a7) `rustc`, for E.3 (r) `rustup`) live in one stub folder the harness puts first on `PATH` for every
run of a series, so that `PATH`, which the `--quick` key holds (C.1 case 4), is the same
in every run whichever stubs the folder holds.

- the stub `cargo` finds the subcommand after a leading `+<toolchain>` and any
  `--config <value>` pairs (`scripts/run_in_runner_env.sh:303` runs `exec cargo --config
  "target.$HOST.runner = …" "$@"`, through `PATH`, so the runner-environment legs reach
  the stub too); it runs the real one for `metadata`, `tree`, `pkgid`, `-vV`, `-V`,
  `--version`, `clippy -V` and `nextest --version`; for anything else (Part M's
  `+<toolchain> check` among them) it appends its arguments, one line, to the log,
  prints `test result: ok. 1 passed; 0 failed` for `test`, `1 test run: 1 passed` for
  `nextest run`, nothing otherwise, and exits 0; one variant fails only the reach's form
  of `tree` (`--prefix none --format {p}`, the call `step_cache_cargo_tree` makes; exit 1)
  and passes the one-`skrifa` form (`tree -p native-theme-egui --target all -d`,
  `CR:765-766`) to the real cargo, so that step does not fail the run;
  one, for E.3 (b2), edits a named file while it runs a named command; and one, for E.3
  (t), which on a `+1.88.0 check` logs whether its `--target-dir` folder exists, then
  runs `mkdir -p` on that value and writes one file into it, then logs the command as
  usual, and, where named, exits 1 for one named command;
- the stub `python3` runs the real one, without logging, for `-c`, `-VV` and for both
  `scripts/step_cache_includes.py` and `scripts/step_cache_includes_before.py` (the
  "before" copy, below), found wherever the script's name stands after interpreter
  flags such as `-I`; for any other script it logs and exits 0. Otherwise the "before"
  run's self-test and keys would call a stubbed helper, C.1 case 6 and the helper's own
  case would fail, and E.3 (d)'s "(iii) and (iv) none" would fail for a reason no commit
  made;
- it moves `target/check-release-cache/` aside before a series of runs and back after
  (a `trap`), and compares the folder's hash (`find … -type f -print0 | sort -z |
  xargs -0 sha256sum | sha256sum`) before the move and after the restore: equal, so no
  record of a stubbed pass survives; the moved-aside folder's hash is also compared
  before and after each single run of a series where that run must not write (the
  `--no-cache` cases); and, for E.3 (c) and (d), it empties `target/check-release-cache/`
  before each single run, the "before" run and the new one alike, so that neither finds
  the other's records (the other checks keep the series' records where they test them);
- the "before" script is the gated commit's parent's (`git show
  <parent>:scripts/check_release.sh`, with the parent's other gate scripts the same way),
  written beside the real ones under a `_before` suffix for the run and removed after
  (it changes to its own directory's parent, `CR:73`). The harness rewrites in those
  copies every path that names another gate script, so that the "before" run uses the
  "before" scripts throughout: `check_release_before.sh`'s `source scripts/step_cache.sh`
  and `source scripts/msrv_checks.sh` lines and its self-test line, `bash
  scripts/step_cache.sh --self-test` rewritten to `bash scripts/step_cache_before.sh
  --self-test` (each where the parent has it), and `step_cache_before.sh`'s path of
  `scripts/step_cache_includes.py`, each to its `_before` name; the rewritten lines are
  listed in the record. The lexer, `scripts/check_panic_patterns.py`, is loaded by the
  helper itself and no commit of GS changes it, so it needs no `_before` copy and its
  path is left as it is. At GS's end, once more `0fa28027`'s script, for the comparison of
  the whole unit. Every run compared sees the same tree and the same untracked files.
- before a series, it runs `scripts/run_in_runner_env.sh --prepare` and stops where it
  does not exit 0 (the legs would otherwise be skipped by an outage, exit 75, or fail,
  and the logs would differ for a reason no commit made).

### E.3 The mechanical checks

Each is a command in the work folder whose output goes into the record. "The 29" are
A.2's quick-keyed test and docs steps; "the 34" C.3's strict-cached steps.

- **(a) a gpui-only change**: a stubbed `--commit` (it records the quick-keyed steps in
  `target/check-release-cache/quick/`, 31 files (the 29 and the two `+1.88.0` MSRV
  checks, A.2), and the 34 in the strict set; otherwise the check stops), then a comment
  appended to `connectors/native-theme-gpui/src/lib.rs`, a stubbed `--quick`, the edit
  reverted. The log's test and doc lines are exactly: `test -p native-theme-os-probe --
  --list` (the probe's listing, `CR:670`, runs every time); a `nextest run
  --no-tests=pass … -p native-theme-gpui --features native,watch` line and its `test -p
  native-theme-gpui --features native,watch --doc` line (gpui's library is doctested); a
  `nextest run … -p native-theme-gpui --lib --no-default-features` line, with no `--doc`
  (`--lib` selects a target); the two gpui runner-environment `test` lines; `doc -p
  native-theme-gpui --no-deps --document-private-items`. Its MSRV lines are exactly the
  four `+1.95.0 check …` (M.1's last four rows, never keyed); the two `+1.88.0` checks are
  skipped (their keys hold no file the edit touches; of gpui's files, only the three
  showcase files iced includes, C.4). The summary's `--quick` list names the
  other 24 of the 29 and the two `+1.88.0` checks (26); its strict list all 34.
- **(a2) to (a7)** each start from (a)'s state again, so no case depends on the one run
  before it: each runs its own stubbed `--commit` with the normal stub on the reverted
  tree first; then the change, a stubbed `--quick`, the change reverted. The four `+1.95.0`
  checks run in each.
  - **(a2)** `connectors/native-theme-gpui/examples/showcase-gpui/pages/basic.rs`: gpui's
    five and iced's seven run (`test (native-theme-iced)`, `… no features`, `… iced_aw,
    watch, native`, its two runner-environment legs, `docs, private items
    (native-theme-iced)`, `docs (native-theme-iced, all features)`), and the `+1.88.0`
    workspace check (its reach holds iced, which includes the file, C.4); 17 test and docs
    steps and the `+1.88.0` `native-theme` check in the `--quick` list (18), 34 in the
    strict list.
  - **(a3)** `docs/todo.md`: none of the 29 and neither `+1.88.0` check runs; 31 listed,
    and 34.
  - **(a4)** `native-theme/src/lib.rs`: 27 of the 29 run, `test
    (native-theme-test-fixtures)` and `test (native-theme-os-probe)` listed, and both
    `+1.88.0` checks run (each reach holds `native-theme`); of the 34, the 21 reaching
    `native-theme` run and 13 are listed (derive's five, test-fixtures' four, the OS
    probe's four).
  - **(a5)** `docs/showcase-exceptions.toml` (outside the packages, not unread, in no
    strict input): all 29 and both `+1.88.0` checks run; 34 listed.
  - **(a6)** the stub variant that fails the reach's `tree` (E.2) used for the `--quick`
    only, its `--commit` run with the normal stub (the one-`skrifa` step passes through to
    the real `cargo tree` and passes): all 29, both `+1.88.0` checks and all 34 run, the
    reason printed once per crate set, and the run exits 0.
  - **(a7)** a toolchain change in the MSRV key: the tree unchanged, a stub `rustc` put
    into E.2's stub folder (already first on `PATH`, so `PATH` itself does not change)
    that passes every call to the real one but `rustc +1.88.0 -vV`, whose real output it
    prints with its `commit-hash:` line changed: both `+1.88.0` checks run and the 29 are
    listed. With the stub removed from that folder, the next `--quick` runs both once more
    (their last pass was recorded under the stub's version), and a third lists them.
- **(b) the strict cache on an unchanged tree**: two stubbed `--commit` runs; the
  second's `unchanged since` list is exactly the 34, by description, and its log holds no
  command of them; every other step ran.
- **(b2) the strict cache's misses**, stubbed, after (b), one change at a time, each
  reverted and the cache brought back to (b)'s state before the next:
  - a comment in `native-theme-derive/src/lib.rs`: 26 of the 34 run (derive's five,
    `native-theme`'s seven, build's four, showcase-support's four, egui-widgets' six);
    test-fixtures' four and the OS probe's four stay unchanged;
  - a comment line in `docs/property-registry.toml` (C.4): 21 run (`native-theme`'s
    seven, build's four, showcase-support's four, egui-widgets' six); derive's five,
    test-fixtures' four, the OS probe's four stay;
  - a comment line in the root `Cargo.toml`: all 34 run. `Cargo.lock` is in the same
    inputs (`rust_config_inputs`) but is not edited for this case: an edited lock is what
    `cargo tree --locked` may refuse, which would show the reach's refusal, not a key's
    miss;
  - a comment in `scripts/check_strict_panic.sh`: exactly the eight `strict-panic`
    steps run;
  - a file changed while its step runs: the record of `check (native-theme-derive)`
    deleted; the stub edits `native-theme-derive/src/lib.rs` while it runs `check -p
    native-theme-derive`; after the run no record file of that step exists; the edit
    reverted; the next run runs the step (and records it). The same for the `--quick`
    set: `test (native-theme-build)`'s record deleted, the stub editing
    `native-theme-build/src/lib.rs` while it runs that step's `nextest run`, in a
    stubbed `--quick`.
- **(c) the full run**: a stubbed full run of the "before" script (E.2) and of the new
  one, at each commit's gate. Each commit's expected difference is its own, so the order
  of the commits (Part F) does not change them. With O1a and O1b answered no, at every
  commit: the two logs are equal byte for byte, and the two outputs, each reduced to its
  status lines without their times, are equal. Nothing else differs: every step of the
  "before" output is in the new one, in the same order, with the same description and
  status. At GS's end, against `0fa28027`: the same, +0. (The +7 and +1 forms that O1a
  (a) and O1b (a) would have needed are in *Under another outcome*, not taken.)
- **(d) `--commit`'s commands**: a stubbed `--commit` of each script, with empty caches,
  at each commit's gate. `0fa28027`'s `--commit` log holds 26 `test` lines: the 18 test
  steps B.3's eleven commands make in `--commit` (`:675` runs for eight crates), the OS
  probe's `--list` line, and the seven runner-environment legs, which the stub logs as
  `--config … test …`. Each commit's own expected difference: (i) the six Part M
  commands inserted right after the last `check -p <crate> --all-targets` line; (ii) each
  of the 18 replaced by `nextest run --no-tests=pass --status-level fail
  --final-status-level fail --color never [--test-threads N] <args>` and, where `doctests_selected`
  holds, `test <args> --doc` after it, the `--list` line and the seven legs unchanged;
  (iii) and (iv) none (with empty caches every step runs, through the same commands).
  At GS's end, against `0fa28027`: (i)'s and (ii)'s differences together.
- **(e) the arguments**: `--commit --quick`, `--no-cache` alone and an unknown argument
  each exit 2 with their message; `--help` exits 0 and prints D.2's usage; `--commit
  --no-cache` after (b) runs all 34 and leaves both sets' hashes unchanged; `--quick
  --no-cache` after (a)'s `--commit` runs all 39 test and docs steps and both `+1.88.0`
  MSRV checks (and the four `+1.95.0`, as always), lists none unchanged, and leaves both
  sets unchanged.
- **(m) the MSRV check against `ci.yml`**: M.1's check passes on `0fa28027`'s `ci.yml`
  and fails, naming the difference, on scratch copies with one `run:` line's arguments
  changed, with a seventh `run: cargo check` line, with a `run: cargo clippy …` line
  added, without `RUSTFLAGS: -Dwarnings`, with a second job-level `env:` variable added,
  with a step-level `env:` under one `run: cargo check` step, with an `if:` on one
  step, with a second workflow-level `env:` variable, with a top-level `defaults:` key,
  without the `msrv` job, with five `run: cargo check` lines (one removed),
  with the first toolchain changed (`@1.88.0` to `@1.89.0`, no longer `Cargo.toml`'s
  `rust-version`), and with the second toolchain's version emptied; and passes, its six
  steps' toolchain following, on a copy with the second toolchain changed (`@1.95.0` to
  `@1.96.0`, which the script reads), on a copy with the `msrv` job moved to the file's
  end (the range then ends at the file's end), and on a copy whose next job,
  `test-build-crate`, gains a seventh `- run: cargo check -p native-theme --locked` line
  (the range stops at that job's header, so the line is not counted; a range that ran
  on would count seven and fail). Each is `bash -c 'source scripts/msrv_checks.sh && msrv_ci_check <copy>'`,
  run from the repository's root (M.1), its exit status and output in the record.
- **(r) a missing toolchain** (M.2): a stubbed `--commit` with a stub `rustup` in E.2's
  stub folder that passes every call to the real one but `rustup toolchain list`, whose
  real output it prints without the `1.88.0-…` line: the run exits 1 at the first MSRV
  step that needs 1.88.0, its output naming `toolchain 1.88.0 is not installed (rustup
  toolchain install 1.88.0)`, and the log holds no `+1.88.0` command; the stub is
  removed after.
- **(t) the 1.88.0 folder's EXIT trap** (O2 (e)): stubbed `--commit` runs with E.2's
  variant that creates the `--target-dir` folder of a `+1.88.0 check`, both caches
  emptied before each run (as for (c)), the folder being `<build_directory>/msrv-1.88`
  as the gate computes it:
  1. a passing run exits 0, its output holds the `du -sb` line of that folder, and after
     the run the folder does not exist;
  2. the same with a later step failing (the variant exiting 1 for `doc -p
     native-theme-derive --no-deps --document-private-items`, after Part M): the run
     exits 1, and the folder does not exist after it;
  3. a folder planted before the run (with a file in it): the log shows it removed
     before the first `+1.88.0` command (the stub's first `+1.88.0 check` logs the folder
     absent, before it creates it), and the run is as 1;
  4. a run that fails before Part M (the variant exiting 1 for `check -p
     native-theme-derive --all-targets`, in the `cargo check` section): it exits 1, its
     output holds no `du -sb` line, and a folder planted again as in case 3 is still
     there with its file (no trap was set and no removal ran); the harness removes it
     afterwards.
  Each run's exit status, the `du -sb` line where expected and the folder's state after
  it go into the record.

### E.4 The real runs at the unit's end

In the core worktree, through the plans' wrappers, each run of the gate script under
the gate lock (`flock <locks>/gate.lock`, the batch's rule 2), each a long run, each
time and summary into the record:

1. `--commit` with both caches empty, on the tree Part F names for GS's last gate (the
   clean tree at GS's last commit: (iv), the review's last fix, or a fix E.4 made
   necessary):
   GS's last gate (the warm-up gate and the branch's first timed `--commit`, as the GPUI
   plan's GS row has it), exit 0, the summary naming no failure and, as skipped, only
   `--commit`'s skips (A.1: the release-tag check, the GPUI Fast legs, the stem-index test
   and the release-only sections); M.1's check and the six MSRV steps passed, each one's time
   recorded (the first run on the two toolchains builds what the build folder and
   sccache do not yet hold, M.3), and the size of the 1.88.0
   checks' build folder (`du -sb`, in bytes) recorded from the line the gate prints just
   before its trap deletes the folder (M.3, O2 (e)).
2. `--commit` again, unchanged: exit 0; its `unchanged since` list is exactly the 34. Any
   other list is a stop: where none is skipped, an environment variable that changes
   between calls is the likely cause (`step_cache_environment` hashes every variable but
   `_`, `SHLVL` and `OLDPWD`), found by comparing two runs' `env`; any later change of a
   key's contents gets a `review-hard` of its own. The 34 steps' summed time in item 1
   (what this cache saves at most) goes into GS's report to the maintainer, so that a
   cache that saves little can be dropped.
3. nextest against `cargo test`, on the built tree, for `test (native-theme, Linux
   features)`, `test (native-theme-gpui, native, watch)` and `test (native-theme-egui,
   all features)`: each step's command run once as `cargo test <args>` and once as
   `cargo_tests` runs it, each a long run of the orchestrator's, started under the gate
   lock and the same wrappers (`flock <locks>/gate.lock <tools>/slot.sh
   <tools>/lowprio.sh <command>`); the times, and the counts: nextest's passed plus the doctests'
   passed equals `cargo test`'s passed, and nextest's skipped plus the doctests' ignored
   equals `cargo test`'s ignored. Unequal counts are a stop. A nextest total slower than
   `cargo test`'s is reported to the maintainer before any other unit of v0.7.1 runs a
   gate: the rationale's premise for nextest would not hold here.
4. `--quick` after items 2 and 3, the tree unchanged (item 3 runs its commands outside
   the script, so it changes neither the tree nor either record set): the 29 listed under
   `--quick` and the two `+1.88.0` MSRV checks (31), the 34 under the strict cache, the
   four `+1.95.0` checks run (warm), exit 0; its time is the floor of every later
   `--quick`, and only the floor: it shows the four `+1.95.0` checks' warm time, but not
   the two `+1.88.0` checks', which it skips.
   Where the 31 run instead, a variable outside the deny-list that changes between calls
   is the likely cause, found as in item 2 and reported. GS's report to the maintainer
   gives the MSRV checks' recurring cost from three runs: item 1's two `+1.88.0` times
   (under O2 (e) each run of them builds a fresh folder, so that is what a `--quick` pays
   wherever anything their key follows changed); this item's four `+1.95.0` times (warm, the floor every
   `--quick` pays); and the GPUI plan's E1, the first timed `--quick`, a gpui unit, for the
   three `+1.95.0` gpui checks with their crate changed; each beside its run's whole time.
   E1 runs after GS has ended, so the report is sent with items 1 to 5 at GS's end and
   completed with E1's numbers once E1 has run. With those numbers the maintainer may
   revisit O3's options and O2's (a) or (d). No threshold is set here.
5. `--quick` with another session simulated, on the same unchanged tree, through the
   same wrappers and the gate lock of the batch's rule 2, each environment change made
   before them (`env … flock <locks>/gate.lock slot.sh lowprio.sh scripts/check_release.sh
   --quick`, the plan's form). A real second session (a new Claude Code session, a new Konsole
   tab) changes the deny-list's per-session variables (C.1), so the runs change those
   mechanically; this needs neither the maintainer nor the desktop. First
   `target/check-release-cache/` is copied aside, its hash recorded (`find … -type f
   -print0 | sort -z | xargs -0 sha256sum | sha256sum`, E.2's form), since these runs
   record under other environments; after the last run it is put back and its hash
   compared, equal (hygiene: the GPUI plan's LN, which changes a comment of
   `scripts/check_release.sh` and so every key's base, comes before E1 anyway). Then
   three runs, each exiting 0:
   - (A) every per-session variable of the deny-list set to a value other than the
     session's (`env CLAUDE_CODE_SESSION_ID=gs-item5 WINDOWID=1 …`, those the session
     does not set included): the 31 listed under `--quick` again;
   - (B) every one of them unset (`env -u CLAUDE_CODE_SESSION_ID -u WINDOWID …`): the 31
     listed again;
   - (C) the control, one variable on no list added (`env GS_ITEM5_PROBE=1 …`, a name
     that, run before it, `git grep -n GS_ITEM5_PROBE -- ':!docs/'` of the worktree and a
     `grep -r` of the registry's `.rs` files find nowhere; outside `docs/` because this
     specification and its plan name it): none of the 31 listed, all 31 run and pass,
     which shows that the `--quick` key follows a variable outside the deny-list.
   In (A), (B) and (C) the strict 34 run (the strict key holds the whole environment):
   that is recorded, not a stop. What the item cannot test: variables off the deny-list
   that change in other situations than these runs (`XAUTHORITY`, `SESSION_MANAGER`,
   `XDG_SESSION_ID`, `TERM`, `COLORTERM`, `KONSOLE_VERSION`, `CLAUDE_CODE_ENTRYPOINT`);
   they are keyed, so a change of one costs re-runs, never a skipped step, and is no
   fault. Where (A) or (B) runs any of the 31, one run per variable of the
   list finds the one that changed the key, and it is reported (a key change for a
   `review-hard`, E.4 item 2); where (C) lists any of the 31, the `--quick` key misses a
   variable it should follow, which is a stop. The record holds each run's environment
   change, list and time.

## Part F — commits, gates, review

GS is one unit of four commits on `v0.7.1-rc2`, in this order, each with its own texts
(D.2 to D.4 as far as they describe that commit's state). `CHANGELOG.md` gets one
entry, D.5, written whole by (iv); no other commit edits it. The order differs from
0.8.0's (`--quick`, nextest, cache): the `--quick` key is built on the cache's functions,
so `--quick` comes last; the MSRV port, independent of the rest, comes first, so that
(c) shows its steps alone.

| Commit | Content | Its gate |
|---|---|---|
| (i) `scripts(check_release, msrv_checks): CI's MSRV checks in --commit …` | Part M (seven steps, `scripts/msrv_checks.sh`, M.3, O2 (e)); its texts | `bash -n scripts/check_release.sh && bash -n scripts/msrv_checks.sh` (one file per `bash -n`: `bash -n a b` checks `a` alone and reads `b` as its first argument); E.3 (m); E.3 (r); E.3 (t); E.3 (c) (the full run unchanged, O1a answered no); (d) with the six MSRV commands and nothing else added; (e)'s `--help` case (the `--commit` usage line changes) |
| (ii) `scripts(check_release): nextest in --commit …` | Part B (`CR8:513-552` without `step_cached` and `step_passed`); its texts | `bash -n scripts/check_release.sh`; E.3 (c), (d); (e)'s `--help` case (the usage's nextest paragraph changes) |
| (iii) `scripts(check_release, step_cache): the step cache …` | Part C's strict key and the shared parts of C.1 (`step_cache.sh`, `step_cache_includes.py`, the reach, the reviewed list, `cache_tool_versions`, `rust_config_inputs`, the cache folder's creation), the self-test step (C.6), `--no-cache` with its message `--no-cache goes with --commit`; `scripts/msrv_checks.sh` in the strict base's script list (C.2; the `--quick` base takes every script anyway, A.2); its texts | `bash -n scripts/check_release.sh && bash -n scripts/step_cache.sh` `&& bash -n scripts/msrv_checks.sh`; `python3 -m py_compile scripts/step_cache_includes.py`; `bash scripts/step_cache.sh --self-test` with every case of C.1 and C.4 but those of the `--quick` key, which (iv) adds: C.1 case 4 (the `--quick` environment), case 5 (the `--quick` base), case 7's `--quick`-form assertions (a probe and an unlisted registry build script accepted), case 6's `--quick` half (the edited `.md` include target and the `cargo tree` stub failing, in the `--quick` form), case 8 whole (it needs both sets) and case 9's `--quick` half; case 11 (`step_cache_command_crates`) lands with the `+1.88.0` keying, in (iv); case 10 (the full metadata) is in (iii); and E.1's first, third and fourth breaks; E.3 (b), (b2) but its `--quick` half, E.3 (c) (unchanged, O1b answered no), (d), (e)'s `--commit --quick`-free cases (`--no-cache` alone, an unknown argument, `--help`, `--commit --no-cache`) |
| (iv) `scripts(check_release, step_cache): --quick, the per-task gate …` | A.1 to A.4; the `--quick` key, base, environment and set (C.1); under O3's keyed option (chosen), the two `+1.88.0` MSRV steps' `--quick` key (A.2) together with `step_cache_command_crates` and C.1 case 11; `scripts/msrv_checks.sh`, M.1's comment gains `--quick` (D.2's rule), with M.2's keying (its first three bullets); the `--no-cache` message naming `--quick`; its texts; D.5 | `bash -n` on each script, one per call; the whole self-test and E.1; E.3 (m) and (t) (the commit edits `scripts/msrv_checks.sh`); E.3 (a) to (a7) (with their MSRV counts, and (a7)), (b), (b2) whole, (c), (d), (e) whole |

GS starts only once the maintainer has answered O1 to O3 and decision 4, which came with
the approval of this specification and its plan on 2026-11-15; the answers are the
outcome this document is written for, and the plan's Task 0 records them before GS's
first commit. E.4 and the review wait for all four commits.

Then the review (below); its fixes are new commits, each gated by the gates of the
commit it fixes. GS's last commit is (iv) or the review's last fix. E.4's item 1 is GS's
last gate, run on the clean tree at GS's last commit, as the GPUI plan's GS row has it:
it "passes and changes nothing, on the clean tree at GS's last commit (a last fix is
committed first, its content the staged change its own gate passed)"; items 2 to 4
follow on the unchanged tree. O1a and O1b were answered no, so nothing of GS enters the
full run. The subjects
follow the repository's form; each body says what changed and why and quotes its gate's
outputs. No commit changes a file of the GPUI specification's E.2 scope (the GPUI plan
checks this before its E1).

- **By whom**: `implement` (Opus, high), the commits in sequence, against the gates
  above; the orchestrator runs E.4. The stub harness and the checks are written by the
  implementer in the work folder and handed over with the commits.
- **Review**: one `review-hard` over the four commits, given this specification and the
  diff, not the author's conclusion, before E.4 runs. Its brief names what a miss skips
  silently: the two keys and what each follows (A.2, C.1 to C.4), `QUICK_UNREAD`, the
  deny-list, the reviewed registry list, the include rules, the record conditions
  (`step_passed`), the explicit status checks (A.2), Part M's commands and check
  against `ci.yml:266-283`, and the EXIT trap (O2 (e)) and its exit status (O2
  (e)'s rules, E.3 (t)), the folder's path among them: written into the trap's text
  (`printf %q`) or held in a global never unset, not a `local` of `msrv_checks`, which
  under `set -u` (`CR:44`) fails the trap. Its findings are fixed by new commits, and E.3 runs again on
  the fixed tree before E.4.
- **After E.4**: a fix E.4 makes necessary is a new commit, reviewed by `review-hard`
  again, and E.3 and E.4 run again. An MSRV-only lint E.4 item 1 finds is a real defect
  (CI's `msrv` job fails on the same tree); where its fix would change a file of the GPUI
  specification's E.2 scope, GS stops and the orchestrator reports to the maintainer,
  since GS must change none of those files.
- **The batch's documents** cite `scripts/check_release.sh` by line at `0fa28027`,
  which their headers name; GS moves those lines, and the citations stay true as
  `0fa28027`'s, so none is re-pointed. A later unit finds a cited command by its text.
- **The cadence** after GS is S1's, on the ported script: the first `--quick` on a change (the GPUI
  plan's E1; E.4 item 4's runs on an unchanged tree) is timed into the record.

## Under another outcome

**Not taken (2026-11-15).** The maintainer chose the outcome the body is written for, so
none of the lists below applies; the section stays as the record of the options weighed.

The body is written for O3's keyed option, O2 (e), O1a and O1b no, and decision 4 (y)
(*The outcome this document is written for*, at the top). For each other answer, the list
below names what is known to change; where two answers combine, both lists apply, and a
combination that needs its own items is named under one of them. The counts were
recomputed from A.3's reaches, as the body's were. Every list also includes one item not
repeated in each: *The outcome this document is written for* (the paragraph after *The
full run*, at the top) restated for the chosen outcome.

The lists are not claimed complete: proving a list complete for each of eleven outcomes
costs a reviewer more than it gives, and a check after the edit finds what a list
missed at less cost. So, had the maintainer chosen another outcome, the gate-speed
plan's first task would have (the procedure as written, kept with the lists):
1. applied the chosen outcome's lists (and a named combination's items);
2. grepped the body, case-insensitively (`grep -in`), for these outcome-dependent terms:
   `+1.88.0`, `+1.95.0`, `31`, `29`, `35`, `33`, `MSRV`, `rust-version`, `two
   toolchains`, `seven`, `six`, `eight`, `four commits`, `O1`, `O2`, `O3` (and, under decision 4
   (x), `reviewed`, `registry`, `full metadata` and `(y)`; under an O2 other than (e),
   `trap` and `msrv-1.88`), and fixed every hit that is false under the chosen outcome;
3. had that edit checked, since its fixes of grep hits need judgement, as every
   specification edit is: `doc-review` passes until one finds nothing serious, then one
   confirming `design` check, all before GS starts.

**O3, the plain form** (the six checks run in every `--commit` and `--quick`, none keyed):
- decision 1: title "a test or rustdoc step"; (c) says all six MSRV checks keep running,
  the two `+1.88.0` ones building a fresh folder every run under (e) (19 s, measured);
- M.1: the comment reads "`--commit` and `--quick` run all of them, exactly as the job
  does; the full run leaves them to `--commit` and CI";
- M.2: bullet 1, "in `--quick` the six run whatever changed", M.1's check always; bullet
  2's sentence on the `+1.88.0` keys dropped; bullet 3 "Not keyed by either key", its O3
  sentences dropped; the header sentence "CI's MSRV job (every `cargo check` it runs,
  each on its toolchain, -D warnings) runs in --commit and --quick; the full run leaves
  it to --commit and CI.";
- A.2: the `+1.88.0` bullet dropped (29 quick-keyed); C.1 case 11 and
  `step_cache_command_crates` dropped; A.4: all six MSRV checks in its first bullet; its
  second bullet and the "leaves to `--commit`" sentence name the test and docs steps
  only;
- D.2: header and usage `--quick` line "a test or docs step"; D.3 and D.4 follow A.4;
  D.5's clause "and run the MSRV checks of CI's `msrv` job.";
- E.3: (a) 29 record files, all six MSRV lines, 24 listed; (a2) 17 listed; (a3) 29; (a4)
  2; (a5) and (a6) none; the six run in each of (a2) to (a6); (a7) dropped; (e)'s
  `--quick --no-cache` runs the 39 "and all six MSRV checks, as always"; E.2: "the (a)
  to (a6) series", and the stubs `cargo` and `python3` alone (no `rustc` stub);
- decision 9: "every step but the test and docs steps";
- O2 (e): the cost sentence "every `--commit` and every `--quick` (and the full run
  under O1a (a)) build both `+1.88.0` checks in a fresh folder"; the Proposed
  paragraph's re-check: item 4 runs all six, so it shows the two `+1.88.0` checks'
  fresh-folder cost in a `--quick` beside the four's warm cost, and E1 adds the three
  gpui checks with their crate changed (its clause "item 4 alone shows only the floor,
  since O3's keyed option skips the two" dropped);
- E.4: item 4 lists 29 and runs all six, so it shows the two `+1.88.0` checks' cost in
  every `--quick` too (a fresh folder's under (e); a warm one under O2 (a), (b) or (d),
  whose lists say so); its clause "but not the two `+1.88.0` checks', which it skips"
  dropped, and the recurring cost taken from item 4's six times and E1; item 5 the 29;
- Part F: row (iv) without the keying, `step_cache_command_crates` and case 11; row
  (iii) without case 11's clause; (iv)'s gate without (a7) and the keyed MSRV counts;
- O1a (b): "every `--commit` and every `--quick` runs the six".

**O3, the six-keyed option** (all six keyed in `--quick`):
- decision 1: title "a test, rustdoc or MSRV step"; (c) names all six as keyed;
- M.1: the comment reads "`--commit` and `--quick` run them, `--quick` each only where
  anything its key follows changed (its crates' reach and included files, the files
  outside the crates but `QUICK_UNREAD`, the tools, the environment, its toolchain); the
  full run leaves them to `--commit` and CI";
- M.2: bullet 1, "the six follow A.2's key"; the keys of all six computed after the
  toolchain check; bullet 3, each check's key adds its toolchain's `rustc -vV`, and
  M.1's check alone is never keyed; the header sentence's parenthesis "(in --quick each
  check where anything its key follows changed: its crates' reach and included files,
  the files outside the crates but QUICK_UNREAD, the tools, the environment, its
  toolchain)";
- A.2: the bullet covers all six, 35 quick-keyed; the four `+1.95.0` reaches: egui's
  check, egui, egui-widgets, `native-theme`, derive, showcase-support, test-fixtures; the
  three gpui checks, gpui, `native-theme`, derive, showcase-support, test-fixtures;
- A.4: the four `+1.95.0` checks out of its first bullet (M.1's check stays); all six in
  its second bullet and in what it leaves to `--commit`; D.3 and D.4 follow A.4;
- D.2: "a test, docs or MSRV step"; D.5's clause "and run the MSRV checks of CI's `msrv`
  job, which `--quick` skips where everything their key follows, their toolchain among
  it, is unchanged since they last passed";
- E.3: (a) 35 record files, three MSRV lines (the gpui checks), three skipped, 27
  listed; (a2) to (a6)'s sentence "The four `+1.95.0` checks run in each" dropped, each
  case's MSRV lines as follows; (a2) four run (gpui's three, the `+1.88.0` workspace
  check), two listed (the `+1.88.0` `native-theme` check, the `+1.95.0` egui check), 19;
  (a3) none, 35; (a4) all six, 2; (a5) and (a6) all six, none listed; (a7) in two
  halves, each with the stub `rustc` changing one version line and followed, with the
  stub removed, by the two runs the body names: the `rustc +1.88.0 -vV` change, both
  `+1.88.0` checks run and the 29 and the four `+1.95.0` checks listed (33); the `rustc
  +1.95.0 -vV` change, the four `+1.95.0` checks run and the 29 and the two `+1.88.0`
  checks listed (31); (e)'s `--quick --no-cache` runs the 39 "and all six MSRV checks";
- C.1 case 11: "used by the six MSRV keys"; decision 9: "every step but the ones it
  keys (the test and docs steps and the six MSRV checks)";
- O2 (e): the Proposed paragraph's re-check as E.4 item 4's under this option (item 4
  runs no MSRV check; the cost from item 1 and E1, the egui check's in a `--quick` not
  measured);
- E.4: item 4 lists 35 and runs no MSRV check (it shows the floor only); the re-check
  takes the MSRV cost from item 1 (all six) and the GPUI plan's E1 (the three gpui
  checks, their crate changed), and the egui check's cost in a `--quick` is then not
  measured; item 5 the 35;
- Part F: row (iv)'s keying clause covers all six; its gate's (a7) in the two halves
  above;
- O1a (b): "every `--commit` runs the six, and every `--quick` each of them wherever
  anything its key follows changed".

**O3, the middle option** (the six in `--commit` only):
- mechanism: in `--quick` the whole MSRV section, M.1's check included, is left out with
  one `print_skipped "MSRV (CI's msrv job)"` line, printed `skipped (--quick mode)` and
  listed under `Skipped (--quick mode):` (A.1's texts); under O2 (e) no 1.88.0 folder and
  no trap in `--quick`;
- decision 1: title "What `--quick` runs: `--commit`'s steps but the MSRV section, and
  a test or rustdoc step is left out where its key equals the key of its last pass";
  (c): the MSRV checks are not in `--quick`; decision 8's title and *The full run*
  paragraph: the MSRV steps "in `--commit`" (the self-test's "`--commit` and `--quick`"
  unchanged); decision 9: "every step but the test and docs steps, which it keys, and
  the MSRV section, which `--quick` leaves to `--commit`";
- A.4: its lead "Every step of `--commit` but the MSRV section:";
- E.2: "the (a) to (a6) series", the stubs `cargo` and `python3` alone;
- O2 (e): the cost sentence "every `--commit` (and the full run under O1a (a)) builds
  both `+1.88.0` checks in a fresh folder"; the Proposed paragraph's re-check: item 4
  runs no MSRV check, the cost is item 1's alone;
- M.1: the comment reads "`--commit` runs them; `--quick` and the full run leave them to
  `--commit` and CI";
- M.2: bullet 1, "in `--commit` (and the full run where O1a (a)); left out of `--quick`";
  bullet 2's sentence on the `+1.88.0` keys and bullet 3's O3 sentences dropped; the
  header sentence "CI's MSRV job (every `cargo check` it runs, each on its toolchain, -D
  warnings) runs in --commit; --quick and the full run leave it to --commit and CI.";
  A.2: the `+1.88.0` bullet, C.1 case 11 and
  `step_cache_command_crates` dropped (29 quick-keyed); A.4: the MSRV checks out of its
  first bullet, and all six in what it leaves to `--commit`;
- D.2: the usage `--quick` line "--commit's steps but CI's MSRV checks, and a test or
  docs step is skipped where …", the header's `--quick` entry the same; D.3 and D.4 follow;
  D.5's clause "and `--commit` runs the MSRV checks of CI's `msrv` job";
- E.3: (a) 29 record files, no MSRV line in the `--quick` log, `MSRV (CI's msrv job)`
  under `Skipped (--quick mode):`, 24 listed; (a2) to (a6) no MSRV line, and (a2) 17,
  (a3) 29, (a4) 2, (a5) and (a6) none listed; (a7) dropped; (e)'s `--quick --no-cache`
  runs the 39 and no MSRV check;
- E.4: item 4 lists 29, no MSRV check (the re-check takes the MSRV cost from item 1
  alone); item 5 the 29;
- Part F: row (iv) without the keying, `step_cache_command_crates` and case 11; row
  (iii) without case 11's clause; (iv)'s gate without (a7) and the keyed MSRV counts;
  row (iv)'s `scripts/msrv_checks.sh` clause: M.1's comment gains "`--quick`" in the
  middle option's form above, with no keying; row (iv) also brings the mechanism above,
  the one `print_skipped "MSRV (CI's msrv job)"` in `--quick`;
- O1a (b): "every `--commit` runs the six; `--quick` none, so a change made after the
  release's last `--commit` goes without them locally, CI's `msrv` job finding it on the
  push".

**O3 declined** (no MSRV check in the gate): everything of the MSRV checks falls away:
- the title's "and CI's MSRV checks"; How to read's row M; *The full run* paragraph (one
  step, the self-test, under O1); decision 1 (c)'s MSRV sentences, the title's
  `+1.88.0` words, and (b)'s "the MSRV checks" and "`msrv`" (they are then left out
  under (c) too); decision 8 (it becomes "not added"); decision 9: "every step but the
  test and docs steps"; Part M (M.1 to M.3) and `scripts/msrv_checks.sh`; D.1's first
  row;
- O1a (O1 is then O1b alone), O2 whole, O3's options;
- A.2's `+1.88.0` bullet (29 quick-keyed), C.1 case 11 and `step_cache_command_crates`,
  A.4's MSRV words (its first bullet's, second bullet's and the "leaves to `--commit`"
  sentence's);
- C.2's strict base list without `scripts/msrv_checks.sh` (four scripts), and D.3's list
  the same; C.3's "the MSRV checks (M.2)" among the steps the strict cache does not key;
- D.2's MSRV texts (the header's `--quick` entry, "a test or docs step"; the usage's
  `--commit` line; M.2's header sentence; the usage's `--quick` line's MSRV words; and
  the paragraph after the usage: its sentences on the MSRV checks), D.3's MSRV items and
  its `## msrv_checks.sh` section, D.4's MSRV items, D.5's last clause;
- E.2: "the (a) to (a6) series", the stubs `cargo` and `python3` alone; E.3 (t);
- E.3 (m), (r), (c)'s O1a form, (d)'s (i) difference, (a) to (a6)'s MSRV lines and counts
  ((a) 29 record files, 24 listed; (a2) 17; (a3) 29; (a4) 2; (a5) and (a6) none), (a7),
  (e)'s MSRV clause (`--quick --no-cache` runs the 39); E.4 item 1's MSRV clauses, item
  4's 31 (29) and its MSRV sentences and report, item 5's 31 (29), and decision 8's
  report of their time;
- Part F: commit (i) (GS is commits (ii) to (iv), its last commit (iv) or the review's
  last fix; the intro's "four commits" becomes three, its sentence on the MSRV port
  coming first goes, and "the review over the four commits" names three), its row and
  gate, the sentence that there is no commit (i) where O3 is declined (it then holds),
  rows (iii) and (iv)'s clauses on `scripts/msrv_checks.sh`, case 11 and the `+1.88.0`
  keying, the O1a sentence, the review brief's "Part M's commands and
  check against `ci.yml:266-283`" and its trap clause, and the sentence on an MSRV-only
  lint after E.4;
- *What could not be verified*: the bullets on the MSRV checks' pass, the 1.88.0 folder's
  size, the sccache wrapper's cost for 1.88.0 and the incident behind decision 8.

**O2 (a)** (a kept `<build_directory>/msrv-1.88`):
- M.3 and O2: no trap and no leftover removal; the folder kept between runs and swept by
  hand at each close-out, the maintainer's exception to the build-folder rule; its size
  from `du -sb` after the run (E.4 item 1);
- the two `+1.88.0` checks are then warm after the first run, so decision 1 (c)'s reason
  for keying them falls away, and O3's best becomes the plain form: apply its list above
  (unless the maintainer keeps the keyed option all the same);
- decision 1 (c)'s sentence on the two `+1.88.0` checks (a fresh folder every run, 14.0
  s + 5.2 s), and decision 8's, O1a (a)'s and O3's cost sentences: the 19 s measured
  under (e) is a first run's, later runs warm (not measured);
- E.4: item 1 is the folder's first, fresh build; under the plain form item 4 runs the
  two `+1.88.0` checks warm and so shows their warm cost (the plain form's "a fresh
  folder's" does not apply); where the keyed option is kept, item 4's parenthesis "(under
  O2 (e) each run of them builds a fresh folder …)" reads "warm after the folder's first
  build", and their warm cost in a `--quick` is then not measured (item 4 skips them, and
  E1, a gpui unit, does not reach them);
- E.3 (t), E.2's variant for it, row (i)'s "E.3 (t)" and the review brief's trap clause
  dropped; O2 (e)'s cost sentence and Proposed paragraph stay as the record of the
  proposal.

**O2 (b)** (`target/`): as (a), with no `--target-dir` (the commands `ci.yml`'s as
written, `msrv_ci_check` comparing them whole, E.3 (c) and (d)'s lines without it), the
build in `target/` on `/home`, swept by `sweep-target.sh`.

**O2 (c)** (the two `+1.88.0` checks left to CI):
- How to read's row M without "where 1.88.0 builds (O2)"; D.1's first row without "the
  1.88.0 build folder"; D.3's `## msrv_checks.sh` section "(the toolchains read, the four
  checks it runs and the six it compares, `msrv_ci_check`, the toolchain check)";
- Part M: five steps (M.1's check and the four `+1.95.0` checks; the table's two
  `+1.88.0` rows stay as `msrv_ci_check`'s pairs, not as steps), M.1's "A seventh step"
  reading "A fifth step", E.3 (r) removing the `1.95.0-…` line instead (the one
  toolchain the gate then checks), M.3 and the trap gone (E.3 (t), E.2's variant for it, row
  (i)'s "E.3 (t)" and the review brief's trap clause with them);
  `msrv_ci_check` still checks all six lines of `ci.yml`, so `MSRV_TOOLCHAIN` is still
  read; M.1's comment reads "`--commit` and `--quick` run the four on the connector
  toolchain; the two at the workspace's `rust-version` are CI's alone; the full run
  leaves them to `--commit` and CI";
- decision 1: title "a test or rustdoc step"; (c): the four `+1.95.0` checks are
  compile-only steps that keep running, the two `+1.88.0` ones CI's alone;
- M.2: bullet 1, "in `--quick` the four run whatever changed", M.1's check always;
  bullet 2's toolchain check for 1.95.0 alone, its sentence on the `+1.88.0` keys
  dropped; bullet 3 "Not keyed by either key", its O3 sentences dropped; the header
  sentence "CI's MSRV job on its connector toolchain (every `cargo check` it runs there,
  -D warnings) runs in --commit and --quick, its rust-version checks being CI's alone;
  the full run leaves it to --commit and CI.";
- A.4: its second bullet and the "leaves to `--commit`" sentence name the test and docs
  steps only (its first bullet already names the four);
- O3's keyed option has nothing to key: 29 quick-keyed, A.2's `+1.88.0` bullet, C.1 case
  11, `step_cache_command_crates` and E.3 (a7) dropped; E.3 (a) 29 record files and four
  MSRV lines, 24 listed, (a2) 17, (a3) 29, (a4) 2, (a5) and (a6) none, the four run in
  each; (e)'s `--quick --no-cache` runs the 39 "and the four `+1.95.0` checks"; E.4 item
  4 lists 29 and runs the four, its clause on the two `+1.88.0` checks dropped, the
  recurring cost from item 4's four times and E1 alone; item 5 the 29;
- Part F: row (i)'s "Part M (seven steps, …" reading "five steps", without M.3, and its
  gate without (t); row (iv) without the keying, `step_cache_command_crates`, case 11
  and (a7), its gate's MSRV counts as above; row (iii) without case 11's clause; E.2: "the (a) to (a6) series", the stubs `cargo` and
  `python3` alone; decision 9: "every step but the test and docs steps";
- E.4 item 1: "M.1's check and the four MSRV steps passed"; E.3 (c)'s sum at GS's end
  "+0, +5, +1 or +6 as O1 decides";
- O1a (b): "every `--commit` and every `--quick` runs the four `+1.95.0` ones; the two
  at the workspace's `rust-version` are CI's alone";
- combined with the plain form, the counts above hold as they stand (with the two
  `+1.88.0` checks gone, the keyed option keys nothing and equals the plain form);
  combined with the middle option, the same record and list counts ((a) 29 record files,
  24 listed, (a2) 17, (a3) 29, (a4) 2, (a5) and (a6) none), but no MSRV line in any
  `--quick` log, (e)'s `--quick --no-cache` running the 39 and no MSRV check, and E.4 item
  4 running none;
- **combined with the six-keyed option**, the four `+1.95.0` checks are keyed, and these
  items replace every item of this list and of the six-keyed option's list that speaks
  of keying (where the two lists conflict, these hold):
  - decision 1: title "a test, rustdoc or MSRV step"; (c): the four keyed, the two
    `+1.88.0` checks CI's alone; decision 9: "every step but the ones it keys (the test
    and docs steps and the four MSRV checks)";
  - M.1's comment "`--commit` and `--quick` run the four on the connector toolchain,
    `--quick` each only where anything its key follows changed (its crates' reach and
    included files, the files outside the crates but `QUICK_UNREAD`, the tools, the
    environment, its toolchain); the two at the workspace's `rust-version` are CI's
    alone; the full run leaves them to `--commit` and CI"; M.2: bullet 1 "the four follow A.2's key", bullet 2's toolchain check for
    1.95.0 alone with the four's keys computed after it, bullet 3 each key adding
    `rustc +1.95.0 -vV`, M.1's check never keyed; the header sentence's parenthesis "(in
    --quick each check where anything its key follows changed: its crates' reach and
    included files, the files outside the crates but QUICK_UNREAD, the tools, the
    environment, its toolchain)";
  - A.2: the bullet covers the four (the six-keyed option's reaches), 33 quick-keyed;
    `step_cache_command_crates` and C.1 case 11 stay ("used by the four MSRV keys", whose
    commands are `-p` forms); A.4: the four out of its first bullet, in its second and in
    what it leaves to `--commit`; D.3 and D.4 follow A.4;
  - D.2: "a test, docs or MSRV step"; D.5's clause "and run the MSRV checks of CI's
    `msrv` job on its connector toolchain, which `--quick` skips where everything their
    key follows, their toolchain among it, is unchanged since they last passed";
  - E.3: (a) 33 record files, three run (gpui's), one listed (egui's), 25; (a2) three
    run, one listed, 18; (a3) 33; (a4), (a5) and (a6) all four run; (a7) kept in its
    `rustc +1.95.0 -vV` form: the four run, the 29 listed; E.2 keeps the `rustc` stub;
    (e)'s `--quick --no-cache` runs the 39 "and the four `+1.95.0` checks";
  - E.4: items 4 and 5 the 33, item 4 running no MSRV check; the re-check from item 1
    and E1 (the egui check's cost in a `--quick` not measured);
  - Part F: row (iv) with the four's keying, `step_cache_command_crates` and case 11
    (row (iii) as in the body), (iv)'s gate with (a7)'s `+1.95.0` form;
  - O1a (b): "every `--commit` runs the four `+1.95.0` ones and every `--quick` each of
    them where anything its key follows changed; the two at the workspace's
    `rust-version` are CI's alone";
- E.3 (c)'s O1a form +5 (the heading and five status lines), (d) four inserted commands;
  O1a and the full-run paragraph: five MSRV steps, six in all with the self-test;
- D.2's usage and header "CI's MSRV checks on its connector toolchain (the `rust-version`
  ones are CI's alone)", and their `--quick` texts "a test or docs step"; D.3's "(`ci.yml`'s
  `msrv` job on its connector toolchain; the `rust-version` ones are CI's alone)", D.4's
  list the same; D.5's clause "and run the MSRV checks of CI's `msrv` job on its
  connector toolchain"; the strict base list keeps `msrv_checks.sh`;
- E.4 item 1 without the `+1.88.0` times and the folder's size; *What could not be
  verified* without the 1.88.0 folder's and wrapper's bullets; decision 8's cost without
  1.88.0; O3's For without the two 1.88.0 cases, which O2 (c) gives up.

**O2 (d)** (`--target-dir <build_directory>` itself): first the trial O2 names (a
`--commit`, then the stable steps' freshness and the folder's size checked); then as (a),
with the shared folder in place of `msrv-1.88` (no exception needed, swept with the build
folder), and O3's best the plain form, as under (a).

O2 and O3 depend on each other: O3's keyed option is proposed because O2 (e) makes the
two `+1.88.0` checks build a fresh folder every run, so under (a), (b) or (d) its reason
falls away and the plain form becomes the better O3 (each list above says so); with
O2 (e) and the keyed option chosen on 2026-11-15, this is moot.

**O1a (a)** (the seven MSRV steps in the full run):
- Part F: in commit (i), gated by E.3 (c)'s O1a form (+7; +5 under O2 (c));
- *The full run* paragraph, decision 8's title ("… and in the full run", without "on
  approval (O1)") and M.2 bullet 1: the full run runs them, building a fresh 1.88.0
  folder too under (e);
- M.2's header sentence, "CI's MSRV job (every `cargo check` it runs, each on its
  toolchain, -D warnings) runs" followed by the modes O3 gives, and nothing on the full
  run leaving it: the keyed option "in every mode (in --quick the rust-version checks
  where anything their key follows changed: their crates' reach and included files, the
  files outside the crates but QUICK_UNREAD, the tools, the environment, their
  toolchain)."; the plain form "in every mode, so no local gate misses what it checks."
  (0.8.0's); the six-keyed option "in every mode (in --quick each check where anything
  its key follows changed: its crates' reach and included files, the files outside the
  crates but QUICK_UNREAD, the tools, the environment, its toolchain)."; the middle
  option "in --commit and the full run; --quick leaves it to --commit and CI.";
- M.1's comment: the keyed option "every mode runs them, `--quick` the two at the
  workspace's `rust-version` only where anything their key follows changed (their
  crates' reach and included files, the files outside the crates but `QUICK_UNREAD`, the
  tools, the environment, their toolchain)"; the plain form 0.8.0's "Every mode runs all
  of them, exactly as the job does"; the six-keyed option "every mode runs them,
  `--quick` each only where anything its key follows changed (its crates' reach and
  included files, the files outside the crates but `QUICK_UNREAD`, the tools, the
  environment, its toolchain)"; the middle option "`--commit` and the full run run them; `--quick` leaves
  them to `--commit` and CI";
- D.2: the usage's `--commit` line's "CI's MSRV checks" moves to a sentence of its own
  with the same modes; D.3 and D.4 list them in those modes, and D.4's parenthesis
  "(while the full run lacks them, …)" goes;
- D.5's clause: the keyed option, "and `--commit`, the full run and `--quick` run the MSRV
  checks of CI's `msrv` job, `--quick` skipping the two at the workspace's `rust-version`
  where everything their key follows, their toolchain among it, is unchanged since they
  last passed"; the plain form, "and every mode runs the MSRV checks of CI's `msrv` job";
  the six-keyed option, "and every mode runs the MSRV checks of CI's `msrv` job,
  `--quick` skipping each where everything its key follows, its toolchain among it, is
  unchanged since it last passed"; the middle option, "and `--commit` and the full run run the MSRV checks
  of CI's `msrv` job";
- E.4 item 1 unchanged (a `--commit`); the close-out's full run then times them too.

**O1b (a)** (the step cache's self-test in the full run): commit (iii), gated by E.3
(c)'s O1b form (+1); C.6 and
*The full run* paragraph say every mode; decision 16's title "The step cache's self-test
in every mode" (without "the full run's part pending O1"); D.3 and D.4 list the
self-test in every mode; D.5 unchanged.

**Decision 4 (x)** (registry build scripts and proc macros trusted as pinned, 0.8.0's
choice; nothing below changes the `--quick` key, which never used the list, C.4): what
exists only for (y) goes:
- `CACHE_REVIEWED_REGISTRY_CODE`, its refusal and C.2's review of the 23 build scripts
  and 15 proc macros (kept as findings in GS's record, not as a list the gate reads),
  with the **Upkeep** paragraph;
- the full metadata read (C.2's bullet; C.1's "the reach", "and the full metadata through
  two variables"; the self-test's synthetic full metadata), `step_cache_full_metadata`,
  C.1 case 10, and C.4's sentence that the `--quick` key stays independent of the
  full-metadata read (`CR8:779-804`'s `rust_reach_resolve` reads only the workspace
  metadata, so (x) needs no full metadata);
- case 7's registry half (an unlisted registry build script refused in the strict form,
  accepted in the `--quick` form; the probe half stays), and row (iii)'s "an unlisted
  registry build script accepted";
- decision 4's *The `--quick` reach*: "skips the host-probe refusal" (without "and
  (y)'s list"); decision 5's "the full metadata unread" among the reasons a strict key
  is off;
- D.1's reach row, D.3's "the reviewed registry list", the review brief's "the reviewed
  registry list", row (iii)'s "the reviewed list"; the header's and the closing
  section's mention of decision 4 (y) as awaiting approval.

## What could not be verified

- **nextest's speed on 0.7.x**, against its per-process start-up (B.4): not run here
  (no cargo builds were made for this document); E.4 item 3 measures it on three steps.
  0.8.0's commits give no timing.
- **nextest's output on these crates**: that 0.9.138's summary line matches the
  counting (`[0-9]+ tests? run: [0-9]+ passed`) was shown on 0.8.0 by `24711322`, the
  same version; not re-run here. E.4 item 3 checks the counts.
- **Order-dependent tests**: B.4's reading found no test relying on another's state;
  only a run under nextest shows it (E.4 item 1 runs every test step so).
- **Whether nextest reads `RUST_TEST_THREADS`**: its help names only
  `NEXTEST_TEST_THREADS`; B.2 passes the cap explicitly, so nothing depends on it.
- **The cache's gain**: no measurement on 0.8.0 or here. A compile-only step on an
  untouched crate is already fast through Cargo's fingerprints, so the strict cache
  saves most where a step re-documents (`native-theme`'s two docs steps) and on
  `native-theme-derive`'s tests; E.4 items 1 and 2 measure it. Whether an environment
  variable varies between the orchestrator's calls (every strict key would miss, and
  every `--quick` key where it is outside the deny-list) is not known; E.4 items 2 and 4
  show it.
- **`--quick`'s time**, its keys' computation among it, on a clean tree and on a
  connector unit, and the cost of the compile-only steps it keeps (decision 1): not
  measured; E.4 item 4 and the first timed `--quick` on a change (E1) give them.
- **That the six MSRV checks pass locally at `0fa28027`**, and their time: the two
  `+1.88.0` ones passed locally on 2026-11-15 (O2's measurement); the four `+1.95.0`
  ones were not run here, and whether CI's `msrv` job was green on `0fa28027` was not
  looked up. E.4 item 1 runs all six.
- **The incident behind decision 8** (CI's MSRV job red for a day unnoticed): as
  reported by the orchestrating session; not traced here to a CI run.
- **The registry review of C.2** is a `grep` of each build script and proc macro for
  program runs, file reads and variables, not a reading of every line; a read hidden
  behind a helper crate the grep does not name would be missed.
- **The disk cost of the 1.88.0 checks' own build folder** (M.3): 628,345,955 bytes
  (`du -sb`, decimal) from a fresh build, measured on 2026-11-15 (O2); a kept (a)
  folder's growth over many runs not measured. **Whether rustup installs a missing
  toolchain on its own here** (`RUSTUP_AUTO_INSTALL`): not read; M.2's check makes it
  not matter.
- **What a fresh 1.88.0 build costs through the global sccache wrapper** (M.3, O2):
  measured on 2026-11-15 (O2): Cargo 1.88.0 does use the wrapper (sccache counted 286
  compile requests a run, 64 not cacheable, 214 hits and no miss in the warm run), and a
  warm (e) run of the two checks took about 19 s. Not measured: a run after an sccache
  eviction.

## Open points

- **O1 — the full run.** S1 says it is not changed. GS adds to `--commit` and `--quick`
  seven MSRV steps (CI's six checks, `ci.yml:262-283`, and the check that they are the
  job's, M.1) and the step cache's self-test (C.6). Two questions:
  - **O1a, the seven MSRV steps in the full run.** (a) Yes: the release gate stays a
    superset of the per-commit subset; the full run takes the two toolchains' checks
    (1.88.0's under O2 (e): about 19 s a run, measured; 1.95.0's not measured, warm from
    `--commit` by then; under (e) the 1.88.0 folder is deleted after every run, so the
    full run's two `+1.88.0` checks start from a fresh folder too). (b) No, as S1 says: then the full
    run checks less than `--commit`. Its cost is small: every `--commit` runs the six
    MSRV checks, and every `--quick` the four `+1.95.0` ones and the two `+1.88.0` ones
    wherever anything their key follows changed (O3's keyed option), the release's
    documents, sweep and version units included (each ends with `--quick`, S1's cadence),
    so only a change made after the last of those and before the full run goes without
    them locally; CI's `msrv` job
    would find it on the push, which S4 makes before the asset run, as a red run and an
    e-mail, and it would not ship.
  - **O1b, the step cache's self-test in the full run.** It tests the tools of
    `--commit` and `--quick`, which the full run does not use. (a) Yes: the release
    gate proves the scripts it releases work (0.8.0's choice, `CR8:1322`). (b) No: the
    full run stays exactly as S1 says; the self-test runs in every `--commit` and
    `--quick`, so a broken tool is seen there.
  - Proposed: (a) for both. GS proceeds with the eight steps in `--commit` and `--quick`
    either way and adds to the full run only what the maintainer approves (Part F).
  - **Answered 2026-11-15: O1a no, O1b no.** The full run stays unchanged, as this
    document is written for.
- **O2 — where the two `+1.88.0` MSRV checks build** (M.3). Cargo 1.88.0 ignores the
  global `build.build-dir` (`~/.cargo/config.toml:8`; its `cargo metadata` has no
  `build_directory` field and reports `target_directory` the worktree's `target/`), so as
  `ci.yml` writes them the two checks would build into `target/` on `/home`, against the
  user's rule that builds stay in the build folder on the work SSD. 1.95.0 and the
  stable toolchain read the setting (their `build_directory` is
  `/run/media/tibi/work/cargo-build/fe/88b35d036b2878`). The user's rule also says: "A
  separate build folder is a whole extra build of tens of GB: only when truly needed,
  under `/run/media/tibi/work/cargo-build/`, and deleted by the agent that made it before
  its task ends." What the sweep reaches (`~/.claude/scripts/sweep-target.sh:43-66`): its
  lock check and its cleanup of `incremental` and `examples` folders look up to three
  levels below the build folder (`find "$dir" -maxdepth 3`), so they reach
  `msrv-1.88/debug/`; its `cargo sweep` runs with `CARGO_TARGET_DIR` set to the build
  folder itself, so it likely does not sweep `msrv-1.88/debug/deps` (not verified). And
  every compile goes through the global wrapper `rustc-low-priority.sh`, which runs
  sccache (`~/.cargo/config.toml:9`; M.3): sccache serves dependencies' compiles it has
  cached, not the workspace's incremental crates (`config.toml:11-14`), so a fresh
  1.88.0 folder costs what sccache does not serve; only (e)'s cost is measured (below).
  - (a) Both `+1.88.0` steps get `--target-dir <build_directory>/msrv-1.88`,
    `<build_directory>` from the stable toolchain's `cargo metadata`, `target/msrv-1.88`
    where it reports none. The builds stay on the work SSD, inside the worktree's own
    build folder. But it is a separate build folder in the rule's sense, kept between
    runs (a 1.88.0-only build, its size measured at E.4 item 1), which the rule allows
    only when truly needed and wants deleted by the agent that made it at its task's
    end; and the sweep likely leaves its `deps` alone. Choosing (a) is the maintainer's
    exception to that clause, with the folder kept and swept by hand (or removed) at
    each close-out.
  - (b) Accept `target/` for them, kept small with `sweep-target.sh`: no new path, but
    1.88.0's build on `/home`, which the rule exists to prevent (a `target/` once filled
    2.9 TB, the rule's own history).
  - (c) Run only the four `+1.95.0` checks locally and leave the two `+1.88.0` ones to
    CI: no build on `/home` and no extra folder, but the local gate then misses what CI
    checks at the workspace's own MSRV, the case decision 8 was made for. It gives up
    exactly the checks the history's 1.88.0 cases needed: `4470c0da` (CI's `msrv` job red
    over nine dead-code markers Rust 1.88 counts as used) and `62498b45` (a dead-code
    allow kept for Rust 1.88), decision 8.
  - (d) `--target-dir <build_directory>` itself: 1.88.0's artifacts in the same folder as
    the stable toolchain's, swept with it, no extra folder. Whether Cargo 1.88.0's
    target-dir layout coexists there with the newer toolchains' build-dir layout
    (`debug/` under both, written by Cargo versions of different formats) is not known.
    Partial evidence: the stable toolchain and 1.95.0 already share that folder, and it
    has the classic layout (`debug/` with `build/`, `deps/`, `incremental/` and the
    libraries, beside `release/`, `package/` and the cross targets' folders, listed for
    this document); 1.88.0 is the one Cargo there that reads no `build-dir`. So it still
    needs a measured trial (a `--commit`, then the stable steps' freshness and the
    folder's size checked) before it is chosen. It shares the workspace's build folder
    rather than making a separate one, so it is inside the rule too if the trial passes.
  - (e) A separate 1.88.0 folder, `<build_directory>/msrv-1.88` (`target/msrv-1.88` where
    none is reported), that the gate deletes at the end of every run, whatever its
    outcome, and removes a leftover folder before the first MSRV step (a run killed by
    SIGKILL never runs its trap). The deletion is an EXIT trap set at the start of the
    MSRV section, not before (so a run that never reaches Part M has none), rather than a
    removal after the second `+1.88.0` step, which a failing later step, an interrupt
    or a `SIGTERM` would skip: the trap also covers those exits; it prints
    the folder's `du -sb` for the record and removes the folder only where the folder
    exists, and it never changes the run's exit status: it calls no `exit` and no `return
    <status>`, and every command in it is guarded (`[ -d <folder> ] && du -sb <folder> ||
    true`, `rm -rf <folder> || true`), so bash keeps the status the run exits with. Both
    halves matter. Under `CR`'s `set -e` (`CR:43`), an unguarded failing command inside an
    EXIT trap makes a passing run exit 1 (`bash -c 'set -e; trap "du -sb /nonexistent
    >/dev/null 2>&1; echo after" EXIT; exit 0'` exits 1 without printing `after`); and a
    trap that saves the status and returns it with `exit` is easily written wrong:
    `bash -c 'set -e; t(){ local s; s=$?; true; exit $s; }; trap t EXIT; exit 1'` exits 0
    (`$?` is read after `local s`, which succeeded), so a failing gate would pass. With
    the guarded form and no `exit`, the same run exits 0, 1 and 3 for `exit 0`, `exit 1`
    and `exit 3`, 1 where `set -e` ends it at a failing `false`, and 143 on `SIGTERM`, each
    with the folder printed and removed; a command substitution's or a `( … )` subshell's
    exit does not run the trap (all run for this document on bash 5.3.20). `CR` at
    `0fa28027` sets no trap (`grep -n trap scripts/check_release.sh` finds none), and a
    second `trap … EXIT` would replace this one, so a later EXIT trap in `CR` must be
    merged into it. The folder's path is written into the trap's text when the trap is
    set (`printf %q`), or held in a global variable set before it and never unset: a
    `local` of `msrv_checks` is out of scope when the trap runs, and under `set -u`
    (`CR:44`) the trap would fail (a trap naming a function's `local` exits 1, "unbound
    variable", and leaves the folder behind; both safe forms exit with the run's status
    and remove it; run for this document on bash 5.3.20). E.3 (t) tests it.
    Within the rule as written (a separate folder under
    `cargo-build/`, deleted by what made it at its task's end), no exception needed, no
    sweep needed. The cost: every `--commit`, every `--quick` where their key differs
    from their last pass's (O3's keyed option, A.2), and the full run under O1a (a)
    build both `+1.88.0` checks in a fresh folder; sccache serves the dependencies'
    compiles it holds for 1.88.0 (all of them from the second run on, if its cache keeps
    them), and the rest (the workspace crates, and the build scripts and proc macros
    where sccache does not serve them) is compiled each run; about 19 s a run, measured
    below.
  - **Proposed: (e).** It is the one option that is inside the user's rule as written
    without a trial ((d) needs one) or an exception ((a)), and that keeps the checks
    local ((c)) and off `/home` ((b)). Its cost was the unknown, so before O2 went to the
    maintainer the orchestrator ran the two `+1.88.0` checks twice in a fresh
    `<build_directory>/msrv-1.88`, deleting it after each run as (e) does, and timed the
    second run (sccache warm for 1.88.0): about 19 s a run (below). A comparison with a
    `--quick` of the same tree is not possible until the ported script exists; the
    proposal is (e) on that known number, and it is the maintainer's to decide. The one
    re-check is E.4 item 4's report: the two `+1.88.0` checks' recurring cost from E.4
    item 1 (a fresh folder each run under (e)), the four `+1.95.0` checks' warm cost from
    item 4 (the floor) and from the GPUI plan's E1 (a gpui unit, run after GS ends, so
    the report is completed then), each beside its run's time; item 4 alone shows only
    the floor, since O3's keyed option skips the two
    `+1.88.0` checks there. With those numbers the maintainer may revisit (a) or (d) here,
    and O3's options. No threshold is set in advance.
  - **Measured 2026-11-15** (the orchestrator, worktree `v071` at `0fa28027`, through
    `/home/tibi/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/slot.sh` and
    `/home/tibi/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/lowprio.sh`:
    cargo at nice 19 with `ionice -c 3` (`lowprio.sh`'s `renice -n 19` and `ionice -c 3`),
    and each rustc at nice 19 with `ionice -c 2 -n 7`, set by the compile wrapper
    `~/.cargo/rustc-low-priority.sh` before it runs sccache;
    `RUSTFLAGS=-Dwarnings`, CI's two commands with `--target-dir
    <build_directory>/msrv-1.88`, the folder deleted after each run; the run's script set
    no job count, `CARGO_BUILD_JOBS` was unset and `~/.cargo/config.toml` sets no `jobs`,
    so Cargo ran with its default job count on the machine's 32 CPUs; no other cargo
    ran, only Zed's two language servers `cargo-tom` and `cargo-appraiser`, which build
    nothing): run 1, 20.0 s + 6.0 s; run 2, 14.0 s + 5.2 s, so about 19 s per gate run
    under (e). Both runs passed. The folder held 628,345,955 bytes after run 1 and
    628,345,964 after run 2 (`du -sb`, decimal; about 600M as `du -sh` would print it),
    each measured just before its deletion. sccache, from its statistics before and
    after each run: 286 compile requests per run, 64 of them not cacheable; run 1, 181
    hits and 33 misses (1.88.0 builds of earlier sessions were in its cache already);
    run 2, 214 hits and 0 misses. What (a) would save is at most those 19 s per run, at
    the cost of a kept folder of at least 628 MB (decimal) that grows until swept, and
    the maintainer's exception, so the proposal stays (e). Not measured: a run after an
    sccache eviction; the time under the gate's own job count (the plans' wrappers export
    `CARGO_BUILD_JOBS=16`, `todo_v0.7.1_gpui-kit-0.9.1-plan.md` P1); the time beside a
    `--quick` of the same tree (the ported `--quick` does not exist yet on 0.7.x). The
    raw files are kept at
    `/home/tibi/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/batch-2026-11-15/o2-measure/`
    (`run.sh`, `result.txt`, the two runs' logs and the sccache statistics before and
    after each run).
  - What each option changes elsewhere is in *Under another outcome*; the spec is
    written for (e). Under (e) the two `+1.88.0` commands carry `--target-dir <folder>`,
    which `msrv_ci_check` leaves out of its comparison (the one argument the script adds),
    and E.3 (c) and (d)'s inserted lines carry it.
  - GS starts only once O2 is answered (Part F), and v0.7.1's E1 onward waits for GS.
  - **Answered 2026-11-15: (e).**
- **O3 — the MSRV checks in `--commit` and `--quick` at all** (decision 8). S1, as the
  maintainer approved it, ports `--quick`, nextest and the step cache; it does not name
  the MSRV checks. Decision 8 adds them on the orchestrating session's decision, which
  goes beyond the approved S1 and needs the maintainer's explicit approval.
  - For: CI's `msrv` job runs them on every push of a branch with a pull request, and
    under S4 that push comes before the full run, so an MSRV-only lint is otherwise
    first seen by CI, as a red run and an e-mail. The history names four such cases
    (decision 8): `4470c0da` and `62498b45` at 1.88.0 and `c36a874f` at 1.95.0 on 0.8.0,
    and `a9c34cdd` on 0.7.x itself, the case `ci.yml:262-263` records; the orchestrating
    session reports such a run left red for a day unnoticed.
  - Against: two more toolchains' checks in every `--commit` and `--quick`, warm through
    the build folder and sccache except where O2 (e) deletes the 1.88.0 folder each run
    (1.88.0's under (e): about 19 s a run, measured, O2; 1.95.0's not measured; E.4 item
    1 times both), O2's build-folder question, and a seventh step and a sourced
    file to keep `ci.yml` and the script in step; without them, CI alone catches such a
    lint, after the push.
  - The middle option: the MSRV checks in `--commit` only. Its cost: the units the
    cadence gates with `--quick` are pushed after `--quick` (decision 1 (b)), so an
    MSRV-only lint a unit brings reaches CI before the next `--commit`, as a red run and an
    e-mail. Its benefit: under O2 (e), the fresh 1.88.0 build is paid only at `--commit`
    (every third code unit and the others S1 names), not at every unit's end.
  - The plain form: the six checks in `--commit` and `--quick`, run every time. About 19
    s of `+1.88.0` checks in every `--quick` under O2 (e), measured; the four `+1.95.0`
    checks' warm time not measured.
  - The six-keyed option: the six checks carry the `--quick` key in `--quick`, each with
    its toolchain's `rustc -vV` added (A.2's form); in `--commit` they always run, their
    passes recorded in the `--quick` set. For: a documents-only unit, or a `--quick` on a
    tree a gate already passed, skips all six; every code change within a check's reach
    still runs it before the push. Against: six steps under a key `review-hard` must check
    (the reach of a `--workspace --exclude` command, the toolchains' versions in the key);
    the compile-only backstop of decision 1 (c), which catches what a key misses, is lost
    for all six in `--quick` (CI keeps it, `--commit` runs them always), although the
    reason for keying, a fresh folder each run that no fingerprint helps, holds for the
    two `+1.88.0` checks alone.
  - The keyed option (the two `+1.88.0` checks keyed): only those two carry the `--quick`
    key, with `rustc +1.88.0 -vV` added (A.2); the four `+1.95.0` checks run every time,
    in the shared build folder, where fingerprints make an unchanged crate cheap (their
    warm time not measured; E.4 item 4 measures it). For: the 19 s that recur under (e)
    are paid only where anything the two checks' key follows changed (a gpui-only unit
    skips both, A.3), as under the six-keyed option; every code change within their reach
    still runs them before the push, so it beats the middle option on CI e-mails; the four
    `+1.95.0` checks keep the compile-only backstop; and two keyed steps, not six, for
    `review-hard` to check. Against: the four `+1.95.0` checks' warm cost in every
    `--quick`, not measured, which the six-keyed option would save where nothing in their
    reach changed; and the backstop is lost for the two `+1.88.0` checks in `--quick` (CI
    and `--commit` keep it).
  - Proposed: the keyed option (the two `+1.88.0` checks keyed). On what is known it is
    the best: it removes the one measured recurring cost (19 s a run under (e)) wherever
    nothing it checks changed, keeps the plain form's coverage for every change its key
    follows, and keys nothing that fingerprints already make cheap; the six-keyed option
    would save the four `+1.95.0` checks' warm time, unmeasured, at the cost of their
    backstop and a wider key. What would change this: a measured warm time of the four
    `+1.95.0` checks that is large beside a `--quick` (E.4 item 4), which would favour the
    six-keyed option. The re-check is O2's (E.4 item 4's report). Every other outcome's
    changes are in *Under another outcome*. GS starts only once O3 (and O2) is answered
    (Part F).
  - **Answered 2026-11-15: the keyed option.**

Decision 4: **answered 2026-11-15, (y).** Beyond O1 to O3 and decision 4, all answered,
nothing here is a preference.
