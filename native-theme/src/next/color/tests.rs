use super::*;

type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

fn channels(c: Color) -> [f32; 4] {
    [c.r, c.g, c.b, c.a]
}

fn assert_close(actual: [f32; 4], expected: [f64; 4]) {
    let close = actual
        .iter()
        .zip(expected)
        .all(|(a, e)| (f64::from(*a) - e).abs() < 1e-5);
    assert!(close, "{actual:?} != {expected:?}");
}

#[test]
fn hex_forms_parse() -> TestResult {
    assert_eq!(
        Color::from_hex("#ff8000")?,
        Color::from_srgb8(255, 128, 0, 255)
    );
    assert_eq!(
        Color::from_hex("#FF800080")?,
        Color::from_srgb8(255, 128, 0, 128)
    );
    assert_eq!(
        Color::from_hex("#f80")?,
        Color::from_srgb8(255, 136, 0, 255)
    );
    assert_eq!(
        Color::from_hex("#F808")?,
        Color::from_srgb8(255, 136, 0, 136)
    );
    assert_eq!(
        "#3daee9".parse::<Color>()?,
        Color::from_srgb8(61, 174, 233, 255)
    );
    assert_eq!(" #fff ".parse::<Color>()?, Color::WHITE);
    Ok(())
}

#[test]
fn hex_rejections() {
    for refused in [
        "",
        "#",
        "fff",
        "ff8000",
        "#ff",
        "#fffff",
        "#fffffff",
        "#fffffffff",
        "#ggg",
        "#+ff",
        "#+fffff",
        "# ff0000",
        "#ff 000",
    ] {
        assert!(Color::from_hex(refused).is_err(), "{refused}");
        assert!(refused.parse::<Color>().is_err(), "{refused}");
    }
    assert!(matches!(
        Color::from_hex("#zz"),
        Err(Error::InvalidColor { input }) if input == "#zz"
    ));
}

#[test]
fn function_forms_parse() -> TestResult {
    assert_eq!(
        "srgb(0.25, 0.5, 1, 1)".parse::<Color>()?,
        Color::new(0.25, 0.5, 1.0, 1.0)
    );
    assert_eq!(
        "srgb(1.5,-0.25,0,0.5)".parse::<Color>()?,
        Color::new(1.5, -0.25, 0.0, 0.5)
    );
    assert_eq!(
        " srgb ( 0 , 0 , 0 , 0 ) ".parse::<Color>()?,
        Color::TRANSPARENT
    );
    assert_eq!(
        "p3(1, 0, 0, 0.5)".parse::<Color>()?,
        Color::from_display_p3(1.0, 0.0, 0.0, 0.5)
    );
    Ok(())
}

#[test]
fn function_rejections() {
    for refused in [
        "srgb(0.5, 0.5, 0.5)",
        "srgb(0.5, 0.5, 0.5, 0.5, 0.5)",
        "srgb(0.5, 0.5, 0.5, 1.5)",
        "srgb(0.5, 0.5, 0.5, -0.5)",
        "srgb(inf, 0.5, 0.5, 1)",
        "srgb(NaN, 0.5, 0.5, 1)",
        "srgb(0.5, 0.5, 0.5, NaN)",
        "srgb(a, 0.5, 0.5, 1)",
        "srgb(0.5, 0.5, 0.5, 1",
        "srgb 0.5, 0.5, 0.5, 1)",
        "rgb(0.5, 0.5, 0.5, 1)",
        "display_p3(0.5, 0.5, 0.5, 1)",
        "p3(0.5, 0.5, 1)",
        "sr gb(0.5, 0.5, 0.5, 1)",
    ] {
        assert!(refused.parse::<Color>().is_err(), "{refused}");
    }
}

#[test]
fn display_writes_hex_when_exact_and_srgb_otherwise() {
    assert_eq!(Color::from_srgb8(61, 174, 233, 255).to_string(), "#3daee9");
    assert_eq!(
        Color::from_srgb8(61, 174, 233, 128).to_string(),
        "#3daee980"
    );
    assert_eq!(
        Color::new(0.25, 1.5, -0.5, 1.0).to_string(),
        "srgb(0.25, 1.5, -0.5, 1)"
    );
    assert_eq!(
        Color::new(0.1, 0.0, 0.0, 1.0).to_string(),
        "srgb(0.1, 0, 0, 1)"
    );
}

#[test]
fn display_from_str_round_trip() -> TestResult {
    let colors = [
        Color::TRANSPARENT,
        Color::BLACK,
        Color::WHITE,
        Color::from_srgb8(1, 2, 3, 4),
        Color::new(0.1, 0.2, 0.3, 0.4),
        Color::new(1.093_066_4, -0.226_742, -0.150_134_6, 1.0),
        Color::from_display_p3(0.5, 0.25, 0.75, 0.5),
        Color::new(1e-7, 3.0e5, -0.0, 1.0),
    ];
    for c in colors {
        assert_eq!(c.to_string().parse::<Color>()?, c, "{c}");
    }
    Ok(())
}

#[test]
fn srgb8_round_trip_on_every_value() {
    for v in 0..=u8::MAX {
        let c = Color::from_srgb8(v, v, v, v);
        assert_eq!(c.to_srgb8(), [v; 4]);
        assert_eq!(Color::from_srgb8(v, 0, 0, 255).to_srgb8(), [v, 0, 0, 255]);
    }
}

#[test]
fn to_srgb8_clamps_then_rounds() {
    assert_eq!(
        Color::new(1.093, -0.2, 0.5, 1.0).to_srgb8(),
        [255, 0, 128, 255]
    );
    assert_eq!(
        Color::new(f32::NAN, f32::INFINITY, f32::NEG_INFINITY, 2.0).to_srgb8(),
        [0, 0, 0, 255]
    );
}

#[test]
fn premultiplied_srgb8_multiplies_then_clamps() {
    // 1.2 × 0.5 = 0.6 → 153; −0.17 × 0.5 → 0; 0.5 × 0.5 = 0.25 → 63.75 → 64.
    assert_eq!(
        Color::new(-0.17, 1.2, 0.5, 0.5).to_premultiplied_srgb8(),
        [0, 153, 64, 128]
    );
    assert_eq!(
        Color::new(0.2, 0.4, 0.6, 0.0).to_premultiplied_srgb8(),
        [0, 0, 0, 0]
    );
    assert_eq!(
        Color::new(f32::NAN, 1.0, 1.0, f32::NAN).to_premultiplied_srgb8(),
        [0, 0, 0, 0]
    );
}

#[test]
fn non_finite_rules() {
    let c = Color::new(0.5, 0.5, 0.5, 0.5);
    assert_eq!(c.with_alpha(f32::NAN).a, 0.0);
    assert_eq!(c.with_alpha(f32::INFINITY).a, 0.0);
    assert_eq!(c.with_alpha(1.5).a, 1.0);
    assert_eq!(c.with_alpha(-0.5).a, 0.0);
    assert_eq!(c.with_alpha(0.25).a, 0.25);
    assert_eq!(c.mul_alpha(f32::NAN).a, 0.0);
    assert_eq!(c.mul_alpha(f32::INFINITY).a, 0.0);
    assert_eq!(c.mul_alpha(0.5).a, 0.25);
    assert_eq!(c.mul_alpha(4.0).a, 1.0);
    assert_eq!(
        Color::new(f32::NAN, f32::INFINITY, -2.0, 1.5).clamped(),
        Color::new(0.0, 0.0, 0.0, 1.0)
    );
    assert_eq!(c.mix_srgb(Color::WHITE, f32::NAN), c);
}

#[test]
fn display_p3_conversions_match_css_color_4() {
    // Expected: CSS Color 4 conversions.js, gam_sRGB(XYZ_to_lin_sRGB(lin_P3_to_XYZ(lin_P3(c))))
    // and gam_P3(XYZ_to_lin_P3(lin_sRGB_to_XYZ(lin_sRGB(c)))), evaluated in Python with the
    // matrices as the published fractions, printed to seven decimals.
    let p3_to_srgb = [
        ([1.0, 0.0, 0.0], [1.093_066_4, -0.226_742, -0.150_134_6]),
        ([0.0, 1.0, 0.0], [-0.511_605, 1.018_265_7, -0.310_674_6]),
        ([0.0, 0.0, 1.0], [0.0, 0.0, 1.042_021_6]),
        ([1.0, 1.0, 1.0], [1.0, 1.0, 1.0]),
        ([0.5, 0.25, 0.75], [0.537_831_8, 0.232_131_9, 0.777_056_9]),
    ];
    for ([r, g, b], [er, eg, eb]) in p3_to_srgb {
        assert_close(
            channels(Color::from_display_p3(r, g, b, 0.5)),
            [er, eg, eb, 0.5],
        );
        assert_close(
            Color::from_display_p3(r, g, b, 1.0).to_display_p3(),
            [f64::from(r), f64::from(g), f64::from(b), 1.0],
        );
    }
    let srgb_to_p3 = [
        ([1.0, 0.0, 0.0], [0.917_487_6, 0.200_286_8, 0.138_560_6]),
        ([0.0, 1.0, 0.0], [0.458_401_6, 0.985_264_6, 0.298_294_7]),
        ([0.0, 0.0, 1.0], [0.0, 0.0, 0.959_588]),
    ];
    for ([r, g, b], [er, eg, eb]) in srgb_to_p3 {
        assert_close(Color::rgb(r, g, b).to_display_p3(), [er, eg, eb, 1.0]);
    }
}

#[test]
fn composite_over_is_source_over() {
    // Half red over half blue: αo = 0.5 + 0.5 × (1 − 0.5) = 0.75;
    // red = (1 × 0.5 + 0 × 0.5 × 0.5) / 0.75 = 2/3; blue = (0 + 1 × 0.5 × 0.5) / 0.75 = 1/3.
    let half_red = Color::new(1.0, 0.0, 0.0, 0.5);
    let half_blue = Color::new(0.0, 0.0, 1.0, 0.5);
    assert_close(
        channels(half_red.composite_over(half_blue)),
        [2.0 / 3.0, 0.0, 1.0 / 3.0, 0.75],
    );
    // Half black over white: αo = 0.5 + 1 × 0.5 = 1; each channel (0 + 1 × 1 × 0.5) / 1 = 0.5.
    assert_eq!(
        Color::new(0.0, 0.0, 0.0, 0.5).composite_over(Color::WHITE),
        Color::new(0.5, 0.5, 0.5, 1.0)
    );
    // An opaque source hides the backdrop; a transparent one shows it.
    let opaque = Color::new(0.1, 0.2, 0.3, 1.0);
    assert_eq!(opaque.composite_over(half_blue), opaque);
    assert_eq!(
        Color::new(0.1, 0.2, 0.3, 0.0).composite_over(half_blue),
        half_blue
    );
    // Both transparent: αo = 0, transparent black.
    assert_eq!(
        Color::new(1.0, 0.5, 0.2, 0.0).composite_over(Color::new(0.4, 0.6, 0.8, 0.0)),
        Color::TRANSPARENT
    );
}

#[test]
fn mix_srgb_is_premultiplied_color_mix() {
    // Half red and opaque blue at t = 0.5: premultiplied (0.5, 0, 0) and (0, 0, 1);
    // interpolated (0.25, 0, 0.5), α = 0.5 × 0.5 + 1 × 0.5 = 0.75;
    // un-premultiplied (0.25 / 0.75, 0, 0.5 / 0.75) = (1/3, 0, 2/3).
    let half_red = Color::new(1.0, 0.0, 0.0, 0.5);
    let blue = Color::new(0.0, 0.0, 1.0, 1.0);
    assert_close(
        channels(half_red.mix_srgb(blue, 0.5)),
        [1.0 / 3.0, 0.0, 2.0 / 3.0, 0.75],
    );
    // Opaque black and white at t = 0.25: 0 × 0.75 + 1 × 0.25 = 0.25 per channel.
    assert_eq!(
        Color::BLACK.mix_srgb(Color::WHITE, 0.25),
        Color::new(0.25, 0.25, 0.25, 1.0)
    );
    assert_eq!(half_red.mix_srgb(blue, 0.0), half_red);
    assert_eq!(half_red.mix_srgb(blue, 1.0), blue);
    assert_eq!(half_red.mix_srgb(blue, 7.0), blue);
    // Interpolated alpha 0: the premultiplied value, here transparent black.
    assert_eq!(
        Color::new(1.0, 1.0, 1.0, 0.0).mix_srgb(Color::new(0.5, 0.5, 0.5, 0.0), 0.5),
        Color::TRANSPARENT
    );
}

#[test]
fn luminance_and_contrast() {
    assert_eq!(Color::BLACK.contrast_ratio(Color::WHITE), 21.0);
    assert_eq!(Color::WHITE.contrast_ratio(Color::BLACK), 21.0);
    assert_eq!(Color::WHITE.relative_luminance(), 1.0);
    assert_eq!(Color::BLACK.relative_luminance(), 0.0);
    let gray = Color::rgb(0.5, 0.5, 0.5);
    assert_eq!(gray.contrast_ratio(gray), 1.0);
    // An out-of-gamut channel below zero is clamped at the sum: luminance >= 0.
    assert_eq!(Color::rgb(-1.0, -1.0, -1.0).relative_luminance(), 0.0);
    assert_eq!(Color::rgb(f32::NAN, 0.0, 0.0).relative_luminance(), 0.0);
    assert!(
        Color::rgb(f32::INFINITY, 0.0, 0.0).contrast_ratio(Color::rgb(f32::INFINITY, 0.0, 0.0))
            >= 1.0
    );
}

fn assert_near(actual: f32, expected: f64) {
    assert!(
        (f64::from(actual) - expected).abs() < 1e-6,
        "{actual} != {expected}"
    );
}

#[test]
fn luminance_vectors_pin_the_coefficients_and_the_threshold() {
    // Expected values from
    // python3 -I -c "lin=lambda v: v/12.92 if abs(v)<=0.04045 else ((abs(v)+0.055)/1.055)**2.4;
    //   L=lambda r,g,b: 0.2126*lin(r)+0.7152*lin(g)+0.0722*lin(b); ..."
    // Mid grey, power segment: ((0.5 + 0.055) / 1.055)^2.4 = 0.2140411.
    assert_near(Color::rgb(0.5, 0.5, 0.5).relative_luminance(), 0.214_041_14);
    // #3daee9 (61, 174, 233): L = 0.3714740; against white (1.05) / (L + 0.05) = 2.491257.
    let breeze_blue = Color::from_srgb8(61, 174, 233, 255);
    assert_near(breeze_blue.relative_luminance(), 0.371_473_96);
    assert_near(breeze_blue.contrast_ratio(Color::WHITE), 2.491_257_1);
    // Linear segment (0.04 <= 0.04045): green alone, 0.7152 × 0.04 / 12.92 = 0.002214241.
    assert_near(
        Color::rgb(0.0, 0.04, 0.0).relative_luminance(),
        0.002_214_241_5,
    );
}

#[test]
fn luminance_is_finite_after_the_cast() {
    // f32::MAX gives about 5.6e91 in f64, beyond f32: not finite as an f32, so 0.0.
    let huge = Color::rgb(f32::MAX, 0.0, 0.0).relative_luminance();
    assert!(huge.is_finite());
    assert_eq!(huge, 0.0);
    let ratio = Color::rgb(f32::MAX, 0.0, 0.0).contrast_ratio(Color::WHITE);
    assert!(ratio.is_finite() && ratio >= 1.0);
}

#[test]
fn a_term_of_weight_zero_contributes_nothing() {
    let hidden = Color::new(f32::INFINITY, f32::NEG_INFINITY, f32::INFINITY, 0.0);
    let half_blue = Color::new(0.0, 0.0, 1.0, 0.5);
    // Transparent source over a backdrop: the backdrop.
    assert_eq!(hidden.composite_over(half_blue), half_blue);
    // An opaque source over a transparent backdrop: the source.
    let opaque = Color::new(0.1, 0.2, 0.3, 1.0);
    assert_eq!(opaque.composite_over(hidden), opaque);
    // An opaque source hides a backdrop of any channels: weight (1 − 1) × 1 = 0.
    let wild_backdrop = Color::new(f32::INFINITY, 0.0, 0.0, 1.0);
    assert_eq!(opaque.composite_over(wild_backdrop), opaque);
    // color-mix: t = 0 and t = 1 leave the other colour out; a transparent colour adds nothing.
    assert_eq!(opaque.mix_srgb(wild_backdrop, 0.0), opaque);
    assert_eq!(wild_backdrop.mix_srgb(opaque, 1.0), opaque);
    // Weights 0 and 0.5: alpha 0.5, channels 0.1 × 0.5 / 0.5 and so on.
    let mixed = hidden.mix_srgb(opaque, 0.5);
    assert!(
        [mixed.r, mixed.g, mixed.b, mixed.a]
            .iter()
            .all(|c| !c.is_nan())
    );
    assert_close(channels(mixed), [0.1, 0.2, 0.3, 0.5]);
}
