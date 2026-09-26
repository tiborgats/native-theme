# The mapping

<!-- Generated from `mapping.toml` by `mapping_doc::render`; regenerate with `cargo test -p native-theme-egui --lib -- --ignored regenerate_mapping_md`. Do not edit. -->

One row per native leaf (spec §5, §13.1). A sink is a base-style field; «scope» or «scope:variant» a role cell's field; \[surface\] a `Frame`'s.

## Totals

| group | leaves | DIRECT | SCOPED | DERIVED | UNMAPPABLE |
|---|---:|---:|---:|---:|---:|
| Foundation (`defaults`, `text_scale`, `layout`) | 66 | 18 | 1 | 37 | 10 |
| Surfaces (window, dialog, popover, card, tooltip, menu) | 108 | 8 | 55 | 31 | 14 |
| Buttons (button, link, switch, checkbox, segmented control) | 99 | 12 | 44 | 29 | 14 |
| Inputs (input, combo box, list) | 78 | 4 | 35 | 28 | 11 |
| Indicators (scrollbar, slider, progress bar, splitter, separator, spinner) | 40 | 6 | 17 | 8 | 9 |
| Chrome (tab, sidebar, toolbar, status bar, expander) | 91 | 0 | 53 | 22 | 16 |
| **TOTAL** | **482** | **48** | **205** | **155** | **74** |

UNMAPPABLE sub-tags (§2): `egui-limited` 54, `source-side gap` 15, `widgets-crate` 5.

## Rows

### Foundation (`defaults`, `text_scale`, `layout`)

| leaf | verdict | sinks, or the test of its route | note |
|---|---|---|---|
| `defaults.accent_color` | UNMAPPABLE |  | `egui-limited`: egui: an accent field in Visuals |
| `defaults.accent_text_color` | UNMAPPABLE |  | `egui-limited`: egui: an accent field in Visuals |
| `defaults.background_color` | DIRECT | `visuals.panel_fill`; `visuals.widgets.noninteractive.bg_fill`; `visuals.widgets.noninteractive.weak_bg_fill` |  |
| `defaults.border.color` | DIRECT | `visuals.widgets.noninteractive.bg_stroke.color`; `visuals.window_stroke.color` |  |
| `defaults.border.corner_radius` | DIRECT | `visuals.widgets.noninteractive.corner_radius`; «scrollbar» `visuals.widgets.inactive.corner_radius`; «scrollbar» `visuals.widgets.hovered.corner_radius`; «scrollbar» `visuals.widgets.active.corner_radius`; «slider» `visuals.widgets.inactive.corner_radius` (when: the radius is below half the slider rail height, §6.8) |  |
| `defaults.border.corner_radius_lg` | DIRECT | `visuals.menu_corner_radius` |  |
| `defaults.border.line_width` | DIRECT | `visuals.widgets.noninteractive.bg_stroke.width`; `visuals.window_stroke.width` |  |
| `defaults.border.opacity` | DERIVED | `visuals.widgets.noninteractive.bg_stroke.color`; `visuals.window_stroke.color`; «checkbox» `visuals.widgets.noninteractive.bg_stroke.color`; «checkbox» `visuals.widgets.inactive.bg_stroke.color`; «checkbox» `visuals.widgets.hovered.bg_stroke.color`; «checkbox» `visuals.widgets.active.bg_stroke.color`; «checkbox» `visuals.widgets.open.bg_stroke.color`; «input» `visuals.widgets.hovered.bg_stroke.color`; «input» `visuals.widgets.active.bg_stroke.color`; \[window\] `stroke.color`; \[dialog\] `stroke.color`; \[popover\] `stroke.color`; \[card\] `stroke.color` (when: card.border.color is not fully transparent, §6.13); \[tooltip\] `stroke.color`; «menu» `visuals.widgets.inactive.bg_stroke.color` (when: menu.border.color is not fully transparent, §6.13); «menu» `visuals.widgets.hovered.bg_stroke.color` (when: menu.border.color is not fully transparent, §6.13); «menu» `visuals.widgets.active.bg_stroke.color` (when: menu.border.color is not fully transparent, §6.13); «menu» `visuals.widgets.open.bg_stroke.color` (when: menu.border.color is not fully transparent, §6.13); `visuals.widgets.inactive.bg_stroke.color`; `visuals.widgets.hovered.bg_stroke.color`; `visuals.widgets.active.bg_stroke.color`; `visuals.widgets.open.bg_stroke.color`; «button» `visuals.widgets.noninteractive.bg_stroke.color`; «button» `visuals.widgets.inactive.bg_stroke.color`; «button» `visuals.widgets.hovered.bg_stroke.color`; «button» `visuals.widgets.active.bg_stroke.color`; «button» `visuals.widgets.open.bg_stroke.color`; «checkbox:selected» `visuals.widgets.noninteractive.bg_stroke.color`; «checkbox:selected» `visuals.widgets.inactive.bg_stroke.color`; «checkbox:selected» `visuals.widgets.hovered.bg_stroke.color`; «checkbox:selected» `visuals.widgets.active.bg_stroke.color`; «checkbox:selected» `visuals.widgets.open.bg_stroke.color`; «segmented_control» `visuals.widgets.noninteractive.bg_stroke.color`; «segmented_control» `visuals.widgets.inactive.bg_stroke.color`; «segmented_control» `visuals.widgets.hovered.bg_stroke.color`; «segmented_control» `visuals.widgets.active.bg_stroke.color`; «segmented_control» `visuals.widgets.open.bg_stroke.color`; «input» `visuals.widgets.noninteractive.bg_stroke.color`; «input» `visuals.widgets.inactive.bg_stroke.color`; «input» `visuals.widgets.open.bg_stroke.color`; «combo_box» `visuals.widgets.noninteractive.bg_stroke.color`; «combo_box» `visuals.widgets.inactive.bg_stroke.color`; «combo_box» `visuals.widgets.hovered.bg_stroke.color`; «combo_box» `visuals.widgets.active.bg_stroke.color`; «combo_box» `visuals.widgets.open.bg_stroke.color`; «tab» `visuals.widgets.noninteractive.bg_stroke.color` (when: tab.border.color is not fully transparent, §6.13); «tab» `visuals.widgets.inactive.bg_stroke.color` (when: tab.border.color is not fully transparent, §6.13); «tab» `visuals.widgets.hovered.bg_stroke.color` (when: tab.border.color is not fully transparent, §6.13); «tab» `visuals.widgets.active.bg_stroke.color` (when: tab.border.color is not fully transparent, §6.13); «tab» `visuals.widgets.open.bg_stroke.color` (when: tab.border.color is not fully transparent, §6.13); «sidebar» `visuals.widgets.noninteractive.bg_stroke.color`; «toolbar» `visuals.widgets.noninteractive.bg_stroke.color`; «status_bar» `visuals.widgets.noninteractive.bg_stroke.color`; «expander» `visuals.widgets.noninteractive.bg_stroke.color`; «expander» `visuals.widgets.inactive.bg_stroke.color`; «expander» `visuals.widgets.hovered.bg_stroke.color`; «expander» `visuals.widgets.active.bg_stroke.color`; «expander» `visuals.widgets.open.bg_stroke.color` |  |
| `defaults.border.shadow_enabled` | DERIVED | `visuals.popup_shadow.offset`; `visuals.popup_shadow.blur`; `visuals.popup_shadow.spread` (when: never: egui's own spread, 0, is Shadow::NONE's, §6.14); `visuals.popup_shadow.color` | probe `false` |
| `defaults.danger_color` | DIRECT | `visuals.error_fg_color` |  |
| `defaults.danger_text_color` | DERIVED | → T18(a) |  |
| `defaults.disabled_opacity` | DIRECT | `visuals.disabled_alpha` |  |
| `defaults.disabled_text_color` | UNMAPPABLE |  | `egui-limited`: egui: a sixth Widgets entry, or a WidgetState::Disabled (§14 item 6) |
| `defaults.focus_ring_color` | DERIVED | → T14(b) |  |
| `defaults.focus_ring_offset` | DERIVED | → T14(b) |  |
| `defaults.focus_ring_width` | DERIVED | → T14(b) |  |
| `defaults.font.color` | UNMAPPABLE |  | `egui-limited`: egui: a text colour per role (§14 item 40) |
| `defaults.font.defined_size` | UNMAPPABLE |  | `egui-limited`: egui: a stated-size unit beside FontId::size (§5.1) |
| `defaults.font.family` | DERIVED | → T6(d) |  |
| `defaults.font.size` | DIRECT | `text_styles[Body].size`; `spacing.extra_text_line_spacing` (when: the theme's line box is taller than epaint's row, §6.15); «slider» `visuals.widgets.noninteractive.expansion` (when: Body row \> 1.25 · thumb_diameter, §6.6); «slider» `visuals.widgets.inactive.expansion` (when: Body row \> 1.25 · thumb_diameter, §6.6); «slider» `visuals.widgets.hovered.expansion` (when: Body row \> 1.25 · thumb_diameter, §6.6); «slider» `visuals.widgets.active.expansion` (when: Body row \> 1.25 · thumb_diameter, §6.6); «slider» `visuals.widgets.open.expansion` (when: Body row \> 1.25 · thumb_diameter, §6.6) |  |
| `defaults.font.style` | DERIVED | → T6(d) | probe `"italic"` |
| `defaults.font.weight` | DERIVED | → T6(d) | probe `700` |
| `defaults.icon_sizes.dialog` | DERIVED | → T18(e) |  |
| `defaults.icon_sizes.large` | DERIVED | → T18(e) |  |
| `defaults.icon_sizes.panel` | DERIVED | → T18(e) |  |
| `defaults.icon_sizes.small` | DERIVED | → T18(e) | probe `20.0` |
| `defaults.icon_sizes.toolbar` | DERIVED | → T18(e) |  |
| `defaults.info_color` | DERIVED | → T18(a) |  |
| `defaults.info_text_color` | DERIVED | → T18(a) |  |
| `defaults.line_height` | DERIVED | `spacing.extra_text_line_spacing` (when: the theme's line box is taller than epaint's row, §6.15) |  |
| `defaults.link_color` | DIRECT | `visuals.hyperlink_color` |  |
| `defaults.mono_font.color` | DERIVED | → T18(a) |  |
| `defaults.mono_font.defined_size` | UNMAPPABLE |  | `egui-limited`: egui: a stated-size unit beside FontId::size (§5.1) |
| `defaults.mono_font.family` | DERIVED | → T6(d) |  |
| `defaults.mono_font.size` | DIRECT | `text_styles[Monospace].size` |  |
| `defaults.mono_font.style` | DERIVED | → T6(d) | probe `"italic"` |
| `defaults.mono_font.weight` | DERIVED | → T6(d) | probe `700` |
| `defaults.muted_color` | DIRECT | `visuals.weak_text_color` |  |
| `defaults.selection_background` | DIRECT | `visuals.selection.bg_fill` |  |
| `defaults.selection_inactive_background` | UNMAPPABLE |  | `source-side gap`: native-theme: a text colour for the unfocused selection fill (§14 item 45) |
| `defaults.selection_text_color` | DIRECT | `visuals.selection.stroke.color` |  |
| `defaults.shadow_color` | DIRECT | `visuals.window_shadow.color` (when: window.border.shadow_enabled, §6.14); `visuals.popup_shadow.color` (when: defaults.border.shadow_enabled, §6.14); \[dialog\] `shadow.color` (when: dialog.border.shadow_enabled, §6.14); \[popover\] `shadow.color` (when: popover.border.shadow_enabled, §6.14); \[tooltip\] `shadow.color` (when: tooltip.border.shadow_enabled, §6.14) |  |
| `defaults.success_color` | DERIVED | → T18(a) |  |
| `defaults.success_text_color` | DERIVED | → T18(a) |  |
| `defaults.surface_color` | UNMAPPABLE |  | `egui-limited`: egui: a content-surface fill in Visuals |
| `defaults.text_color` | DIRECT | `visuals.widgets.noninteractive.fg_stroke.color` |  |
| `defaults.text_selection_background` | UNMAPPABLE |  | `egui-limited`: egui: a text-selection pair apart from Visuals::selection |
| `defaults.text_selection_color` | UNMAPPABLE |  | `egui-limited`: egui: a text-selection pair apart from Visuals::selection |
| `defaults.warning_color` | DIRECT | `visuals.warn_fg_color` |  |
| `defaults.warning_text_color` | DERIVED | → T18(a) |  |
| `layout.container_margin` | DERIVED | → T18(a) |  |
| `layout.section_gap` | DERIVED | → T18(a) |  |
| `layout.widget_gap` | DERIVED | `spacing.item_spacing.x`; `spacing.item_spacing.y` | probe `11.0` |
| `layout.window_margin` | SCOPED | \[central_panel\] `inner_margin.top`; \[central_panel\] `inner_margin.right`; \[central_panel\] `inner_margin.bottom`; \[central_panel\] `inner_margin.left` |  |
| `text_scale.caption.line_height` | DERIVED | → T18(e) |  |
| `text_scale.caption.size` | DIRECT | `text_styles[Small].size` |  |
| `text_scale.caption.weight` | DERIVED | → T18(e) | probe `700` |
| `text_scale.dialog_title.line_height` | DERIVED | → T18(e) |  |
| `text_scale.dialog_title.size` | DERIVED | → T18(e) |  |
| `text_scale.dialog_title.weight` | DERIVED | → T18(e) |  |
| `text_scale.display.line_height` | DERIVED | → T18(e) |  |
| `text_scale.display.size` | DERIVED | → T18(e) |  |
| `text_scale.display.weight` | DERIVED | → T18(e) |  |
| `text_scale.section_heading.line_height` | DERIVED | → T18(e) |  |
| `text_scale.section_heading.size` | DIRECT | `text_styles[Heading].size` |  |
| `text_scale.section_heading.weight` | DERIVED | → T18(e) |  |

### Surfaces (window, dialog, popover, card, tooltip, menu)

| leaf | verdict | sinks, or the test of its route | note |
|---|---|---|---|
| `card.background_color` | SCOPED | \[card\] `fill` |  |
| `card.border.color` | SCOPED | \[card\] `stroke.color` | probe `"#ff000080"` |
| `card.border.corner_radius` | SCOPED | \[card\] `corner_radius` |  |
| `card.border.line_width` | SCOPED | \[card\] `stroke.width` | probe `2.0` |
| `card.border.padding.bottom` | SCOPED | \[card\] `inner_margin.bottom` |  |
| `card.border.padding.left` | SCOPED | \[card\] `inner_margin.left` |  |
| `card.border.padding.right` | SCOPED | \[card\] `inner_margin.right` |  |
| `card.border.padding.top` | SCOPED | \[card\] `inner_margin.top` |  |
| `card.border.shadow_enabled` | UNMAPPABLE |  | `source-side gap`: native-theme: shadow offset, blur and spread in WidgetBorderSpec (§14 item 10) |
| `dialog.background_color` | SCOPED | \[dialog\] `fill` |  |
| `dialog.body_font.color` | SCOPED | «dialog» `visuals.widgets.noninteractive.fg_stroke.color` |  |
| `dialog.body_font.defined_size` | UNMAPPABLE |  | `egui-limited`: egui: a stated-size unit beside FontId::size (§5.1) |
| `dialog.body_font.family` | UNMAPPABLE |  | `egui-limited`: a TextStyle key naming a FontFamily::Name (declined, §5.8 item 7) |
| `dialog.body_font.size` | SCOPED | «dialog» `text_styles[Body].size` |  |
| `dialog.body_font.style` | DERIVED | → T18(d) | probe `"italic"` |
| `dialog.body_font.weight` | DERIVED | → T18(c) | probe `700` |
| `dialog.border.color` | SCOPED | \[dialog\] `stroke.color` |  |
| `dialog.border.corner_radius` | SCOPED | \[dialog\] `corner_radius` |  |
| `dialog.border.line_width` | SCOPED | \[dialog\] `stroke.width` |  |
| `dialog.border.padding.bottom` | SCOPED | \[dialog\] `inner_margin.bottom` |  |
| `dialog.border.padding.left` | SCOPED | \[dialog\] `inner_margin.left` |  |
| `dialog.border.padding.right` | SCOPED | \[dialog\] `inner_margin.right` |  |
| `dialog.border.padding.top` | SCOPED | \[dialog\] `inner_margin.top` |  |
| `dialog.border.shadow_enabled` | DERIVED | \[dialog\] `shadow.offset`; \[dialog\] `shadow.blur`; \[dialog\] `shadow.spread` (when: never: egui's own spread, 0, is Shadow::NONE's, §6.14); \[dialog\] `shadow.color` | probe `false` |
| `dialog.button_gap` | SCOPED | «dialog» `spacing.item_spacing.x` |  |
| `dialog.button_order` | DERIVED | → T18(e) |  |
| `dialog.icon_size` | DERIVED | → T18(a) |  |
| `dialog.max_height` | DERIVED | → T18(a) |  |
| `dialog.max_width` | DERIVED | → T18(a) |  |
| `dialog.min_height` | DERIVED | → T18(a) |  |
| `dialog.min_width` | DERIVED | → T18(a) |  |
| `dialog.title_font.color` | DERIVED | → T18(a) |  |
| `dialog.title_font.defined_size` | UNMAPPABLE |  | `egui-limited`: egui: a stated-size unit beside FontId::size (§5.1) |
| `dialog.title_font.family` | UNMAPPABLE |  | `egui-limited`: a TextStyle key naming a FontFamily::Name (declined, §5.8 item 7) |
| `dialog.title_font.size` | SCOPED | «dialog» `text_styles[Heading].size` |  |
| `dialog.title_font.style` | DERIVED | → T18(a) | probe `"italic"` |
| `dialog.title_font.weight` | DERIVED | → T18(a) |  |
| `menu.background_color` | DIRECT | `visuals.window_fill` |  |
| `menu.border.color` | SCOPED | «menu» `visuals.widgets.inactive.bg_stroke.color`; «menu» `visuals.widgets.hovered.bg_stroke.color`; «menu» `visuals.widgets.active.bg_stroke.color`; «menu» `visuals.widgets.open.bg_stroke.color` | probe `"#ff000080"` |
| `menu.border.corner_radius` | SCOPED | «menu» `visuals.widgets.inactive.corner_radius`; «menu» `visuals.widgets.hovered.corner_radius`; «menu» `visuals.widgets.active.corner_radius`; «menu» `visuals.widgets.open.corner_radius` | probe `4.0` |
| `menu.border.line_width` | SCOPED | «menu» `visuals.widgets.inactive.bg_stroke.width`; «menu» `visuals.widgets.hovered.bg_stroke.width`; «menu» `visuals.widgets.active.bg_stroke.width`; «menu» `visuals.widgets.open.bg_stroke.width`; «menu» `spacing.button_padding.x` (when: a side of the pair is stated, §5 intro); «menu» `spacing.button_padding.y` (when: a side of the pair is stated, §5 intro) | probe `2.0` |
| `menu.border.padding.bottom` | DERIVED | «menu» `spacing.button_padding.y` |  |
| `menu.border.padding.left` | DERIVED | «menu» `spacing.button_padding.x` |  |
| `menu.border.padding.right` | DERIVED | «menu» `spacing.button_padding.x` |  |
| `menu.border.padding.top` | DERIVED | «menu» `spacing.button_padding.y` |  |
| `menu.border.shadow_enabled` | UNMAPPABLE |  | `source-side gap`: native-theme: shadow offset, blur and spread in WidgetBorderSpec (§14 item 10) |
| `menu.disabled_text_color` | SCOPED | «menu:disabled» `visuals.widgets.noninteractive.fg_stroke.color`; «menu:disabled» `visuals.widgets.inactive.fg_stroke.color` |  |
| `menu.font.color` | SCOPED | «menu» `visuals.widgets.noninteractive.fg_stroke.color`; «menu» `visuals.widgets.inactive.fg_stroke.color` |  |
| `menu.font.defined_size` | UNMAPPABLE |  | `egui-limited`: egui: a stated-size unit beside FontId::size (§5.1) |
| `menu.font.family` | UNMAPPABLE |  | `egui-limited`: a TextStyle key naming a FontFamily::Name (declined, §5.8 item 7) |
| `menu.font.size` | SCOPED | «menu» `text_styles[Body].size` |  |
| `menu.font.style` | DERIVED | → T18(d) | probe `"italic"` |
| `menu.font.weight` | DERIVED | → T18(c) | probe `700` |
| `menu.hover_background` | SCOPED | «menu» `visuals.widgets.hovered.weak_bg_fill`; «menu» `visuals.widgets.active.weak_bg_fill`; «menu» `visuals.widgets.open.weak_bg_fill` |  |
| `menu.hover_text_color` | SCOPED | «menu» `visuals.widgets.hovered.fg_stroke.color`; «menu» `visuals.widgets.active.fg_stroke.color`; «menu» `visuals.widgets.open.fg_stroke.color` |  |
| `menu.icon_size` | DERIVED | → T18(a) |  |
| `menu.icon_text_gap` | SCOPED | «menu» `spacing.icon_spacing` |  |
| `menu.row_height` | SCOPED | «menu» `spacing.interact_size.y` |  |
| `menu.separator_color` | SCOPED | «menu» `visuals.widgets.noninteractive.bg_stroke.color` |  |
| `popover.background_color` | SCOPED | \[popover\] `fill` |  |
| `popover.border.color` | SCOPED | \[popover\] `stroke.color` |  |
| `popover.border.corner_radius` | SCOPED | \[popover\] `corner_radius` |  |
| `popover.border.line_width` | SCOPED | \[popover\] `stroke.width` |  |
| `popover.border.padding.bottom` | SCOPED | \[popover\] `inner_margin.bottom` |  |
| `popover.border.padding.left` | SCOPED | \[popover\] `inner_margin.left` |  |
| `popover.border.padding.right` | SCOPED | \[popover\] `inner_margin.right` |  |
| `popover.border.padding.top` | SCOPED | \[popover\] `inner_margin.top` |  |
| `popover.border.shadow_enabled` | DERIVED | \[popover\] `shadow.offset`; \[popover\] `shadow.blur`; \[popover\] `shadow.spread` (when: never: egui's own spread, 0, is Shadow::NONE's, §6.14); \[popover\] `shadow.color` | probe `false` |
| `popover.font.color` | SCOPED | «popover» `visuals.widgets.noninteractive.fg_stroke.color` |  |
| `popover.font.defined_size` | UNMAPPABLE |  | `egui-limited`: egui: a stated-size unit beside FontId::size (§5.1) |
| `popover.font.family` | UNMAPPABLE |  | `egui-limited`: a TextStyle key naming a FontFamily::Name (declined, §5.8 item 7) |
| `popover.font.size` | SCOPED | «popover» `text_styles[Body].size` |  |
| `popover.font.style` | DERIVED | → T18(d) | probe `"italic"` |
| `popover.font.weight` | DERIVED | → T18(c) | probe `700` |
| `tooltip.background_color` | SCOPED | \[tooltip\] `fill` |  |
| `tooltip.border.color` | SCOPED | \[tooltip\] `stroke.color` |  |
| `tooltip.border.corner_radius` | SCOPED | \[tooltip\] `corner_radius` |  |
| `tooltip.border.line_width` | SCOPED | \[tooltip\] `stroke.width` |  |
| `tooltip.border.padding.bottom` | SCOPED | \[tooltip\] `inner_margin.bottom` |  |
| `tooltip.border.padding.left` | SCOPED | \[tooltip\] `inner_margin.left` |  |
| `tooltip.border.padding.right` | SCOPED | \[tooltip\] `inner_margin.right` |  |
| `tooltip.border.padding.top` | SCOPED | \[tooltip\] `inner_margin.top` |  |
| `tooltip.border.shadow_enabled` | DERIVED | \[tooltip\] `shadow.offset`; \[tooltip\] `shadow.blur`; \[tooltip\] `shadow.spread` (when: never: egui's own spread, 0, is Shadow::NONE's, §6.14); \[tooltip\] `shadow.color` | probe `false` |
| `tooltip.font.color` | SCOPED | «tooltip» `visuals.widgets.noninteractive.fg_stroke.color` |  |
| `tooltip.font.defined_size` | UNMAPPABLE |  | `egui-limited`: egui: a stated-size unit beside FontId::size (§5.1) |
| `tooltip.font.family` | UNMAPPABLE |  | `egui-limited`: a TextStyle key naming a FontFamily::Name (declined, §5.8 item 7) |
| `tooltip.font.size` | SCOPED | «tooltip» `text_styles[Body].size` |  |
| `tooltip.font.style` | DERIVED | → T18(d) | probe `"italic"` |
| `tooltip.font.weight` | DERIVED | → T18(c) | probe `700` |
| `tooltip.max_width` | DIRECT | `spacing.tooltip_width` |  |
| `window.background_color` | SCOPED | \[window\] `fill` |  |
| `window.border.color` | SCOPED | \[window\] `stroke.color` |  |
| `window.border.corner_radius` | DIRECT | `visuals.window_corner_radius` |  |
| `window.border.line_width` | SCOPED | \[window\] `stroke.width` |  |
| `window.border.padding.bottom` | DIRECT | `spacing.window_margin.bottom` | probe `5.0` |
| `window.border.padding.left` | DIRECT | `spacing.window_margin.left` | probe `5.0` |
| `window.border.padding.right` | DIRECT | `spacing.window_margin.right` | probe `5.0` |
| `window.border.padding.top` | DIRECT | `spacing.window_margin.top` | probe `5.0` |
| `window.border.shadow_enabled` | DERIVED | `visuals.window_shadow.offset`; `visuals.window_shadow.blur`; `visuals.window_shadow.spread` (when: never: egui's own spread, 0, is Shadow::NONE's, §6.14); `visuals.window_shadow.color` | probe `false` |
| `window.inactive_title_bar_background` | SCOPED | \[window_title_bar\] `fill` |  |
| `window.inactive_title_bar_text_color` | DERIVED | → T18(e) |  |
| `window.title_bar_background` | DIRECT | `visuals.widgets.open.weak_bg_fill` |  |
| `window.title_bar_font.color` | DERIVED | → T18(e) |  |
| `window.title_bar_font.defined_size` | UNMAPPABLE |  | `egui-limited`: egui: a stated-size unit beside FontId::size (§5.1) |
| `window.title_bar_font.family` | UNMAPPABLE |  | `egui-limited`: a TextStyle key naming a FontFamily::Name (declined, §5.8 item 7) |
| `window.title_bar_font.size` | DERIVED | → T18(e) |  |
| `window.title_bar_font.style` | DERIVED | → T18(d) | probe `"italic"` |
| `window.title_bar_font.weight` | DERIVED | → T18(c) |  |

### Buttons (button, link, switch, checkbox, segmented control)

| leaf | verdict | sinks, or the test of its route | note |
|---|---|---|---|
| `button.active_background` | DIRECT | `visuals.widgets.active.weak_bg_fill` (when: stated; None copies button.hover_background, §6.4); «button» `visuals.widgets.active.weak_bg_fill` (when: stated; None copies button.hover_background, §6.4) |  |
| `button.active_text_color` | DIRECT | `visuals.widgets.active.fg_stroke.color`; «button» `visuals.widgets.active.fg_stroke.color` |  |
| `button.background_color` | DIRECT | `visuals.widgets.inactive.weak_bg_fill`; `visuals.widgets.hovered.weak_bg_fill` (when: button.hover_background is translucent, §6.1); `visuals.widgets.active.weak_bg_fill` (when: the pressed layer is translucent, §6.1); «button» `visuals.widgets.inactive.weak_bg_fill`; «button» `visuals.widgets.open.weak_bg_fill`; «button» `visuals.widgets.hovered.weak_bg_fill` (when: button.hover_background is translucent, §6.1); «button» `visuals.widgets.active.weak_bg_fill` (when: the pressed layer is translucent, §6.1); «button:disabled» `visuals.widgets.inactive.weak_bg_fill` (when: button.disabled_background is None, §6.4); «button:disabled» `visuals.selection.bg_fill` (when: button.disabled_background is None, §6.4) |  |
| `button.border.color` | DIRECT | `visuals.widgets.inactive.bg_stroke.color`; `visuals.widgets.hovered.bg_stroke.color`; `visuals.widgets.active.bg_stroke.color`; `visuals.widgets.open.bg_stroke.color`; «button» `visuals.widgets.noninteractive.bg_stroke.color`; «button» `visuals.widgets.inactive.bg_stroke.color`; «button» `visuals.widgets.hovered.bg_stroke.color`; «button» `visuals.widgets.active.bg_stroke.color`; «button» `visuals.widgets.open.bg_stroke.color` |  |
| `button.border.corner_radius` | DIRECT | «button» `visuals.widgets.noninteractive.corner_radius`; «button» `visuals.widgets.inactive.corner_radius`; «button» `visuals.widgets.hovered.corner_radius`; «button» `visuals.widgets.active.corner_radius`; «button» `visuals.widgets.open.corner_radius`; `visuals.widgets.inactive.corner_radius`; `visuals.widgets.hovered.corner_radius`; `visuals.widgets.active.corner_radius`; `visuals.widgets.open.corner_radius` |  |
| `button.border.line_width` | DIRECT | «button» `visuals.widgets.noninteractive.bg_stroke.width`; «button» `visuals.widgets.inactive.bg_stroke.width`; «button» `visuals.widgets.hovered.bg_stroke.width`; «button» `visuals.widgets.active.bg_stroke.width`; «button» `visuals.widgets.open.bg_stroke.width`; `visuals.widgets.inactive.bg_stroke.width`; `visuals.widgets.hovered.bg_stroke.width`; `visuals.widgets.active.bg_stroke.width`; `visuals.widgets.open.bg_stroke.width`; `spacing.button_padding.x` (when: a side of the pair is stated, §5 intro); `spacing.button_padding.y` (when: a side of the pair is stated, §5 intro); «button» `spacing.button_padding.x` (when: a side of the pair is stated, §5 intro); «button» `spacing.button_padding.y` (when: a side of the pair is stated, §5 intro) |  |
| `button.border.padding.bottom` | DERIVED | `spacing.button_padding.y`; «button» `spacing.button_padding.y` |  |
| `button.border.padding.left` | DERIVED | `spacing.button_padding.x`; «button» `spacing.button_padding.x` |  |
| `button.border.padding.right` | DERIVED | `spacing.button_padding.x`; «button» `spacing.button_padding.x` |  |
| `button.border.padding.top` | DERIVED | `spacing.button_padding.y`; «button» `spacing.button_padding.y` |  |
| `button.border.shadow_enabled` | UNMAPPABLE |  | `source-side gap`: native-theme: shadow offset, blur and spread in WidgetBorderSpec (§14 item 10) |
| `button.disabled_background` | SCOPED | «button:disabled» `visuals.widgets.inactive.weak_bg_fill` (when: stated; None copies button.background_color, §6.4); «button:disabled» `visuals.selection.bg_fill` (when: stated; None copies button.background_color, §6.4) |  |
| `button.disabled_opacity` | SCOPED | «button» `visuals.disabled_alpha` |  |
| `button.disabled_text_color` | SCOPED | «button:disabled» `visuals.widgets.noninteractive.fg_stroke.color`; «button:disabled» `visuals.widgets.inactive.fg_stroke.color`; «button:disabled» `visuals.selection.stroke.color` |  |
| `button.font.color` | DIRECT | `visuals.widgets.inactive.fg_stroke.color`; `visuals.widgets.open.fg_stroke.color`; «button» `visuals.widgets.noninteractive.fg_stroke.color`; «button» `visuals.widgets.inactive.fg_stroke.color`; «button» `visuals.widgets.open.fg_stroke.color` |  |
| `button.font.defined_size` | UNMAPPABLE |  | `egui-limited`: egui: a stated-size unit beside FontId::size (§5.1) |
| `button.font.family` | UNMAPPABLE |  | `egui-limited`: a TextStyle key naming a FontFamily::Name (declined, §5.8 item 7) |
| `button.font.size` | SCOPED | «button» `override_font_id.size`; `text_styles[Button].size` |  |
| `button.font.style` | DERIVED | → T18(d) | probe `"italic"` |
| `button.font.weight` | DERIVED | → T18(c) | probe `700` |
| `button.hover_background` | DIRECT | `visuals.widgets.hovered.weak_bg_fill`; «button» `visuals.widgets.hovered.weak_bg_fill`; `visuals.widgets.active.weak_bg_fill` (when: button.active_background is None, §6.4); «button» `visuals.widgets.active.weak_bg_fill` (when: button.active_background is None, §6.4) |  |
| `button.hover_text_color` | DIRECT | `visuals.widgets.hovered.fg_stroke.color`; «button» `visuals.widgets.hovered.fg_stroke.color` |  |
| `button.icon_text_gap` | SCOPED | «button» `spacing.icon_spacing` |  |
| `button.min_height` | DIRECT | `spacing.interact_size.y`; «button» `spacing.interact_size.y` |  |
| `button.min_width` | DERIVED | → T18(a) |  |
| `button.primary_background` | SCOPED | «button» `visuals.selection.bg_fill` |  |
| `button.primary_text_color` | SCOPED | «button» `visuals.selection.stroke.color` |  |
| `checkbox.background_color` | SCOPED | «checkbox» `visuals.widgets.inactive.bg_fill` (when: checkbox.unchecked_background is None, §6.4); «checkbox» `visuals.widgets.open.bg_fill` (when: checkbox.unchecked_background is None, §6.4); «checkbox» `visuals.widgets.hovered.bg_fill` (when: checkbox.unchecked_background is None and the hover layer is translucent or None, §6.1, §6.4); «checkbox» `visuals.widgets.active.bg_fill` (when: checkbox.unchecked_background is None and the hover layer is translucent or None, §6.1, §6.4); «checkbox:disabled» `visuals.widgets.inactive.bg_fill` (when: checkbox.disabled_background and checkbox.unchecked_background are None, §6.4) |  |
| `checkbox.border.color` | SCOPED | «checkbox:selected» `visuals.widgets.noninteractive.bg_stroke.color`; «checkbox:selected» `visuals.widgets.inactive.bg_stroke.color`; «checkbox:selected» `visuals.widgets.hovered.bg_stroke.color`; «checkbox:selected» `visuals.widgets.active.bg_stroke.color`; «checkbox:selected» `visuals.widgets.open.bg_stroke.color`; «checkbox» `visuals.widgets.noninteractive.bg_stroke.color` (when: checkbox.unchecked_border_color is None, §6.4); «checkbox» `visuals.widgets.inactive.bg_stroke.color` (when: checkbox.unchecked_border_color is None, §6.4); «checkbox» `visuals.widgets.hovered.bg_stroke.color` (when: checkbox.unchecked_border_color is None, §6.4); «checkbox» `visuals.widgets.active.bg_stroke.color` (when: checkbox.unchecked_border_color is None, §6.4); «checkbox» `visuals.widgets.open.bg_stroke.color` (when: checkbox.unchecked_border_color is None, §6.4) |  |
| `checkbox.border.corner_radius` | SCOPED | «checkbox» `visuals.widgets.noninteractive.corner_radius`; «checkbox» `visuals.widgets.inactive.corner_radius`; «checkbox» `visuals.widgets.hovered.corner_radius`; «checkbox» `visuals.widgets.active.corner_radius`; «checkbox» `visuals.widgets.open.corner_radius` |  |
| `checkbox.border.line_width` | SCOPED | «checkbox» `visuals.widgets.noninteractive.bg_stroke.width`; «checkbox» `visuals.widgets.inactive.bg_stroke.width`; «checkbox» `visuals.widgets.hovered.bg_stroke.width`; «checkbox» `visuals.widgets.active.bg_stroke.width`; «checkbox» `visuals.widgets.open.bg_stroke.width` |  |
| `checkbox.border.padding.bottom` | DERIVED | «checkbox» `spacing.icon_width_inner` (when: all four checkbox.border.padding sides are stated, §6.11) |  |
| `checkbox.border.padding.left` | DERIVED | «checkbox» `spacing.icon_width_inner` (when: all four checkbox.border.padding sides are stated, §6.11) |  |
| `checkbox.border.padding.right` | DERIVED | «checkbox» `spacing.icon_width_inner` (when: all four checkbox.border.padding sides are stated, §6.11) |  |
| `checkbox.border.padding.top` | DERIVED | «checkbox» `spacing.icon_width_inner` (when: all four checkbox.border.padding sides are stated, §6.11) |  |
| `checkbox.border.shadow_enabled` | UNMAPPABLE |  | `source-side gap`: native-theme: shadow offset, blur and spread in WidgetBorderSpec (§14 item 10) |
| `checkbox.checked_background` | SCOPED | «checkbox:selected» `visuals.widgets.noninteractive.bg_fill`; «checkbox:selected» `visuals.widgets.inactive.bg_fill`; «checkbox:selected» `visuals.widgets.hovered.bg_fill`; «checkbox:selected» `visuals.widgets.active.bg_fill`; «checkbox:selected» `visuals.widgets.open.bg_fill` |  |
| `checkbox.disabled_background` | SCOPED | «checkbox:disabled» `visuals.widgets.inactive.bg_fill` (when: stated; None copies the idle fill, §6.4) |  |
| `checkbox.disabled_opacity` | SCOPED | «checkbox» `visuals.disabled_alpha` |  |
| `checkbox.disabled_text_color` | SCOPED | «checkbox:disabled» `visuals.override_text_color`; «checkbox:disabled» `visuals.widgets.inactive.fg_stroke.color` |  |
| `checkbox.font.color` | SCOPED | «checkbox» `visuals.override_text_color` |  |
| `checkbox.font.defined_size` | UNMAPPABLE |  | `egui-limited`: egui: a stated-size unit beside FontId::size (§5.1) |
| `checkbox.font.family` | UNMAPPABLE |  | `egui-limited`: a TextStyle key naming a FontFamily::Name (declined, §5.8 item 7) |
| `checkbox.font.size` | SCOPED | «checkbox» `override_font_id.size` |  |
| `checkbox.font.style` | DERIVED | → T18(d) | probe `"italic"` |
| `checkbox.font.weight` | DERIVED | → T18(c) | probe `700` |
| `checkbox.hover_background` | SCOPED | «checkbox» `visuals.widgets.hovered.bg_fill` (when: stated; None copies the idle fill, §6.4); «checkbox» `visuals.widgets.active.bg_fill` (when: stated; None copies the idle fill, §6.4) |  |
| `checkbox.indicator_color` | SCOPED | «checkbox» `visuals.widgets.noninteractive.fg_stroke.color`; «checkbox» `visuals.widgets.inactive.fg_stroke.color`; «checkbox» `visuals.widgets.hovered.fg_stroke.color`; «checkbox» `visuals.widgets.active.fg_stroke.color`; «checkbox» `visuals.widgets.open.fg_stroke.color` |  |
| `checkbox.indicator_width` | DIRECT | `spacing.icon_width`; «checkbox» `spacing.icon_width`; «checkbox» `spacing.icon_width_inner` (when: all four checkbox.border.padding sides are stated, §6.11) |  |
| `checkbox.label_gap` | DIRECT | `spacing.icon_spacing`; «checkbox» `spacing.icon_spacing` |  |
| `checkbox.unchecked_background` | SCOPED | «checkbox» `visuals.widgets.inactive.bg_fill` (when: stated; None copies checkbox.background_color, §6.4); «checkbox» `visuals.widgets.open.bg_fill` (when: stated; None copies checkbox.background_color, §6.4); «checkbox» `visuals.widgets.hovered.bg_fill` (when: checkbox.hover_background is translucent or None, §6.1, §6.4); «checkbox» `visuals.widgets.active.bg_fill` (when: checkbox.hover_background is translucent or None, §6.1, §6.4); «checkbox:disabled» `visuals.widgets.inactive.bg_fill` (when: checkbox.disabled_background is None, §6.4) |  |
| `checkbox.unchecked_border_color` | SCOPED | «checkbox» `visuals.widgets.noninteractive.bg_stroke.color` (when: stated; None copies checkbox.border.color, §6.4); «checkbox» `visuals.widgets.inactive.bg_stroke.color` (when: stated; None copies checkbox.border.color, §6.4); «checkbox» `visuals.widgets.hovered.bg_stroke.color` (when: stated; None copies checkbox.border.color, §6.4); «checkbox» `visuals.widgets.active.bg_stroke.color` (when: stated; None copies checkbox.border.color, §6.4); «checkbox» `visuals.widgets.open.bg_stroke.color` (when: stated; None copies checkbox.border.color, §6.4) |  |
| `link.active_text_color` | DERIVED | → T18(a) |  |
| `link.background_color` | DERIVED | → T18(a) | probe `"#ff000080"` |
| `link.disabled_text_color` | SCOPED | «link:disabled» `visuals.hyperlink_color` |  |
| `link.font.color` | SCOPED | «link» `visuals.hyperlink_color` |  |
| `link.font.defined_size` | UNMAPPABLE |  | `egui-limited`: egui: a stated-size unit beside FontId::size (§5.1) |
| `link.font.family` | UNMAPPABLE |  | `egui-limited`: a TextStyle key naming a FontFamily::Name (declined, §5.8 item 7) |
| `link.font.size` | SCOPED | «link» `override_font_id.size` |  |
| `link.font.style` | DERIVED | → T18(d) | probe `"italic"` |
| `link.font.weight` | DERIVED | → T18(c) | probe `700` |
| `link.hover_background` | DERIVED | → T18(a) |  |
| `link.hover_text_color` | DERIVED | → T18(a) |  |
| `link.underline_enabled` | DERIVED | → T18(a) |  |
| `link.visited_text_color` | DERIVED | → T18(a) |  |
| `segmented_control.active_background` | SCOPED | «segmented_control» `visuals.selection.bg_fill` |  |
| `segmented_control.active_text_color` | SCOPED | «segmented_control» `visuals.selection.stroke.color` |  |
| `segmented_control.background_color` | SCOPED | «segmented_control» `visuals.widgets.inactive.weak_bg_fill`; «segmented_control» `visuals.widgets.open.weak_bg_fill`; «segmented_control» `visuals.widgets.hovered.weak_bg_fill` (when: segmented_control.hover_background is translucent or None, §6.1, §6.4); «segmented_control» `visuals.widgets.active.weak_bg_fill` (when: segmented_control.hover_background is translucent or None, §6.1, §6.4) |  |
| `segmented_control.border.color` | SCOPED | «segmented_control» `visuals.widgets.noninteractive.bg_stroke.color`; «segmented_control» `visuals.widgets.inactive.bg_stroke.color`; «segmented_control» `visuals.widgets.hovered.bg_stroke.color`; «segmented_control» `visuals.widgets.active.bg_stroke.color`; «segmented_control» `visuals.widgets.open.bg_stroke.color` |  |
| `segmented_control.border.corner_radius` | SCOPED | «segmented_control» `visuals.widgets.noninteractive.corner_radius`; «segmented_control» `visuals.widgets.inactive.corner_radius`; «segmented_control» `visuals.widgets.hovered.corner_radius`; «segmented_control» `visuals.widgets.active.corner_radius`; «segmented_control» `visuals.widgets.open.corner_radius` |  |
| `segmented_control.border.line_width` | SCOPED | «segmented_control» `visuals.widgets.noninteractive.bg_stroke.width`; «segmented_control» `visuals.widgets.inactive.bg_stroke.width`; «segmented_control» `visuals.widgets.hovered.bg_stroke.width`; «segmented_control» `visuals.widgets.active.bg_stroke.width`; «segmented_control» `visuals.widgets.open.bg_stroke.width`; «segmented_control» `spacing.button_padding.x` (when: a side of the pair is stated, §5 intro); «segmented_control» `spacing.button_padding.y` (when: a side of the pair is stated, §5 intro) |  |
| `segmented_control.border.padding.bottom` | DERIVED | «segmented_control» `spacing.button_padding.y` |  |
| `segmented_control.border.padding.left` | DERIVED | «segmented_control» `spacing.button_padding.x` | probe `5.0` |
| `segmented_control.border.padding.right` | DERIVED | «segmented_control» `spacing.button_padding.x` | probe `5.0` |
| `segmented_control.border.padding.top` | DERIVED | «segmented_control» `spacing.button_padding.y` |  |
| `segmented_control.border.shadow_enabled` | UNMAPPABLE |  | `source-side gap`: native-theme: shadow offset, blur and spread in WidgetBorderSpec (§14 item 10) |
| `segmented_control.disabled_opacity` | SCOPED | «segmented_control» `visuals.disabled_alpha` |  |
| `segmented_control.font.color` | SCOPED | «segmented_control» `visuals.widgets.noninteractive.fg_stroke.color`; «segmented_control» `visuals.widgets.inactive.fg_stroke.color`; «segmented_control» `visuals.widgets.hovered.fg_stroke.color`; «segmented_control» `visuals.widgets.active.fg_stroke.color`; «segmented_control» `visuals.widgets.open.fg_stroke.color` |  |
| `segmented_control.font.defined_size` | UNMAPPABLE |  | `egui-limited`: egui: a stated-size unit beside FontId::size (§5.1) |
| `segmented_control.font.family` | UNMAPPABLE |  | `egui-limited`: a TextStyle key naming a FontFamily::Name (declined, §5.8 item 7) |
| `segmented_control.font.size` | SCOPED | «segmented_control» `override_font_id.size` |  |
| `segmented_control.font.style` | DERIVED | → T18(d) | probe `"italic"` |
| `segmented_control.font.weight` | DERIVED | → T18(c) | probe `700` |
| `segmented_control.hover_background` | SCOPED | «segmented_control» `visuals.widgets.hovered.weak_bg_fill` (when: stated; None copies segmented_control.background_color, §6.4); «segmented_control» `visuals.widgets.active.weak_bg_fill` (when: stated; None copies segmented_control.background_color, §6.4) |  |
| `segmented_control.segment_height` | SCOPED | «segmented_control» `spacing.interact_size.y` |  |
| `segmented_control.separator_width` | SCOPED | «segmented_control» `spacing.item_spacing.x` | probe `2.0` |
| `switch.checked_background` | SCOPED | «switch» `visuals.selection.bg_fill`; «switch:disabled» `visuals.selection.bg_fill` (when: switch.disabled_checked_background is None, §6.4) |  |
| `switch.disabled_checked_background` | SCOPED | «switch:disabled» `visuals.selection.bg_fill` (when: stated; None copies switch.checked_background, §6.4) |  |
| `switch.disabled_opacity` | SCOPED | «switch» `visuals.disabled_alpha` |  |
| `switch.disabled_thumb_color` | UNMAPPABLE |  | `widgets-crate`: native-theme-egui-widgets: Switch (docs/todo_egui-widgets-spec.md §4.1) |
| `switch.disabled_unchecked_background` | SCOPED | «switch:disabled» `visuals.widgets.inactive.weak_bg_fill` (when: stated; None copies switch.unchecked_background, §6.4) |  |
| `switch.hover_checked_background` | DERIVED | → T18(a) |  |
| `switch.hover_unchecked_background` | SCOPED | «switch» `visuals.widgets.hovered.weak_bg_fill` (when: stated; None copies switch.unchecked_background, §6.4); «switch» `visuals.widgets.active.weak_bg_fill` (when: stated; None copies switch.unchecked_background, §6.4) |  |
| `switch.thumb_background` | UNMAPPABLE |  | `widgets-crate`: native-theme-egui-widgets: Switch (docs/todo_egui-widgets-spec.md §4.1) |
| `switch.thumb_diameter` | UNMAPPABLE |  | `widgets-crate`: native-theme-egui-widgets: Switch (docs/todo_egui-widgets-spec.md §4.1) |
| `switch.track_height` | SCOPED | «switch» `spacing.interact_size.y` |  |
| `switch.track_radius` | SCOPED | «switch» `visuals.widgets.noninteractive.corner_radius`; «switch» `visuals.widgets.inactive.corner_radius`; «switch» `visuals.widgets.hovered.corner_radius`; «switch» `visuals.widgets.active.corner_radius`; «switch» `visuals.widgets.open.corner_radius` |  |
| `switch.track_width` | DERIVED | → T18(a) |  |
| `switch.unchecked_background` | SCOPED | «switch» `visuals.widgets.inactive.weak_bg_fill`; «switch» `visuals.widgets.open.weak_bg_fill`; «switch» `visuals.widgets.hovered.weak_bg_fill` (when: switch.hover_unchecked_background is translucent or None, §6.1, §6.4); «switch» `visuals.widgets.active.weak_bg_fill` (when: switch.hover_unchecked_background is translucent or None, §6.1, §6.4); «switch:disabled» `visuals.widgets.inactive.weak_bg_fill` (when: switch.disabled_unchecked_background is None, §6.4) |  |

### Inputs (input, combo box, list)

| leaf | verdict | sinks, or the test of its route | note |
|---|---|---|---|
| `combo_box.arrow_area_width` | DERIVED | «combo_box» `spacing.icon_spacing` |  |
| `combo_box.arrow_icon_size` | SCOPED | «combo_box» `spacing.icon_width`; «combo_box» `spacing.icon_spacing` (when: combo_box.arrow_area_width is stated, §6.12) |  |
| `combo_box.background_color` | SCOPED | «combo_box» `visuals.widgets.inactive.weak_bg_fill`; «combo_box» `visuals.widgets.open.weak_bg_fill`; «combo_box» `visuals.widgets.hovered.weak_bg_fill` (when: combo_box.hover_background is translucent or None, §6.1, §6.4); «combo_box» `visuals.widgets.active.weak_bg_fill` (when: combo_box.hover_background is translucent or None, §6.1, §6.4); «combo_box:disabled» `visuals.widgets.inactive.weak_bg_fill` (when: combo_box.disabled_background is None, §6.4) |  |
| `combo_box.border.color` | SCOPED | «combo_box» `visuals.widgets.noninteractive.bg_stroke.color`; «combo_box» `visuals.widgets.inactive.bg_stroke.color`; «combo_box» `visuals.widgets.hovered.bg_stroke.color`; «combo_box» `visuals.widgets.active.bg_stroke.color`; «combo_box» `visuals.widgets.open.bg_stroke.color` |  |
| `combo_box.border.corner_radius` | SCOPED | «combo_box» `visuals.widgets.noninteractive.corner_radius`; «combo_box» `visuals.widgets.inactive.corner_radius`; «combo_box» `visuals.widgets.hovered.corner_radius`; «combo_box» `visuals.widgets.active.corner_radius`; «combo_box» `visuals.widgets.open.corner_radius` |  |
| `combo_box.border.line_width` | SCOPED | «combo_box» `visuals.widgets.noninteractive.bg_stroke.width`; «combo_box» `visuals.widgets.inactive.bg_stroke.width`; «combo_box» `visuals.widgets.hovered.bg_stroke.width`; «combo_box» `visuals.widgets.active.bg_stroke.width`; «combo_box» `visuals.widgets.open.bg_stroke.width`; «combo_box» `spacing.button_padding.x` (when: a side of the pair is stated, §5 intro); «combo_box» `spacing.button_padding.y` (when: a side of the pair is stated, §5 intro) |  |
| `combo_box.border.padding.bottom` | DERIVED | «combo_box» `spacing.button_padding.y` |  |
| `combo_box.border.padding.left` | DERIVED | «combo_box» `spacing.button_padding.x` |  |
| `combo_box.border.padding.right` | DERIVED | «combo_box» `spacing.button_padding.x` |  |
| `combo_box.border.padding.top` | DERIVED | «combo_box» `spacing.button_padding.y` |  |
| `combo_box.border.shadow_enabled` | UNMAPPABLE |  | `source-side gap`: native-theme: shadow offset, blur and spread in WidgetBorderSpec (§14 item 10) |
| `combo_box.disabled_background` | SCOPED | «combo_box:disabled» `visuals.widgets.inactive.weak_bg_fill` (when: stated; None copies combo_box.background_color, §6.4) |  |
| `combo_box.disabled_opacity` | SCOPED | «combo_box» `visuals.disabled_alpha` |  |
| `combo_box.disabled_text_color` | SCOPED | «combo_box:disabled» `visuals.widgets.noninteractive.fg_stroke.color`; «combo_box:disabled» `visuals.widgets.inactive.fg_stroke.color` |  |
| `combo_box.font.color` | SCOPED | «combo_box» `visuals.widgets.noninteractive.fg_stroke.color`; «combo_box» `visuals.widgets.inactive.fg_stroke.color`; «combo_box» `visuals.widgets.hovered.fg_stroke.color`; «combo_box» `visuals.widgets.active.fg_stroke.color`; «combo_box» `visuals.widgets.open.fg_stroke.color` |  |
| `combo_box.font.defined_size` | UNMAPPABLE |  | `egui-limited`: egui: a stated-size unit beside FontId::size (§5.1) |
| `combo_box.font.family` | UNMAPPABLE |  | `egui-limited`: a TextStyle key naming a FontFamily::Name (declined, §5.8 item 7) |
| `combo_box.font.size` | SCOPED | «combo_box» `text_styles[Button].size` |  |
| `combo_box.font.style` | DERIVED | → T18(d) | probe `"italic"` |
| `combo_box.font.weight` | DERIVED | → T18(c) | probe `700` |
| `combo_box.hover_background` | SCOPED | «combo_box» `visuals.widgets.hovered.weak_bg_fill` (when: stated; None copies combo_box.background_color, §6.4); «combo_box» `visuals.widgets.active.weak_bg_fill` (when: stated; None copies combo_box.background_color, §6.4) |  |
| `combo_box.min_height` | SCOPED | «combo_box» `spacing.interact_size.y` |  |
| `combo_box.min_width` | DIRECT | `spacing.combo_width` |  |
| `input.background_color` | DIRECT | `visuals.text_edit_bg_color`; «input:disabled» `visuals.text_edit_bg_color` (when: input.disabled_background is None, §6.4) |  |
| `input.border.color` | SCOPED | «input» `visuals.widgets.noninteractive.bg_stroke.color`; «input» `visuals.widgets.inactive.bg_stroke.color`; «input» `visuals.widgets.open.bg_stroke.color`; «input» `visuals.widgets.hovered.bg_stroke.color` (when: input.hover_border_color is None, §6.4); «input» `visuals.widgets.active.bg_stroke.color` (when: input.hover_border_color is None, §6.4) |  |
| `input.border.corner_radius` | SCOPED | «input» `visuals.widgets.noninteractive.corner_radius`; «input» `visuals.widgets.inactive.corner_radius`; «input» `visuals.widgets.hovered.corner_radius`; «input» `visuals.widgets.active.corner_radius`; «input» `visuals.widgets.open.corner_radius` |  |
| `input.border.line_width` | SCOPED | «input» `visuals.widgets.noninteractive.bg_stroke.width`; «input» `visuals.widgets.inactive.bg_stroke.width`; «input» `visuals.widgets.hovered.bg_stroke.width`; «input» `visuals.widgets.active.bg_stroke.width`; «input» `visuals.widgets.open.bg_stroke.width` |  |
| `input.border.padding.bottom` | DERIVED | → T18(b) |  |
| `input.border.padding.left` | DERIVED | → T18(b) |  |
| `input.border.padding.right` | DERIVED | → T18(b) |  |
| `input.border.padding.top` | DERIVED | → T18(b) |  |
| `input.border.shadow_enabled` | UNMAPPABLE |  | `source-side gap`: native-theme: shadow offset, blur and spread in WidgetBorderSpec (§14 item 10) |
| `input.caret_color` | DIRECT | `visuals.text_cursor.stroke.color` |  |
| `input.disabled_background` | SCOPED | «input:disabled» `visuals.text_edit_bg_color` (when: stated; None copies input.background_color, §6.4) |  |
| `input.disabled_opacity` | SCOPED | «input» `visuals.disabled_alpha` |  |
| `input.disabled_text_color` | SCOPED | «input:disabled» `visuals.widgets.noninteractive.fg_stroke.color`; «input:disabled» `visuals.widgets.inactive.fg_stroke.color` |  |
| `input.focus_border_color` | DERIVED | → T18(h) |  |
| `input.font.color` | SCOPED | «input» `visuals.widgets.noninteractive.fg_stroke.color`; «input» `visuals.widgets.inactive.fg_stroke.color`; «input» `visuals.widgets.hovered.fg_stroke.color`; «input» `visuals.widgets.active.fg_stroke.color`; «input» `visuals.widgets.open.fg_stroke.color` |  |
| `input.font.defined_size` | UNMAPPABLE |  | `egui-limited`: egui: a stated-size unit beside FontId::size (§5.1) |
| `input.font.family` | UNMAPPABLE |  | `egui-limited`: a TextStyle key naming a FontFamily::Name (declined, §5.8 item 7) |
| `input.font.size` | SCOPED | «input» `text_styles[Body].size` |  |
| `input.font.style` | DERIVED | → T18(d) | probe `"italic"` |
| `input.font.weight` | DERIVED | → T18(c) | probe `700` |
| `input.hover_border_color` | SCOPED | «input» `visuals.widgets.hovered.bg_stroke.color` (when: stated; None copies input.border.color, §6.4); «input» `visuals.widgets.active.bg_stroke.color` (when: stated; None copies input.border.color, §6.4) |  |
| `input.min_height` | DERIVED | → T18(a) |  |
| `input.placeholder_color` | SCOPED | «input» `visuals.weak_text_color` |  |
| `input.selection_background` | SCOPED | «input» `visuals.selection.bg_fill` |  |
| `input.selection_text_color` | SCOPED | «input» `visuals.selection.stroke.color` |  |
| `list.alternate_row_background` | DIRECT | `visuals.faint_bg_color` |  |
| `list.background_color` | DERIVED | → T18(a) |  |
| `list.border.color` | DERIVED | → T18(a) |  |
| `list.border.corner_radius` | SCOPED | «list» `visuals.widgets.noninteractive.corner_radius` |  |
| `list.border.line_width` | SCOPED | «list» `visuals.widgets.noninteractive.bg_stroke.width` |  |
| `list.border.padding.bottom` | DERIVED | → T18(a) |  |
| `list.border.padding.left` | DERIVED | → T18(a) |  |
| `list.border.padding.right` | DERIVED | → T18(a) |  |
| `list.border.padding.top` | DERIVED | → T18(a) |  |
| `list.border.shadow_enabled` | UNMAPPABLE |  | `source-side gap`: native-theme: shadow offset, blur and spread in WidgetBorderSpec (§14 item 10) |
| `list.disabled_text_color` | SCOPED | «list:disabled» `visuals.widgets.noninteractive.fg_stroke.color` |  |
| `list.grid_color` | SCOPED | «list» `visuals.widgets.noninteractive.bg_stroke.color` |  |
| `list.header_background` | DERIVED | → T18(a) |  |
| `list.header_font.color` | SCOPED | «list» `visuals.widgets.active.fg_stroke.color` |  |
| `list.header_font.defined_size` | UNMAPPABLE |  | `egui-limited`: egui: a stated-size unit beside FontId::size (§5.1) |
| `list.header_font.family` | UNMAPPABLE |  | `egui-limited`: a TextStyle key naming a FontFamily::Name (declined, §5.8 item 7) |
| `list.header_font.size` | DERIVED | → T18(e) |  |
| `list.header_font.style` | DERIVED | → T18(a) | probe `"italic"` |
| `list.header_font.weight` | DERIVED | → T18(a) | probe `700` |
| `list.hover_background` | SCOPED | «list» `visuals.widgets.hovered.bg_fill`; «list» `visuals.widgets.active.bg_fill` |  |
| `list.hover_text_color` | DERIVED | → T18(a) |  |
| `list.item_font.color` | SCOPED | «list» `visuals.widgets.noninteractive.fg_stroke.color` |  |
| `list.item_font.defined_size` | UNMAPPABLE |  | `egui-limited`: egui: a stated-size unit beside FontId::size (§5.1) |
| `list.item_font.family` | UNMAPPABLE |  | `egui-limited`: a TextStyle key naming a FontFamily::Name (declined, §5.8 item 7) |
| `list.item_font.size` | SCOPED | «list» `text_styles[Body].size` |  |
| `list.item_font.style` | DERIVED | → T18(d) | probe `"italic"` |
| `list.item_font.weight` | DERIVED | → T18(c) | probe `700` |
| `list.row_height` | SCOPED | «list» `spacing.interact_size.y` |  |
| `list.selection_background` | SCOPED | «list» `visuals.selection.bg_fill` |  |
| `list.selection_text_color` | SCOPED | «list» `visuals.selection.stroke.color` |  |

### Indicators (scrollbar, slider, progress bar, splitter, separator, spinner)

| leaf | verdict | sinks, or the test of its route | note |
|---|---|---|---|
| `progress_bar.border.color` | UNMAPPABLE |  | `egui-limited`: Frame::stroke on a Frame laid round the bar (declined, §5.8 item 8) |
| `progress_bar.border.corner_radius` | DERIVED | → T18(a) |  |
| `progress_bar.border.line_width` | UNMAPPABLE |  | `egui-limited`: Frame::stroke on a Frame laid round the bar (declined, §5.8 item 8) |
| `progress_bar.border.padding.bottom` | UNMAPPABLE |  | `egui-limited`: egui: a progress-bar style (§14 item 36) |
| `progress_bar.border.padding.left` | SCOPED | «progress_bar» `spacing.item_spacing.x` | probe `5.0` |
| `progress_bar.border.padding.right` | UNMAPPABLE |  | `egui-limited`: egui: a progress-bar style (§14 item 36) |
| `progress_bar.border.padding.top` | UNMAPPABLE |  | `egui-limited`: egui: a progress-bar style (§14 item 36) |
| `progress_bar.border.shadow_enabled` | UNMAPPABLE |  | `source-side gap`: native-theme: shadow offset, blur and spread in WidgetBorderSpec (§14 item 10) |
| `progress_bar.fill_color` | SCOPED | «progress_bar» `visuals.selection.bg_fill` |  |
| `progress_bar.min_width` | DERIVED | → T18(a) |  |
| `progress_bar.track_color` | SCOPED | «progress_bar» `visuals.extreme_bg_color` |  |
| `progress_bar.track_height` | SCOPED | «progress_bar» `spacing.interact_size.y` |  |
| `scrollbar.groove_width` | DERIVED | `spacing.scroll.bar_width` (when: overlay mode, or a thumb wider than the groove, §6.5); `spacing.scroll.bar_inner_margin` (when: not overlay mode, §6.5); `spacing.scroll.bar_outer_margin` (when: not overlay mode, §6.5) |  |
| `scrollbar.min_thumb_length` | DIRECT | `spacing.scroll.handle_min_length` |  |
| `scrollbar.overlay_mode` | DERIVED | `spacing.scroll.floating`; `spacing.scroll.floating_allocated_width` (when: egui's own value is not 0.0, §6.5); `spacing.scroll.bar_width` (when: the thumb is narrower than the groove, §6.5); `spacing.scroll.bar_inner_margin` (when: the thumb is narrower than the groove, §6.5); `spacing.scroll.bar_outer_margin` (when: the thumb is narrower than the groove, §6.5); `spacing.scroll.floating_width` (when: egui's own floating width is not the thumb width, §6.5) |  |
| `scrollbar.thumb_active_color` | DIRECT | `visuals.widgets.active.bg_fill` (when: stated; None copies scrollbar.thumb_hover_color, §6.9); «scrollbar» `visuals.widgets.active.bg_fill` (when: stated; None copies scrollbar.thumb_hover_color, §6.9) |  |
| `scrollbar.thumb_color` | DIRECT | `visuals.widgets.inactive.bg_fill`; `visuals.widgets.open.bg_fill`; «scrollbar» `visuals.widgets.inactive.bg_fill`; «scrollbar» `visuals.widgets.open.bg_fill`; `spacing.scroll.foreground_color` (when: never: written false in the base style, so the handle reads bg_fill, §5.5, §5.9) |  |
| `scrollbar.thumb_hover_color` | DIRECT | `visuals.widgets.hovered.bg_fill`; «scrollbar» `visuals.widgets.hovered.bg_fill`; `visuals.widgets.active.bg_fill` (when: scrollbar.thumb_active_color is None, §6.9); «scrollbar» `visuals.widgets.active.bg_fill` (when: scrollbar.thumb_active_color is None, §6.9) |  |
| `scrollbar.thumb_width` | DERIVED | `spacing.scroll.bar_width` (when: not overlay mode, §6.5); `spacing.scroll.floating_width` (when: overlay mode, §6.5); `spacing.scroll.bar_inner_margin` (when: not overlay mode, §6.5); `spacing.scroll.bar_outer_margin` (when: not overlay mode, §6.5) |  |
| `scrollbar.track_color` | DIRECT | `visuals.extreme_bg_color`; «scrollbar» `visuals.extreme_bg_color` |  |
| `separator.line_color` | SCOPED | «separator» `visuals.widgets.noninteractive.bg_stroke.color` |  |
| `separator.line_width` | SCOPED | «separator» `visuals.widgets.noninteractive.bg_stroke.width` |  |
| `slider.disabled_fill_color` | SCOPED | «slider:disabled» `visuals.selection.bg_fill` (when: stated; None copies slider.fill_color, §6.4) |  |
| `slider.disabled_opacity` | SCOPED | «slider» `visuals.disabled_alpha` |  |
| `slider.disabled_thumb_color` | UNMAPPABLE |  | `widgets-crate`: native-theme-egui-widgets: Slider (docs/todo_egui-widgets-spec.md §4.2) |
| `slider.disabled_track_color` | SCOPED | «slider:disabled» `visuals.widgets.inactive.bg_fill` (when: stated; None copies slider.track_color, §6.4) |  |
| `slider.fill_color` | SCOPED | «slider» `visuals.selection.bg_fill`; «slider» `visuals.slider_trailing_fill` (when: never: written true in every slider cell, §5.9); «slider:disabled» `visuals.selection.bg_fill` (when: slider.disabled_fill_color is None, §6.4) |  |
| `slider.thumb_color` | SCOPED | «slider» `visuals.widgets.hovered.bg_fill` (when: slider.thumb_hover_color is None, §6.9); «slider» `visuals.widgets.active.bg_fill` (when: slider.thumb_hover_color is None, §6.9) |  |
| `slider.thumb_diameter` | DERIVED | «slider» `spacing.interact_size.y`; «slider» `visuals.widgets.noninteractive.expansion` (when: Body row \> 1.25 · thumb_diameter, §6.6); «slider» `visuals.widgets.inactive.expansion` (when: Body row \> 1.25 · thumb_diameter, §6.6); «slider» `visuals.widgets.hovered.expansion` (when: Body row \> 1.25 · thumb_diameter, §6.6); «slider» `visuals.widgets.active.expansion` (when: Body row \> 1.25 · thumb_diameter, §6.6); «slider» `visuals.widgets.open.expansion` (when: Body row \> 1.25 · thumb_diameter, §6.6); `visuals.handle_shape` (when: never: HandleShape::Circle in the base style and every slider cell, a diameter's knob, §5.9, §6.6) |  |
| `slider.thumb_hover_color` | SCOPED | «slider» `visuals.widgets.hovered.bg_fill` (when: stated; None copies slider.thumb_color, §6.9); «slider» `visuals.widgets.active.bg_fill` (when: stated; None copies slider.thumb_color, §6.9) |  |
| `slider.tick_mark_length` | UNMAPPABLE |  | `egui-limited`: egui: tick marks on Slider |
| `slider.track_color` | SCOPED | «slider» `visuals.widgets.inactive.bg_fill`; «slider» `visuals.widgets.open.bg_fill`; «slider:disabled» `visuals.widgets.inactive.bg_fill` (when: slider.disabled_track_color is None, §6.4) |  |
| `slider.track_height` | DIRECT | `spacing.slider_rail_height`; «slider» `visuals.widgets.inactive.corner_radius` (when: defaults.border.corner_radius exceeds half the rail height, §6.8) |  |
| `spinner.diameter` | DERIVED | «spinner» `spacing.interact_size.y` (when: spinner.diameter is at least spinner.min_diameter, §6.7) |  |
| `spinner.fill_color` | SCOPED | «spinner» `visuals.widgets.active.fg_stroke.color` |  |
| `spinner.min_diameter` | DERIVED | «spinner» `spacing.interact_size.y` (when: spinner.min_diameter exceeds spinner.diameter, §6.7) |  |
| `spinner.stroke_width` | UNMAPPABLE |  | `widgets-crate`: native-theme-egui-widgets: Spinner (docs/todo_egui-widgets-spec.md §4.3) |
| `splitter.divider_color` | SCOPED | «splitter» `visuals.widgets.noninteractive.bg_stroke.color` |  |
| `splitter.divider_width` | SCOPED | «splitter» `visuals.widgets.noninteractive.bg_stroke.width`; «splitter» `visuals.widgets.hovered.fg_stroke.width`; «splitter» `visuals.widgets.active.fg_stroke.width` |  |
| `splitter.hover_color` | SCOPED | «splitter» `visuals.widgets.hovered.fg_stroke.color`; «splitter» `visuals.widgets.active.fg_stroke.color` |  |

### Chrome (tab, sidebar, toolbar, status bar, expander)

| leaf | verdict | sinks, or the test of its route | note |
|---|---|---|---|
| `expander.arrow_color` | DERIVED | → T18(f) |  |
| `expander.arrow_icon_size` | DERIVED | «expander» `spacing.icon_width_inner` |  |
| `expander.border.color` | SCOPED | «expander» `visuals.widgets.noninteractive.bg_stroke.color`; «expander» `visuals.widgets.inactive.bg_stroke.color`; «expander» `visuals.widgets.hovered.bg_stroke.color`; «expander» `visuals.widgets.active.bg_stroke.color`; «expander» `visuals.widgets.open.bg_stroke.color` |  |
| `expander.border.corner_radius` | SCOPED | «expander» `visuals.widgets.noninteractive.corner_radius`; «expander» `visuals.widgets.inactive.corner_radius`; «expander» `visuals.widgets.hovered.corner_radius`; «expander» `visuals.widgets.active.corner_radius`; «expander» `visuals.widgets.open.corner_radius` |  |
| `expander.border.line_width` | SCOPED | «expander» `visuals.widgets.noninteractive.bg_stroke.width`; «expander» `visuals.widgets.inactive.bg_stroke.width`; «expander» `visuals.widgets.hovered.bg_stroke.width`; «expander» `visuals.widgets.active.bg_stroke.width`; «expander» `visuals.widgets.open.bg_stroke.width`; «expander» `spacing.button_padding.x` (when: a side of the pair is stated, §5 intro); «expander» `spacing.button_padding.y` (when: a side of the pair is stated, §5 intro) |  |
| `expander.border.padding.bottom` | DERIVED | «expander» `spacing.button_padding.y` |  |
| `expander.border.padding.left` | UNMAPPABLE |  | `egui-limited`: egui: a per-widget Margin (§14 item 35) |
| `expander.border.padding.right` | DERIVED | «expander» `spacing.button_padding.x` |  |
| `expander.border.padding.top` | DERIVED | «expander» `spacing.button_padding.y` |  |
| `expander.border.shadow_enabled` | UNMAPPABLE |  | `source-side gap`: native-theme: shadow offset, blur and spread in WidgetBorderSpec (§14 item 10) |
| `expander.font.color` | SCOPED | «expander» `visuals.widgets.noninteractive.fg_stroke.color`; «expander» `visuals.widgets.inactive.fg_stroke.color`; «expander» `visuals.widgets.hovered.fg_stroke.color`; «expander» `visuals.widgets.active.fg_stroke.color`; «expander» `visuals.widgets.open.fg_stroke.color` |  |
| `expander.font.defined_size` | UNMAPPABLE |  | `egui-limited`: egui: a stated-size unit beside FontId::size (§5.1) |
| `expander.font.family` | UNMAPPABLE |  | `egui-limited`: a TextStyle key naming a FontFamily::Name (declined, §5.8 item 7) |
| `expander.font.size` | SCOPED | «expander» `text_styles[Button].size` |  |
| `expander.font.style` | DERIVED | → T18(d) | probe `"italic"` |
| `expander.font.weight` | DERIVED | → T18(c) | probe `700` |
| `expander.header_height` | SCOPED | «expander» `spacing.interact_size.y` |  |
| `expander.hover_background` | SCOPED | «expander» `visuals.widgets.hovered.weak_bg_fill` (when: stated; None leaves no highlight, §6.4); «expander» `visuals.widgets.active.weak_bg_fill` (when: stated; None leaves no highlight, §6.4); «expander» `visuals.collapsing_header_frame` (when: never: written true in every expander cell, the frame the hover fill lies in, §6.1) |  |
| `sidebar.background_color` | SCOPED | \[panel_left\] `fill`; \[panel_right\] `fill`; «sidebar» `visuals.panel_fill` |  |
| `sidebar.border.color` | SCOPED | «sidebar» `visuals.widgets.noninteractive.bg_stroke.color` |  |
| `sidebar.border.corner_radius` | SCOPED | \[panel_left\] `corner_radius`; \[panel_right\] `corner_radius` | probe `4.0` |
| `sidebar.border.line_width` | SCOPED | «sidebar» `visuals.widgets.noninteractive.bg_stroke.width` |  |
| `sidebar.border.padding.bottom` | SCOPED | \[panel_left\] `inner_margin.bottom`; \[panel_right\] `inner_margin.bottom` | probe `5.0` |
| `sidebar.border.padding.left` | SCOPED | \[panel_left\] `inner_margin.left`; \[panel_right\] `inner_margin.left` | probe `5.0` |
| `sidebar.border.padding.right` | SCOPED | \[panel_left\] `inner_margin.right`; \[panel_right\] `inner_margin.right` | probe `5.0` |
| `sidebar.border.padding.top` | SCOPED | \[panel_left\] `inner_margin.top`; \[panel_right\] `inner_margin.top` | probe `5.0` |
| `sidebar.border.shadow_enabled` | UNMAPPABLE |  | `source-side gap`: native-theme: shadow offset, blur and spread in WidgetBorderSpec (§14 item 10) |
| `sidebar.font.color` | SCOPED | «sidebar» `visuals.widgets.noninteractive.fg_stroke.color`; «sidebar» `visuals.widgets.inactive.fg_stroke.color`; «sidebar» `visuals.widgets.hovered.fg_stroke.color`; «sidebar» `visuals.widgets.active.fg_stroke.color`; «sidebar» `visuals.widgets.open.fg_stroke.color` |  |
| `sidebar.font.defined_size` | UNMAPPABLE |  | `egui-limited`: egui: a stated-size unit beside FontId::size (§5.1) |
| `sidebar.font.family` | UNMAPPABLE |  | `egui-limited`: a TextStyle key naming a FontFamily::Name (declined, §5.8 item 7) |
| `sidebar.font.size` | SCOPED | «sidebar» `text_styles[Body].size` |  |
| `sidebar.font.style` | DERIVED | → T18(d) | probe `"italic"` |
| `sidebar.font.weight` | DERIVED | → T18(c) | probe `700` |
| `sidebar.hover_background` | SCOPED | «sidebar» `visuals.widgets.hovered.weak_bg_fill`; «sidebar» `visuals.widgets.active.weak_bg_fill` |  |
| `sidebar.selection_background` | SCOPED | «sidebar» `visuals.selection.bg_fill` |  |
| `sidebar.selection_text_color` | SCOPED | «sidebar» `visuals.selection.stroke.color` |  |
| `status_bar.background_color` | SCOPED | \[panel_bottom\] `fill`; «status_bar» `visuals.panel_fill` |  |
| `status_bar.border.color` | SCOPED | «status_bar» `visuals.widgets.noninteractive.bg_stroke.color` |  |
| `status_bar.border.corner_radius` | SCOPED | \[panel_bottom\] `corner_radius` | probe `4.0` |
| `status_bar.border.line_width` | SCOPED | «status_bar» `visuals.widgets.noninteractive.bg_stroke.width` |  |
| `status_bar.border.padding.bottom` | SCOPED | \[panel_bottom\] `inner_margin.bottom` |  |
| `status_bar.border.padding.left` | SCOPED | \[panel_bottom\] `inner_margin.left` |  |
| `status_bar.border.padding.right` | SCOPED | \[panel_bottom\] `inner_margin.right` |  |
| `status_bar.border.padding.top` | SCOPED | \[panel_bottom\] `inner_margin.top` |  |
| `status_bar.border.shadow_enabled` | UNMAPPABLE |  | `source-side gap`: native-theme: shadow offset, blur and spread in WidgetBorderSpec (§14 item 10) |
| `status_bar.font.color` | SCOPED | «status_bar» `visuals.widgets.noninteractive.fg_stroke.color`; «status_bar» `visuals.widgets.inactive.fg_stroke.color`; «status_bar» `visuals.widgets.hovered.fg_stroke.color`; «status_bar» `visuals.widgets.active.fg_stroke.color`; «status_bar» `visuals.widgets.open.fg_stroke.color` |  |
| `status_bar.font.defined_size` | UNMAPPABLE |  | `egui-limited`: egui: a stated-size unit beside FontId::size (§5.1) |
| `status_bar.font.family` | UNMAPPABLE |  | `egui-limited`: a TextStyle key naming a FontFamily::Name (declined, §5.8 item 7) |
| `status_bar.font.size` | SCOPED | «status_bar» `text_styles[Body].size` |  |
| `status_bar.font.style` | DERIVED | → T18(d) | probe `"italic"` |
| `status_bar.font.weight` | DERIVED | → T18(c) | probe `700` |
| `tab.active_background` | SCOPED | «tab» `visuals.selection.bg_fill` |  |
| `tab.active_text_color` | SCOPED | «tab» `visuals.selection.stroke.color` |  |
| `tab.background_color` | SCOPED | «tab» `visuals.widgets.inactive.weak_bg_fill`; «tab» `visuals.widgets.open.weak_bg_fill`; «tab» `visuals.widgets.hovered.weak_bg_fill` (when: tab.hover_background is translucent or None, §6.1, §6.4); «tab» `visuals.widgets.active.weak_bg_fill` (when: tab.hover_background is translucent or None, §6.1, §6.4) |  |
| `tab.bar_background` | SCOPED | «tab» `visuals.panel_fill` |  |
| `tab.border.color` | SCOPED | «tab» `visuals.widgets.noninteractive.bg_stroke.color`; «tab» `visuals.widgets.inactive.bg_stroke.color`; «tab» `visuals.widgets.hovered.bg_stroke.color`; «tab» `visuals.widgets.active.bg_stroke.color`; «tab» `visuals.widgets.open.bg_stroke.color` | probe `"#ff000080"` |
| `tab.border.corner_radius` | SCOPED | «tab» `visuals.widgets.noninteractive.corner_radius`; «tab» `visuals.widgets.inactive.corner_radius`; «tab» `visuals.widgets.hovered.corner_radius`; «tab» `visuals.widgets.active.corner_radius`; «tab» `visuals.widgets.open.corner_radius` | probe `4.0` |
| `tab.border.line_width` | SCOPED | «tab» `visuals.widgets.noninteractive.bg_stroke.width`; «tab» `visuals.widgets.inactive.bg_stroke.width`; «tab» `visuals.widgets.hovered.bg_stroke.width`; «tab» `visuals.widgets.active.bg_stroke.width`; «tab» `visuals.widgets.open.bg_stroke.width`; «tab» `spacing.button_padding.x` (when: a side of the pair is stated, §5 intro); «tab» `spacing.button_padding.y` (when: a side of the pair is stated, §5 intro) | probe `2.0` |
| `tab.border.padding.bottom` | DERIVED | «tab» `spacing.button_padding.y` |  |
| `tab.border.padding.left` | DERIVED | «tab» `spacing.button_padding.x` |  |
| `tab.border.padding.right` | DERIVED | «tab» `spacing.button_padding.x` |  |
| `tab.border.padding.top` | DERIVED | «tab» `spacing.button_padding.y` |  |
| `tab.border.shadow_enabled` | UNMAPPABLE |  | `source-side gap`: native-theme: shadow offset, blur and spread in WidgetBorderSpec (§14 item 10) |
| `tab.font.color` | SCOPED | «tab» `visuals.widgets.noninteractive.fg_stroke.color`; «tab» `visuals.widgets.inactive.fg_stroke.color`; «tab» `visuals.widgets.open.fg_stroke.color` |  |
| `tab.font.defined_size` | UNMAPPABLE |  | `egui-limited`: egui: a stated-size unit beside FontId::size (§5.1) |
| `tab.font.family` | UNMAPPABLE |  | `egui-limited`: a TextStyle key naming a FontFamily::Name (declined, §5.8 item 7) |
| `tab.font.size` | SCOPED | «tab» `text_styles[Body].size` |  |
| `tab.font.style` | DERIVED | → T18(d) | probe `"italic"` |
| `tab.font.weight` | DERIVED | → T18(c) | probe `700` |
| `tab.hover_background` | SCOPED | «tab» `visuals.widgets.hovered.weak_bg_fill` (when: stated; None copies tab.background_color, §6.4); «tab» `visuals.widgets.active.weak_bg_fill` (when: stated; None copies tab.background_color, §6.4) |  |
| `tab.hover_text_color` | SCOPED | «tab» `visuals.widgets.hovered.fg_stroke.color`; «tab» `visuals.widgets.active.fg_stroke.color` |  |
| `tab.min_height` | SCOPED | «tab» `spacing.interact_size.y` |  |
| `tab.min_width` | DERIVED | → T18(a) |  |
| `toolbar.background_color` | SCOPED | \[panel_top\] `fill`; «toolbar» `visuals.panel_fill` |  |
| `toolbar.bar_height` | DERIVED | «toolbar» `spacing.interact_size.y` |  |
| `toolbar.border.color` | SCOPED | «toolbar» `visuals.widgets.noninteractive.bg_stroke.color` |  |
| `toolbar.border.corner_radius` | SCOPED | \[panel_top\] `corner_radius` |  |
| `toolbar.border.line_width` | SCOPED | «toolbar» `visuals.widgets.noninteractive.bg_stroke.width`; «toolbar» `spacing.interact_size.y` (when: toolbar.bar_height is stated, §6.10) |  |
| `toolbar.border.padding.bottom` | SCOPED | \[panel_top\] `inner_margin.bottom`; «toolbar» `spacing.interact_size.y` (when: toolbar.bar_height is stated, §6.10) |  |
| `toolbar.border.padding.left` | SCOPED | \[panel_top\] `inner_margin.left` |  |
| `toolbar.border.padding.right` | SCOPED | \[panel_top\] `inner_margin.right` |  |
| `toolbar.border.padding.top` | SCOPED | \[panel_top\] `inner_margin.top`; «toolbar» `spacing.interact_size.y` (when: toolbar.bar_height is stated, §6.10) |  |
| `toolbar.border.shadow_enabled` | UNMAPPABLE |  | `source-side gap`: native-theme: shadow offset, blur and spread in WidgetBorderSpec (§14 item 10) |
| `toolbar.font.color` | SCOPED | «toolbar» `visuals.widgets.noninteractive.fg_stroke.color`; «toolbar» `visuals.widgets.inactive.fg_stroke.color`; «toolbar» `visuals.widgets.hovered.fg_stroke.color`; «toolbar» `visuals.widgets.active.fg_stroke.color`; «toolbar» `visuals.widgets.open.fg_stroke.color` |  |
| `toolbar.font.defined_size` | UNMAPPABLE |  | `egui-limited`: egui: a stated-size unit beside FontId::size (§5.1) |
| `toolbar.font.family` | UNMAPPABLE |  | `egui-limited`: a TextStyle key naming a FontFamily::Name (declined, §5.8 item 7) |
| `toolbar.font.size` | SCOPED | «toolbar» `text_styles[Body].size` |  |
| `toolbar.font.style` | DERIVED | → T18(d) | probe `"italic"` |
| `toolbar.font.weight` | DERIVED | → T18(c) | probe `700` |
| `toolbar.icon_size` | DERIVED | → T18(a) |  |
| `toolbar.item_gap` | SCOPED | «toolbar» `spacing.item_spacing.x` |  |

## Base owners (§5.9)

The leaf that wins an egui field globally, and the leaves that displace it inside a role scope or a `Surface` frame.

| egui field | base owner | displaced to a scope or a surface |
|---|---|---|
| `corner_radius` | — (egui's own value stands) | `card.border.corner_radius` \[card\], `dialog.border.corner_radius` \[dialog\], `popover.border.corner_radius` \[popover\], `sidebar.border.corner_radius` \[panel_left\], `sidebar.border.corner_radius` \[panel_right\], `status_bar.border.corner_radius` \[panel_bottom\], `toolbar.border.corner_radius` \[panel_top\], `tooltip.border.corner_radius` \[tooltip\] |
| `fill` | — (egui's own value stands) | `card.background_color` \[card\], `dialog.background_color` \[dialog\], `popover.background_color` \[popover\], `sidebar.background_color` \[panel_left\], `sidebar.background_color` \[panel_right\], `status_bar.background_color` \[panel_bottom\], `toolbar.background_color` \[panel_top\], `tooltip.background_color` \[tooltip\], `window.background_color` \[window\], `window.inactive_title_bar_background` \[window_title_bar\] |
| `inner_margin.bottom` | — (egui's own value stands) | `card.border.padding.bottom` \[card\], `dialog.border.padding.bottom` \[dialog\], `layout.window_margin` \[central_panel\], `popover.border.padding.bottom` \[popover\], `sidebar.border.padding.bottom` \[panel_left\], `sidebar.border.padding.bottom` \[panel_right\], `status_bar.border.padding.bottom` \[panel_bottom\], `toolbar.border.padding.bottom` \[panel_top\], `tooltip.border.padding.bottom` \[tooltip\] |
| `inner_margin.left` | — (egui's own value stands) | `card.border.padding.left` \[card\], `dialog.border.padding.left` \[dialog\], `layout.window_margin` \[central_panel\], `popover.border.padding.left` \[popover\], `sidebar.border.padding.left` \[panel_left\], `sidebar.border.padding.left` \[panel_right\], `status_bar.border.padding.left` \[panel_bottom\], `toolbar.border.padding.left` \[panel_top\], `tooltip.border.padding.left` \[tooltip\] |
| `inner_margin.right` | — (egui's own value stands) | `card.border.padding.right` \[card\], `dialog.border.padding.right` \[dialog\], `layout.window_margin` \[central_panel\], `popover.border.padding.right` \[popover\], `sidebar.border.padding.right` \[panel_left\], `sidebar.border.padding.right` \[panel_right\], `status_bar.border.padding.right` \[panel_bottom\], `toolbar.border.padding.right` \[panel_top\], `tooltip.border.padding.right` \[tooltip\] |
| `inner_margin.top` | — (egui's own value stands) | `card.border.padding.top` \[card\], `dialog.border.padding.top` \[dialog\], `layout.window_margin` \[central_panel\], `popover.border.padding.top` \[popover\], `sidebar.border.padding.top` \[panel_left\], `sidebar.border.padding.top` \[panel_right\], `status_bar.border.padding.top` \[panel_bottom\], `toolbar.border.padding.top` \[panel_top\], `tooltip.border.padding.top` \[tooltip\] |
| `override_font_id.size` | — (egui's own value stands) | `button.font.size` «button», `checkbox.font.size` «checkbox», `link.font.size` «link», `segmented_control.font.size` «segmented_control» |
| `shadow.blur` | — (egui's own value stands) | `dialog.border.shadow_enabled` \[dialog\], `popover.border.shadow_enabled` \[popover\], `tooltip.border.shadow_enabled` \[tooltip\] |
| `shadow.color` | — (egui's own value stands) | `defaults.shadow_color` \[dialog\], `defaults.shadow_color` \[popover\], `defaults.shadow_color` \[tooltip\], `dialog.border.shadow_enabled` \[dialog\], `popover.border.shadow_enabled` \[popover\], `tooltip.border.shadow_enabled` \[tooltip\] |
| `shadow.offset` | — (egui's own value stands) | `dialog.border.shadow_enabled` \[dialog\], `popover.border.shadow_enabled` \[popover\], `tooltip.border.shadow_enabled` \[tooltip\] |
| `shadow.spread` | — (egui's own value stands) | `dialog.border.shadow_enabled` \[dialog\], `popover.border.shadow_enabled` \[popover\], `tooltip.border.shadow_enabled` \[tooltip\] |
| `spacing.button_padding.x` | `button.border.line_width`, `button.border.padding.left`, `button.border.padding.right` | `button.border.line_width` «button», `button.border.padding.left` «button», `button.border.padding.right` «button», `combo_box.border.line_width` «combo_box», `combo_box.border.padding.left` «combo_box», `combo_box.border.padding.right` «combo_box», `expander.border.line_width` «expander», `expander.border.padding.right` «expander», `menu.border.line_width` «menu», `menu.border.padding.left` «menu», `menu.border.padding.right` «menu», `segmented_control.border.line_width` «segmented_control», `segmented_control.border.padding.left` «segmented_control», `segmented_control.border.padding.right` «segmented_control», `tab.border.line_width` «tab», `tab.border.padding.left` «tab», `tab.border.padding.right` «tab» |
| `spacing.button_padding.y` | `button.border.line_width`, `button.border.padding.bottom`, `button.border.padding.top` | `button.border.line_width` «button», `button.border.padding.bottom` «button», `button.border.padding.top` «button», `combo_box.border.line_width` «combo_box», `combo_box.border.padding.bottom` «combo_box», `combo_box.border.padding.top` «combo_box», `expander.border.line_width` «expander», `expander.border.padding.bottom` «expander», `expander.border.padding.top` «expander», `menu.border.line_width` «menu», `menu.border.padding.bottom` «menu», `menu.border.padding.top` «menu», `segmented_control.border.line_width` «segmented_control», `segmented_control.border.padding.bottom` «segmented_control», `segmented_control.border.padding.top` «segmented_control», `tab.border.line_width` «tab», `tab.border.padding.bottom` «tab», `tab.border.padding.top` «tab» |
| `spacing.combo_width` | `combo_box.min_width` |  |
| `spacing.extra_text_line_spacing` | `defaults.font.size`, `defaults.line_height` |  |
| `spacing.icon_spacing` | `checkbox.label_gap` | `button.icon_text_gap` «button», `checkbox.label_gap` «checkbox», `combo_box.arrow_area_width` «combo_box», `combo_box.arrow_icon_size` «combo_box», `menu.icon_text_gap` «menu» |
| `spacing.icon_width` | `checkbox.indicator_width` | `checkbox.indicator_width` «checkbox», `combo_box.arrow_icon_size` «combo_box» |
| `spacing.icon_width_inner` | — (egui's own value stands) | `checkbox.border.padding.bottom` «checkbox», `checkbox.border.padding.left` «checkbox», `checkbox.border.padding.right` «checkbox», `checkbox.border.padding.top` «checkbox», `checkbox.indicator_width` «checkbox», `expander.arrow_icon_size` «expander» |
| `spacing.interact_size.y` | `button.min_height` | `button.min_height` «button», `combo_box.min_height` «combo_box», `expander.header_height` «expander», `list.row_height` «list», `menu.row_height` «menu», `progress_bar.track_height` «progress_bar», `segmented_control.segment_height` «segmented_control», `slider.thumb_diameter` «slider», `spinner.diameter` «spinner», `spinner.min_diameter` «spinner», `switch.track_height` «switch», `tab.min_height` «tab», `toolbar.bar_height` «toolbar», `toolbar.border.line_width` «toolbar», `toolbar.border.padding.bottom` «toolbar», `toolbar.border.padding.top` «toolbar» |
| `spacing.item_spacing.x` | `layout.widget_gap` | `dialog.button_gap` «dialog», `progress_bar.border.padding.left` «progress_bar», `segmented_control.separator_width` «segmented_control», `toolbar.item_gap` «toolbar» |
| `spacing.item_spacing.y` | `layout.widget_gap` |  |
| `spacing.scroll.bar_inner_margin` | `scrollbar.groove_width`, `scrollbar.overlay_mode`, `scrollbar.thumb_width` |  |
| `spacing.scroll.bar_outer_margin` | `scrollbar.groove_width`, `scrollbar.overlay_mode`, `scrollbar.thumb_width` |  |
| `spacing.scroll.bar_width` | `scrollbar.groove_width`, `scrollbar.overlay_mode`, `scrollbar.thumb_width` |  |
| `spacing.scroll.floating` | `scrollbar.overlay_mode` |  |
| `spacing.scroll.floating_allocated_width` | `scrollbar.overlay_mode` |  |
| `spacing.scroll.floating_width` | `scrollbar.overlay_mode`, `scrollbar.thumb_width` |  |
| `spacing.scroll.foreground_color` | `scrollbar.thumb_color` |  |
| `spacing.scroll.handle_min_length` | `scrollbar.min_thumb_length` |  |
| `spacing.slider_rail_height` | `slider.track_height` |  |
| `spacing.tooltip_width` | `tooltip.max_width` |  |
| `spacing.window_margin.bottom` | `window.border.padding.bottom` |  |
| `spacing.window_margin.left` | `window.border.padding.left` |  |
| `spacing.window_margin.right` | `window.border.padding.right` |  |
| `spacing.window_margin.top` | `window.border.padding.top` |  |
| `stroke.color` | — (egui's own value stands) | `card.border.color` \[card\], `defaults.border.opacity` \[window\], `defaults.border.opacity` \[dialog\], `defaults.border.opacity` \[popover\], `defaults.border.opacity` \[card\], `defaults.border.opacity` \[tooltip\], `dialog.border.color` \[dialog\], `popover.border.color` \[popover\], `tooltip.border.color` \[tooltip\], `window.border.color` \[window\] |
| `stroke.width` | — (egui's own value stands) | `card.border.line_width` \[card\], `dialog.border.line_width` \[dialog\], `popover.border.line_width` \[popover\], `tooltip.border.line_width` \[tooltip\], `window.border.line_width` \[window\] |
| `text_styles[Body].size` | `defaults.font.size` | `dialog.body_font.size` «dialog», `input.font.size` «input», `list.item_font.size` «list», `menu.font.size` «menu», `popover.font.size` «popover», `sidebar.font.size` «sidebar», `status_bar.font.size` «status_bar», `tab.font.size` «tab», `toolbar.font.size` «toolbar», `tooltip.font.size` «tooltip» |
| `text_styles[Button].size` | `button.font.size` | `combo_box.font.size` «combo_box», `expander.font.size` «expander» |
| `text_styles[Heading].size` | `text_scale.section_heading.size` | `dialog.title_font.size` «dialog» |
| `text_styles[Monospace].size` | `defaults.mono_font.size` |  |
| `text_styles[Small].size` | `text_scale.caption.size` |  |
| `visuals.collapsing_header_frame` | — (egui's own value stands) | `expander.hover_background` «expander» |
| `visuals.disabled_alpha` | `defaults.disabled_opacity` | `button.disabled_opacity` «button», `checkbox.disabled_opacity` «checkbox», `combo_box.disabled_opacity` «combo_box», `input.disabled_opacity` «input», `segmented_control.disabled_opacity` «segmented_control», `slider.disabled_opacity` «slider», `switch.disabled_opacity` «switch» |
| `visuals.error_fg_color` | `defaults.danger_color` |  |
| `visuals.extreme_bg_color` | `scrollbar.track_color` | `progress_bar.track_color` «progress_bar», `scrollbar.track_color` «scrollbar» |
| `visuals.faint_bg_color` | `list.alternate_row_background` |  |
| `visuals.handle_shape` | `slider.thumb_diameter` |  |
| `visuals.hyperlink_color` | `defaults.link_color` | `link.disabled_text_color` «link:disabled», `link.font.color` «link» |
| `visuals.menu_corner_radius` | `defaults.border.corner_radius_lg` |  |
| `visuals.override_text_color` | — (egui's own value stands) | `checkbox.disabled_text_color` «checkbox:disabled», `checkbox.font.color` «checkbox» |
| `visuals.panel_fill` | `defaults.background_color` | `sidebar.background_color` «sidebar», `status_bar.background_color` «status_bar», `tab.bar_background` «tab», `toolbar.background_color` «toolbar» |
| `visuals.popup_shadow.blur` | `defaults.border.shadow_enabled` |  |
| `visuals.popup_shadow.color` | `defaults.border.shadow_enabled`, `defaults.shadow_color` |  |
| `visuals.popup_shadow.offset` | `defaults.border.shadow_enabled` |  |
| `visuals.popup_shadow.spread` | `defaults.border.shadow_enabled` |  |
| `visuals.selection.bg_fill` | `defaults.selection_background` | `button.background_color` «button:disabled», `button.disabled_background` «button:disabled», `button.primary_background` «button», `input.selection_background` «input», `list.selection_background` «list», `progress_bar.fill_color` «progress_bar», `segmented_control.active_background` «segmented_control», `sidebar.selection_background` «sidebar», `slider.disabled_fill_color` «slider:disabled», `slider.fill_color` «slider», `slider.fill_color` «slider:disabled», `switch.checked_background` «switch», `switch.checked_background` «switch:disabled», `switch.disabled_checked_background` «switch:disabled», `tab.active_background` «tab» |
| `visuals.selection.stroke.color` | `defaults.selection_text_color` | `button.disabled_text_color` «button:disabled», `button.primary_text_color` «button», `input.selection_text_color` «input», `list.selection_text_color` «list», `segmented_control.active_text_color` «segmented_control», `sidebar.selection_text_color` «sidebar», `tab.active_text_color` «tab» |
| `visuals.slider_trailing_fill` | — (egui's own value stands) | `slider.fill_color` «slider» |
| `visuals.text_cursor.stroke.color` | `input.caret_color` |  |
| `visuals.text_edit_bg_color` | `input.background_color` | `input.background_color` «input:disabled», `input.disabled_background` «input:disabled» |
| `visuals.warn_fg_color` | `defaults.warning_color` |  |
| `visuals.weak_text_color` | `defaults.muted_color` | `input.placeholder_color` «input» |
| `visuals.widgets.active.bg_fill` | `scrollbar.thumb_active_color`, `scrollbar.thumb_hover_color` | `checkbox.background_color` «checkbox», `checkbox.checked_background` «checkbox:selected», `checkbox.hover_background` «checkbox», `checkbox.unchecked_background` «checkbox», `list.hover_background` «list», `scrollbar.thumb_active_color` «scrollbar», `scrollbar.thumb_hover_color` «scrollbar», `slider.thumb_color` «slider», `slider.thumb_hover_color` «slider» |
| `visuals.widgets.active.bg_stroke.color` | `button.border.color`, `defaults.border.opacity` | `button.border.color` «button», `checkbox.border.color` «checkbox:selected», `checkbox.border.color` «checkbox», `checkbox.unchecked_border_color` «checkbox», `combo_box.border.color` «combo_box», `defaults.border.opacity` «checkbox», `defaults.border.opacity` «input», `defaults.border.opacity` «menu», `defaults.border.opacity` «button», `defaults.border.opacity` «checkbox:selected», `defaults.border.opacity` «segmented_control», `defaults.border.opacity` «combo_box», `defaults.border.opacity` «tab», `defaults.border.opacity` «expander», `expander.border.color` «expander», `input.border.color` «input», `input.hover_border_color` «input», `menu.border.color` «menu», `segmented_control.border.color` «segmented_control», `tab.border.color` «tab» |
| `visuals.widgets.active.bg_stroke.width` | `button.border.line_width` | `button.border.line_width` «button», `checkbox.border.line_width` «checkbox», `combo_box.border.line_width` «combo_box», `expander.border.line_width` «expander», `input.border.line_width` «input», `menu.border.line_width` «menu», `segmented_control.border.line_width` «segmented_control», `tab.border.line_width` «tab» |
| `visuals.widgets.active.corner_radius` | `button.border.corner_radius` | `button.border.corner_radius` «button», `checkbox.border.corner_radius` «checkbox», `combo_box.border.corner_radius` «combo_box», `defaults.border.corner_radius` «scrollbar», `expander.border.corner_radius` «expander», `input.border.corner_radius` «input», `menu.border.corner_radius` «menu», `segmented_control.border.corner_radius` «segmented_control», `switch.track_radius` «switch», `tab.border.corner_radius` «tab» |
| `visuals.widgets.active.expansion` | — (egui's own value stands) | `defaults.font.size` «slider», `slider.thumb_diameter` «slider» |
| `visuals.widgets.active.fg_stroke.color` | `button.active_text_color` | `button.active_text_color` «button», `checkbox.indicator_color` «checkbox», `combo_box.font.color` «combo_box», `expander.font.color` «expander», `input.font.color` «input», `list.header_font.color` «list», `menu.hover_text_color` «menu», `segmented_control.font.color` «segmented_control», `sidebar.font.color` «sidebar», `spinner.fill_color` «spinner», `splitter.hover_color` «splitter», `status_bar.font.color` «status_bar», `tab.hover_text_color` «tab», `toolbar.font.color` «toolbar» |
| `visuals.widgets.active.fg_stroke.width` | — (egui's own value stands) | `splitter.divider_width` «splitter» |
| `visuals.widgets.active.weak_bg_fill` | `button.active_background`, `button.background_color`, `button.hover_background` | `button.active_background` «button», `button.background_color` «button», `button.hover_background` «button», `combo_box.background_color` «combo_box», `combo_box.hover_background` «combo_box», `expander.hover_background` «expander», `menu.hover_background` «menu», `segmented_control.background_color` «segmented_control», `segmented_control.hover_background` «segmented_control», `sidebar.hover_background` «sidebar», `switch.hover_unchecked_background` «switch», `switch.unchecked_background` «switch», `tab.background_color` «tab», `tab.hover_background` «tab» |
| `visuals.widgets.hovered.bg_fill` | `scrollbar.thumb_hover_color` | `checkbox.background_color` «checkbox», `checkbox.checked_background` «checkbox:selected», `checkbox.hover_background` «checkbox», `checkbox.unchecked_background` «checkbox», `list.hover_background` «list», `scrollbar.thumb_hover_color` «scrollbar», `slider.thumb_color` «slider», `slider.thumb_hover_color` «slider» |
| `visuals.widgets.hovered.bg_stroke.color` | `button.border.color`, `defaults.border.opacity` | `button.border.color` «button», `checkbox.border.color` «checkbox:selected», `checkbox.border.color` «checkbox», `checkbox.unchecked_border_color` «checkbox», `combo_box.border.color` «combo_box», `defaults.border.opacity` «checkbox», `defaults.border.opacity` «input», `defaults.border.opacity` «menu», `defaults.border.opacity` «button», `defaults.border.opacity` «checkbox:selected», `defaults.border.opacity` «segmented_control», `defaults.border.opacity` «combo_box», `defaults.border.opacity` «tab», `defaults.border.opacity` «expander», `expander.border.color` «expander», `input.border.color` «input», `input.hover_border_color` «input», `menu.border.color` «menu», `segmented_control.border.color` «segmented_control», `tab.border.color` «tab» |
| `visuals.widgets.hovered.bg_stroke.width` | `button.border.line_width` | `button.border.line_width` «button», `checkbox.border.line_width` «checkbox», `combo_box.border.line_width` «combo_box», `expander.border.line_width` «expander», `input.border.line_width` «input», `menu.border.line_width` «menu», `segmented_control.border.line_width` «segmented_control», `tab.border.line_width` «tab» |
| `visuals.widgets.hovered.corner_radius` | `button.border.corner_radius` | `button.border.corner_radius` «button», `checkbox.border.corner_radius` «checkbox», `combo_box.border.corner_radius` «combo_box», `defaults.border.corner_radius` «scrollbar», `expander.border.corner_radius` «expander», `input.border.corner_radius` «input», `menu.border.corner_radius` «menu», `segmented_control.border.corner_radius` «segmented_control», `switch.track_radius` «switch», `tab.border.corner_radius` «tab» |
| `visuals.widgets.hovered.expansion` | — (egui's own value stands) | `defaults.font.size` «slider», `slider.thumb_diameter` «slider» |
| `visuals.widgets.hovered.fg_stroke.color` | `button.hover_text_color` | `button.hover_text_color` «button», `checkbox.indicator_color` «checkbox», `combo_box.font.color` «combo_box», `expander.font.color` «expander», `input.font.color` «input», `menu.hover_text_color` «menu», `segmented_control.font.color` «segmented_control», `sidebar.font.color` «sidebar», `splitter.hover_color` «splitter», `status_bar.font.color` «status_bar», `tab.hover_text_color` «tab», `toolbar.font.color` «toolbar» |
| `visuals.widgets.hovered.fg_stroke.width` | — (egui's own value stands) | `splitter.divider_width` «splitter» |
| `visuals.widgets.hovered.weak_bg_fill` | `button.background_color`, `button.hover_background` | `button.background_color` «button», `button.hover_background` «button», `combo_box.background_color` «combo_box», `combo_box.hover_background` «combo_box», `expander.hover_background` «expander», `menu.hover_background` «menu», `segmented_control.background_color` «segmented_control», `segmented_control.hover_background` «segmented_control», `sidebar.hover_background` «sidebar», `switch.hover_unchecked_background` «switch», `switch.unchecked_background` «switch», `tab.background_color` «tab», `tab.hover_background` «tab» |
| `visuals.widgets.inactive.bg_fill` | `scrollbar.thumb_color` | `checkbox.background_color` «checkbox», `checkbox.background_color` «checkbox:disabled», `checkbox.checked_background` «checkbox:selected», `checkbox.disabled_background` «checkbox:disabled», `checkbox.unchecked_background` «checkbox», `checkbox.unchecked_background` «checkbox:disabled», `scrollbar.thumb_color` «scrollbar», `slider.disabled_track_color` «slider:disabled», `slider.track_color` «slider», `slider.track_color` «slider:disabled» |
| `visuals.widgets.inactive.bg_stroke.color` | `button.border.color`, `defaults.border.opacity` | `button.border.color` «button», `checkbox.border.color` «checkbox:selected», `checkbox.border.color` «checkbox», `checkbox.unchecked_border_color` «checkbox», `combo_box.border.color` «combo_box», `defaults.border.opacity` «checkbox», `defaults.border.opacity` «menu», `defaults.border.opacity` «button», `defaults.border.opacity` «checkbox:selected», `defaults.border.opacity` «segmented_control», `defaults.border.opacity` «input», `defaults.border.opacity` «combo_box», `defaults.border.opacity` «tab», `defaults.border.opacity` «expander», `expander.border.color` «expander», `input.border.color` «input», `menu.border.color` «menu», `segmented_control.border.color` «segmented_control», `tab.border.color` «tab» |
| `visuals.widgets.inactive.bg_stroke.width` | `button.border.line_width` | `button.border.line_width` «button», `checkbox.border.line_width` «checkbox», `combo_box.border.line_width` «combo_box», `expander.border.line_width` «expander», `input.border.line_width` «input», `menu.border.line_width` «menu», `segmented_control.border.line_width` «segmented_control», `tab.border.line_width` «tab» |
| `visuals.widgets.inactive.corner_radius` | `button.border.corner_radius` | `button.border.corner_radius` «button», `checkbox.border.corner_radius` «checkbox», `combo_box.border.corner_radius` «combo_box», `defaults.border.corner_radius` «scrollbar», `defaults.border.corner_radius` «slider», `expander.border.corner_radius` «expander», `input.border.corner_radius` «input», `menu.border.corner_radius` «menu», `segmented_control.border.corner_radius` «segmented_control», `slider.track_height` «slider», `switch.track_radius` «switch», `tab.border.corner_radius` «tab» |
| `visuals.widgets.inactive.expansion` | — (egui's own value stands) | `defaults.font.size` «slider», `slider.thumb_diameter` «slider» |
| `visuals.widgets.inactive.fg_stroke.color` | `button.font.color` | `button.disabled_text_color` «button:disabled», `button.font.color` «button», `checkbox.disabled_text_color` «checkbox:disabled», `checkbox.indicator_color` «checkbox», `combo_box.disabled_text_color` «combo_box:disabled», `combo_box.font.color` «combo_box», `expander.font.color` «expander», `input.disabled_text_color` «input:disabled», `input.font.color` «input», `menu.disabled_text_color` «menu:disabled», `menu.font.color` «menu», `segmented_control.font.color` «segmented_control», `sidebar.font.color` «sidebar», `status_bar.font.color` «status_bar», `tab.font.color` «tab», `toolbar.font.color` «toolbar» |
| `visuals.widgets.inactive.weak_bg_fill` | `button.background_color` | `button.background_color` «button», `button.background_color` «button:disabled», `button.disabled_background` «button:disabled», `combo_box.background_color` «combo_box», `combo_box.background_color` «combo_box:disabled», `combo_box.disabled_background` «combo_box:disabled», `segmented_control.background_color` «segmented_control», `switch.disabled_unchecked_background` «switch:disabled», `switch.unchecked_background` «switch», `switch.unchecked_background` «switch:disabled», `tab.background_color` «tab» |
| `visuals.widgets.noninteractive.bg_fill` | `defaults.background_color` | `checkbox.checked_background` «checkbox:selected» |
| `visuals.widgets.noninteractive.bg_stroke.color` | `defaults.border.color`, `defaults.border.opacity` | `button.border.color` «button», `checkbox.border.color` «checkbox:selected», `checkbox.border.color` «checkbox», `checkbox.unchecked_border_color` «checkbox», `combo_box.border.color` «combo_box», `defaults.border.opacity` «checkbox», `defaults.border.opacity` «button», `defaults.border.opacity` «checkbox:selected», `defaults.border.opacity` «segmented_control», `defaults.border.opacity` «input», `defaults.border.opacity` «combo_box», `defaults.border.opacity` «tab», `defaults.border.opacity` «sidebar», `defaults.border.opacity` «toolbar», `defaults.border.opacity` «status_bar», `defaults.border.opacity` «expander», `expander.border.color` «expander», `input.border.color` «input», `list.grid_color` «list», `menu.separator_color` «menu», `segmented_control.border.color` «segmented_control», `separator.line_color` «separator», `sidebar.border.color` «sidebar», `splitter.divider_color` «splitter», `status_bar.border.color` «status_bar», `tab.border.color` «tab», `toolbar.border.color` «toolbar» |
| `visuals.widgets.noninteractive.bg_stroke.width` | `defaults.border.line_width` | `button.border.line_width` «button», `checkbox.border.line_width` «checkbox», `combo_box.border.line_width` «combo_box», `expander.border.line_width` «expander», `input.border.line_width` «input», `list.border.line_width` «list», `segmented_control.border.line_width` «segmented_control», `separator.line_width` «separator», `sidebar.border.line_width` «sidebar», `splitter.divider_width` «splitter», `status_bar.border.line_width` «status_bar», `tab.border.line_width` «tab», `toolbar.border.line_width` «toolbar» |
| `visuals.widgets.noninteractive.corner_radius` | `defaults.border.corner_radius` | `button.border.corner_radius` «button», `checkbox.border.corner_radius` «checkbox», `combo_box.border.corner_radius` «combo_box», `expander.border.corner_radius` «expander», `input.border.corner_radius` «input», `list.border.corner_radius` «list», `segmented_control.border.corner_radius` «segmented_control», `switch.track_radius` «switch», `tab.border.corner_radius` «tab» |
| `visuals.widgets.noninteractive.expansion` | — (egui's own value stands) | `defaults.font.size` «slider», `slider.thumb_diameter` «slider» |
| `visuals.widgets.noninteractive.fg_stroke.color` | `defaults.text_color` | `button.disabled_text_color` «button:disabled», `button.font.color` «button», `checkbox.indicator_color` «checkbox», `combo_box.disabled_text_color` «combo_box:disabled», `combo_box.font.color` «combo_box», `dialog.body_font.color` «dialog», `expander.font.color` «expander», `input.disabled_text_color` «input:disabled», `input.font.color` «input», `list.disabled_text_color` «list:disabled», `list.item_font.color` «list», `menu.disabled_text_color` «menu:disabled», `menu.font.color` «menu», `popover.font.color` «popover», `segmented_control.font.color` «segmented_control», `sidebar.font.color` «sidebar», `status_bar.font.color` «status_bar», `tab.font.color` «tab», `toolbar.font.color` «toolbar», `tooltip.font.color` «tooltip» |
| `visuals.widgets.noninteractive.weak_bg_fill` | `defaults.background_color` |  |
| `visuals.widgets.open.bg_fill` | `scrollbar.thumb_color` | `checkbox.background_color` «checkbox», `checkbox.checked_background` «checkbox:selected», `checkbox.unchecked_background` «checkbox», `scrollbar.thumb_color` «scrollbar», `slider.track_color` «slider» |
| `visuals.widgets.open.bg_stroke.color` | `button.border.color`, `defaults.border.opacity` | `button.border.color` «button», `checkbox.border.color` «checkbox:selected», `checkbox.border.color` «checkbox», `checkbox.unchecked_border_color` «checkbox», `combo_box.border.color` «combo_box», `defaults.border.opacity` «checkbox», `defaults.border.opacity` «menu», `defaults.border.opacity` «button», `defaults.border.opacity` «checkbox:selected», `defaults.border.opacity` «segmented_control», `defaults.border.opacity` «input», `defaults.border.opacity` «combo_box», `defaults.border.opacity` «tab», `defaults.border.opacity` «expander», `expander.border.color` «expander», `input.border.color` «input», `menu.border.color` «menu», `segmented_control.border.color` «segmented_control», `tab.border.color` «tab» |
| `visuals.widgets.open.bg_stroke.width` | `button.border.line_width` | `button.border.line_width` «button», `checkbox.border.line_width` «checkbox», `combo_box.border.line_width` «combo_box», `expander.border.line_width` «expander», `input.border.line_width` «input», `menu.border.line_width` «menu», `segmented_control.border.line_width` «segmented_control», `tab.border.line_width` «tab» |
| `visuals.widgets.open.corner_radius` | `button.border.corner_radius` | `button.border.corner_radius` «button», `checkbox.border.corner_radius` «checkbox», `combo_box.border.corner_radius` «combo_box», `expander.border.corner_radius` «expander», `input.border.corner_radius` «input», `menu.border.corner_radius` «menu», `segmented_control.border.corner_radius` «segmented_control», `switch.track_radius` «switch», `tab.border.corner_radius` «tab» |
| `visuals.widgets.open.expansion` | — (egui's own value stands) | `defaults.font.size` «slider», `slider.thumb_diameter` «slider» |
| `visuals.widgets.open.fg_stroke.color` | `button.font.color` | `button.font.color` «button», `checkbox.indicator_color` «checkbox», `combo_box.font.color` «combo_box», `expander.font.color` «expander», `input.font.color` «input», `menu.hover_text_color` «menu», `segmented_control.font.color` «segmented_control», `sidebar.font.color` «sidebar», `status_bar.font.color` «status_bar», `tab.font.color` «tab», `toolbar.font.color` «toolbar» |
| `visuals.widgets.open.weak_bg_fill` | `window.title_bar_background` | `button.background_color` «button», `combo_box.background_color` «combo_box», `menu.hover_background` «menu», `segmented_control.background_color` «segmented_control», `switch.unchecked_background` «switch», `tab.background_color` «tab» |
| `visuals.window_corner_radius` | `window.border.corner_radius` |  |
| `visuals.window_fill` | `menu.background_color` |  |
| `visuals.window_shadow.blur` | `window.border.shadow_enabled` |  |
| `visuals.window_shadow.color` | `defaults.shadow_color`, `window.border.shadow_enabled` |  |
| `visuals.window_shadow.offset` | `window.border.shadow_enabled` |  |
| `visuals.window_shadow.spread` | `window.border.shadow_enabled` |  |
| `visuals.window_stroke.color` | `defaults.border.color`, `defaults.border.opacity` |  |
| `visuals.window_stroke.width` | `defaults.border.line_width` |  |

## Left to egui (`[unwritten]`)

| egui field | why egui's own value stands |
|---|---|
| `always_scroll_the_only_direction` | input behaviour (§5.10) |
| `animation_time` | the accessibility preference, not a theme leaf (§4.3) |
| `card.outer_margin.bottom` | as window.outer_margin.left |
| `card.outer_margin.left` | as window.outer_margin.left |
| `card.outer_margin.right` | as window.outer_margin.left |
| `card.outer_margin.top` | as window.outer_margin.left |
| `card.shadow.blur` | as card.shadow.offset |
| `card.shadow.color` | as card.shadow.offset |
| `card.shadow.offset` | Frame::group has no shadow (frame.rs:178-183), and card.border.shadow_enabled is UNMAPPABLE (§5.2) |
| `card.shadow.spread` | as card.shadow.offset |
| `central_panel.corner_radius` | as central_panel.stroke.width |
| `central_panel.fill` | Frame::central_panel's, the base style's visuals.panel_fill, which defaults.background_color writes (§3.4, §5.9; frame.rs:191-193) |
| `central_panel.outer_margin.bottom` | as window.outer_margin.left |
| `central_panel.outer_margin.left` | as window.outer_margin.left |
| `central_panel.outer_margin.right` | as window.outer_margin.left |
| `central_panel.outer_margin.top` | as window.outer_margin.left |
| `central_panel.shadow.blur` | as central_panel.stroke.width |
| `central_panel.shadow.color` | as central_panel.stroke.width |
| `central_panel.shadow.offset` | as central_panel.stroke.width |
| `central_panel.shadow.spread` | as central_panel.stroke.width |
| `central_panel.stroke.color` | as central_panel.stroke.width |
| `central_panel.stroke.width` | Frame::central_panel has no stroke, radius or shadow (frame.rs:191-193); Surface::CentralPanel writes only its inner margin, from layout.window_margin (§4.4) |
| `compact_menu_style` | no reader anywhere in egui 0.36.2 (§5.10) |
| `debug` | egui's developer debug painting, DebugOptions (egui/src/style.rs:324, :1333): not appearance data |
| `dialog.outer_margin.bottom` | as window.outer_margin.left |
| `dialog.outer_margin.left` | as window.outer_margin.left |
| `dialog.outer_margin.right` | as window.outer_margin.left |
| `dialog.outer_margin.top` | as window.outer_margin.left |
| `drag_value_text_style` | left at egui's TextStyle::Button (egui/src/style.rs:1434), whose size button.font writes (§5.9) |
| `explanation_tooltips` | whether widgets explain themselves in tooltips is application behaviour (§5.10) |
| `interaction.interact_radius` | input-behaviour policy; ResolvedTheme exposes none of Style::interaction (§5.10) |
| `interaction.multi_widget_text_select` | as interaction.interact_radius |
| `interaction.resize_grab_radius_corner` | as interaction.interact_radius |
| `interaction.resize_grab_radius_side` | as interaction.interact_radius |
| `interaction.selectable_labels` | as interaction.interact_radius |
| `interaction.show_tooltips_only_when_still` | as interaction.interact_radius |
| `interaction.tooltip_delay` | as interaction.interact_radius |
| `interaction.tooltip_grace_time` | as interaction.interact_radius |
| `override_font_id.family` | always FontFamily::Proportional where a scope writes the size: the family reaches egui as bytes, never as a name (§8.1) |
| `override_text_style` | left at None: native-theme has no leaf forcing one TextStyle on all text; four scopes write override_font_id instead (§5.10) |
| `override_text_valign` | vertical placement of mixed text in a row is layout policy with no native leaf (§5.10) |
| `panel_bottom.outer_margin.bottom` | as window.outer_margin.left |
| `panel_bottom.outer_margin.left` | as window.outer_margin.left |
| `panel_bottom.outer_margin.right` | as window.outer_margin.left |
| `panel_bottom.outer_margin.top` | as window.outer_margin.left |
| `panel_bottom.shadow.blur` | as panel_left.shadow.offset |
| `panel_bottom.shadow.color` | as panel_left.shadow.offset |
| `panel_bottom.shadow.offset` | as panel_left.shadow.offset |
| `panel_bottom.shadow.spread` | as panel_left.shadow.offset |
| `panel_bottom.stroke.color` | as panel_left.stroke.width |
| `panel_bottom.stroke.width` | as panel_left.stroke.width |
| `panel_left.outer_margin.bottom` | as window.outer_margin.left |
| `panel_left.outer_margin.left` | as window.outer_margin.left |
| `panel_left.outer_margin.right` | as window.outer_margin.left |
| `panel_left.outer_margin.top` | as window.outer_margin.left |
| `panel_left.shadow.blur` | as panel_left.shadow.offset |
| `panel_left.shadow.color` | as panel_left.shadow.offset |
| `panel_left.shadow.offset` | Frame::side_top_panel has no shadow (frame.rs:185-189); sidebar, toolbar and status_bar border.shadow_enabled are UNMAPPABLE (§14 item 10) |
| `panel_left.shadow.spread` | as panel_left.shadow.offset |
| `panel_left.stroke.color` | as panel_left.stroke.width |
| `panel_left.stroke.width` | Frame::side_top_panel has no stroke (frame.rs:185-189); the panel's separator line is painted from the parent Ui's noninteractive.bg_stroke (§4.4) |
| `panel_right.outer_margin.bottom` | as window.outer_margin.left |
| `panel_right.outer_margin.left` | as window.outer_margin.left |
| `panel_right.outer_margin.right` | as window.outer_margin.left |
| `panel_right.outer_margin.top` | as window.outer_margin.left |
| `panel_right.shadow.blur` | as panel_left.shadow.offset |
| `panel_right.shadow.color` | as panel_left.shadow.offset |
| `panel_right.shadow.offset` | as panel_left.shadow.offset |
| `panel_right.shadow.spread` | as panel_left.shadow.offset |
| `panel_right.stroke.color` | as panel_left.stroke.width |
| `panel_right.stroke.width` | as panel_left.stroke.width |
| `panel_top.outer_margin.bottom` | as window.outer_margin.left |
| `panel_top.outer_margin.left` | as window.outer_margin.left |
| `panel_top.outer_margin.right` | as window.outer_margin.left |
| `panel_top.outer_margin.top` | as window.outer_margin.left |
| `panel_top.shadow.blur` | as panel_left.shadow.offset |
| `panel_top.shadow.color` | as panel_left.shadow.offset |
| `panel_top.shadow.offset` | as panel_left.shadow.offset |
| `panel_top.shadow.spread` | as panel_left.shadow.offset |
| `panel_top.stroke.color` | as panel_left.stroke.width |
| `panel_top.stroke.width` | as panel_left.stroke.width |
| `popover.outer_margin.bottom` | as window.outer_margin.left |
| `popover.outer_margin.left` | as window.outer_margin.left |
| `popover.outer_margin.right` | as window.outer_margin.left |
| `popover.outer_margin.top` | as window.outer_margin.left |
| `scroll_animation.duration.max` | as scroll_animation.points_per_second |
| `scroll_animation.duration.min` | as scroll_animation.points_per_second |
| `scroll_animation.points_per_second` | the accessibility preference, not a theme leaf (§4.3) |
| `spacing.combo_height` | no native leaf states a combo-box maximum height (§5.9) |
| `spacing.default_area_size.x` | sizes every free Area; dialog.max_width goes per call (§5.8 item 6) |
| `spacing.default_area_size.y` | as spacing.default_area_size.x |
| `spacing.indent` | no native leaf states an indent width (egui/src/style.rs:404, :1459) |
| `spacing.indent_ends_with_horizontal_line` | whether an indented region is ruled is layout policy with no native leaf (§5.10) |
| `spacing.interact_size.x` | no native analogue: a Grid column floor, the DragValue and colour-swatch width (§7.5) |
| `spacing.menu_margin.bottom` | as spacing.menu_margin.left |
| `spacing.menu_margin.left` | native-theme states no menu-container padding; menu.border is the item's (§5.9) |
| `spacing.menu_margin.right` | as spacing.menu_margin.left |
| `spacing.menu_margin.top` | as spacing.menu_margin.left |
| `spacing.menu_spacing` | no reader anywhere in egui 0.36.2 (§5.10) |
| `spacing.menu_width` | no reader anywhere in egui 0.36.2 (§5.10) |
| `spacing.scroll.active_background_opacity` | as spacing.scroll.dormant_background_opacity |
| `spacing.scroll.active_handle_opacity` | as spacing.scroll.dormant_background_opacity |
| `spacing.scroll.content_margin.bottom` | as spacing.scroll.content_margin.left |
| `spacing.scroll.content_margin.left` | no native leaf states a scroll-content margin (egui/src/style.rs:509, :596) |
| `spacing.scroll.content_margin.right` | as spacing.scroll.content_margin.left |
| `spacing.scroll.content_margin.top` | as spacing.scroll.content_margin.left |
| `spacing.scroll.dormant_background_opacity` | no native leaf describes an auto-hide fade curve (§5.9) |
| `spacing.scroll.dormant_handle_opacity` | as spacing.scroll.dormant_background_opacity |
| `spacing.scroll.fade.size` | as spacing.scroll.fade.strength |
| `spacing.scroll.fade.strength` | no platform fact records a scroll-edge fade (§5.10) |
| `spacing.scroll.interact_background_opacity` | as spacing.scroll.dormant_background_opacity |
| `spacing.scroll.interact_handle_opacity` | as spacing.scroll.dormant_background_opacity |
| `spacing.slider_width` | no native leaf states a slider length (§5.9) |
| `spacing.text_edit_width` | no native leaf states a text-edit default width (§5.9) |
| `text_styles[Body].family` | always FontFamily::Proportional: the family reaches egui as bytes, never as a name (§8.1; T6(d)) |
| `text_styles[Button].family` | as text_styles\[Body\].family |
| `text_styles[Heading].family` | as text_styles\[Body\].family |
| `text_styles[Monospace].family` | always FontFamily::Monospace (§8.1; T6(d)) |
| `text_styles[Small].family` | as text_styles\[Body\].family |
| `tooltip.outer_margin.bottom` | as window.outer_margin.left |
| `tooltip.outer_margin.left` | as window.outer_margin.left |
| `tooltip.outer_margin.right` | as window.outer_margin.left |
| `tooltip.outer_margin.top` | as window.outer_margin.left |
| `url_in_tooltip` | application behaviour (§5.10) |
| `visuals.button_frame` | left at egui's true (egui/src/style.rs:1546): native-theme has no leaf for whether buttons draw a frame |
| `visuals.clip_rect_margin` | deprecated; setting it has no effect (§5.10) |
| `visuals.code_bg_color` | ResolvedTheme has no code-block colour (§5.10) |
| `visuals.dark_mode` | egui's own for the scheme: every style starts from egui::Theme::default_style() (§3.4; egui/src/style.rs:1500, :1567) |
| `visuals.image_loading_spinners` | application behaviour (§5.10) |
| `visuals.ime_composition.active_underline_stroke.color` | as visuals.ime_composition.active_underline_stroke.width |
| `visuals.ime_composition.active_underline_stroke.width` | native-theme carries no IME-composition colour or width (§5.10) |
| `visuals.ime_composition.inactive_underline_stroke.color` | as visuals.ime_composition.active_underline_stroke.width |
| `visuals.ime_composition.inactive_underline_stroke.width` | as visuals.ime_composition.active_underline_stroke.width |
| `visuals.ime_composition.legacy_visuals` | a winit workaround, platform behaviour rather than theme data (§5.10) |
| `visuals.indent_has_left_vline` | whether an indented region is ruled is layout policy with no native leaf (§5.10) |
| `visuals.interact_cursor` | native-theme has no pointer-cursor leaf (§5.10) |
| `visuals.numeric_color_space` | how a colour picker prints numbers is not theme data (§5.10) |
| `visuals.resize_corner_size` | native-theme states no resize-grip size (§5.10) |
| `visuals.selection.stroke.width` | no native source; egui's 1.0 (§6.2) |
| `visuals.striped` | native-theme has no are-lists-striped boolean (§5.10) |
| `visuals.text_cursor.blink` | caret blink is input behaviour with no native leaf (§5.9) |
| `visuals.text_cursor.off_duration` | as visuals.text_cursor.blink |
| `visuals.text_cursor.on_duration` | as visuals.text_cursor.blink |
| `visuals.text_cursor.preview` | caret preview is input behaviour with no native leaf (§5.9) |
| `visuals.text_cursor.stroke.width` | no native caret width exists (§5.9) |
| `visuals.text_options.color_transfer_function` | how epaint writes glyph colours into the font atlas; native-theme carries no leaf for it (epaint/src/text/mod.rs:31-32) |
| `visuals.text_options.font_hinting` | an operating-system text-rendering setting with no native leaf (§5.10) |
| `visuals.text_options.max_texture_side` | a font-atlas texture limit, not appearance data; native-theme carries no leaf for it (epaint/src/text/mod.rs:28-29) |
| `visuals.text_options.subpixel_binning` | fractional glyph positions, with no native leaf (§5.10) |
| `visuals.weak_text_alpha` | read only where visuals.weak_text_color is None (egui/src/style.rs:1141-1143), which defaults.muted_color writes (§5.9) |
| `visuals.widgets.inactive.fg_stroke.width` | no native text-stroke width; egui's own stands, and §6.1 copies it into hovered, active and open |
| `visuals.widgets.noninteractive.fg_stroke.width` | no native text-stroke width; egui's own stands (§6.1) |
| `visuals.widgets.open.fg_stroke.width` | written only as §6.1's copy of inactive's width, which is egui's own: no native text-stroke width |
| `visuals.window_highlight_topmost` | no consumer (§5.10) |
| `window.corner_radius` | Frame::window's, from the base style's visuals.window_corner_radius, which window.border.corner_radius writes (§3.4; frame.rs:199) |
| `window.inner_margin.bottom` | as window.inner_margin.left |
| `window.inner_margin.left` | Frame::window's, from the base style's spacing.window_margin, which window.border.padding writes (§3.4; egui/src/containers/frame.rs:198) |
| `window.inner_margin.right` | as window.inner_margin.left |
| `window.inner_margin.top` | as window.inner_margin.left |
| `window.outer_margin.bottom` | as window.outer_margin.left |
| `window.outer_margin.left` | egui's preset frames have no outer margin (egui/src/containers/frame.rs:170-175) and native-theme states none |
| `window.outer_margin.right` | as window.outer_margin.left |
| `window.outer_margin.top` | as window.outer_margin.left |
| `window.shadow.blur` | as window.shadow.offset |
| `window.shadow.color` | as window.shadow.offset |
| `window.shadow.offset` | Frame::window's, the base style's visuals.window_shadow, which window.border.shadow_enabled gates (§3.4, §6.14; frame.rs:200) |
| `window.shadow.spread` | as window.shadow.offset |
| `window_title_bar.corner_radius` | as window_title_bar.inner_margin.left |
| `window_title_bar.inner_margin.bottom` | as window_title_bar.inner_margin.left |
| `window_title_bar.inner_margin.left` | Frame::window's, as the window frame's; the title bar states only its fill (§5.2) |
| `window_title_bar.inner_margin.right` | as window_title_bar.inner_margin.left |
| `window_title_bar.inner_margin.top` | as window_title_bar.inner_margin.left |
| `window_title_bar.outer_margin.bottom` | as window.outer_margin.left |
| `window_title_bar.outer_margin.left` | as window.outer_margin.left |
| `window_title_bar.outer_margin.right` | as window.outer_margin.left |
| `window_title_bar.outer_margin.top` | as window.outer_margin.left |
| `window_title_bar.shadow.blur` | as window_title_bar.inner_margin.left |
| `window_title_bar.shadow.color` | as window_title_bar.inner_margin.left |
| `window_title_bar.shadow.offset` | as window_title_bar.inner_margin.left |
| `window_title_bar.shadow.spread` | as window_title_bar.inner_margin.left |
| `window_title_bar.stroke.color` | as window_title_bar.inner_margin.left |
| `window_title_bar.stroke.width` | as window_title_bar.inner_margin.left |
| `wrap_mode` | text-wrapping behaviour, not appearance data (§5.10) |
