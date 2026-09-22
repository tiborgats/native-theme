//! The Inputs tab.

use gpui::{
    ClipboardItem, Context, IntoElement, ParentElement, SharedString, Styled, Window, div,
    prelude::*, px,
};
use gpui_component::{
    ActiveTheme, Disableable, Icon, IconName, Sizable, Size, WindowExt,
    button::ButtonVariants,
    checkbox::Checkbox,
    color_picker::ColorPicker,
    h_flex,
    input::{
        Input, InputGroup, InputGroupAddon, InputGroupAddonAlignment, InputGroupButton,
        InputGroupText, InputGroupTextarea, NumberInput, OtpInput, Textarea,
    },
    label::Label,
    notification::Notification,
    radio::{Radio, RadioGroup},
    rating::Rating,
    select::Select,
    slider::Slider,
    switch::Switch,
    v_flex,
};

use native_theme_gpui::{geometry, variants};

use crate::app::Showcase;
use crate::support::{
    NativeStyled, format_font_info, native_geometry, native_value, refined, section, with_gap,
};
use crate::{PROBE_RATING, probe};

impl Showcase {
    // -----------------------------------------------------------------------
    // Tab: Inputs
    // -----------------------------------------------------------------------
    pub(crate) fn render_inputs_tab(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let fi = format_font_info(&self.original_font, &self.original_mono_font);
        let t = cx.theme().clone();
        let widget_gap = geometry::widget_gap(&self.layout);
        let checkbox_a = self.checkbox_a;
        let checkbox_b = self.checkbox_b;
        let checkbox_c = self.checkbox_c;
        let switch_on = self.switch_on;
        let radio_index = self.radio_index;
        let slider_value = self.slider_value;

        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            // Text Input
            .child(section("Text Input"))
            .child(
                div()
                    .id("tt-input")
                    .child(
                        with_gap(v_flex(), widget_gap)
                            .child(refined(
                                Input::new(&self.input_state)
                                    .with_size(Size::Medium)
                                    .w(px(360.0)),
                                native_geometry(cx, geometry::input).as_ref(),
                            ))
                            // The same control height without the rest of the
                            // refinement: what `geometry::input_height` is for
                            // (`Input::h`, input/input.rs:257), and a field
                            // that must line up with the one above without
                            // taking its border or text size.
                            .child({
                                let input = Input::new(&self.input_height_state)
                                    .with_size(Size::Medium)
                                    .w(px(360.0));
                                match native_value(cx, geometry::input_height) {
                                    Some(height) => input.h(height),
                                    None => input,
                                }
                            }),
                    )
                    .on_hover(self.hover_info(&fi, "Input", &[("border", "input", t.input, "gpui-component/input/input.rs:714"), ("bg", "background", t.background, "gpui-component/theme/mod.rs:383"), ("text", "foreground", t.foreground, "gpui-component/input/input.rs:105"), ("disabled text", "muted_foreground", t.muted_foreground, "gpui-component/input/input.rs:102"), ("disabled bg", "input", t.input, "gpui-component/input/input.rs:101"), ("placeholder", "muted_foreground", t.muted_foreground, "gpui-component/input/input.rs:499")], &[
                            ("border-radius", format!("radius: {}px", t.radius.as_f32())),
                            ("focus_ring", format!("{}", t.focus_ring)),
                        ("geometry", "geometry::input: input.min_height (control height), border.corner_radius, line_width, input.font".to_string()), ("second field", "geometry::input_height alone: the same control height, nothing else".to_string())], &[
                            ("fill", "input_background(): the window background in light mode, and input mixed toward transparent in dark -- one accessor, two sources (theme/mod.rs)"),
                            ("focus ring", "the connector uses the platform's focus_ring_width only as a switch: upstream drops the ring where Theme::focus_ring is off (styled.rs, FocusableExt::focus_ring_style), and draws it 3px wide at half the ring colour's alpha where it is on (styled.rs, FOCUS_RING_WIDTH), so the platform's width itself is Tier U. The ring's colour is `ring`, shown on the InputGroup and OtpInput panels"),
                            ("disabled fill", "input mixed toward transparent, not muted as the panel had claimed (input/input.rs, input_style)"),
                            ("placeholder colour", "Tier U: input.placeholder_color is modelled from each platform's own placeholder colour -- inheritance-rules.toml lists falling back to muted_color as wrong -- but Input hands its editor the shared muted_foreground on every render and takes no colour of its own (input/input.rs, Input::render)"),
                            ("padding", "inner editor (Tier U)"),
                        ])),
            )
            // Textarea
            .child(section("Textarea (multi-line, the same input surface)"))
            .child(
                div()
                    .id("tt-textarea")
                    .child(refined(
                        Textarea::new(&self.textarea_demo)
                            .h(px(90.0))
                            .w(px(360.0)),
                        native_geometry(cx, geometry::input).as_ref(),
                    ))
                    .on_hover(self.hover_info(&fi, "Textarea", &[("bg", "background", t.background, "gpui-component/theme/mod.rs:383"), ("text", "foreground", t.foreground, "gpui-component/input/input.rs:105"), ("border", "input", t.input, "gpui-component/input/input.rs:714"), ("focus ring", "ring", t.ring, "gpui-component/input/input.rs:681"), ("disabled text", "muted_foreground", t.muted_foreground, "gpui-component/input/input.rs:102")], &[
                        ("geometry", "geometry::input: input.min_height as the control height, border.corner_radius, border.line_width, input.font -- the same refinement the single-line Input above takes, because a Textarea renders as one (input/textarea.rs, Textarea::into_input)".to_string())], &[
                        ("row height", "1.25rem, the line height Input sets for every row (input/input.rs, Input::render), not the resolved font's. Input applies the caller's refinement after it, and defaults.line_height is modelled, but no builder carries it -- our gap"),
                    ])),
            )
            // InputGroup
            .child(section("InputGroup"))
            .child(
                div()
                    .id("tt-input-group")
                    .child(
                        v_flex()
                            .gap_3()
                            .w(px(360.0))
                            .child(refined(
                                InputGroup::new("input-group-inline")
                                    .input(Input::new(&self.input_group_state))
                                    .addon(
                                        InputGroupAddon::new("input-group-inline-addon")
                                            .child(Icon::new(IconName::Search)),
                                    ),
                                native_geometry(cx, geometry::input).as_ref(),
                            ))
                            .child(refined(
                                InputGroup::new("input-group-trailing")
                                    .input(Input::new(&self.input_group_button_state))
                                    .addon(
                                        InputGroupAddon::new("input-group-trailing-addon")
                                            .align(InputGroupAddonAlignment::InlineEnd)
                                            .child(
                                                // `InputGroupButton::new` is a ghost
                                                // button that upstream repaints, inside
                                                // a group, with a hover of `theme.muted`
                                                // (`input/group.rs:544-583`): a grey that
                                                // under Breeze is barely distinguishable
                                                // from the field, while every button
                                                // around it hovers blue. A custom variant
                                                // makes upstream skip that repaint.
                                                InputGroupButton::new("input-group-copy")
                                                    .native(cx, geometry::input_group_button)
                                                    .custom(variants::ghost_button(cx))
                                                    .icon(IconName::Copy)
                                                    .label("Copy")
                                                    .tooltip("Copy the field to the clipboard")
                                                    .on_click(cx.listener(
                                                        |this, _, window, cx| {
                                                            let value = this
                                                                .input_group_button_state
                                                                .read(cx)
                                                                .value();
                                                            cx.write_to_clipboard(
                                                                ClipboardItem::new_string(
                                                                    value.to_string(),
                                                                ),
                                                            );
                                                            window.push_notification(
                                                                Notification::success(value)
                                                                    .title("Copied")
                                                                    .autohide(true),
                                                                cx,
                                                            );
                                                        },
                                                    )),
                                            ),
                                    ),
                                native_geometry(cx, geometry::input).as_ref(),
                            ))
                            .child(
                                InputGroup::new("input-group-textarea")
                                    .input(InputGroupTextarea::new(
                                        &self.input_group_textarea_state,
                                    ))
                                    .addon(
                                        InputGroupAddon::new("input-group-textarea-addon")
                                            .align(InputGroupAddonAlignment::BlockEnd)
                                            .child(
                                                InputGroupText::new().child("Markdown supported"),
                                            ),
                                    ),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "InputGroup", &[("border", "input", t.input, "gpui-component/input/group.rs:317"), ("focus ring", "ring", t.ring, "gpui-component/input/group.rs:315"), ("addon text", "muted_foreground", t.muted_foreground, "gpui-component/input/group.rs:403"), ("addon button hover", "secondary_hover", t.secondary_hover, "native-theme-gpui/variants.rs:55")], &[("geometry", "geometry::input on the frame: input.min_height (single-line groups only), border.corner_radius, line_width, input.font".to_string()), ("addon radius", "geometry::input_group_button: button.border.corner_radius alone. gpui-component scales a control's radius with its size, so an in-group button would take radius/2 (input/group.rs, InputGroupButton::render_in_group), where the model records one radius per widget".to_string())], &[
                            ("addon padding", "inner (Tier U)"),
                            ("addon button", "native_theme_gpui::variants::ghost_button: flat idle, hover = secondary_hover (the platform's button.hover_background). Upstream's own in-group ghost would hover with muted (input/group.rs, InputGroupButton::render_in_group)"),
                        ])),
            )
            // Number Input
            .child(section("Number Input"))
            .child(
                div()
                    .id("tt-number-input")
                    .child(
                        NumberInput::new(&self.number_input_state)
                            .native(cx, geometry::input)
                            .placeholder("Enter a number")
                            .with_size(Size::Medium)
                            .w(px(200.0)),
                    )
                    .on_hover(self.hover_info(&fi, "NumberInput", &[("border", "input", t.input, "gpui-component/input/number_input.rs:118"), ("bg", "background", t.background, "gpui-component/theme/mod.rs:383"), ("text", "foreground", t.foreground, "gpui-component/input/input.rs:105"), ("disabled text", "muted_foreground", t.muted_foreground, "gpui-component/input/input.rs:102"), ("disabled bg", "input", t.input, "gpui-component/input/input.rs:101")], &[
                            ("geometry", "geometry::input on the field: input.min_height (control height), border.corner_radius, line_width, input.font -- the same builder the Input above takes".to_string())], &[
                            ("padding", "inner editor (Tier U), as Input"),
                            ("step buttons", "hardcoded +/- icons; the Size enum sets their min width (input/number_input.rs, NumberInput::render: min_w_6 / min_w_8), not the field's height"),
                        ])),
            )
            // Checkboxes
            .child(section("Checkboxes"))
            .child(
                div()
                    .id("tt-checkbox")
                    .child(
                        v_flex()
                            .gap_3()
                            .child(
                                refined(
                                    Checkbox::new("cb-a"),
                                    native_geometry(cx, geometry::checkbox).as_ref(),
                                )
                                .label("Enable notifications")
                                .checked(checkbox_a)
                                .on_click(cx.listener(
                                    |this, val: &bool, _w, _cx| {
                                        this.checkbox_a = *val;
                                    },
                                )),
                            )
                            .child(
                                refined(
                                    Checkbox::new("cb-b"),
                                    native_geometry(cx, geometry::checkbox).as_ref(),
                                )
                                .label("Auto-save drafts")
                                .checked(checkbox_b)
                                .on_click(cx.listener(
                                    |this, val: &bool, _w, _cx| {
                                        this.checkbox_b = *val;
                                    },
                                )),
                            )
                            .child(
                                refined(
                                    Checkbox::new("cb-c"),
                                    native_geometry(cx, geometry::checkbox).as_ref(),
                                )
                                .label("Disabled checkbox")
                                .checked(checkbox_c)
                                .disabled(true),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Checkbox", &[("checked bg", "primary", t.primary, "gpui-component/checkbox.rs:307"), ("checkmark", "primary_foreground", t.primary_foreground, "gpui-component/checkbox.rs:195"), ("unchecked border", "input", t.input, "gpui-component/checkbox.rs:238"), ("unchecked fill", "background", t.background, "gpui-component/theme/mod.rs:383")], &[
                            ("indicator radius", format!("radius, capped at 4px: {}px", t.radius.as_f32().min(4.0))),
                        ("geometry", "geometry::checkbox: checkbox.label_gap, checkbox.font".to_string())], &[
                            ("font colour", "carried as size and weight only. Upstream wraps a Checkbox label in a div that sets foreground itself and re-sets muted_foreground there when disabled (checkbox.rs, Checkbox::render), and the disabled hook applies muted_foreground before this refinement, so a carried colour would never reach the label and would displace the disabled colour of custom children (native-theme-gpui geometry.rs, geometry::checkbox)"),
                            ("unchecked fill", "input_background(), as an Input's: the window background in light mode, input faded toward transparent in dark (checkbox.rs, Checkbox::render)"),
                            ("indicator size", "rems(0.75 / 0.875 / 1 / 1.125) per Size (checkbox.rs, Checkbox::render indicator_size), so the box already scales with the platform font -- the rem is Theme::font_size. What has no route is checkbox.indicator_width, which the model states in absolute px: Size::Size falls into the same catch-all arm as Medium. Tier U for the px, not for the scaling"),
                        ])),
            )
            // Radio group
            .child(section("Radio Group"))
            .child(
                div()
                    .id("tt-radio")
                    .child(
                        // `RadioGroup` takes `impl Into<Radio>`, so a `&str`
                        // child would build a `Radio` with no refinement;
                        // built here instead, each row carries the platform's
                        // label gap and font. The group overwrites the id
                        // (`radio.rs:406`), not the style.
                        RadioGroup::horizontal("rg-1")
                            .child(
                                Radio::new("rg-a")
                                    .native(cx, geometry::radio)
                                    .label("Option A"),
                            )
                            .child(
                                Radio::new("rg-b")
                                    .native(cx, geometry::radio)
                                    .label("Option B"),
                            )
                            .child(
                                Radio::new("rg-c")
                                    .native(cx, geometry::radio)
                                    .label("Option C"),
                            )
                            .selected_index(radio_index)
                            .on_click(cx.listener(|this, ix: &usize, _w, _cx| {
                                this.radio_index = Some(*ix);
                            })),
                    )
                    .on_hover(self.hover_info(&fi, "Radio", &[("selected fill and border", "primary", t.primary, "gpui-component/radio.rs:186"), ("unselected border", "input", t.input, "gpui-component/radio.rs:188"), ("unselected fill", "background", t.background, "gpui-component/theme/mod.rs:383"), ("upstream label", "foreground", t.foreground, "gpui-component/radio.rs:212")], &[
                            ("geometry", "geometry::radio: checkbox.label_gap, and checkbox.font including its colour, which the label takes because upstream sets foreground on the row and mutes the label child instead (platform-facts §2.5: radio metrics are the checkbox's)".to_string())], &[ ("unselected fill", "input_background(), as an Input's -- not the input at 50% upstream computes and never paints. Disabled halves the border and a checked fill, never an unchecked one (gpui-component radio.rs, Radio::render)"), ("corner radius", "a circle, through radius_full() -- and square where the theme's radius is 0, since radius_full() follows it (gpui-component styled.rs, rounded_full_style). The radius * 0.5 is the row's, which only the focus ring shows (gpui-component radio.rs, Radio::render)"), ("indicator size", "rems per the Size enum, so it follows the root font size rather than the theme's checkbox metrics (gpui-component radio.rs, Radio::render indicator_size)")])),
            )
            // Switch
            .child(section("Switch"))
            .child(
                div()
                    .id("tt-switch")
                    .child(
                        h_flex()
                            .gap_6()
                            .child(
                                Switch::new("sw-feature")
                                    .label("Feature toggle")
                                    .checked(switch_on)
                                    .on_click(cx.listener(|this, val: &bool, _w, _cx| {
                                        this.switch_on = *val;
                                    })),
                            )
                            .child(
                                Switch::new("sw-disabled")
                                    .label("Disabled")
                                    .checked(true)
                                    .disabled(true),
                            ),
                    )
                    .on_hover(self.hover_info(&fi, "Switch", &[("on track", "primary", t.primary, "gpui-component/switch.rs:139"), ("off track", "switch", t.switch, "gpui-component/switch.rs:140"), ("thumb", "switch_thumb", t.switch_thumb, "gpui-component/switch.rs:146"), ("disabled label", "muted_foreground", t.muted_foreground, "gpui-component/switch.rs:147")], &[("border-radius", format!("radius: {}px", t.radius.as_f32()))], &[("on track", "primary by default, and Switch::color replaces it -- a per-instance receiver for the model's switch.checked_background that nothing in the connector feeds, since ThemeColor has no field for it (gpui-component switch.rs, Switch::color). Our gap"), ("disabled", "the track at 50%, never the thumb: gpui multiplies each primitive's alpha rather than fading the subtree as a group, so fading both would let the track show through (gpui-component switch.rs, Switch::render disabled_bg)"), ("size", "Tier U: track and thumb are px literals per Size, on children of the wrapper the refinement lands on (gpui-component switch.rs, Switch::render), while the model states switch.track_width, track_height and thumb_diameter"), ("corner radius", "fully round unless the theme's radius is under 4px, in which case the theme's is used (gpui-component switch.rs, Switch::render radius). switch.track_radius is modelled and has no receiver"), ("animation timing", "reads the theme's spring_move (switch.rs, Switch), the same writable Theme::motion the Accordion and Collapsible use; the connector leaves it at its default because native-theme models no motion")])),
            )
            // Slider
            .child(section(format!("Slider (value: {:.0})", slider_value)))
            .child(
                div()
                    .id("tt-slider")
                    .child(Slider::new(&self.slider_state).w(px(360.0)))
                    .on_hover(self.hover_info(&fi, "Slider", &[("track", "slider_bar", t.slider_bar, "gpui-component/slider.rs:164"), ("thumb", "slider_thumb", t.slider_thumb, "gpui-component/slider.rs:170"), ("text", "foreground", t.foreground, "gpui-component/slider.rs:272")], &[], &[("track height", "Tier U, not an absence: slider.track_height is modelled and carried. Upstream sets h_1p5() on SliderIndicator, a child of the element the refinement lands on (slider.rs, Slider::render)"), ("thumb size", "Tier U, not an absence: slider.thumb_diameter is modelled and carried. The thumb is built by a closure inside the indicator, deeper still than the track (slider.rs, Slider::render)")])),
            )
            // Rating
            .child(section(format!(
                "Rating ({} of 5 stars)",
                self.rating_value
            )))
            .child(
                div()
                    .id("tt-rating")
                    .child(
                        with_gap(h_flex(), widget_gap)
                            .items_center()
                            .child(probe(PROBE_RATING, {
                                // The stars are inline icons, so the platform's
                                // small icon size is what they take; `Rating`
                                // has no geometry builder of its own.
                                let rating = Rating::new("rating-1")
                                    .value(self.rating_value)
                                    .on_click(cx.listener(
                                        |this, value: &usize, _w, cx| {
                                            this.rating_value = *value;
                                            cx.notify();
                                        },
                                    ));
                                match native_value(cx, geometry::icon_size_small) {
                                    Some(size) => rating.with_size(size),
                                    None => rating,
                                }
                            }))
                            .child(
                                Label::new(SharedString::from(format!(
                                    "value: {}",
                                    self.rating_value
                                )))
                                .text_sm()
                                .text_color(t.muted_foreground),
                            )
                            .child({
                                let disabled = Rating::new("rating-disabled").value(2).disabled(true);
                                match native_value(cx, geometry::icon_size_small) {
                                    Some(size) => disabled.with_size(size),
                                    None => disabled,
                                }
                            }),
                    )
                    .on_hover(self.hover_info(&fi, "Rating", &[("active star", "yellow", t.yellow, "gpui-component/rating.rs:120"), ("inactive star", "foreground", t.foreground, "gpui-component/root.rs:596")], &[("star size", "geometry::icon_size_small: defaults.icon_sizes.small".to_string())], &[
                            ("active colour", "cx.theme().yellow unless Rating::color overrides it (rating.rs, Rating::render active_color)"),
                            ("hover preview", "upstream keeps its own hovered value (rating.rs, RaitingState::hovered_value)"),
                            ("inactive colour", "none of its own: Rating colours only a filled or hovered star, so an empty one takes the window's text colour (rating.rs, Rating::render)"),
                        ])),
            )
            // OTP Input
            .child(section("OTP Input (6 digits)"))
            .child(
                div()
                    .id("tt-otp")
                    .child(OtpInput::new(&self.otp_state).groups(2))
                    .on_hover(self.hover_info(&fi, "OtpInput", &[("border", "input", t.input, "gpui-component/input/otp_input.rs:117"), ("focus ring", "ring", t.ring, "gpui-component/input/otp_input.rs:121"), ("box fill", "background", t.background, "gpui-component/theme/mod.rs:383"), ("digit", "foreground", t.foreground, "gpui-component/input/input.rs:105"), ("caret", "caret", t.caret, "gpui-component/input/otp_input.rs:159")], &[("border-radius", format!("radius: {}px", t.radius.as_f32()))], &[("fill", "input_background(), as an Input's, and the digits take the Input's foreground. The secondary_foreground and muted_foreground the panel had named colour a masked asterisk, and this OtpState is not masked (input/otp_input.rs, OtpInput)"),
                            ("digit count", "configurable"), ("groups", "2")])),
            )
            // Select
            .child(section("Select (a Combobox's trigger, with the carried font colour)"))
            .child(
                div()
                    .id("tt-select")
                    .child(refined(
                        Select::new(&self.select_demo)
                            .placeholder("Pick an icon theme…")
                            .w(px(260.0)),
                        native_geometry(cx, geometry::select).as_ref(),
                    ))
                    .on_hover(self.hover_info(&fi, "Select", &[("trigger bg", "background", t.background, "gpui-component/theme/mod.rs:383"), ("upstream trigger text", "foreground", t.foreground, "gpui-component/input/input.rs:105"), ("trigger border", "input", t.input, "gpui-component/select.rs:541"), ("focus ring", "ring", t.ring, "gpui-component/select.rs:548"), ("placeholder", "muted_foreground", t.muted_foreground, "gpui-component/select.rs:445"), ("disabled text", "muted_foreground", t.muted_foreground, "gpui-component/select.rs:478")], &[
                        ("geometry", "geometry::select: combo_box.min_height (control height), min_width, border.corner_radius, combo_box.font -- and, unlike geometry::combobox, the font's colour as well (native-theme-gpui geometry.rs, select)".to_string())], &[
                        ("carried colour", "the one difference from a Combobox, such as the toolbar's preset switch: Select's selected-title child sets its own colour, so a carried colour yields to the disabled colour instead of beating it, and the connector carries it (native-theme-gpui geometry.rs, combobox)"),
                        ("caret", "its colour is themed -- upstream paints it with muted_foreground (select.rs, Caret) -- and its size is not: Caret maps Size::Size into the same arm as Medium (select.rs, Caret::render), so combo_box.arrow_icon_size has no route at all, not even through the Size::Size escape hatch a DataTable row accepts. Tier U for the size"),
                    ])),
            )
            // Color Picker
            .child(section("ColorPicker"))
            .child(
                div()
                    .id("tt-colorpicker")
                    .child(ColorPicker::new(&self.color_picker_state).label("Pick a color"))
                    .on_hover(self.hover_info(&fi, "ColorPicker", &[("popover", "popover", t.popover, "gpui-component/styled.rs:197"), ("featured red", "red", t.red, "gpui-component/color_picker.rs:205"), ("featured red light", "red_light", t.red_light, "gpui-component/color_picker.rs:206"), ("featured blue", "blue", t.blue, "gpui-component/color_picker.rs:207"), ("featured blue light", "blue_light", t.blue_light, "gpui-component/color_picker.rs:208"), ("featured green", "green", t.green, "gpui-component/color_picker.rs:209"), ("featured green light", "green_light", t.green_light, "gpui-component/color_picker.rs:210"), ("featured yellow", "yellow", t.yellow, "gpui-component/color_picker.rs:211"), ("featured yellow light", "yellow_light", t.yellow_light, "gpui-component/color_picker.rs:212"), ("featured cyan", "cyan", t.cyan, "gpui-component/color_picker.rs:213"), ("featured cyan light", "cyan_light", t.cyan_light, "gpui-component/color_picker.rs:214"), ("featured magenta", "magenta", t.magenta, "gpui-component/color_picker.rs:215"), ("featured magenta light", "magenta_light", t.magenta_light, "gpui-component/color_picker.rs:216")], &[("border-radius", format!("radius: {}px", t.radius.as_f32()))], &[("swatch", "the picked colour itself, edged with it darkened by 30%: background and input frame only an empty swatch, and this one starts with a value (color_picker.rs, ColorPickerButton)"), ("palette grid", "the rows under the featured swatches are gpui-component's own colour scales, not the platform's (color_picker.rs, color_palettes)")])),
            )
            // Date Picker
            .child(section("DatePicker"))
            .child(
                div()
                    .id("tt-datepicker")
                    .child(
                        gpui_component::date_picker::DatePicker::new(&self.date_picker_state)
                            .placeholder("Select a date"),
                    )
                    .on_hover(self.hover_info(&fi, "DatePicker", &[("border", "input", t.input, "gpui-component/time/date_picker.rs:443"), ("popover", "popover", t.popover, "gpui-component/styled.rs:197"), ("selected day", "primary", t.primary, "gpui-component/time/calendar.rs:185")], &[("border-radius", format!("radius: {}px", t.radius.as_f32()))], &[
                            ("calendar icon", "an IconName::Calendar built inline with no setter (time/date_picker.rs, DatePicker)"),
                            ("format", "%Y/%m/%d unless the application sets another, whatever the locale (time/date_picker.rs, DatePickerState::date_format)"),
                        ])),
            )
            // Calendar
            .child(section("Calendar"))
            .child(
                div()
                    .id("tt-calendar")
                    .child(gpui_component::calendar::Calendar::new(
                        &self.calendar_state,
                    ))
                    .on_hover(self.hover_info(&fi, "Calendar", &[("border", "border", t.border, "gpui-component/time/calendar.rs:195"), ("selected day", "primary", t.primary, "gpui-component/time/calendar.rs:185"), ("today", "accent", t.accent, "gpui-component/time/calendar.rs:189"), ("text", "foreground", t.foreground, "gpui-component/time/calendar.rs:180")], &[("border-radius", format!("radius_lg: {}px", t.radius_lg.as_f32()))], &[("fill", "none: a Calendar sets an edge, a radius and a padding but no background, so the window shows through (time/calendar.rs, Calendar)"), ("month navigation", "ChevronLeft and ChevronRight built inline with no setter (time/calendar.rs, Calendar)")])),
            )
    }
}
