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
//! | `button` | [`button`], [`button_primary`], [`button_danger`], [`button_success`], [`button_warning`], [`button_link`], [`button_toggle`], [`toolbar_button`], [`button_default`]; [`segment`] and [`expander`] for the buttons of a segmented control and of an expander's header |
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
//! default for that widget, inside the closure, never written as a literal,
//! except where the platform draws nothing (a transparent fill, a zero width
//! or radius).
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
use iced_core::{Background, Border, Color, Shadow};
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

pub(crate) use crate::next::LineAlign;

/// A border's stroke colour as the theme states it, and where it lies.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Line {
    pub color: Color,
    pub align: LineAlign,
}

impl Line {
    /// A stroke handed to iced as given.
    pub(crate) const fn outside(color: Color) -> Self {
        Self {
            color,
            align: LineAlign::Outside,
        }
    }

    /// The colour iced on wgpu is handed for this stroke on a widget it
    /// paints `fill` under (`crate::next`'s module doc): iced ends the fill at
    /// the border's inner edge, so an `Inside` stroke, which the theme lays
    /// over the fill, is the stroke composited over the fill, and a `Center`
    /// one is composited over the fill at half its alpha, the band's mean.
    /// An opaque or `Outside` stroke is as stated, and so is any stroke over
    /// no fill (a transparent `fill`).
    pub(crate) fn over(self, fill: Color) -> Color {
        if self.color.a >= 1.0 {
            return self.color;
        }
        match self.align {
            LineAlign::Outside => self.color,
            LineAlign::Inside => composite_over(self.color, fill),
            LineAlign::Center => composite_over(
                self.color,
                Color {
                    a: fill.a / 2.0,
                    ..fill
                },
            ),
        }
    }

    /// This stroke where it lies `align`: the 0.7.2 stroke as a 0.8.0 one
    /// that lies there, for the parity test's expected values.
    #[cfg(test)]
    pub(crate) fn aligned(self, align: LineAlign) -> Self {
        Self { align, ..self }
    }
}

/// A border as the theme states it: the stroke, its width and its corners.
/// [`Outline::over`] gives the `Border` iced is handed over a fill.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Outline {
    pub line: Line,
    pub width: f32,
    pub radius: Radius,
}

impl Outline {
    /// The 0.7.2 model's border, its stroke as given.
    pub(crate) fn of(border: &ResolvedWidgetBorder) -> Self {
        Self {
            line: Line::outside(to_color(border.color)),
            width: border.line_width,
            radius: Radius::new(border.corner_radius),
        }
    }

    /// The border iced is handed on a widget it paints `fill` under.
    pub(crate) fn over(self, fill: Color) -> Border {
        Border {
            color: self.line.over(fill),
            width: self.width,
            radius: self.radius,
        }
    }

    /// This outline in `line`, a state's own stroke, where one is stated.
    pub(crate) fn with(self, line: Option<Line>) -> Self {
        Self {
            line: line.unwrap_or(self.line),
            ..self
        }
    }

    /// [`Line::aligned`] of the outline's stroke.
    #[cfg(test)]
    pub(crate) fn aligned(self, align: LineAlign) -> Self {
        Self {
            line: self.line.aligned(align),
            ..self
        }
    }
}

/// The alpha iced's own button classes scale a disabled button's fill and
/// label by (iced_widget 0.14.2 `src/button.rs`, `fn disabled`:
/// `scale_alpha(0.5)`), its disabled look where a theme states no opacity.
pub(crate) const ICED_DISABLED_BUTTON_ALPHA: f32 = 0.5;

/// A disabled button's fill, label and border, of the disabled `fill` and
/// `label` where the theme states them and the `idle` pair the button keeps
/// where it does not. Where the theme states a disabled `opacity`, the pair
/// is faded by it ([`faded`]) and the border is its stroke over the fill,
/// faded too. Where it states none, a disabled colour it states is drawn as
/// stated, and one it leaves to iced is faded as iced's own classes fade a
/// disabled button, to [`ICED_DISABLED_BUTTON_ALPHA`]; the border is its
/// stroke over the fill so painted, which iced's classes leave as it is
/// (spec §10, rule 0).
fn disabled_button(
    fill: Option<Color>,
    idle: Color,
    label: Option<Color>,
    idle_label: Color,
    outline: Outline,
    opacity: Option<f32>,
) -> (Color, Color, Border) {
    match opacity {
        Some(opacity) => {
            let fill = fill.unwrap_or(idle);
            (
                faded(fill, opacity),
                faded(label.unwrap_or(idle_label), opacity),
                faded_border(outline.over(fill), opacity),
            )
        }
        None => {
            let fill = fill.unwrap_or(idle.scale_alpha(ICED_DISABLED_BUTTON_ALPHA));
            let label = label.unwrap_or(idle_label.scale_alpha(ICED_DISABLED_BUTTON_ALPHA));
            (fill, label, outline.over(fill))
        }
    }
}

/// A state's fill as the theme states it, where the style lays it over the
/// fill under it: a colour, or no fill (a stated `Paint::None`), which
/// paints nothing in that state whatever lies under it. The 0.7.2 model
/// states every such fill as a colour.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Layer {
    Color(Color),
    Nothing,
}

impl Layer {
    /// The fill painted where this layer lies over `base`: the colour
    /// composited over it ([`composite_over`]), or nothing.
    pub(crate) fn over(self, base: Color) -> Color {
        match self {
            Self::Color(color) => composite_over(color, base),
            Self::Nothing => Color::TRANSPARENT,
        }
    }

    /// The fill painted where this layer is painted as given.
    pub(crate) fn as_given(self) -> Color {
        match self {
            Self::Color(color) => color,
            Self::Nothing => Color::TRANSPARENT,
        }
    }
}

/// The colour of a fill iced's own style paints: none where it paints a
/// gradient or nothing, which no stroke is composited over.
fn painted(background: Option<Background>) -> Color {
    match background {
        Some(Background::Color(color)) => color,
        Some(Background::Gradient(_)) | None => Color::TRANSPARENT,
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
/// Hovered and pressed, the border is `button.hover_border_color` where the
/// theme states one (a pressed button is under the pointer); an iced button
/// takes no keyboard focus (iced_widget 0.14.2 `src/button.rs:257-273`), so
/// `button.focus_border_color` has no receiver.
///
/// `shadow` and `snap` have no native source and come from
/// `button::Style::default()`: the model carries a shadow color but no offset
/// or blur, and `snap` is a renderer setting (`cfg!(feature = "crisp")`).
#[must_use = "this returns the style function; it does not apply it"]
pub fn button(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
    button_from(&ButtonSource::of(resolved))
}

/// What the button styles read from the theme, of either model: each
/// field's 0.7.2 name, soft options as `None` where unset. A hover or
/// pressed fill or label no rule states is `None`, and the button is then
/// iced's own for that status (spec §2, `from_state`), never the idle fill
/// composited over itself; the disabled look where the theme states no
/// opacity is iced's own too (spec §10, rule 0). The 0.7.2 model states
/// both.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ButtonSource {
    pub background: Color,
    pub hover_background: Option<Layer>,
    pub active_background: Option<Layer>,
    pub disabled_background: Option<Color>,
    pub checked_background: Option<Color>,
    pub label: Color,
    pub hover_label: Option<Color>,
    pub active_label: Option<Color>,
    /// The disabled label where the theme states one.
    pub disabled_label: Option<Color>,
    pub checked_label: Option<Color>,
    pub disabled_opacity: Option<f32>,
    pub border: Outline,
    /// The primary button's own border: 0.7.2's `border`, which the model
    /// gives every button.
    pub primary_border: Outline,
    pub hover_border_color: Option<Line>,
    /// The border of a pressed button: 0.7.2's `hover_border_color`, which a
    /// pressed button keeps.
    pub pressed_border_color: Option<Line>,
    /// The primary button's own hovered and pressed borders: 0.7.2's
    /// `hover_border_color`, which the model gives every button.
    pub primary_hover_border_color: Option<Line>,
    pub primary_pressed_border_color: Option<Line>,
    pub primary_background: Color,
    pub primary_label: Color,
    pub default_background: Option<Color>,
    pub default_label: Option<Color>,
    pub default_border_color: Option<Line>,
    /// The palette's background and text, which iced's primary family is
    /// generated against.
    pub window_background: Color,
    pub window_text: Color,
}

impl ButtonSource {
    /// The 0.7.2 model's reads.
    pub(crate) fn of(resolved: &ResolvedTheme) -> Self {
        let b = &resolved.button;
        let d = &resolved.defaults;
        let line = |c: Option<Rgba>| c.map(|c| Line::outside(to_color(c)));
        Self {
            background: to_color(b.background_color),
            hover_background: Some(Layer::Color(to_color(b.hover_background))),
            active_background: b.active_background.map(|c| Layer::Color(to_color(c))),
            disabled_background: b.disabled_background.map(to_color),
            checked_background: b.checked_background.map(to_color),
            label: to_color(b.font.color),
            hover_label: Some(to_color(b.hover_text_color)),
            active_label: Some(to_color(b.active_text_color)),
            disabled_label: Some(to_color(b.disabled_text_color)),
            checked_label: b.checked_text_color.map(to_color),
            disabled_opacity: Some(b.disabled_opacity),
            border: Outline::of(&b.border),
            primary_border: Outline::of(&b.border),
            hover_border_color: line(b.hover_border_color),
            pressed_border_color: line(b.hover_border_color),
            primary_hover_border_color: line(b.hover_border_color),
            primary_pressed_border_color: line(b.hover_border_color),
            primary_background: to_color(b.primary_background),
            primary_label: to_color(b.primary_text_color),
            default_background: b.default_button_background.map(to_color),
            default_label: b.default_button_text_color.map(to_color),
            default_border_color: line(b.default_button_border_color),
            window_background: to_color(d.background_color),
            window_text: to_color(d.text_color),
        }
    }

    /// The outline a button shows in `status`: under the pointer the hover
    /// border where the theme states one, pressed the pressed one, else the
    /// one at rest.
    fn outline_at(&self, status: iced_widget::button::Status) -> Outline {
        use iced_widget::button::Status;
        match status {
            Status::Active | Status::Disabled => self.border,
            Status::Hovered => self.border.with(self.hover_border_color),
            Status::Pressed => self.border.with(self.pressed_border_color),
        }
    }

    /// The hovered and the pressed fill: each layer over the idle fill.
    /// `active_background` is a soft option: `None` is the platform saying
    /// the pressed state has no fill of its own, so it copies the hover
    /// layer. With no hover layer either, the theme states nothing for the
    /// state, and its fill is `None`: iced's own for that status (spec §2,
    /// `from_state`; spec §10, rule 0).
    fn layered(&self) -> (Option<Color>, Option<Color>) {
        let over = |layer: Option<Layer>| layer.map(|layer| layer.over(self.background));
        (
            over(self.hover_background),
            over(self.active_background.or(self.hover_background)),
        )
    }

    /// Every stroke where it lies `align`, for the parity test's expected
    /// values.
    #[cfg(test)]
    pub(crate) fn aligned(self, align: LineAlign) -> Self {
        let line = |l: Option<Line>| l.map(|l| l.aligned(align));
        Self {
            border: self.border.aligned(align),
            primary_border: self.primary_border.aligned(align),
            hover_border_color: line(self.hover_border_color),
            pressed_border_color: line(self.pressed_border_color),
            primary_hover_border_color: line(self.primary_hover_border_color),
            primary_pressed_border_color: line(self.primary_pressed_border_color),
            default_border_color: line(self.default_border_color),
            ..self
        }
    }

    /// [`Self::aligned`], and the state options `listed` names unset, for
    /// the parity test's expected values.
    #[cfg(test)]
    pub(crate) fn listed(self, listed: crate::next::parity::Listed) -> Self {
        let s = listed.states;
        let a = self.aligned(listed.align);
        Self {
            hover_background: a.hover_background.filter(|_| !s.hover_fill),
            active_background: a.active_background.filter(|_| !s.pressed_fill),
            hover_label: a.hover_label.filter(|_| !s.hover_label),
            active_label: a.active_label.filter(|_| !s.pressed_label),
            hover_border_color: a.hover_border_color.filter(|_| !s.hover_stroke),
            primary_hover_border_color: a.primary_hover_border_color.filter(|_| !s.hover_stroke),
            pressed_border_color: a.pressed_border_color.filter(|_| !s.pressed_stroke),
            primary_pressed_border_color: a
                .primary_pressed_border_color
                .filter(|_| !s.pressed_stroke),
            ..a
        }
    }

    /// The source of the primary button: its own borders in place of the
    /// plain ones.
    fn primary(self) -> Self {
        Self {
            border: self.primary_border,
            hover_border_color: self.primary_hover_border_color,
            pressed_border_color: self.primary_pressed_border_color,
            ..self
        }
    }
}

/// [`button`] of what it reads.
pub(crate) fn button_from(
    b: &ButtonSource,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
    use iced_widget::button::{Status, Style};

    let source = *b;
    let idle = b.background;
    let (hovered, pressed) = b.layered();

    let label = b.label;
    let hovered_label = b.hover_label;
    let pressed_label = b.active_label;
    let opacity = b.disabled_opacity;

    move |theme, status| {
        let iced = Style::default();
        let outline = source.outline_at(status);
        // A state the theme states no fill for is iced's own neutral class
        // for the status, fill and label, with a label the theme states for
        // the state laid over it; a state it states a fill for is lettered
        // in the label the theme states for it, else the label at rest.
        let own = |fill: Option<Color>, text: Option<Color>| match fill {
            Some(fill) => (Some(Background::Color(fill)), text.unwrap_or(label)),
            None => {
                let own = iced_widget::button::secondary(theme, status);
                (own.background, text.unwrap_or(own.text_color))
            }
        };
        let (background, text_color) = match status {
            Status::Active => (Some(Background::Color(idle)), label),
            Status::Hovered => own(hovered, hovered_label),
            Status::Pressed => own(pressed, pressed_label),
            // A translucent disabled fill replaces the idle one and lets the
            // window through, so it is emitted as given rather than
            // composited.
            Status::Disabled => {
                let (fill, label, border) = disabled_button(
                    source.disabled_background,
                    idle,
                    source.disabled_label,
                    label,
                    outline,
                    opacity,
                );
                return Style {
                    background: Some(Background::Color(fill)),
                    text_color: label,
                    border,
                    shadow: iced.shadow,
                    snap: iced.snap,
                };
            }
        };
        Style {
            background,
            text_color,
            border: outline.over(painted(background)),
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
/// a selected button, with no hover of its own (gpui-component 0.9.1
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
    button_toggle_from(&ButtonSource::of(resolved), checked)
}

/// [`button_toggle`] of what it reads.
pub(crate) fn button_toggle_from(
    b: &ButtonSource,
    checked: bool,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
    use iced_widget::button::{Status, Style};

    let plain = button_from(b);

    let source = *b;
    let opacity = b.disabled_opacity;

    move |theme, status| {
        let style = plain(theme, status);
        if !checked {
            return style;
        }
        // A checked fill the theme does not state is the pressed button's,
        // fill and label, iced's own where the theme states no pressed one
        // either; a checked label it states is laid over it. A stated
        // checked fill is lettered in the checked label, else the pressed
        // one the theme states, else the label at rest.
        let pressed = plain(theme, Status::Pressed);
        let (fill, label) = match source.checked_background {
            Some(fill) => (
                fill,
                source
                    .checked_label
                    .or(source.active_label)
                    .unwrap_or(source.label),
            ),
            None => (
                painted(pressed.background),
                source.checked_label.unwrap_or(pressed.text_color),
            ),
        };
        let outline = source.outline_at(status);
        let (background, text_color, border) = match status {
            Status::Active | Status::Hovered | Status::Pressed => (fill, label, outline.over(fill)),
            Status::Disabled => match source.disabled_background {
                Some(_) => disabled_button(
                    source.disabled_background,
                    fill,
                    source.disabled_label,
                    source.label,
                    outline,
                    opacity,
                ),
                None => disabled_button(None, fill, None, label, outline, opacity),
            },
        };
        Style {
            background: Some(Background::Color(background)),
            text_color,
            border,
            ..style
        }
    }
}

/// A toolbar's button, for `button(..).style(..)`: flat at rest, in the
/// toolbar's states where the theme states them, and `checked` a toolbar
/// button that is on.
///
/// iced has no toolbar and no flat button class; this replaces the flat
/// button an application would otherwise build from the plain button's
/// leaves. Each `toolbar.button_*` field the theme leaves unset falls back to
/// that flat button's leaf.
///
/// At rest the button has no fill and its label is `button.font.color`.
/// Under the pointer its fill is `toolbar.button_hover_background`, or
/// `button.hover_background` where that is unset, and its label
/// `button.hover_text_color`; pressed, its fill is
/// `toolbar.button_active_background`, else `button.active_background`, else
/// `button.hover_background`, and its label `button.active_text_color`.
/// Checked, it keeps `toolbar.button_checked_background` (the pressed fill
/// where that is unset) in every enabled status, as the model states no
/// hovered and no pressed checked toolbar button, and its label is
/// `button.checked_text_color` where the theme states the checked fill and
/// the checked text, `button.active_text_color` otherwise. Disabled, it has no
/// fill and its label is `button.disabled_text_color`.
///
/// A stated fill is painted as given over the toolbar: translucent is a
/// layer, transparent nothing. The border is `button.border.line_width` wide
/// and rounded by `button.border.corner_radius`: transparent at rest and
/// disabled, and `toolbar.button_hover_border_color` under the pointer and
/// while pressed, checked or not, where the theme states it. A checked
/// button at rest is outlined in `toolbar.button_checked_border_color`,
/// Breeze's plain button pen, where the theme states it, and keeps that pen
/// under the pointer and pressed where the theme states no
/// `toolbar.button_hover_border_color`. iced lays a
/// button out by its padding and draws the border on its bounds
/// (iced_widget 0.14.2 `src/button.rs:379-385`), so the width moves no
/// content. `shadow` and `snap` come from `button::Style::default()`, as
/// [`button`]'s do.
#[must_use = "this returns the style function; it does not apply it"]
pub fn toolbar_button(
    resolved: &ResolvedTheme,
    checked: bool,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
    toolbar_button_from(
        &ButtonSource::of(resolved),
        &ToolbarButtonSource::of(resolved),
        checked,
    )
}

/// What [`toolbar_button`] reads of the toolbar, of either model: each
/// `toolbar.button_*` field, `None` where unset.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ToolbarButtonSource {
    pub hover_background: Option<Layer>,
    pub active_background: Option<Layer>,
    pub checked_background: Option<Color>,
    pub hover_border_color: Option<Line>,
    pub checked_border_color: Option<Line>,
    /// The outline of a pressed button that is off, and of a checked one
    /// under the pointer and pressed: 0.7.2's `button_hover_border_color`,
    /// over `button_checked_border_color` where checked.
    pub pressed_border_color: Option<Line>,
    pub checked_hover_border_color: Option<Line>,
    pub checked_pressed_border_color: Option<Line>,
}

impl ToolbarButtonSource {
    /// The 0.7.2 model's reads.
    pub(crate) fn of(resolved: &ResolvedTheme) -> Self {
        let t = &resolved.toolbar;
        let line = |c: Option<Rgba>| c.map(|c| Line::outside(to_color(c)));
        let checked_hover = line(
            t.button_hover_border_color
                .or(t.button_checked_border_color),
        );
        Self {
            hover_background: t.button_hover_background.map(|c| Layer::Color(to_color(c))),
            active_background: t
                .button_active_background
                .map(|c| Layer::Color(to_color(c))),
            checked_background: t.button_checked_background.map(to_color),
            hover_border_color: line(t.button_hover_border_color),
            checked_border_color: line(t.button_checked_border_color),
            pressed_border_color: line(t.button_hover_border_color),
            checked_hover_border_color: checked_hover,
            checked_pressed_border_color: checked_hover,
        }
    }

    /// Every stroke where it lies `align`, for the parity test's expected
    /// values.
    #[cfg(test)]
    pub(crate) fn aligned(self, align: LineAlign) -> Self {
        let line = |l: Option<Line>| l.map(|l| l.aligned(align));
        Self {
            hover_border_color: line(self.hover_border_color),
            checked_border_color: line(self.checked_border_color),
            pressed_border_color: line(self.pressed_border_color),
            checked_hover_border_color: line(self.checked_hover_border_color),
            checked_pressed_border_color: line(self.checked_pressed_border_color),
            ..self
        }
    }
}

/// [`toolbar_button`] of what it reads.
pub(crate) fn toolbar_button_from(
    b: &ButtonSource,
    t: &ToolbarButtonSource,
    checked: bool,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
    use iced_widget::button::{Status, Style};

    // The layers are painted as given: a toolbar button has no fill at rest
    // for them to lie over. Where the theme states a layer for neither the
    // toolbar nor the button, the state is [`button`]'s, which is iced's own
    // there.
    let plain = button_from(b);
    let hovered = t
        .hover_background
        .or(b.hover_background)
        .map(Layer::as_given);
    let pressed = t
        .active_background
        .or(b.active_background)
        .or(b.hover_background)
        .map(Layer::as_given);
    let checked_fill = t.checked_background;

    let label = b.label;
    let hover_label = b.hover_label;
    let active_label = b.active_label;
    let checked_label = b.checked_label;
    // A disabled label the theme leaves to iced is the label at rest, faded
    // by the theme's disabled opacity as `button_from` fades it, or as
    // iced's own classes fade it where the theme states no opacity.
    let disabled_label = b.disabled_label.unwrap_or(match b.disabled_opacity {
        Some(opacity) => faded(b.label, opacity),
        None => b.label.scale_alpha(ICED_DISABLED_BUTTON_ALPHA),
    });

    let outline = Outline {
        line: Line::outside(Color::TRANSPARENT),
        ..b.border
    };
    let t = *t;

    move |theme, status| {
        let iced = Style::default();
        // A state the theme states no fill for, of the toolbar or of the
        // button, is the plain button's, fill and label, iced's own there; a
        // stated fill is lettered in the label the theme states for the
        // state, else the label at rest.
        let state = |fill: Option<Color>, text: Option<Color>, status| match fill {
            Some(fill) => (fill, text.unwrap_or(label)),
            None => {
                let plain = plain(theme, status);
                (painted(plain.background), plain.text_color)
            }
        };
        let (hovered, hovered_label) = state(hovered, hover_label, Status::Hovered);
        let (pressed, pressed_label) = state(pressed, active_label, Status::Pressed);
        let checked_label = match checked_fill {
            Some(_) => checked_label.unwrap_or(pressed_label),
            None => pressed_label,
        };
        let fill = checked_fill.unwrap_or(pressed);
        let (background, text_color, line) = match (checked, status) {
            (false, Status::Active) => (Color::TRANSPARENT, label, None),
            (false, Status::Hovered) => (hovered, hovered_label, t.hover_border_color),
            (false, Status::Pressed) => (pressed, pressed_label, t.pressed_border_color),
            (true, Status::Active) => (fill, checked_label, t.checked_border_color),
            (true, Status::Hovered) => (fill, checked_label, t.checked_hover_border_color),
            (true, Status::Pressed) => (fill, checked_label, t.checked_pressed_border_color),
            (_, Status::Disabled) => (Color::TRANSPARENT, disabled_label, None),
        };
        let border = outline.with(line).over(background);
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
/// beside a differently rounded one. Hovered and pressed, the border is
/// `button.hover_border_color` where the theme states one (a pressed button is
/// under the pointer); an iced button takes no keyboard focus (iced_widget
/// 0.14.2 `src/button.rs:257-273`), so `button.focus_border_color` has no
/// receiver. Disabled is the button's disabled pair
/// where the platform states a disabled fill, which it gives every button
/// class; where it states none it dims by opacity alone
/// (`docs/platform-facts.md` §2.1.6), and the class keeps its idle pair. Either
/// is faded by `button.disabled_opacity` (see [`faded`]).
pub(crate) fn class_button(
    b: &ButtonSource,
    fill: Color,
    label: Color,
    class: fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
    use iced_widget::button::{Status, Style};

    let idle = fill;
    let idle_label = label;
    let disabled = match b.disabled_background {
        Some(_) => (b.disabled_background, idle, b.disabled_label, b.label),
        None => (None, idle, None, idle_label),
    };
    let opacity = b.disabled_opacity;
    let source = *b;

    move |theme, status| {
        let iced = class(theme, status);
        let outline = source.outline_at(status);
        let (background, text_color, border) = match status {
            Status::Active => (
                Some(Background::Color(idle)),
                idle_label,
                outline.over(idle),
            ),
            // Neither the fill nor the label of these two states has a native
            // source, so both are iced's own answer for this class.
            Status::Hovered | Status::Pressed => (
                iced.background,
                iced.text_color,
                outline.over(painted(iced.background)),
            ),
            Status::Disabled => {
                let (fill, idle, label, idle_label) = disabled;
                let (fill, label, border) =
                    disabled_button(fill, idle, label, idle_label, outline, opacity);
                (Some(Background::Color(fill)), label, border)
            }
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
/// the border, which is the primary variant's own where the 0.8.0 model
/// states one (windows-11 lays its stroke inside the box, the plain button's
/// outside). The model states no hovered and no pressed primary fill, so
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
    button_primary_from(&ButtonSource::of(resolved))
}

/// [`button_primary`] of what it reads.
pub(crate) fn button_primary_from(
    b: &ButtonSource,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
    use iced_widget::button::{Status, Style};

    let b = &b.primary();
    let class = class_button(
        b,
        b.primary_background,
        b.primary_label,
        iced_widget::button::primary,
    );

    let primary = iced_core::theme::palette::Primary::generate(
        b.primary_background,
        b.window_background,
        b.window_text,
    );
    let hovered = primary.strong.color;
    let pressed = primary.base.color;
    let label = b.primary_label;
    let source = *b;
    let outline_at = move |status| source.outline_at(status);

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
            border: outline_at(status).over(background),
            ..style
        }
    }
}

/// The platform's own default button, the one Enter activates in a dialog, for
/// `button(..).style(..)`.
///
/// Where the theme states `button.default_button_background` (KDE's Breeze
/// gives its default button a look of its own), the button is that fill,
/// lettered in `button.default_button_text_color`, else `button.font.color`,
/// and framed in `button.default_button_border_color`, else
/// `button.border.color`; under the pointer the same fill and label, framed in
/// `button.hover_border_color` where the theme states it, else that frame;
/// pressed, [`button`]'s fill and label in that hovered frame; disabled,
/// [`button`]'s. Where the theme states none, it is [`button_primary`] in every
/// state: the default button is the primary one on macOS and Windows
/// (`docs/platform-facts.md` §2.3).
#[must_use = "this returns the style function; it does not apply it"]
pub fn button_default(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
    button_default_from(&ButtonSource::of(resolved))
}

/// [`button_default`] of what it reads.
pub(crate) fn button_default_from(
    b: &ButtonSource,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
    use iced_widget::button::{Status, Style};

    let plain = button_from(b);
    let primary = button_primary_from(b);

    let fill = b.default_background;
    let label = b.default_label.unwrap_or(b.label);
    let border = b.border.with(b.default_border_color);
    let hovered_border = border.with(b.hover_border_color);
    let pressed_border = border.with(b.pressed_border_color);

    move |theme, status| {
        let Some(fill) = fill else {
            return primary(theme, status);
        };
        let rest = plain(theme, Status::Active);
        match status {
            Status::Active => Style {
                background: Some(Background::Color(fill)),
                text_color: label,
                border: border.over(fill),
                ..rest
            },
            Status::Hovered => Style {
                background: Some(Background::Color(fill)),
                text_color: label,
                border: hovered_border.over(fill),
                ..rest
            },
            Status::Pressed => {
                let style = plain(theme, Status::Pressed);
                Style {
                    border: pressed_border.over(painted(style.background)),
                    ..style
                }
            }
            Status::Disabled => plain(theme, Status::Disabled),
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
        &ButtonSource::of(resolved),
        to_color(resolved.defaults.danger_color),
        to_color(resolved.defaults.danger_text_color),
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
        &ButtonSource::of(resolved),
        to_color(resolved.defaults.success_color),
        to_color(resolved.defaults.success_text_color),
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
        &ButtonSource::of(resolved),
        to_color(resolved.defaults.warning_color),
        to_color(resolved.defaults.warning_text_color),
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
    button_link_from(&LinkSource::of(resolved))
}

/// What [`button_link`] reads, of either model. A hover or pressed fill or
/// label `None` is iced's own text button's for the status (spec §2,
/// `from_state`).
#[derive(Clone, Copy, Debug)]
pub(crate) struct LinkSource {
    pub background: Color,
    /// The hover layer over the idle fill.
    pub hover_background: Option<Layer>,
    /// The fill of a pressed link, as painted: the 0.7.2 model's idle one.
    pub pressed_background: Option<Color>,
    pub label: Color,
    pub hover_label: Option<Color>,
    pub active_label: Option<Color>,
    pub disabled_label: Color,
    pub disabled_opacity: f32,
}

impl LinkSource {
    /// The 0.7.2 model's reads.
    pub(crate) fn of(resolved: &ResolvedTheme) -> Self {
        let l = &resolved.link;
        Self {
            background: to_color(l.background_color),
            hover_background: Some(Layer::Color(to_color(l.hover_background))),
            pressed_background: Some(to_color(l.background_color)),
            label: to_color(l.font.color),
            hover_label: Some(to_color(l.hover_text_color)),
            active_label: Some(to_color(l.active_text_color)),
            disabled_label: to_color(l.disabled_text_color),
            // `LinkTheme` states no `disabled_opacity`: the defaults' fades it.
            disabled_opacity: resolved.defaults.disabled_opacity,
        }
    }

    /// The state options `listed` names unset, for the parity test's
    /// expected values; a link has no stroke.
    #[cfg(test)]
    pub(crate) fn listed(self, listed: crate::next::parity::Listed) -> Self {
        let s = listed.states;
        Self {
            hover_background: self.hover_background.filter(|_| !s.hover_fill),
            pressed_background: self.pressed_background.filter(|_| !s.pressed_fill),
            hover_label: self.hover_label.filter(|_| !s.hover_label),
            active_label: self.active_label.filter(|_| !s.pressed_label),
            ..self
        }
    }
}

/// [`button_link`] of what it reads.
pub(crate) fn button_link_from(
    l: &LinkSource,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
    use iced_widget::button::{Status, Style};

    let idle = Some(Background::Color(l.background));
    let hovered = l
        .hover_background
        .map(|hover| Background::Color(hover.over(l.background)));
    let pressed = l.pressed_background.map(Background::Color);

    let label = l.label;
    let hovered_label = l.hover_label;
    let pressed_label = l.active_label;
    let disabled_label = faded(l.disabled_label, l.disabled_opacity);

    move |theme, status| {
        let iced = iced_widget::button::text(theme, status);
        // A hover or a press the theme states no fill for is iced's own text
        // button's for the status, fill and label, with a label the theme
        // states for it laid over; a stated fill is lettered in the label
        // the theme states for the state, else the label at rest.
        let own = |fill: Option<Background>, text: Option<Color>| match fill {
            Some(fill) => (Some(fill), text.unwrap_or(label)),
            None => (iced.background, text.unwrap_or(iced.text_color)),
        };
        let (background, text_color) = match status {
            Status::Active => (idle, label),
            Status::Hovered => own(hovered, hovered_label),
            Status::Pressed => own(pressed, pressed_label),
            Status::Disabled => (idle, disabled_label),
        };
        Style {
            background,
            text_color,
            border: iced.border,
            shadow: iced.shadow,
            snap: iced.snap,
        }
    }
}

/// The values [`text_input`] and [`text_editor`] each emit, of the field's
/// [`InputSource`]: the fill, the border and the value of a field at rest,
/// hovered, focused and disabled, and the two colours every state shares.
/// The 0.7.2 model states one `InputTheme` for both fields, the multi-line
/// one's border at rest being its own `text_area.border`; the 0.8.0 model
/// gives the editor every colour of its own, from `text_area`.
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
    fn new(i: &InputSource) -> Self {
        let idle = i.background;
        // A translucent disabled fill replaces the idle one and lets the
        // window through, so it is emitted as given.
        let disabled = i.disabled_background.unwrap_or(i.background);
        let text = i.text;
        let opacity = i.disabled_opacity;

        // Both border soft options fall back to the border's own color: the
        // platform saying the field does not change outline in that state.
        let border = i.border;
        let outlined = |line: Option<Line>| border.with(line).over(idle);
        // An inset focus ring (offset -width: libadwaita's `outline-offset:
        // -$width`, `docs/platform-facts.md` §2.1.5) lies inside the field's
        // edge, where iced paints a border (`renderer::Quad { bounds, border,
        // .. }`, iced_widget 0.14.2 `src/text_input.rs:473-478`), so it is
        // drawn exactly as a border that wide. iced's quad puts the border in
        // place of the fill under it rather than over it (`mix(color,
        // border_color, ..)`, iced_wgpu 0.14.0 `src/shader/quad/solid.wgsl:78-83`),
        // while the platform lays the ring over the field's fill, so a
        // translucent ring is composited over the fill here. A ring that lies
        // outside the edge (offset 0 or more: Windows' +1), straddles it
        // (macOS's -1 with width 3, KDE's -1 with width 2) or lies inside it
        // with a gap (an offset below minus the width) is not a border iced
        // can draw; there a focused field recolours its rest line, and
        // `crate::focus_ring` draws the ring round the field it wraps.
        let focus_color = i.focus_border_color.map_or(i.focus_ring_color, |l| l.color);
        let focused = if i.focus_ring_width > 0.0 && i.focus_ring_offset == -i.focus_ring_width {
            Border {
                color: composite_over(focus_color, idle),
                width: border.width.max(i.focus_ring_width),
                radius: border.radius,
            }
        } else {
            outlined(i.focus_border_color)
        };
        // A field the theme states no opacity for is faded as iced's own
        // disabled field is: not at all (iced_widget 0.14.2
        // `src/text_input.rs`, `default`, `Status::Disabled`).
        let fade = |color: Color| opacity.map_or(color, |opacity| faded(color, opacity));
        let disabled_border = border.over(disabled);

        Self {
            active: (idle, border.over(idle), text),
            hovered: (idle, outlined(i.hover_border_color), text),
            focused: (idle, focused, text),
            disabled: (
                fade(disabled),
                Border {
                    color: fade(disabled_border.color),
                    ..disabled_border
                },
                fade(i.disabled_text),
            ),
            placeholder: i.placeholder,
            selection: i.selection,
        }
    }
}

/// What the field styles read, of either model: the field's colours, its
/// border, and the focus ring's colour, width and offset.
#[derive(Clone, Copy, Debug)]
pub(crate) struct InputSource {
    pub background: Color,
    pub disabled_background: Option<Color>,
    pub text: Color,
    pub disabled_text: Color,
    pub disabled_opacity: Option<f32>,
    pub border: Outline,
    pub hover_border_color: Option<Line>,
    pub focus_border_color: Option<Line>,
    pub focus_ring_color: Color,
    pub focus_ring_width: f32,
    pub focus_ring_offset: f32,
    pub placeholder: Color,
    pub selection: Color,
}

impl InputSource {
    /// The 0.7.2 model's reads: `input.*` with `border` as the field's
    /// border.
    pub(crate) fn of(
        i: &ResolvedInputTheme,
        border: &ResolvedWidgetBorder,
        d: &ResolvedDefaults,
    ) -> Self {
        Self {
            background: to_color(i.background_color),
            disabled_background: i.disabled_background.map(to_color),
            text: to_color(i.font.color),
            disabled_text: to_color(i.disabled_text_color),
            disabled_opacity: Some(i.disabled_opacity),
            border: Outline::of(border),
            hover_border_color: i.hover_border_color.map(|c| Line::outside(to_color(c))),
            focus_border_color: i.focus_border_color.map(|c| Line::outside(to_color(c))),
            focus_ring_color: to_color(d.focus_ring_color),
            focus_ring_width: d.focus_ring_width,
            focus_ring_offset: d.focus_ring_offset,
            placeholder: to_color(i.placeholder_color),
            selection: to_color(i.selection_background),
        }
    }

    /// Every stroke where it lies `align`, for the parity test's expected
    /// values.
    #[cfg(test)]
    pub(crate) fn aligned(self, align: LineAlign) -> Self {
        let line = |l: Option<Line>| l.map(|l| l.aligned(align));
        Self {
            border: self.border.aligned(align),
            hover_border_color: line(self.hover_border_color),
            focus_border_color: line(self.focus_border_color),
            ..self
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
    text_input_from(&InputSource::of(
        &resolved.input,
        &resolved.input.border,
        &resolved.defaults,
    ))
}

/// [`text_input`] of what it reads.
pub(crate) fn text_input_from(
    s: &InputSource,
) -> impl Fn(&Theme, iced_widget::text_input::Status) -> iced_widget::text_input::Style + Clone + use<>
{
    use iced_widget::text_input::{Status, Style};

    let input = InputColors::new(s);

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
    text_editor_from(&InputSource::of(
        &resolved.input,
        &resolved.text_area.border,
        &resolved.defaults,
    ))
}

/// [`text_editor`] of what it reads.
pub(crate) fn text_editor_from(
    s: &InputSource,
) -> impl Fn(&Theme, iced_widget::text_editor::Status) -> iced_widget::text_editor::Style + Clone + use<>
{
    use iced_widget::text_editor::{Status, Style};

    let input = InputColors::new(s);

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
/// unchecked box is the box at rest. The model states no hover fill for a
/// checked one, and a state the platform does not state has no distinct
/// appearance, so a hovered checked box keeps the checked box's fill. Under
/// the pointer the border is `.hover_border_color` where the theme states
/// one, checked or not; `None` keeps the border at rest. `.disabled_background`
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

/// The fills, the border strokes and the label a box, or a radio, that is
/// not disabled shows, of either model: the 0.7.2 model states one
/// `CheckboxTheme` for both controls. The four hover values are `None` where
/// the theme states nothing for the hovered box, which is then iced's own
/// (spec §2, `from_state`); the 0.7.2 model's unset soft options are the
/// platform saying the box looks as it does at rest, and its reads state
/// that.
#[derive(Clone, Copy, Debug)]
pub(crate) struct CheckFills {
    pub checked: Color,
    /// The unchecked fill: `checkbox.unchecked_background`, or the box's own
    /// fill where that is unset.
    pub unchecked: Color,
    /// The fill of an unchecked box under the pointer: its hover layer over
    /// the unchecked fill.
    pub hover_fill: Option<Color>,
    pub checked_border: Line,
    /// `checkbox.unchecked_border_color`, or the box's own border colour
    /// where that is unset.
    pub unchecked_border: Line,
    /// The border of an unchecked box under the pointer.
    pub hover_border: Option<Line>,
    /// The border of a checked box under the pointer.
    pub checked_hover_border: Option<Line>,
    /// The label of a box under the pointer.
    pub hover_label: Option<Color>,
}

impl CheckFills {
    /// The 0.7.2 model's reads.
    pub(crate) fn of(c: &ResolvedCheckboxTheme) -> Self {
        // The four soft options are the platform saying the box looks no
        // different: unchecked, it has the checkbox's own fill and border;
        // under the pointer, the fill and the border it has at rest.
        let line = |c: Rgba| Line::outside(to_color(c));
        let unchecked = to_color(c.unchecked_background.unwrap_or(c.background_color));
        let unchecked_border = line(c.unchecked_border_color.unwrap_or(c.border.color));
        let checked_border = line(c.border.color);
        Self {
            checked: to_color(c.checked_background),
            unchecked,
            hover_fill: Some(c.hover_background.map_or(unchecked, |hover| {
                composite_over(to_color(hover), unchecked)
            })),
            checked_border,
            unchecked_border,
            hover_border: Some(c.hover_border_color.map_or(unchecked_border, line)),
            checked_hover_border: Some(c.hover_border_color.map_or(checked_border, line)),
            hover_label: Some(to_color(c.font.color)),
        }
    }

    /// Every stroke where it lies as `listed` says, and the hover options it
    /// names unset, for the parity test's expected values.
    #[cfg(test)]
    pub(crate) fn listed(self, listed: crate::next::parity::Listed) -> Self {
        let (align, s) = (listed.align, listed.states);
        let line = |l: Option<Line>| l.map(|l| l.aligned(align)).filter(|_| !s.hover_stroke);
        Self {
            checked_border: self.checked_border.aligned(align),
            unchecked_border: self.unchecked_border.aligned(align),
            hover_fill: self.hover_fill.filter(|_| !s.hover_fill),
            hover_border: line(self.hover_border),
            checked_hover_border: line(self.checked_hover_border),
            hover_label: self.hover_label.filter(|_| !s.hover_label),
            ..self
        }
    }

    /// The label of a box under the pointer: the hover label the theme
    /// states; else, where the theme states no hovered fill for an
    /// unchecked box, `iced`'s, iced's own for the status, and the label at
    /// rest where it states the fill (a checked box keeps its checked one).
    fn hover_label_at(
        &self,
        is_checked: bool,
        label: Color,
        iced: impl Fn() -> Option<Color>,
    ) -> Option<Color> {
        match (self.hover_label, is_checked || self.hover_fill.is_some()) {
            (Some(text), _) => Some(text),
            (None, true) => Some(label),
            (None, false) => iced(),
        }
    }

    /// The fill and the border colour of a box that is checked or not, under
    /// the pointer or not, the stroke laid over that fill ([`Line::over`]).
    /// Under the pointer an unchecked box takes the hover fill and a checked
    /// one keeps its own, and either takes the hover border; each the theme
    /// states nothing for is `iced`'s, iced's own fill and border colour for
    /// the status.
    fn at(
        &self,
        is_checked: bool,
        is_hovered: bool,
        iced: impl Fn() -> (Color, Color),
    ) -> (Color, Color) {
        let (fill, line) = match (is_checked, is_hovered) {
            (true, false) => (Some(self.checked), Some(self.checked_border)),
            (true, true) => (Some(self.checked), self.checked_hover_border),
            (false, false) => (Some(self.unchecked), Some(self.unchecked_border)),
            (false, true) => (self.hover_fill, self.hover_border),
        };
        let fill = fill.unwrap_or_else(|| iced().0);
        (fill, line.map_or_else(|| iced().1, |line| line.over(fill)))
    }
}

/// What the check box style reads from the theme, of either model.
#[derive(Clone, Copy, Debug)]
pub(crate) struct CheckboxSource {
    pub fills: CheckFills,
    /// The disabled fill of an unchecked box and of a checked one, soft
    /// options.
    pub disabled_background: Option<Color>,
    pub checked_disabled_background: Option<Color>,
    pub disabled_opacity: Option<f32>,
    /// The check mark.
    pub mark: Color,
    pub label: Color,
    pub disabled_label: Color,
    pub border_width: f32,
    pub border_radius: f32,
}

impl CheckboxSource {
    /// The 0.7.2 model's reads.
    pub(crate) fn of(resolved: &ResolvedTheme) -> Self {
        let c = &resolved.checkbox;
        Self {
            fills: CheckFills::of(c),
            disabled_background: c.disabled_background.map(to_color),
            checked_disabled_background: c.disabled_background.map(to_color),
            disabled_opacity: Some(c.disabled_opacity),
            mark: to_color(c.indicator_color),
            label: to_color(c.font.color),
            disabled_label: to_color(c.disabled_text_color),
            border_width: c.border.line_width,
            border_radius: c.border.corner_radius,
        }
    }

    /// [`CheckFills::listed`] of the box's fills, for the parity test's
    /// expected values.
    #[cfg(test)]
    pub(crate) fn listed(self, listed: crate::next::parity::Listed) -> Self {
        Self {
            fills: self.fills.listed(listed),
            ..self
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
    checkbox_from(&CheckboxSource::of(resolved), backdrop)
}

/// [`checkbox_over`] of what it reads.
pub(crate) fn checkbox_from(
    c: &CheckboxSource,
    backdrop: Color,
) -> impl Fn(&Theme, iced_widget::checkbox::Status) -> iced_widget::checkbox::Style + Clone + use<>
{
    use iced_widget::checkbox::{Status, Style};

    let ground = backdrop;

    let fills = c.fills;
    // A translucent disabled fill replaces the idle one, so it is as given. With
    // none stated the platform dims by opacity alone
    // (`docs/platform-facts.md` §2.1.6): the box keeps its enabled look.
    let disabled = c.disabled_background;
    let checked_disabled = c.checked_disabled_background;
    let opacity = c.disabled_opacity;

    let mark = c.mark;
    let label = c.label;
    let disabled_label = c.disabled_label;

    let border_width = c.border_width;
    let border_radius = Radius::new(c.border_radius);

    move |theme, status| {
        let own = || iced_widget::checkbox::primary(theme, status);
        let iced = || {
            let own = own();
            let fill = match own.background {
                Background::Color(c) => c,
                Background::Gradient(_) => Color::TRANSPARENT,
            };
            (fill, own.border.color)
        };
        // A box the theme states no opacity for is drawn in the colours it
        // states, not faded.
        let fade = |under: Color, color: Color| {
            opacity.map_or(color, |opacity| faded_as_one(ground, under, color, opacity))
        };
        let (background, border_color, text_color, icon_color) = match status {
            Status::Active { is_checked } => {
                let (fill, border) = fills.at(is_checked, false, iced);
                (fill, border, Some(label), mark)
            }
            Status::Hovered { is_checked } => {
                let (fill, border) = fills.at(is_checked, true, iced);
                let text = fills.hover_label_at(is_checked, label, || own().text_color);
                (fill, border, text, mark)
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
                let (at_rest, line, disabled) = if is_checked {
                    (fills.checked, fills.checked_border, checked_disabled)
                } else {
                    (fills.unchecked, fills.unchecked_border, disabled)
                };
                let (background, mark) = match disabled {
                    Some(fill) => (fill, disabled_label),
                    None => (at_rest, mark),
                };
                let fill = composite_over(background, ground);
                (
                    fade(ground, background),
                    fade(fill, line.over(background)),
                    Some(fade(ground, disabled_label)),
                    fade(fill, mark),
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
            text_color,
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
/// the hover fill of the *unselected* circle only, and
/// `checkbox.hover_border_color`, which frames a hovered circle, selected or
/// not, where the theme states it: a hovered selected radio is the selected
/// radio in that border, and where the theme leaves both unset a hovered
/// radio is the circle at rest. `radio::Status` has no disabled variant
/// (`radio.rs:474-487`), so the model's disabled fields have no receiver
/// here.
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
    let c = &resolved.checkbox;
    radio_from(
        CheckFills::of(c),
        to_color(c.indicator_color),
        to_color(c.font.color),
        c.border.line_width,
    )
}

/// [`radio`] of what it reads: the circle's fills and borders, the dot, the
/// label and the border's width.
pub(crate) fn radio_from(
    fills: CheckFills,
    dot: Color,
    label: Color,
    border_width: f32,
) -> impl Fn(&Theme, iced_widget::radio::Status) -> iced_widget::radio::Style + Clone + use<> {
    use iced_widget::radio::{Status, Style};

    move |theme, status| {
        let own = || iced_widget::radio::default(theme, status);
        let iced = || {
            let own = own();
            let fill = match own.background {
                Background::Color(c) => c,
                Background::Gradient(_) => Color::TRANSPARENT,
            };
            (fill, own.border_color)
        };
        let (background, border_color, text_color) = match status {
            Status::Active { is_selected } => {
                let (fill, border) = fills.at(is_selected, false, iced);
                (fill, border, Some(label))
            }
            Status::Hovered { is_selected } => {
                let (fill, border) = fills.at(is_selected, true, iced);
                let text = fills.hover_label_at(is_selected, label, || own().text_color);
                (fill, border, text)
            }
        };
        Style {
            background: Background::Color(background),
            dot_color: dot,
            border_width,
            border_color,
            text_color,
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
/// (where it states nothing for the hovered switch, the 0.8.0 model's track
/// is iced's own for the status, spec §2 `from_state`; 0.7.2's unset hover
/// is the track at rest) and `.disabled_*` replacing it, as given. The thumb is
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
/// quads, in place of the fill under them, so each outline is emitted over
/// the fill under it in each state as it lies ([`Line::over`]): the 0.7.2
/// model's track outline inside the track, as the platforms stroke it, and
/// its thumb outline as given. `text_color` has no native source and comes from
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
    toggler_from(&SwitchSource::of(resolved), backdrop)
}

/// What the switch style reads from the theme, of either model: each
/// field's 0.7.2 name, soft options as `None` where unset.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SwitchSource {
    /// `None` where the theme states no disabled opacity: a disabled switch
    /// is then drawn in the colours it states, not faded.
    pub disabled_opacity: Option<f32>,
    pub checked_background: Color,
    pub unchecked_background: Color,
    /// The track of a switch under the pointer, on and off: its hover layer
    /// over the track at rest. `None` where the theme states nothing for the
    /// hovered switch, which is then iced's own (spec §2, `from_state`); the
    /// 0.7.2 model's unset hover layers are the platform saying the track
    /// looks as it does at rest, and its reads state that.
    pub hovered_checked_background: Option<Color>,
    pub hovered_unchecked_background: Option<Color>,
    pub disabled_checked_background: Option<Color>,
    pub disabled_unchecked_background: Option<Color>,
    pub thumb_background: Color,
    /// The off thumb: `switch.unchecked_thumb_background`, or the on one
    /// where that is unset.
    pub unchecked_thumb_background: Color,
    /// The thumb of a disabled switch, on and off.
    pub disabled_thumb_color: Option<Color>,
    pub disabled_unchecked_thumb_color: Option<Color>,
    /// The track's outline on and off, as stated (width, stroke), where the
    /// theme states both.
    pub checked_track_border: Option<(f32, Line)>,
    pub unchecked_track_border: Option<(f32, Line)>,
    /// The disabled track's outline, where stated; `None` keeps the one at
    /// rest.
    pub disabled_checked_track_border: Option<(f32, Line)>,
    pub disabled_unchecked_track_border: Option<(f32, Line)>,
    pub thumb_border: Option<(f32, Line)>,
    pub track_radius: Option<f32>,
    pub track_height: Option<f32>,
    pub thumb_diameter: Option<f32>,
    /// The off thumb's diameter: `switch.unchecked_thumb_diameter`, or the
    /// on one's where that is unset.
    pub unchecked_thumb_diameter: Option<f32>,
}

impl SwitchSource {
    /// The 0.7.2 model's reads. The 0.7.2 connector laid the track's outline
    /// over the track's fill and handed the thumb's to iced as given: the
    /// one is read as a stroke inside the track, the other as one outside
    /// the thumb.
    pub(crate) fn of(resolved: &ResolvedTheme) -> Self {
        let s = &resolved.switch;
        let outline = |colour: Option<Rgba>, width: Option<f32>, align: LineAlign| {
            colour.zip(width).map(|(colour, width)| {
                (
                    width,
                    Line {
                        color: to_color(colour),
                        align,
                    },
                )
            })
        };
        let track = |colour| outline(colour, s.track_border_line_width, LineAlign::Inside);
        let unchecked_thumb = s.unchecked_thumb_background.unwrap_or(s.thumb_background);
        let checked = to_color(s.checked_background);
        let unchecked = to_color(s.unchecked_background);
        let hovered = |layer: Option<Rgba>, track: Color| {
            Some(layer.map_or(track, |layer| composite_over(to_color(layer), track)))
        };
        Self {
            disabled_opacity: Some(s.disabled_opacity),
            checked_background: checked,
            unchecked_background: unchecked,
            hovered_checked_background: hovered(s.hover_checked_background, checked),
            hovered_unchecked_background: hovered(s.hover_unchecked_background, unchecked),
            disabled_checked_background: s.disabled_checked_background.map(to_color),
            disabled_unchecked_background: s.disabled_unchecked_background.map(to_color),
            thumb_background: to_color(s.thumb_background),
            unchecked_thumb_background: to_color(unchecked_thumb),
            disabled_thumb_color: s.disabled_thumb_color.map(to_color),
            disabled_unchecked_thumb_color: s.disabled_thumb_color.map(to_color),
            checked_track_border: track(s.checked_track_border_color),
            unchecked_track_border: track(s.unchecked_track_border_color),
            disabled_checked_track_border: track(s.disabled_checked_track_border_color),
            disabled_unchecked_track_border: track(s.disabled_unchecked_track_border_color),
            thumb_border: outline(
                s.thumb_border_color,
                s.thumb_border_line_width,
                LineAlign::Outside,
            ),
            track_radius: Some(s.track_radius),
            track_height: Some(s.track_height),
            thumb_diameter: Some(s.thumb_diameter),
            unchecked_thumb_diameter: Some(s.unchecked_thumb_diameter.unwrap_or(s.thumb_diameter)),
        }
    }

    /// Every stroke where it lies as `listed` says, and the hover options it
    /// names unset, for the parity test's expected values.
    #[cfg(test)]
    pub(crate) fn listed(self, listed: crate::next::parity::Listed) -> Self {
        let (align, s) = (listed.align, listed.states);
        let edge = |e: Option<(f32, Line)>| e.map(|(width, line)| (width, line.aligned(align)));
        Self {
            hovered_checked_background: self.hovered_checked_background.filter(|_| !s.hover_fill),
            hovered_unchecked_background: self
                .hovered_unchecked_background
                .filter(|_| !s.hover_fill),
            checked_track_border: edge(self.checked_track_border),
            unchecked_track_border: edge(self.unchecked_track_border),
            disabled_checked_track_border: edge(self.disabled_checked_track_border),
            disabled_unchecked_track_border: edge(self.disabled_unchecked_track_border),
            thumb_border: edge(self.thumb_border),
            ..self
        }
    }
}

/// [`toggler_over`] of what it reads.
pub(crate) fn toggler_from(
    s: &SwitchSource,
    backdrop: Color,
) -> impl Fn(&Theme, iced_widget::toggler::Status) -> iced_widget::toggler::Style + Clone + use<> {
    use iced_widget::toggler::{Status, Style};

    let ground = backdrop;
    let opacity = s.disabled_opacity;

    let checked = s.checked_background;
    let unchecked = s.unchecked_background;
    // Where the theme states nothing for the hovered switch, its track is
    // iced's own for the status.
    let hovered_checked = s.hovered_checked_background;
    let hovered_unchecked = s.hovered_unchecked_background;
    let disabled_checked = s.disabled_checked_background.unwrap_or(checked);
    let disabled_unchecked = s.disabled_unchecked_background.unwrap_or(unchecked);

    let thumb = s.thumb_background;
    // The off thumb: a soft option copying the on one.
    let unchecked_thumb = s.unchecked_thumb_background;
    let disabled_thumb = s.disabled_thumb_color.unwrap_or(thumb);
    let disabled_unchecked_thumb = s.disabled_unchecked_thumb_color.unwrap_or(unchecked_thumb);

    // A disabled switch is faded by `switch.disabled_opacity` as one widget
    // over the ground: the thumb on its track (see `faded_as_one`). A switch
    // the theme states no opacity for is drawn in the colours it states.
    let fade = move |under: Color, color: Color| {
        opacity.map_or(color, |opacity| faded_as_one(ground, under, color, opacity))
    };
    let disabled_as_one = |track: Color, thumb: Color| {
        (
            fade(ground, track),
            fade(composite_over(track, ground), thumb),
        )
    };
    let disabled_on = disabled_as_one(disabled_checked, disabled_thumb);
    let disabled_off = disabled_as_one(disabled_unchecked, disabled_unchecked_thumb);

    // The outlines, where the theme states a colour and a width: the track's
    // per state over that state's fill, a disabled one faded as one with it
    // over the ground, and the thumb's over the thumb (docs/platform-facts.md
    // §2.21). iced's quad paints its border in place of the fill under it,
    // so each stroke is handed to iced as it lies over the fill painted
    // under it ([`Line::over`]).
    let over =
        |edge: Option<(f32, Line)>, fill: Color| edge.map(|(width, line)| (width, line.over(fill)));
    let rest_edge_on = s.checked_track_border;
    let rest_edge_off = s.unchecked_track_border;
    let disabled_edge = |edge: Option<(f32, Line)>, rest: Option<(f32, Line)>, track: Color| {
        over(edge.or(rest), track).map(|(width, edge)| (width, fade(ground, edge)))
    };
    let disabled_edge_on = disabled_edge(
        s.disabled_checked_track_border,
        rest_edge_on,
        disabled_checked,
    );
    let disabled_edge_off = disabled_edge(
        s.disabled_unchecked_track_border,
        rest_edge_off,
        disabled_unchecked,
    );
    let thumb_edge = s.thumb_border;
    let disabled_thumb_edge = |track: Color, thumb: Color| {
        over(thumb_edge, thumb)
            .map(|(width, edge)| (width, fade(composite_over(track, ground), edge)))
    };
    let disabled_thumb_edge_on = disabled_thumb_edge(disabled_checked, disabled_thumb);
    let disabled_thumb_edge_off = disabled_thumb_edge(disabled_unchecked, disabled_unchecked_thumb);

    let track_radius = s.track_radius.map(Radius::new);
    // Guarded so that a track with no height, or a thumb taller than its
    // track, falls back to iced's own ratio rather than dividing by zero or
    // insetting the thumb by a negative length; so does a theme that states
    // neither. The off thumb is `unchecked_thumb_diameter` across where the
    // theme states one.
    let track_height = s.track_height;
    let ratio = |thumb: Option<f32>| {
        let (height, thumb) = track_height.zip(thumb)?;
        if height > 0.0 && thumb >= 0.0 && thumb <= height {
            Some((height - thumb) / (2.0 * height))
        } else {
            None
        }
    };
    let padding_ratio = ratio(s.thumb_diameter);
    let unchecked_padding_ratio = ratio(s.unchecked_thumb_diameter);

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
                let (hovered, thumb) = if is_toggled {
                    (hovered_checked, thumb)
                } else {
                    (hovered_unchecked, unchecked_thumb)
                };
                (
                    hovered.unwrap_or_else(|| painted(Some(iced.background))),
                    thumb,
                )
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
            Status::Active { is_toggled } | Status::Hovered { is_toggled } => {
                let rest = if is_toggled {
                    rest_edge_on
                } else {
                    rest_edge_off
                };
                (over(rest, background), over(thumb_edge, foreground))
            }
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
            border_radius: track_radius.or(iced.border_radius),
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
/// border is `combo_box.border.*`, its colour `.hover_border_color` under the
/// pointer where the theme states one, the stroke over the fill it lies on
/// ([`Line::over`]). Where the 0.8.0 model states nothing for the hovered
/// field, its fill and stroke colour are iced's own for the status (spec §2,
/// `from_state`). The placeholder is
/// `input.placeholder_color`: the model states it once, for every field that
/// has one.
///
/// An open field (`pick_list::Status::Opened`) is `combo_box.open_background`
/// layered over the idle fill, as the hover fill is, and outlined in
/// `.open_border_color`, whatever the pointer does: Breeze draws an open
/// combo box as a pressed button, framed in its focus colour (platform-facts
/// §2.24). Where the theme leaves either unset, that part follows the
/// pointer, as `Opened` carries whether it is on the field
/// (`pick_list.rs:838-849`): the hovered fill or border on it, the idle one
/// off it. The border's width and radius are `combo_box.border`'s in every
/// status. `handle_color` has no native
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
    pick_list_from(&PickListSource::of(resolved))
}

/// What the drop-down field's style reads from the theme, of either model:
/// each field's 0.7.2 name, soft options as `None` where unset.
#[derive(Clone, Copy, Debug)]
pub(crate) struct PickListSource {
    pub background: Color,
    /// The field under the pointer: its hover layer over the idle fill.
    /// `None` where the theme states nothing for the hovered field, which is
    /// then iced's own (spec §2, `from_state`); the 0.7.2 model's unset hover
    /// layer is the platform saying the field looks as it does at rest, and
    /// its reads state that.
    pub hovered_background: Option<Color>,
    pub label: Color,
    pub placeholder: Color,
    /// The open field's layer over the idle fill; `None` follows the
    /// pointer.
    pub open_background: Option<Layer>,
    /// The open field's layer under the pointer: 0.7.2's
    /// `open_background`, which the field keeps under the pointer.
    pub open_hover_background: Option<Layer>,
    pub border: Outline,
    /// The border under the pointer, `None` as the hover fill.
    pub hover_border_color: Option<Line>,
    /// The open field's border; `None` follows the pointer.
    pub open_border_color: Option<Line>,
    /// The open field's border under the pointer: 0.7.2's
    /// `open_border_color`.
    pub open_hover_border_color: Option<Line>,
}

impl PickListSource {
    /// The 0.7.2 model's reads.
    pub(crate) fn of(resolved: &ResolvedTheme) -> Self {
        let c = &resolved.combo_box;
        let idle = to_color(c.background_color);
        let border = Outline::of(&c.border);
        let line = |c: Rgba| Line::outside(to_color(c));
        Self {
            background: idle,
            hovered_background: Some(
                c.hover_background
                    .map_or(idle, |hover| composite_over(to_color(hover), idle)),
            ),
            label: to_color(c.font.color),
            placeholder: to_color(resolved.input.placeholder_color),
            open_background: c.open_background.map(|c| Layer::Color(to_color(c))),
            open_hover_background: c.open_background.map(|c| Layer::Color(to_color(c))),
            border,
            hover_border_color: Some(c.hover_border_color.map_or(border.line, line)),
            open_border_color: c.open_border_color.map(line),
            open_hover_border_color: c.open_border_color.map(line),
        }
    }

    /// Every stroke where it lies as `listed` says, and the hover options it
    /// names unset, for the parity test's expected values.
    #[cfg(test)]
    pub(crate) fn listed(self, listed: crate::next::parity::Listed) -> Self {
        let (align, s) = (listed.align, listed.states);
        let line = |l: Option<Line>| l.map(|l| l.aligned(align));
        Self {
            hovered_background: self.hovered_background.filter(|_| !s.hover_fill),
            border: self.border.aligned(align),
            hover_border_color: line(self.hover_border_color).filter(|_| !s.hover_stroke),
            open_border_color: line(self.open_border_color),
            open_hover_border_color: line(self.open_hover_border_color),
            ..self
        }
    }
}

/// [`pick_list`] of what it reads.
pub(crate) fn pick_list_from(
    c: &PickListSource,
) -> impl Fn(&Theme, iced_widget::pick_list::Status) -> iced_widget::pick_list::Style + Clone + use<>
{
    use iced_widget::pick_list::{Status, Style};

    let idle = c.background;
    let hovered = c.hovered_background;

    let label = c.label;
    let placeholder = c.placeholder;

    let open = c.open_background.map(|fill| fill.over(idle));
    let open_hovered = c.open_hover_background.map(|fill| fill.over(idle));

    let border = c.border;
    let hovered_border = c.hover_border_color;
    let open_border = c.open_border_color;
    let open_hovered_border = c.open_hover_border_color;

    move |theme, status| {
        let iced = iced_widget::pick_list::default(theme, status);
        // The fill and the stroke the pointer gives; an open field takes its
        // own where the theme states them. Each the theme states nothing for
        // is iced's own for the status.
        let (fill, line) = match status {
            Status::Active | Status::Opened { is_hovered: false } => {
                (Some(idle), Some(border.line))
            }
            Status::Hovered | Status::Opened { is_hovered: true } => (hovered, hovered_border),
        };
        let (fill, line) = match status {
            Status::Opened { is_hovered: false } => (open.or(fill), open_border.or(line)),
            Status::Opened { is_hovered: true } => {
                (open_hovered.or(fill), open_hovered_border.or(line))
            }
            Status::Active | Status::Hovered => (fill, line),
        };
        let background = fill.unwrap_or_else(|| painted(Some(iced.background)));
        let color = line.map_or(iced.border.color, |line| line.over(background));
        Style {
            text_color: label,
            placeholder_color: placeholder,
            handle_color: iced.handle_color,
            background: Background::Color(background),
            border: Border {
                color,
                ..border.over(background)
            },
        }
    }
}

/// The platform's own drop-down, for `.menu_style(..)` on a `PickList` or a
/// `ComboBox`.
///
/// Replaces `iced_widget::overlay::menu::default` (`overlay/menu.rs:646`),
/// which paints the list in the palette's weak background and its selected row
/// in the primary family. A menu has no `Status`, so the closure takes the
/// theme alone.
///
/// A drop-down is framed as a popup, in `popover.border`'s colour and width
/// (`docs/platform-facts.md` §2.16, "Popover / Dropdown"). Its panel and rows are the
/// menu's: the panel `menu.background_color`, the label `menu.font.color`,
/// the selected row's label and fill `menu.hover_text_color` and
/// `menu.hover_background`, which each theme states against the menu's
/// fill. Its corners are `menu.border.corner_radius`: iced rounds the
/// highlighted row with the panel's radius (`overlay/menu.rs:550`), and the
/// popover's would round every hovered row to a pill. A selected row is a
/// row highlight, painted over a panel the widget also paints, so it is
/// emitted as given rather than composited: a translucent fill (KDE,
/// Windows) lays itself over the panel.
///
/// `shadow` has no native source -- the model has `defaults.shadow_color` but
/// no offset or blur -- and comes from `overlay::menu::default(theme)`.
#[must_use = "this returns the style function; it does not apply it"]
pub fn menu(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme) -> iced_widget::overlay::menu::Style + Clone + use<> {
    menu_from(&MenuSource::of(resolved))
}

/// What the menu style reads from the theme, of either model. Each value
/// `None` is one the theme leaves unstated, iced's own then
/// (`overlay::menu::default`; spec §10, rule 0, and §2, `from_state`, for
/// the two hovered values); the 0.7.2 model states all but the shadow.
#[derive(Clone, Copy, Debug)]
pub(crate) struct MenuSource {
    pub background: Color,
    pub label: Color,
    /// The label of the item under the pointer.
    pub selected_label: Option<Color>,
    /// The highlight of the item under the pointer, painted as given over
    /// the panel.
    pub selected_background: Option<Color>,
    /// The frame's stroke; `None` with a stated `width` is no line.
    pub line: Option<Line>,
    pub width: Option<f32>,
    /// The one radius iced rounds the popup and the item highlight with.
    pub radius: Option<f32>,
    pub shadow: Option<Shadow>,
}

impl MenuSource {
    /// The 0.7.2 model's reads.
    pub(crate) fn of(resolved: &ResolvedTheme) -> Self {
        let m = &resolved.menu;
        let p = &resolved.popover;
        Self {
            background: to_color(m.background_color),
            label: to_color(m.font.color),
            selected_label: Some(to_color(m.hover_text_color)),
            selected_background: Some(to_color(m.hover_background)),
            line: Some(Line::outside(to_color(p.border.color))),
            width: Some(p.border.line_width),
            radius: Some(m.border.corner_radius),
            shadow: None,
        }
    }

    /// The stroke where it lies as `listed` says, and the radius iced's own
    /// where it says so, for the parity test's expected values.
    #[cfg(test)]
    pub(crate) fn listed(self, listed: crate::next::parity::Listed) -> Self {
        Self {
            line: self.line.map(|l| l.aligned(listed.align)),
            radius: self.radius.filter(|_| !listed.iced_corners),
            ..self
        }
    }
}

/// [`menu`] of what it reads. The frame's stroke is laid over the panel's
/// fill ([`Line::over`]); a selected row is a highlight iced paints over the
/// panel, so its fill is as given.
pub(crate) fn menu_from(
    m: &MenuSource,
) -> impl Fn(&Theme) -> iced_widget::overlay::menu::Style + Clone + use<> {
    use iced_widget::overlay::menu::Style;

    let m = *m;

    move |theme| {
        let iced = iced_widget::overlay::menu::default(theme);
        let border = Border {
            color: match (m.line, m.width) {
                (Some(line), _) => line.over(m.background),
                (None, Some(_)) => Border::default().color,
                (None, None) => iced.border.color,
            },
            width: m.width.unwrap_or(iced.border.width),
            radius: m.radius.map_or(iced.border.radius, Radius::new),
        };
        Style {
            background: Background::Color(m.background),
            border,
            text_color: m.label,
            selected_text_color: m.selected_label.unwrap_or(iced.selected_text_color),
            selected_background: m
                .selected_background
                .map_or(iced.selected_background, Background::Color),
            shadow: m.shadow.unwrap_or(iced.shadow),
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
/// carries no rail border, so the rail's border is iced's too.
///
/// The handle is outlined `slider.thumb_border_line_width` wide, inside the
/// handle (iced draws one quad with its border, `slider.rs:506-522`), in
/// `.thumb_border_color` at rest and `.thumb_hover_border_color` (else the
/// rest colour) hovered and dragged, emitted as given, as the toggler's
/// thumb edge is: no line at rest where `thumb_border_color` is unset, the
/// handle filled whole at its outer size. A width unset, 0 or not finite
/// draws no outline: the border is iced's own, 0 wide and transparent.
/// iced's `Hovered` and `Dragged` cover the whole slider, not the handle
/// alone (`slider.rs:421-427`), and it has no focused status
/// (`slider.rs:578-585`): `thumb_focus_border_color` is not drawn.
#[must_use = "this returns the style function; it does not apply it"]
pub fn slider(
    resolved: &ResolvedTheme,
) -> impl Fn(&Theme, iced_widget::slider::Status) -> iced_widget::slider::Style + Clone + use<> {
    slider_from(&SliderSource::of(resolved))
}

/// What the slider style reads from the theme, of either model: each
/// field's 0.7.2 name, soft options as `None` where unset.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SliderSource {
    pub fill_color: Color,
    pub track_color: Color,
    pub track_height: Option<f32>,
    pub thumb_color: Color,
    /// The thumb under the pointer, its hover layer over the thumb's fill;
    /// `None` where the theme states nothing for the hovered thumb, which
    /// is then iced's own (spec §2, `from_state`). The 0.7.2 model's unset
    /// hover layer is the platform saying the thumb does not change, and
    /// its reads state that.
    pub hovered_thumb: Option<Color>,
    /// The dragged thumb, its pressed layer over the thumb's fill; `None`,
    /// iced's own, where the theme states none (0.7.2 states none).
    pub pressed_thumb: Option<Color>,
    pub thumb_diameter: Option<f32>,
    /// The width of the thumb's outline in every status, where it is drawn.
    pub thumb_border_line_width: Option<f32>,
    /// The thumb's stroke at rest.
    pub thumb_border_color: Option<Line>,
    /// The thumb's stroke under the pointer and while dragged, each the one
    /// under the pointer where unstated, and that one the stroke at rest.
    pub thumb_hover_border_color: Option<Line>,
    pub thumb_pressed_border_color: Option<Line>,
}

impl SliderSource {
    /// The 0.7.2 model's reads.
    pub(crate) fn of(resolved: &ResolvedTheme) -> Self {
        let s = &resolved.slider;
        let thumb = to_color(s.thumb_color);
        let line = |c: Rgba| Line::outside(to_color(c));
        let hover_border = s.thumb_hover_border_color.map(line);
        Self {
            fill_color: to_color(s.fill_color),
            track_color: to_color(s.track_color),
            track_height: Some(s.track_height),
            thumb_color: thumb,
            hovered_thumb: Some(
                s.thumb_hover_color
                    .map_or(thumb, |hover| composite_over(to_color(hover), thumb)),
            ),
            pressed_thumb: None,
            thumb_diameter: Some(s.thumb_diameter),
            thumb_border_line_width: s.thumb_border_line_width,
            thumb_border_color: s.thumb_border_color.map(line),
            thumb_hover_border_color: hover_border,
            thumb_pressed_border_color: hover_border,
        }
    }

    /// Every stroke where it lies as `listed` says, and the hover options
    /// it names unset, for the parity test's expected values.
    #[cfg(test)]
    pub(crate) fn listed(self, listed: crate::next::parity::Listed) -> Self {
        let (align, s) = (listed.align, listed.states);
        let line = |l: Option<Line>| l.map(|l| l.aligned(align));
        Self {
            hovered_thumb: self.hovered_thumb.filter(|_| !s.hover_fill),
            thumb_border_color: line(self.thumb_border_color),
            thumb_hover_border_color: line(self.thumb_hover_border_color)
                .filter(|_| !s.hover_stroke),
            thumb_pressed_border_color: line(self.thumb_pressed_border_color),
            ..self
        }
    }
}

/// [`slider`] of what it reads. A rail height or a thumb diameter the
/// theme leaves unstated is iced's own.
pub(crate) fn slider_from(
    s: &SliderSource,
) -> impl Fn(&Theme, iced_widget::slider::Status) -> iced_widget::slider::Style + Clone + use<> {
    use iced_widget::slider::{Handle, HandleShape, Rail, Status, Style};

    let filled = s.fill_color;
    let remaining = s.track_color;
    let rail_width = s.track_height;

    let thumb = s.thumb_color;
    let hovered_thumb = s.hovered_thumb;
    let pressed_thumb = s.pressed_thumb;
    // The model states the thumb's diameter; iced states a circular handle by
    // its radius.
    let handle_radius = s.thumb_diameter.map(|d| d / 2.0);
    let width = s
        .thumb_border_line_width
        .filter(|w| w.is_finite() && *w > 0.0);
    let edge = move |line: Option<Line>| width.zip(line);
    let rest_line = s.thumb_border_color;
    let hovered_line = s.thumb_hover_border_color.or(rest_line);
    let pressed_line = s.thumb_pressed_border_color.or(hovered_line);
    let rest_edge = edge(rest_line);

    move |theme, status| {
        let iced = iced_widget::slider::default(theme, status);
        // A thumb the theme states no fill for in this status is iced's own
        // handle's fill.
        let handle_background = match status {
            Status::Active => Some(thumb),
            Status::Hovered => hovered_thumb,
            Status::Dragged => pressed_thumb,
        }
        .map_or(iced.handle.background, Background::Color);
        let (border_width, border_color) = match status {
            Status::Active => rest_edge,
            Status::Hovered => edge(hovered_line),
            Status::Dragged => edge(pressed_line),
        }
        .map_or(
            (iced.handle.border_width, iced.handle.border_color),
            |(width, line)| (width, line.over(painted(Some(handle_background)))),
        );
        Style {
            rail: Rail {
                backgrounds: (Background::Color(filled), Background::Color(remaining)),
                width: rail_width.unwrap_or(iced.rail.width),
                border: iced.rail.border,
            },
            handle: Handle {
                shape: match handle_radius {
                    Some(radius) => HandleShape::Circle { radius },
                    None => iced.handle.shape,
                },
                background: handle_background,
                border_width,
                border_color,
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
    let s = &resolved.scrollbar;
    scrollable_from(
        to_color(s.track_color),
        to_color(s.thumb_color),
        to_color(s.thumb_hover_color),
        s.thumb_active_color.map(to_color),
        s.overlay_mode,
    )
}

/// [`scrollable`] of what it reads: the rail's fill, the scroller's at
/// rest, hovered and dragged (a soft option), and whether the bar floats.
pub(crate) fn scrollable_from(
    track: Color,
    thumb: Color,
    hovered_thumb: Color,
    active_thumb: Option<Color>,
    overlay_mode: bool,
) -> impl Fn(&Theme, iced_widget::scrollable::Status) -> iced_widget::scrollable::Style + Clone + use<>
{
    use iced_widget::scrollable::{AutoScroll, Rail, Scroller, Status, Style};

    // A soft option: `None` is the platform saying a dragged scroller looks no
    // different from a hovered one, so it copies that color.
    let dragged_thumb = active_thumb.unwrap_or(hovered_thumb);
    // A floating bar is hidden at rest: iced 0.14 has no auto-hide and draws
    // a floating bar in every status (iced_widget 0.14.2
    // `src/scrollable.rs:1097-1121`, `:1176-1182`), and reveals
    // nothing on scroll, so the closest it comes to the platforms' overlay
    // scrollbar is a bar shown while the pointer is over the scrollable or
    // drags it, and transparent while it is `Active`.
    let floats = overlay_mode;

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
    scrollbar_from(Some(s.groove_width), Some(s.thumb_width), s.overlay_mode)
}

/// [`scrollbar`] of what it reads: the groove's and the thumb's widths, iced's
/// own where the theme states none, and whether the bar floats.
pub(crate) fn scrollbar_from(
    groove_width: Option<f32>,
    thumb_width: Option<f32>,
    overlay_mode: bool,
) -> iced_widget::scrollable::Scrollbar {
    let mut bar = iced_widget::scrollable::Scrollbar::new();
    if let Some(width) = groove_width {
        bar = bar.width(width);
    }
    if let Some(width) = thumb_width {
        bar = bar.scroller_width(width);
    }

    if overlay_mode { bar } else { bar.spacing(0.0) }
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
    let p = &resolved.progress_bar;
    progress_bar_from(
        to_color(p.track_color),
        to_color(p.fill_color),
        Outline::of(&p.border),
    )
}

/// [`progress_bar`] of what it reads; the groove's border lies on the track.
pub(crate) fn progress_bar_from(
    track: Color,
    filled: Color,
    border: Outline,
) -> impl Fn(&Theme) -> iced_widget::progress_bar::Style + Clone + use<> {
    use iced_widget::progress_bar::Style;

    let border = border.over(track);

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
    rule_from(to_color(resolved.separator.line_color))
}

/// [`rule`] and [`toolbar_rule`] of the line's colour.
pub(crate) fn rule_from(
    line: Color,
) -> impl Fn(&Theme) -> iced_widget::rule::Style + Clone + use<> {
    use iced_widget::rule::Style;

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
    let s = &resolved.splitter;
    pane_grid_from(to_color(s.hover_color), s.divider_width)
}

/// [`pane_grid`] of the split's colour and width.
pub(crate) fn pane_grid_from(
    color: Color,
    width: f32,
) -> impl Fn(&Theme) -> iced_widget::pane_grid::Style + Clone + use<> {
    use iced_widget::pane_grid::{Line, Style};

    let split = Line { color, width };

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
    let t = &resolved.tooltip;
    panel_from(
        to_color(t.background_color),
        Some(to_color(t.font.color)),
        Some(Outline::of(&t.border)),
    )
}

/// The surfaces [`tooltip`], [`container_card`], [`dialog`] and [`toolbar`]
/// paint, of what they read: the fill, the label colour, and the border on
/// the fill, each `None` that keeps `container::Style::default()`'s.
pub(crate) fn panel_from(
    panel: Color,
    label: Option<Color>,
    border: Option<Outline>,
) -> impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<> {
    use iced_widget::container::Style;

    let border = border.map(|border| border.over(panel));

    move |_theme| {
        let iced = Style::default();
        Style {
            text_color: label.or(iced.text_color),
            background: Some(Background::Color(panel)),
            border: border.unwrap_or(iced.border),
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
    let c = &resolved.card;
    panel_from(
        to_color(c.background_color),
        None,
        Some(Outline::of(&c.border)),
    )
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
    segmented_control_from(&SegmentedSource::of(resolved), disabled)
}

/// What the segmented control's styles read from the theme, of either
/// model: each field's 0.7.2 name, soft options as `None` where unset.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SegmentedSource {
    /// The window under the control.
    pub window_background: Color,
    pub background: Color,
    pub label: Color,
    pub border: Border,
    /// The line between two segments, as stated.
    pub separator: Color,
    pub active_background: Color,
    pub active_text_color: Color,
    pub hover_background: Option<Color>,
    pub disabled_opacity: f32,
}

impl SegmentedSource {
    /// The 0.7.2 model's reads.
    fn of(resolved: &ResolvedTheme) -> Self {
        let s = &resolved.segmented_control;
        Self {
            window_background: to_color(resolved.defaults.background_color),
            background: to_color(s.background_color),
            label: to_color(s.font.color),
            border: border_of(&s.border),
            separator: to_color(s.border.color),
            active_background: to_color(s.active_background),
            active_text_color: to_color(s.active_text_color),
            hover_background: s.hover_background.map(to_color),
            disabled_opacity: s.disabled_opacity,
        }
    }
}

/// [`segmented_control`] of what it reads.
pub(crate) fn segmented_control_from(
    s: &SegmentedSource,
    disabled: bool,
) -> impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<> {
    use iced_widget::container::Style;

    let fade = SegmentedFade::of(s, disabled);

    let group = fade.over_ground(s.background);
    let label = fade.over_group(s.label);

    let mut border = s.border;
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
    fn of(s: &SegmentedSource, disabled: bool) -> Self {
        let ground = s.window_background;
        Self {
            ground,
            group: composite_over(s.background, ground),
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
    segment_separator_from(&SegmentedSource::of(resolved), disabled)
}

/// [`segment_separator`] of what it reads.
pub(crate) fn segment_separator_from(
    s: &SegmentedSource,
    disabled: bool,
) -> impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<> {
    use iced_widget::container::Style;

    let line = SegmentedFade::of(s, disabled).over_group(s.separator);

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
    segment_from(&SegmentedSource::of(resolved), selected, position)
}

/// [`segment`] of what it reads.
pub(crate) fn segment_from(
    s: &SegmentedSource,
    selected: bool,
    position: SegmentPosition,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
    use iced_widget::button::{Status, Style};

    let (idle, hovered, label) = if selected {
        let fill = Some(s.active_background);
        (fill, fill, s.active_text_color)
    } else {
        (None, s.hover_background, s.label)
    };
    let fade = SegmentedFade::of(s, true);
    let disabled = idle.map(|fill| fade.over_group(fill));
    let disabled_label = match idle {
        Some(fill) => fade.over(composite_over(fill, fade.group), label),
        None => fade.over_group(label),
    };

    let inner_radius = (s.border.radius.top_left - s.border.width).max(0.0);
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
    expander_from(&ExpanderSource::of(resolved))
}

/// What the expander's styles read from the theme, of either model.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ExpanderSource {
    pub hover_background: Option<Color>,
    pub label: Color,
    /// The opacity a disabled header is faded by: the expander states none
    /// of its own, so the theme's.
    pub disabled_opacity: f32,
    /// The frame's colour, width and corners.
    pub border: Border,
    pub frame_enabled: Option<bool>,
}

impl ExpanderSource {
    /// The 0.7.2 model's reads.
    fn of(resolved: &ResolvedTheme) -> Self {
        let x = &resolved.expander;
        Self {
            hover_background: x.hover_background.map(to_color),
            label: to_color(x.font.color),
            // `ExpanderTheme` states no `disabled_opacity`: the defaults' fades it.
            disabled_opacity: resolved.defaults.disabled_opacity,
            border: Border {
                color: to_color(x.border.color),
                width: x.border.line_width,
                radius: Radius::new(x.border.corner_radius),
            },
            frame_enabled: x.frame_enabled,
        }
    }
}

/// [`expander`] of what it reads.
pub(crate) fn expander_from(
    x: &ExpanderSource,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
    use iced_widget::button::{Status, Style};

    let hovered = x.hover_background;
    let label = x.label;
    let opacity = x.disabled_opacity;

    // The header draws no outline: a stated frame is `expander_frame`'s, a
    // stated absence of one is none, and where the theme states nothing no
    // platform outlines a header alone.
    let border = Border {
        width: 0.0,
        ..x.border
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
    expander_frame_from(&ExpanderSource::of(resolved))
}

/// [`expander_frame`] of what it reads.
pub(crate) fn expander_frame_from(
    x: &ExpanderSource,
) -> impl Fn(&Theme) -> iced_widget::container::Style + Clone + use<> {
    use iced_widget::container::Style;

    let border = Border {
        width: match x.frame_enabled {
            Some(true) => x.border.width,
            Some(false) | None => 0.0,
        },
        ..x.border
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
    let d = &resolved.dialog;
    panel_from(
        to_color(d.background_color),
        Some(to_color(d.body_font.color)),
        Some(Outline::of(&d.border)),
    )
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
    let t = &resolved.toolbar;
    panel_from(
        to_color(t.background_color),
        Some(to_color(t.font.color)),
        None,
    )
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
    rule_from(to_color(resolved.toolbar.border.color))
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
    application_from(to_color(resolved.window.background_color))
}

/// [`application`] of the window's background.
pub(crate) fn application_from(
    background: Color,
) -> impl Fn(&Theme) -> iced_core::theme::Style + Clone + use<> {
    use iced_core::theme::{Base, Style};

    move |theme| Style {
        background_color: background,
        text_color: theme.base().text_color,
    }
}
