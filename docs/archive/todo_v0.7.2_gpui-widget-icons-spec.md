# 0.7.2: gpui-component's own icons follow the theme's icon set — specification

- Rationale: [`todo_v0.7.1_gpui-widget-icons-rationale.md`](todo_v0.7.1_gpui-widget-icons-rationale.md)
  ("the rationale"), decisions W1 to W11, §3 (what the maintainer approves) and §4
  (what was not checked). Approved by the maintainer with the rationale and the plan
  on 2026-11-19. It
  decides nothing the rationale did not decide, except the small points under
  *Decisions taken while specifying*; what needs a decision the rationale does not
  contain is under *Open points*.
- It builds on the GPUI Kit 0.9.1 specification,
  [`todo_v0.7.1_gpui-kit-0.9.1-spec.md`](todo_v0.7.1_gpui-kit-0.9.1-spec.md)
  ("the 0.9.1 specification"), whose parts landed in v0.7.1 and are in the tree: B.13 (the connector's
  `widgets::Checkbox` asks the icon resolver for its check mark: W8's first point),
  B.3 (the Widget Info's `own_icons` text, written for W11 (b)), B.15 (the citation in
  `SC/info/icons.rs:15-17`), D.3 and D.4 (the backlog's two upstream entries on the
  widgets' icons: `docs/todo.md:3972` closes, `:3812` is rewritten) and E (the
  renumbering of upstream citations, `SC/info/` left out). None of them is repeated
  here; where this document edits a text one of them wrote, it names that part and
  quotes the text that part gives.
- Written: 2026-11-14, against `v0.7.1-rc2` at `0fa28027`; every repository `file:line` is
  that commit's unless it names another, and stays so. Amended 2026-11-19 for the fold into
  0.7.2 (below), on `v0.7.2-rc1` at `a3fc81f6`: the amended passages were read on that tree
  and name a place by its file and a quoted phrase or an item, never by a line. Every
  upstream line is read in Cargo's registry at the versions under *Abbreviations*. Commands
  are run from the worktree's root.
- **Line numbers are for finding the place, and they are `0fa28027`'s.** The tree has moved
  since: v0.7.1's close-out, egui 0.37, GPUI Kit 0.10.0 and the KDE steps 2 to 5 are all on
  `v0.7.2-rc1`, so most lines differ, and none is re-pointed. Every edit is applied by the
  text it quotes, the item it names or the test's name. Where each anchor stands at
  `a3fc81f6` is listed in the two reviews of 2026-11-19 (the plan's record,
  `icons/review-partA.md` and `icons/review-partEG.md`), which the briefs give as a lookup;
  a passage amended on 2026-11-19 is found by the function or text it names.
  Where a quoted text no longer stands, the implementer stops and reports. A count is the
  one read on the tree at the unit's commit.
- **Release: inside 0.7.2.** The maintainer decided on 2026-11-19 to build this work in
  0.7.2, which supersedes the rationale's W11 (b) and, for this work, the lane that S3 of
  `docs/todo_v0.7.1_batch-speed-rationale.md` gave it. The units are commits on
  `v0.7.2-rc1` in the worktree `.claude/worktrees/lane2-b`, after the KDE steps 4 and 5:
  no branch, no merge and no version step of their own. The version stays 0.7.2, and
  0.7.2's one close-out (its full check, asset run, stamps and archive row) closes this
  work too (Part G). The batch's working rules are cited as "rule N"
  (`docs/todo_v0.7.1_batch-speed-rationale.md`, *Appendix — the batch's working rules*).
- Every value below was read today from the file named, or computed today; the
  computations are named where they are given. What could not be verified is marked
  so, with the reason. No cargo command was run. The amendment of 2026-11-19 ran none
  either.

**Abbreviations.** As in the rationale: `GC`, `GB`, `GP`, `GF` are the `src/` folders of
gpui-component 0.9.1, gpui-base 0.9.1, gpui-pre 0.3.12 and gpui-fast 0.1.15 in
`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`; `GA` is gpui-kit-assets
0.9.1; `CN` is `connectors/native-theme-gpui`; `SC` is `CN/examples/showcase-gpui`; `NT`
is `native-theme`.

The lines were read at those versions and stay theirs. The tree at `a3fc81f6` builds on
gpui-component, gpui-base and gpui-kit-assets 0.10.0 and gpui-fast 0.1.16 (gpui-pre is still
0.3.12). Every upstream citation of this document and of the rationale was read again there on
2026-11-19: of 153, 106 stand as written, 45 stand at other lines or with additions (the
corrections below and Appendix 2's note name the additions), and 2 were false at 0.10.0 and
are corrected (the loading button, E.4). A text this work writes into the tree cites 0.10.0
(F.5 item 1).

## How to read it

Seven parts. A to E are the work, by owner; F is the work outside the code; G the
order, the gates and the close-out. Two appendices hold the per-name table and the list
of what gpui-component's widgets draw.

| Part | Rationale | Files it owns |
|---|---|---|
| A — The core's new method | W4 ("Passing over") | `NT/src/freedesktop.rs`, `NT/src/icons.rs` (the method and its tests) |
| B — The lookup and the one-colour test | W3, W6, W9 | `CN/src/icons.rs` (two table entries, the lookup, the test, their tests), `scripts/connector_parity_exceptions.toml` (one entry's name list, one new entry) |
| C — The measurement, the first stop | W3 "The measurement", W9, §4 | `SC/tests.rs` (one ignored test, appended), new `docs/evidence/widget-icons/<date>/` (a table and its README; the sheets stay in the git-ignored `target/`), `docs/evidence/README.md` (one section) |
| D — The handle | W1, W2, W4, W5 | new `CN/src/icons/widget.rs`, `CN/src/icons.rs` (module doc, the module's declaration and re-export), `CN/src/lib.rs` (`apply_system_theme`, the doctest gate, the crate doc), `CN/src/widgets/checkbox.rs` (two doc comments) |
| E — The showcase | W1, W3 "The showcase's two lists", W6, W9, W10 | `SC/main.rs`, `SC/app.rs`, `SC/support.rs`, `SC/demo.rs`, `SC/pages/buttons.rs`, `SC/pages/icons.rs`, `SC/info/mod.rs`, `SC/info/chrome.rs`, `SC/info/icons.rs`, `SC/info/buttons.rs`, `SC/tests.rs`, `docs/showcase-exceptions.toml` (two reasons); by E.3's rule, the `SC/info/` files whose texts speak of an image (`data.rs`, `inputs.rs`, `feedback.rs`, `layout.rs`) |
| F — Documentation | W2, W3, W4, W7, W8, W9, W11 | `CN/README.md`, `CHANGELOG.md`, `ROADMAP.md`, `docs/todo.md`, `docs/issues/0004_kde-open-items-rationale.md`, `docs/archive/` |
| G — Order, gates, close-out | — | the units on `v0.7.2-rc1`, their gates, what 0.7.2's close-out takes from this work |

Every edit has one owner. `CN/src/icons.rs` is edited by B (the tables, the lookup,
tests) and by D (the module doc, `mod widget;`, the re-export), at places that do not
meet; `SC/tests.rs` by C (one ignored test) and E. No unit re-points a line citation its
edits move (G.2, S6).

**Lines that must not move.** The gpui showcase's test
`every_colour_claim_is_read_at_the_line_it_cites` (in `CN/src/showcase.rs`) opens each colour
claim at the connector line it cites, and fails where a claimed line has moved. The claims
cite lines of `colors.rs`, `geometry.rs`, `variants.rs` and eight files of `widgets/`; none
cites `CN/src/icons.rs` or `CN/src/lib.rs`. Of those files this document edits one,
`widgets/checkbox.rs`, in two doc comments (D.11): one above the lines the claims cite,
replaced line for line, and one below every line they cite. So no claimed line moves; a unit
whose edit would move one stops and reports.

## Decisions taken while specifying

Each is the smaller or the safer choice and changes no result of the rationale; the
options are named where there were more than one.

1. **A. The core method is `FreedesktopLoader::load_where(self, accept)`.** It reuses
   the loader's own order functions, `first_of_names` (`NT/src/freedesktop.rs:336-348`)
   and the two passes of `find_icon_with` (`:290-308`, extracted into a function of the
   same body, A.1), with a lookup closure that keeps a found file only where its loaded
   icon passes `accept`. So the order of the theme's icons is one piece of code, as W4
   asks ("the order of the theme's icons stays one"), and `load` is not touched. The
   other way weighed, a second copy of the order inside the new function, would let the
   two drift.
2. **A. A file that does not load counts as an icon that did not pass**, and the search
   goes on. `load` returns `None` there (`load_freedesktop_icon_by_name`, `:392-393`:
   the first path found is the only one loaded); `load_where(|_| true)` therefore
   differs from `load()` in that one case, which its documentation states. An unreadable
   file is no icon to test.
3. **B. The lookup is not behind `svg-rasterize`; the handle is.** W2 puts the handle
   behind the feature. The lookup is also the showcase's, and the showcase must build
   without the feature: `scripts/check_release.sh:549-550` lints the connector with
   `--all-targets --no-default-features`, which builds the showcase, and its comment
   says that run is the only one that compiles the showcase's code for builds without
   `widgets` and the bundled sets. Without the rasteriser no SVG can be judged, so the
   lookup's result says "not judged" (`GpuiIcon::mask == None`) and `IconUse::Mask`
   finds nothing. The other way, `required-features = ["svg-rasterize"]` on the
   showcase, would drop it from that run.
4. **B. The one-colour test is rasterised at 16 px** (`ONE_COLOUR_TEST_SIZE`), the
   handle's lookup size (W9); W6 asks for "once, small". The judgement does not depend
   on the size an icon is drawn at, so one size serves the handle and the showcase.
5. **B. The deviation is measured weighted by alpha.** native-theme's rasteriser returns
   straight alpha, divided back from resvg's premultiplied pixels with integer
   division (`NT/src/rasterize.rs:98-100`, `NT/src/color.rs:329-349`), so a pixel of
   low alpha can come back with a channel far from the icon's colour (at alpha 1 every
   channel is 0 or 255). A channel's difference times the pixel's alpha
   (`a × |c − C|`, 0 to 65 025) is what that division leaves bounded. The reference
   colour is a pixel's too, so where it is not opaque it went through the same division;
   the difference is taken less that pixel's own rounding (B.3). The tolerance is
   then the largest such value the bundled sets give (B.3), not a number chosen here.
6. **B. An embedded image is found in the text**: an element named `image` (B.3). W6
   says such an SVG counts as not drawable; the rasteriser cannot see it (it is built
   without `raster-images`, `NT/Cargo.toml:74`), so only the text shows it. A mention in
   a comment counts too, which errs towards passing the icon over.
7. **C. The measurement is an ignored test of the gpui showcase**,
   `widget_icons_listing` in `SC/tests.rs`. It needs the connector's lookup (built
   before it) and the showcase's own: its list of roles, `role_for_gpui_icon`, and the
   names its slots that take an `Icon` alone draw (W1's item of the listing), which only
   the showcase knows. A connector example would need a copy of those. The test reads
   the installed themes only through native-theme and the connector.
8. **D. The handle's state is shared through an `Arc` holding an `RwLock<Arc<State>>`** (and,
   beside the lock, the flag of D.6 and the load counter as atomics, D.4). The connector
   shares no state across threads today (no `Mutex` or `RwLock` outside the tests of
   `CN/src/watch.rs`); the core keeps its caches in `Mutex`es and treats a lock it
   cannot take as a miss, never unwrapping it (`NT/src/freedesktop.rs:87-99`,
   `:215-227`). A reader (the resolver, the asset source) takes the read lock, clones
   the inner `Arc` and lets go; a writer (the handle's methods, on the main thread)
   builds a new `State` and replaces the `Arc` whole. A poisoned lock is read through
   `PoisonError::into_inner`: the one write under the lock is an assignment, which
   cannot leave a half-written value, so the value it holds is whole. `AssetSource`
   requires `'static + Send + Sync` (`GP/assets.rs:13`, `GF/assets.rs:13`); a global
   requires `'static` (`GP/global.rs:22`, `GF/global.rs:22`); the resolver is kept in an
   `Rc` (`GC/icon.rs:77`) and needs neither `Send` nor `Sync`.
9. **D. An empty icon is a path the handle's asset source answers with `Ok(None)`.** On
   both cores, an `Icon`'s path reaches `Window::paint_svg` with no bytes
   (`GP/elements/svg.rs:182-185`, `GF` the same lines), whose atlas build closure asks
   `SvgRenderer::render_alpha_mask`, which asks the asset source and returns `Ok(None)`
   where it answers `Ok(None)` (`GP/svg_renderer.rs:255-261`, `GF/svg_renderer.rs:255-261`);
   the atlas then returns `Ok(None)` without inserting a tile
   (`AtlasState::get_or_insert_with`, `GP/platform.rs:1864-1866`,
   `GF/platform.rs:1882-1884`; every platform atlas of both cores delegates to it, read
   in gpui-pre-wgpu, -windows, -apple and gpui-fast-wgpu, -windows, -apple), and
   `paint_svg` returns `Ok(())` (`GP/window.rs:5254-5265`; `GF/window.rs` the same code
   at `:5306-5314`). Nothing is drawn and nothing is logged. The other option, an empty
   SVG, was not read through usvg (whether an `<svg>` with no children parses was not
   checked), and it would take a tile per size. The asset source is asked again each
   frame for that path; its answer is a map lookup.
10. **D. The served paths begin with `native-theme-gpui/`.** Every gpui-component and
    gpui-kit-assets path is `icons/<stem>.svg` (`GA/build.rs:48`; the whole catalogue is
    embedded from `icons/**/*.svg`, `GA/src/native_assets.rs:7-11`; all 106 lines of
    `GA/default-icons.txt` begin `icons/`), so no upstream path can be one of them; the
    README reserves the prefix for the connector.
11. **D. The resolver reads the mode as the connector's observer does**:
    `NativeTheme::is_dark` (`CN/src/lib.rs:998-1002`, the `GpuiTheme` global's mode,
    else the mode `apply` last installed) where a `NativeTheme` is installed, else the
    `GpuiTheme` global's mode; with neither, the handle answers "not mine" (without the
    `GpuiTheme` global no gpui-component widget draws). The new module is a descendant of
    the crate root, so it calls the private method.
12. **D. The list of the 106 `IconName`s becomes a library constant**,
    `GPUI_ICON_NAMES` (crate-private), since the handle loads every name and
    gpui-component's `IconName` has no list of its own. It is a plain list, not the test
    module's macro (`CN/src/icons.rs:1199-1217`), whose exhaustive `match` is what makes
    the dependency canary report a new variant and must not break an application's build;
    a test holds the two lists equal.
13. **D. The report names icons by their asset paths** (`icons/check.svg`), the strings the
    resolver and the lookup speak: gpui-component's `IconName` derives `Clone` alone
    (`GC/icon.rs:17-18`), so a report holding it could derive neither `Debug` nor
    `PartialEq`.
14. **E. The showcase keeps its handle in a global of its own**, `ShowcaseWidgetIcons`,
    set by `main` before the window opens and by the test helper `open_with_widget_icons`
    (`open_with` sets none, decision 27), so `Showcase::new` keeps its signature.
15. **E. The judgements are kept beside the loaded icons**, two vectors indexed as
    `loaded_icons` and `gpui_icons` are (as `loaded_icon_sources` and `gpui_icon_sources`
    are, `SC/app.rs:566-569`), so the `IconEntry` tuple and the tests that destructure it
    do not change.
16. **E. The widgets' own icons are classified once: `not_themeable`, everywhere.** The
    Widget Info's `own_icons` keeps its kind, `not_themeable`, at its 38 call sites
    (counted at `a3fc81f6`, `grep -rn 'own_icons(' SC | grep -v tests.rs`, 39 lines, of which one
    is the definition, `SC/info/mod.rs:444`; 33 of the notes are labelled "own icons",
    one through a variable, `SC/info/feedback.rs:970`, `:976`, and the others "check mark",
    "mixed mark", "mark" and "why they stay", the last relabelled by E.8),
    and the icon-theme Select's notes on the same icons keep the same kind (decision 19),
    with one text stating when the icons are the chosen theme's and when they are
    upstream's. The reason for `not_themeable` over `instance`: no theme value reaches
    them in every case (under the pixel sets, a set the floor declines, the built-in row
    and for an icon in several colours they stay upstream's, and their colour is never the
    theme's own icon's), and the kind is what `a_widgets_own_icons_are_named_gpui_components`
    reads. A text that changed with the handle's state would need the state in every
    builder that makes one of those 38 calls.
17. **F. W8's backlog entry is a dated line of the existing entry** *Checkbox: the check
    mark's glyph and size* (`docs/todo.md:186-212`), which is about the mark the model
    does not state; a second entry would split one item.
18. **F. This specification and its plan are named for the release they are built in; the
    rationale keeps its name.** With the fold (head) they are
    `todo_v0.7.2_gpui-widget-icons-{spec,plan}.md`, as the two earlier folds into 0.7.2
    renamed theirs, and D0 (G.1) re-points the live documents that name them; mentions
    inside `docs/archive/` are left (rule 22 (a)). The rationale stays
    `todo_v0.7.1_gpui-widget-icons-rationale.md`: it is named, as its two siblings of
    2026-11-14 are, for the cycle it was written in, no fold renamed a rationale, and the
    archived 0.9.1 documents and the open-items rationale cite it by that name.
19. **E. The icon-theme Select's two notes on the widgets' own icons keep `not_themeable`**
    (`upstream's icons`, `why they stay`, E.8), relabelled for what they now say: the same
    fact as decision 16, the same kind.
20. **B. The one-colour test is a public function, `icons::mask_drawable`** (settled by
    the coordinator on 2026-11-14; the rationale's W6, last paragraph, and §3 item 1 now
    say so). The showcase applies it to the icons it loads by a role (W3 keeps them by
    role, `load_all_icons`, `SC/support.rs:467-502`), which do not come through the
    lookup; an application that draws a theme's icon as a mask needs the same answer; a
    copy in the showcase would be a second implementation. B.3 specifies it.
21. **C, D. The load's cost has a rule** (settled the same day; the rationale's W3
    "The measurement", its seventh stop condition, and §4). The listing times the
    handle's loads, with the one-colour judgements this work adds, beside the showcase's
    reload of today, per switch to a judged preset in a fixed run (C.4, item 7): a theme
    switch can make the handle load two icon themes, one per mode, so a figure per theme
    alone would hide what a switch costs (corrections below); where
    the handle's takes longer, the plan stops (C.5, condition 7) and the maintainer
    decides whether D.13 applies. D.13 says what the background load changes, so the
    stop has a specified outcome either way.
22. **C. No drawing of an installed icon theme is committed** (settled the same day; the
    rationale's W3 "The measurement"). The committed listing is a table: names, the name
    found, its size, the judgement and a hash of the bytes. The sheets the session reads
    are written under `target/`, which `.gitignore:2` (`/target/`) keeps out of git.
23. **G. The documents are committed before the code.** The three are tracked since
    `0a9141b0`, as written on 2026-11-14. The work's first commit, D0 (G.1), commits them as
    approved: the rationale amended, this specification amended and renamed, the plan
    written anew under its new name (decision 18). A text that cites one of them at a commit
    cites D0 or a later one. A text the showcase shows, a source
    comment or a test attribute names a backlog entry by its title, never a rationale's
    or a specification's path (E.7's doc, C.2's `#[ignore]` reason).
24. **D. A `None` icon theme is resolved once per choice**, by `system_icon_theme()` when a
    method makes a choice (D.8: `set_theme_icons` while following, `fix`, `follow_theme`
    from `Off` or `Fixed`, which builds the key from what was told; `set_theme_icons` while
    `Off` or `Fixed` makes no choice and detects nothing), before the key is compared. The
    told `ThemeIcons` keeps `None` ("the system's"); the key is a resolved type, `IconsKey`
    (D.8), whose freedesktop modes hold a detected name or `ModeKey::DetectionFailed`, and
    detection runs only where the set is Freedesktop (every other set's key holds
    `ModeKey::NoIconTheme`). The key then names what is loaded, and a second choice while
    the system's theme is unchanged loads nothing. A later change of the system's theme
    reaches the handle with the next choice, which the watcher's `apply_system_theme` and
    the showcase's reload make.
25. **D. The cache keeps every table of the current set** (D.8; the rationale's W5, "If a
    theme's files change"): a switch loads only icon themes not yet loaded since the set
    last changed; a change of set drops them all, as W5 gives a new set new paths. The
    alternative, one table per mode and no more, reloads `breeze` after every light/dark
    round trip under kde-breeze, and a theme switch whose other mode kept an earlier
    theme's icon theme would load a table no one sees. The cost W5 accepts: a theme's files
    changed while the application runs are loaded again only after another set was chosen
    and the theme again, and even then the new drawing reaches only sizes a window has not
    drawn yet and windows opened afterwards; a drawing a window already holds stays for the
    run, since GPUI's atlas keeps one tile per path and size (`GP/window.rs:5247-5258`,
    `GP/platform.rs:1859-1861`) and the theme's paths are the same as before.
26. **D. The handle serves nothing until it is installed and its `assets()` was called**:
    without its asset source every served path fails to load and is logged at each draw
    (D.6), so the floor's glyphs would vanish too; `NotMine` keeps gpui-component's glyphs
    and the report names the cause (`NotInstalled`, `NoAssetSource`).
27. **E. The showcase's test harness installs no handle.** The showcase's tests run
    outside the runner environment at every per-commit gate (`scripts/check_release.sh:667`,
    `cargo test -p native-theme-gpui --features native,watch`, which runs the showcase's
    test module, `CN/Cargo.toml:118-123`, `test = true`), so a handle following the theme in
    every window-opening test (about 260 sites open a window in `SC/tests.rs` at
    `a3fc81f6`; 248 when this was written) would
    load the machine's installed icon themes in each and make the tests depend on them.
    `open_with` installs none, and every reader of the showcase's handle handles its
    absence (E.2); a variant, `open_with_widget_icons`, installs one that follows the theme,
    for the tests of the handle in the showcase (E.2, E.11). The other way, a handle
    installed and switched off, needed a flag on the global so the reloads left it off.

## Corrections to the rationale found while specifying

None changes a decision.

- **The exceptions file's `SidebarToggleButton` reason** names `PanelLeftClose` and
  `PanelLeftOpen`; at 0.9.1 the toggle also draws `PanelRightOpen` and
  `PanelRightClose` for a right-hand sidebar (`GC/sidebar/mod.rs:360-368`). E.12's new
  reason names all four.
- **§1.1, `Application::with_assets`**: `GP/app.rs:266-273` (the rationale says
  `:265-273`); `GF/app.rs:267-274`.
- **§1.2's list with lines** is Appendix 2 (read today with a script over `GC/`, test
  modules left out): 52 names in 43 files (at 0.9.1; 44 files at 0.10.0, Appendix 2's
  note).
- **W3's condition 7 is measured per switch, not per theme.** The rationale compares "the
  handle's load of a judged theme" with the showcase's reload for that theme. A theme
  switch can make the handle load two icon themes (kde-breeze's `breeze` and `breeze-dark`)
  where the showcase reloads one, and a mode switch can make it load none; so C.4 item 7
  times what each switch of a fixed run of presets makes each side do, and condition 7
  compares those figures. The handle's side counts the one-colour judgements this work
  adds to a switch; the showcase's side is its reload of today.
- **At gpui-component 0.10.0** (read 2026-11-19; none changes a decision):
  - A loading `Button` with no icon shows a `Spinner` where an icon would sit, so W3's
    "without which gpui-component draws no spinner" no longer holds. The rationale's
    paragraph is amended, and E.4 is written for the showcase's two loading buttons.
  - A check box's indeterminate state draws `Minus` through the same raw `svg` path as its
    check mark, so the resolver answers it. The connector's `widgets::Checkbox` draws its
    dash the same two ways as its check (stroked, or through the resolver), so W8 holds for
    both marks. `Minus` was an essential already.
  - Under a right-to-left layout the breadcrumb, the submenu arrow and the sidebar menu's
    caret draw `ChevronLeft`, and the calendar and the pagination swap their two chevrons.
    These are names among the 106, served like any other and drawn empty where the set
    lacks one (W4). The showcase lays out left to right only.
  - `radio.rs` names `Check` itself; the radio drew the same mark before.

## Open points for the maintainer

1. **At the measurement's stop, whether a longer load is noticeable** (condition 7,
   C.5): the maintainer's judgement on the two figures, which decides whether D.13 is built.
2. (Closed by the fold: the asset run is 0.7.2's own, once, in its close-out, G.4.)
3. **The icon theme packages' versions** for the measurement's README (C.4): of the judged
   themes only, read by the maintainer on the day of the run, since the session reads nothing
   in the icon base directories. He is asked for them with the approval, when he is also told
   that the run reads the installed icon themes through the library.

---

## Part A — The core's new method (W4)

### A.1 `NT/src/freedesktop.rs`

**Read today.** `find_icon_with` (`:290-308`) takes the chain once and tries
`<name>-symbolic` through the whole chain, then `<name>`, marking the second pass
symbolic where the name itself ends in `-symbolic` (`:300-307`). `first_of_names`
(`:336-348`) tries, theme by theme, each name as `<name>-symbolic`, then as itself,
with the same mark. `lookup_in_chain(name, size, chain)` (`:260-269`) keeps the first
theme of `chain` whose own directory holds the file; called with one theme it is that
theme's lookup, so its answer over a chain is the first of its answers over the chain's
themes one by one (`first_in_chain`, `:138-146`). `load_icon_file` (`:412-434`) loads a
path. The module exists on Linux with `system-icons` (`NT/src/lib.rs:324-325`).

**Changes.**

1. **`first_of_name`** (new, private), after `find_icon_with`:
   `fn first_of_name(name: &str, chain: &[String], lookup: impl FnMut(&str, &[String]) -> Option<PathBuf>) -> Option<(PathBuf, bool)>`,
   whose body is `find_icon_with`'s lines `:297-307` with `lookup_in_chain(…, size, chain)`
   replaced by `lookup(…, chain)`; its doc: "The two passes of [`find_icon`]: `<name>-symbolic`
   through the whole chain, then `<name>`; with whether the icon is symbolic."
   `find_icon_with` keeps its signature and becomes: take the chain
   (`cached_theme_chain(theme)?`), then `first_of_name(name, &chain, |n, c|
   lookup_in_chain(n, size, c))`. Its behaviour does not change; the ignored test
   `indexed_lookup_matches_the_unfiltered_lookup_on_installed_themes` (`:1494-1561`)
   still compares it with the unfiltered lookup.
2. **`first_accepted_of_name`** (new, private):
   `fn first_accepted_of_name(name: &str, chain: &[String], lookup: impl FnMut(&str, &String) -> Option<PathBuf>, load: impl FnMut(&Path, bool) -> Option<IconData>, accept: &mut dyn FnMut(&IconData) -> bool) -> Option<IconData>`.
   It calls `first_of_name` with a closure that, given a name and the chain, goes through
   the chain's themes in order and, for each, looks the name up in that theme alone,
   loads the file found with `load(path, name.ends_with("-symbolic"))` (the mark
   `first_of_name` gives the same pass), and keeps the path only where the icon loaded
   and `accept` passes it, storing that icon in a variable the closure owns; the stored
   icon is returned. Doc: "The first icon of [`first_of_name`]'s order that loads and
   that `accept` passes: an icon found and refused, or a file that does not load, is
   passed over for the next one that order comes to, through the same theme's chain."
3. **`first_accepted_of_names`** (new, private): the same over `first_of_names`, with
   `lookup: impl FnMut(&str, &String) -> Option<PathBuf>` given straight to it inside the
   accepting closure. Doc as 2, of [`first_of_names`]'s order.
4. **`load_freedesktop_icon_by_name_where`** and **`load_freedesktop_icon_by_names_where`**
   (new, `pub(crate)`, `#[must_use]`), after `load_freedesktop_icon_by_name` (`:386-394`):
   the arguments of `load_freedesktop_icon_by_name` and `…_by_names` plus
   `accept: &mut dyn FnMut(&IconData) -> bool`; each takes `cached_theme_chain(theme)?`
   and `lookup_size(size)`, and calls 2 or 3 with `|n, t| lookup_in_chain(n, size,
   std::slice::from_ref(t))` and `|path, symbolic| load_icon_file(path, symbolic,
   fg_color)`. Doc: "[`load_freedesktop_icon_by_name`] for the first icon that `accept`
   passes (see [`first_accepted_of_name`])."

No arithmetic is added. Every new function is reached from A.2's method, so none is dead
in a build with the module.

### A.2 `NT/src/icons.rs`, `FreedesktopLoader::load_where`

After `load_with` (`:322-380`), before `load_indicator` (`:382`):

`pub fn load_where(self, accept: impl FnMut(&IconData) -> bool) -> Option<IconData>`,
`#[must_use]`, `#[allow(unused_variables)]` as `load_with` has it (`:324`), with the same
two `cfg` blocks as `load_with` (`:326`, `:376`). The parameter is bound without `mut`
and rebound mutable inside the Linux block (`let mut accept = accept;`), so the build
without the block warns of no `unused_mut`, which the `allow` does not cover (an error
under `-D warnings`, `scripts/check_release.sh:529-538`, and the cross-target legs
`:494-496`, which build the core for macOS and Windows). On Linux with `system-icons` it picks
the theme as `load_with` does (the set one, else `system_icon_theme()`; `:328-335`) and
matches `self.wanted`:

| Form | Order it goes on in |
|---|---|
| `Wanted::AnyOf(names)` | `load_freedesktop_icon_by_names_where`: theme by theme of the chain, in each the names in order, each as `<name>-symbolic` then itself |
| `Wanted::Id(IconId::Role(role))` | the role's freedesktop name (`icon_name(role, IconSet::Freedesktop)?`), then as `Name` |
| `Wanted::Id(IconId::Name(name))` | `load_freedesktop_icon_by_name_where`: `<name>-symbolic` in each theme of the chain, in order, then `<name>` in each |
| `Wanted::Id(IconId::Custom(provider))` | where a theme is known and the provider names a freedesktop icon, that name as `Name`; then the provider's own SVG (`provider.icon_svg(IconSet::Freedesktop)`), returned only where `accept` passes it |

Off Linux or without `system-icons`: `None`.

**Its documentation:**

```rust
/// [`Self::load`], for the first icon that `accept` passes: an icon the loader
/// finds and `accept` refuses is passed over for the next one this loader would
/// come to, in its own order and in the same theme and the themes it inherits
/// from, never another. By names ([`Self::any`]): theme by theme, each name as
/// `<name>-symbolic`, then as itself. By a name, or a role's name: the symbolic
/// form through the whole chain, then the plain name. A custom
/// [`IconProvider`]: its freedesktop name so, then its own SVG.
///
/// For a caller that can use only some icons (a toolkit that draws an icon
/// as a mask of one colour cannot use one in several colours), so that one
/// unusable drawing does not hide a usable one under the next name.
///
/// `load_where(|_| true)` is [`Self::load`] but in one case: where the first
/// file found cannot be read or decoded, `load` gives `None` and `load_where`
/// goes on to the next.
///
/// Requires the `system-icons` feature, and Linux; `None` otherwise.
```

The struct's doc example (`:182-198`) is unchanged; the method's doc has no example
(the tests below show it).

### A.3 Tests

In `NT/src/freedesktop.rs`'s `mod tests` (`:745-748`, `#[cfg(feature =
"system-icons")]`), with the existing `fake_lookup` (`:1305-1316`, a per-theme lookup,
`(&str, &String)`, which `first_accepted_of_name(s)` take), a new chain-level one for
`first_of_name`, `fake_chain_lookup(files) -> impl FnMut(&str, &[String]) -> Option<PathBuf>`
(the first theme of the given chain, in order, for which `files` holds `(theme, name)`,
answered `theme/name`, as `fake_lookup` answers), and a fake `load` that returns
`IconData::Svg` of the path's text:

- `first_accepted_of_names_goes_on_in_its_order`: chain `["Adwaita",
  "AdwaitaLegacy"]`; files `("Adwaita", "a")`, `("Adwaita", "b-symbolic")`,
  `("AdwaitaLegacy", "a-symbolic")`; names `["a", "b"]`. Accepting everything gives
  `Adwaita/a` (and `first_of_names` the same path); refusing `Adwaita/a` gives
  `Adwaita/b-symbolic` (the theme's own later name before the parent's earlier one);
  refusing both gives `AdwaitaLegacy/a-symbolic`; refusing all gives `None`; a fake
  `load` that fails on `Adwaita/a` gives `Adwaita/b-symbolic` with everything accepted.
- `first_accepted_of_name_goes_on_in_its_order`: chain `["child", "parent"]`; files
  `("parent", "x-symbolic")`, `("child", "x")`, `("parent", "x")`. Accepting everything
  gives `parent/x-symbolic`, which `first_of_name` with `fake_chain_lookup` over the same
  files gives too (the symbolic pass through the chain first); refusing it gives `child/x`; refusing that too
  gives `parent/x`; refusing all gives `None`.

In `NT/src/icons.rs`, a new test module `load_where_tests`
(`#[cfg(all(test, target_os = "linux", feature = "system-icons"))]`,
`#[allow(clippy::unwrap_used, clippy::expect_used)]`), each test a child process of
`native_theme_test_fixtures::run` (the pattern of `NT/src/freedesktop.rs:825-852`), the
themes written by hand where they inherit (as `CN/src/icons.rs:3085-3134` writes its
`index.theme`s), each SVG a fixture whose `<title>` names its theme and file:

- `load_where_goes_on_by_names`: a scalable theme `passing` with `refused.svg` and
  `kept.svg`; `FreedesktopLoader::any(&["refused", "kept"]).theme("passing")` with an
  `accept` refusing the SVG titled `passing/refused` loads `passing/kept`; `load()`
  loads `passing/refused`.
- `load_where_goes_on_by_a_name_through_the_chain`: `child` (`Inherits=parent`) with
  `x.svg`, `parent` with `x-symbolic.svg`; `new("x").theme("child").load()` is
  `parent/x-symbolic` (the present order); `load_where` refusing it is `child/x`.
- `load_where_by_a_role`: a theme with `dialog-error.svg`;
  `new(IconRole::DialogError)` with `|_| true` equals `load()`, with `|_| false` is `None`.
- `load_where_by_a_provider`: a provider naming `edit-copy` with an SVG of its own, a
  theme with `edit-copy.svg`: refusing the theme's icon gives the provider's SVG;
  refusing both gives `None`.

And one test in the existing `mod icon_loader_tests` (`NT/src/icons.rs:2368-2370`,
`#[cfg(test)]` with no feature gate), itself `#[cfg(not(all(target_os = "linux", feature =
"system-icons")))]`: `load_where_is_none_without_the_freedesktop_loader`,
`FreedesktopLoader::new("edit-copy").load_where(|_| true)` is `None`.

### A.4 Nothing changes for present callers

`load`, `load_with`, `any`, `new`, `load_freedesktop_icon_by_name(s)`, `find_icon`,
`find_any_icon` and `first_of_names` keep their signatures and behaviour;
`find_icon_with` keeps its signature and calls `first_of_name`, whose body is its own.
Every existing test of both files passes unchanged; the connector's
`the_theme_s_own_name_wins_in_each_convention_s_theme` (`CN/src/icons.rs:3028-3136`)
and the core's `first_of_names_*` tests (`:1318-1364`) pin the order the new method
reuses.

---

## Part B — The lookup and the one-colour test (W3, W6, W9)

### B.1 The two table entries

`CN/src/icons.rs:640-641`, `freedesktop_name_candidates`:

| Line | Present | New |
|---|---|---|
| `:640` | `IconName::Loader => &["process-working", "content-loading"],   // exact` | `IconName::Loader => &["process-working-symbolic", "content-loading"], // exact: the symbolic name outright, so Breeze's strip of frames under the plain name is never found` |
| `:641` | `IconName::LoaderCircle => &["process-working", "content-loading"], // exact` | `IconName::LoaderCircle => &["process-working-symbolic", "content-loading"], // exact: as Loader` |

`Check` stays `&["dialog-ok", "object-select"]` (W3; its order is decided on the
measurement, condition 3). The entry form "Breeze's, then GTK's" stands, so the
function's doc (`:526-527`) does not change.

**Every reader, and what it then asserts:**

- `freedesktop_name_for_gpui_icon` (`:498-520`, deprecated): on a desktop that is not
  GTK its answer for `Loader` and `LoaderCircle` becomes `process-working-symbolic`; on
  a GTK desktop it stays `content-loading`. Its doc does not change; the CHANGELOG says
  it (F.3).
- `all_kde_names_resolve_in_breeze` (`:2738-2775`): loads
  `FreedesktopLoader::new("process-working-symbolic").theme(&theme).size(24)`, which
  tries `process-working-symbolic-symbolic`, then `process-working-symbolic`
  (`NT/src/freedesktop.rs:300-307`): it now checks, where Breeze is the system theme,
  that Breeze has `process-working-symbolic` (W3; §4's "recorded" item). Unchanged
  text.
- `gnome_names_resolve_in_adwaita` (`:2777-2803`): the GTK answer is unchanged.
- The fixture rows `CONVENTIONS` (`:2941-2942`): `(IconName::Loader, "process-working",
  "content-loading")` and the `LoaderCircle` row become `(…, "process-working-symbolic",
  "content-loading")`.
- `the_theme_s_own_name_wins_in_each_convention_s_theme` (`:3028-3136`), checked against
  the new rows: `breeze-only` holds `process-working-symbolic.svg`, which
  `any(["process-working-symbolic", "content-loading"])` finds as the second form of the
  first name (`…-symbolic-symbolic` is absent), titled `breeze-only/process-working-symbolic`;
  `gtk-only` and `gtk-over-legacy` find their own `content-loading.svg` before any
  parent (the legacy theme holds `process-working-symbolic.png`, reached only after);
  `both-names` gives the first name. Its assertions do not change.
- The showcase's `load_gpui_icons` (`SC/support.rs:688-693`) reads the entry; Part E
  replaces it by the connector's lookup.

### B.2 The lookup: `IconUse`, `GpuiIcon`, `load_gpui_icon`

In `CN/src/icons.rs`, after `to_rgba` (`:1181-1190`) and before `#[cfg(test)] mod tests`
(`:1192`). Not behind `svg-rasterize`
(decision 3).

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum IconUse { Mask, Any }

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct GpuiIcon { pub data: IconData, pub mask: Option<bool> }

#[must_use]
pub fn load_gpui_icon(icon: IconName, set: IconSet, icon_theme: Option<&str>,
    size: Option<u32>, color: Option<Rgba>, usage: IconUse) -> Option<GpuiIcon>
```

(`Rgba` is `native_theme::Rgba`, which the crate re-exports.)

**What it serves, per set** (W3; Appendix 1 lists every name):

| Set | Icon |
|---|---|
| `Lucide` | `LucideLoader::new(lucide_name_for_gpui_icon(icon)?).load()`; `StarFill` has no name: `None` |
| `Material` | `MaterialLoader::new(material_name_for_gpui_icon(icon)?).load()`; `StarOff`: `None` |
| `Freedesktop`, on Linux | `FreedesktopLoader::any(freedesktop_name_candidates(icon))` (an empty slice: `None`), with `.color_opt(color)`, `.size(s)` where `size` is `Some(s)` and `.theme(t)` where `icon_theme` is `Some(t)` (else the system's icon theme, as the loader takes it); `IconUse::Any` → `.load()`; `IconUse::Mask` → `.load_where(…)` with an `accept` that judges each icon with `mask_drawable`, keeps the judgement of the icon it passes, and passes `Some(true)` |
| `Freedesktop` off Linux, `SfSymbols`, `SegoeFluent`, any later set | `None` (W7; the candidates exist on Linux only, `:555`) |

The freedesktop arm is `#[cfg(target_os = "linux")]`; off Linux, a
`#[cfg(not(target_os = "linux"))]` statement `let _ = (icon_theme, size, color);` consumes
the three parameters only that arm reads, as `is_freedesktop_theme_available` does
(`NT/src/icons.rs:1295-1298`), so CI's macOS and Windows builds of the connector warn of no unused variable (no local leg of `scripts/check_release.sh` builds the connector for another target).
Six parameters, under clippy's `too_many_arguments` limit of seven. A set's own feature off (`lucide-icons`, `material-icons`, `system-icons`): its loader
returns `None`, so the lookup does. `size` and `color` reach the freedesktop loader only:
the bundled sets are scalable and coloured where they are drawn. Under `IconUse::Mask`
a Lucide or Material icon that `mask_drawable` does not judge `Some(true)` gives `None`
(there is nothing to pass over to). The result's `mask` is the judgement: under
`IconUse::Mask` the one `accept` made of the icon it passed (each icon is rasterised once,
which C.4's timing then measures as the handle does it), under `IconUse::Any`
`mask_drawable(&data)` of the icon found.

**Documentation** (one paragraph each, in the crate's style):

- `IconUse`: "What a caller of [`load_gpui_icon`] can draw. `Mask`: only an icon a mask
  of one colour carries as it is, which is how gpui draws an SVG by its path (gpui-pre
  `src/svg_renderer.rs`, `render_alpha_mask`), so the lookup passes over an icon in
  several colours and a raster for the next one the set's order comes to. `Any`: any
  icon the set has, for a caller that draws an image in its own colours too."
- `GpuiIcon`: "An icon [`load_gpui_icon`] found, with whether a mask carries it
  ([`mask_drawable`]): `Some(true)` an SVG that paints one colour, `Some(false)` an icon
  a mask cannot carry (pixels, several colours, nothing painted, an embedded image),
  `None` an SVG not judged, without the `svg-rasterize` feature."
- `load_gpui_icon`: what the table above says, and: "This is the lookup the
  [`WidgetIcons`] handle serves gpui-component's widgets from, with `Some(16)`, no colour
  and `IconUse::Mask`; an application's own icons for the same names take their size and
  colour. Under SF Symbols and Segoe Fluent the connector has no table from a name to an
  icon, so there is none. Without `svg-rasterize` no SVG is judged, and `IconUse::Mask`
  finds none." (The `WidgetIcons` link resolves only with the feature: written as
  `` `WidgetIcons` `` in code font without a link, so the docs build without it passes
  rustdoc's `-D warnings`.)

### B.3 The one-colour test: `mask_drawable`

In `CN/src/icons.rs`, beside B.2, a public function (decision 20):

`#[must_use] pub fn mask_drawable(data: &IconData) -> Option<bool>` — it takes an icon
the caller loaded by any route (by a role, by a name, a provider's) and answers whether
gpui's mask carries it. It exists with every feature set: not behind `svg-rasterize`
(decision 3), not behind a set's feature, on every platform. `IconData` is the core's,
which the crate re-exports (`CN/src/lib.rs:266`). What it returns:

- `IconData::Rgba(_)`, and any later variant: `Some(false)`.
- `IconData::Svg(bytes)`, without `svg-rasterize`: `None`.
- With `svg-rasterize`: `Some(false)` where `embeds_image(bytes)`; else
  `native_theme::rasterize::rasterize_svg(bytes, ONE_COLOUR_TEST_SIZE)`: an error or a
  result that is not `IconData::Rgba` is `Some(false)` (it paints nothing a mask can
  show); else `one_colour_deviation(&rgba)`: `None` (no pixel painted) is `Some(false)`,
  `Some(d)` is `Some(d <= ONE_COLOUR_TOLERANCE)`.

Private items, each `#[cfg(feature = "svg-rasterize")]` (only `mask_drawable`'s branch
with the feature and the tests use them, so without it none is dead code):

- `const ONE_COLOUR_TEST_SIZE: u32 = 16;` (decision 4).
- `fn embeds_image(svg: &[u8]) -> bool`: the bytes hold an element named `image` or
  `feImage` (a filter primitive that draws an image by a `data:` or an external reference,
  which native-theme's rasteriser, built without `raster-images`, does not draw either): a
  `<` followed by an optional XML name and `:` (a prefix), then `image` or `feImage`, then an
  ASCII whitespace, `/` or `>`. The bytes are searched as bytes, without indexing or slicing
  (`windows`, `position`, `strip_prefix` on slices).
- `fn one_colour_deviation(rgba: &RgbaData) -> Option<u32>`: the painted pixels are those
  with alpha above 0 (`as_chunks::<4>()`, destructured, as `CN/src/icons.rs:1170-1174`
  does); `None` where there is none. The reference colour is the red, green and blue of
  the first painted pixel, in row order, whose alpha `A` is the greatest (a `fold` that
  keeps the earlier pixel on a tie). Its rounding allowance `ρ` is `0` where `A` is 255
  (`unpremultiply_alpha` divides no opaque pixel, `NT/src/color.rs:340`) and otherwise
  `⌈255 / A⌉`, the step by which the reference's channel moves for one unit of its
  premultiplied value: `254u32.saturating_add(u32::from(A)).checked_div(u32::from(A))?`
  (`A` is above 0, a painted pixel's, so the `?` never returns; the same form as
  `NT/src/freedesktop.rs:507-508`, which the strict-panic run lints,
  `scripts/check_release.sh:573`). A pixel's deviation is the largest, over its three
  channels, of `u32::from(a).saturating_mul(u32::from(c.abs_diff(r)).saturating_sub(ρ))`;
  the result is the largest deviation. No other arithmetic (`saturating_mul`,
  `saturating_add`, `saturating_sub`, `checked_div` and `abs_diff` pass
  `clippy::arithmetic_side_effects` and `clippy::integer_division`,
  `scripts/check_strict_panic.sh:88-89`).
  **Why the allowance.** Without it, an icon with no opaque part compares every pixel with
  a reference that the division rounded too, and its worst deviation reaches that of an
  icon with an opaque part (computed below); with it, half of it. An alpha-weighted mean
  as the reference was weighed and not taken: in an icon at one opacity the interior
  pixels share the reference's alpha and so its premultiplied value and its rounding, and
  they carry most of the weight, so the mean moves the reference by less than one step.
  The allowance changes nothing for the two-colour fixture (opaque, `ρ = 0`) nor for an
  icon with an opaque pixel; the tolerance is measured with it, as every judgement is.
- `const ONE_COLOUR_TOLERANCE: u32 = 3812` (amended 2026-11-19, after the measurement;
  first written as the largest deviation of the bundled sets, 492, which judged Breeze's
  one-colour `weather-clear-night-symbolic`, 570, as several colours: a number set on the
  bundled sets alone, not on the themes it judges). The measurement's 1,802 rows
  (`docs/evidence/widget-icons/2026-11-19/listing.md`) fall in two groups with nothing
  between: one-colour icons up to 570 (the bundled sets up to 492), about two levels of a
  channel at full alpha, the rounding of antialiased edges; icons in several colours from
  25500 (Breeze's `cpu`, 100 levels). The value is their geometric mean, rounded down
  (`⌊√(570 × 25500)⌋`), the boundary with the same margin to both sides, a factor of 6.7.
  Its doc says this, with both figures, their icons, the date, resvg 0.48.1 and the
  evidence directory.

**Its documentation** (`mask_drawable`): "Whether gpui's mask carries `data` as it is: an
SVG counts as one colour where every pixel it paints, rasterised at 16 px by native-theme,
has the same red, green and blue, whatever its alpha, within the rounding of antialiased
edges; a symbolic icon with half-transparent parts is one colour, and so is one of a
colour that is not grey, which gpui paints in the widget's colour, as it paints every
icon. `Some(false)` for an SVG in several colours, one that paints nothing, one that
embeds an image (native-theme's rasteriser does not draw it and gpui's does), and for
pixels; `None` for an SVG without the `svg-rasterize` feature, which has no rasteriser
to judge it. The test is by what the icon paints, not by its text: an icon may set its
colours through a stylesheet, as Breeze's do. For an application that draws an icon
it loaded itself (by a role, through `SystemTheme::icon` or a loader) as an `Icon`, a
mask in the widget's text colour: where this is `Some(false)`, draw it with
`gpui::img` instead, in its own colours (see [`to_image_source`]), or not at all where
the widget takes an `Icon` alone. [`load_gpui_icon`] gives the same answer in
[`GpuiIcon::mask`]."

**Tests**, in `CN/src/icons.rs`'s `mod tests`, `#[cfg(feature = "svg-rasterize")]`:

- `the_one_colour_tolerance_separates_the_bundled_sets_from_two_colours`, with
  `#[cfg(all(feature = "lucide-icons", feature = "material-icons"))]` too: for every name
  of `ALL_ICON_NAMES` with a Lucide or Material name, the bundled SVG (`LucideLoader`,
  `MaterialLoader`) as loaded, and coloured with `native_theme::icons::colorize_monochrome_svg`
  in each preset's resolved `defaults.text_color`, light and dark, for every key of
  `native_theme::theme::Theme::list_presets()` (17 today: 34 colours); `M` is the
  largest `one_colour_deviation` of them all. A fixture of two colours: an SVG of two
  equal rectangles side by side filled with kde-breeze's resolved light
  `defaults.text_color` and `defaults.accent_color` (read from the preset in the test, not
  written as literals), whose deviation is `F`. Asserts `M <= ONE_COLOUR_TOLERANCE` (the
  message prints `M` and says that a resvg or tiny-skia upgrade changes the rasteriser's
  rounding, and that ONE_COLOUR_TOLERANCE is re-derived as its doc says, from a new run of
  the measurement, whose tool the evidence README's commit holds), and `ONE_COLOUR_TOLERANCE < F` (the message prints both). Every
  bundled icon that `mask_drawable` does not judge `Some(true)` is named in a further
  assertion's message: the bundled sets are one colour throughout (W6). This test can turn
  the dependency canary red on a rasteriser upgrade with no defect in the connector; the
  re-derivation is the response, and the commit that makes it says the new figures.
- `a_half_transparent_part_keeps_an_icon_one_colour`: one colour at `fill-opacity` 0.5 and
  at 1 → `Some(true)`.
- `an_icon_with_no_opaque_part_is_one_colour`: an SVG whose every shape is at `opacity` 0.5,
  in each preset text colour of the tolerance test, with antialiased edges (a circle and a
  diagonal stroke): `Some(true)` for each, and its deviation is printed. Where no pixel is
  opaque, the reference pixel itself comes back through the rasteriser's division, which
  the allowance `ρ` covers. Computed on 2026-11-14 over a model, every channel value 0 to
  255 and every alpha, of the premultiplied 8-bit value as the nearest to the exact one
  (either way on a half) and `unpremultiply_alpha`'s division (`NT/src/color.rs:329-349`):
  an icon with an opaque reference deviates by at most 254; one whose greatest alpha is
  128 (or 127) by at most 254 (252) without `ρ` and 126 (124) with it. So with `ρ` the
  half-transparent icon's worst case is half the opaque icons' from which the tolerance is
  measured (a bound that adds the two pixels' division remainders, about 510, overstates
  it: they enter with opposite signs and so stay under one step together). The model is not
  tiny-skia's code (not read); a failure here means the tolerance, derived from icons with
  opaque parts, does not cover a half-transparent symbolic icon, and the implementer stops
  and reports.
- `an_svg_that_paints_nothing_is_not_drawable`: an `<svg>` with a `viewBox` and no shape →
  `Some(false)`.
- `an_svg_that_embeds_an_image_is_not_drawable`: an `<image>` with a `data:` PNG URL, one
  with an `svg:` prefix, and a filter whose `<feImage>` takes a `data:` reference →
  `Some(false)`; `embeds_image` is false for an SVG whose only
  `image` is inside an attribute value (`id="imagery"`).
- `pixels_are_not_drawable`: an `IconData::Rgba` → `Some(false)`.
- `an_unparseable_svg_is_not_drawable`: `b"not svg"` → `Some(false)`.
- `the_lookups_judgement_is_mask_drawables` (with `lucide-icons` too): for every name with
  a Lucide icon, `load_gpui_icon(…, IconUse::Any)`'s `mask` equals `mask_drawable` of its
  `data`.

`#[cfg(not(feature = "svg-rasterize"))]`: `without_the_rasteriser_no_svg_is_judged`:
`mask_drawable` of any SVG is `None`, and `load_gpui_icon(…, IconUse::Mask)` under Lucide
is `None` (with `lucide-icons`).

**Tests of the lookup**, `mod tests`:

- `the_lookup_takes_the_tables_name_in_lucide_and_material` (with both features): for every
  name, `load_gpui_icon(icon, Lucide, None, None, None, IconUse::Any).map(|i| i.data)` equals
  `lucide_name_for_gpui_icon(icon).and_then(|n| LucideLoader::new(n).load())`, and the
  same for Material; `StarFill` under Lucide and `StarOff` under Material are `None`.
- `the_lookup_has_no_icon_under_sf_symbols_and_segoe_fluent`: `None` for every name.

In `mod freedesktop_mapping_tests` (itself `#[cfg(test)] #[cfg(target_os = "linux")]`,
`CN/src/icons.rs:2614-2617`, where `system-icons` gates only some imports inside it), each
new fixture test carrying `#[cfg(feature = "system-icons")]` itself, child-process fixtures
as `:3085-3134`:

- `the_mask_lookup_passes_over_several_colours_and_pixels`: a theme `masks` with
  `tab-close.svg` in two colours (two rectangles of the two kde-breeze colours above, read
  from the preset) and `window-close.svg` in one: `IconName::Close` under `IconUse::Any` is
  `tab-close`'s, under `IconUse::Mask` `window-close`'s with `mask == Some(true)`
  (`#[cfg(feature = "svg-rasterize")]`); a theme `rasters` with `tab-close.png` and
  `window-close.svg`: `Mask` gives the SVG.
- `the_lookup_looks_up_at_the_size_given`: a theme of two fixed directories, 16 and 24,
  through `native_theme_test_fixtures::write_fixed_theme` (`tools/test-fixtures/src/lib.rs:201-209`),
  each holding `go-down.svg` with a title naming its size: `ChevronDown` with `Some(16)` is
  the 16 px file, with `None` the 24 px one (the loader's default size, `NT/src/icons.rs:226`,
  `:267`).

### B.4 `scripts/connector_parity_exceptions.toml`

`scripts/check_connector_parity.py` compares the column-0 `pub fn`s of a module every
connector declares, `icons` among them (its docstring, "What is scanned"). The
exception at `:651-655` (`connector = ["iced", "egui"]`, `kind = "absent"`, "they
translate gpui-component's `IconName` …") gains `"icons::load_gpui_icon"` at the end of
its `name` list; a new exception after it:

```toml
[[exception]]
connector = ["iced", "egui"]
name = "icons::mask_drawable"
kind = "absent"
reason = "gpui draws an SVG by its path as a mask in one colour (gpui-pre src/svg_renderer.rs, render_alpha_mask), so an icon in several colours cannot be drawn that way; iced and egui draw an icon as an image in its own colours"
```

The handle's methods and its types are not free functions; the script does not read them.

---

## Part C — The measurement: the first stop (W3, W9)

### C.1 What is built before it, and the one check made earlier

Units 1 and 2 (G.1): Part A and Part B. Nothing of Part D or E. One check of condition 4 is
made before B.1 is committed, as unit 2's first step (G.1): with B.1's two entries changed in
the working tree, uncommitted, `cargo test -p native-theme-gpui --features native,watch --lib
all_kde_names_resolve_in_breeze`, run outside the runner environment on the maintainer's
machine, loads `process-working-symbolic` from the installed Breeze through the library
(B.1). Red: Breeze has no `process-working-symbolic`, condition 4 is met, and the plan stops
for the maintainer before B.1 lands (B.1 would leave that test red at the end of unit 2,
`scripts/check_release.sh:667` runs it outside the runner environment). Skipped (it prints
that the system theme is not Breeze): the check falls to the table's item 5 in unit 3, and
the gate cannot be red on it either.

### C.2 The tool

`SC/tests.rs`, a new test (decision 7), `#[cfg(all(target_os = "linux", feature =
"svg-rasterize"))]` (it rasterises and judges, and the showcase's test build without the
feature, which clippy's no-feature run compiles, `scripts/check_release.sh:549-550`, has no
rasteriser; it calls `detect_linux_desktop`, which exists on Linux only,
`NT/src/detect.rs:85`, and measures freedesktop themes, which are Linux's):

```rust
#[test]
#[ignore = "the widget icons' measurement: reads the installed icon themes, writes the table into $NT_WIDGET_ICONS_LISTING and the sheets under target/widget-icons-sheets/ (docs/evidence/README.md, widget-icons)"]
fn widget_icons_listing()
```

Where `NT_WIDGET_ICONS_LISTING` is unset it prints that and returns. A relative value is
taken from the repository's root, which the test computes as `env!("CARGO_MANIFEST_DIR")`
joined with `../..` (a test runs in its package's directory, `CN/`); the test refuses a
directory that is not under `<root>/docs/evidence/widget-icons/`. The sheets go to
`<root>/target/widget-icons-sheets/<run>/`, `<run>` the listing directory's last
component, a path the test builds itself: `target/` is ignored by git (`.gitignore:2`,
`/target/`), so no drawing reaches a commit (decision 22). It is run outside the runner
environment, which hides the installed themes (`scripts/run_in_runner_env.sh`), in the
dev profile the gates build (no second build of the gpui stack): condition 7 compares two
figures of one build, so the profile scales both alike:

`NT_WIDGET_ICONS_LISTING=docs/evidence/widget-icons/<date> cargo test -p native-theme-gpui --features native,watch --example showcase-gpui -- --ignored widget_icons_listing --nocapture`

The run reads the maintainer's installed icon themes, the machine's state, through the
library, as the connector's existing installed-theme tests do when the gates run them; the
maintainer is told of the run with the approval (Open point 3). The session itself reads no
file outside the project folder for it.

It reads the themes only through native-theme and the connector: `load_gpui_icon`,
`mask_drawable`, `FreedesktopLoader` (`new`, `any`, `load`, `load_where`),
`list_freedesktop_themes`, `is_freedesktop_theme_available`,
`native_theme::detect::system_icon_theme`, `SystemTheme::from_system`, `Theme::list_presets`
with `resolve`, the showcase's `load_all_icons`, `load_gpui_icons` and `role_for_gpui_icon`,
`native_theme_gpui::icons::to_image_source`, and `native_theme::rasterize::rasterize_svg`
for the sheets, which it encodes with the dev-dependency `image`'s PNG encoder
(`CN/Cargo.toml`, `[dev-dependencies]`, `features = ["png"]`). A sheet's cell is the icon
rasterised at 64 px: an icon the widgets would be served drawn as gpui draws it, its alpha
in black on white; an icon "as loaded" in its own colours on white; pixels as they are,
centred. Cells run in rows of 10 in the order of `SC/support.rs`'s `GPUI_ICONS`, so the
committed table gives each cell's name.

**A hash** identifies an icon's bytes in the table: FNV-1a, 64 bits, with its published
offset basis and prime, over the SVG's bytes as loaded or an RGBA icon's pixel buffer,
written in hex; a few lines of the test, with `wrapping_mul` (no new dependency).

**The name an icon was found under.** native-theme returns an icon's bytes, not the file it
came from, so the test finds the name through the library: for an icon served under an
entry's candidates in theme `T`, the first candidate `n` whose own lookup,
`FreedesktopLoader::any(&[n])` in `T` at the same size and with the same test, gives the
same bytes, is the name. Its form: a candidate that already ends in `-symbolic`
(`emblem-ok-symbolic`; after B.1 `process-working-symbolic`) is found in that form, as the
loader marks it (`NT/src/freedesktop.rs:307`, `:345`); for another, the same lookup of
`[format!("{n}-symbolic")]` giving the same bytes says the symbolic form, else the plain one.
Each re-lookup uses the first lookup's size, colour and test (the `Any` rows' colour too).
Where two candidates' files have the same bytes the earlier one is named, which the table's
header says. Which theme of `T`'s
chain held the file the library does not say: the table gives the theme searched, and the
other installed themes whose row for the same name has the same hash (a parent's icon shows
there, since every installed theme is listed).

### C.3 The themes

- **Judged**: each freedesktop icon theme a bundled preset names, from resolving every
  `Theme::list_presets()` key in both modes and keeping `icon_set == Freedesktop`
  (`breeze`, `breeze-dark`, `Adwaita` today: `kde-breeze.toml:16`, `:601`;
  `adwaita.toml:12`), and the session's system theme (`system_icon_theme()`). One that is
  not installed (`is_freedesktop_theme_available`) is named as not judged.
- **Listed, not judged**: every other theme of `list_freedesktop_themes()`, at 16 px only (a
  theme searched only as a parent, such as `AdwaitaLegacy`, is among them and is not judged
  on its own).
- **The bundled sets**: Lucide and Material, the repository's own files, at their one size;
  their rows are in the table like the others.
- **Pairs** (W5): each freedesktop preset's light and dark icon theme (`breeze` with
  `breeze-dark`; `Adwaita` with itself), and the system theme's two
  (`SystemTheme::from_system()`, `icon_theme_for` each mode).

### C.4 The committed table

`docs/evidence/widget-icons/<date>/listing.md`, its header naming the commit, the build
profile, the hash and the rule for the name found (C.2), then:

1. **The 106 names**, for each judged theme and the two bundled sets, one row per name and
   lookup, three lookups: `IconUse::Mask` at 16 px and at 24 px with no colour (what the
   handle would serve, W9), and `IconUse::Any` at the loader's default size with the
   showcase's colour (what the showcase's gallery draws). A row: the `IconName`; the name
   and form found (C.2), or "none"; the theme searched; the size: the lookup's size and the
   icon's own, an SVG's `viewBox` attribute as written (or "no viewBox") or pixels' width ×
   height; one colour: `mask_drawable`'s answer (`yes`; `no` with the reason: pixels,
   several colours, nothing painted, an embedded image) and, for an SVG, its deviation
   beside `ONE_COLOUR_TOLERANCE` (computed by the test as `one_colour_deviation` does, from
   `rasterize_svg` at 16 px: the private function is not reachable from the showcase, so
   the test repeats its few lines, named as a copy of it); the hash; the other themes with
   the same hash. For a bundled set the name found is the table's name and the theme the set's.
   For each listed, not judged theme, the 16 px `Mask` rows only. The test holds three named
   copies of the connector's private items, `one_colour_deviation`, `ONE_COLOUR_TOLERANCE`'s
   value (`T_copy`) and `embeds_image` (`embeds_copy`), and asserts for every SVG row that
   rasterises: `mask_drawable == Some(!embeds_copy && d <= T_copy)`, `d` the copied
   deviation (`d <= T_copy` false where no pixel is painted), so a copy that drifts from the
   connector fails the run.
2. **The counts**, per judged theme and bundled set: names served at 16 px and at 24 px;
   names empty at 16 px.
3. **The nine essentials** (`ESSENTIAL_ICONS`'s list, written out in the test, which runs
   and is removed before Part D exists: `WindowClose`, `WindowMinimize`, `WindowMaximize`, `WindowRestore`, `Close`,
   `ChevronDown`, `Check`, `Minus`, `Plus`), each found at 16 px or not, per judged theme
   and bundled set.
4. **The four names' candidates**, per judged theme: for `Check`, `CircleCheck`, `Loader`
   and `LoaderCircle`, one row for each name of the entry and for the role's own name
   (`emblem-default` for `Check`, `object-select-symbolic` for `CircleCheck`,
   `content-loading-symbolic` for `Loader`, `NT/src/model/icons.rs:728`, `:774`, `:775`;
   `LoaderCircle` has no role), each `FreedesktopLoader::new(name)` in the theme at 16 px:
   found or not, size, one colour, hash, and whether it is the icon served (same hash).
5. **`Loader` and `LoaderCircle` as strips**: the served SVG's `viewBox`, and for `breeze`
   and `breeze-dark` whether `FreedesktopLoader::new("process-working-symbolic")` finds an
   icon.
6. **Light and dark**, per pair (C.3): for each name the two 16 px `Mask` hashes and
   whether they are equal, and the count of equal ones.
7. **The timings** (condition 7), per switch to a judged preset, on one machine, in one
   process of the dev build, with `std::time::Instant`. A *switch* is one step of a fixed
   run the test makes: from the first preset of `Theme::list_presets()` whose icon set is
   Lucide, to each preset naming a judged theme in list order, light then dark (`kde-breeze`
   light, `kde-breeze` dark, `adwaita` light, `adwaita` dark today), then to the system's
   theme (`SystemTheme::from_system()`, its mode). For each step:
   - **the handle's loads**: the icon themes the switch makes D.8 load, by D.8's cache
     rule (a table per icon theme of the new key not yet loaded since the set last changed;
     all dropped on a change of set) with E.2's telling of both modes (the theme's two
     variants' icon themes, as the showcase will tell them): the test keeps that small
     model of the cache, a set of icon theme names cleared on a change of set, since Part
     D does not exist yet, and times `load_gpui_icon(icon, IconSet::Freedesktop,
     Some(theme), Some(16), None, IconUse::Mask)` for each of the 106 names, once per icon
     theme to load (a step that loads none times nothing: zero); and, as cost this work
     adds to a switch, `mask_drawable` of each icon the showcase's two lists load for that
     switch (the judgements E.2 and E.3 add, below), on this side;
   - **the showcase's reload of today**: what `Showcase::reload_icons` does for that switch
     without a window on the tree U3a is built on: `load_all_icons(IconSet::Freedesktop,
     Some(theme), tint)` and `load_gpui_icons(Some(IconSet::Freedesktop), Some(theme), tint)`
     (`SC/app.rs:1132-1134`), then `to_image_source(data, None, None)` for each icon the two
     loaded, as `rebuild_icon_caches` converts a system set's icons (`:718-743`). `theme` is
     the shown mode's icon theme; `tint` is `icon_tint`'s colour (`:647-652`): the resolved
     `defaults.font.color` of that variant. A switch whose theme and tint are unchanged
     reloads nothing today (`IconLoad`, `:1114-1129`): zero. Not timed: the animation caches
     (`rebuild_animation_caches`) and the redraw, so this figure is smaller than the
     reload's whole cost, and in that the comparison errs towards stopping.

   The run is made once as a warm-up (not recorded; it fills native-theme's process-wide
   caches of theme chains and file names, `NT/src/freedesktop.rs:83-100`, `:214-228`, which
   both sides share), then five times, each from its start (the model cache cleared).
   Recorded per step and side: each of the five times in milliseconds, the median and the
   largest. Condition 7 compares the two medians of each step.

8. **The slots that take an `Icon` alone** (W1): the names those slots draw, traced on
   `0fa28027` from every `svg_only` call (`SC/chrome.rs:451`, `SC/app.rs:2225`,
   `SC/demo.rs:1791`, `:3340`, `:3511`, `:3896`, `:7441`, `:7596`), and traced again at
   `a3fc81f6`, where the names are the same 23, to the names they are given: the palette's page icons (`SC/main.rs:207-221`: `Eye`, `CircleCheck`,
   `ALargeSmall`, `Inbox`, `Bell`, `CaseSensitive`, `LayoutDashboard`,
   `GalleryVerticalEnd`, `ChartPie`, `Star`, `Palette`), presets (`Palette`) and modes
   (`Settings`, `Sun`, `Moon`, `SC/chrome.rs:477-491`); the theme-error alert (`CircleX`);
   the Sidebar samples (`LayoutDashboard`, `Inbox`, `Settings`, `SC/main.rs:546-565`); the
   Buttons page's icon buttons (`Check`, `Search`, `Delete`, `SC/pages/buttons.rs:60-79`),
   two loading buttons (`Loader`, `:159`; the second, without an icon, came with the GPUI Kit
   0.10.0 port and is given the same name as its loading icon) and toggles (`Star`, `Heart`, `:215`, `:224`); the
   input group's Copy (`SC/pages/inputs.rs:126`); the Collapsible (`ChevronDown`,
   `ChevronRight`, `SC/pages/layout.rs:377-381`); the Stepper (`Search`, `Settings`,
   `CircleCheck`, `SC/support.rs:100-104`): 23 names. Per judged theme, each name's icon as
   the showcase will draw it (the role's from `load_all_icons` where `role_for_gpui_icon`
   gives a role and it finds an icon, else `load_gpui_icon` at the default size with
   `IconUse::Any` and the tint above): its source (role or table), one colour, hash, and
   whether the slot would lose it (an SVG judged not one colour). The test holds the 23
   names as a constant whose comment names the call sites by their functions
   (`palette_entry`, `Showcase::render` for `demo::alert`, `demo::sidebar`,
   `demo::button_turning` (which `demo::button` and `demo::loading_button` call),
   `demo::toggle`, `demo::input_groups`, `demo::collapsible`, `demo::stepper`, each calling
   `svg_only`);
   the implementer confirms each by reading it. The Basic page's Edit tool button, which the
   KDE steps 4 and 5 added (`IconRole::ActionEdit`), takes any element (`demo::icon_button`),
   so it is no such slot (E.14).
9. **The conditions of C.5**, each with its verdict where the table decides it.

`docs/evidence/widget-icons/<date>/README.md`, as `docs/evidence/README.md` asks of a run
by hand: what produced it (the command of C.2, the commit), the date, what the run reports
of the machine through the library and `std` (`std::env::consts::OS`; the desktop
`native_theme::detect::detect_linux_desktop()` reports; the system icon theme detected),
the judged themes' names; and the machine's versions, which `docs/evidence/README.md:5-7`
asks a run by hand to state: the judged icon themes' packages, each with its version.
The session may not read the themes' directories (they lie outside the project folder, in
`/usr/share/icons/` and the other icon base directories), so the maintainer reads them once,
on the day of the run: for each judged theme, the package that owns its directory
and that package's version, by the package manager's query of the directory's owner, or
"not from a package" for a theme installed by hand. The maintainer gives them to the session, which
writes them in a table of the README (theme, package, version) under the sentence "Read by
the maintainer on <date> from the package manager; the session does not read the icon base
directories." A theme given no version is written "not given". A listed theme that is not
judged is named without a version. And: "No drawing of an installed icon theme is kept here: each is that
theme's artwork under its own licence. `listing.md` names each icon, its size, whether it is
one colour and a hash of its bytes; the sheets the run drew were read in the repository's
`target/` and shown to the maintainer, and are not kept."

### C.5 The seven stop conditions

The plan stops for the maintainer where one is met, with the table and the sheets, which the
session opens from `target/widget-icons-sheets/<run>/`; otherwise the work goes on and the
table is part of what the maintainer sees before the release. Where none is met there is no
stop: the plan's record takes each verdict and the sheets' paths, the maintainer is told them
in the next status, and U3c follows at once.

| # | Condition (W3) | How the table decides it |
|---|---|---|
| 1 | a judged theme, or Lucide or Material, lacks an essential at 16 px | item 3 |
| 2 | a judged theme, or a bundled set, leaves more than half of the 106 empty | empty at 16 px ≥ 54 (106 / 2 = 53; "more than half") |
| 3 | for `Check`, `CircleCheck`, `Loader` or `LoaderCircle`, the first name found in a judged theme does not draw the glyph while a later name of the entry does, or two judged themes need different orders | by eye, from the `<theme>-four.png` sheets, item 4 naming each row; where no name draws the glyph, the nearest stands and the record says so (Adwaita's two cases, W3). For `Check` under Breeze this decides the entry's order |
| 4 | `Loader` or `LoaderCircle` is a strip of frames in a judged theme, or `breeze`/`breeze-dark` has no `process-working-symbolic` | item 5: a `viewBox` whose height is at least twice its width; the `new(…)` lookup |
| 5 | W9's size condition fails | per judged theme, from item 1: served at 16 px ≥ served at 24 px, and every essential found at 24 px is found at 16 px |
| 6 | a slot that takes an `Icon` alone would lose its icon in a judged theme | item 8 |
| 7 | the handle's load of a judged theme takes longer than the showcase's reload of today | item 7: in any step of the run, the handle's median above the showcase's. At the stop the maintainer judges whether it is noticeable: if so, D.13 is built with Part D; if not, the load stays on the calling thread (D.8) |

The sheets the session reads: `<theme>-16.png`, `<theme>-24.png` (the served icons, judged
themes and the bundled sets), `<theme>-four.png` (item 4, as loaded and as a mask),
`pair-<light>-<dark>-16.png` (item 6, side by side), `slots-<theme>.png` (item 8, the icons
that would be lost, as loaded).

### C.6 What is recorded

`listing.md` and `README.md` are committed in `docs/evidence/widget-icons/<date>/`, nothing
else; `docs/evidence/README.md` gains a section after `## breeze-offscreen/`:

```markdown
## `widget-icons/`

What the installed freedesktop icon themes and the bundled Lucide and Material sets give
gpui-component's 106 `IconName`s through the gpui connector's lookup, at 16 and 24 px,
whether a mask of one colour carries each, and how long the lookup takes beside the gpui
showcase's own reload of its icons: the measurement before the connector's widget icons
were built ([specification](../todo_v0.7.2_gpui-widget-icons-spec.md), Part C). Runs by
hand, one directory per date, written by the gpui showcase's ignored test
`widget_icons_listing`, which was removed once its run was recorded (commit `<sha>` holds
it); each holds `listing.md`, a table, and a `README.md`. No drawing of
an installed icon theme is kept: each icon is named, with its size, whether it is one colour
and a hash of its bytes; the sheets of a run were read in the repository's `target/` and
are not kept.

| Run | Date | Why |
|-----|------|-----|
| [`<date>/`](widget-icons/<date>/) | <date> | the seven stop conditions of the specification's C.5 |
```

The plan's record names the run, each condition's verdict and, for condition 7, both
medians per step of the run and the maintainer's word. A condition met stops the plan before
Part D; a change the maintainer then decides (`Check`'s order, the size, an entry, D.13) is
made in this specification first.

---

## Part D — The handle (W1, W2, W4, W5)

### D.1 Where it lives

A new file `CN/src/icons/widget.rs`, module `widget`, declared in `CN/src/icons.rs` after its
`use` block (`:75-87`):

```rust
#[cfg(feature = "svg-rasterize")]
mod widget;
#[cfg(feature = "svg-rasterize")]
pub use widget::{ESSENTIAL_ICONS, IconAnswer, LoadedSet, ModeIcons, ThemeIcons, WidgetIconAssets, WidgetIcons, WidgetIconsReport};
#[cfg(feature = "svg-rasterize")]
pub(crate) use widget::tell_installed;
```

(`icons.rs` and `icons/widget.rs` side by side is a file module with a child; the module
is private, so `lib.rs` reaches `tell_installed`, D.10, as `icons::tell_installed` through
that crate-visible re-export.) Every item
below is in `widget.rs` unless it says otherwise, documented (`#![warn(missing_docs)]`,
`CN/src/lib.rs:221`), with no `unwrap`, `expect`, indexing or unchecked arithmetic
(`CN/src/lib.rs:223-225`; the strict-panic run, `scripts/check_release.sh:624-627`).

### D.2 The types

- **`pub const ESSENTIAL_ICONS: [IconName; 9]`** — `WindowClose`, `WindowMinimize`,
  `WindowMaximize`, `WindowRestore`, `Close`, `ChevronDown`, `Check`, `Minus`, `Plus`
  (W4). Doc: "The glyphs nearly every application's windows, dialogs and forms draw: the
  window controls, the close button of a dialog, a sheet and a notification, the caret of
  a select and a combo box, the check mark of a check box, a chosen row and a menu item,
  and a number input's minus and plus (the minus is a mixed check box's dash too). An icon
  set that lacks one of them, in the icons of
  either colour mode, is not served (see [`WidgetIcons`]); an application replaces the
  list with [`WidgetIcons::set_essentials`]."
- **`pub(crate) const GPUI_ICON_NAMES: &[IconName]`** — the 106 names in the test module's
  order (`CN/src/icons.rs:1218-1325`), in `widget.rs`, its one user, so a build without
  the feature has no unused constant (decision 12). Test, in `widget.rs`:
  `the_library_list_is_every_icon_name`, the asset paths of `GPUI_ICON_NAMES` and of
  `crate::icons::tests::ALL_ICON_NAMES` (`pub(super)`, so visible to `icons` and its
  children), in order, are equal.
- **`pub struct ThemeIcons`** (`Clone`, `Debug`, `PartialEq`, `Eq`), private fields: the
  set, and for each mode the icon theme told, or nothing told. Constructors:
  `pub fn new(set: IconSet, light: Option<&str>, dark: Option<&str>) -> Self` (both modes)
  and `pub fn for_mode(set: IconSet, mode: ColorMode, icon_theme: Option<&str>) -> Self`
  (one mode). Conversions: `From<&native_theme::Resolved>` (`for_mode(r.icon_set, r.mode,
  r.icon_theme)`), `From<&SystemTheme>` (`new(sys.icon_set, sys.icon_theme_for(Light),
  sys.icon_theme_for(Dark))`), `From<&crate::Loaded>` (`for_mode(l.icon_set, l.mode,
  l.icon_theme)`). Doc: "An icon set and the freedesktop icon theme of each colour mode,
  as the handle is told them. A `Resolved` and a `Loaded` carry one mode's icon theme:
  told one mode, the handle keeps the other mode's while the set stays the same, and takes
  the told one for both when the set changes, until it is told the other. An icon theme of
  `None` is the system's, detected when the handle makes a choice from it (it is told it
  while following, fixed to it, or set following again).
  The icon themes matter for the freedesktop set only."
- **`pub enum IconAnswer`** (`Clone`, `Debug`, `PartialEq`, `Eq`, `#[non_exhaustive]`):
  `Draw(SharedString)` (the set's icon's path), `Empty(SharedString)` (the set has none for
  this name: the empty icon's path), `NotMine`. Doc per W2's "A public lookup".
- **`pub enum WidgetIconsReport`** (`Clone`, `Debug`, `PartialEq`, `Eq`,
  `#[non_exhaustive]`), in the order D.9 checks: `NotInstalled` (`install` never ran for
  this handle, so no resolver asks it); `Off` (switched off); `NoAssetSource` (installed,
  but this handle's `assets()` was never called, so no served path could load: D.6); `NotTold` (following the theme, told nothing yet); `NotServed { set:
  IconSet, fixed: bool }` (SF Symbols, Segoe Fluent or a set this connector does not know,
  W7); `Loaded(LoadedSet)`.
- **`pub struct LoadedSet`** (same derives, `#[non_exhaustive]`, public fields): `set:
  IconSet`; `fixed: bool` (the application's fixed choice, else the theme's); `light:
  ModeIcons`; `dark: ModeIcons`; `unknown_essentials: Vec<SharedString>` (names of the floor
  outside the 106 the tables know, an `IconName` newer than this crate: not judged, D.8);
  and `pub fn served(&self) -> bool` (neither mode gave nothing nor lacks an essential).
- **`pub struct ModeIcons`** (same derives, `#[non_exhaustive]`, public fields):
  `icon_theme: Option<String>` (the freedesktop icon theme loaded, the detected one where
  none was told; `None` for the bundled sets, or where detection failed); `gave_nothing:
  bool`; `missing_essentials: Vec<SharedString>`; `missing: Vec<SharedString>` (every name
  the mode's table answers `Empty`: drawn empty while the set is served, and, where it is not
  served, the names it lacks, for the application to see why); `kept: Vec<SharedString>`
  (`StarFill` under Lucide, which keeps upstream's glyph). Every name is the `IconName`'s
  asset path (decision 13).
- **`pub struct WidgetIcons`** (`Clone`; `Debug` written by hand, printing the report;
  `Default` = `new()`), private field: the shared state (decision 8, D.4).
- **`pub struct WidgetIconAssets<A>`**, private fields `inner: A` and the shared state;
  `impl<A: gpui::AssetSource> gpui::AssetSource for WidgetIconAssets<A>`.
- **`ThemeIcons::merged(&self, told: &ThemeIcons) -> ThemeIcons`** (private): D.2's rule
  for telling one mode, which `set_theme_icons` applies and
  `told_one_mode_keeps_the_other_while_the_set_stays` tests without an `App`.

### D.3 The methods of `WidgetIcons`, and the state they leave

The handle holds: whether `install` ran (*installed*) and whether its `assets()` was
called (*wrapped*), two flags of the shared state (D.4); the theme's icons as last told
(*told*, possibly nothing); its *state*, `Following`, `Fixed(ThemeIcons)` or `Off`; the
*floor*; a *cache* of loaded tables; and *serving*, the record of the last choice served
(D.8). The state makes the *choice*: `Following` → told (none while nothing is told),
`Fixed(x)` → `x`, `Off` → none.

The signatures, all methods of `impl WidgetIcons` (`App` is gpui's, through
`extern crate gpui_kit as gpui`, `CN/src/lib.rs:228`):

```rust
pub fn new() -> Self
pub fn assets<A: gpui::AssetSource>(&self, inner: A) -> WidgetIconAssets<A>
pub fn install(&self, cx: &mut App)
pub fn set_theme_icons(&self, icons: impl Into<ThemeIcons>, cx: &mut App)
pub fn follow_theme(&self, cx: &mut App)
pub fn fix(&self, icons: impl Into<ThemeIcons>, cx: &mut App)
pub fn switch_off(&self, cx: &mut App)
pub fn set_essentials(&self, icons: impl IntoIterator<Item = IconName>, cx: &mut App)
#[must_use] pub fn lookup(&self, path: &str, cx: &App) -> IconAnswer
#[must_use] pub fn report(&self) -> WidgetIconsReport
```

`new` and `assets` are `#[must_use]` too; `impl Default for WidgetIcons` calls `new` (which
clippy's `new_without_default` asks for).

| Method | `Off` | `Following` | `Fixed(x)` |
|---|---|---|---|
| `new()` | — | the start: told nothing, floor `ESSENTIAL_ICONS`, neither installed nor wrapped, empty cache, nothing serving; loads nothing | — |
| `assets(inner)` | wrapped from now on (D.6); returns the wrapper | the same | the same |
| `install(cx)` | installed from now on; stores a clone in the private global `InstalledWidgetIcons(WidgetIcons)` (`impl Global`), sets the resolver (`gpui_component::set_icon_resolver(cx, …)`, a closure holding a clone, mapping `Draw(p)`/`Empty(p)` to `Some(p)`, `NotMine` to `None`), refreshes the windows; replaces a resolver and a handle installed before | the same | the same |
| `set_theme_icons(t, cx)` | told := told merged with `t`; nothing loaded | told := merged; the choice is told: D.8 | told := merged; nothing else |
| `follow_theme(cx)` | → `Following`: D.8 for told; where serving still holds told's key (an `Off` after following, nothing told since), it is served again without a load | nothing | → `Following`: D.8 for told |
| `fix(t, cx)` | → `Fixed(t)` (a mode not given takes the given one's): D.8 | the same | the same, `t` replacing `x` |
| `switch_off(cx)` | nothing | → `Off`; the cache and serving are kept | → `Off`; the same |
| `set_essentials(l, cx)` | floor := `l`; serving judged again (D.8 step 5), no load | the same | the same |
| `lookup(path, cx)` | `NotMine` | D.7 | D.7 |
| `report()` | D.9 | D.9 | D.9 |

The cache and serving are kept across every change of state (`Off` included): a later choice
of the same set reuses the cache, and a choice whose key serving holds is served again with
no load (D.8). Each method that changes what `lookup` answers calls `cx.refresh_windows()`
(the hook's documentation, `GC/icon.rs:90-91`); one that changes nothing does not. A `None` icon
theme of a freedesktop choice is resolved by `system_icon_theme()` once, when a method makes
a choice (D.8), before anything is compared (decision 24): the choice's key names the icon
themes as detected, and a failed detection is kept as `ModeKey::DetectionFailed` for that
mode.

**The type's documentation** (the handle's doc, the README's section in short): what it
is (W2's three things: an asset source around the application's, the resolver, the theme's
icons and three states); how an application installs it, with a `no_run` example the
README's quick start repeats, and that the handle installed must be the one whose `assets()`
went to `Application::with_assets` (else the report says `NoAssetSource` and the widgets keep
gpui-component's glyphs); that `apply_system_theme` tells it and an application on the
`apply` path tells it (`set_theme_icons(&resolved)`); that it follows the mode; that it
answers the 106 paths of gpui-component's `IconName` whoever draws them, an application's
own artwork under those paths included; that a set lacking an essential, a set that gives
nothing, and SF Symbols and Segoe Fluent are not served (the widgets keep gpui-component's
glyphs), and `StarFill` under Lucide keeps upstream's; that an icon the set lacks is drawn
empty, with `report` saying which; that it needs `svg-rasterize`.

### D.4 Sharing (decision 8)

`Shared`, behind an `Arc`, holds `state: RwLock<Arc<State>>`, `installed: AtomicBool`,
`wrapped: AtomicBool` and a load counter, `loads: AtomicUsize` (`fetch_add`, which wraps rather than panics; read by the
tests, D.12). `State` (private, immutable once built): told, the state, the floor, the
current choice's key, `choice: Option<IconsKey>` (D.8's key of the choice the state makes,
D.3, resolved when the method made it, decision 24; `None` where there is no choice: `Off`,
or following with nothing told), the
cache (the set it belongs to and its tables by icon theme, each table an `Arc<Table>`, so
that a new `Cache` shares the tables it keeps instead of copying their bytes; a `Table`:
asset path → `Draw(path)`, `Empty` or `NotMine`, with the bytes by served path,
`HashMap<SharedString, Cow<'static, [u8]>>`, the bundled sets' bytes borrowed,
`Cow::Borrowed`, a theme's owned), and *serving*: the last choice's key with its two tables
and its report (served or declined by the floor, D.8 steps 5 and 6), or the not-served
marker of a choice whose set this release does not serve (D.8 step 1, W7), or nothing. The
key is kept because neither of its two readers may detect again: D.7's lookup, which does no I/O,
compares it with serving's key, and D.13's landing load compares it with the load's key. In
the synchronous build of D.8 serving's key equals `choice` whenever `choice` is `Some`, since
every method that makes a choice ends in D.8 step 1 or 6 or finds serving holding the key
already; D.13 is where the two differ, while a load is in flight. Readers
clone the inner `Arc<State>` under the read lock; writers replace it. The private alias
`type SharedState = Arc<Shared>;` names the field's type, so no signature or field trips
clippy's `type_complexity`.

### D.5 The paths (W5, decision 10)

- A bundled set: `native-theme-gpui/<set>/<file>`, `<set>` the set's `IconSet::name()`
  (`lucide`, `material`; `NT/src/model/icons.rs:482-490`) and `<file>` the part of the
  `IconName`'s path after `icons/` (`check.svg`).
- A freedesktop theme: `native-theme-gpui/freedesktop/<icon theme>/<file>`, the icon theme
  as loaded (a directory name: it holds no `/`).
- The empty icon: `native-theme-gpui/empty.svg`.

One path per set, icon theme and name; another set changes every path, so a drawn icon of
one set is never shown for another (GPUI keeps a drawn SVG by its path and size,
`GP/window.rs:5247-5262`). The icons of a set no longer served stay in GPUI's cache (W5).

### D.6 The asset source, and an application that does not use it

`assets(inner)` sets `wrapped` and returns a `WidgetIconAssets` sharing the handle's state.
Its `load(path)`: the empty icon's path → `Ok(None)` (decision 9); a served path of the
current serving → `Ok(Some(bytes.clone()))`; any other path → `inner.load(path)`.
`list(path)` → `inner.list(path)` (the handle's paths are not listed). The bytes of a set no
longer served are not kept: GPUI has drawn and cached them at the sizes already drawn, and a
new size asks for the current set's paths only, since the resolver answers no other.

An installed handle whose `assets()` was never called would answer paths no asset source
holds: gpui-kit's `Assets` returns an error for a path it lacks (`GA/src/native_assets.rs:23-29`),
`render_alpha_mask` passes it on (`GP/svg_renderer.rs:257`), no tile is inserted
(`GP/platform.rs:1864-1866`) and `paint_svg`'s error is logged at each draw
(`GP/elements/svg.rs:182-185`): every widget icon would vanish, the floor's included. So
`lookup` answers `NotMine` while `wrapped` is false, and the report says `NoAssetSource`. A
wrapper given to another `App`, or a second handle's wrapper, cannot be told apart; the
README and the type's doc say the installed handle must be the wrapped one.

### D.7 The lookup and the resolver: one rule

`lookup(path, cx)` answers from tables only where all of these hold, and `NotMine`
otherwise: the handle is installed and wrapped; the state is not `Off`; there is a choice; *serving* holds
that choice's key (the set and both modes' icon themes, as resolved by D.3), the state's
`choice` (D.4), compared without a second detection; the choice's set
is served under the floor in both modes (D.8 step 5); the path is one of the 106; and the
mode is known (decision 11). Then it answers the mode's table: `Draw(served path)`,
`Empty(empty path)`, or `NotMine` (`StarFill` under Lucide). So every transition of D.3
falls under this one rule: after `fix(Material)` then `follow_theme` with nothing told,
there is no choice; told SF Symbols while following, serving holds the not-served marker
for that choice, never the previous set's tables (W7); a set that fails the floor is
`NotMine` throughout. The resolver set by `install` maps `Draw(p)` and `Empty(p)` to
`Some(p)`, `NotMine` to `None`. It does no I/O and no allocation beyond cloning the path's
`SharedString` (an `Arc`).

### D.8 Loading, the floor and serving; the cache

Where a method makes a choice (D.3), on the calling thread. The choice is first resolved
into its key, `IconsKey` (private: `{ set: IconSet, light: ModeKey, dark: ModeKey }`, with
`enum ModeKey { NoIconTheme, Theme(String), DetectionFailed }`): for every set but
Freedesktop (Lucide, Material, and SF Symbols, Segoe Fluent and any set this connector does
not know, the not-served marker's key included) both modes are `NoIconTheme`; a freedesktop
mode is the told name, or, where it was told `None`, `system_icon_theme()`'s name or
`DetectionFailed`, detected once when the choice is made (decision 24). `DetectionFailed`
is reachable from the system theme too: `SystemTheme::icon_theme_for` is `None` only where
detection failed (`NT/src/lib.rs:740-753`). Where serving already holds the choice's key (a
choice served before, then `Off`, then followed or fixed again), nothing is loaded or judged
and only the state changes.

1. SF Symbols, Segoe Fluent, or a set this connector does not know: a change of set, so the
   cache is emptied as in step 2; nothing loaded; serving becomes the not-served marker for
   that choice; report `NotServed`.
2. A set other than the cache's: the cache is emptied and belongs to the new set (W5 ties a
   new set to a new path set; its old tables would never be answered again).
3. Lucide, Material: one table, `load_gpui_icon(icon, set, None, None, None, IconUse::Mask)`
   for each of `GPUI_ICON_NAMES`, used for both modes; loaded only where the cache has none.
4. Freedesktop: one table per icon theme of the choice's two modes (one where they are the
   same), each name `load_gpui_icon(icon, Freedesktop, Some(theme), Some(16), None,
   IconUse::Mask)` (W9's 16 px, a private `const WIDGET_ICON_SIZE: u32 = 16`); loaded only
   where the cache has no table of that icon theme, so a switch loads only the icon themes
   not yet loaded since the set last changed (a light/dark switch under kde-breeze loads
   `breeze-dark` once, and nothing when it comes back). A mode whose key is
   `DetectionFailed` loads nothing and gets an empty table, every name `Empty`, as a theme
   that is not installed gives one; so step 5 finds it `gave_nothing`, lists every name in
   `missing` and every essential in `missing_essentials`, and its `ModeIcons::icon_theme` is
   `None`; it counts no load.
5. A table's answers: a name found → `Draw(path)` with its bytes; not found → `Empty`; under
   Lucide, `StarFill` → `NotMine`, reported as kept. Per mode: no `Draw` → `gave_nothing`;
   an essential of the floor whose answer is `Empty` → `missing_essentials` (`NotMine`, the
   kept `StarFill`, is no missing name); a floor name outside `GPUI_ICON_NAMES` (an
   `IconName` newer than this crate) is not judged and is listed in `unknown_essentials`.
   The set is served where neither `gave_nothing` nor `missing_essentials` holds in either
   mode (W4).
6. Serving becomes the choice's key with its two tables and report (served or declined by
   the floor); the new `State` replaces the old; the windows are refreshed where what
   `lookup` answers changed (D.3), not otherwise.

Steps 2 to 5 are the private function `serve(key: &IconsKey, floor: &[SharedString], cache:
&Cache) -> Served`, which needs no `App`; `Served` holds the new `Cache` (the kept tables'
`Arc`s cloned, D.4), the new serving and `loaded: usize`, the number of tables it loaded in
steps 3 and 4 (counted with `saturating_add`). `serve` touches no counter; its caller, the
method, adds `loaded` to the handle's `loads` (`fetch_add`) in step 6, so the
`#[gpui::test]` tests read `loads` and the plain fixture tests read `loaded` (D.12).

The cache keeps every table of the current set loaded since the set last changed, and drops
them all when the set changes (step 2): within a run that is one table per icon theme a user
or a theme chose under one set, each the bytes of at most 106 SVGs. The same key again loads
nothing (W2, "Loading"); a change of the floor judges the tables again (step 5) without
loading. Telling the theme's icons while fixed or off loads nothing.

### D.9 The report

`report()`, checked in this order: not installed → `NotInstalled` (so a wrapped handle that
was never installed says so, not `Loaded`); `Off` → `Off`; not wrapped → `NoAssetSource`;
following and told nothing → `NotTold`; a load in flight for the current choice (D.13 only)
→ `Loading`; serving's not-served marker → `NotServed { set,
fixed }`; else `Loaded(LoadedSet)` of serving, with `fixed` and the two `ModeIcons` (step 5's
findings), whether served or not. The README tells an application to check it for the sets
it supports.

### D.10 `apply_system_theme` tells the handle

`CN/src/lib.rs:1187-1196`. After `apply_inner(…)`, under `#[cfg(feature =
"svg-rasterize")]`, a call of a crate-private function in `widget.rs`,
`icons::tell_installed(sys, cx)` (`pub(crate) fn tell_installed(sys: &SystemTheme, cx: &mut
App)`, re-exported to the crate by D.1), which clones the handle of the `InstalledWidgetIcons` global
where one is installed (`cx.try_global`) and calls `set_theme_icons(sys, cx)`; with no
handle installed it returns and does nothing, so `apply_system_theme` behaves as today. The
doc (`:1173-1186`) gains,
after its first paragraph: "Where the application installed a
`icons::WidgetIcons` handle (feature `svg-rasterize`), it tells the handle
the theme's icon set and the icon theme of each mode, so gpui-component's widgets draw the
theme's icons." (In code font without a link: the item exists only with the feature, and
a build of the docs without it would report a broken link under rustdoc's `-D warnings`.)
The signature does not change; `apply` does not change (its `ResolvedTheme` names no set).
`from_preset` does not change.

### D.11 The crate's other edits

- `CN/src/lib.rs:16`, "`//! - **icons** in the theme's icon set: [`icons`];`" becomes two
  lines, wrapped at the list's 79 columns (the item on `:14` is 79 wide; one line would be
  82): "`//! - **icons** in the theme's icon set, gpui-component's widgets' too:`" and
  "`//!   [`icons`];`" (the continuation indented as the list's other continuations).
- `CN/src/lib.rs:20-52`, the crate doc's *Quick Start*: its block (`:25-45`) stays as it
  is, and one sentence is appended to the paragraph after it (`:47-52`), after
  "builders.": "To have gpui-component's widgets draw their own icons in the theme's set,
  an application installs an `icons::WidgetIcons` handle (feature `svg-rasterize`; see the
  README's *The widgets' own icons*)." (the handle in code font without a link, as `:18`'s
  `widgets` and D.10's doc line, for the docs build without the feature). The block does
  not gain the handle, though the README's quick start does (F.1 item 2): a crate-root
  doctest that needs `svg-rasterize` would fail a plain `cargo test -p native-theme-gpui
  --no-default-features`, doctests included (`WidgetIcons` is behind the feature, D.1); it
  would be the first crate-doc example outside a cfg'd module to need a feature (the
  `watch` example, `:207`, is `rust,ignore`); and it would be a third copy of the code the
  handle's doc (D.3) and the README hold. Wrapped at the paragraph's 79 columns, `:52`
  ("`//! builders.`") becomes three lines.
- `CN/src/lib.rs:277-280`, the README doctest gate: the doc becomes the three lines "`/// The
  README's code blocks, compiled as doctests so they cannot drift from the API; the`",
  "`/// runtime-theme-change block needs `watch`, the theme-drawn widgets block `widgets`, and`",
  "`/// the widget icons' blocks `svg-rasterize`.`"; the attribute becomes
  `#[cfg(all(doctest, feature = "watch", feature = "widgets", feature = "svg-rasterize"))]`.
  Same line count.
- `CN/src/icons.rs`'s module doc: the overview table (`:5-19`) gains rows for
  `load_gpui_icon` ("Look up gpui-component's [`IconName`] in an icon set by the
  connector's tables, as the widgets' own icons are served"), `mask_drawable` ("Whether
  gpui's one-colour mask carries an icon") and `WidgetIcons` ("Serve gpui-component's
  widgets the theme's icons (feature `svg-rasterize`)"), the last as code without a link;
  and a section after the end of the module doc (its last paragraph `:69-73`; the prose of
  *Role icons*, from `:21`, runs to there), `# The icons gpui-component's widgets draw`,
  saying in four sentences what D.3's type doc says, pointing to `WidgetIcons` and to the
  README. The sentence "An icon the set does not have is `None`; nothing is substituted."
  (`:29-30`) stays: it is about the role icons, and the handle's exceptions are stated in its
  own doc. The sentence after it (`:30-32`), "[`icon_name`] is another thing: the
  [`IconName`] of gpui-component's bundled Lucide glyph, whatever the theme's set.", is no
  longer true with the handle installed; its two lines `:31-32` become
  "`//! [`IconName`] of gpui-component's bundled Lucide glyph, drawn in the`" and
  "`//! theme's set instead where an installed `WidgetIcons` handle serves it.`" (code
  without a link, as the table's row; same line count).
- `CN/src/widgets/checkbox.rs`, two doc comments. Since B.13 of the 0.9.1 specification and
  the GPUI Kit 0.10.0 port, the `None` arm of `look.mark_stroke` asks the application's
  resolver for `IconName::Check`, or for `IconName::Minus` where the box is indeterminate, so
  an installed handle serves the check box's and the radio's unstroked mark, and the dash,
  from the theme's set. The docs of `CheckboxLook::mark_stroke` and of `Radio`, and the
  comment above the `None` arm, already say the mark goes through the application's icon
  resolver; that holds with the handle, and they stay as they are. The two edits:
  - the last line of `CheckboxLook::dot`'s doc, "`/// the radio then draws gpui-component's
    own mark.`", becomes "`/// the radio then draws the check box's mark (`mark_stroke`).`":
    one line for one, since the colour claims cite lines below it (*Lines that must not
    move*);
  - `Checkbox`'s doc, its paragraph that begins "The check mark:", gains after its last
    sentence: "With an installed `icons::WidgetIcons` (feature `svg-rasterize`), that
    resolver answers both from the theme's icon set where the handle serves it." (code
    font without a link, as D.10's doc line; it lies below every line the claims cite, so
    its line count is free).

### D.12 Tests (`widget.rs`'s `mod tests`)

`#[gpui::test]` tests, each with `gpui_kit::init(cx)` first and, unless it tests the
absence, `assets(())` called before `install` (the test `App`'s own asset source is `()`,
`GP/app/test_context.rs:176`, which answers `Ok(None)` for every path, `GP/assets.rs:21-29`);
the asset source tested by calling `AssetSource::load` on the `WidgetIconAssets<()>` that
`assets(())` returns. A load count is read from the handle's `loads` before and after the
action:

- `a_path_outside_the_106_is_not_mine` (`lucide-icons`): installed and serving Lucide,
  `resolve_icon_path` of `icons/no-such.svg` is unchanged and `lookup` is `NotMine`.
- `without_its_asset_source_the_handle_serves_nothing` (`lucide-icons`): installed, told
  Lucide, `assets()` never called: every one of the 106 paths `NotMine`, `resolve_icon_path`
  unchanged, the report `NoAssetSource`; after `assets(())`, served.
- `lucide_is_served_and_star_fill_kept` (`lucide-icons`): after
  `set_theme_icons(ThemeIcons::new(Lucide, None, None))`, 105 paths resolve to
  `native-theme-gpui/lucide/<file>`, `icons/star-fill.svg` is unchanged; the report is
  `Loaded` with `served()` and `kept == ["icons/star-fill.svg"]` and `missing` empty in both
  modes (the kept `StarFill` is no missing name, D.8 step 5); the asset source answers each
  served path with the bytes `LucideLoader` gives for the table's name, the empty path with
  `Ok(None)`.
- `material_draws_star_off_empty` (`material-icons`): `icons/star-off.svg` → `Empty`;
  `missing == ["icons/star-off.svg"]`.
- `the_three_states` (`lucide-icons`, `material-icons`): told Lucide (loads +1), Lucide
  served; `fix(Material)` (loads +1; the cache, Lucide's, is dropped as the set changes)
  serves Material; telling Lucide again while fixed (loads +0) keeps Material; `follow_theme`
  serves Lucide (loads +1: D.8 step 2 dropped Lucide's table); `switch_off` makes every path
  `NotMine` and the report `Off` (loads +0); `follow_theme` again serves Lucide (loads +0:
  serving still holds Lucide's key, D.8).
- `a_choice_with_nothing_told_serves_nothing` (`material-icons`): `fix(Material)`, then
  `follow_theme` with nothing told: every path `NotMine`, the report `NotTold`.
- `sf_symbols_and_segoe_fluent_are_not_served` (`lucide-icons`): serving Lucide, then told
  SF Symbols while following: every path `NotMine` (not Lucide's), the report
  `NotServed { set: SfSymbols, fixed: false }`; told Lucide again, Lucide is
  served and loads +1 (the choice of SF Symbols was a change of set and emptied the cache,
  D.8 step 1); the same for `fix` of Segoe Fluent, `fixed: true`.
- `told_one_mode_keeps_the_other_while_the_set_stays`: `ThemeIcons::merged`, without an
  `App`: Freedesktop `breeze`/`breeze-dark` told, then `for_mode(Freedesktop, Light, "a")`
  keeps dark `breeze-dark`; `for_mode(Lucide, …)` then `for_mode(Freedesktop, Dark, "b")`
  gives `b` for both.
- `the_same_set_loads_once` (`lucide-icons`): two `set_theme_icons` of the same icons, loads
  +1.
- `a_floor_name_outside_the_tables_is_listed` (`lucide-icons`, served Lucide being what the
  floor is judged against): `set_essentials` with every name of
  `GPUI_ICON_NAMES` (no newer variant exists to test with, at 0.9.1 or at 0.10.0, so the
  test checks the
  other half: `unknown_essentials` is empty and the floor judged), written so that a newer
  variant added upstream shows in `unknown_essentials` (its doc says so).
- `a_handle_never_installed_says_so` (`lucide-icons`): wrapped and told Lucide, never
  installed: the report is `NotInstalled`, `lookup` `NotMine`.
- In `#[cfg(all(target_os = "linux", feature = "system-icons"))]`, child-process fixtures,
  each a plain `#[test]` returning `Result<(), Box<dyn std::error::Error>>` that re-runs
  itself in a child process through `crate::icon_fixture::run` with the themes it writes,
  as `CN/src/icons.rs:2104-2130` does. They call D.8's `serve(key: &IconsKey, floor:
  &[SharedString], cache: &Cache) -> Served` and D.7's choice of a table, a private
  `answer(state: &State, path: &str, dark: bool) -> IconAnswer`, and read the serving's
  report and `Served::loaded`, with no `App`; only `the_resolver_follows_the_mode` needs one, for `GpuiTheme::change`, and is a
  `#[gpui::test]` in the same child-process form:
  `a_set_without_an_essential_is_not_served` (a theme without `list-add`: every path
  `NotMine`, `missing_essentials == ["icons/plus.svg"]`, and `missing` lists
  `icons/plus.svg`), `an_empty_floor_serves_what_the_set_gives` (the same theme after
  `set_essentials([])`: served, `icons/plus.svg` → `Empty`), `a_set_that_gives_nothing_is_not_served`
  (an icon theme that is not installed: `gave_nothing` in both modes),
  `the_resolver_follows_the_mode` (two themes whose `window-close.svg`s differ, told as
  light and dark; `GpuiTheme::change(Dark, None, cx)` turns `icons/window-close.svg`'s answer
  to the dark theme's path, with no call to the handle), `a_freedesktop_path_names_its_theme`
  (the served path is `native-theme-gpui/freedesktop/<theme>/window-close.svg`),
  `a_mode_switch_loads_only_the_new_icon_theme` (`serve` of the key (`light-a`, `light-a`)
  over an empty cache: `loaded == 1`; of (`light-a`, `dark-a`) over the cache it returned:
  `loaded == 1`; of (`light-a`, `light-a`) again over the next: `loaded == 0`).
- `a_mode_whose_detection_failed_gives_nothing` (a plain `#[test]`, no fixture needed):
  `serve` of the key `{ set: Freedesktop, light: Theme("light-a"), dark: DetectionFailed }`
  over an empty cache, with no theme installed under that name either (so `light` gives
  nothing too), and of `{ light: DetectionFailed, dark: DetectionFailed }`: in the dark mode
  `loaded` counts no load for it, its `ModeIcons` has `gave_nothing`, `icon_theme == None`,
  every name in `missing` and the nine in `missing_essentials`; the set is not served. The
  first key gives `loaded == 1`: by D.8 a load is a table built in step 3 or 4, and step 4
  builds `light-a`'s table (the 106 lookups run and find nothing, so every name is `Empty`
  and light too has `gave_nothing`, with `icon_theme == Some("light-a")`), while the
  `DetectionFailed` mode builds none. The second key gives `loaded == 0`.
- `apply_system_theme_tells_the_installed_handle` (`#[cfg(all(feature = "svg-rasterize",
  feature = "lucide-icons"))]`, in `CN/src/lib.rs`'s
  `apply_tests`, with `super::tests::system_theme()` as its neighbours, `:2880-2896`):
  `sys.icon_set = IconSet::Lucide`, the handle wrapped and installed,
  `apply_system_theme(&sys, cx)`: the report is `Loaded` Lucide, not fixed.

**Globals and test windows.** The installed handle is a gpui global (`cx.set_global`),
kept per `App`, as gpui-component's `IconResolver` is (`GC/icon.rs:77-97`); every
`#[gpui::test]` builds its own `App`, so a handle installed in one test is not seen by
another. The handle's state and its load counter live in the `WidgetIcons` value a test
creates, so they are per test too, whichever thread runs the test or a task. gpui's test
windows are never active; nothing of the handle reads a window's activation (the resolver
reads the mode from the `App`'s globals), and `refresh_windows` with no window open does
nothing.

### D.13 If the load moves to a background task (condition 7)

Built only where the measurement meets condition 7 and the maintainer finds the longer load
noticeable (C.5); otherwise D.8 stands and this section is left out of the plan. What it
changes, and nothing else:

- Where D.8 would load (steps 3 and 4, a table not in the cache), the method computes the
  choice's key and the icon themes to load, and starts a task on gpui's background executor
  (`cx.background_executor().spawn`) that runs `load_gpui_icon` for each (it needs no `App`;
  `IconData`, `SharedString` and the tables are `Send`), awaited by a foreground task
  (`cx.spawn`) that is `.detach()`ed (a dropped gpui `Task` is cancelled). The method
  returns at once. Steps 1 and 2 and the floor stay on the calling thread. A load in flight
  is recognised by the state's set of loads in flight, the icon themes (with their set)
  being loaded; a choice whose missing tables are all in flight already starts no second
  load and waits for them; its key is the current choice when they land.
- On completion the foreground task merges the loaded tables into the cache of the state as
  it is then (where the cache still belongs to the load's set; else they are dropped),
  judges the floor with the floor as it is then (step 5), and makes them the serving only
  where the state's current choice still equals the load's key (its `choice`, D.4); otherwise the tables stay
  in the cache for a later choice and nothing else changes. A later choice made meanwhile
  (one that loads, or one that loads nothing, such as `follow_theme` back to a set already
  served, or `switch_off`) is therefore never undone by an earlier load.
- Until the tables are in, `lookup` answers by D.7's rule from what is serving: the previous
  choice's tables where they are still the current choice's (none, while the new choice is
  loading), so the widgets show gpui-component's glyphs (`NotMine`) while the new set loads.
  The report then gives `Loading { set, fixed }`, a new variant of `WidgetIconsReport` (it is
  `#[non_exhaustive]`), in D.9's order after `NotTold`.
- The tests of D.12 that call these methods then run `cx.run_until_parked()` before they
  assert; three more: `a_later_choice_wins_over_a_load_in_flight` (following Lucide served;
  `fix(Material)` starts a load, emptying the cache as a change of set; `follow_theme` before
  it ends starts Lucide's load; after `run_until_parked`, Lucide is served, Material's late
  table is dropped since the cache belongs to Lucide, the report is `Loaded` Lucide),
  `switching_off_during_a_load_stays_off`, and `a_second_choice_of_a_loading_theme_loads_once`
  (told the same icons twice before the executor runs: loads +1). The load counter is the shared atomic of D.4, so
  it counts a load whichever thread runs it.
- The README's section (F.1) says the load runs in the background and the widgets draw
  gpui-component's glyphs until it ends; the showcase's report caption (E.5) gains, for
  `Loading`, "gpui-component's widgets: their own icons until {set} is loaded".

---

## Part E — The showcase (W1, W3, W6, W9, W10)

Every handle-related item below is under `#[cfg(feature = "svg-rasterize")]`, with the
`#[cfg(not(…))]` arm named, as the showcase pairs `widgets` today
(`SC/pages/basic.rs:316-322`); the no-feature build is what
`scripts/check_release.sh:549-550` lints.

### E.1 `SC/main.rs`: the asset source and the handle

`:1092-1093`, `gpui_kit::application().with_assets(gpui_kit::assets::Assets)`: with the
feature, a `WidgetIcons::new()` is created before it and the call becomes
`.with_assets(icons.assets(gpui_kit::assets::Assets))`; inside `run`, after `app::init(cx)`
(`:1096`), `app::install_widget_icons(&icons, cx)`. Without the feature, unchanged.

### E.2 `SC/app.rs`: the handle's global and its state

- New, beside `GPUI_BUILTIN_ROW` (`:618-620`): `pub(crate) struct
  ShowcaseWidgetIcons(pub(crate) WidgetIcons)`, `impl Global`, and `pub(crate) fn
  install_widget_icons(icons: &WidgetIcons, cx: &mut App)`: `icons.install(cx)` and the
  global set (decision 14). `main` (E.1) calls `assets` on the handle before.
- **Without the global.** Every reader of it takes it with `cx.try_global::<ShowcaseWidgetIcons>()`
  and handles `None` with no panic: `reload_icons` tells nothing; E.4's `spinner_default`
  answers `Upstream` (the spinners as today); E.5's caption takes the `NotInstalled` text. And
  `apply_system_theme`'s tell (D.10) does nothing with no handle installed (`tell_installed`
  reads its own global with `try_global` and returns where there is none).
- **The test harness** (decision 27). `open_with` (`SC/tests.rs:119-130`) installs no
  handle, so no test it opens loads the machine's installed icon themes, and the chrome and
  the samples draw as with no widget icons (their own icons do not depend on the handle; the
  two loading buttons keep today's override, E.4). A new helper, `open_with_widget_icons(cx:
  &mut TestAppContext, options: gpui::WindowOptions)`, with `open_with`'s parameters
  (`SC/tests.rs:119-122`; a test calls it with `crate::window_options(…)` as `open` builds
  them, `:109-115`), is `open_with` plus a new handle, `assets(())` called on it and the
  wrapper dropped (the call marks the handle wrapped, D.6), and `install_widget_icons(&icons,
  cx)` after `crate::app::init(cx)`, so the handle follows the Select as in the application.
  The test `App`'s own asset source is `()` (`GP/app/test_context.rs:176`), which answers
  `Ok(None)` for every path (`GP/assets.rs:21-29`), and the dropped wrapper never reaches
  gpui: so in these tests a served glyph draws nothing (on `Ok(None)` the atlas inserts no
  tile, `GP/platform.rs:1857-1867`). Nothing breaks, since their assertions read the report,
  the captions and the info notes, not a glyph's pixels. E.11's five tests use it, and they alone:
  `the_icon_theme_select_puts_the_widget_icons_in_its_state`,
  `the_widget_icons_follow_the_set_the_chrome_draws`,
  `a_fixed_row_keeps_the_widget_icons_where_the_detection_fails`,
  `the_loading_buttons_keep_the_spinners_icon_where_the_set_is_served` and
  `the_icons_page_reports_the_widget_icons`.
- **The other mode's icon theme** (`#[cfg(feature = "svg-rasterize")]`, as its only reader
  is the handle's telling, so the no-feature clippy run, `scripts/check_release.sh:549-550`,
  sees no unread field). The showcase keeps, beside `current_icon_theme` and
  `has_toml_icon_theme` (`:589-593`), the other mode's: `other_icon_theme: Option<String>` and
  `other_icon_theme_explicit: bool`; the icon set is one per theme (`NT/src/model/mod.rs:283-286`),
  so `current_icon_set` serves both. `Showcase::new` (`:1726-1736`) starts both at `None`
  and `false`, and its `Err` branch (`:1748-1752`) leaves them so. `install_system`
  (`:1816-1855`) takes them from the `SystemTheme`, which holds both variants
  (`system.icon_theme_for(other mode)`, explicit as the shown mode is, `:1831-1832`).
  `install_preset` (`:1859-1888`) holds the `Theme`, `nt`, with both variants, and resolves
  only the shown mode (`:1867`); with the feature it also resolves `nt` in the other mode and
  keeps that `Resolved`'s `icon_theme` and its own `icon_theme_explicit` (the three-tier
  precedence of `Resolved::icon_theme`, `NT/src/model/resolved.rs:279-287`, not a copy of
  it). Where that second resolution fails, the other mode takes the shown mode's icon theme,
  so a stale mode of an earlier theme never decides the floor (D.8 step 5 judges both
  modes).
- **`reload_icons`** (`:1108-1140`), before the comparison of `IconLoad` (so it runs on every
  path that reloads, W10), where the global is set:
  1. **The theme's icons, both modes.** For each mode, `default_icon_choice(current_icon_set,
     theme)` with the icon theme the theme names for that mode (the shown mode's
     `self.preset_icon_theme()`; the other's `other_icon_theme` where
     `other_icon_theme_explicit`), and of that choice its effective set
     (`IconSetChoice::effective_icon_set(current_icon_set)`) and its icon theme: the
     choice's own `IconSetChoice::freedesktop_theme()` where it names one, else (for
     `IconSetChoice::System`, or a freedesktop set the theme leaves to the system) the
     showcase's own detection, `self.detected_icon_theme()` (`:945-951`, which honours the
     tests' `icon_theme_detection_for_test` hook), so the handle is told an explicit icon
     theme and never detects on its own. Told as `ThemeIcons::new(set, light, dark)` where
     both modes' choices give the same set, else the shown mode's set and icon theme for
     both.
  2. **Detection failed.** Where the row's own icons depend on the showcase's detection and
     it fails, the showcase loads no freedesktop icon itself (`SC/support.rs:673-684`), so it
     tells the handle nothing for this reload and calls `switch_off`; the next reload whose
     detection gives a theme tells it and puts it back in the row's state (step 3). Two rows'
     icons depend on it: the row that follows the theme (`self.icon_choice_follows_preset`),
     where a mode's choice in step 1 is a freedesktop set that names no icon theme, since that
     row's icons are step 1's choices; and the `system` row (`IconSetChoice::System` chosen),
     where `system_icon_set()` is Freedesktop (Linux, `NT/src/detect/icon_theme.rs:23-33`),
     since step 3 fixes the detected theme. Under these two rows a failed detection means
     `switch_off`, and step 3 is not run for that reload. Under every other row, the built-in row
     (`GPUI_BUILTIN_ROW`, `SC/app.rs:620`), "Material (bundled)", "Lucide (bundled)" and a
     named freedesktop theme (`IconSetChoice::Freedesktop`; the labels are
     `IconSetChoice`'s `Display`, `NT/src/icons.rs:1355-1365`), whose icons do not depend on
     it, a detection that fails in step 1 does not switch the handle off: the reload tells
     the handle nothing of the theme's icons and step 3 runs as usual, so the widgets keep the
     row's set beside a chrome of that set (W10's rows, and no two sets in one window). No
     told icons are served while the state is fixed or off (D.3), and a told `None` would make
     the handle detect on its own, which step 1 avoids. The told icons stay those of the last
     reload that told them; choosing the following row later runs `reload_icons` again
     (`select_icon_set`, `SC/app.rs:1066-1074`), whose steps 1 and 2 tell the theme's icons
     or switch the handle off before step 3 follows. (A theme whose icon theme falls to the
     detection: one that names none or one not installed, for which `default_icon_choice`
     returns `IconSetChoice::System`, `NT/src/icons.rs:1439-1458`.)
  3. **The row's state.** The built-in row (`self.icon_set_enum.is_none()`) → `switch_off`;
     `self.icon_choice_follows_preset` → `follow_theme`; `IconSetChoice::System` →
     `fix(ThemeIcons::new(system_icon_set(), detected, detected))`, `detected` the
     showcase's detection now (failed: step 2); another choice →
     `fix(ThemeIcons::new(effective, theme, theme))`, `theme` the row's
     `IconSetChoice::freedesktop_theme()`.

  Each call that changes nothing loads nothing (D.8), so a reload that changes nothing costs
  the lookups of the key. (`apply_system_theme` has already told the handle both modes of
  the system theme, D.10; step 1 then tells it `default_icon_choice`'s answers, so the
  widgets follow the set the chrome draws.)
- **Two vectors** (decision 15): `loaded_icon_masks: Vec<Option<bool>>` and
  `gpui_icon_masks: Vec<Option<bool>>`, beside `loaded_icon_sources` (`:566-569`), filled in
  `reload_icons` after the loads: `mask_drawable` of each loaded role icon (decision 20) and
  the judgements `load_gpui_icons` returns (E.3). A judgement is read with `.get(ix)`, `None`
  where the vector is shorter (the tests empty `gpui_icons` by hand, `SC/tests.rs:4682`), and
  `None` counts as a mask, as an SVG is drawn today.
- **`chrome_icon`** (`:964-974`), the role first (W3): built-in → as today; else where
  `role_for_gpui_icon(icon)` gives a role and `loaded_icons` holds an icon for it, that icon,
  under the gallery's name of `icon`: an SVG whose `loaded_icon_masks` entry is not
  `Some(false)` → `ChromeIcon::Loaded`; an SVG judged `Some(false)` or pixels →
  `ChromeIcon::raster(name, &self.loaded_icon_sources, ix)`; else the list of names' entry,
  as `ChromeIcon::of` gives it with `gpui_icon_masks` (E.3). `sample_icon` (`:1055-1061`)
  calls `chrome_icon` and so follows.
- **`role_chrome_icon`** (`:1028-1050`): an SVG judged `Some(false)` → `ChromeIcon::raster`
  as pixels are (W6).

### E.3 `SC/support.rs`: the list of names

- **`load_gpui_icons`** (`:653-769`) keeps its parameters and the entries' order and
  returns the entries and, in the same order, their judgements
  (`(Vec<IconEntry>, Vec<Option<bool>>)`, named by an alias beside `IconEntry`,
  `pub(crate) type GpuiIconsLoad`, so its signature stays under clippy's
  `type_complexity`); its callers take the pair: `SC/app.rs:1134` and the two tests of
  E.10 (C.2's `widget_icons_listing`, which calls it too, is removed in unit 3's last commit,
  G.1, before this change).
  Under `None` (built-in) as today. Under `IconSet::SfSymbols` and `IconSet::SegoeFluent`,
  as today (the role's icon for a name with a role, none otherwise; W3: "no rule of the
  showcase goes into the connector's function"), each judged with `mask_drawable`. Under
  Lucide, Material and Freedesktop: each name's
  `native_theme_gpui::icons::load_gpui_icon(icon, set, fd_theme, None, fg_color,
  IconUse::Any)`, `fd_theme` as today (`:677-684`: the given theme, else the system's
  detected one, else none: nothing loads); the source `Bundled` or `System` by
  `is_system_set`, `NotFound` where none; the role kept in the entry for the gallery's
  label. The by-role branch (`:700-740`), the by-name closure (`:687-693`) and the
  Lucide/Material branch (`:752-763`) go, except, for SF Symbols and Segoe Fluent only,
  the role loads of today: `role_for_gpui_icon(icon)` and `SfSymbolsLoader::new(r)` or
  `SegoeFluentLoader::new(r)` with `color_opt(fg_color)` (`:701`, `:712-713`), a name with no
  role `None`; the freedesktop fallback and the other arms of that branch go. Under the
  built-in row the judgements are 106 `None`s (nothing is loaded or drawn as an image). The
  doc (`:648-652`) says the lookup is the connector's (`load_gpui_icon`), by the connector's
  table at the page's size, any icon, and by role under the two pixel sets.
- **`ChromeIcon::of`** (`:831-852`) takes the judgements too: an entry whose SVG is judged
  `Some(false)` → `Self::raster(name, sources, ix)`; the rest as today.
- **`ChromeIcon::Raster`'s doc** (`:784-790`) adds: "or an SVG in several colours, which a
  mask would show as a silhouette (native_theme_gpui::icons::mask_drawable)". `svg_only`'s
  doc (`:874-879`) adds that such an SVG is `Missing` there too.
- **`ChromeIcon`'s doc** (`:771-774`) keeps "Icon themes are never mixed".
- **The texts that speak of pixels** (`SC/info/chrome.rs`). With E.2 and this section a
  `ChromeIcon::Raster` is pixels or an SVG in several colours, and `svg_only` makes either
  `Missing`. Four texts change so that each is true of both:
  - `chrome_icon_note`, the `Raster` arm: "{set} gives it as pixels, not an SVG, so it is
    drawn" becomes "{set} gives it as pixels, or as an SVG in several colours, which a mask
    of one colour cannot show (native-theme-gpui/icons.rs, mask_drawable), so it is drawn";
  - `chrome_icon_note`, the `Missing` arm: "none: {set} holds no SVG for {name}" becomes
    "none: {set} holds no SVG of one colour for {name}";
  - `chrome_icon_note`, the `Loaded` arm: its ending ", so a coloured icon shows its shape
    only" becomes ", which carries an icon of one colour";
  - `RASTER_ICON_COLOUR`: its ending "a freedesktop raster icon in its own colours" becomes
    "a freedesktop theme's raster icon, or its SVG in several colours, in its own colours".

  The phrase "holds no SVG for" stands in other places too, each changed the same way: the
  command palette's `entry icons` note, `demo::spinner_note`, and in `SC/tests.rs` the
  helper `icon_note_start` and the tests' own phrases (seven lines in all at `a3fc81f6`, and
  "gives it as pixels, not an SVG" once). Before the unit's report the implementer reads
  every text beside an `is_raster()` or a `ChromeIcon::Raster` in `SC/` and every "pixels"
  and "no SVG" in `SC/info/`, makes each true of an SVG in several colours drawn as an
  image, and lists each in the report. The kinds are not told apart in the type: a note that
  says which of the two holds would need a reason on `Raster` and on `Missing`, which is
  not built here.

### E.4 `SC/demo.rs`, `SC/info/buttons.rs`, `SC/pages/buttons.rs`: the two loading buttons

The Buttons page has two loading Buttons since the GPUI Kit 0.10.0 port: `buttons-loading`,
which has an icon (`demo::button`), and `buttons-loading-no-icon`, which has none
(`demo::loading_button`). Both are built by `demo::button_turning`, and both are given the
sample icon for `IconName::Loader` as the icon their Spinner turns, because the Spinner's own
default was upstream's glyph. With the handle that default follows the set (W3). So where the
handle draws the set's `Loader`, neither Button is given a loading icon, and both Spinners
turn the icon every other Spinner of the window turns.

**One question, asked in one place.** `SC/demo.rs` gains:

```rust
/// What the Spinner's own default icon, gpui-component's Loader, is in this
/// window: upstream's glyph, the chosen set's (the widget icons serve it),
/// or nothing (the widget icons serve the set, and it has no Loader).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SpinnerDefault { Upstream, Set, Empty }

pub(crate) fn spinner_default(cx: &App) -> SpinnerDefault
```

With the feature it takes the handle of the `ShowcaseWidgetIcons` global (`cx.try_global`)
and asks `lookup` for `IconName::Loader`'s path, taken with `IconNamed::path`, not written
out: `IconAnswer::Draw(_)` is `Set`, `IconAnswer::Empty(_)` is `Empty`, and `NotMine`, any
later variant (the enum is `#[non_exhaustive]`) and no global are `Upstream`. Without the
feature it is `Upstream`. `lookup` answers from a table only where the set is served (D.7),
so the report is not read a second time.

`button_turning` asks it once, where `state` is `ButtonState::Loading`:

| Answer | `buttons-loading` | `buttons-loading-no-icon` |
|---|---|---|
| `Upstream` | as today: its icon, where drawn, is also its loading icon | as today: the sample icon, where drawn, is its loading icon |
| `Set` | `.icon(icon)` where its icon is drawn; no `loading_icon` | no `loading_icon` |
| `Empty` | as `Upstream` | as `Upstream` |

Under `Empty` the Spinner's default would turn nothing, so both Buttons keep the override and
turn the showcase's own sample icon (the role `StatusBusy`'s where the set has one, E.2's
`chrome_icon`). `DemoButton`, `demo::button` and `demo::loading_button` keep their fields and
signatures, and the first Button keeps its own icon, so the page still shows both of
gpui-component's cases: a Spinner in place of an icon, and a Spinner where an icon would sit.

**The info.** `info::buttons::button` gains a parameter `default_spinner: bool` after `icon`,
true for `Set` alone. Its other callers pass `false`; `grep -n 'buttons::button('` finds them
(two in `SC/info/overlays.rs`; `sized_button`, `action_button` and `toggle_button` in
`SC/demo.rs`; two in `SC/tests.rs`). Each of its two loading arms gets a twin placed before
it, since the first match wins: `(ButtonState::Loading, _) if icon && default_spinner` and
`(ButtonState::Loading, _) if default_spinner`. Each twin has the present `interaction` note
and a `spinner` note, still `not_themeable` (decision 16). In the two present arms,
"upstream's Loader" becomes "gpui-component's Loader, drawn as the application's icon
resolver answers it (the `spinner icon` line says what is turned here)", so they hold under
`Upstream` and `Empty`; in the arm with an icon, the citation `(button/button.rs,
Button::loading_icon)` moves to follow "in place of its own default", so that two
parentheticals do not stand side by side (amended 2026-11-19, U5's review). `spinner_note`
under `Set`, which `button_turning` does not reach, says the Spinner turns the widget icons'
Loader of the chosen set. For the Button with an icon:

> "in place of the button's icon (button/button_icon.rs, ButtonIcon). The Button is given no loading icon, so the Spinner turns its own default, gpui-component's Loader (gpui-component/spinner.rs, Spinner::new), which the connector's widget icons serve from the chosen icon theme (native-theme-gpui/icons/widget.rs, WidgetIcons): that theme's loading icon by the connector's table, not gpui-component's glyph. It inherits the button's text colour and turns every 0.8s, a literal (gpui-component/spinner.rs, Spinner::new)"

For the Button without one:

> "where an icon would sit: a loading Button without an icon shows a Spinner there (button/button.rs, RenderOnce for Button). This one is given no loading icon, so the Spinner turns its own default, gpui-component's Loader (gpui-component/spinner.rs, Spinner::new), which the connector's widget icons serve from the chosen icon theme (native-theme-gpui/icons/widget.rs, WidgetIcons): that theme's loading icon by the connector's table, not gpui-component's glyph. It inherits the button's text colour and turns every 0.8s, a literal (gpui-component/spinner.rs, Spinner::new)"

In `button_turning`, under `Set`: the Button with an icon gets the instance line `("icon",
icon.note("the Button shows its Spinner and its label"))` in place of its `spinner icon`
line, since the sample icon is then the Button's own icon, which the Spinner stands in for,
and not what the Spinner turns; the Button without one gets no `spinner icon` line. Under
`Upstream` and `Empty` both keep their `spinner icon` line. `spinner_note` becomes
`pub(crate)` and takes the answer as a second parameter: an icon that is shown gives today's
text; one that is not gives today's text under `Upstream` (with E.3's "no SVG of one
colour"), and under `Empty`:

> "none: {set} holds no SVG of one colour for it, so the Button is given no icon to turn, and the Spinner's own default, gpui-component's Loader, is answered by the connector's widget icons, which serve {set} and find no Loader in it: the Spinner draws nothing (native-theme-gpui/icons/widget.rs, WidgetIcons)"

`SC/pages/buttons.rs`: the comment at the first loading Button adds that where the widget
icons serve the chosen set's Loader, the Spinner's own default is already that set's and
neither loading Button is given a loading icon (`demo::button_turning`). Nothing else on the
page changes.

The tests opened with `open` or `open_with` install no handle (decision 27), so there the
answer is `Upstream` and both Buttons and their infos are as today. E.11 tests the rest.

### E.5 `SC/pages/icons.rs`: the report and the two labels (W4, W9)

In `render_gpui_icons_section` (`:219-272`), after the heading (`:270`), two captions
(`demo::caption`):

- `icons-gpui-lookup`: "This gallery: each name looked up as the page draws it, at the icon theme's 24 px or the bundled set's own, any icon the set has; a name it does not find is marked" — under SF Symbols and Segoe Fluent instead: "This gallery: SF Symbols and Segoe Fluent are known by role only, so a name shows its role's icon, and a name with no role none"; under the built-in row: "This gallery: gpui-component's own icons, which nothing looks up".
- `icons-widget-icons`, the handle's report, one of:
  - `NotInstalled`: "gpui-component's widgets: their own icons (the widget icons are not installed)"
  - `Off`: "gpui-component's widgets: their own icons (the built-in set is chosen, or no system icon theme was detected)"
  - `NotTold`: "gpui-component's widgets: their own icons (no icon set given yet)"
  - `NotServed { set, .. }`: "gpui-component's widgets: their own icons: {set} is not served to them in this release"
  - `NoAssetSource`: "gpui-component's widgets: their own icons (the widget icons have no asset source)"
  - `Loaded` served: "gpui-component's widgets: {set}{themes}, {size}in one colour; not in the set, so drawn empty: {missing}" (`{size}` "at 16 px and " for a freedesktop set, nothing for the bundled sets; `{set}` the set's `IconSet::name()`; `{themes}` for a freedesktop set " ({light}, {dark})", the two `ModeIcons::icon_theme`s, or " ({theme})" where they are the same, a mode whose `icon_theme` is `None` (its detection failed) written "no icon theme detected", and nothing for the bundled sets; `{missing}` the gallery names of a mode's `missing` paths, comma-separated, or "none": where the two modes' lists are equal (always under a bundled set, whose one table serves both modes, and under one icon theme for both), that list once; where they differ, both, light's first as in the not-served caption, "light: {light's}; dark: {dark's}", each list as above), and where `kept` is not empty "; kept as gpui-component's own: StarFill"
  - `Loaded` not served, one mode's reason (light's first): "gpui-component's widgets: their own icons: {set}{themes} gave nothing" or "… lacks {names}, which nearly every window needs"
  - any later variant (the report is `#[non_exhaustive]`, so the `match` has a wildcard arm): "gpui-component's widgets: a state of the widget icons this showcase does not describe"
  - without the feature: "gpui-component's widgets: their own icons (this build has no widget icons: feature svg-rasterize)"

Both texts come from two functions of `SC/pages/icons.rs`, `gallery_lookup_caption(set:
Option<IconSet>) -> &'static str` and `widget_icons_caption(report:
Option<&WidgetIconsReport>) -> String` (with the feature; without it the last line's text;
`None`, no `ShowcaseWidgetIcons` global, as under `open_with`, gives the `NotInstalled`
text), which the page calls and the
tests of E.11 call to read the expected text. A path is shown by its gallery name
(`GPUI_ICONS`, `SC/support.rs:529-636`). Neither
caption is a shared element (`docs/showcase-elements.toml` lists the chrome and the Basic
page only, its header, `:120`), so the element list does not change.

### E.6 `SC/info/icons.rs`: the gallery's `lookup` line

`gpui_icon` (`:157-191`) gains a parameter `by_role: bool` (true under SF Symbols and Segoe
Fluent), which `demo::gpui_icon` (`SC/demo.rs:8683`) passes. The `lookup` instance
(`:177-190`):

- the built-in arm unchanged;
- `by_role` with a role: present text of the role arm (`:182-185`), "by IconRole::{role}, the role the showcase maps the name to (showcase-gpui/support.rs, role_for_gpui_icon); a freedesktop theme without an icon for the role is asked for each name the connector gives the icon (native-theme-gpui/icons.rs, freedesktop_name_candidates) instead, in that theme only, never another (showcase-gpui/support.rs, load_gpui_icons)", becomes: "by IconRole::{role}, the role the showcase maps the name to (showcase-gpui/support.rs, role_for_gpui_icon): the connector has no table from a name to an SF Symbol or a Segoe glyph";
- `by_role` with no role: "none: SF Symbols and Segoe Fluent are known by role only, and no role maps to this name";
- otherwise, both arms' present texts (the role arm above and `:187-188`, "by name: no IconRole maps to it, so the showcase asks the set for the name the connector gives it there; a freedesktop theme for each of the icon's names in turn, in that theme only (native-theme-gpui/icons.rs, freedesktop_name_candidates; showcase-gpui/support.rs, load_gpui_icons)") become one: "by name, as the connector serves gpui-component's own icons: the set's name for it in the connector's table (native-theme-gpui/icons.rs, load_gpui_icon), in a freedesktop theme each of the icon's names in turn, in that theme only (native-theme-gpui/icons.rs, freedesktop_name_candidates), at the page's size, and any icon the set has; the chrome and the samples draw a name that has a role by the role first".

The citations name their crate's file (`every_prose_citation_still_exists`,
`CN/src/showcase.rs:3691`, reports a bare name two cited crates hold as ambiguous).

### E.7 `SC/info/mod.rs`: `own_icons` (decision 16)

The text B.3 of the 0.9.1 specification gives (`:446` at `0fa28027`), "{icons}: gpui-component's own icons whichever icon theme is chosen, so under any other they are not that theme's. Upstream builds them inside the widget and draws each by its asset path, which it first hands to the application's icon resolver (gpui-component/icon.rs, resolve_icon_path), a hook gpui-component 0.9 added (gpui-component/icon.rs, set_icon_resolver); the showcase sets no resolver yet, so each path draws upstream's icon", becomes:

> "{icons}: drawn by gpui-component inside the widget, each by its asset path, which it first hands to the application's icon resolver (gpui-component/icon.rs, resolve_icon_path). The showcase installs the connector's widget icons there (native-theme-gpui/icons/widget.rs, WidgetIcons): under an icon set of SVGs that has the nine glyphs nearly every window needs, these are the chosen set's icons, looked up by name (a freedesktop theme's at 16 px) and drawn in one colour, and one the set lacks is not drawn; under gpui-component's built-in Lucide, under a set that lacks one of the nine, and under SF Symbols or Segoe Fluent, they are gpui-component's own, and so is the filled star under Lucide. The Icons page reports which holds"

Its doc (B.3's ten-line doc above `own_icons`, in place of `0fa28027`'s eleven) becomes: "What a Widget Info says of the icons
gpui-component builds and draws inside a widget on its own, `icons` naming them and where
upstream draws them. gpui-component draws each by its asset path, which it hands to the
application's icon resolver first (gpui-component icon.rs, `set_icon_resolver`,
`resolve_icon_path`); the showcase installs the connector's widget icons there
(`ShowcaseWidgetIcons`), which serve the chosen icon theme where they can and say what
they serve on the Icons page." (No document's path: decision 23.)

**The test** `a_widgets_own_icons_are_named_gpui_components` (`SC/tests.rs:9280`): its
phrase `"gpui-component's own icons whichever icon theme is chosen"` (`:9353`) becomes
`"drawn by gpui-component inside the widget, each by its asset path"`, its failure message
"…does not say how the icons it draws of its own are drawn…". Its doc (`:9262-9278`): the
five lines B.3 of the 0.9.1 specification leaves at `:9262-9266`,

```rust
/// Every sample whose gpui-component widget draws icons of its own says
/// they are gpui-component's whichever icon theme is chosen (decision D1):
/// gpui-component 0.9 has an icon resolver hook (`set_icon_resolver`) and
/// the showcase does not set it yet, so they stay upstream's. The widgets
/// are those whose shown configuration draws one
```

become these five, the block's 76 columns kept:

```rust
/// Every sample whose gpui-component widget draws icons of its own says
/// how they are drawn (decision D1): by asset path, through the icon
/// resolver, which the showcase's widget icons answer from the chosen icon
/// theme where they can, gpui-component's own otherwise. The widgets are
/// those whose shown configuration draws one
```

The rest of the doc, from the line that begins "(gpui-component 0.9.1): a
DropdownButton's caret, …" on the tree
to `:9278`, the list of widgets, stays as it is. The test's name stays (other documents
cite it).

Without `svg-rasterize` (amended 2026-11-19, U5's review: Part E's head asks every
handle-related item to name its `#[cfg(not(…))]` arm, and this one did not), `own_icons`
keeps the same opening, up to where the text above begins to speak of the widget icons, and
then says that this build has no widget icons (feature `svg-rasterize`), so the showcase
sets no resolver and each path draws gpui-component's own icon, whichever icon theme is
chosen. The count of the essential glyphs is formatted from `ESSENTIAL_ICONS.len()`.

### E.8 `SC/info/chrome.rs`, `icon_set_select` (`:975-1005`)

| Note | Present (`0fa28027`) | New |
|---|---|---|
| `instance("raster sets", …)` (`:991`) | ends "…only the widgets that take an Icon alone -- a Toggle, a Button's icon beside its label, a command-palette entry, the Alert, a Stepper step and a Sidebar item -- show none of theirs there, each info saying so" | the same, then "; an SVG icon in several colours is drawn as an image too, and as none in those widgets, since a mask would show its outline in one colour (native-theme-gpui/icons.rs, mask_drawable). The icons gpui-component's widgets draw of their own stay gpui-component's under these two sets: the connector's widget icons do not serve a set of pixels in this release" |
| `not_themeable("upstream's icons", …)` (`:993-996`) | label "upstream's icons"; text begins "the icons a widget names as it renders, with no setter: …" | label "widgets' own icons", still `not_themeable` (decisions 16, 19); text "the icons a widget names as it renders, with no setter, which the connector's widget icons answer from the chosen icon theme where they can (below): …" followed by the present list unchanged from "this Select's and the preset Combobox's caret" to its end, with "the page row's menu Button's caret where the chosen icon theme has no ChevronDown, upstream's Caret too, which the showcase lays in as the Button's child (demo::native_tab_bar)" left as it is |
| `not_themeable("why they stay", own_icons(…))` (`:997-1000`) | label "why they stay" | label "how they are drawn", still `not_themeable`; the same `own_icons(…)` call |
| `instance("pages", …)` (`:1001-1004`) | "the samples on the other pages show the chosen icon theme's icons for the icons they are given, as the chrome does, and none where it has none; only the icons their widgets draw of their own stay gpui-component's" | "the samples on the other pages show the chosen icon theme's icons for the icons they are given, as the chrome does, and none where it has none; the icons their widgets draw of their own are drawn as the widgets' own icons above say" |

Without `svg-rasterize` (amended 2026-11-19, as E.7), the "widgets' own icons" note's
clause "which the connector's widget icons answer … (below)" says instead that this build
has no widget icons, so the widgets keep gpui-component's own. E.6's general `lookup` line
begins "by name: the set's name for it in the connector's table", not "by name, as the connector serves gpui-component's own icons",
in both builds.

A test reading these labels: `git grep -nE "upstream's icons|why they stay" -- connectors/native-theme-gpui/examples/showcase-gpui/tests.rs` finds
none today; the implementer reruns it.

### E.9 `SC/info/buttons.rs`

E.4's two new arms; the doc of `button` adds the parameter.

### E.10 The two tests that held the chrome to the list of names

`the_chrome_icons_come_from_the_chosen_set` (`SC/tests.rs:8612`) and
`the_page_samples_icons_come_from_the_chosen_set` (`:8888`) compute the expected icon from
`load_gpui_icons(Some(Material), None, None)` (`:8631`, `:8909`). Their expected value
becomes the role's icon where the name has a role and Material's role icon exists
(`MaterialLoader::new(role).load()`, as `load_all_icons` loads it), else the list of names'
entry; their doc comments say so (W3: the chrome takes the role first). The Material
chevrons now differ between the two (Appendix 1: `ChevronDown` is `expand_more` by the table
and the role `NavDown`'s arrow), which these tests then cover.

**Every test the chrome's role-first lookup turns red, and its edit.** Found by reading every
`chrome_icon`, `sample_icon`, `gpui_icons`, `loaded_icons`, `load_gpui_icons`,
`buttons-loading` and Icons-page selector in `SC/tests.rs` and every `Loader`,
`process-working` and `freedesktop_name_*` in `CN/src/` and `CN/tests/` (2026-11-14). The
tests that take an icon away, or give it pixels, do it in `gpui_icons` alone; a name with a
role would then still find its role's icon in `loaded_icons`:

| Test (`SC/tests.rs`) | Names it touches with a role | Edit |
|---|---|---|
| `the_chrome_icons_come_from_the_chosen_set` (`:8612`) | `CircleX`, `Settings`, the page icons with roles | the expectation above |
| `the_page_samples_icons_come_from_the_chosen_set` (`:8888`) | most of `PAGE_SAMPLE_ICONS` (`:8796`) | the expectation above; the two loading buttons' notes do not change (`open` installs no handle, so E.4's answer is `Upstream`), and the test's `Loader` case stays, with E.3's phrase |
| `remove_icons` (`:8863`), the helper of `a_page_sample_icon_the_set_lacks_is_absent` (`:8944`), `own_chevrons_come_from_the_chosen_set` (`:9030`), `the_dialog_samples_icons_follow_the_chosen_set` (`:9104`) | `ChevronUp`, `ChevronDown`, `ChevronRight`, `CircleX`, `TriangleAlert`, `Search`, `Settings`, `Copy`, `CircleCheck`, `Delete`, `Loader` | it also empties, in `loaded_icons`, the entry of `role_for_gpui_icon(name)` for each name it is given |
| `rasterize_icons` (`:18055`), with names at `:9159`, `:18237`, `:18390`, `:18448`, `:18516` | `CircleX`, `TriangleAlert`, `ChevronUp`, `ChevronDown`, `ChevronRight`, `Settings`, `Search`, `Copy`, `CircleCheck`, `Delete` | for each name it is given, it also gives the role's entry in `loaded_icons` the same pixels (it already does so for the roles it is given, `:18069`) |
| `the_theme_error_alerts_icon_follows_the_chosen_set` (`:8740`) | `CircleX` (its loop at `:8772-8777`) | the loop also empties `loaded_icons`' `DialogError` entry (`role_for_gpui_icon(&IconName::CircleX)`) |
| `the_page_menu_button_shows_its_caret_without_the_sets_chevron` (`:18752`), a fourth user of `remove_icons` (`:18756`) | `ChevronDown` | none of its own: `remove_icons`' edit above covers it |

The names these helpers are given without a role (`Inbox`, `Star`, `Undo`, `Folder`, `PanelLeft`; `CircleX`'s role is `DialogError`, `Loader`'s `StatusBusy`, `Delete`'s `ActionDelete`, by `icon_name`, `CN/src/icons.rs:141-196`) keep today's lookup. Not touched by the change (their names have no role, `role_for_gpui_icon` is `None`):
`a_panel_toggle_the_set_has_no_icon_for_is_labelled` (`:5231`, `PanelLeft`),
`a_chrome_icon_the_set_lacks_is_absent` (`:8675`, `SquareTerminal`),
`a_sidebar_item_losing_and_regaining_its_icon_is_drawn` (`:4672`, `LayoutDashboard`, which
clears `gpui_icons` whole: the `.get` of E.2 keeps the masks' shorter length harmless).
`a_widgets_own_icons_are_named_gpui_components` changes with E.7. In the connector, no test
but those of B.1 reads the two changed table entries
(`git grep -nwE 'Loader|LoaderCircle|process-working' -- connectors/native-theme-gpui/src connectors/native-theme-gpui/tests`,
read on `0fa28027`: in `CN/src/icons.rs`, the role mapping `StatusBusy => Loader` (`:183`), the
Lucide and Material tables' two entries (`:268-269`, `:412-413`), the two changed freedesktop
entries (`:640-641`), the list of names (`:1275-1276`), a test expecting `Some(IconName::Loader)`
for the role (`:1704`), two doc comments naming Breeze's `process-working` stylesheet (`:977`,
`:2194`) and the fixture rows (`:2941-2942`); in `CN/src/widgets/spinner.rs`, the doc that names
the spinner's own files (`:366`, `:371`), not the table; none in `CN/tests`). The implementer reruns the showcase suite after
each change of E.2 and E.3 and lists any other red test with its edit in the plan's record.
E.3's phrases reach these tests too: `icon_note_start`'s `Missing` arm and each literal
"holds no SVG for".

### E.11 New showcase tests (`SC/tests.rs`, `#[gpui::test]`, with the feature)

Each is `#[cfg(feature = "svg-rasterize")]` and, where it chooses a bundled set,
`#[cfg(feature = "lucide-icons")]` or `material-icons` as the set it chooses; a caption's
expected text comes from E.5's two functions. The five that need the handle open their
window with `open_with_widget_icons` (E.2) and read the handle from the
`ShowcaseWidgetIcons` global.

**Independent of the machine.** `Showcase::new` installs the system's theme, so at start
the handle follows whatever the machine has (a freedesktop theme on Linux; on the macOS and
Windows runners SF Symbols or Segoe Fluent, `NotServed { fixed: false, .. }`).
No assertion reads that start: each test first installs a bundled-set preset with the
existing helper `use_preset` (`catppuccin-mocha` for Lucide, `material-3` for Material, both
naming their set, `catppuccin-mocha.toml:4`, `material-3.toml:9`), or, in the test of a
fixed row, fixes a bundled set and sets the theme's icon theme and the detection by hand,
and asserts only after it, so its assertions hold whatever the machine has installed. "Following" is read from the
report as `WidgetIconsReport::Loaded(set)` with `set.fixed == false` (a pattern
`Loaded(LoadedSet { fixed: false, .. })`), "fixed" as `fixed == true`.

- `the_icon_theme_select_puts_the_widget_icons_in_its_state`: after `use_preset("catppuccin-mocha")`,
  the default row → `Loaded` Lucide, not fixed; the built-in row → `Off`; "Material
  (bundled)" → `Loaded` Material, fixed; a theme install (`use_preset("catppuccin-latte")`)
  while Material is fixed keeps `Loaded` Material, fixed; the default row again → `Loaded`
  Lucide, not fixed.
- `the_widget_icons_follow_the_set_the_chrome_draws`: under the default row, after
  `use_preset` of `catppuccin-mocha` and then of `material-3`, the report's set equals
  `self.icon_set_enum` each time.
- `a_fixed_row_keeps_the_widget_icons_where_the_detection_fails` (E.2 step 2; also
  `#[cfg(target_os = "linux")]`, where the system's set is Freedesktop, as
  `a_failed_icon_theme_detection_is_shown_as_a_failure` is, `SC/tests.rs:11523`, and
  `material-icons`): after `use_preset("adwaita")`, "Material (bundled)" chosen
  (`select_icon_set(&IconSetChoice::Material.to_string(), …)`) → `Loaded` Material, fixed.
  Then, in one update: `icon_theme_detection_for_test` set to a failing detection (the
  `PlatformUnsupported` error of `:11529-11533`); `current_icon_theme` and
  `other_icon_theme` set to `Some("no-such-icon-theme")` and `other_icon_theme_explicit` to
  `true` (as `:12578-12582` sets the first), so `default_icon_choice` gives
  `IconSetChoice::System` in both modes and step 1 falls to the failing detection; and
  `reload_icons` called. Expected: the report is still `Loaded` Material, fixed, with
  `served()`, not `Off` (a reload that switched the handle off here would leave the widgets
  drawing gpui-component's Lucide beside a Material chrome). Then the `system` row
  (`select_icon_set(&IconSetChoice::System.to_string(), …)`, as `:11534`) under the same
  failing detection → `Off` (step 2 holds for that row); "Material (bundled)" again →
  `Loaded` Material, fixed. Then the following row, taken as `--icon-set default` takes it
  (`icon_choice_follows_preset` set, then `show_icon_choice` and `reload_icons`,
  `SC/main.rs:1044-1050`), under the same failing detection → `Off` (step 2 holds for it
  too: under the unknown icon theme its choice is `IconSetChoice::System`); then a working
  detection (`|| Ok("breeze".to_string())`, as `SC/tests.rs:11625` gives one) and
  `reload_icons` → `Loaded`, not fixed (`fixed == false`; step 2's "puts it back in the
  row's state", whatever the machine has installed: a theme that gives nothing is `Loaded`
  too, not served).
- `a_name_with_a_role_takes_the_roles_icon_and_the_gallery_the_tables`: under Lucide,
  `chrome_icon(&IconName::Delete)` is the role `ActionDelete`'s SVG and the gallery's entry
  for `Delete` is Lucide's `delete` (Appendix 1); under Material, `ChevronDown` likewise.
- `an_svg_in_several_colours_is_an_image_in_the_chrome`: a `ChromeIcon::of` over an entry
  judged `Some(false)` with a source gives `Raster`; `svg_only` gives `Missing`.
- `a_role_icon_in_several_colours_is_an_image`: with Lucide chosen, `loaded_icons`'
  `DialogError` entry replaced by a two-colour SVG (two rectangles of kde-breeze's light
  text and accent colours, read from the preset) and `loaded_icon_masks` judged again by
  `mask_drawable`, `role_chrome_icon(IconRole::DialogError)` is `Raster` and
  `chrome_icon(&IconName::CircleX)` (its role path) is `Raster`, each `Missing` through
  `svg_only`.
- `the_loading_buttons_keep_the_spinners_icon_where_the_set_is_served`
  (`lucide-icons`): under Lucide (after `use_preset("catppuccin-mocha")`, the default row;
  the handle serves Lucide, whose `Loader` draws) `buttons-loading`'s info carries E.4's
  `spinner` note for a Button with an icon and an `icon` line, and no `spinner icon` line;
  `buttons-loading-no-icon`'s carries E.4's note for a Button without one, and no `spinner
  icon` line. Under the built-in row both carry the present notes and their `spinner icon`
  lines.
- `a_spinner_with_nothing_to_turn_says_so` (a plain `#[test]`): `spinner_note` of a
  `SampleIcon` whose `drawn` is `ChromeIcon::Missing`, with `SpinnerDefault::Empty`, begins
  "none:" and says the Spinner draws nothing; with `SpinnerDefault::Upstream` it begins
  "gpui-component's own Loader".
- `the_icons_page_reports_the_widget_icons`: under Lucide the `icons-widget-icons` caption
  reads the served text with `none` missing and `StarFill` kept; under the built-in row the
  `Off` text.

### E.12 `docs/showcase-exceptions.toml` (W10)

`SidebarToggleButton` and `SpeechButton` (`:52-53`) stay. In each reason, as Part E of the
0.9.1 work left it (its citations re-read at 0.9.1), the opening "it would mix icon themes:"
becomes "under SF Symbols and Segoe Fluent, which the connector's widget icons do not serve
yet, it would mix icon themes:"; in `SidebarToggleButton`'s, "`PanelLeftClose` or
`PanelLeftOpen`" becomes "`PanelLeftClose`, `PanelLeftOpen`, `PanelRightClose` or
`PanelRightOpen`" (`GC/sidebar/mod.rs:360-368`); in both, "so under any other chosen icon
theme it would show another icon theme's icon" becomes "so under those two it would show
gpui-component's Lucide beside the set's icons, as it would under a set the widget icons
decline for lacking a glyph every window needs; under an SVG icon set the widget icons
serve, it would show that set's. Shown once the widget icons serve those two sets
(`docs/todo.md`, *gpui: the widgets' own icons under SF Symbols and Segoe Fluent*)". The
header's kinds (`:12-13`) keep "a widget that would mix icon sets". `scripts/check_widget_coverage.py`
reads a reason only for being non-empty (`:763-766`).

### E.13 What does not change

`docs/showcase-elements.toml` (no shared element added or moved, so the route tests'
`every_listed_leaf_has_a_route` asks for no new `HOW` entry); the parity comparator and its
exceptions (read in the sources: the one sample on a gpui-component glyph,
`basic.drop_down.trigger.arrow`, is one gpui does not dump, `docs/showcase-exceptions.toml:927-944`,
and the connector's check box mark changes only under catppuccin-mocha, where the exception at
`:756` records that the sample reads the fill); the egui and iced showcases (rationale §5);
`check_widget_coverage.py`'s widgets (no widget added).

### E.14 The KDE steps 2 to 5, already on the tree, and what of them this part touches

They are on `v0.7.2-rc1` (built 2026-11-18); nothing is merged, and nothing of them waits.

- **Edit** (steps 4 and 5, B.10): the Basic page's fourth tool button, `basic-icon-edit`,
  is drawn by `demo::icon_button` from `role_chrome_icon(IconRole::ActionEdit)`, with the
  elements `basic.icons.edit` and `basic.icons.edit.icon`. E.2's change of
  `role_chrome_icon` (an SVG judged `Some(false)` is drawn as an image) applies to it as to
  Copy, Paste and Delete, and `loaded_icon_masks` holds `ActionEdit`'s judgement as every
  role's. `demo::icon_button` takes any element, so Edit is no slot that takes an `Icon`
  alone and C.4's 23 names do not change; `ActionEdit` maps to no `IconName`, so the widget
  icons never serve it. This part adds no element (E.13). Done-criteria: U3a's constant
  holds the 23 names; in U5, Edit is drawn through `role_chrome_icon` with E.2's judgement.
- **The toolbar buttons** (steps 4 and 5, B.9): `demo::icon_button`, `demo::toolbar_button`
  and `toolbar_item` are built on `variants::toolbar_button`, and a checked tool button
  letters its icon in the checked text colour where the icon follows the text colour. With
  E.2 an SVG in several colours on those buttons is drawn as an image in its own colours,
  as B.9 says of a raster; a one-colour SVG stays a mask and takes the colour. Nothing of
  B.9 changes this part's edits.
- **`demo::button`** (steps 2 and 3, 3F.4; then the GPUI Kit 0.10.0 port): its body is
  `button_turning` now, which E.4 is written for.

---

## Part F — Documentation

### F.1 `CN/README.md`

1. **What it does**, the *Icons* bullet (`:49-56`): after its first sentence, "With
   `svg-rasterize` (on by default), a handle serves gpui-component's widgets the theme's
   icons in place of gpui-component's Lucide glyphs; see
   [The widgets' own icons](#the-widgets-own-icons)."
2. **Quick start** (`:98-136`): the block becomes the following, and a sentence before it,
   "The handle `WidgetIcons` wraps the application's asset source and serves
   gpui-component's widgets the theme's icons; `apply_system_theme` tells it the theme's
   icons, and on the `apply` path the application tells it, from the `Resolved` that
   resolving a preset gives (`from_preset` returns the `ResolvedTheme` alone)." The block
   (compiled by the README doctest gate, D.11):

   ```rust,no_run
   use gpui_kit::{AppContext as _, WindowOptions};
   use native_theme_gpui::icons::WidgetIcons;
   use native_theme_gpui::{AccessibilityPreferences, ColorMode, Theme, apply, apply_system_theme, to_theme};
   # use gpui_kit::{Context, IntoElement, Render, Window, div};
   # struct MyView;
   # impl Render for MyView {
   #     fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
   #         div()
   #     }
   # }

   let icons = WidgetIcons::new();
   gpui_kit::application()
       .with_assets(icons.assets(gpui_kit::assets::Assets))
       .run(move |cx| {
           gpui_kit::init(cx);
           icons.install(cx);
           match native_theme_gpui::SystemTheme::from_system() {
               Ok(sys) => apply_system_theme(&sys, cx),
               Err(_) => {
                   let prefs = AccessibilityPreferences::from_system();
                   if let Ok(nt) = Theme::preset("adwaita")
                       && let Ok(resolved) = nt.resolve(ColorMode::Light)
                   {
                       let theme = to_theme(&resolved.variant, &nt.name, resolved.mode.is_dark(), &prefs);
                       apply(theme, &resolved.variant, &prefs, cx);
                       icons.set_theme_icons(&resolved, cx);
                   }
               }
           }
           let opened = gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| cx.new(|_| MyView));
           if let Err(error) = opened {
               eprintln!("could not open the window: {error}");
           }
       });
   ```

   Not compiled here: the implementer confirms it under the gate. It is the code of the
   handle's own doc example (D.3), which U4's doctest run compiles: U6 copies the example as U4
   committed it, and where U4 had to change it, the README takes U4's. It is the crate doc's
   manual path (`CN/src/lib.rs:58-67`: `Theme::preset`, `resolve`, `to_theme` with
   `&nt.name`, the name `from_preset` passes, `:464-469`), with a let chain (edition 2024,
   `Cargo.toml:18`; the crate's MSRV is 1.95.0).
3. **Light and dark under a preset** (`:189-221`), after its last paragraph: "An application
   that serves the widgets' own icons tells its handle each variant's icons, from the
   `Resolved` that `Theme::resolve` returns, as the quick start does."
4. **A new section `## The widgets' own icons`** after *Theme-drawn widgets* (ending
   `:519`), before *How re-application works* (`:521`): (a) what the handle does and that it
   needs `svg-rasterize`; (b) the three steps (create, wrap the asset source, install after
   `gpui_kit::init` and before the first theme) and that installing replaces a resolver set
   before; (c) the two paths (`apply_system_theme` tells it; on `apply`,
   `set_theme_icons(&resolved)`, once per variant applied), and the rule for choosing what to
   tell: the set the application draws its own icons from (`default_icon_choice` falls back
   to the system's where the theme's is not installed); (d) the three states and what an
   application calls for a set its user chose; (e) the paths it reserves,
   `native-theme-gpui/…`, and that an application's own artwork under one of the 106
   `icons/*.svg` paths gets the set's icon once a set is served; (f) its own resolver over
   the handle's: set after `install`, calling `lookup` for the three answers; (g) the report,
   and that an application checks it for the sets it supports; (h) **Limits**, as a
   `### What the widget icons leave as they are` subsection: the two stated exceptions to
   "nothing is substituted", the floor (a set lacking one of `ESSENTIAL_ICONS`, or a set
   that gives nothing, keeps gpui-component's glyphs for every widget) and SF Symbols and
   Segoe Fluent (not served in this release); `StarFill` under Lucide; an `IconName` a later
   gpui-component adds, which the connector's tables do not know: "not mine", so it
   keeps upstream's glyph among the served set's until the connector's tables gain it (the
   tables' wildcard arms, those of `lucide_name_for_gpui_icon`, `material_name_for_gpui_icon`
   and `freedesktop_name_candidates` in `CN/src/icons.rs`); an icon the set lacks
   drawn empty, a button whose only content it is drawn empty (a password field's eye, a
   carousel's arrows, a sidebar's toggle), an alert's or a notification's status icon gone
   where the theme's is in several colours; only an icon of one colour served, at 16 px in a
   freedesktop theme; what stays outside the hook (native menus on macOS and Windows, a
   `gpui_kit_assets::IconName` rendered by itself, the Markdown task box's mark, an `Icon`
   built from bytes); the connector's `widgets::Checkbox`, whose mark is its own stroked
   check, or dash in the mixed state, where the theme states
   `checkbox.check_mark_stroke_width`, and the set's `Check` or `Minus` otherwise, beside gpui-component's check box and a menu's or list's check mark, which
   show the set's; the connector's `widgets::Spinner`, which turns the theme's animated
   indicator or arc, beside gpui-component's `Spinner`, which turns the set's `Loader` (the
   three-dot loading icon under Adwaita, which has no spinner icon); and staleness: the
   handle keeps serving the drawings it loaded until the set changes (it keeps every table
   of the current set, so a light/dark switch loads each icon theme once), so a theme whose
   files change while the application runs is loaded again only after another set has been
   chosen and the theme again, and the new drawing then reaches only sizes a window has not
   drawn yet and windows opened afterwards: a drawing a window already holds stays for the
   run, since GPUI keeps one drawing per path and size and the theme's paths are the same
   (W5). The section states each limit in plain words. It names no decision of
   the rationale and links no document under `docs/`: the 0.7.1 documentation audit
   (`8a22644d`) took plan history and private names out of the public READMEs. The "(W5)"
   above is this specification's pointer for the writer, not README text.
5. **Features** (`:715-747`): the `svg-rasterize` row adds ", and the widget icons
   (`icons::WidgetIcons`)".
6. **Role icons** (`:886-920`): the sentence "`icons::icon_name` is another thing: the
   `IconName` of gpui-component's bundled Lucide glyph, whatever the theme's set."
   (`:893-895`) becomes "`icons::icon_name` is another thing: the `IconName` gpui-component
   draws for the role, its Lucide glyph, or the theme's icon for that name where the widget
   icons serve the theme's set (see [The widgets' own icons](#the-widgets-own-icons))."
7. **Limits** (`:577`): after its first paragraph ("Two things a theme cannot reach in
   gpui-component 0.9.1; each is a request to upstream."), one sentence: "What the
   connector's widget icons leave as they are is in
   [The widgets' own icons](#what-the-widget-icons-leave-as-they-are)."
8. **What gets mapped**, the *Icons* bullet (`:1077-1084`): its last sentence, which ends
   "a role's icon in the theme's own set is `SystemTheme::icon` (see
   [Role icons](#role-icons)).", gains a clause before its full stop: "; and the widget icons (`icons::WidgetIcons`,
   feature `svg-rasterize`) serve gpui-component's widgets their own icons from these tables
   (see [The widgets' own icons](#the-widgets-own-icons))", the bullet rewrapped to its 78
   columns.

### F.2 Crate documentation

D.11 and the handle's doc (D.3); `load_gpui_icon`'s and `mask_drawable`'s (B.2, B.3); A.2's
method doc in the core.

### F.3 `CHANGELOG.md`, `## [0.7.2]`

No new section: the *Added* bullets go at the end of `## [0.7.2]`'s `### Added` and the
*Changed* bullets at the end of its `### Changed`, in the order given, one physical line each.

***Added***:

> - **native-theme-gpui**: `icons::WidgetIcons` (feature `svg-rasterize`, on by default) serves gpui-component's widgets the theme's icon set: the caret of a `Select`, a dialog's close button, the window controls, a menu's check mark and every other icon gpui-component draws by its `IconName`, an application's own `Icon::new(IconName::…)` too. It wraps the application's asset source, installs gpui-component's icon resolver (`set_icon_resolver`), follows the theme (`apply_system_theme` tells it; on the `apply` path the application does) and its light and dark modes, or a set the application fixes, and reports what it serves. It serves Lucide, Material and freedesktop themes by the connector's tables, a freedesktop icon at 16 px and only an icon of one colour, and draws an icon the set lacks empty; a set that lacks one of nine glyphs nearly every window needs (`icons::ESSENTIAL_ICONS`, replaceable), a set that gives nothing, and SF Symbols and Segoe Fluent are not served, and the widgets keep gpui-component's glyphs, as they keep its filled star under Lucide.
> - **native-theme-gpui**: `icons::load_gpui_icon`, the lookup the handle serves from, for an application's own icons by gpui-component's `IconName` at its size and colour, and `icons::mask_drawable`, whether gpui's one-colour mask carries an icon.
> - **native-theme**: `FreedesktopLoader::load_where`, which takes the caller's test and returns the first icon that passes it, going on in the loader's own order (by names: theme by theme, each name symbolic first; by a name or a role: the symbolic form through the theme and those it inherits from, then the plain name).
> - **Showcases (gpui)**: gpui-component's widgets draw the chosen icon theme's icons; the Icons page reports what the widget icons serve and labels its own gallery's lookup apart from theirs.

***Changed***:

> - **native-theme-gpui**: `freedesktop_name_candidates` gives `Loader` and `LoaderCircle` `process-working-symbolic`, `content-loading` (it gave `process-working`, `content-loading`), so Breeze's strip of animation frames under the plain name is never found; the deprecated `freedesktop_name_for_gpui_icon` answers `process-working-symbolic` for them outside GTK desktops, where a theme with only the plain strip now gives no icon.
> - **native-theme-gpui**: `apply_system_theme` tells an installed `icons::WidgetIcons` the theme's icon set and the icon theme of each mode.
> - **Showcases (gpui)**: the gallery of gpui-component's `IconName`s looks each name up by the connector's table, as the widgets' icons are served (under Lucide `Delete` is gpui-component's backspace glyph, under Material the chevrons are chevrons); the chrome and the samples keep drawing a name that has a role by the role first. An SVG icon in several colours is drawn as an image where the widget takes any element and as none where it takes an `Icon` alone, as pixels are. The Buttons page's two loading buttons no longer give their spinner an icon where the widget icons serve the set and its `Loader`.

### F.4 `ROADMAP.md`

No new section. One paragraph is appended to the section `## v0.7.2 — …`, after its last:

```markdown
gpui-component's widgets draw icons of their own, the caret of a select, a dialog's
close button, the window controls, a menu's check mark, about fifty in all, which were
always its bundled Lucide glyphs. gpui-component 0.9 added a hook for them, and the gpui
connector's `icons::WidgetIcons` uses it: the widgets draw the theme's icon set, Lucide,
Material or a freedesktop theme such as Breeze or Adwaita, in the theme's light and dark
modes, and an icon the set lacks is not drawn. Where a set lacks a glyph nearly every
window needs, and under SF Symbols and Segoe Fluent, whose icons are pixels, the widgets
keep gpui-component's own. Serving those two sets is the next step.
```

The section's heading gains the icons. Proposed, and the maintainer's to word: `## v0.7.2 —
egui 0.37, GPUI Kit 0.10.0, gpui-component's own icons in the theme's icon set, and the
focus ring, the hover outlines and the toolbar buttons from issue #4`. Before changing it,
U6 greps the tracked files for a link to the present heading's anchor (none at `a3fc81f6`).

The paragraph *Also in the 0.7.x releases* says "tags N open items `[v0.7.x]`". The number
written is what `grep -c '^- \[ \] \[v0.7.x\]' docs/todo.md` gives on U6's tree once F.5's
edits are made (74 at `a3fc81f6`; F.5 opens one such entry, item 1, and closes one, item 8).

### F.5 `docs/todo.md`

Conventions as its header (`:1-21`): a dated line `*<what> (0.7.2, <date>, <sha>):*`; an
entry cited by heading and opening words.

**New entries** (each "the backlog gets an entry" and "recorded" of the rationale):

1. W7 (`:893-897`) and §5: a new section in *Toolkit Connectors*, in the place item 8
   empties: after *gpui: the editor's status colours are partly upstream's scheme* and
   before *gpui: the theme-drawn slider beside upstream's keys and `SetValue`* (four
   sections of 0.7.2 now follow the entry item 8 moves out, so the place is named by both
   neighbours):

   ```markdown
   ### gpui: the widgets' own icons under SF Symbols and Segoe Fluent

   - [ ] [v0.7.x] The connector's `icons::WidgetIcons` serves gpui-component's widgets
         Lucide, Material and freedesktop icons, and nothing under SF Symbols and Segoe
         Fluent: the connector knows those sets' icons by role only (29 of the 106
         `IconName`s at most, `Close` not among them) and they load as pixels, where the
         hook names an SVG path (`docs/todo_v0.7.1_gpui-widget-icons-rationale.md` W7).
         So under them the widgets draw gpui-component's Lucide beside the application's
         native icons. To do: a table from each of the 106 names to an SF Symbol and to a
         Segoe glyph, each entry checked on the platform (the macOS and Windows runners),
         and the pixels carried in an SVG that embeds them: both cores build resvg 0.46.0
         with `raster-images` (gpui-pre 0.3.12 `Cargo.toml:456-462`, gpui-fast 0.1.16
         `:276-282`), whose default resolver takes a `data:` PNG, JPEG, GIF or WebP (usvg 0.46.0
         `src/parser/image.rs:59-74`, resvg 0.46.0 `src/image.rs:62-77`; PNG keeps the alpha
         without loss); a mask needs only the alpha; GPUI rasterises an SVG at twice the
         device pixels it is drawn at (gpui-pre 0.3.12 `src/window.rs:5247-5251`,
         `src/svg_renderer.rs:81`); the widgets draw at 12 to 16 px, at 24, and an empty
         list's inbox at 48 and a select's at 28 (gpui-component 0.10.0
         `src/list/delegate.rs:83`, `src/table/delegate.rs:195`, `src/select.rs:289`). The
         embedding was read, not run. `docs/showcase-exceptions.toml`'s
         `SidebarToggleButton` and `SpeechButton` are shown with this step. *Segoe Fluent
         icons as SVG outlines* above is another way for Segoe.
   ```
2. W4 (`:773-775`) and §5, after 1:

   ```markdown
   ### gpui: Lucide's and Material's window maximize and restore are full-screen glyphs

   - [ ] [1.0] Under Lucide and Material the widget icons draw a window's maximize and
         restore controls as the sets' full-screen glyphs, Lucide's `maximize` and
         `minimize-2`, Material's `open_in_full` and `close_fullscreen`
         (`connectors/native-theme-gpui/src/icons.rs`, `lucide_name_for_gpui_icon` and
         `material_name_for_gpui_icon`, `WindowMaximize` and `WindowRestore`), since
         neither set has window glyphs, and the roles give the same
         (`docs/todo_v0.7.1_gpui-widget-icons-rationale.md` W4). Lucide has a plain square
         and a pair of squares that would read as maximize and restore; Material was not
         read for them. A change of the two tables' entries, with the measurement's
         rows for the four controls (`docs/evidence/widget-icons/<date>/listing.md`) and the
         sets' drawings, which are the repository's own (`native-theme/icons/`), beside it.
   ```
3. W3 (`:588-592`) and §5: in *Core API*, after *Icons: recolour a Breeze
   `current-color-scheme` stylesheet from the theme, as KIconLoader does* (`:470-510`):

   ```markdown
   ### Icons: no role for a caret

   - [ ] [1.0] A select's caret is a glyph of its own on every platform, and the model has
         no `IconRole` for it. The gpui showcase's selects draw gpui-component's caret,
         which the widget icons serve by the connector's table (`ChevronDown`: Material's
         `expand_more`), while the iced and egui showcases draw theirs by the nearest role,
         `NavDown` (Material's arrow), so the three differ under Material
         (`docs/todo_v0.7.1_gpui-widget-icons-rationale.md` W3). A role for a disclosure
         caret, read from each platform, would let all three draw one icon.
   ```
4. W3 (`:594-597`), after 3:

   ```markdown
   ### Icons: StatusCheck's and StatusBusy's freedesktop names

   - [ ] [1.0] The roles' freedesktop names, `emblem-default` for `StatusCheck` and
         `content-loading-symbolic` for `StatusBusy` (`native-theme/src/model/icons.rs`,
         `icon_name`), are what every user of a role loads, in iced and egui too. The gpui
         connector's table, which the widget icons serve, gives `Check` `dialog-ok`,
         `object-select` and `Loader` `process-working-symbolic`, `content-loading`
         (`docs/todo_v0.7.1_gpui-widget-icons-rationale.md` W3): `object-select-symbolic`
         is recorded as the chosen row's check mark on both desktops, `emblem-default` as
         nothing. Review the two with the measurement's rows for each name
         (`docs/evidence/widget-icons/<date>/listing.md`, item 4), drawing the icons again
         from the installed themes where they are to be seen: no drawing of an installed
         theme is kept in the repository.
   ```
5. W8 (`:926-927`): after the last line of *Checkbox: the check mark's glyph and size*
   (`:186-212`):

   ```markdown
         *Widget icons (0.7.2, <date>, <sha>):* the gpui connector's `widgets::Checkbox`
         draws its own mark, the outline of gpui-component's Lucide check, or of its dash
         in the mixed state, stroked at `checkbox.check_mark_stroke_width`, where the theme
         states it (kde-breeze, adwaita, material-3), and the set's `Check` or `Minus`
         through the widget icons elsewhere; under those
         three its mark is not the icon set's, while a menu's and a list's check mark are
         (`docs/todo_v0.7.1_gpui-widget-icons-rationale.md` W8). A mark's shape in the model
         would give the connector's mark one.
   ```

**Entries this work changes** (grep `docs/todo.md` for `icon resolver`, `icons follow`,
`set_icon_resolver`, `own_icons`, `widgets build`, today and again at implementation):

6. *Upstream PR to gpui*, "PR: let an application drop the SVGs" (`:3812` today, rewritten by
   the 0.9.1 specification's D.4): the sentence that begins "Until then the gpui showcase's
   page samples and chrome draw the chosen theme's icons where the showcase builds the icon"
   (`:3843-3846` today, without the clause D.4 removes) is removed, and after the entry's
   last line: "*Widget icons (0.7.2, <date>, <sha>):* the gpui connector's
   `icons::WidgetIcons` answers each icon set with paths of its own, so the widgets' own
   icons follow a switch without this. It would not fix the staleness the handle has: the
   handle keeps serving the drawings it loaded until the set changes, so a theme whose files
   change while the application runs is loaded again only after another set was chosen and
   the theme again, and the new drawing then reaches only sizes a window has not drawn yet
   and windows opened afterwards; a drawing a window already holds stays for the run, since
   the theme's paths are the same (`docs/todo_v0.7.1_gpui-widget-icons-rationale.md` W5).
   Dropping a window's drawn SVGs would let such a window redraw from the handle's reloaded
   bytes after that second choice, and before it would redraw the same old bytes. What it
   would give an application besides: a path it serves itself, redrawn from what its asset
   source answers now."
7. *Upstream PRs to gpui-component*, "PR: an `Icon` that draws an image, or an icon slot"
   (`:4063` today), after its last line: "*Widget icons (0.7.2, <date>, <sha>):* still so; the
   widget icons carry an SVG of one colour only, so under SF Symbols and Segoe Fluent they
   serve nothing, and the gpui showcase's slots that take an `Icon` alone also show nothing
   for an SVG in several colours (`docs/todo_v0.7.1_gpui-widget-icons-rationale.md` W6, W7)."

**The entry this work closes.**

8. *gpui: the widgets' own icons follow the theme's icon set* (`[v0.7.x]`), the section
   the 0.9.1 specification's D.3 wrote in v0.7.1 in `## Toolkit Connectors`
   ("gpui-component's widgets draw their own Lucide glyphs … until the connector or the gpui
   showcase sets the application's icon resolver"). This work does what it asks: the
   connector's handle sets the resolver and the showcase installs it. What it leaves, SF
   Symbols and Segoe Fluent (W7), is entry 1 above. Since 2026-11-18 the entry carries a
   dated line, *GPUI Kit 0.10.0 (2026-11-18):*, which names three more of the widgets' own
   icons (the Spinner's `Loader` on a loading `Button`, the mixed check box's `Minus`,
   `ChevronLeft` in a right-to-left layout); the move keeps it. The section's one item is done, so the
   whole section moves to `docs/archive/todo_closed.md`, to the end of its `## Toolkit
   Connectors` group (`:963`, the group that runs to `## Publishing Prep`, `:1942`, today),
   verbatim with its box ticked, as that file's header asks (`:1-14`), ending with the dated
   line, at the item's indentation, in the form the file's closing lines take (`:877`,
   ``*Closed 2026-10-27 by `b933efe0` (the v0.7.0 release asset run):*``; `:904` the same
   without a commit): "*Closed <date> by `<sha of unit 4>` (the
   connector's `icons::WidgetIcons`) and `<sha of unit 5>` (the gpui showcase installs it):*
   gpui-component's widgets draw the theme's icon set under Lucide, Material and a
   freedesktop theme; under SF Symbols and Segoe Fluent they keep gpui-component's glyphs,
   the open entry *gpui: the widgets' own icons under SF Symbols and Segoe Fluent*; the
   three icons its line of 2026-11-18 names are names of the connector's tables and are
   served with the rest." The
   move is unit 6's (G.1), with the other backlog edits, after units 4 and 5 have landed and
   their commits are known.
9. `docs/issues/0004_kde-open-items-rationale.md`, *The releases after v0.7.1*, after the
   paragraph that follows its list ("The other order for 2 and 3, …"): "*2026-11-19:* the
   first half of item 3, gpui-component's own icons, was built inside 0.7.2 by the
   maintainer's decision; the step that serves the two pixel sets stays a later release."

Nothing else closes whole (the request for the hook closed in v0.7.1, the 0.9.1
specification's D.3). `python3 scripts/check_todo_citations.py` checks the preset
citations; these entries cite none.

### F.6 `docs/archive/`

In 0.7.2's close-out (G.4) the rationale, this specification and the plan move to
`docs/archive/` with 0.7.2's other documents and are named in 0.7.2's one row of
`docs/archive/README.md`; they get no row of their own. In the same commit every path mention of the three documents in
every tracked file outside `docs/archive/` is rewritten for the new place (rule 22 (a);
v0.7.1's convention: a document whose header dates its lines keeps its line citations,
never a stale path), found after the move with
`git grep -nE 'todo_v0\.7\.(1_gpui-widget-icons-rationale|2_gpui-widget-icons-(spec|plan)|2_gpui-widget-icons-\{spec,plan\})' -- ':!docs/archive'`
(the last alternative the brace form, `todo_v0.7.2_gpui-widget-icons-{spec,plan}.md`);
a mention that already points into `docs/archive/`, in any relative form or as the absolute
URL, is left, and so are a citation dated "at `<sha>`" and a path inside another document's
instructions that names its own future target, each named in the commit body. They include
F.5's entries (which cite
`docs/todo_v0.7.1_gpui-widget-icons-rationale.md`), C.6's section of
`docs/evidence/README.md` (which links this specification), and the other documents of the
batch that name the three. The connector's README names none of them (F.1 item 4). The check filters per match, not per line, and prints nothing but
the mentions the exemptions above leave, each named in the commit body:

```sh
git grep -noE '(archive/)?todo_v0\.7\.(1_gpui-widget-icons-rationale|2_gpui-widget-icons-(spec|plan)|2_gpui-widget-icons-\{spec,plan\})' -- ':!docs/archive' | grep -v ':archive/'
```

The three documents' own relative links are checked again after the move, since they now
resolve from `docs/archive/`, with rule 22 (b)'s exemptions (placeholders, texts quoted
from other files, URLs), and each one that no longer resolves is rewritten in the same
commit. The archive index check (`scripts/check_release.sh:1033`) reads the row.

---

## Part G — Order, gates, close-out

### G.1 Units

Inside 0.7.2: commits on `v0.7.2-rc1` in the worktree `.claude/worktrees/lane2-b`, after
`a3fc81f6`. No branch, no merge, no version step. Where another part says "unit N", it is U N
here; unit 3's three steps are U3a to U3c, unit 0 is D0, and unit 7 is 0.7.2's close-out
(G.4).

| Unit | Parts | Why it holds together |
|---|---|---|
| D0 — the documents | the approved rationale, this specification renamed, the plan under its new name, and the two live mentions of the old names re-pointed (decision 18), with the approval's date written in the rationale's status and the plan's *Status* | committed first (decision 23), so every later commit points at documents in the tree |
| U1 — the core | A | A.1's functions are reached only through A.2's method; without it they are dead code, an error under `-D warnings`. A.2's `accept` is rebound `mut` inside the Linux block, so the macOS and Windows legs see no `unused_mut` |
| U2 — the lookup | first, C.1's check of `process-working-symbolic` with B.1 uncommitted (a stop where it fails); then B.1 to B.4 | B.1 without its fixture rows fails `the_theme_s_own_name_wins_in_each_convention_s_theme`; the lookup without B.4 fails `check_connector_parity.py` |
| U3a — the measuring tool | C.2 to C.4: the ignored test in `SC/tests.rs` | committed and gated before the run, so the commit C.6 names holds the tool that measured |
| U3b — the run and the verdict | C.4's table, C.5's seven conditions | **the first stop**, and not a commit: a condition met stops the work for the maintainer, with the table and the sheets; none met, the work goes on (C.5) |
| U3c — the record | C.6: `listing.md`, its README, the section of `docs/evidence/README.md`; the tool removed | one commit, so no later unit has to keep the tool compiling |
| U4 — the handle | D; F.2's D items | after the verdict; with D.13 only where condition 7 was met and the maintainer found the load noticeable. D.11's doctest gate lands here and compiles nothing new until U6 |
| U5 — the showcase | E | E.2 needs the handle; E.7's text and the test's phrase change together; E.10's tests fail with E.2's role-first chrome alone |
| U6 — outside the code | F.1, F.3, F.4, F.5 | beside U5, on other files. F.5's entries name the evidence (U3c); item 8's closing line names U4's and U5's commits, so U6 is committed after U5; F.4's count is taken after F.5 |

Order: D0, U1, U2, U3a, U3b (the stop), U3c, U4, U5 with U6. U4 to U6 do not start before
the verdict. Then 0.7.2's close-out (G.4).

**Gates.** During a unit the agent runs the tests the unit names and the ones its edits
reach, among them: in U1, the full check's step "test (native-theme, Linux features, stem
index)", which `--commit` and `--quick` skip and which compares the lookup A.1 rewrites with
the unfiltered one; in U4, D.12's tests on both cores (the asset source is an API of both)
and the connector's doctests (D.3's example, D.11's gate). The orchestrator runs two gates,
as the KDE steps 4 and 5 did:

| After | Gate |
|---|---|
| D0 | the five documentation checkers |
| U1, U2 and U3a | `scripts/check_release.sh --commit`, then the push |
| U3c | the five documentation checkers; its code change, one removed test, is proven by the last gate |
| U4, U5 with U6 | `--commit` (the last code unit), the two GPUI Fast legs and the five documentation checkers, then the push |

The two GPUI Fast legs are `cargo test -p native-theme-gpui --features
gpui-fast,native,watch --all-targets` and `scripts/run_in_runner_env.sh test -p
native-theme-gpui --features gpui-fast,native,watch --example showcase-gpui`, which
`--commit` skips. Clippy builds the connector and the showcase on GPUI Fast in every gate.
A commit before a `--commit` is proven by that `--commit` and by its unit's own tests; a
failure the gate finds is fixed by a new commit, and the plan's record names the commit
that brought it.

### G.2 Line citations

By S6 of `docs/todo_v0.7.1_batch-speed-rationale.md` (the maintainer's decision of
2026-11-15), no unit re-points a line citation its edits move: the existing line-number
citations of the edited files are left as they are and may drift, and a citation this work
writes of a tracked file names the file (with a quoted phrase where useful), never a line.
Pinned sources keep their lines: an upstream crate at a version, a URL at a commit, a
citation "at `<sha>`". The line numbers this specification gives for finding a place are
read at `0fa28027`, as its header says.

**Lines the colour claims cite.** As the head's *Lines that must not move* says: of the
files the claims cite, this work edits `widgets/checkbox.rs` alone, in two doc comments
(D.11), and moves no claimed line.

### G.3 Gates

From `scripts/check_release.sh` at `0fa28027`; its lines are that commit's and are left, and
a step is found by its command. Under `--commit` and `--quick` the test steps run through
cargo nextest where it is installed; the showcases' tests in the runner environment stay on
`cargo test`.

| Command (line) | Catches |
|---|---|
| `cargo test -p native-theme` (`:675`), `--features "$NT_FEATURES"` (`:683-684`) | A.3; A.4's unchanged tests |
| `cargo test -p native-theme-gpui --features native,watch` (`:667`), `--lib --no-default-features` (`:704-705`); full run: `--features gpui-fast,native,watch --all-targets` (`:714-715`) | B.1's fixture test, B.3's tests and the tolerance, D.12, the doctests (README blocks, D.11's gate), `every_colour_claim_is_read_at_the_line_it_cites`, and `every_prose_citation_still_exists` for E.4, E.6 to E.8's citations (both library tests, `CN/src/showcase.rs:2819` and `:3691`, under `#[cfg(test)] mod showcase;`, `CN/src/lib.rs:243-244`) |
| the showcase in the runner environment (`:845-855`) | E.7's phrase, E.10, E.11 |
| clippy `-D warnings` (`:529-550`; the cross-target legs `:494-511`) | the no-feature build of the showcase (decision 3), unused items |
| `check_strict_panic.sh` (`:572-575`, `:592-595` the core; `:624-632` the connector and the showcase) | no panic path in A, B, D, E |
| `python3 scripts/check_panic_patterns.py` (`:400`) | the same, by pattern |
| `bash scripts/check_features.sh` (`:443`) | each feature alone, no warning |
| `check_connector_parity.py` (`:918`) | B.4 |
| `check_widget_coverage.py` (`:914`) | E.12's reasons non-empty |
| `check_showcase_parity.py --check-list` (`:940`) | E.13: the list unchanged |
| the five documentation checkers (`:964-983`) | F.5 (`check_todo_citations.py`) |
| rustdoc `-D warnings`, private items included (`:1002-1010`) | the intra-doc links of A.2, B, D (the `WidgetIcons` mention without a link outside the feature) |
| `cargo fmt --all` | formatting |
| full run only: `scripts/update_provenance.sh check` (`:1302`), `scripts/update_compatibility.sh check` (`:1339`) | both stamps go stale with unit 1 until 0.7.2's asset run: unit 1 edits `native-theme/src`, which the provenance stamp (`scripts/update_provenance.sh:39-70`) and the compatibility stamp (`scripts/update_compatibility.sh:20-23`) hash |

**Not covered by any gate:** the wording of every doc, README, CHANGELOG, roadmap and
backlog text above; the measurement's verdicts; what the
installed themes draw.

### G.4 What 0.7.2's close-out takes from this work

The full check, the compatibility run, the asset run with its capture window, both stamps,
the CHANGELOG's date and CI are 0.7.2's close-out's own, made once for the whole release.
This work adds:

1. **The archive** (F.6): the rationale, this specification and the plan move with 0.7.2's
   other documents, in 0.7.2's one row, with F.6's rewrite of the path mentions and its
   check.
2. **Before the close-out**, the search over the stamped paths (rule 22 (c)) with this
   work's stem, `gpui-widget-icons`: expected, nothing.
3. **The look at the captures** before the asset commit. In the gpui screenshots,
   gpui-component's own glyphs are the preset's icon set's: the carets of the Basic page's
   select and of the chrome's icon-theme Select and preset Combobox, a checked menu row, the window controls where the
   showcase draws the title bar, each drawn empty where the set lacks one; and the Icons
   page's two captions where a capture shows them. The Basic page's number field shows no
   Minus or Plus under a theme (its steps are the showcase's own chevrons,
   `demo::number_input`), and its check boxes keep the connector's stroked mark where the
   preset states `checkbox.check_mark_stroke_width`. In the screenshots workflow's captures
   under macos-monterey, macos-tahoe and windows-11 the widgets keep gpui-component's
   glyphs (SF Symbols and Segoe Fluent are not served).
4. **Parity.** The comparison (`scripts/check_showcase_parity.py`) compares the chrome and
   the Basic page at their sample points; whether a sample point lies on a widget's glyph
   was not checked (not verified). A difference it reports is a fault, settled by rule 19
   of `docs/todo_v0.7.1_batch-speed-rationale.md`, not asked: one this work's changes
   explain is fixed; one that is a toolkit's limit gets a `[[parity.group]]` exception with
   its reason and source, only for the difference shown, and is reviewed.
5. **The sheets** under `target/widget-icons-sheets/` are deleted on the maintainer's word.

---

## Appendix 1 — The 106 names

Read from `CN/src/icons.rs` at `0fa28027` (`lucide_name_for_gpui_icon` `:210-329`,
`material_name_for_gpui_icon` `:354-471`, `freedesktop_name_candidates` `:557-678`) with a
script, and B.1's two entries. **E**: one of `ESSENTIAL_ICONS`; **W**: a gpui-component
widget draws it (Appendix 2). Under Lucide the handle answers `StarFill` "not mine"; a `—`
under Material is drawn empty. In a freedesktop theme the first name the theme has, in the
by-names order, of one colour (B.2); under SF Symbols and Segoe Fluent nothing.

| `IconName` | | Lucide | Material | freedesktop candidates |
|---|---|---|---|---|
| `ALargeSmall` |  | `a-large-small` | `format_size` | `format-font-size-more`, `zoom-in` |
| `ArrowDown` | W | `arrow-down` | `arrow_downward` | `go-down-skip`, `go-bottom` |
| `ArrowLeft` |  | `arrow-left` | `arrow_back` | `go-previous-skip`, `go-first` |
| `ArrowRight` |  | `arrow-right` | `arrow_forward` | `go-next-skip`, `go-last` |
| `ArrowUp` | W | `arrow-up` | `arrow_upward` | `go-up-skip`, `go-top` |
| `Asterisk` | W | `asterisk` | `emergency` | `rating`, `starred` |
| `Ban` | W | `ban` | `block` | `action-unavailable` |
| `Battery` |  | `battery` | `battery_0_bar` | `battery` |
| `BatteryCharging` |  | `battery-charging` | `battery_charging_full` | `battery-100-charging`, `battery-full-charging` |
| `BatteryFull` |  | `battery-full` | `battery_full` | `battery-100`, `battery-full` |
| `BatteryLow` |  | `battery-low` | `battery_2_bar` | `battery-020`, `battery-low` |
| `BatteryMedium` |  | `battery-medium` | `battery_4_bar` | `battery-050`, `battery-good` |
| `BatteryWarning` |  | `battery-warning` | `battery_alert` | `battery-010`, `battery-caution` |
| `Bell` |  | `bell` | `notifications` | `notification-active`, `alarm` |
| `BookOpen` |  | `book-open` | `menu_book` | `help-contents` |
| `Bot` |  | `bot` | `smart_toy` | `face-smile` |
| `Building2` |  | `building-2` | `apartment` | `applications-office`, `network-workgroup` |
| `Calendar` | W | `calendar` | `calendar_today` | `view-calendar`, `x-office-calendar` |
| `CaseSensitive` | W | `case-sensitive` | `match_case` | `format-text-uppercase`, `format-text-rich` |
| `ChartPie` |  | `chart-pie` | `pie_chart` | `office-chart-pie`, `x-office-spreadsheet` |
| `Check` | EW | `check` | `check` | `dialog-ok`, `object-select` |
| `ChevronDown` | EW | `chevron-down` | `expand_more` | `go-down` |
| `ChevronLeft` | W | `chevron-left` | `chevron_left` | `go-previous` |
| `ChevronRight` | W | `chevron-right` | `chevron_right` | `go-next` |
| `ChevronsUpDown` | W | `chevrons-up-down` | `unfold_more` | `handle-sort`, `list-drag-handle` |
| `ChevronUp` | W | `chevron-up` | `expand_less` | `go-up` |
| `CircleAlert` |  | `circle-alert` | `error` | `emblem-important` |
| `CircleCheck` | W | `circle-check` | `check_circle` | `emblem-ok-symbolic`, `object-select` |
| `CircleUser` |  | `circle-user` | `account_circle` | `user-identity`, `avatar-default` |
| `CircleX` | W | `circle-x` | `cancel` | `dialog-error` |
| `Close` | EW | `x` | `close` | `tab-close`, `window-close` |
| `Copy` | W | `copy` | `content_copy` | `edit-copy` |
| `Cpu` |  | `cpu` | `memory` | `cpu`, `computer` |
| `Dash` |  | `minus` | `remove` | `list-remove` |
| `Delete` |  | `delete` | `delete` | `edit-delete` |
| `Ellipsis` | W | `ellipsis` | `more_horiz` | `overflow-menu`, `view-more-horizontal` |
| `EllipsisVertical` |  | `ellipsis-vertical` | `more_vert` | `overflow-menu`, `view-more` |
| `ExternalLink` | W | `external-link` | `open_in_new` | `external-link`, `insert-link` |
| `Eye` | W | `eye` | `visibility` | `view-visible`, `view-reveal` |
| `EyeOff` | W | `eye-off` | `visibility_off` | `view-hidden`, `view-conceal` |
| `File` |  | `file` | `description` | `text-x-generic` |
| `FileText` |  | `file-text` | `description` | `text-x-generic` |
| `Folder` |  | `folder` | `folder` | `folder` |
| `FolderClosed` |  | `folder-closed` | `folder` | `folder` |
| `FolderOpen` |  | `folder-open` | `folder_open` | `folder-open` |
| `Frame` |  | `frame` | `crop_free` | `select-rectangular`, `selection-mode` |
| `GalleryVerticalEnd` |  | `gallery-vertical-end` | `view_carousel` | `view-list-icons`, `view-paged` |
| `Github` |  | `github` | `code` | `vcs-branch`, `applications-engineering` |
| `Globe` |  | `globe` | `language` | `globe`, `web-browser` |
| `HardDrive` |  | `hard-drive` | `hard_drive` | `drive-harddisk` |
| `Heart` |  | `heart` | `favorite` | `emblem-favorite`, `starred` |
| `HeartOff` |  | `heart-off` | `heart_broken` | `non-starred` |
| `Inbox` | W | `inbox` | `inbox` | `mail-folder-inbox`, `mail-send-receive` |
| `Info` | W | `info` | `info` | `dialog-information` |
| `Inspector` | W | `scan` | `developer_mode` | `code-context`, `preferences-system-details` |
| `LayoutDashboard` |  | `layout-dashboard` | `dashboard` | `view-grid` |
| `Loader` | W | `loader` | `progress_activity` | `process-working-symbolic`, `content-loading` |
| `LoaderCircle` |  | `loader-circle` | `autorenew` | `process-working-symbolic`, `content-loading` |
| `Map` |  | `map` | `map` | `find-location` |
| `Maximize` | W | `maximize` | `open_in_full` | `view-fullscreen` |
| `MemoryStick` |  | `memory-stick` | `memory_alt` | `memory`, `media-flash` |
| `Menu` |  | `menu` | `menu` | `open-menu` |
| `Mic` | W | `mic` | `mic` | `audio-input-microphone` |
| `Minimize` | W | `minimize` | `minimize` | `window-minimize` |
| `Minus` | EW | `minus` | `remove` | `list-remove` |
| `Moon` |  | `moon` | `dark_mode` | `weather-clear-night` |
| `Network` |  | `network` | `lan` | `network-workgroup` |
| `Palette` |  | `palette` | `palette` | `palette`, `color-select` |
| `PanelBottom` | W | `panel-bottom` | `dock_to_bottom` | `view-split-top-bottom`, `view-dual` |
| `PanelBottomOpen` | W | `panel-bottom-open` | `web_asset` | `view-split-top-bottom`, `view-dual` |
| `PanelLeft` | W | `panel-left` | `side_navigation` | `sidebar-expand-left`, `sidebar-show` |
| `PanelLeftClose` | W | `panel-left-close` | `left_panel_close` | `view-left-close`, `sidebar-show` |
| `PanelLeftOpen` | W | `panel-left-open` | `left_panel_open` | `view-left-new`, `sidebar-show` |
| `PanelRight` | W | `panel-right` | `right_panel_close` | `sidebar-expand-right`, `sidebar-show-right` |
| `PanelRightClose` | W | `panel-right-close` | `right_panel_close` | `view-right-close`, `sidebar-show-right` |
| `PanelRightOpen` | W | `panel-right-open` | `right_panel_open` | `view-right-new`, `sidebar-show-right` |
| `Pause` |  | `pause` | `pause` | `media-playback-pause` |
| `Play` |  | `play` | `play_arrow` | `media-playback-start` |
| `Plus` | EW | `plus` | `add` | `list-add` |
| `Redo` |  | `redo` | `redo` | `edit-redo` |
| `Redo2` |  | `redo-2` | `redo` | `edit-redo` |
| `RefreshCw` | W | `refresh-cw` | `refresh` | `view-refresh` |
| `Replace` | W | `replace` | `find_replace` | `edit-find-replace` |
| `ResizeCorner` |  | `grip` | `drag_indicator` | `drag-handle`, `list-drag-handle` |
| `RotateCw` |  | `rotate-cw` | `rotate_right` | `object-rotate-right` |
| `Search` | W | `search` | `search` | `edit-find` |
| `Settings` |  | `settings` | `settings` | `preferences-system` |
| `Settings2` |  | `settings-2` | `tune` | `configure`, `preferences-other` |
| `SortAscending` | W | `arrow-up-narrow-wide` | `arrow_upward` | `view-sort-ascending` |
| `SortDescending` | W | `arrow-down-wide-narrow` | `arrow_downward` | `view-sort-descending` |
| `Square` | W | `square` | `square` | `media-playback-stop` |
| `SquareTerminal` |  | `square-terminal` | `terminal` | `utilities-terminal` |
| `Star` | W | `star` | `star` | `non-starred` |
| `StarFill` | W | — | `star_fill1` | `starred` |
| `StarOff` |  | `star-off` | — | `non-starred` |
| `Sun` |  | `sun` | `light_mode` | `weather-clear` |
| `ThumbsDown` |  | `thumbs-down` | `thumb_down` | `rating-unrated`, `process-stop` |
| `ThumbsUp` |  | `thumbs-up` | `thumb_up` | `approved`, `checkbox-checked` |
| `TriangleAlert` | W | `triangle-alert` | `warning` | `dialog-warning` |
| `Undo` |  | `undo` | `undo` | `edit-undo` |
| `Undo2` | W | `undo-2` | `undo` | `edit-undo` |
| `User` | W | `user` | `person` | `system-users` |
| `WindowClose` | EW | `x` | `close` | `window-close` |
| `WindowMaximize` | EW | `maximize` | `open_in_full` | `window-maximize` |
| `WindowMinimize` | EW | `minus` | `minimize` | `window-minimize` |
| `WindowRestore` | EW | `minimize-2` | `close_fullscreen` | `window-restore` |

## Appendix 2 — What gpui-component's widgets draw (0.9.1)

Every `IconName::` in `GC/` outside test modules (a `#[cfg(test)] mod … {` and what follows
it in its file; `src/input/group/tests.rs` left out), read with a script on 2026-11-14: 52
names in 43 files. Two more reach the resolver as raw `svg` paths: the check box's
(`checkbox.rs:221`) and the questionnaire's (`questionnaire/components.rs:866`), both
`Check`, both counted below.

At 0.10.0 (read 2026-11-19 with the same script; the plan's record, `icons/upstream-reread.md`,
has every line): the same 52 names, in 44 files. Added: `checkbox.rs` draws `Minus` for
the indeterminate state through the same raw `svg` path as `Check`; `radio.rs` names
`Check` itself; `breadcrumb.rs`, `menu/popup_menu.rs` and `sidebar/menu.rs` draw
`ChevronLeft` under a right-to-left layout, and `time/calendar.rs` and `pagination.rs`
swap their two chevrons there. Nothing is gone. The lines below are 0.9.1's.

| File | Names (line) |
|---|---|
| `accordion.rs` | ChevronDown :327 |
| `alert.rs` | Info :79, :93, CircleCheck :100, TriangleAlert :107, CircleX :114, Close :247 |
| `attachment.rs` | Close :609, RefreshCw :631, Ban :801, :815 |
| `avatar/avatar.rs` | User :35 |
| `breadcrumb.rs` | ChevronRight :143 |
| `carousel/carousel.rs` | ChevronLeft :752, ChevronRight :753, ChevronUp :754, ChevronDown :755 |
| `checkbox.rs` | Check :221 |
| `clipboard.rs` | Check :110, Copy :112 |
| `combobox.rs` | Inbox :270 |
| `command/state.rs` | Check :874, Search :958 |
| `dialog/dialog.rs` | Close :859 |
| `diff/mod.rs` | ArrowDown :1080, ArrowUp :1087, ChevronsUpDown :1094, ChevronDown :1096, ChevronRight :1282, ChevronDown :1284, Plus :1635 |
| `dock/tab_panel.rs` | PanelLeft :254, PanelLeftOpen :255, PanelRight :256, PanelRightOpen :257, PanelBottom :258, PanelBottomOpen :259, Minimize :316, Maximize :317, Ellipsis :343, Close :529 |
| `input/clear_button.rs` | Close :12 |
| `input/input.rs` | Eye :502, EyeOff :504, ChevronRight :584, ChevronDown :586 |
| `input/number_input.rs` | Minus :170, Plus :200 |
| `input/otp_input.rs` | Asterisk :137 |
| `input/search.rs` | CaseSensitive :367, Replace :391, ChevronLeft :403, ChevronRight :413, Close :432 |
| `inspector.rs` | Inspector :517, Close :531 |
| `list/delegate.rs` | Inbox :83 |
| `list/list.rs` | Search :705 |
| `menu/popup_menu.rs` | Check :1270, :1321, ExternalLink :1462, ChevronRight :1506 |
| `message_scroller.rs` | ArrowDown :471 |
| `notification.rs` | Info :49, CircleCheck :50, TriangleAlert :51, CircleX :52, Close :464 |
| `pagination.rs` | ChevronLeft :119, ChevronRight :121, Ellipsis :235 |
| `questionnaire/components.rs` | Check :866 |
| `rating.rs` | StarFill :162, Star :164 |
| `searchable_list/adapter.rs` | Check :130 |
| `searchable_list/item.rs` | Check :42 |
| `select.rs` | ChevronDown :60, Inbox :284 |
| `setting/page.rs` | Undo2 :264 |
| `setting/settings.rs` | Search :147, :190 |
| `sheet.rs` | Close :212 |
| `sidebar/menu.rs` | ChevronRight :354 |
| `sidebar/mod.rs` | PanelLeftOpen :360, PanelRightOpen :362, PanelLeftClose :366, PanelRightClose :368 |
| `speech/button.rs` | Square :135, Mic :137 |
| `speech/indicator.rs` | Square :130 |
| `spinner.rs` | Loader :25 |
| `table/delegate.rs` | Inbox :195 |
| `table/state.rs` | SortAscending :1895, SortDescending :1896, ChevronsUpDown :1897 |
| `time/calendar.rs` | ChevronLeft :107, ChevronRight :110 |
| `time/date_picker.rs` | Calendar :779 |
| `title_bar.rs` | WindowMinimize :149, WindowRestore :150, WindowMaximize :151, WindowClose :152 |

A loading `Button` draws a `Spinner` (`button/button_icon.rs`, through `Button::loading_icon`),
whose default is `Loader` (`spinner.rs:25`). From 0.10.0 a loading `Button` without an icon
draws one too (`button/button.rs`, `RenderOnce for Button`).

## What was not verified

- Nothing was compiled or run (no cargo command was allowed): every signature, `cfg` and
  test above is read against the sources, not built. Among them: that `load_where`'s
  closures satisfy the borrow checker as described (a closure owning the found icon, given
  to `first_of_names` as `FnMut`); that the README block of F.1 compiles; that
  `first_of_name`'s extraction leaves `indexed_lookup_matches_the_unfiltered_lookup_on_installed_themes`
  passing (read: the same body).
- The tolerance's value and that the two-colour fixture lies above it (B.3): the test
  measures both; if `M ≥ F` the implementer stops and reports.
- What the installed themes give, how many are one colour, which names are found at 16 and
  24 px, and every listing item (Part C): measured by the plan.
- Whether an `<svg>` with no children parses in usvg (decision 9 does not need it).
- That an empty icon's `Ok(None)` is asked for every frame without a measurable cost (read:
  one map lookup).
- That the five tests opened with `open_with_widget_icons` (E.2, E.11) pass as written:
  they draw a window with the handle following, so a widget glyph's path there is the
  handle's; their assertions read the report, the captions and the info notes, not a
  glyph's path, and are made after a bundled-set preset is installed or, in
  `a_fixed_row_keeps_the_widget_icons_where_the_detection_fails`, a bundled set fixed (that
  `no-such-icon-theme` is installed on no machine is assumed, as `SC/tests.rs:12577-12582`
  assumes it). The other tests run with no handle, as today.
- Which theme of a judged theme's chain held an icon's file: native-theme returns bytes, not
  paths, and the measurement reads the themes through it only (C.2); the table gives the
  theme searched and the other installed themes with the same hash. Where two candidates'
  files have the same bytes, the earlier one is named.
- The timings' precision: five runs of each step in one process of the dev build, on one
  machine, read by the maintainer as figures, not as a statistical test; the showcase's
  figure leaves out the animation caches and the redraw (C.4, item 7), which errs towards
  stopping. That D.8's load costs no more than the 106 calls C.4 times (its map inserts and
  the floor's check are not timed), and that the test's model of D.8's cache matches Part D
  as built (D.12's `a_mode_switch_loads_only_the_new_icon_theme` tests the real one).
- That a fixture with no opaque part stays within the tolerance (B.3's
  `an_icon_with_no_opaque_part_is_one_colour`): computed over a model of the premultiplied
  value's rounding and `unpremultiply_alpha`'s division, not over tiny-skia's code (not
  read), so its worst case, half the opaque icons' with the allowance `ρ`, is the model's;
  the tolerance is a measured maximum, not the opaque icons' worst case, so the test is
  not proven to pass; not run.
- That a second resolution of a preset in the other mode (E.2) costs nothing noticeable at a
  theme switch: not measured.
- That `crate::icon_fixture::run` re-runs a `#[gpui::test]` by its name in the child as it
  re-runs a plain `#[test]` (D.12's `the_resolver_follows_the_mode`): the macro's test name
  was not read. Where it does not, the test checks the mode's table through `answer` with
  `dark` true and false, and a `#[gpui::test]` with no fixture checks that the crate-private
  function `lookup` reads the mode with (decision 11) gives dark after
  `GpuiTheme::change(Dark, None, cx)` and light after the change back.
- The 0.9.1 specification's B.3, B.13, B.15, D.3, D.4 and E landed in v0.7.1 and were found
  on the tree by the reviews of 2026-11-19; E.7 and F.5 quote their texts and stop where a
  text differs.
- The amendment of 2026-11-19: nothing was compiled or run for it. E.4's three answers and
  its texts, and E.3's four changed texts, were read against the tree, not built. The
  reviews did not check that cargo nextest runs the showcase example's tests as `cargo
  test` does (decision 27's reasoning does not depend on it), and upstream's default
  glyphs' shapes were not compared again (gpui-kit-assets 0.10.0's files are byte-identical
  to 0.9.1's).
