# v0.7.0 — GPUI Kit 0.8.1 (G4): Rationale

**Status:** design, not implemented. Written 2026-10-26, the day GPUI Kit 0.8.1 was published.
The maintainer approved P1 and P3 the same day (§3). The other decisions are settled by logic,
and the maintainer may veto any of them. The evidence comes from:

- a trial build on a throwaway worktree of `d2d4a3af` (rustc 1.99.1);
- the upstream repository at tags `v0.8.0` and `v0.8.1`;
- the published `gpui-pre` 0.3.9 and 0.3.10, `gpui-fast` 0.1.10 and 0.1.12, `gpui-fast-web`
  0.1.12 and `taffy` 0.15.0 crates, downloaded from static.crates.io.

A spec and a plan follow (P1–P15 below). This document says *why*.

Citation conventions:

- Upstream crates: `GC` = gpui-component 0.8.1 `crates/component/src/`, `GB` = gpui-base 0.8.1
  `crates/base/src/`, `GP` = gpui-pre 0.3.10 `src/`, `GF` = gpui-fast 0.1.12 `src/`.
- Our crate: `CN` = `connectors/native-theme-gpui/`, `SC` = `CN/examples/showcase-gpui/`.
- Upstream line numbers are of the new versions unless "0.8.0" or "0.3.9" is written. Our line
  numbers, connector and repository files alike, are of `d2d4a3af`. `CHANGELOG.md` entries are
  cited by their text, since the branch's other work moves their lines.

---

## 0 — What this document is for

GPUI Kit 0.8.1 was released on 2026-10-26 (release notes:
<https://github.com/longbridge/gpui-kit/releases/tag/v0.8.1>). Its crates — gpui-kit,
gpui-component, gpui-base, gpui-kit-assets, gpui-component-macros 0.8.1 — were published to
crates.io between 08:32 and 08:36 UTC. They pin `gpui-pre = "=0.3.10"` (published 00:38 UTC)
and ask for `gpui-fast = "0.1.12"` (08:19 UTC; upstream workspace `Cargo.toml`).

The connector's floors are carets (`CN/Cargo.toml:66`, `:70`, `:75`: `"0.8.0"`). So:

- **The released 0.6.1 is not affected.** It asks for `^0.7.1`, which does not take 0.8.x.
- **The coming v0.7.0 is.** Shipped as the branch stands, any application that resolves its
  dependencies fresh gets 0.8.1 with gpui-pre 0.3.10 (measured, §1.2). v0.7.0's claims,
  tests and screenshots would then describe 0.8.0, which its users do not run.
- **The branch's own CI goes red at its first push that resolves 0.8.1**: the test code of
  §1.3 does not compile there. The nightly dependency canary does not see it: a scheduled
  workflow runs on the default branch, `main`, whose released 0.6.1 code asks for `^0.7.1`.
  (That canary has failed every night since at least 2026-10-23, in its *Test* and *Widget
  coverage* steps. The cause is unrelated to 0.8.1 and not diagnosed here.)

The question is what v0.7.0 changes for 0.8.1 before it is released.

---

## 1 — What was measured and read

### 1.1 What 0.8.1 is

The release notes call it a compatible patch for *Kit-owned* APIs. Upstream deferred the
`MotionTokens` field additions (#3616) and the Textarea associated-type change (#2842) to keep
it one.

Two parts of the release are not compatible:

- **The two GPUI snapshots change public GPUI APIs**, which Kit re-exports. The notes list
  them: `Keystroke::parse` and `KeyBinding::new` take `&'static str`, a `secondary-` modifier
  needs the OS, `unparse` takes an `OperatingSystem`, and `ScrollWheelEvent` is renamed
  `ScrollEvent` and gains a `source` field.
- **gpui-pre 0.3.10 moves from taffy 0.13.0 to taffy 0.15.0**, the layout engine
  (gpui-pre `Cargo.toml`: `[dependencies.taffy] version = "0.15.0"`, was `"0.13.0"`).

The theme schema is untouched:

- No file under `GB/theme*` or `GC/theme/` changed, and no `Theme`, `ThemeColor`,
  `ThemeConfig` or tokens field was added, removed or renamed.
- No `cx.theme().…` or `tokens.…` read was added or removed in the non-test code of either
  crate. Three reads moved: the table's `table_active` (re-indented only), the Dialog's motion
  tokens (also read in two new functions, §1.6), and TextView's `tokens.typography.mono` (now
  the fallback behind `inline_code_style`).
- `IconName` is unchanged. gpui-kit-assets and gpui-component-macros changed only their
  manifests and the assets' examples, so the connector's icon tables and the README's
  statement about the icon lists (`CN/README.md:57`) hold, with the version moved.

### 1.2 The trial build

Method:

- A throwaway worktree of `d2d4a3af` (the main tree has another session's work in progress).
- The four requirements raised to 0.8.1 (`CN/Cargo.toml:66`, `:70`, `:75`, `:104`) and the
  `gpui` alias to `0.3.10` (`:63`), then `cargo update -p gpui-kit -p gpui-component -p
  gpui-base`.

**Without the floor bump**, the same `cargo update -p …` on the existing lock moved only
gpui-component-macros: `cargo update -p` keeps the rest of the locked graph.

**A fresh resolve is different.** Two scratch projects that depend on `gpui-component =
"0.8.0"`, one with no `rust-version` and one with `rust-version = "1.88"`, both lock
gpui-component 0.8.1 and gpui-pre 0.3.10 on `cargo generate-lockfile`. This is what an
application gets.

**The lockfile change:**

- The GPUI family moves: gpui-kit, gpui-kit-assets, gpui-component, gpui-component-macros and
  gpui-base go to 0.8.1; the 20 `gpui-pre*` crates go 0.3.9 → 0.3.10; gpui-fast and its eight
  sub-crates go 0.1.10 → 0.1.12.
- **Two duplicates disappear:**
  - wgpu 29.0.4 and its eight sub-crates, plus naga 29.0.4: `gpui-pre-wgpu` 0.3.9's copy
    (`cargo tree -i wgpu@29.0.4`). gpui-pre now shares gpui-fast's copy.
  - taffy 0.13.0 (gpui-pre 0.3.9's) and 0.14.0. One taffy remains, 0.15.0.
- proptest 1.11 → 1.12, with new lock entries `bit-vec` 0.10.1, `core_detect` and `borsh`.
  These are optional gpui-pre dependencies (`proptest`'s `attr-macro`, per `cargo tree -i
  proptest -e all`). Nothing in a normal build compiles them.
- `image` stays one copy, 0.25.10.

**Compile results:**

- `cargo check -p native-theme-gpui --all-targets`, on gpui-pre and with `--features
  gpui-fast,native,watch`:
  - **The library and the showcase binary compile with no error and no warning on both
    cores.**
  - **The test code does not compile**: one error in the library's test module and six in the
    showcase's tests, plus a deprecation warning (§1.3).

**Test results.** These are after a minimal trial migration of §1.3's sites: upstream's
`is_mac()` literals for the bindings, and the host OS through `cfg!` in the tests. That was a
stand-in to get the suite running. P3–P5 specify the code to write, which differs (`load`,
`cx.platform_info()`), and its own gates decide its results. Both cores fail the same tests:

| Target | gpui-pre | GPUI Fast | Failures |
|---|---|---|---|
| lib | 371 passed, 4 failed | 373 passed, 4 failed | `compat::the_readme_states_the_manifest_floors`, `compat::the_quick_start_states_the_manifest_floors` (README floors), `showcase::every_colour_claim_is_read_at_the_line_it_cites` ("125 of 821 colour claims"), `showcase::every_prose_citation_still_exists` ("14 of 966 prose citations") |
| seams | 18 passed | 18 passed | none |
| showcase | 322 passed, 1 failed | 322 passed, 1 failed | `tests::the_fallback_stack_copy_is_gpuis` ("gpui-pre 0.3.9 is not in the cargo metadata", §1.9) |

Every failure is a gate that works as designed: version statements and citations that have to
move. None is a behaviour failure.

**MSRV** (`RUSTFLAGS=-Dwarnings`, CI's `msrv` job):

- 1.95.0 checks the library on both cores.
- 1.94.0 fails in gpui-pre 0.3.10 on `std::hint::cold_path` (`GP/profiler.rs:473`, `:494`),
  the same failure `CN/Cargo.toml:6-11` records for 0.3.9.
- **So the MSRV stays 1.95.0.**

**Not run in the trial:**

- clippy;
- `scripts/check_release.sh`;
- the parity and capture scripts;
- `cargo audit`;
- the macOS and Windows targets.

**Not measured:** the run-time layout under taffy 0.15 (§1.8).

### 1.3 gpui-pre 0.3.10's key handling, and the five places it reaches us

What changed, in both GPUI cores (gpui-fast 0.1.12 has the same code, `GF/keymap/binding.rs:35`,
`GF/platform/keystroke.rs:120`):

- **`Keystroke::parse(source: &'static str)`** (`GP/platform/keystroke.rs:120`) now
  parses with no OS. A `secondary` component returns `InvalidKeystrokeError` (`:158-161`):
  `let os = os.ok_or_else(..)?`.
  - In 0.3.9 the same literal parsed to Cmd on macOS and Ctrl elsewhere (0.3.9 `:143`).
  - `parse_with_os(source: &str, os)` (`:125`) is the new OS-aware form.
- **`KeyBinding::new(keystrokes: &'static str, …)`** (`GP/keymap/binding.rs:35`) parses with
  that same `Keystroke::parse` and ends in `.unwrap()` (`:47`). Its doc says so: "Panics on
  parse error", and 0.3.9's did too.
  - `KeyBinding::load(…, cx: &App) -> Result<Self, InvalidKeystrokeError>` (`:51`) parses
    with `parse_with_os(source, cx.platform_info().os)`.
- **`unparse(&self, os: OperatingSystem)`** (`GP/platform/keystroke.rs:241`, `:397`) now
  takes the OS.
- **`ScrollWheelEvent` is a deprecated alias of `ScrollEvent`**, which has a new field,
  `source: ScrollSource` (`GP/interactive.rs:522-542`).
- **The new OS identity is `App::platform_info() -> PlatformInfo`** (`GP/app.rs:2001`,
  `GP/platform/platform_info.rs`).
  - It is runtime state, not a `cfg`. A browser on macOS follows Mac conventions, so the OS
    is not the compile target.
  - The test platform reports the host's OS (`GP/platform/test/platform.rs:208-220`) and can
    be told another one (`TestAppContext::set_platform_info`, `GP/app/test_context.rs:127`).

Upstream migrated its own bindings by choosing the literal at run time:
`KeyBinding::new(if cx.platform_info().is_mac() { "cmd-c" } else { "ctrl-c" }, …)`
(`GC/diff/mod.rs:77`, `:86`). Its tests use `parse_with_os(key, cx.platform_info().os)`.

What it does to the branch, by site:

| Site (`d2d4a3af`) | On 0.8.1 | How it shows |
|---|---|---|
| `SC/app.rs:178-181`, `KeyBinding::new("secondary-q" / "-b" / "-k" / "-,", …)` in `app::init` | compiles (the literals are `'static`), **panics at start-up** inside gpui (`binding.rs:47`) | measured: with only this file unmigrated, **240 of 323 showcase tests panic** at `gpui-pre-0.3.10/src/keymap/binding.rs:47:10`, since the test harness calls `app::init` (`SC/tests.rs:125`) |
| `SC/pages/typography.rs:97` `KBDS` (`"secondary-c"` …) through `SC/demo.rs:6478` `Keystroke::parse(keys).ok()?` | compiles, every parse fails, `filter_map` drops all four Kbds (`SC/pages/typography.rs:196`) | **the Typography page's Kbd row is silently empty**; `the_kbds_take_the_showcases_modifier` (`SC/tests.rs:17424`) fails on it |
| `SC/pages/feedback.rs:136` `SAVE_KEYS = "secondary-s"` through `:142` | compiles, the parse fails, the fallback prints the raw literal | **the Save tooltip reads "Save file (secondary-s)"**; the tooltip test (`SC/tests.rs:17453`) fails on it |
| `SC/tests.rs:2380` `fn secondary(key)`, `Keystroke::parse(&format!(..))` | does not compile (not `'static`) | fixed naively with an OS argument, it compiles. But its `unwrap_or_default()` returns `""` on a parse failure, so **`no_inspector_toggle_remains` (`:2403`) would pass while checking nothing**: it compares every binding with an empty string |
| `SC/tests.rs:416` (`ScrollWheelEvent`), `:2229` (`parse(keys)`), `:2363`, `:2382`, `:2414`, `:5345` (`unparse()`); `CN/src/widgets/tests.rs:1714` (`parse(key)`) | 7 compile errors, 1 deprecation warning | `cargo check --all-targets` fails on both cores |

The library's own code calls none of these outside its tests: `CN/src` compiles unchanged.

### 1.4 Why the no-panic rule did not catch the start-up panic

**None of our code panics. The panic is inside a dependency function we call:**

- `KeyBinding::new` is documented to panic on a parse error.
- The four literals were valid when they were written: 0.3.9's grammar had `secondary`.
- In 0.3.10 the same literals are a parse error, so gpui-pre's own `.unwrap()` fires.

**Our gates all look only at code we write:**

- `scripts/check_panic_patterns.py` is a regex over our source: `.unwrap()`, `.expect(`,
  `panic!(` and the rest.
- `scripts/check_strict_panic.sh` is clippy with `unwrap_used`, `expect_used`,
  `indexing_slicing`, `arithmetic_side_effects` and others promoted to errors, all of them
  lints on our code.
- The crate roots deny `unwrap_used` and `expect_used` (`CN/src/lib.rs:200-201`).

**No lint flags a call to someone else's function that panics on its input.** So the rule has
a gap: dependency functions documented to panic on input that is valid today and can become
invalid when the dependency changes its grammar.

**Why it would not have shipped.** The showcase's test harness runs `app::init`, so 240 tests
fail loudly, and CI's gpui test job and `check_release.sh` run those tests on both cores. P6
closes the gap with a gate, so that the next such case fails at lint time rather than at test
or run time.

### 1.5 Toolbar: upstream's fixed height became a minimum

`RenderOnce for Toolbar` (`GC/toolbar.rs:253-255`) set `h_7` / `h_8` / `h_12` in 0.8.0 and
sets `min_h_7` / `min_h_8` / `min_h_12` in 0.8.1 ("Toolbar: grow with taller content", #3651).

`geometry::toolbar` (`CN/src/geometry.rs:1023-1038`) does two things:

- **sets `.h_auto()`**: its doc says this "also replaces the `h_8` gpui-component's `Toolbar`
  sets at its Small size" (`:998-999`);
- **adds `min_h(bar_height)` only where the theme states one.**

On 0.8.1, `h_auto` still overrides the height, but nothing resets the new minimum. So where
`toolbar.bar_height` is unstated, upstream's 2 rem minimum stays. The unstated case is
kde-breeze and adwaita, whose presets say "sizes to content" (`kde-breeze.toml:340`).

**Measured in the trial:** the Buttons page's component Toolbar (`SC/demo.rs:1427`) under six
presets, with a throwaway probe test:

| Preset | `bar_height` | Toolbar height | Button height |
|---|---|---|---|
| kde-breeze | none | 47 px | 34 px |
| adwaita | none | 38 px | 26 px |
| windows-11 | 48 | 48 px | 27 px |
| macos-sonoma | 48 | 48 px | 27 px |
| material-3 | 64 | 64 px | 44 px |
| ios-26 | 64 | 64 px | 44 px |

In every case the content and padding are already taller than 32 px, so **no pixel changes
today**. But two statements are no longer true in general, and a smaller body font or a
denser theme would show it:

- the connector's contract, "the height is the content's" (`CN/README.md:365` and the
  geometry doc);
- the Widget Info claim at `SC/info/buttons.rs:902` ("h_8 … the height always, by the
  content's").

The chrome's toolbar (`SC/chrome.rs:247`) is our own row, not upstream's `Toolbar`, so it is
not affected.

**Why no test failed:**

- The showcase test `the_toolbar_is_the_models_toolbar` (`SC/tests.rs:1398`) holds the bar to
  `bar_height` where one is stated, and says of the unstated case "nothing to hold it to"
  (`:1412`).
- The connector's unit test `toolbar_carries_the_models_toolbar` (`CN/src/geometry.rs:2811`)
  asserts `min_size.height == None` there. On 0.8.1, that `None` is the leak itself: it leaves
  upstream's minimum in place.

### 1.6 Dialog: an exit animation, and the curve moved

In 0.8.1 a closing Dialog animates out (#3603):

- **`Dialog::exit_duration`** (`GC/dialog/dialog.rs:558-568`): zero under reduced motion and
  for `DialogEntrance::None`; `ANIMATION_DURATION` (0.25 s) for SlideDown; the motion tokens'
  `duration_normal` otherwise.
- **`Dialog::motion_transition`** (`:570-580`) builds a `gpui_base::motion::Transition`.
  - SlideDown keeps its literal `cubic_bezier(1./3., 0.72, 2./3., 1.)`.
  - The other entrances sample `easing_enter` and clamp it to 0–1 (`:578`). In 0.8.0 that
    clamp sat in `render` (0.8.0 `:664`).
- **The Dialog no longer uses gpui-pre's `AnimationExt`** (the Sheet still does,
  `GC/sheet.rs:239`).
- The closing dialog stays in the stack, takes no input, and its backdrop fades through a new
  child that still reads `theme().overlay`.

**The connector's behaviour stays right:**

- `geometry::dialog_entrance` chooses the entrance, and the exit follows it.
- Reduced motion zeroes the exit as it skips the entrance.
- The showcase tests that close dialogs pass on both cores. They close them under reduced
  motion (P8).

**But some text is false or out of date:**

- The Dialog's Widget Info (`SC/info/chrome.rs:1334-1337`) cites "gpui-pre/elements/animation.rs,
  AnimationExt", and "dialog/dialog.rs, Dialog::render, line 664" for the clamp. Both are now
  false.
- The same claims describe only the entrance ("slide and fade in"). That is true but now
  incomplete.
- The connector's dialog doc citations (`CN/src/geometry.rs:572-624`, `:1176`, `:1400`) point
  at lines 0.8.1 moved or removed. Among them is "Reduced motion skips every entrance upstream
  (`:621-623`)", a rule that now lives in `exit_duration`.

### 1.7 Other behaviour changes that read no theme field

All of these are in non-test code. None needs a connector change:

- **Table** (`GC/table/state.rs:1527-1532`): `Column::text_center` and `text_right` are now
  applied. In 0.8.0 they were stored and never read. The showcase sets no column alignment.
- **TimeField** (`GC/time/time_field.rs`): `px_1()` moved from the frame to the inner field,
  which is now `size_full()`. The inset is the same. The showcase refines no TimeField padding
  (`SC/demo.rs:4543`).
- **Scroll areas** (`GC/scroll/scrollable.rs:163-170`, `:191-206`): the content no longer
  inherits the caller's max size on the scrolled axis. The wrapper is now a flex column (or a
  flex row, for a horizontal area). The showcase tests pass. The parity run and the
  screenshots are where a layout change would show (P11).
- **ColorPicker hex** (`GB/color_picker.rs:71`): channels are now rounded, not truncated, so
  50 % alpha reads `80`, not `7F`.
- **Input:** tab characters align to tab stops; a focused input asks its scroll containers
  to reveal it.
- **Command:** no empty-list padding while loading.
- **Markdown tables:** sparse rows are no longer padded.
- **Focus:** a disabled focused Button keeps its Tab position, a click on blank Toolbar space
  no longer takes focus, and the focus trap stops at a repeated stop.
- **Sheets and notifications** stay inside `window.fully_visible_bounds()`, which is the whole
  window on a desktop.
- **The gpui-base parts the connector builds on:**
  - the headless Tabs, slider and progress parts of the `widgets` feature did not change;
  - `scrollbar.rs` only renames `ScrollWheelEvent`;
  - `resizable/` reads the drag state when events arrive (#3629), a behaviour fix with no theme
    read.

### 1.8 taffy 0.15 on gpui-pre, and the three upstream-issue drafts

`docs/todo.md` holds three issue texts that are ready to paste, which the maintainer has not
filed yet:

**1. "On gpui-pre a labelled `Separator` draws its label at the line's end."** The draft's
whole cause is taffy 0.13.0 giving the free space twice: once to the auto margins, then again
to `justify-content`. taffy 0.14 does not. taffy 0.15.0 keeps the fix: "The auto margins have
absorbed all of the free space, leaving none for `justify-content`" — `free_space = 0.0`
(`taffy-0.15.0/src/compute/flexbox.rs:2274-2275`).

- gpui-pre 0.3.10 builds on taffy 0.15.0, so the label should now be centred on both cores.
- **Not yet seen on screen**: the trial ran no capture.

**2. "gpui-pre clips a rounded box's children to a rectangle"** still holds. `ContentMask`
still says "only rectangular content masks are supported" (`GP/window.rs:2239-2246`). The
draft's line numbers moved.

**3. "A non-panicking `TextSystem::try_resolve_font`"** still holds. `resolve_font` (`GP/text_system.rs:352`)
still panics where nothing resolves (`:621`), and gpui-fast does the same. The draft's line
numbers moved.

**The other open gpui-fast entries** are unchanged by 0.1.12:

- `gpui-fast-web` 0.1.12 still pins `unicode-properties = "=0.1.3"`;
- `Window::painted_quads()` still returns the scene's own quads only (`GF/window.rs:2925-2927`).

### 1.9 The text system, and the showcase's inspector

`TextSystem`'s font cache was rewritten in gpui-pre 0.3.10:

- **Each font now maps to a `CachedFont`**: `Direct(id)`, `Fallback(id)` or `Missing`
  (`GP/text_system.rs:244`).
- **`resolve_font_uncached` stores fonts by role** (`:570`): the fallback family as `Direct`,
  and the font asked for as `Fallback`.
- **`get_font_for_id` reads only `Direct` entries** (`:336`).

Two consequences for the showcase:

- **`DrawnFamilies::of`** (`SC/inspector.rs:310`) calls `resolve_font` and then
  `get_font_for_id` to name the family gpui draws a theme font in. Its reasoning holds:
  - 0.3.9 cached each family under its own name, the fallback family as `Ok(id)` and a missing
    family as an error (0.3.9 `resolve_font`, `font_id`).
  - 0.3.10 caches the fallback family as `Direct(id)` and the asked family as `Fallback(id)`,
    and `get_font_for_id` reads `Direct` only.
  - Either way the id reads back as the family that was drawn.
  What changes is its doc (`:292-306`): the cited lines, and its description of the cache's
  type (`Result` → `CachedFont`).
- **`the_fallback_stack_copy_is_gpuis`** (`SC/tests.rs:10370`) fails by design until its
  version and lines are re-cited (§1.2).

gpui-fast 0.1.12 changed `text_system.rs` and `fast/text.rs` too. Its lines are re-read in P9.

### 1.10 New public API

| API (0.8.1) | Theme data a connector could feed? |
|---|---|
| `Dialog::centered`, `AlertDialog::centered` (`GC/dialog/dialog.rs:495`, `alert_dialog.rs:276`) | A flag: vertical placement. native-theme has no field for it, and `docs/platform-facts.md` no row (P13). |
| `TextView::inline_code_style` / `list_style` / `blockquote_style(StyleRefinement)` (`GC/text/compat.rs:114`, `:121`, `:127`) | A style seam: inline code's font, colours, border, radius and padding; list markers; blockquote border and fill. The model has `defaults.mono_font`, which the connector already feeds as `mono_font_family` (`CN/src/config.rs:65`) and which inline code falls back to. It has no inline-code, list-marker or blockquote fields (`docs/property-registry.toml`) (P13). |
| `TextView::image_source`, `Button::track_focus(&FocusHandle)`, `HoverCard::trigger_style` (layout only), `TableDelegate::cell_label`, `InputState::set_scrollbar_renderer` / `EditorScrollbarContext`, the Diff highlighter and parser options | None: application behaviour, accessibility, or a renderer hook. gpui-component never calls the scrollbar hook itself, so the default editor scrollbar, which the connector themes, is unchanged. |

### 1.11 Citations

**Counted by the gates** (§1.2): 125 of 821 colour claims and 14 of 966 prose citations no
longer point at what they cite. Colour claims by file:

| File | Claims |
|---|---|
| gpui-component `button/button.rs` | 74 |
| gpui-component `diff/mod.rs` | 16 |
| gpui-component `input/input.rs` | 14 |
| gpui-component `table/state.rs` | 12 |
| gpui-component `dialog/dialog.rs` | 4 |
| gpui-component `time/time_field.rs` | 2 |
| gpui-component `sheet.rs` | 2 |
| gpui-base `input/base/element.rs` | 1 |

**The upstream-diff review** found every cited field still read in the same file. It also found
two claims whose **text**, not just line, is false: §1.5 and §1.6.

**This time gpui-pre and gpui-fast are in scope too.** Both changed, so their citations move:

- G3's K3 left gpui-pre's as they were, since it did not change then.
- The cited gpui-pre files that changed: `elements/div.rs`, `window.rs`, `text_system.rs`,
  `platform/keystroke.rs` and `platform.rs`. Among them is `SC/tests.rs:2377-2379`'s "gpui-pre
  keystroke.rs:143-150", which is now false: `secondary` needs the OS.
- `elements/animation.rs`, `styled.rs` and `text_system/line.rs` did not change.

---

## 2 — Decisions

### P1 — Floors: GPUI Kit 0.8.1 and gpui-pre 0.3.10 in v0.7.0

**Options:**

- **(a) Ship v0.7.0 on 0.8.0 as it is.**
- **(b) Floors 0.8.1** (`gpui-component`, `gpui-base`, `gpui-kit` and the dev `gpui-kit`), the
  `gpui` alias to `0.3.10`.
- **(c) Pin `=0.8.0`.**
- **(d) Wait for 0.8.2.**

**(b).** The reasons:

- **Users would run 0.8.1 anyway.** v0.7.0 is unreleased and asks for `^0.8.0`, so every
  fresh resolve of it takes 0.8.1 today.
- **(a) ships claims and tests of a version a fresh resolve no longer gets** (measured, §1.2):
  - the 125 colour claims and 14 prose citations that moved;
  - the README's *Verified against* line (`CN/README.md:95`);
  - the toolbar contract of §1.5.
  Users would run code paths that no test of ours ran on their version.
- **(c)** is the exact pin in a library that the manifest's own comment rules out
  (`CN/Cargo.toml:60-62`). It would conflict with any application on 0.8.1.
- **(d)** has no date. 0.8.1 is the version users get now.
- **Nobody is left behind.** No published crate asks for 0.8.0, and 0.6.1 stays on `^0.7.1`.

**The `gpui` alias moves to `0.3.10`:** gpui-component 0.8.1 pins `=0.3.10`, so 0.3.10 is
the only version that resolves. A caret of 0.3.9 would state a floor nothing can build on. It
stays a caret (`:60-62`).

**MSRV stays 1.95.0, measured** (§1.2). The comment at `CN/Cargo.toml:6-11` gains the new
measurement: 0.3.10's `profiler.rs:473`.

### P2 — Lockfile: the scoped update the trial measured

After P1's manifest change, run `cargo update -p gpui-kit -p gpui-component -p gpui-base`,
which changes exactly the packages of §1.2.

- **`cargo audit` on the new lock is the gate.** Its non-GPUI closure gains only optional
  lock entries (proptest's), but the run, not the expectation, decides.
- **`.cargo/audit.toml`'s ignore comments are re-checked against the new lock**: they name
  the crates that reach each advisory ("gpui-component 0.8.0", "gpui-pre 0.3.9"). The
  comments are re-pointed with `cargo tree -i`. An entry the new lock no longer reaches is
  removed, which is that file's own rule ("remove the entry when the dependency moves").
- `cargo tree -p native-theme-gpui -e normal` before and after goes in the commit body.
- **A fresh resolve is not taken.** It would mix an unreviewed refresh of everything else into
  this change. That is G3's K2 reasoning, still valid.

### P3 — The showcase's key bindings: `KeyBinding::load`, so no binding can panic

**Options:**

- **(a) Upstream's pattern**: `KeyBinding::new(if cx.platform_info().is_mac() { "cmd-q" } else
  { "ctrl-q" }, …)` (`GC/diff/mod.rs:77`).
- **(b) `KeyBinding::load("secondary-q", Box::new(Quit), None, false, None, cx)`**, which
  returns a `Result`.
- **(c) `KeyBinding::new` with a literal chosen by `cfg!(target_os = "macos")`.**

**The arguments:**

- **(a) and (c) keep a call that panics on a parse error.** This change happens because such
  a call started to panic: its literal was valid, and only upstream's grammar kept it so.
  Explicit `cmd-` / `ctrl-` literals may be less likely to change than `secondary-` was, but
  nothing guarantees it.
  - (a) and (c) are each one line per binding.
  - (c) also states the OS a second way, beside gpui's runtime `platform_info()`. The two agree
    on every native target the showcase builds for.
  - Either option would need an allow under P6's gate.
- **(b) has no panic path.** Its costs:
  - **A longer call**, with a `Result` to handle.
  - **A `cx`:** `macos_bindings` gains an `&App` parameter. Its plain `#[test]`,
    `the_macos_menus_keys_are_the_higs` (`SC/tests.rs:2224`), becomes a `#[gpui::test]`.
  - **On Windows the binding goes through the platform keyboard mapper.** `new` maps through
    `DummyKeyboardMapper`, which changes nothing (`GP/platform/keyboard.rs:29-36`). `load`
    maps through `cx.keyboard_mapper()`:
    - Linux's is the same `DummyKeyboardMapper` (`gpui-pre-linux/src/linux/platform.rs:578-580`);
    - macOS's mapper changes nothing with `use_key_equivalents = false`
      (`gpui-pre-macos/src/keyboard.rs:33-46`);
    - Windows' mapper translates the key through its virtual-key code even then
      (`gpui-pre-windows/src/keyboard.rs:39-60`).
    With `use_key_equivalents = false` it looks the key up in the current layout's own table
    and maps the virtual key back through the same layout (`:139-145`). That is a round trip,
    and it is the translation gpui applies to every binding loaded with `load`, which
    gpui-kit's own script API uses (`crates/shell/src/engine/quickjs/window_api.rs:258`).
  - **It keeps one spelling of the showcase's modifier, `secondary-`**, shared with the Kbd
    samples and the Save tooltip (P4). Their tests already check that they agree
    (`SC/tests.rs:17424`).

**(b), for all eight bindings: the four of `app::init` and the four of `macos_bindings`
(`SC/app.rs:178-181`, `:203-206`).** The project's rule is no runtime panic, and only (b)
meets it without an exception.

**The Windows difference is an accepted risk, and it is not measured.**
- No test can see it: the test platform's mapper is `DummyKeyboardMapper`
  (`GP/platform/test/platform.rs:484-486`).
- CI's Windows leg builds the showcase but runs no showcase test (`ci.yml`,
  `gpui-fast-build`).
- It is accepted because the mapping is gpui's own round trip for loaded bindings, and the
  worst case is one showcase shortcut on Windows layouts that map a key differently, not the
  library.

**How a `load` error is handled:** the binding is left out. No panic, and no fallback to a
guessed key. `app::init` returns nothing, so there is no caller to hand the error to.

**A left-out binding cannot pass unnoticed:**

- for `app::init`'s four, the binding table test (`SC/tests.rs:2350-2375`), which looks up each
  action's highest-precedence binding, fails for any action that lost its binding;
- for the macOS four, `the_macos_menus_keys_are_the_higs` does.

Both run on every host. With all eight on `load`, the showcase's non-test code calls no
`KeyBinding::new` at all, which is what P6's gate then enforces.

### P4 — The Kbd samples and the Save tooltip read the OS from gpui

- **`demo::kbd`** (`SC/demo.rs:6478`) becomes `Keystroke::parse_with_os(keys,
  cx.platform_info().os)`. It already has `cx`.
- **`save_tooltip()`** (`SC/pages/feedback.rs:141`) takes `&App` and does the same.
- `KBDS` and `SAVE_KEYS` keep their `secondary-` spelling.

**Why `cx.platform_info().os`:** it is the OS gpui's own parser takes, in `load` too (§1.3).
`Kbd::format` then draws the parsed keystroke by the compile target, `cfg!(target_os =
"macos")` (`GC/kbd.rs:105-151`). The two agree on every native target the showcase builds for.

**The visible result is unchanged:** "Ctrl+S" off macOS, "⌘S" on it, and four Kbds again on
the Typography page.

### P5 — Test code: migrated as upstream migrated its own, and the one test that would go vacuous made strict

**The mechanical part:**

- `activate_key` (`CN/src/widgets/tests.rs:1714`) parses with `cx.update(|_, cx|
  Keystroke::parse_with_os(key, cx.platform_info().os))`, as gpui-component's own
  `activate_key`s do in 0.8.1 (`GC/checkbox.rs`, `switch.rs`, `button/toggle.rs` tests).
- `scroll_at` (`SC/tests.rs:416`) builds a `gpui::ScrollEvent` with `source:
  ScrollSource::Pointer`, a wheel. It is named in full, not through `..Default::default()`,
  so the event says what it simulates.
- The `unparse()` calls take the test platform's OS, from the `cx` each test already has.
- The Kbd and tooltip tests (`SC/tests.rs:17424`, `:17453`) parse their `secondary-`
  constants with `parse_with_os`. They become `#[gpui::test]`s where they need a `cx` for the
  OS or for `save_tooltip(&App)`.

**`secondary(key)` (`SC/tests.rs:2380`) takes the OS and fails the test where the parse
fails, instead of returning `""`.** As it is, `no_inspector_toggle_remains` (`:2403`) would
pass vacuously (§1.3). A helper whose failure silently satisfies an assertion is a wrong
safety net.

**No new cross-OS test.** `set_platform_info` (§1.3) could run the bindings under Mac on a
Linux host. But that half would test gpui's parser rather than our code. The drawn half cannot
follow: `Kbd::format` draws by the compile target (`GC/kbd.rs:105-151`), so under a Mac
platform on Linux the tooltip is not "⌘S". Each host's own tests keep checking that host's
half, as today.

### P6 — A gate for dependency functions that panic on their input

**Options:**

- **(a) Nothing**: the tests caught it.
- **(b) A pattern list in `scripts/check_panic_patterns.py`.**
- **(c) clippy's `disallowed-methods`**, in a new `CN/clippy.toml`.

**Where the file goes.** clippy reads the `clippy.toml` nearest the package's manifest. So a
file in `CN/` applies to the gpui connector, its tests and its showcase, and to no other crate.
A workspace file would also apply to crates that do not depend on GPUI, where the configured
paths name nothing.

**(c).** The reasons:

- **(a)** catches the case only at test time, and only where a test reaches the call. The
  Kbd row was caught only because a test happens to parse those constants.
- **(b)** is a regex over `src/` (check_release passes each member's `src/`). It does not see
  `examples/`, where every one of today's sites is, and it cannot tell `gpui::KeyBinding::new`
  from another type's `new`.
- **(c)** is type-aware, and the ordinary clippy runs that deny warnings already lint every
  target on both cores: CI's clippy job (`RUSTFLAGS: -Dwarnings`, `--all-targets`, once per
  core) and `check_release.sh`'s *clippy (-D warnings)* section. The canary's clippy does not
  deny warnings, and it runs on `main` anyway (§0).
  - It names the method and the reason in the error: "panics on a parse error; use
    `KeyBinding::load`".
  - A deliberate, guarded call stays possible: `#[allow(clippy::disallowed_methods)]` at that
    site, with its reason. One case is the inspector's `resolve_font`, behind its own guard
    (§1.9). The allow is visible in review.

**The list is an inventory, not a guess.** The candidates are every public function of
gpui-pre, gpui-fast, gpui-base, gpui-component and gpui-kit whose documentation says it
panics (`# Panics`, "Panics on/if/when", "will panic"): 84 such doc lines in 0.8.1 / 0.3.10 /
0.1.12. The sweep sorts them into two groups:

- **Listed: a panic that depends on an argument's value**, whether or not we call the
  function today, since a future call is what the gate is for. Examples:
  - `KeyBinding::new`'s parse error (`GP/keymap/binding.rs:32`);
  - `TextSystem::resolve_font`'s missing font (`GP/text_system.rs:351`);
  - `shape_line` on a text with a newline (`:668-670`);
  - a line boundary out of range (`GP/text_system/line.rs:280-281`);
  - gpui-base's non-finite or non-positive scroll-bounce parameters
    (`GB/scroll_bounce.rs:65`, `:93`) and motion ratio (`GB/motion.rs:484`).
- **Not listed: a panic on framework state or misuse**, with the reason recorded. Examples:
  - a global that was never set (`GP/global.rs:33`, `GP/app.rs:2520`);
  - a re-entrant update (`GP/app.rs:190-191`);
  - a wrong arena or App (`GP/window.rs:424`, `:488`);
  - the test and bench harnesses' assertions.
  These hold for every correct gpui program, and gpui's ordinary calls (`Theme::global`, an
  entity's `update`) would otherwise need an allow at hundreds of sites.

`KeyBinding::new` and `TextSystem::resolve_font` are known members of the listed group.

The sweep is repeated at each GPUI Kit upgrade, as one step of the upgrade's plan: a new
panicking function only enters through an upgrade.

**Done-criterion, mechanical:** a planted `gpui::KeyBinding::new("a", …)` in a non-test
showcase function fails `cargo clippy -p native-theme-gpui --all-targets -- -D warnings` on
both cores, and removing it passes. This proves that the configured paths resolve through the
`gpui_kit as gpui` alias and on GPUI Fast. That is not assumed, because clippy resolves a
disallowed path by its definition, and the two cores define it in different crates under the
same library name.

**Tests stay out of scope**, as for the other panic gates: a test may panic on purpose. clippy
applies the config to test code too, so the test modules that call such functions with literals
carry a module-level allow, with the reason. A module-level allow is visible in review, as a
site allow is.

### P7 — Toolbar: the unstated height is the content's again

**`geometry::toolbar`'s `None` arm resets the minimum to gpui's default**, `min_h` auto
(`GP/style.rs:789`, `min_size: Size::auto()`). That arm is the one with no `bar_height`.

- This restores what the connector states: where a theme states no bar height, the bar sizes
  to its content. Upstream's new 2 rem floor is a literal, not a theme value.
- An application that wants a floor still sets one after the refinement, as the doc already
  says.

**Text corrected:**

- the geometry doc (`CN/src/geometry.rs:998-999`): `min_h_8`, and both overrides named;
- the README row (`CN/README.md:365`);
- the Widget Info claim (`SC/info/buttons.rs:902`).

**The guard is the existing unit test's expectation.** `toolbar_carries_the_models_toolbar`
(`CN/src/geometry.rs:2811`) runs over every preset in both modes and asserts
`out.min_size.height == t.bar_height.map(..)`, i.e. `None` where no bar height is stated. A
`None` refinement leaves upstream's minimum in place, which is the defect. The expectation
becomes `Some(Length::Auto)` there. So the test fails until the fix lands and holds the fix
for every preset, whatever their content heights.

A showcase measurement would not guard it: §1.5 shows the content is taller than the floor in
all six presets measured, so such a test passes with or without the fix. The showcase test's comment "nothing
to hold it to" (`SC/tests.rs:1412`) gets a clause saying why the chrome's own row, which it
measures, is not upstream's `Toolbar`.

**Measured: no pixel changes today** (§1.5), so the screenshots do not move for this.

### P8 — Dialog: the claims say what 0.8.1 draws, entrance and exit

**The connector's behaviour is unchanged** (§1.6). The text changes:

- **The Dialog's Widget Info animation lines** (`SC/info/chrome.rs:1334-1337`):
  - drop the `AnimationExt` citation;
  - cite `Dialog::motion_transition` for the curve and the 0–1 clamp, and `exit_duration` for
    the duration;
  - say the dialog slides or fades *in and out* on the same curve and duration;
  - say that reduced motion shows and removes it at once.
- **The connector's dialog doc citations** (`CN/src/geometry.rs:572-624`, `:1176`, `:1400`)
  are re-pointed, and the reduced-motion sentence cites `exit_duration`.

No new test. The exit is upstream's, and upstream's own tests cover it. It is driven by the
entrance the connector already chooses and tests. The showcase test
`the_dialog_samples_draw_a_raster_sets_icon` (`SC/tests.rs:8798`) opens each dialog sample and
closes it with Escape, under reduced motion (`without_motion`), where the exit is zero. It
passes on both cores.

### P9 — Citations: G3's K3 method, with gpui-pre and gpui-fast in scope

**The K3 rule is used unchanged** (archived `todo_v0.7.0_gpui-kit-0.8.0-rationale.md`, K3):

1. unique text where the line is unique in both files;
2. else difflib's equal-block position where git's `histogram` line map agrees;
3. else set aside and read by hand.

It is the measured method (M8 T17: 1 961 numbers, 0 to fix after its fix round), and G3 used
it. The files in scope are K3's:

- `CN/`: sources, tests, examples, README and manifest comments;
- `docs/showcase-exceptions.toml`, which cites lines of `root.rs`, `input/input.rs`,
  `button/button.rs` and `scroll/scrollable.rs`, four files 0.8.1 changed;
- `scripts/connector_parity_exceptions.toml`;
- `tools/showcase-support/src`.

What changes is which upstream crates are in scope:

- **Both GPUI cores changed** (0.3.9 → 0.3.10, 0.1.10 → 0.1.12), so their cited files are in
  scope, along with gpui-component and gpui-base.
- So are the gpui-pre platform crates the inspector doc cites (`gpui-pre-wgpu`,
  `gpui-pre-apple`, `gpui-pre-windows` and their GPUI Fast twins), where their files changed.

**Read by hand, beyond what the script sets aside:**

- **The panels of widgets whose behaviour changed** (§1.6, §1.7): Dialog (with P8's rewrite),
  Table, TimeField, Sheet, Input. The Toolbar panel (§1.5) is P7's: its text changes together
  with P7's fix, and no gate reads it (its citation names a symbol that still exists).
- **The inspector's `DrawnFamilies` doc** (§1.9), rewritten against the new cache on both
  cores.
- **The `secondary` doc at `SC/tests.rs:2377-2379`.**
- **The three issue drafts** (P10, P11).
- **The connector's own doc-comment citations.** No gate covers these, which is the open `[1.0]`
  entry *gpui: a citation gate for the connector's doc comments*. As in G3, they are
  renumbered in the one script run and the set-aside ones are hand-read, never in a second run.

**Gate:** the two citation gates and `the_fallback_stack_copy_is_gpuis` pass on both cores.

### P10 — The other two issue drafts: re-cited, still to be filed

- **Rounded clip:** `GP/window.rs:2239-2246`, gpui-fast's `rounded_clip.rs` as re-read.
- **`try_resolve_font`:** `GP/text_system.rs:352`, the panic at `:621`, gpui-fast's as
  re-read.

The texts gain "0.8.1, gpui-pre 0.3.10, gpui-fast 0.1.12" and keep their substance. They stay
for the maintainer to file (an agent files nothing).

### P11 — taffy 0.15: the Separator entry closes once a capture shows it; parity and screenshots re-run

**Before the Separator draft is dropped:** on gpui-pre, the Layout page's "Section Break" has
to be centred on its line at rest (the same `--capture` comparison G3 Task 9 used), and the
GPUI Fast capture has to agree. Then the `todo.md` entry closes with a dated line naming
taffy 0.15.0's `flexbox.rs:2274-2275` and moves to `docs/archive/todo_closed.md`.

**No exception or claim names the label's position today.** The two exception files and
`SC/info/layout.rs` were checked, so nothing else changes for the label. Any exception the
parity run below shows to be unnecessary goes.

**A layout-engine bump can move other things too**, as can 0.8.1's scroll-area flex change
(§1.7):

- **the Linux parity run** (`scripts/check_showcase_parity.py`, `check_connector_parity.py`)
  runs on the new lock;
- **the gpui screenshots are regenerated** through the release scripts. That was a release
  gate already (`generate_assets_release.sh`, the PROVENANCE stamp), and it now also has to
  show the centred label.

**If the label is still at the end:** the entry stays open, the draft is re-cited to taffy
0.15.0, and this document's §1.8 is corrected.

### P12 — The `todo.md` entries 0.8.1 touches get dated lines

**Citations that name their version stay as they are**, in `docs/todo.md` and in
`ROADMAP.md`'s upstream list (`:137-171`). "`tab/tab.rs:958-966` at gpui-component 0.8.0" stays
true of 0.8.0. This is K3's rule for `docs/todo.md`: its entries are not renumbered.

**An entry gets a dated line** — *GPUI Kit 0.8.1 (2026-10-26, G4):* — **only where it waits
on upstream and 0.8.1 is the release it waited for:**

- **the Separator draft** (P11) and **the two other drafts** (P10);
- **`unicode-properties`:** still pinned by gpui-fast-web 0.1.12;
- **`painted_quads`:** unchanged in gpui-fast 0.1.12 (`GF/window.rs:2925`).

(The base-palette repair's entry needs none: the C round fixed it and closed the entry in
`e92c288a`, C-04.)

No other `[upstream]` entry is answered by 0.8.1:
- its non-test diff adds and removes no theme read (§1.1);
- `WindowBorder` gained only safe-area insets;
- the Input's placeholder change is accessibility only;
- the Dialog gained no `max_h`;
- the new public API (§1.10) adds no style seam beyond TextView's and the trigger-layout one.
The one partial answer, inline code's seam, is P13's.

### P13 — New upstream API: what is shown, mapped, or recorded

**`Dialog::centered`: recorded, not used.**

- Which vertical placement each platform gives an in-window modal dialog is a platform fact.
  `docs/platform-facts.md` has no row for it, and the model has no field (`[dialog]`,
  `docs/property-registry.toml:480-492`).
- Calling `.centered()` in the showcase without theme data would be a hard-coded look, which
  the connector rules forbid.
- Adding the field means rows for every platform, presets, inheritance and three connectors.
  That is a model change, not a dependency update, so it is not for the last step of v0.7.0.
- **What happens now:**
  - a `docs/todo.md` entry, *Dialog: the platform's vertical placement*, `[1.0]`, with the
    seam (`GC/dialog/dialog.rs:495`) and the facts it needs. The tag is `[1.0]`, not
    `[v0.7.x]`: `DialogTheme` is not `#[non_exhaustive]`
    (`native-theme/src/model/widgets/mod.rs:810-815`), so a new public field breaks struct
    literals and cannot go in a patch release;
  - the Dialog's Widget Info states placement as not themeable: upstream's default, anchored at
    its top a tenth of the viewport down unless the caller sets `margin_top`
    (`GC/dialog/dialog.rs:667-673`).

**`TextView` styles: recorded, not mapped.**

- The model has no inline-code, list-marker or blockquote fields.
- Inline code's font already follows `defaults.mono_font` through `mono_font_family`.
- The Tier U note in the TextView panel (`SC/info/typography.rs:375`, "accent again … Tier U")
  changes: upstream now has a seam (`inline_code_style`). The gap is the model's, not
  upstream's. The note says so and links the new `todo.md` entry, *Markdown: inline code,
  list markers and blockquotes in the model*, `[post-1.0]`. The tag is `[post-1.0]` because
  these are not platform controls, and `docs/platform-facts.md` has no rows a value could
  come from.

**Everything else in §1.10: nothing to do.** None of it carries theme data.

### P14 — Versions, CHANGELOG, README, docs

**Every statement of the stack as current moves to 0.8.1 / 0.3.10 / 0.1.12:**

- **`CN/README.md`:** the Compatibility table (`:76-79`), the *Verified against* line
  (`:95`, with the new date), the quick start (`:109`), the alias comment (`:144`), the
  *Backends* section (`:822-837`), and the 0.8.0 mentions in rows (`:29`, `:57`, `:365`).
  `compat::` checks the table and the quick start.
- **`docs/COMPATIBILITY.toml`:** through `scripts/update_compatibility.sh`.
- **`ROADMAP.md:72-73`.**
- **The manifest comments:** `CN/Cargo.toml:6-11`, `:56-62`, `:76-82`. The `image` comment
  still holds: gpui-pre 0.3.10 and gpui-fast 0.1.12 both ask for `image` `0.25.1`.
- **`.cargo/audit.toml`'s comments** (P2).

**CHANGELOG `[Unreleased]`:**

- **The *Changed* entry that states the floors ("requires gpui-component, gpui-base and
  gpui-kit 0.8.0 and gpui-pre 0.3.9 …") is rewritten in place.**
  v0.7.0 is unreleased, so the entry states what v0.7.0 requires. There is no migration note
  (pre-1.0).
- **The Kbd and toolbar fixes need no CHANGELOG entry of their own.** They repair what the
  bump would have broken, and no released version had the defect. The toolbar's contract
  ("sizes to content") is already stated.
- **The new P6 gate gets an `### Added` line**, `**Development**:`, as the other gates have
  (the entries for `scripts/check_docsrs.py`, `check_connector_parity.py`,
  `check_readme_versions.py`).

The `secondary` change is upstream's. The connector's public API does not change.

### P15 — Where G4 sits: after the C round, before the close-out, as one writer

`v0.7.0-rc1` is being written by the C round (`docs/todo_code-review-findings.md`, *C round*,
2026-10-26). G4 enters at a task boundary on a clean tree, as G3 entered between M9b's tasks
(K13).

**The shared files:** `Cargo.lock`, `CHANGELOG.md`, `docs/todo.md`,
`docs/todo_code-review-findings.md`, `docs/showcase-exceptions.toml`, the showcase. One writer
at a time keeps them consistent.

**The unit**, between which no other commit lands:

1. the bump (P1, P2), with the README's Compatibility table and quick start (from P14), since
   `compat::` fails on them the moment the manifest moves;
2. the migration (P3–P5);
3. the citations (P9), whose hand-read of the behaviour-changed panels includes P8's Dialog
   claims, so those claims are touched once.

The bump turns these gates red and the unit turns them green, and `check_release.sh --commit`
runs them.

**After the unit**, each its own task: P6, P7, P10–P13 and the rest of P14.

**Until the unit lands, nothing is red**: the branch's lock holds 0.8.0, and the canary
tests `main` (§0). The unit is the step that takes 0.8.1, and its gates pass with it.

---

## 3 — The maintainer's approvals

- **P1, the floors to GPUI Kit 0.8.1 and gpui-pre 0.3.10: approved** (2026-10-26).
- **P3, the showcase's key bindings through `KeyBinding::load`: approved** (2026-10-26). This
  includes the unmeasured Windows keyboard-mapper difference as an accepted risk.

## 4 — What was checked, and what was not

**Checked:**

- **The trial build and tests** (§1.2), on both cores, before and after the migration. That
  includes the measured 240-test panic with only `app.rs` unmigrated, and MSRV 1.95.0 and
  1.94.0.
- **The release notes and the full `v0.8.0..v0.8.1` diff of `crates/base/src` and
  `crates/component/src`** (non-test code), read for theme reads, literals and layout. The
  Toolbar and Dialog findings were then re-read in the source.
- **gpui-pre 0.3.9 against 0.3.10**: the manifest, `keymap/binding.rs`,
  `platform/keystroke.rs`, `platform_info.rs`, the test platform, `text_system.rs`,
  `window.rs`.
- **gpui-fast 0.1.10 against 0.1.12**: the list of changed files, plus `keymap/binding.rs`,
  `platform/keystroke.rs`, `text_system.rs`, `window.rs`.
- **taffy 0.15.0**: `compute/flexbox.rs`.
- **The toolbar probe** (§1.5), under six presets on gpui-pre.
- **A fresh resolve** of `gpui-component = "0.8.0"`, with and without `rust-version = "1.88"`
  (§1.2).
- **The keyboard mappers** of gpui-pre, its test platform, macOS and Windows (P3), and
  `Kbd::format`'s OS source (P4, P5).
- **The open `[upstream]` entries against the 0.8.1 diff** (P12), `DialogTheme`'s
  exhaustiveness (P13), `.cargo/audit.toml` (P2), and the citation-bearing files outside the
  gates (P9).

**Not checked; the plan's gates decide:**

- clippy with the new code;
- `scripts/check_release.sh`;
- the parity scripts and captures, which include the Separator (P11);
- `cargo audit`;
- the macOS and Windows targets;
- the toolbar probe on GPUI Fast;
- whether `disallowed-methods` resolves through the alias (P6's done-criterion);
- the Windows keyboard mapper's effect on the showcase's bindings (P3: accepted, and no test
  can see it).

**The trial is gone after this document.** Its worktree and its build folder under
`/run/media/tibi/work/cargo-build/` were removed. Its patches were trial-quality, not the code
P3–P5 specify.

## 5 — Out of scope

- **Using `Button::track_focus`, `HoverCard::trigger_style`, `TableDelegate::cell_label`,
  the Diff options or `set_scrollbar_renderer` in the showcase.** None of them carries theme
  data or changes how a theme reaches a widget.
- **The dialog-placement and Markdown model fields** (P13): recorded, for after v0.7.0.
- **The deferred upstream changes** (`MotionTokens` fields, the Textarea type). They come
  with GPUI Kit 0.9 and are that version's work.
