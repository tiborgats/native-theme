//! Transitions and timing (S2.7): how a part's style changes over time, an
//! element's presentations, and repeating loops.
//!
//! The query is stateless: the connector compares successive query results
//! per element and part, and a transition starts when a property's resolved
//! value changes; the transition stated by the style after the change governs,
//! both directions: among its transitions for the property's group that apply
//! from the element's kept flags before the change (`Transition.from`), the
//! later in stated order ([`governing`]). A property whose group has no such
//! transition takes its new value at once: no transition is no animation,
//! never a toolkit's default one.
//!
//! Appearing layers interpolate from zero alpha, layers match by stack
//! position, colours interpolate in premultiplied sRGB. A stroke or an edge
//! whose width becomes absent (the whole stroke stated `Absent`, or the
//! edge) interpolates to zero width and its paint to zero alpha, and one that
//! appears from them, a width interpolated to zero or below ending it; an
//! edge whose paint alone becomes absent keeps its width, its paint
//! interpolating to zero alpha.

use native_theme_derive::ThemeParts;

use super::part_style::{PartStyle, PartStylePatch, ResolvedPartStylePatch};
use super::{KeyedItem, merge_keyed_by, timing};
use crate::next::key::Key;
use crate::next::selector::{PartRef, StateFlag, StateFlags};
use crate::next::units::{Fraction, Ms, Px, Scalar};
use crate::next::value::Value;

/// A timing function.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block, check)]
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
    /// source ([`evaluate`]).
    Platform(#[parts(block = "set")] PlatformCurve),
}

/// CSS's step positions (`steps()`'s second argument).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
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

/// A platform's own timing curve; closed. A curve a toolkit computes by a
/// function of its own is one, named as the toolkit names it and implemented
/// to that function ([`evaluate`]), even where it equals a cubic Bézier in
/// exact arithmetic: it keeps the toolkit's rounding and constants (S2.7,
/// provisional: CONTRA-7). A curve whose control points the source writes is
/// a [`Timing::CubicBezier`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(block)]
pub enum PlatformCurve {
    /// Kirigami's velocity-based animations.
    KirigamiVelocity,
    /// A `QEasingCurve::Type` Qt computes in code.
    Qt(QtEasing),
    /// An `AdwEasing` libadwaita computes in code.
    Adw(AdwEasing),
}

/// Qt's easing types the presets use, each to `qtbase@6.12.0
/// src/3rdparty/easing/easing.cpp` through `curveToFunc`
/// (`src/corelib/tools/qeasingcurve.cpp:1000-1048`); QML's `Easing.<type>`
/// is the same `QEasingCurve`. A member is added, implemented to its source,
/// with the fact that needs it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum QtEasing {
    /// `easeInQuad`: `t²`.
    InQuad,
    /// `easeOutQuad`: `−t(t − 2)`.
    OutQuad,
    /// `easeInOutQuad`: two quadratic pieces.
    InOutQuad,
    /// `easeInCubic`: `t³`.
    InCubic,
    /// `easeOutCubic`: `(t − 1)³ + 1`.
    OutCubic,
    /// `easeInOutCubic`: two cubic pieces.
    InOutCubic,
    /// `easeInOutSine`: `−(cos(πt) − 1) / 2`.
    InOutSine,
    /// `easeOutExpo`: `1.001 · (1 − 2^(−10t))`, 1 at `t = 1`.
    OutExpo,
}

/// libadwaita's easings the presets use, each to `libadwaita@1.10.0
/// src/adw-easing.c` through `adw_easing_ease`. A member is added,
/// implemented to its source, with the fact that needs it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum AdwEasing {
    /// `ease_in_cubic`: `t³`.
    EaseInCubic,
    /// `ease_out_cubic`: `(t − 1)³ + 1`.
    EaseOutCubic,
    /// `ease_in_out_sine`: `−(cos(πt) − 1) / 2`.
    EaseInOutSine,
    /// `ease_in_expo`: `2^(10(t − 1))`, 0 at `t = 0`.
    EaseInExpo,
    /// `ease_out_expo`: `1 − 2^(−10t)`, 1 at `t = 1`.
    EaseOutExpo,
}

/// The fraction of the way from the old value to the new `timing` gives at
/// the elapsed fraction `t` (S2.7's forward reading), in doubles, as the
/// toolkits compute it. `t` is clamped to `0..=1`, the elapsed fraction's
/// range, as `QEasingCurve::valueForProgress` clamps it (`qtbase@6.12.0
/// src/corelib/tools/qeasingcurve.cpp:1469-1478`), and a non-finite `t` is
/// 0.
///
/// `Linear` is `t`. A cubic Bézier is solved for `t` as libadwaita solves the
/// curves it passes its control points to (`libadwaita@1.10.0
/// src/adw-easing.c:477-541`: `x(s) = t` by thirty bisections, `y(s)`, the
/// ends exact). Steps are CSS's step easing function with its jump (CSS
/// Easing Functions Level 1, §2.3.2, the input progress `t`). A spring is
/// libadwaita's `AdwSpringAnimation` from 0 to 1 (`adw-spring-animation.c`),
/// its damping coefficient from the damping ratio ([`damping_coefficient`],
/// `adw_spring_params_new`): `t` the elapsed fraction of the spring's own
/// duration ([`spring_duration`]), the one libadwaita estimates from
/// `rest_epsilon` and `clamp` (`calculate_duration`, `:177-243`), whole
/// milliseconds as it counts them, and the value its `oscillate` gives there
/// (`:101-153`), the end value at and past the duration (`:265-282`);
/// `time_scale` scales that duration, not the curve. The curves, the solver
/// and the spring are the clean room's (the plan's D10), written from
/// behavioural descriptions of those functions and their vector tables. A
/// platform curve is computed as the toolkit's own function
/// computes it; the result may leave `0..=1` where the toolkit's does (Qt's
/// out-expo scales by 1.001, so it passes 1 just before its end; a spring
/// overshoots).
///
/// `None` for [`PlatformCurve::KirigamiVelocity`], whose motion follows the
/// velocity, not the elapsed fraction alone; for steps with no jump to make
/// (`count` 0, or 1 with `jump-none`); and for a spring whose motion never
/// ends (undamped, libadwaita's infinite duration) or whose parameters give
/// no finite motion.
#[must_use]
pub fn evaluate(timing: &Timing, t: f64) -> Option<f64> {
    let t = if t.is_finite() {
        t.clamp(0.0, 1.0)
    } else {
        0.0
    };
    match timing {
        Timing::Linear => Some(timing::adw_ease(timing::AdwEasing::Linear, t)),
        Timing::CubicBezier(x1, y1, x2, y2) => Some(timing::cubic_bezier(
            f64::from(*x1),
            f64::from(*y1),
            f64::from(*x2),
            f64::from(*y2),
            t,
        )),
        Timing::Steps { count, jump } => steps(t, *count, *jump),
        Timing::Spring { .. } => spring(timing)?.value_at_fraction(t),
        Timing::Platform(PlatformCurve::Qt(easing)) => Some(timing::qt_ease(qt_easing(*easing), t)),
        Timing::Platform(PlatformCurve::Adw(easing)) => {
            Some(timing::adw_ease(adw_easing(*easing), t))
        }
        Timing::Platform(PlatformCurve::KirigamiVelocity) => None,
    }
}

/// A spring's estimated duration (S2.7): the time libadwaita's
/// `AdwSpringAnimation` takes from 0 to 1 before it reports its end value,
/// in whole milliseconds as it counts them, before `time_scale`, which scales
/// it. A connector gives [`evaluate`] the elapsed time over this duration
/// times the resolved `time_scale`. `None` for any other timing, for a
/// spring whose motion never ends (undamped) and for parameters that give no
/// finite motion.
#[must_use]
pub fn spring_duration(timing: &Timing) -> Option<Ms> {
    let duration = spring(timing)?.duration_ms()?;
    (duration != timing::SPRING_DURATION_INFINITE).then_some(Ms(duration as f32))
}

/// A spring timing's motion from 0 to 1, its parameters widened to doubles.
fn spring(timing: &Timing) -> Option<timing::Spring> {
    let Timing::Spring {
        mass,
        stiffness,
        damping_ratio,
        initial_velocity,
        rest_epsilon,
        clamp,
        ..
    } = timing
    else {
        return None;
    };
    Some(timing::Spring {
        damping_ratio: f64::from(*damping_ratio),
        mass: f64::from(*mass),
        stiffness: f64::from(*stiffness),
        initial_velocity: f64::from(*initial_velocity),
        rest_epsilon: f64::from(*rest_epsilon),
        clamp: *clamp,
    })
}

/// The clean-room curve of a Qt easing the model names.
fn qt_easing(easing: QtEasing) -> timing::QtEasing {
    match easing {
        QtEasing::InQuad => timing::QtEasing::InQuad,
        QtEasing::OutQuad => timing::QtEasing::OutQuad,
        QtEasing::InOutQuad => timing::QtEasing::InOutQuad,
        QtEasing::InCubic => timing::QtEasing::InCubic,
        QtEasing::OutCubic => timing::QtEasing::OutCubic,
        QtEasing::InOutCubic => timing::QtEasing::InOutCubic,
        QtEasing::InOutSine => timing::QtEasing::InOutSine,
        QtEasing::OutExpo => timing::QtEasing::OutExpo,
    }
}

/// The clean-room curve of a libadwaita easing the model names.
fn adw_easing(easing: AdwEasing) -> timing::AdwEasing {
    match easing {
        AdwEasing::EaseInCubic => timing::AdwEasing::EaseInCubic,
        AdwEasing::EaseOutCubic => timing::AdwEasing::EaseOutCubic,
        AdwEasing::EaseInOutSine => timing::AdwEasing::EaseInOutSine,
        AdwEasing::EaseInExpo => timing::AdwEasing::EaseInExpo,
        AdwEasing::EaseOutExpo => timing::AdwEasing::EaseOutExpo,
    }
}

/// The damping coefficient a spring stated by its damping ratio uses:
/// `damping_ratio · 2·√(mass · stiffness)`, in doubles as libadwaita
/// computes it, then narrowed. `None` where `mass · stiffness` or the ratio
/// is negative, or the result is not finite.
#[must_use]
pub fn damping_coefficient(mass: f32, stiffness: f32, damping_ratio: f32) -> Option<f32> {
    let coefficient = timing::damping_coefficient(
        f64::from(damping_ratio),
        f64::from(mass),
        f64::from(stiffness),
    )?;
    Some(coefficient as f32).filter(|c| c.is_finite())
}

/// CSS's step easing function (CSS Easing Functions Level 1, §2.3.2) at the
/// input progress `t` in `0..=1`: the current step `floor(t · count)`, one
/// more for `jump-start` and `jump-both`, at most the number of jumps (the
/// steps, one more for `jump-both`, one fewer for `jump-none`), over that
/// number. `None` where there is no jump to make.
fn steps(t: f64, count: u32, jump: StepJump) -> Option<f64> {
    let count = f64::from(count);
    let jumps = match jump {
        StepJump::JumpStart | StepJump::JumpEnd => count,
        StepJump::JumpBoth => count + 1.0,
        StepJump::JumpNone => count - 1.0,
    };
    if jumps < 1.0 || count < 1.0 {
        return None;
    }
    let mut step = (t * count).floor();
    if matches!(jump, StepJump::JumpStart | StepJump::JumpBoth) {
        step += 1.0;
    }
    Some(step.clamp(0.0, jumps) / jumps)
}

/// The groups of a part's properties a transition animates. The focus
/// indicator's motion is its own `transition` (S2.10).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
pub enum PropertyGroup {
    /// The fill.
    Fill,
    /// The stroke, its edges' widths too where no `StrokeWidth` transition
    /// applies in the style after the change.
    Stroke,
    /// The stroke's edges' widths: where a transition of it applies, it times
    /// them in `Stroke`'s place.
    StrokeWidth,
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
    /// The state layer of the part's `interaction` (S2.12): its alpha alone;
    /// its colour changes at once.
    StateLayer,
}

/// A transition of a part's property groups.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block, check)]
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
    /// The states it applies from: a change where the element held at least
    /// one of these flags before it, taken after the exclusive groups' drop
    /// ([`ResolvedTheme::kept_states`](crate::next::ResolvedTheme::kept_states), S8); `None`:
    /// from any state.
    pub from: Option<Vec<StateFlag>>,
}

/// A list of transitions merges by property group and `from` set (S6.1),
/// two sets of the same flags one key.
impl KeyedItem for Transition {
    fn merge_keyed(earlier: &mut Vec<Self>, later: Vec<Self>) {
        let mut held = split_by_group(std::mem::take(earlier));
        merge_keyed_by(&mut held, split_by_group(later), |a, b| {
            a.properties == b.properties && same_flags(a.from.as_deref(), b.from.as_deref())
        });
        *earlier = held;
    }
}

/// Whether two `from` sets hold the same flags; `None`, any state, only
/// matches itself.
pub(crate) fn same_flags(a: Option<&[StateFlag]>, b: Option<&[StateFlag]>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => a.iter().all(|f| b.contains(f)) && b.iter().all(|f| a.contains(f)),
        (None, Some(_)) | (Some(_), None) => false,
    }
}

/// The transition governing a change of `group`'s properties (S2.7): among
/// `transitions`, the style's after the change, the last in stated order
/// that lists the group and applies from `before`, the element's kept flags
/// before the change (no `from`, or at least one of its flags held). For
/// [`PropertyGroup::StrokeWidth`], the stroke's widths, the `Stroke`
/// transition governs where no `StrokeWidth` one applies, as `Stroke` times
/// every field of the stroke but where a `StrokeWidth` transition applies.
/// `None`: the new value at once.
#[must_use]
pub fn governing(
    transitions: &[ResolvedTransition],
    group: PropertyGroup,
    before: StateFlags,
) -> Option<&ResolvedTransition> {
    let last = |group: PropertyGroup| {
        transitions.iter().rev().find(|transition| {
            transition.properties.contains(&group)
                && transition
                    .from
                    .as_ref()
                    .is_none_or(|from| from.iter().any(|flag| before.contains(flag)))
        })
    };
    match group {
        PropertyGroup::StrokeWidth => last(group).or_else(|| last(PropertyGroup::Stroke)),
        _ => last(group),
    }
}

/// Each transition as one per property group it lists; one listing no group
/// is kept under its empty list, for validation to refuse (S12).
fn split_by_group(transitions: Vec<Transition>) -> Vec<Transition> {
    let mut out = Vec::new();
    for transition in transitions {
        if transition.properties.is_empty() {
            out.push(transition);
            continue;
        }
        let groups = transition.properties.clone();
        out.extend(groups.into_iter().map(|group| Transition {
            properties: vec![group],
            ..transition.clone()
        }));
    }
    out
}

/// How a transition draws a change.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block, check)]
pub enum TransitionKind {
    /// The old value interpolated to the new: layers matched by stack
    /// position, appearing layers from zero alpha, colours in premultiplied
    /// sRGB.
    Interpolate,
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
        merge_keyed_by(earlier, later, |a, b| a.event == b.event);
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
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
#[parts(block, check)]
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
        #[parts(signed)]
        dx: Value<Px>,
        /// The vertical displacement.
        #[parts(signed)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ThemeParts)]
#[parts(toml)]
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
#[parts(block, check)]
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
    #[parts(patch)]
    pub patch: PartStylePatch,
}
