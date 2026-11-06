// SPDX-License-Identifier: Apache-2.0
//
// This file ports material-color-utilities' Java code at commit bd811e4c
// (github.com/material-foundation/material-color-utilities):
// dynamiccolor/ColorSpec2021.java, DynamicColor.java, DynamicScheme.java,
// ContrastCurve.java, ToneDeltaPair.java and TonePolarity.java;
// scheme/SchemeTonalSpot.java; palettes/TonalPalette.java; hct/Hct.java,
// HctSolver.java, Cam16.java and ViewingConditions.java; contrast/Contrast.java;
// utils/ColorUtils.java and MathUtils.java. Its notices follow; it stays under
// that licence whatever licence the rest of the crate is used under (the 0.8.0
// plan's D10). The crate's LICENSE-APACHE holds the licence, its NOTICE the
// port's entry.
//
// Copyright 2021 Google LLC
// Copyright 2022 Google LLC
// Copyright 2023 Google LLC
// Copyright 2025 Google LLC
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

//! `m3_role` (S2.2): a role of material-color-utilities' dynamic scheme from
//! a seed, as `MaterialDynamicColors` gives it.
//!
//! The port holds what the scheme `token.material` states needs: the
//! `TONAL_SPOT` variant (`SchemeTonalSpot`) under the 2021 specification
//! (`ColorSpec2021`), on the phone platform (`DynamicScheme`'s default), and
//! the roles of `md.sys.color`. Each step is the source's, in its order, in
//! double precision. A branch the source takes only for another variant
//! (`isMonochrome`, `isFidelity`) or another platform is left out, and so are
//! the palettes' key colours, the HCT colour's chroma and tone and the
//! opacity, which none of these roles reads.

use std::sync::LazyLock;

use crate::selector::Scheme;
use crate::tokens::MaterialVariant;

/// The specification version the port implements, as `spec_versions`
/// declares it (`SpecVersion.SPEC_2021`).
const SPEC_2021: &str = "spec_2021";

/// The dynamic scheme's role `role` (a `md.sys.color` role, `primary`,
/// `on_surface_variant`) from the seed `accent` (straight-alpha
/// `[r, g, b, a]`), under `variant`, `spec_version` and `contrast_level`
/// (`-1..=1`, 0 the standard level), in `scheme`: an opaque colour, each
/// channel a byte over 255, as `DynamicColor.getArgb` gives it.
///
/// The seed enters as the source's ARGB integer: each channel clamped to
/// `0..=1` and rounded to a byte, as
/// [`Color::to_srgb8_clamped`](crate::Color::to_srgb8_clamped) rounds it,
/// its alpha unread, as `Hct.fromInt` reads none.
///
/// `None` for a role the scheme does not define, a variant other than
/// `TonalSpot` and a version other than `spec_2021`, which the port does not
/// implement.
#[must_use]
pub fn m3_role(
    role: &str,
    accent: [f32; 4],
    variant: MaterialVariant,
    spec_version: &str,
    contrast_level: f32,
    scheme: Scheme,
) -> Option<[f32; 4]> {
    let role = Role::from_name(role)?;
    if variant != MaterialVariant::TonalSpot || spec_version != SPEC_2021 {
        return None;
    }
    let [r, g, b, _] = accent.map(|c| (c.clamp(0.0, 1.0) * 255.0).round() as u8);
    let source_hue = cam16_hue(Rgb { r, g, b });
    let scheme = DynamicScheme::tonal_spot(
        source_hue,
        scheme == Scheme::Dark,
        f64::from(contrast_level),
    );
    let Rgb { r, g, b } = get_hct(&scheme, role);
    Some([
        f32::from(r) / 255.0,
        f32::from(g) / 255.0,
        f32::from(b) / 255.0,
        1.0,
    ])
}

/// An opaque sRGB colour, the source's ARGB integer with alpha 255.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

// ---------------------------------------------------------------------------
// utils/MathUtils.java

/// `MathUtils.lerp`.
fn lerp(start: f64, stop: f64, amount: f64) -> f64 {
    (1.0 - amount) * start + amount * stop
}

/// `MathUtils.clampDouble`.
fn clamp_double(min: f64, max: f64, input: f64) -> f64 {
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

/// `MathUtils.sanitizeDegreesDouble`.
#[allow(
    clippy::modulo_arithmetic,
    reason = "a float remainder, which cannot panic"
)]
fn sanitize_degrees_double(degrees: f64) -> f64 {
    let degrees = degrees % 360.0;
    if degrees < 0.0 {
        degrees + 360.0
    } else {
        degrees
    }
}

/// `MathUtils.signum`, and Java's `Math.signum` on a number that is not
/// NaN: the sign as -1, 0 or 1.
fn signum(num: f64) -> f64 {
    if num < 0.0 {
        -1.0
    } else if num == 0.0 {
        0.0
    } else {
        1.0
    }
}

/// `MathUtils.matrixMultiply`.
fn matrix_multiply(row: [f64; 3], matrix: [[f64; 3]; 3]) -> [f64; 3] {
    let [r0, r1, r2] = row;
    let [[m00, m01, m02], [m10, m11, m12], [m20, m21, m22]] = matrix;
    let a = r0 * m00 + r1 * m01 + r2 * m02;
    let b = r0 * m10 + r1 * m11 + r2 * m12;
    let c = r0 * m20 + r1 * m21 + r2 * m22;
    [a, b, c]
}

/// Java's `Math.round` on a double, as a double: the nearest integer, a half
/// toward positive infinity (`(long) Math.floor(x + 0.5)` without its
/// rounding of `x + 0.5`).
fn java_round(x: f64) -> f64 {
    let floor = x.floor();
    if x - floor >= 0.5 { floor + 1.0 } else { floor }
}

// ---------------------------------------------------------------------------
// utils/ColorUtils.java

/// `ColorUtils.WHITE_POINT_D65`.
const WHITE_POINT_D65: [f64; 3] = [95.047, 100.0, 108.883];

/// `ColorUtils.argbFromLinrgb`.
fn argb_from_linrgb(linrgb: [f64; 3]) -> Rgb {
    let [r, g, b] = linrgb.map(delinearized);
    Rgb { r, g, b }
}

/// `ColorUtils.argbFromLstar`.
fn argb_from_lstar(lstar: f64) -> Rgb {
    let y = y_from_lstar(lstar);
    let component = delinearized(y);
    Rgb {
        r: component,
        g: component,
        b: component,
    }
}

/// `ColorUtils.yFromLstar`.
fn y_from_lstar(lstar: f64) -> f64 {
    100.0 * lab_invf((lstar + 16.0) / 116.0)
}

/// `ColorUtils.lstarFromY`.
fn lstar_from_y(y: f64) -> f64 {
    lab_f(y / 100.0) * 116.0 - 16.0
}

/// `ColorUtils.linearized`.
fn linearized(rgb_component: u8) -> f64 {
    let normalized = f64::from(rgb_component) / 255.0;
    if normalized <= 0.040449936 {
        normalized / 12.92 * 100.0
    } else {
        ((normalized + 0.055) / 1.055).powf(2.4) * 100.0
    }
}

/// `ColorUtils.delinearized`: `(int) Math.round(…)` clamped to `0..=255`.
fn delinearized(rgb_component: f64) -> u8 {
    let normalized = rgb_component / 100.0;
    let delinearized = if normalized <= 0.0031308 {
        normalized * 12.92
    } else {
        1.055 * normalized.powf(1.0 / 2.4) - 0.055
    };
    java_round(delinearized * 255.0).clamp(0.0, 255.0) as u8
}

/// `ColorUtils.labF`.
fn lab_f(t: f64) -> f64 {
    let e = 216.0 / 24389.0;
    let kappa = 24389.0 / 27.0;
    if t > e {
        t.powf(1.0 / 3.0)
    } else {
        (kappa * t + 16.0) / 116.0
    }
}

/// `ColorUtils.labInvf`.
fn lab_invf(ft: f64) -> f64 {
    let e = 216.0 / 24389.0;
    let kappa = 24389.0 / 27.0;
    let ft3 = ft * ft * ft;
    if ft3 > e {
        ft3
    } else {
        (116.0 * ft - 16.0) / kappa
    }
}

// ---------------------------------------------------------------------------
// hct/ViewingConditions.java

/// The viewing conditions' values the port reads.
struct ViewingConditions {
    aw: f64,
    nbb: f64,
    ncb: f64,
    c: f64,
    nc: f64,
    n: f64,
    rgb_d: [f64; 3],
    fl: f64,
    z: f64,
}

/// `ViewingConditions.DEFAULT`: `defaultWithBackgroundLstar(50.0)`.
static DEFAULT_VIEWING_CONDITIONS: LazyLock<ViewingConditions> = LazyLock::new(|| {
    ViewingConditions::make(
        WHITE_POINT_D65,
        200.0 / std::f64::consts::PI * y_from_lstar(50.0) / 100.0,
        50.0,
        2.0,
        false,
    )
});

impl ViewingConditions {
    /// `ViewingConditions.make`.
    fn make(
        white_point: [f64; 3],
        adapting_luminance: f64,
        background_lstar: f64,
        surround: f64,
        discounting_illuminant: bool,
    ) -> ViewingConditions {
        let background_lstar = background_lstar.max(0.1);
        let [x0, x1, x2] = white_point;
        let [[m00, m01, m02], [m10, m11, m12], [m20, m21, m22]] = XYZ_TO_CAM16RGB;
        let r_w = (x0 * m00) + (x1 * m01) + (x2 * m02);
        let g_w = (x0 * m10) + (x1 * m11) + (x2 * m12);
        let b_w = (x0 * m20) + (x1 * m21) + (x2 * m22);
        let f = 0.8 + (surround / 10.0);
        let c = if f >= 0.9 {
            lerp(0.59, 0.69, (f - 0.9) * 10.0)
        } else {
            lerp(0.525, 0.59, (f - 0.8) * 10.0)
        };
        let d = if discounting_illuminant {
            1.0
        } else {
            f * (1.0 - ((1.0 / 3.6) * ((-adapting_luminance - 42.0) / 92.0).exp()))
        };
        let d = clamp_double(0.0, 1.0, d);
        let nc = f;
        let rgb_d = [
            d * (100.0 / r_w) + 1.0 - d,
            d * (100.0 / g_w) + 1.0 - d,
            d * (100.0 / b_w) + 1.0 - d,
        ];
        let k = 1.0 / (5.0 * adapting_luminance + 1.0);
        let k4 = k * k * k * k;
        let k4f = 1.0 - k4;
        let fl = (k4 * adapting_luminance) + (0.1 * k4f * k4f * (5.0 * adapting_luminance).cbrt());
        let n = y_from_lstar(background_lstar) / x1;
        let z = 1.48 + n.sqrt();
        let nbb = 0.725 / n.powf(0.2);
        let ncb = nbb;
        let [d0, d1, d2] = rgb_d;
        let rgb_a_factors = [
            (fl * d0 * r_w / 100.0).powf(0.42),
            (fl * d1 * g_w / 100.0).powf(0.42),
            (fl * d2 * b_w / 100.0).powf(0.42),
        ];
        let [a0, a1, a2] = rgb_a_factors.map(|factor| (400.0 * factor) / (factor + 27.13));
        let aw = ((2.0 * a0) + a1 + (0.05 * a2)) * nbb;
        ViewingConditions {
            aw,
            nbb,
            ncb,
            c,
            nc,
            n,
            rgb_d,
            fl,
            z,
        }
    }
}

// ---------------------------------------------------------------------------
// hct/Cam16.java

/// `Cam16.XYZ_TO_CAM16RGB`.
const XYZ_TO_CAM16RGB: [[f64; 3]; 3] = [
    [0.401288, 0.650173, -0.051461],
    [-0.250268, 1.204414, 0.045854],
    [-0.002079, 0.048952, 0.953127],
];

/// `Cam16.fromInt(argb).getHue()`: `fromIntInViewingConditions` and
/// `fromXyzInViewingConditions` in the default viewing conditions, as far
/// as the hue.
fn cam16_hue(argb: Rgb) -> f64 {
    let viewing_conditions = &*DEFAULT_VIEWING_CONDITIONS;
    let red_l = linearized(argb.r);
    let green_l = linearized(argb.g);
    let blue_l = linearized(argb.b);
    let x = 0.41233895 * red_l + 0.35762064 * green_l + 0.18051042 * blue_l;
    let y = 0.2126 * red_l + 0.7152 * green_l + 0.0722 * blue_l;
    let z = 0.01932141 * red_l + 0.11916382 * green_l + 0.95034478 * blue_l;

    let [[m00, m01, m02], [m10, m11, m12], [m20, m21, m22]] = XYZ_TO_CAM16RGB;
    let r_t = (x * m00) + (y * m01) + (z * m02);
    let g_t = (x * m10) + (y * m11) + (z * m12);
    let b_t = (x * m20) + (y * m21) + (z * m22);

    let [d0, d1, d2] = viewing_conditions.rgb_d;
    let r_d = d0 * r_t;
    let g_d = d1 * g_t;
    let b_d = d2 * b_t;

    let fl = viewing_conditions.fl;
    let r_af = (fl * r_d.abs() / 100.0).powf(0.42);
    let g_af = (fl * g_d.abs() / 100.0).powf(0.42);
    let b_af = (fl * b_d.abs() / 100.0).powf(0.42);
    let r_a = signum(r_d) * 400.0 * r_af / (r_af + 27.13);
    let g_a = signum(g_d) * 400.0 * g_af / (g_af + 27.13);
    let b_a = signum(b_d) * 400.0 * b_af / (b_af + 27.13);

    let a = (11.0 * r_a + -12.0 * g_a + b_a) / 11.0;
    let b = (r_a + g_a - 2.0 * b_a) / 9.0;

    let atan2 = b.atan2(a);
    let atan_degrees = atan2.to_degrees();
    sanitize_degrees_double(atan_degrees)
}

// ---------------------------------------------------------------------------
// hct/HctSolver.java

/// `HctSolver.SCALED_DISCOUNT_FROM_LINRGB`.
const SCALED_DISCOUNT_FROM_LINRGB: [[f64; 3]; 3] = [
    [
        0.001200833568784504,
        0.002389694492170889,
        0.0002795742885861124,
    ],
    [
        0.0005891086651375999,
        0.0029785502573438758,
        0.0003270666104008398,
    ],
    [
        0.00010146692491640572,
        0.0005364214359186694,
        0.0032979401770712076,
    ],
];

/// `HctSolver.LINRGB_FROM_SCALED_DISCOUNT`.
const LINRGB_FROM_SCALED_DISCOUNT: [[f64; 3]; 3] = [
    [1373.2198709594231, -1100.4251190754821, -7.278681089101213],
    [-271.815969077903, 559.6580465940733, -32.46047482791194],
    [1.9622899599665666, -57.173814538844006, 308.7233197812385],
];

/// `HctSolver.Y_FROM_LINRGB`.
const Y_FROM_LINRGB: [f64; 3] = [0.2126, 0.7152, 0.0722];

/// `HctSolver.CRITICAL_PLANES`.
const CRITICAL_PLANES: [f64; 255] = [
    0.015176349177441876,
    0.045529047532325624,
    0.07588174588720938,
    0.10623444424209313,
    0.13658714259697685,
    0.16693984095186062,
    0.19729253930674434,
    0.2276452376616281,
    0.2579979360165119,
    0.28835063437139563,
    0.3188300904430532,
    0.350925934958123,
    0.3848314933096426,
    0.42057480301049466,
    0.458183274052838,
    0.4976837250274023,
    0.5391024159806381,
    0.5824650784040898,
    0.6277969426914107,
    0.6751227633498623,
    0.7244668422128921,
    0.775853049866786,
    0.829304845476233,
    0.8848452951698498,
    0.942497089126609,
    1.0022825574869039,
    1.0642236851973577,
    1.1283421258858297,
    1.1946592148522128,
    1.2631959812511864,
    1.3339731595349034,
    1.407011200216447,
    1.4823302800086415,
    1.5599503113873272,
    1.6398909516233677,
    1.7221716113234105,
    1.8068114625156377,
    1.8938294463134073,
    1.9832442801866852,
    2.075074464868551,
    2.1693382909216234,
    2.2660538449872063,
    2.36523901573795,
    2.4669114995532007,
    2.5710888059345764,
    2.6777882626779785,
    2.7870270208169257,
    2.898822059350997,
    3.0131901897720907,
    3.1301480604002863,
    3.2497121605402226,
    3.3718988244681087,
    3.4967242352587946,
    3.624204428461639,
    3.754355295633311,
    3.887192587735158,
    4.022731918402185,
    4.160988767090289,
    4.301978482107941,
    4.445716283538092,
    4.592217266055746,
    4.741496401646282,
    4.893568542229298,
    5.048448422192488,
    5.20615066083972,
    5.3666897647573375,
    5.5300801301023865,
    5.696336044816294,
    5.865471690767354,
    6.037501145825082,
    6.212438385869475,
    6.390297286737924,
    6.571091626112461,
    6.7548350853498045,
    6.941541251256611,
    7.131223617812143,
    7.323895587840543,
    7.5195704746346665,
    7.7182615035334345,
    7.919981813454504,
    8.124744458384042,
    8.332562408825165,
    8.543448553206703,
    8.757415699253682,
    8.974476575321063,
    9.194643831691977,
    9.417930041841839,
    9.644347703669503,
    9.873909240696694,
    10.106627003236781,
    10.342513269534024,
    10.58158024687427,
    10.8238400726681,
    11.069304815507364,
    11.317986476196008,
    11.569896988756009,
    11.825048221409341,
    12.083451977536606,
    12.345119996613247,
    12.610063955123938,
    12.878295467455942,
    13.149826086772048,
    13.42466730586372,
    13.702830557985108,
    13.984327217668513,
    14.269168601521828,
    14.55736596900856,
    14.848930523210871,
    15.143873411576273,
    15.44220572664832,
    15.743938506781891,
    16.04908273684337,
    16.35764934889634,
    16.66964922287304,
    16.985093187232053,
    17.30399201960269,
    17.62635644741625,
    17.95219714852476,
    18.281524751807332,
    18.614349837764564,
    18.95068293910138,
    19.290534541298456,
    19.633915083172692,
    19.98083495742689,
    20.331304511189067,
    20.685334046541502,
    21.042933821039977,
    21.404114048223256,
    21.76888489811322,
    22.137256497705877,
    22.50923893145328,
    22.884842241736916,
    23.264076429332462,
    23.6469514538663,
    24.033477234264016,
    24.42366364919083,
    24.817520537484558,
    25.21505769858089,
    25.61628489293138,
    26.021211842414342,
    26.429848230738664,
    26.842203703840827,
    27.258287870275353,
    27.678110301598522,
    28.10168053274597,
    28.529008062403893,
    28.96010235337422,
    29.39497283293396,
    29.83362889318845,
    30.276079891419332,
    30.722335150426627,
    31.172403958865512,
    31.62629557157785,
    32.08401920991837,
    32.54558406207592,
    33.010999283389665,
    33.4802739966603,
    33.953417292456834,
    34.430438229418264,
    34.911345834551085,
    35.39614910352207,
    35.88485700094671,
    36.37747846067349,
    36.87402238606382,
    37.37449765026789,
    37.87891309649659,
    38.38727753828926,
    38.89959975977785,
    39.41588851594697,
    39.93615253289054,
    40.460400508064545,
    40.98864111053629,
    41.520882981230194,
    42.05713473317016,
    42.597404951718396,
    43.141702194811224,
    43.6900349931913,
    44.24241185063697,
    44.798841244188324,
    45.35933162437017,
    45.92389141541209,
    46.49252901546552,
    47.065252796817916,
    47.64207110610409,
    48.22299226451468,
    48.808024568002054,
    49.3971762874833,
    49.9904556690408,
    50.587870934119984,
    51.189430279724725,
    51.79514187861014,
    52.40501387947288,
    53.0190544071392,
    53.637271562750364,
    54.259673423945976,
    54.88626804504493,
    55.517063457223934,
    56.15206766869424,
    56.79128866487574,
    57.43473440856916,
    58.08241284012621,
    58.734331877617365,
    59.39049941699807,
    60.05092333227251,
    60.715611475655585,
    61.38457167773311,
    62.057811747619894,
    62.7353394731159,
    63.417162620860914,
    64.10328893648692,
    64.79372614476921,
    65.48848194977529,
    66.18756403501224,
    66.89098006357258,
    67.59873767827808,
    68.31084450182222,
    69.02730813691093,
    69.74813616640164,
    70.47333615344107,
    71.20291564160104,
    71.93688215501312,
    72.67524319850172,
    73.41800625771542,
    74.16517879925733,
    74.9167682708136,
    75.67278210128072,
    76.43322770089146,
    77.1981124613393,
    77.96744375590167,
    78.74122893956174,
    79.51947534912904,
    80.30219030335869,
    81.08938110306934,
    81.88105503125999,
    82.67721935322541,
    83.4778813166706,
    84.28304815182372,
    85.09272707154808,
    85.90692527145302,
    86.72564993000343,
    87.54890820862819,
    88.3767072518277,
    89.2090541872801,
    90.04595612594655,
    90.88742016217518,
    91.73345337380438,
    92.58406282226491,
    93.43925555268066,
    94.29903859396902,
    95.16341895893969,
    96.03240364439274,
    96.9059996312159,
    97.78421388448044,
    98.6670533535366,
    99.55452497210776,
];

/// `HctSolver.sanitizeRadians`.
#[allow(
    clippy::modulo_arithmetic,
    reason = "a float remainder, which cannot panic"
)]
fn sanitize_radians(angle: f64) -> f64 {
    (angle + std::f64::consts::PI * 8.0) % (std::f64::consts::PI * 2.0)
}

/// `HctSolver.trueDelinearized`.
fn true_delinearized(rgb_component: f64) -> f64 {
    let normalized = rgb_component / 100.0;
    let delinearized = if normalized <= 0.0031308 {
        normalized * 12.92
    } else {
        1.055 * normalized.powf(1.0 / 2.4) - 0.055
    };
    delinearized * 255.0
}

/// `HctSolver.chromaticAdaptation`.
fn chromatic_adaptation(component: f64) -> f64 {
    let af = component.abs().powf(0.42);
    signum(component) * 400.0 * af / (af + 27.13)
}

/// `HctSolver.hueOf`: the CAM16 hue of a linear RGB colour, in radians.
fn hue_of(linrgb: [f64; 3]) -> f64 {
    let [d0, d1, d2] = matrix_multiply(linrgb, SCALED_DISCOUNT_FROM_LINRGB);
    let r_a = chromatic_adaptation(d0);
    let g_a = chromatic_adaptation(d1);
    let b_a = chromatic_adaptation(d2);
    let a = (11.0 * r_a + -12.0 * g_a + b_a) / 11.0;
    let b = (r_a + g_a - 2.0 * b_a) / 9.0;
    b.atan2(a)
}

/// `HctSolver.areInCyclicOrder`.
fn are_in_cyclic_order(a: f64, b: f64, c: f64) -> bool {
    let delta_ab = sanitize_radians(b - a);
    let delta_ac = sanitize_radians(c - a);
    delta_ab < delta_ac
}

/// `HctSolver.intercept`.
fn intercept(source: f64, mid: f64, target: f64) -> f64 {
    (mid - source) / (target - source)
}

/// `HctSolver.lerpPoint`.
fn lerp_point(source: [f64; 3], t: f64, target: [f64; 3]) -> [f64; 3] {
    let [s0, s1, s2] = source;
    let [t0, t1, t2] = target;
    [s0 + (t0 - s0) * t, s1 + (t1 - s1) * t, s2 + (t2 - s2) * t]
}

/// `HctSolver.setCoordinate`: the segment's intersection with the plane
/// `coordinate` of `axis`; `None` for an axis outside `0..3`.
fn set_coordinate(
    source: [f64; 3],
    coordinate: f64,
    target: [f64; 3],
    axis: usize,
) -> Option<[f64; 3]> {
    let t = intercept(*source.get(axis)?, coordinate, *target.get(axis)?);
    Some(lerp_point(source, t, target))
}

/// `HctSolver.isBounded`.
fn is_bounded(x: f64) -> bool {
    (0.0..=100.0).contains(&x)
}

/// `HctSolver.nthVertex`: the `n`th possible vertex (`0..12`) of the
/// intersection of the plane `y` and the RGB cube, `[-1, -1, -1]` where it
/// lies outside the cube. `n & 3` and `n & 1` are the source's `n % 4` and
/// `n % 2`.
fn nth_vertex(y: f64, n: u8) -> [f64; 3] {
    let [k_r, k_g, k_b] = Y_FROM_LINRGB;
    let coord_a = if n & 3 <= 1 { 0.0 } else { 100.0 };
    let coord_b = if n & 1 == 0 { 0.0 } else { 100.0 };
    if n < 4 {
        let g = coord_a;
        let b = coord_b;
        let r = (y - g * k_g - b * k_b) / k_r;
        if is_bounded(r) {
            [r, g, b]
        } else {
            [-1.0, -1.0, -1.0]
        }
    } else if n < 8 {
        let b = coord_a;
        let r = coord_b;
        let g = (y - r * k_r - b * k_b) / k_g;
        if is_bounded(g) {
            [r, g, b]
        } else {
            [-1.0, -1.0, -1.0]
        }
    } else {
        let r = coord_a;
        let g = coord_b;
        let b = (y - r * k_r - g * k_g) / k_b;
        if is_bounded(b) {
            [r, g, b]
        } else {
            [-1.0, -1.0, -1.0]
        }
    }
}

/// `HctSolver.bisectToSegment`.
fn bisect_to_segment(y: f64, target_hue: f64) -> [[f64; 3]; 2] {
    let mut left = [-1.0, -1.0, -1.0];
    let mut right = left;
    let mut left_hue = 0.0;
    let mut right_hue = 0.0;
    let mut initialized = false;
    let mut uncut = true;
    for n in 0..12 {
        let mid = nth_vertex(y, n);
        let [mid_r, ..] = mid;
        if mid_r < 0.0 {
            continue;
        }
        let mid_hue = hue_of(mid);
        if !initialized {
            left = mid;
            right = mid;
            left_hue = mid_hue;
            right_hue = mid_hue;
            initialized = true;
            continue;
        }
        if uncut || are_in_cyclic_order(left_hue, mid_hue, right_hue) {
            uncut = false;
            if are_in_cyclic_order(left_hue, target_hue, mid_hue) {
                right = mid;
                right_hue = mid_hue;
            } else {
                left = mid;
                left_hue = mid_hue;
            }
        }
    }
    [left, right]
}

/// `HctSolver.midpoint`.
fn midpoint(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    let [a0, a1, a2] = a;
    let [b0, b1, b2] = b;
    [(a0 + b0) / 2.0, (a1 + b1) / 2.0, (a2 + b2) / 2.0]
}

/// `HctSolver.criticalPlaneBelow`.
fn critical_plane_below(x: f64) -> i32 {
    (x - 0.5).floor() as i32
}

/// `HctSolver.criticalPlaneAbove`.
fn critical_plane_above(x: f64) -> i32 {
    (x - 0.5).ceil() as i32
}

/// `HctSolver.bisectToLimit`. A middle plane lies strictly between two
/// planes of `-1..=255`, so inside `CRITICAL_PLANES`; one outside it, which
/// the source would throw on, ends that axis' bisection.
fn bisect_to_limit(y: f64, target_hue: f64) -> [f64; 3] {
    let [mut left, mut right] = bisect_to_segment(y, target_hue);
    let mut left_hue = hue_of(left);
    for axis in 0..3 {
        let (Some(&left_axis), Some(&right_axis)) = (left.get(axis), right.get(axis)) else {
            continue;
        };
        if left_axis != right_axis {
            let (mut l_plane, mut r_plane) = if left_axis < right_axis {
                (
                    critical_plane_below(true_delinearized(left_axis)),
                    critical_plane_above(true_delinearized(right_axis)),
                )
            } else {
                (
                    critical_plane_above(true_delinearized(left_axis)),
                    critical_plane_below(true_delinearized(right_axis)),
                )
            };
            for _ in 0..8 {
                if r_plane.abs_diff(l_plane) <= 1 {
                    break;
                }
                let m_plane = ((f64::from(l_plane) + f64::from(r_plane)) / 2.0).floor() as i32;
                let Some(&mid_plane_coordinate) = usize::try_from(m_plane)
                    .ok()
                    .and_then(|index| CRITICAL_PLANES.get(index))
                else {
                    break;
                };
                let Some(mid) = set_coordinate(left, mid_plane_coordinate, right, axis) else {
                    break;
                };
                let mid_hue = hue_of(mid);
                if are_in_cyclic_order(left_hue, target_hue, mid_hue) {
                    right = mid;
                    r_plane = m_plane;
                } else {
                    left = mid;
                    left_hue = mid_hue;
                    l_plane = m_plane;
                }
            }
        }
    }
    midpoint(left, right)
}

/// `HctSolver.inverseChromaticAdaptation`.
fn inverse_chromatic_adaptation(adapted: f64) -> f64 {
    let adapted_abs = adapted.abs();
    let base = f64::max(0.0, 27.13 * adapted_abs / (400.0 - adapted_abs));
    signum(adapted) * base.powf(1.0 / 0.42)
}

/// `HctSolver.findResultByJ`: the colour of the hue, chroma and Y, if found.
fn find_result_by_j(hue_radians: f64, chroma: f64, y: f64) -> Option<Rgb> {
    // Initial estimate of j.
    let mut j = y.sqrt() * 11.0;
    let viewing_conditions = &*DEFAULT_VIEWING_CONDITIONS;
    let t_inner_coeff = 1.0 / (1.64 - 0.29_f64.powf(viewing_conditions.n)).powf(0.73);
    let e_hue = 0.25 * ((hue_radians + 2.0).cos() + 3.8);
    let p1 = e_hue * (50000.0 / 13.0) * viewing_conditions.nc * viewing_conditions.ncb;
    let h_sin = hue_radians.sin();
    let h_cos = hue_radians.cos();
    for iteration_round in 0..5 {
        let j_normalized = j / 100.0;
        let alpha = if chroma == 0.0 || j == 0.0 {
            0.0
        } else {
            chroma / j_normalized.sqrt()
        };
        let t = (alpha * t_inner_coeff).powf(1.0 / 0.9);
        let ac = viewing_conditions.aw
            * j_normalized.powf(1.0 / viewing_conditions.c / viewing_conditions.z);
        let p2 = ac / viewing_conditions.nbb;
        let gamma = 23.0 * (p2 + 0.305) * t / (23.0 * p1 + 11.0 * t * h_cos + 108.0 * t * h_sin);
        let a = gamma * h_cos;
        let b = gamma * h_sin;
        let r_a = (460.0 * p2 + 451.0 * a + 288.0 * b) / 1403.0;
        let g_a = (460.0 * p2 - 891.0 * a - 261.0 * b) / 1403.0;
        let b_a = (460.0 * p2 - 220.0 * a - 6300.0 * b) / 1403.0;
        let r_c_scaled = inverse_chromatic_adaptation(r_a);
        let g_c_scaled = inverse_chromatic_adaptation(g_a);
        let b_c_scaled = inverse_chromatic_adaptation(b_a);
        let linrgb = matrix_multiply(
            [r_c_scaled, g_c_scaled, b_c_scaled],
            LINRGB_FROM_SCALED_DISCOUNT,
        );
        let [l0, l1, l2] = linrgb;
        if l0 < 0.0 || l1 < 0.0 || l2 < 0.0 {
            return None;
        }
        let [k_r, k_g, k_b] = Y_FROM_LINRGB;
        let fnj = k_r * l0 + k_g * l1 + k_b * l2;
        if fnj <= 0.0 {
            return None;
        }
        if iteration_round == 4 || (fnj - y).abs() < 0.002 {
            if l0 > 100.01 || l1 > 100.01 || l2 > 100.01 {
                return None;
            }
            return Some(argb_from_linrgb(linrgb));
        }
        // Iterates with Newton method,
        // Using 2 * fn(j) / j as the approximation of fn'(j)
        j = j - (fnj - y) * j / (2.0 * fnj);
    }
    None
}

/// `HctSolver.solveToInt`: the sRGB colour of the hue, chroma and L*, the
/// chroma reduced where the gamut has less.
///
/// The source's 0, `findResultByJ`'s "not found", is a transparent black no
/// solution of an opaque colour can be; `None` stands for it here.
fn solve_to_int(hue_degrees: f64, chroma: f64, lstar: f64) -> Rgb {
    if chroma < 0.0001 || lstar < 0.0001 || lstar > 99.9999 {
        return argb_from_lstar(lstar);
    }
    let hue_degrees = sanitize_degrees_double(hue_degrees);
    let hue_radians = hue_degrees / 180.0 * std::f64::consts::PI;
    let y = y_from_lstar(lstar);
    if let Some(exact_answer) = find_result_by_j(hue_radians, chroma, y) {
        return exact_answer;
    }
    let linrgb = bisect_to_limit(y, hue_radians);
    argb_from_linrgb(linrgb)
}

// ---------------------------------------------------------------------------
// hct/Hct.java

/// `Hct.isYellow`.
fn is_yellow(hue: f64) -> bool {
    (105.0..125.0).contains(&hue)
}

// ---------------------------------------------------------------------------
// palettes/TonalPalette.java

/// A tonal palette: a hue and a chroma (`TonalPalette.fromHueAndChroma`).
#[derive(Clone, Copy, Debug)]
struct TonalPalette {
    hue: f64,
    chroma: f64,
}

impl TonalPalette {
    /// `TonalPalette.tone`, uncached.
    fn tone(self, tone: u8) -> Rgb {
        if tone == 99 && is_yellow(self.hue) {
            average_argb(self.tone(98), self.tone(100))
        } else {
            solve_to_int(self.hue, self.chroma, f64::from(tone))
        }
    }

    /// `TonalPalette.getHct(tone).toInt()`.
    fn get_hct(self, tone: f64) -> Rgb {
        if tone == 99.0 && is_yellow(self.hue) {
            return self.tone(99);
        }
        solve_to_int(self.hue, self.chroma, tone)
    }
}

/// `TonalPalette.averageArgb`: each channel's mean, `Math.round` of the
/// float, which a half rounds up.
fn average_argb(argb1: Rgb, argb2: Rgb) -> Rgb {
    let mean = |c1: u8, c2: u8| java_round((f64::from(c1) + f64::from(c2)) / 2.0) as u8;
    Rgb {
        r: mean(argb1.r, argb2.r),
        g: mean(argb1.g, argb2.g),
        b: mean(argb1.b, argb2.b),
    }
}

// ---------------------------------------------------------------------------
// contrast/Contrast.java

/// `Contrast.CONTRAST_RATIO_EPSILON`.
const CONTRAST_RATIO_EPSILON: f64 = 0.04;

/// `Contrast.LUMINANCE_GAMUT_MAP_TOLERANCE`.
const LUMINANCE_GAMUT_MAP_TOLERANCE: f64 = 0.4;

/// `Contrast.ratioOfYs`.
fn ratio_of_ys(y1: f64, y2: f64) -> f64 {
    let lighter = f64::max(y1, y2);
    let darker = if lighter == y2 { y1 } else { y2 };
    (lighter + 5.0) / (darker + 5.0)
}

/// `Contrast.ratioOfTones`.
fn ratio_of_tones(t1: f64, t2: f64) -> f64 {
    ratio_of_ys(y_from_lstar(t1), y_from_lstar(t2))
}

/// `Contrast.lighter`: -1 where no tone reaches the ratio.
fn lighter(tone: f64, ratio: f64) -> f64 {
    if !(0.0..=100.0).contains(&tone) {
        return -1.0;
    }
    let dark_y = y_from_lstar(tone);
    let light_y = ratio * (dark_y + 5.0) - 5.0;
    if !(0.0..=100.0).contains(&light_y) {
        return -1.0;
    }
    let real_contrast = ratio_of_ys(light_y, dark_y);
    let delta = (real_contrast - ratio).abs();
    if real_contrast < ratio && delta > CONTRAST_RATIO_EPSILON {
        return -1.0;
    }
    let return_value = lstar_from_y(light_y) + LUMINANCE_GAMUT_MAP_TOLERANCE;
    if !(0.0..=100.0).contains(&return_value) {
        return -1.0;
    }
    return_value
}

/// `Contrast.lighterUnsafe`.
fn lighter_unsafe(tone: f64, ratio: f64) -> f64 {
    let lighter_safe = lighter(tone, ratio);
    if lighter_safe < 0.0 {
        100.0
    } else {
        lighter_safe
    }
}

/// `Contrast.darker`: -1 where no tone reaches the ratio.
fn darker(tone: f64, ratio: f64) -> f64 {
    if !(0.0..=100.0).contains(&tone) {
        return -1.0;
    }
    let light_y = y_from_lstar(tone);
    let dark_y = ((light_y + 5.0) / ratio) - 5.0;
    if !(0.0..=100.0).contains(&dark_y) {
        return -1.0;
    }
    let real_contrast = ratio_of_ys(light_y, dark_y);
    let delta = (real_contrast - ratio).abs();
    if real_contrast < ratio && delta > CONTRAST_RATIO_EPSILON {
        return -1.0;
    }
    let return_value = lstar_from_y(dark_y) - LUMINANCE_GAMUT_MAP_TOLERANCE;
    if !(0.0..=100.0).contains(&return_value) {
        return -1.0;
    }
    return_value
}

/// `Contrast.darkerUnsafe`.
fn darker_unsafe(tone: f64, ratio: f64) -> f64 {
    let darker_safe = darker(tone, ratio);
    f64::max(0.0, darker_safe)
}

// ---------------------------------------------------------------------------
// dynamiccolor/DynamicScheme.java, scheme/SchemeTonalSpot.java

/// A dynamic scheme: the conditions a role is resolved under.
struct DynamicScheme {
    is_dark: bool,
    contrast_level: f64,
    primary_palette: TonalPalette,
    secondary_palette: TonalPalette,
    tertiary_palette: TonalPalette,
    neutral_palette: TonalPalette,
    neutral_variant_palette: TonalPalette,
    error_palette: TonalPalette,
}

impl DynamicScheme {
    /// `new SchemeTonalSpot(sourceColorHct, isDark, contrastLevel)` under
    /// `SPEC_2021`: `ColorSpec2021`'s `TONAL_SPOT` palettes of the source
    /// hue, and `DynamicScheme`'s error palette, as the spec's is empty.
    fn tonal_spot(source_hue: f64, is_dark: bool, contrast_level: f64) -> DynamicScheme {
        let palette = |hue, chroma| TonalPalette { hue, chroma };
        DynamicScheme {
            is_dark,
            contrast_level,
            primary_palette: palette(source_hue, 36.0),
            secondary_palette: palette(source_hue, 16.0),
            tertiary_palette: palette(sanitize_degrees_double(source_hue + 60.0), 24.0),
            neutral_palette: palette(source_hue, 6.0),
            neutral_variant_palette: palette(source_hue, 8.0),
            error_palette: palette(25.0, 84.0),
        }
    }
}

// ---------------------------------------------------------------------------
// dynamiccolor/ContrastCurve.java, ToneDeltaPair.java, TonePolarity.java

/// `ContrastCurve`: a value at each of four contrast levels.
#[derive(Clone, Copy, Debug)]
struct ContrastCurve {
    low: f64,
    normal: f64,
    medium: f64,
    high: f64,
}

/// `new ContrastCurve(low, normal, medium, high)`.
const fn curve(low: f64, normal: f64, medium: f64, high: f64) -> ContrastCurve {
    ContrastCurve {
        low,
        normal,
        medium,
        high,
    }
}

impl ContrastCurve {
    /// `ContrastCurve.get`.
    fn get(self, contrast_level: f64) -> f64 {
        if contrast_level <= -1.0 {
            self.low
        } else if contrast_level < 0.0 {
            lerp(self.low, self.normal, (contrast_level - -1.0) / 1.0)
        } else if contrast_level < 0.5 {
            lerp(self.normal, self.medium, (contrast_level - 0.0) / 0.5)
        } else if contrast_level < 1.0 {
            lerp(self.medium, self.high, (contrast_level - 0.5) / 0.5)
        } else {
            self.high
        }
    }
}

/// `TonePolarity`, the members `ColorSpec2021`'s roles use.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TonePolarity {
    /// `NEARER`.
    Nearer,
    /// `LIGHTER`.
    Lighter,
}

/// `ToneDeltaPair`.
#[derive(Clone, Copy, Debug)]
struct ToneDeltaPair {
    role_a: Role,
    role_b: Role,
    delta: f64,
    polarity: TonePolarity,
    stay_together: bool,
}

// ---------------------------------------------------------------------------
// dynamiccolor/DynamicColor.java

/// `DynamicColor.foregroundTone`.
fn foreground_tone(bg_tone: f64, ratio: f64) -> f64 {
    let lighter_tone = lighter_unsafe(bg_tone, ratio);
    let darker_tone = darker_unsafe(bg_tone, ratio);
    let lighter_ratio = ratio_of_tones(lighter_tone, bg_tone);
    let darker_ratio = ratio_of_tones(darker_tone, bg_tone);
    let prefer_lighter = tone_prefers_light_foreground(bg_tone);
    if prefer_lighter {
        let negligible_difference = (lighter_ratio - darker_ratio).abs() < 0.1
            && lighter_ratio < ratio
            && darker_ratio < ratio;
        if lighter_ratio >= ratio || lighter_ratio >= darker_ratio || negligible_difference {
            lighter_tone
        } else {
            darker_tone
        }
    } else if darker_ratio >= ratio || darker_ratio >= lighter_ratio {
        darker_tone
    } else {
        lighter_tone
    }
}

/// `DynamicColor.tonePrefersLightForeground`.
fn tone_prefers_light_foreground(tone: f64) -> bool {
    java_round(tone) < 60.0
}

// ---------------------------------------------------------------------------
// dynamiccolor/ColorSpec2021.java

/// The roles of `md.sys.color`, each `ColorSpec2021`'s `DynamicColor` of
/// that name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Role {
    Background,
    OnBackground,
    Surface,
    SurfaceDim,
    SurfaceBright,
    SurfaceContainerLowest,
    SurfaceContainerLow,
    SurfaceContainer,
    SurfaceContainerHigh,
    SurfaceContainerHighest,
    OnSurface,
    SurfaceVariant,
    OnSurfaceVariant,
    InverseSurface,
    InverseOnSurface,
    Outline,
    OutlineVariant,
    Shadow,
    Scrim,
    SurfaceTint,
    Primary,
    OnPrimary,
    PrimaryContainer,
    OnPrimaryContainer,
    InversePrimary,
    Secondary,
    OnSecondary,
    SecondaryContainer,
    OnSecondaryContainer,
    Tertiary,
    OnTertiary,
    TertiaryContainer,
    OnTertiaryContainer,
    Error,
    OnError,
    ErrorContainer,
    OnErrorContainer,
    PrimaryFixed,
    PrimaryFixedDim,
    OnPrimaryFixed,
    OnPrimaryFixedVariant,
    SecondaryFixed,
    SecondaryFixedDim,
    OnSecondaryFixed,
    OnSecondaryFixedVariant,
    TertiaryFixed,
    TertiaryFixedDim,
    OnTertiaryFixed,
    OnTertiaryFixedVariant,
}

impl Role {
    /// The role `DynamicColor.name` names.
    fn from_name(name: &str) -> Option<Role> {
        Some(match name {
            "background" => Role::Background,
            "on_background" => Role::OnBackground,
            "surface" => Role::Surface,
            "surface_dim" => Role::SurfaceDim,
            "surface_bright" => Role::SurfaceBright,
            "surface_container_lowest" => Role::SurfaceContainerLowest,
            "surface_container_low" => Role::SurfaceContainerLow,
            "surface_container" => Role::SurfaceContainer,
            "surface_container_high" => Role::SurfaceContainerHigh,
            "surface_container_highest" => Role::SurfaceContainerHighest,
            "on_surface" => Role::OnSurface,
            "surface_variant" => Role::SurfaceVariant,
            "on_surface_variant" => Role::OnSurfaceVariant,
            "inverse_surface" => Role::InverseSurface,
            "inverse_on_surface" => Role::InverseOnSurface,
            "outline" => Role::Outline,
            "outline_variant" => Role::OutlineVariant,
            "shadow" => Role::Shadow,
            "scrim" => Role::Scrim,
            "surface_tint" => Role::SurfaceTint,
            "primary" => Role::Primary,
            "on_primary" => Role::OnPrimary,
            "primary_container" => Role::PrimaryContainer,
            "on_primary_container" => Role::OnPrimaryContainer,
            "inverse_primary" => Role::InversePrimary,
            "secondary" => Role::Secondary,
            "on_secondary" => Role::OnSecondary,
            "secondary_container" => Role::SecondaryContainer,
            "on_secondary_container" => Role::OnSecondaryContainer,
            "tertiary" => Role::Tertiary,
            "on_tertiary" => Role::OnTertiary,
            "tertiary_container" => Role::TertiaryContainer,
            "on_tertiary_container" => Role::OnTertiaryContainer,
            "error" => Role::Error,
            "on_error" => Role::OnError,
            "error_container" => Role::ErrorContainer,
            "on_error_container" => Role::OnErrorContainer,
            "primary_fixed" => Role::PrimaryFixed,
            "primary_fixed_dim" => Role::PrimaryFixedDim,
            "on_primary_fixed" => Role::OnPrimaryFixed,
            "on_primary_fixed_variant" => Role::OnPrimaryFixedVariant,
            "secondary_fixed" => Role::SecondaryFixed,
            "secondary_fixed_dim" => Role::SecondaryFixedDim,
            "on_secondary_fixed" => Role::OnSecondaryFixed,
            "on_secondary_fixed_variant" => Role::OnSecondaryFixedVariant,
            "tertiary_fixed" => Role::TertiaryFixed,
            "tertiary_fixed_dim" => Role::TertiaryFixedDim,
            "on_tertiary_fixed" => Role::OnTertiaryFixed,
            "on_tertiary_fixed_variant" => Role::OnTertiaryFixedVariant,
            _ => return None,
        })
    }

    /// The role's `palette`.
    fn palette(self, s: &DynamicScheme) -> TonalPalette {
        match self {
            Role::Background
            | Role::OnBackground
            | Role::Surface
            | Role::SurfaceDim
            | Role::SurfaceBright
            | Role::SurfaceContainerLowest
            | Role::SurfaceContainerLow
            | Role::SurfaceContainer
            | Role::SurfaceContainerHigh
            | Role::SurfaceContainerHighest
            | Role::OnSurface
            | Role::InverseSurface
            | Role::InverseOnSurface
            | Role::Shadow
            | Role::Scrim => s.neutral_palette,
            Role::SurfaceVariant
            | Role::OnSurfaceVariant
            | Role::Outline
            | Role::OutlineVariant => s.neutral_variant_palette,
            Role::SurfaceTint
            | Role::Primary
            | Role::OnPrimary
            | Role::PrimaryContainer
            | Role::OnPrimaryContainer
            | Role::InversePrimary
            | Role::PrimaryFixed
            | Role::PrimaryFixedDim
            | Role::OnPrimaryFixed
            | Role::OnPrimaryFixedVariant => s.primary_palette,
            Role::Secondary
            | Role::OnSecondary
            | Role::SecondaryContainer
            | Role::OnSecondaryContainer
            | Role::SecondaryFixed
            | Role::SecondaryFixedDim
            | Role::OnSecondaryFixed
            | Role::OnSecondaryFixedVariant => s.secondary_palette,
            Role::Tertiary
            | Role::OnTertiary
            | Role::TertiaryContainer
            | Role::OnTertiaryContainer
            | Role::TertiaryFixed
            | Role::TertiaryFixedDim
            | Role::OnTertiaryFixed
            | Role::OnTertiaryFixedVariant => s.tertiary_palette,
            Role::Error | Role::OnError | Role::ErrorContainer | Role::OnErrorContainer => {
                s.error_palette
            }
        }
    }

    /// The role's `tone`, the tone before contrast adjustment.
    fn tone(self, s: &DynamicScheme) -> f64 {
        let dark = |dark: f64, light: f64| if s.is_dark { dark } else { light };
        let level = s.contrast_level;
        match self {
            Role::Background | Role::Surface => dark(6.0, 98.0),
            Role::OnBackground | Role::OnSurface => dark(90.0, 10.0),
            Role::SurfaceDim => {
                if s.is_dark {
                    6.0
                } else {
                    curve(87.0, 87.0, 80.0, 75.0).get(level)
                }
            }
            Role::SurfaceBright => {
                if s.is_dark {
                    curve(24.0, 24.0, 29.0, 34.0).get(level)
                } else {
                    98.0
                }
            }
            Role::SurfaceContainerLowest => {
                if s.is_dark {
                    curve(4.0, 4.0, 2.0, 0.0).get(level)
                } else {
                    100.0
                }
            }
            Role::SurfaceContainerLow => {
                if s.is_dark {
                    curve(10.0, 10.0, 11.0, 12.0).get(level)
                } else {
                    curve(96.0, 96.0, 96.0, 95.0).get(level)
                }
            }
            Role::SurfaceContainer => {
                if s.is_dark {
                    curve(12.0, 12.0, 16.0, 20.0).get(level)
                } else {
                    curve(94.0, 94.0, 92.0, 90.0).get(level)
                }
            }
            Role::SurfaceContainerHigh => {
                if s.is_dark {
                    curve(17.0, 17.0, 21.0, 25.0).get(level)
                } else {
                    curve(92.0, 92.0, 88.0, 85.0).get(level)
                }
            }
            Role::SurfaceContainerHighest => {
                if s.is_dark {
                    curve(22.0, 22.0, 26.0, 30.0).get(level)
                } else {
                    curve(90.0, 90.0, 84.0, 80.0).get(level)
                }
            }
            Role::SurfaceVariant => dark(30.0, 90.0),
            Role::OnSurfaceVariant => dark(80.0, 30.0),
            Role::InverseSurface => dark(90.0, 20.0),
            Role::InverseOnSurface => dark(20.0, 95.0),
            Role::Outline => dark(60.0, 50.0),
            Role::OutlineVariant => dark(30.0, 80.0),
            Role::Shadow | Role::Scrim => 0.0,
            Role::SurfaceTint | Role::Primary | Role::Secondary | Role::Tertiary | Role::Error => {
                dark(80.0, 40.0)
            }
            Role::OnPrimary | Role::OnSecondary | Role::OnTertiary | Role::OnError => {
                dark(20.0, 100.0)
            }
            Role::PrimaryContainer
            | Role::SecondaryContainer
            | Role::TertiaryContainer
            | Role::ErrorContainer => dark(30.0, 90.0),
            Role::OnPrimaryContainer
            | Role::OnSecondaryContainer
            | Role::OnTertiaryContainer
            | Role::OnErrorContainer => dark(90.0, 30.0),
            Role::InversePrimary => dark(40.0, 80.0),
            Role::PrimaryFixed | Role::SecondaryFixed | Role::TertiaryFixed => 90.0,
            Role::PrimaryFixedDim | Role::SecondaryFixedDim | Role::TertiaryFixedDim => 80.0,
            Role::OnPrimaryFixed | Role::OnSecondaryFixed | Role::OnTertiaryFixed => 10.0,
            Role::OnPrimaryFixedVariant
            | Role::OnSecondaryFixedVariant
            | Role::OnTertiaryFixedVariant => 30.0,
        }
    }

    /// The role's `isBackground`.
    fn is_background(self) -> bool {
        matches!(
            self,
            Role::Background
                | Role::Surface
                | Role::SurfaceDim
                | Role::SurfaceBright
                | Role::SurfaceContainerLowest
                | Role::SurfaceContainerLow
                | Role::SurfaceContainer
                | Role::SurfaceContainerHigh
                | Role::SurfaceContainerHighest
                | Role::SurfaceVariant
                | Role::InverseSurface
                | Role::SurfaceTint
                | Role::Primary
                | Role::PrimaryContainer
                | Role::Secondary
                | Role::SecondaryContainer
                | Role::Tertiary
                | Role::TertiaryContainer
                | Role::Error
                | Role::ErrorContainer
                | Role::PrimaryFixed
                | Role::PrimaryFixedDim
                | Role::SecondaryFixed
                | Role::SecondaryFixedDim
                | Role::TertiaryFixed
                | Role::TertiaryFixedDim
        )
    }

    /// The role's `background`.
    fn background(self, s: &DynamicScheme) -> Option<Role> {
        Some(match self {
            Role::OnBackground => Role::Background,
            Role::OnSurface
            | Role::OnSurfaceVariant
            | Role::Outline
            | Role::OutlineVariant
            | Role::Primary
            | Role::PrimaryContainer
            | Role::Secondary
            | Role::SecondaryContainer
            | Role::Tertiary
            | Role::TertiaryContainer
            | Role::Error
            | Role::ErrorContainer
            | Role::PrimaryFixed
            | Role::PrimaryFixedDim
            | Role::SecondaryFixed
            | Role::SecondaryFixedDim
            | Role::TertiaryFixed
            | Role::TertiaryFixedDim => highest_surface(s),
            Role::InverseOnSurface | Role::InversePrimary => Role::InverseSurface,
            Role::OnPrimary => Role::Primary,
            Role::OnPrimaryContainer => Role::PrimaryContainer,
            Role::OnSecondary => Role::Secondary,
            Role::OnSecondaryContainer => Role::SecondaryContainer,
            Role::OnTertiary => Role::Tertiary,
            Role::OnTertiaryContainer => Role::TertiaryContainer,
            Role::OnError => Role::Error,
            Role::OnErrorContainer => Role::ErrorContainer,
            Role::OnPrimaryFixed | Role::OnPrimaryFixedVariant => Role::PrimaryFixedDim,
            Role::OnSecondaryFixed | Role::OnSecondaryFixedVariant => Role::SecondaryFixedDim,
            Role::OnTertiaryFixed | Role::OnTertiaryFixedVariant => Role::TertiaryFixedDim,
            Role::Background
            | Role::Surface
            | Role::SurfaceDim
            | Role::SurfaceBright
            | Role::SurfaceContainerLowest
            | Role::SurfaceContainerLow
            | Role::SurfaceContainer
            | Role::SurfaceContainerHigh
            | Role::SurfaceContainerHighest
            | Role::SurfaceVariant
            | Role::InverseSurface
            | Role::Shadow
            | Role::Scrim
            | Role::SurfaceTint => return None,
        })
    }

    /// The role's `secondBackground`.
    fn second_background(self) -> Option<Role> {
        match self {
            Role::OnPrimaryFixed | Role::OnPrimaryFixedVariant => Some(Role::PrimaryFixed),
            Role::OnSecondaryFixed | Role::OnSecondaryFixedVariant => Some(Role::SecondaryFixed),
            Role::OnTertiaryFixed | Role::OnTertiaryFixedVariant => Some(Role::TertiaryFixed),
            _ => None,
        }
    }

    /// The role's `contrastCurve`.
    fn contrast_curve(self) -> Option<ContrastCurve> {
        Some(match self {
            Role::OnBackground => curve(3.0, 3.0, 4.5, 7.0),
            Role::OnSurface
            | Role::InverseOnSurface
            | Role::OnPrimary
            | Role::OnSecondary
            | Role::OnTertiary
            | Role::OnError
            | Role::OnPrimaryFixed
            | Role::OnSecondaryFixed
            | Role::OnTertiaryFixed => curve(4.5, 7.0, 11.0, 21.0),
            Role::OnSurfaceVariant
            | Role::OnPrimaryContainer
            | Role::OnSecondaryContainer
            | Role::OnTertiaryContainer
            | Role::OnErrorContainer
            | Role::OnPrimaryFixedVariant
            | Role::OnSecondaryFixedVariant
            | Role::OnTertiaryFixedVariant => curve(3.0, 4.5, 7.0, 11.0),
            Role::Outline => curve(1.5, 3.0, 4.5, 7.0),
            Role::OutlineVariant
            | Role::PrimaryContainer
            | Role::SecondaryContainer
            | Role::TertiaryContainer
            | Role::ErrorContainer
            | Role::PrimaryFixed
            | Role::PrimaryFixedDim
            | Role::SecondaryFixed
            | Role::SecondaryFixedDim
            | Role::TertiaryFixed
            | Role::TertiaryFixedDim => curve(1.0, 1.0, 3.0, 4.5),
            Role::Primary
            | Role::InversePrimary
            | Role::Secondary
            | Role::Tertiary
            | Role::Error => curve(3.0, 4.5, 7.0, 7.0),
            Role::Background
            | Role::Surface
            | Role::SurfaceDim
            | Role::SurfaceBright
            | Role::SurfaceContainerLowest
            | Role::SurfaceContainerLow
            | Role::SurfaceContainer
            | Role::SurfaceContainerHigh
            | Role::SurfaceContainerHighest
            | Role::SurfaceVariant
            | Role::InverseSurface
            | Role::Shadow
            | Role::Scrim
            | Role::SurfaceTint => return None,
        })
    }

    /// The role's `toneDeltaPair`.
    fn tone_delta_pair(self) -> Option<ToneDeltaPair> {
        let pair = |role_a, role_b, polarity, stay_together| ToneDeltaPair {
            role_a,
            role_b,
            delta: 10.0,
            polarity,
            stay_together,
        };
        Some(match self {
            Role::Primary | Role::PrimaryContainer => pair(
                Role::PrimaryContainer,
                Role::Primary,
                TonePolarity::Nearer,
                false,
            ),
            Role::Secondary | Role::SecondaryContainer => pair(
                Role::SecondaryContainer,
                Role::Secondary,
                TonePolarity::Nearer,
                false,
            ),
            Role::Tertiary | Role::TertiaryContainer => pair(
                Role::TertiaryContainer,
                Role::Tertiary,
                TonePolarity::Nearer,
                false,
            ),
            Role::Error | Role::ErrorContainer => pair(
                Role::ErrorContainer,
                Role::Error,
                TonePolarity::Nearer,
                false,
            ),
            Role::PrimaryFixed | Role::PrimaryFixedDim => pair(
                Role::PrimaryFixed,
                Role::PrimaryFixedDim,
                TonePolarity::Lighter,
                true,
            ),
            Role::SecondaryFixed | Role::SecondaryFixedDim => pair(
                Role::SecondaryFixed,
                Role::SecondaryFixedDim,
                TonePolarity::Lighter,
                true,
            ),
            Role::TertiaryFixed | Role::TertiaryFixedDim => pair(
                Role::TertiaryFixed,
                Role::TertiaryFixedDim,
                TonePolarity::Lighter,
                true,
            ),
            _ => return None,
        })
    }
}

/// `ColorSpec2021.highestSurface`.
fn highest_surface(s: &DynamicScheme) -> Role {
    if s.is_dark {
        Role::SurfaceBright
    } else {
        Role::SurfaceDim
    }
}

/// `ColorSpec2021.getHct(scheme, color).toInt()`: the role's palette at its
/// tone.
fn get_hct(scheme: &DynamicScheme, color: Role) -> Rgb {
    let tone = get_tone(scheme, color);
    color.palette(scheme).get_hct(tone)
}

/// `ColorSpec2021.getTone`: the role's tone, adjusted for contrast against
/// its backgrounds and for its delta to its pair.
fn get_tone(scheme: &DynamicScheme, color: Role) -> f64 {
    let decreasing_contrast = scheme.contrast_level < 0.0;

    // Case 1: dual foreground, pair of colors with delta constraint.
    if let Some(tone_delta_pair) = color.tone_delta_pair() {
        let role_a = tone_delta_pair.role_a;
        let role_b = tone_delta_pair.role_b;
        let delta = tone_delta_pair.delta;
        let polarity = tone_delta_pair.polarity;
        let stay_together = tone_delta_pair.stay_together;

        // The source's third clause, `DARKER` in light mode, names a
        // polarity no 2021 role has.
        let a_is_nearer = polarity == TonePolarity::Nearer
            || (polarity == TonePolarity::Lighter && !scheme.is_dark);
        let nearer = if a_is_nearer { role_a } else { role_b };
        let farther = if a_is_nearer { role_b } else { role_a };
        let am_nearer = color == nearer;
        let expansion_dir = if scheme.is_dark { 1.0 } else { -1.0 };
        let mut n_tone = nearer.tone(scheme);
        let mut f_tone = farther.tone(scheme);

        // 1st round: solve to min, each
        if let (Some(bg), Some(n_contrast_curve), Some(f_contrast_curve)) = (
            color.background(scheme),
            nearer.contrast_curve(),
            farther.contrast_curve(),
        ) {
            let n_contrast = n_contrast_curve.get(scheme.contrast_level);
            let f_contrast = f_contrast_curve.get(scheme.contrast_level);
            let bg_tone = get_tone(scheme, bg);

            // If a color is good enough, it is not adjusted.
            // Initial and adjusted tones for `nearer`
            if ratio_of_tones(bg_tone, n_tone) < n_contrast {
                n_tone = foreground_tone(bg_tone, n_contrast);
            }
            // Initial and adjusted tones for `farther`
            if ratio_of_tones(bg_tone, f_tone) < f_contrast {
                f_tone = foreground_tone(bg_tone, f_contrast);
            }

            if decreasing_contrast {
                // If decreasing contrast, adjust color to the "bare minimum"
                // that satisfies contrast.
                n_tone = foreground_tone(bg_tone, n_contrast);
                f_tone = foreground_tone(bg_tone, f_contrast);
            }
        }

        // If constraint is not satisfied, try another round.
        if (f_tone - n_tone) * expansion_dir < delta {
            // 2nd round: expand farther to match delta.
            f_tone = clamp_double(0.0, 100.0, n_tone + delta * expansion_dir);
            // If constraint is not satisfied, try another round.
            if (f_tone - n_tone) * expansion_dir < delta {
                // 3rd round: contract nearer to match delta.
                n_tone = clamp_double(0.0, 100.0, f_tone - delta * expansion_dir);
            }
        }

        // Avoids the 50-59 awkward zone.
        if (50.0..60.0).contains(&n_tone) {
            // If `nearer` is in the awkward zone, move it away, together with
            // `farther`.
            if expansion_dir > 0.0 {
                n_tone = 60.0;
                f_tone = f64::max(f_tone, n_tone + delta * expansion_dir);
            } else {
                n_tone = 49.0;
                f_tone = f64::min(f_tone, n_tone + delta * expansion_dir);
            }
        } else if (50.0..60.0).contains(&f_tone) {
            if stay_together {
                // Fixes both, to avoid two colors on opposite sides of the "awkward
                // zone".
                if expansion_dir > 0.0 {
                    n_tone = 60.0;
                    f_tone = f64::max(f_tone, n_tone + delta * expansion_dir);
                } else {
                    n_tone = 49.0;
                    f_tone = f64::min(f_tone, n_tone + delta * expansion_dir);
                }
            } else {
                // Not required to stay together; fixes just one.
                if expansion_dir > 0.0 {
                    f_tone = 60.0;
                } else {
                    f_tone = 49.0;
                }
            }
        }

        // Returns `nTone` if this color is `nearer`, otherwise `fTone`.
        return if am_nearer { n_tone } else { f_tone };
    }

    // Case 2: No contrast pair; just solve for itself.
    let mut answer = color.tone(scheme);

    let (Some(background), Some(contrast_curve)) =
        (color.background(scheme), color.contrast_curve())
    else {
        return answer; // No adjustment for colors with no background.
    };

    let bg_tone = get_tone(scheme, background);
    let desired_ratio = contrast_curve.get(scheme.contrast_level);

    if ratio_of_tones(bg_tone, answer) >= desired_ratio {
        // Don't "improve" what's good enough.
    } else {
        // Rough improvement.
        answer = foreground_tone(bg_tone, desired_ratio);
    }

    if decreasing_contrast {
        answer = foreground_tone(bg_tone, desired_ratio);
    }

    if color.is_background() && (50.0..60.0).contains(&answer) {
        // Must adjust
        if ratio_of_tones(49.0, bg_tone) >= desired_ratio {
            answer = 49.0;
        } else {
            answer = 60.0;
        }
    }

    let Some(second_background) = color.second_background() else {
        return answer;
    };

    // Case 3: Adjust for dual backgrounds.
    let bg_tone1 = get_tone(scheme, background);
    let bg_tone2 = get_tone(scheme, second_background);

    let upper = f64::max(bg_tone1, bg_tone2);
    let lower = f64::min(bg_tone1, bg_tone2);

    if ratio_of_tones(upper, answer) >= desired_ratio
        && ratio_of_tones(lower, answer) >= desired_ratio
    {
        return answer;
    }

    // The darkest light tone that satisfies the desired ratio,
    // or -1 if such ratio cannot be reached.
    let light_option = lighter(upper, desired_ratio);

    // The lightest dark tone that satisfies the desired ratio,
    // or -1 if such ratio cannot be reached.
    let dark_option = darker(lower, desired_ratio);

    // Tones suitable for the foreground.
    let mut availables = Vec::with_capacity(2);
    if light_option != -1.0 {
        availables.push(light_option);
    }
    if dark_option != -1.0 {
        availables.push(dark_option);
    }

    let prefers_light =
        tone_prefers_light_foreground(bg_tone1) || tone_prefers_light_foreground(bg_tone2);
    if prefers_light {
        return if light_option == -1.0 {
            100.0
        } else {
            light_option
        };
    }
    if let [only] = availables.as_slice() {
        return *only;
    }
    if dark_option == -1.0 {
        0.0
    } else {
        dark_option
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `java_round` rounds a half toward positive infinity, as Java's
    /// `Math.round` does, and a value below a half down.
    #[test]
    fn java_round_rounds_halves_up() {
        assert_eq!(java_round(59.5), 60.0);
        assert_eq!(java_round(-0.5), 0.0);
        assert_eq!(java_round(-1.5), -1.0);
        assert_eq!(java_round(0.49999999999999994), 0.0);
    }

    /// A role the scheme does not define, another variant and another
    /// version give no colour; a role it defines an opaque one.
    #[test]
    fn m3_role_answers_only_what_it_implements() {
        let seed = [0.25, 0.5, 0.75, 1.0];
        let role =
            |role, variant, version| m3_role(role, seed, variant, version, 0.0, Scheme::Light);
        assert!(
            role("primary", MaterialVariant::TonalSpot, "spec_2021").is_some_and(|c| c[3] == 1.0)
        );
        assert_eq!(
            role("accent", MaterialVariant::TonalSpot, "spec_2021"),
            None
        );
        assert_eq!(role("primary", MaterialVariant::Vibrant, "spec_2021"), None);
        assert_eq!(
            role("primary", MaterialVariant::TonalSpot, "spec_2025"),
            None
        );
    }
}
