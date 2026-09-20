# v0.5.9 — gpui connector on GPUI Kit 0.6.4: Rationale

Status: Design (2026-09-19); nothing implemented
Crates: `connectors/native-theme-gpui` (all code changes), workspace
`Cargo.toml` / `Cargo.lock` / `CHANGELOG.md` (version, lockfile, notes)
Companion specification:
[`todo_v0.5.9_gpui-kit-0.6.4-spec.md`](todo_v0.5.9_gpui-kit-0.6.4-spec.md)
Companion plan:
[`todo_v0.5.9_gpui-kit-0.6.4-plan.md`](todo_v0.5.9_gpui-kit-0.6.4-plan.md)
Predecessor: the v0.5.8 documents
(`todo_v0.5.8_gpui-component-0.6-{rationale,spec,plan}.md`), whose decisions
D1–D43 stand unless this document says otherwise.

---

## 0 -- What this document is for

The specification says what v0.5.9 does. This document says why: what was
measured, which alternatives were weighed, and what was left out on purpose.

Conventions. Upstream citations name the published crate sources
(`~/.cargo/registry/src/…/<crate>-<version>/`), e.g. "gpui-component 0.6.4
`src/theme/mod.rs:270-281`". "The probe" is a throwaway git worktree of `main`
(`9360978`) whose manifest was moved to the 0.6.4 / 0.3.5 floors and the
`tree-sitter-rust` dev feature first, and whose lockfile was then moved with
`cargo update -p gpui-kit -p gpui-component -p gpui-base -p gpui-kit-assets -p gpui-pre`
(the four tree-sitter crates arrive only because of that manifest edit); it was
never committed. Every number below was measured on 2026-09-19 on the
maintainer's machine (rustc 1.98.1) unless a source is given.

The brief: GPUI Kit 0.6.4 is out; the connector must be *absolutely*
compatible with it; compatibility with older GPUI Kit versions is not needed.

---

## 1 -- The situation, argued from evidence

### 1.1 What upstream published

| Crate | Versions since our floor | Dates (crates.io) |
|---|---|---|
| gpui-kit, gpui-component, gpui-base, gpui-kit-assets | 0.6.0 → 0.6.1 → 0.6.2 → **0.6.4** (no 0.6.3) | 09-03, 09-09, 09-18, 09-18 |
| gpui-pre | 0.3.3 → 0.3.4 → **0.3.5** | 09-03, 09-07, 09-14 |

gpui-component 0.6.2 and 0.6.4 both require `gpui-pre ^0.3.5` (published
`Cargo.toml:277-279`); 0.6.0 and 0.6.1 required `^0.3.1`. 0.6.4 differs from
0.6.2 in two files the connector never touches
(`gpui-component/src/chart/pie_chart.rs`, `gpui-base/src/positioner.rs`;
`diff -rq` of the two tarballs), so everything below that says "0.6.4" entered
with 0.6.2.

### 1.2 The released connector no longer compiles on a fresh resolve

0.6.2 removed the `Tiles` dock panel and with it three things the connector
names: `ThemeColor::tiles`, `ThemeConfigColors::tiles`
(gpui-component `src/theme/theme_color.rs`, `src/theme/schema.rs`; present in
0.6.1, absent in 0.6.2 and 0.6.4) and `Theme::tile_grid_size` / `tile_shadow`
/ `tile_radius` (`src/theme/mod.rs`; the connector never wrote these).

The connector's requirement is `gpui-component = "0.6.0"`, a caret. Cargo
therefore resolves 0.6.4 for every new project and every `cargo update`, and
**native-theme-gpui 0.5.8 as published on crates.io fails to build** there:

```
error[E0609]: no field `tiles` on type `&mut gpui_component::ThemeColor`      (src/colors.rs:487)
error[E0609]: no field `tiles` on type `gpui_component::ThemeConfigColors`    (src/config.rs:189)
error[E0609]: no field `tiles` on type `&gpui_component::ThemeColor`          (src/config.rs:189)
```

Evidence: the probe (first pass), and independently the nightly dependency
canary, which was green through 2026-09-17 and failed with exactly these
errors on 2026-09-18 (run 35393538690 at 20:49 UTC; 0.6.2 was published at 09:17 UTC and 0.6.4 at 16:35 UTC that day).
The canary did what it was built for; this document is the response.

Only consumers holding an older `Cargo.lock` still build. That is the reason
this is a release of its own and not an item for the next feature release.

### 1.3 The measured cost

Probe, second pass (the three `tiles` lines and the showcase swatch deleted,
nothing else touched):

- `cargo clippy -p native-theme-gpui --all-targets --all-features -- -D warnings`: clean.
  The showcase (6313 lines, the largest consumer of upstream widget APIs in
  the repository) compiles unchanged. No deprecation fires: upstream's new
  `on_change` on `Checkbox` / `Switch` / `Radio` keeps `on_click` as an alias.
- `cargo test -p native-theme-gpui --all-features`: 187 pass, 3 fail. All three
  are count tripwires doing their job (`colors.rs:1036`, `:1054` expect 139
  `ThemeColor` fields, now 138; `config.rs:353` expects 127 exported config
  colours, now 126). Every observer, base-layer and accessibility test passes
  on gpui-pre 0.3.5.
- `cargo +1.95.0 check -p native-theme-gpui --lib --locked`: passes, so the
  declared `rust-version = "1.95.0"` is still sufficient. It is also still
  necessary: `cargo +1.94.0 check … --ignore-rust-version` fails in gpui-pre
  0.3.5 itself (`src/profiler.rs:473, 494`, `std::hint::cold_path` is unstable
  on 1.94.0). The floor does not move.
- Lockfile closure: 13 packages leave (`aws-lc-rs`, `aws-lc-sys`, `cmake`,
  `syntect`, `bincode`, `fancy-regex`, `fs_extra`, `gpui-pre-http-client-tls`,
  `gpui-pre-reqwest-client`, `rustls-platform-verifier`(+`-android`),
  `webpki-root-certs`, `webpki-roots`) and six arrive: `objc2-screen-capture-kit`
  (macOS only) and, from the new `tree-sitter-rust` dev feature (§2.11),
  `tree-sitter`, `tree-sitter-json`, `tree-sitter-language`,
  `tree-sitter-rust` and `streaming-iterator`. No new `-sys` crate: the tree-sitter crates compile
  their bundled C with `cc`, and the C toolchain the closure already needed is
  on every runner, so the CI `apt-get` list (`ci.yml:41`) needs no change —
  and the one crate that wanted `cmake`, `aws-lc-sys`, is among the thirteen
  that leave.
- `cargo audit` on the moved lockfile: one vulnerability, `RUSTSEC-2026-0285`
  in `rustls 0.23.43`. It is **not** this release's doing — `main`'s lockfile
  has it too — but `ci.yml:117-123` runs `cargo audit` as a hard job on every
  push, so the release commit cannot be green while it stands. `cargo update -p
  rustls` moves 0.23.43 → 0.23.45 ("Locking 1 package"), and the audit then
  exits 0 (six allowed warnings for unmaintained crates remain, as on `main`).
  That one line is in this release (E19); nothing else about it is.

So the compile fix is two deleted statements (three compiler errors, because
`config.rs:189` names the field twice), one deleted showcase swatch and three
changed numbers. The rest
of this document is about the word *absolutely*: what changed upstream that a
compiler and the existing tests cannot see.

### 1.4 What changed upstream behind the compiler's back

Found by diffing the 0.6.0 and 0.6.4 sources of every upstream file the
connector imports from, cites, or re-implements.

**(a) gpui-base now reads the OS reduced-motion preference itself**
(new in 0.6.2: gpui-base `src/reduce_motion.rs`, called from
`gpui_base::init`). macOS and Windows are read synchronously during `init`;
Linux is read from the XDG portal "a moment after `init` returns" and then
followed live. Upstream decides whether it still owns the flag by comparing
`cx.reduce_motion()` with the value it last wrote (`apply_preference`,
`:89-101`): equal → it may write; different → "the application set it" and
upstream leaves the flag alone for as long as it differs.

The connector's `apply` and `apply_accessibility` call
`cx.set_reduce_motion(prefs.reduce_motion)` unconditionally
(`src/lib.rs:697, 756`). Against 0.6.0 that was the only writer. Against
0.6.4 it collides with upstream's:

| Caller passes | OS says | macOS / Windows (sync read) | Linux (async read) |
|---|---|---|---|
| `default()` (`false`) | reduce | connector writes `false` over upstream's `true`; upstream sees a foreign value and **stops following: animations run against the user's setting** | connector's `false` equals upstream's initial state, the portal answer arrives later and wins: reduced |
| `from_system()` | anything | normally the same answer (same OS API) | normally the same answer, from overlapping sources: upstream reads only the XDG portal's `reduced-motion` key; native-theme's GNOME reader reads that key and falls back to `gsettings enable-animations` (`native-theme/src/gnome/mod.rs:197-199`), its KDE reader reads `AnimationDurationFactor = 0` from kdeglobals (`native-theme/src/kde/mod.rs:42-52`), and `AccessibilityPreferences::from_system()` takes the platform reader's accessibility block and consults `detect_reduced_motion()` (`gsettings`, `native-theme/src/detect.rs:674-685`) only as a fallback when that block said `false` |
| `true` | no reduce | connector wins, upstream leaves the flag alone | same |

The first row is the problem: the preset entry points document
`&AccessibilityPreferences::default()` as the way to say "no text scaling"
(`src/lib.rs:212-214`), and a caller who picks a colour preset has not asked
to override the user's motion setting. With 0.6.4 the connector silently
switches off an accessibility feature that works without it, and does so on
two platforms out of three.

**(b) `Theme::change` now probes fonts** (gpui-component `src/theme/mod.rs:279-280`,
new `system_font.rs`, `mono_font.rs`). Both probes act only on upstream's
*default* family names (`.SystemUIFont`; `Menlo` / `Consolas` /
`DejaVu Sans Mono`); "a family set explicitly is used as-is"
(`system_font.rs:14-15`, `mono_font.rs` `resolve_default_mono_font`). The
connector always names both families from the resolved theme, in the styled
theme (`src/lib.rs:159, 164`) and in both `ThemeConfig`s, and its `apply`
writes the theme global directly rather than through `Theme::change`. The
probes therefore never alter a native family, with one benign exception: a
theme whose mono family *is* upstream's default name and is not installed gets
upstream's installed alternative instead of GPUI's generic fallback.

**(c) The base scrollbar projection gained mobile branches**
(`src/theme/mod.rs:314, 325, 335`, `.when(gpui_base::is_mobile(), …)`).
`is_mobile()` is a compile-time `cfg!(ios | android)`; native-theme has no
mobile readers and the connector's override writes width, inset and radius
itself, after upstream's. No effect on any supported target.

**(d) `ThemeConfig` JSON keys `chart_bullish` / `chart_bearish` became
`chart.bullish` / `chart.bearish`** (`schema.rs:381, 384`; `chart_bullish` / `chart_bearish` fields). The connector
fills the struct fields, never the JSON, so only an application that
serialises the connector's config with one upstream version and parses it with
another is affected; with a single floor that cannot happen.

**(e) Every styling seam survived, at new line numbers.** The `geometry`
builders rest on one upstream behaviour per widget: the widget applies its own
size defaults and *then* `refine_style(&self.style)`. Fourteen of the fifteen
changed files still do (thirteen widget files and `root.rs`; specification §4
lists old → new lines); the fifteenth, `dialog/dialog.rs`, re-applies one
property *after* the refinement (§1.4k); `tooltip`, `popover`, `dialog/description`, `table`,
`progress`, `accordion`, `spinner` are byte-identical (`cmp`). But the upstream `file:line` citations in the connector's sources and README
(69 lines match the search of specification §7) are now stale, and — more important — the only
way this was established is a human reading fifteen diffs.

**(f) Unchanged where it matters:** gpui-base `src/theme.rs`, `src/styled.rs`,
`src/resizable/resize_handle.rs` and gpui-pre `src/color.rs`, `src/style.rs`
are byte-identical; `scrollbar.rs` changed internally (drag throttling, thumb
geometry) with no change to any `pub` item; the component `IconName` is the
same 101-variant set (`gpui-kit-assets/default-icons.txt`, identical in 0.6.1
and 0.6.4). Comparing the field lists of the three structs the connector
writes or reads: `Theme` lost exactly the three `tile_*` fields and gained
none (23 → 20), and `ThemeConfig` is field-identical. `ThemeTokens` is not a
hand-written struct — `define_theme_tokens!` generates it from a name list
that mirrors `ThemeColor` (`theme/theme_color.rs:343-374`) — so it lost the
`tiles` token with the colour (139 → 138) and is otherwise unchanged. The
connector never names a token field, and the `ThemeColor` tripwire already
guards the list both structs come from.

**(g) Ghost buttons now hover with `accent`** (upstream PR #3100; 0.6.0
`src/button/button.rs:1080-1085`: `secondary` ±10 % at 0.8 opacity; 0.6.4
`:1125-1132`: `tokens.accent`, halved in dark mode, and the hovered *label*
switched to `accent_foreground` at `:1141`, where 0.6.0 had no `Ghost` arm and
fell through to `secondary_foreground` (`:919`); pressed is
`tokens.button_active`). The connector maps `accent` to the platform accent
colour (`src/colors.rs:242`), which is right for the token's older uses: menu,
list, table, calendar and toggle highlights, where platform-facts records the
selection colour (`docs/platform-facts.md:1232`). For a *button* hover
platform-facts records a subtle fill on all four platforms (`:1168`), which
the connector already serves through `secondary_hover` ←
`button.hover_background` (`src/colors.rs:265`). So on 0.6.4 every ghost
button — the application's and the ones upstream builds internally in 17 files
(`tab/tab_bar.rs`, `dock/tab_panel.rs`, `dialog/dialog.rs`, `sheet.rs`,
`input/input.rs`, `input/search.rs`, `input/group.rs`, `menu/app_menu_bar.rs`,
`pagination.rs`, `time/date_picker.rs`, `sidebar/mod.rs`, `sidebar/menu.rs`,
`notification.rs`, `clipboard.rs`, `setting/page.rs`,
`button/dropdown_button.rs`, `inspector.rs`) — becomes a selection-coloured
pill on hover, label included. Nothing fails; it is visible only on screen.

**(h) Three new components** (`Carousel`, `Empty`, `InputGroup`) read only
tokens that existed before: `radius` / `radius_tokens()` (computed from
`Theme::radius` on each call, `src/theme/mod.rs:471-480`), `motion_tokens`,
`foreground`, `muted`, `muted_foreground`, `border`, `input`, `ring`,
`danger`, `focus_ring`. All are mapped, and the tripwire proves no field is
left at zero. The editor and Markdown changes are API and performance work
and read no new token. What is missing is not mapping but *sight*: the
showcase renders none of the three, has no code editor and no Markdown view,
so nobody has looked at them under a native theme.

**(i) A v0.5.8 error surfaced while building the seam tests.**
`geometry::menu_item` is documented as the style for an "application-built
`MenuItem`" (`src/geometry.rs:92`, `README.md:153`, `docs/todo_gpui-full-theme.md:48`, v0.5.8 spec §9). There
is no such thing: `MenuItemElement` is `pub(crate)` and its module private in
0.6.0 and in 0.6.4 alike (`src/menu/menu_item.rs:10-11`, `src/menu/mod.rs:6`),
and `PopupMenu` builds its rows itself. The builder is still a correct
`StyleRefinement` for a menu row an application draws with its own elements,
and that is what the documentation must say.

**(k) The dialog's maximum height is no longer the caller's.** 0.6.4 applies
`.max_h(max_height)` — what is left of the viewport, `dialog/dialog.rs:535` —
*after* `refine_style`, inside a block upstream itself marks "There style is
high priority, can't be overridden" (refinement `:621`, override `:631`; 0.6.0
had no such line). `geometry::dialog`'s `max_h(px(d.max_height))` therefore no
longer reaches upstream's `Dialog`; its `min_h` still does, because
`min_h_24()` runs *before* the refinement. There is no `Dialog::max_h` prop to
route it through the way the width goes through `props.max_width`
(`:419-421`, which `geometry::dialog_max_width` already uses). This is the one
seam 0.6.4 actually took away, and no test would have caught it: a dialog must
be open to be measured (§2.6).

**(l) Selected rows lost their outline.** 0.6.4 removed the 1 px
`list_active_border` overlay `ListItem` drew while `Theme::list.active_highlight`
was on (0.6.0 `list/list_item.rs:249-255`) and the matching border on the
selected `Table` row; selection is now a flat fill. `ThemeColor::list_active_border`
still exists and the connector still maps it, but in 0.6.4 no widget reads it —
only `schema.rs` does, as the JSON fallback for `table_active_border`. Nothing
to change: the field must hold a value (`colors.rs:1054` proves none is left at
zero), and the mapping is already right if upstream restores the outline.

**(j) Nothing guards `Theme`'s own shape.** The three `tile_*` fields were
`Theme` fields the connector never set (§1.2). They arrived upstream, sat
through 0.6.0 and 0.6.1 at upstream's defaults while the connector overwrote
every neighbouring field, and were noticed only when their *removal* broke the
build. The compiler sees a field that disappears; nothing sees a field that
appears. The count tripwires cover `ThemeColor` (138) and the config export
(126); `Theme`'s own 20 fields have no guard at all. Four of them are left at
upstream's default today, correctly: `notification` (placement, margins,
`max_items`, width, delivery), `list` (`active_highlight`), `sheet`
(`margin_top`) and `motion` (eleven fields: four durations, three easings, two
springs and two travel distances, `theme/motion.rs:8-20`) have no
native-theme receiver —
`ResolvedTheme` models none of them, and native-theme's only duration model is
the spinner frame timing (`native-theme/src/model/animated.rs:133`).

---

## 2 -- Options considered

### 2.1 Release vehicle

| Option | Verdict |
|---|---|
| **v0.5.9 now, compatibility only** | **Chosen.** The published crate is uncompilable for new users (§1.2). v0.6.0 is reserved for the egui connector and is weeks of work. |
| Fold into v0.6.0 | Rejected: leaves crates.io broken for the duration. |
| Yank 0.5.8 | Rejected by the maintainer (2026-09-19). 0.5.8 still builds from an existing lockfile; once 0.5.9 exists a fresh resolve picks it. |

### 2.2 Version requirements

| Option | Verdict |
|---|---|
| **Caret floors moved to the release: `gpui-component`, `gpui-base`, `gpui-kit` `"0.6.4"`, `gpui-pre` `"0.3.5"`** | **Chosen.** The brief drops older versions, and the code cannot serve both sides of 0.6.2 anyway (a field either exists or does not). `0.3.5` is both the newest gpui-pre and the minimum gpui-component 0.6.4 accepts, so the two edges unify by construction. |
| Exact pin `=0.6.4` | Rejected again, with open eyes. It would have prevented §1.2, but a connector and its application must share *one* gpui-component, so a pin forbids every consumer from taking 0.6.5 until a connector release exists; upstream ships weekly. v0.5.8 chose the caret over a pin for this same reason (its rationale §2.14, the dependency-policy table); the nightly canary, added two days after that release, is what turns the residual risk into a same-day report, and it did so here (§1.2). Revisit trigger: a second compile break from a patch release within this 0.6 line (§4). |
| `cfg`-gate the `tiles` lines on the upstream version | Rejected: needs a build script sniffing a dependency's version, to support versions the brief says we do not need. |

### 2.3 The `tiles` colour

Delete the mapping (`colors.rs:487`), the config export (`config.rs:189`) and
the showcase swatch; move the tripwires to 138 / 126. There is nothing to map
to: upstream removed the widget, not just the token. native-theme never had a
`tiles` concept of its own (the value was `defaults.background_color`), so no
core type changes.

### 2.4 Reduced motion (§1.4a)

| Option | Verdict |
|---|---|
| Keep writing unconditionally, document "pass `from_system()`" | Rejected. Leaves the first row of the table: a documented call pattern that disables an OS accessibility setting on macOS and Windows, and behaves differently on Linux. |
| Stop forwarding; leave the flag to gpui-base | Rejected. native-theme reads sources upstream does not (`gsettings enable-animations` and KDE's `AnimationDurationFactor`, which answer on sessions whose portal lacks the `reduced-motion` key; `native-theme/src/gnome/mod.rs:197-199`, `src/kde/mod.rs:42-52`, `src/detect.rs:674-685`), and `apply_accessibility(true)` is a documented, tested way for an application to switch motion off. |
| Write `prefs.reduce_motion \|\| detect_reduced_motion()` | Rejected. The connector would *write* an OS reading of its own over a flag it does not own, behind a caller who passed explicit preferences, and the headless tests would depend on the desktop settings of the machine running them. |
| Write only `true`, never `false` | Rejected. A request could never be withdrawn (the existing test `apply_accessibility_forwards_reduce_motion` encodes that requirement). |
| Record the previous flag value on `true`, write it back on `false` | Rejected after tracing it. If the flag was already `true` (upstream's reading), upstream still owns it and may meanwhile follow the OS to `false`; writing the recorded `true` back would then switch reduction *on* against both the OS and the caller. A value recorded earlier is stale by construction. |
| **Undo only the connector's own switch** | **Chosen.** `true`: if the flag is off, switch it on and remember that the connector did; if it is already on, do nothing. `false`: if the connector had switched it on, switch it off and forget; then, in either case, ask upstream to re-read the OS (`gpui_base::apply_system_reduce_motion`, public since 0.6.2, which writes only while the flag still equals its own last reading). |

Why this is enough. The only thing to remember is one `bool`, and the only
value the connector ever restores is `false`, which is by definition what the
flag held before the connector touched it. After the undo the flag equals
upstream's initial or last-written `false`, so upstream owns it again; the
re-read then brings in the current OS value on macOS and Windows, where
upstream does not follow live (`reduce_motion.rs:55-58`: "a change made while
the application runs reaches it only through this call"). `default()` never
clears a flag somebody else set, identically on all three platforms.

Why the re-read runs on every `false`, not only after an undo. On macOS and
Windows the flag is normally switched on by upstream during `init`, before the
connector is called, so the connector never owns it. If the user then switches
the OS setting off and the application re-applies (`apply_system_theme`, or
`apply` with a fresh `from_system()`), 0.5.8 cleared the flag; a connector that
did nothing on `false` would leave it on until restart. The re-read is
upstream's documented way to refresh, costs one OS call, is a no-op on Linux
(the portal is already followed) and under the test scheduler. It overrides
the application in the one case upstream cannot detect: gpui-base compares
values, not provenance (`reduce_motion.rs:93-100`), so an application write
that happens to equal gpui-base's own last reading is indistinguishable from
it and is replaced when the OS reading changes. Specification §5.4 documents
that. This is not the rejected `\|\|` option: the connector writes nothing of
its own.

Limits, documented rather than engineered around: while the connector's
request stands, the flag differs from upstream's last write, so upstream
discards OS readings, including the Linux portal's first answer if `apply`
ran before it arrived; after the undo, Linux follows the OS again from the
portal's next change signal. An application that wants the flag forced either
way sets it itself after `apply`.

The `bool` lives in a private `Global` of its own rather than in
`NativeTheme`, because `apply_accessibility` deliberately works before any
`apply` (`src/lib.rs:693-698`) and must not create an empty `NativeTheme`.

Testability: `apply_system_reduce_motion` returns immediately under GPUI's
test scheduler (`reduce_motion.rs:65-68, 79-85`), so the behaviour is
deterministic in `#[gpui::test]`. "Somebody else set the flag" is simulated
with `cx.set_reduce_motion(true)` before `apply`. The code and its three tests
were run in the probe: the library suite passes (194 tests once §2.12's
tripwire is counted) and clippy is clean. The re-read itself cannot be
observed headlessly for the same reason; it rests on upstream's documented
contract (`reduce_motion.rs:52-64`) and its own tests.

### 2.5 Fonts (§1.4b)

No code change. Recorded because upstream's new module documents a cost the
v0.5.8 design did not know: a *named* family that is not installed is resolved
through GPUI's fallback stack on every text run, the failed lookup being
re-formatted each time (`system_font.rs:3-9`, upstream's statement, not measured here). OS readers return families the
desktop is configured to use, so this concerns presets applied on a machine
without the preset's font. Pre-existing, not a 0.6.4 regression, and the fix
(checking `cx.text_system().all_font_names()` and falling back to
`.SystemUIFont`, as upstream does for its defaults) is a behaviour decision
about which font a preset may be shown in. Deferred to `docs/todo.md` with
this evidence (§4).

### 2.6 Making seam compatibility mechanical (§1.4e)

v0.5.8 rejected rendered tests: "CI has no display, and the screenshot
workflow already covers rendering" (v0.5.8 rationale §2.23). The first half
was a wrong premise. `TestAppContext::add_window_view` renders into GPUI's
test platform with no display; upstream's own 0.6.4 suites measure layout that
way (`gpui-base/src/resizable/mod.rs:433-456`, `cx.debug_bounds` at `:440, 447, 449`). The second
half does not hold either: a screenshot shows that a button looks plausible,
not that it is 40 px high because the connector said so.

A spike in the probe settled feasibility: a `Button` wrapped in a
shrink-to-fit `div().debug_selector(..)`, drawn once in a test window,
measures **32 px unstyled and 40 px with `geometry::button` at text scale
1.5** (kde-breeze; 40 is the builder's own `size.height`). At scale 1.0 both
are 32 px, because Breeze's button height equals upstream's default, so a
rendered test must use an input where native and upstream differ or it proves
nothing. That is the same lesson as v0.5.8 error 53.

| Option | Verdict |
|---|---|
| Re-read the fifteen diffs at every upstream release | Rejected as the *only* defence. It is what was done today; it took the largest share of this analysis and leaves no artefact the canary can run. |
| **Rendered seam tests for the builders that set a box dimension the widget would otherwise fix itself** | **Chosen.** An integration-test file renders the real widget with and without the refinement and asserts the measured box. When upstream reorders a render so that its defaults win, the nightly canary fails the same evening instead of a user noticing a wrong size months later. |
| Use `gpui_kit::test` (`ElementSnapshot`, gpui-kit 0.6.1+ / gpui-base 0.6.1+) instead of raw `debug_bounds` | Rejected for now. It needs the `test-support` feature of gpui-kit, gpui-base and gpui-component and an `.test_support()` registration on an interactive element; `debug_selector` needs only the `gpui-pre/test-support` dev-dependency the crate already has, and measures the same box. |

Scope, kept deliberately small. Six builders set a height (or minimum
height) on a widget's root that competes with one the widget sets for itself,
which is exactly where a reordered upstream render would silently win: `button`,
`input`, `select`, `combobox`, `list_item`, `progress`. All six were written
and run in the probe (measured unstyled → styled: 32 → 40, 32 → 34, 32 → 34,
32 → 34, 34 → 30, 8 → 6 px); the negative control (the `refine_style` call
removed) was run on `input` and `combobox` and fails as it must. `select`
and `combobox` need the `adwaita` preset, because Breeze's combo-box minimum
equals upstream's own height. `combobox` returns the same refinement as
`select` but meets a different upstream seam (`combobox.rs:997`), so it has
its own test. The unstyled numbers are machine-independent: every one of them
is a rem multiple or a line height, and GPUI's default line height is the
constant φ (`gpui-pre 0.3.5 src/style.rs:494`), not a font metric —
`ListItem`'s 34 px is `py_1` plus 16 × 1.618 rounded
(`list/list_item.rs:184-193`). CI runs them on `ubuntu-latest`
(`ci.yml:85-101`).

What the six prove differs, and the file says so. `button`, `input` and
`progress` write the same `size.height` their widget writes for itself, so
they fail if upstream ever applies the caller's style before its own defaults.
`select` and `combobox` write `min_size.height`, and `list_item` a height the
widget leaves content-driven, so those three prove the refinement reaches the
root box and wins there — a weaker claim, and the honest one.

Three more builders set a height and are *not* rendered, for stated reasons:
`accordion_title` (applied to an inner title element that carries no selector,
so its box cannot be read from outside; `accordion.rs` is byte-identical in
0.6.4), `dialog` (an overlay that must be open — and its `max_h` no longer
reaches the widget at all, §1.4k) and `menu_item` (no upstream receiver,
§1.4i). The remaining builders set text size, padding, gaps or radii, except
the widths `tooltip` (`max_w`) and `button`, `progress`, `select` (`min_w`)
set, against which upstream writes nothing that could compete. All of these
keep their source citation as evidence and nothing more is claimed for them.
Extending the file later costs one builder function and one macro call.

The base layer is out of reach, not out of scope. gpui-base's scrollbar is
painted rather than built from elements, and the thumb rectangle it computes
lives in a private field of a private struct (`ThumbGeometry`,
`gpui-base 0.6.4 src/scrollbar.rs:1176-1231`; upstream's own headless tests
read it from inside the crate). Nothing outside gpui-base can measure it, so
the six connector values keep their source citation, re-read for 0.6.4 in
§2.7 — including the two clamps 0.6.4 added (`logical_length.min(track)` and
`inset.clamp(0, logical_length / 2)`, `:1210-1211`), which bound a thumb inset
larger than half the thumb where 0.6.0 did not.

### 2.7 The stale citations

All upstream `file:line` citations in `src/`, the showcase and the connector
README are re-verified against 0.6.4 / 0.3.5 and rewritten, because a wrong
line number under a claim like "refined at `:650`" is worse than none. The
search has to be wider than it looks: six comments continue a citation on the
next line in the bare form `` `:658-666` ``, with no file name on the line, and
five of those ranges are already wrong (specification §7 names them). The
repository's *other* documents — `docs/todo.md`, `docs/todo_gpui-full-theme.md`,
`ROADMAP.md` — keep their 0.6.0-era citations, knowingly: they are a gap
analysis and a roadmap, not the crate's own documentation, and re-verifying
them is a separate pass (§4). The one line Task 4 edits there is corrected
while it is open. They
are evidence for a specific version, so each module states the version once
in its header and the citations drop the repeated "0.6.0". No checker script:
a script can confirm a line exists, not that it still means what the comment
says; §2.6 covers meaning where meaning can be tested.

### 2.8 Icons

The 101-variant set is unchanged, so the three name tables need no edit beyond
their "0.6.0" wording. Two adjacent items were weighed and left out:

- *Keying the tables by `gpui_kit_assets::IconName`* (the `docs/todo.md`
  follow-up that waited for a 0.6.1 floor). The floor no longer blocks it, but
  it changes three public function signatures and is a feature, not
  compatibility. It stays in `docs/todo.md`, with its precondition marked met.
- *Refreshing the Lucide bundle 1.41.0 → 1.43.0* (upstream's assets are on
  1.43.0, `gpui-kit-assets/README.md:23, 51`). The bundle is native-theme's
  own and does not interact with upstream's. Recorded reason for not being on
  the newest, per the dependency policy: a glyph refresh needs the hand audit
  and changes every screenshot; it does not belong in a release whose purpose
  is to make the crate build again.

### 2.9 What is *not* re-verified

gpui-pre 0.3.3 → 0.3.5 changed `src/app.rs` by 123 lines. The connector's
contract with it (`observe_global` activation at the end of the flush,
notification dedup, `refresh_windows`, `defer`) is exercised by the observer
tests, which pass on 0.3.5; the citations are updated (§2.7) but the
mechanism is not re-derived from source. The tests are the evidence.

### 2.10 Ghost-button hover (§1.4g)

| Option | Verdict |
|---|---|
| Re-map `accent` to the subtle button hover | Rejected twice over. Menus, lists, tables, the calendar and toggles would lose the platform's selection highlight — the meaning upstream gives the token ("used for accents such as hover background on MenuItem, ListItem, etc.", `theme/schema.rs:254-255`) and the one platform-facts `:1232` records for macOS (`selectedContentBackgroundColor`) and KDE (`[Colors:Selection] BackgroundNormal`); on Windows and GNOME that row already holds the same subtle fill as the button row at `:1168`, so the conflict is a macOS/KDE one. And it would only half-work: the hovered ghost *label* is `accent_foreground` (§1.4g), so a subtle fill would carry selection-coloured text. |
| Set `Theme::tokens.accent` (what the ghost button reads) apart from `ThemeColor::accent` | Rejected. Menu items, toggles and table cells read the same `tokens.accent` (`menu/menu_item.rs:117, 121`, `button/toggle.rs:155, 202`, `table/state.rs:2198`), so the split does not separate buttons from selections; and `Theme::change` rebuilds the tokens from the config's colours (`theme/schema.rs:1103`), which would undo it. |
| Ship a `ButtonCustomVariant` helper for applications (`geometry`-style: `native_ghost(native)`) | Partial. Fixes the application's own ghost buttons, not the 17 upstream-internal sites. Worth having only together with the upstream fix; not in v0.5.9. |
| **Record as Tier U, propose upstream a `ghost_hover` token (fallback `accent`), and look at it first** | **Chosen for v0.5.9.** Whether the accent fill is objectionable on Breeze (whose own flat-button hover is an accent-tinted frame) is a judgment for the maintainer's eyes; the showcase's "Button Variants" row and every dialog close button show it. The decision to do more is taken at the visual check (plan Task 8), with this section as the brief. |

### 2.11 Showcase coverage (§1.4h)

The showcase is the only place a human sees upstream widgets under native
themes. It is also the screenshot source, but only for one tab: every capture
in `scripts/generate_gpui_screenshots.sh:70` and in `screenshots.yml` passes
`--tab buttons`, and all five new sections live in other tabs. They are
therefore looked at once, by eye, at the visual check (§9 gate 2) and leave no
artefact; widening the captured tabs is a follow-up in `docs/todo.md`, not
part of a compatibility release. Added in v0.5.9, because 0.6.4 is
what makes them exist or changed them: `Carousel`, `Empty`, `InputGroup`; a
code editor (exercises the mono font, caret,
selection, the highlighter style of D41 and the now-public search); a
Markdown `TextView` (headings, inline code, code block, link, table, quote:
`muted`, `border`, `link`, the mono font). Weighed and left for a later
showcase pass, since 0.6.4 did not touch their theming: `DockArea`,
`Stepper`, `Rating`, `Pagination`, `HoverCard`, the command palette and the
chat components (`Bubble`, `Message`, `Attachment`).

### 2.12 Guarding `Theme`'s shape (§1.4j)

| Option | Verdict |
|---|---|
| Leave it; re-read the struct at each upstream release | Rejected. That is what happened between 0.6.0 and 0.6.2, and it silently left three fields unmapped for two releases. It also leaves the canary blind. |
| Count the fields at runtime, as `ThemeColor` does (`Theme` derives `Serialize`) | Rejected. It catches a change but cannot say which field, and it costs a serialisation in the suite. |
| **Name every field in an exhaustive destructuring, in one test** | **Chosen.** `Theme` is not `#[non_exhaustive]` and all 20 fields are public, so `let Theme { … } = theme;` compiles only while the field list is exactly what the connector was written against. An added field is `error[E0027]: pattern does not mention field …` (measured), a removed one is an unknown-field error. No runtime cost, and the pattern itself documents which fields `to_theme` sets, which come from `Theme::from(&ThemeColor)`, and which are deliberately left at upstream's default. |

It is the smallest thing that turns the §1.4j failure mode into a build
failure the nightly canary reports, and it is the same tripwire idiom the
crate already uses three times.

---

## 3 -- Decision record

| # | Decision |
|---|---|
| E1 | Release as v0.5.9, compatibility only. |
| E2 | Floors: gpui-component / gpui-base / gpui-kit `0.6.4`, gpui-pre `0.3.5`; carets kept. `Cargo.lock` moved to the same versions. |
| E3 | `tiles` removed from the mapping, the config export and the showcase; tripwires 138 / 126. |
| E4 | Reduced motion: the connector undoes only its own switch, and every `false` asks gpui-base to re-read the OS (§2.4); private `Global` holding one `bool`; three new tests. |
| E5 | No font code change; follow-up recorded in `docs/todo.md`. |
| E6 | Rendered seam tests for `button`, `input`, `select`, `combobox`, `list_item`, `progress` in `connectors/native-theme-gpui/tests/seams.rs`, run by CI and the canary like every other test. |
| E7 | All upstream citations re-verified and rewritten for 0.6.4 / 0.3.5. |
| E8 | `rust-version` stays 1.95.0: measured sufficient (1.95.0 builds) and necessary (1.94.0 does not) on the 0.6.4 closure. |
| E9 | Icon tables: wording only. Assets-enum keying and the Lucide refresh stay out. |
| E10 | No CI workflow change (no new system package; verified from the lockfile diff). |
| E11 | Release gates as for v0.5.8: `./pre-release-check.sh`, screenshots regenerated on the maintainer's desktop (`Cargo.lock` is a stamped path), tag and upload only on the maintainer's explicit go. |
| E12 | Ghost-button hover: Tier U + upstream proposal; the maintainer judges it at the visual check (§2.10). |
| E13 | Showcase gains Carousel, Empty, InputGroup, a code editor and a Markdown view (§2.11). |
| E14 | 0.5.8 is not yanked (maintainer, 2026-09-19). |
| E15 | The v0.5.8 design documents move to `docs/archive/` in this release; these v0.5.9 documents follow once implemented (maintainer's standing rule, 2026-09-19). |
| E16 | `geometry::menu_item` documentation corrected: no upstream receiver exists; the function stays (§1.4i). |
| E17 | One test names all 20 `Theme` fields in an exhaustive destructuring, so a field added or removed upstream stops the build (§2.12). |
| E18 | `geometry::dialog` keeps `max_h`, but the documentation stops claiming it reaches upstream's `Dialog`: 0.6.4 overrides it with a viewport-derived value after the refinement (§1.4k). Tier U, with a `Dialog::max_h` prop as the upstream proposal. |
| E19 | The lockfile also takes `cargo update -p rustls` (0.23.43 → 0.23.45). `RUSTSEC-2026-0285` predates this release, but `cargo audit` is a hard CI job, so without it the release commit cannot be green (§1.3). |

---

## 4 -- Deliberately not done, and the trigger to revisit

| Item | Trigger |
|---|---|
| Exact pins on the gpui stack | A second compile break from a 0.6.x patch release. |
| Installed-font check for preset families (§2.5) | A report of slow text or wrong fallback with a preset; or upstream exposing its probe. |
| Icon tables keyed by the assets enum; Lucide 1.43 | The next icon work item. |
| `gpui_kit::test` snapshots | A seam that `debug_bounds` cannot measure but a snapshot can. |
| Re-deriving gpui-pre's effect-queue behaviour from source | An observer test failing on a new snapshot. |
| Forcing reduced motion against the OS | A user asking for it; needs an explicit API, not a `bool` in preferences. |
| Rendered tests for the other `geometry` builders | A builder gaining a box dimension, or an upstream change to one of those widgets that a reader cannot settle from the diff. |
| A rendered guard for the base-layer scrollbar geometry | gpui-base exposing the thumb rectangle outside the crate (today `ThumbGeometry` and its bounds are private, §2.6). |
| Screenshots of the tabs the new showcase sections live in | The next screenshot pass; today every capture is `--tab buttons` (§2.11). |
| Re-verifying the upstream citations in `docs/todo.md`, `docs/todo_gpui-full-theme.md` and `ROADMAP.md` | The next time one of those documents is revised; four of their citations are already stale at 0.6.4 (§2.7). |
| `geometry::dialog`'s `max_h` | Upstream accepting a `Dialog::max_h` prop, or making the caller's `max_h` win (§1.4k, E18). |

---

## 5 -- Open questions for the maintainer

1. Ghost-button hover (§2.10): acceptable as upstream renders it, or does it
   need the application-side helper before an upstream fix lands? Decided at
   the visual check.

Answered 2026-09-19: 0.5.8 is not yanked (E14); implemented design documents
are archived (E15).
