# v0.7.1 — Model completeness: Specification, widget catalogue

**Status:** draft, 2026-10-28; merged from eight groups' catalogues, not yet through a check pass. Part of the
specification ([`todo_v0.7.1_model-completeness-spec.md`](todo_v0.7.1_model-completeness-spec.md), "S…"; its
decisions are the rationale's, [`todo_v0.7.1_model-completeness-rationale.md`](todo_v0.7.1_model-completeness-rationale.md),
"R…"). The traceability is in `todo_v0.7.1_model-completeness-spec-trace-<platform>.md`.

## What this is

Every **typed widget** (a kind two or more platforms draw, R14, S5.1) with its parts, widget values, variants,
sizes, flags and the source of each platform's state model; every **keyed component** (a kind one platform draws);
the **colour roles and text ramp** S3 and S4.4 leave open; and the **tokens, conditions, readers and placement
strategies** the rows need beyond S4, S7.1, S9 and S13. It states structure only: no platform value appears here
(values come from the facts, S17). Rows of the gap map are cited by ID.

Eight groups wrote it in parallel, each for a share of the map's rows: window chrome and overlays (group 1),
buttons and toggles (2), fields and indicators (3), containers (4), colour roles and text ramp (5), the controls the
model lacks that two or more platforms list (6), the controls only one platform lists (7), and what crosses widgets
(8). The 47 kirigami-addons rows (K-KA-n†) were traced during the merge. Each section says which group wrote it;
what other groups proposed for a widget is merged at the end of its section ("Merged from other groups and rows").
Where groups named one thing differently, the merge kept one name (the rename table below); where a group needed
something the core lacks, the addition is cited as `ADD-Ann` (a detail inside a decided mechanism), `ADD-Bnn` (a
mechanism the rationale did not decide) or `CONTRA-n` (a contradiction with the rationale), from the merged list kept
for the specification's check pass.

## Entry format

Each `### <widget>` section states:

- **Kind:** typed widget, and which platforms draw it (counting the cross-check decisions that found it drawn
  inside an element, R14).
- **Parts:** each part's name and what it is. Every part is a `PartStyle` (S2.11), so only names are listed with the
  fields that matter; a part marked **element** (or *(element)*) has its own interaction state and is queried as an
  element of its own (glossary), its ancestor being the widget.
- **Widget values:** typed values beyond parts (gaps, alignment, order, behaviour, presentation transitions by event,
  placement, breakpoints), each with its type. A presentation-transition list is `presentations`
  (`Vec<PresentationTransition>`, S2.7), keyed by event; an event beyond `Appear`, `Disappear` and `Move` is a keyed
  event (`@event`). A value of type `Placement` is `{ strategy: PlacementStrategy, params: PlacementParams }` (S13,
  ADD-A21, ADD-A22). Every widget's "when focus becomes visible" value is `focus_visible_when: FocusVisibleWhen` (ADD-A16),
  whatever variant list a group wrote for it.
- **Variants:** typed where two or more platforms share the meaning (R15), keyed (`@name`) per platform otherwise.
- **Sizes and densities** per platform; **widget-specific flags** (`@name` keyed, `StateFlag::Widget(Key)`; an
  unprefixed widget flag is a typed one, ADD-A15); **exclusive groups** with their precedence (R3).
- **may_be_absent:** the parts or values some platform does not draw (S6.3).
- **State-model source** per platform: where the gate's combination list comes from (R4).
- **Today's fields:** how each field of `docs/property-registry.toml` maps into the new parts.

Names: a widget value is named without its unit suffix where a group wrote none; the field's Rust and TOML name
carries `_px` or `_ms` where its type is a plain length or duration (S1), so `show_delay` and `show_delay_ms` are one
field. Group-local labels of core additions are rewritten to the merged IDs.

## Selector notation

The groups wrote selectors in the traceability (and in "Today's fields") with one notation, varying in spelling:

| Spelling | Meaning (S7) |
|---|---|
| `@ {a,b}` | the element's states: all of them hold together |
| `!{a}`, `not {a}` | none of these states |
| `variant v`, `variants {v,w}`, `!variant v`, `widget[v]` | the element's (or the named ancestor's) variants; `@v` is keyed |
| `cond c`, `condition c`, `!cond c`, `flavour kirigami` | a condition (S7.1): `window_inactive`, `contrast_high`, `scheme:dark`, `flavour:kirigami`, `system_setting:<key>` (ADD-A04), … |
| `context w.part`, `ctx any W`, `context any(W.part {states})` | `ContextMatch` with `AnyAncestor` |
| `ctx nearest{A,B}=A`, `context nearest of {A, B} is A` | `ContextMatch` with `NearestAmong` |
| `pos first`, `position first`, `pos !first`, `prev x{…}`, `next x{…}` | `PositionMatch`; `position parity odd` is ADD-B06 |
| `size s`, `density d`, `size_class c`, `h_size_class c`, `v_size_class c` | `SizeMatch` (per axis: ADD-A20) |
| `dir rtl`, `direction rtl` | `Direction` |
| `/` between alternatives | one rule per alternative |

## Rename table

Each row is one thing two or more groups named differently, or a name that collided with another thing; the
traceability uses the new names. Decided by the rationale's rules: R14 (typed if two or more platforms draw it,
counting the cross-check decisions; keyed otherwise), R15 (typed variants where two or more platforms share the
meaning), principle 2 (one mechanism, one name per thing), and the owning group's name where the kinds are the same.

| # | Old name (group) | New name | Reason |
|---:|---|---|---|
| 1 | `dialog.platter` (7) | `dialog.body` | the action sheet's box is the dialog box group 1 owns |
| 2 | `dialog.values.screen_edge_margin` (7) | `dialog.values.window_margin` | group 1's value for the same margin |
| 3 | `dialog` variant `action_sheet` (7) | `@action_sheet` | iOS alone: keyed |
| 4 | `dialog` variant `full_screen`, parts `header_title`, `action_bar`, `divider` (6) | variant `full_size`; `title`, `action_area`, `header.stroke` | group 1's names: ContentDialog's full size, Material's full-screen dialog |
| 5 | `dialog.values.button_gap` (6) | `dialog.values.action_layout.row_gap` | group 1's (today's `button_gap`) |
| 6 | `window.values.caption_button_side` (8) | `window.values.caption_button_layout` | group 1's value holds side and order |
| 7 | `window.tab_bar.values.presentation` (7) | `window.values.presentations`, event `Move` | presentations are widget values |
| 8 | `window` variant `@utility` (6) | typed `utility` | group 1 typed it (Windows' tool window, macOS's utility panel) |
| 9 | `navigation_bar.back_button` (7) | `button` variant `back` in `context toolbar.bar`; values `back_title_fallback`, `back_display_mode`; toolbar event `@back_label` | group 1 typed the back button; group 6 places iOS's navigation bar inside `toolbar` |
| 10 | `navigation_bar` for Material's floating toolbar (7) | `toolbar` variant `@floating`, parts `leading_content`, `trailing_content` | group 6: app bars are `toolbar` |
| 11 | `scroll_edge_effect` (7), `scroll_area` (8) | `scroll_view.edge_effect`; variants `soft_edge`, `hard_edge`; values `edge_effect_height`, `edge_effect_edges` | group 3 typed `scroll_view` with this part (iOS, macOS, GNOME's undershoot) |
| 12 | `writing_tools.glow`, `writing_tools.affordance` (3) | `text_area.writing_tools_glow` (group 8's name), `input.writing_tools_affordance` | typed parts of the text widgets (macOS and iOS draw them in the app's text view); C-33's panel stays out (R1.1), its glow is carried |
| 13 | `drag_preview` (8), `list.values.drag_image` (4) | `drag_image` (7); `drag_preview.body` → `drag_image.image`; the list value → `drag_image.image.opacity`, `drag_image.values.hotspot` in `context list` | one widget for the dragged image |
| 14 | `placeholder` widget (8) | `redaction`, part `mask` | `placeholder` is the text widgets' part |
| 15 | `list.rubberband` (4) | `rubber_band.body` in `context list` | group 6 typed `rubber_band` |
| 16 | `navigation_view.transition_shadow`, `navigation_view.dimming`, `sheet.dimming`, `sidebar.dimming`, `sidebar.transition_shadow` (6); `sheet.grabber`, `drawer.handle` (8) | `edge_shadow`, `scrim`, `sheet.handle`, `sidebar.handle` | one name per part kind: `scrim` as `dialog.scrim` and `role.global.scrim`, `edge_shadow` and `handle` as group 4's sidebar |
| 17 | `sidebar.body`, `sidebar.divider`, `sidebar.item`, `sidebar.item_icon`, `sidebar.values.icon_gap` (6) | `sidebar.pane`, `sidebar.separator`, `list.row`, `list.leading_icon` (variant `sidebar`), `list.values.leading_gap` | group 4: a sidebar's rows are `list` rows |
| 18 | `sidebar` variant `rail`, parts `container`, `active_indicator`, `icon`, `label`, `menu_button`, `badge` (7) | typed `compact`; `pane`, `list.row_selection_indicator`, `list.leading_icon`, `list.row` text, `toggle_button`, `badge` in context | group 4's collapsed rail |
| 19 | `stack` (8) | `navigation_view` | the page stack (GtkStack) |
| 20 | `spin_button` (8) | `stepper` | group 6 |
| 21 | `main_window.dock`, `main_window.values` (8) | `component.dock_widget` | KDE alone: group 7's component |
| 22 | widget `frame` (6), `group_box` (4) | `group_box` | one widget; `frame` is a part of `window`, `scroll_view`, `calendar` |
| 23 | context `button_group` (2) | `layout` keyed variant `@linked`, `values.item_gap` | GTK's `.linked` box; no button-group widget |
| 24 | `checkbox.group_header`, `checkbox.values.row_spacing`, `column_spacing` (7) | `radio_group.header`, `radio_group.values.row_gap`, `column_gap` | group 2 typed `radio_group` (Windows, macOS) |
| 25 | `button` typed `command_link`, parts `title`, `description` (7) | keyed KDE `@command_link`; `label`, `subtitle` | Windows' command link is Win32 (R1.3), so one platform draws it |
| 26 | `button` variants `paste_control`, `repeat` (7) | `@paste_control` (iOS), `@repeat` (Windows) | one platform each: keyed |
| 27 | `button` keyed `@help` (2) | typed `help` | macOS and KDE draw it (group 6) |
| 28 | `button` keyed `@disclosure`, `@push_disclosure` (2) | `expander` (its `arrow`; variant `@push_disclosure`) | group 4's expander carries the disclosure controls |
| 29 | `button` variants `icon`, `filled`, `tonal`, `outlined` (6) | `icon_only`, `primary`, `tinted`, `@outlined` | group 2's names for the same looks |
| 30 | variant `suggested` (4, 6) | `primary` | group 2's name for the suggested look |
| 31 | `input` variant `password` (6) | `secure` | group 3's |
| 32 | `tab` variant `navigation`, parts `item`, `item_icon`, `item_label`, `indicator` (6) | typed `bottom_tab_bar` | KDE, Material, iOS, macOS draw it; parts and state model are not the tab's |
| 33 | `bottom_tab_bar.selection_pill` (4) | `bottom_tab_bar.indicator` | as `tab.indicator`, `segmented_control.indicator` |
| 34 | `tab.body` (8) | `tab.tab` | group 4's part |
| 35 | `list.item`, `list.item.fill` (7, 8) | `list.row`; state fills on `list.row_background` | group 4's parts |
| 36 | `list.body`, `selection_indicator`, `item_selection_visual`, `row_expander`, `branch_arrow`, `multi_select_check_box`, `grid_item`, `grid_item_check_box`, `item_inner_border` (7, 8) | `view`, `row_selection_indicator`, `disclosure`, `selection_check`, `row_background` under the typed `grid`, `selection_check` under `grid`, `row_inner_border` | group 4's parts and variant |
| 37 | `list` variants `listbox`, `@alternating` (7, 8) | `@listbox` (Windows), `alternating_rows` | keyed; group 4's typed variant |
| 38 | `spinner.container`, variants `loading_indicator`, `contained` (7); `spinner.glyph` (8) | `spinner.body`; `@loading_indicator`, `@contained`; `spinner.icon.glyph` | group 3's parts; Material alone: keyed |
| 39 | `progress_bar` variant `circular` (6) | `spinner` variant `determinate` | group 3 put the determinate circular indicator in `spinner` |
| 40 | `expander.glyph` (8) | `expander.arrow` | group 4's |
| 41 | `separator.body` (8) | `separator.line` | group 4's |
| 42 | `switch.on_off_label` (8) | `switch.on_glyph`, `switch.off_glyph` | group 2's |
| 43 | `scrollbar` variant `annotated`, part `scroll_button` (7) | `@annotated`; `arrow_button` | keyed (Windows); group 3's part |
| 44 | `input.loupe`, `inline_prediction`, `spelling_mark` (8) | `magnifier`, `prediction`, `misspelling` | group 3's |
| 45 | `text_area.find_indicator` (8) | `text_area.find_match @ {selected}` | group 3: the current match is `{selected}` |
| 46 | `input.header_inactive` (1), `input.kde_header_inactive` (4), `input.inactive_roles` (8) | `input.inactive.<set>` | KDE's stated Inactive values, one S9 input |
| 47 | `popover.values.anchor_gap` (3) | `popover.values.placement` (its `gap`) | group 1's placement value |
| 48 | `toast.values.duration_ms` (8) | `toast.values.durations` | group 6's keyed list |
| 49 | `combo_box.values.placement` (1) | `combo_box.values.popup_placement` | group 3's |
| 50 | `component.side_tab_bar` (4) | `component.multi_tab_bar` | group 7's (KMultiTabBar) |
| 51 | `toolbar_customization` (4), keyed `component.toolbar_editor` (6) | typed `toolbar_editor` | KDE, macOS and iOS draw it |
| 52 | `component.token_field.token`, `token_menu_arrow`, `values.token_spacing` (7) | `chip` in `context component.token_field`, `chip.menu_arrow`, `component.token_field.values.chip_gap` | a token is a chip (group 7's own note) |
| 53 | the glass container's merge spacing as `Material.params` (8) | `glass_container.values.merge_gap` | group 7's widget value |
| 54 | `window.content.fill` with the material `background_extension` (8, MAC-X06) | `background_extension.extension` | group 7's widget |
| 55 | `token.lengths` (4), `token.kirigami` (2), `token.opacities` (5), `token.colour_effects.frame_contrast` (2) | `token.params.layout`, `token.params.opacity`, `token.params.colour.frame_contrast` | one keyed-scalar token mechanism (ADD-A44) |
| 56 | value `focus_visible`, `token.behaviour.focus_visible`; types `FocusVisiblePolicy`, `FocusVisible` (2, 3, 8) | `focus_visible_when`, `token.behaviour.focus_visible_when`; `FocusVisibleWhen` | the bare name is the state flag (ADD-A16) |
| 57 | `role.keyed.<x>` (8) | `role.@<x>` | S2.2's grammar |
| 58 | `role.@palette_dark`, `role.@palette_shadow` (7), `role.<set>.shade.light` (8) | `role.@shade_dark`, `role.@shade_shadow`, `role.@shade_light` | group 5's KDE keyed shades (Qt's 3-D roles, from Window) |
| 59 | `active_background`, `link_background`, `visited_background`, `negative_background`, `neutral_background`, `positive_background` (8) | `background_active`, … | group 5's record fields |
| 60 | `role.global.selection_text_inactive` (8) | `role.global.selection_text` under `cond window_inactive` | the inactive group is a rule (S3.2, ADD-A48) |
| 61 | `role.selection_background_inactive`, `role.find_highlight`, `role.drop_highlight` (3, 7, 8) | `role.global.…` | global roles (S3) |
| 62 | condition `BehindModal` (8) | `TintDimmed` | group 5's, UIKit's own name (ADD-A10) |
| 63 | `IconStyle.symbol_variant` (4) | `IconStyle.variant` | group 8's (ADD-A37) |
| 64 | `values.presentation` (1, 4, 7, 8) | `values.presentations` | the list of presentation transitions |
| 65 | `icon_gap` (2, 6), `icon_label_gap` (4), `item_icon_gap` (4) | `icon_text_gap` | today's name, used by groups 1 and 3 |
| 66 | `auto_repeat_delay`, `auto_repeat_interval` (2), `autorepeat_*` (6) | `repeat_delay`, `repeat_interval` | groups 3, 7, 8 |
| 67 | `spring_load_delay_ms` (7), `drag_expand_delay_ms` (8); `drag_hover_switch_ms` (6) | `drag_hover_expand_delay_ms`; `drag_hover_switch_delay_ms` | group 4's |
| 68 | widget values `*_spacing`, `spacing` (2, 4, 6, 7, 8) | `*_gap`, `gap` | R16's word; `letter_spacing` (a text-style field), the variant `@collapse_spacing` and Kirigami's spacing token keys keep theirs |
| 69 | conditions written as states, `{reduced_motion}` (8) | `cond reduced_motion` | notation: a condition is not a state flag |

## Where group 6's §4.2 kinds go (its index, reconciled)

| §4.2 kind | Platforms (rows) | Decision |
|---|---|---|
| badge | all six | typed `badge` |
| autocomplete | GNOME, KDE, Material, iOS, Windows (CTK) | typed `autocomplete` |
| date picker | macOS, Windows, KDE, Material, iOS | typed `date_picker` |
| calendar | Windows, KDE, GNOME, iOS (+ Material, macOS inside their date pickers) | typed `calendar` |
| level indicator/gauge | GNOME, macOS, iOS, Windows (CTK) | typed `level_indicator`; KDE's LCD number and LED are one-platform kinds: keyed components `lcd_number`, `led`; iOS C-31 (determinate circular progress) is drawn inside `spinner` (its typed `determinate`; group 6 wrote `progress_bar`) |
| dial/circular slider | macOS, KDE | typed `dial` |
| menu bar | macOS, Windows, KDE, GNOME, iOS | typed `menu_bar`; macOS's Help-menu search is drawn inside `menu` |
| navigation/app bar | GNOME, KDE, Material, iOS, Windows | split by what is drawn: the page stack's push/pop is typed `navigation_view` (GNOME, KDE); bottom and Expressive bars and iOS's navigation bar are drawn inside `toolbar`; Windows' top NavigationView is a `view_switcher`; KDE's KPageView is a keyed component `page_view` |
| pull-to-refresh | macOS (27), Windows, KDE, Material, iOS | typed `pull_to_refresh` |
| rich text view | KDE, macOS, iOS, Material (Windows' MarkdownTextBlock is Toolkit Labs: out, R1.2) | typed `rich_text` |
| search field | macOS, Windows, GNOME, Material, iOS | typed `search_field` |
| stepper/spin box | macOS, Windows, KDE, GNOME, iOS | typed `stepper` |
| about dialog | GNOME, KDE, macOS (Windows' `ShellAbout`: out, R1.1) | typed `about_dialog` |
| banner/inline message | Windows, KDE, GNOME, Material | typed `banner` |
| bottom tab bar | macOS, KDE, Material, iOS | typed `bottom_tab_bar` (the merge: its parts and state model are not the tab's) |
| breadcrumb/path control | macOS, Windows, KDE, GNOME | typed `breadcrumb` |
| carousel | Windows, GNOME, Material, iOS | typed `carousel` |
| colour button | macOS, Windows (CTK), KDE, GNOME, iOS (C-8) | typed `colour_button` |
| colour picker | Windows, KDE, GNOME | typed `colour_picker` |
| media transport controls | macOS, Windows, GNOME, iOS | typed `media_controls` |
| page control/pager | Windows, KDE, GNOME, iOS | typed `page_indicator` |
| preference rows/boxed list | GNOME, KDE, macOS, Windows (CTK) | typed `form`; KDE's list section header and inline view header are drawn inside `list` |
| sheet/bottom sheet | KDE, GNOME, Material, iOS | typed `sheet` |
| split button | macOS, Windows, GNOME, Material | typed `split_button` |
| status page/placeholder | macOS, KDE, GNOME, iOS | typed `status_page` |
| teaching tip | macOS, Windows, KDE, iOS | typed `teaching_tip` |
| text input system UI | all four rows | out (R1.1: an input method's own window, OS-drawn popups) |
| avatar | Windows, KDE, GNOME | typed `avatar` |
| chart | macOS, KDE, iOS (Swift Charts, KQuickCharts: R1.2) | typed `chart` |
| chip | Windows (CTK), KDE, Material | typed `chip` |
| edit menu | KDE, Material, iOS | typed `edit_menu` |
| file chooser | GNOME, KDE (GTK's own chooser, KFileWidget); GNOME's portal chooser and Windows' Common Item Dialog are out (R1.1) | typed `file_chooser` |
| find bar | macOS, KDE, iOS | typed `find_bar` |
| font chooser | GNOME, KDE (Win32 `ChooseFont`: out, R1.1) | typed `font_chooser` |
| group box/frame, image frame | GNOME, iOS, KDE, macOS | one typed widget `group_box` (group 6 wrote `frame`; a frame with an optional title; the image-view frame styles are its variants) |
| menu button | macOS, Windows, Material | drawn inside `button` (the typed variant `menu` and its `menu_indicator`) |
| rubber band | macOS, KDE, iOS | typed `rubber_band` |
| shortcuts dialog | GNOME, KDE (iOS's discoverability HUD: out, R1.1) | typed `shortcuts_dialog` |
| time picker | Windows, KDE, Material | typed `time_picker` |
| toast/snackbar | KDE, GNOME, Material | typed `toast` |
| wizard | GNOME, KDE (Windows' Aero wizard is Win32: preset, R1.3) | typed `wizard` |
| button group/toggle group | KDE (Q2.3 is a DelayButton), Material | Material's button group is drawn inside `segmented_control`; KDE's DelayButton is a one-platform kind: keyed component `delay_button` |
| composite dialogs | GNOME, KDE | out (R1.1: print, page setup, app chooser, Open With are dialogs a toolkit only invokes); GtkDialog's action area is `dialog`'s |
| editable label | GNOME, iOS | drawn inside `input` (GNOME's cross-check: GN-2.45 belongs to `[input]`) |
| emoji chooser | GNOME (KDE's plasma-emojier is a separate Plasma program: out, R1.1) | keyed component `emoji_chooser` |
| FAB | KDE (RoundButton), Material | drawn inside `button` (a round button and Material's FAB styles are button variants); Material's FAB menu is a keyed component `fab_menu` |
| full-screen dialog | KDE, Material | drawn inside `dialog` |
| help button | macOS, KDE | drawn inside `button` |
| icon button | Material, iOS | drawn inside `button` |
| image cropper | macOS (ImageKit), Windows (CTK) | typed `image_cropper` |
| ink toolbar | Windows, iOS | typed `ink_toolbar` |
| inspector/utility panel | macOS, iOS | drawn inside `window` (HUD and utility panels) and `sidebar` (the trailing inspector column) |
| navigation drawer/pane | GNOME, Windows | drawn inside `sidebar`; GtkAssistant's step sidebar is `wizard`'s |
| password field | Windows, GNOME | drawn inside `input` (the typed variant `secure`) |
| picker wheel | Material, iOS (and Windows' looping selectors, W2-04) | typed `picker_wheel` |
| rating | Windows, KDE (and macOS's rating level indicator) | typed `rating` |
| text ruler | macOS, KDE | typed `ruler` |
| toolbar customization | KDE (Windows' is Win32: preset, R1.3) | typed `toolbar_editor` (the merge: KDE, macOS and iOS draw it, group 4's rows) |
| view switcher | Windows, GNOME | typed `view_switcher`; GNOME's tab overview is a keyed component `tab_overview` |
| wrap box | GNOME, Material (chip set) | typed `wrap_box` |

## Window and chrome

### window

*From group 1's catalogue.*

- **Kind:** typed widget (every platform draws a window; its chrome is drawn by the toolkit where it draws its
  own frame, R1.1, R13).
- **Parts:**
  - `frame` — the window's outer box: `fill` (the decoration's border band: KDE's Frame role, GNOME's
    non-compositing frame), `stroke` (the outline: GNOME's inner outline `Inside`, KDE's outline
    `Outside`, macOS's outer edge line, the DWM border), `corners` (per corner; `Continuous` on macOS; the
    outermost container of `Radius::Concentric`, measured per device class on iOS, as group 8 records),
    `shadow` (layers, or an elevation entry; GNOME's outer ring and macOS's inner light rim are layers,
    the rim `inset`), `padding` (the border band's width per side: KDE's border size, GNOME's `.solid-csd`),
    `transitions` (the activation shadow fade).
  - `content` — the client area: `fill` (window background; Mica, Mica Alt, the AppKit window material,
    group 8's `background_extension`), `bindings` (`Surface(Window)`), `transitions` (Mica's fade to its
    inactive fallback).
  - `title_bar` — the bar: `fill` (colour, gradient, material), `stroke` and `shadow` (GNOME's bottom shade line
    and side borders as inset layers, its drop shadow; macOS's separator line or shadow), `min_height`,
    `bindings` (`Surface(Header)`), `transitions`.
  - `title_bar_content` — the box holding title and buttons: `padding`, `opacity` (the inactive dimming of
    everything inside it, as one group), `transitions`.
  - `title` — `text` (font, fill, elision), `padding`, `transitions`.
  - `subtitle` — `text`, `padding`, `opacity`.
  - `title_icon` — the application icon in the title bar, or macOS's proxy (document) icon: `icon`,
    `padding`.
  - `title_status` — text the toolkit appends to the title for a window state (macOS's "Edited"): `text`.
  - `title_chevron` — macOS's document-title chevron shown on hover: `glyph`, `opacity`, `transitions`.
  - `caption_buttons` — the group of caption buttons: `padding` (leading inset, vertical position); a rule's
    context for group hover (macOS shows all three glyphs while the group is hovered).
  - `caption_button` *(element)* — one caption button; its identity is a variant (below): `fill` (WinUI's
    rectangle), `min_width`, `min_height`, `padding`, `corners`, `transitions`.
  - `caption_button_glyph` — the glyph and the shape behind it (GNOME's circle, KDE's hover circle, macOS's
    traffic light): `glyph` (source per identity: Segoe Fluent Icons code points, Breeze's paths, theme icons),
    `icon` (KDE's window-menu button shows the window icon recoloured), `fill`, `stroke` (the rim), `corners`
    (`Full`), `padding`, `min_width`, `min_height`, `opacity`, `transitions`.
  - `window_menu` — the anchor part of the window menu, which is a `menu` element whose ancestor is this
    part: `bindings` (KDE draws it in the window's own decoration palette, K-P4-1†).
  - `resize_zone` *(element)* — one per edge and corner (identity by variant): `cursor`.
  - `tab_bar`, `tab` *(element)*, `tab_title`, `tab_close_button` *(element)*, `new_tab_button`
    *(element)*, `tab_separator` — macOS's native window tabs (MAC-W09; group 7 adds `tab_bar` under
    `{drop_target}`, MAC-SF07†).
  - `launch_screen` — the launch or splash screen the system draws from the application's values:
    `fill`, `bindings`.
  - `launch_icon` — the splash icon: `icon` (size), `fill` (Android's icon background).
- **Widget values:**
  - `caption_button_layout: CaptionLayout { leading: Vec<CaptionItem>, trailing: Vec<CaptionItem> }`, with
    `CaptionItem { Button(Key), Spacer, Icon }`: which buttons, on which side, in which order (read on GNOME
    and KDE; group 8's `window.values.caption_button_side`, W3-54†, is this value; mirrored by a `dir rtl`
    rule).
  - `caption_button_gap_px: Value<f32>`; `caption_spacer_width_px: Value<f32>`.
  - `caption_button_capabilities: Vec<(Key, WindowCapability, WhenMissing)>`, `WindowCapability { Closable,
    Minimizable, Maximizable, Resizable, Sovereign, ProvidesHelp, Shadeable }`, `WhenMissing { Hide,
    Disable }`: GNOME hides the minimise button of a transient window, Windows greys the maximise button of a
    fixed-size one.
  - `caption_button_menus: Vec<(Key, Vec<Key>)>` and `caption_button_menu_placement: Placement`: the
    menu a caption button opens (macOS's zoom-button tiling menu); its look is `menu` in `context
    window.caption_button`.
  - `title_alignment: Value<TitleAlignment>`, `TitleAlignment { Start, Center, CenterFullWidth, End }`;
    `CenterFullWidth` centres on the whole bar and falls back to the free side when it meets the buttons (KDE).
  - `title_margin_px`, `title_bar_box_gap_px`, `title_icon_gap_px`, `title_subtitle_gap_px: Value<f32>`.
  - `title_bar_click_actions: TitleBarActions { double_click, middle_click, right_click: Value<Key> }` (read).
  - `window_menu_items: Vec<Key>` (the actions in order, with separators); `window_menu_placement:
    Placement` (at the pointer; below a button's rectangle); `title_menu_placement: Placement` (macOS's title
    pop-up menu).
  - `resize_band: ResizeBand { outside: Sides<Value<f32>>, inside: Sides<Value<f32>>, corner_px: Value<f32>,
    includes_shadow: Value<bool> }`.
  - `full_screen_title_bar_reveal: Value<RevealTrigger>`, `RevealTrigger { None, PointerAtTopEdge,
    SwipeFromTop }`.
  - `status_bar_style: Value<StatusBarStyle>`, `StatusBarStyle { Default, LightContent, DarkContent }` (iOS's
    status bar style, Android's system-bar icon tone: values the application configures for the system);
    `navigation_bar_contrast_scrim: Value<bool>` (Android).
  - `default_size: (Value<f32>, Value<f32>)` (Kirigami's window, in grid units); `mobile_height_breakpoint_px`
    (group 4's GN-1.197).
  - `attention_flash: AttentionFlash { period_ms: Value<f32>, count: Value<f32> }` (Windows).
  - `tab_bar_overflow: Value<Key>` (macOS; the kind is to measure, MAC-P4-5†).
  - `presentations`: keyed events `@title_bar_reveal` (full-screen reveal), `@launch_exit` (splash into the
    first frame), `@predictive_back` (Material's in-app back gesture), `Move` (group 7's tab parting).
- **Variants:**
  - on the window, typed: `dialog` (a dialog's window: GNOME's message window shadow, the close-only caption
    of GNOME and Windows, macOS alerts and panels), `utility` (Windows' tool window, macOS's utility panel:
    small caption and buttons, own radius and shadow), `transparent_title_bar` (GNOME's `.flat` header bar,
    macOS's `titlebarAppearsTransparent`).
  - keyed: GNOME `@ssd`, `@ssd_frame`, `@default_decoration`, `@standalone`, `@collapse_spacing`, `@inline`,
    `@devel`; Windows `@tall`, `@collapsed`, `@titlebar_control_expanded`, `@mica_alt`, `@round_small`,
    `@compact_overlay`; macOS `@hud`, `@title_hidden`, `@no_toolbar`, `@toolbar_unified`,
    `@toolbar_unified_compact`, `@toolbar_expanded`, `@toolbar_preference`, `@separator_line`,
    `@separator_shadow`.
  - on `caption_button` (its identity), typed: `close`, `minimize`, `maximize`, `restore`, `help` (KDE,
    Windows), `window_menu` (KDE's menu button, GNOME's `appmenu`); keyed KDE `@on_all_desktops`,
    `@keep_above`, `@keep_below`, `@shade`, `@exclude_from_capture`, `@application_menu`.
  - on `resize_zone`, typed: `top`, `right`, `bottom`, `left`, `top_left`, `top_right`, `bottom_right`,
    `bottom_left`.
- **Sizes and densities:** none; the title-bar heights are variants (Windows' height option, GNOME's
  decoration kinds, macOS's toolbar styles); KDE's button and border sizes are read settings.
- **Conditions used:** `window_inactive`, `not_main` and `app_inactive` (macOS), `maximized`, `full_screen`,
  `tiled` (per edge; maximised on one axis counts as tiled to its edges), `contrast_high`,
  `reduced_transparency`, `scheme:dark`, `idiom:phone|pad` (group 8), `mobile_form_factor`,
  `system_setting:<key>` (KDE's breezerc switches, Windows' accent on title bars, macOS's wallpaper tinting),
  and this group's `not_composited`, `shaded`, `attention_flash` (ADD-A01, ADD-A02, ADD-A03).
- **Widget-specific flags:** typed `hover`, `pressed`, `checked` (KDE's toggled caption buttons), `disabled`,
  `scrolled` (macOS's separator shown while content scrolls beneath, on `title_bar`); keyed macOS
  `@document_edited` (on the window: the close button's dot and the title's "Edited").
- **Exclusive groups:** Windows caption buttons `[disabled, pressed, hover]`.
- **may_be_absent:** `frame.shadow` (maximised, full screen, non-compositing), `frame.stroke`, `frame.padding`,
  `title_bar` (KDE's `HideTitleBar`, full screen, `@title_hidden`), `subtitle`, `title_icon`, `title_status`,
  `title_chevron` (all but macOS), `caption_buttons.padding`, `window_menu` (iOS, Material), `resize_zone` and
  `resize_band` (iOS, Material; maximised), the `tab_*` parts (all but macOS), `launch_screen` and
  `launch_icon` (GNOME, KDE, macOS), `status_bar_style` and `navigation_bar_contrast_scrim` (all but iOS and
  Material), `attention_flash` (all but Windows), `default_size` (all but Kirigami), `tab_bar_overflow`.
- **State-model source:**
  - Windows: the DWM caption buttons' states as `AppWindowTitleBar` names them (rest, hover, pressed,
    inactive; per button, with close's own), WinUI's `WindowCaptionButton` style (`generic.xaml:15697`) and the
    `TitleBar` control's visual states (`TitleBar/TitleBar.xaml`, activation and display states).
  - GNOME: libadwaita's `_window.scss` (backdrop, tiled, maximized, fullscreen, `.solid-csd`, `.ssd`,
    `.ssd-frame`, HC) and `_header-bar.scss` (backdrop, `.flat`, `.default-decoration`, `.collapse-spacing`,
    window controls :hover/:active), GtkWindowControls' per-window rules (`gtkwindowcontrols.c`).
  - KDE: Breeze's decoration branches (`breezedecoration.cpp`: active, maximised, tiled per edge, shaded,
    border size) and button branches (`breezebutton.cpp`: hovered, pressed, checked, active).
  - macOS: `NSWindow` key and main state, `styleMask`, `toolbarStyle`, `titlebarSeparatorStyle`,
    `titleVisibility`, `titlebarAppearsTransparent`, the standard buttons' states (group rollover, pressed,
    disabled, document edited).
  - iOS: the scene's `activeAppearance` trait; `UILaunchScreen` keys; the status bar style enum.
  - Material: none for in-window chrome (the system draws the frame); `WindowInsetsController` appearance
    and the splash-screen theme attributes.
- **Today's fields:** `background_color` → `content.fill`; `title_bar_background` → `title_bar.fill`;
  `title_bar_font` → `title.text.font` (its colour → `title.text.fill`); `inactive_title_bar_background` →
  `title_bar.fill @ cond window_inactive`; `inactive_title_bar_text_color` → `title.text.fill @ cond
  window_inactive`; `border.color`, `border.line_width_px` → `frame.stroke`; `border.corner_radius_px` →
  `frame.corners`; `border.shadow_enabled` → `frame.shadow` (no layers is no shadow, R8); `border.padding_*` →
  `frame.padding`.

- **Merged from other groups and rows:**
  - Group 6: keyed macOS `@hud` (the HUD window's material, title bar and close button, MAC-N14); MAC-N16's utility panel is the typed `utility` above (shorter title bar, small caption buttons, title font).
  - Group 7: `tab_bar` under `{drop_target}` (insertion feedback) and the presentation event `Move` (tabs parting), MAC-SF07†.
  - Group 8: `title_icon` under `cond system_setting:title_icons` (MAC-X31†); `content.fill` with the platform's material; `values.keyboard_avoidance: PlacementStrategy` (`KeyboardAvoid(…)`, W-P4-3†, MAT-P4-2†); `frame.corners` as the outermost `Radius::Concentric` container, measured per `cond idiom` (X-CR-4; per-device values are ADD-B18).
  - Group 4: GN-1.197's header-bar breakpoint is `values.mobile_height_breakpoint`.
  - kirigami-addons: keyed KDE `@ka_settings` (the settings window: `values.default_size`, `content.min_width`, `content.min_height`, K-KA-41†).

## Overlays

### dialog

*From group 1's catalogue.*

- **Kind:** typed widget (every platform: AdwDialog and AdwAlertDialog, QDialog, KMessageBox and Kirigami's
  dialogs, NSAlert and sheets, UIAlertController, Material's dialog, ContentDialog).
- **Parts:**
  - `scrim` — the dimming behind the dialog: `fill` (`= role.global.scrim`, or libadwaita's derivation from
    the shade colour), `transitions`.
  - `body` — the dialog box (group 7 calls it `platter` for the iOS action sheet): `fill` (colour or material),
    `stroke` (GNOME's inner outline, Kirigami's border), `corners`, `shadow`, `padding`, `min_width`,
    `max_width`, `min_height`, `max_height`, `bindings` (`Surface(Dialog)`; Kirigami's View set; iOS's
    elevated roles).
  - `header` — the title area: `padding`, `fill` and `bindings` (KDE's Header-filled tools area), `stroke`
    (the separator under it, shown while the content overflows).
  - `title` — `text`, `padding`.
  - `subtitle` — `text` (Material's subhead, Kirigami's prompt subtitle).
  - `message` — the body text and content area: `text`, `padding`, `corners` (GNOME's scrolled alert body).
  - `icon` — the hero or standard icon: `icon`, `glyph`, `padding`.
  - `icon_badge` — macOS's caution badge over the application icon of a critical alert: `icon`, `padding`.
  - `close_button` *(element)* — Kirigami's header close button: `glyph` (its icon changes on hover), `fill`.
  - `action_area` — the footer or command area: `fill` (WinUI's command area), `stroke` (the line above it:
    WinUI's separator, Kirigami's and Material's when the content overflows), `padding`, `min_height`.
  - `action` *(element)* — one response or action button (GNOME's response buttons, iOS's alert actions,
    Material's text actions, ContentDialog's buttons, KDE's button box): `fill` (and appended state layers),
    `stroke`, `corners`, `padding`, `min_width`, `max_width`, `min_height`, `text`, `icon`, `transitions`. It is
    the dialog's own element, not a `button`, because its geometry and roles are the dialog's; a preset whose
    platform uses its button look states it with the same roles (no model inheritance, R20).
  - `separator` — the hairlines between action cells (iOS 13–18; group 7's action sheet).
  - `cancel_platter` — the action sheet's separate cancel box (group 7, C-11).
- **Widget values:**
  - `title_alignment`, `message_alignment: Value<Alignment>`; `title_message_gap_px`,
    `message_actions_gap_px: Value<f32>`.
  - `action_layout: ActionLayout { row_order: Vec<ActionRole>, stacked_order: StackedOrder, alignment:
    Value<Alignment>, equal_widths: Value<bool>, row_gap_px: Value<f32>, column_gap_px: Value<f32>,
    stack_rule: StackRule }`; `ActionRole { Accept, Reject, Destructive, Help, Reset, Apply, Other }` (Qt's
    button roles, which KDE's `KdeLayout` orders); `StackedOrder { SameAsRow, Reversed, AcceptFirst }`;
    `StackRule { Never, WhenOverflowing { horizontal_padding_px: Value<f32> }, ByCount { max_in_row: u32 } }`.
  - `presentation_mode: Value<DialogPresentation>`, `DialogPresentation { Floating, BottomSheet, Sheet,
    Window, FullSize }` (AdwDialog's window fallback, SwiftUI's choice on macOS).
  - `presentation_breakpoint: Breakpoint { max_width_px: Value<f32>, max_height_px: Value<f32> }` (AdwDialog's
    bottom sheet; Material's full-screen dialog).
  - `window_margin: EdgeMargin { min_px: Value<f32>, max_px: Value<f32>, from_window_px: Value<f32>,
    to_window_px: Value<f32> }` per axis: the margin to the host's edges, constant (Kirigami, Material) or
    growing with the window (AdwDialog's floating sheet); group 7's `screen_edge_margin` (C-11) is this value.
  - `natural_size: (Value<f32>, Value<f32>)` (AdwDialog's default, Kirigami's MenuDialog and SearchDialog).
  - `placement: Placement`; `keyboard_avoidance: PlacementStrategy` (group 8's `KeyboardAvoid(…)` family).
  - `presentations`: `Appear`, `Disappear`, keyed `@sheet_attach`, `@sheet_detach`.
  - `button_icons: Value<bool>` (KDE, read); `standard_button_icons: Vec<(ActionRole, Key)>` (icon names);
    `type_icons: Vec<(Key, Key)>` (Kirigami's prompt types to icon names).
  - `message_wrap: MessageWrap { wrap_above_screen_fraction: Value<f32>, sample_chars: Value<f32>,
    min_width_screen_fraction: Value<f32>, min_width_chars: Value<f32>, elide_above_screen_fraction:
    Value<f32>, scroll_above_screen_fraction: Value<f32> }` (KMessageBox; character counts need the operand
    of ADD-B17).
- **Variants:** typed `alert` (a message dialog: AdwAlertDialog, KMessageBox and Kirigami's PromptDialog,
  NSAlert, UIAlertController's alert, Material's basic dialog), `separate_window` (a dialog in its own window:
  AdwDialog's fallback, Qt dialogs, Kirigami's native-window popup type, macOS's app-modal alert), `full_size`
  (ContentDialog's `FullSizeDesired`, Material's full-screen dialog), `warning`, `information`, `critical`
  (NSAlert's styles, KMessageBox's standard icons, Kirigami's prompt types); keyed GNOME `@bottom_sheet`,
  `@short`, `@narrow`, `@wide`, `@message_dialog`; KDE `@question`, `@success`, `@menu_dialog`,
  `@search_dialog`, `@flat_footer`; macOS `@sheet`; iOS `@action_sheet` (group 7); Material `@with_icon`. On
  `action`: typed `primary` (GNOME's suggested response; group 2's name for the suggested look) and
  `destructive`, keyed iOS `@cancel`; the default action is the state `default`.
- **Sizes and densities:** none; AdwDialog's breakpoint classes are the keyed variants `@narrow`, `@short`
  (R16); the accessibility text sizes are the condition `accessibility_text_size` (ADD-A08).
- **Widget-specific flags:** typed `default`, `hover`, `pressed`, `focus_visible`, `disabled` (on `action`),
  `scrolled`, and `overflowing` (ADD-A13); keyed KDE `@has_toolbar`, `@scroll_view_content`.
- **Exclusive groups:** Windows `[disabled, pressed, hover]` on `action` (Button's CommonStates).
- **may_be_absent:** `scrim` (KDE Qt Widgets dialogs, macOS app-modal alerts), `header`, `subtitle`, `icon`,
  `icon_badge` (all but macOS), `close_button` (all but KDE), `separator`, `cancel_platter` (all but iOS),
  `action_area.fill`, `action_area.stroke`, `presentation_breakpoint` (all but GNOME and Material),
  `message_wrap` (all but KDE), `button_icons`, `standard_button_icons`, `type_icons` (all but KDE),
  `keyboard_avoidance`.
- **State-model source:** Windows: ContentDialog's visual state groups (`DialogShowingStates`,
  `DialogSizingStates`, `ButtonsVisibilityStates`, `DefaultButtonStates`; `ContentDialog_themeresources.xaml`)
  and Button's CommonStates; GNOME: `_dialogs.scss`, `_message-dialog.scss` (floating, bottom sheet, `.alert`,
  `.short`, `.narrow`, HC) and the response appearances (`adw-alert-dialog.c`); KDE: Kirigami's
  `Dialog.qml`, `PromptDialog.qml`, `MenuDialog.qml`, `SearchDialog.qml`, qqc2's `Dialog.qml` and
  `DialogButtonBox.qml`, KMessageBox; Material: the dialog tokens (`_md-comp-dialog.scss`, action states);
  macOS: `NSAlert.Style`, sheet against app-modal presentation; iOS: `UIAlertController.Style`,
  `UIAlertAction.Style`, `preferredAction`.
- **Today's fields:** `background_color` → `body.fill`; `body_font` → `message.text.font`; `min_width_px`,
  `max_width_px`, `min_height_px`, `max_height_px` → `body.min_width` …; `button_gap_px` →
  `values.action_layout.row_gap_px`; `button_order` → `values.action_layout.row_order`; `entrance` →
  `values.presentations` (event `Appear`); `title_font` → `title.text.font`; `icon_size_px` →
  `icon.icon.size`; `border.*` → `body.stroke`, `body.corners`, `body.padding`, `body.shadow`.

- **Merged from other groups and rows:**
  - Group 7: keyed iOS `@action_sheet` (C-11): the actions' box is `body`, the separate cancel box `cancel_platter`, the actions `action`, the hairlines `separator`, with `title` and `message`; `values.window_margin`; the presentation event `Appear` (kind `Slide` from the bottom); on iPad it is a popover (a context, not a part).
  - Group 6: typed `full_size` also covers Kirigami's full-screen dialog (Q2.58†) and Material's full-screen dialog (MAT-N26): `header`, `title`, `close_button`, `action_area`, the line under the header as `header.stroke`; Kirigami chooses an inset dialog, a full-window layer or a separate window by width (`values.presentation_mode`, `values.presentation_breakpoint`, under `cond mobile_form_factor`). GtkDialog's action area: `action_area.padding`, `values.action_layout.row_gap` (GN-2.57†).
  - kirigami-addons: keyed KDE `@ka_dialog` (the one surface of every addons dialog and popup: corners, View binding, stroke, shadow, K-KA-18†), `@ka_form_card_dialog` (K-KA-17†; widget flag `@has_cancel` shows the close button), `@ka_message_dialog` (K-KA-19†; the icon above the text under the connector-set `@narrow`), `@ka_list_dialog` (the combo row's list dialog, K-KA-12†).

### popover

*From group 1's catalogue.*

- **Kind:** typed widget (GtkPopover, qqc2 Popup, Breeze popups, NSPopover, UIPopoverPresentationController,
  WinUI Flyout, Material's menu container used as a popover surface).
- **Parts:**
  - `body` — `fill` (colour or material: acrylic, `.popover`, glass), `stroke`, `corners`, `shadow` (layers
    or an elevation entry; Material's surface tint is an appended fill layer, R8), `padding`, `min_width`,
    `max_width`, `min_height`, `max_height` (`Auto` where the platform sizes by content), `text` (today's
    `font`), `bindings` (`Surface(Popover)`; iOS's elevated roles), `transitions`.
  - `arrow` — the tail: `min_width` (base), `min_height` (height), `corners` (tip rounding), `fill`, `stroke`.
  - `dismiss_overlay` — WinUI's light-dismiss overlay: `fill`.
- **Widget values:** `placement: Placement` (gap to the anchor, screen-edge margin, arrow side);
  `has_arrow: Value<bool>`; `default_size: (Value<f32>, Value<f32>)` (UIKit's size when the app states none);
  `full_size_content: Value<bool>` (macOS); `presentations` (`Appear`, `Disappear`).
- **Variants:** keyed GNOME `@osd`, `@magnifier`; macOS `@detached`. (GNOME's touch-selection bubble is group
  6's `edit_menu`.)
- **Sizes and densities:** none.
- **Widget-specific flags:** none (a popover's content carries the states).
- **Exclusive groups:** none.
- **may_be_absent:** `arrow` (KDE, Windows, Material), `dismiss_overlay` (all but Windows), `body.shadow` (non-
  compositing), `full_size_content`, `default_size`.
- **State-model source:** GNOME `_popovers.scss` (`.menu`, `.osd`, `.touch-selection`, `.magnifier`,
  `:backdrop`, HC; `.solid-csd` ancestors); KDE Breeze's `renderMenuFrame` branches (alpha or not) and qqc2's
  `Popup.qml`; macOS `NSPopover.Behavior`, `animates`, detached; iOS `permittedArrowDirections`,
  `popoverLayoutMargins`; Windows FlyoutPresenter (no visual states beyond its theme resources); Material
  menu container tokens.
- **Today's fields:** `background_color` → `body.fill`; `font` → `body.text.font`; `border.*` →
  `body.stroke`, `body.corners`, `body.padding`, `body.shadow`.

- **Merged from other groups and rows:**
  - Group 3: the gap to the anchor is `values.placement`'s `gap` (Q1.16, MAC-I06).
  - kirigami-addons: keyed KDE `@ka_dialog` for addons' popups (K-KA-18†).

### tooltip

*From group 1's catalogue.*

- **Kind:** typed widget (GtkTooltip, QToolTip, qqc2 ToolTip, KToolTipWidget, NSToolTip and expansion tool
  tips, WinUI ToolTip, Material's plain and rich tooltips).
- **Parts:**
  - `body` — `fill` (colour or material: in-app acrylic, `.toolTip`), `stroke` (KDE's border mixed from
    background and text), `corners`, `shadow`, `padding`, `min_height` (Material), `max_width` (pixels, or
    characters by ADD-B17), `bindings` (`Surface(Tooltip)`, Kirigami's `Complementary`).
  - `label` — `text` (font, tracking, line height, wrap).
  - `title` — the rich tooltip's subhead: `text`.
  - `icon` — the tooltip's icon (GNOME): `icon`.
  - `accelerator` — the shortcut shown in a tooltip (group 8, W3-26): `text`.
  - `arrow` — Material's caret: `min_width`, `min_height`, `glyph` or `corners`, `fill`.
- **Widget values:** `show_delay_ms` (today), `reshow_delay_ms` (browse mode), `browse_timeout_ms`,
  `hide_delay_ms` (after the pointer leaves), `keyboard_show_delay_ms: Value<f32>`; `show_on_press:
  Value<bool>` (tablet mode, touch; with `show_delay_ms` = `token.behaviour.long_press_ms` under that
  condition, as group 8 states for Q3.28); `display_time: DisplayTime { base_ms: Value<f32>, per_char_ms:
  Value<f32>, free_chars: Value<f32> }`, `Absent` for "never times out while hovered" (`may_be_absent`);
  `icon_text_gap_px: Value<f32>`; `placement: Placement`; `presentations` (`Appear`, `Disappear`).
- **Variants:** typed `rich` (Material's rich tooltip, KDE's KToolTipWidget); keyed macOS `@expansion`.
- **Sizes and densities:** none.
- **Widget-specific flags:** none (a tooltip has no interaction state; the rich tooltip's actions are
  `button` elements in `context tooltip.body variant rich`).
- **Exclusive groups:** none.
- **may_be_absent:** `title` (all but Material and KDE's rich), `icon` (all but GNOME), `arrow` (all but
  Material), `accelerator` (all but Windows), `body.min_height`, `body.max_width` (KDE plain text),
  `display_time`, `reshow_delay_ms`, `browse_timeout_ms`, `keyboard_show_delay_ms`.
- **State-model source:** GNOME `_tooltip.scss` and `gtktooltip.c` (browse mode); KDE Breeze's
  `PE_PanelTipLabel`, qtbase `qtooltip.cpp`, qqc2's `ToolTip.qml`; macOS tool-tip and expansion tool-tip
  behaviour (measured); Windows ToolTip visual states and `ToolTipService` timings; Material plain and rich
  tooltip tokens and Compose `TooltipBox`.
- **Today's fields:** `background_color` → `body.fill`; `font` → `label.text.font`; `max_width_px` →
  `body.max_width`; `show_delay_ms` → `values.show_delay_ms`; `border.*` → `body.stroke`, `body.corners`,
  `body.padding`, `body.shadow`.

- **Merged from other groups and rows:**
  - Groups 2 and 4: Kirigami's tooltips use `values.show_delay_ms` under `cond flavour:kirigami` (Q1.136, Q1.65, Q1.82), and in `context sidebar.pane variant compact`.
  - Group 8: `accelerator` (W3-26); `values.hide_delay_ms` (MAT-X25).
  - Group 3: the shortcut tooltip of an action field is `tooltip.body` in `context input` (Q1.18).

### menu

*From group 1's catalogue.*

- **Kind:** typed widget (popover menus and context menus: GtkPopoverMenu, QMenu, qqc2 Menu, NSMenu, UIMenu,
  MenuFlyout, Material's menu and Compose's text context menu). The menu bar is group 6's `menu_bar`; a
  combo box's list is `combo_box`'s.
- **Parts:**
  - `body` — the popup: `fill` (colour, translucency, material: acrylic, KWin blur, glass), `stroke`,
    `corners`, `shadow`, `padding` (the inset of items from the popup's edges), `min_width`, `max_width`,
    `min_height`, `max_height`, `bindings`, `transitions`.
  - `item` *(element)* — one row: `fill` (highlight; appended state layers), `stroke` (KDE's outline; its
    weak-focus line), `corners` (the highlight's own radius), `shadow` (GNOME's HC inset ring), `padding`,
    `min_width`, `min_height`, `opacity`, `focus`, `transitions`.
  - `item_label` — `text` (font, tracking, line height, fill per state).
  - `item_subtitle` — `text` (iOS, macOS 14.4+, Material's supporting text).
  - `item_icon` — the leading icon: `icon` (size, fill per state, KDE's selected-mode effect), `opacity`.
  - `item_trailing_icon` — iOS's item image, Material's trailing icon: `icon`.
  - `accelerator` — the shortcut text: `text`, `opacity`, `padding`.
  - `check_indicator` — the check, radio or mixed mark and its column: `glyph`, `stroke` (GNOME's radio ring),
    `min_width`, `min_height`, `padding`. On KDE the indicator is a `checkbox` element in `context menu.item`.
  - `submenu_arrow` — `glyph`, `opacity`, `min_width` (its box).
  - `badge` — macOS's item badge: `text`, `fill`.
  - `separator` — `fill` or `stroke`, `padding` (insets), `min_height`.
  - `group_separator` — iOS's thick separator between inline sections: `fill`, `min_height`.
  - `group` — Material Expressive's item group box: `fill`, `corners`, `padding`.
  - `section_header` — KDE's section item, macOS's section header, iOS's menu title, Material Expressive's
    section label: `text`, `stroke` (KDE's line after the text), `padding`, `min_height`.
  - `submenu_header` — GNOME's sliding-submenu header: `glyph` (back arrow), `text`, `padding`.
  - `scroll_arrow` *(element)* — the scroll areas of a menu taller than its space: `glyph`, `fill`,
    `min_height`.
  - `split_button` *(element)*, `split_divider` — WinUI's split menu item: the chevron button and the line.
  - `dismiss_overlay` — WinUI's light-dismiss overlay: `fill`.
- **Widget values:** `submenu_open_delay_ms`, `submenu_close_delay_ms: Value<f32>`; `submenu_switch:
  Value<SubmenuSwitch>`, `SubmenuSwitch { Immediate, TimerRestartedOnMotion, CloseTimer, SafeTriangle }`
  (GNOME's timer restarted on motion, KDE's and Windows' close timeout, macOS's triangle);
  `submenu_presentation: Value<SubmenuPresentation>`, `SubmenuPresentation { Cascade, Slide, Inline }`;
  `submenu_placement: Placement`; `placement: Placement`; `presentations` (`Appear`, `Disappear`, keyed
  `@submenu_push`, `@submenu_expand`); `show_item_icons: Value<ItemIcons>`, `ItemIcons { Always, Never,
  OnlyWithoutLabel }`; `check_column`, `icon_column: Value<ColumnReservation>`, `ColumnReservation { Always,
  WhenAnyItemNeeds, Never }`; `check_column_width_px`, `icon_column_width_px`, `icon_text_gap_px` (today),
  `accelerator_gap_px`, `indent_step_px`, `group_gap_px`, `icon_row_gap_px: Value<f32>`;
  `accelerator_alignment: Value<Alignment>`; `scrollable: Value<bool>`; `item_order: Value<ItemOrder>`,
  `ItemOrder { Fixed, NearestFirst, Automatic }` (UIKit's element order); `palette_columns: Value<f32>`
  (per size); `activation_flash` (group 8, MAC-X12); `focus_visible` (ADD-A16).
- **Variants:** on `item`, typed `check`, `radio`, `destructive` (iOS, macOS's SwiftUI destructive item),
  `palette` (macOS's palette menu, iOS's `displayAsPalette`), `icon_only` (GNOME's iconic sections, Material
  Expressive's icon-only menus); keyed GNOME `@circular`, Windows `@split`. On the menu, keyed Material
  `@vibrant`, `@horizontal`; iOS `@compact` (context-menu appearance); Material `@text_context` (Compose
  foundation's own text context menu).
- **Sizes and densities:** iOS's element sizes on `item`: `small`, `regular` (medium), `large`. No density.
- **Widget-specific flags:** typed `hover`, `pressed`, `selected` (keyboard selection), `checked`,
  `indeterminate`, `open` (submenu shown), `disabled`, `focus_visible`, `opened_by_touch` (on the menu);
  keyed KDE `@placed_below`, `@placed_above` (on the menu, set by the connector from the placement result:
  Breeze joins the popup's edge to its menu bar or combo box, K1.64).
- **Exclusive groups:** Windows `[disabled, pressed, hover]` (MenuFlyoutItem's CommonStates).
- **may_be_absent:** `item.stroke`, `item_subtitle`, `item_trailing_icon`, `badge` (all but macOS),
  `group_separator` (all but iOS), `group` (all but Material Expressive), `submenu_header` (all but GNOME),
  `scroll_arrow` (GNOME, iOS, Material, Windows), `split_button`, `split_divider`, `dismiss_overlay` (all but
  Windows), `check_indicator` (KDE: drawn by `checkbox`), `palette_columns`, `item_order`, `indent_step_px`.
- **State-model source:** Windows: MenuFlyoutItem's, ToggleMenuFlyoutItem's, RadioMenuFlyoutItem's and
  MenuFlyoutSubItem's visual state groups (CommonStates, CheckStates, PaddingSizeStates, SubMenuOpened;
  `MenuFlyout_themeresources.xaml`); GNOME: `_menus.scss` selectors (`:hover`, `:selected`, `:active`,
  `:disabled`, `:checked`, HC) and `gtkmodelbutton.c`; KDE: Breeze's `drawMenuItemControl` branches
  (`st:5982-6185`) and qqc2's `MenuItem.qml`; Material: `_md-comp-menu.scss` and the Expressive menu tokens
  (rest, hover, focus, pressed, selected, disabled); macOS: `NSMenuItem.state` (on, off, mixed), `isEnabled`,
  highlight, `NSMenu.presentationStyle`; iOS: `UIMenuElement.State`, `UIMenuElement.Attributes`,
  `UIMenu.ElementSize`, `UIMenu.Options`.
- **Today's fields:** `background_color` → `body.fill`; `separator_color` → `separator.fill`; `font` →
  `item_label.text.font`; `row_height_px` → `item.min_height`; `icon_text_gap_px` →
  `values.icon_text_gap_px`; `icon_size_px` → `item_icon.icon.size`; `hover_background` → `item.fill @
  {hover}`; `hover_text_color` → `item_label.text.fill @ {hover}`; `disabled_text_color` →
  `item_label.text.fill @ {disabled}`; `border.color`, `border.line_width_px` → `body.stroke`;
  `border.corner_radius_px` → `body.corners`; `border.shadow_enabled` → `body.shadow`; `border.padding_*` →
  `item.padding` (K1.63's reading of today's field).

- **Merged from other groups and rows:**
  - Group 6: parts `search_field` and `found_item_pointer` (macOS's Help-menu search, MAC-N33†).
  - Group 4: a toolbar's overflow menu is this widget in `context toolbar.overflow_button`, placed by `toolbar.values.overflow_placement` (W1-102).
  - Group 3: suggestion menus under a field are `menu.body`, `menu.item` in `context input` (MAC-P3-7†).
  - Group 8: `values.submenu_open_delay_ms` (W3-41, MAT-X25, MAC-X12), `values.activation_flash` (MAC-X12), `accelerator` (W3-26).
  - kirigami-addons: `values.presentation_mode` (popup, bottom sheet, dialog) under `cond mobile_form_factor` (ConvergentContextMenu, K-KA-31†).

## Buttons and toggles

### button

*From group 2's catalogue.*

- **Kind:** typed widget (all six platforms).
- **Parts:**
  - `body` — the bezel or container: fill (state layers are appended paint layers, R3), stroke, corners, shadow
    (layers or an elevation entry), padding, min sizes, transform, focus indicator, bindings (KDE's tool-button
    colour set, Q1.9).
  - `label` — the title text.
  - `subtitle` — a second text line under the title (iOS `UIButton.Configuration.subtitle`, B-8).
  - `icon` — the image (icon style: size, fill per state, rendering mode, weight and scale for SF Symbols; a glyph
    for system button types, B-12).
  - `menu_indicator` — the arrow or chevron of a button that opens a menu (GN-1.40, K1.27, B-18†).
  - `attention_dot` — libadwaita's needs-attention dot (GN-1.41).
  - `activity_indicator` — the spinner that replaces the image while loading (B-10); its look is its fill here,
    its motion the spinner widget's.
- **Widget values** (`ButtonValues`):
  - `icon_text_gap: Value<f32>` — between icon and label (today's `icon_text_gap`).
  - `icon_placement: IconPlacement` — `Leading | Trailing | Top | Bottom` (B-9 `imagePlacement`, MAC-B09
    `imagePosition`, Q1.9 text-under-icon).
  - `subtitle_gap: Value<f32>` — iOS `titlePadding` (B-8).
  - `menu_indicator_gap: Value<f32>` — between label and indicator (K1.27 `Button_ItemSpacing`).
  - `menu_trigger: MenuTrigger` — `Primary | LongPress` (B-18†, the touch-and-hold variant).
  - `shows_icon: Value<bool>` — whether push buttons show icons (K1.28, read from `kdeglobals`).
  - `focus_visible_when: FocusVisibleWhen` — when focus becomes visible (K1.21, R17; type: ADD-A16).
  - `repeat_delay: Value<f32>`, `repeat_interval: Value<f32>` (ms; K-RL07†).
  - `attention_dot_offset: (Value<f32>, Value<f32>)` — from the trailing and top edges (GN-1.41).
- **Variants.** Typed (meaning shared by two or more platforms):
  `primary` (accent-filled: GNOME `.suggested-action`, WinUI `AccentButtonStyle`, Material filled, iOS `filled`,
  macOS `tintProminence` primary), `tinted` (tinted, not filled: iOS `tinted`, macOS `NSTintProminence.secondary`,
  Material tonal), `flat` (no bezel at rest: GNOME `.flat`, KDE `setFlat`/auto-raise and Kirigami `flat`, WinUI
  `SubtleButtonStyle`, Material text, iOS `plain`, macOS `isBordered = false`), `destructive` (GNOME
  `.destructive-action`, macOS `hasDestructiveAction`, iOS destructive role), `pill` (GNOME `.pill`, iOS capsule
  corner style, macOS 26 capsule border shape), `circular` (GNOME `.circular`, KDE `_kde_round_button`, macOS
  circular bezel and circle border shape), `glass` (iOS 26, macOS 26), `help` (macOS's help bezel, KDE's ContextualHelpButton: group 6), `back` (group 1), `toggle` (a button with an on state:
  GtkToggleButton, Qt checkable, WinUI `ToggleButton`, Material toggle, macOS `pushOnPushOff`, iOS
  `changesSelectionAsPrimaryAction`; Material draws its unselected toggle unlike a plain button, MAT-B12),
  `menu` (opens a menu: GtkMenuButton, Qt `HasMenu`/QML menu flag, iOS pull-down, WinUI DropDownButton, Material
  menu button, macOS pull-down), and the **content variants** `icon_only` and `with_icon` (GTK's `.image-button`
  and `.image-text-button`, Material's leading-icon padding, AppKit's image-only padding; text only is the base),
  which the connector sets from the element's content (ADD-A61).
  Keyed, per preset:
  - `adwaita`: `@raised`, `@osd`, `@opaque`, `@card`, `@arrow_only` (`.arrow-button`), `@arrow_up`,
    `@arrow_start`, `@arrow_end`, `@arrow_none` (GtkMenuButton's direction; down is the base).
  - `kde-breeze`: `@neutral` (neutral highlight), `@highlighted` (QML `Button.highlighted`), `@text_under`
    (`toolButtonStyle` text under icon).
  - `macos-*`: the bezel styles `@flexible_push`, `@small_square`, `@toolbar`, `@accessory_bar`,
    `@accessory_bar_action`, `@badge`, `@recessed`, `@round_rect`,
    `@inline`, `@textured_rounded`, `@shadowless_square` (push is the base; borderless is `flat`, circular
    `circular`, glass `glass`, help `help`; the disclosure bezels are `expander`'s); the 26 border shape `@rounded_rectangle` (capsule is `pill`, circle `circular`,
    automatic the base).
  - `ios-*`: `@gray`, `@borderless`, `@bordered` (with `tinted` / `primary` for `borderedTinted` /
    `borderedProminent`), `@clear` (with `glass`, and `primary` for the prominent glass styles), corner styles
    `@corner_fixed`, `@corner_small`, `@corner_medium`, `@corner_large`, `@corner_dynamic` (capsule is `pill`),
    roles `@cancel` and, if verified, `@close_role`, `@confirm_role`; system types `@close`, `@info`,
    `@contact_add`; `@done` (bar item); `@indicator_none`, `@indicator_popup` (pull-down indicator; automatic is
    the base); `@menu_on_hold`.
  - `material-3`: `@elevated`, `@outlined`, `@square` (Expressive shape family; round is the base).
  - `windows-11`: none from these rows.
- **Sizes and densities:** `macos-*`: mini, small, regular, large, extra large (`NSControl.ControlSize`; MAC-B08,
  MAC-B09, MAC-B11†). `ios-*`: mini, small, regular (UIKit `medium`), large (`UIButton.Configuration.Size`, B-3).
  `material-3`, flavour `expressive` only: xsmall → mini, small, medium → regular, large, xlarge → extra large
  (MAT-B9). Others: none. Which size an element that states none has is declared (ADD-A60). Densities:
  none from these rows (g8's density rows decide).
- **Widget-specific flags:** `@needs_attention` (`adwaita`), `@loading` (`ios-*`). Typed flags used: hover,
  pressed, focused, focus visible, disabled, checked, indeterminate (WinUI toggle), open, default, drop target.
- **Exclusive groups:** `windows-11`: `{disabled, pressed, hover}` (CommonStates of the Button, ToggleButton and
  accent/subtle templates; check states are named with them). Others: from the state-model source below; none
  stated by these rows (libadwaita's CSS and Breeze's branches combine flags; K1.29's "pressed wins" is a rule
  order, not a group).
- **may_be_absent:** `subtitle` (all but `ios-*`), `attention_dot` (all but `adwaita`), `activity_indicator` (all
  but `ios-*`), `menu_indicator` (absent in the base; set by rules under variant `menu`), `body.focus` where focus
  is drawn elsewhere; values `subtitle_gap`, `menu_trigger` (`ios-*` only), `shows_icon` (`kde-breeze` only),
  `repeat_delay`, `repeat_interval` (where no auto-repeat button), `attention_dot_offset`.
- **State-model source:**
  - `adwaita`: libadwaita's selectors in `_buttons.scss`, `_toolbars.scss` (flat by container), `_linked.scss`,
    `_common.scss` (needs-attention); GTK's `:hover`, `:active`, `:checked`, `:disabled`, `:drop(active)`,
    `.has-open-popup`, `:focus-visible`, `prefers-contrast: more`.
  - `kde-breeze`: Breeze's `Helper::renderButtonFrame` branches (`hp:647-741`) and `Style` `CE_PushButton`,
    `CC_ToolButton` (`st:4516-4543`, `:5327-5380`); flavour `kirigami`: qqc2-desktop-style `Button.qml`,
    `ToolButton.qml` state mapping (Q1.8, Q1.9).
  - `macos-*`: `NSControl.StateValue` (on, off, mixed), highlighted, enabled × `NSButton.BezelStyle` ×
    `NSControl.ControlSize` × `NSControl.BorderShape` (26) × key window, main, application active × appearance;
    the probe renders each.
  - `ios-*`: `UIControl.State` (normal, highlighted, disabled, selected, focused) × `UIButton.Configuration` style
    × `buttonSize` × `cornerStyle` × `UIButton.ButtonType`; measured.
  - `windows-11`: the VisualStateGroups of the Button, ToggleButton (with Checked*/Indeterminate*),
    AccentButtonStyle and SubtleButtonStyle templates (`Button_themeresources.xaml`,
    `ToggleButton_themeresources.xaml`).
  - `material-3`: the tokens `latest/_md-comp-button*.scss` (enabled, hover, focus, pressed, disabled; selected and
    unselected for toggles) per variant and, Expressive, per size.
- **Today's fields:** `background_color` → `body.fill`; `font` → `label.text.font` (its `color` →
  `label.text.fill`); `min_width` → `body.min_width`; `min_height` → `body.min_height`; `icon_text_gap` →
  `values.icon_text_gap`; `primary_background` → `body.fill @ variant primary` (on `macos-*`: `@ {default}`);
  `primary_text_color` → `label.text.fill @ variant primary` (and `icon.icon.fill`); `disabled_opacity` →
  `body.opacity @ {disabled}`; `hover_background` → `body.fill @ {hover}`; `hover_text_color` →
  `label.text.fill @ {hover}`; `active_background` → `body.fill @ {pressed}`; `active_text_color` →
  `label.text.fill @ {pressed}`; `disabled_background` → `body.fill @ {disabled}`; `disabled_text_color` →
  `label.text.fill @ {disabled}`; `checked_background` → `body.fill @ {checked}`; `checked_text_color` →
  `label.text.fill @ {checked}`; `border.color` / `border.line_width` → `body.stroke` (each edge's paint and
  width); `border.corner_radius` → `body.corners`; `border.padding_*` → `body.padding`; `border.shadow_enabled` →
  `body.shadow` (layers or none; the flag goes, R8).

- **Merged from other groups and rows:**
  - Group 1: typed `back` (GNOME's AdwBackButton, WinUI's TitleBar back button, iOS's navigation-bar back button) in `context window.title_bar` or `context toolbar.bar` (GN-1.278†, W1-011, C-2), with the values `back_title_fallback` (UIKit shortens the back title, then replaces it by a generic one) and `back_display_mode` (C-2), and the toolbar's presentation event `@back_label`; keyed Windows `@pane_toggle`.
  - Group 6: the menu button is the typed `menu` here (MAC-N29†, W2-18, MAT-N8: `menu_indicator`, its gap and press animation); `circular` covers KDE's RoundButton (Q2.4–Q2.8: a gloss as an appended linear paint layer, the content shifted under `{pressed}`/`{checked}` as a transform, the Selection binding under `{focused}`); typed `help` (macOS's help bezel MAC-N10, KDE's ContextualHelpButton Q2.48: value `help_tooltip_on_click`, cursor `WhatsThis`, ADD-A59); Material's icon buttons are `icon_only` with `primary`, `tinted` and `@outlined` (MAT-N4, MAT-N5, C-29); keyed Material `@fab`, `@extended_fab` (sizes small, regular, large) and `@lowered` (MAT-N1, MAT-N2).
  - Group 7: keyed iOS `@paste_control` (C-17) with the value `display_mode: Icon | Label | IconAndLabel`; keyed Windows `@repeat` (W2-19) with `repeat_delay`, `repeat_interval`; keyed KDE `@command_link` (K2.7: its title is `label`, its description `subtitle`, with `icon`; Windows' command link is Win32, R1.3, so KDE alone draws it).
  - Merge: group 2's macOS bezel `@help` is the typed `help`; `@disclosure` is the `expander`'s arrow and `@push_disclosure` the `expander` variant (MAC-B01, MAC-EX01).
  - Group 4: KDE's drawer handle button is a `button` in `context sidebar.handle` (Q1.61); GNOME's toolbar children are flat by the rule in `context nearest of {toolbar.bar, popover}` (GN-1.151).
  - GNOME's linked buttons select on `context any layout variant @linked` with position rules (GN-1.42; position among siblings of any kind is CONTRA-3).
  - kirigami-addons: keyed KDE `@ka_floating` (FloatingButton, K-KA-26†) with the value `hit_extension` (margins that enlarge the hit area).

### link

*From group 2's catalogue.*

- **Kind:** typed widget (GNOME, KDE, macOS, iOS, Windows).
- **Parts:** `text` — the link text (its text style carries the underline as `decoration`); `body` — the box
  behind it (a link button's background and borders, W1-144; iOS's pressed highlight box, LK-1; GNOME's
  `button.link`, GN-1.266); `icon` — the external-link icon after the text (Kirigami `UrlButton`, Q1.136).
- **Widget values:** `icon_text_gap: Value<f32>` (Q1.136); `shows_url_tooltip: Value<bool>` (Q1.136; the tooltip's
  look and delay are the tooltip widget's); `focus_visible_when: FocusVisibleWhen` (MAC-LK03).
- **Variants.** Typed: `inline` — a link inside running text (GNOME label links, WinUI `Hyperlink`, AppKit
  `NSTextView` links, iOS text-view links, KDE rich-text labels); the base is a standalone link control
  (GtkLinkButton, WinUI `HyperlinkButton`, Kirigami `LinkButton`/`UrlButton`, `KUrlLabel`). Keyed: `kde-breeze`
  `@url` (UrlButton), `@url_label` (KUrlLabel), `@float` (KUrlLabel's underline-on-hover mode). GNOME's OSD links are
  no variant: `.osd` binds the `Osd` surface set, whose `link` and `link_visited` roles the text refers to
  (GN-1.265, R5.2).
- **Sizes and densities:** none.
- **Widget-specific flags:** none; typed: hover, pressed, focus visible, disabled, visited.
- **Exclusive groups:** `windows-11`: `{disabled, pressed, hover}` (HyperlinkButton's CommonStates).
- **may_be_absent:** `body` (paint absent where the platform draws text only), `icon` (all but `kde-breeze`),
  `text.text.decoration` (no underline in some states, W1-143), `text.focus` (KDE draws focus by weight, Q1.135);
  values `shows_url_tooltip`, `focus_visible` where not stated.
- **State-model source:** `adwaita`: `_links.scss` (`:visited`, `:active`, `:disabled`, `.osd &`, `button.link`);
  `kde-breeze`: Kirigami `LinkButton.qml`, `UrlButton.qml`, KWidgetsAddons `kurllabel.cpp`; `macos-*`:
  `NSTextView.linkTextAttributes` and the focus ring under Full Keyboard Access; `ios-*`: the text view's link
  interaction (pressed), measured; `windows-11`: HyperlinkButton's template states and `Hyperlink.cpp`'s
  underline per state.
- **Today's fields:** `font` → `text.text.font`; `visited_text_color` → `text.text.fill @ {visited}`;
  `underline_enabled` → `text.text.decoration` (a decoration or none); `background_color` → `body.fill`;
  `hover_background` → `body.fill @ {hover}`; `hover_text_color` → `text.text.fill @ {hover}`;
  `active_text_color` → `text.text.fill @ {pressed}`; `disabled_text_color` → `text.text.fill @ {disabled}`.

### segmented_control

*From group 2's catalogue.*

- **Kind:** typed widget (GNOME `AdwToggleGroup`, macOS `NSSegmentedControl`, iOS `UISegmentedControl`, Windows
  Community Toolkit `Segmented`, Material segmented button). KDE draws none natively: today's preset fills it
  from the tab bar as a proxy (platform-facts §2.25), which no row of this group settles (see the merge notes).
- **Parts:**
  - `track` — the group: fill, stroke, corners, padding (the inset of the segments and of the indicator), shadow,
    focus where the platform rings the group.
  - `segment` — one segment, an **element** (its own hover, pressed, checked, disabled, focus): fill, stroke,
    corners (concentric with the track, or rounded only at the ends by position), padding, min sizes, shadow,
    focus.
  - `label`, `icon` — a segment's text and image.
  - `indicator` — the selected thumb drawn as a part of its own (iOS, SG-1; macOS 12–15's raised segment and 26's
    sliding indicator, MAC-SG02, MAC-SG09; the Toolkit's selected item border and pill, W1-138). It is queried
    with the states of the checked segment it lies under.
  - `separator` — the line between two segments, an element whose previous and next siblings are segments (it
    hides beside a checked one: GN-1.243, SG-3, MAC-SG01; ADD-B07).
  - `check` — Material's check glyph in a selected segment (MAT-SG1).
  - `menu_indicator` — AppKit's per-segment menu glyph (MAC-SG07).
- **Widget values:** `segment_width: SegmentWidth` — `Fit` (each its content's width: AdwToggleGroup's default,
  AppKit `fit`, the Toolkit's pivot and button styles), `Fill` (content plus an equal share of extra space:
  AppKit's default `fill`), `FillEqually` (equal widths across the control: Material, AppKit `fillEqually`,
  UIKit's default), `FillProportionally` (AppKit), `EqualToWidest` (the Toolkit's `EqualPanel`) — GN-P3-25†,
  MAC-P3-5†, MAT-P3-12†, SG-9†, W-P3-5†; `item_gap: Value<f32>` (W-P3-5†); `compression: Key` (AppKit
  `activeCompressionOptions`); `image_scaling: Key` (AppKit `imageScaling`, MAC-SG10†); `indicator_move:
  PresentationTransition` (event `Move`: the indicator sliding to the new segment, SG-5, MAC-SG09, W1-138);
  `indicator_draggable: Value<bool>` (SG-5); `focus_visible_when: FocusVisibleWhen`.
- **Variants.** Typed: `pill` (GNOME `.round`, AppKit `.capsule` and the 26 capsule shape), `momentary` (AppKit
  `momentary` tracking, UIKit `isMomentary`), content variants `icon_only`, `with_icon` (GNOME's minimum widths,
  GN-1.244; the Toolkit's item layouts, W1-154†). Keyed: `adwaita` `@flat`, `@osd`, `@vertical`; `macos-*` `@menu`
  on a segment (`showsMenuIndicator`), `@rounded`, `@round_rect`, `@textured_rounded`, `@textured_square`, `@small_square`,
  `@separated`, `@rounded_rectangle` (26 shape), `@select_any`; `windows-11` `@pivot`, `@button` (the default
  style is the base).
- **Sizes and densities:** `macos-*`: mini, small, regular, large (MAC-SG10†). `ios-*`: no sizes; the height
  follows Dynamic Type, whose default category the preset states (R10, SG-9†). Densities: none from these rows.
- **Widget-specific flags:** none; a chosen segment is `checked` (a segment is a toggle; select-any segments are
  several checked toggles; R3 keeps `selected` for list selection).
- **Exclusive groups:** `windows-11`: `{disabled, pressed, hover}` (SegmentedItem's CommonStates, which name
  Selected, PointerOverSelected and PressedSelected with them).
- **may_be_absent:** `indicator` (`adwaita`, `material-3` draw the checked segment's own fill), `separator`
  (`windows-11`, `material-3` draw none), `check` (all but `material-3`), `menu_indicator` (all but `macos-*`),
  `track.fill` (Material, `@flat`); values `compression`, `image_scaling` (`macos-*` only), `indicator_move`,
  `indicator_draggable`, `item_gap` where not drawn.
- **State-model source:** `adwaita`: `_toggle-group.scss` selectors (`toggle:hover`, `:active`, `:checked`,
  `:disabled`, `.flat`, `.round`, `.osd`, `.vertical`, `prefers-contrast: more`); `macos-*`: per segment
  selected, highlighted, enabled × `NSSegmentedControl.Style` × `trackingMode` × control size × key window ×
  appearance; `ios-*`: `UIControl.State` per segment × `isMomentary`, measured; `windows-11`: the Toolkit's
  `SegmentedItem.xaml` visual states (`:426-603`) per style; `material-3`: the outlined segmented button tokens
  (selected, unselected × enabled, hover, focus, pressed, disabled).
- **Today's fields:** `background_color` → `track.fill`; `font` → `label.text.font`; `active_background` →
  `segment.fill @ {checked}` (`indicator.fill` where the platform draws an indicator); `active_text_color` →
  `label.text.fill @ {checked}`; `segment_height` → `segment.min_height`; `separator_width` →
  `separator.min_width`; `disabled_opacity` → `track.opacity @ {disabled}` (the Toolkit fades the item only:
  `segment.opacity @ {disabled}`); `hover_background` → `segment.fill @ {hover}`; `border.color` /
  `border.line_width` → `track.stroke`; `border.corner_radius` → `track.corners`; `border.padding_*` →
  `segment.padding`; `border.shadow_enabled` → `track.shadow` / `indicator.shadow`.

- **Merged from other groups and rows:**
  - Group 6: Material's button group, keyed `@standard_group`, `@connected`: inner corners by state, the pressed width multiplier and its spring (MAT-N6).
  - kirigami-addons: keyed KDE `@ka_segmented_button` (K-KA-24†: joined segments, outer corners only, a hard shadow layer hidden while pressed) and `@ka_radio_selector` (K-KA-25†: a sliding `indicator` pill).

### switch

*From group 2's catalogue.*

- **Kind:** typed widget (all six platforms; KDE's is Kirigami's, qqc2-desktop-style `SwitchIndicator`).
- **Parts:**
  - `body` — the whole control with its label: padding, min width (Q1.103, W1-124).
  - `track` — fill, stroke, corners, focus (GN-1.206, MAT-SW9), opacity (applied to the thumb and glyphs it
    contains).
  - `thumb` — the knob or handle: fill, stroke (the rim), shadow (layers or an elevation entry), corners, min
    width and height (a capsule where width and height differ, SW-3, MAC-SW01; growing or stretching by state),
    transform (iOS 26's lens).
  - `track_fill` — KDE's highlight overlay from the track's start to the thumb's trailing edge (Q1.100).
  - `on_glyph`, `off_glyph` — the status shapes on the track (GNOME `switch-on/off-symbolic`, iOS's "I" and "O",
    macOS under differentiate without colour).
  - `thumb_icon` — Material's check or close icon on the handle (MAT-SW3).
  - `state_layer` — Material's halo (MAT-SW4).
  - `label` — the switch's label text (SW-7, Q1.103); `state_label` — WinUI's On/Off content text; `header` —
    WinUI's header (W1-124).
- **Widget values:** `thumb_move: PresentationTransition` (event `Move`: the thumb sliding between off and on;
  GN-1.203, SW-5, Q1.102, MAT-SW8, W1-123); `track_inset: Value<f32>` (Kirigami's track inset, an expression on
  the grid unit, Q1.99; ADD-A56); `label_gap: Value<f32>`; `pre_content_margin`, `post_content_margin:
  Value<f32>` (WinUI, W1-124); `focus_visible_when: FocusVisibleWhen`.
- **Variants.** Typed: none. Keyed: `material-3` `@icons` (the switch with handle icons). GNOME's OSD switch is a
  context (`ctx any … variant @osd`, GN-1.205; ADD-B08).
- **Sizes and densities:** `macos-*`: mini, small, regular, large (MAC-SW05). Others none.
- **Widget-specific flags:** none; typed: hover, pressed, focus visible, disabled, checked.
- **Exclusive groups:** `windows-11`: `{disabled, pressed, hover}` (ToggleSwitch's CommonStates; ToggleStates is a
  group of its own and maps to `checked`).
- **may_be_absent:** `track_fill` (all but `kde-breeze`), `on_glyph`, `off_glyph` (absent in the base; set under
  the conditions that show them), `thumb_icon` (all but `material-3`), `state_layer` (all but `material-3`),
  `state_label`, `header` (all but `windows-11`), `label`, `thumb.shadow` / `thumb.stroke` where not drawn; values
  `track_inset` (all but `kde-breeze`), `pre_content_margin`, `post_content_margin` (all but `windows-11`).
- **State-model source:** `adwaita`: `_switch.scss` selectors and `gtkswitch.c` (status shapes, slide);
  `kde-breeze`: qqc2-desktop-style `SwitchIndicator.qml` / `Switch.qml` states (flavour `kirigami`; the flavour
  `widgets` has no Qt Widgets switch, which the facts settle); `macos-*`: `NSSwitch` state, highlighted, enabled ×
  control size × key window × appearance; `ios-*`: `UISwitch` `isOn`, tracking, enabled, measured; `windows-11`:
  ToggleSwitch's CommonStates × ToggleStates (`ToggleSwitch_themeresources.xaml:232-321`); `material-3`: the
  switch tokens (selected, unselected × enabled, hover, focus, pressed, disabled; with icons).
- **Today's fields:** `track_width`, `track_height` → `track.min_width`, `track.min_height`; `thumb_diameter` →
  `thumb.min_width`, `thumb.min_height @ {checked}`; `unchecked_thumb_diameter` → the same in the base;
  `track_radius` → `track.corners`; `checked_background` → `track.fill @ {checked}`; `unchecked_background` →
  `track.fill`; `thumb_background` → `thumb.fill @ {checked}`; `unchecked_thumb_background` → `thumb.fill`;
  `disabled_opacity` → `track.opacity @ {disabled}`; `hover_checked_background` → `track.fill @ {checked,hover}`;
  `hover_unchecked_background` → `track.fill @ {hover}`; `disabled_checked_background` →
  `track.fill @ {checked,disabled}`; `disabled_unchecked_background` → `track.fill @ {disabled}`;
  `disabled_thumb_color` → `thumb.fill @ {disabled}`; `unchecked_track_border_color` → `track.stroke`;
  `checked_track_border_color` → `track.stroke @ {checked}`; `disabled_unchecked_track_border_color` →
  `track.stroke @ {disabled}`; `disabled_checked_track_border_color` → `track.stroke @ {checked,disabled}`;
  `track_border_line_width` → `track.stroke` widths; `thumb_border_color`, `thumb_border_line_width` →
  `thumb.stroke`.

- **Merged from other groups and rows:**
  - Group 4: KDE's switch delegate in `context list.row` (Q1.83); GNOME's in toolbars by the container rule (GN-1.152).
  - Group 8: iOS's On/Off Labels are `on_glyph`, `off_glyph` under `cond on_off_labels` (X-A11-6).
  - kirigami-addons: in `context form.row variant @ka_form_card` without padding and label (K-KA-8†).

### checkbox

*From group 2's catalogue.*

- **Kind:** typed widget (GNOME, KDE, macOS, Windows, Material; iOS only as the circular list-selection
  indicator, CK-1, which is the `selection` variant).
- **Parts:**
  - `body` — the whole control with its label: padding, corners, min height and width (W1-040), focus where the
    platform rings indicator and label together (GN-1.61).
  - `box` — the indicator: fill, stroke, corners, shadow (KDE's hover ring and macOS 12–15's bevel are inset
    layers, R7), min sizes, focus where the platform rings the box (K1.57).
  - `mark` — the check mark, the indeterminate dash or dots by rule: a glyph (GNOME and Material paths, Breeze's
    `QPainterPath`, Segoe Fluent code points, AppKit's whole-control rendering, iOS's named symbol), opacity and
    transform for its entrance.
  - `label` — the text; its focus where Kirigami rings the label (Q1.21).
  - `state_layer` — Material's halo (MAT-C2).
- **Widget values:** `label_gap: Value<f32>`; `indicator_cell: Value<f32>` (the cell the drawn box is centred in,
  K1.53); `touch_target: Value<f32>` (MAT-C9); `appear: PresentationTransition` (iOS's selection column sliding
  in with edit mode, CK-2†); `focus_visible_when: FocusVisibleWhen`.
- **Variants.** Typed: `selection` (the round check of a list in selection mode: GNOME `.selection-mode`, iOS
  `UICellAccessory.multiselect`). Keyed: `adwaita` `@text_button` (GTK sets `.text-button` on a labelled check,
  the connector from content); `kde-breeze` `@neutral`.
- **Sizes and densities:** `macos-*`: mini, small, regular, large. `ios-*`: the size follows Dynamic Type (default
  category stated, R10). Others none.
- **Widget-specific flags:** none; typed: hover, pressed, focus visible, disabled, checked, indeterminate, invalid
  (Material's error, MAT-C5).
- **Exclusive groups:** `windows-11`: `{disabled, pressed, hover}` (CheckBox's CombinedStates, which name the check
  state with them).
- **may_be_absent:** `state_layer` (all but `material-3`), `body.focus` / `box.focus` / `label.focus` (each where
  the platform rings another part), `mark` (absent while unchecked), `box.shadow`; values `indicator_cell`
  (`kde-breeze`), `touch_target` (`material-3`), `appear` (`ios-*`).
- **State-model source:** `adwaita`: `_checks.scss` (`check`, `radio`, `checkbutton`, `:checked`,
  `:indeterminate`, `:hover`, `:active`, `:disabled`, `.selection-mode`, `prefers-contrast: more`); `kde-breeze`:
  Breeze `Helper::renderCheckBox` (`hp:831-981`) and `Style` `CE_CheckBox`; flavour `kirigami`: qqc2
  `CheckBox.qml`, `CheckIndicator.qml`; `macos-*`: `NSButton` checkbox `StateValue` on, off, mixed × highlighted ×
  enabled × control size × key window × appearance, with the whole-control rendering (S15); `ios-*`: the list
  cell's selected, editing and disabled states, measured; `windows-11`: CheckBox's CombinedStates (12 states);
  `material-3`: checkbox tokens (selected, unselected, indeterminate × enabled, hover, focus, pressed, disabled ×
  error).
- **Today's fields:** `background_color` → `body.fill`; `font` → `label.text.font`; `indicator_color` →
  `mark.glyph.fill`; `indicator_width` → `box.min_width`, `box.min_height` (on `kde-breeze` the cell:
  `values.indicator_cell`, K1.53); `check_mark_stroke_width` → `mark.glyph.stroke_width`; `label_gap` →
  `values.label_gap`; `checked_background` → `box.fill @ {checked}`; `disabled_opacity` →
  `body.opacity @ {disabled}`; `hover_background` → `box.fill @ {hover}`; `disabled_background` →
  `box.fill @ {disabled}`; `disabled_text_color` → `label.text.fill @ {disabled}`; `unchecked_background` →
  `box.fill`; `unchecked_border_color` → `box.stroke`; `border` → `box.stroke`, `box.corners`, `body.padding`;
  `radio_indicator_width`, `radio_dot_diameter` → `radio` (below).

- **Merged from other groups and rows:**
  - Group 1: KDE draws the menu's check as a `checkbox` element in `context menu.item` (K1.69, Q1.31).
  - Group 4: KDE's check delegate in `context list.row` (Q1.83).
  - Group 6: KDE's checkable group title is a `checkbox` in `context group_box` (K2.8).
  - kirigami-addons: in `context form.row variant @ka_form_card` (K-KA-8†) and `context list.row variant @ka_rounded` (K-KA-38†).

### radio

*From group 2's catalogue.*

- **Kind:** typed widget (GNOME, KDE, macOS, Windows, Material; iOS has none). Split from today's `checkbox`
  because the platforms state its colours and geometry apart (W1-043, MAT-C1) and its parts differ.
- **Parts:** `body` (the whole control: padding, corners, focus; GNOME rounds a grouped one fully, GN-1.63),
  `ring` (the outer circle: fill, stroke, corners, shadow, focus), `dot` (the inner dot: glyph or fill, min sizes
  growing by state, W1-041, stroke, transform for its entrance), `label`, `state_layer` (Material).
- **Widget values:** `label_gap`, `touch_target`, `focus_visible` (as checkbox).
- **Variants:** none from these rows. **Sizes:** `macos-*` mini, small, regular, large.
- **Widget-specific flags:** none; typed: hover, pressed, focus visible, disabled, checked.
- **Exclusive groups:** `windows-11`: `{disabled, pressed, hover}` (RadioButton's CommonStates).
- **may_be_absent:** `state_layer` (all but `material-3`), `ring.focus` / `body.focus` / `label.focus` (as
  checkbox), `dot` (absent while unchecked); `touch_target`.
- **State-model source:** as `checkbox`, with Breeze's `renderRadioButton` (`hp:995-1075`), AppKit's radio
  `NSButton` and the whole-control rendering (`SystemControl::AppKitRadio`), WinUI's RadioButton template, and
  Material's radio tokens.
- **Today's fields:** `checkbox.radio_indicator_width` → `ring.min_width`, `ring.min_height`;
  `checkbox.radio_dot_diameter` → `dot.min_width`, `dot.min_height @ {checked}`; the shared `checkbox` colours are
  converted into `radio`'s bases the same way (`checked_background` → `ring.fill @ {checked}`, `indicator_color`
  → `dot.fill`, `unchecked_border_color` → `ring.stroke`, …).

- **Merged from other groups and rows:**
  - Group 4: KDE's radio delegate in `context list.row` (Q1.83).

### radio_group

*From group 2's catalogue.*

- **Kind:** typed widget: Windows draws one (`RadioButtons`, W2-25, listed for Windows only in §4.2) and macOS draws
  it inside the radio (`RadioGroupPickerStyle`, MAC-RL02†, a cross-check row), so two platforms (R14).
- **Parts:** `header` — the group's label (W2-25 header colour, disabled colour, margin; MAC-RL02†'s label
  beside the column).
- **Widget values:** `row_gap: Value<f32>`, `column_gap: Value<f32>`, `header_gap: Value<f32>`, `header_placement:
  HeaderPlacement` (`Above | Leading`).
- **Variants:** keyed `macos-*` `@horizontal` (`horizontalRadioGroupLayout()`). **Sizes:** `macos-*` mini, small,
  regular, large.
- **Flags, exclusive groups:** none (the group holds no interaction state; its radios do).
- **may_be_absent:** `header`, `header_placement`.
- **State-model source:** `windows-11`: `RadioButtons_themeresources.xaml`; `macos-*`: the SwiftUI host's measured
  layout.
- **Today's fields:** none (`checkbox.label_gap` and `layout.widget_gap` were the nearest).

- **Merged from other groups and rows:**
  - Group 7: Windows' `RadioButtons` (W2-25): `header` colour, disabled colour and padding; `row_gap`, `column_gap` (group 7 had placed them on `checkbox`).

## Fields

### input

*From group 3's catalogue.*

**Kind:** typed widget (all six platforms draw a single-line text field).

**Parts:**

| Part | What it is |
|---|---|
| `body` | the field's box: fill, per-side stroke (WinUI's thicker accent bottom edge, Material's filled-field active indicator as the bottom edge), corners, shadow (macOS's inner edge shading and 26 outer ring; GNOME's inset drop-target and high-contrast rings), padding, size limits, focus indicator, bindings (GNOME's `.error`/`.warning`/`.success` rebind the accent for the subtree) |
| `text` | the entered text (text style) |
| `placeholder` | the placeholder text |
| `prefix`, `suffix` | Material's prefix and suffix text |
| `floating_label` | Material's label: inside at rest, floated onto the outline (transform) when focused or populated |
| `header` | WinUI's header label above the field |
| `supporting_text` | text under the field: Material's supporting (helper) text, WinUI's description |
| `counter` | Material's trailing character counter |
| `icon` | element: an icon or inline action inside the field (GNOME's primary/secondary icons, iOS's overlay views, Kirigami's search icon and ActionTextField actions, Material's leading/trailing icons); typed variants `leading`, `trailing` |
| `clear_button` | element: the clear button (glyph or icon, fill and stroke per state) |
| `reveal_button` | element: the password-reveal toggle (its icon per checked state) |
| `caps_lock_indicator` | macOS's caps-lock glyph in a secure field |
| `autofill_button` | the AutoFill / strong-password affordance inside the field (macOS, iOS) |
| `progress` | GNOME's entry progress line (`progress-fraction`) |
| `caret` | the line caret's paint; its shape, width and blink come from `token.caret` (R12) and the widget values below |
| `block_caret` | the block caret (overwrite mode, Qt's input mask): fill and the character's text fill |
| `selection` | the selected-text highlight: fill and selected text fill |
| `preedit` | element: the IME composition text; typed variant `selected_clause`, keyed variants per platform hint (GTK's `.whole`, `.selection`, `.prediction`, `.prefix`, `.suffix`, `.spelling-error`, `.compose-error`; WinUI's clause attributes) |
| `misspelling` | the spell-check mark (text decoration) |
| `correction` | iOS's autocorrection mark (and bubble), Material's suggestion underline |
| `prediction` | inline prediction ghost text (iOS 17+, macOS 14+) |
| `selection_handle` | element: a touch selection handle (Material, KDE tablet mode, WinUI grippers, iOS; GNOME's GN-2.42) |
| `magnifier` | the touch text magnifier (GNOME GtkText, Material) |

**Widget values:**

| Value | Type | Rows |
|---|---|---|
| `icon_text_gap` | length | GN-1.43, MAT-I7, Q1.18 |
| `leading_inset`, `trailing_inset` | length | I-3 |
| `affix_gap` | length (prefix/suffix to text) | MAT-I8 |
| `floating_label_gap` | length (label to text, populated) | MAT-I3 |
| `notch_padding` | length: the gap cut into the outline round the floated label, which the toolkit cuts at layout | MAT-I4 |
| `header_gap`, `supporting_text_gap` | length | W1-031, W1-150, MAT-I5 |
| `clear_button_visibility` | enum `ShowWhen { Never, Always, WhileEditing, UnlessEditing, WhenNotEmpty, WhenFocusedAndNotEmpty }` | I-2, W1-030, K1.46 |
| `reveal_button_visibility` | `ShowWhen` | K1.48 |
| `mask_character` | `Value<char>` | K1.48, MAC-I03, I-9, MAT-I19 |
| `mask_reveal_ms` | duration (last typed character shown; 0 none) | K1.48, I-9, MAT-I19 |
| `caret_width` | length, base `= token.caret.width`; rules (KDE tablet mode) | Q1.14 |
| `focus_visible_when` | enum `FocusVisibleWhen { Keyboard, AnyFocus, FocusWithin }` (R17) | GN-1.50 |
| `min_font_size` | length (shrink-to-fit floor) | I-7 |
| `default_width` | length (implicit width) | Q1.12 |
| `magnifier_zoom` | number | GN-P3-26, MAT-I17 |
| `magnifier_offset` | length (above the caret or touch point) | GN-P3-26, MAT-I17 |
| `search_delay` | duration (Kirigami search field debounce; selected under `context any(search_field)`) | Q1.17 |
| `suggestion_placement` | `PlacementStrategy` + params (macOS suggestion menus) | MAC-P3-7 |
| `presentations` | `Vec<PresentationTransition>` (clear button fade in and out; none on WinUI) | K1.46, W1-030 |

**Variants.** Typed: `flat` (no frame: GNOME `.flat`, KDE frameless, macOS borderless, iOS `none`),
`line` (a plain line frame: iOS `.line`, macOS bordered-not-bezeled), `secure` (password field, all six),
`warning` (GNOME `.warning`, KDE neutral highlight). Keyed: `adwaita` `@success`; `kde-breeze`
`@border_top`, `@border_right`, `@border_bottom`, `@border_left` (Breeze's `_breeze_borders_sides`: only
the named edges drawn, K1.42); `macos-*` `@square_bezel`, `@rounded_bezel`; `ios-*` `@bezel`;
`material-3` `@filled` (outlined is the base).

**Sizes and densities.** `macos-*`: the control sizes AppKit draws for a text field (of mini to extra
large, MAC-X17; which ones, per generation, from the probe). `ios-*`: none (height follows Dynamic Type, I-10, read by the readers' milestone).
`material-3`: its density steps (keyed). Others: none.

**Widget-specific flags.** `empty` (the field holds no text: Material's floated label, WinUI's and KDE's
clear button). States used besides: hover, pressed (icons, buttons), focused, focus within, focus visible,
disabled, read-only, invalid, autofilled, drop target, checked (reveal toggle).

**Exclusive groups.** `windows-11`: TextBox `CommonStates` {Normal, Disabled, PointerOver, Focused} and
`ButtonStates` {ButtonVisible, ButtonCollapsed}; their precedence is read from the control's state code in
the value phase.

**may_be_absent:** `prefix`, `suffix`, `floating_label`, `header`, `supporting_text`, `counter`,
`caps_lock_indicator`, `autofill_button`, `progress`, `block_caret`, `correction`, `prediction`,
`selection_handle`, `magnifier`, `reveal_button`, `body.shadow` layers, `body.focus`;
values `notch_padding`, `floating_label_gap`, `affix_gap`, `header_gap`, `supporting_text_gap`,
`mask_reveal_ms`, `min_font_size`, `default_width`, `magnifier_*`, `search_delay`, `suggestion_placement`.

**State-model source:**

- `windows-11`: TextBox template visual state groups (`CommonStyles/TextBox_themeresources.xaml`:
  CommonStates, ButtonStates).
- `adwaita`: `_entries.scss` selectors (`:focus-within`, `:disabled`, `.flat`, `:drop(active)`,
  `block-cursor`, `progress`, icon `:hover`/`:active`), `_labels.scss` (`.error`, `.warning`, `.success`),
  the high-contrast media query; GtkText's pre-edit and magnifier nodes.
- `kde-breeze`: `breezestyle.cpp` line-edit frame and panel branches (enabled, mouse over, focus, neutral
  highlight, frame on or off, per-side borders) with the input-widget engine; the `kirigami` flavour from
  qqc2-desktop-style `TextField.qml`, `SearchField.qml`, `private/MobileCursor.qml` and Kirigami's
  `SearchField.qml`, `ActionTextField.qml`, `PasswordField.qml`.
- `material-3`: the outlined and filled text-field token states (enabled, hover, focus, error, disabled) of
  the pinned token set; Compose's text field for caret, handles, magnifier, secure field and autofill.
- `macos-*`: `NSTextField` / `NSSecureTextField`: enabled, editable, first responder, bezel style, bordered,
  control size, window key state, appearance.
- `ios-*`: `UIControl.State` (normal, disabled, focused), editing, `borderStyle`, the content-size category.

**Today's fields:**

| Today | New |
|---|---|
| `background_color` | `body.fill` |
| `placeholder_color` | `placeholder.text.fill` |
| `caret_color` | `caret.fill` (base `= token.caret.color`) |
| `selection_background`, `selection_text_color` | `selection.fill`, `selection.text.fill` (bases referring to the selection roles) |
| `font` | `text.text.font`, its colour `text.text.fill` |
| `min_height_px` | `body.min_height` |
| `disabled_opacity` | `body.opacity` under `{disabled}` |
| `hover_border_color` | `body.stroke` paint under `{hover}` |
| `focus_border_color` | `body.stroke` under `{focused}` where the platform recolours the frame; `body.focus` where it draws a ring |
| `disabled_background`, `disabled_text_color` | `body.fill`, `text.text.fill` under `{disabled}` |
| `border.*` | `body.stroke`, `body.corners`, `body.padding`; `shadow_enabled` → `body.shadow` |

- **Merged from other groups and rows:**
  - Group 1: in `context dialog.body variant alert` (DL-9) and `variant @search_dialog` (Q1.118); `values.context_menu_items` (GN-P4-8†).
  - Group 4: the in-place cell editor in `context list.cell {@editing}` (K-RL05†, MAC-L13, GN-1.279†).
  - Group 6: the password field is the typed `secure` (group 6 wrote `password`): `reveal_button`, `caps_lock_indicator`, values `mask_character` (a preference list), `hint_timeout_ms`, `peek_while_pressed` (GN-2.23, GN-2.58†, W2-16); typed `editable_label` (view colours while editing; iOS's rename field in the navigation bar and its title-to-field transition; GN-2.45, RL-2†).
  - Group 7: `selection_handle` takes the typed identities `start`, `end`, `insertion` (the insertion handle rotated by a transform rule, GN-2.42).
  - Group 8: the caret part and its fields (ADD-A24); parts `grammar_mark`, `dictation_effect`, `caret_accessory`, `floating_caret`, `drop_caret`; flag `@composing` (K3.48†).
  - Merge: `writing_tools_affordance` (the Writing Tools affordance, MAC-P3-8†; group 3 had named it `writing_tools.affordance`).
  - kirigami-addons: `strength_meter`, `strength_meter_fill` (the password strength meter's segments and their fill), value `strength_meter_gap`, keyed flags `@quality_bad`, `@quality_poor`, `@quality_weak`, `@quality_good`, `@quality_excellent` (K-KA-11†); `counter` under the keyed flag `@at_limit` (K-KA-9†).

### text_area

*From group 3's catalogue.*

**Kind:** typed widget (multi-line text view: GtkTextView, QTextEdit / qqc2 TextArea, NSTextView, UITextView,
WinUI's multi-line TextBox, Material's textarea field).

**Parts:** `body` (the frame and fill), `text`, `placeholder`, `caret`, `block_caret`, `selection` (one
element per selected range, P4-13), `preedit`, `misspelling`, `correction`, `prediction`,
`selection_handle`, `magnifier` (all as on `input`), and:

| Part | What it is |
|---|---|
| `gutter` | GtkTextView's border windows (gutters) |
| `find_match` | element: a find-and-replace match; the current match is `{selected}` |
| `highlight` | element: an application text highlight; keyed variants per platform style (iOS 18's highlight styles) |
| `link` | detected-data and link text (text style with decoration) |
| `invisible` | element: the substitute glyph TextKit 1 draws for an invisible character; typed variants `space`, `tab`, `line_break`, `no_break_space` |

**Widget values:**

| Value | Type | Rows |
|---|---|---|
| `document_margin` | `Sides<Value<f32>>`: the inset inside the document, which scrolls with the text (Qt's `documentMargin`, Apple's `textContainerInset`); `body.padding` is the fixed frame inset | K-P4-9, P3-1, MAC-I10 |
| `line_padding` | length: Apple's `lineFragmentPadding` at each line's start and end | P3-1, MAC-I10 |
| `scrollbar_inset` | `Sides<Value<f32>>` (WinUI's inner scroll bar) | W1-036 |
| `default_width`, `default_height` | length | Q1.12 |
| `min_rows` | integer | MAT-I16 |
| `resize` | enum `Resize { None, Vertical, Both }` | MAT-I16 |
| `empty_ranges_draw_carets` | `Value<bool>` | P4-13 |
| `intent_styles` | `Vec<(Key, TextStyle)>`: how the text view renders Foundation's presentation intents, keyed by intent; absent where it renders none | SF-5, MAC-SF05 |
| `focus_visible_when`, `caret_width`, `magnifier_zoom`, `magnifier_offset` | as on `input` | |

**Variants.** Typed: `flat` (no frame: macOS `NSScrollView` border none, iOS `none`), `line` (macOS line
border). Keyed: `adwaita` `@inline` (`textview.inline`, GN-1.54); `macos-*` `@bezel`, `@groove`; `ios-*`
`@rounded_rect` (iOS 17's `roundedRect`). The enclosing `NSScrollView`'s border is `scroll_view.frame`
(below), selected by the same variants where the text view is in one.

**Sizes and densities.** `macos-*`: control sizes do not apply to `NSTextView`; none elsewhere.

**Widget-specific flags.** `empty` (as `input`). States: focused, focus within, focus visible, disabled,
read-only, drop target, selected (find match).

**Exclusive groups.** `windows-11`: as `input` (the multi-line box is a TextBox).

**may_be_absent:** `gutter`, `find_match`, `highlight`, `invisible`, `link`, `block_caret`, `correction`,
`prediction`, `selection_handle`, `magnifier`; values `document_margin`, `line_padding`,
`scrollbar_inset`, `default_*`, `min_rows`, `intent_styles`.

**State-model source:** `windows-11` as `input`; `adwaita` `_views.scss` (`textview`, `> border`,
`.inline`, `:drop(active)`); `kde-breeze` the QTextEdit frame branches and qqc2 `TextArea.qml`;
`material-3` the textarea form of the text-field tokens; `macos-*` `NSTextView` (editable, selectable,
first responder, window key) inside `NSScrollView` (`borderType`); `ios-*` `UITextView` (editable,
`borderStyle`, `UIFindInteraction` states, `textHighlightAttributes`).

**Today's fields:** `border.*` → `body.stroke`, `body.corners`, `body.padding` (the fixed frame inset;
the scrolling part moves to `values.document_margin`), `body.shadow`. Everything today read from `input`
is stated on `text_area`'s own parts (no model inheritance, R20).

- **Merged from other groups and rows:**
  - Group 8: the current find match (macOS's find indicator) is `find_match @ {selected}` (MAC-X15); `drop_caret` (P3-9†).
  - Merge: `writing_tools_glow` (the Writing Tools glow over rewritten text: MAC-X28, TA-4, C-33; group 3 had named it `writing_tools.glow`).
  - Group 1: `values.context_menu_items` (GN-P4-8†); the overwrite caret under the flag `@overwrite` (ADD-A23).

### combo_box

*From group 3's catalogue.*

**Kind:** typed widget (GtkDropDown, QComboBox / qqc2 ComboBox, NSPopUpButton and NSComboBox, iOS's
pop-up button, WinUI ComboBox, Material's select).

**Parts:**

| Part | What it is |
|---|---|
| `body` | the trigger box: fill, stroke per state, corners, shadow (Breeze's button shadow on the non-flat, non-editable combo), padding, size limits, focus indicator, bindings (Kirigami: Button set, View set when editable) |
| `label` | the current value's text |
| `placeholder` | WinUI's placeholder text |
| `icon` | a leading icon (Material) |
| `arrow` | element: the indicator glyph (single or double chevron; on macOS the AppKit rendering, `SystemControl::AppKitPopUp`), its fill per state, its rotation when open |
| `arrow_area` | macOS's accent-filled arrow area of the pop-up, `NSComboBox`'s separate arrow button |
| `header` | WinUI's header text |
| `floating_label`, `supporting_text` | Material's select field parts (as `input`) |
| `popup` | the drop-down surface: fill (a material on WinUI), stroke, corners, shadow, padding, size limits |
| `item` | element: a row of the popup, with its own states |
| `item_mark` | the selected item's check mark glyph (GNOME after the label, iOS) |
| `item_pill` | WinUI's selection pill on the selected item |

Where a platform's popup is its menu (GNOME's menu-style popover, the qqc2 combo's Menu, macOS's pop-up
menu, iOS's menu, Material's menu) the preset states `popup`, `item` and `item_mark` with the facts of that
menu; the gate checks both against the same facts (R20: no model inheritance). The editable combo's text is
an `input` element inside the combo, selected by `context any(combo_box)`; GNOME's search entry in the popup
is an `input` in `context any(combo_box.popup)`.

**Widget values:**

| Value | Type | Rows |
|---|---|---|
| `label_arrow_gap` | length | GN-1.233 |
| `item_mark_position` | enum `MarkPosition { Leading, Trailing }` | GN-1.234 |
| `header_gap`, `supporting_text_gap` | length | W1-137, MAT-CB1 |
| `max_visible_items` | integer | W1-133, MAC-CB04 |
| `popup_placement` | `PlacementStrategy` + params (popup width against the field, alignment, height limit) | W1-136, MAT-P3-9, K1.245 |
| `presentations` | `Vec<PresentationTransition>` (WinUI's split open from the selected item, KDE Quick's menu fade) | W1-136, Q1.125 |

**Variants.** Typed: `editable` (KDE, macOS `NSComboBox`, WinUI, Material's editable menu, GTK's combo with
entry), `flat` (GNOME `.flat`, KDE frameless, macOS borderless), `pull_down` (macOS `pullsDown`, iOS's menu
button that does not change selection), `warning` (KDE neutral highlight). Keyed: `adwaita`
`@gtk_combo_box` (GN-1.238); `material-3` `@filled`; `ios-*` the button configurations of `button` (CB-2).

**Sizes and densities.** `macos-*`: mini, small, regular, large. `material-3`: density (keyed).

**Widget-specific flags.** None of its own. States: hover, pressed, focused, focus visible, disabled, open;
items: hover, pressed, selected, disabled; the popup: focus within, opened by touch.

**Exclusive groups.** `windows-11`: ComboBox `CommonStates` {Normal, PointerOver, Pressed, Disabled},
`FocusStates` {Focused, FocusedPressed, Unfocused, PointerFocused, FocusedDropDown}, `DropDownStates`
{Opened, Closed}, `EditableModeStates`; ComboBoxItem `CommonStates` {Normal, PointerOver, Disabled,
Pressed, Selected, SelectedUnfocused, SelectedDisabled, SelectedPointerOver, SelectedPressed} and
`InputModeStates` {InputModeDefault, TouchInputMode, GameControllerInputMode} (read from
`ComboBox/ComboBox_themeresources.xaml`); precedence read in the value phase.

**may_be_absent:** `placeholder`, `icon`, `arrow_area`, `header`, `floating_label`, `supporting_text`,
`item_mark`, `item_pill`, `body.shadow`; values `item_mark_position`, `header_gap`, `supporting_text_gap`,
`max_visible_items`.

**State-model source:** `windows-11` the ComboBox and ComboBoxItem visual state groups above;
`adwaita` `_dropdowns.scss`, `_buttons.scss` (the trigger is a toggle button: `:checked` while open),
`_deprecated.scss`; `kde-breeze` `breezestyle.cpp` combo-box branches (editable, frame, mouse over, focus,
sunken/open, neutral) and the helper's button frame; qqc2 `ComboBox.qml` for `kirigami`; `material-3` the
outlined and filled select tokens; `macos-*` `NSPopUpButton` (pulls down, bordered, control size, key
window) and `NSComboBox`; `ios-*` `UIButton.Configuration` with its indicator and menu states.

**Today's fields:** `background_color` → `body.fill`; `font` → `label.text.font` (colour
`label.text.fill`); `min_height_px`, `min_width_px` → `body.min_height`, `body.min_width`;
`arrow_icon_size_px` → `arrow.glyph.size`; `arrow_area_width_px` → `arrow_area.min_width` (absent where no
area is drawn); `disabled_opacity` → `body.opacity` under `{disabled}`; `hover_background` → `body.fill`
under `{hover}`; `disabled_background`, `disabled_text_color` → `body.fill`, `label.text.fill` under
`{disabled}`; `border.*` → `body.stroke`, `body.corners`, `body.padding`, `body.shadow`.

- **Merged from other groups and rows:**
  - Group 1: the popup's placement is `values.popup_placement` (GN-P4-2†, K-P3-5†, MAC-P4-8†; group 1 wrote `values.placement`).
  - kirigami-addons: keyed KDE `@dialog_mode`, `@page_mode` (the form combo row's display modes, K-KA-12†).

### slider

*From group 3's catalogue.*

**Kind:** typed widget (all six; the range slider as a variant).

**Parts:**

| Part | What it is |
|---|---|
| `body` | the widget box: padding round the track, container fill per state (WinUI), opacity (GTK's disabled filter) |
| `track` | the inactive track (trough) |
| `fill` | the active track (value fill, GTK's highlight) |
| `fill_level` | GTK's `show-fill-level` second trough |
| `disabled_range` | the track outside iOS 26's `enabledRange` |
| `thumb` | element: the handle (two under `range`): shape by corners and size, transform (GNOME's teardrop rotation, iOS 26's growing lens), shadow (incl. a rim ring as a spread layer), stroke, focus indicator |
| `thumb_inner` | WinUI's inner accent dot |
| `halo` | Material's state-layer halo round the handle |
| `tick` | element: a tick mark (its fill depends on whether it lies within the fill) |
| `stop_indicator` | element: Material's stop dot |
| `value_label` | GTK's `draw-value` label |
| `value_indicator` | the value bubble shown while dragging (Material's value indicator, WinUI's thumb tooltip) |
| `header` | WinUI's header |
| `minimum_image`, `maximum_image` | iOS's minimum- and maximum-value images |
| `inset_icon` | Material Expressive's icon inside the track |

**Widget values:**

| Value | Type | Rows |
|---|---|---|
| `tick_gap` | length (ticks to track) | GN-1.100, K1.103, MAC-SL04, SL-6 |
| `draw_ticks` | `Value<bool>` (Breeze's option, read) | K1.103 |
| `value_label_gap` | length | GN-1.102 |
| `value_label_position` | `LabelPosition` | GN-1.102, GN-1.107 |
| `value_indicator_gap` | length | MAT-S6 |
| `value_indicator_placement` | `PlacementStrategy` + params | W1-073 |
| `thumb_track_gap` | length (Material's handle–track gap) | MAT-S2 |
| `stop_indicator_gap` | length | MAT-S4 |
| `image_gap` | length | SL-5 |
| `header_gap` | length | W1-072 |
| `value_transition` | `Transition` | SL-9 |
| `snap_to_ticks` | `Value<bool>` | SL-9 |
| `presentations` | `Vec<PresentationTransition>` (value indicator show and hide; a rule under `reduced_motion`) | MAT-S6 |

**Variants.** Typed: `vertical` (GNOME, Material, WinUI, macOS), `range` (KDE Quick RangeSlider, Material,
WinUI via CTK), `centered` (fill from a neutral value: Material's centred slider, macOS `neutralValue`),
`ticks_before`, `ticks_after` (marks on one side, both for both sides: GNOME marks, macOS
`tickMarkPosition`, KDE, WinUI `TickPlacement`). Keyed: `windows-11` `@ticks_inline`; `ios-*`
`@thumbless`.

**Sizes and densities.** `macos-*`: mini, small, regular, large. `material-3`: the five Expressive sizes
(xsmall to xlarge; mapping to `ControlSize` is ADD-A62).

**Widget-specific flags.** `in_fill` (a tick or stop indicator inside the value fill: K1.103, MAT-S4,
MAT-S5), `overlapping` (a range thumb overlapping the other, MAT-S10, as R3 names). Keyed `adwaita`
`@fine_tune` (also on `scrollbar`). States: hover, pressed, focused, focus visible, disabled.

**Exclusive groups.** `windows-11`: Slider `CommonStates` {Normal, PointerOver, Pressed, Disabled} and
`FocusEngagementStates` {FocusDisengaged, FocusEngagedHorizontal, FocusEngagedVertical}.

**may_be_absent:** `fill_level`, `disabled_range`, `thumb` (`@thumbless`), `thumb_inner`, `halo`,
`stop_indicator`, `value_label`, `value_indicator`, `header`, `minimum_image`, `maximum_image`,
`inset_icon`, `thumb.stroke`, `thumb.shadow`; values `value_label_*`, `value_indicator_*`,
`thumb_track_gap`, `stop_indicator_gap`, `image_gap`, `header_gap`, `draw_ticks`.

**State-model source:** `windows-11` Slider visual states (`CommonStyles/Slider_themeresources.xaml`);
`adwaita` `_scale.scss` (`:hover`, `:active`, `:disabled`, `.marks-before`/`.marks-after`, `.fine-tune`,
vertical, high contrast); `kde-breeze` `breezestyle.cpp` slider branches and `breezehelper.cpp` handle
rendering (mouse over, pressed, focus, neutral), breezerc's tick option, qqc2 `Slider.qml` and
`RangeSlider.qml` for `kirigami`; `material-3` the slider tokens (baseline and Expressive sizes);
`macos-*` `NSSlider` (tick mark position, control size, `tintProminence`, `neutralValue`, key window);
`ios-*` `UISlider` (`Style`, `TrackConfiguration`, `UIControl.State`).

**Today's fields:** `fill_color` → `fill.fill`; `track_color` → `track.fill`; `thumb_color` →
`thumb.fill`; `track_height_px` → `track.min_height` = `max_height` (and `fill`'s); `thumb_diameter_px` →
`thumb.min_width`, `thumb.min_height` with `thumb.corners` full; `tick_mark_length_px` →
`tick.min_height`; `disabled_opacity` → `body.opacity` under `{disabled}`; `thumb_hover_color` →
`thumb.fill` under `{hover}` (or `context any(slider {hover})` where the whole slider's hover drives it,
GTK); `disabled_fill_color`, `disabled_track_color`, `disabled_thumb_color` → `fill.fill`, `track.fill`,
`thumb.fill` under `{disabled}`.

- **Merged from other groups and rows:**
  - Group 6: keyed `@hue`, `@alpha`, `@value` in `context colour_picker` (GN-2.53†, W2-05).
  - Group 8: `thumb.shadow` (K3.4); `values.repeat_delay`, `values.repeat_interval`, `values.trough_repeat` (K3.76†, K-P3-17†, MAC-X12).
  - Group 1: `values.trough_click` (GN-P4-5†, R17).

## Indicators

### spinner

*From group 3's catalogue.*

**Kind:** typed widget (indeterminate busy indicator, and the determinate circular indicator where the
platform draws one: AdwSpinner and GtkSpinner, KBusyIndicatorWidget and qqc2 BusyIndicator, macOS's
spinning `NSProgressIndicator`, `UIActivityIndicatorView`, WinUI ProgressRing, Material's circular
progress indicator).

**Parts:**

| Part | What it is |
|---|---|
| `body` | the element's box: size limits, opacity (disabled) |
| `track` | the full circle under the arc (AdwSpinner, Material, WinUI's background ring) |
| `indicator` | the arc (indeterminate) or the value arc / pie (determinate); its `fill` paints the stroke |
| `spoke` | a spoke (iOS) or fin (macOS); its fill is the leading spoke's colour |
| `icon` | the rotated or animated icon (KDE's `process-working`, GtkSpinner's animated SVG and its reduced-motion replacement) as a glyph |

**Widget values:**

| Value | Type | Rows |
|---|---|---|
| `diameter` | length (the default size) | today's `diameter_px`, Q1.120 |
| `stroke_width` | length | today's `stroke_width_px`, MAT-SN1, MAT-SN3, Q1.121 |
| `stroke_ratio` | number: stroke as a fraction of the radius, applied by the toolkit at layout where the platform derives it from the size | GN-1.227 |
| `stroke_weight` | `Value<u16>`: the icon weight GTK scales the stroke by | GN-1.227, GN-1.232 |
| `track_stroke_width` | length | MAT-SN3 |
| `indicator_track_gap` | length | MAT-SN1, MAT-SN3 |
| `cap` | `LineCap` (ADD-A34) | GN-1.227, MAT-SN4, SP-1a, MAC-SN01 |
| `spoke_count` | integer | SP-1a, MAC-SN01 |
| `spoke_inner_radius`, `spoke_length`, `spoke_width` | length | SP-1a, MAC-SN01 |
| `spoke_opacity_ramp` | `Vec<Value<f32>>` (per spoke behind the leading one) | SP-1a, MAC-SN01 |
| `wave_amplitude`, `wavelength` | length (`@wavy`) | MAT-SN2 |
| `loop` | `Loop` (S2.7): the indeterminate motion (arc extension and contraction, rotation, steps, four-colour cycle); a rule under `animations_off` stops it | GN-1.229, MAT-SN6, SP-2a, MAC-SN02, K1.230, Q1.119, Q1.120, W1-130, MAT-SN5 |
| `value_transition` | `Transition` (determinate value change) | MAT-SN6 |
| `presentations` | `Vec<PresentationTransition>` (fade in on start, out on stop) | SP-5a, MAC-SN05, Q1.120 |

**Variants.** Typed: `determinate` (macOS, WinUI, Material). Keyed: `adwaita` `@gtk_spinner` (GtkSpinner,
GN-1.230); `material-3` `@wavy`, `@thick`, `@four_colour`.

**Sizes and densities.** `macos-*`: mini, small, regular (MAC-SN03). `ios-*`: regular (`medium`), large
(SP-3a). Others: none (sizes are `body` limits and `diameter`).

**Widget-specific flags.** None. States: disabled.

**Exclusive groups.** `windows-11`: ProgressRing `CommonStates` {Inactive, DeterminateActive, Active}.

**may_be_absent:** `track`, `spoke`, `icon`, `indicator`; values `stroke_ratio`, `stroke_weight`,
`track_stroke_width`, `indicator_track_gap`, `spoke_*`, `wave_*`, `value_transition`. (`ios-*` draws no
icon, SP-4a; KDE draws neither arc nor spokes.)

**State-model source:** `windows-11` `ProgressRing/ProgressRing.xaml` states; `adwaita`
`adw-spinner-paintable.c`, `_spinner.scss`, `gtkspinner.c` (reduced motion); `kde-breeze`
`kbusyindicatorwidget.cpp`, qqc2 `BusyIndicator.qml` (running, visible); `material-3` the circular
progress-indicator tokens (determinate, indeterminate, Expressive variants); `macos-*`
`NSProgressIndicator` (style spinning, indeterminate, control size, displayed when stopped); `ios-*`
`UIActivityIndicatorView.Style`, animating, `hidesWhenStopped`.

**Today's fields:** `diameter_px` → `values.diameter`; `min_diameter_px` → `body.min_width`,
`body.min_height`; `stroke_width_px` → `values.stroke_width`; `fill_color` → `indicator.fill` (arc),
`spoke.fill` (spokes) or `icon.glyph.fill` (icon), per platform. The animated-icon model
(`AnimatedIcon`, `TransformAnimation::Spin`) becomes `values.loop` with `icon.glyph`.

- **Merged from other groups and rows:**
  - Group 7: Material's loading indicator, keyed `@loading_indicator` and `@contained` (container colour, size and shape, with a `Loop` of shape keyframes: ADD-B03), MAT-N24; `body` is its container (group 7 wrote `container`).
  - Group 6: iOS's determinate circular progress (C-31) is the typed `determinate` here (group 6 had proposed `progress_bar` variant `circular`).
  - Group 8: GtkSvg's animated states are `icon.glyph` with `values.loop` (GN-P3-19†).

### progress_bar

*From group 3's catalogue.*

**Kind:** typed widget (all six; plus KDE's `KCapacityBar` as a variant).

**Parts:**

| Part | What it is |
|---|---|
| `body` | the element's box: padding (Kirigami's vertical insets), size limits |
| `track` | the trough |
| `indicator` | the value fill, and the moving indicator when indeterminate |
| `secondary_indicator` | the second moving indicator of an indeterminate bar (WinUI, Material) |
| `stop_indicator` | Material's end dot |
| `buffer` | Material's buffer (secondary progress) bar |
| `buffer_dots` | the dots of Material's buffer track |
| `label` | a title label (SwiftUI's `label`) |
| `value_label` | the value text (GTK `show-text`, KDE's percentage, SwiftUI's `currentValueLabel`) |

**Widget values:**

| Value | Type | Rows |
|---|---|---|
| `min_indicator_length` | length: the least fill a non-zero value shows | K1.114 |
| `indicator_track_gap` | length | MAT-P2 |
| `stop_indicator_gap` | length | MAT-P3 |
| `wave_amplitude`, `wavelength` | length (`@wavy`) | MAT-P1 |
| `pulse_block_fraction`, `pulse_step` | number (GTK's pulse block and step) | GN-1.113 |
| `indeterminate_loop` | `Loop` | GN-1.113, K1.110, MAC-P01, W1-075, MAT-P6 |
| `value_transition` | `Transition` | PB-3, MAC-P04, W1-078, MAT-P6 |
| `label_position` | enum `LabelPosition { Above, Below, Inline, Leading, Trailing }` | K1.115, PB-4, MAC-P3-4, K1.113 |
| `label_gap`, `label_alignment` | length; enum alignment | K1.113, PB-4, MAC-P3-4 |

**Variants.** Typed: `vertical` (GNOME, Material, WinUI). Keyed: `adwaita` `@osd`; `kde-breeze`
`@capacity`, `@capacity_inline`; `ios-*` `@bar`; `material-3` `@wavy`, `@thick`, `@four_colour`.

**Sizes and densities.** `macos-*`: control sizes (regular, small, mini where AppKit draws them). Others:
none.

**Widget-specific flags.** `empty` (value zero: GTK hides the fill node). Keyed `windows-11` `@paused`,
`@error`. States: indeterminate, disabled.

**Exclusive groups.** `windows-11`: ProgressBar `CommonStates` {Normal, Determinate, Updating,
UpdatingError, Error, Paused, Indeterminate, IndeterminateError, IndeterminatePaused}.

**may_be_absent:** `secondary_indicator`, `stop_indicator`, `buffer`, `buffer_dots`, `label`,
`value_label`, `track.stroke`; values `min_indicator_length`, `indicator_track_gap`, `stop_indicator_gap`,
`wave_*`, `pulse_*`, `value_transition`, `label_*`.

**State-model source:** `windows-11` `ProgressBar/ProgressBar.xaml` states; `adwaita`
`_progress-bar.scss` (`.osd`, vertical, `show-text`, empty) and `gtkprogressbar.c` (pulse);
`kde-breeze` `breezestyle.cpp` progress-bar branches (busy, selected item, label) with breezerc's animation
options, qqc2 `ProgressBar.qml`, `kcapacitybar.cpp`; `material-3` the linear progress-indicator tokens;
`macos-*` `NSProgressIndicator` (bar style, indeterminate, control size, key window); `ios-*`
`UIProgressView.Style` and SwiftUI `ProgressView`.

**Today's fields:** `fill_color` → `indicator.fill`; `track_color` → `track.fill`; `track_height_px` →
`track.min_height` = `track.max_height`; `min_width_px` → `body.min_width`; `border.*` → `track.stroke`,
`track.corners` (the indicator's corners become `indicator.corners`), `body.padding`.

- **Merged from other groups and rows:**
  - Group 4: GtkTreeView's cell progress bars in `context list.cell` (GN-1.279†).

### scrollbar

*From group 3's catalogue.*

**Kind:** typed widget (all six; iOS and Material draw a non-interactive indicator).

**Parts:**

| Part | What it is |
|---|---|
| `body` | the bar's box: thickness, padding (the trough's margins and the indicator's inset from the view's edges) |
| `track` | the trough: fill (WinUI's acrylic), per-side stroke (the content-side separator line, KDE and legacy macOS), corners, padding (the thumb's inset) |
| `thumb` | element: the knob / indicator: fill, stroke (outline, knob-style border), corners, size limits, opacity |
| `arrow_button` | element: an arrow button (KDE's optional ones, WinUI's), its glyph, fill, stroke, pressed scale |

The corner square and the view frame belong to `scroll_view` (below).

**Widget values:**

| Value | Type | Rows |
|---|---|---|
| `auto_hide_delay` | duration (WinUI's contract delay) | GN-1.91, SB-3, MAT-SC1, MAT-SC3, Q1.41, W1-064 |
| `hide_after_leave` | duration | GN-1.91 |
| `presentations` | `Vec<PresentationTransition>` (fade out, slide in, iOS's flash on appearance as a keyed event) | GN-1.91, SB-3, MAT-SC1, MAT-SC3, Q1.41 |
| `always_visible` | `Value<bool>` | MAT-SC3 |
| `thumb_max_fraction` | number (of the viewport) | MAT-SC3 |
| `overscroll_shrink` | `Value<bool>`: the thumb shortens by the overshoot | SB-7, MAC-P3-3, K-P4-13 |
| `arrow_buttons_start`, `arrow_buttons_end` | enum `ArrowButtons { None, Single, Double }` (read on KDE) | K1.94 |
| `trough_click` | enum `TroughClick { Page, JumpToPosition }` (read on KDE and macOS) | K-RV1, MAC-P3-2, Q1.39, W-P4-16 |
| `repeat_delay`, `repeat_interval` | duration | Q1.42, MAC-P3-2, W-P4-16 |

**Variants.** Typed: `vertical` (base horizontal), `dark`, `light` (a knob style for light or dark content:
macOS `KnobStyle` dark and light, iOS `IndicatorStyle` black and white; the default adapts by the
`over_dark_content` state).

**Conditions used.** `overlay_scrollbars` (ADD-A11), `transient_touch` (Kirigami's touch
overlay), contrast, flavour.

**Sizes and densities.** `macos-*`: regular, small, mini (MAC-S07). Others: none.

**Widget-specific flags.** Keyed `adwaita` `@fine_tune`. States: hover (of the bar, and of the thumb),
pressed, disabled, over dark content.

**Exclusive groups.** `windows-11`: ScrollBar `CommonStates` (per template part), `ScrollingIndicatorStates`
{TouchIndicator, MouseIndicator, NoIndicator}, `ConsciousStates` {Collapsed, Expanded,
ExpandedWithoutAnimation, CollapsedWithoutAnimation}; the expanded state is `{hover}` on the bar, its delay
`auto_hide_delay`.

**may_be_absent:** `arrow_button`, `track.stroke`, `thumb.stroke`, `track.fill` (indicators without a
track); values `hide_after_leave`, `thumb_max_fraction`, `arrow_buttons_*`, `repeat_*`.

**State-model source:** `windows-11` `CommonStyles/ScrollBar_themeresources.xaml` visual states;
`adwaita` `_scrolling.scss` (`.overlay-indicator`, `.hovering`, `.fine-tune`, `:hover`, `:active`,
`:disabled`, high contrast) and `gtkscrolledwindow.c` (timings); `kde-breeze` `breezestyle.cpp`
scroll-bar branches (groove hover, thumb hover, focus of the view, arrows) with the scrollbar engine, qqc2
`ScrollBar.qml` (interactive and touch overlay); `material-3` Compose's `nonInteractiveScrollbar` and
`ViewConfiguration`; `macos-*` `NSScroller` (scroller style, knob style, control size, key window);
`ios-*` the scroll indicator (`indicatorStyle`, grabbed, iOS 26 edge effect).

**Today's fields:** `track_color` → `track.fill`; `thumb_color` → `thumb.fill`; `thumb_hover_color` →
`thumb.fill` under `{hover}`; `thumb_active_color` → `thumb.fill` under `{pressed}`; `groove_width_px` →
`body.min_height` (horizontal) and `body.min_width` under `vertical`; `thumb_width_px` → `thumb.min_height`
and `thumb.min_width` under `vertical`; `min_thumb_length_px` → `thumb.min_width` and `thumb.min_height`
under `vertical`; `overlay_mode` → the condition `overlay_scrollbars` (ADD-A11).

- **Merged from other groups and rows:**
  - Group 7: keyed Windows `@annotated` (W2-28): parts `annotation_labels` (the labelled rail) and `annotation_tooltip` (min height, max width); its arrow buttons are `arrow_button`.
  - Group 8: `values.auto_hide_delay` (MAC-X12), `values.repeat_delay`, `values.repeat_interval`.
  - Group 1: `values.trough_click` (GN-P4-5†).

## Containers

### scroll_view

*From group 3's catalogue.*

**Kind:** typed widget, added by this group: the scrolling container round a viewport. KDE, macOS and
Windows draw its frame and corner (Q1.43, MAC-I08, K1.96, MAC-S06, W1-065); GNOME's undershoot and Apple
26's scroll edge effect are its `edge_effect`. (Group 7 lists the iOS scroll edge effect, C-18, as a keyed
component and group 8 macOS's, MAC-X05: two platforms, so typed here, R14.)

**Parts:** `frame` (border and background: KDE's framed background, `NSScrollView.borderType`), `corner`
(the square where two bars meet), `edge_effect` (the shade or material at a scrolled edge: GTK undershoot,
Apple 26's soft or hard edge).

**Widget values:** `scrollbar_margin` (`Sides<Value<f32>>`, WinUI, W1-065).

**Variants.** Typed: `flat`, `line` (as `text_area`). Keyed: `kde-breeze` `@framed`
(`StyleHints.showFramedBackground`); `macos-*` `@bezel`, `@groove`.

**Sizes, densities, flags.** None. States: focus within (KDE's thumb follows the view's focus, K1.92).

**may_be_absent:** `frame`, `corner`, `edge_effect`; `scrollbar_margin`.

**State-model source:** `kde-breeze` the frame and corner branches of `breezestyle.cpp`, qqc2
`ScrollView.qml`; `macos-*` `NSScrollView` (`borderType`, scroller style); `windows-11`
`CommonStyles/ScrollViewer_themeresources.xaml` and `ScrollView_themeresources.xaml`; `adwaita` the
`undershoot` nodes of `scrolledwindow`; `ios-*` the scroll edge effect styles.

**Today's fields:** none.

- **Merged from other groups and rows:**
  - Group 7's typed `scroll_edge_effect` (iOS C-18, macOS MAC-X05) and group 8's `scroll_area` are this widget: `edge_effect` is a `Material` paint with blur and a fade mask (a `Linear` gradient over a blur); typed variants `soft_edge`, `hard_edge` (UIKit's and AppKit's styles; `automatic` is the application's choice, not a look); values `edge_effect_height: Value<f32>`, `edge_effect_edges` (top, bottom, leading, trailing); GNOME's overshoot and undershoot gradients (GN-3.06); the bar it lies under carries `scrolled`. `edge_effect` may be absent (Windows, KDE and Material draw none). State-model source: UIKit's `UIScrollEdgeEffect.Style` and its AppKit equivalent (group 7).

### list

*From group 4's catalogue.*

- **Kind:** typed widget. Every platform draws it (list, table, tree, grid/icon view): GNOME (GtkListView,
  GtkColumnView, GtkListBox, AdwSidebar's rows, deprecated GtkTreeView and GtkIconView), KDE (Qt item views,
  qqc2 ItemDelegate family, Kirigami delegates, KIO places view), macOS (NSTableView, NSOutlineView, source
  lists), iOS (UICollectionView lists, UITableView, SwiftUI `Table` and `Form`), Material (lists, data table),
  Windows (ListView, GridView items, NavigationView items through the variant `sidebar`). Trees are lists with
  disclosures (R14's cross-check); the "tree view", "grid view", "items view", "list box" and "swipe actions"
  kinds group 7 lists (W2-12, W2-13, W2-14, W2-33, W2-29, W-P4-11†) are this widget by the same test.
- **Parts:**
  - `view` — the list's own surface: fill (binds the `View` surface set), frame stroke (MAC-L16, MAT-L13),
    corners and shadow when the whole list is a card (GNOME `.boxed-list`), padding; its `{disabled}` fill
    (GN-1.176).
  - `section` — the platter that holds one section's rows (iOS inset-grouped, L-2): fill, corners, stroke,
    shadow.
  - `row` — **element**, one item: padding, min height (Auto where content-sized, L-9), the primary text,
    opacity (per-item disabled fade, W1-112), transform (reorder scale, W1-110), transitions; its focus
    indicator lives on `row_background`.
  - `row_background` — the shape drawn for the row's states inside the row (GNOME's whole-row fill, KDE's
    inset bubble, macOS's inset selection, WinUI's rounded backplate, Material's container and Expressive
    shapes): fill, stroke (KDE's outline, K1.171), corners (per corner, so merged selections round only the
    ends, MAC-L02), shadow (separate boxed-list cards, dragged elevation), focus indicator.
  - `row_selection_indicator` — WinUI's selection pill (W1-107; W1-089 in the sidebar variant): fill, corners,
    size limits, transitions. may_be_absent.
  - `row_separator` — the line between rows (GN-1.162, L-3, MAT-L6, boxed-list separators GN-1.165).
  - `column_separator` — the vertical line between cells (GN-1.169, MAC-L06).
  - `cell` — **element** in a multi-column row: padding, corners (KDE rounds the selection on a row's first and
    last cells only, by position among cells, K1.170), fill (KDE paints the selection per cell; a shaded
    sort column, K1.183), stroke (GNOME's editable-cell focus and editing outlines, GN-1.172).
  - `header` — the header row: fill, bottom stroke (the header's bottom line), min height.
  - `header_section` — **element**, a column header button: fill, text, padding, transitions (hover fade),
    opacity and transform while dragged (MAC-L15, GN-1.171).
  - `header_separator` — the line between header sections (K1.178, MAC-L05, GN-1.169).
  - `header_corner` — the corner piece beside the header above the vertical scroll bar (K1.178, MAC-L18†).
  - `sort_indicator` — glyph in a sorted header section (GN-1.168, K1.179, MAC-L05 named images
    `NSAscendingSortIndicator`/`NSDescendingSortIndicator`, MAT-L12, L-22†).
  - `section_header` — **element**, a section's heading or group row (GN-1.163, L-4, MAC-L09, MAC-SB03,
    SD-4, MAT-SB4, W1-093, Q1.63): text, fill (sticky plain header, group-row material), padding, min height.
  - `section_footer` — a section's footer text (L-5).
  - `footer` — a table's footer bar (MAT-L12): fill, min height, text. may_be_absent.
  - `disclosure` — **element**, a tree row's expander arrow or an expandable item's chevron (GN-1.173, K1.176,
    MAC-L10, L-21†, Q1.85, MAT-L9, W1-093): glyph, fill and corners (Material's chevron container),
    transform (rotation) with its transition.
  - `branch_line` — tree branch lines (K1.176, Q1.85, GN-1.279†). may_be_absent.
  - `leading_icon` — the row's leading image, icon or avatar (L-8, SD-5, MAT-L2, Q1.80, Q1.87, K1.181,
    MAC-SB01/02): icon style, size, corners.
  - `secondary_text` — subtitle, supporting or value text (L-7, MAT-L1, Q1.87, MAC-L04).
  - `trailing_text` — trailing supporting text or a value cell's trailing value (MAT-L1, L-7, MAT-SB5).
  - `overline_text` — Material's overline (MAT-L1). may_be_absent.
  - `trailing_icon` — Material's trailing icon (MAT-L3).
  - `accessory` — **element**, a system accessory with a keyed identity carried as a variant
    (`@disclosure_indicator`, `@checkmark`, `@detail`, `@delete`, `@insert`, `@reorder`, `@multiselect`,
    `@outline_disclosure`, `@label`, `@popup_menu`; L-12): glyph, fill, size.
  - `selection_check` — the multi-select check the list draws itself (WinUI's presenter checkbox W1-109;
    iOS's multiselect circle is `accessory @multiselect`): fill, stroke, corners, glyph.
  - `swipe_area` — the area a swipe reveals (Q1.89, L-13): fill, shadow at its edges.
  - `swipe_action` — **element**, an action button there (L-13, MAC-L12, MAT-L11, Q1.88): fill, text, icon,
    corners, transitions.
  - `swipe_handle` — Kirigami's handle that opens the actions (Q1.88). may_be_absent.
  - `drop_indicator` — the insertion line (and its end circle) between rows (MAC-L11, K1.182, GN-1.279†,
    W1-110): stroke, glyph (the end circle), corners.
  - the marquee selection rectangle is the `rubber_band` widget in `context list` (GN-1.175, GN-1.280†).
  - `placeholder` — the not-yet-loaded item (W-RV-2†): fill, size limits. may_be_absent.
  - `section_index` — **element**, iOS's A–Z index (L-15): text, fill; its tracking background is its
    `{pressed}` fill. may_be_absent.
  - `undershoot` — the scroll-edge shade under a column view's header (GN-1.178): fill (gradient), stroke.
    may_be_absent.
- **Widget values:** `row_background_inset: Sides<Value<f32>>` (rules by position: K1.170's larger first-row
  top); `row_gap: Value<f32>` (GN-1.166, Q1.79, MAT-L6, MAT-L8); `intercell_gap: (Value<f32>, Value<f32>)`
  (MAC-L07); `row_separator_inset: Sides<Value<f32>>` and `row_separator_aligns_to_text: Value<bool>` (L-3);
  `section_separators: { top: Value<bool>, bottom: Value<bool> }` (L-3); `section_inset: Sides<Value<f32>>`
  (L-2, MAC-L01, MAC-SB04, SD-2); `indent_per_level: Value<f32>` (GN-1.173, K1.176, MAC-L10, L-21†, Q1.85);
  `indent_includes_disclosure: Value<bool>` (GN-1.279†); `separators_follow_indent: Value<bool>` (L-21†);
  `disclosure_gap: Value<f32>`; `leading_gap: Value<f32>` (MAT-L7, Q1.87, L-7, W1-092);
  `secondary_text_gap: Value<f32>` (L-7); `trailing_gap: Value<f32>`;
  `secondary_text_stacks: Value<bool>` (accessibility-size layout, L-19, rule under the large text sizes);
  `section_headers_sticky: Value<bool>` (L-4); `filler_rows: Value<bool>`, `filler_row_height: Value<f32>`
  (L-17); `header_fills_trailing: Value<bool>` (Q1.86); `header_item_gap: Value<f32>` (K1.179);
  `header_text_alignment: Value<Alignment>` (MAC-L05); `sort_indicator_inverted: Value<bool>` (K1.179, read);
  `focus_visible_when: Value<FocusVisibleWhen>` (ADD-A16; Q1.78, K1.175, MAC-L14); `swipe_full_expansion:
  Value<bool>` (L-13); the dragged row's image is the `drag_image` widget in `context list` (K-RL06†, W1-110); `reorder_hint_offset: Value<f32>` (W1-110); `drag_hover_expand_delay_ms: Value<f32>`
  (GN-1.279†); `typeahead_hide_ms: Value<f32>` (GN-1.279†'s interactive search); `elided_tooltip:
  Value<bool>` (Q1.82: an elided row shows its full text in a tooltip after the tooltip's own
  `show_delay_ms`, by press and hold under `cond tablet_mode`); presentation transitions
  keyed by event: `insert`, `delete`, `reload`, `reorder`, `edit_mode`,
  `expand`, `collapse`, `swipe`, `drag_lift`, `selection_move` (WinUI's pill, W1-090), `placeholder_fade`
  (W-RV-2†) (L-14, L-16, W1-113, K1.185†, Q1.84, Q1.89); the deselection fade (L-11) and GN-1.165's row transitions are property transitions on `row_background`.
- **Variants:** typed: `plain` (iOS plain, macOS plain), `inset` (iOS insetGrouped, macOS inset, GNOME
  `.boxed-list`), `separated` (GNOME `.boxed-list-separate`, Material Expressive segmented list),
  `sidebar` (iOS sidebar, macOS source list, GNOME `.navigation-sidebar`/AdwSidebar, WinUI NavigationView
  items, Material drawer items, KDE drawer items and side panels), `table` (multi-column with a header:
  GNOME column view, KDE table views, macOS NSTableView, iOS `Table`, Material data table), `grid` (GNOME grid
  view, flow box and icon view; KDE icon mode; WinUI GridView), `row_separators` (GNOME `.separators`, macOS
  horizontal grid, Material dividers), `column_separators` (GNOME `.column-separators`, macOS vertical grid),
  `alternating_rows` (KDE, macOS), `destructive` on `swipe_action` (iOS, macOS). Keyed: GNOME `@rich_list`,
  `@data_table`; iOS `@grouped`, `@sidebar_plain`, `@prominent_header`, `@extra_prominent_header` (on
  `section_header`), `@value_cell`, `@subtitle_cell`, `@accompanied_sidebar_cell`, the `accessory` identities;
  GNOME `@tree_view`, `@icon_view` (the deprecated GtkTreeView and GtkIconView, GN-1.279†, GN-1.280†); KDE
  `@column_view` (QColumnView, K1.184†), `@with_subtitle` (Q1.87) (KDE's container styles `@places`,
  `@side_panel`, `@global`, `@context` are the sidebar's variants, matched as `context sidebar.pane[@places]`);
  Material `@avatar`, `@image`, `@video`
  (the leading element's kind, MAT-L2); macOS `@full_width`, `@automatic`, `@row_size_small`, `@row_size_medium`, `@row_size_large` (MAC-L08; the
  rationale's "a list stating its own row size style has that as a variant"), `@dashed_horizontal_grid`;
  KDE `@itemrow` is not needed (alternation is `alternating_rows`); Material `@two_line`, `@three_line`.
- **Sizes and densities:** macOS: row size by variant (above) and by the condition `container_size(sidebar,
  small|medium|large)` for source lists at the default row size (MAC-SB01); iOS: none (Dynamic Type, L-9:
  `row.min_height` Auto with a minimum); Material: none in lists; Windows, GNOME, KDE: none.
- **Widget-specific flags:** `@sorted` (header_section; K1.177, MAC-L18†), `@sort_column` (cell; K1.183),
  `@editing` (row, cell; GN-1.172, L-10, L-14), `@swiped` (row; L-10, Q1.84), `@context_menu` (row; the
  row whose popup or context menu is open: GN-1.160 `.has-open-popup`, MAC-L19†; kept apart from `open`,
  which a tree row uses for expanded), `@placeholder` (row; W-RV-2†), `@reorder_target` (W1-110), `@hidden_place` (row; KDE's hidden places,
  K1.142†). Typed flags
  used: hover, pressed, focused, focus_visible, focus_within, selected, checked, disabled, open (expanded),
  dragged, drop_target.
- **Exclusive groups:** Windows: `[disabled, pressed, hover]` (ListViewItemPresenter's common states);
  Material: `[dragged, pressed, focus_visible, hover]` (one state layer at a time).
- **may_be_absent:** `section`, `row_selection_indicator`, `column_separator`, `header_corner`, `footer`,
  `branch_line`, `overline_text`, `trailing_icon`, `accessory`, `selection_check`, `swipe_handle`,
  `placeholder`, `section_index`, `undershoot`, `section_footer`; `row_background.stroke`,
  `row_background.shadow`, `row_separator` (iOS sidebar, KDE), `row_background.fill @ position odd variant
  alternating_rows` (iOS has no alternation, L-18: no rule), `filler_rows`.
- **State-model source:** GNOME: libadwaita selectors in `_lists.scss`, `_column-view.scss`, `_views.scss`,
  `_sidebars.scss`, `_deprecated.scss` (tree and icon views); KDE: Breeze's `PE_PanelItemViewItem`,
  header and branch branches (`st:4751-4900`, `st:6808-6900`, `st:5229-5299`) and qqc2's
  `DefaultListItemBackground.qml`/`ItemDelegate.qml` states; macOS: `NSTableView.Style`, `rowSizeStyle`,
  `NSTableCellView.backgroundStyle` (normal, emphasized), key and main window, first responder;
  iOS: `UICellConfigurationState` (highlighted, selected, focused, disabled, editing, swiped, expanded, drag
  and drop states) per `UICollectionLayoutListConfiguration.Appearance`; Material: the list, data-table,
  list-expand, list-reorder and list-reveal tokens (rest, hover, focus, pressed, dragged, disabled, each
  selected); Windows: ListViewItem's and GridViewItem's visual state groups (CommonStates, DisabledStates,
  MultiSelectStates, DataVirtualizationStates, ReorderHintStates, DragStates) and NavigationViewItem's for
  the `sidebar` variant.
- **Today's fields:** `background_color` → `view.fill`; `item_font` → `row.text`; `alternate_row_background`
  → `row_background.fill @ position odd variant alternating_rows` (ADD-B06); `selection_background` →
  `row_background.fill @ {selected}`; `selection_text_color` → `row.text.fill @ {selected}`; `header_font` →
  `header_section.text`; `header_background` → `header.fill` (and `header_section.fill`); `grid_color` →
  `row_separator.stroke` and `column_separator.stroke`; `row_height_px` → `row.min_height`;
  `header_height_px` → `header.min_height`; `hover_background` → `row_background.fill @ {hover}`;
  `hover_text_color` → `row.text.fill @ {hover}`; `disabled_text_color` → `row.text.fill @ {disabled}`;
  `border` → `view.stroke`, `view.corners`, `view.shadow` (from `shadow_enabled`), `row.padding` (its
  `padding_*`, which presets used for the row).

- **Merged from other groups and rows:**
  - Group 6: Kirigami's `section_header` with `section_header_line`, and `inline_header` (Q2.41, Q2.42); the marquee is the `rubber_band` widget in `context list` (GN-1.175, GN-1.280†; group 4 had a `rubberband` part).
  - Group 7: the column browser (MAC-N08): `disclosure` as the branch arrow, `column_separator`, `column_resize_handle`; grid items are `row` and `row_background` under the typed `grid` (MAC-N13, W2-13, W2-14) with `row_inner_border` (WinUI's inner selection border) and `selection_check` (the grid item's corner check box); value `check_box_placement`; keyed Windows `@listbox` (W2-33); tree parts (W2-12): `disclosure` with its rotation, `branch_line`, `row_selection_indicator`, `indent_per_level`, `selection_check`, the compact density's row height; `drag_hover_expand_delay_ms` (W-P4-11†); `swipe_action` with the keyed flags `@pre_threshold`, `@post_threshold` and the event `reveal` (W2-29).
  - Group 8: alternating rows by the parity selector (Q3.13, ADD-B06); drop-target rules on `row_background` and `view` (GN-3.66†, MAC-X21, W3-49†).
  - Group 1: `row` in `context menu.body` (GN-1.79). Group 2: iOS's checkmark accessory is `accessory @ variant @checkmark` (CK-1).
  - The dragged row's image and count are the `drag_image` widget in `context list` (K-RL06†, W1-110; group 4 had a `drag_image` value here).
  - kirigami-addons: keyed KDE `@ka_rounded` (K-KA-38†, K-KA-39†, K-KA-46†), `@ka_table` (K-KA-36†, K-KA-37†), `@ka_tree` (K-KA-40†), `@ka_indicator` (K-KA-39†), `@bold`; part `unread_indicator` under the keyed flag `@unread`; keyed flags `@current` (row, cell), `@descending` (header section); value `column_min_width`.

### tab

*From group 4's catalogue.*

- **Kind:** typed widget (tabs attached to a page or a document strip): GNOME (GtkNotebook, AdwTabBar), KDE
  (QTabBar/QTabWidget, qqc2 TabBar), macOS (NSTabView), Material (primary and secondary tabs), Windows
  (TabView). iOS's `UITabBar` is the app-level bottom tab bar, a different kind: its rows go to
  `bottom_tab_bar`, a typed widget of its own.
- **Parts:**
  - `bar` — **element**, the strip: fill (paint with material: Mica Alt, W1-088), stroke (its base line or rule
    on the page side: GN-1.116, K1.127, MAT-TB5, GN-1.125's shade line), padding (GN-1.121, W1-085), min
    height, corners and shadow.
  - `tab` — **element**, one tab: fill, stroke (W1-086's gradient border on the sides and top; GN-1.135's
    high-contrast inset border), corners (K1.121 on the bar side only), padding, min and max width and height,
    opacity, shadow (dragged, GN-1.133), transitions (hover fade K1.132; GN-1.131's background transition).
  - `label` — text (font, elision: none on KDE and Material, fade at the edges on GNOME GN-1.129), and the
    focus indicator of kind `underline` where focus is a line under the label (K1.129).
  - `icon` — icon style per state (W1-081, K1.130, MAT-TB4).
  - `close_button` — **element** (GN-1.120, GN-1.130, K1.124, W1-082): fill, glyph, corners, size limits,
    opacity, transitions.
  - `add_button` — **element** (W1-083). may_be_absent.
  - `scroll_button` — **element**, overflow arrows (GN-1.119, K1.125, W1-083): fill, glyph, padding.
  - `separator` — the line between tabs (GN-1.126, K1.123, W1-084, MAC-TB06†); hidden beside a selected or
    hovered tab by position rules on the previous and next sibling (ADD-B07).
  - `indicator` — the selected tab's indicator (today's `active_indicator_*`; GNOME's notebook bar, also on
    hover GN-1.117; KDE's strip K1.120; Material's MAT-TB1/TB2): fill, corners, size limits.
  - `attention_indicator` — GNOME's attention line (GN-1.128). may_be_absent.
  - `drop_indicator` — the bar a drop would land at (GN-1.127). may_be_absent.
  - `overflow_edge` — KDE's tear indicator where tabs are cut off (K1.126). may_be_absent.
  - `pane` — the page the bar belongs to (K1.128, MAC-TB01, GN-1.122, W1-088): fill, stroke, corners
    (cut where the bar touches, by a rule under the bar's position), padding.
- **Widget values:** `tab_gap: Value<f32>` (negative where tabs overlap: Q1.50, K-P3-4†); `alignment:
  Value<Alignment>` (start, centre; K1.131, read); `bar_pane_overlap: Value<f32>` (MAC-TB02); `width_mode:
  Value<TabWidthMode>` (equal, size-to-content, compact; W1-085; GNOME's `base_width`, `pinned_width`
  GN-1.129); `expanding: Value<bool>` (K1.123); `indicator_edge: Value<EdgeSide>` (top, bottom, left, right;
  rules by the bar's position variant; today's `active_indicator_side`); `indicator_length:
  Value<IndicatorLength>` (label, tab; MAT-TB2); `attention_width_fraction: Value<f32>` (GN-1.128);
  `outward_corner_radius: Value<f32>` (today's `active_outward_corner_radius_px`, `@ {selected}`);
  `close_button_shown: Value<CloseButtonShown>` (always, on hover, selected or hover; W1-082) with the
  padding change as rules on `tab.padding` by `variant closable`; `overflow: Value<TabOverflow>` (scroll,
  scroll buttons; K-P3-4†, MAT-P4-6†, GN-1.119); `scroll_to_selected: Value<ScrollToSelected>` (centre,
  visible with a margin; MAT-P4-6†, K-P3-4†); `scroll_margin: Value<f32>`; `drag_hover_switch_delay_ms:
  Value<f32>` (GN-1.132, K1.133†); `focus_visible_when` (Q1.51); `icon_text_gap: Value<f32>` (K1.130, MAT-TB4); `separator_inset:
  Sides<Value<f32>>` (W1-084); presentation transitions keyed by event:
  `open`, `close`, `reorder`, `scroll`, `resize` (GN-1.131), `move` (K1.133†), `indicator_move` (MAT-TB6),
  `attention` (GN-1.128), `drag`, `tear_out` (W1-087, W-SF09†).
- **Variants:** typed on `bar`: `top`, `bottom`, `left`, `right` (bar position: GNOME notebook, KDE, macOS);
  typed on `tab`/`bar`: `document` (browser-style strip: AdwTabBar, KDE document mode, WinUI TabView; pane-
  attached is the default), `closable` (GNOME, KDE, Windows). Keyed: GNOME `@inline`, `@shrunk` (AdwTabBar),
  `@pinned`; KDE `@static` (K1.123); Material `@icon_and_label` (MAT-TB4); macOS `@border_none`, `@border_line`, `@border_bezel`, `@no_tabs`
  (MAC-TB03); Material `@secondary` (MAT-TB1), `@scrollable` (MAT-TB7).
- **Sizes and densities:** macOS: `mini`, `small`, `regular`, `large` (MAC-TB05); others none (WinUI's
  compact width mode is `width_mode`).
- **Widget-specific flags:** `@needs_attention` (GN-1.128); typed flags used: hover, pressed, selected,
  focus_visible, focus_within (AdwTabBar's focus on the tab box child, GN-1.134), disabled, dragged,
  drop_target (on `bar`).
- **Exclusive groups:** Windows: `[disabled, pressed, hover]` (TabViewItem CommonStates).
- **may_be_absent:** `add_button`, `attention_indicator`, `drop_indicator`, `overflow_edge`, `indicator`
  (macOS, Windows draw none), `separator` (macOS selected neighbours, GNOME notebook), `scroll_button`
  (qqc2, Material), `pane` (document strips), `label.focus`.
- **State-model source:** GNOME: `_notebook.scss`, `_tab-view.scss` selectors and `adw-tab.c`; KDE:
  Breeze's tab-shape branches (`hp:1299-1430`, `st:7021-7200`) and qqc2 `TabButton.qml`; macOS:
  `NSTabView` (pressed, selected, key and non-key window, `controlSize`, `tabPosition`, `tabViewBorderType`);
  Material: primary and secondary navigation-tab tokens (active, inactive × hover, focus, pressed);
  Windows: TabViewItem visual state groups in `TabView.xaml` (CommonStates, DisabledStates, CloseIconStates,
  DragStates).
- **Today's fields:** `background_color` → `tab.fill`; `active_background` → `tab.fill @ {selected}`;
  `active_text_color` → `label.text.fill @ {selected}`; `bar_background` → `bar.fill`; `min_width_px`,
  `min_height_px` → `tab.min_width`, `tab.min_height`; `item_gap_px` → `values.tab_gap`;
  `active_indicator_color` → `indicator.fill`; `active_indicator_width_px` → the indicator's thickness
  (`indicator.min_height`, or `min_width` on a vertical bar); `active_indicator_side` →
  `values.indicator_edge`; `active_outward_corner_radius_px` → `values.outward_corner_radius`;
  `hover_background` → `tab.fill @ {hover}`; `hover_text_color` → `label.text.fill @ {hover}`; `font` →
  `label.text`; `border` → `tab.stroke`, `tab.corners`, `tab.padding`, `tab.shadow`.

- **Merged from other groups and rows:**
  - iOS's `UITabBar` and the navigation bars of KDE, Material and SwiftUI are the `bottom_tab_bar` widget (group 6 had proposed `tab` variant `navigation`).
  - Group 8: `tab.tab` transitions (X-MO-6; group 8 wrote `tab.body`).

### sidebar

*From group 4's catalogue.*

- **Kind:** typed widget, the side pane (navigation drawer, split view's sidebar column, side panel): GNOME
  (AdwNavigationSplitView/OverlaySplitView sidebars, `.sidebar-pane`), KDE (Kirigami Global/Context/Overlay
  drawers, qqc2 Drawer, Qt side panels, KIO places panel), macOS (source-list sidebar, `NSSplitViewItem`
  sidebar), iOS (`UISplitViewController` primary column), Material (navigation drawer), Windows
  (NavigationView pane, SplitView pane). Group 6's "navigation drawer/pane" is this widget. Its rows are
  `list` rows with variant `sidebar`.
- **Parts:**
  - `pane` — **element**: fill (paint: material on macOS, iOS, WinUI's overlay acrylic), stroke (the inner edge
    only, flipping with `end` and the direction, GN-1.139; the full frame under KDE's option, K1.140), corners
    (per corner: Material rounds the end corners only, MAT-SB7; floating glass, SD-6, MAC-SB06), shadow,
    padding, min and max width, bindings (the surface set it draws on: Kirigami's colour set by modality,
    Q1.53; GNOME's sidebar set), opacity and transitions (GN-1.136's backdrop transition).
  - `separator` — Kirigami's segmented inner-edge separator drawn apart beside the global toolbar (Q1.55):
    stroke, with `separator_inset` values.
  - `edge_shadow` — the gradient strip at a modal drawer's edge (Q1.56): fill (linear gradient), size
    limits, transitions. may_be_absent.
  - `scrim` — the dim over the covered content (Q1.57, MAT-SB6, SD-9†): fill, transitions.
  - `handle` — **element**, the floating drawer handle (Q1.61): fill, corners, shadow, icon, size limits,
    padding. may_be_absent.
  - `header` — the pane's header area (Q1.63, Q1.66's header toolbar, W1-095's pane-toggle, back and search
    area): text (title level), icon, padding, min height.
  - `toggle_button` — **element**, the pane toggle (hamburger) and back buttons (W1-095, GNOME's sidebar
    toggle in split views): size limits, glyph.
  - `content` — the content area beside the pane (W1-097): fill, stroke, corners (rounded where it meets the
    pane).
  - `capacity_bar` — the per-device usage bar in a places panel (K1.142†): track fill, fill, corners, min
    height. may_be_absent.
  - `banner` is the banner widget in `context sidebar.pane` (GN-1.146).
- **Widget values:** `width`, `min_width`, `max_width`: `Value<f32>`; `width_fraction: Value<f32>` (GN-1.147,
  SD-10†, MAC-SB09†, Q1.60, MAT-SB7, W1-094); `collapsed_width: Value<f32>` (Q1.59, W1-094);
  `display_mode_breakpoints: Vec<(Key, Value<f32>)>` (the widget's own breakpoints, R16: W-P4-19†'s
  compact and expanded thresholds; P4-16†); `columns_by_width` (iOS's arrangement per app width,
  P4-16†) as rules on `values.display_mode` by `size_class`; `display_mode: Value<Key>` (Windows Left,
  LeftCompact, LeftMinimal, Top; iOS tile, overlay, displace); `resize_grip_width: Value<f32>` (Q1.60); `floating_inset: Sides<Value<f32>>` (the floating sidebar's inset from
  the window edges, SD-6, MAC-SB06);
  `separator_inset: Sides<Value<f32>>` (Q1.55); `drag_hover_activate_delay_ms: Value<f32>` (GN-1.145);
  `snap_back` (Q1.58) as a presentation transition; presentation transitions keyed by event: `open`,
  `close`, `collapse`, `expand`, `morph_to_tab_bar` (SD-7), `submenu_push`, `submenu_pop` (Q1.64), `labels_fade`
  (Q1.59), `handle_slide` (Q1.61), `scrim_fade` (MAT-SB9, Q1.57).
- **Variants:** typed: `overlay` (modal or overlaying: Kirigami modal drawers, Material modal drawer,
  WinUI's overlay pane, iOS overlay behaviour, GNOME OverlaySplitView collapsed), `compact` (collapsed rail:
  Kirigami collapsed GlobalDrawer, WinUI LeftCompact), `floating` (macOS 26, iOS 26 inset glass sidebars),
  `end` (trailing side: GNOME `.end`, Kirigami right drawers, WinUI's right pane). Keyed: GNOME `@secondary`
  (middle pane, GN-1.137); KDE `@global`, `@context`, `@places` (KIO), `@side_panel` (Qt
  `_kde_side_panel_view`); Windows `@left_minimal`, `@top`; KDE `@edge_top`, `@edge_bottom` (drawers on the top or bottom edge, Q1.60); iOS `@displace`;
  Material `@bottom_sheet`
  (MAT-SB7's bottom-sheet drawer shape).
- **Sizes and densities:** macOS: the sidebar size is the condition `container_size(sidebar, small | medium |
  large)` (R2.3), selecting `list` rules (MAC-SB01); others none.
- **Widget-specific flags:** typed flags used: open (shown), focus_within (KDE's focused-view edge colour,
  K1.140), drop_target (drag-hover activation).
- **Exclusive groups:** none (the pane has no common-state group; rows follow `list`).
- **may_be_absent:** `separator`, `edge_shadow`, `scrim`, `handle`, `header`, `toggle_button`, `content`,
  `capacity_bar`; `pane.stroke` (floating, modal), `pane.shadow`.
- **State-model source:** GNOME: `_sidebars.scss` selectors and the split views' properties
  (`adw-navigation-split-view.c`, `adw-overlay-split-view.c`); KDE: Kirigami `OverlayDrawer.qml`,
  `GlobalDrawer.qml`, `ContextDrawer.qml` states (modal, collapsed, open, position) and Breeze's side-panel
  branch (`st:3958-3963`); macOS: `NSVisualEffectView.State` with the key state, `NSSplitViewItem` behaviour
  and collapse; iOS: `UISplitViewController.DisplayMode` and `SplitBehavior`, collapsed; Material: navigation
  drawer tokens (standard, modal); Windows: NavigationView's `DisplayModeGroup` and SplitView's pane states.
- **Today's fields:** `background_color` → `pane.fill`; `font` → `list.row.text @ variant sidebar`;
  `selection_background` → `list.row_background.fill @ {selected} variant sidebar`; `selection_text_color` →
  `list.row.text.fill @ {selected} variant sidebar`; `hover_background` → `list.row_background.fill @ {hover}
  variant sidebar`; `border` → `pane.stroke`, `pane.corners`, `pane.padding`, `pane.shadow`.

- **Merged from other groups and rows:**
  - Group 6: typed `overlay` (GN-2.19, W2-30) and `inspector` (the trailing inspector column, MAC-N19, C-41†); `scrim`, `edge_shadow`, `separator` (group 6 wrote `dimming`, `transition_shadow`, `divider`); values `open_width`, `compact_width`; presentation events `@open`, `@close`; GtkStackSidebar rows are list rows with the keyed `@stack_sidebar` and a `needs_attention` dot (GN-2.34); the file chooser's places rows are list rows in `context any file_chooser` with keyed `@new_bookmark`, `@drop_placeholder` (GN-P3-22†).
  - Group 7: Material's navigation rail is the typed `compact` (group 7 wrote `rail`, MAT-N13): its container is `pane`, its items list rows (`row_selection_indicator`, `leading_icon`, `row` text), its menu button `toggle_button`, its badge the `badge` widget in context; keyed `@expanded`, `@modal`.
  - Group 8: `handle` (Q3.31; group 8 wrote `drawer.handle`).
  - kirigami-addons: the settings window's sidebar (K-KA-41†).

### toolbar

*From group 4's catalogue.*

- **Kind:** typed widget: GNOME (`.toolbar`, AdwToolbarView top and bottom bars), KDE (QToolBar, qqc2 ToolBar,
  Kirigami ActionToolBar, page header and global toolbar), macOS (NSToolbar), iOS (UIToolbar, bar button
  items), Material (top app bar), Windows (CommandBar). Group 6's "navigation/app bar" overlaps for
  Material's top app bar and Kirigami's page header; this group traces them here.
- **Parts:**
  - `bar` — **element**: fill (paint, material: MAC-TL09, TB-1; transparent at the scroll edge by the state
    `scrolled`), stroke (one edge: KDE's bottom line, K1.151; iOS's top hairline, TB-2; GNOME's `.border`
    line, GN-1.154), shadow (raised bars, GN-1.153; on-scroll elevation, MAT-TA1), corners and padding
    (`.osd`, GN-1.149), min height, bindings (KDE's Header set by position, Q1.68, Q1.71), transitions.
  - `item` — **element**, a tool button or bar item (K1.152, W1-099, MAC-TL02, TB-6, P3-3†, MAC-TL11†):
    fill, stroke, corners, padding, opacity, focus indicator, transitions.
  - `item_icon` — icon style per state (MAT-TA2, MAC-TL02, K1.158).
  - `item_label` — text per state, under or beside the icon.
  - `item_menu_indicator` — the menu arrow or split part with its separator line (K1.155; W1-102's sub-item
    chevron): glyph, stroke.
  - `item_check` — the check glyph of a toggle item in the overflow menu (W1-100). may_be_absent.
  - `item_group` — **element**, items sharing one background (TB-5, MAC-TL03; 26's glass capsules): fill
    (material), corners, shadow, padding.
  - `separator` — the line between items (K1.156, Q1.70, W1-101, Q1.73's leading separator, MAC-TL08's tracking
    separator): stroke, corners.
  - `handle` — KDE's drag handle (K1.156). may_be_absent.
  - `overflow_button` — **element** (K1.157, Q1.75, MAC-TL06, W1-102, MAT-TA8†): glyph, fill, size limits.
  - `title` — the bar's title (Q1.72, MAT-TA3, MAT-TA7): text, padding.
  - `subtitle` — (MAT-TA4). may_be_absent.
  - `undershoot` — the scroll-edge shade under a bar (GN-1.156): fill, stroke. may_be_absent.
- **Widget values:** `item_gap: Value<f32>` (today); `icon_text_gap: Value<f32>` (K1.154, MAC-TL02);
  `label_position: Value<LabelPosition>` (icon only, text beside, text under, text only; K1.153 read per
  toolbar kind, W1-099, MAC-TL02, Q1.76); `fixed_space_width: Value<f32>`, `flexible_space_min_width:
  Value<f32>` (TB-8, MAC-TL10†); `group_gap: Value<f32>` (TB-5, MAC-TL03); `title_alignment:
  Value<Alignment>` (MAT-TA3); `collapse_on_scroll: Value<bool>` (MAT-TA3); `items_flat_by_default:
  Value<bool>` (Q1.76); `tooltip_shows_shortcut: Value<bool>` (Q1.76); `overflow_placement:
  PlacementStrategy` (Q1.75 below, or above for a footer; W1-102); `overflow_menu` refers to the `menu`
  widget in `context toolbar.overflow_button` (W1-102); presentation transitions keyed by event: `open`,
  `close` (W1-104), `items_change` (P3-4†), `height_change`, `separator_fade` (Q1.74), `search_expand`
  (MAC-TL07), `scroll_state` (MAT-TA1), `customize` (P4-10†, MAC-P3-6†).
- **Variants:** typed: `top`, `bottom` (header or footer position: KDE ToolBar, GNOME's top and bottom bars,
  iOS toolbars against navigation bars, Material top app bar; Q1.68), `prominent` on `item` (iOS 26, macOS 26:
  TB-4, MAC-TL04), `raised` (GNOME's raised bar; Material's on-scroll bar is a state). Keyed: GNOME `@osd`,
  `@border`, `@collapse_spacing`; KDE `@main` (main toolbar against other toolbars, K1.153, K1.158),
  `@global`, `@page_header`, `@split`, `@inline_small`, `@inline_large` (the menu indicator's styles on `item`,
  K1.155); macOS `@tracking` (on `separator`, MAC-TL08), `@unified`, `@unified_compact`, `@expanded`, `@preference` (MAC-TL01),
  `@search` (MAC-TL07); iOS `@done`, `@padding_removed` (on `item`, TB-4, TB-8); Material `@center_aligned`, `@medium`, `@large`,
  `@medium_flexible`, `@large_flexible` (MAT-TA3); Windows `@compact`, `@minimal`, `@hidden` (closed display
  modes, W1-103).
- **Sizes and densities:** iOS: the compact height under the vertical size class `compact` (TB-3, ADD-A20);
  others by variant.
- **Widget-specific flags:** typed flags used: hover, pressed, checked (W1-100), selected (P3-3†,
  MAC-TL11†), disabled, focus_visible, open (W1-103's open command bar), scrolled (TB-1, MAT-TA1).
- **Exclusive groups:** Windows: `[disabled, pressed, hover]` (AppBarButton CommonStates).
- **may_be_absent:** `item_menu_indicator`, `item_check`, `item_group`, `handle`, `title`, `subtitle`,
  `undershoot`; `bar.shadow`, `bar.stroke` on some edges.
- **State-model source:** GNOME: `_toolbars.scss` selectors; KDE: Breeze's tool-button and toolbar branches
  (`st:4577-4624`, `st:5162-5227`, `st:5526-5685`) and qqc2/Kirigami toolbar QML (position, flat, display);
  macOS: `NSToolbar.Style`, `NSToolbarItem` (bordered, style, enabled, selected), key window; iOS:
  `UIBarButtonItem.Style`, `isSelected`, `UIBarAppearance` standard/compact/scroll-edge; Material: top app
  bar tokens (rest, on-scroll); Windows: AppBarButton and AppBarToggleButton visual states, CommandBar's
  `DisplayModeStates` and open states.
- **Today's fields:** `font` → `item_label.text` (and `title.text` where a preset states one); `bar_height_px`
  → `bar.min_height`; `item_gap_px` → `values.item_gap`; `background_color` → `bar.fill`; `icon_size_px` →
  `item_icon.icon.size`; `border` → `bar.stroke`, `bar.corners`, `bar.padding`, `bar.shadow`.

- **Merged from other groups and rows:**
  - Group 6: typed `bottom` (GtkActionBar, Material's bottom app bar); keyed Material `@docked`, `@floating`, `@vibrant`; iOS's navigation bar: parts `title`, `large_title`, `subtitle`, `prompt`, `title_menu_chevron`, `document_header`, keyed `@large_title`, `@compact`, `@with_search`, its appearance by the `scrolled` flag, the item-style layouts `@navigator`, `@browser`, `@editor` (GN-2.20, MAT-N11, MAT-N25, C-1, C-28, C-43†).
  - Group 7: Material's floating toolbar (MAT-P4-7†): parts `leading_content`, `trailing_content`, keyed `@collapsed`, events `@expand`, `@collapse`; iOS's back button is `button` variant `back` in `context toolbar.bar` with the event `@back_label` (C-2; group 7 wrote `navigation_bar`).
  - Group 1: presentation events `@reveal` (AdwToolbarView's bars, GN-P3-5†) and `@hide_on_swipe` (iOS's bars, P4-3†).
  - Group 4: the badge on an item (TB-7, MAC-TL05), the search field (MAT-TA6, MAC-TL07) and the avatar (MAT-TA5) are those widgets in `context toolbar.bar`.
  - kirigami-addons: keyed KDE `@floating` (FloatingToolBar, K-KA-28†).

### status_bar

*From group 4's catalogue.*

- **Kind:** typed widget: GNOME (its bottom bar: GtkActionBar, AdwToolbarView's bottom bar, GN-1.157), KDE
  (QStatusBar), macOS (the window's bottom content border). WinUI has none; the Win32 status bar belongs to
  a Win32 preset (R1.3).
- **Parts:** `bar` — fill, stroke (the top line only: K1.160, GN-1.157, MAC-ST01), padding, min height,
  text; `item` — an item's text and padding.
- **Widget values:** `item_gap: Value<f32>` (GN-1.157).
- **Variants:** keyed: KDE `@main_window` (the top line is drawn only for a QMainWindow's status bar).
- **Sizes and densities:** none.
- **Widget-specific flags:** none; the window's inactive look is the condition `window_inactive`.
- **Exclusive groups:** none.
- **may_be_absent:** `bar.stroke` (KDE outside a main window), `bar.min_height` (Auto where content-sized).
- **State-model source:** GNOME `_toolbars.scss` (action bar, backdrop); KDE Breeze `st:5312-5325`; macOS
  `NSWindow.contentBorderThickness(for: .minY)`, key and main window.
- **Today's fields:** `font` → `bar.text`; `background_color` → `bar.fill`; `border` → `bar.stroke`,
  `bar.padding`.

### splitter

*From group 4's catalogue.*

- **Kind:** typed widget: GNOME (GtkPaned), KDE (QSplitter, dock resize handles, qqc2 SplitView, Kirigami
  ColumnView separators), macOS (NSSplitView), iOS (column divider), Material (drag handle), Windows
  (Community Toolkit sizers).
- **Parts:** `divider` — **element**: fill, stroke (GNOME's two lines on either side of the wide handle),
  min width or height (its thickness), bindings (Kirigami's Header set, Q1.93), cursor, transitions;
  `grip` — the grip, dimple or drag handle on the divider (MAC-SP02, MAT-SP1, W1-157†, SP-2): fill, glyph,
  corners, size limits, padding, shadow.
- **Widget values:** `hit_extension: Value<f32>` (Q1.92, Q1.93; a rule `@ {hover}`); `presentations`
  transitions for showing and hiding a column belong to `sidebar` (SP-3).
- **Variants:** typed: `wide` (GNOME `.wide`, macOS thick and pane splitter), `sidebar` (the split beside a
  floating sidebar: no divider on macOS 26 and iOS 26, MAC-SP04, SP-1). Keyed: macOS `@thin`, `@thick`,
  `@pane_splitter`; KDE `@dock`, `@column_view`.
- **Sizes and densities:** none.
- **Widget-specific flags:** typed flags used: hover, pressed, dragged (MAC-SP03), focus_visible (MAT-SP2),
  disabled (W1-157†).
- **Exclusive groups:** Windows: `[disabled, pressed, hover]` (SizerBase's states).
- **may_be_absent:** `grip`; `divider.fill`, `divider.stroke` (variant `sidebar`).
- **State-model source:** GNOME `_paned.scss`; KDE Breeze splitter branch and qqc2 `SplitView.qml`; macOS
  `NSSplitView.DividerStyle`; iOS column resizing states; Material drag-handle tokens; Windows SizerBase
  visual states.
- **Today's fields:** `divider_width_px` → `divider.min_width` (its thickness); `divider_color` →
  `divider.fill`; `hover_color` → `divider.fill @ {hover}`.

### separator

*From group 4's catalogue.*

- **Kind:** typed widget, every platform.
- **Parts:** `line` — the line's stroke (Conventions: `stroke.top` horizontal, `stroke.left` vertical;
  position centred where the platform centres it on a half pixel, K1.201), min size (GNOME's spacer).
- **Widget values:** `inset: Sides<Value<f32>>` (start and end insets per container and orientation: GN-1.187,
  W1-119, MAT-D1), by rules with `context`.
- **Variants:** typed: `horizontal`, `vertical`. Keyed: GNOME `@spacer` (no line, GN-1.188); KDE `@light`
  (Q1.95); iOS `@opaque` (SE-2; its stroke refers to the keyed roles `@separator` and `@opaque_separator`); Material `@inset`, `@inset_start`, `@inset_end` (MAT-D1).
- **Sizes and densities:** none.
- **Widget-specific flags:** none.
- **Exclusive groups:** none.
- **may_be_absent:** `line.stroke` (variant `@spacer`).
- **State-model source:** no states; variants from libadwaita `_misc.scss`, Kirigami `Separator.qml`, Material
  divider, UIKit separator colours, WinUI separators per container.
- **Today's fields:** `line_color` → `line.stroke` paint; `line_width_px` → `line.stroke` width.

- **Merged from other groups and rows:**
  - Group 8: `line.stroke` with `Edge.snap` (K-P3-20†, ADD-A33).

### card

*From group 4's catalogue.*

- **Kind:** typed widget: GNOME (`.card`), KDE (Kirigami Card/AbstractCard), iOS (cell background
  configurations), Material (filled, elevated, outlined cards), Windows (card surfaces, Community Toolkit
  SettingsCard). macOS's NSBox is the group box (group 6), where its rows are traced.
- **Parts:** `body` — **element**: fill, stroke (position outside on iOS, CD-1), corners, shadow (layers or an
  elevation entry; KDE's lift layer is a zero-blur shadow layer, Q1.128), padding, focus indicator, bindings
  (the `Card` surface set; Kirigami's View set, Q1.129), transitions; `icon` (MAT-CA3); `banner` — Kirigami's
  banner image area (Q1.131): padding, min size; `banner_band` — the band behind the banner title: fill,
  height relative to the title; `banner_title` — text and icon. The footer is a `toolbar` in `context
  card.body` with variant `bottom` (Q1.132).
- **Widget values:** `section_gap: Value<f32>` (between header, content and footer, Q1.130);
  `background_inset: Sides<Value<f32>>` (iOS's background insets, CD-1); `banner_band_height_factor:
  Value<f32>` (Q1.131).
- **Variants:** typed: `filled`, `elevated`, `outlined` (Material; Windows' default card is outlined-like,
  its secondary and tertiary fills keyed), `activatable` (GNOME `.activatable`, Kirigami clickable cards,
  Windows clickable SettingsCard, Material interactive cards). Keyed: GNOME `@osd`; Windows `@secondary`,
  `@tertiary`; KDE `@banner`.
- **Sizes and densities:** none.
- **Widget-specific flags:** typed flags used: hover, pressed, checked (Q1.126), focus_visible, dragged,
  disabled (MAT-CA2).
- **Exclusive groups:** Material: `[dragged, pressed, focus_visible, hover]`; Windows: `[disabled, pressed,
  hover]`.
- **may_be_absent:** `icon`, `banner`, `banner_band`, `banner_title`; `body.shadow` (OSD card, outlined).
- **State-model source:** GNOME `_misc.scss` card selectors; Kirigami `DefaultCardBackground.qml`; iOS
  `UIBackgroundConfiguration.updated(for:)`; Material card tokens (filled, elevated, outlined × states);
  Windows SettingsCard visual states.
- **Today's fields:** `background_color` → `body.fill`; `border` → `body.stroke`, `body.corners`,
  `body.padding`, `body.shadow` (from `shadow_enabled`).

### expander

*From group 4's catalogue.*

- **Kind:** typed widget: GNOME (AdwExpanderRow, GtkExpander), KDE (KCollapsibleGroupBox), macOS (disclosure
  and push-disclosure buttons, SwiftUI DisclosureGroup), iOS (outline disclosure), Windows (Expander).
  Material's expandable list item is a `list` row with a `disclosure` (MAT-L9, MAT-EX1).
- **Parts:** `header` — **element**: fill, stroke, corners (GN-1.261), padding, min height, text, focus
  indicator; `arrow` — **element**: glyph (source per platform; macOS through the system control rendering,
  S15), fill and corners (WinUI's chevron button), opacity, transform (rotation per state) with its
  transition; `content` — the disclosed area: fill (GN-1.260, W1-141), stroke, corners, padding.
- **Widget values:** `arrow_side: Value<Side>` (today), `arrow_gap: Value<f32>` (today), `content_indent:
  Value<f32>` (today), `drag_hover_expand_delay_ms: Value<f32>` (GN-1.262); presentation transitions keyed by event: `expand`,
  `collapse` (the content's reveal: GN-1.259, K1.251, W1-142, EX-2).
- **Variants:** keyed: GNOME `@row` (AdwExpanderRow against the plain GtkExpander); macOS `@push_disclosure`
  (MAC-EX01); iOS `@header`, `@cell` (EX-3); Windows `@expand_up` (the content opens above the header, with
  its own border thickness and corners, W1-141; a variant because the look changes with it, which a widget
  value cannot select).
- **Sizes and densities:** macOS: `mini`, `small`, `regular`, `large` (MAC-EX05†).
- **Widget-specific flags:** typed flags used: open (expanded), hover, pressed, focus_visible, disabled.
- **Exclusive groups:** Windows: `[disabled, pressed, hover]`.
- **may_be_absent:** `content.fill`, `content.stroke`, `arrow.fill` (outside Windows).
- **State-model source:** GNOME `_lists.scss` (expander row) and `_expanders.scss`; KDE Breeze arrow branch
  (`st:4424-4427`) and `kcollapsiblegroupbox.cpp`; macOS `NSButton` disclosure states and control sizes;
  iOS `UICellAccessory.OutlineDisclosureOptions`; Windows Expander visual states (CommonStates,
  ExpandStates, ExpandDirectionStates).
- **Today's fields:** `font` → `header.text`; `header_height_px` → `header.min_height`;
  `arrow_icon_size_px` → `arrow.glyph.size`; `hover_background` → `header.fill @ {hover}`; `arrow_color` →
  `arrow.glyph.fill`; `arrow_side`, `arrow_gap_px`, `content_indent_px` → the values of the same names;
  `frame_enabled` → whether `header.stroke` and `content.stroke` are stated or `Absent`; `border` →
  `header.stroke`, `header.corners`, `header.padding`.

- **Merged from other groups and rows:**
  - Group 2: AppKit's `@disclosure` bezel is this widget's `arrow` (the whole-control rendering), and `@push_disclosure` its keyed variant (MAC-B01, MAC-EX01).
  - Group 8: presentations of KCollapsibleGroupBox (K3.37); `arrow.transitions` (X-MO-6).

### layout

*From group 4's catalogue.*

- **Kind:** typed widget with values only (no parts drawn): a layout container's default spacing, queried
  per container element so that size-class rules apply (R16); today's top-level `layout` table. Every
  platform.
- **Parts:** none.
- **Widget values:** today's `widget_gap`, `container_margin`, `window_margin`, `section_gap`; and
  `label_gap` (label to control, per size: MAC-LY01, W1-146), `button_gap` (MAC-LY01, W1-146), `header_gap`
  (control to header, W1-146), `surface_text_margin` (W1-146), `item_gap` and `line_gap` (flow layouts:
  GN-P3-24†, RL-3†, MAC-RL01†, W-P4-9†), `flow_max_per_line` (GN-P3-24†), `max_content_width` and
  `tightening_threshold` (AdwClamp, GN-1.195; iOS readable width, LY-2), `form_label_alignment`,
  `form_baseline_alignment`, `form_field_growth`, `form_wraps` (K1.210, MAC-LY02†), `pane_gap` (MAT-LA1,
  MAT-SP3), `max_panes_horizontal`, `max_panes_vertical`, `pane_width`, `pane_height`, `avoids_hinge`
  (MAT-P3-13†), `column_width`, `column_min_width` (Kirigami ColumnView, Q1.93, Q1.94; CardsLayout, Q1.133),
  `max_columns` (Q1.133); presentation transition `column_scroll` (Q1.94). Lengths that scale with text are
  expressions (ADD-A55).
- **Variants:** typed: `form`, `flow`, `clamp`, `page`, `pane`. Keyed: GNOME `@preferences_page`; KDE
  `@cards`, `@cards_list`, `@column_view`.
- **Sizes and densities:** macOS control sizes on `label_gap` and `button_gap` (MAC-LY01); size classes:
  Material's window classes (MAT-LA1, MAT-P3-13†), iOS's compact and regular (LY-1).
- **Widget-specific flags:** none. **Exclusive groups:** none. **may_be_absent:** every value but today's
  four (a platform without a clamp, a flow default or panes states them `Absent`).
- **State-model source:** no states; size classes from Material's window size classes and UIKit's traits.
- **Today's fields:** the four keep their names as widget values.

- **Merged from other groups and rows:**
  - Group 2: GNOME's `.linked` boxes are the keyed `@linked` with a negative `item_gap` (GN-1.42).
  - kirigami-addons: `column_width` (the settings window's columns, K-KA-41†).

## Typed widgets the model lacks today (alphabetical)

### about_dialog

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.39, K-P3-8†, Q2.57†, MAC-N32†); its window is a `dialog` element.
- **Parts:** `icon`; `name`; `version` (GNOME's pill); `description`; `section_title`; `credits`;
  `copyright`; `person_name`; `person_task`; `link_button` *(element)*.
- **Widget values:** `icon_size` (`Value<f32>`); `icon_max_width_fraction` (`Value<f32>`, Kirigami);
  `margins` (`Sides<Value<f32>>`); `gap`; `subpage_margins`; `width` (macOS's panel).
- **Variants:** keyed KDE flavours: `widgets` (KAboutApplicationDialog, tabs), `kirigami` (AboutPage on form
  cards) as the flavour condition.
- **may_be_absent:** `version` pill shape (all but GNOME draw plain text); `link_button` (macOS).
- **State-model source:** GNOME `_misc.scss`, `adw-about-dialog.ui`; KDE `kaboutapplicationdialog.cpp`,
  `AboutItem.qml`; macOS `NSApplication.AboutPanelOptionKey` and the panel as drawn.
- **Today's fields:** none (MAC-N32†'s stand-in `dialog.*` stays `dialog`'s).

- **Merged from other groups and rows:**
  - kirigami-addons: keyed KDE `@ka_about` (K-KA-44†) with the value `release_gap`.

### autocomplete

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.56†, K2.29†, MAT-N27, C-38†, W-P3-17†; Windows' AutoSuggestBox list, W2-15).
  The field is an `input` element whose ancestor is the autocomplete (rules select `context any autocomplete`).
- **Parts:** `popup` (the list's container); `item` *(element)*; `item_label`; `item_icon`; `token` (a chosen
  suggestion inserted into rich text, W-P3-17†).
- **Widget values:** `placement` (`PlacementStrategy`, S13); `width` (enum: as the field, by content);
  `max_visible_items` (`u32`); `filter_delay_ms` (`Value<f32>`, GN-2.56†); `mode` (enum: popup, inline
  completion as selected text, both: K2.29†); `trigger_prefix` (`Option<String>`, W-P3-17†); `hover_selects`
  (`bool`, GN-2.56†).
- **Variants:** Material keyed `@filled`, `@outlined` (they select the field's look through context);
  iOS keyed `@menu` (inline bar placement), `@list` (stacked placement).
- **Sizes:** none.
- **Flags:** `opened_above` (set by the connector when placement flips, so attached corners can square off on
  the side touching the field: W2-15).
- **Exclusive groups:** Windows `CommonStates` per item [disabled, pressed, hover].
- **may_be_absent:** `token` (all but Windows); `item_icon` (GNOME, KDE).
- **State-model source:** GNOME `gtkentrycompletion.c` + GtkTreeView row selectors; KDE `kcompletionbox.cpp`
  (Breeze item-view branches); Material autocomplete token files; iOS `UISearchSuggestionItem`; Windows the
  ListViewItem visual states of the AutoSuggestBox and RichSuggestBox templates.
- **Today's fields:** none.

- **Merged from other groups and rows:**
  - kirigami-addons: the form file row's path completion (K-KA-13†).

### avatar

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.14, Q2.28, Q2.29, W2-24).
- **Parts:** `body`; `initials`; `fallback_icon`; `image` (clipped to the body's shape); `badge` slot (a
  `badge` element in context).
- **Widget values:** `default_size` (`Value<f32>`); `palette` (`Vec<Paint>`: libadwaita's gradient pairs,
  Kirigami's colours; their choice per element: ADD-B14); `initials_size` (follows the
  avatar's size: ADD-B11); `fallback_icon_inset`.
- **Variants:** keyed GNOME `@contrasted`; Windows group form where its template states it.
- **may_be_absent:** `body.stroke` (GNOME, Windows where none); `fallback_icon` (GNOME).
- **State-model source:** GNOME `_avatar.scss`; KDE `Avatar.qml`; Windows PersonPicture template.
- **Today's fields:** none.

- **Merged from other groups and rows:**
  - kirigami-addons: keyed KDE `@ka_avatar`, `@button` (K-KA-33†).

### background_extension

*From group 7's catalogue.*

- **Kind:** typed widget: iOS C-19 (`UIBackgroundExtensionView`) and macOS MAC-X06 (`NSBackgroundExtensionView`),
  both 26 only.
- **Parts:** `extension` (the mirrored and blurred content: a `Material` paint over a transformed copy of the
  neighbouring content; the mirror is a presentation of content, so the part carries the blur and tint, and the
  mirroring is a boolean widget value).
- **Widget values:** `mirrors_content: bool`.
- **Variants/sizes/flags:** none. **may_be_absent:** all but iOS 26 and macOS 26 presets.
- **State-model source:** UIKit/AppKit view classes; appearances.
- **Today's fields:** none.

- **Merged from other groups and rows:**
  - Group 8: MAC-X06 (group 8 had placed it on `window.content`).

### badge

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.36, GN-2.37, Q2.25–Q2.27, MAC-N11, MAT-N10, W2-08, C-15).
- **Parts:** `body` (the dot or the pill: fill, stroke, corners, padding, minimum size); `label` (the count or
  string); `icon` (Windows' icon form, Kirigami's icon).
- **Widget values:** `anchor_offset_x`, `anchor_offset_y` (`Value<f32>`, the placement on its anchor icon or
  item: MAT-N10, GN-2.37, C-15); `anchor_corner` (enum: top-end, top-start, trailing-inline); `cutout`
  (`Option<Value<f32>>`: the gap libadwaita's indicator bin masks out of the anchor around the badge, GN-2.36);
  `max_count` (`Option<u32>`, where a count is capped).
- **Variants:** typed form `dot`, `count`, `icon`; typed severity `info`, `success`, `warning`, `error`,
  `attention` (Windows' Attention and libadwaita's needs-attention dot, both accent; Windows' Caution and
  Critical are the typed `warning` and `error`); keyed: macOS `@circular`,
  `@inline` (deprecated count badge in source lists); iOS `@tab_bar`, `@bar_button`, `@list_row`.
- **Sizes:** GNOME regular and small (the tab button counter); Material small (dot) and large (label) as the
  `dot`/`count` forms, no sizes.
- **Flags:** `multiline` (Kirigami's pill radius only while the label is on one line, Q2.27).
- **Exclusive groups:** none.
- **may_be_absent:** `icon` (GNOME, macOS, Material, iOS); `body.stroke` (all but KDE); `values.cutout`
  (all but GNOME).
- **State-model source:** GNOME `_view-switcher.scss` and `_buttons.scss` (`.needs-attention`, tab button);
  KDE `kirigami/src/controls/Badge.qml` types; macOS `NSButton.BezelStyle.badge` and SwiftUI `.badge`;
  Material `_md-comp-badge.scss`; Windows `InfoBadge` styles (no visual state groups: one style per form and
  severity); iOS `UITabBarItem` badge attributes, `UIBarButtonItem.Badge`.
- **Today's fields:** none. Today's stand-in `defaults.danger_color` (C-15) becomes the reference
  `= role.status.error.fill` in `badge.body.fill`.

- **Merged from other groups and rows:**
  - Group 4: on a tab-bar item (T-7), a toolbar item (TB-7, MAC-TL05) and a dragged list row's count (W1-110), in context.

### banner

*From group 6's catalogue.*

- **Kind:** typed widget, the in-window inline message (GN-2.11, GN-2.31, K2.13, Q2.16–Q2.21, MAT-N23, W2-07).
- **Parts:** `body`; `icon`; `icon_backplate` (Windows); `title`; `message`; `close_button` *(element)*;
  `action_area`; `divider` (Material); `edge_line` (KDE header and footer positions, GtkInfoBar's shade line);
  `image` (Material rich banner).
- **Widget values:** `presentations` (`@reveal`, `@conceal`: GNOME's slide-down, KDE's height and opacity);
  `actions_wrap` (enum: below the text when they do not fit, KDE; horizontal against vertical layout, Windows);
  `content_gap`, `action_gap`; `heights` (`Vec<(Key, Value<f32>)>` per line count and form factor, Material);
  `inner_corner_factor` (KDE's inner radius as a fraction of the outer: an expression on the body's corners).
- **Variants:** typed severity `info`, `success`, `warning`, `error`; keyed GNOME `@question` (GtkInfoBar),
  `@sidebar` (AdwBanner in a sidebar); KDE `@inline`, `@header`, `@footer`; Material `@basic`, `@rich`,
  `@vibrant`.
- **Conditions:** window inactive (GNOME's backdrop fill and dimmed content).
- **Flags:** `multiline` (KDE's top-aligned icon; Material's two-line heights).
- **Exclusive groups:** Windows `CommonStates` on its buttons.
- **may_be_absent:** `icon_backplate` (all but Windows); `divider`, `image` (all but Material); `edge_line`
  (Windows, Material); `icon` (GNOME's AdwBanner).
- **State-model source:** GNOME `_toolbars.scss` (`banner`), `_deprecated.scss` (`infobar`); KDE
  `kmessagewidget.cpp`, `InlineMessage.qml`; Material banner tokens; Windows InfoBar severity states.
- **Today's fields:** none. Stand-ins `defaults.{info,success,warning,danger}_color` (GN-2.31, Q2.16, W2-07)
  become references to `role.status.*` in this widget's rules.

- **Merged from other groups and rows:**
  - Group 4: GN-1.146's sidebar banner in `context sidebar.pane`. Group 8: presentations (K3.37).
  - kirigami-addons: keyed KDE `@ka_inline_message` (FormInlineMessage, K-KA-10†) and `@ka_banner` (K-KA-29†).

### bottom_tab_bar

*Written in the merge.*

- **Kind:** typed widget: the application-level tab bar that switches the app's top-level views, apart from `tab`
  (tabs attached to a page or a document strip). iOS (`UITabBar`, iPadOS's top tab bar), KDE (Kirigami's
  `NavigationTabBar`), Material (the navigation bar), macOS (SwiftUI's `TabView` with `.sidebarAdaptable`). Group 4
  wrote its parts from iOS's rows (T-1 … T-17†, P3-2†); group 6 had proposed it as `tab` variant `navigation`
  (Q2.33–Q2.36, MAT-N12, MAC-N23, C-22); the merge keeps one widget, because its parts, layout and state model are not
  the tab's.
- **Parts:**
  - `bar` — **element**: fill (paint with material; transparent at the scroll edge by `scrolled`), stroke (the top
    hairline, one edge), corners, shadow and inset of the floating capsule (iOS 26), min height (by size class: T-14,
    ADD-A20), padding.
  - `item` — **element**, one destination: fill, stroke, corners, padding, focus indicator, transitions; per-state
    colours (normal, selected, disabled, focused).
  - `item_icon` — icon style per state (the selected symbol variant: `icon.variant`, ADD-A37).
  - `item_label` — text per state (weight under `{selected}`, elision).
  - `indicator` — the selected item's indicator (Material's active indicator, iOS 26's glass lens; group 4 named it
    `selection_pill`).
  - `drop_indicator` — where a dragged tab would land (T-17†).
  - The badge on an item is the `badge` widget in `context bottom_tab_bar.item` (T-7); the More list's rows are `list`
    rows in `context bottom_tab_bar` (P3-2†); iOS 26's bottom accessory is the keyed `component.bottom_accessory` in
    `context bottom_tab_bar.bar` (T-12).
- **Widget values:** `floating_inset: Sides<Value<f32>>` (T-9); `item_gap`, `item_width: Value<f32>`,
  `item_positioning: Key` (T-3); `max_items: u32` (P3-2†, by size class); `max_content_width: Value<f32>`,
  `equal_width_items: Value<bool>` (Q2.33); `presentations` keyed by event: `selection_move` (Morph, T-8),
  `selection_change`, `hide`, `show` (T-15), `minimize` (T-10), `customize` (T-17†, P3-2†), `to_sidebar` (iPad's tab bar
  becoming a sidebar, C-22); `focus_visible_when` (ADD-A16).
- **Variants:** typed `floating` (iOS 26's capsule, T-9); keyed iOS `@stacked`, `@inline`, `@compact_inline` (item
  layouts, T-3), `@search` (T-11), `@prominent` (T-16), `@more` (P3-2†), `@tab_sidebar` (iPadOS's top bar, T-13); macOS
  `@sidebar_adaptable` (MAC-N23).
- **Sizes and densities:** none; the height follows the size class (iOS: `v_size_class compact`, T-14).
- **Widget-specific flags:** keyed iOS `@minimized` (T-10). Typed flags used: hover, pressed, selected, checked
  (Kirigami's checked tab button), disabled, focus_visible, dragged, drop_target (on `bar`).
- **Exclusive groups:** none stated by the rows (Material's state layers follow its tokens).
- **may_be_absent:** `indicator` (iOS 13–18), `drop_indicator`, `bar.stroke` (floating), `bar.shadow`; values
  `floating_inset`, `max_items`, `max_content_width`, `equal_width_items`.
- **State-model source:** iOS `UITabBarItemAppearance` (normal, selected, disabled, focused) per layout appearance
  (stacked, inline, compact inline), `UITabBarController.Mode`, `tabBarMinimizeBehavior`; KDE Kirigami
  `NavigationTabBar.qml` and `NavigationTabButton.qml` (hover, pressed, checked, focus); Material the navigation-bar
  tokens (`_md-comp-navigation-bar.scss`, `_md-comp-nav-bar*.scss`: active, inactive × hover, focus, pressed); macOS SwiftUI `TabView`
  (sidebar-adaptable and toolbar tab bar, MAC-N23, measured).
- **Today's fields:** none (today's `tab.*` stand-ins stay `tab`'s).

### breadcrumb

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.52†, K2.27†, Q2.44, MAC-N07, W2-10).
- **Parts:** `bar`; `crumb` *(element)*; `crumb_label`; `crumb_icon`; `separator` *(element: KDE's arrows are
  hovered and open a menu)*; `ellipsis` *(element: Windows' overflow item)*; `ellipsis_popup`; `placeholder`
  (macOS); `places_selector` *(element, KDE)*; `bottom_separator` (Kirigami's Header-set line).
- **Widget values:** `separator_padding`; `presentations` (`@scroll`: Kirigami's scroll to the current
  crumb); `editable` (`bool`: KDE's editable-path mode, whose field is an `input` in context).
- **Variants:** keyed macOS `@standard`, `@pop_up`; GNOME's path bar is linked buttons: the `button` rules for
  `linked` apply in context (GN-1.42).
- **Flags:** `current` (the last crumb), `navigator_active` (KDE: the navigator of the focused view).
- **Exclusive groups:** Windows `CommonStates` per item.
- **may_be_absent:** `ellipsis*` (all but Windows); `places_selector` (all but KDE); `placeholder` (all but
  macOS); `crumb_icon` (Windows).
- **State-model source:** GNOME `gtkpathbar.c` + button selectors; KDE `kurlnavigatorbutton*.cpp` and
  `BreadcrumbControl.qml`; macOS `NSPathControl.Style`; Windows BreadcrumbBarItem visual states.
- **Today's fields:** none (W2-10's stand-in `link.*` stays `link`'s).

- **Merged from other groups and rows:**
  - Group 8: `values.submenu_open_delay_ms` (K-P4-11†).

### calendar

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.25, C-6, K2.4, K2.23† grid, W2-02, MAT-N16 grid, MAC-N04 graphical).
- **Parts:** `frame`; `header`; `nav_button` *(element)*; `title` *(element: Windows' month-year header
  button, iOS's month-year toggle)*; `weekday_label`; `week_number`; `day` *(element)*; `day_label`;
  `today_marker` (GNOME's underline, Material's outline); `decoration` (iOS dot, image, custom);
  `range_band`; `year` *(element: Material year cells, Windows year and decade items)*; `month` *(element:
  Windows month items)*; `clock_face`, `clock_hand` (macOS clockAndCalendar); `time_row` (iOS).
- **Widget values:** `cell_gap` (`Value<f32>`); `grid_padding` (`Sides<Value<f32>>`); `presentations`
  (`@view_switch`: Windows' zoom between day, month and year views); `first_weekday_from_locale` (`bool`).
- **Variants:** keyed iOS `@wheels` (month-year wheel), Windows `@month_view`, `@year_view`, `@decade_view`.
- **Sizes:** macOS control sizes where AppKit draws the graphical style in them.
- **Flags:** `today`, `other_month` (Windows out-of-scope), `out_of_range`, `blackout`, `marked`
  (GNOME's checked day), `non_working_day` (KDE), `observance_day` (KDE), `range_start`, `range_end`,
  `in_range`.
- **Exclusive groups:** Windows `CommonStates` per item [disabled, pressed, hover], and its selection states
  combined as its template lists them (selected, selected-hover, selected-pressed, selected-disabled).
- **may_be_absent:** `week_number` (all but KDE); `decoration` (all but iOS); `range_band` (all but
  Material); `clock_face`, `clock_hand` (all but macOS); `time_row` (all but iOS); `today_marker` (where a
  fill or weight marks today).
- **State-model source:** GNOME `_calendar.scss` selectors; KDE `KDateTable` and Breeze
  `PE_PanelItemViewItem`; Windows `CalendarView` item states; Material date-picker tokens; iOS
  `UICalendarView` and `UIControl.State`; macOS `NSDatePicker` graphical style.
- **Today's fields:** none.

- **Merged from other groups and rows:**
  - kirigami-addons: keyed KDE `@ka_date_popup` (K-KA-20†, K-KA-21†), the keyed flag `@other_decade`, the value `scroll_physics: ScrollPhysics`.

### carousel

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.38, MAT-N18, W2-31, C-48†). Its indicator is a `page_indicator` in context.
- **Parts:** `item` *(element: Material's item container with state layers)*; `nav_button` *(element:
  Windows' next and previous)*; `nav_glyph`.
- **Widget values:** `item_gap`; `scroll_timeout_ms` (GNOME); `page_transition` (`Transition` with a spring
  timing, GNOME; Material's item settle); `nav_buttons_visibility` (enum: always, on hover); `presentations`
  (`@page_curl`, iOS's keyed curl transition); `inter_page_gap`.
- **Variants:** keyed Material `@multi_browse`, `@hero`, `@uncontained` where its tokens state them; iOS
  `@page_curl`, `@scroll`.
- **Flags:** none.
- **may_be_absent:** `nav_button`, `nav_glyph` (all but Windows); `item` paint (GNOME, iOS).
- **State-model source:** GNOME `adw-carousel.c`; Material carousel-item tokens; Windows FlipView button
  states; iOS `UIPageViewController.TransitionStyle`.
- **Today's fields:** none. Material's masked shape change while scrolling is driven by the scroll position:
  ADD-B13.

### chart

*From group 6's catalogue.*

- **Kind:** typed widget (C-45†, MAC-N37†, K-P3-10†).
- **Parts:** `plot_area`; `grid_line_major`; `grid_line_minor`; `axis_tick`; `axis_label`; `legend`;
  `legend_swatch`; `legend_label`; `series` (its colour per series index: ADD-B15); `selection_mark`.
- **Widget values:** `series_colours` (`Vec<Value<Color>>`, Swift Charts' default order; KDE's derivation
  from the highlight colour, ADD-A53, ADD-A57); `plot_insets`; `labels_per_axis` (`u32`); `legend_gap`;
  `default_size` (KQuickCharts' pie control).
- **Variants:** keyed KDE `@line`, `@pie`.
- **may_be_absent:** `grid_line_minor` (Swift Charts where none); `selection_mark` (KDE).
- **State-model source:** Swift Charts `AxisMarks` defaults; KQuickCharts controls.
- **Today's fields:** none.

### chip

*From group 6's catalogue.*

- **Kind:** typed widget (Q2.22–Q2.24, MAT-N9, W2-43†).
- **Parts:** `body`; `label`; `leading_icon`; `trailing_icon`; `avatar`; `remove_button` *(element)*.
- **Widget values:** `icon_text_gap`; `padding_without_icon` (KDE).
- **Variants:** typed `elevated` against flat (Material; Windows and KDE draw flat); keyed Material
  `@assist`, `@filter`, `@input`, `@suggestion`; KDE `@interactive`, `@closable`.
- **States:** KDE and the filter chip `checked`; Material's input chip and Windows' token `selected`;
  `dragged` (Material).
- **Exclusive groups:** Windows `CommonStates` with its selected combinations as listed by the template.
- **may_be_absent:** `avatar` (all but Material input); `trailing_icon` (KDE); `leading_icon` (Windows).
- **State-model source:** KDE `DefaultChipBackground.qml`; Material chip tokens; Windows
  `TokenizingTextBoxItem.Token.xaml` states.
- **Today's fields:** none.

- **Merged from other groups and rows:**
  - Group 7: the tokens of macOS's `NSTokenField` (MAC-N03) are chips in `context component.token_field`: keyed `@default`, `@rounded`, `@squared`, `@plain_squared`, part `menu_arrow`.

### code_editor

*From group 6's catalogue.*

- **Kind:** typed widget (GtkSourceView and KTextEditor, both in by R1.2; GN-SF01†–GN-SF13†, GN-P3-27†,
  K-SF01†–K-SF11†, K-P3-11†–K-P3-14†).
- **Parts:** `text` (the editor's fill and normal text); `selection` (with a rule for `none_of: [focused]`,
  GtkSourceView's unfocused selection); `caret` *(element)*; `bidi_secondary_caret`; `extra_caret` (KDE's
  multiple cursors); `current_line`; `gutter`; `line_number`; `current_line_number`; `gutter_separator`;
  `fold_marker`; `mark` *(element, keyed variants)*; `modified_line_marker`; `saved_line_marker`;
  `whitespace_mark` (keyed variants `@space`, `@tab`, `@newline`, `@nbsp`); `indentation_guide`;
  `bracket_match`; `bracket_mismatch`; `search_match`; `replace_match`; `right_margin`;
  `right_margin_area`; `dynamic_wrap_indicator`; `background_pattern`; `syntax_span`; `misspelled_word`;
  `template_field`; `template_placeholder` *(element)*; `inline_annotation`; `annotation_border`; `minimap`;
  `minimap_slider`; `minimap_fade`; `scroll_mark`; `completion_popup`; `completion_row` *(element)*;
  `completion_icon`; `completion_detail`; `completion_details_pane`; `informative_popup`.
- **Widget values:** `gutter_padding` (`Sides<Value<f32>>`); `right_margin_column` (`u32`); `minimap_width`;
  `completion_placement` (`PlacementStrategy`); `completion_offset_y` (an expression on `line_height`);
  `bracket_match_animation` (`Option<Transition>`); `whitespace_mark_size`.
- **Variants:** `syntax_span` takes one variant per syntax role: typed where both platforms name the role
  (`comment`, `string`, `function`, `constant`, `preprocessor`, `special_char`, `error`, `warning`,
  `type` (GtkSourceView `type`, KSyntaxHighlighting `DataType`), `decimal` (`DecVal`), `base_n`, `float`
  (`floating-point`)), keyed for the rest (GtkSourceView's `def:` styles and Markdown roles,
  KSyntaxHighlighting's `Keyword`, `ControlFlow`, `Operator`, …) and for per-language overrides. `mark`:
  keyed `@bookmark`, `@breakpoint_active`, `@breakpoint_reached`, `@breakpoint_disabled`, `@execution`,
  `@warning`, `@error`. `caret`: keyed KDE `@vi_block`, `@vi_underline`, `@vi_half_block`.
  `inline_annotation`: typed `warning`, `error`, keyed `@accent`.
- **Flags:** `selected` on `syntax_span` (KDE's selected-text colours); `focused` and `read_only` on
  `template_placeholder`; per-line keyed flags for the marks a line carries (KDE mixes the current line with
  them).
- **Bindings:** `syntax_span` binds its text colour as the foreground for the caret over it (K-P3-11†).
- **may_be_absent:** `extra_caret`, `fold_marker`, `mark`, `modified_line_marker`, `saved_line_marker`,
  `indentation_guide`, `replace_match`, `dynamic_wrap_indicator`, `template_*`, `annotation_border`,
  `minimap_fade`, `scroll_mark` (GNOME); `bidi_secondary_caret`, `background_pattern`, `inline_annotation`,
  `informative_popup` (KDE); `bracket_mismatch`, `selection` (GNOME's Adwaita schemes state neither: GTK's
  selection stands, a reference to the `text_area` selection).
- **Readers:** KDE: which KSyntaxHighlighting theme draws (K-SF01†).
- **State-model source:** GtkSourceView's style-scheme reference (`style-reference.md`) and the Adwaita
  schemes; KTextEditor's `kateconfig.cpp` colour list and the Breeze editor themes.
- **Today's fields:** none. Stand-ins `input.caret_color`, `input.background_color`,
  `defaults.text_selection_background` (GN-SF02†, GN-SF03†, K-SF02†, K-P3-11†) are the scheme's values here,
  not those fields.

### colour_button

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.27, K2.20, MAC-N05, W2-42†, C-8).
- **Parts:** `body` *(element)*; `swatch`; `swatch_ring` (GNOME's light-swatch ring, macOS's active ring,
  iOS's multicolour ring); `checkerboard` (under a translucent colour); `chevron` (macOS minimal, Windows);
  `expand_button` *(element, macOS expanded)*.
- **Widget values:** `swatch_inset` (`Sides<Value<f32>>`); `swatch_min_width`, `swatch_min_height`.
- **Variants:** keyed macOS `@default`, `@minimal`, `@expanded`.
- **Sizes:** macOS control sizes.
- **Flags:** `light_colour` (GNOME's `.light`, set when the colour is light), `active` (macOS's well active).
- **Exclusive groups:** Windows `CommonStates`.
- **may_be_absent:** `chevron`, `expand_button`, `checkerboard`, `swatch_ring` where not drawn.
- **State-model source:** GNOME `_buttons.scss` (`button.color`); KDE Breeze push-button branches; macOS
  `NSColorWell.Style`; Windows ColorPickerButton states; iOS `UIColorWell` + `UIControl.State`.
- **Today's fields:** none.

- **Merged from other groups and rows:**
  - kirigami-addons: value `swatch_gap` (K-KA-14†).

### colour_picker

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.53†, GN-2.27 chooser swatches, K2.25†, W2-05).
- **Parts:** `plane` (the saturation/value field; Windows' box or ring spectrum); `crosshair` *(element:
  focused against unfocused)*; `value_strip` (KDE); `checkerboard`; `preview` (Windows previous and current;
  GNOME's sample); `swatch` *(element: chooser swatches, colour wells)*; `swatch_ring`.
- **Widget values:** `grid_gap`; `plane_min_size`; `swatch_corners` (`Corners`, GNOME's per-edge radii by
  position use `position` rules instead).
- **Variants:** keyed Windows `@box`, `@ring`; KDE `@qt_dialog`. Hue, alpha and value sliders are `slider`
  elements in context with keyed variants `@hue`, `@alpha`, `@value`; the eyedropper a `button`; hex and
  channel fields `input`s.
- **Flags:** none.
- **Cursor:** `plane.cursor` crosshair (ADD-A59).
- **may_be_absent:** `value_strip` (GNOME, Windows); `swatch` (Windows).
- **State-model source:** GNOME `gtkcolorplane.c`, `gtkcolorscale.c`, `_color-chooser.scss`; KDE
  `qcolordialog.cpp` frames; Windows ColorPicker and ColorSpectrum states.
- **Today's fields:** none.

### date_picker

*From group 6's catalogue.*

- **Kind:** typed widget (MAC-N04, W2-03, W2-04 date half, K2.23†, MAT-N16, C-5). The month grid it opens is a
  `calendar` element (its ancestor is the date picker).
- **Parts:** `field` *(element: the button, pill or combo showing the date)*; `field_label`; `placeholder`;
  `glyph` (Windows' calendar glyph); `segment` *(element: a day, month or year part of a text-field date,
  selected while edited)*; `segment_separator` (Windows' spacer lines); `popup` (the flyout, popover, menu or
  docked container); `header`; `headline`; `supporting_text`; `divider`; `quick_item` *(element: KDE's word
  items Today, Next Week …)*.
- **Widget values:** `placement` (`PlacementStrategy`); `popup_gap` (`Value<f32>`); `presentations`
  (`Presentations`: popup appear and disappear).
- **Variants:** typed `text_field` (macOS textField, KDE QDateEdit without popup, Windows DatePicker field),
  `with_popup` (Windows CalendarDatePicker, KDE KDateComboBox, iOS compact, Material docked), `inline`
  (macOS clockAndCalendar, iOS inline); keyed macOS `@text_field_and_stepper`, Material `@modal`, `@input`.
- **Sizes:** macOS `NSControl.ControlSize` as AppKit draws it per style.
- **Flags:** `has_value` (placeholder against selected text colours, W2-03).
- **Exclusive groups:** Windows `CommonStates` [disabled, pressed, hover].
- **may_be_absent:** `glyph` (all but Windows); `segment`, `segment_separator` (iOS, Material);
  `quick_item` (all but KDE); `header`, `headline`, `supporting_text`, `divider` (all but Material).
- **State-model source:** macOS `NSDatePicker.Style` × control states; Windows CalendarDatePicker and
  DatePicker visual state groups; KDE Breeze `CC_ComboBox` and menu branches (`kdatecombobox.cpp`); Material
  date-picker token files; iOS `UIDatePickerStyle`.
- **Today's fields:** none (W2-03 and C-5 point at `combo_box.*`, which keeps its own).

- **Merged from other groups and rows:**
  - kirigami-addons: keyed KDE `@ka_date_popup` (K-KA-20†, K-KA-22†).

### dial

*From group 6's catalogue.*

- **Kind:** typed widget (K2.5, Q2.2, MAC-N15).
- **Parts:** `groove`; `value_arc`; `handle` *(element)*; `knob_dot` (macOS); `tick`.
- **Widget values:** `thickness`; `start_angle_deg`, `end_angle_deg`; `default_size` (`Value<f32>`, Q2.2).
- **Variants:** none typed; keyed macOS `@circular_slider` (the slider type that draws it).
- **Sizes:** macOS control sizes.
- **Flags:** none.
- **may_be_absent:** `value_arc` (macOS); `knob_dot` (KDE); `tick` (KDE).
- **State-model source:** KDE Breeze `CC_Dial` branches (`st:8144-8217`) and the QML Dial; macOS
  `NSSlider` circular type × control states.
- **Today's fields:** none.

### drag_image

*From group 7's catalogue.*

- **Kind:** typed widget (macOS MAC-X21, iOS C-24, Windows W3-49†, KDE K-RL06†, GNOME GN-2.49; Material's drag
  shadow is the application's image). Owned by the cross-cutting group (g8: MAC-X21, W3-49†) and the list
  group (K-RL06†); this group adds what GN-2.49 and C-24 need.
- **Parts:** `image` (the dragged image's window: foreground colour, translucency), `lift_preview` (scale,
  shadow, corners: iOS), `count_badge` (the drag-count badge: element with text), `caption` and `glyph`
  (Windows' caption and drop glyph; W3-49†).
- **Widget values:** `lift_delay_ms: Value<f32>`; presentation transitions for events `lift`, `drop`, `cancel`.
- **Variants/sizes/flags:** `Dragged`, `DropTarget` (typed). **may_be_absent:** `lift_preview`, `count_badge`,
  `caption`, `glyph` as the platforms differ.
- **State-model source:** UIKit `UIDragInteraction`/preview parameters (iOS); AppKit `NSDraggingSession` (macOS);
  libadwaita's `dnd` selector (GNOME).
- **Today's fields:** none.

- **Merged from other groups and rows:**
  - Group 8's `drag_preview` is this widget: its `body` is `image`; parts `caption`, `glyph`, `count_badge`, `drop_badge` (the drop-operation feedback, P3-10†), `drop_description` (Windows' drop description, W3-49†); presentation event `Move` (the drop outcome on Material's drag shadow, MAT-P4-8†).
  - Group 4: a list's drag image (K-RL06†, W1-110): `image.opacity` and `values.hotspot` in `context list`; the source row is `{dragged}`.

### edit_menu

*From group 6's catalogue.*

- **Kind:** typed widget (C-14, Q2.53†, MAT-N30†).
- **Parts:** `body`; `item` *(element)*; `item_label`; `item_icon`; `divider`; `paging_chevron`
  *(element)*; `arrow` (towards the selection).
- **Widget values:** `placement` (`PlacementStrategy`: above the selection start, below its end without
  room); `gap_to_selection`.
- **Variants:** typed `horizontal`, `vertical` (iOS's pointer style; Android's overflow list); keyed iOS
  `@compact`.
- **may_be_absent:** `item_label` (KDE icon-only); `arrow` (KDE, Material); `paging_chevron` (KDE).
- **State-model source:** iOS `UIEditMenuInteraction`; KDE `MobileTextActionsToolBarImpl.qml` (ToolButton
  states); Android floating `ActionMode` (measured).
- **Today's fields:** none.

- **Merged from other groups and rows:**
  - Group 7: WinUI's `CommandBarFlyout` (W2-23): parts `primary_commands`, `secondary_menu` (a `menu` element), `expand_button`; the joined-corner rule; events `Appear`, `expand`, `collapse`; shadow.
  - Group 1: GNOME's touch-selection bubble (GN-1.184, GN-1.286†).

### file_chooser

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.48, GN-P3-22†, GN-P3-23†, K2.26†). Its places list is a `sidebar`, its path bar
  a `breadcrumb`, its file view a `list`, its buttons `button`s, all in its context.
- **Parts:** `frame` (GNOME's line below the chooser); `preview_pane` (KDE).
- **Widget values:** `gap`; `icon_size_slider` (`bool`, KDE: a `slider` in context).
- **Rules in context:** the view-toggle and new-folder buttons flat (GNOME, `button` in `context any
  file_chooser`); the places rows, eject button, new-bookmark row, drop placeholder (`sidebar` in context,
  keyed variants); list and grid cells (`list` in context).
- **may_be_absent:** `preview_pane` (GNOME); `frame` line (KDE).
- **State-model source:** GNOME `_file-chooser.scss`, `gtkplacessidebar.c`; KDE `kfilewidget.cpp` and Breeze.
- **Today's fields:** none.

### find_bar

*From group 6's catalogue.*

- **Kind:** typed widget (C-25, K2.30†, MAC-N18). Its pattern field is an `input` in context.
- **Parts:** `bar`; `result_count`; `nav_button` *(element)*; `options_button` *(element)*; `done_button`
  *(element)*; `status_label`.
- **Widget values:** `placement` (enum: above the keyboard, top or bottom of the scroll view).
- **Flags:** `match`, `no_match`, `neutral` (KDE's field backgrounds and status text by the search result).
- **may_be_absent:** `result_count` (KDE); `status_label` (iOS, macOS); `done_button` (KDE).
- **State-model source:** iOS `UIFindInteraction`; KDE `katesearchbar.cpp`; macOS `NSTextFinder`.
- **Today's fields:** none. The highlights in the text are `code_editor.search_match` and the global
  `find_highlight` role.

### font_chooser

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.28, GN-P3-20†, K2.28†).
- **Parts:** `requester` *(element: GNOME's font dialog button, KDE's KFontRequester label and button)*;
  `requester_label`; `requester_separator`; `family_row_label` (drawn in its own face); `preview`;
  `filter_title`.
- **Widget values:** `grid_gap`; `row_margins`; `preview_min_lines` (`u32`).
- **Variants:** keyed KDE `@sunken_label` (the requester's styled panel).
- **may_be_absent:** `requester_separator` (KDE; GNOME hides it); `filter_title` (KDE).
- **State-model source:** GNOME `_buttons.scss` (`button.font`), `gtkfontchooserwidget.ui`; KDE
  `kfontrequester.cpp`, `kfontchooser.cpp`.
- **Today's fields:** none.

### form

*From group 6's catalogue.*

- **Kind:** typed widget: preference pages, groups and their rows (GN-2.01–GN-2.10, Q2.45–Q2.47, Q2.55†,
  Q2.56†, MAC-N21, W2-40†). A boxed list's rounding and row hover stay `list`'s (GN-1.165); a row's switch,
  entry, combo and buttons are those widgets' elements in context.
- **Parts:** `group_title`; `group_description`; `section_title`; `field_label`; `card` (the boxed group's
  background, Kirigami's cards, SwiftUI's section card, Windows' settings card); `row` *(element)*;
  `row_title`; `row_subtitle`; `row_prefix`; `row_suffix`; `row_icon`; `row_edit_icon` (AdwEntryRow);
  `row_indicator`; `row_arrow` (combo row, action entry, expander); `row_separator`.
- **Widget values:** `group_gap`; `page_margins` (`Sides<Value<f32>>`); `page_gap`; `row_gap`;
  `column_gap`; `label_alignment` (enum per mode: wide, narrow); `narrow_max_width` (`Value<f32>`);
  `collapse` (enum: when the wide layout does not fit, KDE; by width thresholds, Windows' wrap thresholds and
  GNOME's preferences-dialog thresholds as `Vec<(Key, Value<f32>)>`); `title_shrink` (`Transition`: AdwEntryRow's
  title to caption, GN-2.04); `separator_hide` (`Transition`: Kirigami hides a separator beside a hovered
  entry).
- **Variants:** typed row kinds `action`, `entry`, `combo`, `switch`, `button`, `expander`, `spin`,
  `property`; typed `primary` (group 6 wrote `suggested`), `destructive` (button rows, GN-2.08); typed `monospace` (GNOME property and
  entry rows); keyed GNOME `@error`, `@warning`, `@success` (entry rows); keyed KDE `@cards`, `@flat` (Kirigami Forms style); Windows layouts `@right`,
  `@right_wrapped`, `@right_wrapped_no_icon`, `@left`, `@vertical`.
- **Flags:** `has_text` (AdwEntryRow's title shrinks), `clickable` (Kirigami's hover fill only on clickable
  entries); position rules (`first`, `last`) round the card's outer corners and hide separators beside a hovered
  neighbour (`previous`/`next` sibling with `{hover}`).
- **Exclusive groups:** Windows `CommonStates` on the settings card [disabled, pressed, hover].
- **may_be_absent:** `row_edit_icon`, `row_indicator` (all but GNOME); `field_label` (GNOME, macOS);
  `card` (KDE flat style).
- **State-model source:** GNOME `_lists.scss`, `_preferences.scss`; KDE `FormLayout.qml`,
  `forms/cards/*.qml`; macOS SwiftUI `GroupedFormStyle`; Windows SettingsCard and SettingsExpander states.
- **Today's fields:** none (`list.*`, `expander.header_height`, `layout.*` stand-ins stay theirs).

- **Merged from other groups and rows:**
  - kirigami-addons: keyed KDE `@ka_form_card` (K-KA-1†–K-KA-9†), `@width_limited` (set by the connector from `values.card_max_width`, R16), `@ka_info_card` (K-KA-15†), `@date_time`, `@icon_row`, `@check`, `@radio`, `@link`; part `page`; values `card_max_width`, `icon_text_gap`, `grid_columns`, `grid_min_cell_width`, `grid_gap`, `field_label_gap`, `arrow_gap`, `leading_gap`, `trailing_gap`, `row_separator_inset`, `presentations`.

### glass_container

*From group 7's catalogue.*

- **Kind:** typed widget: iOS C-20 (`UIGlassContainerEffect`) and macOS MAC-X04 (`NSGlassEffectContainerView`).
  Owned jointly with g8 (MAC-X04).
- **Parts:** `surface` (a `Material` paint with identity `glass`, tint, interactivity as a variant).
- **Widget values:** `merge_gap: Value<f32>` (the distance within which neighbouring glass elements merge).
- **Variants:** typed `regular`, `clear`; flags: `Hover`/`Pressed` for the interactive glass.
- **may_be_absent:** everything but the 26 presets.
- **State-model source:** UIKit/AppKit glass-effect style enums.
- **Today's fields:** none.

- **Merged from other groups and rows:**
  - Group 8: the interactive glass's press response and morph are `PartStyle.transitions` (X-MT-8, X-MT-9); the merge spacing is `values.merge_gap` (group 8 had placed it in `Material.params`).

### group_box

*From group 6's catalogue.*

- **Kind:** typed widget (group 6 named it `frame`): a frame with an optional title (GN-2.32, C-32, K2.8, Q2.12, Q2.13, K2.21, MAC-N12).
- **Parts:** `body`; `title`; `focus_line` (KDE's line under a focused title); `title_check` (a `checkbox`
  element in context: KDE's checkable title).
- **Widget values:** `title_margin`; `title_position` (enum: inside the frame, on its edge, above).
- **Variants:** typed `flat` (KDE QGroupBox flat; GNOME `.flat`); keyed KDE `@box`, `@panel`,
  `@win_panel`, `@styled_panel`, `@quick_frame`, `@quick_group_box`; macOS `@photo`, `@gray_bezel`,
  `@groove`, `@button`.
- **States:** hover and focus outlines (KDE's framed scroll area).
- **may_be_absent:** `title`, `focus_line` (where none).
- **State-model source:** GNOME `_misc.scss` (`frame`); iOS SwiftUI `GroupBoxStyle`; KDE Breeze
  `PE_FrameGroupBox`, `CE_ShapedFrame` and frame-shadow branches, `Frame.qml`, `GroupBox.qml`; macOS
  `NSImageFrameStyle`.
- **Today's fields:** none (`card.*` stand-ins stay `card`'s).

- **Merged from other groups and rows:**
  - Group 4: NSBox's `title` position and gap (MAC-BX01, K1.211); keyed macOS `@primary`, `@custom`, `@separator`, `@transparent` (MAC-BX02). Group 6 named this widget `frame`; the merge keeps `group_box`, which does not clash with the `frame` parts of `window`, `scroll_view` and `calendar`.

### image_cropper

*From group 6's catalogue.*

- **Kind:** typed widget (MAC-P4-12†, W-RV-1†).
- **Parts:** `frame`; `mask` (outside the crop); `outline`; `overlay`; `thumb` *(element)*.
- **Widget values:** `padding` (`Sides<Value<f32>>`).
- **Variants:** typed `corner`, `edge` on `thumb` (Windows' primary and secondary styles; ImageKit's handles,
  measured).
- **Exclusive groups:** Windows `CommonStates` per thumb.
- **State-model source:** ImageKit `IKImageView` crop tool (measured); Windows `ImageCropperThumb.xaml`.
- **Today's fields:** none.

### ink_toolbar

*From group 6's catalogue.*

- **Kind:** typed widget (W2-36, C-40†).
- **Parts:** `bar`; `tool_button` *(element)*; `palette` (the pen colour flyout); `palette_swatch`
  *(element)*; `stroke_width_selector`; `ruler`.
- **Widget values:** none beyond placement of the flyout (`PlacementStrategy`).
- **Rules:** iOS's palette follows the scheme the application sets on it (`colorUserInterfaceStyle`), the
  scheme condition.
- **State-model source:** Windows InkToolbar states; PencilKit `PKToolPicker` (measured).
- **Today's fields:** none.

### keycap

*From group 7's catalogue.*

- **Kind:** typed widget, because two platforms draw a keycap: GNOME's shortcut label (GN-2.40) and KDE's
  `KCommandBar` keys (K-RL01†). Windows, macOS, iOS and Material draw shortcuts as text (cross-check, all 3).
  The input listed GN-2.40 as one platform's component; the KDE row, filed as "command bar", is the second.
- **Parts:** `key` (the cap: fill, stroke (inset line), corners, padding, min width, text style), `separator`
  (text between keys, "+" or ", "), `label` (the container of keys and separators; focus indicator).
- **Widget values:** `key_gap: Value<f32>` (label spacing between caps), `key_horizontal_margin: Value<f32>`.
- **Variants:** keyed `@legacy` (GTK's deprecated shortcut label: other padding, other focus offset);
  `@on_selected` (separators take the selected row's text colour in KDE). **Sizes:** none.
- **Flags:** `Focused`/`FocusVisible` for `label`. **Exclusive groups:** none.
- **may_be_absent:** `separator` (GNOME), `label.focus` outside the legacy variant.
- **State-model source:** libadwaita selectors by file (`_shortcuts-dialog.scss:34-57`, `_deprecated.scss:344-364`);
  KCommandBar's paint code (`kcommandbar.cpp:299-387`).
- **Today's fields:** none.

- **Merged from other groups and rows:**
  - KDE's Quick command bar shows shortcuts as plain text, not keycaps (K-KA-42†).

### level_indicator

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.26, MAC-N06, MAC-N22, C-30, W2-41†).
- **Parts:** `track`; `fill` (the value); `block` (a discrete segment); `tick`; `scale_tick` (Windows'
  scale ticks); `needle`; `value_label`; `unit_label`; `scale_arc`; `trail_arc` (Windows), `cap` (iOS's
  end caps).
- **Widget values:** `block_gap` (`Value<f32>`); `block_min_length` (`Value<f32>`); `thickness`;
  `start_angle_deg`, `end_angle_deg` (circular); `thresholds` (`Vec<(Key, f32)>`: where low, high, full or
  warning, critical begin, stated by the platform's defaults: GtkLevelBar offsets, NSLevelIndicator
  warning/critical values).
- **Variants:** typed `continuous`, `discrete` (GNOME, macOS), `linear`, `circular` (iOS, macOS SwiftUI,
  Windows radial), `vertical` (GNOME; macOS orientation); keyed macOS `@relevancy`, iOS `@accessory`,
  `@capacity`.
- **Sizes:** macOS control sizes.
- **Flags:** GNOME `low`, `high`, `full`; macOS `warning`, `critical` (the connector sets the flag the value
  falls in by `values.thresholds`).
- **Exclusive groups:** none.
- **may_be_absent:** `needle`, `scale_arc`, `trail_arc`, `unit_label`, `value_label` (all but Windows and the
  SwiftUI gauges); `block` (iOS, Windows); `tick` (GNOME, iOS); `cap` (all but iOS).
- **State-model source:** GNOME `_level-bar.scss`; macOS `NSLevelIndicator.Style`; iOS and macOS SwiftUI
  `GaugeStyle`; Windows `RadialGauge.xaml` states (Normal, Disabled).
- **Today's fields:** none (`progress_bar.fill_color`, MAC-N06's stand-in, stays `progress_bar`'s).

### media_controls

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.54†, W2-35, C-42†, MAC-N30†).
- **Parts:** `bar`; `button` *(element: play, pause, full screen, …, keyed by variant)*; `time_label`;
  `volume_popup`; `route_button` *(element: AirPlay)*.
- **Widget values:** `auto_hide_ms` (`Value<f32>`); `presentations` (`@show`, `@hide`: GNOME's crossfade,
  Windows' fade-out); `gap`. The seek bar is a `slider` and the volume scale a `slider` in context.
- **Variants:** keyed macOS `@inline`, `@floating`, `@minimal`; buttons keyed `@play`, `@volume`,
  `@full_screen`, `@frame_step`.
- **Flags:** `route_active` (iOS's active tint).
- **Exclusive groups:** Windows `CommonStates` per button.
- **may_be_absent:** `route_button` (GNOME, Windows); `volume_popup` (iOS).
- **State-model source:** GNOME `gtkmediacontrols.ui`, `gtkvideo.c`; Windows MediaTransportControls visual
  states; iOS and macOS AVKit control styles.
- **Today's fields:** none.

- **Merged from other groups and rows:**
  - Group 7: `play_overlay` (the centred play image over the video, GN-2.44) and `cast_button` (W-P4-18†).
  - kirigami-addons: keyed KDE `@ka_album` (K-KA-35†), value `volume_popup_hide_delay_ms`.

### menu_bar

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.30, K2.1, Q2.9, Q2.10, MAC-N20, MAC-N34†, W2-22, C-37†).
- **Parts:** `bar`; `item` *(element: a top-level title)*; `item_label`; `app_name` (macOS's bold application
  menu title); `status_item` *(element: macOS status items)*; `extra_window` (SwiftUI `MenuBarExtra`'s window,
  MAC-N34†); `focus_line` (KDE's weak-focus line).
- **Widget values:** `height` (`Value<f32>`); `item_gap`; `status_item_gap`; `submenu_corners`
  (`Corners`, GNOME's submenu popovers); `reveal` (`Presentations`, iPadOS's hidden bar shown at the top edge);
  `extra_window_gap` (`Value<f32>`).
- **Variants:** keyed KDE `@quick` (Kirigami MenuBar where it differs; also the `kirigami` flavour).
- **Sizes:** macOS: the "Menu bar size" accessibility setting is the condition `ContainerSize { container:
  menu_bar, size: large }` (S7.1), not a size.
- **Flags:** `weak_focus` (KDE).
- **Exclusive groups:** Windows `CommonStates` [disabled, pressed, hover] with `open` (menu shown) as
  its Selected state.
- **may_be_absent:** `app_name`, `status_item`, `extra_window` (all but macOS); `focus_line` (all but KDE);
  `reveal` (all but iOS).
- **State-model source:** GNOME `_menus.scss` (`menubar > item`); KDE Breeze `CE_MenuBarItem` branches and
  `MenuBarItem.qml`; macOS menu-bar title states (normal, highlighted) and appearance; Windows `MenuBarItem`
  visual states; iPadOS `UIMainMenuSystem`.
- **Today's fields:** none (today's `menu.*` are the open menus' and stay there).

### navigation_view

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.18, Q2.14, Q2.15): the stack that pushes and pops pages.
- **Parts:** `page`; `edge_shadow` (the shade beside a page sliding over another; group 6 wrote `transition_shadow`); `scrim` (group 6 wrote `dimming`: the scrim
  over the page beneath).
- **Widget values:** `presentations` (`Presentations` with keyed events `@push`, `@pop`, `@replace`,
  `@layer_push`, `@layer_pop`; several entries per event run together: ADD-B10, ADD-A25, ADD-A26);
  `swipe_back` (`bool`); `swipe_duration_min_ms`, `swipe_duration_max_ms` (`Value<f32>`).
- **Variants:** keyed KDE `@layer` (PageRow layers).
- **Rules by condition:** reduced motion (libadwaita crossfades).
- **may_be_absent:** `edge_shadow`, `scrim` (KDE); `swipe_back` false where absent.
- **State-model source:** GNOME `adw-navigation-view.c` transitions and `_misc.scss`; KDE `StackView.qml`
  transitions and `PageRow.qml`.
- **Today's fields:** none.

- **Merged from other groups and rows:**
  - Group 8: `edge_shadow`, `scrim` (GN-3.04, GN-3.05; group 6 wrote `transition_shadow`, `dimming`); presentations (X-MO-3's push with parallax, MAC-P3-10†'s page controller styles, ADD-A26); GtkStack's transitions (GN-3.22; group 8 wrote `stack`).

### page_indicator

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.38 indicators, GN-2.55†, C-9, Q2.11, W2-20, W2-21).
- **Parts:** `dot` *(element; the current page is `selected`)*; `line` (GNOME's line indicator); `platter`
  (iOS's background); `progress` (iOS's timer fill); `nav_button` *(element)*; `page_button` *(element:
  Windows' number panel)*; `selection_indicator` (Windows PagerControl).
- **Widget values:** `dot_gap`; `max_visible` (`u32`); `nav_buttons_visibility` (enum: always, on hover);
  `follows_scroll` (`bool`: GNOME interpolates size and opacity with the scroll position, ADD-B13); `edge_dots_shrink` (`bool`, iOS).
- **Variants:** typed `vertical` (iOS direction, Windows orientation); keyed GNOME `@lines`; iOS
  `@automatic`, `@prominent`, `@minimal`; Windows `@button_panel`, `@combo_box`, `@number_box`.
- **Flags:** `scrubbing` (iOS).
- **Exclusive groups:** Windows `CommonStates` per pip and button.
- **may_be_absent:** `line` (all but GNOME); `platter`, `progress` (all but iOS); `page_button`,
  `selection_indicator`, `nav_button` (all but Windows).
- **State-model source:** GNOME `adw-carousel-indicator-*.c`; iOS `UIPageControl`; KDE `PageIndicator.qml`;
  Windows PipsPager and PagerControl states.
- **Today's fields:** none.

### picker_wheel

*From group 6's catalogue.*

- **Kind:** typed widget (C-7, MAT-N31†, W2-04's looping selectors).
- **Parts:** `container`; `item` *(element; the centred item is `selected`)*; `selection_band`.
- **Widget values:** `item_height`; `visible_items` (`u32`); `curvature` (`Value<f32>`); `off_centre_fade`;
  `component_gap`; `snap` (`Transition` with a spring); `momentum` (`ScrollPhysics`).
- **Variants:** none typed.
- **Interpolation by scroll position** (Material's text style between two ramp roles, iOS's fade and
  curvature): ADD-B13.
- **State-model source:** iOS `UIPickerView`; Material `ScrollField.kt`; Windows `LoopingSelector` states.
- **Today's fields:** none.

- **Merged from other groups and rows:**
  - kirigami-addons: the time popup's tumblers (K-KA-23†), the item opacity by distance from the centre (ADD-B13).

### pull_to_refresh

*From group 6's catalogue.*

- **Kind:** typed widget (C-10, Q2.54†, MAC-N24, MAT-N28, W2-26).
- **Parts:** `indicator` (container); `spinner` (the pull-progress arc or spokes; while refreshing a `spinner`
  element in its context); `arrow` (Windows' arrow glyph); `title` (iOS's attributed title); `strip`
  (Kirigami's strip above the content).
- **Widget values:** `trigger_distance` (`Value<f32>`); `refresh_delay_ms` (`Value<f32>`, Q2.54†);
  `content_shift` (`Value<f32>`); `presentations` (`@collapse`, `@content_shift`); `spin` (`Loop`, Windows);
  `progress_sweep` (enum: arc by pull progress, spokes by pull progress).
- **Variants:** none.
- **Flags:** `pulling`, `refreshing`.
- **may_be_absent:** `arrow` (all but Windows); `title` (all but iOS); `strip` (all but KDE).
- **State-model source:** iOS `UIRefreshControl`; KDE `PullDownIndicator.qml`; macOS `NSScrollView`
  refresh controller (27); Material Compose `PullToRefresh` states; Windows `RefreshVisualizerState`.
- **Today's fields:** none.

### rating

*From group 6's catalogue.*

- **Kind:** typed widget (K2.19, W2-06, MAC-N06's rating style).
- **Parts:** `star` *(element)*; `caption`.
- **Widget values:** `item_gap`; `half_steps` (`bool`); `glyph` (`Glyph` source of the star).
- **Flags:** `filled`, `half`, `placeholder`, `preview` (hover preview, KDE and Windows).
- **Exclusive groups:** Windows `CommonStates`.
- **State-model source:** KDE `kratingwidget.cpp`; Windows RatingControl states; macOS
  `NSLevelIndicator.Style.rating`.
- **Today's fields:** none.

### redaction

*Written in the merge.*

- **Kind:** typed widget (macOS MAC-P4-13†, iOS P4-18†: SwiftUI's `redacted(reason: .placeholder)`): the look of
  content drawn as a placeholder mask while it loads, list rows included. Group 8 proposed it as `placeholder`; the
  merge renames it, because `placeholder` is the text widgets' part (placeholder text). An element is redacted by the
  state flag `Redacted` (ADD-A14); the mask's look is this widget's, queried in the redacted element's context.
- **Parts:** `mask` — the shape that replaces text and images, keeping their size: fill per scheme, corners, the shape
  per line of text, transitions (an animation, if the platform draws one).
- **Widget values:** none stated by the rows.
- **Variants, sizes, densities:** none.
- **Widget-specific flags:** none; the redacted element carries `Redacted`.
- **Exclusive groups:** none.
- **may_be_absent:** `mask.transitions`.
- **State-model source:** SwiftUI `RedactionReasons` (placeholder) on macOS and iOS, measured.
- **Today's fields:** none.

### rich_text

*From group 6's catalogue.*

- **Kind:** typed widget (K-SF12†–K-SF17†, K-P4-14†, MAC-SF02†–MAC-SF04†, MAC-SF06†, SF-2†–SF-4†, SF-6†,
  MAT-P3-20†).
- **Parts:** `paragraph`; `heading_1` … `heading_6`; `emphasis`; `strong`; `strikethrough`; `subscript`;
  `superscript`; `inline_code`; `code_block`; `preformatted`; `block_quote`; `list_marker`; `task_box`;
  `table`; `table_cell`; `table_header_cell`; `rule` (horizontal rule); `link` *(element)*; `definition_term`;
  `definition_description`.
- **Widget values:** `block_margins` (`Vec<(PartName, Sides<Value<f32>>)>`: each block part's margins, as
  Qt's HTML and Markdown defaults state them, K-SF14†, K-SF17†, K-P4-14†); `list_indent_per_level`;
  `list_marker_gap`; `list_marker_kinds` (`Vec<Key>` per nesting level); `quote_indent_per_level`.
- **Variants:** typed `markdown` (KDE Markdown importer, SwiftUI `Text` Markdown), `html` (KDE's HTML parser,
  Apple's HTML import, Compose `AnnotatedString.fromHtml`).
- **Flags:** `checked` uses the common flag on `task_box`.
- **may_be_absent:** under `markdown` on macOS and iOS every block part (headings, lists, quotes, code
  blocks, tables, rule: MAC-SF02†, SF-2†); `task_box` (all but KDE); `definition_*` (all but KDE's HTML);
  `subscript`, `superscript` (KDE Markdown).
- **State-model source:** KDE `qtextmarkdownimporter.cpp`, `qtextdocumentlayout.cpp`, Qt's HTML parser
  defaults; Apple `InlinePresentationIntent`, `PresentationIntent`, the HTML importer; Compose
  `Html.android.kt` span mapping. Links: hover, pressed, visited where the platform draws them.
- **Today's fields:** none. Stand-ins: `defaults.mono_font` (K-SF13†, MAC-SF03†, SF-3†) becomes the text
  ramp's `Monospace` role referenced from `inline_code.text`; `defaults.link_color` (MAC-SF04†, SF-4†) the
  link role referenced from `link.text.fill`.

### rubber_band

*From group 6's catalogue.*

- **Kind:** typed widget (C-47†, K2.12, MAC-N31†).
- **Parts:** `body`.
- **Widget values:** `presentations` (`@disappear`: iOS's fade).
- **Rules in context:** filled inside a list, graphics view or main window; outline only elsewhere (KDE: the
  base is the outline, a `context any list` rule adds the fill).
- **State-model source:** iOS `UIBandSelectionInteraction`; KDE Breeze `CE_RubberBand`; macOS as drawn by
  Finder and `NSCollectionView` (measured).
- **Today's fields:** none.

- **Merged from other groups and rows:**
  - Group 4: the list's marquee (GN-1.175, GN-1.280†). Group 8: KDE's dock drop zone shows it filled in `context window`, outlined elsewhere (K-SF18†).

### ruler

*From group 6's catalogue.*

- **Kind:** typed widget (K2.32†, MAC-N17).
- **Parts:** `body`; `tick` (variants by level); `label`; `marker` *(element: macOS markers, KDE's pointer)*.
- **Widget values:** `thickness`; `tick_lengths` (`Vec<(Key, Value<f32>)>`: end, big, medium, little, base).
- **State-model source:** KDE `kruler.cpp`; macOS `NSRulerView` (measured).
- **Today's fields:** none.

### search_field

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.21, GN-2.22, C-3, MAC-N02, MAT-N14, MAT-N15, W2-15). Its text is an `input`
  element in its context.
- **Parts:** `field` (the bezel or container); `search_icon`; `placeholder`; `clear_button` *(element)*;
  `cancel_button` *(element, iOS)*; `query_button` *(element, Windows)*; `recents_indicator` (macOS);
  `leading_icon`, `trailing_icon`, `avatar` (Material); `supporting_text`; `token` *(element: iOS search
  tokens)*; `bar` (GNOME's search bar, Material's search bar container); `bar_close_button` *(element)*;
  `view` (Material's search view); `view_header`; `view_divider`.
- **Widget values:** `placeholder_alignment` (enum: centred, leading) with the transition when focus moves it
  (macOS, MAC-N02); `presentations` (`@cancel_appear`, `@cancel_disappear`, `@bar_reveal`, `@view_open`);
  `suggestions` (an `autocomplete` element in context, W2-15).
- **Variants:** keyed GNOME `@inline`, `@flat`, `@shrunk` (search bar); iOS `@default`, `@prominent`,
  `@minimal`; Material `@docked`, `@full_screen` (view), `@contained` (Expressive).
- **Sizes:** macOS control sizes.
- **Flags:** `has_text` (clear button shown).
- **Rules by context:** inside `toolbar` (macOS toolbar search item, iOS 26's glass capsule in a toolbar or
  tab bar).
- **Exclusive groups:** Windows `CommonStates` per button [disabled, pressed, hover].
- **may_be_absent:** `cancel_button`, `token` (all but iOS); `query_button` (all but Windows);
  `recents_indicator` (all but macOS); `avatar`, `view*` (all but Material); `bar`, `bar_close_button`
  (macOS, Windows, iOS).
- **State-model source:** GNOME `_toolbars.scss` (`searchbar`), `_entries.scss`; iOS `UISearchBar`,
  `UISearchTextField`, `UISearchBar.Style`; macOS `NSSearchField` cell states; Material search-bar and
  search-view tokens; Windows AutoSuggestBox visual states.
- **Today's fields:** none (`input.*`, the stand-in, stays `input`'s).

- **Merged from other groups and rows:**
  - Group 4: in a toolbar (MAT-TA6, MAC-TL07), `context toolbar.bar`.
  - kirigami-addons: keyed KDE `@ka_search_popup` (K-KA-32†), values `view_placement`, `auto_search_min_chars`.

### sheet

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.17, C-12, Q2.37–Q2.40, K-SF23†, MAT-N19–N21, MAT-SF03†).
- **Parts:** `body`; `handle` (the grabber); `header`; `title`; `header_separator`; `footer`;
  `footer_separator`; `scrim` (group 6 wrote `dimming`); `bottom_bar` (AdwBottomSheet's always-visible bar); `divider` (Material side
  sheet); `close_button` *(element)*.
- **Widget values:** `detents` (`Vec<(Key, Value<f32>)>`); `undimmed_up_to` (`Option<Key>`, iOS);
  `max_width` (`Value<f32>`); `min_top_inset`; `anchor` (enum: bottom centred, centred, side); `swipe_to_close`
  (`Value<f32>` threshold); `presentations` (`@open`, `@close`, `@settle`, `@bottom_bar_reveal`, springs);
  `presenter_transform` (`Transform`) and `presenter_corners` (`Corners`) (iOS 13–18's stacked-card effect
  on the presenter); `top_padding_range` (GNOME's top padding by height).
- **Variants:** typed `bottom` (GNOME, iOS, Material), `floating` (Material floating, iOS 26's inset sheet at
  partial detents, to verify); keyed GNOME `@flush`; iOS `@page`, `@form`, `@edge_attached`; Material
  `@side`, `@modal`, `@standard`, `@detached`; KDE `@overlay` (centred OverlaySheet).
- **Conditions and sizes:** reduced motion; size class (Material's wide windows, iOS's compact height);
  `MobileFormFactor` (Kirigami's drag handle).
- **Flags:** `minimised` (Material).
- **may_be_absent:** `handle` (KDE desktop, iOS without grabber); `bottom_bar` (all but GNOME); `divider`,
  `close_button` (all but Material); `presenter_*` (all but iOS).
- **State-model source:** GNOME `_bottom-sheet.scss`, `adw-bottom-sheet.c`; iOS
  `UISheetPresentationController`; KDE `OverlaySheet.qml`; Material sheet tokens and `ModalBottomSheet.kt`.
- **Today's fields:** none (`dialog.*` stand-ins stay `dialog`'s).

- **Merged from other groups and rows:**
  - Group 1: iOS's placement per size class and with the keyboard (SF-7†, SF-8†, P3-6†): `values.anchor` by `h_size_class` and `v_size_class` (ADD-A20), `values.keyboard_avoidance`.
  - Group 8: `handle` (Q3.31; group 8 wrote `grabber`), `scrim` (X-SC-1, MAT-X23; group 6 wrote `dimming`), presentations per modal style (X-MO-4).
  - kirigami-addons: keyed KDE `@ka_bottom_drawer` (K-KA-30†, K-KA-31†).

### shortcuts_dialog

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.41, K-RL02†).
- **Parts:** `nav_pill` *(element)*; `section_title`; `row`; `row_title`; `row_subtitle`;
  `key_sequence_button` *(element)*; `clear_button` *(element)*; `extender_indicator`.
- **Widget values:** `page_gap`; `row_min_height`.
- **Flags:** `recording` (KDE's key-sequence button held pressed), `expanded` (the in-row editor open:
  `open`).
- **may_be_absent:** `nav_pill` (KDE); `key_sequence_button`, `clear_button`, `extender_indicator` (GNOME).
- **State-model source:** GNOME `_shortcuts-dialog.scss`; KDE `kshortcutseditor*.cpp`,
  `kkeysequencewidget.cpp` and Breeze button branches.
- **Today's fields:** none.

- **Merged from other groups and rows:**
  - kirigami-addons: keyed KDE `@ka_quick` (K-KA-43†), part `footer`, value `natural_size` (the dialog layer's size).

### split_button

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.29, MAC-N28†, MAT-N7, W2-17).
- **Parts:** `primary` *(element)*; `secondary` *(element: the arrow part)*; `primary_label`;
  `primary_icon`; `arrow`; `divider`.
- **Widget values:** `part_gap` (`Value<f32>`, negative where the parts overlap, GNOME); `divider_margin`.
- **Variants:** typed `primary` (group 6 wrote `suggested`), `destructive`, `flat` (GNOME, and Windows' accent style as `suggested`);
  typed `toggle` (Windows ToggleSplitButton; its `checked`); keyed GNOME `@opaque`.
- **Sizes:** Material five sizes (`ControlSize` Mini to ExtraLarge as Material's xsmall to xlarge); macOS
  control sizes.
- **Flags:** none (`open` on `secondary` rotates `arrow`, Material).
- **Rules:** inner corners by state (Material: rest, hovered, pressed, selected); the divider hidden on hover
  and focus (GNOME, flat).
- **Exclusive groups:** Windows `CommonStates` per part.
- **State-model source:** GNOME `_buttons.scss` (`splitbutton`); macOS `NSComboButton.Style.split`;
  Material split-button tokens; Windows SplitButton visual states.
- **Today's fields:** none.

- **Merged from other groups and rows:**
  - kirigami-addons: keyed KDE `@ka_double_floating` (K-KA-27†). Group 6's typed `suggested` is `primary` (group 2's name for the suggested look).

### status_page

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.13, C-16, Q2.30–Q2.32, MAC-N35†).
- **Parts:** `icon`; `title`; `description`; `action_area`; `spinner` (GNOME's spinner variant; a `spinner`
  or `progress_bar` element in context).
- **Widget values:** `margins`; `icon_to_title`, `title_to_description`, `description_to_actions`
  (`Value<f32>`); `max_width`; `alignment`.
- **Variants:** typed `compact` (GNOME), `loading` (GNOME spinner, iOS `.loading()`, KDE LoadingPlaceholder),
  `search` (iOS `.search()`, macOS search variant); keyed KDE `@informational`, `@actionable`.
- **may_be_absent:** `spinner` (unless `loading`); `action_area` where empty.
- **State-model source:** GNOME `_misc.scss` (`statuspage`); iOS `UIContentUnavailableConfiguration`; KDE
  `PlaceholderMessage.qml`, `LoadingPlaceholder.qml`; macOS `ContentUnavailableView`.
- **Today's fields:** none (`text_scale.display`, GN-2.13's stand-in, becomes a text-ramp reference).

- **Merged from other groups and rows:**
  - Group 3: in context of the spinner (GN-1.232) and the busy indicator (Q1.121).
  - kirigami-addons: keyed KDE `@ka_card_placeholder` (K-KA-16†).

### stepper

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.05, GN-2.24, C-4, K2.3, Q2.1, MAC-N01, W2-01). Where the platform pairs it with
  a text field (spin box), the text is part of the stepper.
- **Parts:** `body`; `text`; `increment` *(element)*; `decrement` *(element)*; `increment_glyph`;
  `decrement_glyph`; `separator` (between the buttons, or between text and buttons); `popup` (Windows'
  compact spin popup); `header` (Windows' header label).
- **Widget values:** `button_layout` (enum: column at the end, both ends, stacked above and below, segments
  side by side); `repeat_delay_ms`, `repeat_interval_ms` (`Value<f32>`: C-4, MAC-N01);
  `field_gap` (`Value<f32>`: macOS's field-to-stepper gap); `popup_offset_x`, `popup_offset_y`;
  `invalid_input` (enum: revert, keep: W2-01).
- **Variants:** typed `vertical` (GNOME vertical spin button; Windows popup is not this); keyed GNOME `@row`
  (AdwSpinRow's circular buttons), KDE `@flat`, Windows `@inline`, `@compact`.
- **Sizes:** macOS control sizes.
- **Flags:** none (a half at its limit is `disabled` on its element).
- **Exclusive groups:** Windows `CommonStates` per spin button [disabled, pressed, hover].
- **may_be_absent:** `text` (iOS, macOS); `popup`, `header` (all but Windows); `separator` where none is drawn.
- **State-model source:** GNOME `_spin-button.scss`; iOS `UIStepper` + `UIControl.State`; KDE Breeze
  `CC_SpinBox` branches (per-arrow hover); macOS `NSStepper` cell states and sizes; Windows NumberBox and its
  spin-button visual states.
- **Today's fields:** none.

- **Merged from other groups and rows:**
  - Group 8: `values.repeat_delay`, `values.repeat_interval` (GN-3.69†, K3.76†; group 8 wrote `spin_button`).

### teaching_tip

*From group 6's catalogue.*

- **Kind:** typed widget (C-36†, MAC-N36†, W2-09, K2.31†).
- **Parts:** `body`; `tail`; `top_highlight` (Windows); `hero`; `image`; `title`; `message`; `close_button`
  *(element)*; `action_area`.
- **Widget values:** `placement` (`PlacementStrategy`); `min_width`, `max_width`; `tail_size`, `tail_margin`;
  `light_dismiss` (`bool`); `presentations` (`@expand`, `@contract`, `@appear`, `@disappear`); `text_insets`
  (KDE).
- **Variants:** typed `inline`, `popover` (TipKit both); keyed Windows `@alternate_close`, `@hero_bottom`.
- **may_be_absent:** `tail` (inline, KDE); `top_highlight`, `hero` (all but Windows); `title`, `image` (KDE).
- **State-model source:** iOS and macOS TipKit `TipView` styles; Windows TeachingTip visual states; KDE
  `qwhatsthis.cpp`.
- **Today's fields:** none.

- **Merged from other groups and rows:**
  - kirigami-addons: keyed KDE `@ka_onboarding` (K-KA-45†): parts `target_frame` (the moving frame round the current control) and `backdrop` (a blur material over everything else), placement `Keyed(ka_onboarding_tip)`.

### time_picker

*From group 6's catalogue.*

- **Kind:** typed widget (K2.24†, MAT-N17, W2-04).
- **Parts:** `field` *(element)*; `field_segment`; `segment_separator`; `header`; `flyout`;
  `highlight_band`; `accept_button`, `dismiss_button` *(elements)*; `dial`; `dial_selector`;
  `dial_track`; `dial_centre`; `dial_label`; `selector_cell` *(element)*; `period_selector` *(element)*;
  `time_separator`; `headline`.
- **Widget values:** `field_min_width`, `field_max_width`; `item_height`; `presentations` (`@open`);
  `invalid_entry` (enum: message box, KDE).
- **Variants:** keyed Material `@dial`, `@input`, `@vertical`, `@horizontal`; KDE `@combo`, `@spin_box`.
  The looping selectors are `picker_wheel` elements in context.
- **Exclusive groups:** Windows `CommonStates`.
- **may_be_absent:** `dial*`, `period_selector` (all but Material); `flyout`, `highlight_band`,
  `accept_button`, `dismiss_button` (all but Windows).
- **State-model source:** KDE `ktimecombobox.cpp`; Material time-picker tokens; Windows TimePicker and
  DateTimePickerFlyout states.
- **Today's fields:** none.

- **Merged from other groups and rows:**
  - kirigami-addons: keyed KDE `@ka_time_popup` (K-KA-23†); its tumblers are `picker_wheel` elements in context.

### toast

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.12, GN-2.50, Q2.49–Q2.52, MAT-N22).
- **Parts:** `body`; `title`; `action_button` *(element)*; `close_button` *(element)*.
- **Widget values:** `durations` (`Vec<(Key, Value<f32>)>`: GNOME's default; KDE short, default, long;
  Material short, long); `natural_width`, `max_width`; `margins` (`Sides<Value<f32>>`); `stack_gap`;
  `stack_max_height`; `presentations` (`@appear`, `@disappear`, `@replace`, `@displace`).
- **Variants:** keyed GNOME `@app_notification` (the legacy in-app notification).
- **Bindings:** `body` binds the OSD surface set (GNOME) or the Complementary set (KDE).
- **Flags:** `multiline` (Material's two-line height).
- **Conditions:** high contrast (GNOME's outline).
- **may_be_absent:** `close_button` (GNOME, KDE); `body.shadow` where none.
- **State-model source:** GNOME `_misc.scss` (`toast`), `adw-toast-overlay.c`; KDE
  `PassiveNotificationsManager.qml`; Material snackbar tokens.
- **Today's fields:** none.

- **Merged from other groups and rows:**
  - Group 8: `body` (GN-3.09), `values.durations` (W3-41, MAT-X25; group 8 wrote `duration_ms`).

### toolbar_editor

*Written in the merge.*

- **Kind:** typed widget: the editor in which the user customises a toolbar's items. KDE (KXmlGui's `KEditToolBar`:
  two lists and move buttons, K-P4-10†), macOS (`NSToolbar`'s customization palette, MAC-P3-6†), iOS (the customizable
  navigation-bar items, P4-10†). Group 6 listed it as KDE's keyed component `toolbar_editor` and group 4 traced the
  macOS and iOS rows to `toolbar_customization`; three platforms draw it, so it is typed (R14) under group 6's name.
  Windows' Customize Toolbar dialog is Win32 (preset, R1.3).
- **Parts:** `available_list`, `current_list` (KDE's two lists; `list` elements in context), `move_button` (the
  buttons that move an item between them; `button` elements in context); the item palette and the drop feedback of
  macOS and iOS as `toolbar` parts in context (`toolbar.item @ {dragged}`, `toolbar.bar @ {drop_target}`). The context
  menu is `menu`'s.
- **Widget values:** none of its own; the toolbar's presentation events `customize` and `items_change` carry the motion
  (P4-10†, MAC-P3-6†).
- **Variants, sizes, densities, flags, exclusive groups:** none stated.
- **may_be_absent:** `available_list`, `current_list`, `move_button` (macOS and iOS customise in place).
- **State-model source:** KDE `kedittoolbar.cpp` with Breeze's item-view and button branches; AppKit's
  customization palette (`NSToolbar.allowsUserCustomization`, MAC-P3-6†, measured); UIKit's customisable
  navigation-bar items (`UINavigationItem.customizationIdentifier`, P4-10†, measured).
- **Today's fields:** none.

### view_switcher

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.15, GN-2.16, GN-2.33, W2-27, W2-32, W2-11).
- **Parts:** `bar`; `item` *(element)*; `item_icon`; `item_label`; `indicator` (Windows' selection pill,
  Pivot's underline, the top NavigationView's bottom indicator); `nav_button` *(element)*;
  `overflow_button` *(element)*; `item_separator`.
- **Widget values:** `item_gap`; `item_min_width` per keyed policy; `drag_hover_switch_delay_ms`;
  `presentations` (`@bar_reveal`); `indicator_transition` (`Transition`: the pill's slide).
- **Variants:** keyed GNOME `@narrow`, `@wide`, `@title`, `@bar`, `@sidebar`, `@inline`, `@stack`;
  Windows `@selector_bar`, `@pivot`, `@navigation_top`. Counters are `badge` elements in context.
- **Flags:** `needs_attention` (a `badge` dot in context).
- **Exclusive groups:** Windows `CommonStates` per item.
- **State-model source:** GNOME `_view-switcher.scss`, `_toggle-group.scss`, `_buttons.scss`
  (`stackswitcher`); Windows SelectorBarItem, PivotHeaderItem, NavigationViewItem (top) states.
- **Today's fields:** none (`segmented_control.*`, `tab.*` stand-ins stay theirs).

- **Merged from other groups and rows:**
  - Group 4: `values.breakpoints` (libadwaita's preferences thresholds, GN-1.197).

### wizard

*From group 6's catalogue.*

- **Kind:** typed widget (GN-RL01†, GN-2.51, K2.22).
- **Parts:** `header`; `title`; `subtitle`; `banner`; `step_list`; `step` *(element)*; `page_frame`;
  `action_area`.
- **Widget values:** `button_order` (`Vec<Key>`: GtkAssistant's Close, Cancel, Last, Back, Forward, Apply);
  `button_gap`; `buttons_equal_width` (`bool`); `button_placement` (enum: action area, header bar).
- **Variants:** keyed GNOME `@header_bar`; KDE `@classic`.
- **Flags:** `current` (the highlighted step).
- **may_be_absent:** `step_list`, `step` (KDE); `banner`, `subtitle` (GNOME).
- **State-model source:** GNOME `gtkassistant.ui`, `_deprecated.scss`; KDE `qwizard.cpp` classic style.
- **Today's fields:** none.

### wrap_box

*From group 6's catalogue.*

- **Kind:** typed widget (GN-2.46, MAT-RL01†).
- **Parts:** none.
- **Widget values:** `child_gap`, `line_gap` (`Value<f32>`); `wraps` (`bool`).
- **Rules in context:** Material's chip set is a `wrap_box` with `chip` children.
- **Today's fields:** none.

## Keyed components

One-platform kinds (R14, S5.1): a `Component { key, parts, values }` the preset declares (S10). Group 6 wrote its components as table rows, group 7 as sections; both are kept, by platform. Group 6's `toolbar_editor` is a typed widget after the merge (three platforms draw it).

### GNOME

*From group 6:*

| Component | Platform | Parts | Values | Rows |
|---|---|---|---|---|
| `emoji_chooser` | GNOME | `section_button`, `cell`, `completion_row` | — | GN-2.43 |
| `tab_overview` | GNOME | `grid`, `thumbnail`, `thumbnail_close_button`, `attention_dot`, `new_tab_button`, `drop_target` | `grid_gap`, `columns_min`, `columns_max`, `thumbnail_width_range`, presentations `@open`, `@close`, `@reorder`, `@thumbnail_fade` | GN-2.35 |

#### component.dock_frame_drop (gnome)

*From group 7.*

- **Kind:** keyed component (GNOME's libpanel; R1.2 in). Rows GN-SF14†, GN-SF15†, GN-SF16†. No other
  platform draws a dock frame (cross-check: macOS, iOS, KDE, Windows, Material all 3, "G19: KDDockWidgets'
  artwork is that library's own", KDE's QMainWindow gap indicator K-SF18† is Qt Widgets' and is another row).
- **Parts:**
  - `frame`: the `PanelFrame` itself while a drop is over it (its own `:drop(active)` shadow is switched off
    here, so GN-3.66†'s catch-all rule does not apply: a rule under {drop_target} giving `Layers([])`);
  - `drop_overlay`: the overlay shown over the frame while a page is dragged over it (fill, margin as
    padding-free inset, corners, stroke ring inside under {drop_target}); an element (states: drop_target);
  - `drop_position_button`: one of five buttons, element, keyed identity `top`, `right`, `bottom`, `left`,
    `centre` (the centre one circular, the others with a corner radius: rules by identity); carries a glyph
    (`ThemeIcon` per identity) and a shadow; states: drop_target (the button under the drop);
  - `switcher_indicator`: the insertion mark in a frame switcher where the dragged page would land.
- **Widget values:** `button_gap: Value<f32>` (gap in the grid of five buttons); `button_layout`: a plain
  enum `CentredGrid` (the five sit in a grid centred on the frame; stated by the preset, not computed).
- **Variants:** none typed; keyed: none. **Sizes/densities:** none.
- **Flags:** `DropTarget` (typed). No widget-specific flag. **Exclusive groups:** none.
- **may_be_absent:** none (libadwaita-family only; the component is declared by `adwaita` alone).
- **Transitions:** `drop_overlay` has property transitions on fill and stroke (GN-SF14† states background and
  border transitions) in its `PartStyle.transitions`.
- **State-model source:** libadwaita-family stylesheet selectors by file: libpanel `stylesheet.css`
  (`paneldropcontrols`, `:drop(active)`), per GN-SF14†..GN-SF16† (P).
- **Today's fields:** none (no dock widget in the model).

### KDE

*From group 6:*

| Component | Platform | Parts | Values | Rows |
|---|---|---|---|---|
| `lcd_number` | KDE | `frame`, `segment` (keyed variants `@outline`, `@filled`, `@flat`) | — | K2.6 |
| `led` | KDE | `body` (keyed variants `@round`, `@rectangular`, `@raised`, `@sunken`, `@flat`; flag `@on`) | — | K2.18 |
| `page_view` | KDE | `icon_list`, `title_header`, `header_separator`, `search_container` | `margins` | K2.14 |
| `delay_button` | KDE | `body`, `progress_overlay` | `fill_transition` (A3(e)) | Q2.3 |

#### component.tool_box (kde)

*From group 7.*

- **Kind:** keyed component (QToolBox; Qt Widgets). Row K2.9. Cross-check: GNOME 3 (expanders are the
  nearest), Windows 3, macOS 3, iOS 3, Material 3.
- **Parts:** `page_button` (the tab row of one page; element; states hover, selected via `Checked`, pressed,
  disabled, focus-visible), `page_icon` (icon style), `page_label` (text style), `selected_outline` (the
  tab-shaped outline in the focus colour; stroke and per-corner radii of `page_button` under {checked}; see
  ADD-B03), `hover_outline` (the same under {hover}), `page_area` (the content frame of the open
  page).
- **Widget values:** `item_gap: Value<f32>` (`ToolBox_TabItemSpacing`), `tab_margin_width: Value<f32>`
  (`ToolBox_TabMarginWidth`), `tab_min_width: Value<f32>` (`ToolBox_TabMinWidth`, also `page_button.min_width`).
- **Variants/sizes:** none. **Flags:** typed only. **Exclusive groups:** Breeze's: disabled, pressed, hover.
- **Transitions:** the hover fade is a property transition on the outline stroke (`breezetoolboxengine.cpp`,
  duration read through the speed factor: a derived duration, S4.1).
- **may_be_absent:** none.
- **State-model source:** Breeze's state branches (`st:7385-7510`, `st:2772-2804`, `hp:743-782`) and its
  animation engine (K2.9).
- **Today's fields:** none.

#### component.dock_widget (kde)

*From group 7.*

- **Kind:** keyed component (QDockWidget title; Qt Widgets). Row K2.10. Cross-check: only inspector/utility
  panel names it (K2.10 is KDE's tool panel); GNOME, macOS, iOS, Material and Windows 3.
- **Parts:** `title_bar` (container; optional frame), `title` (text, elided at the end: `Elision::End`),
  `float_button`, `close_button` (elements; frameless glyph buttons; glyph `ThemeIcon`/standard pixmap
  `SP_DockWidgetCloseButton`; states hover, pressed, disabled), `separator` (line under the title),
  `frame` (the optional frame around a docked widget: `DockWidgetDrawFrame`).
- **Widget values:** `draws_frame: bool`, set by the reader from `breezerc [Style] DockWidgetDrawFrame`
  (`reader:kde:breezerc.Style.DockWidgetDrawFrame`). `frame` is `may_be_absent` when false.
- **Variants/sizes/flags:** none beyond typed states. **Exclusive groups:** Breeze's.
- **may_be_absent:** `frame`, `separator` (K2.10: "optional frame").
- **State-model source:** Breeze's branches (`st:7512-7586`, `st:968-969`, `st:2125-2131`).
- **Today's fields:** none.

#### component.mdi_sub_window (kde)

*From group 7.*

- **Kind:** keyed component (QMdiSubWindow; Qt Widgets). Row K2.11. Windows has no MDI control (3).
- **Parts:** `frame` (outline; inactive outline under not `@active`), `title_bar` (top-rounded corners; fill from
  the `[WM]` colour group), `title` (text), `caption_button` (one part type, element, keyed identity
  `minimise`, `maximise`, `restore`, `close` as for the window, R13), `active_separator` (the line under the
  active title bar in the highlight colour), `shadow` (the MDI window shadow, a shadow of `frame`).
- **Widget values:** `title_margin_width: Value<f32>` (`TitleBar_MarginWidth`).
- **Variants/sizes:** none. **Flags:** `@active` (the sub-window is the active one: per element, so it is a
  widget flag and not the window condition). **Exclusive groups:** none beyond the typed Breeze group.
- **may_be_absent:** `active_separator` (drawn only active), `shadow` (a window without shadow).
- **State-model source:** Breeze's branches (`st:8241-8366`, `hp:1581-1645`, `breezemdiwindowshadow.cpp:39-60`).
- **Today's fields:** none. The `[WM]` colours are colour roles (the colour-role group; row K2.11 only names
  that this component binds them).

#### component.title_widget (kde)

*From group 7.*

- **Kind:** keyed component (KTitleWidget, KWidgetsAddons; R1.2 in). Row K2.15. Cross-check: GNOME 3
  (AdwStatusPage and group headers are different), Windows 3, macOS 3, iOS 4, Material 4.
- **Parts:** `container`, `icon` (icon style), `title` (text style, one per level), `comment` (text, coloured
  by message type).
- **Widget values:** none (alignment of icon and text is the layout, `layout.*`).
- **Variants:** keyed text level `@level_1` .. per the keyed text styles of K1.270 (the declared levels);
  keyed message type `@info`, `@positive`, `@warning`, `@error`, `@plain` for `comment` (the colour is a rule
  per variant referring to the surface's status roles).
- **Sizes:** none. **Flags/exclusive groups:** none. **may_be_absent:** `icon`, `comment`.
- **State-model source:** the widget's own code (`kwa ktitlewidget.cpp:30-80`): message type enumeration.
- **Today's fields:** `text_scale` (partial): the level fonts map to keyed text-ramp entries.

#### component.multi_tab_bar (kde)

*From group 7.*

- **Kind:** keyed component (KMultiTabBar, KWidgetsAddons; R1.2 in). Row K2.16. Cross-check: GNOME 3
  (GtkNotebook with side tabs is the typed tab), macOS 3, iOS 3, Windows 4, Material 4.
- **Parts:** `bar`, `tab_button` (element; vertical or horizontal; states hover, pressed, `Checked`, disabled),
  `tab_icon`, `tab_label`, `separator` (the line on the content side of the bar).
- **Widget values:** `item_gap: Value<f32>`.
- **Variants:** keyed bar edge `@left`, `@right`, `@top`, `@bottom` (the `KMultiTabBar::Position` values; the
  list is confirmed against `kmultitabbar.cpp` in the value phase: K2.16 marks its source lines "to verify").
- **Sizes/flags/exclusive groups:** typed states; Breeze's group.
- **may_be_absent:** `tab_label` (icon-only tabs).
- **State-model source:** Breeze's `st:1400-1422` and the widget's code.
- **Today's fields:** none.

#### component.splitter_collapser (kde)

*From group 7.*

- **Kind:** keyed component (KSplitterCollapserButton). Row K2.17. Cross-check: GNOME 4, Windows 4,
  Material 4; macOS and iOS 3 (a toolbar item / a display-mode button elsewhere, other rows).
- **Parts:** `button` (the overlay button; element; states hover, pressed), `arrow` (glyph; direction by
  variant).
- **Widget values:** presentation transitions `Appear` and `Disappear`, kind `Fade` (the `QTimeLine` duration
  of K2.17; whether the speed factor scales it is read from the source).
- **Variants:** keyed side `@left`, `@right`, `@top`, `@bottom` (arrow direction).
- **Sizes/flags/exclusive groups:** none. **may_be_absent:** none.
- **State-model source:** the widget's own code (`ksplittercollapserbutton.cpp:26`, `:184`).
- **Today's fields:** `splitter.*` is the divider, not this button; none.

#### component.list_item_drag_handle (kde, flavour kirigami)

*From group 7.*

- **Kind:** keyed component (Kirigami `ListItemDragHandle`). Row Q2.43. Cross-check: GNOME 3, iOS 3, Material 3
  (their grippers are sheet handles and reorder accessories, other rows), macOS 4, Windows 4.
- **Parts:** `handle` (element; icon `handle-sort` in an `IconStyle`; opacity reduced at rest and full under
  {hover} and {pressed}: a rule on `opacity`), `dragged_row_shadow` is not stated (the row moves; no part).
- **Widget values:** presentation transition for event `drop` (the drop animation, kind `Move`, duration
  `= token long duration` times the speed factor, timing `CubicBezier` of Qt's InOutQuad); `autoscroll_step:
  Value<f32>` (one grid unit, an expression on the Kirigami grid unit token); `autoscroll_interval_ms:
  Value<f32>`.
- **Variants/sizes:** icon size is the token `icon_sizes.small_medium` (Kirigami's `smallMedium`).
  **Flags:** `Hover`, `Pressed`, `Dragged`. **Exclusive groups:** none.
- **may_be_absent:** none. Declared in the `kirigami` flavour only.
- **State-model source:** Kirigami's QML states (`ListItemDragHandle.qml:130-131`, `:159-165`, `:233-263`).
- **Today's fields:** none.

#### component.command_bar (kde)

*From group 7.*

- **Kind:** keyed component (KCommandBar, KConfigWidgets; R1.2 in). Row K-RL01†. Cross-check: Windows 3
  (`CommandBar` there is the toolbar), macOS 2 (Help-menu search, other row), iOS, GNOME, Material: no palette.
  Its keycaps are the typed `keycap` (below, a finding).
- **Parts:** `frame` (sunken styled-panel frame, frame forced on; the drop shadow is its `shadow`, a
  `ShadowLayer` coloured by the palette `Dark` role), `search_field` (the frameless search field: an
  element of the typed `input` with a keyed variant `@frameless`, a leading `search` icon and a clear
  button), `results` (the tree of actions: an element of the typed `list` with tree parts), `shortcut_cell`
  (the cell holding the keycaps), `key` and `separator` are the `keycap` widget's parts.
- **Widget values:** `preferred_width_divisor: Value<f32>` (the space's width divided by this), `preferred_height_divisor:
  Value<f32>`, `min_width`/`min_height` (on `frame`), `top_offset: Value<f32>`, `keycap_hmargin_extra: Value<f32>`
  (the "+ 2" per side on top of the focus frame margin, a length expression, see ADD-A44); placement: top-centre
  of the window's content space: `PlacementStrategy::Keyed(command_bar_top_centre)` (ADD-A21).
- **Variants/sizes/flags:** none. States of `results` rows: typed (hover, selected).
- **may_be_absent:** none.
- **State-model source:** the widget's code (`kcommandbar.cpp:158-168`, `:299-387`, `:417-440`, `:473-476`,
  `:572-605`, `:712-731`).
- **Today's fields:** none.

#### component.char_select (kde)

*From group 7.*

- **Kind:** keyed component (KCharSelect, KWidgetsAddons; R1.2 in). Row K-RL04†. Cross-check: macOS 2 and
  iOS 3 (the Character Viewer and the keyboard are system-drawn, R1.1), Windows 3, GNOME 3.
- **Parts:** `table` (the view; fill from the `View` surface's background), `cell` (one character, centred;
  element; states hover, selected, focus-visible), `cell_missing` is the same part under widget flag
  `@glyph_missing`.
- **Widget values:** `cell_width_x_height_range: (Value<f32>, Value<f32>)` (the clamp in x-heights),
  `cell_width_min_font_height: bool` (at least the font height), `cell_height_rule`: the computation of
  `kcharselect.cpp:299-307`, read in the value phase. The widest-glyph step is computed by the toolkit at
  layout from the font, as the rule's inputs are the font's metrics (no new model mechanism, note).
- **Variants/sizes:** none. **Flags:** `@glyph_missing` (the font lacks the glyph or the cell is empty: the cell is
  filled with the `Button` surface's background, else the `View`'s). **Exclusive groups:** none.
- **may_be_absent:** none.
- **State-model source:** the widget's own paint code (`kcharselect.cpp:115-131`, `:255-310`, `:1089-1121`).
- **Today's fields:** none.

#### component.album_viewer (kde)

*From the kirigami-addons rows (K-KA-34†), traced in the merge.*

- **Kind:** keyed component (kirigami-addons' `AlbumMaximizeComponent`, R1.2 in; K-KA-34†): a full-window media viewer
  above everything, on its own overlay layer. No other platform's file lists an in-application media viewer.
- **Parts:** `scrim` (the dim behind it), `header` (the header bar in the alternate background), `header_separator`,
  `title` (bold), `subtitle`, `close_button` (element), `footer` (the caption footer, height-limited),
  `footer_separator`, `image` (the shown image with its inset). Its actions are `button` elements; previous and next
  are `button` variant `@ka_floating` in context; the loading indicator a `spinner` in context; video controls the
  `media_controls` variant `@ka_album` (K-KA-35†).
- **Widget values:** `image_inset: Value<f32>`; presentation transitions for the image's zoom and rotation.
- **Variants, sizes, flags, exclusive groups:** none.
- **may_be_absent:** `subtitle`, `footer`, `footer_separator`.
- **State-model source:** the component's QML as K-KA-34† cites it (`ka/components/AbstractMaximizeComponent.qml`, `ka/components/AlbumMaximizeComponent.qml`, `ka/components/ImageMaximizeDelegate.qml`).
- **Today's fields:** none.

### macOS

#### component.token_field (macos)

*From group 7.*

- **Kind:** keyed component (`NSTokenField`). Row MAC-N03. Cross-check: macOS "chip 2" (tokens are input
  chips), iOS 2/3 (not supported), Material 2 (input chips, app layout), KDE 3 (Kirigami chips), GNOME 3.
- **Parts:** `field` (the text field part: an element of the typed `input` for the frame, text and caret),
  the tokens: `chip` elements in context (fill, stroke, corners, text; states hover, pressed, `Selected` (accent fill), disabled; the menu arrow at the token's trailing edge is `chip.menu_arrow`; the merge, after group 7's own note that a token is a chip).
- **Widget values:** `chip_gap: Value<f32>`.
- **Variants:** keyed token style, on the `chip` elements, `@default`, `@rounded`, `@squared`, `@plain_squared`
  (`NSTokenField.TokenStyle`). **Sizes:** `ControlSize` (mini .. large), AppKit's `NSControl.ControlSize`.
  **Flags:** typed. **Exclusive groups:** none.
- **may_be_absent:** `chip.menu_arrow`.
- **State-model source:** AppKit's control states and enums (`NSTokenField.TokenStyle`), appearances (R4).
- **Today's fields:** none. (A token is a chip: it is the typed `chip`, as the merge decided.)

#### component.rule_editor (macos)

*From group 7.*

- **Kind:** keyed component (`NSRuleEditor`, `NSPredicateEditor`). Row MAC-N09. Cross-check: all other
  platforms 3 (macOS table only).
- **Parts:** `row` (element; alternating stripe fills by position), `popup_button` (the criteria menus: an
  element of the typed `combo_box`, pop-up variant), `value_field` (an element of the typed `input`),
  `add_button`, `remove_button` (the round plus and minus buttons: elements of the typed `button`, circular
  variant), `nested_row_area` (the indented area of nested rows).
- **Widget values:** `indent_per_level: Value<f32>`.
- **Variants:** none. **Sizes:** `ControlSize` (NSRuleEditor rows follow row-height styles; per AppKit enum).
  **Flags:** typed. **Exclusive groups:** none.
- **may_be_absent:** `remove_button` (the last row's remove is disabled, not absent: none absent).
- **Position:** stripes need odd/even sibling position (ADD-B06).
- **State-model source:** AppKit control states and `NSRuleEditor` row and nesting mode enums (MAC-N09: M).
- **Today's fields:** none.

#### component.route_picker (macos)

*From group 7.*

- **Kind:** keyed component (`AVRoutePickerView`, AVKit). Row MAC-P3-9†. Its popover of devices and the
  Picture in Picture window are drawn by the system (out, R1.1); the button is a view the application embeds
  and configures.
- **Parts:** `button` (element; bordered or borderless), `button_glyph` (glyph, system image).
- **Widget values:** `bordered: bool` (`isRoutePickerButtonBordered`).
- **Variants:** none. **Sizes:** none stated (AVKit view). **Flags:** `@route_active` (a route is active:
  `activeTintColor`). **Exclusive groups:** none.
- **Colour per state:** read `routePickerButtonColor(for:)` for normal, highlighted, active, active-highlighted:
  `reader:macos:AVRoutePickerView.routePickerButtonColor(for:) per state`.
- **may_be_absent:** none.
- **State-model source:** AppKit/AVKit control states (`AVRoutePickerView.ButtonState`).
- **Today's fields:** none.

#### component.context_help (macos)

*From group 7.*

- **Kind:** keyed component (the `NSHelpManager` context-help window). Row MAC-P4-11†. Included under R1.1
  as a toolkit's own equivalent of a tooltip (the application sets only the text; a toolkit that draws its
  own windows draws this one); nearest model element `tooltip.*` is a different control (MAC-T01..T06†).
- **Parts:** `window` (fill, stroke, shadow, corners), `text` (font and colour), margins are `window.padding`.
- **Widget values:** placement: `PlacementStrategy::Keyed(appkit_context_help)` (against the given point,
  ADD-A21); `mode_cursor: Value<CursorShape>` (the pointer while context-help mode is active).
- **Variants/sizes/flags:** none; **may_be_absent:** `mode_cursor` (to verify, row).
- **State-model source:** AppKit appearances (light, dark); no control states.
- **Today's fields:** `tooltip.*` (nearest; not the same window).

### Windows

#### component.semantic_zoom (windows)

*From group 7.*

- **Kind:** keyed component (WinUI `SemanticZoom`). Row W2-34. Cross-check: all other platforms 3, iOS 2 maps
  the word "zoom" to the window zoom button (another row).
- **Parts:** `zoom_out_button` (element; fill, stroke, glyph, font size; states PointerOver, Pressed,
  Disabled by WinUI visual states), `zoom_out_glyph` (glyph).
- **Widget values:** presentation transition for event `zoom` (zoom in/out between views; kind from the
  template's animation, `Keyed` if it is not one of the open list).
- **Variants/sizes/flags:** none. **Exclusive groups:** CommonStates (disabled, pressed, hover).
- **may_be_absent:** none.
- **State-model source:** WinUI visual state groups by template (`dxaml generic.xaml`).
- **Today's fields:** none.

#### component.key_tip (windows)

*From group 7.*

- **Kind:** keyed component (WinUI `KeyTip`, access-key badge). Row W2-37. Cross-check: GNOME, KDE, macOS,
  iOS, Material 3 (all underline mnemonics or draw none).
- **Parts:** `badge` (fill, stroke, corners, padding, text style, foreground), `text`.
- **Widget values:** placement: `PlacementStrategy::Keyed(winui_key_tip)` with the placement offset as a
  parameter (ADD-A21).
- **Variants/sizes/flags:** none. High contrast and theme dictionaries are rules under the conditions.
- **Appear/Disappear:** presentation transitions are not stated by W2-37; none.
- **may_be_absent:** none.
- **State-model source:** WinUI theme dictionaries per theme and high-contrast (`generic.xaml:1010-1012`).
- **Today's fields:** `tooltip.*` (nearest, not the same).

#### component.tabbed_command_bar (windows, Windows Community Toolkit)

*From group 7.*

- **Kind:** keyed component (CTK `TabbedCommandBar`; R1.2 in). Row W-RV-3†. Other platforms: no ribbon (all
  3 or absent).
- **Parts:** `tab_row` (a NavigationView in Top mode: an element of the typed top navigation, whose look is
  W2-11), `content_border` (fill, stroke, corners, thickness; high-contrast variant), `command_bar` (an
  element of the typed `toolbar`, with `label_position`), `element_container` (margin), `contextual_tab`
  (text colour in accent; element; states Rest, PointerOver, Pressed, Disabled, Selected, SelectedPointerOver,
  SelectedPressed).
- **Widget values:** presentation transition for event `tab_change` (PopIn on the content), `label_position:
  plain enum Right`.
- **Variants:** keyed `@contextual` (contextual tab). **Sizes/flags:** none. **Exclusive groups:** WinUI's
  CommonStates and SelectionStates by template.
- **may_be_absent:** none.
- **State-model source:** WinUI visual states by template (`TabbedCommandBarItem.xaml:9-24`, `:83-89`).
- **Today's fields:** `toolbar.*`, `tab.*` (neither has a tab row joined to a command panel).

#### component.casting_picker (windows)

*From group 7.*

- **Kind:** keyed component (`CastingDevicePicker`; the application sets its `Appearance`, the system draws the
  list: R1.1 "configures for the system to draw"). Row W-P4-18†. Its cast button is the typed
  `media_controls` part `cast_button`. macOS' route picker (MAC-P3-9†) is the same job on another
  platform, with a different object, so each stays keyed.
- **Parts:** `picker` (flyout surface), `title`, `item` (element; states hover, `Selected`), `selected_item`
  is `item` under {selected}.
- **Widget values:** placement `PlacementStrategy::WinUiFlyout(..)` (`Show(Rect, Placement)`); the six
  appearance colours are bases of the parts (accent, background, foreground and their selected twins).
- **Variants/sizes/flags:** none. **may_be_absent:** `title`.
- **Reads:** `reader:windows:DevicePickerAppearance getters` (R, to verify whether they return the system
  defaults).
- **State-model source:** WinUI visual states; the `Appearance` object.
- **Today's fields:** none.

### iOS

#### component.context_menu_preview (ios)

*From group 7.*

- **Kind:** keyed component (UIKit `UIContextMenuInteraction` preview). Row C-13. Cross-check: macOS 3,
  Windows 3, KDE 3, GNOME 3, Material 3.
- **Parts:** `preview` (the lifted view: `transform` scale, `corners` clipping path, `shadow`), `dim` (the
  blur and dim over everything else: a `Material` paint of the platform plus a tint layer).
- **Widget values:** presentation transitions: events `lift` and `dismiss` (keyed events; kinds `Scale`,
  `Fade`; timing a `Spring`); placement `PlacementStrategy::Keyed(uikit_context_menu)` (the menu next to the
  preview, ADD-A21).
- **Variants:** none. **Sizes:** none. **Flags:** none. **Conditions:** `ReducedTransparency` (material to
  opaque) and `ReducedMotion` as rules.
- **may_be_absent:** none (26 to verify per row).
- **State-model source:** UIKit appearances and the interaction's preview parameters.
- **Today's fields:** none. Reader: `readers (R18)`.

#### component.bottom_accessory (ios, 26 only)

*From group 7.*

- **Kind:** keyed component (`UITabAccessory`, iOS 26). Row C-21. Cross-check: GNOME 1 (AdwBottomSheet's bottom
  bar is a bottom-sheet part, another row), macOS 3, others 3.
- **Parts:** `accessory` (the capsule: a `Material` paint, `Corners::Full`, shadow), `content` (the app's view
  area, padding).
- **Widget values:** `height: Value<f32>`, insets as `accessory.padding`; presentation transition for event
  `environment_change` (kind `Morph`).
- **Variants:** keyed `@inline`, `@expanded` (`UITabAccessory.Environment`, to confirm in the value phase).
  **Sizes/flags:** none.
- **may_be_absent:** the component is not declared by `ios-13`..`ios-18`.
- **State-model source:** UIKit trait `tabAccessoryEnvironment`.
- **Today's fields:** none. Reader: `readers (R18)`.

#### component.keyboard (ios)

*From group 7.*

- **Kind:** keyed component. The keyboard itself is drawn only by the system (out, R1.1); the application's
  part is what it configures: the appearance and the input accessory it draws. Row C-23.
- **Parts:** `input_accessory` (the keyboard toolbar: fill or material; padding), nothing for the keys.
- **Widget values:** `appearance: plain enum { Default, Light, Dark, FollowsScheme }`
  (`UIKeyboardAppearance`, set by the application so that the keyboard matches the theme); presentation
  transitions for events `show` and `hide` (kind `Slide`; duration and timing are the keyboard
  notification's, read, and not the preset's).
- **Variants/sizes/flags:** none. **may_be_absent:** none.
- **Reads:** keyboard-notification duration and curve: `readers (R18)`.
- **State-model source:** UIKit appearances.
- **Today's fields:** none.

#### component.document_launch_view (ios)

*From group 7.*

- **Kind:** keyed component (`UIDocumentViewController.LaunchOptions`, iOS 18+). Row C-44†. Cross-check:
  GNOME (status page), Windows, KDE 3.
- **Parts:** `background`, `title`, `subtitle`, `primary_action`, `secondary_action` (elements of the typed
  `button`, with variants), `foreground_accessory`, `background_accessory`, `browser` (the document browser
  frame it opens).
- **Widget values:** none stated. **Variants/sizes/flags:** none. **may_be_absent:** the accessories, the browser.
- **State-model source:** UIKit configuration objects and control states; 26 to verify (row).
- **Today's fields:** none. Reader: `readers (R18)`.

#### component.activity_rings (ios)

*From group 7.*

- **Kind:** keyed component (HealthKitUI `HKActivityRingView`). Row C-46†.
- **Parts:** `ring` (element-free; stroked arc; keyed identity by variant), `track` (the background ring).
- **Widget values:** `ring_gap: Value<f32>`; the ring width is `ring.stroke` width; `ring_progress` is data,
  not theme.
- **Variants:** keyed `@move`, `@exercise`, `@stand` (three rings with an Apple Watch paired, the Move ring
  only without one: the data decides which are drawn, so none is `may_be_absent`).
- **Paint:** ring colours may be gradients (C27; row says "gradient or flat: M-r"): see ADD-A34, ADD-B02.
- **State-model source:** the view's own configuration (HealthKitUI); light and dark appearances.
- **Today's fields:** none. Reader: `readers (R18)`.

### Material

*From group 6:*

| Component | Platform | Parts | Values | Rows |
|---|---|---|---|---|
| `fab_menu` | Material | `close_button`, `item`, `item_label`, `item_icon` | `item_gap`, `@open`/`@close` morph | MAT-N3 |

## Colour roles and text ramp

*From group 5's catalogue.*

### Policy: typed set, typed global, or keyed

1. A **typed surface set** exists when a platform states a background and a foreground for a region that parts bind
   (`Binding::Surface`, S3.3): KDE's seven sets (Q3.1), libadwaita's window, view, header bar, sidebar, secondary
   sidebar, card, dialog, popover and OSD. A set is declared by the preset (ADD-A48); `Window` always.
2. A **global role** is one the platform states once for the whole theme (accent, status, scrim, highlights).
3. A **keyed role** is every other colour the platform names: a ladder (Material's, WinUI's tiers, Apple's label, fill and
   background hierarchies, the system palette). Keyed roles are declared in `[declare] colour_roles` and referenced as
   `role.@key`.
4. A role never repeats another: where a platform has no separate value, the preset states an expression
   (`= role.window.foreground`), so a toolkit follows the platform and the gate checks the reference (R20).

#### surface sets

`SurfaceSet` is closed to these thirteen variants (S3 listed twelve; `SecondarySidebar` is added [GN-1.137]):

| Set | What it is | Declared by (sources of the list) |
|---|---|---|
| `Window` | the root surface; mandatory; what `surface.<role>` falls back to (S3.3) | every preset |
| `View` | content the user edits or reads: lists, text views, entries | KDE View (Q3.1, Q3.2); libadwaita `view_*` [GN-1.274]; the content/background hierarchy elsewhere |
| `Button` | KDE's Button set: push buttons, check and switch indicators, sliders | KDE only as a set (Q3.1, Q3.2); other platforms state button fills per part |
| `Header` | header bars, menu bars, page headers, column/drawer separators | KDE Header (Q3.1, Q3.2); libadwaita `headerbar_*` [GN-1.15] |
| `Sidebar` | sidebars, source lists | libadwaita `sidebar_*` [GN-1.138]; other platforms when they state one |
| `SecondarySidebar` | libadwaita's second sidebar | libadwaita `secondary_sidebar_*` [GN-1.137] |
| `Card` | cards, boxed lists | libadwaita `card_*` [GN-1.255]; WinUI card fills when stated as a set |
| `Popover` | popovers, menus, flyouts | libadwaita `popover_*` [GN-1.182] |
| `Dialog` | dialogs and sheets | libadwaita `dialog_*`; iOS elevated sheet via binding (R5.2) |
| `Tooltip` | tooltips | KDE Tooltip (Q3.1); Kirigami swaps it for `Complementary` (Q1.37) |
| `Selection` | the selected item: list/menu selection, KDE Selection set | KDE Selection (Q3.1); every platform's list selection |
| `Complementary` | KDE's inverse-scheme set: passive notifications, tooltips in Kirigami | KDE only [K3.54, Q3.1] |
| `Osd` | libadwaita `.osd`: toasts, OSD toolbars and their accent overrides | libadwaita [GN-3.09, GN-3.17] |

Thumbnail and overview colours of libadwaita (`--thumbnail-*`, `--overview-*`) are a background and a foreground only
and no part binds them as a surface: they are keyed roles (below), not sets [GN-1.276].

A preset declares the sets it fills in `[declare] surface_sets = [...]` (ADD-A48). A binding to, or a
reference through, an undeclared set is a resolution error (S12). Per platform:

| Platform | Sets declared (from the rows named) |
|---|---|
| `kde-breeze` | Window, View, Button, Selection, Tooltip, Complementary, Header (all seven, each with Active, Inactive and Disabled groups) [K3.54, Q3.1] |
| `adwaita` | Window, View, Header, Sidebar, SecondarySidebar, Card, Dialog, Popover, Osd (+ Selection if the value phase finds a stated selection pair) [libadwaita `_colors.scss`, GN-3.09] |
| `windows-11`, `macos-*`, `ios-*`, `material-3` | Window and every set whose background and foreground the platform's facts state for a region; the value phase declares them from the facts. Ladders go to keyed roles (below), they are not sets |

#### role record

`SurfaceRoles` is the record every declared set states. Fields (the S3 list, closed here):

| Field | Meaning | Required | Reasons |
|---|---|---|---|
| `background` | the set's fill | required | all |
| `background_alternate` | alternate row/band fill (KDE `BackgroundAlternate`); alternating rows only where a list opts in | may_be_absent | Q3.13 |
| `foreground` | primary text and glyph colour | required | all |
| `foreground_inactive` | the platform's secondary (muted) text level | required | KDE `ForegroundInactive`; today's `muted_color` |
| `foreground_active` | KDE `ForegroundActive`, used as the information colour | may_be_absent | K3.54, Q3.5 |
| `link`, `link_visited` | link text, visited link text | `link` required; `link_visited` may_be_absent | K3.54, GN-1.265 |
| `negative`, `neutral`, `positive` | the set's status text colours (KDE `ForegroundNegative` etc.); a platform without per-set values states `= role.status.<s>.standalone` | required | K3.54, Q3.10 |
| `decoration_focus`, `decoration_hover` | KDE's focus and hover decoration colours of the set | may_be_absent | Q3.11, Q3.12 |
| `border` | the set's stroke colour (translucent where the platform's is) | required | all; absorbs today's `border.color` and `border.opacity` as `alpha(...)` |
| `shade` | the translucent shade line/overlay of the set (libadwaita `*_shade_color`) | may_be_absent | GN-1.270 |
| `disabled` | `DisabledRoles` | may_be_absent as a whole | R5.1 |

Additions to the S3 sketch (ADD-A48): `background_active`, `background_link`, `background_visited`,
`background_negative`, `background_neutral`, `background_positive` (KDE's six computed status backgrounds, tinted from the
set's foreground roles) [K3.52]; `shade_darker` (libadwaita's `headerbar_darker_shade_color`) [GN-1.15]. All
may_be_absent.

`DisabledRoles` has the same colour fields as `SurfaceRoles` except `disabled`; every field may_be_absent. On KDE each is
the KColorScheme disabled effect applied to the active value (`kde_intensity`, `kde_color`, `kde_contrast` with
`token.colour_effects`) [K3.50]; on a platform that dims by opacity (libadwaita) the record stays absent and the parts'
`{disabled}` rules set `opacity` from `token.params.opacity` instead [GN-1.271, GN-1.272].

The inactive group is no record field: it is a colour-role rule under `WindowInactive` on the active field (libadwaita's
`*_backdrop_color`, KDE's `[Colors:*][Inactive]` inputs and effects, AppKit's unemphasised colours) [K3.51, GN-1.136,
S3.2].

#### global roles

`GlobalRoles` (S3 list, closed here):

| Field | Meaning | Reasons |
|---|---|---|
| `accent` | the accent fill (libadwaita `accent_bg_color`, today's `accent_color`) | GN-3.14 |
| `accent_foreground` | text/glyph on the accent (today's `accent_text_color`) | GN-3.14 |
| `accent_standalone` | accent used as text or icon colour on the surface, distinct from the fill (libadwaita `accent_color`, OKLab-derived: `adw_standalone`); elsewhere `= role.global.accent` | GN-1.268, GN-3.13 |
| `accent_ramp` | keyed: WinUI Light1-3, Dark1-3 | W3-12 (other group) |
| `status` | `StatusRoles`: `success`, `warning`, `error`, `info`, `destructive`, each a `StatusRole { fill, foreground, standalone }`; `destructive` and `error` are separate entries [GN-1.269]; `neutral` and the two background fields are added (ADD-A48) | GN-1.269, W3-17/18, K3.52 |
| `shadow_reference` | what the platform names as the shadow colour (today's `shadow_color`; macOS `shadowColor`, Material `shadow`) | today's `shadow_color`, MAT-X5 |
| `selection_text` | the highlight behind selected text (today's `text_selection_background`) | today's fields |
| `selection_text_foreground` | the selected text's colour (today's `text_selection_color`); added (ADD-A48) | today's fields |
| `find_highlight` | the find/search match highlight | MAC-DF04 |
| `drop_highlight` | the drop-target highlight | R12 |
| `scrim` | modal backdrop colour (with its alpha) | MAT-X23, W3-11, W1-125 |

`selection_text_inactive` of the S3 sketch is **removed** (ADD-A48): the inactive text-selection look is a rule under
`WindowInactive` on `selection_text` (S3.2), like every inactive group.

#### keyed roles per platform

Keyed roles are declared in `[declare] colour_roles`; the names are snake_case of the platform's own. Families, with the
rows that need them (the member lists are the facts', the value phase states them):

**GNOME (`adwaita`).** `thumbnail_background`, `thumbnail_foreground`, `overview_background`, `overview_foreground`
[GN-1.276]. (Header bar, sidebar, card and popover shades are record fields [GN-1.270]; the window shade `--shade-color`
is `role.window.shade`.)

**KDE (`kde-breeze`).** `shade_light`, `shade_midlight`, `shade_mid`, `shade_dark`, `shade_shadow`: Qt's 3-D frame and
grid roles, computed from the Window background by `kcu_shade` with `token.colour_effects.contrast` [K3.53]. They are
keyed, not per set: KDE computes them from Window.

**macOS (`macos-*`).** Label hierarchy beyond the record: `label_tertiary`, `label_quaternary`, `label_quinary`
(14+) [MAC-DF01]; the system fill ramp `system_fill`, `system_fill_secondary`, `system_fill_tertiary`,
`system_fill_quaternary`, `system_fill_quinary` [MAC-DF02]; the system palette beyond red, orange, green and blue:
`system_yellow`, `system_purple`, `system_pink`, `system_teal`, `system_indigo`, `system_cyan`, `system_mint`,
`system_brown`, `system_gray` (and the four already named, stated as `system_red`, `system_orange`, `system_green`,
`system_blue`) [MAC-DF03]; `under_page_background`, `text_background` (against `View.background` =
`controlBackgroundColor`), `selected_control`, `selected_control_secondary`, `selected_control_text` [MAC-DF04,
MAC-DF05]; `highlight` (the separate highlight setting, an input, S9.1) [MAC-DF05]. The accent-derived colours are inputs
(R2.6).

**iOS (`ios-*`).** Label hierarchy: `label_tertiary`, `label_quaternary` (tertiary no longer conflated with disabled
text: `window.disabled.foreground` states disabled separately) [D-9, X-CO-5]; fills `system_fill`, `system_fill_secondary`,
`system_fill_tertiary`, `system_fill_quaternary` [D-10]; backgrounds `system_background`, `system_background_secondary`,
`system_background_tertiary`, `system_grouped_background`, `system_grouped_background_secondary`,
`system_grouped_background_tertiary` [D-11], and the elevated variant of each (`..._elevated`), which a window-wide
`Elevated` rule re-points the base roles to and which a sheet or popover binds locally (R5.2) [D-14]; `separator_opaque`
(the translucent separator is `border`) [D-12]; `placeholder` [X-CO-2]; the system palette `system_red`, `system_orange`,
`system_yellow`, `system_green`, `system_mint`, `system_teal`, `system_cyan`, `system_blue`, `system_indigo`,
`system_purple`, `system_pink`, `system_brown`, `system_gray`, `system_gray2` .. `system_gray6` [D-13]. Tint-dependent
roles are inputs (R2.6); increased-contrast variants are colour-role rules under `ContrastIncreased` [D-15].

**Windows (`windows-11`).** The WinUI tiers, one keyed role per brush of `Common_themeresources_any.xaml`, in the
families the rows name: text tiers (`text_tertiary`, `text_inverse`, text on accent: secondary, disabled, selected) and accent
text/fill tiers [W3-12, W3-13, W3-14, W3-46]; stroke tiers (control, strong, card, surface, divider, on-accent) [W3-15];
fill tiers (control, strong, subtle, control-alt, on-image, solid background base..senary, base-alt) [W3-16]; status
backgrounds and neutral [W3-17, W3-18]; the eight high-contrast system colours [W3-35]. These rows belong to the
crosscutting group; this file only fixes that they are keyed.

**Material (`material-3`).** All fifty `md.sys.color` roles of `latest/_md-sys-color.scss:20-206`, in snake_case:
primary, secondary, tertiary and error with their `on_`, `_container` and `on_..._container` forms; the fixed family
(`primary_fixed`, `primary_fixed_dim`, `on_primary_fixed`, `on_primary_fixed_variant`, and the same for secondary and
tertiary); `background`, `on_background`, `surface`, `on_surface`, `surface_variant`, `on_surface_variant`, `surface_dim`,
`surface_bright`, `surface_container_lowest`, `_low`, `surface_container`, `_high`, `_highest`; `outline`,
`outline_variant`; `inverse_surface`, `inverse_on_surface`, `inverse_primary`; `surface_tint`; `shadow`, `scrim`
[MAT-X1]. The typed record's fields and the global roles reference them (`accent = role.@primary`; the scheme function
`m3_role` supplies them from the seed input, R2.6).

### Text ramp

#### text roles

`TextRole` is closed to nineteen slots (S4.4 listed eighteen; `Caption2` is added so that Apple's smallest style, Material's
`label-small` and the emphasised captions have a slot). A slot is filled by a **reference to a keyed style** (ADD-A42): the
preset states the keyed style's name, so "the facts say which of its styles fills it" is literal and a style is never
stated twice. Where a platform has fewer styles than slots, neighbouring slots **share** a style (marked "=" below);
the share is a stated fact, never a toolkit default.

| Slot | Use | Apple (macOS/iOS) | Windows | GNOME | KDE | Material |
|---|---|---|---|---|---|---|
| `DisplayLarge` | the largest hero text | `large_title` (iOS: `extra_large_title` where iOS draws it, TS-4) | `display` | `title_1` | `heading_1` | `display_large` |
| `DisplayMedium` | hero text | `large_title` = | `display` = | `title_1` = | `heading_1` = | `display_medium` |
| `DisplaySmall` | hero text, today's `text_scale.display` | `large_title` = | `display` = | `title_1` = | `heading_1` = | `display_small` |
| `LargeTitle` | page/screen title | `large_title` | `title_large` | `title_1` | `heading_1` = | `headline_large` |
| `Title1` | dialog/page title, today's `dialog_title` | `title_1` | `title` | `title_2` | `heading_1` | `headline_small` |
| `Title2` | sheet/section title | `title_2` | `subtitle` | `title_3` | `heading_2` | `title_large` |
| `Title3` | sub-section title | `title_3` | `subtitle` = | `title_4` | `heading_3` | `title_large` = |
| `Headline` | section heading, today's `section_heading` | `headline` | `subtitle` = | `heading` | `heading_3` = | `title_medium` |
| `Subheadline` | secondary line under a heading | `subheadline` | `body` = | `body` = | `heading_4` | `title_small` |
| `Body` | running and UI text, today's `defaults.font` | `body` | `body` | `body` | the default font (`font`) | `body_medium` |
| `BodyEmphasized` | emphasised body | `body_emphasized` | `body_strong` | `heading` = | `heading_5_primary` | `body_medium_emphasized` (expressive); `title_small` (baseline) |
| `Callout` | body-sized text, slightly set apart | `callout` | `body_large` | `body` = | the default font = | `body_large` |
| `Footnote` | notes | `footnote` | `caption` = | `caption` = | `smallest_readable` = | `body_small` = |
| `Caption` | smallest readable, today's `text_scale.caption` | `caption_1` | `caption` | `caption` | `smallest_readable` | `body_small` |
| `CaptionEmphasized` | emphasised caption | `caption_1_emphasized` | `caption` = | `caption_heading` | `smallest_readable` = | `body_small_emphasized` (expressive); `label_medium` (baseline) |
| `Caption2` | the smallest style | `caption_2` | `caption` = | `caption` = | `smallest_readable` = | `label_small` |
| `Label` | control labels | `control_content` (macOS), `body` = (iOS) | `body` = | `body` = | the default font = | `label_large` |
| `Monospace` | monospace text, today's `mono_font` | the platform's monospace font as the facts state it (macOS `monospacedSystemFont`) | as today's `mono_font` is sourced | `monospace` (the `monospace-font-name` setting) | `[General] fixed` | as today's `mono_font` is sourced |
| `Numeric` | tabular figures | `body` + monospaced digits (D-8) | `body` = | `numeric` | the default font = | `body_medium` = |

The assignment of KDE's levels follows Kirigami's own use (Q1.5: dialog and page title level 1, sheet header level 2, form
section level 3, drawer section level 4, list section header level 5 bold). The Material "expressive/baseline"
alternatives are a `Flavour` rule on the token (S7.1); the emphasised set exists only in Expressive [MAT-TS4]. Every
cell above is a proposal bound by the facts: the value phase may change a style name only with a cited reason.

#### keyed text styles per platform

A keyed text style is a `TextStyle` (S2.8) the preset declares in `[declare] text_styles`. Each platform lists **all** of
its own named styles, so nothing the platform draws is lost whether or not a slot references it.

**Apple.** The eleven text styles, snake_case: `large_title`, `title_1`, `title_2`, `title_3`, `headline`,
`subheadline`, `body`, `callout`, `footnote`, `caption_1`, `caption_2` [MAC-TS01, TS-1]; for each, the emphasised-weight
form `<style>_emphasized` (the HIG's "emphasized" column; Bold Text uses it) [D-7, MAC-TS01]; for each, the tight and loose
leading forms `<style>_tight`, `<style>_loose` (font-descriptor traits; they differ in `line_height` only) [D-6,
MAC-TS03]; iOS adds `extra_large_title`, `extra_large_title_2` (iOS 17) [TS-4]; the named AppKit fonts `label`, `message`,
`palette`, `menu_bar`, `control_content` and `small_system` [MAC-TS02]. Per style: `letter_spacing` is
`sf_tracking(font_size, design)` (ADD-A54) [D-1]; `font.axes` carries `opsz` as `FollowsSize` [D-2]; `font.width` and the
family carry the system design and width [D-3]; `font.features` carries monospaced digits where a style uses them [D-8];
`line_height` is each style's own [D-4]. The Dynamic Type tables (size per content-size category, non-linear) are read by
the iOS reader, the preset states the default category [D-5, TS-2, R10]; Bold Text is the `BoldText` condition (a token
rule on `font.weight`) [TS-3].

**Windows.** The nine Fluent styles: `caption`, `body`, `body_strong`, `body_large`, `body_large_strong`, `subtitle`,
`title`, `title_large`, `display` [W1-147].

**GNOME.** libadwaita's classes: `title_1`, `title_2`, `title_3`, `title_4`, `heading`, `body`, `caption_heading`,
`caption`, `document`, `monospace`, `numeric` [GN-1.189]. `document` takes its family and size from
`document-font-name` [GN-1.191]; `numeric` carries the `tnum` feature [GN-1.192]; `font.axes` and `font.features` receive
the Pango `@axis=` and `#feature=` segments of the font settings [GN-1.193]; `line_height` of `body`, `caption` and
`document` is a multiple of the font size [GN-1.190]. The relative label sizes the stylesheet states are keyed styles
whose size is `mul(token.text.roles.Body.font.size, factor)`: `smaller` (subtitles, progress text), `narrow_switcher_label`,
`badge_label`, `tab_counter` [GN-1.194].

**KDE.** Kirigami `Heading` levels `heading_1` .. `heading_5` as factors of the default font size (KTitleWidget's levels 1-4
state the same factors, so one style serves both and its fact cites both sources) [Q1.3, K1.270]; the types per level:
`heading_<n>_primary` (DemiBold weight) and `heading_<n>_secondary` (text fill at reduced alpha) [Q1.4];
`smallest_readable`, a full font (family, style, size) read from `smallestReadableFont`, also Kirigami's `smallFont`
[K1.271, Q1.2]; the Breeze-derived fonts `flat_group_title` (bold, one point larger than the group box font),
`command_link_title` (bold), `menu_section` (bold) [K1.272], whose size uses `add` (ADD-A56). The default, fixed, menu and tool-bar
fonts are the Body and Monospace slots and the widgets' parts.

**Material.** The fifteen baseline styles, `display_large` .. `label_small` (five groups of large, medium, small: display,
headline, title, body, label) [MAT-TS1], and the fifteen Expressive emphasised styles `<style>_emphasized` [MAT-TS4].
Per style: `letter_spacing` [MAT-TS2]; `font.family` the brand or plain typeface, with its weight name [MAT-TS3];
`font.axes` for `crsv`, `fill`, `grad`, `hexp`, `opsz`, `rond`, `slnt`, `wdth`, `wght` [MAT-TS5].

#### tokens carried by these rows

- **Opacities.** `token.params.opacity: Vec<(Key, Value<f32>)>` (ADD-A44): `disabled` (today's `disabled_opacity`), `strong_disabled`,
  `dim`, `dimmer`. libadwaita's high-contrast values are Tokens rules under `ContrastHigh` [GN-1.271, GN-1.273]. Parts
  refer to `token.params.opacity.<key>` in `opacity` or in `alpha(...)`.
- **Icon sizes.** `IconSizeTokens` (ADD-A43): the five typed sizes of today (`toolbar`, `small`, `large`, `dialog`, `panel`, each
  `Value<f32>`, may_be_absent) and `keyed: Vec<(Key, IconStyle)>`, so a context also carries weight, scale and rendering mode
  [D-19, MAC-IC02]. Kirigami: `small`, `small_medium`, `medium`, `large`, `huge`, `enormous`, `for_labels`
  (`kirigami_label_icon_size(line_height)`, ADD-A53) [Q1.1]. macOS: `button`, `list_row`, `tab`, `toolbar` (a rule per toolbar
  size mode), `status_item`, `sidebar` (rules under `ContainerSize { sidebar, small|medium|large }`) [MAC-IC01]. The
  alert icon size is a measured value of `dialog` [MAC-IC04].
- **Focus.** iOS's keyboard halo (shape follows the view's rounded rect and curve; position inside, outside or automatic) is
  `tokens.focus` with `corners.curve = Continuous` and `offset` [D-18].

#### today's fields

`[defaults]` (docs/property-registry.toml:96-147) and `[text_scale]` (:147-151):

| Today's field | New home |
|---|---|
| `defaults.font` (family, size, weight, style, color) | the `Body` slot's keyed style: `font.family`, `font.size`, `font.weight`, `font.style`; `color` -> `TextStyle.fill` = `= surface.foreground` |
| `defaults.line_height` | each style's own `line_height` (an expression on `font_size`); no global ratio (R20) |
| `defaults.mono_font` | the `Monospace` slot |
| `defaults.background_color` | `role.window.background` |
| `defaults.text_color` | `role.window.foreground` |
| `defaults.accent_color` | `role.global.accent` (+ `accent_standalone` where the platform has one) |
| `defaults.accent_text_color` | `role.global.accent_foreground` |
| `defaults.surface_color` | `role.view.background` (the content surface; Card/Dialog etc. when the platform states them) |
| `defaults.muted_color` | `role.window.foreground_inactive` |
| `defaults.shadow_color` | `role.global.shadow_reference`; libadwaita's shade colours -> `shade` of each set |
| `defaults.link_color` | `<set>.link` |
| `defaults.selection_background`, `selection_text_color` | `role.selection.background`, `role.selection.foreground` |
| `defaults.selection_inactive_background` | `role.selection.background` under a `WindowInactive` colour-role rule |
| `defaults.text_selection_background`, `text_selection_color` | `role.global.selection_text`, `selection_text_foreground` |
| `defaults.disabled_text_color` | `role.window.disabled.foreground` (and the other sets') |
| `defaults.danger_color`, `danger_text_color` | `role.global.status.error.fill`, `.foreground` (and `destructive` where separate) |
| `defaults.warning_*`, `success_*`, `info_*` | `role.global.status.<s>.fill`, `.foreground` |
| `defaults.focus_ring_color`, `focus_ring_width_px`, `focus_ring_offset_px` | `tokens.focus` (`paint`, `width`, `offset`) |
| `defaults.border.color`, `border.opacity` | `<set>.border`, the opacity folded in as `alpha(...)` |
| `defaults.border.line_width_px`, `corner_radius_px`, `corner_radius_lg_px`, `shadow_enabled`, padding fields | no global: each part states its own `stroke`, `corners`, `shadow`, `padding` (R20, no model inheritance); the preset repeats the number per part |
| `defaults.disabled_opacity` | `token.params.opacity.disabled` |
| `defaults.icon_sizes.{toolbar,small,large,dialog,panel}_px` | `tokens.icon_sizes.{toolbar,small,large,dialog,panel}` |
| `defaults.icon_theme` | unchanged: a `Theme`-level field (outside G5) |
| `text_scale.caption` | `Caption` slot (-> keyed style) |
| `text_scale.section_heading` | `Headline` slot |
| `text_scale.dialog_title` | `Title1` slot |
| `text_scale.display` | `DisplaySmall` slot (and `LargeTitle`, which most platforms share with it) |
| `TextScaleEntry.size_pt`/`size_px`, `weight` | the keyed style's `font.size` (`FontSize`), `font.weight` |
| `TextScaleEntry.line_height_pt`/`line_height_px` | the keyed style's `line_height` (ADD-A40) |
| accessibility flags (§2.1.7) | conditions (S7.1), not G5 |

## Tokens, conditions, readers, placement

*From group 8's catalogue.*

### tokens

Token keys are the platform's own names, declared by the preset (S10). A key marked "(none)" below means the
platform has no such ramp and parts state the value themselves.

**token.motion.durations** (`Vec<(Key, Value<f32>)>`, S4.1). A scaled duration is `= mul(token.motion.durations.x,
token.motion.speed_factor)`, an unscaled one a plain duration (R9).

| Platform | Keys (one per duration the platform states) | Rows |
|---|---|---|
| libadwaita | the CSS transition tokens; the timed animations defined in code; the revealer and stack defaults; the animated-scroll duration | GN-3.18, GN-3.21, GN-3.22, GN-3.24 |
| kde-breeze | `breeze_state` (the Breeze state animation: factor times a base while the factor is set, breezerc `AnimationsDuration` otherwise); `qt_fixed` (Qt's own unscaled timings: side-icon fade, tab move); `kf_timeline` (KMessageWidget, KCollapsibleGroupBox); Kirigami `very_short`, `short`, `long`, `very_long` (derived from `long`, floor of one millisecond by `max`, ADD-A56, ADD-B11) and `human_moment` (unscaled) | K3.31, K3.37, K3.38†, Q3.23–Q3.25, Q3.27 |
| windows-11 | `normal`, `fast`, `faster`; `connected` (connected animation); one entry per built-in theme transition the facts measure (entrance, content, add and delete, reorder, reposition, popup, pane, drill in, press tilt, fade, split open and close, drag item) | W3-30, W3-32, W3-33, W3-34 |
| macos-* | the AppKit durations (`NSAnimationContext` default, window resize time, per-control durations the probe records), `navigation`; | MAC-X11 |
| ios-* | `navigation` (push and pop), `modal` styles, the symbol-effect durations | X-MO-1, X-MO-3, X-MO-4, X-MO-9 |
| material-3 | the motion-scheme duration tokens (short, medium, long, extra long families), the ripple's, the shadow transition's | MAT-X13, MAT-X9, MAT-X7 |

**token.motion.timings** (`Vec<(Key, Timing)>`): cubic Béziers, springs (`Timing::Spring`), `Timing::Keyed` where a
platform's curve is its own (Kirigami's velocity-based curves). Keys per platform: libadwaita's easing curves and
default timed-animation easing and spring parameters per animation (GN-3.19, GN-3.20); Kirigami's curves in use
(Q3.26); Qt's `InOutSine` default (K3.37); WinUI's entering, exiting and control key spline (W3-31); UIKit's default
spring (X-MO-2) and AppKit's media-timing curves (MAC-X11); Material's easing and spring tokens (MAT-X13).

**token.motion.speed_factor** (`Value<f32>`): read on KDE (K3.30, K3.90, Q3.24); stated 1 elsewhere. Rules set it
to zero under `AnimationsOff` where the platform's switch does that (GN-3.25, Q3.25, W3-37, W3-52†, MAT-X20).

**token.theme_change** (S4.3), one kind per preset, measured where no source states it: libadwaita `Suspend`
(GN-3.23); `kde-breeze` (K-P3-15†), `windows-11` (W-P3-16†, which also covers the accent change), `macos-*`
(MAC-P4-9†, accent and highlight change), `ios-*` (X-MO-11†), `material-3` (MAT-P3-18†).

**token.elevation** (`Vec<(Key, Vec<ShadowLayer>)>`, S4/R8): the ramp's keys, referred to by `Shadow::Elevation(key)`.

| Platform | Keys | Rows |
|---|---|---|
| windows-11 | the elevation steps WinUI names (1 to 128), each its ambient layer, the dark scheme's intensity as a colour-role-free rule under `scheme:dark` | W3-01, W3-02, W3-03 |
| material-3 | levels 0 to 5, each a key and an ambient layer (layers per level), per-state level by rules | MAT-X2, MAT-X3, MAT-X4, MAT-X5 |
| kde-breeze | the window shadow size steps and the popup shadow size steps (two tables, two layers per step), with `token.params.shadow.strength` and the shadow colour read from breezerc | K3.1, K3.2 |
| Kirigami (flavour `kirigami`) | one entry per Kirigami and qqc2 surface | Q3.21 |
| libadwaita, macos-*, ios-* | (none): layers are stated per part (`PartStyle.shadow.layers`) | GN-3.01, MAC-X01, X-SH-1 |

**token.params** (ADD-A44), keyed scalar tokens in groups, each a number, length, duration or opacity as its name says:
`shadow.strength` (K3.1, K3.2); `motion.animation_steps` (K3.31); `colour.contrast` and `colour.frame_contrast`
(K3.53, K3.91, Q3.14, Q3.15); `layout.grid_unit` (Q3.29); `layout.optical_centering` (MAT-X37†);
`caret.aspect_ratio` (GN-3.36); `shape.<step>` for Material's shape scale (MAT-X11).

**token.colour_effects** (`Vec<(Key, Value<f32>)>`, S4): groups `disabled`, `inactive` and `icon`, one key per
KColorScheme effect parameter: intensity effect and amount, colour effect, amount and colour, contrast effect and
amount (K3.50, K3.51), the inactive group's `ChangeSelectionColor` and `Enable` (K3.40), the icon effects' gray,
transparency and gamma parameters (K3.60, Q3.18). A palette from a KDE port states these itself (R19).

**token.behaviour** (S4.2): the existing fields (double-click time and distance, drag threshold, hover-open delay,
key repeat delay and interval, scroll lines per step, hide pointer while typing, scroll physics) carry these rows:
GN-3.47, GN-3.48, GN-3.67†, GN-3.68†, K3.70–K3.72, K-P3-16†, W3-41, MAC-X12, MAC-P3-12†, MAC-P3-17†, MAT-P3-14†,
MAT-P3-15†, MAT-X32†, P3-13†, GN-P3-15†. Scroll physics (`token.behaviour.scroll`) carries kinetic friction,
overshoot and its spring, animated-scroll duration and curve, snap settle, momentum: GN-3.24, GN-3.06, K-P3-18†,
MAC-P4-4†, MAC-X32†, W3-52†, X-MO-8, MAT-P3-16†. New fields: ADD-A45. `focus_visible` (when focus becomes visible, a
`FocusVisible` value per widget with this token as the default): ADD-A17.

**token.focus** (S4/R12): the platform's one indicator (width, offset, kind, corners, transition, secondary ring,
ADD-A17), by reference from every focusable part; parts that differ state their own (KDE's underline under tab
labels, K3.21; Material's inner offset, MAT-X10; WinUI's per-control margin, W3-20). Rows: GN-3.27–GN-3.29,
K3.20–K3.24, Q3.30, W3-19, W3-20, MAC-X13, X-FO-1, MAT-X10.

**token.caret** (S4.2): colour, width, shape, blink on and off times, blink timeout, blink style, secondary colour,
`hides_while_selecting`. "No blink" is `blink_on_ms` and `blink_off_ms` `Absent` (both `may_be_absent`). Rows:
GN-3.32–GN-3.38, GN-3.73†, K3.41–K3.43, W3-24, W-P3-9†, MAC-X12, MAC-X14, X-CA-1, X-CA-2, P3-9†, MAT-X25. The caret
is also a part of the text widgets (ADD-A24). Additions: ADD-A24.

**token.text** (S4.4): `roles` and `keyed` as S4.4 states, per platform the styles its facts list. Group 8 adds
keyed `taskbar` (K3.47), the per-role line height, letter spacing and optical-size axis (MAC-X18, W3-23, MAT-X16,
X-DT-5, GN-3.57's weight), the default size category's sizes on `ios-*` (X-DT-1), `monospace` with a source on
`windows-11` (W3-47†) and the platform default sans and monospace on `material-3` (MAT-X36†). `token.text.fallbacks`
(ADD-A41) holds the family fallbacks, emoji family and per-language fallback map (all FM rows); `token.text.rendering`
(ADD-B04) holds antialiasing, hinting, subpixel order and smoothing contrast.

**token.icon_sizes**: contexts beyond the five groups (GN-3.59, K3.63, W3-27), as keys of the same token; weight
and scale are per `IconStyle` (X-IC-2), not tokens.

**token.pointer** (ADD-A46): cursor theme, cursor size, pointer colours where read; `drag_cursors` keyed by drop
operation (copy, move, link, ask, none, not-allowed) for GN-P3-12†, K-P4-7†, W-P4-5†, MAT-P4-3†, P3-10†.

**token.size_classes**: no row of group 8 adds a threshold list. `ios-*` takes its class from the system (X-SV-1,
read by the connector at the root, R16); Material's width thresholds are stated by the widget groups' rows.

**token.material_scheme**: variant, specification version and the contrast level set by rules under the contrast
conditions (MAT-X18, MAT-P3-19†); with `input.seed` (MAT-X17).

**Shape and colour tokens by role:** the colour roles (S3) hold colours; group 8 adds to them in ADD-A48 and uses
`role.global.scrim` (GN-3.04, Q3.22, W3-11, X-SC-1, MAT-X23, MAT-X1), `role.global.drop_highlight` (GN-3.66†,
MAC-X21, W3-49†), `role.global.find_highlight` (MAC-X15), `role.@*` for WinUI's tiers (W3-13–W3-18, W3-46†),
Material's ladder (MAT-X1) and UIKit's label prominence levels (X-CO-2, X-CO-5).

### conditions

S7.1 already holds: `Scheme`, `ContrastIncreased`, `ContrastHigh`, `ReducedTransparency`, `ReducedMotion`,
`AnimationsOff`, `BoldText`, `ButtonShapes`, `DifferentiateWithoutColour`, `TabletMode`, `MobileFormFactor`,
`BatterySaver`, `ContainerSize`, `WindowInactive`, `NotMain`, `AppInactive`, `Maximized`, `FullScreen`, `Tiled`,
`Elevated`, `TransientTouch`, `Input(InputKind)`, `Flavour`, `Preset`. Group 8's rows use them as follows (reader
mapping per platform: macOS's and iOS's Increase Contrast is `ContrastIncreased`; Windows contrast themes and
GNOME's high contrast are `ContrastHigh`; Material's medium and high levels are `ContrastIncreased` and
`ContrastHigh`; KDE has no contrast condition, K3.91):

- `AnimationsOff` from a speed factor of zero on KDE (K3.30, K3.31); `ReducedMotion` as a degree on KDE is the factor
  (K3.90).
- `Input(Touch)` and `Input(Pen)` carry Material's pointer-precision sizing (MAT-X31) and WinUI's touch menu sizes
  (W3-40, with the `OpenedByTouch` flag for the element).
- `Flavour(baseline)` and `Flavour(expressive)` carry the Material look choice (MAT-P3-19†, MAT-X13).

Conditions the rows need beyond S7.1 (ADD-A04), each with its source and the rows:

| Condition | Meaning, source | Rows |
|---|---|---|
| `OnOffLabels` | the switch draws I and O marks; iOS's On/Off Labels setting | X-A11-6 |
| `PreferCrossFade` | iOS's Prefer Cross-Fade Transitions: presentation transitions become crossfades | X-A11-9, X-MO-10 |
| `FullKeyboardAccess` | macOS's and iOS's Full Keyboard Access: buttons show focus, an extra focus indicator | MAC-X13, MAC-X31†, X-FO-3† |
| `AccessibilityTextSize` | the content size category is an accessibility category; layouts change axis | X-DT-3, X-A11-10 |
| `Idiom` | `Phone`, `Pad` (the idiom axis beside the size class) | X-SV-1 |
| `TintDimmed` | the window or scene is covered by a modal presentation; iOS dims the tint | X-CO-3 |
| `SystemSetting(Key)` | a platform-specific switch the reader reads, declared by the preset in `[declare] conditions`: Win32 `menu_fade`, `tooltip_fade`, `selection_fade`, `combo_animation`, `listbox_smooth_scroll`, `hot_tracking`, `drop_shadow`, `ui_effects` (W3-53†); KDE `ui_effects` (K-P3-24†); macOS `wallpaper_tinting` (MAC-X27) and `title_icons` (MAC-X31†); iOS `glass_look` (X-MT-11) | W3-53†, K-P3-24†, MAC-X27, MAC-X31†, X-MT-11 |

Conditions the rows do not need: a "colour filters" or "invert" condition (system-applied, R1.1: K3.93, W3-56†,
MAC-P4-3†, X-A11-8, MAT-X35†), a "window asking for attention" or "not responding" condition (drawn by the shell or
the compositor: GN-P3-7†, K-P4-5†, K3.16†).

### readers

One line per read: platform, what, API or key as the row states it, rows. `(R18)` marks the platforms without a
reader until the platform-support milestone; the milestone implements these, the model already carries them.

**GNOME (libadwaita and GTK)**

- portal accent RGB snapped to the nearest of libadwaita's nine accents (`adw-settings-impl-portal.c:140`,
  `adw-accent-color.c:160`), replacing the raw RGB the reader uses: GN-3.15.
- accent-color enum, `org.gnome.desktop.interface` (schema lines 314-320), fallback to the portal: GN-3.16.
- reduced-motion, `org.gnome.desktop.a11y.interface` (schema lines 18-24); `enable-animations` already read: GN-3.25,
  GN-3.26.
- keyboard-focus-visible-timeout, `org.gnome.desktop.a11y.interface` (lines 25-32): GN-3.30.
- caret blink setting, period and timeout, `org.gnome.desktop.interface` (lines 63-85): GN-3.32, GN-3.33, GN-3.34.
- always-show-text-caret, `org.gnome.desktop.a11y` (lines 12-19): GN-3.38.
- cursor theme and size, `org.gnome.desktop.interface` (lines 180-189): GN-3.46.
- double-click time and drag threshold, `org.gnome.desktop.peripherals` (lines 243-252): GN-3.47, GN-3.48; key repeat
  (lines 144-159): GN-3.68†.
- antialiasing, hinting, subpixel order, `org.gnome.desktop.interface` (lines 277-312): GN-3.56.
- font-name family list (the reader drops it today, `gnome/mod.rs:194-200`) and `fc-match` for the emoji family:
  GN-FM01†, GN-FM02†.
- text direction, `gtk_get_locale_direction()`: GN-FM03†.

**KDE (Breeze and Kirigami)**

- breezerc `[Common] ShadowSize`, `ShadowStrength`, `ShadowColor`: K3.1, K3.2.
- kwinrc `[Effect-blur] BlurStrength`, `NoiseStrength`, `Saturation`: K3.11.
- kdeglobals `[KDE] AnimationDurationFactor`: K3.30, K3.90, Q3.24; plasmarc `[Units] longDuration`: Q3.24.
- breezerc `[Style] AnimationsEnabled`, `AnimationSteps`, `AnimationsDuration`, `StackedWidgetTransitionsEnabled`:
  K3.31.
- kdeglobals `[ColorEffects:Disabled]` (six keys: intensity, colour, contrast effect and amount), `[ColorEffects:Inactive]`
  including `ChangeSelectionColor` and `Enable`, and the `[Colors:*][Inactive]` subgroups: K3.40, K3.50, K3.51.
- kdeglobals `[Colors:*]` (all seven sets, the keys the reader flattens today): Q3.1, K3.54.
- kdeglobals `[KDE] contrast` (unread today) and `frameContrast` (the factor kept, not only finished colours):
  K3.53, K3.91, Q3.14.
- kdeglobals `[KDE] CursorBlinkRate`: K3.41; `DoubleClickInterval`: K3.70; `StartDragDist`, `StartDragTime`: K3.71;
  `WheelScrollLines`: K3.72; `SingleClick`: K3.73; `SmoothScroll`: K-P3-18†; `GraphicEffectsLevel`: K-P3-24†.
- breezerc `[Style] MnemonicsMode`: K3.44.
- kdeglobals `[General] XftAntialias`, `XftHintStyle`, `XftSubPixel` (keys to verify in plasma-workspace): K3.45.
- kdeglobals font strings with their extras, `[General] taskbarFont`: K3.46, K3.47.
- icon theme `index.theme` `*Default` sizes: K3.63.
- kcminputrc `[Mouse] cursorTheme`, `cursorSize`; key repeat (`kwin src/main.cpp:111-112`,
  `src/keyboard_input.cpp:196-209`): K3.64, K-P3-16†.
- `org.kde.KWin.TabletModeManager` D-Bus property (to verify): K3.83.
- breezerc and kwinrc menu opacity and blur (K3.10, K3.11): K3.92.
- `fc-match` for the emoji family and per language; `QGuiApplication::layoutDirection()`: K-FM01†, K-FM02†, K-FM03†.

**Windows**

- `UISettings.GetColorValue` for `AccentDark1`–`3`, `AccentLight1`–`3`: W3-12.
- `SPI_GETKEYBOARDCUES`: W3-21, W3-48†.
- `SPI_GETCLEARTYPE`, `SPI_GETFONTSMOOTHING`, `SPI_GETFONTSMOOTHINGTYPE`, `SPI_GETFONTSMOOTHINGCONTRAST`: W3-22.
- `SPI_GETCARETWIDTH`, `GetCaretBlinkTime`, `UISettings.CaretWidth`, `UISettings.CaretBlinkRate`,
  `SPI_GETCARETTIMEOUT` (to verify): W3-24.
- `UISettings.CursorSize`, `SM_CXCURSOR`: W3-29.
- `GetSysColor` under high contrast, `AccessibilitySettings.HighContrastScheme`, `UISettings.UIElementColor` (to
  verify): W3-35.
- `UISettings.AnimationsEnabled`, `SPI_GETCLIENTAREAANIMATION`: W3-37; `UISettings.AdvancedEffectsEnabled`, registry
  `EnableTransparency`: W3-38.
- `GetDoubleClickTime` or `UISettings.DoubleClickTime`, `SPI_GETMOUSEHOVERTIME`, `SPI_GETMENUSHOWDELAY`, `SM_CXDRAG`,
  `SM_CYDRAG`, `SPI_GETWHEELSCROLLLINES`, `UISettings.MessageDuration`, `SPI_GETKEYBOARDDELAY`: W3-41;
  `SM_CXDOUBLECLK`, `SM_CYDOUBLECLK`: W-P3-12†; `SPI_GETWHEELSCROLLCHARS`: W3-52†; `SPI_GETMOUSEVANISH`: W-P3-10†.
- `HKCU\Console` `FaceName`, `FontSize`, `FontWeight` (host to verify): W3-47†.
- `SPI_GETMENUFADE`, `SPI_GETTOOLTIPFADE`, `SPI_GETSELECTIONFADE`, `SPI_GETCOMBOBOXANIMATION`,
  `SPI_GETLISTBOXSMOOTHSCROLLING`, `SPI_GETHOTTRACKING`, `SPI_GETDROPSHADOW`, `SPI_GETUIEFFECTS`: W3-53†.
- `SPI_GETMENUDROPALIGNMENT`: W3-55†.
- `IDWriteFontFallback::MapCharacters` (emoji string; per locale), `LanguageFontGroup`: W-FM01†, W-FM02†.

**macOS**

- `NSColor` read without clipping to sRGB, `colorSpace`: MAC-X10.
- `NSColor` resolved under `NSAppearance` vibrantLight and vibrantDark: MAC-X03; under `accessibilityHighContrastAqua`
  and `…DarkAqua`, stored as the inputs' per-contrast bases and colour rules: MAC-X22.
- `NSColor.withSystemEffect` (rollover, pressed, deepPressed, disabled) of each read colour, stated as state rules:
  MAC-X30†.
- `NSAnimationContext.duration`, `NSWindow.animationResizeTime(_:)`, `CAMediaTimingFunction` control points: MAC-X11.
- `NSEvent.doubleClickInterval`, `keyRepeatDelay`, `keyRepeatInterval`, `NSButtonCell.getPeriodicDelay`, user defaults
  `NSTextInsertionPointBlinkPeriodOn` and `…Off`, `NSInitialToolTipDelay`: MAC-X12.
- `NSApplication.isFullKeyboardAccessEnabled` or user default `AppleKeyboardUIMode` (both to verify): MAC-X13,
  MAC-X31†.
- `NSWorkspace.accessibilityDisplayShouldDifferentiateWithoutColor` (already probed): MAC-X24.
- pointer size and colour user defaults, link cursor: MAC-X20 (keys to verify).
- `NSScrollView` line and page scroll properties: MAC-P3-17†; user default `NSScrollAnimationEnabled`: MAC-P4-4†.
- `CTFontCreateForString` on an emoji string; `CTFontCopyDefaultCascadeListForLanguages`: MAC-FM01†, MAC-FM02†.

**iOS (R18 milestone; the application stands in until then, R2.6)**

- `UIColor.resolvedColor(with:)` over trait combinations; wide-gamut components; `UIColor.Prominence`: X-CO-1, X-CO-4,
  X-CO-5.
- `isDarkerSystemColorsEnabled`, `accessibilityContrast`; `isReduceTransparencyEnabled`; `isReduceMotionEnabled`,
  `prefersCrossFadeTransitions`; `isBoldTextEnabled`, `legibilityWeight`; `isOnOffSwitchLabelsEnabled`;
  `shouldDifferentiateWithoutColor`; `preferredContentSizeCategory`, `isAccessibilityCategory`, `UIFontMetrics`:
  X-A11-1…X-A11-7, X-A11-10, X-DT-2, X-DT-3, X-DT-4, X-MO-10, X-MT-3.
- Dynamic Type size per style per content size category: X-DT-1.
- `UITraitCollection` size classes and idiom, `layoutDirection`: X-SV-1, X-SV-3, P3-14†.
- `UIScrollView.DecelerationRate`, `minimumPressDuration`, `UIBackgroundConfiguration.shadowProperties`,
  `UIBarAppearance.backgroundEffect`, `UICornerConfiguration`: X-MO-8, X-BT-1, X-SH-2, X-MT-2, X-CR-2.
- `CTFontCreateForString`, `CTFontCopyDefaultCascadeListForLanguages`: FM-1†, FM-2†. The user's Liquid Glass look
  preference (source to verify): X-MT-11. SF Symbols loader and the reader itself: X-IC-7, X-RD-1.

**Android (R18 milestone)**

- dynamic colour scheme roles and variant: MAT-X17; `UiModeManager.getContrast()`: MAT-X18; `Configuration.uiMode`:
  MAT-P3-18†.
- `Configuration.fontScale`, `FontScaleConverter`, `fontWeightAdjustment`: MAT-X19.
- `Settings.Global.ANIMATOR_DURATION_SCALE`, `TRANSITION_ANIMATION_SCALE`: MAT-X20.
- `ViewConfiguration` `getLongPressTimeout`, `getDoubleTapTimeout`, `getScrollDefaultDelay`, `getScrollBarFadeDuration`
  (MAT-X25); `getScaledTouchSlop`, `getScaledPagingTouchSlop`, `getScaledDoubleTapSlop`,
  `getScaledVerticalScrollFactor` (MAT-X32†); `getKeyRepeatTimeout`, `getKeyRepeatDelay` (MAT-P3-14†);
  `getScrollFriction`, `getScaledMinimumFlingVelocity`, `getScaledMaximumFlingVelocity` (MAT-P3-16†).
- `android:ambientShadowAlpha`, `android:spotShadowAlpha` and framework light dimensions: MAT-X3.
- `Typeface.DEFAULT`, `Typeface.MONOSPACE`: MAT-X36†. `AFontMatcher_match` (emoji, per locale):
  MAT-FM01†, MAT-FM02†. `Configuration.getLayoutDirection()`: MAT-FM03†. `softInputMode`, `WindowInsetsAnimation`:
  MAT-P4-2†.

Fixes of today's reads that these rows contain: GNOME's dropped font list (GN-FM02†), the raw portal accent
(GN-3.15), KDE's unread `contrast` and the flattened `frameContrast` (K3.53, Q3.14), the Windows reader's one
`reduce_motion` flag standing for several settings (W3-37, W3-53†).

### placement strategies

S13's closed list gains (ADD-A21), each a pure function of the anchor, the element's size, the window, the keyboard's
rectangle where it applies, and the direction, implemented once to its platform's source:

| Strategy | What it places | Parameters | Rows |
|---|---|---|---|
| `WinUiFlyout`, `AppKitMenu` (existing) with a `handedness` parameter | pop-up menus opening to the left or right of their anchor under the Handedness setting | `handedness`, read by `SPI_GETMENUDROPALIGNMENT` on Windows; the other platforms have no such setting (the pointer rows of W3-55† say so) | W3-55† |
| `KeyboardAvoid(WinUiScrollIntoView)` | the focused field is scrolled into view when the touch keyboard shows (`InputPane` `Showing`, `Hiding`) | the nearest scroll container, the motion (measured) | W-P4-3† |
| `KeyboardAvoid(AndroidPan)`, `(AndroidResize)`, `(AndroidInsetsAnimation)` | what moves when the on-screen keyboard opens: pan the window, resize it, or move content by the insets animation | `softInputMode`, the inset animation | MAT-P4-2† |
| `MaterialScaffoldSlot(Fab)`, `MaterialScaffoldSlot(Snackbar)` | the floating action button at end or centre with its spacing, the snackbar above it and the bottom bar | position, spacing keys | MAT-X38† |
| `MaterialLevitatedPane` | a pane floating above the scaffold like a sheet or pop-up: alignment and optional scrim | alignment, scrim | MAT-SF04† |

Not placement strategies, by R1.1: the shell or compositor moving a window for the keyboard (GN-P3-9†, K-P3-21†).
The first row's `handedness` and the keyboard strategies take the keyboard's rectangle from the connector, as the
direction and the window rectangle come from it.

### system controls

None. No row of group 8 needs `SystemControl` (S15): the AppKit whole-control renderings are the checkbox, radio,
pop-up and disclosure rows of the control groups (MAC-C06, MAC-CB01, MAC-EX02, MAC-L10). The gradient fill of the
macOS 12 to 15 checkbox (MAC-X09) is a `PaintKind::Linear` and needs no rendering.
