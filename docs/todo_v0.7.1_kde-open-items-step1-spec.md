# 0.7.1: issue #4's open KDE items, step 1 (the table's grid and the menu panel) — specification

- Rationale: [`issues/0004_kde-open-items-rationale.md`](issues/0004_kde-open-items-rationale.md)
  ("the rationale"), decisions D2 and D3 and what D10 says a step carries outside
  the code. To be implemented once free of errors (the maintainer, 2026-11-15:
  "When the plan that goes in the 0.7.1 version is ready, without errors, start
  implementing it."), with its plan. It decides
  nothing the rationale did not decide, except the small points under *Decisions
  taken while specifying*; what needs a decision the rationale does not contain is
  under *Open points*.
- It continues [`issues/0004_kde-hand-overrides-rationale.md`](issues/0004_kde-hand-overrides-rationale.md)
  ("the first rationale"), whose §5.5 and §5.6 deferred both items, and the backlog
  entries at `docs/todo.md:1767-1933`. Form: the specification of the first KDE
  fixes, [`archive/todo_v0.7.1_kde-hand-overrides-spec.md`](archive/todo_v0.7.1_kde-hand-overrides-spec.md).
- Written: 2026-11-14, against `v0.7.1-rc2` at `0fa28027`. Every `file:line` is that
  commit's unless it names another. `v0.8.0-rc1` is the parked branch at `78ee0efd`
  (the main checkout); `P8` is that checkout, `F` its `docs/facts/`.
- The batch's working rules are cited as "rule N (batch-speed rationale, Appendix)":
  `docs/todo_v0.7.1_batch-speed-rationale.md`, *Appendix — the batch's working
  rules*, authoritative where this text differs.
- Release: v0.7.1, D10 (a), which the maintainer chose on 2026-11-15 (open point
  1). The release numbers below are the order the maintainer approved with S3's
  amendment (2026-11-15, `docs/todo_v0.7.1_batch-speed-rationale.md`; D0 does not
  touch this step, which adds no field): 0.7.1 the GPUI Kit completion and step 1,
  0.7.2 egui 0.37, 0.7.3 KDE steps 2 and 3, 0.7.4 KDE steps 4 and 5, 0.7.5 the widget
  icons, 0.7.6 the caret, 0.7.7 KDE steps 6 and 7.
- Every value below was read today from the file named, or computed today; the
  computations are named where they are given. What could not be verified is
  marked so, with the reason.

## How to read it

Eight parts. A to F are the work, by owner; G is the work outside the code that D10
gives every step; H the order, the gates and the close-out.

| Part | Rationale | Files it owns |
|---|---|---|
| A — The shade function | D2, "The reader" | new `native-theme/src/kde/shade.rs`, new `native-theme/tests/vectors/kde_shade.tsv`, the module declaration in `native-theme/src/kde/mod.rs` |
| B — KDE reader and preset | D2, D3 | `native-theme/src/kde/colors.rs`, the tests in `native-theme/src/kde/mod.rs`, `native-theme/src/presets/kde-breeze.toml` |
| C — Facts, rules, registry | D2, D3 | `docs/platform-facts.md`, `docs/inheritance-rules.toml` (`docs/property-registry.toml`: no edit) |
| D — gpui connector | D2 (the chart's grid), D3 (gpui) | `connectors/native-theme-gpui/src/colors.rs`, `src/contract.rs`, `examples/showcase-gpui/info/theme_map.rs` |
| E — iced connector and its showcase | D3 (iced) | `connectors/native-theme-iced/src/styles.rs`, `src/styles/aw.rs`, `src/contract/rows.rs`, `src/contract/derived.rs`, `contract-leaves.toml` (generated), `src/lib.rs`, `README.md`, `examples/showcase-iced.rs` |
| F — egui and the showcases' tables | D2, D3 (egui) | none: what stays, and why |
| G — Outside the code | D10 | `CHANGELOG.md`, `ROADMAP.md`, the first rationale (§4.2, §5.5, §5.6, §7, §8), the rationale's status line, `docs/todo.md`, `docs/archive/README.md` |
| H — Order, gates, close-out | D10 | both release stamps, the asset run |

Every edit has one owner. One file is edited by two parts:
`native-theme/src/kde/mod.rs` (Part A adds the module declaration, Part B changes
tests found by name); `CHANGELOG.md` is Part G's alone. An edit is applied by
its quoted text or its test's name; the line numbers are those of `0fa28027` and move
with the edits above them. A citation this step writes names the document, with a
quoted phrase where useful, and no line number of a file that changes; the existing
citations of lines that move are left as they are (the maintainer's S6,
`docs/todo_v0.7.1_batch-speed-rationale.md`).

**Lines that must not move.** The gpui showcase's test
`every_colour_claim_is_read_at_the_line_it_cites`
(`connectors/native-theme-gpui/src/showcase.rs:2819`) opens each colour claim at the
source line it cites. At `0fa28027` the claims pin lines 281 to 718 of
`connectors/native-theme-gpui/src/colors.rs` (listed today from every
`colors.rs:N` in `examples/showcase-gpui/` and `src/showcase.rs`), among them `:440`
(`table_row_border`) and `:446` (`chart_grid`, cited by `info/theme_map.rs:1244`).
Part D keeps the line count of `colors.rs` from line 1 to 718.

## Decisions taken while specifying

Each is the smaller or the safer choice and changes no result of the rationale.

1. **A. Where the function goes.** `native-theme/src/kde/shade.rs`, module `shade`,
   declared private in `native-theme/src/kde/mod.rs`; the function and its role
   enum keep their names, `kde_shade` and `KdeShadeRole`. The table goes to
   `native-theme/tests/vectors/kde_shade.tsv`, the path it has on `v0.8.0-rc1`:
   `src/kde/` is as deep as `src/platform/`, so the carried test's
   `include_str!("../../tests/vectors/kde_shade.tsv")` (P8 `kde_shade.rs:339`) holds
   unchanged. The 0.7.x tree has `src/kde/qt_color_vectors.rs` as precedent for
   tables beside the reader, but moving the table there would change the carried
   file.
2. **A. The notice** goes into the carried file's own module doc, as a paragraph
   after its three lines: the 0.7.x tree has no `platform/` parent to hold it, and
   `kde/mod.rs`'s module doc is about the reader.
3. **A. The roles the reader does not use.** The reader takes `KdeShadeRole::Mid`
   alone, so outside tests rustc finds `Light`, `Midlight`, `Dark` and `Shadow`
   never constructed (`dead_code`, a warning, an error under the release check's
   `-D warnings`). The allow goes on the module's declaration, not into the file:
   `v0.8.0-rc1` does the same for its clean-room files
   (`P8 native-theme/src/platform/mod.rs:35-39`, "The clean room's file stays as
   written"). It is `#[cfg_attr(not(test), allow(dead_code, reason = …))]`: in a
   test build the carried test constructs every role, and the crate's own pattern
   for such allows is `cfg_attr(not(..), allow(dead_code))` (`native-theme/src/error.rs:379-400`,
   `macos.rs:60`, `:71`). The module is compiled only on Linux with `kde`
   (`native-theme/src/lib.rs:270-271`, `#[cfg(all(target_os = "linux", feature = "kde"))] pub mod kde;`):
   in the release check that is every Linux build of the core with `$NT_FEATURES`,
   `--all-features` or a connector's `native` (which turns `kde` on through `linux`,
   `native-theme/Cargo.toml` `[features]`), and none of the macOS, Windows or
   `--no-default-features` legs. The alternative weighed is `expect(dead_code, …)`
   in the same `cfg_attr`: it would fail the build wherever the lint does not fire,
   and builds that compile the module without running `dead_code` (rustdoc's, among
   them `check_release.sh:1004-1006`, which documents the core's private items with
   `$NT_FEATURES`, and the docs.rs emulation) were not checked for how
   they treat an unfulfilled expectation; `allow` cannot fail there. Kept: `allow`.
4. **B. Two constants, one helper each.** `CONTRAST_DEFAULT: i32 = 7` beside
   `FRAME_CONTRAST_DEFAULT` and `MENU_PANEL_MIX: f64 = 0.3` beside
   `PRESSED_BUTTON_MIX`, in the file's pattern of one constant per use with its
   citation (`colors.rs:358-369`). `MENU_PANEL_MIX` is an `f64` because Breeze's
   `Metrics::Blend_Value` is a `qreal` (the file says so at `colors.rs:364-368`);
   `BLEND_VALUE` (`:18`) is the `f32` alpha form. Both give #f4f5f5 / #1c1f22 under
   Breeze (computed, see Part B §B.3).
5. **D. The chart's grid is assigned where it is today.** `assign_list_table`
   (`colors.rs:414`) takes the resolved theme in place of its unused `_is_dark`, as
   `assign_misc` (`:516`) takes it already, so `tc.chart_grid` stays at `:446` and
   no pinned line moves. A field on `ResolvedColors` would move every line from
   `:83` on and every claim with them.
6. **D. The gpui pair "hovered menu row" gets two exceptions.** It compares the
   hover tint over `menu.background_color` (native) with the tint over the
   `popover` token (emitted, `contract.rs:1987-1998`). With Breeze's menu panel in
   kde-breeze the emitted pair is the lower one in both modes (computed below),
   which `no_pair_contrasts_worse_than_the_platforms_own` fails. The cause is the
   one D3 states for gpui: gpui-component has no menu surface; so the pair
   excepts `kde-breeze/light` and `kde-breeze/dark` with that reason, in the form
   of `IDLE_TAB_EXCEPTIONS` (`contract.rs:1864`).
7. **E. `UNREAD_COLOURS`: three leaf entries where one widget entry was.**
   `menu.border.color` reaches no row once both menu styles frame in
   `popover.border`, and the whole-widget entry `popover` (`derived.rs:1532-1539`)
   would name `popover.border.color`, which two rows then read, so
   `every_colour_leaf_reaches_a_row_or_is_named` fails (`contract.rs:5631-5646`).
   The popover's two other colour leaves still reach no row: `popover.background_color`
   (the drop-down keeps the menu's fill, D3 (i)) and `popover.font.color`. The kinds
   are the ones the enum defines: `Ruled` for `menu.border.color` and
   `popover.background_color` (iced has a receiver, and D3 fills it from another
   leaf: the frame from `popover.border`, the drop-down's fill from
   `menu.background_color`); `NoReceiver` for `popover.font.color` (`NoWidget`
   requires a widget's name, not a leaf, `contract.rs:5700-5703`). `Ruled`'s doc
   names the M5 review as its only source of rulings and gains the rationale's.
8. **E. The `iced_aw` menu bar's frame becomes `DERIVED`.** `bar_border` has no
   native source after D3 (`Border::default()`), so its six walked fields go to
   `DERIVED` beside `path_border`'s, and its `BorderRow` goes.
9. **E. The Extra page's panel padding returns the window menus'.** Once its line
   is `popover.border.line_width`, `aw_menu_padding` (`showcase-iced.rs:15026-15037`)
   computes what `menu_popup_padding` (`:15009-15014`) computes, so its body becomes
   `menu_popup_padding(resolved)`: equal by construction, with no test needed to
   hold two copies together. The name stays, since five tests and two Widget Info
   texts name it. No gate reads the leaves in a padding function's own text
   (searched: the showcase's tests that read its source, `SHOWCASE` at `:20765`,
   check which `styles::*` function dresses each widget constructor; the route test
   draws `menu_popup_padding` through `drawn_by`, `:29032`, and names no
   `aw_menu_padding`), so nothing needs the leaves spelled out in it.
10. **C. Three facts cells gain a clause.** The grid's cell states KColorScheme's
    default 7, which the reader uses (a default is used only where the facts name
    it); the menu panel's says Qt Quick's menu was not read (D3's last bullet); the
    popover's background cell gives the measured Qt Widgets combo box popup (D3's
    fact, which no row states today).
11. **G. The first rationale's §4.2 sentence is reworded and its §5.5 and §5.6 each
    gain one line** pointing at the decision that settled them. D10 names only §7 and
    §8; without these edits §4.2 and the two sections still say the panel and the grid
    wait.
12. **E. The iced showcase's window menus keep their own style function, held
    equal to the connector's by a test.** D3 says the showcase uses the
    connector's `styles::aw::menu`. Read today: `styles::aw` exists only with the
    connector's feature `iced_aw`
    (`connectors/native-theme-iced/Cargo.toml`, `iced_aw = ["widgets", "dep:iced_aw"]`);
    the example's required features are `widgets`, `spinner`, `system-fonts`,
    `advanced-shaping`; the window menus (`menu_bar`, `showcase-iced.rs:5395`, and
    the page tabs' menu, `:6501`) are built in every build, and the release check
    builds and tests the showcase without the feature (`scripts/check_release.sh:621`,
    strict panic, default features; `:841`, tests) and with it (`:619`, `:843`; each
    the leg's `run_check` or `run_tests` line, its command on the next); the
    screenshots are taken with it (`scripts/generate_screenshots_iced.sh:35`). Today
    `menu_bar_style` (`showcase-iced.rs:6518-6550`) already returns exactly what Part E
    makes `styles::aw::menu` return. The options:
    - (a) The showcase keeps `menu_bar_style` for its window menus, unchanged but for
      its doc, and a new test built with `iced_aw` holds it equal to `styles::aw::menu`
      on every bundled combination and status.
    - (b) The window menus call `styles::aw::menu` where the feature is on and
      `menu_bar_style` where it is off: two code paths, the showcase's routes and
      `drawn_by` name both, and the build without the feature still needs the
      showcase's own function.
    - (c) `iced_aw` joins the example's required features: one function, but
      `cargo test -p native-theme-iced` without the feature then skips the showcase,
      and the release check's legs at `:621` and `:841` change or go.

    **(a).** D3's aim is one look that cannot drift, and (a) reaches it with a gate:
    the test fails when either definition changes alone. (b) keeps both definitions
    as well and adds a second path with nothing held equal in the build without the
    feature. (c) gives up two legs of the release check to remove a function of
    thirty lines. The rationale's D3 says so.

    **What the test holds, and what it does not.** Both functions return
    `iced_aw::style::menu_bar::Style`, which derives `Debug`, `Clone` and `Copy` and
    not `PartialEq` (`iced_aw-0.14.1/src/style/menu_bar.rs:6`), so the test compares
    the two `Debug` texts, status by status: every field of the `Style` (the bar's
    and the menus' fills, borders and shadows, the open path's fill and border). It
    does not hold what the showcase gives the `MenuBar` and its `Menu`s outside the
    `Style`: the panels' padding and offset (`menu_popup_padding`, `:15009-15014`,
    which the Extra page's padding returns, decision 9), the Extra page's
    `AW_MENU_GAP` (`:15000`), and the window menu bar's sides
    (`layout.container_margin`, `MENU_BAR_PADDING`, `:5610-5615`).
13. **E. A test holds each menu Info's "not drawn" leaf to the leaf the style draws
    the line from.** The Widget Info names why an edge is not drawn by a hand-written
    leaf (`line_zero`, `showcase-iced.rs:12169-12171`; shown as "not drawn (…)",
    `:9297`), and no gate compares it with the leaf the style draws the line from
    (searched: every `line_zero(` and `.zero` site of the file, `:9297`,
    `:9374-9425`, `:12070-15507`). One test reads the text:
    `every_page_colour_is_the_style_output_at_its_status` (`:28559`) has
    `colour_row_faults` (`:28479-28548`) parse each "not drawn (…)" row and
    `zero_reason_holds` (`:28354-28374`) check that each leaf it names is 0 or
    unstated in the theme. It does not check that the style reads that leaf: after
    E.2 without edit 0, material-3's drop-down Info would say "not drawn
    (menu.border.line_width 0)", which holds, since `menu.border` resolves to no
    line. After E.2 and E.3 the two menu Infos would name `menu.border` for
    a line the styles take from `popover.border`, so E.6 changes both and adds
    `the_menu_infos_name_the_width_the_menus_draw`.
14. **B. A `[KDE] contrast` that is not an integer gives the default, 7**, as a
    missing key does. KConfig's `readEntry` keeps its default where it cannot convert
    the value (kconfig `src/core/kconfiggroup.cpp:256-269` at v6.31.0, which the
    reader already cites, `native-theme/src/kde/mod.rs:158-164`), and KColorScheme
    reads the key as `readEntry("contrast", 7)` (B.1), so this is what Plasma's own
    palette takes. A value with a fraction, `4.5`, is taken as not an integer here;
    how KConfig's conversion treats it was not read (kconfig's source is not in the
    tree, only the reader's citation of it).

## Corrections to the rationale found while specifying

All are amended in the rationale; none is open.

- The showcase's window menus keep their own function, held equal to the
  connector's by a test (D3's iced bullet; decision 12).
- gpui's contrast pair "hovered menu row" degrades on kde-breeze once the menu panel
  is Breeze's (D3's gpui bullet and §3 item 3; decision 6).
- **A decision changed: the drop-down keeps the menu's fill** (D3 (i), §3 item 3),
  found by the confirming check of this specification, which first had the fill from
  the popover (D3 (ii)). A drop-down's rows are the menu's colours, which each preset
  states against the menu's fill; on the popover's fill they do not hold. Checked
  today: in adwaita's dark mode the highlighted row, `adwaita.toml` `[dark.menu]
  hover_background` #3a3a3e (`:696`), would lie on `[dark.popover] background_color`
  #36363a (`:758`), 1.06:1, where on the menu's #222226 it is 1.40:1 (WCAG ratio,
  computed in Python), with the same label on both. The label's contrast would fall on
  24 of the 25 combinations whose fill would change (G.5 item 2's 25; it rises only in
  adwaita's light mode, 12.12 to 12.65, where the popover is #ffffff on #fafafb); the
  second confirming check computed these from the preset files and the documented
  inheritance, not through the crate's resolver. With (i), no drop-down line
  has the colour of its fill on any of the 34 combinations (E.7, computed): the
  tokyo-night and one-dark dark cases of (ii) are gone.

A correction to the backlog, not to the rationale, found while specifying: the
upstream entry `docs/todo.md:2403-2413` says 25 of 34 combinations differ (measured
2026-10-11). At `0fa28027` it is 23 (computed: `windows-11` now states one colour for
both); after this step 25 (kde-breeze joins). G.5 item 2 updates it.

## Open points for the maintainer

1. **Where step 1 goes (D10).** Answered by the maintainer, 2026-11-15: D10 (a),
   step 1 ships in v0.7.1, as this document is written. The re-specification for
   (b), step 1 as v0.7.2, is not taken.
2. **The asset run (Part H)** needs the maintainer's desktop unlocked and untouched
   for 4 to 6 minutes (the last run's measurement, in the first plan's §8), once,
   after the last change of everything that lands in v0.7.1; and once before it,
   for unit 3's dry parity run (H.1), announced the same way.

## Cost and order

v0.7.1 was finished once (2026-11-14, `0fa28027`); this step reopens it.

- **What it needs approved:** the rationale's D2, D3 and D10's choice of where step 1
  goes (§3 items 2, 3 and the step-1 part of 6). D0, D4 to D9 and the order of the
  later releases are not needed for it.
- **Files**, counted from Parts A to H: 2 new (`native-theme/src/kde/shade.rs`,
  `native-theme/tests/vectors/kde_shade.tsv`); 22 edited: 16 in the code and data
  (`native-theme/src/kde/mod.rs`, `kde/colors.rs`, `presets/kde-breeze.toml`,
  `docs/platform-facts.md`, `docs/inheritance-rules.toml`, the gpui connector's
  `src/colors.rs`, `src/contract.rs` and `examples/showcase-gpui/info/theme_map.rs`,
  the iced connector's `src/styles.rs`, `src/styles/aw.rs`, `src/contract/rows.rs`,
  `src/contract/derived.rs`, `contract-leaves.toml` (regenerated), `src/lib.rs`,
  `README.md` and `examples/showcase-iced.rs`), 6 outside the code (`CHANGELOG.md`,
  `ROADMAP.md`, the two rationales, `docs/todo.md`, `docs/archive/README.md`);
  plus this specification and its plan, and the files the asset run writes.
- **The asset run** is 0.7.1's fourth: the branch's history holds three
  (`d8b9ec1c`, `387fc449`, `0f849012`, the commits that changed
  `docs/assets/PROVENANCE.toml` since v0.7.0, listed with `git log`). It is shared
  with the GPUI Kit port's completion (H.4).
- **Branch:** `v0.7.1-rc2` (the worktree's branch), whose pull request is #10 (the
  project's records; not looked up here, no network). Its title and description then
  cover this work too; editing or approving them is the maintainer's.
- **Order with the sibling specification** (`docs/todo_v0.7.1_gpui-kit-0.9.1-spec.md`):
  the GPUI Kit port's work, its Part G commits included, lands first, since its
  re-pointing run of upstream citations must apply to a tree in which no line this
  step writes exists yet; then step 1's units 1 to 3 (H.1); then unit 4, which edits
  after the port's Part G the documents both touch (`CHANGELOG.md`, `ROADMAP.md`,
  `docs/todo.md`), by quoted text; and one asset run (unit 5). So every
  edit of this specification is applied by its quoted text or its test's name; its
  line numbers are `0fa28027`'s and serve to find the place.

## Part A — The shade function

**Source.** `native-theme/src/platform/kde_shade.rs` on `v0.8.0-rc1` (434 lines,
sha256 `8cbf3aad…09c3` of the whole file, `b1b8fa52…e7d9` of lines 4-434), and
`native-theme/tests/vectors/kde_shade.tsv` there (273 lines, 16 207 bytes, sha256
`8328e762276bc22fee887e2c1dec1ff1436f911f9d028a428e343a15ebbe8913`). Its notice is in
its parent, `P8 native-theme/src/platform/mod.rs:1-18`: the KDE functions there "were
written in a clean room, from behavioural descriptions and the vector tables alone
(the plan's D10)". The table's header (lines 1-12) names its sources (kcolorscheme
6.31.0, kguiaddons 6.31.0, qtbase 6.12.0), its route (`compiled_source`) and its
generator (`tools/kde-vectors/generate.py`, which exists on `v0.8.0-rc1` only).

**What the file is.** Self-contained: it uses no other module, no crate but `std`, and
no type of the model (read today: no `use` outside its test module, which uses
`super::*`). Its public items are the enum `KdeShadeRole` (five roles) and
`kde_shade(c: [f32; 4], role: KdeShadeRole, contrast: f32) -> [f32; 4]`: straight-alpha
RGBA in 0..=1 in, the shade of the KDE colour scheme's `contrast` step out, through
Qt's 16-bit colour storage at every step; values of `contrast` beyond ±10 act as ±10
(its doc, `:273-282`). Its test module holds `rounding_to_half` and `vectors`, which
reads the table through `include_str!`, checks every one of its 260 rows to 1e-3 on
the 0-255 scale and asserts the row count (`:426`).

### A.1 The new file `native-theme/src/kde/shade.rs`

Written with the Write tool (no `git show … >` redirect: the project's Write hooks must
see it). Content, in this order:

1. Lines 1-3 of the source file as they are.
2. A blank `//!` line and this paragraph (the notice):

   ```text
   //!
   //! Written in a clean room on the parked 0.8.0 branch, from behavioural
   //! descriptions and its vector table alone, and carried here unchanged below this
   //! paragraph (`native-theme/src/platform/kde_shade.rs` and its notice in
   //! `native-theme/src/platform/mod.rs:1-18` on `v0.8.0-rc1` at 78ee0efd). No line
   //! of KDE's or Qt's code is in it. The table its test reads,
   //! `tests/vectors/kde_shade.tsv`, was generated there by `tools/kde-vectors` from
   //! kcolorscheme 6.31.0, kguiaddons 6.31.0 and qtbase 6.12.0, as its header says;
   //! the generator is not on this branch.
   ```
3. Lines 4-434 of the source file, byte for byte.

**Check:** `tail -n +13 native-theme/src/kde/shade.rs | sha256sum` gives
`b1b8fa5228f019036fa5f4e23936bc470669ff2f115a80005d6937b03e05e7d9` (the notice adds
nine lines, one blank `//!` and eight of text, so line 4 of the source, its blank line
after the module doc, is line 13 here), and `head -n 3` equals the source's lines 1-3
(sha256 `92068628…e073`). H.1 makes these checks a step of unit 1's gate.

### A.2 The new file `native-theme/tests/vectors/kde_shade.tsv`

The table, byte for byte (a new folder `native-theme/tests/vectors/`). **Check:**
`sha256sum` gives `8328e762…8913` as above. Cargo does not take a `.tsv` in `tests/`
for a test target. `native-theme/Cargo.toml` has no `include` or `exclude` key (read
today: a search of the file for both words finds none), so Cargo packages the crate's
whole folder but for what it leaves out by its own rules, `tests/vectors/` among the
packaged files; the test module needs it in a packaged build. That the package list
holds it was not run (no cargo command).

### A.3 The declaration in `native-theme/src/kde/mod.rs`

After `:29` (`pub(crate) mod fonts;`), insert:

```rust
/// KColorScheme's shades of a colour, in Qt's arithmetic: the clean-room
/// function of `v0.8.0-rc1`, carried with its vector test (its own notice).
#[cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "the reader takes the Mid shade alone; the module is carried as the clean room wrote it, and its test builds every role"
    )
)]
mod shade;
```

### A.4 What the project's gates make of the carried code

| Gate | What it checks | The carried file |
|---|---|---|
| `#![deny(clippy::unwrap_used)]` and `#![deny(clippy::expect_used)]`, two attributes (`native-theme/src/lib.rs:176`, `:177`) | the two calls | none outside the test module (read today) |
| `scripts/check_strict_panic.sh -p native-theme --lib --features "$NT_FEATURES"` (`check_release.sh:573`; `:575` is the leg without features, which does not compile the module; `:593`, `:595` the cross-target legs, which do not either) | fifteen panic-prone lints (`scripts/check_strict_panic.sh`, `STRICT_PANIC_LINTS`), `arithmetic_side_effects`, `indexing_slicing`, `modulo_arithmetic` among them | the same list as on `v0.8.0-rc1` (the `STRICT_PANIC_LINTS` arrays diffed equal today), where the file carries `#[allow(clippy::modulo_arithmetic, reason = …)]` on `wrap` (`:132-135`) and two `#[expect(clippy::neg_cmp_op_on_partial_ord, …)]` (`:118-121`, `:283-286`); same MSRV, 1.88.0 (both workspaces' `rust-version`). Whether it passes in this tree is **not verified**: no cargo run was allowed. If it does not, the work stops and the maintainer is told what failed: under the project's rule a strict-panic finding in code is fixed or brought to the maintainer, never allowed by the implementer, and the carried file is not edited (`dead_code`, decision 3, is no panic lint and is not touched by this) |
| `scripts/check_panic_patterns.py` (`check_release.sh:400`) | `.unwrap()`, `.expect(`, `panic!(` … outside test code | none outside the test module |
| clippy `-D warnings` (`check_release.sh:529-538`) | default lints, `dead_code` | `dead_code` handled by A.3; the two `#[expect]`s must be fulfilled under this toolchain, as on `v0.8.0-rc1` |
| `.claude/hooks/no-invented-values.sh` (main checkout) | nine regexes over Write/Edit content; it exempts `*/src/platform/*`, which `src/kde/` is not | none of the nine matches the file (each regex run with `grep -cE` over it today: 0 matches) |
| `.claude/hooks/no-runtime-panics.sh` | `.unwrap()`, `.expect(`, `panic!` …, `unsafe` | skips content holding `#[cfg(test)]`; none outside it anyway |
| rustdoc, `missing_docs` (`lib.rs:174`) | public items reachable from the crate's API | none: `shade` is private, its items crate-internal |
| `cargo doc … --document-private-items --features "$NT_FEATURES"` under `-D warnings` (`check_release.sh:1004-1006`) | every doc of the crate, private items included: broken intra-doc links, rustdoc warnings | the carried docs use backticks, no intra-doc links (read today); the notice adds none |

The vector test is the module's own; it runs in `cargo test -p native-theme --features "$NT_FEATURES"`
(`check_release.sh:683-684`) and in any build with `kde` on Linux. `P8`'s
`tests/platform_vectors.rs` (`vectors_kde_shade`, `:158-171`) is not carried: it reads
the same rows through `P8`'s `Color`, `channels`, `number` and `assert_color` helpers,
which this tree has not; the module's own `vectors` checks the same 260 rows to the
same tolerance.

## Part B — KDE reader and preset

### B.1 The keys and how they are read

| Key | Read today | Use |
|---|---|---|
| `[Colors:Window] BackgroundNormal` | yes, `window_bg` (`colors.rs:638`) | the colour the grid shades; the first colour of the menu panel's mix |
| `[Colors:View] BackgroundNormal` | yes, `view_bg` (`:701`) | Base, the second colour of the mix |
| `[KDE] contrast` | no | the shade's step |

**`[KDE] contrast`.** KColorScheme reads it as an integer with the default 7:
`contrastF` returns `0.1 * g.readEntry("contrast", 7)` (`F/kde/tokens.toml:11343-11353`,
key `token.params.color.contrast`, quoting kcolorscheme 6.31.0
`src/kcolorscheme.cpp:519-527`); Plasma writes it into kdeglobals from the scheme,
default 7 (the same fact, plasma-workspace 6.8.2 `kcms/colors/colorsapplicator.cpp:318`);
Breeze's schemes state 4 (`docs/platform-facts.md:1430`, BreezeLight.colors:195 and
BreezeDark.colors:202 at breeze f0b1d75). The reader:

- reads `ini.get("KDE", "contrast")`, trims it and parses it as an `i32`; a missing key
  or a value that is not an integer gives `CONTRAST_DEFAULT`, 7 (as `frame_contrast`,
  `colors.rs:442-450`, gives its default for a missing or non-numeric
  `frameContrast`), which is what KConfig itself returns: `readEntry` keeps the
  default it is given where it finds no key or cannot convert the value, as the
  reader already cites for the icon sizes (`native-theme/src/kde/mod.rs:158-164`:
  kconfig `src/core/kconfiggroup.cpp:690-693` and `:256-269` at v6.31.0), so
  `readEntry("contrast", 7)` gives 7 for both (decision 14);
- passes the integer to `kde_shade` as an `f32` (`as f32`; the file converts so at
  `:644`) without bounding it: the function treats a step beyond ±10 as ±10 (its doc
  and the table's rows at 12, `kde_shade.tsv:36`, `:66`, equal to those at 10).

### B.2 `native-theme/src/kde/colors.rs`

**Imports.** After `:4` (`use crate::Rgba;`): `use super::shade::{KdeShadeRole, kde_shade};`.

**Constants.**

- After `:22` (`const FRAME_CONTRAST_DEFAULT: f64 = 0.2;`), a blank line and:

  ```rust
  /// `kdeglobals` `[KDE] contrast` where it is unset or not an integer,
  /// KColorScheme's own default: `contrastF` is `0.1 * readEntry("contrast", 7)`
  /// (kcolorscheme.cpp:519-527 at kcolorscheme v6.31.0; docs/platform-facts.md
  /// §2.15).
  const CONTRAST_DEFAULT: i32 = 7;
  ```
- After `:369` (`const PRESSED_BUTTON_MIX: f64 = 0.3;`), a blank line and:

  ```rust
  /// How far a Qt Widgets menu's panel moves from `Window` towards `Base`:
  /// `frameBackgroundColor()`, `KColorUtils::mix(Window, Base,
  /// Metrics::Blend_Value)`, the `qreal` 0.3 (`drawPanelMenuPrimitive`,
  /// breezestyle.cpp:4708; breezehelper.cpp:200-203; breezemetrics.h:176 at breeze
  /// f0b1d75; docs/platform-facts.md §2.6).
  const MENU_PANEL_MIX: f64 = 0.3;
  ```

**Helpers.** After `frame_contrast` (`:442-450`), these twenty lines (a blank line
first):

```rust

/// KColorScheme's contrast step: `[KDE] contrast` as an integer, or
/// [`CONTRAST_DEFAULT`] where unset or not an integer. [`kde_shade`] takes a
/// step beyond ±10 as ±10, so it is not bounded here.
fn contrast(ini: &configparser::ini::Ini) -> i32 {
    ini.get("KDE", "contrast")
        .and_then(|v| v.trim().parse::<i32>().ok())
        .unwrap_or(CONTRAST_DEFAULT)
}

/// KColorScheme's `MidShade` of `window` at `contrast`, which `QPalette::Mid`
/// is (kcolorscheme.cpp:685 at 27066d47; docs/platform-facts.md §2.15), read
/// back to 8 bits: [`kde_shade`] returns Qt's 16-bit channels over 65535, and
/// [`Rgba::from_f32`] rounds each times 255 to the nearest, which is
/// `qt_div_257` of the 16-bit value (`div_257` here): a channel `s / 65535`
/// times 255 is `s / 257`, never halfway between two integers.
fn mid_shade(window: Rgba, contrast: i32) -> Rgba {
    let [r, g, b, a] = kde_shade(window.to_f32_array(), KdeShadeRole::Mid, contrast as f32);
    Rgba::from_f32(r, g, b, a)
}
```

The intra-doc links resolve in the private-items rustdoc leg (`check_release.sh:1004-1006`):
`CONTRAST_DEFAULT` and `kde_shade` are in scope (the constant above, the import), and
`Rgba::from_f32` is public. The rounding claim, checked today: `s / 257` has the
fractional part `k / 257`, at least 0.5 / 257 ≈ 0.0019 from one half, far above an
`f32`'s error at 255.

**`populate_colors`** (`:476-760`).

- Its doc, `:463-466` (four lines, "/// card's edges, the menu's hover and separator,
  the list's hover and the" to "/// list's hover) come from kdeglobals'"), becomes four
  lines:

  ```text
  /// card's edges, the menu's hover, separator and panel, the list's hover,
  /// the table's grid and the scroll bar's resting handle. The blends on the
  /// accent (the checked checkbox, the progress fill, the switch's on track and
  /// its outline, and the list's hover) come from kdeglobals'
  ```
- Menu block (`:716-728`). The comment `:716-720` (five lines) becomes seven:

  ```text
      // Menu (docs/platform-facts.md §2.6): the panel is `frameBackgroundColor()`,
      // the window moved towards the view by `Blend_Value`; the text is the
      // window text; a hovered item is the focus colour at `Blend_Value`
      // lettered in the window text; the separator is `separatorColor()`, the
      // window moved towards its text by the frame contrast (breezestyle.cpp:4708,
      // :5984, :6021-6029, :6155; breezehelper.cpp:200-203, :329-332 at breeze
      // f0b1d75).
  ```
  After `:728` (the separator's statement) add:

  ```rust
  variant.menu.background_color = window_bg
      .zip(view_bg)
      .map(|(window, base)| mix(window, base, MENU_PANEL_MIX));
  ```
- List block (`:730-746`). After `:736` (`variant.list.alternate_row_background = …`)
  add:

  ```rust
  // A table's grid is `QPalette::Mid`, the window's `MidShade` at the
  // scheme's contrast (qcommonstyle.cpp:5126-5128 at qtbase v6.11.0;
  // kcolorscheme.cpp:685 at 27066d47; docs/platform-facts.md §2.15).
  variant.list.grid_color = window_bg.map(|window| mid_shade(window, contrast(ini)));
  ```

Missing inputs give `None`, as every derivation of the reader (`Option::zip`), and the
preset's value stands.

### B.3 The values under Breeze

| Field | Light | Dark | How computed today |
|---|---|---|---|
| `list.grid_color`, contrast 4 | #c4c8cc | #1c1f21 | the table's rows `kde_shade.tsv:16` (`#eff0f1 mid 4`) and `:46` (`#202326 mid 4`), each channel × 255 rounded; equal to the measurement (`docs/evidence/breeze-offscreen/2026-11-13/out_light.txt:132`, `out_dark.txt:132`) |
| `list.grid_color`, contrast 7 (unset) | #bdc1c6 | #1c1e21 | rows `:26`, `:56`; equal to the first rationale's §5.6 |
| `list.grid_color`, contrast 0 / −3 / 10 / 12 | #ced1d4 / #d5d7da / #b5babf / #b5babf | — | rows `:21`, `:41`, `:31`, `:36` |
| `menu.background_color` | #f4f5f5 | #1c1f22 | `mix_qt`'s arithmetic (`colors.rs:242-256`, then `QtColor::to_rgba`) re-done in Python with numpy `float32` for the `f32` steps, inputs (239,240,241)/(255,255,255) and (32,35,38)/(20,22,24), bias 0.3 and `f64::from(0.3f32)` alike; equal to the measurement (`out_light.txt:73`, `out_dark.txt:73`) |

### B.4 `native-theme/src/presets/kde-breeze.toml`

Comment form as the file's, but with no line number (the maintainer's S6,
`docs/todo_v0.7.1_batch-speed-rationale.md`): a value citation's first line starts
with `# platform-facts.md (§x.y):`, on the line above the value, the form
`scripts/check_preset_citations.py` reads once the GPUI plan's unit LN has landed
(the GPUI specification's Part I, I.3), before this step's units.

**Light.**

1. `[light.menu]`, after `:331` (`separator_color = "#c6c8c9"`):

   ```toml
   # platform-facts.md (§2.6): frameBackgroundColor(), mix(Window #eff0f1, Base #ffffff, Blend_Value 0.3), a Qt
   # Widgets menu's panel (breezestyle.cpp:4708, breezehelper.cpp:200-203, breezemetrics.h:176 at breeze f0b1d75; measured)
   background_color = "#f4f5f5"
   ```
2. `[light.list]`, after `:352` (`alternate_row_background = "#f7f7f7"`):

   ```toml
   # platform-facts.md (§2.15): QPalette::Mid, KColorScheme's MidShade of [Colors:Window] BackgroundNormal 239,240,241
   # (BreezeLight.colors:129, breeze v6.7.5) at [KDE] contrast 4 (BreezeLight.colors:195 at breeze f0b1d75), a QTableView's
   # grid (qcommonstyle.cpp:5126-5128 at qtbase v6.11.0; kcolorscheme.cpp:685 at 27066d47; measured)
   grid_color = "#c4c8cc"
   ```

**Dark.**

3. `[dark.menu]`, after `:919` (`separator_color = "#4c4e51"`): item 1 with
   `mix(Window #202326, Base #141618, Blend_Value 0.3)` and `background_color = "#1c1f22"`.
4. `[dark.list]`, after `:940` (`alternate_row_background = "#1d1f22"`): item 2 with
   `[Colors:Window] BackgroundNormal 32,35,38 (BreezeDark.colors:130, breeze v6.7.5)`,
   `(BreezeDark.colors:202 at breeze f0b1d75)` and `grid_color = "#1c1f21"`.

The Window and View line numbers at v6.7.5 are the file's own (`kde-breeze.toml:19`,
`:23`, `:604`, `:608`); the `[KDE] contrast` lines are the facts row's, at f0b1d75 (not
re-read here: the Breeze sources are not in the tree). The other files' citations
of this file's lines are left as they are and may drift (S6).

### B.5 Tests that change, `native-theme/src/kde/mod.rs`

| Test or item | Today | After |
|---|---|---|
| `struct Scheme` (`:1174-1187`) and `Scheme::kdeglobals` (`:1189-1213`) | `frame_contrast: Option<&str>`, written as `[KDE]\nframeContrast=…` | a field `contrast: Option<&'static str>`; `kdeglobals` writes one `[KDE]` group where either is `Some`, `contrast=` then `frameContrast=` under it; the struct's doc ends "…with an optional `[Colors:Header]` group and `[KDE]` `contrast` and `frameContrast`." |
| `BREEZE_LIGHT_SCHEME`, `BREEZE_DARK_SCHEME` (`:1215-1244`) | no contrast | `contrast: Some("4")`; each doc gains a new last sentence, "`[KDE] contrast=4` (BreezeLight.colors:195 at breeze f0b1d75; docs/platform-facts.md §2.15)." (Dark: the same with `BreezeDark.colors:202`) |
| `derived_leaves`, `resolved_leaves` (`:1246-1372`) | 29 leaves | 31, and both return types `[(&'static str, Option<Rgba>); 29]` (`:1248`, `:1309`) become `; 31]`: append `("list.grid_color", mode.list.grid_color)` and `("menu.background_color", mode.menu.background_color)`, resolved `Some(theme.list.grid_color)`, `Some(theme.menu.background_color)`; doc "The thirty-one colour leaves …" |
| `the_derived_colours_are_the_presets_under_breeze` (`:1397-1414`) | 29 leaves equal and stated | passes with B.2 and B.4 (B.3's values equal the preset's); no edit |
| `the_window_set_moves_the_popover_edge` (`:1482-1522`) | six / eight leaves | append `"list.grid_color", "menu.background_color"` to both lists; doc: "The Window set moves the popover's edge, the scroll-bar handles, the menu's hover text, separator and panel and the table's grid, and the toolbar only where the scheme has no Header group to take it from." |
| `the_view_set_moves_the_card_edge` (`:1524-1532`) | `["card.border.color"]` | rename `the_view_set_moves_the_card_edge_and_the_menu_panel`; `["card.border.color", "menu.background_color"]` |
| `a_colour_scheme_keeps_the_stated_edge_geometry` (`:1657-1687`) | its `Scheme` literal (`:1661-1670`) names every field and has no `..BREEZE_LIGHT_SCHEME` | add `contrast: None,` after `frame_contrast: Some("0.5"),` (the test compares border geometry, which the contrast does not move); without it the test module does not compile |
| new, after `the_frame_contrast_moves_the_edges` (`:1563-1581`): `the_contrast_moves_the_table_grid` | — | `Scheme { contrast: Some("0"), ..BREEZE_LIGHT_SCHEME }` and `Scheme { contrast: None, ..BREEZE_LIGHT_SCHEME }` each move `["list.grid_color"]` alone; doc "`[KDE] contrast` moves the table's grid and nothing else; without it the grid is KColorScheme's default step's." |

The struct's literals, listed today (`grep -n 'Scheme {'` over `kde/mod.rs`): seventeen,
of which fourteen spread `..BREEZE_LIGHT_SCHEME` (`:1422`, `:1442`, `:1471`, `:1488`,
`:1503`, `:1526`, `:1539`, `:1552`, `:1565`, `:1591`, `:1596`, `:1622`) or
`..without_header` (`:1636`, `:1643`) and need no edit, two are the constants above,
and one, `:1661`, names every field. No other struct gains a field in this step.
`a_header_group_takes_a_missing_key_from_the_window` (`:1615-1650`) appends
`[Colors:Header]` after `kdeglobals()`'s text, which now ends in a `[KDE]` group: a
new group header, so the append reads as before.

Not affected (checked): `an_accent_moves_…`, `the_button_set_moves_…`,
`the_header_set_moves_the_toolbar`, `the_view_decorations_move_…`,
`the_frame_contrast_moves_the_edges` (none of their inputs is a new leaf's),
`scheme_colours_reach_the_combo_box_menu_and_popover` (`:1159-1172`, asserts no new
leaf), `native-theme/tests/reader_kde.rs` (no grid or menu fill assertion; its
fixtures state no `contrast`).

### B.6 Tests, `native-theme/src/kde/colors.rs`

New, after `test_missing_groups_leave_the_new_fields_to_the_preset` (`:1588-1605`).
Each expected colour is asserted with `assert_eq!(…, Some(Rgba::rgb(…)))`, and each
`is_none()` check of test 5 is written in an Edit of its own: the project's
`no-invented-values.sh` hook refuses an Edit whose text holds both
`= Some(Rgba::rgb(0…` and `is_none()`.

1. `test_table_grid_is_qts_mid_shade`: for `("239,240,241", Rgba::rgb(0xc4, 0xc8, 0xcc))`
   and `("32,35,38", Rgba::rgb(0x1c, 0x1f, 0x21))`, the content
   `"[Colors:Window]\nBackgroundNormal={window}\n\n[KDE]\ncontrast=4\n"` gives that
   `list.grid_color`.
2. `test_table_grid_follows_the_contrast`: window 239,240,241; `contrast=0` gives
   (0xce,0xd1,0xd4), `contrast=-3` (0xd5,0xd7,0xda), `contrast=10` (0xb5,0xba,0xbf),
   `contrast=12` the same as 10 ("a step beyond 10 acts as 10"), no `[KDE]` group
   (0xbd,0xc1,0xc6) ("KColorScheme's default, 7"), `contrast=high` and `contrast=4.5`
   the same as none ("not an integer").
3. `test_menu_panel_is_breezes`: `[Colors:Window] BackgroundNormal` and
   `[Colors:View] BackgroundNormal` 239,240,241 / 255,255,255 give
   `Rgba::rgb(0xf4, 0xf5, 0xf5)`; 32,35,38 / 20,22,24 give `Rgba::rgb(0x1c, 0x1f, 0x22)`.
4. `a_custom_scheme_moves_the_menu_panel_and_the_grid`: window 230,220,200, view
   250,240,230, no `[KDE]`: `menu.background_color == Some(mix(window, view,
   MENU_PANEL_MIX))`, `list.grid_color == Some(mid_shade(window, CONTRAST_DEFAULT))`,
   and each `assert_ne!` against test 3's and test 1's Breeze Light values.
5. `test_missing_groups_leave_the_panel_and_the_grid_to_the_preset`: only
   `[Colors:View] BackgroundNormal`: both `None`; only `[Colors:Window]
   BackgroundNormal`: `list.grid_color` is `Some`, `menu.background_color` `None`.

Changed: `test_full_mapping_completeness` (`:1713-1861`) asserts
`v.list.grid_color.is_some()` and `v.menu.background_color.is_some()` (the
`BREEZE_DARK` fixture has both inputs, `:770-795`).

### B.7 The watcher and the source list

No change. `[KDE] contrast` is a key of `kdeglobals`, which the reader lists under
"# Sources" (`kde/mod.rs:3-24`), the watcher observes (`watch/kde.rs:12`,
`WATCHED_FILES`), and `SOURCE_NOTIFICATIONS` ties (`watch/mod.rs:568-569`); the table
and its tests (`every_listed_source_has_a_table_row`, `:717`;
`the_kde_watcher_observes_every_kde_source`, `:741`) tie files, not keys. A change of
the colour scheme rewrites `kdeglobals`, which already fires the watcher.

## Part C — Facts, rules, registry

### C.1 `docs/platform-facts.md`

Columns: `Property | macOS | Windows | KDE | GNOME | Material` (headers `:1251`,
`:1419`, `:1452`); the KDE cell is the fourth. Each edit replaces that cell's text and
nothing else on the line; no line is added.

**`:1430`, `grid_color` (§2.15).** The cell as it stands is quoted in full in the first
specification (`docs/archive/todo_v0.7.1_kde-hand-overrides-spec.md:2022`) and was read
again today unchanged. After the measured pair and its sources (`— Breeze #c4c8cc,
Breeze Dark #1c1f21 **(measured)** ([out_light.txt:132, :133, :135](…),
[out_dark.txt:132, :133, :135](…))`) and before `; a list or tree view draws no line
between rows`, insert:

```text
; where a scheme states no `[KDE] contrast`, KColorScheme takes the step 7 ([kcolorscheme.cpp:519-527](https://invent.kde.org/frameworks/kcolorscheme/-/blob/27066d471c93629efee8459d9f69490caa92b7c4/src/kcolorscheme.cpp#L519-L527)), at which the Mid shade of Breeze's window colour is #bdc1c6 and of Breeze Dark's #1c1e21 (computed)
```

So the cell gives, in this order: Breeze's step 4 and the measured pair it yields,
then the default step and the computed pair it would yield for the same window
colours.

The link's lines are kcolorscheme 6.31.0's (`F/kde/tokens.toml:11351`); that they are
`contrastF` at `27066d47`, the commit the cell's other links name, is **not verified**
(no network): the cell's `:526` at that commit lies inside them. The implementer opens
the link and, if the lines differ, cites the ones that hold `contrastF`.

**`:1253`, `background_color` (§2.6).** Append to the cell, after its last
`([out_light.txt:74](…), [out_dark.txt:74](…))`:

```text
 · Qt Quick (qqc2-desktop-style's `Menu`): ❓ not read
```

**`:1454`, `background_color` (§2.16 Popover / Dropdown).** The KDE cell today:

```text
← `defaults.background_color`: qqc2-desktop-style's Popup fills `Kirigami.Theme.backgroundColor` ([Popup.qml:46-48](https://invent.kde.org/frameworks/qqc2-desktop-style/-/blob/v6.31.0/org.kde.desktop/Popup.qml#L46-L48)) and sets no colour set, so it inherits one, the Window set where nothing above it sets another (Kirigami's default, [platformtheme.cpp:115, :308](https://invent.kde.org/frameworks/kirigami/-/blob/v6.31.0/src/platform/platformtheme.cpp#L115))
```

Append:

```text
; a Qt Widgets combo box's popup, a `QListView` (`SH_ComboBox_Popup` 0), shows its rows on the Window colour, Breeze #eff0f1, Breeze Dark #202326 **(measured on the popup's view)** ([combo_light.txt:1-4](evidence/breeze-offscreen/2026-11-14/combo_light.txt#L1-L4), [combo_dark.txt:1-4](evidence/breeze-offscreen/2026-11-14/combo_dark.txt#L1-L4)); the popup's own frame was not measured
```

(read today: line 1 of each file names the style, the Window colour and the popup's
view; lines 2-4 are rows filled #b9dcee / #284c61 for the current row and #eff0f1 /
#202326 for the others).

### C.2 `docs/inheritance-rules.toml`

Two `why_wrong` strings change in place, one line each. The
`[uniform]` rules and their `# see wrong_safety_nets` marks (`:194`, `:245`) stay: the
inherited value is still wrong on KDE, which now states its own.

- `:539`, `[wrong_safety_nets."menu.background_color"]`. Today:
  `why_wrong     = "macOS vibrancy and Win COLOR_MENU differ; KDE's Qt Widgets menu panel is mix(Window, Base, 0.3), #f4f5f5 / #1c1f22 under Breeze, four to five levels from the window colour, which kde-breeze keeps until the field's other readers allow (platform-facts.md:1253, §2.6)"`.
  New: `why_wrong     = "macOS vibrancy and Win COLOR_MENU differ; KDE's Qt Widgets menu panel is mix(Window, Base, 0.3), #f4f5f5 / #1c1f22 under Breeze, four to five levels from the window colour, which kde-breeze states and the KDE reader derives (platform-facts.md §2.6)"`.
- `:659`, `[wrong_safety_nets."list.grid_color"]`. Today ends
  `…far brighter in dark mode; kde-breeze keeps the inherited value until the field's other readers allow (platform-facts.md:1430, §2.15)"`.
  New ending: `…far brighter in dark mode; kde-breeze states it and the KDE reader derives it from the scheme's contrast (platform-facts.md §2.15)"`.

### C.3 `docs/property-registry.toml`

No edit: no field is added (`list.grid_color`, `:385`; `menu.background_color` under
`[menu]`, `:254`), and the registry states names and types only.

### C.4 What the five documentation checkers require

| Checker (run from the worktree root) | Reads | This step |
|---|---|---|
| `scripts/check_platform_facts_rows.py` | registry fields; each facts row's first cell and the `### 2.x` headings | no first cell, heading or field changes: no finding |
| `scripts/check_section_refs.py` | `§N.M` in the registry, the rules, `docs/showcase-elements.toml`, `docs/new-os-version-guide.md` | the two strings cite §2.6 and §2.15, which exist (`:1250`, `:1417`) |
| `scripts/check_preset_citations.py`, as the GPUI plan's unit LN leaves it (the GPUI specification's I.3) | every facts citation in a preset comment, in either form, read and counted; it compares none with the facts file and fails only on a preset it cannot read | B.4's four comments, in the new form, are read: no finding |
| `scripts/check_wrong_safety_nets.py` | the `[uniform]` marks and the entries name each other; `inherits_from` is the rule's source; both strings present | keys, sources and marks unchanged |
| `scripts/check_todo_citations.py` | `docs/todo.md`: a preset key path resolves in the preset named before it in its entry; a preset line number only with `at <sha>` | Part G's lines cite `kde-breeze.toml` `[light.menu] background_color`, `[dark.menu] background_color`, `[light.list] grid_color`, `[dark.list] grid_color`, which exist once B.4 lands: G's backlog edits land with or after it; no line number |

The Rust gates that read the facts file: `native-theme/src/presets/documented_sizes.rs`
(sizes only; after LN it finds rows by section and name) and
`native-theme/tests/platform_facts_xref.rs` (font, radii, line height, accent, icon
set): neither reads these rows.

## Part D — gpui connector

### D.1 `chart_grid`'s source, `connectors/native-theme-gpui/src/colors.rs`

The line count from 1 to 718 stays (header, *Lines that must not move*).

- `:269`, `    assign_list_table(&mut tc, &c, is_dark);` becomes
  `    assign_list_table(&mut tc, &c, resolved);`.
- `:414`, `fn assign_list_table(tc: &mut ThemeColor, c: &ResolvedColors, _is_dark: bool) {`
  becomes `fn assign_list_table(tc: &mut ThemeColor, c: &ResolvedColors, resolved: &ResolvedTheme) {`
  (89 characters; `ResolvedTheme` is imported at `:13`).
- `:441-446` (six lines) today:

  ```rust
      // The grid of a chart (`chart/mod.rs:493`, `chart/bar_chart.rs:878`,
      // `chart/radar_chart.rs:507`): the model's grid-line colour, which
      // platform-facts §2.15 states as macOS `gridColor` and Material's
      // `outline-variant`, and which elsewhere inherits `defaults.border.color`
      // (`docs/inheritance-rules.toml:245`).
      tc.chart_grid = c.list_grid;
  ```
  become (six lines):

  ```rust
      // The grid of a chart (`chart/mod.rs:493`, `chart/bar_chart.rs:878`,
      // `chart/radar_chart.rs:507`): a set of separating lines, so the model's
      // separator colour (issue #4's D2); `list.grid_color` is a table's grid,
      // Qt's `Mid` on KDE, which would all but vanish under a chart in dark
      // mode, and stays `table_row_border`'s (above).
      tc.chart_grid = rgba_to_hsla(resolved.separator.line_color);
  ```
  `:446` still names `chart_grid`, which the claim at `info/theme_map.rs:1244` needs.
- The test `chart_grid_is_the_list_grid_colour` (doc `:831-833`, the test `:834-854`): its three doc lines
  become "/// K4, as issue #4's D2 changed it: a chart's grid is the separator" /
  "/// colour, on every preset in both variants; the table's grid," / "/// `table_row_border`,
  keeps `list.grid_color` (`contract.rs`'s rows)."; the name
  `chart_grid_is_the_separator_colour`; `:848` `rgba_to_hsla(resolved.separator.line_color),`;
  `:849` `"{} {mode:?}: chart_grid is not separator.line_color",`. Same line count.

`config.rs`'s `the_config_carries_the_native_chart_grid` (`:367-383`) compares the
config with the direct path's `chart_grid`, whatever its source: no edit.

### D.2 `connectors/native-theme-gpui/src/contract.rs`

- The row at `:255-259`: `native: |r| r.list.grid_color,` (`:257`) becomes
  `native: |r| r.separator.line_color,`, and a comment line goes above its `Row {`:
  ``    // A chart's grid: the separator colour (issue #4's D2); a table's is `table_row_border`'s, below.``
  The `table_row_border` row (`:422-431`) stays.
- After `:1866` (the second `IDLE_TAB_EXCEPTIONS`), a blank line and:

  ```rust
  /// gpui-component fills a menu with the `popover` token
  /// (`menu/popup_menu.rs:1610-1611`), the window's colour on KDE, where
  /// Breeze's menu panel, `menu.background_color`, is `mix(Window, Base, 0.3)`
  /// (issue #4's D3): the hover tint lies on a surface a few levels from the
  /// platform's, and gpui-component has no menu surface token.
  const MENU_PANEL_SURFACE: &str = "gpui-component paints a menu on the popover \
       token, the window colour on KDE, where Breeze's panel is mix(Window, Base, 0.3)";
  ```
- The pair "hovered menu row" (`:1987-1998`): `exceptions: &[],` (`:1997`) becomes
  these four lines, as rustfmt leaves them (the project has no `rustfmt.toml`, so its
  default width of 100 holds, and the one-line form would be 105 characters):

  ```rust
          exceptions: &[
              ("kde-breeze/light", MENU_PANEL_SURFACE),
              ("kde-breeze/dark", MENU_PANEL_SURFACE),
          ],
  ```
  (labels as `Combination::label` writes them, `:1356-1359`). With the comment line of
  the `chart_grid` row (unit 2) and the constant's eight lines, the file grows by 12
  lines after `:254`.

Ratios behind it, computed today in Python (WCAG luminance, straight-alpha source-over,
as `pair_ratio`, `:2409-2412`; gpui's `Hsla` round trips not reproduced, so the last
digit is **not verified**): light #232629 on #3daee94d over the emitted #eff0f1 10.54,
over the native #f4f5f5 10.90; dark #fcfcfc over #202326 8.80, over #1c1f22 9.18. On
every other combination `menu.background_color` does not change, so the pair's result
there does not either.

Not affected: `UNREAD_COLOURS`' `menu.background_color` (`:1282-1289`, "a `PopupMenu`
paints `popover_style`") stays true; `separator.line_color` already reaches a sink
(the connector's `widgets::Separator`, not named in `UNREAD_COLOURS`) and now one more,
and `list.grid_color` keeps `table_row_border`, so
`every_colour_leaf_reaches_a_sink_or_is_named` (`:2933`) needs no entry.

### D.3 The showcase, `connectors/native-theme-gpui/examples/showcase-gpui/`

- `info/theme_map.rs:1246`, `            |i| i.config("model", "list.grid_color"),` (the
  `ThemeToken::ChartGrid` arm, `:1239-1247`) becomes
  `            |i| i.config("model", "separator.line_color"),`. `:998`
  (`TableRowBorder`, `list.grid_color`) stays.
- `info/charts.rs` (`:108-111`, `:163-166`, `:221-224`, `:405-408`): the chart panels
  claim the token `chart_grid` and the upstream line that paints it, not a model leaf:
  no edit. `info/leaves.rs:1352-1356` and `info/data.rs:486-498` route `list.grid_color`
  to the table: unchanged. The showcase's own popups fill from `menu.background_color`
  (`demo.rs:389`, `:646`, `:8261`) and so take Breeze's panel on KDE: no edit; their
  test (`tests.rs:9878-9975`) runs under adwaita.

### D.4 `chart_grid` per bundled preset

Computed today with a Python script from the preset files by
`docs/inheritance-rules.toml`'s rules (`"list.grid_color"` and `"separator.line_color"`
← `defaults.border.color`, `:245`, `:253`), not by the crate's resolver (no cargo run).
"Before" is `list.grid_color`, "after" `separator.line_color`; the contrast is on
`window.background_color`, a translucent grid composited first.

| Preset | Before, light / dark | After, light / dark | Contrast on the window, before → after |
|---|---|---|---|
| kde-breeze | #bcc0bf / #4d545b | the same (both inherit the border) | unchanged, 1.61 / 2.06 |
| adwaita | #dcdcde / #434347 | #d5d5d5 / #4a4a4e | 1.31 → 1.41 / 1.61 → 1.80 |
| windows-11 | #0000000f / #00000019 | the same | unchanged |
| macos-monterey | #e6e6e6 / #1a1a1a | #00000019 / #ffffff19 | 1.06 → 1.24 / 1.36 → 1.36 |
| macos-tahoe | #e6e6e6 / #1a1a1a | #00000019 / #ffffff19 | 1.25 → 1.25 / 1.04 → 1.34 |
| material-3 | #cac4d0 / #49454f | the same (both stated) | unchanged |
| ios-13 | #c6c6c8 / #38383a | the same (the separator states the border's value) | unchanged |
| the ten colour-scheme presets | the border colour | the same (each states its separator as its border colour) | unchanged |

`table_row_border` is `list.grid_color` before and after; it changes on kde-breeze only,
from #bcc0bf / #4d545b to #c4c8cc / #1c1f21.

## Part E — iced connector and its showcase

### E.1 The two styles, field by field

`styles::menu` (`connectors/native-theme-iced/src/styles.rs:1233-1274`), an
`iced_widget::overlay::menu::Style` (`iced_widget-0.14.2/src/overlay/menu.rs:597-611`):

| Field | Today | After |
|---|---|---|
| `background` | `menu.background_color` | the same (D3 (i)) |
| `border.color` | `menu.border.color` | `popover.border.color` |
| `border.width` | `menu.border.line_width` | `popover.border.line_width` |
| `border.radius` | `menu.border.corner_radius` | the same |
| `text_color` | `menu.font.color` | the same |
| `selected_text_color` | `menu.hover_text_color` | the same |
| `selected_background` | `menu.hover_background` | the same |
| `shadow` | `overlay::menu::default(theme).shadow`, `Shadow::default()` (`:659`) | the same |

`styles::aw::menu` (`src/styles/aw.rs:113-160`), an `iced_aw::style::menu_bar::Style`
(`iced_aw-0.14.1/src/style/menu_bar.rs:7-26`):

| Field | Today | After |
|---|---|---|
| `bar_background` | `menu.background_color` | `window.background_color` |
| `bar_border` | `menu.border` | `Border::default()` |
| `bar_shadow` | `menu_bar::primary(..).bar_shadow` | the same |
| `menu_background` | `menu.background_color` | the same |
| `menu_border` | `menu.border` | `popover.border` (colour, width, radius) |
| `menu_shadow` | `menu_bar::primary(..).menu_shadow` | that where `popover.border.shadow_enabled`, else `Shadow::default()` |
| `path` | `menu.hover_background` | the same |
| `path_border` | `menu_bar::primary(..).path_border` | the same |

The new `styles::aw::menu` returns, field for field, what the showcase's
`menu_bar_style` (`showcase-iced.rs:6518-6550`) returns today (read side by side).

### E.2 `src/styles.rs`, `menu`

- Doc. Lines `:1233-1244` (twelve: from "/// The platform's own menu, for
  `.menu_style(..)` on a `PickList` or a" to "/// composited: a translucent fill (KDE,
  Windows) lays itself over the panel.") are replaced by the nineteen lines below, the
  "Replaces" paragraph (`:1236-1239`) kept as it is inside them; `:1245` (`///`) and the
  `shadow` paragraph (`:1246-1247`) stay:

  ```text
  /// The platform's own drop-down, for `.menu_style(..)` on a `PickList` or a
  /// `ComboBox`.
  ///
  /// (the "Replaces …" paragraph as it is)
  ///
  /// A drop-down is framed as a popup, in `popover.border`'s colour and width
  /// (platform-facts §2.16, "Popover / Dropdown"). Its panel and rows are the
  /// menu's: the panel `menu.background_color`, the label `menu.font.color`,
  /// the selected row's label and fill `menu.hover_text_color` and
  /// `menu.hover_background`, which each theme states against the menu's
  /// fill. Its corners are `menu.border.corner_radius`: iced rounds the
  /// highlighted row with the panel's radius (`overlay/menu.rs:550`), and the
  /// popover's would round every hovered row to a pill. A selected row is a
  /// row highlight, painted over a panel the widget also paints, so it is
  /// emitted as given rather than composited: a translucent fill (KDE,
  /// Windows) lays itself over the panel.
  ```
- Body (`:1252-1274`). After `:1254` (`let m = &resolved.menu;`) insert
  `    let p = &resolved.popover;`; `:1256`
  (`    let background = to_color(m.background_color);`) stays; `:1261`
  (`    let border = border_of(&m.border);`) becomes the five lines

  ```rust
      let border = Border {
          color: to_color(p.border.color),
          width: p.border.line_width,
          radius: Radius::new(m.border.corner_radius),
      };
  ```
  `Border` and `Radius` are imported (`:88`, `:90`). The file grows by 12 lines from
  `:1233` on (7 in the doc, 5 in the body); the radius line, today's `:1261`, is
  `:1272` after.

iced insets the highlighted row by the border's width on the left and right only
(`overlay/menu.rs:546-547`), so where the drop-down gains a line, its highlight is that
much narrower. It does not inset it at the top or the bottom: the overlay paints the
panel and its frame first, then the list over it, whose rows start at the panel's
top edge and end at its bottom edge (`overlay/menu.rs:312-330`, the frame then the
list in the same bounds; `:535-540`, row `i` at `option_height * i` from the top),
so a highlighted first or last row paints over the frame's top or bottom line
(read from the code, not rendered). The connector's style cannot change this: the
`Style` has no inset for the rows. G.5 records it.

### E.3 `src/styles/aw.rs`, `menu`

- `:37`, `use iced_core::{Background, Color};` becomes
  `use iced_core::{Background, Border, Color, Shadow};`.
- Doc, `:119-135` today (from "The bar and the menus it opens are the same surface
  here" to "…the consumer's own widgets."). New:

  ```text
  /// The bar lies on the window: `window.background_color`, with no frame
  /// (`Border::default()`): the model has no menu-bar fill, and `MenuTheme`'s
  /// border is no bar's. A menu is the menu's panel, `menu.background_color`,
  /// framed by the popup's border, `popover.border` (its colour, width and
  /// radius): the platforms frame a menu as they frame every popup
  /// (platform-facts §2.6: the popup border is §2.16's), and `MenuTheme`'s
  /// border states no line of its own. `path` -- the highlight under the item
  /// the pointer is on -- is `menu.hover_background`, emitted as given: it is
  /// a row highlight painted over a panel the widget also paints, so the
  /// platform's layering happens by itself.
  ///
  /// Three fields have no native source and come from
  /// `menu_bar::primary(theme, status)`: the bar's shadow and the menus'
  /// shadow, because the model carries `defaults.shadow_color` but no offset or
  /// blur -- a menu casts it only where `popover.border.shadow_enabled` states
  /// one --, and `path_border`, because the model has no border round a
  /// highlighted row.
  ///
  /// (the paragraph "`menu.hover_text_color`, `menu.font.color`, … the
  /// consumer's own widgets." as it is)
  ```
  The doc grows from 17 lines to 22 (the kept third paragraph is four lines,
  `:132-135`).
- Body, `:140-161` (22 lines, from `    use iced_aw::style::menu_bar::Style;` to the
  closure's `    }`), becomes these 25:

  ```rust
      use iced_aw::style::menu_bar::Style;

      let bar = to_color(resolved.window.background_color);
      let panel = to_color(resolved.menu.background_color);
      let highlight = to_color(resolved.menu.hover_background);
      let frame = border_of(&resolved.popover.border);
      let shadow = resolved.popover.border.shadow_enabled;

      move |theme, status| {
          let iced = iced_aw::style::menu_bar::primary(theme, status);
          Style {
              bar_background: Background::Color(bar),
              bar_border: Border::default(),
              bar_shadow: iced.bar_shadow,
              menu_background: Background::Color(panel),
              menu_border: frame,
              menu_shadow: if shadow {
                  iced.menu_shadow
              } else {
                  Shadow::default()
              },
              path: Background::Color(highlight),
              path_border: iced.path_border,
          }
      }
  ```
  The file grows by 8 lines from `:119` on.

### E.4 The contract

**`src/contract/rows.rs`.**

- `MENU_ROWS` (`:1512-1540`) does not change: `styles::menu.background` (`:1517`)
  stays `menu.background_color`, as the text and selected rows stay the menu's.
- `MENU_BORDER_ROWS` (`:1542-1552`): native `NativeBorder { color: to_color(r.popover.border.color), width: r.popover.border.line_width, radius: r.menu.border.corner_radius }`;
  a doc line above: `/// A drop-down is framed in the popover's line, at the menu's radius (issue #4's D3).`
- `AW_MENU_ROWS`' doc (`:2597-2598`): "the bar and the menus share the one panel the
  model states, and the open path is the menu's hover fill" → "the bar lies on the
  window, a menu is the menu's panel, and the open path is the menu's hover fill".
  `styles::aw::menu.bar_background`'s native (`:2604`) → `to_color(r.window.background_color)`.
- `AW_MENU_BORDER_ROWS` (`:2621-2638`): its doc (`:2621-2623`) → "/// A menu's frame, the popover's
  border. The bar has none (`Border::default()`), and `path_border` has no native
  source: both are in `DERIVED`."; the `styles::aw::menu.bar` row is removed;
  `native_aw_menu_border` (`:2640-2648`) reads `r.popover.border.color`,
  `.line_width`, `.corner_radius`, its doc "/// The popup's border, `popover.border`,
  which frames an `iced_aw` menu."

**`src/contract/derived.rs`.**

- `DERIVED`: after `styles::aw::menu.menu_shadow` (`:834-838`), six entries, each
  `#[cfg(feature = "iced_aw")]`, for `styles::aw::menu.bar.border.color`, `.width`,
  `.radius.top_left`, `.radius.top_right`, `.radius.bottom_right`,
  `.radius.bottom_left`: the first "connector: `Border::default()`, no frame -- the bar
  lies on the window, and the model states no menu-bar border (`MenuTheme`'s is no
  bar's)", the others "connector: `Border::default()` -- the same". The
  `menu_shadow` entry's text (`:837`) → "iced_aw default: menu_bar::primary(..).menu_shadow
  where popover.border.shadow_enabled, Shadow::default() where not -- the model has a
  shadow color but no offset or blur". The `path_border` entry's text
  (`styles::aw::menu.path.border.color`, `:842-844`: "iced_aw default:
  menu_bar::primary(..).path_border -- the model's one menu border is the panel's,
  and drawing it around a highlighted row would outline something the platform does
  not") → "iced_aw default: menu_bar::primary(..).path_border -- the model has no
  border round a highlighted row", as E.3 rewrites the same sentence of the doc; the
  five entries after it say "the same" and stay.
- `Unread::Ruled`'s doc (`:1289-1291`): "iced has a receiver, and a ruling of the M5
  review fills it from another leaf" → "iced has a receiver, and a ruling (the M5
  review's, or a rationale's decision) fills it from another leaf".
- `UNREAD_COLOURS`: after the `menu.separator_color` entry (`:1487-1495`):

  ```rust
      UnreadColour {
          leaf: "menu.border.color",
          why: Unread::Ruled(
              "both menu styles frame their panel in `popover.border`, the \
               popup's frame (`src/styles.rs` and `src/styles/aw.rs`, `menu`), \
               into iced's `menu::Style::border` \
               (`iced_widget-0.14.2/src/overlay/menu.rs:602`) and `iced_aw`'s \
               `menu_border` (`iced_aw-0.14.1/src/style/menu_bar.rs:18`): \
               `MenuTheme`'s border has no kind of its own and states no line \
               (`MenuTheme` in `native-theme/src/model/widgets/mod.rs`), and the \
               facts give a menu's frame to the popover \
               (docs/platform-facts.md §2.6); \
               docs/issues/0004_kde-open-items-rationale.md D3",
          ),
      },
  ```
  The two upstream lines were read 2026-11-15 in Cargo's registry: `pub border: Border`
  of iced's menu `Style` and `pub menu_border: Border` of `iced_aw`'s menu bar `Style`.
  They give the reason the `file.rs:line` that `Unread::Ruled` needs
  (`connectors/native-theme-iced/src/contract.rs`, the test's `evidence.contains(".rs:")`);
  the connector's own lines are named by file and function, not by number (the
  maintainer's S6, `docs/todo_v0.7.1_batch-speed-rationale.md`).
- The `popover` entry (`:1532-1539`) becomes two leaf entries:

  ```rust
      UnreadColour {
          leaf: "popover.background_color",
          why: Unread::Ruled(
              "a drop-down, the one surface the popover's fill could reach, is \
               filled from `menu.background_color` (`src/styles.rs`, `menu`), \
               the background iced fills its list with \
               (`iced_widget-0.14.2/src/overlay/menu.rs:316-324`): its \
               rows' highlight and text are the menu's, stated against the menu's \
               fill (docs/issues/0004_kde-open-items-rationale.md D3 (i))",
          ),
      },
      UnreadColour {
          leaf: "popover.font.color",
          why: Unread::NoReceiver(
              "iced 0.14 has no popover, nor has `iced_aw` 0.14.1 (its \
               `context_menu::Style.background` is the backdrop of a modal, \
               `style/context_menu.rs:8-14`); a drop-down, which takes only the \
               popover's line, is a menu's list lettered in the menu's text \
               (`iced_widget-0.14.2/src/overlay/menu.rs:598-611`)",
          ),
      },
  ```
  (`let background = to_color(m.background_color);` is in `src/styles.rs`'s `menu`,
  `:1256` today; iced's `:316-324`, read 2026-11-15, is the `fill_quad` that paints
  the menu's bounds with `style.border` and `style.background`.)

**`src/contract/pairs.rs`**: no change. `MENU_PAIRS` (`:476-526`, doc and `cfg` included) compare the
drop-down's label and highlight with the menu's own fields over the menu's fill, on
both sides, before and after.

**`contract-leaves.toml`** is generated: after E.2-E.4, regenerate it with
`cargo test -p native-theme-iced --all-features --lib -- --ignored regenerate_contract_leaves`
(`src/contract.rs:5225-5226`) and commit it. Expected differences (read against
today's `:627-685`, `:1054-1057`): `menu.background_color` moves
`styles::aw::menu.menu_background` and `styles::menu.background`, no longer
`styles::aw::menu.bar_background`; `menu.border.color` is named (`UNREAD_COLOURS`,
`ruled`); `popover.background_color` is named by its own entry (`ruled`);
`popover.border.color` moves `styles::aw::menu.menu.border` and `styles::menu.border`;
`popover.font.color` is named by its own entry (`no receiver`);
`window.background_color` moves `styles::aw::menu.bar_background` beside
`styles::application.background_color`; the row `styles::aw::menu.bar.border` is gone
from `[rows]`. The same moves seen from the rows (`[rows]`, today `:1204-1215`,
`:1382-1383`): `styles::aw::menu.bar_background`'s `leaves` becomes
`["window.background_color"]`, and `styles::aw::menu.menu.border`'s and
`styles::menu.border`'s become `["popover.border.color"]`. The test `every_colour_leaf_reaches_a_row_or_is_named` fails while the
file differs.

### E.5 The crate's documentation

What reaches the two styles from `popover.*` (E.1): only its border. The drop-down
takes `popover.border`'s colour and line width; the `iced_aw` menu takes its colour,
line width and corner radius, and `popover.border.shadow_enabled` decides whether it
casts `iced_aw`'s shadow. Neither takes `popover.background_color`: both fill from
`menu.background_color`. And neither draws the colour or width of a `menu.border` a theme states (the
drop-down keeps only its corner radius): no bundled preset states a line there (E.7's
source), but `MenuTheme`'s border can be stated in a
user's theme (`native-theme/src/model/widgets/mod.rs:257`), which then loses that
frame in both styles.

- `src/lib.rs:355` (one line, "//!   reach nothing. A dialog, which neither has
  either, is the consumer's") becomes these eight; `:353-354` before it and `:356` on
  stay:

  ```text
  //!   reach nothing, but for four `popover.border` leaves, which iced's menus
  //!   take, as the platforms frame a popup: a drop-down (`styles::menu`) is
  //!   lined in `popover.border`'s colour and width, and an `iced_aw` menu
  //!   (`styles::aw::menu`) is framed in its colour, width and corner radius,
  //!   with `iced_aw`'s shadow only where `popover.border.shadow_enabled`
  //!   states one; so a theme's `menu.border` colour and width reach neither, and
  //!   both fill from `menu.background_color`. A dialog, which neither iced nor
  //!   `iced_aw` has either, is the consumer's
  ```
  The file grows by 7 lines from `:356` on.
- `src/lib.rs:318`, the `styles` row's last cell, one line, in place. The row lists
  the 33 functions of `styles` itself, among them `menu`, and not `styles::aw` (the
  table has no row for `styles::aw`, which `:154` names under the `iced_aw` feature),
  so its text speaks of the drop-down alone; the `iced_aw` menu's fields are said in
  `:355`'s item above. "the widget's own resolved theme; fields the model lacks come
  from iced's default" → "the widget's own resolved theme (`menu`, a drop-down's: its
  fill the menu's, its line the popover's border's, as the platforms frame a
  popup); fields the model lacks come from iced's default".
- `README.md:653-654` ("- **Widgets iced lacks.** Neither iced 0.14 nor `iced_aw`
  0.14.1 has a" / "  popover or a status bar, so `popover.*` and `status_bar.*` reach
  nothing.") become these eight lines; `:655` on stays:

  ```markdown
  - **Widgets iced lacks.** Neither iced 0.14 nor `iced_aw` 0.14.1 has a
    popover or a status bar, so `status_bar.*` reaches nothing, and of
    `popover.*` only the border reaches iced, in its menus, as the platforms
    frame a popup: a drop-down (`styles::menu`) takes `popover.border`'s colour
    and width, and an `iced_aw` menu (`styles::aw::menu`) its colour, width and
    corner radius, and casts `iced_aw`'s shadow where
    `popover.border.shadow_enabled` states one. Both fill from
    `menu.background_color`, and neither draws the colour or width of a
    `menu.border` a theme states.
  ```

### E.6 The showcase, `connectors/native-theme-iced/examples/showcase-iced.rs`

Every edit below gives its exact lines (`0fa28027`'s). Searched today for every statement that a drop-down or
a menu is framed by `menu.border` (`grep -n 'menu\.border'` over the file, padding and
radius hits set aside): `:6516`, `:15019`, `:15028`, `:15064`, `:15259`, `:15264`,
`:15271`, `:15729`, and the tests `:26903`, `:26957`, `:27046`, `:27065-27070`,
`:27100`, `:27857`, `:27866`; plus the `menu_paints` zero leaf (`:12912`). `:9661`
and `:15220` name the output field `menu_border` of `styles::aw::menu` ("menu.border"
as the contract spells the field), not the model's leaf, and stay.

**The drop-down's Info (every build).**

0. `menu_paints` (`:12897-12914`): `&line_zero("menu"),` (`:12912`) →
   `&line_zero("popover"),`. One line, in place. After E.2 the drop-down's line is
   `popover.border.line_width`; left as it is, the Info would say "not drawn
   (menu.border.line_width 0)" on material-3, whose popover line is 0 wide.

**The Extra page (built with `iced_aw`).**

1. `aw_menu_padding` (`:15016-15037`, decision 9). Its doc, `:15016-15025` (ten
   lines, from "/// The padding of the Extra page's menu panels round their rows, the"
   to "/// [`menu_popup_padding`]."), becomes seven:

   ```text
   /// The padding of the Extra page's menu panels round their rows, the
   /// drop-downs' and the context menu's popup: [`menu_popup_padding`]'s, the
   /// window's menus' own, since both are framed by the popover's border:
   /// `popover.border.padding` inside `popover.border.line_width` (the line
   /// `styles::aw::menu`'s `menu_border` and [`menu_panel`]'s container paint
   /// over the panel's padding, iced_aw 0.14.1
   /// `src/widget/menu/menu_tree.rs:770-779`).
   ```
   Its body, `:15028-15036` (nine lines, from `    let line =
   resolved.menu.border.line_width;` to the `Padding { … }` literal's closing
   `    }`), becomes the one line `    menu_popup_padding(resolved)`. The
   `#[cfg(feature = "iced_aw")]` (`:15026`), the signature (`:15027`) and the closing
   brace (`:15037`) stay.
2. `menu_panel` (`:15061-15079`): doc lines `:15063-15064` (two) become three:

   ```text
   /// `menu_shadow`, read from its output (`menu.background_color`, framed by
   /// `popover.border`, with iced_aw's shadow where
   /// `popover.border.shadow_enabled`). `iced_aw`'s `ContextMenu`
   ```
3. `aw_menu_info` (`:15203-15291`):
   - the row `                    ("bar border", "bar.border"),` (`:15219`) is removed;
   - `            &line_zero("menu"),` (`:15223`) → `            &line_zero("popover"),`;
   - the "panel padding" string (`:15258-15260`, three lines) →
     `"aw_menu_padding: popover.border.padding inside the panel's \` /
     `line, the side plus popover.border.line_width; iced_aw's 5 \` /
     `where unstated",` (three lines, indented as today);
   - the "bar padding" string (`:15264-15265`, two lines) →
     `"none: the bar has no frame, its titles side by side with no \` /
     `gap, as the window's menu bar's",` (two lines);
   - the "context menu" string (`:15269-15272`, four lines) →
     `"our own popup, drawn as the panel (styles::aw::menu's \` /
     `menu_background, menu_border and menu_shadow): \` /
     `menu.background_color framed by popover.border, padded by \` /
     `aw_menu_padding; its entries menu_row",` (four lines);
   - the not-themeable "shadows" string (`:15278`, one line) →
     `"the model has no shadow geometry — iced_aw's own, a menu's \` /
     `only where popover.border.shadow_enabled",` (two lines).
   Net: the function keeps its line count (one line out, one in).
4. The Extra page's `MenuBar` (`:15729-15733`): the comment (`:15729-15732`, four
   lines, "// The titles inside the bar's line, `menu.border` …") and
   `    .padding(Padding::new(menu_t.border.line_width))` (`:15733`) are replaced by
   three comment lines and no `.padding(..)` call:

   ```text
       // The titles side by side with no gap, as the window's menu bar's titles
       // are (iced_aw's own `spacing`, none); the bar has no frame, so no padding:
       // `MenuBar`'s is `Padding::ZERO` (iced_aw 0.14.1 `src/widget/menu/menu_bar.rs:136`).
   ```

5. Tests:
   - `the_extra_menus_are_the_themes_menus` (`:26890-…`): doc `:26898-26904` —
     "inside `menu.border`'s line" → "inside `popover.border`'s line"; "is
     `popover.border.padding` plus `menu.border.line_width` in from its popup's corner,
     read from the leaves (12 px), where the popover's own line made it 13" → "is
     `popover.border.padding` plus `popover.border.line_width` in from its popup's
     corner, read from the leaves (13 px)"; "framed by `menu.border`" → "framed by
     `popover.border`". Code: `wide.menu.border.line_width` (`:26957`),
     `theirs.menu.border.line_width` (`:27046`) → `popover`; the two messages "inside
     menu.border" (`:26968`, `:27055`) → "inside popover.border"; the splice at
     `:27065-27070`: comment → "// The splice widens kde-breeze's 1 px popover line to
     2, in a colour of its own."; `menu.border.color` and `.line_width` →
     `popover.border.color` and `.line_width`; the message "menu.border.color frames"
     (`:27100`) → "popover.border.color frames". If the context-popup part then
     fails because the 2 px line moves the popup's first entry or the popup's
     rectangle, the rectangle is taken from the frame drawn with the splice applied
     (an `extra_frame` after the splice, its `rects`), not from the unspliced one
     (`:27037-27040`).
   - `the_extra_infos_say_what_the_page_draws` (`:27827-…`): "panel padding" expects
     `["aw_menu_padding", "popover.border.padding", "popover.border.line_width"]`;
     "context menu" expects `["menu.background_color", "popover.border",
     "aw_menu_padding", "menu_row"]`; two rows are added to its list,
     `(&menu, "bar padding", &["no frame"][..])` and
     `(&menu, "shadows", &["popover.border.shadow_enabled"][..])` (H.3).

**The window menus keep `menu_bar_style`** (decision 12).

- Its doc's last paragraph, `:6515-6517` (three lines, "/// Not the connector's
  `styles::aw::menu`, which the Extra page shows: that" to "/// which platform-facts
  §2.6 says is not the popup's."), becomes four:

  ```text
  /// The same `Style` as the connector's `styles::aw::menu`, which the Extra
  /// page shows and `the_window_menus_draw_as_the_connectors` holds it to:
  /// the window's menus are built in every build of the showcase, and
  /// `styles::aw` only with the connector's `iced_aw` feature.
  ```
  Its body stays.
- Routes (`:8034`, `:8037`, `:8046-8051`, `:8443`) and `drawn_by`'s
  `"menu_bar_style"` arm (`:29033-29047`): no edit.

**New tests**, in the showcase's test module after `drawn_by` (`:29008-29075`), so no
line before `:29008` moves:

- `the_window_menus_draw_as_the_connectors`, `#[cfg(feature = "iced_aw")]`
  (decision 12): for every `(label, r)` of `bundled_combinations()` (`:28985`), with
  `theme = native_theme_iced::to_theme(r, "menus")`, and for each of `Active`,
  `Hovered`, `Pressed`, `Disabled`, `Focused`, `Selected`, `format!("{:?}", …)` of
  `menu_bar_style(r)(&theme, s)` equals that of `styles::aw::menu(r)(&theme, s)`, the
  message naming the combination and the status. It fails today (E.3 not applied):
  the connector fills the bar from `menu.background_color` and frames it in
  `menu.border`.
- `the_menu_infos_name_the_width_the_menus_draw` (decision 13): under material-3
  light, the one bundled preset whose popover line is 0 wide (E.7), with the state
  installed as the file's tests install a preset (`install_in_mode`, as at
  `:25726-25733`), take the leaf the drop-down's Info names for its undrawn line, the
  `zero` of the "menu border" paint of `menu_paints(&state)`, less its trailing
  " 0"; splice that leaf to 3 in the resolved theme's serialised form (as
  `the_menus_panel_lines_name_what_pads_and_shadows_it` splices a width,
  `:25748-25763`); assert that
  `styles::menu(&spliced)(&theme).border.width` is 3. With `iced_aw`, the same for
  the Extra page: the leaf is the one `aw_menu_info(&state)`'s text names after "not
  drawn (" on its "panel border" row, and the width is
  `styles::aw::menu(&spliced)(&theme, AwStatus::Active).menu_border.width`. It fails
  where an Info names a leaf its style does not draw the line from; today, after E.2
  and E.3 without edits 0 and 3 above, it fails on both.

### E.7 What changes on screen, per bundled preset

Computed today from the presets as D.4 says (the popover's corner radius from
`defaults.border.corner_radius_lg`, `docs/inheritance-rules.toml:103`; a menu border
the preset does not state resolves to transparent, 0, 0, no shadow,
`native-theme/src/resolve/validate_helpers.rs:279-291`). "Bar" is
`styles::aw::menu`'s bar; "Panel" is the fill of its menus and of `styles::menu`'s
drop-down, both `menu.background_color` (D3 (i)); "Menu frame" is `styles::aw::menu`'s
`menu_border`; "Drop-down line" `styles::menu`'s border. "Line on the panel" is the
WCAG contrast of the popover's line, composited source-over on the panel where it is
translucent, against the panel, computed in Python. A single value means no change.
kde-breeze's panel is after Part B.

| Preset/mode | Bar | Panel (menu and drop-down) | Menu frame (today none) | Drop-down line (today none); radius | Line on the panel |
|---|---|---|---|---|---|
| kde-breeze/light | #eff0f1 | #eff0f1 → #f4f5f5 | #c2c5c9 1 px, r 8 | #c2c5c9 1 px; r 0 | 1.59 |
| kde-breeze/dark | #202326 | #202326 → #1c1f22 | #474c51 1 px, r 8 | #474c51 1 px; r 0 | 1.91 |
| adwaita/light | #fafafb | #fafafb | #0000000d 1 px, r 15 | #0000000d 1 px; r 0 | 1.12 |
| adwaita/dark | #222226 | #222226 | #0000000d 1 px, r 15 | #0000000d 1 px; r 0 | 1.02 |
| windows-11/light | #f9f9f9 → #f3f3f3 | #f9f9f9 | #0000000f 1 px, r 8 | #0000000f 1 px; r 0 | 1.14 |
| windows-11/dark | #2c2c2c → #202020 | #2c2c2c | #00000033 1 px, r 8 | #00000033 1 px; r 0 | 1.12 |
| macos-monterey/light | #ececec | #ececec | #00000019 0.5 px, r 10 | #00000019 0.5 px; r 0 | 1.24 |
| macos-monterey/dark | #323232 | #323232 | #ffffff19 0.5 px, r 10 | #ffffff19 0.5 px; r 0 | 1.36 |
| macos-tahoe/light | #ffffff | #ffffff | #00000019 0.5 px, r 10 | #00000019 0.5 px; r 0 | 1.25 |
| macos-tahoe/dark | #1e1e1e | #1e1e1e | #ffffff19 0.5 px, r 10 | #ffffff19 0.5 px; r 0 | 1.34 |
| material-3/light | #f3edf7 → #fef7ff | #f3edf7 | 0 px, r 4 (as today; the bar loses its r 4) | 0 px; r 4 | no line |
| material-3/dark | #211f26 → #141218 | #211f26 | 0 px, r 4 (as today; the bar loses its r 4) | 0 px; r 4 | no line |
| ios-13/light | #ffffff | #ffffff | #c6c6c8 0.5 px, r 13 | #c6c6c8 0.5 px; r 0 | 1.71 |
| ios-13/dark | #000000 | #000000 | #38383a 0.5 px, r 13 | #38383a 0.5 px; r 0 | 1.79 |
| catppuccin-latte, -frappe, -macchiato, -mocha / light | #eff1f5 | #eff1f5 | #bcc0cc 1 px, r 12 | #bcc0cc 1 px; r 0 | 1.61 |
| catppuccin-latte, -frappe / dark | #303446 | #303446 | #51576d 1 px, r 12 | #51576d 1 px; r 0 | 1.72 |
| catppuccin-macchiato/dark | #24273a | #24273a | #494d64 1 px, r 12 | #494d64 1 px; r 0 | 1.77 |
| catppuccin-mocha/dark | #1e1e2e | #1e1e2e | #45475a 1 px, r 12 | #45475a 1 px; r 0 | 1.80 |
| nord light / dark | #eceff4 / #2e3440 | the same | #d8dee9 / #434c5e 1 px, r 12 | #d8dee9 / #434c5e 1 px; r 0 | 1.17 / 1.45 |
| dracula light / dark | #fffbeb / #282a36 | the same | #b8b8c8 / #6272a4 1 px, r 12 | #b8b8c8 / #6272a4 1 px; r 0 | 1.89 / 3.03 |
| gruvbox light / dark | #fbf1c7 / #282828 | the same | #d5c4a1 / #504945 1 px, r 12 | #d5c4a1 / #504945 1 px; r 0 | 1.51 / 1.67 |
| solarized light / dark | #fdf6e3 / #002b36 | the same | #93a1a1 / #586e75 1 px, r 12 | #93a1a1 / #586e75 1 px; r 0 | 2.48 / 2.79 |
| tokyo-night light / dark | #e1e2e7 / #1a1b26 | the same | #c4c8da / #292e42 1 px, r 12 | #c4c8da / #292e42 1 px; r 0 | 1.29 / 1.27 |
| one-dark light / dark | #fafafa / #282c34 | the same | #e0e0e0 / #3e4451 1 px, r 12 | #e0e0e0 / #3e4451 1 px; r 0 | 1.26 / 1.43 |

The drop-down's fill changes only on kde-breeze, with its menu panel; there the
drop-down lies four to five levels from Breeze's combo popup, whose rows are on the
window colour (measured, C.1): #f4f5f5 against #eff0f1 in light (5, 5, 4 levels per
channel) and #1c1f22 against #202326 in dark (4, 4, 4), the cost D3 (i) accepts.
No drawn line has its panel's own colour on any of the 34 combinations (each
compared). The faintest: adwaita's dark #0000000d at 1.02:1 and light at 1.12:1,
windows-11's at 1.14:1 and 1.12:1: drawn, barely seen. The same lines frame the
`iced_aw` menus, whose panel is the same fill.

The menus' shadow: `popover.border.shadow_enabled` is true on all 34 combinations, so
every menu keeps iced_aw's shadow, as today. The drop-down keeps no shadow, as today.
The showcase's window menus look the same before and after (they already draw this,
`menu_bar_style`), but for kde-breeze's new panel; the Extra page's menu bar and
every drop-down change as the table says.

## Part F — egui and the showcases' tables

**egui: nothing changes in the connector.** Its routes are sources, not values, and
stay true: `src/mapping.md:136` (`menu.background_color` → `visuals.window_fill`),
`:390` (`list.grid_color` → «list» `noninteractive.bg_stroke.color`), `:660`, `:676`;
`mapping.toml:764-766`, `:1962-1964`; the README's table (`README.md:46`). No test
pins a kde-breeze value of the two fields (searched today: `#bcc0bf`, `#4d545b` and
their decimal and `0x` forms over `*.rs`, `*.toml`, `*.py`, `*.sh` outside
`docs/archive/`; the hits are the three `docs/inheritance-rules.toml` strings and
`kde-breeze.toml` itself, which states the two values for other fields:
`[light.defaults.border] color` and `[dark.defaults.border] color` (`:74`, `:659`),
`[light.slider] disabled_fill_color` and dark (`:260`, `:847`), `[light.splitter]
hover_color` and dark (`:437`, `:1025`); no test). The tests that
read the fields compare routes: `style/base.rs:525` (`window_fill` is
`menu.background_color`), the contrast pairs `src/contract.rs:1762-1822` (emitted and
native read the same field; `asserted: false`). What a user sees on KDE changes as D2
and D3 accept: a separator or a group frame inside a «list» scope takes Qt's grid;
every popup egui fills with `window_fill` (a combo box's among them) and an
`egui::Window`, a modal or a tooltip drawn without the connector's surface frames take
Breeze's menu panel. Part G's CHANGELOG line says so.

**The showcases' own tables** paint their grid from `list.grid_color`
(`showcase-iced.rs:11887`, `TableColours`; `showcase-egui/pages/basic.rs:1757`;
`showcase-gpui/demo.rs:9418`), and their Widget Info and route texts name the field,
not a value (`showcase-iced.rs:8281-8283`; `showcase-egui/pages/basic.rs:1734-1739`;
`showcase-gpui/info/data.rs:486-498`). Under kde-breeze their lines become Qt's grid,
as gpui-component's tables do through `table_row_border`. No showcase test asserts a
grid or menu colour's value (searched as above); the ones that read them take the
installed theme's.

## Part G — Outside the code (D10)

### G.1 `CHANGELOG.md`, `## [0.7.1]`

One physical line each, in the style of the 0.7.1 entry's KDE lines (`:26-29`).

Under `### Fixed`, after the line that begins
"- **native-theme**: kde-breeze states `dialog.icon_size` 64":

```markdown
- **native-theme**: kde-breeze and the KDE reader: a table's grid is Qt's, `QPalette::Mid`, KColorScheme's mid shade of the window colour at the scheme's `[KDE] contrast` (`list.grid_color`, #c4c8cc / #1c1f21 under Breeze), where it inherited the border colour, #bcc0bf / #4d545b, far brighter than Qt's in dark mode. The reader computes it from `[Colors:Window] BackgroundNormal` and `[KDE] contrast` (KColorScheme's default 7 where the key is missing or not an integer), with KColorScheme's shade function in Qt's arithmetic. gpui-component's tables (`table_row_border`) and the three showcases' tables draw it; in the egui connector a separator or a group frame inside the «list» role, which takes the role's stroke, turns as faint, while a list's own frame keeps `list.border.color`.
- **native-theme**: kde-breeze and the KDE reader: a menu's panel is Breeze's, `mix(Window, Base, 0.3)` (`menu.background_color`, #f4f5f5 / #1c1f22), where it inherited the window colour, #eff0f1 / #202326. The egui connector fills `visuals.window_fill` with it, so every popup, a combo box's among them, which Breeze fills with the window colour, and an `egui::Window`, a modal or a tooltip drawn without the connector's surface frames take that colour too. Menus drawn from the field take it as their panel: egui's (the egui showcase's window menus among them), the iced connector's `styles::aw::menu` menus and its `styles::menu` drop-downs (which lie four to five levels from Breeze's combo popup, on the window colour), and the iced and gpui showcases' window menus; gpui-component paints its own menus in the `popover` token, which stays the window colour.
```

At the end of `### Changed`, after its last entry (at `0fa28027` "- **All crates**:
the manifests no longer state `homepage`…"; by then the GPUI plan's LN has added its
**Development** line after it):

```markdown
- **native-theme-gpui**: `chart_grid` takes `separator.line_color`, where it took `list.grid_color`: a chart's grid is a set of separating lines, and on KDE `list.grid_color` is now Qt's table grid, which would all but vanish under a chart in dark mode. The colour changes on `adwaita` (#d5d5d5 / #4a4a4e, where it was #dcdcde / #434347) and the two macOS presets (#00000019 / #ffffff19, where it was #e6e6e6 / #1a1a1a).
- **native-theme-iced**: `styles::aw::menu` draws the menu bar on the window, `window.background_color` with no frame, and frames its menus in `popover.border` (colour, width and radius), with `iced_aw`'s shadow only where `popover.border.shadow_enabled`; it filled the bar as a menu and framed both in `menu.border`, whose line no bundled preset states, so their menus had no frame. `styles::menu`, the drop-down of a `PickList` and a `ComboBox`, keeps the menu's fill, text and highlight and takes the popover's line (colour and width), keeping `menu.border.corner_radius`: a drop-down is framed as the platform frames a popup. The colour and width of a `menu.border` that a theme of your own states are now drawn by neither style. On screen: the bar's fill changes on `windows-11` and `material-3` (whose bar loses its 4 px radius); the menus gain the popover's frame and radius, and the drop-down the popover's line, on every preset but `material-3`; a drop-down's highlighted row is that line narrower on each side, and a highlighted first or last row paints over the line at the top or the bottom, which iced does not inset; on KDE both take the new menu panel colour.
```

The 0.7.1 date (`:8`) follows the asset run (Part H).

### G.2 `ROADMAP.md:104-110`

Today, from the end of `:104` to `:110`: "The platform facts for / the focus ring, the default button, the menu panel and
the table's grid are / corrected too, and their values wait for a connector design;
the selection of / an unfocused view waits for a text colour the model does not have
yet. The / next step proposed is Breeze's hover outlines: Breeze shows hover on a
button, / a combo box, a check box and a slider handle by recolouring the outline,
which / the model can state for text inputs only." `:104` keeps "focus ring, and what
only gpui-component can change." and ends there; the text from "The platform facts
for" to `:110`'s end is replaced by these lines:

```markdown
A table's grid is Qt's, computed from the colour scheme, and a menu's panel is
Breeze's; the iced connector's menu bar lies on the window, its menus and
drop-downs are framed as the platforms frame a popup, and the gpui connector
draws a chart's grid in the separator colour. The platform facts for the focus
ring and the default button are corrected too, and their values wait for a
connector design; the selection of an unfocused view waits for a text colour
the model does not have yet. The reporter's remaining items, the focus ring and
Breeze's hover outlines among them, are planned for later 0.7.x releases
(`docs/issues/0004_kde-open-items-rationale.md`).
```

And `ROADMAP.md:264`'s count, "tags 29 open items `[v0.7.x]`" (29 or the number the
GPUI Kit port's G.2 wrote there), becomes the number
`grep -c '^- \[ \] \[v0.7.x\]' docs/todo.md` prints on unit 4's tree, after G.5
has opened its entries; the commit body records it. Today the grep prints 41; the
port's work adds 2 and G.5 adds 3 (1a, 1b and 1d; 1c is `[upstream]`), so 46 is
expected, and another figure is explained in the commit body before the unit lands.

### G.3 The first rationale, `docs/issues/0004_kde-hand-overrides-rationale.md`

The reply draft's edits (§8) apply only while the reply has not been posted; a posted
reply is not edited (the batch's rule, as the steps 2–3 specification's 2I.3 states it).

- **§4.2** (`:350-354`): the sentence "On the panels the connectors paint, the two
  differ by three levels per channel at most (#b9dcef against #bcdff1 in light,
  #294d61 against #264a5e in dark): iced and egui paint `menu.background_color`,
  gpui paints its `popover` token, and on KDE all three are the window colour (§5.5
  says why the panel's own colour waits)." becomes "On the panels the connectors
  painted when this was decided, the two differ by three levels per channel at most
  (#b9dcef against #bcdff1 in light, #294d61 against #264a5e in dark): iced and egui
  painted `menu.background_color`, gpui its `popover` token, and on KDE all three
  were the window colour. Since 0.7.1 kde-breeze's menu panel is Breeze's (§5.5,
  settled by `0004_kde-open-items-rationale.md` D3): egui's menus and the iced
  connector's menus and drop-downs paint it, while gpui's menus keep the window
  colour.", rewrapped within the paragraph; "**Decision: A**" and
  what follows stay. The other references to §5.5 and §5.6 (`:119`, `:589`, `:994`) record
  what that document weighed and which backlog entries it wrote; they stay.
- **§5.5** (`:924`), a line after the heading:
  "*Settled by `0004_kde-open-items-rationale.md` D3 (0.7.1): kde-breeze and the KDE
  reader state Breeze's panel; the iced connector's menu bar takes the window's
  colour, and its menus and drop-downs keep the menu's panel, framed by the popover's
  border.*"
- **§5.6** (`:939`), a line after the heading:
  "*Settled by `0004_kde-open-items-rationale.md` D2 (0.7.1): kde-breeze and the KDE
  reader state Qt's grid; gpui's `chart_grid` takes the separator colour; egui's list
  role was not split.*"
- **§7, `:1024`**, B6's third cell → "popups take the window colour since 0.7.0;
  Breeze's menu panel, `mix(Window, Base, 0.3)`, is kde-breeze's and the reader's since
  0.7.1 (`0004_kde-open-items-rationale.md` D3). gpui-component paints a menu in its
  `popover` token, which stays the window colour, so in gpui the small difference
  remains".
- **§7, `:1027`**, B9's third cell → "**fixed in 0.7.1** (`0004_kde-open-items-rationale.md`
  D2): `list.grid_color` is Qt's `Mid`, #c4c8cc / #1c1f21, which `table_row_border`
  takes; the override is no longer needed".
- **§8, the reply.** (1) In the list "Version 0.7.1 fixes these:", after the bullet
  "**Table header.**" (`:1062-1064`), insert:

  ```markdown
  > - **Table lines.** You were right that they are too bright in the dark theme.
  >   native-theme now gives a table's grid the colour Qt draws it in, Qt's `Mid`
  >   (#c4c8cc in the light theme, #1c1f21 in the dark), computed from your colour
  >   scheme, so `table_row_border` is that colour and you need not set it. Charts
  >   in gpui-component keep a grid of their own, in the separator colour, since
  >   Qt's grid colour would all but vanish under a chart.
  ```
  (2) `:1072-1080`: in "you could delete these overrides: …", after "`table_hover`,"
  insert "`table_row_border`,"; "Still yours to keep: the 16 px groove (until
  gpui-component's table is fixed), `focus_ring`, `muted` (that one is your
  application's own choice) and `table_row_border`." → "Still yours to keep: the 16 px
  groove (until gpui-component's table is fixed), `focus_ring` and `muted` (that one
  is your application's own choice)." (3) The bullet "**Table lines.**" of "Some things
  are not fixed yet" (`:1086-1095`) is removed. (4) The bullet "**Menus.**"
  (`:1105-1107`) is split: its fix goes to the list "Version 0.7.1 fixes these:",
  after the bullet (1) inserts, and what stays unfixed in GPUI stays where the bullet
  is. Inserted after (1)'s "**Table lines.**":

  ```markdown
  > - **Menus.** Breeze's menu colour is very slightly different from the
  >   window's (a little toward the list colour). native-theme now gives a menu
  >   that colour, and the iced and egui connectors draw it.
  ```
  and the bullet at `:1105-1107` becomes:

  ```markdown
  > - **Menus in GPUI.** gpui-component paints every menu in its `popover`
  >   colour, the window's on KDE (since 0.7.0, so you no longer need to set
  >   `popover`), and has no colour of its own for a menu, so in GPUI the small
  >   difference from Breeze's menu colour stays.
  ```
  (5) The bullet "**Hover borders on buttons.**" (`:1102-1104`) ends "It is at the top
  of my list.", which D10's proposed order makes untrue. D10's proposal
  (`docs/issues/0004_kde-open-items-rationale.md`, D10, "The releases after v0.7.1";
  the order is the maintainer's): 1. the egui 0.37 upgrade; 2. steps 2 to 5, the
  reporter's remaining items, the focus ring (step 2), the button's outlines
  (step 3) and the toolbar buttons (step 5) among them; 3. gpui-component's own
  icons; 4. the caret's blink; 5. steps 6 and 7, the table header in step 7. The
  icons before steps 2 to 5 is the other order it weighs and does not propose. The
  sentence "It is at the top of my list." becomes "It is planned, after the focus
  ring, in a coming 0.7.x release; an update of the libraries native-theme is built
  on comes before both. In GPUI it depends on whether gpui-component's buttons can
  take it, which I have yet to try." (D5, "Connectors", gpui: a trial decides whether
  a custom variant can carry the border; "an update of the libraries" is D10's first
  release, the egui 0.37 upgrade, said in words a GPUI user needs no context for),
  rewrapped inside the bullet.

  (6) "The rest of your list" (`:1121-1126`): its sentence "Toolbar buttons and the
  pressed sort header: native-theme has no values for their states yet; they belong
  to the same work as the hover borders." takes both to come with the hover borders,
  while D10 puts the toolbar buttons among steps 2 to 5 and the table header in
  step 7, last, and D7 ("The table header") and §1's table say that no toolkit's
  table, gpui-component's included, can show a header's states: they reach an
  application only through the model and the connectors' accessors. It becomes
  "Toolbar buttons: native-theme has no values for their states yet; they come in
  the same series of releases as the hover borders. The pressed sort header has none
  either; it comes last, and since no GUI library's table can show it, its values
  will reach an application only for a table it draws itself.", rewrapped.

  (7) The bullet "**The focus ring.**" (`:1096-1101`) says what gpui-component does and
  that turning the ring off is right "until then", and nothing of what native-theme
  plans, which (5)'s "after the focus ring" now presumes. After "…is a good choice on
  KDE; the README now shows how and says what stays visible." add: "In a coming
  0.7.x release, after an update of the libraries native-theme is built on,
  native-theme plans to state that Breeze shows its ring for keyboard focus only,
  and the connectors to follow it; for gpui-component's own widgets that means
  switching its ring off until gpui-component can tell." ("as you did" is said once,
  in the sentence before; D4: the field `focus_ring_keyboard_only`, and
  gpui-component's band switched off where it is true).

  The texts (2), (5), (6) and (7) change, in full after this step (the batch's
  reply-draft rule; each replaces its paragraph or bullet whole):

  ```markdown
  > With 0.7.1 (which is built on GPUI Kit 0.9.1) you could delete these
  > overrides: the font size, the scroll bar's colours, thumb width and minimum
  > length, `popover`, `selection`, `accent`, `table_hover`, `table_row_border`,
  > the writes to `theme.tokens` (`Theme::update` keeps them in step by itself
  > there) and the second write of the scroll bar styles (the connector does that
  > itself). Still yours to keep: the 16 px groove (until gpui-component's table
  > is fixed), `focus_ring` and `muted` (that one is your application's own
  > choice). The scroll bar mode override is not needed as long as
  > `gpui_kit::init` runs before `apply_system_theme`.
  ```

  ```markdown
  > - **The focus ring.** Breeze draws a ring only when you move with the Tab key.
  >   With the mouse it only recolours a text field's border. gpui-component draws
  >   its ring however the focus came. GPUI itself can tell keyboard from mouse,
  >   so I will ask gpui-component to use that. Until then, turning the ring off,
  >   as you did, is a good choice on KDE; the README now shows how and says what
  >   stays visible. In a coming 0.7.x release, after an update of the libraries
  >   native-theme is built on, native-theme plans to state that Breeze shows its
  >   ring for keyboard focus only, and the connectors to follow it; for
  >   gpui-component's own widgets that means switching its ring off until
  >   gpui-component can tell.
  > - **Hover borders on buttons.** Breeze shows hover by recolouring a button's
  >   border, not its fill. native-theme has no value for that yet, which is why
  >   you had to type it yourself. It is planned, after the focus ring, in a coming
  >   0.7.x release; an update of the libraries native-theme is built on comes
  >   before both. In GPUI it depends on whether gpui-component's buttons can take
  >   it, which I have yet to try.
  ```

  ```markdown
  > The rest of your list, briefly. Toolbar buttons: native-theme has no values for
  > their states yet; they come in the same series of releases as the hover
  > borders. The pressed sort header has none either; it comes last, and since no
  > GUI library's table can show it, its values will reach an application only for
  > a table it draws itself. Dialog text is the general font, and Breeze sets no
  > dialog window size. Taking dark or light from `system_is_dark()` for the
  > fallback was right. And for looking at every colour of a theme, the showcase
  > example shows each widget's colours with where they come from.
  ```

  The note after the reply (`:1128-1129`), "The sentences "I am reporting it there",
  "I will ask gpui-component" and "at the top of my list" are the maintainer's to
  confirm before posting.", becomes: "The sentences "I am reporting it there", "I will
  ask gpui-component", "after an update of the libraries native-theme is built on",
  "native-theme plans to state that Breeze shows its ring for keyboard focus only",
  "It is planned, after the focus ring, in a coming 0.7.x release", "an update of the
  libraries native-theme is built on comes before both", "In GPUI it depends on
  whether gpui-component's buttons can take it, which I have yet to try", "they come
  in the same series of releases as the hover borders" and "it comes last" are the
  maintainer's to confirm before posting (putting the update first is itself the
  maintainer's to decide): they follow
  `0004_kde-open-items-rationale.md`'s proposed order (D10), and "0.7.x" its proposed
  versions (D0 (a); under (b) or (c) some of these releases are minor ones, and the
  wording changes with them)." The same holds for "later 0.7.x releases" in G.2's
  roadmap text, which follows D0 (a) too.

### G.4 The rationale's status line

`docs/issues/0004_kde-open-items-rationale.md`'s "Status:" paragraph (under its title)
gains, once step 1 is implemented:
"Step 1 (D2, D3) implemented on `<date>` for 0.7.1, by
`../archive/todo_v0.7.1_kde-open-items-step1-{spec,plan}.md`." The rationale itself
stays in `docs/issues/`: six steps remain.

### G.5 `docs/todo.md`

Conventions as the first specification's C.4 (the file's header `:1-21`): a dated line
`*<what> (<date>, <sha>):*` names the commit; key paths, no preset line numbers.

1. **"KDE values that wait for a connector design"** (`:1826-1913`). Items (3) and (4)
   are done, (1), (2), (5) stay, so the entry stays. Its opening says each item is a
   value that waits for a connector design, so the reading task item (3) carries goes
   to an entry of its own (item 1a). Items (3) and (4)'s text (`:1869-1896`, from "(3)
   **The menu panel's colour**" to "(the gpui README's overrides section).") becomes
   "(3) **The menu panel's colour** and (4) **the table's grid**: done in 0.7.1
   (below)." Item (5) keeps its number. After the entry's last line (`:1913`):

   ```markdown
         *Issue #4, open items step 1 (0.7.1, <date>, <sha>):* (3) and (4) done
         (`docs/issues/0004_kde-open-items-rationale.md` D2, D3). kde-breeze
         (`kde-breeze.toml` `[light.menu] background_color`, `[dark.menu] background_color`,
         `[light.list] grid_color`, `[dark.list] grid_color`) and the KDE reader state
         Breeze's menu panel, `mix(Window, Base, 0.3)`, and Qt's grid, `MidShade` of the
         window at `[KDE] contrast`, through the carried `kde_shade`
         (`native-theme/src/kde/shade.rs`, with its table). gpui's `chart_grid` takes
         `separator.line_color`; the iced connector's menu bar lies on the window and
         its menus and drop-downs take the popover's frame on the menu's fill; egui
         fills every popup with the menu's colour.
   ```
1a. **A new entry**, after "KDE values that wait for a connector design" and before
   "The selection of a view without the focus" (`:1914`), in the same section:

   ```markdown
   - [ ] [v0.7.x] **Qt Quick's menu fill, for the facts.** `docs/platform-facts.md`
         §2.6's KDE `background_color` cell states a Qt Widgets menu's panel, `mix(Window, Base, 0.3)`, which kde-breeze and
         the KDE reader follow since 0.7.1, and marks Qt Quick's as not read. Read and
         cite qqc2-desktop-style's `Menu` fill, which M4 T36 read as
         `Kirigami.Theme.backgroundColor` (from *kde-breeze: the Qt Widgets menu fill*,
         closed into item (3) of "KDE values that wait for a connector design"), and
         state it in the row.
   ```
1b. **A new entry** after 1a, in the same section (found by the second review of this
   specification and checked against the files on 2026-11-14):

   ```markdown
   - [ ] [v0.7.x] **KDE: a menu's frame is Qt Quick's, its fill Qt Widgets'.**
         `docs/platform-facts.md` §2.6's KDE `background_color` cell records a Qt
         Widgets menu's panel outlined 1 px in
         `frameOutlineColor()`, #c6c8c9 / #4c4e51 (measured), and §2.6's
         `border.*` rows send the menu's frame to the popover ("popup border
         from §2.16"), whose KDE cell is qqc2-desktop-style's Popup, which kde-breeze
         states as #c2c5c9 / #474c51 (`kde-breeze.toml` `[light.popover.border] color`,
         `[dark.popover.border] color`). Since 0.7.1 kde-breeze's menu fill follows Qt
         Widgets, `mix(Window, Base, 0.3)`, and its frame Qt Quick: up to five levels
         apart in a 1 px line. The model's `menu.border.color`, which no preset states
         and which resolves to no line, could carry the Qt Widgets outline; the readers
         that frame a menu from `popover.border` would then read it: the iced
         connector's `styles::aw::menu` and `styles::menu`, and the three showcases'
         window menus (iced `menu_bar_style`, gpui `MenuLook`, egui `menu_modifier`).
         The other way, not taken in step 1: the resolver giving `menu.border` the
         popover's border where a theme states none, so the readers keep reading
         `menu.border` and a theme that states one keeps it; it would make the
         menu's border a border of its own kind, which the model does not give it
         today (`native-theme/src/model/widgets/mod.rs`, `MenuTheme`). Not part of
         `docs/issues/0004_kde-open-items-rationale.md`'s step 1.
   ```
1c. **A new section** after *iced: a PickList cuts a long label without an ellipsis,
   and its menu rows run out of the menu* (`:4428-4429`):

   ```markdown
   ### iced: a drop-down's highlighted first or last row paints over its frame

   - [ ] [upstream] iced 0.14 draws a `PickList`'s or a `ComboBox`'s drop-down as its
         panel and frame, then its rows over them in the same bounds, the first row at
         the top edge and the last at the bottom (iced_widget 0.14.2
         `src/overlay/menu.rs:312-330`, `:535-540`), and insets a highlighted row by the
         frame's width on the left and the right only (`:546-547`, in the row's quad,
         `:542-553`, which `:550` rounds with the panel's radius): a highlighted first
         or last row covers the frame's top or bottom line. Since 0.7.1 the connector's
         `styles::menu` frames the drop-down in `popover.border`, so the line exists on
         every bundled preset but material-3; `overlay::menu::Style` has no row inset,
         so the connector cannot change it. Read from the code, not rendered.
   ```
1d. **A new section** after 1c (D3 (i), "the backlog gets the entry"):

   ```markdown
   ### iced: a drop-down on the popover's fill

   - [ ] [v0.7.x] `docs/platform-facts.md` §2.16 puts a drop-down with the popover
         ("Popover / Dropdown"), and Breeze fills a combo box's popup with the window
         colour (measured on its view, §2.16's KDE `background_color` cell). Since
         0.7.1 the iced connector's `styles::menu` frames a `PickList`'s or a
         `ComboBox`'s drop-down with the popover's line but fills it with
         `menu.background_color`, since its rows are the menu's colours
         (`menu.hover_background`, the menu's text), which each preset states against
         the menu's fill: on the popover's fill they do not hold. In adwaita's dark
         mode the highlighted row, `adwaita.toml` `[dark.menu] hover_background`
         #3a3a3e, would lie on `[dark.popover] background_color` #36363a at 1.06:1,
         where on the menu's #222226 it is 1.40:1, with the same white label on both
         (`docs/issues/0004_kde-open-items-rationale.md` D3 (i)). Taking the popover's
         fill needs a row highlight stated against the popover's fill, which the
         model does not have; libadwaita's value for a dropdown's highlighted row was
         not read. On kde-breeze the drop-down so lies four to five levels from
         Breeze's combo popup (#f4f5f5 against #eff0f1 in light, #1c1f22 against
         #202326 in dark).
   ```
2. **The upstream entry "a menu-surface token"** (`:2403-2413`). After its last line:

   ```markdown
         *Issue #4 (0.7.1, <date>, <sha>):* at 0fa28027 23 of the 34 combinations differ
         (`windows-11` now states one colour for both); kde-breeze now states Breeze's
         menu panel, #f4f5f5 / #1c1f22, apart from its popover, so 25 do. The gpui
         contract's "hovered menu row" pair excepts both kde-breeze modes for it
         (`connectors/native-theme-gpui/src/contract.rs`, `MENU_PANEL_SURFACE`).
   ```
3. **"Model: the menu bar has no fill of its own"** (`:1183-1199`). After its last line:

   ```markdown
         *Issue #4 (0.7.1, <date>, <sha>):* the iced connector's `styles::aw::menu` fills the
         bar from `window.background_color` too
         (`docs/issues/0004_kde-open-items-rationale.md` D3), so a field of the bar's
         own would have that reader as well.
   ```
No entry closes whole, so nothing moves to `docs/archive/todo_closed.md`.

### G.6 `docs/archive/`

At the plan's close-out, this specification and its plan move to `docs/archive/`
(`docs/archive/README.md`: implemented design documents live there; the project's rule
that archiving is a plan's last task), and the v0.7.1 row (`docs/archive/README.md:51`)
gains: "; `todo_v0.7.1_kde-open-items-step1-{spec,plan}.md`: step 1 of the open KDE
items (the table's grid and the menu panel); their rationale stays with the issue, at
`docs/issues/0004_kde-open-items-rationale.md`". The links between the documents are
rewritten for the new place.

## Part H — Order, gates, close-out

### H.1 Units

Each unit lands as one change. Its gate at its end is the batch's cadence
(`docs/todo_v0.7.1_batch-speed-rationale.md`, S1), which replaces a run of every
gate of H.3 at every unit's end: during a unit, the agent runs only the touched
tests (its targeted commands); at a unit's end, `--quick`, by the orchestrator, as
a long run until its time is measured (the first timed into the record);
`--commit` every third code unit of the release, counted from the last
`--commit`, on the release's last code unit before its documents and version units
(rule 6 (batch-speed rationale, Appendix): here unit 3), after a lock or toolchain
change (the warm-up gate), and wherever the plan names a unit structural; a
documents-only unit (comments, rustdoc prose, READMEs and Markdown alone;
`docs/platform-facts.md` counts as code, since two tests read it by `include_str!`)
runs the five documentation checkers and `--quick`, unit 4 among them, the release's
last unit before the close-out and its gated head (rule 20: no extra `--commit`,
unit 3's covering the code beneath it); the full
run where the plan has it (the close-out, unit 5), unchanged; the plan names each
unit's gate. H.3's
commands are what `--commit` and the full run hold. The full run's two stamp
checks fail from the first commit of unit 1 until unit 5's
asset run renews both stamps, because `CHANGELOG.md:8` dates the 0.7.1 entry (H.3's
last row; the first KDE fixes' specification says the same of its parts,
`docs/archive/todo_v0.7.1_kde-hand-overrides-spec.md:823-829`); no unit leaves a test
red for a later one to fix. The units are committed in order, not pushed one by
one: units 1 to 4 are pushed together before the asset run (H.4), unit 5 after it.
Why each holds together:

| Unit | Parts | What would be red if a part of it landed alone |
|---|---|---|
| 0 — the documents | the rationale (`docs/issues/0004_kde-open-items-rationale.md`), this specification and its plan, as approved | nothing: they are documents no gate reads. They are added in the GPUI Kit port's commit (0) (`docs/todo_v0.7.1_gpui-kit-0.9.1-spec.md`, *Order and gates*) where they were approved by then; what that commit did not add is added here, before unit 1, in a commit of its own; where nothing is left, there is no unit 0. Units 1 to 5 cite the rationale (the new `UNREAD_COLOURS` texts, G.5's dated lines), and G.4 and G.6 edit and move these files, so they are tracked first |
| 1 — KDE values | A; B (B.2, B.4, B.5, B.6); C; D.2's `MENU_PANEL_SURFACE` and the two exceptions of "hovered menu row" | B.2 without B.4: `the_derived_colours_are_the_presets_under_breeze` (`kde/mod.rs:1397`), which needs the preset to state each derived leaf. B.4 without B.2: the same test, the reader then stating nothing. B.5's `contrast` field without the `:1661` edit: the test module does not compile. B.4 without the exceptions: gpui's `no_pair_contrasts_worse_than_the_platforms_own` (`connectors/native-theme-gpui/src/contract.rs:2425`) on `kde-breeze/light` and `/dark`; the exceptions without B.4: the same test, as stale (`:2442-2448`). A without B: nothing red (the module is unused, A.3's `allow` covers it), but it has no reader. C's cells and strings say what B.4 states; no gate reads them |
| 2 — the chart's grid | D.1; D.2's `chart_grid` row; D.3 | D.1 without the row: the contract's row check (`contract.rs:255-259`) on adwaita and the two macOS presets, where the two sources differ (D.4); the row without D.1: the same check. D.3 is text; no gate reads it |
| 3 — iced | E.2, E.3, E.4 (with the regeneration of `contract-leaves.toml`), E.5, E.6 | E.2 alone: `MENU_BORDER_ROWS`, and `UNREAD_COLOURS`' `popover` entry (a row now reads `popover.border.color`). E.3 alone: `AW_MENU_ROWS`, `AW_MENU_BORDER_ROWS`, the tripwire (the bar's border fields in no row and not in `DERIVED`), and the showcase's `the_extra_menus_are_the_themes_menus` and `the_extra_infos_say_what_the_page_draws`, which read `aw_menu_padding` and `aw_menu_info` (E.6). E.4 without the regeneration: `every_colour_leaf_reaches_a_row_or_is_named`. E.6's new tests without E.2 and E.3: decision 12's and decision 13's fail |
| 4 — outside the code | G.1 to G.3, G.5 | G.5's key paths need B.4 (`check_todo_citations.py`); G.5's dated lines name the merges of units 1 to 3 |
| 5 — close-out | H.4's asset run; G.4; G.6 | shared with the GPUI Kit port's work; after everything that lands in v0.7.1 |

**Unit 1's gate includes the carried files' check**, since no test catches a carried
file that is not byte for byte the clean room's. From the worktree root (zsh):

```sh
tail -n +13 native-theme/src/kde/shade.rs | sha256sum
# b1b8fa5228f019036fa5f4e23936bc470669ff2f115a80005d6937b03e05e7d9
head -n 3 native-theme/src/kde/shade.rs | sha256sum
# 92068628d9ea312fb03c4ef57c8718732b4ea4574825c22c652e721862dee073
sha256sum native-theme/tests/vectors/kde_shade.tsv
# 8328e762276bc22fee887e2c1dec1ff1436f911f9d028a428e343a15ebbe8913
```
The expected values were computed on 2026-11-14 from `v0.8.0-rc1`
(`git show v0.8.0-rc1:native-theme/src/platform/kde_shade.rs | tail -n +4 | sha256sum`,
`| head -n 3 | sha256sum`, and the table's `git show … | sha256sum`). The three output
lines go into unit 1's commit body, under "Carried byte for byte from v0.8.0-rc1
(78ee0efd):", so the review reads them with the diff.

**Unit 3 ends with a dry parity run**, so that the comparison the asset run makes at
rest (H.4) is not met for the first time in unit 5. It captures the three showcases
on the desktop and takes the focus from the user's windows (`scripts/capture_window.sh`,
which the screenshot scripts source, makes each showcase window the active one), so
it is announced to the maintainer before it starts and run when the desktop may be
left untouched. With units 1 and 3 landed (kde-breeze's new panel is unit 1's), from
the worktree root:

```sh
DRY=$(mktemp -d)
for kit in iced gpui egui; do bash scripts/generate_screenshots_$kit.sh "$DRY"; done
python3 scripts/check_showcase_parity.py "$DRY"
rm -rf "$DRY"
```
as `scripts/generate_assets_release.sh:136-166` runs them, on the presets the
screenshot scripts capture (`SCREENSHOT_ENTRIES`, `scripts/capture_window.sh:352-…`,
kde-breeze among them). The screenshot scripts write into the connectors' tracked
`docs/assets/` folders (their `OUTPUT_DIR`, e.g. `scripts/generate_screenshots_iced.sh:26`),
so after the run those files are restored with `git restore` on each folder and
`git status` shows them unchanged; nothing of the run is committed. A difference the
comparator reports is a fault, never a question (rule 19 (batch-speed rationale,
Appendix)): it is fixed, or, where it is a toolkit's limit, excepted with its reason
and source in a reviewed exception entry, and the maintainer is told either way.

Order: unit 1 first; units 2 and 3 after it in either order (they touch no file of
each other; unit 2 edits `connectors/native-theme-gpui/src/contract.rs` as unit 1 did,
at other lines); unit 4 after 1 to 3; unit 5 last.

Checked for every pair of units:

- Unit 1 alone, against the other crates: unit 1 changes kde-breeze's resolved
  `menu.background_color` and `list.grid_color`. gpui: the pair above (inside the
  unit); `chart_grid_is_the_list_grid_colour` still holds, as `chart_grid` still reads
  `list.grid_color`; the `table_row_border` row reads the field. egui: its routes and
  its menu pairs read the field on both sides (`asserted: false`, F). iced: unit 1
  does not change `contract-leaves.toml`. The walk splices a colour into the resolved
  theme's serialised form and reads it back without resolving again
  (`spliced_colour`, `connectors/native-theme-iced/src/contract.rs:4871-4882`), so a
  splice moves exactly the outputs that read the leaf. Before unit 3 the rows that
  read `menu.background_color` read it directly (`rows.rs:1517`, `:2604`, `:2610`),
  as the styles do (`styles.rs:1256`, `styles/aw.rs:144`), so any other colour moves
  them in every combination, whatever kde-breeze's own value; `list.grid_color` is
  `UNREACHABLE` (`derived.rs:1228-1234`). The export lists, per leaf, the rows its
  splices move; neither changes.
- Unit 2 alone: no other crate reads `chart_grid`.
- Unit 3 alone: no other crate reads the iced styles; the iced showcase's tests under
  kde-breeze read the installed theme, whichever unit 1 state it is in.
- Unit 1 then 3, or 3 then 1: the same end state; unit 3 regenerates the export.

### H.2 Citations of lines this step moves

None is re-pointed (the maintainer's S6, `docs/todo_v0.7.1_batch-speed-rationale.md`):
the citations other files make of the lines this step moves are left as they are and
may drift. The three gates that compared a facts line's number with its text stop
doing so before this step lands (the GPUI plan's unit LN, the GPUI specification's
Part I).

### H.3 Gates

From `scripts/check_release.sh` at `0fa28027`:

| Command (line) | Catches |
|---|---|
| `cargo test -p native-theme --features "$NT_FEATURES"` (`:683-684`) | the carried vector test; B.5 and B.6; the preset against the reader under both Breeze schemes |
| `cargo test -p native-theme` (`:675`) | the presets and inheritance tests without `kde` |
| `cargo test -p native-theme-gpui --features native,watch` (`:667`), `--lib --no-default-features` (`:704-705`), `--features gpui-fast,native,watch --all-targets` (`:714-715`) | D.1's test, the contract rows and pairs (D.2), `every_colour_claim_is_read_at_the_line_it_cites` |
| `cargo test -p native-theme-iced --no-default-features` (`:736-737`), `--features iced_aw,watch,native` (`:738-739`) | the rows, `DERIVED`, `UNREAD_COLOURS`, the pairs, `every_colour_leaf_reaches_a_row_or_is_named` against `contract-leaves.toml` |
| `cargo test -p native-theme-egui --no-default-features` and `--all-features` (`:759-762`) | the routes F names |
| the showcase tests in the runner environment (`:835-853`) | E.6's tests, the route tests, D.3 |
| clippy `-D warnings` (`:529-538`, the cross-target legs `:494-511`) and `check_strict_panic.sh` (the core `:573`, `:575`, cross-target `:593`, `:595`; the connectors `:616`, `:625`, `:627`, `:634`; the showcases `:596-607`, `:619-622`, `:629-640`) | A.4; the new reader and connector code |
| rustdoc under `-D warnings`, private items included (`:1002-1010`; the iced and egui all-features legs `:1015`, `:1019`) | the new docs' intra-doc links (B.2's helpers) |
| `python3 scripts/check_panic_patterns.py` (`:400`) | the carried file and the new code |
| the five documentation checkers (`:965-983`) | Part C, G.5 |
| `cargo fmt --all` | formatting |
| full run only: `scripts/update_provenance.sh check` (`:1302`) and `scripts/update_compatibility.sh check` (`:1339`) | both release stamps go stale with this step's edits (`docs/assets/PROVENANCE.toml`, `docs/COMPATIBILITY.toml`); while the CHANGELOG dates 0.7.1, the full run fails on each until the asset run re-stamps them |

**Not covered by any gate:** the wording of every comment, doc, README, CHANGELOG,
roadmap, backlog and rationale text above; the KDE cells of C.1 against the preset; the
link of C.1's new clause (opened by the implementer); and
the reader's exactness for non-Breeze schemes beyond the 260 vector rows.

Showcase texts this step changes that no existing gate reads, each with its check
(a manual reading, or a test row this specification adds):

| Text | Check |
|---|---|
| gpui `info/theme_map.rs:1246`, the Chart grid swatch's "model" row, `separator.line_color` (D.3) | by hand: in the running gpui showcase, the Theme Map page, the Info of the `chart_grid` swatch reads "model: separator.line_color" (the theme map's tests read swatch ids and claims, not this row, `tests.rs:11173-11200`) |
| iced `aw_menu_info`'s "bar padding" row (E.6, item 3) | a test, cheap: `the_extra_infos_say_what_the_page_draws` (`showcase-iced.rs:27836`) gains `(&menu, "bar padding", &["no frame"][..])`; its `line` helper finds a row by its label, which `widget_tooltip` writes as "  {what}: {val}" (`:9304-9313`) |
| iced `aw_menu_info`'s not-themeable "shadows" row (E.6, item 3) | the same test gains `(&menu, "shadows", &["popover.border.shadow_enabled"][..])` |
| iced `aw_menu_padding`'s and `menu_panel`'s doc comments, the `MenuBar` comment (E.6, items 1, 2, 4) | read in the diff against E.6's texts |

The iced showcase's route texts for the window menus (`:8034`, `:8037`, `:8046-8051`,
`:8443`) do not change (decision 12), and `every_hand_colour_route_names_a_function_that_reads_it`
(`:29102`) reads them.

### H.4 The asset run

Both stamps are stale from the first commit of this step until the asset run. The run
is shared with the GPUI Kit port's completion (`docs/todo_v0.7.1_gpui-kit-0.9.1-rationale.md`,
N8 "CHANGELOG, README, version and assets":
"the asset run is repeated once, after the last change of all four documents' work
that lands in v0.7.1, and the CHANGELOG's date follows it"): it runs once, after the
last change of everything that lands in v0.7.1, in the steps of the sibling
specification's *G.4 The asset run* (`docs/todo_v0.7.1_gpui-kit-0.9.1-spec.md`, Part
G): before the run, `cargo audit`, the compatibility run
(`scripts/update_compatibility.sh run`), the branch pushed once that run has passed,
the full `scripts/check_release.sh` while CI runs on it, with only the provenance
stamp failing (`docs/todo_v0.7.1_batch-speed-rationale.md`, S4), and the three
showcases built in advance; the asset run starts only when CI and the full check are
both green; then `scripts/generate_assets_release.sh`
(`check_release.sh:1415`) in the maintainer's capture window; then one commit of the
assets, both stamps and the 0.7.1 date, the full `scripts/check_release.sh` again,
and CI. What the new screenshots show from this step, for
the maintainer's look before the run is committed: under kde-breeze, the three
showcases' tables in Qt's grid and egui's popups in Breeze's menu panel; on every
preset, the iced Extra page's menu bar and every iced drop-down as E.7's table says;
the gpui Charts page's grids on adwaita and the macOS presets (screenshots of those are
taken only where the asset script captures them, not verified here).

**The parity comparison.** The asset script compares the three showcases with
`scripts/check_showcase_parity.py` on one run, the window at rest
(`scripts/generate_assets_release.sh:164`, given `$DUMP_DIR` alone: a `rest` run, the
script's docstring `:36-41`; a `menu=` run, a menu held open, is not part of it). What
step 1 changes there, from the code: the tables' grid, which all three paint from
`list.grid_color` (F), so the three move together; the window menu bars, whose fill
none of step 1's leaves feeds (iced's `menu_bar_style` fills the bar from
`window.background_color`, as before). The window menus' panels, were a `menu=` run
made, take `menu.background_color` in all three: iced `menu_bar_style`'s `panel`
(`showcase-iced.rs:6522`), gpui's `MenuLook` (`showcase-gpui/demo.rs:389-390`, "the
popup `menu.background_color` framed by the popover's border"), egui's base
`window_fill` (`connectors/native-theme-egui/src/style/base.rs:201`). Their frames
take `popover.border` in all three, which step 1 does not change: iced's
`menu_bar_style`, gpui's `MenuLook`, and the egui showcase's `menu_modifier`
(`connectors/native-theme-egui/examples/showcase-egui/demo.rs:371-395`: the popup's
`window_stroke` from `popover.border`'s colour and width, `menu_corner_radius` from its
radius, `menu_margin` from its padding), which its window menu bar applies
(`demo::menu_bar`, `demo.rs:748-750`, called from `chrome.rs:330`). So step 1 does
not move the three showcases' window menus apart: their panels change together, from
the one leaf. The Extra page and the drop-downs, which change, have no counterpart the
comparison pairs them with at rest (not verified beyond the docstring and the call).

## What was not verified

- That the carried file passes this tree's clippy, strict-panic and `-D warnings` runs,
  and its vector test (no cargo run was allowed); A.4 gives what was read instead.
- That `kcolorscheme.cpp:519-527` at `27066d47` is `contrastF` (C.1), and the
  `[KDE] contrast` lines of the Breeze schemes beyond what the facts row cites.
- The resolved values of D.4 and E.7 were computed from the preset files by the
  documented rules, not by the crate's resolver; the gpui pair ratios in D.2 to the
  second decimal, not through gpui's `Hsla`.
- That the Extra page's context-popup test still finds its pixels with a 2 px popover
  line (E.6, the Extra page's item 5); the test is its own proof.
- Whether the combo box popup's measured view colour is the whole popup's, and whether
  Breeze rounds a combo popup's highlighted row or its corners (the rationale's §4,
  unchanged).
- Qt Quick's menu fill (C.1, G.5).
- The drop-down labels' contrast figures for option (ii) (24 of 25 combinations, D3)
  were computed by the second confirming check from the preset files and the
  documented inheritance, not through the crate's resolver; adwaita's dark
  highlight (1.06 against 1.40) and E.7's line contrasts were computed today.
- That pull request #10 is the branch's (Cost and order): taken from the project's
  records, not looked up (no network).
- How KConfig converts a `contrast` with a fraction, `4.5` (decision 14); a value it
  cannot convert gives the default, by the reader's own citation of kconfig.
- That `cargo package` lists `tests/vectors/kde_shade.tsv` (A.2); read from the
  manifest only.
- How rustdoc's builds treat an unfulfilled `#[expect]` (decision 3); the
  specification keeps `allow`, which does not depend on it.
