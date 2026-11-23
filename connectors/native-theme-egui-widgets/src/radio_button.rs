//! [`RadioButton`]: egui's radio button, its dot the theme's size.

use native_theme_egui::convert::{clamp_length, to_color32, unit_interval};
use native_theme_egui::egui;
use native_theme_egui::native_theme::color::Rgba;
use native_theme_egui::native_theme::next::{
    PartStated, PartStyle, ResolvedTheme as NextResolvedTheme, Stroke as NextStroke, StrokeAlign,
    TextStated, WidgetState,
};
use native_theme_egui::next::{fill_of, stroke_color_of, stroke_width_of, text_color_of};
use native_theme_egui::{ResolvedTheme, Role, ThemeAtlas};

use crate::parts::Parts;
use crate::{next, scope};

/// egui's radio dot is a circle of radius `small_icon_rect.width() / 3.0`
/// (`egui/src/widgets/radio_button.rs:101`), `small_icon_rect` being `Spacing::icon_width_inner`
/// square (`egui/src/style.rs:477-478`): the divisor, egui's own.
const EGUI_DOT_DIVISOR: f32 = 3.0;

/// A radio button: egui's own, in the `Role::Checkbox` scope, its dot
/// `checkbox.radio_dot_diameter` across.
///
/// **Tier W.** One `egui::RadioButton` in the checkbox role's scope — the `Selected` variant
/// while it is checked, whose cell fills the circle with `checkbox.checked_background` and
/// outlines it in `checkbox.border.color`; for `.enabled(false)` the `Disabled` variant, or
/// `SelectedDisabled` while it is checked (the checked circle and dot where the theme states
/// no disabled fill), faded as one over its backdrop — with one per-instance change: egui
/// paints the dot at a third of `Spacing::icon_width_inner` in radius
/// (`egui/src/widgets/radio_button.rs:101`), a sink the checkbox's check mark shares
/// (`egui/src/widgets/checkbox.rs:132-133`), so where the theme
/// states `checkbox.radio_dot_diameter` the scope's `icon_width_inner` becomes
/// `EGUI_DOT_DIVISOR · 0.5 · radio_dot_diameter` for this radio alone, and the dot is that
/// many pixels across, in the cell's `fg_stroke`, `checkbox.indicator_color`. Where the theme
/// states none (ios-13 and the colour-scheme presets) or a size that is not finite, the dot
/// is egui's own. Where the theme sizes the radio apart from the check box
/// (`checkbox.radio_indicator_width`, Material's 20 against its 18, `docs/platform-facts.md:1243`)
/// the scope's `icon_width` becomes that width for this radio alone. And the circle's outline lies inside
/// `checkbox.indicator_width`, as the check box's square's does: egui's circle is half the
/// icon's width plus the state's `expansion` in radius and epaint strokes it outside that
/// (`epaint/src/tessellator.rs:1531`), so this radio's `expansion` gives back the stroke's
/// width. Everything else is egui's: layout, interaction and accessibility
/// (`Role::RadioButton`).
///
/// Under an atlas of the 0.8.0 model the radio is drawn from its own homes, laid over the
/// check box's cell before egui's radio is added: `radio.indicator`'s fill and stroke at rest,
/// hovered and pressed, `radio.dot`'s fill checked, `radio.label`'s colour and
/// `radio.disabled_opacity`, each where the theme states it, the cell's elsewhere ([`Over`]).
///
/// Like egui's, it holds no value: a click is `Response::clicked`, and the caller selects it.
///
/// With no atlas installed it adds `egui::RadioButton`.
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct RadioButton {
    checked: bool,
    text: egui::WidgetText,
    enabled: bool,
    backdrop: Option<egui::Color32>,
}

impl RadioButton {
    /// A radio button, `checked` or not, labelled `text`.
    pub fn new(checked: bool, text: impl Into<egui::WidgetText>) -> Self {
        Self {
            checked,
            text: text.into(),
            enabled: true,
            backdrop: None,
        }
    }

    /// `false` shows the platform's disabled radio button, its disabled colours faded by
    /// `checkbox.disabled_opacity` as one widget over its backdrop ([`Self::backdrop`]), and
    /// takes no input.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// The colour under the radio button, which a disabled one is faded over as one widget
    /// ([`crate::fade::fade_as_one`]); the window's, `defaults.background_color`, where not
    /// given.
    pub fn backdrop(mut self, backdrop: egui::Color32) -> Self {
        self.backdrop = Some(backdrop);
        self
    }
}

/// Where egui's `RadioButton` painted its circle, its dot and its label
/// (`egui/src/widgets/radio_button.rs`): its icon atom `Spacing::icon_width` wide at the start
/// of its row, the circle and the dot `Spacing::icon_rectangles` of that atom, the dot a third
/// of the inner one in radius, the label `Spacing::icon_spacing` after the atom.
fn radio_parts(
    ui: &egui::Ui,
    response: &egui::Response,
    checked: bool,
    label: egui::Vec2,
) -> Parts {
    let rect = response.rect;
    let atom = egui::Rect::from_x_y_ranges(
        rect.left()..=rect.left() + ui.spacing().icon_width,
        rect.y_range(),
    );
    let (small, big) = ui.spacing().icon_rectangles(atom);
    let mut parts = Parts::default();
    parts.push("indicator", big);
    if checked {
        let radius = small.width() / EGUI_DOT_DIVISOR;
        parts.push(
            "dot",
            egui::Rect::from_center_size(small.center(), egui::Vec2::splat(radius + radius)),
        );
    }
    parts.push(
        "label",
        crate::parts::text_after(atom, ui.spacing().icon_spacing, label),
    );
    parts
}

/// One state of a radio of a 0.8.0 theme, laid over the scope's entry for it: each value where
/// the theme states it.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct Entry {
    /// `radio.indicator`'s fill: the circle, `bg_fill`.
    pub(crate) fill: Option<egui::Color32>,
    /// `radio.indicator`'s stroke width, 0 for a stroke stated as none: `bg_stroke.width`.
    pub(crate) stroke_width: Option<f32>,
    /// `radio.indicator`'s stroke colour and where it lies: `bg_stroke.color`.
    pub(crate) stroke: Option<(egui::Color32, StrokeAlign)>,
    /// `radio.dot`'s fill, checked: the dot, which egui paints in `fg_stroke.color`
    /// (`egui/src/widgets/radio_button.rs`).
    pub(crate) dot: Option<egui::Color32>,
}

/// What a radio of a 0.8.0 theme lays over the check box's cell its scope opens: its own homes
/// in place of the check box's, so a theme that styles the radio unlike the check box draws it
/// so.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Over {
    /// At rest: the scope's `inactive` entry.
    pub(crate) rest: Entry,
    /// Hovered: its `hovered` entry.
    pub(crate) hovered: Entry,
    /// Pressed: its `active` entry.
    pub(crate) pressed: Entry,
    /// `radio.label`'s colour, at rest: the label, on `override_text_color`, as the cell
    /// letters the check box's.
    pub(crate) label: Option<egui::Color32>,
    /// `radio.disabled_opacity`: the scope's `disabled_alpha`.
    pub(crate) disabled_opacity: Option<f32>,
}

impl Over {
    /// The radio of `r`, `checked` or not, `disabled` or not, at rest, hovered and pressed. A
    /// hovered or pressed value is read where a rule for that state states it (the part
    /// queried with the state's flag alone, spec §2, `from_state`); else it is taken as the
    /// connector takes an unstated state (spec §10, rule 0): egui's own where egui's own
    /// entries change it between rest and that state, else the radio's at rest. A disabled
    /// radio's values are read where a rule for `DISABLED` states them, the scope's disabled
    /// cell elsewhere (its disabled colours, faded as egui fades a disabled widget).
    pub(crate) fn next_of(r: &NextResolvedTheme, checked: bool, disabled: bool) -> Self {
        let radio = &r.radio;
        let mut base = WidgetState::NONE;
        if checked {
            base |= WidgetState::CHECKED;
        }
        if disabled {
            base |= WidgetState::DISABLED;
        }
        // Each read at `base | state`, where a rule for `ruled` alone states it (`NONE`: always).
        let entry = |state: WidgetState, rule: WidgetState| {
            let indicator = radio.indicator(base | state);
            let dot = radio.dot(base | state | WidgetState::CHECKED);
            let (ruled_dot, ruled) = (radio.dot(rule), radio.indicator(rule));
            let at = |part: &PartStyle, bit: PartStated| {
                rule == WidgetState::NONE || part.from_state.contains(bit)
            };
            let stroke = match (&indicator.stroke, stroke_color_of(&indicator)) {
                (NextStroke::Uniform { align, .. }, Some(colour)) => Some((colour, *align)),
                _ => None,
            };
            Entry {
                fill: at(&ruled, PartStated::FILL)
                    .then(|| fill_of(&indicator))
                    .flatten(),
                stroke_width: at(&ruled, PartStated::STROKE)
                    .then(|| stroke_width_of(&indicator))
                    .flatten(),
                stroke: at(&ruled, PartStated::STROKE).then_some(stroke).flatten(),
                dot: at(&ruled_dot, PartStated::FILL)
                    .then(|| fill_of(&dot))
                    .flatten(),
            }
        };
        let off = WidgetState::DISABLED;
        let (rest, hovered, pressed) = if disabled {
            (
                entry(WidgetState::NONE, off),
                Entry::default(),
                Entry::default(),
            )
        } else {
            (
                entry(WidgetState::NONE, WidgetState::NONE),
                entry(WidgetState::HOVER, WidgetState::HOVER),
                entry(WidgetState::PRESSED, WidgetState::PRESSED),
            )
        };
        let label = (!disabled || radio.label(off).from_state.contains(TextStated::COLOR))
            .then(|| text_color_of(&radio.label(base)))
            .flatten();
        Self {
            rest,
            hovered,
            pressed,
            label,
            disabled_opacity: radio.disabled_opacity,
        }
    }

    /// Lay the radio over the scope's entries, before it is faded. `stock` is egui's own entries
    /// for the scheme. A translucent stroke laid inside or across the circle's edge is
    /// composited over the entry's fill, as the connector hands egui the check box's
    /// (`native-theme-egui`, `style::line`): egui ends the circle's fill where its stroke
    /// begins.
    fn lay(&self, visuals: &mut egui::Visuals, stock: &egui::style::Widgets) {
        if let Some(label) = self.label {
            visuals.override_text_color = Some(label);
        }
        if let Some(opacity) = self.disabled_opacity.filter(|o| o.is_finite()) {
            visuals.disabled_alpha = unit_interval(opacity);
        }
        let Self {
            rest,
            hovered,
            pressed,
            ..
        } = *self;
        let w = &mut visuals.widgets;
        let stock_rest = &stock.inactive;
        for (entry, state, own) in [
            (&mut w.inactive, rest, stock_rest),
            (&mut w.hovered, hovered, &stock.hovered),
            (&mut w.active, pressed, &stock.active),
        ] {
            if let Some(fill) = pick(state.fill, rest.fill, stock_rest.bg_fill, own.bg_fill) {
                entry.bg_fill = fill;
            }
            if let Some(width) = pick(
                state.stroke_width,
                rest.stroke_width,
                stock_rest.bg_stroke.width,
                own.bg_stroke.width,
            ) {
                entry.bg_stroke.width = width;
            }
            let own_line =
                |e: &egui::style::WidgetVisuals| (e.bg_stroke.color, StrokeAlign::Outside);
            if let Some((colour, align)) = pick(
                state.stroke,
                rest.stroke,
                own_line(stock_rest),
                own_line(own),
            ) {
                entry.bg_stroke.color = laid_over(colour, align, entry.bg_fill);
            }
            if let Some(dot) = pick(
                state.dot,
                rest.dot,
                stock_rest.fg_stroke.color,
                own.fg_stroke.color,
            ) {
                entry.fg_stroke.color = dot;
            }
        }
    }
}

/// One property of a state's entry: as `stated`; where it is unstated and the radio states it
/// at rest (`at_rest`), egui's own value for the state where egui's own entries change it
/// between rest (`own_rest`) and that state (`own_state`), else the radio's at rest; `None`,
/// the scope's, where the radio states neither.
fn pick<T: PartialEq>(
    stated: Option<T>,
    at_rest: Option<T>,
    own_rest: T,
    own_state: T,
) -> Option<T> {
    stated.or_else(|| {
        at_rest.map(|at_rest| {
            if own_state == own_rest {
                at_rest
            } else {
                own_state
            }
        })
    })
}

/// A stroke's colour as egui is handed it over `fill`: as stated where it is opaque or lies
/// outside the edge; else composited over `fill` (inside) or over `fill` at half its alpha
/// (centred on the edge), as the connector's `Line::over` hands egui a part's stroke.
fn laid_over(colour: egui::Color32, align: StrokeAlign, fill: egui::Color32) -> egui::Color32 {
    if colour.a() == u8::MAX {
        return colour;
    }
    match align {
        StrokeAlign::Inside => fill.blend(colour),
        StrokeAlign::Center => fill.gamma_multiply(0.5).blend(colour),
        _ => colour,
    }
}

/// The colours a radio button reads of a theme, every colour read in one place, so the crate's
/// colour-leaf coverage test sees the widget's reads.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Look {
    /// `defaults.background_color`: the colour faded over where no backdrop is given; `None`,
    /// egui's own fade of a disabled widget.
    pub(crate) background: Option<Rgba>,
}

impl Look {
    pub(crate) fn of(t: &ResolvedTheme) -> Self {
        Self {
            background: Some(t.defaults.background_color),
        }
    }

    /// The colours of a radio button of a 0.8.0 theme: the window's background
    /// (`colors.window.background`), where stated.
    pub(crate) fn next_of(r: &NextResolvedTheme) -> Self {
        Self {
            background: next::window_background(r),
        }
    }
}

/// The sizes a radio button reads of a theme, each `None` where egui's own stays.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Geometry {
    /// `checkbox.radio_dot_diameter`: the dot.
    pub(crate) dot: Option<f32>,
    /// `checkbox.radio_indicator_width`: the circle.
    pub(crate) width: Option<f32>,
    /// The space between the circle and the label; `None`, the cell's `icon_spacing`.
    pub(crate) label_gap: Option<f32>,
}

impl Geometry {
    pub(crate) fn of(t: &ResolvedTheme) -> Self {
        let checkbox = &t.checkbox;
        Self {
            dot: checkbox.radio_dot_diameter.and_then(scope::length),
            width: checkbox.radio_indicator_width.and_then(scope::length),
            label_gap: None,
        }
    }

    /// The sizes of a radio button of a 0.8.0 theme: `radio.dot` and `radio.indicator`'s
    /// minimum widths at rest and `radio.label_gap`, each where stated and finite.
    pub(crate) fn next_of(r: &NextResolvedTheme) -> Self {
        let [dot, _] = r.radio.dot(WidgetState::NONE).min_size;
        let [width, _] = r.radio.indicator(WidgetState::NONE).min_size;
        Self {
            dot: dot.and_then(scope::length),
            width: width.and_then(scope::length),
            label_gap: scope::length(r.radio.label_gap),
        }
    }
}

impl egui::Widget for RadioButton {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let RadioButton {
            checked,
            text,
            enabled,
            backdrop,
        } = self;
        let Some(atlas) = ThemeAtlas::from_ctx(ui.ctx()) else {
            return ui.add_enabled(enabled, egui::RadioButton::new(checked, text));
        };
        let theme = ui.ctx().theme();
        let read = match atlas.resolved_for(theme) {
            Some(t) => Some((Look::of(t), Geometry::of(t), None)),
            None => atlas
                .next_resolved_for(theme)
                .map(|r| (Look::next_of(r), Geometry::next_of(r), Some(r))),
        };
        let Some((look, geometry, next)) = read else {
            return scope::open_faded(ui, Role::Checkbox, checked, enabled, None, |ui| {
                ui.add(egui::RadioButton::new(checked, text))
            });
        };
        let Geometry {
            dot,
            width,
            label_gap,
        } = geometry;
        let ground = backdrop.or_else(|| look.background.map(to_color32));
        let over = |ui: &mut egui::Ui, disabled: bool| {
            if let Some(r) = next {
                let stock = ui.ctx().theme().default_visuals().widgets;
                Over::next_of(r, checked, disabled).lay(ui.visuals_mut(), &stock);
            }
        };
        scope::open_faded_with(ui, Role::Checkbox, checked, enabled, ground, over, |ui| {
            if let Some(gap) = label_gap {
                ui.spacing_mut().icon_spacing = gap;
            }
            if let Some(width) = width {
                ui.spacing_mut().icon_width = clamp_length(width);
            }
            if let Some(dot) = dot {
                ui.spacing_mut().icon_width_inner = clamp_length(EGUI_DOT_DIVISOR * 0.5 * dot);
            }
            // The circle's outline inside the indicator's width, as the check box's is: egui
            // gives the circle half the icon's width plus the state's `expansion` as its radius
            // (`egui/src/widgets/radio_button.rs`) and epaint strokes a circle outside its radius
            // (`epaint/src/tessellator.rs:1531`), so each state's expansion gives back its
            // stroke's width.
            let widgets = &mut ui.style_mut().visuals.widgets;
            for state in [
                &mut widgets.noninteractive,
                &mut widgets.inactive,
                &mut widgets.hovered,
                &mut widgets.active,
                &mut widgets.open,
            ] {
                state.expansion -= state.bg_stroke.width;
            }
            let label = text
                .clone()
                .into_galley(
                    ui,
                    Some(egui::TextWrapMode::Extend),
                    f32::INFINITY,
                    egui::FontSelection::Default,
                )
                .size();
            let response = ui.add(egui::RadioButton::new(checked, text));
            radio_parts(ui, &response, checked, label).store(ui.ctx(), response.id);
            response
        })
    }
}
