# 0.7.x: the text caret's blink — specification

- Rationale: [`todo_v0.7.1_caret-blink-rationale.md`](todo_v0.7.1_caret-blink-rationale.md)
  (decisions B1 to B6), checked, not yet approved by the maintainer. This
  document says what changes, where, to which value, with which tests and
  gates. It decides nothing the rationale did not decide, except the small
  points listed under *Decisions taken while specifying*; what needs the
  maintainer is under *Open points*, the last section.
- Written: 2026-11-14, against `v0.7.1-rc2` at `0fa28027` (worktree
  `.claude/worktrees/v071`). Every `file:line` is that commit's unless it
  names another version.
- Abbreviations as in the rationale: `GC`, `GB` are gpui-component 0.9.1's and
  gpui-base 0.9.1's `src/` in Cargo's registry; `EG` egui 0.36.2's; `IW`
  iced_widget 0.14.2's; `F` is `docs/facts/` on the parked branch `v0.8.0-rc1`
  at `78ee0efd` (the checkout `/home/tibi/Rust/native-theme`); `CG`, `CE`,
  `CI` the `src/` folders of the gpui, egui and iced connectors.
- Status: not approved. B6 (the version and the release) is the
  maintainer's; this specification is written for the proposed option, B6 (a)
  as a release of its own after v0.7.1, and *Under the other B6 options* says
  what differs.
- The batch's working rules are cited as "rule N (batch-speed rationale,
  Appendix)": the numbered rules of *Appendix — the batch's working rules* in
  `docs/todo_v0.7.1_batch-speed-rationale.md`, authoritative where this
  specification and they differ; below, "rule N" is short for that.
- **This release is the release after `<prev-tag>`** in the maintainer's
  order, item 4 of the list the KDE document's D10 proposes in *The releases
  after v0.7.1* (`docs/issues/0004_kde-open-items-rationale.md`, a document
  still being amended, so cited by its heading). Under that proposal, with the
  KDE steps 2 to 5 as two releases (`docs/todo_v0.7.1_batch-speed-rationale.md`,
  S5), it lands fifth after v0.7.1 (0.7.6): egui 0.37, the KDE steps 2 and 3,
  the KDE steps 4 and 5, and gpui-component's own icons land before it, and
  `<prev-tag>` is the icons'. D10's item 3 leaves open whether the icons'
  follow-up, "the step that serves the two pixel sets (its W7)", is cut as a
  release of its own before this one; where it is, `<prev-tag>` is W7's
  where it stands immediately before, and this release's number is one
  higher (0.7.7 under the proposal); W7 runs in the second lane, so the lane
  predecessor stays steps 4 and 5's. This work runs in the KDE lane (S3 as
  amended by the maintainer on 2026-11-15; *Order and gates*, *The branch*):
  v0.7.1 and the KDE steps 2 to 5 are on the tree this work starts from,
  and egui 0.37 and the icons arrive at the merge. They edit the same files
  (the facts tables, the presets' sections, the egui mapping and its
  counts, the GNOME reader's reads). So every edit below is applied by its
  anchor (the quoted text, the key, the test's name); a line number is
  `0fa28027`'s and serves to find the place; a count is given as today's
  value and as a delta (+3 leaves; the inputs' DERIVED +2 and UNMAPPABLE +1;
  `egui-limited` +1; `gsettings` reads +3), and the implementer adds the
  delta to the value the tree holds then; and the facts rows go right after
  §2.4's `caret_color` row, by A.8's rule, wherever that row is then. No
  line citation is moved or re-pointed (S6, E.5).

## How to read it

Six parts. Each lists its changes by file, its tests and the gates that cover
it.

| Part | Rationale | Files it owns |
|---|---|---|
| 0 — Evidence | B3 | `docs/evidence/winui-host/`, `docs/evidence/ios-host/`, `docs/evidence/README.md` |
| A — Model | B2 | `native-theme-derive/src/` (`parse.rs`, `gen_ranges.rs`, `lib.rs`), `native-theme/src/model/widgets/mod.rs`, `native-theme/src/resolve/` (`validate.rs`, `validate_helpers.rs`, `tests.rs`), `native-theme/src/error.rs` (A.4), `native-theme/tests/proptest_roundtrip.rs`, `docs/property-registry.toml`, `docs/inheritance-rules.toml`, `docs/platform-facts.md` |
| B — Presets | B3 | five presets, `native-theme/src/presets/documented_sizes.rs`, `native-theme/src/presets/README.md`, `docs/new-os-version-guide.md` |
| C — Readers | B4 | `native-theme/src/kde/mod.rs`, `native-theme/src/gnome/mod.rs`, `native-theme/src/watch/gnome.rs`, `native-theme/src/watch/mod.rs`, `native-theme/src/windows.rs`, the Windows reader's comment in `native-theme/Cargo.toml` (C.3) |
| D — Connectors and showcases | B5 | `connectors/native-theme-gpui/` (`src/lib.rs`, `README.md`, the showcase's `info/leaves.rs`), `connectors/native-theme-egui/` (`src/style/base.rs`, `mapping.toml`, `src/mapping.md`, `src/mapping_tests.rs`, `src/contract.rs`, `src/lib.rs`, `README.md`, the showcase's `tests.rs`), `connectors/native-theme-iced/` (`src/contract/derived.rs`, `src/lib.rs`, `README.md`, `examples/showcase-iced.rs`), `docs/showcase-elements.toml`, `tools/showcase-support/src/values.rs` |
| E — Documentation, citations, close-out | B6, §5 | `CHANGELOG.md`, `native-theme/README.md` and its table's copy in `native-theme/src/lib.rs` (E.2), `ROADMAP.md`, `docs/todo.md`, the rule for line citations (E.5), the asset run, both stamps, the archive |

Every edit has one owner, and with no citation re-pointed (S6) no file of
the table is edited by more than one part.

The parts are not the units of work: Part A alone fails every gate that runs
the egui connector's tests (*Order and gates* gives the units, why, and the
gates' cadence).

**Lines that move.** The three facts rows, the three `[no_inheritance]`
entries (A.7) and every other insertion move lines that other files cite by
number. None of those citations is re-pointed (S6, E.5): they are left as
they are and may drift, and the gates that compared a citation's number with
its line no longer do so after v0.7.1's unit (`documented_sizes.rs`,
`tests/platform_facts_xref.rs`, `scripts/check_preset_citations.py`).

## Decisions taken while specifying

They change no result of the rationale; each is the smaller or the safer
choice, and each says what it rests on.

1. **The `u32` range check is a lower bound only** (A.1): a new attribute
   `#[theme(min_u32 = "1")]`, a `RangeCheck::MinU32 { min }` and a helper
   that reports `min: Some(1.0), max: None`, which `RangeViolation` documents
   as open-ended (`native-theme/src/error.rs:19-22`) and prints `1..=inf`.
   A `range_u32 = "1..=4294967295"` mirroring `range_u16` was not taken: it
   would print the type's maximum as a bound, and B2 sets no upper bound.
2. **The two new helpers go at the end of `validate_helpers.rs`**, after
   `check_defaults_ranges` (`:769-913`).
3. **The facts rows go right after `caret_color`** (`docs/platform-facts.md:1209`
   at `0fa28027`), the row the blink fields stand beside in the struct and
   the registry (A.8 gives the rule); no other line of the file changes.
4. **The new facts cells link their sources** in the URL patterns
   `docs/platform-facts.md` already uses for each project, at the pins
   `F/sources.toml` records (A.8 lists them). No link was opened (no network);
   E.6 opens them.
5. **Halving is integer division into both durations, on KDE and GNOME
   alike.** B4 says so for KDE (Qt's `flashTime / 2`) and "halved into both
   durations" for GNOME; an odd `cursor-blink-time` (allowed: 100 to 2500)
   therefore gives two phases 0.5 ms short of GTK's. The halving is written
   `checked_div(2)` on a `u32`, not `/ 2`: the strict-panic run denies
   `clippy::integer_division` and `clippy::arithmetic_side_effects`
   (`scripts/check_strict_panic.sh:88-89`) on native-theme's library
   (`scripts/check_release.sh:572-575`, and per target `:592-595`), and a
   method call is neither; its `Option` joins the parse's `Option` chain, so
   the code needs no `allow` and no fallback value. The file's other form,
   `#[allow(clippy::integer_division, clippy::arithmetic_side_effects)]` with
   a comment (`native-theme/src/color.rs:341`, `winicons.rs:120`), is kept for
   arithmetic that has no checked method.
6. **A GNOME value outside its schema's range is malformed** (B4: "a missing
   or malformed setting leaves the preset's"): `cursor-blink-time` outside
   100..=2500 and `cursor-blink-timeout` outside 1..=2147483647 (the schema,
   gsettings-desktop-schemas 51.0
   `schemas/org.gnome.desktop.interface.gschema.xml.in:70-77`, `:78-85`,
   read 2026-11-14; `F/gnome/tokens.toml:1741`, `:1782` quote the same)
   leave the preset's values, as does text that is not a number.
7. **Part 0 copies the `ios-host-macos-15` image only.** The run's
   `run.toml` gives that image `preset = "ios-13"` and the other,
   `ios-host-macos-26`, `preset = "ios-26"` (commit `72fcf6a7`), a preset
   0.7.x does not have; B3 brings "the cited file", the one ios-13 cites.
8. **The presets' gate** (B.2) extends `documented_sizes.rs`'s
   `soft_field_failures` with the presets platform-facts does not cover but
   another source does (`ios-13`), instead of a second copy of the function.
9. **gpui: the mapping is a private function**, `caret_blink`, not a public
   accessor: B5 rejected an accessor (its option (b)), and a public one would
   need a parity entry for iced and egui (`scripts/check_connector_parity.py`).
10. **gpui: `apply_inner` keeps its signature.** It takes the theme's blink
    from the variant it is handed for the styled theme's mode; only
    `apply_accessibility` hands none, and that is the call that leaves the
    blink (D.1).
11. **gpui: the half-sum is exact.** `(on + off) / 2` milliseconds is taken
    as `(on + off) × 500` microseconds in `u64`, written
    `u64::from(on).saturating_add(u64::from(off)).saturating_mul(500)`: the
    strict-panic run lints the gpui library too
    (`scripts/check_release.sh:624-627`), where `+` and `*` on a runtime
    `u64` trip `clippy::arithmetic_side_effects`; the methods cannot
    saturate (2 × `u32::MAX` × 500 is below `u64::MAX`), so an odd sum loses
    nothing and nothing overflows.
12. **Windows: `build_theme` keeps its signature; `read_windows` applies the
    blink to the result.** `build_theme` has seven parameters
    (`native-theme/src/windows.rs:525-533`), clippy's default
    `too_many_arguments` fires above seven, nothing in `native-theme` allows
    it (no `clippy.toml` beside the crate, no `allow` in its source), and the
    module is compiled on every target (`native-theme/src/lib.rs:339-344`),
    so an eighth parameter would fail `cargo clippy -p native-theme
    --all-targets -- -D warnings` and its `$NT_FEATURES` and cross-target
    legs (`scripts/check_release.sh:465-497`, `:529-541`). Of the two ways,
    (a) a helper `read_windows` calls on `build_theme`'s result and (b) the
    parameter under `#[allow(clippy::too_many_arguments)]`, (a) is taken: it
    needs no lint exception and leaves the 14 test calls of `build_theme`
    as they are, and the tests lose nothing, since the helper is pure and
    is tested on the results of the tests' own `light_reader` and
    `dark_reader` (C.3). Only with the approval of the `unsafe` call (C.5
    says what holds without it). The other signatures this specification
    adds or extends stay within clippy's defaults: no function gets more
    than six parameters (`soft_field_failures` gets its sixth), no new type
    is nested past what `type_complexity` allows (the one new parameter
    type is `&[(&str, Option<T>)]`), and no crate turns on a pedantic lint
    such as `needless_pass_by_value` (the workspace and its crates have no
    `[lints]` table; the crates' `lib.rs` deny only panic-related lints and,
    in gpui, `disallowed_methods`: `native-theme/src/lib.rs:175-177`,
    `connectors/native-theme-gpui/src/lib.rs:223-225`,
    `connectors/native-theme-egui/src/lib.rs:158-164`).
13. **Documentation the rationale does not name, written by the rules the
    neighbouring text follows:** a bullet in the iced connector's *What iced
    cannot take* (crate docs and README), the presets README's gate
    sentence, a bullet per platform in `docs/new-os-version-guide.md`, the
    core README's *Behaviour* list, `error.rs`'s and `validate_with_dpi`'s
    docs of `ResolutionIncomplete` (A.4), and a dated sentence in the backlog's
    *Windows watcher: the window-only sources* entry, which lists the
    sources the Windows watcher does not hear.
14. **`tools/showcase-support` prints a JSON integer as itself.** Its
    `number` reads every number as `f32` (`tools/showcase-support/src/values.rs:192-197`),
    which shows a `u32` above 2^24 rounded: the largest time-out,
    4294967295, would read "4294967300 ms". No value shown today changes
    (every integer leaf today is a `u16`). The egui showcase's test
    `value_line` (`connectors/native-theme-egui/examples/showcase-egui/tests.rs:7095-7106`),
    which states the same rule ("a number as the `f32` it is prints", R11
    §B), changes with it, so the two rules agree above 2^24 (D.4).
15. **The backlog entries of E.4 are tagged `[v0.7.x]`**: the roadmap closes
    the model's gaps in 0.7.x releases (`ROADMAP.md:147-150`).
16. **The gpui showcase states the blink in the text input's leaf rows**
    (`info/leaves.rs`), the form every leaf takes there; the `Input`'s
    `WidgetInfo` (`info/inputs.rs:107-250`) gets no line of its own.
17. **The documents of this work are committed first** (the batch's shared
    rule: `docs/todo_v0.7.x_gpui-widget-icons-spec.md` decision 23,
    `docs/todo_v0.7.1_gpui-kit-0.9.1-spec.md`'s commit (0)). v0.7.1's
    commit (0) adds every document of this cycle the maintainer has approved
    by then, this rationale and this specification among them if they are
    approved by then. This work's first commit, unit 0 (*Order and gates*),
    adds to `docs/` the approved documents of this work that no earlier
    commit added: the rationale, this specification and the plan too, each
    where v0.7.1's commit (0) did not add it. Where nothing
    is left to add, there is no unit 0 and the work starts with unit 2. A
    text that cites one of them at a commit cites the commit that added it.
18. **Withdrawn** (S6, decided by the maintainer on 2026-11-15): the
    citations already off at `0fa28027` are left as they are, and there is
    no unit 1 that re-pointed them (E.5).

## Corrections to the rationale found while specifying

None changes a decision.

- §1.1, iOS: `F` cites two images for each blink value, `ios-host-macos-15`
  and `ios-host-macos-26` (`blink_on`'s evidence at `F/ios/tokens.toml:20618` and `:20624`, `blink_off`'s at `:20636` and `:20642`);
  the rationale names the first. Decision 7 says which one is copied.
- §1.1, Windows and §4: the measured file is `frames.toml`, whose
  `[system]` table holds `GetCaretBlinkTime = 530` at line 248
  (`git show efbffffe:docs/evidence/winui-host/2026-11-06-run-42308526155/winui-host-windows-11-arm/frames.toml`).
- §4 lists the output of `gsettings get` as unread. Read on 2026-11-14 on
  this machine (a KDE Plasma session, so the values are not GNOME's
  defaults; only the form is used): `cursor-blink-time` printed `1000`,
  `cursor-blink-timeout` `10`, `cursor-blink` `true`, each on one line,
  unquoted.
- B5's egui paragraph names the sinks per field. `mapping.toml`'s test T2
  needs one more: `caret_blink_off` also declares `on_duration`, because a
  theme that states no `caret_blink_off` leaves `on_duration` to egui (D.2).
- B3's "the README sections to add there" are two: `winui-host/` and
  `ios-host/` (Part 0).

## Part 0 — Evidence

Implements B3's evidence paragraph. First of all the parts, after unit 0,
where there is one (*Order and gates*), which edits only documents: the iOS
commit is on one local branch only, `facts-ios-m1b3`
(`git for-each-ref --contains 72fcf6a7`, read 2026-11-14), so it is copied
before anything else is done with that branch. (The WinUI commit
`efbffffe` is on `v0.8.0-tools-hosts-m1b`, which is also on `origin`.)
Both commits are in this repository's object store, which the worktree shares
(`git rev-parse --git-common-dir` is `/home/tibi/Rust/native-theme/.git`).

### 0.1 The files

Each is written with `git show <commit>:<path> > <path>` at the same path,
unchanged, and checked with `git hash-object <path>` against the blob id
below (`git ls-tree -r <commit>`).

| Commit | Path | Blob | What it is |
|---|---|---|---|
| `efbffffe` | `docs/evidence/winui-host/2026-11-06-run-42308526155/README.md` | `ea36fef1` | the run's own description |
| `efbffffe` | `docs/evidence/winui-host/2026-11-06-run-42308526155/run.toml` | `529b55ae` | its record: image, OS, toolkit |
| `efbffffe` | `docs/evidence/winui-host/2026-11-06-run-42308526155/winui-host-windows-11-arm/frames.toml` | `8ddb6f7f` | the cited file: `[system]` `GetCaretBlinkTime = 530` at line 248 |
| `72fcf6a7` | `docs/evidence/ios-host/2026-11-06-run-42275028573/README.md` | `24db20e6` | the run's own description |
| `72fcf6a7` | `docs/evidence/ios-host/2026-11-06-run-42275028573/run.toml` | `ac774a59` | its record |
| `72fcf6a7` | `docs/evidence/ios-host/2026-11-06-run-42275028573/analysis/behavior_values.py` | `3d1d5b2f` | the script the values are derived with; its docstring is the method |
| `72fcf6a7` | `docs/evidence/ios-host/2026-11-06-run-42275028573/ios-host-macos-15/behavior_values.toml` | `ed5470ef` | the cited file: `blink_on_s = 0.65` (line 11), `blink_off_s = 0.35` (12), `repeat_count` (18), `samples_span_s` (19), `blink_timeout = "none"` (20) |
| `72fcf6a7` | `…/ios-host-macos-15/measure1/index.toml` | `7ebebb4c` | the script's input |
| `72fcf6a7` | `…/ios-host-macos-15/measure1/screen/light/caret/default/system/text_view.png` | `92e8648f` | the script's input |
| `72fcf6a7` | `…/ios-host-macos-15/measure1/screen/dark/caret/default/system/text_view.png` | `c5e0f461` | the script's input |

Not copied: the WinUI run's recordings and text-scale run (its README names
them); the `ios-host-macos-26` image (decision 7); the seven other crops of
`ios-host-macos-15` (nine in all) that the earlier commit `b38c1795`
(on `origin/v0.8.0-tools-ios-m2`) holds; and that image's
`measure-differ-levels.txt` (`b38c1795`, blob `bc7eb25f`, 1,192,955 bytes),
the two runs' differences, which no value of this release reads. The iOS
README's "Kept here" list (both images, all crops, the differ file) is the
evidence repository's and cannot match a set kept for one preset's two
values, so carrying the differ file alone would not make it match; the
folder README (0.2) says what is kept. The iOS README also links run
42268160866's README (`../2026-11-06-run-42268160866/README.md`), which is
not kept: 0.2 says so. A recorded file is not edited
(`docs/evidence/README.md:8-9`), so the two copied READMEs stay as they are.

Size: `measure1/index.toml` is 811,237 bytes, about six times the largest
file `docs/evidence/` holds today (135,880 bytes,
`os-probe/2026-10-26-run-40348854007/os-probe-macos-15/not-key/probe.toml`).
No gate limits a file's size (`scripts/check_release.sh` and the workflows
have no size check; `.gitattributes` only marks `docs/assets/*.svg` binary),
and `docs/` is in no crate's package.

Check, verified on 2026-11-14 on copies of the five files: `python3 -I
docs/evidence/ios-host/2026-11-06-run-42275028573/analysis/behavior_values.py
docs/evidence/ios-host/2026-11-06-run-42275028573/ios-host-macos-15`
rewrites `behavior_values.toml` byte for byte (`cmp` against the blob) with
Pillow 13.0.0, printing one `DeprecationWarning` (`Image.getdata`, which
Pillow 14 removes; the script's line 121). After the copy the same command
and `git diff --exit-code` on the file repeat the check in the tree.

### 0.2 `docs/evidence/README.md`

Two sections appended after `## breeze-offscreen/` (the file ends at line
47), in the form of `## os-probe/` (`:13-32`):

```
## `winui-host/`

What `tools/winui-host` (on the parked branch `v0.8.0-tools-hosts-m1b`,
never published), a WinUI 3 application dispatched through that branch's
`.github/workflows/os-probe.yml` (its winui-host job, which this branch's
workflow does not have), read on a GitHub Windows runner. Kept here: the file a citation reads, with the run's own `README.md`
and `run.toml`, copied unchanged from this repository's commit `efbffffe`.
The run's README describes the run as the evidence repository keeps it, and
names recordings and a text-scale run that are not kept here.

| Run | Date | Commit | Why |
|-----|------|--------|-----|
| [`2026-11-06-run-42308526155/`](winui-host/2026-11-06-run-42308526155/) ([run](https://github.com/tiborgats/native-theme/actions/runs/42308526155)) | 2026-11-06 | `7bad463f` on `v0.8.0-tools-hosts-m1b` | `GetCaretBlinkTime`, windows-11's caret blink ([specification](../todo_v0.7.x_caret-blink-spec.md) Part 0) |

A citation links the file and its line
(`evidence/winui-host/2026-11-06-run-42308526155/winui-host-windows-11-arm/frames.toml#L248`).

## `ios-host/`

What `tools/ios-host` (on the parked branch `v0.8.0-tools-ios-m2`, never
published), an iOS application run in the iOS Simulator by that branch's
`.github/workflows/os-probe.yml` (its ios-host job, which this branch's
workflow does not have), read on a GitHub macOS runner. Kept here, copied
unchanged from this repository's commit
`72fcf6a7`: the run's own `README.md` and `run.toml`, the script that derives
the cited values (`analysis/behavior_values.py`, its docstring the method),
and of the image `ios-host-macos-15` (the iOS 18.6 simulator, preset
`ios-13`) the cited `behavior_values.toml` and the inputs the script reads
(`measure1/index.toml` and the two caret crops). The run's other image,
`ios-host-macos-26`, is the parked `ios-26` preset's and is not kept, nor are
the image's other crops and `measure-differ-levels.txt`, which the run's
README lists as kept, nor the README of run 42268160866 it links to: they
are in the evidence repository and in this repository's commit `b38c1795`
(on `v0.8.0-tools-ios-m2`).

| Run | Date | Commit | Why |
|-----|------|--------|-----|
| [`2026-11-06-run-42275028573/`](ios-host/2026-11-06-run-42275028573/) ([run](https://github.com/tiborgats/native-theme/actions/runs/42275028573)) | 2026-11-06 | `7bb1e784` on `v0.8.0-tools-ios-m2` | ios-13's caret blink ([specification](../todo_v0.7.x_caret-blink-spec.md) Part 0) |

`python3 -I docs/evidence/ios-host/2026-11-06-run-42275028573/analysis/behavior_values.py docs/evidence/ios-host/2026-11-06-run-42275028573/ios-host-macos-15`
writes `behavior_values.toml` again from the kept inputs (Pillow). A citation
links the file and its line
(`evidence/ios-host/2026-11-06-run-42275028573/ios-host-macos-15/behavior_values.toml#L11`).
```

The two links to this specification are relative to `docs/evidence/`:
`../todo_v0.7.x_caret-blink-spec.md` while it is in `docs/`; E.6 changes
them to `../archive/todo_v0.7.x_caret-blink-spec.md` when it archives it.

## Part A — The model

Implements B2 and B1's place. Struct, derive, validation, registry, rules
and facts rows land in one unit of work with Part B and D.2 (*Order and gates*, unit 3): the registry key, the
derive inventory and the facts row are checked against each other (A.10),
and the new leaves need egui's mapping rows at once.

### A.1 The derive: a `u32` lower bound

`native-theme-derive/src/parse.rs`:

- `RangeCheck` (`:18-29`) gains `MinU32 { min: u32 }`, its doc
  `` /// `check_min_u32(self.field, min, ...)`: at least `min`, no upper bound ``.
- `parse_one_field` (`:268-436`): the `is_range_check` test (`:319-321`)
  gains `|| meta.path.is_ident("min_u32")`, and its message (`:323`) becomes
  `"only one of check, range, range_u16 and min_u32 may be given"`. A new
  branch after `range_u16`'s (`:392-397`): reads the value as a `LitStr`,
  parses its trimmed text as `u32`, on failure the error
  `"min_u32 must be a u32"` at the literal's span, else sets
  `range_check = Some(RangeCheck::MinU32 { min })`.
- Tests in its `mod tests` (`:497` on), after `range_u16_min_above_max_errors`:
  `min_u32_is_parsed` (`#[theme(category = "soft_option", min_u32 = "1")]`
  on `pub caret: Option<u32>` parses to `Some(RangeCheck::MinU32 { min: 1 })`,
  matched with `matches!`), `min_u32_not_a_u32_errors` (`min_u32 = "-1"`:
  the message contains `"u32"`), `min_u32_beside_another_range_errors`
  (`range_u16 = "1..=2", min_u32 = "1"`: contains `"only one"`).

`native-theme-derive/src/gen_ranges.rs`: `gen_check_stmts` (`:35-160`) gains
an arm after `RangeU16` (`:83-96`) that emits
`crate::resolve::validate_helpers::check_min_u32(#value, #min_lit, prefix, #field_name, errors);`.
The soft-option wrapping (`:98-103`) applies unchanged: a `None` is not
checked.

`native-theme-derive/src/lib.rs`: the field attribute list (`:45-54`) gains,
after the `range_u16` line,
``/// - `#[theme(min_u32 = "1")]` -- u32 lower bound, no upper bound``.

`native-theme/src/resolve/validate_helpers.rs`, at the end of the file
(decision 2): `pub(crate) fn check_min_u32(value: u32, min: u32, prefix:
&str, field: &str, errors: &mut Vec<crate::error::RangeViolation>)`, which
pushes `RangeViolation { path: format!("{prefix}.{field}"), value:
f64::from(value), min: Some(f64::from(min)), max: None }` where `value <
min`; rustdoc in the form of `check_range_u16`'s (`:415-418`), stating that
there is no upper bound.

### A.2 The three fields

`native-theme/src/model/widgets/mod.rs`, `InputTheme` (`:114-156`), right
after `caret_color` (`:121-122`, B1 (a): beside it). Exact text:

```rust
    /// How long the caret is shown in each blink cycle, in milliseconds, 1 or
    /// more. A blink is stated whole, with
    /// [`caret_blink_off`](Self::caret_blink_off), or not at all; `None`
    /// where the theme states none, and the toolkit's own blink applies
    /// (docs/platform-facts.md §2.4).
    #[serde(rename = "caret_blink_on_ms")]
    #[theme(category = "soft_option", min_u32 = "1")]
    pub caret_blink_on: Option<u32>,
    /// How long the caret is hidden in each blink cycle, in milliseconds; 0
    /// means the caret does not blink, whatever
    /// [`caret_blink_on`](Self::caret_blink_on) is. `None` where the theme
    /// states no blink (docs/platform-facts.md §2.4).
    #[serde(rename = "caret_blink_off_ms")]
    #[theme(category = "soft_option")]
    pub caret_blink_off: Option<u32>,
    /// How long after the last input the caret stops blinking, shown, in
    /// milliseconds, 1 or more; stated only beside a blink. `None` where the
    /// platform's caret never stops (KDE, Material, iOS) or no source gives
    /// its time-out (Windows, macOS) (docs/platform-facts.md §2.4).
    #[serde(rename = "caret_blink_timeout_ms")]
    #[theme(category = "soft_option", min_u32 = "1")]
    pub caret_blink_timeout: Option<u32>,
```

The derive gives `ResolvedInputTheme` the same three fields as `Option<u32>`
with the same docs (`native-theme-derive/src/gen_structs.rs:56-63`), adds
the keys to `InputTheme::FIELD_NAMES` (so `Theme::lint_toml` knows them),
passes them through validation unchanged (`gen_validate.rs`, the
`SoftOption` arm) and checks `caret_blink_on` and `caret_blink_timeout`
where stated (A.1). No `inherit_from`: no rule fills them (A.7).

### A.3 Merge

Generated, field by field (`native-theme-derive/src/gen_merge.rs:31-59`): an
overlay's `Some` replaces the base's, its `None` leaves it. So an overlay
that states one duration over a preset that states both gives a whole blink,
and over one that states none a half blink, which validation rejects (A.4);
an overlay cannot unset a field (B2). Nothing is written by hand.

### A.4 "A blink is stated whole or not at all"

`native-theme/src/resolve/validate_helpers.rs`, at the end of the file after
`check_min_u32`: `pub(crate) fn check_caret_blink(input: &InputTheme,
prefix: &str, missing: &mut Vec<String>)`. It reads the source variant's
three fields and pushes at most one path, `format!("{prefix}.caret_blink_off_ms")`
or `format!("{prefix}.caret_blink_on_ms")`, by this table (O: `caret_blink_on`
stated; T: `caret_blink_timeout` stated; `—`: unset):

| `caret_blink_on` | `caret_blink_off` | `caret_blink_timeout` | Result |
|---|---|---|---|
| — | — | — | valid: the theme states no blink |
| — | — | T | missing `caret_blink_off_ms` |
| O | — | — | missing `caret_blink_off_ms` |
| O | — | T | missing `caret_blink_off_ms` |
| — | 0 | — | valid: no blink |
| — | 0 | T | valid: no blink; the time-out means nothing |
| O | 0 | — | valid: no blink |
| O | 0 | T | valid: no blink |
| — | 1 or more | — | missing `caret_blink_on_ms` |
| — | 1 or more | T | missing `caret_blink_on_ms` |
| O | 1 or more | — | valid: blinks, never stops |
| O | 1 or more | T | valid: blinks, stops T ms after the last input |

The rule, as the function states it: `caret_blink_off` unset while either
other field is stated names `caret_blink_off_ms`; `caret_blink_off` above 0
while `caret_blink_on` is unset names `caret_blink_on_ms`. The keys are
written as literals (`InputTheme` derives no `toml_key`); the tests below pin
them to the serde renames. Rustdoc: the rule, B2's reason in one sentence
("a connector or an application then has three cases to tell"), and that a
range check of a stated field runs after it (`check_min_u32`).

The range checks (A.1) then run in Phase 2: `caret_blink_on` 0 is
`input.caret_blink_on_ms` out of range even beside `caret_blink_off` 0 (B2:
"stated values 1 or more"); `caret_blink_timeout` 0 is
`input.caret_blink_timeout_ms` out of range.

`native-theme/src/resolve/validate.rs`, `validate_with_dpi`: one line after
the per-widget extraction (`let link = …;`, `:162`) and before the Phase 1
short-circuit (`:164-168`):
`validate_helpers::check_caret_blink(&self.input, "input", &mut missing);`.
A missing path is then `Error::ResolutionIncomplete` like any other
(`:166-167`), before the range checks; a path from a whole theme gets its
`light.` or `dark.` prefix as the others do (`native-theme/src/error.rs:122-130`).

Two docs then say more than "missing": `native-theme/src/error.rs:117`,
"/// Theme resolution found missing fields that could not be inherited.",
becomes "/// Theme resolution found missing fields that could not be
inherited, or a caret blink stated in part (`input.caret_blink_*_ms`),
named by the key to add." (re-wrapped as rustfmt leaves comments: by
hand, at the file's width), and `validate_with_dpi`'s `# Errors`
(`native-theme/src/resolve/validate.rs:35-36`), "Returns
[`crate::Error::ResolutionIncomplete`] if any required fields are still
`None` after resolution.", gains ", or if `input` states half a caret blink
(the key to add, as `Error::ResolutionIncomplete` names it)". Its `Display`
(`error.rs:260-296`) lists the path under "widget fields" (`field_category`,
`:318-328`) and needs no change. The one citation of `error.rs` lines
(`connectors/native-theme-egui/src/watch.rs:139`, `:60-62`) is above the
edit.

### A.5 Tests of A.1 to A.4

`native-theme/src/resolve/tests.rs`, at its end (`:3108`), with its helpers
`fully_populated_variant` (`:780`), `missing_paths` and
`range_violation_paths` (`:2741-2757`):

1. `a_blink_stated_whole_or_not_at_all_validates`: the seven valid rows of
   A.4 (its rows 1, 5, 6, 7, 8, 11 and 12, with 500 for a stated duration
   and 10000 for a stated time-out), each on `fully_populated_variant()`
   with `input` set, `validate()` is `Ok`, and the resolved `input` carries
   the three values as set.
2. `half_a_blink_names_the_key_to_add`: the five invalid rows (2, 3, 4, 9
   and 10), each
   `missing_paths(&v)` equal to the one path the table gives
   (`["input.caret_blink_off_ms"]` or `["input.caret_blink_on_ms"]`).
3. `a_zero_shown_duration_or_time_out_is_out_of_range`: `on 0, off 500` and
   `on 0, off 0` give `range_violation_paths == ["input.caret_blink_on_ms"]`;
   `on 500, off 500, timeout 0` gives `["input.caret_blink_timeout_ms"]`; the
   violation's `min` is `Some(1.0)` and `max` `None`.
4. `the_largest_time_out_validates`: `on 500, off 500, timeout u32::MAX` is `Ok`.
5. `an_overlay_merges_a_blink_field_by_field`: `Theme::preset("adwaita")`
   merged with `Theme::from_toml` of `[light.input] caret_blink_off_ms = 0`
   and `[dark.input] caret_blink_off_ms = 0` resolves with
   `caret_blink_on == Some(600)`, `caret_blink_off == Some(0)`,
   `caret_blink_timeout == Some(10000)` (Part B's preset values, in the same
   unit of work); `Theme::preset("nord")` merged with
   `[light.input] caret_blink_on_ms = 400` fails `resolve(ColorMode::Light)`
   with `ResolutionIncomplete` whose `missing` contains
   `"light.input.caret_blink_off_ms"`.

`native-theme/tests/proptest_roundtrip.rs`, `arb_input_theme` (`:261-284`):
the strategy tuple gains three `proptest::option::of(any::<u32>())` and the
struct literal (`:269-283`) the three fields from them. It is the one struct
literal of `InputTheme` with every field named (`git grep -n 'InputTheme {'`:
the other two, `native-theme/src/model/widgets/mod.rs:1626` and
`native-theme/tests/merge_behavior.rs:347`, end in `..Default::default()`).
The round trip proves the renames serialise and parse back for any `u32`.

### A.6 `docs/property-registry.toml`

- Header, `:24-25`: `#   "u16"    — 16-bit unsigned int (CSS font-weight: 400=Normal, 700=Bold;`
  / `#              integer milliseconds for an `_ms` key)` become
  `#   "u16"    — 16-bit unsigned int (CSS font-weight: 400=Normal, 700=Bold;`
  / `#              integer milliseconds for `tooltip.show_delay_ms`)` and,
  after them, `#   "u32"    — 32-bit unsigned int (integer milliseconds for the`
  / `#              caret's blink, `input.caret_blink_*_ms`)`.
- `[input]`, after `caret_color` (`:206`), keys padded to column 30 as the
  table's are:

```toml
caret_blink_on_ms            = "u32"
caret_blink_off_ms           = "u32"
caret_blink_timeout_ms       = "u32"
```

### A.7 `docs/inheritance-rules.toml`

- `:444-445`, `    # soft_option sizes, enums, flags and the tooltip delay: stay None where the`
  / `    # platform states none`, become
  `    # soft_option sizes, enums, flags, the tooltip delay and the caret's blink:`
  / `    # stay None where the platform states none` (two lines, as before).
- After `"tooltip.show_delay",` (`:461`), aligned as it is:

```toml
    "input.caret_blink_on",            # milliseconds: macOS's blink is not established (§2.4)
    "input.caret_blink_off",           # milliseconds, 0 no blink: macOS's is not established (§2.4)
    "input.caret_blink_timeout",       # milliseconds: KDE's, Material's and iOS's caret never stops (§2.4)
```

### A.8 `docs/platform-facts.md`: three rows

**Placement.** A new row goes beside the row it belongs with, and the rows
must stay inside §2.4 (`check_platform_facts_rows.py`). The row they belong
with is `caret_color` (`:1209` at `0fa28027`), which the blink fields stand
beside in the struct (A.2) and the registry (A.6): the three rows go right
after it, in the order below, found by that row's name. No other line of
the file changes: no blank line is dropped, since no citation is moved for
the lines below them (S6).

Columns: Property, macOS, Windows, KDE, GNOME, Material (`:1201`). Pins
from `F/sources.toml`:
plasma-integration v6.8.2 (`:85-96`), qtbase v6.12.0 (`:496-502`),
qtdeclarative v6.12.0 (`:511-517`), GTK 4.24.1 (`:586-592`),
gsettings-desktop-schemas 51.0 (`:631-642`), androidx
`e4d933c98df7baa7ea2b83eea6e086915961e0ff` (`:832-843`; its note says the
GitHub mirror `androidx/androidx` holds the same commits),
microsoft-ui-xaml `winui3/release/2.5.1` =
`ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd` (`:861-871`). URL patterns as the
file already uses them: `invent.kde.org/plasma/plasma-integration/-/blob/<tag>/`,
`github.com/qt/qtbase/blob/<tag>/`, `github.com/qt/qtdeclarative/blob/<tag>/`,
`gitlab.gnome.org/GNOME/gtk/-/blob/<tag>/`,
`gitlab.gnome.org/GNOME/gsettings-desktop-schemas/-/blob/<tag>/`,
`github.com/androidx/androidx/blob/<sha>/`,
`github.com/microsoft/microsoft-ui-xaml/blob/<sha>/`. The source of each
cell is `F`'s entry named before it; the macOS cells cite `F` itself,
since its entries are pending. The GNOME cells cite what the session read on
2026-11-14 besides: gsettings-desktop-schemas 51.0
`schemas/org.gnome.desktop.interface.gschema.xml.in`, `cursor-blink` lines
63-69 (type `b`, default `true`, "Whether the cursor should blink."),
`cursor-blink-time` lines 70-77 (type `i`, range 100..2500, default 1200,
"Length of the cursor blink cycle, in milliseconds."), `cursor-blink-timeout`
lines 78-85 (type `i`, range 1..2147483647, default 10, "Time after which
the cursor stops blinking, in seconds."); GTK 4.24.1 `gtk/gtksettings.c`,
the properties `gtk-cursor-blink` (boolean, default TRUE),
`gtk-cursor-blink-time` (int 100..G_MAXINT, default 1200) and
`gtk-cursor-blink-timeout` (int 1..G_MAXINT, default 10, "The timer is reset
after each user interaction. Setting this to zero has the same effect as
setting gtk-cursor-blink to FALSE."), cited at the lines the parked facts
quote for the time and the time-out (`gtksettings.c:364-366`, `:378-380`,
`F/gnome/tokens.toml:1745`, `:1787`) and by name for `gtk-cursor-blink`,
which the facts do not quote; and GTK 4.24.1 `gtk/gtktext.c`, `blink_cb`,
which past the time-out ("we've blinked enough without the user doing
anything, stop blinking") sets `priv->cursor_alpha = 1.0` and removes the
blink: the caret is shown and stays, the model's meaning of the time-out
(B2). The Windows no-blink value is Microsoft's page of `GetCaretBlinkTime`,
read 2026-11-14 (open point 2).

The first row, `caret_blink_on` (`F/kde/tokens.toml:1599-1617`, `F/gnome/tokens.toml:1731-1751`,
`F/windows/tokens.toml:2711-2727`, `F/macos/tokens.toml:46129-46135`,
`:47501-47507`, `F/material/tokens.toml:1758-1766`):

```
| `caret_blink_on` | ❓ — no source gives it: `NSTextInsertionIndicator.h` (Xcode 16.4) declares no blink period, and the blink's user defaults were unset on the test runner; pending in the 0.8.0 facts (`docs/facts/macos/tokens.toml:47501-47507` on `v0.8.0-rc1` at `78ee0efd`, U605) · macOS 26: ❓ — pending as well (`:46129-46135`, U316) | ⚙ 530 **(API)** — `GetCaretBlinkTime` read on the WinUI host's Windows 11 image ([frames.toml:248](evidence/winui-host/2026-11-06-run-42308526155/winui-host-windows-11-arm/frames.toml#L248)); WinUI's text box shows the caret for one blink time, then hides it for one ([TextBoxHelpers.cpp:55-89](https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/dxaml/xcp/core/text/TextBox/TextBoxHelpers.cpp#L55-L89)) | ⚙ 500 **(impl)** — half Qt's cursor flash time, by integer division ([qwidgetlinecontrol.cpp:1497-1509](https://github.com/qt/qtbase/blob/v6.12.0/src/widgets/widgets/qwidgetlinecontrol.cpp#L1497-L1509); Qt Quick's [qquicktextinput.cpp:4574-4578](https://github.com/qt/qtdeclarative/blob/v6.12.0/src/quick/items/qquicktextinput.cpp#L4574-L4578)), which plasma-integration sets from kdeglobals `[KDE] CursorBlinkRate`: 1000 where unset, a positive value bounded to 200..2000, 0 or less no blink ([khintssettings.cpp:79-80](https://invent.kde.org/plasma/plasma-integration/-/blob/v6.8.2/qt6/src/platformtheme/khintssettings.cpp#L79-L80)) | ⚙ 600 **(GTK)** — half `cursor-blink-time`, "Length of the cursor blink cycle, in milliseconds.", 1200 by default, 100 to 2500 ([org.gnome.desktop.interface.gschema.xml.in:70-77](https://gitlab.gnome.org/GNOME/gsettings-desktop-schemas/-/blob/51.0/schemas/org.gnome.desktop.interface.gschema.xml.in#L70-L77); GTK's `gtk-cursor-blink-time`, [gtksettings.c:364-366](https://gitlab.gnome.org/GNOME/gtk/-/blob/4.24.1/gtk/gtksettings.c#L364-L366)): the cycle's visible half, shown, then fading out ([gtktext.c:6866-6873](https://gitlab.gnome.org/GNOME/gtk/-/blob/4.24.1/gtk/gtktext.c#L6866-L6873)) | 500 **(impl)** — Compose shows the cursor 500 ms, then hides it 500 ms ([CursorAnimationState.kt:76-83](https://github.com/androidx/androidx/blob/e4d933c98df7baa7ea2b83eea6e086915961e0ff/compose/foundation/foundation/src/commonMain/kotlin/androidx/compose/foundation/text/input/internal/CursorAnimationState.kt#L76-L83)) |
```

The second, `caret_blink_off` (`F/kde/tokens.toml:1619-1637`, `F/gnome/tokens.toml:1753-1771`,
`F/windows/tokens.toml:2729-2737`, `F/macos/tokens.toml:46137-46141`,
`:47509-47513`, `F/material/tokens.toml:1768-1776`; the GNOME switch and
the Windows no-blink value as read on 2026-11-14, above):

```
| `caret_blink_off` | ❓ — as `caret_blink_on` (`docs/facts/macos/tokens.toml:47509-47513` on `v0.8.0-rc1` at `78ee0efd`, U605) · macOS 26: ❓ — pending as well (`:46137-46141`, U316) | ⚙ 530 **(API)** — the same blink time, hidden ([frames.toml:248](evidence/winui-host/2026-11-06-run-42308526155/winui-host-windows-11-arm/frames.toml#L248)); 0 where `GetCaretBlinkTime` returns `INFINITE`: no blink ([GetCaretBlinkTime](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcaretblinktime), read 2026-11-14); WinUI's text box takes 500 where the call returns 0, a failure ([TextBoxHelpers.cpp:55-89](https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/dxaml/xcp/core/text/TextBox/TextBoxHelpers.cpp#L55-L89)) | ⚙ 500 **(impl)** — the other half; 0 where `CursorBlinkRate` is 0 or less: no blink ([khintssettings.cpp:79-80](https://invent.kde.org/plasma/plasma-integration/-/blob/v6.8.2/qt6/src/platformtheme/khintssettings.cpp#L79-L80)) | ⚙ 600 **(GTK)** — the other half: hidden, then fading in ([gtktext.c:6866-6873](https://gitlab.gnome.org/GNOME/gtk/-/blob/4.24.1/gtk/gtktext.c#L6866-L6873)); 0 where `cursor-blink`, "Whether the cursor should blink.", `true` by default, is off: no blink ([org.gnome.desktop.interface.gschema.xml.in:63-69](https://gitlab.gnome.org/GNOME/gsettings-desktop-schemas/-/blob/51.0/schemas/org.gnome.desktop.interface.gschema.xml.in#L63-L69); GTK's `gtk-cursor-blink`, [gtksettings.c](https://gitlab.gnome.org/GNOME/gtk/-/blob/4.24.1/gtk/gtksettings.c)) | 500 **(impl)** — the hidden 500 ms ([CursorAnimationState.kt:76-83](https://github.com/androidx/androidx/blob/e4d933c98df7baa7ea2b83eea6e086915961e0ff/compose/foundation/foundation/src/commonMain/kotlin/androidx/compose/foundation/text/input/internal/CursorAnimationState.kt#L76-L83)) |
```

The third, `caret_blink_timeout` (`F/kde/tokens.toml:1641-1650`, `F/gnome/tokens.toml:1773-1789`,
`F/windows/tokens.toml:2711-2718`, `:2739-2742`, `F/macos/tokens.toml:46543-46547`,
`:47693-47700`, `F/material/tokens.toml:1778-1787`):

```
| `caret_blink_timeout` | ❓ — not established; pending in the 0.8.0 facts (`docs/facts/macos/tokens.toml:47693-47700` on `v0.8.0-rc1` at `78ee0efd`, U606) · macOS 26: ❓ — pending as well (`:46543-46547`, U328) | ❓ — not one value: WinUI's text box stops blinking after the whole blink periods that fit in 5 s, three periods where none fits (`CheckCaretBlinkTimeout`, [TextBoxView.cpp:2742-2763](https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/dxaml/xcp/core/native/text/Controls/TextBoxView.cpp#L2742-L2763)), a rule over the blink time; the system's own `SPI_GETCARETTIMEOUT` is not read | **(none)** — never stops: Qt's blink timer runs for as long as the field has focus ([qwidgetlinecontrol.cpp:1497-1531](https://github.com/qt/qtbase/blob/v6.12.0/src/widgets/widgets/qwidgetlinecontrol.cpp#L1497-L1531)) | ⚙ 10000 **(GTK)** — `cursor-blink-timeout`, "Time after which the cursor stops blinking, in seconds.", 10 by default, 1 to 2147483647 ([org.gnome.desktop.interface.gschema.xml.in:78-85](https://gitlab.gnome.org/GNOME/gsettings-desktop-schemas/-/blob/51.0/schemas/org.gnome.desktop.interface.gschema.xml.in#L78-L85); GTK's `gtk-cursor-blink-timeout`, an int 1..G_MAXINT, 10 by default ([gtksettings.c:378-380](https://gitlab.gnome.org/GNOME/gtk/-/blob/4.24.1/gtk/gtksettings.c#L378-L380)), whose doc comment says "The timer is reset after each user interaction"); past it the caret is shown and stays (`blink_cb`, [gtktext.c](https://gitlab.gnome.org/GNOME/gtk/-/blob/4.24.1/gtk/gtktext.c)) | **(none)** — never stops: the blink loops until the cursor animation is cancelled ([CursorAnimationState.kt:78-83](https://github.com/androidx/androidx/blob/e4d933c98df7baa7ea2b83eea6e086915961e0ff/compose/foundation/foundation/src/commonMain/kotlin/androidx/compose/foundation/text/input/internal/CursorAnimationState.kt#L78-L83)) |
```

The table has no iOS column; ios-13 cites its evidence in the preset (B3,
B.1). The KDE tag **(impl)** follows the KDE cell of `show_delay`
(`:1290`), which tags Qt's constants so. The Windows tag is **(API)**, "System
API, function, or queryable constant" (`:11`), as `show_delay`'s Windows cell
tags `SPI_GETMOUSEHOVERTIME`'s reading on the probe images; **(measured)**
(`:21`) is for values measured from rendered pixels.

### A.10 Checkers and tests a new field meets

| Check | What it requires of the three fields | Where it is met |
|---|---|---|
| `property_registry_keys_match_derive_inventory` (`native-theme/src/model/mod.rs:2055`) | each serialised key of `InputTheme` is a key of `[input]` and back (types are not compared, `:2050-2052`) | A.6 |
| `scripts/check_platform_facts_rows.py` | a §2.4 row per registry key, named by the key without `_ms` (its docstring, "Field names") | A.8 |
| `scripts/check_section_refs.py` | each `§` in the registry, the inheritance rules, `docs/showcase-elements.toml` and `docs/new-os-version-guide.md` names a facts section | A.7's `(§2.4)`, B.3 |
| `scripts/check_wrong_safety_nets.py` | nothing: no `[uniform]` rule names the fields | — |
| `scripts/check_todo_citations.py` | a key path in `docs/todo.md` names a key its preset has; a preset line citation carries `at <sha>` | E.4 cites no preset by line, and names `input.caret_blink_*_ms` keys only as model paths |
| `scripts/check_preset_citations.py` | a `platform-facts.md:N` above a value cites a row named after its key (its line-number comparison dropped by v0.7.1's unit, S6) | nothing: B.1's comments name the facts file and section without a line number, `platform-facts.md (§2.4)` (S6); the checker reads them as citations and compares none (v0.7.1 Part I, I.3) |
| `no_inheritance_fields_are_not_inherited` (`native-theme/src/resolve/inheritance.rs:816`) | no rule inherits a `[no_inheritance]` field | A.7 |
| `documented_sizes.rs` | nothing by itself; B.2 adds the gate the tooltip delay has | B.2 |
| `proptest_roundtrip.rs` | the one full struct literal compiles | A.5 |
| egui `every_leaf_has_exactly_one_row_and_every_row_is_well_formed`, `verdict_totals_match_the_specification`, `mapping_md_is_current`, T2, T10 | a row per new leaf, the counts | D.2 |
| iced `every_unreachable_value_is_one_a_preset_loses`; the iced showcase's `the_element_list_parses_and_every_leaf_has_a_route` | D.3 and D.4 | D.3, D.4 |
| gpui showcase `every_listed_leaf_has_a_route` (`examples/showcase-gpui/tests.rs:15198`) | a `HOW` line per leaf an element lists | D.4 |
| gpui `every_theme_field_is_named` | the field list's grouping | D.1 |

## Part B — Presets

Implements B3. In unit 3 with Part A and D.2 (*Order and gates*).

### B.1 The lines

Each block goes into both `[light.input]` and `[dark.input]`; the dark block is
the light one, line for line (the blink is not per mode). Where a preset
states `caret_color`, the block follows it; elsewhere it follows the table's
last value line, before the blank lines and the next table.

`kde-breeze.toml`: after `disabled_text_color` (`:174` light, the matching
line of `[dark.input]`, `:748` on):

```toml
# platform-facts.md (§2.4): half Qt's cursor flash time, 1000 where CursorBlinkRate is unset (khintssettings.cpp:79-80 at plasma-integration 6.8.2; qwidgetlinecontrol.cpp:1497-1509 at qtbase 6.12.0)
caret_blink_on_ms = 500
# platform-facts.md (§2.4): the other half, hidden
caret_blink_off_ms = 500
# caret_blink_timeout_ms not stated: platform-facts.md (§2.4), Qt's caret never stops blinking (qwidgetlinecontrol.cpp:1497-1531 at qtbase 6.12.0)
```

`adwaita.toml`: after `caret_color` (`:149` light, `:586` dark):

```toml
# platform-facts.md (§2.4): half cursor-blink-time, 1200 by default (org.gnome.desktop.interface.gschema.xml.in:70-77 at gsettings-desktop-schemas 51.0; gtktext.c:6866-6873 at GTK 4.24.1)
caret_blink_on_ms = 600
# platform-facts.md (§2.4): the other half, hidden
caret_blink_off_ms = 600
# platform-facts.md (§2.4): cursor-blink-timeout, 10 s by default (org.gnome.desktop.interface.gschema.xml.in:78-85 at gsettings-desktop-schemas 51.0)
caret_blink_timeout_ms = 10000
```

`material-3.toml`: after `caret_color` (`:149` light, `:668` dark):

```toml
# platform-facts.md (§2.4): Compose shows the cursor 500 ms (CursorAnimationState.kt:76-83 at androidx e4d933c9)
caret_blink_on_ms = 500
# platform-facts.md (§2.4): and hides it 500 ms
caret_blink_off_ms = 500
# caret_blink_timeout_ms not stated: platform-facts.md (§2.4), the blink loops until the animation is cancelled (CursorAnimationState.kt:78-83)
```

`windows-11.toml`: after `disabled_background` (`:178` light, the matching
line of `[dark.input]`, `:802` on):

```toml
# platform-facts.md (§2.4): GetCaretBlinkTime 530 on the WinUI host's Windows 11 image; WinUI shows the caret one blink time (TextBoxHelpers.cpp:55-89 at microsoft-ui-xaml 2.5.1)
caret_blink_on_ms = 530
# platform-facts.md (§2.4): and hides it one blink time
caret_blink_off_ms = 530
# caret_blink_timeout_ms not stated: platform-facts.md (§2.4), WinUI's is a rule over the blink time, not one value (TextBoxView.cpp:2742-2763 at microsoft-ui-xaml 2.5.1)
```

`ios-13.toml`: after `placeholder_color` (`:164` light, the matching line of
`[dark.input]`, `:497` on), in the form of its tooltip comment (`:190-191`):

```toml
# measured on iOS 18.6, which no platform-facts column covers: the caret's blink animation shows it, then fades it out,
# for 0.65 s of each 1 s cycle (docs/evidence/ios-host/2026-11-06-run-42275028573/ios-host-macos-15/behavior_values.toml:11)
caret_blink_on_ms = 650
# hidden, then fading in, for the other 0.35 s (behavior_values.toml:12)
caret_blink_off_ms = 350
# caret_blink_timeout_ms unset: the blink repeats without end and still blinks after 12 s of samples (behavior_values.toml:18-20)
```

`macos-monterey.toml`, `macos-tahoe.toml` and the ten colour-scheme presets:
unchanged (B3); the macOS presets carry no comment for an unset tooltip delay
either.

Every comment takes the batch's one form, today's with the line number
removed (S6): `# platform-facts.md (§2.4): …` above a stated value, and
`# K not stated: platform-facts.md (§2.4), …` for a key the platform does
not give. `check_preset_citations.py` reads them as citations and compares
none (v0.7.1 Part I, I.3); the ios-13 comments contain no
`platform-facts.md`.

Lines added: kde-breeze 5 per variant, adwaita 6, material-3 5, windows-11
5, ios-13 6.

### B.2 The presets' gate

`native-theme/src/presets/documented_sizes.rs`:

- `soft_field_failures` (`:1295-1342`) gains a parameter after `want`,
  `elsewhere: &[(&str, Option<T>)]`: presets of `UNSOURCED_PRESETS` that
  state the field from a source outside platform-facts, each with the value
  it must state. In the loop over `UNSOURCED_PRESETS` (`:1327-1340`) a preset
  in `elsewhere` must state exactly its value (failure text
  `"{name} {mode:?}: states {field} = {got:?}, its source gives {expected:?}"`);
  every other one must state none, as now. An `elsewhere` entry that names
  no preset of `UNSOURCED_PRESETS` is a failure
  (`"{name}: not an unsourced preset, so it has no source outside platform-facts"`),
  not ignored. Its doc says so. The two callers (`:1362`, `:1390`) pass
  `&[]`.
- Three constants after `TOOLTIP_SHOW_DELAY` (`:1344-1354`), each with the doc
  form of that one, naming its row without a line (`platform-facts.md` §2.4
  `caret_blink_on`, `caret_blink_off`, `caret_blink_timeout`; S6):
  `CARET_BLINK_ON: [(NativePreset, Option<u32>); 6]` = KdeBreeze 500, Adwaita
  600, MacosMonterey None, MacosTahoe None, Windows11 530, Material3 500;
  `CARET_BLINK_OFF` the same values; `CARET_BLINK_TIMEOUT` = KdeBreeze None,
  Adwaita 10000, the other four None.
- A test `native_themes_state_the_caret_blink`: asserts
  `facts_row("2.4", "caret_blink_on").is_some()`, and the same for
  `caret_blink_off` and `caret_blink_timeout` (v0.7.1's Part I.2 gives the
  file `facts_row` in place of `facts_line(N)` and `facts_heading(N)`, S6);
  then runs `soft_field_failures`, which after v0.7.1's Part I.2 takes a row
  name where it took `line: usize` (its two existing callers pass
  `"§2.7 show_delay"` and `"§2.22 entrance"`), three times:
  `"input.caret_blink_on_ms"` with the row `"§2.4 caret_blink_on"` and
  `&[("ios-13", Some(650))]`, `"input.caret_blink_off_ms"` with
  `"§2.4 caret_blink_off"` and `&[("ios-13", Some(350))]`,
  `"input.caret_blink_timeout_ms"` with `"§2.4 caret_blink_timeout"` and
  `&[]`, reading `|t| t.input.caret_blink_on`
  and `|v| v.input.caret_blink_on` and so on, and asserts no failure. Its
  doc: the native themes state the blink as platform-facts gives it, static
  and live; ios-13 states its measured blink; no other preset states one.

### B.3 Documents of the presets

`native-theme/src/presets/README.md`, after the paragraph at `:44-47`:

```
The caret's blink, `input.caret_blink_on_ms`, `caret_blink_off_ms` and
`caret_blink_timeout_ms`, is gated so too (§2.4), but for `ios-13`, which
no platform-facts column covers and which states the blink measured on iOS
18.6 (`docs/evidence/ios-host/`); the macOS presets leave it unset, since no
source gives macOS's blink.
```

`docs/new-os-version-guide.md`, a last bullet of each platform's *What to
look for* list: KDE's (ending `:86`), Windows' (ending `:128`), macOS's
(ending `:182`) and GNOME's (ending `:228`), in that order:

```
- The caret's blink: plasma-integration's `CursorBlinkRate` default (1000)
  and its bound (200..2000), and Qt's halving of the flash time (500:
  `input.caret_blink_on_ms`, `caret_blink_off_ms`): `docs/platform-facts.md`
  §2.4.

- `GetCaretBlinkTime` on the runner image (530 on the WinUI host's:
  `input.caret_blink_on_ms`, `caret_blink_off_ms`), and whether WinUI's text
  box still stops blinking after 5 s (`caret_blink_timeout_ms` stays unset
  while that is a rule over the blink time): `docs/platform-facts.md` §2.4.

- A measured caret blink: no source gives macOS's yet, so
  `input.caret_blink_*_ms` stay unset: `docs/platform-facts.md` §2.4.

- The defaults of `cursor-blink-time` (1200) and `cursor-blink-timeout`
  (10 s) in gsettings-desktop-schemas (600: `input.caret_blink_on_ms`,
  `caret_blink_off_ms`; 10000: `caret_blink_timeout_ms`):
  `docs/platform-facts.md` §2.4.
```

## Part C — Readers

Implements B4. Each reader writes the variant of the mode the desktop is in,
as for every value it reads (B1); the pipeline is not changed: it merges the
reader's sparse variant over the platform preset field by field
(`native-theme/src/pipeline.rs:63-75`) and validates. Every reader merges
over its own platform's preset (kde-breeze, adwaita, windows-11;
`native-theme/src/pipeline.rs:603-626`, and `from_system_with_preset`
accepts only a generation of the running platform,
`native-theme/src/lib.rs:994-996`), and each of those states both durations,
so a reader that writes a time-out or `caret_blink_off` alone still gives a
whole blink.

### C.1 KDE

`native-theme/src/kde/mod.rs`:

- `# Sources` (`:9`): `//! - `kdeglobals`: the colours, fonts, icon theme and accessibility flags.`
  becomes `//! - `kdeglobals`: the colours, fonts, icon theme, accessibility flags and caret blink rate.`
  (one line; the first code span, which
  `watch::tests::every_listed_source_has_a_table_row` reads, is unchanged).
- `read_kde_ini` (`:62-91`): after the icon theme (`:78`), one call,
  `caret_blink(ini, &mut variant.input);`.
- A new private `fn caret_blink(ini: &configparser::ini::Ini, input: &mut
  crate::model::InputTheme)` after `kdeglobals_reduce_motion` (`:262-267`).
  It reads `ini.get("KDE", "CursorBlinkRate")`, trims it and parses it as
  `i32` (Qt reads the value with `toInt()`); text that is not an `i32`
  leaves both fields unset. A value above 0 is clamped to 200..=2000
  (`clamp(200, 2000)`), turned into a `u32` (`u32::try_from(…).ok()`) and
  halved with `checked_div(2)` (decision 5: no `/`, which the strict-panic
  run denies), and the half (100 to 1000) becomes both `caret_blink_on`
  and `caret_blink_off`; 0 or less sets `caret_blink_off` to `Some(0)`
  alone. It never writes `caret_blink_timeout`. Rustdoc: the
  rule with its upstream sources, written into the rustdoc as they stand
  here (plasma-integration 6.8.2
  `qt6/src/platformtheme/khintssettings.cpp:79-80`; qtbase 6.12.0
  `src/widgets/widgets/qwidgetlinecontrol.cpp:1497-1509` for the halving),
  and that an unset key leaves the preset's 500 / 500, which is Qt's at the
  unset default of 1000. The parked facts' quote of the plasma-integration
  line (`F/kde/tokens.toml:1606-1607`) is this specification's source only
  and is not written into the rustdoc.
- The KDE watcher watches `kdeglobals` already (`native-theme/src/watch/kde.rs:12`):
  no change.

Tests, in `kde/mod.rs`'s `tests`, through `from_kde_content` and its
`reader_mode` (`:672`):

1. `the_blink_rate_is_halved_within_qts_bound`: `[KDE]` with
   `CursorBlinkRate=` `1000` → on 500, off 500; `1001` → 500, 500; `100` →
   100, 100; `5000` → 1000, 1000; `0` → on None, off Some(0); `-1` → on None,
   off Some(0); `fast` → both None; no key → both None; the time-out None in
   every case.
2. `a_kde_blink_rate_of_0_resolves_to_no_blink`: content `[KDE]\nCursorBlinkRate=0\n`
   through `crate::pipeline::run_pipeline(result, "kde-breeze")?` (the call
   `text_scale_reaches_the_font_size_once` makes, `gnome/mod.rs`): the light
   variant's `input.caret_blink_off == Some(0)` and `caret_blink_on ==
   Some(500)`, the preset's (the fixture states no colour scheme, so the
   reader's variant is light).

### C.2 GNOME and Budgie

`native-theme/src/gnome/mod.rs` (Budgie shares the reader,
`native-theme/src/pipeline.rs:1021`):

- Module doc, first line: `//! GNOME portal reader: reads accent color, color scheme, contrast,`
  / `//! fonts, text scale, accessibility flags, and icon_set from the` becomes
  `… fonts, text scale, accessibility flags, the caret's blink and icon_set from the`
  re-wrapped within the same lines where it fits.
- `# Sources`, after the `overlay-scrolling` item (`:28`):

```
//! - `org.gnome.desktop.interface cursor-blink`: whether the caret blinks.
//! - `org.gnome.desktop.interface cursor-blink-time`: the caret's blink cycle.
//! - `org.gnome.desktop.interface cursor-blink-timeout`: when the caret stops
//!   blinking.
```

- `GnomePortalData` (`:401-431`) gains, after `overlay_scrolling`:
  `cursor_blink: Option<bool>` (doc: `cursor-blink`; None = not provided),
  `cursor_blink_time: Option<u32>` (doc: `cursor-blink-time` in
  milliseconds, within the schema's 100..=2500; None = not provided or
  outside it), `cursor_blink_timeout: Option<u32>` (doc:
  `cursor-blink-timeout` in seconds, within 1..=2147483647; None = not
  provided or outside it).
- `build_theme` (`:611-645`): three more reads, as the reader reads its
  other keys of that schema, through the same `gsetting`, inside the same
  deadline (`GSETTINGS_DEADLINE`, `:376`): `cursor_blink:
  flag(gsetting("org.gnome.desktop.interface", "cursor-blink"))`;
  `cursor_blink_time`: the text parsed as `u32`, kept only within
  100..=2500; `cursor_blink_timeout`: parsed as `u32`, kept only within
  1..=2147483647 (decision 6). The output form read on 2026-11-14 is a bare
  integer and `true` / `false` (*Corrections*); `read_gsetting` strips
  single quotes (`:378-394`), which integers and booleans do not carry.
- `build_gnome_variant_pure` (`:503-563`), after the `overlay-scrolling`
  block: a time sets `variant.input.caret_blink_on` and `caret_blink_off`
  to `time.checked_div(2)` (decision 5); `cursor_blink == Some(false)` then
  sets `caret_blink_off` to `Some(0)`, whatever the time (B4); a time-out
  sets `caret_blink_timeout` to `seconds.checked_mul(1000)`, or `u32::MAX`
  where that overflows (`.unwrap_or(u32::MAX)`; B2: about 49.7 days, the
  same to a user). A comment cites the schema, gsettings-desktop-schemas
  51.0 `schemas/org.gnome.desktop.interface.gschema.xml.in:63-85` (the
  three keys, read 2026-11-14: `cursor-blink`, type `b`, default `true`;
  `cursor-blink-time`, type `i`, 100..2500, default 1200, "Length of the
  cursor blink cycle, in milliseconds."; `cursor-blink-timeout`, type `i`,
  1..2147483647, default 10, "Time after which the cursor stops blinking,
  in seconds."), and GTK 4.24.1's use of them: the properties
  `gtk-cursor-blink`, `gtk-cursor-blink-time` and `gtk-cursor-blink-timeout`
  (`gtk/gtksettings.c`), and `gtk/gtktext.c`'s `blink_cb`, which past the
  time-out shows the caret and stops the blink, so the model's time-out (B2)
  is GTK's. The comment describes `blink_cb` in these words and quotes no
  text of `gtktext.c` (an LGPL source: no code from it, the maintainer's
  D10, as the plan's licence rule says); the code this
  specification read for it (`priv->cursor_alpha = 1.0;
  remove_blink_timeout (self);`) is not written into it. The paragraph that
  follows is this specification's reasoning, not comment text:
  GTK 4.24.1's `blink_cb` declares `int blink_timeout;`, sets it from
  `get_cursor_blink_timeout`, which returns `int`, and tests
  `if (now > priv->blink_start_time + blink_timeout * 1000000)` (read
  2026-11-14): the multiplication is `int` by `int`, and only the addition
  to the `guint64` `blink_start_time` is 64-bit, so the product overflows a
  32-bit `int` for a time-out above 2147 s, a signed overflow C leaves
  undefined. The reader is not affected: it reads the setting, not GTK's
  product, and its checked multiplication in `u32` gives the setting's
  milliseconds up to 4294967 s and `u32::MAX` above. Above 2147 s the model
  then states the setting, which GTK's own caret need not follow.
- `default_gnome_data` (`:1362-1377`), the tests' other literal of
  `GnomePortalData`, sets the three to `None`.

`native-theme/src/watch/gnome.rs`, `WATCHED` (`:16-30`), after
`("org.gnome.desktop.interface", "overlay-scrolling")`:
`("org.gnome.desktop.interface", "cursor-blink")`,
`("org.gnome.desktop.interface", "cursor-blink-time")`,
`("org.gnome.desktop.interface", "cursor-blink-timeout")`.

`native-theme/src/watch/mod.rs`, `SOURCE_NOTIFICATIONS` (`:568-678`), after
the `overlay-scrolling` row (`:623-627`): three rows of the form
`("gnome", "org.gnome.desktop.interface cursor-blink", "org.gnome.desktop.interface cursor-blink")`,
one per key, each formatted as the neighbouring rows are. The three tests
that hold these together then require exactly this:
`every_listed_source_has_a_table_row` (`:717`, the doc list against the
table), `the_gnome_watcher_observes_every_gnome_source` (`:759`, each row
in `WATCHED`) and `the_gnome_reader_lists_every_gsetting_it_reads`
(`:772`, each key `build_theme` reads in the doc list).

Tests, in `gnome/mod.rs`'s `tests`:

1. `one_gsettings_read_per_key` (`:1322-1345`): each count gains 3 (at
   `0fa28027` `(None, 9), (Some(ColorScheme::PreferDark), 8)` become `12`
   and `11`; a release before this one may have changed the base, the
   header's rule).
2. `the_blink_time_is_halved_into_both_durations`: `build_theme` with a
   `gsetting` that answers `cursor-blink-time` with `"1000"`: on 500, off
   500; with `"1201"`: 600, 600.
3. `cursor_blink_off_states_no_blink`: `cursor-blink` `"false"` with time
   `"1000"`: on 500, off 0; `"false"` alone: on None, off 0.
4. `the_time_out_is_read_in_milliseconds`: `"10"` → 10000; `"4294967"` →
   4294967000; `"4294968"` → `u32::MAX`; `"2147483647"` → `u32::MAX`.
5. `a_malformed_blink_setting_keeps_the_presets`: time `"99"`, `"2501"`,
   `"fast"`; time-out `"0"`, `"-5"`; `cursor-blink` `"maybe"`: the field it
   feeds stays None.
6. `the_gnome_blink_resolves_over_adwaita`: time `"1000"` through
   `crate::pipeline::run_pipeline(result, "adwaita")?`: the active variant's
   `input` is 500 / 500 / 10000 (the time-out the preset's).

### C.3 Windows

`native-theme/src/windows.rs`. The call that reads the blink time is
`GetCaretBlinkTime` (open point 2 gives what Microsoft's pages say and why
the rule of B4 selects it); it is `unsafe` and waits for the maintainer's
approval; without the approval nothing of C.3 is done, and C.5 lists what
the other parts then drop or word differently.
(`UISettings::CaretBlinkRate`, the safe call weighed, is not used: open
point 2.)

- `native-theme/Cargo.toml`'s comment above the Windows dependency (`:30`),
  "# The Windows reader: UISettings, the system metrics and fonts.", which
  describes the reader as well, becomes "# The Windows reader: UISettings,
  the system metrics, the fonts and the caret's blink time." in the same
  commit (unit 4; a manifest change, so unit 4 is structural, rule 24).
- Module doc, first paragraph (`:1-10`): "reads light or dark, the accent
  color and its shades, …" gains "the caret's blink time" in its list.
  `# Sources not watched` (`:31-48`): its intro (`:33-38`), "Each is
  reported by a message Windows sends to top-level windows, which the
  watcher, without a window, does not receive: …", does not hold for the
  blink time, for which no such message was read; the intro becomes "Each
  but the last is reported by a message Windows sends to top-level windows,
  …" (the rest unchanged), and a last item follows the `GetSysColor` item:
  `//! - `GetCaretBlinkTime`: the caret's blink time. No message or event` /
  `//!   that reports its change was read, and the watcher hears none, so a` /
  `//!   change shows at the next read.`
  (outside `# Sources`, so the watcher's table test does not read it).
- The read: a new `fn read_caret_blink_time() -> u32` under
  `#[cfg(all(target_os = "windows", feature = "windows"))]` and
  `#[allow(unsafe_code)]`, calling `GetCaretBlinkTime` (windows 0.62.2
  `src/Windows/Win32/UI/WindowsAndMessaging/mod.rs:722`, `pub unsafe fn
  GetCaretBlinkTime() -> u32`, feature `Win32_UI_WindowsAndMessaging`
  already on, `native-theme/Cargo.toml:141`) in one `unsafe` block under a
  `// SAFETY:` comment: it takes no argument and Microsoft's page states no
  precondition (the page linked), in the form of `read_all_system_fonts`
  (`:182-200`). The crate denies `unsafe_code` (`native-theme/src/lib.rs:175`);
  the exception per function is the file's form (`:189`, `:237`, `:280`,
  `:447`, `:491`). `read_windows` (`:673-709`) keeps its call of
  `build_theme` (`:700`), binds the result, and returns it after
  `apply_caret_blink_to_reader(&mut result, read_caret_blink_time())`.
- `build_theme` (`:525-643`) is not changed (decision 12). A new pure `fn
  apply_caret_blink_to_reader(result: &mut crate::ReaderResult, time: u32)`
  applies the mapping below to the variant `build_theme` filled: the
  `ReaderOutput::Single`'s `mode`, the only form `build_theme` returns
  (`:630-633`); the match is exhaustive, so its `Dual` arm applies it to both
  variants, as the blink is one setting for the desktop (B1). A new pure `fn
  apply_caret_blink_time(input: &mut crate::model::InputTheme, time: u32)`,
  compiled on every target, maps it: `CARET_BLINK_FAILED` (0) leaves both
  fields unset; `CARET_BLINK_NONE` sets `caret_blink_off` to `Some(0)`; any
  other value sets `caret_blink_on` and `caret_blink_off` to it (WinUI shows
  and hides the caret one blink time each, `F/windows/tokens.toml:2693-2695`).
  The two are local constants of `windows.rs`: `CARET_BLINK_FAILED: u32 = 0`
  and `CARET_BLINK_NONE: u32 = 4_294_967_295`, the value of Win32's
  `INFINITE` (windows 0.62.2 `src/Windows/Win32/System/Threading/mod.rs:2145`,
  `pub const INFINITE: u32 = 4294967295u32;`), each with Microsoft's page
  for its meaning (open point 2). They cannot name the crate's constant:
  the `windows` crate is a dependency on Windows only
  (`native-theme/Cargo.toml:138-150`), and the module is compiled on every
  target (`native-theme/src/lib.rs:339-344`). Both pure functions,
  `apply_caret_blink_time` and `apply_caret_blink_to_reader`, are compiled
  on every target, without a `cfg`, as `build_theme` is, so their tests
  run on every platform; off Windows, and on Windows without the `windows`
  feature, the module's `allow(dead_code)` (`native-theme/src/lib.rs:339-344`)
  keeps them warning-free. Rustdoc of `apply_caret_blink_time` and of
  `apply_caret_blink_to_reader`: the mapping, its upstream source as A.8's
  Windows cell links it (microsoft-ui-xaml 2.5.1
  `src/dxaml/xcp/core/text/TextBox/TextBoxHelpers.cpp:55-89`; the parked
  facts' `F` line is not written into the rustdoc), and that the watcher is
  not told of a change; of `read_caret_blink_time`: the call,
  its `SAFETY`, and the two special values it may return. Nothing of this
  needs arithmetic, and no function has more than two parameters.
- Placement, by the item each follows: the two constants, then
  `apply_caret_blink_time`, then `apply_caret_blink_to_reader`, right after
  `build_theme` (which ends at `:643`, before `WindowsReader`'s doc); and
  `read_caret_blink_time` right after `read_windows` (which ends at `:709`,
  before the test module).

Tests, in `windows.rs`'s `tests` (they run on every platform: the module is
compiled outside Windows too, `native-theme/src/lib.rs:339-344`); the 14
test calls of `build_theme` are not touched:
`a_blink_time_states_both_durations` (`apply_caret_blink_time` of 530 →
530 / 530, time-out None); `the_no_blink_value_states_no_blink`
(`CARET_BLINK_NONE` → off 0, on None); `a_failed_read_keeps_the_presets`
(`CARET_BLINK_FAILED` → both None); `the_blink_reaches_the_active_variant_only`
(`dark_reader()` (`:785-796`) through `apply_caret_blink_to_reader` with 530,
then `to_theme` as `dark_theme()` does: the dark variant's `input` is 530 /
530 and the light variant is `None`, not built; and `light_reader()`
(`:767-778`) the same way: the light variant's `input` is 530 / 530 and the
dark variant is `None`).

A Windows-only test, so that a `read_windows` that skips the helper fails:
`the_windows_reader_maps_the_systems_blink_time`, under
`#[cfg(all(target_os = "windows", feature = "windows"))]`, reads
`read_caret_blink_time()`, then `read_windows()`, and asserts that the
active variant's three fields are what `apply_caret_blink_time` makes of
the value just read, whatever it is: a failure (0) leaves both durations
unset, `INFINITE` gives off 0 and on unset, any other value both durations
equal to it; the time-out is unset. It does not depend on the runner's
setting or on the call succeeding. It does depend on `read_windows()`
returning `Ok` on CI's Windows image, `windows-latest`
(`.github/workflows/ci.yml:221-228`), which no run has shown: the
measurement of 530 was on `windows-11-vs2026-arm64` (Part 0's `run.toml`),
and no existing test calls `read_windows()`. That is first seen in CI
(*Not verified*), and since a red CI run emails the maintainer, the notice
of the branch's one push (*Order and gates*) says that its CI run is this
test's first.

### C.4 macOS, the pipeline

macOS: nothing is read (B4, §1.1); `native-theme/src/macos.rs` is not edited.
The pipeline: not edited (B4's last item; C's introduction).

### C.5 Without the approval of the `unsafe` call

Where the maintainer does not approve `GetCaretBlinkTime` (open point 2),
Windows stays unread in this release and windows-11's 530 / 530 stands.
What changes elsewhere:

- C.3: not done; `windows.rs` is not edited (decision 12 and the
  Windows-only test fall with it); the Windows legs of *Order and gates*
  check unit 4 for nothing new on Windows.
- A.8 and B.1: unchanged. The Windows cells and the windows-11 lines state
  the platform, not the reader (the `⚙` mark says the user can change the
  value, which stays true).
- E.1, the CHANGELOG's reader entry: its last sentence becomes "The Windows
  reader does not read the blink yet: the call that reads the user's
  setting is `unsafe` and waits for approval (`docs/todo.md`, *Windows
  reader: the caret's blink time*), and windows-11's 530 / 530 stands."
- E.2: `native-theme/README.md:74` and `native-theme/src/lib.rs:108` keep
  "`UISettings`, the system metrics and fonts", and `native-theme/Cargo.toml`'s
  comment (`:30`) is not edited, so unit 4 changes no manifest.
- E.3: the ROADMAP section reads "… and the KDE and GNOME readers read the
  user's; the Windows reader does not yet."
- E.4: the dated sentence in *Windows watcher: the window-only sources* is
  not written; a new entry is, under `## Core API` after E.4's
  *windows-11: the caret's blink time-out*:

```
### Windows reader: the caret's blink time

- [ ] [v0.7.x] The Windows reader reads no caret blink. The call that reads
      the user's setting, `GetCaretBlinkTime` (Microsoft's page, read
      2026-11-14: the blink time in milliseconds, `INFINITE` for no blink,
      0 for a failure), is `unsafe`, and the one new `unsafe` call it needs
      in `native-theme/src/windows.rs` waits for the maintainer's approval
      (`docs/archive/todo_v0.7.x_caret-blink-spec.md` C.3 and open point
      2). The safe `UISettings.CaretBlinkRate` is not used: its page does
      not say it is the same setting. Until then windows-11 states the
      measured 530 / 530.
```

- E.6 and *Order and gates*: nothing else changes.

## Part D — Connectors and showcases

Implements B5. D.2 (egui) is in unit 3 with Parts A and B; D.1, D.3 and
D.4 are unit 5 (*Order and gates*).

### D.1 gpui

`connectors/native-theme-gpui/src/lib.rs`:

1. **`caret_blink`**, a new private function after `tooltip_show_delay`
   (`:952-957`): `fn caret_blink(resolved: &ResolvedTheme) ->
   Option<gpui_base::input::CursorBlink>`. `caret_blink_off` `None` → `None`
   (the theme states no blink); `Some(0)` → `Some(CursorBlink::steady())`;
   `Some(off)` with `caret_blink_on` `Some(on)` →
   `CursorBlink::blinking(Duration::from_micros(u64::from(on).saturating_add(u64::from(off)).saturating_mul(500)))`
   (decision 11: no `+` or `*`, which the strict-panic run denies on the
   gpui library), and where `caret_blink_timeout` is `Some(t)`,
   `.with_idle_timeout(Duration::from_millis(u64::from(t)))` (a widening
   conversion and a `const fn`, no arithmetic, so the strict-panic list does
   not apply; `GB/input/base/blink_cursor.rs:20-41`); `Some(off)` with `caret_blink_on`
   `None`, which validation rejects (A.4), → `None`. Rustdoc: the rule, and
   that gpui-base blinks in equal phases only (`:28-34`), so the half-sum
   keeps the platform's rate and loses the share of the cycle the caret is
   shown (B5 (b)).
2. **`to_theme`** (`:326-397`): after the `scrollbar_mode` block (`:364-370`),
   where `caret_blink(resolved)` is `Some`, `theme.cursor_blink` is set to
   it, with a comment citing B5; otherwise upstream's default stays
   (`GC/theme/mod.rs:797`).
3. **The write rule.** A new global after `TooltipDelayWrite` (`:1249-1260`),
   in its form: `#[derive(Default)] struct CursorBlinkWrite { last_written:
   Option<CursorBlink>, app_blink: CursorBlink }` with `impl Global`, its
   doc: the blink the connector last installed on the styled theme, and the
   application's own, which a theme without a blink brings back (rationale
   B5). A new `fn cursor_blink_to_install(theme: Option<CursorBlink>, cx:
   &mut App) -> CursorBlink`, after `install_tooltip_delay` (`:1263-1281`)
   and in its form: it reads the installed `GpuiTheme::global(cx).cursor_blink`;
   where that is not the one the connector last wrote, it becomes the
   application's; the result is the theme's blink, else the application's,
   and it is recorded as the last written. Its doc says it decides by value
   as `install_tooltip_delay` does, with the same consequence.
4. **`apply_inner`** (`:1293-1377`): after the `gpui_component::init` block
   (`:1326-1329`) and before `let installed = GpuiTheme::global_mut(cx);`
   (`:1333`), the blink to install: the variant handed for the styled
   theme's mode (`dark` where `is_dark`, else `light`) — `Some` from `apply`
   and `apply_system_theme`, `None` from `apply_accessibility` — gives
   `cursor_blink_to_install(caret_blink(variant), cx)`; `None` keeps the
   installed theme's `cursor_blink`. The line
   `theme.cursor_blink = installed.cursor_blink;` (`:1338`) assigns that
   value instead, and the comment above the block (`:1330-1332`) says that
   the four settings are the application's and the blink the theme's or the
   application's (decision 10).
5. **The observer** (`install_observer_once`, `:1548-1602`) is not changed:
   it never writes the blink (B5).

Doc text in `lib.rs`:

- `apply`'s step 3 (`:1092-1098`), present: "3. writes the styled theme,
  keeping the application's own `notification`, `list`, `sheet`, `motion`
  and `cursor_blink` settings of the installed one, as upstream's
  `Theme::change` does; then installs …". New: "3. writes the styled
  theme, keeping the application's own `notification`, `list`, `sheet` and
  `motion` settings of the installed one, as upstream's `Theme::change`
  does, and with the variant's caret blink as `cursor_blink`, or the
  application's where the variant states none (*The caret's blink* below);
  then installs …" (the rest unchanged).
- A paragraph after the *Reduced motion* paragraph (`:1147-1155`):

```
/// The caret's blink: where the variant states one (`input.caret_blink_on_ms`,
/// `caret_blink_off_ms`, `caret_blink_timeout_ms`), `cursor_blink` is
/// `CursorBlink::steady()` for a `caret_blink_off_ms` of 0, else
/// `CursorBlink::blinking` with each phase half of the two durations together
/// (gpui-base blinks in equal phases, so the rate stays the platform's and the
/// share of the cycle the caret is shown does not), with the time-out, where
/// stated, as its idle time-out. Where the variant states none, the
/// application's blink comes back: the one installed before the connector's
/// first write, or set by the application since (gpui-component's 500 ms,
/// without a time-out, where it set none). It is told apart by value, as the
/// tooltip delay is: an application blink equal to the one the connector last
/// wrote is taken for the connector's, and a later theme without a blink then
/// brings back the application's earlier one. Nothing else writes it: a mode
/// switch through `Theme::change` or `sync_system_appearance` keeps the blink
/// of the variant `apply` installed, and [`apply_accessibility`] leaves it.
```

- `apply_accessibility` (`:1198-1204`): a last sentence, "It leaves
  `cursor_blink` as it is installed."
- Crate docs, *Theme Field Coverage* (`:104-105`): "the styled `Theme`'s
  `focus_ring`, `scrollbar_mode` and `highlight_theme`," becomes "the styled
  `Theme`'s `focus_ring`, `scrollbar_mode`, `highlight_theme` and
  `cursor_blink`,". The `input` row (`:116`), present "| `input` | 3 of 13 +
  geometry | border, caret, selection_background; height, …", becomes "|
  `input` | 6 of 16 + geometry | border, caret, selection_background; the
  caret's blink (`caret_blink_on`, `caret_blink_off`, `caret_blink_timeout`)
  as `cursor_blink`, installed by [`apply`]; height, …" (the rest unchanged).

Tests in `lib.rs`: items 1, 3 and 4 in `mod tests` (`:1606`), items 2 and
5, the `#[gpui::test]`s, in `mod apply_tests` (`:2411`), where
`apply_keeps_the_applications_own_settings` and the tooltip tests are:

1. `every_theme_field_is_named` (`:1618-1673`): `"cursor_blink"` moves from
   the last group to the first ("Set by `to_theme` from the resolved native
   theme", where a theme states a blink), and the last group's comment
   (`:1650-1654`) loses "or caret blink": "native-theme models no
   notification placement, list highlight default, sheet margin or motion
   tokens (`ResolvedTheme` has no such field), so `to_theme` leaves
   upstream's default and `apply` keeps the installed theme's."
2. `apply_keeps_the_applications_own_settings` (`:3470-3507`): its doc
   becomes "M6-01: `notification`, `list`, `sheet` and `motion` are the
   application's settings, …"; the `steady` binding, its precondition, its
   assertion and its write go (four lines). (Nord states no blink, so the
   test would still pass with them; they leave because the blink is no
   longer an application setting there.)
3. `caret_blink_maps_each_case` (no gpui context): adwaita light →
   `blinking(600 ms).with_idle_timeout(10 s)`; kde-breeze → `blinking(500
   ms)`; ios-13 → `blinking(500 ms)` (650 + 350 halved); catppuccin-mocha →
   `None`; adwaita with `caret_blink_off = Some(0)` → `steady()`; on 501,
   off 500 → `blinking(Duration::from_micros(500_500))`; off 500 with on
   `None` → `None`.
4. `to_theme_sets_a_stated_blink_and_leaves_upstreams`: `to_theme` of
   adwaita has adwaita's blink; of catppuccin-mocha, `CursorBlink::default()`.
5. `#[gpui::test]`s, with `apply_preset` (`:3269-3276`) and presets loaded
   as `tooltip_delay_presets` does (`:3251-3263`), each asserting the styled
   theme's `cursor_blink` and gpui-base's (`gpui_base::Theme::global(cx)`,
   which `sync_base` fills, `GC/theme/mod.rs:463`):
   - `apply_installs_the_themes_caret_blink`: adwaita → its blink;
   - `a_theme_without_a_blink_brings_back_the_applications`: steady set by
     the application before the first `apply`; adwaita → adwaita's;
     catppuccin-latte → steady; and without an application value, latte
     after adwaita → `CursorBlink::default()`;
   - `a_blink_the_application_sets_between_applies_is_its_own`: adwaita;
     the application sets steady through `GpuiTheme::update`; latte →
     steady; adwaita → adwaita's; latte → steady;
   - `an_application_blink_equal_to_the_connectors_is_taken_for_the_connectors`:
     steady before the first `apply`; adwaita; the application sets exactly
     adwaita's blink; latte → steady (the application's earlier value: the
     limit the docs state);
   - `apply_accessibility_leaves_the_caret_blink`: adwaita; the application
     sets steady; `apply_accessibility` with a text scale of 1.5 → steady;
   - `a_mode_switch_keeps_the_applied_variants_blink`: kde-breeze dark and
     light, the light one with `input.caret_blink_off = Some(0)` (as
     `an_upstream_mode_switch_installs_that_variants_delay` edits its light
     variant, `:3384`); `apply` dark, then light → steady;
     `GpuiTheme::change(GpuiThemeMode::Dark, None, cx)` → still steady (the
     applied light variant's, not the dark one's 500 ms); `apply` dark →
     `blinking(500 ms)`;
   - `apply_system_theme_installs_the_os_modes_blink`: the `SystemTheme` of
     `super::tests::system_theme()` (`:2125-2131`, which skips off Linux
     without a reader feature, as its other callers do), its `light` set to
     kde-breeze's light variant with `input.caret_blink_off = Some(0)`
     (steady) and its `dark` to adwaita's dark variant (600 / 600, time-out
     10000), as `apply_system_theme_restores_the_three_after_a_switch` sets
     `sys.light`, `sys.dark` and `sys.mode` (`:2880-2892`). Both blinks
     differ from `CursorBlink::default()` (500 ms, no time-out), which
     kde-breeze's own 500 / 500 would equal, so neither case passes without
     the change. In the test's one app: with `sys.mode = ColorMode::Dark`,
     `apply_system_theme` installs `blinking(600 ms).with_idle_timeout(10 s)`
     (the dark variant's, not the light one's `steady()`); then with
     `sys.mode = ColorMode::Light`, `apply_system_theme` again installs
     `steady()` (the light variant's, a stated blink replacing the
     connector's own previous write).

`connectors/native-theme-gpui/README.md`:

- *How re-application works*, a paragraph after the tooltip paragraph
  (`:547-559`): the text of the `lib.rs` paragraph above, with
  "`apply` and `apply_system_theme` also install the theme's caret blink, …"
  as its opening and the example "iOS's 650 / 350 becomes 500 / 500" after
  "does not".
- *Overriding theme values*, first bullet (`:640-644`), present "every field
  of it but the application's `notification`, `list`, `sheet`, `motion`
  and `cursor_blink` settings;", becomes "every field of it but the
  application's `notification`, `list`, `sheet` and `motion` settings, and
  `cursor_blink` where the theme states no blink (`apply_accessibility`
  keeps it in any case);". Second bullet (`:645-649`), "`focus_ring` and
  `scrollbar_mode` survive it;" becomes "`focus_ring`, `scrollbar_mode` and
  `cursor_blink` survive it;" (gpui-component keeps the blink across
  `Theme::change`, its test `GC/theme/mod.rs:1047-1067`).
- *What gets mapped*, a bullet after *Tooltip delay* (`:1072-1076`):
  "- **Caret blink** — `input.caret_blink_on_ms`, `caret_blink_off_ms` and
  `caret_blink_timeout_ms`, installed by `apply` as gpui-component's
  `Theme::cursor_blink` (*How re-application works*): steady where
  `caret_blink_off_ms` is 0, else blinking with each phase half of the two
  durations together, since gpui-base blinks in equal phases, and the
  time-out as its idle time-out."

### D.2 egui

`connectors/native-theme-egui/src/style/base.rs`, `base_style`, after
`v.text_cursor.stroke.color = …` (`:217`): where `t.input.caret_blink_off`
is `Some(off)`, `v.text_cursor.blink = off != 0`, `v.text_cursor.off_duration`
= `off` in seconds, and, where `t.input.caret_blink_on` is `Some(on)`,
`v.text_cursor.on_duration` = `on` in seconds; where it is `None`, nothing
of `text_cursor` but the colour is written (B5). A comment cites B5 and that
egui has no time-out. egui paints the caret while the time in the cycle is
below `on_duration` (`EG/text_selection/visuals.rs:290-317`), which A.4's
lower bound keeps above 0.

Milliseconds to seconds is one expression, written the same in `base.rs` and
in the contract's `expected` (T10 compares `f32` exactly, `contract.rs:143-145`,
`:1109-1117`): `(f64::from(ms) / 1000.0) as f32`, the `u32` widened without
loss and divided once, then rounded once to `f32` (650 gives the `f32`
nearest 0.65). No lint of the strict-panic run applies: its list
(`scripts/check_strict_panic.sh:76-92`) has no cast lint and no float lint
(`clippy::arithmetic_side_effects` leaves float operators to
`clippy::float_arithmetic`, which is not on it, as the tooltip delay's
`f32::from(ms) / 1000.0` at `base.rs:318` shows, which passes that run on
the egui library, `scripts/check_release.sh:633-634`), and the egui crate
denies no cast lint (`connectors/native-theme-egui/src/lib.rs:158-164`; the
workspace sets no `[lints]`).

egui 0.37, which lands before this release (the header), changes nothing
here: in egui 0.37.0 (Cargo's registry) `TextCursorStyle` has the same five
fields (`src/style.rs:1061-1076`), its defaults are the same (`blink: true`,
`on_duration: 0.5`, `off_duration: 0.5`, `src/style.rs:1083-1085`), and `paint_text_cursor`
blinks the same way (`src/text_selection/visuals.rs:291` on). The line
numbers of `EG` above are 0.36.2's.

Tests in `base.rs`'s `tests`:

- `the_base_style_writes_its_rows_and_nothing_else` (`:462-612`): beside the
  caret colour's assertions (`:535-539`), `v.text_cursor.blink` is `true`
  and `on_duration` and `off_duration` are the preset's in seconds (adwaita
  0.6 / 0.6, kde-breeze 0.5 / 0.5).
- `caret_blink_is_the_themes_in_seconds_or_egui_own`, in the form of
  `tooltip_delay_is_the_themes_in_seconds_or_egui_own` (`:710-733`):
  ios-13 → `true`, 0.65, 0.35; catppuccin-mocha → egui's own three values;
  adwaita with `caret_blink_off = Some(0)` → `false`, `on_duration` 0.6,
  `off_duration` 0.0; adwaita with `caret_blink_off = None` and
  `caret_blink_on` kept → egui's own three values; no notes in any case.

`connectors/native-theme-egui/mapping.toml`:

- After `["input.caret_color"]` (`:1638-1640`):

```toml
["input.caret_blink_on"]
verdict = "derived"
sinks   = [{ path = "visuals.text_cursor.on_duration", when = "input.caret_blink_off is stated; a theme that states no blink leaves egui's own" }]

["input.caret_blink_off"]
verdict = "derived"
probe   = 0
sinks   = [
  { path = "visuals.text_cursor.blink", when = "a change between two stated values above 0 holds it at true" },
  { path = "visuals.text_cursor.off_duration" },
  { path = "visuals.text_cursor.on_duration", when = "unstated, it leaves the whole blink to egui, input.caret_blink_on's duration included" },
]

["input.caret_blink_timeout"]
verdict  = "unmappable"
sub_tag  = "egui-limited"
upstream = "egui: a time-out after which TextCursorStyle stops blinking, the caret shown"
```

  The `on_duration` sink on `caret_blink_off`'s row is needed by T2
  (`mapping_tests.rs:1328-1437`): splicing adwaita's `caret_blink_off` (600)
  with macOS's (unset) moves `on_duration` from 0.6 to egui's 0.5, and T2
  rejects a move no sink of the spliced row declares. `probe = 0` makes T2
  splice a 0 into kde-breeze light (`:1399-1414`), the one splice that moves
  the `blink` sink, since no bundled preset states 0.
- `[unwritten]` (`:2903` on): the rows `"visuals.text_cursor.blink"`,
  `"visuals.text_cursor.on_duration"` and `"visuals.text_cursor.off_duration"`
  (`:2971-2973`) go: a path there may not be a declared sink (`:429`).

`connectors/native-theme-egui/src/mapping_tests.rs`:

Counts below are deltas (the header's rule), with their value at
`0fa28027`:

- `every_leaf_has_exactly_one_row_and_every_row_is_well_formed`
  (`:234-438`): the leaf count gains 3 (`516` at `:239` becomes `519`), and
  its message's ResolvedTheme count likewise (`"512 ResolvedTheme leaves + 4
  layout leaves (§5.7)"` becomes `"515 …"`, `:240`): a `None` soft option
  is a leaf too (`:152-155`).
- `verdict_totals_match_the_specification` (`:443-497`): the inputs'
  DERIVED +2 and UNMAPPABLE +1 (`("inputs", [4, 35, 36, 12])` becomes
  `[4, 35, 38, 13]`); `egui-limited` +1 (`52` becomes `53`) in the
  assertion and in the doc (`:440`).

`connectors/native-theme-egui/src/contract.rs`: `read_style` (`:198` on)
gains `["visuals", "text_cursor", "blink"] => Val::Bool(…)`,
`["visuals", "text_cursor", "on_duration"]` and `"off_duration"` `=>
Val::F32(…)`, beside the caret colour's arm (`:241`); `expected`
(`:530` on) gains three formulas beside the tooltip delay's (`:991-995`),
in its form: `blink` is `Val::Bool(off != 0)` where `caret_blink_off` is
`Some(off)`; `off_duration` the seconds of a stated `caret_blink_off`;
`on_duration` the seconds of `caret_blink_on` where both are stated, each
by the expression above; each `return Ok(None)` otherwise. Without them T10 fails with "no formula": the
sinks are written by a formula, not by a row's conversion (`:521-529`).

`connectors/native-theme-egui/src/mapping.md` is regenerated, never edited:
`cargo test -p native-theme-egui --lib -- --ignored regenerate_mapping_md`
(`src/mapping_doc.rs:19`), and `mapping_md_is_current` holds it. Its totals
gain Inputs +3 leaves, DERIVED +2, UNMAPPABLE +1, `egui-limited` +1. At
`0fa28027` (`src/mapping.md:14`, `:17`, `:19`): Inputs 87 leaves, DERIVED
36, UNMAPPABLE 12; TOTAL 516, DERIVED 171, UNMAPPABLE 91; `egui-limited`
52. After this change, on that base: Inputs 90, 38, 13; TOTAL 519, 173,
92; `egui-limited` 53.

`connectors/native-theme-egui/src/tripwires.rs`: no change. `TextCursorStyle`
is already destructured field by field (`:216-223`), and T1b's three
defaults (`:252-261`) are not the blink's: the connector writes the blink
from the theme, never reasoning from egui's 0.5.

`connectors/native-theme-egui/README.md`, the table at `:51`: "| a text
field's fill and caret | …" becomes "| a text field's fill, its caret and the
caret's blink | `visuals.text_edit_bg_color`, `visuals.text_cursor` |"; and
the same list in prose in the crate docs,
`connectors/native-theme-egui/src/lib.rs:92`, "a text field's fill and
caret colour" becomes "a text field's fill, its caret's colour and the
caret's blink" (re-wrapped within its paragraph, `:86-96`).

### D.3 iced

`connectors/native-theme-iced/src/contract/derived.rs`, `UNREACHABLE`
(`:1076` on), three entries at its end, in the form of `dialog.entrance`'s
(`:1077-1115`). Evidence, the same for the three but its last clause:
"iced_widget 0.14.2 blinks the caret of its text input and text editor at a
private constant, 500 ms shown and 500 ms hidden (`src/text_input.rs:1693`,
read at `:515-518`; `src/text_editor.rs:544`), and neither `Style` has a
caret field" + "; iced cannot take the shown duration" / "; iced cannot take
the hidden duration, nor 0, a caret that does not blink" / "; it never stops
blinking, so it has no time-out to take". The `lost` functions:

- `input.caret_blink_on`: `caret_blink_on` where `caret_blink_off` is
  stated and above 0 and `caret_blink_on` is not 500, as `"{on}ms"`; else
  `None`.
- `input.caret_blink_off`: `caret_blink_off` where it is stated and not 500,
  as `"{off}ms"` (`"0ms: no blink"` for 0); else `None`.
- `input.caret_blink_timeout`: a stated `caret_blink_timeout`, as `"{t}ms"`.

`every_unreachable_value_is_one_a_preset_loses` (`contract.rs:1961-2025`)
requires the exceptions to be exactly the combinations whose `lost` is
`None`. With Part B's presets, over the 34 combinations
(`contract-leaves.toml:28`, the list; `:26` its comment):

- `caret_blink_on` and `caret_blink_off`: lost on `adwaita`, `windows-11`
  and `ios-13`, light and dark; exceptions: `kde-breeze/light` ("the preset
  states 500 ms, which iced draws"), `kde-breeze/dark`, `material-3/light`,
  `material-3/dark` ("the same"), `macos-monterey/light` ("the preset states
  no blink, a soft option, so there is none to lose"), `macos-monterey/dark`,
  `macos-tahoe/light`, `macos-tahoe/dark` and the twenty colour-scheme
  combinations ("the same") — 28.
- `caret_blink_timeout`: lost on `adwaita/light` and `adwaita/dark`; the 32
  others are exceptions ("the preset states no time-out, a soft option, so
  there is none to lose", then "the same").

`connectors/native-theme-iced/contract-leaves.toml`: unchanged. It records
the colour leaves only (its header, `:9-10`), and
`every_colour_leaf_reaches_a_row_or_is_named` holds it to that walk; the
three fields are not colours.

`connectors/native-theme-iced/src/lib.rs`, *What iced cannot take*
(`:347-385`), and `connectors/native-theme-iced/README.md`, its list
(`:647-680`), a bullet after `spinner.min_diameter`'s (decision 13):

```
- **The caret's blink.** iced draws the caret 500 ms shown and 500 ms
  hidden, a private constant of its text input and text editor, and never
  stops (`iced_widget` 0.14.2 `src/text_input.rs:1693`,
  `src/text_editor.rs:544`): `input.caret_blink_on`, `.caret_blink_off` and
  `.caret_blink_timeout` reach nothing.
```

(with `//! ` before each line in `lib.rs`).

### D.4 Showcases

Each states what its toolkit does with the three fields in the focused text
input's Widget Info, as its other leaf rows do (B5).

- `docs/showcase-elements.toml`, the element `basic.text_inputs.focused`
  (`:1203-1209`), the one that draws the caret: its `leaves` gain
  `"input.caret_blink_on_ms", "input.caret_blink_off_ms",
  "input.caret_blink_timeout_ms"` right after `"input.caret_color"`, the
  registry's order (A.6; the list's rule, `:28-35`). Same line.
- gpui, `connectors/native-theme-gpui/examples/showcase-gpui/info/leaves.rs`,
  `HOW`, after the `input.caret_color` entry (`:1064-1068`), three entries
  with the prefix `""`:
  - `input.caret_blink_on_ms`: `"gpui: Theme::cursor_blink, which apply installs (lib.rs, caret_blink): each phase half of caret_blink_on and caret_blink_off together, as gpui-base blinks in equal phases (gpui-base/input/base/blink_cursor.rs, CursorBlink::blinking); upstream's 500ms where the theme states no blink"`;
  - `input.caret_blink_off_ms`: `"gpui: Theme::cursor_blink: steady where it is 0 (CursorBlink::steady), else its half of the phase; upstream's 500ms where the theme states no blink"`;
  - `input.caret_blink_timeout_ms`: `"gpui: Theme::cursor_blink's idle time-out (CursorBlink::with_idle_timeout): the caret stops blinking, shown, that long after the last focus, edit or movement; it never stops where unstated"`.
- egui: nothing by hand. The showcase's line for a leaf is its
  `mapping.toml` row's route (`examples/showcase-egui/info.rs:524-545`,
  `mapping_key` strips `_ms`), so D.2's rows give it.
- iced, `connectors/native-theme-iced/examples/showcase-iced.rs`,
  `iced_route` (`:7749-8552`), an arm after the input's padding arm
  (`:7907-7909`): `("input", "caret_blink_on_ms" | "caret_blink_off_ms" |
  "caret_blink_timeout_ms") => "not reachable: iced blinks the caret 500ms
  shown and 500ms hidden, a private constant, and never stops
  (text_input.rs, CURSOR_BLINK_INTERVAL_MILLIS; text_editor.rs, Focus)".into()`.
  `the_element_list_parses_and_every_leaf_has_a_route` (`:24982-25000`)
  requires it.
- `tools/showcase-support/src/values.rs`, `number` (`:192-197`), decision 14:
  a JSON number that is an unsigned integer (`as_u64`) is printed as
  itself, any other as the `f32` it holds, as now. Its doc says so. A test
  beside `a_ms_leaf_reads_its_value` (`:353-375`),
  `a_u32_ms_leaf_prints_whole`: `value_at` of `input.caret_blink_timeout_ms`
  in a JSON whose `input.caret_blink_timeout` is `4294967295` reads
  `"4294967295 ms"`.
- `connectors/native-theme-egui/examples/showcase-egui/tests.rs`,
  `value_line` (`:7095-7106`), decision 14: `the_info_is_the_manifest`
  (`:7116`) compares the first line of each `mapping.toml` row Widget Info
  prints, through `values::value_at`, with it (`:7153-7166`). Its `Number`
  arm takes `number`'s rule:
  `serde_json::Value::Number(n) => n.as_u64().map_or_else(|| n.as_f64().map_or_else(|| n.to_string(), |n| format!("{}", n as f32)), |n| n.to_string()),`.
  Its doc's "a number as the `f32` it is prints" becomes "an unsigned
  integer as itself and any other number as the `f32` it is prints".

## Part E — Documentation, citations, close-out

### E.1 `CHANGELOG.md`

The entries go under a new heading `## [0.7.N] - <date>`, with the number
proposed for this release and the date filled in by the version bump (E.6),
never `## [Unreleased]` (rule 14, for every release of the batch). Its
section stands above every other `## [`
heading of the file, whichever releases land before it (the header); where the
merge (*Order and gates*, *The branch*) meets a conflict there, this release's
section is put first. No gate checks the order of the headings: `scripts/check_release.sh:1309` and
`:1344` look only for the version's dated heading. Sections in 0.7.0's
order (`CHANGELOG.md:34`, `:90`, `:213`).

`### Breaking Changes` (under B6 (a); the words of `CHANGELOG.md:66-68`):

```
- **native-theme**: `InputTheme` gains three fields, `caret_blink_on`, `caret_blink_off` and `caret_blink_timeout` (see *Added*). `InputTheme` is not `#[non_exhaustive]`, so a struct literal of it outside the crate needs the new fields or `..Default::default()`; `ResolvedInputTheme` is `#[non_exhaustive]` already.
```

`### Added`:

```
- **native-theme**: `input.caret_blink_on_ms`, `input.caret_blink_off_ms` and `input.caret_blink_timeout_ms` (`InputTheme::caret_blink_on`, `caret_blink_off` and `caret_blink_timeout`, each an `Option<u32>`), how the text caret blinks: how long it is shown and how long hidden in each cycle, `caret_blink_off_ms = 0` for a caret that does not blink, and how long after the last input the blinking stops, the caret shown. A theme states both durations, or `caret_blink_off_ms = 0`, or none; one duration alone, or a time-out without a blink, fails to resolve with `Error::ResolutionIncomplete` naming the key to add, and a shown duration or a time-out of 0 is out of range. Each value from its `docs/platform-facts.md` §2.4 row: `kde-breeze` 500 / 500 (half Qt's cursor flash time), `adwaita` 600 / 600 and a time-out of 10000 (GTK's `cursor-blink-time` and `cursor-blink-timeout`), `windows-11` 530 / 530 (`GetCaretBlinkTime` on the WinUI host's runner image), `material-3` 500 / 500 (Compose); `ios-13` 650 / 350, measured on iOS 18.6 (`docs/evidence/ios-host/`). The macOS presets leave the blink unset, since no source gives it, and so do the colour-scheme presets.
- **native-theme**: the KDE reader reads kdeglobals' `[KDE] CursorBlinkRate` as plasma-integration hands it to Qt: bounded to 200..2000 and halved into the two durations, 0 or less no blink. The GNOME and Budgie reader reads `cursor-blink`, `cursor-blink-time` and `cursor-blink-timeout`, and the watcher fires on a change of each. The Windows reader reads the caret's blink time (`GetCaretBlinkTime`); the watcher is not told of a change, which shows at the next read.
- **native-theme-iced**: the crate documentation and the README list the caret's blink among what iced cannot take: iced blinks its caret 500 ms shown and 500 ms hidden, a private constant, and never stops, so `input.caret_blink_on_ms`, `caret_blink_off_ms` and `caret_blink_timeout_ms` reach nothing.
- **Showcases**: the focused text input's Widget Info lists the caret's blink in all three showcases, with its value and what the toolkit does with it: gpui installs it as `Theme::cursor_blink`, egui as `Visuals::text_cursor` (its time-out unmappable), and iced cannot take it.
```

`### Changed`:

```
- **native-theme-gpui**: `apply` and `apply_system_theme` install the theme's caret blink as gpui-component's `Theme::cursor_blink`, where they kept the application's always (since 0.7.1): steady where `caret_blink_off_ms` is 0, else blinking with each phase half of the two durations together (gpui-component blinks in equal phases), and the time-out as its idle time-out. Where the theme states no blink the application's own comes back, as for the tooltip delay, decided by value; `apply_accessibility` leaves the blink, and a mode switch through gpui-component keeps it.
- **native-theme-egui**: the base style writes the theme's blink into `Visuals::text_cursor` (`blink`, `on_duration`, `off_duration`) where the theme states one; egui has no time-out, so `input.caret_blink_timeout` is unmappable.
```

The iced entry is under *Added*, not *Changed*: iced's behaviour does not
change, the crate documents one more value it cannot take, as 0.7.0 listed
`dialog.entrance` (`CHANGELOG.md:134`).

### E.2 `native-theme/README.md`

- *Core concepts*, the *Behaviour* item (`:51`): "`expander.frame_enabled`
  and `tab.active_indicator_side`, where the toolkit's own value applies too"
  becomes "`expander.frame_enabled`, `tab.active_indicator_side` and the
  caret's blink (`input.caret_blink_on`, `caret_blink_off`,
  `caret_blink_timeout`), where the toolkit's own value applies too", the
  rest unchanged. A new item after *Switch outlines* (`:52`):

```
  - *Caret blink:* `input.caret_blink_on_ms` and `caret_blink_off_ms` state a blink together, and `caret_blink_off_ms = 0` alone states a caret that does not blink; `caret_blink_timeout_ms` beside them stops the blinking that long after the last input, the caret shown. A theme that states one duration alone, or a time-out without a blink, fails to resolve, `Error::ResolutionIncomplete` naming the key to add, so an overlay states both durations: it merges field by field over a preset that may state none. An overlay cannot unset a preset's time-out; to keep the caret blinking it states the largest, `caret_blink_timeout_ms = 4294967295` (about 49.7 days).
```

- *Platform readers*, the Windows row (`:74`): "`UISettings`, the system
  metrics and fonts" becomes "`UISettings`, the system metrics, the fonts
  and the caret's blink time"; the same row of the crate docs' copy of the
  table, `native-theme/src/lib.rs:108`, likewise (C.5: both stay without the
  approval). The same text in `native-theme/Cargo.toml`'s comment (`:30`)
  changes with C.3, in unit 4, not here.

### E.3 `ROADMAP.md`

Under B6 (a) as a release of its own, a section in the place the
maintainer's order of the releases gives it (the KDE document's D10,
*The releases after v0.7.1*, proposes it as item 4 of D10's list),
written at the release's close-out in the form of the delivered ones
(`ROADMAP.md:85-110`), heading with the version B6 sets:

```
## v0.7.N — the caret's blink

**Delivered:** the model states how the text caret blinks:
`input.caret_blink_on_ms` and `caret_blink_off_ms`, how long the caret is
shown and hidden (`caret_blink_off_ms = 0`: it does not blink), and
`caret_blink_timeout_ms`, how long after the last input it stops. kde-breeze,
adwaita, windows-11, material-3 and ios-13 state their platform's blink, and
the KDE, GNOME and Windows readers read the user's. The gpui connector
installs it as gpui-component's `cursor_blink`, the egui connector as egui's
`text_cursor`; iced has no receiver. macOS's blink, the caret's width and
the rest of what the platforms draw of the caret are in the backlog.
```

`ROADMAP.md:93-94` ("and `apply` keeps the application's `cursor_blink`,
which native-theme does not model") describes v0.7.1 and stays.

### E.4 `docs/todo.md`

New entries, the rationale's §5, under `## Core API`, after the entry
*Model: a hovered segment's text colour* (`:1545`). Text:

```
### Model: the caret beyond its blink

- [ ] [v0.7.x] What the 0.8.0 facts state of the text caret beside its
      blink, which the model has no field for (`docs/archive/todo_v0.7.1_caret-blink-rationale.md`
      §5; `docs/facts/<platform>/tokens.toml` on
      `v0.8.0-rc1` at `78ee0efd`). Its width: 1 px on KDE and Windows, 2 px
      on macOS, iOS and Material (key `token.caret.width`), and on GNOME no
      fixed width, the stem the line height × 0.04 + 1 (`docs/todo_v0.7.1_model-completeness-gap-map-gnome.md`,
      GN-3.36); only egui can take it (`Visuals::text_cursor.stroke.width`),
      while gpui-base draws 1 px (`CURSOR_WIDTH`, gpui-base 0.9.1
      `src/input/base/blink_cursor.rs:8`) and iced a 1 px quad. The blink's
      style, at once or fading (`token.caret.blink_style`: hard on KDE,
      Windows and Material, a fade on GNOME and iOS); whether a read-only
      field shows a caret (`token.caret.shown_in_read_only`: no on KDE,
      GNOME, Windows, iOS and Material); the secondary caret at a direction
      boundary (`token.caret.secondary`: none on KDE, GNOME, Windows and
      Material); and that the caret stays solid while the user types
      (`token.caret.solid_while_typing`: on KDE, GNOME, iOS and Material).
      No toolkit of the three takes these four. Each is a field and a step
      of its own, with its facts rows and presets, once a toolkit draws it
      or an application asks for it.

### Readers: what a reader reads for the whole desktop reaches one variant

- [ ] [v0.7.x] The KDE, GNOME and Windows readers fill the variant of the
      mode the desktop is in, and the pipeline takes the other from the
      preset (`native-theme/src/lib.rs`, `ReaderOutput::Single`: "The
      pipeline fills the inactive variant from the platform preset.";
      `native-theme/src/pipeline.rs`, "the merge keeps the preset's own
      there"). For what a reader reads for the
      desktop as a whole (the fonts, the scroll bars' overlay mode, the icon
      theme, the caret's blink), an application that switches between light
      and dark by itself and applies the other variant gets the preset's
      value there until the next read: a user who turned blinking off sees
      the caret blink in that mode. The gpui connector is spared it for the
      blink, which gpui-component keeps across its own mode switch. A rule
      for which reader values go into both variants is a design of its own:
      the icon theme and the accent's shades are meant to differ by mode
      (`docs/archive/todo_v0.7.1_caret-blink-rationale.md` B1).

### macOS: the caret's blink

- [ ] [v0.7.x] macOS's blink is not established: `NSTextInsertionIndicator.h`
      (Xcode 16.4) declares no blink period, and the blink's user defaults
      were unset on the test runner
      (`docs/facts/macos/tokens.toml:47501-47513` and `:46129-46141` on
      `v0.8.0-rc1` at `78ee0efd`, U605 and U316). So `macos-monterey` and `macos-tahoe`
      state no `input.caret_blink_*_ms`, and the macOS reader reads none.
      To measure (a run that records a focused text field's caret over
      time, as the iOS host's behaviour pass does,
      `docs/evidence/ios-host/`), then to read the user's setting where one
      exists.

### windows-11: the caret's blink time-out

- [ ] [v0.7.x] WinUI's text box stops blinking after the whole blink periods
      that fit in 5 s, three periods where none fits (`CheckCaretBlinkTimeout`,
      microsoft-ui-xaml 2.5.1 `src/dxaml/xcp/core/native/text/Controls/TextBoxView.cpp:2742-2763`;
      `docs/facts/windows/tokens.toml:2711-2718` on `v0.8.0-rc1` at `78ee0efd`, pending
      U505): a rule over the blink time, not one value. The system's own
      caret time-out, `SPI_GETCARETTIMEOUT` (windows 0.62.2
      `src/Windows/Win32/UI/WindowsAndMessaging/mod.rs:6108`), is not read;
      the parked specification lists it to verify
      (`docs/todo_v0.7.1_model-completeness-spec-widgets.md:8909-8910` at
      `0fa28027`: "`SPI_GETCARETTIMEOUT` (to verify): W3-24"). So
      windows-11 states no `input.caret_blink_timeout_ms`. Read what the
      system time-out is and how WinUI's rule relates to it before a value
      is given.
```

And in the entry *Theme watcher: OS changes it does not report*, its item
**Windows watcher: the window-only sources** (`:260-300`), a sentence
appended at its end in the entry's dated form (decision 13): "*Caret blink
(<the release's date>):* the reader reads the caret's blink time
(`docs/archive/todo_v0.7.x_caret-blink-spec.md` C.3), a source the watcher
hears no change of either; which message Windows sends for it was not read."
(Only with the approval of the `unsafe` call; C.5 gives the entry written
instead.)

The entries cite this specification and the rationale at their archived
paths, since E.6 moves them in the same release. The tag lines' citations
are code and facts paths, which `scripts/check_todo_citations.py` does not
read; no preset and no changing file is cited by line (S6, E.5).

### E.5 Line citations

None is re-pointed or converted (S6, decided by the maintainer on
2026-11-15): the citations of lines the edits move, and those already off
at `0fa28027`, are left as they are and may drift. The citations this work
writes of a tracked file that changes name the document, with a quoted
phrase or a row's name where useful, never a line number (B.1's preset
comments, E.4's entries); those of pinned sources (an upstream crate at a
version, a URL at a commit, a citation "at <sha>", the parked
`docs/facts/` on `v0.8.0-rc1` at `78ee0efd`) keep their lines.


### E.6 Close-out

- Open the new facts cells' links (decision 4) and correct any that does
  not show the cited lines.
- The version bump B6 sets (the workspace version, the inter-crate
  requirements, `Cargo.lock`, and the date of E.1's `## [0.7.N] - <date>`
  heading, with its number corrected where B6 set another), as commit
  `a3c10475` did for 0.7.1.
- The release's compare link, which `scripts/check_release.sh:1417` names
  as a release step ("Date the CHANGELOG entry, set its compare link"): a
  line at the top of the CHANGELOG's link list, in the form its existing
  links have (`CHANGELOG.md:1802`, `[0.7.0]:
  https://github.com/tiborgats/native-theme/compare/v0.6.1...v0.7.0`):
  `[0.7.N]: https://github.com/tiborgats/native-theme/compare/v<the previous
  release>...v0.7.N`.
- Asset run (`scripts/generate_assets_release.sh`), after the bump: the
  provenance stamp hashes `Cargo.lock`, so a run before the bump is stale at
  once. Parts A to D edit `native-theme/src/`, the connectors' `src/` and
  `examples/` and `tools/showcase-support/`, which
  `docs/assets/PROVENANCE.toml` and `docs/COMPATIBILITY.toml` hash; both
  stamps fail the full check once the CHANGELOG entry is dated
  (`scripts/check_release.sh:1288-1300`, `:1325-1335`). It needs the
  maintainer's desktop unlocked and untouched, once, near the end.
- The full `scripts/check_release.sh`; the semver step reports the
  `InputTheme` break and asks for the version decision (B6).
- Archive: this specification, the rationale and the plan move to
  `docs/archive/` with their row in `docs/archive/README.md`
  (`scripts/check_release.sh:1033` checks that the index names every
  archived file, expanding a brace set). The two prefixes cannot share one
  brace set, so the row, in the form of `docs/archive/README.md:51` and
  after the newest release's row (the releases before this one add theirs
  first), is the one below (the plan's own name in place of
  `todo_v0.7.x_caret-blink-plan.md` if it is named otherwise). The links of
  0.2 are updated.

```
| v0.7.N | `todo_v0.7.1_caret-blink-rationale.md`, `todo_v0.7.x_caret-blink-{spec,plan}.md`: the text caret's blink |
```

## Order and gates

The parts are not units of work: each unit below passes its gate (the cadence
after the table), and Part A alone would fail any gate that runs the egui
connector's tests. The model's resolved struct has no
`skip_serializing_none` (`native-theme-derive/src/gen_structs.rs:23-29`), so
the three fields serialise as `null` even unstated; egui's `leaf_paths`
counts a `null` as a leaf (`connectors/native-theme-egui/src/mapping_tests.rs:152-186`),
and `every_leaf_has_exactly_one_row_and_every_row_is_well_formed` then fails
on "leaf with no row" and on its count (`:234-262`) until `mapping.toml`
has the rows. The rows' sinks need `base.rs` to write them (T2 requires the
`off_duration` sink to move, T10 holds every sink to its formula); T2 needs
a pair of presets whose values differ (kde-breeze's 500 against adwaita's
600), and D.2's `base.rs` tests and A.5's test 5 need the presets' values.
So the model, the presets and egui are one unit.

The other walks over every leaf were read for the three new leaves and are
not affected: the gpui contract's `every_colour_leaf_reaches_a_sink_or_is_named`
(`connectors/native-theme-gpui/src/contract.rs:2933-2950`) and iced's
`reach` (`connectors/native-theme-iced/src/contract.rs:5262-5270`) take
`colour_leaves` (`gpui :2696-2706`, iced `:4889-4899`), which keeps a
`null` only where a colour splices into it, which a `u32` does not;
egui-widgets' walk (`connectors/native-theme-egui-widgets/src/tests.rs:3242-3265`)
skips a `null` whose colour splice fails; egui's `roles_are_the_widget_fields_of_resolved_theme`
(`connectors/native-theme-egui/src/roles.rs:418-432`) reads the top-level
keys; egui's T4 (`mapping_tests.rs:672-698`) states only `null`s a float
deserialises into; the iced showcase's walks (`showcase-iced.rs:25057`,
`:28626`, `:29102`) read the colour export; `tools/showcase-support`
reads only the leaves `docs/showcase-elements.toml` lists; the core has no
count of leaves.

| Unit | Content | Gate, and what it must cover (`scripts/check_release.sh` line) |
|---|---|---|
| 0 — the documents | the approved documents of this work that no earlier commit added: the rationale, this specification and the plan too, each where v0.7.1's commit (0) did not add it (decision 17); no unit 0 where none is left | the five data-document checks and `--quick`, or `--commit` where it is the warm-up gate; no gate reads them but the archive index, which does not list them until E.6 |
| 2 | Part 0 | blob ids (0.1); the reproduction check (0.1); the five data-document checks and `--quick` |
| 3 | Part A (derive, fields, `check_caret_blink`, the `error.rs` and `validate.rs` docs, registry, rules, facts rows, A.5's tests), Part B (presets, B.2's gate, B.3) and D.2 (egui: `base.rs`, `mapping.toml`, `mapping.md` regenerated, `mapping_tests.rs`'s counts, `contract.rs`, the README row) | `--quick` or `--commit` as the count gives it (rule 6; not structural by rule 24, since it changes no manifest, lock, build script, toolchain or gate script; the plan's table); either must cover `cargo check` per crate (`:427`), clippy (`:529-549`), the tests (the loop `:665-682`, `:684`; egui `:760-762`), strict panic (`:572-575`, egui `:633-634`), the five data-document checks (`:965-983`), docs (`:1000-1021`) |
| 4 | Part C (Windows only with open point 2's approval, C.5 otherwise) | `--quick` or `--commit` as the count gives it (rule 6), and `--commit` where C.3 is done, since its edit of `native-theme/Cargo.toml`'s comment is a manifest change (rule 24); as to the cross-target legs: `--quick` runs every step of `--commit`, the cross-target legs among them (`docs/todo_v0.7.1_gate-speed-spec.md`, decision 1 (c); rule 24), and the summary must name no skipped cross-target check; it must cover the native-theme tests with `$NT_FEATURES` (the readers', the watcher's three), `check_features.sh` (`:443`), strict panic (`:572-575`); for the Windows half, run locally, not left to CI: the cross-target checks and clippy (`:465-497`) and the Windows strict-panic legs (`:582-595`). The Windows and macOS targets are installed on this machine (the session's reading of 2026-11-14), and `check_release.sh` only warns where one is missing (`:466-472`, `:490-491`, `:583-584`), so a missing target would pass silently; CI's Windows leg then runs C.3's Windows-only test |
| 5 | D.1 (gpui), D.3 (iced), D.4 (showcases, `tools/showcase-support`); after unit 3, whose presets D.3's `UNREACHABLE` exceptions and D.1's tests read; its edits of `connectors/native-theme-gpui/src/` and its examples keep the line count above every line a colour claim cites, or re-point those claims in the same commit (S6's fourth gate, `every_colour_claim_is_read_at_the_line_it_cites` in `src/showcase.rs`) | `--quick` or `--commit` as the count gives it (rule 6): the connector tests (`:705-772`), clippy (`:740-773`), strict panic for the gpui library and the showcases (`:616-654`), `cargo doc` of the connectors (`:1009-1021`) |
| 6 | Part E: E.1 to E.4, the version bump, close-out (E.6) | E.1 to E.4: the five data-document checks and `--quick`, also where they are the last unit before the merge, the gated head (rule 20: no extra `--commit`); the version bump: `--commit`, a lock change's (it changes the manifests and `Cargo.lock`, rule 6); the close-out: the full run: docs.rs emulation (`:1078`), semver checks (`:1162`, soft), the two stamps |

Order: 0 (where there is one), 2, 3, then 4 and 5, then 6. There is no
unit 1: the citations it re-pointed are left as they are (S6), and the
other units keep their numbers. Units 4 and 5 do not depend on each other.
**The gates' cadence** (the batch's S1, `docs/todo_v0.7.1_batch-speed-rationale.md`,
approved by the maintainer on 2026-11-15; it replaces this section's earlier
sentence that `--commit` runs at the end of every unit). During a unit, the
agent runs only the touched tests (its targeted commands). At a unit's end,
`scripts/check_release.sh --quick` (what it runs on 0.7.x is defined by
`docs/todo_v0.7.1_gate-speed-spec.md`, S1; v0.7.1 ports it), by the
orchestrator, as a long run until its time is measured (the first timed into
the record); and `scripts/check_release.sh --commit` (the per-commit subset,
`:29-34`) where the table says (rule 6): every third code unit counted from
the last `--commit`; on the release's last code unit before its documents
and version units (the later of units 4 and 5); after a lock or toolchain
change (a warm-up gate: a new worktree, a `Cargo.lock` or toolchain change,
the merge; and the version bump, which changes the manifests and
`Cargo.lock`), so a documents unit between that unit and the bump takes
`--quick`, the gated head among them (the documents unit before the merge,
rule 20, with no extra `--commit`); and on a unit that is structural, one that changes a
manifest, `Cargo.lock`, a build script, the toolchain or the gate scripts
(rule 24; of units 2 to 5 only unit 4 is, and only with C.3, which edits
`native-theme/Cargo.toml`'s comment). A documents-only unit runs the
five documentation checkers and `--quick`. The full run where the plan has it
(the close-out), unchanged. A commit between two `--commit` runs is proven by
`--quick` only; a failure the next `--commit` finds is fixed by a new commit,
and the record names the commit that brought it.

**The branch** (S2). The release's branch is made from the gated head of the
release before it in its own lane (the KDE lane, S3 as amended on
2026-11-15: v0.7.1 → the KDE steps 2 and 3 → steps 4 and 5 → the caret →
steps 6 and 7; here the steps 4–5 release, once its documents unit, the
last unit before its merge, has passed its gate and its reviews are closed,
rule 20), and merged with the tag of the release immediately
before it in the maintainer's release order (under D10's proposal with S5's
pairs the icons', `v0.7.5`, which carries steps 4 and 5 through its own
merge), which may be the other lane's:
`git -c merge.conflictStyle=diff3 merge --no-ff --no-commit -m "Merge <prev-tag> into v0.7.N-rc1" <prev-tag>`
(rule 17; the merge commit made only after the resolutions and
regenerations below), once, after this release's last unit before the merge
has passed its gate, every review is closed and the tag exists, before X,
the version bump and the close-out (S2 as amended on 2026-11-15: a merge,
not a rebase, so no commit changes its sha). The orchestrator resolves every
conflicted hunk by both sides' quoted texts, a citation like any other text
(S6), names each in the record, and stops where a resolution would change a
text's meaning; every file resolved, but Markdown that no test parses (so
`.rs` files, manifests, a TOML a test reads, a script,
`docs/platform-facts.md`, a README and the egui connector's `mapping.md` are
in), is reviewed, by `review-hard` where it changes a hard part of a core
(rule 21 (4)). Where both sides re-pointed the same line a gate reads by
number (a gpui colour claim, S6's fourth gate), its number is the base plus
both sides' deltas, which the warm-up gate confirms. A generated file
(`mapping.md` among them) is regenerated by its command on the merged tree,
as the orchestrator's long run under the gate lock (rule 2), never resolved
by hand; a count or verdict tuple a test or gate checks
(egui's leaf count and verdict tuples, `one_gsettings_read_per_key`'s
counts) is the base plus both sides' deltas, and a prose count takes the
tag's side and is named (rule 21). `Cargo.lock`'s toolkit versions are
compared with the branch's start's, and where one changed (egui 0.37
normally arrives only here), the upstream lines this specification cites
are read again in the new version and the forms the units took on the older
one re-written (rules 19 and 25), with no cargo run before the warm-up but
`cargo fmt --all --check` and, for the agent that re-writes the code
forms, one `nice 19` `-p` build or clippy of the crate it re-wrote, through
`slot.sh` (rule 21); such a commit in a code file is reviewed. Then
the warm-up gate runs last, covering those commits, and X recounts every
stated count in the files either side changed, outside the pinned
documents, and runs the five documentation checkers (rule 17). Nothing is pushed and no close-out (the
version bump, the compatibility run, the full run, the asset run) starts
before that merge. Where the branch is made from the tag itself, there is no
merge.

**The lanes** (S3, as amended by the maintainer on 2026-11-15). This work
runs in the KDE lane, in the worktree `kde-b`, which the steps 2–3 release
used before it and which it reuses once that release has its tag (rule 18);
the second lane, beside it, holds gpui-component's own icons alone. The
machine's limits hold for both lanes together: at most two cargo processes (one gate, and one `nice 19`
targeted build in the other lane), the gates serialized by the shared
`slot.sh` locks, at most four agents (six where two run no cargo), one asset
run at a time, one capture window and one private-X rehearsal at a time, one
Fable agent at a time. The files this work and the icons' release both
edit meet at the merge, by quoted text, as every edit here is placed:
`docs/evidence/README.md`, where both append a section after
`## breeze-offscreen/` (Part 0.2; the icons' specification), so the merge
conflicts there for certain; `connectors/native-theme-gpui/src/lib.rs` (D.1;
the icons' D.10); `connectors/native-theme-gpui/README.md` (D.1; the icons'
F.1, in a different section); and the release documents `CHANGELOG.md`, `ROADMAP.md`
and `docs/todo.md`.

**The close-out's order** (S4). The branch is pushed once the compatibility
run (E.6, `scripts/update_compatibility.sh run`) has passed, the work's one
push before E.6's asset run, which needs HEAD pushed
(`scripts/generate_assets_release.sh:17`, checked at `:74-87`); the full
`scripts/check_release.sh` runs while CI runs on that push; the asset run
starts when both are green (rule 9: CI green, and the full check's only
failure the provenance stamp, which only the asset run refreshes). Where C.3 is done, the notice of that push names
its CI run as the first of C.3's Windows-only test (*Not verified*), since a
red CI run emails the maintainer. The version decision and the release, its
tag and its publication are the maintainer's (B6).

Not covered by any gate: the doc and Markdown text of every part, the
README sentences, the links of A.8 (E.6 opens them).

## Under the other B6 options

- **B6 (b), a minor version.** Nothing in Parts 0 to D changes; the
  CHANGELOG heading becomes the minor version when the release sets it, and
  E.3's heading likewise.
- **B6 (c), `#[non_exhaustive]` model structs first.** A release of its own
  before this one, specified separately (not written). This specification's
  Breaking Changes entry (E.1) then goes; every in-repository struct literal
  of a model struct outside `native-theme`'s crate stops compiling
  (`native-theme/tests/merge_behavior.rs:347` among them) and is that
  release's work, not this one's.
- **Into v0.7.1 instead of a release of its own.** The entries of E.1 go
  under `## [0.7.1]`'s sections (a `### Breaking Changes` section is added
  there under (a)); `CHANGELOG.md:12`'s last sentence, "`apply` keeps the
  application's `cursor_blink`, which native-theme does not model, as it keeps
  `notification`, `list`, `sheet` and `motion`.", is removed, since the gpui
  entry of E.1 then describes the same release; `ROADMAP.md:93-94`, "; and
  `apply` keeps the application's `cursor_blink`, which native-theme does
  not model", becomes "; and `apply` installs the theme's caret blink, which
  the model now states, as its `cursor_blink`", and E.3's section is not
  written, its content joining v0.7.1's paragraph. E.1's gpui entry then
  has no earlier release to compare with and reads: "**native-theme-gpui**:
  `apply` and `apply_system_theme` install the theme's caret blink as
  gpui-component's `Theme::cursor_blink`: steady where `caret_blink_off_ms`
  is 0, else blinking with each phase half of the two durations together
  (gpui-component blinks in equal phases), and the time-out as its idle
  time-out. Where the theme states no blink the application's own comes
  back, as for the tooltip delay, decided by value; `apply_accessibility`
  leaves the blink, and a mode switch through gpui-component keeps it." (in
  `### Added`). v0.7.1's asset run and stamps are made again. Further:
  - E.6's version bump falls: the workspace is 0.7.1 already
    (`Cargo.toml:17`; the inter-crate requirements are 0.7.1, `Cargo.toml:26`,
    `native-theme/Cargo.toml:72`,
    `connectors/native-theme-egui-widgets/Cargo.toml:40`), and
    the heading `## [0.7.1] - 2026-11-14` exists (`CHANGELOG.md:8`); its
    date is v0.7.1's close-out's to set again where the release falls on
    another day.
  - E.6's compare link becomes v0.7.1's own, `[0.7.1]:
    https://github.com/tiborgats/native-theme/compare/v0.7.0...v0.7.1`, at
    the top of the link list; at `0fa28027` the list starts with `[0.7.0]`
    (`CHANGELOG.md:1802`), so the line is written once, by whichever of
    v0.7.1's works closes the release, and not again where it is there.
  - E.6's archive row is not a new one: the existing v0.7.1 row
    (`docs/archive/README.md:51`) gains "; `todo_v0.7.1_caret-blink-rationale.md`,
    `todo_v0.7.x_caret-blink-{spec,plan}.md`: the text caret's blink", as
    the KDE step 1 specification's G.6 extends the same row.
  - E.3's heading and section are not written (above); E.1's
    `## [0.7.N] - <date>` heading is not added.
  - The header's paragraph that begins "**This release is the release after `<prev-tag>`**"
    becomes: "**This work lands in v0.7.1**, beside the GPUI Kit 0.9.1 port
    and the KDE step 1 (`docs/todo_v0.7.1_kde-open-items-step1-spec.md`),
    on the tree the v0.7.1 work before it leaves; no release lands between
    `0fa28027` and it, and the work after v0.7.1 (egui 0.37, the KDE steps
    2 to 5, gpui-component's own icons) lands after it. Every edit below is
    applied by its anchor (the quoted text, the key, the test's name); a
    line number is `0fa28027`'s and serves to find the place; a count is
    given as today's value and as a delta (+3 leaves; the inputs' DERIVED
    +2 and UNMAPPABLE +1; `egui-limited` +1; `gsettings` reads +3), and the
    implementer adds the delta to the value the tree holds then; the facts
    rows go right after §2.4's `caret_color` row, by A.8's rule, wherever
    that row is then."
  - D.2's paragraph that begins "egui 0.37, which lands before this release"
    becomes: "egui 0.37 lands after this work: v0.7.1 ships egui 0.36.2
    (`connectors/native-theme-egui/Cargo.toml:42`), whose line numbers `EG`
    above gives, and the upgrade is a backlog entry (`docs/todo.md`,
    *Dependencies: the newer releases the outdated check did not show
    (2026-11-14)*). It changes nothing here when it lands: in egui 0.37.0
    (Cargo's registry) `TextCursorStyle` has the same five fields
    (`src/style.rs:1061-1076`), its defaults are the same (`blink: true`,
    `on_duration: 0.5`, `off_duration: 0.5`, `src/style.rs:1083-1085`), and
    `paint_text_cursor` blinks the same way
    (`src/text_selection/visuals.rs:291` on)."

## Not verified

- Whether `UISettings.CaretBlinkRate` is the system's caret blink time: its
  page does not say (open point 2), and it was not measured.
- The line numbers of `gtk-cursor-blink` in GTK 4.24.1's
  `gtk/gtksettings.c` and of `blink_cb` in `gtk/gtktext.c`: not counted and
  not quoted by the parked facts, so A.8 cites them by name; the other two
  properties' lines are the facts' (`:364-366`, `:378-380`), not re-counted.
- That `read_windows()` returns `Ok` on CI's Windows image,
  `windows-latest`: no run and no existing test has shown it, so C.3's
  Windows-only test first sees it in CI; the notice of the branch's one
  push (*Order and gates*) says so, since a red CI run emails the
  maintainer.
- How KConfig hands on a malformed `CursorBlinkRate`: the reader keeps the
  preset's 500 / 500, which is Qt's at the default 1000; if Qt gets 0 from
  a malformed value instead, Qt would not blink where the reader says it
  does.
- Whether the settings portal forwards changes of the three GNOME keys:
  `native-theme/src/watch/gnome.rs:7-15` says it forwards every key of the
  schema; its cited lines were not re-read for these keys.
- The URLs of A.8: built from `F/sources.toml`'s pins and the file's patterns,
  not opened.
- The upstream sources behind `F`'s entries, as in the rationale's §4.
- That the Windows and macOS Rust targets are installed on this machine:
  the session's reading; not checked here (it needs `rustup`, outside what
  this specification's author could read).

Verified since the first version: Part 0's reproduction (byte-identical
output, Pillow 13.0.0, one `DeprecationWarning`), GNOME's three keys and
GTK's use of them (read by the session on 2026-11-14, C.2 and A.8),
Microsoft's page of `GetCaretBlinkTime` (open point 2), egui 0.37.0's
`TextCursorStyle` (D.2), and the types in GTK 4.24.1's `blink_cb` time-out
test (`int blink_timeout`, `int get_cursor_blink_timeout`, the `guint64`
`blink_start_time`; read 2026-11-14 from
`gitlab.gnome.org/GNOME/gtk/-/raw/4.24.1/gtk/gtktext.c`, C.2).

## Open points

1. **B6: the version and the release (the maintainer's).** Options: (a) a
   0.7.x release with the break named in the CHANGELOG (proposed; this
   specification); (b) a minor version; (c) `#[non_exhaustive]` model
   structs first. And where it lands: a release of its own after v0.7.1
   (proposed, item 4 of D10's list in the KDE document) or v0.7.1.
   *Under the other B6 options* gives what differs.
2. **Windows: the `unsafe` call that reads the blink time (B4; the
   maintainer's).** Read on Microsoft Learn on 2026-11-14:
   - `UISettings.CaretBlinkRate`
     (`learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uisettings.caretblinkrate`):
     "Gets the blink rate of a new caret created by the app view." Property
     value: "The blink rate of the new caret, in milliseconds." Nothing more:
     the page does not say that it is the setting `GetCaretBlinkTime`
     returns, nor what it gives where blinking is turned off.
   - `GetCaretBlinkTime`
     (`learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcaretblinktime`):
     "Retrieves the time required to invert the caret's pixels. The user can
     set this value." Return value: "If the function succeeds, the return
     value is the blink time, in milliseconds. A return value of INFINITE
     indicates that the caret does not blink. A return value is zero
     indicates that the function has failed." It takes no argument and the
     page states no precondition; the `SAFETY` comment says so and links it.

   By B4's rule (the safe call only where the documentation shows it is the
   same setting) the reader calls `GetCaretBlinkTime`: one new `unsafe` call
   in `windows.rs`, in the form of the file's other system calls. The
   choice left is the maintainer's: approve that call, or leave Windows
   unread in this release (the preset's 530 / 530 stays, and C.3 is not
   done; C.5 lists every edit that then falls or reads differently, with
   its text).
