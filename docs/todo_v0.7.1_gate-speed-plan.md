# v0.7.1: the gate's speed (`--quick`, nextest, the step cache) and CI's MSRV checks — implementation plan

- Specification: [`todo_v0.7.1_gate-speed-spec.md`](todo_v0.7.1_gate-speed-spec.md)
  (Parts M, A to F, *Under another outcome*, *Open points*). Rationale:
  [`todo_v0.7.1_batch-speed-rationale.md`](todo_v0.7.1_batch-speed-rationale.md), S1
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
- Written: 2026-11-15, against `v0.7.1-rc2` at `0fa28027`. Status: **settled
  2026-11-15.** The maintainer answered the specification's open points and decision 4
  (O1a no, O1b no, O2 (e), O3's keyed option, decision 4 (y)) and approved the 0.7.1
  plans as written: "When the plan that goes in the 0.7.1 version is ready, without
  errors, start implementing it." (*The maintainer's part*).
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
