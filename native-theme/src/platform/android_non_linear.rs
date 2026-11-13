// SPDX-License-Identifier: Apache-2.0
//
// This file ports the Android Open Source Project's
// core/java/android/content/res/FontScaleConverterFactory.java and
// FontScaleConverterImpl.java and core/java/android/util/MathUtils.java's
// lerp, lerpInv, saturate and constrainedMap (frameworks/base at
// android-17.0.0_r1), and the threshold of androidx's compose/ui/ui-text
// SpannableExtensions.android.kt (androidx at e4d933c9), whose notices follow; it stays under that licence whatever
// licence the rest of the crate is used under (the 0.8.0 plan's D10). The
// crate's LICENSE-APACHE holds the licence, its NOTICE the notice of
// frameworks/base's NOTICE file.
//
// Copyright (C) 2009 The Android Open Source Project
// Copyright (C) 2022 The Android Open Source Project
// Copyright 2020 The Android Open Source Project
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

//! S9.4's `AndroidNonLinear` curve: Android 14's non-linear font scaling,
//! which scales large sizes less (MAT-X19).
//!
//! `FontScaleConverterFactory.forScale` picks the converter of a font scale:
//! none below the first table's key as a scale less 0.01 (its key of 1.05
//! is 104, so 1.03), and a size scales by the factor; the table of the
//! scale's key (the scale times 100, truncated);
//! between two tables' keys one interpolated between them; beyond the last
//! one mapping 1 sp to the factor. `FontScaleConverterImpl` maps a size
//! through its table, interpolating linearly between rows (from zero below
//! the first) and by the last row's ratio beyond it, the size's sign kept.
//! Each step is the source's, in single precision as Java's `float`; no
//! cache, so a scale gives what a process asking for it first gets.
//!
//! [`android_non_linear`] converts sp to dp as `TypedValue.applyDimension`
//! does (`core/java/android/util/TypedValue.java:430-445`): through the
//! converter, else times the factor; [`android_non_linear_inverse`] dp back
//! to sp as its `deriveDimension` does (`:471-493`). The model's logical
//! pixel is the dp, and a size stated in it at a factor of one is the sp.

/// The tables, by the scale whose key holds each (`FontScaleConverterFactory`'s
/// static block): each maps [`COMMON_SP`] to these dp.
const TABLES: [(f32, [f32; 9]); 8] = [
    (1.05, [8.4, 10.5, 12.6, 14.8, 18.6, 20.6, 24.4, 30.0, 100.0]),
    (1.1, [8.8, 11.0, 13.2, 15.6, 19.2, 21.2, 24.8, 30.0, 100.0]),
    (1.15, [9.2, 11.5, 13.8, 16.4, 19.8, 21.8, 25.2, 30.0, 100.0]),
    (1.2, [9.6, 12.0, 14.4, 17.2, 20.4, 22.4, 25.6, 30.0, 100.0]),
    (1.3, [10.4, 13.0, 15.6, 18.8, 21.6, 23.6, 26.4, 30.0, 100.0]),
    (1.5, [12.0, 15.0, 18.0, 22.0, 24.0, 26.0, 28.0, 30.0, 100.0]),
    (1.8, [14.4, 18.0, 21.6, 24.4, 27.6, 30.8, 32.8, 34.8, 100.0]),
    (2.0, [16.0, 20.0, 24.0, 26.0, 30.0, 34.0, 36.0, 38.0, 100.0]),
];

/// The sp every table maps (`createInterpolatedTableBetween`'s
/// `commonSpSizes`, each table's `fromSp`).
const COMMON_SP: [f32; 9] = [8.0, 10.0, 12.0, 14.0, 18.0, 20.0, 24.0, 30.0, 100.0];

/// The font scale above which Compose keeps a line height stated in sp in
/// proportion to its font's change, rather than converting it as a value of
/// its own (`compose/ui/ui-text` `SpannableExtensions.android.kt:224-250`'s
/// `isNonLinearFontScalingActive`, a `Float` compared with a `Double`).
pub(crate) const PROPORTIONAL_LINE_HEIGHT_ABOVE: f64 = 1.05;

/// `size` in sp, at the font scale `font_scale`, in dp: the converter's
/// `convertSpToDp`, or `size × font_scale` where the scale has no converter.
#[must_use]
pub fn android_non_linear(size: f32, font_scale: f32) -> f32 {
    match for_scale(font_scale) {
        Some(converter) => converter.sp_to_dp(size),
        None => size * font_scale,
    }
}

/// `size` in dp, at the font scale `font_scale`, back in sp: the converter's
/// `convertDpToSp`, or `size / font_scale` where the scale has no converter.
#[must_use]
pub fn android_non_linear_inverse(size: f32, font_scale: f32) -> f32 {
    match for_scale(font_scale) {
        Some(converter) => converter.dp_to_sp(size),
        None => size / font_scale,
    }
}

/// A `FontScaleConverterImpl`: a table of sp and the dp they map to.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Converter {
    /// [`COMMON_SP`] to these dp.
    Table([f32; 9]),
    /// 1 sp to this many dp, the converter `forScale` gives beyond its
    /// tables.
    Linear(f32),
}

impl Converter {
    /// `convertSpToDp`.
    fn sp_to_dp(self, sp: f32) -> f32 {
        match self {
            Converter::Table(dp) => lookup_and_interpolate(sp, &COMMON_SP, &dp),
            Converter::Linear(dp) => lookup_and_interpolate(sp, &[1.0], &[dp]),
        }
    }

    /// `convertDpToSp`.
    fn dp_to_sp(self, dp: f32) -> f32 {
        match self {
            Converter::Table(to) => lookup_and_interpolate(dp, &to, &COMMON_SP),
            Converter::Linear(to) => lookup_and_interpolate(dp, &[to], &[1.0]),
        }
    }
}

/// `FontScaleConverterImpl.lookupAndInterpolate`.
fn lookup_and_interpolate(source_value: f32, source_values: &[f32], target_values: &[f32]) -> f32 {
    let source_value_positive = source_value.abs();
    let sign = signum(source_value);
    let index = source_values.binary_search_by(|probe| probe.total_cmp(&source_value_positive));
    match index {
        Ok(index) => sign * target_values.get(index).copied().unwrap_or(f32::NAN),
        Err(insertion) => {
            if insertion > 0 && insertion == source_values.len() {
                let (Some(&start_sp), Some(&start_dp)) =
                    (source_values.last(), target_values.last())
                else {
                    return f32::NAN;
                };
                if start_sp == 0.0 {
                    return 0.0;
                }
                let scaling_factor = start_dp / start_sp;
                return source_value * scaling_factor;
            }
            let (start_sp, end_sp, start_dp, end_dp) = match insertion.checked_sub(1) {
                None => (
                    0.0,
                    source_values.first().copied().unwrap_or(f32::NAN),
                    0.0,
                    target_values.first().copied().unwrap_or(f32::NAN),
                ),
                Some(lower) => (
                    source_values.get(lower).copied().unwrap_or(f32::NAN),
                    source_values.get(insertion).copied().unwrap_or(f32::NAN),
                    target_values.get(lower).copied().unwrap_or(f32::NAN),
                    target_values.get(insertion).copied().unwrap_or(f32::NAN),
                ),
            };
            sign * constrained_map(start_dp, end_dp, start_sp, end_sp, source_value_positive)
        }
    }
}

/// `FontScaleConverterFactory.forScale`, uncached: `None` below the first
/// table's scale less 0.01.
fn for_scale(font_scale: f32) -> Option<Converter> {
    let keys = TABLES.map(|(scale, _)| key(scale));
    let first = keys.first().copied().map(scale_from_key)?;
    let active = font_scale >= first - 0.01;
    if !active {
        return None;
    }
    match keys.binary_search(&key(font_scale)) {
        Ok(index) => TABLES.get(index).map(|(_, dp)| Converter::Table(*dp)),
        Err(insertion) => {
            let (Some(lower), Some(&end_key)) = (insertion.checked_sub(1), keys.get(insertion))
            else {
                return Some(Converter::Linear(font_scale));
            };
            let (Some(&start_key), Some((_, start)), Some((_, end))) =
                (keys.get(lower), TABLES.get(lower), TABLES.get(insertion))
            else {
                return Some(Converter::Linear(font_scale));
            };
            let interpolation_point = constrained_map(
                0.0,
                1.0,
                scale_from_key(start_key),
                scale_from_key(end_key),
                font_scale,
            );
            let (start, end) = (Converter::Table(*start), Converter::Table(*end));
            Some(Converter::Table(COMMON_SP.map(|sp| {
                lerp(start.sp_to_dp(sp), end.sp_to_dp(sp), interpolation_point)
            })))
        }
    }
}

/// `FontScaleConverterFactory.getKey`: Java's `(int)` of a `float`, which
/// truncates and saturates as `as` does.
#[allow(
    clippy::cast_possible_truncation,
    reason = "the source's truncation to its integer key"
)]
fn key(font_scale: f32) -> i32 {
    (font_scale * 100.0) as i32
}

/// `FontScaleConverterFactory.getScaleFromKey`.
#[allow(
    clippy::cast_precision_loss,
    reason = "the source's `(float) key`; a key is at most a few hundred"
)]
fn scale_from_key(key: i32) -> f32 {
    key as f32 / 100.0
}

/// `MathUtils.lerp`.
fn lerp(start: f32, stop: f32, amount: f32) -> f32 {
    start + (stop - start) * amount
}

/// `MathUtils.lerpInv`.
fn lerp_inv(a: f32, b: f32, value: f32) -> f32 {
    if a != b { (value - a) / (b - a) } else { 0.0 }
}

/// `MathUtils.saturate`, `constrain(value, 0, 1)`, which passes NaN
/// through as `clamp` does.
fn saturate(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

/// `MathUtils.constrainedMap`.
fn constrained_map(
    range_min: f32,
    range_max: f32,
    value_min: f32,
    value_max: f32,
    value: f32,
) -> f32 {
    lerp(
        range_min,
        range_max,
        saturate(lerp_inv(value_min, value_max, value)),
    )
}

/// Java's `Math.signum`: -1 or 1, a zero or NaN itself.
fn signum(value: f32) -> f32 {
    if value > 0.0 {
        1.0
    } else if value < 0.0 {
        -1.0
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]

    use super::{TABLES, android_non_linear, android_non_linear_inverse, key};

    /// A table's scale gives its own row: 1.3's maps 14 sp to 18.8 dp
    /// (`FontScaleConverterFactory.java:101-109`).
    #[test]
    fn a_table_gives_its_rows() {
        assert_eq!(android_non_linear(14.0, 1.3), 18.8);
        assert_eq!(android_non_linear(100.0, 2.0), 100.0);
    }

    /// The keys are the scales times 100 truncated in `float`: 1.05's is
    /// 104, as the source computes it, and the keys increase.
    #[test]
    fn keys_truncate() {
        assert_eq!(key(1.05), 104);
        assert!(TABLES.windows(2).all(|pair| match pair {
            [(a, _), (b, _)] => key(*a) < key(*b),
            _ => false,
        }));
    }

    /// Below the first table's key less 0.01 a size scales by the factor;
    /// beyond the last table too, through the converter of 1 sp.
    #[test]
    fn outside_the_tables_the_factor() {
        assert_eq!(android_non_linear(30.0, 1.0), 30.0);
        assert_eq!(android_non_linear(30.0, 0.85), 30.0 * 0.85);
        assert_eq!(android_non_linear(30.0, 2.5), 30.0 * 2.5);
    }

    /// Past a table's last row the last row's ratio holds; below its first
    /// it interpolates from zero; the sign is kept.
    #[test]
    fn past_and_below_the_rows() {
        assert_eq!(android_non_linear(200.0, 2.0), 200.0);
        assert_eq!(android_non_linear(4.0, 2.0), 8.0);
        assert_eq!(android_non_linear(-14.0, 1.3), -18.8);
    }

    /// The inverse maps a table's dp back to its sp.
    #[test]
    fn the_inverse_maps_back() {
        assert_eq!(android_non_linear_inverse(18.8, 1.3), 14.0);
        assert_eq!(android_non_linear_inverse(15.0, 0.5), 30.0);
    }
}
