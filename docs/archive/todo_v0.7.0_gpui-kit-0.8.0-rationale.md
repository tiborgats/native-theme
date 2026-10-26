# v0.7.0 — GPUI Kit 0.8.0 (G3): Rationale

**Status:** design, drafted 2026-10-21 from the two investigation reports and the upstream
sources, self-checked; revised the same day for the maintainer's decision on D1 (K14–K18),
self-checked again; checked independently; approved by the maintainer on 2026-10-21, as
D11 in [`todo_v0.7.0_code-review-plan.md`](todo_v0.7.0_code-review-plan.md) records; being implemented as
milestone G3. What to build: [`todo_v0.7.0_gpui-kit-0.8.0-spec.md`](todo_v0.7.0_gpui-kit-0.8.0-spec.md).
Tasks and gates: [`todo_v0.7.0_gpui-kit-0.8.0-plan.md`](todo_v0.7.0_gpui-kit-0.8.0-plan.md).
*Added 2026-10-21 (G3 T1 fix round 1):* the decisions run to K19, Task 1's deviation, after
K14–K18; K19's "a draw prepaints everything before it paints anything" reads
`window.rs:3757-3821` (the prompt, drag and tooltip paint included), where it cites `:3757-3813`.

GPUI Kit v0.8.0 was released on 2026-10-19: gpui-kit, gpui-kit-assets, gpui-component,
gpui-component-macros and gpui-base 0.8.0, on gpui-pre 0.3.9 (unchanged) or on GPUI Fast
0.1.10. The maintainer's decision (2026-10-20): *"fast forward to gpui-kit v0.8.0, skip
gpui-kit v0.7.3. So the gpui-fast version is 0.1.10, we must support at this point.
Investigate what is new, what should we add/change in the gpui connector and the
showcase."* The work is milestone **G3** of the whole-codebase review
(`docs/todo_code-review-plan.md`, decision D11 at registration): the gpui connector and its
showcase only. It runs as the one writer between M9b's tasks, and its first green push
closes M9a's one red CI job.

Abbreviations: `GK7`/`GK8` = gpui-kit 0.7.3/0.8.0, `GC7`/`GC8` = gpui-component 0.7.3/0.8.0
`src/`, `GB7`/`GB8` = gpui-base 0.7.3/0.8.0 `src/`, `GF7`/`GF10` = gpui-fast 0.1.7/0.1.10
`src/` (all from `~/.cargo/registry/src/index.crates.io-*/`), `CN` =
`connectors/native-theme-gpui/`, `SC` = `CN/examples/showcase-gpui/`, `MUX` =
microsoft-ui-xaml at `8463f45162149de0ec3ad7df752596893fe3e13e` (the pin
`docs/platform-facts.md` uses). The reports, the diagnosis and the check passes cited by
name are kept in `.superpowers/sdd/todo_code-review-plan/g3/` (plan Task 0); nothing a
task needs is only there
(spec, Inputs). Our line numbers are of `v0.7.0-rc1` at `7492e62f`, those
the D1 revision added (§1.3's second half, §1.5, K14–K18) of `a172d3dd`, or of `2e80f87e` /
`fa90c6c0` where so marked; the iced showcase
(`connectors/native-theme-iced/examples/showcase-iced.rs`), which nearly every M9b commit
changes, is cited by function name only. Locate by content.

---

## 1 -- What was measured and read (2026-10-20/21)

### 1.1 The trial build (a throwaway worktree at `7492e62f`, rustc 1.99.1)

- Only the four version requirements in `CN/Cargo.toml` (`:66`, `:70`, `:75`, `:104`)
  raised to 0.8.0, then `cargo update -p gpui-kit -p gpui-component -p gpui-base`:
  15 packages change in the lock (gpui-kit, gpui-kit-assets, gpui-component,
  gpui-component-macros, gpui-base to 0.8.0; gpui-fast and its eight sub-crates 0.1.7 →
  0.1.10; `gpui-pre-ios` 0.3.9 added, an iOS-only dependency of gpui-fast-platform). `image`
  stays one copy, 0.25.10.
- `cargo check -p native-theme-gpui --all-targets`, with and without
  `--features gpui-fast,native,watch`, and the showcase example on both cores: **no error,
  no warning**. `Theme` is still destructured exhaustively (`CN/src/lib.rs`), the
  `ThemeColor` field count (139) holds, every `IconName` the tables name exists.
- `cargo test -p native-theme-gpui --lib`: gpui-pre 355 passed, 4 failed; GPUI Fast 358
  passed, 5 failed. The four: the two README floor tests (`compat::`), and the two showcase
  citation gates — 184 of 798 colour claims and 15 of 928 prose citations point at lines
  that moved (every cited field is still read in the same file; none is gone). The fifth,
  on GPUI Fast only: `a_loading_progress_bar_stands_still_under_reduced_motion` reads
  `None` for the fill colour (§1.2).
- **Not run in the trial:** the showcase's own tests (`SC/tests.rs`, about 273), the seam
  tests (`CN/tests/seams.rs`), the MSRV checks, `cargo audit`. Not measured anywhere yet:
  whether the `window.refresh()` and `.occlude()` workarounds are still needed, and the
  GPUI Fast run-time layout.

### 1.2 What changed upstream that reaches us

- **Theme schema: nothing.** `GC7/theme/` and `GC8/theme/` are identical (`diff -rq`), as
  are `icon.rs` and gpui-kit-assets' sources: no colour, token or icon to map.
- **API:** nothing the connector or showcase names was removed or changed signature (the
  compile confirms it). New: `TabVariant::Folder`; DataTable's `fit_column`,
  `fixed_cols_border`, `focus_ring(bool)`, `select_on_modifier_click`;
  `ResizablePanel::fixed()` and a public `ResizeHandle`; `accessibility_label` on
  gpui-component's `Slider` and `NumberInput`, and disabled state reported to accessibility
  by gpui-base's own controls; `scroll_bounce(bool)` on seven widgets; `SelectionFormat::
  SourceAcrossStyles`; `DockToggleEvent`; editor options; an optional `syntect`
  highlighter; `#[doc(hidden)] pub const GPUI_FAST` in gpui-kit (`GK8/lib.rs:119-122`).
- **Behaviour:** every gpui-component scrolling widget is wrapped in gpui-base's
  `ScrollBounce` (`GC8/scroll/scrollable.rs:188`, `list/list.rs:603`, `tree.rs:127`,
  `table/state.rs:2727`, `diff/mod.rs:557`, `message_scroller.rs:383`,
  `setting/page.rs:233`, `sidebar/mod.rs:455`, `command/state.rs:1033`), which claims the
  wheel for the innermost area under the pointer in the capture phase and stops its
  propagation (`GB8/scroll_bounce.rs:248-270`, `:556-610`) even where bounce is off, off
  macOS, iOS and Android (`:170-178`; upstream's test
  `a_disabled_inner_area_still_keeps_its_scroll`); there is no scroll chaining (at its end
  the inner area stretches, `the_innermost_area_takes_the_whole_gesture`). The text caret is
  1 px everywhere; completion popups draw as popup-menu rows; a Tree selects on click; the
  scrollbar thumb's length is `viewport / content × track`; a Button's hover style is
  registered whenever it is interactive and not selected (`GC8/button/button.rs:736-747`),
  which moves every later line of that file by 6; the variant colour functions and their
  `outline` arms are unchanged (present in 0.7.3, `GC7/button/button.rs:989-1345`).
- **gpui-fast 0.1.7 → 0.1.10:** the stale child-list crash M9a's CI hit
  (`gpui-fast-panic-diagnosis.md`, kept in `.superpowers/sdd/todo_code-review-plan/g3/`) is
  fixed: `apply_children` compares Taffy's
  actual children and `set_children` records robbed parents (`GF10/fast/layout.rs:652-689`;
  release note v0.1.9, #68). The test-mode carry-forward of every last-frame
  `debug_bounds` is gone (`GF7/fast/retained.rs:1360-1371` removed; release notes v0.1.9,
  #74/#75), which is the reason the two test draw helpers call `window.refresh()`. New:
  rounded overflow clips, whose children are painted into a clip-group `LayerFrame`
  (`GF10/fast/rounded_clip_scene.rs:1-10`, `:60-75`) instead of the scene's `quads`, so
  `Window::painted_quads()` (`GF10/window.rs:2925`) no longer sees them; the whole scene is
  reachable through `Window::painted_scene()` (`GF10/fast/rounded_clip.rs:232-237`,
  test-support) and the public `SceneLayers::frames` / `LayerContent::scene()`
  (`GF10/fast/layers/scene.rs:66-69`, `:186`, re-exported at `GF10/gpui.rs:123-126`).
  0.1.8 fixed an unconstrained text measurement answered at another width (content-sized
  boxes); 0.1.10 synced Zed (wgpu 30, an opt-in `hot-reload` feature whose dependencies do
  not enter our lock, source-over compositing of translucent layers).
  *Added 2026-10-22 (G3 T6 fix round 1), on "#74/#75" above:* the debug-bounds fix is PR
  #74; the v0.1.9 notes list it with #75, which aligns composited scroll content's bounds.
- **Floors upstream sets:** gpui-kit, gpui-component and gpui-base 0.8.0 each ask for
  `gpui-fast = "0.1.10"` (`GK8` `Cargo.toml:275-279`, `GC8` `:288-292`, `GB8` `:93-97`) and
  pin `gpui-pre = "=0.3.9"` (`GK8` `:264-266`, `GC8` `:278-280`, `GB8` `:89-91`). No crate of
  the family declares a `rust-version`; `std::hint::cold_path`, our 1.95.0 floor's reason,
  is still at `profiler.rs:473` in both cores.
- **A side finding:** gpui-kit 0.7.3 asks for `gpui-fast = "0.1.7"`, a caret, so a fresh
  resolve of 0.7.3 already takes 0.1.10; the workspace is on 0.1.7 only by its lock, and
  the README's Verified line says 0.1.8 (`CN/README.md:95`).

### 1.3 Platform facts for `TabVariant::Folder`

Folder is "tabs shaped like a folder's index tabs, as in a web browser: the selected tab
rises from the bar with rounded top corners and curves out into the content below it"
(`GC8/tab/tab.rs:50-61`), with a 1 px separator between unselected tabs and an inset hover
fill (`:958-1027`); one radius serves the top corners and the outward curves
(`FolderTabMetrics`, `:16-46`): `Theme::radius` at the XSmall and Small sizes, `radius_lg`
at Medium and Large, capped at a third of the tab's height (`:30-45`), the connector
mapping the two from `defaults.border.corner_radius` and `.corner_radius_lg`
(`CN/src/lib.rs:327-328`).

- **Windows (WinUI 3 `TabView`) is a folder bar.** The selected `TabViewItem` is filled
  with a path whose top corners take `OverlayCornerRadius` (8, platform-facts §2.11
  `border.corner_radius`) and whose bottom corners curve *out* by a hard-coded 4 px
  ("Assumes 4px curving-out corners, which are hardcoded in the markup", MUX
  `controls/dev/TabView/TabViewItem.cpp:106`, in `UpdateTabGeometry`, `:98-123`; drawn as
  `SelectedBackgroundPath`, set 4 outside the tab, `Margin="-4,0"`, `TabView.xaml:551`);
  each tab draws a 1 px `TabSeparator` (`TabView.xaml:553`, `DividerStrokeColorDefaultBrush`,
  `TabView_themeresources.xaml:46`, margin `0,8,0,8`, `:266`), hidden on the selected tab,
  on the one left of it (`TabView.cpp:217-231`) and under the pointer (`TabView.xaml:322`,
  `:332`); the hover fill is `TabViewItemHeaderBackgroundPointerOver` (§2.11
  `hover_background`). So Folder's shape is WinUI's, with two differences: WinUI's
  curves (4) are not its top radius (8), and its separator is `DividerStrokeColorDefault`,
  not the border colour. (Folder also strokes no outline: `folder_tab_shape` fills its path
  only, `GC8/tab/tab.rs:445-497`; WinUI's, below, follows the curves.)
- **WinUI's outline follows the curves** (read at the pin for D1, 2026-10-21). The selected
  tab's own border is 1 on its top and sides only (`TabViewSelectedItemBorderThickness`
  `1,1,1,0`, `TabView_themeresources.xaml:268`; set in the `Selected` state,
  `TabView.xaml:344-345`), in `TabViewSelectedItemBorderBrush`, a gradient that is
  `CardStrokeColorDefault` above and transparent over the bottom 4 px (`MappingMode="Absolute"`,
  `EndPoint="0,4"`, both stops at offset 1.0, flipped by `ScaleY="-1"`,
  `TabView_themeresources.xaml:72-80`, dark `:150-158`); and its container is widened by 1
  on each side (`TabViewSelectedItemHeaderMargin` `-1,0,-1,1`, `:269`, set at
  `TabView.xaml:343`), so the side edge lies just outside the fill path's vertical edge. The
  `Selected` state shows two 4 × 4 arcs, one at each foot (`LeftRadiusRenderArc`,
  `RightRadiusRenderArc`, `TabView.xaml:547-548`, `Margin` −4 on their outer side; made
  visible at `:338-339`), filled in `TabViewBorderBrush`: a band 1 wide between the radii 3
  and 4 about the point 4 above and 4 outside the fill path's corner, that is on the inner
  side of the curve, whose top end covers the same pixel column as the side edge; the band
  ends 1 above the item's bottom (the path's `…3H0`), where it meets the neighbours' 1 px
  rule (`:546`). Read from the markup alone (not rendered): the container ends 1 above the
  bottom (the margin's 1) and is transparent over its bottom 4, so one row may lie between
  the side edge and the arc's top. So WinUI's outline is the side edge, then the arc, then
  the bar's rule. `TabViewBorderBrush` is `CardStrokeColorDefault`
  (`TabView_themeresources.xaml:68`, `:146`), the brush §2.11's `border.color` gives the
  selected tab and `windows-11.toml` states as `tab.border.color` (`[light.tab.border]`
  `#0000000f`, `[dark.tab.border]` `#00000019`). The bar itself draws a 1 px rule in that
  brush under every tab (`BottomBorderLine`, `TabView.xaml:546`, and the strip's own,
  `:36-37`), collapsed under the selected one (`:337`).
- **The fills:** an unselected tab is transparent, `TabViewItemHeaderBackground` =
  `LayerOnMicaBaseAltFillColorTransparent` (`TabView_themeresources.xaml:6`, `:84`), and so
  is the bar, `TabViewBackground` = `SubtleFillColorTransparent` (`:5`, `:83`): both show
  the window, `SolidBackgroundFillColorBase` `#F3F3F3` / `#202020`
  (`Common_themeresources_any.xaml:272`, `:68`), which is why the preset's
  `tab.background_color` and `tab.bar_background` are right to inherit the window colour.
  So, at rest, WinUI loses nothing by painting the selected item's path under its right
  neighbour (a hovered neighbour's translucent fill lies over the right flare there).
  The selected path is `TabViewItemHeaderBackgroundSelected` = `SolidBackgroundFillColorTertiary`
  (`TabView_themeresources.xaml:7`, `:85`; `#F9F9F9` / `#282828`,
  `Common_themeresources_any.xaml:274`, `:70`): the folder stands out from the bar by that
  difference. The preset states `tab.active_background` `#f3f3f3` / `#202020` instead
  (`windows-11.toml` `[light.tab]`, `[dark.tab]`), the last item `docs/todo.md`'s entry
  *windows-11: what WinUI 3 draws, or what Win32 reports* still lists as open (K14). `DividerStrokeColorDefault`, the
  separator's colour, is `#0F000000` light and `#15FFFFFF` dark
  (`Common_themeresources_any.xaml:257`, `:53`).
- **No other platform is.** `docs/platform-facts.md` §2.11: KDE's Breeze tab is joined to
  the frame with a `Highlight` strip on its top edge and darker unselected tabs; its
  selected frame is one rounded rectangle, rounded on the top corners only
  (`roundedPath(strokedRect(frameRect), corners, …)`, Breeze `kstyle/breezehelper.cpp:1482`
  at the facts' pin `f0b1d75`; `corners = CornersTop`, the `border.corner_radius` cell,
  `docs/platform-facts.md:1356`). GNOME's notebook tab carries an accent underline and is
  square (its rule, `notebook > header > tabs > tab`, sets no `border-radius`, libadwaita
  1.10.0 `src/stylesheet/widgets/_notebook.scss:119`; the file's one radius is the arrows',
  `:112`); AdwTabBar's tab is rounded `border-radius: $button_radius` on every
  corner (libadwaita 1.10.0 `src/stylesheet/widgets/_tab-view.scss:160`). Material marks the
  selected tab with an indicator line, its container `corner-none`; macOS draws a CoreUI
  bezel whose shape no source publishes. None of them curves a tab out at its feet.

### 1.4 What `.occlude()` does in the showcase today

Eleven calls. Three are not scroll workarounds and are not touched: the capture pointer
shield (`SC/app.rs:2232`), the title-bar sample's info overlay (`SC/demo.rs:905`) and the
app-menu popup frame (`SC/demo.rs:715`). Eight keep a nested scroller's wheel from
scrolling the page: the DataTable (`SC/demo.rs:4295`), List (`:4532`), Tree (`:4762`),
MessageScroller (`:4979`), the Scrollable Area (`:6506`), the editor and the Diff
(`SC/pages/typography.rs:227`, `:242`) and the Settings frame (`SC/pages/layout.rs:482`, a
plain GPUI `overflow_y_scroll` div around gpui-component's `Settings`). The occlusion costs
hover and tooltips for whatever lies behind the occluding box. The existing gate,
`a_nested_scroller_keeps_the_wheel_to_itself` (`SC/tests.rs:448`), covers the List and the
Tree only.

### 1.5 How the three connectors draw a tab bar today (for D1)

- **gpui:** under a native theme the connector's `widgets::TabBar` draws its own tabs on
  gpui-base's headless `Tabs` and `Tab` (`CN/src/widgets/tab_bar.rs:1-2`, `render`
  `:348-456`); only without one does it render gpui-component's `TabBar` (`fallback`,
  `:321`). Every tab keeps a border `tab.border.line_width` wide on all four sides, coloured
  on the selected tab only (`:389`), whose top corners take `tab.border.corner_radius`
  (`:395`); the row scrolls sideways (`overflow_x_scroll`, `:435`). The showcase's Basic
  tabs, page tabs and inspector tabs are all this bar (`SC/demo.rs:1859`
  `native_tab_strip`, under `:1869` `native_tab_bar` for the page and inspector tabs and
  `:2023` `tab_row` for the Basic ones), each described by `SC/info/layout.rs:709`
  `native_tab_row`. gpui paints an element's children in tree order,
  so a later tab paints over anything an earlier one draws outside its own box.
- **iced:** no tab widget of the connector's own; `styles::aw::tab_bar` styles iced_aw's
  `TabBar` and `Tabs` (`connectors/native-theme-iced/src/styles/aw.rs:204`), and
  `tab_indicator` / `tab_indicator_line` (`connectors/native-theme-iced/src/lib.rs`) lay
  the selected tab's line over a tab or over its place in a bar. The showcase draws the
  page tabs and the inspector tabs as rows of `tab_button`s styled by `tab_style` (an
  `iced::Border` of one width on all four sides, rounded on top), in `tab_row`; without
  `iced_aw` the Basic tabs are a `row` of `tab_button`s of their own, wrapped, built by the
  default build's Basic tab code (since M9b T5, `cffb6355`; in `view_basic` at `2e80f87e`,
  moving into a `basic_tab_row` in M9b's T6 fix round; `tab_row` has the other two callers
  only); with `iced_aw` the Basic tabs and the Extra page's
  `TabBar` / `Tabs` are iced_aw's. iced's `Border` has one width for all sides; a canvas
  frame clips to its bounds (`iced_wgpu-0.14.0` `src/geometry.rs:105-119`); the core
  renderer fills rounded quads and clips to a layer (`iced_core-0.14.0`
  `src/renderer.rs:24`, `:62`).
- **egui:** the connector ships no `egui::Widget` (`connectors/native-theme-egui/README.md:46`);
  its `mapping.toml` names the tab leaves an application reads itself
  (`["tab.active_indicator_side"]` is `derived`, `tested_by = "T18(a)"`,
  `connectors/native-theme-egui/mapping.toml:2349-2351`). The showcase's `tab_bar`
  (`connectors/native-theme-egui/examples/showcase-egui/demo.rs:1192`) draws each tab as
  an `egui::Button` with a stroke on all four sides and top corners rounded (`:1220`), and
  paints the selected tab's line after it (`:1249`). epaint fills convex polygons only
  (`epaint-0.36.2` `src/shapes/path_shape.rs:14`); a `Painter` clips to a rectangle
  (`egui-0.36.2` `src/painter.rs:71`) and strokes circles (`:370`).
- **The shared list:** the three showcases' Basic tabs, page tabs and inspector tabs are
  elements of `docs/showcase-elements.toml` (`basic.tabs.*` `:1361-1390`,
  `chrome.page_tabs.*` `:523-544`, `chrome.side_panel.inspector_tabs.*` `:394-415`), and
  the Basic page's column rule counts the two tabs, `2 * tab.min_width + tab.item_gap`
  (`:619`; gpui's `basic_column_count`, `SC/pages/basic.rs:240`; egui's `column_count`,
  `connectors/native-theme-egui/examples/showcase-egui/pages/basic.rs:137`, `:151`; the
  iced showcase's `basic_column_count`). Each showcase's test holds every listed leaf to a
  route (`SC/tests.rs:12696` `every_listed_leaf_has_a_route`; the iced showcase's
  `the_element_list_parses_and_every_leaf_has_a_route`; egui's
  `widget_info_shows_the_listed_elements_rows`,
  `connectors/native-theme-egui/examples/showcase-egui/tests.rs:4123`, whose routes come
  from `mapping.toml`). The release's parity comparison captures kde-breeze, material-3
  and catppuccin-mocha only (`scripts/capture_window.sh:354-361`).

---

## 2 -- Decisions

### K1 — Floors: GPUI Kit 0.8.0, which brings gpui-fast 0.1.10 (the maintainer's decision)

Options: (a) stay on 0.7.3 and only lock gpui-fast 0.1.10; (b) floors 0.8.0; (c) a range
covering 0.7.3 and 0.8.0; (d) (b) plus a direct `gpui-fast = "0.1.10"` dependency.
(b), as decided, and for reasons that hold on their own: v0.7.0 is unreleased, so no
published `native-theme-gpui` requires 0.7.3 and nobody is left behind (the released 0.6.1
asks for `^0.7.1`, which does not take 0.8.0, so its users stay on 0.7.x, where it builds);
every showcase claim cites one line of one version, so (c) cannot be true for both and
would test only the lock's. (a) keeps
claims true but leaves applications on 0.7.3 free to lock a gpui-fast before 0.1.9, which
crashes in the showcase's case (K4); 0.8.0's `^0.1.10` rules that out for everyone. (d) adds
a second statement of a version gpui-kit already enforces, which drifts; under the
connector's form A (GPUI named through gpui-kit, G1) gpui-kit chooses the GPUI Fast
version, so the README's *Backends* section states that 0.8.0 requires gpui-fast 0.1.10 or
later, and the Compatibility table keeps the floors the manifest states (its test,
`compat::the_readme_states_the_manifest_floors`, compares the table with the manifest).
`gpui = { package = "gpui-pre", version = "0.3.9" }` stays a caret.

### K2 — Lockfile: the scoped update the trial measured

`cargo update -p gpui-kit -p gpui-component -p gpui-base`, the trial's command: the 15
packages of §1.1 and nothing else. A fresh resolve would also take about 111 newer patch
releases outside the family (the report's §5.2); they are not G3's change and would mix
an unreviewed dependency refresh into it. `cargo tree -p native-theme-gpui -e normal`
before and after, in the commit body. `cargo audit` runs on the new lock (the non-gpui
closure is unchanged, so no new advisory is expected; the run, not the expectation, is
the gate).

### K3 — Citations: re-pointed by script (unique text, else a diff line map), then read by hand

Three ways to re-point a citation by script; what none of them maps is read by hand.

- **(a) Unique text**, G1's method (archived 0.7.3 spec §2.1): a cited line's trimmed 0.7.3
  text found exactly once in the 0.8.0 file gives its new number; anything else is set
  aside. Pros: exact where it applies (the whole line is matched); needs only a path and a
  number, so it serves prose comments as well as claims. Cons: every repeated line is set
  aside, however clearly its surroundings place it — in `button/button.rs` all the
  `if outline {` and shared `cx.theme().…` lines below the hover block, which moved by a
  plain +6.
- **(b) The named symbol**: a citation moves to where the symbol or field it names now is,
  and is set aside where it names none, the symbol is gone, or it occurs more than once.
  Pros: no diff; a claim's own field is the anchor. Cons: it needs the same attribution
  (which crate's file, which numbers belong to a citation), so it removes none of the other
  designs' work; prose comments have no extractable symbol, so all of them would be set
  aside; and most fields occur more than once in the cited files (in 0.8.0,
  `tab_active_foreground` on 8 lines of `tab/tab.rs`, `muted_foreground` on 9 of
  `diff/mod.rs`, `button_primary` on 5 and `input` on 4 of `button/button.rs`,
  `tab_foreground` on 5 of `tab/tab.rs`), so most claims would be set aside too.
- **(c) A diff line map**: a `difflib` alignment of each changed upstream file, 0.7.3 →
  0.8.0, mapping the lines of its equal blocks. This is M8 T17's method (its re-map and its
  whole check, `cites.py`'s `line_map`: 1 961 numbers, 0 to fix after its fix round), and
  what a reader does by hand for `button.rs`'s +6. Pros: maps repeated lines by their place.
  Cons: a repeated line can be aligned with the wrong copy, even inside a long equal block
  (check 7: `tab/tab.rs:262`'s `},` placed at `:320`, the new Folder arm's, inside a 70-line
  block, where git's histogram map has the right `:314`; over every line of the changed
  files, 198 of 74 090 repeated-text lines disagree with git, all in blocks of three lines
  or more), so a minimum block length is no guard; a second, independent alignment is.

Measured over the 472 moved first numbers of the report's list in one-crate changed files
(check 5's simulation, re-run here): (a) maps 377, and (c) maps the same 377 to the same
lines; of the 91 (a) sets aside as repeated, (c) maps 89 (34 in `SC/info/`, inside the unit
that holds M9b, 55 elsewhere), and an equal-block minimum of three lines loses none; 2 stay
set aside and 4 are gone. The 94 cited lines whose text is still at their old number were
measured too (check 6): (i)/(ii) map 91 to that same number and 8 numbers (3 first
numbers, 4 range ends, 1 continuation) to another copy, which is the right one, so the
rule has no "stays" shortcut. **The choice: (a) where the text is unique in both files, else
(c)'s position where difflib's equal block and git's `histogram` line map agree, else set
aside** — exact where (a) is, nearly all of (a)'s hand-read set mapped mechanically (the
button offset included). The second map is part of the rule, run before anything is
written, for every number (c) maps, whether it moves or not: gates (2) and (4) read text,
so they cannot tell copies of a repeated line apart, and this is what does. It is cheap
(one `git diff --no-index` per changed file and a short hunk-header parse) and, measured
(check 7), agrees with difflib on 101 of 101 first numbers, 29 of 29 continuations and 110
of 111 range ends, the one disagreement being the `tab/tab.rs:262` case above, where git is
right.

Around the line rule, all three need the same two answers, which the checks of these
documents found gaps in (comma lists and continuations, a second run over moved numbers,
the crate a path means, Task 1's lines already at 0.8.0); each is closed in spec §3.1 and
§3.3: every form the repo writes is followed by M8 T17's chain rule; a chain moves whole or
is set aside whole; a path is attributed among every crate with the file (unchanged files
and gpui-pre included), a crate named nearest before the number winning and an unnamed
path two crates hold set aside; a citation is in scope only if its crate's file changed;
the lines of Task 1's commits are left alone, listed and read at 0.8.0 in Task 2's
review; and gate (4) re-derives all of it in code of its
own, by a scan wider than the chain rule (M8 T17's re-review found a check blind where it
shared the re-map's rule). gpui-kit and gpui-fast files are in scope (gpui-pre is
unchanged, 0.3.9 on both sides, so its citations stay as they are). 
The hand-read set adds what a line number
cannot carry: the panels of widgets whose behaviour changed. The scope is
every citation the repo states as current — `CN/` (sources, tests, examples, README,
manifest comments), `docs/showcase-exceptions.toml`, `tools/showcase-support/src` — not the
history files (ledger, archive, CHANGELOG released sections) and not `docs/todo.md`, whose
entries name their version and gain dated lines instead (`docs/todo.md`'s header:
entries are cited by heading and opening words). A gate for the connector's own doc-comment
citations (the `[1.0]` entry *gpui: a citation gate for the connector's doc comments*) is
not built here: it needs one unambiguous citation syntax across the connector's comments
(about 200 were read by hand at the 0.7.0 bump, that entry says), a rewrite of its own; G3
renumbers them in Task 2's one script run and hand-reads the ones it sets aside in Task 3b
(never a second run, which would read already-moved numbers against 0.7.3), and the entry
gains a dated line.

### K4 — The stale child-list crash: upstream's fix, our regression test, no workaround

M9a's controller ruled a showcase workaround (distinct element ids for an item with and
without its icon) plus the diagnosis's regression test (`m9a-gpuifast-fix-brief.md`),
before 0.8.0 was known. Options now: (a) as ruled; (b) the floor's gpui-fast ≥ 0.1.10 and
the regression test; (c) both. (b): the fix is upstream's own (§1.2), K1's floor makes a
crashing gpui-fast unreachable for any build of this connector, and the workaround would
key the Sidebar items by a detail unrelated to their identity, with a comment about a bug
nobody can hit. The test (`a_sidebar_item_losing_and_regaining_its_icon_is_drawn`, the
diagnosis's §3c) fails on 0.1.7 on any host and must pass on 0.1.10 on both cores: it
guards a regression in the floor as well as in our code. Also dropped from the brief: the
`[upstream]` report (fixed upstream; nothing to file) and a CHANGELOG *Fixed* line (the
`gpui-fast` feature is unreleased — v0.6.1 has none, and GPUI Kit 0.7.1, which it built
on, had no GPUI Fast core — so no released user could reach the crash).

### K5 — The progress-bar test: read the whole painted scene

The trial's options: (a) `render_to_image`, which renders through the platform window's
renderer and is untried on the headless test platform; (b) skip the colour assertion on GPUI Fast; (c) ask upstream to
include clip-group quads in `painted_quads`. Not listed there: (d) read the quads from
`Window::painted_scene()`, descending into the clip-group frames (`tile_size == 0`, whose
content is one scene, `LayerContent::scene()`), through public test-support API (§1.2).
(d): it keeps the assertion on both cores, and it closes a hole wider than the one test —
a test that asserts a quad is **absent** (`SC/tests.rs`, the page-tab row that must paint
no accent, `:3151`; an unselected tab that must not be outlined) passes vacuously on GPUI
Fast once the quad moves into a rounded clip. So every test read of painted quads (and of underlines, which a clip group's scene
holds too) in both test modules goes through one helper per module (the lib tests and the showcase example
are separate crates; the two copies of about 20 lines name each other). The one
order-sensitive read (the switch's outline over its thumb, `CN/src/widgets/tests.rs`) keeps
`painted_quads()`, since paint order is defined within one scene only, and stops skipping
when a quad it needs is missing. (c) is filed as an `[upstream]` entry for the maintainer:
`painted_quads()` documents "the quads in the most recently rendered frame's scene", which
it no longer is under a rounded clip.

### K6 — `window.refresh()` in the test draw helpers: removed only if measured

Options: keep it (harmless on gpui-pre, and gpui-kit 0.8.0's own test helper does the same,
`TestWindowExt::render_frame` calls `refresh()` before drawing, `GK8/test.rs:348-349`; but it
turns GPUI Fast's subtree reuse off in every test frame, so the GPUI Fast suite never
exercises the reuse a real window does); remove it on the release note's word; remove it
behind a measurement. The last, since our absence tests are the ones that exercise reuse
and upstream's helper choosing a full frame for its own tests does not show ours need one: the reason it exists
is gone in 0.1.10 (§1.2), but the five tests that needed it are the proof, not the release
note: take it out of both helpers (`SC/tests.rs` `draw`,
`CN/src/widgets/tests.rs` `redraw`), run the five absence tests named in the todo entry and
both whole suites on both cores; if all pass, the call and its comments go and the entry
*File the gpui-fast issue: in test mode a partly reused frame* closes unfiled; if any fails,
the call stays with a comment that states the 0.1.10 cause found, and the entry gets a dated
line instead.

### K7 — The eight scroll `.occlude()`s: each removed only after a test shows it is unneeded

0.8.0's `ScrollBounce` keeps the wheel inside every gpui-component scroller (§1.2), which is
what the occlusion was for, and the occlusion blocks hover behind its box. Options: keep all
eight (works, keeps the cost and comments that are no longer true); remove all eight on the
sources' word; remove each behind a test. The last, because two of the
eight do not wrap a plain gpui-component scroller (the Settings frame is GPUI's own
`overflow_y_scroll` div; the editor is gpui-base's input, which keeps the wheel through its
own listener, upstream's test `a_textarea_keeps_its_scroll_from_the_area_around_it`,
`GB8/input/base/element.rs:3832`). So the gate generalises: `a_nested_scroller_keeps_the_wheel_to_itself`
covers all eight sites (on screen, the page able to scroll, the inner scroller moved, the
page not), passes with every occlusion first, then each `.occlude()` is removed alone and
stays removed only if the test still passes on both cores. The sources lead to expect the
seven gpui-component and gpui-base scrollers' occlusions to go and the Settings frame's to
stay, with a comment naming its 0.8.0 reason; the test decides each. Nothing is lost
where an occlusion goes: at an inner scroller's end the page still does not scroll, as
under the occlusion (upstream has no chaining); and over an inner area whose content fits,
the page now scrolls, since `ScrollBounce` lets the wheel go when it cannot move
(`GB8/scroll_bounce.rs:570`), where the occlusion swallowed it.

*Added 2026-10-22 (G3 T6):* the test decided the Settings frame too, and its occlusion
went (Task 5, `342232d6`): the `Settings` fills the frame, so the frame's own
`overflow_y_scroll` never scrolls, and the scroll owner under it is the Settings' page list,
which `ScrollBounce` wraps (`GC8/setting/page.rs:233`); Task 6 made the frame
`overflow_hidden`, the clip its dead `overflow_y_scroll` gave (spec §4). The trade-off is
also wider than an inner area whose content fits: the occlusions covered whole demo boxes,
so the page now scrolls over their parts that do not scroll as well (the frames' padding
and borders, the Settings' heading, sidebar search field and resize handle), as native
toolkits do.

*Added 2026-10-22 (G3 T6 fix round 1), on the paragraph above:* the frame was already
`overflow_hidden` through the showcase's `demo_frame`, so the fix round removed the
`.overflow_hidden()` Task 6 added: the dead `overflow_y_scroll` deleted alone leaves the same
clip.

### K8 — `TabVariant::Folder`: shown in the showcase as upstream's variant

Shown, by the rule G1 set (every new element a gpui-kit release brings is shown): a Folder
`TabBar` on the Layout page, over a panel in `tab_active` so the selected tab joins it, as
the variant is meant to sit, with a Widget Info panel whose colour claims the gate holds —
`tab_bar`, `tab_foreground`, `tab_active_foreground`, `muted_foreground` (a disabled tab),
`tab_active` (the shape), `secondary` (the hover) and `border` (the separators), every one
a field the connector maps, so Folder is native in colour already. Its literals (paddings,
top padding, separator height, the 4 px gap, the one radius for corners and curves, the
height) and the hover's *button* token are `.not_themeable` notes, and so is the theme's
outward corner radius, which upstream's variant has no seam for (K14, K15): the native
folder look is `widgets::TabBar`'s. Not on the Basic page (K17): that page's elements are
shared by the three showcases (`docs/showcase-elements.toml`), neither egui nor iced has an
upstream variant to show beside it, and the theme's own tab shape reaches the Basic page's
existing tab row in all three instead.

Whether anything *chooses* the folder shape was decision D1, which the maintainer has
decided (K14).

### K9 — Scroll bounce: not modelled (settled)

Options: (a) record it as not modelled; (b) a theme value (`scroll.bounce`) with gpui
receivers (`scroll_bounce(bool)` on the seven widgets) and facts per platform. (a), for
four reasons. It is a platform behaviour that upstream already gets right on each OS:
on by build target on macOS, iOS and Android, off elsewhere (`GB8/scroll_bounce.rs:170-178`),
and always off under reduce motion (`:551`), which the connector forwards from the OS.
What (b) would add is only a foreign platform's behaviour under a foreign preset (a macOS
preset bouncing on Linux), which is not what a native theme is for. There is no user
setting to read: Apple documents elasticity as a property of each scroll view
(`NSScrollView.Elasticity`, `automatic` letting AppKit decide), and we found no documented
system setting for it. And neither egui nor iced has a receiver, so it would be a gpui-only
field. The decision is recorded in the README's *What gets mapped* and in
`docs/todo_gpui-full-theme.md`.

### K10 — The side panel keeps its width when the window resizes (settled by parity)

The report's D2 asked whether `ResizablePanel::fixed()` should replace the side panel's
width bookkeeping. It cannot replace it: `side_panel_width` and `rearrange` (`SC/app.rs:1905-1930`)
exist because a panel *leaving* the group (the side-panel toggle) would hand its width to
its neighbour, which `fixed()` does not address. What `fixed()` does address is a window
resize: without it, a resizable group scales every panel in proportion
(`GB8/resizable/mod.rs:395-440`), so the side panel would grow with the window (the code
suggests it, and the source predicts the plain case fails, spec §7; whether the panel's
`flex_none` prevents it is not measured), whereas the iced
and egui showcases keep theirs at `LEFT_PANEL_WIDTH` pixels. The parity rule settles
it: a test resizes the window and asserts the side panel's width; if it fails, the side
panel's `resizable_panel()` (`SC/app.rs:2118`) gains `.fixed()`; if it passes, nothing
changes and the commit body says so.

### K11 — `widgets::Slider`: an accessibility label, and its disabled state reported

The connector's theme-drawn `Slider` replaces gpui-component's when a native theme is
installed; it must announce at least what the replaced widget does. 0.8.0's
`gpui_component::slider::Slider` takes an `accessibility_label` (`GC8/slider.rs:126-130`)
and gpui-base's slider reports disabled. So `widgets::Slider::accessibility_label` (as its
`Checkbox` and `ProgressBar` have, `CN/src/widgets/checkbox.rs:586`,
`CN/src/widgets/progress.rs:150`), forwarded on the fallback path and written as the
node's label on the themed path (a `div` with `Role::Slider`). The disabled state: gpui-base's
`A11yDisabledExt` is `pub(crate)` (`GB8/a11y_disabled.rs:28`) and GPUI has no
`aria_disabled`, so options are (a) a wrapper element of the connector's own that sets the
node disabled through `Element::write_a11y_info`, public in both cores (`gpui-pre`
`src/element.rs:121`, `GF10/element.rs:122`), without `unsafe` or a panic path (gpui-base's
own, with its test-support variant, is 145 lines before its tests); (b) leave it, with an upstream request to make the extension public; (c) rebuild the
themed slider on gpui-base's. (a), with (b) filed so the copy can go later: (c) is a
rewrite for a one-flag gap, and (b) alone leaves the themed widget less accessible than the
one it replaces. The tests need a seam: `Slider::render` returns an `AnyElement`, which forwards
no accessibility method, so the themed root is built by a crate-private `themed_root` that
returns the typed wrapper, as `Checkbox::row` returns its gpui-base primitive for the
checkbox tests (spec §6), and the fallback is read through gpui-base's test-support
registration of its own slider (gpui-kit's `TestWindowExt::try_find`);
`Window::debug_a11y_tree_json()` is not relied on, since the window builds that tree only
while the platform reports assistive technology active, which the test platform never
does. The
connector's other themed controls are built on gpui-base primitives that report disabled
themselves. The showcase's sliders and number inputs, none of which
GPUI can associate with the text beside it, each get a label naming what it sets.

### K12 — The other new variants and options: shown, mapped, or recorded why not

The principle G1 set (L9 there): an *element* is shown; a behaviour or option with no theme
value is not, and is recorded. So, beyond K8–K11: DataTable's `fit_column` (double-click a
divider to fit) is behaviour an application's delegate implements, with no theme value —
not shown; `fixed_cols_border`, `select_on_modifier_click` and `focus_ring(bool)` likewise
(the Data page has no fixed columns; its focus ring shows on keyboard focus only in 0.8.0,
and the claim citing it is re-read); the public `ResizeHandle` is noted on the `[1.0]`
entry *gpui: the splitter renderer into the connector*, where it may serve; the completion
menu's new look stays excluded as completion popups were (G1 L9); the 1 px caret has no
model field (`input` states `caret_color` only) and no panel states a width; `syntect`,
the editor options, `SelectionFormat::SourceAcrossStyles`, `DockToggleEvent` (Dock is not
shown), `MessageScrollerState::logical_scroll_top`, `use_display_p3`,
`AppLifecyclePhase::Disconnected` and `hot-reload` name nothing we draw. `GPUI_FAST`
resolves the ROADMAP bullet "a gpui-fast marker constant", but the connector needs no check:
under form A it follows gpui-kit, and an assertion that its own feature equals
`GPUI_FAST` would reject the supported case of an application that turns on
`gpui-kit/gpui-fast` alone.

### K13 — Where G3 sits: between M9b's tasks, as one writer; M9a closes on its first green push

M9b's fix phase is the branch's writer. The controller's ruling
is one writer, G3 between M9b's tasks; a parallel G3 in a worktree was the alternative, and
is worse: both milestones write `Cargo.lock`, `CHANGELOG.md` and `docs/todo.md`, and the
branch's builds and gates are sequential anyway. G3 enters only at a task boundary, on a
clean tree, and hands the tree back at one. Tasks 1–3 are one unit: the bump turns the two
citation gates red, and they are green again only after the hand-read commit, so no M9b
commit may land between them (M9b's per-task gate, `scripts/check_release.sh --commit`,
runs the gpui gates too). To keep that window short, the hand-reading no gate needs (the
connector's own doc-comment citations, the version-stating prose) is a Task 3b after the
unit's push. The shared files are `Cargo.lock`, `.cargo/audit.toml`, `CHANGELOG.md`,
`docs/showcase-exceptions.toml` (all three showcases' work writes it; Task 2's set-aside
list therefore records content, not only line numbers), `docs/todo.md`, `docs/todo_code-review-findings.md`, `.github/workflows/ci.yml` and
`tools/showcase-support/src/capture.rs` (a comment G3 re-cites; M9b's T2 moved iced's
capture onto that crate, and its fix round still edits it); one writer at a time is what keeps
them consistent. The first push after the unit (the maintainer's approval) is the run that
turns M9a's red job green (run 39490500851, *Test gpui connector*, the GPUI Fast step), and
M9a's Step 8 closes on it; the `rustls-webpki` advisory of the same run was fixed by M9b
T1's lock commit. D1's tasks (11–15) enter the same way, after the unit (K16).

### K14 — D1: `windows-11`'s tabs take WinUI 3's folder shape, as one theme value (the maintainer's decision)

The maintainer decided D1 (2026-10-21): *"Yes, in G3."* WinUI 3's `TabView` is a folder bar
and no other platform's is (§1.3), so the folder shape becomes `windows-11`'s native tab
look, modelled as theme data: a model field with a facts row, `windows-11`'s value, and
receivers in all three connectors and their showcases. What the model carries:

- **(a) A shape enum** (`tab.shape = "folder"`), the curves taking the top radius. One
  field, but untrue for the one platform that has the shape: WinUI's curves are 4, its top
  corners 8 (§1.3). Drawing WinUI's 4 would need the connectors to hard-code it, a theme
  value outside the theme.
- **(b) A boolean** (`tab.folder_enabled`, as `expander.frame_enabled`): (a)'s problem
  unchanged. Taking the curves from `defaults.border.corner_radius` instead would draw 4 on
  `windows-11` (`windows-11.toml:70`, WinUI's `ControlCornerRadius`) with no new length,
  but only by coincidence: MUX hard-codes the 4 in the markup, independent of any corner
  radius resource (`TabViewItem.cpp:106`; the arcs' fixed `Width="4" Height="4"`,
  `TabView.xaml:547-548`), and a theme with another control radius would curve wrong.
- **(c) An enum or boolean plus a curve radius:** true, but the switch is redundant with the
  radius being stated, and it admits states with no meaning (a folder without a radius, a
  radius without the folder) that validation would have to reject.
- **(d) One optional length, the radius of the selected tab's outward bottom corners**
  (`tab.active_outward_corner_radius_px`): `None` is today's shape, a stated radius the
  folder shape. Everything else WinUI's selected tab needs is in the model already: its top
  corners (`tab.border.corner_radius`, 8 for `windows-11`), its fill (`tab.active_background`),
  its outline's colour and width (`tab.border.color`, `CardStrokeColorDefault`, the brush of
  WinUI's arcs too; `tab.border.line_width` 1). The one rule the shape adds to existing
  fields is WinUI's own: the outline has no bottom edge and follows the curves instead
  (`1,1,1,0` and the arcs, §1.3).
- **(e) (d) plus the separator:** its colour (`DividerStrokeColorDefault`, now sourced,
  §1.3), its width (1) and inset (8 above and below), and when it hides (the selected tab,
  its left neighbour, the hovered tab). True values, but a second concept, a divider between
  tabs, independent of the shape (a bar could have either without the other); it adds a
  colour leaf (an iced contract row and `contract-leaves.toml` entries, an egui mapping row,
  a gpui `TabLook` field), two lengths and per-tab hiding in three connectors, about as much
  as the shape itself. Leaving it out states nothing false: the facts already record the
  separator as one "which no `tab` field models" (`docs/platform-facts.md:1360`), and
  today's `windows-11` tabs have none either.
- **(f) (d) plus the bar's bottom rule** (WinUI's `BottomBorderLine`): no new field, since
  its brush is `tab.border.color`'s, but it gives `tab.border`, the selected tab's outline, a
  second meaning (a rule under the whole bar) and collides with the showcases' own rule under
  the page tabs, which the gpui showcase states as `separator.*` "the model states no
  tab-bar rule of its own" (`SC/info/layout.rs:809`).

**(d)**, the smallest model that is true and that every connector can draw (K15). (e) and
(f) go to a `[1.0]` entry in `docs/todo.md` with §1.3's citations, so a later release can
add them without reopening the shape.

The field: `TabTheme::active_outward_corner_radius`, serialised `active_outward_corner_radius_px`
(the `_px` convention), a `soft_option` with `check = "non_negative"`, as `tab.item_gap`
is; `active_` because it belongs to the selected tab, as `active_background` and the
`active_indicator_*` fields do; "outward corner" because it is a corner radius
(`border.corner_radius`'s word) of corners that curve out instead of in. Not a field of
`tab.border`: `Border` is the one structure every widget's border shares
(`docs/property-registry.toml`, `[_structures.Border]`), so a tab-only field there would
appear, unstated, in every widget's border and in every connector's border tables. A stated
radius of 0 draws today's shape. Validation rejects a negative or non-finite one
(`check_non_negative`, `native-theme/src/resolve/validate_helpers.rs:357-371`, which the
derive applies to a `soft_option` where it is stated); a connector handed one anyway (a
resolved theme's fields are public) treats it as it treats its other lengths: gpui's
`TabLook::of` is `None` for any non-finite length, so the bar is gpui-component's
(`CN/src/widgets/tab_bar.rs:93-95`, `side`), and iced's and egui's draw no flare, as iced's
`tab_indicator_strip` draws no line for a width that is not positive and finite.

Values, every one from §1.3's sources, none invented: Windows **4** (`TabViewItem.cpp:106`,
`:98-123`; `TabView.xaml:547-548`, `:551`), so `windows-11` states `4.0` in both variants;
KDE none (Breeze's frame is `CornersTop`), GNOME none (the notebook's tabs are square,
AdwTabBar's rounded on every corner), Material none (`corner-none`): each cell states the
shape its platform documents, with that citation, and the presets leave the field unstated;
macOS ❓ (the bezel's shape is not published), unstated. The community presets cite no
source for any tab shape and leave it unstated, as they leave every documented size
(`native-theme/src/presets/documented_sizes.rs`'s header). A live Windows theme keeps the
preset's 4: the pipeline merges the reader's output over the platform preset
(`native-theme/src/pipeline.rs:70-77`), and the Windows reader writes no `tab` field
(`native-theme/src/windows.rs`).

**`tab.active_background` is corrected in G3 (Task 15).** The shape is painted in it, and
with the preset's `#f3f3f3` / `#202020`, the window colour the bar and the unselected tabs
also show (§1.3), the selected tab and its curves would be invisible: only the faint arcs
and the missing bottom edge would show. The value is wrong by the platform's own source:
WinUI fills the selected tab with `SolidBackgroundFillColorTertiary`, `#F9F9F9` /
`#282828` (§1.3). Whether `windows-11` states WinUI's brushes or Win32's is settled: since
M2b the preset states what WinUI 3 draws; `input.background_color` was moved to its WinUI
brush on that ground (M2b fix round 2) and `button.checked_background` to
`AccentFillColorDefault` (9284f52b); M4 left the reader writing only what the OS reports
while "the preset states the rest"; and the entry's one remaining open item is this one
(its M4 Step 8 line: "Still open: `tab.active_background`"). So Task 15 writes `#f9f9f9` /
`#282828` with their citations, rewrites the facts row's Windows `active_background` cell
(`docs/platform-facts.md:1347`, "← `defaults.background_color`", which WinUI's resources
contradict) to the brush, and closes the entry. What the `windows-11` captures then show
is WinUI's folder: the selected tab and its curves `#F9F9F9` over the `#F3F3F3` bar (dark
`#282828` over `#202020`), its outline running down into the arcs, no bottom edge, no WinUI
bar rule and no separators (K14 (e), (f)); under the page tabs, the showcases' own
`separator.*` rule stays.

### K15 — How each connector draws it: the same shape in all three, painted after the tabs

What is drawn, in every connector, where the radius `r` is stated (spec §12.2): the selected
tab as today but without a bottom edge to its outline; at each of its two bottom corners a
flare, filled `tab.active_background`: an `r` × `r` square less the quarter disc of radius
`r` about the square's upper outer corner; where `tab.border.line_width` `w` is positive,
the outline continued down the flare's curve as a band `w` wide on the disc's side of it,
in `tab.border.color`, to the bar's bottom edge. The square sits `w` in from the tab's
outer edge, so the disc's centre is `r − w` outside it: our tab's outer bounds include its
border, and WinUI's side edge lies just outside its fill (§1.3), so this is WinUI's
geometry restated. The band's top end then covers the side edge's own column, continuing
it, and the fill paints over the side edge's foot below that, as WinUI's gradient leaves
it transparent there. One continuous line, the band tapering out to the bar's edge, is our
choice where WinUI's markup ends the arc at the neighbours' rule and may leave one row
between edge and arc (§1.3): the model states no bar rule (K14 (f)). The flares lie over a
hovered neighbour's fill, where WinUI's hover lies over its right flare: ours keeps the
selected tab's shape whole. Room for the flares at both ends of the row, `r` inside its clip
(they reach `r − w` past the tab; `r`, so the room does not depend on the outline's width),
as upstream's Folder pads its row by its overhang (`GC8/tab/tab_bar.rs:686-687`).

**When the flares are painted.** A flare lies over its neighbour's bottom corner. WinUI paints
the selected item's path under its right neighbour and loses nothing, because an unselected
tab is transparent there (§1.3); here an unselected tab is filled `tab.background_color`,
for `windows-11` the opaque window colour (`windows-11.toml:24`, inherited), so whatever
paints a neighbour after the flare covers it. Options: (a) paint the flares with the selected
tab and round each neighbour's facing bottom corner away — the neighbour's rounded fill
still covers the outline band, which lies on the disc's side of the curve; (b) paint each
flare as part of the neighbouring tab, found by index — wrong where that tab has wrapped to
the next line (gpui's `TabBar::wrap`, egui's `horizontal_wrapped`), and impossible in a gap
between tabs, which no element owns in iced; (c) paint both flares after every tab of the row, from
the selected tab's laid-out bounds — one rule for every layout, gap and wrap. **(c).**

**With what.** Every toolkit fills and strokes rounded rectangles and clips to a rectangle, so
each flare is two annuli clipped to its square: about the disc's centre, one from `r` out to
`r√2 + 1` (past the square's far corner) in the fill colour — inside the square that is the
square less the disc — and, where `w > 0`, one from `r − w` to `r` in the outline colour.
Paths are the alternative and worse in each: gpui's are not readable by the tests (no
`painted_paths`; the quads are, through K5's `scene_quads`), iced's canvas frame clips to
its own bounds, so it cannot draw beside the tab (§1.5), egui fills convex polygons only
(§1.5). The missing bottom edge: gpui sets the selected tab's bottom border to 0 and
adds `w` to its bottom padding, so the label stays where it is; iced's `Border` and egui's
`Button` stroke take one width for all sides, so there a strip `tab.active_background`
covers the bottom edge between the side edges, painted with the flares (and not where a
bottom-side indicator, which covers that edge itself, is stated; the order is tabs and
their indicators, then the strip, then the flares, in all three, spec §12.2). The result is
the same shape in all three, to each toolkit's anti-aliasing and pixel clipping (iced and
egui clip to whole physical pixels, so at a fractional scale up to a pixel of fill can show
beside a square, spec §12.2).

- **gpui — `widgets::TabBar` draws it, not upstream's Folder.** Upstream's Folder cannot be
  the receiver: (i) the themed path does not use gpui-component's `TabBar` at all (§1.5);
  (ii) Folder draws one radius for corners and curves, `Theme::radius` or `radius_lg`
  capped at a third of the height (§1.3), which on `windows-11` is 4 and 4 at the Small size
  (`defaults.border.corner_radius`) or 8 and 8 at Medium (`corner_radius_lg`,
  `windows-11.toml:70-71`), never WinUI's 8 and 4; (iii) its paddings, heights, gap, its
  `secondary` hover and its separators in `border` are literals or other tokens, so the
  theme's tab sizes, padding, font, gap, hover, outline and indicator would all be
  ignored. So the 8/4 difference is kept: the top corners come from `tab.border.corner_radius`,
  the curves from the new field. `TabLook` gains `outward_radius` (it is `#[non_exhaustive]`,
  `CN/src/widgets/tab_bar.rs:25`); the selected tab's bottom inset is 0 wherever the bar
  measures or decorates it (it has no bottom border then); the selected tab records its
  bounds while the frame is prepainted, through a `canvas` as `part_bounds` records a part's (`CN/src/widgets/mod.rs:145-156`),
  and a `canvas` after the tabs in the row paints the flares in that frame's paint phase,
  within the row's clip (`Window::with_content_mask`, `paint_quad` and `quad`: gpui-pre
  `src/window.rs:4163`, `:4684`, `:7813`; gpui-fast 0.1.10 `:4238`, `:4760`, `:7919`). Both
  cores prepaint the root before they paint it (gpui-pre `src/window.rs:3710`, `:3746`;
  gpui-fast 0.1.10 `:3767`, `:3807`); GPUI Fast's reuse of an unchanged subtree is what a
  test must still show does not drop the flares. Without a native theme the bar is gpui-component's, which no theme value reaches, as
  today.
- **iced — a connector widget, `tab_flares`.** iced cannot draw outside a widget's bounds
  except in a widget of its own, and the flares lie outside the selected tab's. So
  `native_theme_iced::tab_flares(resolved, tabs, selected)` wraps any element whose layout
  holds the tabs (the room for the flares goes outside it, a padded container, since iced_aw's
  bars take no padding of their own, spec §12.4), draws it, then draws the flares and the bottom-edge strip about the node
  at the layout path `selected` (`&[i]` for a `row` of tabs or an iced_aw `TabBar`, `&[0, i]` for an iced_aw
  `Tabs` with its bar on top, whose layout is `[bar, content]`, iced_aw 0.14.1
  `src/widget/tabs.rs:386-394`), with the core renderer's `fill_quad` and `with_layer`
  only, so the `widgets` feature needs no `canvas`. iced's layers do not nest their clips
  (`start_layer` pushes its bounds as they are, `iced_graphics-0.14.0` `src/layer.rs:85-98`,
  `iced_wgpu-0.14.0` `src/lib.rs:674-676`), so each square is cut by the `viewport` the
  widget is drawn with — a scrollable passes its visible area (`iced_widget-0.14.2`
  `src/scrollable.rs:1187-1205`) — and a flare outside it is not drawn; and the annulus
  quads do not snap to the pixel grid (`Quad`'s `snap` defaults to the `crisp` feature,
  `iced_core-0.14.0` `src/renderer.rs:89-100`, which iced 0.14 turns on by default). gpui
  and egui need neither: their masks intersect with the enclosing one. It is the counterpart of
  `tab_indicator_line`, which lays the line over a tab's place for the same reason. Two
  contract rows (`crate::tab_flares` fill and outline) as `tab_indicator_line` has one
  (`connectors/native-theme-iced/src/contract/rows.rs:172-185`). iced_aw's limit stays:
  the selected tab under the pointer is drawn as `Hovered`
  (`connectors/native-theme-iced/src/styles/aw.rs:165`), with no outline and square
  corners (`:234-237`), while its flares, their arcs and the strip stay; the routes say so.
- **egui — the application draws it, as it draws the indicator.** The connector ships no
  widget (§1.5); its `mapping.toml` gains the row
  `["tab.active_outward_corner_radius"]`, `derived`, `tested_by = "T18(a)"` (an application
  reads it from `ThemeAtlas::resolved_for`), and the showcase's `tab_bar` paints the flares
  after its tabs through a `Painter` clipped to each flare's square.
- **Connector parity:** `tab_flares` is iced's alone, so `scripts/connector_parity_exceptions.toml`
  gains `absent` entries for gpui and egui beside `tab_indicator`'s (`:445-455`), for the same
  reasons.

The other gates a new leaf reaches (each named in spec §12): the registry's inventory test,
`scripts/check_platform_facts_rows.py`, the documented-sizes gate, egui's
`every_leaf_has_exactly_one_row_and_every_row_is_well_formed` (and its leaf count, 515,
`connectors/native-theme-egui/src/mapping_tests.rs:239`) with the generated `mapping.md`,
`scripts/check_connector_parity.py`, the iced contract's tests and `contract-leaves.toml`,
and the three showcases' route tests over `docs/showcase-elements.toml`. Two reach nothing:
`docs/showcase-exceptions.toml`, since the parity comparison captures no preset that states
the radius (§1.5), and `scripts/check_widget_coverage.py`, since no upstream widget is new.
`contract-leaves.toml` exports colour leaves only, so the new length is not in it; the two
new rows move `tab.active_background` and `tab.border.color` only where a bundled preset
states the radius, so the export changes when `windows-11` does (Task 15), not before.

*Added 2026-10-22 (G3 T15), from the G3 T13 and T14 reviews:* iced and egui round a clip's
edges to the nearest physical pixel (spec §12.2's G3 T15 line), so "up to a pixel of fill"
above is at most half a physical pixel in those two; gpui's outward mask keeps up to a pixel.
The selected tab's iced_aw citations above moved with Task 13's doc in
`connectors/native-theme-iced/src/styles/aw.rs`: `styles::aw::tab_bar`, given in §1.5 as
`aw.rs:204`, is at `:211`, and "no outline and square corners", given as `:234-237`, at
`:241-244` (both since `2e7dffcd`). The room outside an iced_aw bar is filled
`tab.bar_background`, as the bar is (spec §12.4's G3 T15 line).

### K16 — Where the D1 work runs: five G3 tasks after the unit, each one writer at an M9b boundary

M9b is mid-way (on 2026-10-21 T1–T6 of 19 done, T6's fix round 1 open with T5's two tab
residuals, T7 in progress, `.superpowers/sdd/todo_code-review-plan/progress.md`; refreshed at
approval) and writes the iced showcase and,
through its fixes, the iced connector; M9c has not started (its briefs are written,
`.superpowers/sdd/todo_code-review-plan/m9c-T*-brief.md`, and none names a tab bar).
Options:

- **(a) Inside the unit (Tasks 1–3):** lengthens the window in which M9a's CI stays red and
  M9b is held, which the unit exists to keep short, and puts a model change and a second
  citation renumbering (K18) beside the upstream one. No.
- **(b) Later G3 tasks**, each at an M9b task boundary on a clean tree, one writer: the core
  first (Task 11), then the three receivers in any order (12 gpui, 13 iced, 14 egui), then
  the switch-on (Task 15: `windows-11`'s value, the documented-sizes rows, the shared list).
  Pros: it is what "in G3" says; one design, one drawing contract and one set of test
  numbers, carried into three toolkits by one milestone; egui has no other writer before
  M9c; G3 already writes M9b's shared files between its tasks. Cons: G3 writes files M9b
  owns (the iced showcase, and the connector's `src/lib.rs`, `src/contract/rows.rs`,
  `src/styles/aw.rs`'s doc and `contract-leaves.toml`), and M9b's later briefs must find their
  regions by content (they do: each says "locate every region by content"). Open M9b and
  M9c tasks on the same code (their briefs, read 2026-10-21): M9b's T8 the iced
  `basic_column_count` (the section gap, `m9b-T8-brief.md:19-21`), T10 the Extra page's
  iced_aw `TabBar` (`m9b-T10-brief.md:27-30`), T19 the page list's order and labels that
  `page_tabs` draws and the shared list's header (`m9b-T19-brief.md:13-21`), each of which
  Task 13 or 15 also edits; M9c's T7 the egui `column_count` (the section gap,
  `m9c-T7-brief.md:13`, `:21`), which Task 14 also edits. No other M9b or M9c brief names
  a tab bar, a column count, the shared list's tab entries, `contract-leaves.toml`, the
  parity exceptions, the egui mapping rows D1 adds, or an edit to the data files D1 writes
  (some cite lines of `windows-11.toml`, `mapping.md` or `inheritance-rules.toml`, for other
  widgets, by content). Whichever comes second
  re-reads the other's work.
- **(c) Hand the iced part into M9b and the egui part into M9c:** each owner knows its file's
  gates; but D1 then closes only at M9c's end, M9c's written plan must be reopened, G3 cannot
  turn the value on until both have landed, and the maintainer's decision names G3.

**(b).** The cost if it is wrong — an M9b task in flight on the same functions — is bounded
by conditions on the tasks that share code: Task 13 starts only at a boundary where none of
M9b's T5 (its review and fix rounds), T8, T10 and T19 is open — each either done or not
begun — nor an M9b fix round carrying tab items (T5's re-review carried two, the inspector
tab never varied and a test restating `tab_open`, to "T6's fix round if any, else Step 8's
final fix", `progress.md`); Task 14 likewise for M9c's T7, should M9c have started;
Task 15 for T19, for any M9b task open on the iced `src/contract.rs` test it re-words, and
for any M9c task open on the egui `src/style/variants.rs` comment it re-points (spec §12.6). **Why the
switch-on is last:** until all three draw the shape, a bundled preset stating it would be
theme data a connector ignores; so Tasks 11–14 test with the radius set on a resolved
`windows-11` theme, and only Task 15 writes it into the preset. Between Tasks 11 and 15 a
user theme stating the radius is drawn by the connectors done so far: an unreleased branch,
for the length of five tasks. **Why the shared list is last:** the three showcases' route
tests read it (§1.5), so a leaf listed before the last showcase routes it turns that one's
test red; each of Tasks 12–14 adds its showcase's route ahead (an unlisted route is unused,
and no test rejects one), and Task 15 lists the leaf.

*Added 2026-10-22 (G3 T15), correcting "an unlisted route is unused, and no test rejects one"
above for gpui:* gpui's `every_how_entry_is_shown` (`connectors/native-theme-gpui/src/showcase.rs`)
fails on a `HOW` entry that no listed element's leaf takes, so the gpui showcase's seven
routes for `tab.active_outward_corner_radius_px` were not committed in Task 12 and land in
Task 15 with the listing (spec §12.3's G3 T12 line). iced's and egui's route tests do not
reject an unused route (iced's `iced_route` is a `match` the tests walk over the listed
leaves; egui's lines come from its `mapping.toml` row), so Tasks 13 and 14 committed theirs
ahead, as planned.

### K17 — The Basic page and `docs/showcase-elements.toml`: the existing tab elements take the shape; upstream's Folder stays off

K8 kept Folder off the Basic page because egui and iced lacked the variant. Re-decided with
D1:

- **(a) Upstream's Folder on the Basic page:** it is gpui-component's variant, not the
  theme's look; egui and iced have nothing to show beside it, so K8's reason stands.
- **(b) A new shared element, "folder tabs", in all three:** it would draw what the existing
  tab row draws — the theme decides the shape, not the page — so under `windows-11` the two
  rows would be the same, and under every other preset too.
- **(c) The existing tab elements draw whatever shape the theme states:** the Basic tabs,
  the page tabs and the inspector tabs are drawn from `tab.*` in all three showcases
  already (§1.5), so the new leaf joins their lists.

**(c).** The list gains the leaf on the three selected tabs (`basic.tabs.one`,
`chrome.page_tabs.basic`, `chrome.side_panel.inspector_tabs.widget`, after
`tab.active_indicator_side`, registry order), on the three bars (their tabs inset by the
radius, after `tab.item_gap_px`) and on `basic.page`, whose column rule makes room for both
flares: `2 * tab.min_width + tab.item_gap + 2 * tab.active_outward_corner_radius`, the rule
in the list's header (`:619`) and in the three column counts (gpui's and iced's `basic_column_count`, egui's
`column_count`). No sample point is
added: the comparator captures no preset that states the radius (§1.5), so a sample would
compare nothing; each showcase's own test holds the shape instead, with the same numbers in
all three (spec §12).

### K18 — Breaking change, CHANGELOG, READMEs, docs; the core files' renumbering

The release is 0.7.0, breaking already. `TabTheme` is not `#[non_exhaustive]`, so a field
added to it breaks a struct literal outside the crate: a *Breaking Changes* line, as the
existing ones for `TooltipTheme`, `DialogTheme`, `ListTheme` and `SwitchTheme` (CHANGELOG
`[Unreleased]`); `cargo semver-checks` reports it, an expected soft warning at the close. The
resolved `ResolvedTabTheme` is `#[non_exhaustive]` (generated, `native-theme-derive/src/gen_structs.rs:26`),
and so is gpui's `TabLook`: no break there; iced's `tab_flares` is an addition. The new option
reaches: *Added* lines (the field and `windows-11`'s value; gpui's `widgets::TabBar`; iced's
`tab_flares`; the showcases' tabs); the two README sentences that list the optional sizes
(`README.md:41`, `native-theme/README.md:47`); the gpui README's `TabBar` row
(`CN/README.md:443`); the iced README's widget lists (`connectors/native-theme-iced/README.md:311`,
`:466-468`); egui's generated `src/mapping.md`; `docs/platform-facts.md` (the new row, and
the Windows `active_background` cell, K14), `docs/property-registry.toml`,
`docs/inheritance-rules.toml`; a *Fixed* line for `windows-11`'s selected fill; the doc comments of the field,
`TabLook`, `widgets::TabBar` and `tab_flares`; the two documents that list the sizes the
documented-sizes gate holds (`native-theme/src/presets/README.md:25-29`,
`docs/new-os-version-guide.md:231-237`); `docs/todo.md` (the `[1.0]` entry of K14; the
WinUI-brushes entry closing on `tab.active_background`, K14); and
`docs/todo_gpui-full-theme.md`'s 0.8.0 note.

**The core files' line citations, weighed again.** The facts row, the inheritance-rules
entry and the model field each insert lines into a file the repository cites by line (the
registry key and the egui mapping row do too, but nothing cites those files past their
insertion points, spec §12.1).
Inserted after `:1365` with nothing else changed, the facts row moves 540
`platform-facts.md:N` first numbers past §2.11's table, their continuations, the
documented-sizes gate's 250 bare numbers past `:1365`, and every bare `(:N)` written after
another file's numbers (counted at `a172d3dd`–`2e80f87e`, outside `docs/archive/`,
`docs/evidence/`, `.superpowers/` and the ledger). Four check passes after D1 (8 to 11) each
found a new way a rule-driven renumbering of that set goes wrong silently (probe numbers,
bare ranges across a line break, facts numbers after another file's, a file-name definition
held only in a scratchpad). Under the overcomplication clause:

- **(a) The rule set, patched again** (M8 T17's chain rule with its stops and continuations,
  a wider whole-check scan, fixtures). Pros: no edit beyond the row. Cons: every pass has
  found another miss; the rules decide hundreds of numbers by heuristics, and a miss lands
  where no test reads it (comments). Its correctness rests on the rules being complete, which
  four passes have shown they were not.
- **(b) A placement that moves as few lines as possible, every candidate listed and decided
  by hand.** The row must be in §2.11's table (`check_platform_facts_rows.py`), so it goes
  last there; removing the blank line `:1401` (between §2.14's heading and its table, which
  then join as §2.11's and §2.12's already do, `:1343-1344`, `:1366-1367`) brings every line
  from `:1402` back to its number. Only `:1366`–`:1400` move: 157 candidate numbers in 22
  files at `2e80f87e` (36 of them the documented-sizes gate's own, which its tests read),
  123 to move and 34 to keep (spec Appendix B). Each is found by one pattern over every
  number in the scope, so none is missed by construction, and decided by a person reading
  its line; the reviewer re-runs the pattern and reads the decisions. The inheritance rules
  (the blank `:464` after the array removed: lines `:455`–`:463` move; one stale range,
  re-pointed by content) and the model file (no blank line near) are found by a scan of
  every number in range on a line that names the file and the three lines after it, with
  one exact pattern (spec §12.1): for the model file, at `fa90c6c0`, 13 numbers on 8 lines
  past the insertion (a continuation among them, `:571`; six citations already stale and
  re-pointed by content; one other file's number), and the stale ones below it re-pointed
  too; for the registry and the egui mapping, which gain a line as well, nothing to move. Cons: two
  blank lines go; about 160 decisions by hand, of which 123 are a plain "+1" on a facts
  anchor.
- **(c) A content gate**: each `platform-facts.md:N` citation must land on a row that names
  the leaf it is cited for. Existing gates already do this for two files: `documented_sizes.rs`
  (its `ROWS` and `FIELD_ROWS` numbers and `facts_line(N)` calls, held by
  `native_themes_state_documented_sizes`, `native_themes_state_documented_fields` and
  `every_field_citation_names_its_platform_facts_row`) and
  `native-theme/tests/platform_facts_xref.rs` (its `line(N)` calls). No gate reads the presets'
  comments, native-theme's and the connectors' doc comments, `docs/inheritance-rules.toml` or
  `docs/todo.md`. A gate for them needs to know which leaf a comment is cited for, which a
  comment does not state in a fixed form (a citation names its row's neighbour, a section, or
  another platform's cell on purpose), so it would be a new heuristic tool of its own. Kept
  where it exists, as gate (G4); not built.
- **(d) Defer the row to a later facts edit:** not available; `check_platform_facts_rows.py`
  fails a registry field without a row.

**(b).** It is correct by construction for the facts window (every number in it is listed)
and by a scan of every number near each mention for the other two files (nothing outside
the windows moves), it needs no chain rule and no new script, it is checked by one
pattern and a reading, and it is about a fifth of (a)'s size. Its cost is two removed blank
lines and the hand decisions, which are listed in advance at `2e80f87e` (spec Appendix B)
and only re-checked at the task's base. Task 2's renumbering (K3) is untouched: it maps
upstream citations, a different set, and (b) is no second run of it (spec §3.3). The scope
includes `docs/todo.md`, as M8 T17's did: a digits-only change keeps the entries' text true
and is not a rewrite.

*Added 2026-10-21 (K19):* the *Breaking Changes* lines gain one more, `widgets::PartBounds`,
whose observer now takes the window.

### K19 — Widget Info under GPUI Fast's partial frames: an element-state presence token; `PartBounds` gets the window

*Added 2026-10-21 as a Task 1 deviation, adopted by the controller's ruling (the design and its
review, `g3-T1-partial-frames.md` and `g3-T1-partial-frames-review.md`, in the review's SDD
directory); the maintainer accepted the `PartBounds` break the same day, as the controller
relays. The approved text above is unchanged.*

**What Task 1 measured.** After the bump, eight showcase tests failed on GPUI Fast 0.1.10
only, all in `draw`'s check that no frame drew two info targets under one id
(`a_selected_list_row_and_an_unselected_one_show_different_infos`,
`a_selected_list_or_tree_rows_info_names_the_selection_text_colour`,
`a_table_row_reports_what_the_table_paints_on_it`,
`escape_clears_the_query_then_closes_the_palette`, `the_menus_answer_the_keyboard`,
`the_palette_reports_itself`, `the_palette_installs_a_preset`, `the_palette_switches_page`).
Traced with render and record counters in `the_menus_answer_the_keyboard`: on gpui-fast 0.1.7
the showcase's `epoch_marker` ran in every one of its 31 frames, while 0.1.10 draws partial
frames. A keystroke's or a mouse move's
frame built `MenuBar` and `Inspector`, or the showcase's `ListRow` views, and drew `Showcase`
from the last frame. So `epoch_marker`, `Showcase`'s first child, did not run, and the rebuilt
view's targets recorded a second time in one epoch. The next frame built `Showcase` and drew
`Inspector` from the last frame. Its targets then kept an older epoch and read as not drawn:
their hovers were dropped, and the choice and the layout dump lost them. That second half is a
run-time defect on GPUI Fast, not a test artefact. Which 0.1.7→0.1.10 change makes these views
qualify for reuse is not pinned down; 0.1.7 already has the same splice and reuse code.

**How gpui-fast 0.1.10 draws a view from the last frame** (read; `GF10` paths):
- Every entity view is a retained subtree (`fast/retained.rs:1-18`). A view that is not dirty
  is drawn from its record; one dirty only through a nested view is spliced, its nested dirty
  views built (`fast/splice.rs:1-21`); otherwise it is built (`retained.rs:1492-1532`). A
  notify marks the view and its ancestors dirty (`window.rs:2294-2306`).
- A view drawn from the last frame runs none of its prepaint or paint code. Its records are
  copied, the element states it used among them (`window.rs:4079-4084`, `:4163-4168`;
  `splice.rs:779-784`), and a composited scroll layer's likewise (`fast/layers/reuse.rs:101-106`,
  `:204-209`).
- An element state lives while each frame uses or copies it, and goes with the first frame
  that does not (`window.rs:1221-1231`, `:3561-3567`; gpui-pre `window.rs:1195-1202`).
- gpui has no per-frame or per-view hook.

**The options.**
1. *Flag a duplicate only where the bounds differ.* It masks a collision at equal bounds, and
   fixes neither the stale "drawn" of a removed target nor the reused view read as not drawn.
   Rejected.
2. *Force full frames* (`window.refresh()`, at run time or in the tests only). At run time it
   defeats GPUI Fast. In the tests only, they would stop exercising real GPUI Fast frames, and
   the run-time defect would stay hidden. Rejected.
3. *Per-view epochs from `current_view()`.* A reused view runs no code, so its presence is
   never signalled. A view removed under a rebuilt parent is not seen. Rows drawn inside
   gpui-component's List, Table and palette entities have no marker of ours. And
   `current_view()` unwraps an empty stack. Rejected.
4. *Defer, the eight red until a later task.* Widget Info stays broken on a supported backend,
   the run-time half untested, and the plan's expected red is no longer true. Rejected.
5. *An element-state presence token.* Each recorder keeps an `Rc<Presence>` in gpui element
   state, and its record holds a `Weak`: the record stands for a drawn element while the
   `Weak` is alive. It rides on what gpui itself carries forward. It needs no hook and no
   dependency, and it gives the same answer on both cores: gpui-pre draws only a `.cached()`
   view from the last frame and copies its element states too (gpui-pre `view.rs:530-546`,
   `window.rs:3996-4013`). **Chosen.**
6. *Hitbox-based "drawn".* It covers hover only, not the layout dump, and there is no public
   query. Rejected.
7. *A per-frame hook through `on_next_frame` or `request_animation_frame`.* It keeps the
   window drawing and still gives no presence. Rejected.
8. *An upstream hook.* Not needed.

**Sub-decisions of 5.**
- *Where the token lives:* `with_global_id(key)` and `with_element_state::<Rc<Presence>>`,
  under the recorder's id path and the record's id. A state is keyed by its type too
  (`window.rs:7420-7433`), so the downcast's `unwrap` (`:4495`) cannot fail, and the `expect`
  (`:4497`) cannot either, since a box is only put back holding a state (`:4501`,
  `:4513-4516`). `use_keyed_state` would need an entity per target; a token held in a closure
  dies with the frame's arena.
- *The end of a frame:* a `Window::defer` scheduled by the frame's first record
  (`window.rs:2645-2650`; it runs as the drawing update flushes its effects, `app.rs:1351-1366`,
  `:2080-2105`). A marker's paint would not do: a partial frame skips it.
- *What the end of a frame updates:* the registry only when the frame changed what it decides.
  An update outside drawing has every view that recorded into it built again under a notified
  view (`fast/dependencies.rs:757-797`). The design agent measured it: updating at every frame
  end built a reused view again (3 renders against 2). So the frame bookkeeping (`open`, and in
  the tests `painted`) is kept in `Cell`s and set through reads.
- *The tests' duplicate check:* draws are counted by the alternation of prepaint and paint (a
  draw prepaints everything before it paints anything, `window.rs:3757-3813`). A fresh record
  whose replaced holder is still drawn at the frame's end is flagged too. Frame ends as the
  boundary gave false duplicates under gpui-kit's test helpers, which draw several times in
  one update. Its assumption: every draw that records paints a target or a `record` box. A
  draw whose only records are `.listed()` boxes or part observers would give a false
  duplicate, a loud one, never a miss.
- *Parts:* each part gets its own token, since a part comes and goes while its control stays
  (`Part::Tab(ix)` with the tab count, `StartThumb` on a range slider only, `Label` only with
  a label). So `widgets::PartBounds` becomes `Fn(Part, Bounds<Pixels>, &mut Window, &mut App)`,
  gpui's own prepaint-callback shape. The alternatives weighed:
  - a connector-held token handed over as an opaque handle: it breaks the signature just the
    same, and adds a public type and a liveness contract to keep;
  - a token passed through `&mut App`: a hidden side channel that depends on order;
  - a second, additive observer type: two APIs for one purpose, in a pre-1.0 crate whose
    0.7.0 breaks already;
  - parts tied to the control's own token: wrong for the parts that come and go.

  `PartBounds` shipped in 0.6.0, so its change is a *Breaking Changes* line. 0.7.0 breaks
  already, and nothing promises compatibility before 1.0.
- *Hover pruning* (the review's I1): a hovered target that a frame recording nothing took
  away gets no frame end, so it stays in `hovered`. Drawn again as a new element away from the
  pointer, it would be chosen. So `record_target` drops the id from `hovered` when the record
  it replaces stands for an element no longer drawn. What remains is lateness: after such a
  frame, the choice is revisited at the next record or at a hover event of that id. It is
  never wrong meanwhile, since an element not drawn is never chosen.
- *A limitation:* inside a scroll area that GPUI Fast composites while it scrolls (after two
  scrolled frames, `fast/layers/policy.rs:41`, `:286-287`), the children's prepaint is skipped
  and the content is drawn moved by the scroll (`fast/layers/paint.rs:233-235`,
  `layers/reuse.rs:39`). The records there keep the bounds of the layer's last repaint. Hovers
  and the choice hold, since gpui moves the hitboxes and the choice compares areas only; the
  layout dump does not. No gate reads a GPUI Fast dump after a scroll: the dump and screenshot
  scripts build the gpui showcase on gpui-pre, and Task 9 dumps at rest. A `[1.0]`
  `docs/todo.md` entry records it, with the fix if one is ever needed.
- *A cost, not a defect:* every record is an update made while drawing, which GPUI Fast counts
  as a write. So a retained subtree that read the registry and did not write it itself is
  built again in the next frame (`dependencies.rs:757-797`, `:646-653`), as a view that
  records before a view nested in it records is. Task 9 measures how much reuse the showcase
  keeps.

The cost if this is wrong: one more rewrite of the registry inside the showcase, and of one
public closure type.

**Moved citations** (the review's M10): the change moves `part_bounds` in
`CN/src/widgets/mod.rs` from `:145-156` (K15 above; spec §12.3) to `:153-164`, and
`SC/app.rs:2232`'s `.occlude()` (§1.4) to `:2229`, since the marker's three lines go. Tasks 5
and 12 locate by content.

---

## 3 -- The maintainer's approvals

D1, the one decision these documents asked for, is decided (K14). The maintainer's approvals
G3 needs (checkpoints, not preferences): the documents; the two pushes (after Task 3, which
closes M9a's red CI, and at the close); the GPUI Fast run on the desktop session (Task 9),
since it takes window focus as M9a's batch runs did.

*Added 2026-10-21:* the maintainer accepted K19's `widgets::PartBounds` break, as the
controller relays; the controller's ruling adopted K19 as a Task 1 deviation.

---

## 4 -- Out of scope

- A 0.6.x release (its users stay on GPUI Kit 0.7.x, where it builds, K1).
- Scroll bounce as theme data (K9); scroll chaining (upstream has none; the todo entry
  keeps asking for it).
- DataTable `fit_column` in the showcase (K12).
- WinUI's tab separators and the bar's bottom rule as theme data (K14): a `[1.0]` entry.
- Upstream's `TabVariant::Folder` as the receiver of the folder shape (K15).
- A parity capture of `windows-11` (K17).
- A citation gate for the connector's doc comments (K3).
- Refreshing the non-gpui dependencies (K2).
- Filing anything upstream: the drafts go in `docs/todo.md`; the maintainer files them.
- The egui and iced connectors and showcases beyond D1's work in them (Tasks 11 and 13–15,
  K16): M9b and M9c are theirs.
