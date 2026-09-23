//! What the Theme Map's rows report about themselves (spec §3.4).
//!
//! A swatch is one ThemeColor field as the installed theme holds it. Where
//! a native theme is installed for the mode, the connector wrote it: the
//! value's claim cites the connector line that writes the field, and the
//! notes name the model fields that line's value is read from and how it
//! is made from them. Where the making branches on the mode, the note says
//! how in both.

use gpui_component::theme::Theme;

use super::{ColorClaim, WidgetInfo, claim, hsla_to_hex, percent_text, px_text, text};
use crate::demo::{ControlHeight, ControlWidget, ThemeToken};

/// `hover_color`, which makes the field from the model's colour.
fn hover(info: WidgetInfo) -> WidgetInfo {
    info.config(
        "hover_color",
        "the colour faded to 90% of its alpha and blended over window.background_color (native-theme-gpui/derive.rs, hover_color)",
    )
}

/// `active_color`, which makes the field from the model's colour.
fn active(info: WidgetInfo) -> WidgetInfo {
    info.config(
        "active_color",
        "the colour's lightness × 0.9 in light mode and × 0.8 in dark, or × 1.1 and × 1.2 where it is below 0.15 (native-theme-gpui/derive.rs, active_color)",
    )
}

/// `light_variant`, which makes the field from the model's colour.
fn light(info: WidgetInfo) -> WidgetInfo {
    info.config(
        "light_variant",
        "in dark mode the colour's lightness raised by 0.15, to at most 0.95; in light mode the colour faded to 80% of its alpha and blended over window.background_color (native-theme-gpui/derive.rs, light_variant)",
    )
}

/// How list_active_border is made, which table_active_border copies.
fn list_active_border(info: WidgetInfo) -> WidgetInfo {
    info.config(
        "derived",
        "button.primary_background faded to 60% of its alpha and blended over window.background_color (native-theme-gpui/colors.rs, assign_list_table)",
    )
}

/// One row of the swatch table: the installed value of its field, claimed
/// at the connector line that writes it, and the notes naming the model
/// fields the connector reads for it and how it makes the value from them,
/// where it is not the model field as it is.
pub struct Row {
    pub value: ColorClaim,
    notes: fn(WidgetInfo) -> WidgetInfo,
}

/// The row of `token`.
pub fn row(t: &Theme, token: ThemeToken) -> Row {
    let (value, notes): (ColorClaim, fn(WidgetInfo) -> WidgetInfo) = match token {
        ThemeToken::Background => (
            claim(
                "value",
                "background",
                t.background,
                "native-theme-gpui/colors.rs:254",
            ),
            |i| i.config("model", "window.background_color"),
        ),
        ThemeToken::Foreground => (
            claim(
                "value",
                "foreground",
                t.foreground,
                "native-theme-gpui/colors.rs:255",
            ),
            |i| i.config("model", "defaults.text_color"),
        ),
        ThemeToken::Accent => (
            claim(
                "value",
                "accent",
                t.accent,
                "native-theme-gpui/colors.rs:262",
            ),
            |i| {
                i.config("model", "menu.hover_background").config("derived", "none: the model field as it is. gpui-component's accent is its item highlight, not the platform's accent colour, so it takes the menu's hover (native-theme-gpui/colors.rs, assign_core)")
            },
        ),
        ThemeToken::AccentForeground => (
            claim(
                "value",
                "accent_foreground",
                t.accent_foreground,
                "native-theme-gpui/colors.rs:263",
            ),
            |i| i.config("model", "menu.hover_text_color"),
        ),
        ThemeToken::Border => (
            claim(
                "value",
                "border",
                t.border,
                "native-theme-gpui/colors.rs:264",
            ),
            |i| i.config("model", "defaults.border.color"),
        ),
        ThemeToken::Muted => (
            claim("value", "muted", t.muted, "native-theme-gpui/colors.rs:265"),
            |i| {
                i.config("model", "defaults.text_color, window.background_color").config("derived", "defaults.text_color faded to 10% of its alpha and blended over window.background_color (native-theme-gpui/colors.rs, to_theme_color)")
            },
        ),
        ThemeToken::MutedForeground => (
            claim(
                "value",
                "muted_foreground",
                t.muted_foreground,
                "native-theme-gpui/colors.rs:266",
            ),
            |i| i.config("model", "defaults.muted_color"),
        ),
        ThemeToken::Input => (
            claim("value", "input", t.input, "native-theme-gpui/colors.rs:267"),
            |i| i.config("model", "input.border.color"),
        ),
        ThemeToken::Ring => (
            claim("value", "ring", t.ring, "native-theme-gpui/colors.rs:268"),
            |i| i.config("model", "defaults.focus_ring_color"),
        ),
        ThemeToken::Selection => (
            claim(
                "value",
                "selection",
                t.selection,
                "native-theme-gpui/colors.rs:269",
            ),
            |i| i.config("model", "input.selection_background"),
        ),
        ThemeToken::Caret => (
            claim("value", "caret", t.caret, "native-theme-gpui/colors.rs:546"),
            |i| i.config("model", "input.caret_color"),
        ),
        ThemeToken::Link => (
            claim("value", "link", t.link, "native-theme-gpui/colors.rs:270"),
            |i| i.config("model", "link.font.color"),
        ),
        ThemeToken::LinkHover => (
            claim(
                "value",
                "link_hover",
                t.link_hover,
                "native-theme-gpui/colors.rs:277",
            ),
            |i| i.config("model", "link.hover_text_color"),
        ),
        ThemeToken::LinkActive => (
            claim(
                "value",
                "link_active",
                t.link_active,
                "native-theme-gpui/colors.rs:278",
            ),
            |i| i.config("model", "link.active_text_color"),
        ),
        ThemeToken::Overlay => (
            claim(
                "value",
                "overlay",
                t.overlay,
                "native-theme-gpui/colors.rs:516",
            ),
            |i| {
                i.config("model", "defaults.shadow_color").config("derived", "defaults.shadow_color's hue, saturation and lightness, its alpha replaced: 1 where reduced transparency is asked for, else 0.5 in dark mode and 0.4 in light (native-theme-gpui/colors.rs, assign_misc)")
            },
        ),
        ThemeToken::Primary => (
            claim(
                "value",
                "primary",
                t.primary,
                "native-theme-gpui/colors.rs:282",
            ),
            |i| i.config("model", "button.primary_background"),
        ),
        ThemeToken::PrimaryForeground => (
            claim(
                "value",
                "primary_foreground",
                t.primary_foreground,
                "native-theme-gpui/colors.rs:283",
            ),
            |i| i.config("model", "button.primary_text_color"),
        ),
        ThemeToken::PrimaryHover => (
            claim(
                "value",
                "primary_hover",
                t.primary_hover,
                "native-theme-gpui/colors.rs:284",
            ),
            |i| {
                hover(i.config(
                    "model",
                    "button.primary_background, window.background_color",
                ))
            },
        ),
        ThemeToken::PrimaryActive => (
            claim(
                "value",
                "primary_active",
                t.primary_active,
                "native-theme-gpui/colors.rs:285",
            ),
            |i| active(i.config("model", "button.primary_background")),
        ),
        ThemeToken::Secondary => (
            claim(
                "value",
                "secondary",
                t.secondary,
                "native-theme-gpui/colors.rs:289",
            ),
            |i| i.config("model", "button.background_color"),
        ),
        ThemeToken::SecondaryForeground => (
            claim(
                "value",
                "secondary_foreground",
                t.secondary_foreground,
                "native-theme-gpui/colors.rs:290",
            ),
            |i| i.config("model", "button.font.color"),
        ),
        ThemeToken::SecondaryHover => (
            claim(
                "value",
                "secondary_hover",
                t.secondary_hover,
                "native-theme-gpui/colors.rs:291",
            ),
            |i| i.config("model", "button.hover_background"),
        ),
        ThemeToken::SecondaryActive => (
            claim(
                "value",
                "secondary_active",
                t.secondary_active,
                "native-theme-gpui/colors.rs:292",
            ),
            |i| {
                active(i.config("model", "button.active_background, else button.background_color").config("derived", "button.active_background where the model states one; where it states none, button.background_color through active_color (native-theme-gpui/colors.rs, assign_secondary)"))
            },
        ),
        ThemeToken::Button => (
            claim(
                "value",
                "button",
                t.button,
                "native-theme-gpui/colors.rs:338",
            ),
            |i| {
                i.config("model", "button.background_color").config(
                    "copied",
                    "secondary's value (native-theme-gpui/colors.rs, assign_buttons)",
                )
            },
        ),
        ThemeToken::ButtonHover => (
            claim(
                "value",
                "button_hover",
                t.button_hover,
                "native-theme-gpui/colors.rs:354",
            ),
            |i| {
                i.config("model", "button.hover_background, button.background_color").config("derived", "secondary_hover laid over button: the blend lerps by the layer's alpha and keeps the base's, so an opaque secondary_hover stands as it is and a translucent one is composited over the idle fill (native-theme-gpui/colors.rs, assign_buttons)")
            },
        ),
        ThemeToken::ButtonActive => (
            claim(
                "value",
                "button_active",
                t.button_active,
                "native-theme-gpui/colors.rs:355",
            ),
            |i| {
                i.config("model", "button.active_background, else button.background_color").config("derived", "secondary_active laid over button: the blend lerps by the layer's alpha and keeps the base's, so an opaque secondary_active stands as it is and a translucent one is composited over the idle fill (native-theme-gpui/colors.rs, assign_buttons)")
            },
        ),
        ThemeToken::ButtonForeground => {
            (
                claim(
                    "value",
                    "button_foreground",
                    t.button_foreground,
                    "native-theme-gpui/colors.rs:356",
                ),
                |i| {
                    i.config("model", "button.font.color").config("copied", "secondary_foreground's value (native-theme-gpui/colors.rs, assign_buttons)")
                },
            )
        }
        ThemeToken::ButtonSecondary => (
            claim(
                "value",
                "button_secondary",
                t.button_secondary,
                "native-theme-gpui/colors.rs:357",
            ),
            |i| {
                i.config("model", "button.background_color").config(
                    "copied",
                    "secondary's value (native-theme-gpui/colors.rs, assign_buttons)",
                )
            },
        ),
        ThemeToken::ButtonSecondaryHover => (
            claim(
                "value",
                "button_secondary_hover",
                t.button_secondary_hover,
                "native-theme-gpui/colors.rs:358",
            ),
            |i| {
                i.config("model", "button.hover_background, button.background_color").config("derived", "secondary_hover laid over button_secondary: the blend lerps by the layer's alpha and keeps the base's, so an opaque secondary_hover stands as it is and a translucent one is composited over the idle fill (native-theme-gpui/colors.rs, assign_buttons)")
            },
        ),
        ThemeToken::ButtonSecondaryActive => (
            claim(
                "value",
                "button_secondary_active",
                t.button_secondary_active,
                "native-theme-gpui/colors.rs:359",
            ),
            |i| {
                i.config("model", "button.active_background, else button.background_color").config("derived", "secondary_active laid over button_secondary: the blend lerps by the layer's alpha and keeps the base's, so an opaque secondary_active stands as it is and a translucent one is composited over the idle fill (native-theme-gpui/colors.rs, assign_buttons)")
            },
        ),
        ThemeToken::ButtonSecondaryForeground => {
            (
                claim(
                    "value",
                    "button_secondary_foreground",
                    t.button_secondary_foreground,
                    "native-theme-gpui/colors.rs:360",
                ),
                |i| {
                    i.config("model", "button.font.color").config("copied", "secondary_foreground's value (native-theme-gpui/colors.rs, assign_buttons)")
                },
            )
        }
        ThemeToken::ButtonPrimary => (
            claim(
                "value",
                "button_primary",
                t.button_primary,
                "native-theme-gpui/colors.rs:362",
            ),
            |i| {
                i.config("model", "button.primary_background").config(
                    "copied",
                    "primary's value (native-theme-gpui/colors.rs, assign_buttons)",
                )
            },
        ),
        ThemeToken::ButtonPrimaryHover => (
            claim(
                "value",
                "button_primary_hover",
                t.button_primary_hover,
                "native-theme-gpui/colors.rs:363",
            ),
            |i| {
                hover(
                    i.config(
                        "model",
                        "button.primary_background, window.background_color",
                    )
                    .config(
                        "copied",
                        "primary_hover's value (native-theme-gpui/colors.rs, assign_buttons)",
                    ),
                )
            },
        ),
        ThemeToken::ButtonPrimaryActive => (
            claim(
                "value",
                "button_primary_active",
                t.button_primary_active,
                "native-theme-gpui/colors.rs:364",
            ),
            |i| {
                active(i.config("model", "button.primary_background").config(
                    "copied",
                    "primary_active's value (native-theme-gpui/colors.rs, assign_buttons)",
                ))
            },
        ),
        ThemeToken::ButtonPrimaryForeground => (
            claim(
                "value",
                "button_primary_foreground",
                t.button_primary_foreground,
                "native-theme-gpui/colors.rs:365",
            ),
            |i| {
                i.config("model", "button.primary_text_color").config(
                    "copied",
                    "primary_foreground's value (native-theme-gpui/colors.rs, assign_buttons)",
                )
            },
        ),
        ThemeToken::ButtonDanger => (
            claim(
                "value",
                "button_danger",
                t.button_danger,
                "native-theme-gpui/colors.rs:367",
            ),
            |i| {
                i.config("model", "defaults.danger_color").config(
                    "copied",
                    "danger's value (native-theme-gpui/colors.rs, assign_buttons)",
                )
            },
        ),
        ThemeToken::ButtonDangerHover => {
            (
                claim(
                    "value",
                    "button_danger_hover",
                    t.button_danger_hover,
                    "native-theme-gpui/colors.rs:368",
                ),
                |i| {
                    hover(i.config("model", "defaults.danger_color, window.background_color").config("copied", "danger_hover's value (native-theme-gpui/colors.rs, assign_buttons)"))
                },
            )
        }
        ThemeToken::ButtonDangerActive => (
            claim(
                "value",
                "button_danger_active",
                t.button_danger_active,
                "native-theme-gpui/colors.rs:369",
            ),
            |i| {
                active(i.config("model", "defaults.danger_color").config(
                    "copied",
                    "danger_active's value (native-theme-gpui/colors.rs, assign_buttons)",
                ))
            },
        ),
        ThemeToken::ButtonDangerForeground => (
            claim(
                "value",
                "button_danger_foreground",
                t.button_danger_foreground,
                "native-theme-gpui/colors.rs:370",
            ),
            |i| {
                i.config("model", "defaults.danger_text_color").config(
                    "copied",
                    "danger_foreground's value (native-theme-gpui/colors.rs, assign_buttons)",
                )
            },
        ),
        ThemeToken::ButtonInfo => (
            claim(
                "value",
                "button_info",
                t.button_info,
                "native-theme-gpui/colors.rs:372",
            ),
            |i| {
                i.config("model", "defaults.info_color").config(
                    "copied",
                    "info's value (native-theme-gpui/colors.rs, assign_buttons)",
                )
            },
        ),
        ThemeToken::ButtonInfoHover => (
            claim(
                "value",
                "button_info_hover",
                t.button_info_hover,
                "native-theme-gpui/colors.rs:373",
            ),
            |i| {
                hover(
                    i.config("model", "defaults.info_color, window.background_color")
                        .config(
                            "copied",
                            "info_hover's value (native-theme-gpui/colors.rs, assign_buttons)",
                        ),
                )
            },
        ),
        ThemeToken::ButtonInfoActive => (
            claim(
                "value",
                "button_info_active",
                t.button_info_active,
                "native-theme-gpui/colors.rs:374",
            ),
            |i| {
                active(i.config("model", "defaults.info_color").config(
                    "copied",
                    "info_active's value (native-theme-gpui/colors.rs, assign_buttons)",
                ))
            },
        ),
        ThemeToken::ButtonInfoForeground => (
            claim(
                "value",
                "button_info_foreground",
                t.button_info_foreground,
                "native-theme-gpui/colors.rs:375",
            ),
            |i| {
                i.config("model", "defaults.info_text_color").config(
                    "copied",
                    "info_foreground's value (native-theme-gpui/colors.rs, assign_buttons)",
                )
            },
        ),
        ThemeToken::ButtonSuccess => (
            claim(
                "value",
                "button_success",
                t.button_success,
                "native-theme-gpui/colors.rs:377",
            ),
            |i| {
                i.config("model", "defaults.success_color").config(
                    "copied",
                    "success's value (native-theme-gpui/colors.rs, assign_buttons)",
                )
            },
        ),
        ThemeToken::ButtonSuccessHover => {
            (
                claim(
                    "value",
                    "button_success_hover",
                    t.button_success_hover,
                    "native-theme-gpui/colors.rs:378",
                ),
                |i| {
                    hover(i.config("model", "defaults.success_color, window.background_color").config("copied", "success_hover's value (native-theme-gpui/colors.rs, assign_buttons)"))
                },
            )
        }
        ThemeToken::ButtonSuccessActive => (
            claim(
                "value",
                "button_success_active",
                t.button_success_active,
                "native-theme-gpui/colors.rs:379",
            ),
            |i| {
                active(i.config("model", "defaults.success_color").config(
                    "copied",
                    "success_active's value (native-theme-gpui/colors.rs, assign_buttons)",
                ))
            },
        ),
        ThemeToken::ButtonSuccessForeground => (
            claim(
                "value",
                "button_success_foreground",
                t.button_success_foreground,
                "native-theme-gpui/colors.rs:380",
            ),
            |i| {
                i.config("model", "defaults.success_text_color").config(
                    "copied",
                    "success_foreground's value (native-theme-gpui/colors.rs, assign_buttons)",
                )
            },
        ),
        ThemeToken::ButtonWarning => (
            claim(
                "value",
                "button_warning",
                t.button_warning,
                "native-theme-gpui/colors.rs:382",
            ),
            |i| {
                i.config("model", "defaults.warning_color").config(
                    "copied",
                    "warning's value (native-theme-gpui/colors.rs, assign_buttons)",
                )
            },
        ),
        ThemeToken::ButtonWarningHover => {
            (
                claim(
                    "value",
                    "button_warning_hover",
                    t.button_warning_hover,
                    "native-theme-gpui/colors.rs:383",
                ),
                |i| {
                    hover(i.config("model", "defaults.warning_color, window.background_color").config("copied", "warning_hover's value (native-theme-gpui/colors.rs, assign_buttons)"))
                },
            )
        }
        ThemeToken::ButtonWarningActive => (
            claim(
                "value",
                "button_warning_active",
                t.button_warning_active,
                "native-theme-gpui/colors.rs:384",
            ),
            |i| {
                active(i.config("model", "defaults.warning_color").config(
                    "copied",
                    "warning_active's value (native-theme-gpui/colors.rs, assign_buttons)",
                ))
            },
        ),
        ThemeToken::ButtonWarningForeground => (
            claim(
                "value",
                "button_warning_foreground",
                t.button_warning_foreground,
                "native-theme-gpui/colors.rs:385",
            ),
            |i| {
                i.config("model", "defaults.warning_text_color").config(
                    "copied",
                    "warning_foreground's value (native-theme-gpui/colors.rs, assign_buttons)",
                )
            },
        ),
        ThemeToken::Danger => (
            claim(
                "value",
                "danger",
                t.danger,
                "native-theme-gpui/colors.rs:306",
            ),
            |i| i.config("model", "defaults.danger_color"),
        ),
        ThemeToken::DangerForeground => (
            claim(
                "value",
                "danger_foreground",
                t.danger_foreground,
                "native-theme-gpui/colors.rs:307",
            ),
            |i| i.config("model", "defaults.danger_text_color"),
        ),
        ThemeToken::DangerHover => (
            claim(
                "value",
                "danger_hover",
                t.danger_hover,
                "native-theme-gpui/colors.rs:308",
            ),
            |i| hover(i.config("model", "defaults.danger_color, window.background_color")),
        ),
        ThemeToken::DangerActive => (
            claim(
                "value",
                "danger_active",
                t.danger_active,
                "native-theme-gpui/colors.rs:309",
            ),
            |i| active(i.config("model", "defaults.danger_color")),
        ),
        ThemeToken::Red => (
            claim("value", "red", t.red, "native-theme-gpui/colors.rs:647"),
            |i| i.config("model", "defaults.danger_color"),
        ),
        ThemeToken::RedLight => (
            claim(
                "value",
                "red_light",
                t.red_light,
                "native-theme-gpui/colors.rs:648",
            ),
            |i| light(i.config("model", "defaults.danger_color, window.background_color")),
        ),
        ThemeToken::Success => (
            claim(
                "value",
                "success",
                t.success,
                "native-theme-gpui/colors.rs:311",
            ),
            |i| i.config("model", "defaults.success_color"),
        ),
        ThemeToken::SuccessForeground => (
            claim(
                "value",
                "success_foreground",
                t.success_foreground,
                "native-theme-gpui/colors.rs:312",
            ),
            |i| i.config("model", "defaults.success_text_color"),
        ),
        ThemeToken::SuccessHover => (
            claim(
                "value",
                "success_hover",
                t.success_hover,
                "native-theme-gpui/colors.rs:313",
            ),
            |i| hover(i.config("model", "defaults.success_color, window.background_color")),
        ),
        ThemeToken::SuccessActive => (
            claim(
                "value",
                "success_active",
                t.success_active,
                "native-theme-gpui/colors.rs:314",
            ),
            |i| active(i.config("model", "defaults.success_color")),
        ),
        ThemeToken::Green => (
            claim("value", "green", t.green, "native-theme-gpui/colors.rs:649"),
            |i| i.config("model", "defaults.success_color"),
        ),
        ThemeToken::GreenLight => (
            claim(
                "value",
                "green_light",
                t.green_light,
                "native-theme-gpui/colors.rs:650",
            ),
            |i| light(i.config("model", "defaults.success_color, window.background_color")),
        ),
        ThemeToken::Warning => (
            claim(
                "value",
                "warning",
                t.warning,
                "native-theme-gpui/colors.rs:316",
            ),
            |i| i.config("model", "defaults.warning_color"),
        ),
        ThemeToken::WarningForeground => (
            claim(
                "value",
                "warning_foreground",
                t.warning_foreground,
                "native-theme-gpui/colors.rs:317",
            ),
            |i| i.config("model", "defaults.warning_text_color"),
        ),
        ThemeToken::WarningHover => (
            claim(
                "value",
                "warning_hover",
                t.warning_hover,
                "native-theme-gpui/colors.rs:318",
            ),
            |i| hover(i.config("model", "defaults.warning_color, window.background_color")),
        ),
        ThemeToken::WarningActive => (
            claim(
                "value",
                "warning_active",
                t.warning_active,
                "native-theme-gpui/colors.rs:319",
            ),
            |i| active(i.config("model", "defaults.warning_color")),
        ),
        ThemeToken::Yellow => (
            claim(
                "value",
                "yellow",
                t.yellow,
                "native-theme-gpui/colors.rs:653",
            ),
            |i| i.config("model", "defaults.warning_color"),
        ),
        ThemeToken::YellowLight => (
            claim(
                "value",
                "yellow_light",
                t.yellow_light,
                "native-theme-gpui/colors.rs:654",
            ),
            |i| light(i.config("model", "defaults.warning_color, window.background_color")),
        ),
        ThemeToken::Info => (
            claim("value", "info", t.info, "native-theme-gpui/colors.rs:321"),
            |i| i.config("model", "defaults.info_color"),
        ),
        ThemeToken::InfoForeground => (
            claim(
                "value",
                "info_foreground",
                t.info_foreground,
                "native-theme-gpui/colors.rs:322",
            ),
            |i| i.config("model", "defaults.info_text_color"),
        ),
        ThemeToken::InfoHover => (
            claim(
                "value",
                "info_hover",
                t.info_hover,
                "native-theme-gpui/colors.rs:323",
            ),
            |i| hover(i.config("model", "defaults.info_color, window.background_color")),
        ),
        ThemeToken::InfoActive => (
            claim(
                "value",
                "info_active",
                t.info_active,
                "native-theme-gpui/colors.rs:324",
            ),
            |i| active(i.config("model", "defaults.info_color")),
        ),
        ThemeToken::Blue => (
            claim("value", "blue", t.blue, "native-theme-gpui/colors.rs:651"),
            |i| i.config("model", "defaults.info_color"),
        ),
        ThemeToken::BlueLight => (
            claim(
                "value",
                "blue_light",
                t.blue_light,
                "native-theme-gpui/colors.rs:652",
            ),
            |i| light(i.config("model", "defaults.info_color, window.background_color")),
        ),
        ThemeToken::List => (
            claim(
                "value",
                "list",
                t.colors.list,
                "native-theme-gpui/colors.rs:389",
            ),
            |i| i.config("model", "list.background_color"),
        ),
        ThemeToken::ListActive => (
            claim(
                "value",
                "list_active",
                t.list_active,
                "native-theme-gpui/colors.rs:391",
            ),
            |i| i.config("model", "list.selection_background"),
        ),
        ThemeToken::ListActiveBorder => (
            claim(
                "value",
                "list_active_border",
                t.list_active_border,
                "native-theme-gpui/colors.rs:393",
            ),
            |i| {
                list_active_border(i.config(
                    "model",
                    "button.primary_background, window.background_color",
                ))
            },
        ),
        ThemeToken::ListEven => (
            claim(
                "value",
                "list_even",
                t.list_even,
                "native-theme-gpui/colors.rs:394",
            ),
            |i| i.config("model", "list.alternate_row_background"),
        ),
        ThemeToken::ListHead => (
            claim(
                "value",
                "list_head",
                t.list_head,
                "native-theme-gpui/colors.rs:400",
            ),
            |i| i.config("model", "list.header_background"),
        ),
        ThemeToken::ListHover => (
            claim(
                "value",
                "list_hover",
                t.list_hover,
                "native-theme-gpui/colors.rs:390",
            ),
            |i| i.config("model", "list.hover_background"),
        ),
        ThemeToken::Table => (
            claim("value", "table", t.table, "native-theme-gpui/colors.rs:402"),
            |i| i.config("model", "list.background_color"),
        ),
        ThemeToken::TableActive => (
            claim(
                "value",
                "table_active",
                t.table_active,
                "native-theme-gpui/colors.rs:404",
            ),
            |i| {
                i.config("model", "list.selection_background").config(
                    "copied",
                    "list_active's value (native-theme-gpui/colors.rs, assign_list_table)",
                )
            },
        ),
        ThemeToken::TableActiveBorder => (
            claim(
                "value",
                "table_active_border",
                t.table_active_border,
                "native-theme-gpui/colors.rs:405",
            ),
            |i| {
                list_active_border(i.config("model", "button.primary_background, window.background_color").config("copied", "list_active_border's value (native-theme-gpui/colors.rs, assign_list_table)"))
            },
        ),
        ThemeToken::TableEven => (
            claim(
                "value",
                "table_even",
                t.table_even,
                "native-theme-gpui/colors.rs:406",
            ),
            |i| {
                i.config("model", "list.alternate_row_background").config(
                    "copied",
                    "list_even's value (native-theme-gpui/colors.rs, assign_list_table)",
                )
            },
        ),
        ThemeToken::TableHead => (
            claim(
                "value",
                "table_head",
                t.table_head,
                "native-theme-gpui/colors.rs:407",
            ),
            |i| i.config("model", "list.header_background"),
        ),
        ThemeToken::TableHeadForeground => (
            claim(
                "value",
                "table_head_foreground",
                t.table_head_foreground,
                "native-theme-gpui/colors.rs:411",
            ),
            |i| i.config("model", "list.header_font.color"),
        ),
        ThemeToken::TableFoot => (
            claim(
                "value",
                "table_foot",
                t.table_foot,
                "native-theme-gpui/colors.rs:419",
            ),
            |i| {
                i.config("model", "window.background_color").config("derived", "the model states no footer colour of any kind, so the window's own background (native-theme-gpui/colors.rs, assign_list_table)")
            },
        ),
        ThemeToken::TableFootForeground => (
            claim(
                "value",
                "table_foot_foreground",
                t.table_foot_foreground,
                "native-theme-gpui/colors.rs:420",
            ),
            |i| {
                i.config("model", "defaults.muted_color").config("derived", "the model states no footer colour of any kind, so the muted text colour (native-theme-gpui/colors.rs, assign_list_table)")
            },
        ),
        ThemeToken::TableHover => (
            claim(
                "value",
                "table_hover",
                t.table_hover,
                "native-theme-gpui/colors.rs:403",
            ),
            |i| {
                i.config("model", "list.hover_background").config(
                    "copied",
                    "list_hover's value (native-theme-gpui/colors.rs, assign_list_table)",
                )
            },
        ),
        ThemeToken::TableRowBorder => (
            claim(
                "value",
                "table_row_border",
                t.table_row_border,
                "native-theme-gpui/colors.rs:414",
            ),
            |i| i.config("model", "list.grid_color"),
        ),
        ThemeToken::Tab => (
            claim("value", "tab", t.tab, "native-theme-gpui/colors.rs:425"),
            |i| i.config("model", "tab.background_color"),
        ),
        ThemeToken::TabActive => (
            claim(
                "value",
                "tab_active",
                t.tab_active,
                "native-theme-gpui/colors.rs:426",
            ),
            |i| i.config("model", "tab.active_background"),
        ),
        ThemeToken::TabActiveForeground => (
            claim(
                "value",
                "tab_active_foreground",
                t.tab_active_foreground,
                "native-theme-gpui/colors.rs:427",
            ),
            |i| i.config("model", "tab.active_text_color"),
        ),
        ThemeToken::TabBar => (
            claim(
                "value",
                "tab_bar",
                t.tab_bar,
                "native-theme-gpui/colors.rs:428",
            ),
            |i| i.config("model", "tab.bar_background"),
        ),
        ThemeToken::TabBarSegmented => (
            claim(
                "value",
                "tab_bar_segmented",
                t.tab_bar_segmented,
                "native-theme-gpui/colors.rs:435",
            ),
            |i| i.config("model", "segmented_control.background_color"),
        ),
        ThemeToken::TabForeground => (
            claim(
                "value",
                "tab_foreground",
                t.tab_foreground,
                "native-theme-gpui/colors.rs:436",
            ),
            |i| i.config("model", "tab.font.color"),
        ),
        ThemeToken::Sidebar => (
            claim(
                "value",
                "sidebar",
                t.sidebar,
                "native-theme-gpui/colors.rs:438",
            ),
            |i| i.config("model", "sidebar.background_color"),
        ),
        ThemeToken::SidebarForeground => (
            claim(
                "value",
                "sidebar_foreground",
                t.sidebar_foreground,
                "native-theme-gpui/colors.rs:439",
            ),
            |i| i.config("model", "sidebar.font.color"),
        ),
        ThemeToken::SidebarAccent => (
            claim(
                "value",
                "sidebar_accent",
                t.sidebar_accent,
                "native-theme-gpui/colors.rs:440",
            ),
            |i| i.config("model", "sidebar.selection_background"),
        ),
        ThemeToken::SidebarAccentForeground => (
            claim(
                "value",
                "sidebar_accent_foreground",
                t.sidebar_accent_foreground,
                "native-theme-gpui/colors.rs:441",
            ),
            |i| i.config("model", "sidebar.selection_text_color"),
        ),
        ThemeToken::SidebarBorder => (
            claim(
                "value",
                "sidebar_border",
                t.sidebar_border,
                "native-theme-gpui/colors.rs:442",
            ),
            |i| i.config("model", "window.border.color"),
        ),
        ThemeToken::SidebarPrimary => (
            claim(
                "value",
                "sidebar_primary",
                t.sidebar_primary,
                "native-theme-gpui/colors.rs:443",
            ),
            |i| i.config("model", "button.primary_background"),
        ),
        ThemeToken::SidebarPrimaryForeground => (
            claim(
                "value",
                "sidebar_primary_foreground",
                t.sidebar_primary_foreground,
                "native-theme-gpui/colors.rs:444",
            ),
            |i| i.config("model", "button.primary_text_color"),
        ),
        ThemeToken::Scrollbar => (
            claim(
                "value",
                "scrollbar",
                t.scrollbar,
                "native-theme-gpui/colors.rs:525",
            ),
            |i| i.config("model", "scrollbar.track_color"),
        ),
        ThemeToken::ScrollbarThumb => (
            claim(
                "value",
                "scrollbar_thumb",
                t.scrollbar_thumb,
                "native-theme-gpui/colors.rs:526",
            ),
            |i| i.config("model", "scrollbar.thumb_color"),
        ),
        ThemeToken::ScrollbarThumbHover => (
            claim(
                "value",
                "scrollbar_thumb_hover",
                t.scrollbar_thumb_hover,
                "native-theme-gpui/colors.rs:527",
            ),
            |i| i.config("model", "scrollbar.thumb_hover_color"),
        ),
        ThemeToken::Accordion => (
            claim(
                "value",
                "accordion",
                t.accordion,
                "native-theme-gpui/colors.rs:494",
            ),
            |i| i.config("model", "window.background_color"),
        ),
        ThemeToken::GroupBox => (
            claim(
                "value",
                "group_box",
                t.group_box,
                "native-theme-gpui/colors.rs:498",
            ),
            |i| {
                i.config("model", "button.background_color, window.background_color").config("derived", "button.background_color faded to 40% of its alpha in light mode and to 30% in dark, blended over window.background_color (native-theme-gpui/colors.rs, assign_misc)")
            },
        ),
        ThemeToken::GroupBoxForeground => (
            claim(
                "value",
                "group_box_foreground",
                t.group_box_foreground,
                "native-theme-gpui/colors.rs:500",
            ),
            |i| i.config("model", "defaults.text_color"),
        ),
        ThemeToken::Chart1 => (
            claim(
                "value",
                "chart_1",
                t.chart_1,
                "native-theme-gpui/colors.rs:459",
            ),
            |i| i.config("model", "defaults.accent_color"),
        ),
        ThemeToken::Chart2 => (
            claim(
                "value",
                "chart_2",
                t.chart_2,
                "native-theme-gpui/colors.rs:462",
            ),
            |i| {
                i.config("model", "defaults.accent_color").config("derived", "defaults.accent_color's hue turned 0.2 of the way round the wheel, its saturation raised to at least 0.3, its lightness and alpha kept (native-theme-gpui/colors.rs, assign_charts)")
            },
        ),
        ThemeToken::Chart3 => (
            claim(
                "value",
                "chart_3",
                t.chart_3,
                "native-theme-gpui/colors.rs:467",
            ),
            |i| {
                i.config("model", "defaults.accent_color").config("derived", "defaults.accent_color's hue turned 0.4 of the way round the wheel, its saturation raised to at least 0.3, its lightness and alpha kept (native-theme-gpui/colors.rs, assign_charts)")
            },
        ),
        ThemeToken::Chart4 => (
            claim(
                "value",
                "chart_4",
                t.chart_4,
                "native-theme-gpui/colors.rs:472",
            ),
            |i| {
                i.config("model", "defaults.accent_color").config("derived", "defaults.accent_color's hue turned 0.6 of the way round the wheel, its saturation raised to at least 0.3, its lightness and alpha kept (native-theme-gpui/colors.rs, assign_charts)")
            },
        ),
        ThemeToken::Chart5 => (
            claim(
                "value",
                "chart_5",
                t.chart_5,
                "native-theme-gpui/colors.rs:477",
            ),
            |i| {
                i.config("model", "defaults.accent_color").config("derived", "defaults.accent_color's hue turned 0.8 of the way round the wheel, its saturation raised to at least 0.3, its lightness and alpha kept (native-theme-gpui/colors.rs, assign_charts)")
            },
        ),
        ThemeToken::ChartBullish => (
            claim(
                "value",
                "chart_bullish",
                t.chart_bullish,
                "native-theme-gpui/colors.rs:326",
            ),
            |i| i.config("model", "defaults.success_color"),
        ),
        ThemeToken::ChartBearish => (
            claim(
                "value",
                "chart_bearish",
                t.chart_bearish,
                "native-theme-gpui/colors.rs:327",
            ),
            |i| i.config("model", "defaults.danger_color"),
        ),
        ThemeToken::DescriptionListLabel => (
            claim(
                "value",
                "description_list_label",
                t.description_list_label,
                "native-theme-gpui/colors.rs:503",
            ),
            |i| {
                i.config("model", "defaults.border.color, window.background_color").config("derived", "defaults.border.color faded to 20% of its alpha and blended over window.background_color (native-theme-gpui/colors.rs, assign_misc)")
            },
        ),
        ThemeToken::DescriptionListLabelForeground => (
            claim(
                "value",
                "description_list_label_foreground",
                t.description_list_label_foreground,
                "native-theme-gpui/colors.rs:504",
            ),
            |i| i.config("model", "defaults.muted_color"),
        ),
        ThemeToken::DragBorder => (
            claim(
                "value",
                "drag_border",
                t.drag_border,
                "native-theme-gpui/colors.rs:551",
            ),
            |i| {
                i.config("model", "button.primary_background").config("derived", "button.primary_background faded to 65% of its alpha, so translucent (native-theme-gpui/colors.rs, assign_misc)")
            },
        ),
        ThemeToken::DropTarget => (
            claim(
                "value",
                "drop_target",
                t.drop_target,
                "native-theme-gpui/colors.rs:553",
            ),
            |i| {
                i.config("model", "button.primary_background").config("derived", "button.primary_background faded to 20% of its alpha, so translucent (native-theme-gpui/colors.rs, assign_misc)")
            },
        ),
        ThemeToken::Popover => (
            claim(
                "value",
                "popover",
                t.popover,
                "native-theme-gpui/colors.rs:491",
            ),
            |i| i.config("model", "popover.background_color"),
        ),
        ThemeToken::PopoverForeground => (
            claim(
                "value",
                "popover_foreground",
                t.popover_foreground,
                "native-theme-gpui/colors.rs:492",
            ),
            |i| i.config("model", "popover.font.color"),
        ),
        ThemeToken::ProgressBar => (
            claim(
                "value",
                "progress_bar",
                t.progress_bar,
                "native-theme-gpui/colors.rs:543",
            ),
            |i| i.config("model", "progress_bar.fill_color"),
        ),
        ThemeToken::Skeleton => (
            claim(
                "value",
                "skeleton",
                t.skeleton,
                "native-theme-gpui/colors.rs:548",
            ),
            |i| i.config("model", "button.background_color"),
        ),
        ThemeToken::SliderBar => (
            claim(
                "value",
                "slider_bar",
                t.slider_bar,
                "native-theme-gpui/colors.rs:530",
            ),
            |i| i.config("model", "slider.fill_color"),
        ),
        ThemeToken::SliderThumb => (
            claim(
                "value",
                "slider_thumb",
                t.slider_thumb,
                "native-theme-gpui/colors.rs:531",
            ),
            |i| i.config("model", "slider.thumb_color"),
        ),
        ThemeToken::Switch => (
            claim(
                "value",
                "switch",
                t.switch,
                "native-theme-gpui/colors.rs:534",
            ),
            |i| {
                i.config("model", "switch.unchecked_background").config("derived", "none: the model field as it is. ThemeColor has no field for a checked switch, so the connector maps switch.checked_background nowhere (native-theme-gpui/colors.rs, assign_misc)")
            },
        ),
        ThemeToken::SwitchThumb => (
            claim(
                "value",
                "switch_thumb",
                t.switch_thumb,
                "native-theme-gpui/colors.rs:540",
            ),
            |i| i.config("model", "switch.thumb_background"),
        ),
        ThemeToken::StatusBar => (
            claim(
                "value",
                "status_bar",
                t.status_bar,
                "native-theme-gpui/colors.rs:556",
            ),
            |i| i.config("model", "status_bar.background_color"),
        ),
        ThemeToken::StatusBarBorder => (
            claim(
                "value",
                "status_bar_border",
                t.status_bar_border,
                "native-theme-gpui/colors.rs:557",
            ),
            |i| i.config("model", "status_bar.border.color"),
        ),
        ThemeToken::TitleBar => (
            claim(
                "value",
                "title_bar",
                t.title_bar,
                "native-theme-gpui/colors.rs:447",
            ),
            |i| i.config("model", "window.title_bar_background"),
        ),
        ThemeToken::TitleBarBorder => (
            claim(
                "value",
                "title_bar_border",
                t.title_bar_border,
                "native-theme-gpui/colors.rs:448",
            ),
            |i| i.config("model", "window.border.color"),
        ),
        ThemeToken::WindowBorder => (
            claim(
                "value",
                "window_border",
                t.window_border,
                "native-theme-gpui/colors.rs:449",
            ),
            |i| i.config("model", "window.border.color"),
        ),
        ThemeToken::Magenta => (
            claim(
                "value",
                "magenta",
                t.magenta,
                "native-theme-gpui/colors.rs:663",
            ),
            |i| {
                i.config("model", "defaults.accent_color").config("derived", "hue 0.833, a literal, with defaults.accent_color's lightness and its saturation capped at 0.85, opaque (native-theme-gpui/colors.rs, assign_base_colors)")
            },
        ),
        ThemeToken::MagentaLight => (
            claim(
                "value",
                "magenta_light",
                t.magenta_light,
                "native-theme-gpui/colors.rs:664",
            ),
            |i| {
                light(i.config("model", "defaults.accent_color, window.background_color").config("derived", "magenta as its own swatch states it, through light_variant (native-theme-gpui/colors.rs, assign_base_colors)"))
            },
        ),
        ThemeToken::Cyan => (
            claim("value", "cyan", t.cyan, "native-theme-gpui/colors.rs:673"),
            |i| {
                i.config("model", "defaults.info_color").config("derived", "hue 0.5, a literal, with defaults.info_color's lightness and its saturation capped at 0.85, opaque (native-theme-gpui/colors.rs, assign_base_colors)")
            },
        ),
        ThemeToken::CyanLight => (
            claim(
                "value",
                "cyan_light",
                t.cyan_light,
                "native-theme-gpui/colors.rs:674",
            ),
            |i| {
                light(i.config("model", "defaults.info_color, window.background_color").config("derived", "cyan as its own swatch states it, through light_variant (native-theme-gpui/colors.rs, assign_base_colors)"))
            },
        ),
    };
    Row { value, notes }
}

/// Whether the connector writes `field` as one of the 12 base-palette
/// colours, which a `ThemeConfig` cannot carry.
fn base_palette(field: &str) -> bool {
    matches!(
        field,
        "red"
            | "red_light"
            | "green"
            | "green_light"
            | "blue"
            | "blue_light"
            | "yellow"
            | "yellow_light"
            | "magenta"
            | "magenta_light"
            | "cyan"
            | "cyan_light"
    )
}

/// The Theme Map's swatch of `token`. `native` is whether a native theme is
/// installed for the current mode, which is when the connector wrote the
/// value.
pub fn swatch(t: &Theme, token: ThemeToken, native: bool) -> WidgetInfo {
    let Row { value, notes } = row(t, token);
    let field = value.field;
    let hex = hsla_to_hex(value.value);
    let alpha = value.value.a;
    let info = WidgetInfo::new("ThemeColor").variant(field);
    let info = if native {
        let info = notes(info.color(value));
        if base_palette(field) {
            info.config("installed", "to_theme_color writes it, and a ThemeConfig cannot carry it, so the connector writes it again after every rebuild upstream makes (native-theme-gpui/lib.rs, repair_base_palette)")
        } else {
            info.config("installed", "to_theme puts it on the Theme as to_theme_color wrote it, and the ThemeConfig stored for a light/dark switch carries it as hex, so after a switch it is rounded to 8 bits a channel (native-theme-gpui/lib.rs, to_theme; native-theme-gpui/config.rs, theme_color_to_config_colors)")
        }
    } else {
        info.instance(
            "value",
            format!(
                "{hex}, gpui-component's own: no native theme is installed for this mode, so the connector wrote none of this page's values"
            ),
        )
    };
    let info = if alpha < 1. {
        info.instance(
            "translucent",
            format!(
                "{} alpha: the swatch paints it over the page's background, which shows through",
                percent_text(alpha)
            ),
        )
    } else {
        info
    };
    info.color(claim("frame", "border", t.border, "showcase"))
        .color(claim(
            "label",
            "foreground",
            t.foreground,
            "gpui-component/label.rs:211",
        ))
        .not_themeable(
            "geometry",
            "none: a square of the showcase's own in its frame, which reads Theme::border, Theme::radius and the platform's defaults.border.line_width (showcase-gpui/support.rs, demo_frame)",
        )
        .instance("label", format!("{field} {hex}"))
}

/// The Theme Map's row stating the control height of `widget`, which
/// `geometry::control_height` computed as `height` where a native theme is
/// installed.
pub fn control_height(
    t: &Theme,
    widget: ControlWidget,
    height: Option<&ControlHeight>,
) -> WidgetInfo {
    let w = widget.name();
    let info = text::label_of(format!("control height, {w}")).color(claim(
        "text",
        "foreground",
        t.foreground,
        "gpui-component/label.rs:211",
    ));
    let info = match height {
        Some(h) => info
            .config(
                "height",
                format!(
                    "{}px: the larger of {w}.min_height {}px and ceil({w}.font.size {}px × the text-scaling factor × defaults.line_height {}) + 2 × {w}.border.padding_vertical {}px (native-theme-gpui/lib.rs, text_scale_factor)",
                    px_text(h.height),
                    px_text(h.min_height),
                    px_text(h.font_size),
                    h.line_height,
                    px_text(h.padding_vertical),
                ),
            )
            .instance(
                "used by",
                match widget {
                    ControlWidget::Button => {
                        "geometry::button, as the Button's height (native-theme-gpui/geometry.rs, button)"
                    }
                    ControlWidget::Input => {
                        "geometry::input and geometry::input_height, as the Input's height (native-theme-gpui/geometry.rs, input_height)"
                    }
                },
            ),
        None => info.instance(
            "height",
            "none: no native theme is installed for this mode, so the connector computes no control height",
        ),
    };
    info.instance(
        "style",
        "text_sm -- a rem, so it follows the platform's font -- in the foreground the Label paints itself (label.rs, Label::render)",
    )
}
