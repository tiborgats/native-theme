//! What the Theme Map's rows report about themselves (spec §3.4).
//!
//! A swatch is one ThemeColor field as the installed theme holds it. Where
//! a 0.8.0 theme is installed for the mode, the connector wrote it: the
//! value's claim cites the connector line that writes the field, and the
//! notes name the homes of the 0.8.0 model that line's value is read from
//! -- the globals, and each widget's part, state and property, as the
//! connector reads them (native-theme-gpui next/mod.rs, slice_colors and
//! widget_colors) -- and how it is made from them. Where the making branches
//! on the mode, the note says how in both; where it branches on what the
//! theme states, the swatch's "branch" note says which way it went.

use gpui_component::theme::Theme;
use native_theme::next::{PartStated, ResolvedTheme as NextTheme, WidgetState};
use native_theme_gpui::next::state_fill_of;

use super::{ColorClaim, WidgetInfo, as_rgba, claim, percent_text, px_text};
use crate::demo::ThemeToken;

/// `hover_color`, which makes the field from the model's colour.
fn hover(info: WidgetInfo) -> WidgetInfo {
    info.config(
        "hover_color",
        "the colour faded to 90% of its alpha and blended over window.frame(normal).fill (native-theme-gpui/derive.rs, hover_color)",
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
        "in dark mode the colour's lightness raised by 0.15, to at most 0.95; in light mode the colour faded to 80% of its alpha and blended over window.frame(normal).fill (native-theme-gpui/derive.rs, light_variant)",
    )
}

/// How list_active_border is made, which table_active_border copies.
fn list_active_border(info: WidgetInfo) -> WidgetInfo {
    info.config(
        "derived",
        "button.body(NONE; variant=Primary).fill faded to 60% of its alpha and blended over window.frame(normal).fill (native-theme-gpui/colors.rs, assign_list_table)",
    )
}

/// One row of the swatch table: the installed value of its field, claimed
/// at the connector line that writes it, and the notes naming the 0.8.0
/// homes the connector reads for it (native-theme-gpui next/mod.rs,
/// slice_colors and widget_colors) and how it makes the value from them,
/// where it is not the home's value as it is.
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
                "native-theme-gpui/colors.rs:456",
            ),
            |i| i.config("model", "window.frame(normal).fill"),
        ),
        ThemeToken::Foreground => (
            claim(
                "value",
                "foreground",
                t.foreground,
                "native-theme-gpui/colors.rs:449",
            ),
            |i| i.config("model", "colors.window.foreground"),
        ),
        ThemeToken::Accent => (
            claim(
                "value",
                "accent",
                t.accent,
                "native-theme-gpui/colors.rs:463",
            ),
            |i| {
                i.config("model", "menu.item(HOVER).fill").config("derived", "none: the model field as it is. gpui-component's accent is its item highlight, not the platform's accent colour, so it takes the menu's hover (native-theme-gpui/colors.rs, assign_core)")
            },
        ),
        ThemeToken::AccentForeground => (
            claim(
                "value",
                "accent_foreground",
                t.accent_foreground,
                "native-theme-gpui/colors.rs:464",
            ),
            |i| i.config("model", "menu.item_label(HOVER).color"),
        ),
        ThemeToken::Border => (
            claim(
                "value",
                "border",
                t.border,
                "native-theme-gpui/colors.rs:450",
            ),
            |i| i.config("model", "colors.window.border"),
        ),
        ThemeToken::Muted => (
            claim("value", "muted", t.muted, "native-theme-gpui/colors.rs:465"),
            |i| {
                i.config("model", "colors.window.foreground, window.frame(normal).fill").config("derived", "colors.window.foreground faded to 10% of its alpha and blended over window.frame(normal).fill (native-theme-gpui/colors.rs, to_theme_color)")
            },
        ),
        ThemeToken::MutedForeground => (
            claim(
                "value",
                "muted_foreground",
                t.muted_foreground,
                "native-theme-gpui/colors.rs:451",
            ),
            |i| i.config("model", "colors.window.foreground_muted"),
        ),
        ThemeToken::Input => (
            claim("value", "input", t.input, "native-theme-gpui/colors.rs:466"),
            |i| i.config("model", "input.field(NONE).stroke.color"),
        ),
        ThemeToken::Ring => (
            claim("value", "ring", t.ring, "native-theme-gpui/colors.rs:452"),
            |i| {
                i.config("model", "colors.focus_ring, focus_ring.stroke.width, focus_ring.trigger").config("derived", "the colour as stated where gpui-component's band is on for any focus; made opaque where the theme says keyboard only or its ring width is not above 0: there ring colours a focused control's border or line and a Tab's keyboard ring, and the band after a key press takes it with its alpha set to 0.5 (native-theme-gpui/colors.rs, ring_colour)")
            },
        ),
        ThemeToken::Selection => (
            claim(
                "value",
                "selection",
                t.selection,
                "native-theme-gpui/colors.rs:467",
            ),
            |i| {
                i.config("model", "colors.text_selection.background").not_themeable(
                    "unfocused",
                    "not mapped: colors.text_selection.unfocused_background and unfocused_foreground, a text selection's colours while its window is not active: an Input hands its editor this one global selection at every render, whatever the window's state, and no text colour for the selected text (gpui-component/input/input.rs, Input::render, set_editor_style), so a selection in an inactive window cannot take others",
                )
            },
        ),
        ThemeToken::Caret => (
            claim("value", "caret", t.caret, "native-theme-gpui/colors.rs:775"),
            |i| i.config("model", "colors.caret"),
        ),
        ThemeToken::Link => (
            claim("value", "link", t.link, "native-theme-gpui/colors.rs:468"),
            |i| i.config("model", "link.label(NONE).color"),
        ),
        ThemeToken::LinkHover => (
            claim(
                "value",
                "link_hover",
                t.link_hover,
                "native-theme-gpui/colors.rs:475",
            ),
            |i| i.config("model", "link.label(HOVER).color"),
        ),
        ThemeToken::LinkActive => (
            claim(
                "value",
                "link_active",
                t.link_active,
                "native-theme-gpui/colors.rs:476",
            ),
            |i| i.config("model", "link.label(PRESSED).color"),
        ),
        ThemeToken::Overlay => (
            claim(
                "value",
                "overlay",
                t.overlay,
                "native-theme-gpui/colors.rs:720",
            ),
            |i| {
                i.config("model", "colors.shadow").config("derived", "transparent black where reduced transparency is asked for: upstream's own colour for no overlay (dialog/dialog.rs:335-337), so no scrim is drawn; else colors.shadow's hue, saturation and lightness at an alpha of 0.5 in dark mode and 0.4 in light (native-theme-gpui/colors.rs, assign_misc)")
            },
        ),
        ThemeToken::Primary => (
            claim(
                "value",
                "primary",
                t.primary,
                "native-theme-gpui/colors.rs:480",
            ),
            |i| i.config("model", "button.body(NONE; variant=Primary).fill"),
        ),
        ThemeToken::PrimaryForeground => (
            claim(
                "value",
                "primary_foreground",
                t.primary_foreground,
                "native-theme-gpui/colors.rs:481",
            ),
            |i| i.config("model", "button.label(NONE; variant=Primary).color"),
        ),
        ThemeToken::PrimaryHover => (
            claim(
                "value",
                "primary_hover",
                t.primary_hover,
                "native-theme-gpui/colors.rs:490",
            ),
            |i| {
                hover(i.config(
                    "model",
                    "button.body(NONE; variant=Primary).fill, window.frame(normal).fill",
                ))
            },
        ),
        ThemeToken::PrimaryActive => (
            claim(
                "value",
                "primary_active",
                t.primary_active,
                "native-theme-gpui/colors.rs:482",
            ),
            |i| active(i.config("model", "button.body(NONE; variant=Primary).fill")),
        ),
        ThemeToken::Secondary => (
            claim(
                "value",
                "secondary",
                t.secondary,
                "native-theme-gpui/colors.rs:504",
            ),
            |i| i.config("model", "button.body(NONE).fill"),
        ),
        ThemeToken::SecondaryForeground => (
            claim(
                "value",
                "secondary_foreground",
                t.secondary_foreground,
                "native-theme-gpui/colors.rs:505",
            ),
            |i| i.config("model", "button.label(NONE).color"),
        ),
        ThemeToken::SecondaryHover => (
            claim(
                "value",
                "secondary_hover",
                t.secondary_hover,
                "native-theme-gpui/colors.rs:506",
            ),
            |i| i.config("model", "button.body(HOVER).fill"),
        ),
        ThemeToken::SecondaryActive => (
            claim(
                "value",
                "secondary_active",
                t.secondary_active,
                "native-theme-gpui/colors.rs:507",
            ),
            |i| {
                active(i.config("model", "button.body(PRESSED).fill, else button.body(NONE).fill").config("derived", "button.body(PRESSED).fill where a rule states it for PRESSED; where none does, button.body(NONE).fill through active_color (native-theme-gpui/colors.rs, assign_secondary)"))
            },
        ),
        ThemeToken::Button => (
            claim(
                "value",
                "button",
                t.button,
                "native-theme-gpui/colors.rs:549",
            ),
            |i| {
                i.config("model", "button.body(NONE).fill").config(
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
                "native-theme-gpui/colors.rs:564",
            ),
            |i| {
                i.config("model", "button.body(HOVER).fill, button.body(NONE).fill").config("derived", "secondary_hover laid over button, source-over: an opaque secondary_hover stands as it is, and a translucent one is composited over the idle fill, covering the layer's alpha plus the fill's times what the layer leaves (native-theme-gpui/colors.rs, over)")
            },
        ),
        ThemeToken::ButtonActive => (
            claim(
                "value",
                "button_active",
                t.button_active,
                "native-theme-gpui/colors.rs:565",
            ),
            |i| {
                active(i.config("model", "button.body(NONE).fill, the composite's base; and button.body(PRESSED).fill, else button.body(NONE).fill, for secondary_active").config("derived", "secondary_active laid over button, source-over: an opaque secondary_active stands as it is, and a translucent one is composited over the idle fill, covering the layer's alpha plus the fill's times what the layer leaves (native-theme-gpui/colors.rs, over)"))
            },
        ),
        ThemeToken::ButtonForeground => {
            (
                claim(
                    "value",
                    "button_foreground",
                    t.button_foreground,
                    "native-theme-gpui/colors.rs:566",
                ),
                |i| {
                    i.config("model", "button.label(NONE).color").config("copied", "secondary_foreground's value (native-theme-gpui/colors.rs, assign_buttons)")
                },
            )
        }
        ThemeToken::ButtonSecondary => (
            claim(
                "value",
                "button_secondary",
                t.button_secondary,
                "native-theme-gpui/colors.rs:567",
            ),
            |i| {
                i.config("model", "button.body(NONE).fill").config(
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
                "native-theme-gpui/colors.rs:568",
            ),
            |i| {
                i.config("model", "button.body(HOVER).fill, button.body(NONE).fill").config("derived", "secondary_hover laid over button_secondary, source-over: an opaque secondary_hover stands as it is, and a translucent one is composited over the idle fill, covering the layer's alpha plus the fill's times what the layer leaves (native-theme-gpui/colors.rs, over)")
            },
        ),
        ThemeToken::ButtonSecondaryActive => (
            claim(
                "value",
                "button_secondary_active",
                t.button_secondary_active,
                "native-theme-gpui/colors.rs:569",
            ),
            |i| {
                active(i.config("model", "button.body(NONE).fill, the composite's base; and button.body(PRESSED).fill, else button.body(NONE).fill, for secondary_active").config("derived", "secondary_active laid over button_secondary, source-over: an opaque secondary_active stands as it is, and a translucent one is composited over the idle fill, covering the layer's alpha plus the fill's times what the layer leaves (native-theme-gpui/colors.rs, over)"))
            },
        ),
        ThemeToken::ButtonSecondaryForeground => {
            (
                claim(
                    "value",
                    "button_secondary_foreground",
                    t.button_secondary_foreground,
                    "native-theme-gpui/colors.rs:570",
                ),
                |i| {
                    i.config("model", "button.label(NONE).color").config("copied", "secondary_foreground's value (native-theme-gpui/colors.rs, assign_buttons)")
                },
            )
        }
        ThemeToken::ButtonPrimary => (
            claim(
                "value",
                "button_primary",
                t.button_primary,
                "native-theme-gpui/colors.rs:572",
            ),
            |i| {
                i.config("model", "button.body(NONE; variant=Primary).fill")
                    .config(
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
                "native-theme-gpui/colors.rs:496",
            ),
            |i| {
                hover(
                    i.config(
                        "model",
                        "button.body(NONE; variant=Primary).fill, window.frame(normal).fill",
                    )
                    .config(
                        "copied",
                        "primary_hover's value (native-theme-gpui/colors.rs, assign_buttons)",
                    ),
                )
            },
        ),
        ThemeToken::ButtonPrimaryActive => {
            (
                claim(
                    "value",
                    "button_primary_active",
                    t.button_primary_active,
                    "native-theme-gpui/colors.rs:573",
                ),
                |i| {
                    active(i.config("model", "button.body(NONE; variant=Primary).fill").config(
                    "copied",
                    "primary_active's value (native-theme-gpui/colors.rs, assign_buttons)",
                ))
                },
            )
        }
        ThemeToken::ButtonPrimaryForeground => (
            claim(
                "value",
                "button_primary_foreground",
                t.button_primary_foreground,
                "native-theme-gpui/colors.rs:574",
            ),
            |i| {
                i.config("model", "button.label(NONE; variant=Primary).color")
                    .config(
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
                "native-theme-gpui/colors.rs:576",
            ),
            |i| {
                i.config("model", "colors.error.fill").config(
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
                    "native-theme-gpui/colors.rs:497",
                ),
                |i| {
                    hover(i.config("model", "colors.error.fill, window.frame(normal).fill").config("copied", "danger_hover's value (native-theme-gpui/colors.rs, assign_buttons)"))
                },
            )
        }
        ThemeToken::ButtonDangerActive => (
            claim(
                "value",
                "button_danger_active",
                t.button_danger_active,
                "native-theme-gpui/colors.rs:577",
            ),
            |i| {
                active(i.config("model", "colors.error.fill").config(
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
                "native-theme-gpui/colors.rs:578",
            ),
            |i| {
                i.config("model", "colors.error.foreground").config(
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
                "native-theme-gpui/colors.rs:580",
            ),
            |i| {
                i.config("model", "colors.info.fill").config(
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
                "native-theme-gpui/colors.rs:498",
            ),
            |i| {
                hover(
                    i.config("model", "colors.info.fill, window.frame(normal).fill")
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
                "native-theme-gpui/colors.rs:581",
            ),
            |i| {
                active(i.config("model", "colors.info.fill").config(
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
                "native-theme-gpui/colors.rs:582",
            ),
            |i| {
                i.config("model", "colors.info.foreground").config(
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
                "native-theme-gpui/colors.rs:584",
            ),
            |i| {
                i.config("model", "colors.success.fill").config(
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
                    "native-theme-gpui/colors.rs:499",
                ),
                |i| {
                    hover(i.config("model", "colors.success.fill, window.frame(normal).fill").config("copied", "success_hover's value (native-theme-gpui/colors.rs, assign_buttons)"))
                },
            )
        }
        ThemeToken::ButtonSuccessActive => (
            claim(
                "value",
                "button_success_active",
                t.button_success_active,
                "native-theme-gpui/colors.rs:585",
            ),
            |i| {
                active(i.config("model", "colors.success.fill").config(
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
                "native-theme-gpui/colors.rs:586",
            ),
            |i| {
                i.config("model", "colors.success.foreground").config(
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
                "native-theme-gpui/colors.rs:588",
            ),
            |i| {
                i.config("model", "colors.warning.fill").config(
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
                    "native-theme-gpui/colors.rs:500",
                ),
                |i| {
                    hover(i.config("model", "colors.warning.fill, window.frame(normal).fill").config("copied", "warning_hover's value (native-theme-gpui/colors.rs, assign_buttons)"))
                },
            )
        }
        ThemeToken::ButtonWarningActive => (
            claim(
                "value",
                "button_warning_active",
                t.button_warning_active,
                "native-theme-gpui/colors.rs:589",
            ),
            |i| {
                active(i.config("model", "colors.warning.fill").config(
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
                "native-theme-gpui/colors.rs:590",
            ),
            |i| {
                i.config("model", "colors.warning.foreground").config(
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
                "native-theme-gpui/colors.rs:521",
            ),
            |i| i.config("model", "colors.error.fill"),
        ),
        ThemeToken::DangerForeground => (
            claim(
                "value",
                "danger_foreground",
                t.danger_foreground,
                "native-theme-gpui/colors.rs:522",
            ),
            |i| i.config("model", "colors.error.foreground"),
        ),
        ThemeToken::DangerHover => (
            claim(
                "value",
                "danger_hover",
                t.danger_hover,
                "native-theme-gpui/colors.rs:491",
            ),
            |i| hover(i.config("model", "colors.error.fill, window.frame(normal).fill")),
        ),
        ThemeToken::DangerActive => (
            claim(
                "value",
                "danger_active",
                t.danger_active,
                "native-theme-gpui/colors.rs:523",
            ),
            |i| active(i.config("model", "colors.error.fill")),
        ),
        ThemeToken::Red => (
            claim("value", "red", t.red, "native-theme-gpui/colors.rs:878"),
            |i| i.config("model", "colors.error.fill"),
        ),
        ThemeToken::RedLight => (
            claim(
                "value",
                "red_light",
                t.red_light,
                "native-theme-gpui/colors.rs:906",
            ),
            |i| light(i.config("model", "colors.error.fill, window.frame(normal).fill")),
        ),
        ThemeToken::Success => (
            claim(
                "value",
                "success",
                t.success,
                "native-theme-gpui/colors.rs:525",
            ),
            |i| i.config("model", "colors.success.fill"),
        ),
        ThemeToken::SuccessForeground => (
            claim(
                "value",
                "success_foreground",
                t.success_foreground,
                "native-theme-gpui/colors.rs:526",
            ),
            |i| i.config("model", "colors.success.foreground"),
        ),
        ThemeToken::SuccessHover => (
            claim(
                "value",
                "success_hover",
                t.success_hover,
                "native-theme-gpui/colors.rs:492",
            ),
            |i| hover(i.config("model", "colors.success.fill, window.frame(normal).fill")),
        ),
        ThemeToken::SuccessActive => (
            claim(
                "value",
                "success_active",
                t.success_active,
                "native-theme-gpui/colors.rs:527",
            ),
            |i| active(i.config("model", "colors.success.fill")),
        ),
        ThemeToken::Green => (
            claim("value", "green", t.green, "native-theme-gpui/colors.rs:879"),
            |i| i.config("model", "colors.success.fill"),
        ),
        ThemeToken::GreenLight => (
            claim(
                "value",
                "green_light",
                t.green_light,
                "native-theme-gpui/colors.rs:907",
            ),
            |i| light(i.config("model", "colors.success.fill, window.frame(normal).fill")),
        ),
        ThemeToken::Warning => (
            claim(
                "value",
                "warning",
                t.warning,
                "native-theme-gpui/colors.rs:529",
            ),
            |i| i.config("model", "colors.warning.fill"),
        ),
        ThemeToken::WarningForeground => (
            claim(
                "value",
                "warning_foreground",
                t.warning_foreground,
                "native-theme-gpui/colors.rs:530",
            ),
            |i| i.config("model", "colors.warning.foreground"),
        ),
        ThemeToken::WarningHover => (
            claim(
                "value",
                "warning_hover",
                t.warning_hover,
                "native-theme-gpui/colors.rs:493",
            ),
            |i| hover(i.config("model", "colors.warning.fill, window.frame(normal).fill")),
        ),
        ThemeToken::WarningActive => (
            claim(
                "value",
                "warning_active",
                t.warning_active,
                "native-theme-gpui/colors.rs:531",
            ),
            |i| active(i.config("model", "colors.warning.fill")),
        ),
        ThemeToken::Yellow => (
            claim(
                "value",
                "yellow",
                t.yellow,
                "native-theme-gpui/colors.rs:881",
            ),
            |i| i.config("model", "colors.warning.fill"),
        ),
        ThemeToken::YellowLight => (
            claim(
                "value",
                "yellow_light",
                t.yellow_light,
                "native-theme-gpui/colors.rs:909",
            ),
            |i| light(i.config("model", "colors.warning.fill, window.frame(normal).fill")),
        ),
        ThemeToken::Info => (
            claim("value", "info", t.info, "native-theme-gpui/colors.rs:533"),
            |i| i.config("model", "colors.info.fill"),
        ),
        ThemeToken::InfoForeground => (
            claim(
                "value",
                "info_foreground",
                t.info_foreground,
                "native-theme-gpui/colors.rs:534",
            ),
            |i| i.config("model", "colors.info.foreground"),
        ),
        ThemeToken::InfoHover => (
            claim(
                "value",
                "info_hover",
                t.info_hover,
                "native-theme-gpui/colors.rs:494",
            ),
            |i| hover(i.config("model", "colors.info.fill, window.frame(normal).fill")),
        ),
        ThemeToken::InfoActive => (
            claim(
                "value",
                "info_active",
                t.info_active,
                "native-theme-gpui/colors.rs:535",
            ),
            |i| active(i.config("model", "colors.info.fill")),
        ),
        ThemeToken::Blue => (
            claim("value", "blue", t.blue, "native-theme-gpui/colors.rs:880"),
            |i| i.config("model", "colors.info.fill"),
        ),
        ThemeToken::BlueLight => (
            claim(
                "value",
                "blue_light",
                t.blue_light,
                "native-theme-gpui/colors.rs:908",
            ),
            |i| light(i.config("model", "colors.info.fill, window.frame(normal).fill")),
        ),
        ThemeToken::List => (
            claim(
                "value",
                "list",
                t.colors.list,
                "native-theme-gpui/colors.rs:594",
            ),
            |i| i.config("model", "list.container(NONE).fill"),
        ),
        ThemeToken::ListActive => (
            claim(
                "value",
                "list_active",
                t.list_active,
                "native-theme-gpui/colors.rs:596",
            ),
            |i| i.config("model", "list.highlight(SELECTED).fill"),
        ),
        ThemeToken::ListActiveBorder => (
            claim(
                "value",
                "list_active_border",
                t.list_active_border,
                "native-theme-gpui/colors.rs:629",
            ),
            |i| {
                list_active_border(i.config(
                    "model",
                    "button.body(NONE; variant=Primary).fill, window.frame(normal).fill",
                ))
            },
        ),
        ThemeToken::ListEven => (
            claim(
                "value",
                "list_even",
                t.list_even,
                "native-theme-gpui/colors.rs:597",
            ),
            |i| i.config("model", "list.row(NONE; alternate=true).fill"),
        ),
        ThemeToken::ListHead => (
            claim(
                "value",
                "list_head",
                t.list_head,
                "native-theme-gpui/colors.rs:603",
            ),
            |i| i.config("model", "list.header(NONE).fill"),
        ),
        ThemeToken::ListHover => (
            claim(
                "value",
                "list_hover",
                t.list_hover,
                "native-theme-gpui/colors.rs:595",
            ),
            |i| {
                i.config("model", "list.highlight(HOVER).fill, else menu.item(HOVER).fill").config("derived", "list.highlight(HOVER).fill where the theme states it; where it does not, gpui-component's own hovered row, its accent at 60% of its alpha, the accent the connector writes being menu.item(HOVER).fill (native-theme-gpui next/mod.rs, list_row_hover)")
            },
        ),
        ThemeToken::ListHighlightForeground => (
            claim(
                "value",
                "list_highlight_foreground",
                t.list_highlight_foreground,
                "native-theme-gpui/colors.rs:609",
            ),
            |i| {
                i.config("model", "colors.window.foreground").config("derived", "the model states no colour for the part of a row that matches a search, so the row's resting text colour, foreground's value (native-theme-gpui/colors.rs, assign_list_table)")
            },
        ),
        ThemeToken::Table => (
            claim("value", "table", t.table, "native-theme-gpui/colors.rs:611"),
            |i| i.config("model", "list.container(NONE).fill"),
        ),
        ThemeToken::TableActive => (
            claim(
                "value",
                "table_active",
                t.table_active,
                "native-theme-gpui/colors.rs:613",
            ),
            |i| {
                i.config("model", "list.highlight(SELECTED).fill").config(
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
                "native-theme-gpui/colors.rs:630",
            ),
            |i| {
                list_active_border(i.config("model", "button.body(NONE; variant=Primary).fill, window.frame(normal).fill").config("copied", "list_active_border's value (native-theme-gpui/colors.rs, assign_list_table)"))
            },
        ),
        ThemeToken::TableEven => (
            claim(
                "value",
                "table_even",
                t.table_even,
                "native-theme-gpui/colors.rs:614",
            ),
            |i| {
                i.config("model", "list.row(NONE; alternate=true).fill")
                    .config(
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
                "native-theme-gpui/colors.rs:615",
            ),
            |i| i.config("model", "list.header(NONE).fill"),
        ),
        ThemeToken::TableHeadForeground => (
            claim(
                "value",
                "table_head_foreground",
                t.table_head_foreground,
                "native-theme-gpui/colors.rs:619",
            ),
            |i| i.config("model", "list.header_label(NONE).color"),
        ),
        ThemeToken::TableFoot => (
            claim(
                "value",
                "table_foot",
                t.table_foot,
                "native-theme-gpui/colors.rs:641",
            ),
            |i| {
                i.config("model", "window.frame(normal).fill").config("derived", "the model states no footer colour of any kind, so the window's own background (native-theme-gpui/colors.rs, assign_list_table)")
            },
        ),
        ThemeToken::TableFootForeground => (
            claim(
                "value",
                "table_foot_foreground",
                t.table_foot_foreground,
                "native-theme-gpui/colors.rs:624",
            ),
            |i| {
                i.config("model", "colors.window.foreground_muted").config("derived", "the model states no footer colour of any kind, so the muted text colour (native-theme-gpui/colors.rs, assign_list_table)")
            },
        ),
        ThemeToken::TableHover => (
            claim(
                "value",
                "table_hover",
                t.table_hover,
                "native-theme-gpui/colors.rs:612",
            ),
            |i| {
                i.config(
                    "model",
                    "list.highlight(HOVER).fill, else menu.item(HOVER).fill",
                )
                .config(
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
                "native-theme-gpui/colors.rs:622",
            ),
            |i| i.config("model", "list.grid_color"),
        ),
        ThemeToken::Tab => (
            claim("value", "tab", t.tab, "native-theme-gpui/colors.rs:646"),
            |i| i.config("model", "tab.tab(NONE).fill"),
        ),
        ThemeToken::TabActive => (
            claim(
                "value",
                "tab_active",
                t.tab_active,
                "native-theme-gpui/colors.rs:647",
            ),
            |i| i.config("model", "tab.tab(SELECTED).fill"),
        ),
        ThemeToken::TabActiveForeground => (
            claim(
                "value",
                "tab_active_foreground",
                t.tab_active_foreground,
                "native-theme-gpui/colors.rs:648",
            ),
            |i| i.config("model", "tab.label(SELECTED).color"),
        ),
        ThemeToken::TabBar => (
            claim(
                "value",
                "tab_bar",
                t.tab_bar,
                "native-theme-gpui/colors.rs:649",
            ),
            |i| i.config("model", "tab.bar(NONE).fill"),
        ),
        ThemeToken::TabBarSegmented => (
            claim(
                "value",
                "tab_bar_segmented",
                t.tab_bar_segmented,
                "native-theme-gpui/colors.rs:656",
            ),
            |i| i.config("model", "segmented_control.track(NONE).fill"),
        ),
        ThemeToken::TabForeground => (
            claim(
                "value",
                "tab_foreground",
                t.tab_foreground,
                "native-theme-gpui/colors.rs:657",
            ),
            |i| i.config("model", "tab.label(NONE).color"),
        ),
        ThemeToken::Sidebar => (
            claim(
                "value",
                "sidebar",
                t.sidebar,
                "native-theme-gpui/colors.rs:659",
            ),
            |i| i.config("model", "sidebar.container(NONE).fill"),
        ),
        ThemeToken::SidebarForeground => (
            claim(
                "value",
                "sidebar_foreground",
                t.sidebar_foreground,
                "native-theme-gpui/colors.rs:660",
            ),
            |i| i.config("model", "sidebar.item_label(NONE).color"),
        ),
        ThemeToken::SidebarAccent => (
            claim(
                "value",
                "sidebar_accent",
                t.sidebar_accent,
                "native-theme-gpui/colors.rs:661",
            ),
            |i| i.config("model", "sidebar.item(SELECTED).fill"),
        ),
        ThemeToken::SidebarAccentForeground => (
            claim(
                "value",
                "sidebar_accent_foreground",
                t.sidebar_accent_foreground,
                "native-theme-gpui/colors.rs:662",
            ),
            |i| i.config("model", "sidebar.item_label(SELECTED).color"),
        ),
        ThemeToken::SidebarBorder => (
            claim(
                "value",
                "sidebar_border",
                t.sidebar_border,
                "native-theme-gpui/colors.rs:663",
            ),
            |i| i.config("model", "sidebar.container(NONE).stroke.color"),
        ),
        ThemeToken::SidebarPrimary => (
            claim(
                "value",
                "sidebar_primary",
                t.sidebar_primary,
                "native-theme-gpui/colors.rs:483",
            ),
            |i| i.config("model", "button.body(NONE; variant=Primary).fill"),
        ),
        ThemeToken::SidebarPrimaryForeground => (
            claim(
                "value",
                "sidebar_primary_foreground",
                t.sidebar_primary_foreground,
                "native-theme-gpui/colors.rs:484",
            ),
            |i| i.config("model", "button.label(NONE; variant=Primary).color"),
        ),
        ThemeToken::Scrollbar => (
            claim(
                "value",
                "scrollbar",
                t.scrollbar,
                "native-theme-gpui/colors.rs:732",
            ),
            |i| i.config("model", "scrollbar.track(NONE).fill"),
        ),
        ThemeToken::ScrollbarThumb => (
            claim(
                "value",
                "scrollbar_thumb",
                t.scrollbar_thumb,
                "native-theme-gpui/colors.rs:733",
            ),
            |i| i.config("model", "scrollbar.thumb(UNFOCUSED_VIEW).fill"),
        ),
        ThemeToken::ScrollbarThumbHover => (
            claim(
                "value",
                "scrollbar_thumb_hover",
                t.scrollbar_thumb_hover,
                "native-theme-gpui/colors.rs:734",
            ),
            |i| i.config("model", "scrollbar.thumb(HOVER).fill"),
        ),
        ThemeToken::Accordion => (
            claim(
                "value",
                "accordion",
                t.accordion,
                "native-theme-gpui/colors.rs:748",
            ),
            |i| i.config("model", "window.frame(normal).fill"),
        ),
        ThemeToken::GroupBox => (
            claim(
                "value",
                "group_box",
                t.group_box,
                "native-theme-gpui/colors.rs:752",
            ),
            |i| {
                i.config("model", "button.body(NONE).fill, window.frame(normal).fill").config("derived", "button.body(NONE).fill faded to 40% of its alpha in light mode and to 30% in dark, blended over window.frame(normal).fill (native-theme-gpui/colors.rs, assign_misc)")
            },
        ),
        ThemeToken::GroupBoxForeground => (
            claim(
                "value",
                "group_box_foreground",
                t.group_box_foreground,
                "native-theme-gpui/colors.rs:709",
            ),
            |i| i.config("model", "colors.window.foreground"),
        ),
        ThemeToken::Chart1 => (
            claim(
                "value",
                "chart_1",
                t.chart_1,
                "native-theme-gpui/colors.rs:678",
            ),
            |i| i.config("model", "colors.accent.fill"),
        ),
        ThemeToken::Chart2 => (
            claim(
                "value",
                "chart_2",
                t.chart_2,
                "native-theme-gpui/colors.rs:681",
            ),
            |i| {
                i.config("model", "colors.accent.fill").config("derived", "colors.accent.fill's hue turned 0.2 of the way round the wheel, its saturation raised to at least 0.3, its lightness and alpha kept (native-theme-gpui/colors.rs, assign_charts)")
            },
        ),
        ThemeToken::Chart3 => (
            claim(
                "value",
                "chart_3",
                t.chart_3,
                "native-theme-gpui/colors.rs:686",
            ),
            |i| {
                i.config("model", "colors.accent.fill").config("derived", "colors.accent.fill's hue turned 0.4 of the way round the wheel, its saturation raised to at least 0.3, its lightness and alpha kept (native-theme-gpui/colors.rs, assign_charts)")
            },
        ),
        ThemeToken::Chart4 => (
            claim(
                "value",
                "chart_4",
                t.chart_4,
                "native-theme-gpui/colors.rs:691",
            ),
            |i| {
                i.config("model", "colors.accent.fill").config("derived", "colors.accent.fill's hue turned 0.6 of the way round the wheel, its saturation raised to at least 0.3, its lightness and alpha kept (native-theme-gpui/colors.rs, assign_charts)")
            },
        ),
        ThemeToken::Chart5 => (
            claim(
                "value",
                "chart_5",
                t.chart_5,
                "native-theme-gpui/colors.rs:696",
            ),
            |i| {
                i.config("model", "colors.accent.fill").config("derived", "colors.accent.fill's hue turned 0.8 of the way round the wheel, its saturation raised to at least 0.3, its lightness and alpha kept (native-theme-gpui/colors.rs, assign_charts)")
            },
        ),
        ThemeToken::ChartBullish => (
            claim(
                "value",
                "chart_bullish",
                t.chart_bullish,
                "native-theme-gpui/colors.rs:537",
            ),
            |i| i.config("model", "colors.success.fill"),
        ),
        ThemeToken::ChartBearish => (
            claim(
                "value",
                "chart_bearish",
                t.chart_bearish,
                "native-theme-gpui/colors.rs:538",
            ),
            |i| i.config("model", "colors.error.fill"),
        ),
        ThemeToken::ChartGrid => (
            claim(
                "value",
                "chart_grid",
                t.chart_grid,
                "native-theme-gpui/colors.rs:636",
            ),
            |i| i.config("model", "separator.color"),
        ),
        ThemeToken::DescriptionListLabel => (
            claim(
                "value",
                "description_list_label",
                t.description_list_label,
                "native-theme-gpui/colors.rs:756",
            ),
            |i| {
                i.config("model", "colors.window.border, window.frame(normal).fill").config("derived", "colors.window.border faded to 20% of its alpha and blended over window.frame(normal).fill (native-theme-gpui/colors.rs, assign_misc)")
            },
        ),
        ThemeToken::DescriptionListLabelForeground => (
            claim(
                "value",
                "description_list_label_foreground",
                t.description_list_label_foreground,
                "native-theme-gpui/colors.rs:710",
            ),
            |i| i.config("model", "colors.window.foreground_muted"),
        ),
        ThemeToken::DragBorder => (
            claim(
                "value",
                "drag_border",
                t.drag_border,
                "native-theme-gpui/colors.rs:739",
            ),
            |i| {
                i.config("model", "button.body(NONE; variant=Primary).fill").config("derived", "button.body(NONE; variant=Primary).fill faded to 65% of its alpha, so translucent (native-theme-gpui/colors.rs, assign_misc)")
            },
        ),
        ThemeToken::DropTarget => (
            claim(
                "value",
                "drop_target",
                t.drop_target,
                "native-theme-gpui/colors.rs:741",
            ),
            |i| {
                i.config("model", "button.body(NONE; variant=Primary).fill").config("derived", "button.body(NONE; variant=Primary).fill faded to 20% of its alpha, so translucent (native-theme-gpui/colors.rs, assign_misc)")
            },
        ),
        ThemeToken::Popover => (
            claim(
                "value",
                "popover",
                t.popover,
                "native-theme-gpui/colors.rs:745",
            ),
            |i| i.config("model", "popover.panel.fill"),
        ),
        ThemeToken::PopoverForeground => (
            claim(
                "value",
                "popover_foreground",
                t.popover_foreground,
                "native-theme-gpui/colors.rs:746",
            ),
            |i| i.config("model", "popover.text.color"),
        ),
        ThemeToken::ProgressBar => (
            claim(
                "value",
                "progress_bar",
                t.progress_bar,
                "native-theme-gpui/colors.rs:772",
            ),
            |i| i.config("model", "progress_bar.fill(NONE).fill"),
        ),
        ThemeToken::Skeleton => (
            claim(
                "value",
                "skeleton",
                t.skeleton,
                "native-theme-gpui/colors.rs:736",
            ),
            |i| i.config("model", "button.body(NONE).fill"),
        ),
        ThemeToken::SliderBar => (
            claim(
                "value",
                "slider_bar",
                t.slider_bar,
                "native-theme-gpui/colors.rs:759",
            ),
            |i| i.config("model", "slider.active_track(NONE).fill"),
        ),
        ThemeToken::SliderThumb => (
            claim(
                "value",
                "slider_thumb",
                t.slider_thumb,
                "native-theme-gpui/colors.rs:760",
            ),
            |i| i.config("model", "slider.thumb(NONE).fill"),
        ),
        ThemeToken::Switch => (
            claim(
                "value",
                "switch",
                t.switch,
                "native-theme-gpui/colors.rs:763",
            ),
            |i| {
                i.config("model", "switch.track(NONE).fill").config("derived", "none: the model field as it is. ThemeColor has no field for a checked switch, so the connector maps switch.track(CHECKED).fill nowhere (native-theme-gpui/colors.rs, assign_misc)")
            },
        ),
        ThemeToken::SwitchThumb => (
            claim(
                "value",
                "switch_thumb",
                t.switch_thumb,
                "native-theme-gpui/colors.rs:769",
            ),
            |i| i.config("model", "switch.thumb(CHECKED).fill"),
        ),
        ThemeToken::StatusBar => (
            claim(
                "value",
                "status_bar",
                t.status_bar,
                "native-theme-gpui/colors.rs:778",
            ),
            |i| i.config("model", "status_bar.bar.fill"),
        ),
        ThemeToken::StatusBarBorder => (
            claim(
                "value",
                "status_bar_border",
                t.status_bar_border,
                "native-theme-gpui/colors.rs:779",
            ),
            |i| i.config("model", "status_bar.bar.stroke.color"),
        ),
        ThemeToken::TitleBar => (
            claim(
                "value",
                "title_bar",
                t.title_bar,
                "native-theme-gpui/colors.rs:666",
            ),
            |i| i.config("model", "window.title_bar(normal).fill"),
        ),
        ThemeToken::TitleBarBorder => (
            claim(
                "value",
                "title_bar_border",
                t.title_bar_border,
                "native-theme-gpui/colors.rs:667",
            ),
            |i| i.config("model", "window.frame(normal).stroke.color"),
        ),
        ThemeToken::WindowBorder => (
            claim(
                "value",
                "window_border",
                t.window_border,
                "native-theme-gpui/colors.rs:668",
            ),
            |i| i.config("model", "window.frame(normal).stroke.color"),
        ),
        ThemeToken::Magenta => (
            claim(
                "value",
                "magenta",
                t.magenta,
                "native-theme-gpui/colors.rs:884",
            ),
            |i| {
                i.config("model", "colors.accent.fill").config("derived", "hue 0.833, a literal, with colors.accent.fill's lightness and its saturation capped at 0.85, opaque (native-theme-gpui/colors.rs, assign_base_colors)")
            },
        ),
        ThemeToken::MagentaLight => (
            claim(
                "value",
                "magenta_light",
                t.magenta_light,
                "native-theme-gpui/colors.rs:910",
            ),
            |i| {
                light(i.config("model", "colors.accent.fill, window.frame(normal).fill").config("derived", "magenta as its own swatch states it, through light_variant (native-theme-gpui/colors.rs, assign_base_colors)"))
            },
        ),
        ThemeToken::Cyan => (
            claim("value", "cyan", t.cyan, "native-theme-gpui/colors.rs:892"),
            |i| {
                i.config("model", "colors.info.fill").config("derived", "hue 0.5, a literal, with colors.info.fill's lightness and its saturation capped at 0.85, opaque (native-theme-gpui/colors.rs, assign_base_colors)")
            },
        ),
        ThemeToken::CyanLight => (
            claim(
                "value",
                "cyan_light",
                t.cyan_light,
                "native-theme-gpui/colors.rs:911",
            ),
            |i| {
                light(i.config("model", "colors.info.fill, window.frame(normal).fill").config("derived", "cyan as its own swatch states it, through light_variant (native-theme-gpui/colors.rs, assign_base_colors)"))
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

/// Whether `token` is made from secondary_active, which the connector takes
/// from `button.body(PRESSED).fill` where a rule states it for `PRESSED`
/// and makes through `active_color` where none does
/// (native-theme-gpui/colors.rs, assign_secondary; native-theme-gpui
/// next/mod.rs, slice_colors).
fn reads_secondary_active(token: ThemeToken) -> bool {
    matches!(
        token,
        ThemeToken::SecondaryActive | ThemeToken::ButtonActive | ThemeToken::ButtonSecondaryActive
    )
}

/// Whether `token` is made from list_hover, which the connector takes from
/// `list.highlight(HOVER).fill` where the theme states it and leaves
/// gpui-component's own hovered row where it does not (native-theme-gpui
/// next/mod.rs, list_row_hover).
fn reads_list_hover(token: ThemeToken) -> bool {
    matches!(token, ThemeToken::ListHover | ThemeToken::TableHover)
}

/// The "branch" note of `token`'s swatch, where the connector's making of it
/// branches on what the installed 0.8.0 theme states: `pressed`, whether a
/// rule states `button.body(PRESSED).fill` (its `from_state`), and `hover`,
/// whether the theme states `list.highlight(HOVER).fill`; `None` for a
/// token made one way only.
pub fn branch_note(token: ThemeToken, pressed: bool, hover: bool) -> Option<&'static str> {
    if reads_secondary_active(token) {
        return Some(if pressed {
            "a rule states button.body(PRESSED).fill, so secondary_active is that colour as it is: active_color is not applied"
        } else {
            "no rule states button.body(PRESSED).fill, so secondary_active is button.body(NONE).fill through active_color"
        });
    }
    if reads_list_hover(token) {
        return Some(if hover {
            "the theme states list.highlight(HOVER).fill, so list_hover is that colour as it is"
        } else {
            "the theme states no list.highlight(HOVER).fill, so list_hover is gpui-component's own hovered row: menu.item(HOVER).fill, the accent the connector writes, at 60% of its alpha (native-theme-gpui next/mod.rs, list_row_hover; gpui-component theme/schema.rs, ThemeColor::apply_config)"
        });
    }
    None
}

/// A swatch's "translucent" note, where `value` is not opaque as its colour
/// text prints it (`as_rgba`: an alpha byte below 255): its alpha, and that
/// the page's background shows through.
pub fn translucent_note(value: gpui::Hsla) -> Option<String> {
    (as_rgba(value).a < u8::MAX).then(|| {
        format!(
            "{} alpha: the swatch paints it over the page's background, which shows through",
            percent_text(value.a)
        )
    })
}

/// The Theme Map's swatch of `token`. `native` is the 0.8.0 theme installed
/// for the current mode, if one is, which is when the connector wrote the
/// value.
pub fn swatch(t: &Theme, token: ThemeToken, native: Option<&NextTheme>) -> WidgetInfo {
    let Row { value, notes } = row(t, token);
    let field = value.field;
    let hex = as_rgba(value.value);
    let translucent = translucent_note(value.value);
    let info = WidgetInfo::new("ThemeColor").variant(field);
    let info = if let Some(r) = native {
        let info = notes(info.color(value));
        let branch = branch_note(
            token,
            state_fill_of(&r.button.body(WidgetState::PRESSED)).is_some(),
            r.list
                .highlight(WidgetState::HOVER)
                .stated
                .contains(PartStated::FILL),
        );
        let info = match branch {
            Some(note) => info.config("branch", note),
            None => info,
        };
        if base_palette(field) {
            info.config("installed", "theme_color_of writes it, and a ThemeConfig cannot carry it, so the connector writes it again after every rebuild upstream makes (native-theme-gpui/lib.rs, repair_base_palette)")
        } else {
            info.config("installed", "next::to_theme puts it on the Theme as theme_color_of wrote it, and the ThemeConfig stored for a light/dark switch carries it as hex, so after a switch it is rounded to 8 bits a channel (native-theme-gpui next/mod.rs, to_theme; native-theme-gpui/config.rs, theme_color_to_config_colors)")
        }
    } else {
        info.instance(
            "value",
            format!(
                "{hex}, gpui-component's own: no native theme is installed for this mode, so the connector wrote none of this page's values"
            ),
        )
    };
    let info = match translucent {
        Some(note) => info.instance("translucent", note),
        None => info,
    };
    info.color(claim("frame", "border", t.border, "showcase"))
        .color(claim(
            "label",
            "foreground",
            t.foreground,
            "gpui-component/label.rs:248",
        ))
        .not_themeable(
            "geometry",
            format!(
                "none: a {}px square, SWATCH_SIZE, the showcase's own choice rather than a platform's, as the theme states no swatch (spec §3.1), in its frame, which reads Theme::border, Theme::radius and the platform's defaults.border.line_width (showcase-gpui/support.rs, demo_frame)",
                px_text(crate::pages::SWATCH_SIZE.as_f32())
            ),
        )
        .not_themeable(
            "swatch gaps",
            format!(
                "{}px between two swatches of a row and {}px between the rows, SWATCH_GAP_X and SWATCH_GAP_Y, and {} between the square and its label, SWATCH_LABEL_GAP: the showcase's own choices rather than a platform's, as the theme states no swatch table (spec §3.1)",
                px_text(crate::pages::SWATCH_GAP_X.as_f32()),
                px_text(crate::pages::SWATCH_GAP_Y.as_f32()),
                super::rems_text(crate::pages::SWATCH_LABEL_GAP, t)
            ),
        )
        .instance("label", format!("{field} {hex}"))
}
