//! Apple's size-to-tracking tables (S2.2's `sf_tracking`, ADD-A54): the
//! tracking of SF Pro, SF Pro Rounded and New York at a font size, from the
//! HIG's tables (`gen_sf_tracking`, generated from `hig-typography@2026-02-09`
//! by `scripts/generate_sf_tracking.py`).
//!
//! The size in points (S9.4's DPI) reads the table's 1/1000 em column: a size
//! between two rows takes that value interpolated linearly between them, one
//! below the first row or above the last that row's (the HIG: the system font
//! "dynamically adjusts tracking at every point size"). The tracking is that
//! value times the size, in 1/1000 em, converted back by the same DPI.

use super::gen_sf_tracking::{NEW_YORK, SF_PRO, SF_PRO_ROUNDED};

/// The typeface whose tracking table `sf_tracking` reads (S2.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TrackingDesign {
    /// SF Pro, the system font.
    SfPro,
    /// SF Pro Rounded.
    SfProRounded,
    /// New York, the system serif.
    NewYork,
}

impl TrackingDesign {
    /// The design's table: each row a size in points and its tracking in
    /// 1/1000 em, the sizes increasing.
    fn table(self) -> &'static [(f32, f32)] {
        match self {
            TrackingDesign::SfPro => &SF_PRO,
            TrackingDesign::SfProRounded => &SF_PRO_ROUNDED,
            TrackingDesign::NewYork => &NEW_YORK,
        }
    }
}

/// The tracking of a font of `font_size` logical px in `design`, in logical
/// px, the size converted to points by `font_dpi` (S9.4). Zero where the size
/// or the DPI is not finite, or the DPI not above zero (S2.2's *Totality*).
#[must_use]
pub fn sf_tracking(font_size: f32, design: TrackingDesign, font_dpi: f32) -> f32 {
    if !font_size.is_finite() || !font_dpi.is_finite() || font_dpi <= 0.0 {
        return 0.0;
    }
    let (size, dpi) = (f64::from(font_size), f64::from(font_dpi));
    let points = size * 72.0 / dpi;
    let tracking_points = per_mille(design.table(), points) * points / 1000.0;
    let tracking = (tracking_points * dpi / 72.0) as f32;
    if tracking.is_finite() { tracking } else { 0.0 }
}

/// The tracking in 1/1000 em at `points`: the table's value at a row,
/// interpolated linearly between two rows, the end row's outside them.
fn per_mille(rows: &[(f32, f32)], points: f64) -> f64 {
    let row = |(size, tracking): (f32, f32)| (f64::from(size), f64::from(tracking));
    let (Some(&first), Some(&last)) = (rows.first(), rows.last()) else {
        return 0.0;
    };
    let ((first_size, first_tracking), (last_size, last_tracking)) = (row(first), row(last));
    if points <= first_size {
        return first_tracking;
    }
    if points >= last_size {
        return last_tracking;
    }
    for pair in rows.windows(2) {
        if let &[below, above] = pair {
            let ((s0, t0), (s1, t1)) = (row(below), row(above));
            if points <= s1 {
                return t0 + (t1 - t0) * (points - s0) / (s1 - s0);
            }
        }
    }
    last_tracking
}

#[cfg(test)]
mod tests {
    use super::{TrackingDesign, sf_tracking};

    /// `got` is `expected` to a single's precision.
    fn close(got: f32, expected: f64) -> bool {
        (f64::from(got) - expected).abs() <= 1e-6 * expected.abs().max(1.0)
    }

    /// A row's size gives its own value: SF Pro's 17 pt row, -26/1000 em
    /// (`gen_sf_tracking`), at 72 DPI, where a point is a px, and at 144 DPI,
    /// where 34 px are those 17 pt.
    #[test]
    fn a_row_gives_its_value() {
        assert!(close(
            sf_tracking(17.0, TrackingDesign::SfPro, 72.0),
            -0.442
        ));
        assert!(close(
            sf_tracking(34.0, TrackingDesign::SfPro, 144.0),
            -0.884
        ));
    }

    /// Between rows the tracking in 1/1000 em is interpolated linearly: SF
    /// Pro's 13 pt row is -6 and its 14 pt row -11, so 13.5 pt is -8.5.
    #[test]
    fn between_rows_interpolates() {
        let got = sf_tracking(13.5, TrackingDesign::SfPro, 72.0);
        assert!(close(got, -8.5 * 13.5 / 1000.0), "{got}");
    }

    /// Below the first row and above the last the end row's value holds:
    /// SF Pro's 6 pt row is +41 and its 96 pt row 0.
    #[test]
    fn outside_the_rows_the_end_row_holds() {
        assert!(close(
            sf_tracking(4.0, TrackingDesign::SfPro, 72.0),
            41.0 * 4.0 / 1000.0
        ));
        assert!(close(sf_tracking(120.0, TrackingDesign::SfPro, 72.0), 0.0));
    }

    /// Each design reads its own table: at 6 pt SF Pro +41, SF Pro Rounded
    /// +87 and New York +40; New York's last row, 260 pt, -18, holds above
    /// it.
    #[test]
    fn each_design_reads_its_table() {
        assert!(close(sf_tracking(6.0, TrackingDesign::SfPro, 72.0), 0.246));
        assert!(close(
            sf_tracking(6.0, TrackingDesign::SfProRounded, 72.0),
            0.522
        ));
        assert!(close(sf_tracking(6.0, TrackingDesign::NewYork, 72.0), 0.24));
        assert!(close(
            sf_tracking(300.0, TrackingDesign::NewYork, 72.0),
            -5.4
        ));
    }

    /// The generated tables hold rows copied by hand from the HIG
    /// (`hig-typography@2026-02-09`'s `typography.json`), each by its JSON
    /// pointer below `#/primaryContentSections/0/content/` and its cells as
    /// the page writes them (size, 1/1000 em, points): a check of the
    /// generator's parse independent of it.
    #[test]
    fn hand_copied_rows_are_in_the_tables() {
        let rows = [
            // `83/rows/12`: 17 -26 -0.43
            (TrackingDesign::SfPro, 17.0, -26.0),
            // `83/rows/15`: 20 -23 -0.45
            (TrackingDesign::SfPro, 20.0, -23.0),
            // `83/rows/29`: 34 +12 +0.40
            (TrackingDesign::SfPro, 34.0, 12.0),
            // `81/tabs/1/content/1/rows/15`: 20 +18 +0.36
            (TrackingDesign::SfProRounded, 20.0, 18.0),
            // `81/tabs/2/content/1/rows/23`: 28 -12 -0.33
            (TrackingDesign::NewYork, 28.0, -12.0),
            // `81/tabs/2/content/1/rows/57`: 260 -18 -4.57
            (TrackingDesign::NewYork, 260.0, -18.0),
        ];
        for (design, size, per_mille) in rows {
            assert!(
                design.table().contains(&(size, per_mille)),
                "{design:?} {size} pt"
            );
        }
    }

    /// Two points worked out by hand from the rows above and their
    /// neighbours. New York at 13.5 pt, 72 DPI: its 13 pt row
    /// (`81/tabs/2/content/1/rows/8`: 13 +4 +0.05) and 14 pt row
    /// (`rows/9`: 14 +2 +0.03) give +3/1000 em halfway, so 3 × 13.5 / 1000 =
    /// 0.0405 px. SF Pro at 30 px, 108 DPI: 30 × 72 / 108 = 20 pt, its row
    /// -23/1000 em, so -23 × 20 / 1000 = -0.46 pt, × 108 / 72 = -0.69 px.
    #[test]
    fn hand_worked_points() {
        let between = sf_tracking(13.5, TrackingDesign::NewYork, 72.0);
        assert!(close(between, 0.0405), "{between}");
        let scaled = sf_tracking(30.0, TrackingDesign::SfPro, 108.0);
        assert!(close(scaled, -0.69), "{scaled}");
    }

    /// A size or DPI outside the function's domain gives zero.
    #[test]
    fn not_finite_is_zero() {
        assert_eq!(sf_tracking(f32::NAN, TrackingDesign::SfPro, 72.0), 0.0);
        assert_eq!(sf_tracking(17.0, TrackingDesign::SfPro, 0.0), 0.0);
        assert_eq!(sf_tracking(17.0, TrackingDesign::SfPro, f32::INFINITY), 0.0);
    }
}
