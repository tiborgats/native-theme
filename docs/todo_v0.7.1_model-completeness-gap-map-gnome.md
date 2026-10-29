# Gap map — GNOME as read: libadwaita 1.10.0, GTK 4.24.1, gnome-shell and mutter 51.0 (preset `adwaita`)

(review 2026-10-28: the title said "GNOME 47+ / libadwaita / GTK 4"; it now names only the versions read. libadwaita draws visibly differently between minor versions, GN-P3-18†.)

Model at caeddd6b (`docs/property-registry.toml`), checked field by field. Read 2026-10-27.

## Sources and how they are cited

| Short | Source |
|---|---|
| `A/w/F:L` | libadwaita tag **1.10.0** (commit `c9ab118612eee92324143783c4641c98eaded3f0`), `src/stylesheet/widgets/F`, line L |
| `A/s/F:L` | libadwaita 1.10.0, `src/stylesheet/F` (`_colors.scss`, `_common.scss`, `_drawing.scss`) |
| `A/c/F:L` | libadwaita 1.10.0, `src/F` (C sources and `.ui` templates) |
| `G/F:L` | GTK tag **4.24.1** (commit `2ea935e3ec680fb122f245b28e18c3495df8c344`), `gtk/F` |
| `GDK/F:L` | GTK 4.24.1, `gdk/F` |
| `S/<schema>:L` | gsettings-desktop-schemas **51.0**, `schemas/<schema>.gschema.xml.in` |
| `M/F:L`, `GS/F:L`, `GS-data/F:L` | mutter tag **51.1** (path from the repository root), gnome-shell tag **51.1** `js/ui/F` and `data/F`, as defined in *Cross-check (pass 2)* and the seeds file; used in §4.1–§4.3 by the review of 2026-10-28 |

Source kinds: **read** = the reader can query it at run time (the settings portal forwards these gsettings keys under their schema names, as `native-theme/src/gnome/mod.rs:10-12` already relies on; the GDK Wayland backend maps them to GtkSettings at `GDK/wayland/gdksettings-wayland.c:270-312`); **published** = libadwaita / GTK source at the refs above; **measure** = a GTK 4 / libadwaita probe app (libadwaita draws its own stylesheet on any desktop), GTK Inspector, or a screen recording for motion. Values quoted below are what the cited line says; nothing is estimated. "to verify" marks what I did not confirm.

Two mechanisms recur and are written once here:

- **HC** — every `@media (prefers-contrast: more)` block. The model has `AccessibilityPreferences.high_contrast` (a flag) but no high-contrast variant of any colour, opacity or line, so each HC block below is a gap of its own (the global ones in §4.3).
- **States libadwaita adds everywhere**: `:hover`, `:active` / `.keyboard-activating` (pressed), `:checked`, `:selected`, `.has-open-popup`, `:drop(active)`, `:disabled`, `:backdrop` (window not focused), `:focus-visible`. The model carries hover and disabled almost everywhere, pressed and checked for a few widgets, and none of the combinations.

---

## 4.1 Elements the model has: missing parts, states and properties

### Window and its chrome (`[window]`)

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.01 | Window (CSD) | Drop shadow geometry: three layers (`0 0 14px 5px`, `0 0 5px 2px`, `0 0 0 1px`), each with its own colour | published `A/w/_window.scss:3-5` | `window.border.shadow_enabled`, `defaults.shadow_color` (one colour, no geometry) |
| GN-1.02 | Window (CSD), unfocused | The backdrop shadow (three different layers) and its 200 ms ease-out transition | published `A/w/_window.scss:19-27`; `A/s/_common.scss:2` | none |
| GN-1.03 | Window (CSD) | Inner outline: 1px `outline` inset by −1px in `$window_outline_color` (white 7 %), a second edge inside the outer 1px shadow ring; HC 30 % | published `A/w/_window.scss:9-13`; `A/s/_colors.scss:303-304` | `window.border.color` (one edge only) |
| GN-1.04 | Window (CSD), HC | Outer ring alpha change under HC | published `A/w/_window.scss:12-17` | none |
| GN-1.05 | Window states | Tiled / half-tiled (radius 0, no outline, `0 0 0 1px $border_color` ring), maximized and fullscreen (radius 0, no shadow, no transition) | published `A/w/_window.scss:49-72` | `window.border.corner_radius` (one value, no state) |
| GN-1.06 | Window radius per corner | `.ssd-frame` squares the bottom corners only | published `A/w/_window.scss:74-77` | `window.border.corner_radius` (one radius for all corners) |
| GN-1.07 | Window without compositing (`.solid-csd`) | Square frame drawn as three inset shadows (5px border-colour, 4px header-bar colour, 1px border-colour), padding 5, its backdrop variant | published `A/w/_window.scss:80-94` | none |
| GN-1.08 | Window, server-side frame (`.ssd`) | 1px ring alpha, HC variant | published `A/w/_window.scss:96-103` | `window.border` |
| GN-1.09 | Message-dialog windows | Own three-layer shadow (`0 0 14px 2px`, `0 0 5px 2px`, `0 0 0 1px`) different from the main window's | published `A/w/_window.scss:36-47` | none |
| GN-1.10 | Window, developer build (`window.devel`) | Striped header-bar background (`devel-stripes.svg` cross-faded 5 %) | published `A/w/_header-bar.scss:237-247`; `A/w/_toolbars.scss:353-362` | none |
| GN-1.11 | Title bar (AdwHeaderBar) | Height: 47 standalone, 46 as titlebar, 37 / 36 for `.default-decoration`, 40 / 30 shrunk (`.collapse-spacing`) | published `A/w/_header-bar.scss:2`, `:53-54`, `:94-110`, `:149-165` | none (seed: gpui `info/chrome.rs:207` "the model states no title-bar height") |
| GN-1.12 | Title bar | Content padding (6 7 7 7; 3 4 4 4 default-decoration; bottom 6 as titlebar), start/end box spacing 6, margin 6 between start/end boxes and title | published `A/w/_header-bar.scss:20-39`, `:56-58`, `:98-100` | none |
| GN-1.13 | Title bar | Bottom shade line (`inset 0 -1px --headerbar-shade-color`) and the two side borders (`inset ±1px $headerbar_border_color`, from `--headerbar-border-color` at border opacity); HC variant | published `A/w/_header-bar.scss:5-13`; `A/s/_colors.scss:42-45`, `:264` | `window.border.color` (window edge, not the bar's lines) |
| GN-1.14 | Title bar | Its own drop shadow below a non-flat titlebar (`0 1px` + `0 2px 4px`, header-bar shade at 50 %), HC variant | published `A/w/_header-bar.scss:78-92` | none |
| GN-1.15 | Title bar colours | `--headerbar-fg-color`, `--headerbar-shade-color`, `--headerbar-darker-shade-color`, `--headerbar-border-color` (L and D) | published `A/s/_colors.scss:40-45`, `:103-107` | `window.title_bar_background`, `window.title_bar_font.color` (bg and fg only) |
| GN-1.16 | Title bar, unfocused | Contents fade to `filter: opacity(0.5)` (not a colour change) with a 200 ms ease-out transition on both fill and filter | published `A/w/_header-bar.scss:41-51`; `A/s/_common.scss:2` | `window.inactive_title_bar_text_color` (a colour; GNOME dims the whole content incl. buttons) |
| GN-1.17 | Title bar variants | `.flat` header bar (no fill, no shade), inline header bar inside AdwToolbarView (no backdrop change) | published `A/w/_header-bar.scss:112-147`; `A/w/_deprecated.scss:456-458` | none |
| GN-1.18 | Title and subtitle (AdwWindowTitle) | Title: bold, 12 px side padding; subtitle: `font-size: smaller`, dimmed (`--dim-opacity`), 12 px padding; title/subtitle negative margins | published `A/w/_header-bar.scss:213-235` | `window.title_bar_font` (no subtitle font, no subtitle colour, no padding) |
| GN-1.19 | Title font source | libadwaita's title is the UI font made bold; `titlebar-font` is meant only for when `titlebar-uses-system-font` is false (default true), and no component this file reads uses it: GTK 4.24.1, mutter 51.0 and gnome-shell 51.0 contain no `titlebar-font` / `titlebar-uses-system-font` (grep), and mutter's own frames are a GTK 4 header bar (GN-1.23) — the model has one `title_bar_font`, which the reader fills from `titlebar-font` | read `S/org.gnome.desktop.wm.preferences:179-185` (`titlebar-uses-system-font`); published `A/w/_header-bar.scss:215-218` | `window.title_bar_font` (partial: which font applies is not expressible) (review 2026-10-28: "applies … to mutter-drawn frames" removed; mutter 51.0 does not read the key) |
| GN-1.20 | Window controls (GtkWindowControls) | Which buttons appear and on which side (`button-layout`, default `'appmenu:close'`) | read `S/org.gnome.desktop.wm.preferences:27-30`; `G/gtksettings.c:857` (`gtk-decoration-layout`); `GDK/wayland/gdksettings-wayland.c:291` | none |
| GN-1.21 | Window control button | Geometry: button min-width 24, padding 5, gap between controls 3; `.default-decoration` 22×22, padding 4, image padding 3; app icon margin 9 / 7 | published `A/w/_header-bar.scss:60-75`, `:175-207` | none |
| GN-1.22 | Window control button | Shape and fill: a circle behind the glyph (`image { border-radius: 100%; padding: 4px; background: $button_color }`), hover `$button_hover_color`, pressed `$button_active_color`, 200 ms transition, HC inset ring | published `A/w/_header-bar.scss:189-201`; `A/w/_buttons.scss:1-3` | none (icons themselves exist: `IconRole::WindowClose` etc., `model/icons.rs:57-65`) |
| GN-1.23 | Server-side frame drawn by mutter (X11 apps that request SSD) | The frame mutter draws: `mutter-x11-frames`, a GTK 4 process, draws it as a GtkHeaderBar with `.titlebar.default-decoration` in a window with `.ssd-frame`, and loads libadwaita (`adw_init`) when `XDG_CURRENT_DESKTOP` contains GNOME (or `MUTTER_FRAMES_PLATFORM_LIBRARY=adwaita`), so libadwaita's `.default-decoration` (GN-1.11, GN-1.21) and `.ssd-frame` (GN-1.06, GN-1.08) rules draw it; otherwise the GTK 4 theme GTK loads (Default when `gtk-theme` names no GTK 4 theme, GN-P3-17†) | published `M/src/frames/main.c:35-71`, `:104-111`; `M/src/frames/meta-frame-header.c:117-122`; `M/src/frames/meta-frame.c:568`, `:587` | `window.title_bar_*` (review 2026-10-28: "to verify which theme mutter-x11-frames uses" settled from mutter 51.0; source kind measure → published; the `titlebar-font` mention dropped, see GN-1.19) |
| GN-1.278† | Back button (AdwBackButton) in a header bar inside AdwNavigationView | `go-previous-symbolic` image button with the `.back` class, flat like every header-bar button; tooltip = the previous page's title ("Back" without one); long press or right click opens a navigation menu (popover menu, no arrow, start-aligned) listing the pages to pop to | published `A/c/adw-back-button.c:323-326`, `:345-376`, `:399-416`, `:527-529`; `A/c/adw-header-bar.c:136-138`, `:349-360` | none |
| GN-1.282† | CSD resize zone | The band outside the visible frame where the pointer resizes the window: the window's CSS border plus its shadow extent, at least `RESIZE_HANDLE_SIZE` 12 px per side; along each edge the corner zone reaches `RESIZE_HANDLE_CORNER_SIZE` 24 px, or the corner radius if larger; none when maximised, fullscreen, or tiled without edge constraints; resize cursors per edge (GN-3.67†) | published `G/gtkwindow.c:191-192`, `:1548-1596`, `:4196-4201` | none |

### Button (`[button]`)

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.24 | Button, checked + hover / checked + pressed | 35 % and 40 % currentColor fills | published `A/w/_buttons.scss:5-6`, `:44-55` | `button.checked_background` (rest only) |
| GN-1.25 | Button, drop target | `:drop(active)`: text in accent, inset 2px accent ring | published `A/w/_buttons.scss:104-108`; `A/s/_colors.scss:301` | none |
| GN-1.26 | Button, `.has-open-popup` | menu button with its popup open (drawn as checked/hover per variant) | published `A/w/_lists.scss:81`, `:88`; `A/w/_buttons.scss:354` | none |
| GN-1.27 | Button padding per content | Image-only (min-width 24, 5 side), image+text (9, inner spacing 4, label padding 2), arrow button (9, spacing 4); the model has one padding per side (text 17 and base 10 sit in platform-facts) | published `A/w/_buttons.scss:66-102` | `button.border.padding_*` (one set) |
| GN-1.28 | Button shapes / size variants | `.circular` (34×34, radius 9999, padding 0), `.pill` (padding 10/32, radius 9999) | published `A/w/_buttons.scss:312-326`; menubutton `:404-406`, `:427-429` | `button.border.corner_radius` (one radius) |
| GN-1.29 | Button variant `.flat` (default in header bars and toolbars) | Transparent rest; hover 7 %, pressed 16 %, checked 10 %, checked+hover 13 %, checked+pressed 19 %; disabled-unchecked at `$strong_disabled_opacity` 30 %; HC inset ring on hover/pressed/checked | published `A/w/_buttons.scss:234-280`; `A/s/_colors.scss:268-272`, `:311`; toolbar mapping `A/w/_toolbars.scss:1-27` | none (seed: gpui `geometry.rs:291` "the model states no flat-button frame") |
| GN-1.30 | Button variant `.suggested-action` | Primary hover / pressed / checked / checked+hover / checked+pressed overlays (`image()` layers of 10 % currentColor, `RGB(0 0 6 / 20%)`, 15 %, 5 %, 30 %), outer focus ring offset 1 | published `A/w/_buttons.scss:155-181`, `:220-228` | `button.primary_background`, `button.primary_text_color` (rest only; seed: iced `contract/rows.rs:470` "no hovered and no pressed variant of a primary") |
| GN-1.31 | Button variant `.destructive-action` | Whole variant: text in `--destructive-color` (standalone red), fill destructive currentColor 15 %, hover 20 %, pressed 35 %, checked 35 / 40 / 45 % | published `A/w/_buttons.scss:10-15`, `:183-211`, `:230-232` | `defaults.danger_color` (a status colour, no button variant) |
| GN-1.32 | Button variant `.opaque` | Fill `color-mix(window-bg 85 %, window-fg)`, window fg text, opaque hover/pressed overlays | published `A/w/_buttons.scss:8`, `:213-218` | none |
| GN-1.33 | Button variant `.raised` | Restores the raised look inside toolbars / header bars | published `A/w/_toolbars.scss:20-40` | none |
| GN-1.34 | Button variant `.osd` | 32×32 minimum, black 65 % fill, white 90 % text, hover / pressed / checked mixes, outer focus ring offset 1, HC 1px currentColor ring | published `A/w/_buttons.scss:112-153`; `A/s/_colors.scss:291-292` | none |
| GN-1.35 | Button variant `.card` (activatable card button) | Card fill, hover 4 % / pressed 8 % view overlays, checked accent 25 / 32 / 39 %, focus offset −1 | published `A/w/_buttons.scss:328-365`; `A/s/_colors.scss:274-278` | none |
| GN-1.36 | Button motion | `background` and `box-shadow` 200 ms `ease-out-quad` (`cubic-bezier(0.25, 0.46, 0.45, 0.94)`) | published `A/s/_common.scss:1`, `:6-7`; `A/w/_buttons.scss:26` | none |
| GN-1.37 | Button focus ring placement | Inset −2 on plain/flat buttons; outer offset 1 on `.osd`, `.suggested-action`, `.opaque`; −1 on `.card` | published `A/w/_buttons.scss:26`, `:116`, `:158`, `:237`, `:334`; mixin `A/s/_drawing.scss:12-28` | `defaults.focus_ring_offset` (one global value) |
| GN-1.38 | Button HC | Inset 1px `$border_color` ring at rest | published `A/w/_buttons.scss:31-33` | `button.border.*` (no HC variant; platform-facts records it as prose) |
| GN-1.39 | Button content (AdwButtonContent) | Icon–label spacing 6 plus label end padding 2; bold label; padding drop next to an arrow | published `A/w/_buttons.scss:626-645` | `button.icon_text_gap` (one number; the label-padding part is not separable) |
| GN-1.40 | Menu button arrow | Arrow glyph per direction (`pan-down/up/start/end-symbolic`, `open-menu-symbolic` for none), 16×16 | published `A/w/_buttons.scss:451-469` | none |
| GN-1.41 | Button "needs attention" dot | 6×6 accent radial dot at `right 3px`, appears with `needs_attention` 150 ms ease-in animation (stack switcher, view switcher) | published `A/s/_common.scss:106-125`; `A/w/_buttons.scss:299-302` | none |
| GN-1.42 | Linked buttons (`.linked`) | Joined group: inner corners squared, −1px overlap, horizontal and vertical | published `A/w/_linked.scss:2-46` | none |

### Text input (`[input]`, `[text_area]`)

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.43 | Entry icons | Gap between icon and text (`border-spacing: 6px`; icon margins 6) | published `A/w/_entries.scss:8`, `:59-60` | none |
| GN-1.44 | Entry icons | Icon opacity per state: 70 % rest, 100 % hover, 80 % pressed; HC 85 % / 90 % | published `A/w/_entries.scss:42-57` | none |
| GN-1.45 | Entry, invalid / error state | `.error` (and `.warning`, `.success`) recolours text and focus ring through `--accent-color` → `--error-color` | published `A/w/_labels.scss:21-29`; entries inherit via `A/s/_drawing.scss:20` (ring uses `--accent-color`) | none (iced `contract/rows.rs` has no invalid row either) |
| GN-1.46 | Entry, drop target | Inset 1px accent ring | published `A/w/_entries.scss:71-76` | none |
| GN-1.47 | Entry `.flat` variant | No fill, no radius, padding 2, min-height 0 | published `A/w/_entries.scss:30-38` | none |
| GN-1.48 | Entry progress (GtkEntry progress-fraction) | 2px accent bottom line, margin-bottom 3 | published `A/w/_entries.scss:79-92` | none |
| GN-1.49 | Overwrite-mode block caret of entries (GtkText's `block-cursor` node) | `block-cursor` colours: view bg on view fg | published `A/w/_entries.scss:24-27` | `input.caret_color` (line caret only) (review 2026-10-28: the rule is the entries' only; GtkTextView's and GtkSourceView's block caret is GN-P4-4†) |
| GN-1.50 | Entry focus | Focus ring on `:focus-within` with the 200 ms focus transition | published `A/w/_entries.scss:17`; `A/s/_common.scss:3-5` | `input.focus_border_color` (colour only) |
| GN-1.51 | Entry HC | Inset 1px `$border_color` ring | published `A/w/_entries.scss:13-15` | `input.border.*` (no HC variant) |
| GN-1.52 | Text view gutters | `textview > border` (gutter windows) fill `mix(window-bg, view-bg 50 %)` | published `A/w/_views.scss:38-40` | none |
| GN-1.53 | Text view, drop target | Caret turns accent | published `A/w/_views.scss:42-44` | none |
| GN-1.54 | Inline text view (`textview.inline`) | Transparent, inherited colour | published `A/w/_views.scss:31-36` | none |
| GN-1.281† | IME pre-edit text (GtkText, GtkTextView) | The uncommitted composition: without compositor style hints, one single underline under the whole pre-edit; with `text-input-v3` hints GTK styles a `preedit` CSS node per hint with the classes `.whole`, `.selection`, `.prediction`, `.prefix`, `.suffix`, `.spelling-error`, `.compose-error` — libadwaita 1.10.0 has no `preedit` rule (grep of `src/stylesheet`), so what each hint looks like: measure (probe with an input method); GtkIMContextSimple (compose sequences) underlines its pre-edit | published `G/gtkimcontextwayland.c:764-781`, `:884-896`; `G/gtkimcontextsimple.c:1362` | none |

### Checkbox / radio (`[checkbox]`)

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.55 | Unchecked + pressed | Fill `$trough_active_color` (25 %) and no ring | published `A/w/_checks.scss:42-50`; `A/s/_colors.scss:282` | none |
| GN-1.56 | Unchecked + hover | The ring colour changes (`$trough_hover_color`), not a fill — model has a hover fill only | published `A/w/_checks.scss:33-40` | `checkbox.hover_background` (fill), no hover border colour |
| GN-1.57 | Checked + hover, checked + pressed | Overlays: 10 % currentColor (accent fg) on hover, `RGB(0 0 6 / 20%)` pressed | published `A/w/_checks.scss:52-60` | none (seed: gpui `widgets/checkbox.rs:49` "the model states no hover for a checked one"; iced `contract/rows.rs:399`) |
| GN-1.58 | Indeterminate (mixed) state | Accent fill, `dash.svg` glyph (its geometry) | published `A/w/_checks.scss:52-56`, `:79-81`; asset `src/stylesheet/assets/dash.svg` | none |
| GN-1.59 | Unchecked ring width | Withdrawn (review 2026-10-28): false gap — `checkbox.border.line_width` exists and a 2 px inset ring is a 2 px border drawn inside the box; the 2 against the inherited 1 is a value for `platform-facts.md` §2.5, not a model gap. Text kept: The ring is a 2px inset shadow (model's line width inherits 1) | published `A/w/_checks.scss:27` | `checkbox.border.line_width` (field exists; platform-facts §2.5 inherits `defaults.border.line_width` 1 — a value question, listed so the 2px ring is not missed) |
| GN-1.60 | Check corner radius vs radio | Check radius `$check_radius` 6, radio 100 % | published `A/w/_checks.scss:1`, `:67-77` | `checkbox.border.corner_radius` (one radius; radio is implicit) |
| GN-1.61 | Whole checkbutton (indicator + label) | Its own padding 3, radius 9 and focus ring around indicator **and** label; `.text-button` padding 4; label gap `border-spacing: 4px` | published `A/w/_checks.scss:3-17` | `checkbox.label_gap`, `checkbox.border.padding_*` (indicator only) |
| GN-1.62 | Selection-mode checks (`checkbutton.selection-mode`) | Round, padding 7, label margin 6 | published `A/w/_checks.scss:83-95` | none |
| GN-1.63 | Grouped checkbutton (`.grouped`) | Round container radius 999 | published `A/w/_checks.scss:15-17` | none |
| GN-1.64 | Check motion | 200 ms `ease-out-quad` on background and ring | published `A/w/_checks.scss:22`; `A/s/_common.scss:6-7` | none |
| GN-1.65 | Checkbox HC | Ring `$border_color`, hover ring 60 %, pressed fill 60 % | published `A/w/_checks.scss:29-49` | none |

### Menu (`[menu]`)

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.66 | Menu item, pressed | `:active` fill `$selected_active_color` (19 %) | published `A/w/_menus.scss:146-148` | `menu.hover_background` (hover only) |
| GN-1.67 | Menu item, keyboard-selected | `:selected` drawn as hover | published `A/w/_menus.scss:141-144` | `menu.hover_background` (same value; state not named) |
| GN-1.68 | Menu item corner radius | Items are rounded (`$menu_radius` 9) inside the popup | published `A/w/_menus.scss:134-139`; `A/s/_common.scss:10` | `menu.border.corner_radius` (registry uses `menu.border` for the popup; no item radius) |
| GN-1.69 | Menu popup inner padding | `$menu_margin` 6 around the item box, contents padding 0 | published `A/w/_menus.scss:58-66`; `A/s/_common.scss:11` | none (popover padding is 8: §2.16) |
| GN-1.70 | Menu min width | `popover.menu > contents { min-width: 120px }`; item min-width 40 | published `A/w/_menus.scss:1-4`, `:134-136` | none (seed: gpui `demo.rs:8194` "the model states no menu width") |
| GN-1.71 | Menu separator | Margin 6 0 | published `A/w/_menus.scss:68-74` | `menu.separator_color` (colour only) |
| GN-1.72 | Accelerator label | Dimmed (`--dim-opacity`), start margin 12 | published `A/w/_menus.scss:76-81` | none |
| GN-1.73 | Check / radio items | 14px indicator, padding 2, no fill, radio 1px border at `$dimmer_opacity` (30 %, HC 80 %), margins 6/−2 | published `A/w/_menus.scss:83-132` | none |
| GN-1.74 | Submenu arrow | `go-next-symbolic` 16px at `$dimmer_opacity` 30 % (HC 80 %) | published `A/w/_menus.scss:158-178` | none |
| GN-1.75 | Submenu header (sliding submenu) | Back arrow `go-previous-symbolic`, bold title, padding 4 / 32 | published `A/w/_menus.scss:174-183` | none |
| GN-1.76 | Submenu slide | GtkPopoverMenu's stack slides left/right (`SLIDE_LEFT_RIGHT`), duration GtkStack's default 200 ms: the menu sets no duration (no `transition-duration` / `transition_duration` in `G/gtkpopovermenu*` or `G/gtkmenusectionbox.c`, grep) | published `G/gtkpopovermenu.c:336-340`; `G/gtkstack.c:1021-1023`, `:2953` | none (review 2026-10-28: "to verify the menu does not override it" settled) |
| GN-1.77 | Submenu hover-open delay | `OPEN_TIMEOUT` 80 ms, only in nested popover menus (menu bar) and only while another submenu is open; with none open the submenu opens at once (the rule: GN-P4-6†) | published `G/gtkmodelbutton.c:1423`, `:1458-1466`; `G/gtkmenusectionbox.c:345` | none (review 2026-10-28: the delay's scope added) |
| GN-1.78 | Inline icon-button row / circular-button row in menus | 32×32 flat icon buttons; circular 11-padded buttons with 1px ring, selected / pressed fills | published `A/w/_menus.scss:10-56` | none |
| GN-1.79 | List rows inside a popover menu | Padding 9 12 / 0 12, radius 9, hover/selected/pressed fills | published `A/w/_menus.scss:185-246` | none |
| GN-1.80 | Menu HC | Inset 1px ring on hover / selected / pressed items | published `A/w/_menus.scss:150-156` | none |
| GN-1.285† | Menu item icons | A popover-menu item shows its icon only in an iconic (button-row) section or when it has no label: a labelled item never shows an icon, so a GTK menu has no icon column | published `G/gtkmodelbutton.c:646-667` | `menu.icon_size_px`, `icon_text_gap_px` (assume an icon beside the label) |

### Tooltip (`[tooltip]`)

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.81 | Tooltip content | Icon–text spacing 6 | published `A/w/_tooltip.scss:17-18` | none |
| GN-1.82 | Tooltip browse mode | `BROWSE_TIMEOUT` 60 ms (next tooltip while one is up), `BROWSE_DISABLE_TIMEOUT` 500 ms | published `G/gtktooltip.c:71-73` | `tooltip.show_delay` (first delay only) |
| GN-1.83 | Tooltip placement | Below the widget and centred on it, flipped above when there is no room, slid sideways to stay on screen; the anchor is the widget's bounds grown by 4 px on each side (pointer-triggered: `MAX (4, cursor size − 32)`). Pointer-triggered: when that anchor is taller than 48 + 8 px, a cursor-size square at the pointer − 4 is the anchor instead; otherwise the tooltip is shifted sideways so its centre is at most 32 px from the pointer | published `G/gtktooltip.c:581-689` | none (review 2026-10-28: "measure … to verify" settled from `G/gtktooltip.c:617-687`) |
| GN-1.283† | Tooltip width limit | The label wraps at `max-width-chars` 50: a width in characters of the tooltip font, not pixels (platform-facts §2.7 `max_width` is "(none)" for GNOME, the preset carries 360) | published `G/ui/gtktooltipwindow.ui:17-18` | `tooltip.max_width` (pixels) |

### Scrollbar (`[scrollbar]`)

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.84 | Thumb shape | Pill radius (`99px`) on thumb and trough | published `A/w/_scrolling.scss:9`, `:16` | none |
| GN-1.85 | Overlay indicator, collapsed | Narrow thumb 3px when not hovered, trough margins 4 | published `A/w/_scrolling.scss:66-111` | `scrollbar.thumb_width` (expanded width only) |
| GN-1.86 | Overlay indicator outline | Thumb outline 1px in `--scrollbar-outline-color` at 60 % (35 % collapsed), offset −8; HC full / 60 % | published `A/w/_scrolling.scss:53-75`; `A/s/_colors.scss:80`, `:142` | none |
| GN-1.87 | Overlay indicator, hovered track | Trough appears (10 % currentColor; HC 25 %) | published `A/w/_scrolling.scss:76-82` | `scrollbar.track_color` (one colour, not per state) |
| GN-1.88 | Fine-tune state | Thumb in accent 60 % (HC 100 %) during fine scrolling | published `A/w/_scrolling.scss:118-127` | none |
| GN-1.89 | Disabled thumb | Opacity 0 | published `A/w/_scrolling.scss:33` | none |
| GN-1.90 | Scrollbar motion | `all 200ms linear` on trough and thumb (size and colour) | published `A/w/_scrolling.scss:4`, `:8`, `:18` | none |
| GN-1.91 | Overlay auto-hide timings | Fade-out delay 2000 ms, fade 1000 ms (ease-out cubic), hide 500 ms after leave | published `G/gtkscrolledwindow.c:191-196`, `:3765`, `:3787`, `:3816` | `scrollbar.overlay_mode` (on/off only) |
| GN-1.92 | Thumb in popover menus | Min length 26 | published `A/w/_menus.scss:6-8` | `scrollbar.min_thumb_length` (one value) |
| GN-1.93 | Scrollbar HC | Thumb 40 / 60 / 100 % | published `A/w/_scrolling.scss:25-31` | none |

### Slider (`[slider]`)

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.94 | Track and fill shape | Pill radius 99 on trough and highlight | published `A/w/_scale.scss:1-17` | none |
| GN-1.95 | Slider widget padding | `padding: 12px` around the trough | published `A/w/_scale.scss:27` | none |
| GN-1.96 | Thumb outline | `0 0 0 1px ---slider-border-color` (`RGB(0 0 6 / 10%)`, HC 35 %) | published `A/w/_scale.scss:31-35`, `:51` | none (seed: gpui `info/inputs.rs:1445` "the model states no thumb outline") |
| GN-1.97 | Thumb shadow | `0 2px 4px RGB(0 0 6 / 20%)`; disabled → transparent; marked slider `1px 1px 4px` | published `A/w/_scale.scss:51`, `:83-85`, `:105` | none |
| GN-1.98 | Track / fill hover and pressed | Trough `$trough_hover_color`, highlight +10 % overlay on `:hover`/`:active` | published `A/w/_scale.scss:62-78` | `slider.thumb_hover_color` (thumb only) |
| GN-1.99 | Thumb pressed | Same as hover (white) | published `A/w/_scale.scss:62`, `:74-76` | none |
| GN-1.100 | Marks (ticks) | Mark indicator 1×6 px, colour at `--dim-opacity`, distance 6 from trough; fine-tune 3 px shorter | published `A/w/_scale.scss:21-23`, `:113-124`, `:144-149` | `slider.tick_mark_length` (length only; see Appendix) |
| GN-1.101 | Thumb shape with marks | Rotated 45° teardrop, one square corner, when marks are on one side | published `A/w/_scale.scss:98-111`, `:152-170` | none |
| GN-1.102 | Value label (`draw-value`) | Dimmed, tabular figures, margin 9 (12 = marks size when marked) | published `A/w/_scale.scss:93-96`, `:126-135` | none |
| GN-1.103 | Fill level (`show-fill-level`) | `> fill` drawn as a second trough | published `A/w/_scale.scss:45-47` | none |
| GN-1.104 | Slider focus ring | On the thumb, offset 0 | published `A/w/_scale.scss:29` | `defaults.focus_ring_*` |
| GN-1.105 | Disabled thumb outline | `outline-color` 20 % (HC 50 %) | published `A/w/_scale.scss:80-91` | none |
| GN-1.106 | Slider motion | 200 ms ease-out-quad on trough and highlight | published `A/w/_scale.scss:2`, `:13` | none |
| GN-1.107 | Vertical slider | Mark and value placement mirrored (sizes as horizontal) | published `A/w/_scale.scss:173-227` | none (no orientation) |
| GN-1.108 | Slider HC | Trough 30 % + inset 1px border, hover 40 % | published `A/w/_scale.scss:6-9`, `:66-68` | none |

### Progress bar (`[progress_bar]`)

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.109 | Track and fill radius | Pill 99 on trough and progress, fill end radii | published `A/w/_scale.scss:3`, `:14`; `A/w/_progress-bar.scss:30-56` | `progress_bar.border.corner_radius` (one radius, inherits 9) |
| GN-1.110 | Progress text (`show-text`) | Dimmed, `font-size: smaller`, tabular figures | published `A/w/_progress-bar.scss:17-21` | none |
| GN-1.111 | Vertical orientation | Min height 80, width 8 | published `A/w/_progress-bar.scss:10-15` | `progress_bar.track_height`, `min_width` (horizontal only) |
| GN-1.112 | `.osd` variant | 2px, no trough, square ends | published `A/w/_progress-bar.scss:59-83` | none |
| GN-1.113 | Indeterminate (pulse) mode | Block is 1/`activity_blocks` (5) of the bar, `pulse-step` 0.1, its motion | published `G/gtkprogressbar.c:473-476`, `:408-416` | none (seed: gpui `widgets/progress.rs:139` "the model states no motion") |
| GN-1.114 | Empty bar | Progress node hidden at fraction 0 | published `A/w/_progress-bar.scss:85` | none |
| GN-1.115 | Progress HC | Trough 30 % + inset border | published `A/w/_scale.scss:6-9` | none |

### Tab bar (`[tab]`) — GtkNotebook and AdwTabBar

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.116 | Notebook header rule | 1px `$border_color` line on the page side of the header | published `A/w/_notebook.scss:4-10`, `:12-58` | none (seed: gpui `info/layout.rs:941` "no tab-bar rule of its own") |
| GN-1.117 | Notebook tab hover indicator | Hover draws the 4px inset bar in `$border_color` | published `A/w/_notebook.scss:17`, `:29`, `:41`, `:53` | `tab.active_indicator_color` (checked only) |
| GN-1.118 | Indicator sides Left / Right | Vertical notebooks draw the bar on the left or right | published `A/w/_notebook.scss:36-58` | `tab.active_indicator_side` (Top / Bottom only) |
| GN-1.119 | Notebook scroll arrows | Flat 16px arrows with `pan-*` glyphs, padding 4, margin −5 | published `A/w/_notebook.scss:60-117` | none |
| GN-1.120 | Close button in a tab | Flat, 20×20, text at 30 % → full on hover, margins 4 | published `A/w/_notebook.scss:144-164` | none |
| GN-1.121 | Notebook strip padding | Tabs padding 4, inter-tab margin 4 each side, tab bottom padding 4 | published `A/w/_notebook.scss:167-210` | `tab.item_gap` (see Appendix), no strip padding |
| GN-1.122 | Notebook page background | Page stack in `--view-bg-color` | published `A/w/_notebook.scss:213-215` | none |
| GN-1.123 | Notebook disabled tab | Opacity filter, except label and button | published `A/w/_notebook.scss:135-141` | `disabled_opacity` (global) |
| GN-1.124 | AdwTabBar tab | Min height 26, padding 4, radius 9; selected 10 %, selected+hover 13 %, selected+pressed 19 %; hover 7 %, pressed 16 %; single-tab no fill | published `A/w/_tab-view.scss:58-88`, `:155-181` | `tab.*` (no pressed, no selected+hover, no selected+pressed) |
| GN-1.125 | AdwTabBar strip | Fill / text in header-bar colours, bottom shade line, padding 1 (top 0), tab box padding 6, min height 34; start/end action padding; backdrop fill + 50 % content; inline and shrunk variants | published `A/w/_tab-view.scss:1-24`, `:26-29`, `:90-136` | `tab.bar_background` (colour only) |
| GN-1.126 | AdwTabBar separators | Between tabs, margins 3, hidden next to the selected tab with a 150 ms ease-in-out fade | published `A/w/_tab-view.scss:36-44` | none |
| GN-1.127 | AdwTabBar drop indicator | 2px accent-at-50 % bar (HC 100 %), margins 3 6 | published `A/w/_tab-view.scss:46-55` | none |
| GN-1.128 | AdwTabBar attention indicator | 2px accent-at-50 % line under the tab, `translateY(4px)`; width 0.6 × tab, min 20, max 180, pinned 14; 250 ms ease-in-out | published `A/w/_tab-view.scss:170-180`; `A/c/adw-tab.c:24-28`, `:868-872` | none |
| GN-1.129 | AdwTabBar tab widths | Base width 118, pinned 26, max non-expanding 220, label fade 18 / 36 px at the edges | published `A/c/adw-tab.c:18-22`; `A/c/adw-tab-box.c:38-41` | `tab.min_width` (no max, no base, no pinned) |
| GN-1.130 | AdwTabBar close button | 24×24 round flat button; shows/hides with 150 ms ease-in-out | published `A/w/_tab-view.scss:162-168`; `A/c/adw-tab.c:19`, `:858-862` | none |
| GN-1.131 | AdwTabBar motion | Open / close 200 ms, reorder 250 ms, scroll 200 ms, resize 200 ms, all `ADW_EASE`; tab background 150 ms ease-in-out | published `A/c/adw-tab-box.c:30-36`; `A/w/_tab-view.scss:59` | none |
| GN-1.132 | AdwTabBar drag-hover switch | `DROP_SWITCH_TIMEOUT` 500 ms | published `A/c/adw-tab-box.c:26` | none |
| GN-1.133 | Dragged tab (`dnd tab`) | Header-bar fill + 19 % overlay, three-layer shadow, margin 25, HC outline | published `A/w/_tab-view.scss:138-153` | none |
| GN-1.134 | Tab focus ring | AdwTabBar: on the tab box child (`:focus-within:focus-visible`), radius 9; notebook: on the checked tab, offset −2 | published `A/w/_tab-view.scss:31-34`; `A/w/_notebook.scss:2` | `defaults.focus_ring_*` |
| GN-1.135 | Tab HC | Inset 1px border on hover / pressed / selected AdwTabBar tabs | published `A/w/_tab-view.scss:61-67` | none |

### Sidebar (`[sidebar]`)

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.136 | Sidebar, window unfocused | `--sidebar-backdrop-color` with a 200 ms ease-out transition | published `A/w/_sidebars.scss:5-8`; `A/s/_colors.scss:50`, `:112` | none |
| GN-1.137 | Secondary (middle) sidebar of three panes | Its own bg / fg / backdrop / border / shade colours | published `A/w/_sidebars.scss:42-83`; `A/s/_colors.scss:54-58`, `:116-120` | none |
| GN-1.138 | Sidebar shade colour | `--sidebar-shade-color` (used by undershoot and dimming inside it) | published `A/w/_sidebars.scss:10`; `A/s/_colors.scss:51`, `:113` | `defaults.shadow_color` (one shade for all) |
| GN-1.139 | Sidebar edge side | The 1px edge is on the content side and flips with `.end` and RTL | published `A/w/_sidebars.scss:21-40` | `sidebar.border` (no side) |
| GN-1.140 | Sidebar row geometry | Row radius 9, min height 36, padding 0 8 (AdwSidebar box 3 14, spacing 12), margin 0 6 2 | published `A/w/_sidebars.scss:189-201`, `:224-239` | none (no row height or item radius in `[sidebar]`) |
| GN-1.141 | Sidebar row states | Hover 7 %, pressed 16 %, selected 10 % (neutral, not accent), selected+hover 13 %, selected+pressed 19 %, has-open-popup | published `A/w/_sidebars.scss:126-147`; `A/s/_colors.scss:268-272` | `sidebar.hover_background`, `sidebar.selection_background` (no pressed, no combinations) |
| GN-1.142 | Sidebar section headings | `.header > box` margin 6, heading margin 6, first header no top margin; separators margin 6 | published `A/w/_sidebars.scss:152-170` | none |
| GN-1.143 | Sidebar grid items | `child` / `flowboxchild` radius 9, margin 3, padding 8 | published `A/w/_sidebars.scss:196-205` | none |
| GN-1.144 | AdwSidebar prefix / suffix areas | Margins 6 | published `A/w/_sidebars.scss:211-222` | none |
| GN-1.145 | Sidebar drag-hover activation | `TIMEOUT_ACTIVATE` 500 ms | published `A/c/adw-sidebar.c:23` | none |
| GN-1.146 | Sidebar banner | Banner mixed with the sidebar bg, its own edge | published `A/w/_sidebars.scss:12-19` | none |
| GN-1.147 | Sidebar width | NavigationSplitView / OverlaySplitView: min 180, max 280, fraction 0.25 (both), in `sp` by default | published `A/c/adw-navigation-split-view.c:934`, `:952`, `:971`, `:1007-1008`; `A/c/adw-overlay-split-view.c:1037`, `:1055`, `:1074` | none (seed: gpui `main.rs:332`, egui `main.rs:41` "the model states no side-panel width") (review 2026-10-28: the navigation fraction, "to verify", is 0.25; the `sp` unit added) |
| GN-1.148 | Sidebar HC | Inset 1px ring on selected / hover / pressed rows | published `A/w/_sidebars.scss:172-187` | none |

### Toolbar (`[toolbar]`) and AdwToolbarView bars

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.149 | `.toolbar.osd` | Padding 12, radius 15, OSD colours | published `A/w/_toolbars.scss:106-110` | `toolbar.border.corner_radius` (no variant) |
| GN-1.150 | Shrunk toolbar (`.collapse-spacing`) | Vertical padding 3 | published `A/w/_toolbars.scss:112-115`, `:284-296` | none |
| GN-1.151 | Toolbar children styling | Buttons, menu buttons, split buttons and drop-downs become flat; raised inside popovers | published `A/w/_toolbars.scss:1-97` | none |
| GN-1.152 | Switch inside a toolbar | Vertical margins 4 | published `A/w/_toolbars.scss:72-75` | none |
| GN-1.153 | AdwToolbarView raised top / bottom bar | Header-bar fill, two-layer shadow (`0 ±1px`, `0 ±2px 4px`, shade 50 %), HC variant | published `A/w/_toolbars.scss:298-348` | `toolbar.border.shadow_enabled` (flag only) |
| GN-1.154 | AdwToolbarView `.border` style | 1px `--headerbar-darker-shade-color` line instead of the shadow | published `A/w/_toolbars.scss:323-330`, `:341-347` | none |
| GN-1.155 | Bars, window unfocused | Raised bar backdrop fill and content at opacity 0.5, 200 ms | published `A/w/_toolbars.scss:302-311` | none |
| GN-1.156 | Toolbar view undershoot | Scroll edge shade under the bars | published `A/w/_toolbars.scss:350-351`; `A/s/_drawing.scss:30-71` | none |

### Status bar (`[status_bar]`)

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.157 | Status bar / its GNOME successor | GtkStatusbar is deprecated; GNOME's bottom bar is GtkActionBar / AdwToolbarView bottom bar (§4.2 GN-2.20): top shade line, header-bar fill, padding 7 6 6 6, spacing 6, backdrop | published `A/w/_toolbars.scss:199-246` | `status_bar.*` (no top line distinct from border, no item gap — seed: gpui `info/chrome.rs:1164` "no status-bar item gap") |

### List / table (`[list]`), column view, tree expander

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.158 | Row pressed | `$view_active_color` 8 % | published `A/w/_lists.scss:75-81`; `A/s/_colors.scss:275` | none |
| GN-1.159 | Selected + hover / selected + pressed | Accent 32 % / 39 % | published `A/w/_lists.scss:83-89`; `A/s/_colors.scss:277-278` | `list.selection_background` (rest only) |
| GN-1.160 | Row with open popup | `.has-open-popup` drawn as hover (selected: as selected+hover) | published `A/w/_lists.scss:81`, `:88` | none |
| GN-1.161 | Row focus ring | Inset on the row (`focus-ring()` default −2) | published `A/w/_lists.scss:47-48` | `defaults.focus_ring_*` |
| GN-1.162 | Row separators | `.separators` lists and `row.separator`: 1px `$border_color` bottom (left in horizontal lists) | published `A/w/_lists.scss:24-32` | `list.grid_color` (colour only, no variant) |
| GN-1.163 | List section header (`listview > header`) | `.heading` font, padding 18 top / 6 bottom / 2 sides; rich-list headers 12 sides | published `A/w/_lists.scss:40-45`, `:113-116` | none |
| GN-1.164 | Rich list | Row padding 8 12, min-height 32, inner spacing 12 (a variant) | published `A/w/_lists.scss:103-117` | `list.row_height`, `list.border.padding_*` (one variant) |
| GN-1.165 | Boxed list (`.boxed-list`) | Card frame, rows separated by 1px `--card-shade-color`, row hover 3 % / pressed 8 %, outer rows rounded 12, focus ring −1, 200 ms row transitions | published `A/w/_lists.scss:1-3`, `:343-365`, `:520-554`; `A/w/_misc.scss:201-208` | `card.*` + `list.*` (no boxed-list variant) |
| GN-1.166 | Separate boxed list (`.boxed-list-separate`) | Each row a card, 12 apart, card-fill hover / pressed overlays, selected accent 25 %, drop-target shadow stack | published `A/w/_lists.scss:556-633` | none |
| GN-1.167 | Column view header button | Text at 40 % (HC 60 %), hover 70 % (HC 90 %), pressed 100 %; `.caption-heading` font; padding 3 6; line-height 100 % | published `A/w/_column-view.scss:12-48` | `list.header_font`, `header_background` (no header states) |
| GN-1.168 | Sort indicator | `pan-up` / `pan-down` 16px | published `A/w/_column-view.scss:50-60` | none |
| GN-1.169 | Column separators | `.column-separators`: 1px `$border_color` between cells and headers | published `A/w/_column-view.scss:95-100` | `list.grid_color` |
| GN-1.170 | Cell padding | Cell 8 6; `.data-table` 2 vertical | published `A/w/_column-view.scss:81-111` | `list.border.padding_*` (row, not cell) |
| GN-1.171 | Header being dragged (`button.dnd`) | Accent fill and accent fg | published `A/w/_column-view.scss:63-71` | none |
| GN-1.172 | Editable cell focus | 2px `$focus_border_color` outline; editing: 2px `--accent-color` | published `A/w/_column-view.scss:116-126` | none |
| GN-1.173 | Tree expander (GtkTreeExpander) | Expander arrow (`pan-end` / `pan-down`, 16px), spacing 4, per-depth indent (`indent` nodes) | published `A/w/_column-view.scss:128-130`, `A/w/_expanders.scss:1-10`; indent width: measure (probe) — `G/gtktreeexpander.c:95-105` describes the nodes | none (platform-facts §2.15 note calls it toolkit-internal) |
| GN-1.174 | Grid view / flow box items | Padding 3, radius 9, hover 4 %, pressed 8 %, selected accent 25 / 32 / 39 %, focus ring | published `A/w/_views.scss:52-91` | none |
| GN-1.175 | Rubberband selection | 1px accent border, accent 20 % fill | published `A/w/_views.scss:47-50` | none |
| GN-1.176 | `.view` disabled | Text at disabled opacity, fill `mix(window-bg 60 %, view-bg)` | published `A/w/_views.scss:6-9` | `list.disabled_text_color` (no disabled fill) |
| GN-1.177 | View selected cell radius | `.view:selected` radius 9 | published `A/w/_views.scss:11-17` | none |
| GN-1.178 | List undershoot | Column view top undershoot (shade line + 4px gradient) | published `A/w/_column-view.scss:113`; `A/s/_drawing.scss:30-71` | none |
| GN-1.279† | GtkTreeView (deprecated since 4.10, still drawn by older apps and inside GtkEntryCompletion) | Styled as a column view, plus: tree lines (`enable-tree-lines`, off by default) in `border-left-color` and grid lines (`enable-grid-lines`, none by default) in `border-top-color`, both currentColor 20 % over the view bg (HC 40 %), 1 px, dotted (a 2-px texture with one pixel coloured, repeated); per-depth indent = expander size (16 + 2: the larger of the expander's min-width / min-height plus half the 4-px horizontal separator) + `level-indentation` (default 0); expander `pan-end` / `pan-down` 16 px at 70 %, 100 % on hover and pressed; selected rows square; drop indicator a 1 px accent line before or after a row (a box when dropping into it); header buttons 4 px side padding and a 1 px bottom border; cell progress bars (accent fill, 10 % trough), cell editing entries and spin buttons (square, view bg, accent border when focused); interactive-search popover hides after 5 s; drag-hover expand 500 ms | published `A/w/_deprecated.scss:206-337`; `G/deprecated/gtktreeview.c:274-275`, `:284-290`, `:1652-1657`, `:2655-2681`, `:2834-2845`, `:4150-4298` | `list.*`, `list.grid_color` (one solid colour, no tree lines) |
| GN-1.280† | GtkIconView (deprecated since 4.10) | View fill, default inset focus ring, HC 1 px inset ring on selected items, drop target as a 1 px accent box; item padding 6, margin 6, row and column spacing 6, item text spacing 0 (C defaults); `rubberband` node as GN-1.175 | published `A/w/_deprecated.scss:112-144`; `G/deprecated/gtkiconview.c:442-527`, `:920-925` | `list.*` (no grid-item spacing) |

### Popover (`[popover]`)

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.179 | Popover shadow | Three layers: `0 0 0 1px` 5 %, `0 1px 5px 1px` 9 %, `0 2px 14px 3px` 5 % (the first is the outline) | published `A/w/_popovers.scss:19-27` | `popover.border.shadow_enabled`, `defaults.shadow_color` |
| GN-1.180 | Popover arrow (tail) | Tail 24 wide × 12 high (`has-arrow` default on), filled with the popover colour, transparent 1px border (HC `$border_color`) | published `G/gtkpopover.c:162-163`; `A/w/_popovers.scss:37-46` | none |
| GN-1.181 | Popover distance from anchor | Gap between anchor and tail tip | measure (probe) — to verify | none |
| GN-1.182 | Popover shade colour | `--popover-shade-color` (separators / undershoot inside) | published `A/w/_popovers.scss:81`; `A/s/_colors.scss:72`, `:134` | `defaults.shadow_color` |
| GN-1.183 | Popover without compositing | Square, 1px `--border-color`, no shadow | published `A/w/_popovers.scss:48-60` | none |
| GN-1.184 | OSD / touch-selection / magnifier popovers | OSD fill, `0 0 1px` white 10 % edge | published `A/w/_popovers.scss:62-79` | none |
| GN-1.185 | Popover width | No model width (popover sizes to content; menu min 120) | published `A/w/_menus.scss:1-4` | none (seed: gpui `demo.rs:8012` "the model states no popover width") |
| GN-1.286† | Touch selection bubble (GtkText's edit menu on touch) | An OSD popover below the selection, pointing at its rectangle grown by 5 px on each side; content box margin 10, spacing 5; a `.linked` row of image buttons with the menu's touch icons (`edit-cut-symbolic`, `edit-copy-symbolic`, `edit-paste-symbolic`, …); fill and edge as GN-1.184 | published `G/gtktext.c:6384-6392`, `:6503-6507`, `:6540-6596`; `A/w/_popovers.scss:62-79` | `popover.*` (no button-row content) |

### Splitter, separator (`[splitter]`, `[separator]`)

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.186 | Wide splitter | Two 1px lines (one each side of the 5px handle), not one | published `A/w/_paned.scss:8-34` | `splitter.divider_color`, `divider_width` |
| GN-1.187 | Separator margins | In menus 6 0, in sidebars 6 | published `A/w/_menus.scss:68-70`; `A/w/_sidebars.scss:152-156` | none |
| GN-1.188 | Spacer separator | `.spacer`: no line, 12 min size | published `A/w/_misc.scss:26-31` | none |

### Text scale (`[text_scale]`) and fonts

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.189 | Type ramp roles | `.title-3` (136 %, 700), `.title-4` (118 %, 700), `.caption-heading` (82 %, 700), `.heading` vs `.body` | published `A/w/_labels.scss:35-73` | `text_scale.{caption, section_heading, dialog_title, display}` (four roles) |
| GN-1.190 | Line height per role | `.body` and `.caption` 140 %, `.document` 140 % | published `A/w/_labels.scss:59-79` | `text_scale_entry.line_height_*`, `defaults.line_height` (font metric ratio, no body role) |
| GN-1.191 | Document font | `.document` font family / size from `document-font-name` | read `S/org.gnome.desktop.interface:158-163`; published `A/w/_labels.scss:75-79` | none |
| GN-1.192 | Tabular figures | `.numeric` → `font-variant-numeric: tabular-nums` (used by spin buttons, scale values, progress text, calendar) | published `A/w/_labels.scss:86-88`; `A/w/_spin-button.scss:3`; `A/w/_calendar.scss:5` | none (no font features in `Font`) |
| GN-1.193 | Font features and variations from settings | Pango strings may carry `@axis=` variations and `#feature=` features; the model's `Font` has neither | read (the same `font-name` key; Pango format, platform-facts §1.4.1) | `Font` (family/size/weight/style/colour only) |
| GN-1.194 | Small label sizes | `font-size: smaller` (subtitles, progress text), `0.75rem` (narrow view switcher), `0.6rem` (badges), 10.5 / 8 px (tab button counter) | published `A/w/_header-bar.scss:222`; `A/w/_view-switcher.scss:16`, `:111`; `A/w/_buttons.scss:647-656` | none |

### Layout (`[layout]`)

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.195 | Content clamp (AdwClamp) | Maximum content width 600 and tightening threshold 400 (preferences pages, status pages, about dialog) | published `A/c/adw-clamp.c:199`, `:222`; `A/c/adw-clamp-layout.c:462-463` | none |
| GN-1.196 | Preferences page | Margins 24 12, group spacing 24 | published `A/w/_preferences.scss:1-4` | `layout.section_gap` (measured 18, different context) |
| GN-1.197 | Adaptive breakpoints | Header bar treats windows under 800 px high as mobile; preferences view switcher thresholds 110 / 400 | published `A/c/adw-header-bar.c:153-156`; `A/c/adw-preferences-dialog.c:23-24` | none |

### Switch (`[switch]`)

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.198 | Unchecked pressed | `$trough_active_color` 25 % | published `A/w/_switch.scss:9` | none |
| GN-1.199 | Checked pressed | Overlay `RGB(0 0 6 / 20%)` | published `A/w/_switch.scss:46` | none |
| GN-1.200 | Thumb hover / pressed colour | `$slider_hover_color` (white) on hover and pressed | published `A/w/_switch.scss:36-39`; `A/s/_colors.scss:289` | `switch.thumb_background` (checked thumb), `unchecked_thumb_background` (no hover thumb) |
| GN-1.201 | Thumb shadow | `0 2px 4px RGB(0 0 6 / 20%)`, transparent when disabled | published `A/w/_switch.scss:29-33` | none |
| GN-1.202 | Status shapes | On / off glyphs (`switch-on-symbolic`, `switch-off-symbolic`) shown when `show-status-shapes` is on; HC lets them inherit the colour | read `S/org.gnome.desktop.a11y.interface:11-17`; published `G/gtkswitch.c:720-743`, `G/gtksettings.c:586-594`; `A/w/_switch.scss:52-54` | none |
| GN-1.203 | Thumb slide motion | `ANIMATION_DURATION` 100 ms, ease-out cubic | published `G/gtkswitch.c:187`, `:203-210` | none |
| GN-1.204 | Track colour motion | 200 ms `ease-out-quad` (`$button_transition` via focus-ring) | published `A/w/_switch.scss:18`; `A/s/_common.scss:6-7` | none |
| GN-1.205 | OSD switch | Checked track white 60 % | published `A/w/_switch.scss:57-60` | none |
| GN-1.206 | Switch focus ring | Outer, offset 1 | published `A/w/_switch.scss:18` | `defaults.focus_ring_offset` (global, inset) |
| GN-1.207 | Switch HC | Track 30 / 40 / 50 % | published `A/w/_switch.scss:11-16`; `A/s/_colors.scss:284-286` | none |

### Dialog (`[dialog]`) — AdwAlertDialog and AdwDialog

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.208 | Dialog shadow | Three layers (`0 0 14px 2px` 3 %, `0 0 5px 2px` 10 %, `0 0 0 1px` 5 %) + HC | published `A/w/_dialogs.scss:76-91` | `dialog.border.shadow_enabled` |
| GN-1.209 | Dialog inner outline | 1px `$window_outline_color` inset (HC 30 %); alerts drop it under HC | published `A/w/_dialogs.scss:81-85`; `A/w/_message-dialog.scss:60-66` | `dialog.border.color` |
| GN-1.210 | Dimming (scrim) behind the dialog | `RGB(from --shade-color … alpha × 2)` | published `A/w/_dialogs.scss:71-74` | none |
| GN-1.211 | Non-alert AdwDialog radius | `$dialog_radius` 15 vs alert 18 — two dialog kinds | published `A/w/_dialogs.scss:77`; `A/w/_message-dialog.scss:1`, `:60-61` | `dialog.border.corner_radius` (one) |
| GN-1.212 | Response buttons | Min height 24, min width 60, padding 10 20, radius 12; `.short`: min width 0, padding 8 16 | published `A/w/_message-dialog.scss:119-149` | `button.*` (no dialog-button variant) |
| GN-1.213 | Response appearance | Suggested / destructive responses (button variants): the response button gets `.suggested-action` or `.destructive-action`, nothing for the default appearance | published `A/c/adw-alert-dialog.c:300-310`; button variants GN-1.30/31 | none (review 2026-10-28: the lines, "to verify", found) |
| GN-1.214 | Message area spacing | Area spacing 24; heading–body 10; child margin-top 8; bottom padding 9 | published `A/w/_message-dialog.scss:93-112` | `dialog.border.padding_*` (outer only; platform-facts §2.22 note says internal spacing is not exposed) |
| GN-1.215 | `.short` alert variant | Paddings 24→18, spacing 18 / 6, response padding 18 / 9 | published `A/w/_message-dialog.scss:127-150` | none |
| GN-1.216 | Response layout widths | `RESPONSE_HORZ_PADDING` 48 / 36 (short) used to decide stacked vs side-by-side | published `A/c/adw-alert-dialog.c:180-181` | none |
| GN-1.217 | Wide alert width | 600 sp | published `A/c/adw-alert-dialog.c:176` | `dialog.max_width` (one; platform-facts records both in prose) |
| GN-1.218 | Floating-sheet margins | Horizontal padding 30 → 120 and vertical 20 → 120 as the window grows from 720 to 1440 | published `A/c/adw-floating-sheet.c:24-32` | none |
| GN-1.219 | AdwDialog default size | `DEFAULT_NATURAL_SIZE` 200 | published `A/c/adw-dialog.c:22` | `dialog.min_width` |
| GN-1.220 | Entrance spring | Damping ratio 0.62, stiffness 500, min scale 0.8; reduced motion: fade only (damping 1) | published `A/c/adw-floating-sheet.c:22`, `:34-35`, `:85`, `:378-381`, `:482` | `dialog.entrance` (enum ScaleFade; no spring parameters) |
| GN-1.221 | Exit motion | The same spring (damping 0.62, mass 1, stiffness 500; damping 1 under reduced motion) run from the current progress back to 0, clamped so it does not overshoot | published `A/c/adw-floating-sheet.c:80-92`, `:482-491` | none (review 2026-10-28: "to verify by recording" settled from the source; source kind measure → published) |
| GN-1.222 | Bottom-sheet presentation | On narrow windows AdwDialog presents as a bottom sheet (`dialog.bottom-sheet`), side margins 30, flush outlines | published `A/w/_dialogs.scss:93-112`; see AdwBottomSheet GN-2.17 | none |
| GN-1.223 | Alert body scrolling | Scrolled body: scrollbar top padding 6, top radius 18 on the viewport | published `A/w/_message-dialog.scss:81-91` | none |
| GN-1.224 | Legacy GtkMessageDialog | Title bar min-height 32, response area padding 24/21, spacing 12, buttons 10 20 radius 12 | published `A/w/_message-dialog.scss:7-47` | none |
| GN-1.225 | Dialog title font colour vs window | Withdrawn (review 2026-10-28): false gap — the model has both, `dialog.background_color` and the dialog's own text colours `dialog.body_font.color` / `dialog.title_font.color` (`docs/property-registry.toml` `[dialog]`); a preset can set them apart from `defaults`. Text kept: Dialog bg/fg (`--dialog-bg-color`, `--dialog-fg-color`) | published `A/s/_colors.scss:66-67`, `:128-129` | `dialog.background_color`, `body_font.color` (fg not separate from defaults) |
| GN-1.284† | Alert response order and arrangement | Side by side: equal widths, spacing 12, in the order the app added the responses, mirrored in RTL; stacked (`.compact`): the first response at the bottom and each next one above it, spacing 12 | published `A/c/adw-alert-dialog.c:179`, `:617-682` | `dialog.button_order` (one enum; the stacked order is the reverse of the side-by-side one) |

### Spinner (`[spinner]`)

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.226 | AdwSpinner track | Full circle at opacity 0.15 under the arc | published `A/c/adw-spinner-paintable.c:21`, `:364-369` | none (`spinner.fill_color` is the arc) |
| GN-1.227 | Stroke proportional to size | Stroke = radius / 8 (2 px at 16 px), scaled by `-gtk-icon-weight` between 0.25× and 1.5×; round caps | published `A/c/adw-spinner-paintable.c:315-320`, `:353-354` | `spinner.stroke_width` (one fixed width) |
| GN-1.228 | Size range | AdwSpinner min 16, max 64 (radius 8–32) | published `A/c/adw-spinner.c:14-15`; `A/c/adw-spinner-paintable.c:17-18` | `spinner.diameter`, `min_diameter` (no max) |
| GN-1.229 | Arc motion | Cycle 1200 ms, arc extends / contracts with ease-in-out-sine, arc length 0.015π–0.9π, start angle 0.35π, 53-cycle loop, linear driver | published `A/c/adw-spinner-paintable.c:17-31`, `:121`, `:141`, `:532-540` | `TransformAnimation::Spin { duration_ms }` (rotation of an icon only; `model/animated.rs:121-126`) |
| GN-1.230 | GtkSpinner (GTK 4.24) | Plays the animated SVG `process-working.gpa`; under reduced motion `sand-watch.svg` | published `G/gtkspinner.c:158-161` | `spinner.*` (see Appendix: platform-facts describes a rotating icon) |
| GN-1.231 | Disabled spinner | Opacity `--disabled-opacity` | published `A/w/_spinner.scss:1-11` | `defaults.disabled_opacity` |
| GN-1.232 | Status-page spinner | Icon size 64, weight 267 | published `A/w/_misc.scss:176-186` | none |

### Combo box / drop-down trigger (`[combo_box]`)

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.233 | Label–arrow gap | `border-spacing: 6px` in the button box | published `A/w/_dropdowns.scss:2-3` | none (`arrow_area_width` is "none" for GNOME) |
| GN-1.234 | Drop-down list popup | Menu-style popover, padding-top 6, rows min-width 0; selected item marked with `object-select-symbolic` after the label | published `A/w/_dropdowns.scss:26-41`; `G/gtkdropdown.c:762-770` | `menu.*` (no selected-item mark) (review 2026-10-28: the icon name, "to verify", found in the C, not a `.ui` file) |
| GN-1.235 | Search in the popup | Search entry padding 6 and an undershoot under it | published `A/w/_dropdowns.scss:34-40` | none |
| GN-1.236 | `.flat` drop-down | Flat button look | published `A/w/_dropdowns.scss:10-12`; `A/w/_toolbars.scss:68-70` | none |
| GN-1.237 | Combo pressed / open / checked | The trigger is a toggle button: checked while open (button checked fills) | published `A/w/_buttons.scss:44-55` (trigger is a GtkToggleButton, platform-facts §2.24) | none (`combo_box.hover_background` only) |
| GN-1.238 | Legacy GtkComboBox | Button padding 2 vertical, min-height 30 | published `A/w/_deprecated.scss:100-110` | none |

### Segmented control (`[segmented_control]`) — AdwToggleGroup

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.239 | Group padding and inner radius | Outer padding `---group-padding` 3; toggle radius = 9 − 3; the group's radius 9 | published `A/w/_toggle-group.scss:9-29` | `segmented_control.border` (one radius, one padding) |
| GN-1.240 | Checked toggle shadow | Two layers `0 1px 3px 1px` 7 %, `0 2px 6px 2px` 3 % | published `A/w/_toggle-group.scss:79-100` | none |
| GN-1.241 | Toggle pressed | `$active_color` 16 % | published `A/w/_toggle-group.scss:69-71` | none |
| GN-1.242 | Disabled group / disabled checked | Group fill and checked fill mixed at disabled opacity; image/label at 30 % | published `A/w/_toggle-group.scss:16-18`, `:34-60`, `:85-88` | `segmented_control.disabled_opacity` ("(none)" for GNOME) |
| GN-1.243 | Separators between toggles | Margins (6 − 3) × 1, hidden next to the checked toggle with a 200 ms fade, colour | published `A/w/_toggle-group.scss:141-159` | `segmented_control.separator_width` (no colour, no margins) |
| GN-1.244 | Min widths | Icon toggle 34, text toggle 18 + 2 × 3, image-text 10 + 2 × 3 | published `A/w/_toggle-group.scss:31`, `:39`, `:49` | none |
| GN-1.245 | `.round` variant | Radius 17, padding 15 − 3, separator margins 9 − 3 | published `A/w/_toggle-group.scss:161-183` | none |
| GN-1.246 | `.flat` variant | No group fill; checked 10 %, checked+hover 13 %, checked+pressed 19 % | published `A/w/_toggle-group.scss:185-221` | none |
| GN-1.247 | OSD variant | Checked toggle white / dark-on-white, focus ring colour override | published `A/w/_toggle-group.scss:102-135` | none |
| GN-1.248 | Vertical orientation | Vertical toggles min width 34 − 2 × 3 | published `A/w/_toggle-group.scss:137-139` | none |
| GN-1.249 | Motion | `color`, `background`, `box-shadow` 200 ms ease-out-quad | published `A/w/_toggle-group.scss:5-7`, `:63` | none |
| GN-1.250 | Focus ring | Outer, offset 0 (OSD 1) | published `A/w/_toggle-group.scss:63`, `:118` | `defaults.focus_ring_*` |
| GN-1.251 | Segmented HC | Inset 1px border on group, hover/pressed toggles and checked toggle | published `A/w/_toggle-group.scss:20-26`, `:73-77`, `:90-99` | none |

### Card (`[card]`)

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.252 | Card shadow | Three layers: `0 0 0 1px` 3 % (outline), `0 1px 3px 1px` 7 %, `0 2px 6px 2px` 3 %; HC outline `$border_color` | published `A/w/_misc.scss:201-214` | `card.border.shadow_enabled` |
| GN-1.253 | Activatable card | Hover 3 % / pressed 8 % overlays, focus transition | published `A/w/_misc.scss:227-239` | none |
| GN-1.254 | Card focus ring | Offset −1 | published `A/w/_misc.scss:227` | `defaults.focus_ring_offset` |
| GN-1.255 | Card fg and shade | `--card-fg-color`, `--card-shade-color` (row separators, keycaps) | published `A/s/_colors.scss:61-63`, `:123-125` | `card.background_color` only |
| GN-1.256 | OSD card | 10 % currentColor fill, OSD fg, no shadow | published `A/w/_misc.scss:216-224` | none |

### Expander (`[expander]`)

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.257 | Expanded arrow colour | AdwExpanderRow arrow turns `--accent-color` and opacity 1 when expanded | published `A/w/_lists.scss:408-415` | `expander.arrow_color` (one state) |
| GN-1.258 | Arrow rotation motion | `-gtk-icon-transform` 200 ms ease-out-quad (rotate 0.5 turn → 0) | published `A/w/_lists.scss:389-401` | none |
| GN-1.259 | Reveal motion | Revealer `slide-up`, GtkRevealer default 250 ms: the row sets no duration (no `transition-duration` in `A/c/adw-expander-row.ui` or `.c`, grep) | published `A/c/adw-expander-row.ui:74-77`; `G/gtkrevealer.c:717-719`, `:770` | none (review 2026-10-28: "to verify" settled) |
| GN-1.260 | Nested content fill | `list.nested` in card-shade at 50 % | published `A/w/_lists.scss:378-385` | none (seed: gpui `contract.rs:551` "the model has no accordion or expander fill") |
| GN-1.261 | GtkExpander (plain) | Title radius 6, arrow `pan-end` / `pan-down` 16px at opacity 0.7 → 1 on hover / pressed, focus ring on the title | published `A/w/_expanders.scss:1-27` | `expander.*` (no hover arrow opacity, no title radius) |
| GN-1.262 | GtkExpander drag-hover expand | `TIMEOUT_EXPAND` 500 ms | published `G/gtkexpander.c:136` | none |
| GN-1.263 | Disabled arrow | Opacity filter | published `A/w/_lists.scss:403-405`; `A/w/_expanders.scss:7` | `defaults.disabled_opacity` |

### Link (`[link]`)

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.264 | Link focus ring | Default inset ring | published `A/w/_links.scss:34-36` | `defaults.focus_ring_*` |
| GN-1.265 | Links on OSD | `$osd_link_color` (accent 50 % + white), visited 75 % | published `A/w/_links.scss:21-31`; `A/s/_colors.scss:293-294` | none |
| GN-1.266 | `button.link` | Button-shaped link: transparent in every state, underlined label | published `A/w/_links.scss:38-49` | none |
| GN-1.267 | Underline geometry | Thickness and offset of `text-decoration: underline` | measure (probe; Pango underline metrics) | `link.underline_enabled` (on/off) |

### Global defaults (`[defaults]`)

| ID | Element | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-1.268 | Standalone accent | `--accent-color` (Oklab L ≤ 0.5 light, ≥ 0.85 dark) for accent-coloured text and icons, distinct from `--accent-bg-color` | published `A/s/_colors.scss:145-170`; `A/c/adw-accent-color.c:130-157` (reader already ports it for the focus ring only, `native-theme/src/gnome/accent.rs`) | `defaults.accent_color` (= bg), `link_color` (partial) |
| GN-1.269 | Status colour triples | For destructive, success, warning, error: bg, fg **and** standalone colour; destructive and error are separate tokens | published `A/s/_colors.scss:11-29`, `:83-92`, `:172-194` | `defaults.danger_color`, `danger_text_color`, … (two of three; no separate destructive vs error) |
| GN-1.270 | Shade colours | `--shade-color`, card / popover / header-bar / sidebar shades | published `A/s/_colors.scss:44-45`, `:51`, `:63`, `:72`, `:79`, `:106-113`, `:125`, `:134`, `:141` | `defaults.shadow_color` (one) |
| GN-1.271 | Dim opacity | `--dim-opacity` 55 % (HC 90 %) applied as opacity to labels and icons (`.dimmed`) | published `A/s/_colors.scss:319`, `:326`; `A/w/_labels.scss:13-15` | `defaults.muted_color` (a colour, not an opacity on icons) |
| GN-1.272 | Second disabled opacity | `$strong_disabled_opacity` 30 % (flat buttons, toggle images, edit icons) | published `A/s/_colors.scss:311`; `A/w/_buttons.scss:272-274` | `defaults.disabled_opacity` (one) |
| GN-1.273 | Dimmer opacity | `$dimmer_opacity` 30 % (HC 80 %) for secondary glyphs (menu arrows, split separators, caps-lock) | published `A/s/_colors.scss:309-310` | none |
| GN-1.274 | View colours vs window colours | `--view-bg-color` / `--view-fg-color` (lists, text views) | published `A/s/_colors.scss:36-37`, `:99-100` | `defaults.surface_color` (bg only, no view fg) |
| GN-1.275 | Window fg vs view fg | Withdrawn (review 2026-10-28): duplicate, see GN-1.274 (the view fg the model lacks; the window fg is `defaults.text_color`). Text kept: Separate tokens, same default values | published `A/s/_colors.scss:32-37` | `defaults.text_color` |
| GN-1.276 | Thumbnail / overview colours | `--thumbnail-*`, `--overview-*` | published `A/s/_colors.scss:75-76`, `:137-138`, `:241-251` | none |
| GN-1.277 | Active-toggle colours | Withdrawn (review 2026-10-28): false gap, as the row itself says — `segmented_control.active_background` and `active_text_color` carry both. Text kept: `--active-toggle-bg-color`, `--active-toggle-fg-color` | published `A/s/_colors.scss:238-248` | `segmented_control.active_*` (exists) — listed for completeness, no gap |

---

## 4.2 Standard widgets the model lacks entirely

Each row names the widget and every visible property it has; "nearest" is the model section an implementation would borrow from today.

| ID | Widget | What it draws (all missing) | Source | Nearest model field |
|---|---|---|---|---|
| GN-2.01 | AdwActionRow | Header min-height 50, margins 12, spacing 6, title-box margins 6, title/subtitle spacing 3, subtitle `smaller` + dimmed, prefix/suffix spacing 6, icon margin 6, disabled icon | published `A/w/_lists.scss:123-169` | `list.*`, `expander.header_height` |
| GN-2.02 | Property row (`row.property`) | Title smaller + dimmed, subtitle full size; monospace subtitle variant | published `A/w/_lists.scss:426-452` | none |
| GN-2.03 | AdwEntryRow | Header margins 6, editable padding 0 6, edit icon / indicator 24 (padding 5), indicator 30 % (HC 80 %), disabled text opacity, focus ring `.focused` offset −1 with row transition, error/warning/success colours, monospace variant | published `A/w/_lists.scss:171-293` | `input.*` |
| GN-2.04 | AdwEntryRow title motion | Title shrinks to a caption above the text: 150 ms `ADW_EASE` | published `A/c/adw-entry-row.c:18`, `:781-784` | none |
| GN-2.05 | AdwSpinRow | Circular 30×30 up/down buttons, margins 10 2, no entry frame | published `A/w/_lists.scss:240-273` | none |
| GN-2.06 | AdwComboRow | Drop-down arrow, inline list, popover min-width 120, search bar margin 6 + undershoot | published `A/w/_lists.scss:299-327` | `combo_box.*` |
| GN-2.07 | AdwSwitchRow | A row with a GtkSwitch suffix (switch rules above) | published `A/w/_switch.scss`; `A/w/_lists.scss:123-169` | `switch.*` |
| GN-2.08 | AdwButtonRow | Min height 40, margins 12, spacing 6, heading title; suggested (accent fill, overlays, outer focus offset 1) and destructive variants | published `A/w/_lists.scss:454-514` | `button.*` |
| GN-2.09 | AdwExpanderRow (as a list row) | Row structure in boxed lists (the expander section covers its arrow and header) | published `A/w/_lists.scss:367-420`, `:529-553` | `expander.*` (partial) |
| GN-2.10 | AdwPreferencesGroup / Page / Dialog | Group spacing 6, header bottom margin 6, single-line header min-height 34; page margins 24 12, spacing 24; dialog view-switcher thresholds | published `A/w/_preferences.scss:1-18`; `A/c/adw-preferences-dialog.c:23-24` | `layout.*` |
| GN-2.11 | AdwBanner | Fill `mix(--banner-color 30 %, window-bg)`, padding 6, backdrop 15 % + content 50 %, sidebar variants, spacing 6 / 36 centred / 9 vertical, button min width 84 / 160, slide-down reveal (GtkRevealer default 250 ms) | published `A/w/_toolbars.scss:251-270`; `A/w/_sidebars.scss:12-19`; `A/c/adw-banner.c:15-21`; `A/c/adw-banner.ui:10`; `G/gtkrevealer.c:717-719` | none |
| GN-2.12 | AdwToast / AdwToastOverlay | Fill `$toast_bg_color`, white fg, two-layer shadow, pill radius 150, margin 12 / bottom 24, spacing 6, padding 6 / start 12, HC outline, dark-standalone accents; natural width 450; show / hide 300 ms `ADW_EASE` with scale 0.05, replace 500 ms; default timeout 5 s | published `A/w/_misc.scss:38-69`; `A/s/_colors.scss:296-297`; `A/c/adw-toast-overlay.c:20-24`, `:144`, `:196-198`, `:271-275`; `A/c/adw-toast.c:475` | none |
| GN-2.13 | AdwStatusPage | Margins 36 12, spacing 36 / 12, icon 128 at dim opacity with weight 134, `.title-1`; `.compact` 24 / 96 / `.title-2`; spinner variant 64 / 267 | published `A/w/_misc.scss:126-197` | `text_scale.display` (title only) |
| GN-2.14 | AdwAvatar | Circle, bold initials, 14 gradient colour pairs + initials colour, `.contrasted` white text, image mode; initials size relative to avatar size (to verify `A/c/adw-avatar.c`) | published `A/w/_avatar.scss:1-38`; `A/c/adw-avatar.c:20` | none |
| GN-2.15 | AdwViewSwitcher / ViewSwitcherTitle / ViewSwitcherBar / ViewSwitcherSidebar | Spacing 3, min-height 34; narrow (0.75rem labels, min 18, padding 3 / 2, top 4) and wide (2 12, spacing 6) buttons; min natural widths 100 / 120; bar padding 7; title margins; sidebar counter pill 1.6em / dot 8; drag-hover switch 500 ms; bar reveal 250 ms | published `A/w/_view-switcher.scss:1-77`; `A/w/_sidebars.scss:257-283`; `A/c/adw-view-switcher-button.c:14`, `:29-30`; `A/c/adw-view-switcher-bar.c:166` | `segmented_control.*`, `tab.*` |
| GN-2.16 | AdwInlineViewSwitcher | Toggle-group based switcher with badge offsets, label padding 6, image-text spacing 6; drag-hover expand 500 ms | published `A/w/_toggle-group.scss:223-258`; `A/c/adw-inline-view-switcher.c:80` | `segmented_control.*` |
| GN-2.17 | AdwBottomSheet | Dimming (shade × 2), three-layer shadow, drag handle 54×6 (25 %, margin 15, radius 99), top radius 15, inner outline, flush variants, bottom bar with four-layer shadow; transitions `border-radius` 100 ms ease-out, `box-shadow` 250 ms ease; open spring damping 0.8 stiffness 400; bottom-bar reveal 250 ms `ADW_EASE`; top padding 30 → 120 | published `A/w/_bottom-sheet.scss:1-162`; `A/c/adw-bottom-sheet.c:27-34`, `:216`, `:1225-1243` | `dialog.*` (seed: gpui `info/overlays.rs:221` "the model states no sheet") |
| GN-2.18 | AdwNavigationView (push / pop) | Spring damping 1, mass 1, stiffness 1000; transition shadow (56 px, two gradients of the shade colour) and dimming; reduced motion crossfades; swipe-back gesture (animation 100–400 ms) | published `A/c/adw-navigation-view.c:22`, `:899`, `:2151-2152`; `A/w/_misc.scss:244-247`; `A/s/_drawing.scss:113-147`; `A/c/adw-swipe-tracker.c:18-20` | none |
| GN-2.19 | AdwNavigationSplitView / AdwOverlaySplitView | Sidebar width (GN-1.147), overlay pane in window colours with transition shadow + dimming, overlay spring (1, 0.5, 500), `.sidebar-pane` / `.content-pane` colours | published `A/w/_sidebars.scss:1-93`; `A/c/adw-overlay-split-view.c:1164-1165`; `A/w/_misc.scss:244-247` | `sidebar.*` |
| GN-2.20 | GtkActionBar (bottom bar) | Header-bar fill and fg, top shade line (HC border), padding 7 6 6 6, spacing 6, backdrop fill + 50 % content, inline / shrunk variants | published `A/w/_toolbars.scss:199-246` | `status_bar.*`, `toolbar.*` |
| GN-2.21 | GtkSearchBar | Header-bar fill, bottom shade line, padding 6 6 7 6, close button (18×18, padding 4, round, flat, margins 10 / 4), backdrop, inline / flat / shrunk variants | published `A/w/_toolbars.scss:121-193` | `toolbar.*` |
| GN-2.22 | GtkSearchEntry | Search / clear icons (entry icon opacity rules), search-specific margins — to verify any `entry.search` rule (none found in libadwaita 1.10.0) | published `A/w/_entries.scss:42-61` | `input.*` |
| GN-2.23 | GtkPasswordEntry | Peek icon, caps-lock indicator at 30 % (HC 80 %); password hint timeout (GtkSettings default 0) | published `A/w/_entries.scss:63-69`; `G/gtksettings.c:782` | `input.*` |
| GN-2.24 | GtkSpinButton | Entry frame; up/down flat buttons with 1px separators (10 %, HC 50 %), text min width 28 padding 6, buttons min 16 × 22, end radii; vertical variant (text 30 × 30, buttons top/bottom); tabular figures | published `A/w/_spin-button.scss:1-76` | `input.*`, `button.*` |
| GN-2.25 | GtkCalendar | 1px border, header with flat square buttons and bottom line, grid padding 3, day labels margin 3 padding 3, today underline (`inset 0 -2px $border_color`), selected day accent bg/fg radius 9, checked (marked) day accent 30 %, other-month days 30 %, tabular figures, focus ring | published `A/w/_calendar.scss:1-58` | none |
| GN-2.26 | GtkLevelBar | Block height 8, pill radius 99; continuous and discrete modes (block min 26, gap 2, end radii); colours: low → warning bg, high / filled → accent bg, full → success bg, empty → trough (HC trough + inset border); vertical variant; disabled | published `A/w/_level-bar.scss:1-95` | `progress_bar.*` |
| GN-2.27 | GtkColorDialogButton / colour swatches | `button.color` padding 5, swatch radius 9 − 4.5, overlay radius 9 − 5, light-swatch ring 10 % view fg; colour chooser swatches with per-edge radii, 4px focus ring, drop target ring, HC ring; editor sample | published `A/w/_buttons.scss:368-386`; `A/w/_color-chooser.scss:1-109` | none |
| GN-2.28 | GtkFontDialogButton | `button.font`: hidden separator, spacing 6, bold inner label | published `A/w/_buttons.scss:306-310` | `button.*` |
| GN-2.29 | AdwSplitButton (GtkMenuButton split) | Radius 9, separator margins 6 (flat: 30 % / HC 80 %, hidden on hover and focus), arrow button padding 4, image / text paddings, joined inner corners with −1 overlap, suggested / destructive / opaque variants with 1px inset separator, disabled | published `A/w/_buttons.scss:472-624` | `button.*` |
| GN-2.30 | GtkPopoverMenuBar | Bottom 1px `$border_color` line, padding-bottom 1, items padding 4 8, min-height 16, radius 9, selected 10 %, HC ring; submenu popovers padding 0 0 4, radius 15 | published `A/w/_menus.scss:248-282` | `menu.*` (seed: gpui `info/chrome.rs:250`, `:325` "the model states no menu bar") |
| GN-2.31 | GtkInfoBar (deprecated, still drawn) | Padding 6 / 7, spacing 6, bottom shade line, info/question/warning/error fills = type colour 30 % over window bg, action hover 5 % / pressed 10 %, round flat close button 18 | published `A/w/_deprecated.scss:150-191` | `defaults.info_color` etc. |
| GN-2.32 | GtkFrame | 1px `$border_color`, radius 12, label margin 4; `.frame` class | published `A/w/_misc.scss:4-15` | `card.*` |
| GN-2.33 | GtkStackSwitcher | Linked buttons; label / image padding; text buttons min-width 100; needs-attention dot | published `A/w/_buttons.scss:282-303` | `segmented_control.*` |
| GN-2.34 | GtkStackSidebar | Row padding 10 4, label padding 6, needs-attention dot | published `A/w/_sidebars.scss:95-107` | `sidebar.*` |
| GN-2.35 | AdwTabOverview / AdwTabGrid / AdwTabThumbnail | Overview bg/fg, thumbnail radius 16 (card + 4), margin 6, card picture outline, pinned thumbnail colours, attention dot 12px accent with shadow, close / unpin buttons (24, circle, thumbnail 75 % fill, hover / pressed mixes), new-tab button margin 18, drop target fill/ring; grid spacing 5, columns 2–8, thumbnail width 100–500; open 400 ms `ADW_EASE`, crossfade on reduced motion; grid open/close/reorder 200 / 200 / 250 ms; thumbnail fade 250 ms | published `A/w/_tab-view.scss:183-304`; `A/c/adw-tab-overview.c:29-30`, `:1936-1941`, `:2217`; `A/c/adw-tab-grid.c:23-52`; `A/c/adw-tab-thumbnail.c:18-19` | none |
| GN-2.36 | AdwIndicatorBin badges | Dot 6 (margin 2, 40 % currentColor; attention: accent), badge 13 high with 0.6rem bold white label, padding 4; mask | published `A/w/_view-switcher.scss:83-124` | none |
| GN-2.37 | Tab button (AdwTabButton) | Counter label 800 weight, 10.5px / 8px small, indicator offset | published `A/w/_buttons.scss:647-663` | none |
| GN-2.38 | AdwCarousel + indicator dots / lines | Page spring (1, 0.5, 500), scroll timeout 150, indicator animation (linear) | published `A/c/adw-carousel.c:22`, `:1223-1224`; `A/c/adw-carousel-indicator-dots.c:361-363`; `A/c/adw-carousel-indicator-lines.c:346-348`; indicator geometry: GN-2.55† | none (review 2026-10-28: the indicator geometry, "to verify", is published, GN-2.55†) |
| GN-2.39 | AdwAboutDialog | Main page margins 12, spacing 6 / 18, app icon 128 px with `.icon-dropshadow` (legacy GtkAboutDialog: `.large-icons` at 128), version pill (accent text, padding 3 18, radius 999), subpage margins 18 12 | published `A/w/_misc.scss:87-120`; `A/c/adw-about-dialog.ui:50-56`; `A/w/_dialogs.scss:48-56` | none (review 2026-10-28: the icon size, "to verify", is the image's `pixel-size` 128 in the `.ui`) |
| GN-2.40 | Shortcut label / keycap (GtkShortcutLabel, AdwShortcutLabel) | Keycap fill 10 %, inset bottom 2px card shade (HC + 1px border), radius 6, padding 6 (2 6 legacy), min width 20, `smaller` font; label spacing 6; focus ring offset 1 / 4 | published `A/w/_shortcuts-dialog.scss:34-57`; `A/w/_deprecated.scss:344-364` | none |
| GN-2.41 | AdwShortcutsDialog | Page spacing 18, navigation pill buttons (radius 99, padding 3 12), shortcut row padding 9 12, min-height 36, caption dimmed subtitles | published `A/w/_shortcuts-dialog.scss:1-32` | none |
| GN-2.42 | Text selection handles and magnifier (touch) | Handles 20×20 accent circles with one square corner, padding 24 20, insertion handle rotated 45°; magnifier view bg | published `A/w/_text-selection.scss:1-30` | none |
| GN-2.43 | GtkEmojiChooser | Picker popover, section buttons, emoji cells, completion rows | published `A/w/_emoji-chooser.scss:1-96` (not read line by line — to verify each rule) | none |
| GN-2.44 | GtkVideo / media controls | OSD play image 64×64 radius 32 | published `A/w/_misc.scss:75-81` | none |
| GN-2.45 | GtkEditableLabel | Editing state in view colours | published `A/w/_labels.scss:94-97` | `input.*` |
| GN-2.46 | AdwWrapBox / AdwWrapLayout | Child spacing and line spacing (properties; both default to 0 px, so the gaps are the app's: GN-P3-24†) | published `A/c/adw-wrap-layout.c:1003-1004`, `:1111-1112` (GN-P3-24†) | `layout.widget_gap` (review 2026-10-28: the defaults, "to verify", settled by GN-P3-24†) |
| GN-2.47 | AdwSpinner as a widget | Withdrawn (review 2026-10-28): duplicate, see GN-1.226–GN-1.232 (AdwSpinner) and GN-1.230 (GtkSpinner). Text kept: (Covered as `[spinner]` GN-1.226-1.232; listed so GtkSpinner vs AdwSpinner are both accounted for) | — | `spinner.*` |
| GN-2.48 | GtkFileChooser / places sidebar | GTK's own chooser: places sidebar rows (GN-P3-22†), path bar (GN-2.52†), file list and grid (GN-P3-23†). A GNOME app rarely shows it: GtkFileDialog goes through the file-chooser portal whenever one is available, and on GNOME since 47 the portal's chooser is Files (GN-P3-21†); GTK draws its own only without the portal (no portal service or interface, portals disabled) or when an app embeds GtkFileChooserWidget / GtkFileChooserDialog | published `A/w/_file-chooser.scss:1-130` (`:131-266` not read line by line — to verify); `G/gtkfiledialog.c:917`; `G/gtkfilechoosernativeportal.c:487`; `GDK/gdk.c:617-650` | `sidebar.*`, `list.*` (review 2026-10-28: "drawn by GTK when the portal is not used" missed that on GNOME the portal's chooser is Files; the row now says when GTK's chooser shows) |
| GN-2.49 | Drag icon (`dnd`) | Window fg colour; dragged tab look (GN-1.133) | published `A/s/_common.scss:30-32` | none |
| GN-2.50 | `.app-notification` (legacy in-app notification) | OSD, padding 10, spacing 10, bottom radii 15, top gradient | published `A/w/_deprecated.scss:466-476` | none |
| GN-2.51 | GtkAssistant sidebar (deprecated) | Sidebar colours, step labels padding 6 12 radius 9, highlighted step 10 % | published `A/w/_deprecated.scss:50-94` | `sidebar.*` |
| GN-2.52† | GtkPathBar (the breadcrumb of GTK's own file chooser) | A horizontally scrolling `.linked` row of toggle buttons, one per folder, the current folder checked; root, home, desktop and recent buttons carry symbolic icons (`.image-button`, or icon + label); the path bar's own buttons keep the raised button look (the chooser's flat rule reaches only the view-toggle and new-folder buttons beside the path bar, `#pathbarbox > stack > box > button` and `menubutton > button`); non-image buttons 9 px side padding, icon–label spacing 4, label padding 2 | published `G/gtkpathbar.c:195-204`, `:420-506`, `:536-562`, `:617-650`; `G/ui/gtkfilechooserwidget.ui:30-80`; `A/w/_file-chooser.scss:81-108` | `button.*`, GN-1.42 (linked) (review 2026-10-28: "flat inside the file chooser" was wrong: GtkPathBar is not a child the flat selector matches) |
| GN-2.53† | GtkColorChooserWidget editor (the custom-colour page of the colour chooser) | Grid spacing 12; circular eyedropper button (`color-select-symbolic`); 300 × 300 saturation / value plane with a crosshair (focused: white 60 % 3-px bands under black 80 % 1-px lines; unfocused: RGB 0.8 at 80 % 1-px lines), plane focus ring outer offset 2, `crosshair` pointer; vertical hue scale (rainbow texture trough) and horizontal alpha scale (checkerboard under a linear gradient to the colour), both GtkScales with the slider rules of GN-1.94–1.108; sample swatch radius 9; hex entry; OSD popups for the numeric entries, radius `$popover_radius` 15 | published `G/ui/gtkcoloreditor.ui:36-146`; `G/gtkcolorplane.c:95-122`, `:193`; `G/gtkcolorscale.c:74-156`; `A/w/_color-chooser.scss:97-109`; `A/s/_common.scss:13`; checkerboard: 10 × 10 px cells in `#A8A8A8` and `#545454` (`G/deprecated/gtkcolorchooser.c:247-260`, also under translucent swatches, `G/gtkcolorswatch.c:117`) | none (review 2026-10-28: the checkerboard cell, "to verify", settled) |
| GN-2.54† | GtkMediaControls in GtkVideo; GtkScaleButton / GtkVolumeButton | Controls bar `.osd.bottom` at the bottom of the video, revealed by a crossfade and hidden with the pointer after 3 s without motion; box spacing 6: flat play button (`media-playback-start-symbolic`), time label, seek GtkScale, duration label (tabular figures, `tnum=1`), volume button; the overlay play image is GN-2.44; the scale button is a flat button opening a `.scale-popup` popover: flat `list-add-symbolic` button, vertical scale 100 high, flat `list-remove-symbolic` button | published `G/ui/gtkmediacontrols.ui:18-64`; `G/ui/gtkvideo.ui:14-46`; `G/gtkvideo.c:117-153`; `G/ui/gtkscalebutton.ui:8-51` | none |
| GN-2.55† | AdwCarousel indicators (the geometry GN-2.38 left to verify) | Dots: radius 3, selected 4, opacity 30 % → 90 %, both interpolated with the scroll position, spacing 7, margin 6, in the widget's `color`; lines: 3 thick, 35 long, spacing 5, opacity 30 %, the active line 90 %, margin 2 | published `A/c/adw-carousel-indicator-dots.c:17-22`, `:86-145`; `A/c/adw-carousel-indicator-lines.c:16-21`, `:83-141` | none |
| GN-2.56† | GtkEntryCompletion popup (deprecated since 4.10, still drawn) | Popover `.entry-completion` below the entry, no arrow, no autohide, as wide as the entry, holding a header-less GtkTreeView (GN-1.279†) of at most 10 rows with hover selection; libadwaita has no rule for `.entry-completion` (grep); refilter delay 100 ms | published `G/deprecated/gtkentrycompletion.c:94`, `:480-515`, `:1164-1180`, `:2023` | `popover.*`, `list.*` |
| GN-2.57† | GTK's own composite dialogs: print, page setup, app chooser, legacy GtkDialog | Print dialog: page preview `paper` white with text `RGB(0 0 6 / 80%)` and a 1 px `$border_color` border, flat title bar, action box in view colours; page setup: flat title bar; app chooser: flat shrunk title bar (top padding 3), flat search bar, content margins 6; GtkDialog action area: margin 6, spacing 6 | published `A/w/_dialogs.scss:1-42`; `A/w/_deprecated.scss:11-44` | `dialog.*` |
| GN-2.58† | Password mask glyph (GtkPasswordEntry, any GtkText with `visibility` off) | The invisible character is the first of U+25CF ●, U+2022 •, U+2731 ✱, U+273A ✺ that the font covers, else `*` | published `G/gtktext.c:3688-3729` | none |

---

## 4.3 Properties that cross widgets

### Elevation and shadows

| ID | Property | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-3.01 | Shadow layers | A list of layers per element, each with x/y offset, blur, spread, colour, inset flag. GNOME uses 1-, 2-, 3- and 4-layer stacks: window (active, backdrop, HC), message window, dialog/floating sheet, bottom sheet and its bottom bar, popover, card / boxed list, toast, checked toggle, switch thumb, slider thumb, title bar, toolbar-view bars, dragged tab, tab-overview attention dot | published: the per-element rows above (GN-1.01, 1.02, 1.09, 1.14, 1.97, 1.153, 1.179, 1.201, 1.208, 1.240, 1.252, 2.12, 2.17, 2.35) | `border.shadow_enabled`, `defaults.shadow_color` |
| GN-3.02 | Outline vs border | libadwaita draws many outlines as a `0 0 0 1px` spread shadow outside the box (no layout space), and window / dialog inner outlines as `outline` with −1px offset | published `A/w/_window.scss:3-10`; `A/w/_popovers.scss:19-21`; `A/w/_misc.scss:206` | `border.line_width` (inside border only) |
| GN-3.03 | Inset shadows as lines | Header bars, search bars, action bars, sidebars, paned separators, notebook indicators draw their lines as `inset` box-shadows | published `A/w/_header-bar.scss:5-7`; `A/w/_paned.scss:14-34`; `A/w/_sidebars.scss:21-40` | `border.color` (no side, no inset) |
| GN-3.04 | Dimming / scrim | Dialog and bottom-sheet scrim (shade alpha × 2); navigation and overlay split-view dimming | published `A/w/_dialogs.scss:71-74`; `A/w/_bottom-sheet.scss:2-4`; `A/s/_drawing.scss:125-147` | none |
| GN-3.05 | Transition (edge) shadow | 56 px two-gradient shade cast by a page sliding over another | published `A/s/_drawing.scss:113-147` | none |
| GN-3.06 | Scroll edge indication | Undershoot (1px shade line + 4px gradient, HC border) and overshoot (two radial gradients 12 % / 5 % currentColor) | published `A/s/_drawing.scss:30-111`; `A/w/_scrolling.scss:129-167` | none |
| GN-3.07 | Icon shadows | `.lowres-icon` (4 × 1px) and `.icon-dropshadow` (5 layers incl. `0 1px 12px`) | published `A/s/_common.scss:89-104` | none |

### Translucent materials

| ID | Property | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-3.08 | Translucent fills over the parent | Nearly every libadwaita fill is `currentColor` at a percentage over whatever is behind (buttons 10 %, hover 7 %, troughs 15 %, view hover 4 %, selection accent 25 %); the model stores final opaque colours resolved over one background, which is wrong over a different parent (card, sidebar, header bar, OSD) | published `A/s/_colors.scss:263-289`; `A/w/_buttons.scss:1-15` | every `*_background` (opaque or fixed-alpha colour) |
| GN-3.09 | OSD material | `.osd`: black 70 % fill, white 90 % fg, accent overrides (white-ish accent, dark standalone), scrollbar outline; applies to toolbars, cards, popovers, buttons, links, switches, toggle groups, progress bars, dialogs | published `A/s/_common.scss:43-60`; `A/s/_colors.scss:291-294` | none |
| GN-3.10 | Reduced transparency | Withdrawn (review 2026-10-28): not a gap, as the row itself says (GNOME has no such setting; the model's flag stays unset). Text kept: GNOME has no such setting (platform-facts §2.1.7 "(none)") — no gap | — | `reduce_transparency` |

### Style classes as variants

| ID | Property | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-3.11 | Variant dimension | libadwaita's look is per style class: `.flat`, `.raised`, `.suggested-action`, `.destructive-action`, `.opaque`, `.pill`, `.circular`, `.osd`, `.card`, `.boxed-list`, `.boxed-list-separate`, `.navigation-sidebar`, `.rich-list`, `.data-table`, `.column-separators`, `.separators`, `.linked`, `.toolbar`, `.inline`, `.compact`, `.round`, `.short`, `.property`, `.monospace`, `.numeric`, `.document`, `.dimmed`, `.accent`, `.error` / `.warning` / `.success`, `.title-1…4`, `.heading`, `.caption`, `.caption-heading`, `.body`, `.background`, `.view`, `.frame`. The model has one look per widget (plus `primary_*` on button) | published: the files cited in §4.1/§4.2 | `button.primary_*` only |
| GN-3.12 | Status-coloured widgets | `.error`, `.warning`, `.success` recolour any label / entry / row / icon through `--accent-*` | published `A/w/_labels.scss:21-29`; `A/w/_lists.scss:284-292` | `defaults.danger_*`, `warning_*`, `success_*` (colours, no mechanism) |
| GN-3.13 | `.accent` text | Label in the standalone accent | published `A/w/_labels.scss:17-19` | `defaults.accent_color` (bg tone) |

### Accent and system colour derivation

| ID | Property | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-3.14 | Accent roles | bg (`--accent-bg-color`), fg (`--accent-fg-color`, always white from the style manager), standalone (`--accent-color`) | published `A/s/_colors.scss:166-170`; `A/c/adw-style-manager.c:149-166` | `defaults.accent_color`, `accent_text_color` (no standalone) |
| GN-3.15 | Accent snapping | libadwaita snaps the portal RGB to the nearest of its 9 accents before use | published `A/c/adw-settings-impl-portal.c:140`; `A/c/adw-accent-color.c:160` | reader behaviour (`gnome/mod.rs:503-519` uses the raw RGB) — not a model field, see Appendix |
| GN-3.16 | Accent fallback key | `org.gnome.desktop.interface accent-color` (enum of 9 names) when the portal lacks `accent-color` | read `S/org.gnome.desktop.interface:314-320` | reader (portal only today) |
| GN-3.17 | OSD / toast accent override | Dark-mode standalone formula inside OSD and toasts in light mode | published `A/s/_common.scss:45-54`; `A/w/_misc.scss:44-50` | none |

### Motion

| ID | Property | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-3.18 | Motion tokens | Duration + easing per transition: `$button_transition` (bg, box-shadow 200 ms ease-out-quad), `$focus_transition` (outline colour/width/offset 200 ms ease-out-quad), `$backdrop_transition` (200 ms ease-out), `$row_transition` (bg, bg-image, radius 200 ms ease-out-quad), `$toggle_transition`, scrollbar `all 200ms linear`, tab bar 150 ms ease-in-out, bottom sheet 100 ms / 250 ms, needs-attention 150 ms ease-in | published `A/s/_common.scss:1-7`, `:114`; `A/w/_lists.scss:1-3`; `A/w/_toggle-group.scss:5-7`; `A/w/_scrolling.scss:4`; `A/w/_tab-view.scss:39`, `:59`; `A/w/_bottom-sheet.scss:10`, `:30` | none |
| GN-3.19 | Easing curves | `ease-out-quad` cubic-bezier; AdwEasing set (`ADW_EASE` = CSS ease etc.); AdwTimedAnimation default `ADW_EASE_OUT_CUBIC` | published `A/s/_common.scss:1`; `A/c/adw-easing.c:560-626`; `A/c/adw-timed-animation.c:275` | none |
| GN-3.20 | Springs | Damping ratio / mass / stiffness per animation: floating sheet (0.62, 500), bottom sheet (0.8, 400), navigation (1, 1, 1000), overlay split view / carousel / flap (1, 0.5, 500) | published (rows GN-1.220, 2.17, 2.18, 2.19, 2.38) | `dialog.entrance` (enum only) |
| GN-3.21 | Timed animations in C | Toast 300 / 300 / 500; tab bar 150–250; tab overview 400; entry-row title 150; view stack crossfade 200; style switch 250; spinner 1200-ms cycle | published `A/c/adw-toast-overlay.c:20-22`; `A/c/adw-tab.c:19`, `:28`; `A/c/adw-tab-overview.c:30`; `A/c/adw-entry-row.c:18`; `A/c/adw-view-stack.c:1653`; `A/c/adw-style-manager.c:18`; `A/c/adw-spinner-paintable.c:20` | none |
| GN-3.22 | GTK container defaults | GtkRevealer 250 ms, GtkStack 200 ms (used by expander rows, banners, search bars, menus, switcher bar) | published `G/gtkrevealer.c:717-719`, `:770`; `G/gtkstack.c:1021-1023`, `:2953` | none |
| GN-3.23 | Light/dark switch | 250 ms switch: on every style update libadwaita turns CSS transitions off for 250 ms, with no crossfade (corrected 2026-10-28 by the rationale's seventh check; it read "stylesheet crossfade window") | published `A/c/adw-style-manager.c:18`, `:237-239`, `:298-300` | none |
| GN-3.24 | Kinetic and animated scrolling | Deceleration friction 4, overshoot friction 20, max overshoot 100; keyboard/page scroll animation 200 ms | published `G/gtkscrolledwindow.c:181-189`, `:2319`, `:2388` | none |
| GN-3.25 | Reduced motion: what changes | `enable-animations` false → AdwAnimation jumps to the end (read ✓); `reduced-motion` → floating sheet fade only, navigation crossfade, tab overview crossfade, GtkSpinner sand-watch | read (exists: `reduce_motion`); published `A/c/adw-animation-util.c:41-66`; `A/c/adw-floating-sheet.c:482`; `A/c/adw-navigation-view.c:899`; `A/c/adw-tab-overview.c:2217`; `G/gtkspinner.c:158-161` | `AccessibilityPreferences.reduce_motion` (flag; the per-widget replacement motions are not in the model) |
| GN-3.26 | Reduced-motion key | `org.gnome.desktop.a11y.interface reduced-motion` (enum) besides the portal key | read `S/org.gnome.desktop.a11y.interface:18-24` | reader (portal + `enable-animations` today) |
| GN-3.71† | Window open, close, minimise, maximise (GNOME Shell) | Normal window opens from scale 0.01 × 0.05 about its bottom centre and opacity 0, 150 ms ease-out-expo; dialog opens from scale-y 0 about its centre, 100 ms ease-out-quad; close: 150 ms ease-out-quad to scale 0.8 about the centre and opacity 0 (dialog: 100 ms to scale-y 0); minimise / unminimise 400 ms ease-out-expo; the size-change animation, 250 ms ease-out-quad (a snapshot of the old frame scales to the new rectangle and fades out while the window scales from the old size to the new), runs for normal windows only, on every size change mutter announces: maximise, unmaximise, tiling (unless the tile follows a window being dragged), full screen, leaving full screen, and a move to another monitor; under reduced motion a normal window only fades in and out, a dialog appears at once and fades out on close, minimise / unminimise fade, and a size change is a crossfade: the window takes its new size at once while the old frame's snapshot fades out in place over 250 ms ease-out-quad; popups, menus and tooltips map without motion (only normal and dialog windows animate) | published `GS/windowManager.js:28-34`, `:1156-1290`, `:1299-1388`, `:1475-1554`, `:1571-1640`; `M/src/core/window.c:2944-2946`, `:3357-3359`, `:3658-3660`, `:3792-3793`, `:3865-3866`, `:4766-4767` | `dialog.entrance` (in-window only) (review 2026-10-28: the size-change animation extended to full screen, leaving it and monitor moves, from mutter's callers; its reduced-motion form, "to verify", settled from `:1345-1371`) |

### Focus

| ID | Property | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-3.27 | Focus ring per widget | Placement (inset / outer), offset and target node per widget: −2 (default), −1 (cards, rows, boxed lists), 0 (slider thumb, toggle group), +1 outer (switch, OSD / suggested buttons), +4 outer (legacy shortcut label), width 4 (colour swatches) | published `A/s/_drawing.scss:12-28` and each `@include focus-ring(…)` cited above | `defaults.focus_ring_width`, `focus_ring_offset` (one global pair) |
| GN-3.28 | Focus ring motion | Grows in from `offset + 4` (inset) with colour, width and offset animated 200 ms ease-out-quad | published `A/s/_drawing.scss:13-17`; `A/s/_common.scss:3-5` | none |
| GN-3.29 | Focus ring corner radius | Follows the target's radius (outline follows `border-radius`) — derived, but the target differs from the widget box (checkbutton includes the label; slider = thumb; notebook = checked tab) | published `A/w/_checks.scss:8`; `A/w/_scale.scss:29`; `A/w/_notebook.scss:2` | none |
| GN-3.30 | Focus-visible timeout | Keyboard focus ring hides after a timeout: key `keyboard-focus-visible-timeout` (−1 = toolkit default, 0 = forever); GTK default 3 s | read `S/org.gnome.desktop.a11y.interface:25-32`; published `G/gtksettings.c:958-969`, `G/gtkwindow.c:195`, `:6341-6349` | none |
| GN-3.31 | Focus ring HC colour | `$focus_border_color_hc` (accent 80 %) | published `A/s/_drawing.scss:24-26`; `A/s/_colors.scss:308` | `defaults.focus_ring_color` (the reader swaps it under HC; no HC field) |

### The text caret

| ID | Property | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-3.32 | Caret blink on/off | `cursor-blink` (default true) | read `S/org.gnome.desktop.interface:63-69`; `G/gtksettings.c:355` | none |
| GN-3.33 | Blink period | `cursor-blink-time` (default 1200 ms, range 100–2500) | read `S/org.gnome.desktop.interface:70-77`; `G/gtksettings.c:364` | none |
| GN-3.34 | Blink timeout | `cursor-blink-timeout` (default 10 s): caret stops blinking | read `S/org.gnome.desktop.interface:78-85`; `G/gtksettings.c:378` | none |
| GN-3.35 | Blink shape | Caret fades: visible 25 %, linear fade-out 25 %, hidden 25 %, fade-in 25 % of the cycle | published `G/gtktext.c:6858-6873`, `:2753` | none |
| GN-3.36 | Caret width | Stem = line height × `gtk-cursor-aspect-ratio` (0.04) + 1 (no gsetting) | published `G/gtksettings.c:397`; `G/gtkrenderlayout.c:136`, `:224`, `:265` | none |
| GN-3.37 | Secondary (split) caret | `-gtk-secondary-caret-color` and `gtk-split-cursor` for bidi text | published `G/gtkcssstylepropertyimpl.c:1454`; `G/gtksettings.c:388` | none |
| GN-3.38 | Caret in labels (keyboard navigation) | `always-show-text-caret` | read `S/org.gnome.desktop.a11y:12-19`; `GDK/wayland/gdksettings-wayland.c:305` | none (review 2026-10-28: the open range `:12-…` closed) |
| GN-3.39 | Caret colour per widget | `caret-color: currentColor` on entries, labels, text views; accent on drop target | published `A/w/_entries.scss:11`; `A/w/_labels.scss:2`; `A/w/_views.scss:21`, `:42-44`; `A/s/_common.scss:73-78` | `input.caret_color` (input only) |
| GN-3.73† | Caret visibility rules | The caret is drawn only in the focused widget and only while no text is selected (a selection hides it), and not at all when nothing is displayed; while a drag hovers an entry a second caret marks the drop position | published `G/gtktext.c:2739-2756` (GtkText); GtkTextView the same: no caret while text is selected (`G/gtktextlayout.c:1790-1795`) and none once the view loses focus (`G/gtktextview.c:5957-5961`) | `input.caret_color` only (review 2026-10-28: "GtkTextView: to verify" settled) |

### Selection

| ID | Property | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-3.40 | Text selection, unfocused | `selection` without `:focus-within`: view fg 10 % | published `A/s/_common.scss:64-66` | `defaults.text_selection_background` (focused only); `selection_inactive_background` is the list one |
| GN-3.41 | Text selection keeps text colour | `color: transparent` → selected text keeps its own colour | published `A/s/_common.scss:66` | `defaults.text_selection_color` (expects a colour) |
| GN-3.42 | Selected + hover / pressed (lists, grids, cards) | Accent 32 % / 39 % | published `A/s/_colors.scss:276-278` | none |
| GN-3.43 | Neutral selection (sidebars, menus, tab bars, flat toggles) | `$selected_color` 10 % / 13 % / 19 % of currentColor, not accent | published `A/s/_colors.scss:270-272` | `sidebar.selection_background` (one colour) |
| GN-3.66† | Drop-target highlight | A catch-all rule draws every non-window widget under a drop with `border-color`, a 1 px inset ring and the caret in `$drop_target_color` (= `--accent-bg-color`); lists, column views, tab views, icon views, the places sidebar, stack switchers and expanders switch it off and draw their own: button 2 px ring + accent text (GN-1.25), entry 1 px (GN-1.46), text view accent caret (GN-1.53), tree view 1 px line before / after a row (GN-1.279†), icon view 1 px box (GN-1.280†), places-sidebar insertion row (2 px high accent bar, padding 0 8), colour swatch 2 px ring, tab overview ring and fill (GN-2.35); a drop-down hands it to its button | published `A/s/_common.scss:73-87`; `A/w/_lists.scss:34-37`; `A/w/_column-view.scss:8-10`; `A/w/_file-chooser.scss:34-46`; `A/w/_color-chooser.scss:73-81`; `A/w/_dropdowns.scss:20-24`; `A/w/_tab-view.scss:300-304`; `A/s/_colors.scss:301` | none |

### Window chrome and the unfocused window

| ID | Property | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-3.44 | Backdrop state per widget | Which elements change when the window loses focus: header bar, search bar, action bar, tab bar, toolbar-view raised bars, banners, sidebars (fills); their contents (opacity 0.5); window shadow — all with 200 ms ease-out | published `A/w/_header-bar.scss:41-51`; `A/w/_toolbars.scss:133-141`, `:215-223`, `:260-268`, `:302-311`; `A/w/_tab-view.scss:13-23`; `A/w/_sidebars.scss:5-8`; `A/w/_window.scss:19-27` | `window.inactive_title_bar_*`, `defaults.selection_inactive_background` |
| GN-3.45 | Window chrome geometry | Withdrawn (review 2026-10-28): duplicate, see GN-1.11–GN-1.22. Text kept: Title-bar height, padding, control buttons — GN-1.11…1.22 | published | none |
| GN-3.46 | Pointer cursor theme and size | `cursor-theme`, `cursor-size` (default 24; larger for a11y) | read `S/org.gnome.desktop.interface:180-189`; `GDK/wayland/gdksettings-wayland.c:272-273` | none |
| GN-3.67† | Pointer shape per element | GTK sets the cursor per widget: `text` over entries, text views and selectable labels, `pointer` over links and link buttons, `col-resize` / `row-resize` over paned handles, `crosshair` on the colour plane, `nw-resize` … `se-resize` on the resize zone (GN-1.282†); the pointer is hidden (`none`) while typing in an entry or text view until it moves | published `G/gtktext.c:3382`, `:7291`; `G/gtktextview.c:2001`, `:5300-5312`; `G/gtklabel.c:804-806`; `G/gtklinkbutton.c:382`; `G/gtkpaned.c:390-397`, `:1528`; `G/gtkcolorplane.c:193`; `G/gtkwindow.c:1743-1761` | none |
| GN-3.72† | Parent window under an attached modal dialog (GNOME Shell) | The parent is dimmed: brightness −0.3 (`DIM_BRIGHTNESS`), dim 500 ms / undim 250 ms ease-out-quad, only while mutter attaches modal dialogs (`Meta.prefs_get_attach_modal_dialogs`): `org.gnome.mutter attach-modal-dialogs`, false in mutter's schema, set true by gnome-shell's override for GNOME sessions | published `GS/windowManager.js:36-38`, `:126-160`; read `M/data/org.gnome.mutter.gschema.xml.in:26-33`, `GS-data/00_org.gnome.shell.gschema.override:1-2` | none (review 2026-10-28: the key's default, "to verify", settled as GN-SF21† found it) |

### Visible timings (behaviour that shows)

| ID | Property | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-3.47 | Double-click time and distance | `double-click` 400 ms (read); distance 5 px (GTK default, no gsetting) | read `S/org.gnome.desktop.peripherals:243-247`; published `G/gtksettings.c:333`, `:343` | none |
| GN-3.48 | Drag threshold | `drag-threshold` 8 | read `S/org.gnome.desktop.peripherals:248-252`; `G/gtksettings.c:430` | none |
| GN-3.49 | Long-press time | 500 ms (GTK default, no gsetting) | published `G/gtksettings.c:943` | none |
| GN-3.50 | Mnemonic underline delay | Underlines appear 300 ms after Alt is held | published `G/gtkwindow.c:193`; `G/gtkpopover.c:160` | none |
| GN-3.51 | Drag-hover activation | 500 ms on expanders, view switchers, sidebars, tab bars and tab grids | published `G/gtkexpander.c:136`; `A/c/adw-view-switcher-button.c:14`; `A/c/adw-sidebar.c:23`; `A/c/adw-tab-box.c:26`; `A/c/adw-tab-grid.c:25` | none |
| GN-3.52 | Submenu open delay | Withdrawn (review 2026-10-28): duplicate, see GN-1.77. Text kept: 80 ms (GN-1.77) | published `G/gtkmodelbutton.c:1423` | none |
| GN-3.53 | Tooltip browse timings | Withdrawn (review 2026-10-28): duplicate, see GN-1.82. Text kept: GN-1.82 | published `G/gtktooltip.c:71-73` | `tooltip.show_delay` |
| GN-3.54 | Scrollbar auto-hide | Withdrawn (review 2026-10-28): duplicate, see GN-1.91. Text kept: GN-1.91 | published `G/gtkscrolledwindow.c:191-196` | `scrollbar.overlay_mode` |
| GN-3.55 | Toast timeout | Withdrawn (review 2026-10-28): duplicate, see GN-2.12 (its "default timeout 5 s", same source line). Text kept: 5 s default | published `A/c/adw-toast.c:475` | none |
| GN-3.68† | Key repeat | `repeat` (true), `delay` 500 ms, `repeat-interval` 30 ms: how fast a held key repeats characters and caret moves; GTK receives it as the compositor's `wl_keyboard.repeat_info` | read `S/org.gnome.desktop.peripherals:144-159`; `GDK/wayland/gdkseat-wayland.c:1581-1599` | none |
| GN-3.69† | Widget auto-repeat | Spin-button arrows: first repeat after 500 ms, then every 50 ms, the step growing by `climb-rate` after 5 repeats; slider and scrollbar trough paging: 500 ms, then every 250 ms. GTK 4 uses these constants: the gsettings keys `gtk-timeout-initial` / `gtk-timeout-repeat` still exist, but `G/gtksettings.c` has no such property (grep) | published `G/gtkspinbutton.c:63-67`, `:1374-1400`; `G/gtkrange.c:73-74`, `:2923-2938`; `S/org.gnome.desktop.interface:190-199` | none |

### Text rendering and icons

| ID | Property | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-3.56 | Antialiasing / hinting / subpixel order / rendering mode | `font-antialiasing`, `font-hinting`, `font-rgba-order`, `font-rendering` | read `S/org.gnome.desktop.interface:277-312`; `GDK/wayland/gdksettings-wayland.c:292-300` | none |
| GN-3.57 | Icon stroke weight | `-gtk-icon-weight` (134 large icons and status pages, 267 spinner page) | published `A/s/_common.scss:38-41`; `A/w/_misc.scss:137`, `:163`, `:185` | none |
| GN-3.58 | Symbolic icon palette | `-gtk-icon-palette`: success, warning, error, accent standalone colours recolour symbolic icons | published `A/s/_common.scss:19-22` | none (icons are drawn in `currentColor` by connectors) |
| GN-3.59 | Icon sizes by context | 14 (checks, menu checks), 16 (normal ✓), 32 (large ✓), 64 / 96 / 128 (status page), 128 (about) | published `A/w/_checks.scss:25`; `A/w/_misc.scss:136`, `:162`, `:184`; `A/w/_dialogs.scss:53-55` | `defaults.icon_sizes.*` (five contexts; none of these) |
| GN-3.60 | Icon opacity in widgets | Entry icons 70 / 100 / 80 %, menu arrows 30 %, notebook close 30 %, expander arrow 70 % | published (GN-1.44, 1.74, 1.120, 1.261) | none |

### Accessibility variants

| ID | Property | What is missing | Source | Nearest model field |
|---|---|---|---|---|
| GN-3.61 | High-contrast variant of the theme data | Global changes: `--border-opacity` 15 → 50 %, `--dim-opacity` 55 → 90 %, `--disabled-opacity` 50 → 40 %, trough 15/20/25 → 30/40/50 %, focus ring 50 → 80 %, window outline 7 → 30 %, dimmer 30 → 80 %; plus every per-widget HC rule (inset 1px rings on buttons, entries, rows, menus, sidebars, toggle groups, tabs; title-bar and bar lines turn `$border_color`; scrollbar 40/60/100 %) | published `A/s/_colors.scss:257-261`, `:284-286`, `:304`, `:308-310`, `:323-329`; per-widget rows above | `AccessibilityPreferences.high_contrast` (flag; the reader only swaps the focus ring, `gnome/mod.rs:168-183`) |
| GN-3.62 | Status shapes | Withdrawn (review 2026-10-28): duplicate, see GN-1.202 (the same key; GtkSwitch is the only GTK 4.24.1 widget that reads `gtk-show-status-shapes`, `G/gtkswitch.c:727`, `:739`, grep). Text kept: `show-status-shapes` (switch on/off glyphs; GTK says "widgets like switches") | read `S/org.gnome.desktop.a11y.interface:11-17`; published `G/gtksettings.c:586-594`, `G/gtkswitch.c:720-743` | none |
| GN-3.63 | Text scaling and `sp` | `text-scaling-factor` read ✓; libadwaita sizes some widths in `sp` (alert dialog widths, clamps with sp unit) that scale with it | read ✓; published `A/c/adw-alert-dialog.c:175-177`; `A/c/adw-clamp-layout.c:166-169` | `AccessibilityPreferences.text_scaling_factor` (exists; which geometry scales is not marked in the model) |
| GN-3.64 | Large pointer | Withdrawn (review 2026-10-28): duplicate, see GN-3.46. Text kept: `cursor-size` (GN-3.46) | read | none |
| GN-3.65 | Always-visible caret | Withdrawn (review 2026-10-28): duplicate, see GN-3.38. Text kept: GN-3.38 | read | none |
| GN-3.70† | Visual bell | `visual-bell` (false) and `visual-bell-type` (`'fullscreen-flash'` default, or `'frame-flash'`): an alert flashes the screen or the window that rang it, besides or instead of the sound (`audible-bell`); mutter draws it, and its source states the flash's colour, opacity, timing and rate limits: GN-P3-14† | read `S/org.gnome.desktop.wm.preferences:207-247`; published `M/src/compositor/compositor.c:1454-1545`; `M/src/core/bell.c:60-61`, `:203-225` | none (review 2026-10-28: the flash was left to measure; mutter 51.0 publishes it, so the source kind measure → published and the values are GN-P3-14†'s; "GNOME Shell draws it" → mutter) |

---

## Appendix — discrepancies found in passing (not gaps; for the platform-facts / reader owners)

Each is what libadwaita 1.10.0 / GTK 4.24.1 says against what `docs/platform-facts.md` or the reader states today; each needs its own check before any change.

1. §2.7 tooltip `border.shadow_enabled` "(Adwaita CSS) box-shadow" — the tooltip sets `box-shadow: none` (`A/w/_tooltip.scss:15`).
2. §2.6 menu `border.corner_radius` "(none) — items are rectangular" — `modelbutton { border-radius: $menu_radius }`, 9 (`A/w/_menus.scss:138`; `A/s/_common.scss:10`).
3. §2.15 `alternate_row_background` "(Adwaita CSS) even row" — no `even`/`odd` rule exists in libadwaita 1.10.0's stylesheet (grep of `src/stylesheet/`).
4. §2.5 `border.corner_radius` "← defaults.border.corner_radius" (9) — a check is `$check_radius` 6 (`A/w/_checks.scss:1`, `:67-68`).
5. §2.5 `label_gap` 8 — `checkbutton { border-spacing: 4px }` (`A/w/_checks.scss:4`); to verify what else adds space.
6. §2.11 `item_gap` "(GTK) 0" — `notebook > header.top > tabs > tab { margin-left: 4px; margin-right: 4px }` (`A/w/_notebook.scss:181-184`); the cell cites `_notebook.scss:119-124` only.
7. §2.9 `tick_mark_length` "(none) — no ticks" — GtkScale draws marks 6 px long (`A/w/_scale.scss:21`, `:119-122`).
8. §2.10 `border.corner_radius` "← defaults" (9) — trough and progress are `99px` pills; §1.4.4 / §2.10 `min_width` 80 — the horizontal trough's `min-width` is 150, 80 is the vertical `min-height` (`A/w/_progress-bar.scss:5`, `:12`).
9. §2.23 note "rotating `process-working-symbolic` icon" — GTK 4.24.1's GtkSpinner plays `process-working.gpa` (animated SVG), `sand-watch.svg` under reduced motion (`G/gtkspinner.c:158-161`).
10. §2.1.3 `selection_inactive_background` "(none) — `:backdrop`" — unfocused text selection is a distinct colour (view fg 10 %, `A/s/_common.scss:64-66`).
11. Reader: `window.title_bar_font` from `titlebar-font` — libadwaita's title is the UI font in bold (`A/w/_header-bar.scss:213-218`), and `titlebar-uses-system-font` defaults to true (`S/org.gnome.desktop.wm.preferences:179-185`).
12. Reader: the portal accent is used as given; libadwaita snaps it to the nearest of its 9 accents (`A/c/adw-settings-impl-portal.c:140`, `A/c/adw-accent-color.c:160`) and sets `accent_fg_color` white (`A/c/adw-style-manager.c:160-161`).

---

## Cross-check (pass 2, 2026-10-28)

The GNOME rows checked against the other five surveys: every control they list that this file did not (A), every capability C01–C28 (B), every "(none)" cell of the GNOME column of `docs/platform-facts.md` (C). Rows added in this pass carry a trailing "†" and sit at the end of their section's table.

Sources new in this pass, besides those at the top:

| Short | Source |
|---|---|
| `G/deprecated/F:L` | GTK 4.24.1, `gtk/deprecated/F` (fetched from gitlab.gnome.org at the tag) |
| `GS/F:L` | gnome-shell tag **51.1** (commit `33da06854e229c256afd5872fedd83c28cbfa176`), `js/ui/F` |
| GTKgal | GTK 4.24.1, `docs/reference/gtk/visual_index.md` (the widget gallery: every widget GTK ships) |
| ADWgal | libadwaita 1.10.0, `doc/widget-gallery.md` (every widget libadwaita ships) |

### A. Controls other surveys list

Decision: **1** = GNOME draws it as a control of its own (a §4.2 row); **2** = GNOME draws it inside an element the model has (the §4.1 rows that cover it); **3** = GNOME has no such control.

| Control | Decision | Rows, or reason and source |
|---|---|---|
| bottom tab bar | 1 | AdwViewSwitcherBar: GN-2.15 (buttons, bar padding 7, reveal 250 ms) in a GtkActionBar, GN-2.20 |
| date picker | 3 | No date picker or date entry: GTKgal and ADWgal list only GtkCalendar (GN-2.25), which apps place in a menu-button popover |
| pull-to-refresh | 3 | No such widget in GTKgal or ADWgal; GTK draws only the overshoot at a scroll edge (GN-3.06) |
| breadcrumb/path control | 1 | GN-2.52† (GtkPathBar of GTK's file chooser; it is private to the file chooser, GTKgal lists no public path bar) |
| carousel | 1 | AdwCarousel: GN-2.38 (filed under "page control/pager" in pass 1); its indicator geometry GN-2.55† |
| colour picker | 1 | GN-2.27 (swatches; filed under "colour button"); the editor page GN-2.53† |
| dial/circular slider | 3 | No dial in GTKgal or ADWgal; the nearest, GtkScaleButton, pops up a straight vertical scale (GN-2.54†) |
| fab | 3 | No floating action button in ADWgal or libadwaita's style classes (`doc/style-classes.md`); an app overlays a `.circular` / `.pill` `.suggested-action` button (GN-1.28, GN-1.30) |
| find bar | 1 | GtkSearchBar GN-2.21 with GtkSearchEntry GN-2.22; GtkTreeView's type-ahead search popover is in GN-1.279† |
| menu button | 2 | `[button]`: GN-1.26 (`.has-open-popup`), GN-1.27 (arrow-button padding), GN-1.28 (circular / pill menu buttons), GN-1.40 (arrow glyphs) |
| navigation rail | 3 | No rail in ADWgal; narrow layouts collapse the sidebar (AdwNavigationSplitView / AdwOverlaySplitView, GN-2.19) or use AdwViewSwitcherBar (GN-2.15) |
| rating | 3 | No rating widget in GTKgal or ADWgal |
| time picker | 3 | No time picker in GTKgal or ADWgal |
| token field | 3 | No token / tag entry in GTKgal or ADWgal |
| access key badge | 3 | GTK marks access keys by underlining the mnemonic after 300 ms of Alt (GN-3.50), no badge |
| action sheet | 2 | `[dialog]`: AdwAlertDialog presented as a bottom sheet on narrow windows (GN-1.222, GN-2.17), stacked responses (GN-1.284†) |
| autocomplete | 1 | GN-2.56† (GtkEntryCompletion); emoji completion is in GN-2.43 |
| back button | 2 | `[window]` title bar: GN-1.278† (AdwBackButton) |
| background extension | 3 | No content-under-sidebar extension in ADWgal; split views place the content beside the sidebar (GN-2.19) |
| bottom accessory | 1 | AdwBottomSheet's bottom bar, the always-visible bar that opens the sheet: GN-2.17 |
| collection view | 2 | `[list]`: GtkGridView / GtkFlowBox items GN-1.174; deprecated GtkIconView GN-1.280† |
| column browser | 3 | No column browser in GTKgal or ADWgal |
| context menu preview | 3 | A GTK context menu is a GtkPopoverMenu pointing at the click (GTKgal "PopoverMenu"); there is no lifted preview |
| dock/panel title bar | 3 | No dock or MDI widgets in GTKgal or ADWgal; panes are GtkPaned (`[splitter]`) |
| drag and drop | 2 | Drag icon GN-2.49, dragged tab GN-1.133, drag threshold GN-3.48, drop-target highlight GN-3.66† |
| edit menu | 2 | `[menu]`: the right-click menu of a text widget is a popover menu; on touch, GN-1.286† (selection bubble) with the handles GN-2.42 |
| full-screen dialog | 3 | AdwDialog's presentation modes are auto, floating and bottom sheet (`A/c/adw-dialog.h:25-27`); none fills the window |
| glass container | 3 | No glass material: libadwaita sets no `backdrop-filter` (grep of `src/stylesheet`); the nearest is the OSD fill (GN-3.09) |
| ink toolbar | 3 | No ink or pen widgets in GTKgal or ADWgal |
| inspector/utility panel | 3 | No utility-panel window or inspector widget in GTKgal's "Windows" group or ADWgal; side inspectors are AdwOverlaySplitView panes (GN-2.19) |
| keyboard | 3 | The on-screen keyboard is GNOME Shell's, drawn outside the app's window in the shell's theme; GTK only passes `input-purpose` / `input-hints` to the compositor, which choose a layout, not a look |
| large content viewer hud | 3 | No such HUD in GTKgal or ADWgal |
| list box | 2 | `[list]`: GtkListBox is the `list` node, GN-1.158–1.166 |
| loading indicator | 2 | `[spinner]`: AdwSpinner GN-1.226–1.232 (the only indeterminate indicator besides the progress bar's pulse, GN-1.113) |
| media transport controls | 1 | GN-2.54† (GtkMediaControls, GtkScaleButton); the overlay play image GN-2.44 (filed under "icon button" in pass 1) |
| paste control | 3 | No paste-permission control: GTK reads the clipboard without a confirmation UI (GTKgal has none) |
| picker wheel | 3 | No wheel picker in GTKgal or ADWgal |
| rubber band | 2 | `[list]`: GN-1.175 (list and grid views); GtkTreeView and GtkIconView use the same `rubberband` node (GN-1.279†, GN-1.280†) |
| rule editor | 3 | No rule editor in GTKgal or ADWgal |
| scrim | 2 | `[dialog]`: GN-1.210; bottom sheet and split-view dimming GN-3.04 |
| scroll edge effect | 2 | Undershoot and overshoot GN-3.06; under toolbar-view bars GN-1.156; column-view top GN-1.178 |
| scrollbar annotation | 3 | GtkScrollbar draws no marks or labels (GN-1.84–1.93; GTKgal "Scrollbar") |
| swipe actions | 3 | No swipe-to-reveal row actions in ADWgal; AdwSwipeTracker drives only navigation, carousels, sheets and split views (GN-2.18, GN-2.38, GN-2.17, GN-2.19) |
| teaching tip | 3 | No teaching tip in GTKgal or ADWgal; an app points a plain GtkPopover at a widget (`[popover]`) |
| text ruler | 3 | GtkTextView has no ruler (GTKgal) |
| tool box | 3 | No QToolBox-like accordion; the nearest is a column of expander rows (GN-2.09) or GtkExpanders (`[expander]`) |
| toolbar flyout | 3 | Header bars and toolbars have no overflow menu; apps add a primary menu button (`open-menu-symbolic`, GN-1.40) themselves |
| touch bar | 3 | No hardware touch bar on the platform |
| tree view | 2 | `[list]`: GtkTreeExpander in list / column views GN-1.173; the deprecated GtkTreeView GN-1.279† |
| writing tools | 3 | No writing-assistant UI in GTKgal or ADWgal |
| zoom button | 3 | No semantic-zoom control in GTKgal or ADWgal; AdwTabOverview (GN-2.35) is the only zoomed-out view and has no zoom button |

Pass 1 filed eight §4.2 rows under a control category GNOME does not have (the pass-1 control-category tag from which §4.2's table of the main map is built), so the main map shows GNOME as having controls it lacks: GN-2.40 (keycap shortcut label) as "help button", GN-2.41 (shortcuts dialog) as "about dialog", GN-2.42 (text selection handles) as "tool button", GN-2.43 (emoji chooser) as "chip", GN-2.44 (video play overlay) as "icon button", GN-2.45 (editable label) as "search field", GN-2.46 (wrap box) as "button group/toggle group", GN-2.49 (drag icon) as "image frame". The controls behind those names, decided here:

| Control | Decision | Rows, or reason and source |
|---|---|---|
| help button | 3 | No help or "What's This" button in GTKgal or ADWgal |
| tool button | 2 | GTK 4 has no tool-button class (GTKgal); toolbar and header-bar buttons are flat buttons: GN-1.29, GN-1.151 |
| chip | 3 | No chip or tag widget in GTKgal or ADWgal |
| icon button | 2 | `[button]`: image-only buttons GN-1.27, `.circular` GN-1.28, `.flat` GN-1.29 |
| image frame | 3 | GtkImage and GtkPicture draw no frame (GTKgal); a frame is GtkFrame, GN-2.32 |
| button group/toggle group | 1 | AdwToggleGroup is `[segmented_control]` (GN-1.239–1.251); linked buttons GN-1.42 |
| search field | 1 | GN-2.21, GN-2.22 (unchanged; GN-2.45 belongs to `[input]`) |
| about dialog | 1 | GN-2.39 (unchanged; GN-2.41 is the shortcuts dialog) |

### B. Capabilities

| Capability | Rows added, or why no further row |
|---|---|
| C01 Shadow geometry | No further row: every surface's shadow already has its row (GN-3.01 lists them) |
| C02 Elevation | No further row: libadwaita has no elevation levels and no surface tint; each surface states its own shadow layers (GN-3.01) |
| C03 Materials | No further row: libadwaita 1.10.0 sets no `backdrop-filter` (grep of `src/stylesheet`), although GTK 4.24.1 parses the property (`G/gtkcssstylepropertyimpl.c:1435`); translucency is alpha fills (GN-3.08) and the OSD fill (GN-3.09); there is no reduce-transparency setting (GN-3.10) |
| C04 Colour composition | No further row of its own; A's rows carry it (GN-1.279†, GN-2.55†) |
| C05 Colour roles | No further row: GN-1.268–1.277 hold the roles libadwaita defines |
| C06 Colour variant axes | GN-3.72† (the parent window dimmed under an attached modal dialog) |
| C07 States | No further row of its own; A's rows carry it (GN-1.279†, GN-1.280†, GN-2.52†, GN-3.66†) |
| C08 State-layer model | No further row: hover and pressed are fixed currentColor fills (GN-3.08, GN-1.29), not overlay layers with opacities; no ripple |
| C09 Variants | No further row of its own; GN-2.57† adds the composite dialogs' flat title bars |
| C10 Size variants and density | No further row: libadwaita has no density or control-size setting; adaptivity is breakpoints (GN-1.197) and the `.compact` / `.short` variants already tagged |
| C11 Shape | No further row |
| C12 Border model | No further row of its own; A's rows carry it (GN-1.279† dotted lines, GN-3.66† rings) |
| C13 Parts | No further row of its own; A's rows carry it (GN-1.278†, GN-1.286†, GN-2.52†–2.55†, GN-2.58†) |
| C14 Motion | GN-3.71† (window motion drawn by GNOME Shell). Popovers, menus and tooltips open with no motion: `G/gtkpopover.c`, `G/gtktooltip.c` and `G/gtktooltipwindow.c` contain no animation (grep), and the shell animates only normal and dialog windows (`GS/windowManager.js:1512-1554`) |
| C15 Focus indication | No further row of its own; GN-2.53† adds the colour plane's outer ring |
| C16 Text caret | GN-3.73† (caret hidden while text is selected; the drop-position caret) |
| C17 Selection and highlights | GN-1.281† (IME pre-edit, including the `.spelling-error` hint), GN-3.66† (drop-target highlight). GTK's text widgets have no find highlight of their own |
| C18 Typography | No further row: libadwaita sets no `letter-spacing` or `font-variation-settings` (grep); line heights GN-1.190, features GN-1.192 / 1.193; C's GN-1.283† adds a width stated in characters |
| C19 Icons | GN-1.285† (a labelled menu item never shows its icon). The rest is in GN-3.57–3.60; GTK 4.24.1 has `-gtk-icon-filter` and `-gtk-icon-style` (`G/gtkcssstylepropertyimpl.c:1297`, `:1309`) but libadwaita sets neither (grep), so disabled icons fade with their widget's opacity |
| C20 Window chrome | GN-1.282† (resize zone), GN-3.71†, GN-3.72† |
| C21 Visible timings | GN-3.68† (key repeat), GN-3.69† (spin-button and trough auto-repeat); A's rows add the video controls' 3 s (GN-2.54†), completion's 100 ms (GN-2.56†) and the tree view's 5 s search popover (GN-1.279†) |
| C22 Pointer | GN-3.67† (pointer shape per element, hidden while typing) |
| C23 Accessibility variants | GN-3.70† (visual bell) |
| C24 Whole controls | GN-2.52†–GN-2.58† (from A) |
| C25 Reader coverage | GN-3.68† (`org.gnome.desktop.peripherals.keyboard`), GN-3.70† (`visual-bell`, `visual-bell-type`) |
| C26 Layout and placement | GN-1.284† (alert response order, reversed when stacked) |
| C27 Gradient paint | No further row beyond A's GN-2.53† (hue and alpha gradients); libadwaita's other gradients already have rows (GN-1.10, GN-2.14, GN-3.05, GN-3.06) |
| C28 Colour space | No further row: libadwaita states its colours in sRGB, or derives them in Oklab from sRGB inputs (`A/s/_colors.scss:25-29`, `:145-170`); GTK 4.24.1's colour states (`GDK/gdkcolorstate.c`) serve content, not the stylesheet |

### C. The "(none)" cells of the GNOME column

**a** = GNOME does not draw it, the cell is right; **b** = GNOME draws it: the row that covers it.

| platform-facts line | a / b | Row, or reason |
|---|---|---|
| 1061 `selection_inactive_background` | b | GN-3.40: text selection in an unfocused widget is view fg 10 %. List selection has no `:backdrop` rule (grep of `A/w`), so in an unfocused window it stays as it is; the cell's reason ("`:backdrop` handles this") names a rule that does not exist (Appendix item 10) |
| 1130 `reduce_transparency` | a | No such GNOME setting (GN-3.10) |
| 1139 icon size `dialog` | a | AdwAlertDialog and GTK 4's GtkMessageDialog draw no icon (no image in `A/c/adw-alert-dialog.ui` or `G/gtkmessagedialog.c`); the large icons are the status page's and the about dialog's (GN-3.59) |
| 1140 icon size `panel` | a | The toolkit has no panel; GNOME Shell's top bar is not drawn by the app |
| 1167 `inactive_title_bar_text_color` | b | GN-1.16: the bar's contents fade to opacity 0.5, not to a colour |
| 1170 window `border.padding_horizontal` | a | The window draws no padding; content margins are the widgets' own |
| 1171 window `border.padding_vertical` | a | As 1170 |
| 1180 button `border.color` | a | No border at rest; the high-contrast ring is GN-1.38 |
| 1206 input `border.color` | a | No border at rest; the high-contrast ring is GN-1.51 |
| 1216 input `border.shadow_enabled` | a | No shadow |
| 1218 input `hover_border_color` | a | No border, so no hover border; the high-contrast ring does not change on hover (GN-1.51) |
| 1236 checkbox `border.shadow_enabled` | a | No drop shadow; the ring is an inset shadow (GN-1.59) |
| 1247 checkbox `unchecked_background` | a | Transparent at rest; the pressed fill is GN-1.55 |
| 1263 menu `border.color` | b | GN-1.179: the popover's outline is its first shadow layer, `0 0 0 1px` (GN-3.02) |
| 1264 menu `border.line_width` | b | GN-1.179 (1 px, as a shadow spread) |
| 1265 menu `border.corner_radius` | b | GN-1.68: items are rounded, 9; the cell ("items are rectangular") is wrong (Appendix item 2) |
| 1266 menu `border.shadow_enabled` | b | GN-1.179 |
| 1285 tooltip `max_width` | b | GN-1.283†: the label wraps at 50 characters |
| 1322 slider `tick_mark_length` | b | GN-1.100: marks are 6 px; the cell ("no ticks") is wrong (Appendix item 7) |
| 1338 progress bar `border.shadow_enabled` | a | No shadow |
| 1339 progress bar `border.color` | a | No border at rest; the high-contrast inset border is GN-1.115 |
| 1366 tab `active_outward_corner_radius` | a | Notebook tabs are square; AdwTabBar tabs are rounded on all four corners (GN-1.124), with no outward curve |
| 1377 sidebar `border.corner_radius` | a | A docked sidebar pane is square; its rows are rounded (GN-1.140) |
| 1378 sidebar `border.shadow_enabled` | a | No shadow when docked; the overlaid pane's transition shadow and dimming are GN-2.19 |
| 1390 toolbar `bar_height` | a | No fixed height: the bar is its padding plus its tallest child; the 47 is the header bar's (GN-1.11) |
| 1396 toolbar `border.color` | a | `.toolbar` draws no border; AdwToolbarView's `.border` line is GN-1.154 |
| 1397 toolbar `border.corner_radius` | a | Square; the `.osd` toolbar's 15 is GN-1.149 |
| 1399 toolbar `border.shadow_enabled` | b | GN-1.153: AdwToolbarView's raised bars cast a two-layer shadow |
| 1412 status bar `border.corner_radius` | a | Square |
| 1413 status bar `border.shadow_enabled` | a | No shadow |
| 1430 list `grid_color` | b | GN-1.169 (column separators), GN-1.162 (row separators), GN-1.279† (GtkTreeView's dotted grid and tree lines) |
| 1439 list `border.corner_radius` | a | A plain list is square; the boxed list's 12 is GN-1.165 |
| 1441 list `border.shadow_enabled` | a | A plain list has none; the boxed list's is GN-1.165 |
| 1532 switch `unchecked_track_border_color` | a | No border in any state |
| 1533 switch `hover_unchecked_track_border_color` | a | As 1532 |
| 1534 switch `disabled_unchecked_track_border_color` | a | As 1532 |
| 1535 switch `checked_track_border_color` | a | As 1532 |
| 1536 switch `disabled_checked_track_border_color` | a | As 1532 |
| 1538 switch `thumb_border_color` | a | No border; the thumb's edge is its shadow (GN-1.201) |
| 1539 switch `hover_thumb_border_color` | a | As 1538 |
| 1540 switch `disabled_thumb_border_color` | a | As 1538 |
| 1554 dialog `min_height` | a | Sizes to its content |
| 1555 dialog `max_height` | b | GN-1.218: bounded by the window less the floating sheet's vertical margins |
| 1568 dialog `icon_size` | a | No icon (as 1139) |
| 1600 spinner `min_diameter` | b | GN-1.228: AdwSpinner's minimum is 16; the cell is wrong |
| 1601 spinner `stroke_width` | b | GN-1.227: AdwSpinner's stroke is radius / 8; the cell ("icon-based") describes GtkSpinner only |
| 1614 combo box `border.color` | a | No border at rest; the high-contrast ring is GN-1.38 |
| 1617 combo box `min_width` | a | Sizes to its content |
| 1620 combo box `arrow_area_width` | a | No separate arrow area: the arrow is a 16 px inline icon (GN-1.40) after a 6 px gap (GN-1.233) |
| 1622 combo box `border.shadow_enabled` | a | No shadow |
| 1635 segmented `font.color` | a | No colour of its own: an unchecked toggle inherits its parent's foreground; the checked one uses `--active-toggle-fg-color` (`A/w/_toggle-group.scss:80-81`, GN-1.277) |
| 1639 segmented `separator_width` | b | 1 px: `separator { min-width: 1px; min-height: 1px }` (`A/w/_misc.scss:20-23`) applies to the toggle group's separators, whose margins and fade are GN-1.243; the cell is wrong, the value is published |
| 1641 segmented `border.corner_radius` | b | GN-1.239: the group's radius is 9 (`$button_radius`); the cell is wrong |
| 1642 segmented `border.color` | a | No border at rest; the high-contrast ring is GN-1.251 |
| 1643 segmented `border.line_width` | a | As 1642 |
| 1644 segmented `border.shadow_enabled` | b | GN-1.240: the checked toggle's two-layer shadow |
| 1646 segmented `disabled_opacity` | b | GN-1.242 |
| 1658 card `border.color` | b | GN-1.252: the outline is the first shadow layer |
| 1659 card `border.line_width` | b | GN-1.252 (1 px, as a shadow spread) |
| 1662 card `border.padding_horizontal` | a | Set by the app |
| 1663 card `border.padding_vertical` | a | As 1662 |
| 1697 link `background_color` | a | Inline text, no fill; `button.link` is transparent (GN-1.266) |
| 1698 link `hover_background` | a | Hover changes the colour (`A/w/_links.scss:11-13`), not a fill |

Tallies: A — 51 listed controls: 8 decided 1, 12 decided 2, 31 decided 3 (plus the 8 categories pass 1 misfiled). B — 12 capabilities gained rows, 16 need no further row. C — 63 cells: 44 a, 19 b; seven b cells state a wrong value or reason (1061, 1265, 1322, 1600, 1601, 1639, 1641).

## Seed follow-up (2026-10-28)

Rows for the four seed groups no row covered after pass 2 (G12, G19, G24, G27; the main
map's §5). The source abbreviations are defined in
`todo_v0.7.1_model-completeness-gap-map-seeds.md`, *Seed follow-up sources*. CAPS and SRC
are the row's capability tags (main map §4.3) and source kinds.

### Code editor and rich text (G12, G27)

**Code editor.** libadwaita and GTK have no code view; the GNOME code editor component is
GtkSourceView (a GNOME library, not part of libadwaita; whether it counts as the platform is §6.2's
question). It ships the style schemes *Adwaita* and *Adwaita-dark*, which name each other as their
variants (`variant` / `dark-variant` / `light-variant` metadata, read with
`gtk_source_style_scheme_get_metadata`, since 5.4). A buffer whose app sets no scheme uses
**classic**, not Adwaita; GNOME Text Editor defaults to `'Adwaita'` (`org.gnome.TextEditor`
`style-scheme`, gnome-text-editor `data/org.gnome.TextEditor.gschema.xml:101-102` on `main`, read
2026-10-28, to pin). Plain `GtkTextView` gutters are GN-1.52.

| ID | § | Model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| GN-SF01† | 4.2 | code editor | Which scheme draws | Default scheme `classic` ("follow[s] more closely the GTK theme") unless the app sets one; Adwaita/Adwaita-dark pair through scheme metadata | P: `gsv/gtksourceview/gtksourcestylescheme.c:50-51`, `:76`, `:1659-1674`, `:1687`; `gsv/gtksourceview/gtksourcebuffer.c:510`; `gsv/data/styles/Adwaita.xml:24-27`; `Adwaita-dark.xml:24-27` | none | C05, C24 | P |
| GN-SF02† | 4.2 | code editor | Editor text and fill; selection | `text` style (fill and text colour); `selection` and `selection-unfocused` when a scheme states them (Adwaita states neither, so GTK's selection stands); the scheme is applied as generated CSS on `textview` | P: `gsv/data/styles/Adwaita.xml:104`; `Adwaita-dark.xml:107`; `gsv/docs/style-reference.md:117-127`; `gsv/gtksourceview/gtksourcestylescheme.c:1002-1025` | `input.background_color`, `defaults.text_color` (field values, not the scheme's); `defaults.selection_inactive_background` | C05, C17, C24 | P |
| GN-SF03† | 4.2 | code editor | Caret, secondary caret | `cursor` colour as `caret-color`; the bidirectional secondary caret `-gtk-secondary-caret-color`, the primary at alpha × 0.5 unless `secondary-cursor` is stated | P: `Adwaita.xml:98`; `Adwaita-dark.xml:101`; `gsv/docs/style-reference.md:129-138`; `gsv/gtksourceview/gtksourcestylescheme.c:797-852` | `input.caret_color` (no secondary caret) | C16, C24 | P |
| GN-SF04† | 4.2 | code editor | Current line | `current-line` background (only the background is used) | P: `Adwaita.xml:96`; `Adwaita-dark.xml:99`; `gsv/docs/style-reference.md:140-142` | none | C17, C24 | P |
| GN-SF05† | 4.2 | code editor | Gutter (line numbers) | `line-numbers` text and fill (used for all gutters), `current-line-number` fill, text and bold (bold defaults to true), `line-numbers-border` as a 1 px right border; the renderer's horizontal padding (`xpad`) on each side | P: `Adwaita.xml:97`, `:100`; `Adwaita-dark.xml:100`, `:103`; `gsv/docs/style-reference.md:144-160`; `gsv/gtksourceview/gtksourcestylescheme.c:1027-1050`; `gsv/gtksourceview/gtksourcegutterrendererlines.c:441-443`, `:482-486` | none (GN-1.52 is the plain text view's gutter fill) | C13, C18, C24 | P |
| GN-SF06† | 4.2 | code editor | Invisibles | `draw-spaces` colour; the marks are drawn as paths: space, tab arrow, newline, no-break space | P: `Adwaita.xml:99`; `Adwaita-dark.xml:102`; `gsv/docs/style-reference.md:178-181`; `gsv/gtksourceview/gtksourcespacedrawer.c:858-965` | none | C13, C24 | P |
| GN-SF07† | 4.2 | code editor | Bracket matching | `bracket-match` (Adwaita: bold only, no colour) and `bracket-mismatch` (not stated by Adwaita) | P: `Adwaita.xml:95`; `Adwaita-dark.xml:98`; `gsv/docs/style-reference.md:162-168` | none | C17, C18, C24 | P |
| GN-SF08† | 4.2 | code editor | Search match | `search-match` fill and text colour (Adwaita-dark's fill is translucent) | P: `Adwaita.xml:103`; `Adwaita-dark.xml:106`; `gsv/gtksourceview/gtksourcesearchcontext.c:412` | none (GNOME C17 row: "GTK's text widgets have no find highlight of their own") | C04, C17, C24 | P |
| GN-SF09† | 4.2 | code editor | Right margin | `right-margin`: the line in its foreground at alpha 40/255 and the area right of it in its background at 15/255, premixed with the editor fill; default column 80 | P: `Adwaita.xml:102`; `gsv/docs/style-reference.md:170-176`; `gsv/gtksourceview/gtksourceview.c:134-138`, `:5115-5170` | none | C04, C13, C24 | P |
| GN-SF10† | 4.2 | code editor | Syntax roles | 30 `def:` styles with foreground, background, bold, italic, underline (incl. `error` style with `underline-color`), strikethrough: comment, string, statement, type, function, identifier, constant, number/decimal/base-n/floating-point/boolean, preprocessor, special-char, note, shebang, error, warning, and the Markdown roles `heading`, `inline-code`, `list-marker`, `link-text`, `link-destination`, `emphasis`, `strong-emphasis`, `preformatted-section`; per-language overrides (C, C#, CSS, Diff, Go, Python, Rust, Vala, XML) | P: `Adwaita.xml:106-185`; `Adwaita-dark.xml:109-189`; `gsv/docs/style-reference.md:65-114` | none | C05, C18, C24 | P |
| GN-SF11† | 4.2 | code editor | Background pattern, overview map | `background-pattern` (grid) fill; `map-overlay` (the GtkSourceMap slider; falls back to `selection`) | P: `Adwaita.xml:94`, `:101`; `gsv/docs/style-reference.md:183-186`; `gsv/gtksourceview/gtksourcemap.c:520-521` | none | C13, C24 | P |
| GN-SF12† | 4.2 | code editor | Annotations (5.18+) | Warning, error and accent colours of inline annotations, taken from the scheme's `diff:changed-line`, `diff:removed-line`, `diff:added-line` | P: `gsv/gtksourceview/gtksourcestylescheme.c:70-72`, `:754-795`; `gsv/gtksourceview/gtksourceannotation.c:39`, `:392-400`; `Adwaita.xml:157-161`; `Adwaita-dark.xml:160-164` | `defaults.warning_color`, `danger_color`, `success_color` (status colours, not these) | C05, C24 | P |
| GN-SF13† | 4.2 | code editor | Completion and informative popups | GtkSourceView's own CSS: radius 11, layered shadows, margins, row min-height 28, icon cell 16 + padding 6, the "before" column at opacity 0.65, details pane; informative warning/error fills from `@warning_color` / `@error_color` | P: `gsv/gtksourceview/GtkSourceView.css:1-198` | `popover.*`, `list.*` | C01, C11, C13, C24 | P |

**Rich text / Markdown: not on this platform.** GTK 4.24.1 and libadwaita 1.10.0 have no Markdown
renderer (`grep -ril markdown` over `G/` and `A/c/` finds only GTK's debug output,
`gtk/gtkdebug.c:147`). Their rich text is Pango markup in labels, whose tags are fixed attributes
with no theme look: `<tt>` is "Monospace font", `<big>`/`<small>` relative sizes, `<b>` bold; no
headings, lists, quotes or code blocks (Pango markup reference, docs.gtk.org/Pango/pango_markup.html,
Pango 1.58.2, read 2026-10-28). Links in labels are the `link` node, `[link]`'s rows
(`A/w/_links.scss:34`). The Markdown roles GNOME colours are syntax roles of the code editor above.

### Drop zones and dialog placement (G19, G24)

#### G19 — drop zone under a dragged pane, tab or dock

Covered by: GN-1.127 (AdwTabBar insertion bar), GN-1.132 (drag-hover switch), GN-1.133 (the dragged tab),
GN-2.35 (tab overview drop target fill and ring), GN-3.51 (drag-hover activation), GN-3.66† (the catch-all
`:drop(active)` highlight and the widgets that replace it), GN-1.166 (separate boxed list drop target).
A tab dropped outside any tab bar becomes a new window through `AdwTabView::create-window`, with no drop zone
drawn ("This can happen after a tab has been dropped on desktop": `A/c/adw-tab-view.c:2795-2807`).

Not in libadwaita or GTK: a pane dragged onto another pane. The only libadwaita sources with a drop target or a
drop motion controller are the tab widgets, the sidebar and the inline view switcher (`adw-tab.c`,
`adw-tab-box.c`, `adw-tab-grid.c`, `adw-tab-thumbnail.c`, `adw-sidebar.c`, `adw-inline-view-switcher.c`, by
grep); the split views have none, and `G/gtkpaned.c` contains no "drop".

GNOME's docking library is **libpanel** (GNOME/libpanel, "IDE-like applications using GTK 4 and libadwaita";
GNOME Builder uses it). It draws a drop zone of its own, styled from libadwaita's colour variables. Whether it
is in scope is §6.2's question, as for kirigami-addons; the rows are listed so the rationale can decide.

| ID | section | model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| GN-SF14† | 4.2 | dock frame (libpanel) | Drop zone over a dock frame (`paneldropcontrols`) | While a page is dragged over a `PanelFrame`, the frame shows a drop overlay: accent fill at 10 %, radius 10, margin 1, `background` and `border` transitions 300 ms; when the drop is over it, accent 30 % and a 1 px inset accent ring. The frame's own `:drop(active)` shadow is switched off, so GN-3.66†'s catch-all does not apply. | P: `LP/stylesheet.css:405-420`; `LP/panel-frame.c:613`, `:656`, `:670` | none | C24, C17, C07, C04, C14 | P |
| GN-SF15† | 4.2 | dock frame (libpanel) | Drop-position buttons (top, right, bottom, left, centre) | Five buttons in a grid centred on the frame, 6 apart; icons `panel-top-symbolic`, `panel-right-symbolic`, `panel-bottom-symbolic`, `panel-left-symbolic`, `panel-center-symbolic`; the centre one circular, the others radius 11, padding 0; fill accent, glyph accent-foreground at 80 %, shadow `0 3px 5px` black 20 %; under the drop: fill mix(accent, accent-foreground 30 %), glyph accent-foreground. | P: `LP/panel-drop-controls.ui:6-61`; `LP/stylesheet.css:421-434` | none | C24, C13, C01, C07, C19 | P |
| GN-SF16† | 4.2 | dock frame (libpanel) | Frame-switcher drop indicator | The indicator drawn in a frame switcher where a dragged page would land: accent at 80 %. | P: `LP/stylesheet.css:88-90`; `LP/panel-frame-switcher.c:102-139`, `:782` | none | C24, C17 | P |

#### G24 — where a modal dialog is placed

Covered by: GN-1.218 (floating-sheet margins 30 → 120 / 20 → 120 over 720 → 1440), GN-1.219 (default size),
GN-1.222 (bottom-sheet look: side margins, flush outlines — but not *when*), GN-2.17 (the bottom sheet's look and
its top padding 30 → 120), GN-3.72† (the parent dimmed under an attached dialog; its "key's default: to verify"
is settled in the last row below).

| ID | section | model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| GN-SF17† | 4.1 | [dialog] | AdwDialog / AdwAlertDialog: floating or bottom sheet (the breakpoint) | With `presentation-mode` `AUTO` (the default) a dialog is a centred floating sheet, and a bottom sheet when the window is at most 450 wide (`max-width: 450px`) or at most 360 high (`max-height: 360px`); it also gets the classes `.narrow` / `.short` per breakpoint, which change the alert's paddings (GN-1.215). The app can force `FLOATING` or `BOTTOM_SHEET`. | P: `A/c/adw-dialog.c:24-25`, `:28-35`, `:451-478`, `:2131-2133` | none | C26, C10 | P |
| GN-SF18† | 4.1 | [dialog] | AdwDialog floating sheet: position | Centred both ways (`(width − sheet) × 0.5`, `(height − sheet) × 0.5`) in the dialog host, which is the whole `AdwWindow` (its title bar is a hidden gizmo; the header bar is part of the content), inside GN-1.218's margins. | P: `A/c/adw-floating-sheet.c:181-226` (centring `:217-218`); `A/c/adw-window.c:325-331` | none | C26 | P |
| GN-SF19† | 4.1 | [dialog] | AdwDialog bottom sheet: position | Attached to the bottom edge; not full width (`full-width` false), horizontally centred (`align` 0.5, mirrored in RTL), natural width at least 360; height capped by GN-2.17's top padding; no drag handle. | P: `A/c/adw-dialog.c:511-522`; `A/c/adw-bottom-sheet.c:27-30`, `:588-599`, `:617-630`, `:1125` | none | C26 | P |
| GN-SF20† | 4.1 | [dialog] | AdwDialog shown as its own window | When the parent window is not an `AdwWindow` / `AdwApplicationWindow`, or is not resizable, the dialog becomes a separate non-resizable window (`.dialog-window`), placed by the compositor (next row). | P: `A/c/adw-dialog.c:2073-2104`, `:613-631` | none | C26 | P |
| GN-SF21† | 4.1 | [dialog] | Dialog windows: mutter's placement | Under GNOME Shell `attach-modal-dialogs` is true (gnome-shell's override; mutter's own default is false): an attached modal dialog is centred both ways on its parent's frame and kept on screen. A transient dialog that is not attached is centred horizontally on its parent with "twice as much space below as on top" (top at a third of the free height). Applies to GTK's own dialog windows (GtkAlertDialog, the file chooser's window when not the portal's), legacy GtkDialog, and AdwDialog's window fallback. | R: `org.gnome.mutter attach-modal-dialogs` (`M/data/org.gnome.mutter.gschema.xml.in:26-33`; `GS-data/00_org.gnome.shell.gschema.override:1-2`); P: `M/src/core/constraints.c:1249-1294`; `M/src/core/place.c:977-1008` | none | C26, C25 | R, P |

---

## Relabelled controls (2026-10-28)

Pass 2 asked the platforms about 18 controls under a wrong control name (a tagging
error: GN-2.40–2.46, GN-2.49, K2.7, K2.15–2.17, K2.22, Q2.43, W2-13, W2-14, W2-19,
W2-25), so this check asked again for this platform. Decisions: 1 a new §4.2 row, 2 a
new §4.1 row, 3 a row already covers it (the same job, possibly done another way),
4 the platform has no such thing.

Sources as in the GNOME file (`A/w/`, `A/c/`, `G/`, `G/deprecated/`, GTKgal, ADWgal at libadwaita 1.10.0 / GTK 4.24.1); `G/ui/F:L` is GTK 4.24.1 `gtk/ui/F` (the `G/F:L` rule). `A/c/adw-overlay-split-view.c` and `G/deprecated/gtkassistant.c` were fetched from gitlab.gnome.org at the tags on 2026-10-28.

| Control | Decision 1–4 | Row added, row covering it, or the source showing it is absent |
|---|---|---|
| command link button (K2.7) | 3 | GTKgal and ADWgal have no command-link button. The same thing (a clickable title with a description under it) is an activatable AdwActionRow in a boxed list: GN-2.01 (title, dimmed `smaller` subtitle, prefix/suffix), GN-1.165 (boxed list, row hover / pressed), GN-1.158–1.160 (activatable row pressed, selected + hover / pressed, open popup; hover is `list.hover_background`), `A/w/_lists.scss:75-89`. A trailing arrow is an image the app adds, not part of the row |
| title widget (K2.15) | 3 | No KTitleWidget counterpart in GTKgal or ADWgal; page and section titles are AdwStatusPage (GN-2.13: icon, `.title-1`, description), AdwPreferencesGroup's header (GN-2.10; its title is `.heading`, its description `.body` `.dimmed`, `A/c/adw-preferences-group.ui:20-41`, the roles in GN-1.189), AdwWindowTitle (GN-1.18) and the `.title-1`…`.title-4` classes (GN-1.189). GNOME colours no comment line by message type |
| side tab bar (K2.16) | 3 | GtkNotebook with tabs on the left or right: GN-1.118 (indicator on the left / right side, `A/w/_notebook.scss:36-58`), GN-1.119 (up / down scroll arrows), GN-1.121 (strip padding, including the `.left` / `.right` rules of `A/w/_notebook.scss:167-210`), GN-1.116, GN-1.117, GN-1.120, GN-1.122, GN-1.123; a vertical page switcher is GtkStackSidebar (GN-2.34). No edge-docked tool-view tab bar: no dock widgets in GTKgal or ADWgal (table A, "dock/panel title bar") |
| splitter collapser (K2.17) | 4 | GtkPaned's nodes are its two children and the `separator` (`G/gtkpaned.c:92-99`), no button; AdwOverlaySplitView's nodes are the sidebar and content panes only (`A/c/adw-overlay-split-view.c:125-141`). A pane is shown or hidden by a button the app places in a header bar, which is a plain flat button (GN-1.29) |
| wizard (K2.22) | 1 | GtkAssistant (deprecated 4.10, still shipped: GTKgal "Assistant"). GN-2.51 covers its sidebar only; new 4.2 row: GtkAssistant page frame and navigation buttons. Current GNOME apps build multi-step flows from AdwNavigationView pages (GN-2.18) |
| drag handle (Q2.43) | 3 | The bottom sheet's drag handle: GN-2.17 (54×6, 25 %, margin 15, radius 99; hidden by `show-drag-handle`). No row-reorder handle widget in GTKgal or ADWgal; reordering draws the drag icon (GN-2.49) and the separate boxed list's drop-target shadow (GN-1.166) |
| grid view items (W2-13) | 3 | GtkGridView / GtkFlowBox items: GN-1.174 (padding, radius 9, hover / pressed, selected 25 / 32 / 39 %, focus ring); in sidebars GN-1.143; GtkIconView GN-1.280†; selection-mode checks GN-1.62; rubber band GN-1.175; drag icon GN-2.49. GNOME draws no selected-item border and no corner check box |
| items view (W2-14) | 3 | GtkListView / GtkGridView items: GN-1.158–1.166 (row states, selected + hover / pressed, focus, separators), GN-1.174 (grid items); disabled items use the global disabled opacity (GN-1.123's mechanism, `--disabled-opacity`). No inner selection border in GNOME |
| repeat button (W2-19) | 3 | GTKgal has no repeat-button class; GTK repeats only inside GtkSpinButton (buttons GN-2.24) and the slider / scrollbar trough: timings GN-3.69† (`G/gtkspinbutton.c:63-67`, `G/gtkrange.c:73-74`) |
| radio group (W2-25) | 3 | GTKgal has no radio-group widget, only GtkCheckButton in a group, which becomes a `radio` node with the `.grouped` class (`G/gtkcheckbutton.c:207-212`): GN-1.60, GN-1.61, GN-1.63. A headed radio set is an AdwPreferencesGroup (header GN-2.10) of action rows with a radio prefix (GN-2.01); the spacing of free-standing radios is the app's GtkBox spacing |

Tally: 1 — 1 (wizard); 2 — 0; 3 — 8; 4 — 1 (splitter collapser).

| ID | section | model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| GN-RL01† | 4.2 | wizard | GtkAssistant page frame and navigation buttons (deprecated, still shipped) | Two layouts. Default (`use-header-bar` unset → FALSE): an action area under the page, a box with spacing 6 and margins 6, buttons in the order Close, Cancel, Last, Back, Forward, Apply (those the page type needs), all `.text-button` and sized equal by one size group. With `use-header-bar`: Back and Cancel packed at the header bar's start, the others at its end, vertically centred, the default button made `.suggested-action`; the header bar shows no window buttons. The current page's title is the window title. libadwaita styles only the sidebar (GN-2.51), so the rest is the plain button and header-bar look | published `G/ui/gtkassistant.ui:3-8`, `:30-108`, `:114-122`; `G/deprecated/gtkassistant.c:420-430`, `:440-463`, `:470-481`, `:981`; `A/w/_deprecated.scss:46-94` (only `window.assistant .sidebar`) | `dialog.button_gap_px`, `dialog.button_order` (dialogs only) | C24, C09, C26 | P |

## Fonts and mirroring (2026-10-28)

A completeness check of the map found two gaps no row covered: the emoji font and
the fallback chain for scripts the UI font lacks (all six platforms), and right-to-left
layout where this file had no general row. The source abbreviations are defined in
`todo_v0.7.1_model-completeness-gap-map-seeds.md`, *Fonts and mirroring sources*.

Not covered by any row (GN-1.193 is features and variations, GN-3.56 rendering). GNOME has no settings
key for an emoji or fallback font: `org.gnome.desktop.interface` has `font-name`, `document-font-name` and
`monospace-font-name` (`S/org.gnome.desktop.interface:107`, `:158`, `:165`) and nothing else for fonts.

| ID | section | model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| GN-FM01† | 4.3 | typography | Emoji font | Pango's itemizer marks emoji runs and asks for the family `emoji`, with a colour font required when the run has a variation selector, and the family `emoji` with colour forbidden for a non-emoji character with a variation selector; the colour wish goes into the fontconfig pattern (`FC_COLOR`). fontconfig resolves `emoji` as on KDE (Noto Color Emoji first). The model has no emoji family. | P: `P/itemize.c:906-937`; `P/pangofc-fontmap.c:2115-2126`; `fc/60-generic.conf:9-47`, `fc/45-generic.conf:8-77`; R: `fc-match emoji` (as on KDE); M: GTK probe, the sample set in a `PangoLayout`, `pango_font_describe` of each run | none (`defaults.font.family`) | C18, C25 | P, R, M |
| GN-FM02† | 4.3 | typography | Fallback chain per language, and the family list | Pango sorts fontconfig's candidates with the run's language (`FC_LANG`): the context language where it covers the script, else the script's sample language; Han has no sample language, so `PANGO_LANGUAGE` / `LANGUAGE` or the context language decides between Japanese, Chinese and Korean glyphs. Separately, the `font-name` string may itself carry a family list (`"Cantarell,Sans"`), which Pango hands to fontconfig to try in turn; the reader keeps only the first name. | P: `P/itemize.c:834-855`; `P/pangofc-fontmap.c:2142`; `P/pango-language.c:870-905` (the Han rule and the environment variables); `native-theme/src/gnome/mod.rs:194-200` (the reader drops the list); R: the `font-name` key (`S/org.gnome.desktop.interface:107`); `fc-match -s` per language (as on KDE); M: GTK probe, the sample set under each `LANG` | none (`defaults.font.family`) | C18, C25 | P, R, M |
| GN-FM03† | 4.3 | mirroring | Right-to-left layout (outside the brief: no such row exists) | GTK sets the default text direction from the locale at start-up and every widget has a direction; libadwaita's stylesheet has 81 `:dir()` rules in 18 files (header bar, buttons, checks, menus, lists, expanders, paned, tab view, sidebars, spin button, scrolling, emoji chooser, …), of which GN-1.139, GN-1.284† and GN-SF19† record three. Icon lookup appends `-rtl` / `-ltr` to the icon name by direction, so an icon theme can ship a mirrored variant; the freedesktop loader (`native-theme/src/freedesktop.rs`) has no direction. Cross-check: MAC-X34†, W3-54†, X-SV-3. | R: `gtk_get_locale_direction()` (`G/gtkmain.c:491`, `:704-730`), `gtk_widget_get_direction()` (`G/gtkwidget.c:7647`); P: `A/w/*.scss` `:dir(rtl)` / `:dir(ltr)` rules (count from a search of `src/stylesheet` at 1.10.0); `G/gtkicontheme.c:2173-2202` (direction suffix); M: GTK probe under an RTL locale | none | C26, C19, C25 | R, P, M |

## Pass 3 (2026-10-28)

The 261 rows the other five platforms added after pass 1 (IDs ending in "†"), asked of GNOME. Decisions: **1** GNOME draws it and no row covered it (a row added below), **2** a row of this file (or a model field the reader fills) covers it, **3** GNOME does not draw it (the source shows the absence). An input row whose parts fall under different decisions is split. Sources are those of the top of this file, of *Cross-check (pass 2)* (`G/deprecated/`, `GS/`, GTKgal, ADWgal) and of the seeds file (`M/` mutter 51.0, `GS-data/` gnome-shell 51.0 `data/`, `gsv/` GtkSourceView 5.22.0, `P/` pango 1.58.2), plus:

| Short | Source |
|---|---|
| `G/theme/Default/F:L`, `G/svg/F:L` | GTK 4.24.1 `gtk/theme/Default/F` and `gtk/svg/F` (the `G/` rule; read from the tag's tree) |
| `G3/F:L` | GTK tag **3.24.52** (the newest 3.24 tag on gitlab.gnome.org, read 2026-10-28), `gtk/F` |
| `A/NEWS:L` | libadwaita 1.10.0, `NEWS` at the repository root |
| `LS/F:L` | GNOME/libspelling tag **0.4.10** (gitlab.gnome.org, read 2026-10-28), path from the repository root |
| GN47 | GNOME 47 release notes, `release.gnome.org/47/`, section "New Open and Save File Dialogs", read 2026-10-28 |
| `S/org.gnome.desktop.a11y.magnifier`, `S/org.gnome.desktop.a11y.mouse` | the `S/` rule (gsettings-desktop-schemas 51.0), fetched at the tag 2026-10-28 |

### Decisions

| Input rows | Decision 1–3 | Row added, row covering it, or the source showing it is absent |
|---|---|---|
| K1.19†, MAC-W13† (drag tiling preview) | 1 | GN-P3-1† (GNOME Shell's tile preview); found beside it, GN-P3-2† (the size label mutter shows while an X11 window with size increments is resized) |
| MAC-W13† (zoom-button tiling menu, tiled-window margins) | 1 | The window's own menu is GN-P3-3†; it has no tiling entry (`GS/windowMenu.js:27-189`). Gaps between tiled windows: no source states any; to verify by measure |
| IOS W-10† | 3 | No Slide Over and no split-view divider: two tiled windows are separate windows with the tiled look of GN-1.05; gnome-shell 51.0's `js/ui/` has no such module (file list) |
| IOS W-11†, K3.39†, MAC-W12† (full screen, title-bar reveal), MAC-X32† (full-screen reveal) | 1 | GN-P3-5† (full screen: the title bar hidden, the toolbar view's bars revealed by the app, the size-change motion); tiled / maximized / minimized looks and motion GN-1.05, GN-3.71† |
| MAC-W12† (Split View divider) | 3 | As IOS W-10† |
| MAC-X32† (tiling, momentum) | 2 | Tiling motion GN-3.71†; momentum scrolling GN-3.24 |
| MAC-W14† | 2 | `.flat` and inline header bars GN-1.17; content under the bars (`extend-content-to-top-edge`) GN-P3-5†; the title widget is the app's (GN-1.18) |
| MAC-W15† | 2 | GN-1.16 (the bar's contents fade to opacity 0.5) |
| MAC-W11† | 3 | AdwWindowTitle has two properties, `title` and `subtitle` (`A/c/adw-window-title.c:124-137`): no title menu, rename popover or edited mark |
| MAT-W4† | 2 | GN-1.01–1.03 (shadow, outline), GN-1.11–1.22 (title bar, controls), GN-1.16 (unfocused) |
| MAT-W5† | 2 | GN-1.05, GN-1.06 (radius per state and per corner); no rounded display corners: no screen-corner actor in `GS/` (grep) |
| W1-148† | 1 | GN-P3-4† (which window controls a window shows, by its kind); no help button: `button-layout` names only `minimize`, `maximize`, `close` and `icon` (`G/gtkwindowcontrols.c:751-752`) |
| W1-149† | 1 | GN-P3-7† (a window asking for attention becomes a notification) |
| K3.15† | 2 | GN-3.72† |
| K3.16† | 1 | GN-P3-6† ("is not responding": dialog and darkened window) |
| K3.66† | 1 | GN-P3-8† (the launched icon zooms out); no busy cursor: mutter has none for start-up (`M/src/core/`, `M/src/backends/` cursor code: no "busy", grep) |
| W-SF12† | 1 | GN-P3-9† (the on-screen keyboard slides the focused window up) |
| IOS B-18†, MAC-N29† | 2 | GN-1.26 (`.has-open-popup`), GN-1.28, GN-1.40 (arrow glyphs); the popover menu GN-1.66–1.80 |
| MAC-N28† | 2 | GN-2.29 (AdwSplitButton) |
| MAC-B11† | 2 | GN-1.27 (minimum widths: 16 base, 24 image-only, `A/w/_buttons.scss:20-21`, `:67`); stack-switcher text buttons 100 (GN-2.33) |
| K-RL07† | 2 | GN-3.69† |
| W-RL02† | 2 | As K2.7 in *Relabelled controls*: GN-2.01, GN-1.165, GN-1.158–1.160 |
| IOS I-9†, K1.48†, MAT-I19† | 2 | GN-2.58† (mask glyph), GN-2.23 (peek icon, caps-lock; the last-character hint `gtk-entry-password-hint-timeout` defaults to 0, and GNOME has no key for it: not in `GDK/wayland/gdksettings-wayland.c:270-312`). No strong-password look: GTK has no password generator (GTKgal) |
| IOS I-10† | 2 | Disabled: the whole entry at `--disabled-opacity` (`A/w/_entries.scss:40`), the model's `input.disabled_opacity`; editing = focused, GN-1.50; height `min-height: 34px` (`:4`), `input.min_height_px`, growing with the text scale (GN-3.63) |
| K1.46† | 2 | GN-2.22, GN-1.44; no fade: the clear icon is shown and hidden with `gtk_widget_set_child_visible` (`G/gtksearchentry.c:783-791`) |
| K1.47† | 2 | GN-1.49 (overwrite block caret); no input mask in `G/gtkentry.c` or `G/gtktext.c` (grep) |
| K1.49†, W1-151†, MAT-I18† (IME composition) | 2 | GN-1.281† |
| K1.49†, W1-151†, MAT-I18† (spell check) | 1 | GN-P3-27† (libspelling's underline; GTK itself has no spell checker) |
| W1-150† | 3 | No helper text under a field: GtkEntry has none, AdwEntryRow's properties have none (`A/c/adw-entry-row.c`: `show-apply-button`, `input-hints`, `input-purpose`, `attributes`, `enable-emoji-completion`, `activates-default`, `text-length`, `max-length`); the nearest is AdwPreferencesGroup's `description` under the group heading (GN-2.10) |
| MAC-I09† | 3 | No autofill: GTKgal has none; an entry passes only `input-purpose` / `input-hints` |
| MAC-I10† | 2 | `text_area.border.padding_*`: GtkTextView's margins default to 0 (`G/gtktextview.c:1053-1085`) and libadwaita adds no padding (`A/w/_views.scss:20-45`) |
| MAT-I17† | 1 | GN-P3-26† (the text magnifier's size and zoom; its look GN-1.184, GN-2.42) |
| IOS SF-1†, MAC-SF01† | 3 | GtkTextView draws no invisibles (`G/gtktextview.c` has no space-drawing property, grep); GtkSourceView's are GN-SF06† |
| IOS SF-5†, MAC-SF05† | 3 | Rich text in a label is Pango markup with no theme look (*Seed follow-up*, "Rich text / Markdown") |
| IOS CK-2† | 2 | GN-1.62 (round selection-mode checks) |
| K1.58† | 2 | `checkbox.check_mark_stroke_width_px` 2 and `radio_dot_diameter_px` 8 carry the check's stroke width and the radio dot's diameter only (`docs/platform-facts.md@fc6109d1:1241-1242`), not the check mark's shape: GN-P4-7†; the 14-px glyph and its padding 3 (`A/w/_checks.scss:23-26`); indeterminate GN-1.58 (review 2026-10-28: "already carry the glyphs" corrected; the shape is GN-P4-7†) |
| MAC-RL02† | 2 | As W2-25 in *Relabelled controls*: GN-1.60, GN-1.61, GN-1.63, GN-2.01, GN-2.10 |
| IOS MN-17† | 2 | GN-1.68 (item radius 9), GN-1.69 (menu padding 6), GN-1.285† (no icon column) |
| IOS MN-18† | 2 | `menu.disabled_text_color`: a disabled label is drawn at `--disabled-opacity` (`A/w/_labels.scss:8-10`); a menu item has no subtitle (its attributes are label, use-markup, icon, verb-icon, submenu-action, hidden-when, custom: `G/gtkpopovermenu.c:99-130`) |
| MAC-MN17† | 3 | No destructive menu item: the menu model's attributes have no such role (`G/gtkpopovermenu.c:99-130`) |
| IOS TT-3†, K1.83† | 2 | GN-1.82 (browse timings), GN-1.83 (placement), GN-1.283† (wrapping); first delay `tooltip.show_delay_ms` (`HOVER_TIMEOUT` 500, `G/gtktooltip.c:71`); no display-time limit: the only timeouts are hover, browse and browse-disable (`:71-73`) |
| K1.84† | 2 | Any content: `gtk_tooltip_set_custom` (`G/gtktooltip.c:290`); GN-1.81, GN-1.83 |
| MAC-T06† | 3 | No expansion tooltip: GtkLabel's `query_tooltip` shows only a link's title (`G/gtklabel.c:1804-1847`) |
| K1.260† | 2 | Hover colour `link.hover_text_color` (the link colour with saturation and lightness × 1.1, `A/w/_links.scss:11-13`); always underlined (`:4`), its geometry GN-1.267; the `pointer` cursor GN-3.67† |
| IOS SB-7† | 2 | `scrollbar.min_thumb_length_px` (popover-menu value GN-1.92), overshoot GN-3.06 and GN-3.24, hover widening GN-1.85, GN-1.87 |
| MAT-SC3† | 2 | GN-1.91 (fade timings), GN-1.85 (trough margins); a maximum thumb fraction: none stated in `A/w/_scrolling.scss`, GtkRange sizing to verify |
| MAT-SC4† | 2 | GN-3.06 (overshoot) |
| W1-152† | 2 | No gap: the reader reads `overlay-scrolling` into `scrollbar.overlay_mode` (`native-theme/src/gnome/mod.rs:28`, `:554-557`, `:637`) |
| W1-153† | 3 | GtkScale has one slider (GTKgal "Scale"; its second trough is the fill level, GN-1.103); no range slider in ADWgal |
| IOS PB-4† | 2 | GN-1.110 (`show-text`) |
| K1.115† | 2 | GN-2.26 (GtkLevelBar, GNOME's capacity bar) |
| MAC-P05† | 2 | No edge at rest; the HC inset border GN-1.115 |
| IOS T-17† | 3 | No user-customisable tab bar in ADWgal; reordering and its drop indicator are GN-1.127, GN-1.131 |
| K1.133† | 2 | GN-1.131 (reorder 250 ms), GN-1.132 (drag-hover switch 500 ms) |
| MAC-TB06† | 2 | GN-1.116, GN-1.124, GN-1.125, GN-1.126 |
| MAC-TB07† | 2 | GN-1.129 (widths, label fade), GN-1.119 (notebook scroll arrows) |
| W-SF09† | 2 | GN-1.127, GN-1.133, *Seed follow-up* G19 |
| MAC-SF07† | 3 | No window tab bar: mutter groups no windows into tabs and GTKgal has no window-tabbing widget; in-app tab bars as drop targets are GN-1.127, GN-3.66† |
| IOS SD-9† | 2 | GN-2.19 (overlay pane, transition shadow, dimming, spring) |
| IOS SD-10†, MAC-SB09† | 2 | GN-1.147 (width), GN-1.197 (breakpoints) |
| K1.142† | 1 | GN-P3-22† (the places sidebar's rows: eject button, section separators, drop rows) |
| MAC-TL10† | 2 | GN-1.188 (`.spacer`); flexible space is an expanding child, no item |
| MAC-TL11† | 2 | GN-1.29 (checked flat button) |
| MAT-TA8† | 3 | Header bars do not overflow (pass 2, "toolbar flyout"); the primary menu button GN-1.40 |
| W1-158† | 3 | Nothing to query: GtkStatusbar is deprecated and states its geometry in CSS (GN-1.157) |
| IOS L-21† | 2 | GN-1.173 (indent: measure) |
| IOS L-22† | 2 | GN-1.167, GN-1.168 |
| IOS L-23† | 2 | GN-2.01–2.08 |
| K1.184† | 3 | No column browser (pass 2) |
| K1.185† | 3 | No expand animation: the tree expander swaps its icon (`A/w/_expanders.scss:1-10`) and rows appear at once; `G/gtktreeexpander.c` and `G/gtktreelistmodel.c` have no animation, `G/gtklistbase.c` only an autoscroll tick (`:1444`) |
| MAC-L17† (controls in a selected row) | 3 | A selected row changes only its fill (`A/w/_lists.scss:68-70`, `:93-95`), so its children draw as anywhere else |
| MAC-L17† (disabled selection) | 2 | GN-1.176 (`.view:disabled`) |
| MAC-L18† | 2 | GN-1.167, GN-1.168; no highlighted sort column and no corner view (`A/w/_column-view.scss`: only `sort-indicator`, `:50`) |
| MAC-L19† | 2 | GN-1.160 (`.has-open-popup`) |
| W1-159†, MAC-N31†, IOS C-47† | 2 | GN-1.175, GN-1.280† |
| K-RL05†, W-RL01† | 2 | GN-2.45 (GtkEditableLabel), GN-1.172 (editable cell), GN-1.279† |
| K-RL06† | 2 | GN-2.49 (drag icon) |
| IOS PO-7† | 2 | GN-1.185 (size), GN-1.181 (gap) |
| IOS DL-13† | 2 | GN-1.216, GN-1.217 (widths), GN-1.223 (scrolling body), GN-3.63 (`sp` widths) |
| IOS SF-7†, SF-8†, SF-9†, K-SF19†, K-SF20†, K-SF21†, K-SF22†, MAC-SF08†, MAC-SF09†, MAC-SF10†, MAT-SF01†, W-SF10†, W-SF11†, W-SF13† | 2 | GN-SF17†–GN-SF21† |
| MAT-SF02†, Q2.58† | 2 | GN-SF17† (floating or bottom sheet), GN-SF20† (own window); no full-window dialog (pass 2) |
| W1-155† | 3 | AdwDialog has auto, floating and bottom-sheet presentations only (pass 2, "full-screen dialog") |
| W1-156†, K1.224†, MAC-IC04† (icon, footer) | 3 | AdwAlertDialog's properties are `heading`, `body`, `extra-child`, `prefer-wide-layout` and the responses (`A/c/adw-alert-dialog.c`): no icon (pass 2, cell 1139), no footer |
| W1-156† (instruction text) | 2 | GN-1.214 (heading and body) |
| MAT-DG10† | 2 | GN-1.284† |
| IOS SG-8† | 2 | Momentary segments are `.linked` buttons, GN-1.42 |
| IOS SG-9† | 1 | GN-P3-25† (toggles sized to their content unless `homogeneous`); disabled GN-1.242; minimum widths GN-1.244 |
| MAC-SG10† | 2 | GN-1.242 (disabled image 30 %), GN-1.277 (checked fg), GN-3.59 (16 px) |
| W1-154† | 2 | GN-1.245–1.247 (round, flat, OSD), GN-1.244 (item kinds), GN-2.16 |
| MAC-SN05† | 3 | No fade on start or stop: AdwSpinner's only opacity is the track's 0.15 (`A/c/adw-spinner-paintable.c:22`) and `A/c/adw-spinner.c` animates none (grep) |
| MAC-EX05† | 2 | GN-1.261 (arrow 16, title radius 6), GN-2.01, GN-2.09 (row header 50); `expander.*` fields |
| W1-157† | 2 | GN-1.186; no gripper and no per-state fill: `A/w/_paned.scss` (36 lines) styles only `> separator` |
| MAC-DF05† | 2 | GN-1.268–1.277 |
| W3-45†, MAC-X30† | 2 | GN-3.08 (fills as currentColor percentages), GN-1.29 |
| W3-46† | 2 | GN-1.269 (status bg / fg / standalone triples) |
| MAC-TS03† | 3 | No tight or loose font variants in Pango or libadwaita; line heights per class GN-1.190 |
| MAT-X36†, W3-47† | 2 | No gap: the reader reads `font-name` and `monospace-font-name` (`native-theme/src/gnome/mod.rs:26`, `:537`) into `defaults.font` and `defaults.mono_font` |
| IOS FM-1†, K-FM01†, MAC-FM01†, MAT-FM01†, W-FM01† | 2 | GN-FM01† |
| IOS FM-2†, K-FM02†, MAC-FM02†, MAT-FM02†, W-FM02† | 2 | GN-FM02† |
| K-FM03†, MAT-FM03†, MAT-FM04†, MAC-X34†, W3-54† | 2 | GN-FM03† (layout and the `-rtl` icon suffix) |
| MAC-LY02† | 2 | GN-2.01 (title at the start, control at the end of a row) |
| IOS RL-3†, MAC-RL01†, MAT-RL01† | 1 | GN-P3-24† (the spacing defaults GN-2.46 left to verify, and GtkFlowBox's) |
| MAT-X37† | 3 | No optical-centring rule in libadwaita's stylesheet; its asymmetric paddings are stated per widget (GN-1.39) |
| MAT-X38† | 2 | Toast placement GN-2.12; no floating action button (pass 2, "fab") |
| MAT-SF04† | 2 | GN-2.19 (the overlay split view's pane over the content) |
| MAC-X33† | 3 | mutter handles no display cutout (no "notch" or "cutout" in `M/src`, grep) |
| MAC-IC05† | 1 | GN-P3-19† (GtkSvg: animated, stateful icons) |
| W3-51† | 2 | GN-3.60; disabled icons fade with their widget (pass 2, C19) |
| IOS C-36†, K2.31†, MAC-N36† | 3 | No teaching tip and no "What's This" (pass 2) |
| IOS C-37† | 2 | GN-2.30 (GtkPopoverMenuBar) |
| MAC-N33† | 3 | No menu search: GTK has no help menu and the menu model has no search item (`G/gtkpopovermenu.c:99-130`) |
| MAC-N34† | 3 | No status items: GNOME Shell 51.0 has no StatusNotifier host (no `StatusNotifier` or `AppIndicator` in `GS/` or gnome-shell `src/`, grep) |
| IOS C-38†, K2.29† | 2 | GN-2.56† (GtkEntryCompletion), GN-2.06 (combo row search) |
| IOS C-39†, K2.26† | 1 | GN-P3-21† (since GNOME 47 the file dialogs are Files; xdg-desktop-portal-gnome 51.0 still delegates to it) (review 2026-10-28: "GNOME 47+" narrowed to what was read), GN-P3-22†, GN-P3-23† (GTK's own chooser); path bar GN-2.52†. GTK 4's chooser has no preview pane and no icon-size slider (`G/ui/gtkfilechooserwidget.ui`: no preview, grep) |
| IOS C-40† | 3 | No ink toolbar (pass 2) |
| IOS C-41† | 3 | No inspector panel (pass 2); an overlay split view's pane is GN-2.19 |
| IOS C-42†, MAC-N30† | 2 | GN-2.54†, GN-2.44 |
| IOS C-43† | 2 | GN-1.11–1.18 (header bar), GN-2.15 (view switcher title) |
| IOS C-44† | 3 | No document launch view in GTKgal or ADWgal; an app's empty state is AdwStatusPage (GN-2.13) |
| IOS C-45†, MAC-N37† | 3 | No chart widget in GTKgal or ADWgal |
| IOS C-46† | 3 | No activity rings in GTKgal or ADWgal |
| IOS C-48† | 2 | GN-2.38, GN-2.55†; page spacing defaults to 0 (`A/c/adw-carousel.c:1062-1065`); no page curl |
| IOS C-49† | 3 | No editing HUD in GTKgal or ADWgal; the nearest is the touch selection bubble GN-1.286† |
| IOS SF-2†, SF-3†, SF-4†, SF-6†, MAC-SF02†, MAC-SF03†, MAC-SF04†, MAC-SF06†, K-SF12†, K-SF13†, K-SF14†, K-SF15†, K-SF16†, K-SF17†, W-SF01†, W-SF02†, W-SF03†, W-SF04†, W-SF05†, W-SF06†, W-SF07†, W-SF08† | 3 | No rich text view (*Seed follow-up*, "Rich text / Markdown: not on this platform"); links in labels are `[link]` |
| IOS RL-1†, K-RL02† (shortcut display) | 2 | GN-2.41 (AdwShortcutsDialog), GN-2.40 (keycaps) |
| K-RL02† (shortcut editor) | 3 | No shortcut editor in GTKgal or ADWgal: AdwShortcutsDialog only displays |
| IOS RL-2† | 2 | GN-2.45 (GtkEditableLabel) |
| K2.23†, K2.24† | 3 | No date or time picker (pass 2) |
| K2.25†, W2-42† | 2 | GN-2.27, GN-2.53† |
| K2.27† | 2 | GN-2.52† |
| K2.28† | 1 | GN-P3-20† (GtkFontChooserWidget); its button GN-2.28 |
| K2.30† | 2 | GN-2.21, GN-2.22; no-match colour through `.error` (GN-3.12); match highlights GN-SF08† |
| K2.32† | 3 | No ruler (pass 2) |
| Q2.53†, MAT-N30† | 2 | GN-1.286†, GN-2.42 |
| Q2.54† | 3 | No pull-to-refresh (pass 2) |
| Q2.55†, Q2.56†, W2-40† | 2 | GN-2.01–2.10, GN-1.165, GN-1.166 |
| Q2.57†, MAC-N32† | 2 | GN-2.39 |
| K-SF01†, K-SF02†, K-SF03†, K-SF04†, K-SF07†, K-SF08†, K-SF09†, K-SF10† | 2 | GN-SF01†, GN-SF02†, GN-SF04†, GN-SF05†, GN-SF07†, GN-SF08†, GN-SF09†, GN-SF10† |
| K-SF05† | 3 | GtkSourceView 5.22 has no change-marker gutter (no such renderer in `gsv/gtksourceview/`, grep) |
| K-SF06† | 2 | Invisibles GN-SF06†; no indentation guides in `gsv/gtksourceview/` (grep) |
| K-SF11† (spell check) | 1 | GN-P3-27† |
| K-SF11† (template placeholders) | 3 | The focused snippet placeholder takes the scheme's `snippet-focus` style (`gsv/gtksourceview/gtksourcebuffer.c:681-697`); Adwaita and Adwaita-dark state none (other bundled schemes do), so under GNOME's schemes it looks like plain text |
| K-SF18† | 2 | GN-SF14†–GN-SF16†, GN-3.66† |
| K-SF23†, MAT-SF03† | 2 | GN-SF19†, GN-2.17 |
| K-RL01† | 3 | No command bar or palette in GTKgal or ADWgal |
| K-RL03† | 2 | GN-2.43 |
| K-RL04† | 3 | No character selector in GTKgal or ADWgal |
| MAC-N27† (IME candidates) | 1 | GN-P3-10† (GNOME Shell's IBus candidate popup) |
| MAC-N27† (accent popup, character viewer, Look Up) | 3 | No press-and-hold accent popup for hardware keys, no dictionary popover in GTKgal, ADWgal or `GS/`; the emoji chooser is GN-2.43 |
| MAC-N35† | 2 | GN-2.13 |
| MAT-N31† | 3 | No wheel picker (pass 2) |
| W2-41† | 3 | No radial gauge; the level bar is GN-2.26 |
| W2-43† | 3 | No chip (pass 2) |
| W-RL03† | 2 | GN-RL01†, GN-2.51 |
| IOS X-MO-11† | 2 | GN-3.23 |
| IOS X-FO-3†, MAT-X34†, MAC-X31† (focus) | 2 | GN-3.27, GN-3.28, GN-3.30 |
| MAC-X31† (window title icons) | 3 | No proxy icon in a title (AdwWindowTitle, as MAC-W11†) |
| IOS X-BT-2† | 2 | GN-3.47, GN-3.48, GN-3.49 |
| MAT-X32† (slops) | 2 | GN-3.47, GN-3.48 |
| MAT-X32† (wheel factor), W3-52† | 1 | GN-P3-15† (the wheel step); kinetic scrolling and overshoot GN-3.24 |
| K3.38† | 3 | No docks (pass 2, "dock/panel title bar"); the toolbar view's bar reveal is GN-P3-5† |
| K3.48† | 2 | GN-1.281† |
| K3.65†, MAT-X33† | 2 | GN-3.67†, GN-3.46; no pointer colour key (`S/org.gnome.desktop.interface`, `S/org.gnome.desktop.a11y.mouse`: none) |
| K3.76† | 2 | GN-3.69† |
| K3.84†, MAC-X38† | 1 | GN-P3-16† (display scale per monitor; how a 1-px line draws at a fractional scale) |
| K3.95† | 2 | GN-3.70†; its flash, left there to measure, is published: GN-P3-14† |
| K3.96† | 3 | No shake-to-find (no such gesture in `GS/` or `M/src`, grep: mutter's "shake" is shaking a maximised window loose, `M/src/compositor/meta-window-drag.c`); GNOME's pointer finder is GN-P3-11† |
| W3-50† | 1 | GN-P3-11† (locate-pointer ripples, the click-assist pie timer); no pointer colour, trails or touch visualisation keys (`S/org.gnome.desktop.a11y.mouse`, `S/org.gnome.desktop.interface`) |
| MAC-X35† | 3 | No compatibility look: AdwStyleManager's properties are colour scheme, dark, high contrast, accent colour and the document and monospace fonts (`A/c/adw-style-manager.c`) |
| MAC-X36† | 1 | GN-P3-17† (libadwaita, GTK 4's Default theme and GTK 3's Adwaita draw differently) |
| MAC-X37† | 1 | GN-P3-18† (libadwaita versions draw differently) |
| MAT-X35†, W3-56† (colour inversion, saturation) | 1 | GN-P3-13† (GNOME Shell's magnifier colour effects) |
| MAT-X35†, W3-56† (colour filter, force dark) | 3 | No colour-filter or force-dark key: gsettings-desktop-schemas 51.0 has no such schema (its `schemas/` file list), and none in `M/` or `GS/` (grep); high contrast GN-3.61 |
| W3-48† | 2 | GN-3.50 (underlines after 300 ms of Alt); no setting to keep them: no mnemonic property in `G/gtksettings.c` (grep) |
| W3-49† (drag cursor per action) | 1 | GN-P3-12† |
| W3-49† (drag image, drop description) | 2 / 3 | Drag image GN-2.49, drop highlight GN-3.66†; no drop-description text (none in `GDK/gdkdrag.c`, grep) |
| W3-53† | 2 | GN-3.25 (`enable-animations` switches all motion) |
| W3-55† | 3 | `left-handed` only swaps buttons (`S/org.gnome.desktop.peripherals:172-175`); GtkPopover places nothing by handedness (`G/gtkpopover.c`, grep) |

Tally: 178 table lines — 25 decided 1, 101 decided 2, 51 decided 3, one "2 / 3"; counted by input row, 277 mentions of the 261 rows (16 rows are split): 37 under 1, 158 under 2, 81 under 3, 1 under "2 / 3". Every input row is named. Rows added: 27 (§4.1 9, §4.2 6, §4.3 12).

### Rows added

| ID | section | model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| GN-P3-1† | 4.1 | [window] | Tile preview (GNOME Shell, while a window is dragged to a screen edge) | A rectangle in `-st-accent-color` at 50 % transparency with a 1 px accent border, drawn below the dragged window; it starts at the window's frame on its monitor at opacity 0 and eases to the tile's rectangle and full opacity in 250 ms ease-out-quad, and fades out in 250 ms; style classes `on-primary`, `tile-preview-left` / `-right` are set but have no rule | P: `GS/windowManager.js:34`, `:384-470`; `GS-data/theme/gnome-shell-sass/widgets/_misc.scss:50-53` (grep: no rule for the other classes) | none | C20, C14, C04, C12 | P |
| GN-P3-2† | 4.1 | [window] | Resize size label (X11 windows with size increments, e.g. terminals) | While such a window is resized, a label "W × H" in increments (columns × rows), centred on the window in the shell's OSD panel style | P: `M/src/x11/window-x11.c:1042-1068`; `GS/windowManager.js:472-495`, `:1973-1984`; `GS-data/theme/gnome-shell-sass/widgets/_osd.scss:64-66` | none | C20, C24 | P |
| GN-P3-3† | 4.1 | [window] | Window menu (title-bar right click) | A right click on a header bar runs `action-right-click-titlebar` (default `'menu'`) and asks the compositor for the window menu; GNOME Shell draws it as a shell popup menu (`.window-menu`, box-pointer side top) with Take Screenshot, Hide, Restore / Maximize, Move, Resize, Always on Top, Always on Visible Workspace, Move to Workspace / Monitor …, Close, in the shell's theme; without compositor support GTK shows its own arrowless popover menu (Restore, Minimize, Maximize, Close) at the pointer | R: `S/org.gnome.desktop.wm.preferences:126-129`; P: `G/gtkwindowhandle.c:161-262`, `:264-279`, `:403-406`; `GS/windowMenu.js:9-13`, `:27-189`; `GS/windowManager.js:527`, `:1699-1700`; the shell's popup-menu rules: to verify | `menu.*` (in-app menus only) | C20, C24, C13 | R, P |
| GN-P3-4† | 4.1 | [window] | Window controls per window kind | The icon and minimize button only on a "sovereign" window (not modal, not transient for a parent); maximize only when it is also resizable, its glyph `window-restore-symbolic` while maximized; close only when deletable. So a dialog window shows close alone and a non-resizable window has no maximize | P: `G/gtkwindowcontrols.c:305-309`, `:327-347`, `:366-374`, `:393-405` | none (GN-1.20 is the layout key only) | C20, C07, C13 | P |
| GN-P3-5† | 4.1 | [window] | Full screen | GtkWindow hides its own title bar while full screen; libadwaita's header bar sits in the content, and the app hides and shows AdwToolbarView's bars with `reveal-top-bars` / `reveal-bottom-bars` (GtkRevealer: top slide-down, bottom slide-up, 250 ms), optionally with the content extending under them (`extend-content-to-top-edge` / `-bottom-edge`); no toolkit rule reveals them on pointer hover. Entering and leaving full screen, and moving to another monitor, run GNOME Shell's size-change animation, the one GN-3.71† gives for maximise (only for normal windows) | P: `G/gtkwindow.c:3936-3949`; `A/c/adw-toolbar-view.c:611-660`, `:736-753`; `G/gtkrevealer.c:717`, `:769-770`; `M/src/core/window.c:3792-3793`, `:3865-3866`, `:4766-4767`; `GS/windowManager.js:1299-1312` | none | C20, C14, C26 | P |
| GN-P3-6† | 4.1 | [window] | Not-responding window (GNOME Shell) | When a window misses mutter's ping (`check-alive-timeout`, 5000 ms), the shell darkens the window's surface (brightness −0.3) and lays a shell message dialog over it, sized to the window: title "“App” Is Not Responding", a description, buttons Force Quit (default) and Wait; the dialog scales in from 0.8 (ease-out-back) and out to 0.8 while fading (ease-out-quad), 150 ms each | R: `M/data/org.gnome.mutter.gschema.xml.in:112-113`; P: `GS/closeDialog.js:10-12`, `:34-43`, `:59-79`, `:87-96`, `:113-169`; the dialog's stylesheet rules: to verify | none | C20, C06, C14, C24 | R, P |
| GN-P3-7† | 4.3 | window chrome | Window asking for attention | A window that is not focused and sets demands-attention or urgent gets no flash: GNOME Shell posts a notification (the app's name and icon, body "“<window title>” is ready"), activating it focuses the window; skip-taskbar windows get none | P: `GS/windowAttentionHandler.js:10-50`, `:59-77` | none | C20, C21, C24 | P |
| GN-P3-8† | 4.3 | motion | App launch from the dash or app grid (GNOME Shell) | A copy of the clicked icon scales to 3× and fades to 0 in 250 ms ease-out-quad, kept inside the monitor; when the app was not running or a new window is asked for; no busy cursor | P: `GS/appDisplay.js:3062-3078`; `GS/iconGrid.js:25-26`, `:186-221` | none | C14, C22 | P |
| GN-P3-9† | 4.3 | placement | On-screen keyboard over a text field (GNOME Shell) | When the caret rectangle the input method reports falls behind the keyboard, the focused window slides up by the keyboard's height (150 ms ease-out-quad) and back when the keyboard hides; a window moved by the user is let go | P: `GS/keyboard.js:20`, `:1753-1796`, `:1798-1848` | none | C26, C14 | P |
| GN-P3-10† | 4.2 | text input system UI | IME candidate popup (GNOME Shell, IBus) | A shell box-pointer popup under the caret rectangle: content padding 6, spacing 6, radius 12, popup-menu fill; candidates in a vertical or horizontal list (as IBus asks), each padding 6 12, radius 8, selected in accent fill / accent fg, hover fill; an index label in the insensitive colour, padding-right 6; a pre-edit / auxiliary text line; previous / next page buttons, padding 6, icons 16, joined (inner radii 0) | P: `GS-data/theme/gnome-shell-sass/widgets/_ibus-popup.scss:1-40`; `GS-data/theme/gnome-shell-sass/_common.scss:31`, `:33`, `:51`; `GS/ibusCandidatePopup.js:32-70`, `:99-115`, `:151-176`, `:210-235`, `:328-333` | none | C24, C13, C26 | P |
| GN-P3-11† | 4.3 | pointer | Pointer locator and click-assist timer (GNOME Shell) | `locate-pointer` (default false), triggered by `locate-pointer-key` (`Control_L`): three concentric ripples at the pointer, 50 px circles in a lightened, 70 %-transparent accent with a 2 px lightened-accent glow, delays 0 / 50 / 350 ms, durations 830 / 1000 / 1000 ms, scaling 0.25→1.5 / 0→1.25 / 0→1, start opacity √1.0 / √0.7 / √0.3 fading to 0. Dwell click and secondary click (both off by default, 1.20 s): a 60×60 pie timer at the pointer, 3 px accent ring, lightened accent fill, fading in over a quarter of the time and sweeping linearly, then zooming to 2× and fading in 150 ms | R: `S/org.gnome.desktop.interface:270-271`; `M/data/org.gnome.mutter.gschema.xml.in:104-105`; `S/org.gnome.desktop.a11y.mouse:4-5`, `:44-45`, `:49-50`, `:54-55`; P: `GS/locatePointer.js:5-35`; `GS/ripples.js:48-77`, `:107-110`; `GS/pointerA11yTimeout.js:7`, `:11-64`, `:80-104`, `:114-130`; `GS-data/theme/gnome-shell-sass/widgets/_a11y.scss:2-17`, `_corner-ripple.scss:3` | none | C22, C23, C14, C25 | R, P |
| GN-P3-12† | 4.3 | pointer | Pointer during a drag | The cursor names the drag's action: `default` (none), `dnd-ask`, `copy`, `dnd-move` (not `move`, "stuck using a mismatched visual metaphor in Adwaita"), `alias` (link); GN-3.67† has no drag cursors | P: `GDK/gdkdrag.c:776-801`; `GDK/wayland/gdkdrag-wayland.c:170` | none | C22, C07 | P |
| GN-P3-13† | 4.3 | accessibility | Magnifier colour effects (GNOME Shell) | With the screen magnifier on, the magnified view can be lightness-inverted (`invert-lightness`, false), desaturated (`color-saturation`, 1.0) and have its brightness and contrast shifted per channel (`brightness-*`, `contrast-*`, 0.0); cross-hairs (thickness 8, `#ff0000`, opacity 0.66, length 4096); a lens-mode view has a 2 px accent border (none full-screen). GNOME has no colour-blind or inversion filter for the whole screen (the lightness inversion is the magnifier's only); its whole-screen colour filter is Night Light, GN-P4-9† | R: `S/org.gnome.desktop.a11y.magnifier:62-71`, `:108-133`, `:150-218`; P: `GS/magnifier.js:24-44`, `:1417-1455`; `GS-data/theme/gnome-shell-sass/widgets/_a11y.scss:19-24` | `AccessibilityPreferences` (no colour effect) | C23, C06, C25 | R, P (review 2026-10-28: "no system colour filter" narrowed to colour-blind and inversion filters; Night Light exists, GN-P4-9†. No colour-blind, daltonize or greyscale code in gnome-shell 51.0 `js/`, `src/` or mutter 51.0 `src/`; `src/shell-invert-lightness-effect.c` is used by the magnifier only, grep) |
| GN-P3-14† | 4.3 | accessibility | Visual bell flash (the values GN-3.70† left to measure) | mutter draws it: a black overlay over the screen (fullscreen-flash) or over the whole window that rang the bell (frame-flash; the screen when the bell names no window), opacity 0 → 192/255 in 50 ms ease-in-quad, auto-reversed, the transition's repeat count 2, or 1 when the previous visual bell was under 3 s ago; bells under 500 ms apart flash nothing | P: `M/src/compositor/compositor.c:1454-1545`; `M/src/core/bell.c:60-61`, `:137-166`, `:203-225`, `:242-248`; `M/src/wayland/meta-wayland-system-bell.c:60-62` | none | C23, C21, C14 | P (review 2026-10-28: "the focused window" → the window that rang the bell, which is the window mutter passes in; the schema's "focused window" text is not what the code does) |
| GN-P3-15† | 4.3 | timings | Scroll distance per wheel step | One wheel detent scrolls page_size^(2/3) pixels of the scrolled view (smaller steps in smaller views); touchpad (surface) deltas are multiplied by 2.5; wheel kinetics start from the same step; a wheel step on a GtkRange (slider) moves it by its adjustment's page increment. No gsetting changes it | P: `G/gtkscrolledwindow.c:201`, `:1206-1229`, `:1390-1433`, `:1505-1546`; `G/gtkrange.c:1888-1896` | none | C21 | P |
| GN-P3-16† | 4.3 | size variants | Display scale per monitor | Each monitor has a scale, fractional on Wayland (`gdk_monitor_get_scale`, 4.14; `gdk_surface_get_scale`, 4.12) next to the integer scale factor; widgets are laid out in logical pixels and rendered at the surface's scale. How libadwaita's 1 px lines (borders, the `0 0 0 1px` outlines of GN-3.02) draw at 125 % or 150 %: measure. The reader handles only X11's text-scale coupling (`native-theme/src/gnome/mod.rs:565-591`) | R: `GDK/gdkmonitor.c:478-514`; `GDK/gdksurface.c:2438-2466`; M: GTK probe at fractional scales, screenshots | none | C10, C12, C25 | R, M |
| GN-P3-17† | 4.3 | size variants | Which stylesheet draws a GTK app | libadwaita apps draw libadwaita's stylesheet (it sets `gtk-theme-name` to `Adwaita-empty` unless `GTK_THEME` is set or Granite is present). A GTK 4 app without libadwaita draws GTK's built-in **Default** theme: the `gtk-theme` key's default `'Adwaita'` is no GTK 4 theme, so GTK falls back to Default (`Adwaita-dark`, `HighContrast`, `HighContrastInverse` map to its dark and high-contrast variants); its radii are button 5, menu 5, popover 9 and window 8, against libadwaita's button 9, menu 9, popover 15. A GTK 3 app draws GTK 3's own `Adwaita` theme (button 5, menu 5, popover 9, window 8, as GTK 4's Default). The preset carries libadwaita only | R: `S/org.gnome.desktop.interface:93-94`; `GDK/wayland/gdksettings-wayland.c:270`; P: `A/c/adw-style-manager.c:454-466`; `G/gtkcssprovider.c:2041-2066`; `G/gtksettingsprivate.h:25`; `G/theme/Default/_common.scss:12-16`; `A/s/_common.scss:8-13`; `G3/theme/Adwaita/_common.scss:9-12` | none (one preset, `adwaita`) | C09, C25 | R, P (review 2026-10-28: GTK 3's radii "5 / 5 / 8 / 9" named, in the order this row uses; the bare list read as popover 8, window 9) |
| GN-P3-18† | 4.3 | size variants | libadwaita versions | An app draws the libadwaita it links, and the look changes between minor versions: 1.7 raised border radii, made the greys colder, added hover / active transitions to switch, check, radio and slider and widened scrollbar padding; 1.8 raised the body and caption line height and added flow-box hover / active; 1.10 lowered large-icon weight, changed the popover shadow; the check glyph differs between 1.7–1.9 and 1.10 (`docs/platform-facts.md@fc6109d1:1242`). This file reads 1.10.0 only | P: `A/NEWS:153-158` (1.10.alpha), `:499-505` (1.8.beta), `:805-810` (1.7.alpha) | none | C25 | P |
| GN-P3-19† | 4.3 | icons | Animated and stateful icons (GtkSvg, GTK 4.22+) | GTK renders SVG with animations and up to 64 states whose transitions animate; GTK 4.24 ships two such icons, `process-working.gpa` (GtkSpinner, GN-1.230) and `sliders.gpa` (the font dialog's "Change Font Features" toggle). Icon themes are still looked up as static `.svg` / `.png` (`G/gtkicontheme.c` has no `.gpa`, grep) | P: `G/svg/gtksvg.c:107-127`, `:2157-2163`; `G/gtkspinner.c:159`; `G/gtkfontchooserdialog.c:195-210` | none (icons are static) | C19, C14, C07 | P |
| GN-P3-20† | 4.2 | font chooser | GtkFontChooserWidget (GtkFontDialog's chooser) | A `.linked` search entry and a "Filters" menu button (popover: "Filter by" in `.title-4`, margin-bottom 10, Monospace and Language checks, a framed language list, spacing 6); the family list, framed, each row drawn in its own face with margins 20 / 10; a preview entry; a Size row of label, slider and spin button; grid spacing 6; a "No Fonts Found" placeholder; a font-features page (its margins and spacings at `:338-388`, contents to verify); the dialog's tweak toggle with the animated sliders icon (GN-P3-19†). libadwaita has no rule for it beyond `button.font` (GN-2.28) | P: `G/ui/gtkfontchooserwidget.ui:55-146`, `:148-200`, `:208-275`, `:276-336`, `:338-388`; `G/gtkfontchooserdialog.c:195-210`; grep of `A/w/` | `list.*`, `input.*` | C24, C13 | P |
| GN-P3-21† | 4.2 | file chooser | Who draws the file dialog | GtkFileDialog runs a native chooser that goes through the file-chooser portal when one is available; since GNOME 47 the open and save dialogs "are based on the existing Files app rather than being a separate codebase" (GN47), and xdg-desktop-portal-gnome 51.0 still hands every FileChooser request to `org.gnome.Nautilus`, so a GTK app's file dialog looks like Files (Nautilus), not GTK's chooser (GN-2.48, GN-P3-22†, GN-P3-23†); what Files draws: to verify in Files' source | P: `G/gtkfiledialog.c:917`; `G/gtkfilechoosernativeportal.c:487`; `GDK/gdk.c:617-650`; GN47; xdg-desktop-portal-gnome tag 51.0 `src/filechooser.c:278-285`, `:383-393` (gitlab.gnome.org, read 2026-10-28) | `sidebar.*`, `list.*` | C24, C25 | P, V (review 2026-10-28: "GNOME 47 and later" rested on the GNOME 47 notes only; the 51.0 portal backend now cited) |
| GN-P3-22† | 4.2 | file chooser | Places sidebar rows (GTK's own chooser; the part GN-2.48 left to verify) | Rows of a `.navigation-sidebar` with padding 0 14 on the inner revealer; icon at 70 % with an 8 px gap (mirrored in RTL), label end padding 2; an eject button (`media-eject-symbolic`, tooltip "Unmount", margin-start 4) that is flat, round, 26×26, margins 3, at 70 % and 100 % on hover / press, and a hidden busy spinner beside it; sections separated by a horizontal separator, no headings; a new-bookmark row in `--accent-color`; a drop placeholder row 2 px of the drop colour; a drop target gets the drop colour and a 1 px inset ring; a selected row shows no pressed shadow | P: `A/w/_file-chooser.scss:1-64`; `G/ui/gtksidebarrow.ui:1-74`; `G/gtkplacessidebar.c:377-401` | `sidebar.*` | C13, C07, C24 | P (review 2026-10-28: the `.ui` range ended past the file's 74 lines) |
| GN-P3-23† | 4.2 | file chooser | File list and grid (GTK's own chooser) | A 1 px `$border_color` line just below the chooser's bottom edge (`box-shadow: 0 1px`, outside the box); the view-toggle and new-folder buttons beside the path bar flat (the path bar's own buttons are not, GN-2.52†); list cells padding 8 6; grid view padding 15 with a top undershoot, items radius 12, margin 3, cell padding 6 12; the "Other Locations" server-list button's arrow rotates half a turn in 200 ms ease-out-quad; the remaining rules (`:130-266`): to verify | P: `A/w/_file-chooser.scss:66-130`; `G/ui/gtkfilechooserwidget.ui:30-80` | `list.*` | C13, C11, C14 | P, V (review 2026-10-28: the line is below the chooser, not under its top; the flat rule's targets corrected) |
| GN-P3-24† | 4.1 | [layout] | Wrap box and flow box spacing (the defaults GN-2.46 left to verify) | AdwWrapBox / AdwWrapLayout: `child-spacing` 0 and `line-spacing` 0 (px), no justification, natural wrapping, start-to-end; GtkFlowBox: `row-spacing` and `column-spacing` 0, at most 7 children per line; no stylesheet rule sets them, so the gaps are the app's | P: `A/c/adw-wrap-box.c:285-287`, `:393-395`; `A/c/adw-wrap-layout.c:1003-1004`, `:1111-1112`, `:1223-1230`; `G/gtkflowbox.c:728`, `:3757-3790` | `layout.widget_gap` | C26 | P |
| GN-P3-25† | 4.1 | [segmented_control] | Toggle width rule | AdwToggleGroup's `homogeneous` defaults to false: each toggle takes its content's width (at least GN-1.244's minimums), equal widths only when the app asks; the group's height is its content's | P: `A/c/adw-toggle-group.c:1146-1148` | none | C26 | P |
| GN-P3-26† | 4.1 | [input] | Text magnifier (touch) | While a selection handle is dragged, GtkText shows a 100×60 magnifier at 2× zoom in a non-autohiding popover above the caret, class `.magnifier` (OSD look GN-1.184, view fill GN-2.42); GtkTextView's is the same at 2× with no size set; its offset from the caret: measure | P: `G/gtktext.c:2205-2221`, `:3110`; `G/gtktextview.c:2126-2141`; `A/w/_popovers.scss:64-70`; `A/w/_text-selection.scss:28-30` | none | C13, C17, C26 | P, M |
| GN-P3-27† | 4.2 | code editor | Spell-check underline (libspelling, used with GtkSourceView) | A misspelled word gets Pango's error underline (`PANGO_UNDERLINE_ERROR_LINE`) in `#e01b24`, the colour fixed in code; the scheme's `def:misspelled-word` style overrides it when stated, which Adwaita and Adwaita-dark do not. GTK and libadwaita have no spell checker; whether libspelling counts as the platform is §6.2's question, as for GtkSourceView | P: `LS/lib/spelling-text-buffer-adapter.c:533-565`, `:606-615`; grep of `gsv/data/styles/Adwaita*.xml` | none | C17, C05, C24 | P |

Found while answering, about this file: GN-3.71† lists the size-change animation for maximise and tiling only, but mutter runs it for full screen and monitor moves too (GN-P3-5†); GN-2.46 and GN-2.48 left their values to verify (now GN-P3-24†, GN-P3-22†, GN-P3-23†); GN-3.70† left the visual bell to measure, but mutter publishes it (GN-P3-14†); and GN-2.48's premise, that GTK draws the chooser "when the portal is not used", misses that on GNOME 47+ the portal's chooser is Files (GN-P3-21†). (Review 2026-10-28: all four are now fixed in those rows: GN-3.71†, GN-2.46, GN-2.48, GN-3.70†.)

## Pass 4 (2026-10-28)

The rows pass 3 and the review added were grouped into kinds and each kind put to
every platform; the coverage table is the main map's Appendix B. This section holds
this platform's rows for the kinds it had none for. Sources are cited in full in
each row.

Sources as in the GNOME file (`A/w/`, `A/s/`, `A/c/`, `G/`, `GDK/`, `S/`, `M/`, `GS/`, `GS-data/`, GTKgal, ADWgal; `gsv/` GtkSourceView 5.22.0 as in the seeds file). New in this pass (fetched from gitlab.gnome.org at the tags, read 2026-10-28):

| Short | Source |
|---|---|
| `GSD/F:L` | gnome-settings-daemon tag **51.1** (commit `4a75955f7481e2bb19fa73fa67456e83d67e8563`), path from the repository root |
| `XDPG/F:L` | xdg-desktop-portal-gnome tag **51.0** (commit `99446c9c9d8197ac4651c5a8e1ee9eb771e1e120`), path from the repository root |

GN-P4-9† (Night Light, kind K23) completes GN-P3-13†, whose "GNOME has no system colour filter" holds for colour-blind and inversion filters only.

| ID | section | model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| GN-P4-1† | 4.3 | window chrome | Screen behind a GNOME Shell system prompt (polkit authentication, keyring, network secrets) | Every shell modal dialog that is not shell-reactive (`ModalDialog`) covers the whole stage with a lightbox under the dialog: a black layer through a radial vignette shader with `brightness` 0.5 and `sharpness` 0.7 (per-pixel alpha `1 − mix(1, 1 − sharpness, t) × brightness`, `t` the distance from the centre scaled to reach 1 at the corners, plus dithering noise), switched on at once (`lightOn()` with no duration); the dialog and its lightbox fade in over 100 ms ease-out-quad and out the same way. The authentication, keyring and network-secret prompts are such dialogs. The alpha per pixel as drawn: measure | P: `GS/modalDialog.js:13`, `:36-91`, `:128-147`, `:178-196`; `GS/lightbox.js:8-28`, `:119-142`, `:191-219`; `GS/components/polkitAgent.js:30-32`; `GS/components/keyring.js:16-18`; `GS/components/networkAgent.js:24-26`; M: screenshot of a polkit prompt over a known background | none (GN-3.72† dims only an app's parent window) | C06, C20, C14 | P, M |
| GN-P4-2† | 4.1 | `[menu]`, `[combo_box]`, `[popover]` | Where a popover menu or drop-down list opens against its owner | GtkPopover's `position` defaults to bottom: the popover hangs below its owner, centred on it (start- or end-aligned when the popover's `halign` says so, mirrored in RTL), flips above when there is no room below (`GDK_ANCHOR_FLIP_Y`), slides sideways to stay on screen (`GDK_ANCHOR_SLIDE_X`), and lets the compositor shrink it when its minimum is below its natural size; a menu button's `direction` (default down) picks the side. A GtkDropDown's list is a popover without arrow, start-aligned, whose width request is the drop-down's own width, so it is at least as wide as the trigger; other popovers take their content's width (GN-1.185) | P: `G/gtkpopover.c:599-628`, `:683`, `:685`, `:1042-1043`, `:1970-1972`; `G/gtkmenubutton.c:40-52`, `:445-449`, `:840-862`; `G/gtkdropdown.c:516-528`; `G/ui/gtkdropdown.ui:55-59` | none (GN-1.181 the gap, GN-1.185 the width) | C26 | P |
| GN-P4-3† | 4.2 | app chooser (portal) | "Open With" dialog of xdg-desktop-portal-gnome (the AppChooser portal) | On GNOME the AppChooser portal is xdg-desktop-portal-gnome's own window, not GTK's GtkAppChooserDialog (GN-2.57†): a non-resizable AdwWindow titled "Open With"; an AdwHeaderBar without window buttons, Cancel at its start and Open (`.suggested-action`, insensitive until an app is selected; the default app, when one is given, starts selected) at its end; an AdwPreferencesPage with a centred description ("Choose an app to open the file “%s”"); a `.boxed-list` GtkListBox of rows: app icon 32 px with margins 10, name label 30 characters wide and ellipsized, spacing 10, an `object-select-symbolic` check (margins 10) shown only on the selected row; the first 3 apps, then a row with a 16 px `view-more-symbolic` image (margins 12) that shows the rest; with no app, an AdwStatusPage (`org.gnome.Software-symbolic`, "No Apps Available") with a `.suggested-action` `.pill` "Find More in Software" button. The same backend provides the Print portal, which shows GTK's GtkPrintUnixDialog (GN-2.57†'s look) in the portal's process. A GtkFileLauncher with `always-ask` sends the OpenURI portal's `ask` option; when the portal front end shows the chooser without it (xdg-desktop-portal's rule, not GNOME's): to verify | P: `XDPG/src/appchooserdialog.ui:4-97`; `XDPG/src/appchooserrow.ui:4-37`; `XDPG/src/appchooserdialog.c:37`, `:340-427`; `XDPG/src/appchooserrow.c:86`, `:102`; `XDPG/data/meson.build:10`, `:19`; `XDPG/src/print.c:579`, `:726`; `G/gtkfilelauncher.c:596-606`; `G/gtkopenuriportal.c:321-324` | `list.*`, `dialog.*` | C24, C13, C26 | P, V (review 2026-10-28: the default app's preselection added, `XDPG/src/appchooserdialog.c:392-397`; the "to verify" narrowed: GtkFileLauncher's `always-ask` is the portal's `ask`) |
| GN-P4-4† | 4.1 | `[text_area]` (and the code editor) | Block caret of GtkTextView / GtkSourceView (overwrite mode; GtkSourceView's Vim mode) | In overwrite mode a GtkTextView draws, instead of the line caret, a rectangle over the character after the insertion point (its Pango extents; none over a zero-width character; at a line end the layout's character width, or none there when the strong and weak carets split), filled with the widget's `caret-color` (GtkSourceView: the scheme's `cursor`, GN-SF03†) at the blink opacity, the character redrawn inside it in the widget's `background-color`; only while the view has focus and no text is selected. GtkSourceView's Vim emulation turns overwrite on in its normal and replace states and off in insert, so its block caret is this one. GN-1.49 is GtkText's (entries') `block-cursor` node, a different rule. GtkSourceView 5.22.0 has no multiple carets (no multi-cursor code, grep of `gsv/gtksourceview/`) | P: `G/gtktextlayout.c:1790-1810`, `:3933-3970`; `G/gskpango.c:465-473`; `G/gtktextutil.c:323-441`; `gsv/gtksourceview/vim/gtksourcevimnormal.c:1460`, `:1508`; `gsv/gtksourceview/vim/gtksourcevimreplace.c:99`; `gsv/gtksourceview/vim/gtksourceviminsert.c:347`, `:380` | `input.caret_color` (line caret only) | C16 | P (review 2026-10-28: the split-caret case is a line end's only, `G/gtktextutil.c:385-396`) |
| GN-P4-5† | 4.1 | `[scrollbar]`, `[slider]` | Click in a scroll-bar or slider trough | GtkRange (scroll bars, scales): with `gtk-primary-button-warps-slider` (default true) a primary click in the trough moves the slider to the pointer and goes on as a drag; Shift + primary or a middle click pages towards the pointer instead (the reverse when the setting is false); a touch always warps. The paging repeat is GN-3.69†. No GNOME gsetting feeds the setting on Wayland (none in `GDK/wayland/gdksettings-wayland.c:270-312`, grep); on X11 GTK reads it from the XSetting `Gtk/PrimaryButtonWarpsSlider`, which gnome-settings-daemon 51.0's xsettings plugin does not set (neither its fixed list nor its gsettings translations), so on GNOME the default holds | P: `G/gtksettings.c:723-738`; `G/gtkrange.c:2025-2110`; `GDK/x11/gdksettings.c:66`; `GSD/plugins/xsettings/gsd-xsettings-manager.c:463-483`, `:485-515` (no "Warp" in it, `gsd-xsettings-gtk.c` or `xsettings-manager.c`, grep) | none (no click behaviour in `scrollbar.*`) | C21, C25 | P (review 2026-10-28: "which daemon sets it: to verify" settled, none does; SRC V dropped) |
| GN-P4-6† | 4.1 | `[menu]` | Submenu switching in nested popover menus (the menu bar, `GTK_POPOVER_MENU_NESTED`) | Entering an item opens its submenu at once when no submenu is open; while one is open, every pointer motion over another item restarts an 80 ms timer, and only when it fires is the open submenu closed and the item's own submenu (if any) opened, so an open submenu stays while the pointer keeps moving across other items; no aim rule (no `sloppy` or direction test in `G/gtkmodelbutton.c`, `G/gtkpopovermenu.c`, `G/gtkpopovermenubar.c`, grep). Nested submenus are arrowless popovers opening to the item's right, aligned to its top. Default popover menus (`GTK_POPOVER_MENU_SLIDING`) slide submenus in as pages instead (GN-1.76) | P: `G/gtkmodelbutton.c:834`, `:1389-1420`, `:1423-1436`, `:1445-1485`, `:1556-1558`; `G/gtkmenusectionbox.c:345-366`; `G/gtkpopovermenubar.c:477`; `G/gtkpopovermenu.c:794`; `G/gtkpopover.c:549-555` | none | C21 | P (review 2026-10-28: the submenu's side, right of the item, added with its lines) |
| GN-P4-7† | 4.1 | `[checkbox]` | Check mark shape | The checked mark is `assets/check.svg`, a 14 × 14 filled path recoloured with the foreground: the outline of a 2 px stroke with round caps and a round outer join along (2, 7) → (6, 11) → (12, 3) (the centres of the path's radius-1 arcs); the radio's dot is `assets/bullet.svg`, the indeterminate dash GN-1.58. The model has the stroke width (`checkbox.check_mark_stroke_width_px`, `platform-facts.md@fc6109d1:1242`) but not the mark's shape, so a connector draws its own check | P: `A/w/_checks.scss:70`, `:76`, `:80`; `A/s/assets/check.svg:2-5`, `:45-47` | `checkbox.check_mark_stroke_width_px` | C13 | P |
| GN-P4-8† | 4.1 | `[menu]` | Text context menu (right click, Menu key) of GtkText (entries) and GtkTextView | A sliding popover menu (look GN-1.66–1.80) without arrow, opening below a 1 × 1 rectangle at the click point (opened from the keyboard: an entry's menu points at the whole widget, a text view's at the insertion point when it is in view), start-aligned, placed as GN-P4-2† says; items: entry — Cut, Copy, Paste, Delete \| Select All, Change direction and Insert Emoji (the last two hidden when disabled); text view — Cut, Copy, Paste, Delete \| Undo, Redo \| Select All, Insert Emoji (hidden when disabled); then the app's extra menu. On touch the selection bubble GN-1.286† replaces it | P: `G/gtktext.c:2845-2882`, `:6372-6427`; `G/gtktextview.c:9248-9310`, `:9325-9415` | `menu.*` | C26, C13 | P (review 2026-10-28: "the K47 row" named as GN-P4-2†; the text view's Insert Emoji is hidden when disabled too, `G/gtktextview.c:9298-9299`) |
| GN-P4-9† | 4.3 | colour filters | Night Light (gnome-settings-daemon's colour plugin, applied by mutter) | A colour-temperature shift of every pixel: `night-light-enabled` (false), `night-light-temperature` 2700 K (6500 K is neutral), from sunset to sunrise (`night-light-schedule-automatic` true) or from `night-light-schedule-from` 20.00 to `night-light-schedule-to` 6.00; the temperature ramps linearly from 6500 K to the set value over the hour before the schedule starts and back over the hour before it ends (shorter when the schedule is shorter), and every change of 10 K or more (the ramp's steps too) is smoothed over 5 s; mutter applies it to each monitor's gamma LUT, with a colour-transform matrix as the fallback; state on D-Bus `org.gnome.SettingsDaemon.Color` (`NightLightActive`, `Temperature`). Every colour of the theme shows warmer. (GN-P3-13†'s "GNOME has no system colour filter" holds for colour-blind and inversion filters only) | R: `GSD/data/org.gnome.settings-daemon.plugins.color.gschema.xml.in:14-38`; `M/data/dbus-interfaces/org.gnome.SettingsDaemon.Color.xml:1-7`; P: `GSD/plugins/color/gsd-night-light.c:68-74`, `:124-129`, `:182-236`, `:258`, `:332-380`, `:795`; `GSD/plugins/color/gsd-color-state.h:32-34`; `M/src/backends/meta-color-manager.c:62`, `:104-107`; `M/src/backends/meta-color-device.c:1499-1516`; `M/src/backends/meta-monitor.c:981-989` | none | C23, C06, C25 | R, P (review 2026-10-28: "any other change" read as excluding the ramp; the ramp's steps go through the same smoothing, `GSD/plugins/color/gsd-night-light.c:384`) |
