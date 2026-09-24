//! What the Buttons page's widgets report about themselves (spec §3.4).

use gpui_component::{Colorize as _, Size, theme::Theme};

use super::{
    ColorClaim, WidgetInfo,
    chrome::{GhostContent, ghost_colours, input_background},
    claim,
};
use crate::demo::{ButtonKind, ButtonState};

/// What a Button of `kind` paints at rest: its fill, where it has one, and
/// its label.
fn at_rest(t: &Theme, kind: ButtonKind) -> Vec<ColorClaim> {
    match kind {
        ButtonKind::Default => vec![
            claim(
                "bg",
                "button",
                t.button,
                "gpui-component/button/button.rs:935",
            ),
            claim(
                "text",
                "button_foreground",
                t.button_foreground,
                "gpui-component/button/button.rs:949",
            ),
        ],
        ButtonKind::Primary => vec![
            claim(
                "bg",
                "button_primary",
                t.button_primary,
                "gpui-component/button/button.rs:936",
            ),
            claim(
                "text",
                "button_primary_foreground",
                t.button_primary_foreground,
                "gpui-component/button/button.rs:954",
            ),
        ],
        ButtonKind::Secondary => vec![
            claim(
                "bg",
                "button_secondary",
                t.button_secondary,
                "gpui-component/button/button.rs:937",
            ),
            claim(
                "text",
                "button_secondary_foreground",
                t.button_secondary_foreground,
                "gpui-component/button/button.rs:961",
            ),
        ],
        ButtonKind::Danger => vec![
            claim(
                "bg",
                "button_danger",
                t.button_danger,
                "gpui-component/button/button.rs:938",
            ),
            claim(
                "text",
                "button_danger_foreground",
                t.button_danger_foreground,
                "gpui-component/button/button.rs:969",
            ),
        ],
        ButtonKind::Success => vec![
            claim(
                "bg",
                "button_success",
                t.button_success,
                "gpui-component/button/button.rs:940",
            ),
            claim(
                "text",
                "button_success_foreground",
                t.button_success_foreground,
                "gpui-component/button/button.rs:983",
            ),
        ],
        ButtonKind::Warning => vec![
            claim(
                "bg",
                "button_warning",
                t.button_warning,
                "gpui-component/button/button.rs:939",
            ),
            claim(
                "text",
                "button_warning_foreground",
                t.button_warning_foreground,
                "gpui-component/button/button.rs:976",
            ),
        ],
        ButtonKind::Info => vec![
            claim(
                "bg",
                "button_info",
                t.button_info,
                "gpui-component/button/button.rs:941",
            ),
            claim(
                "text",
                "button_info_foreground",
                t.button_info_foreground,
                "gpui-component/button/button.rs:990",
            ),
        ],
        ButtonKind::Ghost => vec![claim(
            "text",
            "secondary_foreground",
            t.secondary_foreground,
            "native-theme-gpui/variants.rs:54",
        )],
        ButtonKind::Link => vec![claim(
            "text",
            "link",
            t.link,
            "gpui-component/button/button.rs:993",
        )],
        ButtonKind::Text => vec![claim(
            "text, foreground at 90%",
            "foreground",
            t.foreground.opacity(0.9),
            "gpui-component/button/button.rs:994",
        )],
        ButtonKind::DefaultOutline => vec![
            input_background(t),
            claim(
                "text",
                "button_foreground",
                t.button_foreground,
                "gpui-component/button/button.rs:949",
            ),
        ],
        ButtonKind::PrimaryOutline => vec![
            claim(
                "fill, primary at 10%",
                "primary",
                t.primary.opacity(0.1),
                "gpui-component/button/button.rs:871",
            ),
            claim(
                "text",
                "primary",
                t.primary,
                "gpui-component/button/button.rs:952",
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
                "gpui-component/button/button.rs:1079",
            ),
            claim(
                "active",
                "button_active",
                t.button_active,
                "gpui-component/button/button.rs:1163",
            ),
        ],
        ButtonKind::Primary => vec![
            claim(
                "hover",
                "button_primary_hover",
                t.button_primary_hover,
                "gpui-component/button/button.rs:1086",
            ),
            claim(
                "active",
                "button_primary_active",
                t.button_primary_active,
                "gpui-component/button/button.rs:1170",
            ),
        ],
        ButtonKind::Secondary => vec![
            claim(
                "hover",
                "button_secondary_hover",
                t.button_secondary_hover,
                "gpui-component/button/button.rs:1093",
            ),
            claim(
                "active",
                "button_secondary_active",
                t.button_secondary_active,
                "gpui-component/button/button.rs:1177",
            ),
        ],
        ButtonKind::Danger => vec![
            claim(
                "hover",
                "button_danger_hover",
                t.button_danger_hover,
                "gpui-component/button/button.rs:1100",
            ),
            claim(
                "active",
                "button_danger_active",
                t.button_danger_active,
                "gpui-component/button/button.rs:1185",
            ),
        ],
        ButtonKind::Success => vec![
            claim(
                "hover",
                "button_success_hover",
                t.button_success_hover,
                "gpui-component/button/button.rs:1114",
            ),
            claim(
                "active",
                "button_success_active",
                t.button_success_active,
                "gpui-component/button/button.rs:1199",
            ),
        ],
        ButtonKind::Warning => vec![
            claim(
                "hover",
                "button_warning_hover",
                t.button_warning_hover,
                "gpui-component/button/button.rs:1107",
            ),
            claim(
                "active",
                "button_warning_active",
                t.button_warning_active,
                "gpui-component/button/button.rs:1192",
            ),
        ],
        ButtonKind::Info => vec![
            claim(
                "hover",
                "button_info_hover",
                t.button_info_hover,
                "gpui-component/button/button.rs:1121",
            ),
            claim(
                "active",
                "button_info_active",
                t.button_info_active,
                "gpui-component/button/button.rs:1206",
            ),
        ],
        ButtonKind::Ghost => vec![
            claim(
                "hover bg",
                "secondary_hover",
                t.secondary_hover,
                "native-theme-gpui/variants.rs:55",
            ),
            claim(
                "active bg",
                "secondary_active",
                t.secondary_active,
                "native-theme-gpui/variants.rs:56",
            ),
        ],
        ButtonKind::Link => vec![
            claim(
                "hover text",
                "link_hover",
                t.link_hover,
                "gpui-component/button/button.rs:1139",
            ),
            claim(
                "pressed text",
                "link_active",
                t.link_active,
                "gpui-component/button/button.rs:1215",
            ),
        ],
        ButtonKind::Text => vec![
            claim(
                "hover text",
                "foreground",
                t.foreground,
                "gpui-component/button/button.rs:1140",
            ),
            claim(
                "pressed text, foreground at 70%",
                "foreground",
                t.foreground.opacity(0.7),
                "gpui-component/button/button.rs:1216",
            ),
        ],
        ButtonKind::DefaultOutline => vec![
            claim(
                "hover bg, 50% input mixed with 50% transparent",
                "input",
                t.input.mix_oklab(t.transparent, 0.5),
                "gpui-component/button/button.rs:860-864",
            ),
            claim(
                "active bg, 70% input mixed with 30% transparent",
                "input",
                t.input.mix_oklab(t.transparent, 0.7),
                "gpui-component/button/button.rs:865-869",
            ),
        ],
        ButtonKind::PrimaryOutline => vec![
            claim(
                "hover bg, primary_hover at 20%",
                "primary_hover",
                t.primary_hover.opacity(0.2),
                "gpui-component/button/button.rs:874",
            ),
            claim(
                "active bg, primary_active at 40%",
                "primary_active",
                t.primary_active.opacity(0.4),
                "gpui-component/button/button.rs:877",
            ),
        ],
    }
}

/// The edge of a Button of `kind` that upstream draws: only Default and an
/// outlined Button have one (button/button.rs:656-661). `styled` is whether
/// `geometry::button` refined it, which paints the edge at rest with the
/// platform's colour and leaves upstream's to hover and press.
fn edge(t: &Theme, kind: ButtonKind, styled: bool) -> Option<ColorClaim> {
    match (kind, styled) {
        (ButtonKind::Default | ButtonKind::DefaultOutline, true) => Some(claim(
            "border, hovered or pressed",
            "input",
            t.input,
            "gpui-component/button/button.rs:1001",
        )),
        (ButtonKind::Default | ButtonKind::DefaultOutline, false) => Some(claim(
            "border",
            "input",
            t.input,
            "gpui-component/button/button.rs:1001",
        )),
        (ButtonKind::PrimaryOutline, true) => Some(claim(
            "border, hovered or pressed",
            "primary",
            t.primary,
            "gpui-component/button/button.rs:1003",
        )),
        (ButtonKind::PrimaryOutline, false) => Some(claim(
            "border",
            "primary",
            t.primary,
            "gpui-component/button/button.rs:1003",
        )),
        _ => None,
    }
}

/// `info` with what a Button of `kind` paints at rest, its edge, and what it
/// paints under the pointer.
fn colours(info: WidgetInfo, t: &Theme, kind: ButtonKind, styled: bool) -> WidgetInfo {
    let info = at_rest(t, kind).into_iter().fold(info, WidgetInfo::color);
    let info = edge(t, kind, styled)
        .into_iter()
        .fold(info, WidgetInfo::color);
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
            "gpui-component/button/button.rs:1276",
        )),
        // `opacity(1.5)`: gpui clamps the factor to 1 (gpui-pre
        // color.rs:637-644), so the fill is the token as it stands, in both
        // modes -- the arm does not branch on the mode.
        ButtonKind::Secondary => Some(claim(
            "bg, button_secondary at 150% (clamped to 100%)",
            "button_secondary",
            t.button_secondary.opacity(1.5),
            "gpui-component/button/button.rs:1281",
        )),
        ButtonKind::Danger => Some(claim(
            "bg, at 15%",
            "button_danger",
            t.button_danger.opacity(0.15),
            "gpui-component/button/button.rs:1277",
        )),
        // `input_background()` at 0.5 (button/button.rs:1291-1295), which
        // reads a different field in each mode (theme/mod.rs:379-384).
        ButtonKind::Default if t.is_dark() => Some(claim(
            "bg (input mixed toward transparent), at 50%",
            "input",
            t.input_background().opacity(0.5),
            "gpui-component/theme/mod.rs:381",
        )),
        ButtonKind::Default => Some(claim(
            "bg, at 50%",
            "background",
            t.background.opacity(0.5),
            "gpui-component/theme/mod.rs:383",
        )),
        _ => None,
    }
}

/// What a loading Button of `kind` paints: its colours at rest, faded with
/// the whole element to 80% (button/button.rs:782). It is not interactive,
/// so it takes no hover or press style (:493-495, :669).
///
/// Only the variant the page shows loading has an arm: read another's rest
/// arms before the page loads one.
fn loading(t: &Theme, kind: ButtonKind) -> Vec<ColorClaim> {
    match kind {
        ButtonKind::Primary => vec![
            claim(
                "bg, at 80% (the whole Button fades, button.rs:782)",
                "button_primary",
                t.button_primary.opacity(0.8),
                "gpui-component/button/button.rs:936",
            ),
            claim(
                "text, at 80% (the whole Button fades, button.rs:782)",
                "button_primary_foreground",
                t.button_primary_foreground.opacity(0.8),
                "gpui-component/button/button.rs:954",
            ),
        ],
        _ => Vec::new(),
    }
}

/// The name `size` has in the `Size` enum.
fn size_name(size: Size) -> &'static str {
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
///
/// `size` is `Some` only in the row that shows upstream's size scale, which
/// is built without `geometry::button`; every other Button is built at the
/// default size and refined by it where a native theme is installed, which
/// is what `styled` says. Its geometry line is recorded by `native_info`
/// where `demo::button` applies the builder.
pub fn button(
    t: &Theme,
    kind: ButtonKind,
    state: ButtonState,
    icon: bool,
    size: Option<Size>,
    styled: bool,
) -> WidgetInfo {
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
        ButtonState::Idle => colours(info, t, kind, styled),
        ButtonState::Loading => loading(t, kind).into_iter().fold(info, WidgetInfo::color),
        ButtonState::Disabled => {
            let info = match kind {
                ButtonKind::Primary | ButtonKind::Danger => info.not_themeable("fill", "the variant's own token at 0.15, a literal -- not 0.5, and not a disabled token: the model carries button.disabled_background and the platform states one, and upstream reads neither (button/button.rs, ButtonVariant::disabled)"),
                ButtonKind::Secondary => info.not_themeable("fill", "the variant's own token at 1.5, a literal that gpui clamps to 1 (color.rs, Hsla::opacity), so a disabled Secondary keeps its fill while a disabled Primary or Danger fades to 0.15 -- and not a disabled token: the model carries button.disabled_background and the platform states one, and upstream reads neither (button/button.rs, ButtonVariant::disabled)"),
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
                    "gpui-component/button/button.rs:1284",
                ))
        }
    };
    let info = if styled {
        info.config("font-weight", "geometry::button carries button.font.weight. The label is a child that sets its own size from the Size enum (sizing.rs, button_text_size) and would overrule a size from here, but it sets no weight and neither does anything else on that path, so the platform's weight cascades (button/button.rs, Button::render)")
            .not_themeable("edge", match state {
                ButtonState::Idle => format!("button.border.color at rest: geometry::button gives every variant a border, and upstream's hover and press styles repaint it {} -- gpui applies a hover style after the refinement (gpui-pre/elements/div.rs, hover_style)", match kind {
                    ButtonKind::Default | ButtonKind::DefaultOutline => "input",
                    ButtonKind::Primary | ButtonKind::PrimaryOutline => "primary",
                    ButtonKind::Secondary => "border",
                    ButtonKind::Danger => "button_danger",
                    ButtonKind::Success => "button_success",
                    ButtonKind::Warning => "button_warning",
                    ButtonKind::Info => "button_info",
                    ButtonKind::Ghost | ButtonKind::Link | ButtonKind::Text => "transparent, so it vanishes under the pointer",
                }),
                ButtonState::Disabled => "button.border.color, as at rest: upstream's disabled style sets an edge colour of its own and then replays the caller's style over it, so geometry::button's colour wins (button/button.rs, RenderOnce for Button)".to_string(),
                ButtonState::Loading => "button.border.color, and it stays: a loading Button takes no hover or press style, so nothing repaints it (button/button.rs, Button::interactive). It is painted at 80% all the same: while loading, the whole element fades, border included (button/button.rs, Button::render: the opacity(0.8) at :782)".to_string(),
            })
    } else {
        info.config("border-radius", format!("radius: {}px", t.radius.as_f32()))
    };
    let info = if kind == ButtonKind::Ghost {
        info
    } else {
        info.not_themeable("shadow", "none on a standard variant: Theme::shadow, which the connector sets from border.shadow_enabled, reaches only a ButtonCustomVariant built with .shadow(true) (button/button.rs, ButtonVariant::shadow)")
    };
    let info = info.not_themeable("label size", "a fixed ratio of the platform's base, not a value of its own: the label takes button_text_size (sizing.rs, button_text_size), which is text_xs, text_sm or text_base -- all rems -- and gpui-component sets the rem to Theme::font_size (root.rs, Root::render set_rem_size), which this connector fills from the platform font. So it scales with font.size and cannot be set apart from it: button_text_size has no Size::Size arm");
    let info = match kind {
        ButtonKind::Default => info.instance("variant", "no variant method, so ButtonVariant::Default -- the button family, whose own edge colour is input, not border (button/button.rs, ButtonVariant::border_color)"),
        ButtonKind::Ghost => ghost_variant(info),
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
    let info = match state {
        ButtonState::Idle => info,
        ButtonState::Disabled => info
            .not_themeable("text", "muted_foreground at 0.5, so a disabled button does not keep its variant's text colour. button.disabled_text_color is modelled and carried, and upstream reads it nowhere (button/button.rs, ButtonVariant::disabled)")
            .not_themeable("opacity", "button.disabled_opacity is modelled and inherits defaults.disabled_opacity; upstream multiplies its own literals instead, so the platform's figure has no receiver -- Tier U, not an absence")
            .not_themeable("cursor", "the default arrow, not not-allowed: Button sets cursor_default, and only a link or text variant asks for a pointer (button/button.rs, Button::render)"),
        ButtonState::Loading if icon => info
            .not_themeable("spinner", "stands in for the button's icon, so only a button with one shows it. This one's icon is the chosen icon theme's loading icon, which the Button is also given as its loading icon, so the Spinner turns it in place of upstream's own Loader (button/button.rs, Button::loading_icon; button/button_icon.rs, ButtonIcon). It inherits the button's text colour and turns every 0.8s, a literal (spinner.rs, Spinner::new)")
            .not_themeable("interaction", "inert, as a disabled button is, but not styled as one: it keeps its variant's colours and the whole button fades to 0.8 (button/button.rs, Button::interactive)"),
        ButtonState::Loading => info
            .not_themeable("spinner", "none: a Button shows its spinner in place of its icon, so only a button with one shows it (button/button_icon.rs, ButtonIcon), and the chosen icon theme has no loading icon to give this one; upstream's own Loader would be another icon theme's")
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

/// The variant line of a Ghost Button built with
/// `native_theme_gpui::variants::ghost_button`.
fn ghost_variant(info: WidgetInfo) -> WidgetInfo {
    info.instance("variant", "native_theme_gpui::variants::ghost_button: flat like gpui-component's .ghost(), but with the platform's button.hover_background / active_background. Upstream's own .ghost() would hover with the item-highlight pair (button/button.rs, ButtonVariant::hovered Ghost arm), which is the menu selection colour, not a button hover")
}

/// `info` with what the page's Ghost Button paints, unrefined, and its
/// variant line: the window's toolbar buttons and the status bar's
/// side-panel toggle are that Ghost too, and take it from here so they
/// cannot drift apart.
pub(super) fn native_ghost(info: WidgetInfo, t: &Theme) -> WidgetInfo {
    ghost_variant(colours(info, t, ButtonKind::Ghost, false))
}

/// `info` with what a selected Button built with
/// `native_theme_gpui::variants::ghost_button` paints, and its variant line.
/// Upstream fills a selected Button with its variant's selected style,
/// which for a custom variant is the variant's active colour and its
/// foreground (button/button.rs, ButtonVariant::selected), and gives it no
/// hover or press style: those apply only while it is neither disabled nor
/// selected (button/button.rs, RenderOnce for Button).
pub(super) fn native_ghost_selected(info: WidgetInfo, t: &Theme) -> WidgetInfo {
    ghost_variant(
        info.color(claim(
            "selected bg",
            "secondary_active",
            t.secondary_active,
            "native-theme-gpui/variants.rs:56",
        ))
        .color(claim(
            "text",
            "secondary_foreground",
            t.secondary_foreground,
            "native-theme-gpui/variants.rs:54",
        )),
    )
}

/// A `ButtonGroup` of Default Buttons, which report through the group.
pub fn button_group(t: &Theme) -> WidgetInfo {
    colours(WidgetInfo::new("ButtonGroup"), t, ButtonKind::Default, false)
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
    edge(t, kind, false)
        .into_iter()
        .fold(info, WidgetInfo::color)
        .color(claim(
            "menu bg",
            "popover",
            t.popover,
            "gpui-component/styled.rs:197",
        ))
        .not_themeable("dropdown arrow", "a Caret, not an icon the caller passes: the right half is a Button::dropdown_caret and the glyph comes from select.rs, Caret. The shape is fixed, but its colour is not -- upstream paints it with the button variant's own text colour at 75% (button/button.rs, Button::render dropdown_caret), so it follows the platform through the same token the label does")
        .not_themeable("own icons", super::own_icons("the dropdown arrow's ChevronDown (select.rs, Caret)"))
        .instance("halves", "both take the variant of the Button the showcase gives it: DropdownButton passes it on to the caret half (button/dropdown_button.rs, DropdownButton::effective_variant)")
}

/// The notes every Toggle and ToggleGroup the showcase builds shares: all
/// are at the default Size.
pub(super) fn toggle_notes(info: WidgetInfo, t: &Theme) -> WidgetInfo {
    info.config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable("size", "min_w_8 / h_8 at the default Size -- rems, so the platform's font -- and settable: the refinement comes last, so segmented_control.segment_height, its padding and its font would reach a Toggle through the geometry::toggle nobody has written (button/toggle.rs, Toggle::render)")
}

/// The hovered colours of an unchecked Toggle, of either variant.
pub(super) fn toggle_hover(info: WidgetInfo, t: &Theme) -> WidgetInfo {
    info.color(claim(
        "hover bg",
        "accent",
        t.accent,
        "gpui-component/button/toggle.rs:202",
    ))
    .color(claim(
        "hover text",
        "accent_foreground",
        t.accent_foreground,
        "gpui-component/button/toggle.rs:203",
    ))
}

/// The colours of a checked Toggle, of either variant, which is not
/// hoverable (button/toggle.rs:153), and why they are what they are.
pub(super) fn toggle_checked(info: WidgetInfo, t: &Theme) -> WidgetInfo {
    info.color(claim(
        "checked bg",
        "accent",
        t.accent,
        "gpui-component/button/toggle.rs:155",
    ))
    .color(claim(
        "checked text",
        "accent_foreground",
        t.accent_foreground,
        "gpui-component/button/toggle.rs:156",
    ))
    .not_themeable("checked fill", "accent, the menu highlight, by default -- but not out of reach: a Toggle folds the caller's refinement into its checked style too (button/toggle.rs, Toggle::render), so an application that refines a checked Toggle with segmented_control.active_background and active_text_color gets them. Only an unchecked one's hover is Tier U. Nothing applies them: there is no geometry::toggle -- our gap")
}

/// The unchecked fill of a Ghost Toggle.
fn ghost_toggle_fill(info: WidgetInfo) -> WidgetInfo {
    info.not_themeable("unchecked fill", "none: ToggleVariant defaults to Ghost (button/toggle.rs, ToggleVariant), which paints no background and no border; only .outline() fills")
}

/// A `Toggle`, `checked` or not, showing its icon where `drawn`, and its
/// icon's name as its label where the chosen icon theme has none; `icon`
/// says which.
pub fn toggle(t: &Theme, drawn: bool, icon: String, checked: bool) -> WidgetInfo {
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
    toggle_notes(info, t).instance("icon", icon).instance(
        "click",
        "checks or unchecks it; the showcase keeps the state",
    )
}

/// A `ToggleGroup` of unchecked Toggles, which report through the group.
pub fn toggle_group(t: &Theme) -> WidgetInfo {
    toggle_notes(ghost_toggle_fill(toggle_hover(WidgetInfo::new("ToggleGroup").variant("Ghost"), t)), t)
        .not_themeable("gap", "gap_2 between the toggles -- a rem, so the platform's font; only a segmented group drops it (button/toggle.rs, ToggleGroup::segmented)")
        .instance("toggles", "Left, Center and Right, none checked. ToggleGroup::child takes a Toggle, not an element a target could wrap, so they report through the group (button/toggle.rs, ToggleGroup::child)")
}

/// A `Clipboard` copying `value`.
pub fn clipboard(t: &Theme, value: &'static str) -> WidgetInfo {
    WidgetInfo::new("Clipboard")
        .colors(ghost_colours(t, GhostContent::Icon))
        .config("border-radius", format!("radius: {}px", t.radius.as_f32()))
        .not_themeable("surface", "a Clipboard is a ghost Button and reads no theme field of its own (clipboard.rs, Clipboard::render): transparent until hovered, when it takes accent (at half alpha in dark mode) -- the menu highlight, not the button family. Its icon takes the Ghost variant's secondary_foreground")
        .not_themeable("copy icon", "Copy and Check, built inline with no setter to replace them (clipboard.rs, Clipboard)")
        .not_themeable("own icons", super::own_icons("Copy, and Check once it has copied (clipboard.rs, Clipboard)"))
        .instance("value", value)
}
