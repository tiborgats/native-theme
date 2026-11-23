//! A stroke as the mapping writes it over a fill (`W/M2/fill-stroke-decision.md`, the egui
//! row): egui's tessellator ends a rect's fill at the stroke's inner edge (`epaint` 0.37.0
//! `src/tessellator.rs`, `stroke_and_fill_path`), the rect drawn with `StrokeKind::Inside` in
//! every alignment, so a stroke the theme lays `Outside` the box is drawn as stated. One it lays
//! `Inside`, over the fill, is handed to egui composited over the fill that state paints; one
//! `Center`ed on the edge is composited over that fill at half its alpha, the band's mean
//! (premultiplied `s·a_s + (1 − a_s)·f·a_f/2`, alpha `a_s + (1 − a_s)·a_f/2`). The 0.7.2 model
//! states no alignment: its strokes are `Outside`, handed to egui as given.

use egui::Color32;

/// Where a stroke lies against its box's edge (spec §2, `StrokeAlign`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "Inside and Center come from the 0.8.0 model only, read by `next`'s mapping, built next in M4.E2"
    )
)]
pub(crate) enum LineAlign {
    Outside,
    Inside,
    Center,
}

impl LineAlign {
    /// The 0.8.0 model's alignment; `None` for one this connector does not know.
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "read by `next`'s mapping, built next in M4.E2")
    )]
    pub(crate) fn of(align: native_theme::next::StrokeAlign) -> Option<Self> {
        match align {
            native_theme::next::StrokeAlign::Outside => Some(Self::Outside),
            native_theme::next::StrokeAlign::Inside => Some(Self::Inside),
            native_theme::next::StrokeAlign::Center => Some(Self::Center),
            _ => None,
        }
    }
}

/// A stroke's colour as stated, and where it lies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Line {
    pub color: Color32,
    pub align: LineAlign,
}

impl Line {
    /// The colour egui is handed for this stroke on a box it paints `fill` under: as stated
    /// where it is opaque, lies `Outside`, or lies over no fill (`None`, or a fill stated as
    /// none, which is transparent); else composited over `fill` (`Inside`) or over `fill` at
    /// half its alpha (`Center`).
    pub(crate) fn over(self, fill: Option<Color32>) -> Color32 {
        let Some(fill) = fill else {
            return self.color;
        };
        if self.color.a() == u8::MAX {
            return self.color;
        }
        match self.align {
            LineAlign::Outside => self.color,
            LineAlign::Inside => fill.blend(self.color),
            // 0.5: the band's mean, half of it over the fill (the decision's egui row)
            LineAlign::Center => fill.gamma_multiply(0.5).blend(self.color),
        }
    }
}

#[cfg(test)]
mod tests {
    use egui::Color32;

    use super::{Line, LineAlign};

    const FILL: Color32 = Color32::from_rgb(200, 100, 50);
    const HALF_BLACK: Color32 = Color32::from_rgba_unmultiplied_const(0, 0, 0, 128);

    fn line(align: LineAlign) -> Line {
        Line {
            color: HALF_BLACK,
            align,
        }
    }

    #[test]
    fn each_model_alignment_is_its_own() {
        use native_theme::next::StrokeAlign;
        assert_eq!(
            [
                LineAlign::of(StrokeAlign::Outside),
                LineAlign::of(StrokeAlign::Inside),
                LineAlign::of(StrokeAlign::Center),
            ],
            [
                Some(LineAlign::Outside),
                Some(LineAlign::Inside),
                Some(LineAlign::Center),
            ]
        );
    }

    #[test]
    fn an_outside_stroke_is_as_stated() {
        assert_eq!(line(LineAlign::Outside).over(Some(FILL)), HALF_BLACK);
    }

    #[test]
    fn an_inside_stroke_is_composited_over_the_fill() {
        let got = line(LineAlign::Inside).over(Some(FILL));
        // premultiplied s·a_s + (1 − a_s)·f, alpha a_s + (1 − a_s)·a_f
        let a_s = f32::from(HALF_BLACK.a()) / 255.0;
        let channel = |s: u8, f: u8| f32::from(s) + (1.0 - a_s) * f32::from(f);
        assert_within_one(
            got,
            [
                channel(HALF_BLACK.r(), FILL.r()),
                channel(HALF_BLACK.g(), FILL.g()),
                channel(HALF_BLACK.b(), FILL.b()),
                channel(HALF_BLACK.a(), FILL.a()),
            ],
        );
        assert_ne!(got, HALF_BLACK);
    }

    #[test]
    fn a_centred_stroke_is_composited_over_half_the_fill() {
        let got = line(LineAlign::Center).over(Some(FILL));
        // the decision's formula in premultiplied channels: s·a_s + (1 − a_s)·f·a_f/2, alpha
        // a_s + (1 − a_s)·a_f/2, each rounded by egui's own 8-bit arithmetic within one step
        let a_s = f32::from(HALF_BLACK.a()) / 255.0;
        let channel = |s: u8, f: u8| f32::from(s) + (1.0 - a_s) * f32::from(f) / 2.0;
        let want = [
            channel(HALF_BLACK.r(), FILL.r()),
            channel(HALF_BLACK.g(), FILL.g()),
            channel(HALF_BLACK.b(), FILL.b()),
            channel(HALF_BLACK.a(), FILL.a()),
        ];
        assert_within_one(got, want);
        assert_ne!(got, HALF_BLACK);
    }

    /// Each premultiplied channel of `got` within one 8-bit step of `want`'s.
    fn assert_within_one(got: Color32, want: [f32; 4]) {
        for (got, want) in got.to_array().into_iter().zip(want) {
            assert!((f32::from(got) - want).abs() <= 1.0, "{got} against {want}");
        }
    }

    #[test]
    fn an_opaque_stroke_is_as_stated_in_every_alignment() {
        for align in [LineAlign::Outside, LineAlign::Inside, LineAlign::Center] {
            let opaque = Line {
                color: Color32::from_rgb(1, 2, 3),
                align,
            };
            assert_eq!(opaque.over(Some(FILL)), Color32::from_rgb(1, 2, 3));
        }
    }

    #[test]
    fn a_stroke_over_no_fill_is_as_stated() {
        for align in [LineAlign::Outside, LineAlign::Inside, LineAlign::Center] {
            assert_eq!(line(align).over(None), HALF_BLACK);
        }
    }

    #[test]
    fn a_stroke_over_a_fill_stated_as_none_is_as_stated() {
        for align in [LineAlign::Outside, LineAlign::Inside, LineAlign::Center] {
            assert_eq!(line(align).over(Some(Color32::TRANSPARENT)), HALF_BLACK);
        }
    }
}
