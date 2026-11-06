# Model-gap seeds (native-theme, read-only scan 2026-10-27)

Scope searched: connectors/*/src (all .rs, incl. native-theme-iced/src/contract/ and styles/), connectors/*/mapping.toml, connectors/*/README.md, native-theme/src (doc comments and presets), docs/todo.md, docs/todo_gpui-full-theme.md, docs/todo_iced-full-theme-geometry.md, docs/platform-facts.md, docs/archive/*spec*.md and *rationale*.md, ROADMAP.md.
Phrases (case-insensitive): "the model states no/has no/carries no/lacks/does not carry", "shadow colour but no", "no field", "no leaf", "not in the model", "iced default:", "keeps egui's own", "egui's own", "gpui's own", "upstream's own", "toolkit's default", "no receiver", "states no", "carries no", "no row", "can't/cannot express", "model gap".
Every file:line below is at caeddd6b. The full hit list (1178 lines) was a working file of the scan and is not kept; rerun the phrase search above to regenerate it.
The iced contract's `iced default:` entries (135) are all in `connectors/native-theme-iced/src/contract/derived.rs`.
Group citations are the representative sites read in full; a line ending "(not expanded)" names a file whose hits were counted but not listed. Archive egui spec/rationale hits are historical and appear only where they name a group.

## G01 shadow geometry (only a shadow colour is modelled; no offset, blur, spread, layers)
- ROADMAP.md:88-89 — "the model has a shadow colour and a per-widget `shadow_enabled`, but no shadow's offset, blur, spread or layers"
- ROADMAP.md:166 — "every style keeps iced's own `shadow` until the model carries shadow geometry (v0.7.1)"
- connectors/native-theme-iced/src/contract/derived.rs:78,90,102,114,126,138,205,559,578,590,615,627,639,681,745,770,819 — "the model has a shadow color but no offset or blur" (button, container, overlay menu variants)
- connectors/native-theme-iced/src/contract/derived.rs:529 — scrollable auto_scroll.shadow, "the autoscroll overlay has no native counterpart"; :311 scrollable container.shadow
- connectors/native-theme-iced/src/contract/derived.rs:831 — iced_aw menu_bar bar_shadow, "the model has a shadow color but no offset or blur"
- connectors/native-theme-iced/src/styles.rs:195 — "`button::Style::default()`: the model carries a shadow color but no offset"
- connectors/native-theme-iced/src/styles.rs:1246 — "`shadow` has no native source -- the model has `defaults.shadow_color` but"
- connectors/native-theme-iced/src/styles.rs:1660, 1798, 2143, 2184 — container shadow "the model carries a shadow colour but no offset or blur"
- connectors/native-theme-iced/src/styles/aw.rs:127 — "`menu_bar::primary(theme, status)`: the two shadows, because the model"
- connectors/native-theme-gpui/src/colors.rs:545, 1200 — defaults.shadow_color reads (no geometry)
- connectors/native-theme-egui/mapping.toml:244, 422, 556, 626, 743 — `shadow.spread` / `visuals.*_shadow.spread` "never: egui's own spread, 0"
- docs/todo.md:146 — `shadow_enabled` per widget, shadow geometry open
- docs/todo.md:2204-2217 — `Theme::shadow` has one reader; semantic shadow tokens unread
- docs/todo.md:2864-2866 — theme `shadow` flag one reader; `ButtonVariant::shadow` false
- docs/todo.md:1540-1543 — windows-11 `defaults.shadow_color` one colour serves popup and window shadows
- docs/todo.md:1160-1167 — "platform-facts: no shadow rows for the toolbar and the list"
- docs/platform-facts.md@fc6109d1:1057 — `shadow_color` row (Chapter 2 global colours)

## G02 shadow on a specific widget: progress bar, slider thumb, expander, scrollbar, popup
- docs/todo.md:212-227 — "Slider: the thumb's outline or shadow" (soft shadow on macOS 26 knob; geometry not in model)
- docs/todo.md:740-759 — "Progress bar: Breeze's groove, the radii, the shadow"; "No preset states `progress_bar.border.shadow_enabled`"
- docs/todo.md:1398, 1416 — card/expander shadow inherited, no per-widget source
- docs/todo.md:3263, 3271 — `border.shadow_enabled` and expander `shadow_enabled` (false) without source

## G03 snap (iced renderer setting, not a theme property)
- connectors/native-theme-iced/src/contract/derived.rs:84,96,108,120,132,144,317,565,584,596,621,633,645,687,751,825,937,... — "a renderer setting, cfg!(feature = \"crisp\")"
- derived.rs:554, 705 — rule::default(theme).snap "a renderer setting"

## G04 checked checkbox hover / pressed fill (model states hover and pressed only for the unchecked box)
- connectors/native-theme-iced/src/contract/rows.rs:399, 403 — "the model states no hovered or pressed checked"; "the checked pair where it states none"
- connectors/native-theme-iced/src/contract/rows.rs:901-902 — "`hover_background` is the hover of the *unchecked* box; the model states no hover for a checked one"
- connectors/native-theme-iced/src/styles.rs:716 — "The model states no hover for a checked"
- connectors/native-theme-iced/src/styles.rs:789 — "The model states no hover for a checked box."
- docs/todo.md:51 — "### Checkbox: a checked checkbox's hover appearance"
- docs/todo.md:827-830 — checkbox.disabled_background stands for both checked and unchecked states

## G05 checkbox disabled checked fill / disabled_opacity has no receiver
- connectors/native-theme-iced/src/styles.rs:885 — "the model's disabled fields have no" receiver (checkbox/radio shared CheckboxTheme)
- connectors/native-theme-egui-widgets/src/lib.rs:112 — "the theme states no `checkbox.disabled_background` it is its checked self"
- docs/todo.md:2528-2537 — "`checkbox.disabled_opacity` has **no receiver** in native-theme-iced"; whether the model should state a disabled checked fill

## G06 checkbox check mark glyph / size; radio dot diameter
- connectors/native-theme-iced/src/styles.rs:728 — "The check mark is `checkbox.indicator_color` -- the model has no"
- connectors/native-theme-iced/src/styles.rs:899 — "`checkbox.radio_dot_diameter` has no receiver in `radio::Style`"
- connectors/native-theme-iced/src/lib.rs:1220, 1351 — radio dot size and indicator states none (ios-13, colour-scheme presets; Windows/macOS mark)
- connectors/native-theme-iced/src/contract.rs:119 — "`radio::Style` states its border as a bare"
- docs/todo.md:185-189 — "Checkbox: the check mark's glyph and size"; "The model has no"
- docs/todo.md:239-255 — "Radio: Material's radio is not the checkbox's shape"; preset states no `radio_dot_diameter`
- connectors/native-theme-egui-widgets/src/radio_button.rs:30-31 — dot is egui's own where theme states none
- connectors/native-theme-egui-widgets/src/combo_box.rs (check: 6 hits) (not expanded)

## G07 pressed / hovered primary and status button fills
- connectors/native-theme-iced/src/styles.rs:312 — "The model gives the accent and the three status colors a fill and a label"
- connectors/native-theme-iced/src/styles.rs:380 — "The model states no hovered and no pressed primary fill, so"
- connectors/native-theme-iced/src/contract.rs:3059 — "The model states no hovered and no pressed primary fill, so those two"
- connectors/native-theme-iced/src/contract/rows.rs:470 — "model states no hovered and no pressed variant of a primary or a status"
- docs/todo.md:4014-4015 — `button.checked_background` / `button.active_background` unstated (macos-tahoe); "egui's and gpui's pressed fill"

## G08 pressed segment; hovered selected segment; segment text colour on hover
- connectors/native-theme-iced/src/styles.rs:1927-1928 — "the model states no hover for it, so a hovered selected segment is exactly the selected one. The model states no pressed"
- connectors/native-theme-iced/src/styles.rs:260 — "The model states no hovered and no pressed checked toggle"
- connectors/native-theme-iced/src/contract/rows.rs:2234-2235 — "since the model states no hover of its own for it; pressed, the hovered look, since the model states no pressed segment"
- connectors/native-theme-iced/src/contract/rows.rs:2363 — "rest and when disabled, which the model states no fill for"
- native-theme/src/windows.rs:422 — "which the model cannot carry: it has no hovered segment text"
- native-theme/src/windows.rs:1271 — "a hovered item's highlight text has no field: the hover is not written"
- connectors/native-theme-gpui/src/colors.rs:463 — SegmentedControlTheme note (no hover field discussed)
- docs/todo.md:171 — "Segmented control: the join and the divider"; docs/todo.md:1523 — "Model: a hovered segment's text colour"

## G09 dragged slider thumb; slider handle and rail border
- connectors/native-theme-iced/src/styles.rs:1292, 1299 — "The model states no dragged thumb color"; "carries no border, so the rail's border and the handle's border width"
- connectors/native-theme-iced/src/styles.rs:1319 — "The model states the thumb's diameter; iced states a circular handle by"
- connectors/native-theme-iced/src/contract/rows.rs:1563, 1570 — "The statuses whose handle the model states a *fill* for"; "SliderTheme states no"
- connectors/native-theme-iced/src/contract/rows.rs:1637 — iced states circular handle by radius, model states diameter
- connectors/native-theme-iced/src/contract.rs:2408; contract.rs:2937-2940 — "dragged slider handle vs iced's own default"
- connectors/native-theme-iced/src/contract/pairs.rs:530 — "handle *fill* the model states: a dragged handle's fill is iced's"
- connectors/native-theme-iced/src/contract/derived.rs:211-254 — "SliderTheme carries no border" (rail.border.color/width/radius; handle.border_width/border_color)
- docs/todo.md:212-240 — "Slider: the thumb's outline or shadow"; docs/todo.md:2402 — `slider.tick_mark_length` (no receiver at all)
- ROADMAP.md:91-92 — "a dragged slider thumb" among ~40 "the model states no" properties
- connectors/native-theme-egui-widgets/src/slider.rs:28-38 — rail radius and fg_stroke as egui's own where theme states none; no tick marks

## G10 progress bar: indeterminate motion, minimum width, indeterminate/ circular diameter
- connectors/native-theme-gpui/src/widgets/progress.rs:20 — "The period of an indeterminate bar's sweep, which the model does not"
- connectors/native-theme-gpui/src/widgets/progress.rs:139 — "the model states no motion"
- connectors/native-theme-iced/src/styles.rs:1557 — "`progress_bar.min_width` has no receiver at all"
- connectors/native-theme-iced/src/styles.rs:1557 area (ProgressBar::girth via builder) — see lib.rs
- ROADMAP.md:91-92 — "a circular progress indicator's diameter, motion"
- docs/todo.md:740-759 — groove, radii and shadow

## G11 spinner: stroke width, fins/arc geometry, minimum size, container background/border
- connectors/native-theme-iced/src/contract/derived.rs:924-969 — "container::Style::default().background -- a spinner is painted on whatever the consumer puts it on"; ".border -- SpinnerTheme carries no border" (x6)
- connectors/native-theme-iced/src/styles/aw.rs:474-476 — `.min_diameter` no receiver; "states no minimum anywhere"
- connectors/native-theme-iced/src/spinner.rs (1 hit) (not expanded)
- connectors/native-theme-gpui/src/widgets/spinner.rs (1 hit) (not expanded)
- connectors/native-theme-egui-widgets/README.md:80 — "On macOS the spinner is an arc, not fins: the theme states no fin geometry."
- connectors/native-theme-egui-widgets/src/spinner.rs (2 hits) (not expanded)
- docs/todo.md:361 — "Presets: a spinner stroke width platform-facts does not give"

## G12 scrollbar: container, rails, scroller borders, gap, auto-scroll overlay, margin/anchor
- connectors/native-theme-iced/src/contract/derived.rs:263-318 — "ScrollbarTheme states nothing about the scrolled container" (container.text_color, background, border x6, shadow, snap)
- derived.rs:323-472 — "ScrollbarTheme carries no border" (vertical_rail/horizontal_rail border and scroller border)
- derived.rs:479-481 — scrollable gap "the fill between a horizontal and a vertical scrollbar has no native counterpart"
- derived.rs:486-536 — auto_scroll background, border x6, shadow, icon: "the autoscroll overlay has no native counterpart"
- connectors/native-theme-iced/src/styles.rs:1379 — "`ScrollbarTheme` carries no border"; styles.rs:1518-1525 — "model states no gap ... anchor stay Scrollbar::new()'s own"; `scrollbar.min_thumb_length` no receiver in iced 0.14
- connectors/native-theme-iced/src/styles.rs:1375 — "the model states no appearance for that" (overflow scrollbar)
- docs/todo.md:389-429 — "Presets: the scrollbar thumb colour platform-facts measures"; "`[dark.scrollbar]` states none"; "`macos-tahoe` states no"
- connectors/native-theme-gpui/src/config.rs:101 — "stay upstream's scheme: the model states none of them" (code-highlight line numbers/current line/syntax)

## G13 text input: input icon colour, placeholder, open pick-list handle, icon
- connectors/native-theme-iced/src/contract/derived.rs:186 — text_input.icon "the model carries no input-icon color"
- connectors/native-theme-iced/src/styles.rs:671 — `text_editor::Style` has no `icon` field the model states none of its own for
- connectors/native-theme-iced/src/styles.rs:1177 — placeholder: "the model states it once, for every field"
- connectors/native-theme-iced/src/contract/derived.rs:199 — pick_list handle_color "ComboBoxTheme states the arrow's sizes but not its color"
- connectors/native-theme-iced/src/styles.rs:1182 — "model states no separate open appearance. `handle_color` has no native"
- connectors/native-theme-iced/src/contract/rows.rs:1446-1447 — "The model states no open appearance, so an open field follows its own `is_hovered`"
- docs/todo.md:2389-2402 — egui/gpui list of model fields with no receiver

## G14 switch / toggler: text font, label gap, thumb radius
- connectors/native-theme-iced/src/contract/derived.rs:192-194 — toggler text_color "the model states no font for a switch"
- connectors/native-theme-iced/src/styles.rs:982 — "`toggler::default(theme, status)`: the model states no font for a switch"
- connectors/native-theme-iced/src/contract/rows.rs:1324 — "`SwitchTheme` carries no font"
- connectors/native-theme-iced/src/contract/rows.rs:1358, 1375 — switch inset and geometry cannot state the inset iced asks for
- connectors/native-theme-iced/src/lib.rs:926, 930 — "the model states no thumb radius"; "`SwitchTheme` states no font and no gap for one"

## G15 link (LinkTheme): border, pressed/disabled fill, disabled_opacity, hover darkening
- connectors/native-theme-iced/src/contract/derived.rs:150-181 — button::text border.color/width/radius "LinkTheme carries no border" (x6)
- connectors/native-theme-iced/src/styles.rs:491-493 — "`LinkTheme` states no pressed and no disabled fill"; "carries no border either"
- connectors/native-theme-iced/src/styles.rs:510 — "`LinkTheme` states no `disabled_opacity`"
- connectors/native-theme-iced/src/contract/rows.rs:534, 548, 558 — LinkTheme pressed/disabled fill; "It has no scalar rows: `LinkTheme` carries no border"
- docs/todo.md:1656-1664 — "Links: Kirigami darkens a pressed link"; "The model has no darkening rule"
- connectors/native-theme-egui-widgets/src/links.rs:15, 27 — hover underline egui's own; `link.underline_enabled`

## G16 card / container: text colour, font, fill
- connectors/native-theme-iced/src/contract/derived.rs:571-573 — container text_color "CardTheme carries no font, and iced's None inherits the label color"
- connectors/native-theme-iced/src/styles/aw.rs:63 — "`CardTheme` carries no font, so the text on a"
- connectors/native-theme-iced/src/styles.rs:1700 — "`CardTheme` carries no font, so a card does not recolor the text inside it"
- connectors/native-theme-iced/src/contract/rows.rs:2520 — "platform's own text color -- `CardTheme` carries no font"
- connectors/native-theme-iced/src/contract/rows.rs:2471 — "A card and a menu bar look the same in every status -- the model states one"
- docs/todo.md:3968 — windows-11 states a Fluent card fill; windows-11.toml states no `card.background_color`

## G17 expander frame: resting fill, label colour, hover fill, pressed header, disabled_opacity
- connectors/native-theme-iced/src/contract/derived.rs:757-777 — expander text_color "ExpanderTheme states no label for the frame"; background "ExpanderTheme states no fill for the frame" (764-765)
- connectors/native-theme-iced/src/styles.rs:2010-2015 — "The model states no resting fill for an expander header"; "The model states no pressed header"
- connectors/native-theme-iced/src/styles.rs:2020, 2043 — `ExpanderTheme` states no `disabled_opacity`
- connectors/native-theme-iced/src/styles.rs:2026 — "the theme states nothing (the macOS facts give "(none)")"
- connectors/native-theme-iced/src/styles.rs:2102 — "The model states no fill and no label for the frame"
- connectors/native-theme-iced/src/contract/rows.rs:2363, 2380 — expander resting fill; "since `ExpanderTheme` states no `disabled_opacity`"
- connectors/native-theme-iced/src/contract.rs:3821 — expander disabled header
- connectors/native-theme-egui-widgets/src/expander.rs:51 — arrow and header where theme states none: egui's own
- docs/todo.md:1185 — "### Presets: expander hover fills with no source"
- docs/todo.md:699 — "The tab gap, the check mark's line, the expander's structure, the text area"

## G18 rule / separator radius and fill mode; toolbar and toolbar rule; line holds no text
- connectors/native-theme-iced/src/contract/derived.rs:541-554 — rule radius "SeparatorTheme is a line color and a line width"; fill_mode "SeparatorTheme states nothing about it"
- derived.rs:693-705 — toolbar rule radius/fill_mode
- derived.rs:651-676 — toolbar container border colour/width/radii "the edge is the rule under the bar", "the bar is square"
- derived.rs:710-751 — separator line text_color, border ("a line holds no text", "the line is its fill")
- connectors/native-theme-iced/src/styles.rs:1892 — "the model states the separators' width and no colour of their own"

## G19 pane grid hovered_region / drop target
- connectors/native-theme-iced/src/contract/derived.rs:782-814 — pane_grid hovered_region.background/border "the model states no drop target"
- connectors/native-theme-iced/src/styles.rs:1620-1628 — "the model states no drop target"; "has no receiver: iced draws a split only while it is hovered or picked"
- connectors/native-theme-iced/src/contract/rows.rs:1931 — "`SplitterTheme` states no dragged colour, and the dragged line is"
- ROADMAP.md:159-161 — "highlight under a pane dragged over another ... the model states no drop target"

## G20 menu bar fill, text colour, geometry (iced_aw menu_bar)
- connectors/native-theme-iced/src/styles/aw.rs:127-134 — menu_bar::primary: shadows; "`menu_bar::Style` states no text color and no geometry"; "no receiver"
- connectors/native-theme-iced/src/contract.rs:2662, 2731 — model describes no tab/item/row; menu bar states no text color
- connectors/native-theme-iced/src/contract/pairs.rs:805 — "`styles::aw::menu` has no pair: `menu_bar::Style` states no text color at"
- connectors/native-theme-iced/src/contract/rows.rs:2598 — "one panel the model states, and the open path is the menu's hover fill"
- docs/todo.md:1171-1178 — "Model: the menu bar has no fill of its own"; "The model has no menu-bar fill"
- docs/todo.md:3608 — "The menu-bar row has no fill or font of its own: the model has no"

## G21 sidebar: unselected-item fill, hovered label, border radius, text
- connectors/native-theme-iced/src/styles/aw.rs:290-298 — "the platform states no separate fill for an unselected item"; "`SidebarTheme` states no border ..."; "`sidebar.border.corner_radius` has no receiver: `sidebar::Style` carries"
- connectors/native-theme-iced/src/contract/rows.rs:2763 — "`SidebarTheme` states no hovered label"
- connectors/native-theme-iced/src/contract.rs:2731 — "the model describes no tab, item or row"

## G22 tab bar / tab: unselected-tab border, min width/height, selected flag, separators, flares
- connectors/native-theme-iced/src/styles/aw.rs:193-214 — "The model states nothing for an unselected tab's"; "`tab.min_width` and `.min_height` have no receiver"; "the model describes no tab in"
- connectors/native-theme-iced/src/contract/rows.rs:2720 — tab outlined `tab.border.line_width`, other statuses
- connectors/native-theme-iced/src/contract/pairs.rs / contract.rs:2731 — tab rows
- docs/todo.md:112-115 — "WinUI 3's `TabView` draws two lines around its tabs that the model"
- docs/todo.md:143 — "Tab: the border is the selected tab's, and KDE's live tab colours"
- docs/todo.md:2420 — iced TabBar "carries no selected flag, so a hovered *selected* tab loses its selected"
- docs/todo.md:3693 — gpui tab scrolled into view loses its outer flare (tab_flares.rs context)

## G23 selection list / list row / table header, grid, footer
- connectors/native-theme-iced/src/styles/aw.rs:380-425 — "`list.disabled_text_color` on the list's own fill"; "`list.border.corner_radius` has no receiver: `selection_list::Style` carries"; "`.grid_color` have no receiver at all -- a selection list has no striping"
- connectors/native-theme-iced/src/contract/rows.rs:2504 — "a row the model does describe, through `list.disabled_text_color`"
- connectors/native-theme-iced/src/contract/rows.rs:2843 — `ListTheme` states no `disabled_opacity`
- connectors/native-theme-gpui/src/colors.rs:120-123, 423, 439, 448 — "the line between rows and columns, which the model states separately"; header fill; "the model states no footer colour of any"
- connectors/native-theme-gpui/README.md:345 — `list` token: "Neither widget paints a border or a fill of its own ... the `list` token has no reader"
- connectors/native-theme-iced/src/lib.rs:1604-1608 — iced table has no row height of its own; header height "none"
- docs/todo.md:2722 — "`tree.rs` draws no row content at all"

## G24 title bar / window chrome / toolbar height / dialog placement
- docs/todo.md:2842-2852 — "a `TitleBar`'s 34px (settable; the model states no title-bar height)"
- docs/todo.md:1063 — "### windows-11: the title bar with \"Show accent colour on title bars\""
- docs/todo.md:1705 — "### Dialog: the platform's vertical placement"
- docs/todo.md:2073-2077 — window border shadow drawn by gpui; no theme field
- connectors/native-theme-gpui/src/config.rs:101 — see G12
- connectors/native-theme-iced/src/lib.rs:1719, 1728 — toolbar height "None where the platform states none"
- native-theme/src/windows.rs:1375 area — logical vs effective px (unit question, not a gap)

## G25 focus ring: receiver, width, offset, which part of control is ringed
- connectors/native-theme-iced/README.md:673-677 — "operation to no field, so `focus_ring` cannot ring it"; "iced's theme has no receiver"
- connectors/native-theme-iced/src/lib.rs:374-376 — "operation to no field, so `focus_ring` (feature `widgets`) cannot ring"
- connectors/native-theme-iced/src/focus_ring.rs:31 — "`None` where the theme states no ring"
- connectors/native-theme-gpui/src/widgets/mod.rs:303 — "3 px ring at half `ThemeColor::ring`'s alpha whatever the model states"
- docs/todo.md:502-528 — "platform-facts: which part of a control the focus ring surrounds"
- docs/todo.md:2099 — "`Theme::focus_ring_width` ... has no receiver"

## G26 motion, reduced motion, high contrast, reduced transparency (accessibility receivers)
- connectors/native-theme-iced/README.md:576-577 — `reduce_transparency`, `high_contrast`: "no receiver in iced's theme"
- connectors/native-theme-iced/README.md:677 — "High contrast and reduced transparency. iced's theme has no receiver"
- docs/todo.md:2611 — "native-theme states no motion. KDE has `AnimationDurationFactor`"
- docs/todo.md:2681-2688 — "a whole category the model is missing": `Theme::motion`
- ROADMAP.md:91-92 — "motion, among others"
- connectors/native-theme-gpui/src/widgets/progress.rs:139 — see G10
- docs/platform-facts.md@fc6109d1:1128-1130 — `reduce_motion`, `high_contrast`, `reduce_transparency` (**(none)** cells in KDE/GNOME, see none list)

## G27 text colours and fonts: `*_text_color` meanings, selection text, markdown inline code, window label
- docs/todo.md:910 — "### Model: one `*_text_color` field, two meanings"
- docs/todo.md:528-534 — "Selection: the text colour macOS pairs with the unemphasised selection"
- docs/todo.md:3857-3869 — "Markdown: inline code, list markers and blockquotes in the model" ("The model states none of them")
- connectors/native-theme-gpui/src/colors.rs:216-221 — selection text pairing; defaults.selection_background
- connectors/native-theme-iced/src/styles.rs:2245-2251 — window label `defaults.text_color` "the model states no label of the window's own"
- connectors/native-theme-iced/src/lib.rs:1862-1863 — weight stretch "as the model states none"
- connectors/native-theme-gpui/src/lib.rs (13 hits) (not expanded) (text/line-height/font notes)
- connectors/native-theme-egui/src/fonts.rs:2 hits, native-theme/src/fonts.rs:1 hit (not expanded)

## G28 tooltip delay, sidebar width, popover/overlay delay
- docs/todo.md:3001-3027 — "A sidebar width and a tooltip delay are missing from the model"
- connectors/native-theme-iced/src/lib.rs:1591 — tooltip delay "states none: a `Tooltip` then keeps iced's own delay"
- connectors/native-theme-gpui/src/config.rs (1 hit) (not expanded)

## G29 table / data-grid row height, header height (iced, gpui)
- connectors/native-theme-iced/src/lib.rs:1604-1608 — table row height: "iced's `table` has no row height of its own"
- docs/todo_gpui-full-theme.md:720-722 — list.header_height_px delivered; "None where the theme states none"

## G30 button: minimum height vs exact height; padding sides; active/checked background
- docs/todo.md:3994-3995 — "gpui draws `button.min_height` as an exact height where the model states a minimum"
- docs/todo.md:718-721 — stated side none (KDE's 5) "still has no receiver"; "macOS-monterey states none"
- docs/todo.md:4014-4015 — see G07
- connectors/native-theme-iced/src/lib.rs:697-701 — iced default for padding sides

## G31 combo box / drop-down: right padding, column, arrow geometry (egui, gpui, model)
- connectors/native-theme-egui-widgets/src/combo_box.rs:271 — "states no right padding or column: egui's own place stands"
- connectors/native-theme-egui-widgets/src/combo_box.rs:40, 162 — egui's own layout stands where unstated
- docs/todo.md:3340-3343 — "model has no field for, so windows-11 states WinUI's right padding"; "`[dark.combo_box.border] padding_right_px`"; "no field"
- docs/todo.md:3302 — "windows-11 states no menu"

## G32 generic "egui's own / gpui's own / upstream's own" fallbacks (value stands; sink not in model)
- connectors/native-theme-egui/mapping.toml:19 — "[unwritten]: sink path → why egui's own value stands there"
- connectors/native-theme-egui/mapping.toml:2140, 2144 — `spacing.scroll.floating_allocated_width` / `floating_width`: egui's own value
- connectors/native-theme-egui/src/mapping_doc.rs:311, 338 — "(egui's own value stands)" / "Left to egui" table (33 hits in mapping.md, not expanded)
- connectors/native-theme-egui/src/convert.rs:9, 36, 94, 198, 250, 254, 299 — NaN fallback to egui's own value
- connectors/native-theme-egui/src/ext.rs:14, 168; accessors.rs:23-46 — no-atlas degrade to egui's own
- connectors/native-theme-egui/src/style/base.rs (16 hits), style/derived.rs (10), style/roles.rs (11), style/frames.rs (4), style/states.rs (2), style/mod.rs (4), style_diff.rs (1) (not expanded)
- connectors/native-theme-egui/src/install_tests.rs (25 hits), src/contract.rs (10), src/atlas.rs (7), src/plugin.rs (4), src/roles.rs (2)  (not expanded)
- connectors/native-theme-gpui/src/geometry.rs (17 hits): "the model states no ..." / gpui's own geometry (not expanded)
- connectors/native-theme-gpui/src/colors.rs:180 (window fill), 1051, 1067 — upstream's `background` token vs defaults
- connectors/native-theme-gpui/src/base_layer.rs:50-59 — upstream projection mirrored
- connectors/native-theme-gpui/src/widgets/mod.rs:303 — see G25
- connectors/native-theme-gpui/src/contract.rs (9), src/lib.rs (13), src/icons.rs (3) (not expanded)
- connectors/native-theme-egui-widgets/src/* (scope, parts, links, progress_bar, expander, slider, switch, spinner, tests.rs 19) (not expanded)
- connectors/native-theme-iced/src/contract/rows.rs:1012, 801 — "`checkbox::Style` has no field the model cannot fill"; "`text_editor::Style` has no field the model cannot fill" (inverse: the model cannot fill them)
- connectors/native-theme-iced/src/contract.rs:2928, 2937-2940 — "iced's own default gives ..."
- connectors/native-theme-iced/src/contract.rs:5397-5452 — UNREAD_COLOURS table: "no receiver" for colour leaves (generated list)

## G33 icon / icon-set gaps
- docs/todo.md:3886 — `system_icon_set()` returns Material on other platforms
- connectors/native-theme-gpui/src/icons.rs (3 hits) (not expanded)
- connectors/native-theme-iced/src/icons.rs? — no hits
- docs/platform-facts.md@fc6109d1:1137-1140 — icon sizes `small`/`large`/`dialog`/`panel` with (none) cells (see none list)

## G34 windows-11 values with no field (menu, combo, window)
- docs/todo.md:1504-1543 — "windows-11: values WinUI 3 states no source for"; "WinUI 3 states no selected-text foreground"
- docs/todo.md:3968 — card fill, see G16
- native-theme/src/presets/windows-11.toml (2 hits) (not expanded)
- native-theme/src/windows.rs:422, 1271 — see G08

## G35 gnome / macOS presets mentioning no field
- native-theme/src/gnome/mod.rs (2 hits), native-theme/src/model/mod.rs (2 hits), native-theme/src/resolve/inheritance.rs (1), native-theme/src/error.rs (1) (not expanded)
- docs/todo.md:2933 — "native-theme simply states no such" (upstream gap)
- docs/todo.md:2982 — ".size() from the model would reach it exactly" (size variants)
- docs/todo.md:3259-3263 — "border.line_width 0.5, border.shadow_enabled" no source
- docs/todo.md:429 — macos-tahoe states no scrollbar value (see G12)

## G36 unstated or unsourced presets (the model has a row, no preset value)
- docs/todo.md:103-104 — "The model field is still the one." (checkbox ❓ macOS)
- docs/todo.md:4015 — macos-tahoe neither button.checked_background nor active_background
- docs/todo.md:4014-4015 — see G07

## G37 platform-facts "(none)" cells — 366 at caeddd6b, not yet classified
- Each is one of two kinds: the platform does not draw that thing (it stays, with the reason in the cell), or the platform draws it and no document states its value (a value to measure, under v0.7.1's rule). Classifying them is open work of the gap map (its §6). `grep -n '(none)' docs/platform-facts.md` lists them.


## Section 2: platform-facts.md unknown (❓) cells
Format: platform-facts.md:LINE — row name — platform column — excerpt (first 220 chars). Cells counted: 224 (one line per cell; a line of the file may hold more than one).
Column names for Chapter 1 tables are the section (e.g. 1.1.4 Widget Metrics = macOS, "macOS" section); 'value column' means the table's Value column under that section.

platform-facts.md@fc6109d1:65 — `+monospacedSystemFontOfSize:weight:` — 1.1.1 Fonts (value column) — ✅ · macOS 26: the same face as on 14.8.9 and 15.7.9, `.AppleSystemUIFontMonospaced` 13pt, weight trait 0 ([probe 26 :161-165](evidence/os-probe/2026-10-11-run-38129819806/os-probe-macos-26/probe.toml#L161-L165)); whether
platform-facts.md@fc6109d1:68 — `+menuBarFontOfSize:` — 1.1.1 Fonts (value column) — ❓ weight undocumented; no evidence of SemiBold; likely Regular like other font methods · the OS probe settles it: 13pt, weight trait 0 (Regular) on 14.8.9 and 15.7.9 ([probe 14 :84-88](evidence/os-probe/2026-10-11-run-38
platform-facts.md@fc6109d1:164 — `NSTextInsertionIndicator` — 1.1.2 Colors (value column) — ✅ · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:177 — Window corner radius — 1.1.3 Geometry (value column) — ✅ multiple sources confirm through Sequoia; macOS Tahoe (26) uses variable radii per window style — 16pt (title-bar-only) confirmed by [macos-corner-fix](https://github.com/m4rkw/macos-corner-fix); toolbar window radii: 
platform-facts.md@fc6109d1:178 — Control corner radius — 1.1.3 Geometry (value column) — ❓ no AppKit source: WebKit's `baseBorderRadius = 5` ([`RenderThemeMac.mm:1351`](https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L1351), used a
platform-facts.md@fc6109d1:179 — Frame/border width — 1.1.3 Geometry (value column) — ❓ measured, no Apple docs · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:181 — Scrollbar width — 1.1.3 Geometry (value column) — ✅ Gecko [`ScrollbarDrawingCocoa.cpp`](https://searchfox.org/mozilla-central/source/widget/ScrollbarDrawingCocoa.cpp): overlay non-hovered thumb = 7px, hovered = 11px; Chromium [`native_theme_mac.mm`](https://github.com/c
platform-facts.md@fc6109d1:183 — Focus ring offset — 1.1.3 Geometry (value column) — ❓ measured, no Apple docs; WebKit [`RenderThemeCocoa.mm`](https://github.com/WebKit/WebKit/blob/main/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm) notes "macOS controls have never honored outline offset" — focus ri
platform-facts.md@fc6109d1:185 — Drop shadows — 1.1.3 Geometry (value column) — ✅ · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:199 — NSButton — 1.1.4 Widget Metrics (value column) — ❓ The legacy HIG 12px is inter-button *spacing*, not internal padding. Gecko [`nsNativeThemeCocoa.mm`](https://searchfox.org/mozilla-central/source/widget/cocoa/nsNativeThemeCocoa.mm) `pushButtonSettings` margins `IntMar
platform-facts.md@fc6109d1:209 — NSScroller — 1.1.4 Widget Metrics (value column) — ✅ Gecko [`ScrollbarDrawingCocoa.cpp`](https://searchfox.org/mozilla-central/source/widget/ScrollbarDrawingCocoa.cpp): overlay non-hovered thumb thickness = 7px (8px base − 1px overlay reduction), hovered = 11px; Chromium
platform-facts.md@fc6109d1:210 — NSTabView — 1.1.4 Widget Metrics (value column) — ❓ measured · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:211 — NSTabView — 1.1.4 Widget Metrics (value column) — ❓ measured · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:212 — NSMenuItem — 1.1.4 Widget Metrics (value column) — ❓ measured, plausible · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:213 — NSMenuItem — 1.1.4 Widget Metrics (value column) — ❓ measured; Chromium [`menu_config.cc`](https://chromium.googlesource.com/chromium/src/+/refs/heads/main/ui/views/controls/menu/menu_config.cc) corroborates `item_horizontal_border_padding = 12` · macOS 26: ❓ — no source
platform-facts.md@fc6109d1:214 — NSToolTipManager — 1.1.4 Widget Metrics (value column) — ❓ measured · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:216 — NSTableView — 1.1.4 Widget Metrics (value column) — ❓ measured · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:217 — NSToolbar — 1.1.4 Widget Metrics (value column) — ❓ measured; varies by config · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:218 — NSToolbar — 1.1.4 Widget Metrics (value column) — ✅ HIG: "8 pixels between toolbar controls" · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:226 — NSPopUpButton — 1.1.4 Widget Metrics (value column) — ❓ measured visible indicator; note: WebKit total right-padding is 26px (includes arrow + surrounding space); the OS probe's render (macOS 14.8.9) shows no boundary of an arrow area to measure ([render](evidence/os-probe/
platform-facts.md@fc6109d1:228 — NSSegmentedControl — 1.1.4 Widget Metrics (value column) — ❓ measured · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:229 — NSDisclosureButton — 1.1.4 Widget Metrics (value column) — ❓ measured visible triangle; Gecko `nsNativeThemeCocoa.mm`: cell=21×21px — visible triangle is a subset of the cell · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:238 — Between related controls — 1.1.5 Layout Spacing (value column) — ❓ oversimplified: HIG says 12px for regular push buttons, 8px for mini/icon · macOS 26: same, `NSStackView.spacing`'s default 8 ([docs](https://developer.apple.com/tutorials/data/documentation/appkit/nsstackview/spacing.
platform-facts.md@fc6109d1:239 — Between unrelated groups — 1.1.5 Layout Spacing (value column) — ❓ 20pt is documented as superview-edge margin, not specifically inter-group · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:240 — Label to its associated control — 1.1.5 Layout Spacing (value column) — ✅ HIG: 8px for regular, 6px small, 5px mini · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:242 — Compact spacing (toolbar items) — 1.1.5 Layout Spacing (value column) — ✅ HIG: "8 pixels between toolbar controls" · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:256 — Sidebar (small) — 1.1.6 Icon Sizes (value column) — ✅ Apple HIG Sidebars page (macOS, 2022 archived); metrics table removed from current HIG ~2024 · macOS 26: ❓ — no source gives its macOS 26 value: the current HIG gives no sidebar metrics ([HIG sidebars](https://develope
platform-facts.md@fc6109d1:257 — Sidebar (medium) — 1.1.6 Icon Sizes (value column) — ✅ same source; pre-Big Sur was 18pt (legacy CoreTypes.bundle sizes) · macOS 26: ❓ — no source gives its macOS 26 value (as the small row)
platform-facts.md@fc6109d1:258 — Sidebar (large) — 1.1.6 Icon Sizes (value column) — ✅ same source; pre-Big Sur was 32pt · macOS 26: ❓ — no source gives its macOS 26 value (as the small row)
platform-facts.md@fc6109d1:259 — Menu item — 1.1.6 Icon Sizes (value column) — ❓ inferred from system font size · macOS 26: ❓ — no source gives its macOS 26 value; on 27 NSMenu hides item images by default
platform-facts.md@fc6109d1:260 — Menu bar extra — 1.1.6 Icon Sizes (value column) — ❓ community best-practice (Bjango), not official · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:374 — `DwmGetColorizationColor` — 1.2.2 Colors (value column) — ❓ no source ties it to the title bar; #006fc4, alpha 0xe3, on `windows-11-arm`, where `ColorPrevalence` (accent on title bars) is 0 ([probe.toml:763](evidence/os-probe/2026-10-11-run-38129819806/os-probe-windows-11-arm/p
platform-facts.md@fc6109d1:402 — Disabled opacity — 1.2.3 Geometry (value column) — ❓ no global disabled opacity; WinUI3 uses per-control `*Disabled` color brushes; 0.55 was legacy Win8.x/UWP — current WinUI3 value is 0.3
platform-facts.md@fc6109d1:413 — Button — 1.2.4 Widget Metrics (value column) — ❓ no `ButtonMinHeight` resource; effective=14px text + 5+6 padding + 2 border = 27px; `ContentDialogButtonHeight=32` is dialog-specific
platform-facts.md@fc6109d1:416 — Button — 1.2.4 Widget Metrics (value column) — ❓ hardcoded `Margin="8,0,0,0"` in DropDownButton template; no named XAML resource; consistent with Fluent 2 `size80` spacing
platform-facts.md@fc6109d1:424 — Scrollbar (WinUI3) — 1.2.4 Widget Metrics (value column) — ❓ XAML template inline value; expands on pointer proximity
platform-facts.md@fc6109d1:425 — Scrollbar (WinUI3) — 1.2.4 Widget Metrics (value column) — ❓ 6 = `ScrollBarSize` 12 less two 3px halves of `ScrollBarThumbStrokeThickness` 6, a derivation no source states; the template's named resources are `ScrollBarSize` 12, `ScrollBarVerticalThumbMinWidth` 8 and `ScrollBarTh
platform-facts.md@fc6109d1:434 — Menu item — 1.2.4 Widget Metrics (value column) — ❓ Win32 formula (from [Wine `menu.c`](https://github.com/wine-mirror/wine/blob/master/dlls/user32/menu.c) / [ReactOS `menu.c`](https://github.com/nicknisi/reactos/blob/master/win32ss/user/ntuser/menu.c)): `max(text_heigh
platform-facts.md@fc6109d1:1057 — `shadow_color` — Windows — **(Fluent)** two-layer per elevation (from [Fluent 2 spec](https://fluent2.microsoft.design/elevation)): low L=14%/14% D=28%/14%; high L=24%/20% D=28%/20% (note: FluentUI React web tokens use different opacities). The sa
platform-facts.md@fc6109d1:1110 — `focus_ring_offset` — macOS — -1px **(measured)** (inset) · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1116 — `border.corner_radius` — macOS — 5px **(measured)** ❓ no AppKit source (§1.1.3) · macOS 26: 6 **(WebKit)**, the push button's and pop-up's (§1.1.3); the text field's 5
platform-facts.md@fc6109d1:1117 — `border.corner_radius_lg` — macOS — 10px **(measured)** · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1118 — `border.line_width` — macOS — 0.5px **(measured)** · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1121 — `border.shadow_enabled` — macOS — yes · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1137 — `small` — macOS — sidebar: 16–20pt · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1138 — `large` — macOS — **(none)** · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1139 — `dialog` — macOS — **(none)** · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1140 — `panel` — macOS — **(none)** · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1164 — `title_bar_font.style` — macOS — `+titleBarFontOfSize:` → Normal · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1167 — `inactive_title_bar_text_color` — macOS — **(none)** — system-managed · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1168 — `border.corner_radius` — macOS — 10px **(measured)** through Sequoia (compositor-controlled, not inherited — happens to equal `defaults.border.corner_radius_lg`); Tahoe (26): variable per window style, no public API (see appendix) · macOS 26: ❓ (the Tah
platform-facts.md@fc6109d1:1170 — `border.padding_horizontal` — macOS — **(none)** — use §2.20 layout margins · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1171 — `border.padding_vertical` — macOS — **(none)** — use §2.20 layout margins · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1182 — `min_width` — macOS — **(none)** — sizes to content · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1187 — `icon_text_gap` — macOS — 4 **(measured)** AppKit · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1192 — `hover_background` — macOS — **(measured)** subtle CoreUI highlight · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1194 — `checked_background` — macOS — **(none)** — AppKit publishes no colour for a push-on/push-off `NSButton` that is on; its bezel is its own · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1195 — `checked_text_color` — macOS — **(none)** — not published · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1196 — `active_background` — macOS — ❓ — AppKit publishes no fill for a pressed push button, and the OS probe renders it at rest only ([render](evidence/os-probe/2026-10-11-run-38129819806/os-probe-macos-14/renders/aqua/push_button-regular@2x.png)) · macOS 
platform-facts.md@fc6109d1:1197 — `active_text_color` — macOS — ❓ — not published, and not rendered by the OS probe · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1218 — `hover_border_color` — macOS — ❓ — AppKit publishes no hover colour for a text field's bezel, and the OS probe renders it at rest only ([render](evidence/os-probe/2026-10-11-run-38129819806/os-probe-macos-14/renders/aqua/text_field-regular@2x.png)) · 
platform-facts.md@fc6109d1:1219 — `focus_border_color` — macOS — ❓ for the bezel line: AppKit shows focus with the focus ring around the field, `keyboardFocusIndicatorColor`, "The color to use for the keyboard focus ring around controls" ([docs](https://developer.apple.com/tutorials/d
platform-facts.md@fc6109d1:1220 — `disabled_background` — macOS — ❓ — AppKit publishes no disabled field fill, and the OS probe renders no disabled text field (its disabled renders are the push button, checkbox, radio, switch and slider) · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1232 — `indicator_color` — macOS — white (#ffffff) · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1237 — `border.padding_horizontal` — macOS — **(none)** — checkmark fills indicator · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1238 — `border.padding_vertical` — macOS — **(none)** — checkmark fills indicator · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1244 — `hover_background` — macOS — ❓ — AppKit publishes no hover state for a checkbox, and the OS probe renders it at rest only ([render](evidence/os-probe/2026-10-11-run-38129819806/os-probe-macos-14/renders/aqua/checkbox-regular-off@2x.png)) · macOS 26:
platform-facts.md@fc6109d1:1245 — `disabled_background` — macOS — **(measured)** for the unchecked box, ❓ for the checked one. macOS 14–15: a disabled unchecked box, fill and edge, is the enabled one at half its alpha, #ffffff80 / #ffffff1b where the enabled box is #ffffff / #ffffff37 
platform-facts.md@fc6109d1:1253 — `background_color` — macOS — **(measured)** ≈ `defaults.background_color` (vibrancy) · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1253 — `background_color` — Windows — `COLOR_MENU` — Windows 10+: not supported (§1.2.2); #f0f0f0 on the three Windows probe images ([probe.toml:343](evidence/os-probe/2026-10-11-run-38129819806/os-probe-windows-11-arm/probe.toml#L343)). WinUI3 `MenuFlyoutPr
platform-facts.md@fc6109d1:1254 — `separator_color` — macOS — **(measured)** = `separatorColor` · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1258 — `font.style` — macOS — `+menuFontOfSize:` → Normal · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1259 — `font.color` — macOS — **(measured)** = `labelColor` · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1260 — `row_height` — macOS — NSMenuItem: 22 · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1260 — `row_height` — Windows — ❓ — WinUI3 `MenuFlyoutItem` has no MinHeight (`MenuFlyoutThemeMinHeight` 32 is the presenter's, [MenuFlyout_themeresources.xaml:256, :284](https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df75259
platform-facts.md@fc6109d1:1261 — `border.padding_horizontal` — macOS — NSMenuItem: 12 · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1262 — `border.padding_vertical` — macOS — 3 **(measured)** (22−16)/2 · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1263 — `border.color` — macOS — **(none)** — popup border from §2.16 · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1264 — `border.line_width` — macOS — **(none)** — popup border from §2.16 · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1265 — `border.corner_radius` — macOS — **(none)** — items are rectangular · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1266 — `border.shadow_enabled` — macOS — **(none)** — popup shadow from §2.16 · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1267 — `icon_text_gap` — macOS — 4 **(measured)** AppKit layout · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1268 — `icon_size` — macOS — 16 **(cross-project)** — no Apple API constant; Qt/Firefox/Ladybird/Godot all use 16×16 for NSMenuItem images · macOS 26: ❓ — no source gives its macOS 26 value; on 27 NSMenu hides item images by default
platform-facts.md@fc6109d1:1269 — `hover_background` — macOS — `selectedContentBackgroundColor` · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1277 — `background_color` — macOS — **(preset)** L #2c2c2e D #3a3a3c · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1281 — `font.style` — macOS — `+toolTipsFontOfSize:` → Normal · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1282 — `font.color` — macOS — **(preset)** #ffffff (both variants) · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1283 — `border.padding_horizontal` — macOS — NSToolTipManager: 4 · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1284 — `border.padding_vertical` — macOS — NSToolTipManager: 4 · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1285 — `max_width` — macOS — 300 **(measured)** macOS Sonoma · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1287 — `border.color` — macOS — **(measured)** subtle frame · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1289 — `border.shadow_enabled` — macOS — yes (system popup shadow) · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1290 — `show_delay` — macOS — ❓ — undocumented: neither `NSView.toolTip` ([docs](https://developer.apple.com/tutorials/data/documentation/appkit/nsview/tooltip.json)) nor the HIG's *Offering help* ("tooltips can appear when a person holds the pointer
platform-facts.md@fc6109d1:1304 — `track_color` — macOS — mode-dependent: overlay=transparent, legacy=light gray **(measured)** · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1305 — `thumb_color` — macOS — `#80808080` **(measured)** Sonoma · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1306 — `thumb_hover_color` — macOS — `#60606080` **(measured)** Sonoma · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1308 — `min_thumb_length` — macOS — 40 **(measured)** legacy mode · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1309 — `thumb_width` — macOS — mode-dependent: legacy=15 (fills groove), overlay=7 (idle) / 11 (hover) · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1310 — `overlay_mode` — Windows — `UISettings.AutoHideScrollBars` (Bool, the *Automatically hide scroll bars* setting, 10.0.18362+; it "returns the specified user setting, not whether a specific scroll bar is visible", [UISettings.AutoHideScrollBars](htt
platform-facts.md@fc6109d1:1311 — `thumb_active_color` — macOS — ❓ — NSScroller publishes no colour for a dragged knob; `selectedKnobColor`, "The system color used for the slider knob when it is selected", is deprecated since macOS 11 ("Use NSScroller instead", [docs](https://develope
platform-facts.md@fc6109d1:1324 — `thumb_hover_color` — macOS — ❓ — AppKit publishes no hover colour for a slider knob, and the OS probe renders it at rest only ([render](evidence/os-probe/2026-10-11-run-38129819806/os-probe-macos-14/renders/aqua/slider-regular@2x.png)); whether a la
platform-facts.md@fc6109d1:1336 — `min_width` — macOS — **(none)** — no minimum · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1339 — `border.color` — macOS — **(none)** — not read · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1340 — `border.line_width` — macOS — **(none)** — not read · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1350 — `min_width` — macOS — **(none)** — sizes to label · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1351 — `min_height` — macOS — NSTabView: 24 · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1352 — `border.padding_horizontal` — macOS — NSTabView: 12 · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1353 — `border.padding_vertical` — macOS — 4 **(measured)** (24−16)/2 · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1354 — `border.color` — macOS — **(none)** — CoreUI bezel is a multi-color composite, no single extractable color · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1355 — `border.line_width` — macOS — **(none)** — CoreUI bezel is a multi-stroke composite, no single line width · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1356 — `border.corner_radius` — macOS — **(none)** — CoreUI bezel corners baked into `.car` assets, no queryable constant · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1357 — `border.shadow_enabled` — macOS — no — bezel creates visual depth via edge shading, not a drop shadow · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1361 — `active_indicator_color` — macOS — **(none)** — not published (see `item_gap`): no selection line is described · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1362 — `active_indicator_width` — macOS — **(none)** — no line · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1363 — `active_indicator_side` — macOS — **(none)** — no line · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1364 — `hover_background` — macOS — ❓ — AppKit publishes no hover colour for a tab, and the OS probe renders no tab view · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1365 — `hover_text_color` — macOS — ❓ — not published · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1366 — `active_outward_corner_radius` — macOS — ❓ — AppKit publishes no tab shape: the selected tab is a CoreUI bezel (see `border.corner_radius`) · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1377 — `border.corner_radius` — macOS — **(none)** — rectangular pane · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1378 — `border.shadow_enabled` — macOS — **(none)** — no shadow · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1379 — `hover_background` — macOS — ❓ — no source: AppKit publishes no hover colour for a sidebar item (NSColor has none) and no Apple guidance describes one (HIG *Lists and tables*, *Sidebars*, read 2026-10-26), and the OS probe renders none hovered: its 
platform-facts.md@fc6109d1:1380 — `border.padding_horizontal` — macOS — ❓ — AppKit publishes no inset between a sidebar and its rows, and the OS probe's sidebar is an empty `NSVisualEffectView` of the sidebar material, holding no rows to measure an inset from ([probe 14 :3247](evidence/os-pr
platform-facts.md@fc6109d1:1380 — `border.padding_horizontal` — KDE — ❓ — the sidebar is named (`GlobalDrawer`, `background_color` above), but its padding is not stated: the drawer pads itself by the window's safe-area margins alone, a device inset rather than the style's ([controls/Global
platform-facts.md@fc6109d1:1381 — `border.padding_vertical` — macOS — ❓ — as across · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1381 — `border.padding_vertical` — Windows — ❓ — no one value: above the item list the pane stacks a top padding, a toggle-button row and a search area (`PaneContentGrid`, NavigationView.xaml:291-350); each item's fill sits 2 inside its slot above and below (`Navig
platform-facts.md@fc6109d1:1381 — `border.padding_vertical` — KDE — ❓ — as across
platform-facts.md@fc6109d1:1390 — `bar_height` — macOS — NSToolbar: 38 · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1391 — `item_gap` — macOS — AppKit: 8 · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1392 — `border.padding_horizontal` — macOS — 8 **(measured)** NSToolbar · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1393 — `border.padding_vertical` — macOS — 0 · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1396 — `border.color` — macOS — ❓ — the toolbar's lower edge is the window's title-bar separator, `NSWindow.titlebarSeparatorStyle`, whose default is `.automatic`, "A style indicating that the system determines the type of separator" (macOS 11+, [docs]
platform-facts.md@fc6109d1:1397 — `border.corner_radius` — macOS — ❓ — not published · macOS 26: ❓ — the HIG has toolbar controls' "corner radii that are concentric with bar corners" and gives the bar's none ([HIG toolbars](https://developer.apple.com/tutorials/data/design/human-interfa
platform-facts.md@fc6109d1:1398 — `border.line_width` — macOS — ❓ — the same title-bar separator: a `.line`, a `.shadow` or `.none` as the system chooses under `.automatic` (docs, as `border.color`); no width is published · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1399 — `border.shadow_enabled` — macOS — ❓ — `NSTitlebarSeparatorStyle.shadow`, "A style indicating that the title bar separator is a shadow", is one of the styles `.automatic` chooses from (docs, as `border.color`) · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1412 — `border.corner_radius` — macOS — **(none)** — rectangular bar · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1413 — `border.shadow_enabled` — macOS — **(none)** — no shadow · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1424 — `alternate_row_background` — Windows — **(preset)** L #f9f9f9 D #262626 ❓ — no WinUI3 source
platform-facts.md@fc6109d1:1433 — `border.padding_horizontal` — macOS — NSTableView: 4 · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1434 — `border.padding_vertical` — macOS — 4 **(measured)** (24−16)/2 · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1435 — `hover_background` — macOS — ❓ — no source: AppKit publishes no hover colour for a table row (NSColor has none) and no Apple guidance describes one (HIG *Lists and tables*, *Sidebars*, read 2026-10-26), and the OS probe renders none hovered: its key
platform-facts.md@fc6109d1:1436 — `hover_text_color` — macOS — ❓ — AppKit publishes no hover colour for a table row · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1437 — `disabled_text_color` — macOS — ← `defaults.disabled_text_color`: `disabledControlTextColor` (§2.3 `disabled_text_color`); a disabled cell's text does change, "The text of disabled cells is gray" ([NSCell.isEnabled](https://developer.apple.com/tutorial
platform-facts.md@fc6109d1:1438 — `border.color` — macOS — ❓ — a table's frame is its enclosing NSScrollView's, `borderType`, "A value that specifies the appearance of the scroll view's border" ([docs](https://developer.apple.com/tutorials/data/documentation/appkit/nsscrollview/
platform-facts.md@fc6109d1:1439 — `border.corner_radius` — macOS — ❓ — not published (`border.color`) · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1440 — `border.line_width` — macOS — ❓ — not published (`border.color`) · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1441 — `border.shadow_enabled` — macOS — ❓ — not published · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1457 — `border.color` — macOS — ❓ — AppKit publishes no edge colour for a popover or a pop-up menu, and the OS probe renders neither · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1458 — `border.line_width` — macOS — ❓ — as `border.color` · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1460 — `border.shadow_enabled` — macOS — yes (system popup shadow) · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1461 — `border.padding_horizontal` — macOS — **(none)** — content provides own padding · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1462 — `border.padding_vertical` — macOS — **(none)** — content provides own padding · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1470 — `hover_color` — macOS — ❓ — `NSSplitView.dividerColor` (`divider_color` above) has no hover variant, and AppKit publishes none · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1504 — `container_margin` — macOS HIG — **(none)** — not specified · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1506 — `section_gap` — macOS HIG — 20 **(HIG)** · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1527 — `hover_checked_background` — macOS — ❓ — AppKit publishes no hover colour for a switch, and the OS probe renders it at rest only ([render](evidence/os-probe/2026-10-11-run-38129819806/os-probe-macos-14/renders/aqua/switch-regular-on@2x.png)) · macOS 26: ❓ —
platform-facts.md@fc6109d1:1528 — `hover_unchecked_background` — macOS — ❓ — as `hover_checked_background` · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1532 — `unchecked_track_border_color` — macOS — ❓ — AppKit's `NSSwitch` and the HIG's *Toggles* name no outline ([docs](https://developer.apple.com/tutorials/data/documentation/appkit/nsswitch.json), [HIG toggles](https://developer.apple.com/tutorials/data/design/huma
platform-facts.md@fc6109d1:1533 — `hover_unchecked_track_border_color` — macOS — ❓ — AppKit publishes no switch outline colours, and the OS probe renders the rest state only · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1534 — `disabled_unchecked_track_border_color` — macOS — ❓ — as `unchecked_track_border_color`: the macOS 14–15 disabled off render shows the same soft rim, the fill's 7/255 rising to 12/255 (L) and 20/255 (D) near the edge, where in the same column the enabled off track's 26/
platform-facts.md@fc6109d1:1535 — `checked_track_border_color` — macOS — ❓ — as `unchecked_track_border_color`: the macOS 14–15 on render shows the same kind of soft rim, alpha 88-91/255 along the straight edge over the fill's 64/255 ([D](evidence/os-probe/2026-10-11-run-38129819806/os-probe-
platform-facts.md@fc6109d1:1536 — `disabled_checked_track_border_color` — macOS — ❓ — the renders are not key and draw a disabled on track grey, not in the accent: macOS 14–15 #00000007 / #ffffff07 with the off track's kind of rim, 13/255 / 23/255 ([L](evidence/os-probe/2026-10-26-run-40348854007/os-p
platform-facts.md@fc6109d1:1537 — `track_border_line_width` — macOS — ❓ — as `unchecked_track_border_color` · macOS 26: off: **(measured)** 0 — no edge (`unchecked_track_border_color`); on: ❓ — the on render is the inactive window's (`checked_track_border_color`)
platform-facts.md@fc6109d1:1538 — `thumb_border_color` — macOS — ❓ — AppKit's `NSSwitch` and the HIG's *Toggles* name no outline ([docs](https://developer.apple.com/tutorials/data/documentation/appkit/nsswitch.json), [HIG toggles](https://developer.apple.com/tutorials/data/design/huma
platform-facts.md@fc6109d1:1539 — `hover_thumb_border_color` — macOS — ❓ — AppKit publishes no switch outline colours, and the OS probe renders the rest state only · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1540 — `disabled_thumb_border_color` — macOS — ❓ — as `thumb_border_color`: the macOS 14–15 disabled render shows the same grey pixel between the knob and the track at about half its alpha (L: 113,113,113 at 34/255 above the knob; 80,80,80 at 48/255, then black at 34
platform-facts.md@fc6109d1:1541 — `thumb_border_line_width` — macOS — ❓ — as `thumb_border_color` · macOS 26: **(measured)** 0 — no ring (`thumb_border_color`)
platform-facts.md@fc6109d1:1551 — `border.shadow_enabled` — macOS — yes (sheet overlay) · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1552 — `min_width` — macOS — **(none)** — AppKit-managed · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1553 — `max_width` — macOS — **(none)** — AppKit-managed · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1554 — `min_height` — macOS — **(none)** — AppKit-managed · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1555 — `max_height` — macOS — **(none)** — AppKit-managed · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1556 — `border.padding_horizontal` — macOS — ~20px **(measured)** · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1557 — `border.padding_vertical` — macOS — ~20px **(measured)** · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1558 — `button_gap` — macOS — ~12px **(measured)** · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1561 — `title_font.size` — macOS — ✅ 13pt **(HIG)** — "emphasized system font" = `boldSystemFontOfSize:0` → `systemFontSize` (13pt); corroborated by Cocotron `NSAlert.m` · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1562 — `title_font.weight` — macOS — ✅ Bold (700) **(HIG)** — "emphasized system font" = `boldSystemFontOfSize:0`; corroborated by Cocotron `NSAlert.m` · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1563 — `title_font.style` — macOS — Normal · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1568 — `icon_size` — macOS — 64px (app icon) · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1569 — `entrance` — macOS — ❓ — `NSWindow.AnimationBehavior` names styles only ([docs](https://developer.apple.com/tutorials/data/documentation/appkit/nswindow/animationbehavior-swift.enum.json)); the HIG's *Sheets* states no motion for macOS ([HIG
platform-facts.md@fc6109d1:1602 — `fill_color` — macOS — black (L) / white (D) fins at a graded alpha, the leading one 82/255 **(measured)** ❓ — the macOS 14.8.9 and 15.7.9 probe renders ([L](evidence/os-probe/2026-10-11-run-38129819806/os-probe-macos-14/renders/aqua/progress_
platform-facts.md@fc6109d1:1617 — `min_width` — macOS — **(none)** — sizes to content · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1619 — `arrow_icon_size` — macOS — macOS 14–15: the chevron is 12 × 20 px at 2× (6 × 10) **(measured)** ([render](evidence/os-probe/2026-10-11-run-38129819806/os-probe-macos-14/renders/aqua/pop_up_button-regular@2x.png)) · macOS 26: 14 × 21 px at 2× (7 × 
platform-facts.md@fc6109d1:1620 — `arrow_area_width` — macOS — ~16–18px **(measured)** ❓ (the macOS 14.8.9 probe render shows no arrow-area boundary to measure, §1.1.4) · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1625 — `hover_background` — macOS — ❓ — AppKit publishes no hover colour for a pop-up button, and the OS probe renders it at rest only ([render](evidence/os-probe/2026-10-11-run-38129819806/os-probe-macos-14/renders/aqua/pop_up_button-regular@2x.png)) · ma
platform-facts.md@fc6109d1:1626 — `disabled_background` — macOS — ❓ — AppKit publishes no disabled pop-up fill, and the OS probe renders no disabled pop-up button (its disabled renders are the push button, checkbox, radio, switch and slider) · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1633 — `background_color` — macOS — NSSegmentedControl bg · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1635 — `font.color` — macOS — `controlTextColor` · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1639 — `separator_width` — macOS — 1px · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1640 — `border.padding_horizontal` — macOS — ~8–10px **(measured)** · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1642 — `border.color` — macOS — NSSegmentedControl border · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1645 — `border.padding_vertical` — macOS — ~3px **(measured)** · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1647 — `hover_background` — macOS — ❓ — AppKit publishes no hover colour for a segment, and the OS probe renders the control at rest only ([render](evidence/os-probe/2026-10-11-run-38129819806/os-probe-macos-14/renders/aqua/segmented_control-regular@2x.png
platform-facts.md@fc6109d1:1657 — `background_color` — macOS — ❓ — the macOS counterpart is the box, `NSBox` ("A stylized rectangular box with an optional title", [docs](https://developer.apple.com/tutorials/data/documentation/appkit/nsbox.json)) or SwiftUI `GroupBox`, which "By def
platform-facts.md@fc6109d1:1658 — `border.color` — macOS — ❓ — the default box's look is not published (`background_color`) · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1659 — `border.line_width` — macOS — ❓ — the default box's look is not published (`background_color`) · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1660 — `border.corner_radius` — macOS — ❓ — the default box's look is not published (`background_color`) · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1661 — `border.shadow_enabled` — macOS — ❓ — the default box's look is not published (`background_color`) · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1675 — `header_height` — macOS — **(none)** — content-sized · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1676 — `arrow_icon_size` — macOS — ~13px **(measured)** · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1677 — `border.padding_horizontal` — macOS — **(none)** — app-defined · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1678 — `border.padding_vertical` — macOS — **(none)** — app-defined · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1679 — `border.corner_radius` — macOS — **(none)** · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1680 — `border.color` — macOS — **(none)** · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1681 — `border.line_width` — macOS — **(none)** · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1682 — `border.shadow_enabled` — macOS — **(none)** — no shadow · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1684 — `arrow_gap` — macOS — **(none)** — not documented · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1685 — `content_indent` — macOS — **(none)** — not documented · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1686 — `frame_enabled` — macOS — **(none)** — not documented · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1687 — `hover_background` — macOS — ❓ — AppKit publishes no hover colour for a disclosure control · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1688 — `arrow_color` — macOS — ❓ — the disclosure button, `NSButton.BezelStyle.disclosure`, "A bezel style button for use with a disclosure triangle" ([docs](https://developer.apple.com/tutorials/data/documentation/appkit/nsbutton/bezelstyle-swift.enu
platform-facts.md@fc6109d1:1695 — `visited_text_color` — macOS — **(none)** — same as link · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1696 — `underline_enabled` — macOS — yes · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1697 — `background_color` — macOS — **(none)** — inline · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1698 — `hover_background` — macOS — **(none)** · macOS 26: ❓ — no source gives its macOS 26 value
platform-facts.md@fc6109d1:1699 — `hover_text_color` — macOS — ❓ — `linkColor` has no hover variant, and AppKit publishes none · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1700 — `active_text_color` — macOS — ❓ — not published · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1701 — `disabled_text_color` — macOS — ❓ — not published · macOS 26: ❓ — the same
platform-facts.md@fc6109d1:1738 — `menuBarFontOfSize:` weight — macOS (value column) — ❓ API exists ([Apple docs](https://developer.apple.com/documentation/appkit/nsfont/menubarfont(ofsize:))) but weight is not documented; no evidence of SemiBold — likely Regular like other font methods; needs verification
platform-facts.md@fc6109d1:1749 — Control corner radius = 5px — macOS (value column) — ❓ WebKit [`RenderThemeMac.mm:1351`](https://github.com/WebKit/WebKit/blob/de320ba3ff90229f983988d42d4b0f83315459da/Source/WebCore/rendering/mac/RenderThemeMac.mm#L1351): `const int baseBorderRadius = 5`, for CSS-styled p
platform-facts.md@fc6109d1:1757 — NSButton (checkbox) indicator = 14px — macOS (value column) — ❓ WebKit [`ToggleButtonMac.mm`](https://github.com/WebKit/WebKit/blob/main/Source/WebCore/platform/graphics/mac/controls/ToggleButtonMac.mm): regular={14,14}; Gecko [`nsNativeThemeCocoa.mm`](https://github.com/mozilla/ge
platform-facts.md@fc6109d1:1760 — Other geometry/widget metrics (0.5px, padding values, etc.) — macOS (value column) — ❓ Apple does not publish these. All are measured values. Legacy scrollbar width = ✅ 15px, `scrollerWidth(for: .regular, scrollerStyle: .legacy)` on macOS 14.8.9 ([probe 14 :481](evidence/os-probe/2026-10-11-run-381298198
platform-facts.md@fc6109d1:1763 — NSButton horizontal padding ~8px — macOS (value column) — ❓ WebKit [`RenderThemeMac.mm`](https://github.com/WebKit/WebKit/blob/main/Source/WebCore/rendering/mac/RenderThemeMac.mm) `controlPadding(PushButton)` = 8px horizontal; comment says "AppKit wants to use 11px for mini but
platform-facts.md@fc6109d1:1775 — `+toolTipsFontOfSize:` default size — macOS (value column) — ❓ [Apple API docs](https://developer.apple.com/documentation/appkit/nsfont/1527704-tooltipsfontofsize) do not state default size; [Leopard-era Apple HIG](https://leopard-adc.pepas.com/documentation/UserExperience/Concept
platform-facts.md@fc6109d1:1776 — NSProgressIndicator visual bar vs control frame — macOS (value column) — ❓ bar height 6px is the visual track; `NSProgressIndicatorPreferredThickness`=14px deprecated (Apple: "do not accurately represent the geometry"); WebKit [`ProgressBarMac.mm`](https://github.com/WebKit/WebKit/blob/main/S

## Seed follow-up sources

The abbreviations the platform files' *Seed follow-up* sections cite (2026-10-28).

### Code editor and rich text (G12, G27)

| Short | Source |
|---|---|
| `gsv/F:L` | GtkSourceView tag **5.22.0** (commit `c7b52cf9f9e42035b133902a1a9b18faae00b1d0`), path F from the repository root |
| `ksh/F:L` | KSyntaxHighlighting tag **v6.31.0** (KF 6.31.0, commit `6d8ab7945958e7354295db7f399cce06f7de7571`) |
| `kte/F:L` | KTextEditor tag **v6.31.0** (KF 6.31.0, commit `ea14509c132f2c016d55b38f7a1ec24cc6f9e7d2`) |
| `qt/F:L` | qtbase tag **v6.12.0** (the tag `platform-facts.md` cites), `src/gui/text/F` |
| `A/…`, `G/…` | libadwaita 1.10.0 and GTK 4.24.1, as in the GNOME file |
| `kirigami@eecf183a`, `qqc2@2efe38d5` | the KDE file's Kirigami pin; qqc2-desktop-style `2efe38d55ebcfa97599643daada20c157fa1a8a5` (tag `v6.31.0`, the KDE file's pin) |
| `labs/F:L` | Windows Community Toolkit **Labs**, `CommunityToolkit/Labs-Windows` at `d620e8b5eb39a3f41f3797101a94d9bda6ba2190` (tag `release/weekly/251217`), path `components/MarkdownTextBlock/src/F`. Labs is the toolkit's experimental channel: `MarkdownTextBlock` is **not** in the CTK release the Windows file pins (the `components/` list of `CommunityToolkit/Windows` at `a6b4dc45`, tag v8.2.251219, read through the GitHub contents API, has no MarkdownTextBlock) |
| `MUX` | microsoft-ui-xaml at `ba3a8d59` (tag `winui3/release/2.5.1`), paths under `src/`, as in the Windows file |
| Apple `x/y` | `https://developer.apple.com/documentation/x/y`, fetched as JSON (`…/tutorials/data/documentation/x/y.json`) on 2026-10-28 |
| WWDC26-370 | `https://developer.apple.com/videos/play/wwdc2026/370/` ("Elevate your app's text experience with TextKit"), read 2026-10-28 |
| `compose/`, `latest/` | androidx `e4d933c98df7baa7ea2b83eea6e086915961e0ff` and material-web `b4de401e…` (tag `v2.5.0`) `tokens/versions/latest/sass/`, as in the Material file |

SRC: **R** read at run time, **P** published, **M** measure, **V** to verify. CAPS: the IDs of
`taxonomy.md`. For macOS and iOS a **Gen** column follows *Element*.

**The new controls this file adds to §4.2:** `code editor` (GNOME: GtkSourceView; KDE:
KTextEditor) and `rich text view` (Markdown or HTML rendered by a platform component: KDE/Qt
QTextDocument, Windows Labs MarkdownTextBlock, Apple SwiftUI `Text` and the AppKit/UIKit
attributed-string importers). Both are new rows of the §4.2 table; the other cells are "–" with
the reason given in each platform's section below.

**"A window's own label" (G27).** The seed is iced's application style: the text colour iced gives
every text that states none comes from `defaults.text_color`, because "the model states no label of
the window's own" (`connectors/native-theme-iced/src/styles.rs:2245-2247`). It is a **colour, not a
font**: the platforms that draw window text and view text in different colours are already rows
(GNOME GN-1.274 `--view-fg-color` vs window fg; KDE Q3.1, the Window and View colour sets; macOS
MAC-DF05†, `textColor` that no reader reads, beside `labelColor`). Windows, Material and iOS draw
both with one colour (`TextFillColorPrimary`, `onSurface`, `label`): no row. No further row.

### Drop zones and dialog placement (G19, G24)

| Short | Source |
|---|---|
| `qt/F:L` | qtbase tag **v6.12.0** (the tag the KDE file already cites), `src/widgets/…/F`, fetched from `raw.githubusercontent.com/qt/qtbase/v6.12.0/` |
| `st:L` | `kstyle/breezestyle.cpp`, KDE/breeze @ `c97883a8` (tag `v6.8.2`, as in the KDE file) |
| `kwin/F:L` | KDE/kwin @ `f130ab33c8e6f3b9d728aade87836086ae9a8ed4` (tag `v6.8.2`) (as in the KDE file), `src/F` |
| `kirigami`, `qqc2` | as in the KDE file (`eecf183a`, `2efe38d5`) |
| `kate/F:L` | KDE/kate `master` @ `2b6465a886b9d3bfc6eab25065c14ffe025e6ac3` (tag `v26.08.2`) (2026-10-27), `apps/lib/F` |
| `konsole/F:L` | KDE/konsole `master` @ `c21993d0b73b6fe5d4c52da50b25370b40389e4e` (tag `v26.08.2`), `src/F` |
| `kdenlive/F:L` | KDE/kdenlive `master` @ `ef54398259f2af1d29095a798cab48ca4c52f141` (tag `v26.08.2`) |
| `kddw/F:L` | KDAB/KDDockWidgets tag **v2.5.1** (`674f2845`), `src/F` |
| `A/…`, `G/…` | libadwaita 1.10.0, GTK 4.24.1 (as in the GNOME file) |
| `M/F:L` | mutter tag **51.1** (commit `9134e3a5869ce232bf95b77519005865747ba603`, the mutter of the gnome-shell 51.1 the GNOME file cites) |
| `GS-data/F:L` | gnome-shell tag **51.1** (`33da0685`), `data/F` |
| `LP/F:L` | GNOME/libpanel tag **1.10.4** (commit `ed3e78f3c09d7daed05b2450313dbe138c8f3c87`), `src/F` |
| `MUX/…` | microsoft-ui-xaml @ `ba3a8d59` (tag `winui3/release/2.5.1`, as in the Windows file), `src/controls/dev/…` |
| Learn | learn.microsoft.com, read 2026-10-28, with the page's `updated_at` |
| Apple | `developer.apple.com/tutorials/data/documentation/<path>.json`, read 2026-10-28; HIG pages as the macOS/iOS files read them (2026-10-27) |
| `mw/`, `compose/` | as in the Material file (material-web `b4de401e`, androidx `e4d933c9`) |
| `ax/F:L` | androidx `e4d933c98df7baa7ea2b83eea6e086915961e0ff`, the full path given in the row |

Columns: section · model element · Element · What is missing · Source · Nearest model field · CAPS · SRC.
For macOS and iOS the "What is missing" cell ends with **Gen:** (whether the generations differ).

## Fonts and mirroring sources

The abbreviations the platform files' *Fonts and mirroring* sections cite (2026-10-28).

| Abbrev. | Source |
|---|---|
| `CT` | Apple Core Text / AppKit / UIKit documentation JSON, `https://developer.apple.com/tutorials/data/documentation/<path>.json`, read 2026-10-28: `coretext/ctfontcopydefaultcascadelistforlanguages(_:_:)` (iOS 6.0, macOS 10.8: "An ordered list of CTFontDescriptors for font fallback according to the given language preferences … the system follows this list to pick a fallback font"), `coretext/kctfontcascadelistattribute` (iOS 3.2, macOS 10.5), `coretext/ctfontcreateforstring(_:_:_:)` (iOS 3.2, macOS 10.5), `coretext/ctfontcreateforstringwithlanguage(_:_:_:_:)` (iOS 7.0, macOS 10.9), `appkit/nsfontdescriptor/attributename/cascadelist` and `uikit/uifontdescriptor/attributename/cascadelist` (iOS 7.0; both: "The default value is the system default cascading list for user's locale"), `uikit/nsadaptiveimageglyph` (iOS 18.0) and `appkit/nsadaptiveimageglyph` (macOS 15.0) |
| `ASF` | Apple, *System Fonts*, `https://developer.apple.com/fonts/system-fonts/`, read 2026-10-28: "Apple Color Emoji", version 20.0d1e3, marked "system font" for iOS, macOS, tvOS, watchOS |
| `Learn` | learn.microsoft.com, read 2026-10-28, with the page's `updated_at`: *IDWriteFactory2::GetSystemFontFallback* (2024-02-22); *IDWriteFontFallback::MapCharacters* (2024-02-22; "The text source implementation holds the text and locale"; Windows 8.1); *Typography in Windows* `windows/apps/design/signature-experiences/typography` (2026-07-14; "Fonts for non-Latin languages" table; "Segoe UI Emoji — User-interface font for Emoji"; "Segoe UI Symbol — Fallback font for symbols"); *Segoe UI Emoji font family* `typography/font-list/segoe-ui-emoji` (2021-12-06; file `Seguiemj.ttf`, script tags `Zsye`, `Zsym`); *TextBlock.IsColorFontEnabled* (Windows App SDK, 2026-07-28; default `true`; "Some glyphs in multi-color fonts have different layout metrics when rendered in color"); *LanguageFontGroup Class* `uwp/api/windows.globalization.fonts.languagefontgroup` (2025-11-21; "Supplies a set of font recommendations for a given language or script for various usage cases"); *Fonts* `globalization/fonts-layout/fonts` (2023-11-20; font linking in GDI, registry key `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\FontLink\SystemLink`) |
| `fc/` | fontconfig tag **2.18.3** (the version `native-theme/src/fonts.rs:290` cites), `conf.d/`: `45-generic.conf`, `60-generic.conf`, `65-nonlatin.conf` |
| `qt/` | qtbase tag **v6.12.0** (the KDE file's Qt ref), `src/gui/text/`: `unix/qfontconfigdatabase.cpp`, `qfontdatabase.cpp` |
| `P/` | pango tag **1.58.2** (commit `d360f14004e4a4824f2352f6e2722016f9a51dd6`), `pango/`; GTK 4.24.1 requires pango ≥ 1.58 (`gtk/meson.build:17-22` at tag 4.24.1) |
| `G/`, `A/w/`, `S/` | as the GNOME file: GTK 4.24.1 `gtk/`, libadwaita 1.10.0 `src/stylesheet/widgets/`, gsettings-desktop-schemas 51.0 |
| `st`, `kirigami@eecf183a` | as the KDE file: KDE/breeze @ `c97883a8` `kstyle/breezestyle.cpp`; KDE/kirigami @ `eecf183a` |
| `aosp/` | AOSP `platform/frameworks/base` @ `94b4c163b7dfe5ce3607f7bb8456f9573f7de57d` (tag `android-17.0.0_r1`) (read 2026-10-28 at `refs/heads/main`, re-pinned 2026-10-30), `data/fonts/`: `fonts.xml`, `fallback_order.json` |
| `ndk` | developer.android.com/ndk/reference/group/font (updated 2026-08-27): `AFontMatcher_match`, `AFontMatcher_setLocales`, `ASystemFontIterator_open`, "Available since API level 29" |
| `c-emoji` | developer.android.com/develop/ui/compose/text/emoji (updated 2026-10-22) |
| `a-ref` | developer.android.com reference, read 2026-10-28: `android.content.res.Configuration#getLayoutDirection()`; `android.graphics.drawable.Drawable#isAutoMirrored()` |
| `CAM` | developer.android.com reference `androidx.compose.material.icons.Icons.AutoMirrored` (artifact `material-icons-core`, added in 1.6.0), read 2026-10-28 |
| `GFI` | Google Fonts, *Material Icons Guide*, `developers.google.com/fonts/docs/material_icons`, sections "Icons in RTL" and "Which icons should be mirrored for RTL?" (last updated 2024-07-23) |
| `mw/`, `latest/`, `compose/` | as the Material file: material-web `b4de401eb665ec63474f39319a4ba8f2145974cc` (tag `v2.5.0`; repository root; `latest/` = `tokens/versions/latest/sass/`); androidx `e4d933c98df7baa7ea2b83eea6e086915961e0ff`, `compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/` |
| `m3.io` | m3.material.io *Bidirectionality & RTL* (`foundations/layout/understanding-layout/bidirectionality-rtl`) and the m2 page (`m2.material.io/design/usability/bidirectionality.html`) render with JavaScript only and returned no text: **to verify**, as the Material file says of every m3.io cite |

The probe methods named in the rows are the gap map's §3 probes; "the sample set" means one string
each of emoji (with and without U+FE0F), Han text under the `ja`, `zh-Hans`, `zh-Hant` and `ko`
languages, Arabic, Hebrew, Devanagari and Thai, laid out in the UI font, recording the font of each run.
