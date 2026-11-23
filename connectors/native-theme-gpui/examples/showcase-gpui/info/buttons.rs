//! What the Buttons page's widgets report about themselves (spec §3.4).

use gpui_component::{Colorize as _, Size, theme::Theme};
use native_theme::next::{ResolvedTheme as NextTheme, WidgetState};
use native_theme::theme::ResolvedButtonTheme;
use native_theme_gpui::ResolvedTheme;
use native_theme_gpui::next::geometry::button_disabled;

use super::{
    ColorClaim, WidgetInfo,
    chrome::{GhostContent, chrome_icon_note, ghost_colours, input_background},
    claim, edge_claim, percent_text, px_text, state_stroke_color_of, stated, stated_claim,
    stroke_color_of, stroke_width_of,
};
use crate::demo::{ButtonKind, ButtonState};
use crate::support::ChromeIcon;

/// What a Button of `kind` paints at rest: its fill, where it has one, and
/// its label.
fn at_rest(t: &Theme, kind: ButtonKind) -> Vec<ColorClaim> {
    match kind {
        ButtonKind::Default => vec![
            claim(
                "bg",
                "button",
                t.button,
                "gpui-component/button/button.rs:1036",
            ),
            claim(
                "text",
                "button_foreground",
                t.button_foreground,
                "gpui-component/button/button.rs:1050",
            ),
        ],
        ButtonKind::Primary => vec![
            claim(
                "bg",
                "button_primary",
                t.button_primary,
                "gpui-component/button/button.rs:1037",
            ),
            claim(
                "text",
                "button_primary_foreground",
                t.button_primary_foreground,
                "gpui-component/button/button.rs:1055",
            ),
        ],
        ButtonKind::Secondary => vec![
            claim(
                "bg",
                "button_secondary",
                t.button_secondary,
                "gpui-component/button/button.rs:1038",
            ),
            claim(
                "text",
                "button_secondary_foreground",
                t.button_secondary_foreground,
                "gpui-component/button/button.rs:1062",
            ),
        ],
        ButtonKind::Danger => vec![
            claim(
                "bg",
                "button_danger",
                t.button_danger,
                "gpui-component/button/button.rs:1039",
            ),
            claim(
                "text",
                "button_danger_foreground",
                t.button_danger_foreground,
                "gpui-component/button/button.rs:1070",
            ),
        ],
        ButtonKind::Success => vec![
            claim(
                "bg",
                "button_success",
                t.button_success,
                "gpui-component/button/button.rs:1041",
            ),
            claim(
                "text",
                "button_success_foreground",
                t.button_success_foreground,
                "gpui-component/button/button.rs:1084",
            ),
        ],
        ButtonKind::Warning => vec![
            claim(
                "bg",
                "button_warning",
                t.button_warning,
                "gpui-component/button/button.rs:1040",
            ),
            claim(
                "text",
                "button_warning_foreground",
                t.button_warning_foreground,
                "gpui-component/button/button.rs:1077",
            ),
        ],
        ButtonKind::Info => vec![
            claim(
                "bg",
                "button_info",
                t.button_info,
                "gpui-component/button/button.rs:1042",
            ),
            claim(
                "text",
                "button_info_foreground",
                t.button_info_foreground,
                "gpui-component/button/button.rs:1091",
            ),
        ],
        ButtonKind::Ghost => vec![claim(
            "text",
            "secondary_foreground",
            t.secondary_foreground,
            "native-theme-gpui/variants.rs:62",
        )],
        ButtonKind::Bordered => vec![
            claim("bg", "button", t.button, "showcase"),
            claim(
                "text",
                "button_foreground",
                t.button_foreground,
                "native-theme-gpui/variants.rs:142",
            ),
        ],
        ButtonKind::Link => vec![claim(
            "text",
            "link",
            t.link,
            "gpui-component/button/button.rs:1094",
        )],
        ButtonKind::Text => vec![claim(
            "text, foreground at 90%",
            "foreground",
            t.foreground.opacity(0.9),
            "gpui-component/button/button.rs:1095",
        )],
        ButtonKind::DefaultOutline => vec![
            input_background(t),
            claim(
                "text",
                "button_foreground",
                t.button_foreground,
                "gpui-component/button/button.rs:1050",
            ),
        ],
        ButtonKind::PrimaryOutline => vec![
            claim(
                "fill, primary at 10%",
                "primary",
                t.primary.opacity(0.1),
                "gpui-component/button/button.rs:972",
            ),
            claim(
                "text",
                "primary",
                t.primary,
                "gpui-component/button/button.rs:1053",
            ),
        ],
    }
}

/// What a Button of `kind` paints hovered and pressed.
fn under_the_pointer(t: &Theme, kind: ButtonKind) -> Vec<ColorClaim> {
    match kind {
        ButtonKind::Default => vec![
            claim(
                "hover",
                "button_hover",
                t.button_hover,
                "gpui-component/button/button.rs:1180",
            ),
            claim(
                "active",
                "button_active",
                t.button_active,
                "gpui-component/button/button.rs:1264",
            ),
        ],
        ButtonKind::Primary => vec![
            claim(
                "hover",
                "button_primary_hover",
                t.button_primary_hover,
                "gpui-component/button/button.rs:1187",
            ),
            claim(
                "active",
                "button_primary_active",
                t.button_primary_active,
                "gpui-component/button/button.rs:1271",
            ),
        ],
        ButtonKind::Secondary => vec![
            claim(
                "hover",
                "button_secondary_hover",
                t.button_secondary_hover,
                "gpui-component/button/button.rs:1194",
            ),
            claim(
                "active",
                "button_secondary_active",
                t.button_secondary_active,
                "gpui-component/button/button.rs:1278",
            ),
        ],
        ButtonKind::Danger => vec![
            claim(
                "hover",
                "button_danger_hover",
                t.button_danger_hover,
                "gpui-component/button/button.rs:1201",
            ),
            claim(
                "active",
                "button_danger_active",
                t.button_danger_active,
                "gpui-component/button/button.rs:1286",
            ),
        ],
        ButtonKind::Success => vec![
            claim(
                "hover",
                "button_success_hover",
                t.button_success_hover,
                "gpui-component/button/button.rs:1215",
            ),
            claim(
                "active",
                "button_success_active",
                t.button_success_active,
                "gpui-component/button/button.rs:1300",
            ),
        ],
        ButtonKind::Warning => vec![
            claim(
                "hover",
                "button_warning_hover",
                t.button_warning_hover,
                "gpui-component/button/button.rs:1208",
            ),
            claim(
                "active",
                "button_warning_active",
                t.button_warning_active,
                "gpui-component/button/button.rs:1293",
            ),
        ],
        ButtonKind::Info => vec![
            claim(
                "hover",
                "button_info_hover",
                t.button_info_hover,
                "gpui-component/button/button.rs:1222",
            ),
            claim(
                "active",
                "button_info_active",
                t.button_info_active,
                "gpui-component/button/button.rs:1307",
            ),
        ],
        ButtonKind::Ghost => vec![
            claim(
                "hover bg",
                "secondary_hover",
                t.secondary_hover,
                "native-theme-gpui/variants.rs:63",
            ),
            claim(
                "active bg",
                "secondary_active",
                t.secondary_active,
                "native-theme-gpui/variants.rs:64",
            ),
        ],
        ButtonKind::Bordered => vec![
            claim(
                "hover",
                "button_hover",
                t.button_hover,
                "native-theme-gpui/variants.rs:143",
            ),
            claim(
                "active",
                "button_active",
                t.button_active,
                "native-theme-gpui/variants.rs:144",
            ),
        ],
        ButtonKind::Link => vec![
            claim(
                "hover text",
                "link_hover",
                t.link_hover,
                "gpui-component/button/button.rs:1240",
            ),
            claim(
                "pressed text",
                "link_active",
                t.link_active,
                "gpui-component/button/button.rs:1316",
            ),
        ],
        ButtonKind::Text => vec![
            claim(
                "hover text",
                "foreground",
                t.foreground,
                "gpui-component/button/button.rs:1241",
            ),
            claim(
                "pressed text, foreground at 70%",
                "foreground",
                t.foreground.opacity(0.7),
                "gpui-component/button/button.rs:1317",
            ),
        ],
        ButtonKind::DefaultOutline => vec![
            claim(
                "hover bg, 50% input mixed with 50% transparent",
                "input",
                t.input.mix_oklab(t.transparent, 0.5),
                "gpui-component/button/button.rs:961-965",
            ),
            claim(
                "active bg, 70% input mixed with 30% transparent",
                "input",
                t.input.mix_oklab(t.transparent, 0.7),
                "gpui-component/button/button.rs:966-970",
            ),
        ],
        ButtonKind::PrimaryOutline => vec![
            claim(
                "hover bg, primary_hover at 20%",
                "primary_hover",
                t.primary_hover.opacity(0.2),
                "gpui-component/button/button.rs:975",
            ),
            claim(
                "active bg, primary_active at 40%",
                "primary_active",
                t.primary_active.opacity(0.4),
                "gpui-component/button/button.rs:978",
            ),
        ],
    }
}

/// The edge of a Button of `kind`: upstream's, which only Default and an
/// outlined Button have (button/button.rs:724-729), or, refined from
/// `native`, the resting body's stroke at rest and upstream's colour under
/// the pointer at that stroke's width, which replaces upstream's: none at 0.
fn edge(t: &Theme, kind: ButtonKind, native: Option<&NextTheme>) -> Vec<ColorClaim> {
    let width = native.and_then(|r| stroke_width_of(&r.button.body(WidgetState::NONE)));
    if width.is_some_and(|width| width <= 0.0) {
        return Vec::new();
    }
    let rest = native.and_then(|r| {
        edge_claim(
            "border at rest",
            "button.body(NONE).stroke.color",
            stroke_color_of(&r.button.body(WidgetState::NONE)),
            width,
            "native-theme-gpui/next/builders.rs:50",
        )
    });
    let hovered = match (kind, native.is_some()) {
        (ButtonKind::Bordered, _) => native.and_then(|r| {
            edge_claim(
                "border, hovered or pressed",
                "button.body(HOVER).stroke.color",
                state_stroke_color_of(&r.button.body(WidgetState::HOVER)),
                width,
                "native-theme-gpui/next/builders.rs:819",
            )
        }),
        (ButtonKind::Default | ButtonKind::DefaultOutline, true) => Some(claim(
            "border, hovered or pressed",
            "input",
            t.input,
            "gpui-component/button/button.rs:1102",
        )),
        (ButtonKind::Default | ButtonKind::DefaultOutline, false) => Some(claim(
            "border",
            "input",
            t.input,
            "gpui-component/button/button.rs:1102",
        )),
        (ButtonKind::PrimaryOutline, true) => Some(claim(
            "border, hovered or pressed",
            "primary",
            t.primary,
            "gpui-component/button/button.rs:1104",
        )),
        (ButtonKind::PrimaryOutline, false) => Some(claim(
            "border",
            "primary",
            t.primary,
            "gpui-component/button/button.rs:1104",
        )),
        _ => None,
    };
    rest.into_iter().chain(hovered).collect()
}

/// `info` with what a Button of `kind` paints at rest, its [`edge`], and
/// what it paints under the pointer.
fn colours(
    info: WidgetInfo,
    t: &Theme,
    kind: ButtonKind,
    native: Option<&NextTheme>,
) -> WidgetInfo {
    let info = at_rest(t, kind).into_iter().fold(info, WidgetInfo::color);
    let info = info.colors(edge(t, kind, native));
    under_the_pointer(t, kind)
        .into_iter()
        .fold(info, WidgetInfo::color)
}

/// The fill of a disabled Button of `kind`.
///
/// Only the variants the page disables have an arm: every other one is
/// `None`, so read its arm of `ButtonVariant::disabled` (button/button.rs)
/// before the page disables one.
fn disabled_fill(t: &Theme, kind: ButtonKind) -> Option<ColorClaim> {
    match kind {
        ButtonKind::Primary => Some(claim(
            "bg, at 15%",
            "button_primary",
            t.button_primary.opacity(0.15),
            "gpui-component/button/button.rs:1377",
        )),
        // `opacity(1.5)`: gpui clamps the factor to 1 (gpui-pre
        // color.rs:637-644), so the fill is the token as it stands, in both
        // modes -- the arm does not branch on the mode.
        ButtonKind::Secondary => Some(claim(
            "bg, button_secondary at 150% (clamped to 100%)",
            "button_secondary",
            t.button_secondary.opacity(1.5),
            "gpui-component/button/button.rs:1382",
        )),
        ButtonKind::Danger => Some(claim(
            "bg, at 15%",
            "button_danger",
            t.button_danger.opacity(0.15),
            "gpui-component/button/button.rs:1378",
        )),
        // `input_background()` at 0.5 (button/button.rs:1376-1380), which
        // reads a different field in each mode (theme/mod.rs:493-498).
        ButtonKind::Default if t.is_dark() => Some(claim(
            "bg (input mixed toward transparent), at 50%",
            "input",
            t.input_background().opacity(0.5),
            "gpui-component/theme/mod.rs:505",
        )),
        ButtonKind::Default => Some(claim(
            "bg, at 50%",
            "background",
            t.background.opacity(0.5),
            "gpui-component/theme/mod.rs:507",
        )),
        _ => None,
    }
}

/// What a loading Button of `kind` paints: its colours at rest, faded with
/// the whole element to 80% (button/button.rs:861). It is not interactive,
/// so it takes no hover or press style (:536-538, :737, :755).
///
/// Only the variant the page shows loading has an arm: read another's rest
/// arms before the page loads one.
fn loading(t: &Theme, kind: ButtonKind) -> Vec<ColorClaim> {
    match kind {
        ButtonKind::Primary => vec![
            claim(
                "bg, at 80% (the whole Button fades, button/button.rs:861)",
                "button_primary",
                t.button_primary.opacity(0.8),
                "gpui-component/button/button.rs:1037",
            ),
            claim(
                "text, at 80% (the whole Button fades, button/button.rs:861)",
                "button_primary_foreground",
                t.button_primary_foreground.opacity(0.8),
                "gpui-component/button/button.rs:1055",
            ),
        ],
        _ => Vec::new(),
    }
}

/// The name `size` has in the `Size` enum.
pub(crate) fn size_name(size: Size) -> &'static str {
    match size {
        Size::XSmall => "XSmall",
        Size::Small => "Small",
        Size::Medium => "Medium",
        Size::Large => "Large",
        Size::Size(_) => "a size in pixels",
    }
}

/// A Button of `kind` in `state`, with an icon where `icon` is set: an icon
/// of the chosen icon theme, which a loading Button's spinner turns.
/// `default_spinner` is set for a loading Button given no loading icon
/// because the widget icons serve the chosen set's Loader, which the
/// Spinner's own default then is (`demo::spinner_default`).
///
/// `size` is `Some` only in the row that shows upstream's size scale, which
/// is built without `geometry::button`; every other Button is built at the
/// default size and refined by it from `native`, the installed 0.8.0 theme,
/// where the builder applies (it states every value `geometry::button`
/// reads). Its geometry line is recorded by `next_native_info` where
/// `demo::button` applies the builder.
pub fn button(
    t: &Theme,
    kind: ButtonKind,
    state: ButtonState,
    icon: bool,
    default_spinner: bool,
    size: Option<Size>,
    native: Option<&NextTheme>,
) -> WidgetInfo {
    let styled = native.is_some();
    // The theme whose disabled pair `geometry::button_disabled` gives: one
    // that states every value the builder reads; under any other the
    // Button keeps upstream's disabled style (spec §10, rule 0).
    let disabled_pair = native.filter(|r| button_disabled(r).is_ok());
    let name = kind.name();
    let variant = match (size, state) {
        (Some(size), _) => format!("{name}, {}", size_name(size)),
        (None, ButtonState::Idle) if icon => format!("{name}, icon"),
        (None, ButtonState::Idle) => name.to_string(),
        (None, ButtonState::Disabled) => format!("{name}, disabled"),
        (None, ButtonState::Loading) => format!("{name}, loading"),
    };
    let info = WidgetInfo::new("Button").variant(variant);
    let info = match state {
        ButtonState::Idle => colours(info, t, kind, native),
        ButtonState::Loading => loading(t, kind).into_iter().fold(info, WidgetInfo::color),
        // geometry::button_disabled's line is recorded where demo::button
        // applies it, over the fill and label upstream paints first.
        ButtonState::Disabled if disabled_pair.is_some() => info,
        ButtonState::Disabled => {
            let info = match kind {
                ButtonKind::Primary | ButtonKind::Danger => info.not_themeable("fill", "the variant's own token at 0.15, a literal -- not 0.5, and not a disabled token: the model carries button.disabled_background and the platform states one, and upstream reads neither (button/button.rs, ButtonVariant::disabled)"),
                ButtonKind::Secondary => info.not_themeable("fill", "the variant's own token at 1.5, a literal that gpui clamps to 1 (gpui-pre/color.rs, Hsla::opacity), so a disabled Secondary keeps its fill while a disabled Primary or Danger fades to 0.15 -- and not a disabled token: the model carries button.disabled_background and the platform states one, and upstream reads neither (button/button.rs, ButtonVariant::disabled)"),
                ButtonKind::Default => info.not_themeable("fill", "input_background() at 0.5, a literal, and not a disabled token: the model carries button.disabled_background and the platform states one, and upstream reads neither (button/button.rs, ButtonVariant::disabled)"),
                _ => info,
            };
            disabled_fill(t, kind)
                .into_iter()
                .fold(info, WidgetInfo::color)
                .color(claim(
                    "text, at 50%",
                    "muted_foreground",
                    t.muted_foreground.opacity(0.5),
                    "gpui-component/button/button.rs:1385",
                ))
        }
    };
    let info = if styled {
        info.not_themeable("edge", match (state, native) {
                (_, Some(r)) if stroke_width_of(&r.button.body(WidgetState::NONE)).is_some_and(|width| width <= 0.0) => "none in any state: button.body(NONE).stroke.width is 0, and geometry::button's width replaces upstream's border_1, whose hover and press styles only recolour (button/button.rs, RenderOnce for Button)".to_string(),
                (ButtonState::Idle, _) => format!("button.body(NONE).stroke.color at rest: geometry::button gives every variant a border, and upstream's hover and press styles repaint it {} -- gpui applies a hover style after the refinement (gpui-pre/elements/div.rs, hover_style)", match kind {
                    ButtonKind::Default | ButtonKind::DefaultOutline => "input",
                    ButtonKind::Primary | ButtonKind::PrimaryOutline => "primary",
                    ButtonKind::Secondary => "border",
                    ButtonKind::Danger => "button_danger",
                    ButtonKind::Success => "button_success",
                    ButtonKind::Warning => "button_warning",
                    ButtonKind::Info => "button_info",
                    ButtonKind::Ghost | ButtonKind::Link | ButtonKind::Text => "transparent, so it vanishes under the pointer",
                    ButtonKind::Bordered => "button.body(HOVER).stroke.color, the colour variants::bordered_button gives its custom variant (native-theme-gpui/next/builders.rs, hover_border)",
                }),
                (ButtonState::Disabled, _) => "button.body(NONE).stroke.color, as at rest: upstream's disabled style sets an edge colour of its own and then replays the caller's style over it, so geometry::button's colour wins (button/button.rs, RenderOnce for Button)".to_string(),
                (ButtonState::Loading, _) => "button.body(NONE).stroke.color, and it stays: a loading Button takes no hover or press style, so nothing repaints it (button/button.rs, Button::interactive). It is painted at 80% all the same: while loading, the whole element fades, border included (button/button.rs, Button::render: line 861, the opacity(0.8))".to_string(),
            })
    } else {
        info.config("border-radius", format!("radius: {}px", t.radius.as_f32()))
    };
    let info = match kind {
        ButtonKind::Ghost => info,
        ButtonKind::Bordered => info.not_themeable("shadow", "none: variants::bordered_button builds its custom variant with .shadow(false), and upstream's Default variant has none (native-theme-gpui/variants.rs, bordered_button; button/button.rs, ButtonVariant::shadow)"),
        _ => info.not_themeable("shadow", "none on a standard variant: Theme::shadow, which the connector sets from border.shadow_enabled, reaches only a ButtonCustomVariant built with .shadow(true) (button/button.rs, ButtonVariant::shadow)"),
    };
    let info = if styled {
        styled_label_notes(info)
    } else {
        info.not_themeable("label size", "a fixed ratio of the platform's base, not a value of its own: the label takes button_text_size (sizing.rs, button_text_size), which is text_xs for XSmall, text_sm for Small and Medium, and text_base for Large and Size::Size -- all rems -- and gpui-component's WindowState root plugin sets the rem to Theme::font_size (gpui-component/root.rs, WindowState::prepare set_rem_size), which this connector fills from the platform font. So it scales with font.size and cannot be set apart from it: button_text_size has no Size::Size arm")
    };
    let info = match kind {
        ButtonKind::Default => info.instance("variant", "no variant method, so ButtonVariant::Default -- the button family, whose own edge colour is input, not border (button/button.rs, ButtonVariant::border_color)"),
        ButtonKind::Ghost => ghost_variant(info),
        ButtonKind::Bordered => info.instance("variant", "native_theme_gpui::variants::bordered_button: a custom variant with the Default Button's text, hover and press colours (button_foreground, button_hover, button_active) and button.hover_border_color as its border, which upstream's Default variant draws in input (button/button.rs, ButtonVariant::border_color). A custom variant fills itself at rest with its border colour mixed toward transparent (button/button.rs, ButtonVariant::bg_color), so the showcase gives the Button ThemeColor button, the Default Button's rest fill. Built only where the theme states a hover border; elsewhere the Button is the Default variant"),
        ButtonKind::Link => info
            .not_themeable("fill", "transparent in every state (button/button.rs, ButtonVariant::bg_color, hovered and active)")
            .not_themeable("underline", "always on for this variant (button/button.rs, ButtonVariant::underline)"),
        ButtonKind::Text => info
            .not_themeable("opacity", "the one variant that dims rather than recolours: foreground at 90% idle and 70% pressed, full strength on hover (button/button.rs, ButtonVariant::text_color, hovered, active). Those are literals, and the model states no dimmed copy; the swatches show the dimmed colours upstream paints")
            .not_themeable("fill", "transparent in every state (button/button.rs, ButtonVariant::bg_color, hovered and active)"),
        ButtonKind::DefaultOutline => info.instance("variant", "no variant method, outlined: ButtonVariant::Default, which an outline fills with input_background() and edges with input, not with the button family (button/button.rs, ButtonVariant::outline_background)")
            .not_themeable("outline fill", "input_background() at rest, and input mixed toward transparent under the pointer: 50% hovered and 70% pressed, two literals (button/button.rs, ButtonVariant::outline_background)"),
        ButtonKind::PrimaryOutline => info.not_themeable("outline fill opacity", "0.1 at rest, 0.2 hovered, 0.4 pressed -- three literals, so the platform sets the hue and gpui-component sets how far it is faded (button/button.rs, ButtonVariant::outline_background)"),
        ButtonKind::Primary
        | ButtonKind::Secondary
        | ButtonKind::Danger
        | ButtonKind::Success
        | ButtonKind::Warning
        | ButtonKind::Info => info,
    };
    let info = match (state, disabled_pair.and_then(|r| r.button.disabled_opacity)) {
        (ButtonState::Idle, _) => info,
        (ButtonState::Disabled, Some(opacity)) => info
            .instance("fill and text", "button.body(DISABLED).fill and button.label(DISABLED).color, through geometry::button_disabled: upstream paints its own literals -- input_background() or the variant's token faded, and muted_foreground at 0.5 -- and then replays the caller's refinement over them (button/button.rs, ButtonVariant::disabled and RenderOnce for Button)")
            .config("opacity", if opacity < 1. {
                format!("button.disabled_opacity {}: geometry::button_disabled fades the whole Button by it, over the stated pair", percent_text(opacity))
            } else {
                format!("button.disabled_opacity {}: no fade, the stated pair painted as it is", percent_text(opacity))
            })
            .not_themeable("cursor", "the default arrow, not not-allowed: Button sets cursor_default, and only a link or text variant asks for a pointer (button/button.rs, Button::render)"),
        (ButtonState::Disabled, None) => match native.and_then(|r| button_disabled(r).err()) {
            Some(unstated) => info.config("fill and text", format!("gpui-component's own: the theme states no value gpui can take at {}, which geometry::button_disabled needs", unstated.home())),
            None => info,
        }
            .not_themeable("text", "muted_foreground at 0.5, so a disabled button does not keep its variant's text colour. button.label(DISABLED).color is modelled and carried, and upstream reads it nowhere (button/button.rs, ButtonVariant::disabled)")
            .not_themeable("opacity", "button.disabled_opacity is modelled; upstream multiplies its own literals instead, so the platform's figure has no receiver -- Tier U, not an absence")
            .not_themeable("cursor", "the default arrow, not not-allowed: Button sets cursor_default, and only a link or text variant asks for a pointer (button/button.rs, Button::render)"),
        (ButtonState::Loading, _) if icon && default_spinner => info
            .not_themeable("spinner", "in place of the button's icon (button/button_icon.rs, ButtonIcon). The Button is given no loading icon, so the Spinner turns its own default, gpui-component's Loader (gpui-component/spinner.rs, Spinner::new), which the connector's widget icons serve from the chosen icon theme (native-theme-gpui/icons/widget.rs, WidgetIcons): that theme's loading icon by the connector's table, not gpui-component's glyph. It inherits the button's text colour and turns every 0.8s, a literal (gpui-component/spinner.rs, Spinner::new)")
            .not_themeable("interaction", "inert, as a disabled button is, but not styled as one: it keeps its variant's colours and the whole button fades to 0.8 (button/button.rs, Button::interactive)"),
        (ButtonState::Loading, _) if icon => info
            .not_themeable("spinner", "in place of the button's icon (button/button_icon.rs, ButtonIcon). This one's icon is the chosen icon theme's loading icon, which the Button is also given as its loading icon, so the Spinner turns it in place of its own default (button/button.rs, Button::loading_icon), gpui-component's Loader, drawn as the application's icon resolver answers it (the `spinner icon` line says what is turned here). It inherits the button's text colour and turns every 0.8s, a literal (gpui-component/spinner.rs, Spinner::new)")
            .not_themeable("interaction", "inert, as a disabled button is, but not styled as one: it keeps its variant's colours and the whole button fades to 0.8 (button/button.rs, Button::interactive)"),
        (ButtonState::Loading, _) if default_spinner => info
            .not_themeable("spinner", "where an icon would sit: a loading Button without an icon shows a Spinner there (button/button.rs, RenderOnce for Button). This one is given no loading icon, so the Spinner turns its own default, gpui-component's Loader (gpui-component/spinner.rs, Spinner::new), which the connector's widget icons serve from the chosen icon theme (native-theme-gpui/icons/widget.rs, WidgetIcons): that theme's loading icon by the connector's table, not gpui-component's glyph. It inherits the button's text colour and turns every 0.8s, a literal (gpui-component/spinner.rs, Spinner::new)")
            .not_themeable("interaction", "inert, as a disabled button is, but not styled as one: it keeps its variant's colours and the whole button fades to 0.8 (button/button.rs, Button::interactive)"),
        (ButtonState::Loading, _) => info
            .not_themeable("spinner", "where an icon would sit: a loading Button without an icon shows a Spinner there, turning the Button's loading icon, or the Spinner's own default, gpui-component's Loader, drawn as the application's icon resolver answers it (the `spinner icon` line says what is turned here), where it is given none (button/button.rs:777-786, loading_icon; gpui-component/spinner.rs, Spinner::new). It inherits the button's text colour and turns every 0.8s, a literal (gpui-component/spinner.rs, Spinner::new)")
            .not_themeable("interaction", "inert, as a disabled button is, but not styled as one: it keeps its variant's colours and the whole button fades to 0.8 (button/button.rs, Button::interactive)"),
    };
    let info = if icon && state == ButtonState::Idle {
        info.not_themeable("icon position", "leading, always: Button adds its icon before the label and has no setting for the other side (button/button.rs, Button::render)")
            .not_themeable("icon size", "size_3 / size_3p5 / size_4 / size_6 per the button's Size (icon.rs, Icon::into_svg) -- rems again, so it scales with the platform font, while defaults.icon_sizes is in absolute px and Button exposes no icon_size setter to take one")
    } else {
        info
    };
    match size {
        Some(size) => info
            .not_themeable("padding", "per Size only because this demo omits the refinement the other Button panels apply: upstream takes a copy of the caller's style before the Size arm sets its px_1 / px_2 / px_2p5 / px_3 (XSmall, Small, Medium, Large) and re-applies that copy afterwards (button/button.rs, Button), so the border.padding sides geometry::button carries would win. Left bare on purpose -- this is the panel that shows the enum")
            .not_themeable("min-height", "the same: the Size arm's h_5/h_6/h_8 is overruled by a refinement, so button.min_height would arrive through geometry::button (button/button.rs, Button)")
            .instance("size", format!("{} via the Size enum", size_name(size))),
        None => info,
    }
}

/// A dialog's default Button built through
/// `native_theme_gpui::variants::default_button` under a theme that gives it
/// a look of its own, `b` (`demo::dialog_default_button`): at rest
/// `geometry::default_button_rest`'s fill and frame, under the pointer the
/// variant's hover fill and its colour as the frame, pressed the plain
/// button's `button_active` in that frame, and one label in every state.
pub fn dialog_default(t: &Theme, b: &ResolvedButtonTheme) -> WidgetInfo {
    let width = Some(b.border.line_width);
    let info = WidgetInfo::new("Button").variant("Dialog default");
    let info = info.colors(stated_claim(
        "bg",
        "default_button_background",
        b.default_button_background,
        "native-theme-gpui/geometry.rs:2084",
    ));
    let info = match stated_claim(
        "text",
        "default_button_text_color",
        b.default_button_text_color,
        "native-theme-gpui/variants.rs:274",
    ) {
        Some(text) => info.color(text),
        None => info.color(claim(
            "text",
            "color",
            stated(b.font.color),
            "native-theme-gpui/variants.rs:274",
        )),
    };
    let rest = edge_claim(
        "border at rest",
        "default_button_border_color",
        b.default_button_border_color,
        width,
        "native-theme-gpui/geometry.rs:2085",
    )
    .or_else(|| {
        edge_claim(
            "border at rest",
            "border",
            Some(b.border.color),
            width,
            "native-theme-gpui/geometry.rs:2086",
        )
    });
    let hovered = edge_claim(
        "border, hovered or pressed",
        "hover_border_color",
        b.hover_border_color,
        width,
        "native-theme-gpui/variants.rs:275",
    )
    .or_else(|| {
        edge_claim(
            "border, hovered or pressed",
            "default_button_border_color",
            b.default_button_border_color,
            width,
            "native-theme-gpui/variants.rs:271",
        )
    })
    .or_else(|| {
        edge_claim(
            "border, hovered or pressed",
            "border",
            Some(b.border.color),
            width,
            "native-theme-gpui/variants.rs:271",
        )
    });
    let info = info.colors(rest).colors(hovered);
    let info = info
        .colors(stated_claim(
            "hover",
            "default_button_background",
            b.default_button_background,
            "native-theme-gpui/variants.rs:270",
        ))
        .color(claim(
            "active",
            "button_active",
            t.button_active,
            "native-theme-gpui/variants.rs:353",
        ));
    let edge_note = if b.border.line_width <= 0.0 {
        "none in any state: button.border.line_width is 0, and geometry::button's width replaces upstream's border_1, whose hover and press styles only recolour (button/button.rs, RenderOnce for Button)"
    } else {
        "button.default_button_border_color at rest (button.border.color where unstated), from geometry::default_button_rest after geometry::button, which gives the border its width; hovered and pressed the variant's colour, button.hover_border_color where stated, else that frame -- gpui applies a hover style after the refinement (gpui-pre/elements/div.rs, hover_style)"
    };
    styled_label_notes(info)
        .not_themeable("edge", edge_note)
        .not_themeable("shadow", "none: variants::default_button builds its custom variant with .shadow(false) (native-theme-gpui/variants.rs, default_button; button/button.rs, ButtonVariant::shadow)")
        .instance("variant", "native_theme_gpui::variants::default_button: a custom variant with the default button's hover fill, label and hovered and pressed frame and the plain button's pressed fill (button_active); its rest fill and frame are geometry::default_button_rest's, refined after geometry::button")
}

/// `info` with the label's two config lines of a Button that
/// `geometry::button` styles: the weight it carries and the size
/// `geometry::button_label` gives the label.
fn styled_label_notes(info: WidgetInfo) -> WidgetInfo {
    info.config("font-weight", "geometry::button carries button.font.weight. The label is a child that sets its own size from the Size enum (sizing.rs, button_text_size) and would overrule a size from here, but it sets no weight and neither does anything else on that path, so the platform's weight cascades (button/button.rs, Button::render)")
        .config("label size", "button.font.size through geometry::button_label on the label, which the showcase passes as the Button's child: upstream sizes a .label() from the Size enum on the content element (sizing.rs, button_text_size; button/button.rs, Button::render), which a style on the Button does not reach and a child's own size overrules")
}

/// The variant line of a Ghost Button built with
/// `native_theme_gpui::variants::ghost_button`.
fn ghost_variant(info: WidgetInfo) -> WidgetInfo {
    info.instance("variant", "native_theme_gpui::variants::ghost_button: flat like gpui-component's .ghost(), but with the platform's button.hover_background / active_background. Upstream's own .ghost() would hover with the item-highlight pair (button/button.rs, ButtonVariant::hovered Ghost arm), which is the menu's hover fill (`menu.hover_background`), not a button hover")
}

/// `info` with what the page's Ghost Button paints, unrefined, and its
/// variant line: the status bar's side-panel toggle is that Ghost too, and
/// takes it from here so the two cannot drift apart.
pub(super) fn native_ghost(info: WidgetInfo, t: &Theme) -> WidgetInfo {
    ghost_variant(colours(info, t, ButtonKind::Ghost, None))
}

/// The variant line of a tool Button built with
/// `native_theme_gpui::variants::toolbar_button`.
fn toolbar_variant(info: WidgetInfo) -> WidgetInfo {
    info.instance("variant", "native_theme_gpui::variants::toolbar_button: native_theme_gpui::variants::ghost_button with the platform's toolbar button states where the theme states them, toolbar.button_hover_background, button_active_background and the outline button_hover_border_color, ghost_button's colour for each fill it leaves unstated, and no edge where it states no outline. It goes into a Toolbar by content, not child, which would make it a .ghost() again (toolbar.rs, ToolbarItem; button/button.rs, Button::prepare_for_toolbar)")
}

/// `info` with what a tool Button built with
/// `native_theme_gpui::variants::toolbar_button` paints, and its variant
/// line: the hover and pressed fills `toolbar.button_hover_background` and
/// `toolbar.button_active_background` where `native` states them, else the
/// Ghost's tokens, and the outline under the pointer and while pressed,
/// `toolbar.button_hover_border_color`, at `button.border.line_width`, the
/// width `geometry::toolbar_button_rest` gives the border. The window's
/// toolbar buttons, the Buttons page's toolbar items and the Basic page's
/// icon buttons take it from here so they cannot drift apart.
pub(super) fn native_toolbar(
    info: WidgetInfo,
    t: &Theme,
    native: Option<&ResolvedTheme>,
) -> WidgetInfo {
    let info = info.color(claim(
        "text",
        "secondary_foreground",
        t.secondary_foreground,
        "native-theme-gpui/variants.rs:62",
    ));
    let info = match stated_claim(
        "hover bg",
        "button_hover_background",
        native.and_then(|r| r.toolbar.button_hover_background),
        "native-theme-gpui/variants.rs:171",
    ) {
        Some(hover) => info.color(hover),
        None => info.color(claim(
            "hover bg",
            "secondary_hover",
            t.secondary_hover,
            "native-theme-gpui/variants.rs:63",
        )),
    };
    let info = match stated_claim(
        "active bg",
        "button_active_background",
        native.and_then(|r| r.toolbar.button_active_background),
        "native-theme-gpui/variants.rs:173",
    ) {
        Some(active) => info.color(active),
        None => info.color(claim(
            "active bg",
            "secondary_active",
            t.secondary_active,
            "native-theme-gpui/variants.rs:64",
        )),
    };
    let edge = native.and_then(|r| {
        edge_claim(
            "border, hovered or pressed",
            "button_hover_border_color",
            r.toolbar.button_hover_border_color,
            Some(r.button.border.line_width),
            "native-theme-gpui/variants.rs:172",
        )
    });
    toolbar_variant(info.colors(edge))
}

/// The width of the transparent border `geometry::toolbar_button_rest` gives
/// a tool Button under `native`, `button.border.line_width`, where it states
/// the toolbar's outline; `None` where it gives none.
pub(super) fn rest_border(native: Option<&ResolvedTheme>) -> Option<f32> {
    native
        .filter(|r| r.toolbar.button_hover_border_color.is_some())
        .map(|r| r.button.border.line_width)
}

/// The "fill" note of a tool Button built with
/// `native_theme_gpui::variants::toolbar_button`: what it fills with at rest,
/// under the pointer and while pressed under `native`, each state the
/// toolbar field where `native` states it, else the Ghost's token, as
/// [`native_toolbar`] claims them.
pub(super) fn toolbar_fill(native: Option<&ResolvedTheme>) -> String {
    let toolbar = native.map(|r| &r.toolbar);
    let hover = if toolbar.is_some_and(|t| t.button_hover_background.is_some()) {
        "toolbar.button_hover_background"
    } else {
        "ghost_button's secondary_hover"
    };
    let pressed = if toolbar.is_some_and(|t| t.button_active_background.is_some()) {
        "toolbar.button_active_background"
    } else {
        "ghost_button's secondary_active"
    };
    if toolbar.is_some_and(|t| t.button_hover_border_color.is_some()) {
        format!(
            "none at rest: geometry::toolbar_button_rest's transparent fill replaces the variant's own, its colour toolbar.button_hover_border_color mixed toward transparent (button/button.rs, ButtonVariant::bg_color); under the pointer {hover}, and pressed {pressed}"
        )
    } else {
        format!(
            "none at rest: the variant's colour is ghost_button's, transparent (button/button.rs, ButtonCustomVariant::new), and a custom variant fills at rest with that colour mixed toward transparent (button/button.rs, ButtonVariant::bg_color); under the pointer {hover}, and pressed {pressed}"
        )
    }
}

/// `info` with what a selected Button built with
/// `native_theme_gpui::variants::ghost_button` paints, and its variant line.
/// Upstream fills a selected Button with its variant's selected style,
/// which for a custom variant is the variant's active colour and its
/// foreground (button/button.rs, ButtonVariant::selected), and gives it no
/// hover or press style: it registers the hover on every Button, but paints
/// it, as it paints the press, only while the Button is neither disabled,
/// loading nor selected (button/button.rs:730-762, RenderOnce for Button).
pub(super) fn native_ghost_selected(info: WidgetInfo, t: &Theme) -> WidgetInfo {
    ghost_variant(
        info.color(claim(
            "selected bg",
            "secondary_active",
            t.secondary_active,
            "native-theme-gpui/variants.rs:64",
        ))
        .color(claim(
            "text",
            "secondary_foreground",
            t.secondary_foreground,
            "native-theme-gpui/variants.rs:62",
        )),
    )
}

/// `info` with what a checked tool Button paints, in place of what
/// [`native_toolbar`] claims for an unchecked one:
/// `geometry::toolbar_button_checked`'s fill and text where `native` states
/// the toolbar's checked fill; else, as [`native_ghost_selected`] says, the
/// variant's active colour as the selected fill, which that builder sets
/// too, under the variant's one foreground; and its pen,
/// `toolbar.button_checked_border_color`, where `native` states it. Upstream
/// draws no hover or press style on a selected Button, so it claims no hover
/// or pressed fill, and the edge stays the rest one under the pointer.
fn toolbar_checked(info: WidgetInfo, t: &Theme, native: Option<&ResolvedTheme>) -> WidgetInfo {
    let checked_fill = stated_claim(
        "checked fill",
        "button_checked_background",
        native.and_then(|r| r.toolbar.button_checked_background),
        "native-theme-gpui/geometry.rs:2060",
    );
    let info = match (checked_fill, native) {
        (Some(fill), Some(r)) => {
            let text = match stated_claim(
                "checked text",
                "checked_text_color",
                r.button.checked_text_color,
                "native-theme-gpui/geometry.rs:2056",
            ) {
                Some(text) => text,
                None => claim(
                    "checked text",
                    "active_text_color",
                    stated(r.button.active_text_color),
                    "native-theme-gpui/geometry.rs:2056",
                ),
            };
            info.color(fill).color(text)
        }
        _ => {
            let info = match stated_claim(
                "selected bg",
                "button_active_background",
                native.and_then(|r| r.toolbar.button_active_background),
                "native-theme-gpui/variants.rs:173",
            ) {
                Some(active) => info.color(active),
                None => info.color(claim(
                    "selected bg",
                    "secondary_active",
                    t.secondary_active,
                    "native-theme-gpui/variants.rs:64",
                )),
            };
            info.color(claim(
                "text",
                "secondary_foreground",
                t.secondary_foreground,
                "native-theme-gpui/variants.rs:62",
            ))
        }
    };
    // The pen's width is toolbar_button_rest's, which sets one only where the
    // toolbar's hover outline is stated.
    let info = info.colors(edge_claim(
        "checked edge",
        "button_checked_border_color",
        native.and_then(|r| r.toolbar.button_checked_border_color),
        native.and_then(|r| {
            r.toolbar
                .button_hover_border_color
                .map(|_| r.button.border.line_width)
        }),
        "native-theme-gpui/geometry.rs:2123",
    ));
    info.not_themeable(
        "hover outline while checked",
        "gpui-component draws no hover style on a selected Button (button/button.rs, RenderOnce for Button: the hover style returns early while selected), and replays the caller's refinement over the selected state, so the edge stays the rest one, the checked pen where the theme states it",
    )
}

/// The Size a tool button is built at: what upstream lays a labelled Button
/// of it out at (button/button.rs, RenderOnce for Button).
#[derive(Clone, Copy)]
pub(crate) enum ToolSize {
    /// The default Size, `Size::Medium`: `h_8` with `px_2p5`.
    Medium,
    /// `Size::Small`: `h_6` with `px_2`.
    Small,
}

impl ToolSize {
    /// Upstream's height, its side padding, and the Size's name.
    fn upstream(self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::Medium => ("h_8", "px_2p5", "the default Size"),
            Self::Small => ("h_6", "px_2", "Size::Small"),
        }
    }
}

/// `info` given a tool button's size and corners, a Ghost at `size` refined
/// by `demo::tool_box_of` where `native` is installed: rounded by
/// `button.border.corner_radius` and padded by the `button.border.padding`
/// sides it states, with no fixed height where a top or bottom side is
/// stated, upstream's height and side padding standing only for what it
/// leaves unstated; with no native theme, upstream's whole. `icon_child` is
/// whether the icon is the Button's child, which has upstream lay it out as
/// a labelled Button. `outline` is the width of the transparent border
/// `geometry::toolbar_button_rest` gives the Button, which it takes off each
/// stated side, where the theme states the toolbar's outline
/// ([`rest_border`]).
pub(crate) fn tool_button_size(
    info: WidgetInfo,
    size: ToolSize,
    icon_child: bool,
    native: Option<&ResolvedButtonTheme>,
    outline: Option<f32>,
) -> WidgetInfo {
    let (height, across, at) = size.upstream();
    let labelled = if icon_child {
        "; with its icon a child rather than its icon it is laid out as a labelled Button, not a square icon Button"
    } else {
        ""
    };
    // The size note's tail, the same under either form.
    let upstream_size = |what: String| {
        format!(
            "{what} -- rems, so the platform's font{labelled} (button/button.rs, RenderOnce for Button)"
        )
    };
    let Some(b) = native else {
        return info.not_themeable(
            "size",
            upstream_size(format!("{height} with {across} at {at}")),
        );
    };
    let p = &b.border.padding;
    let stated: Vec<String> = [
        ("top", p.top),
        ("right", p.right),
        ("bottom", p.bottom),
        ("left", p.left),
    ]
    .into_iter()
    .filter_map(|(side, v)| v.map(|v| format!("{side} {}px", px_text(v))))
    .collect();
    let vertical = p.top.is_some() || p.bottom.is_some();
    let radius = format!(
        "button.border.corner_radius, {}px",
        px_text(b.border.corner_radius.max(0.))
    );
    let sets = "which the showcase sets on the Button (demo::tool_box_of)";
    let text = match (stated.is_empty(), vertical) {
        (true, _) => {
            format!("{radius}, {sets}; the theme states no button.border.padding side")
        }
        (false, false) => format!(
            "{radius}, and button.border.padding: {}, {sets}",
            stated.join(", ")
        ),
        (false, true) => format!(
            "{radius}, and button.border.padding: {}, {sets}, with no fixed height: the content's, padded by the stated top and bottom (h_auto){}",
            stated.join(", "),
            if p.top.is_none() || p.bottom.is_none() {
                ", an unstated one none, as upstream pads a Button only across"
            } else {
                ""
            }
        ),
    };
    let text = match outline {
        Some(w) => {
            let grows: Vec<&str> = [
                ("top", p.top, vertical),
                ("right", p.right, true),
                ("bottom", p.bottom, vertical),
                ("left", p.left, true),
            ]
            .into_iter()
            .filter(|&(_, v, laid)| laid && w > 0. && v.is_none_or(|v| v < w))
            .map(|(side, _, _)| side)
            .collect();
            let width = format!("button.border.line_width, {}px", px_text(w));
            let tail = if grows.is_empty() {
                "so the outer box is unchanged".to_string()
            } else {
                format!("which makes the box larger on the {}", grows.join(", "))
            };
            if stated.is_empty() {
                format!(
                    "{text}; geometry::toolbar_button_rest gives the Button a transparent border {width} wide, {tail}"
                )
            } else {
                format!(
                    "{text}; each stated side less {width}, floored at 0, the width of the transparent border geometry::toolbar_button_rest gives the Button, {tail}"
                )
            }
        }
        None => text,
    };
    let info = info.config("radius and padding", text);
    let unstated_across = match (p.left.is_none(), p.right.is_none()) {
        (true, true) => Some(across.to_string()),
        (true, false) => Some(format!("{across} on the left")),
        (false, true) => Some(format!("{across} on the right")),
        (false, false) => None,
    };
    let upstream = match (vertical, unstated_across) {
        (true, None) => None,
        (true, Some(across)) => Some(across),
        (false, None) => Some(height.to_string()),
        (false, Some(across)) => Some(format!("{height} with {across}")),
    };
    match upstream {
        Some(what) => info.not_themeable(
            "size",
            upstream_size(format!(
                "{what} at {at}, upstream's for what button.border.padding leaves unstated"
            )),
        ),
        None => info,
    }
}

/// A `ButtonGroup` of Default Buttons, which report through the group.
pub fn button_group(t: &Theme) -> WidgetInfo {
    colours(WidgetInfo::new("ButtonGroup"), t, ButtonKind::Default, None)
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable("gap", "no gap to set: a ButtonGroup joins its buttons by turning edges off rather than by spacing them (button/button_group.rs, ButtonGroup)")
        .instance("variant", "no variant given, so Default: its edge is input, not border")
        .instance("buttons", "Left, Center and Right. ButtonGroup::child takes a Button, not an element a target could wrap, so they report through the group (button/button_group.rs, ButtonGroup::child)")
}

/// A `DropdownButton` whose Button is of `kind`.
pub fn dropdown_button(t: &Theme, kind: ButtonKind) -> WidgetInfo {
    let info = at_rest(t, kind).into_iter().fold(
        WidgetInfo::new("DropdownButton").variant(kind.name()),
        WidgetInfo::color,
    );
    info.colors(edge(t, kind, None))
        .color(claim(
            "menu bg",
            "popover",
            t.popover,
            "gpui-component/styled.rs:190",
        ))
        .not_themeable("dropdown arrow", "a Caret, not an icon the caller passes: the right half is a Button::dropdown_caret and the glyph comes from select.rs, Caret. The shape is fixed, but its colour is not -- upstream paints it with the button variant's own text colour at 75% (button/button.rs, Button::render dropdown_caret), so it follows the platform through the same token the label does")
        .not_themeable("own icons", super::own_icons("the dropdown arrow's ChevronDown (select.rs, Caret)"))
        .instance("halves", "both take the variant of the Button the showcase gives it: DropdownButton passes it on to the caret half (button/dropdown_button.rs, DropdownButton::effective_variant)")
}

/// The notes every Toggle and ToggleGroup the showcase builds shares: all
/// are at the default Size, and take `geometry::toggle` where `styled`.
pub(super) fn toggle_notes(info: WidgetInfo, t: &Theme, styled: bool) -> WidgetInfo {
    let info = info
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable("size", "min_w_8 / h_8 at the default Size, upstream's own: the model states no flat-button frame (button/toggle.rs, Toggle::render)");
    if styled {
        info.config("text", "button.label(NONE)'s size and weight through geometry::toggle, which lands after the text_sm a Medium Toggle sets on itself (button/toggle.rs, Toggle::render)")
    } else {
        info
    }
}

/// The hovered colours of an unchecked Toggle, of either variant.
pub(super) fn toggle_hover(info: WidgetInfo, t: &Theme) -> WidgetInfo {
    info.color(claim(
        "hover bg",
        "accent",
        t.accent,
        "gpui-component/button/toggle.rs:212",
    ))
    .color(claim(
        "hover text",
        "accent_foreground",
        t.accent_foreground,
        "gpui-component/button/toggle.rs:213",
    ))
}

/// The colours of a checked Toggle, of either variant, which is not
/// hoverable (button/toggle.rs:161), and why they are what they are.
pub(super) fn toggle_checked(info: WidgetInfo, t: &Theme) -> WidgetInfo {
    info.color(claim(
        "checked bg",
        "accent",
        t.accent,
        "gpui-component/button/toggle.rs:164",
    ))
    .color(claim(
        "checked text",
        "accent_foreground",
        t.accent_foreground,
        "gpui-component/button/toggle.rs:165",
    ))
    .not_themeable("checked fill", "accent, the menu highlight, by default -- but not out of reach: a Toggle folds the caller's refinement into its checked style too (button/toggle.rs, Toggle::render), so an application that refines a checked Toggle with segmented_control.active_background and active_text_color gets them. Only an unchecked one's hover is Tier U. Nothing applies them: geometry::toggle carries the Toggle's text alone")
}

/// The unchecked fill of a Ghost Toggle.
fn ghost_toggle_fill(info: WidgetInfo) -> WidgetInfo {
    info.not_themeable("unchecked fill", "none: ToggleVariant defaults to Ghost (button/toggle.rs, ToggleVariant), which paints no background and no border; only .outline() fills")
}

/// A `Toggle`, `checked` or not, showing its icon where `drawn`, and its
/// icon's name as its label where the chosen icon theme has none; `icon`
/// says which. `styled` where a native theme is installed, so the Toggle
/// takes `geometry::toggle`.
pub fn toggle(t: &Theme, drawn: bool, icon: String, checked: bool, styled: bool) -> WidgetInfo {
    let info = WidgetInfo::new("Toggle").variant(match (drawn, checked) {
        (true, true) => "Ghost, checked",
        (true, false) => "Ghost",
        (false, true) => "Ghost, labelled, checked",
        (false, false) => "Ghost, labelled",
    });
    let info = if checked {
        toggle_checked(info, t)
    } else {
        ghost_toggle_fill(toggle_hover(info, t))
    };
    toggle_notes(info, t, styled)
        .instance("icon", icon)
        .instance(
            "click",
            "checks or unchecks it; the showcase keeps the state",
        )
}

/// A `ToggleGroup` of unchecked Toggles, which report through the group.
/// `styled` where a native theme is installed, so each Toggle takes
/// `geometry::toggle`.
pub fn toggle_group(t: &Theme, styled: bool) -> WidgetInfo {
    toggle_notes(ghost_toggle_fill(toggle_hover(WidgetInfo::new("ToggleGroup").variant("Ghost"), t)), t, styled)
        .not_themeable("gap", "gap_2 between the toggles -- a rem, so the platform's font; only a segmented group drops it (button/toggle.rs, ToggleGroup::segmented)")
        .instance("toggles", "Left, Center and Right, none checked. ToggleGroup::child takes a Toggle, not an element a target could wrap, so they report through the group (button/toggle.rs, ToggleGroup::child)")
}

/// A `Clipboard` copying `value`, reading `label` beside its icon where one
/// is given.
pub fn clipboard(t: &Theme, value: &'static str, label: Option<&'static str>) -> WidgetInfo {
    let info = WidgetInfo::new("Clipboard");
    let info = match label {
        Some(label) => info
            .variant("labelled")
            .colors(ghost_colours(t, GhostContent::TextAndIcon))
            .instance("label", format!("{label}: text in the ghost Button's colours, as its icon, the Clipboard handing the label to the ghost Button it builds (gpui-component/clipboard.rs:52, label; Clipboard::render)")),
        None => info.colors(ghost_colours(t, GhostContent::Icon)),
    };
    info
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable("surface", "a Clipboard is a ghost Button and reads no theme field of its own (clipboard.rs, Clipboard::render): transparent until hovered, when it takes accent (at half alpha in dark mode) -- the menu highlight, not the button family. Its icon takes the Ghost variant's secondary_foreground")
        .not_themeable("copy icon", "Copy and Check, built inline with no setter to replace them (clipboard.rs, Clipboard)")
        .not_themeable("own icons", super::own_icons("Copy, and Check once it has copied (clipboard.rs, Clipboard)"))
        .instance("value", value)
}

/// The Buttons page's component `Toolbar` (spec §8.2): gpui-component's own
/// at its default Small Size, refined by `geometry::toolbar`, whose line
/// `demo::component_toolbar` records; `item_gap` is the `toolbar.item_gap`
/// the theme states, and `native` the theme the builder refined it from,
/// where a native theme is installed. Its Buttons report themselves
/// (`toolbar_item`).
pub fn toolbar(item_gap: Option<f32>, native: Option<&ResolvedTheme>) -> WidgetInfo {
    let group_gap = match item_gap {
        Some(g) => format!("{}px, toolbar.item_gap", px_text(g)),
        None => "gap_1, the Toolbar's own Small gap, which the showcase gives each group: the theme states no toolbar.item_gap, and a ToolbarGroup spaces nothing itself (gpui-base/toolbar.rs, RenderOnce for ToolbarGroup)".to_string(),
    };
    let info = WidgetInfo::new("Toolbar")
        .variant("Small, two ToolbarGroups")
        .not_themeable(
            "size",
            "min_h_8, p_1, gap_1 and text_sm at its default Small Size (gpui-component/toolbar.rs, RenderOnce for Toolbar), set before the caller's style, so geometry::toolbar sets the height to the content's and the minimum to toolbar.bar_height where stated, else to auto, and replaces each other one the theme states: the gap by toolbar.item_gap; the padding by toolbar.border's stated sides; the text by toolbar.font's size and weight",
        )
        .not_themeable(
            "items",
            "added with content(), not child(): child() would make a Button a compact ghost and wrap it in an input_h box, 1.5 rem at Small (toolbar.rs, ToolbarItem; button/button.rs, Button::prepare_for_toolbar; sizing.rs, input_h)",
        );
    let info = match native {
        Some(_) => info.config(
            "fill",
            "toolbar.background_color, through geometry::toolbar: upstream's Toolbar is transparent and leaves its surface to the caller (gpui-component/toolbar.rs, Toolbar)",
        ),
        None => info.not_themeable(
            "fill",
            "none of its own: upstream's Toolbar is transparent and leaves its surface to the caller (gpui-component/toolbar.rs, Toolbar)",
        ),
    };
    let info = match native {
        Some(r) if r.toolbar.border.line_width > 0.0 => info.config(
            "edge",
            format!(
                "the bar's bottom rule, toolbar.border.color at toolbar.border.line_width, {}px, inside the bar, through geometry::toolbar: upstream's Toolbar draws no edge of its own (gpui-component/toolbar.rs, RenderOnce for Toolbar)",
                px_text(r.toolbar.border.line_width)
            ),
        ),
        Some(_) => info.config(
            "edge",
            "none: toolbar.border.line_width is 0, so geometry::toolbar draws no rule, and upstream's Toolbar draws no edge of its own (gpui-component/toolbar.rs, RenderOnce for Toolbar)",
        ),
        None => info.not_themeable(
            "edge",
            "none of its own (gpui-component/toolbar.rs, RenderOnce for Toolbar)",
        ),
    };
    info.instance("group gap", group_gap)
        .instance(
            "groups",
            "Edit (Undo2, Redo2, Copy) and View (Search, Maximize, Settings), each a ToolbarGroup with its accessible label, a vertical Separator between them; each Button reports itself",
        )
}

/// One of the Basic page's icon buttons (`demo::icon_button`), showing its
/// icon where `drawn`, else its name: the toolbar button the window's
/// toolbar buttons are, given `native`'s `button.border` radius and stated
/// padding sides by the showcase (`demo::tool_box_of`) where a native theme
/// is installed, as [`tool_button_size`] says -- not `geometry::button`, so
/// it has no edge at rest; where `native` states the toolbar's outline,
/// `geometry::toolbar_button_rest` gives it a transparent one, coloured
/// under the pointer and while pressed. `checked` is `Some(on)` for the
/// edit-mode switch (`Showcase::basic_edit`), `None` for a button that
/// switches nothing; while on, the Button is selected and refined by
/// `geometry::toolbar_button_checked`, and claims only what that paints
/// ([`toolbar_checked`]).
pub fn icon_button(
    t: &Theme,
    drawn: bool,
    checked: Option<bool>,
    native: Option<&ResolvedTheme>,
) -> WidgetInfo {
    let info = WidgetInfo::new("Button").variant(match (checked, drawn) {
        (None | Some(false), true) => "Toolbar, icon",
        (None | Some(false), false) => "Toolbar, labelled",
        (Some(true), true) => "Toolbar, icon, checked",
        (Some(true), false) => "Toolbar, labelled, checked",
    });
    let switch = "an edit-mode switch, on at start; a click switches it (Showcase::basic_edit)";
    let info = match checked {
        Some(true) => toolbar_variant(toolbar_checked(info, t, native).instance("state", switch)),
        Some(false) => native_toolbar(info.instance("state", switch), t, native),
        None => native_toolbar(info, t, native),
    };
    let outlined = native.is_some_and(|r| r.toolbar.button_hover_border_color.is_some());
    let pen = native.is_some_and(|r| r.toolbar.button_checked_border_color.is_some());
    let info = match (outlined, checked) {
        (true, Some(true)) if pen => info.config(
            "edge",
            "the checked pen, toolbar.button_checked_border_color, which geometry::toolbar_button_checked sets on geometry::toolbar_button_rest's border, button.border.line_width wide, and which stays under the pointer and while pressed (the hover outline while checked)",
        ),
        _ => info.not_themeable(
            "edge",
            match (outlined, checked) {
                (true, Some(true)) => "none: geometry::toolbar_button_rest's border is transparent, button.border.line_width wide, and stays so while checked (the hover outline while checked)",
                (true, _) => "none at rest: geometry::toolbar_button_rest's border is transparent, button.border.line_width wide, and takes the variant's colour under the pointer and while pressed (the border, hovered or pressed)",
                (false, _) => "none: upstream gives a border width only to a Default or an outlined Button, and a Ghost is a custom variant (button/button.rs, RenderOnce for Button); the showcase sets none",
            },
        ),
    };
    let info = match native {
        Some(_) => info,
        None => info.config("border-radius", format!("radius: {}px", t.radius.as_f32())),
    };
    tool_button_size(
        info,
        ToolSize::Medium,
        drawn,
        native.map(|r| &r.button),
        rest_border(native),
    )
}

/// One of the component `Toolbar`'s flat Buttons, showing `icon` of the
/// icon theme named `set`, coloured as [`native_toolbar`] says and sized as
/// [`tool_button_size`] says under `native`'s `button`; it runs nothing. Its
/// icon-size line is recorded where `demo::toolbar_item` applies the builder.
pub fn toolbar_item(
    t: &Theme,
    icon: &ChromeIcon,
    set: &str,
    native: Option<&ResolvedTheme>,
) -> WidgetInfo {
    let drawn = !matches!(icon, ChromeIcon::Missing(_) | ChromeIcon::Unlisted(_));
    let info = WidgetInfo::new("Button").variant(if drawn {
        "Toolbar, icon"
    } else {
        "Toolbar, labelled"
    });
    let info = native_toolbar(info, t, native).not_themeable("fill", toolbar_fill(native));
    let info = if drawn {
        info.not_themeable("icon", super::chrome::child_icon_line(icon))
    } else {
        info
    };
    tool_button_size(
        info,
        ToolSize::Medium,
        drawn,
        native.map(|r| &r.button),
        rest_border(native),
    )
    .instance(
        "icon",
        chrome_icon_note(icon, set, "the button shows its name as its label instead"),
    )
    .instance(
        "action",
        "none: the section shows the Toolbar, so its Buttons run nothing",
    )
}
