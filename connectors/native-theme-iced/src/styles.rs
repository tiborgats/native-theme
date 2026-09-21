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
use native_theme::color::Rgba;
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

/// A button class the platform names but does not describe in every state:
/// `fill` and `label` are the idle pair it states, `class` is the iced class
/// this replaces.
///
/// The model gives the accent and the three status colors a fill and a label
/// and nothing else -- no hovered and no pressed variant of either -- and the
/// neutral `button.hover_background` is not theirs to borrow: it is an opaque
/// grey on fifteen of the sixteen presets, which would turn a hovered accent
/// button grey. So those two states follow the no-source rule and come from
/// `class` at run time, which derives them from the same color through the
/// palette (spec section 3.3).
///
/// The border is the button's, not iced's `border::rounded(2)`
/// (`button.rs:739`), so such a button sits beside a native one instead of
/// beside a differently rounded one. Disabled is the button's disabled pair:
/// the platform dims every button class the same way.
fn class_button(
    resolved: &ResolvedTheme,
    fill: Rgba,
    label: Rgba,
    class: fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + use<> {
    use iced_widget::button::{Status, Style};

    let b = &resolved.button;

    let idle = to_color(fill);
    let idle_label = to_color(label);
    let disabled = to_color(b.disabled_background.unwrap_or(b.background_color));
    let disabled_label = to_color(b.disabled_text_color);

    let border = Border {
        color: to_color(b.border.color),
        width: b.border.line_width,
        radius: Radius::new(b.border.corner_radius),
    };

    move |theme, status| {
        let iced = class(theme, status);
        let (background, text_color) = match status {
            Status::Active => (Some(Background::Color(idle)), idle_label),
            // Neither side of these two states has a native source, so both
            // are iced's own answer for this class.
            Status::Hovered | Status::Pressed => (iced.background, iced.text_color),
            Status::Disabled => (Some(Background::Color(disabled)), disabled_label),
        };
        Style {
            background,
            text_color,
            border,
            shadow: iced.shadow,
            snap: iced.snap,
        }
    }
}

/// The platform's own primary button, for `button(..).style(..)`.
///
/// Replaces `iced_widget::button::primary`, which is also iced's *default*
/// class (`button.rs:588-590`): it paints the palette's `primary` family and
/// derives its hovered fill by strengthening that slot.
///
/// The idle pair is the platform's accent pair, `button.primary_background`
/// and `.primary_text_color`; see [`class_button`] for what the other states
/// are and why. That derivation starts from the idle fill itself: the palette
/// slot iced strengthens is `defaults.accent_color`, which every bundled
/// preset also states as `button.primary_background` -- pinned by the contract
/// (`the_primary_pair_is_the_accent_pair`).
#[must_use = "this returns the style function; it does not apply it"]
pub fn button_primary(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + use<> {
    class_button(
        resolved,
        resolved.button.primary_background,
        resolved.button.primary_text_color,
        iced_widget::button::primary,
    )
}

/// The platform's own destructive button, for `button(..).style(..)`.
///
/// Replaces `iced_widget::button::danger`, which paints the palette's `danger`
/// family on a hardcoded `border::rounded(2)`. The idle fill and label are
/// `defaults.danger_color` and `.danger_text_color`; see [`class_button`] for
/// what the other states are and why.
#[must_use = "this returns the style function; it does not apply it"]
pub fn button_danger(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + use<> {
    class_button(
        resolved,
        resolved.defaults.danger_color,
        resolved.defaults.danger_text_color,
        iced_widget::button::danger,
    )
}

/// The platform's own confirming button, for `button(..).style(..)`.
///
/// Replaces `iced_widget::button::success`. The idle fill and label are
/// `defaults.success_color` and `.success_text_color`; see [`class_button`]
/// for what the other states are and why.
#[must_use = "this returns the style function; it does not apply it"]
pub fn button_success(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + use<> {
    class_button(
        resolved,
        resolved.defaults.success_color,
        resolved.defaults.success_text_color,
        iced_widget::button::success,
    )
}

/// The platform's own risky-action button, for `button(..).style(..)`.
///
/// Replaces `iced_widget::button::warning`. The idle fill and label are
/// `defaults.warning_color` and `.warning_text_color`; see [`class_button`]
/// for what the other states are and why.
#[must_use = "this returns the style function; it does not apply it"]
pub fn button_warning(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + use<> {
    class_button(
        resolved,
        resolved.defaults.warning_color,
        resolved.defaults.warning_text_color,
        iced_widget::button::warning,
    )
}

/// The platform's own hyperlink, as a button, for `button(..).style(..)`.
///
/// Replaces `iced_widget::button::text`, which paints the window's own label
/// and dims it to 80 % alpha on hover. The platform states its link colors
/// instead: `link.font.color`, `.hover_text_color`, `.active_text_color` and
/// `.disabled_text_color` on `link.background_color`, with `.hover_background`
/// layered over that fill (C17) -- and every preset states the fill as fully
/// transparent, which is the case that layering was made general for.
///
/// `LinkTheme` states no pressed and no disabled fill, so both keep the idle
/// one: the copy rule of spec section 3.2. It carries no border either, and
/// no shadow geometry, so `border`, `shadow` and `snap` are iced's own for a
/// text button.
#[must_use = "this returns the style function; it does not apply it"]
pub fn button_link(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + use<> {
    use iced_widget::button::{Status, Style};

    let l = &resolved.link;

    let idle = to_color(l.background_color);
    let hovered = composite_over(to_color(l.hover_background), idle);

    let label = to_color(l.font.color);
    let hovered_label = to_color(l.hover_text_color);
    let pressed_label = to_color(l.active_text_color);
    let disabled_label = to_color(l.disabled_text_color);

    move |theme, status| {
        let iced = iced_widget::button::text(theme, status);
        let (background, text_color) = match status {
            Status::Active => (idle, label),
            Status::Hovered => (hovered, hovered_label),
            Status::Pressed => (idle, pressed_label),
            Status::Disabled => (idle, disabled_label),
        };
        Style {
            background: Some(Background::Color(background)),
            text_color,
            border: iced.border,
            shadow: iced.shadow,
            snap: iced.snap,
        }
    }
}

/// The platform's own text field, for `text_input(..).style(..)` and for
/// `ComboBox::input_style(..)`.
///
/// Replaces `iced_widget::text_input::default` (`text_input.rs:1758`), which
/// paints the field on the *window* background and derives its hovered and
/// focused borders from the palette's background and primary families.
///
/// Every color is `input.*`: the fill, the border and its hovered and focused
/// colors, the placeholder, the value and the selection. The disabled state is
/// the platform's disabled fill and disabled text color, both of which the
/// model states. `icon` has no native source -- the model carries no
/// input-icon color (spec section 3.2) -- and comes from
/// `text_input::default(theme, status)`.
#[must_use = "this returns the style function; it does not apply it"]
pub fn text_input(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::text_input::Status) -> iced_widget::text_input::Style + use<> {
    use iced_widget::text_input::{Status, Style};

    let i = &resolved.input;

    let idle = to_color(i.background_color);
    // A translucent disabled fill replaces the idle one and lets the window
    // through, so it is emitted as given.
    let disabled = to_color(i.disabled_background.unwrap_or(i.background_color));

    // Both border soft options fall back to the border's own color: the
    // platform saying the field does not change outline in that state.
    let idle_border = to_color(i.border.color);
    let hover_border = to_color(i.hover_border_color.unwrap_or(i.border.color));
    let focus_border = to_color(i.focus_border_color.unwrap_or(i.border.color));
    let border_width = i.border.line_width;
    let border_radius = Radius::new(i.border.corner_radius);

    let placeholder = to_color(i.placeholder_color);
    let text = to_color(i.font.color);
    let disabled_text = to_color(i.disabled_text_color);
    let selection = to_color(i.selection_background);

    move |theme, status| {
        let iced = iced_widget::text_input::default(theme, status);
        let (background, border_color, value) = match status {
            Status::Active => (idle, idle_border, text),
            Status::Hovered => (idle, hover_border, text),
            Status::Focused { is_hovered: _ } => (idle, focus_border, text),
            Status::Disabled => (disabled, idle_border, disabled_text),
        };
        Style {
            background: Background::Color(background),
            border: Border {
                color: border_color,
                width: border_width,
                radius: border_radius,
            },
            icon: iced.icon,
            placeholder,
            value,
            selection,
        }
    }
}

/// The platform's own multi-line text field, for `text_editor(..).style(..)`.
///
/// Replaces `iced_widget::text_editor::default` (`text_editor.rs:1466`), which
/// derives the same values from the palette that `text_input::default` does.
///
/// The model states one `InputTheme` for both, so this reads exactly the
/// sources [`text_input`] reads. `text_editor::Style` has no `icon` field
/// (`text_editor.rs:1425-1436`), so every field it does have is a native one
/// and nothing here comes from iced.
#[must_use = "this returns the style function; it does not apply it"]
pub fn text_editor(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::text_editor::Status) -> iced_widget::text_editor::Style + use<> {
    use iced_widget::text_editor::{Status, Style};

    let i = &resolved.input;

    let idle = to_color(i.background_color);
    let disabled = to_color(i.disabled_background.unwrap_or(i.background_color));

    let idle_border = to_color(i.border.color);
    let hover_border = to_color(i.hover_border_color.unwrap_or(i.border.color));
    let focus_border = to_color(i.focus_border_color.unwrap_or(i.border.color));
    let border_width = i.border.line_width;
    let border_radius = Radius::new(i.border.corner_radius);

    let placeholder = to_color(i.placeholder_color);
    let text = to_color(i.font.color);
    let disabled_text = to_color(i.disabled_text_color);
    let selection = to_color(i.selection_background);

    move |_theme, status| {
        let (background, border_color, value) = match status {
            Status::Active => (idle, idle_border, text),
            Status::Hovered => (idle, hover_border, text),
            Status::Focused { is_hovered: _ } => (idle, focus_border, text),
            Status::Disabled => (disabled, idle_border, disabled_text),
        };
        Style {
            background: Background::Color(background),
            border: Border {
                color: border_color,
                width: border_width,
                radius: border_radius,
            },
            placeholder,
            value,
            selection,
        }
    }
}
