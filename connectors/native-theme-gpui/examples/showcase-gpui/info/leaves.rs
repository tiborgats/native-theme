//! How gpui applies each theme leaf an element of docs/showcase-elements.toml
//! draws: the muted line Widget Info shows under each `<leaf> <value>` row
//! (the round-11 brief, section B). A line names the route the value takes --
//! the connector's `ThemeColor` field (`colors.rs`), its `geometry::` builder,
//! its `widgets::` control, or the showcase helper that sets it -- or says
//! "not reachable" with the upstream line that keeps it out. Where the
//! showcase could apply a leaf and does not, the line says so and names what
//! is drawn instead. Upstream citations are to gpui-component 0.7.1,
//! gpui-base 0.7.1 and gpui-pre 0.3.8.

/// Each line as (element-id prefix, leaf, line). The first entry whose
/// prefix the hovered element's id starts with and whose leaf matches wins;
/// the prefix "" matches every element, so an element's own entries come
/// before the generic one for the same leaf.
pub const HOW: &[(&str, &str, &str)] = &[
    // --- button: the toggle button that is on --------------------------------
    (
        "basic.buttons.toggle_on",
        "button.checked_background",
        "gpui: geometry::button_checked's bg, replayed over the selected Button's style (demo::toggle_button)",
    ),
    (
        "basic.buttons.toggle_on",
        "button.checked_text_color",
        "gpui: geometry::button_checked's text colour, over the selected Button's style (demo::toggle_button)",
    ),
    (
        "basic.buttons.toggle_on",
        "button.active_background",
        "gpui: geometry::button_checked's bg where checked_background is unstated (demo::toggle_button)",
    ),
    (
        "basic.buttons.toggle_on",
        "button.active_text_color",
        "gpui: geometry::button_checked's text colour where checked_text_color is unstated (demo::toggle_button)",
    ),
    // --- button: the Copy box of Widget Info -----------------------------------
    (
        "chrome.info.copy",
        "button.hover_background",
        "gpui: the Copy box's fill under the pointer (inspector.rs)",
    ),
    (
        "chrome.info.copy",
        "button.hover_text_color",
        "gpui: the Copy box's text under the pointer (inspector.rs)",
    ),
    (
        "chrome.info.copy",
        "button.active_background",
        "gpui: the Copy box's fill while pressed (inspector.rs)",
    ),
    (
        "chrome.info.copy",
        "button.active_text_color",
        "gpui: the Copy box's text while pressed (inspector.rs)",
    ),
    (
        "chrome.info.copy",
        "button.border.corner_radius_px",
        "gpui: the Copy box's corner radius (inspector.rs)",
    ),
    (
        "chrome.info.copy",
        "button.border.padding_top_px",
        "gpui: pads the Copy box's text (inspector.rs)",
    ),
    (
        "chrome.info.copy",
        "button.border.padding_right_px",
        "gpui: pads the Copy box's text (inspector.rs)",
    ),
    (
        "chrome.info.copy",
        "button.border.padding_bottom_px",
        "gpui: pads the Copy box's text (inspector.rs)",
    ),
    (
        "chrome.info.copy",
        "button.border.padding_left_px",
        "gpui: pads the Copy box's text (inspector.rs)",
    ),
    // --- button: the pages menu, variants::ghost_button --------------------------
    (
        "chrome.page_tabs.menu",
        "button.font.color",
        "gpui: variants::ghost_button's foreground, ThemeColor secondary_foreground",
    ),
    (
        "chrome.page_tabs.menu",
        "button.hover_background",
        "gpui: variants::ghost_button's hover, ThemeColor secondary_hover (colors.rs)",
    ),
    (
        "chrome.page_tabs.menu",
        "button.hover_text_color",
        "not reachable: a Custom variant has no hover text colour (button.rs:102-141, 1150-1155)",
    ),
    (
        "chrome.page_tabs.menu",
        "button.active_background",
        "gpui: variants::ghost_button's active, ThemeColor secondary_active (colors.rs)",
    ),
    (
        "chrome.page_tabs.menu",
        "button.border.corner_radius_px",
        "gpui: rounds the tool button's box (demo::tool_button_box)",
    ),
    (
        "chrome.page_tabs.menu",
        "button.border.padding_top_px",
        "gpui: pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.page_tabs.menu",
        "button.border.padding_right_px",
        "gpui: pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.page_tabs.menu",
        "button.border.padding_bottom_px",
        "gpui: pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.page_tabs.menu",
        "button.border.padding_left_px",
        "gpui: pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.page_tabs.menu",
        "defaults.icon_sizes.small_px",
        "gpui: the set's ChevronDown Icon::with_size (demo::native_tab_bar)",
    ),
    (
        "chrome.page_tabs.menu",
        "defaults.icon_theme",
        "gpui: the ChevronDown of the chosen freedesktop theme (Showcase::sample_icon)",
    ),
    (
        "chrome.page_tabs.menu",
        "theme_variant.icon_set",
        "gpui: the ChevronDown of the chosen set, none mixed in (Showcase::sample_icon)",
    ),
    // --- button: the icon-only buttons, variants::ghost_button -----------------
    (
        "chrome.status_bar.toggle",
        "button.active_background",
        "gpui: ghost_button's active, secondary_active; also its selected fill (button.rs:1295)",
    ),
    (
        "chrome.toolbar.",
        "button.font.color",
        "gpui: variants::ghost_button's foreground, ThemeColor secondary_foreground",
    ),
    (
        "basic.icons.",
        "button.font.color",
        "gpui: variants::ghost_button's foreground, ThemeColor secondary_foreground",
    ),
    (
        "chrome.status_bar.toggle",
        "button.font.color",
        "gpui: variants::ghost_button's foreground, ThemeColor secondary_foreground",
    ),
    (
        "chrome.toolbar.",
        "button.hover_background",
        "gpui: variants::ghost_button's hover, ThemeColor secondary_hover (colors.rs)",
    ),
    (
        "basic.icons.",
        "button.hover_background",
        "gpui: variants::ghost_button's hover, ThemeColor secondary_hover (colors.rs)",
    ),
    (
        "chrome.status_bar.toggle",
        "button.hover_background",
        "gpui: variants::ghost_button's hover, ThemeColor secondary_hover (colors.rs)",
    ),
    (
        "chrome.toolbar.",
        "button.hover_text_color",
        "not reachable: a Custom variant has no hover text colour (button.rs:102-141, 1150-1155)",
    ),
    (
        "basic.icons.",
        "button.hover_text_color",
        "not reachable: a Custom variant has no hover text colour (button.rs:102-141, 1150-1155)",
    ),
    (
        "chrome.status_bar.toggle",
        "button.hover_text_color",
        "not reachable: a Custom variant has no hover text colour (button.rs:102-141, 1150-1155)",
    ),
    (
        "chrome.toolbar.",
        "button.active_background",
        "gpui: variants::ghost_button's active, ThemeColor secondary_active (colors.rs)",
    ),
    (
        "basic.icons.",
        "button.active_background",
        "gpui: variants::ghost_button's active, ThemeColor secondary_active (colors.rs)",
    ),
    (
        "chrome.toolbar.",
        "button.border.corner_radius_px",
        "gpui: rounds the tool button's box (demo::tool_button_box)",
    ),
    (
        "basic.icons.",
        "button.border.corner_radius_px",
        "gpui: rounds the tool button's box (demo::tool_button_box)",
    ),
    (
        "chrome.status_bar.toggle",
        "button.border.corner_radius_px",
        "gpui: rounds the tool button's box (demo::tool_button_box)",
    ),
    (
        "chrome.toolbar.",
        "button.border.padding_top_px",
        "gpui: pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.toolbar.",
        "button.border.padding_right_px",
        "gpui: pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.toolbar.",
        "button.border.padding_bottom_px",
        "gpui: pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.toolbar.",
        "button.border.padding_left_px",
        "gpui: pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "basic.icons.",
        "button.border.padding_top_px",
        "gpui: pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "basic.icons.",
        "button.border.padding_right_px",
        "gpui: pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "basic.icons.",
        "button.border.padding_bottom_px",
        "gpui: pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "basic.icons.",
        "button.border.padding_left_px",
        "gpui: pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.status_bar.toggle",
        "button.border.padding_top_px",
        "gpui: pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.status_bar.toggle",
        "button.border.padding_right_px",
        "gpui: pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.status_bar.toggle",
        "button.border.padding_bottom_px",
        "gpui: pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.status_bar.toggle",
        "button.border.padding_left_px",
        "gpui: pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    // --- button: a Default Button refined by geometry::button ------------------
    (
        "",
        "button.active_background",
        "gpui: ThemeColor button_active, the fill blended with it (colors.rs); the pressed fill",
    ),
    (
        "",
        "button.active_text_color",
        "not reachable: a pressed Button keeps its variant's text colour (button.rs:1257-1261)",
    ),
    (
        "",
        "button.background_color",
        "gpui: ThemeColor button (colors.rs, assign_buttons): a Default Button's fill",
    ),
    (
        "",
        "button.border.color",
        "gpui: geometry::button's border colour",
    ),
    (
        "",
        "button.border.corner_radius_px",
        "gpui: geometry::button's corner radius",
    ),
    (
        "",
        "button.border.line_width_px",
        "gpui: geometry::button's border width",
    ),
    (
        "",
        "button.border.padding_top_px",
        "gpui: geometry::button pads the stated side",
    ),
    (
        "",
        "button.border.padding_right_px",
        "gpui: geometry::button pads the stated side",
    ),
    (
        "",
        "button.border.padding_bottom_px",
        "gpui: geometry::button pads the stated side",
    ),
    (
        "",
        "button.border.padding_left_px",
        "gpui: geometry::button pads the stated side",
    ),
    (
        "",
        "button.border.shadow_enabled",
        "not reachable: only a Custom variant takes a shadow (button.rs:1093-1098)",
    ),
    (
        "",
        "button.disabled_background",
        "gpui: geometry::button_disabled fills a disabled Button with it",
    ),
    (
        "",
        "button.disabled_opacity",
        "gpui: geometry::button_disabled fades a disabled Button by it",
    ),
    (
        "",
        "button.disabled_text_color",
        "gpui: geometry::button_disabled letters a disabled Button in it",
    ),
    (
        "",
        "button.font",
        "gpui: `geometry::button_label` gives the label, the Button's child, `button.font`'s size and weight (demo::labelled); a `.label()` would be upstream's `text_sm` at Medium (sizing.rs:337-343)",
    ),
    (
        "",
        "button.font.color",
        "gpui: ThemeColor button_foreground (colors.rs): the label's colour",
    ),
    (
        "",
        "button.hover_background",
        "gpui: ThemeColor button_hover, the fill blended with it (colors.rs): the hover fill",
    ),
    (
        "",
        "button.hover_text_color",
        "not reachable: a hovered Button keeps its variant's text colour (button.rs:1181-1186)",
    ),
    (
        "",
        "button.min_height_px",
        "gpui: geometry::button's height, by the control-height rule",
    ),
    ("", "button.min_width_px", "gpui: geometry::button's min_w"),
    (
        "",
        "button.primary_background",
        "gpui: ThemeColor primary → button_primary (colors.rs): the Primary Button's fill",
    ),
    (
        "",
        "button.primary_text_color",
        "gpui: ThemeColor primary_foreground → button_primary_foreground (colors.rs)",
    ),
    // --- card: demo::card, a Fill GroupBox --------------------------------------
    (
        "",
        "card.background_color",
        "gpui: the GroupBox content's bg (demo::card)",
    ),
    (
        "",
        "card.border.color",
        "gpui: geometry::group_box_content's border colour",
    ),
    (
        "",
        "card.border.corner_radius_px",
        "gpui: geometry::group_box_content's corner radius",
    ),
    (
        "",
        "card.border.line_width_px",
        "gpui: geometry::group_box_content's border width",
    ),
    (
        "",
        "card.border.padding_top_px",
        "gpui: geometry::group_box_content pads the stated side; container_margin the rest",
    ),
    (
        "",
        "card.border.padding_right_px",
        "gpui: geometry::group_box_content pads the stated side; container_margin the rest",
    ),
    (
        "",
        "card.border.padding_bottom_px",
        "gpui: geometry::group_box_content pads the stated side; container_margin the rest",
    ),
    (
        "",
        "card.border.padding_left_px",
        "gpui: geometry::group_box_content pads the stated side; container_margin the rest",
    ),
    (
        "",
        "card.border.shadow_enabled",
        "gpui: not applied — the GroupBox draws no shadow (group_box.rs)",
    ),
    // --- checkbox: widgets::Checkbox and widgets::Radio -------------------------
    (
        "",
        "checkbox.background_color",
        "gpui: widgets::Checkbox's unchecked fill and hover where theirs are unstated (CheckboxLook)",
    ),
    (
        "",
        "checkbox.border.color",
        "gpui: the checked indicator's border, and the unchecked one's where unstated (CheckboxLook)",
    ),
    (
        "",
        "checkbox.border.corner_radius_px",
        "gpui: the box's corner radius (widgets::Checkbox; a radio is round)",
    ),
    (
        "",
        "checkbox.border.line_width_px",
        "gpui: the indicator's border width (widgets::Checkbox)",
    ),
    (
        "",
        "checkbox.border.padding_top_px",
        "gpui: insets the check mark in the box (widgets::Checkbox, CheckboxLook::mark_size)",
    ),
    (
        "",
        "checkbox.border.padding_right_px",
        "gpui: insets the check mark in the box (widgets::Checkbox, CheckboxLook::mark_size)",
    ),
    (
        "",
        "checkbox.border.padding_bottom_px",
        "gpui: insets the check mark in the box (widgets::Checkbox, CheckboxLook::mark_size)",
    ),
    (
        "",
        "checkbox.border.padding_left_px",
        "gpui: insets the check mark in the box (widgets::Checkbox, CheckboxLook::mark_size)",
    ),
    (
        "",
        "checkbox.check_mark_stroke_width_px",
        "gpui: the check mark's stroke (widgets::Checkbox, stroked_check)",
    ),
    (
        "",
        "checkbox.checked_background",
        "gpui: the checked indicator's fill (CheckboxLook)",
    ),
    (
        "",
        "checkbox.disabled_background",
        "gpui: a disabled indicator's fill where stated (CheckboxLook)",
    ),
    (
        "",
        "checkbox.disabled_opacity",
        "gpui: fades the whole disabled control (widgets::Checkbox)",
    ),
    (
        "",
        "checkbox.disabled_text_color",
        "gpui: a disabled control's label, and its mark over a stated disabled fill (CheckboxLook)",
    ),
    (
        "",
        "checkbox.font",
        "gpui: the label's size and weight (widgets::Checkbox); family: the window's",
    ),
    (
        "",
        "checkbox.font.color",
        "gpui: the label's colour (CheckboxLook)",
    ),
    (
        "",
        "checkbox.hover_background",
        "gpui: over the unchecked fill under the pointer (CheckboxLook)",
    ),
    (
        "",
        "checkbox.indicator_color",
        "gpui: the check mark's and the radio dot's colour (CheckboxLook)",
    ),
    (
        "",
        "checkbox.indicator_width_px",
        "gpui: the box's side, the circle's diameter (widgets::Checkbox, widgets::Radio)",
    ),
    (
        "",
        "checkbox.label_gap_px",
        "gpui: the gap between indicator and label (widgets::Checkbox)",
    ),
    (
        "",
        "checkbox.radio_dot_diameter_px",
        "gpui: the selected radio's dot (widgets::Radio)",
    ),
    (
        "",
        "checkbox.radio_indicator_width_px",
        "gpui: the radio's circle, indicator_width where unstated (widgets::Radio, CheckboxLook::radio)",
    ),
    (
        "",
        "checkbox.unchecked_background",
        "gpui: the unchecked indicator's fill (CheckboxLook)",
    ),
    (
        "",
        "checkbox.unchecked_border_color",
        "gpui: the unchecked indicator's border (CheckboxLook)",
    ),
    // --- combo_box: the Combobox of the side panel ------------------------------
    (
        "chrome.side_panel.settings.theme",
        "combo_box.font",
        "gpui: geometry::combobox's size and weight; family: the window's",
    ),
    (
        "chrome.side_panel.settings.theme",
        "combo_box.font.color",
        "not reachable: the Combobox's disabled colour lands before the caller's style (combobox.rs:998)",
    ),
    // --- combo_box: a Select, and the Combobox's metrics ------------------------
    (
        "",
        "combo_box.arrow_area_width_px",
        "gpui: where stated, the trigger keeps upstream's right padding: it has no arrow column",
    ),
    (
        "",
        "combo_box.arrow_icon_size_px",
        "not reachable: the caret is the trigger's own, sized inside it (select.rs:599-600)",
    ),
    (
        "",
        "combo_box.background_color",
        "gpui: the fill under the trigger (demo::combo_surface)",
    ),
    (
        "",
        "combo_box.border.color",
        "gpui: the trigger's edge (demo::combo_fill)",
    ),
    (
        "",
        "combo_box.border.corner_radius_px",
        "gpui: geometry::select's radius, and the surface's (demo::combo_surface)",
    ),
    (
        "",
        "combo_box.border.line_width_px",
        "gpui: not applied — the trigger keeps its own 1px edge (select.rs, input_style)",
    ),
    (
        "",
        "combo_box.border.padding_top_px",
        "gpui: geometry::select pads the stated side",
    ),
    (
        "",
        "combo_box.border.padding_right_px",
        "gpui: geometry::select pads it, unless an arrow column is stated",
    ),
    (
        "",
        "combo_box.border.padding_bottom_px",
        "gpui: geometry::select pads the stated side",
    ),
    (
        "",
        "combo_box.border.padding_left_px",
        "gpui: geometry::select pads the stated side",
    ),
    (
        "",
        "combo_box.border.shadow_enabled",
        "gpui: not applied — the trigger draws no shadow (select.rs, combobox.rs)",
    ),
    (
        "",
        "combo_box.font",
        "gpui: geometry::select's size and weight; family: the window's",
    ),
    (
        "",
        "combo_box.font.color",
        "gpui: geometry::select's text colour",
    ),
    (
        "",
        "combo_box.hover_background",
        "gpui: over the fill under the pointer (demo::combo_surface)",
    ),
    (
        "",
        "combo_box.min_height_px",
        "gpui: geometry::select's height, by the control-height rule",
    ),
    (
        "",
        "combo_box.min_width_px",
        "gpui: geometry::select's min_w",
    ),
    // --- defaults ----------------------------------------------------------------
    (
        "",
        "defaults.border.color",
        "gpui: frames the swatch (inspector.rs)",
    ),
    (
        "",
        "defaults.border.corner_radius_px",
        "gpui: the swatch's corner radius (inspector.rs)",
    ),
    (
        "",
        "defaults.border.line_width_px",
        "gpui: the swatch's frame width (inspector.rs)",
    ),
    (
        "",
        "defaults.disabled_text_color",
        "gpui: a disabled switch's label (SwitchLook)",
    ),
    (
        "",
        "defaults.focus_ring_color",
        "gpui: ThemeColor ring (colors.rs): a focused Input's edge and ring (styled.rs:179-184, focus_ring_style, which hands it to focus_style, :250-298)",
    ),
    (
        "",
        "defaults.focus_ring_offset_px",
        "not reachable: upstream's ring lies against the border, no offset (styled.rs, focus_ring)",
    ),
    (
        "",
        "defaults.focus_ring_width_px",
        "not reachable: the ring is FOCUS_RING_WIDTH, 3px (styled.rs:11); a width > 0 turns it on",
    ),
    (
        "basic.typography.body",
        "defaults.font",
        "gpui: size, weight and family on the Label (demo::type_line)",
    ),
    (
        "basic.switches",
        "defaults.font",
        "gpui: widgets::Switch's label at its size and weight (switch.rs)",
    ),
    (
        "",
        "defaults.font",
        "gpui: the window's font, Theme::font_family and font_size (config.rs; root.rs:436, 444)",
    ),
    (
        "basic.typography",
        "defaults.font.color",
        "gpui: the Label's text colour (demo::type_line)",
    ),
    (
        "basic.switches",
        "defaults.font.color",
        "gpui: widgets::Switch's label, defaults.text_color (SwitchLook)",
    ),
    (
        "",
        "defaults.font.color",
        "gpui: the Label paints `foreground`, defaults.text_color (label.rs:211)",
    ),
    (
        "basic.typography",
        "defaults.font.family",
        "gpui: font_family on the Label (demo::type_line)",
    ),
    (
        "",
        "defaults.font.family",
        "gpui: the window's font, Theme::font_family (config.rs), set by gpui-component's WindowState root plugin (root.rs:444)",
    ),
    (
        "basic.icons.large",
        "defaults.icon_sizes.large_px",
        "gpui: geometry::icon_size_large sizes the icon (demo::sized_icon)",
    ),
    (
        "",
        "defaults.icon_sizes.large_px",
        "gpui: geometry::icon_size_large sizes the icon",
    ),
    (
        "basic.icons",
        "defaults.icon_sizes.small_px",
        "gpui: geometry::icon_size_small sizes the icon (demo::sized_icon)",
    ),
    (
        "",
        "defaults.icon_sizes.small_px",
        "gpui: geometry::icon_size_small sizes the icon (demo::native_sized)",
    ),
    (
        "",
        "defaults.icon_sizes.toolbar_px",
        "gpui: geometry::icon_size_toolbar, toolbar.icon_size, which inherits it",
    ),
    (
        "basic.spinner",
        "defaults.icon_theme",
        "gpui: widgets::Spinner draws that theme's own loading indicator (spinner.rs)",
    ),
    (
        "",
        "defaults.icon_theme",
        "gpui: the freedesktop theme the icon-theme Select names, by default the theme's (app.rs)",
    ),
    (
        "basic.buttons",
        "defaults.line_height",
        "gpui: geometry::button's line height (control-height rule)",
    ),
    (
        "basic.checkboxes",
        "defaults.line_height",
        "gpui: the label's line height (widgets::Checkbox)",
    ),
    (
        "basic.radios",
        "defaults.line_height",
        "gpui: the label's line height (widgets::Radio)",
    ),
    (
        "basic.switches",
        "defaults.line_height",
        "gpui: not applied — the label's line box is the track's height (switch.rs:292)",
    ),
    (
        "basic.text_inputs",
        "defaults.line_height",
        "gpui: geometry::input's line height (control-height rule)",
    ),
    (
        "basic.number_input",
        "defaults.line_height",
        "gpui: geometry::input's line height (control-height rule)",
    ),
    (
        "basic.text_area",
        "defaults.line_height",
        "gpui: geometry::text_area's line height",
    ),
    (
        "basic.drop_down",
        "defaults.line_height",
        "gpui: geometry::select's line height (control-height rule)",
    ),
    (
        "chrome.side_panel.settings.theme_label",
        "defaults.line_height",
        "gpui: not applied — the label keeps gpui's line height, phi (style.rs:494)",
    ),
    (
        "chrome.side_panel.settings.mode_label",
        "defaults.line_height",
        "gpui: not applied — the label keeps gpui's line height, phi (style.rs:494)",
    ),
    (
        "chrome.side_panel.settings.icon_theme_label",
        "defaults.line_height",
        "gpui: not applied — the label keeps gpui's line height, phi (style.rs:494)",
    ),
    (
        "chrome.side_panel.settings.theme",
        "defaults.line_height",
        "gpui: geometry::combobox's line height (control-height rule)",
    ),
    (
        "chrome.side_panel.settings",
        "defaults.line_height",
        "gpui: geometry::select's line height (control-height rule)",
    ),
    (
        "chrome.side_panel.inspector_tabs",
        "defaults.line_height",
        "gpui: not applied — a tab's label keeps gpui's line height, phi (style.rs:494)",
    ),
    (
        "chrome.page_tabs",
        "defaults.line_height",
        "gpui: not applied — a tab's label keeps gpui's line height, phi (style.rs:494)",
    ),
    (
        "basic.tabs",
        "defaults.line_height",
        "gpui: not applied — a tab's label keeps gpui's line height, phi (style.rs:494)",
    ),
    (
        "basic.segmented",
        "defaults.line_height",
        "gpui: not applied — a segment's label keeps gpui's line height, phi (style.rs:494)",
    ),
    (
        "basic.list",
        "defaults.line_height",
        "gpui: geometry::list_item's line height (control-height rule)",
    ),
    (
        "basic.table",
        "defaults.line_height",
        "gpui: each row's line box, the font's size by it (demo::files_table)",
    ),
    (
        "basic.typography.link",
        "defaults.line_height",
        "gpui: the Link's line height, link.font's size by it (demo::link)",
    ),
    (
        "basic.typography",
        "defaults.line_height",
        "gpui: the line box, the font's size by it (demo::type_line)",
    ),
    (
        "basic.expander",
        "defaults.line_height",
        "gpui: the body Label's (demo::body_label); the header keeps gpui's, phi (style.rs:494)",
    ),
    (
        "basic.card",
        "defaults.line_height",
        "gpui: the Label's line height (demo::body_label)",
    ),
    (
        "chrome.menu.theme.preferences.shortcut",
        "defaults.line_height",
        "gpui: the row's line height, which the plain Kbd text inherits (MenuLook; kbd.rs:234-236)",
    ),
    (
        "chrome.menu.",
        "defaults.line_height",
        "gpui: the row's line height (MenuLook)",
    ),
    (
        "chrome.menu_bar.",
        "defaults.line_height",
        "gpui: the title's line height (MenuLook)",
    ),
    (
        "chrome.status_bar",
        "defaults.line_height",
        "gpui: not applied — the bar's text keeps gpui's line height, phi (style.rs:494)",
    ),
    (
        "chrome.info",
        "defaults.line_height",
        "gpui: the line box of every Widget Info line (inspector.rs)",
    ),
    (
        "",
        "defaults.line_height",
        "gpui: the element's line height, as the connector's control-height rule sets it",
    ),
    (
        "",
        "defaults.mono_font",
        "gpui: family, size and weight on the Label (demo::type_line)",
    ),
    (
        "",
        "defaults.mono_font.color",
        "gpui: the Label's text colour (demo::type_line)",
    ),
    (
        "",
        "defaults.muted_color",
        "gpui: the colour of the hint and the how lines (inspector.rs)",
    ),
    // --- expander: demo::native_expander ----------------------------------------
    (
        "",
        "expander.arrow_color",
        "gpui: the arrow Icon's colour, the title's where unstated (demo::native_expander)",
    ),
    (
        "",
        "expander.arrow_gap_px",
        "gpui: the header's gap between arrow and title (demo::native_expander)",
    ),
    (
        "",
        "expander.arrow_icon_size_px",
        "gpui: the arrow Icon's size (demo::native_expander)",
    ),
    (
        "",
        "expander.arrow_side",
        "gpui: the arrow before or after the title (demo::native_expander)",
    ),
    (
        "",
        "expander.border.color",
        "gpui: the frame and the line between items, where framed (demo::native_expander)",
    ),
    (
        "",
        "expander.border.corner_radius_px",
        "gpui: the frame's corner radius, where framed (demo::native_expander)",
    ),
    (
        "",
        "expander.border.line_width_px",
        "gpui: the frame's and the item line's width, where framed (demo::native_expander)",
    ),
    (
        "",
        "expander.border.padding_top_px",
        "gpui: pads the header where stated; upstream's px_3 across where not (demo::native_expander)",
    ),
    (
        "",
        "expander.border.padding_right_px",
        "gpui: pads the header where stated; upstream's px_3 across where not (demo::native_expander)",
    ),
    (
        "",
        "expander.border.padding_bottom_px",
        "gpui: pads the header where stated; upstream's px_3 across where not (demo::native_expander)",
    ),
    (
        "",
        "expander.border.padding_left_px",
        "gpui: pads the header where stated; upstream's px_3 across where not (demo::native_expander)",
    ),
    (
        "",
        "expander.content_indent_px",
        "gpui: the body's left padding (demo::native_expander)",
    ),
    (
        "",
        "expander.font",
        "gpui: the header's size and weight (demo::native_expander); family: the window's",
    ),
    (
        "",
        "expander.font.color",
        "gpui: the header's text colour (demo::native_expander)",
    ),
    (
        "",
        "expander.frame_enabled",
        "gpui: frames the items and parts them unless false (demo::native_expander)",
    ),
    (
        "",
        "expander.header_height_px",
        "gpui: geometry::accordion_title's height on each header",
    ),
    (
        "",
        "expander.hover_background",
        "gpui: the header's fill under the pointer (demo::native_expander)",
    ),
    // --- input: the Basic page's fields ----------------------------------------
    (
        "basic.number_input",
        "input.background_color",
        "gpui: the frame's fill (demo::number_input)",
    ),
    (
        "basic.text_area",
        "input.background_color",
        "gpui: geometry::input_fill on the Textarea (demo::rows_textarea)",
    ),
    (
        "",
        "input.background_color",
        "gpui: the field's fill (demo::input_surface, geometry::input_fill)",
    ),
    (
        "basic.number_input",
        "input.border.color",
        "gpui: the frame's edge (demo::number_input)",
    ),
    (
        "",
        "input.border.color",
        "gpui: the surface's edge (demo::input_surface); a disabled field's, ThemeColor input",
    ),
    (
        "",
        "input.border.corner_radius_px",
        "gpui: geometry::input's corner radius",
    ),
    (
        "",
        "input.border.line_width_px",
        "gpui: geometry::input's border width",
    ),
    (
        "basic.number_input",
        "input.border.padding_top_px",
        "gpui: pads the value's Input between the step buttons (demo::number_input)",
    ),
    (
        "basic.number_input",
        "input.border.padding_right_px",
        "gpui: pads the value's Input between the step buttons (demo::number_input)",
    ),
    (
        "basic.number_input",
        "input.border.padding_bottom_px",
        "gpui: pads the value's Input between the step buttons (demo::number_input)",
    ),
    (
        "basic.number_input",
        "input.border.padding_left_px",
        "gpui: pads the value's Input between the step buttons (demo::number_input)",
    ),
    (
        "",
        "input.border.padding_top_px",
        "gpui: geometry::input pads the stated side",
    ),
    (
        "",
        "input.border.padding_right_px",
        "gpui: geometry::input pads the stated side",
    ),
    (
        "",
        "input.border.padding_bottom_px",
        "gpui: geometry::input pads the stated side",
    ),
    (
        "",
        "input.border.padding_left_px",
        "gpui: geometry::input pads the stated side",
    ),
    (
        "",
        "input.border.shadow_enabled",
        "gpui: not applied — the Input draws no shadow (input/input.rs)",
    ),
    (
        "",
        "input.caret_color",
        "gpui: ThemeColor caret (colors.rs): the Input's caret",
    ),
    (
        "",
        "input.disabled_background",
        "gpui: geometry::input_fill's fill for a disabled field",
    ),
    (
        "",
        "input.disabled_opacity",
        "gpui: geometry::input_fill fades a disabled field by it",
    ),
    (
        "",
        "input.disabled_text_color",
        "not reachable: gpui-base halves a disabled Input's text colour (input/base/element.rs:2541)",
    ),
    (
        "",
        "input.focus_border_color",
        "not reachable: a focused Input edges itself in `ring` after the caller's style (input/input.rs:748-751)",
    ),
    (
        "basic.text_area",
        "input.font",
        "gpui: geometry::text_area's size and weight; family: the window's",
    ),
    (
        "",
        "input.font",
        "gpui: geometry::input's size and weight; family: the window's",
    ),
    (
        "",
        "input.font.color",
        "gpui: not carried — the Input letters its text in `foreground` (input/input.rs:105)",
    ),
    (
        "basic.number_input",
        "input.hover_border_color",
        "gpui: the frame's edge under the pointer (demo::number_input)",
    ),
    (
        "basic.text_area",
        "input.hover_border_color",
        "gpui: not applied — the Textarea keeps its edge under the pointer",
    ),
    (
        "",
        "input.hover_border_color",
        "gpui: the surface's edge under the pointer (demo::input_surface)",
    ),
    (
        "",
        "input.min_height_px",
        "gpui: geometry::input's height, by the control-height rule",
    ),
    (
        "",
        "input.placeholder_color",
        "not reachable: the placeholder is muted_foreground (input/input.rs:559)",
    ),
    // --- layout -------------------------------------------------------------------
    (
        "chrome.menu_bar",
        "layout.container_margin_px",
        "gpui: the menu bar's side padding (demo::menu_bar)",
    ),
    (
        "chrome.side_panel.inspector_tabs",
        "layout.container_margin_px",
        "gpui: the tab bar's side inset (demo::native_tab_bar)",
    ),
    (
        "chrome.page_tabs",
        "layout.container_margin_px",
        "gpui: the tab bar's side inset (demo::native_tab_bar)",
    ),
    (
        "chrome.side_panel.inspector",
        "layout.container_margin_px",
        "gpui: pads the inspector's content (inspector.rs)",
    ),
    (
        "chrome.status_bar",
        "layout.container_margin_px",
        "gpui: not applied — an unstated side keeps StatusBar's px_2 py_1 (status_bar.rs:89-90), the padding's",
    ),
    (
        "basic.card",
        "layout.container_margin_px",
        "gpui: pads a side card.border.padding leaves unstated (demo::card)",
    ),
    (
        "",
        "layout.container_margin_px",
        "gpui: pads the theme settings (demo::side_panel)",
    ),
    (
        "chrome.info",
        "layout.section_gap_px",
        "gpui: the space before a Widget Info section (inspector.rs)",
    ),
    (
        "basic.page",
        "layout.section_gap_px",
        "gpui: the gap between the columns (pages/basic.rs)",
    ),
    (
        "",
        "layout.section_gap_px",
        "gpui: the gap between the column's groups (pages/basic.rs)",
    ),
    (
        "chrome.info",
        "layout.widget_gap_px",
        "gpui: the space between Widget Info rows (inspector.rs)",
    ),
    (
        "chrome.side_panel.inspector",
        "layout.widget_gap_px",
        "gpui: the gap between the inspector's rows (inspector.rs)",
    ),
    (
        "chrome.side_panel",
        "layout.widget_gap_px",
        "gpui: the gap between the settings' rows (demo::theme_settings)",
    ),
    (
        "chrome.status_bar",
        "layout.widget_gap_px",
        "gpui: not applied — StatusBar's own gap_2 between its items (status_bar.rs:88)",
    ),
    (
        "",
        "layout.widget_gap_px",
        "gpui: the gap between a group's heading and rows (pages/basic.rs)",
    ),
    (
        "chrome.content",
        "layout.window_margin_px",
        "gpui: each page pads itself by it (pages/basic.rs)",
    ),
    (
        "",
        "layout.window_margin_px",
        "gpui: the page's padding (pages/basic.rs)",
    ),
    // --- link: demo::link, upstream's Link -------------------------------------
    (
        "",
        "link.active_text_color",
        "not reachable: a pressed Link is `link` at 0.6 opacity (link.rs:84-85)",
    ),
    (
        "",
        "link.background_color",
        "gpui: not applied — the Link paints no fill (link.rs:70-90)",
    ),
    (
        "",
        "link.font",
        "gpui: geometry::link's size and weight; family: the window's",
    ),
    (
        "",
        "link.font.color",
        "gpui: ThemeColor link (colors.rs): the Link's text (link.rs:76)",
    ),
    (
        "",
        "link.hover_background",
        "not reachable: the Link's hover style is upstream's, no fill (link.rs:79-83)",
    ),
    (
        "",
        "link.hover_text_color",
        "not reachable: a hovered Link is `link` at 0.8 opacity (link.rs:79-80)",
    ),
    (
        "",
        "link.underline_enabled",
        "gpui: geometry::link removes upstream's underline where false",
    ),
    // --- list: the Basic page's table, demo::files_table ------------------------
    (
        "basic.table",
        "list.background_color",
        "gpui: the table's fill (demo::files_table)",
    ),
    (
        "basic.table",
        "list.border.color",
        "gpui: the table's frame (demo::files_table)",
    ),
    (
        "basic.table",
        "list.border.corner_radius_px",
        "gpui: the frame's corner radius (demo::files_table)",
    ),
    (
        "basic.table",
        "list.border.line_width_px",
        "gpui: the frame's width (demo::files_table)",
    ),
    (
        "basic.table",
        "list.border.padding_top_px",
        "gpui: pads each cell, none where unstated (demo::files_table)",
    ),
    (
        "basic.table",
        "list.border.padding_right_px",
        "gpui: pads each cell, none where unstated (demo::files_table)",
    ),
    (
        "basic.table",
        "list.border.padding_bottom_px",
        "gpui: pads each cell, none where unstated (demo::files_table)",
    ),
    (
        "basic.table",
        "list.border.padding_left_px",
        "gpui: pads each cell, none where unstated (demo::files_table)",
    ),
    (
        "basic.table",
        "list.hover_background",
        "gpui: an unselected row's fill under the pointer (demo::files_table)",
    ),
    (
        "basic.table",
        "list.hover_text_color",
        "gpui: an unselected row's text under the pointer (demo::files_table)",
    ),
    (
        "basic.table",
        "list.item_font",
        "gpui: each row's size and weight (demo::files_table); family: the window's",
    ),
    (
        "basic.table",
        "list.item_font.color",
        "gpui: an unselected row's text (demo::files_table)",
    ),
    (
        "basic.table",
        "list.row_height_px",
        "gpui: each row's height where stated, else padding and line (demo::files_table)",
    ),
    (
        "basic.table",
        "list.selection_background",
        "gpui: the selected row's fill (demo::files_table)",
    ),
    (
        "basic.table",
        "list.selection_text_color",
        "gpui: the selected row's text (demo::files_table)",
    ),
    (
        "",
        "list.grid_color",
        "gpui: the header's bottom line and the column line (demo::files_table)",
    ),
    (
        "",
        "list.header_background",
        "gpui: the header row's fill (demo::files_table)",
    ),
    (
        "",
        "list.header_font",
        "gpui: the header row's size and weight (demo::files_table); family: the window's",
    ),
    (
        "",
        "list.header_font.color",
        "gpui: the header row's text (demo::files_table)",
    ),
    // --- list: the List, demo::list and demo::ListRow ---------------------------
    (
        "",
        "list.background_color",
        "gpui: the List box's fill (demo::list)",
    ),
    (
        "",
        "list.border.color",
        "gpui: geometry::list's border colour",
    ),
    (
        "",
        "list.border.corner_radius_px",
        "gpui: geometry::list's corner radius",
    ),
    (
        "",
        "list.border.line_width_px",
        "gpui: geometry::list's border width",
    ),
    (
        "",
        "list.border.padding_top_px",
        "gpui: geometry::list_item pads each row by the stated side",
    ),
    (
        "",
        "list.border.padding_right_px",
        "gpui: geometry::list_item pads each row by the stated side",
    ),
    (
        "",
        "list.border.padding_bottom_px",
        "gpui: geometry::list_item pads each row by the stated side",
    ),
    (
        "",
        "list.border.padding_left_px",
        "gpui: geometry::list_item pads each row by the stated side",
    ),
    (
        "",
        "list.hover_background",
        "gpui: ThemeColor list_hover (colors.rs): a ListItem's hover fill",
    ),
    (
        "",
        "list.hover_text_color",
        "gpui: the label's colour under the pointer (demo::ListRow)",
    ),
    (
        "",
        "list.item_font",
        "gpui: geometry::list_item's size and weight; family: the window's",
    ),
    (
        "",
        "list.item_font.color",
        "gpui: geometry::list_item's text colour",
    ),
    (
        "",
        "list.row_height_px",
        "gpui: geometry::list_item's height where stated (control-height rule)",
    ),
    (
        "",
        "list.selection_background",
        "gpui: ThemeColor list_active (colors.rs): the selected ListItem's fill",
    ),
    (
        "",
        "list.selection_text_color",
        "gpui: the selected row's text colour (demo::ListRow)",
    ),
    // --- menu: the showcase's MenuBar (demo::MenuTitle, MenuBar::popup) ---------
    (
        "chrome.menu.theme.preferences.shortcut",
        "menu.font",
        "gpui: the row's font, which the plain Kbd text inherits (MenuLook; kbd.rs:234-236)",
    ),
    (
        "chrome.menu_bar.",
        "menu.border.padding_top_px",
        "gpui: pads the title by the stated side, else MENU_TITLE_PADDING_Y (demo::MenuTitle)",
    ),
    (
        "chrome.menu_bar.",
        "menu.border.padding_right_px",
        "gpui: pads the title by the stated side, else MENU_TITLE_PADDING_X (demo::MenuTitle)",
    ),
    (
        "chrome.menu_bar.",
        "menu.border.padding_bottom_px",
        "gpui: pads the title by the stated side, else MENU_TITLE_PADDING_Y (demo::MenuTitle)",
    ),
    (
        "chrome.menu_bar.",
        "menu.border.padding_left_px",
        "gpui: pads the title by the stated side, else MENU_TITLE_PADDING_X (demo::MenuTitle)",
    ),
    (
        "chrome.menu_bar.",
        "menu.hover_background",
        "gpui: the title's fill under the pointer and while open (demo::MenuTitle)",
    ),
    (
        "chrome.menu_bar.",
        "menu.hover_text_color",
        "gpui: the title's text under the pointer and while open (demo::MenuTitle)",
    ),
    (
        "",
        "menu.background_color",
        "gpui: the popup's fill (MenuLook, MenuBar::popup)",
    ),
    (
        "",
        "menu.border.padding_top_px",
        "gpui: pads the row by the stated side (MenuBar::popup)",
    ),
    (
        "",
        "menu.border.padding_right_px",
        "gpui: pads the row by the stated side, else MENU_ROW_PADDING_X (MenuBar::popup)",
    ),
    (
        "",
        "menu.border.padding_bottom_px",
        "gpui: pads the row by the stated side (MenuBar::popup)",
    ),
    (
        "",
        "menu.border.padding_left_px",
        "gpui: pads the row by the stated side, else MENU_ROW_PADDING_X (MenuBar::popup)",
    ),
    (
        "",
        "menu.font",
        "gpui: family, size and weight from MenuLook (demo::MenuTitle, MenuBar::popup)",
    ),
    (
        "",
        "menu.font.color",
        "gpui: the text colour from MenuLook (demo::MenuTitle, MenuBar::popup)",
    ),
    (
        "",
        "menu.hover_background",
        "gpui: the highlighted row's fill (MenuBar::popup)",
    ),
    (
        "",
        "menu.hover_text_color",
        "gpui: the highlighted row's text (MenuBar::popup)",
    ),
    (
        "",
        "menu.row_height_px",
        "gpui: the row's min_h where stated (MenuBar::popup)",
    ),
    (
        "",
        "menu.separator_color",
        "gpui: the separator row's fill (MenuLook)",
    ),
    // --- popover: the Theme menu's popup ----------------------------------------
    (
        "",
        "popover.border.color",
        "gpui: the popup's edge (MenuLook, MenuBar::popup)",
    ),
    (
        "",
        "popover.border.corner_radius_px",
        "gpui: the popup's corner radius (MenuLook)",
    ),
    (
        "",
        "popover.border.line_width_px",
        "gpui: the popup's border width (MenuLook)",
    ),
    (
        "",
        "popover.border.padding_top_px",
        "gpui: pads the popup's rows; upstream's p_1 where unstated (MenuLook, MenuBar::popup)",
    ),
    (
        "",
        "popover.border.padding_right_px",
        "gpui: pads the popup's rows; upstream's p_1 where unstated (MenuLook, MenuBar::popup)",
    ),
    (
        "",
        "popover.border.padding_bottom_px",
        "gpui: pads the popup's rows; upstream's p_1 where unstated (MenuLook, MenuBar::popup)",
    ),
    (
        "",
        "popover.border.padding_left_px",
        "gpui: pads the popup's rows; upstream's p_1 where unstated (MenuLook, MenuBar::popup)",
    ),
    (
        "",
        "popover.border.shadow_enabled",
        "gpui: the popup's shadow in defaults.shadow_color, upstream's layers, or none (MenuLook)",
    ),
    // --- progress_bar: widgets::ProgressBar -------------------------------------
    (
        "",
        "progress_bar.border.color",
        "gpui: the track's edge (ProgressBarLook)",
    ),
    (
        "",
        "progress_bar.border.corner_radius_px",
        "gpui: the track's and the fill's corner radius (ProgressBarLook)",
    ),
    (
        "",
        "progress_bar.border.line_width_px",
        "gpui: the track's border width (ProgressBarLook)",
    ),
    (
        "",
        "progress_bar.fill_color",
        "gpui: widgets::ProgressBar's fill (ProgressBarLook)",
    ),
    (
        "",
        "progress_bar.min_width_px",
        "gpui: the bar's min_w (widgets::ProgressBar)",
    ),
    (
        "",
        "progress_bar.track_color",
        "gpui: the track's fill (ProgressBarLook)",
    ),
    (
        "",
        "progress_bar.track_height_px",
        "gpui: the bar's height (widgets::ProgressBar)",
    ),
    // --- scrollbar: gpui-base's, through base_layer -----------------------------
    (
        "",
        "scrollbar.groove_width_px",
        "gpui: the track width written onto gpui-base (base_layer::scrollbar_geometry)",
    ),
    (
        "",
        "scrollbar.min_thumb_length_px",
        "gpui: the thumb's shortest length on gpui-base (base_layer::scrollbar_geometry)",
    ),
    (
        "",
        "scrollbar.overlay_mode",
        "gpui: Theme::scrollbar_mode, always shown where false (lib.rs, apply)",
    ),
    (
        "",
        "scrollbar.thumb_active_color",
        "gpui: gpui-base's dragged thumb, the hover colour where unstated (base_layer)",
    ),
    (
        "",
        "scrollbar.thumb_color",
        "gpui: gpui-base's thumb at rest (base_layer::scrollbar_styles)",
    ),
    (
        "",
        "scrollbar.thumb_hover_color",
        "gpui: gpui-base's thumb under the pointer (base_layer::scrollbar_styles)",
    ),
    (
        "",
        "scrollbar.thumb_width_px",
        "gpui: gpui-base's thumb width in every state (base_layer::scrollbar_geometry)",
    ),
    (
        "",
        "scrollbar.track_color",
        "gpui: gpui-base's track in every state (base_layer::scrollbar_styles)",
    ),
    // --- segmented_control: demo::segmented -------------------------------------
    (
        "",
        "segmented_control.active_background",
        "gpui: the selected segment's fill (demo::segmented)",
    ),
    (
        "",
        "segmented_control.active_text_color",
        "gpui: the selected segment's text (demo::segmented)",
    ),
    (
        "",
        "segmented_control.background_color",
        "gpui: the control's fill (demo::segmented)",
    ),
    (
        "",
        "segmented_control.border.color",
        "gpui: the frame and the lines between segments (demo::segmented)",
    ),
    (
        "",
        "segmented_control.border.corner_radius_px",
        "gpui: the frame's corner radius (demo::segmented)",
    ),
    (
        "",
        "segmented_control.border.line_width_px",
        "gpui: the frame's width (demo::segmented)",
    ),
    (
        "",
        "segmented_control.border.padding_top_px",
        "gpui: pads each segment by the stated side (demo::segmented)",
    ),
    (
        "",
        "segmented_control.border.padding_right_px",
        "gpui: pads each segment, SEGMENT_PADDING where unstated (demo::segmented)",
    ),
    (
        "",
        "segmented_control.border.padding_bottom_px",
        "gpui: pads each segment by the stated side (demo::segmented)",
    ),
    (
        "",
        "segmented_control.border.padding_left_px",
        "gpui: pads each segment, SEGMENT_PADDING where unstated (demo::segmented)",
    ),
    (
        "",
        "segmented_control.font",
        "gpui: each label's size and weight (demo::segmented); family: the window's",
    ),
    (
        "",
        "segmented_control.font.color",
        "gpui: an unselected segment's text (demo::segmented)",
    ),
    (
        "",
        "segmented_control.hover_background",
        "gpui: an unselected segment's fill under the pointer (demo::segmented)",
    ),
    (
        "",
        "segmented_control.segment_height_px",
        "gpui: each segment's min_h (demo::segmented)",
    ),
    (
        "",
        "segmented_control.separator_width_px",
        "gpui: the line left of each segment after the first (demo::segmented)",
    ),
    // --- separator ------------------------------------------------------------------
    (
        "chrome.page_tabs.rule",
        "separator.line_color",
        "gpui: the tab strip's bottom border (demo::native_tab_strip)",
    ),
    (
        "chrome.page_tabs.rule",
        "separator.line_width_px",
        "gpui: the tab strip's bottom border (demo::native_tab_strip)",
    ),
    (
        "chrome.menu.",
        "separator.line_width_px",
        "gpui: the separator row's height (MenuLook)",
    ),
    (
        "",
        "separator.line_color",
        "gpui: widgets::Separator's line (SeparatorLook)",
    ),
    (
        "",
        "separator.line_width_px",
        "gpui: widgets::Separator's thickness (SeparatorLook)",
    ),
    // --- sidebar ----------------------------------------------------------------
    (
        "",
        "sidebar.background_color",
        "gpui: ThemeColor sidebar (colors.rs): the side panel's fill (demo::side_panel)",
    ),
    (
        "",
        "sidebar.border.padding_top_px",
        "gpui: pads the panel where stated; else layout.container_margin pads its content (demo::side_panel)",
    ),
    (
        "",
        "sidebar.border.padding_right_px",
        "gpui: pads the panel where stated; else layout.container_margin pads its content (demo::side_panel)",
    ),
    (
        "",
        "sidebar.border.padding_bottom_px",
        "gpui: pads the panel where stated; else layout.container_margin pads its content (demo::side_panel)",
    ),
    (
        "",
        "sidebar.border.padding_left_px",
        "gpui: pads the panel where stated; else layout.container_margin pads its content (demo::side_panel)",
    ),
    (
        "chrome.info",
        "sidebar.font",
        "gpui: size and family of every Widget Info line (inspector.rs)",
    ),
    (
        "",
        "sidebar.font",
        "gpui: the label's size and weight (demo::sidebar_label); family: the window's",
    ),
    (
        "chrome.info",
        "sidebar.font.color",
        "gpui: the Widget Info text colour (inspector.rs)",
    ),
    (
        "",
        "sidebar.font.color",
        "gpui: ThemeColor sidebar_foreground (colors.rs): the side panel's text (demo::side_panel)",
    ),
    // --- slider: widgets::Slider --------------------------------------------------
    (
        "",
        "slider.fill_color",
        "gpui: the rail's filled stretch (SliderLook)",
    ),
    (
        "",
        "slider.thumb_color",
        "gpui: the thumb's fill (SliderLook)",
    ),
    (
        "",
        "slider.thumb_diameter_px",
        "gpui: the thumb's size (widgets::Slider)",
    ),
    (
        "",
        "slider.thumb_hover_color",
        "gpui: over the thumb under the pointer (SliderLook)",
    ),
    (
        "",
        "slider.track_color",
        "gpui: the rail's fill (SliderLook)",
    ),
    (
        "",
        "slider.track_height_px",
        "gpui: the rail's height (widgets::Slider)",
    ),
    // --- spinner: widgets::Spinner --------------------------------------------------
    (
        "",
        "spinner.diameter_px",
        "gpui: the indicator's size (SpinnerLook)",
    ),
    (
        "",
        "spinner.fill_color",
        "gpui: the arc, and a monochrome set's indicator tint (SpinnerLook)",
    ),
    (
        "",
        "spinner.min_diameter_px",
        "gpui: the floor under the indicator's size (SpinnerLook)",
    ),
    (
        "",
        "spinner.stroke_width_px",
        "gpui: the arc's stroke, for a set with no indicator (SpinnerLook)",
    ),
    // --- splitter: demo::resize_handles --------------------------------------------
    (
        "",
        "splitter.divider_color",
        "gpui: gpui-base's resize handle colour (base_layer::resizable_theme)",
    ),
    (
        "",
        "splitter.divider_width_px",
        "gpui: the painted line's width (demo::resize_handles)",
    ),
    (
        "",
        "splitter.hover_color",
        "gpui: the line under the pointer and while dragged (demo::resize_handles, base_layer)",
    ),
    // --- status_bar: upstream's StatusBar, demo::status_bar -----------------------
    (
        "",
        "status_bar.background_color",
        "gpui: ThemeColor status_bar (colors.rs): the bar's fill (status_bar.rs:93)",
    ),
    (
        "",
        "status_bar.border.color",
        "gpui: ThemeColor status_bar_border (colors.rs): the top edge (status_bar.rs:92)",
    ),
    (
        "",
        "status_bar.border.line_width_px",
        "gpui: the top edge's width (demo::status_bar)",
    ),
    (
        "",
        "status_bar.border.padding_top_px",
        "gpui: geometry::status_bar pads the stated side; upstream's py_1 the rest",
    ),
    (
        "",
        "status_bar.border.padding_right_px",
        "gpui: geometry::status_bar pads the stated side; upstream's px_2 the rest",
    ),
    (
        "",
        "status_bar.border.padding_bottom_px",
        "gpui: geometry::status_bar pads the stated side; upstream's py_1 the rest",
    ),
    (
        "",
        "status_bar.border.padding_left_px",
        "gpui: geometry::status_bar pads the stated side; upstream's px_2 the rest",
    ),
    (
        "",
        "status_bar.font",
        "gpui: geometry::status_bar's size and weight; family: the window's",
    ),
    (
        "",
        "status_bar.font.color",
        "gpui: geometry::status_bar's text colour",
    ),
    // --- switch: widgets::Switch ---------------------------------------------------
    (
        "",
        "switch.checked_background",
        "gpui: the track's fill when on (SwitchLook)",
    ),
    (
        "",
        "switch.disabled_checked_background",
        "gpui: a disabled switch's track when on, where stated (SwitchLook)",
    ),
    (
        "",
        "switch.disabled_opacity",
        "gpui: fades the whole disabled switch (widgets::Switch)",
    ),
    (
        "",
        "switch.disabled_thumb_color",
        "gpui: a disabled switch's thumb, where stated (SwitchLook)",
    ),
    (
        "",
        "switch.hover_checked_background",
        "gpui: over the on track under the pointer (SwitchLook)",
    ),
    (
        "",
        "switch.hover_unchecked_background",
        "gpui: over the off track under the pointer (SwitchLook)",
    ),
    (
        "",
        "switch.thumb_background",
        "gpui: the thumb's fill (SwitchLook)",
    ),
    (
        "",
        "switch.thumb_diameter_px",
        "gpui: the thumb's size, centred in the track (SwitchLook)",
    ),
    (
        "",
        "switch.unchecked_thumb_background",
        "gpui: the off thumb's fill, thumb_background where unstated (SwitchLook)",
    ),
    (
        "",
        "switch.unchecked_thumb_diameter_px",
        "gpui: the off thumb's size, thumb_diameter where unstated (SwitchLook)",
    ),
    (
        "",
        "switch.track_height_px",
        "gpui: the track's height (SwitchLook)",
    ),
    (
        "",
        "switch.track_radius_px",
        "gpui: the track's corner radius (SwitchLook)",
    ),
    (
        "",
        "switch.track_width_px",
        "gpui: the track's width (SwitchLook)",
    ),
    (
        "",
        "switch.unchecked_background",
        "gpui: the track's fill when off (SwitchLook)",
    ),
    // --- tab: widgets::TabBar ----------------------------------------------------
    (
        "",
        "tab.active_background",
        "gpui: the selected tab's fill (TabLook)",
    ),
    (
        "",
        "tab.active_text_color",
        "gpui: the selected tab's text (TabLook)",
    ),
    (
        "",
        "tab.background_color",
        "gpui: an unselected tab's fill (TabLook)",
    ),
    (
        "",
        "tab.bar_background",
        "gpui: the tab bar's fill (widgets::TabBar)",
    ),
    (
        "",
        "tab.border.color",
        "gpui: the selected tab's outline (TabLook)",
    ),
    (
        "",
        "tab.border.corner_radius_px",
        "gpui: the selected tab's top corners (TabLook)",
    ),
    (
        "",
        "tab.border.line_width_px",
        "gpui: every tab's border width, shown on the selected one (widgets::TabBar)",
    ),
    (
        "",
        "tab.border.padding_top_px",
        "gpui: pads each tab by the stated side (TabLook)",
    ),
    (
        "",
        "tab.border.padding_right_px",
        "gpui: pads each tab, gpui-component's 12px where unstated (TabLook)",
    ),
    (
        "",
        "tab.border.padding_bottom_px",
        "gpui: pads each tab by the stated side (TabLook)",
    ),
    (
        "",
        "tab.border.padding_left_px",
        "gpui: pads each tab, gpui-component's 12px where unstated (TabLook)",
    ),
    (
        "",
        "tab.font",
        "gpui: each label's size and weight (widgets::TabBar); family: the window's",
    ),
    (
        "",
        "tab.font.color",
        "gpui: an unselected tab's text (TabLook)",
    ),
    (
        "",
        "tab.hover_background",
        "gpui: an unselected tab's fill under the pointer, over the bar (TabLook)",
    ),
    (
        "",
        "tab.hover_text_color",
        "gpui: an unselected tab's text under the pointer (TabLook)",
    ),
    (
        "",
        "tab.item_gap_px",
        "gpui: the gap between tabs (TabLook)",
    ),
    (
        "",
        "tab.active_indicator_color",
        "gpui: the selected tab's line (TabLook, TabIndicator)",
    ),
    (
        "",
        "tab.active_indicator_side",
        "gpui: the edge the selected tab's line lies along (TabIndicator)",
    ),
    (
        "",
        "tab.active_indicator_width_px",
        "gpui: the selected tab's line's thickness (TabIndicator)",
    ),
    ("", "tab.min_height_px", "gpui: each tab's min_h (TabLook)"),
    ("", "tab.min_width_px", "gpui: each tab's min_w (TabLook)"),
    // --- text_area: demo::rows_textarea ------------------------------------------
    (
        "",
        "text_area.border.color",
        "gpui: not applied — the Textarea's edge is the `input` token, input.border.color (input/input.rs:788)",
    ),
    (
        "",
        "text_area.border.corner_radius_px",
        "gpui: geometry::text_area's corner radius",
    ),
    (
        "",
        "text_area.border.line_width_px",
        "gpui: geometry::text_area's border width",
    ),
    (
        "",
        "text_area.border.padding_top_px",
        "gpui: geometry::text_area pads past the editor's own padding",
    ),
    (
        "",
        "text_area.border.padding_right_px",
        "gpui: geometry::text_area pads past the editor's own padding",
    ),
    (
        "",
        "text_area.border.padding_bottom_px",
        "gpui: geometry::text_area pads past the editor's own padding",
    ),
    (
        "",
        "text_area.border.padding_left_px",
        "gpui: geometry::text_area pads past the editor's own padding",
    ),
    (
        "",
        "text_area.border.shadow_enabled",
        "gpui: not applied — the Textarea draws no shadow (input/input.rs)",
    ),
    // --- text_scale ------------------------------------------------------------------
    (
        "",
        "text_scale.caption",
        "gpui: size, weight and line height on the Label (demo::type_line)",
    ),
    (
        "",
        "text_scale.dialog_title",
        "gpui: size, weight and line height on the Label (demo::type_line)",
    ),
    (
        "basic.typography.section_heading",
        "text_scale.section_heading",
        "gpui: size, weight and line height on the Label (demo::type_line)",
    ),
    (
        "",
        "text_scale.section_heading",
        "gpui: size, weight and line height on the heading Label (demo::heading)",
    ),
    // --- theme_variant ---------------------------------------------------------------
    (
        "basic.spinner",
        "theme_variant.icon_set",
        "gpui: widgets::Spinner draws that set's own loading indicator (spinner.rs)",
    ),
    (
        "",
        "theme_variant.icon_set",
        "gpui: the set the icon-theme Select names, by default the theme's (app.rs)",
    ),
    // --- toolbar -----------------------------------------------------------------------
    (
        "chrome.toolbar.",
        "toolbar.font",
        "gpui: set on the row (geometry::toolbar); the button draws an icon only",
    ),
    (
        "",
        "toolbar.background_color",
        "gpui: geometry::toolbar's fill",
    ),
    (
        "",
        "toolbar.bar_height_px",
        "gpui: geometry::toolbar's min_h where stated",
    ),
    (
        "",
        "toolbar.border.color",
        "gpui: the bar's bottom edge colour (demo::toolbar, border_color)",
    ),
    (
        "",
        "toolbar.border.line_width_px",
        "gpui: the bar's bottom edge, inside it (demo::toolbar, border_b)",
    ),
    (
        "",
        "toolbar.border.padding_top_px",
        "gpui: geometry::toolbar pads the stated side; container_margin the rest (demo::toolbar)",
    ),
    (
        "",
        "toolbar.border.padding_right_px",
        "gpui: geometry::toolbar pads the stated side; container_margin the rest (demo::toolbar)",
    ),
    (
        "",
        "toolbar.border.padding_bottom_px",
        "gpui: geometry::toolbar pads the stated side; container_margin the rest (demo::toolbar)",
    ),
    (
        "",
        "toolbar.border.padding_left_px",
        "gpui: geometry::toolbar pads the stated side; container_margin the rest (demo::toolbar)",
    ),
    (
        "",
        "toolbar.font",
        "gpui: geometry::toolbar's size and weight on the row; no text is drawn in it",
    ),
    (
        "",
        "toolbar.icon_size_px",
        "gpui: geometry::icon_size_toolbar sizes the icon",
    ),
    (
        "",
        "toolbar.item_gap_px",
        "gpui: geometry::toolbar's gap where stated, else layout.widget_gap (demo::toolbar)",
    ),
    // --- tooltip: demo::built_tooltip_button --------------------------------------
    (
        "",
        "tooltip.background_color",
        "gpui: geometry::tooltip's fill",
    ),
    (
        "",
        "tooltip.border.color",
        "gpui: geometry::tooltip's border colour",
    ),
    (
        "",
        "tooltip.border.corner_radius_px",
        "gpui: geometry::tooltip's corner radius",
    ),
    (
        "",
        "tooltip.border.line_width_px",
        "gpui: geometry::tooltip's border width",
    ),
    (
        "",
        "tooltip.border.padding_top_px",
        "gpui: geometry::tooltip pads the stated side",
    ),
    (
        "",
        "tooltip.border.padding_right_px",
        "gpui: geometry::tooltip pads the stated side",
    ),
    (
        "",
        "tooltip.border.padding_bottom_px",
        "gpui: geometry::tooltip pads the stated side",
    ),
    (
        "",
        "tooltip.border.padding_left_px",
        "gpui: geometry::tooltip pads the stated side",
    ),
    (
        "",
        "tooltip.border.shadow_enabled",
        "gpui: geometry::tooltip's shadow: shadow_md's layers in defaults.shadow_color, or none",
    ),
    (
        "",
        "tooltip.font",
        "gpui: geometry::tooltip's size and weight; family: the window's",
    ),
    (
        "",
        "tooltip.font.color",
        "gpui: geometry::tooltip's text colour",
    ),
    (
        "",
        "tooltip.max_width_px",
        "gpui: geometry::tooltip_content, less the bubble's padding and border",
    ),
    // --- window ---------------------------------------------------------------------------
    (
        "",
        "window.background_color",
        "gpui: ThemeColor background (colors.rs): the window's fill (Showcase::render)",
    ),
];

/// The line for `leaf` of the element `element`, per [`HOW`].
pub fn how(element: &str, leaf: &str) -> Option<&'static str> {
    HOW.iter()
        .find(|(prefix, l, _)| *l == leaf && element.starts_with(prefix))
        .map(|(_, _, line)| *line)
}
