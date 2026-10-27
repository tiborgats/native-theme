# v0.7.0 — GPUI Kit 0.8.1 (G4): Rationale

**Status:** implemented and archived (2026-10-27; the spec's *As built* says how). Written
2026-10-26, the day GPUI Kit 0.8.1 was published.
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
  (That canary has failed on every scheduled run from 2026-10-20 to 2026-10-25, in its *Test*
  and *Widget coverage* steps. Run 40232134026, 2026-10-25, shows why, and it is unrelated to
  0.8.1:
  - the fresh resolve moves `main` from gpui-component 0.7.1 to 0.7.3;
  - so the released showcase's gpui citation gates fail: "140 of 738 colour claims", "1 of
    745 prose citations";
  - the widget coverage gate finds 0.7.3's `Diff` neither shown nor excepted ("gpui: Diff").

  The step runs under `bash -e`, so it stops at that first failing line, and the later gpui
  and egui test lines never ran there. These failures last until v0.7.0 reaches `main`.)

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
- **Both GPUI cores move to taffy 0.15**, the layout engine:
  - gpui-pre 0.3.10 from 0.13.0: `[dependencies.taffy] version = "0.15.0"`, a caret
    (`Cargo.toml:524-525`, was `"0.13.0"`);
  - gpui-fast 0.1.12 from `=0.14.0`: `version = "=0.15.0"`, exact (`Cargo.toml:344-345`).
- **taffy 0.15.1 exists** (published 2026-10-25). It changes `compute/flexbox.rs`,
  `block.rs`, the alignment, grid and float code and `style/flex.rs`. The flexbox changes
  include box sizing, stretching, empty containers and the hypothetical cross size. So the two
  cores now resolve differently:
  - **an application on the default core gets 0.15.1.** Measured 2026-10-26: a scratch
    project on `gpui-component = "0.8.1"` locks taffy 0.15.1 and gpui-pre 0.3.10, with no
    gpui-fast;
  - **our workspace lock holds 0.15.0** for both cores. The connector's `gpui-fast` feature
    puts gpui-fast's `=0.15.0` into the one resolve, and the dispatched canary resolves the
    same workspace. So no G4 check, CI run or canary run sees 0.15.1 (P11).

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
- cocoa 0.26.0 and cocoa-foundation 0.2.1 are removed, and bit-set 0.9.1 becomes 0.11.1
  (`gk081-lockdiff.txt`).
- **proptest 1.11 → 1.12**, with proptest-macro 0.6, rand_xorshift 0.5, bit-vec 0.10.1,
  core_detect and a `borsh` entry.
  - gpui-pre 0.3.10 asks for proptest optionally (`attr-macro`, `cargo tree -i proptest
    -e all`).
  - native-theme also has `proptest = "1.11"` as a dev-dependency (`native-theme/Cargo.toml:140`).
    So **native-theme's property tests run on proptest 1.12** after the update: the one change
    outside the gpui connector's build, and a test-build one.
- `image` stays one copy, 0.25.10.

**Compile results:**

- `cargo check -p native-theme-gpui --all-targets`, on gpui-pre with the default features and
  on GPUI Fast with `--features gpui-fast,native,watch` (`trial.sh`):
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
  - This was run by hand with `--ignore-rust-version`, since `trial2.sh`'s own line stopped at
    the `rust-version` check. The command and its output are in `gk081-msrv-1.94.txt`.
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
| `SC/pages/typography.rs:97` `KBDS` (`"secondary-c"` …) through `SC/demo.rs:6478` `Keystroke::parse(keys).ok()?` | compiles, every parse fails, `filter_map` drops all four Kbds (`SC/pages/typography.rs:196`) | **the Typography page's Kbd row is silently empty**; `the_kbds_take_the_showcases_modifier` (`SC/tests.rs:17424`) fails, but only because it parses `KBDS` itself: no test checks that the row is drawn (P5) |
| `SC/pages/feedback.rs:136` `SAVE_KEYS = "secondary-s"` through `:142` | compiles, the parse fails, the fallback prints the raw literal | **the Save tooltip reads "Save file (secondary-s)"**; the tooltip test (`SC/tests.rs:17452`) fails on it |
| `SC/tests.rs:2380` `fn secondary(key)`, `Keystroke::parse(&format!(..))` | does not compile: its `unparse()` now takes the OS (the error at `:2382`), and `parse` now takes a `&'static str`, which `&format!(..)` is not | the trial's log shows only the first error: the compiler skips the borrow check of a function that already has a type error. Migrated to `parse_with_os` with an OS argument, it compiles, but its `unwrap_or_default()` returns `""` on a parse failure, so **`no_inspector_toggle_remains` (`:2403`) would pass while checking nothing**: it compares every binding with an empty string |
| `SC/tests.rs:416` (`ScrollWheelEvent`), `:2229` (`parse(keys)`), `:2363`, `:2382`, `:2414`, `:5345` (`unparse()`); `CN/src/widgets/tests.rs:1714` (`parse(key)`) | 7 compile errors in the trial's log, 1 deprecation warning; `secondary`'s borrow error at `:2381` would follow once `:2382` is fixed | `cargo check --all-targets` fails on both cores |

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
`toolbar.bar_height` is unstated, upstream's 2 rem minimum stays. **14 of the 17 bundled
presets state none**; only macos-monterey (38), material-3 (64) and windows-11 (48) do
(`native-theme/src/presets/*.toml`, `bar_height_px`). kde-breeze's says why: "sizes to
content" (`kde-breeze.toml:340`).

**Measured in the trial:** the Buttons page's component Toolbar (`SC/demo.rs:1427`), with a
throwaway probe test:

| Preset | `bar_height` | Toolbar height | Button height |
|---|---|---|---|
| kde-breeze | none | 47 px | 34 px |
| adwaita | none | 38 px | 26 px |
| windows-11 | 48 | 48 px | 27 px |
| material-3 | 64 | 64 px | 44 px |

- On kde-breeze and adwaita the content and padding are already taller than 32 px, so the
  floor changes no pixel there.
- The other 12 presets without a bar height were not measured. Whether the floor shows on
  any of them is not known.
- The probe also asked for "macos-sonoma" and "ios-26", which are not preset keys. The probe
  kept the previous preset and printed its numbers again. The test helper's silence about an
  unknown name is P5's.

Two statements are no longer true in general:

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
- The connector's two unit tests assert `min_size.height == None` there:
  - `toolbar_carries_the_models_toolbar` (`CN/src/geometry.rs:2811`), over every preset;
  - `toolbar_leaves_min_height_unset_without_a_bar_height` (`:2984`), on one theme with
    `bar_height` set to `None`. Its doc says "the toolbar is its content's height, with no
    floor".

  On 0.8.1, that `None` is the leak itself: it leaves upstream's minimum in place.

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
- **`resolve_font_uncached` stores fonts by role** (`:570`). A font that loads is cached as
  `Direct`. A font that does not load is cached as `Fallback`, with the id of the first
  stack family that does, and that family is cached as `Direct`.
- **`get_font_for_id` reads only `Direct` entries** (`:336`).

Two consequences for the showcase:

- **`DrawnFamilies::of`** (`SC/inspector.rs:310`) calls `resolve_font` and then
  `get_font_for_id` to name the family gpui draws a theme font in. Its reasoning holds:
  - 0.3.9 cached each family under its own name, the fallback family as `Ok(id)` and a missing
    family as an error (0.3.9 `resolve_font`, `font_id`).
  - 0.3.10 caches a family that loads as `Direct(id)`. A family that does not load is cached
    as `Fallback(id)` of the stack family drawn instead, which is `Direct` under its own name.
    `get_font_for_id` reads `Direct` only.
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
  `platform/keystroke.rs`, `platform.rs` and `taffy.rs`. `taffy.rs` is cited at
  `CN/src/widgets/separator.rs:92` and on twelve lines of `docs/showcase-exceptions.toml`
(`:307` to `:959` at `7f011f00`); its lines move by
  +11 after `:264`. Among them is `SC/tests.rs:2377-2379`'s "gpui-pre
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

- **`cargo audit` on the new lock is the gate.** Outside the GPUI crates, the lock changes only
  proptest and its dependencies (native-theme's test build, §1.2), and loses cocoa,
  cocoa-foundation, a wgpu copy and two taffy copies. The run, not the expectation, decides.
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

**How a `load` error is handled:** the binding is left out, with one line on stderr — the
showcase's existing pattern for an input it refuses (`SC/app.rs:1416`). No panic, and no
fallback to a guessed key. `app::init` returns nothing, so there is no caller to hand the
error to.

**A left-out binding cannot pass unnoticed:**

- for `app::init`'s four, the binding table test (`SC/tests.rs:2350-2375`), which looks up each
  action's highest-precedence binding, fails for any action that lost its binding;
- for the macOS four, `the_macos_menus_keys_are_the_higs` does.

Both run wherever the showcase tests run: Linux, in CI's gpui test jobs and in
`check_release.sh`. CI's macOS and Windows jobs only build the showcase (`ci.yml`,
`gpui-fast-build`). So the macOS meaning of `secondary-` is tested on Linux, under a Mac
platform identity (P5). With all eight on `load`, the showcase's non-test code calls no
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

**A parse failure is reported, not only absorbed.** Today `demo::kbd` drops a Kbd that does
not parse (`.ok()?`), and the tooltip prints the raw literal. That is how 0.3.10's change
would have gone unseen but for a test. Both keep their fallback, since a showcase must still
draw, and each writes one stderr line naming the keys, as P3's `load` helper does.

**The Kbd row gets a test that it is drawn.** `the_kbds_take_the_showcases_modifier` parses
`KBDS` itself and never calls `demo::kbd`. So a `demo::kbd` left on `parse`, or migrated
wrongly, would empty the row again with only a stderr line to show it. A showcase test opens
the Typography page and finds each of the four `KBDS` ids laid out (`bounds_of`). It is one
short test, and it checks the visible result that the Kbd change exists for. The tooltip needs
no such test: its test calls `save_tooltip` itself (P5).

### P5 — Test code: migrated as upstream migrated its own, and the one test that would go vacuous made strict

**The mechanical part:**

- `activate_key` (`CN/src/widgets/tests.rs:1714`) parses with `cx.update(|_, cx|
  Keystroke::parse_with_os(key, cx.platform_info().os))`, as gpui-component's own
  `activate_key`s do in 0.8.1 (`GC/checkbox.rs`, `switch.rs`, `button/toggle.rs` tests).
- `scroll_at` (`SC/tests.rs:416`) builds a `gpui::ScrollEvent` with `source:
  ScrollSource::Pointer`, a wheel. It is named in full, not through `..Default::default()`,
  so the event says what it simulates.
- The `unparse()` calls take the test platform's OS, from the `cx` each test already has.
- The Kbd and tooltip tests (`SC/tests.rs:17424`, `:17452`) parse their `secondary-`
  constants with `parse_with_os`. They become `#[gpui::test]`s where they need a `cx` for the
  OS or for `save_tooltip(&App)`.

**`secondary(key)` (`SC/tests.rs:2380`) takes the OS and fails the test where the parse
fails, instead of returning `""`.** As it is, `no_inspector_toggle_remains` (`:2403`) would
pass vacuously (§1.3). A helper whose failure silently satisfies an assertion is a wrong
safety net.

**The bindings are tested under macOS too, on Linux.** No showcase test runs on macOS (P3),
so without this the half that macOS users get, `secondary-` as Cmd through `load`, is tested
nowhere. The options:

- **(a) Accept the gap.** It costs nothing now, but the half that differs is left unchecked.
- **(b) A Linux test under a Mac platform identity.** `TestAppContext::set_platform_info`
  (`GP/app/test_context.rs:127`, the same in gpui-fast) is set to `PlatformInfo { os: Mac,
  target: Native }` before the showcase opens. The binding table of `the_menus_run_actions` is
  then checked against `secondary(key, Mac)`, `cmd-q` and the rest.
  - It checks the showcase's choice, not only gpui's parser: a binding that took its key from
    the compile target, or a literal `ctrl-`, fails it.
  - It is one short test, with the table's loop shared as a helper between the two tests.
- **(c) Run the showcase tests on CI's macOS job.** That is a whole second test suite on a
  paid runner, for one table.

**(b).** Its limits are stated in the test's doc:
- `Kbd::format` draws by the compile target (P4), so "⌘S" stays macOS-only and is not tested
  here;
- the macOS menu-bar bindings are bound only where the compile target is macOS (`cfg!`,
  `SC/app.rs:184`), so they are checked by `the_macos_menus_keys_are_the_higs`, not by this
  test.

**`use_preset` fails the test where the preset does not install.** Found during this work:
the showcase's test helper ignores `apply_theme_by_name`'s `bool` (`SC/app.rs:1861-1866`).
So a test that names a preset that does not exist runs, unnoticed, against the theme installed
before. That is how the §1.5 probe measured "macos-sonoma" and "ios-26".

- **The calls:** 96 calls of `use_preset` and 4 of `use_preset_scaled`, which calls it.
- **Two of them fail on purpose:** `the_theme_error_alerts_icon_follows_the_chosen_set` and
  `a_theme_error_is_an_alert` pass `"no-such-preset"` to provoke a theme error
  (`SC/tests.rs:8383`, `:12934`).
- **The fix:** `use_preset` asserts the install, and a second helper, `use_missing_preset`,
  asserts that the install failed. The two error tests call it. Every other call passes a
  literal or a name from a preset list, and none passes `"default"`, whose OS read may
  legitimately fail on a runner.
- **Why here:** an assertion costs nothing where the names are right, and shows any test that
  has been checking the wrong theme. It is the same wrong-safety-net class as `secondary`, in
  the file this change already edits, so it is done here rather than recorded for later.

**`set_preset` gets the same check.** It is the other preset helper (`SC/tests.rs:10878`,
28 calls). It dispatches `SetPreset`, the path the theme settings and the command palette
take, and never checks that the theme installed.
- Its literal calls name kde-breeze, adwaita, nord, material-3 and catppuccin-mocha, all
  bundled presets today. Two pass a variable: `"material-3"`, and the first key of
  `list_presets_for_platform()`. There are also six `"default"` calls and two `"no-such-preset"` calls, made
  to provoke the error (`:11695`, `:11740`).
- **The options:**
  - **(a) Record it for later.** That contradicts the reason above.
  - **(b) Assert the install.** `apply_theme_by_name` sets `Showcase::current_theme_name` to the
    name only where the install succeeded (`SC/app.rs:1883-1887`, a `pub(crate)` field). Every
    caller already holds the showcase entity.
- **(b).** `set_preset` takes the showcase and asserts `current_theme_name == key` for every key
  but `"default"`. The OS read behind `"default"` may legitimately fail on a runner, so those
  six calls stay unasserted, as `use_preset` has no such call to assert.
- The two error calls go to a `set_missing_preset`, which asserts the name did not change, as
  `use_missing_preset` does. Their tests already check the error alert.

**Test code is written so the project's panic hook passes it.**
`.claude/hooks/no-runtime-panics.sh` refuses an edit whose new text holds `.unwrap()`,
`.expect(`, `panic!(`, `unreachable!(`, `unimplemented!(`, `todo!(` or `unsafe`, unless the file is under `tests/` or the text holds `#[test]`,
`#[cfg(test)]` or `#[allow(clippy::unwrap_used`. `SC/tests.rs` and `CN/src/widgets/tests.rs`
match none of these, and `#[gpui::test]` is not `#[test]`. So:
- an edit leaves an existing `.unwrap()` or `.expect(` outside its text. `activate_key`'s edit
  replaces only the `parse` call before its `.unwrap()`;
- a new failure in a test helper is an `assert!`. `secondary` asserts that the parse is `Ok`,
  naming the key, before it reads the keystroke.

Working around the hook, for example by writing the file through the shell, is not an option:
it is the maintainer's gate.

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
  target in three configurations: gpui-pre, GPUI Fast, and since the D round's `29634cfd`, no
  default features. The runs are CI's clippy job (`RUSTFLAGS: -Dwarnings`, `--all-targets`)
  and `check_release.sh`'s clippy sections, `--commit` included. The canary's clippy denies
  warnings too (`dependency-canary.yml:92`), but its schedule runs it on `main` (§0).
  - It names the method and the reason in the error: "panics on a parse error; use
    `KeyBinding::load`".
  - A deliberate, guarded call stays possible: `#[allow(clippy::disallowed_methods)]` at that
    site, with its reason. The lint flags every call, guarded or not, so a guarded site needs
    the allow too. One case is the inspector's `resolve_font`, behind its own guard (§1.9). The
    allow is visible in review.

**The list is an inventory, not a guess.** The candidates are the comment lines, doc comments
and plain ones, that mention a panic (`\b[Pp]anic`) in the `src/` of gpui-base, gpui-component, gpui-kit,
gpui-pre, gpui-fast, and the gpui-pre and gpui-fast sub-crates.
- There are 172 such doc-comment lines (`///`, `//!`) at 0.8.1 / 0.3.10 / 0.1.12.
- A narrower pattern (`# Panics`, "Panics if") finds 84 and misses forms such as "may panic
  if" (`Bounds::inset`, `GP/geometry.rs:1101`; not listed: the condition is the type's, and
  the impl already requires `T: Neg`, `:1089-1097`) and the sub-crates' docs (gpui-pre-sum-tree
  `cursor.rs:418`).
- Plain `//` comments add 107 more lines: 279 in all, measured by the sixth check's reviewer.
  What they hold has not been sorted, and a panic may be noted only there. So the sweep reads
  all 279: 107 more lines to sort, against the risk of missing one.

The sweep sorts each public function the lines document into two groups, and sets aside the
lines that are not a function's panic (a "does not panic" remark):

- **Listed: a panic that depends on an argument's value**, whether or not we call the
  function today, since a future call is what the gate is for. Examples:
  - `KeyBinding::new`'s parse error (`GP/keymap/binding.rs:32`);
  - `TextSystem::resolve_font`'s missing font (`GP/text_system.rs:351`);
  - `WindowTextSystem::shape_line` on a text with a newline (`:668-670`, the `impl` at `:641`;
    a `debug_assert!`, so debug builds only);
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

**The iced and egui connectors have the same gap.** A call into iced, iced_aw or egui that
panics on its input is invisible to the same three gates. That is a `docs/todo.md` entry,
`[1.0]` (P13's Task records it): the same inventory and a `clippy.toml` per connector. It is
not done here, because this change is the gpui connector's, and those crates' upgrades are
where their sweep belongs.

**Done-criterion, mechanical, for every entry, not only one.** A wrong path in the list
flags nothing and passes every run, so each entry is shown to flag a call, in every
configuration.

**Why a clean run proves nothing for GPUI Fast.**
- gpui-kit and gpui-base depend on gpui-pre without `optional` (`gpui-kit-0.8.1/Cargo.toml`,
  `[dependencies.gpui]` `=0.3.10`; gpui-base the same), and so does the connector's own `gpui`
  alias (`CN/Cargo.toml:63`). So gpui-pre stays in the crate graph under
  `--features gpui-fast`.
- Both cores' library is named `gpui` (gpui-pre `Cargo.toml:93-95`, gpui-fast `:94-96`).
- clippy resolves a disallowed path in every crate of that name. It warns only when the path
  is in none of them ("does not refer to a reachable", `clippy-driver`'s strings, rustc
  1.99.1; the per-entry `allow-invalid = true` silences it).

So an entry that resolves in gpui-pre but sits at another path in gpui-fast gives no warning
and gates nothing on GPUI Fast.

**The proof is therefore one planted function that calls every listed entry**:
- it is temporary, in the showcase's non-test code, with `#[allow(dead_code)]`, so an unused
  function does not fail the run on its own;
- its parameters supply what the calls need;
- each call is written as the showcase writes its gpui calls.

In each of the three configurations, every entry that configuration builds must draw a
`disallowed_methods` error at its planted line, with its entry's reason, and no other such
error may appear. The count is of planted lines: `--all-targets` builds the showcase both as an
example and as a test target, so a line may be reported twice. A planted call for an entry that
a core lacks takes that core's `cfg`. A missing error is an entry that gates nothing there. The function is then removed and the ordinary runs pass. This one measurement replaces
a separate check of the unresolved-path warning, a clean run, and a planted call per entry.

- **`allow-invalid = true`** is for an entry whose path is in no crate of some Linux
  configuration. Examples: a gpui-fast-only function under the default features, or a
  platform crate that Linux does not build. Each such entry states its reason. Where none of
  the configurations builds it, the commit body records it as not proven on Linux.
- **At each upgrade**, `CN/clippy.toml`'s header names this planted-function check as the
  sweep's proof. A rename upstream would otherwise only draw a warning, or nothing, where the
  other core still has the path.

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

**The guard is the two existing unit tests' expectations** (§1.5). Both assert `None` where no
bar height is stated, and a `None` refinement leaves upstream's minimum in place, which is the
defect. Both expectations become `Some(Length::Auto)` there:

- **`toolbar_leaves_min_height_unset_without_a_bar_height`** (`CN/src/geometry.rs:2984`) is
  the red-first test. It is one fixture with an explicit `bar_height = None`, so its failure
  depends neither on the presets' data nor on their order. Its name and doc state the old
  contract ("no floor"), so both are rewritten: the minimum is reset to `auto`, so the bar
  sizes to its content.
- **`toolbar_carries_the_models_toolbar`** (`:2811`) runs over every preset in both modes and
  asserts `out.min_size.height == t.bar_height.map(..)`. It holds the fix for every preset,
  whatever its content height.

Neither test passes until the fix lands. Changing only one would leave the other red after
the fix.

A showcase measurement would be a weaker guard. It reads one page's content, which on the two
measured presets is taller than the floor anyway, so it would pass there with or without the
fix. The unit tests read the refinement itself, for every preset. The showcase test's comment
"nothing to hold it to" (`SC/tests.rs:1412`) gets a clause saying why the chrome's own row,
which it measures, is not upstream's `Toolbar`.

**After the fix the sizing is 0.8.0's exactly:** the height `auto`, the minimum `auto`, and
upstream's padding and gap where the theme states none. So no screenshot taken after G4 shows
the floor. Between Task 1 and Task 5 a preset without a bar height may show it, which is not
measured and does not matter: no screenshot is taken in between.

### P8 — Dialog: the claims say what 0.8.1 draws, entrance and exit

**The connector's behaviour is unchanged** (§1.6). The text changes:

- **The Dialog's Widget Info animation lines** (`SC/info/chrome.rs:1334-1337`):
  - drop the `AnimationExt` citation;
  - cite `Dialog::motion_transition` for the curve and the 0–1 clamp, `exit_duration` for the
    duration, and `GC/root.rs:264-269`, where the root applies the transition to the dialog;
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
it. The files in scope are K3's, plus three more:

- K3's: `CN/` (sources, tests, examples, README and manifest comments),
  `docs/showcase-exceptions.toml` and `tools/showcase-support/src`.
  `docs/showcase-exceptions.toml` cites lines of `root.rs`, `input/input.rs`,
  `button/button.rs` and `scroll/scrollable.rs`, four files 0.8.1 changed.
- **`scripts/connector_parity_exceptions.toml`**, which G3 read by hand. One script run is
  simpler and checks more.
- **The egui and iced showcases**
  (`connectors/native-theme-egui/examples/`, `connectors/native-theme-iced/examples/`).
  - They cite gpui-component and gpui-pre lines to say what the gpui showcase draws: 67
    mentions. Examples: egui `demo.rs:272` `button.rs:1008`, now `:1022`; egui `chrome.rs:838`
    gpui-pre 0.3.9 `taffy.rs:523-527`.
  - G3 left the egui showcase's to its own milestone, which has since closed.
  - Their own crates' citations (iced, egui) are out of scope by the crate rule.
  - Their compatibility stamps hash their examples, so the iced and egui stamps are re-taken
    (P14).

What changes is which upstream crates are in scope:

- **Both GPUI cores changed** (0.3.9 → 0.3.10, 0.1.10 → 0.1.12), so their cited files are in
  scope, along with gpui-component and gpui-base.
- **So are their sub-crates**, all 19 of gpui-pre's at 0.3.9 → 0.3.10 and gpui-fast's eight at
  0.1.10 → 0.1.12. Our files cite them, often without a version:
  - gpui-pre-wgpu at `CN/src/widgets/checkbox.rs:483`;
  - gpui-pre-macros 0.3.9 at `CN/src/geometry.rs:488`;
  - gpui-pre-linux `client.rs` at `SC/app.rs:403`;
  - gpui-pre-windows `events.rs` at `:1908`;
  - and the inspector's platform crates.
  Without them in the family, the script would skip these citations, and the version grep
  would swap the versioned ones' versions with no line check.

**The wider scope makes a wrong crate a wrong number.** A citation that names no crate is
attributed by the nearest crate name in its note, or else by the only family crate whose old
file has a non-blank line there. `gates.py` uses the same rule, so it agrees with every
misattribution. In G3 a citation wrongly given to gpui-pre was a harmless "unchanged file".
With gpui-pre, its sub-crates and the iced and egui showcases in scope, the same mistake
rewrites digits. Two cases found by an independent reviewer's `--plan`-only dry run of the
adapted script at HEAD (2026-10-26, nothing written):

- `showcase-iced.rs:385` cites `` `scrollable.rs:548-560` ``, iced_widget's
  `operation.scrollable(…)`. The script gives it to gpui-component's `scroll/scrollable.rs`,
  whose lines there are test asserts, and moves it to 665-677.
- `CN/src/geometry.rs:1043` cites `` `toolbar.rs:243-257` ``, gpui-component 0.8.0's
  `RenderOnce for Toolbar`. Its note names both crates ("Neither gpui-component's `Toolbar`
  nor gpui-base's"). The nearest name is gpui-base, so gpui-base's diff renumbers it.

**So three rules are added**, to the script's copy and to `gates.py` alike:

1. In the iced and egui showcases (`connectors/native-theme-{iced,egui}/examples`), a citation
   whose note names no family crate is out of scope: they cite their own crates mostly without
   a name.
   - The rule is limited to those two directories. `docs/showcase-exceptions.toml` and
     `scripts/connector_parity_exceptions.toml` cite gpui files by bare name too. Their
     `[gpui]` notes name no crate, and only the file header does. The dry run renumbers 27 such
     chains there, a dozen of them moved; for example `showcase-exceptions.toml:48`,
     `components.rs:1023-1032` → `1024-1033`.
   - A wider rule ("outside `CN/`") would drop those citations silently. That would be out of
     scope, not set aside, and `gates.py` would agree.
2. A citation whose note names two or more family crates that each hold the cited file is
   set aside.
3. Task 2's review reads every renumbered chain that is outside `CN/` or attributed to a crate
   new to the scope (gpui-pre and the sub-crates). That was about 75 and about 60 chains in
   the dry run, a bounded read.

Each rule costs a few lines of script. The alternative, trusting the attribution, has already
been shown wrong.

**Two citations sit outside every scope**, in native-theme itself: `native-theme/src/lib.rs:354`
and `native-theme/src/gnome/mod.rs:572-576` state what gpui-pre-linux 0.3.9 does
(`src/linux/x11/client.rs:2671-2678`, `:2620-2666`). The claim still holds in 0.3.10, but the
lines moved. They are present-tense claims, so P14's version grep covers `native-theme/src`
too and the two are hand-read. That is cheaper than widening the script's scope for two lines.

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

### P10 — The other two issue drafts: re-checked, still to be filed

- **Rounded clip:** `GP/window.rs:2239-2246`, gpui-fast's `rounded_clip.rs` as re-read.
- **`try_resolve_font`:** `GP/text_system.rs:352`, the panic at `:621`, gpui-fast's as
  re-read.

**How the drafts change.** `docs/todo.md` is append-only, so neither draft's text is
rewritten.
- Each draft's text names its versions ("gpui-component 0.8.0 on gpui-pre 0.3.9"), so it stays
  true.
- Each gets a dated line, *GPUI Kit 0.8.1 (2026-10-26, G4):*, which says the issue still
  holds and gives its 0.8.1 / 0.3.10 / 0.1.12 lines.
- The maintainer files from the text and that line.

Rewriting the text in place would keep the draft paste-ready in one piece, but against the
file's rule, for no gain the dated line does not give. The drafts stay for the maintainer to
file (an agent files nothing).

### P11 — taffy 0.15: the Separator checked by a test; both cores' layouts compared; parity re-run; 0.15.1 measured once

**The Separator.** The draft's cause is gone from taffy 0.15.0 (§1.8). What is not yet seen is
the label centred in the showcase. The options for seeing it:

- **(a) A desktop capture of the Layout page on each core**, the label's x-range read from the
  PNGs. It is a one-off, it takes the desktop's focus, and it leaves no guard behind.
- **(b) A showcase test**, on both cores.
  - It finds the labelled Separator by its selector (`layout-separator-labelled`).
  - It finds the label's box among the painted quads inside it: the label paints a fill of its
    own (`GC/separator.rs:149`, `.bg(cx.theme().tokens.background)`), and G3 added the scene
    reader (`scene_quads`, `CN/src/widgets/scene_read.rs:22`), which the showcase tests
    already use.
  - It asserts that the box's centre is within a pixel of the line's.
  This is mechanical, it runs in CI on every push, and it would have caught the gpui-pre defect
  the draft describes.
- **(c) The layout dump.** The dump records the Separator's wrapper, not upstream's label
  inside it, so it cannot answer.

**(b).** The draft is about gpui-pre only: GPUI Fast was on taffy 0.14 and never had the
defect. So the entry closes on gpui-pre's measurement.
- **A capture, (a), is the fallback only where the test cannot find the label's box on
  gpui-pre.**
- **On GPUI Fast** the test asserts too, where it finds the box. The scene reader does not
  follow tiled frames, the composited scroll layers (`CN/src/widgets/scene_read.rs:15-18`).
  Other showcase tests already read page content through it on both cores, so this is not
  expected. If it happens, the GPUI Fast half takes a `cfg` with that reason, and no capture is
  made: GPUI Fast is not what the entry is about.

Centred on gpui-pre, the `docs/todo.md` entry closes with a dated line naming taffy's
`flexbox.rs` "the auto margins have absorbed all of the free space" (0.15.0 `:2274-2275`, 0.15.1
`:2320-2321`) and the test, and moves to `docs/archive/todo_closed.md`. If the test shows the
label is not centred, the entry stays open with a dated line giving taffy 0.15 and the measured
positions, this document's §1.8 is corrected, and the test is committed `#[ignore]`d with the
entry's name as its reason. Options for the test then: committed red breaks every later gate;
asserting the defect pins upstream's bug as the contract; dropped, it is lost and the next bump
writes it again. Ignored, it stays green and one `--ignored` run re-checks the entry at each
bump.

**No exception or claim names the label's position today.** The two exception files and
`SC/info/layout.rs` were checked, so nothing else changes for the label.

**Both cores' layouts are compared**, as G3's run-time comparison did (archived G3 spec §9).
Both layout engines moved (§1.1), so a layout difference between the cores can appear or
vanish. The comparison:
- `--dump-layout` for kde-breeze and adwaita, light and dark, from a gpui-pre build and a GPUI
  Fast build;
- each pair compared element by element.
A difference is fixed, or explained in the plan's *Record* (P15). It takes the desktop's
focus, so it is announced.

**The Linux parity run** (`scripts/check_showcase_parity.py`) runs on the final layout, against
the iced and egui showcases: a layout-engine bump and 0.8.1's scroll-area flex change (§1.7)
can move elements. Any exception it shows to be unnecessary goes. The gpui screenshots are then
regenerated by the release scripts, a release gate already (`generate_assets_release.sh`, the
PROVENANCE stamp).

**taffy 0.15.1, which an application on the default core gets (§1.1).** Every other layout
check here runs on our lock's 0.15.0. The options:

- **(a) Accept the gap.** It costs nothing, but it is P1's option (a) again: the release would
  ship checks of a layout engine that a fresh resolve no longer gets.
- **(b) Pin `taffy = "=0.15.0"` in the connector**, so applications get what we test. That is
  an exact pin in a library, which `CN/Cargo.toml:60-62` rules out (P1 (c)). It would also hold
  every application on gpui-pre back from taffy's fixes.
- **(c) A scratch application.** It resolves 0.15.1, but it cannot run the showcase, its
  tests or its dumps.
- **(d) One measurement in the tree, on a lock without gpui-fast.**
  - The `gpui-fast` feature is emptied in the working tree only (`gpui-fast = []`), so the
    resolve drops gpui-fast. `cargo tree -i gpui-fast` shows none.
  - `cargo update -p taffy` then takes 0.15.1.
  - The gpui-pre suite runs (the Separator test among them), with the four gpui-pre dumps,
    compared element by element with the 0.15.0 dumps.
  - `CN/Cargo.toml` and `Cargo.lock` are then restored (`git checkout --`), and nothing of it
    is committed.
  - It costs one rebuild of the gpui-pre stack each way, in the same build folder.

**(d).** It is the only option that measures what users run. A difference is explained in the
plan's *Record*, or fixed where it is ours. The release screenshots stay those of our lock, 0.15.0, and
a visible difference is recorded beside them. If the resolve keeps gpui-fast after all, the
gap is recorded as (a), with the reason.

**The split itself is upstream's.** gpui-fast's exact pin keeps any workspace that builds both
cores off gpui-pre's newest taffy. A `docs/todo.md` entry records it: *gpui-fast pins
`taffy = "=0.15.0"`*, `[upstream]`. It is for the maintainer to raise, and is re-checked at the
next GPUI Kit upgrade.

### P12 — The upstream entries are re-checked at 0.8.1

**What the question is.** `docs/todo.md`'s open `[upstream]` gpui entries and `ROADMAP.md`'s
upstream list (`:137-171`) each claim that upstream does something, citing a line "at
gpui-component 0.8.0". Those citations stay true of 0.8.0. The question is whether each claim
still holds at 0.8.1, and where its line is now: the lists are what an upstream PR is written
from.

**Options:**

- **(a) Leave them.** Their citations name their version.
- **(b) Dated lines only on the entries that wait for a release** (this document's first
  draft).
- **(c) G3's rule** (archived G3 spec §10): every open `[upstream]` gpui entry whose cited
  upstream file changed in 0.8.1's non-test code is re-read at 0.8.1.
  - One that 0.8.1 resolved closes and moves to the archive.
  - One that still holds gets a dated line with its 0.8.1 lines where they moved.
  - One whose file changed but whose claim and lines did not gets a dated "still so" line.
  - `ROADMAP.md`'s bullets are re-cited in place, since ROADMAP states the current plan and is
    not append-only.

**(c).** The reasons:

- (a) and (b) keep claims that nobody re-read against the code they describe. My spot-checks
  (`WindowBorder`, the Input placeholder, `Dialog::max_h`, the theme reads) found none
  resolved, but a spot-check is not a re-read.
- (c) is bounded. About 20 open gpui entries cite files 0.8.1 changed, and fewer cite files it
  changed outside tests (`checkbox.rs`, `button/toggle.rs`, `tab/tab_bar.rs`, `link.rs` and
  `message_scroller.rs` changed in tests only). Four ROADMAP bullets are affected.
- Each cited number's 0.8.1 line comes from §4.1's line rule, through a small read-only lookup
  beside the script (`lookup.py`: crate, file, line in; the new line or "set aside" out). Only
  the claims are read by hand. The script itself cannot do it: its scope is the code, not
  these two documents.

**Entries that wait for a release** get their dated line under the same rule:

- **the Separator draft** (P11) and **the two other drafts** (P10);
- **`unicode-properties`:** still pinned by gpui-fast-web 0.1.12;
- **`painted_quads`:** unchanged in gpui-fast 0.1.12 (`GF/window.rs:2925`).

(The base-palette repair's entry needs none: the C round fixed it and closed the entry in
`e92c288a`, C-04.)

The new public API (§1.10) adds no style seam beyond TextView's and the trigger-layout one.
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
    seam (`GC/dialog/dialog.rs:495`) and the facts it needs. The tag is `[1.0]`, settled
    before 1.0:
    - the placement is a native-look fact on every platform, so 1.0 should not ship without
      it;
    - it needs platform research first;
    - it cannot be a `[v0.7.x]` patch, because `DialogTheme` is not `#[non_exhaustive]`
      (`native-theme/src/model/widgets/mod.rs:810-815`), so a new public field breaks struct
      literals. A later minor release may carry it;
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
  *Backends* section (`:822-837`), and the 0.8.0 mentions in rows (`:29`, `:57`; the `toolbar`
row's, `:365`, with P13's change).
  `compat::` checks the table and the quick start.
- **`docs/COMPATIBILITY.toml`:** through `scripts/update_compatibility.sh`, for gpui, and for
  iced and egui too, since P9 changes their showcases' citations and their stamps hash their
  examples.
- **`ROADMAP.md:72-73`.**
- **`docs/todo_gpui-full-theme.md`:** an appended "gpui-component 0.8.1" note, as G3 appended
  one for 0.8.0. That file records which upstream version its theme map was checked against.
- **The manifest comments:** `CN/Cargo.toml:6-11`, `:56-62`, `:76-82`. The `image` comment
  still holds: gpui-pre 0.3.10 and gpui-fast 0.1.12 both ask for `image` `0.25.1`.
- **`.cargo/audit.toml`'s comments** (P2).
- **native-theme's two gpui-pre-linux citations** (P9), re-read at 0.3.10 and re-cited.

**CHANGELOG `[Unreleased]`:**

- **The entry that states the floors ("requires gpui-component, gpui-base and gpui-kit 0.8.0
  and gpui-pre 0.3.9 …"), under *Breaking Changes* since the close-out audit `5c3bd3bb`, is
  rewritten in place.**
  v0.7.0 is unreleased, so the entry states what v0.7.0 requires. There is no migration note
  (pre-1.0).
- **The Kbd and toolbar fixes need no CHANGELOG entry of their own.** They repair what the
  bump would have broken, and no released version had the defect. The toolbar's contract
  ("sizes to content") is already stated.
- **The new P6 gate gets an `### Added` line**, `**Development**:`, as the other gates have
  (the entries for `scripts/check_docsrs.py`, `check_connector_parity.py`,
  `check_readme_versions.py`).

The `secondary` change is upstream's. The connector's public API does not change.

### P15 — Where G4 sits: after the code review's close, before the release, as one writer

`v0.7.0-rc1` was written by the C round and then the D round on 2026-10-26
(`docs/archive/todo_v0.7.0_code-review-findings.md`, *C round*, *D round*).
- G3 entered between M9b's tasks (K13). G4 could not: the review session asked, on 2026-10-26,
  that G4 not start in this tree until it said the D round was closed.
- Its writers ran cargo and `check_release.sh` continuously, so a dependency bump under one
  would have broken its builds and gates.
- The session said so the same day: the C and D rounds closed, the review's plan and ledger
  archived at `60fa77fd`, the branch pushed at `7f011f00` (after D15's last fix), its writers
  stopped.
  G4 starts from there, on a clean tree.

**The shared files:** `Cargo.lock`, `CHANGELOG.md`, `docs/todo.md`,
`docs/showcase-exceptions.toml`, the showcase. One writer at a time keeps them consistent.

**G4's record.** G1–G3 registered in the review plan and kept their findings in its ledger, while
the review ran. The review is closed now, so G4 needs a record of its own: each task's commits,
a missing input, each failure or difference explained rather than fixed, the taffy 0.15.1
result (P11).
- **(a) A row and a section in the archived review plan and ledger.** Pro: one place, as for
  G1–G3. Con: it reopens documents archived as final (their *As built* notes and Status table
  say the review is closed), for a milestone the review did not run.
- **(b) A *Record* section at the end of G4's plan.** Pro: no new file; it is archived with the
  plan, beside the spec's *As built* note. Con: a task's line rides in a commit that also edits
  the plan. That costs nothing if the Record holds no commit hashes: `git log --grep 'G4 T'`
  lists the commits, since each message ends "(G4 Tn)". So each task writes its lines in its own
  last commit, inside the unit too, and needs no later commit naming a hash.
- **(c) A G4 ledger file.** Pro: the record apart from the steps, as the review kept it. Con: a
  fourth document, with nothing in it the plan's section cannot hold.

**(b).** The approvals are this document's §3, so no decision line elsewhere is needed.

**The screenshots that change** (spec §10) cannot wait in the Record: it is archived at G4's
close, before the release asset run. They go as a dated line on the `docs/todo.md` entry that
run closes (*native-theme-gpui connector*, *Regenerate the gpui screenshots and review the
showcase's visible changes*).

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

**The fresh resolve is tested on the branch too.**
- P2's scoped update deliberately does not take the rest of the newest dependencies, but an
  application's fresh resolve does (§1.2).
- The dependency canary has `workflow_dispatch` (`dependency-canary.yml:25`). Dispatched on
  `v0.7.0-rc1` after the unit's push and again after Task 8's push
  (`gh workflow run dependency-canary.yml --ref v0.7.0-rc1`), it builds and tests the branch
  on exactly that resolve.
- It is one command each time. A failure there is read like any canary failure: an upstream
  release, not this change, unless it touches the gpui stack.
- **How its result is read.** `main`'s standing failures are gpui gates themselves (§0): the
  citation gates and the widget coverage gate, on released code against a newer gpui. So a
  "same failure as `main`" rule would file exactly G4's kind of failure as standing. Instead:
  - **every failure of a gpui clippy, test, doc or coverage line on the branch is G4's**, to
    be diagnosed and fixed or explained;
  - **a failure elsewhere** (native-theme, iced, egui) is read like any canary failure;
  - **if the Clippy, Test or Documentation step stops before its gpui lines** (`bash -e`, §0),
    those lines did not run.
    They are run by hand on the same resolve: a full `cargo update` in the tree, the canary's
    gpui lines, then `Cargo.lock` restored. Without that, the dispatch shows nothing for gpui.

---

## 3 — The maintainer's approvals

- **P1, the floors to GPUI Kit 0.8.1 and gpui-pre 0.3.10: approved** (2026-10-26).
- **P3, the showcase's key bindings through `KeyBinding::load`: approved** (2026-10-26). This
  includes the unmeasured Windows keyboard-mapper difference as an accepted risk.

Neither approval is touched by the later check passes.

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
- **The toolbar probe** (§1.5), under four presets on gpui-pre (two further names were not
  preset keys).
- **A fresh resolve** of `gpui-component = "0.8.0"`, with and without `rust-version = "1.88"`
  (§1.2).
- **The keyboard mappers** of gpui-pre, its test platform, macOS and Windows (P3), and
  `Kbd::format`'s OS source (P4, P5).
- **Three open `[upstream]` entries spot-checked** against the 0.8.1 diff (P12; the full
  re-check is G4's), `DialogTheme`'s exhaustiveness (P13), `.cargo/audit.toml` (P2), and the
  citation-bearing files outside the gates (P9).
- **A fresh resolve of `gpui-component = "0.8.1"`** (2026-10-26): taffy 0.15.1, gpui-pre
  0.3.10, no gpui-fast (§1.1), and taffy 0.15.0 against 0.15.1's source diff.
- **The canary's last scheduled run on `main`** (40232134026), read for its failing steps (§0).
- **A `--plan`-only dry run of G3's script**, adapted as spec §4.1 says, at HEAD after the D
  round's first commits. An independent reviewer ran it on 2026-10-26 and wrote nothing to the
  repository. It found the attribution cases of P9.

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
