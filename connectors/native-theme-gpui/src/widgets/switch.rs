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
use crate::next::{
    Mapped, Unmapped, fill, hsla, radius, state_fill, stated, text_color, text_color_of,
};

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
    /// `defaults.text_color`, and `defaults.disabled_text_color` disabled;
    /// of a 0.8.0 theme, `switch.label`'s at the switch's state
    /// ([`SwitchLook::next_label_home`]).
    pub label: Hsla,
    /// The space between the track and the label: `switch.label_gap` where
    /// a 0.8.0 theme states it; `None`, gpui-component's own `gap_2`
    /// (gpui-component 0.10.0 switch.rs, `Switch::render`), where the theme
    /// states none (`SwitchTheme` has no field for it).
    pub label_gap: Option<Pixels>,
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
            label_gap: None,
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
    /// thumb's minimum width at the checked state, the label's colour
    /// ([`SwitchLook::next_label_home`]), `switch.label_gap` where stated
    /// and `switch.disabled_opacity`.
    ///
    /// A checked track's hover is read at `HOVER|CHECKED` where a rule states
    /// the hovered track's fill, as the query with that flag alone says
    /// (`switch.track(HOVER).from_state`, spec §2): a value stated equal to
    /// the checked track's is a hover layer too, and none is drawn where no
    /// rule states one.
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
        use native_theme::next::{PartStated, WidgetState};
        // The home of a property at the switch's state, for `Unmapped`:
        // each spelt whole, so a claim can cite the line that names it.
        let at = |[none, on, off, off_on]: [&'static str; 4]| match (checked, disabled) {
            (false, false) => none,
            (true, false) => on,
            (false, true) => off,
            (true, true) => off_on,
        };
        let at_rest = |none: &'static str, on: &'static str| if checked { on } else { none };
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
        let base = fill(
            &s.track(rest),
            at_rest("switch.track(NONE).fill", "switch.track(CHECKED).fill"),
        )?;
        let hover = if checked {
            if s.track(WidgetState::HOVER)
                .from_state
                .contains(PartStated::FILL)
            {
                Some(fill(
                    &s.track(WidgetState::HOVER | WidgetState::CHECKED),
                    "switch.track(HOVER|CHECKED).fill",
                )?)
            } else {
                None
            }
        } else {
            state_fill(&s.track(WidgetState::HOVER), "switch.track(HOVER).fill")?
        };
        let track = s.track(WidgetState::NONE);
        let thumb = s.thumb(rest);
        let (label, _) = next_label(r, checked, disabled)?;
        let label_gap = s
            .label_gap
            .map(|gap| length(gap).ok_or(Unmapped("switch.label_gap")))
            .transpose()?;
        finite(
            Self::from_reads(
                &SwitchReads {
                    base,
                    track: fill(
                        &s.track(state),
                        at([
                            "switch.track(NONE).fill",
                            "switch.track(CHECKED).fill",
                            "switch.track(DISABLED).fill",
                            "switch.track(DISABLED|CHECKED).fill",
                        ]),
                    )?,
                    hover,
                    thumb: fill(
                        &s.thumb(state),
                        at([
                            "switch.thumb(NONE).fill",
                            "switch.thumb(CHECKED).fill",
                            "switch.thumb(DISABLED).fill",
                            "switch.thumb(DISABLED|CHECKED).fill",
                        ]),
                    )?,
                    track_border: outline_color(
                        &s.track(state),
                        at([
                            "switch.track(NONE).stroke.color",
                            "switch.track(CHECKED).stroke.color",
                            "switch.track(DISABLED).stroke.color",
                            "switch.track(DISABLED|CHECKED).stroke.color",
                        ]),
                    )?,
                    track_border_width: outline_width(&track, "switch.track(NONE).stroke.width")?,
                    thumb_border: outline_color(
                        &thumb,
                        at_rest(
                            "switch.thumb(NONE).stroke.color",
                            "switch.thumb(CHECKED).stroke.color",
                        ),
                    )?,
                    thumb_border_width: outline_width(
                        &s.thumb(WidgetState::NONE),
                        "switch.thumb(NONE).stroke.width",
                    )?,
                    label,
                    label_gap,
                    track_width: stated(track.min_size[0], "switch.track(NONE).min_size[0]")?,
                    track_height: stated(track.min_size[1], "switch.track(NONE).min_size[1]")?,
                    track_radius: radius(&track, "switch.track(NONE).corners")?,
                    thumb_diameter: stated(
                        thumb.min_size[0],
                        at_rest(
                            "switch.thumb(NONE).min_size[0]",
                            "switch.thumb(CHECKED).min_size[0]",
                        ),
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
            label_gap: s.label_gap,
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

    /// The home [`SwitchLook::next_of`] reads the label's colour at, for a
    /// switch of a 0.8.0 theme `checked` or not, `disabled` or not:
    /// `switch.label(<state>).color` where the theme states it (enabled, at
    /// rest or by a rule; disabled, where a rule states it for `DISABLED`,
    /// as the query with that flag alone says, spec §2); else
    /// gpui-component's own label colour (spec §10, rule 0), the window's
    /// text its switch inherits, `foreground`, which [`crate::next::apply`]
    /// fills from `colors.window.foreground`, and disabled
    /// `muted_foreground` (gpui-component 0.10.0 switch.rs,
    /// `Switch::render`), filled from `colors.window.foreground_muted`.
    ///
    /// # Errors
    ///
    /// [`Unmapped`] where the colour falls to gpui-component's own and the
    /// theme states no colour at that home.
    pub fn next_label_home(
        r: &native_theme::next::ResolvedTheme,
        checked: bool,
        disabled: bool,
    ) -> Mapped<SwitchLabelHome> {
        next_label(r, checked, disabled).map(|(_, home)| home)
    }
}

/// Where a switch of a 0.8.0 theme takes its label's colour
/// ([`SwitchLook::next_label_home`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SwitchLabelHome {
    /// `switch.label(NONE).color`.
    Rest,
    /// `switch.label(CHECKED).color`.
    Checked,
    /// `switch.label(DISABLED).color`.
    Disabled,
    /// `switch.label(DISABLED|CHECKED).color`.
    DisabledChecked,
    /// gpui-component's own, `foreground`, from `colors.window.foreground`.
    WindowForeground,
    /// gpui-component's own disabled, `muted_foreground`, from
    /// `colors.window.foreground_muted`.
    WindowForegroundMuted,
}

impl SwitchLabelHome {
    /// The home, spelt whole.
    #[must_use]
    pub fn path(self) -> &'static str {
        match self {
            Self::Rest => "switch.label(NONE).color",
            Self::Checked => "switch.label(CHECKED).color",
            Self::Disabled => "switch.label(DISABLED).color",
            Self::DisabledChecked => "switch.label(DISABLED|CHECKED).color",
            Self::WindowForeground => "colors.window.foreground",
            Self::WindowForegroundMuted => "colors.window.foreground_muted",
        }
    }
}

/// The label's colour of a switch of a 0.8.0 theme and its home
/// ([`SwitchLook::next_label_home`]). Disabled, a rule stated for
/// `DISABLED` is asked with that flag alone first, as
/// [`CheckboxLook::next_sized`](super::CheckboxLook::next_sized) asks the
/// disabled fill: a colour stated for `CHECKED` alone is not a disabled
/// colour, and gpui-component's own `muted_foreground` stands.
fn next_label(
    r: &native_theme::next::ResolvedTheme,
    checked: bool,
    disabled: bool,
) -> Mapped<(Hsla, SwitchLabelHome)> {
    use native_theme::next::{TextStated, WidgetState};
    let shown = if checked {
        WidgetState::CHECKED
    } else {
        WidgetState::NONE
    };
    let home = if disabled {
        let ruled = r
            .switch
            .label(WidgetState::DISABLED)
            .from_state
            .contains(TextStated::COLOR);
        match (ruled, checked) {
            (true, false) => SwitchLabelHome::Disabled,
            (true, true) => SwitchLabelHome::DisabledChecked,
            (false, _) => SwitchLabelHome::WindowForegroundMuted,
        }
    } else {
        match (text_color_of(&r.switch.label(shown)).is_some(), checked) {
            (true, false) => SwitchLabelHome::Rest,
            (true, true) => SwitchLabelHome::Checked,
            (false, _) => SwitchLabelHome::WindowForeground,
        }
    };
    let colour = match home {
        SwitchLabelHome::WindowForeground => hsla(stated(
            r.colors.window.foreground,
            "colors.window.foreground",
        )?),
        SwitchLabelHome::WindowForegroundMuted => hsla(stated(
            r.colors.window.foreground_muted,
            "colors.window.foreground_muted",
        )?),
        SwitchLabelHome::Disabled | SwitchLabelHome::DisabledChecked => {
            text_color(&r.switch.label(WidgetState::DISABLED | shown), home.path())?
        }
        SwitchLabelHome::Rest | SwitchLabelHome::Checked => {
            text_color(&r.switch.label(shown), home.path())?
        }
    };
    Ok((colour, home))
}

/// The font a switch of a 0.8.0 theme letters its label in, each property
/// where `switch.label(NONE)` states it, else gpui-component's own (spec
/// §10, rule 0): its switch's label sets no family, weight or slant, so
/// the window's stand, and is `input_text_size(Size::Medium)`, `text_sm`
/// (gpui-component 0.10.0 switch.rs, `Switch::render`). One line
/// `typography.line_height_multiplier` tall, as the theme-drawn checkbox's
/// label.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SwitchLabelFont {
    /// `switch.label(NONE).family`, where stated.
    pub family: Option<SharedString>,
    /// `switch.label(NONE).size_px`, where stated; `None`, `text_sm`.
    pub size: Option<Pixels>,
    /// `switch.label(NONE).weight`, where stated.
    pub weight: Option<gpui::FontWeight>,
    /// `switch.label(NONE).style`, where stated.
    pub style: Option<gpui::FontStyle>,
    /// `typography.line_height_multiplier`, a multiple of the size.
    pub line_height: f32,
}

impl SwitchLabelFont {
    /// The label's font of a switch of a 0.8.0 theme.
    ///
    /// # Errors
    ///
    /// [`Unmapped`] where the theme states a size that is not finite or not
    /// positive, or a slant gpui has no form for.
    pub fn next_of(r: &native_theme::next::ResolvedTheme) -> Mapped<Self> {
        use native_theme::next::{TextStated, WidgetState};
        let text = r.switch.label(WidgetState::NONE);
        let has = |bit: TextStated| text.stated.contains(bit);
        let size = if has(TextStated::SIZE_PX) {
            Some(
                length(text.size_px)
                    .filter(|size| *size > px(0.))
                    .ok_or(Unmapped("switch.label(NONE).size_px"))?,
            )
        } else {
            None
        };
        let style = if has(TextStated::STYLE) {
            Some(stated(
                crate::next::font_style(text.style),
                "switch.label(NONE).style",
            )?)
        } else {
            None
        };
        Ok(Self {
            family: has(TextStated::FAMILY).then(|| crate::font_family(&text.family)),
            size,
            weight: has(TextStated::WEIGHT).then(|| gpui::FontWeight(f32::from(text.weight))),
            style,
            line_height: r.typography.line_height_multiplier,
        })
    }

    /// The text of a label in this font, one line tall.
    fn text(&self) -> gpui::Div {
        let text = div().line_height(gpui::relative(self.line_height));
        let mut text = match self.size {
            Some(size) => text.text_size(size),
            None => text.text_sm(),
        };
        if let Some(family) = &self.family {
            text = text.font_family(family.clone());
        }
        if let Some(weight) = self.weight {
            text = text.font_weight(weight);
        }
        if let Some(style) = self.style {
            text.text_style().font_style = Some(style);
        }
        text
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
    label_gap: Option<Pixels>,
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
        // One line of the label's font, the line box the model states,
        // centred on the track -- where gpui-component makes the line box the
        // track's height (switch.rs:301): of a 0.7.2 theme, which states no
        // label font, the body text's size and weight, by
        // `defaults.line_height`; of a 0.8.0 one, `switch.label(NONE)`'s
        // ([`SwitchLabelFont`]).
        let text = match theme {
            ThemeSource::Old(_) => theme.body_text().map(|(size, line, weight)| {
                div()
                    .line_height(gpui::relative(line))
                    .text_size(size)
                    .font_weight(weight)
            }),
            ThemeSource::Next(r) => SwitchLabelFont::next_of(r).ok().map(|font| font.text()),
        };
        let (Some(look), Some(ground), Some(text)) = (looks, ground, text) else {
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
            .map(|row| match look.label_gap {
                Some(gap) => row.gap(gap),
                None => row.gap(LABEL_GAP),
            })
            .opacity(look.opacity)
            // The caller's style, refined onto the row after the widget's
            // own, as gpui-component refines its checkbox's (checkbox.rs:294);
            // its switch refines it onto a box round the row (switch.rs:228).
            .refine_style(&self.style)
            .child(track)
            .when_some(self.label, |switch, label| {
                switch.child(
                    text.text_color(look.label)
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

#[cfg(test)]
mod tests {
    use native_theme::next::{
        Color, Preset, ResolvedTheme, Scheme, SwitchPatch, SwitchRule, TextPatch, Theme,
        ThemeContext, ThemePatch, WidgetPatchMap, WidgetState,
    };

    use super::*;

    /// kde-breeze light, which states no switch label nor label gap, with
    /// `patch` laid over it.
    fn breeze(patch: &ThemePatch) -> ResolvedTheme {
        let mut r = Theme::preset(Preset::KdeBreeze)
            .resolve(&ThemeContext::default().with_scheme(Scheme::Light));
        r.apply_patch(patch);
        r
    }

    /// Where the theme states no label, its colour is gpui-component's own:
    /// the window's text, disabled `muted_foreground`, at the homes
    /// `next::apply` fills them from; no gap and no font property is taken.
    #[test]
    fn an_unstated_switch_label_is_gpui_components_own() -> Mapped<()> {
        let r = breeze(&ThemePatch::new());
        let text = hsla(stated(r.colors.window.foreground, "foreground")?);
        let muted = hsla(stated(r.colors.window.foreground_muted, "muted")?);
        for checked in [false, true] {
            let rest = SwitchLook::next_of(&r, checked, false)?;
            assert_eq!(rest.label, text);
            assert_eq!(rest.label_gap, None);
            assert_eq!(
                SwitchLook::next_label_home(&r, checked, false)?,
                SwitchLabelHome::WindowForeground
            );
            let off = SwitchLook::next_of(&r, checked, true)?;
            assert_eq!(off.label, muted);
            assert_eq!(
                SwitchLook::next_label_home(&r, checked, true)?,
                SwitchLabelHome::WindowForegroundMuted
            );
        }
        let font = SwitchLabelFont::next_of(&r)?;
        assert_eq!(
            (font.family, font.size, font.weight, font.style),
            (None, None, None, None)
        );
        assert_eq!(font.line_height, r.typography.line_height_multiplier);
        Ok(())
    }

    /// A theme patched to state the label's colour, size and weight, a
    /// disabled colour for `DISABLED`, and the gap: the look and the font
    /// take each at the switch's own home.
    #[test]
    fn a_stated_switch_label_is_read_at_its_homes() -> Mapped<()> {
        let red = Color::rgb(1.0, 0.0, 0.0);
        let blue = Color::rgb(0.0, 0.0, 1.0);
        let r = breeze(
            &ThemePatch::new()
                .widget_rules(WidgetPatchMap::new().switch(SwitchPatch::new().label_gap(13.0)))
                .switch(
                    SwitchRule::new().label(TextPatch::new().color(red).size_px(17.0).weight(700)),
                )
                .switch(
                    SwitchRule::new()
                        .states(WidgetState::DISABLED)
                        .label(TextPatch::new().color(blue)),
                ),
        );
        for (checked, disabled, colour, home) in [
            (false, false, red, SwitchLabelHome::Rest),
            (true, false, red, SwitchLabelHome::Checked),
            (false, true, blue, SwitchLabelHome::Disabled),
            (true, true, blue, SwitchLabelHome::DisabledChecked),
        ] {
            let look = SwitchLook::next_of(&r, checked, disabled)?;
            assert_eq!(look.label, hsla(colour), "{home:?}");
            assert_eq!(look.label_gap, Some(px(13.0)), "{home:?}");
            assert_eq!(SwitchLook::next_label_home(&r, checked, disabled)?, home);
        }
        let font = SwitchLabelFont::next_of(&r)?;
        assert_eq!(font.size, Some(px(17.0)));
        assert_eq!(font.weight, Some(gpui::FontWeight(700.0)));
        Ok(())
    }

    /// A theme stating the label's colour for `CHECKED` alone: the checked
    /// switch takes it, and the disabled checked one, which no rule for
    /// `DISABLED` colours, gpui-component's own `muted_foreground`, not the
    /// checked colour the `DISABLED|CHECKED` query also returns.
    #[test]
    fn a_checked_label_colour_is_not_the_disabled_checked_one() -> Mapped<()> {
        let red = Color::rgb(1.0, 0.0, 0.0);
        let r = breeze(
            &ThemePatch::new().switch(
                SwitchRule::new()
                    .states(WidgetState::CHECKED)
                    .label(TextPatch::new().color(red)),
            ),
        );
        let muted = hsla(stated(r.colors.window.foreground_muted, "muted")?);
        assert_eq!(SwitchLook::next_of(&r, true, false)?.label, hsla(red));
        assert_eq!(
            text_color_of(&r.switch.label(WidgetState::DISABLED | WidgetState::CHECKED)),
            Some(hsla(red))
        );
        assert_eq!(SwitchLook::next_of(&r, true, true)?.label, muted);
        assert_eq!(
            SwitchLook::next_label_home(&r, true, true)?,
            SwitchLabelHome::WindowForegroundMuted
        );
        Ok(())
    }
}
