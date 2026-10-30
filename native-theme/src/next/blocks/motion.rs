//! Transitions and timing (S2.7): how a part's style changes over time, an
//! element's presentations, and repeating loops.
//!
//! The query is stateless: the connector compares successive query results
//! per element and part, and a transition starts when a property's resolved
//! value changes; the transition stated by the style after the change governs,
//! both directions. A property whose group has no transition takes its new
//! value at once: no transition is no animation, never a toolkit's default
//! one.

use native_theme_derive::ThemeParts;

use super::part_style::PartStylePatch;
use super::{KeyedItem, merge_by};
use crate::next::key::Key;
use crate::next::selector::PartRef;
use crate::next::units::{Fraction, Ms, Px, Scalar};
use crate::next::value::Value;

/// A timing function.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
pub enum Timing {
    /// CSS's `cubic-bezier(x1, y1, x2, y2)`.
    CubicBezier(f32, f32, f32, f32),
    /// CSS's `linear`.
    Linear,
    /// CSS's `steps(count, jump)`.
    Steps {
        /// The number of steps.
        count: u32,
        /// Where the jumps fall.
        jump: StepJump,
    },
    /// A spring, stated by its damping ratio: the damping coefficient the
    /// motion uses is `damping_ratio · 2·√(mass · stiffness)`
    /// ([`damping_coefficient`]); 0 is undamped, below 1 underdamped, 1
    /// critically damped, above 1 overdamped.
    Spring {
        /// The mass; a platform's spring with no mass of its own states the
        /// mass its evaluation uses.
        mass: f32,
        /// The stiffness.
        stiffness: f32,
        /// The damping as a ratio of critical damping.
        damping_ratio: f32,
        /// The initial velocity.
        initial_velocity: f32,
        /// The distance from rest below which the motion ends (libadwaita's
        /// `epsilon`, Compose's `visibilityThreshold`).
        rest_epsilon: f32,
        /// The motion ends where it first reaches its end value, with no
        /// overshoot (libadwaita's `clamp`).
        clamp: bool,
        /// The factor the spring's time is scaled by:
        /// `= token.motion.speed_factor` where the platform scales springs,
        /// else 1; zero jumps to the end.
        time_scale: Value<f32>,
    },
    /// A platform's own curve, implemented once in native-theme to its cited
    /// source.
    Platform(PlatformCurve),
}

/// CSS's step positions (`steps()`'s second argument).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StepJump {
    /// `jump-start`.
    JumpStart,
    /// `jump-end`.
    JumpEnd,
    /// `jump-none`.
    JumpNone,
    /// `jump-both`.
    JumpBoth,
}

/// A platform's own timing curve; closed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlatformCurve {
    /// Kirigami's velocity-based animations.
    KirigamiVelocity,
}

/// The damping coefficient a spring stated by its damping ratio uses:
/// `damping_ratio · 2·√(mass · stiffness)` (`libadwaita@1.10.0
/// src/adw-spring-params.c:93-94`). `None` where `mass · stiffness` is
/// negative or the result is not finite.
#[must_use]
pub fn damping_coefficient(mass: f32, stiffness: f32, damping_ratio: f32) -> Option<f32> {
    let critical = critical_damping(mass, stiffness)?;
    Some(damping_ratio * critical).filter(|c| c.is_finite())
}

/// The damping ratio of a spring a source states by its damping coefficient
/// (libadwaita's `adw_spring_params_new_full`, AppKit's
/// `CASpringAnimation.damping`): [`damping_coefficient`]'s inverse,
/// `coefficient / (2·√(mass · stiffness))`. `None` where `mass · stiffness`
/// is not above zero or the result is not finite.
#[must_use]
pub fn damping_ratio(mass: f32, stiffness: f32, coefficient: f32) -> Option<f32> {
    let critical = critical_damping(mass, stiffness).filter(|c| *c > 0.0)?;
    Some(coefficient / critical).filter(|r| r.is_finite())
}

/// `2·√(mass · stiffness)`, the critical damping; `None` where it is not a
/// finite number.
fn critical_damping(mass: f32, stiffness: f32) -> Option<f32> {
    let product = mass * stiffness;
    if product < 0.0 {
        return None;
    }
    Some(2.0 * product.sqrt()).filter(|c| c.is_finite())
}

/// The groups of a part's properties a transition animates. The focus
/// indicator's motion is its own `transition` (S2.10).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PropertyGroup {
    /// The fill.
    Fill,
    /// The stroke.
    Stroke,
    /// The corners.
    Corners,
    /// The shape: a morph from the old outline to the new.
    Shape,
    /// The shadow.
    Shadow,
    /// The opacity.
    Opacity,
    /// The transform.
    Transform,
    /// The text style.
    Text,
    /// The icon style.
    Icon,
    /// The glyph.
    Glyph,
    /// The laid-out box: a transition also starts when the connector's own
    /// layout moves the element or part.
    Geometry,
}

/// A transition of a part's property groups.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct Transition {
    /// The property groups it animates. In a list of transitions it merges by
    /// group, a transition listing several groups split per group first
    /// (S6.1).
    pub properties: Vec<PropertyGroup>,
    /// How the change is drawn.
    #[parts(block = "set")]
    pub kind: TransitionKind,
    /// The duration; absent only for a spring, whose time is its own (S12).
    #[parts(may_be_absent)]
    pub duration: Value<Ms>,
    /// The delay before it starts.
    pub delay: Value<Ms>,
    /// The timing: a timing, or a reference to a timing token.
    #[parts(block = "set")]
    pub timing: Value<Timing>,
    /// A theme change suspends it (libadwaita's CSS transitions are, its
    /// `AdwAnimation`s are not).
    pub suspended_by_theme_change: bool,
}

impl KeyedItem for Transition {
    fn merge_keyed(earlier: &mut Vec<Self>, later: Vec<Self>) {
        let mut held = split_by_group(std::mem::take(earlier));
        merge_by(&mut held, split_by_group(later), |a, b| {
            a.properties == b.properties
        });
        *earlier = held;
    }
}

/// Each transition as one per property group it lists.
fn split_by_group(transitions: Vec<Transition>) -> Vec<Transition> {
    transitions
        .into_iter()
        .flat_map(|transition| {
            let groups = transition.properties.clone();
            groups.into_iter().map(move |group| Transition {
                properties: vec![group],
                ..transition.clone()
            })
        })
        .collect()
}

/// How a transition draws a change.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[expect(
    clippy::large_enum_variant,
    reason = "S2.7's variants as stated; a transition is read once per change, not drawn per frame"
)]
pub enum TransitionKind {
    /// The old value interpolated to the new: layers matched by stack
    /// position, appearing layers from zero alpha, colours in premultiplied
    /// sRGB.
    Interpolate,
    /// A ripple from the press point; each value may be absent where the
    /// platform has no counterpart (S6.3).
    Ripple {
        /// The ripple's largest radius.
        #[parts(may_be_absent)]
        max_radius: Value<Px>,
        /// The ripple's initial scale.
        #[parts(may_be_absent)]
        initial_scale: Value<f32>,
        /// The ripple's padding.
        #[parts(may_be_absent)]
        padding: Value<Px>,
        /// The ripple's soft edge.
        #[parts(may_be_absent)]
        soft_edge: Value<Px>,
        /// The ripple's opacity.
        #[parts(may_be_absent)]
        opacity: Value<Fraction>,
        /// The ripple's fade.
        #[parts(may_be_absent)]
        fade: Value<Ms>,
        /// The ripple's minimum press.
        #[parts(may_be_absent)]
        min_press: Value<Ms>,
        /// The ripple's delay after a touch.
        #[parts(may_be_absent)]
        touch_delay: Value<Ms>,
    },
    /// The focus indicator travelling between elements.
    FocusMove,
    /// A platform's keyed transition (SF Symbols' content transitions and
    /// effects, WinUI's `AnimatedIcon` states), its parameters measured.
    Keyed {
        /// The transition, declared by the preset (S10).
        key: Key,
        /// Its parameters.
        params: Vec<(Key, Value<Scalar>)>,
    },
}

/// One event's presentation: its steps run together.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct PresentationTransition {
    /// The event. In a list of presentations it merges by event (S6.1).
    pub event: PresentationEvent,
    /// The steps, run together.
    #[parts(block = "fields")]
    pub steps: Vec<PresentationStep>,
}

impl KeyedItem for PresentationTransition {
    fn merge_keyed(earlier: &mut Vec<Self>, later: Vec<Self>) {
        merge_by(earlier, later, |a, b| a.event == b.event);
    }
}

/// One step of a presentation.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct PresentationStep {
    /// The part it moves: a part of the widget or of one of its element
    /// parts; `None`: the whole element.
    pub part: Option<PartRef>,
    /// What the step draws.
    #[parts(block = "set")]
    pub kind: PresentationKind,
    /// The duration; absent only for a spring (S12).
    #[parts(may_be_absent)]
    pub duration: Value<Ms>,
    /// The delay before it starts.
    pub delay: Value<Ms>,
    /// The timing.
    #[parts(block = "set")]
    pub timing: Value<Timing>,
    /// The time between items, where the element presents items.
    pub stagger: Value<Ms>,
}

/// The event a presentation runs on.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PresentationEvent {
    /// The element appears.
    Appear,
    /// The element disappears.
    Disappear,
    /// The element moves.
    Move,
    /// A widget-specific event, declared (S10).
    Keyed(Key),
}

/// Where a clip reveal starts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RevealOrigin {
    /// The anchor the element is placed against.
    Anchor,
    /// The element's centre.
    Center,
    /// One of the element's edges.
    Edge(EdgeSide),
}

/// What a presentation step draws.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
pub enum PresentationKind {
    /// A fade.
    Fade,
    /// A slide in from an edge.
    Slide {
        /// The edge it slides from.
        from: EdgeSide,
        /// The distance it slides.
        distance: Value<Px>,
    },
    /// A translation, stated for a left-to-right layout: `dx` is mirrored
    /// under right-to-left (S2.4).
    Translate {
        /// The horizontal displacement.
        dx: Value<Px>,
        /// The vertical displacement.
        dy: Value<Px>,
    },
    /// A scale from a factor.
    Scale {
        /// The scale it starts from.
        from: f32,
    },
    /// A morph.
    Morph,
    /// A clip growing from an origin.
    ClipReveal {
        /// Where the clip starts.
        from: RevealOrigin,
    },
    /// A platform's keyed presentation, its parameters measured.
    Keyed {
        /// The presentation kind, declared by the preset (S10).
        key: Key,
        /// Its parameters.
        params: Vec<(Key, Value<Scalar>)>,
    },
    /// No animation: the change is at once.
    Instant,
}

/// An edge of an element, as a presentation names it; `Start` and `End` are
/// mirrored with the layout (S2.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EdgeSide {
    /// The top edge.
    Top,
    /// The bottom edge.
    Bottom,
    /// The start edge.
    Start,
    /// The end edge.
    End,
    /// The edge toward the anchor (WinUI's menus).
    TowardAnchor,
}

/// A repeating animation.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct Loop {
    /// The period; never absent, as the keyframes' `at` are fractions of it.
    pub period: Value<Ms>,
    /// The timing between keyframes where a keyframe states none.
    #[parts(block = "set")]
    pub timing: Value<Timing>,
    /// The keyframes.
    #[parts(block = "fields")]
    pub keyframes: Vec<Keyframe>,
}

/// A keyframe of a loop.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct Keyframe {
    /// Where in the period it falls.
    pub at: Fraction,
    /// The part it styles; `None`: the part the loop is on. A loop that is a
    /// widget's or component's value is on no part, so each of its
    /// keyframes names one (S12).
    pub part: Option<PartRef>,
    /// The timing to the next keyframe; absent: the loop's.
    #[parts(block = "set", may_be_absent)]
    pub timing: Value<Timing>,
    /// The part's style at this keyframe, a shape included.
    pub patch: PartStylePatch,
}
