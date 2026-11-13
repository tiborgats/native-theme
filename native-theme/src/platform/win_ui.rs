// SPDX-License-Identifier: MIT
//
// This file ports TextFormatting::GetScaledFontSize of microsoft-ui-xaml's
// src/dxaml/xcp/core/text/common/TextFormatting.cpp (microsoft-ui-xaml at
// winui3/release/2.5.1), whose notice follows; it stays under that licence
// whatever licence the rest of the crate is used under (the 0.8.0 plan's
// D10).
//
// MIT License
//
// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! S9.4's `WinUi` curve: WinUI's text-size curve, which scales small sizes
//! more than large ones (ADD-A141).
//!
//! `TextFormatting::GetScaledFontSize` (`microsoft-ui-xaml@2.5.1
//! src/dxaml/xcp/core/text/common/TextFormatting.cpp:181-204`) leaves a size
//! at or below zero as it is, takes a size below 1 px as 1 px, and gives
//! `m + max(18 − e·ln(m), 0) × (f − 1)` for `m` that size and `f` the text
//! scale factor: the code's `exp(1)` and 18, not its comment's
//! `-2.5 ln s_i + 15`. Each step is the source's: the size, the factor and
//! `factor − 1` in single precision, as `XFLOAT`; the logarithm in single
//! precision, as the `float` overload of `log` that MSVC's global overloads
//! give a `float` argument; `exp(1)`, the weight and the product in double
//! precision; the sum rounded to single precision once, by the source's
//! cast. The logarithm is taken in double precision and rounded to single
//! once, so it does not depend on the `logf` of the C library the port runs
//! over; it gives the reference build's result on every row of the vector
//! table (`tools/winui-vectors`).

use std::f64::consts::E;

/// The curve's weight at 1 px, where the logarithm is zero
/// (`TextFormatting.cpp:203`'s `18`).
const WEIGHT_AT_ONE_PX: f64 = 18.0;

/// The smallest size the curve reads (`TextFormatting.cpp:195`'s `1.0f`): a
/// smaller size is scaled from it.
const SMALLEST: f32 = 1.0;

/// `size` in logical px at the text scale factor `factor`, in logical px:
/// `GetScaledFontSize`, the text scale factor enabled.
#[must_use]
pub fn win_ui(size: f32, factor: f32) -> f32 {
    if size <= 0.0 {
        return size;
    }
    // `std::max(m_eFontSize, 1.0f)`: the first unless it is less.
    let input = if size < SMALLEST { SMALLEST } else { size };
    let log = f64::from(input).ln() as f32;
    let weight = -E * f64::from(log) + WEIGHT_AT_ONE_PX;
    // `std::max(weight, 0.0)`: the first unless it is less.
    let weight = if weight < 0.0 { 0.0 } else { weight };
    (f64::from(input) + weight * f64::from(factor - 1.0)) as f32
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]

    use super::win_ui;

    /// At the factor 1 a size holds; the run's two sizes at 150 % grow as
    /// the specification's 15.87 and 71.27 px give (S9.4).
    #[test]
    fn the_curve_scales_small_sizes_more() {
        assert_eq!(win_ui(14.0, 1.0), 14.0);
        assert!((win_ui(10.0, 1.5) - 15.87).abs() < 0.005);
        assert!((win_ui(68.0, 1.5) - 71.27).abs() < 0.005);
    }

    /// A size at or below zero holds; below 1 px the curve reads 1 px; past
    /// `e^(18/e)` px the weight is zero and the size holds.
    #[test]
    fn the_ends() {
        assert_eq!(win_ui(0.0, 2.0), 0.0);
        assert_eq!(win_ui(-3.0, 2.0), -3.0);
        assert_eq!(win_ui(0.5, 1.5), win_ui(1.0, 1.5));
        assert_eq!(win_ui(1.0, 1.5), 10.0);
        assert_eq!(win_ui(800.0, 2.0), 800.0);
    }
}
