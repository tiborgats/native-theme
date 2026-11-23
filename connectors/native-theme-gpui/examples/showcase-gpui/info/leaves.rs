//! How gpui applies each theme leaf an element of docs/showcase-elements.toml
//! draws: the muted line Widget Info shows under each `<leaf> <value>` row
//! (the round-11 brief, section B). A line names the route the value takes --
//! the connector's `ThemeColor` field (`colors.rs`), its `geometry::` builder,
//! its `widgets::` control, or the showcase helper that sets it -- or says
//! "not reachable" with the upstream line that keeps it out. Where the
//! showcase could apply a leaf and does not, the line says so and names what
//! is drawn instead. Upstream citations are to gpui-component 0.9.1,
//! gpui-base 0.9.1 and gpui-pre 0.3.12, but where one names its version.

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
    (
        "basic.buttons.toggle_on",
        "button.hover_border_color",
        "not reachable: a selected Button takes no hover or press style, so the toggle that is on draws no hover border (gpui-component/button/button.rs, RenderOnce for Button)",
    ),
    // --- button: a dialog's default button, variants::default_button ----------
    (
        "basic.buttons.dialog_default",
        "button.default_button_background",
        "gpui: geometry::default_button_rest's bg at rest, after geometry::button, and variants::default_button's hover fill; where unstated the Button is .primary(), ThemeColor primary",
    ),
    (
        "basic.buttons.dialog_default",
        "button.default_button_text_color",
        "gpui: variants::default_button's foreground, in every state; where the theme states no button.default_button_background the Primary Button's button_primary_foreground",
    ),
    (
        "basic.buttons.dialog_default",
        "button.default_button_border_color",
        "gpui: geometry::default_button_rest's border colour at rest, after geometry::button; under the pointer and pressed variants::default_button's color, button.hover_border_color where stated",
    ),
    (
        "basic.buttons.dialog_default",
        "button.border.color",
        "gpui: geometry::button's border colour at rest where the Button is .primary(); where the theme states button.default_button_background, the fallback of geometry::default_button_rest's frame and of variants::default_button's colour where button.default_button_border_color and button.hover_border_color are unstated",
    ),
    (
        "basic.buttons.dialog_default",
        "button.hover_border_color",
        "gpui: variants::default_button's color, the border under the pointer and pressed, where the theme states button.default_button_background; elsewhere the Button is .primary(), whose hover and press border is the primary token (gpui-component 0.10.0 button/button.rs:1104, primary) and whose rest border is geometry::button's button.border.color (native-theme-gpui/geometry.rs:285, border_color)",
    ),
    (
        "basic.buttons.dialog_default",
        "button.font.color",
        "gpui: variants::default_button_states's label where the theme states button.default_button_background and no button.default_button_text_color, the variant's foreground in every state; elsewhere the Button is .primary(), lettered in ThemeColor button_primary_foreground (gpui-component 0.10.0 button/button.rs:1055, button_primary_foreground)",
    ),
    (
        "basic.buttons.dialog_default",
        "button.active_background",
        "gpui: variants::default_button's active, ThemeColor button_active (colors.rs), the pressed fill, where the theme states button.default_button_background; elsewhere the Button is .primary(), pressed in ThemeColor button_primary_active (gpui-component 0.10.0 button/button.rs:1271, button_primary_active), which the connector sets to primary_active, a shade of primary (colors.rs)",
    ),
    // --- button: the Basic page's buttons, variants::bordered_button -----------
    (
        "basic.buttons.default",
        "button.hover_border_color",
        "gpui: variants::bordered_button's colour, the border hovered and pressed, where the theme states one; at rest, selected and disabled the border is geometry::button's button.border.color, which upstream replays over those states (gpui-component/button/button.rs, RenderOnce for Button); else the Button is upstream's Default (demo::ButtonKind)",
    ),
    (
        "basic.buttons.tooltip",
        "button.hover_border_color",
        "gpui: variants::bordered_button's colour, the border hovered and pressed, where the theme states one; at rest, selected and disabled the border is geometry::button's button.border.color, which upstream replays over those states (gpui-component/button/button.rs, RenderOnce for Button); else the Button is upstream's Default (demo::ButtonKind)",
    ),
    (
        "basic.buttons.toggle_off",
        "button.hover_border_color",
        "gpui: variants::bordered_button's colour, the border hovered and pressed, where the theme states one; at rest, selected and disabled the border is geometry::button's button.border.color, which upstream replays over those states (gpui-component/button/button.rs, RenderOnce for Button); else the Button is upstream's Default (demo::ButtonKind)",
    ),
    (
        "basic.buttons.primary",
        "button.hover_border_color",
        "not reachable: a Primary Button's hover and press border is the primary token (gpui-component 0.10.0 button/button.rs:1104, primary); at rest it is geometry::button's button.border.color (native-theme-gpui/geometry.rs:285, border_color)",
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
        "not reachable: a Custom variant has no hover text colour (gpui-component/button/button.rs:104-149, ButtonCustomVariant; :1225, Custom; :1239-1244, text_color)",
    ),
    (
        "chrome.page_tabs.menu",
        "button.active_background",
        "gpui: variants::ghost_button's active, ThemeColor secondary_active (colors.rs)",
    ),
    (
        "chrome.page_tabs.menu",
        "button.border.corner_radius_px",
        "gpui: button.body(NONE).corners rounds the tool button's box where stated (demo::tool_button_box)",
    ),
    (
        "chrome.page_tabs.menu",
        "button.border.padding_top_px",
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.page_tabs.menu",
        "button.border.padding_right_px",
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.page_tabs.menu",
        "button.border.padding_bottom_px",
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.page_tabs.menu",
        "button.border.padding_left_px",
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
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
    // --- button: the icon-only buttons, variants::toolbar_button and ghost_button
    (
        "basic.icons.edit",
        "toolbar.button_checked_background",
        "gpui: toolbar.button(CHECKED).fill, next::geometry::toolbar_button_checked's bg, over the selected toolbar Button",
    ),
    (
        "basic.icons.edit",
        "button.checked_text_color",
        "gpui: button.label(CHECKED).color, next::geometry::toolbar_button_checked's text colour, where a rule states the toolbar's checked fill, toolbar.button(CHECKED).fill",
    ),
    (
        "basic.icons.edit",
        "toolbar.button_checked_border_color",
        "gpui: toolbar.button(CHECKED).stroke.color, next::geometry::toolbar_button_checked's border colour, at next::geometry::toolbar_button_rest's width, kept under the pointer and pressed: a selected Button takes no hover or press style (gpui-component 0.10.0 button/button.rs:733, selected; gpui-component 0.10.0 button/button.rs:758, selected)",
    ),
    (
        "chrome.status_bar.toggle",
        "button.active_background",
        "gpui: ghost_button's active, secondary_active; also its selected fill (gpui-component/button/button.rs:1353, active)",
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
        "gpui: next::variants::toolbar_button's hover where no rule states toolbar.button(HOVER).fill, ThemeColor secondary_hover (colors.rs)",
    ),
    (
        "basic.icons.",
        "button.hover_background",
        "gpui: next::variants::toolbar_button's hover where no rule states toolbar.button(HOVER).fill, ThemeColor secondary_hover (colors.rs)",
    ),
    (
        "chrome.status_bar.toggle",
        "button.hover_background",
        "gpui: variants::ghost_button's hover, ThemeColor secondary_hover (colors.rs)",
    ),
    (
        "chrome.toolbar.",
        "button.hover_text_color",
        "not reachable: a Custom variant has no hover text colour (gpui-component/button/button.rs:104-149, ButtonCustomVariant; :1225, Custom; :1239-1244, text_color)",
    ),
    (
        "basic.icons.",
        "button.hover_text_color",
        "not reachable: a Custom variant has no hover text colour (gpui-component/button/button.rs:104-149, ButtonCustomVariant; :1225, Custom; :1239-1244, text_color)",
    ),
    (
        "chrome.status_bar.toggle",
        "button.hover_text_color",
        "not reachable: a Custom variant has no hover text colour (gpui-component/button/button.rs:104-149, ButtonCustomVariant; :1225, Custom; :1239-1244, text_color)",
    ),
    (
        "chrome.toolbar.",
        "button.active_background",
        "gpui: next::variants::toolbar_button's active where no rule states toolbar.button(PRESSED).fill, ThemeColor secondary_active (colors.rs)",
    ),
    (
        "basic.icons.",
        "button.active_background",
        "gpui: next::variants::toolbar_button's active where no rule states toolbar.button(PRESSED).fill, ThemeColor secondary_active (colors.rs)",
    ),
    (
        "chrome.toolbar.",
        "button.border.corner_radius_px",
        "gpui: button.body(NONE).corners rounds the tool button's box where stated (demo::tool_button_box)",
    ),
    (
        "basic.icons.",
        "button.border.corner_radius_px",
        "gpui: button.body(NONE).corners rounds the tool button's box where stated (demo::tool_button_box)",
    ),
    (
        "chrome.status_bar.toggle",
        "button.border.corner_radius_px",
        "gpui: button.body(NONE).corners rounds the tool button's box where stated (demo::tool_button_box)",
    ),
    (
        "chrome.toolbar.",
        "button.border.padding_top_px",
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.toolbar.",
        "button.border.padding_right_px",
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.toolbar.",
        "button.border.padding_bottom_px",
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.toolbar.",
        "button.border.padding_left_px",
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "basic.icons.",
        "button.border.padding_top_px",
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "basic.icons.",
        "button.border.padding_right_px",
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "basic.icons.",
        "button.border.padding_bottom_px",
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "basic.icons.",
        "button.border.padding_left_px",
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.toolbar.",
        "button.border.line_width_px",
        "gpui: button.body(NONE).stroke.width, next::geometry::toolbar_button_rest's border width, where a rule states toolbar.button(HOVER).stroke.color",
    ),
    (
        "basic.icons.",
        "button.border.line_width_px",
        "gpui: button.body(NONE).stroke.width, next::geometry::toolbar_button_rest's border width, where a rule states toolbar.button(HOVER).stroke.color",
    ),
    (
        "chrome.toolbar.",
        "toolbar.button_hover_background",
        "gpui: toolbar.button(HOVER).fill, next::variants::toolbar_button's hover",
    ),
    (
        "basic.icons.",
        "toolbar.button_hover_background",
        "gpui: toolbar.button(HOVER).fill, next::variants::toolbar_button's hover",
    ),
    (
        "chrome.toolbar.",
        "toolbar.button_active_background",
        "gpui: toolbar.button(PRESSED).fill, next::variants::toolbar_button's pressed fill",
    ),
    (
        "basic.icons.",
        "toolbar.button_active_background",
        "gpui: toolbar.button(PRESSED).fill, next::variants::toolbar_button's pressed fill",
    ),
    (
        "chrome.toolbar.",
        "toolbar.button_hover_border_color",
        "gpui: toolbar.button(HOVER).stroke.color, next::variants::toolbar_button's edge under the pointer",
    ),
    (
        "basic.icons.",
        "toolbar.button_hover_border_color",
        "gpui: toolbar.button(HOVER).stroke.color, next::variants::toolbar_button's edge under the pointer",
    ),
    (
        "chrome.status_bar.toggle",
        "button.border.padding_top_px",
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.status_bar.toggle",
        "button.border.padding_right_px",
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.status_bar.toggle",
        "button.border.padding_bottom_px",
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.status_bar.toggle",
        "button.border.padding_left_px",
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    // --- button: a Default Button refined by geometry::button ------------------
    (
        "",
        "button.active_background",
        "gpui: ThemeColor button_active, laid over the fill source-over (colors.rs, over); the pressed fill",
    ),
    (
        "",
        "button.active_text_color",
        "not reachable: a pressed Button keeps its variant's text colour (gpui-component/button/button.rs:1315-1319, text_color)",
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
        "not reachable: only a Custom variant takes a shadow (gpui-component/button/button.rs:1151-1156, shadow)",
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
        "button.focus_border_color",
        "not reachable: with gpui-component's band off, a focused Button's border is ThemeColor ring where it has a border width, and where it has none a 1 px line: inside it in its text colour at 60 %, but just outside it in ring on a link or a text Button and on its edge in ring on an upstream ghost Button; with the band on, its border is ring and the band is drawn round it (styled.rs, focus_style)",
    ),
    (
        "",
        "button.font",
        "gpui: `geometry::button_label` gives the label, the Button's child, `button.font`'s family, size, weight and style (demo::labelled); a `.label()` would be upstream's `text_sm` at Medium (gpui-component/sizing.rs:337-343, button_text_size)",
    ),
    (
        "",
        "button.font.color",
        "gpui: ThemeColor button_foreground (colors.rs): the label's colour",
    ),
    (
        "",
        "button.hover_background",
        "gpui: ThemeColor button_hover, laid over the fill source-over (colors.rs, over): the hover fill",
    ),
    (
        "",
        "button.hover_text_color",
        "not reachable: a hovered Button keeps its variant's text colour (gpui-component/button/button.rs:1239-1244, text_color)",
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
        "gpui: geometry::group_box_content's bg",
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
        "gpui: checkbox.indicator(NONE).fill (radio.indicator(NONE).fill), the unchecked indicator's fill (CheckboxLook::next_of)",
    ),
    (
        "",
        "checkbox.border.color",
        "gpui: checkbox.indicator(CHECKED).stroke.color (radio.indicator(CHECKED)), the checked indicator's border (CheckboxLook::next_of)",
    ),
    (
        "",
        "checkbox.border.corner_radius_px",
        "gpui: checkbox.indicator(NONE).corners, the box's corner radius (CheckboxLook::next_of; a radio is round)",
    ),
    (
        "",
        "checkbox.border.line_width_px",
        "gpui: checkbox.indicator(NONE).stroke.width (radio.indicator(NONE)), the indicator's border width (CheckboxLook::next_of)",
    ),
    (
        "",
        "checkbox.border.padding_top_px",
        "gpui: checkbox.indicator(NONE).padding, the stated side, insets the check mark in the box (CheckboxLook::next_of, CheckboxLook::mark_size)",
    ),
    (
        "",
        "checkbox.border.padding_right_px",
        "gpui: checkbox.indicator(NONE).padding, the stated side, insets the check mark in the box (CheckboxLook::next_of, CheckboxLook::mark_size)",
    ),
    (
        "",
        "checkbox.border.padding_bottom_px",
        "gpui: checkbox.indicator(NONE).padding, the stated side, insets the check mark in the box (CheckboxLook::next_of, CheckboxLook::mark_size)",
    ),
    (
        "",
        "checkbox.border.padding_left_px",
        "gpui: checkbox.indicator(NONE).padding, the stated side, insets the check mark in the box (CheckboxLook::next_of, CheckboxLook::mark_size)",
    ),
    (
        "",
        "checkbox.check_mark_stroke_width_px",
        "gpui: checkbox.checkmark(NONE).stroke_width, the check mark's stroke (CheckboxLook::next_of, stroked_check)",
    ),
    (
        "",
        "checkbox.checked_background",
        "gpui: checkbox.indicator(CHECKED).fill (radio.indicator(CHECKED).fill), the checked indicator's fill (CheckboxLook::next_of)",
    ),
    (
        "",
        "checkbox.disabled_background",
        "gpui: checkbox.indicator(DISABLED).fill (DISABLED|CHECKED checked; radio.indicator), a disabled indicator's fill where a rule states it (CheckboxLook::next_of)",
    ),
    (
        "",
        "checkbox.disabled_opacity",
        "gpui: checkbox.disabled_opacity (radio.disabled_opacity), fades the whole disabled control (CheckboxLook::next_of)",
    ),
    (
        "",
        "checkbox.disabled_text_color",
        "gpui: checkbox.label(DISABLED).color (radio.label(DISABLED)), a disabled control's label, and its mark over a stated disabled fill (CheckboxLook::next_of)",
    ),
    (
        "",
        "checkbox.font",
        "gpui: checkbox.label(NONE) (radio.label(NONE)), the label's family, size, weight and style (widgets::Checkbox, widgets::Radio)",
    ),
    (
        "",
        "checkbox.font.color",
        "gpui: checkbox.label(NONE).color (radio.label(NONE).color), the label's colour (CheckboxLook::next_of)",
    ),
    (
        "",
        "checkbox.hover_background",
        "gpui: checkbox.indicator(HOVER).fill (radio.indicator(HOVER).fill), over the unchecked fill under the pointer where a rule states it; else the fill at rest (CheckboxLook::next_of)",
    ),
    (
        "",
        "checkbox.hover_border_color",
        "gpui: checkbox.indicator(HOVER).stroke.color (HOVER|CHECKED checked; radio.indicator), the indicator's edge under the pointer where a rule states it; else the edge at rest (CheckboxLook::next_of)",
    ),
    (
        "",
        "checkbox.indicator_color",
        "gpui: checkbox.checkmark(CHECKED).color and radio.dot(CHECKED).fill, the check mark's and the radio dot's colour (CheckboxLook::next_of)",
    ),
    (
        "",
        "checkbox.indicator_width_px",
        "gpui: checkbox.indicator(NONE).min_size[0] (radio.indicator(NONE).min_size[0]), the box's side, the circle's diameter (CheckboxLook::next_of)",
    ),
    (
        "",
        "checkbox.label_gap_px",
        "gpui: checkbox.label_gap (radio.label_gap), the gap between indicator and label (CheckboxLook::next_of)",
    ),
    (
        "",
        "checkbox.radio_dot_diameter_px",
        "gpui: radio.dot(NONE).min_size[0], the selected radio's dot (CheckboxLook::next_radio)",
    ),
    (
        "",
        "checkbox.radio_indicator_width_px",
        "gpui: radio.indicator(NONE).min_size[0], the radio's circle (CheckboxLook::next_radio)",
    ),
    (
        "",
        "checkbox.unchecked_background",
        "gpui: checkbox.indicator(NONE).fill (radio.indicator(NONE).fill), the unchecked indicator's fill (CheckboxLook::next_of)",
    ),
    (
        "",
        "checkbox.unchecked_border_color",
        "gpui: checkbox.indicator(NONE).stroke.color (radio.indicator(NONE)), the unchecked indicator's border (CheckboxLook::next_of)",
    ),
    // --- combo_box: the Combobox of the side panel ------------------------------
    (
        "chrome.side_panel.settings.theme",
        "combo_box.font",
        "gpui: geometry::combobox's family, size, weight and style",
    ),
    (
        "chrome.side_panel.settings.theme",
        "combo_box.font.color",
        "not reachable: the Combobox's disabled colour lands before the caller's style (gpui-component/combobox.rs:1035-1042, refine_style)",
    ),
    // --- combo_box: the Basic page's Select, on the 0.8.0 builder --------------
    (
        "basic.drop_down",
        "combo_box.border.corner_radius_px",
        "gpui: next::geometry::select's radius, and the surface's (demo::combo_surface)",
    ),
    (
        "basic.drop_down",
        "combo_box.border.padding_top_px",
        "gpui: next::geometry::select pads the stated side",
    ),
    (
        "basic.drop_down",
        "combo_box.border.padding_right_px",
        "gpui: next::geometry::select pads it, unless an arrow column is stated",
    ),
    (
        "basic.drop_down",
        "combo_box.border.padding_bottom_px",
        "gpui: next::geometry::select pads the stated side",
    ),
    (
        "basic.drop_down",
        "combo_box.border.padding_left_px",
        "gpui: next::geometry::select pads the stated side",
    ),
    (
        "basic.drop_down",
        "combo_box.font",
        "gpui: next::geometry::select's family, size, weight and style",
    ),
    (
        "basic.drop_down",
        "combo_box.font.color",
        "gpui: next::geometry::select's text colour",
    ),
    (
        "basic.drop_down",
        "combo_box.min_height_px",
        "gpui: next::geometry::select's height, by the control-height rule",
    ),
    (
        "basic.drop_down",
        "combo_box.min_width_px",
        "gpui: next::geometry::select's min_w",
    ),
    // --- combo_box: the chrome's Selects, and the Combobox's metrics -----------
    (
        "",
        "combo_box.arrow_area_width_px",
        "gpui: where stated, the trigger keeps upstream's right padding: it has no arrow column",
    ),
    (
        "",
        "combo_box.arrow_icon_size_px",
        "not reachable: the caret is the trigger's own, sized inside it (gpui-component/select.rs:642-643, Caret)",
    ),
    (
        "",
        "combo_box.background_color",
        "gpui: next::geometry::combo_box_fill's fill, under the trigger (demo::combo_surface)",
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
        "gpui: geometry::select's family, size, weight and style",
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
        "basic.drop_down.trigger",
        "combo_box.open_background",
        "not reachable: an open Select draws ring and exposes no open state to style its fill by (gpui-component 0.10.0 select.rs:559, outline_visible; gpui-component 0.10.0 select.rs:596-598, ring)",
    ),
    (
        "basic.drop_down.trigger",
        "combo_box.open_border_color",
        "not reachable: an open Select edges itself in ring and exposes no open state to style (gpui-component 0.10.0 select.rs:559, outline_visible; gpui-component 0.10.0 select.rs:596-598, ring)",
    ),
    (
        "",
        "combo_box.hover_border_color",
        "gpui: an edge drawn under the trigger, which takes the place of the trigger's under the pointer (demo::combo_surface); upstream's trigger has no hover style",
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
        "chrome.menu.",
        "defaults.border.line_width_px",
        "not applied: the highlighted row's outline is the stroke a rule states for menu.item(HOVER), at that stroke's own width (MenuLook::next_of, MenuBar::popup)",
    ),
    (
        "basic.list.",
        "defaults.border.line_width_px",
        "gpui: the width of an unselected row's outline under the pointer in list.hover_border_color (demo::ListRow)",
    ),
    (
        "basic.table",
        "defaults.border.line_width_px",
        "gpui: the width of an unselected row's outline under the pointer in list.hover_border_color (demo::files_table)",
    ),
    (
        "",
        "defaults.border.line_width_px",
        "gpui: the swatch's frame width (inspector.rs)",
    ),
    (
        "",
        "defaults.disabled_text_color",
        "gpui: not read by widgets::Switch, whose disabled label is switch.label(DISABLED).color where a rule states it, else gpui-component's own muted_foreground, colors.window.foreground_muted (SwitchLook::next_of)",
    ),
    (
        "",
        "defaults.focus_ring_color",
        "gpui: the ring the theme-drawn Checkbox, Radio, Slider and Switch draw, as stated (widgets/mod.rs, FocusRing::of); and ThemeColor ring (colors.rs, ring_colour: opaque where the band is off): a focused Input's edge and ring, which focus_ring_style hands to focus_style (gpui-component/styled.rs:179-184, focus_ring_style; :250-298, focus_style). gpui-component's widgets draw that ring 3px wide with its alpha set to 0.5, whatever the colour's own (gpui-component/styled.rs:12, FOCUS_RING_OPACITY; :250-298, focus_style), and the Tab draws another, 2px inset at the full colour, on keyboard focus only (gpui-component/tab/tab.rs:933, ring; :962-973, focus_visible)",
    ),
    (
        "",
        "defaults.focus_ring_offset_px",
        "gpui: the theme-drawn Checkbox, Radio, Slider and Switch's ring, offset + width outside the control's edge, the control's border tinted where the ring lies inset at it (widgets/mod.rs, FocusRing::draw); upstream's own widgets' ring lies against the border, no offset (styled.rs, focus_ring)",
    ),
    (
        "",
        "defaults.focus_ring_keyboard_only",
        "gpui: when gpui-component's band is drawn (Theme::focus_ring off where true, native-theme-gpui/colors.rs, focus_band_on; on there for a focus that followed a key press in a window under Root, native-theme-gpui/lib.rs, FocusBand), and whether the theme-drawn Checkbox, Radio, Slider and Switch (widgets/mod.rs, ring_shows) and the TabBar's focused tab (widgets/mod.rs, focus_followed_key) ring a focus that did not follow a key",
    ),
    (
        "",
        "defaults.focus_ring_width_px",
        "gpui: the theme-drawn Checkbox, Radio, Slider and Switch's ring, this wide, none where it is not positive (widgets/mod.rs, FocusRing::of); upstream's own widgets' ring is FOCUS_RING_WIDTH, 3px (gpui-component/styled.rs:11, FOCUS_RING_WIDTH), and a width > 0 turns it on where the theme does not say keyboard only (native-theme-gpui/colors.rs, focus_band_on)",
    ),
    (
        "basic.typography.body",
        "defaults.font",
        "gpui: size, weight and family on the Label (demo::type_line)",
    ),
    (
        "basic.switches",
        "defaults.font",
        "gpui: not read by widgets::Switch, which letters its label in switch.label(NONE)'s family, size, weight and slant where stated, else gpui-component's own, text_sm in the window's font (SwitchLabelFont::next_of)",
    ),
    (
        "",
        "defaults.font",
        "gpui: the window's font, Theme::font_family and font_size (config.rs; gpui-component/root.rs:725, font_size; :735, font_family)",
    ),
    (
        "basic.typography",
        "defaults.font.color",
        "gpui: the Label's text colour (demo::type_line)",
    ),
    (
        "basic.switches",
        "defaults.font.color",
        "gpui: widgets::Switch's label where the theme states no switch.label(NONE).color: gpui-component's own, the window's text, colors.window.foreground (SwitchLook::next_of)",
    ),
    (
        "",
        "defaults.font.color",
        "gpui: the Label paints `foreground`, defaults.text_color (gpui-component/label.rs:248, foreground)",
    ),
    (
        "basic.typography",
        "defaults.font.family",
        "gpui: font_family on the Label (demo::type_line)",
    ),
    (
        "",
        "defaults.font.family",
        "gpui: the window's font, Theme::font_family (config.rs), set by gpui-component's WindowState root plugin (gpui-component/root.rs:735, font_family)",
    ),
    (
        "basic.icons.large",
        "defaults.icon_sizes.large_px",
        "gpui: geometry::icon_size_large sizes the icon (demo::sized_icon)",
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
        "gpui: next::geometry::icon_size_toolbar, toolbar.button_icon(NONE).size",
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
        "gpui: widgets::Switch's label, one line of its font's size by it",
    ),
    (
        "basic.text_inputs",
        "defaults.line_height",
        "gpui: next::geometry::input's line height (control-height rule)",
    ),
    (
        "basic.number_input",
        "defaults.line_height",
        "gpui: next::geometry::input's line height (control-height rule)",
    ),
    (
        "basic.text_area",
        "defaults.line_height",
        "gpui: next::geometry::text_area's line height",
    ),
    (
        "basic.drop_down",
        "defaults.line_height",
        "gpui: next::geometry::select's line height (control-height rule)",
    ),
    (
        "chrome.side_panel.settings.theme_label",
        "defaults.line_height",
        "gpui: the label's line box, the installed 0.8.0 theme's sidebar.item_label(NONE) line_height where stated, else its size by typography.line_height_multiplier (demo::next_text_in)",
    ),
    (
        "chrome.side_panel.settings.mode_label",
        "defaults.line_height",
        "gpui: the label's line box, the installed 0.8.0 theme's sidebar.item_label(NONE) line_height where stated, else its size by typography.line_height_multiplier (demo::next_text_in)",
    ),
    (
        "chrome.side_panel.settings.icon_theme_label",
        "defaults.line_height",
        "gpui: the label's line box, the installed 0.8.0 theme's sidebar.item_label(NONE) line_height where stated, else its size by typography.line_height_multiplier (demo::next_text_in)",
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
        "gpui: widgets::TabBar's label, tab.font's size by it",
    ),
    (
        "chrome.page_tabs",
        "defaults.line_height",
        "gpui: widgets::TabBar's label, tab.font's size by it",
    ),
    (
        "basic.tabs",
        "defaults.line_height",
        "gpui: widgets::TabBar's label, tab.font's size by it",
    ),
    (
        "basic.segmented",
        "defaults.line_height",
        "gpui: a segment's line box, segmented_control.font's size by it (demo::text_in)",
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
        "gpui: the title's line box, expander.font's size by it (demo::text_in), and the body Label's (demo::body_label)",
    ),
    (
        "basic.card",
        "defaults.line_height",
        "gpui: the Label's line height (demo::body_label)",
    ),
    (
        "chrome.menu.theme.preferences.shortcut",
        "defaults.line_height",
        "gpui: typography.line_height_multiplier, the row's line height as next::geometry::menu_item sets it, which the plain Kbd text inherits (MenuLook::next_of; gpui-component/kbd.rs:236-238, appearance)",
    ),
    (
        "chrome.menu.",
        "defaults.line_height",
        "gpui: typography.line_height_multiplier, the row's line height as next::geometry::menu_item sets it (MenuLook::next_of)",
    ),
    (
        "chrome.menu_bar.",
        "defaults.line_height",
        "gpui: typography.line_height_multiplier, the title's line height as next::geometry::menu_item sets it (MenuLook::title)",
    ),
    (
        "chrome.status_bar",
        "defaults.line_height",
        "gpui: next::geometry::status_bar's line height, status_bar.label's size by it, which the bar's text takes (demo::status_bar)",
    ),
    (
        "chrome.info",
        "defaults.line_height",
        "gpui: the line box of every Widget Info line (inspector.rs)",
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
        "gpui: the title's family, size, weight and style (demo::native_expander)",
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
        "",
        "input.background_color",
        "gpui: the field's fill (demo::input_surface, next::geometry::input_fill)",
    ),
    (
        "basic.number_input",
        "input.border.color",
        "gpui: the frame's edge (demo::number_input)",
    ),
    (
        "",
        "input.border.color",
        "gpui: the surface's edge (demo::input_surface); a disabled field's, the line of its faded surface (demo::faded_surface)",
    ),
    (
        "",
        "input.border.corner_radius_px",
        "gpui: next::geometry::input's corner radius",
    ),
    (
        "",
        "input.border.line_width_px",
        "gpui: next::geometry::input's border width",
    ),
    (
        "basic.number_input",
        "input.border.padding_top_px",
        "gpui: pads the value's Input left of the step buttons (demo::number_input)",
    ),
    (
        "basic.number_input",
        "input.border.padding_right_px",
        "gpui: pads the value's Input left of the step buttons (demo::number_input)",
    ),
    (
        "basic.number_input",
        "input.border.padding_bottom_px",
        "gpui: pads the value's Input left of the step buttons (demo::number_input)",
    ),
    (
        "basic.number_input",
        "input.border.padding_left_px",
        "gpui: pads the value's Input left of the step buttons (demo::number_input)",
    ),
    (
        "",
        "input.border.padding_top_px",
        "gpui: next::geometry::input pads the stated side",
    ),
    (
        "",
        "input.border.padding_right_px",
        "gpui: next::geometry::input pads the stated side",
    ),
    (
        "",
        "input.border.padding_bottom_px",
        "gpui: next::geometry::input pads the stated side",
    ),
    (
        "",
        "input.border.padding_left_px",
        "gpui: next::geometry::input pads the stated side",
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
        "input.caret_blink_on_ms",
        "gpui: Theme::cursor_blink, which apply installs (lib.rs, caret_blink): each phase half of caret_blink_on and caret_blink_off together, as gpui-base blinks in equal phases (gpui-base/input/base/blink_cursor.rs, CursorBlink::blinking); upstream's 500ms where the theme states no blink",
    ),
    (
        "",
        "input.caret_blink_off_ms",
        "gpui: Theme::cursor_blink: steady where it is 0 (CursorBlink::steady), else its half of the phase; upstream's 500ms where the theme states no blink",
    ),
    (
        "",
        "input.caret_blink_timeout_ms",
        "gpui: Theme::cursor_blink's idle time-out (CursorBlink::with_idle_timeout): the caret stops blinking, shown, that long after the last focus, edit or movement; it never stops where unstated",
    ),
    (
        "",
        "input.disabled_background",
        "gpui: next::geometry::input_fill's fill for a disabled field",
    ),
    (
        "",
        "input.disabled_opacity",
        "gpui: next::geometry::input_fill fades a disabled field by it",
    ),
    (
        "",
        "input.disabled_text_color",
        "gpui: next::geometry::input_fill's text colour for a disabled field, which gpui-base draws at half its alpha (gpui-base/input/base/element.rs:3222, opacity)",
    ),
    (
        "",
        "input.focus_border_color",
        "not reachable: a focused Input edges itself in `ring` after the caller's style (gpui-component/input/input.rs:789-792, ring)",
    ),
    (
        "basic.text_area",
        "input.font",
        "gpui: next::geometry::text_area's family, size, weight and style",
    ),
    (
        "",
        "input.font",
        "gpui: next::geometry::input's family, size, weight and style",
    ),
    (
        "",
        "input.font.color",
        "gpui: next::geometry::input's text colour, over the window's `foreground` the field's text inherits (gpui-component/input/input.rs:729, input_style)",
    ),
    (
        "basic.number_input",
        "input.hover_border_color",
        "gpui: the frame's edge under the pointer (demo::number_input)",
    ),
    (
        "",
        "input.hover_border_color",
        "gpui: the surface's edge under the pointer (demo::input_surface)",
    ),
    (
        "",
        "input.min_height_px",
        "gpui: next::geometry::input's height, by the control-height rule",
    ),
    (
        "",
        "input.placeholder_color",
        "not reachable: the placeholder is muted_foreground (gpui-component/input/input.rs:579, muted_foreground)",
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
        "gpui: the tab bar's side inset where sidebar.container(NONE) states no padding side (none where that is unstated); where it states any, the side panel's padding insets it (demo::native_tab_bar)",
    ),
    (
        "chrome.page_tabs",
        "layout.container_margin_px",
        "gpui: the tab bar's side inset (none where that is unstated) (demo::native_tab_bar)",
    ),
    (
        "chrome.side_panel.inspector",
        "layout.container_margin_px",
        "gpui: pads the inspector's content where sidebar.container(NONE) states no padding side (none where that is unstated); where it states any, the side panel's padding pads it (inspector.rs)",
    ),
    (
        "chrome.status_bar",
        "layout.container_margin_px",
        "gpui: not applied — an unstated side keeps StatusBar's px_2 py_1 (gpui-component/status_bar.rs:98-99, px_2), the padding's",
    ),
    (
        "basic.card",
        "layout.container_margin_px",
        "gpui: pads a side card.border.padding leaves unstated (demo::card)",
    ),
    (
        "",
        "layout.container_margin_px",
        "gpui: the margin (none where that is unstated) pads the theme settings where sidebar.container(NONE) states no padding side, and where it states any, a side of the side panel the padding leaves unstated (demo::side_panel)",
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
        "gpui: the gap between the side-panel toggle and the environment text, one item of StatusBar's middle region (demo::status_bar)",
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
        "not reachable: a pressed Link is `link` at 0.6 opacity (gpui-component/link.rs:84-85, active)",
    ),
    (
        "",
        "link.background_color",
        "gpui: next::geometry::link's fill, link.container(NONE).fill, where the Link paints none (gpui-component/link.rs:70-90, render)",
    ),
    (
        "",
        "link.font",
        "gpui: next::geometry::link's family, size, weight and style, link.label(NONE)'s",
    ),
    (
        "",
        "link.font.color",
        "gpui: ThemeColor link (colors.rs): the Link's text (gpui-component/link.rs:76, link)",
    ),
    (
        "",
        "link.hover_background",
        "not reachable: the Link's hover style is upstream's, no fill (gpui-component/link.rs:79-83, hover)",
    ),
    (
        "",
        "link.hover_text_color",
        "not reachable: a hovered Link is `link` at 0.8 opacity (gpui-component/link.rs:79-80, hover)",
    ),
    (
        "",
        "link.underline_enabled",
        "gpui: next::geometry::link removes upstream's underline where link.label(NONE).decoration is none",
    ),
    // --- list: the Basic page's table, demo::files_table ------------------------
    (
        "basic.table",
        "list.header_height_px",
        "gpui: geometry::list_header_height, the header row's height where stated, else a row's (demo::files_table)",
    ),
    (
        "basic.table.header",
        "list.row_height_px",
        "gpui: the header row's height where list.header_height is unstated and this is stated, else padding and line (demo::files_table)",
    ),
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
        "basic.table.header",
        "list.border.corner_radius_px",
        "gpui: the header's fill takes the frame's inner radius, this less list.border.line_width, on its top corners (demo::files_table)",
    ),
    (
        "basic.table.header",
        "list.border.line_width_px",
        "gpui: the frame's line, which the header lies inside; taken off the frame's radius for the inner radius of the fill's top corners (demo::files_table)",
    ),
    (
        "basic.table.row_",
        "list.border.corner_radius_px",
        "gpui: the last row's fill takes the frame's inner radius, this less list.border.line_width, on its bottom corners (demo::files_table)",
    ),
    (
        "basic.table.row_",
        "list.border.line_width_px",
        "gpui: the frame's line, which the row lies inside; taken off the frame's radius for the inner radius of the last row's bottom corners (demo::files_table)",
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
        "list.hover_border_color",
        "gpui: an unselected row's outline under the pointer, a box over the row, square but on the last row's bottom corners (demo::files_table)",
    ),
    (
        "basic.table",
        "list.item_font",
        "gpui: each row's family, size, weight and style (demo::files_table)",
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
        "basic.table.",
        "list.header_hover_background",
        "gpui: demo::files_table's header cells, .hover(..) bg where stated: gpui-component's own table header has no hover style (gpui-component/table/state.rs:1932-1975, render_th)",
    ),
    (
        "basic.table.",
        "list.header_active_background",
        "gpui: demo::files_table's header cells, .active(..) bg where stated, else the hovered fill: gpui-component's own table header has no press style (gpui-component/table/state.rs:1932-1975, render_th)",
    ),
    (
        "basic.table",
        "list.header_separator_color",
        "gpui: the header's line on the first section's trailing edge and along its bottom, laid over the header's and a section's fill (demo::files_table)",
    ),
    (
        "basic.table.",
        "defaults.selection_inactive_background",
        "gpui: demo::files_table's selected row while the window is not active and the theme's inactive fill differs from its selection (selection_pair)",
    ),
    (
        "basic.table.",
        "defaults.selection_inactive_text_color",
        "gpui: demo::files_table's selected row while the window is not active and the theme's inactive fill differs from its selection (selection_pair)",
    ),
    (
        "",
        "list.grid_color",
        "gpui: the column line, and the header's lines where list.header_separator_color is unstated (demo::files_table)",
    ),
    (
        "",
        "list.header_background",
        "gpui: the header row's fill (demo::files_table)",
    ),
    (
        "",
        "list.header_font",
        "gpui: the header row's family, size, weight and style (demo::files_table)",
    ),
    (
        "",
        "list.header_font.color",
        "gpui: the header row's text (demo::files_table)",
    ),
    // --- list: the List, demo::list and demo::ListRow ---------------------------
    (
        "basic.list.",
        "defaults.selection_inactive_background",
        "not reachable: a ListItem paints its selected fill list_active after the caller's style, whatever the window's state (gpui-component/list/list_item.rs:260-267, list_active), and a text colour without its fill is no pair",
    ),
    (
        "basic.list.",
        "defaults.selection_inactive_text_color",
        "not reachable: a ListItem paints its selected fill list_active after the caller's style, whatever the window's state (gpui-component/list/list_item.rs:260-267, list_active), and a text colour without its fill is no pair",
    ),
    (
        "basic.list.",
        "list.hover_border_color",
        "gpui: list.highlight(HOVER).stroke, an unselected row's outline under the pointer, a box over the row in its suffix, on the corners of its fill (demo::ListRow)",
    ),
    (
        "basic.list.row_",
        "list.border.corner_radius_px",
        "gpui: list.container(NONE).corners, the first row's fill takes the frame's inner radius, this less list.container(NONE).stroke.width, on its top corners and the last row's on its bottom corners (demo::ListRow)",
    ),
    (
        "basic.list.row_",
        "list.border.line_width_px",
        "gpui: list.container(NONE).stroke.width, next::geometry::list's line, which the rows lie inside; taken off the frame's radius for the inner radius of the first and last rows' corners (demo::ListRow)",
    ),
    (
        "",
        "list.background_color",
        "gpui: list.container(NONE).fill, next::geometry::list's fill",
    ),
    (
        "",
        "list.border.color",
        "gpui: list.container(NONE).stroke.color, next::geometry::list's border colour",
    ),
    (
        "",
        "list.border.corner_radius_px",
        "gpui: list.container(NONE).corners, next::geometry::list's corner radius",
    ),
    (
        "",
        "list.border.line_width_px",
        "gpui: list.container(NONE).stroke.width, next::geometry::list's border width",
    ),
    (
        "",
        "list.border.padding_top_px",
        "gpui: list.row(NONE).padding, with list.highlight(NONE)'s margin, next::geometry::list_item pads each row by the stated side",
    ),
    (
        "",
        "list.border.padding_right_px",
        "gpui: list.row(NONE).padding, with list.highlight(NONE)'s margin, next::geometry::list_item pads each row by the stated side",
    ),
    (
        "",
        "list.border.padding_bottom_px",
        "gpui: list.row(NONE).padding, with list.highlight(NONE)'s margin, next::geometry::list_item pads each row by the stated side",
    ),
    (
        "",
        "list.border.padding_left_px",
        "gpui: list.row(NONE).padding, with list.highlight(NONE)'s margin, next::geometry::list_item pads each row by the stated side",
    ),
    (
        "",
        "list.hover_background",
        "gpui: ThemeColor list_hover (colors.rs): a ListItem's hover fill",
    ),
    (
        "",
        "list.hover_text_color",
        "gpui: list.item_label(HOVER).color, the label's colour under the pointer (demo::ListRow)",
    ),
    (
        "",
        "list.item_font",
        "gpui: list.item_label(NONE), next::geometry::list_item's family, size, weight and style",
    ),
    (
        "",
        "list.item_font.color",
        "gpui: list.item_label(NONE).color, next::geometry::list_item's text colour",
    ),
    (
        "",
        "list.row_height_px",
        "gpui: list.row(NONE).min_size[1], next::geometry::list_item's height where stated (control-height rule)",
    ),
    (
        "",
        "list.selection_background",
        "gpui: ThemeColor list_active (colors.rs): the selected ListItem's fill",
    ),
    (
        "",
        "list.selection_text_color",
        "gpui: list.item_label(SELECTED).color, next::geometry::list_item_selected's text colour on the selected row (demo::ListRow)",
    ),
    // --- menu: the showcase's MenuBar (demo::MenuTitle, MenuBar::popup) ---------
    (
        "chrome.menu.theme.preferences.shortcut",
        "menu.font",
        "gpui: menu.item_label(NONE), the row's font as next::geometry::menu_item sets it, which the plain Kbd text inherits (MenuLook::next_of; gpui-component/kbd.rs:236-238, appearance)",
    ),
    (
        "chrome.menu_bar",
        "toolbar.background_color",
        "gpui: menu_bar.bar(NONE).fill, the menu bar's fill (demo::menu_bar)",
    ),
    (
        "chrome.menu_bar.",
        "menu.border.padding_top_px",
        "gpui: menu.item(NONE)'s padding side pads the title where menu_bar.item(NONE) states none, else MENU_TITLE_PADDING_Y (MenuLook::title, demo::MenuTitle)",
    ),
    (
        "chrome.menu_bar.",
        "menu.border.padding_right_px",
        "gpui: menu.item(NONE)'s padding side pads the title where menu_bar.item(NONE) states none, else MENU_TITLE_PADDING_X (MenuLook::title, demo::MenuTitle)",
    ),
    (
        "chrome.menu_bar.",
        "menu.border.padding_bottom_px",
        "gpui: menu.item(NONE)'s padding side pads the title where menu_bar.item(NONE) states none, else MENU_TITLE_PADDING_Y (MenuLook::title, demo::MenuTitle)",
    ),
    (
        "chrome.menu_bar.",
        "menu.border.padding_left_px",
        "gpui: menu.item(NONE)'s padding side pads the title where menu_bar.item(NONE) states none, else MENU_TITLE_PADDING_X (MenuLook::title, demo::MenuTitle)",
    ),
    (
        "chrome.menu_bar.",
        "menu.bar_padding_horizontal_px",
        "gpui: menu_bar.item(NONE)'s left and right padding pads the title, over menu.item(NONE)'s (MenuLook::title)",
    ),
    (
        "chrome.menu_bar.",
        "menu.bar_padding_vertical_px",
        "gpui: menu_bar.item(NONE)'s top and bottom padding pads the title, over menu.item(NONE)'s (MenuLook::title)",
    ),
    (
        "chrome.menu_bar.",
        "menu.hover_background",
        "gpui: menu.item(HOVER).fill, the title's fill under the pointer where no rule states menu_bar.item(HOVER).fill, and while open where none states menu_bar.item(OPEN).fill either (MenuLook::next_of, demo::MenuTitle)",
    ),
    (
        "chrome.menu_bar.",
        "menu.hover_text_color",
        "gpui: menu.item_label(HOVER).color, the title's text under the pointer where no rule states menu_bar.item_label(HOVER).color, and while open where none states menu_bar.item_label(OPEN).color either (MenuLook::next_of, demo::MenuTitle)",
    ),
    (
        "chrome.menu_bar.",
        "menu.bar_hover_background",
        "gpui: menu_bar.item(HOVER).fill where a rule states it, the title's fill under the pointer, and while open where none states menu_bar.item(OPEN).fill (MenuLook::next_of, MenuLook::title)",
    ),
    (
        "chrome.menu_bar.",
        "menu.bar_open_background",
        "gpui: menu_bar.item(OPEN).fill where a rule states it, the title's fill while its menu is open, under the pointer too (MenuLook::next_of, MenuLook::title)",
    ),
    (
        "chrome.menu_bar.",
        "menu.bar_hover_text_color",
        "gpui: menu_bar.item_label(HOVER).color where a rule states it, the title's text under the pointer, and while open where none states menu_bar.item_label(OPEN).color (MenuLook::next_of, MenuLook::title)",
    ),
    (
        "chrome.menu_bar.",
        "menu.border.corner_radius_px",
        "gpui: menu.item(NONE).corners rounds the title's fill where stated (MenuLook::title)",
    ),
    (
        "chrome.menu.",
        "menu.border.corner_radius_px",
        "gpui: menu.item(NONE).corners rounds the highlighted row's fill and its outline where stated (MenuLook::next_of)",
    ),
    (
        "chrome.menu.",
        "menu.hover_border_color",
        "gpui: menu.item(HOVER).stroke.color where a rule states it, the highlighted row's outline, a box over the row and its padding, so nothing moves (MenuLook::next_of, MenuBar::popup)",
    ),
    (
        "chrome.menu.theme",
        "menu.panel_padding_horizontal_px",
        "gpui: menu.panel(NONE)'s left and right padding insets the rows inside the popup's frame line, over popover.panel's (MenuLook::next_of, MenuBar::popup)",
    ),
    (
        "chrome.menu.theme",
        "menu.panel_padding_vertical_px",
        "gpui: menu.panel(NONE)'s top and bottom padding insets the rows inside the popup's frame line, over popover.panel's, and leaves no gap between them (MenuLook::next_of, MenuBar::popup)",
    ),
    (
        "chrome.menu.",
        "menu.min_row_height_px",
        "gpui: menu.item(NONE).min_size[1], the row's least height where menu.item(NONE).max_size[1] is unstated, the content growing past it (next::geometry::menu_item)",
    ),
    (
        "chrome.menu.",
        "menu.icon_text_gap_px",
        "not reachable: the Theme menu's items show no icon, and the popup spaces an item's label and its shortcut by MENU_SHORTCUT_GAP instead (MenuBar::popup)",
    ),
    (
        "chrome.menu.",
        "menu.active_background",
        "gpui: menu.item(PRESSED).fill where a rule states it for the pressed state, the row's fill while the mouse button is held on it (MenuLook::next_of, MenuLook::pressed)",
    ),
    (
        "chrome.menu.",
        "menu.active_text_color",
        "gpui: menu.item_label(PRESSED).color where a rule states it for the pressed state, the row's text while the mouse button is held on it, its shortcut's too (MenuLook::next_of, MenuLook::pressed)",
    ),
    (
        "",
        "menu.background_color",
        "gpui: menu.panel(NONE).fill, the popup's fill (MenuLook::next_of, MenuBar::popup)",
    ),
    (
        "",
        "menu.border.padding_top_px",
        "gpui: menu.item(NONE)'s padding side pads the row where stated (next::geometry::menu_item, MenuBar::popup)",
    ),
    (
        "",
        "menu.border.padding_right_px",
        "gpui: menu.item(NONE)'s padding side pads the row where stated (next::geometry::menu_item), else MENU_ROW_PADDING_X (MenuLook::row, MenuBar::popup)",
    ),
    (
        "",
        "menu.border.padding_bottom_px",
        "gpui: menu.item(NONE)'s padding side pads the row where stated (next::geometry::menu_item, MenuBar::popup)",
    ),
    (
        "",
        "menu.border.padding_left_px",
        "gpui: menu.item(NONE)'s padding side pads the row where stated (next::geometry::menu_item), else MENU_ROW_PADDING_X (MenuLook::row, MenuBar::popup)",
    ),
    (
        "",
        "menu.font",
        "gpui: menu.item_label(NONE)'s family, size, weight and style as next::geometry::menu_item sets them (demo::MenuTitle, MenuBar::popup)",
    ),
    (
        "",
        "menu.font.color",
        "gpui: menu.item_label(NONE).color, the text colour from MenuLook::next_of (demo::MenuTitle, MenuBar::popup)",
    ),
    (
        "",
        "menu.hover_background",
        "gpui: menu.item(HOVER).fill, the highlighted row's fill (MenuLook::next_of, MenuBar::popup)",
    ),
    (
        "",
        "menu.hover_text_color",
        "gpui: menu.item_label(HOVER).color, the highlighted row's text (MenuLook::next_of, MenuBar::popup)",
    ),
    (
        "",
        "menu.row_height_px",
        "gpui: menu.item(NONE).max_size[1], the row's height by next::geometry::menu_item's control-height rule where stated: the height at a text scale of 1 or less, a minimum above (MenuBar::popup)",
    ),
    (
        "",
        "menu.separator_color",
        "gpui: menu.separator(NONE).fill, the separator row's fill (MenuLook::next_of)",
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
        "gpui: progress_bar.track(NONE).stroke.color, the track's edge (ProgressBarLook::next_of)",
    ),
    (
        "",
        "progress_bar.border.corner_radius_px",
        "gpui: progress_bar.track(NONE).corners, the track's and the fill's corner radius (ProgressBarLook::next_of)",
    ),
    (
        "",
        "progress_bar.border.line_width_px",
        "gpui: progress_bar.track(NONE).stroke.width, the track's border width (ProgressBarLook::next_of)",
    ),
    (
        "",
        "progress_bar.fill_color",
        "gpui: progress_bar.fill(NONE).fill, widgets::ProgressBar's fill (ProgressBarLook::next_of)",
    ),
    (
        "",
        "progress_bar.min_width_px",
        "gpui: progress_bar.track(NONE).min_size[0], the bar's min_w (ProgressBarLook::next_of)",
    ),
    (
        "",
        "progress_bar.track_color",
        "gpui: progress_bar.track(NONE).fill, the track's fill (ProgressBarLook::next_of)",
    ),
    (
        "",
        "progress_bar.track_height_px",
        "gpui: progress_bar.track(NONE).min_size[1], the bar's height (ProgressBarLook::next_of)",
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
        "gpui: each label's family, size, weight and style (demo::segmented)",
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
        "gpui: the page row's rule, a widgets::Separator laid under the tab strip, its line's colour (demo::native_tab_bar)",
    ),
    (
        "basic.table",
        "separator.line_width_px",
        "gpui: the width of the header's bottom line and the column line, in list.grid_color, the header's in list.header_separator_color where stated (demo::files_table)",
    ),
    (
        "chrome.page_tabs.rule",
        "separator.line_width_px",
        "gpui: the page row's rule, a widgets::Separator laid under the tab strip, its line's thickness, one device pixel at least (demo::native_tab_bar)",
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
        "gpui: sidebar.container(NONE).fill, ThemeColor sidebar (next/mod.rs): the side panel's fill (demo::side_panel)",
    ),
    (
        "",
        "sidebar.border.padding_top_px",
        "gpui: sidebar.container(NONE)'s padding side pads the side panel where any side is stated, a side it leaves unstated by layout.container_margin (none where that is unstated); where no side is stated, layout.container_margin pads the settings, the inspector's tab row and its content (none where that is unstated) (demo::side_panel)",
    ),
    (
        "",
        "sidebar.border.padding_right_px",
        "gpui: sidebar.container(NONE)'s padding side pads the side panel where any side is stated, a side it leaves unstated by layout.container_margin (none where that is unstated); where no side is stated, layout.container_margin pads the settings, the inspector's tab row and its content (none where that is unstated) (demo::side_panel)",
    ),
    (
        "",
        "sidebar.border.padding_bottom_px",
        "gpui: sidebar.container(NONE)'s padding side pads the side panel where any side is stated, a side it leaves unstated by layout.container_margin (none where that is unstated); where no side is stated, layout.container_margin pads the settings, the inspector's tab row and its content (none where that is unstated) (demo::side_panel)",
    ),
    (
        "",
        "sidebar.border.padding_left_px",
        "gpui: sidebar.container(NONE)'s padding side pads the side panel where any side is stated, a side it leaves unstated by layout.container_margin (none where that is unstated); where no side is stated, layout.container_margin pads the settings, the inspector's tab row and its content (none where that is unstated) (demo::side_panel)",
    ),
    (
        "chrome.info",
        "sidebar.font",
        "gpui: size and family of every Widget Info line (inspector.rs)",
    ),
    (
        "",
        "sidebar.font",
        "gpui: the label's family, size, weight and style, at sidebar.item_label(NONE) of the installed 0.8.0 theme (demo::sidebar_label)",
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
        "gpui: slider.active_track(NONE).fill (DISABLED disabled), the rail's filled stretch (SliderLook::next_of)",
    ),
    (
        "",
        "slider.thumb_color",
        "gpui: slider.thumb(NONE).fill (DISABLED disabled), the thumb's fill (SliderLook::next_of)",
    ),
    (
        "",
        "slider.thumb_diameter_px",
        "gpui: slider.thumb(NONE).min_size[0], the thumb's size (SliderLook::next_of)",
    ),
    (
        "",
        "slider.thumb_hover_color",
        "gpui: slider.thumb(HOVER).fill, over the thumb under the pointer where a rule states it; else the thumb at rest (SliderLook::next_of)",
    ),
    (
        "",
        "slider.track_color",
        "gpui: slider.track(NONE).fill (DISABLED disabled), the rail's fill (SliderLook::next_of)",
    ),
    (
        "",
        "slider.track_height_px",
        "gpui: slider.track(NONE).min_size[1], the rail's height (SliderLook::next_of)",
    ),
    (
        "basic.slider.control",
        "slider.thumb_border_color",
        "gpui: slider.thumb(NONE).stroke.color, the band inside the thumb at rest, in place of upstream's ring (ThumbOutline::next_of)",
    ),
    (
        "basic.slider.control",
        "slider.thumb_border_line_width_px",
        "gpui: slider.thumb(NONE).stroke.width, the width of the band inside the thumb; 0, or a stroke stated as none, draws none, upstream's ring included (ThumbOutline::next_of)",
    ),
    (
        "basic.slider.control",
        "slider.thumb_hover_border_color",
        "gpui: slider.thumb(HOVER).stroke.color, the band with the pointer on the thumb, over the focus colour, not while it is dragged (ThumbOutline::next_of)",
    ),
    (
        "basic.slider.control",
        "slider.thumb_focus_border_color",
        "gpui: slider.thumb(FOCUSED).stroke.color, the band on the end thumb while the slider has the focus (ThumbOutline::next_of)",
    ),
    // --- spinner: widgets::Spinner --------------------------------------------------
    (
        "",
        "spinner.diameter_px",
        "gpui: spinner.diameter, the indicator's size at Medium, and the anchor of the other sizes, gpui-component's own ratios to it (SpinnerLook::next_of_size)",
    ),
    (
        "",
        "spinner.fill_color",
        "gpui: spinner.color, the arc, and a monochrome set's indicator tint (SpinnerLook::next_of_size, widgets::Spinner)",
    ),
    (
        "",
        "spinner.min_diameter_px",
        "gpui: spinner.min_diameter, the floor under the indicator's size, at every Size (SpinnerLook::next_of_size)",
    ),
    (
        "",
        "spinner.stroke_width_px",
        "gpui: spinner.stroke_width, the arc's stroke, for a set with no indicator (SpinnerLook::next_of_size)",
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
        "gpui: ThemeColor status_bar (colors.rs): the bar's fill (gpui-component/status_bar.rs:102, status_bar)",
    ),
    (
        "",
        "status_bar.border.color",
        "gpui: ThemeColor status_bar_border (colors.rs): the top edge (gpui-component/status_bar.rs:101, status_bar_border)",
    ),
    (
        "",
        "status_bar.border.line_width_px",
        "gpui: the top edge's width (demo::status_bar)",
    ),
    (
        "",
        "status_bar.border.padding_top_px",
        "gpui: next::geometry::status_barpads the stated side; upstream's py_1 the rest",
    ),
    (
        "",
        "status_bar.border.padding_right_px",
        "gpui: next::geometry::status_barpads the stated side; upstream's px_2 the rest",
    ),
    (
        "",
        "status_bar.border.padding_bottom_px",
        "gpui: next::geometry::status_barpads the stated side; upstream's py_1 the rest",
    ),
    (
        "",
        "status_bar.border.padding_left_px",
        "gpui: next::geometry::status_barpads the stated side; upstream's px_2 the rest",
    ),
    (
        "",
        "status_bar.font",
        "gpui: next::geometry::status_bar'sfamily, size, weight and style",
    ),
    (
        "",
        "status_bar.font.color",
        "gpui: next::geometry::status_bar'stext colour",
    ),
    // --- switch: widgets::Switch ---------------------------------------------------
    (
        "",
        "switch.checked_background",
        "gpui: switch.track(CHECKED).fill, the track's fill when on (SwitchLook::next_of)",
    ),
    (
        "",
        "switch.disabled_checked_background",
        "gpui: switch.track(DISABLED|CHECKED).fill, a disabled switch's track when on (SwitchLook::next_of)",
    ),
    (
        "",
        "switch.checked_track_border_color",
        "gpui: switch.track(CHECKED).stroke.color, the on track's outline, switch.track(NONE).stroke.width wide, where stated (SwitchLook::next_of)",
    ),
    (
        "",
        "switch.disabled_checked_track_border_color",
        "gpui: switch.track(DISABLED|CHECKED).stroke.color, a disabled on track's outline, where stated (SwitchLook::next_of)",
    ),
    (
        "",
        "switch.disabled_opacity",
        "gpui: switch.disabled_opacity, fades the whole disabled switch (SwitchLook::next_of, SwitchLook::faded_over)",
    ),
    (
        "",
        "switch.disabled_thumb_color",
        "gpui: switch.thumb(DISABLED).fill (DISABLED|CHECKED on), a disabled switch's thumb (SwitchLook::next_of)",
    ),
    (
        "",
        "switch.hover_checked_background",
        "gpui: switch.track(HOVER|CHECKED).fill, over the on track under the pointer where it differs from switch.track(CHECKED).fill; else the track at rest (SwitchLook::next_of)",
    ),
    (
        "",
        "switch.hover_unchecked_background",
        "gpui: switch.track(HOVER).fill, over the off track under the pointer where a rule states it; else the track at rest (SwitchLook::next_of)",
    ),
    (
        "",
        "switch.thumb_background",
        "gpui: switch.thumb(CHECKED).fill, the on thumb's fill (SwitchLook::next_of)",
    ),
    (
        "",
        "switch.thumb_border_color",
        "gpui: switch.thumb(NONE).stroke.color (CHECKED on), the thumb's outline, switch.thumb(NONE).stroke.width wide, where stated (SwitchLook::next_of)",
    ),
    (
        "",
        "switch.thumb_border_line_width_px",
        "gpui: switch.thumb(NONE).stroke.width, the thumb outline's width, inside the thumb (SwitchLook::next_of)",
    ),
    (
        "",
        "switch.thumb_diameter_px",
        "gpui: switch.thumb(CHECKED).min_size[0], the on thumb's size, centred on the track's axis; a thumb taller than its track flush with its ends (SwitchLook::next_of)",
    ),
    (
        "",
        "switch.unchecked_thumb_background",
        "gpui: switch.thumb(NONE).fill, the off thumb's fill (SwitchLook::next_of)",
    ),
    (
        "",
        "switch.unchecked_thumb_diameter_px",
        "gpui: switch.thumb(NONE).min_size[0], the off thumb's size (SwitchLook::next_of)",
    ),
    (
        "",
        "switch.track_border_line_width_px",
        "gpui: switch.track(NONE).stroke.width, the track outline's width, inside the track (SwitchLook::next_of)",
    ),
    (
        "",
        "switch.track_height_px",
        "gpui: switch.track(NONE).min_size[1], the track's height (SwitchLook::next_of)",
    ),
    (
        "",
        "switch.track_radius_px",
        "gpui: switch.track(NONE).corners, the track's corner radius (SwitchLook::next_of)",
    ),
    (
        "",
        "switch.track_width_px",
        "gpui: switch.track(NONE).min_size[0], the track's width (SwitchLook::next_of)",
    ),
    (
        "",
        "switch.unchecked_background",
        "gpui: switch.track(NONE).fill, the track's fill when off (SwitchLook::next_of)",
    ),
    (
        "",
        "switch.unchecked_track_border_color",
        "gpui: switch.track(NONE).stroke.color, the off track's outline, switch.track(NONE).stroke.width wide, where stated (SwitchLook::next_of)",
    ),
    // --- tab: widgets::TabBar ----------------------------------------------------
    // The selected tabs before their bars, whose ids are prefixes of theirs.
    (
        "basic.tabs.one",
        "tab.active_outward_corner_radius_px",
        "gpui: the selected tab's outward corners, painted after the tabs (TabLook::outward_radius)",
    ),
    (
        "chrome.page_tabs.basic",
        "tab.active_outward_corner_radius_px",
        "gpui: the selected tab's outward corners, painted after the tabs (TabLook::outward_radius)",
    ),
    (
        "chrome.side_panel.inspector_tabs.widget",
        "tab.active_outward_corner_radius_px",
        "gpui: the selected tab's outward corners, painted after the tabs (TabLook::outward_radius)",
    ),
    (
        "basic.tabs.bar",
        "tab.active_outward_corner_radius_px",
        "gpui: the tabs' inset at the row's ends (TabLook::outward_radius)",
    ),
    (
        "chrome.page_tabs",
        "tab.active_outward_corner_radius_px",
        "gpui: the tabs' inset at the row's ends (TabLook::outward_radius)",
    ),
    (
        "chrome.side_panel.inspector_tabs",
        "tab.active_outward_corner_radius_px",
        "gpui: the tabs' inset at the row's ends (TabLook::outward_radius)",
    ),
    (
        "basic.page",
        "tab.active_outward_corner_radius_px",
        "gpui: room for both flares in the column rule (basic_column_count)",
    ),
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
        "gpui: each label's family, size, weight and style (widgets::TabBar)",
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
        "gpui: the surface's edge (demo::input_surface)",
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
        "gpui: toolbar.button_label(NONE), set on the row (next::geometry::toolbar); the button draws an icon only",
    ),
    (
        "",
        "toolbar.background_color",
        "gpui: toolbar.bar(NONE).fill, next::geometry::toolbar's fill",
    ),
    (
        "",
        "toolbar.bar_height_px",
        "gpui: toolbar.bar(NONE).min_size[1], next::geometry::toolbar's min_h where stated",
    ),
    (
        "",
        "toolbar.border.color",
        "gpui: toolbar.bar(NONE).stroke.color, next::geometry::toolbar's bottom edge colour",
    ),
    (
        "",
        "toolbar.border.line_width_px",
        "gpui: toolbar.bar(NONE).stroke.width, next::geometry::toolbar's bottom edge, inside the bar",
    ),
    (
        "",
        "toolbar.border.padding_top_px",
        "gpui: toolbar.bar(NONE)'s padding side, next::geometry::toolbar pads the stated side; layout.container_margin the rest (demo::toolbar)",
    ),
    (
        "",
        "toolbar.border.padding_right_px",
        "gpui: toolbar.bar(NONE)'s padding side, next::geometry::toolbar pads the stated side; layout.container_margin the rest (demo::toolbar)",
    ),
    (
        "",
        "toolbar.border.padding_bottom_px",
        "gpui: toolbar.bar(NONE)'s padding side, next::geometry::toolbar pads the stated side; layout.container_margin the rest (demo::toolbar)",
    ),
    (
        "",
        "toolbar.border.padding_left_px",
        "gpui: toolbar.bar(NONE)'s padding side, next::geometry::toolbar pads the stated side; layout.container_margin the rest (demo::toolbar)",
    ),
    (
        "",
        "toolbar.icon_size_px",
        "gpui: toolbar.button_icon(NONE).size, next::geometry::icon_size_toolbar sizes the icon",
    ),
    (
        "",
        "toolbar.item_gap_px",
        "gpui: toolbar.item_gap, next::geometry::toolbar's gap where stated, else layout.widget_gap (demo::toolbar)",
    ),
    // --- tooltip: demo::built_tooltip_button --------------------------------------
    (
        "",
        "tooltip.background_color",
        "gpui: next::geometry::tooltip's fill",
    ),
    (
        "",
        "tooltip.border.color",
        "gpui: next::geometry::tooltip's border colour",
    ),
    (
        "",
        "tooltip.border.corner_radius_px",
        "gpui: next::geometry::tooltip's corner radius",
    ),
    (
        "",
        "tooltip.border.line_width_px",
        "gpui: next::geometry::tooltip's border width",
    ),
    (
        "",
        "tooltip.border.padding_top_px",
        "gpui: next::geometry::tooltip pads the stated side",
    ),
    (
        "",
        "tooltip.border.padding_right_px",
        "gpui: next::geometry::tooltip pads the stated side",
    ),
    (
        "",
        "tooltip.border.padding_bottom_px",
        "gpui: next::geometry::tooltip pads the stated side",
    ),
    (
        "",
        "tooltip.border.padding_left_px",
        "gpui: next::geometry::tooltip pads the stated side",
    ),
    (
        "",
        "tooltip.border.shadow_enabled",
        "gpui: next::geometry::tooltip's shadow: shadow_md's layers in defaults.shadow_color, or none",
    ),
    (
        "",
        "tooltip.font",
        "gpui: next::geometry::tooltip's family, size, weight and style",
    ),
    (
        "",
        "tooltip.font.color",
        "gpui: next::geometry::tooltip's text colour",
    ),
    (
        "",
        "tooltip.max_width_px",
        "gpui: next::geometry::tooltip_content, less the bubble's padding and border",
    ),
    (
        "",
        "tooltip.show_delay_ms",
        "gpui: tooltip_show_delay on the element of gpui's own tooltip where stated, else gpui's 500ms (demo::built_tooltip_button)",
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
