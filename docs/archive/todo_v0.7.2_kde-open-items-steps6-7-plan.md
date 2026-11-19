# 0.7.2: issue #4's open KDE items, steps 6 and 7 (the default button; the table header's states and the unfocused selection) — implementation plan

- Rationale: [`../issues/0004_kde-open-items-rationale.md`](../issues/0004_kde-open-items-rationale.md),
  D8 (step 6), D7's second half and D9 (step 7). Specification:
  [`todo_v0.7.2_kde-open-items-steps6-7-spec.md`](todo_v0.7.2_kde-open-items-steps6-7-spec.md)
  ("the spec"), whose 6.I and 7.J this plan carries out.
- Decided by the maintainer on 2026-11-19: steps 6 and 7 are part of 0.7.2, as steps 2 to 5 are,
  together with the text caret's blink
  ([`todo_v0.7.2_caret-blink-plan.md`](todo_v0.7.2_caret-blink-plan.md), "the caret plan"); the
  version stays 0.7.2, which is not released yet. (That morning the works were set for a release
  of their own, 0.7.3; he folded them into 0.7.2 so that no release changes the API a few days
  after another did.) The caret plan's units are built first, on the same branch; this plan's
  follow and end with 0.7.2's close-out, made again once, for all three works.
- Written 2026-11-19. It replaces the plan of 2026-11-14 for a release of its own in a lane
  beside another release (git history keeps it). Status: approved by the maintainer on 2026-11-19.
- `<record>` = `~/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/todo_v0.7.3/`, the
  record of both works (the folder keeps the name it got while they were set for 0.7.3), with this work's files in `<record>/steps67/`, the amendment's in
  `<record>/respec/` and the trials' in `<record>/trial/`. `<tools>` =
  `/home/tibi/Rust/native-theme/.superpowers/sdd/todo_v0.7.2_egui-0.37-and-gpui-kit-0.10.0-plan/tools/`,
  0.7.2's wrappers, used as they are.

## 1. Where and from what

- One worktree, `.claude/worktrees/lane2-b`, branch `v0.7.2-rc1`, from the head the caret plan
  leaves (its last `--commit` gate passed and pushed, its reviews closed). The units are commits
  on that branch: no new branch, no merge, no version step (the workspace is at 0.7.2 since
  `b434b9bc`). The branch is pushed by explicit refspec at each
  `--commit` gate that passes, and each push's CI run is read (the macOS and Windows readers'
  tests run only there; a red run emails the maintainer).
- The evidence already taken (2026-11-19):
  - `<record>/respec/findings-k02-6ABC.md` to `findings-k09-7H.md`: every anchor, count and
    upstream citation of the spec's parts read again on 0.7.2's tree by eight `worker` agents
    (no cargo). Nothing is built yet and nothing is impossible; the spec holds the corrections.
    The counts the units start from are in those files (egui's 525 leaves and its verdict
    tuples, the KDE reader's 39 derived leaves, the iced contract's rows, the 201 listed
    elements).
  - `<record>/trial/t-k1-result.txt`, `t-k1.patch` (T-K1): a custom variant built as the spec's
    6.D.2 says draws Breeze's default button at rest, hovered, pressed and disabled, under
    kde-breeze light and dark, on gpui-pre and on GPUI Fast, with no differing state. gpui has a
    receiver; 6.D holds one case. The patch's test is where K6b's starts.
  - `<record>/trial/t-k2/` (T-K2): the default button's place on the Basic page, measured
    headless in the three showcases (the spec's decision 7 and 6.G). Its patch shows K6b to
    K6d the button's place in their showcases; `compare-built.py` and `rehearsal-t1.sh` beside
    it are the check on the built tree's.
  - `<record>/trial/t-k3-result.txt`, `t-k3.patch` (T-K3): in one gpui test a window is made
    active, inactive and active again, `is_window_active` and an activation observer following
    each change, on both backends; a newly opened test window is not active. K7b's window
    tests start from the patch's test.
  - `<record>/trial/t-k4-result.txt` (T-K4): the two carried files, copied in as they are and
    declared as the spec's 7.B.3 has them, pass `scripts/check_strict_panic.sh` with the gate's
    features; clippy's one finding is `kcu_tint` unused, which the reader's call removes.
  - The twelve macOS captures of 0.7.2's first asset run (`b494d6ed`) counted for the active
    selection fill: the baseline of the
    capture check (the spec's 7.H.2).
- Three local branches hold evidence commits no remote has: `facts-ios-m1b3` (the caret plan),
  and `checkfacts-perf` and `facts-macos-m1b4`, which alone hold `71b4c68a` (the spec's 7.C2).
  None is touched before K7a is committed.

## 2. Units

Each unit is one commit; a review fix is a commit of its own, covered by the next gate. The
spec's 6.I.1 and 7.J.1 say why the units are cut so.

| Unit | The spec's parts | Agent | Gate after it |
|---|---|---|---|
| K6a | Step 6's core: 6.A, 6.B, 6.C | `implement` | `cargo test -p native-theme` with the gate's features; the five documentation checkers. Not `--quick`: the connectors are red until K6b to K6d |
| K6b | gpui: 6.D; 6.G's gpui showcase | `implement` | none of its own: built beside K6c, K6d and K6e |
| K6c | iced: 6.E; 6.G's iced showcase | `implement` | none of its own |
| K6d | egui: 6.F; 6.G's egui showcase | `implement` | none of its own |
| K6e | Step 6's documents: 6.H | `worker` | after K6b to K6e: the check on the built tree and the rehearsal R (below); then one `--commit` with the GPUI Fast legs and the five documentation checkers over K6a to K6e; push; CI read |
| K7a | Step 7's cores, by two builders one after the other: first 7.A, 7.C, 7.C2, 7.D.3 to 7.D.5; then 7.B, 7.D.2, 7.D.6 | `implement`, twice | after the second: `cargo test -p native-theme` with the gate's features; the five documentation checkers; the macOS reader type-checked by the cross-target legs of `scripts/check_release.sh` for `x86_64-apple-darwin` |
| K7b | gpui: 7.E; 7.H's gpui showcase | `implement` | none of its own: built beside K7c, K7d and K7e |
| K7c | iced: 7.F; 7.H's iced showcase | `implement` | none of its own |
| K7d | egui: 7.G; 7.H's egui showcase | `implement` | none of its own |
| K7e | Step 7's documents: 7.I.1 to 7.I.5 | `worker` | after K7b to K7e: `check_showcase_parity.py --check-list`, the check on the built tree and R; then one `--commit` with the GPUI Fast legs and the five documentation checkers over K7a to K7e; push; CI read |
| K | The close-out (7.J.4): its checks before it, the archive, the compatibility run, the push, the full check beside CI, the asset run with its capture window and the capture check, the assets' commit | orchestrator | the five documentation checkers after the archive; the full run, twice |

Before each block of three the orchestrator makes the block's edits of
`docs/showcase-elements.toml` from the spec's text (6.G; 7.H.1 and 7.H.2), uncommitted, so that
the three builders start from the same list; they go into the block's commits by path.

**The check on the built tree**, after each block's reports and before its `--commit` (the
spec's 6.G gives the commands): the three release showcases are built through `slot.sh`, the
nine entries are dumped headless under `xvfb97.lock` by `<record>/trial/t-k2/rehearsal-t1.sh`
into `<record>/steps67/dumps-6` (step 7: `dumps-7`), and `<record>/trial/t-k2/compare-built.py`
compares them: step 6's with T-K2's "after" dumps, step 7's with step 6's (step 7 changes
fills, not places). Expected both times: exit 0, no stop entry, no parity entry, no moved line.
A stop entry or an input fault (exit 2) is a stop; a parity entry or a moved line is fixed or
explained in the record before the gate.

**The rehearsal R** (0.7.2's: `<tools>/rehearsal.sh`, read by
`scripts/check_showcase_parity.py`), on the same binaries. Expected new keys: those of
`basic.buttons.dialog_default`, which join the groups the Buttons rows' other buttons have
(6.G); any other new key is fixed in code or excepted with its reason and source before the
gate.

**The capture check, first run**, after step 7's push: the screenshots workflow is dispatched
on the pushed head, its macOS artifacts are downloaded into `<record>/steps67/captures/` and
their twelve captures counted (the spec's 7.H.2). A capture below the count stops K until the
capture-code fix is made, reviewed and gated, and the workflow's new captures pass.

After step 6's push the session dispatches the same workflow on that head and looks at the
Basic page in its macOS and Windows captures (the spec's 6.I.1): the button's line on the
platforms' own fonts. Nothing of either run is committed.

## 3. Tests and gates

- During a unit: the tests the spec names for it and the ones its edits reach, through
  `<tools>/slot.sh <tools>/lowprio.sh cargo test -p <crate> … <names>`; a showcase's tests through
  `scripts/run_in_runner_env.sh`; tests that read painted primitives in the GPUI Fast form too. A
  test the spec says fails without the change is seen failing first.
- Before each unit's report: `scripts/check_strict_panic.sh` for the crates it changed. K6b's and
  the showcase builders' briefs also say `git grep -n trial_` over their paths finds nothing (the
  trials' names).
- At a gate the orchestrator runs `<tools>/gate.sh [--fast] lane2-b <name> <args>`: two
  `--commit` gates (step 6, step 7), each 7 to 21 minutes as measured in 0.7.2, and the full
  run twice in K (about 23 minutes each, measured at 0.7.1).
- The whole suite and the release checks run in K only. Before K's compatibility run and before
  its asset run the session's own rust-analyzer is stopped: it re-reads `Cargo.lock` while the
  scripts rewrite it, which failed the compatibility run three times on 2026-11-19.

## 4. Reviews

A reviewer is given the commit and the spec's parts, never the author's report; it runs beside
the next unit. A must-fix is a commit of its own before the next gate. At most one Fable agent
runs at a time.

| Commit | Reviewer | Why |
|---|---|---|
| K6a | `review` | the facts rows against 6.B.3's texts, kde-breeze's values, the `mix_qt` split whose body stays, the tests that pin the 16-bit frame; not `review-hard`: the arithmetic is the reader's own, moved |
| K6b, K6c, K6d | three `review` agents at once, beside K7a | each connector's per-state rule against the spec's tables (6.D.2, 6.E.2, 6.F.2), gpui's claims at the lines that read their fields, the regenerated leaves file against 6.E.3's list, egui's focus before hover and the pressed button without fill |
| K7a's resolver and validation hunks: `native-theme/src/resolve/inheritance.rs`, `resolve/validate.rs`, `model/resolved.rs`, `model/defaults.rs` | `review-hard`, narrow | a new inheritance rule and a required resolved field whose order against `selection_text_color`'s resolution decides every theme's value |
| The rest of K7a | `review` | the facts rows against 7.C.3's texts, the evidence files' hashes, the presets' values, the macOS reader's one field; the carried files as byte copies by hash with their notice, the three-branch rule and the boolean parse, each case a test (not `review-hard`: the tint's arithmetic is carried and the vector test holds the reader's rule to all 70 rows of KColorScheme's table) |
| K7b, K7c, K7d | three `review` agents at once, beside the capture check's first run and the link and path checks of 7.J.4's item 1, which ends when these reviews are closed | the window-state plumbing in three showcases, the signature changes with every caller, the tests that make a window active or inactive |
| A capture-code fix, should the capture check ask for one | `review` | it changes the capture code all three showcases use on macOS |
| K6e, K7e, K's commits | the checkers, then the orchestrator reads the diff | data and texts the spec fixes |

Every reviewer also checks that no new code transcribes an LGPL or GPL source, and that the
carried files are 7.B.1's bytes with their notice.

## 5. Briefs

The caret plan's form and rules (its §5), and: the readers and the carried modules are written
from the spec's text and the parked branch's clean-room files alone; Breeze's, KColorScheme's
and Qt's sources are not opened. A showcase builder's brief names its hunk of `t-k2.patch` as the place of the button and
nothing more (the button, its kinds and its helper are the spec's 6.G; the patch's own hunk of
`docs/showcase-elements.toml` is not applied), and says the element list is edited already. In a block of three the brief also says: other
agents edit other folders of the same tree at the same time; change, format (`cargo fmt -p
<crate>`) and stage nothing outside your own paths, which the brief lists.

## 6. Stops

- An anchor that is not found as the spec quotes it: the orchestrator finds its present form;
  where the edit's meaning is unchanged it is made there and the record names it, else the work
  stops.
- A stop entry or an input fault of the check on the built tree; a strict-panic finding or a
  hook's refusal in a carried file (an ordinary clippy finding there is fixed and named in the
  commit body, the spec's 7.B.5); an evidence object of 7.C2 that `git cat-file -e` does not
  find.
- A decision neither the spec nor the record makes; a task that needs more than its parts name;
  a new upstream release or advisory during the work, which is reported with its cost before
  anything is changed for it.
- A unit that fails the same gate twice is not sent a third time: the orchestrator takes it
  over, re-specifies it, or sends it once as `implement-hard`. K6a and K7a are never taken over
  by the orchestrator, which has read LGPL sources for the facts: always a fresh
  `implement-hard`.
- A macOS capture below the capture check's count (7.H.2): after step 7's push it stops K's
  start, in K the assets' commit; a fix
  the close-out needs follows the batch's rule 15.

## 7. The maintainer's part

- The approval of D8, D7's second half and D9, the spec and this plan.
- Told, not asked: where the default button stands on the Basic page and where it wraps
  (decision 7, T-K2); that the macOS capture check reads 0.7.2's captures as its baseline and
  changes no workflow (7.H.2); that the reply on the issue is posted, so no step edits it, and a
  text for a follow-up comment is in 7.I.3, his to post or not; 0.7.2's new ROADMAP heading
  (the caret document's E.3), his to change.
- The capture window asked for on 2026-11-19 for 0.7.2's repeated asset run is withdrawn: that
  run is K's, after the units.
- In K: the capture window, once, 4 to 6 minutes, asked for when everything before it holds;
  what the new captures show. A second window only where a fix follows the asset run (the
  batch's rule 15); the capture check's first run, after step 7's push, is there to find an
  inactive macOS window before K.
- After K: the merge of the pull request (#11), the tag `v0.7.2`, the publication and the GitHub
  release; the pull request's title and description, which the orchestrator brings up to what
  the release holds.

## 8. Cost check

- **What could refute the design, and is it first.** (1) Whether gpui can draw Breeze's default
  button with a custom variant: T-K1, done, yes. (2) Whether the default button fits the Basic
  page: the first placement (a row of its own) was refuted by 0.7.2's dumps; T-K2 measured the
  one the spec now has. (3) Whether the upstream lines still say what the spec builds on: read
  again at gpui-component 0.10.0 and egui 0.37.0, yes. (4) gpui's test windows made active and
  inactive in one test: T-K3, done, yes. (5) The carried files under this tree's lints: T-K4,
  done, they pass. (6) Whether a macOS runner's window is active when it is captured: the
  captures of 0.7.2's first asset run say very likely yes; the capture check reads it on a dispatched run after step 7's
  push, before the close-out, and again in the asset run. (7) The button's line on macOS's and
  Windows's own fonts: a dispatched run after step 6's push shows it. What waits for these:
  nothing.
- **Built twice:** nothing. T-K1's test is where K6b's starts, and T-K2's patch gives the
  button's place. The old plan's two desktop
  measurement blocks are replaced by T-K2 and the check on the built tree, headless; its
  workflow edit and macOS layout dumps by the capture check's baseline.
- **In parallel:** each block of three connector units with its documents unit: four agents,
  three of them building through `slot.sh`, which lets two cargo commands run at a time. The
  three reviewers of a block run no cargo and work beside the next core unit's one builder.
- **What each agent reads:** its parts of the spec, the decisions named in its brief, its part's
  findings file, its trial's patch; not the rationale, not this plan.
- **Tests per round:** the unit's own; one `--commit` per step; the check on the
  built tree and R once per step; the full suite twice, in K.
- **Time (an estimate):** 0.7.2's three earlier folds each took 1 h 40 to 1 h 55 from the
  documents' commit to the last unit's, and its close-out 2 h 30 (measured from the commits'
  times). Here step 6 is two to three hours and step 7, with K7a's two builders and the capture
  check's workflow run, three to five, their gates and checks on the built tree included; then K,
  about two and a half hours with its waits. With the caret plan's three to five: 11 to 16 hours.
- **Compaction:** before K6a (the caret plan's end); after step 6's gate; after step 7's gate,
  before K; earlier where the conversation passes about 300 k tokens.
