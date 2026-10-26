# v0.7.0 — GPUI Kit 0.8.1 (G4): Specification

**Status:** design, drafted 2026-10-26. The maintainer approved rationale P1 and P3 on
2026-10-26. The other decisions are settled by logic, and the maintainer may veto them.

- Reasons: [`todo_v0.7.0_gpui-kit-0.8.1-rationale.md`](todo_v0.7.0_gpui-kit-0.8.1-rationale.md)
  (P1–P15).
- Tasks and gates: [`todo_v0.7.0_gpui-kit-0.8.1-plan.md`](todo_v0.7.0_gpui-kit-0.8.1-plan.md).

**Abbreviations.**

- Upstream sources: `GC` = gpui-component 0.8.1, `GB` = gpui-base 0.8.1, `GP` = gpui-pre 0.3.10
  and `GF` = gpui-fast 0.1.12. Each is the crate's `src/` in the cargo registry
  (`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/<crate>-<version>/src/`). The
  old versions are written out ("gpui-component 0.8.0").
- Our crate: `CN` = `connectors/native-theme-gpui/`, `SC` = `CN/examples/showcase-gpui/`.

**Line numbers.**

- Ours are of `v0.7.0-rc1` at `8086a20e`. The C and D rounds still commit to the branch, so
  every task locates its regions by content and re-reads what this spec describes before it
  writes (§0.2). The D round's D4 is known to move lines this spec cites in
  `CN/src/geometry.rs` (the dialog docs, `toolbar` and its test), `CN/src/config.rs:65` and
  `SC/demo.rs:6478`.
- `docs/todo.md` entries are cited by section, heading and opening words, as its header
  requires. `CHANGELOG.md` entries are cited by their text.

**Inputs.** These are in `.superpowers/sdd/todo_code-review-plan/g4/`, on the project disk and
untracked, like the rest of that directory:

- `gk081-trial-check.log` and `gk081-trial-tests.log`: the trial build. The tests log holds
  the gates' full lists of the 125 colour claims and 14 prose citations that moved.
- `gk081-lockdiff.txt`: the trial's lock change.
- `gk081-diff-review.md`: the review of the upstream non-test diff.
- `gk081-msrv-1.94.txt`: the 1.94.0 run's command and output.
- `trial.sh` and `trial2.sh`.

Nothing a task needs is only there: every count and finding used here is restated.

---

## 0 — Scope and constraints

### 0.1 Deliverables

- the gpui connector, its tests and its showcase on GPUI Kit 0.8.1, gpui-pre 0.3.10 and
  gpui-fast 0.1.12 (§1);
- the showcase's keys parsed and bound without a panic path (§2);
- the test code migrated (§3);
- every upstream citation true for the new versions (§4);
- a lint gate for upstream functions that panic on their input (§5);
- the toolbar's unstated height back to the content's (§6);
- the Separator, the issue drafts, parity and the todo list (§7);
- the new upstream API recorded (§8);
- documents and the compatibility stamp (§9).

### 0.2 Constraints

- **No panic paths.** No `unwrap`, `expect`, `panic!`, `unreachable!`, slice indexing or
  unchecked arithmetic outside tests, and no `unsafe`. The strict panic lints
  (`scripts/check_strict_panic.sh`) pass on the library and on the showcase example, on both
  cores. From §5 on, no non-test call of a listed upstream function goes without a site allow.
- **No invented values.** Every number in a claim, a comment or a document is read from the
  cited source or measured.
- **Both cores.** Every gpui change is tested on gpui-pre (`--features native,watch`) and on
  GPUI Fast (`--features gpui-fast,native,watch`).
- **And without default features.** Since the D round's `29634cfd`, `check_release.sh`
  (`--commit` included) and CI also run `cargo clippy -p native-theme-gpui --all-targets
  --no-default-features -- -D warnings`. New code compiles there with no warning. A helper,
  test or allow used only under `widgets` or an icon feature takes that `cfg`, as D8 did for
  the existing ones.
- **The project's panic hook** (`.claude/hooks/no-runtime-panics.sh`) refuses an edit whose new
  text holds `.unwrap()`, `.expect(` or `panic!(`, unless the text holds `#[test]`,
  `#[cfg(test)]` or `#[allow(clippy::unwrap_used` or the file is under `tests/`. `SC/tests.rs`
  and `CN/src/widgets/tests.rs` are not exempt, and `#[gpui::test]` is not `#[test]`.
  - An edit leaves an existing `.unwrap()` or `.expect(` outside its text.
  - A new failure in test code is an `assert!`.
  - The hook is never worked around (rationale P5).
- **Shared files are located by content.** A task that edits a file the review rounds also write
  re-reads, before writing, the code this spec describes there. Those files are `Cargo.lock`,
  `CHANGELOG.md`, `docs/todo.md`, the ledger, `docs/showcase-exceptions.toml`, the showcase
  and the connector. Where the description no longer holds, the task stops and reports.
  A rename or a move that keeps the role and the behaviour is followed and noted; a changed
  behaviour is a stop.
- **`docs/todo.md` is append-only.** A closed entry moves to `docs/archive/todo_closed.md`
  with its closing commit, as that file's header says.

---

## 1 — Floors, lockfile, MSRV, audit, README floors (P1, P2)

**`CN/Cargo.toml`:**

- `gpui-component` (`:66`), `gpui-base` (`:70`), `gpui-kit` normal (`:75`,
  `default-features = false` kept) and `gpui-kit` dev (`:104`, features kept) go from
  `"0.8.0"` to `"0.8.1"`.
- `gpui = { package = "gpui-pre", version = "0.3.9" }` (`:63`) becomes `"0.3.10"`, still a
  caret.

**Its comments, each re-verified at the new versions:**

- **`rust-version` (`:6-11`):** the date, "on the 0.8.1 closure", and `cold_path` at
  gpui-pre 0.3.10 `src/profiler.rs:473`, which is gpui-fast 0.1.12's line too (measured
  2026-10-26).
- **The `gpui` comment (`:56-62`):** "gpui-component 0.8.1 depends on
  `gpui-pre = "=0.3.10"`", re-read in `GC`'s `Cargo.toml`.
- **The `image` comment (`:76-82`):** "gpui-pre 0.3.10 and gpui-fast 0.1.12 each ask for
  `image` 0.25.1 and up". Their manifests: gpui-pre `Cargo.toml:365-366`, gpui-fast
  `Cargo.toml:186-187`.

**Lockfile:** `cargo update -p gpui-kit -p gpui-component -p gpui-base` on the committed lock,
taken at the task's own start.

- **Gate:** `git diff Cargo.lock` changes exactly the trial's packages, or the commit body
  explains each difference. The trial's packages (`gk081-lockdiff.txt`, rationale §1.2):
  - gpui-kit, gpui-kit-assets, gpui-component, gpui-component-macros and gpui-base go to 0.8.1;
  - the 20 `gpui-pre*` crates go to 0.3.10;
  - gpui-fast and its eight sub-crates go to 0.1.12;
  - wgpu 29.0.4 (with its eight sub-crates and naga 29.0.4), taffy 0.13.0, taffy 0.14.0,
    cocoa 0.26.0 and cocoa-foundation 0.2.1 are removed, and taffy 0.15.0 is added;
  - bit-set 0.9.1 becomes 0.11.1;
  - proptest 1.11 → 1.12, with proptest-macro 0.6, rand_xorshift 0.5, bit-vec 0.10.1,
    core_detect and a `borsh` entry.
- **native-theme's property tests then run on proptest 1.12**: its dev-dependency
  `proptest = "1.11"` (`native-theme/Cargo.toml:140`) resolves to the new version. Its test
  suite is part of the task's gate (`check_release.sh --commit` runs it).
- `cargo tree -p native-theme-gpui -e normal` before and after goes in the commit body.

**MSRV**, measured with the lines of CI's `msrv` job, under `RUSTFLAGS=-Dwarnings`:

- `cargo +1.95.0 check -p native-theme-gpui --locked`, `… --features native,watch --locked`
  and `… --features gpui-fast,native,watch --locked` pass;
- `cargo +1.94.0 check -p native-theme-gpui --locked --ignore-rust-version` fails on
  `cold_path`.

If 1.95.0 fails, `rust-version` becomes the lowest toolchain that passes all three, tried
upwards; the README row, the comment and the CHANGELOG follow. A missing toolchain is
installed only with the maintainer's permission.

**Audit:** `cargo audit` passes on the new lock.

- Each `.cargo/audit.toml` comment that names a GPUI crate is re-read with
  `cargo tree --workspace -i <crate> --target all` and its versions are updated. Today these
  are instant, paste, rustls-pemfile and ttf-parser, naming "gpui-component 0.8.0",
  "gpui-kit-assets 0.8.0" and "gpui-pre 0.3.9".
- An entry the new lock no longer reaches is removed, as the file's header asks.

**README floors:**

- `CN/README.md`'s Compatibility table (`:76-79`): 0.8.1, and gpui-pre 0.3.10.
- Its Quick start (`:109`): 0.8.1.
- Gates: `compat::the_readme_states_the_manifest_floors` and
  `compat::the_quick_start_states_the_manifest_floors`.

**Compile gate:** `cargo check -p native-theme-gpui --all-targets` on both cores, with no
warning, once §2 and §3 are in.

---

## 2 — The showcase's keys (P3, P4)

### 2.1 Bindings through `KeyBinding::load` (P3, approved)

**`SC/app.rs`, `init` (`:176-196`) and `macos_bindings` (`:198-208`):**

- **A private helper** `fn load(keys: &str, action: Box<dyn Action>, cx: &App) -> Option<KeyBinding>`
  calls `KeyBinding::load(keys, action, None, false, None, cx)` (`GP/keymap/binding.rs:51-68`;
  gpui-fast 0.1.12's has the same signature, `GF/keymap/binding.rs:51-58`).
  - `Ok(binding)` becomes `Some(binding)`.
  - `Err(error)` writes one `eprintln!("showcase: the key binding {keys:?} did not parse:
    {error:?}")` line and returns `None`: the binding is left out, with no panic and no
    fallback key. That is the showcase's existing pattern for a refused input
    (`SC/app.rs:1416`).
  - A `match` does both. `.ok()` alone would discard the error before it is printed.
- **`init` builds its four bindings** — `"secondary-q"` `Quit`, `"secondary-b"`
  `ToggleSidePanel`, `"secondary-k"` `OpenCommandPalette`, `"secondary-,"` `OpenPreferences`
  — through `load`, collects the `Some`s, and passes them to `cx.bind_keys`.
  - The bindings are collected before `bind_keys`, which takes `&mut self`.
- **`macos_bindings(cx: &App) -> Vec<KeyBinding>`** builds `"cmd-h"` `Hide`, `"alt-cmd-h"`
  `HideOthers`, `"cmd-m"` `Minimize` and `"cmd-w"` `CloseWindow` the same way.
  - Its doc keeps the HIG reference.
  - Its caller (`:185`) builds the list first and then binds it
    (`let mac = macos_bindings(cx); cx.bind_keys(mac);`): `bind_keys` takes `&mut App`
    (`GP/app.rs:2741`).
- **No `KeyBinding::new` remains in the showcase's non-test code.** `grep -n
  'KeyBinding::new' SC --include=*.rs`, outside test files, finds none. §5's gate then holds
  this.

**The docs** of `init` and `macos_bindings` say why `load` is used: no panic path, since
gpui-pre 0.3.10's `KeyBinding::new` panics on a parse error (`GP/keymap/binding.rs:32-47`).
They also state the Windows note: `load` maps through the platform keyboard mapper, which on
Windows round-trips the key through the current layout (`gpui-pre-windows`
`src/keyboard.rs:39-60`, `:139-145`).

### 2.2 The Kbd samples and the Save tooltip (P4)

- **`SC/demo.rs`, `kbd` (`:6472-6485`):** `Keystroke::parse(keys)` becomes
  `Keystroke::parse_with_os(keys, cx.platform_info().os)` (`GP/platform/keystroke.rs:125`).
  The function keeps `keys: &'static str`.
- **`SC/pages/feedback.rs`, `save_tooltip` (`:141-145`):** it takes `cx: &App` and parses the
  same way. Its caller (`:454`) passes `cx`.
- **The docs of `KBDS` (`SC/pages/typography.rs:94-97`) and `SAVE_KEYS`
  (`SC/pages/feedback.rs:133-136`)** say the `secondary-` modifier is resolved with gpui's
  runtime OS. They also say `Kbd::format` draws by the compile target (`GC/kbd.rs:105-151`),
  and that the two agree on every native target.
- **A parse failure writes one stderr line in both** (`eprintln!("showcase: the keys {keys:?}
  did not parse: {error:?}")`), and each keeps its fallback: the Kbd is left out, and the
  tooltip shows the raw keys (rationale P4).
- **Result:** four Kbds again on the Typography page, and "Save file (Ctrl+S)" off macOS,
  "⌘S" on it.

---

## 3 — Test code (P5)

Each of these sites failed to compile in the trial, or would assert nothing:

- **`CN/src/widgets/tests.rs`, `activate_key` (`:1712-1721`):** the keystroke comes from
  `cx.update(|_, cx| gpui::Keystroke::parse_with_os(key, cx.platform_info().os))`, as
  gpui-component 0.8.1's own `activate_key`s do (`GC/checkbox.rs`, `switch.rs`,
  `button/toggle.rs` tests). `.unwrap()` stays as it is: test code. The edit replaces only the
  `parse` call before it (§0.2, the hook).
- **`SC/tests.rs`, `scroll_at` (`:413-424`):** a `gpui::ScrollEvent`, not the deprecated
  `ScrollWheelEvent` alias. It gains `source: gpui::ScrollSource::Pointer`
  (`GP/interactive.rs:522-555`), with the other fields as they are. No
  `..Default::default()`, so the event says what it simulates.
- **The `unparse()` calls** (`SC/tests.rs:2363`, `:2382`, `:2414`, `:5345`) take the test
  platform's OS: `cx.platform_info().os` from the context each closure already has, read once
  per test where a closure has none.
- **`the_macos_menus_keys_are_the_higs` (`:2224-2242`):**
  - it becomes a `#[gpui::test]`, so it can call `macos_bindings(cx)`;
  - its `bound` closure parses with `parse_with_os(keys, os)`;
  - its assertions are unchanged.
- **`secondary` (`:2377-2384`):**
  - it becomes `fn secondary(key: &str, os: gpui::OperatingSystem) -> String`;
  - it parses with `parse_with_os(&format!("secondary-{key}"), os)` and returns
    `.unparse(os)`;
  - on a parse failure it fails the test instead of returning `""`: it asserts that the parse
    is `Ok`, with the key in the message, before it reads the keystroke (§0.2, the hook);
  - its doc no longer cites gpui-pre `keystroke.rs:143-150`: it cites `GP`'s `:158-168`
    (the `secondary` arm of `parse_internal`);
  - its callers (`:2351-2354`, `:2417`, `:5335`) pass the test platform's OS.
- **`the_kbds_take_the_showcases_modifier` (`:17654`) and
  `the_shortcut_tooltip_names_its_keystroke` (`:17682`)** parse with `parse_with_os` and the
  test platform's OS. Both are plain `#[test]`s today. Each becomes a `#[gpui::test]`, for the
  OS, and the tooltip test also for `save_tooltip(cx)`. Their expected strings are unchanged.
- **`use_preset` (`SC/tests.rs:263-272`)** asserts that `apply_theme_by_name` returned `true`
  (`SC/app.rs:1861-1866`), with the preset's name in the message. `use_preset_scaled` calls
  it, so the assertion covers its 96 calls and `use_preset_scaled`'s 4.
- **A new helper, `use_missing_preset`, asserts that the install failed.** It is for the two
  tests that pass `"no-such-preset"` on purpose to provoke a theme error:
  `the_theme_error_alerts_icon_follows_the_chosen_set` (`:8613`) and
  `a_theme_error_is_an_alert` (`:13164`). Both call it instead of `use_preset`.
- **`set_preset` (`SC/tests.rs:11108`)** gains a `showcase: &Entity<Showcase>` parameter,
  which every caller already holds. After the dispatch, it asserts
  `showcase.current_theme_name == key`, for every key but `"default"`.
  `apply_theme_by_name` sets that field only on a successful install (`SC/app.rs:1883-1887`).
  The six `"default"` calls are not asserted, since the OS read may fail on a runner.
- **A new `set_missing_preset`** asserts that `current_theme_name` did not change. The two
  `"no-such-preset"` calls (`:11925`, `:11970`) use it, and keep their error-alert assertions.
- **Any other call that fails either assertion** was testing the theme installed before. It is
  fixed in this task by its right preset name, and listed in the commit body.
- **A new test, `the_typography_page_draws_its_kbds`:**
  - it shows the Typography page;
  - it finds each of the four `KBDS` ids laid out (`bounds_of`);
  - so a `demo::kbd` whose parse fails fails it (rationale P4).

  The test is red on 0.8.1 without §2.2's change. `the_kbds_take_the_showcases_modifier` parses
  `KBDS` itself and would not see that.
- **A new test, `the_bindings_take_cmd_under_macos`** (rationale P5):
  - before `open`, it calls `cx.set_platform_info(gpui::PlatformInfo { os:
    gpui::OperatingSystem::Mac, target: gpui::PlatformTarget::Native })`
    (`GP/app/test_context.rs:127`; gpui-fast has the same);
  - it checks the four `app::init` bindings against `secondary(key, OperatingSystem::Mac)`,
    `cmd-q` and the rest, through the binding-table loop of `the_menus_run_actions`
    (`SC/tests.rs:2350-2375`), which becomes a helper shared by both tests and taking the OS;
  - its doc states what it does not cover: `Kbd::format` draws by the compile target, and the
    macOS menu-bar bindings are bound only under `cfg!(target_os = "macos")` (`SC/app.rs:184`).

**Gates:**

- `cargo check -p native-theme-gpui --all-targets` passes on both cores with no warning;
- every test passes on both cores except the citation gates and
  `the_fallback_stack_copy_is_gpuis`, which §4 turns green;
- `no_inspector_toggle_remains` has a non-empty key to compare: shown by its assertion, which
  now fails on a parse error rather than comparing with `""`.

---

## 4 — Citations (P9)

### 4.1 Renumbering — digits only, one run

**The script is G3's**, `.superpowers/sdd/todo_code-review-plan/g3/t2/`: `remap.py` with
`upstream.py` and `notes.py`, and its gates `gates.py` with `gate4_additions.py`, which
implement the archived G3 spec §3.1. It is copied to `.superpowers/sdd/todo_code-review-plan/g4/t2/`,
and every G3-specific constant and branch is changed. These are all of them, as of the copy:

- **`upstream.py`:**
  - `FAMILY`: gpui-component, gpui-base, gpui-kit and gpui-kit-assets `("0.8.0", "0.8.1")`,
    gpui-fast `("0.1.10", "0.1.12")`;
  - **gpui-pre joins it** as `("0.3.9", "0.3.10")`: it changed this time, so its citations are
    in scope (rationale §1.11);
  - **so do the sub-crates** (rationale P9): the 19 `gpui-pre-*` crates at `("0.3.9",
    "0.3.10")` and the eight `gpui-fast-*` crates at `("0.1.10", "0.1.12")`. All 54 source
    directories are in the registry (checked 2026-10-26; `cargo fetch` restores any that is
    missing). Their names match `remap.py`'s `NAME_RE` and `LEAD_RE` as written;
  - `PRE` and `ALSO_OLD` go, with their uses (`:46`, `:82-83`).
- **`remap.py`:**
  - its imports and `FAMILY_NAMES` (`:20`, `:40`) and the candidate loop (`:108`) without
    `PRE`;
  - the gpui-pre exemption (`:367-368`) goes;
  - the gpui-fast "0.1.7 / 0.1.8" special case (`:370-371`, `:381-382`, the `alt` map at
    `:392` and `:403`) goes. No citation in scope names two gpui-fast versions together
    (grep, 2026-10-26), so a citation naming a version other than the old or the new one is
    set aside by the generic rule (`:372-379`).
  - `T1_COMMITS` (`:33`): see "Task 1's lines" below;
  - `SCOPE` (`:23-31`) gains `scripts/connector_parity_exceptions.toml`, which G3 read by hand,
    and the egui and iced showcases, `connectors/native-theme-egui/examples` and
    `connectors/native-theme-iced/examples` (rationale P9). Their own crates' citations stay
    out of scope by the crate rule;
  - the docstring and the messages (`:2-3`, `:468`) name G4 and these versions.
- **`gates.py`**, which keeps its own copy of the versions on purpose, so that its attribution
  stays independent of `remap.py`'s:
  - `VERSIONS` (`:22-29`) as `FAMILY` above, gpui-pre included as `("0.3.9", "0.3.10")`;
  - `TASK1` (`:30`) as `T1_COMMITS`;
  - `BASE_REV` (`:20`) is Task 2's starting HEAD;
  - `SCOPE` as `remap.py`'s;
  - the docstring and the messages (`:2`, `:358`).
- **`gate4_additions.py`** reads `BASE_REV` from `gates.py` and needs no change.

**"Task 1's lines" become the unit's own commits.** These are the lines §1–§3's commits wrote,
read with `git show` of each commit, as G3 Task 1's ruling read them. Those lines already cite
the new versions. The script leaves them alone and lists every upstream citation it skips
there, for the review to read at the new versions.

**The check of the copy:** before the run, `grep -n '0\.7\.3\|0\.1\.7\|0\.1\.8\|c8d89e75\|62ac5935'`
over the five files finds nothing. A remaining constant would map numbers against the wrong
versions with every gate passing.

**Three attribution rules, added to `remap.py` and to `gates.py` alike** (rationale P9). Without
them, a citation given to the wrong crate is renumbered by that crate's diff, and `gates.py`,
which attributes the same way, agrees:

1. **In `connectors/native-theme-iced/examples` and `connectors/native-theme-egui/examples`
   only**, a citation whose note names no family crate is out of scope. The rule is limited
   to them. `docs/showcase-exceptions.toml` and `scripts/connector_parity_exceptions.toml`
   cite gpui files unnamed too, in scope, and are renumbered as before. Two cases are known:
   - `connectors/native-theme-iced/examples/showcase-iced.rs:385`'s `` `scrollable.rs:548-560` ``
     cites iced_widget, which the script would give to gpui-component;
   - the egui and iced showcases otherwise cite their own crates unnamed.
2. **A citation whose note names two or more family crates that each hold the cited file is
   set aside.** For example, `CN/src/geometry.rs`'s `` `toolbar.rs:243-257` `` ("Neither
   gpui-component's `Toolbar` nor gpui-base's") is gpui-component's `RenderOnce for Toolbar`,
   but the nearest name is gpui-base.
3. **The plan's output lists every renumbered chain outside `CN/` or attributed to gpui-pre or
   a sub-crate**, the crates new to the scope. Task 2's review reads each one's old text against
   its claim. A `--plan`-only dry run at HEAD on 2026-10-26 gave about 75 and about 60.

Each rule is checked on its example before the run, in `--plan` output:
- the iced citation is out of scope;
- the toolbar citation is set aside;
- `docs/showcase-exceptions.toml:48`'s `components.rs:1023-1032` is still renumbered, to
  `1024-1033` (the dry run's result, rationale P9).

**`lookup.py`, new beside the copy, read-only**, for §7's re-check:

- it takes `crate file line` triples and prints each line's 0.8.1 number by the line rule
  below (`upstream.py`'s `Crates.line_map`), or "set aside" with the rule's reason;
- it writes nothing;
- its own check: for ten numbers §4.1's run moved, it gives the same lines.

**Everything else is G3 spec §3.1's rule, unchanged:**

- the forms and the chain rule;
- a chain moves whole or not at all;
- the file and the crate attribution;
- the line rule: unique text, else a difflib equal block of three or more lines that git's
  `histogram` map confirms, else set aside, with no "stays" shortcut;
- one run, `--plan` before `--write`;
- the four gates (digits only, line text, the two showcase citation gates' failures dropping
  to the set-aside list, and the whole check by `gates.py`'s own wider scan).

**What goes in the commit body:** the counts by form and by rule, and the set-aside list in
full, each entry with its reason, its cited old text and the repo line's own text. §4.2 and
§4.3 read it with `git show`. The script is not run again: a second run would read
already-moved numbers as old ones.

**Known prose numbers the review checked by hand** (`gk081-diff-review.md`), which the script
may set aside:

- `SC/info/data.rs:123` "lines 2254 and 2359": now `:2311`, `:2421`;
- the Dialog's "lines 718-804": now `:775-861`;
- `dialog.rs` `:689` → `:739`, `:722` / `:724` → `:779` / `:781`, `:322-324` → `:335-337`
  (`SC/info/theme_map.rs:173`, `CN/src/colors.rs:543`);
- `CN/src/geometry.rs`'s dialog references: `:497` → `:519`, `:526` → `:549`, `:616` → `:667`,
  `:630` → `:689`, `:725` → `:782`, `:729` → `:786`, `:730` → `:787`, `:735` → `:792`,
  `:760` → `:817`, `entrance.rs:12-25` → `:14-27`, `alert_dialog.rs:277` → `:284`.

These are checked, not trusted: each is read in `GC` when it is touched.

### 4.2 Read by hand: what the gates read (in the unit)

**Every set-aside citation in `SC/info/`**, read in the new source and re-pointed or
rewritten.

**`the_fallback_stack_copy_is_gpuis` (`SC/tests.rs:10600`) and `GPUI_FALLBACK_STACK`'s doc
(`SC/inspector.rs:272-290`):** the versions and the stack lines are re-cited at gpui-pre 0.3.10
and gpui-fast 0.1.12, and the test's own table and doc with them.

**The panels of widgets whose behaviour changed**, each claim re-read even where its line did
not move:

- **Dialog** (P8; `SC/info/chrome.rs`, the Dialog's panel):
  - the backdrop claim's `dialog.rs:327` (`:1321-1326`) → the overlay read's new line (`:340`);
  - the four `animation` texts (`:1331-1339`) are rewritten:
    - they drop "gpui-pre/elements/animation.rs, AnimationExt" (the Dialog no longer uses it,
      `GC/dialog/dialog.rs:558-580`);
    - they cite `Dialog::motion_transition` for the curve and the 0–1 clamp (`:570-580`, the
      clamp at `:578`), `Dialog::exit_duration` for the duration (`:558-568`), and
      `GC/root.rs:264-269`, where the root applies the transition to the dialog;
    - they say the dialog slides and fades (or fades) **in and out** on the same curve and
      duration;
    - the reduced-motion texts say it appears **and leaves** at once (`exit_duration` returns
      zero, `:559-561`);
  - the two "lines 718-804" texts (`:1340-1352`) are re-pointed.
  - The Sheet's `AnimationExt` citation (`:1540`) stays: `GC/sheet.rs:239` still uses it.
- **Table:** the `table/state.rs` claims; the alignment of `Column::text_center` /
  `text_right` is applied now (`GC/table/state.rs:1527-1532`), and the showcase sets none.
- **TimeField:** the `px_1` move (`GC/time/time_field.rs:118-126`).
- **Sheet:** the safe-area inset; zero on a desktop.
- **Input:** the accessibility refactor into `InputAccessibility`. "input_px then refine_style"
  still holds.
- **The Toolbar panel is §6's**, not this section's.

**Gates:** both showcase citation gates and `the_fallback_stack_copy_is_gpuis` pass on both
cores, and `scripts/check_release.sh --commit` is green. This closes the unit.

### 4.3 Read by hand: what no gate reads (after the unit)

**Set-aside citations outside `SC/info/`:**

- `CN/src`, `CN/tests`, the README, `docs/showcase-exceptions.toml` (it cites changed files'
  lines: `root.rs:552`, `input/input.rs:311`, `:596`, `:771`, `:819`, `button/button.rs:737`,
  `scroll/scrollable.rs:482`, as of `8086a20e`), `scripts/connector_parity_exceptions.toml`,
  `tools/showcase-support/src`;
- the showcase's other files (`SC/demo.rs`, `SC/support.rs`, `SC/tests.rs` and the rest);
- the egui and iced showcases' gpui citations (§4.1's scope).

Each is read in the new source and re-pointed or rewritten. A reason text in
`docs/showcase-exceptions.toml` that occurs verbatim in several groups gets the same rewrite in
each: `scripts/check_showcase_parity.py` keys its merge groups by identical reason text.

**`DrawnFamilies`' docs (`SC/inspector.rs:292-328`, the struct's and `of`'s):** rewritten against the new cache
(rationale §1.9):

- `TextSystem::resolve_font` (`GP/text_system.rs:352`, the panic at `:621`);
- `resolve_font_uncached` (`:570`), which caches a family that loads as `CachedFont::Direct`,
  and one that does not as `Fallback` of the stack family drawn instead, itself `Direct`
  (`:244`);
- `get_font_for_id` (`:336`), which reads `Direct` only;
- gpui-fast 0.1.12's counterparts, each read there;
- the platform crates' per-family ids (gpui-pre-wgpu 0.3.10 and gpui-fast-wgpu 0.1.12
  `src/cosmic_text_system.rs`, the apple and windows crates), re-read and re-cited.

The reasoning, that the id reads back as the drawn family, is unchanged.

**The connector's dialog docs (`CN/src/geometry.rs:572-624`, `:1176`, `:1400`):** "Reduced
motion skips every entrance upstream (`:621-623`)" becomes a citation of `exit_duration`
(`GC/dialog/dialog.rs:558-562`) for the entrance and the exit. The rest follows §4.1's
renumbering or this hand-read.

**Version-stating prose:** `grep -rn '0\.8\.0\|0\.3\.9\|0\.1\.10' connectors/native-theme-gpui
connectors/native-theme-egui/examples connectors/native-theme-iced/examples
docs/showcase-exceptions.toml scripts/connector_parity_exceptions.toml
tools/showcase-support/src native-theme/src`. The count at the start goes in the commit body.

`native-theme/src` holds two gpui citations outside the script's scope: `src/lib.rs:354` and
`src/gnome/mod.rs:572-576`. Both cite gpui-pre-linux 0.3.9 `src/linux/x11/client.rs:2671-2678`
and `:2620-2666`. Both are re-read at 0.3.10 and re-cited (rationale P9). In the grep's output,
native-theme's own version numbers are history or are not gpui's.

Two kinds of line are not this task's:
- **`CN/README.md`'s version statements** are §9's (Task 8), and its `toolbar` row is §6's
  (Task 5), so each README edit has one owner;
- **the README's `<!-- compat:begin -->` … `<!-- compat:end -->` line**, which only
  `scripts/update_compatibility.sh` writes.

- **A present-tense claim** gets the new version or is rewritten. Examples: the module headers
  that say "verified against 0.8.0", `docs/showcase-exceptions.toml:20-22`,
  `scripts/connector_parity_exceptions.toml:491` ("gpui-component 0.8.0").
- **A present-tense measurement** is re-measured, not version-swapped. Examples: the
  `CN/src/contract.rs` statements "nothing reads … in 0.8.0" are re-grepped in `GC` and `GB`.
- **A historical statement stays.**

**Stated counts are re-measured:** the `ThemeColor` field count (139, held by its test) and
`IconName` (unchanged: gpui-kit-assets 0.8.1 changed only its manifest and examples).

**Gates:**

- the digits-only check over the re-pointed lines alone, with each new line read by hand;
- a second run of the grep that finds only history or excepted lines, listed in the commit
  body;
- `--commit` green.

---

## 5 — The gate for upstream functions that panic on their input (P6)

**The inventory.** Collect every doc or comment line that mentions a panic
(`grep -rEn '^\s*//.*\b[Pp]anic'`, doc comments and plain ones) in the `src/` of `GP`, `GF`,
`GB`, `GC`, gpui-kit 0.8.1, and the gpui-pre and gpui-fast sub-crates at 0.3.10 and 0.1.12.
There are 279 (172 doc-comment lines, 107 plain; rationale P6) at these
versions. Sort each public function they document:

- **Listed** where the documented panic depends on an argument's value, whether or not the
  connector calls the function today. Known:
  - `KeyBinding::new` (`GP/keymap/binding.rs:32`);
  - `TextSystem::resolve_font` (`GP/text_system.rs:351`);
  - `WindowTextSystem::shape_line` (`:668-670`, the `impl` at `:641`);
  - the line-boundary function at `GP/text_system/line.rs:280-281`;
  - `Bounds::inset` ("may panic", `GP/geometry.rs:1101`), if its doc's condition is an
    argument's;
  - gpui-base's scroll-bounce parameters (`GB/scroll_bounce.rs:65`, `:93`) and motion ratio
    (`GB/motion.rs:484`).
  A gpui-fast function at the same path needs no entry of its own where clippy resolves the
  path in both cores' `gpui` crate. The done-criterion shows whether it does; where it does
  not, the gpui-fast path gets its own entry.
- **Not listed** where it is a panic on framework state or misuse: a global never set, a
  re-entrant update, a wrong arena or App, the test and bench harnesses.
- **Set aside** where the line is not a function's panic, such as a "does not panic" remark.

The commit body holds the whole sorted list: each candidate, its group, and for the not-listed
ones the reason.

**`CN/clippy.toml`**, new. clippy reads the `clippy.toml` nearest the package's manifest, so it
covers the connector, its tests and its showcase, and no other crate. It holds one
`disallowed-methods` entry per listed function:

```toml
disallowed-methods = [
    { path = "gpui::KeyBinding::new", reason = "panics on a parse error (gpui-pre 0.3.10 keymap/binding.rs:32); use KeyBinding::load" },
    # … one entry per listed function, each reason citing its doc line
]
```

**The path of each entry** is the one that makes a planted call fail:

- first the re-export path (`gpui::KeyBinding::new`);
- where clippy does not resolve that, the defining module's path.

The two cores define the items in different crates under one library name, `gpui`.

**Deny level:** `#![deny(clippy::disallowed_methods)]` in `CN/src/lib.rs` (beside `unwrap_used`
and `expect_used`, `:200-201`) and in the showcase's crate root (`SC/main.rs`), so a listed call
is an error in every clippy run, not only under `-D warnings`.

**Today's calls.** The lint flags every call of a listed function, guarded or not, so each call
needs `#[allow(clippy::disallowed_methods)]` at its site, with the reason. A guard, where one is
needed, comes in addition to the allow, not instead of it.

- **`SC/inspector.rs:341`, `resolve_font`:** allowed. It is reached only where the text
  system lists a fallback-stack family, `DrawnFamilies::of`'s existing guard, and its doc
  states the one case left.
- **`SC/demo.rs:2384`, `shape_line`** in `segmented_width` (a `debug_assert!` on a newline,
  `GP/text_system.rs:678-681`): allowed, with this reason. Its one non-test caller passes
  `BASIC_SEGMENTS` (`SC/pages/basic.rs`), constants without a newline. The showcase tests call
  it in debug builds, where the `debug_assert!` would fire (`the_segmented_width_is_the_drawn_controls`
  among them). No new test is needed.
- **The test modules** that call listed functions with literals (`shape_line` in `SC/tests.rs`
  at `:1969`, `:4945`, `:5231`, as of `8086a20e`) carry an allow on the module or the item,
  with the reason "test code may panic".
- Any other call the inventory finds is decided the same way.

**Done-criterion, mechanical, for every entry** (rationale P6). A wrong path flags nothing and
passes every run. A clean run proves nothing for GPUI Fast: gpui-pre stays in its crate graph
(gpui-kit and gpui-base depend on it without `optional`). Both crates are named `gpui`, and
clippy warns about a path only when it is in none of them. So the proof counts errors.

1. **A planted function.** It is temporary, in the showcase's non-test code (for example in
   `SC/demo.rs`), and carries `#[allow(dead_code)]`.
   - It calls every listed entry once, written as the showcase writes its gpui calls.
   - Its parameters supply what the calls need (`&mut App`, `&Window`, a `Font`, …).
   - Its doc says it is the G4 proof and is removed before the commit.
2. **The count.** Run `cargo clippy -p native-theme-gpui --all-targets -- -D warnings` with
   `--features native,watch`, with `--features gpui-fast,native,watch` and with
   `--no-default-features`. In each, clippy reports exactly one `clippy::disallowed_methods`
   error per entry the configuration builds, each with its entry's reason.
   - A missing error is an entry that gates nothing there: its path is corrected, and the run
     repeated.
   - The output of the three runs goes in the commit body.
3. **`allow-invalid = true`** is set only on an entry whose path is in no crate of some Linux
   configuration, such as a gpui-fast-only function or a platform crate that Linux does not
   build. Each such entry states its reason. One that none of the three configurations
   builds is recorded as not proven on Linux.
4. **Removal.** The planted function is removed. The three runs then pass, with no
   unresolved-path warning in their output ("does not refer to a reachable").

**The upgrade checklist:** a line in `CN/clippy.toml`'s header comment says that the sweep and
this planted-function count are redone at each GPUI Kit upgrade (rationale P6).

---

## 6 — Toolbar: the unstated height is the content's (P7)

**`CN/src/geometry.rs`, `toolbar` (`:1023-1038`).** The `None` arm of `t.bar_height` becomes
`None => r.min_h_auto()`, in the same style as `.h_auto()` above it. The `min_h` helper allows
`auto` (gpui-pre-macros 0.3.10 `src/styles.rs:877-882`, `auto_allowed: true`), which restores
gpui's default minimum, `Size::auto()` (`GP/style.rs:789`).

**The doc (`:993-1022`):** the `Toolbar`'s Small size sets `min_h_8` (`GC/toolbar.rs:254`). The
builder replaces it with `bar_height` where the theme states one, and with `auto`, the
content's height, where it states none. It sets the height to `auto` as before.

**The two unit tests that assert the unstated case** (rationale P7) both change:

- **`toolbar_leaves_min_height_unset_without_a_bar_height` (`:2981-2994`), the red-first test:**
  - its `min_size.height` assertion with `bar_height = None` expects
    `Some(gpui::Length::Auto)`;
  - its stated-height half (`Some(40.0)`) is unchanged;
  - its name and doc, which say "no floor", become the reset: for example
    `toolbar_resets_the_min_height_without_a_bar_height`, "the minimum is reset to `auto`, so
    the bar is its content's height, whatever upstream's `Toolbar` sets".
  - It is one fixture with an explicit `None`, so it fails on today's builder whatever the
    presets state. The failure is kept for the commit body.
- **`toolbar_carries_the_models_toolbar` (`:2811-2861`):** the bar-height assertion
  (`:2824-2828`) expects `Some(px(h).into())` where `bar_height` is stated and
  `Some(gpui::Length::Auto)` where it is not. It holds the fix for every preset. Today 14 state
  none; only macos-monterey, material-3 and windows-11 state one.
- Then the fix makes both pass.

**`CN/README.md:365`** (the `toolbar` row) states the same. **The Widget Info claim
`SC/info/buttons.rs:902`** reads: `min_h_8, p_1, gap_1 and text_sm at its default Small Size
(gpui-component/toolbar.rs, RenderOnce for Toolbar)`, then that `geometry::toolbar` sets the
height to the content's and the minimum to `toolbar.bar_height` where stated, else to `auto`.

**`SC/tests.rs:1410-1413`:** the comment "nothing to hold it to" gains why: the chrome's row,
which this test measures, is the showcase's own div, not upstream's `Toolbar`; the builder's
unit test holds the unstated case.

**The sizing is then 0.8.0's exactly**: the height `auto`, the minimum `auto`, and upstream's
padding and gap where the theme states none. So no screenshot taken after this task shows the
floor (rationale P7).

---

## 7 — The Separator, the issue drafts, parity, the todo list (P10, P11, P12)

**The Separator (P11): a showcase test, on both cores.**

- **The test**, `the_labelled_separator_centres_its_label`, in `SC/tests.rs`:
  - it opens the Layout page;
  - it takes the labelled Separator's bounds by its selector, `layout-separator-labelled`
    (`SC/pages/layout.rs:32`, set at `SC/demo.rs:6653`);
  - among the painted quads inside those bounds (`scene_quads`,
    `CN/src/widgets/scene_read.rs:22`), it finds the label's box. That is the one filled with
    the theme's `tokens.background` (`GC/separator.rs:149`) and narrower than the line;
  - it asserts that the box's horizontal centre is within 1 px of the Separator's.
  It runs on both cores through the usual suites.
- **If the label's box cannot be told apart** from the other quads on a core, the fallback
  is decided per core (rationale P11). On GPUI Fast the scene reader skips composited scroll
  layers (`CN/src/widgets/scene_read.rs:15-18`).
  - On a core where the box is found, the test asserts there, with a `cfg` and its reason
    where it is one core only.
  - On the other core, a desktop capture is made, announced first:
    - start the showcase with `--capture --tab layout`, on that core (the default features,
      or `--features gpui-fast`);
    - capture it with `scripts/capture_window.sh`'s `capture_showcase gpui <pid> <png>`
      (`:300-372`);
    - read the label's and the line's x-ranges from the PNG.
  - Where the test holds on neither core, it is recorded as not written, with the reason.
- **Centred on both cores:** the `docs/todo.md` entry *File the gpui-kit issue: on gpui-pre a
  labelled `Separator` draws its label at the line's end* closes. Its dated line names taffy
  0.15.0's `src/compute/flexbox.rs:2274-2275` and the test (or the captures' x-ranges), and the
  entry moves to the archive.
- **Not centred:** the entry stays open, with a dated line giving the measured positions, and
  the rationale's §1.8 is corrected in its own commit.

**The two other drafts (P10)** stay open. Their text is not rewritten: `docs/todo.md` is
append-only, and the text names its versions. Each gets a dated line, *GPUI Kit 0.8.1
(2026-10-26, G4):*, saying it still holds, with its new lines:

- **the rounded clip:** `GP/window.rs:2239-2246`, and gpui-fast 0.1.12's `rounded_clip.rs` and
  `style.rs` lines as read there;
- **`try_resolve_font`:** `GP/text_system.rs:352`, the panic at `:621`, and gpui-fast 0.1.12's
  as read there.

**Both cores' layouts compared (P11)**, as archived G3 spec §9 did. Announced first, since it
takes the desktop's focus:

- **The dumps:** `--dump-layout` for kde-breeze and adwaita, light and dark (four runs), from
  the showcase built with its defaults and built with `--features gpui-fast`, at rest.
- **The comparison:** each pair compared element by element, every element's position and size
  equal, as a JSON diff.
- **A difference** is fixed in its own commit, gated, or explained in the ledger. Both cores
  now share taffy 0.15.0, so a difference G3 recorded may be gone; one that is gone is noted.

**Parity.** On the final layout (after §6), the Linux parity run, as
`scripts/generate_assets_release.sh:126-156` makes it:

1. `bash scripts/generate_screenshots_{gpui,iced,egui}.sh <DUMP_DIR>`, with `DUMP_DIR` a new
   directory in the session's scratchpad;
2. `python3 scripts/check_showcase_parity.py <DUMP_DIR>`.

What follows from it:

- a new difference is fixed, or excepted with its measured reason;
- an exception that no longer matches anything is removed;
- the tracked captures the screenshot scripts overwrite, in each connector's `docs/assets/`,
  are restored afterwards (`git checkout --` of the three asset directories). New screenshots
  are the close-out's `generate_assets_release.sh` run, which stamps them
  (`docs/assets/PROVENANCE.toml`).

**The upstream entries re-checked (P12).** Every open `[upstream]` gpui entry of `docs/todo.md`
whose cited upstream file changed in 0.8.1's non-test code
(`git diff --name-only v0.8.0 v0.8.1 -- crates/base/src crates/component/src`, test-only
changes excluded, plus the gpui-pre and gpui-fast files that changed) is re-read at 0.8.1:

- **resolved by 0.8.1:** it closes, with a dated line naming what resolved it, and moves to the
  archive;
- **still so:** a dated line *GPUI Kit 0.8.1 (2026-10-26, G4):* says so, with its 0.8.1 lines
  where they moved.

Each cited number's 0.8.1 line is found with §4.1's `lookup.py`, not by eye. A number it sets
aside is read in the diff. Entries citing
iced or egui files of the same name are not in scope.

**Also dated, as they wait for a release:**

- *`unicode-properties` back to 0.1.4 when gpui-fast-web unpins it*: gpui-fast-web 0.1.12 still
  pins `=0.1.3`.
- *File the gpui-fast issue: `Window::painted_quads()` omits …*: unchanged in gpui-fast 0.1.12
  (`GF/window.rs:2925-2927`).

**`ROADMAP.md`'s upstream list** (`:137-171`) is re-checked the same way. Its bullets are
re-cited in place, as at gpui-component 0.8.1, since ROADMAP states the current plan.

---

## 8 — New upstream API recorded (P13)

**`docs/todo.md`, two new entries:**

- ***Dialog: the platform's vertical placement*, `[1.0]`.**
  - The seam: `Dialog::centered` / `AlertDialog::centered` (`GC/dialog/dialog.rs:495`,
    `GC/dialog/alert_dialog.rs:276`).
  - Upstream's default: top-anchored at `margin_top` or a tenth of the viewport
    (`GC/dialog/dialog.rs:667-673`; the setter `Dialog::margin_top`, `:488`).
  - What it needs: a `platform-facts.md` row per platform, a `[dialog]` field, presets,
    inheritance, and the three connectors.
  - Why `[1.0]` (settled before 1.0): the placement is a native-look fact on every platform
    and needs platform research. It cannot be a `[v0.7.x]` patch, because `DialogTheme` is not
    `#[non_exhaustive]` (`native-theme/src/model/widgets/mod.rs:810-815`); a later minor may
    carry it.
- ***Markdown: inline code, list markers and blockquotes in the model*, `[post-1.0]`.**
  - The seams: `TextView::inline_code_style`, `list_style` and `blockquote_style`
    (`GC/text/compat.rs:114`, `:121`, `:127`).
  - What exists: the connector already feeds `defaults.mono_font` as `mono_font_family`
    (`CN/src/config.rs:65`), which inline code falls back to.
  - What is missing: no platform-facts rows, and these are not platform controls.
- ***iced and egui: a gate for upstream functions that panic on their input*, `[1.0]`**
  (rationale P6).
  - The same inventory over iced, iced_aw and egui, and a `clippy.toml` per connector.
  - Its place is those crates' next upgrade, as §5's sweep is the gpui connector's.

**Widget Info:**

- **The Dialog's panel gains a `not_themeable("placement", …)` row:** upstream's default, top
  anchored at a tenth of the viewport unless the caller sets `margin_top`
  (`GC/dialog/dialog.rs:667-673`). The theme states no placement; the new todo entry is where
  that is planned.
- **The TextView panel's "inline code" row (`SC/info/typography.rs:375`, "accent again …
  (Tier U)")** keeps its fact, and adds that gpui-component 0.8.1 offers
  `TextView::inline_code_style` (`GC/text/compat.rs:114`), so the gap is native-theme's model,
  named in the new entry.

---

## 9 — Documents and the stamp (P14)

**`CN/README.md`:**

- the *Backends* section (`:822-837`): GPUI Kit 0.8.1, its example manifest line, and GPUI
  Fast 0.1.12 or later (gpui-kit 0.8.1's `gpui-fast = "0.1.12"`);
- the alias comment (`:144`): `version = "0.3.10"`;
- the rows at `:29`, `:57`, `:365`, and the rest of §4.3's grep.

**`docs/COMPATIBILITY.toml` and the README's Verified line** are written only by
`scripts/update_compatibility.sh`, after each connector's last source change:

- `run gpui`;
- `run iced` and `run egui` too, since §4 changes their showcases' citations and their stamps
  hash their examples (`scripts/update_compatibility.sh:74-78`).

Gates: `compat::the_verified_line_is_the_stamps` and `scripts/update_compatibility.sh check`.

**`CHANGELOG.md` `[Unreleased]`:**

- ***Changed*:** the entry that states the floors ("requires gpui-component, gpui-base and
  gpui-kit 0.8.0 and gpui-pre 0.3.9 …") is rewritten in place for 0.8.1, 0.3.10 and GPUI Fast
  0.1.12, with the MSRV as measured in §1.
- ***Added*:** a `**Development**:` entry for `CN/clippy.toml`'s gate, beside the other gates'
  entries.
- No entry for §2, §3 or §6: no released version had those defects (rationale P14).

**`ROADMAP.md:72-73`:** GPUI Kit 0.8.1 on gpui-pre 0.3.10, GPUI Fast 0.1.12. Its upstream list
(`:137-171`) is §7's.

**`docs/todo_gpui-full-theme.md`:** an appended "gpui-component 0.8.1" note: the theme is
unchanged; the Toolbar's floor is reset where unstated; the Dialog exit follows its entrance.

**The review** (re-read before each edit):

- `docs/todo_code-review-plan.md`: the G4 row, decision D12 (the review plan's decision list, not the D round's ledger item D12);
- `docs/todo_code-review-findings.md`: the *G4* section and status row.

---

## 10 — Acceptance

**Both cores:**

- `cargo test -p native-theme-gpui --all-targets` passes, and the doctests on gpui-pre;
- clippy `-D warnings`, the strict panic lints and §5's gate are clean on the library and the
  showcase, and clippy also with `--no-default-features` (§0.2), each run clean in §5's sense;
- the MSRV is as §1.

**The full `./scripts/check_release.sh`** shows only the expected items:

- stale visual assets;
- the existing `v0.6.1` tag;
- the approved soft semver warnings;
- `cargo outdated` as recorded.

The iced and egui stamps are not among them: Task 8 re-takes both before this run (§9).

There is no gpui item. Right after it, the packaging check with `--features gpui-fast` passes,
as G3's acceptance ran it (`cargo package -p native-theme-derive -p native-theme -p
native-theme-gpui --allow-dirty --target-dir target/package-verify --features gpui-fast`).

**CI on the branch is green**, including *Test the gpui connector on GPUI Fast* and the macOS
and Windows GPUI Fast builds.

**The dependency canary, dispatched on the branch** (`gh workflow run dependency-canary.yml --ref
v0.7.0-rc1`), is green, or each failure is read and found outside the gpui stack (rationale
P15). It runs after the unit's push and after the close.
- **Before each dispatch**, the latest scheduled run on `main` (`gh run list --workflow
  dependency-canary.yml --branch main`) is read, and its failing steps and errors are noted
  as the baseline.
- A branch failure with the same step and error is that standing failure. Any other failure
  is G4's until shown otherwise.

**For the close-out's `generate_assets_release.sh`, these screenshots change:**

- the gpui Layout page, if §7 found the label centred;
- whatever the parity run of §7 found moved;
- the ColorPicker's hex text where a channel rounds differently (`GB/color_picker.rs:71`).
