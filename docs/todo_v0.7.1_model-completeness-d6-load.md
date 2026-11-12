# How a bundled preset loads: the budget, the load path, the caches (D6, outcome 1)

Design note, 2026-11-10; revision 2, 2026-11-11; revision 3, 2026-11-11; revision 4, 2026-11-11; revision 5,
2026-11-12; revision 6, 2026-11-12; revision 7, 2026-11-12; revision 8, 2026-11-12. It proposes a budget (its
lines apply as proposed until the maintainer changes them), chooses the load path, decides who keeps a resolution
and a query's result, orders the work, and lists what would defeat the design.
One sentence of it changes a rationale rule (R2.8), a class-3 change the maintainer decides; like D6's other outcomes
it is applied as proposed and open to the maintainer's veto ([O]:5-6, :30).

## Revision 8

The seventh independent check (`load-design-check7.md`, "check 7", 0 serious, 3 should-fix, 10 minor; cited c7-1 to
c7-13, its added situations A1 to A15) found the step's case set undefined, a bound that counts sweeps no task
sizes, and an accepted band whose size and owner were not stated. Every finding was verified at its evidence before
anything changed (8.7); the orchestrator decided the corrections. Where revision 7's list says otherwise (its items
1, 2 and 4), the body holds. The situations of 8.6, A1 to A15 among them, are printed by
`load-check/revision8_situations.py`.

1. **The file defines the cases** (2.4 rule 4): the step asks the probe for each armed case by name; a case asked
   for and not reported fails either run as "not reported"; the probe's own list decides nothing, so revision 7's
   item 4 goes (8.7 says what that leaves). c7-1.
2. **The release run's bound** (2.4 rule 2): it sweeps again until each time has two passes that count, starts no
   sweep later than 30 minutes after the build's end, and finishes and judges a sweep already running. Task P
   records how long a sweep takes; where three quiet sweeps take more than half the bound, the bound is raised (2.4
   *Where the numbers come from*). c7-2.
3. **The band over a met line** (2.4 *What it gives*): a met line stays exceedable within its tolerance, at most
   20 %, with no name from the step, permanently and by design; the printed mark goes into the release's ledger row,
   and a case printed so at two consecutive releases gets a named miss from the release task (*Who makes records*).
   c7-3.
4. **The runnable count is taken whole** (2.4 rule 2): the limit of 10 includes the step's own threads. c7-4.
5. **No pre-sweep mean** (2.4 rule 2): each pass is judged by its own mean, and a unit run makes its three sweeps.
   c7-5.
6. **The stale test applies only where the record is the bound**: under a named miss whose record exceeds its line
   (2.4 rule 1). c7-6.
7. **A named miss keyed by a case the file does not list, and a second named miss of one case, fail either run as
   file defects** (2.4 rule 1). c7-7.
8. Minor: load before a pass worded as what is true (c7-8); revision 5's item 4, c3-11 and the situations' row 10
   (c7-9); the situations' script (c7-10); the self-test's cases (c7-11); the record rule judged before the change
   is written (c7-12); the veto sentence (c7-13).
9. **The two items 8.7 left open, closed by the orchestrator after the check.** The probe's own list serves one
   check and no other (2.4 rule 4, replacing item 1's "the probe's own list decides nothing"): the file lists every
   case the probe can measure, as armed, as "first measured by" a named task, or as not binding, and a case the
   probe can measure that the file does not list fails either run as "unlisted case", a file defect (8.6 row 15,
   task P's *Done* and self-test). A re-record of a case under a named miss keeps the old record where the new runs'
   lowest figure lies within the old record's tolerance; only a record that changes needs a cause (2.4 rule 3 and
   *Who makes records*; 8.6 R1, R2). `revision8_situations.py` models both. c7-1, c7-12.

## Revision 7

The sixth independent check (`load-design-check6.md`, "check 6", 1 serious, 3 should-fix, 9 minor; cited c6-1 to
c6-13) found the load guard carrying two signals where one does the job, the step's own threads and the per-pass
load left unspecified, and a case the file does not list passing every run. Every finding was verified at its
evidence before anything changed (8.6); the orchestrator decided the corrections. Where revision 6's list says
otherwise, the body holds. The situations of 8.6 are printed by `load-check/revision7_situations.py`.

1. **The runnable count is the only load signal** (2.4 rule 2): the one-minute load average, its readings when the
   passes start and end, and the 120 s settle time are removed; before each sweep the step takes the count's mean
   over 2 s, the release run waiting while it is over 10, a unit run sweeping no further. Why the settle time went:
   8.6. c6-1.
2. **The step's own threads**: the count less 2, the sampling thread and the pass's thread, stated once (2.4 rule 2).
   c6-2.
3. **The probe samples the count in a thread of its own** and prints each pass's mean beside the pass's figure (2.4
   rule 2, task P's *Done*). c6-3.
4. **A case in the probe's output that the file does not list fails either run**, as a file defect (2.4 rule 4,
   task P's self-test). c6-4.
5. **A time counts only where at least two of its passes count** (2.4 rule 2); the release run sweeps again until
   two do. c6-9.
6. **The bound and the stale test as formulas** (2.4 rule 1): fail where f > bound × (1 + t), stale where
   f < record × (1 − t), every tolerance a fraction (rule 3). c6-10.
7. **A named miss whose record meets a raised line** is no error: the line is its bound, and it prints "may go" where
   its figure meets the line, as every miss does (2.4 rules 1 and 4). c6-13.
8. **A figure over a met line within its tolerance** passes as noise at that run, printed "over its line, within its
   tolerance"; *What it gives* states the band, and no rule tracks it across runs (8.6). c6-8.
9. **Records follow one rule**: a task records each case its change can move by more than its tolerance; task 2
   lines 1 to 4, task 3 lines 3, 5, 6 and 11 (it changes no mutation), task 7 lines 4 and 10, and lines 1 and 2 under
   a veto of A (2.4 *Who makes records*, section 5). c6-5.
10. Minor: c3-11 points to c5-1 (c6-6); the veto names lines 1 and 2, each missing case with its own named miss
    (c6-7); "an armed case the step ran and the probe did not report" (c6-11); task 6 says to pause other lanes for
    the release run (c6-12).

## Revision 6

The fifth independent check (`load-design-check5.md`, "check 5", 1 serious, 3 should-fix, 7 minor; cited c5-1 to
c5-11, and its list of passages where decision Q applies) found that a named miss's record stays its bound after its
cost falls, named misses keyed coarser than records, an exact count given the 5 % floor, and a load guard blind to
load that starts during the passes. Every finding was verified at its evidence before anything changed (8.5); the
orchestrator decided the corrections (Q to U). Where revision 5's list says otherwise (its items 4 and 9), the body
holds. The lag of item 5 is printed by `load-check/revision6_load.py` item 3.

1. **A stale record fails the release** (2.4 rule 1): under a case's named miss, a figure below its record by more
   than its tolerance prints "record stale: re-record <case>", which fails the release run and is only printed in a
   unit run. Each cost task records every case its change moves, listed per task in *Who makes records*: task 1
   lines 1 to 4, and tasks 2 and 7 join the list. 8.4's c4-2 says what forces the act. c5-1 (decision R).
2. **A named miss is keyed by case**, as a record is (line, preset and case, route included): a case that meets its
   line needs none, and a second missing case needs its own, with its own cause (2.4). c5-2 (decision S).
3. **An exact figure has no tolerance** (line 9's file sizes); the 5 % floor is for measured figures (2.4 rule 3).
   c5-3 (decision T).
4. **Decision Q, the orchestrator's: the wait is the release's alone.** The step runs in two ways (2.4). The
   release run waits, bounded at 30 minutes, and fails with "machine busy, run again" past the bound. A unit run, in
   a unit's gate, does not wait: it compares times only where the load is low over their passes, else prints "not
   compared" for each time and passes or fails on the bytes. Applied in 0.6, 2.4, section 5 and 8.4's c4-4, at the
   fourteen passages check 5 lists.
5. **The load guard** (2.4 rule 2): the three passes run as three sweeps over every case, at nice 19 with idle I/O,
   which the step sets itself. The one load signal is the instantaneous runnable count of `/proc/loadavg`, sampled
   every 100 ms: a pass counts only where it stayed at most 10 on average. The one-minute average is not used, since
   it lags (16 jobs for 40 s leave it under 10, rising from 2). c5-4 (decision U; revision 7's form, c6-1).
6. Minor: a case with no record printed and not compared at a unit's gate (c5-5); an armed case missing from the
   output fails either run, unlike one not compared (c5-6); B's premise stated with its exception for the tests'
   unvalidated stacks (c5-7); "private to the module `resolve`", the shared function taking a parsed read (c5-8);
   line 1's `read` case on a pending preset, a lower bound, the probe's function timing the read alone (c5-9); an
   act that raises a named miss's record names the rise's cause (c5-10); 8.3's c3-11 cites rule 3 (c5-11).

## Revision 5

The fourth independent check (`load-design-check4.md`, "check 4", 2 serious, 7 should-fix, 8 minor; cited c4-1 to
c4-17) found the tests that change real presets' stacks left on the full path, the release's rewriting of records
letting a named miss creep, and trust resting on its callers' discipline. Every finding was verified at its evidence
before anything changed (8.4); the orchestrator decided the corrections. Where revision 4's list says otherwise
(its items 2 and 4), the body holds. The tests named below are printed by `load-check/revision5_mutated.py
<native-theme/src>`.

1. **The tests that change a stack the validating construction built make it `Incremental` after construction**
   (3.9 *The tests' stacks*): `breeze_at` and four other places in `resolve/tests.rs`, nine stacks in
   `src/theme/tests.rs`, so B and the differential switch run on `kde-breeze`'s accent-derivation forms, a palette
   and a KDE port; 1.2 and 3.2 (2) corrected. No verdict changes. c4-1.
2. **No run writes a record** (2.4 rule 4, *Who makes records*): a record changes only by a deliberate act with its
   ledger row. Decision M's writing at the release is removed, its reason in 8.4. c4-2.
3. **The trusted construction takes a `PresetId` and no text**: `Stack::bundled(id, scheme)` and
   `Stack::set_preset_bundled(id)`, reading `id.source()` through E1's cache; `Stack::set_preset` keeps the preset's
   own check on every stack; task 1's controls use a `#[cfg(test)]` twin; build-test item 3 builds through
   `Theme::from_preset` and `Theme::set_preset` (3.3 A, 3.9). c4-3.
4. **The load rule waits** (in the release run alone since revision 6, decision Q; by sweeping again, not by
   waiting, since revision 8): the step builds, then waits up to 30 minutes for a runnable count of at most 10 (a
   load average through revision 6), then measures; past the bound it fails with "machine busy, run again", no
   verdict on the figures (2.4 rule 2). c4-4.
5. **"Meets its line" defined once** (at most the line, nothing added); a case whose record does not meet its line
   has a named miss, checked from the file, however close to the line (2.4 rule 1). c4-5.
6. **Every record has an owner**: line 4's palette case (M2's fourth task), the regenerated presets (the trigger
   step), a route switch (the task that empties the table), a spread over 10 % (the recording task fixes the probe).
   c4-6.
7. **G2 counts the reads the calling thread makes**, thread-local as G1 (2.4, task 1). c4-7.
8. **3.8 gains S17's `[S]:7134` and `native-theme/README.md:88-90`**; the grep claim is replaced by the line-joined
   scans. c4-8.
9. **The probe's doc-hidden function takes the validating construction** (task P). c4-9.
10. Minor: the load limit's reason (c4-10); items 2 and 3 in `every_bundled_preset_validates` per `PresetId`
    (c4-11); `adwaita` for the trusted construction's fixture cases (c4-12); the KDE and GNOME cases on Linux
    (c4-13); the step wired into `check_release.sh` in task 6, not before (c4-14); the record's statistic (c4-15);
    the bytes binary with one test thread (c4-16); R2.8's sentence with the class-2 batch (c4-17).

## Revision 4

The third independent check (`load-design-check3.md`, "check 3", 3 serious, 7 should-fix, 8 minor; cited c3-1 to
c3-18) found the validating construction unspecified (so build-test item 2 could pass through B's own recognition),
no time line binding a release, and named misses with nowhere to cite. Every finding was verified at its evidence
before anything changed; the orchestrator decided the serious findings, the maintainer's place and the gating's
simplification (decisions L to P). Section 8.3 lists what was taken otherwise. The call sites cited for the routes
are printed by `load-check/revision4_sites.py <native-theme/src>`.

1. **Two knobs, stated once** (3.9 *Two constructions*): a stack holds its **checking**, `Full` or `Incremental`.
   On a `Full` stack every validation, at construction and at every change, is the full path (today's
   `validate_reusing`, nothing compared, no trusted form consulted, nothing skipped); B runs only on an
   `Incremental` stack. The **validating construction** is today's `Stack::new` and `Stack::synthetic` with
   `Full`; the build test (through `preset_stack` and `synthetic_stack`, as today) and the facts gate (its
   `Stack::new` and `Stack::synthetic` calls) use it, so the gate's per-vector changes check the derivation layer at
   every condition vector, as today. The **trusted construction** skips the preset's own check and is `Incremental`.
   c3-1 (decision L).
2. **What decides trust** (3.3 A): the route and the embedded text. Only `Theme::from_preset(id)` and
   `Theme::set_preset(id)` take the trusted construction, both with `id.source()`, and only where its `[pending]`
   table is empty. Every other route validates, `preset_stack` with any text and any id included, so the 26 test
   calls of `preset_stack(None, …)` and `validates(None, …)` keep their verdicts. c3-6.
3. **Build-test item 2 checks each trusted form on the full path** (`check_stacked` of the form over the preset, as
   the full path runs it), with a negative control run with the switch off. c3-1 (decision L).
4. **The gating in one section of four rules** (2.4, decisions M, N, P): a figure fails where it passes its bound (its
   line, or a named miss's record) by more than its tolerance; times are compared only at a load average of at most 10
   (the runnable count since revision 7); each tolerance is twice its record's measured spread and at least 5 %; the
   release must compare every armed line (revision 5 removes its writing of records, 8.4). Removed: the factor 5, the
   load guard at 86, the stale record's lower bound, the record-against-line check from the file alone, the "reported
   only" state for a line of wide spread, the doc-hidden function turning the switch on in the gate. c3-2, c3-10,
   c3-11, c3-15, c3-16.
5. **A named miss cites any section of this note or any task**; 3.6 gains rows for lines 4 (KDE's light and dark),
   7, 8 and 11; line 8 without the query cache is "not binding", with its reason; "armed" is defined, and every line
   is armed from its first record or states why not. c3-3, c3-10 (decision N).
6. **The maintainer is no blocker inside M2** (section 5): the step "this note confirmed by the maintainer" is
   gone; the lines apply as proposed. The triggers are evaluated at **the trigger step**, defined here as an M2 step
   after the staging task (staging design §2.7) merges, since neither §2.7 nor P-6 holds one. c3-8 (decision O).
7. **The input cycle search stays inside `combos::check`**, so the preset's own check keeps it
   (`check_preset` → `combinations`); B runs it alone for each layer whose checks over combinations it skips. A
   control for a preset whose inputs form a cycle. c3-4.
8. **A control for an inputs-only layer with an undeclared input** (structural checks while B skips). c3-5.
9. **3.8's list** gains `src/lib.rs:9-11`, `native-theme/README.md:12-15`, `:112-116`, `README.md:72` and
   `CHANGELOG.md:30`. c3-7.
10. **Task 1's presets are those whose table is empty at task 1**, at least `adwaita` and `kde-breeze`
    ([plan]:996-1000); the Windows cases take the synthetic theme while `windows-11`'s is pending. c3-9.
11. Minor: the compared form written field by field with no clone, by exhaustive destructuring (c3-12); the preset's
    layer, index 0, among the compared layers (c3-13); the input cycle check in R2.8's and S8's sentences (c3-14);
    the pt/px defect's line-height route (c3-17); task 4b `implement` with a review (c3-18).

## Revision 3

The second independent check (`load-design-check2.md`, "check 2", 3 serious, 11 should-fix, 9 minor; its findings
are cited c2-1 to c2-23) found B unsound in three places, a cost claim that did not follow, and a build test that A
would leave vacuous. Every finding was verified at its evidence before anything changed; the orchestrator decided
the serious findings, the triggers' timing, the budget step's gating and task 1's agent (decisions H to K). Section 8
lists what was taken otherwise. The derived figures this revision adds are printed by
`load-check/revision3_figures.py` (items cited "r3f N").

1. **Every layer's structural checks at every change** (3.3 B, 3.8's S12 paragraph): `checks::layer` and
   `sets_stated_whole` for every layer above the preset, as `check_stacked` runs them today
   (`src/validate/mod.rs:96-99`), since they read the current preset, its declarations and the layer's left-out.
   A `set_preset` control in task 1. c2-1 (decision H).
2. **B's comparison is plain layer equality with the inputs set aside** (a palette's inputs kept), each layer with
   its kind and left-out, the context compared too; the rules' selectors are part of the layer, so a `when` change
   is never skipped. No flattening. A layer is left out of the comparison where it is empty with its inputs set
   aside. After task 2, the values the flattening reads as `Unknown` (colours, then; strings, flags and integers
   already) are set aside as well, by a comparison over the layers' values, not a flattening. The cost is measured by
   the probe; the rate of equality on the presets' own layers is 0.048–0.059 µs per flattened op (r3f 1). Revision
   2's "a change of inputs alone needs no check" was too broad: on `kde-breeze` the inputs choose the
   accent-derivation layer's form and the reader's left-out, which B compares. c2-2, c2-4, c2-15 (decision H).
3. **The input cycle search runs per layer, over the stack up to it, at every change**, as today
   (`combos/mod.rs:3254-3267` over `stack.layers`, the prefix). A control in task 1. c2-5 (decision H).
4. **Line 4 for a re-read is an expectation to be measured**, not a claim met: the reading's structural checks and
   its comparison are expected at 0.60–1.26 µs per op, so 0.5 ms is reached at about 400 to 840 ops (r3f 6). The
   "300 ops" and the flattening's 2.4 ms are gone. c2-2 (decision H).
5. **The build test gets a validating construction of its own** (3.9), never `from_preset`'s route, with a negative
   control in task 1; S19.1's sentence at [S]:7677 and the doc comments at `src/theme/tests.rs:20-24`, `:50-55`
   rewritten (3.8). c2-3 (decision H).
6. **The differential switch and G1 are thread-local**; G1 counts B's runs only, so the switch's own runs never
   break a "G1 = 0" test; section 6's item 4 says what the switch can detect. c2-6.
7. **The budget step's gating** (2.4): every recorded figure is a ratchet (bytes 1.05, times 5, each from measured
   spread); a recorded figure over its line needs a named miss with its trigger; time figures are compared only on
   the measuring machine's release-mode step, under a load guard, and never in `cargo test` or CI. c2-7, c2-8
   (decision J).
8. **The triggers are evaluated at the staging task's last step** in M2, on the eight presets outcome 2's staging
   regenerates; a part whose trigger fires is built before M3a's first widget unit; the probe therefore comes first.
   The calibration after M3a's first unit is a second look (3.6, section 5). c2-9 (decision I).
9. **Task 1 is `implement-hard`.** c2-10 (decision K).
10. **Task 2's audit list gains `combos/motion.rs:303` and `:601`** and covers equality as well as patterns; task 7's
    "an unrecorded read does not compile" now names how a derived `PartialEq` is kept from reading payloads. c2-11.
11. **A palette applied or removed over a structural reading runs one check** (3.3, 3.5). c2-12.
12. **G2's route and E1's holding** stated: one function `from_system_async` and the test share; the stack holds the
    cache's `Arc`, so the declarations are shared too. c2-13.
13. **R2.8's and S8's new sentences** name a preset whose `[pending]` table is empty and "the crate's own tests".
    c2-14.
14. Minor: the synthetic route's load on the full checks, its mutations under B (c2-16); 0.3's figures on the plain
    route, `material-3` left out (c2-17); the ratios of one check to line 4 (c2-18); task 4b's pending presets
    (c2-19); the counting allocator named (`peak_alloc`, c2-20); the probe renamed **task P** (c2-21); line 11's
    0.9 GB explained (c2-22); 6.7 on task 7's KDE part (c2-23).
15. **The pt/px candidate is a defect in today's checks** (1.2, *A defect of today's checks*): two sides that reach
    the same stated number through a font size and through a length compare as the same length. Recorded for the
    orchestrator's separate fix; B neither causes nor hides it.
16. Line 12 is called what it is, a figure reported only: the budget has eleven lines.

The Revision 3 and Revision 2 lists record those revisions; where a later revision changed a point (revision 4: items
5 and 7 of revision 3, the gating and the build test's route), the body holds.

## Revision 2

The independent check (`load-design-check.md`, 4 serious, 14 should-fix, 6 minor) found that a smaller design does
for what was measured. The orchestrator set the revision's scope; each change below names the finding it answers
(numbers as in the check; section 8 lists what was not taken, or taken otherwise, and why).

1. **Scope.** On this release's path: A (trust), B (the checks over combinations run only where their input
   changed), E1 (one read of a preset per process, held weakly), E2 (the last resolution kept in `WindowState`), the
   query cache only if the query profile asks for it. Moved off the path, each with its trigger (3.6): sparse
   patches (C), the compact form (D), resolution by need, the repairs inside the checks, the kept rules held by
   reference. None of them is called a measured need any more. Findings 4, 7, 8, 10, 15.
2. **B redefined.** The old B1/B2/B3 with "inert statements" and "closed places" (a)–(c) is gone from the path.
   B is now: the checks over combinations are a function of the flattened stack and their context; they run for a
   layer only where the flattened stack up to it differs from one that passed. Inputs never enter the
   flattening, so a change of inputs alone needs no check. Sound by determinism, with one code audit (1.2) and a
   differential switch as its guard. Findings 1, 2, 12, 13, 22, 24.
3. **Colour values out of the checks' sight** (task 2): today a stated colour flattens to `Fact::Number` of its red
   channel (a code fact the check did not report, 1.2); the thirty `Fact` reads are listed and none reads a colour
   place; making that mechanical lets B skip a reading that changes colours only. Finding 2, finding 16's "by type".
4. **`from_system`** has its own budget line (10) and reach rows per reader (3.5); its options are weighed (3.4);
   the way chosen for what A and B leave is the readings proven at build time, decided by a measurement task
   (task 4b). Findings 3, 16.
5. **The read cache is held weakly**, so line 3 counts it with the theme; `font_dpi` reads through it. Finding 5.
6. **The resolution is kept in `WindowState`**, not in `Theme`. Finding 6.
7. **The budget** has a 0.7.0 column with what each line gives up, a line for `from_system` and one for resolve's
   peak memory, each number marked as a judgment, and is rechecked against what gpui, iced and egui do. Findings
   9, 17.
8. **Gating.** Time lines are measured and reported by a release-mode step on the measuring machine, never gated;
   the gates count work (calls of the checks, reads, resolutions built) and gate the byte lines. Finding 11.
9. **Done-criteria** each have a case that fails: negative and positive controls, a differential switch with its
   own positive control. Finding 13.
10. **The build test's own cost** under A is measured at task 1 and placed by the plan's limits. Finding 14.
11. **The rules' changes** re-quoted, each with its new wording and its class: R2.8 (class 3, reworded); S8 twice,
    S9.2, S12, S16, S19.1 (class 2); the code's doc comments named in task 1; S8's `Scope` only with the query
    cache. Findings 17, 18.
12. **Resolve at thirty widgets** projected (an expectation) and the line 5 fallbacks named. Finding 10.
13. **Facts corrected**: `Fact`'s four variants, the accent-derivation layer on `kde-breeze` only, no bundled
    palette yet, the minor figures. Findings 2, 19, 20.
14. **Line 1 while tables are pending** is timed by a doc-hidden function that returns figures, never a theme;
    lines 4 to 8 on presets whose table is empty. Finding 21.
15. **Tasks**: agent types, an archiving task, where the counters live. Finding 23.
16. Line numbers: lines 1 to 9 keep revision 1's; 10 (`from_system`), 11 (resolve's peak) and 12 (an application
    layer that states structure, reported; revision 1's "tenth line") are added.
17. The memory figures of 1.1 recomputed from the marks (`load-check/read_marks.py`), all as the check gave them.

---

**Sources.** All code and documents on `m1b-close` at 7430a0a2, read in
`/home/tibi/Rust/native-theme/.claude/worktrees/m1b-close`; paths below are relative to it, `src/…` is short for
`native-theme/src/…`, and a bare `combos/…`, `stack.rs` or `walk.rs` is under `src/validate/`.

- [P]: `scratch-keep/d6/data/preset_load_profile.md`, the profile; [P raw]: its per-run files
  `scratch-keep/d6/prof/r3/<preset>-light-{synthetic,plain}-{cpu,mem}.txt` and `…-mem-marks.csv`.
- [L]: `scratch-keep/d6/data/preset_load.md` (0.7.0's figures, and the 0.8.0 slice's on a machine at load average
  20 to 86); [O], [D]: `d6-outcome.md`, `d6-decisions.md`; [pkg]: `d6-draft.md`, the D6 package (all under
  `/home/tibi/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/d6/`).
- [R], [S], [plan]: `docs/todo_v0.7.1_model-completeness-rationale.md`, `-spec.md`, `-plan.md`, cited by line.
- [mine]: arithmetic on the figures just named, or a scan of the files, by the scripts in
  `scratch-keep/d6/load-check/`: `revision2_figures.py` (every derived time, rate, factor and projection of
  revision 2), `revision3_figures.py` (those revision 3 adds: the rates of equality and of the structural checks,
  the spread of wall times, the plain route's sums; cited "r3f N"), `preset_counts.py` (1.3), `fact_reads.py` (the
  `Fact` reads of 1.2, pattern matches only), `content_bytes.py` (section 8), `revision4_sites.py` (the call sites
  of `preset_stack`, `validates`, `synthetic_stack`, `Stack::new`, `Stack::synthetic` and `from_preset`, a text
  scan; its `Stack::new` lines under `src/validate` are the flattened stack's, not the theme's),
  `revision5_mutated.py` (the tests that change a stack the validating construction built, 3.9),
  `revision6_load.py` (item 3: the one-minute load average's lag under a burst, the reason 2.4 does not use it),
  `revision7_situations.py` (2.4's rules applied to check 6's situations, 8.6) and
  `revision5_spec_rule.py` (statements of the old rule in the specification and the rationale, 3.8), with check 4's
  `scratch-keep/d6/load-check4/wrapped_rule.py` (the same over the code and the READMEs, line breaks joined). No
  build, test or measurement was run.

**Words.** *Bundled data*: the bundled presets (and, once M2's fourth task bundles them, palettes) and the layers
`Theme` derives from them alone. *The checks over combinations*: S12's *Combinations, conservatively*
([S]:6028-6052), in code `validate::combinations`, which flattens the stack and runs `combos::check`
(`src/validate/mod.rs:199-208`, `combos/mod.rs:466-489`). *The structural checks*: `checks::layer` and its siblings,
which look at each value and rule on its own (`src/validate/mod.rs:3-6`). *The flattened stack*: what
`stack::Stack::new` builds from the layers, the values each states place by place (`stack.rs:1-5`). *A trusted
form*: a stack of a bundled preset whose `[pending]` table is empty with layers `Theme` derives from bundled data
alone, as 3.3 A lists. *A layer's compared form*: the layer with its inputs set aside (a palette's inputs kept), as
B compares it (3.3). *A structural reading*: a reader's reading whose compared form is not empty (it states more
than inputs). A figure written `a / b / c / d` is for `windows-11` / `material-3` / `macos-monterey` /
`macos-tahoe`, the four presets the profile ran.

---

## 0. The decisions in short

1. **Budget** (per preset, release build, the measuring machine), every number a proposal: first load 16 ms;
   `from_system`'s own part 16 ms; peak during a load 16 MiB; kept 8 MiB; a change every application makes 0.5 ms;
   resolve 1 ms, 10 µs when nothing changed, its peak 8 MiB; a styled query 50 µs computed, 2 µs repeated;
   256 KiB in the binary; and the cost of adding an application layer that states structure, reported, not set
   (line 12). Section 2 says which are judgments and what each gives up against 0.7.0.
2. **On this release's path.**
   - **A, trust**: a bundled preset with an empty `[pending]` table, and the layers `Theme` derives from it alone,
     are checked by the build test and by the facts gate at every commit, both through the validating construction
     (the full path, B off), and never at run time. Only `from_preset` and `set_preset` take the trusted
     construction, which takes a `PresetId` and no text and reads the embedded text itself.
   - **B, check only what changed**, on a theme's stack (never on the validating construction's): for each layer
     above the preset, the checks over combinations run only where the stack up to it, each layer in its compared
     form (its inputs set aside), differs from one that passed (a trusted form, or the stack before the change);
     after task 2 also the values the checks read only as stated (colours, strings, flags, integers). Every layer's
     structural checks and its input cycle search run at every change, as today.
   - **E1**: one read of a preset per process while a theme holds it (a `Weak` per preset).
   - **E2**: `WindowState` keeps the resolution it returned; `resolve` returns it again when nothing changed.
   - **The query cache** inside the resolution, built only if task 4a's profile and the connectors' design show a
     connector must query every frame and the query cannot be brought near 2 µs.
3. **`from_system`.** A and B are expected to meet its line for a reading of inputs alone (the Windows reader's
   ordinary reading), and line 4 for a re-read whose compared form is unchanged (an accent alone on GNOME and
   Windows; GNOME's and macOS's scheme switch; after task 2 an accent on KDE and macOS at the same scheme) where the
   reading is small enough: its structural checks and its comparison are expected at 0.60–1.26 µs per op, so about
   400 to 840 ops reach 0.5 ms (r3f 6); the probe and task 4b measure it. A structural reading (KDE's always,
   GNOME's with a font, macOS's, Windows' in high contrast) costs one run of the checks: expected 101.6 / 331.7 /
   6542.2 ms on `windows-11`, `macos-monterey` and `macos-tahoe` (the plain route's figure, a trusted theme's;
   r3f 7), not profiled on `kde-breeze` and `adwaita`; `material-3` has no reader. Task 4b measures it on the four
   readers' fixtures. The way chosen to remove it, where 4b shows the line missed, is **the readings proven at build
   time** (3.4): a reader's places and value kinds are fixed by its code, so the build test proves each reader's
   maximal reading over its own preset and the run time accepts a reading that states those places and no others.
   It changes no API, so it can wait for the figure.
4. **Off the path, each with its trigger** (3.6): C, D, resolution by need, the repairs inside the checks, the kept
   rules held by reference, a feature per preset, and the parts for lines 4, 7, 8 and 11. Their triggers are
   evaluated at the trigger step in M2 (section 5), on the presets outcome 2's staging regenerates; a part whose
   trigger fires is built before M3a's first widget unit.
5. **Errors** arrive where they arrive today; a bundled preset with an empty `[pending]` table cannot be refused at
   run time.
6. **Gating** (2.4): the work G1 to G4 in `cargo test` on every platform; the bytes and the times on the measuring
   machine's release-mode step, by four rules: a case whose record does not meet its line has a named miss of its
   own, and a figure fails past its line, or past its miss's record where that is higher, by more than its
   tolerance; under a miss whose record is over its line, a figure that has fallen below its record by more than
   that is a stale record, which fails the release; times are compared only on a quiet machine: the release run
   sweeps again until it has one and fails with "machine busy, run again" if it stays busy; a unit run, at a unit's
   gate, does not wait, and prints a time it could not compare "not compared"; the tolerance comes from measured
   spread, 0 for an exact count; the release's step must pass, and no run writes a record, only a deliberate act
   does.
7. **Order** (section 5): the probe (task P), then the trigger step once the staging task has merged; the query
   profile before the connectors' design is checked; trust, B and the kept resolution before the first connector's
   code; colour values, the `from_system` figures, the gate's step and every part whose trigger fired before M3a's
   first widget unit; a second look at the calibration after that unit. No step waits for the maintainer.

---

## 1. The facts

### 1.1 Measured ([P]; one widget of thirty)

| | windows-11 | material-3 | macos-monterey | macos-tahoe | source |
|---|--:|--:|--:|--:|---|
| load, base, ms | 259–266 | 4144–4185 | 1278–1293 | 15,635–17,605 | [P] §1 |
| plain `from_preset` until refused, ms | 174–177 | 3004–3051 | 666–671 | 9673–9711 | [P] §1 |
| peak (`VmHWM`, process), MiB | 40.5–41.6 | 127.2–134.9 | 202.0–202.6 | 1242.0 | [P] §1 |
| peak above the load's start (`peak_alloc`, plain / synthetic), MiB | 18.0 / 23.1 | 100.4 / 101.0 | 168.6 / 186.4 | 1827.0 / 1828.9 | [P] §4 |
| resolve, ms | 0.23–0.24 | 0.33–0.39 | 0.72–0.77 | 1.16–1.47 | [P] §1 |
| styled query, µs | 50–53 | 34–49 | 32–36 | 35–37 | [P] §1 |
| file, bytes | 224,294 | 316,862 | 937,875 | 1,759,947 | [P] §1 |
| `[[rules]]` | 343 | 521 | 1983 | 3088 | [P] §1 |
| `read_preset`, ms (synthetic / plain route) | 6.5 / 7.4 | 9.3 / 8.6 | 37.1 / 38.2 | 57.4 / 53.7 | [P] §2, §5 |
| of it the TOML parse, ms (synthetic / plain) | 3.3 / 4.1 | 4.9 / 4.5 | 14.5 / 16.7 | 25.3 / 25.1 | [P raw] `T_TOML_PARSE` |
| check of the preset's own layer, ms | 105.5 | 2305.9 | 342.5 | 6450.9 | [P] §2 |
| of it the flattening and `combos::check` (one run with nothing reused), ms | 102.7 | 2289.7 | 331.0 | 6416.2 | [P raw] `T_FLATTEN` + `T_COMBOS_CHECK`, column `preset` |
| check of the accent layer (it never states more than inputs; nothing on these runs), ms (synthetic / plain) | 17.1 / 16.2 | 1019.0 / 927.8 | 132.6 / 120.8 | 3299.1 / 3192.1 | [P raw] `T_COMBINATIONS`, column `accent` |
| the input cycle search, per call, ms | | | | | 0.001–0.121 over every column, [P raw] `T_INPUT_CYCLES` |
| flattening per op, preset column, plain route, µs | 1.23 | 0.92 | 5.70 | 8.03 | [P raw] `T_FLATTEN` ÷ `C_OPS` |

Memory of the read alone, from the plain route's marks ([P raw] `…-light-plain-mem-marks.csv`, `T_READ_PRESET`'s
begin and end and the highest peak between them, `load-check/read_marks.py`):

| | windows-11 | material-3 | macos-monterey | macos-tahoe |
|---|--:|--:|--:|--:|
| held when the read ends, above its start, MiB | 2.63 | 4.14 | 28.3 | 45.4 |
| peak during the read, above its start, MiB | 4.66 | 7.25 | 37.1 | 62.1 |
| of it inside the TOML parse, MiB (the check's finding 4) | 4.27 | 6.17 | 25.58 | 48.39 |
| held, bytes per rule | 8,042 | 8,332 | 14,983 | 15,423 |

What the theme keeps after a load is 4.0 / 4.9 / 31.9 / 47.6 MiB by [P] §4 (`peak_alloc`, the theme alive; the
profile's figures include what the test holds, so this is not a clean figure of the theme alone).

0.7.0 ([L], second table): first `Theme::preset` 0.457–0.768 ms, a later one 0.92–1.00 µs, `resolve` 0.020–0.023 ms,
a field read 0.7 ns, process peak 4.9–5.1 MiB (4.3–4.4 before the work), 25,902–62,653 bytes a preset, 308,299 the
seven. 0.7.0's `from_system` was not measured.

### 1.2 What the code does (7430a0a2)

**Loading.**
- `Theme::from_preset` reads the text and builds the stack with every check (`src/theme/mod.rs:82-85`, `:333-337`):
  `Stack::new` runs the preset's own check (`preset_problems`), then `validate_reusing` over every layer above it
  (`src/resolve/mod.rs:669-704`).
- `validate_reusing` runs `check_stacked` once per layer above the preset (`src/resolve/mod.rs:1429-1447`), and each
  runs the checks over combinations over the whole stack up to that layer (`src/validate/mod.rs:86-94`, `:199-208`).
  The stack always holds the accent layer (`src/resolve/mod.rs:1313-1317`), which states the accent inputs and
  nothing else (`:1120-1137`): its check costs 17.1 / 1019.0 / 132.6 / 3299.1 ms (1.1).
- On `kde-breeze` alone the stack also holds the accent-derivation layer directly above the preset
  (`derives_accent`, `src/resolve/mod.rs:1870-1881`; derived at `:1167-1210`); `material-3`'s is the
  specification's, M2's fourth task ([plan]:1066-1070), not code yet. No bundled palette and no `Palette::bundled`
  exist yet (`grep 'fn bundled' src/palette` finds none).
- The preset is parsed again by `set_preset` (`src/theme/mod.rs:126`) and by `PresetId::font_dpi`
  (`src/presets/mod.rs:80-94`), which the GNOME and KDE readers call at every read (`src/gnome/mod.rs:187`,
  `src/kde/mod.rs:254`).

**Mutations.** `pick`, `set_contrast`, `set_flavor`, `set_accent`, `add_layer`, `set_reader` and the rest go through
`Stack::change`, which copies the stack, derives the palette's entry, the accent layer and the accent-derivation
layer again and calls `validate()` (`src/resolve/mod.rs:819-841`); `validate()` starts from an empty reuse memory
(`:1412-1414`). So a `pick` on a theme of a preset alone runs the accent layer's check with nothing to reuse: one
flattening and one `combos::check` of the whole stack, 101.6 / 2389.1 / 331.7 / 6542.2 ms if it costs what the
preset's own did on the plain route (`revision2_figures.py` item 2). **An inference from the code, not a
measurement.** Each change also copies the stack, and with it the declarations, which the stack holds by value
(`src/resolve/mod.rs:605`, the copy at `:830`).

**`from_system`** (`src/theme/mod.rs:66-77`): the reader reads first (the KDE and GNOME readers parse the preset for
`font_dpi`), then `from_preset` builds and checks the stack, then `take` calls `set_reader`, a mutation as above
(`:145-152`, `src/resolve/mod.rs:844-855`): two or three reads of the preset and two validations. What each reader
states (no iOS reader exists, `src/reader.rs:116-150`):

| reader, own preset | its layer | source |
|---|---|---|
| KDE, `kde-breeze` | `input.accent`; the text-role fonts `body`, `monospace`, `caption` as `Fields` patches; under `for_preset.kde-breeze`, at the scheme read, the colour results, the inactive results and scheme inputs (inputs), the keyed fonts `toolbar` and `menu`, the icon theme and sizes, the motion speed factor; and in every reading a `Tokens` rule under `scheme:<s>` and `preset:kde-breeze` | `src/kde/mod.rs:229-350`, the rule `:334-344` |
| GNOME, `adwaita` | `input.accent` at both schemes; `body` and `monospace` fonts as `Fields` patches where gsettings gives them; the icon theme under `for_preset.adwaita` | `src/gnome/mod.rs:183-200`, `:214-247` |
| macOS, `macos-tahoe` or `macos-monterey` | the five accent inputs; the roles of both appearances under `for_preset.<own>` (surface sets as `Fields` patches, `src/macos.rs:239`), or under Increase Contrast a `Colors` rule | `src/macos.rs:319-405`, the rule `:357-373` |
| Windows, `windows-11` | the accent and its ramp, inputs at both schemes; under high contrast a `Colors` rule | `src/windows.rs:326-398`, the rule `:376-391` |

The macOS and Windows readers' build functions are compiled on every platform for their tests
(`src/lib.rs:175-177`, `:188-190`); the KDE reader needs Linux and its feature (`:164-165`).

**What the checks over combinations read** (the basis of B):
- The flattened stack: each layer's bases, its part for the current preset and its rules, with what S8 leaves out
  of it and what it is to the stack (`stack.rs:735-780`); the context: the declarations, the preset, the layer's
  kind, the layers below and whether the reader's accent is left out (`src/validate/mod.rs:57-70`; no scheme,
  contrast or flavour).
- No input enters the flattening (`grep inputs src/validate/stack.rs` finds nothing). A reference to an input is
  read as stated (`combos/mod.rs:900-904`, `Ref::Input(_) … => vec![Place::Stated]`). The inputs are read by the
  input cycle check inside `combos::check` (`combos/mod.rs:487`, `:3254-3267`; `combos/inputs.rs:223-253`) and by
  the KDE palette mapping check of a palette layer (`combos/mod.rs:3972`, reading inputs at `:3988`,
  `:4042-4049`). No other site under `src/validate/combos` or `stack.rs` reads a layer's inputs (`grep` for
  `inputs` there finds these and the cycle search's own, `combos/inputs.rs:132`, `:231`). The reuse memory
  compares raw layers, each with its kind and left-out, and the preset and declarations
  (`combos/mod.rs:2203-2244`, the comparison at `:2209-2219`), which decides work, not findings.
- The rules' selectors: a flattened `Source` names its rule by index only (`stack.rs:170-171`); the selector lives
  in the stack's `rules` (`RuleInfo`, `stack.rs:224-230`), which the checks read when they build worlds
  (`combos/mod.rs:1356`, `:1380`). A rule's selector is a field of the layer (`Rule { selector, body }`,
  `src/selector.rs:17-22`), so the layer's own equality compares it.
- Every layer's bases push a `Tokens` source, empty or not (`stack.rs:829-831`), so a layer stating nothing still
  has a place in the flattened stack: the checks use it for order (`Pos`, `combos/mod.rs:733-734`) and to name a
  problem's layer.
- The input cycle search runs inside each layer's check over the stack up to that layer: `check_stacked` builds the
  prefix (`src/validate/mod.rs:91-94`) and `combos::check` searches `stack.layers` (`combos/mod.rs:3254-3257`). An
  input's value is the last layer's stating it (`combos/inputs.rs:116-140`), so a cycle in a lower layer can be
  broken by a higher one: the search over the whole stack would accept what the search over the prefix refuses.
- A stated value flattens as `Stated(Fact)` with four variants, `Unknown`, `Nothing`, `Number`, `Variant`
  (`stack.rs:74-85`); a field a patch sets to `None` or an empty list is a `Touch`, which a lookup reads as
  `Nothing` (`combos/mod.rs:665`). A stated value starts as `Unknown` and takes the first number or variant its walk
  meets (`stack.rs:443-455`, `:585-597`, `:640-642`). So a number, a length, a duration, a font size or a scalar is
  a `Number`, an enum a `Variant`, a string, a flag or an integer stays `Unknown` (`walk.rs:904-921`), and **a
  stated colour is `Number` of its red channel**, since a `Color` walks its four channels as numbers
  (`walk.rs:882-893`).
- The `Fact` reads: thirty sites and two helpers, thirty-two lines (`load-check/fact_reads.py`:
  `combos/mod.rs:2106`, `:2157`, `:2457`, `:2645`, `:2757`, `:2832-2833`, `:2964`, `:2972`, `:3345`, `:3350`, `:3388`,
  `:3403`, `:3409`, `:3414`, `:3446`, `:3515`, `:3526`, `:4012`; `combos/motion.rs:83`, `:190`, `:197`, `:209`,
  `:256`, `:267`, `:296`, `:444`, `:470`, `:473`, `:589`; and the two helpers at `combos/mod.rs:2017-2028`), and two
  reads the script's patterns miss (c2-11): `combos/motion.rs:303`, which binds every stated `Fact` below a timing
  (`OpKind::Set(Val::Stated(fact))`), and `:601`, where `curves.contains(&facts)` compares those payloads by the
  derived `PartialEq` (`Fact`, `Val` and `OpKind` derive it, `stack.rs:75`, `:88`, `:101`). The list is a text scan:
  it covers patterns and the two helpers, not every equality on a payload, which task 2's audit adds. By reading each,
  they look up durations, delays, timings and engines (`motion.rs`), side lengths and the terms of length expressions,
  following references (`combos/mod.rs:2101-2160`), minimum and maximum sizes (`:2832`), corner shapes (`:2645`,
  `:2757`, `:3345-3526`), the text scaling's dynamic type (`:2457`), an icon glyph's source and rendering mode
  (`:2964`, `:2972`) and, for a palette on `kde-breeze`, the tint factor (`:4012`). None names a colour place, and a
  reference followed from them carries the field's type (`walk.rs:657-672` passes `T::ty()` with an expression). So,
  as far as this reading goes, no check reads a colour's `Fact`; the font sizes are read, through a length
  expression's `font_size` (`combos/mod.rs:859-869`) and the terms' references (`:2132-2160`). Task 2 audits the list
  and makes the colour half mechanical.
- Rules interleave with the patches kept on a reference by position, and a base never comes after a rule
  (`combos/mod.rs:733-741`, the check's finding 1): a rule appearing at a referenced place changes what a lookup
  elsewhere answers; a base does not.
- The structural checks read the context too: `checks::in_flavor` checks a layer's `for_preset` part only while
  its preset is current, against that preset's declarations and flavours (`src/validate/checks.rs:65-112`), and
  `sets_stated_whole` reads the layer's left-out (`src/validate/mod.rs:126-129`), which follows the layers above it
  (`src/resolve/mod.rs:1186-1200`). So a layer that did not change can fail them after a change beneath or above it
  (c2-1's case: an application layer whose `[for_preset.adwaita]` part names a key `adwaita` does not declare,
  harmless on `kde-breeze`, refused at `set_preset(adwaita)`).

**A defect of today's checks** (check 2's unverified candidate, verified here from the code; not executed). The inset
and offset checks (`focus_corners`, `concentric`, `combos/mod.rs:3426-3560`) compare two sides as `SideLen`s and
refuse a corner whose sides "can differ" (`ProblemKind::UnequalOffsets`, `UnequalInsets`, `src/error.rs:630-645`). A
side's expression becomes a `Term`, each reference followed to the stated number it reaches, `Term::Number(n)`
(`combos/mod.rs:2155-2158`), and two sides are the same where their terms are equal (`SideLen::same`, `:2077-2088`).
The number is the stated one in its own unit: a font size flattens to its number whether it is `Pt` or `Px`
(`walk.rs:847-851`), and `font_size` in a part reads the part's text's `font.size` (`combos/mod.rs:859-869`).
Resolution gives `font_size` as the part's resolved text size in px (`src/resolve/evaluate.rs:1534-1536`,
`:1803-1813`), converted at `pt × dpi / 72` (`:743-748`) and scaled by the text scaling. The case: on a part with
`focus.corners.top_left = "follow_part"`, its text's font size stated as 12 pt and its line height as 12 px (a line
height is a `FontSize` too, `src/blocks/text.rs:398`), the offsets `top = "= font_size"`, `left = right = "=
line_height"`. The check reads both terms as `Number(12)`, the sides as the same, and raises nothing; at 96 dpi the
top offset resolves to 16 px and the sides to 12 px, the case S12 refuses. `font_size` and `line_height` are allowed
in a part (`src/validate/checks.rs:1254-1263` refuses them only in roles and tokens) and typed as lengths
(`src/expr/ty.rs:740`). A unit-free case exists too: under a text scaling other than 1, `font_size` resolves scaled
while a length token it is compared with does not. No bundled preset meets it: `font_size` appears only in line
heights and letter spacings (`adwaita.toml:2624` and the Apple presets' `sf_tracking` calls), the `follow_part` offsets
refer to `token.focus` (`adwaita.toml:2264-2270`), and the one route through a line height, a padding the concentric
check reads (`inset_chain`, `combos/mod.rs:3839-3866`) that refers to `widget.label.line_height`
(`presets/windows-11.toml:2164`), meets only `auto` line heights (`presets/windows-11.toml:1624-1832`, `:2090`),
which a term reads as opaque (c3-17). It is a defect of the checks, not of this design: B reuses their verdicts and
changes none, and the differential switch cannot see it (both sides run the same checks). The orchestrator fixes it
apart from this note; its failing tests are commit `ebd1c723` on the branch `side-units`.

**Resolution.** `Theme::resolve(&self, window: &mut WindowState, …)` (`src/theme/mod.rs:325-327`) builds the
resolved theme at every call, changed or not (`src/resolve/mod.rs:1552-1567`), cloning each kept rule's selector and
patch (`:2083`, `:2090`). `WindowState` records the last resolution's inputs only (`:281-300`). [R]:346-348 says of
resolution "its result is cached"; the code holds no cache.

**Gate and build test.** The facts gate validates each preset once (`src/gate/mod.rs:160-193`) and, per condition
vector, copies the stack and calls `set_flavor` where the vector names a flavour and `set_reader_stand_in_carrying`
(`:380-396`): one or two full validations a vector. The build test `every_bundled_preset_validates`
(`src/theme/tests.rs:20-35`, `:57-80`) validates each preset once, `Scheme::Light`, through the plain route where
its table is empty and the synthetic route otherwise; CI runs it on Linux, Windows and macOS
(`.github/workflows/ci.yml:239-275`). Its duration is recorded nowhere. `preset_stack` is called by `from_preset`
(`src/theme/mod.rs:83`) and by the tests, 21 times with `None` and once in the build test's `validates`
(`src/theme/tests.rs:60`), which the tests call 5 times with `None` (`:100`, `:126`, `:295`, `:299`, `:992`) and
once with `Some(id)` (`:33`; `revision4_sites.py`). The mutations' own tests (`src/theme/tests.rs:1-2`,
`resolve/mutation_tests.rs`) build their stacks by a struct literal, `resolve/tests.rs:26-51`, not through
`Stack::new`. Other tests change stacks they build through the validating constructions: through `Stack::synthetic`
of `kde-breeze` and `adwaita` (`resolve/tests.rs:625-983`, `:1099-1337`, the helper `breeze_at` at `:1051-1061`),
the stacks with the accent-derivation forms, a palette and a KDE port; and through `preset_stack` and
`synthetic_stack` of fixture texts (`src/theme/tests.rs:1187-1513`); `resolve/tests.rs:246` builds one through
`Stack::new` only to see it refused (`load-check/revision5_mutated.py`). No code under `src/validate`, `src/toml`,
`src/resolve/mod.rs`, `src/layer.rs` or `src/declare.rs` is conditional on a target or a feature (`grep 'cfg('` finds
test attributes only).

**Connectors (0.7.0's model, kept in the tree).** gpui, iced and egui convert a resolution into their toolkit's
objects when the theme changes (`connectors/native-theme-gpui/src/lib.rs:322`, `colors.rs:174`;
`connectors/native-theme-egui/src/ext.rs:197`; `connectors/native-theme-iced/src/styles.rs:198-200`, whose style
closures capture the colours when they are built). egui-widgets reads the resolved theme inside every widget's
`ui()`, that is every frame (`connectors/native-theme-egui-widgets/src/switch.rs:164`, `:198-199`).

### 1.3 Computed for this note ([mine])

| | all eight | range per preset | method |
|---|--:|--:|---|
| bytes | 4,731,936 | 169,338–1,759,947 | file sizes (`preset_counts.py`) |
| in comment-only lines | 1,115,485 (23.6 %) | 61,462–380,858 | lines whose first non-blank character is `#` |
| zlib level 9 | 271,650 (5.7 %) | 17,954–72,749 | `zlib.compress(raw, 9)`; an indicator of redundancy, not a format |

The read takes 27.2 to 40.8 ms per 10⁶ bytes over both routes (`revision2_figures.py` item 1), the parse 39 to 53 %
of it; by that rate the four presets not profiled read in 4.6–6.9 (`adwaita`, 169,338 B), 9.1–13.6 (`kde-breeze`,
334,037 B), 13.3–19.9 (`ios-13`) and 13.7–20.4 ms (`ios-26`): an extrapolation.

Sizes at thirty widgets are known only as [pkg]'s projection (`d6-draft.md:1417-1427`, made before outcome 2
shrinks the Apple presets): factors 4.01 / 4.10 / 6.61 / 6.65 on today's files, from sizes rounded to 0.1 MB (so
about ±5 % for the small presets). Every "at thirty widgets" figure below multiplies by them, assumes the cost grows
with the bytes, and is an expectation.

---

## 2. The budget

### 2.1 The uses and the two rules

Three uses set it. **Each frame**, an immediate-mode connector draws every part (egui-widgets does, 1.2). **Between
two frames**, something changes while the user watches (the window loses focus, the system's scheme or accent
changes): one change, one resolve, every part asked again. **Once**, at start-up or at a switch to a preset, the
preset loads and the system is read.

Units: a frame at 60 Hz is 16.7 ms, at 120 Hz 8.3 ms; a 1920 × 1080 RGBA surface is 8,294,400 bytes, 7.9 MiB. The
reference window has 300 parts, the figure [D]:172-174 uses: **an assumption**, the maintainer's to change; lines 7
and 8 scale with it.

The rules, **each a judgment**: *per frame*, the theme takes under a tenth of a 120 Hz frame; *between frames*, the
whole sequence fits one 60 Hz frame on the measuring machine (a machine six times slower then stays near 100 ms,
the usual limit for a response felt as immediate: general knowledge, not from the project's sources); *once*, the
load alone fits one 60 Hz frame. How a rule's total is split among its lines is a further judgment: other splits
satisfy the same rule.

### 2.2 The lines

| # | Line (per preset, release build) | Budget | What sets it | Today, one widget | 0.7.0, the same call | Given up against 0.7.0 |
|---|---|--:|---|---|---|---|
| 1 | `Theme::from_preset`, a preset's first load in the process | 16 ms | the *once* rule | 259–17,605 ms base; at a release, no table pending, 128 / 3341 / 502 / 9822 ms ([P] §5's three phases, [mine] sum) | 0.457–0.768 ms (first `Theme::preset`) | 21–35 × |
| 2 | peak memory during that load, above the level before it | 16 MiB | twice line 3, transiently | 18.0–1827.0 MiB (1.1, `peak_alloc`), 1.1 to 114 times the line | 0.5–0.8 MiB (`VmHWM` less the process's start, over both schemes' load and resolve: the nearest figure measured) | 20–32 × |
| 3 | memory kept: the theme and one window's resolution | 8 MiB | one full-HD surface: the theme weighs no more than one of the window's own buffers | the theme 4.0 / 4.9 / 31.9 / 47.6 MiB ([P] §4); the resolution not measured | under 0.5–0.8 MiB (the same peak; nothing kept was measured) | 10–16 × or more |
| 4 | a change every application makes: a reader's re-read accepted, `pick`, `set_flavor`, `set_contrast`, `set_accent`, a bundled palette applied or removed, `set_preset` to a preset a live theme holds | 0.5 ms | *between frames*: 0.5 + 1 + 300 × 0.05 = 16.5 ms ≤ 16.7 | not measured; 101.6–6542.2 ms by 1.2's inference | a scheme switch was a `resolve(mode)`, 0.020–0.023 ms | – |
| 5 | `Theme::resolve`, something changed | 1 ms | the same sum | 0.23–1.47 ms | 0.020–0.023 ms | 43–50 × |
| 6 | `Theme::resolve`, nothing changed | 10 µs | *per frame*: 0.01 + 300 × 0.002 = 0.61 ms, 7.3 % of 8.3 ms | 0.23–1.47 ms (no cache) | as line 5 | – |
| 7 | a styled query, computed | 50 µs | the sum of line 4: 300 of them are 15 ms | 32–53 µs ([P]); 16–103 µs on the loaded machine ([L]) | none; a field read 0.7 ns | not like for like (71,000 × a field read) |
| 8 | a styled query, repeated in one resolution | 2 µs | the sum of line 6 | 32–53 µs (nothing kept) | a field read 0.7 ns | about 2,900 × |
| 9 | bytes a preset adds to a binary | 256 KiB | between two facts, below | 169,338–1,759,947 bytes; 4.73 MB the eight | 25,902–62,653 bytes; 308,299 the seven | 4.2–10.1 × a preset; 6.8 × for the eight (2 MiB) |
| 10 | `Theme::from_system`, the theme's own part (the read of the preset and the acceptance of the reading; the reader's system calls are the system's time) | 16 ms | the *once* rule | not measured (task 4b); 1.2 and 3.5 say what it does | not measured | – |
| 11 | peak memory during `Theme::resolve`, above the level before it | 8 MiB | a resolution between frames needs, transiently, no more than the whole theme may keep (line 3) | not measured (the gate's "about 0.9 GB a resolution being built", `tests/facts_gate/run.rs:3789`, is not resolve's: a resolve takes 1.16–1.47 ms, and 0.9 GB in that time would be 612–776 GB/s, r3f 8, beyond any memory's bandwidth (general knowledge); most likely it is the vector's validation, whose check peaks at 1827 MiB on that preset, [P] §4; not verified) | not measured | – |
| 12 | an application layer that states structure (an expression, `absent`, `auto`, a rule on a part, a presence): the cost of adding it | a figure reported, not a budget line | no figure exists; a line set at the calibration if the maintainer wants one | – | – | – |

### 2.3 Which numbers are judgments, and who they bind

Every number of the table is a judgment: lines 1 and 10 rest on the *once* rule, lines 4 to 8 on the two other
rules, their split and the 300 parts, lines 2, 3, 9 and 11 on a yardstick. Only the arithmetic is derived. Row 12
is a figure the probe reports, not a line: the budget has eleven lines. Line 9
lies between two facts: below it the only indicator of what the data needs, 18–73 kB a preset by zlib today
(1.3), 78–484 kB at thirty widgets for the four profiled ([mine]: zlib × the factors, `revision2_figures.py` item
8); above it, every binary carries all eight presets on every platform. 256 KiB a preset is 2 MiB for the eight, 6.8
times 0.7.0's seven: the least defensible number of the eleven, and already passed by the text of six presets at one
widget (`material-3`, `kde-breeze`, `ios-13`, `ios-26` and the two macOS ones are over 262,144 bytes; `adwaita` and
`windows-11` under; 1.3).

Against the toolkits' uses (1.2): gpui, iced and egui convert a resolution into their own objects when it changes,
so the *between frames* lines (4, 5, 7) and the *once* lines (1, 10) bind them, and lines 6 and 8 do not.
egui-widgets reads the theme in every `ui()`; if 0.8.0's egui-widgets queries every frame, lines 6 and 8 bind it;
if it keeps each resolution's results itself, as the others keep their objects, they bind nothing. The connectors'
design states which (section 4).

What a 0.7.0 user gives up at these numbers: a load 21 to 35 times slower, a resolve 43 to 50 times, a read of a
part's look per frame from a field read to a query held at 2 µs (only where a connector queries every frame), ten
or more times the memory, 6.8 times the bytes; in exchange, the model 0.8.0 adds. The maintainer sets the numbers
knowing this.

### 2.4 How each line is measured and gated

The project's rule for timing tests: on the real clock, assert only what holds under any scheduling (lower bounds,
bounds far below the bug's cost, at-most counts), and still catch a regression (memory
`feedback_fix_before_release.md`); a release prerequisite is a mechanical check (memory
`feedback_release_gates_mechanical.md`). The gating must therefore catch a regression on every armed line, never
fail on scheduling noise, and let a known miss pass only with its cause named. Two gates do it.

**Work, gated in `cargo test`** (every gate mode, CI's three platforms, debug). G1: the runs of the checks over
combinations (`validate::combinations`) a call makes, the differential switch's re-runs not counted (a `#[cfg(test)]`
thread-local counter; validation runs on the calling thread: nothing under `src/validate`, `src/resolve`, `src/theme`
or `src/toml` spawns one, `grep 'thread::spawn\|rayon'`), 0 or at least 1 as task 1 lists. G2: the reads of a preset
the calling thread makes while its own theme holds one, at most 1 (a `#[cfg(test)]` thread-local counter, as G1's:
E1's cache is the process's, so another test thread's theme may hold the read, and only this thread's count and its
own themes' `Arc::ptr_eq` hold under any scheduling). G3: a `resolve` with nothing changed builds nothing and returns
the same handle. G4, if the query cache is built: a repeated query evaluates nothing. They catch the loss of A, B, E1,
E2 or the cache deterministically (one check is 203 to 13,084 times line 4, 101.6 / 0.5 to 6542.2 / 0.5 on the plain
route, r3f 7; the accent layer's check on `windows-11` 32 times, 16.2 / 0.5, r3f 4), and do not see a slower read,
resolve or query. The budget step does.

**The budget step** (`scripts/check_cost_budget.py` over the probe's output), on the measuring machine only, from task
6, in two runs: a **unit run** in each unit's gate ([plan]:1152-1162) and the **release run** in
`scripts/check_release.sh`'s full run; never in `cargo test` or on CI. The two differ only in how they meet load (rule
2) and in what fails (rules 1 and 4). Its file, `scripts/cost_budget.toml`, holds each line's number and, for each
line, preset and case (the route, plain or synthetic, part of the case; line 1 on a preset whose table is pending has
the one case `read`, 3.5), either its **record** (a figure, its tolerance as a fraction, and the ledger row that made
it) or why it has none: "first measured by" a named task, or "not binding" with the ledger row that shows it. A case
with a record is **armed**, and a line is armed where its cases are. The file also holds the **named misses**, each
keyed by one case as its record is, with its cause (any section of this note or any task that removes the miss) and
its ledger row. A figure or a record **meets its line** where it is at most the line, nothing added: the words have
this one meaning in this note. Four rules:

1. **Bound.** A case whose record does not meet its line has a named miss of its own, however close to the line its
   record is; a case that meets its line needs none, and two missing cases of one line and preset need two misses,
   each with its cause. A case has at most one named miss (one miss may name several causes). The step checks this
   from the file alone, with no clock. A named miss without its cause or its ledger row, a second named miss of one
   case, and a named miss keyed by a case the file does not list fail either run, as file defects. With t the case's
   tolerance, a fraction (rule 3), a figure f fails where it exceeds its bound by more than its tolerance, f > bound ×
   (1 + t); the bound is the line, or, under its case's named miss, the case's record where that exceeds the line. A
   miss whose record meets its line (the maintainer raised the line above it) is no error: the line is then its case's
   bound, and the miss stays, printed as one that may go where its figure meets the line (rule 4), until the next
   deliberate act that records the case removes it. A figure over a met line by less than its tolerance is noise at
   that run and passes, printed "over its line, within its tolerance" (*What it gives* says what this leaves). Under a
   named miss whose record exceeds its line, where the record is the bound, a figure with f < record × (1 − t) shows
   the record **stale** (the tolerance is at least twice the spread the record's own runs showed, so a fall past it is
   no noise): the step prints "record stale: re-record <case>", which fails the release run. Every figure is printed
   beside its line and its record.
2. **Figures.** Bytes are counted: a counting allocator (`peak_alloc` 0.3 as a dev-dependency, as `cost-profile`
   2356f84e used it; a hand-written `GlobalAlloc` is `unsafe impl`, against the project's rule) in a test binary of
   its own, run with one test thread (a global allocator counts every thread's allocations, so another test's would
   enter each peak; the profile ran so, `profile_mem.rs:6` at 2356f84e), apart from the times' (its atomics took 7.8 %
   of a profile's samples where one binary held both, [P] *Files*), and the preset files' sizes. A time is the lowest
   of its passes' medians (*Load* below says which passes count), since load only adds time ([L]:6-7); the passes are
   three sweeps over every case, so that a burst shorter than a sweep reaches one pass of a case, not all three; one
   process per pass for lines 1 and 10, a figure under 1 µs timed over batches (100 calls a sample, 201 samples, as
   [L]'s query). The step runs at nice 19 with idle I/O, the priority `lowprio.sh` gives every heavy command on the
   machine and the one [P]'s passes ran at ([P]:7-8); it sets that priority itself, since `lowprio.sh` is a session
   script outside the repository and a run from the maintainer's terminal starts at nice −4 (memory
   `feedback_resource_limits.md`). On a machine quiet enough to compare, a single-threaded pass finds an idle thread,
   so the priority costs its figure nothing, and records and figures are taken alike. *Load*: while the passes run,
   the probe samples, in a thread of its own, every 100 ms, the runnable count (the first number of `/proc/loadavg`'s
   fourth field, the threads runnable at that instant). It prints, for each case's pass, beside the pass's figure, the
   mean of the samples from the last one before the pass to the first one after it (lines 1 and 10, one process per
   pass, sample in that process). A pass counts where that mean is at most 10. The limit includes the step's own
   threads, the sampler and the pass's: on 32 hardware threads those two make no difference that matters, and
   subtracting nothing is simplest. A time counts only where at least two of its passes count (of its three, or, in
   the release run, of those and the further sweeps'), and is then the lowest median of those passes: one pass alone
   is no lowest of passes, and a single figure errs toward a false failure.
   - The release run, after the three sweeps, sweeps again over the cases whose time has fewer than two passes that
     count, until each has two. It starts no sweep later than 30 minutes after the build's end; a sweep already
     running finishes and is judged. A time with fewer than two when the last sweep ends fails with "machine busy,
     run again", which is no verdict on the figures (the bytes are still compared and printed).
   - A unit run makes the three sweeps and no more, and prints "not compared" for each time with fewer than two
     passes that count. The bytes are compared either way.
3. **Tolerance.** A fraction, as the file holds it (rule 1 applies it). For a measured figure, twice the spread (the
   highest figure over the lowest, less one) of the runs that made the case's record, and at least 5 %: five runs for
   a time, three for bytes, each measured as the release run measures (rule 2). A figure counted exactly, line 9's
   file sizes, has a tolerance of 0. The record is the lowest of those runs' figures. A re-record of a case under a
   named miss keeps the old record where the new runs' lowest figure lies within the old record's tolerance,
   r × (1 − t) ≤ lowest ≤ r × (1 + t) (the limits rule 1's failure and stale test leave passing); only a record that
   changes needs a cause (*Who makes records*). A case whose runs spread more than 10 % is a defect of the probe (too
   few samples, too short a batch), which the task making the record fixes before it records, so no tolerance
   exceeds 20 %.
4. **The release.** The release's full run passes only where its release run passed on the release commit (after a
   "machine busy", the step alone, run again on that commit, completes the full run) and every case is armed or not
   binding. The file defines the cases: the step asks the probe, by name, for every armed case (each armed time at
   each of its sweeps, each armed bytes case once a run). The probe's own list, of the cases it can measure, serves
   one check and no other: the file lists every case the probe can measure, as armed, as "first measured by" a named
   task, or as not binding, and a case the probe can measure that the file does not list fails either run as
   "unlisted case", a file defect. A case asked for and not reported fails either run as "not reported". A unit run
   names a case with no record in its printout, asks nothing for it and does not compare it, and passes a time it
   printed "not compared". No run writes the file: the step prints, for each named miss whose figure meets its line
   (a miss under a raised line among them, rule 1), that the miss may go, and the release's ledger row holds the
   figures.

*Where the numbers come from.* The load limit: the profile's runs were at a load average of 1.3 to 9.1 on 32 threads
([P]:5-7), and its passes agreed within 0.9 % ([P] §1); at 20 to 86 ([L]:3-5) the lowest of two or three passes
exceeded the quiet figure by 18 to 39 % on loads and by up to 45 % on resolve and 80 % on the query (upper estimates;
r3f 5): over the 5 % floor everywhere, and up to four times the largest tolerance rule 3 allows (20 %), so no time is
measured there. The one-minute load average is not used: it follows e^(−t/60) both ways (the kernel's exponential
average, updated every 5 s; general knowledge), so it lags. Rising from 2 it stays under 10 through 32 jobs for 10 s
or 16 jobs for 40 s (`revision6_load.py` item 3), long enough to raise every case of a sweep; falling, it still counts
a build or a burst that has ended. Hence the runnable count, the only load signal, the load without the lag (the
average also counts threads waiting on I/O), held to the same limit, a judgment until task P's runs. Load before a
pass does not contend with it, and load during it is what its mean measures; its after-effects (caches, a CPU's
temperature and clock, writeback) are not excluded, and task P's runs check them (8.6, c6-1). The count drops the
step's own build's threads as they exit, so nothing is waited out after the build. Not taken: `/proc/pressure/cpu`'s
`avg10`, which lags 10 s and counts only time a thread waited for a CPU, which a burst under the 32 threads does not
cause though it raises the load the limit rests on. Both interfaces are named from general knowledge; neither was read
on the measuring machine for this note, and task P's script reads the field there and fails where it is absent. The
release run's bound, 30 minutes from the build's end, is a judgment: room for another build or test step to end, and
short enough that a release run on a machine busy with a facts-gate run, which takes hours ([plan]:60-62), each
release's unsharded one among them ([plan]:65-68), fails instead of waiting it out. It counts the sweeps' own time,
which no source gives: task P records how long one sweep takes on a quiet machine, and where three take more than half
the bound, the bound is raised, with its ledger row, until they take at most half of it. The 5 % floor is a judgment
until task P's runs measure the spread: about five times the quiet passes' 0.9 %, and ten times the variation hash
order gave a count of calls between two loads (under 0.5 %, [P] *How it was measured*; for bytes the only evidence is
one peak, `macos-tahoe`'s 1828.9 MiB above start, equal in both schemes of one process, [P] §4).

*Who makes records.* No run of the step writes the file. A record changes only by a deliberate act from rule 3's
runs, and a named miss only by a deliberate act, each with its ledger row:

- a task that changes a cost, for each case whose figure its change may move by more than its tolerance, judged
  before the change is written, its *Done* saying so: task P first records every case of lines 1 to 9 and 11; task 1
  every case of lines 1 to 4 (A removes line 1's check and line 2's peak of it, E1 changes what line 3's theme holds,
  B line 4); task 2 lines 1 to 4 (B's comparison, at a change and at a load over the derived layers, and the
  flattening of a colour; its compared form may be a copy kept beside each layer, 3.3, which a load builds and the
  theme keeps); task 3 lines 3, 5, 6 and 11 (the resolution kept; it changes no mutation, which gives a new handle
  through the comparison `resolve` already makes, section 4, so not line 4); task 4b first records line 10 and line
  4's re-reads; task 5 lines 3, 5, 7, 8 and 11 (the cache inside the resolution); task 7 lines 4 and 10 (every
  `Fact` read through the recording module, a reading accepted by membership; with A, no load runs the checks over
  combinations); under a veto of A (section 5) a load runs the preset's check again, so task 7 records lines 1 and 2
  too; a part a trigger brings, the lines it changes;
- the trigger step, on the presets the staging task regenerated, whose records replace task P's on them;
- the calibration after M3a's first unit;
- M2's fourth task, which bundles the palettes ([plan]:1056-1061), for line 4's case of a bundled palette applied
  or removed, "first measured by" that task until then;
- the task that empties a preset's table (M2's task 0 for the small ones, [plan]:996-1000), for the plain route's
  cases, "first measured by" it until then, removing the synthetic route's and line 1's `read`;
- a unit whose own change grows a named miss past its record (a widget unit enlarging the macOS presets), which
  records the case anew with that cause;
- a case printed stale (rule 1) that none of these acts re-recorded, re-recorded with the fall's cause, which the
  release run requires;
- the release task, which reads the ledger: a case without a named miss printed "over its line, within its tolerance"
  at two consecutive releases gets a named miss with its cause, added by that task. This is the release task's
  procedure, not a rule of the step. The case's record is unchanged, so its line stays its bound (rule 1) until an act
  that records the case keeps the miss (a record over the line) or removes it.

An act that records a case over its line names the case's miss; one whose record meets the line removes it; one
that raises a named-miss case's record states the rise and its cause in its ledger row, and one that keeps it (a
re-record within its tolerance, rule 3) states none. A case whose spread stays over 10 % is the recording task's
probe defect (rule 3), not a state of the file. Every line is armed from its first record: lines 1 to 9 and 11 from
task P's, line 10 and line 4's re-reads from task 4b's, line 4's palette case from M2's fourth task's. Line 8 is not
binding where the connectors' design shows no connector querying every frame and the query cache is therefore not
built (2.3, section 4); line 6 is armed even where 2.3 finds no connector needing it, since E2, its mechanism, is
built whatever the connectors do (task 3) and the line then guards it. The maintainer's change of a line is an edit
of the file with its ledger row, which the next run that compares the case applies (at the latest the release run).

*What it gives.* A regression past a bound fails at the first run that compares it, at the latest at the release,
whose release run must compare every armed case. No time is compared under load (rule 2), and each tolerance is at
least twice the spread measured without it. A case whose record is over its line passes only under its own named miss
naming its cause, and only up to its record and its tolerance; only a deliberate act moves the record, so a miss
cannot grow by its tolerance run after run; nor can such a record stay above a cost that fell, since the release run
fails on it as stale. What it does not do: fail a figure that grows while under its line, which is inside the budget
by the line's definition; the figures printed beside each record, kept in each release's ledger row, show that growth.
Nor does the step name a measured figure over a met line by less than its tolerance: one run cannot tell it from
noise, so every run passes it, printed "over its line, within its tolerance". A met line therefore stays exceedable
within its tolerance, at most 20 % (rule 3), without a name from the step, permanently and by design: at a tolerance
of 20 %, line 1's 16 ms may read 19.2 ms at every run. The printed mark goes into the release's ledger row, and a case
printed so at two consecutive releases gets a named miss from the release task (*Who makes records*; 8.7, c7-3). A
unit run on a busy machine passes with its times not compared, so a passing unit gate says nothing of them; the
release run compares them all.

*The options weighed* (revisions 2 and 3, c2-7, c2-8, c3-2, c3-10): *reported only* fails no regression of the read, a
resolve or a query (c2-8); *a bound at k × the line* leaves a named miss unbounded and lets a met line pass k times
over; *a ratchet at 5 × the record under a load of at most 86*, revision 3's, bound no release to its line (c3-2) and
needed a stale-record bound, a reported-only state for wide spreads and a record check from the file to close its gaps
(c3-10); *the release writing the records*, revision 4's, let a named miss grow by its tolerance at every passing run
(c4-2, 8.4); *a unit run that waits*, revision 5's, which fails unit gates on a machine other projects build on, where
unit gates run often, while the release run compares every armed case anyway (decision Q); *the one-minute load
average*, alone or beside the runnable count (revision 6's), which lags, needs a settle time after the step's own
build at every unit gate and misses a burst over all three passes of a case (8.6, c6-1). Chosen: the line as the
bound, a named miss's record over its line as its case's bound and the release failing on a stale one, times compared
only on a quiet machine, the release run sweeping again until it has one, and records moved only by deliberate acts.
*What would change it*: the maintainer asking that growth under a line fail too, which makes every armed line's bound
the smaller of its line and its record; a quiet machine proving rare at unit gates, which would bring back a coarse
bound for loaded runs.

The step needs a release build of `native-theme`'s `timings` test target and of the crate's ignored cost tests (as
[L]'s commands made), whose build time task P records; it builds them first, and nothing is left to wait out
(rule 2).

---

## 3. The load path

### 3.1 What the measured figures make necessary

| The line | The measured figure that breaks it | So |
|---|---|---|
| 1 (16 ms) | the preset's own check, 105.5–6450.9 ms; it checks the same text at every load | the check cannot run at load: **A** |
| 4 (0.5 ms) | every mutation re-runs the checks with nothing reused (1.2), and the accent layer's check alone, a layer stating only inputs, is 16.2–3299.1 ms | a change that leaves the compared stack as it was runs no check over combinations: **B** |
| 6 (10 µs) | resolve rebuilds at every call, 0.23–1.47 ms | the resolution is kept: **E2** |
| 8 (2 µs) | 32–53 µs at every query | a kept result, **if** a connector queries every frame and the query cannot be made fast enough: task 4a decides |
| 10 (16 ms) | after A and B, a structural reading runs the checks once (1.2: the reader's layer flattens to its statements) | measured by task 4b; **the readings proven at build time** if missed (3.4) |
| 1, 2, 3, 9 on the Apple presets | after A: the read 37.1–57.4 ms, its peak 37.1–62.1 MiB, held 28.3–45.4 MiB; the text 0.94–1.76 MB | C, D or building by need, decided at the trigger step, on the presets outcome 2 regenerates (3.6) |
| 5 on `macos-tahoe` | 1.16–1.47 ms | the kept rules by reference or resolution by need, after the resolve profile (3.6) |

### 3.2 The options

**(0) Change nothing.** Gives: no work. Costs: lines 1, 4, 6 and 8 missed by one to four orders of magnitude
(259–17,605 ms against 16; 101.6–6542.2 ms, inferred, against 0.5; 230–1470 µs against 10; 32–53 µs against 2);
line 2 by 1.1 to 114 times. A scheme switch costs what a load's checks cost (1.2). Loading off the UI thread ([D]'s
option B) helps neither the mutations nor the 1.8 GiB peak.

**(1) Trust the bundled presets (A).** `from_preset` and `set_preset` build the stack of a bundled preset whose
`[pending]` table is empty without `validate::check_preset` and without the checks of the layers derived from it.
- *Skipped*: 105.5 / 2305.9 / 342.5 / 6450.9 ms of the preset's own check (1.1) and the derived layers' checks;
  with them the whole peak above the read's (the 1280 MiB vector is the preset check's, [P] §4).
- *What still runs at load*: the read (6.5–57.4 ms) and the missing-input check (0.001–0.004 ms, [P raw]).
- *How the crate knows the text is the validated text*: it is embedded at compile time (`src/presets/mod.rs:35-39`)
  and the build test reads it through the same `PresetId::source()` in the same commit (`src/theme/tests.rs:20-35`);
  the verdict does not depend on the platform (1.2). A digest at load would compare the text with itself. What must
  be added is on the test's side (3.9): its route, `preset_stack` (`src/theme/tests.rs:56-60`), stays the
  validating construction while `from_preset` leaves it for the trusted one (today `from_preset` calls it,
  `src/theme/mod.rs:82-85`, so a design that made that route trusting would leave the test validating nothing,
  c2-3); and the test proves the stack it checked is the stack `from_preset` builds.
- *Breaks*: nothing a user can see. A preset with a non-empty table keeps today's whole path (it is refused either
  way, and no release holds one, [S]:7731).
- *Rests on*: [plan]:146-147 ("S12 stays strict, and S19.1's build test and the facts gate run over the whole model
  at every commit"). It is not the "lenient validation mode" [plan]:160-161 rejects: the same checks, run where the
  data is fixed.
- *Cost*: small; the build test grows by the derived forms (3.9); the facts gate keeps its cost, since it takes the
  validating construction (3.9).

**(2) Check only where the checks' input changed (B).** For each layer above the preset, the checks over
combinations run unless the stack up to it, compared as 3.3 says, equals one that passed. Sound by determinism:
they read nothing but the flattened stack, the selectors and the context, and the inputs only through the cycle
search and a palette's mapping check (1.2), so the same input gives the same findings. Gives: line 4, expected, for
every change that leaves the compared stack as it was. Costs: a comparison per change (3.3). Rests on: 1.2's account
of what the checks read, which task 1's differential switch checks on every `Incremental` stack the crate's unit
tests change: the `stack()` helper's, task 1's cases, and the stacks of the tests that build through the validating
construction and then change them, made `Incremental` after construction (3.9 *The tests' stacks*), among them
`kde-breeze`'s accent-derivation forms, a palette with its KDE port, and the reader's and the port's left-outs.

**(3) Inert statements and closed places** (revision 1's B2). A statement added, removed or replaced at a place no
check can tell apart. As written it was unsound for rules (the check's finding 1, `combos/mod.rs:733-741`) and its
rule (c) missed `Fact` reads at the ends of reference chains (finding 2). Limited to bases, with every `Fact` read
recorded, it is sound; but at run time it needs, per change, the statements at each place across the stack: an
index stored with the preset (a generated artefact, finding 15) or a flattening of the preset once per process
(4.5–213 ms, [P raw] `T_FLATTEN`, over line 1 on `macos-tahoe`). On the readers it serves little: KDE states
literals over the preset's expressions (592 of `kde-breeze`'s 690 colour base statements are expressions,
`preset_counts.py`) and a `Tokens` rule; GNOME's font patches lie under a reference (`adwaita.toml:2904`,
`roles.body = "= token.text.keyed.body"`). Not on the path; its sound core is reused at build time by (10b), where no
run-time index is needed.

**(4) Remove the repeated work inside the checks.** For the cases where the checks run (the build test, the gate, a
structural reading, a layer of line 12): the repeated enumerations, `listed_unstated`'s walk per listed path, the
accent check's other fifteen checks, and the reuse memory extended from the expression steps to every check. Worth
29–42 % of a full validation and most of its peak (revision 1's table: 103.1 / 1346.4 / 578.2 / 6760.3 ms of a
synthetic load), not an order of magnitude ([P] §3: the search is linear in worlds × selectors). Under A it does
not help a first reading at all: the memory a reading would reuse comes from checking the preset, which A no longer
does. Code: `combos/mod.rs` (4065 lines), `outcomes.rs`, `stack.rs`, `checks.rs`; large together. Off the path
(3.6).

**(5) A compact form built at compile time (D).** In a build script: rejected (the model built twice, every user's
and docs.rs's build validating eight presets). Generated beside the presets and committed: removes the parse (39–53
% of the read) by construction; its decode time and size are unknown, so whether it meets line 9 is not derived (by
the zlib indicator the macOS presets at thirty widgets are 305,950 and 483,781 bytes, over 262,144 even compressed,
finding 7); it ties every facts change that moves a preset to a cargo build of a tool and the full checks, commits
binary blobs that do not merge, and duplicates the build test's validation (finding 15). Off the path (3.6).

**(6) Sparse patches (C).** A part's patch holds every block's patch inline (`src/widgets/mod.rs:22-29`), 8.0–15.4 kB
a rule held. Boxing each block's patch in the derive makes a rule cost about what it states. Medium,
compiler-driven; its gain is not derivable from the profile. It does not touch the parse's peak (4.27 / 6.17 / 25.58
/ 48.39 MiB inside `T_TOML_PARSE`, 1.1), so it alone cannot meet line 2 on the larger presets (finding 4). Off the
path (3.6).

**(7) Resolution by need.** *Checking by need* moves refusals from the mutating call to the query, against
[R]:354-356 and [R]:440-441, and a query "runs inside a connector's draw call" ([R]:437-438): rejected. *Building by
need* (a trusted preset's widgets decoded when first asked for, or a part's rules resolved at its first query) makes
the load the non-widget share of the read (0.428 / 0.329 / 0.148 / 0.077 of the four files, `preset_counts.py`) if
the time is proportional to those bytes, which is not known; the first frame asks for most widgets, so it helps an
application of few widgets, and it adds a lazily filled state to the stack, the resolution and the checks. Off the
path (3.6).

**(8) A feature per preset.** Bytes alone: `PresetId` is an enum connectors match in full ([S]:7734-7737), so the
presets left out stay listed and fail with a new error; S16's *Which presets* changes; eight features join the test
matrix. Not needed while line 9 holds for the eight together, or the maintainer wishes it.

**(9) One read per process (E1).** Four ways:
- a `OnceLock` per preset holding the read for the life of the process: a picker visiting the eight keeps all
  eight, 2.63 + 4.14 + 28.3 + 45.4 MiB for the four profiled alone today, 10.6 / 17.0 / 187 / 302 MiB at thirty
  widgets (finding 5), uncounted by line 3;
- **a `Weak` per preset**: the read stays while a theme holds it, and is freed with the last; the cache keeps nothing
  alive beyond the themes, so line 3 counts it as the theme's own; a switch back to a dropped preset reads again,
  which is line 1's cost and no more. **Chosen**;
- only the current preset's: the same as `Weak` for one theme, worse for two;
- for `font_dpi`, a test-checked constant per preset: duplicates a preset value in code. Not needed: `from_system`
  takes the preset from the cache first and holds it across the reader's read, so `font_dpi` reads through it.

A poisoned lock reads without the cache (no panic). Bundled data cannot change in a process, so nothing invalidates.

**(10) For `from_system`'s structural readings**: 3.4 weighs (a) to (f).

**(11) The text compressed at compile time** (a macro that deflates the file into the binary, inflated at first
load): would meet line 9 by zlib's indicator at one widget (17,954–72,749 bytes) for a new dependency and an
inflate on every first load (its rate not measured here). Weighed with D at the trigger of line 9 (3.6).

### 3.3 The choice: A, B, E1, E2

**A. Trust.** As (1). The trusted forms are the stacks the crate builds from bundled data alone: each bundled preset
with an empty `[pending]` table, with the accent layer (whose compared form is empty, so B leaves it out) and, on
`kde-breeze`, the accent-derivation layer in each form `derive_derivation` gives it: for each non-empty set of schemes
at which a layer above gives the accent, with and without the tinting roles (`src/resolve/mod.rs:1186-1193`,
`may_tint` `:1246-1260`), six forms; `material-3`'s forms when M2's fourth task adds its layer; a bundled palette's
source over each preset it serves, under each contrast, when that task bundles palettes (none exists yet, 1.2). One
function lists the forms, each as the stack up to it (the preset, the derived layer, and the context B compares);
B's recognition of a trusted prefix and the build test's item 2 both call it. The list decides only what B may skip:
a form the run time derives and the list lacks is checked, so a short list costs time, never soundness.

*What decides trust.* The id alone, the text being read by the trusted construction itself. Its two functions (3.9)
take a `PresetId` and no text: `Stack::bundled(id, scheme)` and `Stack::set_preset_bundled(id)` read `id.source()`,
the text embedded at compile time (`src/presets/mod.rs:35-39`), through E1's cache, and skip the preset's own check
only where that text's `[pending]` table is empty (a switch, only on an `Incremental` stack). Their only callers
outside the tests are `Theme::from_preset(id)` and `Theme::set_preset(id)`. Every function that takes a text or a
layer validates: `preset_stack` and `synthetic_stack` with any text and any id (the build test's `validates`,
`src/theme/tests.rs:56-60`, and the 21 test calls `preset_stack(None, …)`), `Stack::new` and `Stack::synthetic` as the
facts gate calls them (`src/gate/mod.rs:119-122`, `:169-178`, `:3520`, `:3608`), and `Stack::set_preset`, which
keeps the preset's own check on every stack; the probe's doc-hidden function (task P) builds no stack. Routes that
pair a bundled id with another text exist (`gate::load` takes the id from the text's own `[preset] id`,
`src/gate/mod.rs:115-118`), but all of them validate, so the build test's verdict on `id.source()` is the verdict on what loads. Task 1's controls reach
the trusted construction with a fixture text through a `#[cfg(test)]` twin, `Stack::bundled_text(id, text, scheme)`;
it and `Stack::bundled` share one function private to the module `resolve`, which takes a parsed read (what
`Stack::bundled` gets from E1's cache), not a text; the twin parses its text and calls it. So no code outside that
module and the tests can hand the trusted construction a text: the module's other children, `compose` and
`evaluate` with `evaluate::engine` (`src/resolve/mod.rs:27-28`), see a private item of it but build no `Stack`
(`grep Stack` finds none in their non-test files). Rejected: a crate-internal trusted function taking a text (revision
4's), which only its callers' discipline kept from trusting another text (c4-3); keying trust on `Some(id)` and an
empty table, which would trust any text a test or a tool passes with a bundled id; a digest of the text, which would
compare the embedded text with itself (3.2 (1)).

**B. Check only where the checks' input changed.** After every change S12 holds for the whole stack, as now; what
changes is how that is known. B runs only on an `Incremental` stack, a theme's or a test's made so (3.9); a `Full`
stack, the validating construction's, never compares and never skips.

*What is compared.* A layer's **compared form** is the layer as it is (`Layer`'s own equality, which already holds
`Spelled` and `Shorthands` equal, `src/layer.rs:46-52`, `:122-127`: how a layer was written names a problem's place,
never decides whether there is one) with the `inputs` of its common bases and of each `for_preset` part set aside;
a palette layer keeps its inputs, which the KDE mapping check reads (`combos/mod.rs:3986-3992`, `:4042-4052`). Its
rules are part of it, each with its selector, so a change of a rule's `when` alone is a change (c2-4). The comparison
is written field by field, with no clone: an exhaustive destructuring of `Layer` (`bases`, `for_preset`, `rules`,
`spelled`, `shorthands`, `src/layer.rs:19-31`) and of `Bases` (`colors`, `inputs`, `tokens`, `widgets`,
`src/layer.rs:177-188`), `inputs` skipped but for a palette, each other field by its own equality; inside the crate
`#[non_exhaustive]` does not stop an exhaustive pattern, so a field added breaks the build until the comparison
decides it. A clone to clear the inputs would cost what the profile's clone of a preset's layer and declarations
cost, 0.117–1.049 µs per op against equality's 0.048–0.059 (r3f 1), up to 18 times more on one preset
(`macos-monterey`, 1.049 / 0.059; c3-12). Two stacks up to a layer are **equal** where:
- their preset is the same `PresetId` and their declarations are the same (by `Arc` identity once task 1's stack
  holds the cache's, else by value), and the reader's accent is left out in both or in neither
  (`Context`, `src/validate/mod.rs:57-70`);
- after each leaves out its layers above the preset whose compared form is empty (no colour, token or widget base,
  no `for_preset` part stating anything but inputs, no rule), the two lists of layers, the preset's own layer at
  index 0 among them (a stack's preset layer is set apart from its id: `Stack::set_preset` takes a layer, and tests
  build stacks with `preset: None`, `src/resolve/mod.rs:914-920`, `resolve/tests.rs:26-34`), have the same length
  and, place by place, the same compared form (by `Arc` identity first, else by value), the same kind and the same
  left-out (`Stacked`, `stack.rs:36-44`), as the reuse memory already compares them, `below` starting at the preset
  (`combos/mod.rs:2209-2219`; c3-13);
- the synthetic route's layers, checked at once today (`src/resolve/mod.rs:1428-1446`), are compared as one group.

No flattening is built. After task 2 the compared form also sets aside the value of each stated value the flattening
reads as `Unknown` (colours, then; strings, flags and integers already, `walk.rs:904-921`), keeping its form
(stated, an expression, `absent`, `auto`) and every number, variant and `Nothing` the flattening keeps; how it is
compared without building the flattened form (a walk over the two layers' values, or a copy kept beside each layer
with those values replaced) is task 2's choice, its cost measured by the probe.

*When the checks over combinations run.* For each layer above the preset, in order, the checks over combinations of
that layer (`validate::combinations` over the stack up to it, the KDE palette mapping check included) run unless
- (i) its compared form is empty, or
- (ii) the stack up to it equals a trusted form, or the stack up to some layer of the stack before the change (the
  theme's current one, which passed).

That stack passed wherever it was built and changed only through the stack's own functions. The tests are the
exception: the `stack()` helper builds its stacks unvalidated (`resolve/tests.rs:25`), and tests write a stack's
fields directly (`resolve/mutation_tests.rs:47-58`, `:191`; `resolve/tests.rs:110`). On such a stack's next change B
may skip a layer that never passed; the differential switch, on in those tests, then checks the change on the full
path, and a problem it finds fails the test as a disagreement, so the exception can fail a test but never let a
wrong verdict pass. No current test meets it: the helper's stacks with layers above the preset are only resolved, or
changed over `body(0.2)`, which the full path accepts (check 5, minor 7).

*Always run, at every change, for every layer above the preset* (decision H): its structural checks
(`checks::layer`) and `sets_stated_whole`, as `check_stacked` runs them today (`src/validate/mod.rs:96-99`), since
they read the current preset, its declarations and the layer's left-out (1.2); and the input cycle search over the
stack up to it, as today (`combos/mod.rs:3254-3257`). The search stays where it is, the last of `combos::check`'s
checks (`combos/mod.rs:487`), so every run of the checks over combinations keeps it, the preset's own check among
them (`check_preset` → `combinations`, `src/validate/mod.rs:275`, `:199-208`, which does not go through
`check_stacked`); where B skips a layer's checks over combinations, `check_stacked` runs the search alone over the
stack up to that layer, through a function of `combos` that calls `inputs::input_cycles` (`pub(super)`,
`combos/inputs.rs:223-253`) and reports its cycles as `Checks::input_cycles` does (`combos/mod.rs:3254-3267`).
Moving it out of `combos::check` instead would drop it from the preset's own check, or, moved into `combinations`, let B skip it (c3-4). Once per change, as today: the reader's
state, `listed_unstated` (no work with an empty table, `src/validate/mod.rs:450`), `inputs_missing`, the accent group
and the palette's refusals.

*Why it is sound.* The checks over combinations read the flattened stack, the selectors and the context, and the
inputs only through the cycle search and a palette's mapping check (1.2). The flattened stack is a function of the
layers' compared forms with their kinds and left-outs and of the context: inputs enter no flattened source
(`stack.rs:819-831`, `bases`, flattens colours, tokens and widgets), and a layer's `for_preset` part for another
preset and a rule another preset excludes are left out by the flattening (`stack.rs:745-765`), so comparing them only
adds checks. Equal stacks therefore give the same findings, and a stack equal to one that passed gives none. A layer
whose compared form is empty adds one empty `Tokens` source (1.2) and no rule; leaving it out of the comparison shifts
the places of the layers above it, which the checks use only for order (`combos/mod.rs:733-734`) and to name a
problem's layer, and an empty layer can be named by none; the differential switch checks that claim on every stack a
test builds. After task 2: the values set aside are those the flattening reads as `Unknown`, which no check can read
(task 2's audit, and the flattening itself); their form is kept, so no rule appears or vanishes and no lookup changes
what it composes (`combos/mod.rs:733-741`).

*What it gives* (expected; 3.5 has the figures): `from_preset` and `set_preset` to a trusted preset run no check
over combinations; `pick`, `set_flavor`, `set_contrast` and `set_accent` run none on a theme with no reader, no
palette and no application layer; `set_accent` on `kde-breeze` runs none without a reader (the accent-derivation
layer is in a trusted form, the accent layer's compared form is empty); a reading of inputs alone (Windows'
ordinary one) runs none; a re-read whose compared form is unchanged runs none: an accent alone on GNOME or Windows,
GNOME's scheme switch (its reading is the same; its state changes, which the reader's state check reads) and
macOS's (its reading states both appearances); after task 2 also a re-read that changes only colours, strings,
flags or integers (an accent on KDE or macOS at the same scheme, an icon theme's name). *What it leaves*, each one
run of the checks from the first layer that differs, as today: a structural reading at `from_system`; a re-read that
changes anything else (before task 2 any stated value; after it a number, a variant or the reading's structure: a
font size, KDE's light and dark, which move the results' scheme, a contrast toggle); a change beneath a layer that
states structure (a palette applied or removed beneath it, finding 24 of the first check); a palette applied or
removed over a structural reading, whose left-out it changes (`src/resolve/mod.rs:1168-1178`, c2-12); `set_accent`
on `kde-breeze` with the KDE reader (the reader's left-out follows the accent givers above it, `:1218-1238`, c2-15);
`set_preset` with an application layer that states structure.

*The cost of a change.* The comparison: the preset and every layer the change kept compare by `Arc` identity; a
derived layer (the accent layer's emptiness, the accent-derivation layer, a palette's entry) and a new reading
against the old compare by value. Value equality of a preset's whole layer and its declarations took 0.167 / 0.270 /
0.772 / 1.430 ms, 0.048–0.059 µs per flattened op of the preset (`T_EXPR_MEMORY`, the plain route's accent column,
where the reuse memory compares the preset layer it holds with the stack's; r3f 1; the timer also holds the
declarations' equality and the accent layer's copy, so the rate is an upper estimate): the rate expected for a
reading.
The always-run part: the accent layer's structural checks took 4.0–6.0 µs, a synthetic layer's (3.1 to 3.8 ops)
2.5–8.4 µs (r3f 2), the input cycle search 1.0–6.0 µs a call on a preset and its accent layer (r3f 3), the missing
inputs under 4 µs ([P raw] `T_INPUTS_MISSING`). The 2.5–8.4 µs a layer holds for layers that small: a larger layer's
structural checks grow with it, expected at up to the preset's own rate, 0.55–1.20 µs per op (`T_CHECKS_LAYER_EXEMPT`
÷ `C_OPS`, r3f 2; the preset's checks include completeness checks only a preset has, `src/validate/checks.rs:524`,
`:577`, so this is likely an upper rate). So a change on a theme with no reader is expected at a few tens of µs
(the stack's copy, which after task 1 copies no declarations, and the accent-derivation layer's checks not
measured), and a re-read, or any change on a theme holding a reading, at about 0.60–1.26 µs per op of the reading
for its structural checks and its comparison: 0.5 ms at about 400 to 840 ops (r3f 6). No reader's layer was sized;
the probe and task 4b measure both.

*The differential switch.* A thread-local switch (validation runs on the calling thread, 2.4): where it is on, every
change B accepts with a skip is checked again by the full path (a `Full` stack's validation, 3.9: every layer's
checks over combinations, nothing skipped), and a problem the full path finds is a disagreement. It is on by default
in the crate's own unit tests (the thread-local starts at `cfg!(test)`), where a disagreement fails the test
(test-only code) and the change is refused with the full path's problems, so the tests lose nothing they had. It
has no use outside them: the facts gate takes the full path itself (3.9), and a connector's tests build a theme
whose skips the crate's own tests already cover. The switch's own runs do not count in G1 (2.4), so a "G1 = 0" case
runs with it on; each negative control of task 1 runs with it off as well, so that it tests B's own verdict, not the
full path's. The switch detects a wrong skip only on an `Incremental` stack a unit test changes: the mutation tests'
`stack()` helper builds one (`resolve/tests.rs:26-51`), the tests of 3.9 *The tests' stacks* make theirs one after
construction, and task 1's cases build theirs through the theme or the trusted construction; the cases of 1.2's account (c2-1, c2-4, c2-5, c3-5)
each get a control in task 1 for that reason.

**E1. One read per process**, as (9): a `Weak` per preset, keyed by `PresetId`, to one `Arc` of the read (its
declarations, layer and listed paths). The stack holds that `Arc` (today it holds the declarations by value and
wraps the layer in an `Arc` of its own, `src/resolve/mod.rs:605-606`, `:681`, so a cached `Arc` would drop as soon
as `from_preset` returned, c2-13); a change's copy of the stack then copies no declarations. `set_preset`,
`font_dpi` and `from_system` read through the cache; `from_system` takes the preset's `Arc` before the reader reads
and holds it across the read, so the reader's `font_dpi` (`src/gnome/mod.rs:187`, `src/kde/mod.rs:254`) finds it.

**E2. The last resolution kept per window**, in `WindowState` (section 4).

*What would change the choice.* A check that must read colour values (a contrast check, for one): task 2's
blindness goes and B compares colours. The differential switch finding a stack B skipped and the checks refuse: B is
wrong where it says, and the switch's case becomes a test. The probe showing the structural checks of the layers a
change kept over line 4 on a theme with a reader or an application layer: they run only where the layer, the
preset, its declarations or the layer's left-out changed (the context they read, 1.2), with a control per item.
Task 4b showing every reader's `from_system` within line 10: 3.4's (b) is not built.

### 3.4 `from_system`

`from_system` is the entry of every application that follows the system, at start-up and, through `reload_reader`,
at each system change. After A, the preset costs the read; after B, a reading of inputs alone costs nothing more.
What is left is a structural reading: KDE's always (fonts, results, a `Tokens` rule), GNOME's with a font (always on
a real GNOME session, where gsettings gives `font-name`), macOS's (roles), Windows' in high contrast (a `Colors`
rule). Its compared form is not empty and the stack up to it is new, so its check runs: one flattening and one
`combos::check` of the whole stack with nothing to reuse, since the trusted preset was never checked in the process
(1.2). Expected at the preset check's order on the plain route, a trusted theme's: 101.6 / 331.7 / 6542.2 ms on
`windows-11`, `macos-monterey` and `macos-tahoe` (r3f 7; `material-3` has no reader); not profiled for `kde-breeze`
and `adwaita` ([L]'s whole plain refusals, the read and two checks on a machine at load average 20 to 86, were
705–706 and 832–1376 ms: an indication of the order only). The ways:

- **(a) Change nothing beyond A and B.** Gives line 10 for Windows' ordinary reading, and line 4, where the reading
  is small enough (3.3), for a re-read whose compared form is unchanged. Leaves one check at each structural first
  reading and at each re-read that changes the compared form: KDE at every start and every light-to-dark switch,
  on the maintainer's own desktop.
- **(b) The reader's layer proven at build time.** A reader's keys and value kinds are fixed by its code; only the
  values come from the system. Two forms:
  - *Every shape the reader can produce*, validated at build time: not feasible. The shape depends on the system's
    data, not on the code alone: which fonts kdeglobals sets, the icon theme's size entries, the scheme file's
    groups and options, which gsettings keys are set (1.2's table); the shapes run to hundreds per reader and
    scheme, each a full check.
  - - **The maximal reading, with the places it may leave out proven removable** ("the readings proven at build
    time"). Per reader and own preset, at each scheme and contrast state the reader can report: the reader's build
    function over a fixture that sets every optional value (the readers' tests hold fixtures, `src/kde/tests.rs`,
    `src/gnome/tests.rs`, `src/macos/tests.rs`, `src/windows/tests.rs`) gives the maximal reading; its places are the
    reader's proven places, a list in code beside the build function. The build test validates each trusted form of
    the own preset with the maximal reading over it, records every statement whose `Fact` a check reads (any of the
    four variants, at the end of every reference followed, candidates discarded by S2.2's composition included: the
    check's finding 2, with `Fact`'s payload readable only through the recording lookup, its equality included:
    `Fact`, `Val` and `OpKind` derive `PartialEq` (`stack.rs:75`, `:88`, `:101`), so a `==` or a `contains` on them,
    as at `combos/motion.rs:601`, reads payloads unrecorded; the derive is taken off `Fact` and the recording module
    offers the comparison, recording what it compares, c2-11), and asserts that no statement of the reading is among
    them. For each statement a real reading may leave out, it asserts the place removable: it is a base (a base never
    comes after a rule, so its presence changes no lookup's composition elsewhere, `combos/mod.rs:733-741`), (α) some
    base of the trusted form (not of the reading) states the place at every scheme the statement holds at, and (β)
    every statement at the place, above it or below it, the reading's own included, is a stated literal or a `Fields`
    patch that creates no map entry (not `absent`, `auto`, an expression, an `Append`, a `Touch` or `Clear`, a
    `Shown`; finding 12's question). (α) and (β) at the place itself also cover a lookup that reaches it through a
    reference with a patch kept on it: before and after, that lookup's referent there is a stated base. At run time a
    reading is accepted without the checks over combinations when the stack beneath it is a trusted form, its preset
    is the reader's own, no application layer or palette is in the stack, and its statements are the maximal reading's
    places less only removable ones. Why sound: the stack then differs from a stack the build test passed only in
    values the checks do not read (strings, flags, integers, after task 2 colours, and numbers and variants the
    recording proved unread) and in removable base statements. The checks' path through a stack depends on a value
    only through a `Fact` they read, so values none of them reads lead them along the recorded path whatever they are;
    and a removable statement's absence leaves every lookup the kind it had, at its place and through references, so
    the path is again the recorded one. *Rests on*: the recording being complete (enforced by the payload's privacy,
    its equality included) and task 2's audit. *Cost*: the recording lookup over 1.2's list of `Fact` reads
    (thirty-four lines, c2-11's two among them) and every equality on a payload (`implement-hard`), the proven places
    per reader, the build test's validations (one full check per reader, own preset, scheme and contrast state: about
    a dozen; placed by the plan's limits, perhaps in the gate's per-preset steps, which run in release), a membership
    test at run time. Medium to large. *What it reaches*: every statement of the maximal reading is present in every
    reading with the same preset and scheme unless the system leaves it unset (the KDE results and rule are always
    stated, `src/kde/mod.rs:274`, `:334-344`), so the common readings of all four readers are reached where the build
    test's assertions hold; a statement that fails one is named by the test and removed from the proven list, and a
    reading stating it runs the checks, as today. Which statements pass is not known until the test runs.
- **(c) Closing places by field type.** Close every place of a type whose `Fact` some check reads, so that no record
  is needed: numbers, enums, `Option`s and lists closed; colours and strings open. As the code stands a colour is a
  `Number` (1.2), so it needs task 2 first; and the font sizes are read (1.2), so KDE's and GNOME's fonts stay
  closed whatever is decided. With (3)'s presence rules at run time it would reach macOS's roles and the readings
  of accents and strings, not KDE or GNOME. Folded into (b), where the recording decides per statement instead of
  per type and the presence rules run at build time.
- **(d) A faster check.** The cost is the worlds' enumeration and the per-world lookups with almost no sharing ([P]
  §3); the repairs known (4) are worth 29–42 %, and a different algorithm (a solver over the atoms instead of
  enumerated worlds) has no design yet. It would serve every path the checks still run on (structural readings,
  line 12, the gate's hours, the build test). Its cost and risk cannot be stated before a design. Not chosen; its
  known repairs are off the path with their trigger (3.6).
- **(e) The KDE reader states the scheme inputs alone**, not the results, which the preset's expressions derive:
  the reading would be inputs plus fonts and the rule. It changes S9.3's readings (kdeglobals' results, which can
  differ from the formula where something else applied them, would be lost), and still leaves the fonts and the rule
  structural. Rejected.
- **(f) Validate the new reading off the UI thread**, the old reading in force meanwhile: an API change (a prepared
  reading applied later), the same CPU spent, and the re-theme delayed by the check's time. Rejected as the design;
  an application can already call `from_system_async` before its first frame.

**Chosen: (b), the readings proven at build time**, with (a) until it is built. It is the only way that reaches the
first reading on every reader with a bounded cost, it reuses what (3)'s sound core needs anyway, and it runs nothing
new at run time but a membership test. It changes no API, no connector and no data format, so it waits for task 4b's
figure without rework: **built when task 4b shows a reader's first reading over line 10, or a re-read that changes the
compared form over line 4** (for the Apple readers, on their presets as outcome 2's staging leaves them); then it is
task 7, before M3a's first widget unit. *What would change it*: (b)'s build test rejecting most of a reader's
statements (then that reader's readings run the checks, and (4)'s repairs or (d) are what reduce them); 4b showing the
check on `kde-breeze` and `adwaita` within line 10.

### 3.5 What it is expected to reach

All rows after A, B, E1 and E2, from the measured figures of 1.1; every figure but the first row is an expectation.

| | windows-11 | material-3 | macos-monterey | macos-tahoe | basis |
|---|--:|--:|--:|--:|---|
| line 1, `from_preset`, ms (16) | 6.5–7.4 | 8.6–9.3 | 37.1–38.2 | 53.7–57.4 | the read is what is left (1.1, both routes) |
| line 1 at thirty widgets, ms | 26–30 | 35–38 | 245–253 | 357–382 | the read × [pkg]'s factors |
| line 2, peak, MiB (16) | 4.7 | 7.3 | 37.1 | 62.1 | the read's peak (1.1) |
| line 3, kept, MiB (8) | 2.6 + the resolution | 4.1 + the resolution | 28.3 + | 45.4 + | the read's held bytes, which the theme keeps; the resolution not measured |
| line 4, a change of B's kind on a theme with no reader, ms (0.5) | a few hundredths | the same | the same | the same | the accent layer's structural checks 0.004–0.006, the input cycle search 0.001–0.006, missing inputs under 0.004 (r3f 2, 3; [P raw]); the stack's copy and the comparison of the derived layers not measured (3.3) |
| line 4, a re-read whose compared form is unchanged, ms (0.5) | 0.60–1.26 µs per op of the reading | the same | the same | the same | its structural checks at up to the preset's rate and its equality at the preset layer's (r3f 6); 0.5 ms at about 400 to 840 ops; no reading was sized (3.3) |
| line 5, resolve changed, ms (1) | 0.23–0.24 | 0.33–0.39 | 0.72–0.77 | 1.16–1.47 | measured, unchanged by the design |
| line 5 at thirty widgets, ms | 0.92–0.96 | 1.35–1.60 | 4.76–5.09 | 7.71–9.78 | × the factors, if resolve grows with the bytes (its time was not profiled) |
| line 6, resolve unchanged | the record's comparison and a handle's copy | | | | not measured; task 3 |
| lines 7, 8, query, µs (50, 2) | 50–53 | 34–49 | 32–36 | 35–37 | unchanged until task 4a decides |
| line 9, bytes (262,144) | 224,294 | 316,862 | 937,875 | 1,759,947 | the text, unchanged |

`from_system` (line 10, 16 ms), by reader over its own preset:

| reader, preset, reading | expected, ms | basis |
|---|--:|---|
| Windows, `windows-11`, ordinary (inputs alone) | 6.5–7.5 | the read (6.5–7.4) and line 4's small checks |
| Windows, `windows-11`, high contrast (a rule) | 108–109 | the read + one check on the plain route, 101.6 (r3f 7) |
| macOS, `macos-monterey` / `macos-tahoe` | 369–370 / 6596–6600 | the read + one check, 331.7 / 6542.2 (r3f 7) |
| KDE, `kde-breeze` | 9.1–13.6 + one check | the read extrapolated (1.3); the check not profiled |
| GNOME, `adwaita`, with a font / accent only | 4.6–6.9 + one check / 4.6–6.9 | the same |

A re-read (line 4) whose compared form is unchanged runs no check over combinations: an accent alone on GNOME or
Windows, GNOME's and macOS's scheme switch, and after task 2 an accent on KDE or macOS at the same scheme or a
string; its cost is then the row above. A re-read that changes the compared form runs one: before task 2 any value
it states; after it KDE's light and dark, a font size, a contrast toggle. A palette applied or removed over a
structural reading runs one.

So the design is expected to meet lines 1 to 4 at one widget for `windows-11` and `material-3` once their tables are
empty (line 4 for a change on a theme with no reader; for a re-read where the reading is small enough, which the
probe measures), and line 10 for the Windows reader's ordinary reading. The plan promises empty tables after M2's
task 0 only for `adwaita` and `kde-breeze` ([plan]:996-1000; the staging design's §2.7 expects all four small ones);
until a preset's table is empty its line 1 is the probe's timed read (task P; the case `read`, a lower bound) and its
other lines are the synthetic theme's, labelled so. It misses lines 1, 2, 3 and 9 on the Apple presets today, line 9 on the
six presets whose text passes 256 KiB, line 5 on `macos-tahoe`, line 10 for every structural reading, and lines 7
and 8 until 4a decides. Outcome 2 shrinks the Apple presets by an amount not known from the sources; the triggers of
3.6 are evaluated on the presets it leaves.

### 3.6 What waits, and the trigger that brings each back

A trigger is a measured miss of a budget line, stated as the line and the measurement. **Where it is evaluated**
(decision I): lines 1 and 9 are already missed by measurement on the macOS presets (after A line 1 is at least the
read, 37.1–57.4 ms against 16; their texts are 937,875 and 1,759,947 bytes against 262,144; 1.1), and outcome 1
puts the repair before M3a's first widget unit (`d6-outcome.md` row 1, on the plan's rule "if the slice already
misses it, a task before M3a (…) costs less than one after it", [plan]:569-578). Outcome 2 shrinks the presets by
an amount the sources do not give, so the triggers wait for it, and no longer: they are evaluated at **the trigger
step**, an M2 step this note defines (section 5), run once outcome 2's staging task has merged. That task runs "after
the spec batch that states the rule and before M2's task 0", its gate the generators' `--check`, the checkers'
self-tests, each regenerated preset loading in Rust, and the facts gate green with the eight presets regenerated (the
staging design, §2.7); neither §2.7 nor its plan change P-6 (the staging design, §6.4) holds a probe step, so the
trigger step is this note's, and the plan's text gains it (3.8). There the probe (task P) runs over all eight
presets, release, on the measuring machine, and measures per preset: the read's time (line 1 after A, which leaves
the read and a construction without checks; the probe's doc-hidden function times the read apart), its peak above
the level before it (line 2) and the bytes it holds (line 3's theme part), the held bytes per rule (C's trigger), the
embedded bytes (line 9), `resolve` changed (line 5) and its peak (line 11), and the computed query (line 7), on the
plain route where the table is empty, else on the synthetic route, labelled so. Lines 4, 6, 8 and 10 need tasks 1 to
5 and are evaluated with them (section 5); where line 5, 7 or 11 is missed, its trigger also needs task 4a's profile,
which comes before the connectors' design is checked, so that decision is taken there, still in M2. Each trigger's
outcome goes in the ledger; **a part whose trigger fires is built before M3a's first widget unit**. The calibration after that unit (window and
chrome, [plan]:74-77) is a second look on that unit's figures; a part that fires there is built before M3a's
second unit. The projection to thirty widgets (1.3) is printed beside each figure and fires nothing.

| Part | Trigger | What it would do, and what it needs first |
|---|---|---|
| C, sparse patches | line 3 missed in the probe's kept bytes, or line 2 after D, on a preset whose held bytes per rule (task P reports them for all eight) are the cause | as (6); its gain measured on one preset before the derive changes |
| D, the compact form (or (11), the text compressed) | line 1 or line 9 missed in the probe | a spike first: decode time and size on `windows-11` and `material-3`; if D proceeds, an existing serde format weighed against hand-written encoders, a fixed byte order, and who regenerates it and when (finding 15) |
| building by need | line 1 missed after D, or D's spike decoding no faster than the text's read | as (7) |
| the kept rules held by reference | line 5 or line 11 missed in the probe, and task 4a's resolve profile shows the clones of `src/resolve/mod.rs:2083`, `:2090` are the cost | `Resolution`'s kept rules shared, not cloned |
| resolution by need | line 5 or line 11 missed after the rules are held by reference | a part's rules resolved at its first query, kept per resolution |
| the readings proven at build time, 3.4 (b), task 7 | in task 4b, line 10 missed by a reader's first reading, or line 4 by a re-read that changes the compared form (KDE's light-to-dark switch on the maintainer's desktop, 3.4 (a); a font size; a contrast toggle) | as 3.4 (b); it needs task 2 first |
| the repairs inside the checks, (4) | the plan's gate limits reached ([plan]:70-73); line 12 set at the calibration and missed; or, after task 7, a reader's statements left on the checks with line 10 or line 4 still missed | as (4), the reuse memory to every check first |
| a faster query | line 7 missed in the probe (`windows-11` at 50–53 µs today, 1.1) | the repair task 4a's profile names (section 4), an `implement` task |
| the query cache, task 5 | the connectors' design shows a connector querying every frame and task 4a's profile no repair bringing the query near line 8 (section 4) | task 5; otherwise line 8 is not binding (2.4), its reason that design's ledger row |
| a feature per preset | line 9 missed for the eight together after D, or the maintainer's wish | as (8) |

Line 12, an application layer that states structure: it runs the checks once when added, and again whenever
something beneath it changes (a palette, a reader's compared form, the preset). The probe reports it on reference
layers from task P; a line for it is set at the calibration if the maintainer wants one. Until then such a layer is
added before the first frame.

### 3.7 The API and when an error can arrive

| Call | Errors today | After |
|---|---|---|
| `Theme::from_preset` | `ThemeError::Preset`: a validation problem, or a listed path ([S]:7067-7071) | the same, by today's path, for a non-empty `[pending]` table; for an empty table none: the build test reads and validates the same text |
| `Theme::from_system` | the above; the reader's failures; `ReaderFailure::Invalid` for a reading S12 refuses (`src/theme/mod.rs:66-77`) | the same; a reading S12 refuses is refused as today, since B runs the checks over combinations for every reading whose compared stack differs from one that passed, and every layer's structural checks and input cycle search at every change |
| `set_preset` | `LayerError` for the preset or the layers over it (`src/theme/mod.rs:110-131`) | for the layers over it only, where the new preset's table is empty |
| every other mutation | `LayerError`, the theme as it was | unchanged |
| `resolve`, every query | none ([R]:440-441) | none |

Signatures that change: `Resolution.theme` becomes `Arc<ResolvedTheme>` (task 3); the element queries' results
become shared only if the query cache is built (task 5). `Theme::from_preset` stays fallible: the pending refusal
exists between releases, and the rule against panics leaves only an error to return where a read could fail.

### 3.8 The rules

Each sentence that changes, re-quoted, with its new wording and its class by the plan's *Spec changes*
([plan]:1376-1388: class 2, a model change, an S22 line and a checked batch, `:1383-1385`; class 3, a rationale
decision, the maintainer's, `:1386`).

- **R2.8**, [R]:437-441, class 3. Today: "Resolution checks every rule's derivations against every base and rule
  they can apply over … A preset or layer that fails is an error when it is added, and it is refused (R2.5); so
  resolution and the query cannot fail." Added after it: *A bundled preset whose `[pending]` table is empty (every
  one at a release), a bundled palette, and every layer `Theme` derives from them alone pass these checks in the
  crate's own tests at every commit (S19.1's build test and the facts gate), and a theme does not run them again when
  it loads them. What the system and the application add passes them when it is added: they run over every layer
  whose stack, as they read it, the addition changes, and are not run where nothing they read differs from a stack
  that passed them; the checks of each value against the declarations, and the check that no input reads itself
  over the stack up to each layer, run over every layer at every change.* The
  decision (queries that cannot fail, layers refused when added) stands; the guarantee is the same, stated for each
  kind of data (finding 18's last item). Without the sentence, R2.8's "a preset … that fails is an error when it is
  added" would no longer describe a bundled preset (c2-14).
- **S8**, [S]:4376-4384, class 2. Today: "**Validation** (S12) runs on every change of the stack (S16): it validates
  the whole stack, conservatively (…), and a change that fails is refused whole, leaving the theme as it was (R2.5).
  So resolution and the query cannot fail." To: "**Validation** (S12) holds for the whole stack after every change of
  it (S16), conservatively (…, the parenthesis unchanged): a change after which it would not hold is refused whole,
  leaving the theme as it was (R2.5). Each layer's own checks, and the input cycle check over the stack up to it, run
  at every change; the checks over combinations run over each layer whose stack, as they read it, the change altered
  (S12's *What a change re-checks*); a bundled
  preset whose `[pending]` table is empty and the layers `Theme` derives from it alone pass them in the crate's own
  tests (S19.1) and are not checked again. So resolution and the query cannot fail."
- **S8**, [S]:4386-4392, class 2. Today: "…with `Resolution { theme: ResolvedTheme, change: Change, suspends: bool }`
  …, which `WindowState` records". To: `theme: Arc<ResolvedTheme>`, and after "which `WindowState` records": "and it
  keeps the resolution it returned: for `Unchanged`, `resolve` returns that one again without building one."
- **S9.2**, [S]:4722-4724, class 2. Today: "…S12 validates it, as the S8 leave-out, at every slot where it can state
  its roles." To: "…S12 holds for it, as for the S8 leave-out, at every slot where it can state its roles: S19.1's
  build test validates each form `Theme` can derive over the bundled preset (each set of schemes at which an accent
  is given, with and without the tinting roles), and a theme does not check it again (S8)."
- **S12**, class 2: a paragraph after *Combinations, conservatively* ([S]:6028-6052), before [S]:6054. "**What a
  change re-checks.** The checks over combinations of a layer read the stack up to it only as its flattened values:
  each layer's bases (its part for the current preset among them) and rules with their selectors, what it is to the
  stack and what S8 leaves out of it, the declarations, the preset and whether the reader's accent is left out. They
  read no input but through the input cycle check and a palette's KDE mapping check (S14); a reference to an input is
  read as stated; a stated string, flag or integer is read as stated. So after a change they run for each layer whose
  stack up to it differs, each layer compared with its inputs set aside (a palette's kept) and the layers stating
  nothing else left out, from each form S19.1's build test validates and from the stack, as it was before the change,
  up to each of its layers. At every change, every layer's own checks (each value, patch, rule and reference against
  the current preset's declarations) and the input cycle check over the stack up to it run, and so do the reader's
  state, the listed paths and the missing inputs. S19.1's build test and the facts gate run every check of every
  layer, nothing skipped." With task 2, a second class-2 line adds "a stated colour" to the
  values read as stated, and sets aside in the comparison the values read as stated. With task 7, a third adds the
  readings proven at build time.
- **S16**, [S]:6375, class 2. Today: "Every mutation validates the whole stack and is refused whole on failure, the
  theme staying as it was (S8, S12)". To: "After every mutation the whole stack passes S12, or the mutation is
  refused whole, the theme staying as it was (S8, S12)".
- **S17**, [S]:7134, class 2. Today: "the preset is regenerated with the measured value stated and validated with it
  (through `from_preset` once its table is empty)". To: "… validated with it (through S19.1's build test once its
  table is empty)", the rest unchanged: under R2.8's new sentence `from_preset` checks nothing (c4-8).
- **S19.1**, [S]:7674-7680, class 2, two changes. After "…so `Theme::from_preset` and `Palette::bundled` never meet
  their error in a release (S16).": "A theme does not check them again (R2.8, S8), so the test also validates each
  form of the layers `Theme` derives from bundled data alone (S9.2's accent-derivation layer for each set of schemes
  at which an accent is given, with and without the tinting roles; each bundled palette's source over each preset it
  serves, under each contrast condition, once palettes are bundled), and asserts that the stack `Theme::from_preset`
  builds without the checks equals the stack the validating construction builds from the same text." And the next
  sentence ([S]:7677), today "The test validates each bundled preset through `from_preset` where its `[pending]`
  table is empty, and through `from_preset_synthetic` …", becomes "The test validates each bundled preset through a
  validating construction of its own, which runs every check `Theme::from_preset` no longer runs, where its
  `[pending]` table is empty, and through `from_preset_synthetic` …", the rest unchanged (c2-3): under R2.8's new
  sentence `from_preset`'s own route checks nothing, and a test through it would validate nothing.
- **S8's `Scope`**, [S]:4413-4418 ("A `Scope` holds owned, evaluated values only"): changes only with the query cache
  (task 5), as a class-2 line of that task, not now.
- **S11**: unchanged (D is off the path).

The code's doc comments that state the old rule change in task 1: `src/theme/mod.rs:1-3` ("Every mutation validates
the whole stack"), `:40-45` and `:79-81` (`from_preset`'s and `from_system`'s "a bundled preset failing validation is
`Err(ThemeError::Preset)`"), `src/resolve/mod.rs:596-600` ("Every change validates the whole stack and is refused
whole"), `:817-818`, `:1404-1411`; the build test's own, `src/theme/tests.rs:20-24` ("through `from_preset`
where its `[pending]` table is empty") and `:50-55` ("the plain one (`from_preset`'s)"), which name `from_preset` as
the route of a test it no longer shares (c2-3); and the statements of the old rule to users (c3-7), the whole
repository being the product: the crate's documentation, `src/lib.rs:9-11` ("every change validates the whole stack
and is refused whole"); `native-theme/README.md:12-15` ("every layer is validated over the whole stack when it is
added"), `:88-90` ("`add_layer` validates the layer over the whole stack and refuses it whole, naming every problem,
where it does not hold"; c4-8) and `:112-116` ("Every change validates the whole stack and is refused whole"); the
workspace's `README.md:72` ("Every change validates the whole stack and is refused whole where it does not hold");
and `CHANGELOG.md:30`, under 0.8.0, unreleased (`CHANGELOG.md:8`: the layer calls "each validated over the whole
stack and refused whole"). Each is worded as S16's new sentence: after every change the whole stack passes S12, or
the change is refused whole, the theme staying as it was (for `:88-90`: after `add_layer` the whole stack passes
S12, or the layer is refused whole, naming every problem). The line-joined scans find these and nothing else of the
rule: `load-check4/wrapped_rule.py` over the repository but `docs/archive/` and the plan's documents (also
`src/expr/ty.rs:565`, about the checks; `src/resolve/mod.rs:545`, a set stated whole; `resolve/tests.rs:269`, a test
comment on a refusal), and `load-check/revision5_spec_rule.py` over the specification and the rationale (the
sentences of S8, S16, S17 and S19.1 above, and `[S]:4381`, `:7948`, about what a check reads).

The plan (the orchestrator's text): the D6 item ([plan]:569-578) takes the budget; *Global constraints* an entry
beside *The facts gate's cost* ([plan]:60-77) naming the budget step and where it runs; M2 the tasks of section 5 and
the trigger step, after the staging task (the staging design's §2.7, which P-6 places before task 0); M2's task 0 and
fourth task the records 2.4's *Who makes records* gives them; M3's gate per unit ([plan]:1152-1162) the step.

Unchanged, and relied on: [R]:354-356 (a failing application layer refused when added); [R]:432-433 (adding a layer
checks every rule and reference against the declarations: the structural checks always run); [R]:1349 (a missing
value is an error when its layer is added); [R]:374-377 (an accent that does not state the ramp is refused);
[R]:1311-1317 (a palette refused or suspended); [R]:1383-1388 (no whole non-bundled preset, so every stack's bottom is
bundled data); [S]:6054 ("`Theme::add_layer` validates the layer against the theme and refuses it with the error",
true under B: its structural checks always run, and its checks over combinations whenever its compared form is not
empty); S17's refusal of a listed path, and its synthetic route, whose load stays on the full checks and whose
mutations take B, like any theme's ([S]:7067-7074, :7095-7103; c2-16).

### 3.9 The build test and the facts gate

**Two constructions, one knob.** A stack holds its **checking**, `Full` or `Incremental`, set when it is built and
kept by its copies:
- On a **`Full`** stack every validation is **the full path**: today's `validate_reusing`
  (`src/resolve/mod.rs:1420-1470`), every layer's `check_stacked` with its checks over combinations, nothing
  compared, no trusted form consulted, nothing skipped; at construction after the preset's own check, as `Stack::new`
  runs it today (`:669-704`), and at every later change, a `set_preset`'s check of the new preset included
  (`:914-936`).
- On an **`Incremental`** stack a change takes B (3.3).

The **validating construction** is `Stack::new` and `Stack::synthetic` with `Full`: the preset's own check
(`preset_problems` → `check_preset`, which reaches the input cycle search through `combinations` →
`combos::check`, `src/validate/mod.rs:275`, `combos/mod.rs:487`), then the full path. `preset_stack` and
`synthetic_stack` take it, so the build test's `validates` (`src/theme/tests.rs:56-80`) and every test calling
them (1.2) keep today's verdicts, refusals included; so do the facts gate's constructions (`src/gate/mod.rs:119-122`,
`:169-178`, `:3520`, `:3608`), and since its copies stay `Full`, the gate's per-vector `set_flavor` and
`set_reader_stand_in_carrying` (`:380-396`) take the full path too: the accent-derivation layer is checked at every
condition vector, as today, and the gate's verdicts rest on no part of B.

The **trusted construction** is new, two functions that take a `PresetId` and no text (3.3 A, *What decides
trust*):
- `Stack::bundled(id: PresetId, scheme: Scheme) -> Result<Stack, LayerError>`, `Theme::from_preset`'s: it reads
  `id.source()` through E1's cache and, where the read's `[pending]` table is empty, builds the same stack,
  `Incremental`, without the preset's own check; the layers above the preset take B as any change does, which here
  leaves only their always-run part (the accent layer's and an empty accent-derivation layer's compared forms are
  empty, 3.3). Where the table lists a path it builds as the validating construction does, whose own check refuses
  the path (S17).
- `Stack::set_preset_bundled(&mut self, id: PresetId) -> Result<(), LayerError>`, `Theme::set_preset`'s: on an
  `Incremental` stack where the cached read's table is empty, it is `Stack::set_preset` over that read without the
  new preset's own check (the accent group, the palette's source, then the change, whose layers above take B); on a
  `Full` stack, or for a listed table, it is `Stack::set_preset` over that read, the check included.

`Stack::set_preset`, which takes a layer, keeps the preset's own check on every stack whatever its checking, so
`resolve/mutation_tests.rs:100-105` and `:186-189`, which expect `set_preset(None, Declarations::default(),
Layer::default(), Vec::new())` refused as "a preset failing validation" on the `stack()` helper's `Incremental`
stack, keep their expectation unchanged. Task 1's controls reach the trusted construction with a fixture text
through the `#[cfg(test)]` twin `Stack::bundled_text(id: PresetId, text: &str, scheme: Scheme)`.
`Theme::from_preset_synthetic` builds through the validating construction and makes the stack `Incremental`
(`Stack::into_incremental`, which changes the checking alone), so its mutations take B like any theme's (3.8's last
paragraph).

**The tests' stacks** (c4-1). The mutations' test helper `stack()` (`resolve/tests.rs:26-51`) builds `Incremental`
stacks. A test that builds a stack through the validating construction and then changes it makes it `Incremental`
after construction, by the same `into_incremental`, so that B and the differential switch run there (3.3). They are
(`load-check/revision5_mutated.py`):
- in `resolve/tests.rs`, the helper `breeze_at` (`:1051-1061`), so its callers `a_breeze_port_resolves_as_kde_breeze`,
  `a_palette_keeps_the_place_it_was_first_added_at`, `a_palette_s_refusals` and
  `a_port_leaves_the_stand_in_out_but_its_accent` (`:1099-1337`); the closure `synthetic` of
  `accent_derivation_layer_places` (`:634`); `accent_derivation_layer`'s and
  `a_set_stated_whole_beneath_the_derivation`'s `plain` (`:781`, `:930`); `a_palette_s_refusals`' `on_adwaita`
  (`:1201`);
- in `src/theme/tests.rs`, the stacks of `a_listed_path_is_stated_by_a_layer_above` (`:1188`, `:1200`),
  `a_listed_presence_is_stated_by_an_application_layer` (`:1223`, `:1234`, `:1239`),
  `a_synthetic_theme_keeps_its_route_until_set_preset` (`:1294`), `a_listed_input_is_folded_into_the_preset`
  (`:1362`), `carried_inputs_end_with_their_stand_in` (`:1447`) and `a_declared_input_missing_at_a_slot` (`:1497`).

Not changed: the constructions they only refuse or never change (`resolve/tests.rs:246`, `src/theme/tests.rs:1214`,
`:1260`, `:1488`), and the facts gate's stacks and its tests, which stay `Full`. No verdict changes: each construction
keeps its `Full` verdict; every refusal these tests expect after it comes from a check B never skips (the structural
checks and `sets_stated_whole`, `resolve/tests.rs:957`, `:967`; the listed paths and the missing inputs, run once per
change, `src/theme/tests.rs:1197`, `:1508`; `Stack::set_preset`'s own check through the tests' `switch`, `:1162-1165`;
the palette's refusals before any validation, `resolve/tests.rs:1171`, `:1180`, `:1195`, `:1212`) or, for the role and
input cycle of `resolve/tests.rs:882-896`, from the checks over combinations of a layer whose compared stack differs
(a new stand-in; an accent removed, which changes the accent-derivation layer's form and the reader's left-out,
`src/resolve/mod.rs:1186-1200`) and from the input cycle search, which always runs; and a change B accepts with a skip
is checked again by the full path with the switch on. An unsound skip therefore fails the test, which is what the
change is for.

Rejected: a parameter of `validate_reusing` alone, which a change could not pass on (the gate's mutations call
`Stack::change` → `validate()`, `src/resolve/mod.rs:819-841`, `:1412-1414`), so each mutation would need a full twin;
a validating construction that checks only at construction and lets its changes take B, which leaves the gate's
derivation layers to B's recognition of trusted forms (c3-1) and the gate no second proof of them; B without the
trusted forms on the gate's stacks, a third mode for a saving of gate time nobody needs yet (its shards are inside
the plan's limits, [O] outcome 9).

The build test (`src/theme/tests.rs`), for every `PresetId`, in `every_bundled_preset_validates`:

1. the validating construction accepts it (the plain route for an empty table, the synthetic route otherwise),
   through `validates(Some(id), id.source())`;
2. for an empty table, each trusted form the one function of 3.3 A lists passes the full path's check of that
   form's layer over the stack below it (`check_stacked` with the context the form is listed with, nothing skipped,
   no form consulted), taking from `reused` what item 1's check of the preset found, as `Stack::new` does
   (`src/resolve/mod.rs:676-698`);
3. for an empty table, the stack `Theme::from_preset(id, Scheme::Light)` builds equals the validated one, its
   checking apart (`Stack` derives `PartialEq` in tests, `src/resolve/mod.rs:602`), and so does the stack
   `Theme::set_preset(id)` leaves on a theme of another preset whose table is empty, with no layer, so that E1's
   cache and its keying are under the test;
4. with task 7, each reader's maximal reading over its own preset, and the assertions of 3.4 (b).

Items 2 and 3 need a bundled id and run per `PresetId`, not inside `validates`, whose calls with `None` and a fixture
text (`src/theme/tests.rs:126`, `:299`, and the others of 1.2) keep their verdicts (c4-11).

Item 2 never goes through B, so a form is trusted because the full path passed it, not because B recognised it
(c3-1's circle). The build test's controls, in task 1, each run with the differential switch off:
- *negative, item 1*: the build test's per-preset function, given `adwaita`'s id and a text with an
  empty table and one fault only the checks over combinations find (a reference cycle between two tokens,
  `ProblemKind::ExpressionCycle`, which only `combos` reports, `combos/mod.rs:2542`; `src/validate/checks.rs`
  reports no such kind), refuses it with that problem, G1 ≥ 1; and the trusted construction's twin
  `Stack::bundled_text`, given the same id and text, accepts it with G1 = 0, which shows that the test's route is not
  the trusted one (`adwaita` derives no accent-derivation layer, `src/resolve/mod.rs:1870-1881`, so no layer above
  the fixture's preset has a compared form that is not empty; over `kde-breeze`'s id the derivation layer would sit
  on a preset layer no trusted form lists, B would run its checks and find the cycle, c4-12);
- *negative, item 2*: item 2's per-form check, given a listed form with one statement added that only the checks
  over combinations refuse (a token reference cycle in the form's layer), refuses it with `ExpressionCycle`, G1 ≥ 1;
- *negative, the preset's input cycle* (c3-4): a preset text whose inputs read each other in a cycle is refused by
  the validating construction with `ExpressionCycle` at `inputs.<name>`;
- *positive, item 3*: a test-only hook that makes `Stack::bundled` build a different stack (it skips
  `derive_derivation`) makes item 3 fail on `kde-breeze`;
- *item 2's list against the run time*: task 1's G1 = 0 after `set_accent` on `kde-breeze` with no reader (section
  5), which a list lacking the form the run time derives there fails, since B would then run the checks.

Its cost: today's duration is recorded nowhere (1.2). Items 2 and 4 add about a dozen full checks once the tables are
empty (`kde-breeze`'s six forms, each with the preset check's reuse memory; the readers' maximal readings, each a full
check). Task 1 records its time in debug and release; where it would push a CI test job past the plan's shard limit
(3 h, [O] outcome 9) or a local step past its limit (1 h, 12 GB), the added items move into the facts gate's per-preset
steps, which run in release and are cached per preset (finding 14).

The facts gate keeps validating each preset once per process (`src/gate/mod.rs:160-193`) and each condition vector
on the full path, a second proof that does not rest on B; its hours stay what they are (`NT_FACTS_GATE_TIMING=1`
before and after task 1 shows it). *What would change it*: the plan's calibration showing the gate past its limits
([plan]:70-73), when B on the gate's stacks, with the switch on in its release run, is one of the gate-speed task's
options.

`check_release.sh` gains the budget step (2.4) and no drift step.

### 3.10 The code

| Part | Where | Size |
|---|---|---|
| A, B, E1 | `src/theme/mod.rs` (`from_preset` and `set_preset` to the trusted construction; `preset_stack`, `synthetic_stack` `Full`), `src/resolve/mod.rs` (the stack's checking and `into_incremental`; the trusted construction, `bundled`, `set_preset_bundled` and the test twin; in `validate_reusing`, on an `Incremental` stack, the comparison against the trusted forms and the stack before the change; the stack holding the cache's `Arc`), `src/validate/mod.rs` (`check_stacked` split into the always-run part and the checks over combinations, the input cycle search run alone where these are skipped; the thread-local switch and G1), `src/validate/combos/` (the function running the input cycle search alone), `src/presets/mod.rs` (the cache), `src/gate/mod.rs` (its constructions `Full`), `resolve/tests.rs` (`stack()` `Incremental`; the stacks 3.9 *The tests' stacks* names made `Incremental`), `src/theme/tests.rs` (the build test's items; the same) | small to medium: no check written, some not called |
| task 2, colour values | `stack.rs` (`Flatten::number` and a colour's channels), the compared form's setting aside of the values read as `Unknown`, tests | small code; the audit is the work |
| E2 | `src/resolve/mod.rs` (`WindowState`, `Resolution`), `src/theme/mod.rs` | small |
| task 7, the readings proven at build time | `stack.rs` (`Fact` private to a recording lookup, its `PartialEq` with it), `combos/` (1.2's list of reads), the readers (proven places), `src/theme/tests.rs` or the gate | medium to large |
| task 5, the query cache, if built | `src/resolve/mod.rs`, `src/query/mod.rs`, `src/query/theme.rs`, `src/resolve/evaluate.rs` | medium |

---

## 4. Resolution and query caching

**The resolution: in `WindowState`.** `Theme::resolve` takes `&self` and a `&mut WindowState`
(`src/theme/mod.rs:325-327`); a theme could keep per-window state only through interior mutability and a window
identity it does not have (finding 6). `WindowState` already records the last resolution's inputs
(`src/resolve/mod.rs:281-300`) and is the connector's ([S]:4386-4392). So `WindowState` holds the last
`Arc<ResolvedTheme>` beside its record; `resolve` compares as it does (`:1557-1562`) and for `Change::Unchanged`
returns the same handle without building one. No state and no lock go into `Theme`. A connector may call `resolve`
every frame (line 6).

**The query's result: decided by task 4a.** A computed query costs 32–53 µs and was not profiled ([P] §5). Who could
keep a result:

- *the application*: it does not know what a query reads. Rejected;
- *each connector, a cache of element queries*: four implementations of one key, which must cover everything the
  query reads (states, their order, siblings, data), which only the library knows. Rejected for element queries;
- *each connector, its toolkit objects per resolution*, as gpui, iced and egui do today (1.2): right for what a
  connector derives once per resolution; not for an immediate-mode draw that asks per element and state;
- *a cache type of the library the connector owns and passes*: one implementation, threaded through draw code that
  has no place for it (iced's style functions take `&Theme`, `connectors/native-theme-iced/src/styles.rs:200`);
- *the resolution itself*: one implementation, invalidated by the resolution's own lifetime;
- *no cache, a faster query*: unknown until profiled.

Task 4a profiles `ResolvedTheme::style` and `Theme::resolve` with the cost-profile tooling. The cache is built (task
5), inside the resolution, only if the connectors' design shows a connector that queries every frame (egui-widgets'
port, by 0.7.0's precedent) and the profile shows no fix of the query itself brings it near line 8. If built: keyed by
the element atoms its part's reached rules name (so a list's index and siblings enter only where a rule names a
position; finding 17), compared in full, not by hash alone; bounded and emptied when full, with a test of a list
longer than the bound (correct, and its cost reported); a part whose values read a font metric kept with the metric
answers its evaluation read and asked again (`Element.metrics` is a `&dyn FontMetrics`, `src/query/mod.rs:245`); a
poisoned lock computes without the cache. Then `Scope` becomes a cheap handle that carries its resolution's identity,
S8's class-2 line ([S]:4413-4418).

**The connector keeps what it derives** for its toolkit and rebuilds it when `resolve` reports anything but `Unchanged`
([S]:4394-4395); **the application keeps nothing** and calls `Theme`'s mutations outside a draw.

**What the connectors' design must state** (`docs/todo_v0.7.1_model-completeness-connectors-design.md`,
[plan]:1007-1025): where the connector calls `resolve` and that it holds the returned handle; whether it queries every
frame, and if not, which results it keeps per resolution; each toolkit object it derives and that it is rebuilt on every
change but `Unchanged`; that it calls no mutation inside a draw, and which of its calls are of line 4's kind; that its
`FontMetrics` answers from the toolkit's current fonts at every call; a contract-test row: after each kind of change (a
window condition, the scheme, an accent, a layer), a style that depends on it differs and every derived object was
rebuilt.

*What would change it.* A profile showing the computed query under 2 µs: no cache. A connector that must query from
several threads at once and measures the lock: the cache split per thread.

---

## 5. The order of work

M2 begins with outcome 2's staging task, which runs before M2's task 0 (the staging design, §2.7); task 0 brings the
four small presets' tables toward empty, at least `adwaita`'s and `kde-breeze`'s ([plan]:996-1000); the connectors'
design follows ([plan]:1007-1025); M3a's first widget unit is window and chrome ([plan]:74-77, :741-766). Each
task's diff gets its `review` before it merges; none changes a preset's values or a fact. Agents run their long
commands in the foreground and report once.

**The maintainer's numbers.** No step waits for the maintainer. The eleven lines' numbers (line 9 above all, the
least defensible, 2.3), the 300 parts and the R2.8 sentence go to the maintainer with D6's other vetoable outcomes
("Nothing that blocks the work … the load budget's numbers, proposed from the profile", [O]:30-32; "Every one is open
to the maintainer's veto", [O]:5-6). They apply as proposed until the maintainer changes one; a changed line is an
edit of `cost_budget.toml` with its ledger row, the next run of the step that compares the line's cases (at the
latest the release run) compares against it, and the triggers of 3.6 that rest on it are evaluated again at that
run. The R2.8 sentence is class 3 ([plan]:1386), applied as proposed and
open to the veto as D6's three outcomes that change an earlier decision are ([O]:6); a veto removes A (the trusted
construction becomes the validating one in `from_preset` and `set_preset`, one call each) and leaves lines 1 and 2
missed by the preset check's cost and peak, each missing case with a named miss citing this paragraph; task 7
then records lines 1 and 2 too (2.4 *Who makes records*).

**Before the trigger step**

- **Task P, the probe** (`implement`). `native-theme/tests/timings.rs`'s `bundled_preset_costs` (cost-probe 814e96e7,
  the ledger's queue #253) merged and extended: one process per pass for line 1; the bytes in a test binary of its own
  with `peak_alloc` 0.3 as a dev-dependency, run with one test thread (lines 2, 3, 11; 2.4); line 1 on a preset whose
  table is not empty, the case `read`, through a doc-hidden `native_theme::gate` function that times the read of
  `id.source()` (`read_preset`, E1's once task 1 builds it) and returns figures only, never a theme or a stack
  (finding 21 of the first check), while on a preset whose table is empty line 1 is `Theme::from_preset` timed whole.
  `read` is a lower bound: it leaves out the trusted construction's own part (the stack's build, `derive_derivation`,
  the always-run checks), expected at a few tens of µs, as a change on a theme with no reader (3.3). The function
  builds no stack, so only `from_preset` and `set_preset` take the trusted construction (3.3 A; c4-9, c5-9). Lines 4
  to 8 on the plain theme where a preset's table is empty, else on the synthetic theme, labelled so (its 48 to 312
  extra layers, [P raw] `C_LAYERS`); the embedded bytes; the held bytes per rule for all eight; reference layers for
  line 12; the build first, then the sweeps and the priority of 2.4 rule 2 in both runs, the probe asked for the
  file's armed cases by name and for its list of the cases it can measure (rule 4), the further sweeps and the bound
  in the release run alone; `scripts/check_cost_budget.py` and `scripts/cost_budget.toml` (2.4). *Done:* the probe
  runs the cases it is asked for by name, lists every case it can measure when asked for its list, samples the
  runnable count in a thread of its own and prints, for each case's pass, the pass's mean count (2.4 rule 2) beside
  the pass's figure, and each sweep's duration; the script's `--self-test`, on made-up
  figures, made-up per-pass means and made-up sweep start times, passes and fails as 2.4's four rules say, one case
  each way: a figure over its line by more than its tolerance without a named miss exits non-zero, by less zero and
  printed "over its line, within its tolerance"; a line-9 size one byte over its met line non-zero; a case whose
  record does not meet its line without a named miss non-zero, however close to the line, and a second such case of
  the same line and preset under the first case's miss non-zero; a second named miss of one case, and a named miss
  keyed by a case the file does not list, non-zero in either run; under a named miss whose record exceeds its line,
  a figure over its record by more than its tolerance non-zero, by less zero, and one with f < record × (1 − t)
  "record stale" and non-zero in a release run, printed and zero in a unit run; a named miss whose record is under
  L / (1 + t) of a raised line L: its figure at the line "may go" and zero, its figure under record × (1 − t) not
  stale and zero in a release run, and its figure over the line by more than its tolerance non-zero; a named miss
  without its cause or its ledger row non-zero; in a release run, a time whose passes average over 10 in the three
  sweeps and at most 10 in two later ones compared, one with one pass of three that counts swept again and compared
  once a second counts, one still short of two when the last sweep ends "machine busy" and non-zero, a sweep due
  past the bound not started, and one started before the bound and ending after it judged; in a unit run, a time
  whose three passes all average over 10 "not compared", every time so with the exit decided by the bytes, a time
  with one pass of three that counts "not compared", and one with two of three compared; an armed time "not
  compared" zero in a unit run; an armed case asked for and not reported "not reported" and non-zero in either run;
  a case the probe lists as measurable that the file does not list "unlisted case" and non-zero in either run; a
  case with no record printed and zero in a
  unit run and non-zero in a release run; a not-binding line without its reason non-zero; and `cost_budget.toml`
  byte for byte the same after every case, passing or failing. Then five recorded runs on the branch, measured as
  the release run measures (three for the bytes), each case's spread, tolerance and first record in the ledger and
  in `cost_budget.toml`, a case spreading more than 10 % fixed in the probe first (2.4, rule 3), each case whose
  record does not meet its line with a named miss of its own; the release build's time and a sweep's duration on a
  quiet machine recorded, the bound raised where three sweeps take more than half of it (2.4 *Where the numbers
  come from*). The step is not wired into
  `check_release.sh` before task 6, so it needs no `PLAN_EXCLUDED` entry and no `ci.yml` mark
  (`scripts/check_release.sh:1117-1135`; c4-14).

**The trigger step** (decision I; defined here, since neither the staging design's §2.7 nor its P-6 holds it; a
`worker` step on the measuring machine, release, once the staging task has merged with its regenerated presets passing
its gate, and before the connectors' design is checked; nothing else waits for it): task P's probe over the eight
presets as outcome 2 leaves them, the figures of 3.6 per preset, and each trigger of 3.6 evaluated on them against the
lines as they stand. *Done:* the figures and each trigger's outcome in the ledger; the records of task P's cases on
the regenerated presets made from rule 3's runs, replacing task P's, each case whose record does not meet its line
with its named miss (2.4, *Who makes records*); each part whose trigger fired named as a task before M3a's first
widget unit (below); where line 5, 7 or 11 is missed, its part's decision (the kept rules held by reference, a faster
query) taken with task 4a.

**Before the connectors' design is checked** (it states who keeps a query's result and how a connector resolves)

- The class-2 batch of 3.8 (S8 twice, S9.2, S12, S16, S17, S19.1 twice), checked as every batch, and with it
  R2.8's class-3 sentence, applied as proposed and open to the veto (section 5's first paragraphs; c4-17).
- **Task 4a, the query and resolve profile** (`worker`, on the `cost-profile` branch, release, the measuring machine).
  *Done:* where `style`'s 32–53 µs and `resolve`'s 0.23–1.47 ms go on the four profiled presets, by phase, traced to
  raw files as [P] is; the decisions for task 5, for a faster query and for the kept rules by reference stated from
  it.

**Before the first connector's code** (the connectors are written against these types, and their tests load the
presets M2's task 0 empties)

- **Task 1, trust and B** (`implement-hard`: B's comparison and the split of `check_stacked` leave judgment, and a
  wrong choice there would pass the tests and be unsound; decision K): the stack's checking, A with its two
  constructions and what decides trust (3.3 A, 3.9), B as 3.3 specifies it, the thread-local differential switch,
  E1. *Done:*
  - build-test items 1 to 3 of 3.9 through the validating construction (`validates` → `preset_stack`, `Full`), with
    3.9's controls, each run with the switch off: the negative one for item 1 (a bundled id and a text with a
    fault only the checks over combinations find is refused, `ExpressionCycle`, G1 ≥ 1, and the trusted construction
    accepts the same id and text, G1 = 0), the negative one for item 2 (a listed form with an added token cycle
    refused), the preset's input cycle refused (c3-4), and the positive one for item 3;
  - every existing test that relies on `preset_stack(None, …)` or `validates(None, …)` refusing, among them
    `src/theme/tests.rs:100`, `:296`, `:495-509` and `:992`, unchanged and passing (they take the validating
    construction, 3.3 A), and the gate's tests unchanged and passing;
  - G1 = 0, with the switch on, after each of: `from_preset` on each bundled preset whose table is empty at task 1 (at
    least `adwaita` and `kde-breeze`, [plan]:996-1000), and the twin `Stack::bundled_text` given `adwaita`'s id and a
    fixture text with an empty table (c4-12); `pick(Some)` and `pick(None)`; `set_flavor` where the preset declares
    flavours; `set_contrast` with no palette; `set_accent` of stated colours, and `set_accent(None)`, with no reader
    (on `kde-breeze` too); `set_preset` to another of them with no reader and no application layer; `set_reader` with
    the Windows fixture reading on `windows-11` (on `from_preset`'s theme where its table is empty, else on
    `from_preset_synthetic`'s, whose mutations take B), then a re-read of an identical reading, then one of another
    accent;
  - G1 ≥ 1 after each of: `add_layer` of a layer with a rule on a part; a `replace_layer` that changes only one rule's
    `when` (c2-4); `set_reader` with the KDE fixture reading on `kde-breeze`; `set_accent` on `kde-breeze` with that
    reading set; `set_contrast` under a KDE port with a layer with a rule on a part above the palette (finding 24 of
    the first check); a palette applied over the KDE fixture reading (c2-12); a re-read of the KDE fixture at the
    other scheme; `set_preset` with a structural application layer present; the cases with the KDE reader's fixture
    are compiled where the reader is, Linux with the `kde` feature (`src/lib.rs:164-165`), so they run on CI's Linux
    job alone, enough for a verdict that does not depend on the platform (1.2, 6.7; c4-13);
  - negative controls, each with the switch off and on: `set_preset` from `kde-breeze` to `adwaita` with an
    application layer whose `[for_preset.adwaita]` part names a key `adwaita` does not declare is refused with the
    structural problem (c2-1); a reading of inputs alone whose inputs read each other in a cycle, set beneath an
    application layer that states one of them as a literal, is refused with `ExpressionCycle` while B skips the
    reading's checks over combinations (c2-5); `add_layer` of a layer stating inputs alone, one of them undeclared,
    is refused with `ProblemKind::Undeclared` (`src/validate/checks.rs:1016-1028`) with G1 = 0, and one whose
    declared input is an ill-typed expression with `ProblemKind::IllTyped` (`input_values` → `expression`,
    `:1032`, `:1078-1145`) with G1 = 0, so that the structural checks run where B skips (c3-5);
  - the switch on over the crate's unit tests, the mutation tests' `stack()` helper building `Incremental` stacks
    (`resolve/tests.rs:26-51`), the tests of 3.9 *The tests' stacks* making theirs `Incremental` after construction
    with their expectations unchanged (c4-1), and a test that a skip forced wrongly (a test-only hook) makes it fail
    (its positive control); G1 and the switch thread-local, so that tests running in parallel neither share a count
    nor turn the switch off for another (c2-6);
  - G2, counted per thread (2.4; c4-7): at most one `read_preset` per preset on the test's thread while its own theme
    holds one, two themes of one preset holding the same read (`Arc::ptr_eq`), and `set_preset` back to a preset the
    test's live theme holds reading none; and in `from_system`'s path: `from_system_async` and a crate-internal test
    share one function that takes the preset's `Arc` from the cache, runs the read it is given and builds the theme,
    and the test gives it the KDE and GNOME build functions over their fixtures with fonts set and, for KDE, no
    `Xft.dpi`, so that `font_dpi` runs, at most one read on the thread (`src/gnome/mod.rs:187`, `src/kde/mod.rs:254`;
    c2-13); these two are compiled on Linux with the `kde` and `portal` features (`src/lib.rs:144-145`, `:164-165`)
    and run on CI's Linux job (c4-13);
  - a compile-time assertion that `Theme` is `Send`;
  - the doc comments of 3.8 rewritten, the build test's two among them;
  - the build test's time in debug and release in the ledger, placed by 3.9's rule;
  - the facts gate's time with `NT_FACTS_GATE_TIMING=1` before and after, in the ledger (expected the same, 3.9);
  - the probe's runs (2.4 rule 3) of every case of lines 1, 2, 3 and 4 after the task, their spread, tolerance and
    new records (2.4 *Who makes records*).
- **Task 3, the resolution kept** (`implement`): section 4's first paragraph. *Done:* a second `resolve` with nothing
  changed returns `Unchanged` and the same handle (`Arc::ptr_eq`; G3), and fails if it builds one; every `Theme`
  mutation and every field of `WindowConditions` and `ReaderState` changed alone gives a new handle, the fields taken
  by exhaustive destructuring so a new field breaks the build; the probe's runs of every case of lines 3, 5, 6 and
  11, their spread, tolerance and records (2.4 *Who makes records*).
- **Task 5, the query cache** (`implement-hard`), only if task 4a and the connectors' design decide it (section 4).
  *Done:* every query of the gate's fixture combinations answers the same with the cache and without; each field of
  `Element` and `ElementData` changed alone gives the uncached answer (the key built by exhaustive destructuring); a
  list longer than the bound answers correctly; G4; the probe's runs of every case of lines 3, 5, 7, 8 and 11, their
  spread, tolerance and records; S8's
  `Scope` line.

**Before M3a's first widget unit**

- **Task 2, colour values out of the checks' sight** (`implement-hard`: the audit is judgment). *Done:* each line of
  1.2's list (thirty-four with c2-11's two) and every equality on a `Fact`, `Val` or `OpKind` under `src/validate`
  annotated in the ledger with the place it reads and why no colour place reaches it; a stated colour flattens to
  `Unknown` (a test); the compared form sets aside the values read as `Unknown` (3.3), with its cost per op in the
  probe; every bundled preset's and `src/validate/tests.rs`' problems identical before and after (the existing
  tests unchanged and passing, and one recorded run over the presets); with the switch on, G1 = 0 after a re-read of
  the KDE fixture differing only in its accent and results at the same scheme, of the macOS fixture with another
  accent, and of a reading differing only in a string; G1 ≥ 1 after a reading that adds a colour statement the
  previous one lacked, and after one that changes a font size; the S12 line of 3.8; the probe's runs of every case
  of lines 1 to 4, their spread, tolerance and records (2.4 *Who makes records*).
- **Task 4b, `from_system` on the readers' fixtures** (`implement`, its diff reviewed before it merges, since the test
  it adds merges and the budget step runs it from then on (c3-18); its runs release, on the measuring machine, after
  tasks 1 and 2). The four readers' build functions over their test fixtures, on their own presets (a crate-internal
  ignored test, since the build functions are `pub(crate)`; for a preset whose table is still pending, through a
  test-only construction that skips the table's refusal: the figure is then the current text's, which emptying the
  table will change, c2-19). *Done:* in the ledger, per reader: the reading's size in flattened ops; the first reading
  (line 10); a re-read of an identical reading, of another accent, of the other scheme, of another font size (line 4);
  five runs of each, their spread, tolerance and first records (line 10 armed from them, 2.4), a case whose record
  does not meet its line with its named miss (3.6's row of the readings proven at build time); the test added to the
  budget step's runs, which time line 10 and line 4's re-reads from then on (one process per pass for line 10); the
  decision on task 7 stated from them by 3.4's rule.
- **Task 6, the budget step in the gate** (`implement`): the step enters `check_release.sh`'s full run and the plan's
  gate per unit names it (2.4). *Done:* the step's release run green on the branch, so that every armed case was
  compared (a unit run can pass with none compared), every case armed or not binding, each case whose
  record does not meet its line under a named miss naming its cause; its self-test as in task P. On this machine
  the project's own parallel lanes (a gate of up to 16 threads beside a worktree build, memory
  `feedback_resource_limits.md`) keep the runnable count over 10 by themselves: pause other lanes for the release
  run, or it sweeps again for 30 minutes and fails "machine busy" (c6-12).
- **Task 7, the readings proven at build time** (`implement-hard`), only if task 4b decides it (3.4). *Done:* `Fact`'s
  payload readable only through the recording module, its equality included: the `PartialEq` derive taken off
  `Fact`, and `Val`'s and `OpKind`'s equality and `curves.contains` (`combos/motion.rs:601`) going through the
  module, so that neither a pattern nor a `==` on a payload outside it compiles (c2-11; B compares layers, never a
  `Fact`, so it needs no route); the proven places per reader; build-test item 4, failing for a fixture preset that
  states an expression at a proven place (its positive control); G1 = 0 for each reader's maximal fixture reading
  and each with its removable statements left out, over its own preset; G1 ≥ 1 for a reading with a statement
  outside its proven places, over a preset not its own, and with an application layer present; the differential
  switch over the readers' corpus; the build test's added time; the S12 line of 3.8; the probe's runs of every case
  of lines 4 and 10, and of lines 1 and 2 under a veto of A, their spread, tolerance and records (2.4 *Who makes
  records*).
- **Each part whose trigger fired at the trigger step** (3.6): C (`implement`, its gain first measured on one preset
  by a `worker`); D (a spike by a `worker` first, then, if D proceeds, a design note checked as every design document
  and `implement-hard`), or (11) (`implement`); building by need, resolution by need and the repairs inside the checks
  (`implement-hard`); the kept rules held by reference, a faster query and a feature per preset (`implement`).
  *Done* for each: its trigger line met in the probe on the presets that fired it, its records made (2.4) and its
  named miss removed, or the miss that remains named with its cause and its new record in `cost_budget.toml` and
  reported to the maintainer with D6's other figures.

**At the calibration after M3a's first widget unit** (the second look): each trigger of 3.6 evaluated again on that
unit's figures, its outcome in the ledger; a part that fires there is built before M3a's second unit.

**When the tasks are done:** this note and its checks move to `docs/archive/` with the plan's archiving (the plan's
last task; `worker`).

---

## 6. What would defeat the design

Trust and B:

1. **The embedded data is not what was validated.** Excluded by build-test items 1 to 3 at every commit, in every gate
   mode and on CI's three platforms (1.2): the same text, read through the same `source()`, validated through the
   validating construction (the full path, B off), whose negative control (3.9) fails if the test takes a route that
   checks nothing; and only `from_preset` and `set_preset` trust, through functions that take a `PresetId` and no
   text and read that same embedded text (3.3 A); item 3 builds through `from_preset` and `set_preset` themselves.
2. **The stack that loads is not the stack that was checked**: a derived layer in a form the test did not build.
   Excluded by item 2 over every form, on the full path and never through B (its negative control run with the switch
   off), and item 3; the forms listed by one function the run time and the test call; a form the run time derives
   and the list lacks is checked, not trusted (3.3 A). The facts gate checks each condition vector's derivation
   layer on the full path besides (3.9).
3. **A check changed and the presets not checked again.** Not possible in one commit: the test runs the commit's
   checks on the commit's presets.
4. **B skips a stack whose checks would refuse it**: the checks read something the comparison sets aside (an input
   outside the cycle search and a palette's mapping check), a check B skips depends on context B does not compare,
   or leaving out an empty layer changes a finding. Guarded by 1.2's account of what they read; by the always-run
   part, which keeps every check that reads the context per layer (the structural checks, the input cycle search);
   by task 1's controls for the cases that account names (c2-1, c2-4, c2-5, c3-5); and by the differential switch,
   which detects a wrong skip only on an `Incremental` stack a unit test changes, the stacks of 3.9 *The tests'
   stacks* among them (c2-6, c4-1; the facts gate takes the full path, so it neither runs B nor tests it, 3.9).
5. **A value set aside by task 2 is read after all.** The switch cannot see it (both sides blind). Excluded by the
   audit of 1.2's list and of every equality on a payload, the identical problem lists before and after, and the
   flattening itself: a check that needs a colour's value must change it, which the test of `Unknown` at a colour
   place makes visible.
6. **A reading accepted by task 7 is refused by the checks.** Excluded by the recording lookup (an unrecorded read does
   not compile), build-test item 4, and the switch over the readers' corpus in the crate's unit tests.
7. **A verdict that differs by platform.** No code of the path is conditional (1.2); the build test runs on three
   platforms. Task 7 would add one condition: the KDE reader is compiled only on Linux with its feature
   (`src/lib.rs:164-165`), so build-test item 4's KDE part runs on one CI platform, which is enough for a reader that
   runs on that platform alone (c2-23).
8. **A crate patched downstream** (a vendored preset edited). Outside what the crate can promise; the build test and
   the doc-hidden gate are there to be run.

Caches:

9. **The read cache.** Bundled data cannot change in a process; a `Weak` keeps nothing stale; a poisoned lock reads
   without it.
10. **A resolution reused after a change.** Excluded by task 3's test over every mutation and condition field; layers
    are compared by identity, else by value (`src/resolve/mod.rs:1971-1977`), and never changed in place (`:599-600`).
11. **The query cache's key leaves out something the query reads** (if built). Excluded by the exhaustive
    destructuring and task 5's two tests.
12. **Font metrics changed with nothing else; a scope of an old resolution; a connector's derived object not
    rebuilt.** Section 4's metrics rule, S8's rebuild duty ([S]:4394-4395, :4414-4418), the contract-test row.

---

## 7. What could not be settled from the sources

| # | Open | What settles it |
|---|---|---|
| 1 | What a structural reading costs at `from_system` and at a re-read, on each reader (`kde-breeze` and `adwaita` not profiled) | task 4b |
| 2 | Where a query's 32–53 µs and a resolve's 0.23–1.47 ms go | task 4a |
| 3 | What a mutation costs today (1.2's figures are read off the code), what B's comparison and the always-run part cost on a reading and an application layer, and how large a reading is | task P, task 1's runs, task 4b |
| 4 | The Apple presets' size after outcome 2, and every preset's at thirty widgets ([pkg]'s projection predates outcome 2) | the trigger step; the calibration after M3a's first unit |
| 5 | Whether any check reads a colour's `Fact` (this note's reading of 1.2's list says none) | task 2's audit |
| 6 | Which of each reader's statements pass 3.4 (b)'s assertions | task 7's build test, if built |
| 7 | The build test's duration, today and under A | task 1 |
| 8 | Whether `Theme` is `Send` today | task 1's assertion |
| 9 | The four presets not profiled; their reads are extrapolated in 1.3 | task P, the trigger step |
| 10 | Whether 0.8.0's egui-widgets queries every frame (and so whether line 8 binds) | the connectors' design |
| 11 | The budget's eleven numbers, the 300 parts, the factor of six the *between frames* rule leaves a slower machine | the maintainer, at any time; until then as proposed (section 5) |
| 12 | Each line's spread at a runnable count of at most 10 at the probe's own scale (sub-µs figures above all), on which its tolerance rests, and how often a unit gate finds the machine that quiet | task P's five runs; the unit gates' printouts |
| 13 | Whether a stated `follow_part` or `concentric` corner whose sides reach the same number through a font size and a length is accepted today (1.2's defect, built from the code, not executed) | the orchestrator's separate fix, its failing tests at `ebd1c723` |
| 14 | How long a sweep takes on a quiet machine, which the release run's 30-minute bound counts | task P (2.4 *Where the numbers come from*) |

---

## 8. Findings not taken, or taken otherwise

### 8.1 The first check (`load-design-check.md`)

Every finding of the check was verified at its evidence (the code and document lines it cites read again; its
arithmetic redone by `revision2_figures.py` and `read_marks.py`); all hold. These were not taken as worded:

- **11, "gate each time only at a bound far from its budget (for example 10 × the line)".** Revision 2 did not take
  it, arguing from [L]'s 3.8 times between passes; that argument did not hold (3.8 is below 10, was measured at a
  load average of 20 to 86, and before the lowest of passes, c2-8). Revision 3 bounded each time at 5 times its
  record; revision 4 takes it otherwise again (2.4): a time is compared only at a load of at most 10 (the load
  average then, the runnable count since revision 7), where
  passes agreed within 0.9 %, against its line or a named miss's record, with a tolerance of twice its measured
  spread; on the measuring machine's release-mode step, never in `cargo test`.
- **12, "leave input literals to B2".** Taken otherwise: no B2 is on the path; inputs are outside the checks' input
  entirely (1.2), so B's comparison sets them aside.
- **13, "the counter must be at least 1 for `set_accent` on `kde-breeze`".** Taken with the KDE reader set. Without a
  reader the right count is 0: the accent-derivation layer is in a trusted form and the accent layer's compared form
  is empty.
- **13, "at least 1 for `set_contrast` under a palette that is skipped there".** Taken otherwise: with no reader and no
  structural layer above the palette, the right count can be 0 (the skipped palette's entry states nothing); the
  control puts a structural application layer above it, which is finding 24's case.
- **16, "closed places by type: colours and strings stay inert; needs no generated file".** Taken otherwise: a stated
  colour carries `Fact::Number` of its red channel today (`stack.rs:443-455`, `:585-597`; `walk.rs:882-893`), so
  colours are open only after task 2; strings are (`walk.rs:921`). Numbers and variants are decided by recording in
  task 7, not closed by type, since the font sizes the Linux readers state are read through length expressions
  (`combos/mod.rs:859-869`).
- **17, "only egui queries every frame".** Taken as a fact about 0.7.0's connectors (1.2: egui-widgets reads in every
  `ui()`; gpui, iced and egui convert at a change), not about 0.8.0's, which the connectors' design states.
- **17, "key a part's entry only on the element atoms its reached rules name; test a list longer than the bound".**
  Taken as requirements of task 5, if the cache is built.
- **19, "the range for the four is 357 to 443" bytes of content per rule.** The comparison is dropped with C: revision
  1's content figure (149,961 B for `windows-11`) could not be reproduced by any of three methods
  (`content_bytes.py`: 152,870 to 156,928 B), so no content-per-rule figure is cited.
- **15** (the committed artefacts) and **22** (B2's bookkeeping): moot on the path, since neither a closed-places file
  nor D is built now; 15's corrections are recorded for D's return (3.6), and task 7 records `Fact` reads by
  statement, not by flattened path (`stack.rs:409` embeds a source's number in a list item's place).

### 8.2 The second check (`load-design-check2.md`)

Every finding was verified at its evidence; all hold. Each is taken, these with a difference:

- **c2-2, "give the comparison a default that needs no flattening … the flattened form is needed only for
  colour-only re-reads after task 2".** The default is taken (decision H). For colour-only re-reads the flattened
  form is not taken either: task 2 sets the values the flattening reads as `Unknown` aside in the compared form, by a
  comparison over the layers' values (3.3), so no path builds a flattening to compare. The reason: one form of
  comparison, whose cost rate is measured on the presets' own layers (r3f 1), against two.
- **c2-6, "count only B's runs".** Taken, and the negative controls run with the switch off as well, since with it
  on the full path would refuse what B wrongly accepted and the control would pass for the wrong reason.
- **c2-7, "a ratchet … for example 5 %".** Taken for every named miss, bytes and times (revision 4, 2.4 rule 1:
  a named miss is bounded by its record, with a tolerance of twice the measured spread and at least 5 %). A line met
  is bounded by the line itself (revision 4; revision 3 ratcheted every record).
- **c2-8, the two alternatives.** Revision 4 takes neither as worded: the times are gated against their lines, as
  the first alternative asks, but only where the load is at most 10 (the runnable count since revision 7), so no k
  is needed; the release must
  compare every armed line, as the second asks of the release (2.4 weighs the options).
- **c2-9, "evaluate the triggers at outcome 2's batch … or put line 9 to the maintainer now".** The triggers at the
  trigger step, right after the staging task (decision I; revision 4 names the step, c3-8); line 9 goes to the
  maintainer first among the numbers, as a veto item that blocks nothing (section 5, decision O).
- **c2-13, "an injectable reader" as one route for G2.** Not taken: a function `from_system_async` and the test
  share, given the read to run, tests the code `from_system` runs without adding a reader trait.
- **c2-17**: the plain route's figures replace the synthetic ones wherever a trusted theme is meant (0.3, 3.2, 3.4,
  3.5); 1.1's table keeps both routes' figures as measured.
- **The unverified candidate** (check 2, *Defects in today's code*): verified, and a defect (1.2, *A defect of
  today's checks*); outside this design, fixed apart from it.

**Decisions not applied.** None: each of H to K was applied as given. One figure's scope is stated more narrowly
than decision H states it: the structural checks' 2.5–8.4 µs a layer were measured on the synthetic route's layers,
of 3.1 to 3.8 flattened ops each (r3f 2); a larger layer, a reading or an application layer, is expected to cost in
proportion to its size (3.3). The decision (run them for every layer at every change) stands on either figure, since
it rests on correctness (c2-1).

### 8.3 The third check (`load-design-check3.md`)

Every finding was verified at its evidence: the code and documents it cites read again in the worktree at
7430a0a2 (`src/resolve/mod.rs:669-704`, `:819-841`, `:914-936`, `:1412-1470`; `src/validate/mod.rs:86-122`,
`:199-208`, `:255-290`; `combos/mod.rs:466-489`, `:3254-3267`; `src/theme/mod.rs:66-131`, `:333-337`;
`src/theme/tests.rs:20-80`, `:100`, `:296`, `:495-509`, `:992`; `src/gate/mod.rs:115-200`, `:370-396`;
`src/layer.rs:15-31`, `:175-188`; `src/lib.rs:7-11`; `native-theme/README.md:12-15`, `:110-116`; `README.md:72`;
`CHANGELOG.md:8`, `:30`; `presets/windows-11.toml:1624-1832`, `:2164`; [plan]:996-1009; the staging design's §2.7 and
§6.4's P-6; [O]:5-6, :28-35), and its figures by `revision3_figures.py`. All hold. Each is taken, these with a
difference:

- **c3-1.** Taken as decision L sets it, with the validating construction a property of the stack (`Full`), not a
  parameter of one function, since the facts gate's per-vector changes must take the full path too (3.9 weighs the
  three ways). Item 2 checks each listed form by the full path's own `check_stacked`, not by building it through
  mutations: the check is the one B's skip stands for, and the list only decides what B may skip (3.3 A). The
  differential switch has no use in the gate any more, so its doc-hidden function goes.
- **c3-2.** Taken (decision M). The check's "require a lower load for the release's recording" is taken for every
  time comparison, not the release's alone (2.4 rule 2), since with the factor 5 gone only a quiet machine keeps
  noise inside the tolerance. "Over its line" means over it by more than the tolerance: a figure within its
  measured noise of its line does not fail, which the requirement "never fail on scheduling noise" needs. (Revision
  5 replaces this reading and the release's writing of records, 8.4, c4-2 and c4-5.)
- **c3-3.** Taken (decision N): a named miss cites any section or task; 3.6 gains the rows for lines 4, 7, 8 and 11
  (line 11 had no remedy either; the kept rules held by reference and resolution by need are its remedies, since the
  resolution's clones are its allocation).
- **c3-6.** Taken in its first form ("trusted only for `Some(id)` with an empty table, `None` always validating")
  only in effect: trust is keyed on the route and the embedded text (3.3 A), which also refuses trust to
  `preset_stack(Some(id), other_text)`; no test of today changes.
- **c3-8.** Taken: the step is defined here (the trigger step), §2.7 cited by section; decision O removes the
  maintainer's stop.
- **c3-10.** Taken by decision N and 2.4 rule 3: no "reported only" state; a spread over 10 % is a probe defect.
- **c3-11.** Taken: a record is the lowest of its runs' figures (2.4 rule 3); with no
  stale bound any more, the record bounds only a named miss whose record exceeds its line (since revision 7, c6-13;
  revision 6's stale record under a named miss is a
  different rule, 8.5 c5-1).
- **c3-15, c3-16.** Moot: the byte factor's basis is now stated as a judgment until task P's runs (2.4), and the
  factor 5 and its "2.8 times" are gone.

**Decisions not applied.** One parenthesis of decision N states something false: "lines 6 and 8 when the query
cache is not built" names line 6 among the lines whose mechanism is not built. Line 6 (`resolve` with nothing
changed, 10 µs) rests on E2, the resolution kept in `WindowState`, which is on this release's path whatever task 4a
decides (0.2, 3.3 *E2*, section 4's first paragraph, task 3), and G3 gates it. So line 6 is armed from task P's
first record and has no "not binding" state; only line 8 does, where the connectors' design shows no connector
querying every frame. Decisions L, M, O and P are applied as given, M with the reading of "over its line" stated
under c3-2 (revision 5 removes M's writing of records and replaces that reading, 8.4).

**Not covered.** The synthetic route's own speed (the gate's and the tests' loads of a preset with a pending table),
whose load stays on the full checks and under the plan's gate rules; the readers' system calls; the connectors' own
costs; the icon loaders. No build, test or measurement was run for this note; 1.2's `Fact` reads were read at their
lines, not audited line by line against every caller; the defect of 1.2 and the controls of task 1 were built from
the code's mechanism, not executed; no reader's layer was sized; the toolkit crates themselves were not read beyond
the connectors in this tree. Revision 4's stack checking (`Full`, `Incremental`) and its routes were reasoned from
the code at the lines cited, not compiled; its call sites come from a text scan (`revision4_sites.py`).

### 8.4 The fourth check (`load-design-check4.md`)

Every finding was verified at its evidence in the worktree at 7430a0a2: `src/resolve/mod.rs:596-1000`,
`:1186-1200`, `:1868-1882`; `src/theme/mod.rs:59-131`, `:320-358`; `src/presets/mod.rs:25-100`;
`resolve/tests.rs:20-55`, `:242-280`, `:625-1337`; `src/theme/tests.rs:18-82`, `:120-130`, `:292-300`, `:1155-1513`;
`resolve/mutation_tests.rs:1-60`, `:85-115`, `:170-200`; `src/lib.rs:142-192`; `src/gate/mod.rs:3500-3615`;
`scripts/check_release.sh:1110-1135`; [S]:6680, :7130-7137; [plan]:60-77, :994-1006, :1056-1072, :1150-1163,
:1376-1388; `native-theme/README.md:84-92`; `2356f84e:native-theme/tests/profile_mem.rs:1-30`; the scans
`revision5_mutated.py`, `load-check4/wrapped_rule.py` and `revision5_spec_rule.py`. All hold. The orchestrator
decided the corrections of c4-1 to c4-9; the minor ones are taken where they hold, which is everywhere. These are
taken otherwise than the check wrote them, or with a choice the brief left open:

- **c4-1.** Taken, with the list completed by `revision5_mutated.py`: the check's ranges hold, and the scan adds
  `accent_derivation_layer_places` (`resolve/tests.rs:634`), `a_port_leaves_the_stand_in_out_but_its_accent` (through
  `breeze_at`) and, in `src/theme/tests.rs`, the stacks changed through the helper `switch` (`:1162-1165`, a call of
  `Stack::set_preset`). The conversion is the method `from_preset_synthetic` already needs, not a second
  `#[cfg(test)]` one: making a validated stack `Incremental` is sound wherever it is done (check 4's *Decisions
  confirmed*), so a test-only twin would add nothing.
- **c4-2.** Taken as the orchestrator decided: no run writes a record; neither the check's downward ratchet at the
  release nor `min(record, figure)`. *Why decision M's text goes*: M answered check 3's finding 2 under revision 3,
  where records bounded met lines, so without the release's writing no time line bound a release. Revision 4's rule 1
  bounds a met line by the line itself, so that reason is gone. What M still did was harmful: a named miss's bound,
  its record, moved up by up to its tolerance at every passing full run (1.05^n at a 5 % tolerance, never failing),
  on the lines missed most (1, 2, 3, 9 on the Apple presets, 5 on `macos-tahoe`, 10), and a check script left a
  tracked file changed, so the commit tagged was not the commit checked. A named miss whose figure falls under its
  line is now printed as one that may go, and removed by the next deliberate act that records it; since revision 6
  that act is forced where the figure has fallen below the record by more than its tolerance: the release run fails
  with "record stale" until the case is recorded again (2.4 rule 1, c5-1).
- **c4-3.** Taken: `Stack::bundled(id: PresetId, scheme: Scheme) -> Result<Stack, LayerError>` and
  `Stack::set_preset_bundled(&mut self, id: PresetId) -> Result<(), LayerError>`, no text parameter; the fixture texts
  of task 1's controls through the `#[cfg(test)]` twin `Stack::bundled_text`, the shared inner function private to
  the module `resolve` (revision 6's wording, c5-8). `set_preset`'s trusted path has one reading (3.9): only
  `set_preset_bundled`, on an `Incremental` stack, for an empty table, skips the new preset's own check;
  `Stack::set_preset` keeps it on every stack, so `resolve/mutation_tests.rs:100-105` and `:186-189` keep their expectation, refused as a preset failing
  validation on the `stack()` helper's `Incremental` stack. No test's expectation changes.
- **c4-4.** Taken as the orchestrator decided, with a bound of 30 minutes (its reason in 2.4 *Where the numbers come
  from*). Revision 5 gave a unit's gate the same wait; revision 6 applies the orchestrator's decision Q: the wait and
  "machine busy, run again" are the release run's alone, and a unit run does not wait, compares times only where the
  load is low over their passes (since revision 7, the runnable count alone, with no settle time, c6-1), and prints
  "not compared" otherwise, passing or failing on the bytes. Q's reason: unit gates run often on a machine other
  projects build on, and the release run compares every armed case anyway. So a passing unit gate no longer implies
  that every armed time was compared; a passing release run still does. The check's "say that a release's step needs
  the machine's other builds stopped" is not added as a rule: the wait covers it, and a machine kept busy says so;
  since revision 7, task 6 says it as an instruction (c6-12).
- **c4-5.** Taken: "meets its line" is defined once in 2.4, rule 1 checks from the file that a case whose record
  does not meet its line has a named miss, and a named miss goes only by a deliberate act whose record meets the
  line.
- **c4-6.** Taken as the orchestrator decided; the check's fourth correction (a tolerance of twice a spread over 10 %
  as a state the release accepts) is not taken: the task making the record fixes the probe first (rule 3).
- **c4-7.** Taken as a thread-local count, not a test per isolated process. The reason: the full run and CI run the
  crate's tests with `cargo test`, many threads in one process, where only a per-thread count is free of another
  test's themes; an isolated process would need an integration test binary of its own, which cannot see the crate's
  `#[cfg(test)]` counter and `pub(crate)` read path, or `--test-threads=1` over the whole crate. A thread-local
  counter is G1's mechanism already, reads happen on the calling thread (`from_system` blocks on it,
  `src/theme/mod.rs:59-61`), and the loss of E1 still fails it: without the cache two themes of one preset read
  twice on the thread.
- **c4-8.** Taken; the line-joined scan rerun over the repository finds nothing more of the rule, and the same scan
  over the specification and the rationale (`revision5_spec_rule.py`) finds the sentences 3.8 already re-quotes and
  S17's.
- **c4-9.** Taken by the first of the two ways: the probe's doc-hidden function takes the validating construction
  (since revision 6 it times the read alone and builds no stack, c5-9).
  The reason: on a preset whose table is pending, line 1's figure is the read's (3.5), which the function times
  apart, so it needs no construction without checks; and with no exception, 3.3 A's "only `from_preset` and
  `set_preset`" holds as written.
- **c4-16.** Taken; the claim rests on a global allocator counting every thread's allocations and on the cost-profile
  running its binary with one thread (`profile_mem.rs:6` at 2356f84e). `peak_alloc`'s own source, in Cargo's
  registry, was not read for this revision (outside the paths this revision may read); check 4 read it.

**Not covered.** No build, test or measurement; the tests' verdicts under `Incremental` (3.9 *The tests' stacks*)
were reasoned from each refusal's check at its line, not run; `revision5_mutated.py` is a text scan whose function
bounds follow indentation, so a test that changes a stack only inside a closure defined elsewhere would be missed.

### 8.5 The fifth check (`load-design-check5.md`)

Every finding was verified at its evidence in the worktree at 7430a0a2 (`resolve/tests.rs:20-52`, `:105-112`,
`:700-710`; `resolve/mutation_tests.rs:40-60`, `:185-195`; `src/resolve/mod.rs:20-32`, with `grep Stack` over
`resolve/compose.rs`, `resolve/evaluate.rs` and `resolve/evaluate/`; [plan]:58-78, :1150-1164; [P]:1-8; [L]:1-8;
check 4's finding 4) and in this note's own lines, its arithmetic by `revision6_load.py`. All hold. The orchestrator
decided Q to U; the minor findings are taken where they hold, which is everywhere. Q is applied at the fourteen
passages check 5 lists (revision 5's item 4, 0.6, 2.4's introduction, rules 2 and 4, *Where the numbers come from*,
*What it gives*, *The options weighed*, the step's build, the changed line applied in 2.4 and section 5, task P and
its self-test, task 6's *Done*, and c4-4 above). Taken otherwise, or with a choice the brief left open:

- **c5-1.** Taken as decision R. Task 1 records line 3 too, beside the decision's lines 1, 2 and 4: E1 changes what a
  theme holds, the figure line 3 counts, and R's rule is every case a change moves. By the same rule tasks 3 and 5
  record every line whose figure includes the resolution they change (3, 5, 6, 11 and 3, 5, 7, 8, 11). A stale case
  that no listed act re-records gets an act of its own, so the release run's failure always has a way out. This is
  not revision 3's stale-record bound (2.4 *The options weighed*), a fifth of every record (check 3, minor 11), which
  closed a ratchet's gaps; this one acts only under a named miss, where the record is the bound, at its tolerance.
- **c5-3.** Taken as decision T. The check's second correction, a floor of its own for the allocator's bytes, is not
  taken: T keeps the 5 % floor for every measured figure.
- **c5-4.** Taken as decision U. The one-minute readings before and after the passes are not enough (2.4 *Where the
  numbers come from*), so a pass counts only where the runnable count, sampled every 100 ms, averages at most 10
  over it; since revision 7 the count replaces the readings rather than adding to them (c6-1). The mean, not the
  highest sample: one sample is an instant, and a spike of a few milliseconds cannot move a median of 201 samples.
  Check 5's other signal, `/proc/pressure/cpu`, is weighed and not taken there. A time short of passes that count is
  swept again in the release run and printed "not compared" in a unit run. The step runs at `lowprio.sh`'s priority,
  set by itself.
- **c5-7.** Taken by its first way, the exception stated (3.3 B). The second way, no stack before the change on a
  helper-built stack's first change, would need a mark of having passed, which a test that writes a field after a
  change would leave set, so it would not cover those tests. The differential switch keeps the exception loud.
- **c5-9.** Taken. The validating construction's timing is dropped: it fed no line, and it cost 9.2 s a pass on the
  four profiled presets (the sum of 1.1's preset checks). The function times the read alone and builds no stack, so 3.3
  A no longer lists it among the routes that validate.

**Not covered.** No build, test or measurement. `/proc/loadavg`'s fourth field and `/proc/pressure/cpu` were not read
on the machine: the project's path guard refused the read. They are named from general knowledge (proc(5)), and task
P's script checks the field on the measuring machine. The runnable count's limit of 10 is a judgment from [P]'s load
range, until task P's runs measure it (revision 6's 120 s settle time, from `revision6_load.py`'s arithmetic, is
removed by revision 7, 8.6).

### 8.6 The sixth check (`load-design-check6.md`)

Every finding was verified at its evidence: this note's own lines; `revision6_load.py` and check 6's
`load-check6/average_guard_hole.py` run again (69.8 s, 4.33 and 6.56 at 120 s, 6.61 and 8.81 for the bursts, as the
check gives them); `native-theme/tests/timings.rs` at 814e96e7 (`:264`, `:345-357`: figures per preset, no clock, no
per-pass load); `src/resolve/mod.rs:1550-1566`, `:1969-1980` at 7430a0a2; memory `feedback_resource_limits.md`
("Facts gate threads are capped at 16"; a second, nice-19 build beside the gate). All hold but one premise of c6-5,
below. The orchestrator decided the corrections; they are applied at every passage the check lists. Taken otherwise,
or with a choice the brief left open:

- **c6-1.** Taken: the runnable count is the only load signal; the one-minute average, its start and end readings and
  the 120 s settle time are gone from 2.4, revision 6's items 4 and 5, task P, c4-4, c5-4 and section 7 (and the older
  revisions' lists and 8.1, 8.2 say since when). *Why the settle time was dropped*: it existed only because the
  average remembers the step's own build; the count drops the build's threads as they exit, and load before a pass
  does not contend with it (its after-effects are below), so the settle cost every unit gate 120 s and, after a full
  build, lowered a unit run's effective limit to a background of 6.56 (`revision6_load.py` item 2), against the limit
  of 10 the note derives, on two unmeasured assumptions (the build's average 32 at its end, a constant background).
  *What would change it* (the check's): task P's runs showing the first sweep after a build systematically slower than
  the later ones (caches, temperature, writeback), which brings back a pause of a measured length before the first
  sweep; or slowdowns at a count of at most 10 that the count does not explain (memory stalls, which a direct reclaim
  causes without raising the count), which adds a memory signal such as `/proc/pressure/memory`, not the average.
- **c6-2, c6-3.** Taken: "less 2: the sampling thread, running as it reads, and the pass's thread", stated once in
  rule 2; the probe samples in a thread of its own and prints each pass's mean beside its figure, which task P's
  *Done* names. The pre-sweep mean has no pass running, so it errs by one thread toward sweeping; each pass is still
  judged by its own mean, so this costs at most a sweep whose passes do not count. (Revision 8 subtracts nothing and
  removes the pre-sweep mean, 8.7, c7-4 and c7-5.)
- **c6-5.** Taken: "moves" is "can move by more than its tolerance", applied to tasks 2, 3 and 7 alike. One premise
  does not hold: task 3 changes no mutation. `resolve` sees a change by comparing what it recorded
  (`src/resolve/mod.rs:1557-1562`, `classify` at `:1980`; layers by identity, else by value, `same_layers`,
  `:1969-1977`), so a
  mutation gives a new handle with no code of task 3 in it, and task 3 does not record line 4. Task 2 records lines 1
  to 4: its compared form may be a copy kept beside each layer (3.3), which a load builds and the theme keeps. Task 7
  records lines 4 and 10, not line 2: with A no load runs the checks over combinations, whose `Fact` reads it
  changes; under a veto of A it records lines 1 and 2 (section 5's veto sentence says so too).
- **c6-8.** Taken by its second way, which adds no rule: a figure over its line counts as a miss where its record
  exceeds the line too, which rule 1 already requires a named miss for. The first way, two consecutive release runs
  over the line needing a named miss, adds a rule and state the step does not keep (no run writes the file, and the
  step reads no ledger), for a band one run cannot tell from noise. So the step prints such a figure "over its line,
  within its tolerance" and *What it gives* states the band. (Since revision 8 the act is named: the release task
  gives a case printed so at two consecutive releases a named miss, 8.7, c7-3.)
- **c6-9.** Taken in a form of its own: a time counts only where at least two of its passes count, so it rests on
  the lowest of at least two, as a record does on its runs', and never on a single pass; the release run sweeps again
  until two count, a unit run prints "not compared" with fewer.
- **c6-10.** Taken with the stale test written `f < record × (1 − t)` and the failure `f > bound × (1 + t)`, the
  tolerance a fraction.
- **c6-12.** Taken in task 6 as an instruction, since `scripts/check_release.sh` prints no release checklist to hold
  it (a scan for "checklist" at 7430a0a2 finds none).
- **c6-13.** Taken in the orchestrator's form, not the check's: a named miss whose record meets a raised line fails
  no file check; where its figure meets the line it prints "may go", as for every case, and the next deliberate act
  that records the case removes it. So that the record no longer bounds a case under its line (which *What it does not
  do* would contradict), rule 1's bound under a miss is the record only where that exceeds the line.

*The situations, as rules 1 to 4 read since revision 8* (check 6's nineteen, check 7's A1 to A15, and R1 and R2
for the re-record the orchestrator decided after check 7; `revision8_situations.py` applies the rules to each, and
prints its own variants 4b, 8a and 8b, 9b, 18b and 18c, U, a not-binding case with and without its reason, A6a and
A6b, A11b, R2b). L is the line, r the record, t the
tolerance, f the figure; a time's f is the lowest median of its passes that count, compared only where at least two
count. "Busy" is a pass's mean count over 10.

| # | Situation | Unit run | Release run |
|---|---|---|---|
| 1 | Met line (r ≤ L), noise: f = 1.03 L, t = 5 % | compared where two passes count: f ≤ L(1 + t), passes, printed "over its line, within its tolerance" | passes, printed so |
| 2 | Met line, f grows from 0.5 L to 0.9 L | passes | passes; the growth shows in the printout and the ledger |
| 3 | Named miss (r > L), f = 1.03 r | passes | passes |
| 4 | Named miss grows: f = 1.10 r | fails if compared; with fewer than two passes that count, "not compared", passes | fails |
| 5 | Named miss, f falls under L, f ≥ r(1 − t) | passes, "miss may go" | passes, "miss may go" |
| 6 | Named miss (r > L), stale: f < r(1 − t) | prints "record stale: re-record", passes | fails "record stale" |
| 7 | Exact figure (line 9), met line, one byte over L | fails (bytes compared even when busy) | fails |
| 8 | Exact figure under a named miss (r > L), one byte over or under r | over: fails; under: stale, printed, passes | over: fails; under: fails "stale" |
| 9 | Busy from before the first sweep | three sweeps, no pass counting; every time "not compared"; exit by the bytes and the file check | sweeps again over the times short of two; compared once two count; a time still short when the last sweep that started within 30 minutes of the build's end ends: "machine busy, run again", non-zero, bytes printed |
| 10 | Load rises during the sweeps (sweeps 2 and 3 busy) | each time compared on its passes that count, "not compared" with fewer than two | sweeps again over the times with fewer than two |
| 11 | One case's three passes busy | that time "not compared" | sweeps that case again until two count; no sweep starts past 30 minutes |
| 12 | One of three passes counts | "not compared" | sweeps again until a second counts; no sweep starts past 30 minutes |
| 13 | Case with no record ("first measured by") | named in the printout, not asked for, passes | fails (neither armed nor not binding) |
| 14 | Armed case asked for and not reported | fails "not reported" | fails "not reported" |
| 15 | Case the probe can measure that the file does not list | fails "unlisted case", a file defect (rule 4) | the same |
| 16 | Second missing case of a line and preset under the first case's miss | fails (file check) | fails |
| 17 | Named miss without cause or ledger row | fails | fails |
| 18 | Named miss whose record meets a raised line | file check passes; the line is the bound; f ≤ L prints "may go"; f > L(1 + t) fails; no stale test | the same; f < r(1 − t) passes, "may go" |
| 19 | Met case pushed to 1.04 L by a widget unit, t = 5 % | passes, printed "over its line, within its tolerance" | passes, printed so; the step never names it; the release task gives it a named miss once printed so at two consecutive releases |
| A1 | Met line, f = L(1 + t) exactly | passes, "over its line, within its tolerance" (">" is strict) | the same |
| A1b | Named miss (r > L), f = r(1 + t) exactly | passes | passes |
| A1c | Named miss (r > L), f = r(1 − t) exactly | passes; not stale ("<" is strict) | passes |
| A2 | Tolerance 0 (line 9), named miss, f = r | passes | passes |
| A3 | r = L (meets its line, no miss), f = L | passes; only the figure printed | passes |
| A4 | Miss under a raised line (r < L), L < f ≤ L(1 + t) | passes, "over its line, within its tolerance"; no "may go" | the same |
| A5 | Miss under a raised line, r = 0.8 L, f = 0.7 L | passes, "may go"; no stale test (c7-6) | the same |
| A6 | Every pass busy; a bytes case over its met line by more than t | fails on the bytes; the times "not compared" | fails on the bytes; the times swept again until they count or the bound |
| A7 | Sweeps 1 and 3 count, sweep 2 busy; a time at 2 L | fails: two passes count (c7-5) | fails |
| A8 | Release run, an armed time with no pass at all | fails "not reported" | fails "not reported" |
| A9 | A named miss keyed by a case the file no longer lists | fails, a file defect | the same |
| A10 | Two named misses keyed by one case | fails, a file defect | the same |
| A11 | Release run reaches 30 minutes from the build's end during a sweep | no bound; judged on its three sweeps | the sweep finishes and is judged; no sweep starts after it; a time with two passes that count is compared, one with fewer fails "machine busy" |
| A12 | Quiet machine whose three sweeps together take over 30 minutes (20 each) | compared | compared: sweeps 1 and 2 start within the bound and count, sweep 3 does not start; task P's rule keeps three quiet sweeps within half the bound |
| A13 | An armed case that a probe change no longer produces | fails "not reported" | fails "not reported" |
| A14 | A named miss on a case with no record ("first measured by") | not asked for, passes | fails (not armed) |
| A15 | Met case grown under its line to 0.99 L, noise gives f = 1.02 L, t = 5 % | passes, "over its line, within its tolerance" | the same; one run cannot tell it from noise; the release task names it after two consecutive releases so printed |
| R1 | Re-record of a case under a named miss (r > L), t = 5 %, the new runs' lowest 1.03 r | the act keeps r and states no cause (rule 3); a run at f = 1.03 r passes | the same |
| R2 | The same, the new runs' lowest 1.10 r (R2b: 0.90 r) | the act records it as the new r, its cause in the ledger row (rule 3); a run at that figure passes | the same |

Every row has one answer in each run. A8 and A9 to A13, which had none or two under revision 7, have one now
(c7-1, c7-2, c7-7); row 15 fails "unlisted case", the one check the probe's own list serves (rule 4, decided after
check 7); R1 and R2 have one answer each, the act's and then each run's (rule 3, decided after check 7); row 18's
release answer and A5's are "may go" (c7-6); A7 is caught by a unit run, which no pre-sweep mean stops (c7-5).

**Not covered.** No build, test or measurement. `/proc/loadavg` was not read on the machine; its fourth field is
named from general knowledge, and task P's script checks it there. A single-threaded pass's slowdown at a runnable
count near 10 was not measured. `revision8_situations.py` models the rules' text, not the step;
`revision7_situations.py` stays as revision 7's model, which check 7's script imports.

### 8.7 The seventh check (`load-design-check7.md`)

Every finding was verified at its evidence: this note's own lines as revision 7 left them, at each place check 7
cites; `native-theme/tests/timings.rs` at 814e96e7 (`:345-357`: with no preset named, the probe runs each preset of
`PresetId::all()`, so it enumerates its own cases); `src/resolve/mod.rs:1550-1566` and `:1969-2001` at 7430a0a2
(`resolve` records what it read and calls `classify(last, &recorded)`; `classify` compares the layers through
`same_layers`, the preset, the left-out accent, the reader's values, the scheme and the contrast, then the reader's
conditions, the window and the flavour, so a mutation gives a new handle with no code of task 3 in it);
`revision7_situations.py` and check 7's `load-check7/added_situations.py` run again, which print the answers check 7's
table gives (A8 passing the release run, A5 failing it "stale", A7 "not compared" in a unit run). All hold. The
orchestrator decided the corrections; they are applied at every passage the check lists. Taken otherwise, or with a
choice the brief left open:

- **c7-1.** Taken as decided: the file defines the cases, the step asks the probe for every armed case by name, and
  one asked for and not reported fails either run as "not reported". Only armed cases are asked for: a case "first
  measured by" a task not yet done cannot be measured (line 10 before task 4b, the plain route on a pending preset),
  and asking for it would fail every run. The unit run that runs no sweep went with the pre-sweep mean (c7-5), so
  every run asks for every armed case. *What it leaves*: the probe's own list decides nothing, so revision 7's rule
  that a case in the output the file does not list fails (c6-4) goes; such a case is never asked for and not read. A
  case the probe could measure and the file does not list (c6-4's ninth preset, a new case of line 4, a new reader
  fixture) is then not measured, and no run says so: the file's completeness rests on the acts that write it (2.4
  *Who makes records*). *Closed after the check*, as the orchestrator decided: the probe's own list serves one check
  and no other; the file lists every case the probe can measure (armed, "first measured by" a named task, or not
  binding), and a case the probe can measure that the file does not list fails either run as "unlisted case", a
  file defect (2.4 rule 4; 8.6, row 15).
- **c7-2.** Taken as decided, check 7's reading (b). The rule for the bound states the condition the orchestrator
  set (three quiet sweeps at most half the bound) and raises the bound until it holds; no number is chosen, since
  none is measured (twice the three sweeps' time is the smallest bound that meets it).
- **c7-3.** Taken as decided. The release task's named miss leaves the case's record as it was, so the line stays its
  bound (rule 1, as under a raised line): the miss names the cause and bounds nothing tighter, and an act that records
  the case later keeps it (a record over the line) or removes it. The procedure applies to a case without a named
  miss: a miss under a raised line prints the same mark (A4), and a second miss would be a file defect (c7-7). The
  check's wording slip ("a case over its line passes only under its own named miss … only up to its record") is
  corrected in *What it gives* with the check's text. The check's proposed sentence "No act is owed for it" is not
  taken, since the orchestrator named one.
- **c7-4.** Taken by the simpler way: the count is taken whole, the limit of 10 including the step's own threads.
- **c7-5.** Taken further than the check proposed: the pre-sweep mean goes, rather than staying with its use
  stated. In the release run it decided no verdict that the per-pass means do not reach (they decide whether a pass
  counts, the bound when the sweeping stops); in a unit run it only withheld comparisons, A7's among them. *What its
  removal costs*: a unit run on a saturated machine (over 32 runnable) runs its three sweeps at nice 19, starved,
  where revision 7 stopped it; how long that takes is not known before task P records a sweep's length. The release
  run sweeps again over the times short of two with no pause between sweeps; each pass still counts only where its
  own mean is at most 10.
- **c7-6.** Taken: the stale test applies under a named miss whose record exceeds its line, where the record is the
  bound. A miss whose record meets a raised line is removed by the next act that records the case, as before.
- **c7-7.** Taken: both fail either run as file defects. "One miss may name several causes" is stated with the
  limit, so that a case with two causes has a way to say so.
- **c7-8.** Taken in *Where the numbers come from* and in c6-1 above; 8.6's caches, temperature and writeback stay.
- **c7-9.** Taken: revision 5's item 4, c3-11, and row 10 (rewritten with the table). A grep of the note for
  "pre-sweep", "less 2", "waits", "step ran", "probe's output", "cannot raise", "evidence for" and "load average"
  finds no other live rule; the remaining hits are the revision lists' history, which the later lists and the body
  override, section 8's history, marked with the revision that changed it, or the reason the average is not used.
- **c7-10.** Taken in a script of its own, `load-check/revision8_situations.py`, which models revision 8's text with
  the check's corrections (an armed time not reported fails either run; "not binding" modelled, with and without
  its reason; the bound by the sweeps' start times) and runs the nineteen situations and A1 to A15, each with one
  answer per run (8.6's table). `revision7_situations.py` stays as it was, revision 7's model, since check 7's
  `added_situations.py` imports it unchanged; its two departures (row U, "not binding") are fixed in the new script.
- **c7-11.** Taken: task P's self-test gains the release run's further sweep after one counting pass of three, the
  unit run's two of three compared, and the raised-line case with its record under L / (1 + t) and a figure over
  the line by more than its tolerance failing; with revision 8's own cases (not reported, the two file defects, the
  bound, the raised line's absent stale test).
- **c7-12.** Taken by the first way: "may move", judged before the change is written, which reaches task 3's lines
  3, 5, 6 and 11. The check's further clause, a re-record within the old record's tolerance keeping the old record,
  was not added with the rest, since it changes which records the acts write and was not decided then: re-recording
  a named-miss case the change did not move gives a new lowest of five runs, which noise can raise, and a raise must
  state its cause. *Closed after the check*, as the orchestrator decided: a re-record of a case under a named miss
  keeps the old record where the new runs' lowest figure lies within the old record's tolerance, and only a record
  that changes needs a cause (2.4 rule 3; 8.6, R1 and R2).
- **c7-13.** Taken.

**Not covered.** No build, test or measurement. A sweep's length is unknown until task P, so the bound's first test
and the unit run's length on a saturated machine are not sized. The two items this section left open, the gap c7-1
left (a case the file does not list) and the re-record clause c7-12 left, are closed by the orchestrator's decisions
(c7-1, c7-12 above; revision 8's item 9).
