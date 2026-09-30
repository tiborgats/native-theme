# v0.6.0-rc1 — gpui connector on GPUI Kit 0.7.0: Rationale

Status: Design (2026-09-30), revised on 2026-09-30 and 2026-10-01 after an
independent fact-check, trial implementations and cold reviews (§3); nothing
implemented
Crates: `connectors/native-theme-gpui` (all code changes), `native-theme/icons/`
(three bundled SVGs), workspace `Cargo.lock` / `CHANGELOG.md` /
`docs/COMPATIBILITY.toml` / `docs/showcase-exceptions.toml` / `docs/todo.md`
Companion specification:
[`todo_v0.6.0-rc1_gpui-kit-0.7-spec.md`](todo_v0.6.0-rc1_gpui-kit-0.7-spec.md)
Companion plan:
[`todo_v0.6.0-rc1_gpui-kit-0.7-plan.md`](todo_v0.6.0-rc1_gpui-kit-0.7-plan.md)
Predecessors: the v0.5.8 and v0.5.9 documents
(`archive/todo_v0.5.8_gpui-component-0.6-*.md`,
`archive/todo_v0.5.9_gpui-kit-0.6.4-*.md`), whose decisions D1–D43 and
E1–E21 stand unless this document says otherwise. Decisions here are
numbered **K1–K17**.

---

## 0 -- What this document is for

The specification says what the gpui connector does on GPUI Kit 0.7.0. This
document says why: what was measured, which alternatives were weighed, and
what was left out on purpose.

This work lands **inside v0.6.0-rc1**, on the branch of that name: the
workspace version stays `0.6.0`, the notes go under `## [Unreleased]`, and no
separate patch release is cut. v0.6.0 is not published yet, so the first
v0.6.0 on crates.io should already require the 0.7 stack.

Conventions. Upstream citations name the published crate sources
(`~/.cargo/registry/src/…/<crate>-<version>/src/`), e.g. "gpui-component 0.7.0
`theme/schema.rs:928`". "The probe" is a throwaway git worktree of
`v0.6.0-rc1` at `cc2f3b81` whose manifest was moved to the 0.7.0 / 0.3.7 floors
and whose lockfile was then moved with
`cargo update -p gpui-component -p gpui-base -p gpui-kit`; it was patched
only as far as needed to measure and was never committed. Every number below
was measured on 2026-09-30 on the maintainer's machine (rustc 1.98.1) unless
a source is given.

The brief: GPUI Kit 0.7.0 is out (2026-09-28); is rc1 compatible, what must
change, and which new widgets belong in the showcase.

---

## 1 -- The situation, argued from evidence

### 1.1 What upstream published

| Crate | Version | Date (crates.io) |
|---|---|---|
| gpui-kit, gpui-component, gpui-base, gpui-kit-assets | 0.6.6 → **0.7.0** | 2026-09-28 |
| gpui-pre (25 snapshot packages) | 0.3.6 → **0.3.7** | 2026-09-28 |

gpui-component 0.7.0 requires `gpui-pre = "=0.3.7"` (an exact pin) and
`gpui-base = "0.7.0"` (published `Cargo.toml`). Release notes:
`gh release view v0.7.0 -R longbridge/gpui-kit`; the breaking changes that
touch this crate are the Base-owned `Root` (#3152), the plot primitives'
move to Base (#3258) and the new `ThemeColor::chart_grid` (#3236).

### 1.2 Nothing breaks by itself — but rc1 is not compatible

The connector requires `gpui-component = "0.6.6"`, a caret on a 0.x version,
which never resolves 0.7.0. So no build of rc1 breaks on a fresh resolve:
unlike v0.5.9 (a patch release removed `ThemeColor::tiles`), there is no
emergency. But an application that depends on gpui-kit 0.7 gets a second
gpui (0.3.7 beside the connector's 0.3.6), two `gpui_component::Theme`
globals, and a native theme applied to the one its widgets do not read. rc1
is therefore **not usable** with 0.7, and since v0.6.0 is unreleased, the
right time to move is now.

### 1.3 The measured cost

Probe, pass 1 (floors and lockfile only):

- `cargo update` moves exactly 27 packages (gpui-base, gpui-component,
  gpui-component-macros, gpui-kit, gpui-kit-assets, 22 `gpui-pre-*`), adds
  none and removes none; no non-gpui package changes (diff of the lockfiles'
  `name`/`version` pairs).
- The library fails with five errors in two places:
  `src/widgets/spinner.rs:240` (`ArcData` is now `#[non_exhaustive]`, E0639;
  `Arc::paint` takes 4 arguments instead of 6, E0061 — gpui-base 0.7.0
  `plot/shape/arc.rs:24-37, 243-254`) and `src/icons.rs:158, 290, 439`
  (E0004: `IconName::Ban`, `CircleAlert`, `RefreshCw` not covered).
- With those patched, the showcase fails with five more: the three
  `Root::render_{sheet,dialog,notification}_layer` calls at
  `examples/showcase-gpui/app.rs:2197-2199` (removed upstream) and
  `root.read(cx).notification` twice at `tests.rs:624, 627`.

Probe, pass 2 (everything compiles): `cargo test -p native-theme-gpui
--all-features` — lib 271 passed / 10 failed, `tests/seams.rs` 13 passed,
showcase 172 passed / 2 failed. The twelve failures:

| Test | Why |
|---|---|
| `colors::tests::theme_color_field_count_tripwire`, `no_theme_color_field_is_left_at_default`, `contract::every_theme_color_field_has_a_declared_source` | `ThemeColor` has 139 fields (new `chart_grid`) |
| `showcase::every_theme_color_field_has_one_theme_token` | the Theme Map has no token for `chart_grid` |
| `compat::the_readme_states_the_manifest_floors`, `the_quick_start_states_the_manifest_floors` | README still states 0.6.6 / 0.3.6 |
| `showcase::every_widget_reports_itself` | `NOT_WIDGET_CONSTRUCTORS` exempts the removed `render_*_layer` calls |
| `showcase::the_mirrored_handle_constants_match_gpui_base` | `HANDLE_PADDING` / `HANDLE_SIZE` moved to `resize_handle.rs:12-13` |
| `showcase::every_prose_citation_still_exists` | 1 of 701: `heading_base_font_size` is gone from gpui-base |
| `showcase::every_colour_claim_is_read_at_the_line_it_cites` | 234 of 709 colour claims cite lines that moved |
| `tests::the_palette_switches_page`, `the_palette_installs_a_preset` | a real regression, §1.4(a) |

Also measured on the probe:

- `cargo +1.95.0 check -p native-theme-gpui --lib --locked`: builds.
  `cargo +1.94.0 … --ignore-rust-version`: fails in gpui-pre 0.3.7 itself
  (`src/profiler.rs:473, 494`, `std::hint::cold_path` unstable). The declared
  `rust-version = "1.95.0"` stays exactly right (K1).
- `cargo audit` on the moved lockfile: 0 vulnerabilities, the same six
  allowed warnings as on the branch.
- `cargo clippy --all-targets --all-features -- -D warnings`: only the two
  leftovers of the probe's own patch (an unused `Root` import, an unused
  `root` binding).
- `python3 scripts/check_widget_coverage.py`: gpui discovers 147 widgets;
  **13 are neither shown nor excepted** (`InputToken`, `Questionnaire` and
  its eight parts, `TimeField`, `Toolbar`, `ToolbarGroup`) and
  one exception is stale (`Text`, the chart axis label, no longer a
  gpui-component type). On the branch the script is clean. This is what
  answers "which new widgets" in part mechanically (§1.5).

### 1.4 What changed behind the compiler's back

Found by diffing, 0.6.6 against 0.7.0, every upstream file the connector and
the showcase import from, cite or re-implement, and by comparing, per
`ThemeColor` field, the set of upstream widgets that read it.

**(a) Overlays left the showcase's element tree.** gpui-base 0.7.0 owns
`Root`; gpui-component's `Root` is a re-export (gpui-component 0.7.0
`lib.rs:96`). `Root::render` mounts the application view and, *beside it*,
one overlay per registered `RootPlugin` (gpui-base `root.rs:285-310`);
gpui-component registers a `WindowState` plugin that draws dialogs, sheets
and notifications there (gpui-component `root.rs:20-22, 461-493`). In 0.6.6
the showcase drew the three layers itself, *inside* its own root `div`
(`app.rs:2197-2199`), because 0.6.6's `Root::render` did not.

Three consequences, all real for a user and not artefacts of the test
harness:

1. **The command palette runs nothing.** Confirming an entry dispatches its
   action from the focus, which is the palette's input inside the dialog
   (gpui-component `command/state.rs:629-631`); `Window::dispatch_action`
   walks from the focused node through its ancestors only (gpui-pre 0.3.7
   `window.rs:2442-2450, 6283`). The showcase's handlers for `ShowPage`,
   `SetPreset`, `SetColorMode` sit on its own root `div`
   (`app.rs:2168-2175`), which is no longer an ancestor of the dialog. Only
   `Quit` has an application-wide handler (`app.rs:127`). The probe's debug
   run showed the query arriving, one match selected, Enter reaching
   `confirm`, the dialog closing — and the page staying `Basic`.
2. **Shortcuts pressed inside an overlay are lost.** `secondary-b`,
   `secondary-,` and `secondary-k` are bound with no key context
   (`app.rs:121-126`), so they resolve anywhere, but their handlers live on
   the same `div`: pressed while a sheet or dialog has the focus, they find
   no handler.
3. **The capture pointer shield does not cover overlays — and never covered a
   dialog.** It is "last, so it is the topmost hitbox" (`app.rs:2200-2214`),
   but a dialog paints as a `deferred` element with priority `10 + layer`
   (gpui-base 0.7.0 `dialog.rs:548, 643`; 0.6.6 `:541, :631`), and menus and
   popovers do too (`popup.rs:190-198`); deferred elements paint after the
   whole normal tree, so their hitboxes sit above any ordinary child. This gap
   is **older than 0.7.0** (found by the trial implementation, §3); 0.7.0 adds
   that the whole overlay layer now paints after the view. A capture taken
   with a dialog, sheet or menu open can therefore show it hovered.

**(b) The chart grid would be invisible.** The Bar, Line, Area and Radar
charts now stroke their grids with `ThemeColor::chart_grid`
(gpui-component 0.7.0 `chart/mod.rs:419`, `chart/bar_chart.rs:857`,
`chart/radar_chart.rs:485`; in 0.6.6 they used `border`). `ThemeColor`
derives `Default` (`theme/theme_color.rs:58`), and the connector starts from
`ThemeColor::default()` (`src/colors.rs:235, 608`), so an unassigned
`chart_grid` is transparent black. Upstream's `border.opacity(0.6)` fallback
exists only on the theme-file path (`theme/schema.rs:928`).

**(c) The config tripwire cannot see a new key.**
`theme_config_colors_cover_the_0_6_fields` counts the *non-null* values of
the exported `ThemeConfigColors` and expects 126 (`src/config.rs:353-363`).
0.7.0 adds `chart_grid` (`schema.rs:386-388`, JSON key `chart.grid`); left
`None`, the count stays 126 and the test passes. After any `Theme::change`
the config path would then install upstream's `border × 0.6` over whatever
`to_theme` set. `ThemeConfigColors` serialises 139 keys in 0.6.6 and 140 in
0.7.0: 127 / 128 public fields plus 12 private base-palette fields, which
serialise as `base.*` keys (`schema.rs:643-677`). The connector leaves 13 of
them `None` on purpose: the 12 `base.*` colours (D43) and
`group_box.title.foreground` (`schema.rs:360-361`), which no `ThemeColor`
field feeds.

**(d) Splitter colours no longer reach gpui-component's resizables.**
0.7.0's `gpui_component::h_resizable` / `v_resizable` and the dock install
`resize_handle_appearance()` (gpui-component `resizable.rs:27-41`,
`dock/dock.rs:143`), which paints a `border` hairline and a
`muted_foreground` pill (`resizable.rs:93, 104-111`). Base's own line, drawn
in the `ResizableTheme` colours the connector writes from
`splitter.divider_color` / `hover_color` (`src/base_layer.rs:113-123`), is
drawn only when no renderer is installed or the renderer declines
(gpui-base `resizable/resize_handle.rs:309-336`: "A renderer that declines —
or is absent — leaves the built-in line"). The showcase's own body splitter
is unaffected — it installs its own theme-driven renderer
(`app.rs:2142-2145`, `demo.rs:2090`) — but `Settings`, which the Layout page
and the Preferences sheet show, builds its own `h_resizable` internally
(gpui-component `setting/settings.rs:416`), out of any caller's reach.

**(e) Upstream's `Switch` gained a keyboard focus ring; the connector's did
not.** gpui-component 0.7.0 `switch.rs:166-170, 269-272` draws
`focus_ring_style` on the track while focused (#3120). The connector's
`widgets::Switch` replaces upstream's switch where a native theme is
installed; it is focusable (the test
`a_switch_toggles_on_click_and_from_the_keyboard` tabs to it) but draws no
ring, while the connector's `Checkbox` and `Slider` do
(`src/widgets/checkbox.rs:587, 734`, `slider.rs:237`). gpui-base 0.7.0's
`Switch` now accepts a caller-owned focus handle
(`switch.rs:319-323`, `track_focus`), which is what makes the fix small.

**(f) Charts became interactive by default.** Every chart now gets an id
from its construction site and `interactive` defaults on
(`chart/mod.rs:48-51`, `bar_chart.rs:116-117`): tooltips and hover dimming
(bars 0.45, `bar_chart.rs:26`; pie 0.35, `pie_chart.rs:30`). The showcase
builds each chart from its own helper (`demo.rs:7135, 7147, 7162, 7182,
7200`), so the ids do not collide, but the Widget Info claim `no_hover`
(`info/charts.rs:44-50`, used by all five) is now false.

**(g) Widget changes that make showcase prose false** (from the showcase
diff; each is a Widget Info fix, no connector code):

- `Attachment` became a fixed composer chip (3.5 rem × 14.5 rem at Medium,
  `attachment.rs:158-169, 444-456`), radius `tokens.radius.lg`
  (`:202-208`), full-strength destructive border and description when Failed
  (`:425-426, 1141-1142`), and a status glyph on source-less media — a
  `primary` `Spinner` while uploading, upstream's own `IconName::Ban` when
  failed (`:792-803, 863-866`). False today: `info/data.rs:766, 778, 784,
  789, 805-808, 834-838`.
- `ListItem`: a right-clicked row gains a 1 px `selection` outline
  (`list/list_item.rs:257-275`). False: `demo.rs:4144-4146`,
  `info/data.rs:437, 481`. `src/contract.rs:110-112` ("every reader of
  `selection` is a text selection") was already false in 0.6.6 (a
  right-clicked table row's outline, `table/state.rs:2212`); its 0.7.0
  readers are `input/input.rs:550`, `text/mod.rs:52`,
  `touch_selection/handle.rs:96`, `table/state.rs:2261`,
  `list/list_item.rs:272`, `input/token.rs:50-51`, `time/time_field.rs:135`.
- `Textarea` is now `Sizable` (`input/textarea.rs:161-188`):
  `info/inputs.rs:152` ("with no seam") is false.
- `Popover` no longer calls `render_popover_content` (inline at
  `popover.rs:351-356`); `info/overlays.rs:279` cites it — the symbol still
  exists, so the prose gate cannot catch this.
- `Root::render` no longer sets the rem size, family and foreground; the
  `WindowState` plugin does, in `prepare` / `style` (gpui-component
  `root.rs:434-448`). Cited as "Root::render" at `info/buttons.rs:534`,
  `info/text.rs:79, 154`, `info/typography.rs:33, 220`; `root.rs:594` at
  `info/leaves.rs:690, 715`; `root.rs:582` at `tests.rs:3113`,
  `src/geometry.rs:404`, `src/lib.rs:199`.
- `info/typography.rs:350` cites gpui-base `heading_base_font_size`, removed
  (#3129). The sizes it states still hold: Base still sizes headings as a
  2 / 1.5 / 1.25 / 1.125 / 1 / 1 ladder of a 14 px base
  (gpui-base `text/node.rs:3324-3334`), and gpui-component sets no heading
  refinement (`text/mod.rs:35-61`).
- `info/chrome.rs:400` says gpui-component has no toolbar widget; 0.7.0 has
  one.

**(h) Changes that need no code**, recorded so they are not re-investigated:

- `Theme::change` now runs through a private `edit`
  (`theme/mod.rs:278-316, 375-378`): it re-applies the mode's config,
  reconciles `tokens`, calls `sync_base` and refreshes **every** window; its
  `window` argument is no longer read. The connector's comment at
  `src/lib.rs:915-917` ("upstream refreshes only the window passed") is stale
  prose; the connector's own `cx.refresh_windows()` stays, because
  `sync_base` alone does not refresh (`:453-458`).
- `set_scrollbar_mode` and `sync_scrollbar_appearance` now rebuild the base
  theme through `Theme::update` (`:357-359`); each rebuild notifies the
  connector's observer, which re-writes its overrides — the mechanism v0.5.8
  built for exactly this (`src/lib.rs:1023-1060`). The observer and
  accessibility tests pass on the probe.
- `ThemeTokens::reconcile` (`theme_color.rs:371-387`) compares only within an
  `edit`. The observer's colour-only repair of the 12 base-palette colours
  (`src/lib.rs:1005-1020`) runs outside any edit, so `tokens.red …
  cyan_light` keep upstream's values; no 0.7.0 widget reads those 12 tokens
  and `semantic_tokens()` does not project them. Latent, not live; filed in
  `docs/todo.md` (K15).
- `install_text_view_defaults` moved from `change` into `sync_base`
  (`:457`), which both `edit` and the connector's apply reach.
- Every `geometry` builder's seam still holds: each target widget applies
  its size defaults and *then* `refine_style` (button `:721`, input
  `:762-781`, select `:550-552`, combobox `:995-997`, list item `:209`,
  popover `:355-360`, dialog `:674-677`, group box `:174-177`, accordion
  `:220/:307`); nothing newly lands after a refinement. The dialog's
  `max_h` is now placed by `Positioner` (`dialog.rs:584`), so the
  explanation at `src/geometry.rs:455-459` is stale prose.
- Accordion bodies unmount once closed (`accordion.rs:353-359`). The shared
  element list names only the open body (`basic.expander.details.body`,
  `docs/showcase-elements.toml:1804`), so parity is unaffected.
- `Message` stopped imposing `text_sm` / 1.25; the showcase's message content
  is a `Bubble`, which sets its own (`bubble.rs:207-208`). No visible change.
- The window frame is unchanged: the `WindowState` plugin's `decorate` wraps
  every window in `window_border()` at the default shadow size
  (gpui-component `root.rs:450-458`), as 0.6.6's `Root` did by default.
  `inspector.rs:266-270, 308` cite the old lines.
- `Select`'s popup is exactly the trigger's width (0.6.6 added 2 px,
  `select.rs:609`; 0.7.0 `:615`); the code editor's left padding is capped at
  6 px (`input/input.rs:585-589`). The Basic page has a `Select`
  (`pages/basic.rs:430-436`), but its popup is never open in the dumps or
  captures, and it has no code editor; specification §11 checks that nothing
  the parity comparator and the captures cover moved.
- The resize handle's hit band gained edge/straddle placement (gpui-base
  `resizable/resize_handle.rs:95-126`); the line's colour logic is
  unchanged.

**(i) 203 upstream line citations in the library and README are stale**, and
234 in the showcase's Widget Info (more once Task 2 shifts `src/colors.rs`,
whose lines the Theme Map's claims cite). The showcase's colour claims and
prose citations are machine-checked (v0.5.9 Widget Info gates); the
library's are not, and were mapped mechanically for this document
(specification Appendix A). The map was then checked mechanically against
both versions: 185 entries cite identical code at the new line; 12 are
marked as rewritten or moved; 6 ranges start at the same code but their
bodies changed (the dialog's `max_h`, and five the specification now flags:
the base projection's new `plot`, `ThemeColor`'s new field, the button's
open state, the editor padding cap, the `Textarea`'s tokens and size).

### 1.5 What upstream added, and what a theme showcase should show

The coverage gate settles which new public widgets must appear, or be
excepted with a reason of one of the six kinds (`docs/showcase-exceptions.toml`
header). What they read decides how to show them:

| Widget | Reads | Seam |
|---|---|---|
| `Toolbar` / `ToolbarGroup` (gpui-component) | no colour; Small = `h_8 p_1 gap_1 text_sm` (`toolbar.rs:252-256`) | `Styled`, refinement after the defaults (`:258`); `child()` forces `ghost().compact()` and an `input_h` wrapper, 1.5 rem at Small (`:17-37`, `button.rs:564-566`, `sizing.rs:273`); `content()` does not |
| `Toolbar` (gpui-base) | nothing; no geometry of its own | `Styled`, `InteractiveElement`, `ParentElement` (`toolbar.rs:137-155`); `Role::Toolbar`, arrow keys walk every focusable descendant, wrapping (`:73-133, 183-196`) |
| `TimeField` | through `input_style`: `input_background`, `foreground` (`input/input.rs:98-106`); its own: `input` border, `danger` when invalid, `selection` on the active segment, focus ring, `radius`, `input_h` (`time/time_field.rs:98-135`) | `Styled`, refinement last (`:139`) |
| `DatePicker` with a time | adds a `border` rule and a `muted_foreground` label (`time/date_picker.rs:648-668`), inside the popup | none — not shown (K12) |
| `InputToken` | `radius`, `selection` / `muted`, `border`, `foreground` (`input/token.rs:44-56`) | `Styled` |
| `Questionnaire` + parts | semantic tokens only (colour, radius, typography; spacing is `SpacingTokens::default()`, `theme/mod.rs:557-570`); choices are gpui-base `RadioGroup` / checkbox skinned as cards (`questionnaire/components.rs:671, 989-1004`) | every part `Styled`; but its state has only a fallible constructor (K12) |
| `RadarChart`, `SankeyChart` (both in 0.6.6 already, never shown; charts derive `IntoPlot`, which the coverage script does not discover) | Radar: `chart_grid` rings and spokes, series cycling `chart_1`–`chart_5`, `muted_foreground` labels (`radar_chart.rs:321-328, 485, 540`); Sankey: `chart_1`–`chart_5`, `muted_foreground`, `foreground` (`sankey_chart.rs:370, 567-571, 698`) | none — not shown here (K11) |

---

## 2 -- Decisions

### K1 -- Floors: the 0.7 stack, carets, MSRV unchanged

`gpui = { package = "gpui-pre", version = "0.3.7" }`, `gpui-component =
"0.7.0"`, `gpui-base = "0.7.0"`, dev `gpui-kit = { version = "0.7.0",
features = ["tree-sitter-rust"] }`. Carets, as before (E-series policy: an
exact pin in a library conflicts with any consumer's later snapshot); in
practice gpui-component's own `=0.3.7` pin makes 0.3.7 the only resolvable
gpui-pre. `rust-version` stays `1.95.0`: measured sufficient and necessary on
the new closure (§1.3). The `image` pin (0.25.10) is what gpui-pre 0.3.7
still resolves; unchanged.

Alternative rejected: support 0.6.6 and 0.7.0 side by side (cfg or a range).
0.x carets cannot express it, the `Root` and `Arc` APIs differ, and the brief
of every previous upgrade was "the new stack only".

### K2 -- Spinner: construct `ArcData` with its constructor

`ArcData::new(&(), 0, end - start, start * TAU, end * TAU)` and the 4-argument
`paint(&arc, color, &bounds, window)` (gpui-base `plot/shape/arc.rs:27, 243`).
`pad_angle` defaults to 0, which is what the connector passed. Pure API
move; the arc's geometry is identical (the inner / outer radii were already
builder-set).

### K3 -- Icons: every new `IconName` gets a genuine glyph in every set

`default-icons.txt` grew from 101 to 104 names — `ban`, `circle-alert`,
`refresh-cw` — and nothing was removed or redrawn (gpui-kit-assets
0.6.6 → 0.7.0). The connector's three tables are exhaustive matches, so the
compiler already demands an arm; the question is only which glyph.

- **Lucide**: `ban`, `circle-alert`, `refresh-cw`, Lucide's own names. The
  bundle (`native-theme/icons/lucide/`, pinned to Lucide 1.41.0 in
  `SOURCES.toml`) already has `refresh-cw.svg` (native-theme's
  `ActionRefresh` glyph); `ban.svg` and `circle-alert.svg` are added with
  `scripts/update_icons.sh add lucide …`, which writes the fetched bytes
  unchanged — and, needing the network, re-downloads every bundled file
  from the pinned refs, so an unchanged bundle shows only the new files. gpui-kit-assets 0.7.0's own copies
  are byte-identical to Lucide 1.41.0's (`cmp`), so the showcase draws the
  same glyph whichever set supplies it.
- **Material Symbols**: `Ban` → `block` (circle with a slash; exact),
  `CircleAlert` → `error` (circle with "!"; exact, already bundled),
  `RefreshCw` → `refresh` (a single circular arrow where Lucide draws two;
  close, already bundled — the name native-theme uses for `ActionRefresh`).
  `block` is added with `update_icons.sh add material block`; it exists at the
  pinned commit (HTTP 200, checked).
- **freedesktop**: `Ban` → `action-unavailable` (exact: a circle with a slash
  in both Adwaita and Breeze, found by the loader's symbolic-first lookup,
  `native-theme/src/freedesktop.rs:189-199`); `CircleAlert` →
  `emblem-important` (close: Adwaita's is a filled circle with "!" — in
  `symbolic/legacy`, which Adwaita's `index.theme` lists, so the lookup finds
  it — Breeze's a bare "!"; rendered and compared); `RefreshCw` → `view-refresh` (exact —
  native-theme's own `ActionRefresh` name, `native-theme/src/model/icons.rs:1139`).
  Rejected: `dialog-error` for `CircleAlert` — Adwaita draws it as a circle
  with a minus and Breeze as a red box with an ×, and `CircleX` already maps
  there.

With `RefreshCw` present, `icon_name(IconRole::ActionRefresh)` returns
`Some(IconName::RefreshCw)`: the doc comment at `src/icons.rs:76-79` lists
`ActionRefresh` as having no gpui-component equivalent, which 0.7.0 ended.
Mapped roles 30 → 31, unmapped 12 → 11. Two hand lists name every variant,
and the compiler cannot help with either, because they are data, not
matches: `ALL_ICON_NAMES` (`src/icons.rs`, with its count test 101 → 104)
and the showcase's icon gallery, `GPUI_ICONS` (`examples/showcase-gpui/
support.rs:539-540`, "The 101 gpui-component 0.6.6 IconName variants"),
beside whose `role_for_gpui_icon` table (`:503-506`, "derived from the
connector's icon_name() mapping") `RefreshCw → ActionRefresh` joins. Both
gain the three names; the prose that counts them (`info/icons.rs:49`,
`README.md:41, 505, 520`) says 104.

### K4 -- `chart_grid` comes from `list.grid_color`

The connector must set it (§1.4b). Three candidates:

1. **`list.grid_color`** — the model's own grid-line colour:
   `docs/platform-facts.md:1388` states macOS `gridColor` ("table grid
   lines", §1.1.2), Material's `row-item-outline-color` = `outline-variant`,
   and for KDE, GNOME and Windows no value of their own, which
   `docs/inheritance-rules.toml:238` resolves to `defaults.border.color`. The
   connector already draws the table's grid from it
   (`src/colors.rs:412-414`, `table_row_border = c.list_grid`).
2. `border.opacity(0.6)` — upstream's fallback for a theme file that omits
   the key (`schema.rs:928`). The 0.6 is gpui-component's house constant, not
   any platform's value; copying it would be the invented value this project
   forbids when the model has the colour.
3. `border` — what 0.6.6 drew grids with. The model states a better-named
   colour, and on the platforms without one, (1) *is* the border.

**Chosen: (1).** A chart's grid is grid lines; the platforms that name a
grid colour name it for exactly this. Consequence, stated so it is not
mistaken for a bug: on KDE, GNOME and Windows the grid is drawn at the
border's full strength, heavier than upstream's 60 %.

`to_theme_config` writes `chart.grid` too (`colors.chart_grid =
h(tc.chart_grid)`), so `Theme::change` reproduces the native value instead of
upstream's fallback (the D34 contract: the config copy carries every field
the direct path sets). `src/contract.rs` gains the row
`chart_grid ← list.grid_color`.

### K5 -- The config tripwire names the keys it leaves unset

§1.4c shows a literal count of non-null exports cannot notice a new key the
connector forgot. The test instead collects the keys `ThemeConfigColors`
serialises as `null` and requires that set to be exactly the 12 `base.*`
keys and `group_box.title.foreground` (§1.4c). A future upstream key the
connector does not export then fails the test *by name* until it is
exported or added to the named set with a reason. Measured on 0.7.0 (trial
implementation): 140 keys, 13 null, 127 exported once `chart.grid` is (K4).
The check subsumes the test's hand list of 15 keys asserted `is_some`
(`src/config.rs:328-346`), which is deleted rather than extended: every key
not in the exempt set is now asserted exported.

Rejected: `exported == keys − 1` (this document's first draft): it forgot
the 12 private `base.*` keys and fails at 127 vs 139 — the trial caught it.
Rejected: counting only non-`base.*` keys — correct, but a count again says
*that* something is missing, not *what*.

### K6 -- `Theme` itself: no change

`every_theme_field_is_named` (the v0.5.9 shape tripwire) passes on the probe:
`Theme` gained no field. `gpui_base::Theme` gained `plot: PlotTheme`
(gpui-base `theme.rs:22`), but the connector never builds that struct
literally — it edits the global through `global_mut`
(`src/base_layer.rs:134`) — and upstream fills `plot` from
`Theme::motion` in `base_theme()` (`theme/mod.rs:435`). Plot motion honours
reduced motion through Base's `spring` / `transition`
(gpui-base `plot/hover.rs:99-100, 157-163`; `motion.rs:297, 622`), so the
connector's reduced-motion forwarding covers it.

### K7 -- The showcase hosts its actions in a `RootPlugin`; its shield is deferred

Two problems share one cause (§1.4a): the overlays are no longer inside the
element that carries the showcase's action handlers; and the shield, as an
ordinary element, can never be above a deferred overlay. Options for the
actions:

1. **A showcase `RootPlugin` whose `decorate` wraps the whole root surface**
   (view *and* overlays) in a `div` carrying the action handlers. `decorate`
   runs after every plugin's surface work and receives `root: &Root`, whose
   `view()` downcasts to the `Entity<Showcase>` the handlers update
   (gpui-base `root.rs:26-80, 165, 285-310`; `AnyView::downcast`, gpui-pre
   `view.rs:58`). Plugins are instantiated when a `Root` is created, from
   the factories registered by then (`root.rs:43-44, 147-162`), so
   registering in `app::init`, which runs before any window opens, is
   enough; registered after `gpui_component::init`, it wraps outside
   `WindowState`'s `window_border()`.
2. Dispatch each palette entry's action on the showcase view's focus handle
   (`FocusHandle::dispatch_action`, gpui-pre `window.rs:628-636`) from
   `Command::on_confirm`, whose `IndexPath` is in the input model's
   coordinates (`command/command.rs:156`). Measured on the probe: the three
   palette tests pass. But it fixes only (a)1; shortcuts inside overlays stay
   broken.
3. Register the handlers application-wide with `cx.on_action` (gpui-pre
   `app.rs:2364-2367`). Global listeners get no `Window`, and during a
   dispatch the window is taken out of the app (`update_window_erased`,
   `app.rs:2888`; `with_window` returns `None` for a window on the update
   stack, `:1964-1975`), so the handlers that need a `Window`
   (`app.rs:1936-1975`) would have to run deferred through `cx.defer`,
   changing when their effects land.

**Chosen: (1)** for the actions — the mechanism upstream built for "another
structural wrapper" around the root (the trait's own doc, `root.rs:60-65`),
restoring the 0.6.6 invariant "every overlay is inside the element that
handles the showcase's actions". The `div` in `Showcase::render` loses its
`on_action` list; `track_focus` stays there. The palette entries keep their
`CommandItem::action`. The eight handlers become `pub(crate)` (they are
private `fn`s of `Showcase`, `app.rs:1889-1976`), and `host.rs` joins the
files the showcase's gates read (`SHOWCASE_FILES`, `src/showcase.rs`).

**The shield** stays where it is, the last child of `Showcase::render`'s
root `div`, and becomes `deferred(…).with_priority(usize::MAX)`: deferred
elements are collected wherever they sit in the tree and painted after the
whole normal tree, ordered by priority (gpui-pre `elements/deferred.rs:62-64`,
`window.rs:3713, 3799`), so it is the topmost hitbox above every top-level
overlay — dialogs (priority `10 + layer`), sheets, notifications, menus and
popovers (priority 100). gpui never adds to a priority
(`elements/deferred.rs:25`), so `usize::MAX` cannot overflow. Measured in the
trial (with the shield in the host; its position does not matter to a
deferred element): all 176 showcase tests pass, including a new one that
hovers an open dialog under the shield; without `deferred`, that test fails.
Rejected: moving the shield into the host (the previous draft) — it made the
root's decoration read showcase state and bought nothing. Rejected: an
ordinary shield anywhere — no ordinary element is above a deferred one.

Known limit, stated so it is not mistaken for coverage: a deferred element
*nested inside* another deferred one is prepainted in a later round
(gpui-pre `window.rs:3700-3713`), and hitboxes are inserted at prepaint and
hit-tested newest first (`:1106-1122`, `:5104-5108`), so a submenu
(`menu/popup_menu.rs:1384-1399`) or a `Select` popup opened inside a dialog
(`select.rs:609-636`) sits above the shield whatever its priority. No
capture opens one: `--open-menu theme` opens a top-level menu
(`menu/app_menu_bar.rs:286`).

### K8 -- The showcase opens its window with `gpui_kit::open_window`

gpui-kit 0.7.0's `open_window(options, cx, build)` is "the one window entry
point" and does exactly what `main.rs:1461-1468` and `tests.rs:113-118` do by
hand (gpui-kit `lib.rs:139-159`), returning the content entity, which removes
the `showcase_entity` `Option` dance at `main.rs:1461-1471`. The showcase's
stated rule is to use "the facade an application would"
(`Cargo.toml` dev-dependency comment). It returns an `AnyWindowHandle`, so
the four `*window_handle` uses in `main.rs` (among them `dump_layout`'s,
`:1482-1489`) take the handle as it is — measured in the trial. It must
**not** call `window_border()` itself: `WindowState::decorate` already does,
and a second call nests two client-side frames. `open_window` itself holds an
`expect` on a value its own closure always sets (gpui-kit `lib.rs:156-157`):
upstream code on an unreachable path, which the no-panic rule (it governs
this project's code) does not reach; the showcase adds none.

### K9 -- Splitters: documented, no new API

0.7.0's `gpui_component::h_resizable` is exactly
`gpui_base::h_resizable(id).with_handle_appearance(resize_handle_appearance())`
(gpui-component `resizable.rs:27-30`). gpui-base's own `h_resizable` /
`v_resizable` install no renderer, so its built-in line is drawn in the
`ResizableTheme` colours the connector projects from `splitter.*` — exactly
what 0.6.6's `gpui_component::h_resizable`, a re-export of gpui-base's, drew
(0.6.6 `lib.rs:107-110`). gpui-base is public to applications (gpui-kit
re-exports it as `gpui_kit::base`, gpui-kit `lib.rs:103`). So an application
that wants the splitter's native colours builds its groups with
`gpui_kit::base::h_resizable` (or `gpui_base::h_resizable`); the connector's
README and `base_layer` docs say so. Nothing is added to the connector's API.
What that gives is the colours only, not the native splitter: gpui-base's
built-in line is `HANDLE_SIZE`, 1 px (`resize_handle.rs:13, 330-336`), while
26 of the 40 preset variants state a `splitter.divider_width` of 4 or 6 px;
and `splitter.hover_color` shows only while the handle is dragged
(`src/base_layer.rs:121`). That was equally true on 0.6.6 — not a
regression. The showcase draws the native line with a renderer of its own
(`demo.rs:2087-2147`) that paints only what the model states (width, divider
colour, hover colour); moving it into the connector as a public renderer is
the real fix and is filed in `docs/todo.md`.

Rejected: a public `base_layer::resize_handle_appearance()` returning a
declining renderer (this document's first draft) — it reproduces what the
gpui-base constructor already gives, as a new public item with a test.
Rejected here, not in principle: a connector renderer. One that imitates
upstream's pill would invent its lengths and opacities; one that paints only
what the model states (the showcase's) is the right follow-up, but it is not
0.7.0 work — 0.6.6 drew the same 1 px line.
Rejected: writing `border` / `muted_foreground` to follow the splitter —
they are read by dozens of other widgets.

Documented limit: `Settings` (`setting/settings.rs:416`) and the dock's
edges (`dock/dock.rs:143`) install upstream's renderer internally; no caller
can reach them. README, `base_layer` docs and the showcase's Widget Info say
so; `docs/todo.md` records the upstream request (a handle-appearance setter
on `Settings`).

### K10 -- The connector's `Switch` draws the focus ring

Owning a keyed focus handle, passing it to gpui-base's switch with
`track_focus` (new in 0.7.0, gpui-base `switch.rs:319-323`), and applying
`focus_ring_style` (a `ThemeStyled` method, gpui-component
`styled.rs:127, 165-189`) to the track while focused — the connector's
`Checkbox` pattern (`src/widgets/checkbox.rs:563-587`) and upstream's own
switch (`switch.rs:166-170, 269-272`). The ring's colour is
`ThemeColor::ring`, which the connector maps from
`defaults.focus_ring_color` (`src/colors.rs:197, 268`). Without it, rc1 would
ship a native switch *less* accessible than the upstream one it replaces.

What a test can prove: that the change adds no second tab stop (after Tab
from the only switch in a window, the focused handle is unchanged —
measured in the trial, and shown to fail when a second handle is added). It
cannot prove the connector's handle is the one the base switch tracks:
without `track_focus`, gpui-base keys a handle of its own on the same id
(`switch.rs:325-330`), the test still passes, and the ring never shows. The
ring itself is not observable headlessly: gpui-component names the ring element only under its
own `cfg(test)` (`styled.rs:264-267`), and colour is not in the layout. It is
checked by hand, on an *unchecked* switch under one light and one dark
preset: the connector's track has no border (`src/widgets/switch.rs:269-281`),
so `focus_ring_style` draws only its 50 % halo (`styled.rs:175-189`), which
can land close to an unchecked track's colour — the case upstream changed
its own switch for.

Filed, not done here: the theme-drawn widgets (Checkbox, Slider, now Switch)
draw upstream's fixed ring width and opacity while the model states
`defaults.focus_ring_width` / `focus_ring_offset` (older than 0.7.0); and
0.7.0's upstream `Switch` gained `tab_stop`, `tab_index` and
`focus_ring(bool)`, which the connector's replacement does not offer.

### K11 -- Charts: keep upstream's interactivity; no new chart here

Rejected: `.interactive(false)` to keep the old claim true — the showcase
shows widgets as an application gets them, and hover tooltips are now what
an application gets. `no_hover` is replaced by claims describing the
tooltip: its surface is `popover`, through `popover_style`
(`plot/tooltip.rs:590` → `styled.rs:197`, the line the colour gate reads);
its label `muted_foreground` (`:545`); its guide line per chart — Line and
Area a dashed hairline of `border` mixed toward `foreground`
(`tooltip.rs:125-127`), Bar and Candlestick a band of `foreground` at 8 %
(`.band(…)`, `:128`; `bar_chart.rs:1109`, `candlestick_chart.rs:433`), Pie
none; and the hover dimming literals (`bar_chart.rs:26`, `pie_chart.rs:30`).

The existing Bar, Line and Area charts already draw their grids in
`chart_grid` (grids are on by default, `bar_chart.rs:108`, `line_chart.rs:69`,
`area_chart.rs:71`, and the showcase never turns them off,
`demo.rs:7133-7171`), so K4 is visible without a new section; their grid
claims move from `border` to `chart_grid` (Candlestick keeps `border`,
`candlestick_chart.rs:302-304`).

`RadarChart` and `SankeyChart` are never shown, but both existed in 0.6.6:
they are an older gap, and its cause is the coverage script, which cannot
see charts (they derive `IntoPlot`, not `IntoElement`, so its gpui rule
discovers none). The right fix is the gate — teach it `IntoPlot`, then show
what it finds — not one chart added by hand to an upgrade; `docs/todo.md`
records it. Rejected: adding `RadarChart` here (the first drafts) — it is
not 0.7.0 work, and adding it while `SankeyChart` stays hidden would treat
the same gap two ways. Rejected: a second `LineChart` with `y_axis`, a solid
grid and a reference line — it adds only `muted_foreground` labels and a
dashed line, colours shown elsewhere; the grid is already visible. The regression gate for the connector writing `chart_grid` is the
unit test of K4, not a showcase claim (a colour claim checks the upstream
line that reads a field, not the connector that writes it).

### K12 -- New widgets: show what the coverage gate discovers, except `Questionnaire`

- **`geometry::toolbar` leaves the height to the content** (`h_auto()`, a
  connector change). The builder already states `toolbar.bar_height` only as
  a minimum, because KDE's and GNOME's toolbars size to their content
  (`docs/platform-facts.md:1351`; `src/geometry.rs:708-730`), but a widget
  with a fixed height of its own keeps it under the refinement: 0.7.0's
  component `Toolbar` sets `h_8` at Small (`toolbar.rs:252-256`), which would
  win on KDE. Setting the height to `auto` in the builder is what its doc
  already promises; on a plain `h_flex` row (the chrome's) it changes
  nothing; an application that wants a fixed height sets one after the
  refinement. Rejected: telling every application to add `h_auto()` (the
  previous draft) — the builder can guarantee it once.
- **The component `Toolbar` and `ToolbarGroup`** get a section on the Buttons
  page, built in a `demo.rs` helper that reports itself: the toolbar refined
  with `geometry::toolbar`; two groups ("Edit": Undo2, Redo2, Copy; "View":
  Search, Maximize, Settings), each added with `content()` (with `child()` a
  group, being `Sizable`, would be wrapped in an `input_h` box,
  `toolbar.rs:29-35`), each holding ghost icon buttons also added with
  `content()` (so they keep the theme's padding instead of
  `ghost().compact()`), spaced by `toolbar.item_gap` where stated and
  otherwise by upstream's own Small gap (`gap_1`, `toolbar.rs:254` — a
  `ToolbarGroup` spaces nothing itself); a vertical `Separator` between the
  groups. Widget Info states the size literals and what `child()` would do
  instead; the README's `toolbar` row tells applications to add items with
  `content()` for the same reason.
- **The chrome toolbar stays an `h_flex` row.** Rejected: moving it onto
  `gpui_base::Toolbar` for its `Role::Toolbar` and arrow-key roving (the
  previous drafts). The coverage gate does not ask for it (it counts
  gpui-component's `Toolbar`, which the Buttons page shows); it would give
  gpui's shared chrome keyboard behaviour the egui and iced chrome lack; and
  it could be tested only by hand. A toolbar role and roving for the chrome
  of all three showcases is filed in `docs/todo.md`.
- **`TimeField`** on the Inputs page, beside the existing `DatePicker`.
- **`InputToken`**: an `Input` on the Inputs page holding one inline token,
  inserted at construction and rendered through
  `Input::token(|ctx, _, _| InputToken::new(ctx))` (`input/input.rs:178-186`).
  Measured in the trial: the token is accepted and drawn.
- New widgets are built in `demo.rs` helpers that call `.info(…)`; built
  directly in a page, they fail `every_widget_reports_itself` (trial).
- **`Questionnaire` and its eight parts are excepted**, for a reason none of
  the existing six kinds covers, stated as it is: the widget cannot be built
  without a panic path. `Questionnaire` and seven parts take an
  `Entity<QuestionnaireState>` (`questionnaire/components.rs:308, 377, 536,
  616, 700, 1089, 1160, 1246`); the only constructor of that state is
  fallible — `QuestionnaireState::new(items, cx) -> Result<Self,
  QuestionnaireSchemaError>` (gpui-base `questionnaire/state.rs:40-45`) —
  while gpui builds an entity from an infallible closure (`fn new<T>(&mut
  self, build: impl FnOnce(&mut Context<T>) -> T) -> Entity<T>`, gpui-pre
  `app.rs:2971`). There is no `Default` and no unchecked constructor;
  upstream's own tests `unwrap()` it (`components.rs:1480`). Even an empty
  schema, which cannot fail (`validate_schema` is the only `?`,
  `state.rs:44, 120-147`), arrives as a `Result`, so a fallback does not
  remove the need to unwrap. `reserve_entity` / `insert_entity` take the
  same infallible builder (gpui-pre `app.rs:2987-3001`), and gpui-base
  offers no public schema validation (`validate_schema` is private,
  `state.rs:120`). Weighed and rejected: a loop that retries with the item
  list taken out, which terminates because an empty schema always validates
  — it trades an unreachable panic for a possible hang should upstream ever
  change that, which is not better. The eighth part,
  `QuestionnaireChoiceDescription` (`new() -> Self`, `components.rs:1032`),
  needs no state but is secondary text *inside* a `QuestionnaireChoice`
  card: it is excepted as a composition slot (an existing kind). The
  showcase has no `unwrap` or `expect` outside its tests, and the project
  forbids adding one. `docs/todo.md` records the request to upstream (an
  infallible constructor, or schema validation at a later step), after
  which the widget is shown: a single- and a multiple-choice question whose
  Widget Info states that its choices are gpui-base controls skinned by
  upstream (`components.rs:671, 989-1004`), not the connector's
  `widgets::Radio` / `Checkbox`, and that its spacing is
  `SpacingTokens::default()`.
- The stale `Text` exception is removed; every remaining gpui exception is
  re-read at 0.7.0 and its citation corrected (the file header says they were
  read at 0.6.6).

Rejected: showing `Questionnaire` behind an `expect` on a constant schema.
The schema *is* constant and a test could prove it valid, but that is the
reasoning the no-panic rule exists to refuse; the maintainer may overrule
this one point (it is the only place this design meets the rule). Rejected:
excepting it as "a composition of existing controls" — it has a skin of its
own (choice cards, progress text, a typography ladder), which is why it
should be shown once it can be.

Rejected: a time-enabled `DatePicker` (the first draft). Its only new
surface is a row inside the popup — a `border` rule and a `muted_foreground`
label, colours shown elsewhere — around the same `TimeField` the Inputs page
now shows standalone; a second picker state and Widget Info entry would buy
nothing a capture or a reader could see.

### K13 -- Widget Info is corrected, not reworded to fit

Every false statement of §1.4(f)–(g) is rewritten against the 0.7.0 source.
The rule of v0.5.9 §7 stands: where the cited code no longer says what the
prose claims, the *claim* is the finding.

### K14 -- Citations

Library and README: the 203 mapped entries of Appendix A are applied; the
entries marked rewritten or moved, and the rows of specification §9.2, are
re-read, not shifted. Showcase: the gates (`every_colour_claim_is_read_at_the_line_it_cites`,
`every_prose_citation_still_exists`,
`the_mirrored_handle_constants_match_gpui_base`) drive the work and prove it.
Module headers that name the verified stack say "gpui-component 0.7.0,
gpui-base 0.7.0, gpui-pre 0.3.7".

Rejected for this release: extending the showcase's citation gates to
`src/*.rs`. The library's citations are prose in doc comments, not
structured claims; a gate for them is a separate design (filed in
`docs/todo.md`).

### K15 -- Upstream changes accepted without code

Everything in §1.4(h). The base-palette tokens (latent) and the reload icon
(K16) go to `docs/todo.md`.

### K16 -- Out of scope, on purpose

- **Popover arrow.** `Popover::arrow(true)` is a caller option. Whether a
  platform's popover draws an arrow is a platform fact
  `docs/platform-facts.md` does not record; it needs an entry with sources
  before the model or the showcase can use it. Filed.
- **The toolbar's reload glyph.** All three showcases draw `rotate-cw` /
  `rotate_right` / `object-rotate-right` for "Reload System Theme"
  (`chrome.rs:132`, egui `demo.rs:854-856`, iced `showcase-iced.rs:3866`);
  `refresh-cw` / `view-refresh` is the semantically right glyph now that gpui
  has it, but changing it touches egui and iced and the shared element list.
  Filed.
- The time-enabled `DatePicker` and a second chart with axes: rejected, not
  deferred (K11, K12). `RadarChart` / `SankeyChart`: deferred to the
  coverage gate's fix (K11). The chrome toolbar's role and roving: deferred
  to all three showcases (K12). A connector splitter renderer (K9); the
  focus ring's width / offset from the theme and the `Switch`'s
  `tab_stop` / `tab_index` / `focus_ring` options (K10): filed.
- **Applications' window startup.** Nothing changes in the connector's API,
  but in 0.7.0 a window opened before `gpui_kit::init` gets no `WindowState`
  — no dialogs, no rem scaling, so text scaling silently fails
  (gpui-component `root.rs:434-436`; plugins are captured when a `Root` is
  created, gpui-base `root.rs:43-44, 147-162`). The README's Quick start
  shows `gpui_kit::open_window` after `init`, and its Accessibility row's
  "`Root` sets the window rem" (`README.md:380`) names `WindowState`
  (specification §10.1).
- Dock close buttons, bottom-dock resizing, inline-token activation, editor
  range decorations, Markdown search highlights: not theme surface.
- Exposing `Theme::update` from the connector: the connector replaces the
  whole theme at once (`*GpuiTheme::global_mut(cx) = theme`, then
  `sync_base` and `refresh_windows`), which is what `update` does minus the
  reconcile it does not need.

### K17 -- Version, notes, stamp, archive

Workspace version stays `0.6.0`; `CHANGELOG.md` gains lines under
`## [Unreleased]`; `docs/COMPATIBILITY.toml` and the README's Verified line
are rewritten only by `scripts/update_compatibility.sh run gpui`, after the
work is committed (it refuses a dirty tree). The screenshots and the
theme-switching GIF capture the Basic page only
(`scripts/generate_screenshots_gpui.sh`, `generate_gifs_theme_switching.sh`:
`--tab basic`), whose elements this work must not move (specification §11
checks that); they are regenerated by the maintainer's release run
(`scripts/generate_assets_release.sh`) as always, not by this plan. The last
task moves these three documents to `docs/archive/`.

---

## 3 -- What was verified, and how

| Claim | How |
|---|---|
| Versions and dates | crates.io API |
| Compile errors, test failures, MSRV, audit, clippy, coverage | the probe, commands in §1.3 |
| Palette root cause | debug output on the probe (query, match, selection, dialog state, page before and after Enter); the dispatch path read in gpui-pre 0.3.7 |
| Option 2 of K7 passes the palette tests | run on the probe (3 passed); reverted |
| Upstream behaviour (§1.4) | diffs of both versions' sources; per-field reader comparison of `ThemeColor` |
| Appendix A line map | computed from both versions' sources, then every entry compared line by line in both versions (185 identical, 12 marked, 6 bodies changed) |
| Icon glyphs | SVGs rendered with `rsvg-convert` and compared by eye; upstream files fetched at the pinned refs and `cmp`-ed; `update_icons.sh` writes the fetched bytes unchanged |
| Every cited claim of this document and the specification | an independent fact-check against both versions' sources (about 150 claims; its corrections are folded in) |
| The riskiest code of the plan | a trial implementation of the plan's Tasks 1, 2, 3, 5, the since-removed chrome-toolbar task and the new-sections task's state code on the probe: the corrections it forced are folded in (K5's key set, K7's deferred shield, K8's handle, K10's test, K12's helpers and the dropped role test, `host.rs` in `SHOWCASE_FILES`); final state 279 lib / 13 seams tests and every showcase test passing (176, plus two trial-only tests the plan does not keep), only the gates later tasks own failing, clippy clean |
| Consistency of the three documents | two cold reviews for contradictions, ambiguities, executability in order and omissions; their findings are folded in (among them the showcase's second hand-kept icon list, the nested-overlay limit of the shield, Linux captures) |
| Executability of the plan as written | a literal replay, from a clean `cc2f3b81`, of the plan's Tasks 1–5, the chrome-toolbar task since removed, and the new-sections task (now Task 8): every Expected of Tasks 1–3 matched; its corrections are folded in (tooltip and Switch claim citations, gate-readable prose citations, the Buttons-page toolbar's specifics, cargo filter syntax, imports); end state clippy and fmt clean, lib 279 / 5 expected failures, seams 13, showcase 177, coverage clean |
| Judgment | a maintainer-style critique of K1–K17 for better options and needless cost; adopted: `geometry::toolbar`'s `h_auto`, the shield staying in the view, cutting the chrome-toolbar move, the splitter wording, K5's simpler test, the README startup note, the Switch's manual-check conditions |
