//! `Switch` (spec §2.3): gpui-base's headless switch, track and thumb,
//! painted from `SwitchTheme`.

use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement, Pixels,
    RenderOnce, SharedString, StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _,
    px, rems,
};
use gpui_base::{LogicalStyledExt as _, Switch as BaseSwitch, SwitchThumb, SwitchTrack, spring};
use gpui_component::{ActiveTheme as _, Disableable as _, FocusableExt as _, StyledExt as _};
use native_theme::theme::ResolvedTheme;

use super::{Part, PartBounds, ThemeSource, color, faded, finite, length, over, part_bounds};
use crate::next::{Mapped, Unmapped, fill, hsla, radius, state_fill, stated};

/// The space between the track and the label, which `SwitchTheme` does not
/// state: gpui-component's own, `gap_2` (switch.rs:247).
pub(super) const LABEL_GAP: gpui::Rems = rems(0.5);

type ChangeHandler = Rc<dyn Fn(&bool, &mut Window, &mut App)>;

/// The group name the track's hover style listens to: the whole row is the
/// control the pointer hovers.
const HOVER_GROUP: &str = "native-theme-switch";

/// What a switch paints, per state, from `SwitchTheme`
/// (`docs/archive/todo_gpui-widgets-spec.md` §2.3).
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct SwitchLook {
    /// `switch.track_width`.
    pub track_width: Pixels,
    /// `switch.track_height`.
    pub track_height: Pixels,
    /// `switch.track_radius`.
    pub track_radius: Pixels,
    /// `switch.thumb_diameter`; off, `unchecked_thumb_diameter` where the
    /// theme states one.
    pub thumb: Pixels,
    /// The thumb's inset from the track's top: `0.5 × (track_height −
    /// thumb_diameter)`, the centred thumb the two numbers describe (negative
    /// where the thumb overhangs its track).
    pub inset: Pixels,
    /// The thumb's inset from either end of the track: [`inset`](Self::inset),
    /// or 0 where the thumb overhangs its track, which it then meets flush,
    /// as qqc2-desktop-style's handle does (docs/platform-facts.md §2.21).
    pub end_inset: Pixels,
    /// The track's colour in this state.
    pub track: Hsla,
    /// The track's colour under the pointer, where the control is enabled.
    pub hover_track: Option<Hsla>,
    /// The track's outline, drawn inside it: `switch.checked_track_border_color`
    /// or `unchecked_track_border_color`, disabled their `disabled_*` forms
    /// where the theme states them; `None` where the theme states no colour
    /// or no `track_border_line_width` (docs/platform-facts.md §2.21).
    pub track_border: Option<Hsla>,
    /// The track outline's width: `switch.track_border_line_width`, 0 where
    /// [`track_border`](Self::track_border) is `None`.
    pub track_border_width: Pixels,
    /// The thumb's colour: `switch.thumb_background`, off
    /// `unchecked_thumb_background` where the theme states one, disabled
    /// `disabled_thumb_color`.
    pub thumb_color: Hsla,
    /// The thumb's outline, drawn inside it: `switch.thumb_border_color`;
    /// `None` where the theme states no colour or no
    /// `thumb_border_line_width`.
    pub thumb_border: Option<Hsla>,
    /// The thumb outline's width: `switch.thumb_border_line_width`, 0 where
    /// [`thumb_border`](Self::thumb_border) is `None`.
    pub thumb_border_width: Pixels,
    /// The label's colour: `SwitchTheme` states none, so the window's text,
    /// `defaults.text_color`, and `defaults.disabled_text_color` disabled.
    pub label: Hsla,
    /// The whole control's opacity: `switch.disabled_opacity` when disabled,
    /// on top of the disabled colours (docs/platform-facts.md §2.1.6), else 1.
    pub opacity: f32,
}

impl SwitchLook {
    /// The look of a switch `checked` or not, `disabled` or not, or `None`
    /// when a length the theme gives is not finite.
    ///
    /// The hover and disabled colours are soft options, as the iced connector
    /// reads them (`native-theme-iced` `styles::toggler`): a `None` disabled
    /// colour copies the colour it stands in for (C16), and a `None` hover
    /// leaves the track at rest. An outline is drawn where the theme states
    /// both its colour and its width.
    #[must_use]
    pub fn of(resolved: &ResolvedTheme, checked: bool, disabled: bool) -> Option<Self> {
        let s = &resolved.switch;
        let base = if checked {
            s.checked_background
        } else {
            s.unchecked_background
        };
        let track = match (checked, disabled) {
            (true, true) => s.disabled_checked_background.unwrap_or(base),
            (false, true) => s.disabled_unchecked_background.unwrap_or(base),
            _ => base,
        };
        let hover = if checked {
            s.hover_checked_background
        } else {
            s.hover_unchecked_background
        };
        let hover_track =
            (!disabled).then(|| hover.map_or(color(base), |h| over(color(base), color(h))));
        let rest_thumb = if checked {
            s.thumb_background
        } else {
            s.unchecked_thumb_background.unwrap_or(s.thumb_background)
        };
        let thumb_color = if disabled {
            s.disabled_thumb_color.unwrap_or(rest_thumb)
        } else {
            rest_thumb
        };
        let rest_track_border = if checked {
            s.checked_track_border_color
        } else {
            s.unchecked_track_border_color
        };
        let track_border = match (checked, disabled) {
            (true, true) => s.disabled_checked_track_border_color.or(rest_track_border),
            (false, true) => s
                .disabled_unchecked_track_border_color
                .or(rest_track_border),
            _ => rest_track_border,
        };
        let outline = |colour: Option<native_theme::color::Rgba>, width: Option<f32>| match colour
            .zip(width)
        {
            Some((colour, width)) => length(width).map(|width| (Some(color(colour)), width)),
            None => Some((None, px(0.))),
        };
        let (track_border, track_border_width) = outline(track_border, s.track_border_line_width)?;
        let (thumb_border, thumb_border_width) =
            outline(s.thumb_border_color, s.thumb_border_line_width)?;
        let label = if disabled {
            resolved.defaults.disabled_text_color
        } else {
            resolved.defaults.text_color
        };
        let track_height = length(s.track_height)?;
        let thumb = length(if checked {
            s.thumb_diameter
        } else {
            s.unchecked_thumb_diameter.unwrap_or(s.thumb_diameter)
        })?;
        Some(Self {
            track_width: length(s.track_width)?,
            track_height,
            track_radius: length(s.track_radius)?,
            thumb,
            inset: px((f32::from(track_height) - f32::from(thumb)) / 2.),
            end_inset: px(((f32::from(track_height) - f32::from(thumb)) / 2.).max(0.)),
            track: color(track),
            hover_track,
            track_border,
            track_border_width,
            thumb_color: color(thumb_color),
            thumb_border,
            thumb_border_width,
            label: color(label),
            opacity: super::disabled_opacity(disabled, s.disabled_opacity),
        })
    }

    /// The look of a switch of a 0.8.0 theme, `checked` or not, `disabled` or
    /// not, each field read at its home: the track and the thumb at the
    /// switch's state (`switch.track` and `switch.thumb` at `CHECKED` where
    /// checked and `DISABLED` where disabled, where a theme without a rule
    /// for a disabled state gives the enabled part, the disabled colour's
    /// `None`), the hovered track's fill where a rule states it, the track's
    /// and the thumb's strokes (the colour at the state, the width at rest; no
    /// stroke is no outline), the track's minimum size and corners, the
    /// thumb's minimum width at the checked state, the window's text colour
    /// (`colors.window.foreground`, and `colors.disabled_text` disabled) and
    /// `switch.disabled_opacity`.
    ///
    /// A checked track's hover is read at `HOVER|CHECKED`, where a theme with
    /// no rule for the hover gives the checked track (the table of homes,
    /// `switch.hover_checked_background`, `null_reference`), so it is a
    /// hover layer where it differs from that track.
    ///
    /// # Errors
    ///
    /// [`Unmapped`] where the theme leaves one of them
    /// unstated, states it in a form gpui cannot draw, or gives a length that
    /// is not finite.
    pub fn next_of(
        r: &native_theme::next::ResolvedTheme,
        checked: bool,
        disabled: bool,
    ) -> Mapped<Self> {
        use native_theme::next::WidgetState;
        // The home `switch.<part>(<state>).<property>` at the switch's
        // state, for `Unmapped`.
        macro_rules! home {
            ($checked:expr, $disabled:expr, $part:literal, $prop:literal) => {
                match ($checked, $disabled) {
                    (false, false) => concat!("switch.", $part, "(NONE).", $prop),
                    (true, false) => concat!("switch.", $part, "(CHECKED).", $prop),
                    (false, true) => concat!("switch.", $part, "(DISABLED).", $prop),
                    (true, true) => concat!("switch.", $part, "(DISABLED|CHECKED).", $prop),
                }
            };
        }
        let s = &r.switch;
        let rest = if checked {
            WidgetState::CHECKED
        } else {
            WidgetState::NONE
        };
        let state = if disabled {
            rest | WidgetState::DISABLED
        } else {
            rest
        };
        let base = fill(&s.track(rest), home!(checked, false, "track", "fill"))?;
        let hover = if checked {
            let hovered = fill(
                &s.track(WidgetState::HOVER | WidgetState::CHECKED),
                "switch.track(HOVER|CHECKED).fill",
            )?;
            (hovered != base).then_some(hovered)
        } else {
            state_fill(&s.track(WidgetState::HOVER), "switch.track(HOVER).fill")?
        };
        let track = s.track(WidgetState::NONE);
        let thumb = s.thumb(rest);
        let label = if disabled {
            hsla(r.colors.disabled_text)
        } else {
            hsla(stated(
                r.colors.window.foreground,
                "colors.window.foreground",
            )?)
        };
        finite(
            Self::from_reads(
                &SwitchReads {
                    base,
                    track: fill(&s.track(state), home!(checked, disabled, "track", "fill"))?,
                    hover,
                    thumb: fill(&s.thumb(state), home!(checked, disabled, "thumb", "fill"))?,
                    track_border: outline_color(
                        &s.track(state),
                        home!(checked, disabled, "track", "stroke.color"),
                    )?,
                    track_border_width: outline_width(&track, "switch.track(NONE).stroke.width")?,
                    thumb_border: outline_color(
                        &thumb,
                        home!(checked, false, "thumb", "stroke.color"),
                    )?,
                    thumb_border_width: outline_width(
                        &s.thumb(WidgetState::NONE),
                        "switch.thumb(NONE).stroke.width",
                    )?,
                    label,
                    track_width: stated(track.min_size[0], "switch.track(NONE).min_size[0]")?,
                    track_height: stated(track.min_size[1], "switch.track(NONE).min_size[1]")?,
                    track_radius: radius(&track, "switch.track(NONE).corners")?,
                    thumb_diameter: stated(
                        thumb.min_size[0],
                        home!(checked, false, "thumb", "min_size[0]"),
                    )?,
                    disabled_opacity: s.disabled_opacity,
                },
                disabled,
            ),
            "switch.track(NONE).min_size",
        )
    }

    fn from_reads(s: &SwitchReads, disabled: bool) -> Option<Self> {
        let hover_track = (!disabled).then(|| s.hover.map_or(s.base, |hover| over(s.base, hover)));
        let outline = |colour: Option<Hsla>, width: Option<f32>| match colour.zip(width) {
            Some((colour, width)) => length(width).map(|width| (Some(colour), width)),
            None => Some((None, px(0.))),
        };
        let (track_border, track_border_width) = outline(s.track_border, s.track_border_width)?;
        let (thumb_border, thumb_border_width) = outline(s.thumb_border, s.thumb_border_width)?;
        let track_height = length(s.track_height)?;
        let thumb = length(s.thumb_diameter)?;
        Some(Self {
            track_width: length(s.track_width)?,
            track_height,
            track_radius: length(s.track_radius)?,
            thumb,
            inset: px((f32::from(track_height) - f32::from(thumb)) / 2.),
            end_inset: px(((f32::from(track_height) - f32::from(thumb)) / 2.).max(0.)),
            track: s.track,
            hover_track,
            track_border,
            track_border_width,
            thumb_color: s.thumb,
            thumb_border,
            thumb_border_width,
            label: s.label,
            opacity: super::next_disabled_opacity(disabled, s.disabled_opacity),
        })
    }

    /// This look faded as one control over `ground`, the colour it is drawn
    /// on: where [`opacity`](Self::opacity) is below 1, the track composited
    /// over `ground`, the thumb and both outlines over the track and the
    /// label over `ground`,
    /// each faded by it over `ground`, the opacity then 1. As libadwaita
    /// fades a disabled switch, `filter: Opacity(..)` on the whole widget
    /// (docs/platform-facts.md §2.1.6), so the thumb keeps its colour on the
    /// track and the pair fades over the page, where fading each part on its
    /// own would fade the thumb over the faded track.
    #[must_use]
    pub fn faded_over(self, ground: Hsla) -> Self {
        if self.opacity >= 1. {
            return self;
        }
        let a = self.opacity;
        let track = over(ground, self.track);
        Self {
            track: faded(ground, ground, self.track, a),
            track_border: self.track_border.map(|edge| faded(ground, track, edge, a)),
            thumb_color: faded(ground, track, self.thumb_color, a),
            thumb_border: self.thumb_border.map(|edge| faded(ground, track, edge, a)),
            label: faded(ground, ground, self.label, a),
            opacity: 1.,
            ..self
        }
    }

    /// The thumb's start edge, from the track's start edge (the left, or the
    /// right in a right-to-left layout), at rest `checked` or not.
    #[must_use]
    pub fn thumb_left(&self, checked: bool) -> Pixels {
        if checked {
            px(f32::from(self.track_width) - f32::from(self.end_inset) - f32::from(self.thumb))
        } else {
            self.end_inset
        }
    }
}

/// The values [`SwitchLook::next_of`] builds the look from: the track at rest
/// in the checked state and at the switch's state, its hover layer, the thumb
/// at the switch's state, the outlines' colours and widths (an outline where
/// both are stated), the label's colour and the sizes.
struct SwitchReads {
    base: Hsla,
    track: Hsla,
    hover: Option<Hsla>,
    thumb: Hsla,
    track_border: Option<Hsla>,
    track_border_width: Option<f32>,
    thumb_border: Option<Hsla>,
    thumb_border_width: Option<f32>,
    label: Hsla,
    track_width: f32,
    track_height: f32,
    track_radius: f32,
    thumb_diameter: f32,
    disabled_opacity: Option<f32>,
}

/// A part's stroke colour, where stated as one stroke; `None` where it has
/// none or the theme states none (no outline, as gpui-component's switch
/// draws none).
fn outline_color(part: &native_theme::next::PartStyle, home: &'static str) -> Mapped<Option<Hsla>> {
    use native_theme::next::{PartStated, Stroke};
    if !part.stated.contains(PartStated::STROKE) {
        return Ok(None);
    }
    match &part.stroke {
        Stroke::None => Ok(None),
        Stroke::Uniform { color, .. } => Ok(Some(hsla(*color))),
        _ => Err(Unmapped(home)),
    }
}

/// A part's stroke width, where stated as one stroke; `None` where it has
/// none or the theme states none.
fn outline_width(part: &native_theme::next::PartStyle, home: &'static str) -> Mapped<Option<f32>> {
    use native_theme::next::{PartStated, Stroke};
    if !part.stated.contains(PartStated::STROKE) {
        return Ok(None);
    }
    match &part.stroke {
        Stroke::None => Ok(None),
        Stroke::Uniform { width, .. } => Ok(Some(*width)),
        _ => Err(Unmapped(home)),
    }
}

/// A switch whose track, thumb and states are `SwitchTheme`'s
/// (`docs/archive/todo_gpui-widgets-spec.md` §2.3),
/// on gpui-base's headless `Switch`: click, Enter and Space toggle it, a
/// disabled one is inert, and it reports itself as a switch with its toggled
/// state. The thumb travels on gpui-component's spring (switch.rs:216-225).
/// While focused, the track draws the theme's focus ring
/// (`defaults.focus_ring_color`, `focus_ring_width` and `focus_ring_offset`,
/// as the other theme-drawn controls draw it; where the theme says keyboard
/// only, for a focus that followed a key press), where gpui-component's switch
/// draws its own (switch.rs:280-283).
///
/// It is laid out from the start edge of the layout direction
/// (`LayoutDirection::global`), the right in a right-to-left layout, as
/// gpui-component's switch is (gpui-component 0.10.0 switch.rs, `label_side`
/// and `inset_s`): the label after the track, and the thumb off at the
/// track's start end and on at its far end.
///
/// Controlled: [`Switch::on_change`] receives the requested value. Its
/// `Styled` style refines the row, after the widget's own: the row's layout,
/// box and spacing; the track and the label set their own colours and text.
/// Without a native theme it renders gpui-component's `Switch`.
pub struct Switch {
    id: ElementId,
    label: Option<SharedString>,
    checked: bool,
    disabled: bool,
    on_change: Option<ChangeHandler>,
    observer: Option<PartBounds>,
    backdrop: Option<Hsla>,
    accessibility_label: Option<SharedString>,
    tab_stop: bool,
    tab_index: isize,
    focus_ring: bool,
    style: StyleRefinement,
}

impl Switch {
    /// A new switch, off: a tab stop at index 0, as gpui-base's
    /// (switch.rs:256-257), that draws the focus ring, as gpui-component's
    /// (switch.rs:49-51).
    #[must_use]
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            checked: false,
            disabled: false,
            on_change: None,
            observer: None,
            backdrop: None,
            accessibility_label: None,
            tab_stop: true,
            tab_index: 0,
            focus_ring: true,
            style: StyleRefinement::default(),
        }
    }

    /// The name a screen reader announces, where it is not the label drawn
    /// beside the track or no label is drawn; the drawn label stays as it
    /// is.
    #[must_use]
    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.accessibility_label = Some(label.into());
        self
    }

    /// Whether Tab stops on it; `true` by default.
    #[must_use]
    pub fn tab_stop(mut self, tab_stop: bool) -> Self {
        self.tab_stop = tab_stop;
        self
    }

    /// Its place in its tab group's Tab order; `0` by default.
    #[must_use]
    pub fn tab_index(mut self, tab_index: isize) -> Self {
        self.tab_index = tab_index;
        self
    }

    /// Whether the track draws the focus ring while focused; `true` by
    /// default, as gpui-component's `FocusableExt::focus_ring`.
    #[must_use]
    pub fn focus_ring(mut self, focus_ring: bool) -> Self {
        self.focus_ring = focus_ring;
        self
    }

    /// The colour under the switch, which a disabled one is faded over as
    /// one control ([`SwitchLook::faded_over`]); the window's,
    /// `defaults.background_color`, where not given.
    #[must_use]
    pub fn backdrop(mut self, backdrop: Hsla) -> Self {
        self.backdrop = Some(backdrop);
        self
    }

    /// Hands `observer` the bounds of the track, the thumb and the label as
    /// a frame lays them out ([`PartBounds`]).
    #[must_use]
    pub fn on_part_bounds(mut self, observer: PartBounds) -> Self {
        self.observer = Some(observer);
        self
    }

    /// The label drawn beside the track, and the name a screen reader
    /// announces where [`Switch::accessibility_label`] gives none.
    #[must_use]
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Whether it is on.
    #[must_use]
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Whether it is disabled.
    #[must_use]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Called with the requested value on activation.
    #[must_use]
    pub fn on_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    fn fallback(self) -> AnyElement {
        gpui_component::switch::Switch::new(self.id)
            .when_some(self.label, |switch, label| switch.label(label))
            .when_some(self.accessibility_label, |switch, label| {
                switch.accessibility_label(label)
            })
            .checked(self.checked)
            .disabled(self.disabled)
            .tab_stop(self.tab_stop)
            .tab_index(self.tab_index)
            .focus_ring(self.focus_ring)
            .refine_style(&self.style)
            .when_some(self.on_change, |switch, on_change| {
                switch.on_click(move |checked, window, cx| on_change(checked, window, cx))
            })
            .into_any_element()
    }
}

impl Styled for Switch {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Switch {
    /// The theme-drawn row on gpui-base's switch, or the switch back where
    /// gpui-component's switch is drawn instead.
    pub(super) fn row(self, window: &mut Window, cx: &mut App) -> Result<BaseSwitch, Box<Self>> {
        let (checked, disabled) = (self.checked, self.disabled);
        let Some(theme) = ThemeSource::of(cx) else {
            return Err(Box::new(self));
        };
        let looks = theme.look(
            |r| SwitchLook::of(r, checked, disabled),
            |r| SwitchLook::next_of(r, checked, disabled),
        );
        let ground = self.backdrop.or_else(|| theme.background());
        let (Some(look), Some(ground), Some((label_size, label_line, label_weight))) =
            (looks, ground, theme.body_text())
        else {
            return Err(Box::new(self));
        };
        let look = look.faded_over(ground);
        let observer = self.observer.as_ref();
        let keyboard_only = theme.keyboard_only();
        let thumb_left = spring(
            (self.id.clone(), "thumb"),
            look.thumb_left(checked),
            cx.theme().motion_tokens().spring_move,
            window,
            cx,
        );
        // The switch's own focus handle, handed to the base switch as its one
        // tab stop (gpui-base switch.rs, `Switch::track_focus`), so the track
        // knows when to draw the ring -- the `Checkbox` pattern.
        let focus_handle = window
            .use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        let focused = super::ring_shows(
            (self.id.clone(), "focus-visible"),
            focus_handle.is_focused(window),
            keyboard_only,
            window,
            cx,
        );
        let track = self.track(&look, focused, thumb_left, window, cx);
        Ok(BaseSwitch::new(self.id)
            .checked(checked)
            .disabled(self.disabled)
            .track_focus(&focus_handle)
            .tab_stop(self.tab_stop)
            .tab_index(self.tab_index)
            .when_some(
                self.accessibility_label.or_else(|| self.label.clone()),
                |switch, label| switch.accessibility_label(label),
            )
            .when_some(self.on_change, |switch, on_change| {
                switch.on_change(move |next, _, window, cx| on_change(&next, window, cx))
            })
            .debug_selector(|| "native-switch-row".into())
            .group(HOVER_GROUP)
            .flex_row_dir(cx)
            .items_center()
            .gap(LABEL_GAP)
            .opacity(look.opacity)
            // The caller's style, refined onto the row after the widget's
            // own, as gpui-component refines its checkbox's (checkbox.rs:294);
            // its switch refines it onto a box round the row (switch.rs:228).
            .refine_style(&self.style)
            .child(track)
            .when_some(self.label, |switch, label| {
                switch.child(
                    div()
                        // One line of the label's font, its size by
                        // `defaults.line_height`, the line box the model
                        // states, centred on the track -- where
                        // gpui-component makes the line box the track's
                        // height (switch.rs:301).
                        .line_height(gpui::relative(label_line))
                        .text_size(label_size)
                        .font_weight(label_weight)
                        .text_color(look.label)
                        .debug_selector(|| "native-switch-label".into())
                        .child(label)
                        .children(observer.map(|o| part_bounds(Part::Label, o, px(0.)))),
                )
            }))
    }
}

impl Switch {
    /// The track in `look`, its thumb `thumb_left` from its start edge; while
    /// `focused` (a focus the theme rings, `ring_shows`), enabled, and told to
    /// (`focus_ring`), it draws the focus ring.
    pub(super) fn track(
        &self,
        look: &SwitchLook,
        focused: bool,
        thumb_left: Pixels,
        window: &Window,
        cx: &App,
    ) -> SwitchTrack {
        let observer = self.observer.as_ref();
        let ring = (focused && !self.disabled && self.focus_ring)
            .then(|| ThemeSource::of(cx).and_then(ThemeSource::focus_ring))
            .flatten();
        SwitchTrack::new((self.id.clone(), "track"))
            .checked(self.checked)
            .disabled(self.disabled)
            .relative()
            .flex_none()
            .w(look.track_width)
            .h(look.track_height)
            .rounded(look.track_radius)
            .bg(look.track)
            .when_some(look.hover_track, |track, hover| {
                track.group_hover(HOVER_GROUP, move |style| style.bg(hover))
            })
            .debug_selector(|| "native-switch-track".into())
            // The outline, a box of its own under the thumb: gpui paints a
            // box's border after its children, so on the track it would
            // cross the thumb, which qqc2-desktop-style lays over its
            // track's outline (SwitchIndicator.qml:38-79).
            .when_some(look.track_border, |track, edge| {
                track.child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .w(look.track_width)
                        .h(look.track_height)
                        .rounded(look.track_radius)
                        .border(look.track_border_width)
                        .border_color(edge),
                )
            })
            .child(
                SwitchThumb::new(self.checked)
                    .disabled(self.disabled)
                    .absolute()
                    .top(look.inset)
                    .inset_s(thumb_left, cx)
                    .size(look.thumb)
                    .rounded_full()
                    .bg(look.thumb_color)
                    .when_some(look.thumb_border, |thumb, edge| {
                        thumb.border(look.thumb_border_width).border_color(edge)
                    })
                    .child(
                        // The thumb's whole box, its outline too.
                        div()
                            .absolute()
                            .top(px(-f32::from(look.thumb_border_width)))
                            .left(px(-f32::from(look.thumb_border_width)))
                            .size(look.thumb)
                            .debug_selector(|| "native-switch-thumb".into())
                            .children(observer.map(|o| part_bounds(Part::Thumb, o, px(0.)))),
                    ),
            )
            .children(observer.map(|o| part_bounds(Part::Track, o, px(0.))))
            // Last, so it paints over the outline and the thumb.
            .when_some(ring, |track, ring| ring.draw(track, window))
    }
}

impl RenderOnce for Switch {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        match self.row(window, cx) {
            Ok(row) => row.into_any_element(),
            Err(switch) => switch.fallback(),
        }
    }
}
