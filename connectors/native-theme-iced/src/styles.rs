//! Style functions that give iced's widgets the platform's own colors.
//!
//! Requires the `widgets` feature (on by default).
//!
//! iced derives every widget style from the six-color `Palette`, so a widget
//! it has no palette slot for -- a button's pressed fill, an input's focus
//! border -- is a value iced invents by lightening or darkening. The palette
//! this connector builds cannot correct that: the slot does not exist. These
//! functions do, by handing each widget the resolved theme's own field.
//!
//! Pass a function's result to the widget's style setter, from the
//! `ResolvedTheme` the iced `Theme` was built from:
//!
//! ```rust
//! use iced::widget::{Button, TextInput, button, text_input};
//! use native_theme_iced::{from_preset, styles};
//!
//! let (_theme, resolved) = from_preset("kde-breeze", false)?;
//! let save: Button<'_, ()> = button("Save").style(styles::button_primary(&resolved));
//! let name: TextInput<'_, ()> = text_input("Name", "").style(styles::text_input(&resolved));
//! # Ok::<(), native_theme_iced::Error>(())
//! ```
//!
//! Which function styles which widget:
//!
//! | iced widget | Function |
//! |---|---|
//! | `button` | [`button`], [`button_primary`], [`button_danger`], [`button_success`], [`button_warning`], [`button_link`], [`button_toggle`]; [`segment`] and [`expander`] for the buttons of a segmented control and of an expander's header |
//! | `text_input`, and `ComboBox::input_style` | [`text_input`] |
//! | `text_editor` | [`text_editor`] |
//! | `checkbox` | [`checkbox`], [`checkbox_over`] |
//! | `radio` | [`radio`] |
//! | `toggler` | [`toggler`], [`toggler_over`] |
//! | `pick_list` | [`pick_list`]; [`menu`] for its `menu_style`, and a `ComboBox`'s |
//! | `slider`, `vertical_slider` | [`slider`] |
//! | `scrollable` | [`scrollable`]; [`scrollbar`] for its `direction` |
//! | `progress_bar` | [`progress_bar`] |
//! | `rule` | [`rule`]; [`toolbar_rule`] under a toolbar |
//! | `pane_grid` | [`pane_grid`] |
//! | `tooltip` | [`tooltip`] |
//! | `container` | [`container_card`], [`dialog`], [`toolbar`], [`segmented_control`], [`segment_separator`], [`expander_frame`]: the surfaces iced has no widget for |
//! | `iced::application(..).style(..)` | [`application`], the window's background |
//! | the `iced_aw` widgets (feature `iced_aw`) | `aw::card`, `aw::menu`, `aw::tab_bar`, `aw::sidebar`, `aw::selection_list`, `aw::spinner` |
//!
//! Each function takes a `&ResolvedTheme`, captures the handful of values it
//! needs by value, and returns a `'static` closure a widget can store. Its doc
//! comment names the iced default it replaces and the setter it is passed to.
//!
//! Every closure is `Clone`. It captures only colors and floats, so cloning
//! is free -- and an `impl Trait` return type leaks only auto traits, of
//! which `Clone` is not one, so each signature has to say `+ Clone` for a
//! caller to see it. At least one real setter requires it:
//! `iced_aw::SelectionList::new_with` (see `aw::selection_list`, which the
//! `iced_aw` feature adds). A test in the crate's own suite holds the line: it
//! passes every one of these closures to a `Clone`-bounded function, so a
//! signature that lost `+ Clone` would stop compiling.
//!
//! A `Style` field the native model does not carry is read from iced's own
//! default for that widget, inside the closure, never written as a literal.
//! That read happens while the widget is being drawn, from the `&Theme` in
//! effect, so fifteen of these functions -- `button_danger`,
//! `button_success`, `button_warning`, `button_link`, `text_input`,
//! `toggler`, `toggler_over`, `pick_list`, `menu`, `slider`, `scrollable`,
//! `rule`, `toolbar_rule`, `pane_grid` and `application` --
//! assume that theme is the one `to_theme(resolved)` built from the same
//! `ResolvedTheme`. Under any other theme the borrowed fields are derived from
//! a different palette: the hover of `button_danger`, for one, is then
//! strengthened from a danger color that is not its own idle fill.
//! `button_primary` is not among them: it derives its hover from its own fill,
//! and takes only the shadow and snap from iced's class, which that class
//! leaves at `Style::default()` (`button.rs:735-741`).
//!
//! Two of iced's own classes are left alone on purpose: `container::bordered_box`
//! (`container.rs:595-607`) and `button::subtle` (`button.rs:713-715`) paint
//! `background.weakest`, a pair iced invents with `deviate(background, 0.03)`
//! (`iced_core` theme/palette.rs:472, :482) and that no platform states, so the
//! connector neither writes that slot nor offers a replacement for either class.
//! A surface that should be the platform's is [`container_card`], which paints
//! `card.background_color`.
//!
//! Upstream citations in this module are read at iced_widget 0.14.2 and
//! iced_core 0.14.0.

#[cfg(feature = "iced_aw")]
pub mod aw;

use crate::palette::to_color;
use iced_core::border::Radius;
use iced_core::theme::Theme;
use iced_core::{Background, Border, Color};
use native_theme::color::Rgba;
use native_theme::theme::{
    ResolvedCheckboxTheme, ResolvedDefaults, ResolvedInputTheme, ResolvedTheme,
    ResolvedWidgetBorder,
};

// Every widget module of `iced_widget` shares its name with the helper
// function that builds that widget, and these functions share it too, so each
// one imports its own `Status` and `Style` locally rather than the module.

/// `layer` composited over `base` as the connector composites a theme's state
/// layers: straight-alpha source-over in sRGB, so the result covers
/// `layer.a + base.a × (1 − layer.a)` ([`native_theme::color::composite_over`],
/// which this converts for).
///
/// The platform states a widget's hover and pressed colors as layers it paints
/// over that widget's own fill, while iced replaces the fill outright, so the
/// styles flatten the layer onto the fill before they emit it; an application
/// drawing a layered state of its own -- a stated `hover_background` over a
/// control's fill -- matches them through this. `base` is not assumed opaque:
/// every preset states `link.background_color = "#00000000"`, and a tint over
/// nothing stays that tint rather than turning into an opaque near-black. An
/// opaque layer, and any layer over a transparent base, comes back as it is,
/// and a transparent layer gives `base`.
#[must_use]
pub fn composite_over(layer: Color, base: Color) -> Color {
    let [r, g, b, a] = native_theme::color::composite_over(
        [layer.r, layer.g, layer.b, layer.a],
        [base.r, base.g, base.b, base.a],
    );
    Color { r, g, b, a }
}

/// `color` as a disabled widget shows it, faded by the widget's
/// `disabled_opacity`.
///
/// A platform dims a disabled widget by its disabled colours or by fading the
/// whole widget, and the data makes the one it does not use an identity
/// (`docs/platform-facts.md` §2.1.6: 1.0 on KDE and Windows, no disabled
/// colours of its own on GNOME), so a disabled style applies both. iced has
/// no widget opacity: each colour a disabled style emits has its alpha
/// multiplied by the opacity instead. A non-finite opacity fades nothing.
pub(crate) fn faded(color: Color, opacity: f32) -> Color {
    let opacity = if opacity.is_finite() {
        opacity.clamp(0.0, 1.0)
    } else {
        1.0
    };
    Color {
        a: color.a * opacity,
        ..color
    }
}

/// One colour of a disabled control faded as one widget over `ground`, the
/// opaque colour under the control: `color` composited over `under`, the part
/// it is painted on (`ground` itself, or a fill already composited over it),
/// then [`faded`] by `opacity` and composited over `ground`.
///
/// libadwaita fades a disabled switch or check box as a whole,
/// `filter: Opacity(var(--disabled-opacity))` (`docs/platform-facts.md`
/// §2.1.6), so a thumb or a check mark keeps its colour on its fill and the
/// pair fades over the page; fading each colour on its own, as [`faded`]
/// does, would show the thumb through the faded track. Where the opacity is
/// 1, or not a number, the platform dims by colour alone and `color` is as
/// given.
pub(crate) fn faded_as_one(ground: Color, under: Color, color: Color, opacity: f32) -> Color {
    if !(opacity.is_finite() && opacity < 1.0) {
        return color;
    }
    composite_over(faded(composite_over(color, under), opacity), ground)
}

/// A border as a disabled widget shows it: its colour [`faded`].
fn faded_border(border: Border, opacity: f32) -> Border {
    Border {
        color: faded(border.color, opacity),
        ..border
    }
}

/// `border` as iced outlines a widget: the colour, the line width and the
/// corner radius the theme states.
pub(crate) fn border_of(border: &ResolvedWidgetBorder) -> Border {
    Border {
        color: to_color(border.color),
        width: border.line_width,
        radius: Radius::new(border.corner_radius),
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
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
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
    let opacity = b.disabled_opacity;

    let border = border_of(&b.border);

    move |_theme, status| {
        let iced = Style::default();
        let (background, text_color, border) = match status {
            Status::Active => (idle, label, border),
            Status::Hovered => (hovered, hovered_label, border),
            Status::Pressed => (pressed, pressed_label, border),
            Status::Disabled => (
                faded(disabled, opacity),
                faded(disabled_label, opacity),
                faded_border(border, opacity),
            ),
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

/// The platform's own toggle button, for `button(..).style(..)`: [`button`]
/// while it is off, and while `checked` a toggle button that is on.
///
/// A checked toggle is `button.checked_background` with `.checked_text_color`
/// (`docs/platform-facts.md` §2.3: Breeze, WinUI, libadwaita and Material
/// state the pair, AppKit none). Both are soft options, and `None` is the
/// platform showing a checked toggle in the pressed pair
/// (`docs/inheritance-rules.toml`): [`button`]'s pressed fill,
/// `button.active_background` (the hover layer where that is unstated too)
/// over the idle fill, and `.active_text_color` -- the fallback the gpui
/// connector's `geometry::button_checked` takes. A stated checked fill
/// replaces the idle one and is emitted as given, as gpui paints it.
///
/// The model states no hovered and no pressed checked toggle, so under the
/// pointer and while pressed it is the checked one, as gpui-component paints
/// a selected button, with no hover of its own (gpui-component 0.8.1
/// `src/button/button.rs:730`, `!self.disabled && !selected`). Disabled, it
/// is [`button`]'s disabled pair where the platform states a disabled fill;
/// where it states none it dims by opacity alone (`docs/platform-facts.md`
/// §2.1.6), and the toggle keeps its checked pair. Either has each colour's
/// alpha multiplied by `button.disabled_opacity`, as iced has no widget
/// opacity. The border, `shadow` and `snap` are [`button`]'s.
#[must_use = "this returns the style function; it does not apply it"]
pub fn button_toggle(
    resolved: &ResolvedTheme,
    checked: bool,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
    use iced_widget::button::{Status, Style};

    let plain = button(resolved);
    let b = &resolved.button;

    let pressed = composite_over(
        to_color(b.active_background.unwrap_or(b.hover_background)),
        to_color(b.background_color),
    );
    let fill = b.checked_background.map_or(pressed, to_color);
    let label = to_color(b.checked_text_color.unwrap_or(b.active_text_color));
    let (disabled, disabled_label) = match b.disabled_background {
        Some(background) => (to_color(background), to_color(b.disabled_text_color)),
        None => (fill, label),
    };
    let opacity = b.disabled_opacity;

    move |theme, status| {
        let style = plain(theme, status);
        if !checked {
            return style;
        }
        let (background, text_color) = match status {
            Status::Active | Status::Hovered | Status::Pressed => (fill, label),
            Status::Disabled => (faded(disabled, opacity), faded(disabled_label, opacity)),
        };
        Style {
            background: Some(Background::Color(background)),
            text_color,
            ..style
        }
    }
}

/// A button class the platform names but does not describe in every state:
/// `fill` and `label` are the idle pair it states, `class` is the iced class
/// this replaces.
///
/// The model gives the accent and the three status colors a fill and a label
/// and nothing else -- no hovered and no pressed variant of either -- and the
/// neutral `button.hover_background` is not theirs to borrow: it is the neutral
/// button's hover colour, an opaque grey or tinted neutral on every one of the
/// seventeen presets, which would paint a hovered accent button in the neutral
/// button's hover colour. So
/// those two states follow the no-source
/// rule and come from `class` at run time, which derives them from the same
/// color through the palette.
///
/// The border is the button's, not iced's `border::rounded(2)`
/// (`button.rs:739`), so such a button sits beside a native one instead of
/// beside a differently rounded one. Disabled is the button's disabled pair
/// where the platform states a disabled fill, which it gives every button
/// class; where it states none it dims by opacity alone
/// (`docs/platform-facts.md` §2.1.6), and the class keeps its idle pair. Either
/// is faded by `button.disabled_opacity` (see [`faded`]).
fn class_button(
    resolved: &ResolvedTheme,
    fill: Rgba,
    label: Rgba,
    class: fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
    use iced_widget::button::{Status, Style};

    let b = &resolved.button;

    let idle = to_color(fill);
    let idle_label = to_color(label);
    let (disabled, disabled_label) = match b.disabled_background {
        Some(background) => (to_color(background), to_color(b.disabled_text_color)),
        None => (idle, idle_label),
    };
    let opacity = b.disabled_opacity;

    let border = border_of(&b.border);

    move |theme, status| {
        let iced = class(theme, status);
        let (background, text_color, border) = match status {
            Status::Active => (Some(Background::Color(idle)), idle_label, border),
            // Neither side of these two states has a native source, so both
            // are iced's own answer for this class.
            Status::Hovered | Status::Pressed => (iced.background, iced.text_color, border),
            Status::Disabled => (
                Some(Background::Color(faded(disabled, opacity))),
                faded(disabled_label, opacity),
                faded_border(border, opacity),
            ),
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
/// The idle pair is the platform's primary pair, `button.primary_background`
/// and `.primary_text_color`; see `class_button` for the disabled state and
/// the border. The model states no hovered and no pressed primary fill, so
/// those two are iced's own derivation -- but from the primary fill, not from
/// the palette slot `button::primary` reads. That slot is
/// `defaults.accent_color`, and a platform may state a primary fill that is
/// not its accent: WinUI 3 paints `AccentFillColorDefault`
/// (`docs/platform-facts.md` §2.3). So the pair is generated here as iced
/// generates the palette's (`Primary::generate`, `palette.rs:504-516`, called
/// by `Extended::generate` at `:394-421`), and painted as `button::primary`
/// paints it (`button.rs:598-610`): the pair's `strong` fill while hovered,
/// its `base` fill while pressed, and its label in both -- the stated
/// `.primary_text_color`, as the connector states `primary.base.text`.
#[must_use = "this returns the style function; it does not apply it"]
pub fn button_primary(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
    use iced_widget::button::{Status, Style};

    let b = &resolved.button;
    let class = class_button(
        resolved,
        b.primary_background,
        b.primary_text_color,
        iced_widget::button::primary,
    );

    let palette = crate::palette::to_palette(resolved);
    let primary = iced_core::theme::palette::Primary::generate(
        to_color(b.primary_background),
        palette.background,
        palette.text,
    );
    let hovered = primary.strong.color;
    let pressed = primary.base.color;
    let label = to_color(b.primary_text_color);

    move |theme, status| {
        let style = class(theme, status);
        let background = match status {
            Status::Hovered => hovered,
            Status::Pressed => pressed,
            Status::Active | Status::Disabled => return style,
        };
        Style {
            background: Some(Background::Color(background)),
            text_color: label,
            ..style
        }
    }
}

/// The platform's own destructive button, for `button(..).style(..)`.
///
/// Replaces `iced_widget::button::danger`, which paints the palette's `danger`
/// family on a hardcoded `border::rounded(2)`. The idle fill and label are
/// `defaults.danger_color` and `.danger_text_color`; see `class_button` for
/// what the other states are and why.
#[must_use = "this returns the style function; it does not apply it"]
pub fn button_danger(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
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
/// `defaults.success_color` and `.success_text_color`; see `class_button`
/// for what the other states are and why.
#[must_use = "this returns the style function; it does not apply it"]
pub fn button_success(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
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
/// `defaults.warning_color` and `.warning_text_color`; see `class_button`
/// for what the other states are and why.
#[must_use = "this returns the style function; it does not apply it"]
pub fn button_warning(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
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
/// layered over that fill -- and every preset states the fill as fully
/// transparent, which is the case that layering was made general for.
///
/// `LinkTheme` states no pressed and no disabled fill, so both keep the idle
/// one: a state the platform does not state has no distinct appearance. It
/// carries no border either, and
/// no shadow geometry, so `border`, `shadow` and `snap` are iced's own for a
/// text button.
#[must_use = "this returns the style function; it does not apply it"]
pub fn button_link(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
    use iced_widget::button::{Status, Style};

    let l = &resolved.link;

    let idle = to_color(l.background_color);
    let hovered = composite_over(to_color(l.hover_background), idle);

    let label = to_color(l.font.color);
    let hovered_label = to_color(l.hover_text_color);
    let pressed_label = to_color(l.active_text_color);
    // `LinkTheme` states no `disabled_opacity`: the defaults' fades it.
    let disabled_label = faded(
        to_color(l.disabled_text_color),
        resolved.defaults.disabled_opacity,
    );

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

/// The `input.*` values [`text_input`] and [`text_editor`] both emit, as the
/// model states one `InputTheme` for both: the fill, the border and the value
/// of a field at rest, hovered, focused and disabled, and the two colours
/// every state shares. The border at rest is the one `border` names: the
/// single-line field's `input.border`, or the multi-line field's own
/// `text_area.border`; its hovered and focused colours are `input.*`'s.
#[derive(Clone, Copy)]
struct InputColors {
    active: (Color, Border, Color),
    hovered: (Color, Border, Color),
    focused: (Color, Border, Color),
    disabled: (Color, Border, Color),
    placeholder: Color,
    selection: Color,
}

impl InputColors {
    fn new(i: &ResolvedInputTheme, border: &ResolvedWidgetBorder, d: &ResolvedDefaults) -> Self {
        let idle = to_color(i.background_color);
        // A translucent disabled fill replaces the idle one and lets the
        // window through, so it is emitted as given.
        let disabled = to_color(i.disabled_background.unwrap_or(i.background_color));
        let text = to_color(i.font.color);
        let opacity = i.disabled_opacity;

        // Both border soft options fall back to the border's own color: the
        // platform saying the field does not change outline in that state.
        let border = border_of(border);
        let outlined = |color: Option<Rgba>| Border {
            color: color.map_or(border.color, to_color),
            ..border
        };
        // An inset focus ring (offset -width: libadwaita's `outline-offset:
        // -$width`, `docs/platform-facts.md` §2.1.5) lies inside the field's
        // edge, where iced paints a border (`renderer::Quad { bounds, border,
        // .. }`, iced_widget 0.14.2 `src/text_input.rs:473-478`), so it is
        // drawn exactly as a border that wide. iced's quad puts the border in
        // place of the fill under it rather than over it (`mix(color,
        // border_color, ..)`, iced_wgpu 0.14.0 `src/shader/quad/solid.wgsl:78-83`),
        // while the platform lays the ring over the field's fill, so a
        // translucent ring is composited over the fill here. A ring that lies
        // outside the edge (offset 0 or more: KDE's +2, Windows' +1),
        // straddles it (macOS's -1 with width 3) or lies inside it with a gap
        // (an offset below minus the width) is not a border iced can draw;
        // there a focused field recolours its rest line, and
        // `crate::focus_ring` draws the ring round the field it wraps.
        let focus_color = i.focus_border_color.unwrap_or(d.focus_ring_color);
        let focused = if d.focus_ring_width > 0.0 && d.focus_ring_offset == -d.focus_ring_width {
            Border {
                color: composite_over(to_color(focus_color), idle),
                width: border.width.max(d.focus_ring_width),
                ..border
            }
        } else {
            outlined(i.focus_border_color)
        };

        Self {
            active: (idle, border, text),
            hovered: (idle, outlined(i.hover_border_color), text),
            focused: (idle, focused, text),
            disabled: (
                faded(disabled, opacity),
                faded_border(border, opacity),
                faded(to_color(i.disabled_text_color), opacity),
            ),
            placeholder: to_color(i.placeholder_color),
            selection: to_color(i.selection_background),
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
/// colors, the placeholder, the value and the selection. Where the theme's
/// focus ring is inset at the field's edge (`defaults.focus_ring_offset` is
/// minus `defaults.focus_ring_width`, libadwaita's), a focused field draws it
/// as its border, `defaults.focus_ring_width` wide in
/// `input.focus_border_color` (or the ring's colour), so a field with no rest
/// line still shows its focus. A ring outside or across the edge, or inset
/// with a gap, is no border: the focused field's line at rest then takes
/// `input.focus_border_color` where the theme states one, and
/// [`crate::focus_ring()`] draws the ring round the field it wraps. The disabled state is
/// the platform's disabled fill and disabled text color, both of which the
/// model states, each colour's alpha multiplied by `input.disabled_opacity`
/// (iced has no widget opacity; `docs/platform-facts.md` §2.1.6). `icon` has
/// no native source -- the model carries no
/// input-icon color -- and comes from
/// `text_input::default(theme, status)`.
#[must_use = "this returns the style function; it does not apply it"]
pub fn text_input(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::text_input::Status) -> iced_widget::text_input::Style + Clone + use<>
{
    use iced_widget::text_input::{Status, Style};

    let input = InputColors::new(&resolved.input, &resolved.input.border, &resolved.defaults);

    move |theme, status| {
        let iced = iced_widget::text_input::default(theme, status);
        let (background, border, value) = match status {
            Status::Active => input.active,
            Status::Hovered => input.hovered,
            // A field that is focused *and* hovered takes the focus border,
            // as iced's own default does: its `Focused` arm ignores
            // `is_hovered` (`text_input.rs:1783-1789`).
            Status::Focused { is_hovered: _ } => input.focused,
            Status::Disabled => input.disabled,
        };
        Style {
            background: Background::Color(background),
            border,
            icon: iced.icon,
            placeholder: input.placeholder,
            value,
            selection: input.selection,
        }
    }
}

/// The platform's own multi-line text field, for `text_editor(..).style(..)`.
///
/// Replaces `iced_widget::text_editor::default` (`text_editor.rs:1466`), which
/// derives the same values from the palette that `text_input::default` does.
///
/// The model states one `InputTheme` for both, so this reads the sources
/// [`text_input`] reads, but for the border: its colour, width and radius
/// are `text_area.border`'s, which the model gives the multi-line field
/// apart from the single-line one (inheriting `input.border` where the theme
/// states none), as [`crate::text_area_padding`] reads that border's padding.
/// The hovered and focused border colours are `input.*`'s, which the text
/// area states none of its own for. `text_editor::Style` has no `icon` field
/// (`text_editor.rs:1425-1436`), so every field it does have is a native one
/// and nothing here comes from iced.
#[must_use = "this returns the style function; it does not apply it"]
pub fn text_editor(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::text_editor::Status) -> iced_widget::text_editor::Style + Clone + use<>
{
    use iced_widget::text_editor::{Status, Style};

    let input = InputColors::new(
        &resolved.input,
        &resolved.text_area.border,
        &resolved.defaults,
    );

    move |_theme, status| {
        let (background, border, value) = match status {
            Status::Active => input.active,
            Status::Hovered => input.hovered,
            // Focused and hovered takes the focus border here too, as iced's
            // own default does (`text_editor.rs:1490`).
            Status::Focused { is_hovered: _ } => input.focused,
            Status::Disabled => input.disabled,
        };
        Style {
            background: Background::Color(background),
            border,
            placeholder: input.placeholder,
            value,
            selection: input.selection,
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
/// `.unchecked_background`, and the `Status` says which one it is showing.
/// `.hover_background` is the hover of the *unchecked* box, and is composited
/// as a layer over its fill where the theme states one; unset, a hovered
/// unchecked box is the box at rest. The model states no hover for a checked
/// one, and a state the platform does not state has no distinct appearance,
/// so a hovered checked box is exactly the checked box. `.disabled_background`
/// replaces the fill, as given; where it is not stated the platform dims by
/// opacity alone and a disabled box is its enabled self. Either way a disabled
/// box is faded by `.disabled_opacity` as one widget over the window,
/// `defaults.background_color` ([`checkbox_over`] takes another backdrop):
/// its fill over the window, its border and mark over the fill, its label
/// over the window, each then faded over the window
/// (`docs/platform-facts.md` §2.1.6). An unchecked box may state a border
/// color of its own, `.unchecked_border_color`.
///
/// The check mark is `checkbox.indicator_color` -- the model has no
/// `check_color` -- in every status but `Disabled`. `indicator_color` is the
/// colour the platform states for a foreground **on the accent**
/// (`platform-facts.md` §2.5: macOS white, Fluent
/// `TextOnAccentFillColorPrimary`, KDE `[Colors:Selection] ForegroundNormal`,
/// Adwaita white), and the mark sits on `checked_background`, which is the
/// accent. A disabled box does not show the accent: `.disabled_background`
/// replaces it for the checked and the unchecked state alike. So there the
/// mark takes `checkbox.disabled_text_color`, the one foreground the platform
/// states for everything it dims -- the same field the disabled label takes,
/// and a value the model states rather than one derived here.
///
/// Nothing here comes from iced: every field of `checkbox::Style` has a native
/// source. The label is `checkbox.font.color`, and `.disabled_text_color` when
/// the box is disabled -- the platform dims it, where iced's own class leaves
/// the label to be inherited whole.
///
/// Two native fields are the consumer's builder geometry rather than `Style`
/// fields: `checkbox.indicator_width` is the indicator's side length
/// (`platform-facts.md:984`) and belongs to `Checkbox::size(..)`
/// (`checkbox.rs:176`), which lays the box out as a square of exactly that
/// (`:287`); `checkbox.label_gap` belongs to `Checkbox::spacing(..)`
/// (`checkbox.rs:188`).
#[must_use = "this returns the style function; it does not apply it"]
pub fn checkbox(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::checkbox::Status) -> iced_widget::checkbox::Style + Clone + use<>
{
    checkbox_over(resolved, to_color(resolved.defaults.background_color))
}

/// The fills and the border colours `checkbox.*` states for a box, or a
/// radio, that is not disabled: the model states one `CheckboxTheme` for both
/// controls.
#[derive(Clone, Copy)]
struct CheckFills {
    checked: Color,
    unchecked: Color,
    hovered_unchecked: Color,
    checked_border: Color,
    unchecked_border: Color,
}

impl CheckFills {
    fn new(c: &ResolvedCheckboxTheme) -> Self {
        // The three soft options are the platform saying the box looks no
        // different: unchecked, it has the checkbox's own fill and border;
        // under the pointer, the fill it has at rest.
        let unchecked = to_color(c.unchecked_background.unwrap_or(c.background_color));
        Self {
            checked: to_color(c.checked_background),
            unchecked,
            hovered_unchecked: c.hover_background.map_or(unchecked, |hover| {
                composite_over(to_color(hover), unchecked)
            }),
            checked_border: to_color(c.border.color),
            unchecked_border: to_color(c.unchecked_border_color.unwrap_or(c.border.color)),
        }
    }

    /// The fill and the border colour of a box that is checked or not, under
    /// the pointer or not. The model states no hover for a checked box.
    fn at(&self, is_checked: bool, is_hovered: bool) -> (Color, Color) {
        match (is_checked, is_hovered) {
            (true, _) => (self.checked, self.checked_border),
            (false, false) => (self.unchecked, self.unchecked_border),
            (false, true) => (self.hovered_unchecked, self.unchecked_border),
        }
    }
}

/// [`checkbox`] for a box drawn on `backdrop` rather than on the window: a
/// disabled box is faded as one widget over `backdrop`.
#[must_use = "this returns the style function; it does not apply it"]
pub fn checkbox_over(
    resolved: &ResolvedTheme,
    backdrop: Color,
) -> impl Fn(&Theme, iced_widget::checkbox::Status) -> iced_widget::checkbox::Style + Clone + use<>
{
    use iced_widget::checkbox::{Status, Style};

    let c = &resolved.checkbox;
    let ground = backdrop;

    let fills = CheckFills::new(c);
    // A translucent disabled fill replaces the idle one, so it is as given. With
    // none stated the platform dims by opacity alone
    // (`docs/platform-facts.md` §2.1.6): the box keeps its enabled look.
    let disabled = c.disabled_background.map(to_color);
    let opacity = c.disabled_opacity;

    let mark = to_color(c.indicator_color);
    let label = to_color(c.font.color);
    let disabled_label = to_color(c.disabled_text_color);

    let border_width = c.border.line_width;
    let border_radius = Radius::new(c.border.corner_radius);

    move |_theme, status| {
        let (background, border_color, text_color, icon_color) = match status {
            Status::Active { is_checked } => {
                let (fill, border) = fills.at(is_checked, false);
                (fill, border, label, mark)
            }
            Status::Hovered { is_checked } => {
                let (fill, border) = fills.at(is_checked, true);
                (fill, border, label, mark)
            }
            // The platform states one disabled fill for both, and the box
            // keeps the outline that says whether it is checked. The mark
            // leaves the accent with the fill: `indicator_color` is the
            // on-accent color, and a disabled box no longer shows the accent,
            // so the mark takes the one foreground the platform states for
            // everything it dims -- the same field the disabled label takes.
            // With no disabled fill stated the box is its enabled self. The
            // box is then faded by `checkbox.disabled_opacity` as one widget
            // over the ground: the border and the mark on the fill.
            Status::Disabled { is_checked } => {
                let (at_rest, border) = fills.at(is_checked, false);
                let (background, mark) = match disabled {
                    Some(fill) => (fill, disabled_label),
                    None => (at_rest, mark),
                };
                let fill = composite_over(background, ground);
                (
                    faded_as_one(ground, ground, background, opacity),
                    faded_as_one(ground, fill, border, opacity),
                    faded_as_one(ground, ground, disabled_label, opacity),
                    faded_as_one(ground, fill, mark, opacity),
                )
            }
        };
        Style {
            background: Background::Color(background),
            icon_color,
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
/// (`widgets/mod.rs:176-183`), so this reads exactly the sources [`checkbox`]
/// reads, with `is_selected` where the checkbox has `is_checked` and the check
/// mark serving as the dot -- `checkbox.hover_background` included, which is
/// the hover of the *unselected* circle only, so a hovered selected radio is
/// exactly the selected radio, and where the theme leaves it unset a hovered
/// unselected radio is the circle at rest. `radio::Status` has no disabled
/// variant (`radio.rs:474-487`), so the model's disabled fields have no
/// receiver here.
///
/// `radio::Style` carries its border as a width and a color rather than as an
/// iced `Border`, so there is no corner radius to give it.
///
/// The same two native fields are builder geometry here as on a checkbox:
/// `checkbox.indicator_width` is the radio indicator's diameter
/// (`platform-facts.md:984`), or `checkbox.radio_indicator_width` where the
/// theme sizes the radio apart (`platform-facts.md:1243`) -- the whole circle,
/// not the selected dot, which
/// iced draws at half of it (`radio.rs:409`) -- and belongs to
/// `Radio::size(..)` (`radio.rs:200`, laid out at `:300`), and
/// `checkbox.label_gap` to `Radio::spacing(..)` (`radio.rs:212`).
/// `checkbox.radio_dot_diameter` has no receiver in `radio::Style`;
/// [`crate::radio()`] applies this style with both builders and draws that
/// dot.
#[must_use = "this returns the style function; it does not apply it"]
pub fn radio(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::radio::Status) -> iced_widget::radio::Style + Clone + use<> {
    use iced_widget::radio::{Status, Style};

    let c = &resolved.checkbox;

    let fills = CheckFills::new(c);
    let dot = to_color(c.indicator_color);
    let label = to_color(c.font.color);
    let border_width = c.border.line_width;

    move |_theme, status| {
        let (background, border_color) = match status {
            Status::Active { is_selected } => fills.at(is_selected, false),
            Status::Hovered { is_selected } => fills.at(is_selected, true),
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
/// `is_toggled`, with `.hover_*` layered over it where the theme states one
/// and `.disabled_*` replacing it, as given. The thumb is
/// `switch.thumb_background`, a thumb and so emitted as given in every state
/// -- `.unchecked_thumb_background`, where the theme states one, while not
/// toggled -- and `.disabled_thumb_color` whenever the switch is disabled,
/// toggled or not. A disabled switch is faded by `switch.disabled_opacity` as
/// one widget over the window, `defaults.background_color` ([`toggler_over`]
/// takes another backdrop): the thumb over the track, the track over the
/// window, each then faded over the window (`docs/platform-facts.md` §2.1.6,
/// libadwaita's `filter: Opacity(..)`).
///
/// `border_radius` is `switch.track_radius`, and it shapes the whole widget:
/// iced paints the track and the thumb as two quads with the *same* radius
/// (`toggler.rs:435`, `:461`), and its own `None` would make both perfectly
/// round (`toggler.rs:427-429`) whatever the platform states.
///
/// `padding_ratio` is the inset `switch.track_height` and `.thumb_diameter`
/// state between them, in the unit iced's receiver takes: it multiplies the
/// ratio by the track's height to get the padding on each side
/// (`toggler.rs:444`) and gives the thumb what is left
/// (`toggler.rs:453-454`), so the ratio is
/// `(track_height - thumb_diameter) / (2 * track_height)`. A preset that
/// states no track height, or a thumb taller than its track, would ask for a
/// division by zero or a negative inset; there the ratio is iced's own instead.
/// kde-breeze's thumb is taller than its track (docs/platform-facts.md §2.21),
/// which iced's toggler cannot draw: [`crate::switch()`] draws it.
/// While not toggled the thumb is `.unchecked_thumb_diameter` across where the
/// theme states one (Material's 16 against its 24).
///
/// One native field is the consumer's builder geometry rather than a `Style`
/// field: `switch.track_height` belongs to `Toggler::size(..)`
/// (`toggler.rs:171`), which is the track's *height* -- iced lays the track
/// out as `2 × size` by `size` (`:287`), so the platform's track width follows
/// from it and has no separate receiver.
///
/// The track's border is `switch.checked_track_border_color` or
/// `.unchecked_track_border_color`, `.disabled_*` disabled where stated,
/// `.track_border_line_width` wide, and the thumb's `.thumb_border_color`,
/// `.thumb_border_line_width` wide, each faded as one with a disabled switch;
/// a border whose colour or width the theme leaves unset is iced's own, none
/// (`docs/platform-facts.md` §2.21: libadwaita and macOS outline neither,
/// WinUI and Material only the off track). iced draws both inside their
/// quads, in place of the fill under them, so the track's outline is emitted
/// composited over the track's fill in each state, as the platforms stroke
/// it. `text_color` has no native source and comes from
/// `toggler::default(theme, status)`: the model states no font for a switch,
/// and iced's `None` inherits the surrounding one.
#[must_use = "this returns the style function; it does not apply it"]
pub fn toggler(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::toggler::Status) -> iced_widget::toggler::Style + Clone + use<> {
    toggler_over(resolved, to_color(resolved.defaults.background_color))
}

/// [`toggler`] for a switch drawn on `backdrop` rather than on the window: a
/// disabled switch is faded as one widget over `backdrop`.
#[must_use = "this returns the style function; it does not apply it"]
pub fn toggler_over(
    resolved: &ResolvedTheme,
    backdrop: Color,
) -> impl Fn(&Theme, iced_widget::toggler::Status) -> iced_widget::toggler::Style + Clone + use<> {
    use iced_widget::toggler::{Status, Style};

    let s = &resolved.switch;
    let ground = backdrop;
    let opacity = s.disabled_opacity;

    let checked = to_color(s.checked_background);
    let unchecked = to_color(s.unchecked_background);
    // Each hover layer is a soft option: where it is unset the hovered track
    // is the track at rest.
    let hovered_checked = s
        .hover_checked_background
        .map_or(checked, |hover| composite_over(to_color(hover), checked));
    let hovered_unchecked = s.hover_unchecked_background.map_or(unchecked, |hover| {
        composite_over(to_color(hover), unchecked)
    });
    let disabled_checked = to_color(
        s.disabled_checked_background
            .unwrap_or(s.checked_background),
    );
    let disabled_unchecked = to_color(
        s.disabled_unchecked_background
            .unwrap_or(s.unchecked_background),
    );

    let thumb = to_color(s.thumb_background);
    // The off thumb: a soft option copying the on one.
    let unchecked_thumb = s.unchecked_thumb_background.unwrap_or(s.thumb_background);
    let disabled_thumb = to_color(s.disabled_thumb_color.unwrap_or(s.thumb_background));
    let disabled_unchecked_thumb = to_color(s.disabled_thumb_color.unwrap_or(unchecked_thumb));
    let unchecked_thumb = to_color(unchecked_thumb);

    // A disabled switch is faded by `switch.disabled_opacity` as one widget
    // over the ground: the thumb on its track (see `faded_as_one`).
    let disabled_as_one = |track: Color, thumb: Color| {
        (
            faded_as_one(ground, ground, track, opacity),
            faded_as_one(ground, composite_over(track, ground), thumb, opacity),
        )
    };
    let disabled_on = disabled_as_one(disabled_checked, disabled_thumb);
    let disabled_off = disabled_as_one(disabled_unchecked, disabled_unchecked_thumb);

    // The outlines, where the theme states a colour and a width: the track's
    // per state over that state's fill, a disabled one faded as one with it
    // over the ground, and the thumb's over the track (docs/platform-facts.md
    // §2.21). The platforms stroke the track's outline over its fill (WinUI's
    // `ToggleSwitchStrokeOff` a shape stroke, Material's `track-outline` a
    // border over a `border-box` background), while iced's quad paints its
    // border in place of the fill under it, as for the inset focus ring of
    // `InputColors::new`, so a translucent outline is composited over the
    // track here.
    let outline = |colour: Option<native_theme::color::Rgba>, width: Option<f32>| {
        colour
            .zip(width)
            .map(|(colour, width)| (width, to_color(colour)))
    };
    let over_track = |edge: Option<(f32, Color)>, track: Color| {
        edge.map(|(width, edge)| (width, composite_over(edge, track)))
    };
    let rest_edge_on = outline(s.checked_track_border_color, s.track_border_line_width);
    let rest_edge_off = outline(s.unchecked_track_border_color, s.track_border_line_width);
    let edge_on = over_track(rest_edge_on, checked);
    let edge_off = over_track(rest_edge_off, unchecked);
    let hovered_edge_on = over_track(rest_edge_on, hovered_checked);
    let hovered_edge_off = over_track(rest_edge_off, hovered_unchecked);
    let disabled_edge =
        |colour: Option<native_theme::color::Rgba>, rest: Option<(f32, Color)>, track: Color| {
            over_track(outline(colour, s.track_border_line_width).or(rest), track)
                .map(|(width, edge)| (width, faded_as_one(ground, ground, edge, opacity)))
        };
    let disabled_edge_on = disabled_edge(
        s.disabled_checked_track_border_color,
        rest_edge_on,
        disabled_checked,
    );
    let disabled_edge_off = disabled_edge(
        s.disabled_unchecked_track_border_color,
        rest_edge_off,
        disabled_unchecked,
    );
    let thumb_edge = outline(s.thumb_border_color, s.thumb_border_line_width);
    let disabled_thumb_edge = |track: Color| {
        thumb_edge.map(|(width, edge)| {
            (
                width,
                faded_as_one(ground, composite_over(track, ground), edge, opacity),
            )
        })
    };
    let disabled_thumb_edge_on = disabled_thumb_edge(disabled_checked);
    let disabled_thumb_edge_off = disabled_thumb_edge(disabled_unchecked);

    let track_radius = Radius::new(s.track_radius);
    // Guarded so that a track with no height, or a thumb taller than its
    // track, falls back to iced's own ratio rather than dividing by zero or
    // insetting the thumb by a negative length. The off thumb is
    // `unchecked_thumb_diameter` across where the theme states one.
    let ratio = |thumb: f32| {
        if s.track_height > 0.0 && thumb >= 0.0 && thumb <= s.track_height {
            Some((s.track_height - thumb) / (2.0 * s.track_height))
        } else {
            None
        }
    };
    let padding_ratio = ratio(s.thumb_diameter);
    let unchecked_padding_ratio = ratio(s.unchecked_thumb_diameter.unwrap_or(s.thumb_diameter));

    move |theme, status| {
        let iced = iced_widget::toggler::default(theme, status);
        let (background, foreground) = match status {
            Status::Active { is_toggled } => {
                if is_toggled {
                    (checked, thumb)
                } else {
                    (unchecked, unchecked_thumb)
                }
            }
            Status::Hovered { is_toggled } => {
                if is_toggled {
                    (hovered_checked, thumb)
                } else {
                    (hovered_unchecked, unchecked_thumb)
                }
            }
            Status::Disabled { is_toggled } => {
                if is_toggled {
                    disabled_on
                } else {
                    disabled_off
                }
            }
        };
        let is_toggled = match status {
            Status::Active { is_toggled }
            | Status::Hovered { is_toggled }
            | Status::Disabled { is_toggled } => is_toggled,
        };
        let padding_ratio = if is_toggled {
            padding_ratio
        } else {
            unchecked_padding_ratio
        };
        let (track_edge, thumb_edge) = match status {
            Status::Disabled { is_toggled: true } => (disabled_edge_on, disabled_thumb_edge_on),
            Status::Disabled { is_toggled: false } => (disabled_edge_off, disabled_thumb_edge_off),
            Status::Hovered { is_toggled: true } => (hovered_edge_on, thumb_edge),
            Status::Hovered { is_toggled: false } => (hovered_edge_off, thumb_edge),
            Status::Active { is_toggled: true } => (edge_on, thumb_edge),
            Status::Active { is_toggled: false } => (edge_off, thumb_edge),
        };
        let (background_border_width, background_border_color) =
            track_edge.unwrap_or((iced.background_border_width, iced.background_border_color));
        let (foreground_border_width, foreground_border_color) =
            thumb_edge.unwrap_or((iced.foreground_border_width, iced.foreground_border_color));
        Style {
            background: Background::Color(background),
            background_border_width,
            background_border_color,
            foreground: Background::Color(foreground),
            foreground_border_width,
            foreground_border_color,
            text_color: iced.text_color,
            border_radius: Some(track_radius),
            padding_ratio: padding_ratio.unwrap_or(iced.padding_ratio),
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
/// over it -- a soft option, whose `None` is the platform saying a hovered
/// field looks no different -- its label is `combo_box.font.color` and its
/// border is `combo_box.border.*`. The placeholder is
/// `input.placeholder_color`: the model states it once, for every field that
/// has one.
///
/// `pick_list::Status::Opened` carries whether the pointer is on the field
/// (`pick_list.rs:838-849`), and that is what decides the fill here -- the
/// model states no separate open appearance. `handle_color` has no native
/// source, `ComboBoxTheme` carrying the arrow's sizes but not its color, so it
/// comes from `pick_list::default(theme, status)`.
///
/// One native field is the consumer's builder geometry rather than a `Style`
/// field: `combo_box.arrow_icon_size` belongs to
/// `PickList::handle(..)` (`pick_list.rs:269`), which iced uses as the font
/// size of the arrow glyph (`:600-634`); [`crate::pick_list_handle()`] gives
/// the handle, the open chevron the platforms draw at that size.
#[must_use = "this returns the style function; it does not apply it"]
pub fn pick_list(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::pick_list::Status) -> iced_widget::pick_list::Style + Clone + use<>
{
    use iced_widget::pick_list::{Status, Style};

    let c = &resolved.combo_box;

    let idle = to_color(c.background_color);
    let hovered = c
        .hover_background
        .map_or(idle, |hover| composite_over(to_color(hover), idle));

    let label = to_color(c.font.color);
    let placeholder = to_color(resolved.input.placeholder_color);

    let border = border_of(&c.border);

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
/// composited: the platform measured that fill as it shows, not as a layer.
///
/// `shadow` has no native source -- the model has `defaults.shadow_color` but
/// no offset or blur -- and comes from `overlay::menu::default(theme)`.
#[must_use = "this returns the style function; it does not apply it"]
pub fn menu(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme) -> iced_widget::overlay::menu::Style + Clone + use<> {
    use iced_widget::overlay::menu::Style;

    let m = &resolved.menu;

    let background = to_color(m.background_color);
    let label = to_color(m.font.color);
    let selected_label = to_color(m.hover_text_color);
    let selected_background = to_color(m.hover_background);

    let border = border_of(&m.border);

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
/// with `.thumb_hover_color` under the pointer composited over it
/// ([`composite_over`]): the platform states the hover as a layer over the
/// thumb, as the gpui connector's slider lays it, while iced replaces the
/// handle's fill. Its size is `slider.thumb_diameter`, halved because iced
/// states a circular handle by its radius.
///
/// Two things have no native source. The model states no dragged thumb color,
/// so in that one status the handle's **fill** -- and nothing else about it --
/// is `slider::default(theme, Status::Dragged).handle.background`, as a status
/// button's hovered fill is iced's; its shape stays the
/// platform's thumb diameter, and so does the rail under it. That one field is
/// asserted against iced's own by
/// `a_dragged_slider_handle_takes_its_fill_from_iced`. And `SliderTheme`
/// carries no border, so the rail's border and the handle's border width and
/// color are iced's too.
#[must_use = "this returns the style function; it does not apply it"]
pub fn slider(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::slider::Status) -> iced_widget::slider::Style + Clone + use<> {
    use iced_widget::slider::{Handle, HandleShape, Rail, Status, Style};

    let s = &resolved.slider;

    let filled = to_color(s.fill_color);
    let remaining = to_color(s.track_color);
    let rail_width = s.track_height;

    let thumb = to_color(s.thumb_color);
    // A soft option: `None` is the platform saying the thumb does not change
    // under the pointer, so it is the thumb's own color.
    let hovered_thumb = s
        .thumb_hover_color
        .map_or(thumb, |hover| composite_over(to_color(hover), thumb));
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

/// The platform's own scrollbars, for `scrollable(..).style(..)`.
///
/// Replaces `iced_widget::scrollable::default` (`scrollable.rs:2345`), which
/// paints both rails in the palette's weak background and takes their scrollers
/// from the strongest background, or from the primary family once one is
/// hovered or dragged.
///
/// Each rail is `scrollbar.track_color`, and each scroller is
/// `scrollbar.thumb_color`, `.thumb_hover_color` under the pointer and
/// `.thumb_active_color` while it is dragged. All three are thumbs, so they are
/// emitted as given rather than composited: iced paints them
/// over a rail it also paints.
///
/// Where the theme's scrollbar floats over the content
/// (`scrollbar.overlay_mode`), both rails and their scrollers are transparent
/// while the scrollable is `Active`, and shown as above while the pointer is
/// over it (`Hovered`) or drags a scroller: iced 0.14 draws a floating bar in
/// every status and has no auto-hide, its style asked for `Dragged`,
/// `Hovered` or `Active` alone (iced_widget 0.14.2 `src/scrollable.rs:1097-1121`,
/// `:1176-1182`), and nothing
/// tells a style that the content was scrolled, so the bar is revealed on
/// hover, not on scroll as the platforms' overlay scrollbars are.
///
/// iced states the two axes separately, and its `Status` carries which one the
/// pointer is on (`scrollable.rs:2237-2267`), so a hovered vertical scrollbar
/// leaves the horizontal one idle -- as iced's own default does. The `Status`
/// also says whether either scrollbar is *disabled*, meaning its content does
/// not overflow; the model states no appearance for that, so those two
/// booleans change nothing here.
///
/// Everything else has no native source and comes from
/// `scrollable::default(theme, status)`: `ScrollbarTheme` carries no border,
/// nothing about the scrolled container, no fill for the gap between the two
/// rails and nothing about the autoscroll overlay. The nested
/// `container::Style` has a `Default` of its own, so the compiler cannot
/// enforce naming its fields; they are named anyway, and the contract claims
/// each by name.
///
/// The two widths are not `Style` fields at all: they are built into the
/// `Scrollbar` that [`scrollbar`] returns.
#[must_use = "this returns the style function; it does not apply it"]
pub fn scrollable(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::scrollable::Status) -> iced_widget::scrollable::Style + Clone + use<>
{
    use iced_widget::scrollable::{AutoScroll, Rail, Scroller, Status, Style};

    let s = &resolved.scrollbar;

    let track = to_color(s.track_color);
    let thumb = to_color(s.thumb_color);
    let hovered_thumb = to_color(s.thumb_hover_color);
    // A soft option: `None` is the platform saying a dragged scroller looks no
    // different from a hovered one, so it copies that color.
    let dragged_thumb = to_color(s.thumb_active_color.unwrap_or(s.thumb_hover_color));
    // A floating bar is hidden at rest: iced 0.14 has no auto-hide and draws
    // a floating bar in every status (iced_widget 0.14.2
    // `src/scrollable.rs:1097-1121`, `:1176-1182`), and reveals
    // nothing on scroll, so the closest it comes to the platforms' overlay
    // scrollbar is a bar shown while the pointer is over the scrollable or
    // drags it, and transparent while it is `Active`.
    let floats = s.overlay_mode;

    move |theme, status| {
        let iced = iced_widget::scrollable::default(theme, status);
        let shown = !(floats && matches!(status, Status::Active { .. }));
        let rail = if shown { track } else { Color::TRANSPARENT };
        // Which scroller the pointer is on decides that axis alone. The
        // disabled flags are named and ignored: an upstream addition to the
        // status then fails to compile here rather than falling into a `..`.
        let (vertical, horizontal) = match status {
            Status::Active {
                is_horizontal_scrollbar_disabled: _,
                is_vertical_scrollbar_disabled: _,
            } if floats => (Color::TRANSPARENT, Color::TRANSPARENT),
            Status::Active {
                is_horizontal_scrollbar_disabled: _,
                is_vertical_scrollbar_disabled: _,
            } => (thumb, thumb),
            Status::Hovered {
                is_horizontal_scrollbar_hovered,
                is_vertical_scrollbar_hovered,
                is_horizontal_scrollbar_disabled: _,
                is_vertical_scrollbar_disabled: _,
            } => (
                if is_vertical_scrollbar_hovered {
                    hovered_thumb
                } else {
                    thumb
                },
                if is_horizontal_scrollbar_hovered {
                    hovered_thumb
                } else {
                    thumb
                },
            ),
            Status::Dragged {
                is_horizontal_scrollbar_dragged,
                is_vertical_scrollbar_dragged,
                is_horizontal_scrollbar_disabled: _,
                is_vertical_scrollbar_disabled: _,
            } => (
                if is_vertical_scrollbar_dragged {
                    dragged_thumb
                } else {
                    thumb
                },
                if is_horizontal_scrollbar_dragged {
                    dragged_thumb
                } else {
                    thumb
                },
            ),
        };
        Style {
            container: iced_widget::container::Style {
                text_color: iced.container.text_color,
                background: iced.container.background,
                border: iced.container.border,
                shadow: iced.container.shadow,
                snap: iced.container.snap,
            },
            vertical_rail: Rail {
                background: Some(Background::Color(rail)),
                border: iced.vertical_rail.border,
                scroller: Scroller {
                    background: Background::Color(vertical),
                    border: iced.vertical_rail.scroller.border,
                },
            },
            horizontal_rail: Rail {
                background: Some(Background::Color(rail)),
                border: iced.horizontal_rail.border,
                scroller: Scroller {
                    background: Background::Color(horizontal),
                    border: iced.horizontal_rail.scroller.border,
                },
            },
            gap: iced.gap,
            auto_scroll: AutoScroll {
                background: iced.auto_scroll.background,
                border: iced.auto_scroll.border,
                shadow: iced.auto_scroll.shadow,
                icon: iced.auto_scroll.icon,
            },
        }
    }
}

/// The platform's own scrollbar geometry, for
/// `scrollable(..).direction(Direction::Vertical(..))` and its horizontal and
/// two-axis siblings.
///
/// Not a closure and not a `Style`: a scrollbar's geometry is built into
/// the `Scrollbar` a `Direction` carries, so it cannot travel with
/// [`scrollable`]. Pass the result to
/// `scrollable(..).direction(Direction::Vertical(..))`, or to
/// `Direction::Horizontal(..)`, or to both fields of `Direction::Both`.
///
/// Three native fields reach it:
///
/// - `scrollbar.groove_width` is the rail's width, through `Scrollbar::width`
///   (`scrollable.rs:355`).
/// - `scrollbar.thumb_width` is the scroller's, through
///   `Scrollbar::scroller_width` (`:367`).
/// - `scrollbar.overlay_mode` decides whether the scrollbar floats over the
///   contents or sits beside them. iced's only switch is `Scrollbar::spacing`:
///   a scrollbar with one "will always be displayed, will take layout space,
///   and will not float over the contents" (`scrollable.rs:378-383`). So a
///   platform that states `overlay_mode = false` gets `.spacing(0)` -- the
///   model states no gap between the scrollbar and the contents, and zero is
///   the absence of a gap rather than a length -- and one that states `true`
///   keeps iced's floating default, which is no call at all.
///
/// The margin and the anchor stay `Scrollbar::new()`'s own: the model states
/// neither.
///
/// `scrollbar.min_thumb_length` has no receiver in iced 0.14 at all: iced
/// sizes the scroller itself, `(bounds * ratio).max(2.0)` (`scrollable.rs:2068`),
/// and exposes no minimum. It is recorded in the contract file's unreachable
/// list rather than approximated.
#[must_use = "this returns the configured scrollbar; it does not apply it"]
pub fn scrollbar(resolved: &ResolvedTheme) -> iced_widget::scrollable::Scrollbar {
    let s = &resolved.scrollbar;

    let bar = iced_widget::scrollable::Scrollbar::new()
        .width(s.groove_width)
        .scroller_width(s.thumb_width);

    if s.overlay_mode {
        bar
    } else {
        bar.spacing(0.0)
    }
}

/// The platform's own progress bar, for `progress_bar(..).style(..)`.
///
/// Replaces `iced_widget::progress_bar::primary` (`progress_bar.rs:287`), the
/// class a `ProgressBar` gets with no `.style(..)`: it paints the track in the
/// palette's strong background and the bar in the primary family.
///
/// Every field of `progress_bar::Style` is a native one: the track is
/// `progress_bar.track_color`, the filled bar is `.fill_color`, and the border
/// is `.border.*`. A progress bar has no `Status`, so the closure takes the
/// theme alone and nothing here comes from iced.
///
/// The bar's thickness is not a `Style` field: `progress_bar.track_height` is
/// the argument of `ProgressBar::girth(..)` (`progress_bar.rs:97`), set where
/// the widget is built. `progress_bar.min_width` has no receiver at all:
/// `ProgressBar::length(..)` (`progress_bar.rs:91`) takes the `Length` the
/// consumer lays the bar out with, never a floor under it.
#[must_use = "this returns the style function; it does not apply it"]
pub fn progress_bar(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme) -> iced_widget::progress_bar::Style + Clone + use<> {
    use iced_widget::progress_bar::Style;

    let p = &resolved.progress_bar;

    let track = to_color(p.track_color);
    let filled = to_color(p.fill_color);

    let border = border_of(&p.border);

    move |_theme| Style {
        background: Background::Color(track),
        bar: Background::Color(filled),
        border,
    }
}

/// The platform's own separator, for `rule::horizontal(..).style(..)` and
/// `rule::vertical(..).style(..)`.
///
/// Replaces `iced_widget::rule::default` (`rule.rs:304`), which paints the
/// line in the palette's strong background.
///
/// The color is `separator.line_color`. The line's *thickness* is not a
/// `Style` field either: it is the constructor's argument, so a consumer
/// writes `rule::horizontal(resolved.separator.line_width)`.
///
/// `SeparatorTheme` is that color and that width and nothing else, so the
/// remaining three fields have no native source and come from
/// `rule::default(theme)`: the corner `radius` of the line, the `fill_mode`
/// that decides how much of the container it spans, and `snap`.
#[must_use = "this returns the style function; it does not apply it"]
pub fn rule(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme) -> iced_widget::rule::Style + Clone + use<> {
    use iced_widget::rule::Style;

    let line = to_color(resolved.separator.line_color);

    move |theme| {
        let iced = iced_widget::rule::default(theme);
        Style {
            color: line,
            radius: iced.radius,
            fill_mode: iced.fill_mode,
            snap: iced.snap,
        }
    }
}

/// The platform's own split lines, for `pane_grid(..).style(..)`.
///
/// Replaces `iced_widget::pane_grid::default` (`pane_grid.rs:1319`), the class
/// a `PaneGrid` gets with no `.style(..)`, which draws a hovered split in the
/// palette's primary and a picked one in its strong primary, both 2 px wide.
///
/// A hovered split is `splitter.hover_color` at `splitter.divider_width`. A
/// picked split -- one being dragged -- is the same line: the model states no
/// dragged colour (`docs/platform-facts.md` §2.17 states the divider's width
/// and colour, and `SplitterTheme` adds the hover), and the dragged line is
/// the one under the pointer, so it keeps its hover. `splitter.divider_color`
/// has no receiver: iced draws a split only while it is hovered or picked
/// (`pane_grid.rs:949-956`), never at rest.
///
/// `hovered_region`, the highlight under a pane dragged over another, has no
/// native source -- the model states no drop target -- and comes from
/// `pane_grid::default(theme)`.
#[must_use = "this returns the style function; it does not apply it"]
pub fn pane_grid(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme) -> iced_widget::pane_grid::Style + Clone + use<> {
    use iced_widget::pane_grid::{Line, Style};

    let s = &resolved.splitter;

    let split = Line {
        color: to_color(s.hover_color),
        width: s.divider_width,
    };

    move |theme| Style {
        hovered_region: iced_widget::pane_grid::default(theme).hovered_region,
        picked_split: split,
        hovered_split: split,
    }
}

/// The platform's own tooltip, for `Tooltip::style(..)`.
///
/// A tooltip is styled as a container (`tooltip.rs:139-148`), so this returns a
/// `container::Style` and replaces `iced_widget::container::transparent`
/// (`container.rs:573`) -- the class a `Tooltip` gets with no `.style(..)`,
/// which paints nothing at all and leaves the tip floating over whatever is
/// under it.
///
/// The panel is `tooltip.background_color`, its label `tooltip.font.color` and
/// its outline `tooltip.border.*`. `shadow` and `snap` have no native source
/// and come from `container::Style::default()`: the model carries a shadow
/// color but no offset or blur, and `snap` is a renderer setting.
///
/// `tooltip.max_width` is not a `Style` field: iced wraps the tip's own
/// content element, so a consumer gives that element the width.
#[must_use = "this returns the style function; it does not apply it"]
pub fn tooltip(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<> {
    use iced_widget::container::Style;

    let t = &resolved.tooltip;

    let panel = to_color(t.background_color);
    let label = to_color(t.font.color);

    let border = border_of(&t.border);

    move |_theme| {
        let iced = Style::default();
        Style {
            text_color: Some(label),
            background: Some(Background::Color(panel)),
            border,
            shadow: iced.shadow,
            snap: iced.snap,
        }
    }
}

/// The platform's own card, for `container(..).style(..)`.
///
/// Replaces `iced_widget::container::transparent` (`container.rs:573`), the
/// class a `Container` gets with no `.style(..)`, and the `rounded_box` a
/// consumer reaches for instead, which paints the palette's weak background on
/// a hardcoded radius.
///
/// The fill is `card.background_color` and the outline is `card.border.*`.
///
/// `text_color` stays `None`, which is iced's own value and means *inherit*:
/// `CardTheme` carries no font, so a card does not recolor the text inside it
/// -- the surrounding label color, which the palette already states, goes on
/// showing through. `shadow` and `snap` are iced's for the same reasons they
/// are on a button.
#[must_use = "this returns the style function; it does not apply it"]
pub fn container_card(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<> {
    use iced_widget::container::Style;

    let c = &resolved.card;

    let panel = to_color(c.background_color);

    let border = border_of(&c.border);

    move |_theme| {
        let iced = Style::default();
        Style {
            text_color: iced.text_color,
            background: Some(Background::Color(panel)),
            border,
            shadow: iced.shadow,
            snap: iced.snap,
        }
    }
}

/// Where a segment sits in its segmented control, which decides the corners
/// [`segment`] rounds: a segment rounds the corners it shares with the
/// control's outline and no others.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SegmentPosition {
    /// The leftmost of several segments: its left corners are rounded.
    First,
    /// Between two others: no corner is rounded.
    Middle,
    /// The rightmost of several segments: its right corners are rounded.
    Last,
    /// The one segment of its control: all four corners are rounded.
    Only,
}

impl SegmentPosition {
    /// The position of segment `index` among `count` segments, counted from
    /// the left.
    #[must_use]
    pub fn of(index: usize, count: usize) -> Self {
        let last = count.saturating_sub(1);
        match (index == 0, index >= last) {
            (true, true) => Self::Only,
            (true, false) => Self::First,
            (false, true) => Self::Last,
            (false, false) => Self::Middle,
        }
    }
}

/// The platform's own segmented control, for the `container(..).style(..)`
/// that holds its segments.
///
/// iced has no segmented control, so one is built from iced's own widgets: a
/// container holding a row of buttons, each styled by [`segment`], with a
/// [`segment_separator`] between each two where the control states a
/// `separator_width`; the row's height is `Shrink`, so a separator, `Fill`
/// tall, spans the segments' height:
///
/// ```rust
/// use iced::widget::{Container, container, row, space};
/// use iced::{Element, Fill, Shrink};
/// use native_theme_iced::styles;
///
/// # fn f<'a>(resolved: &native_theme_iced::ResolvedTheme, segments: Vec<Element<'a, ()>>, disabled: bool) -> Container<'a, ()> {
/// let sc = &resolved.segmented_control;
/// let mut parts = Vec::new();
/// for (i, segment) in segments.into_iter().enumerate() {
///     if i > 0 && sc.separator_width > 0.0 {
///         let line = container(space()).width(sc.separator_width).height(Fill);
///         parts.push(line.style(styles::segment_separator(&resolved, disabled)).into());
///     }
///     parts.push(segment);
/// }
/// container(row(parts).height(Shrink))
///     .padding(sc.border.line_width)
///     .style(styles::segmented_control(&resolved, disabled))
/// # }
/// ```
///
/// Its fill is `segmented_control.background_color`, the group every
/// segment lies on, and its outline `segmented_control.border.*`, the line
/// colour at the line width, rounded by the corner radius; the padding of
/// the line width keeps the segments inside the outline. A segment paints
/// only what it lays over the group (its selected or hovered fill), so a
/// translucent group fill, as windows-11 states, shows once, over what lies
/// under the control, as gpui's showcase draws the control.
///
/// `text_color` is `segmented_control.font.color`, which the segments'
/// labels also take from [`segment`]. `shadow` and `snap` have no native
/// source and come from `container::Style::default()`: the model carries a
/// shadow color but no offset or blur, and `snap` is a renderer setting.
///
/// `disabled` is whether the control is disabled, its segments built with no
/// `on_press`: a disabled control is faded as one widget by
/// `segmented_control.disabled_opacity` over the window,
/// `defaults.background_color`, as the model defines the opacity, for the
/// entire widget (`docs/platform-facts.md` §2.1.6): the group fill, the
/// outline, the separators ([`segment_separator`] with the same `disabled`),
/// the segments and their labels ([`segment`], `Status::Disabled`), each
/// composited over what it lies on and faded over the window. Where the
/// opacity is 1 the colours are as stated. The flag must agree with the
/// segments' status: a container styled enabled round segments built with
/// no `on_press` draws an unfaded group under faded segments, and one
/// styled disabled round enabled segments the reverse.
///
/// The control's height is not a `Style` field: `segmented_control.segment_height`
/// is the control's outer height, which a consumer gives the segments less
/// the outline above and below them.
#[must_use = "this returns the style function; it does not apply it"]
pub fn segmented_control(
    resolved: &ResolvedTheme,
    disabled: bool,
) -> impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<> {
    use iced_widget::container::Style;

    let s = &resolved.segmented_control;
    let fade = SegmentedFade::of(resolved, disabled);

    let group = fade.over_ground(to_color(s.background_color));
    let label = fade.over_group(to_color(s.font.color));

    let mut border = border_of(&s.border);
    border.color = fade.over_ground(border.color);

    move |_theme| {
        let iced = Style::default();
        Style {
            text_color: Some(label),
            background: Some(Background::Color(group)),
            border,
            shadow: iced.shadow,
            snap: iced.snap,
        }
    }
}

/// How a segmented control fades as one widget, disabled: by
/// `segmented_control.disabled_opacity` over the window, each colour
/// composited over what it lies on first ([`segmented_control`]). Enabled, an
/// opacity of 1, which leaves every colour as stated.
#[derive(Clone, Copy)]
struct SegmentedFade {
    /// The window under the control.
    ground: Color,
    /// The group fill over the window, what a segment lies on.
    group: Color,
    /// The opacity the control is faded by.
    opacity: f32,
}

impl SegmentedFade {
    fn of(resolved: &ResolvedTheme, disabled: bool) -> Self {
        let s = &resolved.segmented_control;
        let ground = to_color(resolved.defaults.background_color);
        Self {
            ground,
            group: composite_over(to_color(s.background_color), ground),
            opacity: if disabled { s.disabled_opacity } else { 1.0 },
        }
    }

    /// `color` laid on `under`, faded with the control.
    fn over(self, under: Color, color: Color) -> Color {
        faded_as_one(self.ground, under, color, self.opacity)
    }

    /// `color` laid on the window, faded with the control: the group fill and
    /// the outline.
    fn over_ground(self, color: Color) -> Color {
        self.over(self.ground, color)
    }

    /// `color` laid on the group, faded with the control: a separator, a
    /// segment's fill, an unselected segment's label.
    fn over_group(self, color: Color) -> Color {
        self.over(self.group, color)
    }
}

/// The line between two segments of the platform's own segmented control,
/// for `container(space()).width(separator_width).height(Fill).style(..)`
/// between them in the row [`segmented_control`] holds: a fill of
/// `segmented_control.border.color`, the one line colour the control states
/// (the model states the separators' width and no colour of their own). A
/// container, not a `rule::vertical`, because a rule rounds its line's x to
/// a whole point (iced_widget 0.14.2 `src/rule.rs`, `draw`), off the
/// segments' edges. `text_color`, `border`, `shadow` and `snap` are
/// `container::Style::default()`'s. A `separator_width` of 0 is no line: the
/// consumer adds none. `disabled` fades it with the control, as
/// [`segmented_control`] says.
#[must_use = "this returns the style function; it does not apply it"]
pub fn segment_separator(
    resolved: &ResolvedTheme,
    disabled: bool,
) -> impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<> {
    use iced_widget::container::Style;

    let line = SegmentedFade::of(resolved, disabled)
        .over_group(to_color(resolved.segmented_control.border.color));

    move |_theme| Style {
        background: Some(Background::Color(line)),
        ..Style::default()
    }
}

/// One segment of the platform's own segmented control, for
/// `button(..).style(..)` inside the container [`segmented_control`] styles.
///
/// `selected` is whether this is the chosen segment, and `position` where it
/// sits, which decides its corners.
///
/// A segment lies on the control's group fill and paints only what the
/// model lays over it. An unselected segment paints no fill at rest
/// (`background` is `None`) and `.hover_background` under the pointer, laid
/// over the group as given -- a soft option, whose `None` is the platform
/// saying a hovered segment looks no different; its label is `.font.color`.
/// The selected segment is `segmented_control.active_background` with
/// `.active_text_color`, and the model states no hover for it, so a hovered
/// selected segment is exactly the selected one. The model states no pressed
/// segment either: a pressed segment is the hovered one.
///
/// A disabled segment is its enabled self faded as one with the control, as
/// [`segmented_control`] says: its fill over the group, its label over its
/// fill, each faded by `segmented_control.disabled_opacity` over the window.
/// iced gives a button `Status::Disabled` when it has no `on_press`, so a
/// disabled control's segments are built without one, and its container and
/// separators are styled with `disabled` true to agree.
///
/// Each segment rounds the corners it shares with the control's outline, to
/// the outline's inner radius -- `border.corner_radius` less
/// `border.line_width`, and never below zero -- so its fill stays inside the
/// rounded outline; its other corners are square. The segment has no border
/// of its own -- the control's outline is the container's, its separators
/// [`segment_separator`]'s -- so the border's colour and width are iced's own
/// for a button, `button::Style::default()`'s none, and so are `shadow` and
/// `snap`.
#[must_use = "this returns the style function; it does not apply it"]
pub fn segment(
    resolved: &ResolvedTheme,
    selected: bool,
    position: SegmentPosition,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
    use iced_widget::button::{Status, Style};

    let s = &resolved.segmented_control;

    let (idle, hovered, label) = if selected {
        let fill = Some(to_color(s.active_background));
        (fill, fill, to_color(s.active_text_color))
    } else {
        (
            None,
            s.hover_background.map(to_color),
            to_color(s.font.color),
        )
    };
    let fade = SegmentedFade::of(resolved, true);
    let disabled = idle.map(|fill| fade.over_group(fill));
    let disabled_label = match idle {
        Some(fill) => fade.over(composite_over(fill, fade.group), label),
        None => fade.over_group(label),
    };

    let inner_radius = (s.border.corner_radius - s.border.line_width).max(0.0);
    let radius = match position {
        SegmentPosition::First => Radius::new(0.0).left(inner_radius),
        SegmentPosition::Middle => Radius::new(0.0),
        SegmentPosition::Last => Radius::new(0.0).right(inner_radius),
        SegmentPosition::Only => Radius::new(inner_radius),
    };

    move |_theme, status| {
        let iced = Style::default();
        let (background, text_color) = match status {
            Status::Active => (idle, label),
            Status::Hovered | Status::Pressed => (hovered, label),
            Status::Disabled => (disabled, disabled_label),
        };
        Style {
            background: background.map(Background::Color),
            text_color,
            border: Border {
                radius,
                ..iced.border
            },
            shadow: iced.shadow,
            snap: iced.snap,
        }
    }
}

/// The header of the platform's own expander, for `button(..).style(..)`.
///
/// iced has no expander, so one is built from iced's own widgets: a button
/// whose label is the disclosure arrow and the title, above the body a
/// consumer shows while it is expanded. The arrow's colour is
/// [`expander_arrow_color`](crate::expander_arrow_color); its size is
/// `expander.arrow_icon_size`, and the header's height `expander.header_height`,
/// both builder geometry.
///
/// The model states no resting fill for an expander header, so it paints
/// none: `background` is `None` at rest. Under the pointer it is
/// `expander.hover_background` -- a soft option, whose `None` is the platform
/// saying a hovered header looks no different -- emitted as given, because
/// there is no fill of the header's own beneath it to layer it over. The model
/// states no pressed header, so a pressed header is the hovered one.
///
/// The label is `expander.font.color`, and the outline is `expander.border.*`.
/// A disabled header paints no fill, and its label's and outline's alpha is
/// multiplied by `defaults.disabled_opacity`, as iced has no widget opacity:
/// `ExpanderTheme` states no `disabled_opacity` of its own, and a widget
/// without one fades by the defaults' (`docs/inheritance-rules.toml`).
///
/// The header draws no outline -- its width is 0 -- whatever the theme states
/// of `expander.frame_enabled`: a framed expander's frame holds its header and
/// its body together, [`expander_frame`], an unframed one has none, and where
/// the theme states nothing (the macOS facts give "(none)",
/// `docs/platform-facts.md` §2.27) it is drawn unframed, KDE's shape, as the
/// egui connector draws it: platforms either frame header and body together
/// (WinUI, libadwaita) or neither, none the header alone. The border's colour
/// and radius stay `expander.border.*`'s, the radius the hovered fill's.
/// `shadow` and `snap` have no native source and come from
/// `button::Style::default()`.
#[must_use = "this returns the style function; it does not apply it"]
pub fn expander(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
    use iced_widget::button::{Status, Style};

    let x = &resolved.expander;

    let hovered = x.hover_background.map(to_color);
    let label = to_color(x.font.color);
    // `ExpanderTheme` states no `disabled_opacity`: the defaults' fades it.
    let opacity = resolved.defaults.disabled_opacity;

    // The header draws no outline: a stated frame is `expander_frame`'s, a
    // stated absence of one is none, and where the theme states nothing no
    // platform outlines a header alone.
    let border = Border {
        color: to_color(x.border.color),
        width: 0.0,
        radius: Radius::new(x.border.corner_radius),
    };

    move |_theme, status| {
        let iced = Style::default();
        let (background, text_color, border) = match status {
            Status::Active => (None, label, border),
            Status::Hovered | Status::Pressed => (hovered, label, border),
            Status::Disabled => (None, faded(label, opacity), faded_border(border, opacity)),
        };
        Style {
            background: background.map(Background::Color),
            text_color,
            border,
            shadow: iced.shadow,
            snap: iced.snap,
        }
    }
}

/// The frame of the platform's own expander, for the `container(..).style(..)`
/// that holds its header and its body, where the theme states
/// `expander.frame_enabled = true`.
///
/// libadwaita and WinUI frame an expander's header and body together, and
/// KDE frames neither (`docs/platform-facts.md` §2.27), so the frame is the
/// container round both, and the header [`expander`] styles draws no outline
/// of its own:
///
/// ```rust
/// use iced::widget::{column, container};
/// use iced::Element;
/// use native_theme_iced::styles;
///
/// # fn f<'a>(resolved: &native_theme_iced::ResolvedTheme, header: Element<'a, ()>, body: Element<'a, ()>) -> Element<'a, ()> {
/// let x = &resolved.expander;
/// let item = column![header, body];
/// match x.frame_enabled {
///     Some(true) => container(item)
///         .padding(x.border.line_width)
///         .style(styles::expander_frame(&resolved))
///         .into(),
///     _ => container(item).into(),
/// }
/// # }
/// ```
///
/// The outline is `expander.border.*`. Where the theme does not state
/// `frame_enabled = true` its width is 0, so a container in this style never
/// draws a second outline round a header that draws its own, or a frame the
/// platform does not. The model states no fill and no label for the frame,
/// so `background` and `text_color` are `None`, iced's own for a container --
/// no fill, and the label of whatever the expander sits in -- and `shadow`
/// and `snap` come from `container::Style::default()`.
#[must_use = "this returns the style function; it does not apply it"]
pub fn expander_frame(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<> {
    use iced_widget::container::Style;

    let x = &resolved.expander;

    let border = Border {
        color: to_color(x.border.color),
        width: match x.frame_enabled {
            Some(true) => x.border.line_width,
            Some(false) | None => 0.0,
        },
        radius: Radius::new(x.border.corner_radius),
    };

    move |_theme| {
        let iced = Style::default();
        Style {
            text_color: iced.text_color,
            background: iced.background,
            border,
            shadow: iced.shadow,
            snap: iced.snap,
        }
    }
}

/// The platform's dialog surface, for `container(..).style(..)` on the
/// container a consumer draws a dialog in: iced 0.14 and `iced_aw` 0.14.1
/// have no dialog widget.
///
/// The fill is `dialog.background_color`, the outline `dialog.border.*`
/// and the text colour `dialog.body_font.color`, which the body's texts
/// inherit; the title's colour is [`crate::dialog_title_font_color`], for
/// the consumer's title text. `shadow` and `snap` come from
/// `container::Style::default()`: the model carries a shadow colour but no
/// offset or blur, and `snap` is a renderer setting. The padding and the
/// bounds are [`crate::dialog_padding`], [`crate::dialog_max_width`],
/// [`crate::dialog_max_height`], [`crate::dialog_min_width`] and
/// [`crate::dialog_min_height`], whose doc shows the whole container.
#[must_use = "this returns the style function; it does not apply it"]
pub fn dialog(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<> {
    use iced_widget::container::Style;

    let d = &resolved.dialog;

    let panel = to_color(d.background_color);
    let label = to_color(d.body_font.color);

    let border = border_of(&d.border);

    move |_theme| {
        let iced = Style::default();
        Style {
            text_color: Some(label),
            background: Some(Background::Color(panel)),
            border,
            shadow: iced.shadow,
            snap: iced.snap,
        }
    }
}

/// The platform's toolbar surface, for `container(..).style(..)` on the
/// container a consumer draws its toolbar in: iced 0.14 and `iced_aw` 0.14.1
/// have no toolbar widget (docs/platform-facts.md §2.13).
///
/// The fill is `toolbar.background_color` and the text colour
/// `toolbar.font.color`, which the bar's texts inherit. The container draws
/// no outline: the toolbar's edge is one rule under the bar,
/// [`toolbar_rule`] at [`crate::toolbar_border_line_width`], as the gpui
/// connector's `geometry::toolbar` draws it (`border_b`) and the egui
/// connector strokes it as the top panel's separator. `border`, `shadow`
/// and `snap` come from `container::Style::default()`: a frame round the
/// bar is no platform's, the model carries a shadow colour but no offset or
/// blur, and `snap` is a renderer setting. The padding, the gap between the
/// items and the outer height are [`crate::toolbar_padding`],
/// [`crate::toolbar_item_gap`] and [`crate::toolbar_bar_height`], whose doc
/// shows the whole bar. The items' text is in `toolbar.font`, which a style
/// cannot carry: `crate::font(&resolved.toolbar.font)` (feature
/// `system-fonts`) at [`crate::scaled_text_size`]`(resolved.toolbar.font.size,
/// &prefs)`.
#[must_use = "this returns the style function; it does not apply it"]
pub fn toolbar(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<> {
    use iced_widget::container::Style;

    let t = &resolved.toolbar;

    let bar = to_color(t.background_color);
    let label = to_color(t.font.color);

    move |_theme| {
        let iced = Style::default();
        Style {
            text_color: Some(label),
            background: Some(Background::Color(bar)),
            border: iced.border,
            shadow: iced.shadow,
            snap: iced.snap,
        }
    }
}

/// The toolbar's bottom edge, for `rule::horizontal(..).style(..)` under the
/// bar [`toolbar`] styles: `toolbar.border.color`, at the height
/// [`crate::toolbar_border_line_width`] gives `rule::horizontal`. A width of
/// 0 draws none, as the theme states it. `radius`, `fill_mode` and `snap`
/// are iced's, as [`rule`]'s are.
#[must_use = "this returns the style function; it does not apply it"]
pub fn toolbar_rule(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme) -> iced_widget::rule::Style + Clone + use<> {
    use iced_widget::rule::Style;

    let line = to_color(resolved.toolbar.border.color);

    move |theme| {
        let iced = iced_widget::rule::default(theme);
        Style {
            color: line,
            radius: iced.radius,
            fill_mode: iced.fill_mode,
            snap: iced.snap,
        }
    }
}

/// The application's own background and default text, for
/// `iced::application(..).style(..)` (`iced-0.14.0/src/application.rs:394-399`).
///
/// Replaces the theme's base style, `Base::base`
/// (`iced_core-0.14.0/src/theme.rs:291-293`), which takes the background from
/// the palette's background (`:328-336`), under this connector
/// `defaults.background_color`, where the model states the window's own fill,
/// `window.background_color`. The text colour is the base style's: the
/// palette's `background.base.text`, which the connector sets from
/// `defaults.text_color`; the model states no label of the window's own.
///
/// iced paints the background under the whole window and gives the text
/// colour to every text that states none (`theme::Style`,
/// `iced_core-0.14.0/src/theme.rs:222-230`). `application(..).style(..)`
/// takes `Fn(&State, &Theme) -> theme::Style`, so call this inside it, from
/// the `ResolvedTheme` the state keeps:
///
/// ```rust,no_run
/// use native_theme_iced::{ResolvedTheme, from_preset, styles};
///
/// struct State {
///     theme: iced::Theme,
///     resolved: ResolvedTheme,
/// }
/// # fn update(_state: &mut State, _message: ()) {}
/// # fn view(_state: &State) -> iced::Element<'_, ()> { iced::widget::text("").into() }
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let (theme, resolved) = from_preset("adwaita", false)?;
///     let boot = move || State { theme: theme.clone(), resolved: resolved.clone() };
///     iced::application(boot, update, view)
///         .theme(|state: &State| state.theme.clone())
///         .style(|state: &State, theme| styles::application(&state.resolved)(theme))
///         .run()?;
///     Ok(())
/// }
/// ```
#[must_use = "this returns the style function; it does not apply it"]
pub fn application(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme) -> iced_core::theme::Style + Clone + use<> {
    use iced_core::theme::{Base, Style};

    let background = to_color(resolved.window.background_color);

    move |theme| Style {
        background_color: background,
        text_color: theme.base().text_color,
    }
}
