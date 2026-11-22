//! Motion: transitions and their timing curves, the motion tokens, the
//! dialog's entrance and a theme change, the text caret and the UI timings.

/// A property transition of a part.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transition {
    /// The duration, in milliseconds.
    pub duration_ms: u32,
    /// The delay before it starts, in milliseconds.
    pub delay_ms: u32,
    /// The timing curve.
    pub timing: Timing,
}

/// A timing curve.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[non_exhaustive]
pub enum Timing {
    /// Linear.
    #[default]
    Linear,
    /// A cubic Bézier curve through `(0, 0)`, `(x1, y1)`, `(x2, y2)`, `(1, 1)`.
    CubicBezier(f32, f32, f32, f32),
    /// A damped spring.
    Spring {
        /// The mass.
        mass: f32,
        /// The stiffness.
        stiffness: f32,
        /// The damping.
        damping: f32,
    },
    /// A step function of this many steps.
    Steps(u16),
    /// Sine ease-in-out.
    SineInOut,
    /// Exponential ease-out.
    ExpoOut,
}

/// The text caret; its colour is [`ColorRoles::caret`].
///
/// [`ColorRoles::caret`]: crate::next::ColorRoles::caret
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct CaretSpec {
    /// The width, in logical pixels; `None` where unstated (0.7.2 has none).
    pub width: Option<f32>, // None where unstated (0.7.2 has none)
    /// How long the caret shows in a blink, in milliseconds; at least 1
    /// where `Some`; `None` where unstated (the toolkit's default applies,
    /// as in 0.7.2).
    pub blink_on_ms: Option<u32>, // >= 1 when Some; None = unstated (toolkit default applies, as in 0.7.2)
    /// How long it hides in a blink, in milliseconds, stated together with
    /// `blink_on_ms`; `Some(0)` is a steady caret that does not blink.
    pub blink_off_ms: Option<u32>, // stated together with `blink_on_ms`; Some(0) = steady (does not blink)
    /// After how long it stops blinking, in milliseconds; at least 1 where
    /// `Some`; `None` where it blinks indefinitely or this is unstated.
    pub blink_timeout_ms: Option<u32>, // >= 1 when Some; None = blinks indefinitely or unstated
    /// The shape; `None` where unstated (0.7.2 has none).
    pub shape: Option<CaretShape>, // None where unstated (0.7.2 has none)
}

/// The shape of the text caret.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// no `Default`: the type stands only in `Option` fields, where a default would be an invented value
#[non_exhaustive]
pub enum CaretShape {
    /// A vertical bar.
    Bar,
    /// A block over a character.
    Block,
    /// A line under a character.
    Underline,
}

/// Timings of the user interface. Each is `None` where neither the preset
/// nor the live system states it; the toolkit's own value then applies.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct UiTimings {
    /// The delay before a tooltip shows, in milliseconds.
    pub tooltip_show_ms: Option<u32>,
    /// How long a tooltip stays, in milliseconds.
    pub tooltip_hide_ms: Option<u32>,
    /// The time within which a further tooltip shows at once, in
    /// milliseconds.
    pub tooltip_reshow_ms: Option<u32>,
    /// The double-click interval, in milliseconds.
    pub double_click_ms: Option<u32>,
    /// The distance a press must move to start a drag, in logical pixels.
    pub drag_threshold_px: Option<f32>,
    /// The delay before an overlay scroll bar hides, in milliseconds.
    pub scrollbar_auto_hide_ms: Option<u32>,
    /// The delay before a hovered submenu opens, in milliseconds.
    pub menu_hover_open_ms: Option<u32>,
    /// The delay before a held key repeats, in milliseconds.
    pub key_repeat_delay_ms: Option<u32>,
    /// The interval of a repeating key, in milliseconds.
    pub key_repeat_interval_ms: Option<u32>,
}

/// The motion tokens of a theme. 0.7.2 has no motion data beyond the
/// dialog's entrance: each is `None` until a source states it.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct MotionTokens {
    /// A fast transition.
    pub fast: Option<Transition>,
    /// The standard transition.
    pub standard: Option<Transition>,
    /// An emphasised transition.
    pub emphasized: Option<Transition>,
    /// A spring for spatial motion.
    pub spring_spatial: Option<Transition>,
    /// A spring for effects (colour, opacity).
    pub spring_effects: Option<Transition>,
    /// How a change of theme is shown.
    pub theme_change: Option<ThemeChangePolicy>,
}

/// How a dialog moves onto the screen as it opens. It names the motion
/// only; its durations and curves are the toolkit's.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DialogEntranceStyle {
    /// Fades in while scaling up to its full size: Windows, GNOME, Material.
    ScaleFade,
    /// Fades in while rising into place from below: KDE.
    FadeRise,
    /// Fades in, without scaling or moving.
    Fade,
    /// Appears at once, without motion.
    None,
}

/// How a change of theme is shown.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ThemeChangePolicy {
    /// At once.
    Instant,
    /// A crossfade from the old theme to the new.
    Crossfade {
        /// The crossfade's duration, in milliseconds.
        duration_ms: u32,
    },
    /// The parts' transitions suspended while the theme changes.
    SuspendTransitions {
        /// How long they are suspended, in milliseconds.
        duration_ms: u32,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caret_default_states_nothing() {
        let c = CaretSpec::default();
        assert_eq!(c.width, None);
        assert_eq!(c.blink_on_ms, None);
        assert_eq!(c.blink_off_ms, None);
        assert_eq!(c.blink_timeout_ms, None);
        assert_eq!(c.shape, None);
    }

    #[test]
    fn timings_default_states_nothing() {
        let UiTimings {
            tooltip_show_ms,
            tooltip_hide_ms,
            tooltip_reshow_ms,
            double_click_ms,
            drag_threshold_px,
            scrollbar_auto_hide_ms,
            menu_hover_open_ms,
            key_repeat_delay_ms,
            key_repeat_interval_ms,
        } = UiTimings::default();
        for ms in [
            tooltip_show_ms,
            tooltip_hide_ms,
            tooltip_reshow_ms,
            double_click_ms,
            scrollbar_auto_hide_ms,
            menu_hover_open_ms,
            key_repeat_delay_ms,
            key_repeat_interval_ms,
        ] {
            assert_eq!(ms, None);
        }
        assert_eq!(drag_threshold_px, None);
    }

    #[test]
    fn motion_default_states_nothing() {
        let MotionTokens {
            fast,
            standard,
            emphasized,
            spring_spatial,
            spring_effects,
            theme_change,
        } = MotionTokens::default();
        for transition in [fast, standard, emphasized, spring_spatial, spring_effects] {
            assert_eq!(transition, None);
        }
        assert_eq!(theme_change, None);
    }
}
