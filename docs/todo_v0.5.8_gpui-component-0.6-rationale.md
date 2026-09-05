# v0.5.8 — gpui connector on gpui-component 0.6: Rationale

Status: Pending
Crates: `connectors/native-theme-gpui` (primary), `native-theme` (two additive
API items, icon bundles, dependency refresh), workspace manifests
Companion specification:
[`todo_v0.5.8_gpui-component-0.6-spec.md`](todo_v0.5.8_gpui-component-0.6-spec.md)
Companion plan:
[`todo_v0.5.8_gpui-component-0.6-plan.md`](todo_v0.5.8_gpui-component-0.6-plan.md)

---

## 0 -- What this document is for

The specification says what v0.5.8 does. This document says why: the
evidence, every alternative weighed and rejected, why each derivation is a
derivation and not an invention, what was deliberately left out and what
would trigger revisiting it, and the ledger of errors caught while designing.
Citations follow the specification's convention and were verified on
2026-09-05.

The design went through three passes. The first produced a connector-only
plan with a documented "call `apply` again after theme changes" rule and a
substitution for radio metrics. The second, asked to find the best long-term
answer including changes to `native-theme` itself, replaced the rule with an
observer, turned the substitution into a cited platform fact, found that the
connector had been dropping the platform's text-scaling factor, and added a
dependency policy; it also mis-described ten bundled icons as non-Lucide. The
third established that those ten are Lucide icons stored under
gpui-component's names, reversed the Lucide-version decision on that basis,
and moved the bundle to Lucide's own names. A fourth pass, prompted by the
observation that a rationale half the length of its specification is
suspect, found that the reasoning behind the per-icon name choices, the API
shape, the test design and the task order existed only as conclusions; it is
written out in §2.21–§2.25, and §4 now covers every row of the
specification's limits table. A fifth pass, writing and reviewing the implementation
plan, compared the ten renamed files byte-for-byte, traced the observer's
activation order, and found that `apply_system_theme` installed a `ThemeConfig`
for one mode only (errors 39–45). A sixth pass re-verified every entry point
the plan names against the published sources and the repository, traced the
first `apply` through the effect queue, and found that an upstream rebuild in
the same update as the first `apply` escaped the observer (error 46) and that
gpui-component's registry observer can replace the connector's configs by name
(error 47). A seventh pass re-checked the point-in-time facts (crates.io,
toolchain, CI, the docs.rs metadata) and judged the icon decisions by glyph
*state* rather than by name, and found that the Lucide table served
`StarFill` with the hollow star and the freedesktop table served `Star` with
the filled one (errors 48–49). An eighth pass checked the showcase line
anchors, the connector's composite icon functions, the Linux platform crate's
native linkage and the docs.rs question from the other side: what the
three-target list would document. It documents nothing the default target does
not, and the claim that nothing enables gpui-pre's `windows-manifest` feature
was wrong (errors 50–51). A ninth pass asked what `Theme::change` reads
from a `ThemeConfig` that the connector never writes, and what the observer
tests can actually observe: the highlighter style was missing from the config,
and the tests' precondition (a preset whose splitter divider differs from its
border) is satisfied by no shipped preset (errors 52–54). Section 7 records
each correction.

---

## 1 -- The situation, argued from evidence

### 1.1 What changed upstream, and when

On 2026-09-03 longbridge released GPUI Kit 0.6.0 and renamed the repository.
Three things happened at once:

1. The **styled layer kept its name**: `gpui-component 0.6.0` is the crate
   the connector targets; `gpui-kit` is a facade over it, GPUI and
   `gpui-base`.
2. The **unstyled layer was split out** as `gpui-base`, which owns the
   scrollbar, the resize handle, the semantic tokens and its own `Theme`
   global that the styled theme projects into.
3. The **GPUI dependency changed identity**: gpui-component 0.6.0 depends on
   the `gpui-pre` package, not the `gpui` crate, whose last release is 0.2.2
   from 2025-10-22.

### 1.2 Why the connector is broken for new users today

Cargo treats `gpui-component 0.5.1` and `0.6.0`, and `gpui 0.2.2` and
`gpui-pre 0.3.x`, as different crates. An application on 0.6.0 that adds
today's connector gets two copies of each, and the `Theme` our `to_theme`
returns is not the `Theme` its widgets read: the assignment does not compile.
Every new gpui-component application since 2026-09-03 is on 0.6.0. That is
the whole argument for urgency.

### 1.3 What `gpui-pre` is

Not a Zed channel. The gpui-kit maintainer publishes it from a script that
stages Zed's `main`, audits licences, checks that gpui-kit still builds, and
uploads every GPUI workspace crate under a `gpui-pre-` prefix every other
Sunday, as `0.3.N` (`CONTRIBUTING.md:138-217`). Breaking changes from Zed's
`main` arrive as patch releases. Consequences: the connector must depend on
`gpui-pre` (that is the only way its `Hsla` and `Pixels` are gpui-component's
types), and it should keep its GPUI surface small, which it does (spec §4.2).

### 1.4 The measured cost

A throwaway crate outside the repository compiled the connector's sources
against the 0.6.0 stack: 14 library errors, all renames or removed fields;
15 missing icon variants in three exhaustive matches; a compiling library
after those; 31 first-pass showcase errors, six of them from GPUI itself.
The MSRV probes ran the same way. Nothing from the probe is committed.

### 1.5 What the connector was silently not doing

`from_system` and `to_gpui_theme` pass one boolean, `reduce_transparency`,
into `to_theme` (`src/lib.rs:237-246, 266-275`). `AccessibilityPreferences`
has four fields, and its documentation tells consumers to multiply font sizes
by `text_scaling_factor` (`native-theme/src/lib.rs:231-233`). A GNOME user
with "Large Text" got native colours and small text. This was found during
the second review and is the reason accessibility became a first-class input
(§2.12).

---

## 2 -- Options considered

### 2.1 When to move

| Option | Outcome |
|--------|---------|
| **Now, in v0.5.8** | **Chosen.** §1.2; v0.5.8 already holds an unreleased docs.rs fix; cost measured and modest. |
| Wait for 0.6.1 | Rejected. No endpoint in a rolling-snapshot ecosystem; a caret requirement unifies with any 0.6.x anyway. |
| Wait for an official `gpui 0.3` | Rejected. No date; gpui-component would still pin `gpui-pre`. |

### 2.2 Which crate to depend on

| Option | Outcome |
|--------|---------|
| **`gpui-component` + `gpui-base` + `gpui = { package = "gpui-pre" }`** | **Chosen.** Names what the library uses; `gpui-base` because gpui-component re-exports the scrollbar style structs but not the base `Theme`, `ScrollbarTheme` or `ResizableTheme` (`src/scroll/mod.rs:3-8`); the `gpui-pre` name is visible in the manifest. |
| `gpui-kit` facade for the library | Rejected. Hides the GPUI package name and pins versions automatically, but every upstream minor forces a connector release regardless; drags `gpui_platform` and application-level API into a library. **Chosen for the showcase**, which is an application and proves the interop users rely on. |
| Map onto `gpui-base` semantic tokens | Rejected. Eighteen colours and a few scale steps (gpui-base `src/theme_tokens.rs:11-194`) cannot carry 139 colours and per-widget geometry. |

### 2.3 Which version number

v0.5.8, because the workspace shares one version, v0.6.0 is the egui
milestone, and v0.5.7 already shipped breaking changes inside 0.5.x under a
"Breaking Changes" heading. Marked breaking for `native-theme-gpui`
consumers; no migration guide (pre-1.0 rule). Separate connector versioning
was rejected: `version.workspace = true` is a deliberate simplification the
release scripts assume.

### 2.4 The 28 button fields

| Option | Outcome |
|--------|---------|
| **Copy the semantic fields the variant used in 0.5.1** | **Chosen.** Reproduces 0.5.1's rendering (`0.5.1 src/button/button.rs:630-635, 924-929`); reads nothing new; matches the theme's semantics, where `danger_text_color` is the text on a danger surface. |
| Follow upstream's 0.6.0 default | Rejected. `apply_config` derives a **tinted** status button, background `X.mix_oklab(transparent, 0.2)`, text `X` (`schema.rs:845-856`). House style, not a platform's. |
| Leave to fallback | Rejected twice: transparent black on the direct `Theme` (`ThemeColor: Default`, `theme_color.rs:58`); the tint on the `Theme::change` path. |

### 2.5 How to deliver per-widget geometry

| Option | Outcome |
|--------|---------|
| **Pure `StyleRefinement` builders applied with `refine_style`** | **Chosen.** `StyleRefinement: Styled` (gpui-pre `src/style.rs:324`); `refine_style` on every `Styled` type through the blanket `StyledExt` impl (gpui-base `src/styled.rs:79-82, 205`), re-exported by gpui-component. One idiom, no wrappers, headless-testable through the refinement's public fields. Mirrors the iced v0.6.1 plan. |
| Extension trait with a method per widget | Rejected: duplicates `refine_style`. |
| Wrapper widgets | Rejected: owning a widget means owning behaviour and accessibility; the egui-widgets tier model exists for that question, and here the cheapest tier suffices for every widget in the table (82 widget files apply the caller's style after their own). |
| Upstream receivers first | Rejected for reachable widgets: a seam that exists is cheaper than a PR. Kept for the Tier U list (§4). |

### 2.6 What the builders take as input

| Option | Outcome |
|--------|---------|
| **`Native<'_> { resolved, accessibility }`, obtainable from the `NativeTheme` global or built by hand** | **Chosen.** Text sizes must scale by the accessibility factor (§2.12), so the builders need both inputs; a two-reference `Copy` view is the smallest honest signature. `cx.native_theme()` mirrors gpui-component's `cx.theme()` so applications stop threading `resolved` through every view. `Native::unscaled(&resolved)` serves the preset path and tests. |
| `&ResolvedTheme` only | Rejected: forces unscaled text or a hidden global read inside pure functions. |
| `(&ResolvedTheme, f32)` | Rejected: a bare factor loses the other preferences a future builder may need (high contrast). |

### 2.7 Re-application after upstream rebuilds the base layer

gpui-base's scrollbar accepts widths, inset, radius and minimum length
(`src/scrollbar.rs:597-646`), but `Theme::change`, `sync_system_appearance`
and `sync_base` rebuild the base theme with fixed styles (`theme/mod.rs:268-300`).

| Option | Outcome |
|--------|---------|
| **A global observer on `gpui_base::Theme` that re-applies the overrides** | **Chosen.** `App::observe_global` exists (gpui-pre `src/app.rs:2087-2099`); `set_global` and `global_mut` both queue one notification (`:2040-2043, 2062-2065`); pending notifications for one type are deduplicated (`:1662-1664`) and the mark is removed before observers run (`:1817-1821`). The observer's own write therefore produces exactly one further delivery, absorbed by a `reapplying` flag. No upstream code observes the base theme (the only upstream global observer watches `ThemeRegistry`, `theme/registry.rs:46`), so there is no competing writer. Correct mode colours come from the stored variant for the styled theme's current mode; when none is stored, geometry from the other variant and colours from the styled theme. Tested for both survival and termination. |
| Documented "call `apply` again" rule | Rejected (this was the first draft). It puts a call-order obligation on every application and fails silently when forgotten; the observer removes it at the cost of one flag. |
| Upstream PR first | Kept as the durable fix (`docs/todo.md`), not waited for. |

**Activation gap.** `observe_global` inserts the subscription inactive and
activates it through `defer` (gpui-pre `src/app.rs:2087-2099`);
`SubscriberSet::retain` skips inactive entries (`src/subscription.rs:125`).
The first `apply` therefore has a window: every base-theme write made in the
same update after it, before the flush, is delivered while the observer is
still inactive, and its values stand until the next rebuild. The usual
start-up sequence, `apply_system_theme` followed by
`Theme::sync_system_appearance`, does exactly that. Options: a documented
"call `apply` last" rule, rejected as the call-order obligation this section
removes; activating synchronously, impossible because `activate` is internal
to `observe_global`; one deferred re-write queued after the activation,
chosen (D38). Effects are FIFO, so the re-write runs after the activation and
after any same-update write; it costs one extra write of the overrides at
start-up; and it passes `mark = false`, so the active observer treats it like
any other rebuild and "only the observer sets `reapplying`" stays literally
true. Later `apply` calls need nothing, the observer is active.

**The registry observer.** Error 28's grep found gpui-component's one global
observer, on `ThemeRegistry`, and stopped at "it does not write the base
theme". It does more (`theme/registry.rs:41-51`): on a registry change it
replaces the styled theme's `light_theme` / `dark_theme` with the registry's
themes **of the same name**, then calls `Theme::change`, which the
connector's observer handles like any rebuild. The connector's configs carry
the native display name (`Breeze`, `Adwaita`, a preset's name); the default
registry holds `Default`, `Default Light` and `Default Dark`
(`theme/default-theme.json`); and the registry is notified only by
`ThemeRegistry::watch_dir` and `load_themes_from_str`. A collision therefore
needs an application that loads a same-named theme through the registry, and
that theme then wins for the colours while the base-layer geometry is still
restored. Suffixing the connector's config names was rejected: `theme_name()`
is what applications display. Recorded as a limit in the specification and
the README.

### 2.8 Which mode's colours the observer uses

`Theme::change(mode)` rebuilds the styled colours from the stored
`ThemeConfig`, so the scrollbar colours upstream projects are already the new
mode's. The observer prefers the stored `ResolvedTheme` for that mode
(exact, includes the distinct `thumb_active_color` the styled theme lacks)
and falls back to the styled theme's `scrollbar`, `scrollbar_thumb`,
`scrollbar_thumb_hover` with active = hover, which is exactly upstream's own
projection. Geometry is taken from whichever variant is stored; native
presets share geometry across variants.

### 2.9 Fifteen icons and the shape of the tables

| Option | Outcome |
|--------|---------|
| **Bundle the genuine files; tables return `Option`** | **Chosen.** The bundles were expanded in March 2026 to cover gpui-component's icon set (commits `48f67c5`, `05e9464`), and the new files come from the same upstream sources. `Option` because the project rule is "return None rather than substitute", and because the `&'static str` contract already forced a wrong glyph once: `StarOff` in Material was served by `star_border.svg`, a duplicate hollow star, since Material Symbols has no star-off icon (§2.15). The same review by glyph state later found `StarFill` served by Lucide's hollow `star` (§2.21, error 48). |
| Keep `&'static str` and add files | Rejected: perpetuates a contract that can only be kept by fabricating or mislabelling files whenever gpui adds an icon a set lacks. |
| Map to the nearest existing glyph | Rejected: a battery is not a search icon. |

### 2.10 Which Lucide version

| Option | Outcome |
|--------|---------|
| **Upgrade the bundle to 1.41.0, with `github.svg` pinned to 0.577.0** | **Chosen.** The first draft rejected the upgrade because 1.41.0 "lacks `github` and `trash-2`". On inspection `trash-2` was merely renamed: Lucide 1.x keeps `trash-2` as a deprecated alias of `trash` (`icons/trash.json`, `alias.duplicate`) and the `trash.svg` glyph at 1.41.0 is path-identical to the old `trash-2.svg`. Only `github` is gone, because Lucide removed all brand icons (commit `aa8f74eb`), and no similar icon exists in a set that no longer carries brands. Keeping that one file from the older tag is legitimate: it is a Lucide file under Lucide's ISC licence, and the manifest records the pin. Everything else in the bundle exists at 1.41.0, including the eight icons behind the renamed files. The user's policy is newest-or-a-reason, and the reason evaporated. |
| Stay at 0.577.0 | Rejected: the only argument was consistency within the set, and with one documented exception the set is at one revision anyway. |

### 2.11 Which Material revision

Refresh every Material file from upstream HEAD `0cbb08816df0` (2026-09-04)
and add the 14 new ones from the same commit, so the bundle is one revision.
Google revises Symbols glyphs in place and tags no releases; the sampled
files (`search`, `settings`, `star`) already match HEAD, and the only
rasterisation test asserts non-zero pixels. Leaving the existing files at
their March state while adding files from September would create exactly the
mixed-revision bundle §2.10 avoids.

### 2.12 Accessibility as an input

| Option | Outcome |
|--------|---------|
| **`to_theme(.., &AccessibilityPreferences)`; `from_preset(.., &prefs)`; text scaling through `Theme.font_size`; reduce-motion through `App`; builders scale text and grow control heights** | **Chosen.** One parameter carries all four preferences and any future one; `from_preset` takes it because accessibility is orthogonal to theme choice. `Root` sets rem to `theme.font_size` (`root.rs:579`), so scaling the two font sizes scales every rem-relative size in gpui-component the way the platform toolkit scales its text. |
| Keep the boolean, add a separate `scale` call | Rejected: two entry points for one concept, and `from_preset` would still ignore it. |
| Scale in `native-theme`'s resolver | Rejected: the struct's documentation assigns the multiplication to consumers, and a resolved theme is also used for widths and paddings that must not scale. |
| Scale paddings and widths too | Rejected: GNOME's text-scaling-factor scales text, not spacing; heights grow only through the control-height rule (§5.3). |

### 2.13 The `NativeTheme` global

Chosen: `apply` stores the resolved variants and preferences in a `Global`
with a `cx.native_theme()` accessor. Needed by the observer (it must know the
variants), and the idiom gpui-component itself uses for its theme. Rejected:
threading `&ResolvedTheme` through every view (the first draft), which no
gpui application does for its theme.

### 2.14 Dependency policy

Every external requirement is set to its latest stable release at release
time, with a recorded reason for any exception (spec §4.3). Exceptions
considered:

| Crate | Decision | Reason |
|-------|----------|--------|
| `gpui-pre` `0.3.3` | latest | caret, not exact: an exact pin in a library conflicts with any consumer whose other dependencies want a newer snapshot |
| `pollster` `1.0` | upgrade | 1.0.1 (2026-07-10); sole change `FutureExt` for `IntoFuture`; MSRV 1.69 |
| `syn` `3.0.5` | upgrade, gated | 3.0.0 (2026-07-18), five patch releases since; breaking changes are `*Modifiers` structs and `Type::BareFn` → `Type::FnPtr`; the derive crate touches `Type`, `Type::Path`, `Ident`, `ItemStruct`, `Fields::Named`, `Data::Struct`, `GenericArgument`, `PathArguments`, `Expr`, `Attribute`, `parse_str`, `Error`, none of them affected; gate is the derive crate's tests; stay on 2.0.119 only if the gate fails, with the reason recorded |
| `resvg` `0.48.1` | upgrade | maintained font stack; "may result in small rendering changes" is covered by the non-zero-pixel test and regenerated screenshots; predicted to raise the maximum declared floor to 1.89 via `font-types`; measured in Task 1 as unchanged (1.88.0), because `native-theme` uses resvg with `default-features = false` and resvg itself declares 1.85.0 (§2.16) |
| `windows` `0.62.2`, `image`, `objc2` family, `iced 0.14` | none | already latest |

### 2.15 Bundle provenance, manifest, generated tables

The second review compared every bundled Lucide file with Lucide 0.577.0 by
name and found ten files whose names are not Lucide names. A third comparison,
by path data against Lucide files of *other* names, showed that all ten are
Lucide icons stored under gpui-component's icon names: `close` and
`window-close` are `x`, `dash` and `window-minimize` are `minus`, `inspect` is
`scan`, `resize-corner` is `grip`, `sort-ascending` is
`arrow-up-narrow-wide`, `sort-descending` is `arrow-down-wide-narrow`,
`window-maximize` is `maximize`, `window-restore` is `minimize-2`; all eight
of those Lucide-named files are already in the bundle, byte-identical, so the
ten are plain duplicates. So the
bundle is entirely Lucide and entirely covered by `LICENSE-LUCIDE.txt`; what
was missing is a record of the renames, without which a refresh script cannot
update those files and a reader cannot verify them. Two Material files
(`warning`, `info`) match no current upstream variant, and `star_border.svg`
is a duplicate of `star.svg`. The name tables trail the directories
(99 of 103, 76 of 87).

| Option | Outcome |
|--------|---------|
| **Store every file under its upstream name; `SOURCES.toml` with one rule per set plus one per-file exception each; a coverage test; `scripts/refresh-icons.sh`; generated name tables** | **Chosen.** The project's rule is "only genuine source files"; with upstream names the manifest is one rule per set, the refresh script needs no translation table, the ten duplicates are deleted, and `lucide_name_for_gpui_icon` returns what its name promises. Nothing outside the connector uses the old names (repository grep); the role tables reference only `trash-2.svg`, at three lines. Generation removes the table drift and the hand-written coverage test. The rename is breaking for `LucideLoader::new` with the old names, which pre-1.0 is acceptable and is recorded in the changelog. |
| Keep gpui-component's names and record each rename in the manifest | Rejected: eleven per-file exceptions instead of one, ten duplicate files, a `lucide_name_for_gpui_icon` that keeps returning non-Lucide names, and a translation table the refresh script must maintain forever. |
| Leave the bundle undocumented | Rejected: adding 28 files to a bundle whose contents cannot be traced would deepen the problem, and the first refresh would silently skip the ten files. |
| Hand-written table arms plus an invariant test | Rejected: generation makes the invariant true by construction and removes 200 lines of `include_bytes!` arms. |

### 2.16 MSRV

Two measured floors. The workspace floor is re-measured after the dependency
refresh because the rule in commit `0319942` is "maximum declared
`rust-version` across the lock, then a check per member"; resvg 0.48 was
predicted to move that maximum to 1.89 (`font-types`); the measurement on
2026-09-06 found it unchanged at 1.88.0 (resvg declares 1.85.0 and is used
without default features), which is exactly why the rule says measure, not
predict. The connector gets its own per-crate
`rust-version` because the gpui-pre closure declares 1.92 and the workspace
must not inherit a floor `native-theme` does not need; the egui connector
design already uses the same split. Cargo's MSRV-aware resolver
(`resolver = "3"`) resolves against the lowest workspace floor, so the floor
is declared and `cargo update` re-run until the lock is stable; measuring on
a lock resolved against the old floor and stopping there would leave the
lock one step behind the rule.

### 2.17 Changes in `native-theme` considered

| Change | Decision | Reason |
|--------|----------|--------|
| `SystemTheme.layout` | **Add** | approved 2026-08-10; both sources already exist in the pipeline (`pipeline.rs:46, 120-126`); the geometry accessors are otherwise preset-only |
| `AccessibilityPreferences::from_system()` | **Add** | the preset path needs preferences without resolving a `SystemTheme`; an extraction of the KDE/GNOME reader code plus the uncached `detect::detect_reduced_motion()` (`detect.rs:662`; the cached `prefers_reduced_motion()` would freeze the fallback at its first answer, error 54), same coverage as today |
| `RadioTheme` | Rejected | platform-facts §2.5 defines radio metrics as the checkbox's with a circular indicator (`platform-facts.md:947, 969, 1210`); a second struct would restate the same facts |
| `ScrollbarTheme.thumb_radius` | Rejected for now | no platform fact; adding the field means per-platform research, recorded as a todo item; until then upstream's `radius` choice is mirrored |
| `LayoutTheme` resolved to `f32` on `ResolvedTheme` | Rejected | all 16 static presets define the four values and the 4 live presets inherit them from the reader merge, so the data would allow it for presets; but a user theme may omit `[layout]`, and platform-facts §2.20 records no layout defaults for Windows and none for macOS `container_margin` or KDE `section_gap` (`platform-facts.md:1427-1435`); a resolver fallback would invent them, whereas `None` states "the platform specifies nothing" |
| macOS / Windows accessibility readers | Rejected for now | new detection work per platform; research item |

### 2.18 A mode-switch helper

Considered: `native_theme_gpui::set_mode(is_dark, cx)` rebuilding the styled
theme from the stored variant. Not needed, on one condition found while
reviewing the plan (error 41): `apply` must install a `ThemeConfig` for
*every* stored variant, not only for the mode it installs. `to_theme` builds
the config of one mode; `apply_system_theme` stored both variants but, as
first specified, installed one config, so upstream's `Theme::change` to the
other mode would have applied the registry's default palette under natively
sized scrollbars. With both configs installed (D34), `Theme::change(mode)`
re-derives the colours from the connector's own config (exact for the 8-bit
native colours; the derived colours that carry alpha, `overlay`, `drag_border`,
`drop_target`, keep it because the connector writes `#rrggbbaa`, D36), and the observer
restores the base-layer geometry. Applications keep one idiom per concern:
`Theme::change` / `sync_system_appearance` for the mode,
`apply_accessibility` for preferences (§2.22).

One more field had to travel with the config for this to hold: `highlight`.
`Theme::apply_config` installs a config's highlighter style as
`highlight_theme` only when it is `Some` and otherwise keeps whatever the
previous mode left (`schema.rs:1066-1073`). The connector's config carried
`None`, because `to_theme` sets `highlight_theme` directly; with both configs
the connector's, a `Theme::change` to the other mode would have kept, say,
the dark highlighter under a light palette. The 0.5.7 connector never showed
this because the other mode's config was the registry default, which carries
a highlighter. The config now carries upstream's default style for its mode,
the same value `to_theme` installs (D41). It is upstream's default, not a
native value; the theme has no syntax colours.

### 2.19 Iterating `IconName`

`icon_named!` generates the enum and its `IconNamed` impl but no list of
variants (gpui-kit `crates/component-macros/src/lib.rs:114-190`), so the
connector keeps its hand-written `ALL_ICON_NAMES` with the count tripwire.
A generated `IconName::ALL` is added to the upstream PR list; it would
remove the list and the tripwire.

### 2.20 Testing without inspectable upstream types

`ScrollbarStyles` and its style structs have private fields and derive only
`Clone, Default` (gpui-base `src/scrollbar.rs:589, 616, 655`). The geometry
is computed into a connector-owned `ScrollbarGeometry` deriving `PartialEq`
and `Debug`, tested exhaustively; the conversion to `ScrollbarStyles` is one
setter per field. The `apply` test asserts what the base global exposes
(`mode()`, `resizable`) and, after `Theme::change`, that the observer
restored the resize-handle colour and the test returned.

### 2.21 Icon-name choices, one by one

Four rules produced the tables in specification §10.3 and §10.4:

1. Same **meaning** first, then same **appearance**; a glyph that means
   something else is not a match even if it looks alike.
2. Within a family (battery levels), stay in **one** upstream family so the
   icons read consistently side by side.
3. Label every row exact, close or approximate, as the existing table does,
   so a reader knows which rows to review visually.
4. Freedesktop names without the `-symbolic` suffix, because that is the
   convention of the existing table and the Breeze and Adwaita resolution
   tests already pass with it.

Lucide's three battery glyphs were read from their path data: `battery-low`
draws one bar, `battery-medium` two, `battery-full` three, all in a
three-bar body; `battery` is the empty body.

**Lucide.** Fourteen of the fifteen names exist at 1.41.0 under the same
names. `StarFill` does not: Lucide is a stroke set and ships `star`,
`star-off` and `star-half` but no filled star (`star-fill.svg` and
`star-filled.svg` are absent at the tag). gpui-kit's own `star-fill.svg` is
Lucide's `star` with `fill="currentColor"` added, an edit the bundle cannot
carry under the genuine-files rule. The first draft returned `star`, which
would draw the same hollow glyph for `Star` and `StarFill`, the two states of
a favourite toggle, the exact failure §2.9 rejects for `StarOff` in Material.
`StarFill` returns `None` in the Lucide set; an application whose asset
source includes gpui-kit's assets then falls back to gpui-kit's filled file.

**Material Symbols Outlined**

| Variant | Chosen | Why | Rejected |
|---------|--------|-----|----------|
| `Battery` | `battery_0_bar` | the empty state of the vertical family, which is what Lucide's empty outline means | `battery_unknown` (question mark), `battery_full` (a level the source does not show) |
| `BatteryCharging` | `battery_charging_full` | Material has charging icons only with a level; full is the neutral level and the canonical name; Lucide's glyph shows a bolt with no level | `battery_charging_20` etc. (invent a level) |
| `BatteryFull` | `battery_full` | exact | |
| `BatteryLow` | `battery_2_bar` | one of three bars is a third; two of six bars is a third | `battery_1_bar` (a sixth, the first draft's choice); `battery_low`, a separately named icon outside the bar family whose glyph was not inspected |
| `BatteryMedium` | `battery_4_bar` | two of three bars is two thirds; four of six is two thirds | `battery_3_bar` (half); the horizontal `battery_horiz_*` family, because Android's native battery icons are the vertical ones and mixing families breaks the side-by-side reading |
| `BatteryWarning` | `battery_alert` | exact meaning | |
| `Cpu` | `memory` | by meaning Material's `memory` is the processor/memory chip icon Android uses where Lucide uses `cpu`; Material has no icon named for a processor | `developer_board`, by name a circuit board rather than a chip |
| `MemoryStick` | `sd_card` (approximate) | Lucide draws a RAM module; Material has none; `memory` is taken by the chip and would collapse two gpui icons into one glyph; a removable memory medium is the nearest meaning | `memory_alt` exists but its glyph was not inspected; it is the alternative to check during implementation |
| `FileText` | `description` | Material's document icon; exact meaning; already bundled | |
| `HardDrive` | `hard_drive` | exact | |
| `Network` | `lan` | Lucide draws one node above three, a tree; a LAN topology is the nearest meaning | `hub`, by name a hub-and-spoke topology |

Only the names and upstream existence of the Material rows were verified;
glyph descriptions in this table are by meaning and name, which is why the
specification requires a visual check of every `close` and `approximate` row.
| `Pause`, `Play`, `RotateCw` | `pause`, `play_arrow`, `rotate_right` | exact | |
| `Star`, `StarFill`, `StarOff` | `star`, `star_fill1`, `None` | outlined and filled variants of one glyph; no star-off glyph exists | `star_border` (a duplicate of `star`) |

**Freedesktop, Breeze and Adwaita**

| Variant | Breeze | Adwaita | Why |
|---------|--------|---------|-----|
| `Battery` | `battery` | `battery` | both themes have a generic battery device icon |
| `BatteryCharging` | `battery-100-charging` | `battery-full-charging` | neither theme has a level-less charging icon; full is the neutral level, matching the Material choice |
| `BatteryFull` | `battery-100` | `battery-full` | exact |
| `BatteryLow` | `battery-020` | `battery-low` | Breeze names levels in tens; 20 reads as low without being the empty (`000`) or near-empty (`010`) state; Adwaita has the named state |
| `BatteryMedium` | `battery-050` | `battery-good` | Breeze's midpoint; among Adwaita's `low`, `good`, `full`, `good` is the medium state |
| `BatteryWarning` | `battery-010` (approximate) | `battery-caution` | Breeze has no alert icon; `010` is its near-empty level, and `battery-missing` means no battery present, a different message; Adwaita's `caution` is its warning state by name |
| `Cpu` | `cpu` | `computer` (approximate) | Breeze has a processor device icon; Adwaita has none, and the computer is the nearest device |
| `FileText` | `text-x-generic` | `text-x-generic` | the MIME icon for text documents, exact in both |
| `HardDrive` | `drive-harddisk` | `drive-harddisk` | exact in both |
| `MemoryStick` | `memory` | `media-flash` (approximate) | Breeze has a RAM-module device icon (`devices/64/memory.svg`), an exact match; the first draft's `media-flash-memory-stick` matched the *name* "memory stick" (Sony's flash card) and not the meaning; Adwaita has no RAM icon |
| `Network` | `network-workgroup` | `network-workgroup` | a group of computers; `network-wired` is a connection-status plug |
| `Pause`, `Play`, `RotateCw` | `media-playback-pause`, `media-playback-start`, `object-rotate-right` | same | exact |
| `StarFill` | `starred` | `starred` | the filled star, the "starred" state; exact |
| `Star` (changed) | `non-starred` | `non-starred` | both themes ship the pair `non-starred` / `starred` (Breeze `status/{16,22,24}`, Adwaita `symbolic/status`); the earlier claim that freedesktop has no outline/fill pair was wrong (error 49). The hollow `non-starred` is what Lucide's `star` draws, and a `Star` / `StarFill` pair must be distinguishable. `StarOff` keeps `non-starred`, the state it means: freedesktop names are semantic, so answering a state request with the state icon is the table's design, unlike a glyph set, and freedesktop has no slashed star |

### 2.22 API shape, detail by detail

- `apply(theme: GpuiTheme, resolved: &ResolvedTheme, prefs: &AccessibilityPreferences, cx)`
  takes the theme **by value** because it is moved into the styled global,
  never cloned; `resolved` **by reference** because callers keep using it,
  and `NativeTheme` clones it once per `apply`, never per frame
  (`ResolvedTheme: Clone`); `prefs` by reference for the same reason.
- `NativeTheme` stores **both** variants because `Theme::change` switches
  modes without going through the connector, and the observer must then
  find the variant of the mode upstream switched to. `apply_system_theme`
  has both from `SystemTheme`; `apply` stores the one it receives under the
  theme's mode. Each `apply` also (re)builds the `ThemeConfig` of the other
  stored variant under the same display name (§2.18, D34).
- `NativeTheme::resolved(&self, cx)` takes `cx` to read the styled theme's
  current mode; a mode field inside `NativeTheme` would go stale after
  `Theme::change`. There is no `Deref` to `ResolvedTheme` for the same
  reason.
- `Native<'a>` is a `Copy` view with public fields because the builders are
  pure functions: two references as one value keeps every signature short,
  and an application on the preset path can build it by hand.
  `Native::unscaled` uses a `static` default, which is possible because
  `AccessibilityPreferences` has public fields and no `#[non_exhaustive]`
  (`native-theme/src/lib.rs:229-235`).
- `apply_accessibility` is public because preferences change at runtime
  independently of the theme (a portal signal, a settings toggle). Because
  the variants are stored, it rebuilds the styled theme for the current mode
  with the new preferences and re-installs it through the `apply` path, so a
  runtime change of text scaling or transparency takes effect without the
  application keeping `resolved`; with nothing stored it only forwards
  reduce-motion and records the preferences (D35). The alternative, a
  documented "call `to_theme` and `apply` again", is the same call-order
  obligation §2.7 removed for the base layer.
- `base_layer::apply_overrides` is public so an application that writes the
  base theme itself can restore the native values; the observer calls the
  same function.
- `apply` ends with `cx.refresh_windows()`: a theme installed from a timer, a
  portal signal or a menu action must paint immediately in every window;
  upstream refreshes only the single window passed to `Theme::change` (D37).

### 2.23 Test design

- **Headless everywhere.** The builders are pure and are asserted through
  the public fields of `StyleRefinement`. The only `App`-dependent behaviour
  (globals, observer) uses `#[gpui::test]` with `TestAppContext`, which
  gpui-component itself uses for its base-projection tests
  (`theme/mod.rs:696-767`); it needs no window and runs in CI's Linux job.
  Windowed rendering tests were rejected: CI has no display, and the
  screenshot workflow already covers rendering on all three platforms.
- **`s ∈ {1.0, 1.5}`.** At 1.0 the control-height rule must return the
  theme's own value (first branch of the `max`); 1.5 was chosen because for
  the two presets the tests use it makes scaled text plus padding exceed the
  declared minimum (second branch). Checked against the preset files at
  96 dpi (`FontSize::Pt(v)` resolves to `v × dpi / 72`,
  `native-theme/src/model/font.rs:113-115`): kde-breeze has a 10 pt button
  font (13.33 px), line height 1.36, vertical padding 6 and minimum height 32
  (`kde-breeze.toml:34, 48, 90, 103`), so `ceil(13.33 × 1.36) + 12 = 31 < 32`
  at 1.0 and `ceil(20 × 1.36) + 12 = 40 > 32` at 1.5; adwaita has 11 pt
  (14.67 px), 1.21, 5 and 34 (`adwaita.toml:34, 48, 88, 100`), so `28 < 34`
  at 1.0 and `37 > 34` at 1.5. The test still asserts the branch condition it
  exercises instead of assuming it, so a preset whose minimum dominates at
  1.5 is caught and the factor raised.
- **The observer test asserts `resizable.active_handle`.** `ResizableTheme`
  is the only part of the base theme with readable, comparable values that
  upstream rewrites on `Theme::change`; the scrollbar styles are opaque
  (§2.20). Of its two slots only `active_handle` distinguishes the writer:
  every preset inherits `splitter.divider_color` from `defaults.border.color`
  (`inheritance-rules.toml:238`; no preset defines `[splitter]`), which is
  exactly what upstream writes into `handle`, so that slot reads the same
  either way; `active_handle` holds the connector's `splitter.hover_color`
  (the border colour again, via `:273`) against upstream's translucent
  `drag_border`, and the test asserts that they differ before relying on it.
  The first draft searched for a preset whose divider differs from its
  border, which none does, and would have panicked (error 53). The test
  returning is the termination proof: an observer loop would hang the test
  and fail CI's timeout, which is the intended failure mode. `Theme::change` is called with the *same* mode (with one stored variant, a different mode exercises the fallback, which has its own test) and twice, so a flag left set by the first delivery would fail the second assertion (error 40). A third `#[gpui::test]` runs `apply` and `Theme::change` inside one `cx.update`, the only way to reach the window in which the observer is installed but not yet active (§2.7, D38).
- **`no_theme_color_field_is_left_at_default`.** The `size_of` tripwire
  counts fields but cannot see an unassigned one; comparing every field with
  the zero `Hsla` catches exactly the failure 0.6.0 introduced, twenty-eight
  new fields defaulting to transparent black.
- **Icon tests.** Every `Some` must resolve through
  `bundled_icon_to_image_source` so a table entry cannot name a file that is
  not bundled; every `None` must be in an allow-list with a reason so a
  missing mapping cannot hide as an intentional one; the manifest coverage
  test makes an untraceable file a test failure.

### 2.24 Task order

The order in specification §15 follows dependencies and the cost of
re-running gates:

1. Dependency refresh first, so every later gate (tests, MSRV, CI) runs
   once on the final lock instead of twice.
2. `native-theme` additions before the connector, because the connector
   consumes `AccessibilityPreferences::from_system` and `SystemTheme.layout`.
3. Bundles before the connector's manifest switch: the tables are tested
   against the bundles, and while the connector is still on 0.5.1 the bundle
   change reddens only a few old-name icon tests, whereas switching the
   manifest first would leave the library uncompilable until the tables are
   done. For the same reason the manifest, the mechanical first pass and the
   icon tables form one step whose shared gate is the compile.
4. Colours before geometry, because both need a compiling library and the
   colour work restores the tripwire the geometry tests also rely on.
5. `base_layer` before `apply`, because `apply` calls it; `geometry` after
   both, because the showcase needs all three.
6. Showcase last among the code steps, because it exercises everything.
7. MSRV measurement after the code is final, because the floor depends on
   the code.
8. Docs before release, release only on explicit approval, per the project's
   checkpoint rule.

### 2.25 docs.rs targets for the connector

Commit `ce0fdee` (unreleased, in the `[0.5.8]` changelog) added a three-target
`[package.metadata.docs.rs]` list to all three published crates so that
platform-gated items stay visible after docs.rs's 2026-05-01 change to a
single default target. For `native-theme` and `native-theme-iced` that is
right: they have macOS- and Windows-only public items. The connector does not:
its only `cfg(target_os)` gates on public items are Linux (`lib.rs:92-93`
re-exports `LinuxDesktop`; `icons.rs:361` gates
`freedesktop_name_for_gpui_icon`), and the default target *is* Linux, so the
macOS and Windows pages would be the Linux page minus two items.

What the two extra targets would cost: the first cross-target docs build of
the GPUI stack anywhere. gpui-pre 0.3.3, gpui-component 0.6.0 and gpui-kit
0.6.0 declare no docs.rs targets and each was built for
`x86_64-unknown-linux-gnu` only (their `/builds` pages). A Windows target
runs gpui-pre's build script with `windows-manifest` on, because
gpui-component depends on gpui-pre with default features (gpui-component
`Cargo.toml:266-267`; gpui-pre `Cargo.toml:59-64`), and `embed-resource` on a
Linux host then needs `llvm-rc` (embed-resource 3.0.8
`src/non_windows.rs:73-86`, `manifest_required` fails on `NotAttempted`);
docs.rs's image ships `llvm` (`crates-build-env` `linux/packages.txt:1045`),
so it would probably pass, and a macOS target would fetch and build the macOS
platform crates, which this machine has never resolved. Probably passing is
not a reason to take on a risk that buys nothing.

| Option | Outcome |
|--------|---------|
| **Keep `all-features = true`, drop the `targets` list on the connector** | **Chosen** (D40). The default target shows every public item; the build is the one 0.5.7 and the upstream stack already do. Reverses one third of `ce0fdee`; the unreleased changelog line is amended. |
| Keep the three targets and run each cross-target docs build locally before the tag (the sixth pass's D40) | Rejected. It guards a build that documents nothing extra; the local check's value also depended on the host mirroring docs.rs's tool set (`llvm-rc`), which is a second thing to get right. |
| Keep the three targets, no check | Rejected: an untested cross-target build on a published crate's docs. |

### 2.26 Why a test and not a comment for the colour fields

Considered: documenting "assign every new field" in `colors.rs`. Rejected:
the failure is silent at compile time (`ThemeColor: Default`) and invisible
in most screenshots (transparent black over a dark background is dark). Only
a test that compares every field with the zero value fails at the right
moment.

---

## 3 -- The decision record

| # | Decision | One-line reason |
|---|----------|-----------------|
| D1 | Move to gpui-component 0.6 now, in v0.5.8 | type-incompatible with all new applications since 2026-09-03 |
| D2 | Library on `gpui-component` + `gpui-base` + `gpui-pre` | names what it uses; base types not re-exported |
| D3 | Showcase on `gpui-kit` | proves interop |
| D4 | 0.5.8, Breaking entry, no migration guide | workspace version, roadmap, pre-1.0 |
| D5 | `button_*` copy the 0.5.1 semantic sources | native surfaces are solid |
| D6 | `table_foot*` mirror `table_head*` | no footer field |
| D7 | `status_bar*` from `StatusBarTheme` | direct source |
| D8 | `to_theme` / `from_preset` take `&AccessibilityPreferences`; text scaling via `Theme.font_size`; reduce-motion via `App` | §1.5, §2.12 |
| D9 | `NativeTheme` global with `cx.native_theme()`; builders take `Native<'_>` | §2.6, §2.13 |
| D10 | Observer on `gpui_base::Theme` re-applies overrides; `reapplying` flag | §2.7 |
| D11 | Thumb inset `(groove − thumb) / 2`, clamped | §5.1 |
| D12 | Thumb radius and active track border mirror upstream's choices | §5.2 |
| D13 | Resize-handle colours from `SplitterTheme` | direct sources for upstream's two slots |
| D14 | `focus_ring = focus_ring_width > 0` | only the flag has a receiver |
| D15 | Geometry as `StyleRefinement` builders, `Size` helpers, builder helpers | cheapest tier that works |
| D16 | Control height `max(theme height, ceil(text × s × line_height) + 2 × pad_v)` | §5.3 |
| D17 | Progress through the refinement seam, not `Size::Size` | carries the radius exactly (`progress.rs:92-94, 137, 147`) |
| D18 | `radio` uses checkbox metrics as a platform fact | platform-facts §2.5 |
| D19 | Tier U items deferred to v0.6.2 as upstream PRs | §4 |
| D20 | Icon tables return `Option`; bundles gain genuine files | §2.9 |
| D21 | Lucide bundle upgraded to 1.41.0; `github.svg` pinned to 0.577.0 | §2.10 |
| D22 | Material bundle refreshed to HEAD `0cbb08816df0`; `StarOff` → `None`; duplicate `star_border.svg` removed | §2.11; no star-off glyph exists in Material Symbols |
| D23 | Bundle files under upstream names (ten duplicate files deleted, `trash-2` → `trash`); `SOURCES.toml` with one exception per set; refresh script; generated tables | §2.15 |
| D24 | Dependency refresh with recorded exceptions | §2.14 |
| D25 | Two measured MSRV floors | §2.16 |
| D26 | `SystemTheme.layout` and `AccessibilityPreferences::from_system()` added | §2.17 |
| D27 | No `RadioTheme`, no `thumb_radius`, `LayoutTheme` stays `Option` | §2.17 |
| D28 | Publish soft gates removed if the workspace check passes | G11's cause is gone (naga 29.0.4) |
| D29 | `apply` initialises gpui-component when its global is absent; observer and accessors use `try_global` | the styled accessors panic on a missing global (`theme/mod.rs:172-176`; gpui-pre `app.rs:2024-2028, 2040-2047`); `init` is called at most once because it is only called when the styled global does not yet exist, and a second call would install a second registry observer (`theme/registry.rs:41-51`); the project forbids panic paths |
| D30 | No mode-switch helper | §2.18 |
| D31 | `ALL_ICON_NAMES` stays hand-written; `IconName::ALL` proposed upstream | §2.19 |
| D32 | Material battery levels on the vertical bar family: `battery_0_bar`, `battery_2_bar`, `battery_4_bar`, `battery_full` | Lucide's one/two/three of three bars map to two/four/six of six; one family, side-by-side consistency (§2.21) |
| D33 | `MemoryStick` → Breeze `memory` | an exact RAM-module icon exists; the flash-card icon matched only the name (§2.21) |
| D34 | `apply` installs a `ThemeConfig` for every stored variant | `Theme::change` to the other mode must reproduce the native palette, not the registry default (§2.18, error 41) |
| D35 | `apply_accessibility` rebuilds the styled theme from the stored variant | runtime preference changes take effect without the application keeping `resolved` (§2.22) |
| D36 | Config hex keeps alpha: `#rrggbbaa` when below 1 | gpui parses eight digits; without it `overlay`, `drag_border`, `drop_target` turn opaque after `Theme::change` (error 44) |
| D37 | `apply` ends with `App::refresh_windows` | a change from a timer, portal signal or menu action must paint at once in every window; upstream refreshes only the window passed to `Theme::change` (error 45) |
| D38 | `install_observer_once` queues one deferred re-write of the base overrides | the subscription activates at the end of the flush; a rebuild in the same update as the first `apply` would otherwise stand until the next one (§2.7, error 46) |
| D39 | Lucide `StarFill` → `None`; freedesktop `Star` → `non-starred`, `StarFill` → `starred` | one glyph must not stand for two states (§2.21, errors 48–49) |
| D40 | Connector docs.rs metadata: `all-features = true`, no `targets` list | the crate's platform-gated public items are all Linux-gated and appear on the default target; two more targets would add nothing but the first cross-target docs build of the GPUI stack (§2.25, errors 50–51) |
| D41 | `to_theme_config` carries upstream's default highlighter style for its mode | `apply_config` keeps the previous `highlight_theme` when the config has none; with both configs the connector's, a mode switch would keep the wrong highlighter (§2.18, error 52) |

---

## 4 -- Why each Tier U item is unreachable

| Item | Hardcoded at (gpui-component 0.6.0) | Why the caller cannot reach it | Upstream fix |
|------|------|------|------|
| Checkbox indicator | `checkbox.rs:195-199` | inner; `Size::Size` hits the wildcard | honour `Size::Size` or a token |
| Radio indicator | `radio.rs:216-220` | same | same |
| Switch track/thumb | `switch.rs:136-146`; wrapper `:168` | inner | same |
| Slider track/thumb | `slider.rs:218, 288-289`; root `:270` | inner | tokens |
| Tab geometry | `tab/tab.rs:26-76` by `Size` | `impl Styled` at `:606`, never applied | apply the refinement in `render` |
| Separator thickness | `separator.rs:81-82` | absolutely positioned inner line; outer refined at `:137` | thickness builder |
| Resize-handle width | gpui-base `resize_handle.rs:12` | constant | field on `ResizableTheme` |
| Button icon gap; button label size ≠ body | `button.rs:658-666` | inner content row sets `text_base` and the gap | expose gap; label size independent of rem |
| Input padding | not on `input.rs:566-587` | owned by the editor element | padding builder |
| Popup-menu rows | `popup_menu.rs:749` | constructed internally | item style hook |
| Button text tooltip | `button.rs:360` | constructed from a string | accept a `Tooltip` |
| Select/Combobox arrow; accordion arrow | inner; `accordion.rs:281-282` | sized by `Size` | tokens |
| Shadows | `Theme.shadow` read only in `button.rs`; `shadow_*()` in `sheet.rs`, `tooltip.rs`, `input/popovers/*.rs`, `tab/tab.rs`, `table/column.rs`; `tokens.shadow` used only in `theme/` | no guard | route through tokens |
| Segmented control | segmented `TabBar` composed of `Tab`s | inherits the `Tab` problem above | same as Tab |
| Toolbar metrics | — | gpui-component has no toolbar widget; the theme's `toolbar` struct has nothing to land on | a toolbar component |
| Spinner stroke width | the stroke is inside the Lucide/Material SVG | `Spinner` sizes an `Icon`; stroke is not a parameter | a stroke token, or a rasterised spinner |
| Dialog minimum width | `dialog.rs:381-393` | only fixed-width and maximum-width builders | a `min_w` builder |
| Title-bar height | `title_bar.rs:334` | the receiver exists; the theme has no title-bar height field | a native-theme field, if platform facts support one |
| Typography scale | `theme/mod.rs:353-360, 440-448` | tokens are re-derived from `font_size` on every sync and drive only the markdown `TextView`; the theme's `TextScale` has four named entries, not five steps | stored custom typography on the styled theme |
| Spacing scale | — | no component reads `tokens.spacing` | consume the tokens |
| Focus-ring width and offset | `styled.rs:188-215` | computed from the element's own border widths | a ring-width token |
| High contrast | — | no receiver in GPUI or gpui-component | a contrast mode upstream |
| Scrollbar thumb radius | `theme/mod.rs:284-293` | upstream's own choice is mirrored; the theme has no field and platform-facts no row | facts first, then a field |
| Body font weight; global line height | `root.rs:579, 591` | `Root` sets family and size only; `Theme` has no weight or line-height | fields on the styled theme |
| Button icon size | `button.rs:541-542, 579-583` | derived from the button's `Size`, on the inner content | honour an explicit icon size |
| Disabled opacity | `button.rs:716-718` | disabled controls are styled through colours, not an opacity | a disabled-opacity token |
| Popup-menu padding and radius | `popup_menu.rs:401-408, 1112` | `PopupMenu` implements no `Styled`; only width builders exist | `Styled` on `PopupMenu` |
| `DataTable` header font, table rows | — | not examined in this milestone; rows are inner elements | examine in v0.6.2 |
| Sidebar width and padding | `sidebar/mod.rs:27, 432-479` | no theme field for width; padding on inner elements | padding builder |
| Notification, sheet, tile, list highlight, motion durations | — | the theme has no source for any of them | native-theme fields, if platform facts exist |
| macOS / Windows text scaling and high contrast | `pipeline.rs:592, 1004, …` | no reader fills them today; `from_system()` is an extraction, not new detection | reader research |

---

## 5 -- Derivations, and why they are not inventions

A derivation is allowed when the receiver needs a value the theme does not
carry, the formula is written down, and the inputs are theme or preference
values or an explicit mirror of upstream's own choice.

### 5.1 Scrollbar thumb inset

gpui-base anchors the thumb fill `inset` from the track's outer edge, makes
it `thumb_width` wide (`scrollbar.rs:1391-1400`) and trims it by `inset` at
both ends (`:1374`). Platforms draw the thumb centred in the groove:
`inset = (groove − thumb) / 2`, clamped at zero for a thumb as wide as its
groove. The formula is the geometric consequence of two theme values and
upstream's anchoring; upstream's own default (`16`, `6`, `4`) is not centred,
so mirroring it was not an option.

### 5.2 Thumb radius and active track border

The theme has neither. Upstream's projection uses the styled theme's `radius`
and `border` colour (`theme/mod.rs:281-293`); the connector mirrors both with
`defaults.border.corner_radius` and `defaults.border.color`. Nothing upstream
did not already do. platform-facts has no scrollbar radius; the todo item
asks for one so a future release can replace the mirror with a fact.

### 5.3 Control height under text scaling

Toolkits size a control as the larger of its minimum height and its text
height plus vertical margins. The rule
`max(theme height, ceil(font.size × s × defaults.line_height) + 2 × padding_vertical)`
uses only theme values and the preference. At `s = 1` a platform's declared
height already accommodates its text, so the rule returns the theme's own
value; it changes anything only when scaled text would otherwise be clipped
by gpui's fixed heights. Growth was chosen over `min_h` because a refinement
cannot remove upstream's fixed `h_8` (a refinement only overrides fields it
sets), so a fixed height must be written, and it must be large enough.

### 5.4 The 28 button fields, `table_foot`, `focus_ring`

Copies of values the connector already computes for the same variant (§2.4);
footer mirrors header because the theme has no footer and transparent black
is not a colour; a zero-width ring is not drawn, so `width > 0` is the only
reading that does not ignore the theme (in the presets inspected the width is
2.0, so today the flag is always on).

### 5.5 Text scaling of rem

Not a derivation but a consequence worth stating: because `Root` sets rem to
`theme.font_size` (`root.rs:579`), every rem-relative upstream size scales
with the factor. This is what the platform does to its own text, and it is
the reason the connector scales only the two font sizes and not its absolute
overrides.

---

## 6 -- What was deliberately not done, and the trigger to revisit

| Not done | Trigger |
|----------|---------|
| A Lucide `github` replacement | Lucide reinstates brand icons, or the maintainer chooses to return `None` for `Github` in the Lucide set |
| `ScrollbarTheme.thumb_radius` | platform-facts gains scrollbar radius rows |
| macOS / Windows text scaling and high contrast in `from_system()` | reader research recorded in `docs/todo.md` |
| Typography tokens from `TextScale` | upstream stores custom typography on the styled theme, or `TextScale` maps one-to-one |
| Spacing tokens from `LayoutTheme` | any gpui-component widget reads `tokens.spacing` |
| Shadow tokens from `shadow_enabled` | upstream routes component shadows through `tokens.shadow` or `Theme.shadow` |
| Title-bar height | native-theme gains a title-bar height field |
| Wrapper widgets for Tier U items | the corresponding upstream PR is refused |
| `RadioTheme` | platform-facts records radio metrics that differ from the checkbox's |
| `syn 3` if its gate fails | next derive-crate change |

---

## 7 -- Errors found and corrected during design

Kept so the reasoning can be audited. Items 1–17 are from the first pass,
18–26 from the second, 27–33 from the third, 34–38 from the fourth, 39–45
from the implementation-plan pass, 46–47 from the sixth pass, 48–49 from the
seventh, 50–51 from the eighth, 52–54 from the ninth.

1. **First field diff was wrong** (46 fields from a bad `awk` range); corrected by diffing the two upstream structs: 108 → 139.
2. **`grep` undercounted 0.5.1 fields as 103**; the `size_of` tripwire's 108 is authoritative.
3. **Separator thickness assumed reachable**; the line is an inner absolute element (`separator.rs:79-84`).
4. **`Tab` assumed reachable**; its `Styled` impl (`tab.rs:606`) is never applied.
5. **`Size::Size` assumed to set Button height**; it scales padding only (`button.rs:588`).
6. **`ThemeColor::default()` assumed to be upstream's palette**; it is all zeros (`theme_color.rs:58`).
7. **Bundles assumed to contain the new Lucide icons**; only `star` was present.
8. **`ScrollbarStyles` assumed inspectable**; private fields, `Clone, Default` only.
9. **gpui-component assumed to re-export `gpui`**; gpui-kit does.
10. **`set_reduce_motion` assumed platform-fed**; it is application-set.
11. **Upstream doc comments and code disagree** on status button text fallbacks (`schema.rs:935` vs the field docs); recorded so nobody "fixes" the connector to match the comment.
12. **The session's earlier recommendation kept geometry for v0.6.2**; the maintainer pulled it into v0.5.8.
13. **Workspace MSRV assumed to carry over**; five crates under `gpui-pre` refuse 1.88.0.
14. **Freedesktop names were first guessed**; every name was then verified in Breeze and Adwaita.
15. **Layout accessors specified from `ResolvedTheme`**; `LayoutTheme` is on `Theme` (`model/mod.rs:266`), hence `SystemTheme.layout` and `&LayoutTheme` accessors.
16. **Expander listed as having no analogue**; `AccordionItem::title_style` (`accordion.rs:219, 300-306`) is a builder seam.
17. **Showcase arity errors misattributed**; they are `AppMenuBar::new`, `Progress::new`, `Sidebar::left`.
18. **The re-apply rule was accepted as a limitation.** gpui's `observe_global` with deduplicated notifications (`app.rs:1662-1664, 1817-1821, 2087-2099`) makes automatic re-application safe; the rule is gone.
19. **Text scaling was not in scope.** The connector passed only `reduce_transparency` (`lib.rs:237-246`); `text_scaling_factor` was dropped. Now a first-class input.
20. **Button text size was specified as a refinement.** The label size is set on an inner element (`button.rs:658-666`, `sizing.rs:319-324`); at Medium it equals rem, i.e. `font_size`, so the native size is honoured only when button and body sizes coincide. Recorded as Tier U for the other case.
21. **Input text size was marked "verify".** `input_text_size` is applied on the root before the refinement (`input.rs:572, 587`); reachable.
22. **Progress used `Size::Size`.** The refinement seam is better: Progress copies the caller's radii to its fill (`progress.rs:92-94, 137, 147`), so height and radius are both exact.
23. **Radio was a substitution.** platform-facts §2.5 defines radio metrics as the checkbox's; it is a cited fact.
24. **Ten bundled files were declared "not Lucide" and one "a gpui-component copy".** They were compared by *name* against Lucide and by content against gpui-component's assets. Compared by content against Lucide files of other names, all ten are Lucide icons stored under gpui-component's names (§2.15); `close.svg` matched gpui-component's file only because that file is itself Lucide's `x`. The bundle is entirely Lucide. What the milestone fixes is the missing record of the renames.
25. **`star_fill` was a made-up file name.** `star_fill1` follows upstream's file stem.
26. **Dependencies were taken as given.** Every workspace requirement was checked against crates.io; four had crossed a major or minor boundary and each now has a decision.
27. **Presets with `[layout]` were counted as 6 of 20.** A listing cut at thirty lines hid the rest: 16 of 20 define all four keys, the four `*-live` presets none. The count was re-taken without truncation, and the resolved-`f32` option was re-examined on the corrected data before being rejected on the platform-facts gaps (§2.17).
28. **The observer's single-writer assumption was asserted from the wrong grep.** The first grep looked for window-appearance observers; the second looked for `observe_global` and found one, on `ThemeRegistry` (`theme/registry.rs:46`), which does not write the base theme. The assumption holds and is now cited correctly.
29. **`apply` was specified as if the styled global always exists.** `Theme::global`/`global_mut` are thin wrappers over GPUI's panicking accessors. The specification now requires the `has_global` check with `init`, and `try_global` in the observer and accessors.
30. **Lucide 1.41.0 was said to have removed two icons the connector needs.** `trash-2` was renamed to `trash` with an identical glyph and kept as a deprecated alias; only `github` was removed, with the rest of Lucide's brand icons. The bundle now upgrades to 1.41.0 with `github.svg` pinned (§2.10).
31. **"Material `Star`" was posed as an open question.** The real finding is that `star_border.svg` is a byte-different duplicate of `star.svg` (same path data) and was backing `IconName::StarOff`, a slashed star, with a plain hollow star. Material Symbols has no star-off glyph; `StarOff` returns `None` in the Material set and the duplicate is removed. `Star` stays on the hollow `star`, `StarFill` gets the new `star_fill1`.
32. **The ten renamed files were first to be "recorded, not changed".** Once it was verified that nothing outside the connector uses the gpui-component-style names and that the role tables touch only `trash-2.svg`, storing the files under Lucide's names became the cleaner design: one manifest exception instead of eleven, ten duplicates gone, and a table that returns real Lucide names (§2.15).
33. **Two spec sentences still described the files as non-Lucide after the correction.** The full re-read of both documents caught them (§10.1 and the rationale's §0 and §2.9); each now states the corrected finding.
34. **The rationale recorded conclusions where it owed reasoning.** Icon-name choices, API shape, test design and task order appeared only as results in the specification. Prompted by the maintainer's observation about the two documents' relative size, §2.21–§2.25 were written and §4 extended to every limits row. The size ratio itself was not the defect (the project's egui pair is spec 5150 lines to rationale 1837); the missing arguments were.
35. **`BatteryLow` was mapped to `battery_1_bar`.** Reading Lucide's path data shows one of three bars, a third; on Material's six-bar family that is `battery_2_bar`. `BatteryMedium`'s `battery_4_bar` was right for the wrong reason and is now labelled close, not approximate.
36. **`MemoryStick` was mapped to Breeze `media-flash-memory-stick` on a name match.** Breeze has `devices/64/memory.svg`, a RAM module, which is what Lucide draws. Corrected to `memory`, exact.
37. **The specification said the connector maps `Delete` to `trash`.** It maps `Delete` to Lucide's `delete` icon (`icons.rs:167`) and never used `trash-2`; only native-theme's role tables do. The `trash-2` → `trash` rename is therefore a native-theme change alone, and the sentence was corrected.
38. **Glyph descriptions in the icon reasoning were written from memory.** Names and upstream existence were verified; glyph shapes for the Material `close` rows, `hub`, `developer_board`, `battery_low`, Breeze's `battery-missing` and `rating` were not. The descriptions were rewritten to say what was verified, and the specification now requires a visual check of every `close` row, not only the `approximate` ones.
39. **The ten gpui-named files were described as renames with two collapsing pairs, and the Lucide file count after the milestone as 115.** Compared byte-for-byte while writing the implementation plan (2026-09-05): the bundle already holds all eight target files (`x`, `minus`, `scan`, `grip`, `arrow-up-narrow-wide`, `arrow-down-wide-narrow`, `maximize`, `minimize-2`), identical to the ten. They are deletions, not renames; the count becomes 107; and the four name-table gaps (99 of 103) are exactly the four canonical files the hand-written table never listed.
40. **`apply` was specified to set the `reapplying` flag before its own write.** `App::observe_global` defers the subscription's activation to the end of the current effect flush (gpui-pre `src/app.rs:2087-2099`, `SubscriberSet::insert`), and effects run in order, so the notification `apply` queues is delivered before the observer is active; the flag would never be cleared and would swallow the next upstream rebuild. The flag is now set only by the observer's own writes; `apply` accepts one idempotent re-application on the first delivery. The observer test runs `Theme::change` twice to catch a stale flag.
41. **`apply_system_theme` stored both variants but installed one `ThemeConfig`.** Upstream's `Theme::change` to the other mode would have applied the registry's default palette while the observer restored native scrollbar geometry under it; §2.18 claimed an exactness that held for one mode only. Found while reviewing the plan's `apply_inner`; `apply` now installs the other stored variant's config too, and a test switches modes and compares the palette hex-for-hex (D34).
42. **The plan's first `apply_inner` still set `reapplying` through the shared re-apply helper**, reintroducing the hazard error 40 describes. The helper gained a `mark` parameter; `apply` passes `false`, the observer `true`. The observer test's second `Theme::change` would have caught it.
43. **The stale-reference table (§13.7) missed three MSRV mentions.** The root `README.md` badge, `CONTRIBUTING.md` and the MSRV CI item in `docs/todo.md` all say `1.88.0`; added to §13.7 and to the docs task.
44. **The config hex export dropped alpha.** `hsla_to_hex` wrote `#rrggbb` only, so `overlay` (alpha 0.4/0.5), `drag_border` (0.65) and `drop_target` (0.2) became opaque after `Theme::change`, contradicting D34's "reproduces the native palette". gpui's `Rgba::try_from` accepts `#rrggbbaa` (gpui-pre 0.3.3 `src/color.rs:224-262`) and gpui-component's `try_parse_color` delegates to it for `#` strings (`src/theme/color.rs:677-680`); the connector now writes eight digits when alpha is below one (D36). `ThemeConfig.radius` being `usize` is the one remaining round-trip loss, recorded in §14.
45. **`apply` did not repaint.** Upstream's `Theme::change(mode, Some(window), cx)` refreshes the window it is given; the connector's `apply` has no window and would have relied on the next input event. `App::refresh_windows` is public (gpui-pre 0.3.3 `src/app.rs:1074`); `apply` calls it last (D37).
46. **The first `apply` was traced only against the notifications it queued itself.** Error 40 established that the observer misses those; it did not ask what else the same update might write. A `Theme::sync_system_appearance` (or any `Theme::change`) right after the first `apply`, the usual start-up sequence, rebuilds the base theme while the observer is still inactive, and the native scrollbar geometry and handle colours would stand replaced until the next rebuild. `install_observer_once` now queues one deferred re-write after the activation (D38), and a `#[gpui::test]` runs `apply` and `Theme::change` in one update.
47. **The single-writer check stopped at the base theme.** The registry observer (`theme/registry.rs:41-51`) does not write `gpui_base::Theme`, but it replaces the styled theme's configs with same-named registry themes before calling `Theme::change`. With the default registry (`Default`, `Default Light`, `Default Dark`) nothing collides; an application loading a same-named theme through `ThemeRegistry` would replace the connector's colours. Recorded as a limit (§2.7), in the specification (§3.3) and in the README task.
48. **Lucide `StarFill` was served with the hollow `star`.** The row was justified by name ("gpui-kit's own file is Lucide `star` filled") without asking what the connector can draw: a bundled Lucide file has no fill, so `Star` and `StarFill` would have rendered identically, the failure §2.9 rejects for `StarOff`. Lucide 1.41.0 has no filled star (raw probes of the tag). `StarFill` returns `None` in the Lucide set (D39).
49. **The freedesktop star reasoning claimed the themes have no outline/fill pair.** Both Breeze and Adwaita ship `non-starred` and `starred` (and `semi-starred`); the existing table mapped `Star`, Lucide's hollow star, to `starred`, the filled one, and the first draft mapped `StarFill` to the same name. `Star` now maps to `non-starred` and `StarFill` to `starred` (D39); `StarOff` keeps `non-starred`.
50. **The sixth pass claimed that nothing in the stack enables gpui-pre's `windows-manifest` feature.** The grep looked for the feature name in the three downstream manifests and found none, but gpui-component, gpui-base and gpui-kit all depend on gpui-pre *with default features*, and `windows-manifest` is a default (gpui-pre `Cargo.toml:59-64`). For a Windows target the build script runs `embed-resource`, which on a Linux host needs `llvm-rc`. docs.rs's image has it, so the predicted outcome (pass) was probably right for the wrong reason; the corrected facts are in §2.25 and spec §1.3.
51. **The three-target docs.rs list was inherited without asking what it documents.** The connector's only platform-gated public items are Linux-gated and the default target is Linux, so the macOS and Windows pages would show strictly less. The sixth pass added a pre-release check for a build that has no benefit; the eighth drops the target list for this crate instead (D40), amending the unreleased `ce0fdee` changelog line.
52. **The `ThemeConfig` copies carried no highlighter style.** D34 was argued field by field for colours, fonts, radius and shadow, but `Theme::apply_config` also reads `highlight` (`schema.rs:1066-1073`) and, finding `None`, keeps the previous mode's `highlight_theme`. Both registry default themes carry one (`default-theme.json:113, 314`), which is why 0.5.7's single-config design never showed the defect. The config now carries upstream's default style for its mode (D41), and the both-variants test asserts `highlight_theme.appearance` after the switch.
53. **The observer tests searched for a preset whose splitter divider differs from its border colour.** None exists: `splitter.divider_color` inherits `defaults.border.color` and `hover_color` inherits `divider_color` (`inheritance-rules.toml:238, 273`), and no preset defines `[splitter]`. The helper would have panicked and taken three tests with it. The observable is now `active_handle` (connector: border colour via the splitter; upstream: translucent `drag_border`), with the precondition asserted in the helper (§2.23).
54. **`from_system()` used the cached reduce-motion detector.** `prefers_reduced_motion()` stores its first answer in a process-wide `OnceLock` (`detect.rs:654-656, 856-857`); a caller polling `from_system()` before `apply_accessibility` would have seen the reader's fresh value OR-ed with a frozen fallback. The uncached `detect_reduced_motion()` (`:662`) is used instead.

---

## 8 -- Open questions carried forward

Mirrors specification §16.

1. Whether to offer `dialog_width` from `min_width` despite `Dialog::w`'s fixed-width semantics.
2. Applying the `&AccessibilityPreferences` signature to the iced connector in v0.6.1.
3. Connector parity flags until v0.6.1; expected.
4. The origin of the March `warning.svg` and `info.svg` Material files, which the refresh replaces.
