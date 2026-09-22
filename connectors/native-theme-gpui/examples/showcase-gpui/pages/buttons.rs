//! The Buttons tab.

use gpui::{Context, IntoElement, ParentElement, Styled, Window, div, prelude::*};
use gpui_component::{
    ActiveTheme, Disableable, IconName, Sizable, Size,
    button::{Button, ButtonGroup, ButtonVariants, DropdownButton, Toggle, ToggleGroup},
    clipboard::Clipboard,
    h_flex, v_flex,
};

use native_theme_gpui::{geometry, variants};

use crate::app::Showcase;
use crate::support::{NativeStyled, format_font_info, native_geometry, refined, section};
use crate::{PROBE_CLIPBOARD, probe};

impl Showcase {
    // -----------------------------------------------------------------------
    // Tab: Buttons
    // -----------------------------------------------------------------------
    pub(crate) fn render_buttons_tab(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        // One refinement per section (spec §9.1); `None` before `apply` ran.
        let button_style = native_geometry(cx, geometry::button);
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
        let t = cx.theme().clone();
        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            // Button variants
            .child(section("Button Variants (all 10)"))
            .child(
                h_flex()
                    .gap_2()
                    .flex_wrap()
                    .child(
                        div()
                            .id("tt-btn-primary")
                            .child(refined(
                                Button::new("b-primary").label("Primary").primary(),
                                button_style.as_ref(),
                            ))
                            .on_hover(self.hover_info(&fi, "Button (Primary)", &[("bg", "button_primary", t.button_primary, "button/button.rs:936"), ("text", "button_primary_foreground", t.button_primary_foreground, "button/button.rs:954"), ("hover", "button_primary_hover", t.button_primary_hover, "button/button.rs:1086"), ("active", "button_primary_active", t.button_primary_active, "button/button.rs:1170")], &[
                                    ("geometry", "geometry::button: button.min_height/min_width, border.padding_*, corner_radius, line_width, color (spec §9.2)".to_string()), ("font-weight", "geometry::button carries button.font.weight. The label is a child that sets its own size from the Size enum (sizing.rs, button_text_size) and would overrule a size from here, but it sets no weight and neither does anything else on that path, so the platform's weight cascades (button/button.rs, Button::render)".to_string())], &[("edge", "button.border.color at rest: geometry::button gives every variant a border, and upstream's hover and press styles repaint it primary -- gpui applies a hover style after the refinement (gpui-pre/elements/div.rs, hover_style)"), 
                                    ("shadow", "none on a standard variant: Theme::shadow, which the connector sets from border.shadow_enabled, reaches only a ButtonCustomVariant built with .shadow(true) (button/button.rs, ButtonVariant::shadow)"),
                                    ("label size", "a fixed ratio of the platform's base, not a value of its own: the label takes button_text_size (sizing.rs, button_text_size), which is text_xs, text_sm or text_base -- all rems -- and gpui-component sets the rem to Theme::font_size (root.rs, Root::render set_rem_size), which this connector fills from the platform font. So it scales with font.size and cannot be set apart from it: button_text_size has no Size::Size arm"),
                                ])),
                    )
                    .child(
                        div()
                            .id("tt-btn-secondary")
                            .child(refined(
                                Button::new("b-secondary").label("Secondary").secondary(),
                                button_style.as_ref(),
                            ))
                            .on_hover(self.hover_info(&fi, "Button (Secondary)", &[("bg", "button_secondary", t.button_secondary, "button/button.rs:937"), ("text", "button_secondary_foreground", t.button_secondary_foreground, "button/button.rs:961"), ("hover", "button_secondary_hover", t.button_secondary_hover, "button/button.rs:1093"), ("active", "button_secondary_active", t.button_secondary_active, "button/button.rs:1177")], &[
                                    ("geometry", "geometry::button: button.min_height/min_width, border.padding_*, corner_radius, line_width, color (spec §9.2)".to_string()), ("font-weight", "geometry::button carries button.font.weight. The label is a child that sets its own size from the Size enum (sizing.rs, button_text_size) and would overrule a size from here, but it sets no weight and neither does anything else on that path, so the platform's weight cascades (button/button.rs, Button::render)".to_string())], &[("edge", "button.border.color at rest: geometry::button gives every variant a border, and upstream's hover and press styles repaint it border -- gpui applies a hover style after the refinement (gpui-pre/elements/div.rs, hover_style)"), 
                                    ("shadow", "none on a standard variant: Theme::shadow, which the connector sets from border.shadow_enabled, reaches only a ButtonCustomVariant built with .shadow(true) (button/button.rs, ButtonVariant::shadow)"),
                                    ("label size", "a fixed ratio of the platform's base, not a value of its own: the label takes button_text_size (sizing.rs, button_text_size), which is text_xs, text_sm or text_base -- all rems -- and gpui-component sets the rem to Theme::font_size (root.rs, Root::render set_rem_size), which this connector fills from the platform font. So it scales with font.size and cannot be set apart from it: button_text_size has no Size::Size arm"),
                                ])),
                    )
                    .child(
                        div()
                            .id("tt-btn-danger")
                            .child(refined(
                                Button::new("b-danger").label("Danger").danger(),
                                button_style.as_ref(),
                            ))
                            .on_hover(self.hover_info(&fi, "Button (Danger)", &[("bg", "button_danger", t.button_danger, "button/button.rs:938"), ("text", "button_danger_foreground", t.button_danger_foreground, "button/button.rs:969"), ("hover", "button_danger_hover", t.button_danger_hover, "button/button.rs:1100"), ("active", "button_danger_active", t.button_danger_active, "button/button.rs:1185")], &[
                                    ("geometry", "geometry::button: button.min_height/min_width, border.padding_*, corner_radius, line_width, color (spec §9.2)".to_string()), ("font-weight", "geometry::button carries button.font.weight. The label is a child that sets its own size from the Size enum (sizing.rs, button_text_size) and would overrule a size from here, but it sets no weight and neither does anything else on that path, so the platform's weight cascades (button/button.rs, Button::render)".to_string())], &[("edge", "button.border.color at rest: geometry::button gives every variant a border, and upstream's hover and press styles repaint it button_danger -- gpui applies a hover style after the refinement (gpui-pre/elements/div.rs, hover_style)"), 
                                    ("shadow", "none on a standard variant: Theme::shadow, which the connector sets from border.shadow_enabled, reaches only a ButtonCustomVariant built with .shadow(true) (button/button.rs, ButtonVariant::shadow)"),
                                    ("label size", "a fixed ratio of the platform's base, not a value of its own: the label takes button_text_size (sizing.rs, button_text_size), which is text_xs, text_sm or text_base -- all rems -- and gpui-component sets the rem to Theme::font_size (root.rs, Root::render set_rem_size), which this connector fills from the platform font. So it scales with font.size and cannot be set apart from it: button_text_size has no Size::Size arm"),
                                ])),
                    )
                    .child(
                        div()
                            .id("tt-btn-success")
                            .child(refined(
                                Button::new("b-success").label("Success").success(),
                                button_style.as_ref(),
                            ))
                            .on_hover(self.hover_info(&fi, "Button (Success)", &[("bg", "button_success", t.button_success, "button/button.rs:940"), ("text", "button_success_foreground", t.button_success_foreground, "button/button.rs:983"), ("hover", "button_success_hover", t.button_success_hover, "button/button.rs:1114"), ("active", "button_success_active", t.button_success_active, "button/button.rs:1199")], &[
                                    ("geometry", "geometry::button: button.min_height/min_width, border.padding_*, corner_radius, line_width, color (spec §9.2)".to_string()), ("font-weight", "geometry::button carries button.font.weight. The label is a child that sets its own size from the Size enum (sizing.rs, button_text_size) and would overrule a size from here, but it sets no weight and neither does anything else on that path, so the platform's weight cascades (button/button.rs, Button::render)".to_string())], &[("edge", "button.border.color at rest: geometry::button gives every variant a border, and upstream's hover and press styles repaint it button_success -- gpui applies a hover style after the refinement (gpui-pre/elements/div.rs, hover_style)"), 
                                    ("shadow", "none on a standard variant: Theme::shadow, which the connector sets from border.shadow_enabled, reaches only a ButtonCustomVariant built with .shadow(true) (button/button.rs, ButtonVariant::shadow)"),
                                    ("label size", "a fixed ratio of the platform's base, not a value of its own: the label takes button_text_size (sizing.rs, button_text_size), which is text_xs, text_sm or text_base -- all rems -- and gpui-component sets the rem to Theme::font_size (root.rs, Root::render set_rem_size), which this connector fills from the platform font. So it scales with font.size and cannot be set apart from it: button_text_size has no Size::Size arm"),
                                ])),
                    )
                    .child(
                        div()
                            .id("tt-btn-warning")
                            .child(refined(
                                Button::new("b-warning").label("Warning").warning(),
                                button_style.as_ref(),
                            ))
                            .on_hover(self.hover_info(&fi, "Button (Warning)", &[("bg", "button_warning", t.button_warning, "button/button.rs:939"), ("text", "button_warning_foreground", t.button_warning_foreground, "button/button.rs:976"), ("hover", "button_warning_hover", t.button_warning_hover, "button/button.rs:1107"), ("active", "button_warning_active", t.button_warning_active, "button/button.rs:1192")], &[
                                    ("geometry", "geometry::button: button.min_height/min_width, border.padding_*, corner_radius, line_width, color (spec §9.2)".to_string()), ("font-weight", "geometry::button carries button.font.weight. The label is a child that sets its own size from the Size enum (sizing.rs, button_text_size) and would overrule a size from here, but it sets no weight and neither does anything else on that path, so the platform's weight cascades (button/button.rs, Button::render)".to_string())], &[("edge", "button.border.color at rest: geometry::button gives every variant a border, and upstream's hover and press styles repaint it button_warning -- gpui applies a hover style after the refinement (gpui-pre/elements/div.rs, hover_style)"), 
                                    ("shadow", "none on a standard variant: Theme::shadow, which the connector sets from border.shadow_enabled, reaches only a ButtonCustomVariant built with .shadow(true) (button/button.rs, ButtonVariant::shadow)"),
                                    ("label size", "a fixed ratio of the platform's base, not a value of its own: the label takes button_text_size (sizing.rs, button_text_size), which is text_xs, text_sm or text_base -- all rems -- and gpui-component sets the rem to Theme::font_size (root.rs, Root::render set_rem_size), which this connector fills from the platform font. So it scales with font.size and cannot be set apart from it: button_text_size has no Size::Size arm"),
                                ])),
                    )
                    .child(
                        div()
                            .id("tt-btn-info")
                            .child(refined(
                                Button::new("b-info").label("Info").info(),
                                button_style.as_ref(),
                            ))
                            .on_hover(self.hover_info(&fi, "Button (Info)", &[("bg", "button_info", t.button_info, "button/button.rs:941"), ("text", "button_info_foreground", t.button_info_foreground, "button/button.rs:990"), ("hover", "button_info_hover", t.button_info_hover, "button/button.rs:1121"), ("active", "button_info_active", t.button_info_active, "button/button.rs:1206")], &[
                                    ("geometry", "geometry::button: button.min_height/min_width, border.padding_*, corner_radius, line_width, color (spec §9.2)".to_string()), ("font-weight", "geometry::button carries button.font.weight. The label is a child that sets its own size from the Size enum (sizing.rs, button_text_size) and would overrule a size from here, but it sets no weight and neither does anything else on that path, so the platform's weight cascades (button/button.rs, Button::render)".to_string())], &[("edge", "button.border.color at rest: geometry::button gives every variant a border, and upstream's hover and press styles repaint it button_info -- gpui applies a hover style after the refinement (gpui-pre/elements/div.rs, hover_style)"), 
                                    ("shadow", "none on a standard variant: Theme::shadow, which the connector sets from border.shadow_enabled, reaches only a ButtonCustomVariant built with .shadow(true) (button/button.rs, ButtonVariant::shadow)"),
                                    ("label size", "a fixed ratio of the platform's base, not a value of its own: the label takes button_text_size (sizing.rs, button_text_size), which is text_xs, text_sm or text_base -- all rems -- and gpui-component sets the rem to Theme::font_size (root.rs, Root::render set_rem_size), which this connector fills from the platform font. So it scales with font.size and cannot be set apart from it: button_text_size has no Size::Size arm"),
                                ])),
                    )
                    .child(
                        div()
                            .id("tt-btn-ghost")
                            .child(refined(
                                Button::new("b-ghost")
                                    .label("Ghost")
                                    .custom(variants::ghost_button(cx)),
                                button_style.as_ref(),
                            ))
                            .on_hover(self.hover_info(&fi, "Button (Ghost)", &[("text", "secondary_foreground", t.secondary_foreground, "native-theme-gpui/variants.rs:54"), ("hover bg", "secondary_hover", t.secondary_hover, "native-theme-gpui/variants.rs:55"), ("active bg", "secondary_active", t.secondary_active, "native-theme-gpui/variants.rs:56")], &[("geometry", "geometry::button: button.min_height/min_width, border.padding_*, corner_radius, line_width, color (spec §9.2)".to_string()), ("font-weight", "geometry::button carries button.font.weight. The label is a child that sets its own size from the Size enum (sizing.rs, button_text_size) and would overrule a size from here, but it sets no weight and neither does anything else on that path, so the platform's weight cascades (button/button.rs, Button::render)".to_string())], &[("edge", "button.border.color at rest: geometry::button gives every variant a border, and upstream's hover and press styles repaint it transparent, so it vanishes under the pointer -- gpui applies a hover style after the refinement (gpui-pre/elements/div.rs, hover_style)"), 
                                    ("variant", "native_theme_gpui::variants::ghost_button: flat like gpui-component's .ghost(), but with the platform's button.hover_background / active_background. Upstream's own .ghost() would hover with the item-highlight pair (button/button.rs, ButtonVariant::hovered Ghost arm), which is the menu selection colour, not a button hover"),
                                    ])),
                    )
                    .child(
                        div()
                            .id("tt-btn-link")
                            .child(refined(
                                Button::new("b-link").label("Link").link(),
                                button_style.as_ref(),
                            ))
                            .on_hover(self.hover_info(&fi, "Button (Link)", &[("text", "link", t.link, "button/button.rs:993"), ("hover text", "link_hover", t.link_hover, "button/button.rs:1139"), ("pressed text", "link_active", t.link_active, "button/button.rs:1215")], &[("geometry", "geometry::button: button.min_height/min_width, border.padding_*, corner_radius, line_width, color (spec §9.2)".to_string()), ("font-weight", "geometry::button carries button.font.weight. The label is a child that sets its own size from the Size enum (sizing.rs, button_text_size) and would overrule a size from here, but it sets no weight and neither does anything else on that path, so the platform's weight cascades (button/button.rs, Button::render)".to_string())], &[("edge", "button.border.color at rest: geometry::button gives every variant a border, and upstream's hover and press styles repaint it transparent, so it vanishes under the pointer -- gpui applies a hover style after the refinement (gpui-pre/elements/div.rs, hover_style)"),  ("fill", "transparent in every state (button/button.rs, ButtonVariant::bg_color, hovered and active)"), ("underline", "always on for this variant (button/button.rs, ButtonVariant::underline)"), ])),
                    )
                    .child(
                        div()
                            .id("tt-btn-text")
                            .child(refined(
                                Button::new("b-text").label("Text").text(),
                                button_style.as_ref(),
                            ))
                            .on_hover(self.hover_info(&fi, "Button (Text)", &[("text", "foreground", t.foreground, "button/button.rs:994"), ("hover text", "foreground", t.foreground, "button/button.rs:1140"), ("pressed text", "foreground", t.foreground, "button/button.rs:1216")], &[("geometry", "geometry::button: button.min_height/min_width, border.padding_*, corner_radius, line_width, color (spec §9.2)".to_string()), ("font-weight", "geometry::button carries button.font.weight. The label is a child that sets its own size from the Size enum (sizing.rs, button_text_size) and would overrule a size from here, but it sets no weight and neither does anything else on that path, so the platform's weight cascades (button/button.rs, Button::render)".to_string())], &[("edge", "button.border.color at rest: geometry::button gives every variant a border, and upstream's hover and press styles repaint it transparent, so it vanishes under the pointer -- gpui applies a hover style after the refinement (gpui-pre/elements/div.rs, hover_style)"),  ("opacity", "the one variant that dims rather than recolours: foreground at 90% idle and 70% pressed, full strength on hover (button/button.rs, ButtonVariant::text_color, hovered, active). The swatches show foreground itself, since the model states no dimmed copy"), ("fill", "transparent in every state (button/button.rs, ButtonVariant::bg_color, hovered and active)"), ])),
                    )
                    .child(
                        div()
                            .id("tt-btn-outline")
                            .child(refined(
                                Button::new("b-outline")
                                    .label("Outline")
                                    .primary()
                                    .outline(),
                                button_style.as_ref(),
                            ))
                            .on_hover(self.hover_info(&fi, "Button (Primary Outline)", &[("fill", "primary", t.primary, "gpui-component/button/button.rs:871"), ("border, hovered or pressed", "primary", t.primary, "gpui-component/button/button.rs:1003"), ("text", "primary", t.primary, "gpui-component/button/button.rs:952"), ("hover bg", "primary_hover", t.primary_hover, "gpui-component/button/button.rs:874"), ("active bg", "primary_active", t.primary_active, "gpui-component/button/button.rs:877")], &[("geometry", "geometry::button: button.min_height/min_width, border.padding_*, corner_radius, line_width, color (spec §9.2)".to_string()), ("font-weight", "geometry::button carries button.font.weight. The label is a child that sets its own size from the Size enum (sizing.rs, button_text_size) and would overrule a size from here, but it sets no weight and neither does anything else on that path, so the platform's weight cascades (button/button.rs, Button::render)".to_string())], &[("edge", "button.border.color at rest: geometry::button gives every variant a border, and upstream's hover and press styles repaint it primary -- gpui applies a hover style after the refinement (gpui-pre/elements/div.rs, hover_style)"), 
                                    ("outline fill opacity", "0.1 at rest, 0.2 hovered, 0.4 pressed -- three literals, so the platform sets the hue and gpui-component sets how far it is faded (button/button.rs, ButtonVariant::outline_background)"),
                                ])),
                    ),
            )
            // Button sizes: no variant method, so these are ButtonVariant::Default
            // and the panel's `button` pair is what paints them.
            .child(section("Button Sizes"))
            .child(
                div()
                    .id("tt-btn-sizes")
                    .child(
                        h_flex()
                            .gap_2()
                            .items_end()
                            .child(Button::new("s-xs").label("XSmall").with_size(Size::XSmall))
                            .child(Button::new("s-sm").label("Small").with_size(Size::Small))
                            .child(Button::new("s-md").label("Medium").with_size(Size::Medium))
                            .child(Button::new("s-lg").label("Large").with_size(Size::Large)),
                    )
                    .on_hover(self.hover_info(&fi, "Button Sizes", &[("bg", "button", t.button, "gpui-component/button/button.rs:935"), ("text", "button_foreground", t.button_foreground, "gpui-component/button/button.rs:949"), ("hover", "button_hover", t.button_hover, "gpui-component/button/button.rs:1079"), ("active", "button_active", t.button_active, "gpui-component/button/button.rs:1163")], &[("border-radius", format!("radius: {}px", t.radius.as_f32()))], &[
                            ("variant", "these take no variant, so they are ButtonVariant::Default -- the button family, not the secondary one the panel had named (button/button.rs)"),
                            ("size", "XSmall/Small/Medium/Large via Size enum"),
                            ("padding", "per Size only because this demo omits the refinement the other Button panels apply: upstream takes a copy of the caller's style before the Size arm sets its px_1/px_2/px_3 and re-applies that copy afterwards (button/button.rs, Button), so geometry::button's border.padding_* would win. Left bare on purpose -- this is the panel that shows the enum"),
                            ("min-height", "the same: the Size arm's h_5/h_6/h_8 is overruled by a refinement, so button.min_height would arrive through geometry::button (button/button.rs, Button)"),
                        ])),
            )
            // Button group
            .child(section("ButtonGroup"))
            .child(
                div()
                    .id("tt-btn-group")
                    .child(
                        ButtonGroup::new("bg-1")
                            .child(Button::new("bg-a").label("Left"))
                            .child(Button::new("bg-b").label("Center"))
                            .child(Button::new("bg-c").label("Right")),
                    )
                    .on_hover(self.hover_info(&fi, "ButtonGroup", &[("bg", "button", t.button, "gpui-component/button/button.rs:935"), ("text", "button_foreground", t.button_foreground, "gpui-component/button/button.rs:949"), ("hover", "button_hover", t.button_hover, "gpui-component/button/button.rs:1079"), ("active", "button_active", t.button_active, "gpui-component/button/button.rs:1163"), ("border", "input", t.input, "gpui-component/button/button.rs:1001")], &[("border-radius", format!("radius: {}px", t.radius.as_f32()))], &[("variant", "no variant given, so Default: its edge is input, not border"),
                            ("gap", "no gap to set: a ButtonGroup joins its buttons by turning edges off rather than by spacing them (button/button_group.rs, ButtonGroup)")])),
            )
            // Disabled + loading
            .child(section("Disabled State"))
            .child(
                div()
                    .id("tt-btn-disabled")
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Button::new("d-pri").native(cx, geometry::button)
                                    .label("Disabled Primary")
                                    .primary()
                                    .disabled(true),
                            )
                            .child(
                                Button::new("d-sec").native(cx, geometry::button)
                                    .label("Disabled Secondary")
                                    .disabled(true),
                            )
                            .child(
                                Button::new("d-dng").native(cx, geometry::button)
                                    .label("Disabled Danger")
                                    .danger()
                                    .disabled(true),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Disabled Buttons",
                        &[],
                        &[],
                        &[
                            ("fill", "the variant's own token at 0.15, a literal -- not 0.5, and not a disabled token: the model carries button.disabled_background and the platform states one, and upstream reads neither (button/button.rs, ButtonVariant::disabled)"),
                            ("text", "muted_foreground at 0.5, so a disabled button does not keep its variant's text colour. button.disabled_text_color is modelled and carried, and upstream reads it nowhere (button/button.rs, ButtonVariant::disabled)"),
                            ("opacity", "button.disabled_opacity is modelled and inherits defaults.disabled_opacity; upstream multiplies its own literals instead, so the platform's figure has no receiver -- Tier U, not an absence"),
                            ("cursor", "the default arrow, not not-allowed: Button sets cursor_default, and only a link or text variant asks for a pointer (button/button.rs, Button::render)"),
                        ],
                    )),
            )
            .child(section("Loading State"))
            .child(
                div()
                    .id("tt-btn-loading")
                    .child(
                        h_flex().gap_2().child(
                            // `Button` draws its spinner in place of its icon
                            // (`button/button_icon.rs`), so a loading button
                            // needs one to show it; this one is never seen
                            // while `loading` is true.
                            Button::new("l-pri").native(cx, geometry::button)
                                .label("Loading...")
                                .primary()
                                .icon(IconName::Check)
                                .loading(true),
                        ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Loading Button",
                        &[],
                        &[],
                        &[
                            ("spinner", "stands in for the button's icon, so only a button with one shows it -- this one's is set to be replaced. It inherits the button's text colour and turns every 0.8s, a literal (button/button_icon.rs, ButtonIcon; spinner.rs, Spinner::new)"),
                            ("interaction", "inert, as a disabled button is, but not styled as one: it keeps its variant's colours and the whole button fades to 0.8 (button/button.rs, Button::interactive)"),
                        ],
                    )),
            )
            // Buttons with icons
            .child(section("Buttons with Icons"))
            .child(
                div()
                    .id("tt-btn-icons")
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Button::new("bi-save").native(cx, geometry::button)
                                    .label("Save")
                                    .primary()
                                    .icon(IconName::Check),
                            )
                            .child(
                                Button::new("bi-search").native(cx, geometry::button)
                                    .label("Search")
                                    .icon(IconName::Search),
                            )
                            .child(
                                Button::new("bi-del").native(cx, geometry::button)
                                    .label("Delete")
                                    .danger()
                                    .icon(IconName::Delete),
                            ),
                    )
                    .on_hover(self.hover_info(
                        &fi,
                        "Buttons with Icons",
                        &[],
                        &[],
                        &[
                            ("icon position", "leading, always: Button adds its icon before the label and has no setting for the other side (button/button.rs, Button::render)"),
                            ("icon size", "size_3 / size_3p5 / size_4 / size_6 per the button's Size (icon.rs, Icon::into_svg) -- rems again, so it scales with the platform font, while defaults.icon_sizes is in absolute px and Button exposes no icon_size setter to take one"),
                        ],
                    )),
            )
            // DropdownButton
            .child(section("DropdownButton"))
            .child(
                div()
                    .id("tt-dropdown-btn")
                    .child(
                        h_flex()
                            .gap_4()
                            .child(
                                DropdownButton::new("dropdown-1")
                                    .button(Button::new("dropdown-main").label("Save").primary())
                                    .dropdown_menu(|menu, _w, _cx| {
                                        menu.menu("Save as Draft", Box::new(gpui::NoAction))
                                            .separator()
                                            .menu("Export as PDF", Box::new(gpui::NoAction))
                                    }),
                            )
                            .child(
                                DropdownButton::new("dropdown-2")
                                    .button(Button::new("dropdown-sec").label("Actions"))
                                    .dropdown_menu(|menu, _w, _cx| {
                                        menu.menu("Cut", Box::new(gpui::NoAction))
                                            .menu("Copy", Box::new(gpui::NoAction))
                                            .menu("Paste", Box::new(gpui::NoAction))
                                    }),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "DropdownButton", &[("dropdown border", "input", t.input, "gpui-component/button/button.rs:1001"), ("menu bg", "popover", t.popover, "gpui-component/styled.rs:197")], &[], &[("dropdown arrow", "a Caret, not an icon the caller passes: the right half is a Button::dropdown_caret and the glyph comes from select.rs, Caret. The shape is fixed, but its colour is not -- upstream paints it with the button variant's own text colour at 75% (button/button.rs, Button::render dropdown_caret), so it follows the platform through the same token the label does")])),
            )
            // Toggle & ToggleGroup
            .child(section("Toggle & ToggleGroup"))
            .child(
                div()
                    .id("tt-toggle")
                    .child(
                        h_flex()
                            .gap_6()
                            .items_center()
                            .child(
                                Toggle::new("tog-bold")
                                    .icon(IconName::Star)
                                    .checked(self.toggle_bold)
                                    .on_click(cx.listener(|this, checked: &bool, _w, _cx| {
                                        this.toggle_bold = *checked;
                                    })),
                            )
                            .child(
                                Toggle::new("tog-italic")
                                    .icon(IconName::Heart)
                                    .checked(self.toggle_italic)
                                    .on_click(cx.listener(|this, checked: &bool, _w, _cx| {
                                        this.toggle_italic = *checked;
                                    })),
                            )
                            .child(
                                ToggleGroup::new("tog-group-1")
                                    .child(Toggle::new("tg-left").label("Left"))
                                    .child(Toggle::new("tg-center").label("Center"))
                                    .child(Toggle::new("tg-right").label("Right")),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Toggle / ToggleGroup", &[("checked bg", "accent", t.accent, "button/toggle.rs:155"), ("checked text", "accent_foreground", t.accent_foreground, "button/toggle.rs:156"), ("hover bg", "accent", t.accent, "button/toggle.rs:202"), ("hover text", "accent_foreground", t.accent_foreground, "button/toggle.rs:203")], &[("border-radius", format!("radius: {}px", t.radius.as_f32()))], &[("unchecked fill", "none: ToggleVariant defaults to Ghost (button/toggle.rs, ToggleVariant), which paints no background and no border; only .outline() fills"), ("checked fill", "accent, the menu highlight, by default -- but not out of reach: a Toggle folds the caller's refinement into its checked style too (button/toggle.rs, Toggle::render), so an application that refines a checked Toggle with segmented_control.active_background and active_text_color gets them. Only an unchecked one's hover is Tier U. Nothing applies them: there is no geometry::toggle -- our gap"), ("size", "min_w_8 / h_8 at the default Size -- rems, so the platform's font -- and settable: the refinement comes last, so segmented_control.segment_height, its padding and its font would reach a Toggle through the geometry::toggle nobody has written (button/toggle.rs, Toggle::render)")])),
            )
            // Clipboard
            .child(section("Clipboard"))
            .child(
                div()
                    .id("tt-clipboard")
                    .child(
                        h_flex()
                            .gap_4()
                            .child(probe(
                                PROBE_CLIPBOARD,
                                Clipboard::new("clip-1").value("cargo add native-theme"),
                            ))
                            .child(Clipboard::new("clip-2").value("npm install native-theme")),
                    )
                    .on_hover(self.hover_info(&fi, "Clipboard", &[("hover", "accent", t.accent, "gpui-component/button/button.rs:1126"), ("icon", "secondary_foreground", t.secondary_foreground, "gpui-component/button/button.rs:964")], &[("border-radius", format!("radius: {}px", t.radius.as_f32()))], &[("surface", "a Clipboard is a ghost Button and reads no theme field of its own (clipboard.rs, Clipboard::render): transparent until hovered, when it takes accent -- the menu highlight, not the button family. Its icon takes the Ghost variant's secondary_foreground"), ("copy icon", "Copy and Check, built inline with no setter to replace them (clipboard.rs, Clipboard)")])),
            )
    }
}
