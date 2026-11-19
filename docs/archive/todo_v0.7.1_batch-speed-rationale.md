# The batch of 2026-11-15, faster with every check kept — rationale

- Status: **approved by the maintainer on 2026-11-15** (decisions S1 to S5, as proposed
  in the session of that day: "I agree your proposals"); **S3 amended and S6 decided by the
  maintainer on 2026-11-15** (two answers in the same session); **S2's amendment, the
  Appendix's rules 1 to 26 and their three re-readings (rules 7, 9 and 20) approved by the
  maintainer on 2026-11-15**, who also answered the gate-speed points and chose D10 (a). The
  approval was given while this round of fixes was being made; the later refinements are the
  orchestrator's and were reported to the maintainer: rule 2's list of locked runs, rule 12's
  absent author, S6's fourth gate, S3's v0.7.2 paragraph and rule 21's MG additions.
- Written: 2026-11-15, against `v0.7.1-rc2` at `0fa28027`. Every `file:line` is that
  commit's unless it names another.
- Applies to: every rationale, specification and plan of the batch: the GPUI Kit 0.9.1
  completion and KDE step 1 (v0.7.1), issue #4's open KDE items steps 2 to 7
  (`docs/issues/0004_kde-open-items-rationale.md`, D10), gpui-component's own icons
  and the caret's blink.
- Archived on 2026-11-19 by 0.7.2's close-out (rule 22 (a)): 0.7.2 is the batch's last
  release (0.7.1 released 2026-11-17; 0.7.2 holds KDE steps 2 to 7, gpui-component's own
  icons and the caret's blink).

## 1. Why

The batch's plans, as first written, gate every unit with the whole per-commit subset
(`scripts/check_release.sh --commit`, which runs every lint, check, test, doc and
widget-coverage gate of the workspace, `scripts/check_release.sh:52-59`), release each
KDE step on its own, start each release only from its predecessor's tag, run the
icons and caret work only after the KDE steps, and run each release's closing steps one
after another. Measured on this machine: `--commit` 5 to 12 minutes a commit, 29 minutes
cold (before the 16-job cap; under it not measured); the full run about 23 minutes; the
asset script 30 to 37 minutes, 4 to 6 of them on the maintainer's desktop; a CI run 21 to
27 minutes. Nine releases, each with its closing steps and the maintainer's merge, tag and
publish, and roughly eighty `--commit` runs (counted from the plans' units; an estimate of
their time, not a measurement) made the wall-clock time of the batch far longer than its
work.

Two of the maintainer's rules already said otherwise and were not applied:

- 2026-10-15: `--commit` once on a task's last commit, not before every commit inside it;
  intermediate commits inside a task are not each proven green (cost accepted).
- 2026-10-31 and 2026-11-06: a light gate per task, `--commit` every third task and at a
  milestone's close, the full run before a release; during development only the touched
  tests run, and each test once.

The light gate exists only on the parked 0.8.0 branch (`v0.8.0-rc1`), with two other
speed-ups built and reviewed there: `--quick` (`350ee147`, 2026-10-31), `cargo nextest`
for the test steps of `--commit` and `--quick` (`24711322`, 2026-11-07; nextest 0.9.138 is
installed here) and a step cache that skips a step only when every declared input, tool
version, environment variable and the command are unchanged since the step last passed
(`818f0eab`, 2026-11-07, its self-test in every mode; `scripts/step_cache.sh` on that
branch). The full run uses neither nextest nor the cache, on that branch as here.

## 2. Decisions

### S1 — The gate tooling of 0.8.0 on 0.7.x, and the maintainer's cadence

- **The port.** v0.7.1's first unit, before any other unit of v0.7.1 runs a gate, ports to
  `scripts/check_release.sh` on 0.7.x: `--quick`, nextest in `--commit` and `--quick`, and
  the step cache with its self-test (`scripts/step_cache.sh`), from `v0.8.0-rc1`. What is
  0.8.0's own (its data checks, the PLAN_EXCLUDED steps, the ledger checks, the facts gate's
  shared parts of `80b722b8` and `a0ea8218`) is not ported. Its specification
  (`docs/archive/todo_v0.7.1_gate-speed-spec.md`) decides what `--quick` runs on 0.7.x, where most
  units change a connector rather than the core: at least formatting, the panic scan,
  clippy with `-D warnings` and the strict panic lints over the workspace, the tests and
  doctests of every crate the unit changes and of every crate that depends on one it
  changes, and the documentation checks; never fewer checks than a task's own targeted
  commands. The full run is not changed.
- **The cadence**, in every plan of the batch:
  - during a unit, the agent runs only the touched tests (targeted commands, as the plans
    have them);
  - at a unit's end, `--quick`, by the orchestrator, as a long run until its time is
    measured (the first timed into the record);
  - `--commit` every third code unit of a release, counted from the last `--commit`, on
    the last code unit of a release (before its documents, sweep and version units),
    after a lock or toolchain change (the warm-up gate), and wherever a plan names a unit
    structural; a documents-only unit runs the five documentation checkers and `--quick`;
  - the full run where the plans have it (the close-out), unchanged.
- **The cost, accepted by the maintainer on 2026-10-15:** a commit between two `--commit`
  runs is proven by `--quick` only. A failure the next `--commit` finds is fixed by a new
  commit on the branch, never by rewriting a pushed one, and the record names the commit
  that brought it. A specification's sentence that every unit passes `--commit` at its end
  is replaced by this cadence.

### S2 — The next release starts from the previous one's gated head

A release's branch is made from its predecessor's branch head as soon as the predecessor's
last code unit has passed `--commit` and its reviews are closed, not from the
predecessor's tag. Its units run while the predecessor's close-out, capture window and the
maintainer's merge, tag and publish are pending. When the predecessor's tag exists, the
branch is rebased onto it (`git rebase --onto v0.7.<N> <the predecessor's head the branch
was made from> <branch>`), conflicts resolved by the orchestrator and named in the record,
and the warm-up gate (`--commit` on the clean tree) runs before the next unit. Nothing of
the later release is pushed before that rebase, and its close-out (version, compatibility
run, full check, asset run) starts only after it, so its compatibility and semver steps
compare with the released predecessor. A fix the predecessor gets after the branch was made
reaches the later branch through that rebase. With two lanes (S3), "the predecessor"
is two releases: the branch starts from the gated head of the release before it in its
own lane (the KDE lane: v0.7.1 → steps 2 and 3 → steps 4 and 5 → steps 6 and 7; the
second lane: v0.7.1 → the widget icons → the caret), and, when it is cut, it is rebased
onto the tag of the release immediately before it in the maintainer's release order,
which may be the other lane's (`git rebase --onto <prev-tag> <lane-pred-head> <branch>`;
that tag contains the lane predecessor's release, so the replay is the branch's own
commits).

**Amended 2026-11-15, the mechanism only (by the orchestrator, from the plans' re-checks;
approved by the maintainer on 2026-11-15, see the Status line):** the branch is *merged with* `<prev-tag>`, not
rebased onto it: `git -c merge.conflictStyle=diff3 merge --no-ff --no-commit -m "Merge <prev-tag> into v0.7.N-rc1" <prev-tag>` once (the merge commit made by
rule 21 (1)), after the release's last unit before the merge (rule 17; V comes after it) and its reviews
are closed and the tag exists, before the
release's recount (X), version and close-out. Why: a rebase gives every commit a new sha, so the measurements' and the
records' shas become false or unreachable, the docs branches' merge commits are flattened
or dropped, commit subjects naming other commits go stale, and each plan needed a
range-diff and a sha re-pointing commit; a merge keeps every sha, as this repository's
branches already merge `origin/main` and their docs branches. Conflicts are resolved by
the orchestrator in the merge commit, named in the record, and a resolution in a code file
is reviewed; the warm-up gate (`--commit`) follows; X
then recounts every stated count in the files either side changed. Everything else in S2 holds
(the lanes as S3's amendment sets them).

### S3 — A second lane for the icons and the caret

The widget icons and the caret's blink are implemented in a lane of their own, in their own
worktree, beside the KDE lane; the order in which releases are cut stays the maintainer's,
and each release, when it is cut, is rebased onto the latest tag (S2; merged with it instead,
by S2's amendment). The machine's limits
hold for both lanes together: at most two cargo processes (one gate, and one `nice 19`
targeted build in the other lane), the gates serialized by the shared `slot.sh` locks, at
most four agents (six where two run no cargo), one asset run at a time, and never two
Fable agents. Files both lanes edit (`CHANGELOG.md`, `ROADMAP.md`, `docs/todo.md`, the
showcases) meet at the rebase (the merge, by S2's amendment), by quoted text, as the plans
already edit them.

**Amended by the maintainer 2026-11-15:** only the widget icons run in the second lane. The
caret's blink moves into the KDE lane, which becomes v0.7.1 → KDE steps 2 and 3 → steps 4
and 5 → the caret → steps 6 and 7; each of these branches from its lane predecessor's gated
head, so S2 is kept (its second lane reads v0.7.1 → the widget icons). The KDE lane alternates
two worktrees: steps 2 and 3 in `kde-b`, steps 4 and 5 in `v071`, the caret in `kde-b`, steps 6
and 7 in `v071`; the widget icons use `lane2-a`, and `lane2-b` is no longer used. The proposed
release order is unchanged: 0.7.1, 0.7.2 egui 0.37, 0.7.3 KDE steps 2 and 3, 0.7.4 KDE steps 4
and 5, 0.7.5 the widget icons, 0.7.6 the caret, 0.7.7 KDE steps 6 and 7. Why: the caret and
KDE steps 4 to 7 both re-pointed the same facts citations, so each merge had to reconcile up to
about 1,000 citation conflicts.

**v0.7.2 (egui 0.37)** is outside the two lanes. Its rationale, specification and plan are not
written yet; they follow the maintainer's work order and need the maintainer's approval. Its plan
names its worktree (`lane2-b` is free); it is bound to rules 2, 8 and 24 and counted in the agent
limits. It branches from v0.7.1's gated head or from the `v0.7.1` tag, and its plan must keep it
so: rule 18's ancestry check at the MG of steps 2–3 needs v0.7.1's gated head to be an ancestor
of the `v0.7.2` tag. The KDE lane's steps 2–3 release waits at its MG for that tag (rule 17).
The option not taken: egui 0.37's gated head as every later release's common base. It was not
taken because
the upgrade has no rationale yet, so waiting for it would delay every unit, while VF at each MG
(rule 25) is a small cost.

### S4 — The full check beside CI

In each close-out, the release branch is pushed once the compatibility run has passed, and
the full `scripts/check_release.sh` runs while CI runs on it, not before the push. The
asset run starts only when both are green, as before. The cost: a CI run can be red, and
email the maintainer, for a fault the full check would have shown first; `--commit` has by
then checked formatting, clippy and the tests.

### S5 — Three KDE releases instead of six

Steps 2 and 3, steps 4 and 5, and steps 6 and 7 are each one release (the rationale's D10:
"The maintainer may merge or reorder steps"): one branch, one version, one CHANGELOG
section, one ROADMAP section, one close-out and one asset run per pair. Each step keeps
its own units, trial, reviews and record inside the release; a pair's documents state what
both steps bring. Under D0 (a), a pair names its break once. The reply to the reporter
waits for the release of steps 4 and 5, as it waited for step 5.

### S6 — No line numbers in new citations; old ones left as they are

Decided by the maintainer, 2026-11-15 (first "convert every citation to file + key", then, the same
day, narrowed: "The new citations just point the document, not line numbers. The old ones, you can
leave as they are … Yes the line numbers will shift, but who cares? If we need it later, we can
search it.").

- **The decision.** A citation the batch writes names the document only (where useful, with a
  quoted phrase to search for), never a line number of a tracked file that changes. The existing
  line-number citations are left as they are and may drift; nothing re-points them, apart from
  the lines a gate reads by number (the fourth gate below; rule 25's version case). Line numbers
  of pinned sources (an upstream crate at a version, a URL pinned to a commit, a citation "at
  <sha>") are fine, since they never drift.
- **The one change this needs.** Three gates compare a citation's line number with the line it
  names and would turn red on every row added above it: `native-theme/tests/platform_facts_xref.rs`
  (`line(N).contains(…)`), `native-theme/src/presets/documented_sizes.rs` (`facts_line(n)`) and
  `scripts/check_preset_citations.py`. A small unit at the start of v0.7.1 makes them stop using
  line positions: the two Rust checks test that `docs/platform-facts.md` contains the text, the
  preset checker drops its line-number comparison. Nothing else is converted.
- **The fourth gate, kept.** `connectors/native-theme-gpui/src/showcase.rs`'s
  `every_colour_claim_is_read_at_the_line_it_cites` reads each colour claim's cited line too, and
  stays as it is. 243 of its citations name the gpui connector's own files at `0fa28027` (212 in
  the showcase's `info/` files, 31 in its `tests.rs`), counted with
  `git grep -ohE '"native-theme-gpui/[A-Za-z0-9_./-]+\.rs:[0-9]+(-[0-9]+)?"' 0fa28027 -- connectors/native-theme-gpui/examples/ | wc -l`.
  A unit that edits a file with cited lines either keeps the line count above each cited line or
  re-points those claims in the same commit. That, and rule 25's version case, is the only
  re-pointing of repository-file line citations the batch does.
- **The effect on the batch.** Every sweep unit, and all merge machinery for re-pointing
  citations, is deleted from the plans. MG resolves no citation number but a gate-read one: a
  conflict that holds a citation is resolved by both sides' texts like any other text (rule 21 (1)). What remains of X
  is the recount of stated counts in the files either side changed and the five documentation
  checkers (rule 17). Where S1's cadence names a sweep unit, there is none; where S2's amendment
  has X recount, that is all X does.

## 3. Not changed

Every check, test, review, trial and stop rule of the plans; the full run before every
release; the asset run of every release; the maintainer's decisions (merges, tags,
publishes, releases, the reply, the upstream requests, D0's version); the review rules of
the user's instructions.

## Appendix — the batch's working rules

Written by the orchestrating session on 2026-11-15 from the plans' reviews, and approved by the
maintainer on 2026-11-15 with S2's amendment (the Status line names the refinements made after
that approval, reported to the maintainer). The plans cite them as "rule N" (or "batch rule N"); the
numbers are kept as first given, so 13 and the rebase parts of 7 and 12 are superseded and say
so, and so are 23, 21 (2) and 21 (1) item 7, whose only purpose S6 removed; the other rules
keep only what is left once no citation is re-pointed apart from the gate-read lines (S6's fourth
gate, rule 25)
and the caret is in the KDE lane (S3's amendment). Measured on this machine (§1): `--commit` 5
to 12 minutes a commit, 29 minutes cold (before the 16-job cap; not measured under it); the full
run about 23 minutes; the asset run 30 to 37 minutes. The Bash tool's foreground timeout is 2
minutes by default and at most 10 minutes (600000 ms).

The labels the rules use, as the plans define them:

- P1: a plan's tools task (the lock and record folders, its copies of `slot.sh` and
  `lowprio.sh`); it runs before the branch exists.
- P2: a plan's branch-and-worktree task: the branch made from `<lane-pred-head>` (or from
  `<prev-tag>`) in the core worktree, with its start-of-work checks.
- P0: a plan's unit 0, the documents: it adds the plan's documents still untracked and sets
  their status lines, and runs only where one is left (rule 26).
- MG: the merge of `<prev-tag>` into the release's branch, once (rules 17, 21).
- VF: the version forms after MG (rule 25), inside MG's step (3).
- R-MG: the `review` (or `review-hard`) of MG's resolutions and of the commits of its steps (3),
  (6) and (7) (rule 21, step (4)).
- X: the release's check after MG, on the merged tree: the recount of stated counts in the
  files either side changed and the five documentation checkers (rule 17); with no MG there is
  no X.
- V: the version unit (the workspace version, the inter-crate requirements, `Cargo.lock`, the
  CHANGELOG date; rule 14).
- K5: the close-out step of the steps 4–5 and steps 6–7 plans that builds the three showcases
  for release and runs the parity step's layout rehearsal on a private X server (rule 19);
  v0.7.1's K5 is not meant.
- The record: a plan's running record (`<record>/record.md` in the plan's record folder), which
  its close-out copies into the plan's Record section.
- The pinned documents: those whose lines are dated to a commit, as each plan lists them
  (`docs/archive/`, `docs/evidence/`, `docs/issues/`, the dated `docs/todo_v0.7.1_*` and
  `docs/todo_v0.7.x_*` documents, the gap maps and spec traces named `todo_v0.7.1_*`,
  `docs/todo.md`'s dated block); their line citations stay (S6); their path mentions follow
  rule 22 (a).
- The GPUI Fast legs: the two gpui connector test legs `--commit` skips
  (`scripts/check_release.sh:710-716`, `:849-855` at `0fa28027`), run by the orchestrator in a
  gate where a rule names them. They are part of that gate, so each starts under the gate's lock
  (`flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh …`, rule 2), as every long run of
  the orchestrator's that runs cargo does, a trial's included.
- `<lane-pred-head>`: the gated head of the release before this one in its own lane (rule 20),
  which the branch is made from.
- `<prev-tag>`: the tag of the release immediately before this one in the maintainer's order,
  which MG merges.
- `<base>`: `<prev-tag>` where the branch was made from it, else `<lane-pred-head>`.
- `<merge>`: MG's merge commit; with no MG, `<merge>` and `<merge>^1` read as HEAD.
- `<core worktree>`, `<record>`, `<work>`, `<tools>`, `<locks>`, `<run folder>`, approval copy,
  late fallback: as each plan defines them.

1. An agent runs its task's targeted commands (the named tests, `cargo fmt --all --check`, the
   checkers, a `-p` clippy or build) in the foreground, passing the maximum timeout (600000 ms)
   for a long one, each expected to finish well within 10 minutes, stages its change and
   reports; it runs no `--quick`, `--commit`, full check or asset run.
2. The orchestrator runs each gate on the staged change as a long run (`run_in_background`, from
   the stated worktree, the log and a `STATUS` file at absolute paths in the plan's work folder,
   through the wrappers), waits for the exit notice and reads `STATUS` and the log's tail;
   `git status --porcelain --untracked-files=no` prints the same before and after. A red gate
   goes back to the task's author with the log's failing part. The orchestrator commits the gated
   tree with the author's message. Every heavy cargo run takes `<locks>/gate.lock`, so no two run
   at once: every long run of the orchestrator's that runs cargo. That is every `check_release.sh`
   mode, every other command run as part of a gate (the GPUI Fast legs, a CI leg run beside it),
   the feature-set builds after a warm-up gate, MG's regenerations, a trial's run, a
   measurement's builds and blocks, the compatibility run, K5's release builds and the asset
   script. A gate starts as
   `flock <locks>/gate.lock <tools>/slot.sh <tools>/lowprio.sh scripts/check_release.sh …`, the
   asset run as rule 24 says: `slot.sh` takes either of its two slots, so without this lock two
   heavy runs could run at once. S3's "the gates serialized" holds through this lock. Each agent's
   targeted `-p` commands take only `slot.sh` and `lowprio.sh`.
3. The first gate run of a branch is timed into the record.
4. A wait on a condition inside a long run uses the Monitor tool with an until-loop on a file
   (`until grep -q '<text>' <log> || [ -s <STATUS> ]; do sleep 10; done`), never a `pgrep -f` loop.
5. Any other run not known to finish within 10 minutes (the compatibility run, a first release
   build in a new worktree, a measurement block) is a long run by the orchestrator.
6. The cadence (S1): `--quick` at each unit's end; `--commit` every third code unit counted from
   the last `--commit`, on a release's last code unit before its documents and version units
   (S1 names a sweep unit there too, which S6 removes), after a lock or toolchain change (the
   warm-up gates, and V, whose `Cargo.lock` lists the workspace crates' new versions), and at
   units a plan names structural (rule 24). A documents unit between that last code unit and V
   takes `--quick`, rule 20's gated head included; documents-only units: the five documentation
   checkers and `--quick`; the full run in the close-out. A failure the next
   `--commit` finds is fixed by a new commit.
7. S2 (as amended): the branch starts from the lane predecessor's gated head (rules 18, 20) and
   is merged with `<prev-tag>` once (rule 17), or is made from `<prev-tag>` where that tag exists
   when P2 runs (no MG), since that equals the head plus a merge with nothing to resolve (a
   re-reading of S2's "not from the predecessor's tag", put to the maintainer with the plans).
   (The first form, a rebase, is superseded.)
8. S3 (as amended): the second lane holds the widget icons only, the caret is in the KDE lane;
   two lanes share the machine's limits: two cargo processes (one heavy run, one `nice 19`
   targeted build), the heavy runs serialized (rule 2's `gate.lock`), four agents (six where
   two run no cargo), one Fable agent; and rule 24. Enforced by the locks: one heavy run at a
   time (`gate.lock`), at most two cargo commands among those run through `slot.sh` (its two
   slots), `nice 19` through `lowprio.sh`, and rule 24's one-at-a-time runs. The agent counts and
   the one Fable agent are not enforced by anything; the orchestrator keeps them.
9. S4: in a close-out, push after the compatibility run passes; the full check runs while CI runs;
   the asset run starts when both are green: CI green, and the full check's only failure is the
   provenance stamp, which only the asset run refreshes (`check_release.sh:1309-1311` makes a
   stale stamp fatal once V dated the CHANGELOG) (a reading of S4's "both are green", put to the
   maintainer with the plans). "CI green" needs an open pull request (`ci.yml` runs on pushes to
   `main` and on `pull_request`), so a plan opens its draft pull request at its first push;
   opening a draft pull request is on the maintainer's approval list.
10. S5: KDE steps 2+3, 4+5 and 6+7 are one release each.
11. Code units: a unit stays a documents unit only when it changes nothing but comments, rustdoc
    prose, READMEs and Markdown; `docs/platform-facts.md` counts as code (tests parse it); a
    documents unit's `--quick` compiles the READMEs it changes (as doctests). Any
    `--commit` (a warm-up gate, MG's warm-up, a `--commit`-gated fix) restarts the count of three; a
    `--quick`-gated fix does not count toward it. A unit counts as a gpui unit (one whose gate runs
    the GPUI Fast legs) where its diff changes code or strings under `connectors/native-theme-gpui/`.
    A merge of one of the release's own lanes counts the code units it brings.
12. A red warm-up gate, wherever one runs (at P0, at P2 and after MG), goes, with the log's
    failing part, to the author of the unit whose rule the failing test checks (a full struct literal, a count, a mapping), even where the failing
    file is the other side's; where that author is no longer at hand (a lane predecessor's unit,
    at P2), to a new `implement` agent with that log and the unit's specification part; one
    caused by a conflict resolution is the orchestrator's; one whose
    cause is unclear is a stop. The fix of a red warm-up is gated by the warm-up run again. MG's
    warm-up can be the source of a plan's late fallback. (The
    first form's range-diff and sha re-pointing are superseded: a merge changes no sha.)
13. Superseded by rule 22 (no plan writes or reads a later-links file).
14. CHANGELOG: a release's documents unit writes `## [0.7.N] - <date>` with its proposed number,
    never `## [Unreleased]`; V fills the date; its section stands above every other `## [`
    heading.
15. A fix the close-out needs is gated as the task it fixes. How the asset run ended sets its
    case: **finished**, the asset script exited with `STATUS` 0; **not finished**, it exited
    non-zero or was killed. A fix found while the script is still running waits until it has
    ended. One that touches a stamped path (`scripts/update_provenance.sh`'s `SOURCE_PATHS`, the
    compatibility stamp's inputs) resumes the close-out at its first step, `cargo audit` (V stays:
    it comes before the close-out, S2, rules 17 and 22 (c)); when that happens after the asset
    commit, a new asset commit, of the assets and stamps only, is made (never an amend of a pushed
    commit). One that touches no stamped path is committed at once: found before the asset run, it
    is a commit before it; found after a finished asset run, a commit after the asset commit; found
    after an asset run that did not finish, a commit before the asset run is made again (a new run
    folder and a new capture window), the partial outputs left unstaged. After a fix in any case,
    the branch is pushed (the asset script needs HEAD equal to origin's branch): by the
    close-out's push step where the close-out resumes or has not pushed yet, else by refspec at
    once; rule 9's full check runs again, and CI must be green on that commit before the tag. A
    fix committed between the compatibility run and the asset commit leaves the compatibility run's uncommitted files unstaged too; each
    such commit's unstaged exceptions are named in the record. A fix to a close-out step's own
    staged edits is made within that step. A plan's other sections cite this rule rather than
    restating it.
16. The reply's posted state is read, not asked: `cd <core worktree> && gh issue view 4 --comments`;
    posted = a comment by the repository owner carrying the draft's opening sentence
    (`docs/issues/0004_kde-hand-overrides-rationale.md` §8); a failed read is reported and the
    reply edits wait; if posted, the draft is not edited. A plan that edits the draft reads the
    state again in MG's survived checks, or at V where there is no MG; if the reply was posted since its documents unit, a
    documents-only commit reverts that unit's draft edits and the maintainer is told.
17. MG: `cd <core worktree> && git -c merge.conflictStyle=diff3 merge --no-ff --no-commit -m "Merge <prev-tag> into v0.7.N-rc1" <prev-tag>`
    (diff3, so that every conflicted hunk carries its base section; never committing on its own,
    so the regeneration and resolutions of rule 21 (1) always come before the merge commit), once, after the release's last unit before the merge has passed its gate and every review is
    closed and `<prev-tag>` exists; before X, the version (V) and the close-out;
    never while a trial's or a unit's edits are uncommitted, never between a measurement's
    "before" and "after" runs; nothing is pushed before it. Where the branch was made from
    `<prev-tag>`, there is no MG, the plan's start-of-work checks read that tree, and the branch
    may be pushed after any gated unit. X runs on the
    merged tree (`<base>` as the glossary defines it): every stated count in every file either
    side changed (the union of `git diff --name-only <base> <merge>^1` and
    `git diff --name-only <base> <prev-tag>`) outside the pinned documents, recounted, since a
    count stated in one side's file can be made false by the other side's change; then the five
    documentation checkers. "This release's changes" are
    `git diff --name-only <base> <merge>^1` together with `git diff --name-only <merge> HEAD` (the
    commits MG made after the merge). With no MG there is no X: nothing was merged, and the
    documents unit already ran the five checkers.
18. The lane predecessor depends on the maintainer's choices still open (the release order); a
    plan names its `<lane-pred-head>` and the record holding it for the proposal and says how they
    change under each open choice, and its maintainer's section names the dependency. `<lane-pred-head>` is the lane predecessor's gated
    head (rule 20) once its gate has passed and its reviews are closed (rule 20's recorded line).
    P2 checks that the predecessor's record shows it so; before the merge, MG
    checks `git merge-base --is-ancestor <lane-pred-head> <prev-tag>` (the tag holds the lane
    predecessor's release), a stop where it fails; after the merge, the merge commit's parents.
    Where the branch was made from `<prev-tag>`, P2 checks
    `git merge-base --is-ancestor <lane-pred-head> HEAD`. A predecessor's outcome is read from its
    commit messages or record, never from a status line its close-out writes. A worktree is reused
    only once the release that last used it has its tag (checked after
    `git fetch origin tag <tag>`); otherwise the release waits for it.
19. A parity difference (a measurement's, or a comparator's after any capture: every asset run's,
    and v0.7.1's dry parity run KD) is a fault, never a question: the showcase's or connector's author fixes it (a close-out fix under rule 15, the run
    made again), or, where it is a toolkit's limit, it is excepted with its reason and source (an
    exception entry, reviewed); the maintainer is told in either case (a showcase or connector that
    ignores theme data is a bug). An element pushed out of view, where its placement is a design
    choice the maintainer made, stays a question. A difference K5's rehearsal shows before the
    capture is reported while the capture goes ahead, as the plans say. At MG, `Cargo.lock`'s versions of the crates P2 lists are compared before and after; a version
    within P2's accepted set (egui 0.36.2 or 0.37.x, GPUI Kit 0.10.x, patch releases of the others) is rule 25's
    rework, any other a stop. On both paths (the branch made from `<lane-pred-head>` or from
    `<prev-tag>`), P2 compares the tree's `Cargo.lock` with `git show 0fa28027:Cargo.lock` for the
    same crates, and a changed version's readings are made again before the first unit (or P2's
    stop). A measurement's "after" dumps are of its subject unit's last commit
    (or the release's last code commit); a later fix that touches the measured page's layout before
    MG re-runs the "after" blocks (one base), and the dumps are kept until the first push; after MG
    a layout fix is read by K5's after-only checks and the rehearsal and named to the maintainer,
    never compared with the pre-merge "before" dumps; what MG brings is not re-measured, but K5 runs the measurement's after-only checks on the rehearsal's
    merged-tree dumps, since the parity checker counts an element that is not visible as ruled out,
    not as a difference (`scripts/check_showcase_parity.py:1077-1079`), and skips a property fewer
    than two kinds give (`:1080-1081`); a kind missing from a group shows as an unexcepted
    "(dump)" row (`:1034-1037`), but an entry with no dump of any kind forms no group at all
    (`:740-757`) and goes unnoticed. The rehearsal covers every layout entry the measurement used, and the
    check verifies that its entry list holds all of them and is not empty; it requires a dump for
    every entry and kind (the plans give each check's exit codes); and it is
    relative to the measurement's "after" state, not absolute, since a page that scrolls has
    elements outside its visible area by design: before the dumps are deleted, the record keeps,
    per kind and entry, the listed ids present and the ids wholly inside at the "after" run, and the
    check fails where an id present (or inside) then is missing (or not inside) on the merged-tree
    dump; an entry the measurement did not use has its visible-area findings reported, not stopped on (its other checks hold as for the rest). It includes the
    measurement's after-only parity checks, and K5 requires the rehearsal's summary to hold no
    failed-dump line.
20. The gated head is the documents unit, the last unit before MG (with no MG, before V), after
    every review is closed. It takes its cadence's gate (`--quick`, S1); there is no extra
    `--commit`: the last code unit's `--commit` (rule 6, S1's own clause) covers the code beneath
    it. The record holds "the lane's gated head for the next release: <sha> (<unit>; --quick run
    <folder>; reviews closed)", naming that unit's commit (a fix after a red gate included). That
    commit is the next release's `<lane-pred-head>`, since the next release reads the documents
    unit's texts, so they must be in its base; the next release never waits for this release's MG,
    X or tag. A lane's last release (no next release reads its line) has no gated head. (Making
    `<lane-pred-head>` the documents unit rather than the last code unit is a re-reading of S2's
    branch point, "as soon as the predecessor's last code unit has passed `--commit` and its
    reviews are closed", put to the maintainer with the plans.)
21. MG's order and mechanics. The steps are written in run order, (1), (3), (6), (7), (4), then
    (5), keeping the numbers the plans cite ((2) is superseded by S6): R-MG reviews whatever (6)
    and (7) committed too, and the warm-up gate is the last.

    (1) The merge, in this order:

    1. The tag: fetched only where `git rev-parse -q --verify <prev-tag>` fails, and in any case
       the first field of `git ls-remote origin 'refs/tags/<prev-tag>'` compared with
       `git rev-parse <prev-tag>` (the tag ref's own object on both sides, equal for a lightweight
       tag, as this project's release tags from v0.5.9 on are, and for an annotated one, as some
       older ones are; tags are shared by every worktree, and a release tag has been moved
       before); no line printed, or a different sha, is a stop.
    2. Rule 18's ancestry check.
    3. The untracked-overwrite check, in the core worktree: the paths both
       `git ls-files --others --exclude-standard` and
       `git diff --name-only --no-renames --diff-filter=A <lane-pred-head> <prev-tag>` list, each
       compared with `git show <prev-tag>:<path> | cmp - <path>`: removed where equal, a stop
       otherwise.
    4. The merge, with the HEAD before it recorded: rule 17's command,
       `git -c merge.conflictStyle=diff3 merge --no-ff --no-commit -m "Merge <prev-tag> into v0.7.N-rc1" <prev-tag>`.
    5. `Cargo.lock` and the version comparison. A conflicted manifest (`Cargo.toml`) is resolved
       first (by item 6's rules), before this item's cargo commands. Where the merge left
       `Cargo.lock` conflicted, `<prev-tag>`'s side is taken (`git checkout --theirs Cargo.lock`) and this release's lock
       changes are re-applied, one
       `cargo update -p <name>@<theirs-version> --precise <ours-version>` (which resolves without
       building) per package this release changed (a package this release added or removed
       has no `<theirs-version>`: `cargo metadata --format-version 1 > /dev/null` first resolves
       the missing or surplus entry without building or upgrading others, and the diff against
       `<prev-tag>`'s lock afterwards must show only this release's packages, else a stop), the
       commands derived from `git diff <base> <the HEAD recorded at item 4> -- Cargo.lock`, so no
       unit records anything for it. Then the version comparison (rule 19's read of
       `Cargo.lock`, which needs no gate), here so that a stop wastes no build and its result
       tells whether the merged tree can build before VF. Last,
       `<tools>/slot.sh <tools>/lowprio.sh cargo metadata --locked --format-version 1 > /dev/null`
       passes: it builds nothing, and it catches a lock that merged cleanly but is inconsistent.
    6. The resolution of the conflicts (below).
    7. Superseded by S6: there is no second pass.
    8. Generated files regenerated by their commands (always a long run, since at every MG
       `<prev-tag>`'s V has bumped the workspace crates in `Cargo.lock`; where the merged tree
       does not build before VF, the generated file is taken from the HEAD recorded at item 4, recorded as pending, and regenerated in VF's code
       commit, or, where VF makes no commit, by the orchestrator in a commit of its own before
       the warm-up gate, or, where the tree builds only after a warm-up fix, in that fix's commit
       before the warm-up runs again).
    9. `git add` of the resolved and regenerated files, conflicted or clean.
    10. The marker check,
        `git diff --cached -U0 | grep -E '^\+(<{7}|\|{7}|={7}|>{7})( |$)'` (diff3's `|||||||`
        included), prints nothing; a marker-like line that `<prev-tag>`'s own file has at that
        place is not counted. Then `cargo fmt --all --check` passes before the merge commit (on a
        failure, `cargo fmt --all` runs, the files it changed are re-staged and named in the
        record, and the check runs again; a file it changes outside the staged set is a stop).
    11. The staged-path check: every path of `git diff --cached --name-only --no-renames` is in
        `git diff --name-only --no-renames <lane-pred-head> <prev-tag>` or is a regenerated file
        the record names; any other path is a stop, named in the record.
    12. The commit, `git commit --no-edit --cleanup=strip`; `<merge>` recorded.
    13. The parents check: `git log -1 --format=%P <merge>` prints the HEAD recorded at item 4
        and `git rev-parse <prev-tag>^{commit}`, in that order; then
        `grep -m1 '^## \[' CHANGELOG.md` prints this release's heading and
        `git status --porcelain --untracked-files=no` is clean.

    (The tag check of item 1 also runs at P2 where the branch is made from `<prev-tag>`.)

    **The resolution** (item 6) is the orchestrator's. It leaves out `Cargo.lock` (item 5) and
    the generated files (item 8), and never runs `git checkout --conflict=…` on a file it has
    touched (that would put the markers back over its resolution). It resolves every conflicted
    hunk by both sides' quoted texts (S3), a citation like any other text (S6: an old line number
    in a conflict takes either side's text, both drift alike, and a new citation has none; where
    one side re-pointed a line a gate reads by number (S6's fourth gate, rule 25), that side wins;
    where both sides re-pointed the same one, the number is the base plus both sides' deltas,
    confirmed by the warm-up gate), and names each
    resolved hunk in the record; a stop where a resolution changes a text's meaning. `CHANGELOG.md` takes this
    release's section first and `ROADMAP.md` takes it after `<prev-tag>`'s. Counts:

    - A count or verdict tuple a test or gate checks: the base plus both sides' deltas.
    - A prose count: `<prev-tag>`'s side, named in the record (X recounts it).

    (3) Rule 25's rework (VF) (a source the new version needs that the registry lacks is fetched
    by the orchestrator with `cargo fetch`, which builds nothing); its review is within R-MG.

    (6) The path check (rule 22 (b)).

    (7) The plan's survived-the-merge checks, comparing this release's blocks on the merged tree
    with their state at `<merge>^1` (not with an earlier unit's script, whose blocks a later unit
    may have changed; a prose count resolved to `<prev-tag>`'s side in (1) is listed for X's
    recount, not compared), and the anchors of the later tasks (V's heading, every status line
    the close-out sets, the archive row) found on the merged tree: the texts this release
    replaced absent, the remaining anchors and the release's new texts present, the
    struct-literal and count searches run again. The checks read HEAD as it stands at (7), after
    (3) and (6). The blocks VF rewrote in (3) are expected differences from `<merge>^1`: the record
    lists them from VF's commits, and each is compared with the form the specification gives for
    the new version. A survived check never undoes VF.

    (4) R-MG where a file was resolved in (1) or written by (7), except Markdown that no test
    parses (so `.rs`, manifests, workflows, configuration files, a TOML a test reads, a script,
    `docs/platform-facts.md`, a README, which is compiled as doctests and parsed by tests, and
    `connectors/native-theme-egui/src/mapping.md`, which `mapping_doc.rs` reads, are in), where (1)
    resolved `Cargo.lock`, or where VF committed anything: a `review` (a `review-hard` where they
    change a hard part of a core: a resolver, validation, a runtime, concurrency, `unsafe`) of
    `git show --remerge-diff <merge>` and of every commit of steps (3), (6) and (7), with the
    record's list of resolved hunks; a must-fix is a new commit by the author of the commit it
    fixes.

    (5) The warm-up gate (`--commit`), last, covering every commit of steps (1), (3), (4), (6)
    and (7) (VF's, an R-MG must-fix, a path re-point, a fix the survived checks need), which have
    no gate of their own. It always runs the GPUI Fast legs too, since the merge brings the other
    side's code (and `<prev-tag>`'s version bump). Each fix after the warm-up has passed is a
    commit with its own gate (a fix of a red warm-up is gated by the warm-up run again, rule 12).

    Agents in the steps before (5) run no cargo but `cargo fmt --all --check`, which builds
    nothing (a regeneration is the orchestrator's long run); the one exception is VF's
    `implement` agent, which may run one `nice 19` `-p` build or clippy of the crate it rewrote,
    through `slot.sh`. "The release's commit range" is `git log <prev-tag>..HEAD`: it equals
    `<base>..HEAD ^<prev-tag>`, since `<base>` is an ancestor of `<prev-tag>` (rule 18), so it
    keeps the commits the release's own docs lanes merged in and leaves out what `<prev-tag>`
    brought; with no MG, where `<base>` is `<prev-tag>`, it is the same range.
22. (a) The close-out of the batch's last release in the maintainer's order also archives this
    rationale (`docs/todo_v0.7.1_batch-speed-rationale.md`), by the same rules; every release plan
    of the batch names that task as conditional on being last. A close-out that moves documents to `docs/archive/` rewrites every path mention of them in
    every tracked file outside `docs/archive/` (in a document that dates its lines, a dated record
    such as `docs/todo.md`'s dated block, a dated line citation keeps its path and number as
    written, and that document's other path mentions are rewritten; each kept mention is named in
    the commit body, so that the per-match check's output is explained), found with `cd <core worktree> && git grep -nE '<pattern>' -- ':!docs/archive'`
    after the move; a mention that already points into `docs/archive/` in any relative form or as
    an absolute URL is left, and so are a citation dated "at <sha>" and a path inside another
    document's instructions that names its own future target, and a path of a plan's record or work
    folder (`<scratch-keep>/<plan stem>/…`, `.superpowers/sdd/<plan stem>/…`), which does not move (each
    named in the commit body). Its check filters per match, not per line:
    `git grep -noE '(archive/)?<pattern>' -- ':!docs/archive' | grep -v ':archive/'` prints
    nothing but the mentions left by the exemptions above, each named in the commit body. The
    moved documents' own relative links are re-checked after the move, with rule 22 (b)'s
    exemptions (placeholders, quoted texts of other files, URLs) (they now
    resolve from `docs/archive/`). (b) A path check runs at MG's step (6), before its warm-up gate (and at P0 only
    where the branch was made from `<prev-tag>`): every token starting `docs/` in the release's
    specification and plan (its `:N` suffix and `#…` stripped) checked with
    `cd <core worktree> && git ls-files --error-unmatch <path>`, and every relative `](…)` link
    of those two documents (they live in `docs/`) checked from `docs/`
    (`cd <core worktree>/docs && git ls-files --error-unmatch <link>`), brace sets expanded;
    exempt are paths cited "on <branch>" or "at <sha>", paths a unit of this release creates,
    placeholders, glob or regex patterns, URLs, `crate::` links and quoted texts of other files; one that moved to
    `docs/archive/` is re-pointed by a `worker` in a documents-only commit; one gone otherwise is a
    stop. Where there is neither an MG nor a P0, P2 runs it; a re-point made before a warm-up gate
    is covered by that gate, and one P0 finds goes into P0's own commit. (c) Code, tests and rustdoc that name
    one of the release's own planning documents name its archived path (`docs/archive/…`), since
    the close-out moves it after the asset run; before the close-out's first step, V runs, from a
    fenced block (never a table cell), `cd <core worktree> && git grep -noE
    '(archive/)?<document stem>[a-z{},.-]*' -- <the stamped paths: scripts/update_provenance.sh's
    SOURCE_PATHS and scripts/update_compatibility.sh's source paths of every connector>
    | grep -v ':archive/'` (the stem matching the brace form `{spec,plan}` too, filtered per
    match), which prints nothing; a hit is re-pointed then.
23. Superseded by S6: no facts citation is re-pointed at MG.
24. Across both lanes one at a time: the asset run, the capture window and the private-X rehearsal
    (`rehearsal.sh`, Xvfb display `:97`), each held by a shared lock in the `slot.sh` locks folder
    (`flock <locks>/assets.lock …` around the asset run and its capture window,
    `flock <locks>/xvfb97.lock …` around the rehearsal), so a lane waits mechanically. The asset
    lock, the gate lock (rule 2) and the cargo slot are all taken before the maintainer is asked
    for the capture window: the long run starts as
    `flock <locks>/assets.lock flock <locks>/gate.lock <tools>/slot.sh sh -c 'touch <run folder>/HELD;
    until [ -e <run folder>/GO ]; do sleep 5; done; …'; echo $? > <run folder>/OUTER.STATUS`, the
    wait for `HELD` is `until [ -e <run folder>/HELD ] || [ -s <run folder>/OUTER.STATUS ]; do sleep
    10; done` (a failure before `HELD` ends the wait), the maintainer is asked once `HELD` exists,
    and `touch <run folder>/GO` starts the capture. Worktree
    commands (`git worktree add`, `remove`) run with absolute paths. "Structural" (rule 6) means
    a unit that changes a manifest, `Cargo.lock`, a build script, the toolchain or the gate
    scripts (`scripts/check_release.sh`, and from the gate-speed port on `scripts/step_cache.sh`,
    `scripts/step_cache_includes.py`, `scripts/msrv_checks.sh` and `scripts/check_panic_patterns.py`,
    whose lexer the include scan uses (gate-speed specification, C.2 and C.4): the scripts that compute the
    gate and its keys, not every script the gate runs): these change what the gate or its cache keys are computed from, so a `--commit`
    confirms them without relying on the `--quick` key. `--quick` already runs every
    step of `--commit` for the crates a change reaches, their dependents included (gate-speed
    specification, decision 1 (c), A.4), so a new module, a new public API or new model fields are
    no reason on their own, and neither is a leg that only `--commit` was thought to build.
25. Since `<lane-pred-head>` is a pre-merge head, egui 0.37.x and GPUI Kit 0.10.x (v0.7.2) and the other lane's releases
    normally reach a branch only at MG. Each plan names, in MG's step (3), after the version
    comparison (step (1), rule 21) and before the warm-up gate, the task VF "the version forms after MG": for every
    crate whose version changed, a `worker` re-reads every upstream line the specification cites in
    the new version, an `implement` agent rewrites the forms the units took on the older version
    (code, tests, texts naming a version) and a `worker` the documents' forms, each only where the
    content (behaviour, value, API) changed in the new version, never to update a version-pinned
    line citation that still holds (S6), except a line a gate reads by number (for example the
    gpui showcase's `HOW` texts and colour claims, read in the crates `CITED_CRATES` lists, and
    the fallback-stack test's gpui-pre lines),
    which is re-pointed where the new version moved it; each is reviewed
    (within R-MG) and covered by MG's warm-up gate; "none needed" where nothing changed. A
    start-of-work reading of what "landed first" from the other lane moves to MG.
26. A plan's own documents at P1/P0: only a document still untracked in the commit the branch is
    made from is compared with its approval copy by `cmp` (equal: copied; v071's gone: the approval
    copy; different: a stop naming what changed). P1 runs before the branch exists, so it reads
    that commit, not a checkout: `git -C /home/tibi/Rust/native-theme cat-file -e
    <lane-pred-head>:docs/<name>` (`<prev-tag>` where the branch is made from it) failing means
    untracked. A document an earlier commit added (normally v0.7.1's commit (0)) is not compared:
    later close-outs rewrite its path mentions (rule 22 (a)) and set its status line; P1 records
    the commit that added it and copies nothing, and P0 only sets a status line that commit left,
    or does not run.
