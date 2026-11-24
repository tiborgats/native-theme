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

pub(crate) use crate::next::LineAlign;

/// A border's stroke colour as the theme states it, and where it lies; or
/// no stroke, which the theme states as none (`drawn` false).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Line {
    pub color: Color,
    pub align: LineAlign,
    pub drawn: bool,
}

impl Line {
    /// A stroke handed to iced as given.
    pub(crate) const fn outside(color: Color) -> Self {
        Self {
            color,
            align: LineAlign::Outside,
            drawn: true,
        }
    }

    /// No stroke: iced draws it 0 wide ([`Self::width`]), in iced's own
    /// border colour, which no width shows.
    pub(crate) fn none() -> Self {
        Self {
            drawn: false,
            ..Self::outside(Border::default().color)
        }
    }

    /// The width iced draws this stroke at, of the width the border states:
    /// none for no stroke.
    pub(crate) fn width(self, width: f32) -> f32 {
        if self.drawn { width } else { 0.0 }
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
            width: self.line.width(self.width),
            radius: self.radius,
        }
    }

    /// The border iced is handed in a status: [`Self::over`] the fill
    /// painted, `stated` the state's own stroke where the theme states one,
    /// else each of the colour, the width and the radius by [`in_state`]
    /// against iced's own border at rest and in the status (`iced`).
    pub(crate) fn in_state(
        self,
        stated: Option<Line>,
        fill: Color,
        iced: (Border, Border),
    ) -> Border {
        match stated {
            Some(line) => self.with(Some(line)).over(fill),
            None => {
                let rest = self.over(fill);
                Border {
                    color: in_state(None, rest.color, (iced.0.color, iced.1.color)),
                    width: in_state(None, rest.width, (iced.0.width, iced.1.width)),
                    radius: in_state(None, rest.radius, (iced.0.radius, iced.1.radius)),
                }
            }
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

/// One property of a widget in a status: `stated`, what the theme states
/// for the status's state, where it states it (its `from_state` bit set,
/// queried with that state's flag alone); else the toolkit's own treatment
/// of the state (spec §2, `from_state`), which is what iced's own style
/// function changes between rest and the status: iced's own value in the
/// status (`iced.1`) where it differs from iced's own at rest (`iced.0`),
/// and the theme's value at rest (`rest`) where iced makes no change.
pub(crate) fn in_state<T: PartialEq>(stated: Option<T>, rest: T, iced: (T, T)) -> T {
    match stated {
        Some(value) => value,
        None if iced.1 == iced.0 => rest,
        None => iced.1,
    }
}

/// A pill's corners, half the shorter side, as iced rounds one: a radius
/// larger than any side, which both renderers clamp to half the shorter
/// side (iced_wgpu 0.14.0 `src/shader/quad/solid.wgsl`, `solid_vs_main`;
/// iced_tiny_skia 0.14.1 `src/engine.rs`, `draw_quad`), as iced rounds its
/// own pill, the scrollable's autoscroll overlay (iced_widget 0.14.2
/// `src/scrollable.rs`, fn default: `border::rounded(u32::MAX)`).
pub(crate) fn pill() -> Radius {
    Radius::from(u32::MAX)
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
/// fill under it: a colour, the state's whole fill, or no fill (a stated
/// `Paint::None`), which paints nothing in that state whatever lies under
/// it. The 0.7.2 model states every such fill as a colour; the 0.8.0 model
/// as a whole fill or none (spec §9, step 2).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Layer {
    Color(Color),
    /// The fill the part shows in the state, painted as given, as its rest
    /// fill is: a translucent one over what lies under the part.
    Whole(Color),
    Nothing,
}

impl Layer {
    /// The fill painted where this layer lies over `base`: the colour
    /// composited over it ([`composite_over`]), the whole fill, or nothing.
    pub(crate) fn over(self, base: Color) -> Color {
        match self {
            Self::Color(color) => composite_over(color, base),
            Self::Whole(color) => color,
            Self::Nothing => Color::TRANSPARENT,
        }
    }

    /// The fill painted where this layer is painted as given.
    pub(crate) fn as_given(self) -> Color {
        match self {
            Self::Color(color) | Self::Whole(color) => color,
            Self::Nothing => Color::TRANSPARENT,
        }
    }
}

/// The colour of a fill iced's own style paints: none where it paints a
/// gradient or nothing, which no stroke is composited over.
pub(crate) fn painted(background: Option<Background>) -> Color {
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
    /// The pressed fill: 0.7.2's `active_background`, its hover layer where
    /// that is unset, which its connector drew as the pressed one.
    pub active_background: Option<Layer>,
    pub disabled_background: Option<Color>,
    pub checked_background: Option<Color>,
    /// Where the theme states no checked fill, whether a checked button is
    /// drawn in the primary button's fill and label in the same status, by
    /// which iced marks a chosen button (the 0.8.0 model: spec §10, rule
    /// 0a), else in the pressed pair (0.7.2's chain); and whether a stated
    /// checked fill the theme states no checked label for is lettered in
    /// the label at rest (0.8.0: spec §10, rule 0a), else in the pressed one
    /// (0.7.2).
    pub checked_primary: bool,
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
        let hover = to_color(b.hover_background);
        Self {
            background: to_color(b.background_color),
            hover_background: Some(Layer::Color(hover)),
            active_background: Some(Layer::Color(b.active_background.map_or(hover, to_color))),
            disabled_background: b.disabled_background.map(to_color),
            checked_background: b.checked_background.map(to_color),
            checked_primary: false,
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

    /// The border a button shows in `status` over `fill`: under the pointer
    /// the hover stroke where the theme states one, pressed the pressed one,
    /// else [`Outline::in_state`] against `class`, the iced class the button
    /// replaces; the one at rest at rest and disabled.
    fn border_at(
        &self,
        theme: &Theme,
        status: iced_widget::button::Status,
        fill: Color,
        class: fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style,
    ) -> Border {
        use iced_widget::button::Status;
        let iced = (
            class(theme, Status::Active).border,
            class(theme, status).border,
        );
        match status {
            Status::Active | Status::Disabled => self.border.over(fill),
            Status::Hovered => self.border.in_state(self.hover_border_color, fill, iced),
            Status::Pressed => self.border.in_state(self.pressed_border_color, fill, iced),
        }
    }

    /// The hovered and the pressed fill: each layer over the idle fill (a
    /// whole fill as it is, [`Layer::over`]). A
    /// layer `None` is a state the theme states no fill for, whose fill is
    /// then as [`in_state`] gives it against iced's own (spec §2,
    /// `from_state`; spec §10, rule 0).
    fn layered(&self) -> (Option<Color>, Option<Color>) {
        let over = |layer: Option<Layer>| layer.map(|layer| layer.over(self.background));
        (over(self.hover_background), over(self.active_background))
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
            checked_background: a.checked_background.filter(|_| !s.checked),
            checked_primary: a.checked_primary || s.checked,
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
        // Each of the fill, the label and the stroke the theme states
        // nothing for in a state is as [`in_state`] gives it against iced's
        // own neutral class.
        let own = (
            iced_widget::button::secondary(theme, Status::Active),
            iced_widget::button::secondary(theme, status),
        );
        let state = |fill: Option<Color>, text: Option<Color>, line: Option<Line>| {
            let background = in_state(
                fill.map(|fill| Some(Background::Color(fill))),
                Some(Background::Color(idle)),
                (own.0.background, own.1.background),
            );
            let text = in_state(text, label, (own.0.text_color, own.1.text_color));
            let border =
                source
                    .border
                    .in_state(line, painted(background), (own.0.border, own.1.border));
            (background, text, border)
        };
        let (background, text_color, border) = match status {
            Status::Active => (
                Some(Background::Color(idle)),
                label,
                source.border.over(idle),
            ),
            Status::Hovered => state(hovered, hovered_label, source.hover_border_color),
            Status::Pressed => state(pressed, pressed_label, source.pressed_border_color),
            // A translucent disabled fill replaces the idle one and lets the
            // window through, so it is emitted as given rather than
            // composited.
            Status::Disabled => {
                let (fill, label, border) = disabled_button(
                    source.disabled_background,
                    idle,
                    source.disabled_label,
                    label,
                    source.border,
                    opacity,
                );
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
    let primary = button_primary_from(b);

    let source = *b;
    let opacity = b.disabled_opacity;

    move |theme, status| {
        let style = plain(theme, status);
        if !checked {
            return style;
        }
        // A checked fill the theme does not state is, on the 0.8.0 model,
        // the primary button's in the same status, fill and label (disabled,
        // its pair at rest, faded below as the checked pair is); on 0.7.2's,
        // the pressed button's, each as [`in_state`] gives it where the
        // theme states no pressed one either. A checked label the theme
        // states is laid over either. A stated checked fill is lettered in
        // the checked label, else, on the 0.8.0 model, the label at rest
        // (spec §10, rule 0a: "a stated `CHECKED` fill is lettered in the
        // stated `CHECKED` label, else in the label at rest"); on 0.7.2's,
        // the pressed one the theme states, else the label at rest.
        let (fill, label) = match source.checked_background {
            Some(fill) => {
                let pressed = source.active_label.filter(|_| !source.checked_primary);
                (
                    fill,
                    source.checked_label.or(pressed).unwrap_or(source.label),
                )
            }
            None => {
                let shown = if source.checked_primary {
                    let at = match status {
                        Status::Disabled => Status::Active,
                        _ => status,
                    };
                    primary(theme, at)
                } else {
                    plain(theme, Status::Pressed)
                };
                (
                    painted(shown.background),
                    source.checked_label.unwrap_or(shown.text_color),
                )
            }
        };
        let outline = source.border;
        let (background, text_color, border) = match status {
            Status::Active | Status::Hovered | Status::Pressed => (
                fill,
                label,
                source.border_at(theme, status, fill, iced_widget::button::secondary),
            ),
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
    /// The 0.7.2 model's reads: an unset hovered or pressed fill is the
    /// push button's, `button.hover_background` and
    /// `button.active_background` (the button's hovered one where that is
    /// unset), as [`ButtonSource::of`] reads them.
    pub(crate) fn of(resolved: &ResolvedTheme) -> Self {
        let t = &resolved.toolbar;
        let b = &resolved.button;
        let line = |c: Option<Rgba>| c.map(|c| Line::outside(to_color(c)));
        let checked_hover = line(
            t.button_hover_border_color
                .or(t.button_checked_border_color),
        );
        let hover = t.button_hover_background.unwrap_or(b.hover_background);
        let active = t
            .button_active_background
            .or(b.active_background)
            .unwrap_or(b.hover_background);
        Self {
            hover_background: Some(Layer::Color(to_color(hover))),
            active_background: Some(Layer::Color(to_color(active))),
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

    /// [`Self::aligned`], and the state fills `listed` names unset, for the
    /// parity test's expected values.
    #[cfg(test)]
    pub(crate) fn listed(self, listed: crate::next::parity::Listed) -> Self {
        let s = listed.states;
        let a = self.aligned(listed.align);
        Self {
            hover_background: a.hover_background.filter(|_| !s.hover_fill),
            active_background: a.active_background.filter(|_| !s.pressed_fill),
            ..a
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
    // for them to lie over. Where the toolbar's source gives no layer for a
    // state, the fill and the label are as [`in_state`] gives them against
    // iced's own neutral class (spec §10, rule 0), the toolbar button's fill
    // at rest being none.
    let hovered = t.hover_background.map(Layer::as_given);
    let pressed = t.active_background.map(Layer::as_given);
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
    let primary = button_primary_from(b);
    let checked_primary = b.checked_primary;

    move |theme, status| {
        let iced = Style::default();
        let state = |fill: Option<Color>, text: Option<Color>, status| {
            let own = (
                iced_widget::button::secondary(theme, Status::Active),
                iced_widget::button::secondary(theme, status),
            );
            (
                in_state(
                    fill,
                    Color::TRANSPARENT,
                    (painted(own.0.background), painted(own.1.background)),
                ),
                in_state(text, label, (own.0.text_color, own.1.text_color)),
            )
        };
        let (hovered, hovered_label) = state(hovered, hover_label, Status::Hovered);
        let (pressed, pressed_label) = state(pressed, active_label, Status::Pressed);
        // A checked fill the theme does not state is, on the 0.8.0 model,
        // the primary button's in the same status, fill and label, a
        // checked label the theme states laid over it; on 0.7.2's, the
        // pressed toolbar button's. A stated checked fill is lettered in the
        // checked label, else, on the 0.8.0 model, the label at rest (spec
        // §10, rule 0a: "a stated `CHECKED` fill is lettered in the stated
        // `CHECKED` label, else in the label at rest"); on 0.7.2's, the
        // pressed one.
        let (fill, checked_label) = match (checked_fill, status) {
            (None, Status::Active | Status::Hovered | Status::Pressed) if checked_primary => {
                let shown = primary(theme, status);
                (
                    painted(shown.background),
                    checked_label.unwrap_or(shown.text_color),
                )
            }
            (Some(fill), _) if checked_primary => (fill, checked_label.unwrap_or(label)),
            (Some(fill), _) => (fill, checked_label.unwrap_or(pressed_label)),
            (None, _) => (pressed, pressed_label),
        };
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
        let (background, text_color, border) = match status {
            Status::Active => (
                Some(Background::Color(idle)),
                idle_label,
                source.border.over(idle),
            ),
            // Neither the fill nor the label of these two states has a native
            // source, so each is as [`in_state`] gives it against this
            // class.
            Status::Hovered | Status::Pressed => {
                let rest = class(theme, Status::Active);
                let background = in_state(
                    None,
                    Some(Background::Color(idle)),
                    (rest.background, iced.background),
                );
                (
                    background,
                    in_state(None, idle_label, (rest.text_color, iced.text_color)),
                    source.border_at(theme, status, painted(background), class),
                )
            }
            Status::Disabled => {
                let (fill, idle, label, idle_label) = disabled;
                let (fill, label, border) =
                    disabled_button(fill, idle, label, idle_label, source.border, opacity);
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
            border: source.border_at(theme, status, background, iced_widget::button::primary),
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
        // Each of the fill and the label the theme states nothing for in a
        // hover or a press is as [`in_state`] gives it against iced's own
        // text button.
        let rest = iced_widget::button::text(theme, Status::Active);
        let own = |fill: Option<Background>, text: Option<Color>| {
            (
                in_state(fill.map(Some), idle, (rest.background, iced.background)),
                in_state(text, label, (rest.text_color, iced.text_color)),
            )
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
/// [`InputSource`]: the fill, the border and the value of a field at rest
/// and disabled, the hovered and focused fields' strokes, and the two
/// colours every state shares. The 0.7.2 model states one `InputTheme` for
/// both fields, the multi-line one's border at rest being its own
/// `text_area.border`; the 0.8.0 model gives the editor every colour of its
/// own, from `text_area`.
#[derive(Clone, Copy)]
struct InputColors {
    active: (Color, Border, Color),
    border: Outline,
    hover_line: Option<Line>,
    focus_line: Option<Line>,
    /// The focused field's border where an inset focus ring is drawn as
    /// it.
    focus_ring: Option<Border>,
    disabled: (Color, Border, Color),
    placeholder: Color,
    selection: Color,
}

/// Whether a focus ring `width` wide at `offset` from the field's edge is
/// inset at that edge (offset minus the width: libadwaita's), which iced
/// draws as the field's border.
fn inset_ring(width: f32, offset: f32) -> bool {
    width > 0.0 && offset == -width
}

impl InputColors {
    fn new(i: &InputSource) -> Self {
        let idle = i.background;
        // A translucent disabled fill replaces the idle one and lets the
        // window through, so it is emitted as given.
        let disabled = i.disabled_background.unwrap_or(i.background);
        let text = i.text;
        let opacity = i.disabled_opacity;

        let border = i.border;
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
        // can draw; there a focused field takes its focused stroke
        // ([`Self::focused`]), and `crate::focus_ring` draws the ring round
        // the field it wraps.
        let focus_color = i.focus_border_color.map_or(i.focus_ring_color, |l| l.color);
        let focus_ring = inset_ring(i.focus_ring_width, i.focus_ring_offset).then(|| Border {
            color: composite_over(focus_color, idle),
            width: border.width.max(i.focus_ring_width),
            radius: border.radius,
        });
        // A field the theme states no opacity for is faded as iced's own
        // disabled field is: not at all (iced_widget 0.14.2
        // `src/text_input.rs`, `default`, `Status::Disabled`).
        let fade = |color: Color| opacity.map_or(color, |opacity| faded(color, opacity));
        let disabled_border = border.over(disabled);

        Self {
            active: (idle, border.over(idle), text),
            border,
            hover_line: i.hover_border_color,
            focus_line: i.focus_border_color,
            focus_ring,
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

    /// The hovered field: its fill and value at rest, and its stroke where
    /// the theme states one for the state, else the border as
    /// [`Outline::in_state`] gives it against `iced`, iced's own field's
    /// border at rest and hovered.
    fn hovered(&self, iced: (Border, Border)) -> (Color, Border, Color) {
        let (idle, _, text) = self.active;
        (
            idle,
            self.border.in_state(self.hover_line, idle, iced),
            text,
        )
    }

    /// The focused field: its fill and value at rest, and the inset focus
    /// ring drawn as its border, else its stroke where the theme states one
    /// for the state, else the border as [`Outline::in_state`] gives it
    /// against `iced`, iced's own field's border at rest and focused.
    fn focused(&self, iced: (Border, Border)) -> (Color, Border, Color) {
        let (idle, _, text) = self.active;
        let border = self
            .focus_ring
            .unwrap_or_else(|| self.border.in_state(self.focus_line, idle, iced));
        (idle, border, text)
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
    /// border. 0.7.2's field keeps its line at rest in a state it states no
    /// border colour for (its hovered and focused colours are soft
    /// options), but under an inset focus ring, which takes the ring's
    /// colour.
    pub(crate) fn of(
        i: &ResolvedInputTheme,
        border: &ResolvedWidgetBorder,
        d: &ResolvedDefaults,
    ) -> Self {
        let border = Outline::of(border);
        let line = |c: Rgba| Line::outside(to_color(c));
        let inset = inset_ring(d.focus_ring_width, d.focus_ring_offset);
        Self {
            background: to_color(i.background_color),
            disabled_background: i.disabled_background.map(to_color),
            text: to_color(i.font.color),
            disabled_text: to_color(i.disabled_text_color),
            disabled_opacity: Some(i.disabled_opacity),
            border,
            hover_border_color: Some(i.hover_border_color.map_or(border.line, line)),
            focus_border_color: i
                .focus_border_color
                .map(line)
                .or((!inset).then_some(border.line)),
            focus_ring_color: to_color(d.focus_ring_color),
            focus_ring_width: d.focus_ring_width,
            focus_ring_offset: d.focus_ring_offset,
            placeholder: to_color(i.placeholder_color),
            selection: to_color(i.selection_background),
        }
    }

    /// Every stroke where it lies as `listed` says, and the hovered and the
    /// focused stroke unset where it names them, for the parity test's
    /// expected values.
    #[cfg(test)]
    pub(crate) fn listed(self, listed: crate::next::parity::Listed) -> Self {
        let (align, s) = (listed.align, listed.states);
        let line = |l: Option<Line>, unset: bool| l.map(|l| l.aligned(align)).filter(|_| !unset);
        Self {
            border: self.border.aligned(align),
            hover_border_color: line(self.hover_border_color, s.hover_stroke),
            focus_border_color: line(self.focus_border_color, s.focus_stroke),
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
        let own = || {
            (
                iced_widget::text_input::default(theme, Status::Active).border,
                iced.border,
            )
        };
        let (background, border, value) = match status {
            Status::Active => input.active,
            Status::Hovered => input.hovered(own()),
            // A field that is focused *and* hovered takes the focus border,
            // as iced's own default does: its `Focused` arm ignores
            // `is_hovered` (`text_input.rs:1783-1789`).
            Status::Focused { is_hovered: _ } => input.focused(own()),
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

    move |theme, status| {
        let own = || {
            (
                iced_widget::text_editor::default(theme, Status::Active).border,
                iced_widget::text_editor::default(theme, status).border,
            )
        };
        let (background, border, value) = match status {
            Status::Active => input.active,
            Status::Hovered => input.hovered(own()),
            // Focused and hovered takes the focus border here too, as iced's
            // own default does (`text_editor.rs:1490`).
            Status::Focused { is_hovered: _ } => input.focused(own()),
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
    /// The fill of a checked box under the pointer.
    pub checked_hover_fill: Option<Color>,
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
            // 0.7.2's connector kept a checked box's fill under the pointer.
            checked_hover_fill: Some(to_color(c.checked_background)),
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
            checked_hover_fill: self.checked_hover_fill.filter(|_| !s.hover_fill),
            hover_border: line(self.hover_border),
            checked_hover_border: line(self.checked_hover_border),
            hover_label: self.hover_label.filter(|_| !s.hover_label),
            ..self
        }
    }

    /// The fill, the border colour and the stroke's width (of the box's
    /// `width`) of a box that is checked or not, under the pointer or not,
    /// the stroke laid over that fill ([`Line::over`]). Under the pointer
    /// each the theme states nothing for is as [`in_state`] gives it against
    /// `iced`, iced's own fill and border at rest and in the status.
    fn at(
        &self,
        is_checked: bool,
        is_hovered: bool,
        width: f32,
        iced: impl Fn() -> ((Color, Border), (Color, Border)),
    ) -> (Color, Color, f32) {
        let (fill, line, hover_fill, hover_line) = if is_checked {
            (
                self.checked,
                self.checked_border,
                self.checked_hover_fill,
                self.checked_hover_border,
            )
        } else {
            (
                self.unchecked,
                self.unchecked_border,
                self.hover_fill,
                self.hover_border,
            )
        };
        if !is_hovered {
            return (fill, line.over(fill), line.width(width));
        }
        let ((iced_fill, iced_rest), (iced_hover_fill, iced_hover)) = iced();
        let fill = in_state(hover_fill, fill, (iced_fill, iced_hover_fill));
        let rest = Outline {
            line,
            width,
            radius: iced_rest.radius,
        };
        let border = rest.in_state(hover_line, fill, (iced_rest, iced_hover));
        (fill, border.color, border.width)
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
    /// The stroke of a disabled box unchecked and checked, where the theme
    /// states one for the state; `None` where it states none, the stroke
    /// then as [`Outline::in_state`] gives it against iced's own box at
    /// rest and disabled.
    pub disabled_border: Option<Line>,
    pub checked_disabled_border: Option<Line>,
    pub disabled_opacity: Option<f32>,
    /// The check mark.
    pub mark: Color,
    /// The checked disabled box's mark, where the theme states a disabled
    /// mark.
    pub disabled_mark: Option<Color>,
    pub label: Color,
    pub disabled_label: Color,
    pub border_width: f32,
    pub border_radius: f32,
}

impl CheckboxSource {
    /// The 0.7.2 model's reads: a disabled box keeps its outline at rest,
    /// which says whether it is checked.
    pub(crate) fn of(resolved: &ResolvedTheme) -> Self {
        let c = &resolved.checkbox;
        let fills = CheckFills::of(c);
        Self {
            fills,
            disabled_background: c.disabled_background.map(to_color),
            checked_disabled_background: c.disabled_background.map(to_color),
            disabled_border: Some(fills.unchecked_border),
            checked_disabled_border: Some(fills.checked_border),
            disabled_opacity: Some(c.disabled_opacity),
            mark: to_color(c.indicator_color),
            disabled_mark: None,
            label: to_color(c.font.color),
            disabled_label: to_color(c.disabled_text_color),
            border_width: c.border.line_width,
            border_radius: c.border.corner_radius,
        }
    }

    /// [`CheckFills::listed`] of the box's fills, and its disabled strokes
    /// where they lie as `listed` says, unset where it names them, for the
    /// parity test's expected values.
    #[cfg(test)]
    pub(crate) fn listed(self, listed: crate::next::parity::Listed) -> Self {
        let line = |l: Option<Line>| {
            l.map(|l| l.aligned(listed.align))
                .filter(|_| !listed.states.disabled_stroke)
        };
        Self {
            fills: self.fills.listed(listed),
            disabled_border: line(self.disabled_border),
            checked_disabled_border: line(self.checked_disabled_border),
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
    let (disabled_border, checked_disabled_border) = (c.disabled_border, c.checked_disabled_border);
    let opacity = c.disabled_opacity;

    let mark = c.mark;
    let disabled_mark = c.disabled_mark;
    let label = c.label;
    let disabled_label = c.disabled_label;

    let border_width = c.border_width;
    let border_radius = Radius::new(c.border_radius);

    move |theme, status| {
        let own = |status| {
            let own = iced_widget::checkbox::primary(theme, status);
            (painted(Some(own.background)), own.border, own.text_color)
        };
        // A box the theme states no opacity for is drawn in the colours it
        // states, not faded.
        let fade = |under: Color, color: Color| {
            opacity.map_or(color, |opacity| faded_as_one(ground, under, color, opacity))
        };
        let (background, border_color, text_color, icon_color, width) = match status {
            Status::Active { is_checked } => {
                let (fill, border, width) = fills.at(is_checked, false, border_width, || {
                    let (fill, border, _) = own(status);
                    ((fill, border), (fill, border))
                });
                (fill, border, Some(label), mark, width)
            }
            Status::Hovered { is_checked } => {
                let rest = own(Status::Active { is_checked });
                let hovered = own(status);
                let (fill, border, width) = fills.at(is_checked, true, border_width, || {
                    ((rest.0, rest.1), (hovered.0, hovered.1))
                });
                let text = in_state(
                    fills.hover_label.map(Some),
                    Some(label),
                    (rest.2, hovered.2),
                );
                (fill, border, text, mark, width)
            }
            // The platform states one disabled fill for both, and the box
            // keeps the outline that says whether it is checked. The mark
            // leaves the accent with the fill: `indicator_color` is the
            // on-accent color, and a disabled box no longer shows the accent,
            // so the mark takes the one foreground the platform states for
            // everything it dims -- the same field the disabled label takes.
            // With no disabled fill stated the box is its enabled self. A
            // disabled mark the theme states is drawn in its own colour. A
            // disabled stroke the theme states is drawn; one it leaves
            // unstated is as `Outline::in_state` gives it against iced's own
            // box, which recolours its border when disabled (iced_widget
            // 0.14.2 `src/checkbox.rs`, `primary`: `background.weak` for
            // `background.strong` unchecked, `background.strong` for
            // `primary.base` checked). The box is then faded by
            // `checkbox.disabled_opacity` as one widget over the ground: the
            // border and the mark on the fill.
            Status::Disabled { is_checked } => {
                let (at_rest, line, stated_line, disabled) = if is_checked {
                    (
                        fills.checked,
                        fills.checked_border,
                        checked_disabled_border,
                        checked_disabled,
                    )
                } else {
                    (
                        fills.unchecked,
                        fills.unchecked_border,
                        disabled_border,
                        disabled,
                    )
                };
                let (background, mark) = match disabled {
                    Some(fill) => (fill, disabled_label),
                    None => (at_rest, mark),
                };
                let mark = disabled_mark.filter(|_| is_checked).unwrap_or(mark);
                let fill = composite_over(background, ground);
                let (_, own_rest, _) = own(Status::Active { is_checked });
                let (_, own_disabled, _) = own(status);
                let rest = Outline {
                    line,
                    width: border_width,
                    radius: own_rest.radius,
                };
                let border = rest.in_state(stated_line, background, (own_rest, own_disabled));
                (
                    fade(ground, background),
                    fade(fill, border.color),
                    Some(fade(ground, disabled_label)),
                    fade(fill, mark),
                    border.width,
                )
            }
        };
        Style {
            background: Background::Color(background),
            icon_color,
            border: Border {
                color: border_color,
                width,
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
        let own = |status| {
            let own = iced_widget::radio::default(theme, status);
            let border = Border {
                color: own.border_color,
                width: own.border_width,
                radius: Radius::default(),
            };
            (painted(Some(own.background)), border, own.text_color)
        };
        let (background, border_color, text_color, border_width) = match status {
            Status::Active { is_selected } => {
                let (fill, border, width) = fills.at(is_selected, false, border_width, || {
                    let (fill, border, _) = own(status);
                    ((fill, border), (fill, border))
                });
                (fill, border, Some(label), width)
            }
            Status::Hovered { is_selected } => {
                let rest = own(Status::Active { is_selected });
                let hovered = own(status);
                let (fill, border, width) = fills.at(is_selected, true, border_width, || {
                    ((rest.0, rest.1), (hovered.0, hovered.1))
                });
                let text = in_state(
                    fills.hover_label.map(Some),
                    Some(label),
                    (rest.2, hovered.2),
                );
                (fill, border, text, width)
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
/// (where it states nothing for the hovered switch, the track and the thumb
/// are as `in_state` gives them, spec §2 `from_state`: iced's own track
/// is the same under the pointer, so it is the track at rest, and iced's
/// own thumb changes; 0.7.2's unset hover is the switch at rest) and
/// `.disabled_*` replacing it, as given. The thumb is
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
/// the fill under it in each state as it lies (`Line::over`): the 0.7.2
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
    /// The thumb of a hovered switch, on and off.
    pub hovered_thumb: Option<Color>,
    pub hovered_unchecked_thumb: Option<Color>,
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
                        align,
                        ..Line::outside(to_color(colour))
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
            // 0.7.2 states no hovered thumb, and its connector drew the
            // thumb at rest.
            hovered_thumb: Some(to_color(s.thumb_background)),
            hovered_unchecked_thumb: Some(to_color(unchecked_thumb)),
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

    /// The hover options `listed` names unset, for the parity test's
    /// expected values. The strokes keep where 0.7.2's reads lay them (the
    /// track's inside the track, the thumb's as given), which the 0.8.0
    /// presets' match.
    #[cfg(test)]
    pub(crate) fn listed(self, listed: crate::next::parity::Listed) -> Self {
        let s = listed.states;
        Self {
            hovered_checked_background: self.hovered_checked_background.filter(|_| !s.hover_fill),
            hovered_unchecked_background: self
                .hovered_unchecked_background
                .filter(|_| !s.hover_fill),
            hovered_thumb: self.hovered_thumb.filter(|_| !s.hover_thumb),
            hovered_unchecked_thumb: self.hovered_unchecked_thumb.filter(|_| !s.hover_thumb),
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
    let hovered_checked = s.hovered_checked_background;
    let hovered_unchecked = s.hovered_unchecked_background;
    let hovered_on_thumb = s.hovered_thumb;
    let hovered_off_thumb = s.hovered_unchecked_thumb;
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
    // A stroke stated as none is 0 wide, its colour, which no width shows,
    // iced's own (`None` here).
    let over = |edge: Option<(f32, Line)>, fill: Color| {
        edge.map(|(width, line)| (line.width(width), line.drawn.then(|| line.over(fill))))
    };
    let rest_edge_on = s.checked_track_border;
    let rest_edge_off = s.unchecked_track_border;
    let disabled_edge = |edge: Option<(f32, Line)>, rest: Option<(f32, Line)>, track: Color| {
        over(edge.or(rest), track).map(|(width, edge)| (width, edge.map(|e| fade(ground, e))))
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
            .map(|(width, edge)| (width, edge.map(|e| fade(composite_over(track, ground), e))))
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
            // The track and the thumb the theme states nothing for under the
            // pointer are as [`in_state`] gives them against iced's own.
            Status::Hovered { is_toggled } => {
                let (hovered, track, hovered_thumb, thumb) = if is_toggled {
                    (hovered_checked, checked, hovered_on_thumb, thumb)
                } else {
                    (
                        hovered_unchecked,
                        unchecked,
                        hovered_off_thumb,
                        unchecked_thumb,
                    )
                };
                let rest = iced_widget::toggler::default(theme, Status::Active { is_toggled });
                (
                    in_state(
                        hovered,
                        track,
                        (
                            painted(Some(rest.background)),
                            painted(Some(iced.background)),
                        ),
                    ),
                    in_state(
                        hovered_thumb,
                        thumb,
                        (
                            painted(Some(rest.foreground)),
                            painted(Some(iced.foreground)),
                        ),
                    ),
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
        let (background_border_width, background_border_color) = track_edge.map_or(
            (iced.background_border_width, iced.background_border_color),
            |(width, color)| (width, color.unwrap_or(iced.background_border_color)),
        );
        let (foreground_border_width, foreground_border_color) = thumb_edge.map_or(
            (iced.foreground_border_width, iced.foreground_border_color),
            |(width, color)| (width, color.unwrap_or(iced.foreground_border_color)),
        );
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
/// (`Line::over`). Where the 0.8.0 model states nothing for the hovered
/// field, its fill and stroke are as `in_state` gives them (spec §2,
/// `from_state`): iced's own fill is the same under the pointer, so it is
/// the fill at rest, and iced's own border colour changes. The placeholder is
/// `input.placeholder_color`: the model states it once, for every field that
/// has one.
///
/// An open field (`pick_list::Status::Opened`) is `combo_box.open_background`
/// layered over the idle fill, as the hover fill is, and outlined in
/// `.open_border_color`: Breeze draws an open combo box as a pressed button,
/// framed in its focus colour (platform-facts §2.24). An open field off the
/// pointer whose fill or border the theme leaves unset is as `in_state`
/// gives it, the fill at rest and iced's own open border colour; under the
/// pointer (`Opened` carries whether it is on the field,
/// `pick_list.rs:838-849`) the hovered fill or border where the theme
/// states one, else the same. The border's width and radius are
/// `combo_box.border`'s in every status. `handle_color` has no native
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
    /// The open field's layer over the idle fill; `None` where the theme
    /// states none.
    pub open_background: Option<Layer>,
    /// The open field's layer under the pointer: 0.7.2's
    /// `open_background`, which the field keeps under the pointer.
    pub open_hover_background: Option<Layer>,
    pub border: Outline,
    /// The border under the pointer, `None` as the hover fill.
    pub hover_border_color: Option<Line>,
    /// The open field's border; `None` where the theme states none.
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
            // 0.7.2's connector framed an open field it states no open
            // border for in its border at rest.
            open_border_color: Some(c.open_border_color.map_or(border.line, line)),
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
            open_border_color: line(self.open_border_color).filter(|_| !s.open_stroke),
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
        let rest = iced_widget::pick_list::default(theme, Status::Active);
        // The fill and the stroke the theme states for the status: the
        // pointer's, and an open field's own over them; each it states
        // nothing for is as [`in_state`] gives it against iced's own.
        let (fill, line) = match status {
            Status::Active => (Some(idle), Some(border.line)),
            Status::Hovered => (hovered, hovered_border),
            Status::Opened { is_hovered: false } => (open, open_border),
            Status::Opened { is_hovered: true } => (
                open_hovered.or(hovered),
                open_hovered_border.or(hovered_border),
            ),
        };
        let background = in_state(
            fill,
            idle,
            (
                painted(Some(rest.background)),
                painted(Some(iced.background)),
            ),
        );
        Style {
            text_color: in_state(None, label, (rest.text_color, iced.text_color)),
            placeholder_color: placeholder,
            handle_color: iced.handle_color,
            background: Background::Color(background),
            border: border.in_state(line, background, (rest.border, iced.border)),
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
            // The item under the pointer against one at rest, which shows
            // the panel ([`in_state`]).
            selected_text_color: in_state(
                m.selected_label,
                m.label,
                (iced.text_color, iced.selected_text_color),
            ),
            selected_background: in_state(
                m.selected_background.map(Background::Color),
                Background::Color(m.background),
                (iced.background, iced.selected_background),
            ),
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
    /// The filled rail under the pointer and dragged, where the theme
    /// states it; 0.7.2's connector drew it as at rest.
    pub hovered_fill_color: Option<Color>,
    pub dragged_fill_color: Option<Color>,
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
    /// The thumb's stroke under the pointer and while dragged, where the
    /// theme states one for the state, else as [`in_state`] gives it: the
    /// stroke at rest, iced's own handle not changing its border. 0.7.2's
    /// dragged thumb keeps the hovered stroke, and its reads state that.
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
            hovered_fill_color: Some(to_color(s.fill_color)),
            dragged_fill_color: Some(to_color(s.fill_color)),
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
            hovered_fill_color: self.hovered_fill_color.filter(|_| !s.rail),
            dragged_fill_color: self.dragged_fill_color.filter(|_| !s.rail),
            thumb_border_color: line(self.thumb_border_color),
            thumb_hover_border_color: line(self.thumb_hover_border_color)
                .filter(|_| !s.hover_stroke),
            thumb_pressed_border_color: line(self.thumb_pressed_border_color)
                .filter(|_| !s.pressed_stroke),
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
    let hovered_filled = s.hovered_fill_color;
    let dragged_filled = s.dragged_fill_color;
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
    let rest_edge = edge(s.thumb_border_color);
    let hovered_edge = edge(s.thumb_hover_border_color);
    let pressed_edge = edge(s.thumb_pressed_border_color);

    move |theme, status| {
        let iced = iced_widget::slider::default(theme, status);
        let rest = iced_widget::slider::default(theme, Status::Active);
        // The thumb's fill and the filled rail's the theme states nothing
        // for in this status are as [`in_state`] gives them against iced's
        // own.
        let (stated_thumb, stated_fill) = match status {
            Status::Active => (Some(thumb), Some(filled)),
            Status::Hovered => (hovered_thumb, hovered_filled),
            Status::Dragged => (pressed_thumb, dragged_filled),
        };
        let handle_background = in_state(
            stated_thumb.map(Background::Color),
            Background::Color(thumb),
            (rest.handle.background, iced.handle.background),
        );
        let filled = in_state(
            stated_fill.map(Background::Color),
            Background::Color(filled),
            (rest.rail.backgrounds.0, iced.rail.backgrounds.0),
        );
        // The thumb's stroke over the fill it paints in this status: the
        // state's own where the theme states one, else each of the width
        // and the colour as [`in_state`] gives it against iced's own handle,
        // the theme's at rest where iced makes no change.
        let under = painted(Some(handle_background));
        let drawn = |edge: Option<(f32, Line)>| edge.map(|(width, line)| (width, line.over(under)));
        let stated = drawn(match status {
            Status::Active => None,
            Status::Hovered => hovered_edge,
            Status::Dragged => pressed_edge,
        });
        let (rest_width, rest_color) =
            drawn(rest_edge).unwrap_or((rest.handle.border_width, rest.handle.border_color));
        let border_width = in_state(
            stated.map(|(width, _)| width),
            rest_width,
            (rest.handle.border_width, iced.handle.border_width),
        );
        let border_color = in_state(
            stated.map(|(_, color)| color),
            rest_color,
            (rest.handle.border_color, iced.handle.border_color),
        );
        Style {
            rail: Rail {
                backgrounds: (filled, Background::Color(remaining)),
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
/// `.thumb_active_color` while it is dragged, `.thumb_hover_color` where that
/// is unset. All three are thumbs, so they are emitted as given rather than
/// composited: iced paints them over a rail it also paints.
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
    scrollable_from(&ScrollbarSource::of(resolved))
}

/// What the scrollable's style reads from the theme, of either model: the
/// rail's fill, the scroller's at rest, under the pointer and dragged, and
/// whether the bar floats.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ScrollbarSource {
    pub track: Color,
    pub thumb: Color,
    /// The scroller under the pointer; `None` where the theme states nothing
    /// for the hovered thumb, iced's own then (spec §2, `from_state`).
    pub hovered_thumb: Option<Color>,
    /// The dragged scroller; `None`, iced's own, where the theme states
    /// nothing for the pressed thumb.
    pub dragged_thumb: Option<Color>,
    pub overlay_mode: bool,
    /// The scroller's stroke at rest and its width, laid over its fill;
    /// `None`, iced's own, where the theme states none (0.7.2 has no
    /// field for it).
    pub thumb_edge: Option<(f32, Line)>,
    /// The scroller's corners; `None`, iced's own, where the theme states
    /// none (0.7.2 has no field for them).
    pub thumb_radius: Option<Radius>,
    /// The scroller's stroke under the pointer and dragged, where the theme
    /// states one for the state; else as [`in_state`] gives it.
    pub hovered_thumb_line: Option<Line>,
    pub dragged_thumb_line: Option<Line>,
}

impl ScrollbarSource {
    /// The 0.7.2 model's reads. Its unset `thumb_active_color` was drawn by
    /// its connector as the hovered thumb, and its reads state that.
    pub(crate) fn of(resolved: &ResolvedTheme) -> Self {
        let s = &resolved.scrollbar;
        let hovered = to_color(s.thumb_hover_color);
        Self {
            track: to_color(s.track_color),
            thumb: to_color(s.thumb_color),
            hovered_thumb: Some(hovered),
            dragged_thumb: Some(s.thumb_active_color.map_or(hovered, to_color)),
            overlay_mode: s.overlay_mode,
            thumb_edge: None,
            thumb_radius: None,
            hovered_thumb_line: None,
            dragged_thumb_line: None,
        }
    }

    /// The state options `listed` names unset, and the scroller's outline
    /// where it names one the 0.8.0 theme states, for the parity test's
    /// expected values.
    #[cfg(test)]
    pub(crate) fn listed(self, listed: crate::next::parity::Listed) -> Self {
        use crate::next::parity::Added;
        let s = listed.states;
        let listed_self = Self {
            hovered_thumb: self.hovered_thumb.filter(|_| !s.hover_fill),
            dragged_thumb: self.dragged_thumb.filter(|_| !s.pressed_fill),
            ..self
        };
        match listed.added {
            Added::Nothing | Added::CheckedButton => listed_self,
            Added::ScrollerOutline {
                width,
                align,
                rest,
                hovered,
                dragged,
            } => {
                let line = |color| Line {
                    align,
                    ..Line::outside(color)
                };
                Self {
                    thumb_edge: Some((width, line(rest))),
                    thumb_radius: Some(pill()),
                    hovered_thumb_line: Some(line(hovered)),
                    dragged_thumb_line: Some(line(dragged)),
                    ..listed_self
                }
            }
        }
    }
}

/// [`scrollable`] of what it reads. A hovered or dragged scroller the theme
/// states nothing for is iced's own for the status; a scroller's outline or
/// corners it states nothing for are iced's own.
pub(crate) fn scrollable_from(
    s: &ScrollbarSource,
) -> impl Fn(&Theme, iced_widget::scrollable::Status) -> iced_widget::scrollable::Style + Clone + use<>
{
    use iced_widget::scrollable::{AutoScroll, Rail, Scroller, Status, Style};

    let track = s.track;
    let thumb = Background::Color(s.thumb);
    let hovered_thumb = s.hovered_thumb.map(Background::Color);
    let dragged_thumb = s.dragged_thumb.map(Background::Color);
    let thumb_edge = s.thumb_edge;
    let thumb_radius = s.thumb_radius;
    let hovered_line = s.hovered_thumb_line;
    let dragged_line = s.dragged_thumb_line;
    let overlay_mode = s.overlay_mode;
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
        // A scroller under the pointer or dragged is the theme's where it
        // states one, else as [`in_state`] gives it against iced's own for
        // the status, which `iced` holds on that axis.
        let rest = iced_widget::scrollable::default(
            theme,
            Status::Active {
                is_horizontal_scrollbar_disabled: false,
                is_vertical_scrollbar_disabled: false,
            },
        );
        // Each scroller as its axis's state draws it: the fill the theme
        // states for the state, else as [`in_state`] gives it, and the
        // stroke over that fill, the state's own where the theme states one,
        // else each of its colour, width and radius as [`Outline::in_state`]
        // gives it, against iced's own scroller on that axis. A hidden bar
        // draws no stroke.
        let scroller = |axis: fn(&Style) -> Scroller,
                        state: Option<(Option<Background>, Option<Line>)>| {
            let (rest, now) = (axis(&rest), axis(&iced));
            let outline = Outline {
                line: thumb_edge.map_or(Line::outside(rest.border.color), |(_, line)| line),
                width: thumb_edge.map_or(rest.border.width, |(width, _)| width),
                radius: thumb_radius.unwrap_or(rest.border.radius),
            };
            let (fill, line) = match state {
                Some((fill, line)) => (
                    in_state(fill, thumb, (rest.background, now.background)),
                    line,
                ),
                None => (thumb, None),
            };
            let border = outline.in_state(line, painted(Some(fill)), (rest.border, now.border));
            if shown {
                Scroller {
                    background: fill,
                    border,
                }
            } else {
                Scroller {
                    background: Background::Color(Color::TRANSPARENT),
                    border: Border {
                        width: 0.0,
                        ..border
                    },
                }
            }
        };
        let vertical = |s: &Style| s.vertical_rail.scroller;
        let horizontal = |s: &Style| s.horizontal_rail.scroller;
        let hovered = Some((hovered_thumb, hovered_line));
        let dragged = Some((dragged_thumb, dragged_line));
        let (vertical, horizontal) = match status {
            Status::Active {
                is_horizontal_scrollbar_disabled: _,
                is_vertical_scrollbar_disabled: _,
            } => (scroller(vertical, None), scroller(horizontal, None)),
            Status::Hovered {
                is_horizontal_scrollbar_hovered,
                is_vertical_scrollbar_hovered,
                is_horizontal_scrollbar_disabled: _,
                is_vertical_scrollbar_disabled: _,
            } => (
                scroller(vertical, hovered.filter(|_| is_vertical_scrollbar_hovered)),
                scroller(
                    horizontal,
                    hovered.filter(|_| is_horizontal_scrollbar_hovered),
                ),
            ),
            Status::Dragged {
                is_horizontal_scrollbar_dragged,
                is_vertical_scrollbar_dragged,
                is_horizontal_scrollbar_disabled: _,
                is_vertical_scrollbar_disabled: _,
            } => (
                scroller(vertical, dragged.filter(|_| is_vertical_scrollbar_dragged)),
                scroller(
                    horizontal,
                    dragged.filter(|_| is_horizontal_scrollbar_dragged),
                ),
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
                scroller: vertical,
            },
            horizontal_rail: Rail {
                background: Some(Background::Color(rail)),
                border: iced.horizontal_rail.border,
                scroller: horizontal,
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
    /// The control's outline, its stroke laid over the group fill.
    pub border: Outline,
    /// The line between two segments, as stated; `None` where the theme
    /// states no line, which is then drawn as none.
    pub separator: Option<Color>,
    pub active_background: Color,
    pub active_text_color: Color,
    /// An unselected segment's fill under the pointer, painted as given over
    /// the group; `None` where the theme states none, no fill then.
    pub hover_background: Option<Color>,
    /// An unselected segment's fill while pressed, painted as given; `None`
    /// where the theme states none, as [`in_state`] gives it then. 0.7.2's
    /// pressed segment is the hovered one, and its reads state that.
    pub pressed_background: Option<Color>,
    /// The opacity a disabled control is faded by as one widget; `None`
    /// where the theme states none, its colours then drawn as stated.
    pub disabled_opacity: Option<f32>,
    /// The control's shadow as iced draws one; `None`, iced's own, where
    /// the theme states none (0.7.2 has no shadow geometry).
    pub shadow: Option<Shadow>,
}

impl SegmentedSource {
    /// The 0.7.2 model's reads.
    pub(crate) fn of(resolved: &ResolvedTheme) -> Self {
        let s = &resolved.segmented_control;
        Self {
            window_background: to_color(resolved.defaults.background_color),
            background: to_color(s.background_color),
            label: to_color(s.font.color),
            border: Outline::of(&s.border),
            separator: Some(to_color(s.border.color)),
            active_background: to_color(s.active_background),
            active_text_color: to_color(s.active_text_color),
            hover_background: s.hover_background.map(to_color),
            pressed_background: s.hover_background.map(to_color),
            disabled_opacity: Some(s.disabled_opacity),
            shadow: None,
        }
    }

    /// The outline's stroke where it lies as `listed` says, and the pressed
    /// fill unset where it names it, for the parity test's expected values.
    #[cfg(test)]
    pub(crate) fn listed(self, listed: crate::next::parity::Listed) -> Self {
        Self {
            border: self.border.aligned(listed.align),
            pressed_background: self
                .pressed_background
                .filter(|_| !listed.states.pressed_fill),
            ..self
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

    // The stroke over the group fill where the theme lays it there
    // ([`Line::over`]), the pair on the window, faded with the control.
    let mut border = s.border.over(s.background);
    border.color = fade.over_ground(border.color);
    // The shadow where the theme states one, faded with the control: a
    // widget's opacity scales its shadow's alpha.
    let shadow = s.shadow.map(|shadow| Shadow {
        color: faded(shadow.color, fade.opacity),
        ..shadow
    });

    move |_theme| {
        let iced = Style::default();
        Style {
            text_color: Some(label),
            background: Some(Background::Color(group)),
            border,
            shadow: shadow.unwrap_or(iced.shadow),
            snap: iced.snap,
        }
    }
}

/// How a segmented control fades as one widget, disabled: by
/// `segmented_control.disabled_opacity` over the window, each colour
/// composited over what it lies on first ([`segmented_control`]). Enabled, or
/// where the theme states no opacity, an opacity of 1, which leaves every
/// colour as stated.
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
            opacity: match s.disabled_opacity {
                Some(opacity) if disabled => opacity,
                _ => 1.0,
            },
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

    let fade = SegmentedFade::of(s, disabled);
    let line = s.separator.map(|color| fade.over_group(color));

    move |_theme| Style {
        background: line.map(Background::Color),
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

    let (idle, hovered, pressed, label) = if selected {
        let fill = Some(s.active_background);
        (fill, fill, None, s.active_text_color)
    } else {
        (
            None,
            s.hover_background,
            s.pressed_background.map(Some),
            s.label,
        )
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
        let fill = |fill: Option<Color>| fill.map(Background::Color);
        let (background, text_color) = match status {
            Status::Active => (fill(idle), label),
            Status::Hovered => (fill(hovered), label),
            // A pressed fill the theme states nothing for is as
            // [`in_state`] gives it against iced's own for a segment,
            // `button::Style::default()` in every status: the fill at rest.
            Status::Pressed => (
                in_state(
                    pressed.map(fill),
                    fill(idle),
                    (iced.background, iced.background),
                ),
                label,
            ),
            Status::Disabled => (fill(disabled), disabled_label),
        };
        Style {
            background,
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
    /// The header's fill under the pointer, painted as given; `None` where
    /// the theme states none, no fill then.
    pub hover_background: Option<Color>,
    /// The header's fill while pressed, painted as given; `None` where the
    /// theme states none, as [`in_state`] gives it then. 0.7.2's pressed
    /// header is the hovered one, and its reads state that.
    pub pressed_background: Option<Color>,
    pub label: Color,
    /// The opacity a disabled header is faded by: the expander states none
    /// of its own, so the theme's.
    pub disabled_opacity: f32,
    /// The frame's stroke, width and corners.
    pub border: Outline,
    pub frame_enabled: Option<bool>,
    /// The frame's shadow as iced draws one; `None`, iced's own, where the
    /// theme states none (0.7.2 has no shadow geometry).
    pub shadow: Option<Shadow>,
}

impl ExpanderSource {
    /// The 0.7.2 model's reads.
    pub(crate) fn of(resolved: &ResolvedTheme) -> Self {
        let x = &resolved.expander;
        Self {
            hover_background: x.hover_background.map(to_color),
            pressed_background: x.hover_background.map(to_color),
            label: to_color(x.font.color),
            // `ExpanderTheme` states no `disabled_opacity`: the defaults' fades it.
            disabled_opacity: resolved.defaults.disabled_opacity,
            border: Outline::of(&x.border),
            frame_enabled: x.frame_enabled,
            shadow: None,
        }
    }

    /// The frame's stroke where it lies as `listed` says, and the pressed
    /// fill unset where it names it, for the parity test's expected values.
    #[cfg(test)]
    pub(crate) fn listed(self, listed: crate::next::parity::Listed) -> Self {
        Self {
            border: self.border.aligned(listed.align),
            pressed_background: self
                .pressed_background
                .filter(|_| !listed.states.pressed_fill),
            ..self
        }
    }
}

/// [`expander`] of what it reads.
pub(crate) fn expander_from(
    x: &ExpanderSource,
) -> impl Fn(&Theme, iced_widget::button::Status) -> iced_widget::button::Style + Clone + use<> {
    use iced_widget::button::{Status, Style};

    let hovered = x.hover_background;
    let pressed = x.pressed_background;
    let label = x.label;
    let opacity = x.disabled_opacity;

    // The header draws no outline: a stated frame is `expander_frame`'s, a
    // stated absence of one is none, and where the theme states nothing no
    // platform outlines a header alone. Its colour, which no width shows, is
    // the frame's.
    let border = Border {
        width: 0.0,
        ..x.border.over(Color::TRANSPARENT)
    };

    move |_theme, status| {
        let iced = Style::default();
        let (background, text_color, border) = match status {
            Status::Active => (None, label, border),
            Status::Hovered => (hovered.map(Background::Color), label, border),
            // A pressed fill the theme states nothing for is as
            // [`in_state`] gives it against iced's own for the header,
            // `button::Style::default()` in every status: no fill.
            Status::Pressed => (
                in_state(
                    pressed.map(|fill| Some(Background::Color(fill))),
                    None,
                    (iced.background, iced.background),
                ),
                label,
                border,
            ),
            Status::Disabled => (None, faded(label, opacity), faded_border(border, opacity)),
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

    // The frame paints no fill: its stroke lies over nothing. Its shadow,
    // where the theme states one, is drawn with the frame alone.
    let framed = matches!(x.frame_enabled, Some(true));
    let border = Border {
        width: if framed { x.border.width } else { 0.0 },
        ..x.border.over(Color::TRANSPARENT)
    };
    let shadow = x.shadow.filter(|_| framed);

    move |_theme| {
        let iced = Style::default();
        Style {
            text_color: iced.text_color,
            background: iced.background,
            border,
            shadow: shadow.unwrap_or(iced.shadow),
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
