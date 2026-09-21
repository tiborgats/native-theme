//! Style functions that give iced's widgets the platform's own colors.
//!
//! iced derives every widget style from the six-color `Palette`, so a widget
//! it has no palette slot for -- a button's pressed fill, an input's focus
//! border -- is a value iced invents by lightening or darkening. The palette
//! this connector builds cannot correct that: the slot does not exist. These
//! functions do, by handing each widget the resolved theme's own field.
//!
//! Each function takes a `&ResolvedTheme`, captures the handful of values it
//! needs by value, and returns a `'static` closure a widget can store. Its doc
//! comment names the iced default it replaces and the setter it is passed to.
//!
//! A `Style` field the native model does not carry is read from iced's own
//! default for that widget, inside the closure, never written as a literal.

use crate::palette::to_color;
use iced_core::border::Radius;
use iced_core::theme::Theme;
use iced_core::{Background, Border, Color};
use native_theme::theme::ResolvedTheme;

// Every widget module of `iced_widget` shares its name with the helper
// function that builds that widget, and these functions share it too, so each
// one imports its own `Status` and `Style` locally rather than the module.

/// Composite `layer` over `base`: straight-alpha source-over, the general
/// case.
///
/// The platform states a widget's hover and pressed colors as layers it paints
/// over that widget's own fill, while iced replaces the fill outright, so the
/// layer is flattened onto the fill before it is emitted (C17).
///
/// `base` is not assumed opaque, because it often is not: every preset states
/// `link.background_color = "#00000000"`, and a tint over nothing must stay
/// that tint rather than turn into an opaque near-black. So the result keeps
/// the composite's own alpha, `la + ba * (1 - la)`, and its channels are
/// un-premultiplied by it. A fully transparent result has no color to divide
/// out and is returned as `Color::TRANSPARENT`.
///
/// The blend is deliberately in sRGB rather than in linear light. It is not a
/// physical mix of two lights: it reproduces the composite the platform itself
/// performed when it measured the value the preset records (spec section 3.2),
/// and platform compositors -- and iced's own renderer -- blend in sRGB. A
/// linear-light blend here would give a color the platform never shows.
pub(crate) fn composite_over(layer: Color, base: Color) -> Color {
    // How much of `base` reaches the result, in the composite's own alpha.
    let under = base.a * (1.0 - layer.a);
    let alpha = layer.a + under;
    if alpha <= 0.0 {
        return Color::TRANSPARENT;
    }
    if under <= 0.0 {
        // Nothing of `base` reaches the result, so the composite is `layer`.
        // Returning it is not a shortcut: un-premultiplying would divide by
        // `layer.a` and give back a color a few ulps off, which a contract
        // row compares exactly. This is the case a link hover takes, over a
        // `link.background_color` every preset states as fully transparent.
        return layer;
    }
    let blend = |layered: f32, beneath: f32| (layered * layer.a + beneath * under) / alpha;
    Color {
        r: blend(layer.r, base.r),
        g: blend(layer.g, base.g),
        b: blend(layer.b, base.b),
        a: alpha,
    }
}

/// The platform's own plain button, for `button(..).style(..)`.
///
/// This is the neutral class, so it replaces `iced_widget::button::secondary`,
/// which paints the palette's `secondary` family and derives its hovered fill
/// by strengthening that slot. It is not iced's *default* class: a `Button`
/// with no `.style(..)` gets `button::primary` (`button.rs:588-590`), which
/// `styles::button_primary` replaces.
///
/// Here every state is a native field: `button.background_color`,
/// `.hover_background`, `.active_background`, `.disabled_background` and the
/// matching label colors, with the button's own border.
///
/// `shadow` and `snap` have no native source and come from
/// `button::Style::default()`: the model carries a shadow color but no offset
/// or blur, and `snap` is a renderer setting (`cfg!(feature = "crisp")`).
#[must_use = "this returns the style function; it does not apply it"]
pub fn button(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + use<> {
    use iced_widget::button::{Status, Style};

    let b = &resolved.button;

    let idle = to_color(b.background_color);
    let hovered = composite_over(to_color(b.hover_background), idle);
    // `active_background` is a soft option: `None` is the platform saying the
    // pressed state has no fill of its own, so it copies the hover layer.
    let pressed = composite_over(
        to_color(b.active_background.unwrap_or(b.hover_background)),
        idle,
    );
    // A translucent disabled fill replaces the idle one and lets the window
    // through, so it is emitted as given rather than composited.
    let disabled = to_color(b.disabled_background.unwrap_or(b.background_color));

    let label = to_color(b.font.color);
    let hovered_label = to_color(b.hover_text_color);
    let pressed_label = to_color(b.active_text_color);
    let disabled_label = to_color(b.disabled_text_color);

    let border = Border {
        color: to_color(b.border.color),
        width: b.border.line_width,
        radius: Radius::new(b.border.corner_radius),
    };

    move |_theme, status| {
        let iced = Style::default();
        let (background, text_color) = match status {
            Status::Active => (idle, label),
            Status::Hovered => (hovered, hovered_label),
            Status::Pressed => (pressed, pressed_label),
            Status::Disabled => (disabled, disabled_label),
        };
        Style {
            background: Some(Background::Color(background)),
            text_color,
            border,
            shadow: iced.shadow,
            snap: iced.snap,
        }
    }
}
