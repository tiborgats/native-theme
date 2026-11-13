# Updating the Platform Facts for a New OS Version

native-theme takes its platform values from **facts** in
`docs/facts/<platform>/` (specification S17,
[`todo_v0.7.1_model-completeness-spec.md`](todo_v0.7.1_model-completeness-spec.md)):
a bundled preset is generated from its platform's facts, and the few lines no
fact states (its generation, its declarations) are written by hand, each
citing its source. When a new OS version
or a new release of a platform's toolkit ships (KDE Plasma 7, the next macOS,
a libadwaita release, a Windows App SDK release), the facts are checked
against it: the sources are pinned to the new release, what changed becomes a
fact, the preset is regenerated, and the gates say when it is done.

This guide covers that procedure, then each platform's sources and
measurements.

---

## The facts

One directory per platform, each covering its presets
(`scripts/check_facts.py`'s `PLATFORMS`):

| Directory | Presets |
|---|---|
| `docs/facts/gnome/` | `adwaita` |
| `docs/facts/kde/` | `kde-breeze` |
| `docs/facts/macos/` | `macos-monterey`, `macos-tahoe` |
| `docs/facts/ios/` | `ios-13`, `ios-26` |
| `docs/facts/windows/` | `windows-11` |
| `docs/facts/material/` | `material-3` |

A fact is a `[[fact]]` table: the model `key` it states (a path without the
unit suffix, `button.body.min_height`), its `value` as a preset spells it, the
gap-map `rows` it settles, an optional `when` (the conditions, variants and
states it holds under; `preset:<id>` where one generation of the directory
differs from the other), and at least one `[[fact.cite]]` of one of three
kinds:

- `published`: a source pinned in `docs/facts/sources.toml`, its file and
  lines, and a `quote` that occurs there (`absent` names what the lines must
  not hold);
- `read`: an API a reader calls, its definition cited as a `published` one is;
- `measured`: an evidence file under `docs/evidence/<harness>/<date>-run-<id>/`
  with the path of the value in it and a `tolerance`; the run's `run.toml`
  maps each runner image to the preset it measures, and the file records the
  exact OS and toolkit builds. The path names a file of the evidence
  repository,
  [`native-theme-evidence`](https://github.com/tiborgats/native-theme-evidence),
  at the commit `docs/facts/sources.toml` pins (its `evidence = true` entry):
  the checkers and the facts gate read it from a clean checkout at that
  commit, `NATIVE_THEME_EVIDENCE`'s or the source cache's
  (`./scripts/update_sources.sh --only native-theme-evidence`).

For example, adwaita's button height (`docs/facts/gnome/button.toml`):

```toml
[[fact]]
key = "button.body.min_height"
value = "= add(24px, add(part.padding.top, part.padding.bottom))"
rows = []

[[fact.cite]]
kind = "published"
source = "libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:20"
quote = "min-height: 24px;"
```

Where no source gives a value and nothing has measured it yet, the fact is
**pending**: `pending = "U<n>"` in place of the value, naming its entry in the
unmeasured register of the plan's ledger
([`todo_v0.7.1_model-completeness-ledger.md`](todo_v0.7.1_model-completeness-ledger.md)),
and the preset lists the path in its `[pending]` table. A value is never
estimated, copied from another platform or left to a toolkit's default.

`docs/platform-facts.md` lists every value the facts state, per preset, with
its citations. It is generated (`scripts/generate_platform_facts.py`), never
edited and not committed: CI and each release generate it, and a local copy
is written on demand; the hand-written document it replaced, which the planning
documents cite by line, is the one commit `fc6109d1` holds
(`platform-facts.md@fc6109d1:N`).

---

## A new generation, a new preset

One platform preset per visual generation. A new OS version of the same
generation updates the facts of the platform's preset; a new visual
generation gets a new preset beside it, whose key is the vendor's own name
for the generation's first OS version:

| Platform | The key names | Example |
|---|---|---|
| macOS | Apple's codename | `macos-monterey`, `macos-tahoe` |
| Windows | Microsoft's version number | `windows-11` |
| iOS | the iOS version number | `ios-13` (the flat look of iOS 13 to 18), `ios-26` |
| Material | the design version | `material-3` |
| KDE, GNOME | the style, not the desktop | `kde-breeze`, `adwaita` |

The style presets keep their style names: the KDE and GNOME readers read the
installed style, so the style, not the desktop's version, is the generation.
A preset describes the newest release of its generation that a supported
runner measures (the plan's D12), and its `[preset]` table's `generation`
(`libadwaita-1.10`, `macos-26`) names it, written by hand with its
`docs/facts/sources.toml` entry cited.

On macOS and Windows the running OS version chooses the generation, through
the generation tables in `native-theme/src/pipeline.rs`
(`MACOS_GENERATIONS`, `WINDOWS_GENERATIONS`): each row is a preset, the first
OS version of its look and the newest version its facts were verified on
(`last_verified`). A newer version of the same generation, once its facts are
checked, raises its row's `last_verified`; until then `version_match` reports
`PresetOlder` on it. A new generation is a new row.

---

## The procedure

### 1. Pin the release

`docs/facts/sources.toml` pins every platform source to one release set: one
`[[source]]` per source and generation, its id `<name>@<version>` (a commit
pin's version is its commit's first eight hex digits), its `generations` the
presets it serves. Unversioned documentation is a dated snapshot whose
`describes` states the release it describes, and serves only the presets that
describe that release. Add or move the entries for the new release, then:

```sh
python3 scripts/check_sources.py        # the file's form, and every cited project pinned
./scripts/update_sources.sh             # fetch the new pins into the source cache
./scripts/update_sources.sh --verify    # every checkout at its pin (offline)
```

The evidence entry is no platform source: its commit is this project's own,
moved by the facts commits that add runs (step 3), never by a re-pin.

While the 0.8.0 plan runs, the release set is frozen until the catalogue's
last unit is released (the plan's D1, D13), so that every unit's facts
describe one release set. A re-pin then is recorded in the ledger, and every
enabled unit's facts are checked again at the new pin; a new OS version that
waits for the freeze to end is recorded in `docs/todo.md`, as macOS 27 and
iOS 27 are.

### 2. Re-check the facts at the new pin

Move each fact's citations to the new source ids, then let the citation check
say which no longer hold:

```sh
python3 scripts/check_fact_citations.py   # each quote at its pin, each measured value in its evidence
```

A quote that no longer occurs at the cited lines is a source that moved or
changed: find the value in the new release and correct the fact, its value,
its lines and its quote. Within a generation the preset describes its newest
release, so a changed value replaces the fact. Two generations of one
platform share their directory's facts where both hold; where they differ,
each fact names its generation, `when = { conditions = ["preset:<id>"] }`.

### 3. Measure what no source gives

The measurements run on CI, never on a developer's desktop (the plan's D2,
D3):

- `tools/os-probe` (`native-theme-os-probe`, never published) records what
  macOS or Windows reports (fonts, colours, control sizes, system metrics) and
  on macOS renders each control, on each runner image of
  `.github/workflows/os-probe.yml`'s matrix. Add the new OS version's runner
  image once GitHub offers it as generally available (a preview image's run
  may fail without failing the workflow, so it measures nothing a preset may
  rely on).
- The 0.8.0 plan's harnesses: the KDE and GNOME containers (Plasma and
  libadwaita under a pinned distribution snapshot, the KDE accent run, the
  KDE render host, the GNOME harness and vector generator), the Android
  emulator host, the WinUI host and the iOS simulator host. They are jobs of
  the same workflow on the `v0.8.0-tools` branch, chosen by its `jobs` and
  `images` inputs:

  ```sh
  gh workflow run os-probe.yml --ref v0.8.0-tools -f jobs=probe -f images='["macos-26"]'
  ```

Copy the run's artifacts into a new `docs/evidence/<harness>/<date>-run-<id>/`
of the evidence repository,
[`native-theme-evidence`](https://github.com/tiborgats/native-theme-evidence),
never of this one, with its `README.md` and `run.toml`, and add the run to its
`docs/evidence/README.md`'s table: GitHub deletes a run's artifacts after 90
days, and a recorded file is never edited afterwards (a later reading is a new
run). Commit the run on the evidence repository's branch of the same name as
this repository's branch (cut from the commit `docs/facts/sources.toml` pins),
push it, then cite the files from `measured` facts in a commit that moves the
pin (`commit` of the `evidence = true` entry) to that branch's head; the
checks read the new run once the cache's checkout is at the new pin
(`./scripts/update_sources.sh --only native-theme-evidence`, or a clone of
your own named by `NATIVE_THEME_EVIDENCE` in a worktree). When the branch
merges, the evidence branch merges into the target's pinned commit too (never
a rebase or a squash) and the merge pins the result (the 0.8.0 plan's *The
evidence repository*).

### 4. Verify independently

Each unit's facts of a platform are checked by a verifier that is not their
author, from the pinned sources alone (the plan's *How values are checked*):
it derives the unit's state model and declarations first, then a value and a
citation for each key, without seeing the facts, and the two are compared
mechanically. Each disagreement goes to a tie-breaker with both citations and
the source, and so does an audit sample of the agreed facts, which measures
how often an author and a verifier agree on a wrong value.

### 5. Regenerate the preset

```sh
python3 scripts/generate_preset.py <preset-id> --write   # the preset from its facts, the hand-written blocks carried over
python3 scripts/generate_preset.py <preset-id> --check   # whether --write would change the file
python3 scripts/generate_platform_facts.py               # docs/platform-facts.md from the facts, a local copy (not committed)
```

`--write` regenerates every line of `native-theme/src/presets/<id>.toml`
outside its hand-written blocks (`[preset] generation`, the `[declare]` keyed
namespaces and the bases the `read` facts give) and reports on stderr what
the facts leave pending and which model fields the preset leaves unstated.
[`native-theme/src/presets/README.md`](../native-theme/src/presets/README.md)
describes the blocks.

A new preset also needs:

- its entry in the `presets!` list of `native-theme/src/presets/mod.rs`
  (`PresetId`: the variant, its key and its file), which `PresetId::all`
  gives the build test and the facts gate;
- its directory's entry in `scripts/check_facts.py`'s `PLATFORMS`, if the
  directory does not name it yet;
- its row in the generation table, on macOS and Windows.

### 6. Run the gates

```sh
python3 scripts/check_facts.py                 # the facts' form
python3 scripts/check_fact_citations.py        # what their citations say, at the pins
python3 scripts/check_facts_trace.py --enabled # each trace line of the enabled units carried by a fact or a verdict
python3 scripts/check_function_vectors.py      # every platform function the facts call has its vector table
cargo test -p native-theme                     # the facts gate (tests/facts_gate.rs) and the build test
./scripts/check_release.sh --commit
```

The facts gate resolves each preset with no reader and compares every
widget's state-model combinations and every token, colour-role and input fact
with the fact that applies; the build test (`every_bundled_preset_validates`)
reads and validates every bundled preset. A platform function a fact calls
(`kcu_mix`, `adw_standalone`, …) is implemented in
`native-theme/src/platform/` only with a vector table generated from its
reference implementation; a function an LGPL project defines only through the
clean room, from a reviewed written description, never from its code (the
plan's D10).

---

## The platforms

Each platform's release set is its entries in `docs/facts/sources.toml`; the
list below names what a new release usually touches.

### KDE (Breeze)

**Release set:** one dated Arch Linux Archive snapshot (`archlinux@<date>`)
carries the Plasma release (`breeze`, `kwin`, `plasma-workspace`, …), KDE
Frameworks (`kcolorscheme`, `kirigami`, `qqc2-desktop-style`, …), Qt
(`qtbase`, `qtdeclarative`) and kirigami-addons, each pinned at the
snapshot's version.

**What to look for:** `kstyle/breezemetrics.h` and the style's drawing code
(sizes, paddings, frame widths), the colour schemes and KColorScheme's
derivations, Kirigami's units and qqc2-desktop-style's QML controls, and the
`kdeglobals` keys the reader reads.

**Measurements:** the KDE container, its accent run
(`scripts/measure_kde_accent.sh`: Plasma's accent derivation per scheme and
accent) and its render host for QML and Qt Widgets. The KDE colour functions
(`kcu_mix`, `kcu_tint`, `kirigami_tint`, …) have vector tables from
`tools/kde-vectors`; a release that changes KColorUtils or Kirigami's tint
regenerates them.

### GNOME (Adwaita / libadwaita)

**Release set:** libadwaita and GTK (`libadwaita`, `gtk`), the gsettings
schemas, the shell and portal, at one Arch Linux Archive snapshot.

**What to look for:** libadwaita's SCSS (`src/stylesheet/`: the colours,
paddings, radii and font sizes of each widget), GTK's CSS defaults where
libadwaita states nothing, new CSS custom properties, and the portal settings
the reader reads (accent colour, colour scheme, contrast).

**Measurements:** the GNOME container's harness (GTK 4 under mutter's
headless display, the scheme, contrast, accent and fonts pinned) and its
vector generator for libadwaita's colour functions (`adw_standalone`).

### macOS and iOS

**Release set:** each generation's Xcode on its GitHub runner image
(`xcode@16.4` for `macos-monterey` and `ios-13`, `xcode@26.6` for
`macos-tahoe` and `ios-26`), WebKit (for `macos-tahoe`), and dated snapshots
of Apple's documentation (the HIG, the developer documentation as its DocC
JSON).

**What to look for:** the SDK headers (`NSColor`'s and `UIColor`'s semantic
colours, the controls' declarations), the HIG's typography and layout, the
AppKit and UIKit release notes.

**Measurements:** the OS probe on the `macos-*` images (API reads, control
renders in a key window, the accessibility passes) and the iOS simulator
host. Apple's documentation is unversioned: a snapshot taken during a
release's lifetime describes that release only, so a page counts for a
generation's fact only when a snapshot from its lifetime, the pinned SDK
headers or a measurement confirms it (the plan's D12).

### Windows

**Release set:** the microsoft-ui-xaml commit of a Windows App SDK release
(`microsoft-ui-xaml`, `WindowsAppSDK`), the Windows Community Toolkit, and
Microsoft Learn's public source repositories at a commit
(`windows-dev-docs`, `win32`, `sdk-api`, …).

**What to look for:** WinUI 3's theme resources and control templates
(`*_themeresources.xaml`: colours, sizes, paddings, corner radii), the
system metrics and `UIColorType` values the reader reads, the default font.

**Measurements:** the OS probe and the WinUI host on the `windows-11-arm`
image.

### Material (Android)

**Release set:** material-web's token set (the primary reference), a Compose
Material 3 release (`androidx` at a commit), material-color-utilities by
commit, Android's AOSP tag and the emulator's system image.

**What to look for:** the token files
(`tokens/versions/latest/sass/_md-*.scss`), Compose's component defaults and
the dynamic-colour scheme.

**Measurements:** the Android emulator host at the pinned system image.

---

## Adding a new platform

A new platform (a Linux desktop with its own style, a new OS) is a new
facts directory and a new preset, entered as every preset is: its sources
pinned, its facts written and verified, the preset generated and registered
(steps 1 to 6), and its directory added to `scripts/check_facts.py`'s
`PLATFORMS`. Its reader, the code that reads the running system's settings
into the reader layer (specification S9), follows the platform readers the
0.8.0 plan restores at M1b; until then the readers are unlinked, and a
platform is supported through its preset alone (`Theme::from_preset`).
