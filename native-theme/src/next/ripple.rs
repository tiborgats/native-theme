// SPDX-License-Identifier: Apache-2.0
//
// This file ports the Android Open Source Project's
// graphics/java/android/graphics/drawable/RippleAnimationSession.java and
// RippleShader.java (frameworks/base at android-17.0.0_r1), and the formulas
// of androidx's compose/material/material-ripple Ripple.android.kt and
// RippleHostView.android.kt and compose/ui/ui-graphics Color.kt (androidx at
// e4d933c9), whose notices follow; it stays under that licence whatever
// licence the rest of the crate is used under (the 0.8.0 plan's D10). The
// crate's LICENSE-APACHE holds the licence, its NOTICE the notice of
// frameworks/base's NOTICE file.
//
// Copyright (C) 2020 The Android Open Source Project
// Copyright (C) 2021 The Android Open Source Project
// Copyright 2019 The Android Open Source Project
// Copyright 2021 The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A press ripple's three functions (S2.12): how far the effect has run,
//! its noise phase, and its colour at a point, each implemented once to its
//! source.
//!
//! The connector starts the effect each time `pressed` becomes kept, takes
//! the block's colour and alpha as resolved for the first frame it draws
//! after that, and counts `since_start` from that frame; when `pressed`
//! stops being kept, the release, the effect's exit starts, and it ends by
//! its own lifecycle (`progress` returning `None`). Several presses run
//! several ripples, each drawn out. The box the effect is drawn in follows
//! the element's while it runs.
//!
//! [`RippleEffect::AndroidPatterned`] is Android's `RippleDrawable` in its
//! patterned style (`frameworks-base@android-17.0.0_r1
//! graphics/java/android/graphics/drawable/`): `RippleAnimationSession.java`'s
//! lifecycle and `RippleShader.java`'s colour, with the inputs Compose gives
//! it (`androidx@e4d933c9 compose/material3/material3-ripple/src/androidMain/
//! kotlin/androidx/compose/material3/ripple/`). The RenderThread draws it
//! through hwui's `AnimatedDrawables.h`, outside the source cache, which
//! `RippleShader.java:160-162` says is kept in sync with the shader; this
//! follows the shader. The vectors are `tools/motion-vectors`'.

use crate::next::blocks::interaction::{ResolvedRippleStyle, RippleEffect};
use crate::next::color::Color;
use crate::next::units::{Ms, Point, Rect};

/// `ENTER_ANIM_DURATION` and `EXIT_ANIM_DURATION`, in ms
/// (`RippleAnimationSession.java:42-43`).
const ENTER_MS: f64 = 450.0;
const EXIT_MS: f64 = 375.0;
/// The progress the enter reaches and the exit starts from
/// (`RippleAnimationSession.java:114`, `:179`, `:208`).
const ENTERED: f64 = 0.5;
/// `NOISE_ANIMATION_DURATION` and `MAX_NOISE_PHASE` (`:44-45`), the latter
/// `7000 / 214` in Java's integer division.
const NOISE_MS: f64 = 7000.0;
const MAX_NOISE_PHASE: f32 = 32.0;
/// `DEFAULT_EFFECT_COLOR`, the sparkles' colour (`RippleDrawable.java:159`),
/// `0x8dffffff`.
const SPARKLE_ALPHA: u8 = 0x8d;
/// The centre reaches the box's centre at half the progress: the progress
/// doubled, saturated (`RippleShader.java:107`).
const TOWARD_ORIGIN: f32 = 2.0;
/// `setRadius`'s factor (`RippleShader.java:139`).
const RADIUS_FACTOR: f32 = 2.3;

/// How far the effect has run, `since_start` ms after its first frame and,
/// once released, `since_release` ms after the release; `None`: ended.
///
/// `AndroidPatterned` (`RippleAnimationSession.java:113-139`, `:150-220`):
/// from 0 to 0.5 over 450 ms by `(0.4, 0, 0.2, 1)` (`FAST_OUT_SLOW_IN`,
/// `:47-48`, solved exactly: `PathInterpolator` approximates the curve
/// through the native `Path.approximate`, outside the source cache, to its
/// `PRECISION` of 0.002 (`PathInterpolator.java:54`, `:164`), so Android's
/// progress differs from this by about 0.001 at most), then from
/// 0.5 to 1 over 375 ms linearly from the later of the release and the
/// enter's end (`computeDelay`, `:136-139`), ended there. A non-finite time
/// is 0 (S2.2's totality).
#[must_use]
pub fn progress(effect: RippleEffect, since_start: Ms, since_release: Option<Ms>) -> Option<f32> {
    match effect {
        RippleEffect::AndroidPatterned => {
            let since_start = finite(f64::from(since_start.0)).max(0.0);
            let enter = if since_start >= ENTER_MS {
                ENTERED
            } else {
                ENTERED * fast_out_slow_in(since_start / ENTER_MS)
            };
            let Some(since_release) = since_release else {
                return Some(enter as f32);
            };
            let released = (since_start - finite(f64::from(since_release.0))).max(0.0);
            let exit_start = released.max(ENTER_MS);
            if since_start < exit_start {
                return Some(enter as f32);
            }
            let exit = since_start - exit_start;
            (exit < EXIT_MS).then(|| (ENTERED + (1.0 - ENTERED) * (exit / EXIT_MS)) as f32)
        }
    }
}

/// The effect's noise phase, `since_start` ms after its first frame, whose
/// uptime was `start_uptime`.
///
/// `AndroidPatterned`: from the first frame's uptime by `MAX_NOISE_PHASE`
/// (32) over 7000 ms linearly, then held (`RippleAnimationSession.java:181-
/// 220`; `RippleDrawable.java:999`), in `float` as `ValueAnimator.ofFloat`
/// evaluates it, so the sparkles differ per press.
#[must_use]
pub fn phase(effect: RippleEffect, start_uptime: Ms, since_start: Ms) -> f32 {
    match effect {
        RippleEffect::AndroidPatterned => {
            let start = start_uptime.0;
            let end = start + MAX_NOISE_PHASE;
            let fraction = (finite(f64::from(since_start.0)) / NOISE_MS).clamp(0.0, 1.0) as f32;
            start + fraction * (end - start)
        }
    }
}

/// The effect's colour at `point`, its box `bounds`, pressed at `touch`, at
/// `progress` and `phase`, with the block's `ripple` as resolved for its
/// first frame; straight alpha, transparent where the block draws nothing
/// (its colour, alpha or radius not drawn).
///
/// `AndroidPatterned` is `RippleShader.java`'s shader (`:42-122`) with the
/// uniforms its setters give (`:138-198`) and the inputs Compose gives
/// `RippleDrawable`: the box's size rounded to whole pixels and its centre
/// the bounds' integer centre (`RippleHostView.android.kt:178`,
/// `RippleDrawable.java:895-896`); the radius `max_radius`, else half the
/// box's diagonal plus `padding`, rounded (`Ripple.android.kt:105`); the
/// colour with `alpha` replacing its own and the sparkles'
/// `DEFAULT_EFFECT_COLOR`, each packed in 8 bits as Compose's `Color(…)` and
/// `toArgb` pack it (`Color.kt:413-419`); the touch point the point pressed
/// where `bounded`, else the box's centre (`RippleHostView.android.kt:141-
/// 152`); a bounded ripple masked to its box, the outline's clip the
/// connector's. A GLSL builtin is computed as GLSL defines it, in `f32`, a
/// sine through `f64`; a ratio that is not a number is 0 (S2.2's totality).
#[must_use]
pub fn shade(
    effect: RippleEffect,
    point: Point,
    bounds: Rect,
    touch: Point,
    progress: f32,
    phase: f32,
    ripple: &ResolvedRippleStyle,
) -> Color {
    match effect {
        RippleEffect::AndroidPatterned => {
            patterned(point, bounds, touch, progress, phase, ripple).unwrap_or(CLEAR)
        }
    }
}

/// No colour.
const CLEAR: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.0,
};

/// A non-finite number as 0 (S2.2's totality).
fn finite(x: f64) -> f64 {
    if x.is_finite() { x } else { 0.0 }
}

/// `(0.4, 0, 0.2, 1)` at `x`, solved for its parameter by bisection to the
/// double's precision; the exact control points, where Android's are the
/// `float`s nearest them, a difference below the approximation's own.
fn fast_out_slow_in(x: f64) -> f64 {
    fn at(s: f64, c1: f64, c2: f64) -> f64 {
        let omt = 1.0 - s;
        3.0 * omt * omt * s * c1 + 3.0 * omt * s * s * c2 + s * s * s
    }
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let (mut lo, mut hi) = (0.0_f64, 1.0_f64);
    for _ in 0..200 {
        let mid = (lo + hi) / 2.0;
        if at(mid, 0.4, 0.2) < x {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    at((lo + hi) / 2.0, 0.0, 1.0)
}

/// A channel packed in 8 bits as Compose's `Color(…)` packs it, clamped,
/// `(x · 255 + 0.5)` truncated, and read back by `Color.valueOf(int)`.
fn packed(x: f32) -> f32 {
    let x = if x.is_finite() {
        x.clamp(0.0, 1.0)
    } else {
        0.0
    };
    (x * 255.0 + 0.5).floor() / 255.0
}

/// Kotlin's `roundToInt` of a float, as `floor(x + 0.5)`: it differs from
/// `Math.round` at 0.49999997 and at odd values from 2^23, neither a ripple's
/// size.
fn round_half_up(x: f32) -> f32 {
    (x + 0.5).floor()
}

/// The GLSL builtins the shader uses, as GLSL defines them.
fn fract(x: f32) -> f32 {
    x - x.floor()
}

fn step(edge: f32, x: f32) -> f32 {
    if x < edge { 0.0 } else { 1.0 }
}

fn saturate(x: f32) -> f32 {
    x.clamp(0.0, 1.0)
}

fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn modulo(x: f32, y: f32) -> f32 {
    x - y * (x / y).floor()
}

fn mix(x: f32, y: f32, a: f32) -> f32 {
    x * (1.0 - a) + y * a
}

fn sin(x: f32) -> f32 {
    f64::from(x).sin() as f32
}

fn distance(a: (f32, f32), b: (f32, f32)) -> f32 {
    let (dx, dy) = (a.0 - b.0, a.1 - b.1);
    (dx * dx + dy * dy).sqrt()
}

/// `triangleNoise` (`RippleShader.java:43-48`).
fn triangle_noise(n: (f32, f32)) -> f32 {
    let mut x = fract(n.0 * 5.3987);
    let mut y = fract(n.1 * 5.4421);
    let d = y * (x + 21.5351) + x * (y + 14.3137);
    x += d;
    y += d;
    let xy = x * y;
    fract(xy * 95.4307) + fract(xy * 75.04961) - 1.0
}

fn threshold(v: f32, l: f32, h: f32) -> f32 {
    step(l, v) * (1.0 - step(h, v))
}

/// `sparkles` (`:54-64`), `in_sparkleColor.a` its `sparkle_alpha`.
fn sparkles(uv: (f32, f32), t: f32, sparkle_alpha: f32) -> f32 {
    let n = triangle_noise(uv);
    let mut s = 0.0_f32;
    let mut i = 0.0_f32;
    while i < 4.0 {
        let l = i * 0.1;
        let h = l + 0.05;
        let o = sin(std::f32::consts::PI * (t + 0.35 * i));
        s += threshold(n + o, l, h);
        i += 1.0;
    }
    saturate(s) * sparkle_alpha
}

/// `softCircle` (`:65-69`).
fn soft_circle(uv: (f32, f32), xy: (f32, f32), radius: f32, blur: f32) -> f32 {
    let blur_half = blur * 0.5;
    let ratio = distance(uv, xy) / radius;
    let ratio = if ratio.is_nan() { 0.0 } else { ratio };
    1.0 - smoothstep(1.0 - blur_half, 1.0 + blur_half, ratio)
}

/// `softRing` (`:70-77`).
fn soft_ring(uv: (f32, f32), xy: (f32, f32), radius: f32, progress: f32, blur: f32) -> f32 {
    let thickness = 0.05 * radius;
    let current = radius * progress;
    let outer = soft_circle(uv, xy, current + thickness, blur);
    let inner = soft_circle(uv, xy, (current - thickness).max(0.0), blur);
    saturate(outer - inner)
}

/// `subProgress` (`:78-81`).
fn sub_progress(start: f32, end: f32, progress: f32) -> f32 {
    (progress.clamp(start, end) - start) / (end - start)
}

/// `circle_grid` (`:85-92`): `rotate2d(rotation)`, the column-major
/// `mat2(rad.x, -rad.y, rad.y, rad.x)`, times `center - coord`.
fn circle_grid(
    resolution: (f32, f32),
    coord: (f32, f32),
    center: (f32, f32),
    rotation: (f32, f32),
    cell: f32,
) -> f32 {
    let (dx, dy) = (center.0 - coord.0, center.1 - coord.1);
    let x = rotation.0 * dx + rotation.1 * dy + center.0;
    let y = -rotation.1 * dx + rotation.0 * dy + center.1;
    let coord = (
        modulo(x, cell) / resolution.0,
        modulo(y, cell) / resolution.1,
    );
    let normal_radius = cell / resolution.1 * 0.5;
    let radius = 0.65 * normal_radius;
    soft_circle(coord, (normal_radius, normal_radius), radius, radius * 50.0)
}

/// The turbulence uniforms `setNoisePhase` gives (`:157-184`), in doubles
/// then `float` as it computes them: the three circles' centres and
/// rotations.
struct Turbulence {
    circles: [(f32, f32); 3],
    rotations: [(f32, f32); 3],
}

impl Turbulence {
    fn new(phase: f32) -> Turbulence {
        let t = f64::from(phase);
        let scale = f64::from(1.5_f32);
        let right = std::f64::consts::PI * 0.0078125;
        let left = std::f64::consts::PI * -0.0078125;
        let circle = |base: f64, k: f64, angle: f64| {
            (
                (base + t * k * (scale * angle).cos()) as f32,
                (base + t * k * (scale * angle).sin()) as f32,
            )
        };
        let rotation = |r: f64| (r.cos() as f32, r.sin() as f32);
        Turbulence {
            circles: [
                circle(scale * 0.5, 0.01, 0.55),
                circle(scale * 0.2, -0.0066, 0.45),
                circle(scale, -0.0066, 0.35),
            ],
            rotations: [
                rotation(t * right + 1.7 * std::f64::consts::PI),
                rotation(t * left + 2.0 * std::f64::consts::PI),
                rotation(t * right + 2.75 * std::f64::consts::PI),
            ],
        }
    }

    /// `turbulence` (`:93-101`).
    fn at(&self, uv: (f32, f32)) -> f32 {
        let scale = (0.8_f32, 0.8_f32);
        let uv = (uv.0 * scale.0, uv.1 * scale.1);
        let [c1, c2, c3] = self.circles;
        let [r1, r2, r3] = self.rotations;
        let g1 = circle_grid(scale, uv, c1, r1, 0.17);
        let g2 = circle_grid(scale, uv, c2, r2, 0.2);
        let g3 = circle_grid(scale, uv, c3, r3, 0.275);
        let v = (g1 * g1 + g2 - g3) * 0.5;
        saturate(0.45 + 0.8 * v)
    }
}

/// The patterned ripple's `main` (`RippleShader.java:102-122`); `None`
/// where the block draws nothing.
fn patterned(
    point: Point,
    bounds: Rect,
    touch: Point,
    progress: f32,
    phase: f32,
    ripple: &ResolvedRippleStyle,
) -> Option<Color> {
    let color = ripple.color?;
    let alpha = ripple.alpha?.0;
    // The bounds Compose gives the drawable, whole pixels from the box's
    // origin, and their integer centre.
    let width = round_half_up(bounds.width.0).max(0.0);
    let height = round_half_up(bounds.height.0).max(0.0);
    let origin = ((width / 2.0).floor(), (height / 2.0).floor());
    let p = (point.x.0 - bounds.x.0, point.y.0 - bounds.y.0);
    let touch = if ripple.bounded {
        (touch.x.0 - bounds.x.0, touch.y.0 - bounds.y.0)
    } else {
        origin
    };
    let radius = match ripple.max_radius {
        Some(radius) => radius.0,
        None => {
            let (half_w, half_h) = (bounds.width.0 / 2.0, bounds.height.0 / 2.0);
            (half_w * half_w + half_h * half_h).sqrt() + ripple.padding?.0
        }
    };
    // setRadius (`:138-140`) and setResolution (`:194-198`).
    let max_radius = round_half_up(radius) * RADIUS_FACTOR;
    let resolution_scale = (1.0 / width, 1.0 / height);
    let noise_scale = (2.1 / width, 2.1 / height);
    let noise_phase = phase * 0.001;
    let turbulence = Turbulence::new(phase);
    let wave_rgb = [packed(color.r), packed(color.g), packed(color.b)];
    let wave_a = packed(alpha);
    let sparkle_a = f32::from(SPARKLE_ALPHA) / 255.0;

    let fade_in = sub_progress(0.0, 0.13, progress);
    let scale_in = sub_progress(0.0, 1.0, progress);
    let fade_out_noise = sub_progress(0.4, 0.5, progress);
    let fade_out_ripple = sub_progress(0.4, 1.0, progress);
    let toward = saturate(progress * TOWARD_ORIGIN);
    let center = (
        mix(touch.0, origin.0, toward),
        mix(touch.1, origin.1, toward),
    );
    let ring = soft_ring(p, center, max_radius, scale_in, 1.0);
    let ring_alpha = fade_in.min(1.0 - fade_out_noise);
    let uv = (p.0 * resolution_scale.0, p.1 * resolution_scale.1);
    let density_uv = (
        uv.0 - modulo(uv.0, noise_scale.0),
        uv.1 - modulo(uv.1, noise_scale.1),
    );
    let turb = turbulence.at(uv);
    let sparkle_alpha = sparkles(density_uv, noise_phase, sparkle_a) * ring * ring_alpha * turb;
    let fade = fade_in.min(1.0 - fade_out_ripple);
    let wave_alpha = soft_circle(p, center, max_radius * scale_in, 1.0) * fade * wave_a;
    let inside = p.0 >= 0.0 && p.1 >= 0.0 && p.0 < width && p.1 < height;
    let mask = if !ripple.bounded || inside { 1.0 } else { 0.0 };
    // `mix(waveColor, sparkleColor, sparkleAlpha) * mask`, premultiplied;
    // the sparkles white.
    let out = |wave: f32, sparkle: f32| {
        let value = mix(wave, sparkle, sparkle_alpha) * mask;
        if value.is_finite() { value } else { 0.0 }
    };
    let [r, g, b] = wave_rgb.map(|channel| out(channel * wave_alpha, sparkle_a));
    let a = out(wave_alpha, sparkle_a);
    (a > 0.0).then(|| Color {
        r: r / a,
        g: g / a,
        b: b / a,
        a,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::next::units::{Fraction, Px};

    const E: RippleEffect = RippleEffect::AndroidPatterned;

    /// `tools/motion-vectors`' rows: `(since_start, since_release)` and
    /// the progress, `None` ended.
    #[test]
    fn ripple_progress_follows_the_session() {
        let rows: [(f32, Option<f32>, Option<f32>); 14] = [
            (0.0, None, Some(0.0)),
            (100.0, None, Some(0.087_584_145)),
            (225.0, None, Some(0.387_780_67)),
            (449.0, None, Some(0.499_998_72)),
            (450.0, None, Some(0.5)),
            (2000.0, None, Some(0.5)),
            (300.0, Some(100.0), Some(0.460_387_9)),
            (500.0, Some(100.0), Some(0.566_666_66)),
            (600.0, Some(0.0), Some(0.5)),
            (700.0, Some(50.0), Some(0.566_666_66)),
            (1000.0, Some(300.0), Some(0.9)),
            (824.0, Some(0.0), Some(0.5)),
            (825.0, Some(0.0), Some(0.5)),
            (1300.0, Some(500.0), None),
        ];
        for (since_start, since_release, expected) in rows {
            let found = progress(E, Ms(since_start), since_release.map(Ms));
            assert_eq!(found, expected, "{since_start} {since_release:?}");
        }
    }

    #[test]
    fn ripple_phase_advances_once() {
        let rows: [(f32, f32, f32); 5] = [
            (120_000.0, 0.0, 120_000.0),
            (120_000.0, 3500.0, 120_016.0),
            (120_000.0, 7000.0, 120_032.0),
            (120_000.0, 9000.0, 120_032.0),
            (5.0, 700.0, 8.2),
        ];
        for (start, since, expected) in rows {
            assert_eq!(phase(E, Ms(start), Ms(since)), expected, "{start} {since}");
        }
    }

    /// The rows over a 120 × 48 box touched at (30, 20), bounded, the colour
    /// (0.2, 0.4, 0.6) at alpha 0.1, the radius half the diagonal (no
    /// padding), the phase 1000: `(progress, x, y)` and the premultiplied
    /// colour the shader gives, to 1e-3 on the 0–255 scale (S2.2's generated
    /// rows). Synthetic inputs.
    #[test]
    fn ripple_shade_follows_the_shader() {
        let style = ResolvedRippleStyle {
            effect: Some(E),
            color: Some(Color {
                r: 0.2,
                g: 0.4,
                b: 0.6,
                a: 1.0,
            }),
            alpha: Some(Fraction(0.1)),
            bounded: true,
            max_radius: None,
            padding: Some(Px(0.0)),
        };
        let bounds = Rect {
            x: Px(0.0),
            y: Px(0.0),
            width: Px(120.0),
            height: Px(48.0),
        };
        let touch = Point {
            x: Px(30.0),
            y: Px(20.0),
        };
        let rows: [(f32, f32, f32, [f32; 4]); 9] = [
            (
                0.05,
                30.0,
                20.0,
                [0.007_843_138, 0.015_686_275, 0.023_529_414, 0.039_215_688],
            ),
            (
                0.1,
                40.0,
                24.0,
                [0.015_686_275, 0.031_372_55, 0.047_058_83, 0.078_431_375],
            ),
            (
                0.25,
                60.0,
                24.0,
                [0.020_392_157, 0.040_784_314, 0.061_176_475, 0.101_960_786],
            ),
            (
                0.3,
                10.0,
                5.0,
                [0.101_105_2, 0.111_356_8, 0.121_608_41, 0.142_111_63],
            ),
            (
                0.45,
                80.0,
                30.0,
                [0.018_692_812, 0.037_385_624, 0.056_078_434, 0.093_464_054],
            ),
            (
                0.6,
                100.0,
                40.0,
                [0.013_594_771, 0.027_189_542, 0.040_784_314, 0.067_973_85],
            ),
            (0.6, 130.0, 40.0, [0.0, 0.0, 0.0, 0.0]),
            (
                0.9,
                60.0,
                24.0,
                [0.003_398_694_6, 0.006_797_389, 0.010_196_084, 0.016_993_472],
            ),
            (0.0, 30.0, 20.0, [0.0, 0.0, 0.0, 0.0]),
        ];
        for (at, x, y, expected) in rows {
            let point = Point { x: Px(x), y: Px(y) };
            let c = shade(E, point, bounds, touch, at, 1000.0, &style);
            let found = [c.r * c.a, c.g * c.a, c.b * c.a, c.a];
            for (f, e) in found.iter().zip(expected) {
                assert!(
                    (f - e).abs() * 255.0 <= 1e-3,
                    "{at} {x} {y}: {found:?} {expected:?}"
                );
            }
        }
    }
}
