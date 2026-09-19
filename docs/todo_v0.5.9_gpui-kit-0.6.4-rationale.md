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
(`9360978`) whose lockfile was moved with
`cargo update -p gpui-kit -p gpui-component -p gpui-base -p gpui-kit-assets -p gpui-pre`;
it was never committed. Every number below was measured on 2026-09-19 on the
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
  `webpki-root-certs`, `webpki-roots`), one arrives
  (`objc2-screen-capture-kit`, macOS only). No new `-sys` crate, so the CI
  `apt-get` package list needs no change.

So the compile fix is three deleted statements, one deleted showcase swatch and three changed numbers. The rest
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
| `from_system()` | anything | normally the same answer (same OS API) | normally the same answer, from overlapping sources: upstream reads only the XDG portal's `reduced-motion` key; native-theme's GNOME reader reads that key and falls back to `gsettings enable-animations` (`native-theme/src/gnome/mod.rs:197-199`), its KDE reader reads `AnimationDurationFactor = 0` from kdeglobals (`native-theme/src/kde/mod.rs:42-52`), and `AccessibilityPreferences::from_system()` reads `gsettings` alone (`native-theme/src/detect.rs:674-685`) |
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
size defaults and *then* `refine_style(&self.style)`. All fifteen changed
files still do (fourteen widget files and `root.rs`; specification §4 lists
old → new lines); `tooltip`, `popover`, `dialog/description`, `table`,
`progress`, `accordion`, `spinner` are byte-identical (`cmp`). But the upstream `file:line` citations in the connector's sources and README
(69 lines match the search of specification §7) are now stale, and — more important — the only
way this was established is a human reading fifteen diffs.

**(f) Unchanged where it matters:** gpui-base `src/theme.rs`, `src/styled.rs`,
`src/resizable/resize_handle.rs` and gpui-pre `src/color.rs`, `src/style.rs`
are byte-identical; `scrollbar.rs` changed internally (drag throttling, thumb
geometry) with no change to any `pub` item; the component `IconName` is the
same 101-variant set (`gpui-kit-assets/default-icons.txt`, identical in 0.6.1
and 0.6.4).

**(g) Ghost buttons now hover with `accent`** (upstream PR #3100; 0.6.0
`src/button/button.rs:1080-1085`: `secondary` ±10 % at 0.8 opacity; 0.6.4
`:1125-1132`: `tokens.accent`, halved in dark mode; pressed is
`tokens.button_active`). The connector maps `accent` to the platform accent
colour (`src/colors.rs:242`), which is right for the token's older uses: menu,
list, table, calendar and toggle highlights, where platform-facts records the
selection colour (`docs/platform-facts.md:1232`). For a *button* hover
platform-facts records a subtle fill on all four platforms (`:1168`), which
the connector already serves through `secondary_hover` ←
`button.hover_background` (`src/colors.rs:265`). So on 0.6.4 every ghost
button — the application's and the ones upstream builds internally in 17
files (title-bar and tab-bar buttons, dialog and sheet close, input clear,
pagination, date picker, sidebar, notification) — fills with the full accent
colour on hover. Nothing fails; it is visible only on screen.

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
| Exact pin `=0.6.4` | Rejected again, with open eyes. It would have prevented §1.2, but a connector and its application must share *one* gpui-component, so a pin forbids every consumer from taking 0.6.5 until a connector release exists; upstream ships weekly. The canary turned a silent break into a same-day report, which is the mitigation v0.5.8 chose (its rationale §2.14). Revisit trigger: a second compile break from a patch release within this 0.6 line (§4). |
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
| **Undo only the connector's own switch** | **Chosen.** `true`: if the flag is off, switch it on and remember that the connector did; if it is already on, do nothing. `false`: if the connector had switched it on, switch it off and forget; then, in either case, ask upstream to re-read the OS (`gpui_base::apply_system_reduce_motion`, public since 0.6.2, which writes only while upstream still owns the flag). |

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
(the portal is already followed) and under the test scheduler, and cannot
override the application, because upstream writes only a flag it still owns.
This is not the rejected `\|\|` option: the connector writes nothing of its own.

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
were run in the probe: 193 library tests pass, clippy is clean. The re-read
itself cannot be observed headlessly for the same reason; it rests on
upstream's documented contract (`reduce_motion.rs:52-64`) and its own tests.

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
| Use `gpui_kit::test` (`ElementSnapshot`, 0.6.2+) instead of raw `debug_bounds` | Rejected for now. It needs the `test-support` feature of gpui-kit, gpui-base and gpui-component and an `.test_support()` registration on an interactive element; `debug_selector` needs only the `gpui-pre/test-support` dev-dependency the crate already has, and measures the same box. |

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
its own test.

Three more builders set a height and are *not* rendered, for stated reasons:
`accordion_title` (applied to an inner title element that carries no
selector, so its box cannot be read from outside; `accordion.rs` is
byte-identical in 0.6.4), `dialog` (`min_h` / `max_h` of an overlay that must
be open) and `menu_item` (no upstream receiver, §1.4i). The remaining builders
set text size, padding, gaps or radii only. All of these keep their source
citation as evidence and nothing is claimed for them beyond that. Extending
the file later costs one builder function and one macro call.

### 2.7 The stale citations

All upstream `file:line` citations in `src/`, the showcase and the connector
README are re-verified against 0.6.4 / 0.3.5 and rewritten, because a wrong
line number under a claim like "refined at `:650`" is worse than none. They
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
| Re-map `accent` to the subtle button hover | Rejected. Menus, lists, tables, the calendar and toggles would lose the platform's selection highlight, which platform-facts `:1232` records; one token cannot carry both meanings. |
| Set `Theme::tokens.accent` (what the ghost button reads) apart from `ThemeColor::accent` | Rejected. Menu items, toggles and table cells read the same `tokens.accent` (`menu/menu_item.rs:117, 121`, `button/toggle.rs:155, 202`, `table/state.rs:2198`), so the split does not separate buttons from selections; and `Theme::change` rebuilds the tokens from the config's colours (`theme/schema.rs:1103`), which would undo it. |
| Ship a `ButtonCustomVariant` helper for applications (`geometry`-style: `native_ghost(native)`) | Partial. Fixes the application's own ghost buttons, not the 17 upstream-internal sites. Worth having only together with the upstream fix; not in v0.5.9. |
| **Record as Tier U, propose upstream a `ghost_hover` token (fallback `accent`), and look at it first** | **Chosen for v0.5.9.** Whether the accent fill is objectionable on Breeze (whose own flat-button hover is an accent-tinted frame) is a judgment for the maintainer's eyes; the showcase's "Button Variants" row and every dialog close button show it. The decision to do more is taken at the visual check (plan Task 8), with this section as the brief. |

### 2.11 Showcase coverage (§1.4h)

The showcase is the only place a human sees upstream widgets under native
themes, and it is the screenshot source. Added in v0.5.9, because 0.6.4 is
what makes them exist or changed them: `Carousel`, `Empty`, `InputGroup`; a
code editor (exercises the mono font, caret,
selection, the highlighter style of D41 and the now-public search); a
Markdown `TextView` (headings, inline code, code block, link, table, quote:
`muted`, `border`, `link`, the mono font). Weighed and left for a later
showcase pass, since 0.6.4 did not touch their theming: `DockArea`,
`Stepper`, `Rating`, `Pagination`, `HoverCard`, the command palette and the
chat components (`Bubble`, `Message`, `Attachment`).

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

---

## 5 -- Open questions for the maintainer

1. Ghost-button hover (§2.10): acceptable as upstream renders it, or does it
   need the application-side helper before an upstream fix lands? Decided at
   the visual check.

Answered 2026-09-19: 0.5.8 is not yanked (E14); implemented design documents
are archived (E15).
