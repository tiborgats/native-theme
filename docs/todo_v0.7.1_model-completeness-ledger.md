# v0.7.1 → 0.8.0 — Model completeness: Ledger

The working record of [the plan](todo_v0.7.1_model-completeness-plan.md) ("the plan"). One writer: the orchestrating
session; lanes, verifiers, tie-breakers, implementers and reviewers report to it. `scripts/check_ledger.py` checks
the triage (every item has a destination) and, from M4, `--final` (the plan's M4 gate).

## Milestones

| Milestone | Status | Closed at | Notes |
|---|---|---|---|
| D5 `v0.7.1-rc1` merged to `main` | done | fc6109d1 | PR #6, CI 39/39; `main` fast-forwarded on the maintainer's request (2026-10-29) |
| M0 setup | in progress | | `v0.8.0-rc1` and `v0.8.0-tools` cut from fc6109d1; draft PRs #7 and #8 |
| Spikes M · G · K · I · W · A | not started | | |
| M1a slice core | not started | | |
| M1b slice complete | not started | | |
| D6 checkpoint | not started | | |
| M2 model core | not started | | |
| M3 units | not started | | per unit below |
| M4 facts gate complete | not started | | |
| M5 showcases | not started | | |
| M6 release 0.8.0 | not started | | |

## Release set (D1)

Researched 2026-10-29 (the research report's proofs go into `docs/facts/sources.toml`). Decided by the plan's rule,
the newest stable release set at M0, coherent:

- **KDE:** Plasma 6.8.2, KDE Frameworks 6.31.0, Qt 6.12.0, PySide6 6.12.0, in the Arch Linux Archive snapshot that
  carries them, 2026/10/29 (`archlinux@2026-10-29` in `sources.toml`), with kirigami-addons 1.15.0: 1.16.0 came out
  the same day and no snapshot carried it when the release set was pinned (the 2026/10/30 snapshot was still not
  published on 2026-10-30), so it is outside the frozen set.
- **GNOME:** libadwaita 1.10.0, GTK 4.24.1, gsettings-desktop-schemas 51.0; the same snapshot as KDE carries them.
- **Material:** material-web v2.5.0; Compose Material 3 1.5.0-beta02 (androidx `e4d933c9`, released 2026-10-21;
  re-pinned on 2026-10-30 from 1.4.0): R1.6 makes Compose the reference "experimental or not (for some Expressive
  behaviour it is Google's only implementation)", and the Expressive components the specification cites
  (FloatingToolbar and the others) exist only in the 1.5.0 pre-releases; a published pre-release is a release a
  harness can run, which is what D1's rule against `master` commits guards; material-color-utilities by commit (it has
  no tags); the emulator image: Android 17's stable API 37.1 exists only as a 16 KB page-size image, so spike A
  decides between it and API 37.0 (rev 8, 4 KB pages).
- **Windows:** Windows App SDK 2.5.1 with microsoft-ui-xaml `winui3/release/2.5.1`, the newest pair with a WinUI tag
  (2.6.4 is newer but microsoft-ui-xaml has no tag for it), so the WinUI host builds against 2.5.1 too; the Windows
  Community Toolkit 8.2.251219; Microsoft Learn pages at MicrosoftDocs/windows-dev-docs' commit.
- **Apple (D12):** `macos-26` (macOS 26.6.2, Xcode 26.6, the iOS 26.5 simulator) and `macos-15` (macOS 15.7.9, Xcode
  16.4, the iOS 18.6 simulator); the images move within a release, so each evidence file records its exact build.

## Units

Per unit and platform: facts (`—` none, `L` lane writing, `F` written), verification (`V` agreed, `T` at the
tie-breaker, `A` audited), presets (`P`), gate (`G`), connectors (`C`).

| Unit | adwaita | kde-breeze | macos-monterey | macos-tahoe | ios-13 | ios-26 | windows-11 | material-3 | Connectors |
|---|---|---|---|---|---|---|---|---|---|
| slice (`button`, colour roles, slice tokens) | — | — | — | — | — | — | — | — | — |

## Module inventory

Every module, integration test and `lib.rs` item of `native-theme`, one fate each (the plan, *The module
inventory*): ported beside, kept in place, split, deleted, unlinked; with its milestone. Written before the switch.

| Item | Fate | Milestone | Notes |
|---|---|---|---|

## `PLAN_EXCLUDED`

The expected list before the switch is the plan's (*What cannot build meanwhile*). Current list: empty.

| Step | Until | Since | Reason |
|---|---|---|---|

## Full runs

Each milestone's full `scripts/check_release.sh` run: commit, result, every warning read.

| Commit | Milestone | Result | Warnings |
|---|---|---|---|

## Spec-change queue

Class 1 data corrections, class 2 model changes (an S22 line and one independent check of the batch), class 3
rationale changes (the maintainer). Each with its reporter, status and the commit that applied it.

| # | Class | Document and place | Change | Reported by | Status |
|---|---|---|---|---|---|
| 1 | 2 | spec S2.2, S6.1, S13, S14, S17, S18, S21.3; ADD-A113–A118 | the plan's amendments: fact citations and `sources.toml`, vector tables, generated declarations, palette facts, `pending` contract entries | the plan (M0) | applied 1a55a93d; checked 2026-10-29 (7 serious, 16 minor, all fixed in #6), before any work used it |
| 2 | 3 | rationale R1.5, logged under R25.13 | a preset describes the newest release a supported runner measures | the maintainer (D12) | applied dcaefce1 |
| 3 | 2 | catalogue, traces (macOS, iOS), spec S20; ADD-A119; `docs/todo.md` | what only macOS or iOS 27 draws carried to the macOS 27 entry; `newest (R1.5)` covers it | the maintainer (D12) | applied 893df7de; checked with #1 |
| 4 | 1 | catalogue :8193-8347 | five citations of the session's scratchpad paths named by project and commit | the citation inventory (M0) | applied 893df7de |
| 5 | 1 | spec §0 | the name and entry checkers are in the repository | the orchestrator | applied 1a55a93d |
| 6 | 2 | spec S2.2, S5.1, S6.1, S14, S17, S18, S21.3; catalogue presets line; iOS gap map; `docs/todo.md`; R1.5 | the first check's fixes: S17's fields per kind, the quote check's direction and precision, composable conversions, absence citations, `read` facts, evidence paths and what counts as cited, sources keyed `<project>@<ref>` and D12's rule for Apple pages, accent facts by `scheme_file`, the gate's staging, palette facts and `[palette.sources]`; the model's arithmetic named; S9.4's curves vector-checked | the independent check of #1 and #3 | applied abcc9518, except R1.5's rewording, which was the maintainer's (class 3) and is reverted to D12's words; second check: 6 serious, 15 minor |
| 7 | 2 | spec S2.2, S11, S14, S17, S18, S22; `docs/todo.md` | the second check's fixes: the facts' keys and values, the quote's notations (SCSS, CSS, XAML `#aarrggbb`, INI byte lists, `Qt.rgba`), the quotes of one generation's citations checked together, absence citing the whole definition, evidence's side of `conversion` and its preset by the run README, snapshots' `describes` instead of "cited beside", the state models' format, palette facts without rows or generations and `[palette.sources]` keyed by TOML path in S11, examples without invented values | the second independent check | applied 4925164a; third check: 4 serious, 16 minor |
| 8 | 1 | catalogue :8328 | `kcolorscheme@8bb948c src/color-schemes/BreezeLight.colors` against the KDE gap map's `breeze@75561b7f colors/BreezeLight.colors`: two copies of one file; the re-pin settles which one Plasma 6.8.2 installs | the first independent check | open, for the re-pin |
| 9 | 2 | spec S2.2, S11, S14, S17; `sources.toml`; `docs/todo.md` | the third check's fixes: the quote check per generation with a published or read citation (palette facts over all), unitless literals take the value's dimension, XAML `TimeSpan` and `#argb`, C++ and `QColor` byte colours, `seconds_to_ms`; the state model's combinations defined and S10's declaration keys; S17's sources as `sources.toml` defines them (`git`, `sdk`, `distribution`, sites, page snapshots with `describes`, JSON pointers); vector tables' TSV form; nested INI groups; `run.toml` maps images to presets; colour tolerance per channel; the platform directories' presets; palettes' colours in bases only | the third independent check | applied 17e9b9ef; fourth check: 5 serious, 14 minor |
| 10 | 2 | spec S2.2, S17; `sources.toml` | the fourth check's fixes: the closed lists (notations, units by dimension, the number form) held, documented and self-tested by `check_fact_citations.py`, a member added as a class 1 change; values spelled as S11's Spelling table; `sources.toml` gains the `snapshot` kind, palette entries and `[presets]` (preset to release), `check_sources.py` with them; builds per image in `run.toml`; combinations with one value per condition kind, container sizes as conditions, densities, and each fact's own `when`; tolerance's unit; vector tables' header lines | the fourth independent check | applied d7306bf7; fifth check: 4 serious, 11 minor |
| 11 | 2 | spec S2.2, S17; `sources.toml` | the fifth check's fixes: an enum block stated whole by S11's spelling; a tolerance table for values of mixed units; `describes` a list of releases, snapshots keyed by id; `run.toml` names the measured entry and records builds without matching them; the declaration keys listed, `requires_variants` and `sizes_by_*` applied, presets checked per flavour; `sp`, `rem`, `em`; vector tables' `# outputs:` | the fifth independent check | applied e5d76e53; sixth check: 1 serious, 9 minor |
| 12 | 2 | spec S17; `sources.toml` | the sixth check's fixes: a composite measured value's evidence is a table shaped like it, a tolerance per leaf; blocks per the registry; `sdk` entries for images; `describes` may name releases no preset describes; `toolkit` only where one drew; declarations left out are none; flavours not an axis; mixed citation kinds | the sixth independent check | applied; seventh check pending |

## Unmeasured register

Values that could not be measured: the key and combination, the attempts and their evidence, the options; each is
the maintainer's decision.

| Key and combination | Attempts and evidence | Options | Decision |
|---|---|---|---|

## Verification rates

Per lane and unit: the verifier's agreement rate and the audit sample's rate of agreed but wrong facts.

| Unit | Platform | Facts | Agreed | Tie-broken | Audit sample | Agreed but wrong |
|---|---|---|---|---|---|---|

## Spikes and drift runs

| Spike or run | Result | Run URL | Notes |
|---|---|---|---|
| M — macOS probe's button pass | **yes** (2026-10-29): every state, size (extraLarge on 26) and appearance in a key window on macOS 15.7.9 (Xcode 16.4) and 26.6.2 (Xcode 26.6); `highlight:YES` equals a real press pixel for pixel; AppKit draws no hover look for push buttons (record hover as not drawn); runner screen 1× only, no GPU reported; not yet: the accent and contrast passes, focus, mixed, other bezels | [40969040283](https://github.com/tiborgats/native-theme/actions/runs/40969040283) | `tools/os-probe/src/macos/buttons.rs`, `--button-pass`, merged into `v0.8.0-tools` |
| G — GNOME harness and libadwaita vector generator | | | |
| K — KDE container and accent script | | | |
| I — iOS simulator host | | | |
| W — microsoft-ui-xaml coverage and WinUI host | | | |
| A — Android host | | | |

`scripts/check_ledger.py --final` (M4) needs two things the plan leaves open, to be settled before M4: the path of
S19.2's staged inheritance file (named when M1a writes `convert_inheritance_rules.py`; passed as `--staged-file`),
and a per-fact table of drift comparisons in this ledger, which the script then reads (until then `--final` reports
the drift rule as a finding).

## Evidence size

| Date | `docs/evidence/` size | Change | Notes |
|---|---|---|---|

## Tokens

The sum of the agents' reported usage per milestone, against D7's cap (+15,000k for M0–M1b).

| Milestone | Tokens | Cap | Notes |
|---|---|---|---|

## Triage

Every open item of `docs/todo.md`, the catalogue's *Open items*, ROADMAP's *Later — the connectors* and *Also in the
0.7.x releases*, and the plans `docs/todo_gpui-full-theme.md` and `docs/todo_iced-full-theme-geometry.md`, with its
destination: a named milestone, unit or lane; `settled by the model`; `after 0.8.0` with its reason; or `not this
plan's work`.

Drafted by an agent on 2026-10-29 from the rules below and ruled by the orchestrator (the draft's 68 doubtful rows):
a showcase item that touches theme data, Widget Info, the element lists, the parity gates or the screenshots goes to
M5, other showcase behaviour after 0.8.0; a connector convenience helper for a value the toolkit takes only through a
builder is after 0.8.0, the contract table listing the value (S21.3); an item with parts for several destinations
keeps one row naming its main destination and, in its reason, the others; a reader item goes to M1b where the
slice's `ReaderState` needs it, else to the readers unit; items that appeared done were verified and closed in M0 (three, 2026-10-29: the CI legs, the egui widgets crate, the Widget Info audit; their rows removed).

**Identity** (what `scripts/check_ledger.py` matches): Source; Heading as written, hashes included; Item, the
opening words between `“` and `”` as a prefix of the item's words (`docs/todo.md`: an open `- [ ]` item under its
nearest `###` heading since the last `##`, the tag stripped; the catalogue: a `- ` bullet of *Open items* by its
first backticked name; ROADMAP: each `###` subsection and its `- ` bullets under *Later — the connectors*, and *Also
in the 0.7.x releases* as one item; the two connector plans: a fixed list the script holds). ` · l. N` is informational.

| Source | Heading | Item | Destination | Reason |
|---|---|---|---|---|
| docs/todo.md | ### Font rendering preferences — read them from the platform | “Research and document the” · l. 30 | M3 unit readers | S4's `token.text.rendering` (TextRendering: antialiasing, hinting, subpixel order) carries it and the catalogue's readers section lists the reads (GN-3.56, K3.45, W3-22). |
| docs/todo.md | ### Checkbox: a checked checkbox's hover appearance | “Research a checked checkbox's” · l. 53 | M3 unit buttons and toggles | The checkbox's {checked, hover} combination is a state-model rule (S7.2, catalogue `checkbox`), whose facts the unit gathers on every preset. |
| docs/todo.md | ### Tab: WinUI 3's tab separators and its tab bar's bottom rule | “WinUI 3's `TabView` draws” · l. 114 | M3 unit containers | The catalogue's `tab` entry carries the separator element (margin, opacity, hidden by position rules) and the bar's `stroke`; values from the windows facts. |
| docs/todo.md | ### Tab: the border is the selected tab's, and KDE's live tab colours | “`kde-breeze` now states `tab.border.color`,” · l. 145 | M3 unit containers | The catalogue's `tab` parts state the selected tab's stroke by rule and KDE's tab colours derive from the scheme by the platform functions (S2.2 `kcu_mix`, Qt's colour helpers). |
| docs/todo.md | ### Segmented control: the join and the divider | “Research in `docs/platform-facts.md` **first**,” · l. 173 | M3 unit buttons and toggles | The catalogue's `segmented_control` entry carries the join (outer corners by position) and the divider; values from each platform's facts. |
| docs/todo.md | ### Checkbox: the check mark's glyph and size | “Research each platform's check” · l. 187 | M3 unit buttons and toggles | The catalogue's `checkbox.mark` part carries the glyph and its stroke (S2.9 `Glyph`, `mark.glyph.stroke.width`); values from the facts. |
| docs/todo.md | ### Slider: the thumb's outline or shadow | “`SliderTheme` states the thumb's” · l. 214 | M3 unit fields | The catalogue's `slider` thumb part carries a stroke and shadow (S2.4, S2.5); macOS's measured from the probe. |
| docs/todo.md | ### Radio: Material's radio is not the checkbox's shape | “Decision for the maintainer” · l. 237 | M3 unit buttons and toggles | The catalogue gives `radio` an entry of its own (Material's ring and dot), answering the maintainer's question; the values come with the unit. |
| docs/todo.md | ### Theme watcher: OS changes it does not report | “**Windows watcher: the window-only” · l. 259 | M3 unit readers | The watcher must report what the readers read (S16: `on_theme_change` then `reload_reader`); the window-only sources (non-client fonts, high contrast) are among the Windows reads. |
| docs/todo.md | ### Theme watcher: OS changes it does not report | “**macOS watcher: check on” · l. 300 | M1b | Checked on CI's macOS legs when M1b links the watchers again. |
| docs/todo.md | ### Theme watcher: OS changes it does not report | “**iced on macOS: the” · l. 309 | after 0.8.0 | The iced connector's watch subscription thread is connector behaviour outside the mapping, a later connector milestone (R21). |
| docs/todo.md | ### platform-facts: macOS vertical padding is measured outside the border | “`docs/platform-facts.md` derives macOS `border.padding_vertical`” · l. 329 | lane macos | What the probe's 22 and 16 measure and the border's share of the padding are measured macOS facts for the button, field, menu, tab and list. |
| docs/todo.md | ### Presets: a spinner stroke width platform-facts does not give | “`docs/platform-facts.md` §2.23 gives the” · l. 363 | M3 unit indicators | The catalogue's `spinner` parts and R20's absent value kind let a preset state no stroke where the platform rotates an icon; values from the facts. |
| docs/todo.md | ### Presets: the scrollbar thumb colour platform-facts measures | “`docs/platform-facts.md` §2.8 measures `scrollbar.thumb_color`” · l. 391 | M3 unit indicators | The catalogue's `scrollbar` thumb per state from each platform's facts; today's inheritance fallback goes (R20). |
| docs/todo.md | ### Fonts: find a system face by family, weight and style | “A name-only lookup beside” · l. 456 | after 0.8.0 | The `fonts` helpers stay outside the model (S16); the entry itself calls `system_face_family` a spec change for a later version. |
| docs/todo.md | ### Icons: recolour a Breeze `current-color-scheme` stylesheet from the theme, as KIconLoader does | “A Breeze SVG icon” · l. 463 | M2.5 | `ResolvedTheme::icon` takes the part's `IconStyle` with its `fill` and `status_colors` (S16, S2.9, K3.61 ADD-A37), so the loader recolours the stylesheet classes from them. |
| docs/todo.md | ### platform-facts: which part of a control the focus ring surrounds | “`docs/platform-facts.md` states each platform's” · l. 504 | M3 unit buttons and toggles | Which part the focus indicator surrounds is stated per widget by the part carrying S2.10's focus indicator (checkbox, radio; the slider is the fields unit's). |
| docs/todo.md | ### platform-facts: which part of a control the focus ring surrounds | “Open question: does a” · l. 513 | lane windows | Whether WinUI's TextBox shows the system focus visual is a windows fact; S2.10 lets the input's parts state none. |
| docs/todo.md | ### Selection: the text colour macOS pairs with the unemphasised selection | “`defaults.selection_inactive_background` is the selection” · l. 530 | M1a | S3's `selection_text_foreground_unfocused` (MAC-I05) is a colour role of the slice; the macOS lane states it. |
| docs/todo.md | ### Presets: colours with no reproducible source | “These preset colours are” · l. 563 | settled by the model | Every preset value enters from a verified fact (S17, S19.1), so the unsourced colours go and the lanes state each from its source or derivation. |
| docs/todo.md | ### Disabled: presets that state both mechanisms | “The disabled rule (`docs/platform-facts.md`” · l. 656 | settled by the model | Disabled looks become stated `{disabled}` rules (R20, the catalogue's `opacity @ {disabled}`), and the community presets become palettes stating no disabled mechanism (S14, S19.1). |
| docs/todo.md | ### KDE: the live reader's disabled text colour | “`native-theme/src/kde/colors.rs` reads `defaults.disabled_text_color`” · l. 681 | M1b | KColorScheme's effects are the platform function `kde_effects` (S2.2, R5.1 disabled roles) and the KDE reader is ported to S9.3 at M1b. |
| docs/todo.md | ### The tab gap, the check mark's line, the expander's structure, the text area (2026-09-28) | “**gpui Textarea padding below” · l. 706 | not this plan's work | An upstream gpui-component seam (Textarea padding); the connector contract lists what the toolkit cannot draw (S21.3). |
| docs/todo.md | ### The tab gap, the check mark's line, the expander's structure, the text area (2026-09-28) | “**macOS text area padding.**” · l. 719 | lane macos | NSTextView's insets are a macOS fact for `text_area` (published or measured). |
| docs/todo.md | ### The tab gap, the check mark's line, the expander's structure, the text area (2026-09-28) | “**KDE expander header.** `KCollapsibleGroupBox`” · l. 722 | lane kde | KCollapsibleGroupBox's header size and arrow box are KDE facts for the catalogue's `expander`. |
| docs/todo.md | ### The tab gap, the check mark's line, the expander's structure, the text area (2026-09-28) | “**The expander's arrow glyph.**” · l. 728 | M3 unit containers | The catalogue's `expander` arrow part carries its glyph (S2.9 `Glyph`), stated per preset. |
| docs/todo.md | ### The tab gap, the check mark's line, the expander's structure, the text area (2026-09-28) | “**iced Basic tabs under” · l. 731 | M5 | A showcase layout defect (iced_aw tab bar clipping under `tab.min_width`), revisited when the showcases are rebuilt on the query. |
| docs/todo.md | ### The tab gap, the check mark's line, the expander's structure, the text area (2026-09-28) | “**egui Inputs page.** `pages/inputs.rs`” · l. 733 | M5 | A showcase defect (the egui Inputs page reads the outer scope's visuals), fixed when the showcases are rebuilt. |
| docs/todo.md | ### The tab gap, the check mark's line, the expander's structure, the text area (2026-09-28) | “**Card as a group” · l. 736 | M3 unit typed widgets batch | The catalogue's `group_box` is a typed widget of its own, answering whether a group box is the card. |
| docs/todo.md | ### Progress bar: Breeze's groove, the radii, the shadow | “Found with the progress-bar” · l. 742 | M3 unit indicators | The catalogue's `progress_bar` groove, radii and shadow come from each platform's facts. |
| docs/todo.md | ### macOS 27 Golden Gate and iOS 27: no preset of their own yet | “When GitHub offers a” · l. 765 | after 0.8.0 | D12 (a): `macos-tahoe` describes 26.x; macOS 27 waits for a non-preview runner image. |
| docs/todo.md | ### macOS: the window background macOS 14 and 15 draw is not the one they read | “From the D round's” · l. 784 | spike M | Spike M's probe passes put the wallpaper and transparency under control and compare the drawn with the read window background. |
| docs/todo.md | ### macOS: what the OS probe does not render yet | “The OS probe renders” · l. 811 | lane macos | Hovered slider knob, rows and switch are measured macOS facts, from probe passes like spike M's button pass. |
| docs/todo.md | ### macOS: what the OS probe does not render yet | “A disabled checked checkbox” · l. 822 | lane macos | A disabled checked checkbox and radio is a measured {checked, disabled} combination on macOS (S7.2 state model). |
| docs/todo.md | ### material-3: the display names "Material 3" and "Material 3 Expressive" | “When M2d rewrites the” · l. 840 | M1b | `material-3`'s `PresetId::display_name` (S16) is written when M1b writes the preset; Expressive is a flavour, not a second preset (R1.6). |
| docs/todo.md | ### ios-26: 38 required fields have no iOS 26 source | “Step 17 of the” · l. 851 | M1b | The plan creates `ios-26` at M1b and closes this entry there. |
| docs/todo.md | ### Model: one `*_text_color` field, two meanings | “For M3: `docs/platform-facts.md` §2.1.4's” · l. 912 | settled by the model | S3's status roles separate the fill, the foreground on it and the standalone colour (`StatusRoles`, ADD-A48), so one field no longer carries two meanings. |
| docs/todo.md | ### `ResolvedLayoutTheme`: a public type nothing produces | “For the API review” · l. 932 | settled by the model | S16 removes it: today's layout becomes the `layout` widget's values, resolved by the query. |
| docs/todo.md | ### macOS: the reader and the SF Symbols loader on macOS 10.15 and earlier | “For the platform readers” · l. 946 | M1b | The macOS reader's port at M1b is where `read_macos` gets its macOS 11 guard (no runtime panic). |
| docs/todo.md | ### System icons: `.scale()` after a measurement on macOS and Windows | “From M1-T6-07 (deferred by” · l. 962 | settled by the model | The icon's size and scale are part of the resolved `IconStyle` `ResolvedTheme::icon` takes (S2.9, S16). |
| docs/todo.md | ### System icons: `SystemTheme::icon` at a size | “From M6-13 (M6 T4):” · l. 981 | settled by the model | S16's `ResolvedTheme::icon` takes the part's resolved icon style with its size. |
| docs/todo.md | ### Segoe Fluent icons as SVG outlines | “For M6 (M1-T6-07's Segoe” · l. 995 | after 0.8.0 | Segoe glyph outlines as SVG are an icon-pipeline feature the model does not need. |
| docs/todo.md | ### Validation: a bad `defaults.border` value is reported once per widget | “From M3 T9's review” · l. 1014 | settled by the model | R20 removes inheritance and S12's validation reports each problem at the layer and key that states it. |
| docs/todo.md | ### KDE fonts: Qt 6's `styleName`, field 16 | “Research (M4 T11 ruling,” · l. 1025 | M3 unit readers | The KDE font strings with their extras are a reader read (catalogue readers, K3.46); Qt's `styleName` rule is settled from the pinned qfont.cpp there. |
| docs/todo.md | ### GNOME: the reader's dark mode does not read `GTK_THEME` | “From M4 T15 (ruled” · l. 1035 | settled by the model | S16 removes `detect::system_is_dark`; the reader's `ReaderState` is the one source of the scheme. |
| docs/todo.md | ### The GNOME reader applies libadwaita's high-contrast rules | “From the v0.7.0 entry” · l. 1046 | M1a | `adwaita`'s colour roles under the contrast condition (S7.1, S9.3) are the slice's; the per-widget contrast rules follow in each unit. |
| docs/todo.md | ### windows-11: the title bar with "Show accent colour on title bars" | “From M4 T22 (ruled” · l. 1065 | M3 unit readers | The trace's reader lines W1-001 and W1-013 read `HKCU DWM ColorPrevalence` for the window's title bar. |
| docs/todo.md | ### Theme watcher: the trailing debounce has no maximum wait | “From M4 T20's review” · l. 1080 | after 0.8.0 | The watcher's debounce timing is not a theme value and the model does not change it. |
| docs/todo.md | ### Readerless Linux desktops: text scaling and high contrast are not read | “From M4 T19 (2026-10-13):” · l. 1090 | after 0.8.0 | Reading settings on desktops with no reader is platform support beyond the six platforms' readers. |
| docs/todo.md | ### Linux: button order and `detect_platform` decide by the raw desktop | “From M4 T16 (2026-10-12):” · l. 1101 | M1a | At the switch the button order becomes `dialog.values.action_layout.row_order` and `detect_platform` becomes `PresetId::for_platform`/`for_system` (S16), which should read the refined desktop. |
| docs/todo.md | ### Linux: an application that only resolves presets never gets the portal's answer | “From M4 T16 (2026-10-12;” · l. 1114 | after 0.8.0 | The desktop detection stays as it is (S16), so the sync path's portal question is not the model's. |
| docs/todo.md | ### KDE reader: `switch.thumb_background` and `menu.icon_size` are not read | “From M4 T36 (2026-10-13):” · l. 1128 | M3 unit readers | The icon theme's `*Default` sizes are a KDE reader read (K3.63) into `token.icons.sizes`. |
| docs/todo.md | ### kde-breeze: the Qt Widgets menu fill | “From M4 T36 and” · l. 1142 | lane kde | R1.6's flavours (`widgets`, `kirigami`) let kde-breeze state both menu fills; the KDE lane records each from its source. |
| docs/todo.md | ### kde-breeze: the header's "Verified on" date | “From the kde-breeze validator” · l. 1154 | settled by the model | The presets are rewritten from facts and each fact carries its citation and pin (S17, S19.1), so the header's date goes. |
| docs/todo.md | ### platform-facts: no shadow rows for the toolbar and the list | “From the kde-breeze validator” · l. 1162 | M3 unit containers | The toolbar's and list's shadows come from the facts (S2.5) when the containers unit writes the presets. |
| docs/todo.md | ### Model: the menu bar has no fill of its own | “From the M8 T16” · l. 1173 | M3 unit typed widgets batch | The catalogue's `menu_bar` widget has its own fill. |
| docs/todo.md | ### Presets: expander hover fills with no source | “From M6 T16b (2026-10-16):” · l. 1187 | M3 unit containers | The catalogue's `expander` header hover per platform from the facts; community presets become palettes (S14). |
| docs/todo.md | ### Presets: values the M8 T15 platform-facts rows disagree with | “From M8 T15 (2026-10-17):” · l. 1214 | M4 | The lanes convert the imported platform-facts rows these disagreements come from; M4 empties `docs/facts/imported/`. |
| docs/todo.md | ### KDE reader: the toolbar's Header fill and the window border size | “From M8 T16b (2026-10-17):” · l. 1492 | M3 unit readers | The KDE trace's reader line K1.11 (kwinrc BorderSize, BorderSizeAuto) carries the read; the toolbar fill's condition follows from it. |
| docs/todo.md | ### windows-11: values WinUI 3 states no source for | “From the v0.7.0 entry” · l. 1506 | lane windows | The dark selected-text colour, the menu backdrop's fallback and the high-contrast dictionaries each need a published or measured Windows fact (S17). |
| docs/todo.md | ### Model: a hovered segment's text colour | “From the D11/D12 review” · l. 1525 | M3 unit buttons and toggles | The catalogue's segmented_control states each part's text colour per state, so a hovered segment's label colour is a fact of that unit; the Windows reader's high-contrast write follows in the unit's reader step. |
| docs/todo.md | ### windows-11: the validator's findings after T35 left open | “From the windows-11 `preset-validator`” · l. 1538 | lane windows | The list alternate row, the high-contrast mappings, the link's HC states and the splitter hover are Windows facts; the one-shadow-colour question is settled by S2.5's layered shadows. |
| docs/todo.md | ### Linux font DPI: the `xrandr` fallback has no facts row | “From the kde-breeze validator” · l. 1572 | M1b | The reader's font DPI becomes ReaderState.font_dpi (S9.4, which names only forceFontDPI and Xft.dpi), ported with the readers at M1b, where the xrandr step gets a source or is dropped. |
| docs/todo.md | ### Dependencies: zbus 6 and the three unmaintained-crate advisories | “From M8 Step 8” · l. 1598 | not this plan's work | A dependency chore (zbus 6 after ashpd moves, the audit entries, the outdated check), unrelated to the model. |
| docs/todo.md | ### detect: `prefers_reduced_motion` and the reader's block are two caches of one read | “From M4's final review” · l. 1624 | settled by the model | S16 removes prefers_reduced_motion and accessibility_preferences, replaced by Theme::system()'s ReaderState, so the two caches disappear. |
| docs/todo.md | ### KDE: the icon sizes and the icon lookup search the base dirs in different orders | “From M4's final review” · l. 1635 | after 0.8.0 | The icon lookup and the KDE icon-theme base dirs are kept code the model does not change (S16 keeps the icons loaders), a rare-case consistency fix for a later release. |
| docs/todo.md | ### detect: `refined_linux_desktop_sync` stats `kdeglobals` on every call before a system read | “From M4's final review” · l. 1647 | after 0.8.0 | Desktop detection stays as it is (S16), so this caching fix is a performance chore outside the model work. |
| docs/todo.md | ### Links: Kirigami darkens a pressed link | “From M4 T34 (2026-10-13):” · l. 1658 | M3 unit buttons and toggles | The link's pressed colour under the kirigami flavour (R1.6) is a fact stated with qt_darker (S2.2, ADD-A50) in the link entry. |
| docs/todo.md | ### KDE: the switch's hover and disabled thumb outlines | “From M8 T16c (2026-10-17):” · l. 1673 | M3 unit buttons and toggles | The catalogue's switch parts carry each state's outline as rules, so qqc2's hover and disabled outlines are facts of that unit (kirigami flavour, R1.6). |
| docs/todo.md | ### KDE: Union, the Application Style that is data (issue #4) | “**Read the user's Union” · l. 1693 | not this plan's work | Waits on KDE enabling Union by default and a non-Breeze Union style existing; a future reader feature, not the model. |
| docs/todo.md | ### Dialog: the platform's vertical placement | “**Model where each platform” · l. 1707 | M3 unit placement | S13's placement strategies (CenteredInHost etc., the catalogue's placement table) carry where each platform places an in-window dialog. |
| docs/todo.md | ### Values the v0.7.1 gap map found against their sources | “**KDE.** (1) `menu.hover_background` and” · l. 1732 | lane kde | Each of the nine findings is a KDE fact to settle at the pin; (2), (3) and (7) touch colour roles and the reader's [Colors:Header]/inactive reads (M1b, R18), (8) is measured. |
| docs/todo.md | ### Values the v0.7.1 gap map found against their sources | “**GNOME** (libadwaita 1.10.0, `src/stylesheet/`;” · l. 1755 | lane gnome | Findings (1)–(10) are libadwaita facts; (9) is also settled by the spinner widget (S2.7, S19.3); (11) and (12) are reads of M3 unit readers (GN-3.15). |
| docs/todo.md | ### Values the v0.7.1 gap map found against their sources | “**Material.** Compose Material 3” · l. 1775 | lane material | Compose's text selection colour is a Material fact the lane cites at the pin. |
| docs/todo.md | ### Values the v0.7.1 gap map found against their sources | “**Windows.** (1) The *Layering*” · l. 1778 | lane windows | The card's elevation and stroke and the layout gaps are published Windows facts the lane takes. |
| docs/todo.md | ### Values the v0.7.1 gap map found against their sources | “**Menu items are rounded” · l. 1786 | M3 unit overlays | The menu's item corner radius on four platforms is the menu entry's facts across lanes. |
| docs/todo.md | ### Values the v0.7.1 gap map found against their sources | “**macOS.** (1) `platform-facts.md@fc6109d1:1622` says” · l. 1792 | lane macos | The shadows and the dialog icon are macOS facts to measure (spike M's passes); the reader's shadowColor and sidebar width reads are M3 unit readers. |
| docs/todo.md | ### Values the v0.7.1 gap map found against their sources | “**Windows.** (1) The reader” · l. 1803 | M3 unit readers | The overlay-scrollbars setting becomes the reader-read condition overlay_scrollbars (catalogue readers); the TextBox's missing read-only and invalid states are the fields unit's Windows state model. |
| docs/todo.md | ### Values the v0.7.1 gap map found against their sources | “**GNOME, more "(none)" cells” · l. 1808 | lane gnome | Spinner, segmented separator and radius values libadwaita states are GNOME facts. |
| docs/todo.md | ### Values the v0.7.1 gap map found against their sources | “**Material has a scrollbar.**” · l. 1813 | lane material | Compose's scrollbar and rich-text link styling are Material facts (the indicators and buttons units consume them). |
| docs/todo.md | ### Values the v0.7.1 gap map found against their sources | “**KDE's dialog icon.** `kde-breeze.toml`” · l. 1819 | lane kde | The dialog icon size from KStyle's PM_MessageBoxIconSize is a KDE fact. |
| docs/todo.md | ### Values the v0.7.1 gap map found against their sources | “**ios-13 states values for” · l. 1824 | lane ios | What iOS does not draw is declared absent from the facts and the 61 unsourced values are measured (S17, S10 declarations). |
| docs/todo.md | ### native-theme-egui connector | “Map the platform font-rendering” · l. 1845 | M2.1 | `token.text.rendering` is part of S4.4's `TextTokens`, built in M1a, so the egui connector maps it when it moves to the slice. |
| docs/todo.md | ### native-theme-egui connector | “Possible upstream egui issue:” · l. 1855 | not this plan's work | An upstream egui issue to file; the showcase's workaround stands. |
| docs/todo.md | ### native-theme-egui connector | “**A widget font's own” · l. 1870 | not this plan's work | Waits on an epaint hook; the connector contract records the limit (S21.3, R23). |
| docs/todo.md | ### native-theme-gpui connector | “Key the icon tables” · l. 1885 | after 0.8.0 | A test-hardening refactor of the gpui connector's icon tables, unrelated to the model (R21: connector features later). |
| docs/todo.md | ### native-theme-gpui connector | “**For M6: two defects” · l. 1893 | M2.1 | The button's pressed fill becomes a stated fact (macos-tahoe from spike M) and the gpui connector's derived fill goes with its old mapping at M2.1. |
| docs/todo.md | ### native-theme-gpui connector | “**`unicode-properties` back to 0.1.4” · l. 1923 | not this plan's work | Waits on a gpui-fast-web release dropping its exact pin. |
| docs/todo.md | ### native-theme-gpui connector | “**File the gpui-fast issue:” · l. 1940 | not this plan's work | An upstream gpui-fast issue for the maintainer to file. |
| docs/todo.md | ### native-theme-gpui connector | “**The layout dump on” · l. 1966 | after 0.8.0 | A showcase tooling limitation no gate reads, unrelated to the model. |
| docs/todo.md | ### native-theme-gpui connector | “**The gpui showcase keeps” · l. 1991 | M5 | The gpui showcase's Widget Info registry is rebuilt with the showcase at M5, where the records can move out of the entity. |
| docs/todo.md | ### native-theme-gpui connector | “**File the gpui-kit issue: gpui-pre” · l. 2022 | not this plan's work | An upstream gpui-kit issue for the maintainer to file. |
| docs/todo.md | ### native-theme-gpui connector | “**File the gpui-kit issue: a” · l. 2072 | not this plan's work | An upstream gpui-kit issue for the maintainer to file. |
| docs/todo.md | ### native-theme-gpui connector | “a `ghost_hover` / `ghost_hover_foreground`” · l. 2126 | not this plan's work | Upstream gpui-component token; the connector contract lists what the toolkit cannot draw (S21.3, R23). |
| docs/todo.md | ### native-theme-gpui connector | “a menu-surface token. `PopupMenu`” · l. 2135 | not this plan's work | Upstream gpui-component token; the connector contract lists what the toolkit cannot draw (S21.3, R23). |
| docs/todo.md | ### native-theme-gpui connector | “a foreground token for” · l. 2146 | not this plan's work | Upstream gpui-component token; the connector contract lists what the toolkit cannot draw (S21.3, R23). |
| docs/todo.md | ### native-theme-gpui connector | “foreground tokens for a” · l. 2156 | not this plan's work | Upstream gpui-component tokens; the connector contract lists what the toolkit cannot draw (S21.3, R23). |
| docs/todo.md | ### native-theme-gpui connector | “`Checkbox::label` sets `text_color(foreground)` on” · l. 2165 | not this plan's work | Upstream gpui-component receiver; the connector contract lists what the toolkit cannot draw (S21.3, R23). |
| docs/todo.md | ### native-theme-gpui connector | “`WindowBorder`'s frame colour is” · l. 2181 | not this plan's work | Upstream gpui-component literal; the connector contract lists what the toolkit cannot draw (S21.3, R23). |
| docs/todo.md | ### native-theme-gpui connector | “a placeholder colour for” · l. 2189 | not this plan's work | Upstream gpui-component receiver; the connector contract lists what the toolkit cannot draw (S21.3, R23). |
| docs/todo.md | ### native-theme-gpui connector | “the focus ring's width.” · l. 2204 | not this plan's work | Upstream gpui-component receiver; the connector contract lists what the toolkit cannot draw (S21.3, R23). |
| docs/todo.md | ### native-theme-gpui connector | “gpui-component reads `tokens.tab`, `tokens.list`,” · l. 2212 | not this plan's work | Upstream gpui-component tokens to paint or drop; the contract keeps them WRITE_ONLY (S21.3). |
| docs/todo.md | ### native-theme-gpui connector | “the segmented `TabBar` has” · l. 2219 | not this plan's work | Upstream gpui-component receivers; the connector contract lists what the toolkit cannot draw (S21.3, R23). |
| docs/todo.md | ### native-theme-gpui connector | “`Button::icon` discards an `Icon`'s” · l. 2224 | not this plan's work | Upstream gpui-component defects. |
| docs/todo.md | ### native-theme-gpui connector | “`Checkbox` and `Radio` draw” · l. 2229 | not this plan's work | Upstream gpui-component token; the connector contract lists what the toolkit cannot draw (S21.3, R23). |
| docs/todo.md | ### native-theme-gpui connector | “Whether any platform scales” · l. 2237 | settled by the model | Sizes are rule selectors (S7.3) over S2.4's corners, so a per-size radius is a rule the facts state where a platform has one, and the unit's facts answer the question. |
| docs/todo.md | ### native-theme-gpui connector | “a control's radius should” · l. 2247 | not this plan's work | Upstream gpui-component behaviour; the connector restores the radius per call site. |
| docs/todo.md | ### native-theme-gpui connector | “`Toggle` needs a token” · l. 2258 | not this plan's work | Upstream gpui-component token; the connector contract lists what the toolkit cannot draw (S21.3, R23). |
| docs/todo.md | ### native-theme-gpui connector | “`TabVariant::Outline` hovers with `tokens.secondary_hover`,” · l. 2265 | not this plan's work | Upstream gpui-component receiver; the connector contract lists what the toolkit cannot draw (S21.3, R23). |
| docs/todo.md | ### native-theme-gpui connector | “`InputGroupButton` should hover with” · l. 2276 | not this plan's work | Upstream gpui-component literal; the connector contract lists what the toolkit cannot draw (S21.3, R23). |
| docs/todo.md | ### native-theme-gpui connector | “a `Dialog::max_h` prop, or” · l. 2285 | not this plan's work | Upstream gpui-component receiver (Dialog::max_h). |
| docs/todo.md | ### native-theme-gpui connector | “a styled `Theme` scrollbar-style” · l. 2293 | not this plan's work | Upstream gpui-component override (ROADMAP's upstream receivers). |
| docs/todo.md | ### native-theme-gpui connector | “a scrollbar thumb inset” · l. 2295 | not this plan's work | Upstream gpui-base receiver. |
| docs/todo.md | ### native-theme-gpui connector | “`Tab` keeping the caller's” · l. 2310 | not this plan's work | Upstream gpui-component receiver (ROADMAP's upstream receivers). |
| docs/todo.md | ### native-theme-gpui connector | “`Theme.shadow` honoured beyond `Button`;” · l. 2313 | not this plan's work | Upstream gpui-component readers of Theme.shadow and tokens.shadow (ROADMAP's upstream receivers). |
| docs/todo.md | ### native-theme-gpui connector | “`Size::Size` honoured by `Checkbox`” · l. 2327 | not this plan's work | Upstream gpui-component receiver (ROADMAP's upstream receivers). |
| docs/todo.md | ### native-theme-gpui connector | “a disabled `Button` should” · l. 2328 | not this plan's work | Upstream gpui-component receivers for the disabled colours; the contract lists them (S21.3). |
| docs/todo.md | ### native-theme-gpui connector | “the `Link` widget should” · l. 2348 | not this plan's work | Upstream gpui-component Link reading link_hover and link_active. |
| docs/todo.md | ### native-theme-gpui connector | “a `PopupMenu` item style” · l. 2373 | not this plan's work | Upstream gpui-component style hooks. |
| docs/todo.md | ### native-theme-gpui connector | “button label text size” · l. 2374 | not this plan's work | Upstream gpui-component label text size. |
| docs/todo.md | ### native-theme-gpui connector | “public base-palette fields on” · l. 2378 | not this plan's work | Upstream gpui-component public palette fields. |
| docs/todo.md | ### native-theme-gpui connector | “internal scrollbars reserve no” · l. 2379 | not this plan's work | Upstream gpui-component gutters; the showcase's workaround stands. |
| docs/todo.md | ### native-theme-gpui connector | “a right-clicked `Tree` row” · l. 2412 | not this plan's work | Upstream gpui-component Tree argument. |
| docs/todo.md | ### native-theme-gpui connector | “element hooks on the” · l. 2439 | not this plan's work | Upstream gpui-component element hooks. |
| docs/todo.md | ### native-theme-gpui connector | “The iced `geometry` gap” · l. 2490 | settled by the model | S21.3's contract test fails on any resolved field neither mapped nor listed, which is the mechanical listing of geometry no iced builder receives; the helpers themselves stay a later connector feature (R21). |
| docs/todo.md | ### native-theme-gpui connector | “Unreachable in iced 0.14” · l. 2503 | not this plan's work | Upstream iced/iced_aw limits, recorded in the contract's UNREACHABLE list (S21.3). |
| docs/todo.md | ### native-theme-gpui connector | “iced_aw 0.14.1 defects found” · l. 2519 | not this plan's work | Upstream iced_aw defects to file. |
| docs/todo.md | ### native-theme-gpui connector | “File upstream iced_aw issue:” · l. 2532 | not this plan's work | Upstream iced_aw issue to file. |
| docs/todo.md | ### native-theme-gpui connector | “174 of 512 text-on-background” · l. 2546 | settled by the model | Every preset value comes from a verified fact and the community presets become palettes of their projects' published colours (S17, S19.1, S14), so the suspected data errors (material-3's tab pair, nord's placeholder) are checked by the gate rather than judged by preset-validator; the platform's own low contrasts stay as facts. |
| docs/todo.md | ### native-theme-gpui connector | “kde-breeze states `button.hover_background =” · l. 2561 | lane kde | The unblended `#93cee9` hover fills of checkbox, scrollbar, slider, combo box, list and sidebar are kde-breeze values the KDE facts lane states from Breeze's source (S17, S19.1). |
| docs/todo.md | ### native-theme-gpui connector | “`button.hover_text_color` and `button.active_text_color` equal” · l. 2606 | M1b | Whether each platform changes the button label on hover and press is part of the slice's button state model, stated from facts on all eight presets by M1b. |
| docs/todo.md | ### native-theme-gpui connector | “The progress bar's fill” · l. 2615 | settled by the model | The progress bar's accent-on-track pair is the platform's own value, carried as a fact and checked by the gate (S17, R4); nothing in the model or presets is to change. |
| docs/todo.md | ### native-theme-gpui connector | “No bundled preset states” · l. 2620 | M3 unit buttons and toggles | The catalogue's `segmented_control` has its own `track` part (`background_color` → `track.fill`), stated per preset from facts in that unit. |
| docs/todo.md | ### native-theme-gpui connector | “kde-breeze states `switch.thumb_diameter ==” · l. 2625 | lane kde | Whether Breeze's switch handle is as tall as its groove is a KDE fact the lane reads from Breeze's metrics. |
| docs/todo.md | ### native-theme-gpui connector | “Since v0.5.9 the iced” · l. 2628 | lane palettes | solarized and tokyo-night become palettes mapped from their projects' published colours (S14, S19.1), so their window text colour is checked against those colours by the palette lane. |
| docs/todo.md | ### native-theme-gpui connector | “How a platform draws” · l. 2634 | M3 unit buttons and toggles | The model states values per state combination (S7.2 state flags, rules on {checked, disabled}) and the checkbox's state model comes from the facts, so a disabled checked fill is stated where a platform draws one, in that unit. |
| docs/todo.md | ### native-theme-gpui connector | “Audit the other `docs/inheritance-rules.toml`” · l. 2660 | settled by the model | Model inheritance goes and every reference is a derivation stated in the presets and checked by the gate (R20, S19.2), so there are no inheritance rules left to audit. |
| docs/todo.md | ### native-theme-gpui connector | “windows-11 light gives `button.disabled_background” · l. 2665 | lane windows | The windows-11 disabled fills (`ControlFillColorDisabled`) of button, input, checkbox and combo box are stated by the Windows lane from microsoft-ui-xaml at its pin. |
| docs/todo.md | ### native-theme-gpui connector | “Scrollbar thumb radius per” · l. 2696 | M3 unit indicators | The catalogue's `scrollbar.thumb` carries `corners`, stated per preset from facts in the indicators unit. |
| docs/todo.md | ### native-theme-gpui connector | “gpui-component's `Theme::motion` (`MotionTokens`, eleven” · l. 2698 | M2.1 | The model carries motion (S4.1 motion tokens, S2.7 transitions; in M1a), so the gpui connector maps it onto `Theme::motion` when it is rewritten on the slice. |
| docs/todo.md | ### native-theme-gpui connector | “Preset font families that” · l. 2728 | M2.1 | The gpui connector's font mapping (the text ramp, in the slice) is rewritten at M2.1, where the fallback for a family not installed is decided. |
| docs/todo.md | ### native-theme-gpui connector | “Widen the captured screenshot” · l. 2734 | M5 | The showcases are rebuilt and their screenshots regenerated and compared at M5, where the captured tabs are chosen. |
| docs/todo.md | ### native-theme-gpui connector | “Re-verify the upstream `file:line`” · l. 2742 | after 0.8.0 | The upstream toolkits are not pinned sources (D1); their citations in docs/todo.md are re-verified at the next GPUI Kit upgrade, and `docs/todo_gpui-full-theme.md` is archived at M6 as superseded. |
| docs/todo.md | ### native-theme-gpui connector | “**Open question for the” · l. 2756 | M5 | A maintainer question about the gpui showcase's Widget Info panel, decided when the showcases are rebuilt on the query. |
| docs/todo.md | ### native-theme-gpui connector | “Two focus claims hold” · l. 3007 | M5 | Showcase Widget Info claims that depend on the focus ring, revisited when the showcases are rebuilt. |
| docs/todo.md | ### native-theme-gpui connector | “**There is no ambient” · l. 3020 | M2.1 | The body font's weight is in the slice's text ramp (S4.4), so the gpui connector's rewrite decides whether `Root` gets it. |
| docs/todo.md | ### native-theme-gpui connector | “**Model the widgets the” · l. 3038 | M3 unit typed widgets batch | The catalogue now types `badge`, `breadcrumb`, `keycap` (Kbd) and `chip` (Tag), each stated from facts in their alphabetical batches; Alert is a `dialog` variant and `banner`. |
| docs/todo.md | ### native-theme-gpui connector | “**The showcase ignores its” · l. 3051 | M5 | The showcases are rebuilt on the query at M5 and draw text from the text ramp (S4.4), where the Headings demo's mapping is decided. |
| docs/todo.md | ### native-theme-gpui connector | “**`Size::Size(px)` is an escape” · l. 3073 | M2.1 | A survey of gpui's `Size` consumers that the gpui connector's rewrite needs to know what it can map and what its contract lists (S21.3). |
| docs/todo.md | ### native-theme-gpui connector | “**A gate the audit” · l. 3097 | M5 | The showcase gates (element lists, parity, widget coverage) are rebuilt with the showcases (S18), where which gates read prose as evidence is checked. |
| docs/todo.md | ### native-theme-gpui connector | “**A sidebar width and” · l. 3110 | M3 unit containers | The catalogue's `sidebar` states `width_px` and `preferred_width_fraction` with pane limits; the tooltip half is already done. |
| docs/todo.md | ### native-theme-gpui connector | “**Platform icons inside widgets:” · l. 3140 | M2.1 | Per-part glyphs are model data (S2.9, `ResolvedTheme::icon`, S16), so the connectors map them where the toolkit takes an icon, from the slice on. |
| docs/todo.md | ### native-theme-gpui connector | “**`geometry::button` gives every variant” · l. 3151 | M2.1 | The button is the slice: the gpui connector's rewrite maps each button variant's stroke per state from the query, replacing `geometry::button`. |
| docs/todo.md | ### native-theme-gpui connector | “**A tab's `min_height` has” · l. 3166 | M3 unit containers | The tab's sizes are the catalogue's `tab` entry; the gpui connector maps them in the containers unit's connector task. |
| docs/todo.md | ### native-theme-gpui connector | “**An empty state's icon” · l. 3174 | M5 | An empty-state demo's frame in the gpui showcase, revisited when the showcases are rebuilt. |
| docs/todo.md | ### native-theme-gpui connector | “**The colour gate cannot” · l. 3186 | M5 | The colour-claim gate is a showcase gate rebuilt with the showcases (S18). |
| docs/todo.md | ### native-theme-gpui connector | “**Reference — how a” · l. 3211 | M2.1 | A reference for the gpui connector's mapping, used when the connector is rewritten on the query. |
| docs/todo.md | ### native-theme-gpui connector | “Port `scripts/check_widget_coverage.py` to a” · l. 3233 | M5 | The widget-coverage step is excluded until M5, when the showcases and their gates return (PLAN_EXCLUDED). |
| docs/todo.md | ### native-theme-gpui connector | “Split the showcases into” · l. 3247 | M5 | The iced showcase is rebuilt on the query at M5, where it can be split into modules. |
| docs/todo.md | ### native-theme-gpui connector | “`scripts/check_widget_coverage.py` still accepts weak” · l. 3252 | M5 | The widget-coverage step returns with the rebuilt showcases at M5. |
| docs/todo.md | ### native-theme-gpui connector | “The repository's PreToolUse hook” · l. 3261 | M0 | D9 covers the project's hooks and their exemptions; M0 checks them, and the examples' exemption can be aligned with that work. |
| docs/todo.md | ### native-theme-gpui connector | “Run each showcase once” · l. 3264 | M6 | A pre-release check of the showcases with an empty configuration, run before the 0.8.0 release. |
| docs/todo.md | ### native-theme-gpui connector | “The gpui showcase builds” · l. 3268 | M5 | A gpui showcase Widget Info gap, revisited when the showcases are rebuilt. |
| docs/todo.md | ### native-theme-gpui connector | “**The iced showcase: per-instance” · l. 3278 | after 0.8.0 | Per-instance Widget Info for iced is a showcase feature iced's widget tree cannot give (no hover walk); the rebuild at M5 does not need it. |
| docs/todo.md | ### native-theme-gpui connector | “**Follow-up plan: the other” · l. 3305 | settled by the model | Every value is stated from a verified fact, or is automatic or absent where the platform says so (R20, S17), so unsourced sizes cannot remain; each field's value comes from its unit's facts (KDE input padding lane kde, iOS sizes lane ios). |
| docs/todo.md | ### native-theme-gpui connector | “**The colour-scheme presets' sizes” · l. 3415 | settled by the model | Palettes carry only colours mapped from the project's published colours; sizes come from the platform preset (R19, S14). |
| docs/todo.md | ### native-theme-gpui connector | “**A machine-readable platform-facts.** The” · l. 3424 | settled by the model | The facts become data per platform with generated tables and an OS-generation key (S17, R22); built in M0 and M1a. |
| docs/todo.md | ### native-theme-gpui connector | “**WinUI's combobox arrow column:” · l. 3446 | M3 unit fields | The catalogue's `combo_box` has an `arrow_area` element with its own geometry, stated per preset in the fields unit. |
| docs/todo.md | ### native-theme-gpui connector | “**Loose ends of the” · l. 3456 | M5 | Showcase test loose ends of the v0.5.9 plan (two already fixed), revisited when the showcases are rebuilt. |
| docs/todo.md | ### native-theme-gpui connector | “PR: add `Window::screenshot()` API” · l. 3485 | not this plan's work | Upstream gpui PR (a public screenshot API). |
| docs/todo.md | ### native-theme-gpui connector | “PR: let an application drop” · l. 3504 | not this plan's work | Upstream gpui PR (dropping a window's SVG tiles). |
| docs/todo.md | ### native-theme-gpui connector | “Audit gpui-component widgets for” · l. 3570 | not this plan's work | Upstream gpui-component audit; the connector's contract lists what the toolkit cannot draw (S21.3, R23). |
| docs/todo.md | ### native-theme-gpui connector | “PR: expose per-widget padding/margin” · l. 3572 | not this plan's work | Upstream gpui-component PR; the contract lists the receivers that are missing (S21.3). |
| docs/todo.md | ### native-theme-gpui connector | “PR: expose checkbox/radio indicator” · l. 3573 | not this plan's work | Upstream gpui-component PR (inner geometry); the contract lists the receivers that are missing (S21.3). |
| docs/todo.md | ### native-theme-gpui connector | “PR: expose scrollbar dimensions” · l. 3580 | not this plan's work | Upstream gpui-component PR; the contract lists the receivers that are missing (S21.3). |
| docs/todo.md | ### native-theme-gpui connector | “PR: expose button min-height” · l. 3581 | not this plan's work | Upstream gpui-component PR; the contract lists the receivers that are missing (S21.3). |
| docs/todo.md | ### native-theme-gpui connector | “PR: let a scroll” · l. 3585 | not this plan's work | Upstream gpui and gpui-base scrolling behaviour (scroll chaining), not a theme value. |
| docs/todo.md | ### native-theme-gpui connector | “Additional PRs as gaps” · l. 3650 | not this plan's work | Placeholder for further upstream PRs. |
| docs/todo.md | ### native-theme-gpui connector | “Report: a single-select `Combobox`” · l. 3651 | not this plan's work | Upstream gpui-component Combobox defect report. |
| docs/todo.md | ### native-theme-gpui connector | “PR: let a consumer” · l. 3664 | not this plan's work | Upstream gpui-component icon-provider hook. |
| docs/todo.md | ### native-theme-gpui connector | “Left open by the” · l. 3706 | M5 | Showcase-layout loose ends (title bar branches, the menu-bar row, a tab test), revisited when the showcases are rebuilt; the menu bar's own fill is now the catalogue's `menu_bar`. |
| docs/todo.md | ### native-theme-gpui connector | “The gpui showcase's Retina” · l. 3723 | M5 | A macOS capture without the nudge is taken when the showcases' screenshots are regenerated at M5. |
| docs/todo.md | ### native-theme-gpui connector | “PR: Button: let the” · l. 3732 | not this plan's work | Upstream gpui-component PR (Button label text size). |
| docs/todo.md | ### native-theme-gpui connector | “PR: ColorSelect: apply the” · l. 3740 | not this plan's work | Upstream gpui-component PR (ColorSelect refinement). |
| docs/todo.md | ### native-theme-gpui connector | “PR: DataTable: `Styled`, or” · l. 3743 | not this plan's work | Upstream gpui-component PR (DataTable text size). |
| docs/todo.md | ### native-theme-gpui connector | “a public way to” · l. 3745 | not this plan's work | Upstream gpui-base or GPUI PR (an accessibility disabled marker). |
| docs/todo.md | ### native-theme-gpui connector | “PR: an `Icon` that” · l. 3752 | not this plan's work | Upstream gpui-component PR (image icons in widget slots). |
| docs/todo.md | ### native-theme-gpui connector | “PR: let an application set” · l. 3775 | not this plan's work | Upstream gpui-pre-windows PR (window appearance on Windows). |
| docs/todo.md | ### gpui: a tab scrolled into view loses its outer flare | “Found 2026-10-22 in the” · l. 3804 | M3 unit containers | The tab's outward corner radius is the catalogue's `tab` geometry; the gpui TabBar's reveal is fixed in the containers unit's connector task. |
| docs/todo.md | ### Connectors: a public theme builder that takes the mode | “gpui's `to_theme(&resolved, name, is_dark:” · l. 3808 | M2.1 | The gpui connector's public API is rewritten on the new model at M2.1, where today's `ColorMode` is S16's `Scheme`. |
| docs/todo.md | ### iced and egui: a gate for upstream functions that panic on their input | “G4 gives the gpui” · l. 3812 | not this plan's work | A panic gate for the iced and egui connectors belongs to those crates' next upgrade, unrelated to the model. |
| docs/todo.md | ### gpui-fast pins `taffy = "=0.15.0"` | “gpui-fast 0.1.12 requires taffy” · l. 3829 | not this plan's work | Upstream gpui-fast's exact taffy pin, for the maintainer to raise. |
| docs/todo.md | ### Showcases: a gate for one showcase's citations of another's lines | “The egui and iced” · l. 3860 | M5 | A gate for one showcase's citations of another's lines, built when the showcases are rebuilt on the query and their citations change. |
| docs/todo.md | ### gpui: the panic gate's sweep reads function bodies too | “G4's gate for upstream” · l. 3884 | not this plan's work | The panic gate's body sweep belongs to the next GPUI Kit upgrade, unrelated to the model. |
| docs/todo.md | ### gpui showcase: the resize handle's width in Widget Info and `content_width` | “G4's final review (2026-10-27)” · l. 3911 | M5 | A gpui showcase Widget Info and layout estimate, re-read when the showcases are rebuilt. |
| docs/todo.md | ### gpui showcase: look at the overlays that open on input | “The v0.7.0 release review” · l. 3927 | M5 | The overlays are compared on screen with the platforms when M5 regenerates and compares the screenshots. |
| docs/todo.md | ### Mobile readers | “iOS: `from_ios()` via `objc2-ui-kit`” · l. 3949 | after 0.8.0 | R18: the iOS and Android readers are a platform-support milestone after the model release; until then the application stands in for the reader (S16 `set_reader_stand_in`). |
| docs/todo.md | ### Mobile readers | “Android: `from_android()` via `jni`” · l. 3950 | after 0.8.0 | R18: the iOS and Android readers are a platform-support milestone after the model release; until then the application stands in for the reader (S16 `set_reader_stand_in`). |
| docs/todo.md | ### Windows reader | “**A live Windows hover” · l. 3953 | M3 unit readers | The catalogue's readers section lists the Windows read of `SPI_GETMOUSEHOVERTIME` (W3-41), so the hover-time read and its watched source come with the readers unit; its `unsafe` grows under `windows.rs`'s existing exception (D4). |
| docs/todo.md | ### Markdown: inline code, list markers and blockquotes in the model | “gpui-component 0.8.1's `TextView` takes” · l. 3967 | M3 unit typed widgets batch | The catalogue's `rich_text` entry carries `inline_code`, `code_block`, `block_quote` and `list_marker` parts (K-SF13†–K-SF15†), so their values and gpui's `TextView` refinements come with that batch and its connector task. |
| docs/todo.md | ### Font DPI | “Unverified: do the Linux” · l. 3986 | M1b | The readers' port at the slice's scope builds `ReaderState.font_dpi` (S9.4: `forceFontDPI` or `Xft.dpi`), so whether that DPI is logical under Wayland scaling is settled where the KDE and GNOME readers are ported. |
| docs/todo.md | ### Font DPI | “An X11 client through” · l. 3988 | not this plan's work | Upstream: no toolkit tells a theme reader which backend (Xwayland, `startx`) the client uses; the double scaling is winit's and gpui's. |
| docs/todo.md | ### docs.rs feature badges | “docs.rs `doc(cfg)` badges: mark” · l. 3992 | not this plan's work | A documentation-tooling chore (nightly `doc_cfg` badges) that the model does not touch. |
| docs/todo.md | ### `system_icon_set()` decides where it should report | “`system_icon_set()` returns `IconSet::Material` on” · l. 3995 | settled by the model | S16 replaces `system_icon_set` and its forms by the resolved `token.icons` (S4.5), which the preset states and the reader states where the platform has a setting, so no fallback set is chosen by the library. |
| docs/todo.md | ### iced: filing variable-font weights is a workaround for cosmic-text 0.15 | “iced 0.14 draws text” · l. 3998 | not this plan's work | Upstream wait on iced moving to cosmic-text 0.19. |
| docs/todo.md | ### iced showcase: the macOS system menu goes in after the window opens | “On macOS the iced” · l. 4001 | after 0.8.0 | The iced showcase's start-up order is application structure, not theme use; M5 rebuilds the showcases' theme reading only. |
| docs/todo.md | ### macOS: the stated font family "SF Pro" is not a family iced's font database holds | “In the v0.5.9 CI” · l. 4004 | lane macos | The family name a macOS preset states for the system font is a text-ramp fact (S2.8, S4.4) the macOS lane settles on the pinned runner (macos-tahoe in M1a, macos-monterey in M1b); the connectors then check the name at M2.1. |
| docs/todo.md | ### gpui: Settings and dock splitters draw upstream's renderer | “gpui-component 0.7.0's `Settings` divider” · l. 4007 | not this plan's work | Upstream: gpui-component's `Settings` and dock install their own resize-handle renderer; the gpui contract lists the splitter values it cannot reach there (S21.3). |
| docs/todo.md | ### Popover arrow | “Record per platform whether” · l. 4018 | M3 unit overlays | The catalogue's `popover` entry carries the `arrow` part (the tail) with its availability per preset, so the overlays unit's facts record it per platform. |
| docs/todo.md | ### Showcases: the toolbar's "Reload System Theme" glyph | “All three showcases draw” · l. 4021 | after 0.8.0 | A showcase glyph choice unrelated to the model; can be taken in M5's rebuild. |
| docs/todo.md | ### gpui: a citation gate for the connector's doc comments | “The showcase's citations are” · l. 4024 | after 0.8.0 | A citation gate for the gpui connector's prose is tooling the model does not need; the connector's doc comments are rewritten at M2.1, so the gate is worth adding after. |
| docs/todo.md | ### gpui: the splitter renderer into the connector | “The gpui showcase paints” · l. 4042 | M3 unit containers | gpui-base's resize handle takes an appearance on public API since 0.8.0, so the gpui connector can draw the `splitter` widget's values itself; the containers unit's connector task maps them (R23). |
| docs/todo.md | ### Showcases: a toolbar role and arrow-key roving for the chrome toolbar | “The chrome toolbar of” · l. 4053 | after 0.8.0 | The showcases' chrome toolbar accessibility (role, arrow-key roving) is showcase behaviour, not theme data. |
| docs/todo.md | ### gpui: Questionnaire needs an infallible state constructor | “`QuestionnaireState::new` returns a `Result`” · l. 4056 | not this plan's work | Upstream: gpui-base's `QuestionnaireState::new` needs an infallible constructor. |
| docs/todo.md | ### gpui showcase: the Basic tooltip's bubble is never recorded | “The gpui showcase records” · l. 4061 | M5 | The showcases' element lists and parity gates are rebuilt on the query at M5 (S18), where the tooltip bubble gets its own dump entry and panel. |
| docs/todo.md | ### check_showcase_parity.py: the element list's union rule is unchecked | “`docs/showcase-elements.toml` states "A control's” · l. 4064 | M5 | `check_showcase_parity.py` and `docs/showcase-elements.toml` follow the connectors at M5 (S18, R22); the union rule's check belongs to that rewrite. |
| docs/todo.md | ### M6 close (2026-10-16): items carried to M7, M8, M9a and the presets | “Presets (kde-breeze), found by” · l. 4072 | lane kde | Every value named (`segmented_control` hover, the expander's arrow colour and border) is a kde-breeze fact the KDE lane states from pinned Breeze sources; the preset is rewritten from facts (S19.1). |
| docs/todo.md | ### M6 close (2026-10-16): items carried to M7, M8, M9a and the presets | “Presets (windows-11): platform-facts §2.26” · l. 4077 | lane windows | The card's fill is a windows-11 fact (`CardBackgroundFillColorDefault`) the Windows lane states; with no model inheritance the card states it itself (R20, S19.2). |
| docs/todo.md | ### M6 close (2026-10-16): items carried to M7, M8, M9a and the presets | “Presets (windows-11): the checkbox's” · l. 4085 | lane windows | The checkbox's hover fill (a layer over the window or a replacement) is a windows-11 fact the Windows lane settles from microsoft-ui-xaml's visual states (S2.3 paint layers). |
| docs/todo.md | ### M7 fix phase (2026-10-16) | “epaint 0.36.2 asserts that” · l. 4088 | not this plan's work | Upstream: epaint panics on a glyph wider than its atlas; the egui connector's size cap stays until upstream skips it. |
| docs/todo.md | ### M7 close (2026-10-16): items carried to M8, M9a and the presets | “egui text-size cap residuals” · l. 4098 | after 0.8.0 | The egui connector's text-size cap residuals are a robustness limit of the connector, not model data (R21: the connectors' feature work stays later). |
| docs/todo.md | ### gpui showcase: the expander header does not grow above text scale 1 | “The gpui expander header” · l. 4101 | M3 unit containers | The expander's header height under text scaling is resolved by the model (S9.4, S2.11 `min_height`), and the gpui connector maps it in the containers unit's connector task. |
| docs/todo.md | ### Connectors: gpui draws `button.min_height` as an exact height where the model states a minimum | “The model states `button.min_height`” · l. 4104 | M2.1 | The model states `min_height` and `max_height` as bounds on every part (S2.11), so `button.min_height` is a minimum; the gpui connector maps it as one when it moves to the slice. |
| docs/todo.md | ### egui showcase: the Basic list's contents paint over its frame by fractions of a point | “The egui showcase's Basic” · l. 4107 | M5 | The egui showcase's list and its scroll area are rebuilt on the query at M5; the frame-crossing paint is fixed there with the kittest the entry asks for. |
| docs/todo.md | ### egui: the font plan reads the light variant's families only | “Latent: `from_preset`, `SystemThemeExt::to_egui_atlas` and” · l. 4115 | M2.1 | The egui connector's font plan is rebuilt from the query's text ramp when it moves to the slice, which decides whether both schemes' families load. |
| docs/todo.md | ### iced: a PickList cuts a long label without an ellipsis, and its menu rows run out of the menu | “iced 0.14's `PickList` has” · l. 4118 | M3 unit fields | The `combo_box` text's elision is model data (S2.8 `Elision`); iced's `PickList` cannot elide, which the iced contract lists at the fields unit's connector task (R23); the showcase's label cut belongs to M5. |
| docs/todo.md | ### egui showcase: the command palette's highlight does not follow the pointer | “The egui showcase's command” · l. 4121 | after 0.8.0 | The egui command palette's pointer-follow is showcase behaviour, not theme data. |
| docs/todo.md | ### Connectors: an unstated `button.active_background` gives egui's and gpui's pressed fill two colours | “For the maintainer to” · l. 4124 | settled by the model | No field is left unstated (R20, S12: missing is an error), so a pressed button's fill is stated per state on every preset and neither connector needs a fallback of its own. |
| docs/todo.md | ### Showcases: gpui's keyboard shield also stops its own macOS menu shortcuts | “For the maintainer to” · l. 4127 | after 0.8.0 | The gpui keyboard shield's coverage of macOS menu shortcuts is showcase capture behaviour, not theme data. |
| docs/todo.md | ### gpui showcase: the page tab strip does not scroll to a page opened another way | “Where the page tabs” · l. 4130 | after 0.8.0 | The gpui showcase's page tab strip scrolling is showcase behaviour, not theme data. |
| catalogue | ## Open items | “`context_help`” | M3 unit typed widgets batch | `context_help`'s two questions (Breeze's popup shadow on What's This, K2.31†; macOS's mode cursor, MAC-P4-11†) are answered by the KDE and macOS facts of its batch; a finding changes the catalogue (class 1). |
| catalogue | ## Open items | “`ink_toolbar`” | M3 unit typed widgets batch | Whether each platform draws every part (W2-36, C-40†) is the batch's facts step on every preset. |
| catalogue | ## Open items | “`level_indicator.value_label`” | M3 unit typed widgets batch | The value label on the `macos-*` and `ios-*` gauges (MAC-N22, C-30) is settled by the macOS and iOS facts of `level_indicator`'s batch. |
| catalogue | ## Open items | “`sheet.presenter_backdrop`” | lane ios | Whether iOS 26 still draws the card effect behind a sheet (C-12) is an `ios-26` fact (D12: measured on the iOS 26 simulator); it enters with `sheet`'s typed-widgets batch. |
| catalogue | ## Open items | “`component.context_menu_preview`” | lane ios | Both open parts (`component.context_menu_preview`, `bottom_tab_bar.accessory` and its `UITabAccessory.Environment` variants) are iOS 26 facts; they enter with the keyed-components unit and `bottom_tab_bar`'s batch. |
| catalogue | ## Open items | “`text_area.highlight`” | lane macos | macOS 15's highlight styles (MAC-X28) are a `macos-monterey` fact; it enters with the fields unit. |
| catalogue | ## Open items | “`avatar.fallback_icon`” | lane gnome | Whether AdwAvatar draws an icon fallback is an `adwaita` fact from libadwaita at its pin; it enters with `avatar`'s typed-widgets batch. |
| catalogue | ## Open items | “`button`” | M1b | `ios-26` is created at M1b at the slice's scope, `button` included, so iOS 26's further bar-item roles are declared (or found absent) from the iOS lane's facts there. |
| catalogue | ## Open items | “`time_picker`” | lane macos | Whether `NSDatePicker`'s graphical style with time elements alone draws the clock face (MAC-N04) is measured by the macOS probe; it enters with `time_picker`'s typed-widgets batch. |
| catalogue | ## Open items | “`spinner`” | lane ios | SwiftUI's determinate circular ProgressView (C-31) is an iOS measurement (spike I's host); it enters with the indicators unit, and its provisional grants are confirmed or corrected there. |
| catalogue | ## Open items | “`expander`” | lane ios | SwiftUI's standalone DisclosureGroup is measured by the iOS host; `ios-*` stays `available = false` until then; it enters with the containers unit. |
| catalogue | ## Open items | “`component.casting_picker`” | lane windows | The part and field `AccentColor` and `SelectedAccentColor` paint (W-P4-18†) is a Windows fact; it enters with the keyed-components unit. |
| ROADMAP.md | ### Full theme geometry in the iced connector | (subsection) | M2.1 | The iced connector moves to the model at M2.1 and maps each unit's fields in that unit's connector task (S21.3); the subsection is rewritten with ROADMAP at M6. |
| ROADMAP.md | ### Full theme geometry in the iced connector | “replacements for the iced classes” | after 0.8.0 | Replacing iced's own palette-painted classes is connector feature work beyond mapping the resolved fields (R21: the connectors' feature milestones stay later); S21.3's contract already makes every resolved field mapped or listed. |
| ROADMAP.md | ### Full theme geometry in the iced connector | “helpers for the sizes iced” | after 0.8.0 | iced takes these sizes only through builders the application calls, so the contract (S21.3) records them with iced's builder lines; helpers are a connector feature (R21), as docs/todo.md's *The iced `geometry` gap* entry is triaged. |
| ROADMAP.md | ### Full theme geometry in the iced connector | “shadows: every style keeps” | M2.1 | The model carries shadow geometry in every part's `shadow` (S2.5), so the iced styles map it from the slice's button on and per unit after. |
| ROADMAP.md | ### Upstream receivers for the gpui connector | (subsection) | not this plan's work | Upstream PRs to gpui-kit; the gpui connector's contract lists each value its toolkit cannot draw with the toolkit's lines (S21.3, R23). |
| ROADMAP.md | ### Upstream receivers for the gpui connector | “a styled `Theme` scrollbar-style” | not this plan's work | Upstream PR to gpui-kit. |
| ROADMAP.md | ### Upstream receivers for the gpui connector | “`Tab` keeping the caller's” | not this plan's work | Upstream PR to gpui-kit. |
| ROADMAP.md | ### Upstream receivers for the gpui connector | “`Theme.shadow` honoured beyond `Button`,” | not this plan's work | Upstream PR to gpui-kit. |
| ROADMAP.md | ### Upstream receivers for the gpui connector | “`Size::Size` honoured by `Checkbox`” | not this plan's work | Upstream PR to gpui-kit. |
| ROADMAP.md | ### Upstream receivers for the gpui connector | “inner geometry exposed: checkbox/radio” | not this plan's work | Upstream PR to gpui-kit. |
| ROADMAP.md | ### Upstream receivers for the gpui connector | “a `PopupMenu` item style” | not this plan's work | Upstream PR to gpui-kit. |
| ROADMAP.md | ### Upstream receivers for the gpui connector | “a Button label that” | not this plan's work | Upstream PR to gpui-kit. |
| ROADMAP.md | ### Upstream receivers for the gpui connector | “`ColorSelect` applying the caller's” | not this plan's work | Upstream PR to gpui-kit. |
| ROADMAP.md | ### Upstream receivers for the gpui connector | “`DataTable` as `Styled`, or” | not this plan's work | Upstream PR to gpui-kit. |
| ROADMAP.md | ### Upstream receivers for the gpui connector | “an iterable `IconName::ALL` generated” | not this plan's work | Upstream PR to gpui-kit. |
| ROADMAP.md | ### Upstream receivers for the gpui connector | “public base-palette fields on” | not this plan's work | Upstream PR to gpui-kit. |
| ROADMAP.md | ### Upstream receivers for the gpui connector | “`Diff`'s code size taken” | not this plan's work | Upstream PR to gpui-kit. |
| ROADMAP.md | ### Upstream receivers for the gpui connector | “a TextView rule colour” | not this plan's work | Upstream PR to gpui-kit. |
| ROADMAP.md | ### Upstream receivers for the gpui connector | “a scaling `DialogEntrance`: gpui-component” | not this plan's work | Upstream PR to gpui-kit; the model carries the dialog's transition (S2.7), which gpui draws as `Fade` until then. |
| ROADMAP.md | ### Upstream receivers for the gpui connector | “GPUI's own `.tooltip()` honouring” | not this plan's work | Upstream PR to gpui. |
| ROADMAP.md | ### Upstream receivers for the gpui connector | “gpui-base's tooltip hide delay” | not this plan's work | Upstream PR to gpui-kit. |
| ROADMAP.md | ## Also in the 0.7.x releases | “[`docs/todo.md`](docs/todo.md) tags 29” | M0 | D11: no planned 0.7.x release; the section is rewritten at M0 to say the `[v0.7.x]` items are triaged into 0.8.0 or carried after it. |
| docs/todo_gpui-full-theme.md | # Full per-widget theming for the gpui connector | (whole plan) | M6 | Superseded: the gpui connector moves to the model at M2.1 and the units' connector tasks, its gap analysis becoming the contract table (S21.3); the plan archives it at M6 with the other superseded connector plan. |
| docs/todo_gpui-full-theme.md | ### What native-theme provides vs what the connector uses | (gap table) | M2.1 | The gap table is replaced by the gpui connector's contract table (S21.3): every resolved field mapped with gpui's lines or listed with the reason it cannot be drawn. |
| docs/todo_gpui-full-theme.md | ### Execution plan | (Option F2, Steps 1–5) | not this plan's work | Superseded by v0.5.8's geometry seams (the plan's status line); Step 5's upstream proposal is upstream work. |
| docs/todo_iced-full-theme-geometry.md | # Iced Connector: Full Theme Geometry Support | (whole plan) | M6 | Partly implemented and superseded: its remaining work is ROADMAP's *What remains*, triaged in its rows; the plan archives it at M6. |
| docs/todo_iced-full-theme-geometry.md | ## Summary: Functions to Implement | (remaining functions) | after 0.8.0 | The functions still missing are the palette-painted classes of ROADMAP's first *What remains* bullet, triaged the same way (R21). |
| docs/todo_iced-full-theme-geometry.md | ### Font Configuration Helper | (apply_fonts) | after 0.8.0 | A helper setting iced's `Settings` default font is a connector feature (R21); the text ramp's font itself is mapped at M2.1. |
| docs/todo_iced-full-theme-geometry.md | ### Widget Metrics Helpers (already partially exist) | (checkbox_size … tooltip_padding) | after 0.8.0 | Builder-taken sizes are recorded in the contract with iced's builder lines (S21.3); helpers for them are a connector feature (R21). |
| docs/todo_iced-full-theme-geometry.md | ### Spacing Helpers | (spacing) | settled by the model | The model has no portable spacing scale: spacings are keyed per preset in `token.params.layout` (S4.6), so a `spacing(size)` helper has nothing portable to read. |
