//! `Switch` (spec §2.3): gpui-base's headless switch, track and thumb,
//! painted from `SwitchTheme`.

use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, FontWeight, Hsla, InteractiveElement, IntoElement, ParentElement,
    Pixels, RenderOnce, SharedString, Styled, Window, div, prelude::FluentBuilder as _, px, rems,
};
use gpui_base::{Switch as BaseSwitch, SwitchThumb, SwitchTrack, spring};
use gpui_component::{ActiveTheme as _, Disableable as _};
use native_theme::theme::ResolvedTheme;

use super::{color, length, native, over, text_size};

/// The space between the track and the label, which `SwitchTheme` does not
/// state: gpui-component's own, `gap_2` (switch.rs:197).
const LABEL_GAP: gpui::Rems = rems(0.5);

type ChangeHandler = Rc<dyn Fn(&bool, &mut Window, &mut App)>;

/// The group name the track's hover style listens to: the whole row is the
/// control the pointer hovers.
const HOVER_GROUP: &str = "native-theme-switch";

/// What a switch paints, per state, from `SwitchTheme` (spec §2.3).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SwitchLook {
    /// `switch.track_width`.
    pub track_width: Pixels,
    /// `switch.track_height`.
    pub track_height: Pixels,
    /// `switch.track_radius`.
    pub track_radius: Pixels,
    /// `switch.thumb_diameter`.
    pub thumb: Pixels,
    /// The thumb's inset from either end of the track, and from its top:
    /// `0.5 × (track_height − thumb_diameter)`, the centred thumb the two
    /// numbers describe (negative where the thumb overhangs its track).
    pub inset: Pixels,
    /// The track's colour in this state.
    pub track: Hsla,
    /// The track's colour under the pointer, where the control is enabled.
    pub hover_track: Option<Hsla>,
    /// The thumb's colour.
    pub thumb_color: Hsla,
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
    /// The hover and disabled colours are soft options: a `None` copies the
    /// colour it stands in for (C16), as the iced connector reads them
    /// (`native-theme-iced` `styles::toggler`).
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
        let hover_track = (!disabled).then(|| over(color(base), color(hover.unwrap_or(base))));
        let thumb_color = if disabled {
            s.disabled_thumb_color.unwrap_or(s.thumb_background)
        } else {
            s.thumb_background
        };
        let label = if disabled {
            resolved.defaults.disabled_text_color
        } else {
            resolved.defaults.text_color
        };
        let track_height = length(s.track_height)?;
        let thumb = length(s.thumb_diameter)?;
        Some(Self {
            track_width: length(s.track_width)?,
            track_height,
            track_radius: length(s.track_radius)?,
            thumb,
            inset: px((f32::from(track_height) - f32::from(thumb)) / 2.),
            track: color(track),
            hover_track,
            thumb_color: color(thumb_color),
            label: color(label),
            opacity: super::disabled_opacity(disabled, s.disabled_opacity),
        })
    }

    /// The thumb's left edge, from the track's left edge, at rest `checked`
    /// or not.
    #[must_use]
    pub fn thumb_left(&self, checked: bool) -> Pixels {
        if checked {
            px(f32::from(self.track_width) - f32::from(self.inset) - f32::from(self.thumb))
        } else {
            self.inset
        }
    }
}

/// A switch whose track, thumb and states are `SwitchTheme`'s (spec §2.3),
/// on gpui-base's headless `Switch`: click, Enter and Space toggle it, a
/// disabled one is inert, and it reports itself as a switch with its toggled
/// state. The thumb travels on gpui-component's spring (switch.rs:169-178).
///
/// Controlled: [`Switch::on_change`] receives the requested value. Without a
/// native theme it renders gpui-component's `Switch`.
pub struct Switch {
    id: ElementId,
    label: Option<SharedString>,
    checked: bool,
    disabled: bool,
    on_change: Option<ChangeHandler>,
}

impl Switch {
    /// A new switch, off.
    #[must_use]
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            checked: false,
            disabled: false,
            on_change: None,
        }
    }

    /// The label drawn beside the track, and the name a screen reader
    /// announces.
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
            .checked(self.checked)
            .disabled(self.disabled)
            .when_some(self.on_change, |switch, on_change| {
                switch.on_click(move |checked, window, cx| on_change(checked, window, cx))
            })
            .into_any_element()
    }
}

impl RenderOnce for Switch {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(n) = native(cx) else {
            return self.fallback();
        };
        let Some(look) = SwitchLook::of(n.resolved, self.checked, self.disabled) else {
            return self.fallback();
        };
        let checked = self.checked;
        let label_size = text_size(n.resolved.defaults.font.size, n);
        let label_weight = FontWeight(f32::from(n.resolved.defaults.font.weight));
        let thumb_left = spring(
            (self.id.clone(), "thumb"),
            look.thumb_left(checked),
            cx.theme().motion_tokens().spring_move,
            window,
            cx,
        );
        let track = SwitchTrack::new((self.id.clone(), "track"))
            .checked(checked)
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
            .child(
                SwitchThumb::new(checked)
                    .disabled(self.disabled)
                    .absolute()
                    .top(look.inset)
                    .left(thumb_left)
                    .size(look.thumb)
                    .rounded_full()
                    .bg(look.thumb_color)
                    .child(
                        div()
                            .size_full()
                            .debug_selector(|| "native-switch-thumb".into()),
                    ),
            );
        BaseSwitch::new(self.id)
            .checked(checked)
            .disabled(self.disabled)
            .when_some(self.label.clone(), |switch, label| {
                switch.accessibility_label(label)
            })
            .when_some(self.on_change, |switch, on_change| {
                switch.on_change(move |next, _, window, cx| on_change(&next, window, cx))
            })
            .group(HOVER_GROUP)
            .flex()
            .flex_row()
            .items_center()
            .gap(LABEL_GAP)
            .opacity(look.opacity)
            .child(track)
            .when_some(self.label, |switch, label| {
                switch.child(
                    div()
                        // The label's line box is the track's height, as
                        // gpui-component lays it out (switch.rs:237).
                        .line_height(look.track_height)
                        .text_size(label_size)
                        .font_weight(label_weight)
                        .text_color(look.label)
                        .debug_selector(|| "native-switch-label".into())
                        .child(label),
                )
            })
            .into_any_element()
    }
}
