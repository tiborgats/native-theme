# Issue #4: KDE Union — rationale

- Issue: <https://github.com/tiborgats/native-theme/issues/4>, "[Feature]: Support
  for KDE Union theme engine for native KDE looking apps", opened 2026-10-14 by
  @l-raider (who says they are a Rust beginner).
- Written: 2026-10-22, on `v0.7.0-rc1`.
- Status: **answered** — approved by the maintainer and the reply in §6 posted
  on 2026-10-22
  (<https://github.com/tiborgats/native-theme/issues/4#issuecomment-6351678088>);
  the backlog entry of §5 is in `docs/todo.md`, *KDE: Union, the Application
  Style that is data (issue #4)*. The issue stays open until the entry closes.

## 1. The request

The reporter builds apps with gpui-kit and native-theme on KDE, and adjusts "all
the widgets" by hand to look like Breeze. That only matches users who run Breeze.
They ask whether native-theme could use KDE Union, KDE's new style engine, so
that their apps follow whatever style the user runs.

## 2. Facts

Each fact was checked on 2026-10-22. Union is cited at its `master` commit
`2163195cbe` (GitHub mirror `KDE/union`).

### 2.1 What native-theme follows on KDE today

- The KDE reader reads `kdeglobals`: the colour scheme (any scheme, not only
  Breeze's), fonts, the icon theme, and `[KDE] AnimationDurationFactor` for
  reduced motion (0 means reduced); `kcmfonts` for the font DPI
  (`native-theme/src/kde/mod.rs:1-28`, `read_kde_ini` at `:66-95`,
  `reduce_motion` at `:277-283`).
- Widget sizes are Breeze's, whatever the user runs:
  `kde_metrics::populate_widget_sizing` writes constants from Breeze's
  `breezemetrics.h` (`native-theme/src/kde/metrics.rs:1-9`), and the reader's
  result merges over the `kde-breeze` preset (`linux_preset_for_de`,
  `native-theme/src/pipeline.rs:618-627`; the test at `:1565`).
- Nothing in `native-theme/src` or the connectors reads `[KDE] widgetStyle`,
  the user's Application Style.
- Why sizes cannot follow another Application Style: most Qt widget styles
  (Breeze, Oxygen, Fusion) are compiled C++ (a `QStyle`), not data, so there is
  nothing to read. Two style engines that load their looks from files instead
  are Kvantum (Option E) and Union (§2.3).

### 2.2 What gpui-component users get from the connector

`native-theme-gpui` installs colours, fonts and the global geometry (radius,
shadow, focus ring, scrollbar mode) into gpui-component's `Theme` once, and the
scrollbar sizes into gpui-base, which draws the scrollbars. Per-widget
geometry (heights, paddings, radii) cannot go there: gpui-component's `Theme`
has fonts, `radius`, `radius_lg`, `shadow`, `focus_ring`, `scrollbar_mode` and
list, sheet and notification settings, but no button height or padding
(`crates/component/src/theme/mod.rs`, `pub struct Theme`, in
`longbridge/gpui-kit` at `HEAD` on 2026-10-22). It comes from the `geometry`
module's builders, one per kind of widget it can reach, which the application
applies to each widget it creates (`connectors/native-theme-gpui/README.md`,
"What it does" and "Per-widget geometry"; both present in the released v0.6.1
and on `main`). Applying them is "adjusting each widget", which may be part of
what the reporter means.

### 2.3 What Union is and where it stands

- A style engine: a style is described once (input), turned into rules, and
  applied by outputs — a QtQuick Controls style and a QtWidgets `QStyle`
  (`doc/overview.qdoc`). Inputs: CSS, and Plasma SVG themes
  (`src/input/css`, `src/input/plasmasvg`).
- Plasma 6.8 (released 2026-10-14) ships Union as a **beta**. The announcement
  says "Union is not quite ready to be used by default"; users opt in by picking
  "Breeze (Union)" in System Settings → Colors & Themes → Application Style
  (<https://kde.org/announcements/plasma/6/6.8.0/>). Plasma 6.7 had an alpha.
- KDE invites people to write their own Union styles (the 6.8 announcement links
  "creating your own Union style").

### 2.4 How a Union style stores its values

- A style is a folder: `metadata.json` plus `contents/css/style.css`, which
  `@import`s further CSS files, and `contents/images/` (`doc/working-with-styles.qdoc`).
- Union parses CSS with the Rust `cssparser` crate, through
  `cxx-rust-cssparser` ("A C++ library for parsing CSS that uses the Rust
  cssparser crate internally", its README at
  <https://invent.kde.org/libraries/cxx-rust-cssparser>).
- The active style's id is `[KDE] unionStyle`, read through KConfig (the
  application's own file with `kdeglobals` beneath it), default `breeze`
  (`src/platform/plasma/PlasmaPlatformPlugin.cpp:27`); `breeze` is swapped for
  `breeze-mobile` on phones and `breeze-rtl` for right-to-left layouts (`:54-74`).
- Values are CSS custom properties, for example `--corner-radius: 5px;`
  (`src/input/css/styles/breeze/contents/css/variables.css:17`),
  `--scrollbar-size: 20px;` (`:29`), `--button-padding: 8px; /* 6 + 2 */`
  (`button.css:41`, under "Values derived from Breeze QStyle metrics", `:39`).
- Colours come from the user's colour scheme through Union's own functions, for
  example `custom-color("kcolorscheme", "active", "window", "background", "normal")`
  (`variables.css:40`), and are combined with `mix(...)` and `modify-color(...)`.
- Rules match elements by type, hint (CSS class) and state (`:hovered`,
  `:pressed`, `:checked`, `:disabled` …), with descendant and child combinators
  (`doc/css/support.qdoc`).
- Not yet supported but planned: units other than `px`, property inheritance,
  `@media` queries, multiple backgrounds, `:first-child`/`:last-child`,
  `visibility`, `calc()`, transitions and animations (`doc/css/support.qdoc:78-94`).
  So the way a style states a value is still changing.

### 2.5 Which Union styles exist

The Union repository ships three: `breeze`, `breeze-rtl`, `breeze-mobile`. On
2026-10-22, a web search and a search of store.kde.org through its API
(`api.opendesktop.org/ocs/v1/content/data?search=union`, the first 50 results
read; `search=unionstyle`, no results) found no other published Union style.

## 3. Decision 1: should native-theme support Union, and when?

The goal behind the request: an app built on native-theme follows the
**Application Style** the KDE user picked, not only the colour scheme.

### Option A — build a Union reader now

- Pro: the reporter's wish, sooner; native-theme is early on a direction KDE
  is taking.
- Con: Union is a beta that KDE itself does not enable by default (§2.3).
- Con: the style format is still changing (§2.4, the planned list); a reader
  written now follows every Union release.
- Con: no user gains anything yet. The only styles found are Breeze's (§2.5),
  whose values native-theme already has, so the reader would at best repeat
  today's values, and there is no other style to test it against.
- Con: it is more than parsing CSS, which the `cssparser` crate Union itself
  uses would do (§2.4). Read directly, the files also need the part of Union
  that decides which rule applies to which widget in which state, and the
  `var()`, `custom-color`, `mix` and `modify-color` functions, re-implemented
  (§5 names the alternatives).

### Option B — build it later, when set conditions hold (recommended)

- Pro: everything A gives, at the time it gives users something.
- Pro: written against a format KDE itself considers ready (§3.1), so fewer
  rewrites.
- Con: the reporter waits, with no date.
- Con: someone has to remember to check. Answered by a backlog entry checked at
  each Plasma feature release (§5), and the reply asks the reporter to post a
  non-Breeze style they find.

### Option C — decline: out of scope

- Pro: no work, no upkeep.
- Con: Union is KDE's own route to styles that are data and, of the two
  file-based engines looked at, the one that states shapes as numbers
  (Option E); Breeze-only geometry is a gap of the KDE reader (§2.1).
  Declining closes the best door to fixing it.

### Option D — use Union's own C++ library from Rust

- Pro: exactly Union's results, with no re-implementation.
- Con: Union is a C++ library that needs Qt 6.9 or higher and
  `cxx-rust-cssparser` (its README), plus KDE Frameworks for its Plasma
  integration (`src/platform/plasma`). Using it from native-theme means a C++
  toolchain and Qt at build time for everyone who enables it, even behind a
  cargo feature of its own, where the KDE reader today is plain Rust that reads
  text files; and it would not build where Qt is missing.
- Con: calling C++ from Rust goes through a bridge declared `unsafe`
  (`unsafe extern "C++"` in the `cxx` crate); the project allows no `unsafe`.

### Option E — read Kvantum themes instead (another data-based Qt style engine)

- Pro: Kvantum exists today, with many themes and users.
- Pro: part of a theme is numbers: its `.kvconfig` file states frame widths,
  text margins and colours (Kvantum's `doc/Theme-Config`).
- Con: the shapes are SVG pictures: a corner radius, for one, is drawn, not
  stated (same document). native-theme's model and the toolkits take numbers,
  and measuring them from pictures is guesswork, which the project's rules
  forbid; a Kvantum app would get the theme's margins with the wrong corners.
- Con: it is a different request, as large as Union's, for a third-party
  engine; KDE's own direction is Union. If someone asks for Kvantum, it gets
  its own issue and rationale.

### Option F — hand-copied presets for other Qt styles, picked by `[KDE] widgetStyle`

- Pro: would work today for, say, Oxygen or Fusion.
- Con: each style is C++ to transcribe and keep up to date, for few users, and it
  never reaches styles nobody transcribed. It does not answer the request.

### Decision

**B.** Support Union with a reader in native-theme's KDE reader, once the
conditions in §3.1 hold. C gives up the best route to the goal; A, D, E and F
each cost more than they give now.

### 3.1 When to start (the conditions)

Start when **both** hold:

1. **KDE enables Union by default** in a Plasma release, for either of its
   outputs (Qt Quick or Qt Widgets applications): the style format is shared by
   both. This is KDE's own signal that Union is finished enough, and it is a
   yes/no fact in the release announcement. A list of supported CSS features
   would be a finer signal, but it needs a judgment call on each feature; a
   fixed date would be simpler still,
   but says nothing about whether Union is ready. KDE making it the default is
   the simplest test that answers that.
2. **At least one non-Breeze Union style is published** for users to install
   (store.kde.org, a distribution package or elsewhere). Without one, the reader
   gives users nothing they lack.

Both are needed: with only 1, the reader repeats Breeze; with only 2, the format
may still change under it. If users ask for a published non-Breeze style before
KDE makes Union the default, the maintainer decides then.

## 4. The other decisions

### 4.1 Keep the issue open or close it?

- Open: the request stays visible, others can add their votes, and the reporter
  is notified when it is done. Con: an open issue with no work for months.
- Closed as "not planned" / "later": tidy. Con: reads as a no, and it is a yes.

**Keep it open**, labelled `enhancement` as it is. The repository has no
"later"/"blocked" label, and adding one for a single issue is not worth it; the
reply states the conditions, and the backlog entry carries them.

### 4.2 What the reporter adjusts by hand

Three causes are possible, and the reply asks which:

- The reporter uses `native-theme` without `native-theme-gpui`, and so maps
  every value onto gpui-component by hand: the connector does most of that; the
  reply names it.
- Applying the `geometry` builders per widget (§2.2) is the designed way and
  not a defect; the reply points to the README section and the showcase.
- Typed numbers or colours (for example a 5 px radius) mean one of two things:
  the reporter did not find where native-theme gives the value (answered by
  pointing to it, and perhaps by clearer docs), or native-theme or the connector
  misses it — by the project's rule, a connector that ignores theme data is a
  bug to fix.

The reply also says why it matters: typed values only ever match the style
they were copied from; values taken from native-theme already follow the
colour scheme and other platforms, and would follow Union later with no change
to the app's code.

### 4.3 Compare native-theme's Breeze values with Union's Breeze CSS now? No.

Union's Breeze values say they are "derived from Breeze QStyle metrics"
(`button.css:39`) — the source native-theme's Breeze values already cite
(`breezemetrics.h`, `native-theme/src/kde/metrics.rs`) — and they are
sometimes sums of it (`--button-padding: 8px; /* 6 + 2 */`, where the preset
states Breeze's `Button_MarginWidth` 6: `kde-breeze.toml`
`[light.button.border] padding_horizontal_px`). They add no authority, and a
mismatch would most likely show a Union bug rather than ours. Not worth a task.

### 4.4 Ask KDE for anything now? No.

A request to Union's developers now (for example the tool of §5) would be
cheap, and could shape Union while it is still changing. But what to ask
depends on the spec, which waits for §3.1, and a request with no work behind
it carries little weight. Ask when the spec is written.

## 5. Sketch of the future reader (not a spec)

Only to show the plan is feasible; the spec gets written when §3.1 holds.

- **Detect** that Union is the Application Style. Union's QtWidgets style is
  registered under the key `Union` (`src/output/qtwidgets/Union.json`; its own
  `union.themerc`, a hidden entry, states `[KDE] WidgetStyle=Union`). What the
  visible "Breeze (Union)" entry writes into `kdeglobals` is to be checked on a
  Plasma 6.8 system.
- **Find** the style: `[KDE] unionStyle` (§2.4), then the style folder. Where
  Union installs and looks up styles is not yet checked (`src/StyleRegistry.cpp`,
  `src/PackageHandler.cpp`).
- **Get the values**, one of three ways, for the spec to weigh:
  - read the CSS files at run time: parse them with the `cssparser` crate (the
    parser Union itself uses, §2.4), resolve `var()` and the colour functions
    against the colour scheme the reader already reads, and match the rules for
    a fixed list of element types and states that map onto native-theme's
    widget fields. Pure Rust, and it follows any style the user installs; but
    it re-implements Union's matching and functions and follows their changes;
  - or generate presets from published styles ahead of time, the same matching
    done once by a script in this repository, and pick one at run time by
    `unionStyle`. No parsing in the library; but a style the script did not see
    gets nothing, which misses the point of the request;
  - or run a KDE tool that prints the resolved values (no such tool exists:
    `union-styletool` creates, installs, uninstalls, updates, packages, lists
    and inspects styles, `doc/working-with-styles.qdoc`). Union's own results,
    and Union is installed wherever it is the active style; but KDE would have
    to provide it, and native-theme would run a program, as it already runs
    `xrdb` and `xrandr` for the font DPI (`native-theme/src/detect.rs:460`,
    `:484`).
- **Fill** only what the style states. Open question for the spec: a value the
  style does not state — fall back to `kde-breeze` (today's pattern, but under a
  non-Breeze style it mixes two looks) or leave it unset (the toolkit's own
  default)?
- **Scope**: CSS styles only. A Union style built from a Plasma SVG theme draws
  from pictures (Option E's problem) and stays out.
- Connectors need no change for values that fit native-theme's existing
  fields: they read the resolved theme, whatever filled it.
- Limit to state in the docs: a toolkit shows only what its widgets can take
  (a radius, a padding); a Union style that draws a widget from an image cannot
  come through.

The `[upstream]` entry in `docs/todo.md` states the two conditions of §3.1,
checked at each Plasma feature release.

## 6. The reply (posted 2026-10-22)

Written for a Rust beginner: no unexplained terms, each one said once, plainly.
Posted as it stands here.

---

Hi @l-raider, thanks for asking, and welcome to Rust! This is the right place
for this question.

**Short answer:** yes, Union is the right way for native-theme to follow more
KDE styles, and I plan to support it — but not yet. Union is still a beta, and
the only Union styles I could find look like Breeze. Details below, and a
question for you at the end.

### What native-theme already follows on KDE

On KDE, two separate settings decide how an app looks:

1. **Colours** — System Settings → Colors & Themes → Colors. native-theme reads
   the colour scheme picked there, whichever one it is, not only Breeze's. It
   also reads the user's fonts and icon theme.
2. **Shapes and sizes** — System Settings → Colors & Themes → Application Style.
   This decides how round the corners are, how much space there is inside a
   button, how wide a scrollbar is, and so on. Here native-theme always uses
   Breeze's values.

Why always Breeze? Because Breeze is KDE's default style, and because an
Application Style like Breeze is a program, written in C++, not a file of
values: there is nothing native-theme could read to follow a different one.
Breeze's values are copied into native-theme by hand, with notes on where in
Breeze's source code they come from.

So when a user picks a different **colour scheme**, your app already follows it.
When a user picks a different **Application Style**, it does not.

### Why Union would change this

A Union style is not a program. It is a folder of text files, mostly written
in CSS, the same kind of style files websites use. For example, Breeze for
Union says
[`--corner-radius: 5px;`](https://github.com/KDE/union/blob/master/src/input/css/styles/breeze/contents/css/variables.css).
Files like that can be read from Rust. So one day native-theme could read the
Union style the user picked and use its values — for any Union style, not only
Breeze.

If your app takes its values from native-theme, its code would not need to
change for that: it keeps asking native-theme for the values, and they come
from the Union style instead of from the copied Breeze ones.

### Why not now

- **Union is still a beta.** Plasma 6.8's
  [release announcement](https://kde.org/announcements/plasma/6/6.8.0/) says it
  is "not quite ready to be used by default". Users have to switch to it
  themselves ("Breeze (Union)" on the Application Style page).
- **The way Union styles are written is still changing.** Union's own
  [documentation](https://github.com/KDE/union/blob/master/doc/css/support.qdoc)
  lists features it plans to add, such as sizes in units other than pixels, and
  `calc()`, which works a value out from other values. Each of these changes how
  a value is written in the files, so code that reads them today would have to
  change each time Union grows.
- **Every Union style I could find is Breeze.** They are KDE's own: the normal
  one, one for right-to-left languages such as Arabic, and one for phones. I
  also looked on store.kde.org, KDE's store for themes, and found none there
  yet. So today, reading Union styles would give your app Breeze — which
  native-theme already gives you.

**My plan:** this issue stays open, and I check again at each new Plasma
version (6.9, 6.10 and so on).
When KDE makes Union the default style **and** there is at least one Union style
that does not look like Breeze, I teach native-theme to read Union styles. If
you see a Union style that is not Breeze before I do, please post it here.

One honest limit, so it does not surprise you later: even then, only what the
GUI library's widgets let you change comes through. A Union style can set a
corner radius or a padding, and the widgets of gpui-component (the widget
library inside gpui-kit) can take those. But if a style draws a button from a
picture, gpui-component's button cannot copy that.

### A question for you: what did you have to adjust by hand?

You wrote that you adjust all the widgets by hand to look like Breeze. Which
of these three do you mean?

- **You use `native-theme` without `native-theme-gpui`:** then you are passing
  every value to gpui-component yourself. The crate `native-theme-gpui` does
  that for you. Start there.
- **You apply `native-theme-gpui`'s per-widget functions:** that is expected.
  `native-theme-gpui` sets the colours, fonts and a few general sizes (such as
  the corner radius) once, for the whole app. But gpui-component has no setting
  for most per-widget sizes, such as a button's height or padding. So
  `native-theme-gpui` gives you small functions for those widgets, in its
  `geometry` module — for example `geometry::button` for a button — and you
  apply one to each widget you create. The README section
  [Per-widget geometry](https://github.com/tiborgats/native-theme/blob/main/connectors/native-theme-gpui/README.md#per-widget-geometry)
  shows how, and the
  [showcase example](https://github.com/tiborgats/native-theme/tree/main/connectors/native-theme-gpui/examples/showcase-gpui)
  uses them.
- **You typed numbers or colours yourself to match Breeze** (for example a
  corner radius of 5): native-theme should be giving them to you. Please tell
  me which widgets and which values. Then I can either show you where
  native-theme already gives that value, or fix it if it is missing.

This difference matters: values you type yourself only ever match the style
you copied them from. Values you get from native-theme already follow the
user's colour scheme and work on GNOME, macOS and Windows too — and later they
would follow the user's Union style, with no change to your code.

---
