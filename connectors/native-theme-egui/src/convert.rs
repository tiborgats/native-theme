//! Total, panic-free conversions from native-theme values to epaint types.
//!
//! Every function is defined for **all** `f32` bit patterns including `NaN`, `±∞`, subnormals
//! and `-0.0`. None can panic, none allocates, none uses `unsafe`, and every `as` cast is
//! provably exact because the preceding steps constrain the operand to the destination range.
//!
//! # One non-finite rule
//!
//! A theme value that is `NaN` or `±∞` is replaced by **egui's own value for the sink it
//! feeds** — `finite_or(x, <that value>)` — and never by `0.0` unless `0.0` is that value;
//! the call site records a [`crate::Note::ValueSanitised`]. Every sink conversion that can know
//! egui's value takes it as a `base` parameter and applies the rule itself
//! ([`to_corner_radius`], [`to_margin`], [`to_stroke`]). Every layout length that survives
//! the rule then passes [`clamp_length`]; a finite value merely clamped into range is not a
//! substitution and records nothing.
//!
//! This matters because native-theme range-checks a per-widget border's padding sides only
//! (`native-theme-derive/src/gen_ranges.rs:127-141`, `check_padding` at
//! `native-theme/src/resolve/validate_helpers.rs:458-485`), never its `corner_radius` or
//! `line_width`, and [`crate::ResolvedTheme`] derives `Deserialize` with public fields
//! (`native-theme/src/model/resolved.rs:155-156`), which bypasses validation altogether — so a
//! hostile or buggy input really can put a non-finite number in
//! `theme.button.border.corner_radius`, or in a padding side.

/// Re-exported **here and not at the crate root**, because `egui::Rgba`
/// (`egui/src/lib.rs:442` -> `ecolor/src/rgba.rs:3-10`) is *linear f32, premultiplied* while
/// this one is *sRGB u8, straight* (`native-theme/src/color.rs:8-11`, `:43-52`), and this crate
/// re-exports `egui`.
pub use native_theme::color::Rgba;

/// Map `NaN` to `0.0`; leave every other value, including `±∞`, untouched.
///
/// The first step of [`u8_from_f32_saturating`], [`i8_from_f32_saturating`],
/// [`unit_interval`] and [`clamp_length`], which must be total: the one place a `NaN` is
/// folded *inside* a conversion. It is not what a theme `NaN` becomes at a sink — that is
/// [`finite_or`] with egui's own value, and the call site reports that substitution; `denan`
/// itself reports nothing. `f32::clamp` is banned: it *propagates* `NaN` and panics when
/// `min > max` or a bound is `NaN`. Bare `.max()`/`.min()` are order-dependent under `NaN` —
/// `NAN.max(lo).min(hi) == lo` but `NAN.min(hi).max(lo) == hi` — so `denan` runs first, every
/// time.
#[inline]
#[must_use]
pub(crate) const fn denan(v: f32) -> f32 {
    if v.is_nan() { 0.0 } else { v }
}

/// Saturating, rounding `f32` -> `u8`, for `epaint::CornerRadius` (four `u8`,
/// `epaint/src/corner_radius.rs:13-25`) and the alpha byte of [`to_color32_with_opacity`].
/// `epaint::Shadow::{blur, spread}` (`epaint/src/shadow.rs:20`, `:23`) are cited for their
/// representable range only — this crate never writes shadow geometry (§6.14).
///
/// `NaN` -> 0; `v <= 0.0` -> 0; `v >= 255.0` -> 255; otherwise round-half-away-from-zero.
/// `f32::round`, not `round_ties_even`, so a value this crate converts and a value epaint
/// converts through `impl From<f32> for CornerRadius` (`epaint/src/corner_radius.rs:42-44`)
/// agree bit for bit. After `denan` + `max` + `min` the operand is finite and in
/// `0.0..=255.0`, and `.round()` keeps it there, so the cast is exact.
#[inline]
#[must_use]
#[allow(
    clippy::manual_clamp,
    reason = "f32::clamp is banned: denan runs first, in a fixed order"
)]
pub fn u8_from_f32_saturating(v: f32) -> u8 {
    denan(v).max(0.0).min(255.0).round() as u8
}

/// Saturating, rounding `f32` -> `i8`, for `epaint::Margin` (four `i8`,
/// `epaint/src/margin.rs:15-20`) — the only `i8` sink this crate writes from theme data.
/// `epaint::Shadow::offset` (`epaint/src/shadow.rs:15`) is cited for its representable
/// range only; this crate never writes shadow geometry (§6.14).
///
/// `NaN` -> 0, **not** −128, because [`denan`] runs first: `f32::NAN.max(-128.0)` is
/// `-128.0`, so a `NaN` margin would otherwise become the most-negative margin.
/// `v <= -128.0` -> −128; `v >= 127.0` -> 127; otherwise round-half-away-from-zero.
/// Negative values are preserved rather than clamped to zero — egui itself computes a
/// negative outer margin from `expansion` (`egui/src/widget_style.rs:162`,
/// `egui/src/widgets/text_edit/builder.rs:767`).
///
/// Saturation here is a **real** loss of theme data and emits a
/// [`crate::Note::ValueSaturated`] at the call site, which detects it itself (§7.2's
/// emission rule); the helper stays pure and single-valued.
#[inline]
#[must_use]
#[allow(
    clippy::manual_clamp,
    reason = "f32::clamp is banned: denan runs first, in a fixed order"
)]
pub(crate) fn i8_from_f32_saturating(v: f32) -> i8 {
    denan(v).max(-128.0).min(127.0).round() as i8
}

/// Pass a finite `f32` through; substitute `fallback` for `NaN` and `±∞`.
///
/// The crate's one non-finite rule (§6): `fallback` is **egui's own value for the sink** —
/// the value the starting `Style` carries there — so a non-finite theme value leaves egui's
/// look in place instead of inventing one, and it is never `0.0` unless that is egui's
/// value. A substitution made here is **reported**: the call site emits
/// [`crate::Note::ValueSanitised`] for that leaf. `finite_or` already maps `NaN`, so
/// `finite_or(denan(x), fallback)` is never written.
#[inline]
#[must_use]
pub fn finite_or(v: f32, fallback: f32) -> f32 {
    if v.is_finite() { v } else { fallback }
}

/// The largest `f32` strictly below `f32::MAX * GUI_ROUNDING` (≈ `1.0633823e37`).
///
/// `Layout::next_frame` rounds every edge with `round_ui`, `(x / GUI_ROUNDING).round() *
/// GUI_ROUNDING` (`egui/src/layout.rs:635`, `emath/src/gui_rounding.rs:59`), which overflows
/// to `+∞` above this product and trips egui's debug asserts (§6.7); `GUI_ROUNDING` is
/// `1.0 / 32.0`, `pub` at `egui::emath::GUI_ROUNDING` (`emath/src/gui_rounding.rs:18`,
/// re-exported at `emath/src/lib.rs:47` and `egui/src/lib.rs:438`). The product with a power
/// of two is exact, so `next_down` lands on the neighbouring `f32`; float arithmetic and
/// `f32::next_down` are `const` on Rust 1.88.0 already, below the crate's MSRV (§12.4). One
/// step below the product, a lone widget or a solid scrollbar groove inside a `Window`
/// completes its passes (§6.5, §6.7).
const LENGTH_CEILING: f32 = (f32::MAX * egui::emath::GUI_ROUNDING).next_down();

/// Clamp one layout length into `0.0..=LENGTH_CEILING`, total over all `f32`.
///
/// Every length this crate writes into a `Style` or a `Frame` — a `Spacing` field, a stroke
/// width, a `button_padding` component — passes here **after** [`finite_or`] has applied the
/// non-finite rule, so its `NaN` image (`0.0`) is a totality backstop, never the documented
/// fallback. `−∞` floors to `0.0`; `+∞` — a finite theme value the formula's own arithmetic
/// overflowed — caps at the ceiling. A value merely clamped is not sanitised and emits
/// nothing. The cap bounds **one** length: an edge is the sum of every length laid out
/// before it, so two stacked widgets each just under the ceiling can still reach egui's
/// rounding edge; no per-value bound can prevent that, and it stays a stated residual (§6.7).
#[inline]
#[must_use]
#[allow(
    clippy::manual_clamp,
    reason = "f32::clamp is banned: denan runs first, in a fixed order"
)]
pub fn clamp_length(v: f32) -> f32 {
    denan(v).max(0.0).min(LENGTH_CEILING)
}

/// Clamp an opacity into `0.0..=1.0`, total over all `f32`.
///
/// Mandatory before anything reaches `Visuals::disabled_alpha` (`egui/src/style.rs:1126`):
/// `Visuals::disable` (`:1177-1180`) forwards it to `Color32::gamma_multiply`, which carries
/// `debug_assert!(0.0 <= factor && factor.is_finite(), ..)` (`ecolor/src/color32.rs:270-273`);
/// its only caller in egui is `Ui::dnd_drop_zone` (`egui/src/ui.rs:2726-2727`), while
/// `Ui::disable` does not reach it (§7.4). A **non-finite** input is a substitution and is
/// reported as [`crate::Note::ValueSanitised`] at the call site; a finite value merely
/// clamped into range is not, and emits nothing.
#[inline]
#[must_use]
#[allow(
    clippy::manual_clamp,
    reason = "f32::clamp is banned: denan runs first, in a fixed order"
)]
pub fn unit_interval(v: f32) -> f32 {
    denan(v).max(0.0).min(1.0)
}

/// sRGB straight-alpha [`Rgba`] -> `egui::Color32` (sRGB premultiplied,
/// `ecolor/src/color32.rs:31`).
///
/// `const`, allocation-free, total over all 2^32 inputs. It uses
/// `Color32::from_rgba_unmultiplied_const` (`ecolor/src/color32.rs:139`), which premultiplies
/// in **gamma** space, applying no sRGB transfer function: exact at `a == 0` (`:142`) and
/// `a == 255` (`:145`), and at `1..=254` an integer `mul_frac_round(channel, a)` per channel
/// (`:147-151`, `ecolor/src/lib.rs:137-146`) that rounds exactly as the non-`const`
/// `Color32::from_rgba_unmultiplied` does, since that one calls this one
/// (`ecolor/src/color32.rs:133-135`). Do **not** route through `ecolor::Rgba`: feeding it sRGB
/// floats double-encodes gamma (§7.3).
#[inline]
#[must_use]
pub const fn to_color32(c: Rgba) -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied_const(c.r, c.g, c.b, c.a)
}

/// [`to_color32`] with an opacity multiplier folded into alpha.
///
/// Not for a border: every border colour the model states is the final line colour, with
/// `defaults.border.opacity` already folded in (`native-theme/src/model/border.rs`,
/// `DefaultsBorderSpec::opacity`), and the connector paints it as stated. A
/// non-finite `opacity` is `1.0` (no fold), per the non-finite rule; the call site reports
/// it. Never use the result for
/// `WidgetVisuals::bg_fill`, documented "Must never be `Color32::TRANSPARENT`"
/// (`egui/src/style.rs:1292-1294`); `opacity == 0.0` produces exactly that.
#[inline]
#[must_use]
pub fn to_color32_with_opacity(c: Rgba, opacity: f32) -> egui::Color32 {
    let a = u8_from_f32_saturating(f32::from(c.a) * unit_interval(finite_or(opacity, 1.0)));
    egui::Color32::from_rgba_unmultiplied_const(c.r, c.g, c.b, a)
}

/// Uniform `epaint::CornerRadius` from one logical-pixel length, over the sink's own radius.
///
/// A non-finite `radius_px` keeps `base` — the radius the starting `Style` has in that sink —
/// per the non-finite rule; the call site reports it. Saturation at 255 is provably benign:
/// the tessellator re-clamps a corner radius to half the smaller side
/// (`epaint/src/tessellator.rs:638-642`), so 255 simply reads as "pill". No `Note` is
/// emitted for saturation.
#[inline]
#[must_use]
pub fn to_corner_radius(base: egui::CornerRadius, radius_px: f32) -> egui::CornerRadius {
    if radius_px.is_finite() {
        egui::CornerRadius::same(u8_from_f32_saturating(radius_px))
    } else {
        base
    }
}

/// `epaint::Margin` from a widget's per-side padding, over the sink's own margin.
///
/// `ResolvedPadding` has one `Option<f32>` per side (`native-theme/src/model/border.rs:195-204`),
/// each a **per-side** value (`docs/platform-facts.md:910-926`) — do **not** halve. A stated,
/// finite side (`Some(0.0)` included) is rounded to a whole point, saturating; an unstated
/// side (`None`) and, by the non-finite rule, a `NaN` or `±∞` one keep `base`'s side, where
/// `base` is the value egui's own default `Style` for that scheme gives the sink (S1) —
/// `Spacing::window_margin`'s `Margin::same(6)` (`egui/src/style.rs:1456`), a panel frame's
/// `Margin::symmetric(8, 2)` (`egui/src/containers/frame.rs:187`) — never an invented `0`.
/// `base` is a parameter because only the call site knows the sink, as in iced's
/// `padding_or` (`connectors/native-theme-iced/src/lib.rs:329-339`). The call site reports a
/// non-finite side, and a saturated one.
#[inline]
#[must_use]
pub fn to_margin(
    base: egui::Margin,
    padding: &native_theme::theme::ResolvedPadding,
) -> egui::Margin {
    let side = |stated: Option<f32>, own: i8| {
        stated
            .filter(|v| v.is_finite())
            .map_or(own, i8_from_f32_saturating)
    };
    egui::Margin {
        left: side(padding.left, base.left),
        right: side(padding.right, base.right),
        top: side(padding.top, base.top),
        bottom: side(padding.bottom, base.bottom),
    }
}

/// A widget border's padding measured from the outside of its border: each stated, finite
/// side plus the border's line width.
///
/// A platform's padding lies *inside* its border — outer size = border + padding + content
/// (`docs/platform-facts.md:900-902`) — while `Button`, `ComboBox`, `CollapsingHeader` and
/// `TextEdit` draw their stroke *inside* the padding egui gives them: `Button`'s frame margin
/// is `button_padding + expansion − bg_stroke.width` and `Frame` adds the stroke back
/// (`egui/src/widget_style.rs:163-165`, `egui/src/containers/frame.rs:327-331`),
/// `button_frame` strokes `StrokeKind::Inside` within `content ± button_padding`
/// (`egui/src/containers/combo_box.rs:439-460`), the header likewise
/// (`egui/src/containers/collapsing_header.rs:531`, `:566-567`), and `TextEdit` allocates with
/// its margin alone and paints with `margin + expansion − stroke.width`
/// (`egui/src/widgets/text_edit/builder.rs:713`, `:763-767`). Written into those sinks, this
/// padding makes the native outer size exact up to the sinks' whole-point rounding: a
/// `Margin` sink holds `round(side + width)`, and a `Button` realises
/// `round(button_padding − width) + width` per side (`egui/src/widget_style.rs:163-165`,
/// `epaint/src/margin.rs:117-119`), so a fractional padding or width moves an edge by at
/// most half a point. It is exact only where the preset's padding is the platform's
/// inside-the-border padding: `macos-sonoma`'s vertical button padding is derived as outer
/// minus content, `(22 − 16) / 2` (`docs/platform-facts.md:1172`), which already contains
/// the border, so there the button comes out one line width per side taller than the
/// platform's until platform-facts corrects that derivation (`docs/todo.md`).
///
/// The width is `clamp_length(finite_or(line_width, 0.0))`: a non-finite width adds nothing
/// — egui's own padding already holds egui's own stroke, so adding nothing is egui's value
/// there — and a negative one is `0.0`; the call site reports a non-finite width. A finite
/// side plus a finite width can only overflow upwards, and `.min(f32::MAX)` keeps that sum
/// finite, so [`to_margin`] saturates it rather than mistaking it for a non-finite side. A
/// `None` or non-finite side stays `None`, which every consumer resolves to egui's own value.
#[inline]
#[must_use]
pub(crate) fn padding_with_border(
    border: &native_theme::theme::ResolvedWidgetBorder,
) -> native_theme::theme::ResolvedPadding {
    let width = clamp_length(finite_or(border.line_width, 0.0));
    let side = |stated: Option<f32>| {
        stated
            .filter(|v| v.is_finite())
            .map(|v| (v + width).min(f32::MAX))
    };
    native_theme::theme::ResolvedPadding {
        top: side(border.padding.top),
        right: side(border.padding.right),
        bottom: side(border.padding.bottom),
        left: side(border.padding.left),
    }
}

/// `Spacing::button_padding` from a widget's resolved border, under §5's pair rule.
///
/// `button_padding` is one `Vec2` (`egui/src/style.rs:398`) that `Button`, `ComboBox` and
/// `CollapsingHeader` apply symmetrically, so left and right share `.x` and top and bottom
/// share `.y`. Each component is the mean of its pair's two targets: a side of
/// [`padding_with_border`] that is `Some` — the stated side plus the border's line width — is
/// its own target, and a `None` side targets `base`'s component — the starting style's: egui's own default
/// (`vec2(4.0, 1.0)`, `style.rs:1458`) on the base style, the base style's in a role cell — exactly as [`to_margin`] keeps `base`'s side.
/// With both sides stated the component is `(a + b) / 2 + line_width`: the one symmetric value
/// that keeps the native outer size and moves the content off-centre by the least,
/// `|a − b| / 2` (§5, §14 item 32). Filtering non-finite sides in [`padding_with_border`] is
/// what keeps a `NaN` out of the `Vec2` (§7.4). `f32::midpoint` (stable since Rust 1.85)
/// never overflows for finite operands, returns an equal pair bit for bit, and equals
/// `(a + b) / 2.0` wherever that is finite. `button_padding` is `f32`, so no integer
/// saturation is needed here; [`clamp_length`] supplies the `0.0` floor (§6.17) and the layout
/// cap. The expander reads `.x` on its trailing side only
/// (`egui/src/containers/collapsing_header.rs:526`), so its call site takes `.y` from this
/// function and writes `.x` from `padding_with_border(border).right` alone (§7.4).
#[inline]
#[must_use]
pub(crate) fn to_button_padding(
    base: egui::Vec2,
    border: &native_theme::theme::ResolvedWidgetBorder,
) -> egui::Vec2 {
    let padding = padding_with_border(border);
    let pair = |a: Option<f32>, b: Option<f32>, own: f32| {
        clamp_length(f32::midpoint(a.unwrap_or(own), b.unwrap_or(own)))
    };
    egui::vec2(
        pair(padding.left, padding.right, base.x),
        pair(padding.top, padding.bottom, base.y),
    )
}

/// A stated hover or pressed fill composited over the widget's idle fill (C17, §6.1).
///
/// Source-over in premultiplied gamma space, `layer + idle · (1 − layer.a)`, done by
/// ecolor's own `Color32::blend` (`ecolor/src/color32.rs:343-345`): an integer
/// `gamma_multiply_u8` (`:289-298`) plus a saturating add, so it is total, panic-free and
/// divides by no runtime value. An opaque layer returns itself exactly and an `a == 0` layer
/// returns the idle fill exactly. Gamma space rather than linear light, as iced's composite
/// argues (`connectors/native-theme-iced/src/styles.rs:74-78`). The result is rounded to
/// `u8` in premultiplied space, so where the layer is translucent it can differ in rounding
/// from a straight-alpha `f32` composite such as iced's `composite_over`.
#[inline]
#[must_use]
pub fn composite_over(layer: Rgba, idle: Rgba) -> egui::Color32 {
    to_color32(idle).blend(to_color32(layer))
}

/// `epaint::Stroke` from a native colour and a line width, over the sink's own stroke.
///
/// The width is `clamp_length(finite_or(line_width_px, base.width))`: a non-finite width keeps
/// `base.width` — the width the starting `Style` has in that sink — per the non-finite rule
/// (the call site reports it), a negative one is `0.0`, and a huge one is capped. That
/// matters because `Stroke::is_empty` is `width <= 0.0 || color == TRANSPARENT`
/// (`epaint/src/stroke.rs:35-37`) and `NaN <= 0.0` is `false`, so a `NaN` width would not be
/// filtered and would reach the tessellator. Only the width falls back; the colour is always
/// the theme's.
#[inline]
#[must_use]
pub fn to_stroke(base: egui::Stroke, color: Rgba, line_width_px: f32) -> egui::Stroke {
    egui::Stroke::new(
        clamp_length(finite_or(line_width_px, base.width)),
        to_color32(color),
    )
}

/// `epaint::Shadow` from a native shadow colour, keeping `base`'s geometry. See §6.14.
///
/// native-theme carries **no** shadow geometry — only `shadow_enabled: bool`
/// (`native-theme/src/model/border.rs:224`, `:241`) — while `epaint::Shadow` needs
/// `offset: [i8; 2]` (`epaint/src/shadow.rs:15`), `blur: u8` (`:20`) and `spread: u8` (`:23`).
/// Only the colour is replaced; `enabled == false` yields `Shadow::NONE`
/// (`epaint/src/shadow.rs:40-45`).
#[inline]
#[must_use]
pub(crate) fn to_shadow(base: egui::Shadow, color: Rgba, enabled: bool) -> egui::Shadow {
    if enabled {
        egui::Shadow {
            color: to_color32(color),
            ..base
        }
    } else {
        egui::Shadow::NONE
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a test fails by panicking"
)]
mod tests {
    use super::*;
    use native_theme::theme::{ColorMode, ResolvedPadding, ResolvedWidgetBorder, Theme};
    use std::collections::HashSet;

    fn border(line_width: f32, sides: [Option<f32>; 4]) -> ResolvedWidgetBorder {
        ResolvedWidgetBorder {
            color: Rgba::rgb(1, 2, 3),
            corner_radius: 4.0,
            line_width,
            shadow_enabled: false,
            padding: ResolvedPadding {
                top: sides[0],
                right: sides[1],
                bottom: sides[2],
                left: sides[3],
            },
        }
    }

    const NON_FINITE: [f32; 3] = [f32::NAN, f32::INFINITY, f32::NEG_INFINITY];

    /// T17: `finite_or` passes every finite value through bit for bit and substitutes only
    /// for `NaN` and `±∞`.
    #[test]
    fn finite_or_passes_finite_values_and_substitutes_non_finite_ones() {
        for v in [
            -0.0,
            0.0,
            f32::MIN_POSITIVE,
            f32::MAX,
            -f32::MAX,
            1.5,
            1e30,
            -1e30,
        ] {
            assert_eq!(finite_or(v, 7.0).to_bits(), v.to_bits(), "{v}");
        }
        for v in NON_FINITE {
            assert_eq!(finite_or(v, 7.0), 7.0, "{v}");
        }
    }

    /// T17: `clamp_length` is the identity from `0.0` up to below `f32::MAX * GUI_ROUNDING`,
    /// `0.0` for every negative value and for `NaN`, and finite after `round_ui` at the top.
    #[test]
    fn clamp_length_floors_at_zero_and_stays_below_eguis_rounding_edge() {
        use egui::emath::GuiRounding as _;
        let below_edge = (f32::MAX * egui::emath::GUI_ROUNDING).next_down();
        for v in [0.0, 0.5, 1.0, 12.5, 1e6, below_edge] {
            assert_eq!(clamp_length(v), v, "{v}");
        }
        for v in [-1.0, -12.5, -f32::MAX, f32::NEG_INFINITY, f32::NAN] {
            assert_eq!(clamp_length(v), 0.0, "{v}");
        }
        for v in [f32::MAX, f32::INFINITY] {
            assert!(clamp_length(v).round_ui().is_finite(), "{v}");
        }
    }

    /// T17: the saturating casts and the opacity clamp are total, `NaN` going to `0`.
    #[test]
    fn saturating_casts_and_unit_interval_are_total() {
        assert_eq!(u8_from_f32_saturating(f32::NAN), 0);
        assert_eq!(u8_from_f32_saturating(-1.0), 0);
        assert_eq!(u8_from_f32_saturating(2.5), 3); // round half away from zero
        assert_eq!(u8_from_f32_saturating(255.5), 255);
        assert_eq!(u8_from_f32_saturating(f32::INFINITY), 255);
        assert_eq!(i8_from_f32_saturating(f32::NAN), 0);
        assert_eq!(i8_from_f32_saturating(-0.5), -1);
        assert_eq!(i8_from_f32_saturating(-200.0), -128);
        assert_eq!(i8_from_f32_saturating(200.0), 127);
        assert_eq!(i8_from_f32_saturating(f32::NEG_INFINITY), -128);
        assert_eq!(unit_interval(f32::NAN), 0.0);
        assert_eq!(unit_interval(-3.0), 0.0);
        assert_eq!(unit_interval(0.25), 0.25);
        assert_eq!(unit_interval(f32::INFINITY), 1.0);
    }

    /// T17: a `NaN` or `±∞` side, width or radius keeps `base`'s value in that sink.
    #[test]
    fn margins_paddings_strokes_and_radii_keep_the_base_for_non_finite_input() {
        let base_margin = egui::Margin::symmetric(8, 2);
        let base_padding = egui::vec2(4.0, 1.0);
        let base_stroke = egui::Stroke::new(1.0, egui::Color32::RED);
        let base_radius = egui::CornerRadius::same(3);
        for v in NON_FINITE {
            let m = to_margin(
                base_margin,
                &ResolvedPadding {
                    top: Some(v),
                    right: Some(1.0),
                    bottom: None,
                    left: Some(v),
                },
            );
            assert_eq!(m.top, base_margin.top, "{v}");
            assert_eq!(m.left, base_margin.left, "{v}");
            assert_eq!(m.right, 1, "{v}");
            assert_eq!(m.bottom, base_margin.bottom, "{v}");
            assert_eq!(
                to_button_padding(base_padding, &border(1.0, [Some(v); 4])),
                base_padding,
                "{v}"
            );
            let s = to_stroke(base_stroke, Rgba::rgb(0, 0, 255), v);
            assert_eq!(s.width, base_stroke.width, "{v}");
            assert_eq!(s.color, egui::Color32::from_rgb(0, 0, 255), "{v}");
            assert_eq!(to_corner_radius(base_radius, v), base_radius, "{v}");
        }
        assert_eq!(
            to_stroke(base_stroke, Rgba::rgb(0, 0, 255), -2.0).width,
            0.0
        );
        assert_eq!(
            to_corner_radius(base_radius, 6.4),
            egui::CornerRadius::same(6)
        );
        let m = to_margin(
            base_margin,
            &ResolvedPadding {
                top: Some(0.0),
                right: Some(300.0),
                bottom: Some(-300.0),
                left: None,
            },
        );
        assert_eq!(
            (m.top, m.right, m.bottom, m.left),
            (0, 127, -128, base_margin.left)
        );
    }

    /// T17: `padding_with_border` adds the line width to each stated finite side, adds nothing
    /// for a non-finite width, keeps `f32::MAX + width` finite, and `to_button_padding` of
    /// line width `1.0` with sides 5 / 6 gives `.y = 6.5`.
    #[test]
    fn padding_with_border_adds_the_width_to_stated_finite_sides_only() {
        let p = padding_with_border(&border(
            1.0,
            [Some(5.0), None, Some(f32::MAX), Some(f32::NAN)],
        ));
        assert_eq!(p.top, Some(6.0));
        assert_eq!(p.right, None);
        assert!(p.bottom.unwrap().is_finite());
        assert_eq!(p.left, None);
        for w in NON_FINITE {
            let p = padding_with_border(&border(w, [Some(5.0); 4]));
            assert_eq!(
                p,
                ResolvedPadding {
                    top: Some(5.0),
                    right: Some(5.0),
                    bottom: Some(5.0),
                    left: Some(5.0)
                },
                "{w}"
            );
        }
        let p = to_button_padding(
            egui::vec2(4.0, 1.0),
            &border(1.0, [Some(5.0), Some(6.0), Some(6.0), Some(5.0)]),
        );
        assert_eq!(p.y, 6.5);
        assert_eq!(p.x, 6.5);
    }

    /// T17: `to_color32` premultiplies in gamma space with no transfer function; the
    /// opacity fold multiplies alpha and treats a non-finite opacity as `1.0`.
    #[test]
    fn colours_convert_in_gamma_space_and_the_opacity_folds_into_alpha() {
        assert_eq!(
            to_color32(Rgba::rgb(255, 0, 0)),
            egui::Color32::from_rgb(255, 0, 0)
        );
        assert_eq!(to_color32(Rgba::TRANSPARENT), egui::Color32::TRANSPARENT);
        let half = Rgba {
            r: 200,
            g: 100,
            b: 50,
            a: 128,
        };
        assert_eq!(
            to_color32(half),
            egui::Color32::from_rgba_unmultiplied(200, 100, 50, 128)
        );
        assert_eq!(
            to_color32_with_opacity(Rgba::rgb(10, 20, 30), 0.5),
            egui::Color32::from_rgba_unmultiplied(10, 20, 30, 128)
        );
        assert_eq!(
            to_color32_with_opacity(Rgba::rgb(10, 20, 30), 0.0),
            egui::Color32::TRANSPARENT
        );
        for o in NON_FINITE {
            assert_eq!(
                to_color32_with_opacity(Rgba::rgb(10, 20, 30), o),
                egui::Color32::from_rgb(10, 20, 30),
                "{o}"
            );
        }
    }

    fn collect_colours(v: &serde_json::Value, out: &mut HashSet<Rgba>) {
        match v {
            serde_json::Value::String(s) => {
                if let Ok(c) = s.parse::<Rgba>() {
                    out.insert(c);
                }
            }
            serde_json::Value::Object(m) => m.values().for_each(|v| collect_colours(v, out)),
            serde_json::Value::Array(a) => a.iter().for_each(|v| collect_colours(v, out)),
            _ => {}
        }
    }

    /// T17: `composite_over` returns an opaque layer exactly and, for an `a == 0` layer, the
    /// idle fill exactly, over every colour the sixteen presets state in both modes plus the
    /// alpha edges (`0`, `1`, `254`, `255`) no preset states.
    #[test]
    fn composite_over_returns_an_opaque_layer_and_a_transparent_layers_idle_fill_exactly() {
        let mut colours = HashSet::new();
        for info in Theme::list_presets() {
            for mode in [ColorMode::Light, ColorMode::Dark] {
                let resolved = Theme::preset(info.key)
                    .unwrap()
                    .into_variant(mode)
                    .unwrap()
                    .resolve_system()
                    .unwrap();
                collect_colours(&serde_json::to_value(&resolved).unwrap(), &mut colours);
            }
        }
        for c in [
            Rgba::TRANSPARENT,
            Rgba::BLACK,
            Rgba::WHITE,
            Rgba {
                r: 9,
                g: 8,
                b: 7,
                a: 1,
            },
            Rgba {
                r: 250,
                g: 20,
                b: 2,
                a: 254,
            },
        ] {
            colours.insert(c);
        }
        assert!(
            colours.len() > 32,
            "the presets state {} distinct colours",
            colours.len()
        );
        for &layer in &colours {
            for &idle in &colours {
                let over = composite_over(layer, idle);
                if layer.a == 255 {
                    assert_eq!(over, to_color32(layer), "opaque {layer:?} over {idle:?}");
                }
                if layer.a == 0 {
                    assert_eq!(over, to_color32(idle), "clear {layer:?} over {idle:?}");
                }
            }
        }
    }
}
