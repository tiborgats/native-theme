//! Platform timing curves: libadwaita's named easings and its cubic Bézier solver, libadwaita's damped
//! spring, and Qt's named easings. All arithmetic is in `f64`.

/// The named easings of libadwaita that are not cubic Béziers, plus the identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AdwEasing {
    /// The identity: the output equals the input.
    Linear,
    /// The cube of the input.
    EaseInCubic,
    /// The cube of (input − 1), plus 1.
    EaseOutCubic,
    /// Half a cosine period, rising from 0 to 1.
    EaseInOutSine,
    /// A power of two that reaches 1 at the end and is forced to 0 at the start.
    EaseInExpo,
    /// One minus a falling power of two, forced to 1 at the end.
    EaseOutExpo,
}

/// The eased fraction of libadwaita's easing `easing` at the elapsed fraction `p`.
///
/// The input is not clamped: a fraction outside [0, 1] is evaluated by the same formula.
pub(crate) fn adw_ease(easing: AdwEasing, p: f64) -> f64 {
    match easing {
        AdwEasing::Linear => p,
        AdwEasing::EaseInCubic => p * p * p,
        AdwEasing::EaseOutCubic => {
            let u = p - 1.0;
            u * u * u + 1.0
        }
        AdwEasing::EaseInOutSine => -0.5 * ((std::f64::consts::PI * p).cos() - 1.0),
        AdwEasing::EaseInExpo => {
            if p.abs() < f64::EPSILON {
                0.0
            } else {
                2.0_f64.powf(10.0 * (p - 1.0))
            }
        }
        AdwEasing::EaseOutExpo => {
            if (p - 1.0).abs() < f64::EPSILON {
                1.0
            } else {
                -(2.0_f64.powf(-10.0 * p)) + 1.0
            }
        }
    }
}

/// One coordinate of the cubic Bézier from 0 to 1 with inner control coordinates `c1` and `c2`, at parameter `s`.
fn bezier_coordinate(c1: f64, c2: f64, s: f64) -> f64 {
    let r = 1.0 - s;
    3.0 * r * r * s * c1 + 3.0 * r * s * s * c2 + s * s * s
}

/// The curve parameter whose horizontal coordinate is `x`, found by exactly 30 bisection halvings of [0, 1].
fn bezier_solve(x1: f64, x2: f64, x: f64) -> f64 {
    let mut lo = 0.0_f64;
    let mut hi = 1.0_f64;
    for _ in 0..30 {
        let mid = (lo + hi) / 2.0;
        if x < bezier_coordinate(x1, x2, mid) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    (lo + hi) / 2.0
}

/// The eased fraction of the cubic Bézier through (0, 0), (`x1`, `y1`), (`x2`, `y2`), (1, 1) at the elapsed
/// fraction `p`, as libadwaita solves it.
///
/// A fraction within `f64::EPSILON` of 0 or of 1 gives exactly 0 or 1. Otherwise the curve's parameter is found by
/// bisection and its vertical coordinate returned. Nothing is clamped: `p` outside [0, 1] settles at the
/// parameter's nearest end, control points with `y` outside [0, 1] overshoot, and a non-monotone `x` returns
/// whatever bracket the bisection reaches.
pub(crate) fn cubic_bezier(x1: f64, y1: f64, x2: f64, y2: f64, p: f64) -> f64 {
    if p.abs() < f64::EPSILON {
        0.0
    } else if (p - 1.0).abs() < f64::EPSILON {
        1.0
    } else {
        bezier_coordinate(y1, y2, bezier_solve(x1, x2, p))
    }
}

/// The named easings of Qt's `QEasingCurve` that the model lists.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum QtEasing {
    /// The square of the input.
    InQuad,
    /// A parabola decelerating to 1.
    OutQuad,
    /// Two parabola halves meeting at 0.5.
    InOutQuad,
    /// The cube of the input.
    InCubic,
    /// The cube of (input − 1), plus 1.
    OutCubic,
    /// Two cubic halves meeting at 0.5.
    InOutCubic,
    /// Half a cosine period, rising from 0 to 1.
    InOutSine,
    /// One minus a falling power of two, scaled by 1.001, and exactly 1 at the end.
    OutExpo,
}

/// The fraction clamped to [0, 1] with comparison-based minimum and maximum, so that NaN becomes 0.
fn qt_clamp(p: f64) -> f64 {
    let upper = if 1.0 < p { 1.0 } else { p };
    if 0.0 < upper { upper } else { 0.0 }
}

/// The eased fraction of Qt's easing `easing` at the elapsed fraction `p`.
///
/// The fraction is first clamped to [0, 1]; NaN counts as 0.
pub(crate) fn qt_ease(easing: QtEasing, p: f64) -> f64 {
    let t = qt_clamp(p);
    match easing {
        QtEasing::InQuad => t * t,
        QtEasing::OutQuad => (-t) * (t - 2.0),
        QtEasing::InOutQuad => {
            let u = t * 2.0;
            if u < 1.0 {
                u * u / 2.0
            } else {
                let w = u - 1.0;
                -0.5 * (w * (w - 2.0) - 1.0)
            }
        }
        QtEasing::InCubic => t * t * t,
        QtEasing::OutCubic => {
            let u = t - 1.0;
            u * u * u + 1.0
        }
        QtEasing::InOutCubic => {
            let u = t * 2.0;
            if u < 1.0 {
                0.5 * u * u * u
            } else {
                let w = u - 2.0;
                0.5 * (w * w * w + 2.0)
            }
        }
        QtEasing::InOutSine => -0.5 * ((std::f64::consts::PI * t).cos() - 1.0),
        QtEasing::OutExpo => {
            if t == 1.0 {
                1.0
            } else {
                1.001 * (-(2.0_f64.powf(-10.0 * t)) + 1.0)
            }
        }
    }
}

/// The duration libadwaita reports for a spring that never comes to rest.
pub(crate) const SPRING_DURATION_INFINITE: u32 = u32::MAX;

/// The forward-difference step, in seconds, of the over-damped duration search.
const SPRING_DELTA: f64 = 0.001;

/// The last millisecond the clamped spring's search for its first zero examines before giving up.
const SPRING_FIRST_ZERO_LIMIT: u32 = 20_000;

/// The number of Newton repetitions after which the over-damped duration search gives up.
const SPRING_NEWTON_LIMIT: u32 = 1000;

/// The damping coefficient of a spring with damping ratio `ratio`, mass `mass` and stiffness `stiffness`: the
/// ratio times the critical damping 2·√(mass·stiffness).
///
/// None for a negative (or NaN) ratio, a negative mass·stiffness, or a result that is not finite.
pub(crate) fn damping_coefficient(ratio: f64, mass: f64, stiffness: f64) -> Option<f64> {
    if !(ratio >= 0.0 || ratio.abs() < f64::EPSILON) {
        return None;
    }
    let product = mass * stiffness;
    if product < 0.0 {
        return None;
    }
    let damping = ratio * (2.0 * product.sqrt());
    damping.is_finite().then_some(damping)
}

/// A damped spring animating a fraction from 0 to 1, in libadwaita's terms.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Spring {
    /// The damping ratio: 1 is critical, below 1 oscillates, above 1 creeps.
    pub(crate) damping_ratio: f64,
    /// The mass.
    pub(crate) mass: f64,
    /// The stiffness.
    pub(crate) stiffness: f64,
    /// The velocity at the start, in fractions per second; positive moves toward 1.
    pub(crate) initial_velocity: f64,
    /// How close to 1 the motion must come to count as at rest.
    pub(crate) rest_epsilon: f64,
    /// Whether the motion ends at its first arrival at 1 instead of settling.
    pub(crate) clamp: bool,
}

/// The start of the animated range.
const SPRING_FROM: f64 = 0.0;
/// The end of the animated range.
pub(crate) const SPRING_TO: f64 = 1.0;

/// A time in seconds as whole milliseconds, truncated; None where the conversion to an unsigned 32-bit integer
/// has no defined result (NaN, infinite, at or below −1, or too large).
pub(crate) fn seconds_to_ms(seconds: f64) -> Option<u32> {
    let ms = (seconds * 1000.0).trunc();
    (ms.is_finite() && ms >= 0.0 && ms <= f64::from(u32::MAX)).then_some(ms as u32)
}

impl Spring {
    /// The damping coefficient, when the parameters form a spring libadwaita accepts: a valid ratio, a positive
    /// mass and stiffness and a non-negative damping.
    fn damping(&self) -> Option<f64> {
        let damping = damping_coefficient(self.damping_ratio, self.mass, self.stiffness)?;
        let accepted = (damping >= 0.0 || damping.abs() < f64::EPSILON)
            && self.mass > 0.0
            && self.stiffness > 0.0;
        accepted.then_some(damping)
    }

    /// The position of the free motion at `tau_ms` milliseconds, before any duration cut-off.
    ///
    /// None when the parameters are refused or no regime applies (a NaN rate).
    pub(crate) fn position_at(&self, tau_ms: u32) -> Option<f64> {
        let damping = self.damping()?;
        self.regime(damping)?;
        Some(self.position(damping, tau_ms))
    }

    /// Which of the three regimes the parameters fall in, or None where neither comparison holds.
    fn regime(&self, damping: f64) -> Option<Regime> {
        let beta = damping / (2.0 * self.mass);
        let omega0 = (self.stiffness / self.mass).sqrt();
        if (beta - omega0).abs() < f64::from(f32::EPSILON) {
            Some(Regime::Critical)
        } else if beta < omega0 {
            Some(Regime::Under)
        } else if beta > omega0 {
            Some(Regime::Over)
        } else {
            None
        }
    }

    /// The position at `tau_ms` milliseconds for damping coefficient `damping`; NaN where no regime applies.
    fn position(&self, damping: f64, tau_ms: u32) -> f64 {
        let t = f64::from(tau_ms) / 1000.0;
        let beta = damping / (2.0 * self.mass);
        let omega0 = (self.stiffness / self.mass).sqrt();
        let x0 = SPRING_FROM - SPRING_TO;
        let v0 = self.initial_velocity;
        let envelope = (-beta * t).exp();
        match self.regime(damping) {
            Some(Regime::Critical) => SPRING_TO + envelope * (x0 + (beta * x0 + v0) * t),
            Some(Regime::Under) => {
                let omega1 = (omega0 * omega0 - beta * beta).sqrt();
                SPRING_TO
                    + envelope
                        * (x0 * (omega1 * t).cos()
                            + ((beta * x0 + v0) / omega1) * (omega1 * t).sin())
            }
            Some(Regime::Over) => {
                let omega2 = (beta * beta - omega0 * omega0).sqrt();
                SPRING_TO
                    + envelope
                        * (x0 * (omega2 * t).cosh()
                            + ((beta * x0 + v0) / omega2) * (omega2 * t).sinh())
            }
            None => f64::NAN,
        }
    }

    /// The first whole millisecond from 1 at which the motion reaches 1 or comes within the rest epsilon of it;
    /// 0 when none does by 20001 ms.
    fn first_zero(&self, damping: f64) -> u32 {
        let epsilon = self.rest_epsilon;
        let rising = SPRING_TO - SPRING_FROM > f64::EPSILON;
        let falling = SPRING_FROM - SPRING_TO > f64::EPSILON;
        let mut tau = 1_u32;
        let mut y = self.position(damping, tau);
        while (rising && SPRING_TO - y > epsilon) || (falling && y - SPRING_TO > epsilon) {
            if tau > SPRING_FIRST_ZERO_LIMIT {
                return 0;
            }
            tau = tau.saturating_add(1);
            y = self.position(damping, tau);
        }
        tau
    }

    /// The estimated duration in milliseconds after which the animation reports exactly 1.
    ///
    /// [`SPRING_DURATION_INFINITE`] for an undamped spring. For a clamped spring, its first arrival at 1. For a
    /// critical or under-damped spring, the time at which the decay envelope falls to the rest epsilon. For an
    /// over-damped spring, a Newton search for the time at which the motion is within the rest epsilon of 1
    /// (0 if it has not converged after 1001 repetitions). None when the parameters are refused or a time has no
    /// defined integer value (such as a rest epsilon of 1 or more, or a zero slope).
    pub(crate) fn duration_ms(&self) -> Option<u32> {
        let damping = self.damping()?;
        let beta = damping / (2.0 * self.mass);
        if beta.abs() < f64::EPSILON || beta < 0.0 {
            return Some(SPRING_DURATION_INFINITE);
        }
        if self.clamp {
            if (SPRING_TO - SPRING_FROM).abs() < f64::EPSILON {
                return Some(0);
            }
            return Some(self.first_zero(damping));
        }
        let epsilon = self.rest_epsilon;
        let mut s0 = -epsilon.ln() / beta;
        match self.regime(damping)? {
            Regime::Critical | Regime::Under => seconds_to_ms(s0),
            Regime::Over => {
                let value =
                    |seconds: f64| seconds_to_ms(seconds).map(|ms| self.position(damping, ms));
                let mut y0 = value(s0)?;
                let mut slope = (value(s0 + SPRING_DELTA)? - y0) / SPRING_DELTA;
                let mut s1 = (SPRING_TO - y0 + slope * s0) / slope;
                let mut y1 = value(s1)?;
                let mut repetitions = 0_u32;
                while (SPRING_TO - y1).abs() > epsilon {
                    if repetitions > SPRING_NEWTON_LIMIT {
                        return Some(0);
                    }
                    s0 = s1;
                    y0 = y1;
                    slope = (value(s0 + SPRING_DELTA)? - y0) / SPRING_DELTA;
                    s1 = (SPRING_TO - y0 + slope * s0) / slope;
                    y1 = value(s1)?;
                    repetitions = repetitions.saturating_add(1);
                }
                seconds_to_ms(s1)
            }
        }
    }

    /// The animation's value at `tau_ms` milliseconds: exactly 1 at and after the estimated duration, the motion's
    /// position before it. None when the parameters are refused or the duration is undefined.
    #[cfg_attr(
        not(test),
        allow(
            dead_code,
            reason = "the vector table samples by it; the glue (`motion.rs`'s `spring_at`) estimates the duration once"
        )
    )]
    pub(crate) fn value_at_ms(&self, tau_ms: u32) -> Option<f64> {
        let duration = self.duration_ms()?;
        if tau_ms >= duration {
            return Some(SPRING_TO);
        }
        self.position_at(tau_ms)
    }

    /// The animation's value at the fraction `t` of its estimated duration, the time truncated to whole
    /// milliseconds. None for a spring that never rests, refused parameters, or a time with no defined integer
    /// value (NaN or negative `t`).
    #[cfg_attr(
        not(test),
        allow(
            dead_code,
            reason = "the vector table samples by it; the glue (`motion.rs`'s `spring_at`) estimates the duration once"
        )
    )]
    pub(crate) fn value_at_fraction(&self, t: f64) -> Option<f64> {
        let duration = self.duration_ms()?;
        if duration == SPRING_DURATION_INFINITE {
            return None;
        }
        let tau = seconds_to_ms(t * f64::from(duration) / 1000.0)?;
        self.value_at_ms(tau)
    }
}

/// The three motions of a damped spring.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Regime {
    /// Critically damped.
    Critical,
    /// Under-damped: oscillating.
    Under,
    /// Over-damped: creeping.
    Over,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TABLE: &str = include_str!("../../../tests/fixtures/next/timing-rows.txt");

    #[derive(Debug, Clone, Copy)]
    enum Key {
        Qt(QtEasing),
        Adw(AdwEasing),
        Bezier([f64; 4]),
        Spring(f64, f64, bool),
        Skip,
    }

    fn number(token: &str) -> Option<f64> {
        let cleaned: String = token.chars().filter(|c| *c != '_').collect();
        cleaned.parse::<f64>().ok()
    }

    fn numbers(text: &str) -> Vec<f64> {
        text.split(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-' || c == '_' || c == 'e'))
            .filter_map(number)
            .collect()
    }

    fn qt_name(name: &str) -> Option<QtEasing> {
        Some(match name {
            "InQuad" => QtEasing::InQuad,
            "OutQuad" => QtEasing::OutQuad,
            "InOutQuad" => QtEasing::InOutQuad,
            "InCubic" => QtEasing::InCubic,
            "OutCubic" => QtEasing::OutCubic,
            "InOutCubic" => QtEasing::InOutCubic,
            "InOutSine" => QtEasing::InOutSine,
            "OutExpo" => QtEasing::OutExpo,
            _ => return None,
        })
    }

    fn adw_name(name: &str) -> Option<AdwEasing> {
        Some(match name {
            "EaseInCubic" => AdwEasing::EaseInCubic,
            "EaseOutCubic" => AdwEasing::EaseOutCubic,
            "EaseInOutSine" => AdwEasing::EaseInOutSine,
            "EaseInExpo" => AdwEasing::EaseInExpo,
            "EaseOutExpo" => AdwEasing::EaseOutExpo,
            _ => return None,
        })
    }

    /// The identifier after `prefix` in `line`, and the rest of the line after its closing parenthesis.
    fn after<'a>(line: &'a str, prefix: &str) -> Option<(&'a str, &'a str)> {
        let start = line.find(prefix)? + prefix.len();
        let rest = line.get(start..)?;
        let end = rest.find(')')?;
        Some((rest.get(..end)?, rest.get(end..)?))
    }

    /// A spring from the table's `spring(ratio, stiffness, clamp)`, its other parameters the table's fixed ones
    /// widened from single precision.
    fn spring(ratio: f64, stiffness: f64, clamp: bool) -> Spring {
        Spring {
            damping_ratio: f64::from(ratio as f32),
            mass: f64::from(1.0_f32),
            stiffness: f64::from(stiffness as f32),
            initial_velocity: f64::from(0.0_f32),
            rest_epsilon: f64::from(0.001_f32),
            clamp,
        }
    }

    fn spring_key(line: &str) -> Option<(f64, f64, bool)> {
        let (args, _) = after(line, "spring(")?;
        let parts: Vec<&str> = args.split(',').map(str::trim).collect();
        let ratio = number(parts.first()?)?;
        let stiffness = number(parts.get(1)?)?;
        let clamp = *parts.get(2)? == "true";
        Some((ratio, stiffness, clamp))
    }

    fn key_of(line: &str) -> Option<Key> {
        if let Some((name, _)) = after(line, "Qt(QtEasing::") {
            return qt_name(name).map(Key::Qt);
        }
        if let Some((name, _)) = after(line, "Adw(AdwEasing::") {
            return adw_name(name).map(Key::Adw);
        }
        if let Some((args, _)) = after(line, "Timing::CubicBezier(") {
            let v = numbers(args);
            return Some(Key::Bezier([
                *v.first()?,
                *v.get(1)?,
                *v.get(2)?,
                *v.get(3)?,
            ]));
        }
        if line.starts_with("&spring(") {
            let (r, k, c) = spring_key(line)?;
            return Some(Key::Spring(r, k, c));
        }
        if line.starts_with("&steps(") || line.starts_with("&Timing::Steps") {
            return Some(Key::Skip);
        }
        None
    }

    fn ulps_apart(a: f64, b: f64) -> u64 {
        if a == b {
            return 0;
        }
        let ia = a.to_bits() as i64;
        let ib = b.to_bits() as i64;
        if (ia < 0) != (ib < 0) {
            return u64::MAX;
        }
        ia.abs_diff(ib)
    }

    const CURVE_TS: [f64; 6] = [0.0, 0.25, 0.5, 0.75, 0.9, 1.0];
    const TIMED_TS: [f64; 7] = [0.0, 0.1, 0.25, 0.5, 0.75, 0.9, 1.0];

    struct Tally {
        passed: usize,
        failed: Vec<String>,
        skipped: Vec<String>,
    }

    fn check_values(key: Key, expected: &[f64], tally: &mut Tally) {
        let mut ok = true;
        let mut detail = String::new();
        for (index, want) in expected.iter().enumerate() {
            let (found, good) = match key {
                Key::Qt(e) => {
                    let f = qt_ease(e, CURVE_TS.get(index).copied().unwrap_or(f64::NAN));
                    (Some(f), ulps_apart(f, *want) <= 4)
                }
                Key::Adw(e) => {
                    let f = adw_ease(e, CURVE_TS.get(index).copied().unwrap_or(f64::NAN));
                    (Some(f), ulps_apart(f, *want) <= 4)
                }
                Key::Bezier([x1, y1, x2, y2]) => {
                    let p = TIMED_TS.get(index).copied().unwrap_or(f64::NAN);
                    let f = cubic_bezier(
                        f64::from(x1 as f32),
                        f64::from(y1 as f32),
                        f64::from(x2 as f32),
                        f64::from(y2 as f32),
                        p,
                    );
                    (Some(f), (f - want).abs() < 1e-12)
                }
                Key::Spring(r, k, c) => {
                    let p = TIMED_TS.get(index).copied().unwrap_or(f64::NAN);
                    let f = spring(r, k, c).value_at_fraction(p);
                    (f, f.is_some_and(|f| (f - want).abs() < 1e-12))
                }
                Key::Skip => (None, false),
            };
            if !good {
                ok = false;
                detail.push_str(&format!(" [{index}] want {want:e} found {found:?}"));
            }
        }
        if ok {
            tally.passed += 1;
        } else {
            tally.failed.push(format!("{key:?}:{detail}"));
        }
    }

    /// One `assert_eq!(evaluate(&…, p), expected)` line, checked; false when the timing is not one this file has.
    fn check_assert(line: &str, tally: &mut Tally) {
        let body = line.trim_start_matches("assert_eq!(evaluate(");
        let Some(split) = body.rfind("), ") else {
            tally.skipped.push(line.to_string());
            return;
        };
        let (call, expected) = (
            body.get(..split).unwrap_or(""),
            body.get(split + 3..).unwrap_or(""),
        );
        let Some(comma) = call.rfind(", ") else {
            tally.skipped.push(line.to_string());
            return;
        };
        let (timing, arg) = (
            call.get(..comma).unwrap_or(""),
            call.get(comma + 2..).unwrap_or(""),
        );
        let p = if arg == "f64::NAN" {
            f64::NAN
        } else {
            number(arg).unwrap_or(f64::NAN)
        };
        let want = if expected.starts_with("None") {
            None
        } else {
            numbers(expected.trim_start_matches("Some"))
                .first()
                .copied()
        };
        let found = if let Some((name, _)) = after(timing, "QtEasing::") {
            qt_name(name).map(|e| Some(qt_ease(e, p)))
        } else if timing == "&Timing::Linear" {
            Some(Some(adw_ease(AdwEasing::Linear, p)))
        } else if timing.starts_with("&spring(") {
            spring_key(timing).map(|(r, k, c)| spring(r, k, c).value_at_fraction(p))
        } else {
            None
        };
        match found {
            None => tally.skipped.push(line.to_string()),
            Some(found) if found == want => tally.passed += 1,
            Some(found) => tally.failed.push(format!("{line}: found {found:?}")),
        }
    }

    #[test]
    fn vector_table() {
        let mut tally = Tally {
            passed: 0,
            failed: Vec::new(),
            skipped: Vec::new(),
        };
        let mut timed = false;
        let mut key: Option<Key> = None;
        let mut values: Vec<f64> = Vec::new();
        let mut pending: Option<(QtEasing, f64)> = None;
        for raw in TABLE.lines() {
            let line = raw.trim();
            if line.starts_with("# cubic") {
                timed = true;
                continue;
            }
            if line.starts_with('#') || line.starts_with("let ") || line.starts_with("//") {
                key = None;
                values.clear();
                continue;
            }
            if line.starts_with("assert_eq!(evaluate(") {
                check_assert(line, &mut tally);
                continue;
            }
            if let Some(rest) = line.strip_prefix("evaluate(") {
                pending = after(rest, "QtEasing::")
                    .and_then(|(name, tail)| Some((qt_name(name)?, *numbers(tail).first()?)));
                continue;
            }
            if let Some((easing, p)) = pending.take() {
                let want = numbers(line.trim_start_matches("Some")).first().copied();
                let found = qt_ease(easing, p);
                if want.is_some_and(|w| ulps_apart(found, w) <= 4) {
                    tally.passed += 1;
                } else {
                    tally
                        .failed
                        .push(format!("{easing:?}({p}): want {want:?} found {found:e}"));
                }
                continue;
            }
            let tail = if let Some(found) = key_of(line) {
                if matches!(found, Key::Skip) {
                    tally.skipped.push(line.to_string());
                }
                key = Some(found);
                values.clear();
                match found {
                    Key::Qt(_) | Key::Adw(_) => {
                        line.rfind("), ").and_then(|i| line.get(i..)).unwrap_or("")
                    }
                    _ => "",
                }
            } else {
                line
            };
            let Some(current) = key else { continue };
            if matches!(current, Key::Skip) {
                continue;
            }
            values.extend(numbers(tail));
            let width = if timed {
                TIMED_TS.len()
            } else {
                CURVE_TS.len()
            };
            if values.len() >= width {
                check_values(current, &values, &mut tally);
                key = None;
                values.clear();
            }
        }
        println!(
            "passed {}, failed {}, skipped {}",
            tally.passed,
            tally.failed.len(),
            tally.skipped.len()
        );
        for skipped in &tally.skipped {
            println!("skipped: {skipped}");
        }
        for failed in &tally.failed {
            println!("FAILED: {failed}");
        }
        assert!(tally.failed.is_empty());
        assert_eq!(tally.passed, 25);
    }
}
