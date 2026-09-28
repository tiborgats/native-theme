//! What the Inputs page's widgets report about themselves (spec §3.4).

use gpui_component::theme::Theme;
use native_theme_gpui::ResolvedTheme;

use super::{WidgetInfo, chrome::input_background, claim, stated};
use crate::demo::InputField;

/// A single-line `Input` taking the refinement `field` names. `styled` is
/// whether a native theme is installed, so whether `geometry::input` refined
/// a `Refined` field and `geometry::input_height` sized a `HeightOnly` one.
/// Its geometry line is recorded where `demo::text_input` applies the
/// builder.
pub fn input(t: &Theme, field: InputField, styled: bool) -> WidgetInfo {
    let info = WidgetInfo::new("Input");
    let info = match field {
        InputField::Refined => info,
        InputField::HeightOnly => info.variant("control height only"),
    };
    // geometry::input_fill gives a refined field the platform's fill, in
    // place of upstream's (input/input.rs:711, then :719); its geometry line
    // names the fields.
    let filled = field == InputField::Refined && styled;
    let info = info.color(claim(
        "border",
        "input",
        t.input,
        "gpui-component/input/input.rs:714",
    ));
    let info = if filled {
        info
    } else {
        info.color(input_background(t))
    };
    let info = info
        // Input takes only the fill from input_style and drops its foreground
        // (input/input.rs:639), so the text is the colour the showcase sets
        // on its window.
        .color(claim(
            "text, inherited",
            "foreground",
            t.foreground,
            "showcase",
        ))
        .color(claim(
            "placeholder",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/input/input.rs:499",
        ));
    // geometry::input carries border.corner_radius, which the refinement
    // applies after upstream's own (input/input.rs:712, then :719).
    let info = if field == InputField::HeightOnly || !styled {
        info.config("border-radius", format!("radius: {}px", t.radius.as_f32()))
    } else {
        info
    };
    let info = info.config("focus_ring", format!("{}", t.focus_ring));
    let info = if filled {
        info
    } else {
        info.not_themeable("fill", "input_background(): the window background in light mode, and input mixed toward transparent in dark -- one accessor, two sources (theme/mod.rs); geometry::input_fill gives a refined field the platform's")
    };
    let info = info
        .not_themeable("text colour", "none of its own: Input takes only the fill from input_style and drops its foreground (input/input.rs, Input::render), so the text takes the colour the showcase sets on its window")
        .not_themeable("focus ring", "the connector uses the platform's focus_ring_width only as a switch: upstream drops the ring where Theme::focus_ring is off (styled.rs, FocusableExt::focus_ring_style), and draws it 3px wide at half the ring colour's alpha where it is on (styled.rs, FOCUS_RING_WIDTH), so the platform's width itself is Tier U. The ring is `ring` at that half alpha, shown on the InputGroup's info; the OtpInput and Select show `ring` only as their focused border");
    let info = if filled {
        info
    } else {
        info.not_themeable("disabled fill", "input_style's Oklab mix of 80% input and 20% transparent, then faded to half alpha, because a disabled Input fades its fill again (input/input.rs, Input::render) -- a literal pair, and not muted")
    };
    let info = info
        .not_themeable("disabled text colour", "the window's text colour at half alpha: the editor dims its own text when disabled, in its paint preparation (gpui-base/input/base/element.rs, prepaint), so input.disabled_text_color has no route")
        .not_themeable("placeholder colour", "Tier U: input.placeholder_color is modelled from each platform's own placeholder colour -- inheritance-rules.toml lists falling back to muted_color as wrong -- but Input hands its editor the shared muted_foreground on every render and takes no colour of its own (input/input.rs, Input::render)");
    match field {
        InputField::Refined if styled => info,
        InputField::Refined => info.instance(
            "padding",
            "upstream's own input_px and input_py for Size::Medium (input/input.rs, Input::render): no native theme is installed, so geometry::input has no padding to give it",
        ),
        InputField::HeightOnly if styled => info
            .instance(
                "height",
                "the height rule of the field above without the rest of its refinement: what geometry::input_height is for. At a text scale of 1 or less both are input.min_height tall; above 1 each grows around its own text and padding, and this one's are upstream's text_sm and input_py for Size::Medium (input/input.rs, Input::render), not the platform's, so it no longer lines up with the field above",
            )
            .instance(
                "padding",
                "upstream's own input_px and input_py for Size::Medium (input/input.rs, Input::render): geometry::input_height carries no padding",
            ),
        InputField::HeightOnly => info.instance(
            "height",
            "upstream's own for Size::Medium: no native theme is installed, so geometry::input_height has no height to give it",
        ),
    }
}

/// A `Textarea`, `filled` by `geometry::input_fill` or not. Its geometry
/// lines are recorded where `demo::textarea` applies the builders.
pub fn textarea(t: &Theme, filled: bool) -> WidgetInfo {
    let info = WidgetInfo::new("Textarea");
    let info = if filled {
        info
    } else {
        info.color(input_background(t))
    };
    info
        // A Textarea renders as an Input (input/textarea.rs:164), which
        // drops input_style's foreground (input/input.rs:639).
        .color(claim(
            "text, inherited",
            "foreground",
            t.foreground,
            "showcase",
        ))
        .color(claim(
            "border",
            "input",
            t.input,
            "gpui-component/input/input.rs:714",
        ))
        .color(claim(
            "focused border",
            "ring",
            t.ring,
            "gpui-component/input/input.rs:681",
        ))
        .instance("row height", "defaults.line_height times the text size: geometry::input carries the platform's line height, which Input applies after its own 1.25rem (input/input.rs, Input::render: line_height then refine_style), and each row is the window's line height (gpui-base input/base/element.rs, TextElement::request_layout)")
        .instance("height", "the showcase's own 90px, set after geometry::input: the builder's height rule is for a single-line field, and a Textarea's own height goes on after it, where it wins")
        .instance("padding", "none from geometry::input: the showcase clears the refinement's padding sides, because they are a single-line field's -- upstream pads only a single-line Input's root (input/input.rs, Input::render: input_px and input_py when not multi-line)")
        .instance("refinement", "geometry::input, the one the single-line Input above takes, because a Textarea renders as one (input/textarea.rs, Textarea::into_input)")
}

/// A `Textarea` as tall as the rows its state shows: [`textarea`]'s, with
/// the height its rows give it.
pub fn rows_textarea(t: &Theme, filled: bool) -> WidgetInfo {
    let mut info = textarea(t, filled);
    info.instance.retain(|note| note.what != "height");
    info.instance("height", "the rows its state was built to show, three, each the row height above: the showcase clears the single-line height geometry::input sets, so the state's rows size it (gpui-base input/base/state.rs, InputBaseState::auto_grow)")
}

/// The three `InputGroup`s, which report as one. Their geometry lines are
/// recorded where `demo::input_groups` applies the builders.
pub fn input_groups(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("InputGroup")
        .color(claim(
            "border",
            "input",
            t.input,
            "gpui-component/input/group.rs:317",
        ))
        .color(claim(
            "focused border",
            "ring",
            t.ring,
            "gpui-component/input/group.rs:315",
        ))
        .color(claim(
            "focus ring, ring at 50%",
            "ring",
            t.ring.opacity(0.5),
            "gpui-component/input/group.rs:315",
        ))
        .color(claim(
            "addon text",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/input/group.rs:403",
        ))
        .color(claim(
            "addon button hover",
            "secondary_hover",
            t.secondary_hover,
            "native-theme-gpui/variants.rs:55",
        ))
        .not_themeable("addon padding", "inner (Tier U)")
        .not_themeable("addon button", "native_theme_gpui::variants::ghost_button: flat idle, hover = secondary_hover (the platform's button.hover_background). Upstream's own in-group ghost would hover with muted (input/group.rs, InputGroupButton::render_in_group)")
        .instance("groups", "a Search icon of the chosen icon theme before the field, a Copy button after it, and a note under a textarea. geometry::input refines the two single-line groups; the textarea group keeps upstream's frame")
        .instance("padding", "none from geometry::input: the showcase leaves out its padding sides, because the refinement lands on the group's frame and the Input inside it keeps its own padding (input/group.rs, render_control) -- padding the frame too would inset the field twice")
        .instance("copy", "puts the second group's text on the clipboard and says so in a notification")
}

/// A `NumberInput`. Its geometry line is recorded where `demo::number_input`
/// applies the builder.
pub fn number_input(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("NumberInput")
        .color(claim(
            "border",
            "input",
            t.input,
            "gpui-component/input/number_input.rs:118",
        ))
        .color(input_background(t))
        // The digits are the inner Input's, which drops input_style's
        // foreground (input/input.rs:639).
        .color(claim(
            "text, inherited",
            "foreground",
            t.foreground,
            "showcase",
        ))
        .instance("padding", "none from geometry::input: the showcase leaves out its padding sides, because the refinement lands on the frame round the buttons (input/number_input.rs, NumberInput::render) and the Input inside it keeps upstream's own padding")
        .not_themeable("step buttons", "hardcoded +/- icons; the Size enum sets their min width (input/number_input.rs, NumberInput::render: min_w_6 / min_w_8), not the field's height")
        .not_themeable("own icons", super::own_icons("Minus and Plus on its step buttons (input/number_input.rs, NumberInput)"))
}

/// A `Checkbox` reading `label`, `checked` or not, `disabled` or not. Its
/// geometry line is recorded where `demo::checkbox` applies the builder.
pub fn checkbox(t: &Theme, label: &'static str, checked: bool, disabled: bool) -> WidgetInfo {
    let info = WidgetInfo::new("Checkbox")
        .variant(match (checked, disabled) {
            (true, false) => "checked",
            (false, false) => "unchecked",
            (true, true) => "checked, disabled",
            (false, true) => "unchecked, disabled",
        })
        .not_themeable(
            "own icons",
            super::own_icons("the check mark, Check, drawn while it is checked (checkbox.rs, checkbox_check_icon)"),
        );
    // The disabled style is resolved after the checked one, so it wins
    // (gpui-base checkbox.rs:302-316).
    let info = match (checked, disabled) {
        (true, false) => info
            .color(claim(
                "checked bg",
                "primary",
                t.primary,
                "gpui-component/checkbox.rs:307",
            ))
            .color(claim(
                "checked border",
                "primary",
                t.primary,
                "gpui-component/checkbox.rs:239",
            ))
            .color(claim(
                "checkmark",
                "primary_foreground",
                t.primary_foreground,
                "gpui-component/checkbox.rs:195",
            )),
        (false, false) => info.color(input_background(t)).color(claim(
            "unchecked border",
            "input",
            t.input,
            "gpui-component/checkbox.rs:238",
        )),
        (true, true) => info
            .color(claim(
                "checked bg and border, at 50%",
                "primary",
                t.primary.opacity(0.5),
                "gpui-component/checkbox.rs:239-241",
            ))
            .color(claim(
                "checkmark, at 50%",
                "primary_foreground",
                t.primary_foreground.opacity(0.5),
                "gpui-component/checkbox.rs:193",
            )),
        (false, true) => info.color(input_background(t)).color(claim(
            "unchecked border, at 50%",
            "input",
            t.input.opacity(0.5),
            "gpui-component/checkbox.rs:238-243",
        )),
    };
    let info = if disabled {
        info.color(claim(
            "label",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/checkbox.rs:338",
        ))
    } else {
        info.color(claim(
            "label",
            "foreground",
            t.foreground,
            "gpui-component/checkbox.rs:336",
        ))
    };
    let info = info
        .config("indicator radius", format!("radius, capped at 4px: {}px", t.radius.as_f32().min(4.0)))
        .not_themeable("font colour", "carried as size and weight only. Upstream wraps a Checkbox label in a div that sets foreground itself and re-sets muted_foreground there when disabled (checkbox.rs, Checkbox::render), and the disabled hook applies muted_foreground before this refinement, so a carried colour would never reach the label and would displace the disabled colour of custom children (native-theme-gpui geometry.rs, geometry::checkbox)")
        .not_themeable("unchecked fill", "input_background(), as an Input's: the window background in light mode, input faded toward transparent in dark (checkbox.rs, Checkbox::render)")
        .not_themeable("indicator size", "rems(0.75 / 0.875 / 1 / 1.125) per Size (checkbox.rs, Checkbox::render indicator_size), so the box already scales with the platform font -- the rem is Theme::font_size. What has no route is checkbox.indicator_width, which the model states in absolute px: Size::Size falls into the same catch-all arm as Medium. Tier U for the px, not for the scaling")
        .instance("label", label);
    if disabled {
        info.instance(
            "click",
            "none: it is disabled, and the showcase gives it no handler",
        )
    } else {
        info.instance(
            "click",
            "checks or unchecks it; the showcase keeps the state",
        )
    }
}

/// A horizontal `RadioGroup` of Radios reading `labels`, the one at
/// `selected` selected. Its geometry line is recorded where
/// `demo::radio_group` applies the builder.
pub fn radio_group(t: &Theme, labels: &[&'static str], selected: Option<usize>) -> WidgetInfo {
    let selected = selected
        .and_then(|ix| labels.get(ix).copied())
        .unwrap_or("none");
    WidgetInfo::new("RadioGroup")
        .variant("horizontal")
        .color(claim(
            "selected fill and border",
            "primary",
            t.primary,
            "gpui-component/radio.rs:186",
        ))
        .color(claim(
            "unselected border",
            "input",
            t.input,
            "gpui-component/radio.rs:188",
        ))
        // The unselected Radios' fill: Radio::render takes
        // input_background() for an unchecked one (radio.rs:238).
        .color(input_background(t))
        .color(claim(
            "upstream label",
            "foreground",
            t.foreground,
            "gpui-component/radio.rs:212",
        ))
        .not_themeable("unselected fill", "input_background(), as an Input's -- not the input at 50% upstream computes and never paints. Disabled halves the border and a checked fill, never an unchecked one (gpui-component radio.rs, Radio::render)")
        .not_themeable("corner radius", "a circle, through radius_full() -- and square where the theme's radius is 0, since radius_full() follows it (gpui-component styled.rs, rounded_full_style). The radius * 0.5 is the row's, which only the focus ring shows (gpui-component radio.rs, Radio::render)")
        .not_themeable("indicator size", "rems per the Size enum, so it follows the root font size rather than the theme's checkbox metrics (gpui-component radio.rs, Radio::render indicator_size)")
        .instance("radios", format!("{}. RadioGroup::child takes an Into<Radio>, not an element a target could wrap, so they report through the group (radio.rs, RadioGroup::child)", labels.join(", ")))
        .instance("selected", selected)
}

/// A column of Radios reading `labels`, the one at `selected` selected,
/// built Radio by Radio: what a `RadioGroup` paints on each (`radio_group`),
/// in the showcase's own column. Its geometry lines are recorded where
/// `demo::radio_column` applies the builders.
pub fn radio_column(t: &Theme, labels: &[&'static str], selected: Option<usize>) -> WidgetInfo {
    let mut info = radio_group(t, labels, selected).variant("vertical");
    info.instance.retain(|note| note.what != "radios");
    info.instance("radios", format!("{}, each built with Radio::new and reporting through this column: a RadioGroup lays its Radios out gap_3 apart in a child of its own, which its style does not reach (radio.rs, RadioGroup), so the showcase lays them out widget_gap apart itself", labels.join(", ")))
}

/// A `Switch` reading `label`, `checked` or not, `disabled` or not, its
/// checked track handed `native`'s `switch.checked_background` where a
/// native theme is installed.
pub fn switch(
    t: &Theme,
    label: &'static str,
    checked: bool,
    disabled: bool,
    native: Option<&ResolvedTheme>,
) -> WidgetInfo {
    let info = WidgetInfo::new("Switch").variant(match (checked, disabled) {
        (true, false) => "on",
        (false, false) => "off",
        (true, true) => "on, disabled",
        (false, true) => "off, disabled",
    });
    // A disabled Switch fades its track alone, to half alpha (switch.rs:145);
    // the disabled style is resolved after the checked one (gpui-base
    // switch.rs:116-126). The checked track is the colour `Switch::color`
    // was handed, primary where it was handed none (switch.rs:136-139).
    let info = match (checked, disabled, native) {
        (true, false, Some(r)) => info.color(claim(
            "on track",
            "checked_background",
            stated(r.switch.checked_background),
            "showcase",
        )),
        (true, true, Some(r)) => info.color(claim(
            "on track, at 50%",
            "checked_background",
            stated(r.switch.checked_background).opacity(0.5),
            "showcase",
        )),
        _ => info,
    };
    let info = match (checked, disabled) {
        (true, _) if native.is_some() => info,
        (true, false) => info.color(claim(
            "on track",
            "primary",
            t.primary,
            "gpui-component/switch.rs:139",
        )),
        (false, false) => info.color(claim(
            "off track",
            "switch",
            t.switch,
            "gpui-component/switch.rs:140",
        )),
        (true, true) => info.color(claim(
            "on track, at 50%",
            "primary",
            t.primary.opacity(0.5),
            "gpui-component/switch.rs:139-145",
        )),
        (false, true) => info.color(claim(
            "off track, at 50%",
            "switch",
            t.switch.opacity(0.5),
            "gpui-component/switch.rs:140-145",
        )),
    };
    let info = info.color(claim(
        "thumb",
        "switch_thumb",
        t.switch_thumb,
        "gpui-component/switch.rs:146",
    ));
    let info = if disabled {
        info.color(claim(
            "disabled label",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/switch.rs:147",
        ))
    } else {
        info
    };
    // The track takes the theme's radius only under 4px; from 4px up it is
    // rounded by its own height (switch.rs:158-162).
    let info = if t.radius.as_f32() < 4.0 {
        info.config("border-radius", format!("radius: {}px", t.radius.as_f32()))
    } else {
        info.config("border-radius", format!("fully round: the theme's radius, {}px, is 4px or more, so the track is rounded by its own height instead", t.radius.as_f32()))
    };
    let info = match native {
        Some(_) => info.config("on track", "switch.checked_background, which the showcase hands Switch::color, a per-instance receiver: ThemeColor has no field for it, so the connector cannot install it (gpui-component switch.rs, Switch::color)"),
        None => info.not_themeable("on track", "primary: no native theme is installed, so the showcase has no switch.checked_background to hand Switch::color (gpui-component switch.rs, Switch::color)"),
    };
    let info = info
        .not_themeable("disabled", "the track at 50%, never the thumb: gpui multiplies each primitive's alpha rather than fading the subtree as a group, so fading both would let the track show through (gpui-component switch.rs, Switch::render disabled_bg)")
        .not_themeable("size", "Tier U: track and thumb are px literals per Size, on children of the wrapper the refinement lands on (gpui-component switch.rs, Switch::render), while the model states switch.track_width, track_height and thumb_diameter")
        .not_themeable("corner radius", "fully round unless the theme's radius is under 4px, in which case the theme's is used (gpui-component switch.rs, Switch::render radius). switch.track_radius is modelled and has no receiver")
        .not_themeable("animation timing", "reads the theme's spring_move (switch.rs, Switch), the same writable Theme::motion the Accordion and Collapsible use; the connector leaves it at its default because native-theme models no motion")
        .instance("label", label);
    if disabled {
        info.instance(
            "click",
            "none: it is disabled, and the showcase gives it no handler",
        )
    } else {
        info.instance("click", "turns it on or off; the showcase keeps the state")
    }
}

/// A `Slider`.
pub fn slider(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("Slider")
        .color(claim(
            "track",
            "slider_bar",
            t.slider_bar,
            "gpui-component/slider.rs:164",
        ))
        .color(claim(
            "thumb",
            "slider_thumb",
            t.slider_thumb,
            "gpui-component/slider.rs:170",
        ))
        .color(claim(
            "text",
            "foreground",
            t.foreground,
            "gpui-component/slider.rs:272",
        ))
        .not_themeable("track height", "Tier U, not an absence: slider.track_height is modelled and carried. Upstream sets h_1p5() on SliderIndicator, a child of the element the refinement lands on (slider.rs, Slider::render)")
        .not_themeable("thumb size", "Tier U, not an absence: slider.thumb_diameter is modelled and carried. The thumb is built by a closure inside the indicator, deeper still than the track (slider.rs, Slider::render)")
}

/// Where the theme-drawn widgets read what they paint.
#[cfg(feature = "widgets")]
const DRAWN_BY: &str = "native_theme_gpui::widgets, on gpui-base's headless primitive, every part from the theme (docs/todo_gpui-widgets-spec.md)";

/// A theme-drawn `widgets::Checkbox` (spec §2.1 of
/// docs/todo_gpui-widgets-spec.md) reading `label`, `checked` or not,
/// `disabled` or not, under `r`.
#[cfg(feature = "widgets")]
pub fn native_checkbox(
    r: &ResolvedTheme,
    label: &'static str,
    checked: bool,
    disabled: bool,
) -> WidgetInfo {
    let k = &r.checkbox;
    let info = WidgetInfo::new("Checkbox")
        .variant(match (checked, disabled) {
            (true, false) => "checked",
            (false, false) => "unchecked",
            (true, true) => "checked, disabled",
            (false, true) => "unchecked, disabled",
        })
        .config("drawn by", DRAWN_BY);
    let info = native_indicator(info, k, checked, disabled);
    let info = info
        .config("indicator", format!("checkbox.indicator_width: {}px square", k.indicator_width))
        .config("corner radius", format!("checkbox.border.corner_radius: {}px", k.border.corner_radius))
        .config("label gap", format!("checkbox.label_gap: {}px", k.label_gap))
        .config("label font", format!("checkbox.font: {}px, weight {}", k.font.size, k.font.weight))
        .not_themeable("check mark", super::own_icons("Check, the mark gpui-component's own checkbox draws (checkbox.rs, checkbox_check_icon), filling the box inside its border"))
        .instance("label", label);
    if disabled {
        info.instance(
            "click",
            "none: it is disabled, and the showcase gives it no handler",
        )
    } else {
        info.instance(
            "click",
            "checks or unchecks it; the showcase keeps the state",
        )
    }
}

/// The indicator's colours for a checkbox or radio in this state, as
/// `widgets::CheckboxLook` reads them.
#[cfg(feature = "widgets")]
fn native_indicator(
    info: WidgetInfo,
    k: &native_theme::theme::ResolvedCheckboxTheme,
    checked: bool,
    disabled: bool,
) -> WidgetInfo {
    let info = match (checked, disabled) {
        (_, true) => info.color(claim(
            "fill",
            "disabled_background",
            stated(k.disabled_background.unwrap_or(k.background_color)),
            "native-theme-gpui/widgets/checkbox.rs:71",
        )),
        (true, false) => info.color(claim(
            "fill",
            "checked_background",
            stated(k.checked_background),
            "native-theme-gpui/widgets/checkbox.rs:72",
        )),
        (false, false) => info
            .color(claim(
                "fill",
                "unchecked_background",
                stated(k.unchecked_background.unwrap_or(k.background_color)),
                "native-theme-gpui/widgets/checkbox.rs:69",
            ))
            .color(claim(
                "hover layer, over the fill",
                "hover_background",
                stated(k.hover_background.unwrap_or(k.background_color)),
                "native-theme-gpui/widgets/checkbox.rs:78",
            )),
    };
    let info = if checked {
        info.color(claim(
            "border",
            "border",
            stated(k.border.color),
            "native-theme-gpui/widgets/checkbox.rs:82",
        ))
    } else {
        info.color(claim(
            "border",
            "unchecked_border_color",
            stated(k.unchecked_border_color.unwrap_or(k.border.color)),
            "native-theme-gpui/widgets/checkbox.rs:84",
        ))
    };
    if disabled {
        info.color(claim(
            "mark and label",
            "disabled_text_color",
            stated(k.disabled_text_color),
            "native-theme-gpui/widgets/checkbox.rs:87",
        ))
    } else {
        info.color(claim(
            "mark",
            "indicator_color",
            stated(k.indicator_color),
            "native-theme-gpui/widgets/checkbox.rs:89",
        ))
        .color(claim(
            "label",
            "font",
            stated(k.font.color),
            "native-theme-gpui/widgets/checkbox.rs:89",
        ))
    }
}

/// A column of theme-drawn `widgets::Radio`s reading `labels`, the one at
/// `selected` selected (spec §2.2).
#[cfg(feature = "widgets")]
pub fn native_radio_column(
    r: &ResolvedTheme,
    labels: &[&'static str],
    selected: Option<usize>,
) -> WidgetInfo {
    let k = &r.checkbox;
    let chosen = selected
        .and_then(|ix| labels.get(ix).copied())
        .unwrap_or("none");
    let info = WidgetInfo::new("RadioGroup")
        .variant("vertical")
        .config("drawn by", DRAWN_BY);
    // The selected radio's colours, then the unselected ones'.
    let info = native_indicator(info, k, true, false);
    let info = info
        .color(claim(
            "unselected fill",
            "unchecked_background",
            stated(k.unchecked_background.unwrap_or(k.background_color)),
            "native-theme-gpui/widgets/checkbox.rs:69",
        ))
        .color(claim(
            "unselected border",
            "unchecked_border_color",
            stated(k.unchecked_border_color.unwrap_or(k.border.color)),
            "native-theme-gpui/widgets/checkbox.rs:84",
        ))
        .color(claim(
            "hover layer, over an unselected fill",
            "hover_background",
            stated(k.hover_background.unwrap_or(k.background_color)),
            "native-theme-gpui/widgets/checkbox.rs:78",
        ))
        .config("indicator", format!("checkbox.indicator_width: a circle {}px across (platform-facts §2.5: radio buttons are circular)", k.indicator_width))
        .config("label gap", format!("checkbox.label_gap: {}px", k.label_gap))
        .not_themeable("mark", "gpui-component's own check glyph (radio.rs, Radio::render): the model names the radio's indicator a dot (CheckboxTheme::indicator_color) but states no dot size, and gpui has no dot of its own")
        .instance("radios", format!("{}, each a widgets::Radio told its place in the group, widget_gap apart", labels.join(", ")));
    info.instance("selected", chosen)
}

/// A theme-drawn `widgets::Switch` reading `label`, `checked` or not,
/// `disabled` or not (spec §2.3).
#[cfg(feature = "widgets")]
pub fn native_switch(
    r: &ResolvedTheme,
    label: &'static str,
    checked: bool,
    disabled: bool,
) -> WidgetInfo {
    let s = &r.switch;
    let info = WidgetInfo::new("Switch")
        .variant(match (checked, disabled) {
            (true, false) => "on",
            (false, false) => "off",
            (true, true) => "on, disabled",
            (false, true) => "off, disabled",
        })
        .config("drawn by", DRAWN_BY);
    let info = match (checked, disabled) {
        (true, false) => info
            .color(claim(
                "track",
                "checked_background",
                stated(s.checked_background),
                "native-theme-gpui/widgets/switch.rs:63",
            ))
            .color(claim(
                "hover layer, over the track",
                "hover_checked_background",
                stated(s.hover_checked_background.unwrap_or(s.checked_background)),
                "native-theme-gpui/widgets/switch.rs:73",
            )),
        (false, false) => info
            .color(claim(
                "track",
                "unchecked_background",
                stated(s.unchecked_background),
                "native-theme-gpui/widgets/switch.rs:65",
            ))
            .color(claim(
                "hover layer, over the track",
                "hover_unchecked_background",
                stated(
                    s.hover_unchecked_background
                        .unwrap_or(s.unchecked_background),
                ),
                "native-theme-gpui/widgets/switch.rs:75",
            )),
        (true, true) => info.color(claim(
            "track",
            "disabled_checked_background",
            stated(
                s.disabled_checked_background
                    .unwrap_or(s.checked_background),
            ),
            "native-theme-gpui/widgets/switch.rs:68",
        )),
        (false, true) => info.color(claim(
            "track",
            "disabled_unchecked_background",
            stated(
                s.disabled_unchecked_background
                    .unwrap_or(s.unchecked_background),
            ),
            "native-theme-gpui/widgets/switch.rs:69",
        )),
    };
    let info = if disabled {
        info.color(claim(
            "thumb",
            "disabled_thumb_color",
            stated(s.disabled_thumb_color.unwrap_or(s.thumb_background)),
            "native-theme-gpui/widgets/switch.rs:79",
        ))
        .color(claim(
            "label",
            "disabled_text_color",
            stated(r.defaults.disabled_text_color),
            "native-theme-gpui/widgets/switch.rs:84",
        ))
    } else {
        info.color(claim(
            "thumb",
            "thumb_background",
            stated(s.thumb_background),
            "native-theme-gpui/widgets/switch.rs:81",
        ))
        .color(claim(
            "label",
            "text_color",
            stated(r.defaults.text_color),
            "native-theme-gpui/widgets/switch.rs:86",
        ))
    };
    let info = info
        .config(
            "track",
            format!(
                "switch.track_width × track_height: {}×{}px, radius switch.track_radius {}px",
                s.track_width, s.track_height, s.track_radius
            ),
        )
        .config(
            "thumb",
            format!(
                "switch.thumb_diameter: {}px, centred on the track's axis",
                s.thumb_diameter
            ),
        )
        .config(
            "label font",
            "SwitchTheme states none: the window's defaults.font",
        )
        .not_themeable(
            "label gap",
            "gpui-component's gap_2 (switch.rs, Switch::render): SwitchTheme states no label gap",
        )
        .instance("label", label);
    if disabled {
        info.instance(
            "click",
            "none: it is disabled, and the showcase gives it no handler",
        )
    } else {
        info.instance("click", "turns it on or off; the showcase keeps the state")
    }
}

/// A theme-drawn `widgets::Slider` (spec §2.4).
#[cfg(feature = "widgets")]
pub fn native_slider(r: &ResolvedTheme) -> WidgetInfo {
    let s = &r.slider;
    WidgetInfo::new("Slider")
        .config("drawn by", DRAWN_BY)
        .color(claim("rail", "track_color", stated(s.track_color), "native-theme-gpui/widgets/slider.rs:66"))
        .color(claim("filled part", "fill_color", stated(s.fill_color), "native-theme-gpui/widgets/slider.rs:66"))
        .color(claim("thumb", "thumb_color", stated(s.thumb_color), "native-theme-gpui/widgets/slider.rs:66"))
        .color(claim("hover layer, over the thumb", "thumb_hover_color", stated(s.thumb_hover_color.unwrap_or(s.thumb_color)), "native-theme-gpui/widgets/slider.rs:71"))
        .config("rail", format!("slider.track_height: {}px", s.track_height))
        .config("thumb", format!("slider.thumb_diameter: {}px; its centre travels the width less a radius at each end", s.thumb_diameter))
        .not_themeable("thumb outline", "gpui-component's own: a 1px ring of the fill colour at half alpha (slider.rs, Slider::render); the model states no thumb outline")
        .not_themeable("rail radius", "a pill, gpui-component's radius_full (slider.rs, Slider::render); SliderTheme states no rail radius")
        .instance("keyboard", "Left/Down and Right/Up step it, Home and End go to the ends")
}

pub fn rating(t: &Theme, value: usize, disabled: bool) -> WidgetInfo {
    let info = WidgetInfo::new("Rating").not_themeable(
        "own icons",
        super::own_icons("Star, and StarFill for a star at or below the value (rating.rs, Rating)"),
    );
    let info = if disabled {
        info.variant("disabled")
    } else {
        info
    };
    let info = info
        .color(claim(
            "active star",
            "yellow",
            t.yellow,
            "gpui-component/rating.rs:120",
        ))
        // Rating colours only a filled or hovered star (rating.rs:159), so
        // an empty one takes the colour the showcase sets on its window.
        .color(claim(
            "inactive star, inherited",
            "foreground",
            t.foreground,
            "showcase",
        ))
        .not_themeable("active colour", "cx.theme().yellow unless Rating::color overrides it (rating.rs, Rating::render active_color)")
        .not_themeable("inactive colour", "none of its own: Rating colours only a filled or hovered star, so an empty one takes the window's text colour (rating.rs, Rating::render)");
    let info = if disabled {
        info.not_themeable("hover preview", "none: a disabled Rating gives its stars no pointer handlers, so it neither previews a value nor takes a click (rating.rs, Rating::render)")
    } else {
        info.not_themeable(
            "hover preview",
            "upstream keeps its own hovered value (rating.rs, RaitingState::hovered_value)",
        )
    };
    let info = info.instance("value", format!("{value} of 5 stars"));
    if disabled {
        info
    } else {
        info.instance("click", "a star above the value sets it there, and one at or below it clears down to the star before (rating.rs, Rating::render); the showcase keeps the value")
    }
}

/// An `OtpInput`.
pub fn otp_input(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("OtpInput")
        .color(claim(
            "border",
            "input",
            t.input,
            "gpui-component/input/otp_input.rs:117",
        ))
        .color(claim(
            "focused border",
            "ring",
            t.ring,
            "gpui-component/input/otp_input.rs:121",
        ))
        .color(input_background(t))
        .color(claim(
            "digit",
            "foreground",
            t.foreground,
            "gpui-component/input/input.rs:105",
        ))
        .color(claim(
            "caret",
            "caret",
            t.caret,
            "gpui-component/input/otp_input.rs:159",
        ))
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable("fill", "input_background(), as an Input's, and the digits take the Input's foreground. The secondary_foreground and muted_foreground the panel had named colour a masked asterisk, and this OtpState is not masked (input/otp_input.rs, OtpInput)")
        .instance("digit count", "configurable")
        .instance("groups", "2")
}

/// A `Select`. Its geometry line is recorded where `demo::select` applies
/// the builder.
pub fn select(t: &Theme, native: Option<&ResolvedTheme>) -> WidgetInfo {
    combo_fill(WidgetInfo::new("Select"), t, native, false)
        .color(claim(
            "upstream trigger text",
            "foreground",
            t.foreground,
            "gpui-component/input/input.rs:105",
        ))
        .color(claim(
            "focused border",
            "ring",
            t.ring,
            "gpui-component/select.rs:548",
        ))
        .color(claim(
            "placeholder",
            "muted_foreground",
            t.muted_foreground,
            "gpui-component/select.rs:445",
        ))
        .not_themeable("carried colour", "the one difference from a Combobox, such as the theme settings' preset switch: Select's selected-title child sets its own colour, so a carried colour yields to the disabled colour instead of beating it, and the connector carries it (native-theme-gpui geometry.rs, combobox)")
        .not_themeable("caret", "its colour is themed -- upstream paints it with muted_foreground (select.rs, Caret) -- and its size is not: Caret maps Size::Size into the same arm as Medium (select.rs, Caret::render), so combo_box.arrow_icon_size has no route at all, not even through the Size::Size escape hatch a DataTable row accepts. Tier U for the size")
        .not_themeable("own icons", super::own_icons("the caret's ChevronDown (select.rs, Caret), and in its open list Check beside the chosen row (searchable_list/item.rs, SearchableListItemElement) and Inbox while the list is empty (select.rs, SelectState::new)"))
}

/// A drop-down trigger's fill and edge: `native`'s `combo_box.background_color`
/// and `combo_box.border.color`, which the showcase refines the trigger with
/// over upstream's (`demo::combo_fill`), where a native theme is installed;
/// upstream's `input_background()` and `input` where none is, the edge read
/// by a `Combobox` where `combobox`, by a `Select` where not.
pub fn combo_fill(
    info: WidgetInfo,
    t: &Theme,
    native: Option<&ResolvedTheme>,
    combobox: bool,
) -> WidgetInfo {
    match native {
        Some(r) => info
            .color(claim(
                "trigger fill",
                "background_color",
                stated(r.combo_box.background_color),
                "showcase",
            ))
            .color(claim(
                "trigger border",
                "color",
                stated(r.combo_box.border.color),
                "showcase",
            ))
            .config("fill", "combo_box.background_color and combo_box.border.color, which the showcase refines the trigger with over the input_background() fill and input edge it paints first (select.rs, Select::render; combobox.rs, Combobox::render)"),
        None => info
            .color(input_background(t))
            .color(if combobox {
                claim(
                    "trigger border",
                    "input",
                    t.input,
                    "gpui-component/combobox.rs:992",
                )
            } else {
                claim(
                    "trigger border",
                    "input",
                    t.input,
                    "gpui-component/select.rs:541",
                )
            })
            .not_themeable("fill", "input_background(), as an Input's: the window background in light mode, and input mixed toward transparent in dark -- one accessor, two sources (theme/mod.rs, input_background)"),
    }
}

/// A `ColorPicker`.
pub fn color_picker(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("ColorPicker")
        .color(claim(
            "popover",
            "popover",
            t.popover,
            "gpui-component/styled.rs:197",
        ))
        .color(claim(
            "featured red",
            "red",
            t.red,
            "gpui-component/color_picker.rs:205",
        ))
        .color(claim(
            "featured red light",
            "red_light",
            t.red_light,
            "gpui-component/color_picker.rs:206",
        ))
        .color(claim(
            "featured blue",
            "blue",
            t.blue,
            "gpui-component/color_picker.rs:207",
        ))
        .color(claim(
            "featured blue light",
            "blue_light",
            t.blue_light,
            "gpui-component/color_picker.rs:208",
        ))
        .color(claim(
            "featured green",
            "green",
            t.green,
            "gpui-component/color_picker.rs:209",
        ))
        .color(claim(
            "featured green light",
            "green_light",
            t.green_light,
            "gpui-component/color_picker.rs:210",
        ))
        .color(claim(
            "featured yellow",
            "yellow",
            t.yellow,
            "gpui-component/color_picker.rs:211",
        ))
        .color(claim(
            "featured yellow light",
            "yellow_light",
            t.yellow_light,
            "gpui-component/color_picker.rs:212",
        ))
        .color(claim(
            "featured cyan",
            "cyan",
            t.cyan,
            "gpui-component/color_picker.rs:213",
        ))
        .color(claim(
            "featured cyan light",
            "cyan_light",
            t.cyan_light,
            "gpui-component/color_picker.rs:214",
        ))
        .color(claim(
            "featured magenta",
            "magenta",
            t.magenta,
            "gpui-component/color_picker.rs:215",
        ))
        .color(claim(
            "featured magenta light",
            "magenta_light",
            t.magenta_light,
            "gpui-component/color_picker.rs:216",
        ))
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable("swatch", "the picked colour itself, edged with it darkened by 30%: background and input frame only an empty swatch, and this one starts with a value (color_picker.rs, ColorPickerButton)")
        .not_themeable("palette grid", "the rows under the featured swatches are gpui-component's own colour scales, not the platform's (color_picker.rs, color_palettes)")
}

/// A `DatePicker`.
pub fn date_picker(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("DatePicker")
        .color(claim(
            "border",
            "input",
            t.input,
            "gpui-component/time/date_picker.rs:443",
        ))
        .color(claim(
            "popover",
            "popover",
            t.popover,
            "gpui-component/styled.rs:197",
        ))
        .color(claim(
            "selected day",
            "primary",
            t.primary,
            "gpui-component/time/calendar.rs:185",
        ))
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable("calendar icon", "an IconName::Calendar built inline with no setter (time/date_picker.rs, DatePicker)")
        .not_themeable("own icons", super::own_icons("the trigger's Calendar (time/date_picker.rs, DatePicker), and ChevronLeft and ChevronRight on the month buttons of the Calendar it opens (time/calendar.rs, Calendar)"))
        .not_themeable("format", "%Y/%m/%d unless the application sets another, whatever the locale (time/date_picker.rs, DatePickerState::date_format)")
}

/// A `Calendar`.
pub fn calendar(t: &Theme) -> WidgetInfo {
    WidgetInfo::new("Calendar")
        .color(claim(
            "border",
            "border",
            t.border,
            "gpui-component/time/calendar.rs:195",
        ))
        .color(claim(
            "selected day",
            "primary",
            t.primary,
            "gpui-component/time/calendar.rs:185",
        ))
        .color(claim(
            "today",
            "accent",
            t.accent,
            "gpui-component/time/calendar.rs:189",
        ))
        .color(claim(
            "text",
            "foreground",
            t.foreground,
            "gpui-component/time/calendar.rs:180",
        ))
        .config("border-radius", format!("radius_lg: {}px", t.radius_lg.as_f32()))
        .not_themeable("fill", "none: a Calendar sets an edge, a radius and a padding but no background, so the window shows through (time/calendar.rs, Calendar)")
        .not_themeable("month navigation", "ChevronLeft and ChevronRight built inline with no setter (time/calendar.rs, Calendar)")
        .not_themeable("own icons", super::own_icons("ChevronLeft and ChevronRight on its month buttons (time/calendar.rs, Calendar)"))
}
