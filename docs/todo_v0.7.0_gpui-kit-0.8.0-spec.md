# v0.7.0 — GPUI Kit 0.8.0 (G3): Specification

**Status:** design, drafted 2026-10-21, self-checked; revised the same day for the
maintainer's decision on D1 (§12), self-checked again; checked independently; approved by
the maintainer on 2026-10-21, as D11 in [`todo_code-review-plan.md`](todo_code-review-plan.md)
records; being implemented as milestone G3. Reasons:
[`todo_v0.7.0_gpui-kit-0.8.0-rationale.md`](todo_v0.7.0_gpui-kit-0.8.0-rationale.md)
(K1–K18). Tasks and gates: [`todo_v0.7.0_gpui-kit-0.8.0-plan.md`](todo_v0.7.0_gpui-kit-0.8.0-plan.md).
*Added 2026-10-21 (G3 T1 fix round 1):* the rationale's K19, Task 1's deviation (§2.4), follows
K18.

Abbreviations as in the rationale: `GK8`, `GC7`/`GC8`, `GB7`/`GB8`, `GF7`/`GF10` (registry
sources, `~/.cargo/registry/src/index.crates.io-*/<crate>-<version>/src/`), `CN` =
`connectors/native-theme-gpui/`, `SC` = `CN/examples/showcase-gpui/`, `MUX` =
microsoft-ui-xaml at `8463f45162149de0ec3ad7df752596893fe3e13e`. Our line numbers are of
`v0.7.0-rc1` at `7492e62f`, §12's of `a172d3dd`, or of `2e80f87e` / `fa90c6c0` where so
marked (the iced showcase by function name, as it
changes with nearly every M9b commit); locate by content, since M9b commits land before G3
starts.
`docs/todo.md` entries are cited by section, heading and opening words, as its header
requires.

Inputs: the investigation reports `gk080-whats-new.md` and `gk080-trial-build.md` (the
trial's appendices list the 184 colour claims and 15 prose citations that fail),
`gpui-fast-panic-diagnosis.md` and the check passes of these documents, kept in
`.superpowers/sdd/todo_code-review-plan/g3/` (the review's SDD directory, on the project
disk, untracked as the rest of it; copied there when the documents are handed over, plan
Task 0), where the documents' references by name point. Nothing a task needs is
only there: every count and example taken from them is restated here, and the regression
test's code is Appendix A. No source file (code, tests, READMEs, manifests) cites these
documents or the reports by path; the review plan and ledger link the documents, and follow
them to `docs/archive/`.

---

## 0 -- Scope and constraints

### 0.1 Deliverables

The gpui connector, its tests and its showcase on GPUI Kit 0.8.0 / gpui-fast 0.1.10 (§1);
the GPUI Fast test gates (§2); every upstream citation true for 0.8.0 (§3); the nested
scrollers without needless occlusion (§4); `TabVariant::Folder` in the showcase (§5); the
slider's accessibility (§6); the side panel's width on resize (§7); the other new items,
each decided (§8); the GPUI Fast run-time comparison (§9); documents (§10); D1, the selected
tab's outward corners as theme data, `windows-11`'s value and their drawing in all three
connectors and showcases (§12).

### 0.2 Constraints

- No `unwrap` / `expect` / `panic!` / `unreachable!` / slice indexing / unchecked
  arithmetic outside tests; no `unsafe`. Strict panic lints (`scripts/check_strict_panic.sh`)
  on the library and on the showcase example, both cores.
- No invented value: every number in a claim, a comment or a document is read from the
  cited source or measured; an unsourced value stays unstated.
- Connectors and showcases follow theme data: the Folder sample's colours and fonts come
  from the theme (§5); nothing picks a variant by preset name; the folder shape is drawn
  from `tab.active_outward_corner_radius` wherever a connector or showcase draws a tab bar
  itself (§12); an iced application wraps its own bar in `tab_flares`, as it lays
  `tab_indicator_line` (rationale K15).
- A task that edits a file M9b or M9c also writes (the iced connector and showcase, the egui
  connector and showcase, the shared lists): it locates its regions by content; at its
  start it re-reads the code this spec describes there and checks the description still
  holds; where it does not, it stops and reports the difference before writing (plan,
  *Global Constraints*). This spec names such code by its role as well as by its present
  name ("the default build's Basic tab row, since `99af695b` in `basic_tab_row`"), so a rename
  or a move
  that keeps the role and the behaviour is no stop; a changed behaviour is.
- Every change applies to both cores: tests run on gpui-pre (`--features native,watch`)
  and on GPUI Fast (`--features gpui-fast,native,watch`).
- `docs/todo.md` is appended to, never rewritten; an entry that closes moves to
  `docs/archive/todo_closed.md` with a dated line naming the closing commit, as that file's
  header says.

---

## 1 -- Floors, lockfile, MSRV, audit, README floors (K1, K2)

- `CN/Cargo.toml`: `gpui-component` `"0.7.3"` → `"0.8.0"` (`:66`), `gpui-base` (`:70`),
  `gpui-kit` normal (`:75`, `default-features = false` kept) and dev (`:104`, features kept)
  → `"0.8.0"`. `gpui = { package = "gpui-pre", version = "0.3.9" }` unchanged.
- Its comments, each re-verified: the `rust-version` comment (`:6-10`): the measurement's
  date and "on the 0.8.0 closure"; `cold_path` is still at `src/profiler.rs:473` in gpui-pre
  0.3.9 and in gpui-fast 0.1.10. The `gpui` comment (`:56-62`; the hard-floor comment at `:64-65` stays true): "gpui-component 0.8.0
  depends on `gpui-pre = "=0.3.9"`" (`GC8` `Cargo.toml:278-280`). The `image` comment
  (`:76-82`): "gpui-pre 0.3.9 and gpui-fast 0.1.10 each ask for `image` 0.25.1 and up"
  (gpui-pre `Cargo.toml:355-356`, gpui-fast 0.1.10 `Cargo.toml:186-187`), resolving 0.25.10.
- Lockfile: `cargo update -p gpui-kit -p gpui-component -p gpui-base` on the committed lock
  (after M9b's last lock commit). Gate: `git diff Cargo.lock` changes exactly the 15
  packages of rationale §1.1 (5 at 0.8.0, 9 at 0.1.10, `gpui-pre-ios` added), or the
  difference is explained in the commit body. `cargo tree -p native-theme-gpui -e normal`
  before and after in the commit body.
- MSRV, measured with the installed toolchains, the lines of CI's `msrv` job:
  `cargo +1.95.0 check -p native-theme-gpui --locked`, `… --features native,watch --locked`,
  `… --features gpui-fast,native,watch --locked` pass; `cargo +1.94.0 check -p
  native-theme-gpui --locked` fails (on `cold_path`), which shows 1.95.0 is the lowest. If
  1.95.0 fails, `rust-version` becomes the lowest toolchain that passes all three, tried
  release by release upwards (one not installed is installed only with the maintainer's
  permission: rustup writes outside the project), and the README row, the comment and
  CHANGELOG follow.
- `cargo audit` (it reads `.cargo/audit.toml`) passes on the new lock. The file's comments
  name the crates that hold each ignored advisory (`:11-12` instant, `:14-15` paste,
  `:17-20` rustls-pemfile, all "gpui-… 0.7.3"): each re-read with `cargo tree --workspace
  -i <crate> --target all` on the new lock and its version names updated; an entry the
  lock no longer needs is removed, as the file's header asks.
- `CN/README.md` Compatibility table (`:76`, `:77`, `:79`) and Quick start (`:109`): 0.8.0.
  Gates: `compat::the_readme_states_the_manifest_floors`,
  `compat::the_quick_start_states_the_manifest_floors`. The table gains no gpui-fast row
  (rationale K1); §10 states the version in *Backends*.
- Compile gate: `cargo check -p native-theme-gpui --all-targets` on both cores, no warning;
  the form-A check `cargo check -p native-theme-gpui --lib --features
  gpui-kit/gpui-fast,gpui-kit/component` passes (an application that turns on GPUI Fast in
  gpui-kit only).

## 2 -- GPUI Fast 0.1.10 test gates (K4, K5, K6)

### 2.1 The regression test of the stale child-list crash

- `SC/tests.rs`: `a_sidebar_item_losing_and_regaining_its_icon_is_drawn`, the code of
  Appendix A (the diagnosis's §3c, verbatim), its doc comment saying what it guards: a Layout-page Sidebar item drawn
  without its icon, with it, and without it again, under presets of different text sizes,
  is laid out and drawn each time (gpui-fast 0.1.7 panicked in `TextLayout::prepaint`;
  fixed in 0.1.9, which GPUI Kit 0.8.0's floor of gpui-fast 0.1.10 includes). The helper
  names it uses exist at `7492e62f` (`apply_theme_by_name`, `select_icon_set`,
  `gpui_icons`, `chrome_icon(..).shown()`, `TALL_WINDOW`); the implementer adapts it to the
  tree it lands on.
- Red first: before the floor edit, on the lock's gpui-fast 0.1.7, `cargo test -p
  native-theme-gpui --features gpui-fast,native,watch --example showcase-gpui
  a_sidebar_item_losing` fails with the `measurement has not been performed` panic (output
  in the commit body); after it, it passes on both cores.
- The CI case: after the bump, the showcase test binaries of both cores pass whole in the
  stripped environment the diagnosis used (`env -i HOME=<a fresh empty directory> <binary>`,
  the directory made for the run, in the running session's scratchpad), in
  which `the_card_and_the_group_boxes_name_the_container_margin_where_it_pads` failed in CI
  run 39490500851. The gpui-pre binary too: CI's gpui-pre leg runs in the same environment,
  and 0.8.0's behaviour changes could expose another host-dependent test.
- Not done (rationale K4): the showcase's distinct-id workaround, an `[upstream]` entry for
  the crash, a CHANGELOG line.

### 2.2 Reading painted quads on both cores

- *Added 2026-10-21 (G3 T1 fix round 1), the controller's ruling, which overrides the next
  item's "one helper in each test module":* the helpers (`scene_quads`, `scene_underlines`,
  and the clip-group descent) are in one file, `CN/src/widgets/scene_read.rs`, which both
  test modules include with `#[path]`, so a fix to the descent is made once; the mutation
  check runs against that one copy, from both modules.
- One helper in each test module, `CN/src/widgets/tests.rs` and `SC/tests.rs`, named alike
  (`scene_quads(window: &Window) -> Vec<Quad>`), each doc comment naming the other copy:
  - default core: `window.painted_quads()`;
  - `#[cfg(feature = "gpui-fast")]`: the quads of `window.painted_scene()`
    (`GF10/fast/rounded_clip.rs:232-237`), followed by those of every clip-group frame in
    its `layers.frames` — a `LayerFrame` with `tile_size == 0` whose `content.scene()` is
    `Some` (`GF10/fast/rounded_clip_scene.rs:1-10`, `:60-75`; `GF10/fast/layers/scene.rs:80-93`,
    `:186`) — descending into each such scene's own frames the same way. Tiled frames
    (`tile_size != 0`, scroll layers) are not followed: every clip group's scene holds a
    copy of the top-level ones (`rounded_clip_scene.rs`, `new_scene`), so following them
    would count their content more than once. If Task 1's full run on 0.1.10 shows a test
    that needs a primitive held only in a tiled frame, the helper follows the top-level
    tiled frames once, and the commit body names the test. That contingency only shows up
    as a failing positive assertion: an absence test over a composited scroll layer stays
    vacuous, a gap older than 0.1.10 (0.1.7's `Scene` already had `layers`), which the
    helper's doc comment and the `[upstream]` entry below state. The order of the result is
    not paint order, and the doc comment says so.
- Every read that finds a quad by bounds or colour, or asserts that none is painted, uses
  it: `CN/src/widgets/tests.rs` `painted_fill` (`:2161`), the tab-row `painted` closure
  (`:632`), the switch's `edge` (`:1248`); `SC/tests.rs` `:3151`, `:3172` (`hover_box`),
  `:4767` (`tooltip_boxes`), `:8181` (`painted_fill`), `:8224`, `:8254` (`painted_edge`).
  A clip group's scene takes every kind of primitive (`insert_primitive`,
  `rounded_clip_scene.rs`), so the `painted_underlines()` reads (`SC/tests.rs:2660`,
  `:11507`) go through a second helper of the same shape, `scene_underlines`.
- The order-sensitive read (the switch's outline painted over its thumb,
  `CN/src/widgets/tests.rs:1277-1291`) keeps `painted_quads()`, where paint order is
  defined, and no longer skips when a quad is missing: where the edge assertion above it
  found the stated outline, both positions must be found (an assertion, not `if let`).
- Gate: `a_loading_progress_bar_stands_still_under_reduced_motion` passes on GPUI Fast
  0.1.10; both suites pass on both cores. A mutation check, run once and recorded in the
  commit body: with the helper's clip-group descent removed, that test fails on GPUI Fast.
- `docs/todo.md`, *native-theme-gpui connector*: an `[upstream]` entry for the maintainer
  to file on gpui-fast — `Window::painted_quads()` (test-support, `GF10/window.rs:2925`)
  omits the quads painted inside a rounded overflow clip, which go into a clip-group
  `LayerFrame` since 0.1.10, and the primitives of composited scroll layers; our tests read
  `painted_scene()` instead (`scene_quads`), and the descent can go once `painted_quads`
  includes them.

### 2.3 `window.refresh()` in the draw helpers

- Take `window.refresh()` out of `SC/tests.rs` `draw` (`:143-151`) and
  `CN/src/widgets/tests.rs` `redraw` (`:1032-1040`). Run, on GPUI Fast, the five tests the
  todo entry names (`the_dragged_width_survives_the_toggle`,
  `toggling_the_side_panel_mid_drag_is_safe`, `the_inspectors_theme_tab_lays_out`,
  `the_status_bar_names_the_hovered_widget`,
  `under_client_decorations_the_title_bar_holds_the_menus`), then both whole suites on both
  cores.
- All pass: the calls and their comments go, and so do the notes in both modules' headers
  that tell future helpers to call it (`SC/tests.rs:9-12`, `CN/src/widgets/tests.rs:5-8`).
  The `window.refresh()` after a test writes the `Theme` global (`SC/tests.rs:8688`, no
  view notified) is a different use and stays. `docs/todo.md`, *native-theme-gpui connector*, "File the
  gpui-fast issue: in test mode a partly reused frame" closes (fixed in gpui-fast 0.1.9,
  #74/#75; the carry-forward at `GF7/fast/retained.rs:1360-1371` is gone in 0.1.10), not
  filed.
  *Added 2026-10-22 (G3 T6 fix round 1), on "#74/#75":* the fix is PR #74; the v0.1.9 notes
  list it with #75, which aligns composited scroll content's bounds.
- Any fails: the call stays in the failing helper's module with a comment stating the 0.1.10
  cause the failure shows (read in `GF10/fast/retained.rs` / `GF10/window.rs:4119-4144`), and
  the todo entry gets a dated line instead of closing.
- *Added 2026-10-22 (G3 T4), the controller's ruling:* the five pass without `refresh()` on
  GPUI Fast, and gpui-pre's suites pass unchanged. Three GPUI Fast tests failed, and none of
  them shows a GPUI Fast cause. Each was a test that leaned on the forced full frame, so the
  "any fails" rule above, which assumed such a cause, does not apply. Instead the calls go,
  as under "All pass", and the todo entry closes. The three tests change:
  - `a_checkbox_toggles_on_click_enter_and_space`: the harness's `on_change` notifies its
    view.
  - `a_loading_progress_bar_sweeps_its_fill_in_the_theme`: `fill_after` delivers the
    animation's next frame through `Window::simulate_next_frame`, which both cores have
    under `test-support`.
  - `a_target_a_recordless_frame_took_away_does_not_win_when_drawn_again`: it draws the full
    frame §2.4 asks for with its own `window.refresh()`. The three partial-frame tests' docs
    say who draws the frame after the settle.
- *Added 2026-10-22 (G3 T5), on the line above:* "gpui-pre's suites pass unchanged" means
  that no gpui-pre test failed without `refresh()`; the three tests changed for GPUI Fast run
  on gpui-pre too, and pass there as changed. The fix the bullet "All pass" cites is gpui-fast
  0.1.9's, by its release notes: "remove debug bounds for elements no longer drawn" (PR #74;
  the notes list it with #75, which aligns composited scroll content's bounds).

### 2.4 Widget Info under GPUI Fast's partial frames (K4, K5, K6, K19)

*Added 2026-10-21 as a Task 1 deviation, adopted by the controller's ruling (rationale K19).*

- `SC/info/registry.rs`: a record, a target's or an element of the list's, holds a
  `Weak<Presence>`. Its recorder keeps the `Rc` in gpui element state (`begin_record`, called
  from the recorder's prepaint, under the recorder's id path and the record's id).
  The invariants:
  - drawn means the `Weak` is alive;
  - it is read only between draws (the frame's end, the settle timer, hover events, the
    layout dump, the tests);
  - one frame end per update that recorded: the update's first record schedules it through
    `Window::defer`. It updates the registry only when the frame changed what it decides
    (`frame_changes_anything`); in the tests, an assertion after `close_frame` checks that
    nothing is left for it to change.

  `epoch_marker` goes, from `SC/app.rs` and from the tests' `Nested`. The registry's module
  doc says what "drawn" means, with the composited scroll layer's exception, and what its
  updates cost (rationale K19).
- `record_target` drops its id from `hovered` when the record it replaces stands for an
  element no longer drawn (rationale K19, *Hover pruning*). `close_frame`'s doc says when the
  choice is revisited after a frame that recorded nothing.
- The tests' duplicate check counts draws by the alternation of paint and prepaint:
  `painted`, from a target's or a `record` box's paint. It also flags a fresh record beside a
  replaced holder still drawn at the frame's end (`displaced`). Its assumption is stated in
  the registry's doc: every draw that records paints a target or a `record` box; otherwise a
  false duplicate, never a miss.
- `CN/src/widgets/mod.rs`: `PartBounds = Rc<dyn Fn(Part, Bounds<Pixels>, &mut Window, &mut
  App)>`. Its doc says that a frame drawing the control from the last one does not call it,
  nor does one in which GPUI Fast composites a scroll area the control is in. The six
  `on_part_bounds` docs (`checkbox.rs`, twice, `switch.rs`, `slider.rs`, `progress.rs`,
  `tab_bar.rs`) say "as a frame lays them out ([`PartBounds`])". In `SC/elements.rs`, the
  recorders call `begin_record`, and `record` calls `painted`.
- Tests, in `SC/tests.rs`, passing on both cores:
  - `a_reused_views_target_is_hovered_and_listed_after_a_partial_frame` checks:
    - the host's renders at the notify, on both cores;
    - on GPUI Fast, the hosted view not built through the notify and the settle after it;
      on gpui-pre, built;
    - the hosted target still shown over the larger one around it;
    - the layout dump, and `drawn_twice` and `listed_twice` empty.
  - `a_target_a_partial_frame_no_longer_draws_is_gone`: the opposite direction.
  - `a_target_a_recordless_frame_took_away_does_not_win_when_drawn_again`: the hover pruning.
  - `the_duplicate_check_finds_two_targets_drawn_together` and
    `the_duplicate_check_finds_a_fresh_target_beside_a_reused_one`: the check's two paths,
    each asserting that it finds its id; the first checks an element of the list too.

  Each partial-frame test hovers and settles first, then draws a full frame. GPUI Fast builds
  again a view whose hover changed, or one that read the registry since an update of it, so a
  partial frame comes only after that.
- Red first: run the tests against the epoch registry on gpui-fast 0.1.10, and run the
  pruning test on the design's prototype, which had no pruning. Both outputs go in the
  commit body.
- Commit order (plan Task 1): this change is measured green on gpui-fast 0.1.7 on both cores,
  so it is committed before the bump. That keeps "expected red: the two showcase citation
  gates only" true.
- `docs/todo.md`, *native-theme-gpui connector*: the `[1.0]` entry "**The layout dump on
  GPUI Fast is stale inside a scrolled, composited scroll layer.**" `CHANGELOG.md`
  `[Unreleased]` *Breaking Changes*: the `PartBounds` line, with the composite caveat.

## 3 -- Citations (K3)

### 3.1 Renumbering — digits only

- **Scope:** every citation, in the forms below, in `CN/` (`src`, `tests`, `examples`,
  `README.md`, `Cargo.toml` comments), `docs/showcase-exceptions.toml` and
  `tools/showcase-support/src`, whose crate (below) is gpui-component, gpui-base, gpui-kit,
  gpui-kit-assets or gpui-fast and whose file's text differs between the old and new
  version. Old and new: the registry's 0.7.3 and 0.8.0 (gpui-fast 0.1.7 and 0.1.10; a
  citation that says "0.1.7/0.1.8" is checked against 0.1.8 as well). gpui-pre is 0.3.9 on
  both sides. `scripts/connector_parity_exceptions.toml:461` (gpui-component
  `select.rs:599-600`) is outside the script's paths and is read by hand in §3.3.
- **Not Task 1's lines.** Task 1's lines are the `+` lines of `git diff <base> <T1-last>`,
  where `<base>` is the M9b boundary commit Task 1 started from and `<T1-last>` its last
  commit (fix rounds included); `<T1-last>` is Task 2's starting HEAD, so these are line
  numbers of the tree the script runs on. Those lines already cite 0.8.0 / 0.1.10 (the
  regression test, `scene_quads`, the manifest comments, a fix for a 0.8.0 behaviour), and
  reading their numbers as 0.7.3's would move a correct `window.rs:2925` (`painted_quads`)
  to `:2948`, the 0.1.10 line of 0.1.7's `:2925`, with every gate passing. So the script
  and gate (4) leave them alone, and the script prints every upstream citation it skips
  there (file, line, text) into Task 2's commit body; each of those numbers is read at
  0.8.0 / 0.1.10 by Task 2's review, whoever wrote it (a line git shows as `+` because Task
  1 moved or re-wrapped it can carry a 0.7.3 number Task 1 never re-read). A chain with
  numbers both on and off those lines goes on the set-aside list whole, the entry marking
  which of its numbers are Task 1's (already 0.8.0 numbers, for which no old text is
  recorded). Task 2's scope is
  otherwise the citations `<base>` held.
- **Every form the repo writes**, since the script runs once (§3.3) and a number it does not
  see is renumbered by nothing: a cited `file:n` / `file:a-b`, the number on the same line
  or wrapped onto the next one (`tab/tab.rs:` then `/// 79-81`, `SC/demo.rs:1723-1724`,
  `:2209-2210`, `:2232-2233`, `SC/info/data.rs:833-834`); a comma list after it
  (`table/state.rs:1504, 1607, 2063, 2323`, `resizable.rs:27-30, 93, 104-111`); and a bare
  `:n` / `:a-b` / `n` that continues its chain's file, on the same line or a following one
  (`(:744)`, `(:239-243)`, "and :N", a bare number after a comma); a path then a symbol
  then the lines (`dialog/dialog.rs, Dialog::render, lines 716-802`,
  `table/state.rs, render_table_row: lines 2119 and 2235`; 22 moved numbers in `SC/info/`);
  and a number on the next line of a Rust string continued with `\` (`CN/src/contract.rs:911-912`,
  `:932`, `:954`, `:2130`). The chain rule is M8 T17's (its `cites.py`, `m8-T17-report.md:115`, is a
  precedent only: it lives in a session scratchpad, which a reboot clears, so Task 2 writes
  its script from this section): a citation is
  followed across `, `, "and", other words and **any line break that is not blank** (comment
  prefixes, `>`, `*` and `"` matter only to see blank lines and sentence ends), until
  another file name, a sentence end, a blank line or the next citation. The precedent: M8 T17's re-map of
  `platform-facts.md:N` renumbered each chain's first number and its same-line `, :N` items
  but missed the continued `:N` on later lines, and only a whole check found it
  (`.superpowers/sdd/todo_code-review-plan/m8-T17-review.md`, finding 1).
- **A chain moves whole or not at all.** Each number of a chain goes through the line rule
  below; if any number of a chain is set aside, the whole chain goes on the set-aside list.
  A continuation belongs to its chain's file (M8 T17's rule); it goes on the list only where
  no chain is open (after a sentence end, say) or the chain's file is not attributable.
- **The file.** A leading crate directory (`gpui-component/tab/tab.rs:254`,
  `gpui-base/…`, the form of 466 claims in `SC/info/`) names the crate and is stripped
  before matching. A cited path then matches a file under a crate's `src/` whose path ends with it
  on a `/` boundary (`tab_bar.rs` matches `src/tab/tab_bar.rs`; four citations resolve only
  this way, e.g. `SC/demo.rs:1728` `tab_bar.rs:538-548`); two such files in one crate set
  the chain aside (`state.rs:820`, `SC/support.rs:903`).
- **The crate.** The candidates are every crate with a matching file: the family at its old
  version (gpui-component, gpui-base, gpui-kit, gpui-kit-assets 0.7.3, gpui-fast 0.1.7),
  whether that file changed or not, and gpui-pre 0.3.9. A crate named before the number
  wins — the nearest name in its note (`gpui-base …`, `gpui_base`, a `GB` prefix,
  "gpui-pre 0.3.9"); a named crate without the file is passed over for the next nearest
  name, then for the unnamed rule. A crate outside the family takes the citation out of
  scope: any `<crate> <x.y.z>` or `<crate>-<x.y.z>/` before the path (`iced_widget 0.14.2`,
  `cosmic-text 0.15.0`, `epaint 0.36.2`), or a known name (`iced`, `iced_core`,
  `iced_widget`, `iced_aw`, `egui`, `egui_extras`, `epaint`, `emath`, `winit`, `taffy`).
  Unnamed, the one candidate whose file holds a non-blank old line is the crate; two or more
  set the chain aside. A citation whose crate's file is unchanged is left alone (gpui-pre's;
  gpui-component's `radio.rs` and `popover.rs`; gpui-base's `tooltip.rs`): a rule that
  counted changed files only gave about 35 such correct citations to a changed namesake and
  renumbered them wrongly (check 4: `radio.rs:411`, `popover.rs:308`, `popover.rs:384-387`,
  `gpui-pre src/style.rs:554-556`).
- **The line rule** (rationale K3): for each cited line `a` of a file that changed, (i) if
  its trimmed text, not blank, occurs exactly once in the old file and exactly once in the
  new, that line; else (ii) the line that both `difflib.SequenceMatcher` (`autojunk=False`,
  over the two files' trimmed lines; `a` inside an equal block of at least three lines) and
  an independent map, `git diff --no-index --histogram -U0` of the two versions read as a
  line map (about ten lines of hunk-header parsing; a count of 0 means "after line a"),
  give `a` — whether that line is `a` itself or another; where they disagree, or git has no
  line for `a`, set aside; else (iii) set aside. The check runs before the script writes
  anything. Text compares cannot tell copies of a repeated line apart, so this is what does:
  check 7 found difflib and git agreeing on 101 of 101 first numbers, 29 of 29 same-line
  continuations and 110 of 111 range ends, and on the one that differs git is right
  (`SC/info/chrome.rs:681`, `tab/tab.rs:253-262`: difflib puts `:262`'s `},` at `:320`, the
  new Folder arm's, inside a 70-line equal block; git at `:314`). A range `a-b` moves only if both ends map by (i) or (ii) with
  `b' − a' = b − a` and the inside equal; else set aside. There is no "stays" shortcut:
  (i) and (ii) decide even where the old line's text is still at the old number
  (`tab/tab.rs:254` is `tab_active_foreground` in the selected arm at 0.7.3; 0.8.0's `:254`
  holds the same text in `fn hovered`, and the diff puts the cited line at `:306`; check 6
  found 8 such numbers, all of which (i)/(ii) map correctly, and no other cost). Blank,
  past-the-end or vanished lines are set aside.
  Measured over the 472 moved first numbers of the report's list in one-crate changed
  files: (i) maps 377 and difflib maps the same 377 to the same lines; of the 91 (i) sets
  aside as repeated, (ii) maps 89 (34 in `SC/info/`, 55 elsewhere; the three-line minimum
  loses none of them); 2 are set aside and 4 are gone. `button/button.rs`'s +6 below its
  hover block is then the script's (§3.2).
- The script is written and run in Task 2, from this section, once; its command line goes in the
  commit body with the counts by form (first numbers, comma items, continuations) and by
  rule ((i), (ii), set aside, by reason), and its list of set-aside chains — each with its
  cited old text and the repo line's own text, to be found by content later — is kept for
  §3.2 and §3.3: in full in Task 2's commit body (each entry with its reason, its cited
  text and the repo line's own text), which Task 3b reads with `git show`; a scratchpad
  copy is a convenience only. A Task 2 fix round that changes the list restates it whole in
  its own body, and Tasks 3 and 3b read the list from the last Task 2 commit that states
  it.
- Gates: (1) digits only — line by line, the `<T1-last>` tree and the working tree (the
  same line counts, since the script changes digits only; not paired through `git diff`
  hunks, which mis-pair when neighbouring lines swap values) compare equal with their
  digits stripped and hold the same number of digit runs (so a number deleted outright
  fails); (2) line text — for every renumbered number (comma items and
  continuations included), the new line (each end and the inside of a range) holds the old
  line's trimmed text; (3) the failures of
  `showcase::every_colour_claim_is_read_at_the_line_it_cites` (`CN/src/showcase.rs:2819`)
  drop to claims in the §3.2 set, and the failures of
  `showcase::every_prose_citation_still_exists` (`:3691`) likewise; the counts before and
  after go in the commit body (the trial: 184 and 15). (4) The whole check, as M8 T17's
  `cites.py` did (1 961 numbers checked there): the repo files of `<T1-last>`
  and of the
  working tree are aligned line by line, and every number in the scope, found by a scan
  written apart from the re-map, must either hold at its new number the text its 0.7.3 line
  held, or be on the set-aside list; a number that does neither fails, and the failures
  are listed. Because M8 T17's re-review found its check blind wherever it shared the
  re-map's chain rule, the scan takes every `:n`, `n` and `a-b` from an upstream path to the
  end of its note — wider than the chain rule — and attributes files and crates by the
  rules above in code of its own. A number belongs to the last file name before it, and
  only a number whose file is an upstream crate's is checked (a `docs/platform-facts.md:N`
  after a gpui path is not). The note ends at the end of a run of consecutive comment lines
  (`//`, `///`, `//!`, `#`), a README paragraph (up to a blank line), a string literal or a
  TOML value. A non-citation the wider window catches (a pixel value) that fails by chance
  goes on the set-aside list, where the hand read sees what it is. The gate runs on the
  working tree before the commit; on a failure the re-mapped files are checked out back to
  `<T1-last>`, the script is fixed and run again from that tree, or the failing numbers
  go on the set-aside list, each failing number's whole chain (or the numbers of its note
  for that file) — never a run over numbers that already moved.
- *Added 2026-10-21 (G3 T2 review):* Task 2's two deviations, which its review accepted
  (`g3/g3-T2-review.md` §2, §3), are part of this section. (a) After a leading crate
  directory, the rest is the exact path from that crate's `src/` (or its root, for a
  crate-root file such as `build.rs`), as the showcase gates' `candidates()` read it
  (`CN/src/showcase.rs:2572`), not a `/`-bounded suffix, which would set aside
  `gpui-component/tooltip.rs` (`src/tooltip.rs`, `src/plot/tooltip.rs`) and
  `gpui-base/tree.rs`; a path without a crate directory still matches by its end. (b) One
  set-aside rule more, for a continuation whose file is uncertain: a continuation that
  words, not a comma, "and", "then" or an arrow, part from the number before it, and that
  an upstream file named earlier in the note would put at another line, sets its chain
  aside (11 chains; for 3 of them, `[46]`, `[144]`, `[157]`, the chain rule alone would
  have used the wrong file).
- *Added 2026-10-21 (G3 T3 fix round 1), correcting the line above:* `candidates()` reads
  a crate directory's path from that crate's `src/` only, since its roots are each crate's
  `src/` (`CN/src/showcase.rs`, `cited_source_dirs`); the crate-root case (`build.rs`,
  matched where the path names it whole) is Task 2's script's alone, as `901918a1`'s body
  states it.

### 3.2 Read by hand: what the gates read — Task 3's commit, which closes the unit

- The citations §3.1 set aside that the two showcase gates read (`SC/info/`), each read in
  the 0.8.0 source and re-pointed or rewritten; the others are §3.3's. By the
  simulation in §3.1, few first numbers remain (2 repeated outside any equal block, 4 gone,
  over all files), plus the chains set aside whole for one such number and the paths two
  crates hold (§3.1, *The crate*); Task 2's commit body has the real list. Known: the
  leaves panel's `ring` (`input/input.rs`) and gpui-base `element.rs` opacity lines, which
  have several candidates.
- **Buttons:** `GC8/button/button.rs` differs from 0.7.3's only in the hover block
  (`GC7 :724-730` → `GC8 :724` and `:736-747`, `diff`), now registered whenever the button is
  interactive and not selected. Below it every line moved by +6 and above `:724` none, which
  §3.1's line rule maps (the repeated `if outline {` and `cx.theme().…` lines through
  difflib's equal blocks; the unique hover-style lines `:725-728` by rule (i), to
  `:744-747`). A citation into `:724-741` that the script set aside (`:731-741` moved by −6,
  the rest rewritten) is found on the set-aside list and read against the diff; at HEAD the
  only one is `docs/showcase-exceptions.toml:650`'s `button/button.rs:724-728` chain, whose
  repeated `:724` set it aside, and it is §3.3's, by file. The arms the claims take are
  unchanged: `bg_color`, `hovered`, `active` and their `outline` arms are 0.7.3's
  (`GC7/button/button.rs:917`, `:989-1345`), against which G1 chose them.
- **Behaviour-changed panels**, every claim re-read at 0.8.0 even where its line did not
  move: Button (hover registration), DataTable (focus ring on keyboard focus only,
  `GC8/table/data_table.rs:228-234`, the claim at `SC/info/data.rs:119`; the fixed-column
  edge), TabBar (wheel mask, overflow menu, indicator), Tooltip (press dismissal), Tree
  (selects on click), Select (keyboard confirm), Diff (`ScrollBounce` wrapper), Dialog
  (clamped entrance easing, `GC8/dialog/dialog.rs:664`), the scrollbar (thumb length,
  `GB8/scrollbar.rs:1219-1220`), text inputs (1 px caret, `GB8/input/base/blink_cursor.rs:7`).

### 3.3 Read by hand: what no gate reads, and the version-stating prose — Task 3b's commit

These leave no gate red, so they wait until the unit has closed and been pushed, which
shortens the time M9b is held and M9a's CI stays red.

- **No second script run.** The script maps a number by reading the 0.7.3 file at it, so a
  second run over citations Task 2 already moved would re-map them to unrelated lines and
  still pass both §3.1 checks. Task 2's single run covers all of §3.1's scope; its
  set-aside list (each citation with the reason it was set aside) is in full in Task 2's
  commit body (§3.1), each entry with its cited text and the repo line's own text, read
  with `git show` from the last Task 2 commit that states it (§3.1), so Task 3b locates it
  by content after M9b has edited a shared file
  (`docs/showcase-exceptions.toml`, say). Task 3b re-points by hand only what that
  list holds, and reads the gpui-fast and version lines named below.
- The citations on that list outside `SC/info/`, and the ungated comments inside it (`CN/src`, `CN/tests`, the README,
  `docs/showcase-exceptions.toml`, `tools/showcase-support/src`, and the showcase's other
  files, `SC/demo.rs`, `SC/support.rs`, `SC/tests.rs` among them, whose `///` and `//`
  citations no gate reads, wherever Task 2 set them aside: e.g. `SC/demo.rs:4129`
  `select.rs:535-553`, `:4771`
  `tree.rs:41-44`, `SC/support.rs:903` `state.rs:820`, `SC/tests.rs:3201` `tab/tab.rs:79-81`,
  `:3882` `button/button.rs:1312`, `:5974-5975` `switch.rs:204-208`), each read in 0.8.0 and
  re-pointed or rewritten. Known: `docs/showcase-exceptions.toml` `button/button.rs:724-728`, whose
  hover block was restructured (`GC8/button/button.rs:736-747`). A reason text in
  `docs/showcase-exceptions.toml` that occurs verbatim in several groups (the "toolkit
  limit, sub-pixel phase" component in 11, `list/list.rs:602-603` in 4) is one list entry
  per occurrence and gets the same rewrite in each, since
  `scripts/check_showcase_parity.py` keys its merge groups by identical reason text.
- Outside the script's paths: `scripts/connector_parity_exceptions.toml:461` (gpui-component
  `select.rs:599-600`), re-pointed by hand. The egui showcase's `GC/` citations
  (`connectors/native-theme-egui/examples/showcase-egui/chrome.rs:1165`
  `tab/tab_bar.rs:554-584`, `:1376` `dialog/dialog.rs:696-701`, `demo.rs:242`, `:247`
  `button/button.rs:973, :1063`) already miss at 0.7.3 and are M9c's showcase: a
  `docs/todo.md` line for M9c names them with their 0.8.0 targets, read here.
- The `gpui-fast` citations, renumbered by Task 2 where they moved cleanly, re-read here:
  `SC/tests.rs` `src/fast/retained.rs:579-601` (the report: now `:601-623`) and
  `dependencies.rs:513-527`, `tools/showcase-support/src/capture.rs:39-44` (`build.rs:27-28`,
  moved by the hot-reload block, `GF10`'s `build.rs:26-34`). The draw helpers' comments
  citing the removed `retained.rs:1360-1371` are left to §2.3 (Task 4), which removes or
  rewrites them.
- The connector's own citations the gates do not reach: Task 2's run already renumbered
  their clean moves (the report counts citations into changed files: 73 in `geometry.rs`,
  51 in `contract.rs`, 16 in `CN/tests/seams.rs`); the ones it set aside are read and
  re-pointed here by hand, `CN/src/lib.rs:322` and `CN/src/geometry.rs:513` among them
  (`root.rs:440` → `:538`, if Task 2's crate rule did not already resolve them).
- `grep -rn '0\.7\.3\|0\.1\.7\|0\.1\.8' connectors/native-theme-gpui docs/showcase-exceptions.toml tools/showcase-support/src .github/workflows/ci.yml`
  (the count at the start goes in the commit body), except the README's `<!-- compat:begin -->`
  … `<!-- compat:end -->` line, which only `scripts/update_compatibility.sh` writes. Each
  line re-verified at 0.8.0: a present-tense claim gets the new version or is rewritten
  (the grep finds them all; among them: the module headers that say "verified against
  0.7.3", `CN/src/config.rs:8-9`, `CN/src/colors.rs:8-9`, `CN/src/base_layer.rs:13-14`,
  `CN/src/geometry.rs:1`, `:102-103`, `CN/src/variants.rs:23`, `CN/src/widgets/mod.rs:47-48`,
  `SC/info/leaves.rs:8-9`; `docs/showcase-exceptions.toml:20-22`, `:52`, `:53`, `:56`, `:873`;
  `CN/src/compat.rs:111`'s doc example; `CN/README.md:29`, `:56`, `:365`, `:817`, `:831`).
  A present-tense *measurement* is re-measured, not version-swapped: `CN/src/contract.rs:377`,
  `:989`, `:1801`, `:2000`, `:2271`, `:2294`, `:2457` ("nothing reads … in 0.7.3", "a token
  0.7.3 paints nowhere") are each re-grepped in the 0.8.0 sources (the check found
  `tokens.list` and `sidebar_primary` still unread outside `theme/`, so `INERT_SURFACES`
  holds); a statement recording history stays (`.github/workflows/ci.yml:337`, "which GPUI Kit 0.7.3
  fixed"). The `capture.rs:42` note re-verified against gpui-fast 0.1.10's
  `windows-manifest-provided-by-host` (still enabled by gpui-kit, gpui-component and
  gpui-base 0.8.0, rationale §1.2).
- Outside that grep, `git grep -l '0\.7\.3' -- . ':!docs/archive' ':!Cargo.lock'` finds
  only history or dated statements, which stay: `CHANGELOG.md`'s released sections,
  `docs/evidence/README.md` and `docs/evidence/os-probe/2026-10-14-run-38478334574/README.md`,
  the review plan and ledger, `docs/todo.md` and
  `docs/todo_gpui-full-theme.md` (dated lines, §10), and
  `connectors/native-theme-iced/src/styles.rs:228` (it names the gpui-component version it
  compared); `.cargo/audit.toml` is §1's; `ROADMAP.md`, `CHANGELOG.md` `[Unreleased]` and
  the generated `docs/COMPATIBILITY.toml` are §10's.
- Stated counts are re-measured, not renumbered: `ThemeColor` 139 fields (the field-count
  test), 106 `IconName` variants (`GC8/icon.rs` identical to 0.7.3's; README `:56` says
  "0.8.0's list is 0.7.1's").
- Gates: for §3.2, both showcase citation gates pass and `scripts/check_release.sh
  --commit` is green; for §3.3, the digits-only check over its re-pointed lines alone
  (listed) and, for each, the 0.8.0 line read by hand, with the rewritten prose lines
  listed separately in the commit body; a second run of the grep
  above that finds only lines kept as history and the two draw-helper comments Task 4 owns
  (`SC/tests.rs:145`, `CN/src/widgets/tests.rs:1035`), listed in the commit body; and
  `--commit` green.

## 4 -- Nested scrolling (K7)

- `SC/tests.rs`: `a_nested_scroller_keeps_the_wheel_to_itself` (`:448`) covers the eight
  scroll sites of rationale §1.4, each by its debug selector (one added where a site has
  none, the MessageScroller's among them), with the existing method per site: the demo on
  screen in a window shorter than its page (one window size or a page per site, as needed),
  the inner scroller moved by the wheel (its offset where the showcase holds its state, as
  for the List and Tree; else the bounds of its first child, for the Settings frame's plain
  div), the page's (`CONTENT_SCROLL`) not. An inner scroller whose content fits cannot move, so a site whose
  content does not overflow under the test's preset gets a preset or height under which it
  does (stated in the test).
- Order: the extended test passes with every `.occlude()` in place, on both cores. Then,
  one site at a time: remove its `.occlude()` and its comment, run the test on both cores;
  passing, the removal stays; failing, the `.occlude()` comes back with its comment
  rewritten to the 0.8.0 reason (for a plain GPUI scroller, GPUI's `Interactivity`
  scroll listener stops at no one, on both cores, and only gpui-component's widgets are wrapped in
  `ScrollBounce`, `GC8` sites in rationale §1.2). The commit body lists each site's result.
- The three other `.occlude()`s (pointer shield, title-bar info overlay, app-menu frame)
  are not touched.
- The test's doc comment is rewritten: the mechanism is now `ScrollBounce`'s capture-phase
  claim (`GB8/scroll_bounce.rs:248-270`, `:556-610`), plus occlusion where it is kept.
- `docs/todo.md`, *native-theme-gpui connector*, *Upstream PRs to gpui-component*, "PR: let
  a scroll container keep the wheel to itself": a dated line — gpui-component 0.8.0 wraps
  its scrolling widgets in gpui-base's `ScrollBounce`, which keeps the wheel (the sites);
  still open: scroll chaining (at its end the inner area stretches instead of handing over,
  upstream's test `the_innermost_area_takes_the_whole_gesture`) and GPUI's own
  `overflow_*_scroll` divs; and which showcase occlusions remain.
- *Added 2026-10-22 (G3 T6), the controller's ruling at Task 5:* the Settings frame never
  scrolls. The `Settings` it holds is an `h_resizable` (`GC8/setting/settings.rs:416-432`)
  that fills it (318 px of the 320 px frame), so the frame's `overflow_y_scroll` had nothing
  to move and its first child never moves. The scroll owner under the frame is the Settings'
  page list, a `ScrollBounce` over its `list_state` (`GC8/setting/page.rs:228-257`, wrapped
  at `:233`), and the test measures that owner, by the row probe in its first group. The
  frame's `.occlude()` went in Task 5 (`342232d6`), and Task 6 replaced its dead
  `.overflow_y_scroll()` with `.overflow_hidden()`, the clip it gave. Over a box's parts
  that do not scroll (its padding and border, the Settings' heading, sidebar search field
  and resize handle) the page now scrolls too, where the occlusion swallowed the wheel.
- *Added 2026-10-22 (G3 T6 fix round 1), on the line above:* the frame was already
  `overflow_hidden` through `demo_frame` (`SC/support.rs`), so Task 6's added
  `.overflow_hidden()` repeated it, and its fix round removed the call: deleting the dead
  `.overflow_y_scroll()` alone leaves the same clip (gpui's `Style::overflow_mask` clips a
  `Hidden` axis as a `Scroll` one, and a `Scroll` axis with no scrollbar width lays out as
  `Hidden`).

## 5 -- `TabVariant::Folder` in the showcase (K8)

- `SC/demo.rs`: `pub(crate) fn folder_tab_row(ui, cx, id, labels, selected, disabled,
  on_click) -> Stateful<Div>`: `TabBar::new(id).folder()` with the same per-call seams as
  `tab_row`'s upstream path (`SC/demo.rs:2023-2075`): under a native theme each label is a
  child in `tab.font` (`text_in`) and each tab at least `tab.min_width` wide and
  `tab.min_height` tall; without one, `Tab::new().label(label)`. One tab disabled, to show
  the disabled label. Under the bar, a panel filled `cx.theme().tab_active` (the colour the
  selected tab's shape is painted in, `GC8/tab/tab.rs:998`), so the tab joins it. A
  click selects the tab (state kept by the showcase, as the Basic page's tab row does).
- `SC/pages/layout.rs`: a section "TabBar (Folder variant)" with its heading, debug
  selector `layout-folder-tabs`.
- `SC/info/layout.rs`: `pub fn folder_tab_row(t: &Theme, labels, selected, native:
  Option<&ResolvedTheme>) -> WidgetInfo` — colour claims, each at its 0.8.0 line (the
  implementer records the lines; the colour-claim gate checks them): the bar `tab_bar`
  (`GC8/tab/tab_bar.rs:423-427`, `tokens.tab_bar`), a label `tab_foreground`
  (`GC8/tab/tab.rs:210-214`), a hovered label and the selected label `tab_active_foreground`
  (`:264-269`, `:315-320`), the disabled label `muted_foreground` (`:389-393`), the selected
  shape `tab_active` (`:998`), the hover fill `secondary` (`:1012`), the separators `border`
  (`:1025`). `.config`: `Theme::radius` (from `defaults.border.corner_radius`) for the
  XSmall and Small sizes and `radius_lg` (from `corner_radius_lg`) for Medium and Large,
  capped at a third of the tab's height (`:30-45`); the label font and minimum sizes where
  a native theme states them. `.not_themeable`: the top padding, side padding and separator
  height per size (`:31-36`), the 4 px gap (`FOLDER_TAB_GAP`, `:14`), the height (Segmented's,
  `:89-105`), one radius for the top corners and the outward curves, and the hover fill's
  *button* token `secondary` (as Outline's hover, `docs/todo.md`, *Upstream PR candidates
  from v0.5.8 (Tier U)*, "`TabVariant::Outline` hovers with"). `.instance`: the tabs, the
  selected one, the click.
- `SC/tests.rs`, failing first: the Layout page draws `layout-folder-tabs`; under two
  presets whose mapped `tab_bar` and `border` are opaque (chosen by reading them in the
  test), the bar's quad is painted in `tab_bar`, and a 1 px quad between two unselected
  tabs in `border` (read through §2.2's `scene_quads`).
- `docs/todo.md`, the entry "`TabVariant::Outline` hovers with": a dated line, Folder's
  hover reads `tokens.secondary` the same way (`GC8/tab/tab.rs:1012`).
- The theme's outward corners (§12) do not reach this sample: upstream's Folder draws its
  curves at its one radius (`:30-45`). Task 12 adds the `.not_themeable` note "the theme's
  `tab.active_outward_corner_radius` (§12): upstream's Folder curves at `Theme::radius` /
  `radius_lg`, capped at a third of the height; `widgets::TabBar` draws the theme's", once
  the field exists.
- Gate: `python3 scripts/check_widget_coverage.py` stays at gpui `missing 0`.
- *Added 2026-10-22 (G3 T6 fix round 1):* `SC/info/layout.rs`'s `folder_tab_row` takes
  `disabled` too, after `selected`, the index `demo::folder_tab_row` disables, so its
  `.instance` names the disabled tab as well as the selected one.

## 6 -- The slider's accessibility (K11)

- `CN/src/widgets/slider.rs`: `Slider::accessibility_label(self, label: impl
  Into<SharedString>) -> Self` (`#[must_use]`, documented as `Checkbox`'s is). Fallback path
  (`fallback`, `:212-218`): forwarded to `gpui_component::slider::Slider::accessibility_label`.
  Themed path: the root's `.aria_label(label)` beside its `Role::Slider` (`:428`).
- Disabled: a connector element wrapper, in `CN/src/widgets/mod.rs` (private to the crate),
  that delegates every `Element` method to its inner element and adds
  `node.set_disabled()` in `write_a11y_info` while disabled — the shape of gpui-base's
  `A11yDisabled` (`GB8/a11y_disabled.rs`), which is `pub(crate)` there; its doc names that
  source. The themed slider's root is wrapped with it.
- The test seam. `impl RenderOnce for Slider` returns an `AnyElement` on both paths, and
  `AnyElement`'s `Element` impl forwards no accessibility method (gpui-pre
  `src/element.rs:728-780`, gpui-fast 0.1.10 `:747-`: `a11y_role` and `write_a11y_info` keep
  their defaults), so passing a rendered `Slider` to the `a11y_node` harness
  (`CN/src/widgets/tests.rs:1699-1712`) gives an empty node on both paths. The checkbox tests
  solve the same problem with a crate-private typed seam (`Checkbox::row`, `checkbox.rs:632`,
  returning the gpui-base primitive). The slider gets the same: a crate-private
  `Slider::themed_root(self, window, cx) -> Result<SliderRoot, Box<Self>>` that builds the
  themed root, the wrapped `Stateful<Div>` (`SliderRoot` is the wrapper above over it;
  `Stateful<E>` forwards `a11y_role` and `write_a11y_info`, gpui-pre `div.rs:4111-4115`), or
  hands the slider back, boxed as the existing seams do for clippy's `result_large_err`
  (`checkbox.rs:258`, `:632`, `:791`, `switch.rs:367`), where no native theme is installed;
  `render` calls it and on `Err(slider)` renders `slider.fallback()`. A harness sibling, `a11y_node_of(role, element: Option<impl Element>)`, calls
  `write_a11y_info` on the element itself (the wrapper is an `Element`, not a `RenderOnce`).
  `Window::debug_a11y_tree_json()` (gpui-pre `window.rs:7011`) is used on neither path: it
  returns the last tree update, which a frame records only while accessibility is active
  (`src/window/a11y.rs:208-222`, `src/window.rs:3674`, `:3773-3788`), a flag only the
  platform's activation callback sets, and the test platform's `a11y_init` is the trait's
  no-op (gpui-pre `platform.rs:1387`, gpui-fast 0.1.10 `:1409`), so in tests it is always
  `None`.
- Tests (`CN/src/widgets/tests.rs`), failing first, through `themed_root` and
  `a11y_node_of`: a themed slider (kde-breeze) announces its `accessibility_label`; a
  disabled themed slider's node is disabled, an enabled one's is not. The fallback path has
  no typed seam (`gpui_component::slider::Slider::render` returns an opaque
  `impl IntoElement`, the gpui-base slider, `GC8/slider.rs:153`, `:269-273`), but that
  gpui-base slider registers itself for gpui-kit's test API under `test-support`, which the
  connector's gpui-kit dev-dependency turns on (`.id(("slider", entity_id)).test_support()`,
  `GB8/slider.rs:509-511`; its observation records the node's `label` and `disabled`,
  `GB8/test_support.rs:108-115`, `:330-340`). Its test draws a `widgets::Slider` without a
  native theme and reads `gpui_kit::test::TestWindowExt::try_find(window, ("slider",
  state.entity_id()))` (`GK8/test.rs:37`, `:335`): `label()` is the given
  `accessibility_label`, and `disabled()` is `Some(true)` for a disabled slider and `None`
  for an enabled one.
- `SC/demo.rs` `slider` and `number_input` take a required `label: &'static str`, passed to
  `accessibility_label` on every path (`widgets::Slider`, `gpui_component::slider::Slider`,
  `gpui_component::input::NumberInput`, `GC8/input/number_input.rs:80`, and
  `gpui_base::NumberInput`, `GB8/number_input.rs:215`); each call site
  (`SC/pages/basic.rs`, `SC/pages/inputs.rs`) names what its control sets. The required
  parameter is the gate (a call without one does not compile).
- `docs/todo.md`, *native-theme-gpui connector*: an `[upstream]` entry: a public way to mark
  an element's accessibility node disabled (gpui-base's `A11yDisabledExt` public, or a GPUI
  `aria_disabled`), so the connector's copy can go.
- *Added 2026-10-22 (G3 T7 fix round 1), the controller's ruling on the review's I1:* the
  themed root registers for gpui-kit's test API under gpui-base's slider id, `("slider",
  entity_id)` (`GB8/slider.rs:509-511`): `A11yDisabled::new(root, disabled).test_support()`,
  the observation outside the wrapper so its snapshot sees the flag
  (`GB8/a11y_disabled.rs:52-61`), `track_focus` after it so the snapshot sees the focus
  binding (`GB8/observe.rs:25`), and an `InteractiveElement` forward on the wrapper. The
  registration made the typed seam unneeded: `themed_root` and `a11y_node_of` are gone, and
  the themed tests read the drawn frame through `TestWindowExt::try_find`, as the
  fallback's does, each first checking that the connector drew its own slider
  (`native-slider`) exactly under a native theme.

## 7 -- The side panel on a window resize (K10)

- `SC/tests.rs`, first: `the_side_panel_keeps_its_width_when_the_window_resizes` — the
  showcase open with its side panel, its width read, the window made wider and then
  narrower (`cx.simulate_resize`, both sizes still fitting the side panel and a page), the
  width unchanged after each; then the same after a splitter drag (the dragged width kept)
  and after the side panel is hidden and shown again (`rearrange` installs a fresh state,
  the case the bookkeeping exists for). A group rescales only once every panel has a
  measured size (`GB8/resizable/mod.rs:396-398`), which each gets on its first prepaint
  (`:233-237`), so the plain case is expected to fail without `.fixed()`.
- If it fails: the side panel's `resizable_panel()` (`SC/app.rs:2118`) gains `.fixed()`
  (`GB8/resizable/panel.rs:295-305`; the content panel stays flexible, as `fixed` requires),
  and the test passes. If it passes without: no code change; the commit holds the test,
  its body saying why nothing else changed.
- `side_panel_width` and `rearrange` stay (they serve the toggle, rationale K10); the
  existing side-panel tests pass on both cores.

## 8 -- The other new variants and options (K12)

| New in 0.8.0 | Decision |
|---|---|
| `TabVariant::Folder` | shown, §5; `windows-11`'s folder look is `widgets::TabBar`'s, from the theme (§12), not upstream's variant (rationale K15) |
| `ScrollBounce` around scroll widgets | §4 |
| `scroll_bounce(bool)`, `ScrollBounceMotion`, `ScrollableMask::vertical_wheel` | not modelled (K9); the README's *What gets mapped* says so |
| `Slider` / `NumberInput` `accessibility_label`, disabled to accessibility | §6 |
| `ResizablePanel::fixed()` | §7 |
| public `ResizeHandle` / `resize_handle` | a dated line on `docs/todo.md`, *gpui: the splitter renderer into the connector*; the entry *gpui: Settings and dock splitters draw upstream's renderer* gets a dated line (the dock still installs its own, `GC8/dock/dock.rs:58`) |
| DataTable `fit_column`, `fit_col`, `col_width`, `select_on_modifier_click`, `fixed_cols_border`, `focus_ring(bool)`, `max_width` 1200 | not shown: behaviour or options with no theme value; the Data page has no fixed columns |
| completion menu as popup-menu rows | excluded, as completion popups were in G1 |
| 1 px caret | no model field (`caret_color` only); nothing states a width |
| `SelectionFormat::SourceAcrossStyles`, editor options, `editor_host`, `syntect` | nothing the showcase draws; tree-sitter stays |
| `DockToggleEvent`, `MessageScrollerState::logical_scroll_top` | not shown (Dock) / nothing to draw |
| GPUI Fast `use_display_p3`, `AppLifecyclePhase::Disconnected`, `hot-reload` | defaults; the feature stays off |
| `GPUI_FAST` | the ROADMAP bullet "a gpui-fast marker constant" is resolved upstream and needs no check here (K12) |

## 9 -- GPUI Fast at run time

- On the maintainer's session, with the maintainer's go (it takes window focus): the
  showcase built with `--features gpui-fast` and without; `--dump-layout` for kde-breeze and
  adwaita, light and dark, from both builds, each dump compared with its pair (every
  element's position and size equal, a JSON diff, as G1 did); one screenshot of the Layout
  page on GPUI Fast looked at (the Folder sample, the Sidebar), taken as the Linux
  screenshot scripts take theirs (`scripts/capture_window.sh`'s `capture_showcase`,
  spectacle) from the `--features gpui-fast` build, since the showcase's own `--screenshot`
  refuses on Linux (`cli::screenshot_support`). A difference is fixed, or
  explained in the ledger (gpui-fast 0.1.8's text-measurement fix and 0.1.10's compositing
  are the candidates).
- *Added 2026-10-21 (K19):* the GPUI Fast dumps depend on §2.4. Before it, a reused view's
  elements dropped out of the dump. They are taken at rest, before any scroll streak, since
  inside a composited scroll layer the dump is stale (§2.4). The run also counts how much
  reuse the showcase keeps (render counts under a hover sequence), since every record has the
  registry's other readers built again in the next frame (K19). A `docs/todo.md` line follows
  if it is little.

## 10 -- Documents

- `CN/README.md`: *Backends* (`:815-840`): "GPUI Kit 0.8.0", its example manifest line, and
  one sentence that GPUI Kit 0.8.0 builds GPUI Fast 0.1.10 or later (its `gpui-fast =
  "0.1.10"`); *What gets mapped* (`:861`), a line that scroll bounce is not theme data: gpui-base
  turns it on by build target (macOS, iOS, Android) and off under reduce motion (K9);
  *Theme-drawn widgets* (`:428`) or *Accessibility* (`:559`), one line: the themed `Slider`
  takes an `accessibility_label` and reports its disabled state, as gpui-component's does;
  the other prose lines of §3.3.
- `docs/COMPATIBILITY.toml` and the README's Verified line: written only by
  `scripts/update_compatibility.sh run gpui` (which resolves the newest releases on a
  throwaway lock, runs the connector's gates, stamps, restores `Cargo.lock`), after the
  connector's last source change (the stamp hashes `CN/Cargo.toml`, `src`, `examples`,
  `tests` at HEAD); gates `compat::the_verified_line_is_the_stamps` and
  `scripts/update_compatibility.sh check`.
- `CHANGELOG.md` `[Unreleased]`: *Changed* — the native-theme-gpui line naming 0.7.3
  (`:173`) states 0.8.0 and that GPUI Fast is 0.1.10 or later (unreleased, so changed in
  place); the side panel's width on a window resize, if §7 changes it; for each scroller
  whose occlusion §4 removes, the showcase line rationale K7 gives (the page now scrolls
  over a nested area whose content fits, and what lies behind the box is hovered again).
  *Added* — `widgets::Slider::accessibility_label` and
  the themed slider's disabled state for assistive technology; the Layout page's Folder tab
  bar.
  *Added 2026-10-21 (K19):* *Breaking Changes*, the `widgets::PartBounds` line (§2.4).
- `ROADMAP.md`: the v0.7.0 paragraph (`:72-74`) names GPUI Kit 0.8.0; in *v0.7.2 — Upstream
  receivers*, the bullet "a gpui-fast marker constant" goes (gpui-kit 0.8.0's
  `GPUI_FAST`), and every bullet that cites a line "at gpui-component 0.7.x" / "gpui-base
  0.7.3" / "gpui-pre 0.3.9" is re-checked at 0.8.0 and re-cited where its line moved (`Tab`'s
  `.h(height)` at `GC8/tab/tab.rs:958`; the TextView rule at `GB8/text/node.rs:4296`; the
  others as read).
- `docs/todo.md` (append-only), besides §2.2, §2.3, §4, §5 and §6's items:
  - *native-theme-gpui connector*, "`unicode-properties` back to 0.1.4 when gpui-fast-web
    unpins it": a dated line, gpui-fast-web 0.1.10 still pins `=0.1.3`.
  - *Upstream PR candidates from v0.5.8 (Tier U)*, "a scrollbar thumb inset per axis": a
    dated line, still so at gpui-base 0.8.0 (`scrollbar.rs:1225`), the thumb now
    `viewport / content × track` long (`:1219-1220`).
  - *native-theme-gpui connector*, *Upstream PR to gpui*, "PR: add `Window::screenshot()`
    API to gpui" (handed to G3 by M9b T2): a dated line — at gpui-pre 0.3.9 and gpui-fast
    0.1.10 a window renders to an image only through the test-support `render_to_image`
    (gpui-pre `window.rs:2871`, `GF10/window.rs:2915`); the showcases capture through
    `tools/showcase-support`'s `capture::FrameCapture` on macOS and Windows and through
    `scripts/capture_window.sh` (spectacle) on Linux, so the entry's blade-graphics and
    `BitBlt` wording is history. The ask stands.
  - *gpui: the base-palette repair writes colours without their tokens*: a dated line, no
    gpui-component or gpui-base 0.8.0 source reads `tokens.red` … `tokens.cyan_light`
    outside `theme/` (the grep).
  - *gpui: a citation gate for the connector's doc comments*: a dated line, the number of
    connector citations the 0.8.0 bump renumbered by script and by hand.
  - every other open `[upstream]` gpui entry that cites a 0.7.3 line is re-checked at
    0.8.0: one 0.8.0 resolved closes (moves to the archive), one it moved gets a dated line
    with the 0.8.0 line.
- `docs/todo_gpui-full-theme.md`: an appended "gpui-component 0.8.0" note (theme unchanged;
  the Folder variant, shown, and the theme's outward tab corners drawn by `widgets::TabBar`,
  §12; scroll bounce not modelled).
- D1's documents are written by the D1 tasks themselves, each with the change it records
  (§12.7); Task 10 only checks that they are all there.
- The review: `docs/todo_code-review-plan.md` (G3 row, D11) and
  `docs/todo_code-review-findings.md` (*G3* section, status row; M9a's Step 8 line closing on
  the green run) — re-read before each edit.

---

## 11 -- Acceptance

- Both cores: `cargo test -p native-theme-gpui --all-targets` (and the doctests on gpui-pre)
  pass; clippy `-D warnings` and the strict panic lints clean on the library and the
  showcase; MSRV as §1.
- The full `./scripts/check_release.sh`: only the expected items — stale visual assets
  (refreshed by the next asset run), the existing `v0.6.1` tag, the soft semver warnings
  already approved and the one §12.1 adds (`TabTheme` gains a field, the *Breaking Changes*
  line), the iced and egui compatibility stamps (G3's Tasks 11, 13, 14 and 15 change their sources,
  as M9b and M9c do; each is re-stamped by whichever of G3, M9b and M9c closes last, §12.7),
  `cargo outdated` as recorded at M8; no gpui item. Its semver check of the gpui
  crate (`--default-features`) shows no break beyond those `[Unreleased]` lists; its
  packaging check passes, and so does the same check with `--features gpui-fast`, which
  `check_release.sh` does not run (by hand, as G1 did: `cargo package -p
  native-theme-derive -p native-theme -p native-theme-gpui --allow-dirty --target-dir
  target/package-verify --features gpui-fast`), right after the full run, whose packaging
  section first removes the stale local-registry copies of the workspace crates
  (`check_release.sh:973-981`); run at another time it can build against a stale copy.
- CI on the branch green, the GPUI Fast step of *Test gpui connector* and the macOS and
  Windows GPUI Fast builds included.
- Screenshots that change, for the maintainer's next `generate_assets_release.sh`: the gpui
  Layout page (the Folder sample), focused text fields (the 1 px caret), any GPUI Fast
  capture (0.1.10's compositing), and the three showcases' `windows-11` captures on the
  Windows runners (`.github/workflows/screenshots.yml:194-208`, each toolkit's Basic page):
  the selected page and Basic tabs filled `#F9F9F9` (dark `#282828`) over the `#F3F3F3`
  (`#202020`) bar, curving out at their feet, their outline running down into the curves,
  no bottom edge, no WinUI bar rule and no separators between the tabs (the showcases' own
  `separator.*` rule under the page tabs stays, rationale K14 (f)).
- D1 (§12): every gate §12 names passes; `windows-11`'s selected tabs draw the folder shape
  in all three showcases and no other bundled preset's tabs change.
- *Added 2026-10-21 (K19):* §2.4's tests pass on both cores.

---

## 12 -- D1: the selected tab's outward corners (rationale K14–K18; Tasks 11–15)

The maintainer decided D1 (2026-10-21): *"Yes, in G3"* — `windows-11`'s tabs take WinUI 3's
folder shape, as theme data drawn by all three connectors. Notation for this section: `r` is
`tab.active_outward_corner_radius` where it is stated and positive (unstated or 0 draws
today's shape, and nothing below applies; validation rejects a negative or non-finite one,
rationale K14); `w` is `tab.border.line_width`; the
selected tab's outer bounds are `x0`, `y0`, `x1`, `y1` (`y1` its bottom, the bar's bottom
edge); `R = r·√2 + 1`. MUX links are to microsoft-ui-xaml at `8463f45`
(`https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/TabView/…`
for the TabView files, `…/controls/dev/CommonStyles/…` for `Common_themeresources_any.xaml`),
the form `docs/platform-facts.md` uses.

### 12.1 The model, the data files, the renumbering (Task 11)

- `native-theme/src/model/widgets/mod.rs`, `TabTheme`, after `active_indicator_side`
  (`:482-486`):

  ```rust
  /// Radius of the outward curves at the selected tab's two bottom corners,
  /// which spread its fill into the bar's bottom edge, as WinUI 3's `TabView`
  /// draws its selected tab (4, against 8 on its top corners). Where it is
  /// stated, the selected tab's outline (`border`) has no bottom edge and
  /// runs on down the curves. `None` where the platform's selected tab does
  /// not curve out at its feet: all but Windows; macOS publishes no shape
  /// (docs/platform-facts.md §2.11).
  #[serde(rename = "active_outward_corner_radius_px")]
  #[theme(category = "soft_option", check = "non_negative")]
  pub active_outward_corner_radius: Option<f32>,
  ```

  The derive gives `ResolvedTabTheme` the same `Option<f32>` (a `soft_option`,
  `native-theme-derive/src/gen_structs.rs:38-39`'s rule) and the field name list for TOML
  linting.
- `docs/property-registry.toml` `[tab]`, after `active_indicator_side` (`:333`):
  `active_outward_corner_radius_px = "f32"`.
- `docs/inheritance-rules.toml` `[no_inheritance]`, after `"tab.active_indicator_side"`
  (`:454`): `"tab.active_outward_corner_radius", # None: none on KDE, GNOME and Material; unpublished for macOS (§2.11)`
  (and the blank line `:464` removed, below).
- `docs/platform-facts.md` §2.11, a row after `hover_text_color` (`:1365`), the table's last,
  so the rows above keep their numbers (and the blank line `:1401` removed, below):
  - first column `` `active_outward_corner_radius` ``;
  - macOS: `❓ — AppKit publishes no tab shape: the selected tab is a CoreUI bezel (see
    border.corner_radius) · macOS 26: ❓ — the same`;
  - Windows: `**(Fluent)** 4 — the selected TabViewItem is filled with SelectedBackgroundPath,
    set 4 outside the tab on each side (Margin="-4,0", TabView.xaml:551), a geometry whose
    top corners take OverlayCornerRadius (border.corner_radius) and whose bottom corners
    curve out by 4 ("Assumes 4px curving-out corners, which are hardcoded in the markup",
    TabViewItem.cpp:106, UpdateTabGeometry :98-123). The selected tab's outline has no
    bottom edge (TabViewSelectedItemBorderThickness 1,1,1,0, TabView_themeresources.xaml:268),
    lies 1 outside the path's sides (TabViewSelectedItemHeaderMargin -1,0,-1,1, :269), and
    is transparent over its sides' bottom 4 (TabViewSelectedItemBorderBrush, a gradient,
    :72-80, :150-158), where two 4 × 4 arcs, 1 wide on the inner side of the curve, carry it
    on down the curves to the bar's 1px rule, in TabViewBorderBrush = CardStrokeColorDefault,
    border.color's brush (TabView.xaml:547-548, shown in Selected :338-339;
    TabView_themeresources.xaml:68, :146). The 1px rule the bar
    draws in that brush under the other tabs (BottomBorderLine, TabView.xaml:546, collapsed
    under the selected one, :337) and the separator (item_gap) are not modelled`, each
    citation a link at the pin, the code spans as the table writes them;
  - KDE: `**(Breeze src)** (none) — the selected tab is one rounded rectangle,
    roundedPath(strokedRect(frameRect), corners, …) (breezehelper.cpp:1482), rounded on its
    top corners only, corners = CornersTop (breezestyle.cpp:7245-7246, see
    border.corner_radius)`, the links at the pin `:1356` uses;
  - GNOME: `(none) — a GtkNotebook tab is square: notebook > header > tabs > tab sets no
    border-radius (_notebook.scss:119); AdwTabBar, not modelled, rounds every corner,
    border-radius: $button_radius (_tab-view.scss:160)`, the links at libadwaita 1.10.0 as
    the column's others;
  - Material: `**(M3 token)** (none) — container-shape = corner-none
    (v0_192/_md-comp-primary-navigation-tab.scss:54)`, the link as `:1356` gives it.
- Read MUX again at the pin while writing the row (the raw files, as §1.3's second half was
  read on 2026-10-21); a line that differs is cited as read, and the rationale's §1.3 follows.
- The line citations of the three files (rationale K18, option (b)). Every insertion is
  paired with the removal of one blank line close after it, so the lines that move form a
  short window, and every candidate in that window is listed and decided by hand, not by a
  chain rule:
  - `docs/platform-facts.md`: the row inserted after `:1365`, and the blank line `:1401`
    removed (between `### 2.14 Status Bar` at `:1400` and its table at `:1402`, which then
    follows its heading directly, as §2.11's and §2.12's tables do, `:1343-1344`,
    `:1366-1367`). Lines `:1366`–`:1400` (§2.12's and §2.13's rows and the §2.14 heading)
    move by one; every line from `:1402` on keeps its number.
  - `docs/inheritance-rules.toml`: the entry inserted after `:454`, and the blank line
    `:464` removed (between the array's `]` at `:463` and its trailing comment). Lines
    `:455`–`:463` move by one.
  - `native-theme/src/model/widgets/mod.rs`: the field inserted after `:486`. No blank line
    lies near (the struct runs on to `}`), so every later line moves by the block's length.
    Its citations past `:486`, by the model scan below, at `fa90c6c0`: 13 numbers on 8
    lines:
    - `docs/inheritance-rules.toml:81` `:507, :571` (`SidebarTheme`'s and
      `StatusBarTheme`'s `#[theme_inherit(border_kind = "partial", …)]`, both right): move
      by the block's length;
    - ten numbers of six already-stale citations, each re-pointed by content to what it
      names, read after the insertion, with a hand-read entry in the commit body: egui
      `src/accessors.rs:503` `:620` (for `list.header_font`; `pub header_font` is at
      `:634`); egui `src/atlas.rs:308` and `:683` `:1028-1045` (for `LayoutTheme` and its
      `Default`, `pub struct` to `}`, `:1084-1101`); egui `src/contract.rs:119`
      `:1025-1045` (for `LayoutTheme`'s serialisation, which skips a `None`: from
      `#[serde_with::skip_serializing_none]`, `:1078-1101`); the iced showcase's doc
      comment that cites `:884` for `LayoutTheme` (by content; `:1084`);
      `docs/todo.md:167` `:773-803` (for `SegmentedControlTheme`, `:934-966`);
    - one stays: `docs/todo.md:168`'s `991`, a `platform-facts.md` line.

    Citations below `:486` do not move, and four of them are already stale; Task 11
    re-points them by content too (a known-stale citation is a defect, and the fix costs a
    line): egui `src/accessors.rs:286` `:25` (for `window.title_bar_font.color` and
    `window.inactive_title_bar_text_color`; `:30`, `:27`); iced `src/styles.rs:844`
    `widgets/mod.rs:138-140` (the short form; for the one `CheckboxTheme` both controls
    share, the `2.5 Checkbox / Radio Button` banner and the struct, `:176-183`);
    `docs/todo.md:56-57` `:146-188`, `:170-172` (for `CheckboxTheme`, `:178-249`, and its
    `hover_background`, `:230-232`). The fourth, `docs/todo.md:117` `:373-405` (for
    `TabTheme`), sits in the entry *Tab: an active-tab indicator*, whose asks
    `e610750b` (2026-09-30) met: the three `tab.active_indicator_*` fields, their §2.11
    facts rows for all five platforms and their receivers. Task 11 closes that entry: its
    `###` section holds that one item, so the section moves whole, verbatim, to
    `docs/archive/todo_closed.md` under `## Core API`, ending with a dated line naming
    `e610750b`, as that file's header asks; its citation stays as written there, as the
    archive's citations do.
  - The candidates, each file by a construction of its own, all over M8 T17's scope (every
    tracked file outside `docs/archive/`, `docs/evidence/`, `.superpowers/` and
    `docs/todo_code-review-findings.md`, `m8-T17-report.md:117`), with one number pattern,
    `(?<![\w.#])(\d+)(?:-(\d+))?(?![\w])`, both ends of a range tested against the range:
    - the facts window: every match from 1366 to 1401 on any line (the 4-digit form of the
      pattern finds the same);
    - the inheritance window: every match from 455 to 463 on a line that contains
      `inheritance-rules`, or on one of the three lines after it;
    - the model file: every match on a line that contains `widgets/mod.rs` (the short form
      `widgets/mod.rs:N` included; a mention of gpui's own `CN/src/widgets/mod.rs` is set
      aside by reading, and at `fa90c6c0` none carries a number), or on one of the three
      lines after it: those from 487 to 1999 are the moving set, those below 487 are read
      for staleness;
    - the registry and the egui mapping, which gain a line too: every match past `:333` on a
      line that contains `property-registry`, or on one of the three after it, and past
      `:2351` near `mapping.toml`.

    At `fa90c6c0`: the facts window 157 numbers in 22 files (123 move, 34 stay, Appendix B;
    the same as at `2e80f87e` but for the iced showcase's `TALL_WINDOW`, located by
    content); the inheritance window 3 (`ios-13.toml:19`'s `:460-465`, which straddles the
    window and is already stale, so it is re-pointed by content to the
    `[wrong_safety_nets]` header it describes, `:488-498`; and two that stay, iced
    `contract-leaves.toml:329`'s `456`, an `iced_core` line, and
    `docs/todo_code-review-plan.md:554`'s `461`, a `model/mod.rs` line); the model file the
    13 moving numbers above and 9 below; the registry 8, all another file's lines (in
    `docs/showcase-exceptions.toml:787`, `:795`, and `docs/todo_code-review-plan.md:554`),
    so they stay; the mapping 0. Each construction's output goes in the commit body.
  - The decisions: a number moves where it is a line of the edited file, whatever precedes
    it (a facts anchor on its line or a chain, or a bare `(:N)` after another file's numbers,
    as `windows-11.toml:412`'s `(:1380)`); it stays where it is the OS probe's (after
    `probe`), another file's line (an upstream source, the gpui showcase, an egui source), or
    no line at all (`TALL_WINDOW`'s `1400.0`, "a window 1380 wide"). Task 11 re-runs every
    construction at its base: a candidate this section or Appendix B lists keeps its
    decision; any other, new or changed since, is decided by hand the same way; the listed
    lines are located by content, since the files move.
  - `docs/todo.md` is in the scope, as it was in M8 T17's (`m8-T17-report.md:121`): a
    digits-only renumbering keeps an entry's text true and is not a rewrite, so §0.2's
    append-only rule holds.
  - While it is in `documented_sizes.rs` and beside it: five name-less facts citations, each
    already 13 behind the call beside it (pre-existing, not D1's), are rewritten by hand as
    `platform-facts.md:N` with the number the call reads: `platform_facts_xref.rs:327`
    "`§2.17 (:1455-1456)`" → `platform-facts.md:1468-1469`, `:330` "`§2.3 (:1170) and §2.4
    (:1199)`" → `platform-facts.md:1183`, `:1212`; `documented_sizes.rs:1229-1230`
    "(`:1670`) … (`:1673`)" → `:1683`, `:1686`, `:1332` "(`:1277`)" → `:1290`, `:1359`
    "(`:1556`)" → `:1569`. None of these numbers lies in the window.
  - Gates: (G1) the candidate list in the commit body equals the enumeration's output at
    the task's base, each entry with its line's text and its decision; (G2) each number moved by
    a window (or by the model block's length) has a new line in the edited file that holds
    the text its old line held, and its range stays a range of the same length; the
    re-pointed and rewritten numbers are read by hand instead, each listed with its new
    line's text; (G3) in every file the task's deliberate edits do not
    otherwise touch (the facts row, the inheritance entry, the field and its doc, the
    registry key, the two removed blank lines, the hand rewrites, the egui mapping row,
    `mapping.md` and the count, the README sentences, the CHANGELOG lines, the `[1.0]` todo
    entry), and in those files on every line outside the edits, the commit changes digits
    only, and only those the lists move (one commit: the edits cannot be split from the
    renumbering without a red commit); (G4) the content tests that already read these
    numbers pass: `documented_sizes.rs`'s `native_themes_state_documented_sizes`,
    `native_themes_state_documented_fields` and `every_field_citation_names_its_platform_facts_row`,
    and `native-theme/tests/platform_facts_xref.rs`. The reviewer re-runs the enumeration
    and the scans and reads every decision. This is not Task 2's script run again (§3.3): no upstream
    citation is in its scope.
- egui: `connectors/native-theme-egui/mapping.toml`, after `["tab.active_indicator_side"]`
  (`:2349-2351`): `["tab.active_outward_corner_radius"]`, `verdict = "derived"`,
  `tested_by = "T18(a)"`; `src/mapping.md` regenerated
  (`cargo test -p native-theme-egui --lib -- --ignored regenerate_mapping_md`, its header
  `:3`); the leaf count `515` → `516` and its message "511 ResolvedTheme leaves + 4 layout
leaves" → "512 …" (`src/mapping_tests.rs:239-243`).
- Red first: the registry key and the facts row before the field, so
  `property_registry_keys_match_derive_inventory` (`native-theme/src/model/mod.rs`) fails;
  the field makes it pass; egui's `every_leaf_has_exactly_one_row_and_every_row_is_well_formed`
  fails until the mapping row is in.
- `README.md:41` and `native-theme/README.md:47`: a clause of its own after the optional
  sizes' sentence, since an unstated radius applies no toolkit value: "and
  `tab.active_outward_corner_radius`, where unstated the selected tab's feet are square".
- `CHANGELOG.md` `[Unreleased]`: *Breaking Changes* — "**native-theme**: `TabTheme` gains
  `active_outward_corner_radius` (see *Added*); `TabTheme` is not `#[non_exhaustive]`, so a
  struct literal of it outside the crate needs the field or `..Default::default()`; its
  resolved form is `#[non_exhaustive]` already", as the `TooltipTheme` line; *Added* —
  "**native-theme**: `tab.active_outward_corner_radius_px`, the selected tab's outward
  bottom corners (WinUI 3's folder-shaped `TabView`: 4 against its 8 top corners)".
- `docs/todo.md` (append-only):
  - *Core API*, beside *Tab: an active-tab indicator* (`docs/todo.md:112`): a `[1.0]` entry,
    "### Tab: WinUI 3's tab separators and its tab bar's bottom rule", carrying §1.3's MUX
    citations
    (`TabView.xaml:553`, `:322`, `:332`, `:546`, `:36-37`, `:337`; `TabView.cpp:217-231`;
    `TabView_themeresources.xaml:46`, `:266`, `:68`, `:146`; `Common_themeresources_any.xaml:257`,
    `:53`) and what modelling needs (a colour, a width, an inset and when it hides, in three
    connectors; the rule a second meaning for `tab.border`, rationale K14 (e), (f));
- Gates: `--commit` green (the documented-sizes gate, `check_platform_facts_rows.py`,
  `check_section_refs.py`, the registry test, egui's mapping tests among it); the
  renumbering's gates (G1)–(G4); the GPUI Fast suite (Task 11 moves a digit in gpui's
  `src/geometry.rs`).

### 12.2 What is drawn, in every connector

Where `r` is stated (§12's notation), for every tab bar a connector or showcase draws itself
from `tab.*` — gpui's `widgets::TabBar`, the iced bars `tab_flares` wraps (§12.4), the egui
showcase's `tab_bar` — and not for upstream's own variants, which take no such value (§5):

- **The selected tab** is drawn as today — `tab.active_background`, `tab.border` on its top
  and sides, its top corners `tab.border.corner_radius`, the indicator where stated — but its
  outline has **no bottom edge**, and its label does not move.
- **Two flares**, one at each bottom corner, each set `w` in from the tab's outer edge
  (the outer bounds include the border; WinUI's side edge lies just outside its fill,
  rationale §1.3): the left one in the square `[x0 + w − r, x0 + w] × [y1 − r, y1]`, about
  the centre `(x0 + w − r, y1 − r)`; the right one in `[x1 − w, x1 − w + r] × [y1 − r, y1]`,
  about `(x1 − w + r, y1 − r)`. Each is, clipped to its square, an annulus about its
  centre from radius `r` to `R` in `tab.active_background` (inside the square: the square
  less the quarter disc of radius `r`), and, where `w > 0`, an annulus from
  `max(r − w, 0)` to `r` in `tab.border.color` (the outline continued down the curve, on the
  disc's side as WinUI's arcs lie, to the bar's bottom edge). At the square's top the
  outline annulus covers the side edge's column, `[x0, x0 + w]`, so the two are one line;
  below that, the side edge's pixels farther than `r` from the centre lie in the fill
  annulus, which paints over the side edge's foot, as WinUI's gradient leaves it
  transparent there (rationale §1.3). This continuous line, the band tapering out to the
  bar's edge, is our design: WinUI's markup ends its arc 1 above the bar, where the
  neighbours' 1 px rule takes over, and may leave one row between the side edge and the arc
  (rationale §1.3); the model states no bar rule (rationale K14 (f)), so ours runs the band
  to the bar's edge.
- **Painted after every tab of the row**, above any tab's fill and hover, and inside the
  row's clip, so a scrolled-away tab's flares go with it (iced, whose layers do not nest
  their clips, cuts each flare by the draw's viewport, §12.4).
- **Room at both ends:** the row's tabs start `r` after its inner start and end `r` before
  its inner end, inside its clip, so the first and last tabs' flares show (they reach
  `r − w` past the tab; the room is `r`, so it does not depend on the outline's width).
- **The order** in every connector: the tabs, each with its indicator as today; then the
  strip (iced and egui only, §12.4, §12.5), not painted where a bottom-side indicator is
  stated, since that line already covers the bottom edge (it spans the tab's outer width at
  its bottom); then the flares. So the flares' fill lies over a bottom indicator's two end
  columns `[x0, x0 + w]` and `[x1 − w, x1]` alike in all three. No bundled preset states
  both; each connector's tests add one case with a bottom indicator and `r` stated.
- Nothing else changes: the unselected tabs, the gap, the sizes, the indicator's own
  drawing (one exception, the iced Extra page's `Tabs` sample, whose content panel the
  bar's room insets too, §12.4).
- **A toolkit limit:** iced and egui clip a square to whole physical pixels (`iced_wgpu-0.14.0`
  `src/lib.rs:479`, `physical_bounds.snap()`), and the fill annulus reaches past the square
  to `R`, so at a fractional scale up to a pixel of `tab.active_background` can show beside
  a square; the recording-renderer test reads the unrounded bounds and does not see it.
- **Numbers the three showcases' tests share** (`windows-11` with `r = 4` on it, `w = 1`):
  the left centre `(x0 − 3, y1 − 4)`, its square `[x0 − 3, x0 + 1] × [y1 − 4, y1]`; the
  right centre `(x1 + 3, y1 − 4)`, its square `[x1 − 1, x1 + 3] × [y1 − 4, y1]`;
  `R = 4·√2 + 1 ≈ 6.657`, fill annulus width `R − r ≈ 2.657`, outline annulus `3`–`4`; top
  radius 8; the colours the theme's `tab.active_background` and `tab.border.color`, read
  in the test (`#f3f3f3` and `#0000000f` light before Task 15, `#f9f9f9` after).
- **The test seam** for "`windows-11` with `r = 4`" before Task 15 states it: the resolved
  `windows-11` theme with `tab.active_outward_corner_radius = Some(4.0)` set on it (a
  resolved theme's fields are public), installed through each showcase's own seam (§12.3,
  §12.4, §12.5); no test writes a preset or a theme file.

### 12.3 gpui: `widgets::TabBar` (Task 12)

- `CN/src/widgets/tab_bar.rs`: `TabLook` gains `pub outward_radius: Option<Pixels>` —
  "`tab.active_outward_corner_radius`: the selected tab's outward bottom corners, where the
  theme states a positive one" — which `TabLook::of` fills through `side` (`:93-95`), then
  `None` where it is 0 (`side` alone keeps a 0, as it does for the indicator's width); a
  non-finite radius makes the whole look `None`, as any non-finite length does. Under it:
  - the selected tab: `border_b(px(0.))`, its bottom padding `padding_bottom` (or 0) plus
    `border_width`, so the label stays where the bottom border held it;
  - the `BaseTabs` row: `pl(r)` / `pr(r)` (the room at both ends) on the scrolling row
    itself, as upstream pads its Folder row (`GC8/tab/tab_bar.rs:686-687`);
  - the selected tab's bounds, recorded in the frame's prepaint by an absolute `canvas`
    child reaching its outer edge, as `part_bounds` does (`CN/src/widgets/mod.rs:145-156`),
    into an `Rc<Cell<Option<Bounds<Pixels>>>>` made in `render`; with the bottom border 0,
    every inset of the selected tab's bottom side is 0, not `-border_width`: this recorder,
    its `part_bounds` (`tab_bar.rs:408-412`, the `Part::Tab` bounds the layout dump reads;
    `part_bounds` takes one inset for all four sides, `CN/src/widgets/mod.rs:145-156`, so it
    gains a separate bottom inset) and a bottom-side indicator (`indicator`, `:145-159`);
  - a `canvas` after the tabs in the row (absolute, so it takes no room) whose paint reads
    those bounds and paints §12.2's quads, each under `window.with_content_mask` of its
    square, with `paint_quad(quad(..))` (transparent background, the annulus as a border;
    gpui-pre `src/window.rs:4163`, `:4684`, `:7813`; gpui-fast 0.1.10 `:4238`, `:4760`,
    `:7919`). Both cores prepaint the root before they paint it (gpui-pre
    `src/window.rs:3710`, `:3746`; gpui-fast 0.1.10 `:3767`, `:3807`); test 4 holds GPUI
    Fast's reuse of an unchanged subtree.
  - The geometry is a crate-private pure function, `flare_quads(tab: Bounds<Pixels>, look:
    &TabLook) -> Vec<FlareQuad>` (each with its clip square, its bounds, corner radius,
    border width and colour): empty where the radius is unstated, the two fill annuli, and
    the two outline annuli only where `w > 0`; tested without drawing.
  - `TabBar`'s and `TabLook`'s doc comments state the shape; the fallback (no native theme)
    is unchanged.
- Tests (`CN/src/widgets/tests.rs`), failing first, on both cores, with §12.2's resolved
  `windows-11` installed as `CN/src/variants.rs:76-79`'s test installs one
  (`crate::from_preset`, the field set on the resolved theme, `crate::apply`); the
  showcase's tests below install it the same way after the showcase opens
  (`SC/app.rs:1738` `apply_theme_by_name` goes by name and cannot carry the edit):
  1. `flare_quads` for a known tab and `windows-11`'s look with `r = 4`: §12.2's numbers.
  2. A drawn bar under that theme: §2.2's `scene_quads` holds the four quads, each with its
     square (inside the row) as its content mask, and the selected tab's quad has a bottom border of 0 and its
     other borders `w`; the first tab starts `r` inside the row.
  3. On gpui-pre, where `painted_quads()` is paint order (rationale K5): the flare quads come
     after every tab's.
  4. After a second frame with nothing changed, drawn without `window.refresh()` (the test's
     own `cx.update(|w, cx| w.draw(cx).clear(cx))`, whatever Task 4 decides for the shared
     `redraw`, `CN/src/widgets/tests.rs:1032-1041`), the flares are still painted.
  5. With the radius unstated: no flare quad, and the selected tab's bottom border is `w`.
- The showcase:
  - `SC/info/layout.rs` `native_tab_row`: a `.config("outward corners", …)` — stated:
    "`tab.active_outward_corner_radius`, {r}px: the selected tab curves out into the bar's
    bottom edge at both feet, its outline following, with no bottom edge
    (native-theme-gpui/widgets/tab_bar.rs, `TabLook::outward_radius`)"; unstated: "none: the
    theme states no outward corners, so the selected tab's feet are square"; the "outline"
    config (`:791`) says "no bottom edge" where the radius is stated.
  - `SC/info/leaves.rs` `HOW`: the leaf's lines — for the three selected tabs "gpui: the
    selected tab's outward corners, painted after the tabs (TabLook::outward_radius)", for
    the three bars "gpui: the tabs' inset at the row's ends (TabLook::outward_radius)", for
    `basic.page` "gpui: room for both flares in the column rule (basic_column_count)" — each
    element's entry before any prefix of it that would match first (`HOW`'s first-match
    rule, `:11-14`).
  - `SC/pages/basic.rs` `basic_column_count` (`:240`): the two tabs plus `2r`; its doc
    comment says so; a test: with §12.2's resolved `windows-11` passed to
    `basic_column_count`, the count is the one the widest control including `2r` gives.
  - The Folder sample's panel (§5): the `.not_themeable` note §5 names.
  - `native_tab_row`'s nine colour claims cite `native-theme-gpui/widgets/tab_bar.rs:104`
    to `:126`, inside `TabLook::of` (`SC/info/layout.rs:722-779`); the new field and its fill
    line move them, so each is re-pointed at the line read after the edit
    (`every_colour_claim_is_read_at_the_line_it_cites` holds them).
- `CN/README.md:443`, the `TabBar` row: "…and, where `tab.active_outward_corner_radius` is
  stated, its outward bottom corners, the outline following them".
- `CHANGELOG.md` *Added*: "**native-theme-gpui**: `widgets::TabBar` draws the selected tab's
  outward corners where the theme states them (`TabLook::outward_radius`)".

### 12.4 iced: `tab_flares` (Task 13)

- `connectors/native-theme-iced/src/lib.rs`, under `#[cfg(feature = "widgets")]`:

  ```rust
  #[must_use]
  pub fn tab_flares<'a, Message, Renderer>(
      resolved: &native_theme::theme::ResolvedTheme,
      tabs: impl Into<iced_core::Element<'a, Message, iced_core::Theme, Renderer>>,
      selected: &[usize],
  ) -> iced_core::Element<'a, Message, iced_core::Theme, Renderer>
  where
      Message: 'a,
      Renderer: iced_core::Renderer + 'a,
  ```

  `tabs` itself where `r` is unstated; else a widget that hands every `Widget` method but
  `draw` to `tabs` (its tree state included, so wrapping changes no state), and in `draw`
  draws `tabs`, then, about the bounds of the layout node at the path `selected`
  (`&[i]` in a `row` of tabs or an iced_aw `TabBar`; `&[0, i]` in an iced_aw `Tabs` with its
  bar on top), the strip that covers the bottom edge (`[x0 + w, x1 − w] × [y1 − w, y1]` in
  `tab.active_background`, not where a bottom-side indicator is stated, §12.2's order) and
  §12.2's annuli, each in `renderer.with_layer(square ∩
  viewport, …)` with `fill_quad` (`iced_core-0.14.0` `src/renderer.rs:24`, `:62`), the
  annulus quads with `snap: false` (`Quad::default()` snaps under iced's default `crisp`
  feature, `:89-100`). iced's layers do not nest their clips (`iced_graphics-0.14.0`
  `src/layer.rs:85-98`, `iced_wgpu-0.14.0` `src/lib.rs:674-676`), so the cut by the `draw`
  call's `viewport` — a scrollable's visible area (`iced_widget-0.14.2`
  `src/scrollable.rs:1187-1205`) — is what keeps a scrolled-away tab's flares inside the
  strip; a flare whose square misses the viewport is not drawn. A path that names no node
  draws nothing more (no index, no panic). Its geometry is a crate-private
  `tab_flares_look(&ResolvedTabTheme) -> Option<…>` (colours, `r`, `w`; `None` unless `r` is
  positive and finite), as `tab_indicator_strip` is. Its doc says what it draws, where it fits (beside
  `tab_indicator_line`), and iced_aw's limit (rationale K15).
- `src/contract/rows.rs`, beside `crate::tab_indicator_line` (`:172-185`): two
  `AccessorRow`s, `crate::tab_flares (fill)` (native: `tab.active_background` where `r` is
  stated) and `crate::tab_flares (outline)` (native: `tab.border.color` where `r` is stated
  and `w > 0`), `get` from `tab_flares_look`; `every_accessor_returns_its_native_leaf` holds
  them, vacuously until Task 15 (no combination states `r` before it), so a lib test also
  holds the two rows on a resolved theme with `r` set. `contract-leaves.toml` is regenerated (its header, `:2-3`) and committed as the walk
  writes it; no bundled preset states `r` yet, so no leaf moves the new rows there until
  Task 15.
- `scripts/connector_parity_exceptions.toml`, beside `tab_indicator`'s (`:445-455`): `gpui`
  and `egui` `absent` for `tab_flares` — gpui's `widgets::TabBar` draws them itself; an egui
  application reads `tab.active_outward_corner_radius` from `ThemeAtlas::resolved_for`
  (`src/mapping.md`: DERIVED → T18(a)).
- The showcase (`examples/showcase-iced.rs`, by function name):
  - `tab_row`: the scrollable's row padded `r` at both ends and wrapped in `tab_flares(…,
    &[open index])` (the `tagged` tabs are the row's children); this draws the page tabs and
    the inspector tabs (its two callers at `2e80f87e`);
  - the default build's Basic tab row (the arm built without `iced_aw` that tags
    `basic.tabs.bar`: since M9b T5, `cffb6355`; in `basic_tab_row` since `99af695b`, M9b's
    T6 fix round): its own `row` of
    `tab_button`s, wrapped, in
    `tagged("basic.tabs.bar", container(…))` with `.node("basic.tabs.one", &[0, 0])`; the
    row padded `r` at both ends and wrapped in `tab_flares(resolved, …, &[state.basic_tab])`
    inside the container (`tab_flares` passes its child's layout through, so `&[0, 0]` and
    `&[0, 1]` still name the tabs);
  - the iced_aw bars, whose widgets take no padding of their own around the bar
    (`TabBar::padding` pads each tab's label, iced_aw 0.14.1 `src/widget/tab_bar.rs:265`;
    `Tabs`' builders, `src/widget/tabs.rs:149-295`, have none for the bar): the room goes
    outside the wrapper, `container(tab_flares(resolved, x, path)).padding([0, r])`, so the
    path stays relative to the wrapped element `x`:
    - the Basic page: `x` the `Stack` of the bar (its layer 0, `tagged("basic.tabs.bar",
      bar)`, which passes the bar's layout through) and the indicator slots, path
      `&[0, state.basic_tab]`, so the flares come after the slots, as §12.2's order asks;
      `basic.tabs.bar` stays the `TabBar`'s rectangle, the room outside it (no comparison
      preset states the radius, rationale K17);
    - the Extra page: the stand-alone `TabBar`, path `&[active]`, and the `Tabs`, path
      `&[0, active]`; the padded container insets the `Tabs`' content panel by `r` as well,
      on that sample only, which its Widget Info notes;
  - `basic_column_count`: the two tabs plus `2r`;
  - the routes: for the leaf, the three selected tabs "native_theme_iced::tab_flares, over
    the row (under iced_aw: the selected tab under the pointer is iced_aw's Hovered, drawn
    with no outline and square corners, `styles/aw.rs:234-237`, while its flares, their
    arcs and the strip stay)", the bars "the row's padding at both ends" (iced_aw's: "a
    container's padding outside the bar at both ends"), `basic.page`
    "room for both flares in basic_column_count"; for `tab.active_background` and
    `tab.border.color` on the selected tabs: under iced_aw, `crate::tab_flares` among the rows
    the colour reaches, in the showcase's `Drawing` pattern (`AW_TAB_BAR` gains
    `"crate::tab_flares"`, beside `"crate::tab_indicator_line"`); for the tabs `tab_style`
    draws (page tabs, inspector tabs, the default build's Basic tabs), whose routes are hand
    routes (`("tab", "active_background")`, `("tab", "border.color")`), each gains "; and
    `native_theme_iced::tab_flares`'s flares, where `tab.active_outward_corner_radius` is
    stated".
- Tests, failing first: in the lib, `tab_flares_look`'s numbers and the widget's draw through
  a recording renderer (an `iced_core::Renderer` that records `start_layer` bounds and
  `fill_quad`s) for a `row` of fixed-size containers with a background — the strip and
  §12.2's four quads (`snap: false`) under their layers after the containers' quads;
  nothing more with `r` unstated or a path past the last node; drawn with a `viewport` that
  cuts a flare's square, that layer's bounds are the intersection; with the selected tab
  outside the viewport, no flare layer. In the showcase, with §12.2's resolved `windows-11`
  as `State::current_resolved`: the page tabs' first tab starts `r` further in than
  without, the routes of the new leaf, and the Basic column count. In both builds — the default one and
  `--features iced_aw`, the one the Windows runners capture (`.github/workflows/screenshots.yml:37`,
  `:46`) — a test of the Basic tabs' flares built on the module's `basic_frame` and
  `pixels_in` (or the tab rows' own seam, `tab_rows_frame`, since `99af695b`; a `Simulator`
  snapshot read back as an image, with the rectangles from
  `drawn_layout()`, by name): with §12.2's resolved `windows-11` and `tab.active_background`
  set apart from `tab.bar_background` (as `the_basic_tabs_are_the_themes_tabs_and_switch`
  sets it), the physical pixel centred at logical `(x0 − 0.25, y1 − 0.25)` of
  `basic.tabs.one`'s rectangle and its mirror `(x1 + 0.25, y1 − 0.25)` (the snapshot is at
  scale 2, `iced_test-0.14.0` `src/simulator.rs:221`; 4.65 from each flare's centre, past
  the outline band) is `tab.active_background` within the existing tests' `abs_diff ≤ 2`;
  with the radius unstated the same pixels are not `tab.active_background`; and the same
  with the second tab selected, so the path follows the selection.
- `connectors/native-theme-egui/src/install_tests.rs:2924` cites
  `connectors/native-theme-iced/src/lib.rs:2081-2094` for iced's degenerate-factor pin; it is
  already stale (at `2e80f87e` those lines are inside an unrelated test; the pin is
  `scaled_text_size_scales_every_size_alike`'s degenerate-factor loop, `:2714-2720` at
  `2e80f87e`), so it is re-pointed by content after Task 13's edit. Every other citation of
  the iced `lib.rs` past the insertion is re-pointed the same way: at `2e80f87e` the iced
  showcase's two doc comments citing `lib.rs:1278-1336` for font matching (by content; also
  already stale, `:1278` and `:1336` close the indicator code). The repository's other
  citations into that file lie before `:1256` and do not move; one of them, `docs/todo.md:412`'s
  `:1253`, is stale already (`system_font_family` is at `:1437`, past the insertion), so
  Task 13 re-points it by content after its edit, with the others into that file (it is
  Task 13's and not Task 11's because Task 13's insertion moves its target). The widget's impl lives in
  a module of its own, so `lib.rs` grows by the function and its doc only.
- `connectors/native-theme-iced/README.md:311` and `:466-468`: `tab_flares` beside
  `tab_indicator_line`, and a sentence where `tab_indicator_line` is described (by content).
- `CHANGELOG.md` *Added*: "**native-theme-iced**: `tab_flares`, the selected tab's outward
  corners over a row of tabs, an iced_aw `TabBar` or `Tabs`".
- When: at an M9b boundary where none of M9b's T5 (its review and fix rounds), T8 (the
  iced `basic_column_count`), T10 (the Extra page's `TabBar`) and T19 (the page list
  `page_tabs` draws) is open, each done or not begun, nor an M9b fix round carrying tab items
  (rationale K16).

### 12.5 egui: the showcase's `tab_bar` (Task 14)

- `connectors/native-theme-egui/examples/showcase-egui/demo.rs` `tab_bar` (`:1192`): `r` of
  room before the first tab and after the last, inside the scroll area where the row
  scrolls; the selected tab's `Response::rect` kept; after the loop, with the tabs' own
  painter (so the scroll area's clip applies): the strip over the bottom edge
  (`rect_filled`, not where a bottom-side indicator is stated, §12.2's order; the
  indicator stays where `tab_bar` paints it today, after the selected tab), and per flare
  `painter.with_clip_rect(square)` (it intersects with the tabs' own painter's clip,
  `egui-0.36.2` `src/painter.rs:67-75`) with
  `circle_stroke(centre, (r + R) / 2, Stroke::new(R − r, fill))` and, where `w > 0`, with
  `inner = max(r − w, 0)`, `circle_stroke(centre, (r + inner) / 2, Stroke::new(r − inner,
  outline))` (`egui-0.36.2`
  `src/painter.rs:71`, `:370`). Its doc comment says so; `reg.amend_last`'s reads gain
  `("tab.active_outward_corner_radius", …)`, "not stated: square feet" where unstated.
- `pages/basic.rs` `column_count` (`:137`, its tabs at `:151`): the two tabs plus `2r`.
- Tests (`examples/showcase-egui/tests.rs`), failing first, with §12.2's resolved
  `windows-11` installed through a `ThemeAtlas` built from it (`ThemeAtlas::builder(name,
  &light, &dark)`, as `src/install_tests.rs:51` builds one):
  the frame's `FullOutput::shapes` (`egui-0.36.2` `src/data/output.rs:28`) hold, for each
  flare, the two circle strokes with §12.2's centres, radii, widths and colours, each clipped
  to its square; none with `r` unstated; with a bottom indicator also stated, no strip and
  the indicator under the flares; the first page tab starts `r` further in; the Basic
  column count.
- The connector itself gains nothing more: its mapping row is Task 11's.
- When: should M9c have started, at an M9c boundary where its T7 (the egui `column_count`)
  is done or not begun (rationale K16).

### 12.6 The switch-on (Task 15)

- `native-theme/src/presets/windows-11.toml`, `[light.tab]` and `[dark.tab]`:
  `active_outward_corner_radius_px = 4.0`, with the comment
  "# platform-facts.md:<the row> (§2.11): the selected tab's bottom corners curve out by 4
  (TabViewItem.cpp:106 at microsoft-ui-xaml 8463f45)".
- The lines this task inserts in `windows-11.toml`'s `[light.tab]` move one citation: no
  first number past it is cited, but a continuation is,
  `connectors/native-theme-egui/src/style/variants.rs:362` ("`windows-11.toml:210`; dark
  `#33333300`, `:770`"), already stale (`:770` is `focus_border_color`); the dark checkbox's
  `disabled_background = "#33333300"` is at `:807`, in `[dark.checkbox]` (`:795`), at
  `fb0d3ef5` and `2e80f87e` alike (`:774` holds the same text, but in `[dark.input]`, so a
  text compare cannot tell them apart); re-pointed to it, located by its section, as read
  after the edit, the file joining Task 15's (at a boundary where no M9c task on it is
  open).
- `native-theme/src/presets/windows-11.toml`, `[light.tab]` and `[dark.tab]`
  `active_background`: `#f9f9f9` and `#282828` (it is `#f3f3f3` / `#202020`), with the
  comment "# platform-facts.md:1347 (§2.11): TabViewItemHeaderBackgroundSelected =
  SolidBackgroundFillColorTertiary #F9F9F9 (TabView_themeresources.xaml:7;
  Common_themeresources_any.xaml:274 at microsoft-ui-xaml 8463f45)" (dark `:85`, `:70`,
  `#282828`), a comment line of its own directly above the key, under the block's
  "Interactive state colors" heading comment (rationale K14).
- `docs/platform-facts.md:1347`, the Windows `active_background` cell:
  "← `defaults.background_color`" becomes "**(Fluent)** `TabViewItemHeaderBackgroundSelected`
  = `SolidBackgroundFillColorTertiary` — L #F9F9F9, D #282828, while the bar and an
  unselected tab are transparent over the window (`TabViewBackground`,
  `TabViewItemHeaderBackground`)", with links at the pin to `TabView_themeresources.xaml:7`,
  `:85`, `:5`, `:83`, `:6`, `:84` and `Common_themeresources_any.xaml:274`, `:70` (one line
  replaced, none added).
- `docs/todo.md`, *windows-11: what WinUI 3 draws, or what Win32 reports*: re-read first; its last open item closed, it moves to
  `docs/archive/todo_closed.md` with a dated line naming the commit (should any other item
  in it still be open, it stays, with the dated line). Every gate reading `windows-11`'s
  colours passes on the new value (the iced contract and colour-route tests, the gpui
  showcase's claims, the egui mapping tests).
- `native-theme/src/presets/documented_sizes.rs`: the field joins `OPTIONAL_FIELDS` (`:724`,
  `[&str; 5]` → `6`), `FIELDS` (`:952`, `[&str; 18]` → `19`), `field_cell` (`:974`,
  `("2.11", "active_outward_corner_radius")`), `states_field` (`:1009`) and
  `resolved_field` (`:1035`); `FIELD_ROWS` (`:830`) gains six rows at the new facts line:
  `Windows11` `Px(4.0)` "TabViewItem.cpp:106: 4px curving-out corners", `KdeBreeze`
  `Unstated` "(none): CornersTop", `Adwaita` `Unstated` "(none): a square notebook tab",
  `MacosMonterey` and `MacosTahoe` `Unstated` "❓: the bezel's shape is unpublished",
  `Material3` `Unstated` "(none): corner-none", which `native_themes_state_documented_fields`
  (`:1078`) and `every_field_citation_names_its_platform_facts_row` (`:1169`) hold; the
  module header (`:23-30`) and `FieldRow::field`'s doc (`:798-802`) name the field, and so do the two documents that list the gated
  sizes, `native-theme/src/presets/README.md:25-29` and `docs/new-os-version-guide.md:231-237`
  ("tab `item_gap`, `active_indicator_width` and `active_outward_corner_radius`").
- `docs/showcase-elements.toml`: `tab.active_outward_corner_radius_px` after
  `tab.active_indicator_side` on `basic.tabs.one`, `chrome.page_tabs.basic` and
  `chrome.side_panel.inspector_tabs.widget`; after `tab.item_gap_px` on `basic.tabs.bar`,
  `chrome.page_tabs` and `chrome.side_panel.inspector_tabs`; after `tab.item_gap_px` on
  `basic.page`; the header's column rule (`:619`) "2 * tab.min_width + tab.item_gap + 2 *
  tab.active_outward_corner_radius". No `sample` (rationale K17). `python3
  scripts/check_showcase_parity.py --check-list` passes.
- `connectors/native-theme-iced/contract-leaves.toml` regenerated: `windows-11/light` and
  `/dark` now move `tab.active_background` and `tab.border.color` through
  `crate::tab_flares`'s rows (every other combination in `except`), and the two `[rows]`
  tables appear; the iced showcase's colour-route tests over every bundled combination pass
  on it.
- `CHANGELOG.md` *Added*: "**native-theme**: `windows-11` states
  `tab.active_outward_corner_radius` 4, so its selected tab takes WinUI 3's folder shape in
  the three connectors' tab bars; the three showcases' tab rows draw it"; *Fixed*: v0.6.1
  shipped `#f3f3f3` / `#202020` (`git show v0.6.1:native-theme/src/presets/windows-11.toml`,
  `:204`, `:612`), and the entry's earlier corrections are *Fixed* lines, so the fill joins
  the existing one (found by content: "the `windows-11` preset fills the checked button with
  `AccentFillColorDefault` … the card and the selected tab are framed in
  `CardStrokeColorDefault`"), edited in place: "…, and the selected tab is filled with
  `TabViewItemHeaderBackgroundSelected`, `SolidBackgroundFillColorTertiary` (`#f9f9f9`,
  dark `#282828`), where it took the window's `#f3f3f3` / `#202020`".
- `connectors/native-theme-iced/src/contract.rs`, `a_cleared_tab_soft_option_copies_the_idle_fill`
  (by name): its comment says the table's entry "runs on windows-11, where `tab` states the
  same color for an idle and for a selected tab" and that "No bundled preset states them
  differently any more", both false once `windows-11` does; rewritten to say so (the test
  may run on `windows-11` again, its fills now apart). An M9b file: Task 15's boundary
  condition covers it.
- Gates: the three showcases' route tests (rationale §1.5) with the leaf listed; the
  documented-sizes gate; `--commit`; the GPUI Fast suite; a look at the three showcases
  under `windows-11` is the maintainer's, through the Windows runners' captures, against
  what §11 says they show.
- When: at an M9b boundary where M9b's T19 (the shared list's header) is done or not begun
  and no M9b task is open on the iced `src/contract.rs` test it re-words; should M9c have
  started, no M9c task open on egui's `src/style/variants.rs` (rationale K16).

### 12.7 Documents, by task

Task 11: the facts row, the registry, the inheritance rules, the model's doc comment, the
two READMEs' optional-size sentences, the egui mapping and `mapping.md`, the CHANGELOG's
*Breaking Changes* and first *Added* lines, the `[1.0]` `docs/todo.md` entry. Task 12: the gpui
README row, `TabLook`'s and `TabBar`'s docs, the CHANGELOG line, the showcase's info.
Task 13: the iced README, `tab_flares`'s doc, `styles::aw::tab_bar`'s doc (the flares' place
beside iced_aw's style), the parity exceptions, the CHANGELOG line. Task 14: the egui
showcase's docs. Task 15: the preset values and comments, the facts row's Windows
`active_background` cell, the two gated-size lists, the shared list and its header, the
CHANGELOG lines, the WinUI-brushes entry's close. Task 10 (the close): `docs/todo_gpui-full-theme.md`'s note (§10) and a check that every
item above is in; the iced and egui compatibility stamps
(`scripts/update_compatibility.sh run iced`, `run egui`) are run by whichever of G3, M9b
and M9c closes last, after the last change to that connector's sources, as §10 runs gpui's.

---

## Appendix A -- The regression test of §2.1, verbatim

From `gpui-fast-panic-diagnosis.md` §3c (kept in `.superpowers/sdd/todo_code-review-plan/g3/`), written
against `7492e62f`; the implementer adapts it to the tree it lands on (§2.1). On gpui-fast
0.1.7 it fails with the `measurement has not been performed` panic on any host; it passes on
gpui-pre. It edits `gpui_icons` the way `the_theme_error_alerts_icon_follows_the_chosen_set`
already does.

```rust
/// A Layout page Sidebar item drawn without its icon, then with it, then
/// without again, under presets of different text sizes, is laid out and
/// drawn each time: GPUI Fast 0.1.7 reclaimed the row's earlier nodes with
/// stale child lists and panicked prepainting the unmeasured label
/// (src/elements/text.rs:650). On any host: the icons are removed by hand,
/// not by an icon theme the host cannot read.
#[gpui::test]
fn a_sidebar_item_losing_and_regaining_its_icon_is_drawn(cx: &mut TestAppContext) {
    let (showcase, _root, mut cx) = open(cx, TALL_WINDOW);
    let set = |cx: &mut VisualTestContext, icons: bool, preset: &str| {
        cx.update(|window, cx| {
            showcase.update(cx, |this, cx| {
                this.apply_theme_by_name(preset, window, cx);
                if icons {
                    this.select_icon_set("gpui-component built-in (Lucide)", window, cx);
                } else {
                    this.select_icon_set("Material (bundled)", window, cx);
                    this.gpui_icons.clear();
                }
                cx.notify();
            });
        });
        cx.run_until_parked();
        draw(cx);
        let shown = read(cx, &showcase, |this, _| {
            this.chrome_icon(&IconName::LayoutDashboard).shown()
        });
        assert_eq!(shown, icons, "{preset}: the Sidebar items' icons");
    };
    set(&mut cx, false, "macos-tahoe");
    show(&mut cx, &showcase, Page::Layout);
    set(&mut cx, true, "material-3");
    set(&mut cx, false, "ios-13");
}
```

---

## Appendix B -- Task 11's candidates and their decisions, at `2e80f87e`

Every 4-digit number from 1366 to 1401 in the scope of §12.1 (both ends of a range), by
file and line, with the numbers in parentheses. "Move" means one more (a line of
`docs/platform-facts.md` from `:1366` to `:1400`); "stay" means unchanged. Located by
content at the task's base; a candidate that differs is decided by hand (§12.1).

**Move (123):**

- `connectors/native-theme-gpui/src/geometry.rs`: `:989` (1389)
- `docs/inheritance-rules.toml`: `:599` (1397); `:603` (1395, 1395)
- `docs/todo.md`: `:993` (1369, 1371); `:1035` (1369); `:1607` (1379, 1380); `:2365` (1378)
- `native-theme/src/kde/metrics.rs`: `:102` (1390)
- `native-theme/src/presets/adwaita.toml`: `:283` (1389); `:284` (1390); `:289` (1391, 1392); `:312` (1379); `:314` (1380); `:715` (1389); `:716` (1390); `:721` (1391, 1392); `:744` (1379); `:746` (1380)
- `native-theme/src/presets/documented_sizes.rs`: `:189` (1379, 1380); `:194` (1391, 1392); `:196` (1389, 1390); `:221` (1379, 1380); `:226` (1391, 1392); `:228` (1389, 1390); `:253` (1379, 1380); `:258` (1391, 1392); `:260` (1389, 1390); `:285` (1379, 1380); `:290` (1391, 1392); `:292` (1389, 1390); `:317` (1379, 1380); `:322` (1391, 1392); `:324` (1389, 1390); `:349` (1379, 1380); `:354` (1391, 1392); `:356` (1389, 1390)
- `native-theme/src/presets/kde-breeze.toml`: `:340` (1389); `:341` (1390); `:346` (1391, 1392); `:366` (1369); `:372` (1371); `:880` (1389); `:881` (1390); `:886` (1391, 1392); `:906` (1369); `:912` (1371)
- `native-theme/src/presets/macos-monterey.toml`: `:286` (1389); `:288` (1390); `:293` (1391, 1392); `:298` (1369); `:302` (1378); `:308` (1371); `:753` (1389); `:755` (1390); `:760` (1391, 1392); `:765` (1369); `:769` (1378); `:775` (1371)
- `native-theme/src/presets/macos-tahoe.toml`: `:260` (1369); `:647` (1369)
- `native-theme/src/presets/material-3.toml`: `:336` (1389); `:340` (1383); `:344` (1393); `:350` (1378); `:355` (1370, 1371); `:361` (1379, 1380); `:855` (1389); `:859` (1383); `:863` (1393); `:869` (1378); `:874` (1370, 1371); `:880` (1379, 1380)
- `native-theme/src/presets/windows-11.toml`: `:374` (1389); `:376` (1390); `:381` (1391, 1392); `:393` (1369); `:396` (1372, 1373); `:401` (1378); `:412` (1379, 1380); `:971` (1389); `:973` (1390); `:978` (1391, 1392); `:990` (1369); `:993` (1372, 1373); `:998` (1378); `:1009` (1379, 1380)

**Stay (34)**, with the reason:

- the OS probe's lines (after `probe`): `docs/platform-facts.md:227` (five),
  `native-theme/src/presets/macos-monterey.toml:417`, `:884` (two each),
  `macos-tahoe.toml:345`, `:732`;
- another file's lines: the egui showcase's `chrome.rs:589` (the gpui showcase's
  `demo.rs`), `pages/basic.rs:1499` and `tests.rs:1057` (egui's `scroll_area.rs`,
  `window.rs`); the egui connector's `src/accessors.rs:632` (egui-winit's `lib.rs`),
  `src/atlas.rs:714` and `src/install_tests.rs:625`, `:2892` (egui's `ui.rs`, `window.rs`);
  gpui's `SC/tests.rs:145` and `CN/src/widgets/tests.rs:1035` (gpui-fast's `retained.rs`),
  `CN/src/geometry.rs:1296` (gpui-component's `button.rs`); the iced showcase's
  `:1364-1374` (by content); `docs/todo.md:1880`, `:1903` (`retained.rs`), `:3579`
  (`popup_menu.rs`), `:3667` (gpui's `lib.rs`); `native-theme/src/gnome/mod.rs:291`
  (pango's `fonts.c` `parse_int`, continuing `:289`'s `fonts.c:1411-1447`), `:310`, `:979`
  (pango's `fonts.c`);
- no line: the iced showcase's `TALL_WINDOW` `1400.0`; `docs/todo.md:3165` ("a window 1380
  wide").

The full list with each line's text, as the enumeration printed it, goes in Task 11's commit
body (G1).
