# v0.7.1 → 0.8.0 — Model completeness: Plan

**Status:** draft, 2026-10-29, for the maintainer's approval.

**Goal:** the model the specification states, implemented; every platform preset written from facts and passing the
facts gate; the four connectors and their showcases moved to it; released as 0.8.0 (R21, S21).

**Inputs:** the gap map ([`todo_v0.7.1_model-completeness-gap-map.md`](todo_v0.7.1_model-completeness-gap-map.md)),
the rationale ([`…-rationale.md`](todo_v0.7.1_model-completeness-rationale.md), "R…") and the specification (core
[`…-spec.md`](todo_v0.7.1_model-completeness-spec.md), "S…"; catalogue [`…-spec-widgets.md`](todo_v0.7.1_model-completeness-spec-widgets.md);
the six traces). This plan decides the order of the work, its gates and what runs in parallel; it decides no part of
the model. Where the work finds the specification wrong, the specification is corrected first (its S22 register gains
a line) and the work follows.

**Branch:** `v0.8.0-rc1`, cut from `v0.7.1-rc1`; nothing is released before every milestone below is closed (R21).

## Global constraints

- **No invented value.** A value enters a fact only from a source pinned by commit or version (kind `published`), from
  an API a reader calls (kind `read`, its definition cited), or from a measurement recorded with its method, input and
  output (kind `measured`, with its tolerance) (S17). A probe's output is kept as the evidence.
- **No runtime panics, no `unsafe`** except S15's module, approved 2026-10-29, written as `sficons.rs` is.
- **Gates are mechanical.** Every task names the command whose result closes it.
- **One writer per file at a time.** Parallel tasks own disjoint files; a shared file (the catalogue, a preset, the
  facts index) has one owner per round.
- **Sources stay pinned.** The upstream sources the facts cite are kept at their pinned commits in one cache
  (decision D1), so a fact's citation can be re-read.
- **Captures announce themselves** (they take the desktop's focus) and are run and compared by the agent.
- **Each milestone closes with** `scripts/check_release.sh --commit` green, its documents updated, a commit, and the
  branch pushed (standing grant for work branches).

## Decisions the maintainer takes (before or at approval)

- **D1 — the source cache.** The facts cite libadwaita, GTK, GtkSourceView, libspelling, libpanel, Breeze,
  Kirigami, qqc2-desktop-style, kirigami-addons, KColorScheme, plasma-workspace, KTextEditor, KSyntaxHighlighting,
  KQuickCharts, microsoft-ui-xaml, the Windows Community Toolkit, material-web's tokens, Compose Material 3 and
  material-color-utilities, each at a pinned commit; together a few GB. Proposed: `/run/media/tibi/work/native-theme-sources/`
  (the work SSD, outside `/home`'s snapshots), read and written by this work; it needs a write grant there.
- **D2 — local measurement on Linux.** KDE is measured on this machine's Plasma session. GNOME (libadwaita 1.10,
  GTK 4.24) and Android (Compose, an emulator) need either packages on this machine or containers/VMs; proposed:
  a GNOME session in a container (Flatpak's GNOME runtime runs libadwaita apps under Plasma, which is enough for
  geometry and colours, not for the shell) and the Android emulator from the SDK under the work SSD; each install is
  the maintainer's to approve. The accent run's KDE facts (S17) never touch the desktop: `plasma-apply-colorscheme`
  runs with a temporary `XDG_CONFIG_HOME`, so `applyScheme()` writes a throwaway kdeglobals that the KDE reader then
  reads.
- **D3 — CI minutes.** Each preset describes its generation's newest release (R1.5), so the probes measure macOS 15
  and 26, iOS 18 and 26 (simulators on the macOS runners) and Windows 11, on GitHub's runners, dispatched by hand as
  `os-probe.yml` is; proposed: one dispatch per platform per probe round, a few rounds per milestone.

## The order

```text
M0 setup ──► M1 vertical slice (all layers, one widget) ──► M2 model core ─────────────┐
                                    └──► M3 facts infrastructure ──► M4 facts per platform ──► M5 presets ──► M6 gate ─┐
                                                                                                    M7 connectors ◄──┘
                                                                                                    M8 release
```

M2 (implementation) and M3–M4 (facts) run in parallel after the slice; M5 needs both; M7 starts once M2's API is
stable and M5 has the slice's presets, and completes after M6.

## M0 — Setup

- Cut `v0.8.0-rc1` from `v0.7.1-rc1`.
- Commit the specification's two working checkers as `scripts/check_spec_names.py` and
  `scripts/check_spec_entries.py` (today in a session's scratchpad), beside `check_spec_trace.py`, each with
  `--self-test`; they stay until the documents are archived (S18).
- The source cache (D1): a script `scripts/update_sources.sh` clones or fetches each project at the commit the facts
  pin, listed in `docs/facts/sources.toml` (name, URL, commit, licence), and verifies each checkout's commit.
- **Gate:** the three spec checkers green; `update_sources.sh --verify` green.

## M1 — Vertical slice: colour roles, tokens and `button`, end to end on all six presets

The slice proves the whole chain before it is scaled: the model's types for colour roles, the motion and focus tokens
and one typed widget (`button`), the expression language with the functions those need, the rule engine, resolution
and the query, the TOML layer format, validation, the facts format and its checker, the facts for that scope on every
platform, the six presets for that scope, the gate's three runs (normal, perturbed, accent) for that scope, and the
gpui connector drawing a button from the query. Every later milestone scales what the slice settled.

- **Implementation:** S1–S3, S4 (motion, focus, KDE and Material tokens), S5 for `button`, S6, S7, S8, S10–S12, S14's
  parser only; the derive generating `ButtonTheme` from its catalogue entry.
- **Facts:** the facts format's first checker (M3 scales it), then `docs/facts/<platform>/colors.toml`,
  `tokens.toml`, `button.toml`, and `state-models.toml` for `button`, on all six platforms (published where a source
  states it; measured on macOS and iOS by the probe, extended for the button's states and sizes, D3).
- **Presets:** the six presets' colour roles, those tokens and `button`, in S11's format, written from the facts.
- **Gate:** `cargo test -p native-theme --test facts_gate` green for the slice's keys on all six presets, including the
  perturbed run and `kde-breeze`'s accent run; the gpui showcase's button page drawn from the query and compared
  side by side with the platform's screenshot under the same theme.
- **Close:** what the slice found wrong in the specification is corrected there (S22) before M2 scales it.

## M2 — Model core (implementation, after M1)

Scaled from the slice, in independent tasks each owning its files, each closed by its tests and the no-panic gates:

1. **Blocks and expressions:** every S2 block, the full function table (S2.2), totality, units (S1).
2. **Widgets:** the 86 typed widgets and 28 keyed components from the catalogue, by catalogue section, the derive
   generating each part's field set, patch and resolved types and the `may_be_absent`/`auto` tables (S5, S6).
3. **Rules, layers, resolution, query** at full scope: conditions, flags, exclusive groups, scope and bindings,
   element data and metrics, `DrawnAs`, presentations (S7, S8).
4. **Validation** (S12), including the exact satisfiability checks, with a test per refusal.
5. **Readers** (S9): today's KDE, GNOME (portal), macOS and Windows readers ported to `ReaderState` and layers, with
   the new reads the traces list (`reader:` lines); `reload_reader`; the KDE fallbacks.
6. **Palettes** (S14): `Palette`, mappings, `from_port` (Catppuccin's and Dracula's KDE ports), `apply_palette`.
7. **Placement** (S13) and **system control rendering** (S15).
8. **API and migration** (S16, S19): `Theme`, `WindowState`, getters, the removed and replaced items; today's
   inheritance rules converted (S19.2); `docs/property-registry.toml` generated with its drift test (S18).
- **Gate:** `cargo test --workspace`, `scripts/check_release.sh --commit`; every S12 refusal has a test that triggers it.

## M3 — Facts infrastructure (after M1, parallel to M2)

- `scripts/check_facts.py`: every fact's key names a model path the registry has, its `when` parses as S11's, its kind
  and source are well formed, its source names a commit in `sources.toml`, its value parses as S2.2's, and every
  measured fact names its evidence file.
- `scripts/generate_platform_facts.py`: `docs/platform-facts.md` from the data (S17); the two existing scripts read
  the data (S18).
- The probe extended: `tools/os-probe` gains the measurements the slice needed and a scriptable mode per widget and
  state (macOS, Windows; iOS through a simulator host app), writing evidence the checker reads.
- **Gate:** the checker and generator `--self-test` green; `platform-facts.md` regenerated without diff for the
  slice's data.

## M4 — Facts per platform (after M3, six lanes in parallel)

One lane per platform, each owning `docs/facts/<platform>/`. Per lane, in this order: colour roles and inputs, tokens,
then the catalogue's sections in order, each with its state model (S17). For every trace line of that platform whose
target is the section in hand, the facts carry the row (the trace's row ID is cited beside the source).

- **GNOME, KDE, Material, Windows:** mostly `published` (pinned sources), `read` for the reader's values; measured
  where the source is silent (animations by recording, D2).
- **macOS, iOS:** mostly `measured` by the probe (D3), the HIG and the API documentation where they state a value.
- **Gate per section:** `check_facts.py` green; the traces' rows of that section each cited by a fact or verdict;
  the catalogue corrected first where a fact contradicts it.

## M5 — Presets and palettes (after M2 and each lane's section)

The six platform presets rewritten in S11's format from the facts, one owner per preset, section by section as M4
delivers; the ten community palettes as mappings (R19) with their labelled picks, and the KDE ports.
- **Gate:** S19.1's build test (every preset and palette parses and validates); each section's preset part passes the
  gate's runs for that section (M6's test, run per section).

## M6 — The facts gate, complete

- The gate (S17) over every preset, widget, state-model combination, token, role and input; the perturbed run; the
  accent run with the schemes and accents S17 names (measured from Plasma, D2).
- **Gate:** `cargo test -p native-theme --test facts_gate` green on all six presets; no fact without a resolved key and
  no state-model combination without a fact.

## M7 — Connectors and showcases (from M2's stable API; closes after M6)

The gpui, iced, egui and egui-widgets connectors map the resolved model, each with its contract table (S21.3); the
three showcases rebuilt on the query; `AnimatedIcon`'s users moved (S19.3).
- **Gate:** each connector's contract test (every resolved field mapped or listed); the showcase parity gates; the
  screenshots regenerated and compared side by side with the platforms (the project's rule that showcases look as
  the platform does).

## M8 — Release 0.8.0

- CHANGELOG, READMEs, docs.rs build, `generate_assets_release.sh`, the provenance stamps, `check_release.sh` full;
  the v0.7.1 documents archived to `docs/archive/` with the spec checkers removed (S18).
- **Needs the maintainer:** the merge, the tag, the release and the publish.

## How the work runs

- An Opus session orchestrates; code tasks run in `implement` agents with a failing test or a compile as their gate;
  facts lanes run in agents briefed with the platform's sources, its traces and the facts format; a fresh reviewer per
  milestone. Agents read and search only the project, the source cache and the scratchpad (never the whole
  filesystem).
- Parallel by default: M2's tasks among themselves, M4's six lanes, M5's presets; builds and branch writes stay
  sequential.
- Progress is recorded in this document's checklist below and in the project memory.

## Checklist

- [ ] D1, D2, D3 decided
- [ ] M0 setup
- [ ] M1 vertical slice
- [ ] M2 model core
- [ ] M3 facts infrastructure
- [ ] M4 facts: GNOME · KDE · Material · Windows · macOS · iOS
- [ ] M5 presets and palettes
- [ ] M6 facts gate complete
- [ ] M7 connectors and showcases
- [ ] M8 release 0.8.0
