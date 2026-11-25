//! How gpui applies each theme home an element of docs/showcase-elements.toml
//! lists (its `homes`): the muted line Widget Info shows under each
//! `<home> <value>` row (the round-11 brief, section B). A line names the
//! route the value takes -- the connector's `ThemeColor` field (`colors.rs`),
//! its `next::geometry::` builder, its `widgets::` control, or the showcase helper
//! that sets it -- or says "not reachable" with the upstream line that keeps
//! it out. Where the showcase could apply a home and does not, the line says
//! so and names what is drawn instead. Upstream citations are to
//! gpui-component 0.9.1, gpui-base 0.9.1 and gpui-pre 0.3.12, but where one
//! names its version.

/// Each line as (element-id prefix, homes, line). The first entry whose
/// prefix the hovered element's id starts with and whose homes hold the home
/// asked for wins; the prefix "" matches every element, so an element's own
/// entries come before the generic one for the same home.
pub const HOW: &[(&str, &[&str], &str)] = &[
    // --- button: the toggle button that is on --------------------------------
    (
        "basic.buttons.toggle_on",
        &["button.body(CHECKED).fill"],
        "gpui: next::geometry::button_checked's bg, replayed over the selected Button's style (demo::toggle_button)",
    ),
    (
        "basic.buttons.toggle_on",
        &["button.label(CHECKED).color"],
        "gpui: next::geometry::button_checked's text colour, over the selected Button's style (demo::toggle_button)",
    ),
    (
        "basic.buttons.toggle_on",
        &["button.body(PRESSED).fill"],
        "not drawn: where button.body(CHECKED).fill is unstated next::geometry::button_checked sets no fill and gpui-component's own selected fill, button_active, stands, not the pressed fill (spec §10, rule 0a; button/button.rs, ButtonVariant::selected; demo::toggle_button)",
    ),
    (
        "basic.buttons.toggle_on",
        &["button.label(PRESSED).color"],
        "not drawn: where button.label(CHECKED).color is unstated next::geometry::button_checked sets no text colour and gpui-component's own selected label, button_foreground, stands, not the pressed label (spec §10, rule 0a; button/button.rs, ButtonVariant::selected; demo::toggle_button)",
    ),
    (
        "basic.buttons.toggle_on",
        &[
            "button.body(HOVER).stroke.color",
            "button.body(PRESSED).stroke.color",
        ],
        "not reachable: a selected Button takes no hover or press style, so the toggle that is on draws no hover border (gpui-component/button/button.rs, RenderOnce for Button)",
    ),
    // --- button: a dialog's default button, next::variants::default_button ----------
    (
        "basic.buttons.dialog_default",
        &["button.body(DEFAULT).fill"],
        "gpui: next::geometry::default_button_rest's bg at rest, laid over the Button's own geometry refinement, and next::variants::default_button's hover fill; where unstated the Button is .primary(), ThemeColor primary",
    ),
    (
        "basic.buttons.dialog_default",
        &["button.label(DEFAULT).color"],
        "gpui: next::variants::default_button's foreground, in every state; where the theme states no button.body(DEFAULT).fill the Primary Button's button_primary_foreground",
    ),
    (
        "basic.buttons.dialog_default",
        &["button.body(DEFAULT).stroke.color"],
        "gpui: next::geometry::default_button_rest's border colour at rest, after next::geometry::button; under the pointer and pressed next::variants::default_button's color, button.body(HOVER).stroke.color where stated",
    ),
    (
        "basic.buttons.dialog_default",
        &["button.body(NONE).stroke.color"],
        "gpui: next::geometry::button's border colour at rest where the Button is .primary(); where the theme states button.body(DEFAULT).fill, the fallback of next::geometry::default_button_rest's frame and of next::variants::default_button's colour where button.body(DEFAULT).stroke.color and button.body(HOVER).stroke.color are unstated",
    ),
    (
        "basic.buttons.dialog_default",
        &["button.body(HOVER).stroke.color"],
        "gpui: next::variants::default_button's color, the border under the pointer and pressed, where the theme states button.body(DEFAULT).fill; elsewhere the Button is .primary(), whose hover and press border is the primary token (gpui-component 0.10.0 button/button.rs:1104, primary) and whose rest border is next::geometry::button's button.body(NONE).stroke.color (native-theme-gpui/geometry.rs:285, border_color)",
    ),
    (
        "basic.buttons.dialog_default",
        &["button.body(PRESSED).stroke.color"],
        "not reachable: a custom variant has one border colour for hover and press, button.body(HOVER).stroke.color's, next::variants::default_button's color (gpui-component 0.10.0 button/button.rs:1134, Custom: border_color's arm for a custom variant); elsewhere the Button is .primary(), whose press border is the primary token (gpui-component 0.10.0 button/button.rs:1104, primary)",
    ),
    (
        "basic.buttons.dialog_default",
        &["button.label(NONE).color"],
        "gpui: next::variants::default_button_states's label where the theme states button.body(DEFAULT).fill and no button.label(DEFAULT).color, the variant's foreground in every state; elsewhere the Button is .primary(), lettered in ThemeColor button_primary_foreground (gpui-component 0.10.0 button/button.rs:1055, button_primary_foreground)",
    ),
    (
        "basic.buttons.dialog_default",
        &["button.body(PRESSED).fill"],
        "gpui: next::variants::default_button's active, ThemeColor button_active (colors.rs), the pressed fill, where the theme states button.body(DEFAULT).fill; elsewhere the Button is .primary(), pressed in ThemeColor button_primary_active (gpui-component 0.10.0 button/button.rs:1271, button_primary_active), which the connector sets to primary_active, a shade of primary (colors.rs)",
    ),
    // --- button: the Basic page's buttons, next::variants::bordered_button -----------
    (
        "basic.buttons.default",
        &["button.body(HOVER).stroke.color"],
        "gpui: next::variants::bordered_button's colour, the border hovered and pressed, where the theme states one; at rest, selected and disabled the border is next::geometry::button's button.body(NONE).stroke.color, which upstream replays over those states (gpui-component/button/button.rs, RenderOnce for Button); else the Button is upstream's Default (demo::ButtonKind)",
    ),
    (
        "basic.buttons.default",
        &["button.body(PRESSED).stroke.color"],
        "not reachable: a custom variant has one border colour for hover and press, button.body(HOVER).stroke.color's, next::variants::bordered_button's colour (gpui-component 0.10.0 button/button.rs:1134, Custom: border_color's arm for a custom variant)",
    ),
    (
        "basic.buttons.tooltip",
        &["button.body(HOVER).stroke.color"],
        "gpui: next::variants::bordered_button's colour, the border hovered and pressed, where the theme states one; at rest, selected and disabled the border is next::geometry::button's button.body(NONE).stroke.color, which upstream replays over those states (gpui-component/button/button.rs, RenderOnce for Button); else the Button is upstream's Default (demo::ButtonKind)",
    ),
    (
        "basic.buttons.tooltip",
        &["button.body(PRESSED).stroke.color"],
        "not reachable: a custom variant has one border colour for hover and press, button.body(HOVER).stroke.color's, next::variants::bordered_button's colour (gpui-component 0.10.0 button/button.rs:1134, Custom: border_color's arm for a custom variant)",
    ),
    (
        "basic.buttons.toggle_off",
        &["button.body(HOVER).stroke.color"],
        "gpui: next::variants::bordered_button's colour, the border hovered and pressed, where the theme states one; at rest, selected and disabled the border is next::geometry::button's button.body(NONE).stroke.color, which upstream replays over those states (gpui-component/button/button.rs, RenderOnce for Button); else the Button is upstream's Default (demo::ButtonKind)",
    ),
    (
        "basic.buttons.toggle_off",
        &["button.body(PRESSED).stroke.color"],
        "not reachable: a custom variant has one border colour for hover and press, button.body(HOVER).stroke.color's, next::variants::bordered_button's colour (gpui-component 0.10.0 button/button.rs:1134, Custom: border_color's arm for a custom variant)",
    ),
    (
        "basic.buttons.primary",
        &[
            "button.body(HOVER).stroke.color",
            "button.body(PRESSED).stroke.color",
        ],
        "not reachable: a Primary Button's hover and press border is the primary token (gpui-component 0.10.0 button/button.rs:1104, primary); at rest it is next::geometry::button's button.body(NONE).stroke.color (native-theme-gpui/geometry.rs:285, border_color)",
    ),
    (
        "basic.buttons.primary",
        &[
            "button.body(HOVER; variant=Primary).stroke.color",
            "button.body(PRESSED; variant=Primary).stroke.color",
        ],
        "not reachable: a Primary Button's hover and press border is the primary token (gpui-component 0.10.0 button/button.rs:1104, primary)",
    ),
    (
        "basic.buttons.primary",
        &[
            "button.body(NONE; variant=Primary).stroke.color",
            "button.body(NONE; variant=Primary).corners",
            "button.body(NONE; variant=Primary).stroke.width",
        ],
        "gpui: not applied — next::geometry::button edges and rounds the Primary Button as every Button, by button.body(NONE), no variant asked (native-theme-gpui next/builders.rs, button)",
    ),
    // --- button: the Copy box of Widget Info -----------------------------------
    (
        "chrome.info.copy",
        &["button.body(HOVER).fill"],
        "gpui: the Copy box's fill under the pointer (inspector.rs)",
    ),
    (
        "chrome.info.copy",
        &["button.label(HOVER).color"],
        "gpui: the Copy box's text under the pointer (inspector.rs)",
    ),
    (
        "chrome.info.copy",
        &["button.body(PRESSED).fill"],
        "gpui: the Copy box's fill while pressed (inspector.rs)",
    ),
    (
        "chrome.info.copy",
        &["button.label(PRESSED).color"],
        "gpui: the Copy box's text while pressed (inspector.rs)",
    ),
    (
        "chrome.info.copy",
        &[
            "button.body(HOVER).stroke.color",
            "button.body(PRESSED).stroke.color",
        ],
        "not drawn: the showcase builds the Copy box as the 0.7.2 ghost, variants::ghost_button, which has no outline colour of its own and no width to draw one at, so it is edged in none under the pointer or pressed (inspector.rs; native-theme-gpui/variants.rs, ghost_button)",
    ),
    (
        "chrome.info.copy",
        &["button.body(NONE).corners"],
        "gpui: the Copy box's corner radius (inspector.rs)",
    ),
    (
        "chrome.info.copy",
        &["button.body(NONE).padding.top"],
        "gpui: pads the Copy box's text (inspector.rs)",
    ),
    (
        "chrome.info.copy",
        &["button.body(NONE).padding.right"],
        "gpui: pads the Copy box's text (inspector.rs)",
    ),
    (
        "chrome.info.copy",
        &["button.body(NONE).padding.bottom"],
        "gpui: pads the Copy box's text (inspector.rs)",
    ),
    (
        "chrome.info.copy",
        &["button.body(NONE).padding.left"],
        "gpui: pads the Copy box's text (inspector.rs)",
    ),
    // --- button: the pages menu, variants::ghost_button --------------------------
    (
        "chrome.page_tabs.menu",
        &["button.label(NONE).color"],
        "gpui: variants::ghost_button's foreground, ThemeColor secondary_foreground",
    ),
    (
        "chrome.page_tabs.menu",
        &["button.body(HOVER).fill"],
        "gpui: variants::ghost_button's hover, ThemeColor secondary_hover (colors.rs)",
    ),
    (
        "chrome.page_tabs.menu",
        &["button.label(HOVER).color"],
        "not reachable: a Custom variant has no hover text colour (gpui-component/button/button.rs:104-149, ButtonCustomVariant; :1225, Custom; :1239-1244, text_color)",
    ),
    (
        "chrome.page_tabs.menu",
        &["button.body(PRESSED).fill"],
        "gpui: variants::ghost_button's active, ThemeColor secondary_active (colors.rs)",
    ),
    (
        "chrome.page_tabs.menu",
        &["button.body(HOVER).stroke.color"],
        "gpui: next::variants::ghost_button's colour, the border under the pointer and pressed, button.body(HOVER; variant=Flat).stroke.color where a rule states it, laid out by next::geometry::ghost_button_rest and hidden at rest (demo::native_tab_bar)",
    ),
    (
        "chrome.page_tabs.menu",
        &["button.body(PRESSED).stroke.color"],
        "not reachable: a custom variant has one border colour for hover and press, button.body(HOVER; variant=Flat).stroke.color's, next::variants::ghost_button's colour (gpui-component 0.10.0 button/button.rs:1134, Custom: border_color's arm for a custom variant)",
    ),
    (
        "chrome.status_bar.toggle",
        &["button.body(HOVER).stroke.color"],
        "gpui: next::variants::ghost_button's colour, the border under the pointer and pressed while the side panel is closed, button.body(HOVER; variant=Flat).stroke.color where a rule states it, laid out by next::geometry::ghost_button_rest; while it is open the toggle is selected, and a selected Button takes no hover or press style (demo::panel_toggle)",
    ),
    (
        "chrome.status_bar.toggle",
        &["button.body(PRESSED).stroke.color"],
        "not reachable: a custom variant has one border colour for hover and press, button.body(HOVER; variant=Flat).stroke.color's, next::variants::ghost_button's colour (gpui-component 0.10.0 button/button.rs:1134, Custom: border_color's arm for a custom variant)",
    ),
    (
        "chrome.page_tabs.menu",
        &["button.body(NONE).corners"],
        "gpui: button.body(NONE).corners rounds the tool button's box where stated (demo::tool_button_box)",
    ),
    (
        "chrome.page_tabs.menu",
        &["button.body(NONE).padding.top"],
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.page_tabs.menu",
        &["button.body(NONE).padding.right"],
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.page_tabs.menu",
        &["button.body(NONE).padding.bottom"],
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.page_tabs.menu",
        &["button.body(NONE).padding.left"],
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.page_tabs.menu",
        &["icons.sizes.small"],
        "gpui: the set's ChevronDown Icon::with_size (demo::native_tab_bar)",
    ),
    (
        "chrome.page_tabs.menu",
        &["icons.theme"],
        "gpui: the ChevronDown of the chosen freedesktop theme (Showcase::sample_icon)",
    ),
    (
        "chrome.page_tabs.menu",
        &["icons.set"],
        "gpui: the ChevronDown of the chosen set, none mixed in (Showcase::sample_icon)",
    ),
    // --- button: the icon-only buttons, next::variants::toolbar_button and ghost_button
    (
        "basic.icons.edit",
        &["toolbar.button(CHECKED).fill"],
        "gpui: toolbar.button(CHECKED).fill, next::geometry::toolbar_button_checked's bg, over the selected toolbar Button",
    ),
    (
        "basic.icons.edit",
        &["button.label(CHECKED).color"],
        "gpui: toolbar.button_label(CHECKED).color where a rule states it for CHECKED, else button.label(CHECKED).color, next::geometry::toolbar_button_checked's text colour, where a rule states the toolbar's checked fill, toolbar.button(CHECKED).fill; where neither label is stated, none, the variant's own foreground",
    ),
    (
        "basic.icons.edit",
        &["toolbar.button(CHECKED).stroke.color"],
        "gpui: toolbar.button(CHECKED).stroke.color, next::geometry::toolbar_button_checked's border colour, at next::geometry::toolbar_button_rest's width, kept under the pointer and pressed: a selected Button takes no hover or press style (gpui-component 0.10.0 button/button.rs:733, selected; gpui-component 0.10.0 button/button.rs:758, selected)",
    ),
    (
        "chrome.status_bar.toggle",
        &["button.body(CHECKED).fill"],
        "gpui: demo::ghost_checked_variant's active, the selected toggle's fill while the side panel is open, button.body(CHECKED; variant=Flat).fill where a rule states it for CHECKED; else gpui-component's own selected ghost's, ThemeColor secondary_active (gpui-component 0.10.0 button/button.rs:1353, active; demo::panel_toggle)",
    ),
    (
        "chrome.status_bar.toggle",
        &["button.label(CHECKED).color"],
        "gpui: demo::ghost_checked_variant's foreground, the selected toggle's text colour while the side panel is open, button.label(CHECKED; variant=Flat).color where a rule states it for CHECKED; else gpui-component's own selected ghost's, ThemeColor secondary_foreground (demo::panel_toggle)",
    ),
    (
        "chrome.toolbar.",
        &["button.label(NONE).color"],
        "gpui: variants::ghost_button's foreground, ThemeColor secondary_foreground",
    ),
    (
        "basic.icons.",
        &["button.label(NONE).color"],
        "gpui: variants::ghost_button's foreground, ThemeColor secondary_foreground",
    ),
    (
        "chrome.status_bar.toggle",
        &["button.label(NONE).color"],
        "gpui: variants::ghost_button's foreground, ThemeColor secondary_foreground",
    ),
    (
        "chrome.toolbar.",
        &["button.body(HOVER).fill"],
        "gpui: next::variants::toolbar_button's hover where no rule states toolbar.button(HOVER).fill, ThemeColor secondary_hover (colors.rs)",
    ),
    (
        "basic.icons.",
        &["button.body(HOVER).fill"],
        "gpui: next::variants::toolbar_button's hover where no rule states toolbar.button(HOVER).fill, ThemeColor secondary_hover (colors.rs)",
    ),
    (
        "chrome.status_bar.toggle",
        &["button.body(HOVER).fill"],
        "gpui: variants::ghost_button's hover, ThemeColor secondary_hover (colors.rs)",
    ),
    (
        "chrome.toolbar.",
        &["button.label(HOVER).color"],
        "not reachable: a Custom variant has no hover text colour (gpui-component/button/button.rs:104-149, ButtonCustomVariant; :1225, Custom; :1239-1244, text_color)",
    ),
    (
        "basic.icons.",
        &["button.label(HOVER).color"],
        "not reachable: a Custom variant has no hover text colour (gpui-component/button/button.rs:104-149, ButtonCustomVariant; :1225, Custom; :1239-1244, text_color)",
    ),
    (
        "chrome.status_bar.toggle",
        &["button.label(HOVER).color"],
        "not reachable: a Custom variant has no hover text colour (gpui-component/button/button.rs:104-149, ButtonCustomVariant; :1225, Custom; :1239-1244, text_color)",
    ),
    (
        "chrome.toolbar.",
        &["button.body(PRESSED).fill"],
        "gpui: next::variants::toolbar_button's active where no rule states toolbar.button(PRESSED).fill, ThemeColor secondary_active (colors.rs)",
    ),
    (
        "basic.icons.",
        &["button.body(PRESSED).fill"],
        "gpui: next::variants::toolbar_button's active where no rule states toolbar.button(PRESSED).fill, ThemeColor secondary_active (colors.rs)",
    ),
    (
        "chrome.toolbar.",
        &["button.body(NONE).corners"],
        "gpui: button.body(NONE).corners rounds the tool button's box where stated (demo::tool_button_box)",
    ),
    (
        "basic.icons.",
        &["button.body(NONE).corners"],
        "gpui: button.body(NONE).corners rounds the tool button's box where stated (demo::tool_button_box)",
    ),
    (
        "chrome.status_bar.toggle",
        &["button.body(NONE).corners"],
        "gpui: button.body(NONE).corners rounds the tool button's box where stated (demo::tool_button_box)",
    ),
    (
        "chrome.toolbar.",
        &["button.body(NONE).padding.top"],
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.toolbar.",
        &["button.body(NONE).padding.right"],
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.toolbar.",
        &["button.body(NONE).padding.bottom"],
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.toolbar.",
        &["button.body(NONE).padding.left"],
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "basic.icons.",
        &["button.body(NONE).padding.top"],
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "basic.icons.",
        &["button.body(NONE).padding.right"],
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "basic.icons.",
        &["button.body(NONE).padding.bottom"],
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "basic.icons.",
        &["button.body(NONE).padding.left"],
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.toolbar.",
        &["button.body(NONE).stroke.width"],
        "gpui: button.body(NONE).stroke.width, next::geometry::toolbar_button_rest's border width, where a rule states toolbar.button(HOVER).stroke.color",
    ),
    (
        "basic.icons.",
        &["button.body(NONE).stroke.width"],
        "gpui: button.body(NONE).stroke.width, next::geometry::toolbar_button_rest's border width, where a rule states toolbar.button(HOVER).stroke.color",
    ),
    (
        "chrome.toolbar.",
        &["toolbar.button(HOVER).fill"],
        "gpui: toolbar.button(HOVER).fill, next::variants::toolbar_button's hover",
    ),
    (
        "basic.icons.",
        &["toolbar.button(HOVER).fill"],
        "gpui: toolbar.button(HOVER).fill, next::variants::toolbar_button's hover",
    ),
    (
        "chrome.toolbar.",
        &["toolbar.button(PRESSED).fill"],
        "gpui: toolbar.button(PRESSED).fill, next::variants::toolbar_button's pressed fill",
    ),
    (
        "basic.icons.",
        &["toolbar.button(PRESSED).fill"],
        "gpui: toolbar.button(PRESSED).fill, next::variants::toolbar_button's pressed fill",
    ),
    (
        "chrome.toolbar.",
        &["toolbar.button(HOVER).stroke.color"],
        "gpui: toolbar.button(HOVER).stroke.color, next::variants::toolbar_button's edge under the pointer",
    ),
    (
        "chrome.toolbar.",
        &["toolbar.button(PRESSED).stroke.color"],
        "not reachable: a custom variant has one border colour for hover and press, toolbar.button(HOVER).stroke.color's, next::variants::toolbar_button's edge (gpui-component 0.10.0 button/button.rs:1134, Custom: border_color's arm for a custom variant)",
    ),
    (
        "basic.icons.",
        &["toolbar.button(HOVER).stroke.color"],
        "gpui: toolbar.button(HOVER).stroke.color, next::variants::toolbar_button's edge under the pointer",
    ),
    (
        "basic.icons.",
        &["toolbar.button(PRESSED).stroke.color"],
        "not reachable: a custom variant has one border colour for hover and press, toolbar.button(HOVER).stroke.color's, next::variants::toolbar_button's edge (gpui-component 0.10.0 button/button.rs:1134, Custom: border_color's arm for a custom variant)",
    ),
    (
        "basic.icons.",
        &["toolbar.button(HOVER|CHECKED).stroke.color"],
        "not reachable: a selected Button takes no hover or press style, so the checked tool button keeps toolbar.button(CHECKED).stroke.color under the pointer (gpui-component 0.10.0 button/button.rs:733, selected; gpui-component 0.10.0 button/button.rs:758, selected)",
    ),
    (
        "chrome.status_bar.toggle",
        &["button.body(NONE).padding.top"],
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.status_bar.toggle",
        &["button.body(NONE).padding.right"],
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.status_bar.toggle",
        &["button.body(NONE).padding.bottom"],
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    (
        "chrome.status_bar.toggle",
        &["button.body(NONE).padding.left"],
        "gpui: button.body(NONE)'s padding side pads the icon, where stated, for the Size's own (demo::tool_button_box)",
    ),
    // --- button: a Default Button refined by next::geometry::button ------------------
    (
        "",
        &["button.body(PRESSED).fill"],
        "gpui: ThemeColor button_active, laid over the fill source-over (colors.rs, over); the pressed fill",
    ),
    (
        "",
        &["button.label(PRESSED).color"],
        "not reachable: a pressed Button keeps its variant's text colour (gpui-component/button/button.rs:1315-1319, text_color)",
    ),
    (
        "",
        &["button.body(NONE).fill"],
        "gpui: ThemeColor button (colors.rs, assign_buttons): a Default Button's fill",
    ),
    (
        "",
        &["button.body(NONE).stroke.color"],
        "gpui: next::geometry::button's border colour",
    ),
    (
        "",
        &["button.body(NONE).corners"],
        "gpui: next::geometry::button's corner radius",
    ),
    (
        "",
        &["button.body(NONE).stroke.width"],
        "gpui: next::geometry::button's border width",
    ),
    (
        "",
        &["button.body(NONE).padding.top"],
        "gpui: next::geometry::button pads the stated side",
    ),
    (
        "",
        &["button.body(NONE).padding.right"],
        "gpui: next::geometry::button pads the stated side",
    ),
    (
        "",
        &["button.body(NONE).padding.bottom"],
        "gpui: next::geometry::button pads the stated side",
    ),
    (
        "",
        &["button.body(NONE).padding.left"],
        "gpui: next::geometry::button pads the stated side",
    ),
    (
        "",
        &["button.body(NONE).shadows"],
        "not reachable: only a Custom variant takes a shadow (gpui-component/button/button.rs:1151-1156, shadow)",
    ),
    (
        "",
        &["button.body(DISABLED).fill"],
        "gpui: next::geometry::button_disabled fills a disabled Button with it",
    ),
    (
        "",
        &["button.disabled_opacity"],
        "gpui: next::geometry::button_disabled fades a disabled Button by it",
    ),
    (
        "",
        &["button.label(DISABLED).color"],
        "gpui: next::geometry::button_disabled letters a disabled Button in it",
    ),
    (
        "",
        &["button.body(FOCUS_VISIBLE).stroke.color"],
        "not reachable: with gpui-component's band off, a focused Button's border is ThemeColor ring where it has a border width, and where it has none a 1 px line: inside it in its text colour at 60 %, but just outside it in ring on a link or a text Button and on its edge in ring on an upstream ghost Button; with the band on, its border is ring and the band is drawn round it (styled.rs, focus_style)",
    ),
    (
        "",
        &["button.label(NONE)"],
        "gpui: `next::geometry::button_label` gives the label, the Button's child, `button.label(NONE)`'s family, size, weight and style (demo::labelled); a `.label()` would be upstream's `text_sm` at Medium (gpui-component/sizing.rs:337-343, button_text_size)",
    ),
    (
        "",
        &["button.label(NONE).color"],
        "gpui: ThemeColor button_foreground (colors.rs): the label's colour",
    ),
    (
        "",
        &["button.body(HOVER).fill"],
        "gpui: ThemeColor button_hover, laid over the fill source-over (colors.rs, over): the hover fill",
    ),
    (
        "",
        &["button.label(HOVER).color"],
        "not reachable: a hovered Button keeps its variant's text colour (gpui-component/button/button.rs:1239-1244, text_color)",
    ),
    (
        "",
        &["button.body(NONE).min_size[1]"],
        "gpui: next::geometry::button's height, by the control-height rule",
    ),
    (
        "",
        &["button.body(NONE).min_size[0]"],
        "gpui: next::geometry::button's min_w",
    ),
    (
        "",
        &["button.body(NONE; variant=Primary).fill"],
        "gpui: ThemeColor primary → button_primary (colors.rs): the Primary Button's fill",
    ),
    (
        "",
        &["button.label(NONE; variant=Primary).color"],
        "gpui: ThemeColor primary_foreground → button_primary_foreground (colors.rs)",
    ),
    // --- card: demo::card, a Fill GroupBox --------------------------------------
    (
        "",
        &["card.container(NONE).fill"],
        "gpui: card.container(NONE).fill, next::geometry::group_box_content's bg",
    ),
    (
        "",
        &["card.container(NONE).stroke.color"],
        "gpui: card.container(NONE).stroke.color, next::geometry::group_box_content's border colour",
    ),
    (
        "",
        &["card.container(NONE).corners"],
        "gpui: card.container(NONE).corners, next::geometry::group_box_content's corner radius",
    ),
    (
        "",
        &["card.container(NONE).stroke.width"],
        "gpui: card.container(NONE).stroke.width, next::geometry::group_box_content's border width",
    ),
    (
        "",
        &["card.container(NONE).padding.top"],
        "gpui: card.container(NONE)'s padding side, next::geometry::group_box_content pads the stated side; container_margin the rest (demo::card)",
    ),
    (
        "",
        &["card.container(NONE).padding.right"],
        "gpui: card.container(NONE)'s padding side, next::geometry::group_box_content pads the stated side; container_margin the rest (demo::card)",
    ),
    (
        "",
        &["card.container(NONE).padding.bottom"],
        "gpui: card.container(NONE)'s padding side, next::geometry::group_box_content pads the stated side; container_margin the rest (demo::card)",
    ),
    (
        "",
        &["card.container(NONE).padding.left"],
        "gpui: card.container(NONE)'s padding side, next::geometry::group_box_content pads the stated side; container_margin the rest (demo::card)",
    ),
    (
        "",
        &["card.container(NONE).shadows"],
        "gpui: not applied — the GroupBox draws no shadow (group_box.rs)",
    ),
    // --- checkbox: widgets::Checkbox and widgets::Radio -------------------------
    (
        "",
        &[
            "checkbox.indicator(NONE).fill",
            "radio.indicator(NONE).fill",
        ],
        "gpui: checkbox.indicator(NONE).fill (radio.indicator(NONE).fill), the unchecked indicator's fill (CheckboxLook::next_of)",
    ),
    (
        "",
        &[
            "checkbox.indicator(CHECKED).stroke.color",
            "radio.indicator(CHECKED).stroke.color",
        ],
        "gpui: checkbox.indicator(CHECKED).stroke.color (radio.indicator(CHECKED)), the checked indicator's border (CheckboxLook::next_of)",
    ),
    (
        "",
        &["checkbox.indicator(NONE).corners"],
        "gpui: checkbox.indicator(NONE).corners, the box's corner radius (CheckboxLook::next_of; a radio is round)",
    ),
    (
        "",
        &[
            "checkbox.indicator(NONE).stroke.width",
            "radio.indicator(NONE).stroke.width",
        ],
        "gpui: checkbox.indicator(NONE).stroke.width (radio.indicator(NONE)), the indicator's border width (CheckboxLook::next_of)",
    ),
    (
        "",
        &["checkbox.indicator(NONE).padding.top"],
        "gpui: checkbox.indicator(NONE).padding, the stated side, insets the check mark in the box (CheckboxLook::next_of, CheckboxLook::mark_size)",
    ),
    (
        "",
        &["checkbox.indicator(NONE).padding.right"],
        "gpui: checkbox.indicator(NONE).padding, the stated side, insets the check mark in the box (CheckboxLook::next_of, CheckboxLook::mark_size)",
    ),
    (
        "",
        &["checkbox.indicator(NONE).padding.bottom"],
        "gpui: checkbox.indicator(NONE).padding, the stated side, insets the check mark in the box (CheckboxLook::next_of, CheckboxLook::mark_size)",
    ),
    (
        "",
        &["checkbox.indicator(NONE).padding.left"],
        "gpui: checkbox.indicator(NONE).padding, the stated side, insets the check mark in the box (CheckboxLook::next_of, CheckboxLook::mark_size)",
    ),
    (
        "",
        &["checkbox.checkmark(NONE).stroke_width"],
        "gpui: checkbox.checkmark(NONE).stroke_width, the check mark's stroke (CheckboxLook::next_of, stroked_check)",
    ),
    (
        "",
        &[
            "checkbox.indicator(CHECKED).fill",
            "radio.indicator(CHECKED).fill",
        ],
        "gpui: checkbox.indicator(CHECKED).fill (radio.indicator(CHECKED).fill), the checked indicator's fill (CheckboxLook::next_of)",
    ),
    (
        "",
        &["checkbox.indicator(DISABLED|CHECKED).fill"],
        "gpui: checkbox.indicator(DISABLED).fill (DISABLED|CHECKED checked; radio.indicator), a disabled indicator's fill where a rule states it (CheckboxLook::next_of)",
    ),
    (
        "",
        &["checkbox.disabled_opacity"],
        "gpui: checkbox.disabled_opacity (radio.disabled_opacity), fades the whole disabled control (CheckboxLook::next_of)",
    ),
    (
        "",
        &["checkbox.label(DISABLED).color"],
        "gpui: checkbox.label(DISABLED).color (radio.label(DISABLED)), a disabled control's label where a rule states it for DISABLED; else gpui-component's own muted_foreground, colors.window.foreground_muted (CheckboxLook::next_of)",
    ),
    (
        "",
        &["checkbox.label(NONE)", "radio.label(NONE)"],
        "gpui: checkbox.label(NONE) (radio.label(NONE)), the label's family, size, weight and style (widgets::Checkbox, widgets::Radio)",
    ),
    (
        "",
        &["checkbox.label(NONE).color", "radio.label(NONE).color"],
        "gpui: checkbox.label(NONE).color (radio.label(NONE).color), the label's colour (CheckboxLook::next_of)",
    ),
    (
        "",
        &[
            "checkbox.indicator(HOVER).fill",
            "radio.indicator(HOVER).fill",
        ],
        "gpui: checkbox.indicator(HOVER).fill (radio.indicator(HOVER).fill), over the unchecked fill under the pointer where a rule states it; else the fill at rest (CheckboxLook::next_of)",
    ),
    (
        "",
        &[
            "checkbox.indicator(HOVER).stroke.color",
            "checkbox.indicator(HOVER|CHECKED).stroke.color",
            "radio.indicator(HOVER|CHECKED).stroke.color",
            "radio.indicator(HOVER).stroke.color",
        ],
        "gpui: checkbox.indicator(HOVER).stroke.color (HOVER|CHECKED checked; radio.indicator), the indicator's edge under the pointer where a rule states it; else the edge at rest (CheckboxLook::next_of)",
    ),
    (
        "",
        &[
            "checkbox.checkmark(CHECKED).color",
            "radio.dot(CHECKED).fill",
        ],
        "gpui: checkbox.checkmark(CHECKED).color and radio.dot(CHECKED).fill, the check mark's and the radio dot's colour (CheckboxLook::next_of)",
    ),
    (
        "",
        &[
            "checkbox.indicator(NONE).min_size[0]",
            "checkbox.indicator(NONE).max_size[0]",
            "checkbox.indicator(NONE).min_size[1]",
            "checkbox.indicator(NONE).max_size[1]",
        ],
        "gpui: checkbox.indicator(NONE).min_size[0], the box's side; a radio's circle is radio.indicator(NONE).min_size[0] (CheckboxLook::next_of)",
    ),
    (
        "",
        &["checkbox.label_gap", "radio.label_gap"],
        "gpui: checkbox.label_gap (radio.label_gap), the gap between indicator and label (CheckboxLook::next_of)",
    ),
    (
        "",
        &[
            "radio.dot(NONE).min_size[0]",
            "radio.dot(NONE).max_size[0]",
            "radio.dot(NONE).min_size[1]",
            "radio.dot(NONE).max_size[1]",
        ],
        "gpui: radio.dot(NONE).min_size[0], the selected radio's dot (CheckboxLook::next_radio)",
    ),
    (
        "",
        &[
            "radio.indicator(NONE).min_size[0]",
            "radio.indicator(NONE).max_size[0]",
            "radio.indicator(NONE).min_size[1]",
            "radio.indicator(NONE).max_size[1]",
        ],
        "gpui: radio.indicator(NONE).min_size[0], the radio's circle (CheckboxLook::next_radio)",
    ),
    (
        "",
        &[
            "checkbox.indicator(NONE).stroke.color",
            "radio.indicator(NONE).stroke.color",
        ],
        "gpui: checkbox.indicator(NONE).stroke.color (radio.indicator(NONE)), the unchecked indicator's border (CheckboxLook::next_of)",
    ),
    // --- combo_box: the Combobox of the side panel ------------------------------
    (
        "chrome.side_panel.settings.theme",
        &["combo_box.label(NONE)"],
        "gpui: next::geometry::combobox's family, size, weight and style",
    ),
    (
        "chrome.side_panel.settings.theme",
        &["combo_box.label(NONE).color"],
        "not reachable: the Combobox's disabled colour lands before the caller's style (gpui-component/combobox.rs:1035-1042, refine_style)",
    ),
    (
        "chrome.side_panel.settings.theme",
        &["colors.text_selection.background"],
        "not reachable: the closed Combobox shows its choice, no text field; only its open popup's search field selects typed text (gpui-component 0.10.0 combobox.rs:1078-1108, render_popup_shell)",
    ),
    // --- combo_box: the Basic page's Select, on the 0.8.0 builder --------------
    (
        "basic.drop_down",
        &["combo_box.field(NONE).corners"],
        "gpui: next::geometry::select's radius, and the surface's (demo::combo_surface)",
    ),
    (
        "basic.drop_down",
        &["combo_box.field(NONE).padding.top"],
        "gpui: next::geometry::select pads the stated side",
    ),
    (
        "basic.drop_down",
        &["combo_box.field(NONE).padding.right"],
        "gpui: next::geometry::select pads it, unless an arrow column is stated",
    ),
    (
        "basic.drop_down",
        &["combo_box.field(NONE).padding.bottom"],
        "gpui: next::geometry::select pads the stated side",
    ),
    (
        "basic.drop_down",
        &["combo_box.field(NONE).padding.left"],
        "gpui: next::geometry::select pads the stated side",
    ),
    (
        "basic.drop_down",
        &["combo_box.label(NONE)"],
        "gpui: next::geometry::select's family, size, weight and style",
    ),
    (
        "basic.drop_down",
        &["combo_box.label(NONE).color"],
        "gpui: next::geometry::select's text colour",
    ),
    (
        "basic.drop_down",
        &["combo_box.field(NONE).min_size[1]"],
        "gpui: next::geometry::select's height, by the control-height rule",
    ),
    (
        "basic.drop_down",
        &["combo_box.field(NONE).min_size[0]"],
        "gpui: next::geometry::select's min_w",
    ),
    // --- combo_box: the chrome's Selects, and the Combobox's metrics -----------
    (
        "",
        &["combo_box.arrow_area_width"],
        "gpui: where stated, the trigger keeps upstream's right padding: it has no arrow column",
    ),
    (
        "",
        &["combo_box.chevron(NONE).size"],
        "not reachable: the caret is the trigger's own, sized inside it (gpui-component/select.rs:642-643, Caret)",
    ),
    (
        "",
        &["combo_box.field(NONE).fill"],
        "gpui: next::geometry::combo_box_fill's fill, under the trigger (demo::combo_surface)",
    ),
    (
        "",
        &["combo_box.field(NONE).stroke.color"],
        "gpui: the trigger's edge (demo::combo_fill)",
    ),
    (
        "",
        &["combo_box.field(NONE).corners"],
        "gpui: next::geometry::select's radius, and the surface's (demo::combo_surface)",
    ),
    (
        "",
        &["combo_box.field(NONE).stroke.width"],
        "gpui: not applied — the trigger keeps its own 1px edge (select.rs, input_style)",
    ),
    (
        "",
        &["combo_box.field(NONE).padding.top"],
        "gpui: next::geometry::select pads the stated side",
    ),
    (
        "",
        &["combo_box.field(NONE).padding.right"],
        "gpui: next::geometry::select pads it, unless an arrow column is stated",
    ),
    (
        "",
        &["combo_box.field(NONE).padding.bottom"],
        "gpui: next::geometry::select pads the stated side",
    ),
    (
        "",
        &["combo_box.field(NONE).padding.left"],
        "gpui: next::geometry::select pads the stated side",
    ),
    (
        "",
        &["combo_box.field(NONE).shadows"],
        "gpui: not applied — the trigger draws no shadow (select.rs, combobox.rs)",
    ),
    (
        "",
        &["combo_box.label(NONE)"],
        "gpui: next::geometry::select's family, size, weight and style",
    ),
    (
        "",
        &["combo_box.label(NONE).color"],
        "gpui: next::geometry::select's text colour",
    ),
    (
        "",
        &["combo_box.field(HOVER).fill"],
        "gpui: over the fill under the pointer (demo::combo_surface)",
    ),
    (
        "basic.drop_down.trigger",
        &[
            "combo_box.field(OPEN).fill",
            "combo_box.field(OPEN|HOVER).fill",
        ],
        "not reachable: an open Select draws ring and exposes no open state to style its fill by (gpui-component 0.10.0 select.rs:559, outline_visible; gpui-component 0.10.0 select.rs:596-598, ring)",
    ),
    (
        "basic.drop_down.trigger",
        &[
            "combo_box.field(OPEN).stroke.color",
            "combo_box.field(OPEN|HOVER).stroke.color",
        ],
        "not reachable: an open Select edges itself in ring and exposes no open state to style (gpui-component 0.10.0 select.rs:559, outline_visible; gpui-component 0.10.0 select.rs:596-598, ring)",
    ),
    (
        "",
        &["combo_box.field(HOVER).stroke.color"],
        "gpui: an edge drawn under the trigger, which takes the place of the trigger's under the pointer (demo::combo_surface); upstream's trigger has no hover style",
    ),
    (
        "",
        &["combo_box.field(NONE).min_size[1]"],
        "gpui: next::geometry::select's height, by the control-height rule",
    ),
    (
        "",
        &["combo_box.field(NONE).min_size[0]"],
        "gpui: next::geometry::select's min_w",
    ),
    // --- defaults ----------------------------------------------------------------
    (
        "",
        &["colors.window.border"],
        "gpui: frames the swatch (inspector.rs)",
    ),
    (
        "basic.list.scrollbar",
        &["layout.corner_radius"],
        "gpui: the thumb's corner radius written onto gpui-base, thumb_radius (base_layer::scrollbar_geometry)",
    ),
    (
        "",
        &["layout.corner_radius"],
        "gpui: the swatch's corner radius (inspector.rs)",
    ),
    (
        "chrome.menu.",
        &["layout.border_width"],
        "not applied: the highlighted row's outline is the stroke a rule states for menu.item(HOVER), at that stroke's own width (MenuLook::next_of, MenuBar::popup)",
    ),
    (
        "basic.list.",
        &["layout.border_width"],
        "gpui: the width of an unselected row's outline under the pointer in list.highlight(HOVER).stroke.color (demo::ListRow)",
    ),
    (
        "basic.table",
        &["layout.border_width"],
        "gpui: none: an unselected row's outline under the pointer is list.highlight(HOVER)'s stroke at its own width (demo::files_table, TableLook)",
    ),
    (
        "",
        &["layout.border_width"],
        "gpui: the swatch's frame width (inspector.rs)",
    ),
    (
        "",
        &["focus_ring.stroke.color"],
        "gpui: the ring the theme-drawn Checkbox, Radio, Slider and Switch draw over the control, as stated, and the TabBar inside its focused tab (widgets/mod.rs, FocusRing::next_of, FocusRing::next_indicator), each in place of a part's own ring where the theme states one, PartStyle.focus of checkbox.indicator, radio.indicator, switch.track, slider.thumb or tab.tab (widgets/mod.rs, part_focus); and ThemeColor ring (colors.rs, ring_colour: opaque where the band is off): a focused Input's edge and ring, which focus_ring_style hands to focus_style (gpui-component/styled.rs:179-184, focus_ring_style; :250-298, focus_style). gpui-component's widgets draw that ring 3px wide with its alpha set to 0.5, whatever the colour's own (gpui-component/styled.rs:12, FOCUS_RING_OPACITY; :250-298, focus_style), and the Tab draws another, 2px inset at the full colour, on keyboard focus only (gpui-component/tab/tab.rs:933, ring; :962-973, focus_visible)",
    ),
    (
        "",
        &["focus_ring.offset"],
        "gpui: the theme-drawn Checkbox, Radio, Slider and Switch's ring, a band of its width outward from the control's edge moved out by the offset, drawn over the control, inside its edge too where the offset is negative (widgets/mod.rs, FocusRing::next_indicator, FocusRing::draw), a part's own ring's by its own offset (widgets/mod.rs, part_focus); the TabBar's inset ring lies against the tab's edge whatever the offset, which it reports not mapped (widgets/tab_bar.rs, TabLook::next_ring_unmapped); upstream's own widgets' ring lies against the border, no offset (styled.rs, focus_ring)",
    ),
    (
        "",
        &["focus_ring.trigger"],
        "gpui: when gpui-component's band is drawn (Theme::focus_ring off where true, native-theme-gpui/colors.rs, focus_band_on; on there for a focus that followed a key press in a window under Root, native-theme-gpui/lib.rs, FocusBand), and whether the theme-drawn Checkbox, Radio, Slider and Switch (widgets/mod.rs, focus_seen, ThemeSource::ring) and the TabBar's focused tab (widgets/mod.rs, focus_followed_key, ThemeSource::ring) ring a focus that did not follow a key; a part's own ring by its own trigger (widgets/mod.rs, part_focus)",
    ),
    (
        "",
        &["focus_ring.stroke.width"],
        "gpui: the theme-drawn Checkbox, Radio, Slider and Switch's ring and the TabBar's inset ring, this wide, none where it is not positive (widgets/mod.rs, FocusRing::next_of, FocusRing::next_indicator), a part's own ring by its own width (widgets/mod.rs, part_focus); upstream's own widgets' ring is FOCUS_RING_WIDTH, 3px (gpui-component/styled.rs:11, FOCUS_RING_WIDTH), and a width > 0 turns it on where the theme does not say keyboard only (native-theme-gpui/colors.rs, focus_band_on)",
    ),
    (
        "basic.typography.body",
        &["typography.body"],
        "gpui: family, size, line box, weight and slant on the Label, each where stated (demo::type_line)",
    ),
    (
        "",
        &["typography.body"],
        "gpui: the window's font, Theme::font_family and font_size (config.rs; gpui-component/root.rs:725, font_size; :735, font_family)",
    ),
    (
        "basic.typography",
        &["typography.body.color"],
        "gpui: the Label's text colour (demo::type_line)",
    ),
    (
        "",
        &["typography.body.color"],
        "gpui: the Label paints `foreground`, colors.window.foreground (gpui-component/label.rs:248, foreground)",
    ),
    (
        "basic.typography",
        &["typography.body.family"],
        "gpui: font_family on the Label (demo::type_line)",
    ),
    (
        "",
        &["typography.body.family"],
        "gpui: the window's font, Theme::font_family (config.rs), set by gpui-component's WindowState root plugin (gpui-component/root.rs:735, font_family)",
    ),
    (
        "basic.icons.large",
        &["icons.sizes.large"],
        "gpui: icons.sizes.large, next::geometry::icon_size_large sizes the icon (demo::sized_icon)",
    ),
    (
        "basic.icons",
        &["icons.sizes.small"],
        "gpui: icons.sizes.small, next::geometry::icon_size_small sizes the icon (demo::sized_icon)",
    ),
    (
        "",
        &["icons.sizes.small"],
        "gpui: next::geometry::icon_size_small sizes the icon",
    ),
    (
        "",
        &["icons.sizes.toolbar"],
        "gpui: next::geometry::icon_size_toolbar, toolbar.button_icon(NONE).size",
    ),
    (
        "basic.spinner",
        &["icons.theme"],
        "gpui: widgets::Spinner draws that theme's own loading indicator (spinner.rs)",
    ),
    (
        "",
        &["icons.theme"],
        "gpui: the freedesktop theme the icon-theme Select names, by default the theme's (app.rs)",
    ),
    (
        "basic.buttons",
        &["typography.line_height_multiplier"],
        "gpui: next::geometry::button's line height (control-height rule)",
    ),
    (
        "basic.checkboxes",
        &["typography.line_height_multiplier"],
        "gpui: the label's line height (widgets::Checkbox)",
    ),
    (
        "basic.radios",
        &["typography.line_height_multiplier"],
        "gpui: the label's line height (widgets::Radio)",
    ),
    (
        "basic.switches",
        &["typography.line_height_multiplier"],
        "gpui: widgets::Switch's label, one line of its font's size by it",
    ),
    (
        "basic.switches",
        &["switch.label_gap"],
        "gpui: widgets::Switch's gap between the track and the label where stated, gpui-component's own gap_2 (widgets/switch.rs, LABEL_GAP) where not; the two at the row's ends where switch.label_gap_stretches is true (SwitchLook::next_of)",
    ),
    (
        "basic.text_inputs",
        &["typography.line_height_multiplier"],
        "gpui: next::geometry::input's line height (control-height rule)",
    ),
    (
        "basic.number_input",
        &["typography.line_height_multiplier"],
        "gpui: next::geometry::input's line height (control-height rule)",
    ),
    (
        "basic.text_area",
        &["typography.line_height_multiplier"],
        "gpui: next::geometry::text_area's line height",
    ),
    (
        "basic.drop_down",
        &["typography.line_height_multiplier"],
        "gpui: next::geometry::select's line height (control-height rule)",
    ),
    (
        "chrome.side_panel.settings.theme_label",
        &["typography.line_height_multiplier"],
        "gpui: the label's line box, the installed 0.8.0 theme's sidebar.item_label(NONE) line_height where stated, else its size by typography.line_height_multiplier (demo::next_text_in)",
    ),
    (
        "chrome.side_panel.settings.palette_label",
        &["typography.line_height_multiplier"],
        "gpui: the label's line box, the installed 0.8.0 theme's sidebar.item_label(NONE) line_height where stated, else its size by typography.line_height_multiplier (demo::next_text_in)",
    ),
    (
        "chrome.side_panel.settings.mode_label",
        &["typography.line_height_multiplier"],
        "gpui: the label's line box, the installed 0.8.0 theme's sidebar.item_label(NONE) line_height where stated, else its size by typography.line_height_multiplier (demo::next_text_in)",
    ),
    (
        "chrome.side_panel.settings.icon_theme_label",
        &["typography.line_height_multiplier"],
        "gpui: the label's line box, the installed 0.8.0 theme's sidebar.item_label(NONE) line_height where stated, else its size by typography.line_height_multiplier (demo::next_text_in)",
    ),
    (
        "chrome.side_panel.settings.theme",
        &["typography.line_height_multiplier"],
        "gpui: next::geometry::combobox's line height (control-height rule)",
    ),
    (
        "chrome.side_panel.settings",
        &["typography.line_height_multiplier"],
        "gpui: next::geometry::select's line height (control-height rule)",
    ),
    (
        "chrome.side_panel.inspector_tabs",
        &["typography.line_height_multiplier"],
        "gpui: widgets::TabBar's label, tab.label(NONE)'s size by it",
    ),
    (
        "chrome.page_tabs",
        &["typography.line_height_multiplier"],
        "gpui: widgets::TabBar's label, tab.label(NONE)'s size by it",
    ),
    (
        "basic.tabs",
        &["typography.line_height_multiplier"],
        "gpui: widgets::TabBar's label, tab.label(NONE)'s size by it",
    ),
    (
        "basic.segmented",
        &["typography.line_height_multiplier"],
        "gpui: a segment's line box, segmented_control.label(NONE)'s line_height where stated, else its size by typography.line_height_multiplier (demo::next_text_in)",
    ),
    (
        "basic.list",
        &["typography.line_height_multiplier"],
        "gpui: next::geometry::list_item's line height (control-height rule)",
    ),
    (
        "basic.table",
        &["typography.line_height_multiplier"],
        "gpui: typography.line_height_multiplier, each row's line box, its label's size by it where the label states no line height (demo::files_table, demo::next_text_in)",
    ),
    (
        "basic.typography.link",
        &["typography.line_height_multiplier"],
        "gpui: the Link's line height, link.label(NONE)'s size by it (demo::link)",
    ),
    (
        "basic.typography",
        &["typography.line_height_multiplier"],
        "gpui: the line box, the font's size by it (demo::type_line)",
    ),
    (
        "basic.expander",
        &["typography.line_height_multiplier"],
        "gpui: the title's line box, expander.header_label(NONE)'s size by it (demo::next_text_in), and the body Label's (demo::body_label)",
    ),
    (
        "basic.card",
        &["typography.line_height_multiplier"],
        "gpui: the Label's line height (demo::body_label)",
    ),
    (
        "chrome.menu.theme.preferences.shortcut",
        &["typography.line_height_multiplier"],
        "gpui: typography.line_height_multiplier, the row's line height as next::geometry::menu_item sets it, which the plain Kbd text inherits (MenuLook::next_of; gpui-component/kbd.rs:236-238, appearance)",
    ),
    (
        "chrome.menu.",
        &["typography.line_height_multiplier"],
        "gpui: typography.line_height_multiplier, the row's line height as next::geometry::menu_item sets it (MenuLook::next_of)",
    ),
    (
        "chrome.menu_bar.",
        &["typography.line_height_multiplier"],
        "gpui: typography.line_height_multiplier, the title's line height as next::geometry::menu_item sets it (MenuLook::title)",
    ),
    (
        "chrome.status_bar",
        &["typography.line_height_multiplier"],
        "gpui: next::geometry::status_bar's line height, status_bar.label's size by it, which the bar's text takes (demo::status_bar)",
    ),
    (
        "chrome.info",
        &["typography.line_height_multiplier"],
        "gpui: the line box of every Widget Info line (inspector.rs)",
    ),
    (
        "",
        &["typography.monospace"],
        "gpui: family, size, line box, weight and slant on the Label, each where stated (demo::type_line)",
    ),
    (
        "",
        &["typography.monospace.color"],
        "gpui: the Label's text colour (demo::type_line)",
    ),
    (
        "",
        &["colors.window.foreground_muted"],
        "gpui: the colour of the hint and the how lines (inspector.rs)",
    ),
    // --- expander: demo::native_expander ----------------------------------------
    (
        "",
        &["expander.arrow(NONE).color"],
        "gpui: expander.arrow(NONE).color, the arrow Icon's colour; upstream's muted_foreground where unstated (demo::native_expander, ExpanderLook)",
    ),
    (
        "",
        &["expander.arrow_gap"],
        "gpui: expander.arrow_gap, the header's gap between arrow and title (demo::native_expander)",
    ),
    (
        "",
        &["expander.arrow(NONE).size"],
        "gpui: expander.arrow(NONE).size, the arrow Icon's size; upstream's xsmall where unstated (demo::native_expander, ExpanderLook)",
    ),
    (
        "",
        &["expander.arrow_side"],
        "gpui: expander.arrow_side, the arrow before or after the title (demo::native_expander)",
    ),
    (
        "",
        &["expander.container(NONE).stroke.color"],
        "gpui: expander.container(NONE).stroke.color, the frame and the line between items, where framed; upstream's border where unstated (demo::native_expander, ExpanderLook)",
    ),
    (
        "",
        &["expander.container(NONE).corners"],
        "gpui: expander.container(NONE).corners, the frame's corner radius, where framed; upstream's radius_lg where unstated (demo::native_expander, ExpanderLook)",
    ),
    (
        "",
        &["expander.container(NONE).stroke.width"],
        "gpui: expander.container(NONE).stroke.width, the frame's and the item line's width, where framed; upstream's border_1 where unstated (demo::native_expander, ExpanderLook)",
    ),
    (
        "",
        &["expander.header(NONE).padding.top"],
        "gpui: expander.header(NONE)'s padding side, pads the header where stated; upstream's py_2 where not (demo::native_expander)",
    ),
    (
        "",
        &["expander.header(NONE).padding.right"],
        "gpui: expander.header(NONE)'s padding side, pads the header where stated; upstream's px_3 where not (demo::native_expander)",
    ),
    (
        "",
        &["expander.header(NONE).padding.bottom"],
        "gpui: expander.header(NONE)'s padding side, pads the header where stated; upstream's py_2 where not (demo::native_expander)",
    ),
    (
        "",
        &["expander.header(NONE).padding.left"],
        "gpui: expander.header(NONE)'s padding side, pads the header where stated; upstream's px_3 where not (demo::native_expander)",
    ),
    (
        "",
        &["expander.content_indent"],
        "gpui: expander.content_indent, the body's left padding (demo::native_expander)",
    ),
    (
        "",
        &["expander.header_label(NONE)"],
        "gpui: expander.header_label(NONE), the title's family, size, weight and style (demo::native_expander, demo::next_text_in)",
    ),
    (
        "",
        &["expander.header_label(NONE).color"],
        "gpui: expander.header_label(NONE).color, the header's text colour; upstream's foreground on an open title where unstated (demo::native_expander, ExpanderLook)",
    ),
    (
        "",
        &["expander.frame_enabled"],
        "gpui: expander.frame_enabled, frames the items and parts them unless false (demo::native_expander)",
    ),
    (
        "",
        &["expander.header(NONE).min_size[1]"],
        "gpui: expander.header(NONE).min_size[1], each header's height where stated (demo::native_expander)",
    ),
    (
        "",
        &["expander.header(HOVER).fill"],
        "gpui: expander.header(HOVER).fill, the header's fill under the pointer where a rule states it for HOVER (demo::native_expander)",
    ),
    // --- input: the Basic page's fields ----------------------------------------
    (
        "basic.number_input",
        &["input.field(NONE).fill"],
        "gpui: the frame's fill (demo::number_input)",
    ),
    (
        "",
        &["input.field(NONE).fill", "text_area.field(NONE).fill"],
        "gpui: the field's fill (demo::input_surface, next::geometry::input_fill)",
    ),
    (
        "basic.number_input",
        &["input.field(NONE).stroke.color"],
        "gpui: the frame's edge (demo::number_input)",
    ),
    (
        "",
        &["input.field(NONE).stroke.color"],
        "gpui: the surface's edge (demo::input_surface); a disabled field's, the line of its faded surface (demo::faded_surface)",
    ),
    (
        "",
        &["input.field(NONE).corners"],
        "gpui: next::geometry::input's corner radius",
    ),
    (
        "",
        &["input.field(NONE).stroke.width"],
        "gpui: next::geometry::input's border width",
    ),
    (
        "basic.number_input",
        &["input.field(NONE).padding.top"],
        "gpui: pads the value's Input left of the step buttons (demo::number_input)",
    ),
    (
        "basic.number_input",
        &["input.field(NONE).padding.right"],
        "gpui: pads the value's Input left of the step buttons (demo::number_input)",
    ),
    (
        "basic.number_input",
        &["input.field(NONE).padding.bottom"],
        "gpui: pads the value's Input left of the step buttons (demo::number_input)",
    ),
    (
        "basic.number_input",
        &["input.field(NONE).padding.left"],
        "gpui: pads the value's Input left of the step buttons (demo::number_input)",
    ),
    (
        "",
        &["input.field(NONE).padding.top"],
        "gpui: next::geometry::input pads the stated side",
    ),
    (
        "",
        &["input.field(NONE).padding.right"],
        "gpui: next::geometry::input pads the stated side",
    ),
    (
        "",
        &["input.field(NONE).padding.bottom"],
        "gpui: next::geometry::input pads the stated side",
    ),
    (
        "",
        &["input.field(NONE).padding.left"],
        "gpui: next::geometry::input pads the stated side",
    ),
    (
        "",
        &["input.field(NONE).shadows"],
        "gpui: not applied — the Input draws no shadow (input/input.rs)",
    ),
    (
        "",
        &["colors.caret"],
        "gpui: ThemeColor caret (colors.rs): the Input's caret",
    ),
    (
        "",
        &["caret.blink_on_ms"],
        "gpui: Theme::cursor_blink, which apply installs (lib.rs, caret_blink): each phase half of caret_blink_on and caret_blink_off together, as gpui-base blinks in equal phases (gpui-base/input/base/blink_cursor.rs, CursorBlink::blinking); upstream's 500ms where the theme states no blink",
    ),
    (
        "",
        &["caret.blink_off_ms"],
        "gpui: Theme::cursor_blink: steady where it is 0 (CursorBlink::steady), else its half of the phase; upstream's 500ms where the theme states no blink",
    ),
    (
        "",
        &["caret.blink_timeout_ms"],
        "gpui: Theme::cursor_blink's idle time-out (CursorBlink::with_idle_timeout): the caret stops blinking, shown, that long after the last focus, edit or movement; it never stops where unstated",
    ),
    (
        "",
        &["input.field(DISABLED).fill"],
        "gpui: next::geometry::input_fill's fill for a disabled field",
    ),
    (
        "",
        &["input.disabled_opacity"],
        "gpui: next::geometry::input_fill fades a disabled field by it",
    ),
    (
        "",
        &["input.text(DISABLED).color"],
        "gpui: next::geometry::input_fill's text colour for a disabled field, which gpui-base draws at half its alpha (gpui-base/input/base/element.rs:3222, opacity)",
    ),
    (
        "",
        &["input.field(FOCUSED).stroke.color"],
        "not reachable: a focused Input edges itself in `ring` after the caller's style (gpui-component/input/input.rs:789-792, ring)",
    ),
    (
        "basic.text_area",
        &["text_area.text(NONE)"],
        "gpui: next::geometry::text_area's family, size, weight and style",
    ),
    (
        "",
        &["input.text(NONE)", "text_area.text(NONE)"],
        "gpui: next::geometry::input's family, size, weight and style",
    ),
    (
        "",
        &["input.text(NONE).color", "text_area.text(NONE).color"],
        "gpui: next::geometry::input's text colour, over the window's `foreground` the field's text inherits (gpui-component/input/input.rs:729, input_style)",
    ),
    (
        "basic.number_input",
        &["input.field(HOVER).stroke.color"],
        "gpui: the frame's edge under the pointer (demo::number_input)",
    ),
    (
        "",
        &[
            "input.field(HOVER).stroke.color",
            "text_area.field(HOVER).stroke.color",
        ],
        "gpui: the surface's edge under the pointer (demo::input_surface)",
    ),
    (
        "",
        &["input.field(NONE).min_size[1]"],
        "gpui: next::geometry::input's height, by the control-height rule",
    ),
    (
        "",
        &["input.placeholder(NONE).color"],
        "not reachable: the placeholder is muted_foreground (gpui-component/input/input.rs:579, muted_foreground)",
    ),
    // --- layout -------------------------------------------------------------------
    (
        "chrome.menu_bar",
        &["layout.container_margin"],
        "gpui: the menu bar's side padding (demo::menu_bar)",
    ),
    (
        "chrome.side_panel.inspector_tabs",
        &["layout.container_margin"],
        "gpui: the tab bar's side inset where sidebar.container(NONE) states no padding side (none where that is unstated); where it states any, the side panel's padding insets it (demo::native_tab_bar)",
    ),
    (
        "chrome.page_tabs",
        &["layout.container_margin"],
        "gpui: the tab bar's side inset (none where that is unstated) (demo::native_tab_bar)",
    ),
    (
        "chrome.side_panel.inspector",
        &["layout.container_margin"],
        "gpui: pads the inspector's content where sidebar.container(NONE) states no padding side (none where that is unstated); where it states any, the side panel's padding pads it (inspector.rs)",
    ),
    (
        "chrome.status_bar",
        &["layout.container_margin"],
        "gpui: not applied — an unstated side keeps StatusBar's px_2 py_1 (gpui-component/status_bar.rs:98-99, px_2), the padding's",
    ),
    (
        "basic.card",
        &["layout.container_margin"],
        "gpui: pads a side card.container(NONE).padding leaves unstated (demo::card)",
    ),
    (
        "",
        &["layout.container_margin"],
        "gpui: the margin (none where that is unstated) pads the theme settings where sidebar.container(NONE) states no padding side, and where it states any, a side of the side panel the padding leaves unstated (demo::side_panel)",
    ),
    (
        "chrome.info",
        &["layout.section_gap"],
        "gpui: the space before a Widget Info section (inspector.rs)",
    ),
    (
        "basic.page",
        &["layout.section_gap"],
        "gpui: the gap between the columns (pages/basic.rs)",
    ),
    (
        "",
        &["layout.section_gap"],
        "gpui: the gap between the column's groups (pages/basic.rs)",
    ),
    (
        "chrome.info",
        &["layout.widget_gap"],
        "gpui: the space between Widget Info rows (inspector.rs)",
    ),
    (
        "chrome.side_panel.inspector",
        &["layout.widget_gap"],
        "gpui: the gap between the inspector's rows (inspector.rs)",
    ),
    (
        "chrome.side_panel",
        &["layout.widget_gap"],
        "gpui: the gap between the settings' rows (demo::theme_settings)",
    ),
    (
        "chrome.status_bar",
        &["layout.widget_gap"],
        "gpui: the gap between the side-panel toggle and the environment text, one item of StatusBar's middle region (demo::status_bar)",
    ),
    (
        "",
        &["layout.widget_gap"],
        "gpui: the gap between a group's heading and rows (pages/basic.rs)",
    ),
    (
        "chrome.content",
        &["layout.window_margin"],
        "gpui: each page pads itself by it (pages/basic.rs)",
    ),
    (
        "",
        &["layout.window_margin"],
        "gpui: the page's padding (pages/basic.rs)",
    ),
    // --- link: demo::link, upstream's Link -------------------------------------
    (
        "",
        &["link.label(PRESSED).color"],
        "not reachable: a pressed Link is `link` at 0.6 opacity (gpui-component/link.rs:84-85, active)",
    ),
    (
        "",
        &["link.container(NONE).fill"],
        "gpui: next::geometry::link's fill, link.container(NONE).fill, where the Link paints none (gpui-component/link.rs:70-90, render)",
    ),
    (
        "",
        &["link.label(NONE)"],
        "gpui: next::geometry::link's family, size, weight and style, link.label(NONE)'s",
    ),
    (
        "",
        &["link.label(NONE).color"],
        "gpui: ThemeColor link (colors.rs): the Link's text (gpui-component/link.rs:76, link)",
    ),
    (
        "",
        &["link.container(HOVER).fill"],
        "not reachable: the Link's hover style is upstream's, no fill (gpui-component/link.rs:79-83, hover)",
    ),
    (
        "",
        &["link.label(HOVER).color"],
        "not reachable: a hovered Link is `link` at 0.8 opacity (gpui-component/link.rs:79-80, hover)",
    ),
    (
        "",
        &["link.label(NONE).decoration"],
        "gpui: next::geometry::link removes upstream's underline where link.label(NONE).decoration is none",
    ),
    // --- list: the Basic page's table, demo::files_table ------------------------
    (
        "basic.table",
        &["list.header(NONE).min_size[1]"],
        "gpui: list.header(NONE).min_size[1] through next::geometry::list_header_height, the header row's height where stated, else a row's (demo::files_table)",
    ),
    (
        "basic.table.header",
        &["list.row(NONE).min_size[1]"],
        "gpui: list.row(NONE).min_size[1], the header row's height where list.header(NONE).min_size[1] is unstated and this is stated, else padding and line (demo::files_table)",
    ),
    (
        "basic.table",
        &["list.container(NONE).fill"],
        "gpui: list.container(NONE).fill through next::geometry::list, the table's fill (demo::files_table, TableLook)",
    ),
    (
        "basic.table",
        &["list.container(NONE).stroke.color"],
        "gpui: list.container(NONE).stroke.color through next::geometry::list, the table's frame (demo::files_table, TableLook)",
    ),
    (
        "basic.table.header",
        &["list.container(NONE).corners"],
        "gpui: the header's fill takes the frame's inner radius, list.container(NONE).corners less its stroke width, on its top corners (demo::files_table, next_list_inner_radius)",
    ),
    (
        "basic.table.header",
        &["list.container(NONE).stroke.width"],
        "gpui: list.container(NONE).stroke.width, the frame's line, which the header lies inside; taken off the frame's radius for the inner radius of the fill's top corners (demo::files_table, next_list_inner_radius)",
    ),
    (
        "basic.table.row_",
        &["list.container(NONE).corners"],
        "gpui: the last row's fill takes the frame's inner radius, list.container(NONE).corners less its stroke width, on its bottom corners (demo::files_table, next_list_inner_radius)",
    ),
    (
        "basic.table.row_",
        &["list.container(NONE).stroke.width"],
        "gpui: list.container(NONE).stroke.width, the frame's line, which the row lies inside; taken off the frame's radius for the inner radius of the last row's bottom corners (demo::files_table, next_list_inner_radius)",
    ),
    (
        "basic.table",
        &["list.container(NONE).corners"],
        "gpui: list.container(NONE).corners through next::geometry::list, the frame's corner radius (demo::files_table, TableLook)",
    ),
    (
        "basic.table",
        &["list.container(NONE).stroke.width"],
        "gpui: list.container(NONE).stroke.width through next::geometry::list, the frame's width (demo::files_table, TableLook)",
    ),
    (
        "basic.table",
        &[
            "list.row(NONE).padding.top",
            "list.highlight(NONE).margin.top",
        ],
        "gpui: list.row(NONE).padding with list.highlight(NONE).margin, as next::geometry::list_item pads a List row, pads each cell, gpui-component's own table cell padding where unstated (demo::files_table)",
    ),
    (
        "basic.table",
        &[
            "list.row(NONE).padding.right",
            "list.highlight(NONE).margin.right",
        ],
        "gpui: list.row(NONE).padding with list.highlight(NONE).margin, as next::geometry::list_item pads a List row, pads each cell, gpui-component's own table cell padding where unstated (demo::files_table)",
    ),
    (
        "basic.table",
        &[
            "list.row(NONE).padding.bottom",
            "list.highlight(NONE).margin.bottom",
        ],
        "gpui: list.row(NONE).padding with list.highlight(NONE).margin, as next::geometry::list_item pads a List row, pads each cell, gpui-component's own table cell padding where unstated (demo::files_table)",
    ),
    (
        "basic.table",
        &[
            "list.row(NONE).padding.left",
            "list.highlight(NONE).margin.left",
        ],
        "gpui: list.row(NONE).padding with list.highlight(NONE).margin, as next::geometry::list_item pads a List row, pads each cell, gpui-component's own table cell padding where unstated (demo::files_table)",
    ),
    (
        "basic.table",
        &["list.highlight(HOVER).fill"],
        "gpui: list.highlight(HOVER).fill, an unselected row's fill under the pointer; upstream's list_hover, which a hovered ListItem paints, where unstated (demo::files_table, TableLook)",
    ),
    (
        "basic.table",
        &["list.item_label(HOVER).color"],
        "gpui: list.item_label(HOVER).color where a rule states it for HOVER, an unselected row's text under the pointer; else the row keeps its text, as a hovered ListItem does (demo::files_table, TableLook)",
    ),
    (
        "basic.table",
        &["list.highlight(HOVER).stroke.color"],
        "gpui: list.highlight(HOVER)'s stroke where a rule states it for HOVER and its width is above 0, an unselected row's outline under the pointer, a box over the row, square but on the last row's bottom corners; none where not, as upstream draws none (demo::files_table, TableLook)",
    ),
    (
        "basic.table",
        &["list.item_label(NONE)"],
        "gpui: list.item_label(NONE), each row's family, size, weight and style (demo::files_table, demo::next_text_in)",
    ),
    (
        "basic.table",
        &["list.item_label(NONE).color"],
        "gpui: list.item_label(NONE).color, an unselected row's text (demo::files_table, TableLook)",
    ),
    (
        "basic.table",
        &["list.row(NONE).min_size[1]"],
        "gpui: list.row(NONE).min_size[1], each row's height where stated, else padding and line, as upstream's table row sets none (demo::files_table, TableLook)",
    ),
    (
        "basic.table",
        &["list.highlight(SELECTED).fill"],
        "gpui: list.highlight(SELECTED).fill, the selected row's fill (demo::files_table, selection_pair)",
    ),
    (
        "basic.table",
        &["list.item_label(SELECTED).color"],
        "gpui: list.item_label(SELECTED).color, the selected row's text (demo::files_table, selection_pair)",
    ),
    (
        "basic.table.",
        &["list.header(HOVER).fill"],
        "gpui: list.header(HOVER).fill where a rule states it, demo::files_table's header cells' .hover(..) bg: gpui-component's own table header has no hover style (gpui-component/table/state.rs:1932-1975, render_th; demo::SectionFills)",
    ),
    (
        "basic.table.",
        &["list.header(PRESSED).fill"],
        "gpui: list.header(PRESSED).fill where a rule states it, else the hovered fill, demo::files_table's header cells' .active(..) bg: gpui-component's own table header has no press style (gpui-component/table/state.rs:1932-1975, render_th; demo::SectionFills)",
    ),
    (
        "basic.table",
        &["list.header_separator(NONE).fill"],
        "gpui: list.header_separator(NONE).fill, the header's line on the first section's trailing edge and along its bottom, laid over the header's and a section's fill; list.grid_color where unstated (demo::files_table, TableLook)",
    ),
    (
        "basic.table.",
        &["colors.selection.unfocused_background"],
        "gpui: colors.selection.unfocused_background, demo::files_table's selected row's fill while the window is not active, where it differs from colors.selection.background (selection_pair, inactive_selection_differs)",
    ),
    (
        "basic.table.",
        &["colors.selection.unfocused_foreground"],
        "gpui: colors.selection.unfocused_foreground, demo::files_table's selected row's text while the window is not active and colors.selection.unfocused_background differs from colors.selection.background; list.item_label(SELECTED).color where unstated (selection_pair)",
    ),
    (
        "",
        &["list.grid_color"],
        "gpui: the column line, and the header's lines where list.header_separator(NONE).fill is unstated (demo::files_table)",
    ),
    (
        "",
        &["list.header(NONE).fill"],
        "gpui: the header row's fill (demo::files_table)",
    ),
    (
        "",
        &["list.header_label(NONE)"],
        "gpui: the header row's family, size, weight and style (demo::files_table)",
    ),
    (
        "",
        &["list.header_label(NONE).color"],
        "gpui: the header row's text (demo::files_table)",
    ),
    // --- list: the List, demo::list and demo::ListRow ---------------------------
    (
        "basic.list.",
        &["colors.selection.unfocused_background"],
        "not reachable: a ListItem paints its selected fill list_active after the caller's style, whatever the window's state (gpui-component/list/list_item.rs:260-267, list_active), and a text colour without its fill is no pair",
    ),
    (
        "basic.list.",
        &["colors.selection.unfocused_foreground"],
        "not reachable: a ListItem paints its selected fill list_active after the caller's style, whatever the window's state (gpui-component/list/list_item.rs:260-267, list_active), and a text colour without its fill is no pair",
    ),
    (
        "basic.list.",
        &["list.highlight(HOVER).stroke.color"],
        "gpui: list.highlight(HOVER).stroke, an unselected row's outline under the pointer, a box over the row in its suffix, on the corners of its fill (demo::ListRow)",
    ),
    (
        "basic.list.row_",
        &["list.container(NONE).corners"],
        "gpui: list.container(NONE).corners, the first row's fill takes the frame's inner radius, this less list.container(NONE).stroke.width, on its top corners and the last row's on its bottom corners (demo::ListRow)",
    ),
    (
        "basic.list.row_",
        &["list.container(NONE).stroke.width"],
        "gpui: list.container(NONE).stroke.width, next::geometry::list's line, which the rows lie inside; taken off the frame's radius for the inner radius of the first and last rows' corners (demo::ListRow)",
    ),
    (
        "",
        &["list.container(NONE).fill"],
        "gpui: list.container(NONE).fill, next::geometry::list's fill",
    ),
    (
        "",
        &["list.container(NONE).stroke.color"],
        "gpui: list.container(NONE).stroke.color, next::geometry::list's border colour",
    ),
    (
        "",
        &["list.container(NONE).corners"],
        "gpui: list.container(NONE).corners, next::geometry::list's corner radius",
    ),
    (
        "",
        &["list.container(NONE).stroke.width"],
        "gpui: list.container(NONE).stroke.width, next::geometry::list's border width",
    ),
    (
        "",
        &[
            "list.row(NONE).padding.top",
            "list.highlight(NONE).margin.top",
        ],
        "gpui: list.row(NONE).padding, with list.highlight(NONE)'s margin, next::geometry::list_item pads each row by the stated side",
    ),
    (
        "",
        &[
            "list.row(NONE).padding.right",
            "list.highlight(NONE).margin.right",
        ],
        "gpui: list.row(NONE).padding, with list.highlight(NONE)'s margin, next::geometry::list_item pads each row by the stated side",
    ),
    (
        "",
        &[
            "list.row(NONE).padding.bottom",
            "list.highlight(NONE).margin.bottom",
        ],
        "gpui: list.row(NONE).padding, with list.highlight(NONE)'s margin, next::geometry::list_item pads each row by the stated side",
    ),
    (
        "",
        &[
            "list.row(NONE).padding.left",
            "list.highlight(NONE).margin.left",
        ],
        "gpui: list.row(NONE).padding, with list.highlight(NONE)'s margin, next::geometry::list_item pads each row by the stated side",
    ),
    (
        "",
        &["list.highlight(HOVER).fill"],
        "gpui: ThemeColor list_hover (colors.rs): a ListItem's hover fill",
    ),
    (
        "",
        &["list.item_label(HOVER).color"],
        "gpui: list.item_label(HOVER).color, the label's colour under the pointer (demo::ListRow)",
    ),
    (
        "",
        &["list.item_label(NONE)"],
        "gpui: list.item_label(NONE), next::geometry::list_item's family, size, weight and style",
    ),
    (
        "",
        &["list.item_label(NONE).color"],
        "gpui: list.item_label(NONE).color, next::geometry::list_item's text colour",
    ),
    (
        "",
        &["list.row(NONE).min_size[1]"],
        "gpui: list.row(NONE).min_size[1], next::geometry::list_item's height where stated (control-height rule)",
    ),
    (
        "",
        &["list.highlight(SELECTED).fill"],
        "gpui: ThemeColor list_active (colors.rs): the selected ListItem's fill",
    ),
    (
        "",
        &["list.item_label(SELECTED).color"],
        "gpui: list.item_label(SELECTED).color, next::geometry::list_item_selected's text colour on the selected row (demo::ListRow)",
    ),
    // --- menu: the showcase's MenuBar (demo::MenuTitle, MenuBar::popup) ---------
    (
        "chrome.menu.theme.preferences.shortcut",
        &["menu.item_label(NONE)"],
        "gpui: menu.item_label(NONE), the row's font as next::geometry::menu_item sets it, which the plain Kbd text inherits (MenuLook::next_of; gpui-component/kbd.rs:236-238, appearance)",
    ),
    (
        "chrome.menu_bar",
        &["menu_bar.bar(NONE).fill"],
        "gpui: menu_bar.bar(NONE).fill, the menu bar's fill (demo::menu_bar)",
    ),
    (
        "chrome.menu_bar.",
        &["menu.item(NONE).padding.top"],
        "gpui: menu.item(NONE)'s padding side pads the title where menu_bar.item(NONE) states none, else MENU_TITLE_PADDING_Y (MenuLook::title, demo::MenuTitle)",
    ),
    (
        "chrome.menu_bar.",
        &["menu.item(NONE).padding.right"],
        "gpui: menu.item(NONE)'s padding side pads the title where menu_bar.item(NONE) states none, else MENU_TITLE_PADDING_X (MenuLook::title, demo::MenuTitle)",
    ),
    (
        "chrome.menu_bar.",
        &["menu.item(NONE).padding.bottom"],
        "gpui: menu.item(NONE)'s padding side pads the title where menu_bar.item(NONE) states none, else MENU_TITLE_PADDING_Y (MenuLook::title, demo::MenuTitle)",
    ),
    (
        "chrome.menu_bar.",
        &["menu.item(NONE).padding.left"],
        "gpui: menu.item(NONE)'s padding side pads the title where menu_bar.item(NONE) states none, else MENU_TITLE_PADDING_X (MenuLook::title, demo::MenuTitle)",
    ),
    (
        "chrome.menu_bar.",
        &[
            "menu_bar.item(NONE).padding.left",
            "menu_bar.item(NONE).padding.right",
        ],
        "gpui: menu_bar.item(NONE)'s left and right padding pads the title, over menu.item(NONE)'s (MenuLook::title)",
    ),
    (
        "chrome.menu_bar.",
        &[
            "menu_bar.item(NONE).padding.top",
            "menu_bar.item(NONE).padding.bottom",
        ],
        "gpui: menu_bar.item(NONE)'s top and bottom padding pads the title, over menu.item(NONE)'s (MenuLook::title)",
    ),
    (
        "chrome.menu_bar.",
        &["menu.item(HOVER).fill"],
        "gpui: menu.item(HOVER).fill where a rule states it for the hover state (else gpui-component's own accent, demo::menu_hover_of), the title's fill under the pointer where no rule states menu_bar.item(HOVER).fill (MenuLook::next_of, demo::MenuTitle)",
    ),
    (
        "chrome.menu_bar.",
        &["menu.item_label(HOVER).color"],
        "gpui: menu.item_label(HOVER).color where a rule states it for the hover state (else as demo::menu_hover_of gives it), the title's text under the pointer where no rule states menu_bar.item_label(HOVER).color (MenuLook::next_of, demo::MenuTitle)",
    ),
    (
        "chrome.menu_bar.",
        &["menu_bar.item(HOVER).fill"],
        "gpui: menu_bar.item(HOVER).fill where a rule states it, the title's fill under the pointer (MenuLook::next_of, MenuLook::title)",
    ),
    (
        "chrome.menu_bar.",
        &["menu_bar.item(OPEN).fill"],
        "gpui: menu_bar.item(OPEN).fill where a rule states it, the title's fill while its menu is open, under the pointer too; where none states it, gpui-component's own open title, a selected Ghost's secondary_active (gpui-component 0.10.0 menu/app_menu_bar.rs AppMenu::render, button/button.rs ButtonVariant::selected; MenuLook::next_of, MenuLook::title)",
    ),
    (
        "chrome.menu_bar.",
        &["menu_bar.item_label(HOVER).color"],
        "gpui: menu_bar.item_label(HOVER).color where a rule states it, the title's text under the pointer (MenuLook::next_of, MenuLook::title)",
    ),
    (
        "chrome.menu_bar.",
        &["menu_bar.item_label(OPEN).color"],
        "gpui: menu_bar.item_label(OPEN).color where a rule states it, the title's text while its menu is open, under the pointer too; where none states it, the title's text at rest, as a selected Ghost keeps its text (gpui-component 0.10.0 button/button.rs ButtonVariant::selected; MenuLook::next_of, MenuLook::title)",
    ),
    (
        "chrome.menu_bar.",
        &["menu.item(NONE).corners"],
        "gpui: menu.item(NONE).corners rounds the title's fill where stated (MenuLook::title)",
    ),
    (
        "chrome.menu_bar.",
        &["menu.panel(NONE).corners"],
        "gpui: menu.panel(NONE).corners rounds the frame of the popup the title opens, where stated, else popover.panel.corners, else popover_style's radius; not the title's fill (MenuRowsLook::of, MenuBar::popup)",
    ),
    (
        "chrome.menu.",
        &["menu.item(NONE).corners"],
        "gpui: menu.item(NONE).corners rounds the highlighted row's fill and its outline where stated (MenuLook::next_of)",
    ),
    (
        "chrome.menu.",
        &["menu.panel(NONE).corners"],
        "gpui: menu.panel(NONE).corners rounds the popup's frame where stated, else popover.panel.corners, else popover_style's radius; not the row's fill (MenuRowsLook::of, MenuBar::popup)",
    ),
    (
        "chrome.menu.",
        &["menu.item(HOVER).stroke.color"],
        "gpui: menu.item(HOVER).stroke.color where a rule states it, the highlighted row's outline, a box over the row and its padding, so nothing moves (MenuLook::next_of, MenuBar::popup)",
    ),
    (
        "chrome.menu.theme",
        &[
            "menu.panel(NONE).padding.left",
            "menu.panel(NONE).padding.right",
        ],
        "gpui: menu.panel(NONE)'s left and right padding insets the rows inside the popup's frame line, over popover.panel's (MenuLook::next_of, MenuBar::popup)",
    ),
    (
        "chrome.menu.theme",
        &[
            "menu.panel(NONE).padding.top",
            "menu.panel(NONE).padding.bottom",
        ],
        "gpui: menu.panel(NONE)'s top and bottom padding insets the rows inside the popup's frame line, over popover.panel's, and leaves no gap between them (MenuLook::next_of, MenuBar::popup)",
    ),
    (
        "chrome.menu.",
        &["menu.item(NONE).min_size[1]"],
        "gpui: menu.item(NONE).min_size[1], the row's least height where menu.item(NONE).max_size[1] is unstated, the content growing past it (next::geometry::menu_item)",
    ),
    (
        "chrome.menu.",
        &["menu.icon_text_gap"],
        "not reachable: the Theme menu's items show no icon, and the popup spaces an item's label and its shortcut by MENU_SHORTCUT_GAP instead (MenuBar::popup)",
    ),
    (
        "chrome.menu.",
        &["menu.item(PRESSED).fill"],
        "gpui: menu.item(PRESSED).fill where a rule states it for the pressed state, the row's fill while the mouse button is held on it (MenuLook::next_of, MenuLook::pressed)",
    ),
    (
        "chrome.menu.",
        &["menu.item_label(PRESSED).color"],
        "gpui: menu.item_label(PRESSED).color where a rule states it for the pressed state, the row's text while the mouse button is held on it, its shortcut's too (MenuLook::next_of, MenuLook::pressed)",
    ),
    (
        "",
        &["menu.panel(NONE).fill"],
        "gpui: menu.panel(NONE).fill, the popup's fill (MenuLook::next_of, MenuBar::popup)",
    ),
    (
        "",
        &["menu.item(NONE).padding.top"],
        "gpui: menu.item(NONE)'s padding side pads the row where stated (next::geometry::menu_item, MenuBar::popup)",
    ),
    (
        "",
        &["menu.item(NONE).padding.right"],
        "gpui: menu.item(NONE)'s padding side pads the row where stated (next::geometry::menu_item), else MENU_ROW_PADDING_X (MenuLook::row, MenuBar::popup)",
    ),
    (
        "",
        &["menu.item(NONE).padding.bottom"],
        "gpui: menu.item(NONE)'s padding side pads the row where stated (next::geometry::menu_item, MenuBar::popup)",
    ),
    (
        "",
        &["menu.item(NONE).padding.left"],
        "gpui: menu.item(NONE)'s padding side pads the row where stated (next::geometry::menu_item), else MENU_ROW_PADDING_X (MenuLook::row, MenuBar::popup)",
    ),
    (
        "",
        &["menu.item_label(NONE)"],
        "gpui: menu.item_label(NONE)'s family, size, weight and style as next::geometry::menu_item sets them (demo::MenuTitle, MenuBar::popup)",
    ),
    (
        "",
        &["menu.item_label(NONE).color"],
        "gpui: menu.item_label(NONE).color, the text colour from MenuLook::next_of (demo::MenuTitle, MenuBar::popup)",
    ),
    (
        "",
        &["menu.item(HOVER).fill"],
        "gpui: menu.item(HOVER).fill where a rule states it for the hover state, the highlighted row's fill, else gpui-component's own accent (MenuLook::next_of, demo::menu_hover_of, MenuBar::popup)",
    ),
    (
        "",
        &["menu.item_label(HOVER).color"],
        "gpui: menu.item_label(HOVER).color where a rule states it for the hover state, the highlighted row's text, else over a stated hover fill menu.item_label(NONE).color, else gpui-component's own accent_foreground (MenuLook::next_of, demo::menu_hover_of, MenuBar::popup)",
    ),
    (
        "",
        &["menu.item(NONE).max_size[1]"],
        "gpui: menu.item(NONE).max_size[1], the row's height by next::geometry::menu_item's control-height rule where stated: the height at a text scale of 1 or less, a minimum above (MenuBar::popup)",
    ),
    (
        "",
        &["menu.separator(NONE).fill"],
        "gpui: menu.separator(NONE).fill, the separator row's fill (MenuLook::next_of)",
    ),
    // --- popover: the Theme menu's popup ----------------------------------------
    (
        "",
        &["popover.panel.stroke.color"],
        "gpui: the popup's edge (MenuLook, MenuBar::popup)",
    ),
    (
        "",
        &["popover.panel.corners"],
        "gpui: the popup's corner radius (MenuLook)",
    ),
    (
        "",
        &["popover.panel.stroke.width"],
        "gpui: the popup's border width (MenuLook)",
    ),
    (
        "",
        &["popover.panel.padding.top"],
        "gpui: pads the popup's rows; upstream's p_1 where unstated (MenuLook, MenuBar::popup)",
    ),
    (
        "",
        &["popover.panel.padding.right"],
        "gpui: pads the popup's rows; upstream's p_1 where unstated (MenuLook, MenuBar::popup)",
    ),
    (
        "",
        &["popover.panel.padding.bottom"],
        "gpui: pads the popup's rows; upstream's p_1 where unstated (MenuLook, MenuBar::popup)",
    ),
    (
        "",
        &["popover.panel.padding.left"],
        "gpui: pads the popup's rows; upstream's p_1 where unstated (MenuLook, MenuBar::popup)",
    ),
    (
        "",
        &["popover.panel.shadows"],
        "gpui: the popup's shadow in colors.shadow, upstream's layers, or none (MenuLook)",
    ),
    // --- progress_bar: widgets::ProgressBar -------------------------------------
    (
        "",
        &["progress_bar.track(NONE).stroke.color"],
        "gpui: progress_bar.track(NONE).stroke.color, the track's edge (ProgressBarLook::next_of)",
    ),
    (
        "",
        &["progress_bar.track(NONE).corners"],
        "gpui: progress_bar.track(NONE).corners, the track's and the fill's corner radius (ProgressBarLook::next_of)",
    ),
    (
        "",
        &["progress_bar.track(NONE).stroke.width"],
        "gpui: progress_bar.track(NONE).stroke.width, the track's border width (ProgressBarLook::next_of)",
    ),
    (
        "",
        &["progress_bar.fill(NONE).fill"],
        "gpui: progress_bar.fill(NONE).fill, widgets::ProgressBar's fill (ProgressBarLook::next_of)",
    ),
    (
        "",
        &["progress_bar.track(NONE).min_size[0]"],
        "gpui: progress_bar.track(NONE).min_size[0], the bar's min_w (ProgressBarLook::next_of)",
    ),
    (
        "",
        &["progress_bar.track(NONE).fill"],
        "gpui: progress_bar.track(NONE).fill, the track's fill (ProgressBarLook::next_of)",
    ),
    (
        "",
        &[
            "progress_bar.track(NONE).min_size[1]",
            "progress_bar.track(NONE).max_size[1]",
        ],
        "gpui: progress_bar.track(NONE).min_size[1], the bar's height (ProgressBarLook::next_of)",
    ),
    // --- scrollbar: gpui-base's, through base_layer -----------------------------
    (
        "basic.list.scrollbar",
        &[
            "scrollbar.thumb(NONE).corners",
            "scrollbar.thumb(UNFOCUSED_VIEW).corners",
        ],
        "gpui: not applied — gpui-base's thumb is rounded by layout.corner_radius (native-theme-gpui next/mod.rs, scrollbar_geometry)",
    ),
    (
        "",
        &[
            "scrollbar.track(NONE; orientation=Vertical).min_size[0]",
            "scrollbar.track(NONE; orientation=Vertical).max_size[0]",
        ],
        "gpui: the track width written onto gpui-base (base_layer::scrollbar_geometry)",
    ),
    (
        "",
        &["scrollbar.min_thumb_length"],
        "gpui: the thumb's shortest length on gpui-base (base_layer::scrollbar_geometry)",
    ),
    (
        "",
        &["scrollbar.overlay_mode"],
        "gpui: Theme::scrollbar_mode, always shown where false (lib.rs, apply)",
    ),
    (
        "",
        &["scrollbar.thumb(PRESSED).fill"],
        "gpui: gpui-base's dragged thumb, the hover colour where unstated (base_layer)",
    ),
    (
        "",
        &[
            "scrollbar.thumb(NONE).fill",
            "scrollbar.thumb(UNFOCUSED_VIEW).fill",
        ],
        "gpui: gpui-base's thumb at rest is scrollbar.thumb(UNFOCUSED_VIEW).fill, the handle of a view without the focus, which is the plain thumb, scrollbar.thumb(NONE).fill, on a theme with no rule for that state, where the theme states it; where it does not, gpui-component's own scrollbar_thumb (native-theme-gpui next/mod.rs, thumb_at_rest; base_layer::scrollbar_styles)",
    ),
    (
        "",
        &["scrollbar.thumb(HOVER).fill"],
        "gpui: gpui-base's thumb under the pointer (base_layer::scrollbar_styles)",
    ),
    (
        "",
        &[
            "scrollbar.thumb(NONE; orientation=Vertical).min_size[0]",
            "scrollbar.thumb(NONE; orientation=Vertical).max_size[0]",
        ],
        "gpui: gpui-base's thumb width in every state (base_layer::scrollbar_geometry)",
    ),
    (
        "",
        &["scrollbar.track(NONE).fill"],
        "gpui: gpui-base's track in every state (base_layer::scrollbar_styles)",
    ),
    // --- segmented_control: demo::segmented -------------------------------------
    (
        "",
        &["segmented_control.segment(SELECTED).fill"],
        "gpui: segmented_control.segment(SELECTED).fill, the selected segment's fill; upstream's background where unstated (demo::segmented, SegmentedLook)",
    ),
    (
        "",
        &["segmented_control.label(SELECTED).color"],
        "gpui: segmented_control.label(SELECTED).color, the selected segment's text; upstream's tab_active_foreground where unstated (demo::segmented, SegmentedLook)",
    ),
    (
        "",
        &["segmented_control.track(NONE).fill"],
        "gpui: segmented_control.track(NONE).fill, the control's fill; upstream's tab_bar_segmented where unstated (demo::segmented, SegmentedLook)",
    ),
    (
        "basic.segmented.divider_",
        &["segmented_control.separator(NONE).fill"],
        "gpui: segmented_control.separator(NONE).fill, the line left of each segment after the first; none where unstated, as upstream's segmented TabBar draws none (demo::segmented, SegmentedLook)",
    ),
    (
        "basic.segmented.divider_",
        &["segmented_control.track(NONE).stroke.color"],
        "gpui: not applied — the line between two segments is drawn in segmented_control.separator(NONE).fill, not in the frame's colour (demo::segmented, SegmentedLook)",
    ),
    (
        "",
        &["segmented_control.track(NONE).stroke.color"],
        "gpui: segmented_control.track(NONE).stroke.color, the frame, none where unstated as upstream's; the lines between segments in segmented_control.separator(NONE).fill, none where unstated, as upstream's segmented TabBar draws none (demo::segmented, SegmentedLook)",
    ),
    (
        "",
        &["segmented_control.track(NONE).corners"],
        "gpui: segmented_control.track(NONE).corners, the frame's corner radius; upstream's radius_lg where unstated (demo::segmented, SegmentedLook)",
    ),
    (
        "",
        &["segmented_control.track(NONE).stroke.width"],
        "gpui: segmented_control.track(NONE).stroke.width, the frame's width; none where unstated, as upstream's (demo::segmented, SegmentedLook)",
    ),
    (
        "",
        &["segmented_control.segment(NONE).padding.top"],
        "gpui: segmented_control.segment(NONE)'s padding side, pads each segment where stated; none where not, as upstream's (demo::segmented)",
    ),
    (
        "",
        &["segmented_control.segment(NONE).padding.right"],
        "gpui: segmented_control.segment(NONE)'s padding side, pads each segment, SEGMENT_PADDING, upstream's, where unstated (demo::segmented)",
    ),
    (
        "",
        &["segmented_control.segment(NONE).padding.bottom"],
        "gpui: segmented_control.segment(NONE)'s padding side, pads each segment where stated; none where not, as upstream's (demo::segmented)",
    ),
    (
        "",
        &["segmented_control.segment(NONE).padding.left"],
        "gpui: segmented_control.segment(NONE)'s padding side, pads each segment, SEGMENT_PADDING, upstream's, where unstated (demo::segmented)",
    ),
    (
        "",
        &["segmented_control.label(NONE)"],
        "gpui: segmented_control.label(NONE), each label's family, size, weight and slant; upstream's text_sm where gpui can take none (demo::segmented, demo::next_text_in)",
    ),
    (
        "",
        &["segmented_control.label(NONE).color"],
        "gpui: segmented_control.label(NONE).color, an unselected segment's text; upstream's tab_foreground where unstated (demo::segmented, SegmentedLook)",
    ),
    (
        "",
        &["segmented_control.segment(HOVER).fill"],
        "gpui: segmented_control.segment(HOVER).fill, an unselected segment's fill under the pointer, where a rule states it (demo::segmented, SegmentedLook)",
    ),
    (
        "",
        &["segmented_control.segment(NONE).min_size[1]"],
        "gpui: segmented_control.segment(NONE).min_size[1], each segment's min_h; SEGMENT_HEIGHT, upstream's, where unstated (demo::segmented, SegmentedLook)",
    ),
    (
        "",
        &["segmented_control.separator_width"],
        "gpui: segmented_control.separator_width, the line left of each segment after the first, where segmented_control.separator(NONE).fill states its colour (demo::segmented)",
    ),
    // --- separator ------------------------------------------------------------------
    (
        "chrome.page_tabs.rule",
        &["separator.color"],
        "gpui: the page row's rule, a widgets::Separator laid under the tab strip, its line's colour (demo::native_tab_bar)",
    ),
    (
        "basic.table",
        &["separator.thickness"],
        "gpui: separator.thickness, the width of the header's bottom line and the column line, in list.grid_color, the header's in list.header_separator(NONE).fill where stated (demo::files_table)",
    ),
    (
        "chrome.page_tabs.rule",
        &["separator.thickness"],
        "gpui: the page row's rule, a widgets::Separator laid under the tab strip, its line's thickness, one device pixel at least (demo::native_tab_bar)",
    ),
    (
        "chrome.menu.",
        &["separator.thickness"],
        "gpui: the separator row's height (MenuLook)",
    ),
    (
        "",
        &["separator.color"],
        "gpui: widgets::Separator's line (SeparatorLook)",
    ),
    (
        "",
        &["separator.thickness"],
        "gpui: widgets::Separator's thickness (SeparatorLook)",
    ),
    // --- sidebar ----------------------------------------------------------------
    (
        "",
        &["sidebar.container(NONE).fill"],
        "gpui: sidebar.container(NONE).fill, ThemeColor sidebar (next/mod.rs): the side panel's fill (demo::side_panel)",
    ),
    (
        "",
        &["sidebar.container(NONE).padding.top"],
        "gpui: sidebar.container(NONE)'s padding side pads the side panel where any side is stated, a side it leaves unstated by layout.container_margin (none where that is unstated); where no side is stated, layout.container_margin pads the settings, the inspector's tab row and its content (none where that is unstated) (demo::side_panel)",
    ),
    (
        "",
        &["sidebar.container(NONE).padding.right"],
        "gpui: sidebar.container(NONE)'s padding side pads the side panel where any side is stated, a side it leaves unstated by layout.container_margin (none where that is unstated); where no side is stated, layout.container_margin pads the settings, the inspector's tab row and its content (none where that is unstated) (demo::side_panel)",
    ),
    (
        "",
        &["sidebar.container(NONE).padding.bottom"],
        "gpui: sidebar.container(NONE)'s padding side pads the side panel where any side is stated, a side it leaves unstated by layout.container_margin (none where that is unstated); where no side is stated, layout.container_margin pads the settings, the inspector's tab row and its content (none where that is unstated) (demo::side_panel)",
    ),
    (
        "",
        &["sidebar.container(NONE).padding.left"],
        "gpui: sidebar.container(NONE)'s padding side pads the side panel where any side is stated, a side it leaves unstated by layout.container_margin (none where that is unstated); where no side is stated, layout.container_margin pads the settings, the inspector's tab row and its content (none where that is unstated) (demo::side_panel)",
    ),
    (
        "chrome.info",
        &["sidebar.item_label(NONE)"],
        "gpui: size and family of every Widget Info line (inspector.rs)",
    ),
    (
        "",
        &["sidebar.item_label(NONE)"],
        "gpui: the label's family, size, weight and style, at sidebar.item_label(NONE) of the installed 0.8.0 theme (demo::sidebar_label)",
    ),
    (
        "chrome.info",
        &["sidebar.item_label(NONE).color"],
        "gpui: the Widget Info text colour (inspector.rs)",
    ),
    (
        "",
        &["sidebar.item_label(NONE).color"],
        "gpui: ThemeColor sidebar_foreground (colors.rs): the side panel's text (demo::side_panel)",
    ),
    // --- slider: widgets::Slider --------------------------------------------------
    (
        "",
        &["slider.active_track(NONE).fill"],
        "gpui: slider.active_track(NONE).fill (DISABLED disabled), the rail's filled stretch (SliderLook::next_of)",
    ),
    (
        "",
        &["slider.thumb(NONE).fill"],
        "gpui: slider.thumb(NONE).fill (DISABLED disabled), the thumb's fill (SliderLook::next_of)",
    ),
    (
        "",
        &[
            "slider.thumb(NONE).min_size[0]",
            "slider.thumb(NONE).max_size[0]",
            "slider.thumb(NONE).min_size[1]",
            "slider.thumb(NONE).max_size[1]",
        ],
        "gpui: slider.thumb(NONE).min_size[0], the thumb's size (SliderLook::next_of)",
    ),
    (
        "",
        &["slider.thumb(HOVER).fill"],
        "gpui: slider.thumb(HOVER).fill, over the thumb under the pointer where a rule states it; else the thumb at rest (SliderLook::next_of)",
    ),
    (
        "",
        &["slider.track(NONE).fill"],
        "gpui: slider.track(NONE).fill (DISABLED disabled), the rail's fill (SliderLook::next_of)",
    ),
    (
        "",
        &[
            "slider.track(NONE; orientation=Horizontal).min_size[1]",
            "slider.track(NONE; orientation=Horizontal).max_size[1]",
        ],
        "gpui: slider.track(NONE).min_size[1], the rail's height (SliderLook::next_of)",
    ),
    (
        "basic.slider.control",
        &["slider.thumb(NONE).stroke.color"],
        "gpui: slider.thumb(NONE).stroke.color, the band inside the thumb at rest, in place of upstream's ring (ThumbOutline::next_of)",
    ),
    (
        "basic.slider.control",
        &["slider.thumb(NONE).stroke.width"],
        "gpui: slider.thumb(NONE).stroke.width, the width of the band inside the thumb; 0, or a stroke stated as none, draws none, upstream's ring included (ThumbOutline::next_of)",
    ),
    (
        "basic.slider.control",
        &[
            "slider.thumb(HOVER).stroke.color",
            "slider.thumb(PRESSED).stroke.color",
            "slider.thumb(HOVER|FOCUSED).stroke.color",
        ],
        "gpui: slider.thumb(HOVER).stroke.color, the band with the pointer on the thumb, over the focus colour, not while it is dragged (ThumbOutline::next_of)",
    ),
    (
        "basic.slider.control",
        &["slider.thumb(FOCUSED).stroke.color"],
        "gpui: slider.thumb(FOCUSED).stroke.color, the band on the end thumb while the slider has the focus (ThumbOutline::next_of)",
    ),
    // --- spinner: widgets::Spinner --------------------------------------------------
    (
        "",
        &["spinner.diameter"],
        "gpui: spinner.diameter, the indicator's size at Medium, and the anchor of the other sizes, gpui-component's own ratios to it (SpinnerLook::next_of_size)",
    ),
    (
        "",
        &["spinner.color"],
        "gpui: spinner.color, the arc, and a monochrome set's indicator tint (SpinnerLook::next_of_size, widgets::Spinner)",
    ),
    (
        "",
        &["spinner.min_diameter"],
        "gpui: spinner.min_diameter, the floor under the indicator's size, at every Size (SpinnerLook::next_of_size)",
    ),
    (
        "",
        &["spinner.stroke_width"],
        "gpui: spinner.stroke_width, the arc's stroke, for a set with no indicator (SpinnerLook::next_of_size)",
    ),
    // --- splitter: demo::resize_handles --------------------------------------------
    (
        "",
        &["splitter.handle(NONE).fill"],
        "gpui: gpui-base's resize handle colour (base_layer::resizable_theme)",
    ),
    (
        "",
        &["splitter.divider_width"],
        "gpui: the painted line's width (demo::resize_handles)",
    ),
    (
        "",
        &["splitter.handle(HOVER).fill"],
        "gpui: the line under the pointer and while dragged (demo::resize_handles, base_layer)",
    ),
    // --- status_bar: upstream's StatusBar, demo::status_bar -----------------------
    (
        "",
        &["status_bar.bar.fill"],
        "gpui: ThemeColor status_bar (colors.rs): the bar's fill (gpui-component/status_bar.rs:102, status_bar)",
    ),
    (
        "",
        &["status_bar.bar.stroke.color"],
        "gpui: ThemeColor status_bar_border (colors.rs): the top edge (gpui-component/status_bar.rs:101, status_bar_border)",
    ),
    (
        "",
        &["status_bar.bar.stroke.width"],
        "gpui: the top edge's width (demo::status_bar)",
    ),
    (
        "",
        &["status_bar.bar.padding.top"],
        "gpui: next::geometry::status_bar pads the stated side; upstream's py_1 the rest",
    ),
    (
        "",
        &["status_bar.bar.padding.right"],
        "gpui: next::geometry::status_bar pads the stated side; upstream's px_2 the rest",
    ),
    (
        "",
        &["status_bar.bar.padding.bottom"],
        "gpui: next::geometry::status_bar pads the stated side; upstream's py_1 the rest",
    ),
    (
        "",
        &["status_bar.bar.padding.left"],
        "gpui: next::geometry::status_bar pads the stated side; upstream's px_2 the rest",
    ),
    (
        "",
        &["status_bar.label"],
        "gpui: next::geometry::status_bar'sfamily, size, weight and style",
    ),
    (
        "",
        &["status_bar.label.color"],
        "gpui: next::geometry::status_bar'stext colour",
    ),
    // --- switch: widgets::Switch ---------------------------------------------------
    (
        "",
        &["switch.label(NONE)"],
        "gpui: switch.label(NONE), the label's family, size, weight and slant, each where stated; else gpui-component's own, text_sm in the window's font (SwitchLabelFont::next_of)",
    ),
    (
        "",
        &["switch.label(NONE).color"],
        "gpui: switch.label(NONE).color, the off label's colour where stated; else gpui-component's own, the window's text, colors.window.foreground (SwitchLook::next_of)",
    ),
    (
        "",
        &["switch.label(CHECKED).color"],
        "gpui: switch.label(CHECKED).color, the on label's colour where stated; else gpui-component's own, the window's text, colors.window.foreground (SwitchLook::next_of)",
    ),
    (
        "",
        &["switch.label(DISABLED|CHECKED).color"],
        "gpui: switch.label(DISABLED|CHECKED).color, a disabled switch's label when on (SwitchLabelHome::DisabledChecked), where a rule for DISABLED states the colour, asked with DISABLED alone; else gpui-component's own muted_foreground, colors.window.foreground_muted (SwitchLook::next_of)",
    ),
    (
        "",
        &["switch.track(CHECKED).fill"],
        "gpui: switch.track(CHECKED).fill, the track's fill when on (SwitchLook::next_of)",
    ),
    (
        "",
        &["switch.track(DISABLED|CHECKED).fill"],
        "gpui: switch.track(DISABLED|CHECKED).fill, a disabled switch's track when on, where a rule states it for DISABLED; else gpui-component's own, the enabled track at half alpha (SwitchLook::next_of)",
    ),
    (
        "",
        &["switch.track(CHECKED).stroke.color"],
        "gpui: switch.track(CHECKED).stroke.color, the on track's outline, switch.track(NONE).stroke.width wide, where stated (SwitchLook::next_of)",
    ),
    (
        "",
        &["switch.track(DISABLED|CHECKED).stroke.color"],
        "gpui: switch.track(DISABLED|CHECKED).stroke.color, a disabled on track's outline, where stated (SwitchLook::next_of)",
    ),
    (
        "",
        &["switch.disabled_opacity"],
        "gpui: switch.disabled_opacity, fades the whole disabled switch (SwitchLook::next_of, SwitchLook::faded_over)",
    ),
    (
        "",
        &["switch.thumb(DISABLED|CHECKED).fill"],
        "gpui: switch.thumb(DISABLED).fill (DISABLED|CHECKED on), a disabled switch's thumb (SwitchLook::next_of)",
    ),
    (
        "",
        &["switch.track(HOVER|CHECKED).fill"],
        "gpui: switch.track(HOVER|CHECKED).fill, over the on track under the pointer where it differs from switch.track(CHECKED).fill; else the track at rest (SwitchLook::next_of)",
    ),
    (
        "",
        &["switch.track(HOVER).fill"],
        "gpui: switch.track(HOVER).fill, over the off track under the pointer where a rule states it; else the track at rest (SwitchLook::next_of)",
    ),
    (
        "",
        &["switch.thumb(CHECKED).fill"],
        "gpui: switch.thumb(CHECKED).fill, the on thumb's fill (SwitchLook::next_of)",
    ),
    (
        "",
        &[
            "switch.thumb(NONE).stroke.color",
            "switch.thumb(CHECKED).stroke.color",
        ],
        "gpui: switch.thumb(NONE).stroke.color (CHECKED on), the thumb's outline, switch.thumb(NONE).stroke.width wide, where stated (SwitchLook::next_of)",
    ),
    (
        "",
        &["switch.thumb(NONE).stroke.width"],
        "gpui: switch.thumb(NONE).stroke.width, the thumb outline's width, inside the thumb (SwitchLook::next_of)",
    ),
    (
        "",
        &[
            "switch.thumb(CHECKED).min_size[0]",
            "switch.thumb(CHECKED).max_size[0]",
            "switch.thumb(CHECKED).min_size[1]",
            "switch.thumb(CHECKED).max_size[1]",
        ],
        "gpui: switch.thumb(CHECKED).min_size[0], the on thumb's size, centred on the track's axis; a thumb taller than its track flush with its ends (SwitchLook::next_of)",
    ),
    (
        "",
        &["switch.thumb(NONE).fill"],
        "gpui: switch.thumb(NONE).fill, the off thumb's fill (SwitchLook::next_of)",
    ),
    (
        "",
        &[
            "switch.thumb(NONE).min_size[0]",
            "switch.thumb(NONE).max_size[0]",
            "switch.thumb(NONE).min_size[1]",
            "switch.thumb(NONE).max_size[1]",
        ],
        "gpui: switch.thumb(NONE).min_size[0], the off thumb's size (SwitchLook::next_of)",
    ),
    (
        "",
        &["switch.track(NONE).stroke.width"],
        "gpui: switch.track(NONE).stroke.width, the track outline's width, inside the track (SwitchLook::next_of)",
    ),
    (
        "",
        &[
            "switch.track(NONE).min_size[1]",
            "switch.track(NONE).max_size[1]",
        ],
        "gpui: switch.track(NONE).min_size[1], the track's height (SwitchLook::next_of)",
    ),
    (
        "",
        &["switch.track(NONE).corners"],
        "gpui: switch.track(NONE).corners, the track's corner radius (SwitchLook::next_of)",
    ),
    (
        "",
        &[
            "switch.track(NONE).min_size[0]",
            "switch.track(NONE).max_size[0]",
        ],
        "gpui: switch.track(NONE).min_size[0], the track's width (SwitchLook::next_of)",
    ),
    (
        "",
        &["switch.track(NONE).fill"],
        "gpui: switch.track(NONE).fill, the track's fill when off (SwitchLook::next_of)",
    ),
    (
        "",
        &["switch.track(NONE).stroke.color"],
        "gpui: switch.track(NONE).stroke.color, the off track's outline, switch.track(NONE).stroke.width wide, where stated (SwitchLook::next_of)",
    ),
    // --- tab: widgets::TabBar ----------------------------------------------------
    // The selected tabs before their bars, whose ids are prefixes of theirs.
    (
        "basic.tabs.one",
        &["tab.active_outward_corner_radius"],
        "gpui: the selected tab's outward corners, painted after the tabs (TabLook::outward_radius)",
    ),
    (
        "chrome.page_tabs.basic",
        &["tab.active_outward_corner_radius"],
        "gpui: the selected tab's outward corners, painted after the tabs (TabLook::outward_radius)",
    ),
    (
        "chrome.side_panel.inspector_tabs.widget",
        &["tab.active_outward_corner_radius"],
        "gpui: the selected tab's outward corners, painted after the tabs (TabLook::outward_radius)",
    ),
    (
        "basic.tabs.bar",
        &["tab.active_outward_corner_radius"],
        "gpui: the tabs' inset at the row's ends (TabLook::outward_radius)",
    ),
    (
        "chrome.page_tabs",
        &["tab.active_outward_corner_radius"],
        "gpui: the tabs' inset at the row's ends (TabLook::outward_radius)",
    ),
    (
        "chrome.side_panel.inspector_tabs",
        &["tab.active_outward_corner_radius"],
        "gpui: the tabs' inset at the row's ends (TabLook::outward_radius)",
    ),
    (
        "basic.page",
        &["tab.active_outward_corner_radius"],
        "gpui: room for both flares in the column rule (basic_column_count)",
    ),
    (
        "",
        &["tab.tab(SELECTED).fill"],
        "gpui: the selected tab's fill (TabLook)",
    ),
    (
        "",
        &["tab.label(SELECTED).color"],
        "gpui: the selected tab's text (TabLook)",
    ),
    (
        "",
        &["tab.tab(NONE).fill"],
        "gpui: an unselected tab's fill (TabLook)",
    ),
    (
        "",
        &["tab.bar(NONE).fill"],
        "gpui: the tab bar's fill (widgets::TabBar)",
    ),
    (
        "",
        &["tab.tab(SELECTED).stroke.color"],
        "gpui: the selected tab's outline (TabLook)",
    ),
    (
        "",
        &["tab.tab(SELECTED).corners"],
        "gpui: the selected tab's top corners (TabLook)",
    ),
    (
        "",
        &["tab.tab(SELECTED).stroke.width"],
        "gpui: every tab's border width, shown on the selected one (widgets::TabBar)",
    ),
    (
        "",
        &["tab.tab(NONE).padding.top"],
        "gpui: pads each tab by the stated side (TabLook)",
    ),
    (
        "",
        &["tab.tab(NONE).padding.right"],
        "gpui: pads each tab, gpui-component's 12px where unstated (TabLook)",
    ),
    (
        "",
        &["tab.tab(NONE).padding.bottom"],
        "gpui: pads each tab by the stated side (TabLook)",
    ),
    (
        "",
        &["tab.tab(NONE).padding.left"],
        "gpui: pads each tab, gpui-component's 12px where unstated (TabLook)",
    ),
    (
        "",
        &["tab.label(NONE)"],
        "gpui: each label's family, size, weight and style (widgets::TabBar)",
    ),
    (
        "",
        &["tab.label(NONE).color"],
        "gpui: an unselected tab's text (TabLook)",
    ),
    (
        "",
        &["tab.tab(HOVER).fill"],
        "gpui: an unselected tab's fill under the pointer, over the bar (TabLook)",
    ),
    (
        "",
        &["tab.label(HOVER).color"],
        "gpui: an unselected tab's text under the pointer where a rule states it for HOVER; else the selected tab's, gpui-component's own (TabLook)",
    ),
    (
        "",
        &["tab.item_gap"],
        "gpui: the gap between tabs (TabLook)",
    ),
    (
        "",
        &["tab.indicator(SELECTED).fill"],
        "gpui: the selected tab's line (TabLook, TabIndicator)",
    ),
    (
        "",
        &["tab.indicator_side"],
        "gpui: the edge the selected tab's line lies along (TabIndicator)",
    ),
    (
        "",
        &[
            "tab.indicator(SELECTED).min_size[1]",
            "tab.indicator(SELECTED).max_size[1]",
        ],
        "gpui: the selected tab's line's thickness (TabIndicator)",
    ),
    (
        "",
        &["tab.tab(NONE).min_size[1]"],
        "gpui: each tab's min_h (TabLook)",
    ),
    (
        "",
        &["tab.tab(NONE).min_size[0]"],
        "gpui: each tab's min_w (TabLook)",
    ),
    // --- text_area: demo::rows_textarea ------------------------------------------
    (
        "",
        &["text_area.field(NONE).stroke.color"],
        "gpui: the surface's edge (demo::input_surface)",
    ),
    (
        "",
        &["text_area.field(NONE).corners"],
        "gpui: next::geometry::text_area's corner radius",
    ),
    (
        "",
        &["text_area.field(NONE).stroke.width"],
        "gpui: next::geometry::text_area's border width",
    ),
    (
        "",
        &["text_area.field(NONE).padding.top"],
        "gpui: next::geometry::text_area pads past the editor's own padding",
    ),
    (
        "",
        &["text_area.field(NONE).padding.right"],
        "gpui: next::geometry::text_area pads past the editor's own padding",
    ),
    (
        "",
        &["text_area.field(NONE).padding.bottom"],
        "gpui: next::geometry::text_area pads past the editor's own padding",
    ),
    (
        "",
        &["text_area.field(NONE).padding.left"],
        "gpui: next::geometry::text_area pads past the editor's own padding",
    ),
    (
        "",
        &["text_area.field(NONE).shadows"],
        "gpui: not applied — the Textarea draws no shadow (input/input.rs)",
    ),
    // --- text_scale ------------------------------------------------------------------
    (
        "",
        &["typography.caption"],
        "gpui: family, size, line box, weight, slant and colour on the Label, each where stated, typography.caption laid over typography.body, each property it leaves unstated the body's (Typography::over_body; demo::type_line)",
    ),
    (
        "",
        &["typography.title1"],
        "gpui: family, size, line box, weight, slant and colour on the Label, each where stated, typography.title1 laid over typography.body, each property it leaves unstated the body's (Typography::over_body; demo::type_line)",
    ),
    (
        "basic.typography.section_heading",
        &["typography.headline"],
        "gpui: family, size, line box, weight, slant and colour on the Label, each where stated, typography.headline laid over typography.body, each property it leaves unstated the body's (Typography::over_body; demo::type_line)",
    ),
    (
        "",
        &["typography.headline"],
        "gpui: size, line box and weight on the heading Label, each where stated, typography.headline laid over typography.body (Typography::over_body); its family and colour are the Label's own, not read (demo::heading)",
    ),
    // --- theme_variant ---------------------------------------------------------------
    (
        "basic.spinner",
        &["icons.set"],
        "gpui: widgets::Spinner draws that set's own loading indicator (spinner.rs)",
    ),
    (
        "",
        &["icons.set"],
        "gpui: the set the icon-theme Select names, by default the theme's (app.rs)",
    ),
    // --- toolbar -----------------------------------------------------------------------
    (
        "chrome.toolbar.",
        &["toolbar.button_label(NONE)"],
        "gpui: toolbar.button_label(NONE), set on the row (next::geometry::toolbar); the button draws an icon only",
    ),
    (
        "",
        &["menu_bar.bar(NONE).fill", "toolbar.bar(NONE).fill"],
        "gpui: toolbar.bar(NONE).fill, next::geometry::toolbar's fill",
    ),
    (
        "",
        &["toolbar.bar(NONE).min_size[1]"],
        "gpui: toolbar.bar(NONE).min_size[1], next::geometry::toolbar's min_h where stated",
    ),
    (
        "",
        &["toolbar.bar(NONE).stroke.color"],
        "gpui: toolbar.bar(NONE).stroke.color, next::geometry::toolbar's bottom edge colour",
    ),
    (
        "",
        &["toolbar.bar(NONE).stroke.width"],
        "gpui: toolbar.bar(NONE).stroke.width, next::geometry::toolbar's bottom edge, inside the bar",
    ),
    (
        "",
        &["toolbar.bar(NONE).padding.top"],
        "gpui: toolbar.bar(NONE)'s padding side, next::geometry::toolbar pads the stated side; layout.container_margin the rest (demo::toolbar)",
    ),
    (
        "",
        &["toolbar.bar(NONE).padding.right"],
        "gpui: toolbar.bar(NONE)'s padding side, next::geometry::toolbar pads the stated side; layout.container_margin the rest (demo::toolbar)",
    ),
    (
        "",
        &["toolbar.bar(NONE).padding.bottom"],
        "gpui: toolbar.bar(NONE)'s padding side, next::geometry::toolbar pads the stated side; layout.container_margin the rest (demo::toolbar)",
    ),
    (
        "",
        &["toolbar.bar(NONE).padding.left"],
        "gpui: toolbar.bar(NONE)'s padding side, next::geometry::toolbar pads the stated side; layout.container_margin the rest (demo::toolbar)",
    ),
    (
        "",
        &["toolbar.button_icon(NONE).size"],
        "gpui: toolbar.button_icon(NONE).size, next::geometry::icon_size_toolbar sizes the icon",
    ),
    (
        "",
        &["toolbar.item_gap"],
        "gpui: toolbar.item_gap, next::geometry::toolbar's gap where stated, else layout.widget_gap (demo::toolbar)",
    ),
    // --- tooltip: demo::built_tooltip_button --------------------------------------
    (
        "",
        &["tooltip.panel.fill"],
        "gpui: next::geometry::tooltip's fill",
    ),
    (
        "",
        &["tooltip.panel.stroke.color"],
        "gpui: next::geometry::tooltip's border colour",
    ),
    (
        "",
        &["tooltip.panel.corners"],
        "gpui: next::geometry::tooltip's corner radius",
    ),
    (
        "",
        &["tooltip.panel.stroke.width"],
        "gpui: next::geometry::tooltip's border width",
    ),
    (
        "",
        &["tooltip.panel.padding.top"],
        "gpui: next::geometry::tooltip pads the stated side",
    ),
    (
        "",
        &["tooltip.panel.padding.right"],
        "gpui: next::geometry::tooltip pads the stated side",
    ),
    (
        "",
        &["tooltip.panel.padding.bottom"],
        "gpui: next::geometry::tooltip pads the stated side",
    ),
    (
        "",
        &["tooltip.panel.padding.left"],
        "gpui: next::geometry::tooltip pads the stated side",
    ),
    (
        "",
        &["tooltip.panel.shadows"],
        "gpui: next::geometry::tooltip's shadow: shadow_md's layers in colors.shadow, or none",
    ),
    (
        "",
        &["tooltip.text"],
        "gpui: next::geometry::tooltip's family, size, weight and style",
    ),
    (
        "",
        &["tooltip.text.color"],
        "gpui: next::geometry::tooltip's text colour",
    ),
    (
        "",
        &["tooltip.panel.max_size[0]"],
        "gpui: next::geometry::tooltip_content, less the bubble's padding and border",
    ),
    (
        "",
        &["timings.tooltip_show_ms"],
        "gpui: tooltip_show_delay on the element of gpui's own tooltip where stated, else gpui's 500ms (demo::built_tooltip_button)",
    ),
    // --- window ---------------------------------------------------------------------------
    (
        "",
        &["window.frame(normal).fill"],
        "gpui: ThemeColor background (colors.rs): the window's fill (Showcase::render)",
    ),
];

/// The line for `home` of the element `element`, per [`HOW`].
pub fn how(element: &str, home: &str) -> Option<&'static str> {
    HOW.iter()
        .find(|(prefix, homes, _)| homes.contains(&home) && element.starts_with(prefix))
        .map(|(_, _, line)| *line)
}
