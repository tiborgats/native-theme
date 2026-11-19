# The text caret's blink in the model — rationale

Status: written 2026-11-14, revised after three reviews. B1 to B5 approved by the
maintainer on 2026-11-19, with the grant of B4's `unsafe` call on Windows; B6 decided
the same day: option (a), in 0.7.2, together with issue #4's KDE steps 6 and 7. The
specification and plan are [`todo_v0.7.2_caret-blink-spec.md`](todo_v0.7.2_caret-blink-spec.md)
and [`todo_v0.7.2_caret-blink-plan.md`](todo_v0.7.2_caret-blink-plan.md); the
specification's head says what was read again on 0.7.2's tree (GPUI Kit 0.10.0, egui
0.37.0), which this document's lines predate. The file's name carries the version it
was written during.

Abbreviations: `GC`, `GB` are the `src/` folders of gpui-component 0.9.1 and
gpui-base 0.9.1 in Cargo's registry; `EG` is egui 0.36.2's; `IW` is iced_widget
0.14.2's; `F` is `docs/facts/` on the parked branch `v0.8.0-rc1` (its checkout,
commit `78ee0efd`), and `P8` that checkout's `docs/`; `CG`, `CE`, `CI` are the
`src/` folders of the gpui, egui and iced connectors; other paths are in this
worktree at `0fa28027`.

## 0. The problem

KDE, GNOME and Windows let the user set how the text caret blinks, and
applications of the platform follow it: KDE's blink rate, GNOME's blink switch, time and
time-out, Windows's caret blink rate. The model has one caret field, its colour
(`InputTheme::caret_color`, `native-theme/src/model/widgets/mod.rs:121-122`), so a
native-theme application blinks at its toolkit's own rate whatever the user set,
and keeps blinking where the user turned blinking off.

Until now only egui could have taken a value. gpui-component 0.9.0 made the
caret's blink a theme field, `Theme::cursor_blink` (`GC/theme/mod.rs:218-220`),
and the 0.9.1 port keeps the application's value there because "native-theme does
not model" it (`CHANGELOG.md:12`). A visible property the model cannot state is a
gap in the model. This document decides the fields, their readers and what each
connector does with them.

## 1. Facts

### 1.1 What the platforms do

The parked 0.8.0 work verified these, each with its source quoted in `F`; they
are carried over with their citations, not read again here.

| Platform | Shown / hidden | Stops | Source of the live setting | Facts |
|---|---|---|---|---|
| KDE (Qt) | half the flash time each (integer division); 500 / 500 ms at the default 1000 | never | `kdeglobals` `[KDE] CursorBlinkRate`: unset is 1000; a positive value is bounded to 200..2000; 0 or less means no blink (plasma-integration 6.8.2 `khintssettings.cpp:79-80`) | `F/kde/tokens.toml:1599-1650` |
| GNOME (GTK) | half the blink time each; 600 / 600 ms at the default 1200 | after the time-out, 10 s by default | `org.gnome.desktop.interface` `cursor-blink-time` (default 1200, range 100 to 2500) and `cursor-blink-timeout` (default 10, in seconds, range 1 to 2147483647) (gsettings-desktop-schemas 51.0 `interface.gschema.xml.in:70-80`; `gtksettings.c:364-380`); the `cursor-blink` switch is not in `F`'s entries, the parked gap map places it at the schema's `:63-69` and `gtksettings.c:355` (`P8/todo_v0.7.1_model-completeness-gap-map-gnome.md:588`) | `F/gnome/tokens.toml:1732-1789` |
| Windows | one blink time each; 530 / 530 ms measured on the test runner | WinUI's text box stops after the whole periods that fit in 5 s (`F`'s note, from `TextBoxView.cpp:2742-2763`); the fact is pending there for a reason of its checking tool | `GetCaretBlinkTime`, or `UISettings.CaretBlinkRate` (`P8/todo_v0.7.1_model-completeness-spec-widgets.md:10812` names both) | `F/windows/tokens.toml:2711-2742` |
| macOS | not established (pending in `F`; the user defaults that hold it are undocumented and were unset on the test runner) | not established | — | `F/macos/tokens.toml:46129-46147`, `:47501-47518` |
| iOS | 650 / 350 ms, measured | never | none | `F/ios/tokens.toml:20607-20675` |
| Material (Compose) | 500 / 500 ms | never | none in the facts | `F/material/tokens.toml:1758-1787` |

The measured values (Windows, iOS) cite runs whose files are in a separate
evidence repository (`F/sources.toml:1233-1240`). Neither checkout has them in
its working tree, but both runs are in this repository's history: the Windows
run in commit `efbffffe` (`docs/evidence/winui-host/2026-11-06-run-42308526155/winui-host-windows-11-arm/frames.toml`,
`GetCaretBlinkTime = 530`), the iOS run in commit `72fcf6a7`
(`…/2026-11-06-run-42275028573/ios-host-macos-15/behavior_values.toml`,
`blink_on_s = 0.65`, `blink_off_s = 0.35`, `blink_timeout = "none"`). The first
commit is on `origin/v0.8.0-tools-hosts-m1b`; the second only on the local
branch `facts-ios-m1b3`.
The same facts files also state how the caret changes (at once or fading), its
width, and that it stays solid while the user types: other properties (§5).

### 1.2 What the toolkits can take

| Toolkit | Receiver | Lines |
|---|---|---|
| gpui-component 0.9.1 | `Theme::cursor_blink`, a `CursorBlink`: `blinking(interval)` shows and hides the caret for `interval` each (equal phases only); `steady()` never hides it; `with_idle_timeout(t)` stops the blinking, caret shown, `t` after the last focus, edit or movement, and a `t` of zero never starts it. Default: 500 ms, no time-out. Not part of a `ThemeConfig`. The caret stays shown for 300 ms after an edit, a constant. | `GB/input/base/blink_cursor.rs:5-73`, `:166-170`; `GC/theme/mod.rs:218-220`, `:463`; `GC/theme/schema.rs` (no blink key) |
| egui 0.36.2 | `Visuals::text_cursor`: `blink`, `on_duration`, `off_duration` in seconds; the caret is painted while the time in the cycle is below `on_duration`, so an `on_duration` of zero never paints it. Default: on, 0.5 and 0.5. No time-out, no fade. | `EG/style.rs:947-978`; `EG/text_selection/visuals.rs:290-317` |
| iced 0.14 | none: the interval is a private constant, 500 ms, in the text input and the text editor, and neither style has a caret field | `IW/text_input.rs:1693`, `:515-518`; `IW/text_editor.rs:544` |

### 1.3 What the repository does today

- gpui: `to_theme` leaves upstream's default; `apply` keeps the installed theme's
  `cursor_blink` (`CG/lib.rs:1331-1339`); the test `every_theme_field_is_named`
  lists it as left to the application (`:1650-1659`), and
  `apply_keeps_the_applications_own_settings` asserts it survives `apply`
  (`:3475-3507`). The README, the CHANGELOG and the ROADMAP say so
  (`connectors/native-theme-gpui/README.md:642-644`, `CHANGELOG.md:12`,
  `ROADMAP.md:93-94`).
- The tooltip's delay shows how the connector handles a value a theme may or may
  not state: `install_tooltip_delay` remembers the delay it last wrote; any
  other installed value is the application's own; where the theme states none it
  puts the application's back (`CG/lib.rs:1263-1281`).
- egui: the base style sets the caret's colour only (`CE/style/base.rs:217`);
  `CE/mapping.md:852-854` lists `blink`, `on_duration` and `off_duration` as left
  to egui, "caret blink is input behaviour with no native leaf".
- iced: the contract's table of values without a receiver is `UNREACHABLE`
  (`CI/contract/derived.rs:975-1090`); the caret's colour is in its colour list
  (`:1433-1440`).
- The closest existing value is `tooltip.show_delay`: an optional `u16` of
  milliseconds, category `soft_option`, stated by the presets with a citation,
  with no rule of inheritance and no reader
  (`native-theme/src/model/widgets/mod.rs:311-317`;
  `docs/property-registry.toml:274`; `docs/inheritance-rules.toml:461`). The
  registry's header defines an `_ms` key as `u16` (`docs/property-registry.toml:24-25`);
  the model's derive takes an optional field of any type in that category
  (`native-theme-derive/src/gen_structs.rs:38-60`).
- A behaviour a reader overrides from a live setting exists too:
  `scrollbar.overlay_mode`, which the GNOME reader takes from `overlay-scrolling`
  (`native-theme/src/gnome/mod.rs:637`).
- `InputTheme` is not `#[non_exhaustive]`; its resolved form is
  (`native-theme-derive/src/gen_structs.rs:25-27`). One place in the repository
  builds it with every field named and no `..Default::default()`
  (`native-theme/tests/proptest_roundtrip.rs:269-283`).
- The Windows reader creates a `UISettings` object and passes it to its reads
  (`native-theme/src/windows.rs:674`, `:447-448`), each of which gives up
  quietly where a call fails (`:450`, `:483`). `UISettings::CaretBlinkRate` is a
  safe method of the `windows` crate behind a feature the crate already enables
  (windows 0.62.2 `src/Windows/UI/ViewManagement/mod.rs:1921`;
  `native-theme/Cargo.toml:140`). `GetCaretBlinkTime` is an `unsafe` function
  (`src/Windows/Win32/UI/WindowsAndMessaging/mod.rs:722`); it is the call the
  measurement used and the one WinUI's own text box makes (`F`'s note,
  `TextBoxHelpers.cpp:55-89`).
- The readers for KDE, GNOME and Windows fill the variant of the mode the
  desktop is in; the pipeline fills the other from the preset
  (`native-theme/src/lib.rs:525-540`, `:563-569`,
  `native-theme/src/pipeline.rs:68-75`). That holds for everything a reader
  gives: its fonts, `overlay-scrolling` and the icon theme as well
  (`native-theme/src/gnome/mod.rs:525-559`). The gpui connector keeps both
  variants and installs the other one's values when the mode is switched
  without a new read (`CG/lib.rs:1189-1192`, `:1564`).
- One part of the model is outside the variants: `LayoutTheme`, on `Theme`,
  "because spacing is variant-independent"
  (`native-theme/src/model/widgets/mod.rs:1080-1096`); it reaches a connector as
  an argument of its own (`connectors/native-theme-gpui/src/geometry.rs:1441-1459`).
- The model's range checks exist for `f32` and `u16`
  (`native-theme/src/resolve/validate_helpers.rs:397`, `:419`) and run on an
  optional field only where it is stated
  (`native-theme-derive/src/gen_ranges.rs:42-109`). A field that must be stated
  and is not is reported as missing, `Error::ResolutionIncomplete`, collected
  before the range checks (`native-theme/src/resolve/validate.rs:165-180`); a
  range violation is reported as a value outside its bounds
  (`native-theme/src/error.rs:12-36`). Checks written by hand beside the derived
  ones exist (`validate_helpers.rs:769`).
- Every enum in the model is a plain choice without data
  (`docs/property-registry.toml:28`, `:333`, `:488-489`).

## 2. Decisions

### B1 — Where the value lives: on `InputTheme`, stated by presets, overridden by readers

**Options.**

- (a) On `InputTheme`, beside `caret_color`.
- (b) On `ThemeDefaults`.
- (c) On `AccessibilityPreferences`, with reduce-motion and the text scale.
- (d) On `Theme`, outside the two variants, as the layout is.

**(a).** The caret belongs to the text input: `InputTheme` is the one struct that
carries a caret, the text area takes its caret from it
(`native-theme/src/model/widgets/mod.rs:160-162`), and each toolkit's receiver
is the input's. (b) would put one widget's behaviour among the values every
widget inherits from. (c) looks natural for a user setting, but
`AccessibilityPreferences` is not stored in presets
(`native-theme/src/lib.rs:407-410`), so a preset could not state its platform's
default, and an application that previews Material or iOS on another desktop
would have no value. A preset states the platform's default and the reader
replaces it with the user's setting, as for fonts, colours and
`scrollbar.overlay_mode`.

(d) fits the setting best in one respect: the blink is one value for the
desktop, not one per mode, so it would have no limit of the kind described
next. Its cost is in the gpui connector: `apply` and `to_theme` are handed a
variant, a `ResolvedTheme`, and what lies outside the variants reaches a
connector as a further argument (§1.3), so both would need a new parameter, a
breaking change of two entry points for one behaviour. (egui's builder already
takes such values, and `apply_system_theme` has the whole theme.)

With (a) the blink shares a limit every reader value has: a reader fills only
the mode the desktop is in, and the other variant is the preset's (§1.3). An
application that re-reads the theme when the desktop changes, as one with the
watcher does, gets the user's values in the new mode. One that switches the
mode by itself, against the desktop, and applies the other variant gets the
preset's blink there, as it gets the preset's fonts: a user who turned
blinking off would see the caret blink in that mode. Under gpui a mode switch
through gpui-component alone is spared this (B5). A copy of the blink alone
into both variants was weighed and not taken, for consistency: the same holds
for the reader's fonts and its scroll bar mode, and a rule for which reader
values belong to both modes needs each value judged (the icon theme and the
accent's shades are meant to differ by mode). The limit is recorded as what it
is, a general one, in the backlog (§5).

### B2 — The fields

**Options.**

- (a) One field: the interval of one phase.
- (b) Two fields, the shown and the hidden duration, and a third for the
  time-out.
- (c) The whole caret of the parked 0.8.0 model: the three of (b), the blink
  style, the width, the secondary caret, read-only and typing behaviour.
- (d) One field holding a value with data: steady, or blinking with a shown
  and a hidden duration and an optional time-out.

**(b).**

- (a) cannot state iOS, whose phases differ, nor "does not blink", nor GNOME's
  time-out, which is on by default and visible within ten seconds.
- (c) adds five properties that no toolkit of the three can take, except the
  width in egui. Each is a true gap and a step of its own; they are recorded
  (§5).
- (d) would need no rule between fields, no zero that means "no blink" and no
  large number that means "never". It is not taken for three reasons. Every
  enum in the model is a plain choice (§1.3): a value with data would be new
  to the registry, the facts rows, which are per key, and the checkers that
  read them. It merges whole, so an overlay could not change one part over
  what the reader read, for example cancel GNOME's time-out and keep the
  user's rate. And a reader that reads the rate and fails on the time-out
  would have to drop the blink or invent "never", where three fields keep the
  preset's time-out.

The three fields, on `InputTheme`, each optional, category `soft_option`, with
no rule of inheritance:

| Field (TOML key) | Meaning | Stated values |
|---|---|---|
| `caret_blink_on` (`caret_blink_on_ms`) | how long the caret is shown in each cycle | 1 or more |
| `caret_blink_off` (`caret_blink_off_ms`) | how long it is hidden; **0 means it does not blink** | 0 or more |
| `caret_blink_timeout` (`caret_blink_timeout_ms`) | how long after the last input the blinking stops, the caret shown | 1 or more |

- **Unset** means the theme states nothing for that field, and a connector
  leaves the toolkit's own. For the time-out that covers two cases the toolkits
  draw alike: a platform whose caret never stops (KDE, iOS, Material) and one
  whose time-out is not established (Windows, macOS); both toolkits that take a
  blink have no time-out of their own. The facts rows say which case a platform
  is. A stated "never" was weighed: it would be a value no toolkit draws
  differently from unset. One case needs it in practice, an overlay that
  cancels a preset's time-out, since an overlay cannot unset a field; it states
  the largest value, 49 days, which the README says.
- **A blink is stated whole or not at all**, and the model checks it. A theme
  states either no duration; or `caret_blink_off = 0`, which means no blink
  whatever `caret_blink_on` is; or both durations. A time-out is allowed only beside one of the last two, and means
  nothing where the caret does not blink. A variant that states one duration
  alone, or a time-out without a blink, fails validation as incomplete: the
  error the model has for a field that must be stated and is not (§1.3),
  naming the key to add (the other duration; for a lone time-out,
  `caret_blink_off_ms`). The alternative was a rule in each connector that
  ignores half a blink; it would hide invalid data behind the toolkit's
  default, in three places, where the project reports invalid data. The
  bundled presets and the readers write either both durations or
  `caret_blink_off = 0`, so the invalid
  cases come from a theme or an overlay written by hand, and the README tells
  an overlay's author to state both durations: an overlay that states one
  passes over a preset that states the other and fails over one that does
  not. So a connector, or an application reading the resolved theme, has three
  cases to tell and no rule of its own.
- The range checks close the two values that would hide or freeze the caret: a
  shown duration of 0 (egui never paints it) and a time-out of 0 (gpui never
  starts blinking). They set no upper bound, for which there is no source. The
  derive gains a range check for `u32`, beside those it has for `f32` and
  `u16`; the rule over the three fields is one check written by hand beside
  the derived ones, since no other struct needs it.
- **`u32` milliseconds**, not the `u16` of `tooltip.show_delay`: GNOME's
  time-out is a number of seconds a user may set above 65, which a `u16` of
  milliseconds cannot hold. The registry's header gains the type. A `u32` does
  not hold every legal GNOME value either (its largest, in milliseconds, would
  need more): the reader multiplies with a check and stores the largest `u32`,
  about 49 days, for anything beyond, which is the same to a user as the value
  set. One type for the three fields keeps them alike; the durations never
  come near its limit.

### B3 — What the presets state

Each value with the citation `F` carries for it.

| Preset | on / off (ms) | time-out (ms) |
|---|---|---|
| kde-breeze | 500 / 500 | unset: never stops |
| adwaita | 600 / 600 | 10000 |
| material-3 | 500 / 500 | unset: never stops |
| windows-11 | 530 / 530 | unset: see below |
| ios-13 | 650 / 350 | unset: never stops |
| macos-tahoe, macos-monterey | unset: not established | unset: not established |
| the colour-scheme presets (catppuccin, nord, ...) | unset | unset |

- The colour-scheme presets state no platform behaviour, as they state no tooltip
  delay. macOS stays unset until its blink is measured.
- `docs/platform-facts.md` gets a row per field in the input's table. That table
  has no iOS column; ios-13 cites its source in the preset, as it does for the
  tooltip (`native-theme/src/presets/ios-13.toml:190-191`).
- The Windows and iOS values are measurements, and 0.7.x keeps its evidence in
  the repository (`docs/evidence/README.md:3-5`). The plan brings the two runs
  named in §1.1 into `docs/evidence/` from the commits that hold them, as that
  folder keeps a run: the cited file with the run's own description and
  record, and for iOS the script and the input its values are derived from,
  with a section for each of the two harnesses in the folder's README. The
  presets cite them there. The iOS commit is on a local branch only (§1.1), so
  it is copied before anything else is done with that branch.
- windows-11's time-out stays unset. What WinUI does is known (§1.1), but it is
  a rule over the blink time the user set and a system time-out the reader
  does not read yet, not one number a preset can state; it is recorded (§5).

### B4 — What the readers read

A reader replaces the preset's values only with what the desktop states; a
missing or malformed setting leaves the preset's.

- **KDE**: `kdeglobals` `[KDE] CursorBlinkRate`, by the rule of §1.1: a positive
  value is bounded to 200..2000 and halved by integer division into both
  durations; 0 or less sets `caret_blink_off` to 0. The bound is Qt's own, so
  the model holds what a Qt application does with the same file. The KDE
  watcher already watches `kdeglobals`; the reader's list of sources gains the
  key.
- **GNOME and Budgie**, which share the reader
  (`native-theme/src/pipeline.rs:1021`): `cursor-blink-time` halved into both
  durations; `cursor-blink-timeout` in milliseconds, as B2 says; `cursor-blink`
  false sets `caret_blink_off` to 0, whatever the blink time is. They are read as the reader reads its
  other keys of that schema, three more calls of the `gsettings` program within
  the reader's one deadline (`native-theme/src/gnome/mod.rs:373-394`); the
  portal, which the reader uses only for the four cross-desktop settings, was
  weighed and would be a second way to read one schema. The watcher's list gets
  three rows (`native-theme/src/watch/gnome.rs:16-30`), with the entries its
  tests require (`native-theme/src/watch/mod.rs:717-791`). The switch's schema
  lines and what GTK does with a very large time-out are read from the pinned
  sources when the specification is written.
- **Windows**, into both durations, by one of two calls:
  - `UISettings::CaretBlinkRate`, on the object the reader already holds: a
    safe call, so nothing is added to the file's `unsafe`. Whether it is the
    same setting as the caret blink time, in milliseconds, and what it returns
    when blinking is off, only Microsoft's documentation can say; it is read
    when the specification is written.
  - `GetCaretBlinkTime`: the call the measurement used and WinUI's text box
    makes; it is `unsafe`, in `windows.rs`,
    where the reader's other system calls are, each under its own `allow` with
    a `SAFETY` comment (`native-theme/src/windows.rs:189`, `:237`, `:280`,
    `:447`, `:491`).
  **The safe call where the documentation shows it is the same setting; else
  `GetCaretBlinkTime`, which needs the maintainer's approval for the new
  `unsafe` call** (an approval for reads of this kind in that file was given
  for the stopped 0.8.0 plan, `docs/todo_v0.7.1_model-completeness-plan.md:220-222`;
  it is asked again here). Read on Microsoft Learn on 2026-11-14: the page of
  `UISettings.CaretBlinkRate` says only "Gets the blink rate of a new caret
  created by the app view" and "The blink rate of the new caret, in
  milliseconds"; it does not say that it is the setting `GetCaretBlinkTime`
  returns, nor what it gives where blinking is off. The page of
  `GetCaretBlinkTime` says "Retrieves the time required to invert the caret's
  pixels. The user can set this value", the blink time in milliseconds on
  success, "A return value of INFINITE indicates that the caret does not
  blink", and zero on failure. So the rule selects `GetCaretBlinkTime`, and
  the `unsafe` call is the maintainer's to approve (§3 item 4); without the
  approval Windows stays unread in this release and the preset's 530 / 530
  stands. For either call two values are not a blink time and
  are mapped, not stored: the value that reports a failure (WinUI treats a 0
  from `GetCaretBlinkTime` so, `F`'s note) leaves the preset's values, as the
  reader's other failed reads do; and the value that means the user turned
  blinking off sets `caret_blink_off` to 0. Which values those are for each
  call is read from Microsoft's documentation with the rest; a reader that
  stored them as durations would fail validation on the first and blink once
  in 49 days on the second. The Windows watcher gets no notice of a change
  to this setting (`native-theme/src/windows.rs:31-38`), so a change shows at
  the next read; the reader's documentation says so.
- **macOS**: nothing is read (§1.1).
- A reader writes the variant of the mode the desktop is in, as for every value
  it reads (B1).

The alternative to reading at all, presets only, would keep the complaint of §0
for every user who changed the setting, which is the visible case.

### B5 — What the connectors do

**gpui.** Where the theme states a blink (B2), `to_theme` sets
`Theme::cursor_blink`: `steady()` where `caret_blink_off` is 0; otherwise
`blinking`, with the time-out where one is stated. Where it states none it
leaves upstream's default.

gpui-component blinks in equal phases only. Where the two durations differ:

- (a) the shown duration for both phases;
- (b) half of their sum for both phases;
- (c) nothing: leave upstream's default.

**(b).** It keeps the platform's blink rate and loses the share of time the
caret is shown (the sum is taken in a wider type, so two large durations cannot
overflow); (a) changes the rate; (c) loses both. For the one bundled preset
with unequal phases, ios-13, (b) gives 500 ms, which is also upstream's default,
so the choice shows only for an overlay; the crate documentation's table of
what the connector maps (`CG/lib.rs:101` on) and the README state the rule.

`apply` today keeps the application's `cursor_blink` because the theme had
none. With the fields:

- (a) `apply` treats the blink as it treats the tooltip's delay (§1.3): it
  installs the theme's value where the theme states one and remembers that it
  wrote it; where the theme states none, it puts back the application's own
  value, which is whatever was installed that the connector did not write;
- (b) `apply` keeps the application's value always, and an application that
  wants the platform's calls an accessor.

**(a).** The blink is theme data like every other field, and (b) would make the
application ask for one field of the theme. With (a) a theme that states no
blink does not inherit the previous theme's: after adwaita, a colour-scheme
preset gets the application's blink back, not adwaita's time-out. An
application that wants its own blink over a stating theme sets it after `apply`,
as the README's section on overriding theme values describes for every field.

The connector tells its own write from the application's by the value, as the
tooltip's delay does: an installed blink equal to the one it last wrote is its
own. So an application that sets exactly the blink the connector last wrote is
taken for the connector's, and a later theme without a blink puts back the
application's earlier value; the documentation says so.

The connector's observer, which re-installs the stored variant's values after
every change of gpui-component's theme, does not write the blink. For the
tooltip's delay it does; the blink differs in two ways. It is a field of the
very theme the observer watches (gpui-base 0.9.1 `src/theme.rs:30`, filled at
`GC/theme/mod.rs:463`; `CG/lib.rs:1555-1564`),
so an observer write would undo an application's own blink the moment the
application set it through `Theme::update`. And gpui-component keeps
`cursor_blink` across a mode switch by design (`GC/theme/mod.rs:1047-1067`).
So the functions that apply a theme set the blink (`apply` and
`apply_system_theme`), and nothing changes it between them: a switch between
light and dark through gpui-component alone keeps the blink of the variant
that was applied, which for a reader's values is the user's own.
`apply_accessibility`, which applies the stored variant of the current mode
again for a change of the accessibility settings, leaves the blink as it is:
after such a mode switch that variant is the preset's, and an accessibility
signal is no reason to bring the preset's blink back.
The cost: a theme written by hand with a different blink in each mode shows
the applied mode's until `apply` is called again; no platform has such a
blink. The test that
asserts `apply` keeps the field changes to cover four cases: a theme that
states a blink; one that does not after one that did; the application's own
value kept and put back; and the equal-value case above. The sentences of §1.3
are rewritten.

**egui.** Where the theme states a blink, the base style sets `text_cursor.blink`
(false where `caret_blink_off` is 0) and each of `on_duration` and
`off_duration` whose duration is stated; otherwise it leaves egui's. egui has no time-out: `caret_blink_timeout` is recorded as
having no receiver, in `mapping.md` and `mapping.toml`.

**iced.** No receiver: the three fields are added to the contract's
`UNREACHABLE` table, with the lines of the private constant.

**Showcases.** Each states, in the text input's description, what its toolkit
does with the three fields, by the rules its other statements follow.

### B6 — Compatibility, version and release

- `InputTheme` gains three public fields and is not `#[non_exhaustive]`: code
  outside the crate that builds one with every field named, without
  `..Default::default()`, stops compiling. That is a breaking change by Cargo's
  rules, the release check's compatibility step reports it and asks for a
  version decision (`scripts/check_release.sh:1129-1141`), and 0.7.0, a minor
  release, listed the same kind of change under *Breaking Changes* (`CHANGELOG.md:34`, `:66-68`).
  `ResolvedInputTheme` is `#[non_exhaustive]`; nothing breaks there.
- **The version decision is the maintainer's**, and it is the same for every
  0.7.x step that adds a model field (the KDE document's steps too). The
  options:
  - (a) release it as 0.7.x and name the break in the CHANGELOG. The roadmap
    plans fields in 0.7.x (`ROADMAP.md:147-150`), and the project gives no
    compatibility promise before 1.0; but a patch release then breaks the few
    callers that name every field, and they get it by `cargo update`.
  - (b) a minor version for each release that adds a field.
  - (c) one release that makes the model's input structs `#[non_exhaustive]`,
    after which a field is never a break again. It is the larger break once:
    a struct literal, even with `..Default::default()`, no longer compiles
    outside the crate, in the workspace's other crates too.
  **(a) is proposed**, as the roadmap has it, with the break named each time.
  The four documents of 2026-11-14 queue seven releases that add fields (this
  one and the KDE document's steps 2 to 7, 18 fields in all), so (a) means
  seven such notes. (c) would end them, but its one break is the wider: it
  stops the usual way of building a theme in code, a struct literal with
  `..Default::default()`, for every user and in every place the workspace
  itself builds a model struct (not counted here), where (a) breaks only code
  that names every field.
- An application sees a change without asking for it: under gpui and egui the
  caret takes the platform's blink where it took the toolkit's. That is the
  purpose.
- **Release: proposed as a step of its own after v0.7.1**, not part of it. It
  adds model fields, three readers and a change to what `apply` keeps; v0.7.1 is
  the upgrade to GPUI Kit 0.9.1 and fixes. The CHANGELOG sentence of v0.7.1
  about `cursor_blink` stays true until this lands.

## 3. What the maintainer approves

1. B1: the value lives on `InputTheme`, with the limit every reader value has:
   in an application that switches between light and dark by itself and
   applies the other variant, a user who turned blinking off sees the caret
   blink in the mode the desktop is not in.
2. B2: three optional fields in milliseconds, `caret_blink_off = 0` meaning no
   blink, unset meaning nothing stated, and a blink stated whole or rejected.
3. B3: the presets' values.
4. B4: the readers for KDE, GNOME with Budgie, and Windows; for Windows the
   one `unsafe` `GetCaretBlinkTime` call in `windows.rs`, since Microsoft's
   page for the safe call does not show it to be the same setting (read
   2026-11-14), or else Windows left unread in this release; macOS unread and
   unset.
5. B5: `apply` installs the theme's blink as it does the tooltip's delay; the
   connector's observer and `apply_accessibility` leave it alone; under gpui
   unequal phases become two equal ones of the same rate.
6. B6: the version decision for a release that adds model fields, and the
   release this lands in. The order of the releases after v0.7.1 is proposed
   in one place for all four documents, the KDE document's D10.

## 4. What was checked, and what was not

Checked on 2026-11-14 by reading: the lines named in §1.2 and §1.3, and the facts
files' entries of §1.1 (the entries, not the upstream sources they quote). The
first review re-read them.

Not checked:

- The upstream sources behind `F`'s entries. The two measurement files were
  read in the commits named in §1.1; the evidence repository's own copies were
  not.
- Whether `UISettings.CaretBlinkRate` is in fact the system's caret blink
  time: its page does not say (B4), and it was not measured.
- The schema lines of GNOME's `cursor-blink` switch, from when GTK counts its
  time-out, and what GTK does with a very large one.
- How KConfig hands on a malformed `CursorBlinkRate`.
- What `gsettings get` prints for the two integer keys; the reader's parse is
  written against real output when the plan runs.
- Whether the settings portal forwards changes of the three GNOME keys; the
  watcher's source says it forwards every key of the schema
  (`native-theme/src/watch/gnome.rs:7-15`), and that was not re-read.
- The compatibility step of the release check was not run on the change.

## 5. Out of scope, recorded in the backlog by this work

- The caret's width (1 px on KDE and Windows, 2 px on macOS, iOS and Material,
  proportional on GNOME, in `F`): only egui can take it.
- The blink's style (at once or fading), the caret in a read-only field, the
  secondary caret, and that the caret stays solid while typing: no toolkit of
  the three takes them.
- What a reader reads for the desktop as a whole (fonts, the scroll bars'
  overlay mode, the icon theme, the blink) reaches only the variant of the mode
  the desktop is in; the other is the preset's until the next read (B1). A
  rule for which reader values go into both variants is a design of its own.
- macOS's blink: to measure, then to read.
- Windows's time-out: WinUI's rule (§1.1) and the system's own time-out
  (`SPI_GETCARETTIMEOUT`, which the parked specification lists to verify) are
  to be read before a value can be given.
