# v0.7.1: the gate's speed (`--quick`, nextest, the step cache) and CI's MSRV checks — implementation plan

- Specification: [`todo_v0.7.1_gate-speed-spec.md`](todo_v0.7.1_gate-speed-spec.md)
  (Parts M, A to F, *Under another outcome*, *Open points*). Rationale:
  [`../todo_v0.7.1_batch-speed-rationale.md`](../todo_v0.7.1_batch-speed-rationale.md), S1
  (approved by the maintainer on 2026-11-15) and S6.
- Place: this is the plan of **GS**, the row of that name in
  [`todo_v0.7.1_gpui-kit-0.9.1-plan.md`](todo_v0.7.1_gpui-kit-0.9.1-plan.md) (below: the
  GPUI plan), v0.7.1's first unit. It runs in the core worktree
  `/home/tibi/Rust/native-theme/.claude/worktrees/v071`, on `v0.7.1-rc2`, after the GPUI
  plan's P1 (the tools, the hook trial) and P0 (commit (0)), and before its LN and E1;
  its Task 0 runs before P0 (below, *Order*). Every unit of the GPUI and the KDE step 1
  plans after it gates with the ported script.
- The batch's working rules are cited as "rule N": the batch-speed rationale's
  *Appendix — the batch's working rules*, rules 1 to 26, authoritative where this text
  differs (*Working rules*, below).
- Written: 2026-11-15, against `v0.7.1-rc2` at `0fa28027`. Status: **implemented on
  2026-11-17** (commits `e866ea6c..v0.7.1`, the close-out commit that moved this plan
  and its specification here from `docs/` among them); the running record at the end of
  this plan says how it ran. Settled 2026-11-15: the maintainer answered the
  specification's open points and decision 4
  (O1a no, O1b no, O2 (e), O3's keyed option, decision 4 (y)) and approved the 0.7.1
  plans as written: "When the plan that goes in the 0.7.1 version is ready, without
  errors, start implementing it." (*The maintainer's part*). The text below is the
  plan as it was approved: it speaks of the work as still to do, and gives the paths
  of the documents moved here as they were in `docs/`.
- Citations: every `file:line` here is `0fa28027`'s unless it names another commit or
  version; `CR:N` is `scripts/check_release.sh` at `0fa28027`, as in the specification;
  those line numbers are pinned and stay. Every other tracked file that
  changes, and every document of the batch, is cited by its name and a section or quoted
  phrase only, never by a line number (S6).

This plan says who does what, in which order, and how each task is known to be done.
What each commit changes is the specification's; the plan repeats none of it.

## 1. Places, tools and records

- **The worktree**: `/home/tibi/Rust/native-theme/.claude/worktrees/v071` (below
  `<v071>`). Every command starts with `cd <v071>` or uses `git -C <v071>`; nothing is
  read, run or written in the main checkout but the work folder below (the GPUI plan,
  §5, *The worktree*).
- **The work folder**: the GPUI plan's,
  `/home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.1_gpui-kit-0.9.1-plan/`
  (`<work>`), excluded by `.git/info/exclude`; GS uses its subfolder `<work>/GS/`:
  `harness/` (E.2's
  stub harness, its stubs in `harness/stubs/`), `e1/` (E.1's scratch copies), `e3/`
  (E.3's case scripts and their runs), `e4/` (E.4's runs), `review/` (the reviewer's
  inputs). Every run goes into a folder of its own, `<work>/GS/<area>/<task>/run-<n>/`
  (`<area>` one of the above, `<task>` the task or E.4 item, `<n>` from 1, named in
  the record). It is outside the worktree, so nothing in it enters the `--quick` key's
  base (spec E.2).
- **The tools**: the GPUI plan's P1 copies, `<work>/tools/slot.sh` and
  `<work>/tools/lowprio.sh` (`<tools>`), with the durable lock folder
  `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/slot-locks/`; no other
  copy of `slot.sh` is used. Every cargo command, and every stubbed run of the gate
  (which calls the real `cargo metadata` and `cargo tree`), runs as
  `<tools>/slot.sh <tools>/lowprio.sh …`; an environment setting goes before the first
  wrapper or through `env` (the GPUI plan, §5). Each stubbed run is wrapped on its own
  (`stubrun.sh` calls the wrappers); an E.3 series is never wrapped as a whole, so it
  holds at most one slot at a time and frees it between runs.
- **The gate lock** (rule 2): every real run of the gate script, `--commit` and `--quick`
  (E.4 items 1, 2, 4 and 5), starts as `cd <v071> && flock <locks>/gate.lock
  <tools>/slot.sh <tools>/lowprio.sh scripts/check_release.sh <mode>`, `<locks>` being
  the slots' lock folder above, so that no other lane's gate runs at the same time (the
  GPUI plan's *The gate command*); an environment change for E.4 item 5 goes before
  `flock` (`env <change> flock …`). E.4 item 3's real `cargo test` and `cargo_tests`
  commands are the orchestrator's long runs, so they start under the same lock, as
  `cd <v071> && flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh <command>`.
  A run outside a gate takes `slot.sh` and `lowprio.sh` only: the stubbed runs (which
  build nothing). GS's gates run no GPUI Fast legs: GS changes nothing
  under `connectors/native-theme-gpui/` (rule 11's test), and the full run is not one of
  its gates.
- **The record**:
  `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.1_gate-speed-plan/record.md`
  (`<record>`), the path the KDE step 1 plan's close-out copies into this plan file in
  its commit of the plans' records. Only the orchestrator writes it, from the agents'
  reports; no tracked file is edited for it during the work. Task 0 creates its folder
  (`mkdir -p`). The lines the GPUI plan's rows ask for (GS's first timed `--commit`, E.4
  item 4's time, GS's last commit) are also written into the GPUI plan's running record,
  `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.1_gpui-kit-0.9.1-plan/record.md`.
- **Long runs**: every gate, every E.3 series and every E.4 item is the orchestrator's
  long run (rules 2, 5; the GPUI plan, §4, *Long runs*): `run_in_background` from
  `<v071>`, through the wrappers, its log and `STATUS` in a new run folder (above),
  waited for by the
  exit notice or a Monitor until-loop on a file (rule 4), never a `pgrep` loop.
- **The hooks**: the project's PreToolUse hooks (the GPUI plan's P1 trial proved them in
  `<v071>`). Files are written with the Write and Edit tools only; a hook that refuses
  an edit is obeyed and its content is never written another way (no `sed -i`, no
  shell redirection into a tracked file). **The one exception** is E.3's temporary
  edits, which the specification has made by script: the comments of (a) to (a5), (b2)'s
  edits and the stub that edits a file while a step runs. An E.3 case script makes each
  such edit and reverts it itself: it saves the file first and restores it from that
  copy, or restores it from the index with `git checkout -- <path>`, never from `HEAD`
  (which would drop a staged change under test); and it ends with `git diff --quiet`
  exiting 0 and `git diff --cached --name-status` printing what it printed before the
  case. No such edit is ever committed. The hooks watch the Write and Edit tools, not a
  script's writes, so these edits stay exactly the specification's: comments, a TOML
  comment line and the named text edits, never code.

## 2. Order

```
[GPUI P1: tools, hook trial] ─ Task 0 (outcome) ─ [GPUI P0: commit (0)] ─ T-i ─ T-ii ─ T-iii ─ T-iv ─ R (review-hard, fixes) ─ final E.3 ─ E4 ─ close ─→ GPUI LN, E1
```

- **Task 0 before P0.** The maintainer's answers to O1 to O3 and decision 4 came with the
  approval of the documents on 2026-11-15 (the GPUI plan's §7), and the
  specification and this plan are written for them; Task 0 records them in `<record>`,
  so that commit (0) commits the documents GS follows as they stand, and GS edits no
  document of its own during the work. It changes no tracked file. Nothing of GS starts
  before Task 0 is done.
- **The commits in Part F's order**: (i) MSRV, (ii) nextest, (iii) the strict cache,
  (iv) `--quick`, each a task below (T-i to T-iv), one `implement` agent continued from
  one task to the next (SendMessage). The harness lives on disk in `<work>/GS/`, so where
  the agent's context runs out a fresh `implement` agent takes over, its brief naming the
  harness's and case scripts' paths, the task it continues and the record's lines so far.
- **The answers.** GS starts only once the maintainer has answered O1 to O3 and decision
  4 (spec Part F); the answers were given on 2026-11-15, and this plan is written for
  them alone: the other outcomes, not taken (2026-11-15), are the specification's *Under
  another outcome*, kept as a record.
- **R after all four** (Part F, *Review*); its fixes are new commits, each with its own
  gate (R below); then the final E.3 series; then E.4.
- **Who writes a case script**: the task whose gate first runs it (T-i: the harness,
  `bash-n`, `m`, `r`, `t`, `c`, `d`, `e-help`, `null`; T-iii: `py-compile`, `selftest`, `e1`,
  `b`, `b2`, `e-args`, `e-nocache-commit`; T-iv: `a` to `a7`, `e-nocache-quick`,
  `e-both`), each later task extending the scripts whose content grows.

## 3. Tasks

Each commit task ends in one commit on `v0.7.1-rc2`, made by the orchestrator after the
gate, on exactly the gated tree, with the author's message (the GPUI plan, §4, *Who runs
what*; rule 2). The author stages its own paths by name (`git add <path>`; never
`git add -A` or `git commit -a`) and reports. Before each gate and again before the
commit the orchestrator makes the GPUI plan's staged-tree check (`git diff --quiet`;
`git status --porcelain --untracked-files=all` showing no `??` line but the untracked
planning documents the GPUI plan's record lists; `git diff --cached --name-status`
showing exactly the task's files, `A` only for its new ones). The commit body says what
changed and why and quotes the gate's summary lines (Part F). The record gets, per
commit: the sha and subject, the gate's run folder, the E.3 summary line, and anything
the author reported as unsure.

### Task 0 — the outcome

- **The answers** (the maintainer, 2026-11-15): O1a no, O1b no, O2 (e), O3's keyed
  option, decision 4 (y); and the 0.7.1 plans approved as written ("When the plan that
  goes in the 0.7.1 version is ready, without errors, start implementing it."). The
  specification and this plan are written for exactly these answers, and both already
  record them (their status lines, the specification's *Open points*, §7 here), so Task 0
  only records them: no list of the specification's *Under another outcome* applies (it
  is marked not taken), and its procedure (a list applied, the grep of its terms, the
  reviews) is not run.
- **By**: the orchestrator.
- **Steps**: `mkdir -p` of `<record>`'s folder; the answers, quoted with their date, as
  `<record>`'s first entry.
- **Files**: `<record>` only; no document and no tracked file changes.
- **Done when**: `<record>` holds the answers; `git -C <v071> status --porcelain
  --untracked-files=no` prints nothing.
- **Estimate**: minutes (an estimate).

### T-i — commit (i): CI's MSRV checks

- **By**: `implement` (Opus, high).
- **Brief inputs**: the specification's header, *Decisions taken while specifying* 1 and
  8, Part M (M.1 to M.3), O2 (e)'s rules (the trap), D.1's first row, D.2 to D.4 as far
  as they describe this commit's state (D.2's rule: a commit names `--quick` only where
  its tree has it, so M.2's header sentence and the usage say "`--commit`"), E.2, E.3 (c),
  (d), (e)'s `--help` case, (m), (t), Part F's row (i); §5 below. The agent first writes
  E.2's harness (it is T-i's first part): `<work>/GS/harness/stubrun.sh <run folder>
  [--before] [--variant <name>[:<argument>]] -- <the gate's arguments>`, which runs
  `scripts/check_release.sh` (or its `_before` copy) with the stub folder first on
  `PATH`, writing the stub's command log `log`, the gate's output `out` and its exit
  status `status` into the run folder; and one case script per E.3 case,
  `<work>/GS/e3/<case>.sh`, each printing `PASS <case>` or `FAIL <case>: <the
  difference>` and exiting 0 or 1, with `<work>/GS/e3/run.sh <task> <case> …` running
  the listed cases in order into `<work>/GS/e3/<task>/run-<n>/` and ending with
  `E.3 <task>: <k> passed, <m> failed`. A case whose content grows from task to task
  keeps its name and takes the task as its first argument, so one name never holds two
  contents unsaid:
  - `bash-n`: one `bash -n` per gate script the tree has, chained with `&&` (`bash -n a
    b` checks `a` alone): `scripts/check_release.sh`, and `scripts/msrv_checks.sh` and
    `scripts/step_cache.sh` from the commits that add them;
  - `py-compile` (from T-iii): `python3 -m py_compile scripts/step_cache_includes.py`
    exits 0;
  - `selftest` (from T-iii): `bash scripts/step_cache.sh --self-test` and `python3
    scripts/step_cache_includes.py --self-test`, both exit 0, with the cases the tree
    has (T-iii: all but the `--quick` key's; T-iv and `final`: all, case 11 among them
    under O3's keyed option);
  - `e1` (from T-iii): E.1's breaks the tree has (T-iii: the first, the third and the
    fourth; T-iv and `final`: all four);
  - `b2` (from T-iii): T-iii the strict half; T-iv and `final` both halves;
  - `c`, `d`: against the gated commit's parent; in the `final` series against
    `0fa28027`;
  - `e-nocache-commit` (from T-iii): first its own stubbed `--commit`, then the record
    sets the tree has asserted not empty (T-iii: the strict set, 34 record files; T-iv
    and `final`: also the `--quick` set, 31 record files), then `--commit
    --no-cache` runs all 34 and leaves every set's hash unchanged;
  - `e-nocache-quick` (from T-iv): first its own stubbed `--commit`, then both sets
    asserted not empty (34 strict, 31 `--quick` record files), then `--quick
    --no-cache` runs all 39 test and docs steps and both `+1.88.0` checks (and the four
    `+1.95.0`), lists none unchanged, and leaves both sets' hashes unchanged;
  - `e-args`: `--no-cache` alone and an unknown argument each exit 2 with their message;
    `e-both` (from T-iv): `--commit --quick` exits 2; `e-help`: `--help` exits 0 and
    prints D.2's usage in the tree's form.
- **Before the agent starts**, the orchestrator checks the two toolchains:
  `rustup toolchain list` lists `1.88.0-x86_64-unknown-linux-gnu` and
  `1.95.0-x86_64-unknown-linux-gnu` (both listed on 2026-11-15); a missing one is a stop
  (installing a toolchain is the maintainer's to allow).
- **Files**: `scripts/msrv_checks.sh` (new, `A`), `scripts/check_release.sh`,
  `scripts/README.md`, `CONTRIBUTING.md`.
- **The agent's own commands** (foreground, each well under 10 minutes): `bash -n
  scripts/check_release.sh && bash -n scripts/msrv_checks.sh`; `<work>/GS/e3/m.sh` (E.3 (m), 17
  calls of `bash -c 'source scripts/msrv_checks.sh && msrv_ci_check <copy>'` from
  `<v071>`: 13 fail naming their difference, 4 pass); the harness's null check, the new
  script run as both "before" and "new" on the unchanged tree (`<work>/GS/e3/null.sh`:
  the two logs equal byte for byte, the two outputs' status lines equal), which shows
  the harness itself adds no difference; one stubbed `--commit` (`stubrun.sh … --
  --commit`), its time recorded (E.2's "about a minute" is the specification's aim, not
  a measurement).
- **The gate** (the orchestrator's long run, on the staged change):
  `cd <v071> && <work>/GS/e3/run.sh T-i bash-n m r t c d e-help` expecting
  `E.3 T-i: 7 passed, 0 failed`, where:
  - `r` is E.3 (r): with the stub `rustup` listing no 1.88.0, the stubbed `--commit`
    exits 1 at the first MSRV step that needs it, naming `toolchain 1.88.0 is not
    installed (rustup toolchain install 1.88.0)`, and runs no `+1.88.0` command;
  - `t` is E.3 (t)'s four cases: a passing stubbed `--commit` exits 0, prints
    the folder's `du -sb` line, and the folder is gone; with `doc -p native-theme-derive
    --no-deps --document-private-items` failing it exits 1 and the folder is gone; a
    folder planted before the run is logged absent at the first `+1.88.0 check`; with
    `check -p native-theme-derive --all-targets` failing it exits 1, prints no `du -sb`
    line and a folder planted as in case 3 is still there (the harness removes it);
  - `c` is E.3 (c) with O1a and O1b answered no: the "before" and new full runs' logs
    equal byte for byte and their status lines equal;
  - `d` is E.3 (d): the new `--commit` log equals the "before" one with the six Part M
    commands inserted right after the last `check -p <crate> --all-targets` line and
    nothing else;
  - `e-help`: `--help` exits 0 and prints D.2's usage in this commit's form.
- **Review**: R, below.
- **Commit**: `scripts(check_release, msrv_checks): CI's MSRV checks in --commit …`.
  Record: the toolchain listing, the harness's null check and its stubbed run's time,
  the gate's summary and the E.3 series' time (the first E.3 series, timed into the
  record as rule 3 asks of a first run; the branch's first timed `--commit` is E.4 item
  1's).
- **Estimate**: harness 2 to 3 hours, the commit 1 to 2 hours, the gate about 10 to 15
  minutes for about ten stubbed runs at the aimed minute each (estimates, not measured).

### T-ii — commit (ii): nextest

- **By**: the same `implement` agent, continued.
- **Brief inputs**: Part B (B.1 to B.4), decisions 10 and 11, D.1's nextest rows, D.2 to
  D.4 in this commit's form (the usage's nextest paragraph, `HAVE_NEXTEST`, "`--commit`"
  alone), E.3 (c), (d), (e)'s `--help` case, Part F's row (ii), §5.
- **Files**: `scripts/check_release.sh`, `scripts/README.md`, `CONTRIBUTING.md`.
- **The agent's own commands**: `bash -n scripts/check_release.sh` (and `&& bash -n
  scripts/msrv_checks.sh`, which T-i added); `cargo nextest
  --version` through the wrappers (installed: nextest 0.9.138, rationale §1); one stubbed
  `--commit`.
- **The gate**: `cd <v071> && <work>/GS/e3/run.sh T-ii bash-n c d e-help`, expecting
  `E.3 T-ii: 4 passed, 0 failed`: (c) the full runs' logs equal and their status lines
  equal (the full run keeps `cargo test`); (d) against its parent, each of the 18 test
  commands replaced by `nextest run --no-tests=pass --status-level fail
  --final-status-level fail --color never [--test-threads N] <args>` and, where
  `doctests_selected` holds, `test <args> --doc` after it, the OS probe's `--list` line
  and the seven runner-environment legs unchanged; `--help` prints the new nextest
  paragraph.
- **Commit**: `scripts(check_release): nextest in --commit …`. Record: the gate's summary.
- **Estimate**: 1 to 2 hours; the gate about 5 minutes (estimates).

### T-iii — commit (iii): the strict step cache

- **By**: the same `implement` agent, continued.
- **Brief inputs**: Part C (C.1 to C.6) but the `--quick` key, decisions 4 (as answered),
  5, 12 to 17, D.1's cache rows, D.2 to D.4 in this commit's form, E.1, E.2, E.3 (b),
  (b2) but its `--quick` half, (c), (d), (e)'s cases without `--quick`, Part F's row
  (iii), §5.
- **Files**: `scripts/step_cache.sh` (new, `A`), `scripts/step_cache_includes.py` (new,
  `A`), `scripts/check_release.sh`, `scripts/README.md`, `CONTRIBUTING.md`.
- **The agent's own commands**: `bash -n scripts/check_release.sh && bash -n
  scripts/step_cache.sh && bash -n scripts/msrv_checks.sh`;
  `python3 -m py_compile scripts/step_cache_includes.py`;
  `python3 scripts/step_cache_includes.py --self-test` (exit 0);
  `bash scripts/step_cache.sh --self-test` (exit 0, each case `  ok  <case>: ran` or
  `skipped` with its note where the specification allows a skip: cases 1 and 3 and the
  helper's unreadable-file case where the user can write or read anyway, case 2 where
  `en_US.UTF-8` is not installed; case 3's assertions run without `pipefail`, in
  the gate's context, each calling the function in a condition, spec C.1); E.1's first, third and fourth breaks in a scratch copy
  under `<work>/GS/e1/` (`step_cache_key` without the command, the helper accepting a
  `concat!` with `..`, the pipe rule broken in `step_cache_inputs` in two forms, each in
  its own copy: (1) `|| return 1` removed, the local `pipefail` kept, (2) the local
  `pipefail` removed, `|| return 1` kept, case 3 failing on each; one break), each seen to
  fail, the failure quoted.
- **The gate**: `cd <v071> && <work>/GS/e3/run.sh T-iii bash-n py-compile selftest e1 b
  b2 c d e-args e-help e-nocache-commit`, expecting `E.3 T-iii: 11 passed, 0
  failed`: the self-test with every case of C.1 and C.4 but those of the `--quick` key,
  which T-iv adds (cases 4, 5 and 8 whole; the `--quick` halves of cases 6, 7 and 9;
  case 11 lands with the `+1.88.0` keying in T-iv), case 10 in; (b) the second stubbed
  `--commit`'s `unchanged since` list exactly the 34, by description, its log holding
  none of their commands; (b2) a comment in `native-theme-derive/src/lib.rs` 26 run, a
  comment line in `docs/property-registry.toml` 21, a root `Cargo.toml` comment 34, a
  `scripts/check_strict_panic.sh` comment exactly the eight `strict-panic` steps, and a
  file changed while `check (native-theme-derive)` runs leaving no record and re-running
  next time; (c) unchanged (O1b answered no); (d) none; (e) `--no-cache` alone and an unknown argument exit 2
  with their messages, `--help` exits 0, and `e-nocache-commit` as defined in T-i (its
  own `--commit` first, the set not empty, then all 34 run and the hash unchanged).
- **Commit**: `scripts(check_release, step_cache): the step cache …`. Record: the
  self-tests' outputs, E.1's three failures, the gate's summary.
- **Estimate**: 4 to 8 hours, the largest task; the gate about 20 to 30 minutes for about
  18 stubbed runs (estimates).

### T-iv — commit (iv): `--quick`

- **By**: the same `implement` agent, continued.
- **Brief inputs**: Part A (A.1 to A.4), decisions 1 to 3, decision 4's *The `--quick`
  reach*, decisions 5 to 7, 9, 17, C.1's `--quick` parts, C.2's `--quick` parts
  (`step_cache_init_quick` and its failure line, A.2's second key in `step_cached` and
  `step_passed`, the summary's count of both sets and its `--quick` list) and C.5's
  `quick/` set, D.1's `--quick` rows (the
  argument parser and mode exclusion, the title, banner and closing note, `--quick`'s
  step list left out, the `--quick` key), D.2 to D.5, E.1, E.3 whole, Part F's row
  (iv), §5; M.1's comment and M.2's first three bullets (the keying), A.2's `+1.88.0`
  bullet, `step_cache_command_crates` and C.1 case 11.
- **Files**: `scripts/check_release.sh`, `scripts/step_cache.sh`, `scripts/README.md`,
  `CONTRIBUTING.md`, `CHANGELOG.md` (D.5) and `scripts/msrv_checks.sh` (M.1's comment
  gains `--quick`, D.2's rule).
- **The agent's own commands**: `bash -n scripts/check_release.sh && bash -n
  scripts/step_cache.sh && bash -n scripts/msrv_checks.sh`; the whole self-test and the helper's (exit 0, the skips T-iii allows and case 5's
  part where the user can read the synthetic tree's `.git` anyway; case 5's assertions
  run without `pipefail`, in the gate's context, each calling the function in a
  condition, spec C.1); E.1's second break
  (`step_cache_quick_environment` keeping `RUST_TEST_THREADS=4`) seen to fail.
- **The gate**: `cd <v071> && <work>/GS/e3/run.sh T-iv bash-n m t selftest e1 a a2 a3
  a4 a5 a6 a7 b b2 c d e-args e-help e-nocache-commit e-nocache-quick e-both` (`m` and
  `t` because T-iv edits `scripts/msrv_checks.sh`), expecting `E.3 T-iv: 21 passed, 0
  failed`. The cases assert (each from the specification's E.3, which
  holds where this table differs):

  | Case | Change | Runs | Listed under `--quick` / strict |
  |---|---|---|---|
  | (a) | a comment in the gpui connector's `lib.rs` | gpui's five test and docs commands (as E.3 (a) lists them), the four `+1.95.0` checks | 26 (24 of the 29 and the two `+1.88.0`) / 34; 31 record files after the `--commit` |
  | (a2) | the gpui showcase's `pages/basic.rs` | gpui's five, iced's seven, the `+1.88.0` workspace check | 18 / 34 |
  | (a3) | `docs/todo.md` | none of the 31 | 31 / 34 |
  | (a4) | `native-theme/src/lib.rs` | 27 of the 29, both `+1.88.0` | 2 / 13 |
  | (a5) | `docs/showcase-exceptions.toml` | all 31 (it is in no strict input) | none / 34 |
  | (a6) | the stub failing the reach's `tree` | all 31, all 34, exit 0 | none / none |
  | (a7) | the stub `rustc`'s `+1.88.0` commit hash | both `+1.88.0`; then both once more; then none | 29, 29, 31 |
  | (b2), `--quick` half | `native-theme-build/src/lib.rs` changed while its `nextest run` runs | no record written; the next run runs it | — |
  | (e) | `--commit --quick`; `--quick --no-cache` after its own `--commit`, both sets checked not empty first | exit 2; all 39 test and docs steps, both `+1.88.0` and the four `+1.95.0` | none, both sets unchanged |

  and (b) as in T-iii; (c) the full runs equal, +0; (d) none at (iv); `e-both` being `--commit --quick`'s exit 2.
- **Commit**: `scripts(check_release, step_cache): --quick, the per-task gate …`.
  Record: E.1's failure, the gate's summary with each case's counts.
- **Estimate**: 3 to 6 hours; the gate about 40 to 60 minutes for about 38 stubbed runs
  (estimates).

### R — the review, and its fixes

- **By**: `review-hard` (Fable, high; no other Fable agent at the same time), read-only,
  no cargo, given the specification and the diff `git -C <v071> log -p <commit (0)>..<the
  last of the four>`, never the author's report; also the harness and case scripts in
  `<work>/GS/`, read for whether each case asserts what E.3 says.
- **Its brief names** what a miss skips silently (Part F): the two keys and what each
  follows (A.2, C.1 to C.4), `QUICK_UNREAD`, the deny-list, the reviewed registry list
  (decision 4 (y)), the include rules, the record conditions (`step_passed`), the
  explicit status checks (A.2), Part M's commands and their check against `ci.yml`'s
  `msrv` job, and the EXIT trap (O2 (e)) and its exit status, the folder's path
  written into the trap's text (`printf %q`) or held in a global never unset; and that no
  new text cites a tracked, changing file by line (S6) and no script names a planning
  document by any path but its archived one (rule 22 (c)).
- **Fixes**: each must-fix is a new commit by the `implement` agent, re-read by the same
  reviewer, and committed by the orchestrator once both have passed. Its gate is the
  case list of the commit it fixes, in the cases' `final` forms
  (`<work>/GS/e3/run.sh fix-<k> <that list>`; `run.sh` treats any task name `fix-<k>`
  as `final` when it chooses a case's content), with `c` and `d` against the fix's parent
  expecting the difference the fix's brief names (normally none); the final series
  against `0fa28027` then checks the whole. A second must-fix on the same point follows the GPUI plan's §6
  take-over rule; a gate failed twice by the same task is not dispatched a third time
  (§6 there).
- **A fix E.4 makes necessary** (Part F, *After E.4*): a new commit, gated as the commit
  it fixes, `review-hard` again, then the final E.3 series and E.4 again.
- **Estimate**: the review 1 to 2 hours; each fix round 1 to 3 hours with its gates
  (estimates).

### The final E.3 series

Always run, whether or not R asked for a fix, once R is closed and its fixes are
committed: `cd <v071> && <work>/GS/e3/run.sh final bash-n m r t py-compile selftest e1
a a2 a3 a4 a5 a6 a7 b b2 c d e-args e-help e-nocache-commit e-nocache-quick e-both` (every
case of T-i to T-iv once, in that order; `null` is not among them, being the harness's
own check in T-i; since (a2) to (a7) each run their own stubbed `--commit` first, spec
E.3, no case depends on the one before it), with `c` and `d` against `0fa28027` ("at
GS's end": (c) +0, O1a and O1b answered no; (d) (i)'s and (ii)'s differences together),
expecting `E.3 final: 23 passed, 0 failed`; its
summary goes into `<record>`. Estimate: about an hour of stubbed runs (estimate).

### E4 — the real runs, and the report

The orchestrator's long runs in `<v071>`, through the wrappers, the gate script's runs
and item 3's commands under the gate lock (§1), each in
`<work>/GS/e4/item<k>/run-<n>/` (§1's convention), time and summary into `<record>`
(spec E.4):

1. `--commit` with both caches empty (`rm -rf target/check-release-cache`), on the clean
   tree at GS's last commit, (iv), the review's last fix, or a fix E.4 made necessary:
   the last fix, if there is one, is committed after its own gate and the reviewer's
   re-read, as the GPUI plan's GS row has it: "on the clean tree at GS's last commit (a
   last fix is committed first, its content the staged change its own gate passed)". It is
   GS's last gate, the branch's first timed `--commit` and the warm-up gate, covering P0's
   tree (the GPUI plan's GS row; rule 3). Expected: exit 0, no failure, only
   `--commit`'s skips; M.1's check and the six MSRV steps passed with their times; the
   1.88.0 folder's `du -sb` line from the trap. `git status --porcelain
   --untracked-files=no` prints nothing before and after.
2. `--commit` again on the unchanged tree: exit 0, its `unchanged since` list exactly the
   34; any other list is a stop (spec E.4 item 2). The 34 steps' summed time in item 1 goes
   into the report.
3. nextest against `cargo test` for the three named steps, each command once each way,
   the counts equal (a stop otherwise); a slower nextest total is reported to the
   maintainer before any other unit runs a gate.
4. `--quick` on the unchanged tree: 31 listed under `--quick` (the 29 and the two
   `+1.88.0`), the 34 under the strict cache, the four `+1.95.0` run, exit 0; its
   time is the floor, written into both records. Where the 31 run instead, a variable
   outside the deny-list that changes between calls is the likely cause, found by
   comparing two runs' `env` and reported (spec E.4 item 4).
5. Another session, simulated (spec E.4 item 5; nothing goes to the maintainer and
   nothing appears on the desktop). First, with `<run>` the item's run folder
   `<work>/GS/e4/item5/run-<n>` and `<saved>` = `<run>/cache-saved`: `mkdir -p <run>`
   (the run folder only; `cache-saved` is not created beforehand, so that `cp -a` makes
   it a copy, not a subfolder), then
   `cd <v071> && find target/check-release-cache -type f -print0 | sort -z | xargs -0 sha256sum | sha256sum | tee <run>/H1 && cp -a target/check-release-cache <saved>`,
   the hash in `<run>/H1` and in `<record>`. Then three `--quick` runs on the unchanged tree, each
   `cd <v071> && env <change> flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh scripts/check_release.sh --quick`,
   each exiting 0:
   - (A) `<change>` sets every per-session variable of the deny-list (the spec's C.1
     list, `CLAUDE_CODE_SESSION_ID` to `AI_AGENT`) to a value other than the session's:
     the 31 listed under `--quick` again;
   - (B) `<change>` is `-u` for each of them: the 31 listed again;
   - (C) the control, `<change>` `GS_ITEM5_PROBE=1` (first checked absent: `git -C <v071>
     grep -n GS_ITEM5_PROBE -- ':!docs/'` and `grep -rl GS_ITEM5_PROBE --include='*.rs'
     ~/.cargo/registry/src/` print nothing; the specification and this plan, under
     `docs/`, name it, which is expected): none of the 31 listed, all 31 run and pass.
   In (A), (B) and (C) the strict 34 run (the strict key holds the whole environment),
   recorded, not a stop. Where (A) or (B) runs any of the 31, one run per variable finds
   the one, reported (a key change for a `review-hard`); where (C) lists any of the 31,
   a stop. Then the saved folder is put back,
   `cd <v071> && rm -rf target/check-release-cache && cp -a <saved> target/check-release-cache && find target/check-release-cache -type f -print0 | sort -z | xargs -0 sha256sum | sha256sum | cmp - <run>/H1`,
   which exits 0 (hygiene: the GPUI
   plan's LN changes `scripts/check_release.sh` before E1, which changes every key's base
   anyway). Each run's change, list and time go into `<record>`. Not tested here:
   variables off the deny-list that change in other situations (`XAUTHORITY`,
   `SESSION_MANAGER`, `XDG_SESSION_ID`, `TERM`, `COLORTERM`, `KONSOLE_VERSION`,
   `CLAUDE_CODE_ENTRYPOINT`); they are keyed, so a change costs re-runs, never a skipped
   step, and is no fault. The deny-list and its NUL-separated parsing are also checked in
   every gate by the self-test's case 4 (spec C.1), which (A) and (B) confirm end to end.

**The report to the maintainer**, at GS's end, with items 1 to 5: the `--commit` times
(item 1, item 2), the strict cache's saving at most (the 34 steps' time), nextest's
against `cargo test`'s (times and counts), the `--quick` floor, the MSRV checks'
recurring cost from item 1's `+1.88.0` times (a fresh folder each run, O2 (e)) and
item 4's four `+1.95.0` times, the 1.88.0 folder's size, item 5's result, and R's
findings with their fixes. It is **completed after E1** with the GPUI plan's E1 numbers
(the first timed `--quick` on a change, a gpui unit, with the three `+1.95.0` gpui
checks' times), each beside its run's whole time (spec E.4 item 4). With those numbers
the maintainer may revisit O3's options and O2's (a) or (d), or drop a cache that saves
little; no threshold is set.

- **Estimate**: item 1 about 10 to 30 minutes (`--commit` measured 5 to 12 minutes a
  commit and 29 cold before the 16-job cap, rationale §1; the MSRV checks' first builds
  add to it, the two `+1.88.0` ones about 19 s a run measured on 2026-11-15, the
  `+1.95.0` ones not measured); item 2 at most item 1's; items 3 to 5 as §8 estimates,
  not measured
  (estimates).

### The close

- Once E4's five items passed (and every fix is committed and R is closed), the orchestrator
  writes into `<record>` and into the GPUI plan's running record the line "GS's last
  commit: <sha> (E.4 item 1 run <folder>; review-hard closed)". `BASE_REV` is set by
  the GPUI plan (LN's last commit, LN coming between GS and E1), not here.
- The GPUI plan's lines: the first timed `--commit` (item 1's time) and the clean-tree
  `--quick` (item 4's time).
- Before E1, the GPUI plan checks that GS changed no file of its specification's E.2
  scope (its E1 row); GS's staged-tree checks have kept every commit to the files named
  above.
- GS's documents are archived by the KDE step 1 plan's close-out (K5 step 7), which also
  copies `<record>` into this plan file; this plan has no archiving task of its own.

## 4. Working rules (the batch-speed rationale's Appendix)

- **Apply here**: rules 1 and 2 (the agent's targeted commands; the gate is the
  orchestrator's, on the staged change); 3 (the first timed run of the branch's gate
  script is E.4 item 1's `--commit`; T-i's E.3 series, GS's first gate, is timed into
  the record too); 4 and 5
  (waits and long runs); 6 and 24 (the cadence: GS is structural, it changes the gate
  scripts, so its last gate is `--commit`; its commits inside the unit are gated by E.3,
  not each by `--commit`, the cost S1 accepts); 8 (the shared limits once the second
  lane runs); 11 (a fix gated by `--quick` does not count; GS's `--commit` restarts the
  count); 14 (D.5's entry goes into the existing `## [0.7.1]` section, one physical
  line); 22 (c) (a script or text naming one of v0.7.1's planning documents names its
  archived path, `docs/archive/…`). **S6**: a citation GS writes, in a script comment, a
  README or this plan, names a tracked, changing file by name and section or quoted
  phrase only; pinned sources (`CR:N` at `0fa28027`, a registry crate at its version,
  `v0.8.0-rc1:` and commit-dated texts) keep their line numbers.
- **Do not apply**: rules 7, 12, 17 to 21 and 25 (no merge with a previous tag in
  v0.7.1); 9, 15 and 16 (the close-out's and the reply's, not GS's); 10 (KDE releases); 13
  and 23 (superseded); 22 (a) and (b) (the close-out's archiving and MG's path check);
  26 (v0.7.1's commit (0) adds the documents, the GPUI plan's header).

## 5. Rules every brief carries

The GPUI plan's §5 holds for every agent of this plan (the worktree, D10's clean room,
the tools, no `find /`, reads and writes inside the worktree, Cargo's home and build
folder, the work folder and the durable inputs it names, the hooks, no invented value,
one report at the end), and:

- The specification's parts are the whole task: nothing beyond them; a needed change they
  do not name is reported, not made.
- Scripts are edited with Write and Edit only; a hook that refuses an edit is obeyed. The
  one exception is E.3's temporary edits, made and reverted by a case script as §1,
  *The hooks*, says.
- The ported code comes from the project's own `v0.8.0-rc1` and `818f0eab`, read with
  `git show`; no code from an LGPL or GPL source (D10). The registry crates are read for
  the reviewed list only.
- No new text cites a tracked, changing file by line (S6, §4).
- The agent runs no real `--commit`, `--quick` or full run, and no E.3 series as a whole
  (the orchestrator's long runs); it runs single case scripts (their stubbed runs
  included), the self-tests and `bash -n`,
  each in the foreground with the 600000 ms timeout where long (rule 1).
- Every stubbed run and every command calling cargo goes through `<tools>/slot.sh
  <tools>/lowprio.sh`; `build-dir` and the target directory are never set; `cargo
  clean` is never run.
- The stub harness never writes inside `<v071>` but under its `target/`, the `_before`
  copies under `scripts/` during E.3 (c) and (d) only (spec E.2), removed after, and
  E.3's temporary edits (§1, *The hooks*), each reverted by its case; a folder planted
  for E.3 (t) is removed by the harness after case 4.
- Every stubbed run is wrapped on its own (§1, *The tools*).
- The report gives the commit message, the files, each command with its result line, and
  everything the agent was unsure of.

## 6. Stops

The GPUI plan's §6 holds: a stop commits nothing and pushes nothing until the maintainer
has been told. Here also: a toolchain not installed (T-i); E.4 item 2's list other than
the 34; item 3's unequal counts; item 5's control run (C) listing any of the 31; a
slower nextest (reported before any other unit runs a gate); an MSRV-only lint E.4 item
1 finds whose fix would change a file of the GPUI specification's E.2 scope (Part F).
A fix E.4 makes necessary is no stop but R's procedure (*A fix E.4 makes necessary*).

## 7. The maintainer's part

**Settled (2026-11-15).** S1 to S5, S3's amendment and S6 were the maintainer's decisions
already (the batch-speed rationale's status line). The points below were put for approval
with the specification and this plan (the GPUI plan's §7), and the maintainer
settled them on 2026-11-15: O1a no, O1b no, O2 (e), O3's keyed option, decision 4 (y),
and the 0.7.1 plans approved as written ("When the plan that goes in the 0.7.1 version
is ready, without errors, start implementing it."), items 1 to 3 and 5 with them:

1. S2's amendment (the next release merged with `<prev-tag>`, not rebased onto it);
2. the Appendix's rules 1 to 26, as worded after S6 and S3's amendment;
3. the re-readings those rules name: rule 7's of S2's "not from the predecessor's tag"
   (a branch made from `<prev-tag>` where that tag exists), rule 9's of S4's "both are
   green" and rule 20's of S2's branch point (the documents unit as the gated head); no
   re-reading of S1 remains;
4. the specification's O1a, O1b, O2, O3 and decision 4 (y), in the table below;
5. this plan's own additions and departures: Task 0 runs before the GPUI plan's P0; E.3's temporary edits are
   made and reverted by case scripts (§1, *The hooks*), the one script-written change of
   tracked files besides the GPUI plan's `remap.py`; E.4 item 5 simulates another session
   mechanically instead of opening one; and the specification's decision 8 (CI's MSRV
   checks in `--commit` and `--quick`, beyond S1), which O3 asks.

| Point | The question | Proposed | Answered 2026-11-15 |
|---|---|---|---|
| O1a | the seven MSRV steps in the full run too | (a) yes: the release gate stays a superset of `--commit` | no |
| O1b | the step cache's self-test in the full run too | (a) yes, as on 0.8.0 | no |
| O2 | where the two `+1.88.0` MSRV checks build | (e) a separate `<build_directory>/msrv-1.88`, deleted by an EXIT trap after every run (about 19 s a run, measured on 2026-11-15) | (e) |
| O3 | CI's MSRV checks in `--commit` and `--quick` at all (beyond S1) | the keyed option: in both modes, the two `+1.88.0` checks keyed in `--quick` | the keyed option |
| decision 4 | registry build scripts and proc macros under the strict key | (y) a reviewed list, `CACHE_REVIEWED_REGISTRY_CODE`, which puts a short review on every unit that changes `Cargo.lock` (in v0.7.1 the GPUI plan's L1 and L2) | (y) |

This plan applies the rules §4 names. Task 0 records the answers; the documents are
written for them.

**After GS**, the report of E4 gives the maintainer the measured times (`--commit` cold
and warm, the cache's saving, nextest against `cargo test`, the `--quick` floor, the
MSRV checks' recurring cost, the 1.88.0 folder's size), the review's findings, and,
after E1, the first `--quick` on a change; with them the maintainer may revisit O2, O3
or the cache.

**During the work, the maintainer is told**: a stop (§6) and a missing toolchain.

## 8. Time

All figures but the measured ones are estimates. Measured (rationale §1): `--commit` 5 to
12 minutes a commit, 29 cold before the 16-job cap; the full run about 23 minutes; the
two `+1.88.0` checks about 19 s a run (spec, O2). Estimated: Task 0, minutes; T-i to T-iv about 11 to 21 hours of agent work in all (the sum of
their estimates: T-i with the harness 3 to 5, T-ii 1 to 2, T-iii 4 to 8, T-iv 3 to 6),
with gates of about 5 to 60 minutes each at the specification's aimed minute per stubbed
run; R and its
fixes 2 to 5 hours; the final E.3 series about an hour; E4 about 1 to 2½ hours of runs
(estimate: items 1 and 2 at most 30 minutes each, from the measured `--commit` times;
item 3 the three steps run twice, 10 to 30 minutes; item 4 one `--quick`, 5 to 15
minutes; item 5 three `--quick` runs, each with the strict 34 running and (C) with the
31 too, 20 to 45 minutes; none of items 3 to 5 measured). The chain is
sequential: one implementer, the gates between its commits, one review.

## Running record

The orchestrator's running record of this plan, copied at the close-out (K5 step 7) from `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.1_gate-speed-plan/record.md`; its headings are one level down.

## Running record: v0.7.1, the gate-speed plan (GS)

Only the orchestrator writes this file.

### 2026-11-16 — Task 0, the outcome (done)

The maintainer's answers of 2026-11-15, given through the session's question dialog (the chosen option
labels quoted):

- O1 (the full run): "No to both (Recommended)" — O1a no, O1b no; the full run is unchanged, as S1 says.
- O2 and O3 (the two Rust 1.88.0 checks): "Fresh folder, keyed (Recommended)" — O2 (e), O3's keyed option.
- Decision 4 (y), the reviewed registry list: approved ("Reviewed registry list (4 y)").
- Also approved in the same dialog: "Batch rules 1–26 + merge", "Extra build folder", "Draft PR on first
  push"; and KDE step 1 "In 0.7.1 (Recommended)" (D10 (a)).
- The instruction of the same day: "When the plan that goes in the 0.7.1 version is ready, without errors,
  start implementing it, even if you still work on other plans".

The specification and the plan are written for these answers; no document was edited by Task 0.

### 2026-11-16 — T-i, commit (i): started

- The build phase starts in a new session (orchestrator on Opus 5.5, high), from the planning session's
  start instructions.
- Toolchain check (plan T-i, *Before the agent starts*): `rustup toolchain list` lists
  `1.88.0-x86_64-unknown-linux-gnu` and `1.95.0-x86_64-unknown-linux-gnu`.
- Dispatched: one `implement` agent (Opus, high) for the harness, the case scripts and commit (i), on
  `e866ea6c`. The brief names T-i's plan sections and the spec parts by heading. Gate to follow (the
  orchestrator's): `run.sh T-i bash-n m r t c d e-help`, expecting `E.3 T-i: 7 passed, 0 failed`.

### 2026-11-16 — T-i, commit (i): done

- Commit: `32218eda5f4456e32315ad97e60c71cd7475537e` "scripts(check_release, msrv_checks): CI's MSRV checks in
  --commit: …", on `v0.7.1-rc2`, not pushed. Files: `A scripts/msrv_checks.sh`, `M scripts/check_release.sh`,
  `M scripts/README.md`, `M CONTRIBUTING.md` (staged-tree check before the gate and before the commit).
- Gate: `<work>/GS/e3/T-i/run-18`: `E.3 T-i: 7 passed, 0 failed` (bash-n m r t c d e-help), the series 23 s
  (the first E.3 series, rule 3). `git status --porcelain --untracked-files=no` the same before and after the
  gate, empty after the commit.
- The agent's runs: the null check `PASS null` (both runs exit 0, 208 log lines, 146 status lines, equal);
  one stubbed `--commit` (`<work>/GS/harness/T-i/run-1`) 2.8 s, `146 checks · 146 ✅`; `m.sh` 13 exit 1
  and 4 exit 0; (t)'s four cases as E.3 (t) says; (d): the new `--commit` log, 208 lines, is the "before"
  log, 202 lines, with the six commands inserted after the last `check -p` line.
- Diff read by the orchestrator (an Opus diff of 376 lines): it matches Part M and D.2's rule (the texts say
  `--commit` only). The hard-failure path for a missing toolchain (`print_fail`, then `exit 1`) is the same as
  `run_check`'s. No finding.
- The agent was unsure about these points (none blocks):
  - the folder name is computed, `msrv-<major.minor>` of `rust-version`;
  - `msrv_checks` lives in `msrv_checks.sh` and uses `run_check` and `print_fail`;
  - the full run prints no "skipped" line, so (c) is byte-equal;
  - `## msrv_checks.sh` sits after `## check_release.sh`; commit (iii) inserts the `step_cache` sections
    before it (D.3's order);
  - a stubbed run takes about 3 s, against the spec's aimed minute;
  - the stub's settings go through files in `harness/state/`, not the environment;
  - the `tree-fail` and `edit` variants are written but not yet tested; the `rustc` stub for (a7) is not
    written; `d.sh` and `e-help.sh` hold T-i's expectations only;
  - M.2's header sentence's "them" was kept as the spec words it.
- The agent's earlier development runs are also in `e3/T-i/` (run-1 to run-9) and `e3/adhoc/`. The recorded
  ones are `e3/T-i/run-10` to `run-17`, `harness/T-i/run-1` and the gate's `run-18`.

### 2026-11-16 — T-ii, commit (ii): done

- A fresh `implement` agent built it (T-i's agent was not continued: its context was about 240 k tokens; the
  harness is on disk).
- Commit: `06fe9776b4f299212527481ba482257c7fff9209` "scripts(check_release): nextest in --commit: …", not
  pushed. Files: `M scripts/check_release.sh`, `M scripts/README.md`, `M CONTRIBUTING.md`.
- Gate: `<work>/GS/e3/T-ii/run-5`: `E.3 T-ii: 4 passed, 0 failed` (bash-n c d e-help), 12 s; status unchanged
  by the gate, empty after the commit.
- The agent's runs:
  - `cargo nextest --version`: `cargo-nextest 0.9.138 (fc97e97bb 2026-06-21)`.
  - (d) T-ii: the "before" log has 208 lines, 26 of them test lines (18 replaced, 1 listing, 7 legs);
    16 doctest lines expected; `--test-threads 16`; the new log has 224 lines.
  - (c) T-ii: 230 log lines and 169 status lines on both sides.
  - (d) and `e-help` re-run for T-i against `e866ea6c`..`32218eda`: PASS.
  - One stubbed `--commit` (`harness/T-ii/run-1`): 3.0 s, 224 log lines.
- Diff read by the orchestrator: it matches B.1 to B.3 (twelve commands moved to `cargo_tests`; the probe's
  listing, the stem-index test and the runner-environment legs untouched). CONTRIBUTING's paragraph is
  `818f0eab`'s, cut to (ii)'s form. No finding.
- Harness changes:
  - `lib.sh`'s `gs_task_sides` finds a task's commit by its subject; a committed task's (d) compares its
    parent with it through `_before` and `_after` copies (the `_after` copies go beyond E.2; removed after
    the case).
  - `stubrun.sh --after`; each run writes an `environment` file.
  - `d.sh` and `e-help.sh` gained T-ii's forms.
- The agent was unsure about these points:
  - an empty `RUST_TEST_THREADS` or `NEXTEST_TEST_THREADS` counts as unset;
  - B.2's `--color never` and `--test-threads` were followed over CR8, which has neither;
  - README and CONTRIBUTING also say that the runner-environment legs stay on cargo test;
  - `stubrun.sh` (work folder, untracked) was edited by a Python replacement, not by Edit.
- A 14th untracked file, `docs/todo_v0.7.2_gpui-kit-0.10.0-rationale.md`, appeared during T-ii: the
  planning session's v0.7.2 document. It is left untracked and is now on the GPUI record's list.

### 2026-11-16 — T-iii, commit (iii): done

- A fresh `implement` agent built it; its context ended at about 490 k tokens, so it is not resumed.
- Commit: `d733a440ef3625dbe84bb0886dec6adcb18a2755` "scripts(check_release, step_cache): the step cache in
  --commit: …", not pushed. Files: `A scripts/step_cache.sh`, `A scripts/step_cache_includes.py`,
  `M scripts/check_release.sh`, `M scripts/README.md`, `M CONTRIBUTING.md`; 1963 insertions, 19 deletions.
- Gate: `<work>/GS/e3/T-iii/run-24`: `E.3 T-iii: 11 passed, 0 failed` (bash-n py-compile selftest e1 b b2 c d
  e-args e-help e-nocache-commit), 145 s; status unchanged by the gate, empty after the commit.
- Self-tests:
  - helper: exit 0, 32 `ok`, no skip;
  - `step_cache.sh --self-test`: exit 0, 38 `ok` in four groups, no skip; full output in
    `<work>/GS/e3/T-iii/run-15/selftest/step_cache.out`.
- E.1's breaks, each in a scratch copy in `<work>/GS/e1/T-iii/run-2/`. The unbroken control copy passed
  first, then each break failed:
  - the key without the command: "the command's arguments changed: expected ran, got skipped";
  - `..` accepted in a `concat!`: "a concat! with .. refused: expected ('refused', 'holding ..'), got
    accepted";
  - the pipe rule, form (1): case 3 fails, "expected inputs failed; key failed; … got inputs 0eec…";
  - the pipe rule, form (2): case 3 fails on the two inputs assertions.
- Counts on this tree:
  - (b): exactly the 34 unchanged;
  - (b2): derive comment 26, registry line 21, root `Cargo.toml` 34, `check_strict_panic.sh` 8; the edit
    made during `check (native-theme-derive)` leaves no record;
  - `e-nocache-commit`: 34 record files, all 34 run, the hash unchanged.
- Stubbed `--commit` times: 24.7 s cold and 10.9 s warm (`harness/T-iii/run-1`); 11.3 s and 9.6 s later.
- Reviewed registry list: 23 build scripts and 15 proc macros, 37 packages (rustversion has both). They are
  the same 37 name/version pairs that `cargo metadata` and `cargo tree` give for the seven accepted crates.
  Only gpui, iced and egui reach `cc` and `pkg-config`.
- Orchestrator's reading, scoped: `check_release.sh`'s cache hooks (each key computation tested as a
  condition; `--no-cache` alone exits 2; the full run has no cache). The two new scripts (1598 lines) are
  R's, by the plan (review-hard over the four commits).
- **For R**:
  - the agent added an include rule beyond C.4: an include inside a macro body is also refused where the
    defining file has a `use` item naming the macro (it can only refuse more);
  - the cache label says "with uncommitted changes" whenever `git status --porcelain` lists anything,
    untracked documents included, so in `v071` it always does (cosmetic);
  - the self-test's `ok <case>: ran|skipped` reports the step's outcome; a skipped case prints `skip`;
  - the gate hashes the ancestors' `.cargo/config*` and `clippy.toml` up to `/`, as C.1 asks; from `v071`
    that includes the main checkout's root.
- Harness changes:
  - `lib.sh`: `gs_strict_table` (the 34 steps), `gs_check_strict_run` and related helpers;
  - `stubs/python3` passes the `_after` helper copy through;
  - `d.sh` and `e-help.sh` have T-iii's forms;
  - new case scripts: `py-compile`, `selftest`, `e1`, `b`, `b2`, `e-args`, `e-nocache-commit`.

### 2026-11-16 — the maintainer's changes to the order (speed)

Asked whether the work could go faster at equal quality, the maintainer approved four departures from the
plans in the session's question dialog (option labels quoted): "Split R, start now", "E4 beside R", "LN in
own worktree", "L1+L2 one gate". What they mean:

- **R in two parts.** A `review-hard` (Fable) over commits (i) to (iii) starts while T-iv is being built,
  and a short `review-hard` pass over (iv) follows once it is committed. The reading of the harness and
  case scripts (whether each case asserts what E.3 says) goes to an Opus `review` agent after T-iv's commit,
  since T-iv extends them. The fix procedure stays R's.
- **E4 beside R.** E.4's real runs start on the clean tree at commit (iv) while the reviews read. A fix R
  asks for is followed by the final E.3 series and E4 again, as for a fix E.4 makes necessary.
- **LN in a worktree of its own**, made from `d733a440` and running beside GS. It is merged into
  `v0.7.1-rc2` after GS closes, with LN's own `--commit` gate on the merged tree. Its build folder is
  deleted after the merge.
- **L1 and L2 under one `--commit`.** On a red result the two updates are split to find the one that broke
  it.

### 2026-11-16 — R part 1 (`review-hard`, commits (i) to (iii)): findings and decisions

The reviewer read through git and probed on scratch copies; nothing in `v071` was written. Verdict: merge
after the two must-fix items. **Nothing skips a check on the tree at `d733a440`.** Every finding is either a
latent fail-open hole, reproduced on a synthetic package, or a test gap, or a nit.

The orchestrator settles each by logic: a cache key that can be fooled is the silent skip that decision 4 (y)
and C.4 exist to prevent. Each fix below only refuses more, so it can make a step run, never be skipped, and
none is triggered by today's tree. The departures from C.2's and C.4's wording are recorded here and in the
README's sections; the specification is not edited (no review pass on the 0.7.1 documents).

1. **must-fix**: `PATH_ATTR` misses the inner form `#![path = …]`. Fix: `#\s*!?\s*\[\s*path\s*=\s*`, with a
   self-test case.
2. **must-fix**: reach paths holding `*`, `?` or `[` go through `step_cache_inputs`'s glob branch, so an edit
   is hashed as "missing". Fix: refuse such a reach line in `rust_reach_resolve` (or hash it as a literal
   path), with a self-test case.
3. **should-fix, taken**: refuse a `#[path]` target not ending in `.rs` (the reason C.4 gives for `include!`).
4. **should-fix, taken**: refuse a `use` item that renames one of the three include macros, and any
   `include_<word>!` that is not one of them (`egui::include_image!`, for example).
5. **should-fix, taken**: metavariable literals that join into `..`. Fix: refuse an invocation literal that
   starts or ends with `.`.
6. **should-fix, taken**: a target source outside its package is hashed but not scanned. Fix: refuse the reach.
7. **should-fix, taken**: the reviewed list matches name and version only, so a `[patch]` git fork at a
   listed version is accepted. Fix: only crates.io-sourced packages match the list; other sources' build
   code is refused.
8. **should-fix (tests), taken in part**: add self-test cases for a listed name at another version and for one
   config-file edit (`Cargo.lock`, `clippy.toml` or `.config/nextest.toml`). Not taken: moving
   `step_cached`/`step_passed` into `step_cache.sh`, since C.2 places them in `check_release.sh`. Their
   record condition is tested end to end through `check_release.sh` by E.3 (b2)'s "a file changed while a
   step runs" cases, strict and `--quick`.
- **Minors, taken**:
  - the dead store `STEP_CACHE_FULL_METADATA="$1"`;
  - a `jq` failure in `rust_reach_resolve`'s build-code test must refuse (test the negation);
  - `cache_tool_versions`' statuses are checked, a failure turning the cache off;
  - `env -u CARGO_ENCODED_RUSTFLAGS` in the six MSRV commands, since Cargo prefers it over `RUSTFLAGS`.
- **Minor, recorded as a residual, not changed**: the program behind `build.rustc-wrapper` is not keyed, only
  its path through the config file (the user's global configuration).

These go into the one fix round after commit (iv), together with the second `review-hard` pass over (iv),
the Opus review of the harness, and whatever E4's early runs find.

### 2026-11-16 — T-iv, commit (iv): done

- Built by a fresh `implement` agent, whose context ended at about 500 k tokens; it is not resumed.
- Commit: `30a53e29aa232663b2111427f6ee112396b33999` "scripts(check_release, step_cache): --quick, the per-task
  gate: …", not pushed. Files: `M` CHANGELOG.md, CONTRIBUTING.md, scripts/README.md,
  scripts/check_release.sh, scripts/msrv_checks.sh, scripts/step_cache.sh; 1087 insertions, 180 deletions.
- Gate: `<work>/GS/e3/T-iv/run-26`: `E.3 T-iv: 21 passed, 0 failed`, the series 1099 s. The status was
  unchanged by the gate and empty after the commit.
- The agent's runs:
  - every count matches plan T-iv's table: (a) 26/34 with 31 + 34 records; (a2) 18/34; (a3) 31/34;
    (a4) 2/13; (a5) 0/34; (a6) 0/0, exit 0; (a7) 29, 29, 31; (b) the 34; (b2) 26, 21, 34, 8 and both
    "while it runs" halves; `e-nocache-quick` ran 39 + 2 + 4;
  - self-tests: the step cache 55 ok, the helper 32 ok, no skip;
  - E.1's second break failed the self-test ("RUST_TEST_THREADS=4 changed");
  - stubbed `--commit` 36.5 s, stubbed `--quick` 19.6 s.
- The agent was unsure about these points (all handed to R part 2):
  - the `--quick` key decides whenever the strict key is not computed, "an input cannot be hashed" included;
  - an empty base fails `step_cache_init`;
  - the info-line wording;
  - refusal notes de-duplicated by their text;
  - a git path that needs quoting gives no `--quick` base;
  - the `--quick` reach reuses the strict reach's inputs;
  - a re-key costs about 0.4 s per step.
- Harness: `GS_FORM` (`fix-<k>` is treated as `final`), the 31-step `--quick` table, the `rustc` stub, new
  cases a to a7, `e-nocache-quick` and `e-both`; `d.sh` rewritten for `final` and `fix-<k>`. The agent noted
  that `c.sh`'s T-i to T-iii forms still compare HEAD with the tree.

### 2026-11-16 — after (iv): three things in parallel (the maintainer's "Split R" and "E4 beside R")

- E4 item 1, the early real run on `30a53e29`: `<work>/GS/e4/item1/run-1`, running.
- R part 2: a `review-hard` (Fable) over `30a53e29` alone, told which part-1 findings are already decided.
  The planning session was told.
- An Opus `review` of the harness and case scripts against E.1 to E.3.
- Then one fix round with every finding, its gate, the Fable re-read, the final E.3 series, and E4 in full
  on the last commit.

### 2026-11-16 — the harness review (Opus `review`): findings, all taken

The reviewer read the harness and case scripts against E.1 to E.3 and ran no case. The verdict was must-fix
first, then a re-run of `c` for the committed tasks. Every finding is taken in the fix round. The harness is
the work folder's, not a commit.

1. **must-fix**: `c.sh` uses `rev=HEAD` for T-i to T-iv and `fix-*`, so once a task is committed it compares
   a script with itself. At each task's gate the change was staged and HEAD was the parent, so those gates
   were sound (run-26: `before: HEAD (d733a440)`). `d.sh`'s `fix-*` form has the same flaw. Fix:
   - `c` takes its sides from `gs_task_sides` and the `_after` copies, as `d` does;
   - `c` and `d` report a problem when every "before" copy is byte-identical to its new side (outside `null`).
2. **should-fix**: `selftest` accepts an allowed skip without checking that its condition holds. Fix: a
   write/read skip is allowed only under `id -u` 0, the locale skip only where `locale -a` lacks
   `en_US.utf8`.
3. **should-fix**: `gs_finish` never runs `git diff --quiet` (plan §1, *The hooks*). Fix: add it.
4. **should-fix**: `b` checks only the commands the stubs log. Fix: also require the second run's status
   lines to equal the first's minus the 34 descriptions.
5. **should-fix**: (a) takes its expected lines from the gate's own `--commit` log. Fix: assert E.3 (a)'s
   11 lines as fixed patterns, with their count, and exactly four `+1.95.0` checks in every `a*` case.
6. **should-fix**: `run.sh` ignores the case's exit status, and its `^(PASS|FAIL) <case>` grep has no
   delimiter (`a` matches `a2`). The checkers run inside `$(…)` unchecked. Fix: a case passes only on exit
   0 with the line `^PASS <case>$`, and each checker ends with a sentinel line that its caller requires.
- **Nits, taken**:
  - `e1`'s "pipefail kept" count is limited to `step_cache_inputs` and asserted;
  - `b2` uses the shared `gs_save` and `gs_restore_saved`;
  - before each case, a guard checks that `stubs/` holds only `cargo` and `python3` and that no
    `scripts/*_before.*` or `*_after.*` is left.
- **Nit, not taken**: E.2's wording on the moved-aside hash around `--no-cache`. The live-folder comparison
  is the check E.3 (e) needs, and the series end covers the moved-aside folder.
- Confirmed sound by the reviewer:
  - the 34/8/31/39 tables;
  - the before/after rewrite;
  - every temporary edit saved and restored against the index;
  - the exit statuses reaching `status`;
  - bash running the EXIT trap on SIGTERM.

### 2026-11-16 — R part 2 (`review-hard`, commit (iv)): findings and decisions

Verdict: merge, with no must-fix. The reviewer read through git and probed on scratch copies. All the
findings are taken in the fix round; each makes the gate run more, never skip more.

1. **should-fix (a gap in A.2's rule)**: a `--quick` record survives a later failure of the same step in
   `--commit`, since a quick-keyed step runs in `--commit` whatever its record says and can fail with exactly
   the recorded key (a flaky test, the desktop, the network). The reviewer verified it with the commit's own
   functions. Fix: a quick-keyed step that fails, in either mode, deletes its `--quick` record; a pass
   writes it again.
2. **should-fix**: where `git ls-files` exits 0 with an empty listing (a copy inside another repository's
   ignored folder), the base covers no files. Verified. Fix: `step_cache_quick_base` returns 1 where the
   listing does not hold the root `Cargo.toml` and `scripts/step_cache.sh`; add a self-test case.
3. **should-fix**: the decided glob finding also hits the base's file list (`step_cache_inputs` is called
   with git's paths). Verified. Fix: the same fix, paths taken literally in every call.
4. **should-fix**: `"$(cache_tool_versions)"` passed as an argument in two places hides its status (C.1's
   pipe rule). Fix: assign first, test, then pass.
- **Minors, taken**:
  - a. self-test cases for the gate's real order (a strict refusal, then the `--quick` form; the reuse of
    an accepted strict reach), for `step_cache_init` with an empty base, and for a command with neither
    `-p` nor `--workspace`;
  - b. `git ls-files -z`, read NUL-separated, in place of the quoted-path refusal;
  - c. the strict refusal's note in `--quick` says that no strict key is computed, not that the steps run;
  - d. the two `+1.88.0` checks marked by `msrv_check` (a flag or their `--target-dir`), not by comparing
    `$MSRV_TOOLCHAIN`;
  - e. `scripts/README.md` and `CONTRIBUTING.md` add A.4's limit "a run-time read of a file in
    `QUICK_UNREAD`".
- The reviewer confirmed:
  - `QUICK_UNREAD` holds nothing a test, build script, doctest or rustdoc reads, apart from
    `docs/platform-facts.md`, which the include scan keeps;
  - the deny-list equals C.1's;
  - the strict-first rule holds, and the fallback when the strict key cannot be hashed is safe;
  - the `+1.88.0` keying and the trap are right;
  - the usage and the CHANGELOG line equal D.2 and D.5 by `diff`.

### 2026-11-16 — E4 item 1, early run on (iv), and the fix round

- **E4 item 1 on `30a53e29`** (early; the full E4 runs again on the last commit), `<work>/GS/e4/item1/run-1`:
  - `--commit` with both caches emptied: STATUS 0, 942 s (`15m 42s total`);
  - `147 checks · 147 ✅ · 0 ⚠️ · 0 ❌ · 0 unchanged`, the only skips `--commit`'s;
  - `git status --porcelain --untracked-files=no` empty before and after.
  - MSRV: the `ci.yml` check 0 s; `+1.88.0` 15 s (the workspace) and 5 s (native-theme); `+1.95.0` 25 s
    (egui), 19 s (gpui), 15 s (gpui native,watch), 23 s (gpui gpui-fast,native,watch).
  - The 1.88.0 folder: `628345973	/run/media/tibi/work/cargo-build/fe/88b35d036b2878/msrv-1.88` (the trap's
    `du -sb` line).
  - The step cache's notes: native-theme-gpui, -iced and -egui are not keyed, their reach holding
    `pkg-config cc`.
- **The fix round**: one fresh `implement` agent, `fix-1`. Its brief lists every decided finding: R part 1
  1–8 and minors, R part 2 1–4 and minors a–e, the harness review's 1–6 and nits. They are grouped A1–A4
  (helper), B5–B11 (reach and inputs), C12–C15 (check_release), D16 (msrv), E17–E18 (texts), F (self-test
  cases, each seen to fail first, plus a new stubbed case `r12` for C12 and C13) and H1–H7 (the harness,
  first). Gate to follow: `run.sh fix-1` over every case plus `r12`.
- E4 item 3 runs beside the fix (it uses commands outside the scripts): `<work>/GS/e4/item3/run-1`,
  through `<work>/GS/e4/item3.sh`.

### 2026-11-16 — E4 item 3: nextest against cargo test (done; its result stands for the final E4)

The commands are those of `30a53e29`. The fix round does not touch `cargo_tests`, so item 3 is not repeated
after the fix.

- **Counts: equal** (`<work>/GS/e4/item3/run-1`):

  | Step | cargo test passed / ignored | nextest passed + doctests passed | nextest skipped + doctests ignored |
  |---|---|---|---|
  | native-theme, Linux features | 1398 / 11 | 1325 + 73 = 1398 | 8 + 3 = 11 |
  | native-theme-gpui, native, watch | 785 / 6 | 751 + 34 = 785 | 4 + 2 = 6 |
  | native-theme-egui, all features | 368 / 10 | 352 + 16 = 368 | 10 + 0 = 10 |

- **Times.** Run 1 timed each step's nextest and doctests together, beside the fix agent's work: cargo test
  3 + 15 + 22 = 40 s, nextest with doctests 20 + 13 + 17 = 50 s. Its native-theme 20 s was a one-time cost
  of the first nextest invocation; what that cost consisted of was not verified.
- The timing was repeated before anything was reported (`<work>/GS/e4/item3/run-2`, through
  `item3-timing.sh`): each way twice, in alternating order, nextest and doctests apart, timed inside the
  slot. The script's `printf "%.1f"` failed under the Hungarian locale (decimal comma); the exact values are
  in the error lines of the task output, and they are these:

  | Step | cargo test (r1 / r2) | nextest + doctests (r1 / r2) |
  |---|---|---|
  | native-theme, Linux features | 2.69 / 2.69 s | 2.19 + 1.23 = 3.42 / 2.17 + 1.23 = 3.40 s |
  | native-theme-gpui, native, watch | 14.71 / 14.92 s | 11.03 + 1.45 = 12.48 / 11.36 + 1.43 = 12.79 s |
  | native-theme-egui, all features | 21.97 / 22.66 s | 15.67 + 0.90 = 16.57 / 15.72 + 0.89 = 16.61 s |
  | total | 39.4 / 40.3 s | 32.5 / 32.8 s |

  **nextest is faster in total**, by about 7 s (about 18 %), in both rounds and both orders. It is slower
  only on native-theme's fast suite, by about 0.7 s, the cost of the separate doctest pass. Not a stop. It
  goes into GS's report to the maintainer.

### 2026-11-16 — fix-1: staged, gated, in re-read

- The agent's context ended at about 355 k tokens; it is not resumed for new work. Staged: `M` CONTRIBUTING.md,
  scripts/README.md, scripts/check_release.sh, scripts/msrv_checks.sh, scripts/step_cache.sh,
  scripts/step_cache_includes.py; 502 insertions, 192 deletions. The staged tree is
  `e4c7779050e9bac4f2e785509ef829be4f3ac6d8`; the diff is saved at `<work>/GS/review/fix-1.diff`.
- Every fix A1 to E18 was built as decided. Each new self-test case failed on the unfixed code, its failure
  quoted in `<scratchpad>/fix1/{helper,step}-unfixed.out`; so did `r12`, on the unfixed `check_release.sh`
  (`e3/fix-1/run-2/r12`). The five "also" cases already passed before (behaviour that was correct). The
  helper has 37 ok; the step cache's self-test passes; the include scan over the workspace gives the same
  20 targets as before.
- Harness H1 to H7 done. H1 proved: `c.sh T-iii` (06fe9776 against d733a440) and `c.sh T-iv` (d733a440
  against 30a53e29) pass with differing sides, and `c.sh fix-1` failed correctly before the fix existed.
- The agent's judgement calls:
  - B5: reach paths with glob characters refused, `step_cache_inputs --literal` for the `--quick` base;
  - B7: the strict reach only (A.2's `--quick` reach drops registry-code refusals);
  - C15: both notes, in `--quick` only;
  - C12: deletes on every failure while `step_cache.sh` is loaded;
  - the crates.io source string verified on this machine only;
  - a few harness edits made by `sed` or `python3` (untracked work folder, no hook involved).
- Gate: `run.sh fix-1` over 25 cases, running.
- Re-read: one fresh `review-hard` (Fable) over the saved diff and tree, given both findings lists and the
  decisions; the planning session was told. A fresh agent rather than the two earlier reviewers (258 k and
  294 k tokens), to cost less.

### 2026-11-16 — the fix-1 re-read (`review-hard`): result and decisions

- Every decided fix A1 to F is done as decided. The reviewer confirmed:
  - B5: only the `--quick` base passes git's paths, as `--literal`; reach lines with glob characters are
    refused before they are memoised;
  - B11: git's status read from the last entry, an empty listing caught by B10, newline and backslash
    paths followed;
  - C12's deletion: `step_failed` always returns 0, so it cannot change a status or end the run.
  The verdict: commit after one must-fix (N1).
- **N1, must-fix, taken in code**: C12 does nothing under `--no-cache` or in the full run, because
  `step_cache.sh` is sourced only when the cache is on. So a `--commit` pass, then a full-run failure of the
  same step on the same tree, leaves the `--quick` record, and the next `--quick` skips the step. The
  README's sentence overstates it. Fix: source `step_cache.sh` in every mode (sourcing runs nothing); let
  `step_cache_forget` find the cache folder without `step_cache_init`; the texts say `--no-cache` "reads no
  record and records no pass; a failure still removes the step's `--quick` record". The full run's output
  stays byte-equal (E.3 (c) checks it): the deletion prints nothing unless it fails.
- **N2, should-fix, taken**: the metavariable form still accepts joins into `..` by three routes (char
  literals, adjacent body literals, an empty literal between body literals), each verified with rustc
  reading outside the package. Fix: refuse any `concat!` string part ending in `.`, and refuse an
  invocation holding a char literal; one case per route.
- **N3, should-fix, taken**: `MSRV_QUICK_TOOLCHAIN` can be inherited from the environment. Verified: with it
  exported, the refused crates' `check` steps take a `--quick` key. Fix: set it to "" at the top level of
  `msrv_checks.sh`.
- **N4, should-fix, taken**: cases that can fail:
  - C12's `step_cache_forget`: record, forget, miss; forget with no record; forget in an unwritable folder;
  - C15's note text;
  - B5 for an include target and a configuration path;
  - A4's `startswith` and `endswith` apart;
  - B8, a failing `jq`;
  - B11, a newline in a path.
- **N5, nits, taken in the scripts**: `step_cache.sh`'s header on git-sourced build code; the note's "may
  still skip" also names the two rust-version MSRV checks, and drops "may" where the `--quick` set is off.
- **N5, the spec's text, not edited now**: A.2 and C.4 do not yet describe the fixes (the MSRV marker,
  outside target sources refused, A1 to A4, `git ls-files -z`, `env -u`). Recorded for GS's close: the
  specification is corrected to the code before it is archived (K5 step 7), in one documents commit.
- **The reviewer's question, outside GS's scope, not changed**: `CARGO_ENCODED_RUSTFLAGS` and
  `CARGO_ENCODED_RUSTDOCFLAGS` could also drop the `-D warnings` of the cross-target check steps and the
  `cargo doc` steps. Those are 0.7.0's steps, not GS's. The keys hold those variables, so no skip is wrong;
  only the denial can be lost. For the maintainer's report and `docs/todo.md`.
- The follow-up goes to a fresh `implement` agent (fix-1's agent is at about 355 k tokens), added to the
  same staged change, so `fix-1` stays one commit. Then the gate runs again and a short Fable re-read of the
  delta follows.
- The first fix-1 gate, on tree `e4c77790`: `<work>/GS/e3/fix-1/run-27`, `E.3 fix-1: 24 passed, 0 failed`,
  1491 s, status unchanged. It is superseded by the gate after N1 to N5.
- Follow-up dispatched: a fresh `implement` agent for N1 to N5, staging into the same change.

### 2026-11-16 — speed decisions (the maintainer: "the codebase is yours … your call")

1. fix-1's gate and the final E.3 series are one run: `run.sh final <every case> r12`, plus `c d` in
   `fix-1`'s form. They differ only in `c`'s and `d`'s base.
2. No second Fable re-read of the follow-up. N1 to N4 each get a test that fails first; the orchestrator
   reads the delta. A delta over about 300 lines goes to an Opus `review`.
3. LN (`v0.7.1-rc2-ln` at `92ec9297`) is merged into `v0.7.1-rc2` after fix-1's commit and before E4. E4
   item 1's cold `--commit` is then both GS's last gate and LN's structural gate. LN touches none of the 34
   cached steps' inputs but the facts checker's step label.
4. The spec's text corrections (N5) go into the H/O documents unit, not a commit and gate of their own.
5. A new session at GS's end; shorter record entries from here.

### 2026-11-17 — fix-1 committed, LN merged, E4 running

- Follow-up N1 to N5 staged with fix-1; delta 236 insertions and 69 deletions, read by the orchestrator, no
  finding.
  - N3 needed `unset` too: an exported variable stays exported when assigned. Stubbed runs with it exported
    gave 5, 29 and 31 `--quick` records.
  - The usage now carries N1's wording, unlike spec D.2: added to N5's spec-correction list.
  - A 15th untracked planning document appeared, `docs/todo_v0.7.2_egui-0.37-and-gpui-kit-0.10.0-plan.md`
    (the planning session's); left alone.
- Gate (decision 1): `<work>/GS/e3/final/run-2` `E.3 final: 24 passed, 0 failed` (1473 s, every case, `c`
  and `d` against `0fa28027`, plus `r12`), and `<work>/GS/e3/fix-1/run-54` `E.3 fix-1: 2 passed, 0 failed`
  (`c d` against `30a53e29`); status unchanged.
- **fix-1: `4e03057bcebb3acaadb3cab7de37b4f5ce58e532`**, the A1–F and N1–N5 lines in its body.
- **LN merged: `925125a7366f519f976f9dc678b2170f742ccb87`** (`--no-ff` of `v0.7.1-rc2-ln` at `92ec9297`). Only
  `scripts/README.md`'s `check_preset_citations.py` section conflicted: LN's text taken with GS's "(every
  mode)". It was resolved by a short python edit, not the Edit tool (Markdown, no hook concern; recorded as a
  departure from the edit rule). Before the commit: `bash -n` ok, the five checkers exit 0, `--self-test
  ok`; the CHANGELOG auto-merged with LN's one line.
- E4 items 1, 2, 4 and 5 on `925125a7`: `<work>/GS/e4/e4.sh 2`, run folders `item<k>/run-2` and
  `item5/run-2/{A,B,C}`, summary `<work>/GS/e4/summary-2`. Item 1 is GS's last gate and LN's structural gate
  (decision 3).
- LN's worktree removed (it was clean), its branch deleted (it was at `92ec9297`), and its build folder
  `/run/media/tibi/work/cargo-build/7d/3eec415c1d1637` (3.7 G) deleted.

### 2026-11-17 — E4 on `925125a7` (done) and GS's close

All runs had STATUS 0, `git status --porcelain --untracked-files=no` empty before and after, and summaries
in `<work>/GS/e4/summary-2`:
- **Item 1**, `--commit` with the caches emptied (`item1/run-2`): 368 s, `147 checks · 147 ✅`, only
  `--commit`'s skips.
  - MSRV: the `ci.yml` check 0 s; `+1.88.0` 15 s and 6 s; `+1.95.0` 1, 0, 1, 0 s (warm).
  - The 1.88.0 folder: `628345967	…/cargo-build/fe/88b35d036b2878/msrv-1.88` (the trap's `du -sb`).
  - The 34 strict steps took **15 s in all** in this run: the most the strict cache can save on a warm
    build folder.
  - This run is GS's last gate and LN's structural gate (decision 3).
- **Item 2**, `--commit` again (`item2/run-2`): 337 s, `113 ✅ · 34 unchanged`; the unchanged list is
  exactly 34 names, each timed in item 1.
- **Item 3**: 2026-11-16, above (nextest about 18 % faster in total, counts equal).
- **Item 4**, `--quick` (`item4/run-2`): **62 s**, `82 ✅ · 65 unchanged` (the 31 under `--quick`, the two
  `+1.88.0` among them, "Left to --commit", and the 34 strict); the four `+1.95.0` checks run, 0 to 1 s.
  This is the floor.
- **Item 5** (`item5/run-2`): the probe name was absent first (`git grep` exit 1, registry `grep` exit 1).
  - (A) per-session variables changed: 74 s, 31 unchanged;
  - (B) unset: 75 s, 31 unchanged;
  - (C) `GS_ITEM5_PROBE=1`: 340 s, 0 unchanged (the 31 and the 34 all ran and passed);
  - the cache was put back, its hash equal to H1.
- The earlier item 1 on `30a53e29` (942 s) was the first run after other builds, mostly compiling.
- Cosmetic, for GS's report: the cache label reads "with uncommitted changes" whenever untracked documents
  exist (always, in `v071`).

**GS closed.** GS's last commit is `4e03057bcebb3acaadb3cab7de37b4f5ce58e532` (fix-1); the branch head is the
LN merge `925125a7366f519f976f9dc678b2170f742ccb87`. The E.4 item 1 run is `<work>/GS/e4/item1/run-2`;
review-hard closed (two passes and the re-read; the follow-up was read by the orchestrator, decision 2).
Open for later units:
- N5's spec-text corrections (A.2, C.4, D.2's usage) go into the H/O documents unit;
- the `CARGO_ENCODED_RUSTFLAGS` / `CARGO_ENCODED_RUSTDOCFLAGS` question on 0.7.0's cross-target and doc steps
  goes into `docs/todo.md`, in the same unit.
