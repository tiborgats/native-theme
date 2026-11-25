//! [`Switch`]: a two-state switch, a thumb on a track.

use native_theme_egui::convert::{to_color32, to_corner_radius};
use native_theme_egui::egui::{self, AtomExt as _, accesskit};
use native_theme_egui::native_theme::next::{
    LabelSide, PartStyle, ResolvedTheme as NextResolvedTheme, TextStated, WidgetState,
};
use native_theme_egui::native_theme::theme::ResolvedSwitchTheme;
use native_theme_egui::{Note, ResolvedTheme, Role, ThemeAtlas, register_focus_shape};

use crate::parts::Parts;
use crate::{next, scope};

/// A two-state switch: `switch.*`'s track and thumb, the label after it (or before it, on a
/// 0.8.0 theme that says so).
///
/// **Tier P.** `egui/src/widgets/` has no switch or toggle module (`button`, `checkbox`,
/// `color_picker`, `drag_value`, `hyperlink`, `image`, `label`, `progress_bar`,
/// `radio_button`, `separator`, `slider`, `spinner`, `text_edit`), and the connector's
/// substitute, `Button::new(..).selected(checked)` (`egui/src/widgets/button.rs:270`),
/// shows the track colours as a button's fill and no thumb.
///
/// Built as egui's own `Checkbox` is (`egui-0.37.0/src/widgets/checkbox.rs`, `Checkbox::ui`):
/// a `WidgetAtom` whose first atom is the track, the label after it at egui's own
/// `Spacing::icon_spacing`, sensing `Sense::click()`; a click flips the value and marks the
/// response changed.
///
/// * The track is `switch.track_width` × `switch.track_height` at `switch.track_radius`, in
///   `checked_background` or `unchecked_background`, with `hover_checked_background` or
///   `hover_unchecked_background` composited over it while hovered. Under the 0.8.0 model a
///   hovered track's stated fill is the whole fill it shows, taken as it is (spec §9, step 2).
/// * The thumb is a `switch.thumb_diameter` circle in `thumb_background`, centred on the
///   track's axis and inset `0.5 · (track_height − thumb_diameter)` from its ends; a thumb
///   taller than its track overhangs it above and below and sits flush with its ends, as
///   qqc2-desktop-style's handle does (docs/platform-facts.md §2.21). While off it is
///   `unchecked_thumb_diameter` across in `unchecked_thumb_background` where the theme
///   states them (Material's 16 in `outline`, against 24 in `on-primary`). It moves, and
///   grows or shrinks, over the scope's `Style::animation_time`, which the connector sets to
///   `0.0` under reduced motion.
/// * The track is outlined inside its edge, `track_border_line_width` wide, in
///   `checked_track_border_color` or `unchecked_track_border_color`, and the thumb inside its
///   edge, `thumb_border_line_width` wide, in `thumb_border_color`, where the theme states the
///   colour and the width (docs/platform-facts.md §2.21).
/// * The label is text of no widget font of its own, so in `defaults.font.color`: the switch
///   role's cell carries no text colour of the switch's.
/// * `.enabled(false)` paints `disabled_checked_background`, `disabled_unchecked_background`,
///   `disabled_thumb_color` and the tracks' `disabled_*_track_border_color` (a `None` copies
///   the colour it stands for) and the label in
///   `defaults.disabled_text_color`, faded by `switch.disabled_opacity` as one widget over
///   the backdrop (the crate's *Disabled*): the thumb on the track, the pair over
///   [`Switch::backdrop`]. Under `ui.add_enabled(false, ..)` it is faded as one by the
///   calling `Ui`'s `disabled_alpha`, the fade egui gave the painter taken back.
///
/// The focus ring surrounds the track (`register_focus_shape`). AccessKit sees a
/// `Role::Switch`, toggled as the value is.
///
/// Under an atlas of the 0.8.0 model each of these is read at its home: `switch.track` and
/// `switch.thumb` at the switch's state, the label in `switch.label`'s colour where stated, else
/// the body text's, hovered and pressed in `switch.label(HOVER)`'s and `switch.label(PRESSED)`'s
/// where a rule for the state states it, else in egui's own hovered and pressed label colours,
/// `widgets.hovered.fg_stroke` and `widgets.active.fg_stroke`, as egui's `Checkbox` letters its
/// label where `override_text_color` is unset (spec §10 rule 0), at `switch.label(NONE)`'s size where stated, else
/// egui's own, in egui's `Proportional` family as the connector's role fonts are (a family,
/// weight or slant the label states is not drawn: the crate's README, *Limits under the 0.8.0
/// model*), `switch.label_gap` from the track where stated, else egui's own `icon_spacing`,
/// before the track where `switch.label_side` states `Leading`, else after it, and at the far
/// end of a switch laid out wider than its content where `switch.label_gap_stretches` states
/// `true`, the gap the least space between them, else together at the gap; a track or thumb
/// stroke stated as none is no outline. The on and off glyphs `switch.show_state_glyphs` asks
/// for are not drawn ([`Switch::next_glyphs_unmapped`]).
///
/// With no atlas installed, a switch size that is not finite, or a 0.8.0 theme that states no
/// track size, checked thumb size, track or checked thumb fill, or corners egui can draw, or
/// states a label size that is not a positive finite number, it adds `egui::Checkbox`, egui's
/// own two-state control.
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct Switch<'a> {
    on: &'a mut bool,
    label: Option<egui::WidgetText>,
    enabled: bool,
    backdrop: Option<egui::Color32>,
}

impl<'a> Switch<'a> {
    /// A switch showing, and flipping, `on`.
    pub fn new(on: &'a mut bool) -> Self {
        Self {
            on,
            label: None,
            enabled: true,
            backdrop: None,
        }
    }

    /// The text beside the track, after it or where a 0.8.0 theme says so before it, which also
    /// names the switch for assistive technology.
    pub fn label(mut self, text: impl Into<egui::WidgetText>) -> Self {
        self.label = Some(text.into());
        self
    }

    /// `false` shows the platform's disabled switch, its disabled colours faded by its
    /// `disabled_opacity` as one widget over its backdrop ([`Self::backdrop`]), and takes no
    /// input.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// The colour under the switch, which a disabled one is faded over as one widget: the thumb
    /// on its track, the pair over the backdrop ([`crate::fade::fade_as_one`]); the window's,
    /// `defaults.background_color`, where not given.
    pub fn backdrop(mut self, backdrop: egui::Color32) -> Self {
        self.backdrop = Some(backdrop);
        self
    }

    /// Where the 0.8.0 theme `r` states that the track shows on and off glyphs
    /// (`switch.show_state_glyphs`, spec §5), a [`Note::Unmapped`] at that home,
    /// `"switch.show_state_glyphs"`, which the switch does not draw (spec §10, rule 2): the
    /// model states no glyph, nor its size, colour or place on the track, and egui's own
    /// `Checkbox` draws none. `None` where the theme states none or states that the track
    /// shows none.
    #[must_use]
    pub fn next_glyphs_unmapped(r: &NextResolvedTheme) -> Option<Note> {
        (r.switch.show_state_glyphs == Some(true))
            .then(|| Note::unmapped("switch.show_state_glyphs"))
    }
}

/// The track and thumb sizes, every one finite.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Geometry {
    pub(crate) track: egui::Vec2,
    pub(crate) radius: f32,
    pub(crate) thumb: f32,
    /// The thumb while off: `unchecked_thumb_diameter`, or `thumb`.
    pub(crate) unchecked_thumb: f32,
    /// `track_border_line_width`; `None` draws no track outline.
    pub(crate) track_border: Option<f32>,
    /// `thumb_border_line_width`; `None` draws no thumb outline.
    pub(crate) thumb_border: Option<f32>,
    /// The checked thumb's outline width: `thumb_border_line_width`, the 0.8.0 theme's
    /// `switch.thumb(CHECKED)`'s; `None` draws no outline on a checked thumb.
    pub(crate) checked_thumb_border: Option<f32>,
    /// The space between the track and the label; `None`, egui's own `Spacing::icon_spacing`.
    pub(crate) label_gap: Option<f32>,
    /// The label's size, the 0.8.0 theme's `switch.label(NONE).size_px`; `None`, egui's own.
    pub(crate) label_size: Option<f32>,
    /// The label before the track: the 0.8.0 theme's `switch.label_side` stated `Leading`;
    /// `false`, after it, the side egui's own `Checkbox` lays its text on.
    pub(crate) leading: bool,
    /// The width a switch laid out wider than its content has left over put between the label
    /// and the track, the two at the ends of the row with the gap the least space between them:
    /// the 0.8.0 theme's `switch.label_gap_stretches` stated `true`; `false`, the two kept
    /// together at the gap, as egui's own `Checkbox` keeps them.
    pub(crate) stretches: bool,
}

impl Geometry {
    /// The geometry of a switch of a 0.8.0 theme: the track's minimum size and corners at
    /// rest, the thumb's minimum width checked and at rest (the checked one where the theme
    /// states none at rest), the outlines' widths at rest and the thumb's checked (a stroke
    /// stated as none is no outline), `switch.label_gap` and the label's size at rest; `None`
    /// where the theme states no track or checked thumb size, corners egui cannot draw as one
    /// radius, or a label size that is not a positive finite number, which sends the switch to
    /// egui's own counterpart.
    pub(crate) fn next_of(r: &NextResolvedTheme) -> Option<Self> {
        let sw = &r.switch;
        let label = sw.label(WidgetState::NONE);
        let label_size = Self::label_size(
            label
                .stated
                .contains(TextStated::SIZE_PX)
                .then_some(label.size_px),
        )?;
        let track = sw.track(WidgetState::NONE);
        let rest_thumb = sw.thumb(WidgetState::NONE);
        let checked_thumb = sw.thumb(WidgetState::CHECKED);
        let [width, height] = track.min_size;
        let (width, height) = (scope::length(width?)?, scope::length(height?)?);
        let [thumb, _] = checked_thumb.min_size;
        let thumb = scope::length(thumb?)?;
        let [unchecked_thumb, _] = rest_thumb.min_size;
        let unchecked_thumb = match unchecked_thumb {
            Some(d) => scope::length(d)?,
            None => thumb,
        };
        let border = |width: Option<f32>| match width {
            Some(w) => scope::length(w).map(Some),
            None => Some(None),
        };
        Some(Self {
            track: egui::vec2(width, height),
            radius: scope::length(next::radius(&track, Some(width), Some(height))?)?,
            thumb,
            unchecked_thumb,
            track_border: border(next::stroke_width_of(&track))?,
            thumb_border: border(next::stroke_width_of(&rest_thumb))?,
            checked_thumb_border: border(next::stroke_width_of(&checked_thumb))?,
            label_gap: border(sw.label_gap)?,
            label_size,
            leading: sw.label_side == Some(LabelSide::Leading),
            stretches: sw.label_gap_stretches == Some(true),
        })
    }

    /// The label's size of the size the theme states of `switch.label(NONE)`, `stated`:
    /// `Some(None)` where it states none, egui's own; `None` where it states one that is not a
    /// positive finite number, which sends the switch to egui's own counterpart.
    pub(crate) fn label_size(stated: Option<f32>) -> Option<Option<f32>> {
        match stated {
            Some(size) => Some(Some(scope::length(size).filter(|size| *size > 0.0)?)),
            None => Some(None),
        }
    }

    /// The geometry as the switch paints it, so two that paint alike compare equal: an outline
    /// of no width, which paints as none (a stroke stated as none is one, spec §10, rule 2),
    /// given as none.
    #[cfg(test)]
    pub(crate) fn painted(self) -> Self {
        let drawn = |width: Option<f32>| width.filter(|w| *w > 0.0);
        Self {
            track_border: drawn(self.track_border),
            thumb_border: drawn(self.thumb_border),
            checked_thumb_border: drawn(self.checked_thumb_border),
            ..self
        }
    }

    pub(crate) fn of(sw: &ResolvedSwitchTheme) -> Option<Self> {
        let thumb = scope::length(sw.thumb_diameter)?;
        let unchecked_thumb = match sw.unchecked_thumb_diameter {
            Some(d) => scope::length(d)?,
            None => thumb,
        };
        let border = |width: Option<f32>| match width {
            Some(w) => scope::length(w).map(Some),
            None => Some(None),
        };
        Some(Self {
            track: egui::vec2(
                scope::length(sw.track_width)?,
                scope::length(sw.track_height)?,
            ),
            radius: scope::length(sw.track_radius)?,
            thumb,
            unchecked_thumb,
            track_border: border(sw.track_border_line_width)?,
            thumb_border: border(sw.thumb_border_line_width)?,
            checked_thumb_border: border(sw.thumb_border_line_width)?,
            label_gap: None,
            label_size: None,
            leading: false,
            stretches: false,
        })
    }
}

/// The colours a switch reads of a theme, every colour read in one place, so the crate's
/// colour-leaf coverage test sees the widget's reads.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Look {
    pub(crate) checked_background: egui::Color32,
    /// The hovered on track's whole fill: the 0.8.0 theme's stated fill as it is, or 0.7.2's
    /// `hover_checked_background` composited over `checked_background`; `None`, the rest fill.
    pub(crate) hover_checked_background: Option<egui::Color32>,
    pub(crate) unchecked_background: egui::Color32,
    /// The hovered off track's whole fill, as [`Self::hover_checked_background`] over
    /// `unchecked_background`.
    pub(crate) hover_unchecked_background: Option<egui::Color32>,
    pub(crate) disabled_checked_background: Option<egui::Color32>,
    pub(crate) disabled_unchecked_background: Option<egui::Color32>,
    pub(crate) thumb_background: egui::Color32,
    pub(crate) unchecked_thumb_background: Option<egui::Color32>,
    pub(crate) disabled_thumb_color: Option<egui::Color32>,
    /// A disabled checked switch's thumb: `disabled_thumb_color`, the 0.8.0 theme's
    /// `switch.thumb(DISABLED|CHECKED)`'s fill.
    pub(crate) disabled_checked_thumb_color: Option<egui::Color32>,
    pub(crate) unchecked_track_border_color: Option<egui::Color32>,
    pub(crate) checked_track_border_color: Option<egui::Color32>,
    pub(crate) disabled_unchecked_track_border_color: Option<egui::Color32>,
    pub(crate) disabled_checked_track_border_color: Option<egui::Color32>,
    pub(crate) thumb_border_color: Option<egui::Color32>,
    /// A checked switch's thumb outline: `thumb_border_color`, the 0.8.0 theme's
    /// `switch.thumb(CHECKED)`'s stroke colour.
    pub(crate) checked_thumb_border_color: Option<egui::Color32>,
    /// `defaults.font.color`: the label; `None`, egui's own.
    pub(crate) text: Option<egui::Color32>,
    /// A hovered switch's label: the 0.8.0 theme's `switch.label(HOVER)` colour where a rule
    /// for `HOVER` states it, 0.7.2's [`Self::text`]; `None`, egui's own hovered label colour,
    /// the scope's `widgets.hovered.fg_stroke.color` (egui 0.37.0
    /// `src/theme/default_style.rs:172-186`, `src/widget_style/mod.rs:44-50`).
    pub(crate) hover_text: Option<egui::Color32>,
    /// A pressed switch's label: the 0.8.0 theme's `switch.label(PRESSED)` colour where a rule
    /// for `PRESSED` states it, 0.7.2's [`Self::text`]; `None`, egui's own pressed label colour,
    /// the scope's `widgets.active.fg_stroke.color` (egui 0.37.0
    /// `src/theme/default_style.rs:172-186`, `src/widget_style/mod.rs:44-50`).
    pub(crate) pressed_text: Option<egui::Color32>,
    /// `defaults.disabled_text_color`: a disabled switch's label; `None`, egui's own.
    pub(crate) disabled_text: Option<egui::Color32>,
    /// `defaults.background_color`: the colour faded over where no backdrop is given; `None`,
    /// egui's own fade of a disabled widget.
    pub(crate) background: Option<egui::Color32>,
}

impl Look {
    /// The colours of a switch of a 0.8.0 theme, at the homes of the 0.7.2 fields `of` reads:
    /// `switch.track` and `switch.thumb` at `NONE`, `CHECKED`, `DISABLED` and
    /// `DISABLED|CHECKED`, a disabled one asked at its state directly (egui has no disabled
    /// switch whose treatment an unstated value would keep, so spec §2's one-flag query does
    /// not apply); a hovered track's fill where a rule for `HOVER` states it (read at
    /// `HOVER|CHECKED` on a checked switch, as the query with that flag alone says, spec §2,
    /// `from_state`); the label's colour at `switch.label`, else the body text's
    /// (`typography.body.color`), a hovered and a pressed one's at `switch.label(HOVER)` and
    /// `switch.label(PRESSED)` where a rule for that state states it, else egui's own hovered
    /// and pressed label colours ([`Self::hover_text`], [`Self::pressed_text`]), a
    /// disabled one's at `switch.label(DISABLED)` where a rule states it, else
    /// `colors.disabled_text`; the window's background as the ground. A fill stated as layers
    /// that are all gradients is read as unstated ([`next::fill`]). `None` where the theme
    /// states no fill, or one as such layers, at `switch.track(NONE)`, `switch.track(CHECKED)`
    /// or `switch.thumb(CHECKED)`, which sends the switch to egui's own counterpart.
    pub(crate) fn next_of(r: &NextResolvedTheme) -> Option<Self> {
        Self::next_noted(r, &mut Vec::new())
    }

    /// [`Self::next_of`], with a [`Note::Unmapped`] in `unmapped` at each fill's home the
    /// theme states as layers that are all gradients, read as unstated ([`next::fill`]).
    pub(crate) fn next_noted(r: &NextResolvedTheme, unmapped: &mut Vec<Note>) -> Option<Self> {
        let sw = &r.switch;
        let track = |s: WidgetState| sw.track(s);
        let thumb = |s: WidgetState| sw.thumb(s);
        let (on, off, off_on) = (
            WidgetState::CHECKED,
            WidgetState::DISABLED,
            WidgetState::DISABLED | WidgetState::CHECKED,
        );
        let hover = track(WidgetState::HOVER);
        let (text, disabled_text) = Self::label_colors(
            next::text_color_of(&sw.label(WidgetState::NONE)),
            next::state_text_color_of(&sw.label(off)),
            next::body_text(r),
            next::disabled_text_color(r),
        );
        let mut fill = |part: &PartStyle, home: &'static str| next::fill(part, home, unmapped);
        let checked_background = fill(&track(on), "switch.track(CHECKED).fill");
        let unchecked_background = fill(&track(WidgetState::NONE), "switch.track(NONE).fill");
        let disabled_checked_background =
            fill(&track(off_on), "switch.track(DISABLED|CHECKED).fill");
        let disabled_unchecked_background = fill(&track(off), "switch.track(DISABLED).fill");
        let thumb_background = fill(&thumb(on), "switch.thumb(CHECKED).fill");
        let unchecked_thumb_background = fill(&thumb(WidgetState::NONE), "switch.thumb(NONE).fill");
        let disabled_thumb_color = fill(&thumb(off), "switch.thumb(DISABLED).fill");
        let disabled_checked_thumb_color =
            fill(&thumb(off_on), "switch.thumb(DISABLED|CHECKED).fill");
        let hover_checked_background = next::ruled_fill(
            &hover,
            &track(WidgetState::HOVER | WidgetState::CHECKED),
            "switch.track(HOVER|CHECKED).fill",
            unmapped,
        );
        let hover_unchecked_background =
            next::ruled_fill(&hover, &hover, "switch.track(HOVER).fill", unmapped);
        Some(Self {
            checked_background: checked_background?,
            hover_checked_background,
            unchecked_background: unchecked_background?,
            hover_unchecked_background,
            disabled_checked_background,
            disabled_unchecked_background,
            thumb_background: thumb_background?,
            unchecked_thumb_background,
            disabled_thumb_color,
            disabled_checked_thumb_color,
            unchecked_track_border_color: next::stroke_color_of(&track(WidgetState::NONE)),
            checked_track_border_color: next::stroke_color_of(&track(on)),
            disabled_unchecked_track_border_color: next::stroke_color_of(&track(off)),
            disabled_checked_track_border_color: next::stroke_color_of(&track(off_on)),
            thumb_border_color: next::stroke_color_of(&thumb(WidgetState::NONE)),
            checked_thumb_border_color: next::stroke_color_of(&thumb(on)),
            text,
            hover_text: next::state_text_color_of(&sw.label(WidgetState::HOVER)),
            pressed_text: next::state_text_color_of(&sw.label(WidgetState::PRESSED)),
            disabled_text: Some(disabled_text),
            background: next::window_background(r),
        })
    }

    /// The label's colours at rest and disabled of what the theme states of them: `stated`,
    /// `switch.label(NONE)`'s colour, else `body`, the body text's (`typography.body.color`);
    /// `stated_disabled`, the colour a rule for `DISABLED` states of `switch.label`, else
    /// `disabled`, the theme's disabled text (`colors.disabled_text`).
    pub(crate) fn label_colors(
        stated: Option<egui::Color32>,
        stated_disabled: Option<egui::Color32>,
        body: Option<egui::Color32>,
        disabled: egui::Color32,
    ) -> (Option<egui::Color32>, egui::Color32) {
        (stated.or(body), stated_disabled.unwrap_or(disabled))
    }

    /// The colours as the switch paints them, so two looks that paint alike compare equal: the
    /// thumb at rest, which `None` gives as the checked one, given as that one; a disabled
    /// colour, which `None` gives as the one it stands for, given as that one; and a hover
    /// fill that is the track's rest fill, none.
    #[cfg(test)]
    pub(crate) fn painted(self) -> Self {
        let layer = |hover: Option<egui::Color32>, base: egui::Color32| {
            hover.filter(|&hover| hover != base)
        };
        let unchecked_thumb = self
            .unchecked_thumb_background
            .unwrap_or(self.thumb_background);
        Self {
            hover_checked_background: layer(self.hover_checked_background, self.checked_background),
            hover_unchecked_background: layer(
                self.hover_unchecked_background,
                self.unchecked_background,
            ),
            disabled_checked_background: Some(
                self.disabled_checked_background
                    .unwrap_or(self.checked_background),
            ),
            disabled_unchecked_background: Some(
                self.disabled_unchecked_background
                    .unwrap_or(self.unchecked_background),
            ),
            unchecked_thumb_background: Some(unchecked_thumb),
            disabled_thumb_color: Some(self.disabled_thumb_color.unwrap_or(unchecked_thumb)),
            disabled_checked_thumb_color: Some(
                self.disabled_checked_thumb_color
                    .unwrap_or(self.thumb_background),
            ),
            disabled_unchecked_track_border_color: self
                .disabled_unchecked_track_border_color
                .or(self.unchecked_track_border_color),
            disabled_checked_track_border_color: self
                .disabled_checked_track_border_color
                .or(self.checked_track_border_color),
            ..self
        }
    }

    pub(crate) fn of(t: &ResolvedTheme) -> Self {
        let sw = &t.switch;
        let c = to_color32;
        let over = |base, layer: Option<_>| layer.map(|layer| c(base).blend(c(layer)));
        Self {
            checked_background: c(sw.checked_background),
            hover_checked_background: over(sw.checked_background, sw.hover_checked_background),
            unchecked_background: c(sw.unchecked_background),
            hover_unchecked_background: over(
                sw.unchecked_background,
                sw.hover_unchecked_background,
            ),
            disabled_checked_background: sw.disabled_checked_background.map(c),
            disabled_unchecked_background: sw.disabled_unchecked_background.map(c),
            thumb_background: c(sw.thumb_background),
            unchecked_thumb_background: sw.unchecked_thumb_background.map(c),
            disabled_thumb_color: sw.disabled_thumb_color.map(c),
            disabled_checked_thumb_color: sw.disabled_thumb_color.map(c),
            unchecked_track_border_color: sw.unchecked_track_border_color.map(c),
            checked_track_border_color: sw.checked_track_border_color.map(c),
            disabled_unchecked_track_border_color: sw.disabled_unchecked_track_border_color.map(c),
            disabled_checked_track_border_color: sw.disabled_checked_track_border_color.map(c),
            thumb_border_color: sw.thumb_border_color.map(c),
            checked_thumb_border_color: sw.thumb_border_color.map(c),
            text: Some(c(t.defaults.font.color)),
            hover_text: Some(c(t.defaults.font.color)),
            pressed_text: Some(c(t.defaults.font.color)),
            disabled_text: Some(c(t.defaults.disabled_text_color)),
            background: Some(c(t.defaults.background_color)),
        }
    }
}

/// What a switch paints with, read from the atlas before its scope opens.
struct Paint {
    geometry: Geometry,
    sw: Look,
    /// The colour a disabled switch is faded over as one widget, by its own `.enabled(false)`
    /// or under a disabled calling `Ui`; `None` where it is enabled, or nothing is to be faded
    /// as one (`crate::fade::outer_fade`).
    ground: Option<egui::Color32>,
    /// The label's font: [`Geometry::label_size`] in egui's `Proportional` family, which holds
    /// the face the atlas's font plan installed for the body text, as the connector's role fonts
    /// are (`next::text_role_font`), lowered to the atlas's `text_size_cap`; `None`, egui's own.
    font: Option<egui::FontId>,
}

impl egui::Widget for Switch<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Switch {
            on,
            label,
            enabled,
            backdrop,
        } = self;
        // Faded as one where disabled here, or under a disabled calling `Ui` whose fade it
        // takes back.
        let as_one = (!enabled && ui.is_enabled()) || crate::fade::outer_fade(ui).is_some();
        let paint = ThemeAtlas::from_ctx(ui.ctx()).and_then(|atlas| {
            let theme = ui.ctx().theme();
            let (sw, geometry) = match atlas.resolved_for(theme) {
                Some(t) => (Look::of(t), Geometry::of(&t.switch)?),
                None => {
                    let r = atlas.next_resolved_for(theme)?;
                    (Look::next_of(r)?, Geometry::next_of(r)?)
                }
            };
            let cap = atlas.text_size_cap();
            Some(Paint {
                geometry,
                sw,
                ground: as_one.then(|| backdrop.or(sw.background)).flatten(),
                font: geometry
                    .label_size
                    .map(|size| egui::FontId::proportional(cap.map_or(size, |cap| size.min(cap)))),
            })
        });
        match paint {
            Some(paint) => {
                scope::open_faded(ui, Role::Switch, false, enabled, paint.ground, |ui| {
                    switch_ui(ui, on, label, &paint)
                })
            }
            None => {
                let checkbox = match label {
                    Some(label) => egui::Checkbox::new(on, label),
                    None => egui::Checkbox::without_text(on),
                };
                ui.add_enabled(enabled, checkbox)
            }
        }
    }
}

fn switch_ui(
    ui: &mut egui::Ui,
    on: &mut bool,
    label: Option<egui::WidgetText>,
    paint: &Paint,
) -> egui::Response {
    let Paint {
        geometry: g,
        sw,
        ground,
        font,
    } = paint;
    if let Some(font) = font {
        ui.style_mut().override_font_id = Some(font.clone());
    }
    // Faded as one over `ground` (the painter's opacity kept), or not at all here: where the
    // platform dims by colour alone (`disabled_alpha` 1) the colours are as stated.
    let alpha = ui.visuals().disabled_alpha();
    let ground = &ground.filter(|_| alpha < 1.0);
    let as_one = |under: egui::Color32, colour: egui::Color32| match ground {
        Some(ground) => crate::fade::faded(*ground, under.blend(colour), alpha),
        None => colour,
    };
    // The atom holds the track and a thumb that overhangs it above and below.
    let largest = g.thumb.max(g.unchecked_thumb);
    let atom = egui::vec2(g.track.x, g.track.y.max(largest));
    let mut min_size = egui::Vec2::splat(ui.spacing().interact_size.y);
    min_size.y = min_size.y.max(atom.y);
    let text = label.as_ref().map(|l| l.text().to_string());
    // The label as the layout sizes it (`egui/src/atomics/atom_kind.rs:134-135`), for `Parts`.
    let label_size = label.as_ref().map(|l| {
        l.clone()
            .into_galley(
                ui,
                Some(egui::TextWrapMode::Extend),
                f32::INFINITY,
                egui::FontSelection::Default,
            )
            .size()
    });
    let gap = g.label_gap.unwrap_or(ui.spacing().icon_spacing);
    // A stretching gap is an atom of the gap's width between the label and the track that grows
    // into the width left over (`egui-0.37.0/src/atomics/container_atom.rs`,
    // `SizedContainerAtom::paint_at`), the atoms laid without a gap of their own: egui lays its
    // gap between every two atoms, so `Atom::grow` beside a gap would keep twice the gap.
    let stretched = g.stretches && label.is_some();
    let mut atoms = match label {
        Some(label) => egui::Atoms::new(label),
        None => egui::Atoms::new(()),
    };
    let rect_id = egui::IdSalt::new("native-theme-egui-widgets::switch");
    let gap_id = egui::IdSalt::new("native-theme-egui-widgets::switch::gap");
    // The gap atom lies against the label in its grown cell, so its rect ends where the label
    // begins (or begins where it ends).
    let gap_atom = |align| {
        egui::Atom::custom(gap_id, egui::vec2(gap, 0.0))
            .atom_grow(true)
            .atom_align(align)
    };
    let track_atom = egui::Atom::custom(rect_id, atom);
    if g.leading {
        if stretched {
            atoms.push_right(gap_atom(egui::Align2::LEFT_CENTER));
        }
        atoms.push_right(track_atom);
    } else {
        if stretched {
            atoms.push_left(gap_atom(egui::Align2::RIGHT_CENTER));
        }
        atoms.push_left(track_atom);
    }

    let mut prepared = egui::WidgetAtom::new(atoms)
        .gap(if stretched { 0.0 } else { gap })
        .sense(egui::Sense::click())
        .min_size(min_size)
        .allocate(ui);
    if prepared.response.clicked() {
        *on = !*on;
        prepared.response.mark_changed();
    }
    let enabled = ui.is_enabled();
    let checked = *on;
    prepared.response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::Role::CheckBox,
            enabled,
            checked,
            text.as_deref().unwrap_or_default(),
        )
    });
    ui.ctx()
        .accesskit_node_builder(prepared.response.id, |node| {
            node.set_role(accesskit::Role::Switch);
            node.set_toggled(if checked {
                accesskit::Toggled::True
            } else {
                accesskit::Toggled::False
            });
        });

    let hovered = enabled && prepared.response.hovered();
    let pressed = enabled && prepared.response.is_pointer_button_down_on();
    let time = ui.style().animation_time;
    let position = ui
        .ctx()
        .animate_bool_with_time(prepared.response.id, checked, time);
    let text = if !enabled {
        sw.disabled_text
    } else if pressed {
        Some(
            sw.pressed_text
                .unwrap_or(ui.visuals().widgets.active.text_color()),
        )
    } else if hovered {
        Some(
            sw.hover_text
                .unwrap_or(ui.visuals().widgets.hovered.text_color()),
        )
    } else {
        sw.text
    };
    let text = text.unwrap_or(prepared.container.fallback_text_color);
    prepared.container.fallback_text_color = ground.map_or(text, |ground| as_one(ground, text));
    let laid = prepared.paint(ui);
    let Some(atom_rect) = laid.rect(rect_id) else {
        return laid.response;
    };
    // From the atom's left edge, as `Checkbox` places its box (`egui/src/widgets/checkbox.rs:129-132`).
    let track = egui::Rect::from_center_size(
        egui::pos2(atom_rect.left() + 0.5 * atom.x, atom_rect.center().y),
        g.track,
    );
    let radius = to_corner_radius(egui::CornerRadius::ZERO, g.radius);
    register_focus_shape(ui.ctx(), laid.response.id, track, radius);

    let (fill, hover) = match (checked, enabled) {
        (true, true) => (sw.checked_background, sw.hover_checked_background),
        (false, true) => (sw.unchecked_background, sw.hover_unchecked_background),
        (true, false) => (
            sw.disabled_checked_background
                .unwrap_or(sw.checked_background),
            None,
        ),
        (false, false) => (
            sw.disabled_unchecked_background
                .unwrap_or(sw.unchecked_background),
            None,
        ),
    };
    let fill = match hover {
        Some(hover) if hovered => hover,
        _ => fill,
    };
    let rest_edge = if checked {
        sw.checked_track_border_color
    } else {
        sw.unchecked_track_border_color
    };
    let edge = match (checked, enabled) {
        (true, false) => sw.disabled_checked_track_border_color.or(rest_edge),
        (false, false) => sw.disabled_unchecked_track_border_color.or(rest_edge),
        _ => rest_edge,
    };
    let edge = edge.zip(g.track_border);
    let thumb_edge = if checked {
        sw.checked_thumb_border_color.zip(g.checked_thumb_border)
    } else {
        sw.thumb_border_color.zip(g.thumb_border)
    };
    let (rest_thumb, disabled_thumb) = if checked {
        (sw.thumb_background, sw.disabled_checked_thumb_color)
    } else {
        (
            sw.unchecked_thumb_background.unwrap_or(sw.thumb_background),
            sw.disabled_thumb_color,
        )
    };
    let thumb = if enabled {
        rest_thumb
    } else {
        disabled_thumb.unwrap_or(rest_thumb)
    };
    // The thumb grows from its off diameter to its on one as it travels.
    let diameter = egui::lerp(g.unchecked_thumb..=g.thumb, position);
    // The thumb's centre travels between the two ends of the track's axis, each half the
    // track's height in from its end: the thumb inset `0.5 · (track_height − thumb_diameter)`;
    // a thumb taller than the track half its own diameter in, flush with the end.
    let half = 0.5 * g.track.y.max(diameter);
    let ends = (track.left() + half)..=(track.right() - half);
    let centre = egui::pos2(egui::lerp(ends, position), track.center().y);
    // Faded as one: the track over the ground; its outline, drawn inside it, and the thumb and
    // its outline over the track.
    let on_ground = ground.map(|ground| ground.blend(fill));
    let faded = |under: Option<egui::Color32>, colour: egui::Color32| match under {
        Some(under) => as_one(under, colour),
        None => colour,
    };
    let edge = edge.map(|(colour, width)| egui::Stroke::new(width, faded(on_ground, colour)));
    let thumb_edge =
        thumb_edge.map(|(colour, width)| egui::Stroke::new(width, faded(on_ground, colour)));
    let (fill, thumb) = (faded(*ground, fill), faded(on_ground, thumb));
    let painter = ui.painter();
    painter.rect(
        track,
        radius,
        fill,
        edge.unwrap_or(egui::Stroke::NONE),
        egui::StrokeKind::Inside,
    );
    // epaint strokes a circle outside its radius (`epaint/src/tessellator.rs:1531`), so the
    // thumb's outline is drawn inside its diameter. An outline as wide as the thumb's radius
    // fills it, and epaint draws no circle whose radius is not above 0
    // (`tessellator.rs:1494-1496`), so that thumb is a disc in the outline's colour.
    let stroke = thumb_edge.unwrap_or(egui::Stroke::NONE);
    let inner = 0.5 * diameter - stroke.width;
    if inner > 0.0 {
        painter.circle(centre, inner, thumb, stroke);
    } else {
        painter.circle_filled(centre, 0.5 * diameter, stroke.color);
    }
    let mut parts = Parts::default();
    parts.push("track", track);
    parts.push(
        "thumb",
        egui::Rect::from_center_size(centre, egui::Vec2::splat(diameter)),
    );
    if let Some(size) = label_size {
        // Against the stretching gap's atom where it lies between them, else the gap from the
        // track.
        let label = match (laid.rect(gap_id).filter(|_| stretched), g.leading) {
            (Some(gap_rect), true) => crate::parts::text_before(gap_rect, 0.0, size),
            (Some(gap_rect), false) => crate::parts::text_after(gap_rect, 0.0, size),
            (None, true) => crate::parts::text_before(atom_rect, gap, size),
            (None, false) => crate::parts::text_after(atom_rect, gap, size),
        };
        parts.push("label", label);
    }
    parts.store(ui.ctx(), laid.response.id);
    laid.response
}
