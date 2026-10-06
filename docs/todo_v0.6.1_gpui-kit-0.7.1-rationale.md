# v0.6.1 — gpui connector on GPUI Kit 0.7.1: Rationale

**Status:** design, not implemented. Written 2026-10-06 from a probe on a
throwaway worktree of `942302d6` (tag `v0.6.0`), the upstream sources of both
versions in the cargo registry, and the upstream repository at tags `v0.7.0`
and `v0.7.1`; revised the same day after a verification pass that re-read
every claim against its source (§3 lists what that pass corrected).

Companion documents: the specification
[`todo_v0.6.1_gpui-kit-0.7.1-spec.md`](todo_v0.6.1_gpui-kit-0.7.1-spec.md)
(what to build, every citation, Appendices A and B) and the plan
[`todo_v0.6.1_gpui-kit-0.7.1-plan.md`](todo_v0.6.1_gpui-kit-0.7.1-plan.md)
(tasks and gates). This document says *why*; decisions are numbered L1–L15.

Citation conventions: `GC` = gpui-component 0.7.1 `src/`, `GB` = gpui-base
0.7.1 `src/`, `GP` = gpui-pre 0.3.8 `src/`, `CN` =
`connectors/native-theme-gpui/`, `SC` = `CN/examples/showcase-gpui/`. Upstream
line numbers are of the 0.7.1 / 0.3.8 sources unless "0.7.0" is written;
connector line numbers are of `942302d6`.

---

## 0 -- What this document is for

GPUI Kit 0.7.1 was published on 2026-10-05 (release notes:
<https://github.com/longbridge/gpui-kit/releases/tag/v0.7.1>). Its crates —
gpui-kit, gpui-component, gpui-base, gpui-kit-assets 0.7.1 — pin
`gpui-pre = "=0.3.8"` (upstream workspace `Cargo.toml:69`). The connector's
floors are carets (`gpui-component = "0.7.0"`), so every application that
resolves its dependencies fresh gets 0.7.1 — and **native-theme-gpui 0.6.0 does
not compile there.** This is the v0.5.9 situation again (gpui-kit 0.6.4 broke
released 0.5.8), and the answer is the same: a patch release, v0.6.1, that
moves the floors, repairs what broke, and corrects everything 0.7.1 changed
behind the compiler's back — plus the widgets 0.7.1 added, shown under the
native theme.

---

## 1 -- The situation, argued from evidence

### 1.1 The released crate is broken for new users

The nightly dependency canary failed on its first run after the release
(run 37391485451, 2026-10-05 23:59 UTC; jobs Clippy, Feature combinations,
Widget coverage):

```text
error[E0004]: non-exhaustive patterns: `gpui_component::IconName::Mic` and
`gpui_component::IconName::Square` not covered
   --> connectors/native-theme-gpui/src/icons.rs:160:16   (and :295, :447)
```

`IconName` is generated from gpui-kit-assets' `default-icons.txt`
(`GC/icon.rs:13-19`), which gained `icons/mic.svg` and `icons/square.svg`.
The connector's three public tables — `lucide_name_for_gpui_icon`,
`material_name_for_gpui_icon`, `freedesktop_name_for_gpui_icon` — are
exhaustive `match`es without a wildcard, so an added variant is a hard error
in the *library*, i.e. in every downstream build.

### 1.2 Nothing else breaks the build

Probe: a temporary `IconName::Mic | IconName::Square => return None` arm in
each table, then `cargo update -p gpui-component -p gpui-base -p gpui-kit -p
gpui-pre` and `cargo build -p native-theme-gpui --all-targets --all-features`
→ `Finished` in 14 m 35 s, **no error and no warning**. The library, the
integration tests and the showcase all compile on 0.7.1 / 0.3.8 once the two
arms exist. The lockfile change is 83 insertions / 146 deletions; `image`
stays at 0.25.10 (the connector's `image` pin keeps matching gpui-pre's).

### 1.3 What the tests say

`cargo test -p native-theme-gpui --all-features --no-fail-fast` on the
probe (manifest floors untouched): lib 283 passed / 1 failed; seams 13 passed;
showcase 178 passed / 1 failed.

1. `showcase::every_colour_claim_is_read_at_the_line_it_cites` —
   "163 of 729 colour claims are not read at the line they cite": the Widget
   Info citations of moved upstream lines (§1.6). Expected; the gate works.
2. `tests::a_page_sample_icon_the_set_lacks_is_absent`
   (`SC/tests.rs:6034`) — "the Search Button … is no narrower than with its
   icon": both 80 × 32. The Medium label shrank (§1.4a), so icon + gap +
   "Search" now fit inside `button.min_width`, and removing the icon no longer
   narrows the button. A symptom of §1.4a, not a test defect.

Once the manifest floors move, two more tests would fail unless the README
moves with them: `src/compat.rs` compares the README's **Required** table and
its Quick-start dependency lines with `Cargo.toml` in both directions
(`the_readme_states_the_manifest_floors`,
`the_quick_start_states_the_manifest_floors`).

### 1.4 What changed behind the compiler's back

Measured: `--dump-layout` of the Basic page before (main, 0.7.0) and after
(probe, 0.7.1) under the six pairings of
`scripts/generate_screenshots_gpui.sh` (kde-breeze, material,
catppuccin-mocha × dark/light). Of 171–177 elements per run, **only Buttons
and Toggle buttons moved** (6 elements under KDE, 10 under catppuccin, 12 under
Material); checkbox, radio, switch, input, select, slider, progress, tabs,
list, chrome: identical. The dumps are in `target/layout-071/{before,after}/`
(not committed).

**(a) The Medium Button label is now 0.875 of the platform font — a
native-look regression.** gpui-component #3287 ("Align control text sizes
with Input and Select", commit `4708780a`) changed `button_text_size`
(`GC/sizing.rs:337-343`): `Size::Small | Size::Medium => text_sm` (Medium was
`text_base`). The connector sets the window rem to the platform font
(`CN/src/lib.rs:203`: `theme.font_size = px(d.font.size * s)`; gpui-base's
root sets the rem from it), so `text_base` *was* the platform size and
`text_sm` is 87.5 % of it. No preset states a button font *size* (each
`button.font` states a colour, Material a weight too); the size is
`defaults.font`'s by sub-field inheritance
(`docs/inheritance-rules.toml:117, :136-137`). Loss per platform:

| Preset | Font | 0.7.0 label | 0.7.1 label |
|---|---|---|---|
| kde-breeze | 10 pt @ 96 dpi | 13.33 px | 11.67 px |
| adwaita | 11 pt @ 96 dpi | 14.67 px | 12.83 px |
| windows-11 | 10.5 pt @ 96 dpi | 14.00 px | 12.25 px |
| macos-sonoma | 13 pt @ 72 dpi | 13.00 px | 11.38 px |

Measured in the dumps: Breeze label height 18 → 16, Material 20 → 18, label
widths −10 to −14 %.

`geometry::button` cannot repair this: the label's size is set on an *inner*
content element (`GC/button/button.rs:749`) after the root refinement
(`:733`), so a `text_size` on the button is shadowed — the "Tier U" note at
`CN/src/geometry.rs:186-198` already says so. The private escape hatch,
`content_style`, is `pub(crate)` (`:303`). `Size::Large` and `Size::Size(_)`
do reach `text_base`, but that is the rem (`defaults.font`), not
`button.font`, and they change the unstated padding sides, the icon and the
caret with it (`:615-618`, `:659-684`).

**The project already knows the way round it.** A *Small* Button's label was
`text_sm` before 0.7.1 too, and the showcase's chrome answers it with a child
label: `tool_label` (`SC/demo.rs:1165-1192`) passes the text as the Button's
child in `button.font`'s size, line height and weight, and names the Button
with `accessibility_label`. The Basic page's Buttons pass their labels as
children as well (`listed_label`, `SC/demo.rs:7882-7903`, to record the
label's bounds) — which is why the dumps show those labels shrinking: a child
inherits the content element's `text_sm` unless it states a size.
`Button::child` puts children inside the content element (`:772`), after
upstream's own label `div` (`:763-771`), so a child that states its text
size wins; the seam tests' `probed_button` rests on the same route
(`CN/tests/seams.rs:537-543`). 0.7.1 turns a Small-button workaround private
to the showcase into something every application needs at the default size.

**(b) A Toggle's text too — but reachable.** `Toggle` now sets `text_sm` at
Medium on its root (`GC/button/toggle.rs:174-179`; Medium set none before and
inherited 1 rem) *before* the caller's refinement (`:214`), so a refinement
that states a text size wins. The connector has no builder for a `Toggle`.

**(c) DataTable cells too.** `table_cell_size` (`GC/sizing.rs:323-333`) now
sets `text_sm` at Medium (it set none before); `DataTable` is not `Styled`, so
no refinement reaches the cells. `Size::Size(_)` falls to `_ => self` (no
size: the cell inherits), keeps the given row height verbatim (`:57-64`) and
takes the Medium paddings (`:89-94`). The route was known for the row height:
`docs/todo.md` carries the open item "A `DataTable` can take the platform's
row height today" (`:1625-1635`), and the showcase's DataTable panel says the
same in its "row height" note (`SC/info/data.rs`). Since 0.7.1 it is also the
only route that leaves the cells at the inherited text size.

**(d) Checkbox, radio, switch: no change.** Their sizes moved to
`input_text_size` (`GC/checkbox.rs:275-278`, `GC/radio.rs:219-222`) but are set
on the root before the refinement (`:283`, `:223`), and the showcase draws the
connector's own `widgets::` versions by default. The dumps confirm it.

**(e) The focus line of borderless elements — no visible change.**
`focus_ring_style` now delegates to `focus_style` (`GC/styled.rs:179-184`,
`:250-298`): with `Theme::focus_ring` *off*, a bordered element's border is
tinted (`:269-271`) and a borderless one draws a 1 px `ring` line
(`:273-297`). The connector turns the ring off only when
`defaults.focus_ring_width` is 0 (`CN/src/lib.rs:213`), and every bundled
preset states 1, 2 or 3 px in both modes (40 of 40 entries; the Windows
reader takes `SM_CXFOCUSBORDER`, `native-theme/src/windows.rs:202`). Only
claim text changes (§1.6).

**(f) Single-line Input text is no longer clipped in a short frame** (#3343):
Windows 11's field leaves a content box about a pixel shorter than one line
of its text (32 − 5 − 6 less the border, for 14 px × 1.43), and the glyphs
were being cut. An improvement the connector gets for free.

**(g) Accepted without code** — each read, each with no effect on what the
connector paints or claims beyond a citation: DataTable keyboard focus ring
(`GC/table/data_table.rs:176-179`; claim added, L10), Shimmer's dark
highlight (`GC/shimmer.rs:448-454`; claim corrected), Command item radius
(`GC/command/state.rs:707-720`; the palette is unbordered, still
`theme.radius`), the single popup-menu highlight
(`GC/menu/menu_item.rs:116`), RadioGroup keeping a disabled item disabled
(`GC/radio.rs:406-408`), Slider touch drag (`GB/slider.rs:607-670` — the
connector's slider is built on that same `SliderTrack`,
`CN/src/widgets/slider.rs:280`, and inherits it), Clipboard's accessible name
(`GC/clipboard.rs:57`), Questionnaire confirm-on-choose (still excepted, L9),
scroll bounce, Textarea rows (#3330), soft wrap vs. horizontal scroll
(#3358), OtpInput's ladder (`GC/input/otp_input.rs:117-120`; no native
counterpart, no claim states a size), Accordion XSmall and Tag sizes (the
showcase uses Medium only), the Large Switch track 44 × 24
(`GC/switch.rs:187, :192`; the connector draws its own switch and the
upstream fallback is Medium), the dropdown row inset
(`GC/sizing.rs:173-179`; rows are `SearchableListItemElement`, Tier U), a
non-interactive chart keeping its id (upstream's own test,
`GC/chart/mod.rs:690-699`; the showcase sets neither), a `TextView` that
follows its container's text colour (#3329: without an explicit style it takes
the colour its container sets, and on a surface whose text is more than 0.6
apart in lightness it derives its link, muted, code and table colours from it,
`GB/text/text_view.rs:650-664`, `GB/text/style.rs:300-334`; the showcase's
Markdown sits under the page root's `text_color(theme.foreground)`,
`SC/app.rs:2210`, which is the colour its style already has, so nothing is
inherited or derived), scrollable code blocks (#3322; only for a code-block
style with a height cap, and the showcase's `TextView` sets no style,
`SC/demo.rs:5741-5744`), theme-file watching (#3320; nothing in `CN` uses
`ThemeRegistry`), and gpui-kit's hidden `gpui` re-export now naming the Kit
(`gpui-kit/src/lib.rs:100`; nothing in `CN` uses `gpui_kit::gpui::`).

### 1.5 What upstream added, and what a theme showcase should show

`scripts/check_widget_coverage.py` discovers three new public widgets
(147 → 150): **ColorSelect**, **SpeechButton**, **SpeechWaveform**. The
`questionnaire/components.rs` additions are all inside `#[cfg(test)]`. Charts
gained a draw-in motion. Until each widget is shown or excepted, the coverage
gate fails — so no subset of this work that skips them is releasable.
Details in L7–L9.

### 1.6 Citations

Of 1237 citations of the gpui stack in `CN`: 659 cite unchanged files, 125
changed files at unmoved lines, **436 moved** (every one verified
byte-identical at its new line), **17 sites cite code that changed** (13
distinct targets), none cite a removed file. `docs/showcase-exceptions.toml`:
33 moved, 3 changed. Appendix A and B of the spec carry the full map. The
review also found citations that were already wrong at 0.7.0 (spec §9.3).
`ROADMAP.md` names the 0.7.0 stack in three places.

---

## 2 -- Decisions

### L1 -- Floors: the 0.7.1 stack, carets, MSRV re-measured

`gpui-component`, `gpui-base`, dev `gpui-kit` → `"0.7.1"`; `gpui` (package
`gpui-pre`, both the dependency and the dev-dependency) → `"0.3.8"`. 0.7.1 is
a **hard** floor: the `Mic`/`Square` arms do not compile against 0.7.0. Cargo
backtracks, so an application that pins 0.7.0 keeps resolving to
native-theme-gpui 0.6.0 — the raise is not breaking.

Carets, as before. The alternative that would have prevented this break — an
upper bound (`>=0.7.1, <0.7.2`) or an exact pin — turns *every* upstream patch
release, compatible or not, into a resolver conflict until the connector
releases again, and a library that pins conflicts with any consumer whose
other dependencies want a later snapshot. The README states the contract the
project chose instead (Required floors, a Verified line, the nightly canary);
L3 removes the one failure class that has now struck.

`rust-version` stays `1.95.0`, measured 2026-10-06: gpui-pre 0.3.8 still
calls `std::hint::cold_path` (`GP/profiler.rs:473, :494, :614`), stable since
1.95, so 1.94 cannot build it; `cargo +1.95.0 check -p native-theme-gpui
--lib --locked` on the 0.7.1 / 0.3.8 lockfile: `Finished` in 3 m 20 s.

The README's **Required** table and Quick-start lines move with the manifest,
by hand, in the same commit (§1.3: `src/compat.rs` holds them to it). Only
the **Verified** line is the script's.

### L2 -- Icons: `Mic` and `Square` get genuine glyphs in every set

Same rule as v0.6.0's K3: a variant maps to the set's own glyph for the same
thing, or `None`; never another set's glyph.

| `IconName` | Lucide | Material Symbols | freedesktop |
|---|---|---|---|
| `Mic` | `mic` (exact) | `mic` (exact) | `audio-input-microphone` (exact; Icon Naming Spec, Devices) |
| `Square` | `square` (exact) | `square` (exact) | `media-playback-stop` (close: upstream draws `Square` only as `SpeechButton`'s stop glyph, `GC/speech/button.rs:96-100`; Breeze and Adwaita draw the stop square filled, Lucide's is an outline) |

Sources, all verified HTTP 200 on 2026-10-06: Lucide 1.41.0
`icons/mic.svg`, `icons/square.svg` — **byte-identical** to
gpui-kit-assets 0.7.1's `assets/icons/mic.svg` / `square.svg`; Material
Symbols at the pinned commit `0cbb0881…` `symbols/web/mic/…/mic_24px.svg`,
`symbols/web/square/…/square_24px.svg`. Both freedesktop names are installed
here in Breeze (`/usr/share/icons/breeze/devices/16/audio-input-microphone.svg`,
`…/actions/16/media-playback-stop.svg`) and in Adwaita as symbolic icons
(`…/Adwaita/symbolic/devices/audio-input-microphone-symbolic.svg`,
`…/actions/media-playback-stop-symbolic.svg`). The files enter through
`scripts/update_icons.sh add …`; `native-theme/icons/SOURCES.toml` needs no
edit (both sets' `[[set]]` rules cover them), and bundled icons are discovered
by file, so `native-theme` needs no code change.

No `IconRole` means "microphone" or "stop", so `icon_name(IconRole)` is
unchanged.

### L3 -- The icon tables stop breaking downstream builds

The canary did its job, but the cost of the design is that *every* icon
upstream adds in a patch release breaks every released connector for every
new user. The exhaustiveness belongs in the tests, where it breaks only the
canary — the choice v0.5.9 made for `Theme`'s fields (E17: one test names
every field in an exhaustive destructuring).

- Each of the three library tables gets a final
  `#[allow(unreachable_patterns)] _ => return None,` arm: an `IconName` this
  connector does not know draws no icon from this set — the never-mix rule's
  `None`. `allow`, not `expect`: on a later upstream the arm becomes
  reachable, and an unfulfilled `expect` would itself warn. (Toy check under
  `#![deny(warnings)]`: with the attribute the arm compiles clean; without it
  the wildcard is "unreachable pattern".)
- In the test module one macro writes `ALL_ICON_NAMES` and, from the same
  identifiers, a `match` with **no** wildcard (`variant_name`). A variant
  upstream adds then stops the *test module* compiling (E0004) — the canary's
  `cargo clippy --all-targets` and `cargo test` report it — and nothing
  downstream. A variant listed twice is an "unreachable pattern", which
  `-D warnings` rejects. `variant_name` earns its place by naming the icon in
  `every_none_is_an_allowed_gap`'s failure messages, which today cannot say
  which icon failed (`IconName` has no `Debug`). The count test stays, at 106,
  as the reminder to update the counts the prose states.

Rejected: keeping the library matches exhaustive (the status quo: a broken
release per upstream icon); a hand-numbered second list of all variants
(three places to edit per icon instead of one); a runtime list read from
`default-icons.txt` (gpui-kit-assets' data, and a build script).

The showcase's gallery list `GPUI_ICONS` (`SC/support.rs:541`) gains the two
entries and stays hand-kept; guarding it the same way is possible (all 104
pairs are `(stringify!(variant), variant)`) but changes how two call sites
read it, for a list whose only failure is a missing gallery tile. Left for the
upstream wish already on the ROADMAP (an iterable `IconName::ALL`).

### L4 -- The Button label reaches the platform font through a child

New public builder `geometry::button_label(n) -> StyleRefinement`:
`button.font`'s size (with the text-scaling factor) and weight, and the
platform's line height (`relative(defaults.line_height)`, the value every
control builder applies) — the three things the showcase's private
`tool_label` sets by hand today. No colour: the label takes the button's
state colours from the root, as `.label()`'s does and as `geometry::button`
reasons (`CN/src/geometry.rs:186-198`). Its rustdoc gives the one public
route (§1.4a) — a label passed as a child, built like upstream's own label
element, plus the accessible name upstream derived from `.label()`
(`GC/button/button.rs:737-740`):

```rust
Button::new("save")
    .refine_style(&geometry::button(n))
    .child(
        div().min_w_0().whitespace_nowrap().text_ellipsis()
            .refine_style(&geometry::button_label(n))
            .child("Save"),
    )
    .accessibility_label("Save")
```

It carries the line height so that it is complete without `geometry::button`:
a `ButtonGroup` child, which must not take `geometry::button` (the group
manages its corners), still gets the whole label.

The showcase already has both halves of this (§1.4a). They become one:
`labelled` (`SC/demo.rs:7903-7916`) builds the child label with
`geometry::button_label` whenever a native theme is installed; `tool_label`,
the same thing with its style written by hand, goes, and its callers call
`labelled`; every Button the showcase labels goes through `labelled` — except
the "Button Sizes" panel, which shows upstream's own scale on purpose
(`SC/info/buttons.rs:580-581`). The Basic page then returns to the 0.7.0 dumps
element for element; that is the gate.

Why a refinement and not a helper that builds the element: every `geometry`
item is a pure function returning a `StyleRefinement` or a `Size` (module doc,
`CN/src/geometry.rs:1-20`); the four-line pattern is documented once.

What stays out of reach: labels of Buttons that gpui-component's widgets
build for themselves. The root cause is upstream's — a Button cannot be told
its label's size — and the ROADMAP already lists the ask ("button label text
size independent of rem"); it is sharpened there and on `docs/todo.md`, and
the maintainer files it (an outward action). When it lands, `button_label`
applies to the button itself and the child pattern can go.

Rejected: `Size::Large` / `Size::Size` (§1.4a: the rem, not `button.font`,
and other metrics move); leaving it as "Tier U, documented" (a visible
regression on the showcase's default page and in every application, with a
public fix the project already uses); a connector-drawn Button (the `widgets`
feature draws what upstream draws from literals; a Button is themable except
for this one size).

### L5 -- Toggle: `geometry::toggle`, the button font

`geometry::toggle(n)` returns what `button_label` returns, for the `Toggle`
itself, where the refinement lands after upstream's own `text_sm` (§1.4b). A
second name for the same refinement because the module is organised per
widget — each builder says where its refinement lands — and a Toggle takes it
on its root while a Button's label takes it on a child.

*The button font*, because a toggle button is a button natively (a checkable
`QPushButton` or `QToolButton`, a `GtkToggleButton`), and the showcase's Basic
page already draws the native "Toggle button" as a `Button` with
`geometry::button_checked` (`docs/showcase-elements.toml`,
`basic.buttons.toggle_on`: its leaves are `button.*`).

*Only the text.* `Toggle` is `Ghost` by default (`GC/button/toggle.rs:15-16`),
a flat button; `geometry::button`'s border, minimum width and padding would
make a framed push button of it, and the model states no flat-button metrics.
0.7.1 did not change its frame, and it stays upstream's.

*Not the segmented control.* The Toggle panel's note says
`segmented_control.*` "would reach a Toggle through the geometry::toggle
nobody has written" (`SC/info/buttons.rs:658`). That note has the mapping
wrong: in this connector the native segmented control is the segmented
`TabBar` — the `segmented_control.*` rows of `CN/src/contract.rs:929-970` all
name `TabVariant::Segmented`, `tab_bar_segmented` takes
`segmented_control.background_color` (`CN/src/colors.rs:213`),
`docs/todo_gpui-full-theme.md:59` files its geometry under "built from
`Tab`s", and the showcase draws it with `TabBar::segmented()`
(`SC/demo.rs:2070`). A builder mapping it onto `ToggleGroup` as well would
give one native control two gpui homes. The note is corrected (L10).

### L6 -- DataTable: `geometry::data_table_size`

`geometry::data_table_size(n) -> Size` = `Size::Size(px(list.row_height))`,
or `Size::Size(Size::Medium.table_row_height())` where the theme states no row
height (the unstated-size rule: the toolkit's own value; KDE states none).
Under `Size::Size` the cells set no text size and inherit (§1.4c), so a
container refined with the existing `geometry::table` (`list.item_font`,
`CN/src/geometry.rs:549-553`) gives them the platform's list font; padding
stays upstream's Medium. Shape precedent: `geometry::spinner_size`
(`:750-752`). The showcase's `demo::data_table` (`SC/demo.rs:4021-4041`) takes
both, and the open todo item (§1.4c) is closed by it.

The header cells inherit the same text as the body cells, as they did at
0.7.0; `list.header_font`'s size and weight have never reached a DataTable
header and that is not a 0.7.1 matter (L15).

### L7 -- ColorSelect: shown, no geometry builder

Shown on the Inputs page beside the ColorPicker, with its own
`ColorPickerState` (the existing picker's state holds that picker's open flag),
and **empty**: an empty field draws only theme colours — the placeholder in
`muted_foreground`, the swatch edged in `input` — while a picked value edges
the swatch in that colour darkened by 30% (`GC/color_picker.rs:794-796`). The
panel follows the state: what it claims for the placeholder and the swatch
holds only while nothing is picked.
`ColorSelect::new` and `ColorPickerState::new` return plain values
(`GC/color_picker.rs:546`, `GB/color_picker.rs:194`).

No builder: the refinement lands on the outer `ColorPicker` wrapper
(`GC/color_picker.rs:488`), while the framed field is an inner element
(`render_field`, `:762-813`) that sets its own height, padding and text size
from the Input ladder (`:775-776`); only a width gets through (`:508`).
`geometry::select` would put its padding outside the frame. The panel says so
(Tier U), with the colours the frame does take from the theme: border `input`
(`:773`), fill and text from `input_style(false)` (`:763`), placeholder and
caret `muted_foreground` (`:806`, `:812`), `ring` while focused or open
(`:777-779`). Its caret is gpui-component's own icon, so the panel carries the
"own icons" note and joins the test that requires it
(`a_widgets_own_icons_are_named_gpui_components`, `SC/tests.rs:6175-6191`).
Upstream ask (todo): apply the caller's refinement to the field.

### L8 -- Speech: the waveform is shown, the button is excepted, no `speech` feature

- **No `speech` feature** on the dev `gpui-kit`: it adds only the
  `Microphone` and `SystemRecognizer` (`GC/speech/mod.rs:19-35`) and pulls in
  `cpal → alsa-sys`, whose build needs libasound's development package — a new
  CI system dependency for nothing the showcase needs. The widgets and the
  `AudioInput` / `SpeechRecognizer` traits compile without it
  (`GC/lib.rs:80`, `mod.rs:13-36`).
- **SpeechWaveform: shown**, on the Inputs page. It draws nothing before audio
  (`GC/speech/waveform.rs:79`: it draws `state.levels()`), so the showcase
  gives its `SpeechState` an `AudioInput` that delivers a fixed synthetic
  envelope and a `SpeechRecognizer` whose session discards audio and reports
  no text — the route upstream documents ("implement this trait to feed audio
  from elsewhere, e.g. a file in tests", `GC/speech/recognizer.rs:211-214`)
  and its own tests use (`FakeInput`, `FakeRecognizer`,
  `GC/speech/state.rs:389-447`). A plain Button ("Listen" / "Stop") starts and
  stops it. No microphone, no network, nothing recognised; the panel says the
  signal is synthetic. The bars are `primary` while capturing,
  `muted_foreground` otherwise (`waveform.rs:92-96`).

  The input delivers on a timer, one block per 40 ms, not in one burst: the
  waveform's playhead is paced by when levels *arrive*
  (`GC/speech/level.rs:14-35`, `PLAYHEAD_*`), so only a device-like source
  shows the widget as an application will see it.
- **SpeechButton: excepted**, as the kind "it would mix icon sets" — the
  `SidebarToggleButton` precedent (v0.5.9 chrome-UX rationale §9): a button
  that is nothing but gpui-component's own icon, `Mic` or `Square`, chosen
  while rendering with no setter (`GC/speech/button.rs:96-100`), where the
  application can build the same control from a Button of its own. (Without a
  recognizer it renders an empty `div`, `:78-79`.) It joins the icon-provider
  ask on `docs/todo.md`.

### L9 -- Charts: upstream's draw-in stays; Widget Info states it

Every chart draws its data in over 1000 ms on first paint
(`GC/theme/mod.rs:69, :77-79, :125`, easeOutQuart). The model has no motion
field beyond `reduce_motion`, which the connector already forwards
(`CN/src/lib.rs:846-857`) and which skips the draw-in: the plot's appear is a
`Presence` (`GB/plot/appear.rs:245`), and a presence is whole at once under
reduced motion (`GB/motion/presence.rs:61`; upstream's
`test_reduced_motion_skips_the_appear`, `appear.rs:449-455`). So: no code; the
chart panels' shared `hover` note gains a "draw-in" line with those citations.
Captures are `--tab basic` only (`scripts/generate_screenshots_gpui.sh:77`),
and the Basic page has no chart, so no artefact changes. The Questionnaire
exception stands: its state's constructor still returns `Result`
(`GB/questionnaire/state.rs:48-51`).

### L10 -- Widget Info is corrected, not reworded to fit

The claims 0.7.1 made false are rewritten to what the code now does: the
button label's size (`SC/info/leaves.rs:354`, `SC/info/buttons.rs:512, :535`
— `button.font` through the child label where a native theme is installed,
upstream's ladder with Medium at `text_sm` where none is), the Shimmer dark
claim (`SC/info/feedback.rs:422`), the Carousel's no-ring branch
(`SC/info/layout.rs:736-756`: a borderless element now gets a line, not a
tinted border), the DataTable panel's "row height … nothing applies it here
yet" note, and a new DataTable keyboard-focus line. `toggle_notes`
(`SC/info/buttons.rs:655-658`) says what `geometry::toggle` carries and drops
the segmented-control sentence (L5).

### L11 -- Citations: a renumbering commit, then a rewriting commit

The 436 + 33 moved citations are a pure renumbering: each old line was
verified byte-identical at its new line, so whatever the prose claimed about
the old line it claims about the new one. They are applied mechanically
(Appendix A and B) in a commit of their own, and that commit is held to a
mechanical gate — with every digit stripped, each removed line equals an
added line — so nothing but numbers can have changed.

The 17 sites whose code changed (spec §9.2), the citations already wrong at
0.7.0 (§9.3) and the prose naming the verified stack are rewritten by reading,
in a second commit. `every_colour_claim_is_read_at_the_line_it_cites` gates
the showcase's 729 colour claims (it reports 163 today). The library's prose
citations have no mechanical gate beyond the renumbering check; a checker is
out of scope (L15).

### L12 -- The broken test needs no edit

`a_page_sample_icon_the_set_lacks_is_absent` failed because the label shrank
(§1.3). L4 puts the label back at its 0.7.0 size, the Buttons page lays out as
it did, and the test passes as written — the gate of Task 3. It did its job;
it is not changed.

### L13 -- Version, roadmap, notes, stamp

- **Version** `0.6.0` → `0.6.1`, all seven crates (`native-theme` gains two
  bundled SVGs per set). Branch `v0.6.1-rc1` from `main`.
- **ROADMAP.md** had promised the number to something else: "v0.6.1 — Full
  theme geometry in the iced connector" and "v0.6.2 — Upstream receivers for
  the gpui connector". A patch number goes to whichever release ships first,
  and this one cannot wait: the two planned milestones become v0.6.2 and
  v0.6.3 (with `docs/todo_gpui-full-theme.md:72`, which names "the v0.6.2 PR
  list"), and v0.6.1 gets a short delivered entry.
- **CHANGELOG** under a new, undated `## [Unreleased]`; a `### Fixed` line
  says plainly that 0.6.0 does not build against gpui-component 0.7.1 (the
  README promises that a break the canary finds is recorded there).
- **`docs/todo.md`**: the DataTable row-height item is ticked; the notes that
  say a Button label is "already the platform's size" get a dated line; the
  upstream asks are added.
- **Stamp.** `scripts/update_compatibility.sh run gpui` is the only writer of
  `docs/COMPATIBILITY.toml` and the README's Verified line. The maintainer's
  release asset run (`scripts/generate_assets_release.sh`) runs it again for
  all three connectors — it must, since the version bump touches
  `native-theme-egui-widgets/Cargo.toml` and so the egui stamp — but running
  it once on the branch keeps the README from saying "Required 0.7.1" beside
  "Verified against 0.7.0", and it re-resolves the newest upstream, which
  catches a 0.7.2 published in the meantime.
- **CI** runs on `main`, tags and pull requests only (`ci.yml:3-7`); the last
  release found its CI failures after `main` had moved. The hand-over
  therefore asks for a pull request before the fast-forward.
- Screenshots, tag, push, publish: the maintainer's.

### L14 -- Upstream changes accepted without code

§1.4e–g. Each is listed in the spec (§10) with its citation, so a reviewer can
see it was read, not missed.

### L15 -- Out of scope, on purpose

- A mechanical checker for the library's prose citations (the showcase's
  colour claims have one). Goes on `docs/todo.md`.
- `list.header_font` on a DataTable's header (L6): never applied, unchanged
  by 0.7.1.
- A compile-time guard for the showcase's `GPUI_ICONS` (L3).
- `PlotAppearScope` in the showcase (the Charts page replays the draw-in on
  every visit; that *is* upstream's behaviour for an unscoped page).
- A `speech` feature; a microphone or stop `IconRole`.
- Filing the upstream PRs (L4, L7): drafted on `docs/todo.md`, filed by the
  maintainer.

---

## 3 -- What was verified, and how

| Claim | How |
|---|---|
| 0.6.0 fails on 0.7.1 only at the three icon tables | canary log; probe build with the arms stubbed: `Finished`, no warning |
| Exactly two tests fail on the probe | probe `cargo test --no-fail-fast` |
| Only Button/Toggle-button labels move on the Basic page | 12 `--dump-layout` runs, element-by-element diff |
| Medium label = 0.875 rem | `GC/sizing.rs:337-343`; dumps (label heights 18→16, 20→18) |
| A child label that states a size wins | `GC/button/button.rs:749, :763-772`; the showcase's `tool_label` and `listed_label` and the seam tests' `probed_button` already rely on it |
| Toggle refinement lands after its size | `GC/button/toggle.rs:174-179, :214` |
| `Size::Size` table cells inherit | `GC/sizing.rs:57-64, :323-333`; no other text size in `GC/table/` |
| The wildcard arm is warning-free; the macro's match is exhaustive | toy crate under `#![deny(warnings)]`: clean; a missing variant is E0004, a duplicate "unreachable pattern" |
| Icon sources genuine | HTTP 200 at the pinned refs; Lucide files byte-identical to gpui-kit-assets 0.7.1; system icon files listed |
| MSRV stays 1.95.0 | `GP/profiler.rs:473` (`std::hint::cold_path`, so not 1.94); `cargo +1.95.0 check --locked` on the probe: `Finished` |
| 436 citations shifted verbatim, 17 changed | registry diff 0.7.0→0.7.1, every shifted line re-read |
| Three new widgets under the coverage rule | the script's own discovery run on both source trees |
| Reduced motion skips the chart draw-in | `GB/plot/appear.rs:245` → `GB/motion/presence.rs:61`; upstream's test `appear.rs:449-455` |
| The synthetic speech route | upstream's `FakeInput` / `FakeRecognizer` (`GC/speech/state.rs:389-447`); `stop` drops the capture (`:244`) |

**Corrected by the verification pass** (the first draft had these wrong):

1. It proposed a `geometry::toggle_segment` built on `segmented_control.*`,
   claiming the connector maps the segmented control onto Toggle. It maps it
   onto the segmented `TabBar`; the builder is dropped (L5).
2. It proposed a new showcase helper for child labels. The showcase has two
   (`tool_label`, `listed_label`); they become one, `labelled`, and
   `button_label` carries the line height `tool_label` sets (L4).
3. It left the README's Required table to the stamp script. The table is
   hand-written and test-checked against the manifest (L1).
4. It did not see that ROADMAP.md had allocated v0.6.1 and v0.6.2 (L13), that
   `docs/todo.md` already carries the DataTable item (L6), or that ColorSelect
   draws an icon of gpui-component's own (L7).
5. It called Tasks 0–2 with the release tasks "releasable on their own". They
   are not: the coverage gate fails until the three new widgets are shown or
   excepted (§1.5).
6. It replaced the icon count test with a hand-numbered second list; one macro
   is less to maintain (L3). It planned to rewrite a showcase test that needs
   no edit (L12), and to open and re-read each of 436 byte-identical
   citations where a renumbering gate does the job (L11).
7. Citations: `otp_input.rs:116-119` → `:117-120`; `button.rs:614-617` →
   `:615-618`; "no preset states `button.font`" → no preset states its *size*.

**Corrected by the second pass:** `tool_label` is removed rather than kept
beside `labelled`; two Button label sites the list missed (`SC/demo.rs:6453`,
and `:8051`, a tool-button fallback that bypasses `tool_label`) are added, with
a grep that proves none is left; the digits-only gate compared line counts by
parity, which a markdown list line could slip past, and now compares the
removed and added lines as multisets after `cargo fmt`; the speech test's
`levels().len() > 0` becomes `levels().next().is_some()` (clippy's `len_zero`
asks for an `is_empty` that `ExactSizeIterator` has only on nightly);
`info::data::data_table` needs a `styled` parameter for its native-only lines.
Re-checked and confirmed: `Size::Size` has no other effect inside a DataTable
(no `smaller()`; the loading rows take the same row height and padding,
`GC/table/loading.rs:46-54`); the chart tooltip waits for the draw-in
(`GB/plot/element.rs:125-128`); without a `PlotAppearScope` a chart draws in
each time it is painted anew (`GB/plot/appear.rs:107-110`); the connector
leaves upstream's plot motion in place (`CN/src/base_layer.rs:8-9`,
`GC/theme/mod.rs:455`); `AsyncApp::update` returns its closure's value
(`GP/app/async_context.rs:176`).

**Corrected by the third pass:** the ColorSelect was to start from the
picker's default colour while claiming the placeholder and an `input`-edged
swatch, neither of which a picked value shows (`GC/color_picker.rs:794-796`,
`:804-806`) — it now starts empty and its panel follows the state; the
speech state's `cx.observe(…)` is dropped, because gpui already redraws a
window whose draw read an entity that notifies (`GP/window.rs:3460-3480`,
`GP/view.rs:503`), which is why the showcase subscribes nowhere for redraws;
the Toggle panel's native-only line needs the `styled` flag the Button panel
takes. Re-checked and confirmed: `scaled_text_size` is `size ×
text_scale_factor` and the rem is `defaults.font.size × s` with the same
factor (`CN/src/lib.rs:203, :463-474`), and every preset inherits
`button.font.size` from `defaults.font`, so the label sizes are bit-identical
and the layout gate can expect 0 differences; the Basic page's Buttons
(`demo::button`, `built_tooltip_button`, `toggle_button`, `icon_button`) and
the chrome's tool buttons draw the same label sizes before and after, so the
gate holds.

**Fourth pass:** a `TextView` now follows its container's text colour (#3329);
the showcase's Markdown is unaffected, and it joins the accepted changes
(§1.4g, spec §10) with the reason. Re-checked and confirmed: the bundled SVG
tables are generated from the directory listing (`native-theme/build.rs`), and
`native-theme`'s count tests count `IconRole` mappings, not files
(`native-theme/src/model/icons.rs:1848-1866`), so the four new files need no
code change; a DataTable seam needs four delegate methods (`columns_count`,
`rows_count`, `column`, `render_td`; `GC/table/delegate.rs:16-122`), a cost in
proportion to the seam it pins; no release check verifies links, so ROADMAP's
link to the archived spec, written in Task 8, is only dangling until Task 10.
