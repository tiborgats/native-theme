# v0.7.0 — GPUI Kit 0.7.3: Rationale

**Status:** design, reviewed 2026-10-13 (a fact check, a completeness audit, a design critique
and a cold verification, every finding applied; a second review against the tree after M4,
`df86348c`), awaiting the maintainer's approval; nothing implemented. The maintainer approved
G2's API breaks and the pushes and probe dispatch during G1 (2026-10-13).
What to build: [`todo_v0.7.0_gpui-kit-0.7.3-spec.md`](todo_v0.7.0_gpui-kit-0.7.3-spec.md).
Tasks and gates: [`todo_v0.7.0_gpui-kit-0.7.3-plan.md`](todo_v0.7.0_gpui-kit-0.7.3-plan.md).

GPUI Kit v0.7.3 was released on 2026-10-12. Its release notes call v0.7.2 yanked and say to
upgrade straight to 0.7.3 (crates.io shows 0.7.2 not yanked); it moves GPUI to the `gpui-pre`
0.3.9 snapshot and adds UI elements and an optional backend
([release notes](https://github.com/longbridge/gpui-kit/releases/tag/v0.7.3)). The
maintainer's brief (2026-10-12): be compatible with 0.7.3, support its `gpui-fast` backend
with a feature of our own, and give every UI element it introduced native values. The work
lands on `v0.7.0-rc1` as two milestones of the whole-codebase review
(`docs/todo_code-review-plan.md`), after M4: **G1** (compatibility, `gpui-fast`, `Diff`) and
**G2** (new theme values), in the order G1 → M5's fix phase → G2 → M6, M7 (L10).

---

## 1 -- What was measured (2026-10-12/13)

Measured in this design session unless a research report is named: the compile trials, the
test run, the MSRV checks and the citation inventory were run on 2026-10-12.

### 1.1 Compatibility

- **The released connector is not broken.** `native-theme-gpui = "=0.6.1"` with
  `gpui-component = "=0.7.3"` resolves gpui-pre 0.3.9; `cargo check` passes. No 0.6.x patch
  release is needed.
- **`v0.7.0-rc1` builds unchanged** with only the five version requirements raised:
  `cargo check -p native-theme-gpui --all-targets --all-features` (no gpui-fast feature exists
  yet, so this is the gpui-pre build) has no error or warning, and
  `cargo +1.95.0 check -p native-theme-gpui --all-features --locked` passes: the MSRV stays
  1.95.0.
- **What fails:** the showcase's 181 tests and the 16 seam tests pass; 4 of 291 library tests
  fail, each a gate — two README floor tests, the colour-claim gate (140 of 738 claims), the
  prose-citation gate (the tooltip panel cites gpui-base's `SHOW_DELAY`, which 0.7.3 replaced
  by `TooltipDefaults`). The widget-coverage gate misses `Diff`.
- **Upstream:** no theme file changed (`ThemeColor`, `Theme`, `MotionTokens`, gpui-base
  `Theme`) and no icon was added.
- **Citations** (recounted on 2026-10-13 at `df86348c`; M4 removed three): 830 `file:line`
  citations. 212 match a file 0.7.3 changed by path suffix; 186 of them resolve to exactly one
  changed gpui-component or gpui-base file (124 moved, 42 unmoved, 16 whose text now repeats,
  4 whose path exists in both crates); the other 26 match by suffix only and are read by hand. gpui-pre 0.3.8 → 0.3.9 moved lines the connector cites
  too (at least nine ranges, e.g. `app.rs:1237` → `:1308`). 71 prose mentions state "0.7.1" as
  the verified version, several of them measurements ("ThemeColor has 139 fields", "86 sites
  outside `theme/`") that must be re-run, not renumbered.

### 1.2 `gpui-fast`

From a throwaway implementation in a worktree (deleted afterwards):

- **The connector needs support.** Cargo unifies features: with `gpui-component/gpui-fast`
  enabled while the connector stays on gpui-pre, the connector fails with 365 errors (e.g.
  `expected gpui::window::FocusHandle, found gpui::FocusHandle`).
- **gpui-fast 0.1.7** is a fork of gpui-pre 0.3.9 whose library is also named `gpui`, with a
  superset API; the ~85 `gpui::` items the connector and showcase use compile unchanged.
- **The measured form** (variant B): an optional `gpui-fast` dependency, a feature forwarding
  to gpui-component, gpui-base and gpui-kit, and `extern crate gpui_fast as gpui;` in the
  library root, `tests/seams.rs` and the showcase's `main.rs`. Clippy, the strict panic lints
  and `cargo +1.95.0 check --features gpui-fast --locked` pass; default builds contain no
  gpui-fast crate.
- **Tests on GPUI Fast:** six fail (one checkbox test and five showcase absence tests) until the two test draw helpers call `window.refresh()`
  before `window.draw(cx)`: GPUI Fast reuses unnotified subtrees, and in test mode carries the
  previous frame's element bounds forward when it reuses part of a frame
  (`gpui-fast-0.1.7/src/fast/retained.rs:1360-1371`). With the fix all pass on both backends.
- **Side effects:** `--all-features` now selects GPUI Fast (docs.rs, CI's MSRV job,
  `cargo semver-checks`, `scripts/check_features.sh` all use it); the workspace lockfile takes
  `unicode-properties` 0.1.3, pinned by `gpui-fast-web` (not connector-local: `rustybuzz`, used by
  core's `svg-rasterize` path, depends on it too); doctests are separate crates and not
  aliased. GPUI Fast was never run, only built and tested.
- **An alternative upstream documents:** gpui-kit re-exports whichever core it is built on
  (`pub use ::gpui::*;` / `pub use ::gpui_fast::*;`, `crates/kit/src/lib.rs:100-103`) and
  supports aliasing it as `gpui` (`:94`). A connector that names GPUI through gpui-kit follows
  the application's backend through feature unification. Not yet tried.

### 1.3 The new and changed elements

| Element | What it reads | Native support today |
|---|---|---|
| `Diff` (new) | `Styled` on its outer frame; colours `background`, `foreground`, `border`, `muted`, `muted_foreground`, `list_hover`, `selection`, `danger` / `success` (removed / added lines), `info`, `primary`, `radius`; `mono_font_family`, `mono_font_size`, `highlight_theme`; 66 literals | every colour from fields the connector maps; its code size is `rem × mono_font_size / 16`, so it differs from `defaults.mono_font.size` whenever the UI font is not 16 px (smaller below) |
| Dialog entrances (new) | `entrance(DialogEntrance::{SlideDown (default), Fade, FadeSlide, None})`; `Fade` / `FadeSlide` use `motion.duration_normal`; all skipped under `cx.reduce_motion()`; `AlertDialog::margin_top` | default `SlideDown` (travels about a tenth of the window); FadeSlide travels `motion.distance_short`, 3–4 px |
| Tooltip timing (new) | `TooltipDefaults`, a `Global` installed with `install(cx)`: show delay 500 ms, grace 300 ms (`GB/tooltip.rs:58-107`); per-trigger `tooltip_show_delay` | upstream's defaults; GPUI's own `.tooltip()` (used by gpui-component's Attachment and the showcase) defaults to 500 ms and takes a per-element `tooltip_show_delay(Duration)` (gpui-pre 0.3.9 `src/elements/div.rs:735, 1708`) |
| DataTable header row height (new) | `header_row_height`, else the body row height | the body row height |
| Sort button, right-clicked cell outline | look unchanged: sort `secondary_foreground`, `tokens.secondary*`, `radius / 2`; outline `table_active_border` at 0.5 | mapped colours; no showcase claims |
| TextView typography (changed) | gpui-component's `sync_base` installs a `TextViewDefaults` built from the theme (`theme/mod.rs:477`): selection, table head from our mapped colours; headings body × 1.8 … 13⁄15; paragraph gap 0.75 rem | native through the mapped colours and the rem (`defaults.font.size`); `link.underline_enabled = false` and the selection's text colour cannot be honoured |

gpui-base's tooltip **grace period** (`tooltip.rs:80-83, 231, 256-279`) does two things: the
tooltip *stays visible* that long after the pointer leaves, and a trigger entered meanwhile
shows its tooltip at once. A platform's reshow window has the second effect without the first.

### 1.4 Platform facts

- **Tooltip show delay:** WinUI 3 2 × `SPI_GETMOUSEHOVERTIME` (800 ms at WinUI's 400 ms
  fallback constant), Qt and Kirigami 700 ms, GTK 4 500 ms, Compose 0 (shown on pointer enter);
  macOS and iOS undocumented. A reshow behaviour exists on WinUI, Qt and GTK, but each anchors
  and delays it differently (WinUI: within 200 ms of a close, then 1 × hover time; Qt: for
  2000 ms after a tooltip was *shown*, a 20 ms wake-up; GTK: browse mode, 60 ms, ending 500 ms
  after a hide).
- **Dialog entrance:** scale + fade on WinUI (`ContentDialog`) and libadwaita (floating
  `AdwDialog`), both in-window dialogs; KDE's in-window dialog, `Kirigami.Dialog`, fades in while rising
  2 grid units into place (KWin's default-on Scale effect animates toplevel windows such as a `QDialog`); material-web
  slides down 50 px; Compose's `AlertDialog` defines none (Android's platform window animation
  applies); macOS and iOS undocumented.
- **Column header height:** libadwaita 30 (a stylesheet sum), Material 3 56 (token), Breeze
  content-sized (font height + 12), WinUI none (no control), macOS queryable at run time.

---

## 2 -- Decisions

### L1 — Floors: GPUI Kit 0.7.3 on gpui-pre 0.3.9 (G1)

The connector builds on 0.7.1 too, but upstream says to move to 0.7.3, every citation is
renumbered to 0.7.3, `gpui-fast` needs 0.7.3, and 0.7.3 pins `gpui-pre = "=0.3.9"`. The
gpui-pre requirement stays a caret.

### L2 — Citations by content; prose re-measured (G1)

Moved `file:line` citations (gpui-component, gpui-base and gpui-pre) change digits only, in
one commit gated on that. The hand-read ones, the tooltip claim and the 71 version-stating
prose mentions are rewritten or re-measured in the next commit. Claims in the panels of
widgets whose behaviour changed are re-read even where their line did not move.

### L3 — `gpui-fast`: follow the application's backend, with our own feature (G1)

The best outcome is a connector that needs no action: an application that turns on
`gpui-kit/gpui-fast` gets a working connector. So G1 first tries upstream's documented alias
(**form A**): the connector names GPUI as `extern crate gpui_kit as gpui;` (gpui-kit as a
normal dependency with default features off), so it follows gpui-kit's backend; the
connector's own `gpui-fast` feature forwards `gpui-kit/gpui-fast` and `gpui-component/gpui-fast`
(gpui-kit forwards to gpui-component only weakly when its `component` feature is off), for
applications that enable the backend on gpui-component rather than gpui-kit, and for CI. Its
cost: gpui-kit depends on `gpui-pre-platform` (font-kit, X11, Wayland), so form A puts the
platform layer into every library build of the connector and into docs.rs's — free for an
application, which links it anyway; docs.rs already builds gpui-kit 0.7.3 itself, so it can
build that layer. Form A's selling point gets its own check: the library builds with
`--features gpui-kit/gpui-fast,gpui-kit/component` (an application enabling the backend on
gpui-kit only), which must pass under form A. If form A fails any gate —
compiles, tests on both backends (after the test-helper fix), the `#[gpui::test]` macro paths,
MSRV, packaging, the gpui-kit-only check — G1 takes the measured **form B** (§1.2). Either way the maintainer's
requirement holds: a `gpui-fast` feature of our own. Both forms need the test-helper fix, an
explicit docs.rs feature list (the crate is documented against gpui-pre), `--default-features`
for `cargo semver-checks`, a `check_features.sh` that excludes `gpui-fast` from its
all-features run, CI legs for the second backend, and one real run of the showcase on GPUI
Fast (a theme switch and a layout dump compared with gpui-pre's), since it was never run.
The `unicode-properties` 0.1.3 pin is accepted with a todo entry to revert it when upstream
unpins.

### L4 — Tooltip show delay becomes theme data; nothing else about timing (G2)

One field, **`tooltip.show_delay_ms`** (the Rust field `show_delay: Option<u16>`, serialised with the `_ms` unit as sizes carry `_px`). It has a receiver in every connector
(gpui `TooltipDefaults::with_show_delay` for gpui-component's tooltips and GPUI's own
`tooltip_show_delay` per element, egui `interaction.tooltip_delay`, iced `Tooltip::delay`) and a sourced value on four platforms. No reshow field: the platforms do not
share one concept (§1.4), and gpui has no receiver for it alone (its grace period also keeps
the old tooltip visible, so a KDE value of 2000 would leave a stale tooltip up for 2 s), and an upstream PR to separate
gpui's two notions is listed instead. Integer milliseconds like every source; no upper bound
(none is sourced; `u16` caps it at 65 s). No live Windows read: it would need a new `unsafe`
call in native-theme for a setting almost nobody changes, so `windows-11` states WinUI's
default (2 × the default hover time, which the second probe run, spec §6, records). The connector replaces only
the show delay in the installed `TooltipDefaults`, so an application's own grace period
survives; and it tells an application's delay from its own by remembering what it last wrote
(as its `ReduceMotionRequest` records whether the connector itself switched the flag): a delay
in the global that differs from the connector's last write is the application's, kept when a
theme states none, so an application's choice is never overwritten by upstream's 500 ms. Hide times are not modelled.

### L5 — Dialog entrance becomes a coarse theme value (G2)

**`dialog.entrance: Option<DialogEntranceStyle>`** with `ScaleFade` (WinUI's `ContentDialog`,
libadwaita's floating `AdwDialog`, both in-window) and `FadeRise` (KDE: its own in-window
dialog, `Kirigami.Dialog`, fades in while rising 2 grid units into place, `Dialog.qml:303,
306-311` at the KIRI pin; KWin's Scale effect applies to toplevel windows such as a `QDialog`,
not to an in-window dialog, which is what gpui's is). gpui is the only receiver, and it has neither a scale nor a rising entrance (`FadeSlide`
settles from *above* by 3–4 px), so today every stated value reaches gpui as
`DialogEntrance::Fade`, the closest without travel; the field keeps the platforms' distinct
facts for a receiver that can draw them, and a `Scale` entrance goes on the upstream PR list.
The visible effect today: on Windows, GNOME and KDE gpui dialogs fade in instead of sliding
down. `SlideDown` is a third value only if G2's
facts task finds a Material 3 source in Google's Android implementations (Compose,
material-components-android) rather than material-web alone, because M2d re-bases the
Material column on Compose; it would map to gpui's `SlideDown` (whose travel, about a tenth
of the window, is closer to material-web's 50 px than FadeSlide's 3–4 px). Unset elsewhere,
so gpui's default applies. The enum is named `DialogEntranceStyle` so it does not clash with
gpui-component's `DialogEntrance`, and follows the model's other enums (`ArrowSide`): not
`#[non_exhaustive]`, matched exhaustively. This is the weakest of the new values (one receiver,
and both its stated values reach it approximately); it is in because the brief asks for every new element to be native.

### L6 — List header height becomes theme data, after a probe (G2)

**`list.header_height_px: Option<f32>`** (logical pixels, like every size). Receivers:
gpui `DataTable::header_row_height` (a new builder: `data_table_size` returns a `Size`), egui
`TableBuilder::header(height, …)`, and iced, whose `table` widget and showcase build header
cells at an explicit height. Values: libadwaita 30, Material 3 56 (material-web's token; M2d decides it against Compose,
which has no data table), macOS from the OS probe (a `.fullWidth` table, the document-table
style; macOS 14 and 15 must agree for `macos-monterey`);
**KDE unset**, as `list.row_height_px` already is (`kde-breeze.toml`: rows size to their
content), so the toolkit's height applies; Windows and iOS unset (no such control). A height,
not a padding, because every receiver takes a height.

### L7 — TextView: nothing to install (G1)

gpui-component's own `sync_base` already builds TextView's defaults from the colours the
connector maps, and its body text scales from the rem the connector sets from
`defaults.font.size`. Installing our own defaults would mean copying upstream's crate-private
style recipe and would drop its syntax highlighter. Headings keep TextView's ratios (the
maintainer: editors use other heading sizes than the OS). What cannot be honoured is recorded:
`link.underline_enabled = false` (TextView always underlines), the selection's text colour
(the selection paints under the glyphs).

### L8 — `Diff`: shown, held by the gate, its size reported upstream (G1)

The showcase shows `Diff` on the Typography page with a Widget Info panel. Its code size
cannot be fixed from outside without changing every other monospace text: an upstream PR.

### L9 — Excluded, with reasons (upstream items go on the upstream-PR list)

- A readable error when only gpui-kit is on GPUI Fast (form B): a gpui-fast marker constant
  upstream would let the connector fail with a message; the README covers it meanwhile.
- A TextView rule colour separate from `border`: upstream item.

- `AlertDialog::margin_top`: no platform-facts row for dialog placement.
- `RoundedFrameCover`: its backdrop falls back to `background`, already native.
- `<mark>`'s default `#fef08a`, code-block and inline-code fills: no model field.
- `MotionTokens` (entrance duration and easing, the TabBar reveal's spring): the platforms'
  mechanisms differ (storyboards, springs, compositor effects); no field could state them.
- TabBar reveal: upstream's is private; the connector's own TabBar does not clip.
- The sort-indicator size and a right-clicked-cell outline: stated by too few platforms or
  none; their colours get showcase claims.
- Completion popups, ContextMenu, Dock, DropdownButton, Settings: behaviour or accessibility
  only.
- Reduced motion: every new animation obeys `cx.reduce_motion()`, which gpui-base fills from
  the OS (unchanged since 0.7.1).

### L10 — Two milestones, and where they sit among M5–M7

G1 changes the gpui connector and the OS probe (and native-theme only if M4's first CI run
fails, G1 Task 1) and needs no maintainer step to finish; it adds the OS probe's two
new measurements early, so the probe run (for G2) overlaps G1 instead of blocking G2. G2
changes native-theme core: an API break (the widget structs are not `#[non_exhaustive]`, so a
new field breaks struct literals), which the maintainer approved on 2026-10-13 under the review
plan's D1.

Order: **G1 → M5's fix phase → G2 → M6, M7.** M5's review phase (the iced review) ran in
another session while these documents were reviewed (the maintainer's choice, 2026-10-13) and
is at stop point A (`308c988d`); its fixes wait for G1 to close, because two fix phases committing to one branch from one
working tree would interleave, and `check_release.sh --commit` would see the other session's
uncommitted work. G2 follows M5's fixes, so its iced receivers land on reviewed code and are
held by G2's own per-task review. M6 (gpui) and M7 (egui) follow G2, so they review connectors
that already have the new receivers; egui's receivers are part of G2, not M7, because egui's
mapping contract is exhaustive over model leaves and the showcases must follow theme data.
M5's fix phase lands M1-T7-06 (connector accessors named after the model's fields, held by
`scripts/check_connector_parity.py`) and possibly M1-T7-03 (model-leaf coverage); G2's new
accessors and contract rows follow whatever rules those leave.

---

## 3 -- Out of scope

- A 0.6.x patch release (§1.1).
- Reshow and hide tooltip times; heading fields; paragraph spacing; the items of L9.
- A live Windows hover-time read (L4); a later todo entry if wanted: it needs the maintainer's
  approval for its `unsafe` (with the `SAFETY:` comment `unsafe_safety_comments.rs` now
  enforces) and a watched source (`WM_SETTINGCHANGE` / `SPI_SETMOUSEHOVERTIME`, the M4 rule that
  every reader source has a watcher).
- `Diff`'s review features beyond one patch; `examples/tig`.
- Filing the upstream PRs and the gpui-fast issue: drafted; the maintainer files them.
