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
            // A field that is focused *and* hovered takes the focus border,
            // as iced's own default does (`text_input.rs:1787`).
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
            // Focused and hovered takes the focus border here too, as iced's
            // own default does (`text_editor.rs:1490`).
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

/// The platform's own checkbox, for `checkbox(..).style(..)`.
///
/// Replaces `iced_widget::checkbox::primary` (`checkbox.rs:569`), the class a
/// `Checkbox` gets with no `.style(..)`: it paints the palette's background
/// family and takes the check mark from `primary.base.text`.
///
/// The box has two fills, `checkbox.checked_background` and
/// `.unchecked_background`, and the `Status` says which one it is showing. The
/// check mark is `checkbox.indicator_color` -- the model has no `check_color`.
/// `.hover_background` is a layer over whichever fill the box shows (C17);
/// `.disabled_background` replaces it, as given. An unchecked box may state a
/// border color of its own, `.unchecked_border_color`.
///
/// Nothing here comes from iced: every field of `checkbox::Style` has a native
/// source. The label is `checkbox.font.color`, and `.disabled_text_color` when
/// the box is disabled -- the platform dims it, where iced's own class leaves
/// the label to be inherited whole.
#[must_use = "this returns the style function; it does not apply it"]
pub fn checkbox(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::checkbox::Status) -> iced_widget::checkbox::Style + use<> {
    use iced_widget::checkbox::{Status, Style};

    let c = &resolved.checkbox;

    let checked = to_color(c.checked_background);
    // Both soft options copy the checkbox's own fill: the platform saying the
    // box looks no different unchecked, or under the pointer.
    let unchecked = to_color(c.unchecked_background.unwrap_or(c.background_color));
    let hover_layer = to_color(c.hover_background.unwrap_or(c.background_color));
    let hovered_checked = composite_over(hover_layer, checked);
    let hovered_unchecked = composite_over(hover_layer, unchecked);
    // A translucent disabled fill replaces the idle one, so it is as given.
    let disabled = to_color(c.disabled_background.unwrap_or(c.background_color));

    let mark = to_color(c.indicator_color);
    let label = to_color(c.font.color);
    let disabled_label = to_color(c.disabled_text_color);

    let checked_border = to_color(c.border.color);
    let unchecked_border = to_color(c.unchecked_border_color.unwrap_or(c.border.color));
    let border_width = c.border.line_width;
    let border_radius = Radius::new(c.border.corner_radius);

    move |_theme, status| {
        let (background, border_color, text_color) = match status {
            Status::Active { is_checked } => {
                if is_checked {
                    (checked, checked_border, label)
                } else {
                    (unchecked, unchecked_border, label)
                }
            }
            Status::Hovered { is_checked } => {
                if is_checked {
                    (hovered_checked, checked_border, label)
                } else {
                    (hovered_unchecked, unchecked_border, label)
                }
            }
            // The platform states one disabled fill for both, and the box
            // keeps the outline that says whether it is checked.
            Status::Disabled { is_checked } => (
                disabled,
                if is_checked {
                    checked_border
                } else {
                    unchecked_border
                },
                disabled_label,
            ),
        };
        Style {
            background: Background::Color(background),
            icon_color: mark,
            border: Border {
                color: border_color,
                width: border_width,
                radius: border_radius,
            },
            text_color: Some(text_color),
        }
    }
}

/// The platform's own radio button, for `radio(..).style(..)`.
///
/// Replaces `iced_widget::radio::default` (`radio.rs:532`), which paints a
/// transparent circle outlined in the palette's primary family.
///
/// The model states one `CheckboxTheme` for both controls
/// (`widgets/mod.rs:138-140`), so this reads exactly the sources [`checkbox`]
/// reads, with `is_selected` where the checkbox has `is_checked` and the check
/// mark serving as the dot. `radio::Status` has no disabled variant
/// (`radio.rs:474-487`), so the model's disabled fields have no receiver here.
///
/// `radio::Style` carries its border as a width and a color rather than as an
/// iced `Border`, so there is no corner radius to give it.
#[must_use = "this returns the style function; it does not apply it"]
pub fn radio(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::radio::Status) -> iced_widget::radio::Style + use<> {
    use iced_widget::radio::{Status, Style};

    let c = &resolved.checkbox;

    let selected = to_color(c.checked_background);
    let unselected = to_color(c.unchecked_background.unwrap_or(c.background_color));
    let hover_layer = to_color(c.hover_background.unwrap_or(c.background_color));
    let hovered_selected = composite_over(hover_layer, selected);
    let hovered_unselected = composite_over(hover_layer, unselected);

    let dot = to_color(c.indicator_color);
    let label = to_color(c.font.color);

    let selected_border = to_color(c.border.color);
    let unselected_border = to_color(c.unchecked_border_color.unwrap_or(c.border.color));
    let border_width = c.border.line_width;

    move |_theme, status| {
        let (background, border_color) = match status {
            Status::Active { is_selected } => {
                if is_selected {
                    (selected, selected_border)
                } else {
                    (unselected, unselected_border)
                }
            }
            Status::Hovered { is_selected } => {
                if is_selected {
                    (hovered_selected, selected_border)
                } else {
                    (hovered_unselected, unselected_border)
                }
            }
        };
        Style {
            background: Background::Color(background),
            dot_color: dot,
            border_width,
            border_color,
            text_color: Some(label),
        }
    }
}

/// The platform's own switch, for `toggler(..).style(..)`.
///
/// Replaces `iced_widget::toggler::default` (`toggler.rs:561`), which paints
/// the track from the palette's primary family and, hovered and toggled,
/// halves the alpha of the thumb.
///
/// The track is `switch.checked_background` or `.unchecked_background` by
/// `is_toggled`, with `.hover_*` layered over it (C17) and `.disabled_*`
/// replacing it, as given. The thumb is `switch.thumb_background`, a thumb and
/// so emitted as given in every state, and `.disabled_thumb_color` when the
/// switch is off.
///
/// `border_radius` is `switch.track_radius`, and it shapes the whole widget:
/// iced paints the track and the thumb as two quads with the *same* radius
/// (`toggler.rs:435`, `:461`), and its own `None` would make both perfectly
/// round (`toggler.rs:427-429`) whatever the platform states.
///
/// Six fields have no native source and come from `toggler::default(theme,
/// status)`: `SwitchTheme` carries neither border nor thumb inset, so both
/// border widths, both border colors and `padding_ratio` are iced's. So is
/// `text_color`: the model states no font for a switch, and iced's `None`
/// inherits the surrounding one.
#[must_use = "this returns the style function; it does not apply it"]
pub fn toggler(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::toggler::Status) -> iced_widget::toggler::Style + use<> {
    use iced_widget::toggler::{Status, Style};

    let s = &resolved.switch;

    let checked = to_color(s.checked_background);
    let unchecked = to_color(s.unchecked_background);
    // Each hover layer is a soft option copying the track it covers.
    let hovered_checked = composite_over(
        to_color(s.hover_checked_background.unwrap_or(s.checked_background)),
        checked,
    );
    let hovered_unchecked = composite_over(
        to_color(
            s.hover_unchecked_background
                .unwrap_or(s.unchecked_background),
        ),
        unchecked,
    );
    let disabled_checked = to_color(
        s.disabled_checked_background
            .unwrap_or(s.checked_background),
    );
    let disabled_unchecked = to_color(
        s.disabled_unchecked_background
            .unwrap_or(s.unchecked_background),
    );

    let thumb = to_color(s.thumb_background);
    let disabled_thumb = to_color(s.disabled_thumb_color.unwrap_or(s.thumb_background));

    let track_radius = Radius::new(s.track_radius);

    move |theme, status| {
        let iced = iced_widget::toggler::default(theme, status);
        let (background, foreground) = match status {
            Status::Active { is_toggled } => {
                if is_toggled {
                    (checked, thumb)
                } else {
                    (unchecked, thumb)
                }
            }
            Status::Hovered { is_toggled } => {
                if is_toggled {
                    (hovered_checked, thumb)
                } else {
                    (hovered_unchecked, thumb)
                }
            }
            Status::Disabled { is_toggled } => {
                if is_toggled {
                    (disabled_checked, disabled_thumb)
                } else {
                    (disabled_unchecked, disabled_thumb)
                }
            }
        };
        Style {
            background: Background::Color(background),
            background_border_width: iced.background_border_width,
            background_border_color: iced.background_border_color,
            foreground: Background::Color(foreground),
            foreground_border_width: iced.foreground_border_width,
            foreground_border_color: iced.foreground_border_color,
            text_color: iced.text_color,
            border_radius: Some(track_radius),
            padding_ratio: iced.padding_ratio,
        }
    }
}

/// The platform's own drop-down field, for `pick_list(..).style(..)`.
///
/// Replaces `iced_widget::pick_list::default` (`pick_list.rs:904`), which
/// paints the field in the palette's weak background and outlines it in the
/// primary family once it is hovered or open.
///
/// The field is `combo_box.background_color` with `.hover_background` layered
/// over it (C17), its label is `combo_box.font.color` and its border is
/// `combo_box.border.*`. The placeholder is `input.placeholder_color`: the
/// model states it once, for every field that has one.
///
/// `pick_list::Status::Opened` carries whether the pointer is on the field
/// (`pick_list.rs:838-849`), and that is what decides the fill here -- the
/// model states no separate open appearance. `handle_color` has no native
/// source, `ComboBoxTheme` carrying the arrow's sizes but not its color, so it
/// comes from `pick_list::default(theme, status)`.
#[must_use = "this returns the style function; it does not apply it"]
pub fn pick_list(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::pick_list::Status) -> iced_widget::pick_list::Style + use<> {
    use iced_widget::pick_list::{Status, Style};

    let c = &resolved.combo_box;

    let idle = to_color(c.background_color);
    let hovered = composite_over(
        to_color(c.hover_background.unwrap_or(c.background_color)),
        idle,
    );

    let label = to_color(c.font.color);
    let placeholder = to_color(resolved.input.placeholder_color);

    let border = Border {
        color: to_color(c.border.color),
        width: c.border.line_width,
        radius: Radius::new(c.border.corner_radius),
    };

    move |theme, status| {
        let iced = iced_widget::pick_list::default(theme, status);
        let background = match status {
            Status::Active => idle,
            Status::Hovered => hovered,
            Status::Opened { is_hovered } => {
                if is_hovered {
                    hovered
                } else {
                    idle
                }
            }
        };
        Style {
            text_color: label,
            placeholder_color: placeholder,
            handle_color: iced.handle_color,
            background: Background::Color(background),
            border,
        }
    }
}

/// The platform's own menu, for `.menu_style(..)` on a `PickList` or a
/// `ComboBox`.
///
/// Replaces `iced_widget::overlay::menu::default` (`overlay/menu.rs:646`),
/// which paints the list in the palette's weak background and its selected row
/// in the primary family. This is the one function of shape C: a menu has no
/// `Status`, so the closure takes the theme alone.
///
/// Every color is `menu.*`: the panel, its border, the label, and the selected
/// row's label and fill. A selected row is a row highlight, painted over a
/// panel the widget also paints, so it is emitted as given rather than
/// composited (spec section 3.2).
///
/// `shadow` has no native source -- the model has `defaults.shadow_color` but
/// no offset or blur -- and comes from `overlay::menu::default(theme)`.
#[must_use = "this returns the style function; it does not apply it"]
pub fn menu(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme) -> iced_widget::overlay::menu::Style + use<> {
    use iced_widget::overlay::menu::Style;

    let m = &resolved.menu;

    let background = to_color(m.background_color);
    let label = to_color(m.font.color);
    let selected_label = to_color(m.hover_text_color);
    let selected_background = to_color(m.hover_background);

    let border = Border {
        color: to_color(m.border.color),
        width: m.border.line_width,
        radius: Radius::new(m.border.corner_radius),
    };

    move |theme| {
        let iced = iced_widget::overlay::menu::default(theme);
        Style {
            background: Background::Color(background),
            border,
            text_color: label,
            selected_text_color: selected_label,
            selected_background: Background::Color(selected_background),
            shadow: iced.shadow,
        }
    }
}

/// The platform's own slider, for `slider(..).style(..)` and for
/// `vertical_slider(..).style(..)` -- the vertical widget re-exports this one's
/// `Style` and `Status` (`vertical_slider.rs:33-35`).
///
/// Replaces `iced_widget::slider::default` (`slider.rs:676`), which paints
/// both the filled rail and the handle from the palette's primary family and
/// the remaining rail from the strong background.
///
/// The rail's two backgrounds are `slider.fill_color` and `.track_color`, and
/// its width is `slider.track_height`. The handle is `slider.thumb_color`,
/// with `.thumb_hover_color` under the pointer -- a thumb, so emitted as given
/// -- and its size is `slider.thumb_diameter`, halved because iced states a
/// circular handle by its radius.
///
/// Two things have no native source. The model states no dragged thumb color,
/// so that one state's handle is `slider::default(theme, Status::Dragged)`'s,
/// as a status button's hovered fill is iced's (spec section 3.2); it is
/// asserted against iced's own by
/// `a_dragged_slider_handle_comes_from_iced`. And `SliderTheme` carries no
/// border, so the rail's border and the handle's border width and color are
/// iced's too.
#[must_use = "this returns the style function; it does not apply it"]
pub fn slider(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::slider::Status) -> iced_widget::slider::Style + use<> {
    use iced_widget::slider::{Handle, HandleShape, Rail, Status, Style};

    let s = &resolved.slider;

    let filled = to_color(s.fill_color);
    let remaining = to_color(s.track_color);
    let rail_width = s.track_height;

    let thumb = to_color(s.thumb_color);
    // A soft option: `None` is the platform saying the thumb does not change
    // under the pointer, so it copies the thumb's own color.
    let hovered_thumb = to_color(s.thumb_hover_color.unwrap_or(s.thumb_color));
    // The model states the thumb's diameter; iced states a circular handle by
    // its radius.
    let handle_radius = s.thumb_diameter / 2.0;

    move |theme, status| {
        let iced = iced_widget::slider::default(theme, status);
        let handle_background = match status {
            Status::Active => Background::Color(thumb),
            Status::Hovered => Background::Color(hovered_thumb),
            Status::Dragged => iced.handle.background,
        };
        Style {
            rail: Rail {
                backgrounds: (Background::Color(filled), Background::Color(remaining)),
                width: rail_width,
                border: iced.rail.border,
            },
            handle: Handle {
                shape: HandleShape::Circle {
                    radius: handle_radius,
                },
                background: handle_background,
                border_width: iced.handle.border_width,
                border_color: iced.handle.border_color,
            },
        }
    }
}
